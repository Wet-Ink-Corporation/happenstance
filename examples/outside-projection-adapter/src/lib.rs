//! An outside author's projection store adapter.
//!
//! This crate is not part of the library. It is the library's own falsifier: a
//! projection adapter written **from the published documentation**, in a crate
//! positioned so that the orphan rule, the feature flags and the dependency
//! graph all behave the way they would for a stranger. Everything it knows about
//! `ProjectionStore`, `ProjectionProbe` and `projection_store_conformance!` came
//! off the rendered rustdoc pages; the reference fixture and the reference store
//! were on a written denylist while it was built. What that cost is recorded in
//! `.bklg/from-contract-to-published-library/projection-store-freeze/documented-extension-surface/_extension-surface-gaps.md`.
//!
//! # The geometry, which is the point
//!
//! The store type **and both trait impls** live here, in the library. Only the
//! `ProjectionFixture` impl and the one-line suite invocation live in `tests/`.
//! That is not a tidiness preference — it is the shape the orphan rule forces,
//! and the reason `ProjectionProbe` lives in `happenstance-core` behind
//! `conformance` rather than in `happenstance-testkit`. `tests/` is a *different
//! crate*: there, neither a testkit trait nor this store's type would be local,
//! and `impl ProjectionProbe for OutsideProjectionStore` is `error[E0117]`. The
//! transcript is in the gap record above; the placement here is its disposition.
//!
//! # What is in here
//!
//! [`OutsideProjectionStore`] is the conformant one — an in-process key/value
//! read model with a checkpoint per [`ProjectionId`], both moved under one lock.
//! [`CheckpointOnlyStore`] beside it is this author's own analogue of the
//! wrong implementation the suite exists to reject: it commits the checkpoint
//! and silently drops the read-model write. It is here so that "the suite passed"
//! means something, since a suite no store can fail is decorative.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

// The bare flavour, never `SendProjectionStore`, and only one of the two names
// is imported here: the bare one is the weaker requirement, and having both in
// scope makes every method call ambiguous.
use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};

/// Mints one identity per **backing store**, so a batch can say where it came
/// from.
///
/// `CommitError::ForeignBatch` is documented as a run-time rejection, and the
/// port's own page says why a lifetime cannot do it: a lifetime names a region,
/// not an instance. With an owned batch the stamp is a field and the check is an
/// integer comparison, which is exactly what happens in [`commit`].
///
/// [`commit`]: ProjectionStore::commit
static NEXT_STORE_STAMP: AtomicU64 = AtomicU64::new(1);

/// How this adapter fails.
///
/// Hand-written rather than derived, and that is the *cost claim* being kept
/// honest rather than an aesthetic: the port asks only for
/// `core::error::Error + 'static`, and the walkthrough on `ProjectionStore`'s own
/// page spells its `ToyError` out by hand. Reaching for `thiserror` here would
/// have put a second crate in `[dependencies]` — a new edge in the graph — for
/// two lines this crate can write itself.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum OutsideStoreError {
    /// The fault a test fixture armed fired inside `commit`.
    ///
    /// This adapter has no network and no disk, so the only way it can report a
    /// failed commit is to be told to. That is what
    /// [`OutsideProjectionStore::arm_commit_fault`] is for, and declaring the
    /// capability is what obliges this store to leave both halves unchanged when
    /// it fires.
    ArmedFault,
}

impl core::fmt::Display for OutsideStoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ArmedFault => f.write_str("the armed fault fired while committing"),
        }
    }
}

impl core::error::Error for OutsideStoreError {}

/// The write set. Owned, and not a live transaction — this store has no
/// connection to hold one on.
///
/// A batch is a list of intentions plus the stamp of the store that opened it.
/// Applying an event mutates the list; nothing becomes durable until the batch
/// reaches [`commit`](ProjectionStore::commit) or
/// [`reset`](ProjectionStore::reset).
#[derive(Debug)]
pub struct OutsideBatch {
    stamp: u64,
    operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
enum Operation {
    Write { key: String, value: u64 },
    DeleteAll,
}

/// The state one backing store holds, behind one lock.
#[derive(Debug)]
struct Backing {
    stamp: u64,
    rows: BTreeMap<String, u64>,
    checkpoints: BTreeMap<ProjectionId, Checkpoint>,
    fault_armed: bool,
}

/// A projection store an outside author would recognise: a map of rows, a
/// checkpoint per projection, and one lock over both.
///
/// One [`new`](Self::new) is one backing store. Each [`handle`](Self::handle) is
/// another handle onto that same store — which is what makes the second-handle
/// reads every conformance rule performs mean anything, since the invariant this
/// port exists for is only observable from outside the connection that made the
/// commit.
#[derive(Debug, Clone)]
pub struct OutsideProjectionStore {
    backing: Arc<Mutex<Backing>>,
}

impl OutsideProjectionStore {
    /// The reason this store declines to be asked for a refused reset.
    ///
    /// A `&'static str` on the store rather than a sentence typed into a
    /// fixture, because the reason is a fact about *this store* and the fixture
    /// is only where it is declared. A test that wants to assert on the skip
    /// reads it back off the fixture's own constant rather than repeating the
    /// words.
    pub const NO_RESET_PROTECTION: &'static str = "this store holds no protection policy at all: it is a process-local map \
         with no notion of a projection anyone has asked it to guard, so there is \
         no reset it could decline. `reset` here is unconditional, and a refusal \
         it could produce would be a refusal nothing had asked for";

    /// Opens a new, empty backing store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            backing: Arc::new(Mutex::new(Backing {
                stamp: NEXT_STORE_STAMP.fetch_add(1, Ordering::Relaxed),
                rows: BTreeMap::new(),
                checkpoints: BTreeMap::new(),
                fault_armed: false,
            })),
        }
    }

    /// Another handle onto the **same** backing store.
    ///
    /// Spelled out rather than left to `Clone` so the call site reads as what it
    /// is. A conformance fixture's `connect` is this.
    #[must_use]
    pub fn handle(&self) -> Self {
        Self {
            backing: Arc::clone(&self.backing),
        }
    }

    /// Arms the store so the **next** `commit` reports failure.
    ///
    /// This adapter cannot fail on its own, so the fault is injected — which is
    /// the documented shape of the capability: every store that can make a
    /// commit fail does it differently, so the mechanism belongs to the adapter
    /// rather than to the suite. Here it fires *after* the rows have been
    /// applied and *before* the checkpoint moves, which is the interesting
    /// half-way point: the store must put the rows back.
    pub fn arm_commit_fault(&self) {
        self.locked().fault_armed = true;
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, Backing> {
        self.backing
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Default for OutsideProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Applies one operation to a row map. Shared by `commit`, `reset` and the
/// read-through path, so an uncommitted read cannot disagree with what a commit
/// would have done.
fn apply(rows: &mut BTreeMap<String, u64>, operation: &Operation) {
    match operation {
        Operation::Write { key, value } => {
            rows.insert(key.clone(), *value);
        }
        Operation::DeleteAll => rows.clear(),
    }
}

/// What a commit claims, turned into what `checkpoint` will report.
///
/// The wildcard arm is not laziness: [`Authority`] is `#[non_exhaustive]`, so a
/// match without one does not compile outside the contract crate. Treating an
/// unknown future claim as authoritative is the conservative reading — a store
/// that guessed "rebuilding" would tell a runner its rows cannot be trusted when
/// nobody said that.
fn checkpoint_for(authority: Authority, position: SequencePosition) -> Checkpoint {
    match authority {
        Authority::Rebuilding => Checkpoint::Rebuilding { through: position },
        // `Authority::Live`, and any claim a later version adds that this store
        // has not been taught.
        _ => Checkpoint::Live { through: position },
    }
}

/// Whether `position` would move `current` backwards.
///
/// Equality is **not** a regression: the checkpoint is a high-water mark of
/// consideration, and re-considering the same position is a no-op rather than a
/// mistake. Only a strictly lower position is refused.
fn regression(current: Checkpoint, position: SequencePosition) -> Option<SequencePosition> {
    // `let … else` rather than a `match` with two `None` arms: `Checkpoint` is
    // `#[non_exhaustive]`, so `NeverRun` and any variant a later version adds
    // both land here, and both mean the same thing — there is no recorded
    // position for `position` to be below.
    let (Checkpoint::Live { through } | Checkpoint::Rebuilding { through }) = current else {
        return None;
    };
    (position < through).then_some(through)
}

impl ProjectionStore for OutsideProjectionStore {
    type Error = OutsideStoreError;

    // No lifetime, and no `where Self: 'a`. The port asks for an owned batch and
    // this is what that buys: the concrete type, spelled straight out.
    type Batch = OutsideBatch;

    fn begin(&self) -> OutsideBatch {
        OutsideBatch {
            stamp: self.locked().stamp,
            operations: Vec::new(),
        }
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, OutsideStoreError> {
        Ok(self
            .locked()
            .checkpoints
            .get(id)
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

    async fn commit(
        &self,
        batch: OutsideBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<OutsideStoreError>> {
        // One lock held across the whole thing, which is the whole of how this
        // store keeps the read-model write and the checkpoint write together.
        let mut backing = self.locked();

        if batch.stamp != backing.stamp {
            return Err(CommitError::ForeignBatch);
        }

        let current = backing
            .checkpoints
            .get(id)
            .copied()
            .unwrap_or(Checkpoint::NeverRun);
        if let Some(recorded) = regression(current, position) {
            return Err(CommitError::CheckpointRegression {
                current: recorded,
                attempted: position,
            });
        }

        // Deliberately applied *before* the fault is consulted, so that when one
        // is armed this store really has half-committed and really has to undo
        // it. Checking the flag first would make the rule that gates on
        // `COMMIT_FAULT` a green result about a store nothing ever faulted.
        let unchanged = backing.rows.clone();
        for operation in &batch.operations {
            apply(&mut backing.rows, operation);
        }

        if backing.fault_armed {
            backing.fault_armed = false;
            backing.rows = unchanged;
            return Err(CommitError::Store(OutsideStoreError::ArmedFault));
        }

        // `position` is the position *considered*, not the position applied: a
        // batch that wrote nothing still advances the checkpoint, and this store
        // must not check `position` against what the batch contained.
        backing
            .checkpoints
            .insert(id.clone(), checkpoint_for(authority, position));
        Ok(())
    }

    async fn reset(
        &self,
        batch: OutsideBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<OutsideStoreError>> {
        let mut backing = self.locked();

        if batch.stamp != backing.stamp {
            return Err(ResetError::ForeignBatch);
        }

        // `commit`'s dual, under the same lock: the caller's own batch carries
        // the deletes — this store has no idea which rows belong to `id` — and
        // the checkpoint goes back to `NeverRun`, which is a *variant* rather
        // than a position, so a reset projection can never be mistaken for one
        // committed at the first position.
        for operation in &batch.operations {
            apply(&mut backing.rows, operation);
        }
        backing.checkpoints.remove(id);
        Ok(())
    }

    async fn rollback(&self, batch: OutsideBatch) -> Result<(), OutsideStoreError> {
        // Nothing to release: the batch is a list this store never saw.
        drop(batch);
        Ok(())
    }
}

impl ProjectionProbe for OutsideProjectionStore {
    // This store's batch is a list of intentions and the committed rows are a
    // map, so overlaying one on the other is cheap and honest. An adapter that
    // buffers statements it cannot query would declare `false` here, which the
    // port permits outright.
    const READS_THROUGH_BATCH: bool = true;

    fn probe_write(&self, batch: &mut OutsideBatch, key: &str, value: u64) {
        batch.operations.push(Operation::Write {
            key: key.to_owned(),
            value,
        });
    }

    fn probe_delete_all(&self, batch: &mut OutsideBatch) {
        batch.operations.push(Operation::DeleteAll);
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, OutsideStoreError> {
        Ok(self.locked().rows.get(key).copied())
    }

    fn probe_read_through(&self, batch: &OutsideBatch, key: &str) -> Option<u64> {
        let mut rows = self.locked().rows.clone();
        for operation in &batch.operations {
            apply(&mut rows, operation);
        }
        rows.get(key).copied()
    }
}

/// The wrong implementation this author wrote on purpose: a store that moves the
/// checkpoint and silently drops the read-model write.
///
/// It is not a strawman. It is the shape a real adapter takes when the two
/// writes go to two places and only one of them is in the transaction, and it
/// passes every test that watches only the checkpoint — which is exactly the
/// mistake "it compiles, so it is correct" makes. `tests/` drives it through the
/// published rule set to show which rule catches it.
#[derive(Debug, Clone)]
pub struct CheckpointOnlyStore {
    backing: Arc<Mutex<Backing>>,
}

impl CheckpointOnlyStore {
    /// Opens a new, empty backing store.
    #[must_use]
    pub fn new() -> Self {
        Self {
            backing: Arc::new(Mutex::new(Backing {
                stamp: NEXT_STORE_STAMP.fetch_add(1, Ordering::Relaxed),
                rows: BTreeMap::new(),
                checkpoints: BTreeMap::new(),
                fault_armed: false,
            })),
        }
    }

    /// Another handle onto the same backing store.
    #[must_use]
    pub fn handle(&self) -> Self {
        Self {
            backing: Arc::clone(&self.backing),
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, Backing> {
        self.backing
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

impl Default for CheckpointOnlyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectionStore for CheckpointOnlyStore {
    type Error = OutsideStoreError;
    type Batch = OutsideBatch;

    fn begin(&self) -> OutsideBatch {
        OutsideBatch {
            stamp: self.locked().stamp,
            operations: Vec::new(),
        }
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, OutsideStoreError> {
        Ok(self
            .locked()
            .checkpoints
            .get(id)
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

    async fn commit(
        &self,
        batch: OutsideBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<OutsideStoreError>> {
        let mut backing = self.locked();

        if batch.stamp != backing.stamp {
            return Err(CommitError::ForeignBatch);
        }

        // The defect, in one line: the batch is dropped and only the checkpoint
        // moves.
        drop(batch);

        backing
            .checkpoints
            .insert(id.clone(), checkpoint_for(authority, position));
        Ok(())
    }

    async fn reset(
        &self,
        batch: OutsideBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<OutsideStoreError>> {
        let mut backing = self.locked();

        if batch.stamp != backing.stamp {
            return Err(ResetError::ForeignBatch);
        }

        drop(batch);
        backing.checkpoints.remove(id);
        Ok(())
    }

    async fn rollback(&self, batch: OutsideBatch) -> Result<(), OutsideStoreError> {
        drop(batch);
        Ok(())
    }
}

impl ProjectionProbe for CheckpointOnlyStore {
    const READS_THROUGH_BATCH: bool = true;

    fn probe_write(&self, batch: &mut OutsideBatch, key: &str, value: u64) {
        batch.operations.push(Operation::Write {
            key: key.to_owned(),
            value,
        });
    }

    fn probe_delete_all(&self, batch: &mut OutsideBatch) {
        batch.operations.push(Operation::DeleteAll);
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, OutsideStoreError> {
        Ok(self.locked().rows.get(key).copied())
    }

    fn probe_read_through(&self, batch: &OutsideBatch, key: &str) -> Option<u64> {
        let mut rows = self.locked().rows.clone();
        for operation in &batch.operations {
            apply(&mut rows, operation);
        }
        rows.get(key).copied()
    }
}
