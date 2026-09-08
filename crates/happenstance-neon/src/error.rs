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
    /// The column is `bigint`, which is signed and admits zero, against a
    /// `NonZeroU64` — so the conversion is fallible and the failure has to be
    /// nameable. The rejected alternative is `unsigned_abs()`, which reads like a
    /// guard and turns a stored `-3` into position 3.
    #[error("the store returned position {value}, which is not a valid SequencePosition")]
    InvalidPosition {
        /// The rejected value.
        value: i64,
    },

    /// The response was well-formed JSON of the right *shape* and did not carry
    /// a column the decoder needs.
    ///
    /// Distinct from [`MalformedResponse`](Self::MalformedResponse), which is a
    /// `serde_json` failure against the wire types. This one is the adapter's own
    /// `SELECT` and its own decoder disagreeing, which is a bug in this crate
    /// rather than a drift at the endpoint — and the two send a reader to very
    /// different places.
    #[error("the /sql response carried no `{column}` column where this adapter expected one")]
    MissingColumn {
        /// The column the decoder asked for.
        column: &'static str,
    },

    /// A stored `event_type` no longer satisfies the contract's validation.
    ///
    /// Reachable only from a row this adapter did not write, or from validation
    /// tightening under a store that already holds data. A named variant rather
    /// than a panic, because the alternative to a variant is a panic in a
    /// library.
    #[error("a stored event type is not a valid EventType")]
    StoredEventType(#[source] happenstance_core::InvalidEventType),

    /// A stored tag no longer satisfies the contract's validation.
    #[error("a stored tag is not a valid Tag")]
    StoredTag(#[source] happenstance_core::InvalidTag),

    /// A row carries no `EventId`, so the store cannot say what it is.
    ///
    /// `origin_store` and `origin_position` are nullable because a replication
    /// ingest may hold rows minted elsewhere before this store stamps its own.
    /// A row *this* store wrote always has both — unless migration 1's
    /// `store_meta` row is absent, which is the reachable cause and is why this
    /// is not spelled as a decode failure.
    #[error("the event at position {position} carries no EventId; is the store_meta row present?")]
    UnstampedEvent {
        /// Where the unstamped row sits.
        position: u64,
    },

    /// An `origin_store` column held something other than sixteen bytes.
    #[error("a stored StoreId was {len} bytes, not 16")]
    MalformedIdentity {
        /// The length that was refused.
        len: usize,
    },

    /// A `bytea` column did not arrive as the base64 this adapter asked for.
    ///
    /// Every `SELECT` here spells `encode(col, 'base64')` explicitly, so this is
    /// the endpoint and this crate disagreeing rather than a caller's data being
    /// wrong.
    #[error("a bytea column did not decode as base64")]
    MalformedPayload,

    /// A projection batch was begun on a different store instance.
    ///
    /// `rollback` is the one method on the projection port whose error type is
    /// the *adapter's* rather than a dedicated enum, so the refusal
    /// [`CommitError::ForeignBatch`](happenstance_core::CommitError::ForeignBatch)
    /// and [`ResetError::ForeignBatch`](happenstance_core::ResetError::ForeignBatch)
    /// spell has to be nameable here too. The batch is consumed either way; the
    /// refusal is how a caller learns it was holding the wrong one, on the call
    /// that was meant to be the cleanup.
    #[error("the batch was begun on a different store instance")]
    ForeignBatch,
}
