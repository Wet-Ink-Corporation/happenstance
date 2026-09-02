//! The domain vocabulary: one enum of events, one struct that folds them.

use happenstance_core::bytes::Bytes;
use happenstance_core::{EventType, Tags};

use crate::codec::{Codec, CodecError};

/// A Rust type's mapping to the event types and tags a store understands.
///
/// The event set is declared **once**, as a `const`, and the fold that
/// interprets it is exhaustive over the same enum. That single declaration is
/// what a boundary's query is derived from, so the query and the fold cannot
/// name different sets.
///
/// # An empty `EVENT_TYPES` does not compile
///
/// A domain type that declares no event types would derive a query
/// constraining nothing. That is caught by a `const` evaluated once per
/// implementing type rather than on the first read: the alternative — a
/// run-time `Err` — charges the ceremony of an explicit declaration and still
/// defers a mistake the compiler could have caught.
///
/// ```compile_fail
/// // compile_fail: empty_event_types_does_not_compile
/// use happenstance::{Boundary, Codec, CodecError, DecisionModel};
/// use happenstance::{DomainEvent, EventType, Tags, bytes::Bytes};
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// struct Nothing;
///
/// impl DomainEvent for Nothing {
///     const EVENT_TYPES: &'static [EventType] = &[];
///     fn event_type(&self) -> EventType {
///         EventType::from_static("Nothing")
///     }
///     fn tags(&self) -> Tags { Tags::empty() }
///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
///         c.encode(self)
///     }
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
/// }
///
/// #[derive(Clone)]
/// struct Empty(Tags);
///
/// impl DecisionModel for Empty {
///     type Event = Nothing;
///     fn scope(&self) -> &Tags { &self.0 }
///     fn apply(&mut self, _event: Nothing) {}
/// }
///
/// let _ = Empty(Tags::empty()).query();
/// ```
///
/// The spelling is bare `compile_fail`, never `compile_fail,E0080`: rustdoc
/// silently ignores an error code it cannot match, so the stricter-looking
/// spelling is the weaker check. Its companion — the same shape with a
/// non-empty declaration — is on [`DecisionModel`], which is where it can also
/// show the fold.
pub trait DomainEvent: Sized {
    /// Every event type a value of this type can carry.
    ///
    /// `EventType::from_static` is `const`, so the list is built and validated
    /// by the compiler and needs no derive macro to exist.
    const EVENT_TYPES: &'static [EventType];

    /// The event type this value carries.
    ///
    /// Returned **by value**, which costs a borrowed-`Cow` clone and no
    /// allocation. `-> &'static EventType` lost: `EventType` holds a
    /// `Cow<'static, str>`, so const promotion does not apply and the
    /// implementation would have to index `EVENT_TYPES` by position — a second
    /// place to get the mapping wrong, which is the whole hazard this trait
    /// exists to remove.
    fn event_type(&self) -> EventType;

    /// The tags this event carries.
    fn tags(&self) -> Tags;

    /// Encodes this event's payload with `codec`.
    ///
    /// # Errors
    ///
    /// Returns the codec's own failure when the value cannot be serialised.
    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError>;

    /// Decodes a payload written for `event_type`.
    ///
    /// # Errors
    ///
    /// Returns [`CodecError::UnknownEventType`] when `event_type` is not one
    /// this type declares, and the codec's own failure when `data` is not a
    /// valid payload for it.
    fn decode<C: Codec>(
        codec: &C,
        event_type: &EventType,
        data: &Bytes,
    ) -> Result<Self, CodecError>;
}

/// Read events folded into state a decision can be taken on.
///
/// The fold is a `match` over the implementor's own enum, so adding a variant
/// is a compile error in the file that owns the domain rather than a silent
/// gap. The model holds its own validated [`Tags`]: `Tags::from_pairs` is
/// fallible and is the only way in, so the validation is paid once, in the
/// constructor the caller already writes, and never again on a read.
///
/// # `Clone`, not `Default`
///
/// A `Default` supertrait would force the validated scope into a
/// default-constructible field, re-opening the invalid-value hole
/// `Tags::from_pairs` exists to close. `Clone` is what a retrying command loop
/// needs instead: it re-folds from the pristine model rather than from a
/// mutated one.
///
/// # Example
///
/// The companion to [`DomainEvent`]'s `compile_fail`: the same shape with a
/// declaration that is not empty.
///
/// ```
/// use happenstance::{Boundary, Codec, CodecError, DecisionModel};
/// use happenstance::{DomainEvent, EventType, Tags, bytes::Bytes};
///
/// #[derive(serde::Serialize, serde::Deserialize)]
/// enum Seat { Taken, Freed }
///
/// impl DomainEvent for Seat {
///     const EVENT_TYPES: &'static [EventType] = &[
///         EventType::from_static("SeatTaken"),
///         EventType::from_static("SeatFreed"),
///     ];
///     fn event_type(&self) -> EventType {
///         match self {
///             Self::Taken => Self::EVENT_TYPES[0].clone(),
///             Self::Freed => Self::EVENT_TYPES[1].clone(),
///         }
///     }
///     fn tags(&self) -> Tags { Tags::empty() }
///     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
///         c.encode(self)
///     }
///     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
///         -> Result<Self, CodecError> { c.decode(d) }
/// }
///
/// #[derive(Clone)]
/// struct Seats { scope: Tags, taken: u32 }
///
/// impl DecisionModel for Seats {
///     type Event = Seat;
///     fn scope(&self) -> &Tags { &self.scope }
///     fn apply(&mut self, event: Seat) {
///         match event {
///             Seat::Taken => self.taken += 1,
///             Seat::Freed => self.taken -= 1,
///         }
///     }
/// }
///
/// let scope = Tags::from_pairs([("course", "c1")])?;
/// let seats = Seats { scope, taken: 0 };
/// assert_eq!(seats.query()?.items().map_or(0, <[_]>::len), 1);
/// # Ok::<(), Box<dyn core::error::Error>>(())
/// ```
pub trait DecisionModel: Clone {
    /// The one domain enum this model folds.
    type Event: DomainEvent;

    /// The tags every event inside this boundary carries.
    ///
    /// Already validated: [`Tags`] has no infallible constructor that can
    /// produce an invalid value, so returning a reference to a held value is
    /// what keeps the fallibility in the caller's constructor instead of
    /// inside an infallible signature, where it could only become an `unwrap`.
    fn scope(&self) -> &Tags;

    /// Folds one decoded event into this model's state.
    ///
    /// Takes the event **by value**, so the implementor's `match` is
    /// exhaustive over their own enum and a new variant is a compile error in
    /// their own file.
    fn apply(&mut self, event: Self::Event);
}
