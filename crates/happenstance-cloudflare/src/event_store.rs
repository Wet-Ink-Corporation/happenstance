//! A Durable Object-backed [`EventStore`], in the bare `!Send` flavour.
//!
//! # Status: not implemented
//!
//! The types below are real and the bodies are `todo!()`. See the [crate
//! documentation](crate) for what that buys.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE event (
//!     position    INTEGER PRIMARY KEY AUTOINCREMENT,
//!     event_type  TEXT    NOT NULL,
//!     data        BLOB    NOT NULL,
//!     metadata    BLOB,
//!     tags        BLOB    NOT NULL
//! );
//!
//! CREATE TABLE event_tag (
//!     position INTEGER NOT NULL REFERENCES event(position),
//!     tag      TEXT    NOT NULL,
//!     PRIMARY KEY (tag, position)
//! ) WITHOUT ROWID;
//! ```
//!
//! Identical to the SQLite adapter's, and that is the point: the storage engine
//! *is* SQLite. Everything this crate exists to test is above the schema — the
//! flavour, the error type, and the fact that the cursor is not a snapshot.
//!
//! # Why the whole object is one consistency boundary
//!
//! A Durable Object is single-threaded and has exclusive ownership of its
//! storage, so there is no second writer to lose a race to. Append conditions
//! could therefore be evaluated with a plain `SELECT` followed by an `INSERT`
//! and still be atomic — the object cannot yield between them unless the code
//! awaits. That is a *stronger* guarantee than any other adapter in the
//! workspace gets, and it is the reason this skeleton is a poor instrument for
//! the position-allocation axis and a good one for the flavour axis.

use std::num::NonZeroUsize;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, EventStore, InvalidEventType, Query, ReadOptions,
    SequencePosition, SequencedEvent,
};

use crate::sql_storage::{SqlCursor, SqlError, SqlRow, SqlStorage, SqlValue};

/// An event store backed by a Durable Object's SQL storage.
///
/// `!Send` and `!Sync`, because a Durable Object is a single-threaded actor and
/// its storage is reached through a JS handle. It therefore implements
/// [`EventStore`] directly rather than `SendEventStore`, and it is the only
/// adapter in the workspace that does.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct CloudflareEventStore {
    /// The object's SQL storage. Cloning aliases it rather than copying it.
    sql: SqlStorage,
}

impl Default for CloudflareEventStore {
    fn default() -> Self {
        Self::new(SqlStorage::new())
    }
}

impl CloudflareEventStore {
    /// Wraps a Durable Object's SQL storage.
    ///
    /// In phase 9 this takes `worker::SqlStorage` off `State::storage().sql()`.
    #[must_use]
    pub fn new(sql: SqlStorage) -> Self {
        Self { sql }
    }

    /// Creates the schema if it is absent.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the DDL fails.
    pub fn migrate(&self) -> Result<(), CloudflareEventStoreError> {
        todo!("phase 9: CREATE TABLE IF NOT EXISTS")
    }
}

/// How [`CloudflareEventStore`] fails for its own reasons.
///
/// # This type is the ES-6 instrument
///
/// It is `!Send` and `!Sync`, transitively, because [`SqlError::Thrown`] carries
/// a handle to a JS-side value. It is the only error type in the workspace that
/// is: `MemoryStoreError` is uninhabited and `SqliteEventStoreError` has a
/// single placeholder variant, so neither could fail a `Send + Sync` bound if
/// that bound were wrong. See the crate documentation for the compiled result of
/// adding one.
///
/// Note what is *not* here: there is no `ConditionViolated` variant. The DCB
/// concurrency signal travels in
/// [`AppendError::ConditionViolated`], lifted out of the adapter's error type by
/// the contract itself. That is load
/// bearing for ES-6 — see the crate documentation's third finding.
#[derive(Debug, Clone, thiserror::Error)]
#[non_exhaustive]
pub enum CloudflareEventStoreError {
    /// The Workers SQL API failed.
    #[error(transparent)]
    Sql(#[from] SqlError),

    /// A row came back with the wrong number of columns, which means the schema
    /// on disk is not the schema this build expects.
    #[error("a stored row had {actual} columns, expected {expected}")]
    RowShape {
        /// Columns the adapter's `SELECT` asked for.
        expected: usize,
        /// Columns the row actually carried.
        actual: usize,
    },

    /// A column held a [`SqlValue`] variant the adapter cannot decode. Workers
    /// SQL has no boolean and no date, so every mismatch is a real one.
    #[error("column `{column}` held an unusable SQL type")]
    ColumnType {
        /// The column that could not be decoded.
        column: &'static str,
    },

    /// A stored `event_type` no longer satisfies the contract's validation.
    /// Reachable when a row predates a tightening of the rules.
    #[error("a stored event type failed validation on read-back")]
    StoredEventType(#[from] InvalidEventType),

    /// A stored position is not a valid [`SequencePosition`], which is anything
    /// below one or above 2^53 — the latter because Workers SQL widens integers
    /// through a JS number on the way out.
    #[error("the stored position {raw} is not a usable sequence position")]
    StoredPosition {
        /// The rejected value.
        raw: i64,
    },
}

impl EventStore for CloudflareEventStore {
    type Error = CloudflareEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        // No `+ Send`, and no `async`. Both matter, and for once they are cheap
        // rather than merely required: `SqlStorage::exec` is synchronous, so
        // there is nothing to await before the cursor exists, and the object is
        // single-threaded, so there is nothing to send it to.
        //
        // `exec` is still deferred to the first poll. ES-9 requires the stream
        // to be lazy, and here that is not a nicety either: a cursor opened at
        // `read` time and never polled would be a live cursor the object had to
        // keep valid across every subsequent await.
        SqlRowStream {
            state: StreamState::Deferred {
                sql: self.sql.clone(),
                query: query.clone(),
                options,
            },
        }
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        todo!("phase 9: evaluate the condition and INSERT ... RETURNING position")
    }
}

/// The stream [`CloudflareEventStore::read`] returns.
///
/// A real cursor-backed stream, not a placeholder: it holds the storage handle
/// until the first poll, then the live [`SqlCursor`], and it is `!Send` because
/// both of those are. Nothing about it is `Pin`-sensitive — the state machine is
/// written by hand precisely so that it is not a coroutine and its `Send`-ness
/// is decided by its fields rather than by inference.
#[derive(Debug)]
#[non_exhaustive]
pub struct SqlRowStream {
    state: StreamState,
}

/// Where the stream has got to.
#[derive(Debug)]
enum StreamState {
    /// `exec` has not run yet. ES-9's laziness lives here.
    Deferred {
        /// The storage to execute against.
        sql: SqlStorage,
        /// Cloned rather than borrowed, so the stream outlives the caller's
        /// query without a lifetime on the stream type.
        query: Query,
        /// `Copy`, so nothing is cloned.
        options: ReadOptions,
    },
    /// A cursor is open.
    Draining {
        /// The live cursor.
        cursor: SqlCursor,
        /// What is left of [`ReadOptions::limit`], or `None` when unlimited.
        remaining: Option<usize>,
    },
    /// Terminal.
    Done,
}

impl Stream for SqlRowStream {
    type Item = Result<SequencedEvent, CloudflareEventStoreError>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        loop {
            match core::mem::replace(&mut this.state, StreamState::Done) {
                StreamState::Deferred {
                    sql,
                    query,
                    options,
                } => {
                    let (statement, bindings) = render_read(&query, options);
                    match sql.exec(&statement, &bindings) {
                        Ok(cursor) => {
                            this.state = StreamState::Draining {
                                cursor,
                                remaining: options.limit.map(NonZeroUsize::get),
                            };
                        }
                        Err(err) => return Poll::Ready(Some(Err(err.into()))),
                    }
                }

                StreamState::Draining {
                    mut cursor,
                    remaining,
                } => {
                    if remaining == Some(0) {
                        return Poll::Ready(None);
                    }

                    if let Err(err) = check_cursor_still_valid(&cursor) {
                        return Poll::Ready(Some(Err(err)));
                    }

                    match cursor.next_row() {
                        None => return Poll::Ready(None),
                        Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),
                        Some(Ok(row)) => {
                            let decoded = decode_row(&row);
                            this.state = StreamState::Draining {
                                cursor,
                                remaining: remaining.map(|left| left.saturating_sub(1)),
                            };
                            return Poll::Ready(Some(decoded));
                        }
                    }
                }

                StreamState::Done => return Poll::Ready(None),
            }
        }
    }
}

/// Rejects a cursor whose storage moved under it.
///
/// Cloudflare documents that a `SqlStorageCursor` held across an `await` "does
/// not provide a stable snapshot of query results". A lazy stream is exactly
/// that, so the check belongs on every poll rather than once at open. It is the
/// clearest example in this crate of a **capability** limit rather than a type
/// error: the port's signature is satisfied either way, and only a conformance
/// rule can tell the difference.
fn check_cursor_still_valid(cursor: &SqlCursor) -> Result<(), CloudflareEventStoreError> {
    let len = cursor.source_len()?;
    if cursor.offset() > len {
        return Err(SqlError::CursorInvalidated.into());
    }
    Ok(())
}

/// Renders a [`Query`] and [`ReadOptions`] into one statement and its bindings.
///
/// One statement, because the object is the consistency boundary and a
/// multi-statement read would be no more atomic than a single one.
fn render_read(_query: &Query, _options: ReadOptions) -> (String, Vec<SqlValue>) {
    todo!("phase 9: render the DCB query into SQL over event/event_tag")
}

/// Decodes one raw row into a [`SequencedEvent`].
fn decode_row(_row: &SqlRow) -> Result<SequencedEvent, CloudflareEventStoreError> {
    todo!("phase 9: decode position, event_type, data, metadata, tags")
}
