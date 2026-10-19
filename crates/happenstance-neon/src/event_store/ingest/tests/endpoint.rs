//! The live endpoint the ingest tests share, and a fresh schema per store.
//!
//! `tests/support/` cannot be reached from here: each file under `tests/` is a
//! crate of its own, and this module's impls are `cfg(test)`, which is
//! invisible from there. So the client is restated, small, and its constants
//! must move with `tests/support/transport.rs`'s: the same environment
//! variable, the same header, the same round-trip bound. What it keeps of that
//! file is what makes it safe to point at a credential and at the fence: the
//! connection string is never printed, every read-only request is registered
//! in a [`ReadLedger`] before `round_trip` returns and settled by the task that
//! carries it, and no exchange outlives [`ROUND_TRIP_TIMEOUT`] (H-13).
//!
//! What it drops is the process-wide client and the runtime of its own. Each
//! store here gets a transport built inside its test's runtime and spawns onto
//! that runtime, so nothing outlives the test that made it, which that file's
//! process-wide pool cannot say.
//!
//! The schema sweep is restated the same way, for its own prefix: `hsi`, which
//! the conformance suite's sweep (`^hsp?_…`) never matches.
//!
//! # Why the functions here panic
//!
//! An endpoint that cannot be reached is a broken test environment, not an
//! adapter defect, and the message says so rather than letting a bare transport
//! error stand in for a finding.

use core::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use happenstance_core::bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::Uri;
use hyper::header::{CONTENT_TYPE, HeaderName, HeaderValue};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use tokio::sync::OnceCell;

use super::super::NeonIngestStore;
use crate::config::{NeonConfig, quote_ident};
use crate::event_store::NeonEventStore;
use crate::migration;
use crate::transport::{
    HttpResponse, IsolationLevel, ReadLedger, ReadTicket, SqlRequest, SqlStatement, SqlTransport,
};
use crate::wire::ResponseBody;

/// `tests/support/transport.rs`'s, which this must match.
const CONNECTION_ENV: &str = "NEON_CONNECTION";

/// `tests/support/transport.rs`'s header, lowercased as `hyper` wants it.
const CONNECTION_HEADER: &str = "neon-connection-string";

/// `tests/support/transport.rs`'s bound, for its reason: since the fence, a
/// read that never answers would hold every later write on its transport.
const ROUND_TRIP_TIMEOUT: Duration = Duration::from_secs(30);

/// The prefix of every schema these tests mint.
const SCHEMA_PREFIX: &str = "hsi";

/// `tests/support/mod.rs`'s age past which a minted schema is swept.
const SWEEP_AFTER_SECONDS: u64 = 60 * 60;

/// Sweeps once per process.
static SWEPT: OnceCell<()> = OnceCell::const_new();

/// The client this module sends through.
type HttpsClient = Client<hyper_rustls::HttpsConnector<HttpConnector>, Full<Bytes>>;

/// A one-shot HTTPS transport onto Neon's `/sql` endpoint.
///
/// Clones share one client and one [`ReadLedger`], so every store built over
/// clones of one transport is one ES-11 ordering domain.
#[derive(Clone)]
pub(super) struct LiveTransport {
    client: HttpsClient,
    uri: Uri,
    /// A credential. Never printed, never put in an error.
    connection: Arc<str>,
    ledger: ReadLedger,
}

// By hand, so the credential is never printed: a `dbg!` in a failing test must
// not be what puts a password in a CI log. The client is left out as noise.
impl core::fmt::Debug for LiveTransport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LiveTransport")
            .field("uri", &self.uri)
            .field("connection", &"<redacted>")
            .field("ledger", &self.ledger)
            .finish_non_exhaustive()
    }
}

impl LiveTransport {
    /// A transport onto the endpoint `NEON_CONNECTION` names.
    ///
    /// # Panics
    ///
    /// When the variable is absent or is not a URL, or no platform CA store
    /// can be read: a broken test environment, which these `#[ignore]`d tests
    /// report loudly rather than passing quietly over.
    pub(super) fn new() -> Self {
        let connection = std::env::var(CONNECTION_ENV).unwrap_or_else(|_| {
            panic!(
                "a broken test environment, not a non-conformant adapter: `{CONNECTION_ENV}` \
                 is not set, so there is no Neon endpoint. These tests are `#[ignore]`d so a \
                 machine without the secret still has a green gate."
            )
        });
        let connection = connection.trim();
        let uri: Uri = host_of(connection)
            .and_then(|host| format!("https://{host}/sql").parse().ok())
            .unwrap_or_else(|| {
                panic!(
                    "a broken test environment: `{CONNECTION_ENV}` is not a \
                     `postgresql://user:password@host/db` URL; its value is not printed"
                )
            });
        // The provider is named rather than installed process-wide, so nothing
        // here depends on which test installed one first. `ring`, for the
        // licence reason the manifest records.
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_native_roots(rustls::crypto::ring::default_provider())
            .unwrap_or_else(|error| {
                panic!("a broken test environment: no platform CA store could be read ({error})")
            })
            .https_only()
            .enable_http1()
            .enable_http2()
            .build();
        Self {
            client: Client::builder(TokioExecutor::new())
                .pool_max_idle_per_host(1)
                .build(connector),
            uri,
            connection: connection.into(),
            ledger: ReadLedger::new(),
        }
    }

    /// Builds and spawns one round trip onto the caller's runtime, now.
    fn dispatch(
        &self,
        request: &SqlRequest,
    ) -> Result<tokio::task::JoinHandle<Result<HttpResponse, LiveTransportError>>, LiveTransportError>
    {
        let runtime =
            tokio::runtime::Handle::try_current().map_err(|_| LiveTransportError::NoRuntime)?;
        let body = request.body().map_err(|_| LiveTransportError::Request)?;
        let mut builder = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(self.uri.clone())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(
                HeaderName::from_static(CONNECTION_HEADER),
                HeaderValue::from_str(&self.connection).map_err(|_| LiveTransportError::Request)?,
            );
        for (name, value) in request.headers() {
            builder = builder.header(name, value);
        }
        let http_request = builder
            .body(Full::new(Bytes::from(body)))
            .map_err(|_| LiveTransportError::Request)?;

        // Registered before `round_trip` returns, and moved into the task, so
        // the task ending in any way settles the read.
        let ticket = request.read_only.then(|| self.ledger.dispatch());
        // A cheap handle: the client is an `Arc`-backed pool.
        let client = self.client.clone();
        Ok(runtime.spawn(bounded(client, http_request, ticket)))
    }
}

/// How the live transport fails before an HTTP answer exists.
#[derive(Debug, thiserror::Error)]
pub(super) enum LiveTransportError {
    /// The request could not be built. Never carries the connection string.
    #[error("the /sql request could not be built")]
    Request,
    /// `round_trip` was called outside a Tokio runtime.
    #[error("the /sql round trip was dispatched outside a Tokio runtime")]
    NoRuntime,
    /// DNS, TLS, a refused connection.
    #[error("the /sql round trip did not complete")]
    Http(#[source] hyper_util::client::legacy::Error),
    /// The connection dropped part way through the body.
    #[error("the /sql response body did not arrive whole")]
    Body(#[source] hyper::Error),
    /// The spawned task panicked or was cancelled.
    #[error("the task carrying the /sql round trip did not finish")]
    Join(#[source] tokio::task::JoinError),
    /// No answer within [`ROUND_TRIP_TIMEOUT`].
    #[error("the /sql round trip got no answer within {after:?}")]
    Timeout {
        /// The bound that elapsed.
        after: Duration,
    },
}

impl SqlTransport for LiveTransport {
    type Error = LiveTransportError;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        // Dispatched here, at the call, not at the first poll.
        let dispatched = self.dispatch(&request);
        async move { dispatched?.await.map_err(LiveTransportError::Join)? }
    }

    fn reads_settled(&self) -> impl Future<Output = ()> {
        self.ledger.settled()
    }
}

/// One exchange, to an answer, a failure or the bound, then the read settles.
async fn bounded(
    client: HttpsClient,
    request: hyper::Request<Full<Bytes>>,
    ticket: Option<ReadTicket>,
) -> Result<HttpResponse, LiveTransportError> {
    let exchange = async {
        let response = client
            .request(request)
            .await
            .map_err(LiveTransportError::Http)?;
        let status = response.status().as_u16();
        let body = response
            .into_body()
            .collect()
            .await
            .map_err(LiveTransportError::Body)?;
        Ok(HttpResponse::new(status, body.to_bytes()))
    };
    let outcome = tokio::time::timeout(ROUND_TRIP_TIMEOUT, exchange)
        .await
        .unwrap_or(Err(LiveTransportError::Timeout {
            after: ROUND_TRIP_TIMEOUT,
        }));
    drop(ticket);
    outcome
}

/// The host out of a `postgresql://user:password@host[:port]/db?…` URL.
fn host_of(connection: &str) -> Option<String> {
    let after_scheme = connection.split_once("://")?.1;
    let authority = after_scheme.split(['/', '?']).next()?;
    let host = authority
        .rsplit_once('@')
        .map_or(authority, |(_, rest)| rest);
    let host = host.split_once(':').map_or(host, |(name, _)| name);
    (!host.is_empty()).then(|| host.to_owned())
}

/// A fresh, migrated schema's configuration, and a transport onto it.
pub(super) async fn fresh_schema() -> (NeonConfig, LiveTransport) {
    let transport = LiveTransport::new();
    SWEPT.get_or_init(|| sweep(&transport)).await;
    let config = NeonConfig::default().with_schema(mint_schema());
    execute(
        &transport,
        migration::request(migration::MIGRATION_1, &config),
        "migration 1",
    )
    .await;
    (config, transport)
}

/// An ingest store on a fresh schema of the live endpoint.
pub(super) async fn live_store() -> NeonIngestStore<LiveTransport> {
    let (config, transport) = fresh_schema().await;
    NeonIngestStore::open(NeonEventStore::new(transport, config))
        .await
        .expect("a broken test environment: a freshly migrated schema has an identity")
}

/// Sends `request` and requires a 2xx answer.
///
/// # Panics
///
/// On no answer or a non-2xx one: the arrangement, not the adapter, failed.
pub(super) async fn execute(transport: &LiveTransport, request: SqlRequest, what: &str) {
    let response = transport.round_trip(request).await.unwrap_or_else(|error| {
        panic!("a broken test environment: {what} got no answer ({error})")
    });
    assert!(
        response.is_success(),
        "a broken test environment: {what} failed (HTTP {}): {}",
        response.status,
        String::from_utf8_lossy(&response.body)
            .chars()
            .take(400)
            .collect::<String>()
    );
}

/// `hsi_<seconds>_<pid>_<n>`, unique per process and per call.
fn mint_schema() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
    format!(
        "{SCHEMA_PREFIX}_{}_{}_{ordinal}",
        now_seconds(),
        std::process::id()
    )
}

/// Seconds since the epoch; zero on a clock before it, which only makes the
/// sweep keep more.
fn now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// Drops this module's schemas older than an hour.
///
/// Housekeeping, never a failure: a sweep that cannot list or drop leaves the
/// schemas for the next run, and CI's branch per run deletes them anyway.
async fn sweep(transport: &LiveTransport) {
    let listing = SqlRequest::single(SqlStatement::new(format!(
        "SELECT nspname FROM pg_namespace WHERE nspname ~ '^{SCHEMA_PREFIX}_[0-9]+_[0-9]+_[0-9]+$'"
    )))
    .read_only();
    let Ok(response) = transport.round_trip(listing).await else {
        return;
    };
    let Ok(body) = ResponseBody::parse(&response.body) else {
        return;
    };
    let cutoff = now_seconds().saturating_sub(SWEEP_AFTER_SECONDS);
    let stale: Vec<SqlStatement> = body
        .result_sets()
        .first()
        .map(|result| result.rows.as_slice())
        .unwrap_or_default()
        .iter()
        .filter_map(|row| row.get("nspname")?.as_str())
        .filter(|name| minted_at(name).is_some_and(|seconds| seconds < cutoff))
        .map(|name| SqlStatement::new(format!("DROP SCHEMA {} CASCADE", quote_ident(name))))
        .collect();
    if stale.is_empty() {
        return;
    }
    // Housekeeping: a failed drop leaves the schemas for the next sweep, and
    // nothing a test asserts depends on it.
    if let Err(error) = transport
        .round_trip(SqlRequest::batch(stale, IsolationLevel::ReadCommitted))
        .await
    {
        eprintln!("the hsi schema sweep did not complete ({error}); the next run retries it");
    }
}

/// The seconds a schema's name says it was minted at.
fn minted_at(schema: &str) -> Option<u64> {
    schema.split('_').nth(1)?.parse().ok()
}
