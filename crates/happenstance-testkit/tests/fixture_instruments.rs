//! The one fixture in the workspace with a durable medium behind it, and the
//! suite run against it.
//!
//! `acknowledged_writes_survive_a_reopen` (CF-14, CF-17) can only execute
//! against a fixture that supports `REOPEN`, and `MemoryFixture` and
//! `LocalFixture` both decline it for honest reasons — a `Vec` behind an
//! `RwLock` or an `Rc` has no durable medium, so "reopen" could only mean doing
//! nothing, which passes the rule vacuously, or dropping the log, which fails it
//! while the store is perfectly conformant. [`DurableFixture`] is the third
//! answer: a `Vec<SequencedEvent>` standing in for the disk, and a live
//! `MemoryEventStore` that `reopen` throws away and rebuilds from it.
//!
//! The durable side carries `SequencedEvent` rather than `Event`, and the
//! difference is the whole of VT-9's durability half. `with_events` would
//! *arrange* a store — fresh positions, fresh identities, one fresh stamp from
//! the host clock — which is what a caller wants when seeding a fixture and is
//! exactly wrong for a reopen: it would restamp every event, so
//! `recorded_time_survives_a_reopen` would fail against a fixture whose only
//! fault was that its "disk" could not express the question. `restore` reopens
//! instead, preserving every store-assigned fact, which is what a real adapter's
//! open does because it reads back rows it wrote.
//!
//! # Why the wrong implementations are no longer here
//!
//! This file used to hold two more fixtures, each driven by a hand-written
//! `#[should_panic(expected = "…")]` test: one that acknowledged before
//! recording, and one whose handles cached the log head. That is exactly the
//! shape CF-2 rejects by name — it records only *that* something failed, so a
//! mutant which fails the right rule for the wrong reason (a panic in its
//! constructor, an unrelated regression) reads as proof. Both now live in
//! `tests/mutation_coverage/mutants.rs` as `LosingFixture` and
//! `CachedHeadFixture`, registered as data with the exact set of rules each
//! fails, and driven through **every registered** rule so that "and it passes
//! everything else" is asserted rather than assumed.
//!
//! What stays here is the *control*: the correct sibling, running the whole
//! suite. It is a stronger control than a single-rule one, because it says the
//! fixture is conformant in every other respect too.
//!
//! Native only. The harness here is `#[tokio::test]`, and the fixture contract's
//! wasm32 coverage is `memory_conformance_wasm.rs`'s job.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryEventStore, MemoryStoreError, Query,
    ReadOptions, SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_testkit::{Capability, Fixture};

/// A store with a durable medium behind it.
///
/// The "disk" is a `Vec<SequencedEvent>` of everything an append **acknowledged
/// and committed**; the live store is a `MemoryEventStore` that `reopen` throws
/// away and rebuilds from that vector through
/// [`MemoryEventStore::restore`](happenstance_core::MemoryEventStore::restore).
///
/// The element type is `SequencedEvent` and not `Event`, and that is the whole
/// of VT-9's durability half rather than a detail: a disk holding `Event`s can
/// only be reopened by *replaying* them, which mints fresh positions,
/// identities and stamps — so `recorded_time_survives_a_reopen` would be
/// answered by a fixture whose medium could not express the question. See the
/// module documentation.
///
/// This is a *fixture instrument* in the sense of CF-26: it proves the rule can
/// pass, and `LosingFixture` in `tests/mutation_coverage/mutants.rs` proves it
/// can fail. It is not an adapter instrument, and the far end of the durability
/// axis — a real store that can genuinely lose a write under a real fault —
/// remains phase 8's.
#[derive(Debug)]
struct DurableFixture {
    /// The incarnation this store keeps across a reopen — the "mint once at
    /// schema creation" mechanism VT-6 permits, chosen here because it is the
    /// one that makes `recorded_time_survives_a_reopen`'s question askable. The
    /// other permitted mechanism, minting afresh on every open, is what this
    /// fixture did before and is why the swap is worth naming: the reopen rules
    /// were exercised against one mechanism before it and the other after, which
    /// is the evidence they require neither.
    store_id: StoreId,
    /// What has actually been committed. `reopen` *restores* it and nothing
    /// else — restoring rather than replaying is what keeps every
    /// store-assigned fact, which is the point of the element type.
    log: Arc<Mutex<Vec<SequencedEvent>>>,
    /// The live store. Replaced wholesale by `reopen`, so a handle taken before
    /// the call keeps the old one — which is why the rule drops its handle.
    live: Mutex<Arc<MemoryEventStore>>,
}

impl DurableFixture {
    fn new() -> Self {
        // A process-local ordinal is enough: nothing in the suite compares
        // incarnations across fixture instances, and two instances only have to
        // differ from one another.
        static NEXT: AtomicU64 = AtomicU64::new(1);
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&ordinal.to_be_bytes());
        let store_id = StoreId::from_bytes(bytes);

        Self {
            store_id,
            log: Arc::new(Mutex::new(Vec::new())),
            live: Mutex::new(Arc::new(MemoryEventStore::with_store_id(store_id))),
        }
    }
}

/// One handle onto a [`DurableFixture`]'s live store.
///
/// A newtype for the same coherence reason `MemoryHandle` is one: there is no
/// blanket `impl EventStore for Arc<S>` to lean on, and adding one would collide
/// with the blanket impl `trait_variant` emits.
#[derive(Debug)]
struct DurableHandle {
    live: Arc<MemoryEventStore>,
    log: Arc<Mutex<Vec<SequencedEvent>>>,
}

impl SendEventStore for DurableHandle {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl futures_core::Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.live.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        let position = self.live.append(events, condition).await?;
        // Recorded before the acknowledgement, which is the whole property. The
        // guard is acquired after the await and released before the return, so
        // no lock is ever held across a suspension point —
        // `clippy::await_holding_lock` is a workspace-level deny and would
        // otherwise fire here.
        //
        // Snapshotting the live store rather than the batch, and it is not
        // laziness: `append` returns one position, and the identities and times
        // of the rows it just wrote belong to the store. Only the store has
        // them, so only the store can hand the durable side something worth
        // restoring.
        let committed = self.live.snapshot();
        *self.log.lock().unwrap() = committed;
        Ok(position)
    }

    // Both of the below ask the live store rather than the log. That is still
    // the honest answer now the "disk" is a `Vec<SequencedEvent>` and could in
    // principle answer either question: the live store is what a handle is a
    // handle onto, and reading through the log would be answering from a medium
    // no caller can reach. It is also what makes the durability assertion mean
    // something, because `reopen` rebuilds the live store *from* the log, so a
    // head read through a post-reopen handle is a head over exactly what
    // survived.

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // Read through on every call, never remembered at `connect`: a handle
        // that caches the head is `CachedHeadFixture`'s declared defect in
        // `tests/mutation_coverage/mutants.rs`, and this file is its control.
        self.live.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.live.contains_event_id(id).await
    }
}

impl Fixture for DurableFixture {
    type Store = DurableHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const REOPEN: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        DurableHandle {
            live: Arc::clone(&self.live.lock().unwrap()),
            log: Arc::clone(&self.log),
        }
    }

    async fn reopen(&self) {
        let committed = self.log.lock().unwrap().clone();
        *self.live.lock().unwrap() = Arc::new(MemoryEventStore::restore(self.store_id, committed));
    }
}

// The whole suite against the durable fixture. This is the only place in a
// *harness* where `acknowledged_writes_survive_a_reopen` executes: both other
// shipped fixtures decline `REOPEN`, so before this file the rule's body had
// never run, and a typo in its expectation or a reversed argument would have
// shipped green behind a `SKIP` line.
happenstance_testkit::event_store_conformance!(
    mod_name = durable_conformance,
    fixture = DurableFixture::new()
);
