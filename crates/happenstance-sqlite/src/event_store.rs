//! SQLite-backed [`SendEventStore`].
//!
//! # Status: bodies unimplemented, types real
//!
//! Every operation is `todo!()`. The *types* are not: the connection is a real
//! [`rusqlite::Connection`], the error enum wraps [`rusqlite::Error`], and
//! [`SqliteReadStream`] is the state machine the real read path will use. A
//! skeleton that stubs its associated types has stubbed the only part of it a
//! type checker can disagree with, so nothing here is a placeholder.
//!
//! # Why the stream is a hand-written state machine
//!
//! `rusqlite` is synchronous. The only correct way to call it from an async
//! context is [`tokio::task::spawn_blocking`], and `spawn_blocking` **panics**
//! when there is no runtime in thread-local scope. Meanwhile
//! [`EventStore::read`](happenstance_core::EventStore::read) is deliberately
//! *not* `async` — it returns the stream at the top level so that the `Send`
//! flavour can mark the *stream* `Send` rather than merely the future that
//! produces it (ADR-0001, ADR-0008).
//!
//! Put those together and the consequence is forced: `read` runs on whatever
//! thread called it, possibly outside any runtime, so it must not spawn. The
//! spawn has to be deferred to the first `poll_next`, which by definition runs
//! under an executor. **Laziness stops being a nicety and becomes load-bearing**
//! — it is what makes the two constraints compatible at all.
//!
//! One residual risk survives that, and it is why
//! [`SqliteEventStoreError::NoRuntime`] exists: a `poll` under a *non-tokio*
//! executor (`futures::executor::block_on`, say) is still runtime-less.
//! [`tokio::runtime::Handle::try_current`] turns that from a panic into an
//! ordinary stream error, which is what a lazy stream's contract already
//! promises — failures surface as `Err` items rather than up front.
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

use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::vec;

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, Event, InvalidEventType, InvalidTag, Query, ReadOptions,
    SendEventStore, SequencePosition, SequencedEvent,
};
use rusqlite::Connection;
use tokio::runtime::{Handle, TryCurrentError};
use tokio::task::{JoinError, JoinHandle};

/// How many rows one `spawn_blocking` hop fetches.
///
/// The point of paging at all is that a replay of a million events must not be
/// buffered, which is the promise [`EventStore::read`](happenstance_core::EventStore::read)
/// makes. The value is a placeholder until it is measured.
const PAGE_SIZE: usize = 512;

/// A SQLite-backed event store.
///
/// The connection lives behind a [`Mutex`] because [`rusqlite::Connection`] is
/// [`Send`] but **not** [`Sync`]: without the mutex, `&SqliteEventStore` would
/// not be `Send`, and every future in the [`SendEventStore`] flavour captures
/// `&self`. The mutex is what buys `Self: Sync`, and `Self: Sync` is what makes
/// the `Send` flavour implementable at all. It also means this adapter
/// **serialises its writers** by construction — that is the shape it is here to
/// represent, not an accident.
///
/// The [`Arc`] is not for sharing the store; it is so that a
/// [`SqliteReadStream`] can outlive the `&self` borrow that produced it, which
/// it must, because `read` is not `async` and hands the stream back to the
/// caller.
#[derive(Debug, Clone)]
pub struct SqliteEventStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteEventStore {
    /// Wraps an already-open connection.
    ///
    /// The caller is responsible for having applied the schema; use
    /// [`open`](Self::open) to have that done.
    #[must_use]
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// Opens (creating if absent) a store at `path` and applies the schema.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteEventStoreError::Sqlite`] if the file cannot be opened
    /// or the schema cannot be applied.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteEventStoreError> {
        let connection = Connection::open(path)?;
        Self::migrate(&connection)?;
        Ok(Self::new(connection))
    }

    /// Opens a private in-memory store and applies the schema.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteEventStoreError::Sqlite`] if SQLite refuses the
    /// connection or the schema cannot be applied.
    pub fn open_in_memory() -> Result<Self, SqliteEventStoreError> {
        let connection = Connection::open_in_memory()?;
        Self::migrate(&connection)?;
        Ok(Self::new(connection))
    }

    /// Applies the schema in the module documentation.
    fn migrate(_connection: &Connection) -> Result<(), SqliteEventStoreError> {
        todo!("SQLite event store: schema migration")
    }
}

/// How [`SqliteEventStore`] fails.
///
/// Every variant names something `rusqlite` or the surrounding runtime can
/// actually produce. Append-condition violations are **not** here: they travel
/// through [`AppendError::ConditionViolated`], so a caller can tell "rebuild the
/// decision model and retry" from "something broke" without knowing which
/// adapter it holds.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqliteEventStoreError {
    /// The driver failed: I/O, `SQLITE_BUSY`, a constraint, a bad statement.
    #[error("SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A thread panicked while holding the connection mutex.
    ///
    /// Carried as a unit variant rather than wrapping
    /// [`std::sync::PoisonError`], because that type is generic over the guard
    /// and the guard borrows the connection — it is neither `'static` nor
    /// `Send`, and [`EventStore::Error`](happenstance_core::EventStore::Error)
    /// requires `'static`.
    #[error("the SQLite connection mutex was poisoned by a panicking thread")]
    ConnectionPoisoned,

    /// The blocking task carrying a query panicked or was cancelled.
    #[error("the blocking SQLite task did not complete: {0}")]
    Worker(#[from] JoinError),

    /// A stream was polled outside a tokio runtime, so no blocking task could
    /// be spawned.
    ///
    /// See the [module documentation](self) for why this is an error rather
    /// than the panic `spawn_blocking` would otherwise raise.
    #[error("no tokio runtime is available to run the blocking SQLite query: {0}")]
    NoRuntime(#[from] TryCurrentError),

    /// A stored row carried a position SQLite accepted and the contract does
    /// not: [`SequencePosition`] wraps a `NonZeroU64`, so zero and negatives
    /// are unrepresentable.
    #[error("stored position {0} is not a valid sequence position")]
    InvalidPosition(i64),

    /// A stored row carried an event type that no longer validates.
    #[error("stored event type is invalid: {0}")]
    StoredEventType(#[from] InvalidEventType),

    /// A stored row carried a tag that no longer validates.
    #[error("stored tag is invalid: {0}")]
    StoredTag(#[from] InvalidTag),
}

impl SendEventStore for SqliteEventStore {
    type Error = SqliteEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        // Nothing is executed here on purpose. See the module documentation:
        // `read` may legally be called with no runtime in scope, and
        // `spawn_blocking` panics there.
        SqliteReadStream {
            state: ReadState::Idle(Box::new(ReadCursor {
                connection: Arc::clone(&self.connection),
                query: query.clone(),
                options,
                resume_from: options.from,
                remaining: options.limit,
                finished: false,
            })),
        }
    }

    async fn append(
        &self,
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        todo!("SQLite event store: append")
    }
}

/// The stream [`SqliteEventStore::read`](SendEventStore::read) returns.
///
/// # Why this type is `Send`
///
/// Nothing here is `Send` by accident, and each piece earns it separately:
///
/// * the cursor holds `Arc<Mutex<Connection>>`, and `Mutex<T>: Sync` whenever
///   `T: Send` — so wrapping the `!Sync` [`Connection`] is what makes the whole
///   cursor `Send`;
/// * [`JoinHandle<T>`] is `Send` when `T` is, and `T` here is the cursor plus a
///   `Result<Page, SqliteEventStoreError>`, whose error wraps `rusqlite::Error`
///   ([`Send`] + [`Sync`]) and [`JoinError`];
/// * the drain state holds `vec::IntoIter<SequencedEvent>`, `Send` because
///   [`SequencedEvent`] is.
///
/// No `rusqlite` handle that borrows the connection — `Statement`, `Rows`,
/// `Transaction` — ever appears in a field, and that is the load-bearing part:
/// all three are `!Send`, so holding one across the `poll_next` boundary would
/// cost the stream its `Send`-ness and with it the `SendEventStore` impl.
/// Confining them to the inside of the blocking closure is not a style choice.
///
/// The type is also [`Unpin`] — every field is — so `poll_next` needs no pin
/// projection and no `unsafe`, which matters because `unsafe_code` is
/// `forbid`den workspace-wide.
#[derive(Debug)]
pub struct SqliteReadStream {
    state: ReadState,
}

/// Where a [`SqliteReadStream`] is in its life.
#[derive(Debug)]
enum ReadState {
    /// No query in flight; the next poll spawns one.
    Idle(Box<ReadCursor>),
    /// A blocking fetch is running on a `spawn_blocking` thread.
    Fetching(JoinHandle<FetchOutcome>),
    /// Yielding rows already fetched, with the cursor parked for the next page.
    Draining {
        cursor: Box<ReadCursor>,
        rows: vec::IntoIter<SequencedEvent>,
    },
    /// Terminal: exhausted, or errored and not resumable.
    Done,
}

/// What one blocking hop hands back: the cursor it borrowed, and its result.
///
/// The cursor makes the round trip because `spawn_blocking` demands a `'static`
/// closure, so the only way to mutate it on the blocking thread is to move it
/// there and back.
type FetchOutcome = (Box<ReadCursor>, Result<Page, SqliteEventStoreError>);

/// One page of rows, plus whether the query is spent.
#[derive(Debug)]
struct Page {
    rows: Vec<SequencedEvent>,
    exhausted: bool,
}

/// Everything the blocking thread needs to fetch the next page.
#[derive(Debug)]
struct ReadCursor {
    connection: Arc<Mutex<Connection>>,
    query: Query,
    options: ReadOptions,
    /// Where the next page resumes, **inclusive** — the same sense as
    /// [`ReadOptions::from`], which is what seeds it.
    ///
    /// The name matters. It was `resume_after` and it was seeded from an
    /// *inclusive* `from` and then advanced to `last.position`, which is two
    /// different senses in one field: page two would have re-read the last row
    /// of page one, once per page boundary. `AppendCondition::after` is the
    /// exclusive one in this contract and `ReadOptions::from` is the inclusive
    /// one, and they sit two types apart — mixing them is the easiest mistake
    /// in the port and this field made it. Invisible today only because
    /// [`ReadCursor::fetch_page`] is `todo!()`.
    resume_from: Option<SequencePosition>,
    /// What is left of [`ReadOptions::limit`], or `None` for unlimited.
    remaining: Option<usize>,
    finished: bool,
}

impl ReadCursor {
    /// Runs one page's worth of SQL. Called only on a blocking thread.
    fn fetch_page(&mut self) -> Result<Page, SqliteEventStoreError> {
        let _connection = self
            .connection
            .lock()
            .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
        let _budget = self.remaining.map_or(PAGE_SIZE, |left| left.min(PAGE_SIZE));
        todo!(
            "SQLite event store: page query for {:?} {:?}",
            self.query,
            self.options
        )
    }

    /// Folds a fetched page back into the cursor's position and budget.
    ///
    /// `resume_from` stays inclusive, so it must step *strictly past* the last
    /// row — and "past" is direction-dependent, which is why this is not a
    /// `+ 1`. Running out of positions in either direction means the log has no
    /// more rows that way, so the cursor is spent rather than wrapped.
    fn advance(&mut self, page: &Page) {
        let mut exhausted_by_position = false;
        if let Some(last) = page.rows.last() {
            let next = if self.options.backwards {
                // No `SequencePosition::prev`: positions are `NonZeroU64`, so
                // stepping below `FIRST` is the same fact as being spent.
                SequencePosition::new(last.position.get().saturating_sub(1))
            } else {
                last.position.next()
            };
            exhausted_by_position = next.is_none();
            self.resume_from = next;
        }
        if let Some(remaining) = self.remaining.as_mut() {
            *remaining = remaining.saturating_sub(page.rows.len());
        }
        self.finished = page.exhausted || exhausted_by_position || self.remaining == Some(0);
    }
}

impl Stream for SqliteReadStream {
    type Item = Result<SequencedEvent, SqliteEventStoreError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Safe without projection because `Self: Unpin`; see the type's docs.
        let this = Pin::into_inner(self);

        loop {
            // Taking the state by value is what lets the cursor be *moved* into
            // the `'static` closure `spawn_blocking` demands. `Done` is the
            // right placeholder: every arm either restores a live state or is
            // genuinely terminal.
            match std::mem::replace(&mut this.state, ReadState::Done) {
                ReadState::Idle(cursor) => {
                    if cursor.finished {
                        return Poll::Ready(None);
                    }
                    // The deferred spawn. This is the line that could not have
                    // been written inside `read`.
                    let runtime = match Handle::try_current() {
                        Ok(runtime) => runtime,
                        Err(err) => return Poll::Ready(Some(Err(err.into()))),
                    };
                    this.state = ReadState::Fetching(runtime.spawn_blocking(move || {
                        let mut cursor = cursor;
                        let page = cursor.fetch_page();
                        (cursor, page)
                    }));
                }
                ReadState::Fetching(mut handle) => match Pin::new(&mut handle).poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::Fetching(handle);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(join)) => return Poll::Ready(Some(Err(join.into()))),
                    Poll::Ready(Ok((_cursor, Err(err)))) => return Poll::Ready(Some(Err(err))),
                    Poll::Ready(Ok((mut cursor, Ok(page)))) => {
                        cursor.advance(&page);
                        this.state = ReadState::Draining {
                            cursor,
                            rows: page.rows.into_iter(),
                        };
                    }
                },
                ReadState::Draining { cursor, mut rows } => match rows.next() {
                    Some(event) => {
                        this.state = ReadState::Draining { cursor, rows };
                        return Poll::Ready(Some(Ok(event)));
                    }
                    None => this.state = ReadState::Idle(cursor),
                },
                ReadState::Done => return Poll::Ready(None),
            }
        }
    }
}
