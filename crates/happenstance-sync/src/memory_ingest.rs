//! An in-memory store that both appends and ingests, behind the `memory`
//! feature: the oracle the ingest family is proven against.
//!
//! # Why it exists: three jobs, one type
//!
//! The three [`MemorySyncPeer`](crate::MemorySyncPeer) gives, one port over:
//!
//! 1. **The oracle.** `happenstance-sync-testkit`'s ingest family is pointed at
//!    this store first. A rule red here is a wrong rule, not a wrong store.
//! 2. **The doctest target.** An example that needs a store which ingests uses
//!    this one, so it runs rather than merely compiles.
//! 3. **Something to test against.** A runner can be written and tested before
//!    any adapter implements [`IngestStore`](crate::IngestStore).
//!
//! # Why it is not `MemoryEventStore`
//!
//! `MemoryEventStore` lives in `happenstance-core`, and its only `&self` writer
//! is `append`, which mints. Coherence lets this crate add a trait to it and
//! never lets it reach inside (see [`ingest`](crate::ingest)), so the oracle
//! owns its log. It is built from `happenstance-core`'s public API alone, and
//! what it can take from the reference store it takes rather than copies:
//!
//! * **Its incarnation.** [`MemoryIngestStore::new`] uses the identifier a fresh
//!   `MemoryEventStore` would mint, so two oracles never share a `StoreId`.
//! * **Its clock.** A local append and a compensation are stamped by
//!   `MemoryEventStore::with_events`, so the seam that keeps that store total
//!   on `wasm32-unknown-unknown`, which has no clock, is not copied here.
//! * **Its error.** `MemoryStoreError`, which is uninhabited, serves both
//!   ports, so "the in-memory store cannot fail" has one name.
//!
//! What it cannot take is `read`'s filter chain, which is duplicated. The
//! event-store suite and a differential test against `MemoryEventStore`
//! hold the copy to the original.
//!
//! It is **not** built for scale. Every read snapshots under the lock, and
//! every membership question is a set lookup over everything held.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, MemoryEventStore,
    MemoryStoreError, Query, ReadOptions, RecordedAt, SendEventStore, SequencePosition,
    SequencedEvent, StoreId,
};

use crate::classify::{Classify, FailureClass};
use crate::identity::Watermark;
use crate::ingest::{IngestError, IngestGroup, Ingested, SendIngestStore};

/// An in-memory [`EventStore`](happenstance_core::EventStore) that is also an
/// [`IngestStore`](crate::IngestStore).
///
/// It answers every event-store question as `MemoryEventStore` does over the
/// same rows, and ingests as the port's documentation says: foreign events at
/// the tail with their origin identity and recorded time unchanged, a
/// re-delivery skipped by identity alone, compensation minted here only for a
/// group that landed something, and an unheld claim on this store's own
/// identity refusing the whole call (SY-38).
///
/// `Send + Sync`, with one `RwLock` over the log and the set of identities it
/// holds, so the two never disagree. Every check runs before anything is
/// written, and the write is one critical section.
///
/// Positions are dense and start at 1. The specification permits gaps, so
/// nothing may depend on that.
///
/// # Examples
///
/// A foreign event lands with the identity and the time its origin gave it,
/// including a time before 1970:
///
/// ```
/// use happenstance_core::{
///     Event, EventId, EventStore, Query, ReadOptions, RecordedAt, SequencePosition, StoreId,
///     collect,
/// };
/// use happenstance_sync::{IngestGroup, IngestStore, MemoryIngestStore, ReplicatedEvent};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = MemoryIngestStore::new();
/// let origin = StoreId::from_bytes([7; 16]);
/// let shipped = ReplicatedEvent::new(
///     EventId::new(origin, SequencePosition::new(4).ok_or("position zero")?),
///     RecordedAt::from_millis(-86_400_000),
///     Event::new("Shipped", &b"opaque"[..])?,
/// );
///
/// let ingested = store
///     .ingest(&[IngestGroup::new(core::slice::from_ref(&shipped), &[])])
///     .await?;
/// assert_eq!(ingested.appended, 1);
///
/// let log = collect(store.read(&Query::all(), ReadOptions::new())).await?;
/// let landed = log.first().ok_or("the log is empty")?;
/// assert_eq!(landed.id, shipped.id);
/// assert_eq!(landed.recorded_at, shipped.recorded_at);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MemoryIngestStore {
    store_id: StoreId,
    state: RwLock<State>,
}

/// The log, and every identity in it, under one lock.
#[derive(Debug, Default)]
struct State {
    /// Position order, dense from 1.
    log: Vec<SequencedEvent>,
    /// Every `id` in `log`, updated in the same critical section.
    held: BTreeSet<EventId>,
}

impl Default for MemoryIngestStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryIngestStore {
    /// An empty store with a fresh incarnation identifier.
    #[must_use]
    pub fn new() -> Self {
        Self::with_store_id(MemoryEventStore::new().store_id())
    }

    /// An empty store with a caller-chosen incarnation, as
    /// `MemoryEventStore::with_store_id` is.
    #[must_use]
    pub fn with_store_id(store_id: StoreId) -> Self {
        Self {
            store_id,
            state: RwLock::new(State::default()),
        }
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// Every event held, in position order.
    ///
    /// Owned, because the lock is released before the caller looks at it.
    #[must_use]
    pub fn snapshot(&self) -> Vec<SequencedEvent> {
        self.read_guard().log.clone()
    }

    /// Reads through the lock, recovering from poisoning.
    ///
    /// `MemoryEventStore`'s argument holds here too: every mutation is two
    /// `extend`s under the write lock, and nothing before them mutates, so a
    /// panic elsewhere cannot have left the state torn.
    fn read_guard(&self) -> RwLockReadGuard<'_, State> {
        self.state.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// Writes through the lock, recovering from poisoning for the same reason.
    fn write_guard(&self) -> RwLockWriteGuard<'_, State> {
        self.state.write().unwrap_or_else(PoisonError::into_inner)
    }
}

impl State {
    /// The position the next row written takes, `staged` rows past the log.
    fn next_position(&self, staged: usize) -> SequencePosition {
        position_at(self.log.len().saturating_add(staged))
    }

    /// Everything `groups` would write, and the counts, or the refusal. Mutates
    /// nothing, so a refusal leaves the store as it was.
    fn stage(
        &self,
        own: StoreId,
        groups: &[IngestGroup<'_>],
        recorded_at: RecordedAt,
    ) -> Result<(Vec<SequencedEvent>, Ingested), IngestError<MemoryStoreError>> {
        let mut staged = Vec::new();
        let mut staged_ids = BTreeSet::new();
        let mut counts = Ingested::default();
        for group in groups {
            let landed_before = counts.appended;
            for shipped in group.events {
                // Held when the call began: a re-delivery (SY-11).
                if self.held.contains(&shipped.id) {
                    counts.skipped += 1;
                    continue;
                }
                // Not held, and claiming this store's identity: SY-38. Checked
                // before the call's own rows, so a claim on a position this
                // call's compensation took is refused rather than skipped.
                if shipped.id.store() == own {
                    return Err(IngestError::unheld_own_identity(shipped.id));
                }
                // Repeated inside this call.
                if !staged_ids.insert(shipped.id) {
                    counts.skipped += 1;
                    continue;
                }
                staged.push(SequencedEvent::new(
                    self.next_position(staged.len()),
                    shipped.id,
                    shipped.recorded_at,
                    shipped.event.clone(),
                ));
                counts.appended += 1;
            }
            if counts.appended > landed_before {
                for event in group.compensation {
                    let position = self.next_position(staged.len());
                    staged.push(SequencedEvent::new(
                        position,
                        EventId::new(own, position),
                        recorded_at,
                        event.clone(),
                    ));
                    counts.compensated += 1;
                }
            }
        }
        Ok((staged, counts))
    }

    /// Writes `rows`, which [`stage`](Self::stage) or `append` checked.
    fn commit(&mut self, rows: Vec<SequencedEvent>) {
        self.held.extend(rows.iter().map(|row| row.id));
        self.log.extend(rows);
    }
}

/// The time this process's clock gives an event written now.
///
/// Read through `MemoryEventStore::with_events`, which stamps what it is given
/// with core's clock, so this store inherits core's answer on a target with no
/// clock (zero) rather than a second copy of the `cfg` that gives it. The
/// fallback is that same zero, and is not reached: a store built from one
/// event holds one event.
fn now(sample: &Event) -> RecordedAt {
    MemoryEventStore::with_events(core::iter::once(sample.clone()))
        .snapshot()
        .first()
        .map_or(RecordedAt::from_millis(0), |stamped| stamped.recorded_at)
}

/// The dense position for a zero-based index.
///
/// `happenstance-core`'s private helper of the same name, copied: no `Vec`
/// reaches `u64::MAX` elements, so the saturations are never taken.
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index)
        .unwrap_or(u64::MAX - 1)
        .saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// `MemoryStoreError` is uninhabited, so it states no class at all; the impl
/// exists because [`IngestStore::Error`](crate::IngestStore::Error) is bound
/// on [`Classify`].
impl Classify for MemoryStoreError {
    fn class(&self) -> FailureClass {
        match *self {}
    }
}

impl SendEventStore for MemoryIngestStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        let state = self.read_guard();
        let matched = state
            .log
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));
        let limit = options.limit.unwrap_or(usize::MAX);

        // `MemoryEventStore::read`'s chain, in its order: `take` below `rev()`
        // and below both bounds filters, so it cuts the result set and never
        // the scanned one (ES-14), and above `cloned()`, so nothing past
        // `limit` is cloned.
        let selected: Vec<SequencedEvent> = if options.backwards {
            matched
                .rev()
                .filter(|event| options.from.is_none_or(|from| event.position <= from))
                .filter(|event| options.to.is_none_or(|to| event.position >= to))
                .take(limit)
                .cloned()
                .collect()
        } else {
            matched
                .filter(|event| options.from.is_none_or(|from| event.position >= from))
                .filter(|event| options.to.is_none_or(|to| event.position <= to))
                .take(limit)
                .cloned()
                .collect()
        };

        drop(state);
        Snapshot(selected.into_iter())
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first, above the lock (ES-20).
        let Some(first) = events.first() else {
            return Err(AppendError::NoEvents);
        };
        let recorded_at = now(first);

        let mut state = self.write_guard();
        if let Some(condition) = condition {
            let conflict = state.log.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });
            if let Some(conflict) = conflict {
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }

        let own = self.store_id;
        let rows: Vec<SequencedEvent> = events
            .iter()
            .enumerate()
            .map(|(offset, event)| {
                let position = state.next_position(offset);
                SequencedEvent::new(
                    position,
                    EventId::new(own, position),
                    recorded_at,
                    event.clone(),
                )
            })
            .collect();
        let last = state.next_position(events.len().saturating_sub(1));
        state.commit(rows);
        Ok(last)
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        Ok(self.read_guard().log.last().map(|event| event.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        Ok(self.read_guard().held.contains(&id))
    }
}

impl SendIngestStore for MemoryIngestStore {
    type Error = MemoryStoreError;

    fn store_id(&self) -> StoreId {
        self.store_id
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        // Stamped outside the lock, and only when a group carries compensation
        // to stamp. With none, the time is never written.
        let recorded_at = groups
            .iter()
            .find_map(|group| group.compensation.first())
            .map_or(RecordedAt::from_millis(0), now);

        let mut state = self.write_guard();
        let (rows, counts) = state.stage(self.store_id, groups, recorded_at)?;
        state.commit(rows);
        Ok(counts)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        // The SQLite spike's definition: every row's origin pair, this store's
        // own included, folded to the highest position per origin.
        let mut watermark = Watermark::new();
        for row in &self.read_guard().log {
            watermark.advance(row.id.store(), row.id.position());
        }
        Ok(watermark)
    }
}

/// The stream [`MemoryIngestStore`]'s `read` returns.
///
/// Owns its events, so it borrows nothing from the store and is `Send`.
#[derive(Debug)]
struct Snapshot(alloc::vec::IntoIter<SequencedEvent>);

impl Stream for Snapshot {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(self.get_mut().0.next().map(Ok))
    }
}

#[cfg(test)]
mod tests {
    //! None of these asserts a literal local position. Every local position
    //! compared is one the store assigned, read back from `append`, `head` or
    //! the log; the literal positions are **origin** positions, which a peer
    //! supplies.

    use alloc::vec::Vec;

    use happenstance_core::{
        AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, Query,
        QueryItem, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId, Tags,
        collect,
    };

    use super::MemoryIngestStore;
    use crate::{IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent, Watermark};

    const ORIGIN: StoreId = StoreId::from_bytes([0x61; 16]);
    const OTHER_ORIGIN: StoreId = StoreId::from_bytes([0x62; 16]);

    fn id(store: StoreId, position: u64) -> EventId {
        EventId::new(
            store,
            SequencePosition::new(position).expect("every position written here is non-zero"),
        )
    }

    fn event(kind: &str, tag: &str) -> Event {
        Event::new(kind, tag.as_bytes().to_vec())
            .expect("every event type written here is valid")
            .with_tags(Tags::from_pairs([("k", tag)]).expect("every tag written here is valid"))
    }

    fn shipped(id: EventId, millis: i64, event: Event) -> ReplicatedEvent {
        ReplicatedEvent::new(id, RecordedAt::from_millis(millis), event)
    }

    async fn log(store: &MemoryIngestStore) -> Vec<SequencedEvent> {
        let Ok(log) = collect(store.read(&Query::all(), ReadOptions::new())).await;
        log
    }

    fn carrying(log: &[SequencedEvent], id: EventId) -> &SequencedEvent {
        let mut matching = log.iter().filter(|event| event.id == id);
        let found = matching.next().expect("the log carries the identity");
        assert!(matching.next().is_none(), "two events carry {id}");
        found
    }

    #[tokio::test]
    async fn an_empty_ingest_writes_nothing() {
        let store = MemoryIngestStore::new();

        let ingested = store.ingest(&[]).await;

        assert_eq!(ingested.ok(), Some(Ingested::default()));
        assert!(log(&store).await.is_empty());
    }

    #[tokio::test]
    async fn a_foreign_event_lands_with_its_origin_identity_and_time() {
        let store = MemoryIngestStore::new();
        let head = store
            .append(&[event("Local", "a")], None)
            .await
            .expect("an unconditioned append lands");
        let sent = shipped(id(ORIGIN, 4), -86_400_000, event("Shipped", "b"));

        let ingested = store
            .ingest(&[IngestGroup::new(core::slice::from_ref(&sent), &[])])
            .await
            .expect("a foreign event lands");

        assert_eq!(ingested.appended, 1);
        let log = log(&store).await;
        let landed = carrying(&log, sent.id);
        assert_eq!(landed.recorded_at, sent.recorded_at);
        assert_eq!(landed.event, sent.event);
        assert!(landed.position > head, "it landed at or below the head");
    }

    #[tokio::test]
    async fn redelivery_is_a_skip_and_writes_no_compensation() {
        let store = MemoryIngestStore::new();
        let sent = [shipped(id(ORIGIN, 1), 1, event("Shipped", "a"))];
        let answer = [event("Compensated", "c")];
        let group = [IngestGroup::new(&sent, &answer)];
        store
            .ingest(&group)
            .await
            .expect("the first delivery lands");
        let before = log(&store).await;

        let again = store
            .ingest(&group)
            .await
            .expect("a redelivery is no error");

        assert_eq!(
            (again.appended, again.skipped, again.compensated),
            (0, 1, 0)
        );
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn compensation_is_minted_here_after_its_group() {
        let store = MemoryIngestStore::new();
        let sent = [shipped(id(ORIGIN, 1), 1, event("Shipped", "a"))];
        let answer = [event("Compensated", "c")];

        let ingested = store
            .ingest(&[IngestGroup::new(&sent, &answer)])
            .await
            .expect("a group and its compensation land");

        assert_eq!(ingested.compensated, 1);
        let log = log(&store).await;
        let foreign = carrying(&log, sent[0].id);
        let compensation = log
            .iter()
            .find(|row| row.event == answer[0])
            .expect("the compensation is in the log");
        assert_eq!(compensation.id.store(), store.store_id());
        assert_eq!(compensation.id.position(), compensation.position);
        assert!(compensation.position > foreign.position);
    }

    #[tokio::test]
    async fn an_unheld_own_identity_refuses_the_whole_call() {
        let store = MemoryIngestStore::new();
        let head = store
            .append(&[event("Local", "a")], None)
            .await
            .expect("an unconditioned append lands");
        let before = log(&store).await;
        let claim = EventId::new(
            store.store_id(),
            head.next()
                .and_then(SequencePosition::next)
                .and_then(SequencePosition::next)
                .expect("three positions past a fresh head exist"),
        );
        let first = [shipped(id(ORIGIN, 1), 1, event("Shipped", "f1"))];
        let second = [
            shipped(id(ORIGIN, 2), 2, event("Shipped", "f2")),
            shipped(claim, 3, event("Local", "forged")),
        ];

        let refused = store
            .ingest(&[
                IngestGroup::new(&first, &[]),
                IngestGroup::new(&second, &[]),
            ])
            .await;

        assert!(
            matches!(refused, Err(IngestError::UnheldOwnIdentity { id }) if id == claim),
            "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
        );
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn a_claim_on_this_calls_own_compensation_is_refused() {
        let store = MemoryIngestStore::new();
        let head = store
            .append(&[event("Local", "a")], None)
            .await
            .expect("an unconditioned append lands");
        let before = log(&store).await;
        let claim = EventId::new(
            store.store_id(),
            head.next()
                .and_then(SequencePosition::next)
                .expect("two positions past a fresh head exist"),
        );
        let first = [shipped(id(ORIGIN, 1), 1, event("Shipped", "f1"))];
        let answer = [event("Compensated", "c1")];
        let second = [
            shipped(id(ORIGIN, 2), 2, event("Shipped", "f2")),
            shipped(claim, 3, event("Local", "forged")),
        ];

        let refused = store
            .ingest(&[
                IngestGroup::new(&first, &answer),
                IngestGroup::new(&second, &[]),
            ])
            .await;

        assert!(
            matches!(refused, Err(IngestError::UnheldOwnIdentity { id }) if id == claim),
            "expected UnheldOwnIdentity naming {claim}, got {refused:?}"
        );
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn a_held_own_identity_is_a_skip() {
        let store = MemoryIngestStore::new();
        let head = store
            .append(&[event("Local", "a")], None)
            .await
            .expect("an unconditioned append lands");
        let before = log(&store).await;
        let echo = [shipped(
            EventId::new(store.store_id(), head),
            1,
            event("Local", "a"),
        )];

        let ingested = store
            .ingest(&[IngestGroup::new(&echo, &[])])
            .await
            .expect("an echo of a held event is a skip");

        assert_eq!((ingested.appended, ingested.skipped), (0, 1));
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn append_mints_local_identities_and_honours_conditions() {
        let store = MemoryIngestStore::new();
        let blocker = Query::from_item(QueryItem::of_types(["Blocker"]).expect("a valid type"));
        let condition = AppendCondition::new(blocker);

        let empty = store.append(&[], Some(&condition)).await;
        assert!(matches!(empty, Err(AppendError::NoEvents)), "{empty:?}");

        let last = store
            .append(&[event("Blocker", "a"), event("Other", "b")], None)
            .await
            .expect("an unconditioned append lands");
        let before = log(&store).await;
        assert_eq!(before.last().map(|row| row.position), Some(last));
        for row in &before {
            assert_eq!(row.id, EventId::new(store.store_id(), row.position));
        }

        let refused = store.append(&[event("Late", "c")], Some(&condition)).await;
        assert!(
            matches!(refused, Err(AppendError::ConditionViolated(_))),
            "{refused:?}"
        );
        assert_eq!(log(&store).await, before);
    }

    #[tokio::test]
    async fn watermark_is_the_highest_origin_position_per_origin() {
        let store = MemoryIngestStore::new();
        store
            .append(&[event("Local", "a")], None)
            .await
            .expect("an unconditioned append lands");
        let first = [
            shipped(id(ORIGIN, 9), 1, event("Shipped", "a9")),
            shipped(id(ORIGIN, 4), 1, event("Shipped", "a4")),
        ];
        let second = [shipped(id(OTHER_ORIGIN, 2), 1, event("Shipped", "b2"))];
        store
            .ingest(&[
                IngestGroup::new(&first, &[event("Compensated", "c")]),
                IngestGroup::new(&second, &[]),
            ])
            .await
            .expect("foreign events land");

        let Ok(watermark) = store.watermark().await;

        let mut folded = Watermark::new();
        for row in store.snapshot() {
            folded.advance(row.id.store(), row.id.position());
        }
        assert_eq!(watermark, folded);
        assert_eq!(watermark.get(ORIGIN), Some(first[0].id.position()));
        assert_eq!(watermark.get(OTHER_ORIGIN), Some(second[0].id.position()));
    }

    /// Every read the oracle answers is the one core's reference store answers
    /// over the same rows: the filter chain is duplicated, and this is what
    /// holds the copy to the original.
    #[tokio::test]
    async fn reads_agree_with_memory_event_store_restored_from_the_snapshot() {
        let store = MemoryIngestStore::new();
        store
            .append(&[event("Local", "a"), event("Local", "b")], None)
            .await
            .expect("an unconditioned append lands");
        let first = [shipped(id(ORIGIN, 3), 5, event("Shipped", "a"))];
        let second = [
            shipped(id(OTHER_ORIGIN, 7), 6, event("Shipped", "b")),
            shipped(id(OTHER_ORIGIN, 8), 7, event("Other", "a")),
        ];
        store
            .ingest(&[
                IngestGroup::new(&first, &[event("Compensated", "b")]),
                IngestGroup::new(&second, &[]),
            ])
            .await
            .expect("foreign events land");
        store
            .append(&[event("Local", "c")], None)
            .await
            .expect("an unconditioned append lands");
        let reference = MemoryEventStore::restore(store.store_id(), store.snapshot());
        let at = |raw| SequencePosition::new(raw).expect("a non-zero position");
        let shipped_only =
            Query::from_item(QueryItem::of_types(["Shipped"]).expect("a valid type"));
        let tagged_a = Query::from_item(
            QueryItem::new(
                Vec::<&str>::new(),
                Tags::from_pairs([("k", "a")]).expect("a valid tag"),
            )
            .expect("a tag-only item is valid"),
        );
        let reads = [
            (Query::all(), ReadOptions::new()),
            (shipped_only, ReadOptions::new()),
            (tagged_a, ReadOptions::new()),
            (Query::all(), ReadOptions::new().from(at(3))),
            (Query::all(), ReadOptions::new().to(at(4))),
            (Query::all(), ReadOptions::new().from(at(2)).to(at(5))),
            (Query::all(), ReadOptions::new().backwards()),
            (Query::all(), ReadOptions::new().backwards().from(at(4))),
            (Query::all(), ReadOptions::new().limit(0)),
            (Query::all(), ReadOptions::new().limit(1)),
            (Query::all(), ReadOptions::new().limit(1).backwards()),
        ];

        for (query, options) in &reads {
            let Ok(ours) = collect(store.read(query, *options)).await;
            let Ok(theirs) = collect(reference.read(query, *options)).await;
            assert_eq!(ours, theirs, "{query:?} {options:?}");
        }
        let (Ok(our_head), Ok(their_head)) = (store.head().await, reference.head().await);
        assert_eq!(our_head, their_head);
        for row in reference.snapshot() {
            let Ok(ours) = store.contains_event_id(row.id).await;
            assert!(ours, "the oracle lost {}", row.id);
        }
        let Ok(stranger) = store.contains_event_id(id(ORIGIN, 99)).await;
        assert!(!stranger);
    }
}
