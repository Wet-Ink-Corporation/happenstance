//! `EventStore::read` served from `WriteGroupStore::read_groups`, so a reader
//! can run over a SQLite store without a tokio runtime.
//!
//! # Why this exists
//!
//! `SqliteEventStore::read` defers its query to `spawn_blocking` on first
//! poll, and needs a tokio runtime to do it (`SqliteEventStoreError::NoRuntime`
//! otherwise). This crate's manifest has no `tokio`, and the owner approved it
//! that way, so every scenario drives with the testkit's `block_on`. Every
//! other SQLite port method this experiment calls (`append`, `head`,
//! `contains_event_id`, `ingest`, `watermark`, `read_groups`) runs inline under
//! the store's lock and needs no runtime.
//!
//! So a projection over a SQLite spoke reads through this adapter. It reads
//! the same rows by the adapter's other port method, and it is faithful only
//! where **every write group holds one event**: a group names its first
//! position and no other, so a later event's position cannot be recovered. A
//! group of more than one event is refused rather than given a made-up
//! position. Backward reads are refused too; the projection runner reads
//! forward.

use core::fmt;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent,
};
use happenstance_sync::WriteGroupStore;

/// A borrowed store whose `read` is answered from its write groups.
#[derive(Debug)]
pub struct GroupRead<'s, S> {
    store: &'s S,
}

impl<'s, S> GroupRead<'s, S> {
    /// Reads `store` through its write groups.
    #[must_use]
    pub const fn new(store: &'s S) -> Self {
        Self { store }
    }
}

/// Why a [`GroupRead`] could not answer.
#[derive(Debug)]
pub enum GroupReadError<E> {
    /// The store failed.
    Store(E),
    /// A write group held more than one event, so the positions after its
    /// first cannot be recovered.
    MultiEventGroup {
        /// The group's first position.
        first: SequencePosition,
    },
    /// A backward read was asked for.
    Backwards,
}

impl<E> fmt::Display for GroupReadError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Store(_) => f.write_str("the store failed"),
            Self::MultiEventGroup { first } => write!(
                f,
                "the write group at {first} holds more than one event, \
                 so its later positions are unknown"
            ),
            Self::Backwards => f.write_str("a group read only reads forward"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for GroupReadError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Store(error) => Some(error),
            Self::MultiEventGroup { .. } | Self::Backwards => None,
        }
    }
}

/// How many events one `read_groups` page asks for.
const PAGE: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

/// Every event `query` and `options` select, read page by page.
async fn load<S, E>(
    store: &S,
    query: &Query,
    options: ReadOptions,
) -> Result<Vec<SequencedEvent>, GroupReadError<E>>
where
    S: WriteGroupStore<Error = E>,
{
    if options.backwards {
        return Err(GroupReadError::Backwards);
    }
    let mut selected = Vec::new();
    let mut after = None;
    loop {
        let page = store
            .read_groups(after, PAGE)
            .await
            .map_err(GroupReadError::Store)?;
        let Some(last) = page.groups.last().map(happenstance_sync::WriteGroup::first) else {
            return Ok(selected);
        };
        for group in page.groups {
            let position = group.first();
            let [event] = group.events() else {
                return Err(GroupReadError::MultiEventGroup { first: position });
            };
            let in_range = options.from.is_none_or(|from| position >= from)
                && options.to.is_none_or(|to| position <= to);
            if in_range && query.matches(event.event.event_type(), event.event.tags()) {
                if options.limit.is_some_and(|limit| selected.len() >= limit) {
                    return Ok(selected);
                }
                // The read hands out owned events; the payload is a `Bytes`
                // handle.
                selected.push(SequencedEvent::new(
                    position,
                    event.id,
                    event.recorded_at,
                    event.event.clone(),
                ));
            }
        }
        after = Some(last);
    }
}

type Loading<'s, E> =
    Pin<Box<dyn Future<Output = Result<Vec<SequencedEvent>, GroupReadError<E>>> + 's>>;

/// The stream [`GroupRead`]'s `read` returns: the page loop, then its events.
enum GroupStream<'s, E> {
    Loading(Loading<'s, E>),
    Draining(std::vec::IntoIter<SequencedEvent>),
}

impl<E> Stream for GroupStream<'_, E> {
    type Item = Result<SequencedEvent, GroupReadError<E>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Both states are `Unpin`: a boxed future and a vector iterator.
        let this = self.get_mut();
        loop {
            match this {
                Self::Loading(future) => match future.as_mut().poll(cx) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Err(error)) => {
                        *this = Self::Draining(Vec::new().into_iter());
                        return Poll::Ready(Some(Err(error)));
                    }
                    Poll::Ready(Ok(events)) => *this = Self::Draining(events.into_iter()),
                },
                Self::Draining(events) => return Poll::Ready(events.next().map(Ok)),
            }
        }
    }
}

impl<S, E> EventStore for GroupRead<'_, S>
where
    S: EventStore<Error = E> + WriteGroupStore<Error = E>,
    E: core::error::Error + 'static,
{
    type Error = GroupReadError<E>;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // The stream owns its query: it outlives this call's borrow of it.
        let query = query.clone();
        let store = self.store;
        GroupStream::Loading(Box::pin(async move { load(store, &query, options).await }))
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        EventStore::append(self.store, events, condition)
            .await
            .map_err(|error| error.map_store(GroupReadError::Store))
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        EventStore::head(self.store)
            .await
            .map_err(GroupReadError::Store)
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        EventStore::contains_event_id(self.store, id)
            .await
            .map_err(GroupReadError::Store)
    }
}
