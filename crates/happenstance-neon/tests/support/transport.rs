//! The HTTPS transport the conformance suite reaches Neon through.
//!
//! # Why this is in `tests/` and not behind a feature
//!
//! `cargo xtask ci` runs
//!
//! ```console
//! cargo hack check -p happenstance-neon --target wasm32-unknown-unknown \
//!     --feature-powerset --no-dev-deps
//! ```
//!
//! and a Cargo feature is **not target-scoped**. A `live-transport` feature would
//! therefore be switched on somewhere in that powerset, on `wasm32`, where
//! `hyper` and `ring` do not build — so the feature would fail the very step this
//! crate exists to keep passing. `--no-dev-deps` is what makes a dev-dependency
//! invisible to both wasm32 powersets and to the msrv job, and it is
//! compiler-checked rather than argued. The manifest carries the licence verdict
//! and the measurement that every node here was already in `Cargo.lock`.
//!
//! # Eager dispatch, and what it does and does not buy
//!
//! [`SqlTransport::round_trip`] is **not** an `async fn` here. It builds the
//! request and hands it to [`tokio::runtime::Handle::spawn`] *at call time*, then
//! returns a future that awaits the `JoinHandle`. Two things follow.
//!
//! The first is that the concurrency family works at all. It runs each of
//! `CONTENDERS` contenders on a raw OS thread driving `block_on`, deliberately —
//! the testkit has no runtime dependency and must not acquire one — so there is
//! no reactor on those threads and a `hyper` future polled inline there never
//! completes. Spawning moves the work onto the runtime that owns the reactor and
//! leaves the contender waiting on a `JoinHandle`, which is a plain future and
//! needs nothing. `happenstance-postgres` carries the same shape one layer down,
//! and records that `Handle::enter` is the smaller move that does not work: its
//! `EnterGuard` is `!Send`.
//!
//! The second is ES-11, and this is the honest part of the file.
//!
//! # ES-11 is not bought here, and the numbers say by how much
//!
//! `read_result_is_stable_under_concurrent_append` polls the read's stream once
//! and then appends. Spawning at the first poll puts the `SELECT` on the wire
//! before `append` is even called, which is the earliest an adapter is permitted
//! to fix its state, and the specification's conformance path for an
//! asynchronous driver states its own sufficiency condition one sentence later:
//! *a read spawned at its first poll and an append spawned afterwards land in the
//! same queue in that order, so the snapshot precedes the append*.
//!
//! **That condition is false here by construction.** `happenstance-postgres`
//! satisfies it because both operations enter one `PgPool`. A read and an append
//! from this adapter are two independent requests to a proxy that hands each one
//! to whichever backend it likes, and nothing in the protocol orders the
//! snapshot one takes against the commit the other makes.
//!
//! It is not a theoretical gap. Measured, twenty runs of the two ES-11 rules per
//! configuration, `--test-threads=1`, against the live endpoint:
//!
//! | transport | red runs |
//! |---|---|
//! | HTTP/1.1, default pool | **3 of 20** |
//! | HTTP/2, one connection | **1 of 40** |
//!
//! The failure is always the same and always in the direction that matters: the
//! read's snapshot is taken *after* the append commits, so the drained stream
//! carries the `Later` event the rule requires it not to.
//!
//! HTTP/2 is therefore not a performance choice — it is the only ordering
//! primitive available. Two nearly simultaneous requests become two streams
//! written in order down **one** TCP connection, so the proxy at least *receives*
//! them in the order they were issued; over HTTP/1.1 they take two connections
//! and even that is a coin toss. `pool_max_idle_per_host(1)` is half of it: with
//! several warm connections the pool picks one per request and the ordering h2
//! buys is given straight back.
//!
//! What remains is a **capability limit of this adapter**, not a flake to retry
//! away. Receiving two requests in order does not make one backend's snapshot
//! precede another backend's commit, and no configuration of this client can
//! make it. **Nothing here suppresses it** — the rule is mounted, it is not gated
//! by a capability, and a run that loses the race is red.
//!
//! The residual belonged to the clause's owner rather than to this file, and
//! **ADR-0061 settled it**: the sufficiency sentence quoted above is corrected to
//! say *spawned at the first poll, **and** ordered against a later append by
//! something the store itself honours* — a narrowing, since it removes spawn order
//! alone as a route to a conformance claim — and `happenstance-neon` does not
//! satisfy ES-11. The MUST did not move and no capability was minted, which is why
//! this file still mounts the rule and still goes red when it loses.
//!
//! # It owns a runtime, and the shape that looks right does not work
//!
//! The obvious design — and the one this file shipped first — is to capture
//! `Handle::try_current()` at the first `connect()` and spawn onto that, exactly
//! as `happenstance-postgres` and `happenstance-sqlite` do. It is wrong here, and
//! the failure is worth writing down because nothing about it is visible until
//! the second test runs.
//!
//! `#[tokio::test]` builds a **runtime per test** and drops it when that test
//! returns. A handle captured inside the first test therefore names a runtime
//! that is already gone by the second, and every spawn onto it comes back as
//! `JoinError::Cancelled`. Measured, not reasoned about: eight of nineteen
//! projection rules failed on the first live run with
//! `Store(Transport(Join(JoinError::Cancelled(…))))`, and the eight were the ones
//! that ran after whichever test happened to build the shared client.
//!
//! The two adapters that capture a handle do not hit this because the store is
//! constructed *inside* the runtime that will use it, per test. This transport is
//! deliberately process-wide — one client, one connection pool, ninety fixture
//! instances — so it cannot be.
//!
//! So it owns a dedicated multi-thread runtime, held in the same `OnceLock` as
//! the client and never dropped, because a `static` is not. That is **not** the
//! silent inline fallback ADR-0022 §9 forbids: there is no arm here that runs the
//! future on the caller's thread and hopes, and no configuration in which a round
//! trip is dispatched anywhere other than the runtime named here. The failure
//! mode that variant existed to keep nameable — "the future advanced only while
//! someone remembered to poll it" — is structurally unreachable rather than
//! merely unlikely.
//!
//! # The connection string
//!
//! Read from `NEON_CONNECTION` at run time and held in an `Arc<str>`. It is never
//! logged, never put in an error message, never written to a file and never
//! serialised: [`HyperTransport`]'s `Debug` prints the host and the word
//! `<redacted>`, which is what stops a `dbg!` in a failing rule from putting a
//! password in a CI log.

#![allow(dead_code)]

use std::sync::{Arc, OnceLock};

use happenstance_core::bytes::Bytes;
use happenstance_neon::{HttpResponse, SqlRequest, SqlTransport};
use http_body_util::{BodyExt, Full};
use hyper::Uri;
use hyper::header::{CONTENT_TYPE, HeaderName, HeaderValue};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use tokio::runtime::Runtime;

/// The environment variable holding the pooler URL, with its credentials.
pub(crate) const CONNECTION_ENV: &str = "NEON_CONNECTION";

/// The header the endpoint authenticates on.
const CONNECTION_HEADER: &str = "Neon-Connection-String";

/// The one client every fixture in this process shares.
///
/// A client rather than a connection: `hyper-util`'s legacy client pools TCP
/// connections, and the pool is what keeps a warm socket available for the
/// second of two nearly simultaneous requests. Sharing it is also arithmetic —
/// the event-store family builds around ninety fixture instances over one run,
/// and a client per instance would mean ninety pools and ninety TLS handshakes
/// for work that reuses one host.
static SHARED: OnceLock<Shared> = OnceLock::new();

/// What every transport in this process borrows.
struct Shared {
    client: Client<hyper_rustls::HttpsConnector<HttpConnector>, Full<Bytes>>,
    uri: Uri,
    connection: Arc<str>,
    /// The runtime every round trip is dispatched onto.
    ///
    /// Owned rather than borrowed from the caller, for the reason the module docs
    /// give in full: `#[tokio::test]` drops its runtime when its test returns, so
    /// a captured `Handle` is dead by the second test and every spawn onto it
    /// answers `JoinError::Cancelled`. Never dropped, because a `static` is not —
    /// which is exactly the lifetime this needs.
    runtime: Runtime,
}

/// A one-shot HTTPS transport onto Neon's `/sql` endpoint.
#[derive(Clone)]
pub(crate) struct HyperTransport {
    shared: &'static Shared,
}

impl core::fmt::Debug for HyperTransport {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // The connection string is a credential. It is not printed here, and it
        // is not printed anywhere else either — this impl exists so that a `dbg!`
        // in a failing rule cannot be the thing that puts it in a CI log.
        f.debug_struct("HyperTransport")
            .field("uri", &self.shared.uri)
            .field("connection", &"<redacted>")
            .finish()
    }
}

impl HyperTransport {
    /// The shared transport, building it on first use.
    ///
    /// # Panics
    ///
    /// When `NEON_CONNECTION` is absent or unusable, when no platform CA store
    /// can be read, or when the dedicated runtime cannot be built. All three are
    /// broken **test environments** rather than non-conformant adapters, and
    /// [`Fixture::connect`] has no `Result` for exactly that reason — a `Result`
    /// here would put "the secret is not set" into the same channel as "the
    /// adapter is wrong".
    ///
    /// Note what is *not* here: an early return that reports success. A run of
    /// `-- --ignored` with no endpoint must fail loudly, because "no server, pass
    /// quietly" is indistinguishable in CI output from a rule that passed, which
    /// is CF-18's argument one layer out. The green-gate-without-the-secret
    /// property is bought by `#[ignore]` on every gated test, and by nothing else.
    ///
    /// [`Fixture::connect`]: happenstance_testkit::Fixture::connect
    pub(crate) fn shared() -> Self {
        Self {
            shared: SHARED.get_or_init(build_shared),
        }
    }

    /// The host the endpoint answers on, for a message that must not name the
    /// credential.
    pub(crate) fn host(&self) -> &str {
        self.shared.uri.host().unwrap_or("<unknown>")
    }
}

/// Whether the environment carries a usable connection string.
///
/// Read once per call rather than cached: a fixture asks before it panics, and a
/// stale cache would make the message wrong for anyone setting the variable
/// between runs of the same binary.
pub(crate) fn connection_is_configured() -> bool {
    std::env::var(CONNECTION_ENV).is_ok_and(|value| !value.trim().is_empty())
}

/// Builds the process-wide client, URI and credential.
fn build_shared() -> Shared {
    let connection = std::env::var(CONNECTION_ENV).unwrap_or_else(|_| {
        panic!(
            "a broken test environment, not a non-conformant adapter: `{CONNECTION_ENV}` is \
             not set, so there is no Neon endpoint to run the suite against. These tests are \
             `#[ignore]`d precisely so that a machine without the secret still has a green \
             `cargo xtask ci`; they are reached with `-- --ignored`."
        )
    });
    let connection = connection.trim().to_owned();

    let host = host_of(&connection).unwrap_or_else(|| {
        panic!(
            "a broken test environment: `{CONNECTION_ENV}` does not look like a \
             `postgresql://user:password@host/db` URL. Its value is deliberately not \
             printed here."
        )
    });
    let uri: Uri = format!("https://{host}/sql")
        .parse()
        .unwrap_or_else(|error| {
            panic!("a broken test environment: {host} is not a host ({error})")
        });

    // One provider, installed once. `rustls` refuses to guess when more than one
    // could be linked, and `ring` is the one this workspace's licence allowlist
    // already admits — `Apache-2.0 AND ISC`, recorded in the workspace manifest.
    // `aws-lc-rs` is not in this graph and must not be brought into it.
    let _ = rustls::crypto::ring::default_provider().install_default();

    let connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_native_roots()
        .unwrap_or_else(|error| {
            panic!(
                "a broken test environment: no platform CA store could be read ({error}). \
                 The Mozilla root bundle is deliberately not vendored here: `webpki-roots` \
                 is CDLA-Permissive-2.0 and fails this workspace's `cargo deny` allowlist."
            )
        })
        .https_only()
        // HTTP/2, and it is not a performance choice. See the module docs: it is
        // the only thing available that puts two nearly simultaneous requests
        // into a single ordered byte stream, which is what ES-11's read-stability
        // rule needs and what two independent HTTP/1.1 connections do not give.
        // `enable_http1` stays as the ALPN fallback so a proxy that refuses h2
        // still works, degraded.
        .enable_http1()
        .enable_http2()
        .build();

    let client = Client::builder(TokioExecutor::new())
        // One connection, many streams. Without this the pool keeps several warm
        // HTTP/2 connections and the ordering h2 buys is lost again at the point
        // the pool picks which one to use.
        .pool_max_idle_per_host(1)
        .build(connector);

    // Multi-thread, and it is arithmetic rather than taste: the concurrency
    // family puts `CONTENDERS` round trips in flight at once, and a
    // current-thread runtime would turn that race into a queue -- which passes
    // every rule in the family for the wrong reason.
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("neon-conformance")
        .build()
        .unwrap_or_else(|error| {
            panic!("a broken test environment: no runtime could be built ({error})")
        });

    Shared {
        client,
        uri,
        connection: connection.into(),
        runtime,
    }
}

/// The host out of a `postgresql://user:password@host[:port]/db?…` URL.
///
/// Hand-parsed rather than through `url`, which is not in this crate's graph and
/// would be a dependency taken for six lines. The port is dropped: the `/sql`
/// endpoint is HTTPS on 443 whatever port the SQL protocol uses.
fn host_of(connection: &str) -> Option<String> {
    let after_scheme = connection.split_once("://")?.1;
    let authority = after_scheme.split(['/', '?']).next()?;
    let host = authority
        .rsplit_once('@')
        .map_or(authority, |(_, rest)| rest);
    let host = host.split_once(':').map_or(host, |(name, _)| name);
    (!host.is_empty()).then(|| host.to_owned())
}

/// How the live transport fails before an HTTP answer exists.
#[derive(Debug, thiserror::Error)]
pub(crate) enum HyperTransportError {
    /// The request could not be built. Never carries the connection string.
    #[error("the /sql request could not be built")]
    Request,

    /// DNS, TLS, a refused connection, a timeout.
    #[error("the /sql round trip did not complete")]
    Http(#[source] hyper_util::client::legacy::Error),

    /// The connection dropped part way through the body.
    #[error("the /sql response body did not arrive whole")]
    Body(#[source] hyper::Error),

    /// The spawned task panicked or was cancelled.
    #[error("the task carrying the /sql round trip did not finish")]
    Join(#[source] tokio::task::JoinError),
}

impl SqlTransport for HyperTransport {
    type Error = HyperTransportError;

    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        // Everything up to and including `spawn` happens NOW, synchronously, at
        // the call site — not on the first poll of the returned future. That is
        // the whole of the eager dispatch the module docs argue for, and writing
        // this as an `async fn` would undo it in one keystroke with nothing to
        // catch it.
        let dispatched = self.dispatch(&request);
        async move {
            match dispatched {
                Ok(handle) => handle.await.map_err(HyperTransportError::Join)?,
                Err(error) => Err(error),
            }
        }
    }
}

impl HyperTransport {
    /// Builds and spawns one round trip, returning the task to await.
    fn dispatch(
        &self,
        request: &SqlRequest,
    ) -> Result<
        tokio::task::JoinHandle<Result<HttpResponse, HyperTransportError>>,
        HyperTransportError,
    > {
        let body = request.body().map_err(|_| HyperTransportError::Request)?;

        let mut builder = hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(self.shared.uri.clone())
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .header(
                HeaderName::from_static("neon-connection-string"),
                HeaderValue::from_str(&self.shared.connection)
                    .map_err(|_| HyperTransportError::Request)?,
            );
        debug_assert_eq!(
            CONNECTION_HEADER.to_ascii_lowercase(),
            "neon-connection-string"
        );

        // Empty below two statements, because the endpoint ignores both headers
        // on the single form and sending them would suggest they had an effect.
        // The adapter decides that, not this file.
        for (name, value) in request.headers() {
            builder = builder.header(name, value);
        }

        let http_request = builder
            .body(Full::new(Bytes::from(body)))
            .map_err(|_| HyperTransportError::Request)?;

        let client = self.shared.client.clone();
        Ok(self.shared.runtime.spawn(async move {
            let response = client
                .request(http_request)
                .await
                .map_err(HyperTransportError::Http)?;
            let status = response.status().as_u16();
            let collected = response
                .into_body()
                .collect()
                .await
                .map_err(HyperTransportError::Body)?;
            Ok(HttpResponse::new(status, collected.to_bytes()))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::host_of;

    /// The pooler URL's shape, without any real one.
    #[test]
    fn the_host_survives_credentials_a_port_and_a_query() {
        assert_eq!(
            host_of("postgresql://u:p@ep-x-pooler.eu-west-2.aws.neon.tech/db?sslmode=require")
                .as_deref(),
            Some("ep-x-pooler.eu-west-2.aws.neon.tech")
        );
        assert_eq!(
            host_of("postgres://u:p@host:5432/db").as_deref(),
            Some("host")
        );
        assert_eq!(host_of("host/db"), None, "no scheme is not a URL");
    }
}
