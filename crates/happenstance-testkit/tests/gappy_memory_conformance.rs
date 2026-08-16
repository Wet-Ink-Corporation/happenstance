//! The whole suite against `GappyMemoryStore`.
//!
//! It runs so that the gaps are *proved* to be the freedom VT-11 grants rather
//! than a defect asserted to be one, and it is the second instrument in this
//! workspace to run ES-9's interior-gap branch — the half of
//! `read_from_a_gap_position` that only executes where a fixture's own allocator
//! left a hole. The first is `GappedPositionStore` in
//! `tests/mutation_coverage/variants.rs`, which is CF-5's conformant control and
//! is deliberately untouched by this file: what a published, `Send`,
//! caller-strided store adds is **reach**, not coverage.
//!
//! Native only — the harness is `#[tokio::test]`. `memory` because the store is.

#![cfg(all(not(target_arch = "wasm32"), feature = "memory"))]
#![allow(clippy::unwrap_used)]

use core::num::NonZeroU64;
use std::sync::Arc;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, Query, ReadOptions, SendEventStore,
    SequencePosition, SequencedEvent,
};
use happenstance_testkit::{Capability, Fixture, GappyMemoryStore};

/// One handle onto a [`GappyFixture`]'s store.
///
/// A newtype rather than a blanket `impl EventStore for Arc<S>`, for the
/// coherence reason `MemoryHandle` records: `trait_variant` already emits a
/// blanket `impl<T: SendEventStore> EventStore for T`, and proving some
/// downstream `Arc<S>` does not implement `SendEventStore` would need negative
/// reasoning the trait solver does not have. Delegating by hand is the price.
#[derive(Debug, Clone)]
struct GappyHandle(Arc<GappyMemoryStore>);

impl SendEventStore for GappyHandle {
    type Error = <GappyMemoryStore as SendEventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.0.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
}

/// One [`GappyMemoryStore`] per instance, any number of handles onto it.
struct GappyFixture(Arc<GappyMemoryStore>);

impl GappyFixture {
    /// Prime, and greater than one, so the interior-gap branch of
    /// `read_from_a_gap_position` has a hole to read from.
    fn new() -> Self {
        Self(Arc::new(GappyMemoryStore::with_stride(
            NonZeroU64::new(7).unwrap(),
        )))
    }
}

impl Fixture for GappyFixture {
    type Store = GappyHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "GappyMemoryStore is a Vec behind an RwLock, so there is no durable \
         medium to reopen over: discarding process state is indistinguishable \
         from discarding the events",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(GappyHandle(Arc::clone(&self.0)))
    }
}

happenstance_testkit::event_store_conformance!(GappyFixture::new());
