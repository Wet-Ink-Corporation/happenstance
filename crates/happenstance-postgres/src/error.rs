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

    /// `store_meta` has no `store_id` row.
    ///
    /// Almost always means migration 1 has not been applied to the schema this
    /// pool resolves against. Spelled as its own variant rather than as a
    /// `Driver` error because "the schema is not there" and "the server refused
    /// the query" send a reader to entirely different places, and at 2am the
    /// difference is the whole diagnosis.
    #[error(
        "this store has no identity: `store_meta` has no `store_id` row, so migration 1 has probably not been applied to this schema"
    )]
    MissingIdentity,

    /// The `store_id` row is not sixteen bytes.
    ///
    /// [`StoreId`](happenstance_core::StoreId) is `[u8; 16]` exactly, and the
    /// schema's `CHECK` says so — so reaching this means something wrote the row
    /// without going through either.
    #[error("this store's identity is {len} bytes, and a `StoreId` is exactly 16")]
    MalformedIdentity {
        /// The length actually stored.
        len: usize,
    },

    /// A row carries no `EventId`.
    ///
    /// Every event this store appends is stamped with `origin_store` and
    /// `origin_position` in the same statement that inserts it, so a row missing
    /// either was written by something other than this adapter. Reported rather
    /// than papered over with a synthesised id, because a synthesised `EventId`
    /// is a replication identity this store has no authority to mint.
    #[error(
        "the event at position {position} carries no origin identity; it was not written by this adapter"
    )]
    UnstampedEvent {
        /// Where the unstamped row sits.
        position: happenstance_core::SequencePosition,
    },

    /// The task a read stream handed its work to did not finish.
    ///
    /// `read` spawns the cursor's opening onto the runtime rather than polling
    /// it inline, so that the read's state is fixed no later than the first poll
    /// (ES-11) rather than whenever the caller next gets round to polling. A
    /// spawned task can panic or be cancelled, and neither is something the
    /// caller can act on — but ending the stream silently would look exactly
    /// like an empty log, which is why this is an error rather than a `None`.
    #[error("the task this read handed its work to did not finish")]
    Worker(#[from] tokio::task::JoinError),

    /// `read` was polled outside a tokio runtime.
    ///
    /// The stream needs a runtime to hand its work to, and `read` is not `async`
    /// — deliberately, because nesting the stream inside a future drops `+ Send`
    /// from it — so nothing forces the caller to be on one. Reported rather than
    /// panicked: a library that panics on a legitimate call pattern has made the
    /// caller's mistake into its own crash.
    #[error(
        "this read stream was polled outside a tokio runtime, and it needs one to open its snapshot"
    )]
    NoRuntime,

    /// A row's `xact_id` could not be read as the mechanism's frontier type.
    ///
    /// The visibility mechanism stores `pg_current_xact_id()` beside each row
    /// and compares it against `pg_snapshot_xmin(pg_current_snapshot())`. A
    /// value that will not decode means the column is not the one this adapter
    /// wrote, which is a schema disagreement rather than a transient fault —
    /// worth its own name because every other symptom of it is a wrong answer
    /// rather than an error.
    #[error(
        "a row's `xact_id` could not be read as an xid8; the schema is not the one this adapter wrote"
    )]
    UnreadableTransactionId,
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
