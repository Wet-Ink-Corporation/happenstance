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
//! The store and every write set it opens append to one log, so the log is
//! shared and mutated from several owners. `Rc<RefCell<_>>` is that sharing
//! on one thread, which is all a runner's test needs. A `Send` twin would
//! need a lock for a log no two threads ever write, so there is none: the
//! store implements `ProjectionStore` and not `SendProjectionStore`.

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::rc::Rc;

use happenstance_core::{
    Authority, Checkpoint, CommitError, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, ProjectionId, ProjectionStore, ResetError, SequencePosition,
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
#[derive(Debug, Default)]
pub struct RecordingProjectionStore {
    inner: MemoryProjectionStore,
    // Shared with every write set this store opens, each of which appends
    // its own end to it; one thread, so `Rc<RefCell<_>>` (module docs).
    log: Rc<RefCell<Vec<BatchCall>>>,
    serials: Cell<u64>,
    begins: Cell<usize>,
    fail_begin: Option<usize>,
    fail_commit: Option<ProjectionId>,
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
        self.log.borrow().clone()
    }

    /// The committed value under `key`, if any.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<u64> {
        self.inner.get(key)
    }

    /// Every committed row.
    #[must_use]
    pub fn snapshot(&self) -> BTreeMap<String, u64> {
        self.inner.snapshot()
    }

    /// Whether no row is committed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn record(&self, call: BatchCall) {
        self.log.borrow_mut().push(call);
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
            .log
            .borrow_mut()
            .push(BatchCall::Marked(self.guard.serial));
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
    // The store's log, shared so this guard can append from `Drop`.
    log: Rc<RefCell<Vec<BatchCall>>>,
    armed: bool,
}

impl EndGuard {
    fn disarm(&mut self, call: BatchCall) {
        self.log.borrow_mut().push(call);
        self.armed = false;
    }
}

impl Drop for EndGuard {
    fn drop(&mut self) {
        if self.armed {
            self.log.borrow_mut().push(BatchCall::Dropped(self.serial));
        }
    }
}

impl ProjectionStore for RecordingProjectionStore {
    type Error = RecordingStoreError;
    type Batch = RecordingBatch;

    async fn begin(&self) -> Result<RecordingBatch, RecordingStoreError> {
        let nth = self.begins.get();
        self.begins.set(nth.saturating_add(1));
        if self.fail_begin == Some(nth) {
            return Err(RecordingStoreError::Injected);
        }
        let serial = self.serials.get();
        self.serials.set(serial.saturating_add(1));
        let inner = ProjectionStore::begin(&self.inner).await?;
        self.record(BatchCall::Begin(serial));
        Ok(RecordingBatch {
            inner,
            guard: EndGuard {
                serial,
                // A second handle onto the one log, not a copy of it.
                log: Rc::clone(&self.log),
                armed: true,
            },
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, RecordingStoreError> {
        Ok(ProjectionStore::checkpoint(&self.inner, id).await?)
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
        ProjectionStore::commit(&self.inner, inner, id, position, authority)
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
        ProjectionStore::reset(&self.inner, inner, id)
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
        Ok(ProjectionStore::rollback(&self.inner, inner).await?)
    }
}
