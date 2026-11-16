//! A second handle onto one [`MemoryIngestStore`].
//!
//! `MemoryIngestStore` is not `Clone`, [`StoreSyncPeer::new`] takes its event
//! half and its ingest half by value, and no `impl EventStore for Arc<S>`
//! exists anywhere (coherence with `trait_variant`'s blanket impl refuses it).
//! So this crate does what the testkit's `MemoryHandle` does: a local newtype
//! over an `Arc`, implementing the foreign traits, which the orphan rule
//! allows.
//!
//! Only the `Send` flavours are named in this module, as `MemoryIngestStore`
//! implements them; the blanket impls give generic code bound on the bare
//! flavours the rest.
//!
//! [`StoreSyncPeer::new`]: happenstance_sync::StoreSyncPeer::new

use core::num::NonZeroUsize;
use std::sync::Arc;

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, MemoryStoreError, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent, StoreId,
};
use happenstance_sync::write_group::GroupOrderError;
use happenstance_sync::{
    GroupPage, IngestError, IngestGroup, Ingested, MemoryIngestStore, SendIngestStore,
    SendWriteGroupStore, Watermark,
};

/// A cheap handle onto one in-memory log: cloning it gives a second handle
/// onto the **same** log, not a copy.
#[derive(Debug, Clone, Default)]
pub struct Shared {
    // Shared and immutable at this level: the store does its own locking, and
    // every handle a scenario clones (the peer's event half, its ingest half,
    // the instrument's inner store) must see one log.
    store: Arc<MemoryIngestStore>,
}

impl Shared {
    /// A fresh, empty store with its own incarnation.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// A fresh, empty store answering to `store_id`.
    #[must_use]
    pub fn with_store_id(store_id: StoreId) -> Self {
        Self {
            store: Arc::new(MemoryIngestStore::with_store_id(store_id)),
        }
    }

    /// This store's incarnation, without the port's `async` and `Result`.
    #[must_use]
    pub fn id(&self) -> StoreId {
        self.store.store_id()
    }
}

impl SendEventStore for Shared {
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

impl SendIngestStore for Shared {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        SendIngestStore::store_id(&*self.store).await
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

impl SendWriteGroupStore for Shared {
    type Error = GroupOrderError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        SendWriteGroupStore::read_groups(&*self.store, after, max_events).await
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{Event, Query, ReadOptions, SendEventStore, collect};
    use happenstance_testkit::block_on;

    use super::Shared;

    #[test]
    fn shared_is_a_second_handle_onto_one_log() -> Result<(), Box<dyn core::error::Error>> {
        let first = Shared::new();
        let second = first.clone();
        block_on(async {
            let landed = first
                .append(&[Event::new("Written", &b"{}"[..])?], None)
                .await?;
            let read = collect(second.read(&Query::all(), ReadOptions::new())).await?;
            assert_eq!(
                read.iter().map(|e| e.position).collect::<Vec<_>>(),
                [landed]
            );
            assert_eq!(first.id(), second.id());
            Ok(())
        })
    }
}
