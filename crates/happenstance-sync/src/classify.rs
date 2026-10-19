//! What a runner does next, stated by the error that stopped it (SY-36).
//!
//! # Why the class is on the error
//!
//! [`SyncPeer::Error`](crate::SyncPeer::Error) is bounded on [`Classify`], so
//! every error a peer reports says which of four remedies applies. The
//! alternative that lost was a method on the peer,
//! `fn classify(&self, error: &Self::Error) -> FailureClass`. It works only
//! while the peer is in hand, and an error does not stay beside its peer: a
//! runner logs it, boxes it into a report, and carries it across an `.await`
//! after the handle is gone, which SY-16 makes the normal way an edge ends. A
//! bound on the associated type makes the class travel with the value — the
//! placement ADR-0009 chose for an error's thread-safety, for the same reason.
//!
//! The other two alternatives were rejected in
//! [ADR-0026](../../../../.kb/decisions/0026-what-a-sync-peer-is.md): an opaque
//! error, which a runner can treat only one way, and a runner-side table from
//! error type to class, which cannot be written because one HTTP status type is
//! a stop at 401, a split at 413 and a retry at 503.
//!
//! # No `Send`, and object-safe
//!
//! One `&self` method and no generics, so `&dyn Classify` works and the Durable
//! Object's `Rc`-holding error implements it unchanged. A `Send` bound here
//! would undo the measurement on [`SyncPeer::Error`](crate::SyncPeer::Error).

use crate::peer::SyncError;

/// The remedy a failure calls for. One of four, and open to a fifth.
///
/// `#[non_exhaustive]`, so outside this crate a `match` needs a wildcard arm and
/// a fifth class is not a breaking change. The price is that a runner decides
/// what an unknown class means; that is #244's, with the rest of what a runner
/// does per class.
///
/// There is no class for a refusal on the content of the log — a conflicting
/// fact, a violated condition — because SY-33 forbids that refusal. A peer that
/// wants one has found a violation, not a fifth remedy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FailureClass {
    /// The exchange failed and the same exchange can succeed later: retry, with
    /// jittered and capped backoff.
    Transient,
    /// The batch is over a limit the peer has, and its events are not: halve the
    /// batch and retry.
    Split,
    /// This event can never land on this peer, as VT-25's `ExceedsStoreLimit`
    /// says: hold it, report it, and carry on with the rest.
    Park,
    /// Authorisation or configuration: halt this edge and report it.
    Stop,
}

/// An error that states its [`FailureClass`].
///
/// The class is a function of the error alone (SY-36): the same value always
/// answers the same, whatever was retried before it or whichever batch carried
/// it. Only the adapter that produced an error can say which class it is, so
/// the adapter implements this and a runner never infers it.
///
/// # Examples
///
/// ```
/// use happenstance_sync::{Classify, FailureClass};
///
/// #[derive(Debug)]
/// enum HttpError {
///     Status(u16),
///     Timeout,
/// }
///
/// impl Classify for HttpError {
///     fn class(&self) -> FailureClass {
///         match self {
///             Self::Status(401 | 403) => FailureClass::Stop,
///             Self::Status(413) => FailureClass::Split,
///             Self::Status(_) | Self::Timeout => FailureClass::Transient,
///         }
///     }
/// }
///
/// assert_eq!(HttpError::Status(413).class(), FailureClass::Split);
/// assert_eq!(HttpError::Timeout.class(), FailureClass::Transient);
/// ```
pub trait Classify {
    /// The remedy this failure calls for.
    fn class(&self) -> FailureClass;
}

/// The runner's own failures state a class too, so a runner handles them and a
/// peer's through one path (ADR-0026 §2.4).
impl Classify for SyncError {
    fn class(&self) -> FailureClass {
        match self {
            Self::BatchTooLarge { .. } => FailureClass::Split,
            // A gap below the floor needs a re-seed or an accepted loss, which a
            // retry cannot supply; an unknown peer is configuration.
            Self::BelowRetentionFloor { .. } | Self::UnknownPeer { .. } => FailureClass::Stop,
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::boxed::Box;

    use happenstance_core::{EventId, SequencePosition, StoreId};

    use super::{Classify, FailureClass};
    use crate::peer::SyncError;

    #[test]
    fn the_runners_own_errors_state_adr_0026s_classes() {
        let floor = EventId::new(StoreId::from_bytes([1; 16]), SequencePosition::FIRST);
        assert_eq!(
            SyncError::BatchTooLarge {
                actual: 2,
                limit: 1
            }
            .class(),
            FailureClass::Split
        );
        assert_eq!(
            SyncError::BelowRetentionFloor { floor }.class(),
            FailureClass::Stop
        );
        assert_eq!(
            SyncError::UnknownPeer {
                name: Box::from("hub")
            }
            .class(),
            FailureClass::Stop
        );
    }
}
