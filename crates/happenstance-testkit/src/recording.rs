//! A projection store that records each write set's life: the instrument a
//! projection *runner's* tests need, where the conformance suite tests stores.
//!
//! A runner's obligations are about calls, not only about what they leave
//! behind: every `begin` ends in one `commit` or `rollback`, a write set is
//! rolled back rather than dropped bare, and one view's write set ends before
//! another view commits. [`MemoryProjectionStore`] cannot tell a rollback
//! from a drop, or say when a write set ended, so a test over it cannot fail
//! a runner that gets any of these wrong. This wrapper logs each of them, and
//! fails one `begin` or one checkpoint's commits on request.
//!
//! # Why only the bare flavour
//!
//! Every handle onto the store and every write set it opens append to one
//! log, so the log is shared and mutated from several owners. An `Rc` over
//! cells is that sharing on one thread, which is all a runner's test needs. A `Send` twin would
//! need a lock for a log no two threads ever write, so there is none: the
//! store implements `ProjectionStore` and not `SendProjectionStore`.

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use happenstance_core::{
    Authority, Checkpoint, CommitError, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition,
};

/// One thing that happened to a write set, by the serial `begin` gave it.
///
/// Serials count up from zero across the whole store, one per successful
/// `begin`, so two write sets never share one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum BatchCall {
    /// `begin` opened it.
    Begin(u64),
    /// `commit` was called with it, whether or not the commit succeeded.
    Commit(u64),
    /// `reset` was called with it.
    Reset(u64),
    /// `rollback` was called with it.
    Rollback(u64),
    /// It was dropped without any of the calls above.
    Dropped(u64),
    /// The code holding it called [`RecordingBatch::mark`].
    Marked(u64),
}

impl BatchCall {
    /// The serial of the write set this call is about.
    #[must_use]
    pub const fn serial(self) -> u64 {
        match self {
            Self::Begin(serial)
            | Self::Commit(serial)
            | Self::Reset(serial)
            | Self::Rollback(serial)
            | Self::Dropped(serial)
            | Self::Marked(serial) => serial,
        }
    }

    /// Whether this call ends the write set it names.
    #[must_use]
    pub const fn ends(self) -> bool {
        matches!(
            self,
            Self::Commit(_) | Self::Reset(_) | Self::Rollback(_) | Self::Dropped(_)
        )
    }
}

/// Why a [`RecordingProjectionStore`] call failed.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RecordingStoreError {
    /// The fixture injected this failure; nothing underneath it went wrong.
    Injected,
    /// The wrapped store refused a commit in a way this crate has no arm
    /// for. `CommitError` is `#[non_exhaustive]`, so a new arm lands here
    /// rather than being lost.
    Commit(CommitError<MemoryProjectionStoreError>),
    /// The wrapped store refused a reset in a way this crate has no arm for.
    Reset(ResetError<MemoryProjectionStoreError>),
}

impl core::fmt::Display for RecordingStoreError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Injected => f.write_str(
                "the test fixture injected this failure; nothing underneath it went wrong",
            ),
            Self::Commit(err) => write!(f, "the wrapped store refused a commit: {err}"),
            Self::Reset(err) => write!(f, "the wrapped store refused a reset: {err}"),
        }
    }
}

impl core::error::Error for RecordingStoreError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Injected => None,
            Self::Commit(err) => Some(err),
            Self::Reset(err) => Some(err),
        }
    }
}

impl From<MemoryProjectionStoreError> for RecordingStoreError {
    fn from(never: MemoryProjectionStoreError) -> Self {
        match never {}
    }
}

/// A [`MemoryProjectionStore`] that logs every `begin`, `commit`, `reset`,
/// `rollback` and bare drop of its write sets, in call order.
///
/// Read the log with [`calls`](Self::calls), and the rows with
/// [`get`](Self::get) and [`snapshot`](Self::snapshot). The log is never
/// trimmed, which suits a test's few dozen calls and nothing longer.
///
/// # Injected failures
///
/// [`failing_begin`](Self::failing_begin) fails one `begin`, counted across
/// every caller, and [`failing_commits_under`](Self::failing_commits_under)
/// fails every commit under one checkpoint id. A failed `begin` opens no
/// write set and logs nothing. A failed commit is logged as
/// [`BatchCall::Commit`] and discards its write set, as a store whose commit
/// failed would.
///
/// # Handles
///
/// A clone is a second handle onto the **same** rows, checkpoints and log,
/// as two connections onto one database are, with the same injected
/// failures. That is what lets the projection suite run over it, unarmed
/// (`tests/recording_store_conformance.rs`).
///
/// # Example
///
/// ```
/// use happenstance_core::{Authority, ProjectionId, ProjectionStore};
/// use happenstance_core::SequencePosition;
/// use happenstance_testkit::{BatchCall, RecordingProjectionStore};
/// use happenstance_testkit::block_on;
///
/// let store = RecordingProjectionStore::new();
/// let id = ProjectionId::from_static("tally");
/// let at = SequencePosition::new(1).ok_or("one is a position")?;
/// block_on(async {
///     let mut kept = store.begin().await?;
///     kept.write("tally", 1);
///     store.commit(kept, &id, at, Authority::Live).await?;
///     let dropped = store.begin().await?;
///     drop(dropped);
///     Ok::<_, Box<dyn std::error::Error>>(())
/// })?;
///
/// assert_eq!(store.get("tally"), Some(1));
/// assert_eq!(
///     store.calls(),
///     [
///         BatchCall::Begin(0),
///         BatchCall::Commit(0),
///         BatchCall::Begin(1),
///         BatchCall::Dropped(1),
///     ]
/// );
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Debug, Default, Clone)]
pub struct RecordingProjectionStore {
    // Shared by every handle and every write set they open: each appends to
    // the one log, and handles count `begin`s together. One thread, so `Rc`
    // and cells rather than an `Arc` and a lock (module docs).
    shared: Rc<Shared>,
    fail_begin: Option<usize>,
    fail_commit: Option<ProjectionId>,
}

/// What every handle onto one recording store shares.
#[derive(Debug, Default)]
struct Shared {
    inner: MemoryProjectionStore,
    log: RefCell<Vec<BatchCall>>,
    serials: Cell<u64>,
    begins: Cell<usize>,
}

impl Shared {
    fn record(&self, call: BatchCall) {
        self.log.borrow_mut().push(call);
    }
}

impl RecordingProjectionStore {
    /// An empty store that fails nothing.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// An empty store whose zero-based `nth` `begin` fails, counted across
    /// every caller.
    #[must_use]
    pub fn failing_begin(nth: usize) -> Self {
        Self {
            fail_begin: Some(nth),
            ..Self::default()
        }
    }

    /// An empty store whose every commit under `id` fails.
    #[must_use]
    pub fn failing_commits_under(id: ProjectionId) -> Self {
        Self {
            fail_commit: Some(id),
            ..Self::default()
        }
    }

    /// Every call so far, in order.
    ///
    /// A copy rather than a borrow: the log sits behind a `RefCell` that
    /// every open write set appends to, and a borrow handed out here would
    /// make the next append panic.
    #[must_use]
    pub fn calls(&self) -> Vec<BatchCall> {
        // A real duplicate, for the reason above; each item is `Copy`.
        self.shared.log.borrow().clone()
    }

    /// The committed value under `key`, if any.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<u64> {
        self.shared.inner.get(key)
    }

    /// Every committed row.
    #[must_use]
    pub fn snapshot(&self) -> BTreeMap<String, u64> {
        self.shared.inner.snapshot()
    }

    /// Whether no row is committed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.shared.inner.is_empty()
    }
}

/// A memory write set that logs how it ended, including a bare drop.
///
/// Write to it with [`write`](Self::write), as to a
/// [`MemoryProjectionBatch`].
#[derive(Debug)]
pub struct RecordingBatch {
    inner: MemoryProjectionBatch,
    guard: EndGuard,
}

impl RecordingBatch {
    /// Buffers `value` under `key` until the write set commits.
    pub fn write(&mut self, key: impl Into<String>, value: u64) {
        self.inner.write(key, value);
    }

    /// The serial `begin` gave this write set.
    #[must_use]
    pub const fn serial(&self) -> u64 {
        self.guard.serial
    }

    /// Logs [`BatchCall::Marked`] for this write set.
    ///
    /// A test's own milestone in the log, such as "about to panic with this
    /// write set open", so it can assert what happened after it.
    pub fn mark(&self) {
        self.guard
            .shared
            .record(BatchCall::Marked(self.guard.serial));
    }

    /// Logs `call` and hands back the memory write set it ended.
    fn end(self, call: BatchCall) -> MemoryProjectionBatch {
        let Self { inner, mut guard } = self;
        guard.disarm(call);
        inner
    }
}

/// Logs [`BatchCall::Dropped`] unless the write set ended by a call first.
///
/// Its own type, so that [`RecordingBatch`] has no `Drop` and can be taken
/// apart by value in `end`, with no `Option` that is `None` only mid-drop.
#[derive(Debug)]
struct EndGuard {
    serial: u64,
    // The opening store's state, shared so this guard can log from `Drop`.
    shared: Rc<Shared>,
    armed: bool,
}

impl EndGuard {
    fn disarm(&mut self, call: BatchCall) {
        self.shared.record(call);
        self.armed = false;
    }
}

impl Drop for EndGuard {
    fn drop(&mut self) {
        if self.armed {
            self.shared.record(BatchCall::Dropped(self.serial));
        }
    }
}

impl ProjectionStore for RecordingProjectionStore {
    type Error = RecordingStoreError;
    type Batch = RecordingBatch;

    async fn begin(&self) -> Result<RecordingBatch, RecordingStoreError> {
        let shared = &self.shared;
        let nth = shared.begins.get();
        shared.begins.set(nth.saturating_add(1));
        if self.fail_begin == Some(nth) {
            return Err(RecordingStoreError::Injected);
        }
        let serial = shared.serials.get();
        shared.serials.set(serial.saturating_add(1));
        let inner = ProjectionStore::begin(&shared.inner).await?;
        shared.record(BatchCall::Begin(serial));
        Ok(RecordingBatch {
            inner,
            guard: EndGuard {
                serial,
                // A second handle onto the one state, not a copy of it.
                shared: Rc::clone(shared),
                armed: true,
            },
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, RecordingStoreError> {
        Ok(ProjectionStore::checkpoint(&self.shared.inner, id).await?)
    }

    async fn commit(
        &self,
        batch: RecordingBatch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<RecordingStoreError>> {
        let serial = batch.serial();
        let inner = batch.end(BatchCall::Commit(serial));
        if self.fail_commit.as_ref() == Some(id) {
            drop(inner);
            return Err(CommitError::Store(RecordingStoreError::Injected));
        }
        ProjectionStore::commit(&self.shared.inner, inner, id, position, authority)
            .await
            .map_err(|error| match error {
                CommitError::ForeignBatch => CommitError::ForeignBatch,
                CommitError::CheckpointRegression { current, attempted } => {
                    CommitError::CheckpointRegression { current, attempted }
                }
                other => CommitError::Store(RecordingStoreError::Commit(other)),
            })
    }

    async fn reset(
        &self,
        batch: RecordingBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<RecordingStoreError>> {
        let serial = batch.serial();
        let inner = batch.end(BatchCall::Reset(serial));
        ProjectionStore::reset(&self.shared.inner, inner, id)
            .await
            .map_err(|error| match error {
                ResetError::ForeignBatch => ResetError::ForeignBatch,
                ResetError::Refused => ResetError::Refused,
                other => ResetError::Store(RecordingStoreError::Reset(other)),
            })
    }

    async fn rollback(&self, batch: RecordingBatch) -> Result<(), RecordingStoreError> {
        let serial = batch.serial();
        let inner = batch.end(BatchCall::Rollback(serial));
        Ok(ProjectionStore::rollback(&self.shared.inner, inner).await?)
    }
}

/// The probe the projection suite drives, delegated to the memory store
/// beneath, so the suite can run over an unarmed recording store.
impl ProjectionProbe for RecordingProjectionStore {
    /// `true`, as for the memory store this wraps: it applies on write.
    const READS_THROUGH_BATCH: bool = true;

    async fn probe_write(
        &self,
        batch: &mut RecordingBatch,
        key: &str,
        value: u64,
    ) -> Result<(), RecordingStoreError> {
        Ok(ProjectionProbe::probe_write(&self.shared.inner, &mut batch.inner, key, value).await?)
    }

    async fn probe_delete_all(
        &self,
        batch: &mut RecordingBatch,
    ) -> Result<(), RecordingStoreError> {
        Ok(ProjectionProbe::probe_delete_all(&self.shared.inner, &mut batch.inner).await?)
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, RecordingStoreError> {
        Ok(ProjectionProbe::probe_read(&self.shared.inner, key).await?)
    }

    async fn probe_read_through(
        &self,
        batch: &mut RecordingBatch,
        key: &str,
    ) -> Result<Option<u64>, RecordingStoreError> {
        Ok(ProjectionProbe::probe_read_through(&self.shared.inner, &mut batch.inner, key).await?)
    }
}
