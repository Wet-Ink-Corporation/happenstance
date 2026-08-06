//! Error types for the contract layer.
//!
//! Two distinct families live here:
//!
//! * **Validation errors** ([`InvalidTag`], [`InvalidEventType`],
//!   [`InvalidQuery`]) are returned by constructors that enforce the
//!   specification's invariants. They are programmer errors, not runtime
//!   conditions.
//! * **[`AppendError`]** is the outcome of an append, and separates the
//!   specification-defined [`ConditionViolated`] outcome from adapter-specific
//!   failures. See its documentation for why that distinction is in the type
//!   system rather than in a string.

use crate::SequencePosition;

/// A [`Tag`](crate::Tag) failed validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidTag {
    /// The tag, or one half of a `key:value` pair, was empty.
    #[error("a tag must not be empty")]
    Empty,
    /// The tag exceeded [`MAX_TAG_LEN`](crate::MAX_TAG_LEN) bytes.
    #[error("a tag must be at most {max} bytes, got {len}", max = crate::MAX_TAG_LEN)]
    TooLong {
        /// The rejected length, in bytes.
        len: usize,
    },
    /// The tag contained an ASCII control character.
    #[error("a tag must not contain control characters")]
    ControlCharacter,
    /// The key half of a `key:value` pair itself contained a colon, which would
    /// make [`Tag::key`](crate::Tag::key) ambiguous.
    #[error("the key of a `key:value` tag must not contain a colon")]
    ColonInKey,
}

/// An [`EventType`](crate::EventType) failed validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidEventType {
    /// The event type was empty. The specification requires every event to
    /// carry a type.
    #[error("an event type must not be empty")]
    Empty,
    /// The event type exceeded [`MAX_EVENT_TYPE_LEN`](crate::MAX_EVENT_TYPE_LEN)
    /// bytes.
    #[error("an event type must be at most {max} bytes, got {len}", max = crate::MAX_EVENT_TYPE_LEN)]
    TooLong {
        /// The rejected length, in bytes.
        len: usize,
    },
    /// The event type contained an ASCII control character.
    #[error("an event type must not contain control characters")]
    ControlCharacter,
}

/// A [`Query`](crate::Query) or [`QueryItem`](crate::QueryItem) failed
/// validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum InvalidQuery {
    /// A query was built from zero items. The specification requires a query to
    /// hold at least one item or to be the match-all query; use
    /// [`Query::all`](crate::Query::all) for the latter.
    #[error("a query must contain at least one item; use `Query::all()` to match everything")]
    NoItems,
    /// A query item constrained neither types nor tags, and would therefore
    /// match every event — which is [`Query::all`](crate::Query::all)'s job.
    #[error("a query item must constrain at least one of types or tags")]
    UnconstrainedItem,
    /// One of the item's event types was itself invalid.
    #[error(transparent)]
    EventType(#[from] InvalidEventType),
}

impl From<core::convert::Infallible> for InvalidQuery {
    /// Lets [`QueryItem::new`](crate::QueryItem::new) accept both `&str` (which
    /// converts fallibly) and an already-built
    /// [`EventType`](crate::EventType) (which cannot fail).
    fn from(never: core::convert::Infallible) -> Self {
        match never {}
    }
}

/// The specification-defined outcome of an append that was rejected because the
/// store already held an event matching the
/// [`AppendCondition`](crate::AppendCondition).
///
/// This is the DCB concurrency signal. Callers respond by rebuilding their
/// decision model from the current state and retrying — it is an expected,
/// routine outcome under contention, not a fault.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("append condition violated: the store already contains a matching event")]
#[non_exhaustive]
pub struct ConditionViolated {
    /// The position of a conflicting event, when the adapter can identify one
    /// cheaply.
    ///
    /// Purely informational: adapters that detect the conflict without learning
    /// which event caused it (a conditional `INSERT ... WHERE NOT EXISTS`, for
    /// instance) report `None`, and callers must not depend on this being set.
    pub conflicting_position: Option<SequencePosition>,
}

impl ConditionViolated {
    /// A violation with no identified conflicting event.
    pub const fn unspecified() -> Self {
        Self {
            conflicting_position: None,
        }
    }

    /// A violation naming the event that caused it.
    pub const fn at(position: SequencePosition) -> Self {
        Self {
            conflicting_position: Some(position),
        }
    }
}

/// Why an append failed.
///
/// The DCB concurrency signal is lifted out of the adapter's error type and
/// into this enum on purpose. Every store fails differently — disk full, socket
/// closed, `SQLITE_BUSY` — but every store fails *identically* when an append
/// condition is violated, and callers must be able to distinguish "retry the
/// decision" from "this went wrong" without pattern-matching on strings or
/// knowing which adapter they were handed.
///
/// # Examples
///
/// ```
/// # use happenstance_core::{AppendError, ConditionViolated};
/// # fn handle<E: core::fmt::Debug>(result: Result<(), AppendError<E>>) {
/// match result {
///     Ok(()) => {}
///     // Expected under contention: rebuild the decision model and retry.
///     Err(AppendError::ConditionViolated(_)) => { /* retry */ }
///     // Everything else is a genuine failure.
///     Err(other) => panic!("append failed: {other:?}"),
/// }
/// # }
/// ```
///
/// Note the catch-all arm: this enum is `#[non_exhaustive]`, so matching every
/// named variant is not enough and a wildcard is required.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum AppendError<E> {
    /// The store already contained an event matching the append condition.
    #[error(transparent)]
    ConditionViolated(#[from] ConditionViolated),

    /// The caller passed an empty batch.
    ///
    /// The specification defines a batch as a non-empty collection of events,
    /// so there is no position an adapter could honestly return. This is a
    /// caller bug, not a store failure.
    #[error("an append must contain at least one event")]
    NoEvents,

    /// The adapter failed for its own reasons.
    #[error(transparent)]
    Store(E),
}

impl<E> AppendError<E> {
    /// Whether this is the DCB concurrency signal rather than a store failure.
    pub const fn is_condition_violated(&self) -> bool {
        matches!(self, Self::ConditionViolated(_))
    }

    /// Maps the adapter-specific error, leaving a
    /// [`ConditionViolated`](Self::ConditionViolated) untouched.
    ///
    /// Useful when a higher layer wraps an adapter's error in its own type.
    pub fn map_store<F, T>(self, f: F) -> AppendError<T>
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Self::ConditionViolated(violation) => AppendError::ConditionViolated(violation),
            Self::NoEvents => AppendError::NoEvents,
            Self::Store(err) => AppendError::Store(f(err)),
        }
    }
}
