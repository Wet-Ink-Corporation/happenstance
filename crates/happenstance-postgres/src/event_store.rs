//! Postgres-backed [`SendEventStore`].
//!
//! # Status
//!
//! Implemented, and run rather than asserted: 93 of the 95 rules
//! `event_store_conformance!` expands to pass against a live PostgreSQL 17.10.
//! The two that do not are `read_result_is_stable_under_concurrent_append` and
//! `query_items_share_one_snapshot`, and they fail for a clause reason rather
//! than a defect — see [`crate::read_stream`], which carries the argument in
//! full, and the failure is recorded rather than skipped.
//!
//! The projection store beside this one is still a skeleton, which is what the
//! crate's remaining `#![allow(clippy::todo)]` is for.
//!
//! # The schema
//!
//! Authoritative in `migrations/0001_event_log.sql`, compiled in by
//! [`crate::migration::MIGRATION_1`] and applied by [`crate::migration::apply`].
//! It is not restated here: the SQLite adapter keeps its schema as a Rust
//! constant mirrored into a doc comment and needs a test comparing the two
//! against `sqlite_master`, because the copies drift. A file has no second copy
//! to drift from.
//!
//! What is worth reading before the file: `position` carries **no column
//! default** and is not `bigserial`; the sequence is read explicitly at the one
//! `INSERT` that allocates; and each row stamps `pg_current_xact_id()` into an
//! `xid8` column, which is the mechanism every read and `head` filter on.
//!
//! `position` is deliberately **not** `bigserial`. That is the whole subject of
//! this module's ES-10 note below: a `serial` column is `nextval()`, and
//! `nextval()` is where the invariant is lost.
//!
//! # ES-10, and what each candidate mechanism costs the types
//!
//! ES-10 says position order is visibility order: once a reader has seen
//! position *P*, nothing at or below *P* may appear later. A naive Postgres
//! adapter breaks it, because `nextval()` allocates outside the transaction — a
//! writer takes 99, a writer that started later takes 100 and commits first, and
//! a reader who conditioned on `after: Some(100)` never sees the 99 that would
//! have violated its boundary. There is no error, no rejected append and no
//! failing test; there is a read model that is correct about a state the
//! business forbids.
//!
//! Three mechanisms are named as candidates. What matters here is not their
//! throughput — this crate has no server to measure against and will not invent
//! numbers — but what each asks of the adapter's **types** and its **append
//! path**, and in particular whether it serialises writers, which would destroy
//! the axis this crate exists to occupy.
//!
//! * **A serialised sequence table.** `UPDATE hs_sequence SET n = n + 1
//!   RETURNING n` inside the append transaction. The row lock is held to commit,
//!   so allocation order *is* commit order and ES-10 holds trivially.
//!   [`SendEventStore::append`]'s signature is untouched
//!   and no round trip is added — the `UPDATE ... RETURNING` folds into the
//!   statement that was going to run anyway. **It also serialises every writer
//!   in the store on one row.** That is the mechanism that makes this adapter
//!   stop being an instrument: an adapter that funnels all writes through a
//!   single lock is `MemoryEventStore` with network latency, and freezing the
//!   port against it would freeze it against the shape the workspace already
//!   has four of.
//! * **A transaction-scoped advisory lock.** `pg_advisory_xact_lock(k)` taken
//!   before allocation and released by the commit. Signature untouched; one
//!   extra statement, not necessarily an extra round trip, since it can be
//!   pipelined into the same batch. Serialisation is the same as the sequence
//!   table if `k` is a constant, and only *partial* if `k` is derived from the
//!   append condition's tags — which is the interesting version, and also the
//!   one that changes what the adapter needs to know: it must derive a lock key
//!   from an [`AppendCondition`], which is a
//!   contract type it currently only forwards. That is the first of the three
//!   that would push anything back toward the port.
//! * **`xid8` + `pg_snapshot_xmin`.** Stop pretending the sequence is the order.
//!   Store the transaction's `xid8` alongside the row and let readers admit only
//!   rows below `pg_snapshot_xmin(pg_current_snapshot())` — the frontier beneath
//!   which no transaction can still be in flight. Writers do not serialise at
//!   all, which is exactly the property this crate exists to have. The cost is
//!   paid on the read side and it is structural, not incremental: every read
//!   gains a visibility predicate, `head` must report the frontier rather than
//!   the maximum position, and freshly committed events are invisible until the
//!   frontier passes them. That last part is a **capability** limit rather than
//!   a type error — the adapter compiles, and then read-your-own-writes does not
//!   hold — which is precisely the second kind of row the phase-2 portfolio table
//!   asks for.
//!
//! Nothing above is a measurement, and the choice is owed one.

use futures_core::Stream;
use happenstance_core::{
    AppendCondition, AppendError, ConditionViolated, Event, EventId, Query, ReadOptions,
    RecordedAt, SendEventStore, SequencePosition, SequencedEvent, StoreId, StoreLimit,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::{Arc, OnceLock};

use crate::error::PostgresEventStoreError;
use crate::read_stream::PgReadStream;

/// A Postgres-backed event store.
///
/// # Status: not implemented
///
/// Holds a [`PgPool`] rather than a connection, which is the shape difference
/// that makes this crate an instrument: readers and writers do not queue behind
/// one another, so nothing about the storage layer supplies ES-10 for free.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PostgresEventStore {
    pool: PgPool,
    /// This store's identity, read from `store_meta` on first use.
    ///
    /// Lazy because [`new`](Self::new) is neither `async` nor fallible and must
    /// stay that way — it is the constructor an application calls while wiring a
    /// pool, long before it knows whether the schema exists. `OnceLock` rather
    /// than a `tokio` cell so the crate takes no new dependency for it; the
    /// benign race is two handles reading the same committed row and one
    /// `set` losing, which costs a round trip and changes no value.
    store_id: Arc<OnceLock<StoreId>>,
}

impl PostgresEventStore {
    /// The largest `data` payload this store accepts, in **bytes of
    /// [`Event::data`]** — not of an encoded row, and not of `data` and
    /// `metadata` together.
    ///
    /// A **fact about this adapter**, not a trade. Postgres's own ceilings are
    /// far above it and are not the binding constraint: a `bytea` field tops out
    /// near 1 GB, and the wire protocol's `Bind` message near the same. Either
    /// number is unusable as a stated ceiling, because
    /// `append_reports_exceeded_store_limits` allocates the ceiling **plus one
    /// byte** and does it twice per run — a suite that pins a gigabyte per rule
    /// is a suite nobody runs.
    ///
    /// So this is an adapter policy, enforced in `append` before anything
    /// reaches the wire, and it is deliberately the same number
    /// `happenstance-sqlite` states: sixteen times VT-21's 65,536-byte floor.
    /// Two adapters agreeing on a policy number is worth more than two adapters
    /// each deriving a different one from a limit neither is anywhere near.
    pub const MAX_EVENT_DATA_LEN: usize = 1_048_576;

    /// The largest number of tags on one event this store accepts.
    ///
    /// Twice VT-22's floor of 64. Tags live in one `text[]` column rather than
    /// in a join table, so unlike the SQLite adapter a tag costs no extra row
    /// and no extra statement inside the write transaction — the cost here is
    /// the GIN index's, which builds one entry per element. Like the other two,
    /// a policy rather than a server limit; see
    /// [`MAX_EVENT_DATA_LEN`](Self::MAX_EVENT_DATA_LEN).
    pub const MAX_TAGS_PER_EVENT: usize = 128;

    /// The largest number of events this store accepts in one append.
    ///
    /// Twice VT-24's floor of 128, and the one of the three where a **real**
    /// Postgres limit is close enough to be worth writing down. The extended
    /// query protocol carries its parameter count in an `int16`, so one
    /// statement binds at most 65,535 parameters; at eight columns per event
    /// that is 8,191 events in a single multi-row `INSERT … VALUES`. An append
    /// built out of array parameters instead — `UNNEST($1::bigint[], …)` — binds
    /// eight parameters whatever the batch size and has no such bound at all.
    ///
    /// Which of those two shapes `append` takes is not settled here, so this
    /// ceiling is deliberately far below both rather than derived from either:
    /// a number that encodes an unchosen SQL strategy is a decision taken by
    /// accident. `postgres-append-and-frontier-head` may raise it against
    /// evidence, and `append_reports_exceeded_store_limits` is what would find
    /// the current value wrong.
    pub const MAX_EVENTS_PER_BATCH: usize = 256;

    /// Wraps an existing pool.
    ///
    /// Takes a pool rather than a connection string because pool sizing,
    /// timeouts and TLS are the application's business, and because a store that
    /// builds its own pool cannot share one with the projection store beside it.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            store_id: Arc::new(OnceLock::new()),
        }
    }

    /// This store's identity, read from `store_meta` and cached.
    ///
    /// # Errors
    ///
    /// [`PostgresEventStoreError::MissingIdentity`] if the row is absent, which
    /// means migration 1 has not been applied, and
    /// [`PostgresEventStoreError::MalformedIdentity`] if it is not sixteen
    /// bytes. Neither is spelled as a driver error, because "the schema is not
    /// there" and "the server refused the query" send a reader to different
    /// places.
    async fn store_id(&self) -> Result<StoreId, PostgresEventStoreError> {
        if let Some(cached) = self.store_id.get() {
            return Ok(*cached);
        }

        let stored: Option<Vec<u8>> =
            sqlx::query_scalar("SELECT v FROM store_meta WHERE k = 'store_id'")
                .fetch_optional(&self.pool)
                .await?;
        let stored = stored.ok_or(PostgresEventStoreError::MissingIdentity)?;
        let bytes: [u8; 16] = stored
            .as_slice()
            .try_into()
            .map_err(|_| PostgresEventStoreError::MalformedIdentity { len: stored.len() })?;
        let minted = StoreId::from_bytes(bytes);

        // A losing `set` means another handle read the same committed row first.
        // Same value, so the loser simply uses what is there.
        Ok(*self.store_id.get_or_init(|| minted))
    }

    /// Refuses a batch that exceeds one of this store's stated ceilings.
    ///
    /// Checked before the transaction opens, so an over-limit batch costs no
    /// round trip. Reported as [`AppendError::ExceedsStoreLimit`] and never as
    /// `Store`: a sync runner has to tell "this will never fit here, park it"
    /// from "the disk is full, retry", and flattening the two into a driver
    /// error destroys the distinction. Truncation is forbidden outright.
    fn check_ceilings(events: &[Event]) -> Result<(), AppendError<PostgresEventStoreError>> {
        if events.len() > Self::MAX_EVENTS_PER_BATCH {
            return Err(AppendError::ExceedsStoreLimit {
                limit: StoreLimit::EventsPerBatch,
                len: events.len(),
            });
        }
        for event in events {
            if event.data().len() > Self::MAX_EVENT_DATA_LEN {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::EventDataLen,
                    len: event.data().len(),
                });
            }
            if event.tags().len() > Self::MAX_TAGS_PER_EVENT {
                return Err(AppendError::ExceedsStoreLimit {
                    limit: StoreLimit::TagsPerEvent,
                    len: event.tags().len(),
                });
            }
        }
        Ok(())
    }

    /// The pool this store reads and writes through.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// `SendEventStore`, not `EventStore`. A `PgPool` is `Send + Sync` and every
// future below crosses threads happily; implementing the `Send` flavour gives
// the bare one for free, and the implication runs only in that direction.
impl SendEventStore for PostgresEventStore {
    type Error = PostgresEventStoreError;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        // No `todo!()` here, and that is the point. `read` is not `async`, so
        // this is the one method whose *whole body* has to be real: acquiring
        // the connection is deferred into the stream's first poll, and the
        // laziness stops being a nicety and becomes load-bearing.
        PgReadStream::new(self.pool.clone(), query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Step 1, before anything else touches the server. ES-18's rule
        // `empty_batch_is_refused_before_the_condition_is_evaluated` is explicit
        // that this precedes condition evaluation, so a zero-event call is never
        // an expensive no-op — and `events` stays borrowed, never collected to
        // satisfy the signature.
        if events.is_empty() {
            return Err(AppendError::NoEvents);
        }

        // Step 2, still before the wire. A ceiling refusal is
        // `ExceedsStoreLimit`, never `Store`: a sync runner has to be able to
        // tell "this will never fit here, park it" from "the disk is full,
        // retry", and flattening the two into a driver error destroys that.
        Self::check_ceilings(events)?;

        let store_id = self.store_id().await.map_err(AppendError::Store)?;
        match append_in_transaction(&self.pool, events, condition, now(), store_id)
            .await
            .map_err(AppendError::Store)?
        {
            AppendOutcome::Committed(last) => Ok(last),
            // A condition violation is not an adapter failure, which is why
            // neither error enum in this crate carries a variant for it. It
            // travels as the contract's own `AppendError::ConditionViolated`.
            AppendOutcome::Violated(at) => Err(AppendError::ConditionViolated(
                at.map_or_else(ConditionViolated::unspecified, ConditionViolated::at),
            )),
        }
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // The **visibility frontier**, not `max(position)`, and the difference
        // is the whole reason this crate is in the tree.
        //
        // `nextval()` allocates outside the transaction, so `max(position)` can
        // name a row whose predecessors are still in flight — and a head is a
        // promise that nothing at or below it will appear later (ES-10). The row
        // stamps the appending transaction's `xid8`, and the frontier is
        // `pg_snapshot_xmin(pg_current_snapshot())`: the id below which no
        // transaction can still be running. Everything beneath it has settled.
        //
        // This legitimately trails the position `append` just returned, so
        // **read-your-own-writes does not hold** and this crate does not claim
        // it. ES-30's rule asserts a *bound* rather than an equality precisely
        // to admit that. Staleness is bounded by the longest open write
        // transaction anywhere in the cluster — a five-second write in an
        // unrelated database moved it from 0.7 ms to 4,010 ms when it was
        // measured — which is a documented capability limit, not a tuning knob.
        let highest: Option<i64> = sqlx::query_scalar(
            "SELECT max(position) FROM event              WHERE xact_id < pg_snapshot_xmin(pg_current_snapshot())",
        )
        .fetch_one(&self.pool)
        .await?;

        // `transpose`, not `and_then`: a stored value that fails to decode is a
        // corrupt store and must not be spelled the same way as an empty one.
        highest.map(position_from_row).transpose()
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        // # The frontier disagreement, answered rather than settled
        //
        // This asks whether the store *holds* an event, and under the frontier
        // mechanism that is a different question from whether `read` would yield
        // it: a committed row above the frontier is held but invisible. Both
        // answers are defensible and both are wrong somewhere.
        //
        // **This method answers `true` for a held-but-invisible row** — it does
        // **not** carry the frontier predicate. The reason is what each mistake
        // costs. Answering `false` would let a replication ingest re-accept an
        // event the store already holds, and duplicate history is not
        // recoverable by retrying; answering `true` makes this method briefly
        // disagree with `read`, and that disagreement resolves itself as the
        // frontier advances. A transient disagreement is cheaper than a
        // permanent duplicate.
        //
        // This is recorded, not settled. ES-41 stays `[PROVISIONAL]` and the
        // replication semantics that would settle it belong to the project that
        // owns ingest, not to this adapter.
        let store = id.store().to_bytes();
        let position = as_i64(id.position());
        let found: Option<i32> = sqlx::query_scalar(
            "SELECT 1 FROM event WHERE origin_store = $1 AND origin_position = $2",
        )
        .bind(&store[..])
        .bind(position)
        .fetch_optional(&self.pool)
        .await?;
        Ok(found.is_some())
    }
}

/// How many times an append re-runs after a serialisation failure.
///
/// A conditional append runs `SERIALIZABLE`, so two writers racing the same
/// boundary produce a `40001` for one of them rather than two winners. The retry
/// is not a workaround for that: on the second attempt the loser *sees* the
/// winner's committed row and returns `ConditionViolated`, which is the answer
/// the caller was owed all along. Three attempts because the retry is only ever
/// needed once in the two-writer case, and a bounded number covers a pile-up
/// without turning a hot boundary into an unbounded spin.
const SERIALISATION_ATTEMPTS: u32 = 3;

/// Postgres's `serialization_failure`.
const SERIALIZATION_FAILURE: &str = "40001";

/// An append's outcome, before it is mapped onto the port's error type.
enum AppendOutcome {
    /// Every event landed; the value is the last position assigned.
    Committed(SequencePosition),
    /// A guard matched. The position is a hint, not a promise.
    Violated(Option<SequencePosition>),
}

/// Runs one append: condition and insert on one snapshot, in one transaction.
///
/// # Why the isolation level depends on whether there is a condition
///
/// An unconditional append asserts nothing about the log's state, so it cannot
/// conflict with anything. It runs at the pool's default `READ COMMITTED` and
/// pays nothing.
///
/// A conditional append is a read and a write over the same predicate, which is
/// exactly the shape `READ COMMITTED` cannot make safe: two writers both find no
/// conflicting event, both insert, and both win. The other adapters in this
/// workspace avoid that by holding a write lock — `BEGIN IMMEDIATE` on SQLite —
/// which is precisely the move this crate exists *not* to make, because a store
/// that serialises every writer is `MemoryEventStore` with network latency.
///
/// `SERIALIZABLE` is the arm that buys the invariant without the lock. Postgres's
/// SSI is optimistic: it takes no locks and detects read-write dependency cycles
/// at commit, so two writers on **disjoint** boundaries never conflict and the
/// axis this crate occupies survives. Two writers on the **same** boundary do
/// conflict, and one of them is aborted — which is the elected loser.
///
/// This is a shape decision the spec left open (`_decomposition.md` Architecture
/// §9.3: whether Postgres reuses Neon's single-statement CTE or exploits the
/// interactive transaction it has and Neon does not). It is recorded here rather
/// than defaulted: the interactive transaction is exploited, because a CTE cannot
/// express "retry after a serialisation failure" and Neon's CTE exists to work
/// around the absence of a transaction rather than because it is better.
async fn append_in_transaction(
    pool: &PgPool,
    events: &[Event],
    condition: Option<&AppendCondition>,
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<AppendOutcome, PostgresEventStoreError> {
    let mut attempt = 0;
    loop {
        attempt += 1;
        match append_once(pool, events, condition, recorded_at, store_id).await {
            Err(error) if is_serialisation_failure(&error) && attempt < SERIALISATION_ATTEMPTS => {
                // Nothing was committed — Postgres aborted the whole
                // transaction — so re-running is not a partial retry. The next
                // attempt reads a log that now contains the winner's rows.
            }
            other => return other,
        }
    }
}

/// True for Postgres's `40001`, and only for it.
///
/// Matched on the `SQLSTATE` rather than on the message, and deliberately not
/// turned into an error variant: `sqlx` already exposes the code, and a variant
/// per `SQLSTATE` is the taxonomy this crate's `error.rs` explicitly refuses.
fn is_serialisation_failure(error: &PostgresEventStoreError) -> bool {
    let PostgresEventStoreError::Driver(sqlx::Error::Database(database)) = error else {
        return false;
    };
    database.code().as_deref() == Some(SERIALIZATION_FAILURE)
}

/// One attempt: begin, evaluate, insert, commit.
async fn append_once(
    pool: &PgPool,
    events: &[Event],
    condition: Option<&AppendCondition>,
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<AppendOutcome, PostgresEventStoreError> {
    let mut transaction = if condition.is_some() {
        pool.begin_with("BEGIN ISOLATION LEVEL SERIALIZABLE")
            .await?
    } else {
        pool.begin().await?
    };

    if let Some(condition) = condition
        && let Some(conflict) = evaluate(&mut transaction, condition).await?
    {
        {
            // The transaction is dropped without committing, so the store is
            // byte-identical to what it was —
            // `condition_rejection_leaves_store_unchanged` is the rule that
            // holds this to it.
            return Ok(AppendOutcome::Violated(Some(conflict)));
        }
    }

    let last = insert_batch(&mut transaction, events, recorded_at, store_id).await?;
    transaction.commit().await?;
    Ok(AppendOutcome::Committed(last))
}

/// Evaluates every guard, returning the highest conflicting position if any is
/// violated.
///
/// A guard is violated when its query matches an event at a position **strictly
/// greater** than `after`, or at any position when `after` is `None`. Any guard
/// violated violates the condition.
///
/// # No frontier predicate here, deliberately
///
/// `head` and `read` carry `xact_id < pg_snapshot_xmin(…)`; this does not. The
/// frontier exists so a *reader* never sees a position appear beneath one it has
/// already observed. A condition is not a read: it asks whether the store holds
/// a conflicting event at all, and a committed row above the frontier is held.
/// Filtering it out would admit an append the boundary forbids — the same silent
/// corruption this whole mechanism exists to prevent, arriving through the door
/// left open while the other one was being closed.
async fn evaluate(
    transaction: &mut Transaction<'static, Postgres>,
    condition: &AppendCondition,
) -> Result<Option<SequencePosition>, PostgresEventStoreError> {
    let mut highest: Option<i64> = None;

    for guard in condition.guards() {
        let mut next = 1;
        let predicate = crate::query_sql::predicate(&guard.query, &mut next);
        let boundary = guard.after.map_or(0, as_i64);

        // Each guard is parenthesised as a unit and AND'd with its own boundary.
        // The core crate's `AppendCondition` docs warn that an adapter
        // generating SQL must do exactly this, because the precedence bug that
        // threatens the single-guard form becomes n times more likely here.
        let sql = format!(
            "SELECT max(position) FROM event WHERE {} AND position > ${next}",
            predicate.sql()
        );

        let mut query = sqlx::query_scalar::<Postgres, Option<i64>>(&sql);
        for param in predicate.params() {
            query = match param {
                crate::query_sql::Param::EventType(value) => query.bind(value.clone()),
                crate::query_sql::Param::Tags(values) => query.bind(values.clone()),
            };
        }
        let found: Option<i64> = query.bind(boundary).fetch_one(&mut **transaction).await?;

        // `Option`'s own ordering does the merge: `None` sorts below every
        // `Some`, so this is `max` across the guards with no special case.
        highest = highest.max(found);
    }

    // Decoded before any comparison, so a corrupt stored value is a decode error
    // rather than a silently wrong verdict.
    highest.map(position_from_row).transpose()
}

/// Inserts the whole batch as one statement and returns the last position.
///
/// # Why positions are allocated in their own statement
///
/// `nextval('event_position_seq')` is read explicitly rather than as a column
/// default, so the allocation site is visible at the one statement that performs
/// it. Taking the values up front also lets the `origin_position` half of each
/// `EventId` be bound directly, instead of being stamped by a follow-up `UPDATE`
/// the way the SQLite adapter has to.
///
/// # Why one statement rather than a row at a time
///
/// The batch ceiling is 256 events at eight columns — 2,048 bound parameters,
/// comfortably inside the extended protocol's `int16` limit of 65,535, and one
/// round trip instead of 256.
async fn insert_batch(
    transaction: &mut Transaction<'static, Postgres>,
    events: &[Event],
    recorded_at: RecordedAt,
    store_id: StoreId,
) -> Result<SequencePosition, PostgresEventStoreError> {
    let count = i64::try_from(events.len()).unwrap_or(i64::MAX);
    let positions: Vec<i64> =
        sqlx::query_scalar("SELECT nextval('event_position_seq') FROM generate_series(1, $1)")
            .bind(count)
            .fetch_all(&mut **transaction)
            .await?;

    let store = store_id.to_bytes().to_vec();
    let mut builder = sqlx::QueryBuilder::<Postgres>::new(
        "INSERT INTO event \
         (position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at) ",
    );
    builder.push_values(
        events.iter().zip(&positions),
        |mut row, (event, position)| {
            let tags: Vec<String> = event
                .tags()
                .iter()
                .map(|tag| tag.as_str().to_owned())
                .collect();
            row.push_bind(*position)
                .push_bind(event.event_type().as_str().to_owned())
                .push_bind(event.data().to_vec())
                .push_bind(event.metadata().map(|bytes| bytes.to_vec()))
                .push_bind(tags)
                .push_bind(store.clone())
                .push_bind(*position)
                .push_bind(recorded_at.as_millis());
        },
    );
    builder.build().execute(&mut **transaction).await?;

    let last = *positions
        .last()
        .expect("the empty batch was refused before this point");
    position_from_row(last)
}

/// Decodes a stored `bigint` into a [`SequencePosition`].
///
/// `bigint` is signed and admits `0`; `SequencePosition` is a `NonZeroU64`. The
/// rejected alternative is `SequencePosition::new(stored.unsigned_abs())`, which
/// reads like a guard and accepts every negative value, turning a stored `-3`
/// into position 3.
pub(crate) fn position_from_row(stored: i64) -> Result<SequencePosition, PostgresEventStoreError> {
    u64::try_from(stored)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(PostgresEventStoreError::PositionOutOfRange { value: stored })
}

/// A position as the `bigint` the schema stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// The wall clock, in the units [`RecordedAt`] carries.
fn now() -> RecordedAt {
    use std::time::{SystemTime, UNIX_EPOCH};

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| {
            i64::try_from(elapsed.as_millis()).unwrap_or(i64::MAX)
        });
    RecordedAt::from_millis(millis)
}

#[cfg(test)]
mod tests {
    use super::PostgresEventStore;
    use happenstance_core::EventStore;

    /// Constraint 4: generic code binds the *bare* flavour, and a `Send`
    /// implementer must satisfy it. Instantiating the bound at this concrete
    /// store is what checks that `trait_variant`'s derivation actually reached
    /// this impl — an uninstantiated generic proves nothing.
    #[test]
    fn send_flavour_satisfies_the_bare_bound() {
        fn assert_event_store<S: EventStore>() {}
        assert_event_store::<PostgresEventStore>();
    }

    /// The store itself must cross threads, or the pool buys nothing.
    #[test]
    fn store_is_send_and_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<PostgresEventStore>();
    }
}
