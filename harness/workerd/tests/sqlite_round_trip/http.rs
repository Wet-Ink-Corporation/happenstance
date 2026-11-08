//! The native client: an [`Exchange`] over HTTP `POST`, carrying postcard
//! envelopes to the harness Worker's `/do/<object>/sync/<route>`.
//!
//! What a production client owes the port, each kept here: one round trip per
//! exchange; a timeout on every round trip ([`ROUND_TRIP_TIMEOUT`]); a reply
//! read under a cap rather than buffered whole; and every failure stating its
//! class (SY-36) — a status through `class_for`, and a failure to get one at
//! all, a refused connection or a timeout, as `Transient`.
//!
//! [`Exchange`]: happenstance_cloudflare::happenstance_sync::Exchange

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
        Err(HttpError::Refused {
            status,
            class: class_for(status),
            body: String::from_utf8_lossy(&body).into_owned(),
        })
    }
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
            Self::Timeout { .. } | Self::ReplyTooLarge { .. } | Self::Refused { .. } => None,
        }
    }
}

/// A transport failure is `Transient` (the owner's default for #130); a
/// status is its class; anything that would fail the same way again is `Stop`.
impl Classify for HttpError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Transport(_) | Self::Body(_) | Self::Timeout { .. } => FailureClass::Transient,
            Self::Refused { class, .. } => *class,
            Self::Roots(_) | Self::Request(_) | Self::ReplyTooLarge { .. } => FailureClass::Stop,
        }
    }
}
