//! The ingest family compiles, and runs, against a `!Send` store.
//!
//! The store holds the oracle through an `Rc`, as the Durable Object holds its
//! storage, implements only the **bare** `EventStore` and `IngestStore`, and
//! is mounted with the blocking emitter, which imposes no `Send` bound
//! anywhere. If a rule, the fixture trait or an emitter grew a `Send`
//! requirement, this file would stop compiling.

#![cfg(not(target_arch = "wasm32"))]

use std::rc::Rc;

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryStoreError, Query, ReadOptions,
    SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::{
    IngestError, IngestGroup, IngestStore, Ingested, MemoryIngestStore, Watermark,
};

// Only the bare flavour of each port is imported, so method calls are never
// ambiguous; the oracle's `Send` flavour is reached by full path below.
use happenstance_sync_testkit::IngestFixture;
use happenstance_testkit::Capability;

/// A fixture whose store is genuinely `!Send`.
#[derive(Debug, Default)]
struct LocalFixture {
    store: Rc<MemoryIngestStore>,
}

impl LocalFixture {
    fn new() -> Self {
        Self::default()
    }
}

impl IngestFixture for LocalFixture {
    type Store = LocalStore;

    /// Declined for the oracle's reason, which this fixture reaches through an
    /// `Rc`: the store writes a call in one step and cannot fail inside it.
    const MID_GROUP_FAULT: Capability = Capability::declined(
        "this fixture reaches the in-memory oracle through an `Rc`, and the oracle writes a \
         whole ingest in one step with an error type that cannot be built, so nothing can fail \
         between two rows of a group",
    );

    async fn connect(&self) -> Self::Store {
        LocalStore {
            inner: Rc::clone(&self.store),
        }
    }
}

/// A handle onto the shared store, through an `Rc`.
#[derive(Debug)]
struct LocalStore {
    inner: Rc<MemoryIngestStore>,
}

impl EventStore for LocalStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        happenstance_core::SendEventStore::read(&*self.inner, query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        happenstance_core::SendEventStore::append(&*self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        happenstance_core::SendEventStore::head(&*self.inner).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        happenstance_core::SendEventStore::contains_event_id(&*self.inner, id).await
    }
}

impl IngestStore for LocalStore {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        Ok(self.inner.store_id())
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        happenstance_sync::SendIngestStore::ingest(&*self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        happenstance_sync::SendIngestStore::watermark(&*self.inner).await
    }
}

/// The mount below only proves something if the store is `!Send`: a trait
/// with two blanket impls, one only for `Send` types, is ambiguous
/// (`error[E0283]`) when named through a `Send` type.
const _: fn() = || {
    trait AmbiguousIfSend<Marker> {
        fn some_item() {}
    }
    impl<T: ?Sized> AmbiguousIfSend<()> for T {}
    impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}
    <LocalStore as AmbiguousIfSend<_>>::some_item();
};

happenstance_sync_testkit::ingest_store_conformance!(
    mod_name = local_ingest_conformance,
    emit = happenstance_sync_testkit::emit_ingest_blocking,
    fixture = super::LocalFixture::new()
);
