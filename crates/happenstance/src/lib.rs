// The README's code blocks are compiled as doctests. `cfg(doctest)` keeps the
// prose out of the rendered documentation — it would otherwise appear twice, once
// here and once in the module docs below — while still type-checking every
// example. A README example that does not compile is worse than no example: it is
// the first thing a reader tries, and the first impression the crate makes. (D10)
//
// Only this crate's own README. The repository README lives outside the package
// and `include_str!` would not resolve once published, so it is compiled by
// `xtask` instead, which is never published.
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//! DCB-compliant event sourcing, with batteries.
//!
//! One enum of events, one struct that folds them, and a query derived from
//! the same declaration the fold is exhaustive over:
//!
//! ```
//! use happenstance::{
//!     Boundary, Codec, CodecError, DecisionModel, DomainEvent, EventType,
//!     Tags, bytes::Bytes,
//! };
//!
//! #[derive(serde::Serialize, serde::Deserialize)]
//! enum Seat { Taken, Freed }
//!
//! impl DomainEvent for Seat {
//!     const EVENT_TYPES: &'static [EventType] = &[
//!         EventType::from_static("SeatTaken"),
//!         EventType::from_static("SeatFreed"),
//!     ];
//!     fn event_type(&self) -> EventType {
//!         match self {
//!             Self::Taken => Self::EVENT_TYPES[0].clone(),
//!             Self::Freed => Self::EVENT_TYPES[1].clone(),
//!         }
//!     }
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
//!     fn apply(&mut self, event: Seat) {
//!         match event {
//!             Seat::Taken => self.taken += 1,
//!             Seat::Freed => self.taken -= 1,
//!         }
//!     }
//! }
//!
//! let scope = Tags::from_pairs([("course", "c1")])?;
//! let seats = Seats { scope, taken: 0 };
//! assert_eq!(seats.query()?.items().map_or(0, <[_]>::len), 1);
//! # Ok::<(), Box<dyn core::error::Error>>(())
//! ```
//!
//! # What arrives here, and what stays below
//!
//! The discriminator is **encoding**. [`happenstance_core`] deals in opaque
//! bytes on purpose — that is what keeps adapters free of domain knowledge and
//! lets replication forward events without deserialising them. Anything that
//! knows how a payload is *shaped* belongs here, so that the contract crate
//! never grows a `serde` dependency in its default feature set.
//!
//! [ADR-0006](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0006-bare-name-to-the-typed-layer.md)
//! is why the bare name is here rather than on the contract: an application
//! programs against typed events and decision models, and the crate an
//! application reaches for first should be the one it uses. `happenstance-core`
//! is what *adapter* authors pin, and adapter authors are the ones who read
//! version numbers carefully.
//!
//! Everything the contract crate exports is available here under the same
//! paths, so nothing above has to be rewritten when the rest of the typed
//! layer lands.
//!
//! # The vocabulary
//!
//! A linked term is an item you can use today. The three still marked
//! *Planned, and specified in
//! [`spec/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)*
//! say so, and say it about themselves rather than about the page.
//!
//! * **`Codec`** *(planned)* — payload encoding. Events carry a codec tag so
//!   one store can hold more than one encoding at a time, which is what makes
//!   a migration possible.
//! * [**`DomainEvent`**](DomainEvent) — a Rust type's mapping to its
//!   [`EventType`] and [`Tags`].
//! * [**`DecisionModel`**](DecisionModel) — folds read events into decidable
//!   state and produces the matching [`Query`], through [`Boundary`].
//!   Composing several into one query — put them in a tuple, which is a
//!   [`Boundary`] too — is the mechanism that makes a dynamic consistency
//!   boundary *dynamic*.
//! * **The command loop** *(planned)* — read, decide, append, retry on
//!   [`ConditionViolated`](happenstance_core::AppendError::ConditionViolated).
//! * **The typed projection runner** *(planned)* — decoded events, over the
//!   checkpoint pump that stays in the contract crate
//!   ([ADR-0007](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0007-projection-runner-decodes.md)).
//!
//! Adapter authors should depend on [`happenstance_core`] directly rather than
//! on this crate: it is the smaller semver surface, and it is the one the
//! conformance suite is written against.

#![doc(html_no_source)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod composition;

mod boundary;
mod codec;
mod domain;
mod sealed;

#[cfg(test)]
mod tests;

pub use boundary::Boundary;
pub use codec::{Codec, CodecError};
pub use domain::{DecisionModel, DomainEvent};

pub use happenstance_core::*;
