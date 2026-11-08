//! Hand-written fakes over the in-memory reference types (TST-09): a peer that
//! counts its calls, fails a pull or a push on request and names a responder on
//! request; a store whose group read fails on request; a sleep that records
//! what it was asked to wait and returns at once; and a shared handle onto a
//! reference store, so a `StoreSyncPeer` and a test can both hold one.
//!
//! Interior mutability through `Cell`, because the ports take `&self` and the
//! tests flip a switch between steps. `Cell` makes the types `!Sync`, which
//! the bare flavours the runner binds do not mind.

use core::cell::{Cell, RefCell};
use core::num::NonZeroUsize;
use core::time::Duration;
use std::rc::Rc;

use happenstance_core::{MemoryStoreError, SequencePosition, StoreId, StoreLimit};
use happenstance_sync::runner::Sleep;
use happenstance_sync::write_group::GroupOrderError;
use happenstance_sync::{
    Ack, Classify, FailureClass, GroupPage, IngestError, IngestGroup, IngestStore, Ingested,
    MemoryIngestStore, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled,
    PushBatch, SyncPeer, Waiting, Watermark, WriteGroupStore,
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

/// [`MemorySyncPeer`], counting every call, failing pulls and pushes on
/// request, naming a responder on request, and recording the group count of
/// every push it is offered.
///
/// `MemorySyncPeer` names no responder (ADR-0104), so the stamps here stand in
/// for a far side that does: one for pulls and one for acks, so a test can make
/// the two disagree.
#[derive(Debug)]
pub(crate) struct ProbePeer {
    pub(crate) inner: MemorySyncPeer,
    pub(crate) calls: Cell<usize>,
    pub(crate) fail_pull: Cell<Option<FailureClass>>,
    pub(crate) fail_push: Cell<Option<FailureClass>>,
    pub(crate) pull_responder: Cell<Option<StoreId>>,
    pub(crate) ack_responder: Cell<Option<StoreId>>,
    /// The number of groups in each push offered, refused or not.
    pub(crate) offered: RefCell<Vec<usize>>,
    /// Declared in place of the inner peer's limits, when set.
    pub(crate) limits: RefCell<Option<PeerLimits>>,
    /// When set, every pull ignores its token, answers the inner peer's first
    /// page, and claims this is still waiting: a peer stuck on one batch.
    pub(crate) stuck_waiting: Cell<Option<Waiting>>,
}

impl ProbePeer {
    pub(crate) fn new(inner: MemorySyncPeer) -> Self {
        Self {
            inner,
            calls: Cell::new(0),
            fail_pull: Cell::new(None),
            fail_push: Cell::new(None),
            pull_responder: Cell::new(None),
            ack_responder: Cell::new(None),
            offered: RefCell::new(Vec::new()),
            limits: RefCell::new(None),
            stuck_waiting: Cell::new(None),
        }
    }

    /// A probe whose pulls and acks both name `responder`.
    pub(crate) fn answering_as(inner: MemorySyncPeer, responder: StoreId) -> Self {
        let probe = Self::new(inner);
        probe.answer_as(Some(responder));
        probe
    }

    /// Names `responder` on every later pull and ack.
    pub(crate) fn answer_as(&self, responder: Option<StoreId>) {
        self.pull_responder.set(responder);
        self.ack_responder.set(responder);
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
        let pulled = match self.stuck_waiting.get() {
            Some(waiting) => happenstance_sync::SendSyncPeer::pull(&self.inner, None)
                .await?
                .with_waiting(waiting),
            None => happenstance_sync::SendSyncPeer::pull(&self.inner, from).await?,
        };
        Ok(match self.pull_responder.get() {
            Some(responder) => pulled.with_responder(responder),
            None => pulled,
        })
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        self.count();
        self.offered.borrow_mut().push(batch.groups.len());
        if let Some(class) = self.fail_push.get() {
            return Err(ProbePeerError::Injected(class));
        }
        let ack = happenstance_sync::SendSyncPeer::push(&self.inner, batch).await?;
        Ok(match self.ack_responder.get() {
            Some(responder) => ack.with_responder(responder),
            None => ack,
        })
    }

    fn limits(&self) -> PeerLimits {
        // A clone, because the port hands its limits back by value.
        self.limits
            .borrow()
            .clone()
            .unwrap_or_else(|| happenstance_sync::SendSyncPeer::limits(&self.inner))
    }
}

/// How a [`ProbeStore`]'s group read fails.
#[derive(Debug, thiserror::Error)]
pub(crate) enum ProbeStoreError {
    /// A read a test asked to fail, transiently.
    #[error("the probe store's read failed")]
    ReadFailed,
    /// The reference store's own refusal, passed through with its class.
    #[error(transparent)]
    Order(#[from] GroupOrderError),
}

impl Classify for ProbeStoreError {
    fn class(&self) -> FailureClass {
        match self {
            Self::ReadFailed => FailureClass::Transient,
            Self::Order(error) => error.class(),
        }
    }
}

/// [`MemoryIngestStore`], counting every call, recording the size of every
/// group read, failing group reads on request, and refusing ingests on request.
///
/// Its read error differs from its ingest error, which is the case
/// `StepError`'s separate `R` and `I` exist for. It does not append: seed it
/// through `inner`.
#[derive(Debug)]
pub(crate) struct ProbeStore {
    pub(crate) inner: MemoryIngestStore,
    pub(crate) calls: Cell<usize>,
    pub(crate) fail_read: Cell<bool>,
    /// The `max_events` of every group read, in order.
    pub(crate) read_limits: RefCell<Vec<usize>>,
    /// Refuses every ingest as over this limit, when set.
    pub(crate) refuse_ingest: Cell<Option<StoreLimit>>,
}

impl ProbeStore {
    pub(crate) fn new(inner: MemoryIngestStore) -> Self {
        Self {
            inner,
            calls: Cell::new(0),
            fail_read: Cell::new(false),
            read_limits: RefCell::new(Vec::new()),
            refuse_ingest: Cell::new(None),
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
        self.read_limits.borrow_mut().push(max_events.get());
        if self.fail_read.get() {
            return Err(ProbeStoreError::ReadFailed);
        }
        Ok(WriteGroupStore::read_groups(&self.inner, after, max_events).await?)
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
        if let Some(limit) = self.refuse_ingest.get() {
            return Err(IngestError::exceeds_store_limit(limit, usize::MAX));
        }
        IngestStore::ingest(&self.inner, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        self.count();
        IngestStore::watermark(&self.inner).await
    }
}

/// A sleep that records each duration and returns at once (TST-10), and runs
/// a test's hook after each, so a test can change the world while the run
/// "waits".
pub(crate) struct Recorded<'a> {
    pub(crate) slept: RefCell<Vec<Duration>>,
    after_each: Box<dyn Fn(usize) + 'a>,
}

impl<'a> Recorded<'a> {
    pub(crate) fn new() -> Self {
        Self::then(|_| {})
    }

    /// Calls `hook` with the number of sleeps so far after each one.
    pub(crate) fn then(hook: impl Fn(usize) + 'a) -> Self {
        Self {
            slept: RefCell::new(Vec::new()),
            after_each: Box::new(hook),
        }
    }
}

impl Sleep for Recorded<'_> {
    async fn sleep(&self, duration: Duration) {
        let count = {
            let mut slept = self.slept.borrow_mut();
            slept.push(duration);
            slept.len()
        };
        (self.after_each)(count);
    }
}

/// One reference store behind an `Rc`, so a `StoreSyncPeer` holds its two
/// halves and the test keeps a handle to look at the log.
#[derive(Debug, Clone)]
pub(crate) struct Shared(pub(crate) Rc<MemoryIngestStore>);

impl Shared {
    pub(crate) fn new(store_id: StoreId) -> Self {
        Self(Rc::new(MemoryIngestStore::with_store_id(store_id)))
    }
}

impl WriteGroupStore for Shared {
    type Error = GroupOrderError;

    async fn read_groups(
        &self,
        after: Option<SequencePosition>,
        max_events: NonZeroUsize,
    ) -> Result<GroupPage, Self::Error> {
        WriteGroupStore::read_groups(&*self.0, after, max_events).await
    }
}

impl IngestStore for Shared {
    type Error = MemoryStoreError;

    async fn store_id(&self) -> Result<StoreId, Self::Error> {
        IngestStore::store_id(&*self.0).await
    }

    async fn ingest(
        &self,
        groups: &[IngestGroup<'_>],
    ) -> Result<Ingested, IngestError<Self::Error>> {
        IngestStore::ingest(&*self.0, groups).await
    }

    async fn watermark(&self) -> Result<Watermark, Self::Error> {
        IngestStore::watermark(&*self.0).await
    }
}
