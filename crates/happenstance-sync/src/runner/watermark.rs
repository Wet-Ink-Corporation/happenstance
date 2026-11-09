//! Where an edge's confirmation watermark is kept (SY-31, #143).
//!
//! The watermark is a fact about one link: the local position below which the
//! far side has acknowledged everything. It is not an event, because no peer
//! should replicate it and it changes on every sync. It is a checkpoint, kept
//! through [`ProjectionStore`] under [`ProjectionId::sync_watermark`] of the far
//! side's identity, which no application id can collide with (ADR-0082).
//!
//! # In the caller's batch
//!
//! [`Watermark::commit`] takes the caller's batch, so whatever read model the
//! caller wrote into it lands in the same commit as the watermark, or neither
//! does. That is the point of keeping it through the port: a read model whose
//! rows say what was confirmed (Kestrel Cold Chain's `today_schedule`) and the
//! watermark that says it cannot disagree at an instant. The runner writes no
//! rows itself, because the port has no write vocabulary a generic runner
//! could use (PS-9): the caller fills the batch through its adapter's own API.
//!
//! # What is not kept here
//!
//! The pull token. It saves a re-read of what was already ingested, and a
//! re-read is harmless, because ingest skips what it holds (SY-11). An edge
//! [`restored`](super::Edge::restored) from its watermark alone pulls from the
//! start and pushes from the watermark.

use happenstance_core::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, SequencePosition, StoreId,
};

/// One edge's confirmation watermark: the far side, and the first position of
/// the last local write group it acknowledged.
///
/// Built from an edge by [`Edge::watermark`](super::Edge::watermark), read back
/// by [`load`](Self::load), and turned back into an edge by
/// [`Edge::restored`](super::Edge::restored).
///
/// # Examples
///
/// The watermark lands in the batch that carries the caller's read model:
///
/// ```
/// # #[cfg(not(feature = "memory"))]
/// # fn main() {}
/// # #[cfg(feature = "memory")]
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), Box<dyn core::error::Error>> {
/// use happenstance_core::{MemoryProjectionStore, SequencePosition, StoreId};
/// use happenstance_sync::runner::Watermark;
///
/// let hub = StoreId::from_bytes([2; 16]);
/// let store = MemoryProjectionStore::new();
/// let watermark = Watermark::new(hub, SequencePosition::FIRST);
///
/// let mut batch = store.open();
/// batch.write("schedule:confirmed", 1);
/// watermark.commit(&store, batch).await?;
///
/// assert_eq!(Watermark::load(&store, hub).await?, Some(watermark));
/// assert_eq!(store.get("schedule:confirmed"), Some(1));
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Watermark {
    peer: StoreId,
    confirmed: SequencePosition,
}

impl Watermark {
    /// The watermark of the edge with `peer`, at `confirmed`.
    ///
    /// `confirmed` is a local position, as
    /// [`SyncStatus::confirmed`](super::SyncStatus::confirmed) reports it.
    #[must_use]
    pub const fn new(peer: StoreId, confirmed: SequencePosition) -> Self {
        Self { peer, confirmed }
    }

    /// The far side's store identity.
    #[must_use]
    pub const fn peer(&self) -> StoreId {
        self.peer
    }

    /// The first position of the last local write group the far side
    /// acknowledged.
    #[must_use]
    pub const fn confirmed(&self) -> SequencePosition {
        self.confirmed
    }

    /// The projection id the watermark is kept under:
    /// [`ProjectionId::sync_watermark`] of the far side.
    #[must_use]
    pub fn id(&self) -> ProjectionId {
        ProjectionId::sync_watermark(self.peer)
    }

    /// Commits `batch`, with whatever the caller wrote into it, and moves this
    /// watermark's checkpoint to [`confirmed`](Self::confirmed), as one unit.
    ///
    /// The checkpoint is committed [`Authority::Live`]: a watermark has no
    /// rebuild.
    ///
    /// # Errors
    ///
    /// What [`ProjectionStore::commit`] returns, unchanged:
    /// [`CommitError::CheckpointRegression`] if a later watermark is already
    /// kept for this peer, [`CommitError::ForeignBatch`] if `batch` came from
    /// another store, and [`CommitError::Store`] if the adapter failed. Either
    /// way neither the rows nor the watermark changed.
    pub async fn commit<P: ProjectionStore>(
        &self,
        store: &P,
        batch: P::Batch,
    ) -> Result<(), CommitError<P::Error>> {
        store
            .commit(batch, &self.id(), self.confirmed, Authority::Live)
            .await
    }

    /// The watermark kept for `peer`, or `None` if none was ever committed or
    /// it was reset.
    ///
    /// # Errors
    ///
    /// [`LoadWatermarkError::Store`] if the checkpoint cannot be read, and
    /// [`LoadWatermarkError::NotLive`] if what is kept under the id is not a
    /// live checkpoint, which [`commit`](Self::commit) never writes.
    pub async fn load<P: ProjectionStore>(
        store: &P,
        peer: StoreId,
    ) -> Result<Option<Self>, LoadWatermarkError<P::Error>> {
        let checkpoint = store
            .checkpoint(&ProjectionId::sync_watermark(peer))
            .await
            .map_err(LoadWatermarkError::Store)?;
        match checkpoint {
            Checkpoint::NeverRun => Ok(None),
            Checkpoint::Live { through } => Ok(Some(Self::new(peer, through))),
            other => Err(LoadWatermarkError::NotLive(other)),
        }
    }
}

/// Why [`Watermark::load`] could not read a watermark.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum LoadWatermarkError<E> {
    /// The projection store failed to read the checkpoint.
    #[error("the watermark's checkpoint could not be read")]
    Store(#[source] E),
    /// The checkpoint under the watermark's id is not live, so it confirms
    /// nothing: something other than [`Watermark::commit`] wrote it.
    #[error("the checkpoint under a sync watermark's id is {0:?}, not live")]
    NotLive(Checkpoint),
}
