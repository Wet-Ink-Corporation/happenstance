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
    EventGroup, IngestGroup, IngestStore, Ingested, MemoryPeerError, MemoryResume, MemorySyncPeer,
    PeerLimits, Pulled, PushBatch, ReplicatedEvent, SyncPeer, Watermark,
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
/// the peer, ingests through [`IngestStore`], and then asks the store it already
/// has through [`EventStore::contains_event_id`] — the membership question
/// `IngestStore` deliberately does not carry.
async fn exchange<S, P>(
    store: &S,
    peer: &P,
    from: Option<&P::Resume>,
) -> Result<Exchanged<P::Resume>, Failure>
where
    S: EventStore + IngestStore,
    P: SyncPeer,
{
    let Pulled { batch, resume, .. } = peer.pull(from).await?;
    let groups: Vec<IngestGroup<'_>> = batch
        .groups
        .iter()
        .map(|group| IngestGroup::new(&group.events, &[]))
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
        // No `+ Send`: the bare flavour is what lets this stream be built
        // inside a `!Send` store.
        let selected: Vec<Result<SequencedEvent, MemoryStoreError>> = self
            .local
            .snapshot()
            .into_iter()
            .filter(|event| query.matches(event.event_type(), event.tags()))
            .filter(|event| options.from.is_none_or(|from| event.position >= from))
            .map(Ok)
            .collect();
        Replay(selected.into_iter())
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

impl IngestStore for LocalStore {
    type Error = AppendError<MemoryStoreError>;

    fn store_id(&self) -> StoreId {
        self.local.store_id()
    }

    async fn ingest(&self, groups: &[IngestGroup<'_>]) -> Result<Ingested, Self::Error> {
        let mut outcome = Ingested::default();
        for group in groups {
            // The borrow ends before the compensation's await (H-14).
            let fresh = {
                let mut rows = self.ingested.borrow_mut();
                let mut fresh = 0;
                for event in group.events {
                    if rows.iter().any(|row| row.id == event.id) {
                        outcome.skipped += 1;
                    } else {
                        // The store keeps the row; the batch stays the runner's.
                        rows.push(event.clone());
                        fresh += 1;
                    }
                }
                fresh
            };
            outcome.appended += fresh;
            // SY-11: a group whose events were all re-deliveries writes no
            // compensation; the first delivery already wrote it.
            if fresh > 0 && !group.compensation.is_empty() {
                EventStore::append(self, group.compensation, None).await?;
                outcome.compensated += group.compensation.len();
            }
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

/// A stream over a snapshot, `!Send` only in that it never leaves this file.
struct Replay(std::vec::IntoIter<Result<SequencedEvent, MemoryStoreError>>);

impl Stream for Replay {
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(
        mut self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(self.0.next())
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
/// `!Send` one is not. Delete the `Rc` from either local type and this module
/// stops compiling. The positive half shows the two things they wrap are `Send`
/// themselves, so it is the `Rc` that removes it and not something underneath.
mod not_send {
    use super::{LocalPeer, LocalStore};
    use happenstance_core::MemoryEventStore;
    use happenstance_sync::MemorySyncPeer;

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

    let first = exchange(&store, &peer, None)
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(first.ingested.appended, 2, "both foreign events landed");
    assert_eq!(first.ingested.skipped, 0);
    assert!(first.all_held, "the store holds what it ingested");

    // Re-delivery from the start is a skip, not a second copy (SY-11).
    let again = exchange(&store, &peer, Some(&MemoryResume::start()))
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(again.ingested.appended, 0);
    assert_eq!(again.ingested.skipped, 2, "re-delivery is skipped");

    // And the token the first exchange returned is past everything.
    let caught_up = exchange(&store, &peer, Some(&first.resume))
        .await
        .expect("an in-memory exchange does not fail");
    assert_eq!(
        caught_up.ingested,
        Ingested::default(),
        "nothing is left to pull"
    );
}
