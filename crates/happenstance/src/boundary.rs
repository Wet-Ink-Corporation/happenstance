//! The derived query, and the fold that consumes what it nominated.

use core::marker::PhantomData;

use happenstance_core::{EventType, InvalidQuery, Query, QueryItem, SequencedEvent, Tags};

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
/// Every item below the seal is well-formed on purpose. `Ev` is a complete
/// [`DomainEvent`], so `type Event` discharges its own bound and the **only**
/// diagnostic left is the seal — one `E0277`, naming
/// `crate::sealed::Sealed` as the bound `Divergent` does not satisfy. A fence
/// whose associated type also failed would stay red with the supertrait
/// deleted, and would therefore prove nothing about it (RS-62-1: do not trust
/// the error code — measure what the fence actually rejects).
///
/// ```compile_fail
/// // compile_fail: boundary_cannot_be_implemented_outside_the_crate
/// use happenstance::{Boundary, Codec, CodecError, DomainEvent};
/// use happenstance::{EventType, InvalidQuery, Query, SequencedEvent};
/// use happenstance::{Tags, bytes::Bytes};
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Ev;
///
/// const EV: EventType = EventType::from_static("Ev");
/// impl DomainEvent for Ev {
///     const EVENT_TYPES: &'static [EventType] = &[EV];
///     fn event_type(&self) -> EventType { EV }
///     fn tags(&self) -> Tags { Tags::empty() }
///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
///         c.encode(self)
///     }
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
/// }
///
/// struct Divergent;
///
/// impl Boundary for Divergent {
///     type Event = Ev;
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

        // The nomination check *is* the derived query, asked. A second
        // predicate spelled from the same two inputs would be the second place
        // the event set is named, which is the defect this trait removes; and
        // `Query::matches` is the contract's only filter vocabulary — there is
        // no other one to reach for. The cost is one derivation per event.
        // Caching it is what NF-002 rejects: a cached query makes the
        // composite's answer depend on when the boundary was built.
        let Ok(query) = self.query() else {
            // Unreachable for a well-formed model: the `const` above already
            // made an empty `EVENT_TYPES` a compile error, and a non-empty one
            // always yields a constrained item. Spelled as "nominated nothing,
            // folded nothing" rather than as an `unwrap`.
            return Ok(());
        };
        if !query.matches(event.event.event_type(), event.event.tags()) {
            return Ok(());
        }

        // Through the tag, not straight to the codec in hand: an event written
        // under another encoding decodes under the one its own tag names, so a
        // store that holds two generations folds into one model without the
        // caller branching on encoding anywhere.
        let decoded = crate::codec::decode_event::<M::Event, C>(codec, event)?;
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
