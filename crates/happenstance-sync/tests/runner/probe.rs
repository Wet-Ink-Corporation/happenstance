//! Hand-written fakes over the in-memory reference types (TST-09): a peer that
//! counts its calls and fails a pull on request, and a store whose group read
//! fails on request.
//!
//! Interior mutability through `Cell`, because the ports take `&self` and the
//! tests flip a switch between steps. `Cell` makes both types `!Sync`, which
//! the bare flavours the runner binds do not mind.

use core::cell::Cell;
use core::num::NonZeroUsize;

use happenstance_core::{MemoryStoreError, SequencePosition, StoreId};
use happenstance_sync::{
    Ack, Classify, FailureClass, GroupPage, IngestError, IngestGroup, IngestStore, Ingested,
    MemoryIngestStore, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled,
    PushBatch, SyncPeer, Watermark, WriteGroupStore,
};

/// How a [`ProbePeer`] fails: the reference peer's own failure, or one a test
/// injected with the class it chose.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ProbePeerError {
    #[error(transparent)]
    Memory(#[from] MemoryPeerError),
    #[error("an injected {0:?} failure")]
    Injected(FailureClass),
}

impl Classify for ProbePeerError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Memory(error) => error.class(),
            Self::Injected(class) => *class,
        }
    }
}

/// [`MemorySyncPeer`], counting every call and failing pulls on request.
#[derive(Debug)]
pub(crate) struct ProbePeer {
    pub(crate) inner: MemorySyncPeer,
    pub(crate) calls: Cell<usize>,
    pub(crate) fail_pull: Cell<Option<FailureClass>>,
}

impl ProbePeer {
    pub(crate) fn new(inner: MemorySyncPeer) -> Self {
        Self {
            inner,
            calls: Cell::new(0),
            fail_pull: Cell::new(None),
        }
    }

    fn count(&self) {
        self.calls.set(self.calls.get() + 1);
    }
}

impl SyncPeer for ProbePeer {
    type Error = ProbePeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        self.count();
        if let Some(class) = self.fail_pull.get() {
            return Err(ProbePeerError::Injected(class));
        }
        Ok(happenstance_sync::SendSyncPeer::pull(&self.inner, from).await?)
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.count();
        Ok(happenstance_sync::SendSyncPeer::push(&self.inner, batch).await?)
    }

    fn limits(&self) -> PeerLimits {
        happenstance_sync::SendSyncPeer::limits(&self.inner)
    }
}

/// How a [`ProbeStore`]'s group read fails.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ProbeStoreError {
    /// A read a test asked to fail, transiently.
    #[error("the probe store's read failed")]
    ReadFailed,
}

impl Classify for ProbeStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::ReadFailed => FailureClass::Transient,
        }
    }
}

/// [`MemoryIngestStore`], counting every call and failing group reads on
/// request.
///
/// Its read error differs from its ingest error, which is the case
/// `StepError`'s separate `R` and `I` exist for. It does not append: seed it
/// through `inner`.
#[derive(Debug)]
pub(crate) struct ProbeStore {
    pub(crate) inner: MemoryIngestStore,
    pub(crate) calls: Cell<usize>,
    pub(crate) fail_read: Cell<bool>,
}

impl ProbeStore {
    pub(crate) fn new(inner: MemoryIngestStore) -> Self {
        Self {
            inner,
            calls: Cell::new(0),
            fail_read: Cell::new(false),
        }
    }

    fn count(&self) {
        self.calls.set(self.calls.get() + 1);
    }
}

impl WriteGroupStore for ProbeStore {
    type Error = ProbeStoreError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        self.count();
        if self.fail_read.get() {
            return Err(ProbeStoreError::ReadFailed);
        }
        WriteGroupStore::read_groups(&self.inner, after, max_events)
            .await
            .map_err(|never| match never {})
    }
}

impl IngestStore for ProbeStore {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        self.count();
        IngestStore::store_id(&self.inner).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        self.count();
        IngestStore::ingest(&self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.count();
        IngestStore::watermark(&self.inner).await
    }
}
