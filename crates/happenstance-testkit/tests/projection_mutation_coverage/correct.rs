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
//! * **An error only a mutant can produce.** `MutantError` began uninhabited,
//!   exactly as `MemoryProjectionStoreError` is, and grew two variants when two
//!   defects needed a store failure the port's own error variants could not
//!   express. The correct core still returns neither of them, so the claim
//!   "nothing here fails on its own" is preserved by the *bodies* rather than by
//!   the type — and inhabiting the type was a visible event in review rather
//!   than a silent one.
//! * **One connection, modelled.** The store holds an `Rc<Cell<bool>>` and a
//!   batch holds a share in it. Without a resource a batch can fail to give
//!   back, PS-7's second half — *and the store is still usable afterwards* — is
//!   true by construction, and the rule that asserts it is decorative.

use core::cell::{Cell, RefCell};
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

/// How a store in this binary fails for reasons of its own.
///
/// It was **uninhabited** when this file was written, exactly as
/// `MemoryProjectionStoreError` is, and it stopped being so the moment a defect
/// needed a store failure the port's own variants could not express. That
/// transition is deliberately visible in review rather than silent: an
/// uninhabited error is a claim that the correct core cannot fail, and inhabiting
/// it withdraws that claim.
///
/// Both variants are reachable only from a *mutant*. The correct core returns
/// neither: it always holds its connection when it commits, and it never
/// validates a position (PS-21 forbids it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MutantError {
    /// The store's one connection is checked out and was never returned.
    ///
    /// What a pooled adapter answers after a `Drop` that returned its connection
    /// to nothing — the defect a reviewer's probe actually found
    /// (`spec/SPECIFICATION.md:4898-4910`).
    Busy,

    /// The commit named a position nothing in the batch applied.
    ///
    /// What an adapter that *validates* `position` answers. PS-21 forbids the
    /// validation; this variant exists so a store can be written that does it
    /// anyway.
    PositionNotApplied,
}

impl fmt::Display for MutantError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Busy => {
                f.write_str("this store's one connection is checked out and was never returned")
            }
            Self::PositionNotApplied => {
                f.write_str("the commit named a position this batch did not write")
            }
        }
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
/// Takes the batch by `&mut` and drains it rather than consuming it, and that is
/// now load-bearing rather than merely forward-looking: [`MutantBatch`] has a
/// `Drop` impl — it is what returns the store's one connection — and a type with
/// `Drop` cannot be destructured field by field.
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
/// # Why the step set is what it is, and how it grew
///
/// It started at one step, because one defect needed it. It is seven now, and
/// every one of the six added arrived the same way the event-store family's
/// ninth did: *a rule acquired the ability to see a defect there*. A step earns
/// its place when a conformance rule can observe the difference, and not before —
/// adding a seam for a defect no rule can see is how a mutant registry starts
/// describing the instrument instead of the port.
///
/// Read that as a map. There is one step per commit-path obligation the suite now
/// enforces: identity ([`mint_stamp`](Self::mint_stamp), PS-15), scope
/// ([`checkpoint_key`](Self::checkpoint_key), PS-23), monotonicity
/// ([`regression`](Self::regression), PS-22), the forbidden validation
/// ([`validate_position`](Self::validate_position), PS-21), durability
/// ([`commit_writes`](Self::commit_writes), PS-1), undo
/// ([`rollback`](Self::rollback), PS-8) and resource release
/// ([`release_the_connection`](Self::release_the_connection), PS-7).
pub(crate) trait Defect: 'static + Sized {
    /// The registry key, which names the *store*.
    const NAME: &'static str;

    /// The identity a fresh store instance is minted with.
    ///
    /// PS-15's seam. The correct answer is a value from a process-local counter,
    /// so that two instances never compare equal and `commit` can reject a batch
    /// begun on the other one. A store that mints per *type* instead — a `const`,
    /// a `Default`, a hash of the connection string — compiles, passes every
    /// single-store rule, and corrupts silently.
    fn mint_stamp() -> u64 {
        next_stamp()
    }

    /// The key this store files a projection's checkpoint under.
    ///
    /// PS-23's seam, and it is *one* step read by both `commit` and `checkpoint`
    /// rather than a write step and a read step. A store that recorded under one
    /// key and read under another would be two defects wearing one name, and no
    /// adapter is written that way: the single-row checkpoint table this exists
    /// to model gets the key wrong in exactly one place, its schema.
    fn checkpoint_key(id: &ProjectionId) -> String {
        id.as_str().to_owned()
    }

    /// Whether a commit at `position` regresses against `recorded`.
    ///
    /// PS-22's seam. `None` accepts. The correct answer refuses a position
    /// strictly below the recorded one and **accepts an equal one**, which the
    /// clause permits and no rule may assert against.
    fn regression(
        recorded: Checkpoint,
        position: SequencePosition,
    ) -> Option<CommitError<MutantError>> {
        match considered_through(recorded) {
            Some(current) if position < current => Some(CommitError::CheckpointRegression {
                current,
                attempted: position,
            }),
            _ => None,
        }
    }

    /// Whether this store refuses a commit whose position names nothing the batch
    /// applied.
    ///
    /// PS-21's seam, and **the correct answer is always `None`** — the clause
    /// forbids the validation outright. It is a step at all because "validate the
    /// position" is a *reasonable* reading of the port that would be equally
    /// conformant without the rule, which is what makes
    /// `commit_accepts_a_position_the_batch_did_not_write` worth having.
    fn validate_position(batch: &MutantBatch<Self>) -> Option<CommitError<MutantError>> {
        let _ = batch;
        None
    }

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

    /// What a `rollback` does with the batch it was handed.
    ///
    /// PS-8's seam. The correct answer is *nothing*: the batch is a buffer, and
    /// discarding it is the whole of the undo. The defect this exists to model is
    /// a `rollback` that releases its connection without issuing `ROLLBACK`, so
    /// the rows stay durable and the call still reports success.
    fn rollback(state: &mut State, batch: &mut MutantBatch<Self>) {
        let _ = (state, batch);
    }

    /// Returns the store's one connection when a batch is dropped **bare**.
    ///
    /// PS-7's seam, and the only step reached from a `Drop` impl rather than from
    /// a port method — which is exactly why the defect it models is so easy to
    /// ship. `commit` and `rollback` release the connection explicitly; a bare
    /// drop is the path nobody writes a test for.
    fn release_the_connection(connection: &Cell<bool>) {
        connection.set(false);
    }
}

// =====================================================================
// The store, the batch and the fixture
// =====================================================================

/// An in-flight write against a [`MutantStore`].
///
/// Owned, and not a live transaction: it holds a materialised delta, the stamp of
/// the store instance that minted it, and — because the whole point of PS-7 is
/// that a batch can hold a *resource* — a share in that store's one connection.
/// Nothing it holds is visible through the store until
/// [`commit`](ProjectionStore::commit) or [`reset`](ProjectionStore::reset) takes
/// it.
pub(crate) struct MutantBatch<D: Defect> {
    /// The identity of the store instance that minted this batch.
    pub(crate) stamp: u64,
    /// The rows this batch will write when it is applied.
    pub(crate) writes: BTreeMap<String, u64>,
    /// Whether this batch clears the read model before applying its writes.
    pub(crate) clear_all: bool,
    /// The minting store's connection: `true` while it is checked out.
    connection: Rc<Cell<bool>>,
    /// Whether *this* batch is the one holding that connection.
    ///
    /// `false` for a batch begun while the connection was already out — which
    /// the correct store never produces, because it always gets its connection
    /// back.
    pub(crate) holds_the_connection: bool,
    /// The defect this batch's store carries. Never a value.
    defect: PhantomData<D>,
}

impl<D: Defect> Drop for MutantBatch<D> {
    /// Returns the store's connection, if this batch had it.
    ///
    /// This is the **bare drop** path PS-7 is about, and it is also what runs
    /// after `commit` and `rollback` have already released it explicitly — which
    /// is harmless, because releasing a connection twice is idempotent and
    /// modelling it otherwise would invent a defect no adapter has.
    fn drop(&mut self) {
        if self.holds_the_connection {
            D::release_the_connection(&self.connection);
        }
    }
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
            .field("holds_the_connection", &self.holds_the_connection)
            // The `Rc<Cell<bool>>` itself is the store's, not this batch's, and
            // rendering it would print the same shared value under two names.
            .finish_non_exhaustive()
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
    /// This store's one connection: `true` while a batch has it checked out.
    ///
    /// Shared by every handle, because a pooled adapter's handles share a pool.
    /// It is the smallest thing that makes PS-7's second half — *and the store
    /// is still usable afterwards* — observable at all: without a resource a
    /// batch can fail to give back, "dropping a batch leaves the store usable"
    /// is true by construction and the rule is decorative.
    connection: Rc<Cell<bool>>,
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
            connection: Rc::clone(&self.connection),
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
            connection: Rc::new(Cell::new(false)),
            stamp: D::mint_stamp(),
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
        // Checking the connection out. A batch begun while it is already out
        // does not hold it, and every write path below refuses such a batch —
        // which is how a store that never gives its connection back answers
        // `Busy` forever rather than merely losing one write.
        let free = !self.connection.get();
        self.connection.set(true);

        MutantBatch {
            stamp: self.stamp,
            writes: BTreeMap::new(),
            clear_all: false,
            connection: Rc::clone(&self.connection),
            holds_the_connection: free,
            defect: PhantomData,
        }
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        Ok(self
            .state
            .borrow()
            .checkpoints
            .get(&D::checkpoint_key(id))
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
        // Order matters and is the oracle's: identity, then the resource, then
        // the port-level refusals, then the write. A store that checked the
        // position before the stamp would answer the wrong error for a batch that
        // is both foreign and regressing.
        if batch.stamp != self.stamp {
            return Err(CommitError::ForeignBatch);
        }
        if !batch.holds_the_connection {
            return Err(CommitError::Store(MutantError::Busy));
        }

        let key = D::checkpoint_key(id);
        let mut state = self.state.borrow_mut();

        let recorded = state
            .checkpoints
            .get(&key)
            .copied()
            .unwrap_or(Checkpoint::NeverRun);
        if let Some(refusal) = D::regression(recorded, position) {
            return Err(refusal);
        }
        if let Some(refusal) = D::validate_position(&batch) {
            return Err(refusal);
        }

        let checkpoint = match authority {
            Authority::Rebuilding => Checkpoint::Rebuilding { through: position },
            // `Live` and any variant added later: a commit that does not claim a
            // rebuild is claiming the rows are authoritative.
            _ => Checkpoint::Live { through: position },
        };

        D::commit_writes(&mut state, &mut batch, &key, checkpoint);
        drop(state);

        // `COMMIT` returns the connection to the pool. This is the *explicit*
        // release, and it is not the one PS-7 is about — the batch's `Drop` is.
        self.connection.set(false);
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
        if !batch.holds_the_connection {
            return Err(ResetError::Store(MutantError::Busy));
        }

        let key = D::checkpoint_key(id);
        let mut state = self.state.borrow_mut();
        apply(&mut state, &mut batch);
        // *Removing* the key is what returns the projection to `NeverRun`.
        state.checkpoints.remove(&key);
        drop(state);

        self.connection.set(false);
        Ok(())
    }

    async fn rollback(&self, mut batch: Self::Batch) -> Result<(), Self::Error> {
        if !batch.holds_the_connection {
            return Err(MutantError::Busy);
        }

        let mut state = self.state.borrow_mut();
        D::rollback(&mut state, &mut batch);
        drop(state);

        self.connection.set(false);
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
