//! Postgres-backed [`SendEventStore`].
//!
//! # Status: not implemented
//!
//! The types are real; the bodies that would speak to a server are `todo!()`.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE event (
//!     position   bigint PRIMARY KEY,
//!     event_type text   NOT NULL,
//!     data       bytea  NOT NULL,
//!     metadata   bytea,
//!     tags       text[] NOT NULL  -- canonically sorted
//! );
//!
//! CREATE INDEX event_tags_idx ON event USING gin (tags);
//! CREATE INDEX event_type_idx ON event (event_type, position);
//! ```
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
    AppendCondition, AppendError, Event, EventId, Query, ReadOptions, SendEventStore,
    SequencePosition, SequencedEvent,
};
use sqlx::PgPool;

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
        Self { pool }
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
        _events: &[Event],
        _condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        // Blocked on the ES-10 decision above, which is what decides whether
        // this is one statement or three.
        todo!("postgres event store: append")
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        // `todo!()` rather than `SELECT max(position) FROM event`, and the
        // difference is the whole reason this crate is in the tree. On every
        // other adapter the maximum position *is* the head, because allocation
        // happens under the lock that commit releases. Here `nextval()`
        // allocates outside the transaction, so `max(position)` can name a row
        // whose predecessors are still in flight — and a head is a promise that
        // nothing at or below it will appear later (ES-10). What this must
        // return is the **visibility frontier**, which the three candidate
        // mechanisms spell differently: under either lock it collapses back to
        // `max(position)`, and under `xid8` it is `max(position) WHERE xid <
        // pg_snapshot_xmin(pg_current_snapshot())`, which trails the maximum.
        // So this body is blocked on the same open decision `append` is, and
        // writing the cheap version now would encode the answer by accident.
        todo!("postgres event store: head")
    }

    async fn contains_event_id(&self, _id: EventId) -> Result<bool, Self::Error> {
        // `SELECT 1 FROM event WHERE origin_store = $1 AND origin_position = $2`
        // — two columns the intended schema above does not have yet, because
        // nothing writes an origin until `append` exists. It inherits `head`'s
        // question rather than only its blockage: under the `xid8` mechanism a
        // committed row above the frontier is invisible to `read`, so answering
        // `true` for it would make this method disagree with the stream, and
        // answering `false` would make ingest re-accept an event the store
        // already holds. Which of those is right is a replication question, not
        // a SQL one.
        todo!("postgres event store: contains_event_id")
    }
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
