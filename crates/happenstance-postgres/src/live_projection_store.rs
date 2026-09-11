//! Postgres-backed [`SendProjectionStore`] whose batch **is** a live transaction.
//!
//! # Why a second projection store, and why it is an instrument
//!
//! PS-2 is `[FROZEN]` and forbids freezing the projection port until the
//! conformance suite has gone green against adapters at **both ends** of the
//! batch-shape axis: one whose batch is an owned write set replayed at commit,
//! and one whose batch holds a live transaction. Every implementation in this
//! workspace — [`PostgresProjectionStore`](crate::projection_store::PostgresProjectionStore)
//! included — sat at the first end, and until ADR-0062 the port's own signatures
//! made the second unreachable: `begin` was total, synchronous and infallible,
//! while `sqlx`'s only route to a [`Transaction`] is `pool.begin().await?`; and
//! the probe seam was synchronous, infallible and took `&Self::Batch`, while a
//! driver borrows its connection mutably to issue a statement. ADR-0060 recorded
//! that finding. ADR-0062 moved the seam. **This is the adapter that stands at
//! the end the seam now admits.**
//!
//! It is the same reason `happenstance-postgres` exists at all for the event
//! store: not to ship a second way of doing the same thing, but to sit at the
//! far end of an axis and find out whether the port is wrong about something the
//! near-end adapters cannot disagree with. What is different here, and stated
//! rather than discovered:
//!
//! * [`begin`](SendProjectionStore::begin) is a round trip — `BEGIN` on a pooled
//!   connection — and can fail. A buffering store's `begin` is neither.
//! * Every write issued into the batch has **already reached the server** when
//!   the batch is dropped. PS-7's *"dropping a batch rolls back and leaves the
//!   store usable"* is free for a buffer and is a real obligation here: the
//!   pooled connection must go back with its transaction rolled back.
//! * A statement that fails mid-batch **poisons the transaction** — Postgres
//!   answers every subsequent statement with *"current transaction is aborted"*
//!   — so a probe write can fail, and the seam's `Result` is where it says so.
//! * [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
//!   is `true`, and it is a **true statement about this store**: a `SELECT`
//!   through the open transaction sees the transaction's own uncommitted writes.
//!   PS-12's rule, `batch_reads_reflect_pending_writes`, runs here against a real
//!   database for the first time.
//!
//! # What a caller cannot do with it, stated rather than discovered
//!
//! The typed layer's `Projection::apply` is synchronous — buffering a row does
//! not await — so a projection cannot issue a statement into a
//! [`LivePostgresBatch`] from inside `apply`. This store therefore cannot be
//! driven by `happenstance::run_projection` for a projection that writes rows;
//! the buffered [`PostgresProjectionStore`](crate::projection_store::PostgresProjectionStore)
//! is what an application reaches for. Opening the probe seam let the **suite**
//! observe a live-transaction batch; letting an **application** write into one
//! is the axis after this one, and it belongs to the typed layer.
//!
//! # The invariant
//!
//! A read-model write and its checkpoint write must be one transaction. Here
//! that is literal: the caller's statements and the checkpoint upsert are issued
//! on the same [`Transaction`], and [`commit`](SendProjectionStore::commit)
//! commits it once.

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::error::PostgresProjectionStoreError;
use crate::projection_store::{
    AUTHORITY_LIVE, AUTHORITY_REBUILDING, PgParam, authority_to_row, bind_all, mint_stamp,
    position_from_row, position_to_row,
};

/// A Postgres projection batch that **is** an open transaction.
///
/// It owns a pooled connection with `BEGIN` already issued on it. Statements
/// issued through [`execute`](Self::execute) reach the server as they are made;
/// nothing becomes durable until [`commit`](SendProjectionStore::commit).
///
/// `Transaction<'static, Postgres>` owns its `PoolConnection` and borrows
/// nothing, which is what lets the port's `type Batch;` carry no lifetime while
/// this batch holds a live handle — the finding PS-5 was built on.
///
/// Dropping it without committing rolls the transaction back: `sqlx` queues a
/// `ROLLBACK` on the connection as it returns to the pool. That is PS-7 met by
/// the driver rather than by this adapter, and `dropped_batch_leaves_store_usable`
/// is the rule that checks it actually happens.
#[derive(Debug)]
pub struct LivePostgresBatch {
    transaction: Transaction<'static, Postgres>,
    /// The identity of the store that began this batch.
    stamp: u64,
}

impl LivePostgresBatch {
    /// Issues one statement through the open transaction.
    ///
    /// The statement reaches the server now. If it fails, the transaction is
    /// aborted on the server side and every later statement — including the
    /// checkpoint upsert — will be refused; the honest thing to do with the batch
    /// after an `Err` here is to hand it to
    /// [`rollback`](SendProjectionStore::rollback) or drop it.
    ///
    /// # Security
    ///
    /// `sql` is `&'static str`, so it cannot be assembled at run time, and the
    /// values are **bound** rather than interpolated — the same two guards as
    /// [`PostgresProjectionBatch::push`](crate::projection_store::PostgresProjectionBatch::push),
    /// for the same reason: a statement built out of decoded event data is a
    /// SQL injection whose source is the log, and here it does not even wait
    /// for `commit` to reach the server. [`execute_raw_sql`](Self::execute_raw_sql)
    /// is the escape hatch, separately named so that reaching for it is a
    /// decision.
    ///
    /// # Errors
    ///
    /// [`PostgresProjectionStoreError::Driver`] if the server refused the
    /// statement or the connection failed.
    pub async fn execute(
        &mut self,
        sql: &'static str,
        params: impl IntoIterator<Item = PgParam>,
    ) -> Result<(), PostgresProjectionStoreError> {
        self.execute_raw_sql(sql, params.into_iter().collect())
            .await
    }

    /// Issues one statement whose **shape** is computed at run time.
    ///
    /// The honest case is an `IN (…)` list sized by the number of parameters.
    /// Everything [`execute`](Self::execute) says about binding still applies:
    /// values belong in `params`, never in `sql`.
    ///
    /// # Errors
    ///
    /// [`PostgresProjectionStoreError::Driver`] if the server refused the
    /// statement or the connection failed.
    pub async fn execute_raw_sql(
        &mut self,
        sql: &str,
        params: Vec<PgParam>,
    ) -> Result<(), PostgresProjectionStoreError> {
        bind_all(sqlx::query(sql), &params)
            .execute(&mut *self.transaction)
            .await?;
        Ok(())
    }
}

/// A Postgres-backed projection store whose batches are live transactions.
///
/// See the [module documentation](self) for what this store is for and what it
/// cannot do.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct LivePostgresProjectionStore {
    pool: PgPool,
    /// This instance's identity, compared against the one every
    /// [`LivePostgresBatch`] carries. Copied by `Clone` rather than re-minted,
    /// because a clone is the same store and must accept its origin's batches.
    stamp: u64,
}

impl LivePostgresProjectionStore {
    /// Wraps an existing pool.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            stamp: mint_stamp(),
        }
    }

    /// The pool this store opens transactions on.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

impl SendProjectionStore for LivePostgresProjectionStore {
    type Error = PostgresProjectionStoreError;

    /// A live transaction. This is the binding the phase-2 skeleton declared
    /// and could not discharge while `begin` was synchronous; ADR-0062 is what
    /// made it inhabitable.
    type Batch = LivePostgresBatch;

    /// Acquires a pooled connection and issues `BEGIN` on it.
    ///
    /// A round trip, and the failure `begin`'s `Result` exists for: a pool with
    /// no free connection, a server that refused the session.
    ///
    /// # Errors
    ///
    /// [`PostgresProjectionStoreError::Driver`] if the pool or the server
    /// refused to open a transaction.
    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        let transaction = self.pool.begin().await?;
        Ok(LivePostgresBatch {
            transaction,
            stamp: self.stamp,
        })
    }

    /// Reads `id`'s checkpoint, or [`Checkpoint::NeverRun`] when it has no row.
    ///
    /// Through the pool rather than through any open batch: a checkpoint read
    /// is committed state, and a batch is not a parameter here.
    ///
    /// # Errors
    ///
    /// * [`PostgresProjectionStoreError::Driver`] if the pool or the server
    ///   rejected the read;
    /// * [`CheckpointOutOfRange`](PostgresProjectionStoreError::CheckpointOutOfRange)
    ///   or [`InvalidAuthority`](PostgresProjectionStoreError::InvalidAuthority)
    ///   if the row is one this adapter could not have written.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let row: Option<(i64, String)> = sqlx::query_as(
            "SELECT position, authority FROM projection_checkpoint WHERE projection_id = $1",
        )
        .bind(id.as_str())
        .fetch_optional(&self.pool)
        .await?;

        let Some((stored_position, stored_authority)) = row else {
            return Ok(Checkpoint::NeverRun);
        };

        let through = position_from_row(stored_position)?;
        match stored_authority.as_str() {
            AUTHORITY_LIVE => Ok(Checkpoint::Live { through }),
            AUTHORITY_REBUILDING => Ok(Checkpoint::Rebuilding { through }),
            _ => Err(PostgresProjectionStoreError::InvalidAuthority(
                stored_authority,
            )),
        }
    }

    /// Advances `id`'s checkpoint to `position` on the batch's own transaction,
    /// then commits it.
    ///
    /// The regression guard is the same guarded upsert the buffered store uses,
    /// for the same reason: `ON CONFLICT DO UPDATE` takes the row lock, so two
    /// runners racing on one `ProjectionId` are serialised by Postgres rather
    /// than by this adapter. What differs is what a refusal costs. The caller's
    /// statements are **already on the server** inside this transaction, so the
    /// refusal path issues an explicit `ROLLBACK` rather than relying on a drop —
    /// and it does so before building the error, so that a rollback failure is
    /// reported rather than swallowed behind a regression that was only half of
    /// what went wrong.
    ///
    /// # Errors
    ///
    /// * [`CommitError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance, decided before any statement is issued — the foreign batch's
    ///   transaction is dropped, which rolls it back;
    /// * [`CommitError::CheckpointRegression`] if `position` is below the one
    ///   recorded;
    /// * [`CommitError::Store`] if the driver or the server failed, including a
    ///   transaction already aborted by an earlier failed statement.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(CommitError::ForeignBatch);
        }

        let stored_position = position_to_row(position).map_err(CommitError::Store)?;
        let mut transaction = batch.transaction;

        let advanced: Option<(i64,)> = sqlx::query_as(
            "INSERT INTO projection_checkpoint (projection_id, position, authority) \
             VALUES ($1, $2, $3) \
             ON CONFLICT (projection_id) DO UPDATE \
             SET position = excluded.position, authority = excluded.authority \
             WHERE projection_checkpoint.position <= excluded.position \
             RETURNING position",
        )
        .bind(id.as_str())
        .bind(stored_position)
        .bind(authority_to_row(authority))
        .fetch_optional(&mut *transaction)
        .await
        .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;

        if advanced.is_some() {
            return transaction
                .commit()
                .await
                .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)));
        }

        let current: (i64,) =
            sqlx::query_as("SELECT position FROM projection_checkpoint WHERE projection_id = $1")
                .bind(id.as_str())
                .fetch_one(&mut *transaction)
                .await
                .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;
        let current = position_from_row(current.0).map_err(CommitError::Store)?;

        transaction
            .rollback()
            .await
            .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;

        Err(CommitError::CheckpointRegression {
            current,
            attempted: position,
        })
    }

    /// Deletes `id`'s checkpoint on the batch's own transaction, then commits
    /// it. The caller's deletes are already in the transaction.
    ///
    /// # Errors
    ///
    /// * [`ResetError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance;
    /// * [`ResetError::Store`] if the driver or the server failed.
    ///
    /// [`ResetError::Refused`] is never returned: this store holds no protection
    /// policy, for the reason its buffered sibling gives.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }
        let mut transaction = batch.transaction;

        sqlx::query("DELETE FROM projection_checkpoint WHERE projection_id = $1")
            .bind(id.as_str())
            .execute(&mut *transaction)
            .await
            .map_err(|error| ResetError::Store(PostgresProjectionStoreError::Driver(error)))?;

        transaction
            .commit()
            .await
            .map_err(|error| ResetError::Store(PostgresProjectionStoreError::Driver(error)))
    }

    /// Rolls the transaction back and returns its connection to the pool.
    ///
    /// Not a drop: the statements are on the server, and `ROLLBACK` is a round
    /// trip that can fail, which is the whole reason the port keeps `rollback`
    /// as a method beside `Drop`.
    ///
    /// # Errors
    ///
    /// * [`PostgresProjectionStoreError::ForeignBatch`] if `batch` was begun on
    ///   a different store instance — the batch is consumed and its transaction
    ///   dropped either way;
    /// * [`PostgresProjectionStoreError::Driver`] if the `ROLLBACK` failed.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        if batch.stamp != self.stamp {
            return Err(PostgresProjectionStoreError::ForeignBatch);
        }
        batch.transaction.rollback().await?;
        Ok(())
    }
}

/// The conformance suite's write and read seam — and here every member is a
/// statement on the open transaction.
#[cfg(feature = "conformance")]
impl happenstance_core::ProjectionProbe for LivePostgresProjectionStore {
    /// `true`, and true of the store: a `SELECT` through the open transaction
    /// sees the transaction's own uncommitted writes. This is the declaration a
    /// live-transaction adapter could not make honestly before ADR-0062, and the
    /// one that makes the two ends of PS-2's axis distinguishable to the suite.
    const READS_THROUGH_BATCH: bool = true;

    /// Upserts one probe row **on the server**, inside the open transaction.
    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        batch
            .execute(
                "INSERT INTO projection_probe (k, v) VALUES ($1, $2) \
                 ON CONFLICT (k) DO UPDATE SET v = excluded.v",
                [PgParam::text(key), PgParam::int8(value.cast_signed())],
            )
            .await
    }

    /// Deletes every probe row inside the open transaction.
    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        batch.execute("DELETE FROM projection_probe", []).await
    }

    /// Reads one probe row from **committed** state, through the pool.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT v FROM projection_probe WHERE k = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(stored,)| stored.cast_unsigned()))
    }

    /// Reads one probe row **through the open transaction**: committed state,
    /// with this batch's own pending writes layered over it by the server.
    async fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT v FROM projection_probe WHERE k = $1")
            .bind(key)
            .fetch_optional(&mut *batch.transaction)
            .await?;
        Ok(row.map(|(stored,)| stored.cast_unsigned()))
    }
}
