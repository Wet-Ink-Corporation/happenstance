//! SQLite-backed [`SendProjectionStore`].
//!
//! # Status: bodies unimplemented, types real
//!
//! SQLite is the adapter that makes the checkpoint invariant easy to honour:
//! the read-model writes and the checkpoint update share one transaction, so
//! there is no window in which they can disagree.
//!
//! # Why the batch is a buffer and not a `rusqlite::Transaction`
//!
//! The port's own documentation says a batch *is* a live transaction, and
//! justifies the lifetime on `type Batch<'a>` with "a transaction cannot outlive
//! its connection". For this adapter, on the flavour this adapter must
//! implement, that is not available. `type Batch<'a> = rusqlite::Transaction<'a>`
//! fails on [`SendProjectionStore`] for
//! two independent reasons, each confirmed against this crate:
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
//! satisfies the port's actual obligation (read model and checkpoint move
//! together) without the port's suggested mechanism.
//!
//! # Intended schema
//!
//! ```sql
//! CREATE TABLE projection_checkpoint (
//!     projection_id TEXT    PRIMARY KEY,
//!     position      INTEGER NOT NULL
//! ) WITHOUT ROWID;
//! ```
//!
//! Read-model tables themselves are the application's business; this adapter
//! owns only the checkpoint and the transaction that carries it.

use std::path::Path;
use std::sync::{Arc, Mutex};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};
use rusqlite::Connection;
use rusqlite::types::Value;
use tokio::runtime::TryCurrentError;
use tokio::task::JoinError;

/// A SQLite-backed projection store.
///
/// The [`Mutex`] is load-bearing rather than defensive; see the [module
/// documentation](self) for what it buys.
#[derive(Debug, Clone)]
pub struct SqliteProjectionStore {
    connection: Arc<Mutex<Connection>>,
}

impl SqliteProjectionStore {
    /// Wraps an already-open connection.
    #[must_use]
    pub fn new(connection: Connection) -> Self {
        Self {
            connection: Arc::new(Mutex::new(connection)),
        }
    }

    /// Opens (creating if absent) a store at `path` and applies the checkpoint
    /// schema.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteProjectionStoreError::Sqlite`] if the file cannot be
    /// opened or the schema cannot be applied.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, SqliteProjectionStoreError> {
        let connection = Connection::open(path)?;
        Self::migrate(&connection)?;
        Ok(Self::new(connection))
    }

    /// Opens a private in-memory store and applies the checkpoint schema.
    ///
    /// # Errors
    ///
    /// Returns [`SqliteProjectionStoreError::Sqlite`] if SQLite refuses the
    /// connection or the schema cannot be applied.
    pub fn open_in_memory() -> Result<Self, SqliteProjectionStoreError> {
        let connection = Connection::open_in_memory()?;
        Self::migrate(&connection)?;
        Ok(Self::new(connection))
    }

    /// Applies the schema in the module documentation.
    fn migrate(_connection: &Connection) -> Result<(), SqliteProjectionStoreError> {
        todo!("SQLite projection store: schema migration")
    }

    /// A handle a `spawn_blocking` closure can own.
    ///
    /// Every real body starts here: the closure `spawn_blocking` takes must be
    /// `'static`, so it cannot borrow `self`, and the `MutexGuard` must be taken
    /// *inside* the closure — holding one across an await would make the future
    /// `!Send` and cost the adapter its `SendProjectionStore` impl.
    fn handle(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.connection)
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
#[derive(Debug, Clone, Default)]
pub struct SqliteBatch {
    statements: Vec<PendingStatement>,
}

impl SqliteBatch {
    /// An empty batch.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a statement to run when the batch commits.
    pub fn push(&mut self, sql: impl Into<String>, params: impl IntoIterator<Item = Value>) {
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
    fn begin(&self) -> Self::Batch {
        SqliteBatch::new()
    }

    async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let _connection = self.handle();
        todo!("SQLite projection store: read checkpoint")
    }

    async fn commit(
        &self,
        _batch: Self::Batch,
        _id: &ProjectionId,
        _position: SequencePosition,
        _authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        // The real body opens one transaction, replays every statement, writes
        // the checkpoint row, and commits — which is how an owned batch keeps
        // the invariant the port cares about.
        let _connection = self.handle();
        todo!("SQLite projection store: replay batch and advance checkpoint")
    }

    async fn reset(
        &self,
        _batch: Self::Batch,
        _id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        // `commit`'s dual, and the same single transaction: replay the caller's
        // own deletes, then remove the checkpoint row rather than writing a
        // sentinel into it.
        let _connection = self.handle();
        todo!("SQLite projection store: replay the caller's deletes and clear the checkpoint")
    }

    async fn rollback(&self, _batch: Self::Batch) -> Result<(), Self::Error> {
        // Nothing was ever sent to SQLite, so this is a drop. Recorded rather
        // than elided: it is evidence for PS-7, which asks whether dropping a
        // batch must roll back — for a buffering adapter that is free.
        Ok(())
    }
}
