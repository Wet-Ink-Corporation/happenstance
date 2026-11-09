//! The watermark rule (SY-31, #143) against projection stores that are wrong
//! only for a `sync/` id.
//!
//! Each store is the reference `MemoryProjectionStore` with one mistake in how
//! it keeps a runner's watermark, and every other projection it keeps exactly
//! as the reference does. Each test runs the rule and requires it to fail
//! with the message the mistake earns, so a mutant cannot pass on another
//! mutant's defect; the faithful wrapper must pass, so the wrapping itself is
//! not what fails.

#![cfg(not(target_arch = "wasm32"))]

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex, PoisonError};

use happenstance_core::{
    Authority, Checkpoint, CommitError, MemoryProjectionBatch, MemoryProjectionStore,
    MemoryProjectionStoreError, ProjectionId, ProjectionProbe, ProjectionStore, ResetError,
    SequencePosition, StoreId,
};
use happenstance_sync_testkit::Violation;
use happenstance_sync_testkit::watermark_rules::watermark_advances_transactionally_with_a_read_model;
use happenstance_testkit::fixtures::MemoryProjectionHandle;
use happenstance_testkit::{Capability, ProjectionFixture, RuleOutcome};

/// The one way each store keeps a `sync/` id wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mistake {
    /// None: the control.
    Faithful,
    /// Refuses the reserved prefix, as a store that validates every id as an
    /// application's name and was written before ADR-0082 minted one id that
    /// is not.
    RefusesTheReservedPrefix,
    /// Keeps a sync id's checkpoint in the handle that wrote it: the runner's
    /// in-memory watermark, which a restart loses and no other handle reads.
    HandleLocalWatermark,
    /// Keys every peer's watermark under one row, as a key column that keeps
    /// the prefix and drops the peer.
    OneWatermarkForEveryPeer,
    /// Upserts a sync id's checkpoint into a side table before committing the
    /// batch, so the side table moves even when the commit is refused.
    UpsertedWatermark,
}

/// The one id [`Mistake::OneWatermarkForEveryPeer`] files every watermark
/// under.
const SHARED_PEER: StoreId = StoreId::from_bytes([0; 16]);

/// A wrong store's error: the reference's, or its own refusal.
#[derive(Debug)]
enum WrongError {
    Inner(MemoryProjectionStoreError),
    ReservedPrefix,
    /// A port error variant added after this wrapper was written.
    Unknown,
}

impl core::fmt::Display for WrongError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Inner(error) => core::fmt::Display::fmt(error, f),
            Self::ReservedPrefix => f.write_str("`sync/` is not an id this store accepts"),
            Self::Unknown => f.write_str("a port error this wrapper does not know"),
        }
    }
}

impl core::error::Error for WrongError {}

fn inner<T>(result: Result<T, MemoryProjectionStoreError>) -> Result<T, WrongError> {
    result.map_err(WrongError::Inner)
}

/// The reference's commit error as this store's. By reference, because the
/// reference's own error is uninhabited and `Copy`, so nothing is moved out.
fn inner_commit(error: &CommitError<MemoryProjectionStoreError>) -> CommitError<WrongError> {
    match *error {
        CommitError::ForeignBatch => CommitError::ForeignBatch,
        CommitError::CheckpointRegression { current, attempted } => {
            CommitError::CheckpointRegression { current, attempted }
        }
        CommitError::Store(error) => CommitError::Store(WrongError::Inner(error)),
        _ => CommitError::Store(WrongError::Unknown),
    }
}

fn is_watermark(id: &ProjectionId) -> bool {
    id.as_str().starts_with("sync/")
}

/// Positions by id, shared or per handle.
type Kept = Mutex<BTreeMap<ProjectionId, SequencePosition>>;

/// One handle onto a wrong store.
#[derive(Debug)]
struct WrongStore {
    reference: MemoryProjectionHandle,
    mistake: Mistake,
    /// This handle's own map: [`Mistake::HandleLocalWatermark`]'s.
    handle_local: Kept,
    /// The store's side table: [`Mistake::UpsertedWatermark`]'s.
    side_table: Arc<Kept>,
}

impl WrongStore {
    /// The id the reference keeps `id` under.
    fn keyed(&self, id: &ProjectionId) -> ProjectionId {
        if self.mistake == Mistake::OneWatermarkForEveryPeer && is_watermark(id) {
            ProjectionId::sync_watermark(SHARED_PEER)
        } else {
            id.clone()
        }
    }

    /// The map a sync id's checkpoint is kept in, for the two mistakes that
    /// keep one apart from the reference.
    fn apart(&self, id: &ProjectionId) -> Option<&Kept> {
        if !is_watermark(id) {
            return None;
        }
        match self.mistake {
            Mistake::HandleLocalWatermark => Some(&self.handle_local),
            Mistake::UpsertedWatermark => Some(&self.side_table),
            _ => None,
        }
    }
}

/// The rows-only id [`Mistake::HandleLocalWatermark`] commits a batch under,
/// because the reference commits rows only with a checkpoint.
const ROWS_ONLY: ProjectionId = ProjectionId::from_static("rows-only");

impl ProjectionStore for WrongStore {
    type Error = WrongError;
    type Batch = MemoryProjectionBatch;

    async fn begin(&self) -> Result<Self::Batch, Self::Error> {
        inner(self.reference.begin().await)
    }

    async fn checkpoint(&self, id: &ProjectionId) -> Result<Checkpoint, Self::Error> {
        if let Some(kept) = self.apart(id) {
            let through = kept
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get(id)
                .copied();
            return Ok(through.map_or(Checkpoint::NeverRun, |through| Checkpoint::Live { through }));
        }
        inner(self.reference.checkpoint(&self.keyed(id)).await)
    }

    async fn commit(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
        position: SequencePosition,
        authority: Authority,
    ) -> Result<(), CommitError<Self::Error>> {
        if is_watermark(id) {
            match self.mistake {
                Mistake::RefusesTheReservedPrefix => {
                    return Err(CommitError::Store(WrongError::ReservedPrefix));
                }
                Mistake::HandleLocalWatermark => {
                    self.handle_local
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .insert(id.clone(), position);
                    return self
                        .reference
                        .commit(batch, &ROWS_ONLY, position, authority)
                        .await
                        .map_err(|error| inner_commit(&error));
                }
                Mistake::UpsertedWatermark => {
                    self.side_table
                        .lock()
                        .unwrap_or_else(PoisonError::into_inner)
                        .insert(id.clone(), position);
                }
                Mistake::Faithful | Mistake::OneWatermarkForEveryPeer => {}
            }
        }
        self.reference
            .commit(batch, &self.keyed(id), position, authority)
            .await
            .map_err(|error| inner_commit(&error))
    }

    async fn reset(
        &self,
        batch: Self::Batch,
        id: &ProjectionId,
    ) -> Result<(), ResetError<Self::Error>> {
        self.reference
            .reset(batch, &self.keyed(id))
            .await
            .map_err(|error| match error {
                ResetError::Store(error) => ResetError::Store(WrongError::Inner(error)),
                ResetError::ForeignBatch => ResetError::ForeignBatch,
                ResetError::Refused => ResetError::Refused,
                _ => ResetError::Store(WrongError::Unknown),
            })
    }

    async fn rollback(&self, batch: Self::Batch) -> Result<(), Self::Error> {
        inner(self.reference.rollback(batch).await)
    }
}

impl ProjectionProbe for WrongStore {
    const READS_THROUGH_BATCH: bool = true;

    async fn probe_write(
        &self,
        batch: &mut Self::Batch,
        key: &str,
        value: u64,
    ) -> Result<(), Self::Error> {
        inner(self.reference.probe_write(batch, key, value).await)
    }

    async fn probe_delete_all(&self, batch: &mut Self::Batch) -> Result<(), Self::Error> {
        inner(self.reference.probe_delete_all(batch).await)
    }

    async fn probe_read(&self, key: &str) -> Result<Option<u64>, Self::Error> {
        inner(self.reference.probe_read(key).await)
    }

    async fn probe_read_through(
        &self,
        batch: &mut Self::Batch,
        key: &str,
    ) -> Result<Option<u64>, Self::Error> {
        inner(self.reference.probe_read_through(batch, key).await)
    }
}

/// One wrong store, any number of handles.
#[derive(Debug)]
struct WrongFixture {
    store: Arc<MemoryProjectionStore>,
    side_table: Arc<Kept>,
    mistake: Mistake,
}

impl WrongFixture {
    fn new(mistake: Mistake) -> Self {
        Self {
            store: Arc::new(MemoryProjectionStore::new()),
            side_table: Arc::new(Mutex::new(BTreeMap::new())),
            mistake,
        }
    }
}

impl ProjectionFixture for WrongFixture {
    type Store = WrongStore;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    fn connect(&self) -> impl Future<Output = Self::Store> {
        // Refcount bumps, and a fresh handle-local map per handle.
        core::future::ready(WrongStore {
            reference: MemoryProjectionHandle::new(Arc::clone(&self.store)),
            mistake: self.mistake,
            handle_local: Mutex::new(BTreeMap::new()),
            side_table: Arc::clone(&self.side_table),
        })
    }
}

async fn verdict(mistake: Mistake) -> Result<RuleOutcome, Violation> {
    watermark_advances_transactionally_with_a_read_model(async || WrongFixture::new(mistake)).await
}

/// Requires the rule to fail `mistake` as SY-31, with a message carrying
/// `expected`.
async fn fails_with(mistake: Mistake, expected: &str) {
    let violation = verdict(mistake)
        .await
        .expect_err("the watermark rule must reject this store");
    assert_eq!(violation.clause(), "SY-31", "{violation}");
    assert!(
        violation.to_string().contains(expected),
        "`{mistake:?}` failed for another reason: {violation}"
    );
}

#[tokio::test]
async fn the_faithful_wrapper_passes() {
    assert_eq!(verdict(Mistake::Faithful).await, Ok(RuleOutcome::Ran));
}

#[tokio::test]
async fn the_rule_fails_a_store_that_refuses_the_reserved_prefix() {
    fails_with(
        Mistake::RefusesTheReservedPrefix,
        "the store refused to commit a watermark",
    )
    .await;
}

#[tokio::test]
async fn the_rule_fails_a_store_that_keeps_the_watermark_in_its_handle() {
    fails_with(Mistake::HandleLocalWatermark, "a fresh handle loads None").await;
}

#[tokio::test]
async fn the_rule_fails_a_store_that_keys_every_peers_watermark_alike() {
    fails_with(
        Mistake::OneWatermarkForEveryPeer,
        "a fresh handle loads Some",
    )
    .await;
}

#[tokio::test]
async fn the_rule_fails_a_store_that_upserts_the_watermark_around_the_port() {
    fails_with(Mistake::UpsertedWatermark, "a fresh handle loads Some").await;
}
