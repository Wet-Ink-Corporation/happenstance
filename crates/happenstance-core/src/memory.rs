//! An in-memory reference event store.

use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};
use std::sync::{OnceLock, PoisonError, RwLock};

use futures_core::Stream;

use crate::append::AppendCondition;
use crate::error::{AppendError, ConditionViolated};
use crate::event::{Event, SequencePosition, SequencedEvent};
use crate::identity::{EventId, RecordedAt, StoreId};
use crate::query::{Query, ReadOptions};
use crate::store::SendEventStore;

/// The reference implementation of [`EventStore`](crate::EventStore).
///
/// It exists for three reasons, in order of importance:
///
/// 1. it is the oracle the conformance suite is validated against, so that a
///    failing adapter is known to be the adapter's fault and not the suite's;
/// 2. it makes this crate's examples runnable, so the documentation cannot
///    drift from the API;
/// 3. it lets application code be written and tested before any real adapter
///    exists.
///
/// It is **not** built for scale. [`read`](crate::EventStore::read) snapshots
/// the matching events under the lock and streams from that snapshot, so a read
/// never holds the lock across a poll — correct, and deliberately simple.
/// Snapshot cost: payloads are [`Bytes`](bytes::Bytes), so a snapshot bumps
/// refcounts rather than copying data — but each event still costs `t + 2`.
///
/// Positions are dense and start at 1. The specification permits gaps, so
/// nothing may depend on that.
///
/// # Examples
///
/// The full DCB loop — read a decision model, then append conditioned on it:
///
/// ```
/// use happenstance_core::{
///     AppendCondition, Event, EventStore, MemoryEventStore, Query, QueryItem, Tags,
///     read_decision_model,
/// };
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = MemoryEventStore::new();
///
/// // "Has course c1 already been defined?"
/// let query = Query::from_item(QueryItem::new(
///     ["CourseDefined"],
///     Tags::from_pairs([("course", "c1")])?,
/// )?);
///
/// let (events, last_seen) = read_decision_model(&store, &query).await?;
/// assert!(events.is_empty());
///
/// let defined = Event::new("CourseDefined", &b"{\"capacity\":2}"[..])?
///     .with_tags(Tags::from_pairs([("course", "c1")])?);
///
/// // Append only if nothing matching has appeared since we looked.
/// let condition = AppendCondition::new(query.clone()).after_opt(last_seen);
/// store.append(&[defined.clone()], Some(&condition)).await?;
///
/// // Replaying the same command now fails — the course exists.
/// let err = store.append(&[defined], Some(&condition)).await.unwrap_err();
/// assert!(err.is_condition_violated());
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MemoryEventStore {
    events: RwLock<Vec<SequencedEvent>>,
    store_id: StoreId,
}

impl Default for MemoryEventStore {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryEventStore {
    /// Creates an empty store with a fresh incarnation identifier.
    #[must_use]
    pub fn new() -> Self {
        Self::with_store_id(next_store_id())
    }

    /// Creates an empty store with a caller-chosen incarnation.
    ///
    /// Exists for the rule that a reopened store must not reissue an identity:
    /// "reopening" an in-memory store is constructing a new one, and the rule
    /// needs to distinguish "the same store again" from "a different store".
    #[must_use]
    pub fn with_store_id(store_id: StoreId) -> Self {
        Self {
            events: RwLock::new(Vec::new()),
            store_id,
        }
    }

    /// The incarnation this store mints identities under.
    #[must_use]
    pub const fn store_id(&self) -> StoreId {
        self.store_id
    }

    /// Creates a store pre-loaded with `events`, assigned dense positions from
    /// 1.
    ///
    /// Useful for arranging test fixtures without going through
    /// [`append`](crate::EventStore::append).
    pub fn with_events(events: impl IntoIterator<Item = Event>) -> Self {
        let store = Self::new();
        let recorded_at = now();
        let sequenced = events
            .into_iter()
            .enumerate()
            .map(|(index, event)| {
                let position = position_at(index);
                SequencedEvent::new(
                    position,
                    EventId::new(store.store_id, position),
                    recorded_at,
                    event,
                )
            })
            .collect();

        Self {
            events: RwLock::new(sequenced),
            store_id: store.store_id,
        }
    }

    /// Reconstructs a store from a snapshot of what a durable medium held.
    ///
    /// The counterpart of [`snapshot`](Self::snapshot), and deliberately not
    /// [`with_events`](Self::with_events): that one *arranges* a store, assigning
    /// fresh positions, identities and a time, which is what a caller wants when
    /// seeding a fixture. This one *reopens* one, preserving every store-assigned
    /// fact on every event — which is what a real adapter's open does, because it
    /// reads rows it wrote rather than re-accepting events. A store whose reopen
    /// restamps hands every auditor the time of the last restart.
    ///
    /// # Examples
    ///
    /// ```
    /// # use happenstance_core::{Event, EventStore, MemoryEventStore};
    /// # async fn demo() -> Result<(), Box<dyn core::error::Error>> {
    /// let store = MemoryEventStore::new();
    /// store.append(&[Event::new("Issued", &b"{}"[..])?], None).await?;
    ///
    /// // What a durable medium would have kept, and what reopening finds.
    /// let reopened = MemoryEventStore::restore(store.store_id(), store.snapshot());
    /// assert_eq!(reopened.snapshot(), store.snapshot());
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn restore(store_id: StoreId, events: impl IntoIterator<Item = SequencedEvent>) -> Self {
        Self {
            events: RwLock::new(events.into_iter().collect()),
            store_id,
        }
    }

    /// The number of events held.
    pub fn len(&self) -> usize {
        self.read_guard().len()
    }

    /// Whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.read_guard().is_empty()
    }

    /// The position most recently assigned, or `None` if the store is empty.
    pub fn last_position(&self) -> Option<SequencePosition> {
        self.read_guard().last().map(|event| event.position)
    }

    /// A snapshot of every event held, in position order.
    pub fn snapshot(&self) -> Vec<SequencedEvent> {
        self.read_guard().clone()
    }

    /// Reads through the lock, recovering from poisoning.
    ///
    /// A panic in another thread while holding the lock cannot have left this
    /// store inconsistent: every mutation happens in one `extend` under the
    /// write lock, and the validation that precedes it does not mutate. So
    /// poisoning carries no information here and is ignored rather than
    /// propagated as a spurious failure.
    fn read_guard(&self) -> std::sync::RwLockReadGuard<'_, Vec<SequencedEvent>> {
        self.events.read().unwrap_or_else(PoisonError::into_inner)
    }
}

/// A fresh incarnation identifier.
///
/// Process-local and monotonic, which is enough here and would not be enough
/// anywhere else: nothing an in-memory store holds outlives the process, so
/// there is no persistent state a second incarnation could be confused with. A
/// durable adapter has the harder job — mint at database creation, and re-mint
/// when that state is restored or cloned.
///
/// The salt keeps two runs from minting the same identifier, so a test that
/// writes one down cannot accidentally pass by matching a later process's.
fn next_store_id() -> StoreId {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    static SALT: OnceLock<u64> = OnceLock::new();

    // `unsigned_abs` rather than a cast, and `0` where there is no clock: the
    // salt only has to separate two *runs*, and the counter below is what
    // separates two stores within one.
    let salt = *SALT.get_or_init(|| wall_clock_millis().unwrap_or(0).unsigned_abs());

    let ordinal = COUNTER.fetch_add(1, Ordering::Relaxed);

    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&salt.to_be_bytes());
    bytes[8..].copy_from_slice(&ordinal.to_be_bytes());
    StoreId::from_bytes(bytes)
}

/// The host clock, as milliseconds since the Unix epoch.
///
/// A clock that is behind the epoch, or a target that has none at all, yields
/// zero rather than failing: `append` has no error variant for "the clock is
/// broken", and inventing one would put a store failure in the caller's path
/// for a value the caller cannot act on.
fn now() -> RecordedAt {
    RecordedAt::from_millis(wall_clock_millis().unwrap_or(0))
}

/// Milliseconds since the Unix epoch, or `None` on a target with no clock.
///
/// **`SystemTime::now()` panics on `wasm32-unknown-unknown`.** It does not
/// return an error — there is no clock behind it, and the call aborts. That
/// target is the entire reason the bare `EventStore` flavour exists, so a
/// reference store that cannot even be *constructed* there is one nobody can
/// develop against the platform the contract was shaped for. This function is
/// the seam that keeps `MemoryEventStore::new()` total on every target.
///
/// The workspace gate compiles for `wasm32` but does not run there; CI does,
/// which is how this was found. A `cfg` is the only available answer: `std`
/// exposes no fallible clock, and reaching for a JavaScript one would put a
/// `wasm-bindgen` dependency in the contract crate to serve a store that exists
/// for tests and examples.
#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
fn wall_clock_millis() -> Option<i64> {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|since| i64::try_from(since.as_millis()).unwrap_or(i64::MAX))
}

/// No clock on this target; see the sibling above.
///
/// Every event a store on this target accepts is therefore stamped
/// `RecordedAt::from_millis(0)`. That is honest rather than convenient: VT-9's
/// own falsifier names "a target that cannot supply a wall clock at append
/// time", and this is one. It costs the rules nothing — they assert that a
/// recorded time is *stable*, never that it is recent — and an adapter for this
/// platform that has a real clock (a Durable Object does) supplies its own.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
fn wall_clock_millis() -> Option<i64> {
    None
}

/// The dense position for a zero-based index.
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index)
        .unwrap_or(u64::MAX - 1)
        .saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}

/// [`MemoryEventStore`]'s error type, which is uninhabited.
///
/// An uninhabited error is worth having: it proves the contract does not
/// *require* a fallible read path, and it documents at the type level that this
/// store has no failure modes of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unreachable: the in-memory event store cannot fail")]
pub enum MemoryStoreError {}

impl SendEventStore for MemoryEventStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        let guard = self.read_guard();

        // Filter, order and limit under the lock, then release it. The stream
        // that leaves this function borrows nothing.
        let matched = guard
            .iter()
            .filter(|event| query.matches(event.event_type(), event.tags()));

        // After filtering and ordering, never before: `limit` cuts the result
        // set the caller would otherwise have seen. A limit of zero yields
        // nothing, which is the point of it being `Option<usize>`.
        //
        // `unwrap_or(usize::MAX)` is how `Option<usize>` reaches `take`. No
        // store can hold `usize::MAX` events, so the unlimited case is
        // unchanged and pays nothing for the branch it no longer takes.
        let limit = options.limit.unwrap_or(usize::MAX);

        // `from` is the starting bound and `to` the stopping one, so reading
        // backwards swaps which side of the position order each sits on. Both
        // are inclusive in both directions.
        //
        // Where `take` sits in each chain is load-bearing at both ends. Below
        // `rev()` and below both bounds filters, so it cuts the *result* set
        // and never the *scanned* one — one link earlier it is the wrong
        // implementation ES-14 `[FROZEN]` rejects, and in the backwards branch
        // it would return the lowest matching positions instead of the
        // highest. Above `cloned()`, so the tail beyond `limit` is never
        // cloned: collecting first and cutting afterwards made `limit(1)` cost
        // what an unlimited read costs, which is the whole of the reduction
        // `limit` is supposed to buy.
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

        drop(guard);
        Snapshot(selected.into_iter())
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Emptiness first, and **above the lock** — ES-20 `[FROZEN]`.
        //
        // The specification defines `Events` as a non-empty collection, so there
        // is no position to return. Adapters must reject this rather than invent
        // one. The *precedence* is the part that was wrong here until phase 3:
        // evaluating the condition first made `append(&[], Some(&c))` answer
        // `NoEvents` or `ConditionViolated` depending on what the store happened
        // to hold, so two conformant adapters could disagree — and a caller whose
        // retry loop branches on `is_condition_violated()` sees the DCB
        // concurrency signal for what is unambiguously its own bug and retries
        // forever. `ConditionViolated` means "rebuild the decision model and try
        // again"; an empty batch will still be empty next time.
        //
        // It is checked above the lock because emptiness is a precondition on the
        // **argument**, not a question about the store. Nothing in the log can
        // change the answer, so taking a write lock to find out would be a lock
        // acquired to learn something already known.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // From here the whole operation runs under one write lock, which is what
        // makes it atomic: no reader can observe a partially-applied batch, and
        // no concurrent appender can slip between the condition check and the
        // write. That race is precisely what an append condition exists to
        // prevent, so getting it wrong here would make the reference
        // implementation useless as an oracle.
        let mut stored = self.events.write().unwrap_or_else(PoisonError::into_inner);

        if let Some(condition) = condition {
            let conflict = stored.iter().find(|existing| {
                condition.is_violated_by(existing.position, existing.event_type(), existing.tags())
            });

            if let Some(conflict) = conflict {
                // Returning without touching `stored` is what "a rejected
                // append leaves the store byte-identical" means.
                return Err(AppendError::ConditionViolated(ConditionViolated::at(
                    conflict.position,
                )));
            }
        }

        let first_index = stored.len();
        let recorded_at = now();
        stored.extend(events.iter().enumerate().map(|(offset, event)| {
            let position = position_at(first_index + offset);
            // A locally appended event's identity is this store's incarnation
            // paired with the position just assigned, so `id.position()` and
            // `position` agree here. They part company only for an event that
            // arrived through ingest, which this store has no way to accept.
            SequencedEvent::new(
                position,
                EventId::new(self.store_id, position),
                recorded_at,
                event.clone(),
            )
        }));

        Ok(position_at(first_index + events.len() - 1))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // The whole log is behind one lock, so the highest visible position is
        // the last element. An adapter over SQL answers with
        // `SELECT max(position)`; what neither may do is cache it outside the
        // transaction that assigns it, which is what makes a second handle
        // observe a stale head.
        Ok(self.read_guard().last().map(|event| event.position))
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        // A scan, because this store has no index and never will — it exists to
        // be correct and readable, not fast. The shape an adapter uses is
        // `WHERE origin_store = ? AND origin_position = ?`.
        Ok(self.read_guard().iter().any(|event| event.id == id))
    }
}

/// The stream returned by [`MemoryEventStore::read`].
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
    #![allow(clippy::unwrap_used)]

    // Only ONE flavour is imported for method-call syntax. Importing both
    // `EventStore` and `SendEventStore` makes `store.read(..)` ambiguous,
    // because `MemoryEventStore` satisfies both; see the `store` module
    // documentation. The `SendEventStore` path is exercised below through
    // fully-qualified calls.
    use super::{AppendCondition, Event, MemoryEventStore, Query, ReadOptions};
    use crate::store::EventStore;
    use crate::{QueryItem, SequencePosition, Tags, collect};

    fn event(event_type: &str) -> Event {
        Event::new(event_type, &b"{}"[..]).unwrap()
    }

    #[tokio::test]
    async fn append_assigns_dense_positions_from_one() {
        let store = MemoryEventStore::new();

        let last = store.append(&[event("A"), event("B")], None).await.unwrap();
        assert_eq!(last.get(), 2);

        let all = collect(store.read(&Query::all(), ReadOptions::new()))
            .await
            .unwrap();
        assert_eq!(
            all.iter().map(|e| e.position.get()).collect::<Vec<_>>(),
            [1, 2]
        );
    }

    #[tokio::test]
    async fn rejected_append_leaves_the_store_untouched() {
        let store = MemoryEventStore::new();
        store.append(&[event("A")], None).await.unwrap();
        let before = store.snapshot();

        let condition = AppendCondition::new(Query::from_item(QueryItem::of_types(["A"]).unwrap()));
        let err = store
            .append(&[event("B"), event("C")], Some(&condition))
            .await
            .unwrap_err();

        assert!(err.is_condition_violated());
        assert_eq!(store.snapshot(), before);
    }

    #[tokio::test]
    async fn empty_append_is_rejected() {
        let store = MemoryEventStore::new();
        assert!(store.append(&[], None).await.is_err());
    }

    #[tokio::test]
    async fn empty_append_is_refused_before_the_condition_is_evaluated() {
        // ES-20's precedence, at the reference implementation. The conformance
        // suite carries the same claim as
        // `empty_batch_is_refused_before_the_condition_is_evaluated`; this is
        // here as well because `MemoryEventStore` is the oracle the suite is
        // validated against, and the ordering used to be wrong in both.
        let store = MemoryEventStore::new();
        store.append(&[event("Blocker")], None).await.unwrap();

        let condition =
            AppendCondition::new(Query::from_item(QueryItem::of_types(["Blocker"]).unwrap()));
        let err = store.append(&[], Some(&condition)).await.unwrap_err();

        assert!(
            !err.is_condition_violated(),
            "an empty batch is the caller's own bug, and `ConditionViolated` \
             means `retry`: a caller branching on it loops forever. Got {err:?}"
        );
    }

    #[tokio::test]
    async fn with_events_matches_append() {
        let seeded = MemoryEventStore::with_events([event("A"), event("B")]);
        let appended = MemoryEventStore::new();
        appended
            .append(&[event("A"), event("B")], None)
            .await
            .unwrap();

        let seeded_snapshot = seeded.snapshot();
        let appended_snapshot = appended.snapshot();

        // Two stores cannot agree on identity or recorded time — those are facts
        // about *which* store accepted the event and *when*, and the whole point
        // of `StoreId` is that two incarnations differ. What `with_events`
        // claims is parity of position and payload, so that is what is compared.
        let shape = |events: &[crate::SequencedEvent]| {
            events
                .iter()
                .map(|event| (event.position, event.event.clone()))
                .collect::<Vec<_>>()
        };
        assert_eq!(shape(&seeded_snapshot), shape(&appended_snapshot));

        // And each store stamps its own incarnation, which is the property that
        // makes the comparison above the right one.
        assert!(
            seeded_snapshot
                .iter()
                .all(|event| event.id.store() == seeded.store_id())
        );
        assert!(
            appended_snapshot
                .iter()
                .all(|event| event.id.store() == appended.store_id())
        );
        assert_ne!(
            seeded.store_id(),
            appended.store_id(),
            "two stores are two incarnations"
        );
    }

    #[tokio::test]
    async fn append_stamps_identity_from_its_own_incarnation() {
        let store = MemoryEventStore::new();
        store.append(&[event("A"), event("B")], None).await.unwrap();

        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 2);

        for sequenced in &snapshot {
            assert_eq!(sequenced.id.store(), store.store_id());
            // For a locally appended event the two positions agree; they part
            // company only for an event accepted through ingest, which this
            // store cannot do.
            assert_eq!(sequenced.id.position(), sequenced.position);
        }

        assert_ne!(
            snapshot[0].id, snapshot[1].id,
            "two events, two identities, even with equal payloads"
        );
    }

    #[tokio::test]
    async fn structurally_equal_events_get_distinct_identities() {
        // Structural equality is not identity: appending the same event twice
        // must produce two events, not one, and a content hash would collapse
        // them.
        let store = MemoryEventStore::new();
        let twice = [event("A"), event("A")];
        assert_eq!(twice[0], twice[1], "the two events are structurally equal");

        store.append(&twice, None).await.unwrap();

        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 2);
        assert_ne!(snapshot[0].position, snapshot[1].position);
        assert_ne!(snapshot[0].id, snapshot[1].id);
    }

    #[tokio::test]
    async fn implements_the_non_send_flavour_too() {
        // Compiles only because `SendEventStore` implies `EventStore`. This is
        // the property that lets every consumer bind the weaker trait.
        async fn count<S: EventStore>(store: &S) -> usize {
            collect(store.read(&Query::all(), ReadOptions::new()))
                .await
                .map(|events| events.len())
                .unwrap_or_default()
        }

        let store = MemoryEventStore::new();
        store.append(&[event("A")], None).await.unwrap();
        assert_eq!(count(&store).await, 1);
    }

    #[test]
    fn send_flavour_stream_is_send_in_generic_code() {
        // ES-2, first half. On the `Send` flavour it is the *stream* that is
        // `Send`, not merely the future producing it.
        //
        // The bound is written at the *definition*, so inside the function the
        // compiler knows nothing about `S` beyond what `SendEventStore`
        // promises, and the obligation is discharged before monomorphisation.
        // Its predecessor asserted `Send` on a concrete store, where auto-trait
        // leakage from the hidden type satisfied it whatever the trait said: it
        // passed unchanged with `Send` struck from the `trait_variant`
        // attribute, which is what "cannot fail" means.
        //
        // This alone does NOT reject the `async fn read(..) -> Result<impl
        // Stream, E>` refactor — after it, `read` returns a future, the future
        // is `Send`, and this still passes. `spawns_from_generic` is what
        // rejects that, which is why both exist. ES-2 names only this one.
        fn assert_stream_is_send<S: crate::SendEventStore>(store: &S, query: &Query) {
            fn is_send<T: Send>(_: &T) {}
            is_send(&crate::SendEventStore::read(
                store,
                query,
                ReadOptions::new(),
            ));
        }

        assert_stream_is_send(&MemoryEventStore::new(), &Query::all());
    }

    #[tokio::test]
    async fn spawns_from_generic() {
        // ES-2's second half, and ES-3. The composition the `Send` flavour
        // exists to buy: hold a read across an await inside `tokio::spawn`.
        // This is the test that rejects `async fn read(..) -> Result<impl
        // Stream, E>` — `trait_variant` appends `Send` to the outermost `impl
        // Future` and nothing else, so after that refactor the stream held
        // across `collect`'s await is `!Send` and this stops compiling.
        //
        // Each bound was removed in turn and the compiler asked. Two are
        // load-bearing and one is not:
        //   Sync     REQUIRED. Removing it: "future cannot be sent between
        //            threads safely / captured value is not `Send`".
        //            `Arc<S>: Send` needs `S: Send + Sync`, and the body also
        //            holds `&*store` across an await, which needs `&S: Send` —
        //            that is `S: Sync` again (ES-3).
        //   'static  REQUIRED. Removing it: `error[E0310]: the parameter type
        //            `S` may not live long enough`. `tokio::spawn` erases the
        //            future into a task that outlives this frame.
        //   Send     REDUNDANT. Removing it still compiles: `trait_variant`
        //            emits `pub trait SendEventStore: Send`, so the supertrait
        //            already supplies it. Kept because it states the requirement
        //            at the signature rather than hiding it in the derivation —
        //            but nobody should later "discover" it is doing work here.
        fn spawns_from_generic<S: crate::SendEventStore + Send + Sync + 'static>(
            store: alloc::sync::Arc<S>,
        ) -> tokio::task::JoinHandle<usize> {
            tokio::spawn(async move {
                // Bound rather than inlined: edition 2024 RPITIT captures every
                // in-scope lifetime, so the stream borrows the `&Query` even
                // though its hidden type owns everything. Inlining is E0716.
                let query = Query::all();
                let stream = crate::SendEventStore::read(&*store, &query, ReadOptions::new());

                // `map_or` rather than `?` or a binding, deliberately.
                // `S::Error` carries no `Send` bound, so a `Result<_, S::Error>`
                // held across the *next* await would make this future `!Send`.
                // Collapsing to a `usize` first is what lets both awaits sit in
                // one spawned task.
                //
                // And this line does not relax later. ES-6 is `[FROZEN]`, and
                // ADR-0009 settled it the other way round from what an earlier
                // version of this comment expected: `Error` keeps
                // `core::error::Error + 'static` on both ports and both
                // flavours, and the stronger property is a marker trait that
                // generic code asks for. A caller who needs the error itself
                // across an await adds that bound to its own signature and pays
                // for it there; the port never grows one.
                let seen = collect(stream).await.map_or(0, |events| events.len());

                // Read-then-append: a second await against the same borrow, so
                // `&S` really does cross two suspension points.
                let appended = crate::SendEventStore::append(&*store, &[event("Spawned")], None)
                    .await
                    .is_ok();

                seen + usize::from(appended)
            })
        }

        let store = alloc::sync::Arc::new(MemoryEventStore::new());
        store.append(&[event("A")], None).await.unwrap();

        let counted = spawns_from_generic(alloc::sync::Arc::clone(&store))
            .await
            .unwrap();
        assert_eq!(counted, 2);
        assert_eq!(store.len(), 2);
    }

    /// The port a provided method would be added to, in miniature.
    ///
    /// [`EventStore`](crate::EventStore) has no provided method today, so ES-3
    /// and ES-4's rule has nothing on the real port to assert against — and a
    /// rule that can assert nothing is decorative. This trait is the smallest
    /// thing carrying the two properties the clauses are about: it goes through
    /// the same `trait_variant::make` the port does, and its provided body
    /// holds `&self` across an `await`. What is under test is the *derivation*,
    /// not this trait.
    ///
    /// A free generic helper — [`read_decision_model`](crate::read_decision_model)
    /// is the obvious candidate — cannot stand in, and the reason is worth
    /// recording because it looks like it should. A helper bound
    /// `S: EventStore` calls the **bare** flavour's `read`, whose opaque stream
    /// carries no `Send` inside generic code, so asserting `Send` on that
    /// helper's future would be false rather than merely weak. Only a
    /// `trait_variant`-derived twin has a `Send` flavour of the same body.
    #[trait_variant::make(SendProbe: Send)]
    trait Probe {
        /// The required half, standing in for `EventStore::head`.
        async fn value(&self) -> u64;

        /// The provided half, in ES-4's hand-desugared spelling.
        ///
        /// **Not `async fn`.** `trait_variant` clones the `default` block into
        /// the variant with `asyncness: None` and does not rewrite the body, so
        /// an `async fn` provided method becomes a non-`async` function
        /// containing `.await`: `error[E0728]`. That is ES-4's whole content.
        ///
        /// **`where Self: Sync`, at the point of use and nowhere else.** The
        /// `async move` block captures `&self`, and `&Self: Send` holds exactly
        /// when `Self: Sync`. Taken here, only callers of this method pay for
        /// it; moved onto the trait or into the `trait_variant` attribute,
        /// every adapter pays — including the `!Send` edge store the bare
        /// flavour exists for, which is not `Sync` and could not implement the
        /// trait at all (ES-3).
        ///
        /// Strike the clause and rustc's own `help:` proposes
        /// `#[trait_variant::make(SendProbe: Send where Self: Sync)]` — the
        /// attribute-level fix ES-3 names as the wrong one. The suggestion is
        /// the reason this is worth a compiled test rather than a note: it is
        /// one keystroke, it is offered by the compiler, and it silently moves
        /// the obligation onto every implementer.
        fn doubled(&self) -> impl Future<Output = u64>
        where
            Self: Sync,
        {
            // `self.value()` is awaited while `&self` is live, which is what
            // makes this an ES-3 body rather than a decorative one.
            async move { self.value().await * 2 }
        }
    }

    /// Something to instantiate the assertion at. `MemoryEventStore` is not
    /// used: the claim is about `trait_variant`'s derivation, and borrowing the
    /// store would only suggest it was about the store.
    struct Doubler(u64);

    impl SendProbe for Doubler {
        async fn value(&self) -> u64 {
            self.0
        }
    }

    #[test]
    fn provided_method_future_is_send_in_generic_code() {
        // ES-3 and ES-4. The bound is written at the *definition*, so inside
        // this function the compiler knows nothing about `S` beyond what
        // `SendProbe + Sync` promises and the obligation is discharged before
        // monomorphisation. Asserting against a concrete store instead is the
        // defect that retired `read_stream_is_send`: an auto-trait leaks out of
        // the hidden type and satisfies the assertion whatever the trait says.
        //
        // Two independent things have to hold for this to compile, and they
        // fail in different places. The *variant's* copy of the provided body
        // must prove `Send` at its own definition — strike `where Self: Sync`
        // and the trait above stops compiling, which is ES-3's rejection. And
        // `trait_variant` must actually have appended `Send` to the cloned
        // return type — if it stopped doing so, the trait would still compile
        // and this line is what would notice.
        fn assert_provided_future_is_send<S: SendProbe + Sync>(probe: &S) {
            fn is_send<T: Send>(_: &T) {}
            // Fully qualified, so the assertion names *which* flavour's body
            // it is about. Method syntax does resolve here — `S`'s only
            // declared bound is `SendProbe`, so the blanket `Probe` impl
            // `trait_variant` emits is not a candidate — but it would be
            // `error[E0034]` the moment this were written against a concrete
            // store, which satisfies both. That is the module documentation's
            // warning, and it is also why the qualification is not merely
            // decoration: it is the spelling that survives the rewrite.
            is_send(&SendProbe::doubled(probe));
        }

        // ES-4's "under both flavours **simultaneously**". One body, and it has
        // to type-check on the bare flavour as well — which is the flavour a
        // `wasm32` adapter implements and where nothing is `Send`. The bound is
        // again at the definition, so this says something about the trait
        // rather than about `Doubler`.
        fn bare_flavour_offers_the_same_body<S: Probe + Sync>(
            probe: &S,
        ) -> impl Future<Output = u64> {
            Probe::doubled(probe)
        }

        assert_provided_future_is_send(&Doubler(21));

        // Naming it is what instantiates it, in the shape `store.rs`'s ES-5
        // assertion uses: the claim is in the signature, so there is nothing to
        // run and awaiting the future would only exercise `Doubler`.
        let _ = bare_flavour_offers_the_same_body::<Doubler>;
    }

    #[tokio::test]
    async fn tags_filter_with_and_semantics() {
        let store = MemoryEventStore::new();
        store
            .append(
                &[
                    event("A").with_tags(Tags::from_pairs([("course", "c1")]).unwrap()),
                    event("A").with_tags(
                        Tags::from_pairs([("course", "c1"), ("student", "s1")]).unwrap(),
                    ),
                ],
                None,
            )
            .await
            .unwrap();

        let query = Query::from_item(
            QueryItem::tagged(Tags::from_pairs([("course", "c1"), ("student", "s1")]).unwrap())
                .unwrap(),
        );

        let matched = collect(store.read(&query, ReadOptions::new()))
            .await
            .unwrap();
        assert_eq!(matched.len(), 1);
        assert_eq!(matched[0].position.get(), 2);
    }
    // ---------------------------------------------------------------------
    // ES-14: `limit` truncates the filtered, ordered result set — and does so
    // without first materialising the part it is about to throw away.
    // ---------------------------------------------------------------------

    /// A store whose matches are deliberately scattered.
    ///
    /// Neither the head nor the tail of the log is a match, and the two tag
    /// values interleave, so an implementation that limited the *scanned* set
    /// rather than the *result* set answers differently from one that does not
    /// — which a uniformly-matching store would hide.
    async fn scattered() -> MemoryEventStore {
        let store = MemoryEventStore::new();
        let mut batch = Vec::new();
        for index in 0..12u32 {
            let event_type = if index % 3 == 0 { "A" } else { "B" };
            let course = if index % 2 == 0 { "c1" } else { "c2" };
            batch
                .push(event(event_type).with_tags(Tags::from_pairs([("course", course)]).unwrap()));
        }
        store.append(&batch, None).await.unwrap();
        store
    }

    /// Every query/option combination the matrix below walks.
    fn matrix() -> Vec<(Query, ReadOptions)> {
        let queries = [
            Query::all(),
            Query::from_item(QueryItem::of_types(["A"]).unwrap()),
            Query::from_item(
                QueryItem::tagged(Tags::from_pairs([("course", "c1")]).unwrap()).unwrap(),
            ),
            Query::from_items([
                QueryItem::of_types(["A"]).unwrap(),
                QueryItem::tagged(Tags::from_pairs([("course", "c2")]).unwrap()).unwrap(),
            ])
            .unwrap(),
        ];

        let mut combinations = Vec::new();
        for query in queries {
            for backwards in [false, true] {
                for from in [None, Some(3), Some(9)] {
                    for to in [None, Some(10)] {
                        let mut options = ReadOptions::new();
                        if backwards {
                            options = options.backwards();
                        }
                        if let Some(from) = from {
                            options = options.from(SequencePosition::new(from).unwrap());
                        }
                        if let Some(to) = to {
                            options = options.to(SequencePosition::new(to).unwrap());
                        }
                        combinations.push((query.clone(), options));
                    }
                }
            }
        }
        combinations
    }

    #[tokio::test]
    async fn read_limit_is_the_first_n_of_the_unlimited_read() {
        // ES-14 `[FROZEN]`, stated as an oracle that shares no step with the
        // implementation's truncation: the unlimited read *is* the filtered,
        // ordered result set, so `limit(n)` must equal its first `n` — whatever
        // the read path does internally to get there.
        //
        // This is the property the H2 remediation must preserve. It is what
        // rejects `.take(limit)` hoisted one link too far, above the bounds
        // filters or above `rev()`, where it would limit the SCANNED set: on
        // this store the two answers differ across most of the matrix, because
        // the head and the tail of the log are misses.
        let store = scattered().await;

        for (query, options) in matrix() {
            let unlimited = collect(store.read(&query, options)).await.unwrap();

            for limit in [0usize, 1, 2, 5, 100] {
                let limited = collect(store.read(&query, options.limit(limit)))
                    .await
                    .unwrap();
                let expected = &unlimited[..limit.min(unlimited.len())];

                assert_eq!(
                    limited.iter().map(|e| e.position).collect::<Vec<_>>(),
                    expected.iter().map(|e| e.position).collect::<Vec<_>>(),
                    "limit({limit}) must be the first {limit} of the unlimited \
                     read for {options:?}, not a truncation of a differently \
                     ordered or differently filtered set"
                );
            }
        }
    }

    #[tokio::test]
    async fn backwards_limit_takes_the_highest_matches_not_the_lowest() {
        // The ordering invariant `rev()`-before-`take()` protects, stated on
        // its own because the matrix above compares two reads through the same
        // code path and this compares one read against hand-written positions.
        //
        // The layout is Miss, Hit, Hit, Hit, Miss: a `take` above `rev()` would
        // hand back the two LOWEST matching positions, and a `take` above the
        // query filter would hand back nothing at all, because position 1 is a
        // miss. Both are the shape ES-14 `[FROZEN]` rejects.
        let store = MemoryEventStore::new();
        let hit = Tags::from_pairs([("course", "c1")]).unwrap();
        let miss = Tags::from_pairs([("course", "c2")]).unwrap();
        store
            .append(
                &[
                    event("A").with_tags(miss.clone()),
                    event("A").with_tags(hit.clone()),
                    event("A").with_tags(hit.clone()),
                    event("A").with_tags(hit),
                    event("A").with_tags(miss),
                ],
                None,
            )
            .await
            .unwrap();

        let query = Query::from_item(
            QueryItem::tagged(Tags::from_pairs([("course", "c1")]).unwrap()).unwrap(),
        );

        let backwards = collect(store.read(&query, ReadOptions::new().backwards().limit(2)))
            .await
            .unwrap();
        assert_eq!(
            backwards
                .iter()
                .map(|e| e.position.get())
                .collect::<Vec<_>>(),
            [4, 3],
            "backwards `limit(2)` must be the two highest MATCHING positions, \
             in descending order"
        );

        let forwards = collect(store.read(&query, ReadOptions::new().limit(2)))
            .await
            .unwrap();
        assert_eq!(
            forwards
                .iter()
                .map(|e| e.position.get())
                .collect::<Vec<_>>(),
            [2, 3],
            "forwards `limit(2)` must be the two lowest MATCHING positions"
        );
    }

    /// This file's own source, so the read path's shape can be asserted rather
    /// than reasoned about — the pattern `mod module_doc` uses in `store.rs`.
    const SOURCE: &str = include_str!("memory.rs");

    /// The body of `SendEventStore::read`, cut out of the production source.
    ///
    /// Cut *above* this test module on purpose. [`SOURCE`] includes this module
    /// too, and every needle the assertion looks for is spelled here as a
    /// literal, so an uncut haystack would be satisfied by the test itself.
    fn read_body() -> &'static str {
        let production = SOURCE.split("#[cfg(test)]").next().unwrap();
        let open = production.find("    fn read(").unwrap();
        let close = production[open..].find("    async fn append(").unwrap();
        &production[open..open + close]
    }

    #[test]
    fn read_takes_the_limit_before_it_clones() {
        // ES-14 `[FROZEN]` requires `limit` to truncate the filtered, ordered
        // set. It does not require that set to be MATERIALISED first, and
        // materialising it is what this store used to do: `.cloned()` ran over
        // every match and `Vec::truncate` then dropped the tail it had just
        // finished cloning. Measured, `limit(1)` against a million matching
        // events cost what `limit(None)` cost to four significant figures —
        // 4,000,020 heap operations against 4,000,038 — to return one event.
        //
        // No behavioural test can see that. The two spellings agree on every
        // input by construction, which is why the matrix above is a regression
        // guard and not a red test; and an allocation count is out of reach in
        // this crate, because `unsafe_code = "forbid"` bars a counting global
        // allocator. So the shape is pinned instead, in both directions ES-14
        // and the measurement care about:
        //
        //   * BELOW `rev()` and below both bounds filters, so `take` can never
        //     limit the SCANNED set — the wrong implementation ES-14 rejects;
        //   * ABOVE `cloned()`, so it never copies a tail it is about to drop.
        //
        // A pin on source shape is the instrument of last resort, and it is
        // used here for the reason `store.rs` uses one: the obligation is real,
        // it is not a type, and nothing else in the gate can see it.
        const STEPS: &[(&str, &str)] = &[
            ("query-filter", ".filter(|event| query.matches"),
            ("rev", ".rev()"),
            ("bound-filter", ".filter(|event| options."),
            ("take", ".take("),
            ("cloned", ".cloned()"),
            ("collect", ".collect()"),
        ];

        let body = read_body();
        let mut seen: Vec<(usize, &str)> = Vec::new();
        for (label, needle) in STEPS {
            let mut at = 0;
            while let Some(offset) = body[at..].find(needle) {
                seen.push((at + offset, label));
                at += offset + needle.len();
            }
        }
        seen.sort_unstable();
        let order: Vec<&str> = seen.into_iter().map(|(_, label)| label).collect();

        assert_eq!(
            order,
            [
                // one scan, shared by both branches
                "query-filter",
                // backwards: reversed, then bounded, then LIMITED, then cloned
                "rev",
                "bound-filter",
                "bound-filter",
                "take",
                "cloned",
                "collect",
                // forwards: the same chain without the reversal
                "bound-filter",
                "bound-filter",
                "take",
                "cloned",
                "collect",
            ],
            "`MemoryEventStore::read` must apply `take(limit)` below `rev()` \
             and below both bounds filters — above either it would limit the \
             scanned set, which ES-14 [FROZEN] rejects — and above `cloned()`, \
             so the discarded tail is never cloned at all"
        );

        assert!(
            !body.contains("truncate"),
            "`MemoryEventStore::read` still truncates after collecting, so \
             every match beyond `limit` is cloned and then dropped"
        );
    }
}
