//! An in-memory reference projection store.

use alloc::string::{String, ToString};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{PoisonError, RwLock};

use crate::event::SequencePosition;
use crate::projection::{
    Authority, Checkpoint, CommitError, ProjectionId, ResetError, SendProjectionStore,
};

/// The reference implementation of [`ProjectionStore`](crate::ProjectionStore).
///
/// It exists for the same three reasons
/// [`MemoryEventStore`](crate::MemoryEventStore) does, in the same order of
/// importance:
///
/// 1. it is the oracle the conformance suite is validated against, so that a
///    failing adapter is known to be the adapter's fault and not the suite's;
/// 2. it makes this crate's examples runnable, so the documentation cannot
///    drift from the API;
/// 3. it lets application code be written and tested before any real adapter
///    exists.
///
/// It is deliberately **obvious rather than fast**. When a conformance rule
/// fails, the adapter is presumed wrong and this store is presumed right, so its
/// correctness has to be readable in one sitting: one lock over one struct
/// holding both halves, and [`commit`](crate::ProjectionStore::commit) takes
/// that lock exactly once. Splitting it into more than one guarded section is
/// precisely the shape a store that writes the checkpoint without its read model
/// fails for.
///
/// # One end of the batch-shape axis
///
/// This store **applies on write**: a batch is a materialised delta layered over
/// committed state, so it can be read back through before it commits, and
/// `ProjectionProbe::READS_THROUGH_BATCH` is `true` here. The other end of that
/// axis — a batch that buffers and replays at commit, where read-through is
/// impossible and declining it is the conformant answer — is a separate
/// implementation. Nothing a conformance rule needs may assume read-through
/// beyond that constant.
///
/// The probe's name is deliberately not a link, for the reason the crate root
/// gives twice: this page renders whenever `memory` is on, `ProjectionProbe`
/// exists only under `conformance`, and `rustdoc::broken_intra_doc_links` is
/// `deny` — so a link here is a hard error on the crate's **default** feature
/// set, which is what a consumer builds.
///
/// # Examples
///
/// The whole loop: open a batch, write through it, commit the rows and the
/// checkpoint as one unit, read both back, then return the projection to
/// [`Checkpoint::NeverRun`] with the caller's own deletes.
///
/// ```
/// use happenstance_core::{
///     Authority, Checkpoint, MemoryProjectionStore, ProjectionId, ProjectionStore,
///     SequencePosition,
/// };
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// let store = MemoryProjectionStore::new();
/// let id = ProjectionId::new("van_stock");
///
/// // Never run: the enum says so, and no `Option` is involved.
/// assert_eq!(store.checkpoint(&id).await?, Checkpoint::NeverRun);
///
/// // `begin` is neither async nor fallible: opening a buffer cannot fail.
/// let mut batch = store.begin();
/// batch.write("depot-7", 12);
///
/// // Nothing is visible yet — not the row, not the checkpoint.
/// assert_eq!(store.get("depot-7"), None);
/// assert_eq!(store.checkpoint(&id).await?, Checkpoint::NeverRun);
///
/// // One unit of work: the row and the checkpoint, or neither.
/// store
///     .commit(batch, &id, SequencePosition::FIRST, Authority::Live)
///     .await?;
///
/// assert_eq!(store.get("depot-7"), Some(12));
/// assert_eq!(
///     store.checkpoint(&id).await?,
///     Checkpoint::Live { through: SequencePosition::FIRST },
/// );
///
/// // The dual: the caller's own batch carries the deletes, because the port
/// // has no idea what the read model is.
/// let mut clearing = store.begin();
/// clearing.delete_all();
/// store.reset(clearing, &id).await?;
///
/// assert_eq!(store.get("depot-7"), None);
/// assert_eq!(store.checkpoint(&id).await?, Checkpoint::NeverRun);
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct MemoryProjectionStore {
    state: RwLock<State>,
    /// Minted per store *instance*, which is what makes the foreign-batch check
    /// possible at all — a per-type stamp would compare equal everywhere.
    stamp: u64,
}

/// Hand-written, and **not** `#[derive(Default)]`, for the same reason
/// [`MemoryEventStore`](crate::MemoryEventStore)'s is hand-written: `stamp` is
/// this instance's identity, and the derive would fill it with `0` without ever
/// calling the counter, which starts at `1`. Two `default()` stores would then
/// share identity `0`, each would accept the other's batch, and
/// [`CommitError::ForeignBatch`] — the check the oracle exists to be *right*
/// about — would be unreachable through that constructor.
/// `commit_rejects_a_foreign_batch_from_default_stores` and its `reset` twin in
/// `tests/projection_memory.rs` are what keep the derive from coming back.
impl Default for MemoryProjectionStore {
    fn default() -> Self {
        Self::new()
    }
}

/// The read model and the checkpoints, behind **one** lock.
///
/// Keeping them behind one lock is what makes "installed under one lock
/// acquisition" true by construction rather than by discipline. Two locks would
/// compile, pass every test that reads them separately, and reintroduce the
/// window the port exists to close.
#[derive(Debug, Default)]
struct State {
    rows: BTreeMap<String, u64>,
    checkpoints: BTreeMap<String, Checkpoint>,
}

impl MemoryProjectionStore {
    /// A store with no rows and no checkpoints.
    #[must_use]
    pub fn new() -> Self {
        Self {
            state: RwLock::new(State::default()),
            stamp: next_stamp(),
        }
    }

    /// Opens a batch. The inherent twin of
    /// [`begin`](crate::ProjectionStore::begin), so an application author can
    /// drive this store without importing the port.
    #[must_use]
    pub fn open(&self) -> MemoryProjectionBatch {
        MemoryProjectionBatch {
            stamp: self.stamp,
            writes: BTreeMap::new(),
            clear_all: false,
        }
    }

    /// One committed row, or `None`.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<u64> {
        self.read_guard().rows.get(key).copied()
    }

    /// Every committed row, in key order.
    #[must_use]
    pub fn snapshot(&self) -> BTreeMap<String, u64> {
        self.read_guard().rows.clone()
    }

    /// How many committed rows the read model holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.read_guard().rows.len()
    }

    /// Whether the read model holds no committed rows.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.read_guard().rows.is_empty()
    }

    /// Reads through the lock, recovering from poisoning.
    ///
    /// A panic in another thread while the lock was held cannot have left this
    /// store inconsistent: every mutation happens in one guarded section and the
    /// validation preceding it does not mutate. So poisoning carries no
    /// information, and propagating it would surface as a spurious `Err` from
    /// the *oracle* — a rule failure attributed to the adapter under test.
    fn read_guard(&self) -> std::sync::RwLockReadGuard<'_, State> {
        self.state.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// Writes through the lock, recovering from poisoning. See
    /// [`read_guard`](Self::read_guard).
    fn write_guard(&self) -> std::sync::RwLockWriteGuard<'_, State> {
        self.state.write().unwrap_or_else(PoisonError::into_inner)
    }
}

/// An in-flight write against a [`MemoryProjectionStore`].
///
/// Owned, and not a live transaction: it holds a materialised delta plus the
/// stamp of the store instance that minted it. Nothing it holds is visible
/// through the store until [`commit`](crate::ProjectionStore::commit) or
/// [`reset`](crate::ProjectionStore::reset) takes it.
///
/// Dropping it discards it and leaves the store usable — there is nothing to
/// release, because nothing was ever handed to the store.
#[derive(Debug, Clone)]
pub struct MemoryProjectionBatch {
    stamp: u64,
    writes: BTreeMap<String, u64>,
    clear_all: bool,
}

impl MemoryProjectionBatch {
    /// Queues a row.
    pub fn write(&mut self, key: impl Into<String>, value: u64) {
        self.writes.insert(key.into(), value);
    }

    /// Queues removal of every row in the read model.
    ///
    /// This is what a caller puts in the batch they hand to
    /// [`reset`](crate::ProjectionStore::reset): the port never learns which
    /// rows are the read model, so the caller supplies the deletes.
    pub fn delete_all(&mut self) {
        self.writes.clear();
        self.clear_all = true;
    }

    /// Reads a row through the open batch — pending writes layered over
    /// `committed`.
    ///
    /// Gated on `conformance` because its only caller is, and a private method
    /// whose caller is behind a feature is dead code in every build that does
    /// not ask for it. That combination is reachable and ordinary — `default,
    /// unstable-projection` without `conformance` is what an application using
    /// the projection runner resolves — and it failed `RUSTFLAGS="-D warnings"`
    /// with *method `read_through` is never used*.
    ///
    /// **Not `#[expect(dead_code)]`**, which would be actively wrong: `expect`
    /// fires when the lint it names does *not*, so it would be correct in the
    /// builds where the method is dead and an `unfulfilled_lint_expectation`
    /// warning in the `conformance` builds where it is used. `#[allow]` would
    /// work and loses on two counts — it keeps compiling the body into every
    /// consumer's build, and it silences the next method to go genuinely dead
    /// here. A `cfg` that matches the caller's says the true thing.
    #[cfg(feature = "conformance")]
    fn read_through(&self, committed: Option<u64>, key: &str) -> Option<u64> {
        if let Some(pending) = self.writes.get(key) {
            return Some(*pending);
        }
        if self.clear_all {
            return None;
        }
        committed
    }

    /// Applies this batch to `rows`.
    fn apply_to(self, rows: &mut BTreeMap<String, u64>) {
        if self.clear_all {
            rows.clear();
        }
        rows.extend(self.writes);
    }
}

/// [`MemoryProjectionStore`]'s error type, which is uninhabited.
///
/// An uninhabited error is worth having: it proves the contract does not
/// *require* a fallible read path, and it documents at the type level that this
/// store has no failure modes of its own. Populating it would delete that claim.
///
/// Nothing the conformance suite needs is lost.
/// [`CommitError`] and [`ResetError`] are still meaningfully fallible here — a
/// foreign batch and a regressing position are both reachable — and the
/// adapter-failure paths a suite must also exercise come from hostile stores
/// written for that purpose, never from the oracle.
///
/// Distinct from [`MemoryStoreError`](crate::MemoryStoreError), whose `Display`
/// names the event store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("unreachable: the in-memory projection store cannot fail")]
pub enum MemoryProjectionStoreError {}

impl SendProjectionStore for MemoryProjectionStore {
    type Error = MemoryProjectionStoreError;

    // The concrete type, spelled straight out. Under the GAT this line was
    // `type Batch<'a> = MemoryProjectionBatch;` and the methods below still had
    // to say `Self::Batch<'_>` literally or hit `error[E0195]`. There is no
    // lifetime left to mismatch, so an implementer copying this writes their own
    // type in both places and it compiles.
    type Batch = MemoryProjectionBatch;

    fn begin(&self) -> Self::Batch {
        self.open()
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        Ok(self
            .read_guard()
            .checkpoints
            .get(id.as_str())
            .copied()
            .unwrap_or(Checkpoint::NeverRun))
    }

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

        // One guard, held across the check and both writes. Everything below is
        // infallible, so there is no path that mutates and then fails.
        let mut state = self.write_guard();

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

        batch.apply_to(&mut state.rows);
        state.checkpoints.insert(id.to_string(), checkpoint);
        Ok(())
    }

    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        if batch.stamp != self.stamp {
            return Err(ResetError::ForeignBatch);
        }

        // This store has no protection policy, so it never returns `Refused`.
        // That is not a gap: a rule needing a refusal is served by a store
        // written to refuse, and an oracle that refused arbitrarily would be a
        // worse oracle.
        let mut state = self.write_guard();

        batch.apply_to(&mut state.rows);
        // *Removing* the key is what returns the projection to `NeverRun`.
        // Writing a sentinel position would reintroduce exactly the ambiguity
        // the three-variant enum exists to forbid.
        state.checkpoints.remove(id.as_str());
        Ok(())
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        drop(batch);
        Ok(())
    }
}

#[cfg(feature = "conformance")]
impl crate::projection::ProjectionProbe for MemoryProjectionStore {
    /// `true`, because this store applies on write and a batch can therefore be
    /// read back through before it commits.
    const READS_THROUGH_BATCH: bool = true;

    fn probe_write(&self, batch: &mut Self::Batch, key: &str, value: u64) {
        batch.write(key, value);
    }

    fn probe_delete_all(&self, batch: &mut Self::Batch) {
        batch.delete_all();
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        Ok(self.get(key))
    }

    fn probe_read_through(&self, batch: &Self::Batch, key: &str) -> Option<u64> {
        batch.read_through(self.get(key), key)
    }
}

/// The position a checkpoint has been considered through, if any.
///
/// `NeverRun` has none, which is why a first commit at any position is accepted
/// and only a *regression* against a recorded position is refused.
const fn considered_through(checkpoint: Checkpoint) -> Option<SequencePosition> {
    match checkpoint {
        Checkpoint::NeverRun => None,
        Checkpoint::Live { through } | Checkpoint::Rebuilding { through } => Some(through),
    }
}

/// A fresh per-instance stamp.
///
/// Process-local and monotonic, which is all the foreign-batch check needs: a
/// batch cannot outlive the process that minted it. A durable adapter has the
/// harder job — mint at database creation, and re-mint when that state is
/// restored or cloned.
fn next_stamp() -> u64 {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
