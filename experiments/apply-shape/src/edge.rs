//! The `!Send` end: a projection store and a projection that hold an `Rc`.
//!
//! The shape of a Workers deployment (`happenstance-cloudflare`'s store is the
//! workspace's only `!Send` one). It is in the library, not in `tests/`, so that
//! `cargo check --lib --target wasm32-unknown-unknown --no-default-features`
//! type-checks the proposed trait, the runner **and** a `!Send` implementation
//! driven by it — which is the whole of what the wasm32 half of the question
//! asks. It is also run on the host by `tests/edge.rs`.
//!
//! The `Rc` is the assertion, as it is in `happenstance-core`'s own witness
//! store: a type holding one cannot be `Send`, so if anything in the proposed
//! trait or the runner smuggled a `Send` bound in, this file stops compiling.
//!
//! The flavour split, as an application meets it. The bare bound accepts the
//! edge tally:
//!
//! ```
//! use apply_shape::Projection;
//! use apply_shape::edge::EdgeTally;
//!
//! fn runs_here<P: Projection>(_: &P) {}
//! runs_here(&EdgeTally::new("edge"));
//! ```
//!
//! and the `Send` flavour refuses it — `error[E0277]`, the trait bound
//! `EdgeTally: SendProjection` is not satisfied. (The code is advisory, per
//! RS-62-1; the fence above is the control.)
//!
//! ```compile_fail,E0277
//! use apply_shape::SendProjection;
//! use apply_shape::edge::EdgeTally;
//!
//! fn spawnable<P: SendProjection>(_: &P) {}
//! spawnable(&EdgeTally::new("edge"));
//! ```

use core::cell::RefCell;
use core::num::NonZeroUsize;
use std::collections::BTreeMap;
use std::rc::Rc;

use happenstance::{
    Authority, Checkpoint, CommitError, EventStore, Json, ProjectionId, ProjectionStore,
    ResetError, SequencePosition, Tags,
};

use crate::domain::{POISON, Ticked, scope};
use crate::runner::{Ran, RunErrorFor, run};
use crate::shape::{ApplyFailure, Delivered, Policy, Projection};

/// The edge store's only failure.
#[derive(Debug, thiserror::Error)]
#[error("edge store: {0}")]
pub struct EdgeStoreError(&'static str);

/// Everything the edge store holds, shared through an `Rc`.
#[derive(Debug, Default)]
struct EdgeState {
    rows: BTreeMap<String, u64>,
    checkpoints: BTreeMap<String, SequencePosition>,
}

/// A projection store that is `!Send` by construction.
#[derive(Debug, Clone, Default)]
pub struct EdgeStore {
    state: Rc<RefCell<EdgeState>>,
}

/// A buffered write set, also `!Send`: it holds the store's `Rc` so it can
/// read committed state through itself, as a live batch would.
#[derive(Debug)]
pub struct EdgeBatch {
    state: Rc<RefCell<EdgeState>>,
    writes: Vec<(String, u64)>,
}

impl EdgeBatch {
    /// Buffers one row.
    pub fn write(&mut self, key: impl Into<String>, value: u64) {
        self.writes.push((key.into(), value));
    }

    /// The committed value under `key`, then any pending write over it.
    #[must_use]
    pub fn read(&self, key: &str) -> Option<u64> {
        let pending = self
            .writes
            .iter()
            .rev()
            .find(|(written, _)| written == key)
            .map(|(_, value)| *value);
        pending.or_else(|| self.state.borrow().rows.get(key).copied())
    }
}

impl EdgeStore {
    /// A committed row.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<u64> {
        self.state.borrow().rows.get(key).copied()
    }
}

impl ProjectionStore for EdgeStore {
    type Error = EdgeStoreError;
    type Batch = EdgeBatch;

    async fn begin(&self) -> Result<EdgeBatch, EdgeStoreError> {
        Ok(EdgeBatch {
            state: Rc::clone(&self.state),
            writes: Vec::new(),
        })
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, EdgeStoreError> {
        Ok(self
            .state
            .borrow()
            .checkpoints
            .get(id.as_str())
            .map_or(Checkpoint::NeverRun, |&through| Checkpoint::Live {
                through,
            }))
    }

    async fn commit(
        &self,
        batch: EdgeBatch,
        id: &ProjectionId,
        position: SequencePosition,
        _authority: Authority,
    ) -> Result<(), CommitError<EdgeStoreError>> {
        if !Rc::ptr_eq(&batch.state, &self.state) {
            return Err(CommitError::ForeignBatch);
        }
        let mut state = self.state.borrow_mut();
        if let Some(&current) = state.checkpoints.get(id.as_str())
            && position < current
        {
            return Err(CommitError::CheckpointRegression {
                current,
                attempted: position,
            });
        }
        state.rows.extend(batch.writes);
        state.checkpoints.insert(id.as_str().to_owned(), position);
        Ok(())
    }

    async fn reset(
        &self,
        batch: EdgeBatch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<EdgeStoreError>> {
        drop(batch);
        let mut state = self.state.borrow_mut();
        state.rows.clear();
        state.checkpoints.remove(id.as_str());
        Ok(())
    }

    async fn rollback(&self, batch: EdgeBatch) -> Result<(), EdgeStoreError> {
        drop(batch);
        Ok(())
    }
}

/// The edge projection's error.
#[derive(Debug, thiserror::Error)]
pub enum EdgeError {
    /// The store refused.
    #[error(transparent)]
    Store(#[from] EdgeStoreError),
    /// The application refused the event.
    #[error("refused a tick against `{0}`")]
    Refused(String),
}

/// A counter per key, and a count of what was skipped. `!Send`: it keeps an
/// `Rc` log of the ids it has seen, the thing a single-threaded edge
/// application would reach for without thinking.
#[derive(Debug)]
pub struct EdgeTally {
    id: ProjectionId,
    scope: Tags,
    /// What `apply` has been handed, by identity.
    pub seen: Rc<RefCell<Vec<happenstance::EventId>>>,
}

impl EdgeTally {
    /// A fresh tally, named by a literal.
    ///
    /// # Panics
    ///
    /// If `name` is not a valid [`ProjectionId`]. Every caller passes a literal
    /// that is: `ProjectionId::from_static` is the validating door for one.
    #[must_use]
    pub fn new(name: &'static str) -> Self {
        Self {
            id: ProjectionId::from_static(name),
            scope: scope(),
            seen: Rc::default(),
        }
    }
}

impl Projection for EdgeTally {
    type Event = Ticked;
    type Store = EdgeStore;
    type Error = EdgeError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Ticked>,
        batch: &mut EdgeBatch,
    ) -> Result<(), EdgeError> {
        self.seen.borrow_mut().push(event.id());
        let Ticked { key } = event.into_event();
        if key == POISON {
            return Err(EdgeError::Refused(key));
        }
        let next = batch.read(&key).unwrap_or(0) + 1;
        batch.write(key, next);
        Ok(())
    }

    /// Skips, and records the skip in the batch that moves the checkpoint.
    fn on_error(
        &mut self,
        failure: &ApplyFailure<'_, EdgeError>,
        batch: &mut EdgeBatch,
    ) -> impl Future<Output = Result<Policy, EdgeError>> {
        let skipped = batch.read("skipped").unwrap_or(0) + 1;
        batch.write("skipped", skipped);
        let _ = failure.id();
        async { Ok(Policy::Skip) }
    }
}

/// Runs the edge tally to the head of `events`.
///
/// Its existence is the wasm32 assertion: this function type-checks only if
/// `run` accepts a `!Send` store and a `!Send` projection through the bare
/// flavours.
///
/// # Errors
///
/// Whatever `run` reports.
pub async fn run_edge<S: EventStore>(
    events: &S,
    store: &EdgeStore,
    tally: &mut EdgeTally,
) -> Result<Ran, RunErrorFor<S, EdgeTally>> {
    run(events, store, tally, &Json, NonZeroUsize::MIN).await
}
