//! The derived query, and the fold that consumes what it nominated.

use core::marker::PhantomData;

use happenstance_core::{Event, EventType, InvalidQuery, Query, QueryItem, SequencedEvent, Tags};

use crate::codec::{Codec, CodecError};
use crate::domain::{DecisionModel, DomainEvent};

/// What a command loop consumes: a decision model, or a tuple of them.
///
/// # The query is derived, and there is nowhere to put a hand-written one
///
/// `query` is **not** a method on [`DecisionModel`]. A provided method there
/// could be overridden, and an overridden derivation is a hand-maintained
/// query — the second place an event set gets named, which is exactly the
/// defect this trait removes. A free `derive_query::<M>()` function lost for
/// the same reason from the other side: a caller can ignore it and pass their
/// own query to the read.
///
/// So the derivation lives here, on a trait that is **sealed** by a private
/// supertrait: blanket-implemented for every [`DecisionModel`], and
/// implemented for tuples of them. A third implementation cannot be written
/// outside this crate, because the supertrait cannot be named outside it.
///
/// ```compile_fail
/// // compile_fail: boundary_cannot_be_implemented_outside_the_crate
/// use happenstance::{Boundary, Codec, CodecError, DomainEvent};
/// use happenstance::{EventType, InvalidQuery, Query, SequencedEvent};
///
/// struct Divergent;
///
/// impl Boundary for Divergent {
///     type Event = Divergent;
///     fn query(&self) -> Result<Query, InvalidQuery> {
///         Ok(Query::all())
///     }
///     fn absorb<C: Codec>(&mut self, _e: &SequencedEvent, _c: &C)
///         -> Result<(), CodecError> { Ok(()) }
/// }
/// ```
///
/// The compiling companion is [`DecisionModel`]'s example: writing the model
/// is what implements this trait.
pub trait Boundary: crate::sealed::Sealed {
    /// The one domain enum every member of this boundary folds.
    ///
    /// Present from birth rather than added later: the trait is sealed and the
    /// crate publishes from it, and growing a sealed trait a required item is
    /// a breaking change no downstream crate could have prepared for.
    type Event: DomainEvent;

    /// The query this boundary reads with, derived from its event set.
    ///
    /// The event set is the same `EVENT_TYPES` the fold is exhaustive over, so
    /// the query and the fold cannot name different sets.
    ///
    /// An ordinary value: bind it, print it, assert on it. A derivation
    /// reachable only from inside a read would hide its own filter, which is
    /// the failure this trait exists to prevent.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidQuery::UnconstrainedItem`] when the boundary
    /// constrains neither event types nor tags. That is [`Query::all`] wearing
    /// a disguise, and widening a caller's boundary to the whole log is worth
    /// refusing out loud. The other route to it — an empty `EVENT_TYPES` — is
    /// a compile error, so this arm is unreachable for a well-formed model and
    /// is kept anyway: the contract crate offers no infallible constructor for
    /// pre-validated inputs, and library code here does not `unwrap`.
    fn query(&self) -> Result<Query, InvalidQuery>;

    /// Decodes one read event and folds it, if this boundary nominated it.
    ///
    /// An event the derived query never nominated is skipped **by
    /// nomination** — the query is asked, not the decoder — so an over-wide
    /// read costs nothing and reports nothing.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::UnknownEventType`] when a **nominated** event is
    /// not one the domain type declares, because that is a real disagreement
    /// between the declaration and the fold, and the codec's own failure when
    /// a nominated payload cannot be decoded.
    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C) -> Result<(), CodecError>;
}

impl<M: DecisionModel> Boundary for M {
    type Event = M::Event;

    fn query(&self) -> Result<Query, InvalidQuery> {
        // Reading the `const` is what evaluates it, here, once per model.
        let () = AtLeastOneType::<M::Event>::CHECKED;
        derive_query(M::Event::EVENT_TYPES, self.scope())
    }

    fn absorb<C: Codec>(&mut self, event: &SequencedEvent, codec: &C) -> Result<(), CodecError> {
        let () = AtLeastOneType::<M::Event>::CHECKED;
        if !nominates(M::Event::EVENT_TYPES, self.scope(), &event.event) {
            return Ok(());
        }
        let decoded = M::Event::decode(codec, event.event.event_type(), event.event.data())?;
        self.apply(decoded);
        Ok(())
    }
}

/// The claim a signature cannot make: a domain type declares an event type.
///
/// The claim lives in a `const` **item** so that reading it is what evaluates
/// it, once per implementing type. A module-scope `const _: () = …` cannot see
/// an implementor's associated const and would assert nothing at all.
pub(crate) struct AtLeastOneType<E: DomainEvent>(PhantomData<E>);

impl<E: DomainEvent> AtLeastOneType<E> {
    pub(crate) const CHECKED: () = assert!(
        !E::EVENT_TYPES.is_empty(),
        "a DomainEvent must declare at least one event type in EVENT_TYPES",
    );
}

/// The derivation: one query item over the declared types and the scope.
///
/// # Errors
///
/// Returns [`InvalidQuery::UnconstrainedItem`] when `types` and `scope` are
/// both empty — a boundary that constrains nothing.
pub(crate) fn derive_query(types: &[EventType], scope: &Tags) -> Result<Query, InvalidQuery> {
    let item = QueryItem::new(types.iter().cloned(), scope.clone())?;
    Ok(Query::from_item(item))
}

/// Whether the query derived from `types` and `scope` nominates `event`.
///
/// Spelled from the same two inputs [`derive_query`] reads, so the fold and
/// the query cannot disagree about what was selected.
pub(crate) fn nominates(types: &[EventType], scope: &Tags, event: &Event) -> bool {
    types.contains(event.event_type()) && event.tags().contains_all(scope)
}
