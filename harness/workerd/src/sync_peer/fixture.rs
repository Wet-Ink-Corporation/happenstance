//! The sync fixture over one Durable Object's storage, and the client it
//! hands each rule.
//!
//! # What is and is not the network
//!
//! [`ObjectExchange`] builds the `POST` an HTTP client sends — a real
//! `worker::Request`, its body a real `ReadableStream` on this runtime — hands
//! it to the handler the object mounts at `/sync/<route>`, and reads the real
//! `worker::Response` back: the status through `class_for`, as a client over
//! a socket reads it, and the body as bytes. What it leaves out is the socket
//! between the two. That hop is the round trip's subject
//! (`tests/sqlite_round_trip.rs`), which drives the same handler from a native
//! client over real HTTP, locally and on the deployed object.

use happenstance_cloudflare::happenstance_sync::wire::Route;
use happenstance_cloudflare::happenstance_sync::{
    Classify, Exchange, FailureClass, PeerLimits, RemoteSyncPeer, StoreSyncPeer, SyncPeer as _,
};
use happenstance_cloudflare::sync_http::{
    CloudflareSyncPeer, SYNC_PATH_PREFIX, class_for, serve_sync_request,
};
use happenstance_cloudflare::worker::js_sys::Uint8Array;
use happenstance_cloudflare::worker::{self, Method, Request, RequestInit};
use happenstance_sync_testkit::{
    IngestFixture as _, Refusal, RefusalsOf, SyncFixture, batch_over_the_limit,
    oversized_push_refusal,
};
use happenstance_testkit::Capability;

use crate::ingest::fixture::WorkerdIngestFixture;

/// The reason [`WorkerdSyncFixture`] declines
/// [`COMPACTION`](SyncFixture::COMPACTION), in the harness's own words.
///
/// `pub` because it is the one skip the `workerd` runner tolerates:
/// `test/sync-peer.test.ts` carries this text word for word, and the host test
/// in `crate::sync_peer` holds the two together (D10, #158, #480).
pub const COMPACTION_DECLINED: &str = "the object serves a StoreSyncPeer over CloudflareIngestStore, and a store peer \
     cannot read a retention floor through the port (ADR-0028), so the object has no compaction \
     to offer";

/// One isolated log inside the object, reached as a remote peer.
///
/// One instance per ingest fixture, so per namespace: a rule's fixtures see
/// none of each other's events. Each [`connect`](SyncFixture::connect) is a new
/// client over a new server-side peer onto the same log.
#[derive(Debug, Clone)]
pub struct WorkerdSyncFixture {
    /// The log, built by the published constructor over the object's state.
    ingest: WorkerdIngestFixture,
}

impl WorkerdSyncFixture {
    /// A peer fixture over `ingest`'s log.
    #[must_use]
    pub const fn new(ingest: WorkerdIngestFixture) -> Self {
        Self { ingest }
    }

    /// A client declaring `declared` for a far side whose own limits are
    /// [`PeerLimits::minimum`], as the object's own routes declare.
    async fn client(&self, declared: PeerLimits) -> RemoteSyncPeer<ObjectExchange> {
        let store = self.ingest.connect().await;
        // Two handles onto one store: a clone is two `Rc` bumps.
        let far = StoreSyncPeer::with_limits(store.clone(), store, PeerLimits::minimum());
        RemoteSyncPeer::new(ObjectExchange { far }, declared)
    }
}

impl SyncFixture for WorkerdSyncFixture {
    type Peer = RemoteSyncPeer<ObjectExchange>;

    /// Declined: every `connect` already builds a new client over a store
    /// re-derived from the object's state, so the handle swap SY-31's rule
    /// makes is all a reopen here could add, and the restart the capability
    /// stands for is beyond a runner inside one request. The restarts this
    /// harness does drive are phased across two requests, in `crate::restart`.
    const REOPEN: Capability = Capability::declined(
        "each connect builds a new client over a store re-derived from the object's state, so \
         the handle swap SY-31's rule already makes is all a reopen here could do; restarting \
         the object, the crash the capability stands for, is beyond a runner inside one \
         request, and the restarts this harness drives are phased across two (crate::restart)",
    );

    /// Declined, in [`COMPACTION_DECLINED`]'s words: the object serves a
    /// `StoreSyncPeer`, which cannot read a retention floor through the port
    /// (ADR-0028), so it has no compaction to offer until one is built for it.
    const COMPACTION: Capability = Capability::declined(COMPACTION_DECLINED);

    /// [`PeerLimits::minimum`] at both ends, as the object's own routes
    /// declare: the limits a client must be told out of band (ADR-0097).
    async fn connect(&self) -> Self::Peer {
        self.client(PeerLimits::minimum()).await
    }

    /// Two oversized pushes, both `Split`. The first, over the limits both
    /// ends declare, is refused by the client before it sends (#452). The
    /// second comes from a client declaring twice the far side's
    /// `max_batch_bytes`, so it is sent: the object's handler hands it to the
    /// `StoreSyncPeer`, whose refusal leaves as a 413 by `status_for` and
    /// comes back as `Split` through `class_for`, inside a real object.
    async fn refusals(&self, peer: &Self::Peer) -> RefusalsOf<Self::Peer> {
        let mut refusals = oversized_push_refusal(peer).await?;
        let mut wider = PeerLimits::minimum();
        wider.max_batch_bytes = 2 * PeerLimits::minimum().max_batch_bytes;
        let oversized = batch_over_the_limit(&PeerLimits::minimum())?;
        refusals.push(Refusal {
            provoked_by: "a push the client's wider limits admit and the far side refuses",
            declared: FailureClass::Split,
            outcome: self.client(wider).await.push(&oversized).await.map(drop),
        });
        Ok(refusals)
    }
}

/// The client with the socket taken out: a request built as an HTTP client
/// builds it, answered by the object's own handler.
#[derive(Debug, Clone)]
pub struct ObjectExchange {
    /// The server-side peer the object's `/sync/<route>` serves.
    far: CloudflareSyncPeer,
}

impl Exchange for ObjectExchange {
    type Error = ObjectExchangeError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        let request = post(route, &request)?;
        let mut response = serve_sync_request(&self.far, route, request).await?;
        let status = response.status_code();
        if status == 200 {
            return Ok(response.bytes().await?);
        }
        Err(ObjectExchangeError::Refused {
            status,
            class: class_for(status),
            body: response.text().await?,
        })
    }
}

/// A `POST` of `body` to `/sync/<route>`, as a client sends it.
fn post(route: Route, body: &[u8]) -> worker::Result<Request> {
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_body(Some(Uint8Array::from(body).into()));
    Request::new_with_init(&format!("https://harness{SYNC_PATH_PREFIX}{route}"), &init)
}

/// How an [`ObjectExchange`] fails.
#[derive(Debug)]
pub enum ObjectExchangeError {
    /// The handler answered a status other than 200, read back as its class.
    Refused {
        /// The status.
        status: u16,
        /// What `class_for` reads from it.
        class: FailureClass,
        /// The handler's text: the refusal's error and its sources.
        body: String,
    },
    /// The runtime would not build the request or read the response: a
    /// transport failure, so `Transient`.
    Worker(worker::Error),
}

impl From<worker::Error> for ObjectExchangeError {
    fn from(error: worker::Error) -> Self {
        Self::Worker(error)
    }
}

impl core::fmt::Display for ObjectExchangeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Refused {
                status,
                class,
                body,
            } => write!(f, "the object answered {status} ({class:?}): {body}"),
            Self::Worker(error) => write!(f, "the request or its response failed: {error}"),
        }
    }
}

impl core::error::Error for ObjectExchangeError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Refused { .. } => None,
            Self::Worker(error) => Some(error),
        }
    }
}

impl Classify for ObjectExchangeError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Refused { class, .. } => *class,
            Self::Worker(_) => FailureClass::Transient,
        }
    }
}
