//! The native client: an [`Exchange`] over HTTP `POST`, carrying postcard
//! envelopes to the harness Worker's `/do/<object>/sync/<route>`.
//!
//! What a production client owes the port, each kept here: one round trip per
//! exchange; a timeout on every round trip ([`ROUND_TRIP_TIMEOUT`]); a reply
//! read under a cap rather than buffered whole; and every failure stating its
//! class (SY-36) — a status through `class_for`, and a failure to get one at
//! all, a refused connection or a timeout, as `Transient`.
//!
//! One more, because the deployed leg reaches the object through Cloudflare's
//! edge: the platform's own answers while a fresh Worker lands
//! (`platform-miss.mjs`, matched exactly as it matches them) are
//! [`HttpError::PlatformMiss`] and `Transient`, read before `class_for` is
//! consulted. Without it the edge's 404 page would read as `Stop`. The
//! object's own 404s are plain text and never match. [`retrying`] is the
//! small bounded retry the tests' direct calls go through; an [`Edge`] has
//! its own backoff.
//!
//! [`Exchange`]: happenstance_cloudflare::happenstance_sync::Exchange
//! [`Edge`]: happenstance_cloudflare::happenstance_sync::runner::Edge

use core::future::Future;
use core::time::Duration;
use std::sync::Arc;

use bytes::Bytes;
use happenstance_cloudflare::happenstance_sync::wire::Route;
use happenstance_cloudflare::happenstance_sync::{
    Classify, FailureClass, PeerLimits, SendExchange,
};
use happenstance_cloudflare::sync_http::{SYNC_PATH_PREFIX, class_for, max_request_bytes};
use http_body_util::{BodyExt as _, Full, LengthLimitError, Limited};
use hyper::Method;
use hyper::header::{AUTHORIZATION, CONTENT_TYPE};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;

/// How long one round trip may take before it is a `Transient` failure.
///
/// Generous for one pull or push of at most 64 KiB of payload: a deployed
/// object's first request includes its cold start.
pub(crate) const ROUND_TRIP_TIMEOUT: Duration = Duration::from_secs(30);

/// The client every exchange in a test binary shares the connector type of.
type HttpClient = Client<hyper_rustls::HttpsConnector<HttpConnector>, Full<Bytes>>;

/// One harness object, reached over HTTP with the harness's bearer token.
#[derive(Debug, Clone)]
pub(crate) struct HttpExchange {
    /// A pooled client: `Clone` is an `Arc` bump.
    client: HttpClient,
    /// `<base>/do/<object>`, which every route is appended to.
    object: String,
    /// The bearer token the Worker's router requires.
    token: String,
    /// The largest reply body read: the far side's batch and the envelope.
    max_reply: usize,
}

impl HttpExchange {
    /// An exchange with the object named `object` behind the Worker at
    /// `base`, presenting `token`, whose far side declares `limits`.
    ///
    /// # Errors
    ///
    /// [`HttpError::Roots`] if no platform CA store can be read.
    pub(crate) fn new(
        base: &str,
        object: &str,
        token: &str,
        limits: &PeerLimits,
    ) -> Result<Self, HttpError> {
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_native_roots(Arc::new(rustls::crypto::ring::default_provider()))
            .map_err(HttpError::Roots)?
            // The local leg is `wrangler dev` on plain HTTP; the deployed leg
            // is HTTPS.
            .https_or_http()
            .enable_http1()
            .build();
        Ok(Self {
            client: Client::builder(TokioExecutor::new()).build(connector),
            object: format!("{}/do/{object}", base.trim_end_matches('/')),
            token: token.to_owned(),
            max_reply: max_request_bytes(limits),
        })
    }

    /// One request to `path` under the object, bounded by
    /// [`ROUND_TRIP_TIMEOUT`]: the answer's body on a 200, and a
    /// [`HttpError::Refused`] carrying the status's class otherwise.
    ///
    /// # Errors
    ///
    /// [`HttpError`], each variant stating its class.
    pub(crate) async fn call(
        &self,
        method: Method,
        path: &str,
        body: Vec<u8>,
    ) -> Result<Vec<u8>, HttpError> {
        bounded(self.round_trip(method, path, body), ROUND_TRIP_TIMEOUT).await
    }

    /// [`call`](Self::call)'s body, without the bound.
    async fn round_trip(
        &self,
        method: Method,
        path: &str,
        body: Vec<u8>,
    ) -> Result<Vec<u8>, HttpError> {
        let request = hyper::Request::builder()
            .method(method)
            .uri(format!("{}{path}", self.object))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(CONTENT_TYPE, "application/octet-stream")
            .body(Full::new(Bytes::from(body)))
            .map_err(HttpError::Request)?;
        let response = self
            .client
            .request(request)
            .await
            .map_err(HttpError::Transport)?;
        let status = response.status().as_u16();
        let body = Limited::new(response.into_body(), self.max_reply)
            .collect()
            .await
            .map_err(|error| {
                if error.is::<LengthLimitError>() {
                    HttpError::ReplyTooLarge {
                        cap: self.max_reply,
                    }
                } else {
                    HttpError::Body(error)
                }
            })?
            .to_bytes();
        if status == 200 {
            // Reuses the buffer when the body is its only handle.
            return Ok(Vec::from(body));
        }
        Err(refusal(status, String::from_utf8_lossy(&body).into_owned()))
    }
}

/// What a status other than 200 with `body` reads as: a
/// [`HttpError::PlatformMiss`] when it is one of the platform's own answers,
/// and otherwise a [`HttpError::Refused`] carrying `class_for`'s class.
pub(crate) fn refusal(status: u16, body: String) -> HttpError {
    if is_platform_miss(status, &body) {
        HttpError::PlatformMiss { status, body }
    } else {
        HttpError::Refused {
            status,
            class: class_for(status),
            body,
        }
    }
}

/// `platform-miss.mjs`'s `isPlatformMiss`, signature for signature: exact, so
/// that no answer the object or the harness writes is ever taken for one.
fn is_platform_miss(status: u16, body: &str) -> bool {
    match status {
        500 => {
            matches!(
                body.trim(),
                "Worker not found." | "Durable Object reset because its code was updated."
            ) || body.contains(r#"<span class="cf-error-code">1104</span>"#)
        }
        404 => body.contains("<h1>There is nothing here yet</h1>"),
        _ => false,
    }
}

/// How many times [`retrying`] tries before it returns the last failure.
pub(crate) const RETRY_ATTEMPTS: usize = 3;

/// The pause before the second attempt; each later pause doubles it.
const RETRY_FIRST_PAUSE: Duration = Duration::from_secs(2);

/// Runs `attempt` until it answers with anything but a `Transient` failure, or
/// [`RETRY_ATTEMPTS`] times, pausing between attempts; and returns the last
/// outcome.
///
/// For the tests' direct calls — the reset, and a push or a pull made outside
/// an [`Edge`](happenstance_cloudflare::happenstance_sync::runner::Edge) — so a
/// platform miss on the deployed leg does not fail a test the object never
/// saw. Bounded, so a far side that is down stays a failure.
pub(crate) async fn retrying<T, E: Classify>(
    mut attempt: impl AsyncFnMut() -> Result<T, E>,
) -> Result<T, E> {
    let mut pause = RETRY_FIRST_PAUSE;
    let mut outcome = attempt().await;
    for _ in 1..RETRY_ATTEMPTS {
        match &outcome {
            Err(error) if error.class() == FailureClass::Transient => {}
            _ => break,
        }
        tokio::time::sleep(pause).await;
        pause = pause.saturating_mul(2);
        outcome = attempt().await;
    }
    outcome
}

impl SendExchange for HttpExchange {
    type Error = HttpError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, Self::Error> {
        self.call(Method::POST, &format!("{SYNC_PATH_PREFIX}{route}"), request)
            .await
    }
}

/// Runs `round_trip` to an answer, a failure or `limit`, whichever is first.
pub(crate) async fn bounded<T>(
    round_trip: impl Future<Output = Result<T, HttpError>>,
    limit: Duration,
) -> Result<T, HttpError> {
    tokio::time::timeout(limit, round_trip)
        .await
        .unwrap_or(Err(HttpError::Timeout { after: limit }))
}

/// How an [`HttpExchange`] fails.
#[derive(Debug)]
pub(crate) enum HttpError {
    /// No platform CA store could be read. `Stop`: configuration.
    Roots(std::io::Error),
    /// The request could not be built. `Stop`: it would not build again.
    Request(hyper::http::Error),
    /// No answer: a refused connection, DNS, TLS, a reset. `Transient`.
    Transport(hyper_util::client::legacy::Error),
    /// The answer's body broke off. `Transient`.
    Body(Box<dyn core::error::Error + Send + Sync>),
    /// No answer within the bound. `Transient`.
    Timeout {
        /// The bound.
        after: Duration,
    },
    /// The answer's body was longer than the far side's limits allow. `Stop`:
    /// the two ends are configured differently.
    ReplyTooLarge {
        /// The cap.
        cap: usize,
    },
    /// The platform answered, not the object: one of `platform-miss.mjs`'s
    /// signatures, raised while a fresh Worker lands. `Transient`.
    PlatformMiss {
        /// The status.
        status: u16,
        /// The platform's page or message.
        body: String,
    },
    /// The answer was a status other than 200, read back as its class.
    Refused {
        /// The status.
        status: u16,
        /// What `class_for` reads from it.
        class: FailureClass,
        /// The answer's text.
        body: String,
    },
}

impl core::fmt::Display for HttpError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Roots(error) => write!(f, "no platform CA store: {error}"),
            Self::Request(error) => write!(f, "the request did not build: {error}"),
            Self::Transport(error) => write!(f, "no answer: {error}"),
            Self::Body(error) => write!(f, "the answer broke off: {error}"),
            Self::Timeout { after } => write!(f, "no answer within {after:?}"),
            Self::ReplyTooLarge { cap } => write!(f, "the answer is over {cap} bytes"),
            Self::PlatformMiss { status, body } => {
                write!(f, "the platform answered {status}, not the object: {body}")
            }
            Self::Refused {
                status,
                class,
                body,
            } => write!(f, "the object answered {status} ({class:?}): {body}"),
        }
    }
}

impl core::error::Error for HttpError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Roots(error) => Some(error),
            Self::Request(error) => Some(error),
            Self::Transport(error) => Some(error),
            Self::Body(error) => Some(error.as_ref()),
            Self::Timeout { .. }
            | Self::ReplyTooLarge { .. }
            | Self::PlatformMiss { .. }
            | Self::Refused { .. } => None,
        }
    }
}

/// A transport failure is `Transient` (the owner's default for #130), and so
/// is a platform miss; a status is its class; anything that would fail the
/// same way again is `Stop`.
impl Classify for HttpError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Transport(_)
            | Self::Body(_)
            | Self::Timeout { .. }
            | Self::PlatformMiss { .. } => FailureClass::Transient,
            Self::Refused { class, .. } => *class,
            Self::Roots(_) | Self::Request(_) | Self::ReplyTooLarge { .. } => FailureClass::Stop,
        }
    }
}
