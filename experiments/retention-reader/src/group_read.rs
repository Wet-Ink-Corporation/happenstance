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
//! the same rows by the adapter's other port method.
//!
//! # Where it is exact, and what it refuses
//!
//! A group names its first position and no other, and SQLite names a group
//! by `coalesce(write_group, position)`, where `write_group` is the position
//! the group was **written** at. So the read is exact only where **every group
//! was written with one event**. Holding one event now is not enough: a
//! two-event group whose first row was deleted reads back as a group of one,
//! still named by the deleted position, and yielding that position would be
//! wrong.
//!
//! What is refused rather than given a made-up position:
//!
//! - a group that holds more than one event;
//! - an event this store minted whose own position is not the group's name,
//!   which is how a cut group of local events shows itself, since a locally
//!   minted id carries the position it was written at;
//! - group names that do not strictly increase;
//! - a backward read (the projection runner reads forward).
//!
//! **Not detectable:** a cut group of *ingested* events. A foreign id carries
//! its origin's position, not this store's, so nothing in the group says
//! where the survivor sits. No scenario here writes a multi-event group into
//! a store that is read through this adapter, which is the precondition, not
//! something the adapter can check.

use core::fmt;
use core::num::NonZeroUsize;
use core::pin::Pin;
use core::task::{Context, Poll};

use happenstance_core::futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventId, EventStore, Query, ReadOptions, SequencePosition,
    SequencedEvent, StoreId,
};
use happenstance_sync::WriteGroupStore;

/// A borrowed store whose `read` is answered from its write groups.
#[derive(Debug)]
pub struct GroupRead<'s, S> {
    store: &'s S,
    /// The store's own incarnation, to recognise the events it minted.
    own: StoreId,
}

impl<'s, S> GroupRead<'s, S> {
    /// Reads `store`, whose incarnation is `own`, through its write groups.
    #[must_use]
    pub const fn new(store: &'s S, own: StoreId) -> Self {
        Self { store, own }
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
    /// An event this store minted sits in a group named by another position:
    /// the group was cut, and the name is a deleted row's.
    MisnamedGroup {
        /// The position the group is named by.
        first: SequencePosition,
        /// The position the event was minted at.
        minted: SequencePosition,
    },
    /// A group's name did not rise above the one before it.
    OutOfOrder {
        /// The group's name.
        first: SequencePosition,
        /// The previous group's name, at or above it.
        after: SequencePosition,
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
            Self::MisnamedGroup { first, minted } => write!(
                f,
                "an event minted at {minted} is in the group named {first}: the group was cut"
            ),
            Self::OutOfOrder { first, after } => {
                write!(f, "the group named {first} follows the group named {after}")
            }
            Self::Backwards => f.write_str("a group read only reads forward"),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for GroupReadError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Store(error) => Some(error),
            Self::MultiEventGroup { .. }
            | Self::MisnamedGroup { .. }
            | Self::OutOfOrder { .. }
            | Self::Backwards => None,
        }
    }
}

/// How many events one `read_groups` page asks for.
const PAGE: NonZeroUsize = NonZeroUsize::MIN.saturating_add(63);

/// Every event `query` and `options` select, read page by page.
async fn load<S, E>(
    store: &S,
    own: StoreId,
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
        // The cursor advances as each group is checked, below.
        if page.groups.is_empty() {
            return Ok(selected);
        }
        for group in page.groups {
            let position = group.first();
            let [event] = group.events() else {
                return Err(GroupReadError::MultiEventGroup { first: position });
            };
            if let Some(previous) = after.filter(|&previous| previous >= position) {
                return Err(GroupReadError::OutOfOrder {
                    first: position,
                    after: previous,
                });
            }
            if event.id.store() == own && event.id.position() != position {
                return Err(GroupReadError::MisnamedGroup {
                    first: position,
                    minted: event.id.position(),
                });
            }
            after = Some(position);
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
        let (store, own) = (self.store, self.own);
        GroupStream::Loading(Box::pin(
            async move { load(store, own, &query, options).await },
        ))
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

#[cfg(test)]
mod tests {
    use happenstance_core::{Event, EventStore, Query, ReadOptions, collect};
    use happenstance_sqlite::event_store::SqliteEventStore;
    use happenstance_testkit::block_on;

    use super::{GroupRead, GroupReadError};
    use crate::prune::{TempDb, prune_outside_port};

    type Outcome = Result<(), Box<dyn core::error::Error>>;

    fn event(data: &'static str) -> Result<Event, Box<dyn core::error::Error>> {
        Ok(Event::new("Written", data.as_bytes())?)
    }

    #[test]
    fn a_group_of_two_events_is_refused() -> Outcome {
        let store = SqliteEventStore::open_in_memory()?;
        block_on(async {
            store.append(&[event("a")?, event("b")?], None).await?;
            let read = GroupRead::new(&store, store.store_id());
            let got = collect(read.read(&Query::all(), ReadOptions::new())).await;
            assert!(
                matches!(got, Err(GroupReadError::MultiEventGroup { .. })),
                "{got:?}"
            );
            Ok(())
        })
    }

    #[test]
    fn a_backward_read_is_refused() -> Outcome {
        let store = SqliteEventStore::open_in_memory()?;
        block_on(async {
            store.append(&[event("a")?], None).await?;
            let read = GroupRead::new(&store, store.store_id());
            let got = collect(read.read(&Query::all(), ReadOptions::new().backwards())).await;
            assert!(matches!(got, Err(GroupReadError::Backwards)), "{got:?}");
            Ok(())
        })
    }

    /// The survivor of a two-event group whose first row was deleted reads
    /// back as a group of one named by the deleted position. Its own id says
    /// where it was minted, so it is refused rather than misplaced.
    #[test]
    fn a_cut_group_of_local_events_is_refused() -> Outcome {
        let db = TempDb::new("group-read-cut")?;
        block_on(async {
            let store = SqliteEventStore::open(db.path())?;
            let second = store.append(&[event("a")?, event("b")?], None).await?;
            let own = store.store_id();
            let all =
                collect(GroupRead::new(&store, own).read(&Query::all(), ReadOptions::new())).await;
            let Err(GroupReadError::MultiEventGroup { first }) = all else {
                return Err(format!("a whole group of two: {all:?}").into());
            };
            drop(store);

            assert_eq!(prune_outside_port(db.path(), &[first])?, 1);
            let store = SqliteEventStore::open(db.path())?;
            let got =
                collect(GroupRead::new(&store, own).read(&Query::all(), ReadOptions::new())).await;

            assert!(
                matches!(
                    got,
                    Err(GroupReadError::MisnamedGroup { first: named, minted })
                        if named == first && minted == second
                ),
                "{got:?}"
            );
            drop(store);
            Ok::<(), Box<dyn core::error::Error>>(())
        })?;
        db.finish()?;
        Ok(())
    }
}
