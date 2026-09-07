//! SQLite-backed [`SendProjectionStore`].
//!
//! # Stability: this module is not covered by the crate's version
//!
//! Everything else `happenstance-sqlite` publishes is semver-binding at
//! `0.2.0`. This module is not, and the exemption is written down rather than
//! implied: it sits behind the off-by-default `projection-store` feature, which
//! forwards `happenstance-core`'s `unstable-projection`, and that port stays
//! exempt until two adapters at **opposite ends of the batch-shape axis** have
//! passed its conformance suite. Today the two shapes that clear the suite are
//! both testkit-side instruments, which is one shape wearing two hats.
//!
//! On docs.rs the feature shows as a badge on this module. The badge says which
//! flag; this paragraph says what the flag costs, because a reader who takes a
//! dependency on a page cannot see a semver exemption in a feature name.
//!
//! # Status: a projection store that has run the suite it did not write
//!
//! SQLite is the adapter that makes the checkpoint invariant easy to honour: the
//! read-model writes and the checkpoint update share one transaction, so there is
//! no window in which they can disagree. `tests/projection.rs` is where that
//! stops being a claim — it mounts
//! `happenstance_testkit::projection_store_conformance!` against a real temporary
//! file, and every rule in the family either runs against this store or is
//! reported as a skip carrying the fixture's own stated reason.
//!
//! What a green run here does **not** buy is PS-2. That clause wants the suite
//! green against two adapters at opposite ends of the batch-shape axis, and this
//! is a *third* replay-at-commit shape beside `MemoryProjectionStore` and the
//! testkit's buffering variant — see [`SqliteBatch`] for why no `rusqlite`
//! adapter can supply the other end.
//!
//! # Why the batch is a buffer and not a `rusqlite::Transaction`
//!
//! The port's documentation *used to* say a batch is a live transaction, and
//! justified the lifetime on `type Batch<'a>` with "a transaction cannot outlive
//! its connection". ADR-0017 removed the lifetime and PS-4 now says the reverse
//! — a `Batch` MUST NOT be required to be a live transaction — with this crate
//! as one of the reasons why. Even while the lifetime existed,
//! `type Batch<'a> = rusqlite::Transaction<'a>` failed on
//! [`SendProjectionStore`] for two independent reasons, each confirmed against
//! this crate:
//!
//! 1. [`rusqlite::Connection`] is [`Send`] and **not** [`Sync`] — it holds a
//!    `RefCell<InnerConnection>` — so a store that owns one directly is not
//!    `Sync`, so `&Self` is not `Send`, so *every* future in the trait is
//!    rejected, including [`checkpoint`](happenstance_core::ProjectionStore::checkpoint),
//!    which never touches a batch at all.
//! 2. `rusqlite::Transaction<'_>` is itself `!Send`, so `commit` and `rollback`
//!    are rejected on the batch **parameter** alone even when the store is
//!    wrapped to be `Sync`.
//!
//! Reason 1 is fixed here by [`Mutex`]; reason 2 cannot be fixed at all without
//! giving up the live handle. So the batch is an owned, `Send`, replayable write
//! set — [`SqliteBatch`] — opened into a real transaction inside `commit`. That
//! satisfies the port's obligation (read model and checkpoint move together),
//! and it is now the shape the port asks for rather than a departure from it.
//!
//! # The schema this store owns
//!
//! ```sql
//! CREATE TABLE projection_checkpoint (
//!     projection_id TEXT    NOT NULL PRIMARY KEY,
//!     position      INTEGER NOT NULL,
//!     authority     TEXT    NOT NULL
//! ) WITHOUT ROWID;
//!
//! CREATE TABLE projection_meta (
//!     k TEXT    NOT NULL PRIMARY KEY,
//!     v INTEGER NOT NULL
//! ) WITHOUT ROWID;
//! ```
//!
//! The `authority` column is the half an earlier sketch of this schema left out,
//! and leaving it out is not a shortcut but an impossibility: without it
//! [`Checkpoint::Rebuilding`] has
//! nowhere to come from, so a reader asking whether the rows in front of it are
//! authoritative is told `Live` over a half-built read model by every
//! implementation. The **absence of a row** is
//! [`NeverRun`](happenstance_core::Checkpoint::NeverRun) — never a sentinel row
//! at position zero, which would make "never run" and "committed at the first
//! position" the same state and skip event 1 permanently and silently.
//!
//! Read-model tables themselves are the application's business; this adapter owns
//! only the checkpoint and the transaction that carries it. That is also why
//! [`reset`](happenstance_core::ProjectionStore::reset) applies the *caller's*
//! deletes: an adapter that truncated a table of its own choosing would be
//! inventing a read model it does not own.
//!
//! Under `feature = "conformance"` one more table appears —
//! `projection_probe(k, v)`, the conformance suite's own read model, written
//! through [`ProjectionProbe`](happenstance_core::ProjectionProbe). It is behind
//! that feature and not behind a runtime flag, so it cannot reach an
//! application's database.
//!
//! # Two version markers in one file, on purpose
//!
//! An event store and a projection store on one path are **two connections**, and
//! either may be opened without the other — they are separate crate features. So
//! this schema carries [`SCHEMA_VERSION`] in its own `projection_meta` table
//! rather than sharing the event store's `store_meta` counter: the two schemas
//! advance on different stories' schedules, and one counter would make either
//! migration a breaking change to the other.
//!
//! # One crate, one runtime seam
//!
//! Every method that reaches SQLite hops onto a blocking thread through the
//! [`tokio::runtime::Handle`] captured at construction, falling back to
//! [`Handle::try_current`] and reporting
//! [`NoRuntime`](SqliteProjectionStoreError::NoRuntime) when there is neither.
//! That is ADR-0022 §9's decision for the event store's read path, applied here
//! unchanged: a second, different answer inside one crate is the defect. It is
//! also what keeps the [`MutexGuard`](std::sync::MutexGuard) and the `await`
//! apart — a guard held across an await would make the future `!Send` and cost
//! this adapter its [`SendProjectionStore`] impl.

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};
use rusqlite::types::Value;
use rusqlite::{Connection, OptionalExtension, TransactionBehavior, params_from_iter};
use tokio::runtime::{Handle, TryCurrentError};
use tokio::task::JoinError;

/// The projection migration this build knows how to operate.
///
/// Persisted in `projection_meta` so that migration 2 has something to test
/// against: a schema with no version marker cannot be migrated later without
/// guessing. It is **this schema's own** marker, on its own terms — the event
/// store's lives in `store_meta` and moves on a different story's schedule.
pub const SCHEMA_VERSION: u32 = 1;

/// Migration 1, verbatim. Mirrored in the [module documentation](self).
///
/// `IF NOT EXISTS` throughout, because [`SqliteProjectionStore::open`] runs
/// `migrate` on **every** connect and a fixture connects more than once onto one
/// file.
const MIGRATION_1: &str = "\
CREATE TABLE IF NOT EXISTS projection_checkpoint (
    projection_id TEXT    NOT NULL PRIMARY KEY,
    position      INTEGER NOT NULL,
    authority     TEXT    NOT NULL
) WITHOUT ROWID;
CREATE TABLE IF NOT EXISTS projection_meta (
    k TEXT    NOT NULL PRIMARY KEY,
    v INTEGER NOT NULL
) WITHOUT ROWID;";

/// The conformance suite's own read model, created under the same `cfg` as the
/// [`ProjectionProbe`](happenstance_core::ProjectionProbe) impl that writes it.
///
/// Behind the feature rather than behind a runtime `if`: a test read model
/// shipped inside an application's database is a defect no test in this
/// repository could catch, because every test enables the feature.
#[cfg(feature = "conformance")]
const PROBE_TABLE: &str = "\
CREATE TABLE IF NOT EXISTS projection_probe (
    k TEXT    NOT NULL PRIMARY KEY,
    v INTEGER NOT NULL
) WITHOUT ROWID;";

/// The `projection_meta` key holding the migration this file is at.
const SCHEMA_VERSION_KEY: &str = "schema_version";

/// [`Authority::Live`] as it is stored.
///
/// Text rather than an integer, because the value is read by a human with a
/// debugger far more often than by this module, and one byte either way is not
/// the trade. What matters is that it is **stored** rather than inferred.
const AUTHORITY_LIVE: &str = "live";

/// [`Authority::Rebuilding`] as it is stored.
const AUTHORITY_REBUILDING: &str = "rebuilding";

/// Mints the identity one store instance stamps its batches with.
///
/// A process-local ordinal, and the two properties that decide the shape are
/// both about what must *not* happen. It must not be derived from a pointer
/// address — an `Arc` can be freed and a new allocation land where the old one
/// was, so two stores would share an identity — and it must not be minted in
/// `Clone`, because a clone is the same store and must accept its origin's
/// batches.
fn mint_stamp() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// A SQLite-backed projection store.
///
/// The [`Mutex`] is load-bearing rather than defensive; see the [module
/// documentation](self) for what it buys.
#[derive(Debug, Clone)]
pub struct SqliteProjectionStore {
    connection: Arc<Mutex<Connection>>,
    /// This instance's identity, compared against the one [`SqliteBatch`]
    /// carries.
    ///
    /// Copied by `Clone` rather than re-minted: a clone shares the `Arc`, the
    /// connection and the file, so it *is* the same store and must accept the
    /// batches its origin began.
    stamp: u64,
    /// The runtime every blocking hop lands on, captured here rather than looked
    /// up at each call.
    ///
    /// ADR-0022 §9, and the same reasoning the event store records: a store
    /// constructed inside a test's runtime carries a handle out to callers that
    /// are bare OS threads, where `Handle::try_current()` finds nothing. A
    /// [`Handle`] is `Clone`, `Send`, `Sync` and `Unpin`, so carrying one costs
    /// `tests/shapes.rs` nothing.
    runtime: Option<Handle>,
}

impl SqliteProjectionStore {
    /// Wraps an already-open connection onto an **already-migrated** database.
    ///
    /// The caller is responsible for having applied the schema — use
    /// [`open`](Self::open) to have that done — and the connection is used as it
    /// is given, so a caller reaching this constructor directly should have
    /// opened it through [`crate::connection::open_configured`].
    ///
    /// This is where the batch stamp is minted and where the runtime handle is
    /// captured, because it is the point at which a caller is most likely to be
    /// inside a runtime.
    #[must_use]
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
            stamp: mint_stamp(),
            runtime: Handle::try_current().ok(),
        }
    }

    /// Opens (creating if absent) a store at `path` and applies the checkpoint
    /// schema.
    ///
    /// The connection is configured before anything is written to it — WAL, a
    /// stated `synchronous`, and a finite busy timeout — through the one path
    /// [`crate::connection::open_configured`] both stores in this crate share.
    /// Migration is idempotent and safe under a concurrent open.
    ///
    /// # Errors
    ///
    /// * [`SqliteProjectionStoreError::Sqlite`] if the file cannot be opened or
    ///   created — a missing directory, a permission refusal, a corrupt header —
    ///   or if the schema cannot be applied. Nothing partially migrated is left
    ///   behind: every statement runs inside one transaction.
    /// * [`SqliteProjectionStoreError::UnsupportedSchemaVersion`] if the file was
    ///   written by a build that knows a later migration. Operating on an unknown
    ///   schema is how a later migration loses data.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteProjectionStoreError> {
        let mut connection = crate::connection::open_configured(path)?;
        Self::migrate(&mut connection)?;
        Ok(Self::new(connection))
    }

    /// Opens a private in-memory store and applies the checkpoint schema.
    ///
    /// A convenience for a single-handle caller, and **not** what a
    /// [`ProjectionFixture`](https://docs.rs/happenstance-testkit) should reach
    /// for: a private in-memory database is per-*connection*, so a second connect
    /// would open a second, empty database rather than a second handle onto this
    /// one — and every rule that reads back through a fresh handle would fail.
    ///
    /// What it *is* is a complete store: `begin`, `commit`, `reset` and
    /// `rollback` all work against it, on the same checkpoint schema, and the
    /// only thing it cannot do is be looked at from a second connection.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteProjectionStoreError::Sqlite`] if SQLite refuses the
    /// connection or the schema cannot be applied, and
    /// [`SqliteProjectionStoreError::UnsupportedSchemaVersion`] on the version
    /// mismatch [`open`](Self::open) documents — unreachable on a database this
    /// call just created, and checked on the same path rather than on a second
    /// one.
    pub fn open_in_memory() -> Result<Self, SqliteProjectionStoreError> {
        let mut connection = Connection::open_in_memory()?;
        crate::connection::configure(&connection)?;
        Self::migrate(&mut connection)?;
        Ok(Self::new(connection))
    }

    /// Applies the schema in the module documentation, idempotently.
    ///
    /// One `BEGIN IMMEDIATE` for the whole migration, so a half-migrated file
    /// cannot exist and two concurrent opens of one path cannot race into one:
    /// the loser's `CREATE TABLE IF NOT EXISTS` is a no-op and its
    /// `INSERT OR IGNORE` keeps the winner's marker.
    ///
    /// The version is read back **inside the same transaction** that may have
    /// written it, which is what makes the marker a check rather than a decoration.
    fn migrate(connection: &mut Connection) -> Result<(), SqliteProjectionStoreError> {
        let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

        transaction.execute_batch(MIGRATION_1)?;
        #[cfg(feature = "conformance")]
        transaction.execute_batch(PROBE_TABLE)?;
        transaction.execute(
            "INSERT OR IGNORE INTO projection_meta (k, v) VALUES (?, ?)",
            rusqlite::params![SCHEMA_VERSION_KEY, SCHEMA_VERSION],
        )?;

        let found: u32 = transaction.query_row(
            "SELECT v FROM projection_meta WHERE k = ?",
            [SCHEMA_VERSION_KEY],
            |row| row.get(0),
        )?;
        if found > SCHEMA_VERSION {
            return Err(SqliteProjectionStoreError::UnsupportedSchemaVersion {
                found,
                supported: SCHEMA_VERSION,
            });
        }

        transaction.commit()?;
        Ok(())
    }

    /// A handle a `spawn_blocking` closure can own.
    ///
    /// Every real body starts here: the closure `spawn_blocking` takes must be
    /// `'static`, so it cannot borrow `self`, and the `MutexGuard` must be taken
    /// *inside* the closure — holding one across an await would make the future
    /// `!Send` and cost the adapter its [`SendProjectionStore`] impl.
    fn handle(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.connection)
    }

    /// The runtime this store's blocking work hops onto.
    ///
    /// The handle captured at construction wins; [`Handle::try_current`] is the
    /// fallback for a store built outside a runtime and driven inside one.
    ///
    /// # Errors
    ///
    /// [`SqliteProjectionStoreError::NoRuntime`] when there is neither — a store
    /// both constructed *and* driven with no runtime anywhere, which is what
    /// keeps that variant meaning something.
    fn runtime(&self) -> Result<Handle, SqliteProjectionStoreError> {
        match &self.runtime {
            Some(runtime) => Ok(runtime.clone()),
            None => Ok(Handle::try_current()?),
        }
    }

    /// Runs `work` against this store's connection on a blocking thread.
    ///
    /// The one seam every SQL-touching body goes through, so that the lock is
    /// taken and released inside a `'static` closure and no guard is ever live
    /// across an `await`.
    async fn in_blocking_task<T, F>(&self, work: F) -> Result<T, SqliteProjectionStoreError>
    where
        F: FnOnce(&mut Connection) -> Result<T, SqliteProjectionStoreError> + Send + 'static,
        T: Send + 'static,
    {
        let connection = self.handle();
        let runtime = self.runtime()?;
        let joined = runtime
            .spawn_blocking(move || {
                let mut guard = connection
                    .lock()
                    .map_err(|_| SqliteProjectionStoreError::ConnectionPoisoned)?;
                work(&mut guard)
            })
            .await;
        match joined {
            Ok(outcome) => outcome,
            Err(join) => Err(SqliteProjectionStoreError::from(join)),
        }
    }
}

/// One buffered statement and its bound parameters.
///
/// Parameters are [`rusqlite::types::Value`], the driver's own owned value type,
/// rather than `&dyn ToSql`: a batch outlives the call that filled it and is
/// carried across an await, so it can borrow nothing.
#[derive(Debug, Clone)]
pub struct PendingStatement {
    sql: Box<str>,
    params: Box<[Value]>,
}

impl PendingStatement {
    /// The statement text.
    pub fn sql(&self) -> &str {
        &self.sql
    }

    /// The bound parameters, in ordinal order.
    pub fn params(&self) -> &[Value] {
        &self.params
    }
}

/// An in-flight write against a [`SqliteProjectionStore`].
///
/// An owned, [`Send`] write set replayed inside a single transaction at
/// [`commit`](happenstance_core::ProjectionStore::commit) time — not a live
/// `rusqlite::Transaction`. See the [module documentation](self) for the two
/// compiler errors that rule the live handle out.
///
/// # Only `begin` mints one
///
/// There is no `SqliteBatch::new` and no `Default`, and their absence is the
/// stamp's teeth rather than an omission: a batch a caller could build by hand
/// carries no store's identity, so
/// [`CommitError::ForeignBatch`]
/// would have to be reported for it or waived for everything. Fill one through
/// [`push`](Self::push) — queueing the application's own read-model SQL is the
/// caller's whole job — and hand it back to `commit` or `reset`.
///
/// # The statement is fixed and the values are bound
///
/// This is the one place in the workspace a consumer is handed a SQL-text seam,
/// and the values that flow through it are exactly the bytes the library
/// guarantees it does not inspect: ADR-0003 makes payloads opaque `Bytes`,
/// forwarded and validated by nothing. So [`push`](Self::push) takes a
/// `&'static str` — a statement that exists in the source — and the values go
/// beside it as bound parameters. Assembling the text out of decoded event data
/// is a SQL injection whose source is the event log, and it commits inside the
/// same `BEGIN IMMEDIATE` that advances the checkpoint, so it is recorded as
/// *progress*: nothing replays those events and nothing re-derives the rows.
///
/// [`push_raw_sql`](Self::push_raw_sql) is the escape hatch for a statement
/// whose *shape* is genuinely computed — an `IN (…)` list sized at run time is
/// the honest case — and it is separately named so that reaching for it is a
/// decision rather than a default.
#[derive(Debug, Clone)]
pub struct SqliteBatch {
    statements: Vec<PendingStatement>,
    /// The identity of the store that began this batch.
    stamp: u64,
}

impl SqliteBatch {
    /// An empty batch stamped with `stamp`.
    ///
    /// Private, and that is the whole of the foreign-batch defence: `begin` is
    /// the only caller.
    fn stamped(stamp: u64) -> Self {
        Self {
            statements: Vec::new(),
            stamp,
        }
    }

    /// Queues a statement to run when the batch commits.
    ///
    /// # Security
    ///
    /// The statement text is fixed and the values are **bound**, never
    /// interpolated. A statement assembled out of event data at run time is a
    /// SQL injection whose source is the log, and it commits inside the same
    /// `BEGIN IMMEDIATE` that advances the checkpoint — so the projection never
    /// replays those events and nothing re-derives the rows it corrupted.
    ///
    /// That has to be a type obligation rather than a paragraph, because
    /// nothing in the gate reads prose
    /// (`standards/rust/70-rustdoc-obligations.md`, RS-70-5). The interpolated
    /// spelling does not compile:
    ///
    /// ```compile_fail,E0308
    /// use happenstance_sqlite::projection_store::SqliteBatch;
    /// use happenstance_sqlite::rusqlite::types::Value;
    ///
    /// fn queue(batch: &mut SqliteBatch, account: &str) {
    ///     batch.push(
    ///         format!("DELETE FROM balance WHERE account = '{account}'"),
    ///         core::iter::empty::<Value>(),
    ///     );
    /// }
    /// ```
    ///
    /// `&'static str` is the narrowest type that admits every statement written
    /// in source and refuses every statement assembled at run time — a literal,
    /// a `const`, a `concat!`, all of them fine. The alternative considered was
    /// a newtype minted from a literal by a macro, which buys the same
    /// guarantee and costs the caller an import and a wrapper for it. Where the
    /// shape genuinely is computed, reach for
    /// [`push_raw_sql`](Self::push_raw_sql) deliberately.
    pub fn push(&mut self, sql: &'static str, params: impl IntoIterator<Item = Value>) {
        self.push_raw_sql(sql, params);
    }

    /// Queues a statement this crate cannot see the provenance of.
    ///
    /// The unconstrained twin of [`push`](Self::push), for the one case its
    /// `&'static str` cannot express: a statement whose *shape* depends on a
    /// run-time value, of which the honest example is an `IN (…)` list sized by
    /// how many keys are being written. Build the placeholders, bind the values.
    ///
    /// # Security
    ///
    /// The obligation [`push`](Self::push) discharges in the type system moves
    /// to the caller here, in full, and this method's name is the whole of the
    /// warning: **no value may be interpolated into `sql`**. What the two
    /// callers inside this crate do is the pattern
    /// (`probe_write` and `probe_delete_all` below) — the text is fixed, every
    /// value is a [`Value`] beside it.
    ///
    /// The wrong implementation this refuses to hide is the one a reviewer
    /// waves through because it reads like the parameterised form:
    /// `push_raw_sql(format!("… WHERE account = '{account}'"), [])`, with
    /// `account` decoded out of an event payload. It runs in the same
    /// transaction that advances the checkpoint, so a successful injection is
    /// recorded as progress and no later run re-derives the corrupted rows.
    pub fn push_raw_sql(
        &mut self,
        sql: impl Into<String>,
        params: impl IntoIterator<Item = Value>,
    ) {
        self.statements.push(PendingStatement {
            sql: sql.into().into_boxed_str(),
            params: params.into_iter().collect(),
        });
    }

    /// The queued statements, in the order they will run.
    pub fn statements(&self) -> &[PendingStatement] {
        &self.statements
    }

    /// How many statements are queued.
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether nothing is queued.
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// How [`SqliteProjectionStore`] fails.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SqliteProjectionStoreError {
    /// The driver failed: I/O, `SQLITE_BUSY`, a constraint, a bad statement.
    #[error("SQLite failed: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// A thread panicked while holding the connection mutex.
    #[error("the SQLite connection mutex was poisoned by a panicking thread")]
    ConnectionPoisoned,

    /// The blocking task carrying the commit panicked or was cancelled.
    ///
    /// A cancelled commit is genuinely ambiguous — SQLite may or may not have
    /// reached `COMMIT` — which is why it is a distinct variant rather than
    /// folded into [`Sqlite`](Self::Sqlite).
    #[error("the blocking SQLite task did not complete: {0}")]
    Worker(#[from] JoinError),

    /// Called outside a tokio runtime, so no blocking task could be spawned.
    #[error("no tokio runtime is available to run the blocking SQLite work: {0}")]
    NoRuntime(#[from] TryCurrentError),

    /// A stored checkpoint row carried a position the contract cannot represent;
    /// [`SequencePosition`] wraps a `NonZeroU64`.
    #[error("stored checkpoint position {0} is not a valid sequence position")]
    InvalidPosition(i64),

    /// A stored checkpoint row carried an authority discriminant this build does
    /// not know.
    ///
    /// The alternative — resolving it to
    /// [`Authority::Live`] — is the one answer that must not be given: it tells a
    /// reader that a read model whose state this build cannot name is
    /// authoritative.
    #[error("stored checkpoint authority {0:?} is not one this build recognises")]
    InvalidAuthority(String),

    /// The file was written by a build that knows a later projection migration.
    #[error(
        "this build understands projection schema version {supported}, and the \
         database is at version {found}"
    )]
    UnsupportedSchemaVersion {
        /// The version the file carries.
        found: u32,
        /// The version this build knows.
        supported: u32,
    },

    /// A batch begun on a different store instance reached
    /// [`rollback`](happenstance_core::ProjectionStore::rollback).
    ///
    /// `commit` and `reset` report the same refusal through their own port-level
    /// variants; `rollback` returns the adapter's error, so the refusal needs a
    /// variant of its own here. Discarding it silently was the alternative and it
    /// loses for the reason the port rejects a foreign batch at all: a caller
    /// holding a batch from the wrong store has a bug, and the cheapest place to
    /// learn it is the call that was supposed to clean up.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,
}

/// What a commit that reached the database did.
///
/// A commit has two port-level outcomes and one adapter-level one, and only the
/// adapter's can be reported as [`SqliteProjectionStoreError`]. This is how the
/// blocking closure — which knows nothing about
/// [`CommitError`] — hands the regression back to the async body that can spell
/// it.
#[derive(Debug)]
enum CommitOutcome {
    /// Both halves landed.
    Committed,
    /// The attempted position was strictly below the recorded checkpoint, so
    /// nothing was written.
    Regressed {
        current: SequencePosition,
        attempted: SequencePosition,
    },
}

impl SendProjectionStore for SqliteProjectionStore {
    type Error = SqliteProjectionStoreError;

    // The same owned type as before; what left is the GAT's lifetime.
    //
    // The `E0195` transcript this line used to carry is now discharged rather
    // than recorded. It read: binding an owned type does *not* buy the relief
    // PS-5 promises, because the trait method still declares
    // `batch: Self::Batch<'_>`, so writing `batch: SqliteBatch` in the impl is
    // `error[E0195]: lifetime parameters or bounds on method 'commit' do not
    // match the trait declaration` — the literal `Self::Batch<'_>` was
    // mandatory, "and it stays mandatory until the GAT leaves the port itself".
    // The GAT has left the port. `Self::Batch` below names an owned type with
    // no lifetime to mismatch, which is what PS-5 claimed and what this impl
    // compiling is the evidence for.
    type Batch = SqliteBatch;

    // Neither `async` nor fallible — which is what this adapter said it wanted:
    // opening the batch allocates a `Vec` and the port's old `async` + `Result`
    // was a round trip it did not need. PS-6 settled it in that direction.
    //
    // It is also the only mint: the batch carries this instance's stamp, and
    // there is no public constructor that could produce one without it.
    fn begin(&self) -> Self::Batch {
        SqliteBatch::stamped(self.stamp)
    }

    /// How far `id` has been brought, and whether its rows are authoritative.
    ///
    /// # Errors
    ///
    /// * the connection mutex was poisoned, or the blocking task did not
    ///   complete, or there was no runtime to run it on;
    /// * the driver failed;
    /// * the stored row carried a position that is not a
    ///   [`SequencePosition`], or an authority discriminant this build does not
    ///   know — both are reported rather than resolved to a default.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let id = id.clone();
        self.in_blocking_task(move |connection| read_checkpoint(connection, &id))
            .await
    }

    /// Applies `batch` and moves `id`'s checkpoint to `position`, as one unit.
    ///
    /// One `BEGIN IMMEDIATE` carries all of it: the recorded checkpoint is read
    /// under the same write lock that will overwrite it, then every queued
    /// statement runs in the order it was pushed, then the checkpoint is upserted
    /// with the authority this commit claims. Reading the checkpoint *before* the
    /// transaction opens is the defect that shape exists to avoid — two commits
    /// would interleave between the read and the write and both would pass.
    ///
    /// # Errors
    ///
    /// * [`CommitError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance. Decided before any statement is prepared, so a rejected commit
    ///   cannot have moved anything.
    /// * [`CommitError::CheckpointRegression`] if `position` is strictly below
    ///   the recorded checkpoint. An equal position is accepted, which PS-22
    ///   permits.
    /// * [`CommitError::Store`] if the connection mutex was poisoned, the
    ///   blocking task did not complete, there was no runtime to run it on, the
    ///   driver failed, or the recorded row could not be read back.
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

        let id = id.clone();
        let outcome = self
            .in_blocking_task(move |connection| {
                commit_locked(connection, &batch, &id, position, authority)
            })
            .await
            .map_err(CommitError::Store)?;

        match outcome {
            CommitOutcome::Committed => Ok(()),
            CommitOutcome::Regressed { current, attempted } => {
                Err(CommitError::CheckpointRegression { current, attempted })
            }
        }
    }

    /// Applies `batch` and returns `id` to
    /// [`Checkpoint::NeverRun`], as one unit.
    ///
    /// `commit`'s dual, in one `BEGIN IMMEDIATE`: the caller's own deletes, then
    /// the removal of this projection's checkpoint row — removal rather than a
    /// sentinel position, because the absence of a row *is* `NeverRun` and a
    /// sentinel would make a reset indistinguishable from a commit at the first
    /// position.
    ///
    /// It deletes nothing this adapter chose. The read model is the caller's, and
    /// an adapter that truncated a table of its own naming would be scoped to
    /// something the port deliberately never told it.
    ///
    /// # Errors
    ///
    /// * [`ResetError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance, decided before any statement is prepared.
    /// * [`ResetError::Store`] if the connection mutex was poisoned, the blocking
    ///   task did not complete, there was no runtime to run it on, or the driver
    ///   failed.
    ///
    /// [`ResetError::Refused`] is never returned: this store holds no protection
    /// policy, which is PS-18's mechanism left unexercised by a store that has
    /// nothing to protect.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        let id = id.clone();
        self.in_blocking_task(move |connection| reset_locked(connection, &batch, &id))
            .await
            .map_err(ResetError::Store)
    }

    /// Discards the batch without committing.
    ///
    /// Nothing was ever sent to SQLite, so this is a drop — which is also the
    /// evidence PS-7 asks for: for a buffering adapter, "dropping a batch must
    /// roll back" is free, and the store is usable afterwards because nothing was
    /// ever checked out to return.
    ///
    /// # Errors
    ///
    /// [`SqliteProjectionStoreError::ForeignBatch`] if `batch` was begun on a
    /// different store instance. The batch is consumed either way; the refusal is
    /// how a caller learns it was holding the wrong one, on the call that was
    /// meant to be the cleanup.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        if batch.stamp != self.stamp {
            return Err(SqliteProjectionStoreError::ForeignBatch);
        }
        drop(batch);
        Ok(())
    }
}

/// The conformance suite's write and read seam.
///
/// It lives here, in `src/`, and not in `tests/projection.rs`: that is a
/// different crate, where neither [`ProjectionProbe`] nor
/// [`SqliteProjectionStore`] is local and the orphan rule answers
/// `error[E0117]`. The feature that turns it on forwards one flag on a dependency
/// this crate already has.
///
/// [`ProjectionProbe`]: happenstance_core::ProjectionProbe
#[cfg(feature = "conformance")]
impl happenstance_core::ProjectionProbe for SqliteProjectionStore {
    /// `false`, and it is the honest answer rather than a gap.
    ///
    /// A [`SqliteBatch`] is a list of statements that has been sent to SQLite
    /// exactly never, so there is no open transaction to read through — and
    /// answering from *committed* state is what PS-12 forbids by name, because a
    /// projection doing `get` then `set` inside one batch would then read the
    /// value from before the batch began and lose every increment after the
    /// first.
    ///
    /// The alternative is rejected on the record: a shadow map inside the batch
    /// would answer these reads from a second source of truth SQLite never sees,
    /// and would buy a green `batch_reads_reflect_pending_writes` that says
    /// nothing about this adapter's real read path. PS-12 permits `false`
    /// outright, and the two rules that would have used it are emitted as
    /// reported skips rather than omitted.
    const READS_THROUGH_BATCH: bool = false;

    /// Queues one probe row. Synchronous and infallible, because queueing SQL
    /// into a buffer the caller owns cannot fail.
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.push(
            "INSERT INTO projection_probe (k, v) VALUES (?, ?) \
             ON CONFLICT(k) DO UPDATE SET v = excluded.v",
            [
                Value::Text(key.to_owned()),
                // `cast_signed` rather than a fallible conversion: SQLite's
                // INTEGER is signed, and the bit pattern round-trips every `u64`
                // exactly, where a saturating conversion would map two values to
                // one.
                Value::Integer(value.cast_signed()),
            ],
        );
    }

    /// Queues removal of every probe row, so that
    /// [`reset`](happenstance_core::ProjectionStore::reset) can be checked
    /// without the suite knowing what a read model is.
    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.push("DELETE FROM projection_probe", core::iter::empty());
    }

    /// Reads one probe row from **committed** state, through the same blocking
    /// seam as every other method here.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        let key = key.to_owned();
        self.in_blocking_task(move |connection| {
            let stored: Option<i64> = connection
                .query_row(
                    "SELECT v FROM projection_probe WHERE k = ?",
                    [&key],
                    |row| row.get(0),
                )
                .optional()?;
            Ok(stored.map(i64::cast_unsigned))
        })
        .await
    }

    /// Never called, because
    /// [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
    /// is `false`.
    ///
    /// # Panics
    ///
    /// Always. The specification permits exactly this for an adapter whose batch
    /// has no read path, and the panic is the honest answer: any value returned
    /// here would be a claim about pending writes SQLite has never been told
    /// about.
    fn probe_read_through(&self, _batch: &Self::Batch, _key: &str) -> Option<u64> {
        unimplemented!(
            "SqliteBatch buffers its statements, so `READS_THROUGH_BATCH` is \
             `false` and this is never called: there is no open transaction to \
             read through, and answering from committed state is what PS-12 \
             forbids"
        )
    }
}

/// Reads `id`'s checkpoint, or [`Checkpoint::NeverRun`] when it has no row.
///
/// The absence of a row is the `NeverRun` state, which is why a successful
/// `reset` deletes rather than writing a sentinel.
fn read_checkpoint(
    connection: &Connection,
    id: &ProjectionId,
) -> Result<Checkpoint, SqliteProjectionStoreError> {
    let row: Option<(i64, String)> = connection
        .query_row(
            "SELECT position, authority FROM projection_checkpoint WHERE projection_id = ?",
            [id.as_str()],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;

    let Some((stored_position, stored_authority)) = row else {
        return Ok(Checkpoint::NeverRun);
    };

    let through = position_from_row(stored_position)?;
    match stored_authority.as_str() {
        AUTHORITY_LIVE => Ok(Checkpoint::Live { through }),
        AUTHORITY_REBUILDING => Ok(Checkpoint::Rebuilding { through }),
        _ => Err(SqliteProjectionStoreError::InvalidAuthority(
            stored_authority,
        )),
    }
}

/// The position `id`'s checkpoint row records, if it has one.
///
/// Separate from [`read_checkpoint`] because the regression guard is about the
/// position alone: a commit is refused for going backwards whatever the recorded
/// authority claimed.
fn read_position(
    connection: &Connection,
    id: &ProjectionId,
) -> Result<Option<SequencePosition>, SqliteProjectionStoreError> {
    let stored: Option<i64> = connection
        .query_row(
            "SELECT position FROM projection_checkpoint WHERE projection_id = ?",
            [id.as_str()],
            |row| row.get(0),
        )
        .optional()?;

    stored.map(position_from_row).transpose()
}

/// One transaction: the regression guard, the caller's statements, the
/// checkpoint.
fn commit_locked(
    connection: &mut Connection,
    batch: &SqliteBatch,
    id: &ProjectionId,
    position: SequencePosition,
    authority: Authority,
) -> Result<CommitOutcome, SqliteProjectionStoreError> {
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

    // Inside the write lock, not before it: a guard evaluated before `BEGIN
    // IMMEDIATE` lets two commits interleave between the read and the write and
    // both pass.
    if let Some(current) = read_position(&transaction, id)?
        && position < current
    {
        // The transaction is dropped un-committed, so the caller's statements
        // never reach the file.
        return Ok(CommitOutcome::Regressed {
            current,
            attempted: position,
        });
    }

    apply(&transaction, batch)?;
    transaction.execute(
        "INSERT INTO projection_checkpoint (projection_id, position, authority) VALUES (?, ?, ?) \
         ON CONFLICT(projection_id) DO UPDATE SET \
         position = excluded.position, authority = excluded.authority",
        rusqlite::params![id.as_str(), as_i64(position), stored_authority(authority)],
    )?;

    transaction.commit()?;
    Ok(CommitOutcome::Committed)
}

/// One transaction: the caller's deletes, then the checkpoint row's removal.
fn reset_locked(
    connection: &mut Connection,
    batch: &SqliteBatch,
    id: &ProjectionId,
) -> Result<(), SqliteProjectionStoreError> {
    let transaction = connection.transaction_with_behavior(TransactionBehavior::Immediate)?;

    apply(&transaction, batch)?;
    transaction.execute(
        "DELETE FROM projection_checkpoint WHERE projection_id = ?",
        [id.as_str()],
    )?;

    transaction.commit()?;
    Ok(())
}

/// Replays every queued statement, in the order it was pushed.
fn apply(
    transaction: &rusqlite::Transaction<'_>,
    batch: &SqliteBatch,
) -> Result<(), SqliteProjectionStoreError> {
    for statement in batch.statements() {
        transaction.execute(statement.sql(), params_from_iter(statement.params()))?;
    }
    Ok(())
}

/// A [`SequencePosition`] as the integer SQLite stores.
fn as_i64(position: SequencePosition) -> i64 {
    i64::try_from(position.get()).unwrap_or(i64::MAX)
}

/// A stored position as a [`SequencePosition`], or the error that says why not.
///
/// Zero and negatives are rejected rather than resolved: `SequencePosition` wraps
/// a `NonZeroU64`, and both silent answers available — `Live { through: 1 }`, and
/// `NeverRun` — are a lie about a row that exists.
fn position_from_row(stored: i64) -> Result<SequencePosition, SqliteProjectionStoreError> {
    u64::try_from(stored)
        .ok()
        .and_then(SequencePosition::new)
        .ok_or(SqliteProjectionStoreError::InvalidPosition(stored))
}

/// An [`Authority`] as the discriminant this schema stores.
///
/// [`Authority`] is `#[non_exhaustive]`, so a build compiled against a later
/// contract can meet a variant this arm list has never been told about. It is
/// stored as `rebuilding` — *not authoritative* — because that is the direction a
/// reader can recover from: treating an unknown claim as `Live` tells a dashboard
/// that rows nobody can vouch for are the truth.
fn stored_authority(authority: Authority) -> &'static str {
    match authority {
        Authority::Live => AUTHORITY_LIVE,
        _ => AUTHORITY_REBUILDING,
    }
}
