//! SY-17's rule: the suite compiles, and runs, against a `!Send` fixture peer.
//!
//! The peer holds the reference peer through an `Rc`, as a Durable Object holds
//! its transport, and the suite is mounted with the blocking emitter, which
//! imposes no `Send` bound anywhere. If a rule, the fixture trait or an emitter
//! grew a `Send` requirement, this file would stop compiling.

#![cfg(not(target_arch = "wasm32"))]

use std::rc::Rc;

use happenstance_core::StoreId;
use happenstance_sync::{
    Ack, MemoryPeerError, MemoryResume, MemorySyncPeer, PeerLimits, Pulled, PushBatch, SyncPeer,
};
use happenstance_sync_testkit::SyncFixture;

/// A fixture whose peer is genuinely `!Send`.
#[derive(Debug)]
struct LocalFixture {
    peer: Rc<MemorySyncPeer>,
}

impl LocalFixture {
    fn new() -> Self {
        Self {
            peer: Rc::new(MemorySyncPeer::new(StoreId::from_bytes([0x5A; 16]))),
        }
    }
}

impl SyncFixture for LocalFixture {
    type Peer = LocalPeer;

    async fn connect(&self) -> Self::Peer {
        LocalPeer {
            inner: Rc::clone(&self.peer),
        }
    }
}

/// A handle onto the shared peer, through an `Rc`.
#[derive(Debug)]
struct LocalPeer {
    inner: Rc<MemorySyncPeer>,
}

impl SyncPeer for LocalPeer {
    type Error = MemoryPeerError;
    type Resume = MemoryResume;

    async fn pull(&self, from: Option<&Self::Resume>) -> Result<Pulled<Self::Resume>, Self::Error> {
        happenstance_sync::SendSyncPeer::pull(&*self.inner, from).await
    }

    async fn push(&self, batch: &PushBatch) -> Result<Ack, Self::Error> {
        happenstance_sync::SendSyncPeer::push(&*self.inner, batch).await
    }

    fn limits(&self) -> PeerLimits {
        happenstance_sync::SendSyncPeer::limits(&*self.inner)
    }
}

/// The instantiation below only proves something if the peer is `!Send`: a
/// trait with two blanket impls, one only for `Send` types, is ambiguous
/// (`error[E0283]`) when named through a `Send` type.
const _: fn() = || {
    trait AmbiguousIfSend<Marker> {
        fn some_item() {}
    }
    impl<T: ?Sized> AmbiguousIfSend<()> for T {}
    impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}
    <LocalPeer as AmbiguousIfSend<_>>::some_item();
};

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = local_sync_conformance,
    emit = happenstance_sync_testkit::emit_sync_blocking,
    fixture = super::LocalFixture::new()
);
