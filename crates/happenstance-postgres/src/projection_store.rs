//! Postgres-backed [`SendProjectionStore`].
//!
//! # The `Batch` question, and the answer this adapter had to reach
//!
//! The port used to declare `type Batch<'a> where Self: 'a`, and the module
//! documentation on it justified the lifetime with *"a transaction cannot
//! outlive its connection"*. That is true of `rusqlite::Transaction<'a>`, which
//! borrows its `Connection`. It was **not** true here, and that was the finding
//! that fed PS-5: [`PgPool::begin`](sqlx::PgPool::begin) hands back a
//! `Transaction<'static, Postgres>`, taking the `PoolConnection` by value and
//! returning it to the pool on drop. A pooled, networked driver with every
//! opportunity to hand back a borrowed handle chose to hand back an owned one.
//!
//! PS-5 removed the parameter on that evidence, and the evidence stands. What
//! did not stand is the binding this file then carried:
//!
//! ```text
//! type Batch = sqlx::Transaction<'static, Postgres>;   // and five `todo!()`
//! ```
//!
//! **That binding cannot be discharged, and no compiler said so.** `todo!()` has
//! type `!`, which coerces to everything, so a skeleton type-checks against a
//! signature nothing can implement — the hazard the specification already names
//! about skeletons in general, and that this file demonstrated in particular.
//! Three facts settle it:
//!
//! * [`begin`](happenstance_core::ProjectionStore::begin) is **total,
//!   synchronous and infallible**. There is no `Result`, no `await`, and no
//!   argument to fail on.
//! * `sqlx`'s only constructor for a transaction is `Transaction::begin`, which
//!   is `async` and fallible; `Transaction`'s fields are private, so the type
//!   cannot be assembled by hand. `Pool::try_acquire` is synchronous but yields
//!   a `PoolConnection`, and `BEGIN` is still a round trip.
//! * [`ProjectionProbe::probe_write`](happenstance_core::ProjectionProbe::probe_write)
//!   is synchronous and infallible too, so even given a live transaction there
//!   is no point at which this adapter could issue a statement into it.
//!
//! The comment this file used to carry — *"this adapter's real body will acquire
//! the pooled connection at the first statement rather than here"* — was
//! self-refuting: with `Batch = Transaction` there is no first-statement seam
//! inside the adapter, because `begin` must **return** the transaction.
//!
//! So the batch is [`PostgresProjectionBatch`]: an owned, `Send`, `'static` list
//! of parameterised statements, replayed inside one transaction that `commit`
//! and `reset` open for themselves. That is PS-4's deferred write set, the same
//! shape `happenstance-sqlite` reaches by a different route, and what the port's
//! shape change cost this adapter is the deletion of a `where Self: 'a` clause
//! for a lifetime nothing read.
//!
//! # What that costs, stated rather than discovered
//!
//! [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
//! is `false`. Nothing has been sent to Postgres while a batch is open, so there
//! is no pending server state to read through, and answering from *committed*
//! state is what PS-12 forbids by name — a projection doing `get` then `set`
//! inside one batch would read the value from before the batch began and lose
//! every increment after the first.
//!
//! **The consequence for a caller is real and is not hidden:** a projection whose
//! `apply` must read what it has already written in the same batch cannot be
//! written against this adapter. A shadow map inside the batch would buy a green
//! `batch_reads_reflect_pending_writes` about a second source of truth Postgres
//! never sees, and is rejected on the record for that reason.
//!
//! # What this settles about PS-2, which is not this adapter's to change
//!
//! PS-2 is `[FROZEN]` and names a live-transaction adapter as the far end of the
//! batch-shape axis still to be built, offering `rusqlite` or `sqlx` as the two
//! candidates. **Both are now refuted, each by its own mechanism.** `rusqlite`'s
//! `Transaction<'_>` is `!Send`, so a live handle costs the `SendProjectionStore`
//! impl; `sqlx`'s transaction cannot be produced by a total synchronous `begin`
//! at all. The axis end is not merely unbuilt — the port's own signatures forbid
//! it for the two drivers the clause names. That is a result about the freeze,
//! and it belongs to PS-2's owner rather than to this file.
//!
//! # The invariant
//!
//! A read-model write and its checkpoint write must be one transaction, or a
//! restart either replays applied events or skips unapplied ones.
//! [`commit`](happenstance_core::ProjectionStore::commit) replays the caller's
//! statements and does the checkpoint upsert on the same [`Transaction`], then
//! commits once.
//!
//! # Schema
//!
//! `migrations/0002_projection_checkpoint.sql`, applied by
//! [`apply_projection`](crate::migration::apply_projection). Read-model tables
//! are the application's business; this adapter owns the checkpoint and the
//! transaction that carries it.

use core::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};
use sqlx::{PgPool, Postgres, Transaction};

use crate::error::PostgresProjectionStoreError;

/// [`Authority::Live`] as it is stored.
///
/// Text rather than an integer or a `CREATE TYPE`. The value is read by a human
/// with a debugger far more often than by this module; a Postgres enum type is a
/// second schema object with no `IF NOT EXISTS` arm to make the migration
/// idempotent; and the `CHECK` in migration 2 is what keeps the column honest.
/// What matters is that the authority is **stored** rather than inferred.
const AUTHORITY_LIVE: &str = "live";

/// [`Authority::Rebuilding`] as it is stored.
const AUTHORITY_REBUILDING: &str = "rebuilding";

/// The conformance suite's own read model.
///
/// A constant here rather than a file in `migrations/`, and the placement is the
/// decision: a `.sql` file in that directory is, by construction, something a
/// DBA applies. A test read model inside an application's database is a defect
/// no test in this repository could catch, because every test enables the
/// feature that creates it.
#[cfg(feature = "conformance")]
pub const PROBE_TABLE: &str = "\
CREATE TABLE IF NOT EXISTS projection_probe (
    k text   NOT NULL PRIMARY KEY,
    v bigint NOT NULL
);";

/// Mints the identity one store instance stamps its batches with.
///
/// A **process-global** ordinal, taken once per store at construction. Both
/// properties are about what must not happen, and `happenstance-sqlite` records
/// the same two: it must not be derived from a pointer address, because an `Arc`
/// can be freed and a new allocation land where the old one was, giving two
/// stores one identity; and it must not be minted in `Clone`, because a clone is
/// the same store and must accept its origin's batches.
///
/// A counter held *per store* and incremented at
/// [`begin`](SendProjectionStore::begin) is the trap that looks equivalent: it
/// hands store A and store B the identical sequence 1, 2, 3…, and the
/// foreign-batch check never fires.
fn mint_stamp() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// One bound parameter, owned.
///
/// `sqlx` ships no owned dynamic bind value. `PgArguments` is written through the
/// `Arguments` trait's `add`, which is **fallible**, and
/// [`probe_write`](happenstance_core::ProjectionProbe::probe_write) is not — so
/// encoding at push would have to park an impossible failure in the batch and
/// report it at commit, adding a state and an error variant to represent
/// something that cannot happen for these types. Parameters are therefore kept
/// as values and bound at replay.
///
/// # Why every variant carries an `Option` rather than there being one `Null`
///
/// Postgres parameters are typed, and `sqlx` derives the type OID from the Rust
/// type at `bind`. A single untyped `Null` variant would force this module to
/// pick a concrete `None::<T>` at replay, and `None::<i64>` against a `text`
/// column is a server-side type error rather than a silent success. A null with
/// a type is Postgres' own model, so it is this enum's model too.
///
/// The name is `PgParam` and not `PgValue` because `sqlx::postgres::PgValue`
/// exists and is public — on the *decode* side. Nothing here would fail to
/// compile; a reader tracking down which `PgValue` a line meant would lose the
/// time this name saves.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum PgParam {
    /// `text`.
    Text(Option<String>),
    /// `bigint`.
    Int8(Option<i64>),
    /// `integer`.
    Int4(Option<i32>),
    /// `boolean`.
    Bool(Option<bool>),
    /// `double precision`.
    Float8(Option<f64>),
    /// `bytea`.
    Bytea(Option<Vec<u8>>),
}

impl PgParam {
    /// A non-null `text`.
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(Some(value.into()))
    }

    /// A non-null `bigint`.
    pub const fn int8(value: i64) -> Self {
        Self::Int8(Some(value))
    }

    /// A non-null `bytea`.
    pub fn bytea(value: impl Into<Vec<u8>>) -> Self {
        Self::Bytea(Some(value.into()))
    }
}

/// One statement queued into a batch, with its parameters.
#[derive(Debug, Clone)]
struct PendingStatement {
    sql: Box<str>,
    params: Box<[PgParam]>,
}

/// A Postgres projection batch: an owned list of statements that has been sent
/// to the server exactly never.
///
/// It holds no connection, no transaction and no borrow, so it is `Send` and
/// `'static` — which is what lets the port's `type Batch;` carry no lifetime and
/// no `Send` bound while this adapter implements the `Send` flavour.
///
/// [`begin`](SendProjectionStore::begin) is the only way to obtain one, and that
/// is the whole of the foreign-batch defence: `stamped` is private, so a batch
/// always carries the identity of the store that began it.
#[derive(Debug, Clone)]
pub struct PostgresProjectionBatch {
    statements: Vec<PendingStatement>,
    /// The identity of the store that began this batch.
    stamp: u64,
}

impl PostgresProjectionBatch {
    /// An empty batch stamped with `stamp`.
    ///
    /// Private, deliberately. `begin` is the only caller.
    const fn stamped(stamp: u64) -> Self {
        Self {
            statements: Vec::new(),
            stamp,
        }
    }

    /// Queues a statement to run when the batch commits.
    ///
    /// # Security
    ///
    /// `sql` is `&'static str`, so it cannot be assembled at run time, and the
    /// values are **bound** rather than interpolated. A statement built out of
    /// decoded event data is a SQL injection whose source is the log — and it
    /// commits inside the same transaction that advances the checkpoint, so the
    /// projection never replays those events and nothing re-derives the rows it
    /// corrupted. [`push_raw_sql`](Self::push_raw_sql) is the escape hatch, and
    /// it is separately named so that reaching for it is a decision.
    pub fn push(&mut self, sql: &'static str, params: impl IntoIterator<Item = PgParam>) {
        self.statements.push(PendingStatement {
            sql: sql.into(),
            params: params.into_iter().collect(),
        });
    }

    /// Queues a statement whose **shape** is computed at run time.
    ///
    /// The honest case is an `IN (…)` list sized by the number of parameters.
    /// Everything [`push`](Self::push) says about binding still applies: values
    /// belong in `params`, never in `sql`.
    pub fn push_raw_sql(&mut self, sql: impl Into<Box<str>>, params: Vec<PgParam>) {
        self.statements.push(PendingStatement {
            sql: sql.into(),
            params: params.into_boxed_slice(),
        });
    }

    /// How many statements are queued.
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether nothing has been queued.
    ///
    /// An empty batch is still worth committing: the checkpoint write is the
    /// point, and a runner that skipped the commit because its events produced no
    /// read-model change would replay them forever.
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// A Postgres-backed projection store.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PostgresProjectionStore {
    pool: PgPool,
    /// This instance's identity, compared against the one every
    /// [`PostgresProjectionBatch`] carries.
    ///
    /// Copied by `Clone` rather than re-minted, because a clone is the same store
    /// and must accept its origin's batches.
    stamp: u64,
}

impl PostgresProjectionStore {
    /// Wraps an existing pool.
    ///
    /// Sharing one pool with
    /// [`PostgresEventStore`](crate::event_store::PostgresEventStore) is the
    /// expected deployment, which is why neither type builds its own.
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            stamp: mint_stamp(),
        }
    }

    /// The pool this store writes through.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

/// Binds `params` onto `query`, in order.
///
/// A free function rather than a method on [`PgParam`], because `bind` consumes
/// and returns the query by value, so the loop has to own it.
fn bind_all<'q>(
    mut query: sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments>,
    params: &'q [PgParam],
) -> sqlx::query::Query<'q, Postgres, sqlx::postgres::PgArguments> {
    for param in params {
        query = match param {
            PgParam::Text(value) => query.bind(value.as_deref()),
            PgParam::Int8(value) => query.bind(*value),
            PgParam::Int4(value) => query.bind(*value),
            PgParam::Bool(value) => query.bind(*value),
            PgParam::Float8(value) => query.bind(*value),
            PgParam::Bytea(value) => query.bind(value.as_deref()),
        };
    }
    query
}

/// Replays `batch`'s statements, in order, inside `transaction`.
async fn replay(
    transaction: &mut Transaction<'static, Postgres>,
    batch: &PostgresProjectionBatch,
) -> Result<(), sqlx::Error> {
    for statement in &batch.statements {
        bind_all(sqlx::query(&statement.sql), &statement.params)
            .execute(&mut **transaction)
            .await?;
    }
    Ok(())
}

/// A stored `bigint` as a [`SequencePosition`].
///
/// Fallible rather than a cast, for the reason the event store's own converter
/// is: `bigint` is signed and admits zero, and `unsigned_abs()` would turn a
/// corrupt negative row into a plausible position.
fn position_from_row(stored: i64) -> Result<SequencePosition, PostgresProjectionStoreError> {
    u64::try_from(stored)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(PostgresProjectionStoreError::CheckpointOutOfRange { value: stored })
}

/// A [`SequencePosition`] as a `bigint`.
///
/// `SequencePosition` is a `NonZeroU64` and `bigint` is signed, so the top half
/// of the domain has no representation. It is refused rather than wrapped.
fn position_to_row(position: SequencePosition) -> Result<i64, PostgresProjectionStoreError> {
    i64::try_from(position.get()).map_err(|_| {
        PostgresProjectionStoreError::CheckpointOutOfRange {
            // Reported as the negative it would have become, which is the value a
            // reader would find in the table had this been written.
            value: position.get().cast_signed(),
        }
    })
}

/// [`Authority`] as it is stored.
///
/// `Authority` is `#[non_exhaustive]`, so a variant this adapter has never seen
/// is possible. It is written as a value migration 2's `CHECK` refuses, rather
/// than silently stored as `live` — reporting a half-finished rebuild as a live
/// projection is the failure this arm exists to prevent, and a constraint
/// violation names it where it happened.
const fn authority_to_row(authority: Authority) -> &'static str {
    match authority {
        Authority::Live => AUTHORITY_LIVE,
        Authority::Rebuilding => AUTHORITY_REBUILDING,
        _ => "unknown",
    }
}

impl SendProjectionStore for PostgresProjectionStore {
    type Error = PostgresProjectionStoreError;

    /// An owned buffered write set. See the [module documentation](self) for why
    /// this is not `sqlx::Transaction<'static, Postgres>`, which is what this
    /// line said while every body below was `todo!()`.
    type Batch = PostgresProjectionBatch;

    /// Neither `async` nor fallible — and now that is a property this adapter can
    /// actually hold: minting an empty `Vec` and copying a `u64` cannot fail and
    /// touches no connection.
    fn begin(&self) -> Self::Batch {
        PostgresProjectionBatch::stamped(self.stamp)
    }

    /// Reads `id`'s checkpoint, or [`Checkpoint::NeverRun`] when it has no row.
    ///
    /// The absence of a row **is** the `NeverRun` state, which is why a
    /// successful [`reset`](Self::reset) deletes rather than writing a sentinel:
    /// a sentinel position would make a reset indistinguishable from a commit at
    /// the first position.
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

    /// Applies `batch` and advances `id`'s checkpoint to `position`, as one unit.
    ///
    /// # How the regression guard is built, and why not the obvious way
    ///
    /// The upsert carries its own guard — `ON CONFLICT … DO UPDATE … WHERE
    /// projection_checkpoint.position <= excluded.position` — so two runners
    /// racing on one `ProjectionId` are serialised by the row lock `ON CONFLICT
    /// DO UPDATE` takes, without this adapter holding a lock of its own.
    /// `happenstance-sqlite` reads the recorded position first and relies on a
    /// global write lock; Postgres has no such lock, which is the point of this
    /// crate.
    ///
    /// A guarded upsert reports a regression as **zero rows** and cannot say what
    /// the recorded position was: `RETURNING` on a `DO UPDATE` yields the new
    /// row, and Postgres exposes no `OLD` there. So the recorded value is re-read
    /// inside the same transaction before the error is built —
    /// [`CommitError::CheckpointRegression`] carries both values so a caller can
    /// log the gap rather than spend a second round trip on it.
    ///
    /// **The transaction is rolled back on that path**, and that is not
    /// bookkeeping: the caller's statements are already in it, and a refused
    /// commit that left the read model written would be a partial application
    /// with no checkpoint to record it.
    ///
    /// # Errors
    ///
    /// * [`CommitError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance, decided before any statement is prepared;
    /// * [`CommitError::CheckpointRegression`] if `position` is below the one
    ///   recorded — equal is accepted, because a batch that wrote nothing new
    ///   still records that its events were considered;
    /// * [`CommitError::Store`] if the driver or the server failed.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if batch.stamp != self.stamp {
            // Before any SQL, which is what makes "neither store moved"
            // structural rather than something to remember.
            return Err(CommitError::ForeignBatch);
        }

        let stored_position = position_to_row(position).map_err(CommitError::Store)?;

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;

        replay(&mut transaction, &batch)
            .await
            .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;

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

        // Zero rows: the guard refused. Read what it refused against, still
        // inside the transaction, so the pair reported is the pair the guard saw.
        let current: (i64,) =
            sqlx::query_as("SELECT position FROM projection_checkpoint WHERE projection_id = $1")
                .bind(id.as_str())
                .fetch_one(&mut *transaction)
                .await
                .map_err(|error| CommitError::Store(PostgresProjectionStoreError::Driver(error)))?;

        let current = position_from_row(current.0).map_err(CommitError::Store)?;

        // The caller's statements are in this transaction. Discarding it is what
        // makes the refusal leave the read model untouched.
        drop(transaction);

        Err(CommitError::CheckpointRegression {
            current,
            attempted: position,
        })
    }

    /// Applies `batch` and returns `id` to [`Checkpoint::NeverRun`], as one unit.
    ///
    /// `commit`'s dual. It deletes nothing this adapter chose: the read model is
    /// the caller's, and an adapter that truncated a table of its own naming
    /// would be scoped to something the port deliberately never told it.
    ///
    /// # Errors
    ///
    /// * [`ResetError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance;
    /// * [`ResetError::Store`] if the driver or the server failed.
    ///
    /// [`ResetError::Refused`] is never returned: this store holds no protection
    /// policy, which is PS-18's mechanism left unexercised by a store with
    /// nothing to protect.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        let mut transaction = self
            .pool
            .begin()
            .await
            .map_err(|error| ResetError::Store(PostgresProjectionStoreError::Driver(error)))?;

        replay(&mut transaction, &batch)
            .await
            .map_err(|error| ResetError::Store(PostgresProjectionStoreError::Driver(error)))?;

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

    /// Discards the batch without committing.
    ///
    /// Nothing was ever sent to Postgres, so this is a drop — which is also the
    /// evidence PS-7 asks for: for a buffering adapter, "dropping a batch must
    /// roll back" is free, and the store is usable afterwards because no
    /// connection was ever checked out to return.
    ///
    /// # Errors
    ///
    /// [`PostgresProjectionStoreError::ForeignBatch`] if `batch` was begun on a
    /// different store instance. The batch is consumed either way; the refusal is
    /// how a caller learns it was holding the wrong one, on the call that was
    /// meant to be the cleanup.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        if batch.stamp != self.stamp {
            return Err(PostgresProjectionStoreError::ForeignBatch);
        }
        drop(batch);
        Ok(())
    }
}

/// The conformance suite's write and read seam.
///
/// It lives here, in `src/`, and not in `tests/projection.rs`: that is a
/// different crate, where neither [`ProjectionProbe`] nor
/// [`PostgresProjectionStore`] is local and the orphan rule answers
/// `error[E0117]`.
///
/// [`ProjectionProbe`]: happenstance_core::ProjectionProbe
#[cfg(feature = "conformance")]
impl happenstance_core::ProjectionProbe for PostgresProjectionStore {
    /// `false`, and it is the honest answer rather than a gap.
    ///
    /// A [`PostgresProjectionBatch`] is a list of statements that has been sent
    /// to Postgres exactly never, so there is no open transaction to read
    /// through — and answering from *committed* state is what PS-12 forbids by
    /// name. The two rules that would have used it are emitted as reported skips
    /// rather than omitted.
    const READS_THROUGH_BATCH: bool = false;

    /// Queues one probe row. Synchronous and infallible, because queueing a
    /// statement into a buffer the caller owns cannot fail.
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.push(
            "INSERT INTO projection_probe (k, v) VALUES ($1, $2) \
             ON CONFLICT (k) DO UPDATE SET v = excluded.v",
            [
                PgParam::text(key),
                // `cast_signed` rather than a fallible conversion: `bigint` is
                // signed, the bit pattern round-trips every `u64` exactly, and a
                // saturating conversion would map two values to one.
                PgParam::int8(value.cast_signed()),
            ],
        );
    }

    /// Queues removal of every probe row, so that
    /// [`reset`](happenstance_core::ProjectionStore::reset) can be checked
    /// without the suite knowing what a read model is.
    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.push("DELETE FROM projection_probe", core::iter::empty());
    }

    /// Reads one probe row from **committed** state.
    ///
    /// # Errors
    ///
    /// [`PostgresProjectionStoreError::Driver`] if the pool or the server
    /// rejected the read.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        let row: Option<(i64,)> = sqlx::query_as("SELECT v FROM projection_probe WHERE k = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|(stored,)| stored.cast_unsigned()))
    }

    /// Never called, because
    /// [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
    /// is `false`.
    ///
    /// # Panics
    ///
    /// Always. The specification permits exactly this for an adapter whose batch
    /// has no read path, and the panic is the honest answer: any value returned
    /// here would be a claim about pending writes Postgres has never been told
    /// about.
    fn probe_read_through(&self, _batch: &Self::Batch, _key: &str) -> Option<u64> {
        unimplemented!(
            "PostgresProjectionBatch buffers its statements, so `READS_THROUGH_BATCH` \
             is `false` and this is never called: there is no open transaction to \
             read through, and answering from committed state is what PS-12 forbids"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{PgParam, PostgresProjectionBatch, PostgresProjectionStore, mint_stamp};
    use happenstance_core::ProjectionStore;

    /// The bare flavour must come free from the `Send` one, at this concrete
    /// store rather than in the abstract.
    #[test]
    fn send_flavour_satisfies_the_bare_bound() {
        fn assert_projection_store<P: ProjectionStore>() {}
        assert_projection_store::<PostgresProjectionStore>();
    }

    /// The batch does not borrow the store, written in argument position.
    ///
    /// Against the old GAT this had to be written this way because
    /// `assert_static::<Batch<'_>>()` proved nothing — `'_` in a turbofish is
    /// merely *inferred* and the compiler picked `'static`, so the assertion
    /// passed against `type Batch<'a> = Transaction<'a, Postgres>`, the exact
    /// shape it existed to reject. It was written that way first. Argument
    /// position was the fix, because `'_` there elides to a fresh
    /// universally-quantified lifetime rather than to whatever inference finds
    /// convenient.
    ///
    /// The port no longer has a lifetime to elide, so that hazard cannot recur —
    /// and the test is not therefore vacuous. `type Batch;` is still bindable to
    /// a borrowing type by a store carrying a lifetime of its own, and rebinding
    /// this associated type to anything but the owned batch makes this function
    /// `error[E0308]`.
    fn the_batch_does_not_borrow_the_store(
        batch: <PostgresProjectionStore as ProjectionStore>::Batch,
    ) -> PostgresProjectionBatch {
        batch
    }

    #[test]
    fn the_batch_is_owned() {
        fn assert_static<T: 'static>() {}
        fn assert_send<T: Send>() {}

        assert_static::<<PostgresProjectionStore as ProjectionStore>::Batch>();
        assert_send::<<PostgresProjectionStore as ProjectionStore>::Batch>();
        let _ = the_batch_does_not_borrow_the_store;
    }

    /// Two stores must not share an identity, or the foreign-batch check never
    /// fires.
    ///
    /// This is the test that rejects a per-store counter incremented at `begin`,
    /// which hands every store the sequence 1, 2, 3… and reports every foreign
    /// batch as its own.
    #[test]
    fn each_store_mints_a_distinct_stamp() {
        let first = mint_stamp();
        let second = mint_stamp();
        assert_ne!(first, second);
    }

    /// A batch begun on one store carries that store's identity, and a batch
    /// begun on another does not match it.
    ///
    /// Written against the stamps rather than against a live pool, because
    /// building a `PgPool` is a round trip and this proposition is not about the
    /// server.
    #[test]
    fn a_batch_carries_the_stamp_of_the_store_that_began_it() {
        let first = PostgresProjectionBatch::stamped(mint_stamp());
        let second = PostgresProjectionBatch::stamped(mint_stamp());
        assert_ne!(first.stamp, second.stamp);
    }

    /// An empty batch is still a batch worth committing.
    #[test]
    fn a_fresh_batch_is_empty_and_grows() {
        let mut batch = PostgresProjectionBatch::stamped(1);
        assert!(batch.is_empty());
        assert_eq!(batch.len(), 0);

        batch.push("SELECT $1", [PgParam::text("x")]);
        assert!(!batch.is_empty());
        assert_eq!(batch.len(), 1);
    }

    /// A null carries a type, because Postgres parameters are typed.
    ///
    /// The variant that would fail is a single untyped `Null`: bound as
    /// `None::<&str>` against a `bigint` column it is a server-side type error,
    /// and there is no correct choice to make at the bind site.
    #[test]
    fn a_null_parameter_carries_its_type() {
        assert_eq!(PgParam::Int8(None), PgParam::Int8(None));
        assert_ne!(PgParam::Int8(None), PgParam::Text(None));
    }
}
