//! What an adapter hands the suite, and the reference implementation of it.

use alloc::sync::Arc;
use core::future::Future;

use happenstance_core::StoreId;
use happenstance_sync::{
    Ack, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch, SendSyncPeer,
};

/// One isolated far side, and a way to reach it.
///
/// One fixture instance is one isolated backing peer, and each
/// [`connect`](Self::connect) is one handle onto it — the event-store family's
/// contract, one port over. A rule calls the opener it is given for a fresh
/// instance, so no rule sees another's events.
///
/// # No `Send`, on purpose
///
/// Not `trait_variant`-derived and bounded on the bare
/// [`SyncPeer`](happenstance_sync::SyncPeer), so a `!Send` peer — the Durable
/// Object's, which holds its transport behind an `Rc` — is a valid fixture.
/// That is SY-17's rule: the suite compiles against a `!Send` fixture peer,
/// which this crate's `tests/local_sync_conformance.rs` does.
///
/// # One peer handle per rule
///
/// A fixture yields peers, never a peer set, and every rule holds exactly one
/// (SY-8). A rule that needed two would be testing the runner.
pub trait SyncFixture {
    /// The peer adapter under test.
    type Peer: happenstance_sync::SyncPeer;

    /// A handle onto this fixture's backing peer.
    fn connect(&self) -> impl Future<Output = Self::Peer>;
}

/// The reference [`SyncFixture`], over [`MemorySyncPeer`].
///
/// Every rule is proven against this before it is allowed to fail an adapter:
/// a rule red here is a wrong rule, not a wrong peer. Each instance owns one
/// in-memory peer and each [`connect`](SyncFixture::connect) is a new handle
/// onto it.
#[derive(Debug, Clone)]
pub struct MemorySyncFixture {
    /// Shared by every handle, so a handle dropped and reconnected sees the same
    /// log (SY-16's precondition, for the rule that will check it).
    peer: Arc<MemorySyncPeer>,
}

impl MemorySyncFixture {
    /// A fixture over a fresh in-memory peer with the minimum conformant
    /// limits.
    #[must_use]
    pub fn new() -> Self {
        Self {
            peer: Arc::new(MemorySyncPeer::new(StoreId::from_bytes([0xA5; 16]))),
        }
    }
}

impl Default for MemorySyncFixture {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncFixture for MemorySyncFixture {
    type Peer = MemorySyncHandle;

    async fn connect(&self) -> Self::Peer {
        MemorySyncHandle {
            peer: Arc::clone(&self.peer),
        }
    }
}

/// One handle onto a [`MemorySyncFixture`]'s peer.
///
/// A handle rather than the peer itself because a fixture hands out many and
/// they must share one log; `Arc` because the reference peer is `Send + Sync`
/// and the handle keeps it so, which is what lets it run under the tokio
/// emitter.
#[derive(Debug, Clone)]
pub struct MemorySyncHandle {
    peer: Arc<MemorySyncPeer>,
}

impl SendSyncPeer for MemorySyncHandle {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        SendSyncPeer::pull(&*self.peer, from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        SendSyncPeer::push(&*self.peer, batch).await
    }

    fn limits(&self) -> PeerLimits {
        SendSyncPeer::limits(&*self.peer)
    }
}
