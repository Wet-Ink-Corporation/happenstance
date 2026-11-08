//! `RemoteSyncPeer` over an in-process `Exchange` that calls `serve_sync`.
//!
//! Every pull and push here is encoded into a postcard envelope, handed to
//! `serve_sync` as bytes, answered by a `StoreSyncPeer` over the in-memory
//! store, encoded again and decoded on the way back: the whole network path
//! with the network replaced by a function call. The sync suite is mounted at
//! the bottom; the tests above it pin what a transport can break and the suite
//! cannot see.

#![cfg(not(target_arch = "wasm32"))]

use happenstance_core::{
    Event, EventId, EventStore, MemoryStoreError, Query, ReadOptions, RecordedAt, SequencePosition,
    SequencedEvent, StoreId, collect,
};
use happenstance_sync::wire::{CodecError, FORMAT_VERSION, ResumeToken, Route, WireError};
use happenstance_sync::{
    Classify, EventGroup, FailureClass, IngestStore, PeerLimits, PushBatch, RemoteError,
    RemoteSyncPeer, ReplicatedEvent, SendExchange, ServeError, StoreSyncPeer, SyncPeer, serve_sync,
};
use happenstance_sync_testkit::{
    IngestFixture, MemoryIngestFixture, MemoryIngestHandle, RefusalsOf, SyncFixture,
    oversized_push_refusal,
};

type FarSide = StoreSyncPeer<MemoryIngestHandle, MemoryIngestHandle>;

/// The transport: a function call into `serve_sync`, bytes in and bytes out.
///
/// Its error is `serve_sync`'s own, so the class the far side states is the
/// class the client reads — what an HTTP transport does through a status code.
#[derive(Debug, Clone)]
struct Loopback {
    far: FarSide,
}

impl SendExchange for Loopback {
    type Error = ServeError<MemoryStoreError, MemoryStoreError>;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        serve_sync(&self.far, route, &request).await
    }
}

/// A remote peer over a fresh store, and a handle to read that store's log.
async fn remote() -> (RemoteSyncPeer<Loopback>, MemoryIngestHandle) {
    let store = MemoryIngestFixture::new();
    let handle = store.connect().await;
    let far = StoreSyncPeer::new(store.connect().await, store.connect().await);
    let limits = far.limits();
    (RemoteSyncPeer::new(Loopback { far }, limits), handle)
}

async fn log(store: &MemoryIngestHandle) -> Vec<SequencedEvent> {
    collect(EventStore::read(store, &Query::all(), ReadOptions::new()))
        .await
        .expect("the in-memory store's error is uninhabited")
}

/// Payloads no text codec would leave alone: invalid UTF-8, a NUL, every
/// byte value, and metadata beside the data.
fn awkward_batch() -> PushBatch {
    let origin = StoreId::from_bytes([0x72; 16]);
    let every_byte: Vec<u8> = (0..=u8::MAX).collect();
    let events = [
        (1, vec![0xC3, 0x28, 0x00, 0xFF], None),
        (2, every_byte, Some(vec![0xFE, 0xFF, 0x00])),
        (3, Vec::new(), Some(Vec::new())),
    ]
    .into_iter()
    .map(|(position, data, metadata)| {
        let event = Event::new("Opaque", data).expect("a static event type is valid");
        ReplicatedEvent::new(
            EventId::new(
                origin,
                SequencePosition::new(position).expect("test positions are not zero"),
            ),
            RecordedAt::from_millis(1_800_000_000_000),
            match metadata {
                Some(metadata) => event.with_metadata(metadata),
                None => event,
            },
        )
    })
    .collect();
    PushBatch::new(vec![EventGroup::new(None, events)])
}

#[tokio::test]
async fn payload_bytes_come_back_byte_identical_through_the_remote_path() {
    let (peer, _) = remote().await;
    let batch = awkward_batch();

    peer.push(&batch).await.expect("a push inside the limits");
    let pulled = peer.pull(None).await.expect("a pull");

    let sent: Vec<(&[u8], Option<&[u8]>)> = batch
        .events()
        .map(|event| {
            (
                &event.event.data()[..],
                event.event.metadata().map(|m| &m[..]),
            )
        })
        .collect();
    let back: Vec<(&[u8], Option<&[u8]>)> = pulled
        .batch
        .events()
        .map(|event| {
            (
                &event.event.data()[..],
                event.event.metadata().map(|m| &m[..]),
            )
        })
        .collect();
    assert_eq!(back, sent);
    assert_eq!(pulled.batch, batch, "identity and recorded time too");
}

#[tokio::test]
async fn replaying_a_pushed_batch_changes_nothing() {
    let (peer, store) = remote().await;
    let batch = awkward_batch();

    let first = peer.push(&batch).await.expect("the first push");
    let before = log(&store).await;
    let replay = peer.push(&batch).await.expect("the replay");

    assert_eq!((first.appended, first.skipped), (3, 0));
    assert_eq!((replay.appended, replay.skipped), (0, 3));
    assert_eq!(replay.confirmed, first.confirmed);
    assert_eq!(log(&store).await, before);
}

/// The token the client holds is the far side's text, and resuming from it
/// sends nothing twice.
///
/// The text names the first position of the last group forwarded (#420): the
/// batch is one group of three, which the dense in-memory store wrote from
/// position 1, so the group's first is 1 and not its last row's 3.
#[tokio::test]
async fn the_remote_token_resumes_where_the_far_side_left_off() {
    let (peer, _) = remote().await;
    peer.push(&awkward_batch()).await.expect("a push");

    let first = peer.pull(None).await.expect("a first pull");
    let second = peer.pull(Some(&first.resume)).await.expect("a second pull");

    assert_eq!(first.resume, ResumeToken::new("1"));
    assert_eq!(second.batch, PushBatch::empty());
}

/// The far store's identity crosses the envelope on both replies (ADR-0104).
///
/// **Wrong implementation it rejects:** a `serve_sync` that rebuilds the
/// pulled reply through `Pulled::new` and so drops the responder on the way
/// onto the wire, which no in-process test of `StoreSyncPeer` can see.
#[tokio::test]
async fn the_responder_survives_the_envelope_on_pull_and_push() {
    let (peer, store) = remote().await;
    let named = IngestStore::store_id(&store)
        .await
        .expect("the in-memory store's error is uninhabited");

    let ack = peer.push(&awkward_batch()).await.expect("a push");
    let pulled = peer.pull(None).await.expect("a pull");

    assert_eq!(ack.responder, Some(named), "the ack");
    assert_eq!(pulled.responder, Some(named), "the pulled reply");
}

/// Over a `Send` exchange the pull's future is `Send`: a runner can hand it to
/// `tokio::spawn`, which refuses anything else at compile time (#421).
#[tokio::test]
async fn a_remote_pull_over_a_send_exchange_can_be_spawned() {
    let (peer, _) = remote().await;
    peer.push(&awkward_batch()).await.expect("a push");

    let pulled = tokio::spawn(async move { peer.pull(None).await })
        .await
        .expect("the spawned pull ran to completion")
        .expect("a pull");

    assert_eq!(pulled.batch, awkward_batch());
}

#[tokio::test]
async fn a_token_the_far_side_never_minted_is_stop_through_the_exchange() {
    let (peer, _) = remote().await;

    let refused = peer
        .pull(Some(&ResumeToken::new("group:42")))
        .await
        .expect_err("a foreign token");

    assert!(matches!(
        refused,
        RemoteError::Exchange(ServeError::Token(ref foreign))
            if foreign.token().as_str() == "group:42"
    ));
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// A body from a peer one format version ahead is refused by number, as a
/// typed error, and the store is never reached.
#[tokio::test]
async fn serve_sync_refuses_another_format_version_as_a_typed_error() {
    let store = MemoryIngestFixture::new();
    let far = StoreSyncPeer::new(store.connect().await, store.connect().await);
    // Version 999 as a postcard varint, then nothing that decodes.
    let body = [0xE7, 0x07, 0xFF];

    for route in [Route::Pull, Route::Push] {
        let refused = serve_sync(&far, route, &body)
            .await
            .expect_err("version 999");

        assert!(
            matches!(
                refused,
                ServeError::Codec(CodecError::UnsupportedVersion(
                    WireError::UnsupportedFormatVersion {
                        found: 999,
                        supported: FORMAT_VERSION,
                        ..
                    }
                ))
            ),
            "{route}: {refused:?}"
        );
        assert_eq!(refused.class(), FailureClass::Stop);
    }
    assert_eq!(log(&store.connect().await).await, Vec::new());
}

#[tokio::test]
async fn serve_sync_refuses_bytes_that_are_no_envelope() {
    let store = MemoryIngestFixture::new();
    let far = StoreSyncPeer::new(store.connect().await, store.connect().await);

    let refused = serve_sync(&far, Route::Push, &[])
        .await
        .expect_err("an empty body");

    assert!(matches!(
        refused,
        ServeError::Codec(CodecError::Malformed {
            route: Route::Push,
            ..
        })
    ));
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// One fresh in-memory store per fixture; each `connect` is a new client over
/// a new server-side peer onto it.
#[derive(Debug, Default)]
struct RemoteFixture {
    store: MemoryIngestFixture,
}

impl SyncFixture for RemoteFixture {
    type Peer = RemoteSyncPeer<Loopback>;

    async fn connect(&self) -> Self::Peer {
        let far = StoreSyncPeer::with_limits(
            self.store.connect().await,
            self.store.connect().await,
            PeerLimits::minimum(),
        );
        RemoteSyncPeer::new(Loopback { far }, PeerLimits::minimum())
    }

    /// The far side's oversized-push refusal, carried back through the
    /// exchange with its class intact.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        oversized_push_refusal(peer).await
    }
}

happenstance_sync_testkit::sync_peer_conformance!(super::RemoteFixture::default());
