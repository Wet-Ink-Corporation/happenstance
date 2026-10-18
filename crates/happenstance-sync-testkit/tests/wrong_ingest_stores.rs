//! Every ingest rule, failing the wrong store it names, and only that rule.
//!
//! A rule that no store can fail is decorative. Each wrong store here is a
//! faithful ingest store with one plausible mistake added, and each test runs
//! the **whole** enumeration against it: the rule written against that mistake
//! must report a violation of its own clause, and every other rule must pass.
//! The second half is what keeps a mutant from passing for the wrong reason.
//!
//! The faithful store is the control: the same plumbing with no mistake passes
//! every rule, so a red result below is the mistake's and not the plumbing's.
//!
//! Standalone rather than a wrapper of `MemoryIngestStore`, in the shape of
//! `happenstance-sync`'s `HypotheticalAdapterStore`: two of the mistakes reach
//! inside the write, which a wrapper in a third crate cannot, and the oracle
//! carries no mistake switches.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, MutexGuard};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, Ingested, ReplicatedEvent, SendIngestStore,
    Watermark,
};
use happenstance_sync_testkit::{
    IngestFixture, NO_HIDDEN_CLAIM, NO_HIDDEN_CLAIM_REASON, ingest_rules,
};
use happenstance_testkit::{RuleOutcome, block_on};

/// The one mistake a [`WrongIngestStore`] makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// VT-5's wrong implementation: store an ingested event under this store's
    /// own identity at the position it took, keeping the origin's only for
    /// dedupe and the watermark. Everything a runner counts still adds up.
    RemintsOnIngest,
    /// VT-9's, through VT-5's rule: stamp an ingested event with the arrival
    /// time instead of the origin's.
    RestampsRecordedAt,
    /// SY-5's: an order-preserving merge. An ingested event is inserted where
    /// its origin numbered it and every later row is renumbered, so an event
    /// lands below a checkpoint a projection already read past.
    PlacesAtOriginPosition,
    /// SY-12's: decide a re-delivery on content, so a second origin's
    /// byte-identical fact is skipped as if it were the first one again.
    DedupesOnContent,
    /// SY-11's store half, through SY-12's rule: write a group's compensation
    /// again when the whole group was a re-delivery.
    RecompensatesOnRedelivery,
    /// SY-38's first, which its ledger row names: count an unheld claim on
    /// this store's identity as a re-delivery and write the rest of the call.
    DropsTheClaim,
    /// SY-38's second, also the ledger's: check only the batch's first event
    /// for this store's identity, and write any later claim as a foreign row.
    ChecksOnlyTheHead,
    /// SY-38's: commit group by group, so a refusal at the claim keeps every
    /// group before it.
    KeepsGroupsBeforeTheClaim,
    /// SY-38's: treat a claim that meets a row this call already wrote, as a
    /// compensation minted a moment earlier, as held, and skip it. Two events
    /// then share one identity.
    RefusesOnlyClaimsThatLand,
    /// SY-38's over-refusal: refuse an echo of an event this store holds,
    /// which is an ordinary re-delivery.
    RefusesEveryOwnIdentity,
    /// Not a mistake: a store that leaves a gap before each compensation, as
    /// the specification permits. SY-38's rule cannot arrange a claim hidden by
    /// the call's own compensation on it, and must say so rather than pass.
    LeavesAGap,
    /// SY-12's and SY-35's: decode the payload to reach the dedupe key, and
    /// skip an event whose data does not parse as UTF-8 text.
    DecodesPayload,
}

/// The time this store stamps everything it writes itself.
const ARRIVAL: RecordedAt = RecordedAt::from_millis(1_234_567_890_123);

/// What a wrong store fails with. Only a poisoned lock.
#[derive(Debug)]
struct Poisoned;

impl core::fmt::Display for Poisoned {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("the wrong store's lock was poisoned")
    }
}

impl core::error::Error for Poisoned {}

impl Classify for Poisoned {
    fn class(&self) -> FailureClass {
        FailureClass::Stop
    }
}

/// One stored row, and the identity the store dedupes and watermarks it by.
///
/// The two are the same for every mistake but [`Mistake::RemintsOnIngest`].
#[derive(Debug, Clone)]
struct Row {
    stored: SequencedEvent,
    origin: EventId,
}

/// A faithful ingest store with one [`Mistake`].
#[derive(Debug)]
struct WrongIngestStore {
    own: StoreId,
    rows: Mutex<Vec<Row>>,
    mistake: Mistake,
}

impl WrongIngestStore {
    fn rows(&self) -> Result<MutexGuard<'_, Vec<Row>>, Poisoned> {
        self.rows.lock().map_err(|_| Poisoned)
    }

    /// Writes `groups` onto `rows`: every group or none, except under
    /// [`Mistake::KeepsGroupsBeforeTheClaim`].
    fn write_groups(
        &self,
        rows: &mut Vec<Row>,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Poisoned>> {
        let held: BTreeSet<EventId> = rows.iter().map(|row| row.origin).collect();
        let mut working = rows.clone();
        let mut counts = Ingested::default();
        for (group_index, group) in groups.iter().enumerate() {
            // What a group-by-group commit has kept when this group begins.
            let committed =
                (self.mistake == Mistake::KeepsGroupsBeforeTheClaim).then(|| working.clone());
            let landed_before = counts.appended;
            for (event_index, shipped) in group.events.iter().enumerate() {
                let head_of_batch = group_index == 0 && event_index == 0;
                match self.verdict(&held, &working, shipped, head_of_batch) {
                    Verdict::Skip => counts.skipped += 1,
                    Verdict::Land => {
                        self.land(&mut working, shipped);
                        counts.appended += 1;
                    }
                    Verdict::Refuse => {
                        if let Some(kept) = committed {
                            *rows = kept;
                        }
                        return Err(IngestError::unheld_own_identity(shipped.id));
                    }
                }
            }
            if counts.appended > landed_before || self.mistake == Mistake::RecompensatesOnRedelivery
            {
                counts.compensated += self.compensate(&mut working, group.compensation);
            }
        }
        *rows = working;
        Ok(counts)
    }

    /// What this store does with `shipped`, given what it `held` when the call
    /// began and what the call has written so far.
    fn verdict(
        &self,
        held: &BTreeSet<EventId>,
        working: &[Row],
        shipped: &ReplicatedEvent,
        head_of_batch: bool,
    ) -> Verdict {
        let id = shipped.id;
        let claims_own = id.store() == self.own;
        if claims_own && self.mistake == Mistake::RefusesEveryOwnIdentity {
            return Verdict::Refuse;
        }
        let same_content = self.mistake == Mistake::DedupesOnContent
            && working.iter().any(|row| row.stored.event == shipped.event);
        // A store that parses the payload to find a key it dedupes on, and
        // treats bytes it cannot parse as nothing to write.
        let undecodable = self.mistake == Mistake::DecodesPayload
            && core::str::from_utf8(shipped.event.data()).is_err();
        if held.contains(&id) || same_content || undecodable {
            return Verdict::Skip;
        }
        let written_by_this_call = working.iter().any(|row| row.origin == id);
        let checked = self.mistake != Mistake::ChecksOnlyTheHead || head_of_batch;
        if claims_own && checked {
            return match self.mistake {
                Mistake::DropsTheClaim => Verdict::Skip,
                Mistake::RefusesOnlyClaimsThatLand if written_by_this_call => Verdict::Skip,
                _ => Verdict::Refuse,
            };
        }
        if written_by_this_call {
            Verdict::Skip
        } else {
            Verdict::Land
        }
    }

    /// Writes one foreign event at the tail, or where the mistake puts it.
    fn land(&self, working: &mut Vec<Row>, shipped: &ReplicatedEvent) {
        let position = next_position(working);
        let stored_id = match self.mistake {
            Mistake::RemintsOnIngest => EventId::new(self.own, position),
            _ => shipped.id,
        };
        let recorded_at = match self.mistake {
            Mistake::RestampsRecordedAt => ARRIVAL,
            _ => shipped.recorded_at,
        };
        let row = Row {
            stored: SequencedEvent::new(position, stored_id, recorded_at, shipped.event.clone()),
            origin: shipped.id,
        };
        if self.mistake == Mistake::PlacesAtOriginPosition {
            merge_at_origin_position(working, row);
        } else {
            working.push(row);
        }
    }

    /// Mints `compensation` here, and says how many it wrote.
    fn compensate(&self, working: &mut Vec<Row>, compensation: &[Event]) -> usize {
        for event in compensation {
            let mut position = next_position(working);
            if self.mistake == Mistake::LeavesAGap {
                position = position.next().unwrap_or(position);
            }
            let id = EventId::new(self.own, position);
            working.push(Row {
                stored: SequencedEvent::new(position, id, ARRIVAL, event.clone()),
                origin: id,
            });
        }
        compensation.len()
    }
}

/// What a [`WrongIngestStore`] does with one foreign event.
#[derive(Debug, Clone, Copy)]
enum Verdict {
    Skip,
    Land,
    Refuse,
}

/// [`Mistake::PlacesAtOriginPosition`]'s write: `row` goes where its origin
/// numbered it, or at the tail if the log is shorter, and every row from there
/// on is renumbered densely. Identities are left as they were.
fn merge_at_origin_position(working: &mut Vec<Row>, row: Row) {
    let wanted = usize::try_from(row.origin.position().get().saturating_sub(1))
        .unwrap_or(usize::MAX)
        .min(working.len());
    working.insert(wanted, row);
    let mut position = working
        .get(..wanted)
        .and_then(<[Row]>::last)
        .and_then(|before| before.stored.position.next())
        .unwrap_or(SequencePosition::FIRST);
    for later in working.iter_mut().skip(wanted) {
        later.stored.position = position;
        position = position.next().unwrap_or(position);
    }
}

/// The position after the last row's.
fn next_position(rows: &[Row]) -> SequencePosition {
    rows.last()
        .and_then(|row| row.stored.position.next())
        .unwrap_or(SequencePosition::FIRST)
}

impl SendEventStore for WrongIngestStore {
    type Error = Poisoned;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        let selected = self.rows().map(|rows| {
            let mut matched: Vec<SequencedEvent> = rows
                .iter()
                .map(|row| row.stored.clone())
                .filter(|event| query.matches(event.event_type(), event.tags()))
                .filter(|event| {
                    let (low, high) = if options.backwards {
                        (options.to, options.from)
                    } else {
                        (options.from, options.to)
                    };
                    low.is_none_or(|low| event.position >= low)
                        && high.is_none_or(|high| event.position <= high)
                })
                .collect();
            if options.backwards {
                matched.reverse();
            }
            matched.truncate(options.limit.unwrap_or(usize::MAX));
            matched
        });
        Owned(match selected {
            Ok(events) => events.into_iter().map(Ok).collect::<Vec<_>>().into_iter(),
            Err(error) => vec![Err(error)].into_iter(),
        })
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }
        let mut rows = self.rows().map_err(AppendError::Store)?;
        if let Some(condition) = condition
            && let Some(conflict) = rows.iter().find(|row| {
                condition.is_violated_by(
                    row.stored.position,
                    row.stored.event_type(),
                    row.stored.tags(),
                )
            })
        {
            return Err(AppendError::ConditionViolated(ConditionViolated::at(
                conflict.stored.position,
            )));
        }
        let mut last = SequencePosition::FIRST;
        for event in events {
            last = next_position(&rows);
            let id = EventId::new(self.own, last);
            rows.push(Row {
                stored: SequencedEvent::new(last, id, ARRIVAL, event.clone()),
                origin: id,
            });
        }
        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.rows()?.last().map(|row| row.stored.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.rows()?.iter().any(|row| row.stored.id == id))
    }
}

impl SendIngestStore for WrongIngestStore {
    type Error = Poisoned;

    fn store_id(&self) -> StoreId {
        self.own
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        let mut rows = self.rows().map_err(IngestError::Store)?;
        self.write_groups(&mut rows, groups)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        let mut watermark = Watermark::new();
        for row in self.rows()?.iter() {
            watermark.advance(row.origin.store(), row.origin.position());
        }
        Ok(watermark)
    }
}

/// An owned stream, so a read borrows nothing from the store.
#[derive(Debug)]
struct Owned(std::vec::IntoIter<Result<SequencedEvent, Poisoned>>);

impl Stream for Owned {
    type Item = Result<SequencedEvent, Poisoned>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::task::Poll::Ready(self.get_mut().0.next())
    }
}

/// A fixture over one [`WrongIngestStore`].
#[derive(Debug, Clone)]
struct WrongFixture(Arc<WrongIngestStore>);

impl WrongFixture {
    fn new(mistake: Mistake) -> Self {
        Self(Arc::new(WrongIngestStore {
            own: StoreId::from_bytes([0xE1; 16]),
            rows: Mutex::new(Vec::new()),
            mistake,
        }))
    }
}

/// A handle onto a [`WrongFixture`]'s store. A type of its own for the
/// orphan rule's reason, which `MemoryIngestHandle` states.
#[derive(Debug, Clone)]
struct WrongHandle(Arc<WrongIngestStore>);

impl IngestFixture for WrongFixture {
    type Store = WrongHandle;

    async fn connect(&self) -> Self::Store {
        WrongHandle(Arc::clone(&self.0))
    }
}

impl SendEventStore for WrongHandle {
    type Error = Poisoned;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        SendEventStore::read(&*self.0, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        SendEventStore::append(&*self.0, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        SendEventStore::head(&*self.0).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        SendEventStore::contains_event_id(&*self.0, id).await
    }
}

impl SendIngestStore for WrongHandle {
    type Error = Poisoned;

    fn store_id(&self) -> StoreId {
        self.0.own
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        SendIngestStore::ingest(&*self.0, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        SendIngestStore::watermark(&*self.0).await
    }
}

/// Every rule in the enumeration against one fixture, by name, each verdict
/// reduced to the clause it reports.
///
/// Driven by `for_each_ingest_store_rule!` rather than a list written here, so
/// a rule added to the family is run against every mutant below without an
/// edit.
fn verdicts<F, O>(open: O) -> Vec<(&'static str, Result<RuleOutcome, &'static str>)>
where
    F: IngestFixture,
    O: AsyncFn() -> F + Copy,
{
    macro_rules! run_each {
        ($($name:ident),* $(,)?) => {
            vec![$(
                (
                    stringify!($name),
                    block_on(ingest_rules::$name(open)).map_err(|violation| violation.clause()),
                )
            ),*]
        };
    }
    happenstance_sync_testkit::for_each_ingest_store_rule!(run_each)
}

/// `rule` reports `clause`, and every other rule passes.
fn fails_exactly(
    verdicts: &[(&'static str, Result<RuleOutcome, &'static str>)],
    rule: &str,
    clause: &'static str,
) {
    assert!(
        verdicts.iter().any(|(name, _)| *name == rule),
        "`{rule}` is not in the enumeration: {verdicts:?}"
    );
    for (name, verdict) in verdicts {
        if *name == rule {
            assert_eq!(*verdict, Err(clause), "`{name}` must reject its mutant");
        } else {
            assert_eq!(
                *verdict,
                Ok(RuleOutcome::Ran),
                "`{name}` failed `{rule}`'s mutant, so that mutant proves nothing about \
                 `{rule}` alone"
            );
        }
    }
}

#[test]
fn the_faithful_store_passes_every_rule() {
    let all = verdicts(async || WrongFixture::new(Mistake::Faithful));
    assert!(!all.is_empty(), "the enumeration ran no rules");
    for (name, verdict) in all {
        assert_eq!(verdict, Ok(RuleOutcome::Ran), "`{name}`");
    }
}

#[test]
fn ingest_preserves_origin_identity_fails_a_store_that_remints_on_ingest() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RemintsOnIngest)),
        "ingest_preserves_origin_identity",
        "VT-5",
    );
}

#[test]
fn ingest_preserves_origin_identity_fails_a_store_that_restamps_recorded_at() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RestampsRecordedAt)),
        "ingest_preserves_origin_identity",
        "VT-5",
    );
}

#[test]
fn ingested_events_land_above_the_local_head_fails_a_store_that_places_at_origin_position() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::PlacesAtOriginPosition)),
        "ingested_events_land_above_the_local_head",
        "SY-5",
    );
}

#[test]
fn dedupe_reaches_identity_without_decoding_fails_a_store_that_dedupes_on_content() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::DedupesOnContent)),
        "dedupe_reaches_identity_without_decoding",
        "SY-12",
    );
}

#[test]
fn dedupe_reaches_identity_without_decoding_fails_a_store_that_decodes_the_payload() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::DecodesPayload)),
        "dedupe_reaches_identity_without_decoding",
        "SY-12",
    );
}

#[test]
fn dedupe_reaches_identity_without_decoding_fails_a_store_that_recompensates_on_redelivery() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RecompensatesOnRedelivery)),
        "dedupe_reaches_identity_without_decoding",
        "SY-12",
    );
}

#[test]
fn an_unheld_own_identity_stops_the_ingest_fails_a_store_that_drops_the_claim() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::DropsTheClaim)),
        "an_unheld_own_identity_stops_the_ingest",
        "SY-38",
    );
}

#[test]
fn an_unheld_own_identity_stops_the_ingest_fails_a_store_that_checks_only_the_head() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::ChecksOnlyTheHead)),
        "an_unheld_own_identity_stops_the_ingest",
        "SY-38",
    );
}

#[test]
fn an_unheld_own_identity_stops_the_ingest_fails_a_store_that_keeps_groups_before_the_claim() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::KeepsGroupsBeforeTheClaim)),
        "an_unheld_own_identity_stops_the_ingest",
        "SY-38",
    );
}

#[test]
fn an_unheld_own_identity_stops_the_ingest_fails_a_store_that_refuses_only_claims_that_land() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RefusesOnlyClaimsThatLand)),
        "an_unheld_own_identity_stops_the_ingest",
        "SY-38",
    );
}

#[test]
fn an_unheld_own_identity_stops_the_ingest_fails_a_store_that_refuses_every_own_identity() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RefusesEveryOwnIdentity)),
        "an_unheld_own_identity_stops_the_ingest",
        "SY-38",
    );
}

/// A store that leaves a gap before its compensation is conformant, and the
/// rule cannot arrange the hidden claim on it. It says so, rather than
/// reporting a pass it did not check, and every other rule runs.
#[test]
fn an_unheld_own_identity_stops_the_ingest_skips_a_store_that_leaves_a_gap() {
    for (name, verdict) in verdicts(async || WrongFixture::new(Mistake::LeavesAGap)) {
        let expected = if name == "an_unheld_own_identity_stops_the_ingest" {
            RuleOutcome::Skipped {
                capability: NO_HIDDEN_CLAIM,
                reason: NO_HIDDEN_CLAIM_REASON,
            }
        } else {
            RuleOutcome::Ran
        };
        assert_eq!(verdict, Ok(expected), "`{name}`");
    }
}
