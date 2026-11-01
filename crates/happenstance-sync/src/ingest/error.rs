//! Why an ingest did not land.
//!
//! # Why ingest has an error of its own
//!
//! The SQLite spike reused `AppendError<E>`, and two of its arms cannot happen
//! here: `NoEvents`, because an empty ingest is a pull that found nothing and
//! answers `Ok(Ingested::default())`, and `ConditionViolated`, because no
//! condition is evaluated (SY-1). A runner matching a type whose name says
//! "append" would have to handle two states that are illegal on this path.
//!
//! The port owns the shape and the adapter supplies only its store error `E`,
//! the pairing `happenstance-core` uses for `AppendError<E>` and
//! `EventStore::Error`. That is what lets a runner, and the ingest rules in
//! `happenstance-sync-testkit`, recognise a busy store or an unheld own
//! identity without knowing which adapter they hold.

use happenstance_core::{EventId, StoreLimit};

use crate::classify::{Classify, FailureClass};

/// Why an [`ingest`](crate::IngestStore::ingest) did not land.
///
/// Every arm, save the one case below, means **nothing in the call was
/// written**: a store refuses before its transaction opens or rolls the whole
/// call back. A runner therefore retries, splits, parks or stops on the
/// [`class`](Classify::class) alone, and never has to ask the store what part
/// of a batch survived.
///
/// One case leaves the outcome unknown instead: a [`Store`](Self::Store) error
/// raised after the commit was sent and its reply lost. The whole call either
/// landed or did not, never part of it, and re-delivering it converges, because
/// an event the store already holds is a skip.
///
/// No arm refuses an event for its content. The event is already durable at
/// its origin, so refusing it only guarantees the two logs never converge
/// (SY-1, SY-33). [`UnheldOwnIdentity`](Self::UnheldOwnIdentity) is not such
/// a refusal: it is about this store's own identity, not the event's.
///
/// `#[non_exhaustive]`, so an arm added later, such as SY-33's, is not
/// a breaking change. No bounds sit on the enum itself (they go on the
/// `impl`s), so `IngestError<E>` is `Send` and `Sync` exactly when `E` is, and
/// the Durable Object's `Rc`-holding error stays legal (ADR-0009).
///
/// # Examples
///
/// A runner reads the class, never the variant:
///
/// ```
/// use happenstance_core::{EventId, SequencePosition, StoreId};
/// use happenstance_sync::{Classify, FailureClass, IngestError};
///
/// /// An adapter's own failure.
/// #[derive(Debug)]
/// struct DiskFull;
///
/// impl core::fmt::Display for DiskFull {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
///         f.write_str("the disk is full")
///     }
/// }
///
/// impl core::error::Error for DiskFull {}
///
/// impl Classify for DiskFull {
///     fn class(&self) -> FailureClass {
///         FailureClass::Transient
///     }
/// }
///
/// let own = StoreId::from_bytes([7; 16]);
/// let claim = EventId::new(own, SequencePosition::new(3).ok_or("position zero")?);
/// let refused: IngestError<DiskFull> = IngestError::unheld_own_identity(claim);
///
/// // Halt this edge: an operator re-mints the store, and the same batch lands.
/// assert_eq!(refused.class(), FailureClass::Stop);
/// // The adapter's own failure answers its own class.
/// assert_eq!(IngestError::Store(DiskFull).class(), FailureClass::Transient);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum IngestError<E> {
    /// Over a ceiling the store declares, checked before anything is written.
    ///
    /// It mirrors `AppendError::ExceedsStoreLimit`, so a runner tells "this
    /// will never fit here" from "retry" without knowing the adapter (VT-25).
    /// Two limits are about the batch:
    /// [`EventsPerBatch`](StoreLimit::EventsPerBatch) and
    /// [`TagsPerBatch`](StoreLimit::TagsPerBatch) are
    /// [`Split`](FailureClass::Split), because the batch is too big and its
    /// events are not. Groups are atomic, though, so a call of one group over
    /// either limit cannot be split further, and what a runner does then is
    /// #244's. Every other limit is about one event, and is
    /// [`Park`](FailureClass::Park).
    #[error("ingest exceeds the store's {limit} limit: {len}")]
    #[non_exhaustive]
    ExceedsStoreLimit {
        /// Which ceiling was exceeded.
        limit: StoreLimit,
        /// The magnitude that exceeded it, in the limit's own unit.
        len: usize,
    },

    /// An event claims this store's own identity at a position this store
    /// does not hold.
    ///
    /// This store minted that identity, so holding nothing under it means the
    /// store was restored or rewound under an identity it should have re-minted
    /// (VT-6). Writing the event would wedge the local append that later
    /// reaches the claimed position, so nothing in the call is written.
    ///
    /// [`Stop`](FailureClass::Stop), because the remedy is configuration: an
    /// operator re-mints the store's identity, and the same batch then lands,
    /// with the claim as a foreign event (SY-38). A retry without the remedy
    /// fails the same way. The refusal is about this store's identity, not the
    /// event's content or the log's, which is why it is not SY-33's forbidden
    /// kind of refusal.
    #[error(
        "event {id} claims this store's identity and this store does not hold it: \
         the store was restored or rewound without re-minting its identity"
    )]
    #[non_exhaustive]
    UnheldOwnIdentity {
        /// The first such identity the write met.
        id: EventId,
    },

    /// The store's write lock was not acquired, and nothing was written.
    ///
    /// [`Transient`](FailureClass::Transient) whatever the cause's own class:
    /// the same call can succeed once the holder lets go, which is
    /// `AppendError::Busy`'s contract on the ingest path. A store that cannot
    /// tell contention from failure reports [`Store`](Self::Store) instead.
    #[error("the store was busy and refused the ingest before writing anything")]
    Busy(#[source] E),

    /// The adapter failed, and its own [`Classify`] impl states the class.
    ///
    /// The one arm whose outcome can be unknown. When it is raised after the
    /// commit was sent and the reply lost, the call either landed whole or not
    /// at all, and re-delivery converges.
    #[error(transparent)]
    Store(E),
}

/// Constructors for the two struct arms.
///
/// Both arms are `#[non_exhaustive]`, so a field can be added to either
/// later without breaking a match: the offending event's identity on
/// `ExceedsStoreLimit` is the likely one, for a runner that parks a single
/// event. The price is that a struct literal is refused outside this crate,
/// so an adapter builds the arms through these. A match outside the crate
/// spells `{ id, .. }`.
impl<E> IngestError<E> {
    /// [`ExceedsStoreLimit`](Self::ExceedsStoreLimit): `len`, in `limit`'s
    /// own unit, is over the store's ceiling.
    #[must_use]
    pub const fn exceeds_store_limit(limit: StoreLimit, len: usize) -> Self {
        Self::ExceedsStoreLimit { limit, len }
    }

    /// [`UnheldOwnIdentity`](Self::UnheldOwnIdentity): `id` claims this
    /// store's identity, and this store does not hold it.
    #[must_use]
    pub const fn unheld_own_identity(id: EventId) -> Self {
        Self::UnheldOwnIdentity { id }
    }
}

/// Each arm's class is fixed by the arm, except [`Store`](IngestError::Store),
/// which asks the adapter's error. So the class is a function of the value
/// alone (SY-36).
impl<E: Classify> Classify for IngestError<E> {
    fn class(&self) -> FailureClass {
        match self {
            Self::ExceedsStoreLimit { limit, .. } => match limit {
                StoreLimit::EventsPerBatch | StoreLimit::TagsPerBatch => FailureClass::Split,
                // `StoreLimit` is `#[non_exhaustive]` and core's, so this arm
                // also answers a limit added later. Such a limit is assumed to
                // be about one event, as every limit but the batch's two are
                // today, and parking cannot loop where splitting can.
                _ => FailureClass::Park,
            },
            Self::UnheldOwnIdentity { .. } => FailureClass::Stop,
            Self::Busy(_) => FailureClass::Transient,
            Self::Store(error) => error.class(),
        }
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{EventId, SequencePosition, StoreId, StoreLimit};

    use super::IngestError;
    use crate::{Classify, FailureClass};

    /// An adapter error that cannot occur, for the arms that carry none.
    #[derive(Debug)]
    enum Never {}

    impl Classify for Never {
        fn class(&self) -> FailureClass {
            match *self {}
        }
    }

    impl core::fmt::Display for Never {
        fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match *self {}
        }
    }

    /// An adapter error that answers whatever class it was built with.
    #[derive(Debug)]
    struct Answers(FailureClass);

    impl Classify for Answers {
        fn class(&self) -> FailureClass {
            self.0
        }
    }

    const EVERY_CLASS: [FailureClass; 4] = [
        FailureClass::Transient,
        FailureClass::Split,
        FailureClass::Park,
        FailureClass::Stop,
    ];

    #[test]
    fn exceeding_a_batch_limit_is_split() {
        for limit in [StoreLimit::EventsPerBatch, StoreLimit::TagsPerBatch] {
            let over = IngestError::<Never>::ExceedsStoreLimit { limit, len: 2 };
            assert_eq!(over.class(), FailureClass::Split, "{limit}");
        }
    }

    /// The text a refused call carries out of a Durable Object, which the
    /// workerd harness matches on.
    #[test]
    fn a_tags_per_batch_refusal_names_its_limit_and_total() {
        let over = IngestError::<Never>::ExceedsStoreLimit {
            limit: StoreLimit::TagsPerBatch,
            len: 525_312,
        };
        assert_eq!(
            over.to_string(),
            "ingest exceeds the store's tags per batch limit: 525312"
        );
    }

    #[test]
    fn exceeding_a_per_event_limit_is_park() {
        for limit in [
            StoreLimit::EventDataLen,
            StoreLimit::TagsPerEvent,
            StoreLimit::MetadataLen,
        ] {
            let over = IngestError::<Never>::ExceedsStoreLimit { limit, len: 2 };
            assert_eq!(over.class(), FailureClass::Park, "{limit}");
        }
    }

    #[test]
    fn an_unheld_own_identity_is_stop() {
        let claim = EventId::new(StoreId::from_bytes([3; 16]), SequencePosition::FIRST);
        let refused = IngestError::<Never>::UnheldOwnIdentity { id: claim };
        assert_eq!(refused.class(), FailureClass::Stop);
    }

    #[test]
    fn busy_is_transient_whatever_the_cause_says() {
        for cause in EVERY_CLASS {
            assert_eq!(
                IngestError::Busy(Answers(cause)).class(),
                FailureClass::Transient,
                "a busy refusal whose cause answers {cause:?}"
            );
        }
    }

    #[test]
    fn a_store_error_answers_its_own_class() {
        for class in EVERY_CLASS {
            assert_eq!(IngestError::Store(Answers(class)).class(), class);
        }
    }
}
