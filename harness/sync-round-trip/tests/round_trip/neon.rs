//! SQLite through `NeonSyncPeer`, against the live endpoint `NEON_CONNECTION`
//! names.
//!
//! The client is `happenstance-neon`'s own live transport restated, small,
//! because that one lives in its test code, which no other crate can reach.
//! What it keeps is what makes it safe to point at a credential and at the
//! ES-11 fence: the connection string is never printed, every read-only
//! request is registered in a [`ReadLedger`] before `round_trip` returns and
//! settled by the task that carries it, and no exchange outlives
//! [`ROUND_TRIP_TIMEOUT`] (H-13). Each test mints its own schema, `hsr_…`,
//! and drops it once it has passed; CI's branch per run deletes any a failed
//! run leaves.
//!
//! # Why the functions here panic
//!
//! An endpoint that cannot be reached is a broken test environment, not a
//! peer defect, and the message says so.

use core::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use happenstance_core::bytes::Bytes;
use happenstance_neon::config::quote_ident;
use happenstance_neon::{
    HttpResponse, NeonConfig, NeonEventStore, NeonSyncPeer, ReadLedger, ReadTicket, SqlRequest,
    SqlStatement, SqlTransport, migration,
};
use http_body_util::{BodyExt, Full};
use hyper::Uri;
use hyper::header::{CONTENT_TYPE, HeaderName, HeaderValue};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;

/// `happenstance-neon`'s `tests/support/transport.rs`'s, which this must match.
const CONNECTION_ENV: &str = "NEON_CONNECTION";

/// That file's header, lowercased as `hyper` wants it.
const CONNECTION_HEADER: &str = "neon-connection-string";

/// That file's bound: a read that never answers would hold every later write
/// on its transport, behind the fence.
const ROUND_TRIP_TIMEOUT: Duration = Duration::from_secs(30);

/// The client this module sends through.
type HttpsClient = Client<hyper_rustls::HttpsConnector<HttpConnector>, Full<Bytes>>;

/// A one-shot HTTPS transport onto Neon's `/sql` endpoint. Clones share one
/// client and one [`ReadLedger`], so they are one ES-11 ordering domain.
#[derive(Clone)]
struct LiveTransport {
    client: HttpsClient,
    uri: Uri,
    /// A credential. Never printed, never put in an error.
    connection: Arc<str>,
    ledger: ReadLedger,
}

// By hand, so the credential is never printed.
impl core::fmt::Debug for LiveTransport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LiveTransport")
            .field("uri", &self.uri)
            .field("connection", &"<redacted>")
            .finish_non_exhaustive()
    }
}

impl LiveTransport {
    /// A transport onto the endpoint `NEON_CONNECTION` names.
    ///
    /// # Panics
    ///
    /// When the variable is absent or is not a URL, or no platform CA store
    /// can be read.
    fn new() -> Self {
        let connection = std::env::var(CONNECTION_ENV)
            .ok()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or_else(|| {
                panic!(
                    "a broken test environment, not a failing peer: `{CONNECTION_ENV}` is not \
                     set. These tests are #[ignore]d so a machine without the secret stays \
                     green; they are reached with -- --ignored."
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
enum LiveTransportError {
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

/// Sends `request` and requires a 2xx answer.
///
/// # Panics
///
/// On no answer or a non-2xx one: the arrangement, not the peer, failed.
async fn execute(transport: &LiveTransport, request: SqlRequest, what: &str) {
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

/// A Neon peer on a fresh, migrated schema `hsr_<seconds>_<pid>_<n>`.
struct Schema {
    peer: NeonSyncPeer<LiveTransport>,
}

impl Schema {
    async fn fresh() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(1);
        // Relaxed: the counter publishes no other memory; all it must give is
        // a distinct value per call.
        let ordinal = NEXT.fetch_add(1, Ordering::Relaxed);
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs());
        let name = format!("hsr_{seconds}_{}_{ordinal}", std::process::id());
        let transport = LiveTransport::new();
        let config = NeonConfig::default().with_schema(name);
        let both = format!("{}\n{}", migration::MIGRATION_1, migration::MIGRATION_3);
        execute(
            &transport,
            migration::request(&both, &config),
            "migrations 1 and 3",
        )
        .await;
        Self {
            peer: NeonSyncPeer::new(NeonEventStore::new(transport, config)),
        }
    }

    fn store(&self) -> &NeonEventStore<LiveTransport> {
        self.peer.store()
    }

    /// Drops the schema: reached only by a test that passed.
    async fn drop_schema(self) {
        let store = self.store();
        let statement = SqlStatement::new(format!(
            "DROP SCHEMA {} CASCADE",
            quote_ident(&store.config().schema)
        ));
        execute(
            store.transport(),
            SqlRequest::single(statement),
            "dropping the schema",
        )
        .await;
    }
}

/// SQLite to Neon and back, and Neon's own append to SQLite, byte for byte.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn sqlite_through_the_neon_peer_is_byte_identical_both_ways() {
    let schema = Schema::fresh().await;

    super::sqlite_round_trip(&schema.peer, schema.store()).await;

    schema.drop_schema().await;
}

/// A SQLite batch pushed to Neon twice changes nothing.
#[tokio::test]
#[ignore = "needs a live Neon endpoint; set NEON_CONNECTION and run with `-- --ignored`"]
async fn a_sqlite_batch_replayed_to_the_neon_peer_changes_nothing() {
    let schema = Schema::fresh().await;

    super::replay_changes_nothing(&schema.peer, schema.store()).await;

    schema.drop_schema().await;
}
