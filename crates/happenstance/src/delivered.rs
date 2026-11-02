//! The event a projection's `apply` is handed, and its identity.
//!
//! Its own module rather than a section of `runner.rs`, which is already the
//! longest file in the crate, and a top-level one rather than
//! `runner/delivered.rs`, because `tests/doc_budget.rs` reads `src/*.rs` and
//! nothing below it: a nested file would escape the density budget unseen.

use happenstance_core::EventId;

/// One decoded event handed to `Projection::apply`, and its identity.
///
/// It is the argument of [`Projection::apply`](crate::Projection::apply).
/// [`into_event`](Self::into_event) gives back the application's own enum to
/// match on, and [`event`](Self::event) borrows it.
///
/// [`id`](Self::id) is the event's identity: the store that **originated** it
/// and the position it was given there. Both are the same on every replica
/// that holds the event, so a read model keyed on them agrees across replicas.
///
/// # There is no arrival position, on purpose
///
/// Nothing here yields the position the event was read at, the local
/// [`SequencedEvent::position`](happenstance_core::SequencedEvent::position).
/// Two replicas that ingested the same events in different orders assign them
/// different local positions, so a read model that folded one in would differ
/// between replicas that hold the same log (SY-21). `id().position()` is the
/// **origin's** position, which every replica shares, and it is allowed.
///
/// The first fence compiles, and is the control: it is what goes red if this
/// type or its accessors are renamed.
///
/// ```
/// use happenstance::{Delivered, EventId};
/// fn inspect(delivered: &Delivered<u8>) -> (EventId, u8) {
///     (delivered.id(), *delivered.event())
/// }
/// # let _ = inspect;
/// ```
///
/// The second asks for the arrival position, and does not compile, because
/// there is no such method. Rustdoc 1.97.1 does not check the error code on a
/// `compile_fail` fence (RS-62-1), so the `E0599` below is a statement of
/// intent and the control above is what keeps this fence honest.
///
/// ```compile_fail,E0599
/// use happenstance::Delivered;
/// fn leak(delivered: &Delivered<u8>) {
///     let _ = delivered.position();
/// }
/// # let _ = leak;
/// ```
///
/// `#[non_exhaustive]` although every field is private already: it costs
/// nothing, and it says on the page that a later fact may join these two.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Delivered<E> {
    id: EventId,
    event: E,
}

impl<E> Delivered<E> {
    /// Pairs a decoded event with the identity it was read under.
    ///
    /// The runner is the one caller in this crate. It is public so that an
    /// application can unit-test its own `apply` without a store or a runner.
    /// It takes an [`EventId`], so a local position has nowhere to go. A later
    /// fact, such as when the event was recorded, arrives as an `Option` or
    /// through a builder, and this keeps its two arguments.
    ///
    /// Not `const`, deliberately: constness is a promise a later field might
    /// not be able to keep, and nothing needs a `Delivered` at compile time.
    #[must_use]
    pub fn new(id: EventId, event: E) -> Self {
        Self { id, event }
    }

    /// The event's identity: its origin store and its position there.
    ///
    /// By value, because an [`EventId`] is `Copy`.
    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    /// The decoded event, borrowed.
    #[must_use]
    pub const fn event(&self) -> &E {
        &self.event
    }

    /// The decoded event, by value: the application's own enum to match on.
    #[must_use]
    pub fn into_event(self) -> E {
        self.event
    }
}

#[cfg(test)]
mod tests {
    use happenstance_core::{EventId, SequencePosition, StoreId};

    use super::Delivered;

    #[test]
    fn delivered_carries_its_identity_and_its_event() {
        let position = SequencePosition::new(30).expect("thirty is not zero");
        let id = EventId::new(StoreId::from_bytes([7; 16]), position);

        let delivered = Delivered::new(id, 7u8);

        assert_eq!(delivered.id(), id);
        assert_eq!(*delivered.event(), 7);
        assert_eq!(delivered.into_event(), 7);
    }
}
