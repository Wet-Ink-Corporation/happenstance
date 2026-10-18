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
    Classify, FailureClass, IngestError, IngestGroup, Ingested, SendIngestStore, Watermark,
};
use happenstance_sync_testkit::{IngestFixture, ingest_rules};
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

    /// The groups, written onto a copy of `rows` and returned whole, or the
    /// refusal; the caller commits.
    fn write_groups(
        &self,
        rows: &[Row],
        groups: &[IngestGroup<'_>],
    ) -> Result<(Vec<Row>, Ingested), IngestError<Poisoned>> {
        let held: BTreeSet<EventId> = rows.iter().map(|row| row.origin).collect();
        let mut working = rows.to_vec();
        let mut counts = Ingested::default();
        for group in groups {
            let landed_before = counts.appended;
            for shipped in group.events {
                let id = shipped.id;
                if held.contains(&id) {
                    counts.skipped += 1;
                    continue;
                }
                if id.store() == self.own {
                    return Err(IngestError::unheld_own_identity(id));
                }
                if working.iter().any(|row| row.origin == id) {
                    counts.skipped += 1;
                    continue;
                }
                let position = next_position(&working);
                let stored_id = match self.mistake {
                    Mistake::RemintsOnIngest => EventId::new(self.own, position),
                    _ => id,
                };
                let recorded_at = match self.mistake {
                    Mistake::RestampsRecordedAt => ARRIVAL,
                    _ => shipped.recorded_at,
                };
                working.push(Row {
                    stored: SequencedEvent::new(
                        position,
                        stored_id,
                        recorded_at,
                        shipped.event.clone(),
                    ),
                    origin: id,
                });
                counts.appended += 1;
            }
            if counts.appended > landed_before {
                for event in group.compensation {
                    let position = next_position(&working);
                    let id = EventId::new(self.own, position);
                    working.push(Row {
                        stored: SequencedEvent::new(position, id, ARRIVAL, event.clone()),
                        origin: id,
                    });
                    counts.compensated += 1;
                }
            }
        }
        Ok((working, counts))
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
        let (written, counts) = self.write_groups(&rows, groups)?;
        *rows = written;
        Ok(counts)
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
