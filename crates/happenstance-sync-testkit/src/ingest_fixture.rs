//! What a store adapter hands the ingest family, and the reference
//! implementation of it.

use alloc::sync::Arc;
use core::future::Future;

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryStoreError, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    IngestError, IngestGroup, Ingested, MemoryIngestStore, SendIngestStore, Watermark,
};

/// One isolated backing store that both appends and ingests, and a way to
/// reach it.
///
/// One fixture instance is one isolated backing store, and each
/// [`connect`](Self::connect) is one handle onto it: the event-store family's
/// contract, one port over. A rule calls the opener it is given for a fresh
/// instance, so no rule sees another's events.
///
/// # No `Send`, on purpose
///
/// Not `trait_variant`-derived, and bounded on the bare `EventStore` and
/// `IngestStore`, so a `!Send` store, the Durable Object's, is a valid fixture.
/// This crate's `tests/local_ingest_conformance.rs` mounts one.
///
/// # Capabilities arrive as defaulted items
///
/// No rule in the family needs a capability yet, so the trait declares none.
/// One arrives as a defaulted associated `Capability` constant and a defaulted
/// method that arms it, which is additive to every implementation written
/// before it, as `happenstance_testkit::Fixture`'s fault capabilities are.
pub trait IngestFixture {
    /// The store under test: an event store that also ingests.
    type Store: happenstance_core::EventStore + happenstance_sync::IngestStore;

    /// A handle onto this fixture's one backing store.
    ///
    /// An implementation in test code may panic when the environment cannot
    /// connect, as the event-store family's fixtures do: the suite has nothing
    /// to report about a store it never reached.
    fn connect(&self) -> impl Future<Output = Self::Store>;
}

/// The reference [`IngestFixture`], over [`MemoryIngestStore`].
///
/// Every ingest rule is proven against this before it is allowed to fail an
/// adapter: a rule red here is a wrong rule, not a wrong store. Each instance
/// owns one fresh store, with an incarnation of its own, and each
/// [`connect`](IngestFixture::connect) is a new handle onto it.
#[derive(Debug, Clone, Default)]
pub struct MemoryIngestFixture {
    /// Shared by every handle, so two handles see one log.
    store: Arc<MemoryIngestStore>,
}

impl MemoryIngestFixture {
    /// A fixture over a fresh in-memory store.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl IngestFixture for MemoryIngestFixture {
    type Store = MemoryIngestHandle;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // Ready rather than `async`: a handle is a refcount bump, and this
        // says so where a real fixture's `connect` does I/O.
        core::future::ready(MemoryIngestHandle {
            store: Arc::clone(&self.store),
        })
    }
}

/// One handle onto a [`MemoryIngestFixture`]'s store.
///
/// A type of its own because the orphan rule refuses `impl EventStore for
/// Arc<MemoryIngestStore>` in this crate: both the trait and `Arc` are
/// foreign, and `Arc` is not `#[fundamental]`. `Arc` because the store is
/// `Send + Sync` and the handle keeps it so, which is what lets it run under
/// the tokio emitter.
#[derive(Debug, Clone)]
pub struct MemoryIngestHandle {
    store: Arc<MemoryIngestStore>,
}

impl SendEventStore for MemoryIngestHandle {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        SendEventStore::read(&*self.store, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        SendEventStore::append(&*self.store, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        SendEventStore::head(&*self.store).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        SendEventStore::contains_event_id(&*self.store, id).await
    }
}

impl SendIngestStore for MemoryIngestHandle {
    type Error = MemoryStoreError;

    fn store_id(&self) -> StoreId {
        self.store.store_id()
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        SendIngestStore::ingest(&*self.store, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        SendIngestStore::watermark(&*self.store).await
    }
}
