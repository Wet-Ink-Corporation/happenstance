//! The Durable Object as a sync peer over HTTP (#130): `serve_sync` behind an
//! object's fetch handler.
//!
//! # What is here
//!
//! [`serve_sync_request`] answers one `POST` on [`SYNC_PATH_PREFIX`]`pull` or
//! `push` through a [`CloudflareSyncPeer`]: it reads the body under a cap,
//! hands the bytes to [`serve_sync`], and answers with the reply envelope or
//! with the status its failure's class maps to. Everything between the bytes
//! and the store — the format version, the decode, the ingest, the encode — is
//! `happenstance-sync`'s, so this object answers with the same bytes as every
//! other transport. An object mounts it in its own `fetch`:
//!
//! ```no_run
//! use happenstance_cloudflare::sync_http::{CloudflareSyncPeer, serve_sync_request, sync_route};
//! use happenstance_cloudflare::worker::{Request, Response, Result};
//! use happenstance_cloudflare::happenstance_sync::StoreSyncPeer;
//! use happenstance_cloudflare::CloudflareIngestStore;
//!
//! async fn fetch(store: CloudflareIngestStore, request: Request) -> Result<Response> {
//!     let Some(route) = sync_route(&request.path()) else {
//!         return Response::error("no such route", 404);
//!     };
//!     // Two handles onto one store: a clone is two `Rc` bumps.
//!     let peer: CloudflareSyncPeer = StoreSyncPeer::new(store.clone(), store);
//!     serve_sync_request(&peer, route, request).await
//! }
//! ```
//!
//! # Status codes are classes
//!
//! The wire set carries no error message (ADR-0097), so a refusal travels as
//! an HTTP status, and [`status_for`] and [`class_for`] are the one table both
//! ends read: `Transient` 503, `Split` 413, `Park` 422, `Stop` 400. A client
//! reads any other status through [`class_for`] too, which is how a platform
//! failure (a 500 from a trapped instance, a 502 from the edge) arrives as
//! `Transient` and a refused bearer token (403) as `Stop`.
//!
//! # Both halves are the ingest store
//!
//! [`CloudflareSyncPeer`] reads and ingests through [`CloudflareIngestStore`],
//! not through a `CloudflareEventStore` beside it: the write-group read a
//! pull is served from is implemented on the ingest store, and a peer built
//! from one value cloned cannot have its pushes land in a log its pulls never
//! read, which [`StoreSyncPeer`]'s documentation asks of every caller.
//!
//! # What is not here
//!
//! A client. A native client is any HTTP library behind an
//! [`Exchange`](happenstance_sync::Exchange); `harness/workerd`'s round trip
//! uses `hyper`. A client *inside* a Worker, over a Durable Object stub, is
//! not built yet.

use core::future::poll_fn;
use core::pin::pin;

use futures_core::Stream;
use happenstance_sync::wire::Route;
use happenstance_sync::{
    Classify, FailureClass, IngestStore, PeerLimits, ServeError, StoreSyncPeer, SyncPeer,
    WriteGroupStore, serve_sync,
};
use worker::{Method, Request, Response};

use crate::event_store::CloudflareIngestStore;

/// The peer an object serves: both halves are its ingest store, two handles
/// onto one log (the [module](self) documentation says why).
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub type CloudflareSyncPeer = StoreSyncPeer<CloudflareIngestStore, CloudflareIngestStore>;

/// The path the two routes share: `/sync/pull` and `/sync/push`, each the
/// prefix and [`Route::as_str`], which `happenstance-sync` keeps stable for
/// exactly this use.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub const SYNC_PATH_PREFIX: &str = "/sync/";

/// How many bytes a request may carry beyond the payload bytes its batch is
/// limited to: 1 MiB.
///
/// [`PeerLimits::max_batch_bytes`] counts payloads, and the envelope also
/// carries each event's identity, type, tags and the groups' guards. 1 MiB is
/// 8 KiB for each of the 128 events [`PeerLimits::minimum`] admits, which a
/// conformant event's type and tags do not approach. A batch whose envelope
/// needs more is refused as `Split`, and the client halves it.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub const ENVELOPE_ALLOWANCE: usize = 1_048_576;

/// The route `path` names: [`Route::Pull`] for `/sync/pull`, [`Route::Push`]
/// for `/sync/push`, and `None` for anything else.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#[must_use]
pub fn sync_route(path: &str) -> Option<Route> {
    match path.strip_prefix(SYNC_PATH_PREFIX)? {
        "pull" => Some(Route::Pull),
        "push" => Some(Route::Push),
        _ => None,
    }
}

/// The largest body [`serve_sync_request`] reads for a peer declaring
/// `limits`: the batch's payload bytes and [`ENVELOPE_ALLOWANCE`], saturating
/// rather than wrapping for a peer that declares `usize::MAX`.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#[must_use]
pub const fn max_request_bytes(limits: &PeerLimits) -> usize {
    limits.max_batch_bytes.saturating_add(ENVELOPE_ALLOWANCE)
}

/// The status a refusal of class `class` answers with.
///
/// [`FailureClass`] is `#[non_exhaustive]`; a class this build does not know
/// answers 400, so a client stops rather than retrying something nobody here
/// said was safe to retry.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#[must_use]
pub const fn status_for(class: FailureClass) -> u16 {
    match class {
        FailureClass::Transient => 503,
        FailureClass::Split => 413,
        FailureClass::Park => 422,
        _ => 400,
    }
}

/// The class a client reads from a status other than 200.
///
/// The inverse of [`status_for`] on its four statuses. Of the rest, 408, 429
/// and every 5xx are `Transient` — the platform's own failures land there, and
/// a transport failure is transient — and every other status is `Stop`: a
/// refused token, a missing route or a wrong method fails the same way again.
///
/// The table reads statuses this module's server answers. A client reaching an
/// object through Cloudflare's edge also meets answers the platform gives
/// before any object code runs — a 404 "There is nothing here yet" page, a 500
/// "Worker not found.", a 500 page carrying error code 1104 — and a 404 among
/// them would read here as `Stop`. Such a client recognises the platform's own
/// pages first, by exact match, and treats them as `Transient` before it
/// consults this table; `harness/workerd`'s round-trip client,
/// `HttpExchange`, does exactly that.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
#[must_use]
pub const fn class_for(status: u16) -> FailureClass {
    match status {
        413 => FailureClass::Split,
        422 => FailureClass::Park,
        408 | 429 | 500..=599 => FailureClass::Transient,
        _ => FailureClass::Stop,
    }
}

/// Answers one sync request on `route` through `peer`.
///
/// Takes the request by value because it consumes the body.
///
/// * A method other than `POST` answers 405, with `Allow: POST`.
/// * A body over [`max_request_bytes`] of the peer's limits answers 413
///   (`Split`): refused on its declared `Content-Length` before any of it is
///   read, and otherwise the moment the bytes read pass the cap, so an
///   untrusted body is never buffered past it.
/// * A request `serve_sync` answers is a 200 carrying the reply envelope as
///   `application/octet-stream`.
/// * A request `serve_sync` refuses answers [`status_for`] its class, with the
///   error's text as the body.
///
/// That text is the whole error chain, the store's own errors included, so
/// that a client can say why it was refused. It is written for an operator,
/// not a stranger: mount this behind authentication, as `harness/workerd`
/// mounts it behind a bearer token, rather than on a public route.
///
/// # Errors
///
/// A [`worker::Error`] if the body cannot be read or the response cannot be
/// built: the platform answers 500 for it, which [`class_for`] reads as
/// `Transient`.
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub async fn serve_sync_request(
    peer: &CloudflareSyncPeer,
    route: Route,
    mut request: Request,
) -> worker::Result<Response> {
    if request.method() != Method::Post {
        return Response::builder()
            .with_status(405)
            .with_header("allow", "POST")?
            .error("a sync request is a POST", 405);
    }
    let cap = max_request_bytes(&peer.limits());
    let Some(body) = read_capped(&mut request, cap).await? else {
        return Response::error(
            format!("the request body is over the {cap} bytes this peer reads"),
            status_for(FailureClass::Split),
        );
    };
    match serve_sync(peer, route, &body).await {
        Ok(reply) => Response::from_bytes(reply),
        Err(error) => Response::error(text_of(&error), status_for(error.class())),
    }
}

/// The request's body, or `None` once it is longer than `cap`.
async fn read_capped(request: &mut Request, cap: usize) -> worker::Result<Option<Vec<u8>>> {
    // A header that does not parse is ignored rather than refused: it is only
    // an early answer, and the streaming cap below still bounds the read.
    let declared = request
        .headers()
        .get("content-length")?
        .and_then(|length| length.parse::<usize>().ok());
    if declared.is_some_and(|length| length > cap) {
        return Ok(None);
    }
    // `Request::stream` refuses a request with no body at all, which is an
    // empty body here: `serve_sync` refuses it as malformed, by its class.
    if request.inner().body().is_none() {
        return Ok(Some(Vec::new()));
    }
    let mut stream = pin!(request.stream()?);
    // Not pre-sized from `Content-Length`: the header is the client's claim,
    // and a cap from a peer declaring large limits would let it reserve that
    // much before sending a byte (H-13).
    let mut body = Vec::new();
    while let Some(chunk) = poll_fn(|context| stream.as_mut().poll_next(context)).await {
        let chunk = chunk?;
        if body.len().saturating_add(chunk.len()) > cap {
            return Ok(None);
        }
        body.extend_from_slice(&chunk);
    }
    Ok(Some(body))
}

/// The error's message and each source's after it, so a client reading the
/// body sees the store's own words and not only `serve_sync`'s summary.
fn text_of(
    error: &ServeError<
        <CloudflareIngestStore as WriteGroupStore>::Error,
        <CloudflareIngestStore as IngestStore>::Error,
    >,
) -> String {
    let mut text = error.to_string();
    let mut source = core::error::Error::source(error);
    while let Some(cause) = source {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        source = cause.source();
    }
    text
}

#[cfg(test)]
mod tests;
