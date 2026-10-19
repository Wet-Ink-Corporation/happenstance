//! SY-17: the sync port composes over a store and a peer that are not `Send`.
//!
//! The runner is not built yet, but its bound is already decided: it is written
//! against [`EventStore`], [`IngestStore`] and [`SyncPeer`] — the bare flavours,
//! the weaker requirement both flavours meet — never against `SendEventStore`,
//! `SendIngestStore` or `SendSyncPeer`. The reason is a deployment rather than a
//! taste: the `!Send` peer, SQLite inside a Durable Object, sits in the *middle*
//! of a chain, a spoke to one store and a hub to many devices, so a runner bound
//! on the `Send` flavour would exclude the hub from its own topology.
//!
//! A generic body type-checks against its declared bounds whether or not
//! anything instantiates it, so a bound read off the source proves nothing. The
//! proof here is the **instantiation**: [`exchange`] is written the way the
//! runner's step will be written, and it is called with a store and a peer that
//! each hold an `Rc`. Tighten any bound in its `where` clause to the `Send`
//! flavour and this file stops compiling, with `error[E0277]`: the local types
//! implement only the bare flavour, because with an `Rc` inside they cannot
//! implement the other.
//!
//! The instantiation is only evidence if the two types really are `!Send`, so
//! [`not_send`] asserts that at compile time, beside two positive controls that
//! show the assertion is not vacuous.
//!
//! One flavour name is imported per trait, the bare one, and the `Send` flavour
//! is reached by full path where a delegation needs it (CLAUDE.md rule 4).

use core::cell::RefCell;
use std::rc::Rc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, MemoryStoreError,
    Query, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    Classify, EventGroup, FailureClass, IngestError, IngestGroup, IngestStore, Ingested,
    MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch, ReplicatedEvent,
    SyncPeer, Watermark,
};

/// The failure [`exchange`] reports: any of the three ports' own errors.
///
/// Boxed without `Send`, because none of the three associated error types
/// promises it (ADR-0009) and a `!Send` adapter's error is the case this file
/// exists for.
type Failure = Box<dyn core::error::Error>;

/// What one [`exchange`] did.
#[derive(Debug)]
struct Exchanged<R> {
    /// What the store reported for the ingest.
    ingested: Ingested,
    /// Where the next exchange resumes.
    resume: R,
    /// Whether, after the ingest, the store answers held for every pulled event.
    all_held: bool,
}

/// One pull-and-ingest step, bound the way the runner's will be.
///
/// Every bound is the bare flavour. The body uses all three ports: it pulls from
/// the peer, ingests through [`IngestStore`] with `answer` as each group's
/// compensation, and then asks the store it already has through
/// [`EventStore::contains_event_id`] — the membership question `IngestStore`
/// deliberately does not carry.
async fn exchange<S, P>(
    store: &S,
    peer: &P,
    from: Option<&P::Resume>,
    answer: &[Event],
) -> Result<Exchanged<P::Resume>, Failure>
where
    S: EventStore + IngestStore,
    P: SyncPeer,
{
    let Pulled { batch, resume, .. } = peer.pull(from).await?;
    let groups: Vec<IngestGroup<'_>> = batch
        .groups
        .iter()
        .map(|group| IngestGroup::new(&group.events, answer))
        .collect();
    let ingested = IngestStore::ingest(store, &groups).await?;

    let mut all_held = true;
    for event in batch.events() {
        all_held &= EventStore::contains_event_id(store, event.id).await?;
    }

    Ok(Exchanged {
        ingested,
        resume,
        all_held,
    })
}

// ---------------------------------------------------------------------------
// A store that is genuinely `!Send`
// ---------------------------------------------------------------------------

/// A store that holds its log through an `Rc`, as a Durable Object holds its
/// storage.
///
/// `Rc` rather than a bare `RefCell` because `RefCell<T>: Send` whenever
/// `T: Send` — it gives up `Sync`, not `Send` — so a store built on a `RefCell`
/// alone would satisfy the `Send` flavour and prove nothing. The `RefCell`
/// inside the second `Rc` is the store's own interior mutability for ingested
/// rows; no borrow of it is held across an `.await`.
///
/// **An instrument for bounds, not an oracle.** Foreign rows sit beside the
/// local log rather than in it, because `MemoryEventStore` mints every identity
/// it writes and nothing outside `happenstance-core` can make it keep a foreign
/// one (see `ingest.rs`). So [`read`](EventStore::read) and
/// [`head`](EventStore::head) see local appends and compensation only, while
/// [`contains_event_id`](EventStore::contains_event_id) and the watermark see
/// both. A sync-owned store that keeps both in one log is phase 13's in-memory
/// oracle (#108), and the `!Send` store the typed layer instantiates its own
/// entry points against is `crates/happenstance/tests/flavours.rs`.
#[derive(Debug)]
struct LocalStore {
    /// Local appends, and the compensation an ingest writes.
    local: Rc<MemoryEventStore>,
    /// Rows another store minted, kept with their origin identity.
    ingested: Rc<RefCell<Vec<ReplicatedEvent>>>,
}

impl LocalStore {
    fn new(identity: StoreId) -> Self {
        Self {
            local: Rc::new(MemoryEventStore::with_store_id(identity)),
            ingested: Rc::default(),
        }
    }
}

impl EventStore for LocalStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // The reference store's own read, so every `ReadOptions` field means
        // what the contract says. Its stream is `Send`; returning it where the
        // bare flavour asks for no bound is the weaker promise, which is fine.
        happenstance_core::SendEventStore::read(&*self.local, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        happenstance_core::SendEventStore::append(&*self.local, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        happenstance_core::SendEventStore::head(&*self.local).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        let ingested = self.ingested.borrow().iter().any(|row| row.id == id);
        if ingested {
            return Ok(true);
        }
        happenstance_core::SendEventStore::contains_event_id(&*self.local, id).await
    }
}

/// How [`LocalStore`]'s ingest fails: the compensation's append.
///
/// A newtype rather than `AppendError<MemoryStoreError>` itself, because
/// `IngestStore::Error` must implement [`Classify`], and this crate defines
/// neither that trait nor the core type: the orphan rule refuses the impl
/// (`error[E0117]`), exactly as it refuses a third crate's `IngestStore` impl.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct LocalStoreError(#[from] AppendError<MemoryStoreError>);

impl Classify for LocalStoreError {
    fn class(&self) -> FailureClass {
        // The in-memory store cannot fail, and a compensation's condition is
        // `None`, so none of these arrives; were one to, it is not a retry.
        FailureClass::Stop
    }
}

impl IngestStore for LocalStore {
    type Error = LocalStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        Ok(self.local.store_id())
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        let mut outcome = Ingested::default();
        for group in groups {
            // Each group lands whole or not at all. Which of its events are new
            // is decided first, under a shared borrow that ends before the
            // compensation's await (H-14); the compensation is written next;
            // and only once it has landed are the foreign rows kept. A failed
            // append therefore leaves nothing behind, and a re-delivery still
            // finds the group new and writes its answer.
            let fresh: Vec<&ReplicatedEvent> = {
                let rows = self.ingested.borrow();
                group
                    .events
                    .iter()
                    .filter(|event| rows.iter().all(|row| row.id != event.id))
                    .collect()
            };
            outcome.skipped += group.events.len() - fresh.len();
            if fresh.is_empty() {
                // SY-11: a group whose events were all re-deliveries writes no
                // compensation; the first delivery already wrote it.
                continue;
            }
            if !group.compensation.is_empty() {
                EventStore::append(self, group.compensation, None)
                    .await
                    .map_err(|error| IngestError::Store(error.into()))?;
                outcome.compensated += group.compensation.len();
            }
            outcome.appended += fresh.len();
            // The store keeps its own copy of each row; the batch stays the
            // runner's.
            self.ingested
                .borrow_mut()
                .extend(fresh.into_iter().cloned());
        }
        Ok(outcome)
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        let mut watermark = Watermark::new();
        for row in self.ingested.borrow().iter() {
            watermark.advance(row.id.store(), row.id.position());
        }
        Ok(watermark)
    }
}

// ---------------------------------------------------------------------------
// A peer that is genuinely `!Send`
// ---------------------------------------------------------------------------

/// The in-memory peer, held the way a Durable Object holds its transport.
#[derive(Debug)]
struct LocalPeer {
    inner: Rc<MemorySyncPeer>,
}

impl SyncPeer for LocalPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        happenstance_sync::SendSyncPeer::pull(&*self.inner, from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<happenstance_sync::Ack, Self::Error> {
        happenstance_sync::SendSyncPeer::push(&*self.inner, batch).await
    }

    fn limits(&self) -> PeerLimits {
        happenstance_sync::SendSyncPeer::limits(&*self.inner)
    }
}

// ---------------------------------------------------------------------------
// The instantiation is not vacuous
// ---------------------------------------------------------------------------

/// Compile-time `Send` assertions, both ways.
///
/// The negative half is the `static_assertions` construction, written out: a
/// trait with two blanket impls, one of them only for `Send` types, so naming
/// its item through a `Send` type is ambiguous (`error[E0283]`) and through a
/// `!Send` one is not. Make either local type `Send` and this module stops
/// compiling: for `LocalPeer`, swap its `Rc` for an `Arc`; for `LocalStore`,
/// swap both `Rc`s for `Arc`s *and* the `RefCell` for a `Mutex`, because
/// `Arc<RefCell<_>>` is still `!Send` (`RefCell` is `!Sync`, and `Arc<T>` is
/// `Send` only when `T: Send + Sync`).
///
/// The positive half shows that what they wrap is `Send` on its own — the two
/// reference types, and the bare `RefCell` — so it is the `Rc` that removes
/// `Send`, not something underneath it.
mod not_send {
    use core::cell::RefCell;

    use super::{LocalPeer, LocalStore};
    use happenstance_core::MemoryEventStore;
    use happenstance_sync::{MemorySyncPeer, ReplicatedEvent};

    trait AmbiguousIfSend<Marker> {
        fn some_item() {}
    }

    impl<T: ?Sized> AmbiguousIfSend<()> for T {}
    impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

    const fn is_send<T: Send>() {}

    const _: fn() = || {
        <LocalStore as AmbiguousIfSend<_>>::some_item();
        <LocalPeer as AmbiguousIfSend<_>>::some_item();
    };

    const _: () = {
        is_send::<MemoryEventStore>();
        is_send::<MemorySyncPeer>();
        is_send::<RefCell<Vec<ReplicatedEvent>>>();
    };
}

// ---------------------------------------------------------------------------
// The exchange runs
// ---------------------------------------------------------------------------

fn replicated(origin: StoreId, position: u64) -> ReplicatedEvent {
    ReplicatedEvent::new(
        EventId::new(
            origin,
            SequencePosition::new(position).expect("a test position is non-zero"),
        ),
        RecordedAt::from_millis(1_700_000_000_000),
        Event::new("CourseCapacityChanged", "{}").expect("a valid event type"),
    )
}

#[tokio::test]
async fn the_runner_step_runs_over_a_not_send_store_and_peer() {
    let origin = StoreId::from_bytes([7; 16]);
    let peer = LocalPeer {
        inner: Rc::new(MemorySyncPeer::new(origin)),
    };
    peer.inner
        .seed(EventGroup::new(
            None,
            vec![replicated(origin, 1), replicated(origin, 2)],
        ))
        .expect("an unpoisoned peer accepts a seed");
    let store = LocalStore::new(StoreId::from_bytes([9; 16]));
    let answer = [Event::new("SubscriptionReversed", "{}").expect("a valid event type")];

    // Negative control for `all_held`: before the ingest, the store holds none
    // of them, so a `contains_event_id` that always said yes would fail here.
    assert!(
        !EventStore::contains_event_id(&store, replicated(origin, 1).id)
            .await
            .expect("an in-memory probe does not fail"),
        "the store claims an event nobody has delivered"
    );

    let first = exchange(&store, &peer, None, &answer)
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(first.ingested.appended, 2, "both foreign events landed");
    assert_eq!(first.ingested.skipped, 0);
    assert_eq!(first.ingested.compensated, 1, "the group was answered once");
    assert!(first.all_held, "the store holds what it ingested");
    let head = EventStore::head(&store)
        .await
        .expect("an in-memory head does not fail");
    assert!(head.is_some(), "the compensation landed in the local log");

    // Re-delivery from the start is a skip, not a second copy, and writes no
    // second answer (SY-11).
    let again = exchange(&store, &peer, Some(&MemoryResume::start()), &answer)
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(again.ingested.appended, 0);
    assert_eq!(again.ingested.skipped, 2, "re-delivery is skipped");
    assert_eq!(
        again.ingested.compensated, 0,
        "re-delivery is not re-answered"
    );
    assert_eq!(
        EventStore::head(&store)
            .await
            .expect("an in-memory head does not fail"),
        head,
        "a re-delivery wrote to the local log"
    );

    // And the token the first exchange returned is past everything.
    let caught_up = exchange(&store, &peer, Some(&first.resume), &answer)
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(
        caught_up.ingested,
        Ingested::default(),
        "nothing is left to pull"
    );
}
