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

//! # ES-11, and a wrong conclusion that is worth recording
//!
//! Both `read_result_is_stable_under_concurrent_append` and
//! `query_items_share_one_snapshot` pass. They did not always, and the reason
//! they now do is worth writing down, because the first diagnosis was wrong in a
//! way that would have cost a specification amendment.
//!
//! ES-11 requires a read to be evaluated against a state fixed **no later than
//! the first poll**. Both rules exploit the gap deliberately: poll once, append,
//! then drain. With the cursor's opening held as an inline future, that future
//! makes progress only while someone is polling it — so the snapshot was taken
//! during the *drain*, strictly after the append, and both rules failed five
//! times in five runs.
//!
//! The conclusion drawn from that was that an async driver cannot satisfy the
//! clause at all, since its first poll can only *start* the round trip. That was
//! wrong, and consistent failure is what made it look right.
//!
//! What fixes it is that the work is **handed to the runtime** at the first
//! poll and every other operation is too. `open_cursor` is spawned here;
//! `append` is spawned through `PostgresEventStore::on_runtime`. Both land in
//! the same queue, the read's task was enqueued first, and so the snapshot is
//! taken before the append runs. "Fixed no later than the first poll" becomes
//! true in the only sense available to an async adapter: the work was
//! *committed to* at that poll and no longer depends on the caller polling
//! again.
//!
//! The bridge was not built for this. It was built because `sqlx` panics
//! without a runtime in thread-local scope and the conformance suite's
//! concurrency family drives its contenders on raw threads. That it also
//! settles ES-11 is why the earlier diagnosis survived as long as it did: the
//! two failures looked structural because the thing that would have disproved
//! them had not been written yet.
//!
use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_core::Stream;
use happenstance_core::{
    Event, EventId, Query, ReadOptions, RecordedAt, SequencePosition, SequencedEvent, StoreId, Tag,
    Tags,
};
use sqlx::Row as _;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::runtime::Handle;

use crate::error::PostgresEventStoreError;
use crate::event_store::{Visibility, position_from_row};
use crate::query_sql::Param;

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
/// A step whose work was handed to the runtime rather than polled inline.
type Spawned<T> = Pin<Box<tokio::task::JoinHandle<T>>>;

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
    /// The runtime captured by the store that produced this plan.
    ///
    /// Carried rather than looked up, for the reason the store's own field
    /// documents: a read may be polled on a thread with no runtime at all --
    /// the conformance suite's concurrency family does exactly that -- and
    /// `Handle::try_current` there finds nothing.
    runtime: Option<Handle>,
    /// Whether this read carries the visibility frontier.
    visibility: Visibility,
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
    /// `Option` only so that [`Drop`] can move the transaction out. It is
    /// `Some` for the whole of the cursor's useful life.
    tx: Option<Transaction<'static, Postgres>>,
    /// The runtime this cursor's connection must be returned on.
    runtime: Option<Handle>,
}

impl PgCursor {
    /// The live transaction.
    ///
    /// # Panics
    ///
    /// Never in practice: the `Option` exists for [`Drop`] and is `Some` until
    /// the cursor is being destroyed, at which point nothing calls this.
    fn transaction(&mut self) -> &mut Transaction<'static, Postgres> {
        self.tx
            .as_mut()
            .expect("the cursor's transaction is taken only by `Drop`")
    }
}

/// Returns the pooled connection on a runtime, because `sqlx` cannot do it
/// anywhere else.
///
/// `PoolConnection::drop` calls `sqlx`'s own `rt::spawn` to hand the connection
/// back to the pool, and that **panics** when no runtime is in thread-local
/// scope (`sqlx-core-0.8.6/src/pool/connection.rs:200-210`). A read stream is
/// routinely dropped where that is true: the conformance suite's concurrency
/// family runs each contender on a raw OS thread driving `block_on`, so
/// `a_concurrent_reader_never_sees_a_partial_batch` panicked here on its first
/// run even after every other rule in the family had gone green.
///
/// Moving the transaction onto the captured runtime lets its own `Drop` run
/// where `sqlx` expects to be. The rollback is explicit rather than implicit
/// because a read holds a `REPEATABLE READ READ ONLY` transaction open, and
/// leaving it to an implicit drop makes the moment it ends depend on when the
/// spawned task happens to be scheduled — which is exactly the kind of open
/// transaction the frontier mechanism is most sensitive to.
impl Drop for PgCursor {
    fn drop(&mut self) {
        let Some(tx) = self.tx.take() else {
            return;
        };
        let Some(handle) = self.runtime.clone().or_else(|| Handle::try_current().ok()) else {
            // No runtime anywhere. Returning the connection is impossible, and
            // so is reporting it — `Drop` has no channel. Forgetting the
            // transaction leaks the pooled connection, which is strictly better
            // than the panic-inside-panic a naive drop would produce here.
            std::mem::forget(tx);
            return;
        };
        handle.spawn(async move {
            // The result is deliberately discarded: a read wrote nothing, so a
            // failed rollback changes no state a caller could observe, and
            // there is nobody left to tell.
            let _ = tx.rollback().await;
        });
    }
}

/// Where the stream is.
enum ReadState {
    /// Constructed, never polled. Holds no connection.
    Unstarted(Box<CursorPlan>),
    /// Acquiring a connection, opening the snapshot transaction, declaring the
    /// cursor, and fetching the first chunk.
    Opening(Spawned<Opened>),
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
    pub(crate) fn new(
        pool: PgPool,
        runtime: Option<Handle>,
        visibility: Visibility,
        query: &Query,
        options: ReadOptions,
    ) -> Self {
        Self {
            state: ReadState::Unstarted(Box::new(CursorPlan {
                pool,
                runtime,
                visibility,
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
                    // **Spawned, not merely constructed**, and the difference is
                    // ES-11's whole content.
                    //
                    // A future stored here and polled inline makes progress only
                    // while the caller is polling. `read_result_is_stable_under_concurrent_append`
                    // polls exactly once, gets `Pending`, appends, and only then
                    // drains — so an inline future opens its snapshot *after*
                    // that append and the read grows under the caller's feet.
                    // The clause requires the state to be fixed no later than
                    // the first poll, and handing the work to the runtime is how
                    // an async adapter honours that: from here on the snapshot
                    // is being taken whether or not anyone polls again.
                    //
                    // Laziness is not lost. Nothing was spawned, no connection
                    // taken and no transaction opened while the stream sat
                    // unpolled — which is what a stream dropped without being
                    // polled must cost, and what `read` not being `async` exists
                    // to allow.
                    //
                    // `spawn` needs a runtime, and `read` may legally be called
                    // outside one. `NoRuntime` is the honest answer there rather
                    // than a panic from inside a library.
                    // The store's captured handle first, then the caller's own,
                    // and only then an error. A read polled from a raw thread
                    // has no current runtime, which is not hypothetical: the
                    // concurrency family's contenders are exactly that.
                    let handle = plan.runtime.clone().or_else(|| Handle::try_current().ok());
                    let Some(handle) = handle else {
                        this.state = ReadState::Done;
                        return Poll::Ready(Some(Err(PostgresEventStoreError::NoRuntime)));
                    };
                    this.state = ReadState::Opening(Box::pin(handle.spawn(open_cursor(*plan))));
                }
                ReadState::Opening(mut step) => match step.as_mut().poll(cx) {
                    Poll::Pending => {
                        this.state = ReadState::Opening(step);
                        return Poll::Pending;
                    }
                    Poll::Ready(Err(join)) => {
                        // The spawned task panicked or was cancelled. Neither is
                        // an adapter defect the caller can act on, but silently
                        // ending the stream would look like an empty log.
                        this.state = ReadState::Done;
                        return Poll::Ready(Some(Err(PostgresEventStoreError::Worker(join))));
                    }
                    Poll::Ready(Ok(Ok((cursor, rows)))) => {
                        this.state = ReadState::Draining {
                            cursor,
                            rows: rows.into(),
                        };
                    }
                    Poll::Ready(Ok(Err(error))) => {
                        this.state = ReadState::Done;
                        return Poll::Ready(Some(Err(error)));
                    }
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
    // Captured before `plan` is consumed, so the cursor can return its
    // connection on the same runtime that opened it.
    let runtime = plan.runtime.clone();
    // `begin_with` rather than `begin`, because the isolation level is not
    // decoration: ES-11 requires the whole read to see one state of the store,
    // and REPEATABLE READ is how the server is asked for it.
    let mut tx = plan
        .pool
        .begin_with("BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .await?;

    let (declare, params) = declare_sql(&plan.query, plan.options, plan.visibility);
    let mut statement = sqlx::query(&declare);
    for param in &params {
        statement = match param {
            Param::EventType(value) => statement.bind(value.clone()),
            Param::Tags(values) => statement.bind(values.clone()),
        };
    }
    statement.execute(&mut *tx).await?;

    let rows = sqlx::query(&fetch_sql()).fetch_all(&mut *tx).await?;

    Ok((
        Box::new(PgCursor {
            tx: Some(tx),
            runtime,
        }),
        rows,
    ))
}

/// Fetches one chunk, taking the cursor by value and handing it back.
///
/// By value rather than by `&mut` because the state machine cannot hold a borrow
/// of something it also stores — which is the same constraint that sank the
/// naive `fetch` in this module's documentation, met one level up.
async fn fetch_chunk(mut cursor: Box<PgCursor>) -> Fetched {
    // The `&mut` borrow of the transaction begins and ends inside this call, so
    // the cursor is free to move again on the next line.
    let rows = sqlx::query(&fetch_sql())
        .fetch_all(&mut **cursor.transaction())
        .await;
    (cursor, rows)
}

/// The columns every read selects, in the order [`decode_row`] expects them.
///
/// Named once because the `SELECT` and the decoder must agree, and a positional
/// decode against a drifting projection list is a class of bug that shows up as
/// a type error only when the two happen to differ in type.
const SELECTED_COLUMNS: &str = "position, event_type, data, metadata, tags, \
                                origin_store, origin_position, recorded_at";

/// The `DECLARE` that turns a [`Query`] and its [`ReadOptions`] into a cursor,
/// and the parameters it binds.
///
/// # The frontier lives here, not in the `FETCH`
///
/// `xact_id < pg_snapshot_xmin(pg_current_snapshot())` is composed *into* this
/// statement, inside the `REPEATABLE READ` transaction the caller has already
/// opened. That transaction fixes one snapshot, so the frontier is evaluated
/// **once** and every `FETCH` draws from the same fixed set.
///
/// Evaluating it per chunk is the easy way to get the read half wrong, and it
/// fails in the direction that matters: the frontier advances between chunks, so
/// a later `FETCH` would admit rows beneath positions the caller has already
/// been handed — ES-10's violation arriving through the read path rather than
/// the write path, which is the one this whole mechanism was built to close.
fn declare_sql(
    query: &Query,
    options: ReadOptions,
    visibility: Visibility,
) -> (String, Vec<Param>) {
    let mut next = 1;
    let predicate = crate::query_sql::predicate(query, &mut next);

    let mut clauses = vec![
        predicate.sql().to_owned(),
        // The mechanism, inside the snapshot. `Visibility::Naive` yields
        // `TRUE` here, which is the whole of the negative control's difference.
        visibility.predicate().to_owned(),
    ];

    // `from` and `to` are both **inclusive**, in both directions, and both are
    // range predicates over positions rather than index seeks: the specification
    // permits gaps, this is the adapter that produces them, so a position the
    // caller names may not exist.
    //
    // Their roles do not swap with direction; their *position-order* comparisons
    // do. `from` is always the STARTING bound and `to` always the STOPPING one,
    // so reading backwards the read begins at the newest event at or below
    // `from` and stops at `to`. Copying the forward branch's `position <= to`
    // into the backward one is the bug ES-8's `Rejects:` describes one bound
    // over — it is correct reading forwards, so only
    // `read_to_under_backwards_bounds_the_older_end` sees it.
    let (start_op, stop_op) = if options.backwards {
        ("<=", ">=")
    } else {
        (">=", "<=")
    };
    if let Some(from) = options.from {
        clauses.push(format!("position {start_op} {}", as_i64(from)));
    }
    if let Some(to) = options.to {
        clauses.push(format!("position {stop_op} {}", as_i64(to)));
    }

    let order = if options.backwards { "DESC" } else { "ASC" };

    // `LIMIT` is applied by the server rather than by the stream, so a bounded
    // read stops costing rows the caller will never see. It composes with the
    // filter, not with the scan: the limit applies **after** filtering, across
    // items rather than per item.
    let limit = options
        .limit
        .map_or_else(String::new, |limit| format!(" LIMIT {limit}"));

    let sql = format!(
        "DECLARE {CURSOR_NAME} NO SCROLL CURSOR FOR \
         SELECT {SELECTED_COLUMNS} FROM event \
         WHERE {} \
         ORDER BY position {order}{limit}",
        clauses.join(" AND ")
    );
    (sql, predicate.params().to_vec())
}

/// The `FETCH` that advances it.
fn fetch_sql() -> String {
    format!("FETCH FORWARD {FETCH_CHUNK} FROM {CURSOR_NAME}")
}

/// A position as the `bigint` the schema stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// Turns one row into a contract event.
///
/// Every failure here is a **stored** value that no longer satisfies a contract
/// type — a `bigint` position that is zero or negative against a `NonZeroU64`, a
/// `text` event type or tag carrying something validation now rejects. Each gets
/// a named variant rather than a panic, because the alternative to a variant is
/// a panic in a library.
fn decode_row(row: &PgRow) -> Result<SequencedEvent, PostgresEventStoreError> {
    let position = position_from_row(row.try_get::<i64, _>("position")?)?;

    let event_type: String = row.try_get("event_type")?;
    let data: Vec<u8> = row.try_get("data")?;
    let metadata: Option<Vec<u8>> = row.try_get("metadata")?;
    let stored_tags: Vec<String> = row.try_get("tags")?;

    let mut event =
        Event::new(event_type.as_str(), data).map_err(PostgresEventStoreError::EventType)?;

    if !stored_tags.is_empty() {
        let tags = stored_tags
            .iter()
            .map(|tag| Tag::new(tag.as_str()))
            .collect::<Result<Tags, _>>()
            .map_err(PostgresEventStoreError::Tag)?;
        event = event.with_tags(tags);
    }

    // `None` and `Some(empty)` are different values and a conformance rule says
    // so, which is why this is a `map` over the `Option` rather than a
    // `unwrap_or_default`.
    if let Some(metadata) = metadata {
        event = event.with_metadata(metadata);
    }

    // The identity columns are nullable, because a replication ingest may hold
    // rows minted elsewhere. A row this store wrote always has both.
    let origin_store: Option<Vec<u8>> = row.try_get("origin_store")?;
    let origin_position: Option<i64> = row.try_get("origin_position")?;
    let id = match (origin_store, origin_position) {
        (Some(store), Some(origin)) => {
            let bytes: [u8; 16] = store
                .as_slice()
                .try_into()
                .map_err(|_| PostgresEventStoreError::MalformedIdentity { len: store.len() })?;
            EventId::new(StoreId::from_bytes(bytes), position_from_row(origin)?)
        }
        _ => return Err(PostgresEventStoreError::UnstampedEvent { position }),
    };

    let recorded_at = RecordedAt::from_millis(row.try_get::<i64, _>("recorded_at")?);
    Ok(SequencedEvent::new(position, id, recorded_at, event))
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
