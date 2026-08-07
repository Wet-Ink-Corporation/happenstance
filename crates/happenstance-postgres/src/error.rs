//! The failure modes a Postgres adapter actually has.
//!
//! Two enums rather than one, because the two ports have independent `Error`
//! associated types and merging them would force every event-store caller to
//! match on projection-store variants it can never see.
//!
//! Neither enum carries a `ConditionViolated` variant. An append-condition
//! violation is not an adapter failure — the contract reports it through
//! [`AppendError::ConditionViolated`](happenstance_core::AppendError), so that a
//! caller can tell "rebuild the decision model and retry" from "something broke"
//! without knowing which adapter it holds.

use happenstance_core::{InvalidEventType, InvalidTag};

/// How [`PostgresEventStore`](crate::event_store::PostgresEventStore) fails for
/// its own reasons.
///
/// The decoding variants are not defensive padding. A row is written by one
/// version of this adapter and read back by another, and the contract types
/// enforce invariants that the Postgres column types do not: `position` is
/// `bigint`, which is signed and admits `0` and negatives, while
/// [`SequencePosition`](happenstance_core::SequencePosition) is a `NonZeroU64`;
/// `event_type` and `tag` are `text`, which admits control characters and
/// over-long values that [`EventType`](happenstance_core::EventType) and
/// [`Tag`](happenstance_core::Tag) reject. Every one of those is a real row a
/// real database can hand back, and the alternative to a variant is a panic in a
/// library.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PostgresEventStoreError {
    /// The driver, the pool or the server rejected the work.
    ///
    /// This is the common case and it is deliberately not flattened into
    /// per-`SQLSTATE` variants: `sqlx::Error::Database` already exposes the code
    /// through
    /// [`DatabaseError::code`](sqlx::error::DatabaseError::code), and a variant
    /// per code would be a second, worse copy of that.
    #[error("postgres rejected the work")]
    Driver(#[from] sqlx::Error),

    /// A row carried a `position` outside the contract's domain.
    ///
    /// `bigint` is signed and admits zero; `SequencePosition` is a `NonZeroU64`.
    /// The gap between the two is exactly this variant.
    #[error("stored position {value} is not a valid sequence position")]
    PositionOutOfRange {
        /// The value as Postgres returned it.
        value: i64,
    },

    /// A row carried an `event_type` that no longer passes validation.
    #[error("stored event type is not valid: {0}")]
    EventType(#[source] InvalidEventType),

    /// A row carried a tag that no longer passes validation.
    #[error("stored tag is not valid: {0}")]
    Tag(#[source] InvalidTag),
}

/// How [`PostgresProjectionStore`](crate::projection_store::PostgresProjectionStore)
/// fails.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum PostgresProjectionStoreError {
    /// The driver, the pool or the server rejected the work.
    #[error("postgres rejected the work")]
    Driver(#[from] sqlx::Error),

    /// A stored checkpoint was outside the contract's domain.
    ///
    /// Same gap as
    /// [`PostgresEventStoreError::PositionOutOfRange`], for the same reason: a
    /// checkpoint is a `SequencePosition` stored in a signed `bigint`.
    #[error("stored checkpoint {value} is not a valid sequence position")]
    CheckpointOutOfRange {
        /// The value as Postgres returned it.
        value: i64,
    },
}
