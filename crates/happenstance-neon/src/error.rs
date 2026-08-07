//! How a Neon-backed store fails.
//!
//! Five of the six variants correspond to a distinct thing that can go wrong on
//! a single HTTP round trip, and they are distinct because a caller acts
//! differently on each. The sixth,
//! [`InvalidPosition`](NeonError::InvalidPosition), is the impedance mismatch
//! between Postgres' signed `bigint` and
//! [`SequencePosition`](happenstance_core::SequencePosition)'s `NonZeroU64`.
//!
//! [`AppendError::ConditionViolated`](happenstance_core::AppendError::ConditionViolated)
//! is deliberately absent: the DCB concurrency signal is not an adapter failure
//! and lives in the contract crate's enum, not this one.

use serde::Deserialize;

/// The error body Neon's `/sql` endpoint returns when Postgres rejects a
/// statement.
///
/// The endpoint answers with a 4xx status and this JSON, so a SQL error is an
/// *answer*, not a transport failure. Preserving it structurally — rather than
/// stringifying it — is what lets a caller act on SQLSTATE `40001` (serialisation
/// failure, retry) differently from `23505` (unique violation, do not).
#[derive(Debug, Clone, Deserialize, thiserror::Error)]
#[error("{message}")]
#[non_exhaustive]
pub struct NeonSqlError {
    /// The primary human-readable message.
    pub message: String,
    /// The five-character SQLSTATE, when the endpoint supplied one.
    pub code: Option<String>,
    /// Postgres' `DETAIL`.
    pub detail: Option<String>,
    /// Postgres' `HINT`.
    pub hint: Option<String>,
    /// `ERROR`, `FATAL` or `PANIC`.
    pub severity: Option<String>,
}

impl NeonSqlError {
    /// SQLSTATE `40001`: the transaction was aborted to preserve
    /// serialisability.
    ///
    /// Only reachable at [`IsolationLevel::Serializable`](crate::IsolationLevel),
    /// and the reason that level is worth its cost here: it is the one
    /// mechanism that makes a single-statement conditional insert safe without
    /// a schema-level exclusion constraint.
    pub fn is_serialization_failure(&self) -> bool {
        self.code.as_deref() == Some("40001")
    }

    /// SQLSTATE `23505`: a unique constraint was violated.
    pub fn is_unique_violation(&self) -> bool {
        self.code.as_deref() == Some("23505")
    }
}

/// How a Neon-backed store fails for its own reasons.
///
/// Generic over the transport's error so that the transport's own failure mode
/// survives intact rather than being flattened into a string — the crate does
/// not know whether it is wrapping a `reqwest::Error`, a `JsValue` stringified
/// by `wasm-bindgen`, or a test double.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum NeonError<E> {
    /// No HTTP answer was obtained: DNS, TLS, a rejected `fetch`, a timeout.
    ///
    /// The one variant on which a retry is *unsafe* without an idempotency key:
    /// a request that timed out may still have committed, and with no session
    /// there is nothing to ask.
    #[error("the SQL-over-HTTP round trip did not complete")]
    Transport(#[source] E),

    /// The endpoint answered with a non-2xx status and no parsable Neon error
    /// body — a gateway, an auth failure, a rate limit.
    #[error("the /sql endpoint answered with HTTP {status}")]
    Http {
        /// The status code.
        status: u16,
        /// As much of the body as was kept, truncated for the message.
        body: Box<str>,
    },

    /// Postgres rejected the statement, and said why.
    #[error(transparent)]
    Sql(#[from] NeonSqlError),

    /// The response body exceeded the endpoint's hard cap.
    ///
    /// There is no cursor and no continuation token, so this is terminal for the
    /// read that produced it: the only recovery is a narrower query or a smaller
    /// [`ReadOptions::limit`](happenstance_core::ReadOptions::limit), both of
    /// which are new round trips against a new snapshot.
    #[error("the /sql response was {bytes} bytes, over the endpoint's {limit} byte cap")]
    ResponseTooLarge {
        /// The size that was refused.
        bytes: usize,
        /// [`MAX_RESPONSE_BYTES`](crate::MAX_RESPONSE_BYTES).
        limit: usize,
    },

    /// The body was 2xx JSON but not the shape this adapter asked for.
    #[error("the /sql response did not have the shape this adapter asked for")]
    MalformedResponse(#[source] serde_json::Error),

    /// A `position` column held a value `SequencePosition` cannot represent.
    ///
    /// Postgres `bigserial` is signed and starts at 1, so this is only reachable
    /// through a hand-seeded sequence — but the column is `bigint` on the wire
    /// and the conversion is fallible, so the failure has to be nameable.
    #[error("the store returned position {value}, which is not a valid SequencePosition")]
    InvalidPosition {
        /// The rejected value.
        value: i64,
    },
}
