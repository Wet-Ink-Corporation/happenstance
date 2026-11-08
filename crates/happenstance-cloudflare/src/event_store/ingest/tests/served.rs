//! The object as a sync peer over HTTP (#130), on the shim: every request a
//! real `worker::Request`, every answer a real `worker::Response`, and the
//! store behind them the ingest store over the model's `transactionSync`.
//!
//! [`RequestExchange`] is the client with the network taken out: it builds
//! the `POST` an HTTP client would send, hands it to `serve_sync_request`, and
//! reads the status back through `class_for`, as a client over a socket does.
//! The sync family is mounted at the bottom over it. `harness/workerd` runs the
//! same family on a real object, and the round trip over a real socket.

use core::cell::RefCell;
use core::future::Future;

use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::wire::{ResumeToken, Route};
use happenstance_sync::{
    Classify, EventGroup, Exchange, FailureClass, PeerLimits, PushBatch, RemoteError,
    RemoteSyncPeer, ReplicatedEvent, StoreSyncPeer, SyncPeer,
};
use happenstance_sync_testkit::{RefusalsOf, SyncFixture, oversized_push_refusal};
use wasm_bindgen_test::wasm_bindgen_test;
use worker::js_sys::Uint8Array;
use worker::{Method, Request, RequestInit, Response};

use super::support::{log, open};
use crate::event_store::CloudflareEventStore;
use crate::sync_http::{
    CloudflareSyncPeer, SYNC_PATH_PREFIX, class_for, max_request_bytes, serve_sync_request,
};

/// The client: a request built as an HTTP client builds it, answered in
/// process.
#[derive(Debug, Clone)]
struct RequestExchange {
    far: CloudflareSyncPeer,
}

/// A refusal read back from the status, or a request the runtime would not
/// build.
#[derive(Debug, thiserror::Error)]
enum RequestError {
    #[error("the object answered {status} ({class:?}): {body}")]
    Refused {
        status: u16,
        class: FailureClass,
        body: String,
    },
    #[error("the request or the response failed: {0}")]
    Worker(#[from] worker::Error),
}

impl Classify for RequestError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Refused { class, .. } => *class,
            Self::Worker(_) => FailureClass::Transient,
        }
    }
}

impl Exchange for RequestExchange {
    type Error = RequestError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        let request = post(&format!("{SYNC_PATH_PREFIX}{route}"), &request)?;
        let mut response = serve_sync_request(&self.far, route, request).await?;
        let status = response.status_code();
        if status == 200 {
            return Ok(response.bytes().await?);
        }
        Err(RequestError::Refused {
            status,
            class: class_for(status),
            body: response.text().await?,
        })
    }
}

/// A `POST` of `body` to `path` on the object.
fn post(path: &str, body: &[u8]) -> worker::Result<Request> {
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_body(Some(Uint8Array::from(body).into()));
    Request::new_with_init(&format!("https://object{path}"), &init)
}

/// A remote peer over a fresh object, and a handle onto the same store.
fn remote() -> (RemoteSyncPeer<RequestExchange>, CloudflareEventStore) {
    let opened = open();
    let far = StoreSyncPeer::new(opened.store.clone(), opened.store);
    let limits = far.limits();
    (
        RemoteSyncPeer::new(RequestExchange { far }, limits),
        CloudflareEventStore::new(opened.sql),
    )
}

/// Payloads no text codec would leave alone: invalid UTF-8, a NUL, every byte
/// value, and metadata beside the data, in one group of three.
fn awkward_batch() -> PushBatch {
    let origin = StoreId::from_bytes([0x73; 16]);
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

/// A [`RequestExchange`] that keeps a copy of the last request body it sent.
struct Recording {
    inner: RequestExchange,
    sent: RefCell<Vec<u8>>,
}

impl Exchange for Recording {
    type Error = RequestError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        // A copy, because the body is consumed by the request and the test
        // sends the same bytes again.
        self.sent.replace(request.clone());
        self.inner.exchange(route, request).await
    }
}

/// The bytes of a push of [`awkward_batch`], as a client sends them, proved
/// valid by landing on a throwaway object first.
async fn a_valid_push_body() -> Vec<u8> {
    let (throwaway, _) = remote();
    let limits = throwaway.exchange().far.limits();
    let recording = RemoteSyncPeer::new(
        Recording {
            inner: throwaway.into_exchange(),
            sent: RefCell::new(Vec::new()),
        },
        limits,
    );
    let ack = recording
        .push(&awkward_batch())
        .await
        .expect("the push lands");
    assert_eq!(ack.appended, 3);
    recording.into_exchange().sent.into_inner()
}

/// The bytes go in through a request and come back out through a response,
/// identical, with their identity and recorded time; and a replay lands
/// nothing.
///
/// Rejects a binding that reads the body as text (the invalid UTF-8 would be
/// replaced), and one that answers with `Response::ok`'s string body.
#[wasm_bindgen_test]
async fn a_push_and_a_pull_through_requests_round_trip_byte_identical() {
    let (peer, store) = remote();
    let batch = awkward_batch();

    let first = peer.push(&batch).await.expect("a push inside the limits");
    let held = log(&store).await;
    let replay = peer.push(&batch).await.expect("the replay");
    let pulled = peer.pull(None).await.expect("a pull");

    assert_eq!((first.appended, first.skipped), (3, 0));
    assert_eq!((replay.appended, replay.skipped), (0, 3));
    assert_eq!(log(&store).await, held, "the replay changed nothing");
    assert_eq!(pulled.batch, batch, "bytes, identity and recorded time");
}

/// Rejects a binding that serves whatever method arrives, so a `GET` from a
/// crawler or a health check would run a decode against an empty body.
#[wasm_bindgen_test]
async fn a_request_that_is_not_a_post_is_refused_with_405() {
    let (peer, _) = remote();
    let far = &peer.exchange().far;
    let request = Request::new("https://object/sync/pull", Method::Get).expect("a GET");

    let response: Response = serve_sync_request(far, Route::Pull, request)
        .await
        .expect("an answer");

    assert_eq!(response.status_code(), 405);
    assert_eq!(
        response.headers().get("allow").expect("headers read"),
        Some("POST".to_owned())
    );
}

/// A body one byte over the cap is refused as `Split` before anything reads
/// it as an envelope, and the store holds nothing.
///
/// The body is no envelope at all, so a binding that buffered it and handed it
/// to `serve_sync` would answer 400 (`Stop`, malformed) instead of 413.
#[wasm_bindgen_test]
async fn a_body_over_the_cap_is_refused_as_split_before_the_store() {
    let (peer, store) = remote();
    let far = &peer.exchange().far;
    let cap = max_request_bytes(&far.limits());
    let request = post("/sync/push", &vec![0xAB; cap + 1]).expect("a POST");

    let mut response = serve_sync_request(far, Route::Push, request)
        .await
        .expect("an answer");

    assert_eq!(response.status_code(), 413);
    assert_eq!(class_for(response.status_code()), FailureClass::Split);
    assert!(
        response
            .text()
            .await
            .expect("a text body")
            .contains("over the"),
        "the body says why"
    );
    assert_eq!(log(&store).await, Vec::new());
}

/// A request that declares a `Content-Length` over the cap is refused as
/// `Split` on the header alone, and the store holds nothing.
///
/// The body itself is a valid push of one small batch, well under the cap, so
/// only the declared length can refuse it. Rejects a binding that drops the
/// declared-length check and trusts the streaming cap alone, which would read
/// this body, find it small, and land the batch with a 200.
#[wasm_bindgen_test]
async fn a_declared_length_over_the_cap_is_refused_before_the_body_is_read() {
    let body = a_valid_push_body().await;
    let (peer, store) = remote();
    let far = &peer.exchange().far;
    let cap = max_request_bytes(&far.limits());
    let headers = worker::Headers::new();
    headers
        .set("content-length", &cap.saturating_add(1).to_string())
        .expect("the header sets");
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(Uint8Array::from(body.as_slice()).into()));
    let request = Request::new_with_init("https://object/sync/push", &init).expect("a POST");

    let response = serve_sync_request(far, Route::Push, request)
        .await
        .expect("an answer");

    assert_eq!(response.status_code(), 413);
    assert_eq!(log(&store).await, Vec::new());
}

/// A `POST` with no body at all is an empty body, refused as malformed by its
/// class, and not a failure of the binding.
///
/// Rejects a binding that calls `Request::stream` unconditionally, which
/// fails on a request with no body and would reach the client as a 500
/// (`Transient`, retried forever) instead of a 400.
#[wasm_bindgen_test]
async fn a_post_with_no_body_answers_400() {
    let (peer, _) = remote();
    let far = &peer.exchange().far;
    let request = Request::new("https://object/sync/push", Method::Post).expect("a POST");

    let response = serve_sync_request(far, Route::Push, request)
        .await
        .expect("an answer");

    assert_eq!(response.status_code(), 400);
    assert_eq!(class_for(response.status_code()), FailureClass::Stop);
}

/// A refusal `serve_sync` makes keeps its class across the status: a token
/// this object never minted is `Stop`, and arrives at the client as `Stop`.
#[wasm_bindgen_test]
async fn a_foreign_token_answers_400_and_reads_back_as_stop() {
    let (peer, _) = remote();

    let refused = peer
        .pull(Some(&ResumeToken::new("group:42")))
        .await
        .expect_err("a token no store peer minted");

    assert!(
        matches!(
            refused,
            RemoteError::Exchange(RequestError::Refused { status: 400, ref body, .. })
                if body.contains("group:42")
        ),
        "{refused:?}"
    );
    assert_eq!(refused.class(), FailureClass::Stop);
}

/// One fresh object per fixture; each `connect` is a new client over a new
/// server-side peer onto it.
struct ServedFixture {
    opened: super::support::Opened,
}

impl SyncFixture for ServedFixture {
    type Peer = RemoteSyncPeer<RequestExchange>;

    fn connect(&self) -> impl Future<Output = Self::Peer> {
        let store = super::support::connect(&self.opened.host);
        let far = StoreSyncPeer::with_limits(store.clone(), store, PeerLimits::minimum());
        core::future::ready(RemoteSyncPeer::new(
            RequestExchange { far },
            PeerLimits::minimum(),
        ))
    }

    /// The store peer's oversized-push refusal, carried back through the
    /// status with its class intact.
    fn refusals(&self, peer: &Self::Peer) -> impl Future<Output = RefusalsOf<Self::Peer>> {
        oversized_push_refusal(peer)
    }
}

happenstance_sync_testkit::sync_peer_conformance!(
    mod_name = sync_conformance,
    emit = happenstance_sync_testkit::emit_sync_wasm,
    fixture = super::ServedFixture {
        opened: super::super::support::open()
    }
);
