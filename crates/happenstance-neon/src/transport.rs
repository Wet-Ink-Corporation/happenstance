//! The one-shot SQL-over-HTTP transport seam.
//!
//! # Why a trait and not a client
//!
//! Neon's `/sql` endpoint is reached over HTTPS. A real client on the host
//! target drags in a TLS stack, and the gate's `cargo deny` allowlist is
//! MIT / Apache-2.0 / BSD-2 / BSD-3 / ISC / Unicode-3.0 / Zlib — which the usual
//! TLS crates are not reliably inside. On `wasm32-unknown-unknown` there is no
//! TLS stack at all: the only way out of the sandbox is the host's `fetch`, via
//! `wasm-bindgen`. Those are two different clients, and neither of them is the
//! thing this crate exists to prove.
//!
//! So the transport is a trait with exactly one method, and the crate owns no
//! socket. [`NullTransport`] is the in-tree implementation, and it fails every
//! round trip.
//!
//! # What the shape of this trait is asserting
//!
//! [`SqlTransport::round_trip`] takes the whole request **by value** and returns
//! the whole response body **buffered**. There is deliberately no way to spell
//! any of the following, because Neon's endpoint cannot do them:
//!
//! * open a session and hold it — there is no connection handle to return;
//! * read part of a result and decide what to send next inside the same
//!   transaction — there is no interactive transaction;
//! * pull rows incrementally — there is no cursor, so a response is a `Bytes`
//!   that either arrived whole or did not arrive.
//!
//! The one thing the endpoint *does* offer beyond a single statement is the
//! **non-interactive** batch: an array of statements executed server-side inside
//! one `BEGIN`/`COMMIT`, at an isolation level chosen per request. That is
//! [`SqlRequest::statements`] holding more than one element. It is atomic, and it
//! is still one round trip — but the caller cannot branch on statement *n*'s
//! result before statement *n + 1* runs. Every capability limit this crate
//! records comes from that one sentence.
//!
//! # No `async fn` in the trait
//!
//! Spelled `-> impl Future<…>` rather than `async fn`, for the same reason
//! `happenstance-core` does it: `async fn` in a public trait fires rustc's
//! `async_fn_in_trait` lint, and the gate runs `-D warnings`. The desugared form
//! also makes it explicit that no `Send` bound is attached — which is the whole
//! point on `wasm32`.

use core::future::Future;

use serde::Serialize;

/// The largest response body Neon's `/sql` endpoint will return.
///
/// 64 MiB, and it is a hard ceiling rather than a default: there is no cursor to
/// fall back on when a result set exceeds it. See
/// [`NeonError::ResponseTooLarge`](crate::NeonError::ResponseTooLarge).
pub const MAX_RESPONSE_BYTES: usize = 64 * 1024 * 1024;

/// The isolation level a batched request runs at.
///
/// Travels as the `Neon-Batch-Isolation-Level` header, not in the JSON body, and
/// applies only when [`SqlRequest::statements`] holds more than one statement —
/// a single statement is its own implicit transaction and the header is ignored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum IsolationLevel {
    /// Postgres' default. A conditional insert can miss a conflict committed by
    /// a concurrent transaction after this one took its snapshot.
    #[default]
    ReadCommitted,
    /// A stable snapshot for the whole batch.
    RepeatableRead,
    /// The only level at which a read-then-write append condition is safe
    /// without an exclusion constraint, and the only one that can abort with
    /// SQLSTATE `40001`.
    Serializable,
}

impl IsolationLevel {
    /// The header value Neon expects.
    pub const fn as_header_value(self) -> &'static str {
        match self {
            Self::ReadCommitted => "ReadCommitted",
            Self::RepeatableRead => "RepeatableRead",
            Self::Serializable => "Serializable",
        }
    }
}

/// One SQL statement and its bound parameters.
///
/// Parameters are `serde_json::Value` because that is literally what goes on the
/// wire: the endpoint takes a JSON array and infers Postgres types from it.
/// A `bytea` payload therefore has to be rendered as a `\x…` hex string, which
/// roughly doubles its size against
/// [`MAX_RESPONSE_BYTES`] in both directions — a real cost, and one that a
/// binary protocol adapter does not pay.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct SqlStatement {
    /// The SQL text, with `$1`-style placeholders.
    pub query: String,
    /// The bound parameters, positionally.
    pub params: Vec<serde_json::Value>,
}

impl SqlStatement {
    /// A statement with no parameters.
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            params: Vec::new(),
        }
    }

    /// A statement with positional parameters.
    pub fn with_params(query: impl Into<String>, params: Vec<serde_json::Value>) -> Self {
        Self {
            query: query.into(),
            params,
        }
    }
}

/// Everything one round trip carries.
///
/// A request is consumed by the round trip that sends it. That is not
/// ceremony — it is the closest the type system gets to saying "there is no
/// session, so this request cannot be *re-sent into* anything".
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SqlRequest {
    /// One statement, or several to be run server-side in one implicit
    /// transaction. Never zero.
    pub statements: Vec<SqlStatement>,
    /// The isolation level for the batch form.
    pub isolation: IsolationLevel,
    /// Whether to ask the endpoint for a read-only transaction.
    pub read_only: bool,
}

/// The JSON body shape for the multi-statement form.
#[derive(Debug, Serialize)]
struct BatchBody<'a> {
    queries: &'a [SqlStatement],
}

impl SqlRequest {
    /// A single-statement request at the default isolation level.
    pub fn single(statement: SqlStatement) -> Self {
        Self {
            statements: vec![statement],
            isolation: IsolationLevel::ReadCommitted,
            read_only: false,
        }
    }

    /// A multi-statement request, run server-side as one non-interactive
    /// transaction.
    pub fn batch(statements: Vec<SqlStatement>, isolation: IsolationLevel) -> Self {
        Self {
            statements,
            isolation,
            read_only: false,
        }
    }

    /// Marks the request read-only.
    #[must_use]
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// The JSON body to POST.
    ///
    /// The endpoint takes two shapes and distinguishes them structurally: a bare
    /// `{"query": …, "params": […]}` for one statement, and
    /// `{"queries": [ … ]}` for the batch. An empty `statements` serialises as
    /// the batch form with an empty array, which the endpoint rejects; the
    /// adapter never builds one.
    ///
    /// # Errors
    ///
    /// Returns the `serde_json` error if a parameter cannot be serialised.
    pub fn body(&self) -> Result<Vec<u8>, serde_json::Error> {
        match self.statements.as_slice() {
            [only] => serde_json::to_vec(only),
            many => serde_json::to_vec(&BatchBody { queries: many }),
        }
    }

    /// The Neon-specific headers this request needs, beyond `Content-Type` and
    /// the connection string.
    ///
    /// Empty for the single-statement form: the endpoint ignores both headers
    /// there, and sending them would suggest they had an effect.
    pub fn headers(&self) -> Vec<(&'static str, &'static str)> {
        if self.statements.len() < 2 {
            return Vec::new();
        }
        vec![
            (
                "Neon-Batch-Isolation-Level",
                self.isolation.as_header_value(),
            ),
            (
                "Neon-Batch-Read-Only",
                if self.read_only { "true" } else { "false" },
            ),
        ]
    }
}

/// A complete HTTP response, already buffered.
///
/// `Bytes` rather than a reader because there is nothing to read *from* — the
/// endpoint answers once. A body over [`MAX_RESPONSE_BYTES`] never becomes one
/// of these.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status: u16,
    /// The whole body.
    pub body: happenstance_core::bytes::Bytes,
}

impl HttpResponse {
    /// Builds a response.
    pub fn new(status: u16, body: impl Into<happenstance_core::bytes::Bytes>) -> Self {
        Self {
            status,
            body: body.into(),
        }
    }

    /// Whether the status is in the 2xx range.
    pub const fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }
}

/// One round trip to a SQL-over-HTTP endpoint.
///
/// Implementors own the TLS stack, the credentials and the retry policy. This
/// crate owns the SQL and the decoding, and nothing else.
pub trait SqlTransport {
    /// How the transport fails before an HTTP answer exists: DNS, TLS, a
    /// rejected `fetch`, a timeout.
    ///
    /// A non-2xx status is **not** one of these — it is an answer, and it is
    /// where Neon puts SQL errors, so it must reach the decoder intact.
    type Error: core::error::Error + 'static;

    /// Sends `request` and returns the whole answer.
    ///
    /// Written `-> impl Future` rather than `async fn` so no `Send` bound is
    /// implied; a `wasm32` implementation over `fetch` cannot supply one.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] only when no HTTP response was obtained.
    /// # Cancellation, and what this adapter does NOT assume
    ///
    /// This adapter makes **no assumption that dropping the returned future
    /// cancels the request**. `NeonEventStore::append` awaits this method, so
    /// ES-23's answer for the store is inherited from whatever an implementor
    /// does here.
    ///
    /// An implementor **MUST NOT** present its transport as cancellation-safe
    /// unless it can guarantee the endpoint never observes an abandoned
    /// request. That is a guarantee about a remote system rather than about a
    /// `Future`, and dropping a client future is not it: a request already
    /// written to a socket has been sent.
    ///
    /// Stated as this adapter's expectation of a transport rather than as a
    /// measured fact — nothing here has measured what any particular endpoint
    /// does with a request whose response nobody reads.
    fn round_trip(
        &self,
        request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>>;
}

/// The transport that owns no I/O.
///
/// Every round trip fails. It exists so the adapter's types can be instantiated,
/// its trait impls checked at a concrete type, and its doctests compiled,
/// without this crate depending on an HTTP client or a TLS stack.
#[derive(Debug, Clone, Copy, Default)]
#[non_exhaustive]
pub struct NullTransport;

impl NullTransport {
    /// Creates the null transport.
    pub const fn new() -> Self {
        Self
    }
}

/// The only way [`NullTransport`] fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("no SQL-over-HTTP transport is configured; happenstance-neon owns no HTTP client")]
#[non_exhaustive]
pub struct NullTransportError;

impl SqlTransport for NullTransport {
    type Error = NullTransportError;

    fn round_trip(
        &self,
        _request: SqlRequest,
    ) -> impl Future<Output = Result<HttpResponse, Self::Error>> {
        core::future::ready(Err(NullTransportError))
    }
}
