//! The pooled-cursor read stream.
//!
//! # The constraint this type exists to reconcile
//!
//! Two requirements pull in opposite directions, and neither is negotiable.
//!
//! [`EventStore::read`](happenstance_core::EventStore::read) is **not** `async`:
//! the stream sits at the top level of the return type, because that is what
//! lets the `Send` flavour mark the *stream* `Send` rather than merely the
//! future that produces it (ADR-0001, ADR-0008). So `read` cannot await, and it
//! cannot acquire a connection.
//!
//! sqlx's `fetch` **borrows** its executor:
//!
//! ```text
//! pub fn fetch<'e, 'c: 'e, E>(self, executor: E) -> BoxStream<'e, Result<DB::Row, Error>>
//! where E: Executor<'c, Database = DB>
//! ```
//!
//! The returned stream is tied to a connection that, at `read` time, does not
//! exist yet. Writing the obvious thing — acquire, then hand the borrow to
//! `fetch`, then return the result — produces a self-referential value, and the
//! compiler says so:
//!
//! ```text
//! error[E0597]: `conn` does not live long enough
//!    |
//!    |         sqlx::query("SELECT position FROM event").fetch(&mut *conn)
//!    |         ------------------------------------------------------^^^^-
//!    |         |                                                     |
//!    |         |                                                     borrowed value does not live long enough
//!    |         argument requires that `conn` is borrowed for `'1`
//!    | }
//!    | - `conn` dropped here while still borrowed
//! ```
//!
//! # The shape that works
//!
//! Do not hold a borrowed stream at all. Hold the **owner** and re-borrow it for
//! the duration of each step:
//!
//! 1. `read` captures the pool and the query and returns immediately, in state
//!    `ReadState::Unstarted`. Nothing is acquired, so nothing is borrowed.
//! 2. The first `poll_next` starts a future that acquires a connection, opens a
//!    transaction and `DECLARE`s a server-side cursor. That future **owns** the
//!    [`Transaction`], so there is no borrow to outlive.
//! 3. Every subsequent chunk is a `FETCH`, run by a future that takes the cursor
//!    by value and hands it back with the rows. The `&mut` borrow of the
//!    transaction lives entirely inside one `await`, which is a borrow the
//!    compiler can see the end of.
//!
//! That is why the state machine is written by hand instead of with
//! [`futures_util::stream::unfold`]: `unfold` would do, but it makes the type
//! unnameable, and this is precisely the type the phase-2 portfolio exists to
//! record.
//!
//! [`futures_util::stream::unfold`]: https://docs.rs/futures-util/0.3/futures_util/stream/fn.unfold.html
//!
//! # A server-side cursor, not keyset pagination
//!
//! The cheaper-looking alternative — re-issuing
//! `WHERE position > $last ORDER BY position LIMIT n` per chunk against a
//! connection borrowed only for that statement — sidesteps the borrow problem
//! entirely and is **wrong**. The specification's ES-11 requires one `read` to
//! be evaluated against a single consistent state of the store, fixed no later
//! than the first poll; a self-paginating read grows under the caller's feet.
//! `BEGIN ISOLATION LEVEL REPEATABLE READ` plus `DECLARE CURSOR` buys that
//! snapshot from the server, which is the whole reason the transaction is held
//! open across the stream's life rather than the connection alone.
//!
//! The cost is honest and belongs in the capability table: a reader occupies a
//! pooled connection *and* an open transaction for as long as the caller holds
//! the stream. A slow consumer pins a connection and holds back `VACUUM`.

use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{Query, ReadOptions, SequencedEvent};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, Transaction};

use crate::error::PostgresEventStoreError;

/// Rows per `FETCH`.
///
/// Large enough that a replay is not a round trip per event, small enough that
/// one chunk is not a memory event. Phase 10 makes it configurable if a
/// measurement says it should be.
const FETCH_CHUNK: u32 = 1024;

/// The cursor's name.
///
/// A fixed name is safe because each stream opens its own transaction on its own
/// pooled connection, and a cursor is visible only within the transaction that
/// declared it. Two concurrent reads therefore cannot collide, and the name
/// never reaches a query builder as untrusted input.
const CURSOR_NAME: &str = "happenstance_read";

/// A boxed step. Boxed so the state machine has a nameable type; `Send` because
/// [`SendEventStore`](happenstance_core::SendEventStore) requires the whole
/// stream to be.
type Step<T> = Pin<Box<dyn Future<Output = T> + Send>>;

/// What opening a cursor produces: the cursor, and the first chunk it was worth
/// asking for in the same round trip.
type Opened = Result<(Box<PgCursor>, Vec<PgRow>), PostgresEventStoreError>;

/// What one `FETCH` produces. The cursor comes back **either way** — a failed
/// fetch still owns a transaction that has to be dropped in the right place.
type Fetched = (Box<PgCursor>, Result<Vec<PgRow>, sqlx::Error>);

/// Everything `read` captured synchronously, waiting for a first poll.
#[derive(Debug)]
struct CursorPlan {
    pool: PgPool,
    query: Query,
    options: ReadOptions,
}

/// An open server-side cursor, and the transaction that owns it.
///
/// This is the type that resolves the borrow problem: the [`Transaction`] is
/// `'static` because [`PgPool::begin`] hands back a transaction that owns its
/// pooled connection outright, so the cursor can be moved into and out of each
/// step future without borrowing anything that might not outlive it.
struct PgCursor {
    tx: Transaction<'static, Postgres>,
}

/// Where the stream is.
enum ReadState {
    /// Constructed, never polled. Holds no connection.
    Unstarted(Box<CursorPlan>),
    /// Acquiring a connection, opening the snapshot transaction, declaring the
    /// cursor, and fetching the first chunk.
    Opening(Step<Opened>),
    /// A `FETCH` is in flight. The future owns the cursor and returns it.
    Fetching(Step<Fetched>),
    /// Handing out rows from the chunk already in hand.
    Draining {
        cursor: Box<PgCursor>,
        rows: VecDeque<PgRow>,
    },
    /// Exhausted or failed. Dropping the cursor drops the transaction, which
    /// rolls back — which is correct, because a read never wrote anything.
    Done,
}

/// The stream [`PostgresEventStore::read`] returns.
///
/// Lazy: constructing one performs no I/O and holds no connection. The first
/// poll acquires a pooled connection and opens a snapshot transaction, which it
/// holds until the stream is exhausted or dropped. See the [module
/// documentation](self) for why the shape is what it is.
///
/// [`PostgresEventStore::read`]: crate::event_store::PostgresEventStore
pub struct PgReadStream {
    state: ReadState,
}

// Hand-written because the boxed step futures are not `Debug`, and
// `missing_debug_implementations` is a workspace warning. The state name is the
// only part a reader debugging a stuck replay actually wants.
impl fmt::Debug for PgReadStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.state {
            ReadState::Unstarted(_) => "Unstarted",
            ReadState::Opening(_) => "Opening",
            ReadState::Fetching(_) => "Fetching",
            ReadState::Draining { .. } => "Draining",
            ReadState::Done => "Done",
        };
        f.debug_struct("PgReadStream")
            .field("state", &state)
            .finish()
    }
}

impl PgReadStream {
    /// Captures what the read needs, without doing any of it.
    pub(crate) fn new(pool: PgPool, query: &Query, options: ReadOptions) -> Self {
        Self {
            state: ReadState::Unstarted(Box::new(CursorPlan {
                pool,
                query: query.clone(),
                options,
            })),
        }
    }
}

impl Stream for PgReadStream {
    type Item = Result<SequencedEvent, PostgresEventStoreError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // `PgReadStream` holds no self-references — that is the entire point of
        // the design above, and it is what makes every field `Unpin` and lets
        // `get_mut` work without `unsafe`, which is `forbid`den here anyway.
        let this = self.get_mut();

        loop {
            // Taking the state by value is what lets a step future be moved into
            // its own poll. `Done` is the correct thing to leave behind: every
            // arm either restores a state or returns a terminal value.
            match std::mem::replace(&mut this.state, ReadState::Done) {
                ReadState::Unstarted(plan) => {
                    this.state = ReadState::Opening(Box::pin(open_cursor(*plan)));
                }
                ReadState::Opening(mut step) => match step.as_mut().poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::Opening(step);
                        return Poll::Pending;
                    }
                    Poll::Ready(Ok((cursor, rows))) => {
                        this.state = ReadState::Draining {
                            cursor,
                            rows: rows.into(),
                        };
                    }
                    Poll::Ready(Err(err)) => return Poll::Ready(Some(Err(err))),
                },
                ReadState::Fetching(mut step) => match step.as_mut().poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::Fetching(step);
                        return Poll::Pending;
                    }
                    Poll::Ready((_, Err(err))) => {
                        return Poll::Ready(Some(Err(PostgresEventStoreError::Driver(err))));
                    }
                    Poll::Ready((cursor, Ok(rows))) => {
                        if rows.is_empty() {
                            // The cursor is exhausted; dropping it here rolls the
                            // snapshot transaction back and returns the
                            // connection to the pool.
                            return Poll::Ready(None);
                        }
                        this.state = ReadState::Draining {
                            cursor,
                            rows: rows.into(),
                        };
                    }
                },
                ReadState::Draining { cursor, mut rows } => match rows.pop_front() {
                    Some(row) => {
                        this.state = ReadState::Draining { cursor, rows };
                        return Poll::Ready(Some(decode_row(&row)));
                    }
                    None => {
                        this.state = ReadState::Fetching(Box::pin(fetch_chunk(cursor)));
                    }
                },
                ReadState::Done => return Poll::Ready(None),
            }
        }
    }
}

/// Acquires a connection, opens the snapshot transaction, declares the cursor
/// and fetches the first chunk.
///
/// One future rather than three states, because all three are one round trip's
/// worth of latency apart and splitting them would buy nothing but arms.
async fn open_cursor(plan: CursorPlan) -> Opened {
    // `begin_with` rather than `begin`, because the isolation level is not
    // decoration: ES-11 requires the whole read to see one state of the store,
    // and REPEATABLE READ is how the server is asked for it.
    let mut tx = plan
        .pool
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .await?;

    let declare = declare_sql(&plan.query, plan.options);
    sqlx::query(&declare).execute(&mut *tx).await?;

    let rows = sqlx::query(&fetch_sql()).fetch_all(&mut *tx).await?;

    Ok((Box::new(PgCursor { tx }), rows))
}

/// Fetches one chunk, taking the cursor by value and handing it back.
///
/// By value rather than by `&mut` because the state machine cannot hold a borrow
/// of something it also stores — which is the same constraint that sank the
/// naive `fetch` in this module's documentation, met one level up.
async fn fetch_chunk(mut cursor: Box<PgCursor>) -> Fetched {
    // The `&mut` borrow of the transaction begins and ends inside this call, so
    // the cursor is free to move again on the next line.
    let rows = sqlx::query(&fetch_sql()).fetch_all(&mut *cursor.tx).await;
    (cursor, rows)
}

/// The `DECLARE` that turns a [`Query`] and its [`ReadOptions`] into a cursor.
fn declare_sql(_query: &Query, _options: ReadOptions) -> String {
    // Blocked on the tag-matching decision (join table against `text[]` + GIN
    // against `jsonb`), which decides both the `WHERE` clause and whether the
    // parameters can be bound or must be generated. Phase 10.
    todo!("postgres event store: DECLARE ... CURSOR FOR the query")
}

/// The `FETCH` that advances it.
fn fetch_sql() -> String {
    format!("FETCH FORWARD {FETCH_CHUNK} FROM {CURSOR_NAME}")
}

/// Turns one row into a contract event.
fn decode_row(_row: &PgRow) -> Result<SequencedEvent, PostgresEventStoreError> {
    // Every failure path here is already a variant of `PostgresEventStoreError`;
    // what is missing is the column layout, which the tag decision settles.
    todo!("postgres event store: decode a row into a SequencedEvent")
}

#[cfg(test)]
mod tests {
    use super::{PgReadStream, Postgres, Transaction};
    use sqlx::PgPool;

    /// The named phase-2 attempt, as a compiling call site rather than a claim.
    ///
    /// `SPECIFICATION.md` §4.2 asserts that `sqlx::Pool::begin()` returns
    /// `Transaction<'static, Postgres>`, "which owns its pooled connection and
    /// borrows nothing: a driver with every opportunity to hand back a borrowed
    /// handle chose not to". This function is what makes that testable by the
    /// compiler instead of by reading.
    ///
    /// Two things have to hold for it to build. The `'static` in the return type
    /// must be satisfiable from a transaction begun through a *borrow* of the
    /// pool — if `begin` were `fn begin(&'a self) -> Transaction<'a, DB>` the
    /// inner block would not typecheck. And the pool must be droppable while the
    /// transaction is still alive, which is the operational meaning of "owns its
    /// pooled connection".
    ///
    /// It is never called: there is no database. Naming it in a test is what
    /// instantiates the check.
    async fn begin_borrows_nothing_from_the_pool(
        pool: PgPool,
    ) -> Result<Transaction<'static, Postgres>, sqlx::Error> {
        let batch = {
            let borrowed: &PgPool = &pool;
            borrowed.begin().await?
        };
        drop(pool);
        Ok(batch)
    }

    #[test]
    fn pool_begin_yields_an_owned_transaction() {
        let _ = begin_borrows_nothing_from_the_pool;
    }

    /// The stream must be `Send` or `SendEventStore` cannot be implemented, and
    /// the obligation is written at the definition so it is discharged before
    /// monomorphisation rather than by auto-trait leakage.
    #[test]
    fn read_stream_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<PgReadStream>();
    }
}
