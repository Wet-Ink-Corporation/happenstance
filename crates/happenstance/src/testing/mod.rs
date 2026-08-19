//! Given, when, then — a decision model tested without a database.
//!
//! Beat 2 of the first hour is *does the domain model work*, and it is a
//! different question from *pick a database*. This module is where the two come
//! apart: [`given`] builds its own in-memory store, seeds it through the same
//! codec `commit` writes with, folds the boundary through the query the
//! boundary itself derives, and hands back a [`Decision`] to assert on.
//!
//! ```
//! use happenstance::testing::given;
//! use happenstance::{Codec, CodecError, DecisionModel, DomainEvent};
//! use happenstance::{EventType, Tags, bytes::Bytes};
//! # #[tokio::main] async fn main()
//! # -> Result<(), Box<dyn std::error::Error>> {
//!
//! #[derive(Debug, PartialEq)]
//! #[derive(serde::Serialize, serde::Deserialize)]
//! enum Seat { Taken }
//!
//! impl DomainEvent for Seat {
//!     const EVENT_TYPES: &'static [EventType] =
//!         &[EventType::from_static("SeatTaken")];
//!     fn event_type(&self) -> EventType { Self::EVENT_TYPES[0].clone() }
//!     fn tags(&self) -> Tags { Tags::empty() }
//!     fn encode<C: Codec>(&self, c: &C) -> Result<Bytes, CodecError> {
//!         c.encode(self)
//!     }
//!     fn decode<C: Codec>(c: &C, _t: &EventType, d: &Bytes)
//!         -> Result<Self, CodecError> { c.decode(d) }
//! }
//!
//! #[derive(Clone)]
//! struct Seats { scope: Tags, taken: u32 }
//!
//! impl DecisionModel for Seats {
//!     type Event = Seat;
//!     fn scope(&self) -> &Tags { &self.scope }
//!     fn apply(&mut self, _event: Seat) { self.taken += 1 }
//! }
//!
//! let seats = Seats { scope: Tags::empty(), taken: 0 };
//! given(seats)
//!     .event(Seat::Taken)?
//!     .when(|held: &Seats| {
//!         assert_eq!(held.taken, 1);
//!         Ok::<_, core::convert::Infallible>(vec![Seat::Taken])
//!     })
//!     .await?
//!     .then(&[Seat::Taken]);
//! # Ok(()) }
//! ```
//!
//! # What a failed `then` renders, and why the last region is the point
//!
//! Four labelled regions, in this order: `expected:`, `actual:`,
//! `selected by the model's query:`, `seeded but NOT selected:`. The fourth is
//! why this module exists. When a domain type's `EVENT_TYPES` and the fold that
//! reads them disagree, the events that were seeded and **not** selected are the
//! entire diagnosis, and an `assert_eq!`-shaped message that showed only
//! expected and actual would hide exactly the filter that caused the failure.
//!
//! # There is one filter vocabulary, and it is the boundary's own query
//!
//! Nothing here accepts an event-type list, a predicate or a tag filter.
//! *Selected* is defined as "what [`Boundary::query`]
//! selected", which is what makes the fourth region a set difference rather than
//! a guess — and what stops a test from selecting a different set than
//! production does.

mod render;

use std::sync::Arc;

use happenstance_core::{
    Event, EventStore, MemoryEventStore, MemoryStoreError, Query, ReadOptions, SequencePosition,
    SequencedEvent, collect, read_decision_model,
};

use crate::boundary::Boundary;
use crate::codec::{CodecError, Json};
use crate::command::CommandError;
use crate::domain::DomainEvent;

use render::Row;

/// Starts a test over `boundary`, against a store this call creates.
///
/// One `given` is one isolated store, which is the rule every fixture in
/// `happenstance-testkit` follows and the reason there is no constructor here
/// taking a store of your own: accepting one would need a capability model, and
/// that is the testkit's job rather than this module's.
///
/// `boundary` is a single [`DecisionModel`](crate::DecisionModel) or a tuple of
/// them — a tuple is a [`Boundary`] too, so composition costs
/// no new syntax here either.
///
/// # Examples
///
/// See the [module documentation](self) for the whole shape.
pub fn given<B: Boundary>(boundary: B) -> Given<B> {
    Given {
        boundary,
        store: Arc::new(MemoryEventStore::new()),
        pending: Vec::new(),
    }
}

/// A store being seeded, and the boundary that will read it.
///
/// Consuming (`self` → `Self`) so a chain of `?` reads naturally and a
/// half-built value is unusable after an encode failure.
#[derive(Debug)]
#[must_use = "a Given seeds nothing until `when` runs the decision"]
pub struct Given<B: Boundary> {
    boundary: B,
    store: Arc<MemoryEventStore>,
    pending: Vec<Event>,
}

impl<B: Boundary> Given<B> {
    /// Seeds one event, encoded exactly as `commit` would have written it.
    ///
    /// The same codec, the same framing region, the same tags — so a test is
    /// evidence about the application rather than about the DSL. It takes any
    /// [`DomainEvent`], not only the boundary's own: seeding a type the
    /// boundary does **not** nominate is how the fourth region of a failure
    /// message gets something to say.
    ///
    /// The write itself is deferred to [`when`](Self::when), which is the only
    /// `async` step a test author writes. Nothing observable turns on that:
    /// encoding is what can fail here, and encoding is what this signature is
    /// fallible for.
    ///
    /// # Errors
    ///
    /// Returns the codec's own refusal when the value cannot be serialised.
    /// Nothing is seeded when it does, and the builder is consumed, so a
    /// partially seeded store is not reachable.
    // By value, which the signed-off signature fixes and which is the right
    // shape for a caller: a seeded event is handed over, and `&E` would make
    // every line of every test read `.event(&Seat::Taken)?`. The body only
    // borrows it because `DomainEvent`'s three accessors take `&self`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn event<E: DomainEvent>(mut self, event: E) -> Result<Self, CodecError> {
        let event_type = event.event_type();
        let data = event.encode(&Json)?;

        // Unreachable for a well-formed `DomainEvent`: `event_type()` hands back
        // an already-validated `EventType` and the conversion is the infallible
        // identity. Absorbed rather than unwrapped, exactly as the command loop
        // absorbs the same call.
        let built = Event::new(event_type.clone(), data)
            .map_err(|_| CodecError::UnknownEventType { event_type })?;

        self.pending.push(
            built
                .with_tags(event.tags())
                .with_metadata(crate::codec::frame::<Json>(None)),
        );
        Ok(self)
    }

    /// Reads, folds and decides — the one `await` a test author writes.
    ///
    /// The read is `boundary.query()` and nothing else, so *selected* means what
    /// the model's own query selected. A refusal from `decide` is captured
    /// **into** the returned [`Decision`], where
    /// [`then_refused`](Decision::then_refused) asserts it; this `Err` arm is
    /// store and codec failure only.
    ///
    /// # Errors
    ///
    /// * [`CommandError::Boundary`] if the boundary constrains nothing.
    /// * [`CommandError::Read`] if the store fails during the read — which it
    ///   cannot here, because [`MemoryStoreError`] is uninhabited.
    /// * [`CommandError::Append`] if seeding is refused.
    /// * [`CommandError::Decode`] if an event the query **nominated** cannot be
    ///   decoded. That is a real disagreement between a domain type's
    ///   `EVENT_TYPES` and the fold that reads them, and it is not swallowed.
    pub async fn when<D, F>(
        self,
        decide: F,
    ) -> Result<Decision<B::Event>, CommandError<MemoryStoreError, D>>
    where
        D: core::error::Error + 'static,
        F: FnOnce(&B) -> Result<Vec<B::Event>, D>,
    {
        let Self {
            mut boundary,
            store,
            pending,
        } = self;

        if !pending.is_empty() {
            store
                .append(&pending, None)
                .await
                .map_err(CommandError::Append)?;
        }

        // Everything seeded, in the order the store assigned. This is the left
        // operand of the set difference the fourth region reports; the right one
        // is the boundary's own query, below.
        let seeded = collect(store.read(&Query::all(), ReadOptions::new()))
            .await
            .map_err(CommandError::Read)?;

        let query = boundary.query()?;
        let (selected, _) = read_decision_model(&*store, &query)
            .await
            .map_err(CommandError::Read)?;

        for event in &selected {
            boundary
                .absorb(event, &Json)
                .map_err(|source| CommandError::Decode {
                    position: event.position,
                    source,
                })?;
        }

        let outcome = match decide(&boundary) {
            Ok(emitted) => Outcome::Emitted(emitted),
            Err(refusal) => Outcome::Refused(Box::new(refusal)),
        };

        let picked: Vec<SequencePosition> = selected.iter().map(|event| event.position).collect();

        Ok(Decision {
            outcome,
            selected: selected.iter().map(row).collect(),
            unselected: seeded
                .iter()
                .filter(|event| !picked.contains(&event.position))
                .map(row)
                .collect(),
            query,
        })
    }
}

/// What a decision did: emitted events, or refused.
#[derive(Debug)]
enum Outcome<E> {
    Emitted(Vec<E>),
    /// The caller's own error, boxed but **not** stringified: it is still a
    /// typed value and `downcast_ref` still finds it.
    Refused(Box<dyn core::error::Error + 'static>),
}

/// The verbatim `#[must_use]` text [`Decision`] carries, kept beside it.
///
/// An attribute takes a literal and cannot read a `const`, so the two are
/// written twice and `must_use_message_is_verbatim` compares them by reading
/// this file — which is what stops the pair drifting in silence.
#[cfg(test)]
pub(crate) const DECISION_MUST_USE: &str = "a Decision is not appended until it is committed; \
     dropping it discards the events the decision produced";

/// What a decision produced, and the filter that produced it.
///
/// Nothing here has been appended. Everything up to the append is pure, which
/// is what the `#[must_use]` below says out loud: a `Decision` you drop discards
/// the events the decision produced and changes nothing.
#[derive(Debug)]
#[must_use = "a Decision is not appended until it is committed; \
     dropping it discards the events the decision produced"]
#[non_exhaustive]
pub struct Decision<E> {
    outcome: Outcome<E>,
    selected: Vec<Row>,
    unselected: Vec<Row>,
    query: Query,
}

impl<E: DomainEvent + PartialEq + core::fmt::Debug> Decision<E> {
    /// Asserts the decision emitted exactly `expected`.
    ///
    /// # Panics
    ///
    /// Panics when the emitted events differ, rendering four labelled regions —
    /// `expected:`, `actual:`, `selected by the model's query:` and
    /// `seeded but NOT selected:` — followed by one line naming the derived
    /// query. It panics naming the refusal when the decision **refused**:
    /// comparing a refusal against `&[]` and passing is the silent pass this
    /// method exists to prevent.
    ///
    /// `#[track_caller]` puts the panic's location on your own assertion line
    /// rather than inside this module, which is why the `panic!` is written here
    /// and not in the renderer — the attribute does not reach through a helper's
    /// own frame.
    #[track_caller]
    pub fn then(self, expected: &[E]) {
        match &self.outcome {
            Outcome::Emitted(actual) if actual.as_slice() == expected => {}
            Outcome::Emitted(actual) => {
                let message = render::events_differ(
                    &rendered(expected),
                    &rendered(actual),
                    &self.selected,
                    &self.unselected,
                    &self.query,
                );
                panic!("{message}");
            }
            Outcome::Refused(refusal) => panic!(
                "assertion failed: the decision refused rather than emitting \
                 events\nrefusal: {refusal}\nassert a refusal with \
                 `then_refused()`"
            ),
        }
    }

    /// Asserts the decision refused.
    ///
    /// The refusal is your own error type, carried as a typed value the whole
    /// way rather than flattened into a string.
    ///
    /// # Panics
    ///
    /// Panics when the decision emitted events instead, naming them.
    #[track_caller]
    pub fn then_refused(self) {
        if let Outcome::Emitted(actual) = &self.outcome {
            panic!(
                "assertion failed: the decision emitted {} event(s) rather \
                 than refusing\nactual: {actual:?}",
                actual.len()
            );
        }
    }
}

/// Asserts every value's `event_type()` is one its type declares.
///
/// Hand this every variant of your domain enum. It exists because that
/// agreement is **not** compiler-enforceable: a variant may return a type absent
/// from `EVENT_TYPES` and no `const` sees the match arms. The alternative that
/// lost was pretending a hand-written impl can enforce it — it cannot, and a
/// test that runs is worth more than a claim that does not.
///
/// # Panics
///
/// Panics naming the offending value's event type, and the declaration it is
/// missing from.
///
/// # Examples
///
/// ```
/// use happenstance::testing::assert_domain_event;
/// use happenstance::{Codec, CodecError, DomainEvent, EventType, Tags};
/// use happenstance::bytes::Bytes;
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
/// assert_domain_event(&[Seat::Taken, Seat::Freed]);
/// ```
pub fn assert_domain_event<E: DomainEvent>(every_variant: &[E]) {
    for value in every_variant {
        let carried = value.event_type();
        assert!(
            E::EVENT_TYPES.contains(&carried),
            "`event_type()` returned `{}`, which EVENT_TYPES does not declare: \
             [{}]. The declaration and the value have drifted, and no compiler \
             can see it — this is the check that can",
            carried.as_str(),
            E::EVENT_TYPES
                .iter()
                .map(happenstance_core::EventType::as_str)
                .collect::<Vec<_>>()
                .join(", "),
        );
    }
}

/// One read event, as the failure message renders it.
fn row(event: &SequencedEvent) -> Row {
    Row {
        position: event.position,
        event_type: event.event.event_type().clone(),
    }
}

/// `Debug`-renders each value, once, where the renderer cannot.
fn rendered<E: core::fmt::Debug>(events: &[E]) -> Vec<String> {
    events.iter().map(|event| format!("{event:?}")).collect()
}

#[cfg(test)]
mod tests;
