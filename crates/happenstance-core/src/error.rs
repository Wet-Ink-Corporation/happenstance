//! Error types for the contract layer.
//!
//! Two distinct families live here:
//!
//! * **Validation errors** ([`InvalidTag`], [`InvalidEventType`],
//!   [`InvalidQuery`]) are returned by constructors that enforce the
//!   specification's invariants. They are programmer errors, not runtime
//!   conditions.
//! * **[`AppendError`]** is the outcome of an append. It separates
//!   [`ConditionViolated`] and a transient [`Busy`](AppendError::Busy) refusal
//!   from adapter-specific failures; its documentation says why that is in the
//!   type system rather than in a string.

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
    /// The tag contained a control character (Unicode `Cc`).
    #[error("a tag must not contain control characters")]
    ControlCharacter,
    /// The tag contained one of the seven explicit bidirectional formatting
    /// controls: U+202A–U+202E or U+2066–U+2069.
    ///
    /// Separate from [`ControlCharacter`](Self::ControlCharacter) because the
    /// two refusals have different remedies. A control character in an
    /// identifier is almost always an encoding bug; a bidirectional override is
    /// almost always an attack — a tag that renders as one thing in every
    /// console and matches another in every query.
    #[error("a tag must not contain bidirectional formatting controls")]
    BidirectionalControl,
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
    /// The event type contained a control character (Unicode `Cc`).
    #[error("an event type must not contain control characters")]
    ControlCharacter,
    /// The event type contained one of the seven explicit bidirectional
    /// formatting controls: U+202A–U+202E or U+2066–U+2069.
    ///
    /// Separate from [`ControlCharacter`](Self::ControlCharacter) for the reason
    /// [`InvalidTag::BidirectionalControl`] gives.
    #[error("an event type must not contain bidirectional formatting controls")]
    BidirectionalControl,
}

impl From<core::convert::Infallible> for InvalidEventType {
    /// Lets [`Event::new`](crate::Event::new) accept both `&str`, which
    /// converts fallibly, and an already-built [`EventType`](crate::EventType),
    /// whose conversion cannot fail.
    ///
    /// The match has no arms because [`Infallible`](core::convert::Infallible)
    /// has no values, and the compiler accepts an empty match on an uninhabited
    /// type as exhaustive. That is what lets an infallible conversion satisfy a
    /// fallible bound.
    fn from(never: core::convert::Infallible) -> Self {
        match never {}
    }
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
    /// One of the item's tags was itself invalid.
    ///
    /// This is what lets a command handler in a library crate build tags, items
    /// and a query, propagate all three with `?`, and still have one error type
    /// — without a bespoke enum and without reaching for `anyhow`, which the
    /// house style forbids in library code. The conversion direction is
    /// unambiguous: a query can contain tags, and a tag cannot contain a query.
    #[error(transparent)]
    Tag(#[from] InvalidTag),
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct ConditionViolated {
    /// The position of a conflicting event, when the adapter can identify one
    /// cheaply.
    ///
    /// **A hint, not a promise, and a caller must be written for `None`.** An
    /// adapter that detects the conflict without learning which event caused it
    /// reports `None`, and that is not a deficient adapter: a store reached over
    /// one-shot HTTP has no interactive transaction, so the only shape it can
    /// express is a conditional `INSERT … SELECT … WHERE NOT EXISTS`, which
    /// yields a boolean and no row. A retry loop that branches on this field
    /// being `Some` works against an in-process store and stops working against
    /// a remote one.
    pub conflicting_position: Option<SequencePosition>,
}

impl ConditionViolated {
    /// A violation with no identified conflicting event.
    #[must_use]
    pub const fn unspecified() -> Self {
        Self {
            conflicting_position: None,
        }
    }

    /// A violation naming the event that caused it.
    #[must_use]
    pub const fn at(position: SequencePosition) -> Self {
        Self {
            conflicting_position: Some(position),
        }
    }
}

impl core::fmt::Display for ConditionViolated {
    /// Renders the conflicting position when the adapter supplied one.
    ///
    /// Hand-written rather than a `thiserror` attribute because the message has
    /// two shapes, and the field the store already populates was previously
    /// carried and never shown — an operator reading a log got "the store
    /// already contains a matching event" and no way to find which.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("append condition violated: the store already contains a matching event")?;
        if let Some(position) = self.conflicting_position {
            write!(f, " at position {position}")?;
        }
        f.write_str("; rebuild the decision model and retry")
    }
}

impl core::error::Error for ConditionViolated {}

/// Why an append failed.
///
/// The DCB concurrency signal is lifted out of the adapter's error type and
/// into this enum on purpose. Every store fails differently — disk full, socket
/// closed, a lost acknowledgement — but every store fails *identically* when an
/// append condition is violated, and callers must be able to distinguish
/// "retry the decision" from "this went wrong" without pattern-matching on
/// strings or knowing which adapter they were handed.
///
/// A **busy** refusal is lifted out for the same reason, into
/// [`Busy`](Self::Busy). `SQLITE_BUSY` after a busy timeout and a PostgreSQL
/// serialisation failure after an adapter's own retry budget are different
/// driver errors with one meaning — *nothing was written, and the same decision
/// may be taken again* — and a generic caller that could only ask
/// [`is_condition_violated`](Self::is_condition_violated) could not see that
/// meaning behind [`Store`](Self::Store).
///
/// # Examples
///
/// ```
/// # use happenstance_core::{AppendError, ConditionViolated};
/// # fn handle<E: core::fmt::Debug>(result: Result<(), AppendError<E>>) {
/// match result {
///     Ok(()) => {}
///     // Expected under contention: rebuild the decision model and retry.
///     Err(AppendError::ConditionViolated(_)) => { /* re-decide */ }
///     // Transient, and nothing was written: deciding again is safe.
///     Err(AppendError::Busy(_)) => { /* re-decide, within your own bound */ }
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

    /// The batch exceeded a capacity the store documents.
    ///
    /// Distinct from [`Store`](Self::Store) on purpose, and the distinction is
    /// the whole reason the variant exists: a caller that cannot tell "this will
    /// never fit here, park it and tell a human" from "the disk is full, retry"
    /// has to guess, and a sync runner that guesses wrong drops an event
    /// permanently.
    ///
    /// A store MUST report a capacity refusal through this variant rather than
    /// through [`Store`](Self::Store), and MUST NOT truncate instead.
    #[error("append exceeds the store's {limit} limit: {len}")]
    ExceedsStoreLimit {
        /// Which limit was exceeded.
        limit: crate::limits::StoreLimit,
        /// The value that exceeded it — a byte count, a tag count or an event
        /// count, according to `limit`.
        len: usize,
    },

    /// The store refused the append for a transient reason, before the batch
    /// took any effect.
    ///
    /// **Nothing was written, and re-running the decision is safe.** That
    /// sentence is the variant's whole contract, and an adapter reporting it
    /// promises all three parts of it:
    ///
    /// * **Before any effect.** No event of the batch is in the store, and none
    ///   will appear later. A position the store allocated and then abandoned
    ///   may leave a gap, which the specification already permits.
    /// * **Transient.** The same call can succeed once whatever held the store
    ///   has let go: a lock not acquired within the adapter's own bound, or a
    ///   serialisation failure its own retry budget did not absorb. A refusal
    ///   that no retry can cure is [`Store`](Self::Store).
    /// * **Safe to re-run.** Read again, decide again, and append under the new
    ///   read's condition, which is the loop a
    ///   [`ConditionViolated`](Self::ConditionViolated) already asks for.
    ///   Because nothing landed, re-submitting the same batch under the same
    ///   condition is safe too: the condition still guards it.
    ///
    /// **An ambiguous outcome MUST stay [`Store`](Self::Store).** A commit whose
    /// acknowledgement was lost, a connection that dropped after `COMMIT` was
    /// sent, a timeout whose effect is unknown: each may have written, so none
    /// may be reported here, however transient its cause looks. `Busy` is a
    /// claim about what the store holds, not about why the driver failed. A
    /// caller who re-runs an unconditional append on the strength of it would
    /// otherwise append twice. A store that cannot tell is not busy.
    ///
    /// It is **not** the concurrency signal:
    /// [`is_condition_violated`](AppendError::is_condition_violated) answers
    /// `false`, because no condition was evaluated. Nor is it a capacity
    /// refusal, which is [`ExceedsStoreLimit`](Self::ExceedsStoreLimit) and will
    /// never fit. A store is never obliged to produce it: one that serialises
    /// its writers in-process, as `MemoryEventStore` does, has nothing to be
    /// busy with.
    ///
    /// The payload is the adapter's own error, carried for diagnostics exactly
    /// as [`Store`](Self::Store) carries it and reachable through
    /// [`source`](core::error::Error::source). Branch on the variant, never on
    /// the payload.
    #[error("the store was busy and refused the append before writing anything")]
    Busy(#[source] E),

    /// The adapter failed for its own reasons.
    ///
    /// Including every outcome it cannot vouch for: an append reported here may
    /// or may not have landed. [`Busy`](Self::Busy) is the refusal known to have
    /// written nothing.
    #[error(transparent)]
    Store(E),
}

impl<E> AppendError<E> {
    /// Whether this is the DCB concurrency signal rather than a store failure.
    pub const fn is_condition_violated(&self) -> bool {
        matches!(self, Self::ConditionViolated(_))
    }

    /// Whether the store refused transiently, before writing anything.
    ///
    /// `true` only for [`Busy`](Self::Busy). A caller that retries on this
    /// re-runs a decision that is known not to have landed. It is not the
    /// question [`is_condition_violated`](Self::is_condition_violated) asks, and
    /// a loop that wants both asks both.
    ///
    /// ```
    /// # use happenstance_core::AppendError;
    /// let busy: AppendError<std::io::Error> =
    ///     AppendError::Busy(std::io::Error::other("database is locked"));
    /// assert!(busy.is_busy());
    /// assert!(!busy.is_condition_violated());
    ///
    /// // A store failure is neither, whatever its message says.
    /// let broken: AppendError<std::io::Error> =
    ///     AppendError::Store(std::io::Error::other("database is locked"));
    /// assert!(!broken.is_busy());
    /// ```
    pub const fn is_busy(&self) -> bool {
        matches!(self, Self::Busy(_))
    }

    /// Maps the adapter-specific error wherever one is carried, in
    /// [`Store`](Self::Store) and in [`Busy`](Self::Busy), leaving every other
    /// variant untouched.
    ///
    /// Useful when a higher layer wraps an adapter's error in its own type. The
    /// variant survives the mapping: a busy refusal stays busy, because the
    /// classification is the store's claim about what it holds, and a wrapper
    /// has no standing to change it.
    ///
    /// ```
    /// # use happenstance_core::AppendError;
    /// let busy: AppendError<&str> = AppendError::Busy("database is locked");
    /// let wrapped: AppendError<String> = busy.map_store(str::to_owned);
    /// assert!(wrapped.is_busy());
    /// ```
    pub fn map_store<F, T>(self, f: F) -> AppendError<T>
    where
        F: FnOnce(E) -> T,
    {
        match self {
            Self::ConditionViolated(violation) => AppendError::ConditionViolated(violation),
            Self::NoEvents => AppendError::NoEvents,
            Self::ExceedsStoreLimit { limit, len } => AppendError::ExceedsStoreLimit { limit, len },
            Self::Busy(err) => AppendError::Busy(f(err)),
            Self::Store(err) => AppendError::Store(f(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::borrow::ToOwned;
    use alloc::string::ToString;

    use super::AppendError;

    /// A payload a test can name, with the message a driver would give it.
    #[derive(Debug, PartialEq, Eq, thiserror::Error)]
    #[error("database is locked")]
    struct Locked;

    /// `map_store` keeps a busy refusal busy.
    ///
    /// The wrong implementation this rejects is the one-arm shortcut, which
    /// maps `Busy` into `Store`. It compiles and keeps the payload, and it
    /// silently turns a refusal that wrote nothing into one a caller must treat
    /// as possibly written.
    #[test]
    fn map_store_keeps_the_busy_classification() {
        let mapped = AppendError::Busy(Locked).map_store(|locked| locked.to_string());
        assert_eq!(mapped, AppendError::Busy("database is locked".to_owned()));

        let mapped = AppendError::Store(Locked).map_store(|locked| locked.to_string());
        assert_eq!(mapped, AppendError::Store("database is locked".to_owned()));
    }

    /// A busy refusal says so, and keeps the adapter's error as its source.
    ///
    /// `Store` is `transparent` and renders the adapter's message alone. `Busy`
    /// cannot be, or an operator's log line would read the same for a refusal
    /// that wrote nothing and a failure that may have written.
    #[test]
    fn busy_names_itself_and_carries_the_adapter_error_as_its_source() {
        let busy = AppendError::Busy(Locked);
        assert!(busy.to_string().contains("busy"), "got: {busy}");
        let source = core::error::Error::source(&busy).map(ToString::to_string);
        assert_eq!(source.as_deref(), Some("database is locked"));

        let broken = AppendError::Store(Locked);
        assert!(!broken.to_string().contains("busy"), "got: {broken}");
    }
}
