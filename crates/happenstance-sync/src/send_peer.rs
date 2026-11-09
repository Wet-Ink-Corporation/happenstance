//! The `Send` flavour of the network peer's two halves (#421).
//!
//! [`StoreSyncPeer`] and [`RemoteSyncPeer`] implement the bare [`SyncPeer`], so
//! a `!Send` store or transport is a valid half: the Durable Object's SQLite,
//! its stub. That leaves them out of generic code bound on
//! [`SendSyncPeer`](crate::SendSyncPeer), a runner that spawns its edge onto a
//! multi-threaded runtime, and neither type can implement that flavour as
//! well: `trait_variant` already gives every
//! `SendSyncPeer` the bare trait through a blanket impl, so a second impl on
//! the same type is `error[E0119]` (RS-20-4). The `Send` flavour is a second
//! type, [`SendPeer`], wrapping the first.

// One flavour name per trait (CLAUDE.md rule 4): the bare `SyncPeer`, which the
// delegations call, is imported, and `SendSyncPeer` is named by path.
use crate::ingest::SendIngestStore;
use crate::peer::{Ack, PeerLimits, Pulled, PushBatch, SyncPeer};
use crate::remote::{RemoteSyncPeer, SendExchange};
use crate::store_peer::{StoreResume, StoreSyncPeer};
use crate::wire::ResumeToken;
use crate::write_group::SendWriteGroupStore;

/// A [`StoreSyncPeer`] or [`RemoteSyncPeer`] of the
/// [`SendSyncPeer`](crate::SendSyncPeer) flavour.
///
/// It answers exactly as the peer it wraps: each method delegates, and the
/// [`Error`](SyncPeer::Error) and [`Resume`](SyncPeer::Resume) are the
/// wrapped peer's. What it adds is the promise: its futures are `Send`, so a
/// function generic over `SendSyncPeer` can hold them across a
/// `tokio::spawn`. Through `trait_variant`'s blanket impl it is a bare
/// [`SyncPeer`] too.
///
/// It implements the flavour for two peers and their `Send` halves:
///
/// * `SendPeer<StoreSyncPeer<S, I>>`, for `S:`
///   [`SendWriteGroupStore`]` + Sync` and `I:` [`SendIngestStore`]` + Sync`;
/// * `SendPeer<RemoteSyncPeer<X>>`, for `X:` [`SendExchange`]` + Sync`.
///
/// `Sync` because a pull or a push holds `&self` across its awaits, and a
/// borrow is `Send` only when what it borrows is `Sync`.
///
/// It is not a wrapper for any [`SyncPeer`]. A bare peer's futures promise
/// nothing about `Send`, and stable Rust has no way to ask "whose `pull`
/// future is `Send`" of a generic `P`, so the impl is written per peer, where
/// the halves' own flavours supply the promise.
///
/// # Examples
///
/// ```
/// # #[cfg(not(feature = "memory"))]
/// # fn main() {}
/// # #[cfg(feature = "memory")]
/// # fn main() {
/// use happenstance_sync::{SendPeer, SendSyncPeer, StoreSyncPeer};
///
/// // Generic code that may spawn knows the peer by the `Send` flavour only.
/// fn spawnable<P: SendSyncPeer + Sync>(_peer: &P) {}
///
/// let hub = Shared(std::sync::Arc::new(happenstance_sync::MemoryIngestStore::new()));
/// let peer = SendPeer::new(StoreSyncPeer::new(hub.clone(), hub));
/// spawnable(&peer);
/// # }
/// # #[cfg(feature = "memory")]
/// # #[derive(Clone)]
/// # struct Shared(std::sync::Arc<happenstance_sync::MemoryIngestStore>);
/// # #[cfg(feature = "memory")]
/// # mod shared {
/// #     use happenstance_core::*;
/// #     use happenstance_sync::*;
/// #     impl SendWriteGroupStore for super::Shared {
/// #         type Error = write_group::GroupOrderError;
/// #         async fn read_groups(&self, after: Option<SequencePosition>,
/// #             max_events: core::num::NonZeroUsize) -> Result<GroupPage, Self::Error> {
/// #             SendWriteGroupStore::read_groups(&*self.0, after, max_events).await
/// #         }
/// #     }
/// #     impl SendIngestStore for super::Shared {
/// #         type Error = MemoryStoreError;
/// #         async fn store_id(&self) -> Result<StoreId, Self::Error> {
/// #             SendIngestStore::store_id(&*self.0).await
/// #         }
/// #         async fn ingest(&self, groups: &[IngestGroup<'_>])
/// #             -> Result<Ingested, IngestError<Self::Error>> {
/// #             SendIngestStore::ingest(&*self.0, groups).await
/// #         }
/// #         async fn watermark(&self) -> Result<Watermark, Self::Error> {
/// #             SendIngestStore::watermark(&*self.0).await
/// #         }
/// #     }
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct SendPeer<P> {
    peer: P,
}

impl<P> SendPeer<P> {
    /// The `Send` flavour of `peer`.
    #[must_use]
    pub const fn new(peer: P) -> Self {
        Self { peer }
    }

    /// The wrapped peer.
    #[must_use]
    pub const fn get_ref(&self) -> &P {
        &self.peer
    }

    /// The wrapped peer, handed back.
    #[must_use]
    pub fn into_inner(self) -> P {
        self.peer
    }
}

impl<S, I> crate::peer::SendSyncPeer for SendPeer<StoreSyncPeer<S, I>>
where
    S: SendWriteGroupStore + Sync,
    I: SendIngestStore + Sync,
{
    type Error = <StoreSyncPeer<S, I> as SyncPeer>::Error;
    type Resume = StoreResume;

    /// [`StoreSyncPeer`]'s pull.
    ///
    /// # Errors
    ///
    /// As [`StoreSyncPeer`]'s pull.
    async fn pull(&self, from: Option<&StoreResume>) -> Result<Pulled<StoreResume>, Self::Error> {
        SyncPeer::pull(&self.peer, from).await
    }

    /// [`StoreSyncPeer`]'s push.
    ///
    /// # Errors
    ///
    /// As [`StoreSyncPeer`]'s push.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        SyncPeer::push(&self.peer, batch).await
    }

    fn limits(&self) -> PeerLimits {
        SyncPeer::limits(&self.peer)
    }
}

impl<X> crate::peer::SendSyncPeer for SendPeer<RemoteSyncPeer<X>>
where
    X: SendExchange + Sync,
{
    type Error = <RemoteSyncPeer<X> as SyncPeer>::Error;
    type Resume = ResumeToken;

    /// [`RemoteSyncPeer`]'s pull: one [`Route::Pull`](crate::wire::Route::Pull)
    /// exchange.
    ///
    /// # Errors
    ///
    /// As [`RemoteSyncPeer`]'s pull.
    async fn pull(&self, from: Option<&ResumeToken>) -> Result<Pulled<ResumeToken>, Self::Error> {
        SyncPeer::pull(&self.peer, from).await
    }

    /// [`RemoteSyncPeer`]'s push: one [`Route::Push`](crate::wire::Route::Push)
    /// exchange.
    ///
    /// # Errors
    ///
    /// As [`RemoteSyncPeer`]'s push.
    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        SyncPeer::push(&self.peer, batch).await
    }

    fn limits(&self) -> PeerLimits {
        SyncPeer::limits(&self.peer)
    }
}
