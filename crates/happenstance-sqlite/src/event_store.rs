//! SQLite-backed [`EventStore`](happenstance::EventStore).
//!
//! # Status: not implemented
//!
//! The type below exists to pin the shape. See the [crate
//! documentation](crate) for the decisions still open.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE event (
//!     position    INTEGER PRIMARY KEY AUTOINCREMENT, -- monotonic, gaps allowed
//!     event_type  TEXT    NOT NULL,
//!     data        BLOB    NOT NULL,
//!     metadata    BLOB,
//!     tags        BLOB    NOT NULL  -- canonical sorted encoding
//! );
//!
//! -- Tag matching needs `contains all of these tags`, which a join table
//! -- serves better than a blob scan once the log is large.
//! CREATE TABLE event_tag (
//!     position INTEGER NOT NULL REFERENCES event(position),
//!     tag      TEXT    NOT NULL,
//!     PRIMARY KEY (tag, position)
//! ) WITHOUT ROWID;
//!
//! CREATE INDEX event_type_idx ON event(event_type, position);
//! ```
//!
//! `AUTOINCREMENT` is deliberate: it guarantees positions are never reused
//! after a delete, which plain `rowid` does not, and the specification requires
//! uniqueness across the store's whole lifetime.

use futures_core::Stream;
use happenstance::{
    AppendCondition, AppendError, Event, Query, ReadOptions, SendEventStore, SequencePosition,
    SequencedEvent,
};

/// A SQLite-backed event store.
///
/// # Status: not implemented
#[derive(Debug)]
#[non_exhaustive]
pub struct SqliteEventStore {}

/// How [`SqliteEventStore`] fails.
///
/// # Status: not implemented
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqliteEventStoreError {
    /// Placeholder variant; replaced by real failure modes on implementation.
    #[error("the SQLite event store is not implemented yet")]
    Unimplemented,
}

impl SendEventStore for SqliteEventStore {
    type Error = SqliteEventStoreError;

    fn read(
        &self,
        _query: &Query,
        _options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        todo!("SQLite event store: read");
        // Unreachable, but it tells the compiler what the opaque return type is
        // so the signature is checked against the trait today rather than later.
        #[allow(unreachable_code)]
        Pending(core::marker::PhantomData)
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        todo!("SQLite event store: append")
    }
}

/// Stand-in for the real read stream, so that `read`'s signature is type-checked
/// against the trait before the implementation exists.
#[derive(Debug)]
struct Pending(core::marker::PhantomData<SqliteEventStoreError>);

impl Stream for Pending {
    type Item = Result<SequencedEvent, SqliteEventStoreError>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        _cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        core::task::Poll::Ready(None)
    }
}
