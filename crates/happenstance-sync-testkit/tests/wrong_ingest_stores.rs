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

use happenstance_core::bytes::Bytes;
use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId, StoreLimit,
};
use happenstance_sync::{
    Classify, FailureClass, IngestError, IngestGroup, Ingested, ReplicatedEvent, SendIngestStore,
    Watermark,
};
use happenstance_sync_testkit::{
    IngestFixture, NO_HIDDEN_CLAIM, NO_HIDDEN_CLAIM_REASON, Violation, ingest_rules,
};
use happenstance_testkit::{Capability, RuleOutcome, block_on};

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
    /// SY-30's, the one the `LiveStore` audit names: decide the whole call
    /// first, then write it in separately committed chunks of [`CHUNK`] rows,
    /// as a store does that stays under a statement's parameter cap without a
    /// transaction around the chunks. A fault mid-group keeps the chunks
    /// before it.
    ChunksWithoutATransaction,
    /// SY-30's second: commit each group's foreign rows, and then its
    /// compensation, as two writes. A fault in the compensation keeps the
    /// losing events with nothing answering them, which is SY-2's forbidden
    /// state on the ingest path.
    CommitsCompensationApart,
    /// SY-30's, the fixture's half: a fixture that declares the fault
    /// supported and whose arm does nothing, so the rule would compare a log
    /// an ingest never failed to write.
    ArmsNothing,
    /// SY-30's third: roll the group back when the fault fires, and answer
    /// `Ok` with the counts it would have written. The log is untouched, and
    /// a runner advances its watermark past a group that never landed.
    SwallowsTheFault,
    /// SY-18's, the shape the `LiveStore` audit names, on the per-event check
    /// SY-38's rule does not already own: hold only each group's first
    /// foreign event to the ceilings, so a later event over one is written.
    ChecksOnlyEachGroupsHead,
    /// SY-18's second: hold the foreign events to the ceilings and never the
    /// compensation, which the store writes itself.
    LeavesCompensationUnchecked,
    /// SY-18's third: refuse an event exactly at a ceiling, so the
    /// declaration is one larger than what the store takes.
    RefusesAtTheLimit,
    /// Not a mistake in the write: a store whose identity read fails, as a
    /// SQL store's does when its meta row is gone or its server is not
    /// reachable. Every rule that asks the store who it is must report the
    /// failure rather than carry on with an identity it was never told.
    CannotReadItsIdentity,
}

/// The ceilings every [`WrongIngestStore`] declares and enforces, far above
/// what any other rule writes: 15-byte strings, 6-byte undecodable payloads
/// and one tag.
const WRONG_MAX_DATA_LEN: usize = 512;
const WRONG_MAX_TAGS: usize = 8;
const WRONG_MAX_METADATA_LEN: usize = 512;

/// The time this store stamps everything it writes itself.
const ARRIVAL: RecordedAt = RecordedAt::from_millis(1_234_567_890_123);

/// What a wrong store fails with.
#[derive(Debug)]
enum WrongError {
    /// A lock was poisoned.
    Poisoned,
    /// The fault its fixture armed fired, inside the write.
    Faulted,
    /// [`Mistake::CannotReadItsIdentity`]'s identity read failed.
    IdentityUnreadable,
}

impl core::fmt::Display for WrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Poisoned => "the wrong store's lock was poisoned",
            Self::Faulted => "the fault the fixture armed fired between two rows of the ingest",
            Self::IdentityUnreadable => "the wrong store could not read its own identity",
        })
    }
}

impl core::error::Error for WrongError {}

impl Classify for WrongError {
    fn class(&self) -> FailureClass {
        match self {
            // A store that cannot say who it is cannot judge SY-38's claim,
            // and no retry tells it.
            Self::Poisoned | Self::IdentityUnreadable => FailureClass::Stop,
            // An injected fault is the shape of a dropped connection: the same
            // call can land once it is gone.
            Self::Faulted => FailureClass::Transient,
        }
    }
}

/// How many rows [`Mistake::ChunksWithoutATransaction`] commits at a time.
const CHUNK: usize = 2;

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
    /// The fault its fixture armed: the zero-based index, among the rows the
    /// next ingest writes, of the row whose write fails. Taken by that ingest.
    fault: Mutex<Option<usize>>,
    mistake: Mistake,
}

/// What [`WrongIngestStore::write_groups`] decided, before any of it is
/// committed.
#[derive(Debug)]
struct Written {
    counts: Ingested,
    /// The log the call leaves when nothing faults.
    working: Vec<Row>,
    /// The log's length after each group's foreign rows and after its
    /// compensation, in write order: where a store that commits the two halves
    /// apart commits.
    marks: Vec<usize>,
}

impl WrongIngestStore {
    fn rows(&self) -> Result<MutexGuard<'_, Vec<Row>>, WrongError> {
        self.rows.lock().map_err(|_| WrongError::Poisoned)
    }

    /// Commits what `written` decided onto `rows`, row by row in write order,
    /// failing at the row an armed fault names. A faithful store commits once,
    /// at the end, so a fault leaves `rows` as they were; the two chunking
    /// mistakes keep what they had committed before the faulted row.
    fn apply(
        &self,
        rows: &mut Vec<Row>,
        written: Written,
    ) -> Result<Ingested, IngestError<WrongError>> {
        let Written {
            counts,
            mut working,
            marks,
        } = written;
        let start = rows.len();
        let fault = self
            .fault
            .lock()
            .map_err(|_| IngestError::Store(WrongError::Poisoned))?
            .take();
        let written_rows = working.len().saturating_sub(start);
        let Some(after) = fault.filter(|after| *after < written_rows) else {
            *rows = working;
            return Ok(counts);
        };
        let failing = start.saturating_add(after);
        let committed = match self.mistake {
            Mistake::ChunksWithoutATransaction => Some(start.saturating_add(after / CHUNK * CHUNK)),
            Mistake::CommitsCompensationApart => Some(
                marks
                    .iter()
                    .copied()
                    .filter(|mark| *mark <= failing)
                    .max()
                    .unwrap_or(start),
            ),
            _ => None,
        };
        if let Some(committed) = committed {
            working.truncate(committed);
            *rows = working;
        }
        if self.mistake == Mistake::SwallowsTheFault {
            return Ok(counts);
        }
        Err(IngestError::Store(WrongError::Faulted))
    }

    /// Holds the events of `groups` to the declared ceilings before anything
    /// is decided: every one, foreign and compensation, or the ones the
    /// mistake checks.
    fn check_ceilings(&self, groups: &[IngestGroup<'_>]) -> Result<(), IngestError<WrongError>> {
        let foreign = if self.mistake == Mistake::ChecksOnlyEachGroupsHead {
            1
        } else {
            usize::MAX
        };
        let compensation = if self.mistake == Mistake::LeavesCompensationUnchecked {
            0
        } else {
            usize::MAX
        };
        for group in groups {
            let checked = group
                .events
                .iter()
                .take(foreign)
                .map(|shipped| &shipped.event)
                .chain(group.compensation.iter().take(compensation));
            for event in checked {
                self.within_ceilings(event)?;
            }
        }
        Ok(())
    }

    /// `event` is within every ceiling this store declares.
    fn within_ceilings(&self, event: &Event) -> Result<(), IngestError<WrongError>> {
        let over = |len: usize, ceiling: usize| {
            if self.mistake == Mistake::RefusesAtTheLimit {
                len >= ceiling
            } else {
                len > ceiling
            }
        };
        let measured = [
            (
                StoreLimit::EventDataLen,
                event.data().len(),
                WRONG_MAX_DATA_LEN,
            ),
            (StoreLimit::TagsPerEvent, event.tags().len(), WRONG_MAX_TAGS),
            (
                StoreLimit::MetadataLen,
                event.metadata().map_or(0, Bytes::len),
                WRONG_MAX_METADATA_LEN,
            ),
        ];
        match measured
            .into_iter()
            .find(|(_, len, ceiling)| over(*len, *ceiling))
        {
            Some((limit, len, _)) => Err(IngestError::exceeds_store_limit(limit, len)),
            None => Ok(()),
        }
    }

    /// Decides `groups` against `rows`: every group or none, except under
    /// [`Mistake::KeepsGroupsBeforeTheClaim`], which commits the groups before
    /// a refusal onto `rows` itself.
    fn write_groups(
        &self,
        rows: &mut Vec<Row>,
        groups: &[IngestGroup<'_>],
    ) -> Result<Written, IngestError<WrongError>> {
        let held: BTreeSet<EventId> = rows.iter().map(|row| row.origin).collect();
        let mut working = rows.clone();
        let mut counts = Ingested::default();
        let mut marks = Vec::new();
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
            marks.push(working.len());
            if counts.appended > landed_before || self.mistake == Mistake::RecompensatesOnRedelivery
            {
                counts.compensated += self.compensate(&mut working, group.compensation);
            }
            marks.push(working.len());
        }
        Ok(Written {
            counts,
            working,
            marks,
        })
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
    type Error = WrongError;

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
    type Error = WrongError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        if self.mistake == Mistake::CannotReadItsIdentity {
            return Err(WrongError::IdentityUnreadable);
        }
        Ok(self.own)
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        self.check_ceilings(groups)?;
        let mut rows = self.rows().map_err(IngestError::Store)?;
        let written = self.write_groups(&mut rows, groups)?;
        self.apply(&mut rows, written)
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
struct Owned(std::vec::IntoIter<Result<SequencedEvent, WrongError>>);

impl Stream for Owned {
    type Item = Result<SequencedEvent, WrongError>;

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
            fault: Mutex::new(None),
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

    /// Supported, so SY-30's rule runs against every mistake here: its
    /// `ingest` commits through [`WrongIngestStore::apply`], which can fail
    /// at any row.
    const MID_GROUP_FAULT: Capability = Capability::SUPPORTED;

    /// Mirrored from the constants the store enforces, so SY-18's ingest
    /// rule runs against every mistake here.
    const MAX_EVENT_DATA_LEN: Option<usize> = Some(WRONG_MAX_DATA_LEN);
    const MAX_TAGS_PER_EVENT: Option<usize> = Some(WRONG_MAX_TAGS);
    const MAX_METADATA_LEN: Option<usize> = Some(WRONG_MAX_METADATA_LEN);

    async fn connect(&self) -> Self::Store {
        WrongHandle(Arc::clone(&self.0))
    }

    async fn arm_mid_group_fault(&self, after: usize) -> Result<(), Violation> {
        if self.0.mistake == Mistake::ArmsNothing {
            return Ok(());
        }
        let mut fault =
            self.0.fault.lock().map_err(|_| {
                Violation::new("SY-30", "the wrong store's fault lock was poisoned")
            })?;
        *fault = Some(after);
        Ok(())
    }
}

impl SendEventStore for WrongHandle {
    type Error = WrongError;

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
    type Error = WrongError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        SendIngestStore::store_id(&*self.0).await
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

/// Every rule asks the store who it is before it arranges anything, so a
/// store whose identity read fails is reported by every rule, each under its
/// own clause, and none passes on an identity it was never told.
#[test]
fn every_rule_reports_a_store_that_cannot_read_its_identity() {
    let all = verdicts(async || WrongFixture::new(Mistake::CannotReadItsIdentity));
    let clauses = [
        ("ingest_preserves_origin_identity", "VT-5"),
        ("ingested_events_land_above_the_local_head", "SY-5"),
        ("dedupe_reaches_identity_without_decoding", "SY-12"),
        ("an_unheld_own_identity_stops_the_ingest", "SY-38"),
        ("ingest_group_is_atomic_under_a_mid_group_fault", "SY-30"),
        ("ingest_validates_every_event_not_the_head", "SY-18"),
    ];
    assert_eq!(all.len(), clauses.len(), "a rule was added: {all:?}");
    for (name, verdict) in all {
        let (_, clause) = clauses
            .iter()
            .find(|(rule, _)| *rule == name)
            .expect("every rule in the enumeration is listed above");
        assert_eq!(verdict, Err(*clause), "`{name}`");
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

#[test]
fn ingest_group_is_atomic_under_a_mid_group_fault_fails_a_store_that_chunks_without_a_transaction()
{
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::ChunksWithoutATransaction)),
        "ingest_group_is_atomic_under_a_mid_group_fault",
        "SY-30",
    );
}

#[test]
fn ingest_group_is_atomic_under_a_mid_group_fault_fails_a_store_that_commits_compensation_apart() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::CommitsCompensationApart)),
        "ingest_group_is_atomic_under_a_mid_group_fault",
        "SY-30",
    );
}

/// The mutant only the rule's demand for `Err` sees: its log is the faithful
/// store's, and its answer is a lie.
#[test]
fn ingest_group_is_atomic_under_a_mid_group_fault_fails_a_store_that_swallows_the_fault() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::SwallowsTheFault)),
        "ingest_group_is_atomic_under_a_mid_group_fault",
        "SY-30",
    );
}

/// The fixture's half: an armed fault that never fires lets the group land,
/// so the rule is not left comparing a log no ingest failed to write.
#[test]
fn ingest_group_is_atomic_under_a_mid_group_fault_fails_a_fixture_that_arms_nothing() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::ArmsNothing)),
        "ingest_group_is_atomic_under_a_mid_group_fault",
        "SY-30",
    );
}

#[test]
fn ingest_validates_every_event_not_the_head_fails_a_store_that_checks_only_each_groups_head() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::ChecksOnlyEachGroupsHead)),
        "ingest_validates_every_event_not_the_head",
        "SY-18",
    );
}

#[test]
fn ingest_validates_every_event_not_the_head_fails_a_store_that_leaves_compensation_unchecked() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::LeavesCompensationUnchecked)),
        "ingest_validates_every_event_not_the_head",
        "SY-18",
    );
}

/// The declaration off by one: only the rule's control, which writes events
/// exactly at each ceiling, can see it.
#[test]
fn ingest_validates_every_event_not_the_head_fails_a_store_that_refuses_at_the_limit() {
    fails_exactly(
        &verdicts(async || WrongFixture::new(Mistake::RefusesAtTheLimit)),
        "ingest_validates_every_event_not_the_head",
        "SY-18",
    );
}
