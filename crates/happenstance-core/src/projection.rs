//! The projection store port.
//!
//! # Status: behind `unstable-projection`, and this is why
//!
//! Unlike [`EventStore`](crate::EventStore), this port is **not yet frozen**, and
//! naming the `unstable-projection` feature is how you say you accept that. It is
//! exempt from this crate's semver promise for as long as the feature exists.
//!
//! The reason is **not** that nothing tests it, which is what this header used to
//! say. All seventeen conformance rules §4.11 assigns to an adapter's own suite
//! are written and drive this port, a deliberately wrong store that writes a
//! checkpoint without its read model fails one of them by name, and two
//! structurally unlike batch shapes pass all of them.
//!
//! The reason is the bar the specification set for freezing it. `spec/SPECIFICATION.md`
//! §4's PS-2 requires the suite to be green against **two adapters at opposite
//! ends of the batch-shape axis**, and its *Rejects* field names the alternative
//! verbatim: the schedule that freezes this port against `MemoryProjectionStore`
//! and one in-process transaction. That schedule has been overtaken. **Four
//! adapters over storage this workspace does not fully control now clear all
//! seventeen rules** — `happenstance-sqlite`, `happenstance-postgres`,
//! `happenstance-neon` and `happenstance-ladybug` — alongside the two
//! testkit-side instruments. The bar did not move when they arrived, and the
//! reason it did not is the point of the paragraph below.
//!
//! (The name is not a link, for the reason the crate root gives: it is behind
//! `memory`, and `unstable-projection` without `memory` is a configuration this
//! module renders in.)
//!
//! **Why more adapters do not clear it.** PS-2 names *two adapters at opposite
//! ends of the batch-shape axis*, and every implementation there has ever been
//! sits at one end: an **owned, buffered write set** replayed at commit. The far
//! end — a batch holding a **live transaction** — is not merely unbuilt, it is
//! **unreachable through this port's own signatures**. `begin` is total,
//! synchronous and infallible, and every route to a real driver's transaction is
//! `async` and fallible, so no driver whose transaction is acquired
//! asynchronously can be that far end. A skeleton did not report this because
//! `todo!()` has type `!` and coerces to anything; `happenstance-postgres`
//! discovered it against `sqlx` and `happenstance-ladybug` preserved a compiling
//! counter-example at `experiments/live-handle-projection-batch/`.
//!
//! So the freeze waits on a **replacement axis**, not on another adapter, and
//! writing "what would clear it" as a list of adapters was the mistake this
//! paragraph replaces. ADR-0060 re-evaluates the axis.
//!
//! **What is not decided here:** which axis replaces it. `0.2.0` ships with the
//! gate still closed, and this module states the evidence rather than the verdict.
//!
//! Treat the shape as subject to change: a rule may be added, and a signature may
//! move, without a major version, because the feature is what carries the
//! exemption.
//!
//! # The invariant that drives the design
//!
//! A read model and its checkpoint must move together. If the read-model write
//! commits and the checkpoint write does not, a restart replays events that
//! were already applied; if the checkpoint commits first, a crash silently
//! skips events. Neither is acceptable, and no amount of ordering or retrying
//! fixes it — the two writes must be **one** transaction.
//!
//! So the port cannot offer `apply()` and `set_checkpoint()` as independent
//! calls. It hands out an adapter-owned batch and takes it back at commit time
//! together with the position.
//!
//! # What is out of scope at 0.1: a projection that writes back into the log
//!
//! A projection that emits events into the event store as a side effect of
//! applying one is **out of scope for this port at 0.1**, and this paragraph is
//! the port saying so rather than leaving it to be discovered
//! (`spec/SPECIFICATION.md` §4's PS-31).
//!
//! It is a consequence of a decision already taken, not a question still open.
//! An `EventId` is a store-assigned `(StoreId, SequencePosition)` pair minted at
//! append (VT-5), and [`EventStore::append`](crate::EventStore::append) refuses a
//! caller-supplied one (VT-10). An outward-writing projection needs exactly what
//! VT-10 refuses: a write whose identity the *caller* chooses, so that a rebuild
//! re-emitting the same event is a no-op rather than a second fact. Without that,
//! every rebuild duplicates every emitted event, which is the wrong
//! implementation silence here produces. Revisiting it means a deliberate
//! idempotent-emission seam, and VT-5 is what would make one expressible.
//!
//! # The batch is owned, and is not required to be a live transaction
//!
//! [`ProjectionStore::Batch`] is a plain associated type with **no lifetime
//! parameter**. It used to be a generic associated type borrowing from the
//! store, on the reasoning that a transaction cannot outlive its connection.
//! Two compiled results retired that shape and neither is about `Send`: the
//! `error[E0195]` every impl hit, written out once beside the doctest that
//! disposes of it in [`# Implementing it`](ProjectionStore#implementing-it),
//! and a store carrying a lifetime of its own, which made rustc 1.97.1 *ICE*
//! while reporting the region error the GAT's `where Self: 'a` produced. A
//! batch may still *be* a live transaction — the port stops requiring one,
//! which is what lets an adapter with no connection at all implement it.
//!
//! # The `Send` flavour's extra requirement is documented, not declared
//!
//! [`SendProjectionStore`] transitively requires `Batch: Send`, because the
//! batch crosses an await between [`begin`](ProjectionStore::begin) and
//! [`commit`](ProjectionStore::commit). That bound cannot be written on the
//! associated type: `trait_variant` copies associated-type bounds **verbatim**
//! into the bare flavour, so `type Batch: Send;` would impose `Send` on the
//! flavour that exists precisely for `wasm32`, where nothing is. A caller who
//! needs a spawnable runner writes `S::Batch: Send` in their own `where`
//! clause.
//!
//! Projections that cannot be made transactional with their checkpoint must
//! instead be made **idempotent**, so that replaying an event is harmless. That
//! is a property of the projection, not of this port.
//!
//! # Naming what the port hands back
//!
//! ```
//! use happenstance_core::{
//!     Authority, Checkpoint, CommitError, ProjectionId, ResetError, SequencePosition,
//! };
//!
//! // Every state the port can report is one a caller can name.
//! let checkpoint = Checkpoint::Live {
//!     through: SequencePosition::FIRST,
//! };
//! assert!(matches!(checkpoint, Checkpoint::Live { .. }));
//!
//! // `commit` and `reset` fail differently, so a caller matching one is never
//! // offered the other's arms.
//! let refused: ResetError<core::convert::Infallible> = ResetError::Refused;
//! let foreign: CommitError<core::convert::Infallible> = CommitError::ForeignBatch;
//! assert!(matches!(refused, ResetError::Refused));
//! assert!(matches!(foreign, CommitError::ForeignBatch));
//!
//! let _rebuild = (ProjectionId::new("van_stock"), Authority::Rebuilding);
//! ```

use alloc::boxed::Box;
use alloc::string::String;
#[cfg(feature = "conformance")]
use core::future::Future;

use crate::event::SequencePosition;

/// Names a read model within a projection store.
///
/// Distinct projections advance independently, so each needs its own
/// checkpoint.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProjectionId(Box<str>);

impl ProjectionId {
    /// Creates a projection identifier.
    ///
    /// **Infallible, and that is an open question rather than a decision.**
    /// Both sibling identifiers — [`EventType`](crate::EventType) and
    /// [`Tag`](crate::Tag) — validate and return a `Result`; this one accepts
    /// anything, including the empty string, and the value becomes the primary
    /// key of a checkpoint row.
    ///
    /// Do not read the inconsistency as a deliberate "opaque operator-chosen
    /// key" design. There is no decision behind it. It is left standing because
    /// `ProjectionId` belongs to [`ProjectionStore`], which is provisional, has
    /// no conformance suite, and is frozen at a later phase — and a validating
    /// constructor with nothing able to check it would be exactly the decorative
    /// rule this project's conformance discipline exists to prevent. Adding a
    /// fallible `parse` beside this constructor would be worse than either
    /// choice: two constructors enforcing different rules is the defect that
    /// makes an invalid value reachable through the weaker one.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into().into_boxed_str())
    }

    /// The identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl core::fmt::Display for ProjectionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Where a projection has been brought to, and whether its rows can be trusted.
///
/// An enum rather than `(Option<SequencePosition>, bool)` because the tuple can
/// spell `(None, true)` — authoritative, never run — which means nothing. That
/// is this crate's "illegal states are unrepresentable" line, applied where a
/// reader would otherwise write `if let Some(p) = checkpoint` and get it wrong.
/// A bare `Option<SequencePosition>`, the weaker alternative, cannot distinguish
/// a rebuild in flight from an authoritative read model at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Checkpoint {
    /// Never run, or reset, or rebuilding with nothing committed yet.
    ///
    /// Distinct from a checkpoint at the first position: an operator who
    /// substitutes `commit(empty_batch, id, FIRST, _)` for "never run" makes a
    /// runner resume *after* event 1, which is then skipped permanently and
    /// silently.
    NeverRun,

    /// Considered through `through`, and the read model is authoritative.
    Live {
        /// The position this projection has considered up to. A runner resumes
        /// strictly after it.
        through: SequencePosition,
    },

    /// A rebuild is in flight, considered through `through`. Rows are **not**
    /// authoritative.
    Rebuilding {
        /// The position the rebuild has considered up to.
        through: SequencePosition,
    },
}

/// What a commit claims about the rows it leaves behind.
///
/// Taken by value at [`commit`](ProjectionStore::commit) so the claim is made at
/// write time and [`checkpoint`](ProjectionStore::checkpoint) can report it
/// without inferring it. A rebuild in place therefore cannot be spelled as an
/// ordinary commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Authority {
    /// The rows this commit leaves behind are authoritative.
    Live,
    /// A rebuild is in flight; the rows are not yet authoritative.
    Rebuilding,
}

/// Why a [`commit`](ProjectionStore::commit) failed.
///
/// Generic over the adapter's own error rather than carrying a `String`, for the
/// same reason [`AppendError`](crate::AppendError) is: a caller must be able to
/// tell a port-level outcome from an adapter failure without pattern-matching on
/// text. `#[non_exhaustive]`, so a match needs a wildcard arm.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum CommitError<E> {
    /// The batch was begun on a different store instance.
    ///
    /// Rejected at run time rather than at compile time, and deliberately.
    /// Tying the batch to the receiver's lifetime was compiled and refuted: a
    /// lifetime names a *region*, not an *instance*, so two `&Store` references
    /// unify to a common region and `b.commit(a.begin(), …)` still type-checks.
    /// The only type-level fix is a generative brand, which forbids the batch
    /// escaping the closure that begins it — defeating the caller the hazard is
    /// about. So an adapter stamps an identity minted per store instance at
    /// [`begin`](ProjectionStore::begin) and compares it here; with an owned
    /// batch the stamp is a field and the check is an integer comparison.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,

    /// `position` is below the checkpoint already recorded.
    ///
    /// Both values are carried so a caller can log the gap rather than re-derive
    /// it with a second round trip.
    #[error("checkpoint regression: {current} is recorded, {attempted} was attempted")]
    CheckpointRegression {
        /// The checkpoint the store already holds.
        current: SequencePosition,
        /// The position the caller tried to move it to.
        attempted: SequencePosition,
    },

    /// The adapter failed for its own reasons.
    #[error(transparent)]
    Store(E),
}

/// Why a [`reset`](ProjectionStore::reset) failed.
///
/// Separate from [`CommitError`] so a caller matching `commit`'s result never
/// has to consider [`Refused`](Self::Refused), which `commit` cannot produce.
/// One merged `ProjectionError<E>` was the alternative and it lost:
/// `#[non_exhaustive]` already forces a wildcard arm without also forcing dead
/// ones.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum ResetError<E> {
    /// The batch was begun on a different store instance.
    ///
    /// See [`CommitError::ForeignBatch`] for why this is a run-time rejection.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,

    /// This store declines to reset this projection.
    ///
    /// The variant is bare on purpose. The port supplies the mechanism and the
    /// domain decides what to protect, so a `&'static str` here would push a
    /// domain sentence through a port type that cannot validate it, cannot
    /// localise it and cannot keep it in step with the policy that produced it.
    /// An operator learns *that* the reset was declined from this variant, and
    /// *which policy* declined it from the store that holds the policy. A
    /// refusal is not success: it leaves the rows and the checkpoint unchanged.
    #[error("this store declines to reset this projection")]
    Refused,

    /// The adapter failed for its own reasons.
    #[error(transparent)]
    Store(E),
}

/// A store that holds read models and their replay checkpoints.
///
/// See the [module documentation](self) for the transactional invariant this
/// shape exists to enforce, for why the batch is owned, and for its provisional
/// status.
///
/// Two traits, one set of doc attributes. `trait_variant` derives
/// [`SendProjectionStore`] from [`ProjectionStore`] and copies this block onto
/// it, so every sentence here is written to hold on whichever of the two pages
/// you opened. [`ProjectionStore`] states no `Send` requirement and is what
/// generic code binds; [`SendProjectionStore`] adds it and hands back
/// [`ProjectionStore`] free. The same split as
/// [`EventStore`](crate::EventStore), for the same reason.
///
/// # Implementing it
///
/// The whole port, on a store small enough to read at a glance. Note what is
/// *absent*: there is no `Self::Batch<'_>` anywhere, because there is no
/// lifetime to spell. An implementer writes `type Batch = MyBatch;` and then
/// `async fn commit(&self, batch: MyBatch, …)`, and it compiles.
///
/// That used to be the trap that explained why this port had no adapters. While
/// `Batch` was a generic associated type, the trait declared
/// `batch: Self::Batch<'_>`, so naming the concrete type in the impl declared a
/// different set of lifetime generics and rustc answered
/// `error[E0195]: lifetime parameters or bounds on method 'commit' do not match
/// the trait declaration` — with nothing in the workspace saying the literal
/// `Self::Batch<'_>` was required, and nothing to copy. This example is the
/// disposition of that trap: it compiles on every CI run, so it cannot rot back.
///
/// ```
/// use happenstance_core::{
///     Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError,
///     SequencePosition,
/// };
///
/// /// The adapter's own batch. Owned, and not a live transaction.
/// #[derive(Debug, Default)]
/// struct ToyBatch {
///     rows: Vec<(String, u64)>,
/// }
///
/// #[derive(Debug, Default)]
/// struct ToyStore {
///     committed: std::cell::RefCell<Vec<(String, u64)>>,
///     checkpoint: std::cell::Cell<Option<SequencePosition>>,
/// }
///
/// impl ProjectionStore for ToyStore {
///     type Error = ToyError;
///
///     // No lifetime, and no `where Self: 'a`.
///     type Batch = ToyBatch;
///
///     // Not `async`, and not fallible: opening a buffer cannot fail.
///     fn begin(&self) -> ToyBatch {
///         ToyBatch::default()
///     }
///
///     async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, ToyError> {
///         Ok(match self.checkpoint.get() {
///             None => Checkpoint::NeverRun,
///             Some(through) => Checkpoint::Live { through },
///         })
///     }
///
///     // The concrete type, spelled straight out. This is the line that used
///     // to be `error[E0195]`.
///     async fn commit(
///         &self,
///         batch: ToyBatch,
///         _id: &ProjectionId,
///         position: SequencePosition,
///         _authority: Authority,
///     ) -> Result<(), CommitError<ToyError>> {
///         // The rows and the checkpoint, or neither.
///         self.committed.borrow_mut().extend(batch.rows);
///         self.checkpoint.set(Some(position));
///         Ok(())
///     }
///
///     async fn reset(
///         &self,
///         batch: ToyBatch,
///         _id: &ProjectionId,
///     ) -> Result<(), ResetError<ToyError>> {
///         // The caller's batch carries the deletes; this store's whole read
///         // model is the vector, so clearing it is the same unit of work.
///         drop(batch);
///         self.committed.borrow_mut().clear();
///         self.checkpoint.set(None);
///         Ok(())
///     }
///
///     async fn rollback(&self, batch: ToyBatch) -> Result<(), ToyError> {
///         drop(batch);
///         Ok(())
///     }
/// }
///
/// #[derive(Debug)]
/// struct ToyError;
/// impl core::fmt::Display for ToyError {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
///         f.write_str("the toy store failed")
///     }
/// }
/// impl core::error::Error for ToyError {}
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = ToyStore::default();
/// let id = ProjectionId::new("toy");
///
/// let mut batch = store.begin();
/// batch.rows.push(("depot-7".to_owned(), 12));
/// store
///     .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
///     .await?;
///
/// assert_eq!(
///     store.checkpoint(&id).await?,
///     Checkpoint::Live { through: SequencePosition::FIRST },
/// );
/// # Ok(())
/// # }
/// ```
#[trait_variant::make(SendProjectionStore: Send)]
pub trait ProjectionStore {
    /// How this adapter fails.
    type Error: core::error::Error + 'static;

    /// The adapter's write set.
    ///
    /// **Owned, and not required to be a live transaction.** Only the adapter
    /// knows what it is: a SQL transaction, a buffered statement list, a graph
    /// write set. Applying an event mutates it; it becomes durable only when
    /// handed to [`commit`](ProjectionStore::commit) or
    /// [`reset`](ProjectionStore::reset).
    ///
    /// No `Send` bound is written here, and it is not an oversight.
    /// [`SendProjectionStore`] transitively requires `Batch: Send`, but
    /// `trait_variant` copies associated-type bounds verbatim into the bare
    /// flavour — so `type Batch: Send;` would impose `Send` on the flavour that
    /// exists for `wasm32`, where nothing is. The requirement is documented
    /// here and written by the caller as `S::Batch: Send` when they need it.
    type Batch;

    /// Opens a write set.
    ///
    /// Neither `async` nor fallible, and this is not an asymmetry with
    /// [`EventStore`](crate::EventStore) waiting to be tidied away. An
    /// `async fn begin() -> Result<…>` *implies a round trip*, which an adapter
    /// on a one-shot HTTP transport with no connection and no cursor cannot
    /// afford and does not need: opening a buffer cannot fail, and an adapter
    /// that really must talk to its server takes that trip at `commit`.
    fn begin(&self) -> Self::Batch;

    /// How far this projection has been brought, and whether its rows are
    /// authoritative.
    ///
    /// Returns a [`Checkpoint`], never an `Option<SequencePosition>`; the
    /// reason the tuple form lost is recorded on that type. Resume a replay by
    /// feeding `through` to [`ReadOptions::from`](crate::ReadOptions::from)
    /// after advancing past it.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the checkpoint cannot be read.
    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error>;

    /// Applies `batch` and moves `id`'s checkpoint to `position`, as one unit.
    ///
    /// `position` is the position *considered*, not the position applied, so a
    /// batch that wrote nothing still advances the checkpoint. `authority` is
    /// the claim this commit makes about the rows it leaves behind.
    ///
    /// # Errors
    ///
    /// Returns [`CommitError::ForeignBatch`] if `batch` was begun on a different
    /// store instance, [`CommitError::CheckpointRegression`] if `position` is
    /// below the checkpoint already recorded, and [`CommitError::Store`] if the
    /// adapter itself failed. The batch is consumed either way, and a failed
    /// commit leaves both the read model and the checkpoint unchanged.
    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>>;

    /// Applies `batch` and returns `id` to [`Checkpoint::NeverRun`], as one
    /// unit.
    ///
    /// The dual of [`commit`](ProjectionStore::commit): the caller's own batch
    /// carries the deletes, because **the port has no idea what the read model
    /// is**. The alternative — a `reset` that clears the rows itself — would
    /// oblige the adapter to know which tables belong to a [`ProjectionId`],
    /// which is exactly the knowledge this port keeps out.
    ///
    /// # Errors
    ///
    /// Returns [`ResetError::ForeignBatch`] if `batch` was begun on a different
    /// store instance, [`ResetError::Refused`] if this store declines to reset
    /// this projection, and [`ResetError::Store`] if the adapter itself failed.
    /// A refusal, like a failure, leaves both halves unchanged.
    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>>;

    /// Discards the batch without committing.
    ///
    /// Exists because `Drop` cannot await: an adapter holding a real resource
    /// needs somewhere to release it. Dropping a batch bare must also roll back
    /// and leave the store usable.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the rollback fails.
    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error>;
}

/// The write seam the projection conformance suite runs through.
///
/// Without it, generic code holding an adapter's batch can do exactly two things
/// with it — commit it or roll it back. There is no way to put a row *into* the
/// batch and no way to look at the read model afterwards, so the rule carrying
/// this port's entire reason for existing — the read-model write and the
/// checkpoint write become durable together or not at all — cannot be written,
/// and a suite built on [`ProjectionStore`] alone degenerates into a checkpoint
/// test that a store writing *only* checkpoints passes.
///
/// An adapter that wants to be certified implements this beside its
/// [`ProjectionStore`] impl. It is not part of the runtime surface: it exists so
/// a suite that has never heard of the adapter can drive its read model.
///
/// # Why this lives in the contract crate rather than the testkit
///
/// It looks like a testing utility and belongs beside the port anyway, and the
/// reason is coherence rather than taste. An adapter crate implementing a
/// *testkit* trait for its own type is legal — the type is local, so the orphan
/// rule is satisfied. But the natural place to write such an impl is the
/// adapter's own `tests/` directory, and **that is a different crate**. There,
/// neither the trait nor the type is local, `impl ProjectionProbe for MyStore`
/// is rejected by the orphan rule, and the only way out is a **non-dev**
/// dependency on the testkit, feature-gated. Putting the trait here keeps the
/// impl in the adapter's own `src/`, beside the store and behind the adapter's
/// own feature, and costs one flag on a dependency it already has rather than a
/// new edge in its graph:
///
/// ```toml
/// [dependencies]
/// # `unstable-projection`, not optionally: the `impl ProjectionStore` below
/// # is unconditional in the adapter's `src/`, and every item it names lives
/// # behind that feature, which is not in the default set. Forwarding it
/// # through the adapter's own `[features]` the way `conformance` is forwarded
/// # would leave the port impl unable to compile without a flag no consumer
/// # would know to pass.
/// happenstance-core = { version = "…", features = ["unstable-projection"] }
///
/// [features]
/// # Forwards to the contract crate. The `impl ProjectionProbe` lives in `src/`
/// # under `#[cfg(feature = "conformance")]`, because a crate cannot `cfg` on a
/// # dependency's feature — which is also why a `[dev-dependencies]` entry
/// # would not work: it does not exist for the lib build the impl compiles in.
/// conformance = ["happenstance-core/conformance"]
///
/// [dev-dependencies]
/// happenstance-testkit = "…"
/// ```
///
/// **Nothing inside this workspace can fail the wrong version of that
/// decision.** Every fixture here already lives in a crate that depends on the
/// testkit, so the trait would be local, the impls local, the orphan rule
/// silent, and the whole gate green. The falsifier is
/// `documented-extension-surface` (HS-S0015), which builds an outside author's
/// fixture from this documentation alone — named here so the placement is not
/// "simplified" into the testkit on grounds of diff size before it runs.
///
/// # Both flavours, one trait
///
/// The bound is bare [`ProjectionStore`], and there is deliberately no
/// `SendProjectionProbe`. `trait_variant` emits a blanket impl, so
/// [`SendProjectionStore`] implies [`ProjectionStore`] and a `Send` adapter
/// already satisfies this supertrait bound. A second trait would collide with
/// that blanket impl — `error[E0275]` — which is the shape ADR-0008 records.
#[cfg(feature = "conformance")]
pub trait ProjectionProbe: ProjectionStore {
    /// Whether this adapter offers any read path on an open batch.
    ///
    /// Declaring `false` is a conformant answer, not a failure: many adapters
    /// buffer their writes and cannot read them back before commit. A store
    /// declaring `false` still has the read-through rule **emitted as a
    /// reported skip carrying its reason** — never omitted — because a rule
    /// absent from the binary is indistinguishable in CI output from a rule
    /// that passed.
    const READS_THROUGH_BATCH: bool;

    /// Writes one probe row into an open batch.
    ///
    /// Infallible and synchronous: it mutates a batch the caller owns, which
    /// cannot fail. Nothing becomes durable until the batch reaches
    /// [`commit`](ProjectionStore::commit).
    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64);

    /// Queues deletion of every probe row into an open batch.
    ///
    /// Exists so [`reset`](ProjectionStore::reset) is checkable *without the
    /// suite knowing what a read model is*. `reset` takes the caller's own
    /// deletes, so a suite with no way to express "delete everything" cannot
    /// exercise it at all.
    fn probe_delete_all(&self, batch: &mut Self::Batch);

    /// Reads one probe row from the **committed** read model.
    ///
    /// The one asynchronous member, and not arbitrarily so: the other three
    /// mutate a batch the caller already holds, while this is real I/O against
    /// the store.
    ///
    /// Spelled `-> impl Future<…>` rather than `async fn`, and with **no
    /// `+ Send`**. This trait is not under `#[trait_variant::make]`, so `async
    /// fn` here would fire `async_fn_in_trait` under the gate's `-D warnings`;
    /// writing the desugaring by hand also puts the *absence* of the `Send`
    /// bound at the declaration, where a reader can see it. A `Send` bound
    /// would break `wasm32` and could not be relaxed later without a breaking
    /// change — and it is not needed, because suite code binds the weaker
    /// flavour and takes its per-test wrapper as a parameter.
    ///
    /// # Errors
    ///
    /// Returns the adapter's error if the read model cannot be read.
    fn probe_read(&self, key: &str) -> impl Future<Output = Result<Option<u64>, Self::Error>>;

    /// Reads one probe row through an **open** batch, committed state included.
    ///
    /// Only called when [`READS_THROUGH_BATCH`](Self::READS_THROUGH_BATCH) is
    /// `true`; may be `unimplemented!()` otherwise.
    ///
    /// # This signature cannot be met by a batch that is a live transaction
    ///
    /// Stated here because the freeze decision reads this page and would
    /// otherwise read it as scarcity. Synchronous, infallible and `&Self::Batch`
    /// is answerable by a store that holds its pending writes in a map or a
    /// buffer — which is every implementation in this workspace — and is not
    /// answerable at all by one whose batch *is* an open transaction, because a
    /// driver borrows the connection mutably to issue a statement and the
    /// statement is I/O.
    ///
    /// Both halves are compiler-checked below rather than asserted. The mutable
    /// borrow first:
    ///
    /// ```compile_fail,E0596
    /// /// `sqlx`'s `Executor for &mut Transaction`, and `rusqlite`'s `&mut
    /// /// Transaction`, in the smallest shape that carries the obligation.
    /// struct Transaction;
    /// impl Transaction {
    ///     fn select(&mut self, key: &str) -> Option<u64> {
    ///         let _ = key;
    ///         None
    ///     }
    /// }
    ///
    /// // error[E0596]: cannot borrow `*batch` as mutable, as it is behind a
    /// // `&` reference — which is the receiver `probe_read_through` supplies.
    /// fn probe_read_through(batch: &Transaction, key: &str) -> Option<u64> {
    ///     batch.select(key)
    /// }
    /// ```
    ///
    /// and then the await, with the borrow already conceded:
    ///
    /// ```compile_fail,E0728
    /// struct Transaction;
    /// impl Transaction {
    ///     async fn select(&mut self, key: &str) -> Option<u64> {
    ///         let _ = key;
    ///         None
    ///     }
    /// }
    ///
    /// // error[E0728]: `await` is only allowed inside `async` functions and
    /// // blocks — and this method is not one.
    /// fn probe_read_through(batch: &mut Transaction, key: &str) -> Option<u64> {
    ///     batch.select(key).await
    /// }
    /// ```
    ///
    /// The three bodies that remain are each wrong in a different way, and
    /// `tests/probe_live_transaction_shape.rs` runs all three against a store
    /// whose batch is a transaction: declaring `READS_THROUGH_BATCH = false`
    /// states something false about the store and takes the read-through rule as
    /// a reported skip; answering from committed state returns `None` for a row
    /// the transaction can see, which is what this method's own first sentence
    /// forbids; and blocking on the future panics with *"Cannot start a runtime
    /// from within a runtime"*, because the suite always calls the probe from
    /// inside one.
    ///
    /// **Nothing is decided here.** `spec/SPECIFICATION.md` §4's PS-2 is
    /// `[FROZEN]` and names a live-transaction adapter as the end of the
    /// batch-shape axis still to be built; whether this signature moves before
    /// that adapter is written belongs to that clause's owner, not to this
    /// method's documentation. What is recorded is that that end's absence has
    /// a cause in this line, and not only in nobody having got to it.
    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64>;
}

#[cfg(test)]
mod tests {
    // Local override of the workspace `unwrap_used = "deny"`, as the house style
    // permits for test modules.
    #![allow(clippy::unwrap_used)]

    use alloc::rc::Rc;

    use super::{Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError};
    use crate::event::SequencePosition;

    /// The witness store's error. Real, so `Self::Error`'s bound is discharged
    /// by something other than `!`.
    #[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
    #[error("the witness store failed")]
    struct WitnessError;

    /// An owned batch that is deliberately **not** `Send`.
    ///
    /// The `Rc` is the assertion. `trait_variant` copies associated-type bounds
    /// verbatim into the bare flavour, so writing `type Batch: Send;` on the
    /// trait would make this struct an illegal `Batch` — and would break the
    /// `wasm32` target the bare flavour exists for (PS-36).
    #[derive(Debug, Default)]
    struct WitnessBatch {
        writes: usize,
        not_send: Rc<()>,
    }

    /// A zero-sized witness that the port is *implementable*.
    ///
    /// Not a store worth shipping and not a step towards one:
    /// `MemoryProjectionStore` is a separate deliverable, behind the `memory`
    /// feature, with a doctest and real state. This is the smallest thing that
    /// makes AC-001, AC-004 and AC-005 tests rather than assertions about text.
    #[derive(Debug, Default)]
    struct Witness;

    impl ProjectionStore for Witness {
        type Error = WitnessError;

        type Batch = WitnessBatch;

        fn begin(&self) -> Self::Batch {
            WitnessBatch::default()
        }

        async fn checkpoint(&self, _id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
            Ok(Checkpoint::NeverRun)
        }

        async fn commit(
            &self,
            batch: Self::Batch,
            _id: &ProjectionId,
            _position: SequencePosition,
            _authority: Authority,
        ) -> Result<(), CommitError<Self::Error>> {
            drop(batch);
            Ok(())
        }

        async fn reset(
            &self,
            batch: Self::Batch,
            _id: &ProjectionId,
        ) -> Result<(), ResetError<Self::Error>> {
            drop(batch);
            Ok(())
        }

        async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
            drop(batch);
            Ok(())
        }
    }

    /// AC-001. All six items, implemented and driven.
    #[tokio::test]
    async fn the_port_is_implementable_with_an_owned_batch() {
        let store = Witness;
        let id = ProjectionId::new("witness");

        assert_eq!(store.checkpoint(&id).await.unwrap(), Checkpoint::NeverRun);

        let mut batch = store.begin();
        batch.writes += 1;
        store
            .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
            .await
            .unwrap();

        store.reset(store.begin(), &id).await.unwrap();
        store.rollback(store.begin()).await.unwrap();
    }

    /// AC-002. No `_` arm: a fourth state fails this test rather than widening
    /// it silently.
    #[test]
    fn checkpoint_names_every_state_and_no_others() {
        fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
            match checkpoint {
                Checkpoint::NeverRun => None,
                Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
            }
        }

        fn is_authoritative(checkpoint: Checkpoint) -> bool {
            match checkpoint {
                Checkpoint::Live { .. } => true,
                Checkpoint::NeverRun | Checkpoint::Rebuilding { .. } => false,
            }
        }

        assert_eq!(considered_through(Checkpoint::NeverRun), None);
        assert_eq!(
            considered_through(Checkpoint::Live {
                through: SequencePosition::FIRST
            }),
            Some(SequencePosition::FIRST)
        );
        assert_eq!(
            considered_through(Checkpoint::Rebuilding {
                through: SequencePosition::FIRST
            }),
            Some(SequencePosition::FIRST)
        );

        // The pair `(None, true)` the tuple could spell has no counterpart
        // here: every variant that is authoritative also carries a position.
        assert!(!is_authoritative(Checkpoint::NeverRun));
        assert!(is_authoritative(Checkpoint::Live {
            through: SequencePosition::FIRST
        }));
        assert!(!is_authoritative(Checkpoint::Rebuilding {
            through: SequencePosition::FIRST
        }));
    }

    /// AC-003. Two enums, each exhaustively matchable inside this crate over
    /// exactly the arms its own operation can produce.
    #[test]
    fn commit_and_reset_errors_stay_separate() {
        fn commit_arm(error: &CommitError<WitnessError>) -> &'static str {
            match error {
                CommitError::ForeignBatch => "foreign",
                CommitError::CheckpointRegression { .. } => "regression",
                CommitError::Store(_) => "store",
            }
        }

        fn reset_arm(error: &ResetError<WitnessError>) -> &'static str {
            match error {
                ResetError::ForeignBatch => "foreign",
                ResetError::Refused => "refused",
                ResetError::Store(_) => "store",
            }
        }

        assert_eq!(commit_arm(&CommitError::ForeignBatch), "foreign");
        assert_eq!(
            commit_arm(&CommitError::CheckpointRegression {
                current: SequencePosition::FIRST,
                attempted: SequencePosition::FIRST,
            }),
            "regression"
        );
        assert_eq!(reset_arm(&ResetError::ForeignBatch), "foreign");
        assert_eq!(reset_arm(&ResetError::Refused), "refused");

        // The payload is the adapter's own error, recovered by value — not a
        // `String`, and not a `Box<dyn Error>` that has lost its type.
        let recovered = match CommitError::Store(WitnessError) {
            CommitError::Store(error) => error,
            other => panic!("expected Store, got {other:?}"),
        };
        assert_eq!(recovered, WitnessError);
    }

    /// AC-004. No `.await` and no `?`: making `begin` async or fallible is a
    /// compile error here rather than a review miss.
    #[test]
    fn begin_is_neither_async_nor_fallible() {
        let store = Witness;
        let batch: <Witness as ProjectionStore>::Batch = store.begin();
        assert_eq!(batch.writes, 0);
        assert_eq!(Rc::strong_count(&batch.not_send), 1);
    }

    /// AC-005. The bare flavour accepts a `!Send` batch, which is what
    /// `type Batch: Send;` would forbid.
    #[test]
    fn a_non_send_batch_still_implements_the_bare_flavour() {
        fn accepts_the_bare_flavour<P: ProjectionStore>() {}
        accepts_the_bare_flavour::<Witness>();
    }
}

/// The trait-level doc block, read as text because it is published on **two**
/// pages.
///
/// The same mechanism as `store.rs`'s module of this name, and the same defect:
/// `#[trait_variant::make(SendProjectionStore: Send)]` rebuilds the derived trait
/// with `..tr.clone()`, so every `///` line above the derivation is rendered
/// verbatim on `SendProjectionStore`'s page too. `SendProjectionStore` has no doc
/// comment of its own, so the copying is load-bearing — `missing_docs` is what
/// would notice it stopping — and the sentences therefore have to be true on
/// whichever page a reader opened.
///
/// The extractor is duplicated from `store.rs` rather than shared: sharing it
/// would put a test-only module in the crate root, which is the file
/// `happenstance`'s `contract_surface.rs` derives every gate from.
#[cfg(test)]
mod derived_flavour_doc {
    #![allow(clippy::unwrap_used, reason = "test code, per the house style")]

    use alloc::string::String;
    use alloc::vec::Vec;

    /// This file's own source. The doc block is the deliverable, so it is read
    /// rather than trusted.
    const SOURCE: &str = include_str!("projection.rs");

    /// The derivation whose expansion copies the block above it onto a second page.
    const DERIVATION: &str = "#[trait_variant::make(SendProjectionStore: Send)]";

    /// The two flavours, in the link form a reader can click from either page.
    const FLAVOURS: [&str; 2] = ["[`ProjectionStore`]", "[`SendProjectionStore`]"];

    /// The `///` lines the derivation copies, marker removed and fences dropped.
    fn copied_prose() -> Vec<&'static str> {
        let lines: Vec<&str> = SOURCE.lines().collect();
        let make = lines
            .iter()
            .position(|line| line.trim_end() == DERIVATION)
            .expect("the derivation that copies this block onto the second page");
        let start = lines[..make]
            .iter()
            .rposition(|line| {
                !(line.starts_with("///") || line.starts_with("//") || line.starts_with("#["))
            })
            .map_or(0, |index| index + 1);
        let mut fenced = false;
        lines[start..make]
            .iter()
            .filter_map(|line| {
                let text = line.strip_prefix("///")?;
                let text = text.strip_prefix(' ').unwrap_or(text);
                if text.trim_start().starts_with("```") {
                    fenced = !fenced;
                    return None;
                }
                if fenced { None } else { Some(text) }
            })
            .collect()
    }

    /// The copied prose in blank-line-separated paragraphs.
    fn paragraphs() -> Vec<String> {
        copied_prose()
            .split(|line| line.is_empty())
            .filter(|block| !block.is_empty())
            .map(|block| block.join(" "))
            .collect()
    }

    /// A paragraph that distinguishes the flavours names them; it does not point.
    #[test]
    fn no_paragraph_tells_the_flavours_apart_by_deixis() {
        /// Pointers that resolve against the page rather than against a name.
        const DEIXIS: [&str; 6] = [
            "this trait",
            "this is",
            "this flavour",
            "this one",
            "the one to use",
            "instead",
        ];
        for paragraph in paragraphs() {
            let lower = paragraph.to_lowercase();
            if !lower.contains("send") {
                continue;
            }
            for pointer in DEIXIS {
                assert!(
                    !lower.contains(pointer),
                    "{DERIVATION} copies this paragraph verbatim onto \
                     `SendProjectionStore`, where {pointer:?} points at the wrong \
                     trait. Name the flavour: {paragraph}"
                );
            }
        }
    }

    /// Both flavours are named, in link form, in the prose a reader lands on.
    #[test]
    fn the_block_names_both_flavours_in_link_form() {
        let prose = copied_prose().join("\n");
        for flavour in FLAVOURS {
            assert!(
                prose.contains(flavour),
                "the block is rendered on both pages, so it must name {flavour} \
                 rather than leave a reader to infer which trait they are on. \
                 Prose as read:\n{prose}"
            );
        }
    }
}
