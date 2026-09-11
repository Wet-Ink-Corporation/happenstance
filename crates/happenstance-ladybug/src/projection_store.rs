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
//! `experiments/live-handle-projection-batch/` for the live borrowed shape this
//! crate deliberately *also* built — ADR-0017 moved it there when the port's
//! owned `type Batch;` stopped admitting it.
//!
//! # The schema, in full
//!
//! ```text
//! CREATE NODE TABLE IF NOT EXISTS __hs_checkpoint(
//!     projection STRING, position UINT64, authority STRING, PRIMARY KEY(projection))
//! ```
//!
//! One node per [`ProjectionId`], and **in the graph rather than beside it**
//! (ADR-0025 §1). A `BEGIN TRANSACTION` … `COMMIT` on one connection is the only
//! atomicity LadybugDB offers, and PS-1 requires the read-model write and the
//! checkpoint write to become durable together; a sidecar file or a second store
//! would put them in two failure domains, which is the invariant's whole subject.
//!
//! `UINT64`, and `INT64` is **refuted**. `experiments/ladybug-driver-probes/`
//! stored `u64::MAX - 1` into a `UINT64` column and read it back exactly, so
//! [`SequencePosition`]'s `NonZeroU64` fits with no narrowing at all — unlike
//! `happenstance-sqlite` and `happenstance-postgres`, where a signed `bigint`
//! loses the top half of the domain and both adapters carry a fallible converter
//! for it. The variant that converter would have needed here,
//! `PositionOutOfRange`, is gone.
//!
//! The absence of a node **is** [`Checkpoint::NeverRun`], which is why a
//! successful [`reset`](happenstance_core::ProjectionStore::reset) deletes the
//! node rather than writing a sentinel at position zero: a sentinel would make
//! "never run" and "committed at the first position" the same state and skip
//! event 1 permanently and silently.
//!
//! Read models themselves are the application's business; this adapter owns only
//! the checkpoint node and the transaction that carries it. That is also why
//! `reset` applies the *caller's* deletes — an adapter that emptied a node table
//! of its own choosing would be inventing a read model it does not own.
//!
//! Under `feature = "conformance"` one more node table appears —
//! `__hs_probe(k, v)`, the conformance suite's own read model, written through
//! [`ProjectionProbe`](happenstance_core::ProjectionProbe). It is behind that
//! feature and not behind a runtime flag, so it cannot reach an application's
//! graph.
//!
//! # Why the store owns an `Arc<Database>` and opens connections per call
//!
//! Two constraints, and the second was measured rather than reasoned about.
//!
//! `Connection::new` takes `&'db Database`, so a struct holding both a
//! `Database` and a long-lived `Connection` into it is self-referential and will
//! not compile. The alternative — giving the *store* a lifetime and letting the
//! caller own the `Database` — compiles as a struct and then **crashed the
//! compiler** when it implemented the GAT-era port; see
//! `experiments/live-handle-projection-batch/` for the transcript.
//!
//! And the `Arc` is forced: a second `Database::new` on the same directory is
//! **refused by a file lock**, so a second handle onto one backing store cannot
//! be a second `Database`. Two `Connection`s over one shared `Arc<Database>`
//! work, and the second observes what the first committed — which is the
//! out-of-connection observability PS-1's coupling is only visible through
//! (ADR-0025 §6).
//!
//! Opening a connection per call is also LadybugDB's own documented pattern —
//! "each Ti obtains a connection from `db` and concurrently issues read or write
//! queries. This is safe" (`docs.ladybugdb.com/concurrency/`) — and it is what
//! keeps the store `'static`, which every runner that wants to own one needs.
//!
//! # The blocking question
//!
//! Every `lbug` method blocks and the port's methods are `async`. ADR-0025 §3
//! settles it **blocking-only**, because reaching for `spawn_blocking` would put
//! a tokio dependency in a runtime-agnostic adapter — the cost this crate's
//! module documentation already names.
//!
//! **The reason this paragraph used to give for the alternative being open was
//! wrong, and it is corrected rather than deleted, because it is the kind of
//! wrong a reader would act on.** It said `spawn_blocking` is *available* here
//! because the store is `'static` and owns its database. It is not available on
//! those grounds: `spawn_blocking` requires `FnOnce + Send + 'static`, and every
//! method of this port takes `&self`. Whether a closure can be `'static` is a
//! decision about the store's **fields** — an `Arc<Database>` rather than a
//! `Database` — and not about the call site. The `Arc` above is taken for an
//! unrelated reason, which is what would make the feature possible later; it is
//! not what makes it unnecessary now.
//!
//! Runtime-agnosticism is falsified rather than asserted:
//! `tests/projection.rs` mounts the conformance suite **twice**, once under
//! `happenstance_testkit::__emit_projection_blocking`, which needs no runtime at
//! all. A store that reached for `spawn_blocking` panics under the first mount.
//!
//! Claim that narrowly. The tokio emitter expands to `#[tokio::test]`, which is a
//! **current-thread** runtime running one task, so blocking in place starves
//! nothing and the two emitters cannot diverge on account of blocking. What the
//! pair falsifies is *runtime-agnosticism*, not the cost of blocking.
//!
//! # What PS-4's second condition turned out to be
//!
//! PS-4 names this adapter as its falsifier on two conditions.
//!
//! * *"if `lbug`'s graph mutations cannot be expressed as a replayable statement
//!   list"* — they can. Parameterised Cypher is the only mutation surface the
//!   crate offers; there is no builder that holds a handle. **PS-4 survives this
//!   half, and the type checker agrees.**
//! * *"or if its write handle must exist before a traversal that the projection's
//!   own logic depends on"* — this half **has two readings and they get different
//!   answers** (ADR-0025 §4).
//!
//!   The *Cypher-level* reading — statement *n* matching what statement *n−1*
//!   wrote — **did not fire**. The probe opened a transaction, created a node and
//!   matched it back inside the same transaction, and the write was visible. A
//!   deferred write set answers this correctly because replay is one connection,
//!   one transaction, in order. State that narrowly: PS-4's condition did not fire
//!   *for this adapter*, on this engine. It is not a discharge of a clause that
//!   generalises over write-behind shapes the probe says nothing about.
//!
//!   The *Rust-level* reading — `apply` needing the traversal's **value** to
//!   decide what to buffer next — is foreclosed by the port for every batch
//!   shape, because `Projection::apply` is synchronous and a traversal is I/O.
//!   That is a finding about the port rather than about LadybugDB, and it is true
//!   of `happenstance-sqlite` and `happenstance-postgres` identically.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
    SequencePosition,
};

use lbug::{Connection, Database, SystemConfig, Value};

/// The node table this adapter's checkpoints live in.
///
/// `IF NOT EXISTS` because [`LadybugProjectionStore::open`] applies the schema on
/// **every** open and a fixture opens one directory more than once.
const CHECKPOINT_TABLE: &str = "CREATE NODE TABLE IF NOT EXISTS __hs_checkpoint(\
     projection STRING, position UINT64, authority STRING, PRIMARY KEY(projection))";

/// The conformance suite's own read model, created under the same `cfg` as the
/// [`ProjectionProbe`](happenstance_core::ProjectionProbe) impl that writes it.
///
/// Behind the feature rather than behind a runtime `if`, for
/// `happenstance-sqlite`'s reason: a test read model shipped inside an
/// application's graph is a defect no test in this repository could catch,
/// because every test enables the feature.
#[cfg(feature = "conformance")]
const PROBE_TABLE: &str =
    "CREATE NODE TABLE IF NOT EXISTS __hs_probe(k STRING, v UINT64, PRIMARY KEY(k))";

/// Enters the one transaction every write goes through.
const BEGIN: &str = "BEGIN TRANSACTION";

/// Ends it, making both halves durable together.
const COMMIT: &str = "COMMIT";

/// Abandons it — issued **only** where no statement has failed. See
/// [`LadybugProjectionStore::commit`] for why that qualification is the whole of
/// ADR-0025 §7.
const ROLLBACK: &str = "ROLLBACK";

/// Reads one projection's checkpoint node.
const SELECT_CHECKPOINT: &str =
    "MATCH (c:__hs_checkpoint {projection: $projection}) RETURN c.position, c.authority";

/// Writes it. `MERGE` rather than `CREATE`, because a projection commits many
/// times and only the first would be a creation.
const UPSERT_CHECKPOINT: &str = "MERGE (c:__hs_checkpoint {projection: $projection}) \
     SET c.position = $position, c.authority = $authority";

/// Removes it. Removal rather than a sentinel position: the absence of a node
/// *is* [`Checkpoint::NeverRun`].
const DELETE_CHECKPOINT: &str = "MATCH (c:__hs_checkpoint {projection: $projection}) DELETE c";

/// [`Authority::Live`] as it is stored.
///
/// Text rather than a discriminant, for `happenstance-sqlite`'s reason: the value
/// is read by a human with a debugger far more often than by this module. What
/// matters is that it is **stored** rather than inferred.
const AUTHORITY_LIVE: &str = "live";

/// [`Authority::Rebuilding`] as it is stored.
const AUTHORITY_REBUILDING: &str = "rebuilding";

/// Mints the identity one store instance stamps its write sets with.
///
/// A process-local ordinal, for the two reasons `happenstance-sqlite` records:
/// it must not be derived from a pointer address, because an `Arc` can be freed
/// and a new allocation land where the old one was, and it must not be minted in
/// `Clone`, because a clone is the same store and must accept its origin's write
/// sets.
fn mint_stamp() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}

/// One parameterised Cypher statement, buffered until commit.
///
/// Parameters are carried beside the text rather than interpolated into it,
/// because the replay in `commit` goes through
/// `Connection::prepare`/`Connection::execute`, and a statement that has
/// already been stringified cannot be prepared once and executed many times.
///
/// There is **no public constructor**, which is the seam ADR-0025 §2 asks for:
/// one is minted only by [`GraphWriteSet::push`] or its separately-named escape
/// hatch, so the provenance of the Cypher text is decided at the push and not
/// here.
#[derive(Debug, Clone)]
pub struct GraphStatement {
    cypher: Box<str>,
    parameters: Vec<(&'static str, Value)>,
}

impl GraphStatement {
    /// The Cypher text.
    #[must_use]
    pub fn cypher(&self) -> &str {
        &self.cypher
    }

    /// The named parameters, in the order they were given.
    ///
    /// Names are `&'static str` because a Cypher parameter name is part of the
    /// query text: it exists in source wherever the statement does, so nothing is
    /// bought by letting one be assembled at run time and a hole is opened by it.
    #[must_use]
    pub fn parameters(&self) -> &[(&'static str, Value)] {
        &self.parameters
    }

    /// The parameters in the shape `Connection::execute` takes.
    fn bound(&self) -> Vec<(&str, Value)> {
        self.parameters
            .iter()
            .map(|(name, value)| (*name, value.clone()))
            .collect()
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
///
/// # Only `begin` mints one
///
/// There is no `GraphWriteSet::new` and no `Default`, and their absence is the
/// stamp's teeth rather than an omission — the same shape `SqliteBatch` takes,
/// and for the same reason: a write set a caller could build by hand carries no
/// store's identity, so [`CommitError::ForeignBatch`] would have to be reported
/// for it or waived for everything. Fill one through [`push`](Self::push) and
/// hand it back to `commit` or `reset`.
#[derive(Debug, Clone)]
pub struct GraphWriteSet {
    statements: Vec<GraphStatement>,
    /// The identity of the store that began this write set.
    stamp: u64,
}

impl GraphWriteSet {
    /// An empty write set stamped with `stamp`.
    ///
    /// Private, and that is the whole of the foreign-batch defence: `begin` is
    /// the only caller.
    fn stamped(stamp: u64) -> Self {
        Self {
            statements: Vec::new(),
            stamp,
        }
    }

    /// Queues a statement to run when the write set commits.
    ///
    /// # Security
    ///
    /// The Cypher text is fixed and the values are **bound**, never
    /// interpolated. A statement assembled out of event data at run time is a
    /// Cypher injection whose source is the log, and it commits inside the same
    /// transaction that advances the checkpoint — so the projection never
    /// replays those events and nothing re-derives the rows it corrupted.
    ///
    /// That has to be a type obligation rather than a paragraph, because nothing
    /// in the gate reads prose (`standards/rust/70-rustdoc-obligations.md`,
    /// RS-70-5). The interpolated spelling does not compile:
    ///
    /// ```compile_fail,E0308
    /// use happenstance_ladybug::GraphWriteSet;
    /// use happenstance_ladybug::lbug::Value;
    ///
    /// fn queue(writes: &mut GraphWriteSet, label: &str) {
    ///     writes.push(
    ///         format!("MATCH (n:{label}) DELETE n"),
    ///         core::iter::empty::<(&'static str, Value)>(),
    ///     );
    /// }
    /// ```
    ///
    /// **This is the seam ADR-0025 §2 says was missing, put where that record
    /// says to put it.** `happenstance-sqlite`'s `push` takes a `&'static str`
    /// and its `push_raw_sql` is the separately-named hatch; this crate's
    /// `GraphStatement::new` used to take `impl Into<String>` and close nothing,
    /// so the asymmetry was inherited rather than chosen. It is mirrored on the
    /// **push**, not on the constructor — `GraphStatement` now has no public
    /// constructor at all.
    pub fn push(
        &mut self,
        cypher: &'static str,
        parameters: impl IntoIterator<Item = (&'static str, Value)>,
    ) {
        self.push_raw_cypher(cypher, parameters);
    }

    /// Queues a statement this crate cannot see the provenance of.
    ///
    /// The unconstrained twin of [`push`](Self::push), for the one case its
    /// `&'static str` cannot express: a statement whose *shape* depends on a
    /// run-time value, of which the honest example is an `IN […]` list sized by
    /// how many keys are being written. Build the shape, bind the values.
    ///
    /// # Security
    ///
    /// The obligation [`push`](Self::push) discharges in the type system moves to
    /// the caller here, in full, and this method's name is the whole of the
    /// warning: **no value may be interpolated into `cypher`**. The wrong
    /// implementation it refuses to hide is the one a reviewer waves through
    /// because it reads like the parameterised form —
    /// `push_raw_cypher(format!("… {{k: '{key}'}} …"), [])`, with `key` decoded
    /// out of an event payload. It runs in the same transaction that advances the
    /// checkpoint, so a successful injection is recorded as progress and no later
    /// run re-derives the corrupted rows.
    pub fn push_raw_cypher(
        &mut self,
        cypher: impl Into<String>,
        parameters: impl IntoIterator<Item = (&'static str, Value)>,
    ) {
        self.statements.push(GraphStatement {
            cypher: cypher.into().into_boxed_str(),
            parameters: parameters.into_iter().collect(),
        });
    }

    /// The buffered statements, in the order they will be replayed.
    #[must_use]
    pub fn statements(&self) -> &[GraphStatement] {
        &self.statements
    }

    /// The number of buffered statements.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statements.len()
    }

    /// Whether the write set is empty.
    ///
    /// An empty write set is still worth committing: the checkpoint must
    /// advance past events that produced no graph mutation, or a restart
    /// replays them forever.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}

/// How the LadybugDB projection store fails.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum LadybugProjectionStoreError {
    /// The driver rejected a statement, or the C++ side threw.
    #[error("the LadybugDB driver failed")]
    Driver(#[from] lbug::Error),

    /// `COMMIT` itself failed, after every statement in the write set had been
    /// accepted.
    ///
    /// Distinct from [`Driver`](Self::Driver) because it is the one failure
    /// where the caller learns nothing about *which* write was at fault, and
    /// the only correct response is to rebuild the batch and retry rather than
    /// to fix a statement.
    #[error("committing the write set failed")]
    Commit(#[source] lbug::Error),

    /// A checkpoint node was stored but holds a value that is not a position.
    ///
    /// **Narrowed by measurement, and it means exactly one thing: a stored
    /// zero.** The property is `UINT64`, which round-trips `u64::MAX - 1`
    /// exactly, so there is no *upper* end to this narrowing and no negative end
    /// either — only [`SequencePosition`]'s `NonZeroU64` refusing the one value
    /// `UINT64` can hold and a position cannot. That is a corrupt checkpoint
    /// rather than a missing one, and collapsing it into
    /// [`Checkpoint::NeverRun`] would silently replay a projection from event 1.
    ///
    /// This variant's twin, `PositionOutOfRange`, is **gone** (ADR-0025 §1). It
    /// described a position too large for an `INT64` property, which is a gap
    /// this engine does not have: no code path could construct it, and a variant
    /// no implementation can reach is decorative by this repository's own
    /// corollary about rules, applied to an error enum.
    #[error(
        "checkpoint for projection `{projection}` holds {value}, which is not a valid position"
    )]
    MalformedCheckpoint {
        /// The projection whose checkpoint is unusable.
        projection: String,
        /// The value found in the checkpoint property. Zero, and only zero — see
        /// the variant's documentation for why there is no other value it can be.
        value: u64,
    },

    /// A checkpoint node was stored with an authority discriminant this build
    /// does not know.
    ///
    /// Reported rather than resolved to [`Authority::Live`], for
    /// `happenstance-sqlite`'s reason: resolving it tells a reader that a read
    /// model whose state this build cannot name is authoritative.
    #[error("checkpoint authority {0:?} is not one this build recognises")]
    MalformedAuthority(String),

    /// A queried node carried properties of a shape this schema does not
    /// declare.
    ///
    /// Distinct from [`MalformedCheckpoint`](Self::MalformedCheckpoint), which is
    /// about a *value* a `UINT64` can hold and a position cannot. This is about a
    /// row whose columns did not arrive as [`LadybugProjectionStore::open`]
    /// declares them — a missing column, a `NULL`, a string where a `UINT64` was
    /// created. Unreachable through this adapter's own DDL, and reported rather
    /// than `unwrap`ped because the database is a directory an operator can edit.
    #[error("the query `{query}` returned a row this schema cannot decode")]
    UnreadableRow {
        /// The query whose result could not be decoded.
        query: &'static str,
    },

    /// A write set begun on a different store instance reached
    /// [`rollback`](happenstance_core::ProjectionStore::rollback).
    ///
    /// `commit` and `reset` report the same refusal through their own port-level
    /// variants; `rollback` returns the adapter's error, so the refusal needs a
    /// variant of its own here. Discarding it silently was the alternative and it
    /// loses for the reason the port rejects a foreign batch at all: a caller
    /// holding a write set from the wrong store has a bug, and the cheapest place
    /// to learn it is the call that was supposed to clean up.
    #[error("the write set was begun on a different store instance")]
    ForeignBatch,

    /// Another write transaction is already open on this database.
    ///
    /// LadybugDB permits many concurrent readers and exactly one writer
    /// (`docs.ladybugdb.com/cypher/transaction/`). Two commits racing is
    /// therefore routine rather than exceptional, and the caller's correct
    /// response is to retry — so it gets its own variant rather than arriving
    /// as an opaque [`Driver`](Self::Driver) string that every caller would
    /// have to pattern-match on.
    ///
    /// It is recognised by the driver's message rather than by a code, because
    /// `lbug::Error::FailedQuery` carries a `String` and nothing else. The needle
    /// is [`WRITE_CONFLICT_NEEDLE`], and the classification is deliberately
    /// **narrowing-only**: an unrecognised message stays a
    /// [`Driver`](Self::Driver), so a change in the engine's wording costs a
    /// caller its retry hint and never a wrong answer.
    #[error("another write transaction is already open on this database")]
    WriteTransactionInUse,
}

/// The substring that identifies LadybugDB's refusal of a second concurrent
/// writer, lowercased before comparison.
///
/// A message match is a weak instrument and it is used in the one direction
/// where being wrong is cheap: it can only turn a
/// [`Driver`](LadybugProjectionStoreError::Driver) into a
/// [`WriteTransactionInUse`](LadybugProjectionStoreError::WriteTransactionInUse),
/// which is a strictly more specific answer to the same question. A missed match
/// costs a caller a retry hint; there is no direction in which it can invent one.
const WRITE_CONFLICT_NEEDLE: &str = "write transaction";

/// The driver's error, narrowed where this adapter recognises it.
fn classify(error: lbug::Error) -> LadybugProjectionStoreError {
    if error
        .to_string()
        .to_ascii_lowercase()
        .contains(WRITE_CONFLICT_NEEDLE)
    {
        return LadybugProjectionStoreError::WriteTransactionInUse;
    }
    LadybugProjectionStoreError::Driver(error)
}

/// What a commit that reached the database did.
///
/// A commit has two port-level outcomes and one adapter-level one, and only the
/// adapter's can be reported as [`LadybugProjectionStoreError`]. This is how the
/// synchronous body — which knows nothing about [`CommitError`] — hands the
/// regression back to the method that can spell it.
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

/// A LadybugDB-backed projection store whose batch is a deferred write set.
#[derive(Debug, Clone)]
pub struct LadybugProjectionStore {
    database: Arc<Database>,
    /// This instance's identity, compared against the one [`GraphWriteSet`]
    /// carries.
    ///
    /// Copied by `Clone` rather than re-minted: a clone shares the `Arc` and the
    /// directory, so it *is* the same store and must accept the write sets its
    /// origin began.
    stamp: u64,
}

impl LadybugProjectionStore {
    /// Wraps an already-open database whose schema has already been applied.
    ///
    /// **`Arc<Database>` rather than `Database`, and it is forced by measurement
    /// rather than chosen** (ADR-0025 §6). A second `Database::new` on the same
    /// directory is refused by a file lock:
    ///
    /// ```text
    /// IO exception: Could not set lock on file : …\lbug-probe-p0 (Error: 33)
    /// ```
    ///
    /// So a second handle onto one backing store cannot be a second `Database`;
    /// it has to be a second [`Connection`] over one shared `Arc<Database>`, and
    /// whatever owns the database therefore has to be shareable. Use
    /// [`open`](Self::open) to have the schema applied.
    #[must_use]
    pub fn new(database: Arc<Database>) -> Self {
        Self {
            database,
            stamp: mint_stamp(),
        }
    }

    /// Opens (creating if absent) a database at `path` and applies the checkpoint
    /// schema.
    ///
    /// The DDL is idempotent — `CREATE NODE TABLE IF NOT EXISTS` — because a
    /// fixture opens one directory more than once and because a process opening
    /// an existing store must not have to know whether it is the first.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if LadybugDB refuses the
    /// directory — which **includes the case that another `Database` in this or
    /// any process already holds the lock on it**, the measured behaviour §6
    /// rests on — or if the schema cannot be applied.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, LadybugProjectionStoreError> {
        Self::open_with_config(path, SystemConfig::default())
    }

    /// [`open`](Self::open), with the engine configured by the caller.
    ///
    /// `SystemConfig` is LadybugDB's, not this adapter's, and the two settings a
    /// caller most often has an opinion about are the ones this adapter has no
    /// business choosing: `buffer_pool_size` and `max_db_size`. The default
    /// `max_db_size` is `u32::MAX` — 4 GiB of reserved address space **per open
    /// database** — which is invisible to an application that opens one and very
    /// visible to a test binary that opens forty.
    ///
    /// # Errors
    ///
    /// As [`open`](Self::open).
    pub fn open_with_config(
        path: impl AsRef<std::path::Path>,
        config: SystemConfig,
    ) -> Result<Self, LadybugProjectionStoreError> {
        let database = Database::new(path, config)?;
        let store = Self::new(Arc::new(database));
        store.apply_schema()?;
        Ok(store)
    }

    /// A second handle onto the same backing store.
    ///
    /// This is what `ProjectionFixture::SECOND_HANDLE` is answered with, and it
    /// is deliberately **not** [`Clone`]: a clone shares this instance's stamp
    /// and is therefore the *same* store, where a second handle is a different
    /// store instance over one graph. The distinction is what
    /// `commit_rejects_a_foreign_batch` is about, and it is the reason both
    /// methods exist.
    #[must_use]
    pub fn second_handle(&self) -> Self {
        Self::new(Arc::clone(&self.database))
    }

    /// The shared database this store writes into.
    #[must_use]
    pub fn database(&self) -> &Arc<Database> {
        &self.database
    }

    /// Applies the checkpoint schema, idempotently.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if a connection cannot be
    /// opened or the DDL is refused.
    fn apply_schema(&self) -> Result<(), LadybugProjectionStoreError> {
        let connection = self.connect()?;
        connection.query(CHECKPOINT_TABLE)?;
        #[cfg(feature = "conformance")]
        connection.query(PROBE_TABLE)?;
        Ok(())
    }

    /// Opens a connection for one operation.
    ///
    /// Per-call rather than pooled, which is LadybugDB's documented pattern and
    /// the only one available to a store that cannot hold a `Connection` in a
    /// field: `Connection::new` takes `&'db Database`, so a struct holding both
    /// is self-referential.
    ///
    /// # Errors
    ///
    /// Returns [`LadybugProjectionStoreError::Driver`] if the driver cannot
    /// open a connection.
    pub fn connect(&self) -> Result<Connection<'_>, LadybugProjectionStoreError> {
        Ok(Connection::new(&self.database)?)
    }

    /// Runs one parameterised statement on `connection`.
    fn run(
        connection: &Connection<'_>,
        statement: &GraphStatement,
    ) -> Result<(), LadybugProjectionStoreError> {
        let mut prepared = connection.prepare(statement.cypher()).map_err(classify)?;
        connection
            .execute(&mut prepared, statement.bound())
            .map_err(classify)?;
        Ok(())
    }

    /// Reads `id`'s checkpoint through `connection`.
    ///
    /// Shared by [`checkpoint`](happenstance_core::ProjectionStore::checkpoint)
    /// and by the regression guard inside `commit`, so that the two cannot
    /// disagree about what a stored node means.
    fn read_checkpoint(
        connection: &Connection<'_>,
        id: &ProjectionId,
    ) -> Result<Checkpoint, LadybugProjectionStoreError> {
        let mut prepared = connection.prepare(SELECT_CHECKPOINT).map_err(classify)?;
        let mut result = connection
            .execute(
                &mut prepared,
                vec![("projection", Value::String(id.as_str().to_owned()))],
            )
            .map_err(classify)?;

        // The absence of a node *is* `NeverRun`, which is what makes `reset`'s
        // delete correct and a sentinel row wrong.
        let Some(row) = result.next() else {
            return Ok(Checkpoint::NeverRun);
        };

        let (Some(Value::UInt64(stored)), Some(Value::String(authority))) =
            (row.first(), row.get(1))
        else {
            return Err(LadybugProjectionStoreError::UnreadableRow {
                query: SELECT_CHECKPOINT,
            });
        };

        // Zero is the only value a `UINT64` holds and a `SequencePosition` does
        // not, so it is the whole of this conversion's failure domain — see
        // `MalformedCheckpoint`.
        let through = SequencePosition::new(*stored).ok_or_else(|| {
            LadybugProjectionStoreError::MalformedCheckpoint {
                projection: id.as_str().to_owned(),
                value: *stored,
            }
        })?;

        match authority.as_str() {
            AUTHORITY_LIVE => Ok(Checkpoint::Live { through }),
            AUTHORITY_REBUILDING => Ok(Checkpoint::Rebuilding { through }),
            _ => Err(LadybugProjectionStoreError::MalformedAuthority(
                authority.clone(),
            )),
        }
    }

    /// One transaction: the regression guard, the caller's statements, the
    /// checkpoint.
    fn commit_inner(
        &self,
        batch: &GraphWriteSet,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<CommitOutcome, LadybugProjectionStoreError> {
        let connection = self.connect()?;
        connection.query(BEGIN).map_err(classify)?;

        // Inside the transaction, not before it. A guard evaluated before `BEGIN`
        // lets two commits interleave between the read and the write and both
        // pass.
        //
        // Not a bare `?`, and the reason is a case ADR-0025 §7 does not reach.
        // §7 measured that LadybugDB aborts the whole transaction ITSELF on a
        // statement error, so the error path must not issue a `ROLLBACK` — a
        // second error would mask the first. True, and it is about *statement*
        // errors. `read_checkpoint` has three failure modes in which the query
        // SUCCEEDED and the decode did not: `UnreadableRow`,
        // `MalformedCheckpoint` and `MalformedAuthority`. No statement failed,
        // so the engine has aborted nothing, and a bare `?` here returns with
        // the transaction still open — holding a write transaction on the
        // database for as long as the connection lives, which `:483`'s
        // "another write transaction is already open on this database" is the
        // cost of.
        //
        // The rollback is issued best-effort and its result deliberately
        // discarded. That satisfies both halves at once: if the engine did
        // abort — a statement error reaching here through the same `?` — the
        // `ROLLBACK` is refused and the refusal is dropped, so the first error
        // still reaches the caller, which is exactly what §7 is protecting. If
        // it did not abort, the transaction is closed. Discarding is what makes
        // the two cases safe to handle with one line.
        let checkpoint = match Self::read_checkpoint(&connection, id) {
            Ok(checkpoint) => checkpoint,
            Err(error) => {
                let _ = connection.query(ROLLBACK);
                return Err(error);
            }
        };
        let recorded = recorded_position(&checkpoint);
        if let Some(current) = recorded
            && position < current
        {
            // No statement has *failed* here, so the transaction is still open
            // and a `ROLLBACK` is both legal and necessary. That is the exact
            // distinction ADR-0025 §7 draws: the rollback that must not be
            // issued is the one after an error, and this is not one.
            connection.query(ROLLBACK).map_err(classify)?;
            return Ok(CommitOutcome::Regressed {
                current,
                attempted: position,
            });
        }

        for statement in batch.statements() {
            Self::run(&connection, statement)?;
        }

        Self::run(
            &connection,
            &GraphStatement {
                cypher: UPSERT_CHECKPOINT.into(),
                parameters: vec![
                    ("projection", Value::String(id.as_str().to_owned())),
                    ("position", Value::UInt64(position.get())),
                    (
                        "authority",
                        Value::String(stored_authority(authority).to_owned()),
                    ),
                ],
            },
        )?;

        connection
            .query(COMMIT)
            .map_err(LadybugProjectionStoreError::Commit)?;
        Ok(CommitOutcome::Committed)
    }

    /// One transaction: the caller's deletes, then the checkpoint node's removal.
    fn reset_inner(
        &self,
        batch: &GraphWriteSet,
        id: &ProjectionId,
    ) -> Result<(), LadybugProjectionStoreError> {
        let connection = self.connect()?;
        connection.query(BEGIN).map_err(classify)?;

        for statement in batch.statements() {
            Self::run(&connection, statement)?;
        }

        Self::run(
            &connection,
            &GraphStatement {
                cypher: DELETE_CHECKPOINT.into(),
                parameters: vec![("projection", Value::String(id.as_str().to_owned()))],
            },
        )?;

        connection
            .query(COMMIT)
            .map_err(LadybugProjectionStoreError::Commit)?;
        Ok(())
    }
}

// The `Send` flavour, and on evidence rather than convenience: `lbug` writes
// `unsafe impl Send` and `unsafe impl Sync` for both `Database` and
// `Connection`, so `Self: Send` and `&Self: Send` both hold and the derived
// futures can carry `+ Send`. Had the bindings been `!Send` — which cxx-backed
// bindings often are — the bare `ProjectionStore` would have been the only
// honest choice, and *that* would have been the finding.
//
// `tests/send_and_sync.rs` re-checks it against the real types. That file is the
// stand-in module's four assertions, re-pointed rather than deleted with it: they
// reach the same conclusion by a different mechanism, because `unsafe_code` is
// `forbid` in this workspace and forced the stand-in to get `Send + Sync` by
// construction where `lbug` gets it by `unsafe impl` over a C++ pointer.
impl SendProjectionStore for LadybugProjectionStore {
    type Error = LadybugProjectionStoreError;

    // The port used to declare `type Batch<'a> where Self: 'a`, and this
    // adapter bound an *owned* type to it so that dropping the GAT would be a
    // deletion here rather than a redesign. It was: the batch type is the same
    // `GraphWriteSet`, and what left is the lifetime and its `where` clause.
    //
    // ADR-0025 §8 pre-registered the `E0195` spelling trap firing for a third
    // implementer as one of four "it did not hold" conditions. It did not fire:
    // `Self::Batch` names an owned type with no lifetime to mismatch, exactly as
    // it does in `happenstance-sqlite` and `happenstance-postgres`.
    type Batch = GraphWriteSet;

    /// A fresh, empty write set stamped with this store's identity.
    ///
    /// Neither `async` nor fallible, which is what this adapter wanted: opening
    /// the write set allocates a `Vec` and nothing more. It is also the only
    /// mint — there is no public constructor that could produce one without a
    /// stamp.
    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        Ok(GraphWriteSet::stamped(self.stamp))
    }

    /// How far `id` has been brought, and whether its nodes are authoritative.
    ///
    /// # Errors
    ///
    /// * [`LadybugProjectionStoreError::Driver`] if a connection or the query
    ///   fails;
    /// * [`LadybugProjectionStoreError::MalformedCheckpoint`] if the stored
    ///   position is zero, and
    ///   [`MalformedAuthority`](LadybugProjectionStoreError::MalformedAuthority)
    ///   if the stored authority is one this build does not know — both reported
    ///   rather than resolved to a default;
    /// * [`LadybugProjectionStoreError::UnreadableRow`] if the node's properties
    ///   are not the shape this schema declares.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        let connection = self.connect()?;
        Self::read_checkpoint(&connection, id)
    }

    /// Applies `batch` and moves `id`'s checkpoint to `position`, as one unit.
    ///
    /// One `BEGIN TRANSACTION` … `COMMIT` carries all of it: the recorded
    /// checkpoint is read *inside* the transaction that will overwrite it, then
    /// every queued statement runs in the order it was pushed, then the
    /// checkpoint node is merged with the authority this commit claims.
    ///
    /// # No `ROLLBACK` on the error path, and it is measured
    ///
    /// ADR-0025 §7. On a statement error inside a transaction **LadybugDB aborts
    /// the whole transaction itself**: the read-model write made earlier in the
    /// same transaction was already gone before any rollback was attempted, and
    /// the subsequent `ROLLBACK` was *refused*, because there was no longer a
    /// transaction to roll back. So this error path reports the first error and
    /// does not try to clean up after it — issuing a `ROLLBACK` there produces a
    /// second error that masks the first.
    ///
    /// The one `ROLLBACK` this body does issue is on the **regression** path,
    /// where no statement has failed and the transaction is still open. That is
    /// the distinction, and it is why §7 is about the error path specifically.
    ///
    /// It is also good news for PS-1: atomicity is enforced by the engine rather
    /// than by this adapter remembering to ask for it.
    ///
    /// # Errors
    ///
    /// * [`CommitError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance. Decided before a connection is opened, so a rejected commit
    ///   cannot have moved anything.
    /// * [`CommitError::CheckpointRegression`] if `position` is strictly below
    ///   the recorded checkpoint. An equal position is accepted, which PS-22
    ///   permits.
    /// * [`CommitError::Store`] if the driver failed, the recorded node could not
    ///   be read back, or another write transaction was already open.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if batch.stamp != self.stamp {
            // Before any Cypher, which is what makes "neither store moved"
            // structural rather than something to remember.
            return Err(CommitError::ForeignBatch);
        }

        match self
            .commit_inner(&batch, id, position, authority)
            .map_err(CommitError::Store)?
        {
            CommitOutcome::Committed => Ok(()),
            CommitOutcome::Regressed { current, attempted } => {
                Err(CommitError::CheckpointRegression { current, attempted })
            }
        }
    }

    /// Applies `batch` and returns `id` to [`Checkpoint::NeverRun`], as one unit.
    ///
    /// `commit`'s dual, in one transaction: the caller's own deletes, then the
    /// removal of this projection's checkpoint node — removal rather than a
    /// sentinel position, because the absence of a node *is* `NeverRun` and a
    /// sentinel would make a reset indistinguishable from a commit at the first
    /// position.
    ///
    /// It deletes nothing this adapter chose. The read model is the caller's, and
    /// an adapter that emptied a node table of its own naming would be scoped to
    /// something the port deliberately never told it.
    ///
    /// # Errors
    ///
    /// * [`ResetError::ForeignBatch`] if `batch` was begun on a different store
    ///   instance, decided before a connection is opened.
    /// * [`ResetError::Store`] if the driver failed.
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
        self.reset_inner(&batch, id).map_err(ResetError::Store)
    }

    /// Discards the write set without committing.
    ///
    /// Nothing was ever sent to LadybugDB, so this is a drop — which is also the
    /// evidence PS-7 asks for: for a buffering adapter, "dropping a batch must
    /// roll back" is free, and the store is usable afterwards because nothing was
    /// ever checked out to return.
    ///
    /// # Errors
    ///
    /// [`LadybugProjectionStoreError::ForeignBatch`] if `batch` was begun on a
    /// different store instance. The write set is consumed either way; the
    /// refusal is how a caller learns it was holding the wrong one, on the call
    /// that was meant to be the cleanup.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        if batch.stamp != self.stamp {
            return Err(LadybugProjectionStoreError::ForeignBatch);
        }
        drop(batch);
        Ok(())
    }
}

/// The conformance suite's write and read seam.
///
/// It lives here, in `src/`, and not in `tests/projection.rs`: that is a
/// different crate, where neither [`ProjectionProbe`] nor
/// [`LadybugProjectionStore`] is local and the orphan rule answers
/// `error[E0117]`. The feature that turns it on forwards one flag on a dependency
/// this crate already has.
///
/// [`ProjectionProbe`]: happenstance_core::ProjectionProbe
#[cfg(feature = "conformance")]
impl happenstance_core::ProjectionProbe for LadybugProjectionStore {
    /// `false`, and it is the honest answer rather than a gap (ADR-0025 §5).
    ///
    /// A [`GraphWriteSet`] is a list of statements that has been sent to the
    /// engine exactly never, so there is no open transaction to read through —
    /// and answering from *committed* state is what PS-12 forbids by name,
    /// because a projection doing `get` then `set` inside one write set would
    /// then read the value from before the set began and lose every increment
    /// after the first.
    ///
    /// Note what this does **not** say. The engine's transactions do give
    /// read-your-own-writes — measured, and it is the whole of PS-4's Cypher-level
    /// answer — so `true` is unavailable here for a reason about *when this
    /// adapter sends*, not about what LadybugDB can do.
    const READS_THROUGH_BATCH: bool = false;

    /// Queues one probe node. Never awaits and never fails, because queueing Cypher
    /// into a buffer the caller owns cannot fail.
    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        batch.push_raw_cypher(
            "MERGE (p:__hs_probe {k: $k}) SET p.v = $v",
            [
                ("k", Value::String(key.to_owned())),
                // No conversion at all, which is the `UINT64` decision's dividend
                // in the one place a reader can see it: `happenstance-sqlite`
                // spells this `Value::Integer(value.cast_signed())`.
                ("v", Value::UInt64(value)),
            ],
        );
        Ok(())
    }

    /// Queues removal of every probe node, so that
    /// [`reset`](happenstance_core::ProjectionStore::reset) can be checked
    /// without the suite knowing what a read model is.
    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        batch.push("MATCH (p:__hs_probe) DELETE p", []);
        Ok(())
    }

    /// Reads one probe node from **committed** state.
    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        const QUERY: &str = "MATCH (p:__hs_probe {k: $k}) RETURN p.v";

        let connection = self.connect()?;
        let mut prepared = connection.prepare(QUERY).map_err(classify)?;
        let mut result = connection
            .execute(&mut prepared, vec![("k", Value::String(key.to_owned()))])
            .map_err(classify)?;

        match result.next() {
            None => Ok(None),
            Some(row) => match row.first() {
                Some(Value::UInt64(value)) => Ok(Some(*value)),
                _ => Err(LadybugProjectionStoreError::UnreadableRow { query: QUERY }),
            },
        }
    }

    /// Never called, because
    /// [`READS_THROUGH_BATCH`](happenstance_core::ProjectionProbe::READS_THROUGH_BATCH)
    /// is `false`.
    ///
    /// # Panics
    ///
    /// Always. The specification permits exactly this for an adapter whose batch
    /// has no read path, and the panic is the honest answer: any value returned
    /// here would be a claim about pending writes LadybugDB has never been told
    /// about.
    async fn probe_read_through(
        &self,
        _batch: &mut Self::Batch,
        _key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        unimplemented!(
            "GraphWriteSet buffers its statements, so `READS_THROUGH_BATCH` is \
             `false` and this is never called: there is no open transaction to \
             read through, and answering from committed state is what PS-12 \
             forbids"
        )
    }
}

/// The position a checkpoint records, if it records one.
///
/// Separate from [`LadybugProjectionStore::read_checkpoint`] because the
/// regression guard is about the position alone: a commit is refused for going
/// backwards whatever the recorded authority claimed. The wildcard arm is not
/// laziness — [`Checkpoint`] is `#[non_exhaustive]`, and a state this build has
/// never been told about must not be read as "committed at some position".
fn recorded_position(checkpoint: &Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(*through),
        _ => None,
    }
}

/// An [`Authority`] as the discriminant this schema stores.
///
/// [`Authority`] is `#[non_exhaustive]`, so a build compiled against a later
/// contract can meet a variant this arm list has never been told about. It is
/// stored as `rebuilding` — *not authoritative* — because that is the direction a
/// reader can recover from: treating an unknown claim as `Live` tells a dashboard
/// that nodes nobody can vouch for are the truth.
fn stored_authority(authority: Authority) -> &'static str {
    match authority {
        Authority::Live => AUTHORITY_LIVE,
        _ => AUTHORITY_REBUILDING,
    }
}
