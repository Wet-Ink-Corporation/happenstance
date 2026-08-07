//! LadybugDB-backed [`ProjectionStore`](happenstance_core::ProjectionStore),
//! with a deferred write set as its batch.
//!
//! # The shape, and why it is this shape
//!
//! LadybugDB exposes **no transaction handle type**. Its Rust crate has a
//! `Database`, a `Connection<'db>`, a `PreparedStatement` and a `QueryResult`,
//! and that is the whole surface; a transaction is entered by *executing a
//! statement* — `BEGIN TRANSACTION` — and left by executing `COMMIT` or
//! `ROLLBACK` on the same connection
//! (`docs.ladybugdb.com/cypher/transaction/`). Nothing in the type system
//! records that a transaction is open.
//!
//! That fact settles the batch shape here, and it settles it in the direction
//! PS-4 hopes for. There is no live handle to *hold*, so the natural batch is
//! the **deferred write set** PS-4 describes: an owned list of parameterised
//! Cypher statements, replayed inside one `BEGIN TRANSACTION` … `COMMIT` in
//! [`commit`](happenstance_core::ProjectionStore::commit), with the checkpoint
//! write as the last statement before the commit. See
//! [`crate::live_handle`] for the shape this crate deliberately *also* builds,
//! and [What this does not settle](#what-this-does-not-settle) for the half of
//! PS-4 that no skeleton can close.
//!
//! # Why the store owns its `Database` and opens connections per call
//!
//! `Connection::new` takes `&'db Database`, so a struct holding both a
//! `Database` and a long-lived `Connection` into it is self-referential and
//! will not compile. The alternative — giving the *store* a lifetime and
//! letting the caller own the `Database` — compiles as a struct and then
//! **crashes the compiler** when it implements this port; see
//! [`crate::live_handle`] for the transcript and the reason.
//!
//! So the store owns the `Database` and builds a connection inside each method.
//! That is also LadybugDB's own documented pattern — "each Ti obtains a
//! connection from `db` and concurrently issues read or write queries. This is
//! safe" (`docs.ladybugdb.com/concurrency/`) — and it is what keeps the store
//! `'static`, which every runner that wants to own one needs.
//!
//! # The blocking question
//!
//! Every `lbug` method blocks and the port's methods are `async`. Because the
//! store is `'static` and owns its database, `tokio::task::spawn_blocking` is
//! *available* here — a connection can be built inside the spawned closure —
//! but it is not used, because reaching for it would put a tokio dependency in
//! a runtime-agnostic adapter. Phase 11 decides between blocking the executor
//! thread and a runtime-gated `spawn_blocking` feature; the skeleton records
//! that the owned-`Database` layout is what leaves both open.
//!
//! # What this does not settle
//!
//! PS-4 names this adapter as its falsifier on two conditions:
//!
//! * *"if `lbug`'s graph mutations cannot be expressed as a replayable
//!   statement list"* — they can. Parameterised Cypher is the only mutation
//!   surface the crate offers; there is no builder that holds a handle.
//!   **PS-4 survives this half, and the type checker agrees.**
//! * *"or if its write handle must exist before a traversal that the
//!   projection's own logic depends on"* — **not settled, and not settleable by
//!   a skeleton.** A projection that must `MATCH` a node it created earlier in
//!   the same batch gets a different answer from a deferred write set than from
//!   a live transaction, because with a deferred set the traversal runs at
//!   commit time against a graph that does not yet contain the write. That is a
//!   run-time capability limit, not a compile error. Phase 11 closes it by
//!   writing a projection that needs read-your-own-writes.

use happenstance_core::{ProjectionId, SendProjectionStore, SequencePosition};

use crate::stand_in::{self, Connection, Database, Value};

/// One parameterised Cypher statement, buffered until commit.
///
/// Parameters are carried beside the text rather than interpolated into it,
/// because the replay in `commit` goes through
/// `Connection::prepare`/`Connection::execute`, and a statement that has
/// already been stringified cannot be prepared once and executed many times.
#[derive(Debug, Clone)]
pub struct GraphStatement {
    cypher: Box<str>,
    parameters: Vec<(Box<str>, Value)>,
}

impl GraphStatement {
    /// Buffers a statement and its named parameters.
    pub fn new(cypher: impl Into<String>, parameters: Vec<(String, Value)>) -> Self {
        Self {
            cypher: cypher.into().into_boxed_str(),
            parameters: parameters
                .into_iter()
                .map(|(name, value)| (name.into_boxed_str(), value))
                .collect(),
        }
    }

    /// The Cypher text.
    pub fn cypher(&self) -> &str {
        &self.cypher
    }

    /// The named parameters, in the order they were given.
    pub fn parameters(&self) -> &[(Box<str>, Value)] {
        &self.parameters
    }
}

/// An owned, deferred set of graph mutations — this adapter's `Batch`.
///
/// This is the *owned-handle, non-SQL* shape the instrument portfolio exists to
/// supply. It holds no connection, no transaction and no borrow, so it is
/// `Send + 'static`: it can cross a thread, sit in a collection, or be held
/// across an await by a runner that has not yet decided to commit.
///
/// Nothing executes until
/// [`commit`](happenstance_core::ProjectionStore::commit) replays it inside one
/// `BEGIN TRANSACTION` … `COMMIT` together with the checkpoint write, which is
/// how PS-1's atomicity is met without a live handle (PS-4).
#[derive(Debug, Clone, Default)]
pub struct GraphWriteSet {
    statements: Vec<GraphStatement>,
}

impl GraphWriteSet {
    /// An empty write set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a statement to the write set.
    pub fn push(&mut self, statement: GraphStatement) {
        self.statements.push(statement);
    }

    /// The buffered statements, in the order they will be replayed.
    pub fn statements(&self) -> &[GraphStatement] {
        &self.statements
    }

    /// The number of buffered statements.
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether the write set is empty.
    ///
    /// An empty write set is still worth committing: the checkpoint must
    /// advance past events that produced no graph mutation, or a restart
    /// replays them forever.
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// How the LadybugDB projection stores fail.
///
/// Shared by [`LadybugProjectionStore`] and
/// [`LiveHandleProjectionStore`](crate::live_handle::LiveHandleProjectionStore)
/// because both fail in exactly the same ways — which is itself evidence that
/// an error enum is not sensitive to the batch shape.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LadybugProjectionStoreError {
    /// The driver rejected a statement, or the C++ side threw.
    #[error("the LadybugDB driver failed")]
    Driver(#[from] stand_in::Error),

    /// `COMMIT` itself failed, after every statement in the write set had been
    /// accepted.
    ///
    /// Distinct from [`Driver`](Self::Driver) because it is the one failure
    /// where the caller learns nothing about *which* write was at fault, and
    /// the only correct response is to rebuild the batch and retry rather than
    /// to fix a statement.
    #[error("committing the write set failed")]
    Commit(#[source] stand_in::Error),

    /// A checkpoint was stored but is not a valid position.
    ///
    /// LadybugDB's widest integer property is `INT64` and
    /// [`SequencePosition`] is a `NonZeroU64`, so the round trip is a checked
    /// conversion in *both* directions. A zero or negative property is a
    /// corrupted checkpoint, not a missing one, and collapsing the two into
    /// `Ok(None)` would silently replay a projection from the beginning.
    #[error(
        "checkpoint for projection `{projection}` holds {value}, which is not a valid position"
    )]
    MalformedCheckpoint {
        /// The projection whose checkpoint is unusable.
        projection: String,
        /// The value found in the checkpoint property.
        value: i64,
    },

    /// A position too large for LadybugDB to store.
    ///
    /// The other direction of the same narrowing: a position above `i64::MAX`
    /// cannot be written to an `INT64` property at all. Unreachable in practice
    /// and cheap to state, which is the correct trade for a conversion that
    /// would otherwise want an `unwrap`.
    #[error("position {position} exceeds the INT64 range LadybugDB stores")]
    PositionOutOfRange {
        /// The position that could not be stored.
        position: SequencePosition,
    },

    /// Another write transaction is already open on this database.
    ///
    /// LadybugDB permits many concurrent readers and exactly one writer
    /// (`docs.ladybugdb.com/cypher/transaction/`). Two commits racing is
    /// therefore routine rather than exceptional, and the caller's correct
    /// response is to retry — so it gets its own variant rather than arriving
    /// as an opaque [`Driver`](Self::Driver) string that every caller would
    /// have to pattern-match on.
    #[error("another write transaction is already open on this database")]
    WriteTransactionInUse,
}

/// A LadybugDB-backed projection store whose batch is a deferred write set.
///
/// # Status: skeleton
///
/// The types are real and the bodies are `todo!()`. See the
/// [module documentation](self) for why it owns its database and why its batch
/// buffers rather than borrows.
#[derive(Debug)]
pub struct LadybugProjectionStore {
    database: Database,
}

impl LadybugProjectionStore {
    /// Wraps an open database.
    ///
    /// Takes the `Database` by value rather than by reference: a borrowed
    /// database makes the store non-`'static`, and a non-`'static` store cannot
    /// implement this port at all on rustc 1.97.1 — see [`crate::live_handle`].
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Opens a connection for one operation.
    ///
    /// Per-call rather than pooled, which is LadybugDB's documented pattern and
    /// the only one available to a store that owns its database.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if the driver cannot
    /// open a connection.
    pub fn connect(&self) -> Result<Connection<'_>, LadybugProjectionStoreError> {
        Ok(Connection::new(&self.database)?)
    }
}

// The `Send` flavour, and on evidence rather than convenience: `lbug` writes
// `impl Send` and `impl Sync` for both `Database` and `Connection`, so `Self:
// Send` and `&Self: Send` both hold and the derived futures can carry `+ Send`.
// Had the bindings been `!Send` — which cxx-backed bindings often are — the
// bare `ProjectionStore` would have been the only honest choice, and *that*
// would have been the finding.
impl SendProjectionStore for LadybugProjectionStore {
    type Error = LadybugProjectionStoreError;

    // Today's port declares `type Batch<'a> where Self: 'a`. Binding an *owned*
    // type to it adopts phase 6's hypothesis without pre-empting the decision:
    // the lifetime is accepted and then unused, so dropping the GAT later is a
    // deletion here rather than a redesign.
    type Batch<'a>
        = GraphWriteSet
    where
        Self: 'a;

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Option<SequencePosition>, Self::Error> {
        let _ = id;
        todo!("phase 11 implements this")
    }

    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
        todo!("phase 11 implements this")
    }

    async fn commit(
        &self,
        batch: Self::Batch<'_>,
        id: &ProjectionId,
        position: SequencePosition,
    ) -> Result<(), Self::Error> {
        let _ = (batch, id, position);
        todo!("phase 11 implements this")
    }

    async fn rollback(&self, batch: Self::Batch<'_>) -> Result<(), Self::Error> {
        let _ = batch;
        todo!("phase 11 implements this")
    }
}
