//! The one fixture in the workspace with a durable medium behind it, and the
//! suite run against it.
//!
//! `acknowledged_writes_survive_a_reopen` (CF-14, CF-17) can only execute
//! against a fixture that supports `REOPEN`, and `MemoryFixture` and
//! `LocalFixture` both decline it for honest reasons — a `Vec` behind an
//! `RwLock` or an `Rc` has no durable medium, so "reopen" could only mean doing
//! nothing, which passes the rule vacuously, or dropping the log, which fails it
//! while the store is perfectly conformant. [`DurableFixture`] is the third
//! answer: a `Vec<Event>` standing in for the disk, and a live
//! `MemoryEventStore` that `reopen` throws away and rebuilds from it.
//!
//! `MemoryEventStore::with_events` assigns dense positions from 1, which is
//! exactly what `append` assigned in the first place, so a reopen reproduces the
//! *positions* rather than merely the payloads — which is the half the rule's
//! second assertion checks.
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

use std::sync::{Arc, Mutex};

use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryEventStore, MemoryStoreError, Query,
    ReadOptions, SendEventStore, SequencePosition, SequencedEvent,
};
use happenstance_testkit::{Capability, Fixture};

/// A store with a durable medium behind it.
///
/// The "disk" is a `Vec<Event>` of everything an append **acknowledged and
/// committed**; the live store is a `MemoryEventStore` that `reopen` throws away
/// and rebuilds from that vector.
///
/// This is a *fixture instrument* in the sense of CF-26: it proves the rule can
/// pass, and `LosingFixture` in `tests/mutation_coverage/mutants.rs` proves it
/// can fail. It is not an adapter instrument, and the far end of the durability
/// axis — a real store that can genuinely lose a write under a real fault —
/// remains phase 8's.
#[derive(Debug)]
struct DurableFixture {
    /// What has actually been committed. `reopen` replays it and nothing else.
    log: Arc<Mutex<Vec<Event>>>,
    /// The live store. Replaced wholesale by `reopen`, so a handle taken before
    /// the call keeps the old one — which is why the rule drops its handle.
    live: Mutex<Arc<MemoryEventStore>>,
}

impl DurableFixture {
    fn new() -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
            live: Mutex::new(Arc::new(MemoryEventStore::new())),
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
    log: Arc<Mutex<Vec<Event>>>,
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
        self.log.lock().unwrap().extend(events.iter().cloned());
        Ok(position)
    }

    // Both of the below ask the live store rather than the log, and that is the
    // honest answer rather than the convenient one: the "disk" here is a
    // `Vec<Event>`, which carries neither positions nor identities, so it cannot
    // answer either question — only the store that replayed it can. That is also
    // what makes the durability assertion mean something, because `reopen`
    // rebuilds the live store *from* the log, so a head read through a
    // post-reopen handle is a head over exactly what survived.

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
        *self.live.lock().unwrap() = Arc::new(MemoryEventStore::with_events(committed));
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
