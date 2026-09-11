//! The **second batch shape**: a projection store whose batch is a replayable
//! op journal and whose `commit` is the only moment anything reaches the read
//! model.
//!
//! CF-5's second projection conformant variant, and the far end of §6's
//! batch-shape axis (`spec/SPECIFICATION.md:5688-5693`). It is not a defect and
//! it MUST pass every projection rule: a rule that rejects it is a finding about
//! the **rule** (CF-6), never about this store.
//!
//! # The one axis it differs on, and why that difference is legal
//!
//! `variants.rs`'s doctrine — one named axis per variant, because a store that
//! differs on two makes a rule failure unattributable — applies here in full.
//! The axis is **what a batch is**.
//!
//! * The reference store's batch is a *materialised delta*: a
//!   `BTreeMap<String, u64>` plus a `clear_all` flag, collapsed at stage time so
//!   that two writes to one key leave one entry
//!   (`crates/happenstance-core/src/projection_memory.rs:209-250`).
//! * This store's batch is an *ordered journal*: [`Op::Write`] and
//!   [`Op::DeleteAll`] appended in the order the caller made them, collapsed by
//!   nothing, replayed in order inside `commit`. It is the statement list a
//!   Workers `SqlStorage` or Neon-over-one-shot-HTTP adapter queues for a server
//!   it holds no connection to (`references/adapter-shapes.md`), and the reading
//!   of PS-4 that gives the port the least: *"PS-1 is satisfiable by opening the
//!   transaction inside `commit` around a buffered write set"*
//!   (`spec/SPECIFICATION.md:4849-4856`).
//!
//! PS-4 permits it outright, and PS-5 is what makes it expressible: `Batch` is
//! `type Batch;`, an owned value with no lifetime, so the journal is a field
//! rather than a borrow of the store. Nothing is acquired at `begin` — no
//! handle, no transaction, no lock, no borrow of the committed state — which is
//! the property `commit_is_atomic_with_the_read_model` is *shape-blind* to and
//! PS-4's `Rule:` says so about itself.
//!
//! # What it deliberately does **not** differ on
//!
//! [`READS_THROUGH_BATCH`](ProjectionProbe::READS_THROUGH_BATCH) is `true`. A
//! journal is trivially readable — fold it backwards over committed state — so
//! `true` is the honest answer for this shape, and it is also the load-bearing
//! one: with `false`, `batch_reads_reflect_pending_writes` and
//! `rebuild_is_chunk_size_invariant` would skip against *every* conformant
//! variant this family has, and CF-5's positive control would be a control over
//! part of the suite. The `false` instrument already exists and is
//! [`NoBatchReadStore`](crate::variants::NoBatchReadStore); a second one here
//! would buy a skip-shaped hole and nothing else.
//!
//! Every fixture capability is **supported**, for the reason
//! [`MutantFixture`](crate::correct::MutantFixture) supports them: this is an
//! instrument rather than a shipped adapter, and a fixture that declined
//! `COMMIT_FAULT` or `RESET_REFUSAL` would make the two rules gated on them skip
//! against every variant — which is the same hole one level down.
//!
//! # What a green run against this store does *not* prove
//!
//! Not PS-2. PS-2 is `[FROZEN]` and asks for two **adapters** at opposite ends
//! of the axis, and its `Rejects:` clause names the two-instrument monoculture
//! verbatim (`spec/SPECIFICATION.md:4760-4775`). This is a testkit instrument in
//! process memory. What it proves is the narrower and still worth having claim
//! that a projection rule accepted a store built the other way round.
//!
//! Two consumers, one definition: `tests/projection_mutation_coverage.rs`
//! registers it and drives it through the mutant harness, and
//! `tests/projection_conformance_buffering.rs` drives it through
//! `projection_store_conformance!`. Both reach it by `#[path]`, so the registry
//! describes the same code the harness ran.

use core::cell::{Cell, RefCell};
use core::fmt;
use core::future::Future;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};
use happenstance_testkit::{Capability, ProjectionFixture};

// =====================================================================
// The journal
// =====================================================================

/// One queued statement.
///
/// Deliberately *not* a public "write set" type, and deliberately not shared
/// with the reference store: a shared representation would make the two batch
/// shapes structurally alike again, which is the one property this store exists
/// to avoid.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    /// Set `key` to `value`. Two of these for one key are two entries, not one.
    Write {
        /// The read-model key.
        key: String,
        /// The value to set it to.
        value: u64,
    },
    /// Remove every row. What a caller stages before handing the batch to
    /// `reset`, and an ordinary op rather than a flag — so writes *after* it
    /// survive and writes *before* it do not, by replay order alone.
    DeleteAll,
}

/// Replays `journal` onto `rows`, in order.
///
/// The whole of what `commit` does to the read model. Order is the semantics: a
/// collapsed delta would have had to decide last-write-wins at stage time, and
/// this store never decides it at all.
fn replay(rows: &mut BTreeMap<String, u64>, journal: &[Op]) {
    for op in journal {
        match op {
            Op::Write { key, value } => {
                rows.insert(key.clone(), *value);
            }
            Op::DeleteAll => rows.clear(),
        }
    }
}

/// What an open journal has to say about one key.
///
/// Three states rather than an `Option<Option<u64>>`, because the third is a
/// different kind of answer from the other two: [`Silent`](Self::Silent) means
/// *ask committed state*, and collapsing it into "absent" is precisely the bug
/// `batch_reads_reflect_pending_writes` and its mirror exist to catch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Staged {
    /// The journal's last word for this key: this value.
    Value(u64),
    /// The journal's last word: nothing, because a queued `DeleteAll` covers it.
    Cleared,
    /// The journal says nothing about this key; committed state answers.
    Silent,
}

/// What `journal` stages for `key`.
///
/// A backwards scan, which is the read a queued-statement adapter can answer
/// without a server: the last `Write` for the key wins, and a `DeleteAll`
/// reached first means the journal itself answers "absent". Falling off the front
/// is [`Staged::Silent`].
fn staged(journal: &[Op], key: &str) -> Staged {
    for op in journal.iter().rev() {
        match op {
            Op::Write {
                key: written,
                value,
            } if written == key => return Staged::Value(*value),
            Op::DeleteAll => return Staged::Cleared,
            Op::Write { .. } => {}
        }
    }
    Staged::Silent
}

// =====================================================================
// The batch, the store and its error
// =====================================================================

/// An in-flight write against a [`BufferingProjectionStore`].
///
/// Owned, holding **nothing** of the store's: no handle, no transaction, no
/// lock, no borrow. Dropping it is the rollback, and it costs the store nothing
/// — which is why PS-7 holds here for a different reason than it does in the
/// reference store, and why a pooled-connection store answering `Busy` forever
/// is a defect this pair can tell apart from a correct one.
#[derive(Debug)]
pub(crate) struct BufferingBatch {
    /// The identity of the store instance that minted this batch (PS-15).
    ///
    /// A field rather than a lifetime: with `type Batch;` there is no region to
    /// name, and a lifetime names a region rather than an instance anyway.
    stamp: u64,
    /// The queued statements, in the order they were made.
    journal: Vec<Op>,
}

/// How this store fails for reasons of its own.
///
/// One inhabited variant, and it is reachable only after
/// [`ProjectionFixture::arm_commit_fault`] — the fixture declares `COMMIT_FAULT`
/// supported, so a store that could not produce a failing commit would be lying
/// in its capability declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BufferingProjectionStoreError {
    /// The armed fault fired while the journal was being sent.
    ///
    /// The failure a queued-statement adapter actually meets: the batch goes out
    /// as one request and the server refuses it whole. Nothing has been replayed
    /// when it does, which is why PS-1's second conjunct holds here without an
    /// undo path.
    CommitFault,
}

impl fmt::Display for BufferingProjectionStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommitFault => {
                f.write_str("the armed fault fired while this commit was replaying its journal")
            }
        }
    }
}

impl core::error::Error for BufferingProjectionStoreError {}

/// The committed read model and the checkpoints, behind **one** borrow.
///
/// One `RefCell` rather than two, for the reference store's reason: two cells
/// compile, pass every test that reads them separately, and reintroduce the
/// window PS-1 exists to close.
#[derive(Debug, Default)]
struct Committed {
    rows: BTreeMap<String, u64>,
    checkpoints: BTreeMap<String, Checkpoint>,
}

/// A fresh per-instance stamp.
///
/// Process-local and monotonic, which is all a foreign-batch check needs. It
/// starts at `1` so a store that forgot to call it — and left the field at its
/// `Default` — is distinguishable from one that did.
fn next_stamp() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// The position a checkpoint has been considered through, if any.
///
/// `NeverRun` has none, which is why a first commit at any position is accepted
/// and only a *regression* against a recorded position is refused (PS-22).
const fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
        // `NeverRun` and any variant added later: nothing has been considered.
        _ => None,
    }
}

/// A projection store that holds nothing between `begin` and `commit`.
///
/// One instance is one backing store; [`Clone`] is one more **handle** onto it,
/// which is what [`ProjectionFixture::connect`] hands out. The `Rc` is why
/// [`ProjectionFixture::Store`] can be an ordinary associated type rather than a
/// borrowing GAT — the handle owns a refcount instead of borrowing a lifetime
/// from the fixture, which is the difference between a compiling contract and
/// the rustc ICE this repository minimised
/// (`crates/happenstance-testkit/src/contract.rs:108-122`).
#[derive(Debug, Clone)]
pub(crate) struct BufferingProjectionStore {
    committed: Rc<RefCell<Committed>>,
    /// Whether the fixture has armed a fault for the next commit. Shared by
    /// every handle, because an adapter's fault is armed in its *store* rather
    /// than in one session's view of it. Consumed by the commit that fires it.
    fault: Rc<Cell<bool>>,
    /// The one projection this store's policy protects from `reset`, if any.
    /// Shared by every handle, and — unlike the fault — **not** one-shot: a
    /// policy that evaporated after refusing once would protect a regulatory
    /// ledger from the first operator and not from the second.
    protected: Rc<RefCell<Option<String>>>,
    stamp: u64,
}

impl BufferingProjectionStore {
    /// A store over a fresh, empty read model and no checkpoints.
    fn new() -> Self {
        Self {
            committed: Rc::new(RefCell::new(Committed::default())),
            fault: Rc::new(Cell::new(false)),
            protected: Rc::new(RefCell::new(None)),
            stamp: next_stamp(),
        }
    }
}

// The **bare** flavour, never `SendProjectionStore`, and only one of the two
// names is in scope in this file (CLAUDE.md binding constraint 4). No
// `#[async_trait]` appears, here or anywhere: it would inject `+ Send` and make
// the `wasm32` target this shape is modelled on impossible (ADR-0001).
impl ProjectionStore for BufferingProjectionStore {
    type Error = BufferingProjectionStoreError;

    type Batch = BufferingBatch;

    /// Allocates a journal. Acquires nothing.
    ///
    /// There is no connection to check out, no transaction to open and no lock
    /// to take — which is the whole shape. A store that had something to acquire
    /// here would be the other end of the axis.
    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        Ok(BufferingBatch {
            stamp: self.stamp,
            journal: Vec::new(),
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        Ok(self
            .committed
            .borrow()
            .checkpoints
            .get(id.as_str())
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

    /// Replays the journal and records the checkpoint as one unit.
    ///
    /// PS-1's coupling, reached by a different mechanism than the reference
    /// store's: there is no transaction to hold open, so the coupling comes from
    /// the single `RefCell` borrow that both writes happen inside and from
    /// nothing being replayed at all until every refusal has been decided.
    ///
    /// Order is the reference store's: identity, then the port-level refusals,
    /// then the fault, then the write. A store that checked the position before
    /// the stamp would answer the wrong error for a batch that is both foreign
    /// and regressing.
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

        let mut committed = self.committed.borrow_mut();

        let recorded = committed
            .checkpoints
            .get(id.as_str())
            .copied()
            .unwrap_or(Checkpoint::NeverRun);
        if let Some(current) = considered_through(recorded)
            && position < current
        {
            return Err(CommitError::CheckpointRegression {
                current,
                attempted: position,
            });
        }

        // No validation of `position` against the journal: PS-21 forbids it, and
        // a queued-statement adapter is exactly the one tempted to add it,
        // because it has the whole statement list in front of it.

        let checkpoint = match authority {
            Authority::Rebuilding => Checkpoint::Rebuilding { through: position },
            // `Live` and any variant added later: a commit that does not claim a
            // rebuild is claiming the rows are authoritative.
            _ => Checkpoint::Live { through: position },
        };

        // The armed fault fires here, after every port-level refusal and *before*
        // a single op is replayed — which is where a real one fires for this
        // shape: the request either lands whole or is refused whole. `replace`
        // consumes it, so it fires once.
        if self.fault.replace(false) {
            return Err(CommitError::Store(
                BufferingProjectionStoreError::CommitFault,
            ));
        }

        replay(&mut committed.rows, &batch.journal);
        committed
            .checkpoints
            .insert(id.as_str().to_owned(), checkpoint);
        Ok(())
    }

    /// Replays the caller's deletes and records this projection's checkpoint as
    /// an explicit [`Checkpoint::NeverRun`], as one unit.
    ///
    /// PS-16 and PS-17. It is neither a truncate nor `commit(empty, id, FIRST)`:
    /// only this id's checkpoint row is touched, and the variant it is left in is
    /// the one a runner resumes from *inclusive*.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        // Read the policy before the state is borrowed: they are two cells, and a
        // store that took both borrows at once would be modelling a lock this
        // port does not require.
        let protected = self.protected.borrow().as_deref() == Some(id.as_str());
        if protected {
            // PS-18's operative half: a refusal writes **nothing**. A store that
            // sent its deletes and consulted the policy afterwards reports the
            // refusal perfectly honestly over a read model that is already gone.
            return Err(ResetError::Refused);
        }

        let mut committed = self.committed.borrow_mut();
        replay(&mut committed.rows, &batch.journal);
        committed
            .checkpoints
            .insert(id.as_str().to_owned(), Checkpoint::NeverRun);
        Ok(())
    }

    /// Discards the journal.
    ///
    /// PS-8, and it costs nothing: there is no `ROLLBACK` to issue because
    /// nothing was ever sent. That it holds for a *different reason* than the
    /// reference store's is the point of registering this store at all.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        drop(batch);
        Ok(())
    }
}

impl ProjectionProbe for BufferingProjectionStore {
    /// `true`: a journal is readable by folding it backwards over committed
    /// state. See this module's documentation for why the honest answer is also
    /// the load-bearing one.
    const READS_THROUGH_BATCH: bool = true;

    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        // Appended, never collapsed. Nothing reaches `committed` on this path —
        // the assertion `the_read_model_changes_only_at_commit` makes directly,
        // and the one a green suite alone cannot make.
        batch.journal.push(Op::Write {
            key: key.to_owned(),
            value,
        });
        Ok(())
    }

    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        batch.journal.push(Op::DeleteAll);
        Ok(())
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        Ok(self.committed.borrow().rows.get(key).copied())
    }

    async fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        Ok(match staged(&batch.journal, key) {
            Staged::Value(value) => Some(value),
            Staged::Cleared => None,
            Staged::Silent => self.committed.borrow().rows.get(key).copied(),
        })
    }
}

// =====================================================================
// The fixture
// =====================================================================

/// One backing [`BufferingProjectionStore`], and any number of handles onto it.
///
/// `MemoryFixture`'s owned-handle pattern
/// (`crates/happenstance-testkit/src/fixtures.rs:243-292`): one fixture instance
/// is one backing store, each `connect()` a refcount clone. No borrowing GAT
/// anywhere, which is deliberate rather than incidental — it is one of five
/// ingredients of a rustc ICE this repository already minimised and which still
/// reproduces on 1.97.1.
#[derive(Debug)]
pub(crate) struct BufferingProjectionFixture(BufferingProjectionStore);

impl BufferingProjectionFixture {
    /// A fresh, isolated backing projection store.
    pub(crate) fn new() -> Self {
        Self(BufferingProjectionStore::new())
    }
}

impl ProjectionFixture for BufferingProjectionFixture {
    type Store = BufferingProjectionStore;

    // Supported: `SECOND_HANDLE` is a MUST, and every projection rule reads back
    // through a fresh handle. A `Clone` of the `Rc`s is one.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // Supported, and it has to be. A fixture that declined it would make
    // `refused_reset_changes_nothing` skip against every conformant variant this
    // family has — and CF-5's positive control would then say nothing about
    // whether that rule accepts a legal store. This fixture can support it for
    // `MutantFixture`'s reason: it is an instrument, and one protected id held in
    // a cell is as much of a policy as a conformance rule can observe.
    const RESET_REFUSAL: Capability = Capability::SUPPORTED;

    // Supported, for `RESET_REFUSAL`'s reason, and the fault it arms is the one
    // this shape actually meets: the queued request refused whole by the server.
    const COMMIT_FAULT: Capability = Capability::SUPPORTED;

    fn arm_commit_fault(&self) -> impl Future<Output = ()> {
        self.0.fault.set(true);
        // Ready rather than `async move`, for `connect`'s reason: setting a
        // `Cell` is not I/O and should not pretend to be.
        core::future::ready(())
    }

    fn protect_from_reset(&self, id: &ProjectionId) -> impl Future<Output = ()> {
        *self.0.protected.borrow_mut() = Some(id.as_str().to_owned());
        core::future::ready(())
    }

    fn connect(&self) -> impl Future<Output = Self::Store> {
        core::future::ready(self.0.clone())
    }
}
