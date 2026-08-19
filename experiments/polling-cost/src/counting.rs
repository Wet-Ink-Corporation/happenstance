//! The instrument: an `EventStore` that counts what it hands out.
//!
//! It wraps the reference in-memory store and adds nothing to the read path but
//! a counter, so what is measured is the store's own delivery rather than the
//! instrument's. Generic code here binds `EventStore`, never `SendEventStore` —
//! the weaker requirement, which accepts both flavours.

use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::pin::Pin;
use std::task::{Context, Poll};

use happenstance::{
    AppendCondition, AppendError, Event, EventId, EventStore, MemoryEventStore, MemoryStoreError,
    Query, ReadOptions, SequencePosition, SequencedEvent,
};

/// A `MemoryEventStore` that records every read it serves.
///
/// Three quantities, all **observed** rather than derived: how many `read` calls
/// were issued, how many events those reads yielded in total, and which distinct
/// positions were yielded at least once. The third is the headline's denominator
/// and the reason the two selectivity arms differ.
#[derive(Debug)]
pub struct CountingStore {
    /// The store under measurement. Public so a caller can append into it
    /// without the append being counted as a delivery.
    pub inner: MemoryEventStore,
    reads: Cell<u64>,
    delivered: Cell<u64>,
    positions: RefCell<BTreeSet<u64>>,
}

impl CountingStore {
    /// Wraps a store.
    #[must_use]
    pub fn new(inner: MemoryEventStore) -> Self {
        Self {
            inner,
            reads: Cell::new(0),
            delivered: Cell::new(0),
            positions: RefCell::new(BTreeSet::new()),
        }
    }

    /// How many `read` calls have been issued.
    #[must_use]
    pub fn reads(&self) -> u64 {
        self.reads.get()
    }

    /// How many events have been yielded, summed over every read.
    #[must_use]
    pub fn delivered(&self) -> u64 {
        self.delivered.get()
    }

    /// How many **distinct** positions have been yielded at least once.
    #[must_use]
    pub fn distinct_delivered(&self) -> u64 {
        u64::try_from(self.positions.borrow().len()).unwrap_or(u64::MAX)
    }

    /// Forgets everything counted so far, keeping the log.
    ///
    /// Used between a cell's catch-up and the phase it actually measures, so a
    /// steady-state figure is not inflated by the cold replay that preceded it.
    pub fn reset(&self) {
        self.reads.set(0);
        self.delivered.set(0);
        self.positions.borrow_mut().clear();
    }
}

/// The read stream, counting as it is pulled.
struct Counting<'a, S> {
    inner: S,
    delivered: &'a Cell<u64>,
    positions: &'a RefCell<BTreeSet<u64>>,
}

impl<S> futures_core::Stream for Counting<'_, S>
where
    S: futures_core::Stream<Item = Result<SequencedEvent, MemoryStoreError>> + Unpin,
{
    type Item = Result<SequencedEvent, MemoryStoreError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        match Pin::new(&mut this.inner).poll_next(cx) {
            Poll::Ready(Some(Ok(event))) => {
                this.delivered.set(this.delivered.get() + 1);
                this.positions.borrow_mut().insert(event.position.get());
                Poll::Ready(Some(Ok(event)))
            }
            other => other,
        }
    }
}

impl EventStore for CountingStore {
    type Error = MemoryStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl futures_core::Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.reads.set(self.reads.get() + 1);
        Counting {
            inner: Box::pin(EventStore::read(&self.inner, query, options)),
            delivered: &self.delivered,
            positions: &self.positions,
        }
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        EventStore::append(&self.inner, events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(&self.inner).await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(&self.inner, id).await
    }
}
