//! The correct projection store, in the pieces a defect replaces.
//!
//! Every hostile store in this binary is this store with **one step** overridden.
//! That is not tidiness: it is what makes a red rule attributable. A mutant that
//! reimplements [`ProjectionStore`] from scratch fails the rule it was written
//! for *and* whatever else its author got wrong on the way, and the exactness
//! meta-test then catches the second bug in the instrument rather than in the
//! adapter (`crates/happenstance-testkit/tests/mutation_coverage/mutants.rs:11-19`
//! records the event-store family learning this).
//!
//! # What this store is a copy of, and where it deliberately differs
//!
//! It is `MemoryProjectionStore` (`crates/happenstance-core/src/projection_memory.rs`)
//! read and rewritten, not wrapped. Wrapping the oracle would make every mutant
//! a delegation with one branch, and there would be nowhere to put a defect that
//! lives *between* two of the oracle's statements — which is where PS-1's
//! coupling defect lives.
//!
//! Two differences from the oracle are load-bearing:
//!
//! * **`Rc<RefCell<_>>`, not `Arc<RwLock<_>>`.** These stores are driven on one
//!   thread by `happenstance_testkit::block_on`, and being `!Send` is the point:
//!   they implement the **bare** [`ProjectionStore`] flavour, which is the one
//!   ADR-0001 exists for and the one an adapter on `wasm32` can satisfy. A
//!   mutant set that only ever exercised the `Send` flavour would leave the
//!   flavour the port was split for untested by the instrument that proves the
//!   suite discriminates.
//! * **An uninhabited error.** Nothing in the correct core can fail, exactly as
//!   `MemoryProjectionStoreError` cannot, so `Self::Error`'s bound is discharged
//!   by a type with no values. A defect that needs a *store* failure is the
//!   signal to inhabit it, and the type going from empty to non-empty is a
//!   visible event in review rather than a silent one.

use core::cell::RefCell;
use core::fmt;
use core::future::Future;
use core::marker::PhantomData;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};
use happenstance_testkit::{Capability, ProjectionFixture};

use crate::harness::ProjectionSubject;

// =====================================================================
// The state a projection store holds
// =====================================================================

/// The read model and the checkpoints, behind **one** borrow.
///
/// Keeping them in one struct behind one `RefCell` is what makes "durable
/// together or not at all" true by construction in the *correct* core, so a store
/// that breaks the coupling has to say so in a step override rather than by
/// accident. Two cells would compile, pass every test that reads them separately,
/// and reintroduce the window the port exists to close.
#[derive(Debug, Default)]
pub(crate) struct State {
    /// The read model: the probe rows a committed batch left behind.
    pub(crate) rows: BTreeMap<String, u64>,
    /// One checkpoint per key, where the key is normally the projection's id.
    pub(crate) checkpoints: BTreeMap<String, Checkpoint>,
}

/// How every store in this binary fails.
///
/// **Uninhabited**, for `MemoryProjectionStoreError`'s reason: it proves the
/// contract does not *require* a fallible read path, and it documents at the type
/// level that the correct core has no failure modes of its own.
/// [`CommitError`] and [`ResetError`] are still meaningfully fallible — a foreign
/// batch and a regressing position are both reachable — so a rule that needs a
/// rejection has one without this type carrying a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MutantError {}

impl fmt::Display for MutantError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Uninhabited: there is no value to render, and `match *self {}` is how
        // the compiler is told so.
        match *self {}
    }
}

impl core::error::Error for MutantError {}

/// A fresh per-instance stamp.
///
/// Process-local and monotonic, which is all a foreign-batch check needs: a batch
/// cannot outlive the process that minted it. The counter starts at `1` so that a
/// store which forgot to call it — and left the field at its `Default` — is
/// distinguishable from one that did.
fn next_stamp() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// The position a checkpoint has been considered through, if any.
///
/// `NeverRun` has none, which is why a first commit at any position is accepted
/// and only a *regression* against a recorded position is refused.
const fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
        // `NeverRun` and any variant added later: nothing has been considered.
        _ => None,
    }
}

/// Applies a batch's queued writes to the read model.
///
/// Takes the batch by `&mut` and drains it rather than consuming it, because
/// [`MutantBatch`] will grow a `Drop` impl the moment a defect needs one and a
/// type with `Drop` cannot be destructured field by field. Draining now costs
/// nothing and removes a refactor that would otherwise arrive as a compile error
/// in eight places at once.
pub(crate) fn apply<D: Defect>(state: &mut State, batch: &mut MutantBatch<D>) {
    if batch.clear_all {
        state.rows.clear();
    }
    state.rows.append(&mut batch.writes);
}

// =====================================================================
// The seam
// =====================================================================

/// One step of the correct store, replaced.
///
/// Every method has a body, and every body is the correct one. An implementation
/// that overrides nothing is the reference store; an implementation that
/// overrides one method is a mutant whose failure is attributable to that method.
///
/// # Why associated functions rather than methods on a value
///
/// The store is [`MutantStore<D>`], and `D` is never constructed — it is a type
/// carried in a [`PhantomData`] purely so that monomorphisation can pick the
/// right step. A trait of `&self` methods would need a value of `D` in every
/// store, and a `Box<dyn Defect>` inside the store would stop the store being
/// [`UnwindSafe`](std::panic::UnwindSafe), which `harness.rs` is built to avoid
/// needing to assert away.
///
/// # Why the step set is small, and how it grows
///
/// One step, because one defect needs it. The event-store family's `Defect` grew
/// from three steps to nine, each time because *a rule acquired the ability to
/// see a defect there* — `head_of` and `contains` say so in their own doc
/// comments. The same rule applies here: a step earns its place when a
/// conformance rule can observe the difference, and not before. Adding a seam for
/// a defect no rule can see is how a mutant registry starts describing the
/// instrument instead of the port.
pub(crate) trait Defect: 'static + Sized {
    /// The registry key, which names the *store*.
    const NAME: &'static str;

    /// Both halves of a commit, made durable as one unit.
    ///
    /// **One step rather than two**, and that is the whole design of this seam.
    /// PS-1's obligation is the *coupling*, so the interesting defects are the
    /// ways the two halves come apart: one half written and not the other, or
    /// neither written while `Ok` is returned. Splitting this into an
    /// `apply_rows` step and a `record_checkpoint` step would spell the second
    /// defect as two overrides and quietly break "one defect per store" for the
    /// one store that most needs it to hold.
    ///
    /// The `key` is passed in already computed, because it is derived from the
    /// [`ProjectionId`] and the caller has already used it to read the recorded
    /// checkpoint back — a step that recomputed it could disagree with the
    /// regression check that ran a line earlier.
    fn commit_writes(
        state: &mut State,
        batch: &mut MutantBatch<Self>,
        key: &str,
        checkpoint: Checkpoint,
    ) {
        apply(state, batch);
        state.checkpoints.insert(key.to_owned(), checkpoint);
    }
}

// =====================================================================
// The store, the batch and the fixture
// =====================================================================

/// An in-flight write against a [`MutantStore`].
///
/// Owned, and not a live transaction: it holds a materialised delta plus the
/// stamp of the store instance that minted it. Nothing it holds is visible
/// through the store until [`commit`](ProjectionStore::commit) or
/// [`reset`](ProjectionStore::reset) takes it.
pub(crate) struct MutantBatch<D: Defect> {
    /// The identity of the store instance that minted this batch.
    pub(crate) stamp: u64,
    /// The rows this batch will write when it is applied.
    pub(crate) writes: BTreeMap<String, u64>,
    /// Whether this batch clears the read model before applying its writes.
    pub(crate) clear_all: bool,
    /// The defect this batch's store carries. Never a value.
    defect: PhantomData<D>,
}

impl<D: Defect> fmt::Debug for MutantBatch<D> {
    // Hand-written rather than derived: `#[derive(Debug)]` would add a
    // `D: Debug` bound, and `D` is a marker type nobody constructs. Naming the
    // defect is also more useful than naming the type parameter.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutantBatch")
            .field("defect", &D::NAME)
            .field("stamp", &self.stamp)
            .field("writes", &self.writes)
            .field("clear_all", &self.clear_all)
            .finish()
    }
}

/// A projection store whose every step is `D`'s.
///
/// One instance is one backing store; [`Clone`] is one more **handle** onto it,
/// which is what [`ProjectionFixture::connect`] hands out. The `Rc` is the reason
/// [`ProjectionFixture::Store`] can be an ordinary associated type rather than a
/// borrowing GAT: the handle owns a refcount into the state instead of borrowing
/// a lifetime from the fixture, which is the difference between a compiling
/// contract and the rustc ICE this repository minimised
/// (`crates/happenstance-testkit/src/contract.rs:108-122`).
pub(crate) struct MutantStore<D: Defect> {
    state: Rc<RefCell<State>>,
    stamp: u64,
    defect: PhantomData<D>,
}

impl<D: Defect> fmt::Debug for MutantStore<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MutantStore")
            .field("defect", &D::NAME)
            .field("stamp", &self.stamp)
            .finish_non_exhaustive()
    }
}

impl<D: Defect> Clone for MutantStore<D> {
    // Hand-written for [`fmt::Debug`]'s reason: the derive would demand
    // `D: Clone` of a type nobody constructs.
    fn clone(&self) -> Self {
        Self {
            state: Rc::clone(&self.state),
            stamp: self.stamp,
            defect: PhantomData,
        }
    }
}

impl<D: Defect> MutantStore<D> {
    /// A store over a fresh, empty read model and no checkpoints.
    fn new() -> Self {
        Self {
            state: Rc::new(RefCell::new(State::default())),
            stamp: next_stamp(),
            defect: PhantomData,
        }
    }
}

// The **bare** flavour, never `SendProjectionStore`, and only one of the two
// names is in scope in this file (CLAUDE.md binding constraint 4). It is the
// weaker requirement, it is what `ProjectionFixture::Store`'s bound accepts, and
// an `Rc`-backed store could not satisfy the other one.
impl<D: Defect> ProjectionStore for MutantStore<D> {
    type Error = MutantError;

    type Batch = MutantBatch<D>;

    fn begin(&self) -> Self::Batch {
        MutantBatch {
            stamp: self.stamp,
            writes: BTreeMap::new(),
            clear_all: false,
            defect: PhantomData,
        }
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        Ok(self
            .state
            .borrow()
            .checkpoints
            .get(id.as_str())
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

    async fn commit(
        &self,
        mut batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(CommitError::ForeignBatch);
        }

        let mut state = self.state.borrow_mut();

        let recorded = state
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

        let checkpoint = match authority {
            Authority::Rebuilding => Checkpoint::Rebuilding { through: position },
            // `Live` and any variant added later: a commit that does not claim a
            // rebuild is claiming the rows are authoritative.
            _ => Checkpoint::Live { through: position },
        };

        D::commit_writes(&mut state, &mut batch, id.as_str(), checkpoint);
        Ok(())
    }

    async fn reset(
        &self,
        mut batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        let mut state = self.state.borrow_mut();
        apply(&mut state, &mut batch);
        // *Removing* the key is what returns the projection to `NeverRun`.
        state.checkpoints.remove(id.as_str());
        Ok(())
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        drop(batch);
        Ok(())
    }
}

impl<D: Defect> ProjectionProbe for MutantStore<D> {
    /// `true`: this store's batch is a materialised delta layered over committed
    /// state, so it can be read back through before it commits — the same end of
    /// the batch-shape axis `MemoryProjectionStore` sits at.
    const READS_THROUGH_BATCH: bool = true;

    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.writes.insert(key.to_owned(), value);
    }

    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.writes.clear();
        batch.clear_all = true;
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        Ok(self.state.borrow().rows.get(key).copied())
    }

    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64> {
        if let Some(pending) = batch.writes.get(key) {
            return Some(*pending);
        }
        if batch.clear_all {
            return None;
        }
        self.state.borrow().rows.get(key).copied()
    }
}

/// One backing projection store for a [`MutantStore`], and any number of handles
/// onto it.
pub(crate) struct MutantFixture<D: Defect>(MutantStore<D>);

impl<D: Defect> fmt::Debug for MutantFixture<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("MutantFixture").field(&self.0).finish()
    }
}

impl<D: Defect> ProjectionFixture for MutantFixture<D> {
    type Store = MutantStore<D>;

    // Supported, and it has to be: `SECOND_HANDLE` is a MUST, every projection
    // rule reads back through a fresh handle, and a fixture declining it fails
    // every rule rather than skipping — which would make every mutant in this
    // binary indistinguishable from every other.
    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    // The same honest answer `MemoryProjectionFixture` gives, and for the same
    // reason rather than a copied sentence: the correct core holds no protection
    // policy, so `reset` returns `Refused` on no path at all and there is no
    // projection any of these stores could decline to reset.
    const RESET_REFUSAL: Capability = Capability::declined(
        "a projection mutant is a BTreeMap behind an Rc and holds no protection \
         policy, so there is no projection it could decline to reset",
    );

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // Ready rather than `async move`, for `MemoryProjectionFixture::connect`'s
        // reason: acquiring this handle is a refcount bump, and pretending
        // otherwise would hide that a real fixture's `connect` does I/O and this
        // one does not.
        core::future::ready(self.0.clone())
    }
}

impl<D: Defect> ProjectionSubject for MutantFixture<D> {
    const NAME: &'static str = D::NAME;

    fn open() -> Self {
        Self(MutantStore::new())
    }
}
