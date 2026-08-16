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
// The first program below calls `commit`, which is `#[cfg(feature = "json")]`,
// so that fence compiles only where the default features are on. Deliberate, and
// recorded here so it is not read as an oversight: it is the JSON-default first
// program the design signed off, and the two feature-state checks in the gate are
// `cargo check` and `cargo hack check`, neither of which builds a doctest. If a
// gate step ever compiles doctests with `json` off, the fix is a second fence
// behind a `#[cfg(not(feature = "json"))]` doc line calling `commit_with`, the
// way the `[command-loop]` reference below is already written.
//! DCB-compliant event sourcing, with batteries.
//!
//! One enum of events, one struct that folds them, and one call that reads,
//! decides, appends and retries:
//!
//! ```
//! use happenstance::{bytes::Bytes, Codec, CodecError, DecisionModel};
//! use happenstance::{DomainEvent, EventType, MemoryEventStore, Retry};
//! use happenstance::{Tags, commit};
//! use std::error::Error;
//! # #[tokio::main] async fn main() -> Result<(), Box<dyn Error>> {
//!
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
//! #[derive(Clone)]
//! struct Seats { scope: Tags, taken: u32 }
//! impl DecisionModel for Seats {
//!     type Event = Seat;
//!     fn scope(&self) -> &Tags { &self.scope }
//!     fn apply(&mut self, event: Seat) {
//!         match event { Seat::Taken => self.taken += 1 }
//!     }
//! }
//! let store = MemoryEventStore::new();
//! let seats = Seats { scope: Tags::empty(), taken: 0 };
//! let retry = Retry::attempts(3.try_into()?);
//! let take =
//!     |_: &Seats| Ok::<_, core::convert::Infallible>(vec![Seat::Taken]);
//! let done = commit(&store, seats, retry, take).await?;
//! assert_eq!(done.attempts, 1);
//! # Ok::<(), Box<dyn Error>>(()) }
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
//! A linked term is an item you can use today. The one still marked
//! *(planned), and specified in
//! [`spec/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)*
//! says so, and says it about itself rather than about the page.
//!
//! * [**`Codec`**](Codec) — payload encoding. `Json` is on by default;
//!   `Postcard` and `Cbor` arrive with the features named below. Events carry
//!   a codec tag, so one store can hold more than one encoding at a time,
//!   which is what makes a payload migration possible.
//! * [**`DomainEvent`**](DomainEvent) — a Rust type's mapping to its
//!   [`EventType`] and [`Tags`].
//! * [**`DecisionModel`**](DecisionModel) — folds read events into decidable
//!   state and produces the matching [`Query`], through [`Boundary`]. Its
//!   item page carries the program above over two variants, where the fold
//!   has more than one arm to be exhaustive over.
//!   Composing several into one query — put them in a tuple, which is a
//!   [`Boundary`] too — is the mechanism that makes a dynamic consistency
//!   boundary *dynamic*.
//! * [**The command loop**][command-loop] — read, decide, append, retry on
//!   [`ConditionViolated`](happenstance_core::AppendError::ConditionViolated).
//!   Bounded by a [`Retry`] you pass in, re-deciding from a pristine model on
//!   every attempt. Its own page carries the policy.
//! * **The typed projection runner** *(planned)* — decoded events, over the
//!   checkpoint pump that stays in the contract crate
//!   ([ADR-0007](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0007-projection-runner-decodes.md)).
//!
//! # Features
//!
//! Every one of them adds. Turning any off leaves the rest compiling, and the
//! three the contract crate also has mean there exactly what they mean here.
//!
//! | feature | what it turns on |
//! | --- | --- |
//! | `json` *(default)* | `Json` — JSON payloads, readable in a console |
//! | `postcard` | `Postcard` — compact binary, not self-describing |
//! | `cbor` | `Cbor` — binary, and self-describing where postcard is not |
//! | `std` *(default)* | the standard library, forwarded to the contract |
//! | `memory` *(default)* | `MemoryEventStore`, forwarded to the contract |
//! | `serde` | the contract's wire-format derives, for replication |
//!
//! Adapter authors should depend on [`happenstance_core`] directly rather than
//! on this crate: it is the smaller semver surface, and it is the one the
//! conformance suite is written against.
//!

// The vocabulary's command-loop reference resolves to whichever door this build
// has. A plain `](commit)` would be an unresolved link — a *hard* rustdoc error,
// not a warning — in every build without `json`, which is the failure the
// design's mock caught before any of this was written.
#![cfg_attr(feature = "json", doc = "[command-loop]: commit")]
#![cfg_attr(not(feature = "json"), doc = "[command-loop]: commit_with")]
#![doc(html_no_source)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod composition;

mod boundary;
mod codec;
mod command;
mod domain;
mod sealed;

#[cfg(test)]
mod tests;

pub use boundary::Boundary;
#[cfg(feature = "cbor")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbor")))]
pub use codec::Cbor;
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use codec::Json;
#[cfg(feature = "postcard")]
#[cfg_attr(docsrs, doc(cfg(feature = "postcard")))]
pub use codec::Postcard;
pub use codec::{Codec, CodecError};
#[cfg(feature = "json")]
#[cfg_attr(docsrs, doc(cfg(feature = "json")))]
pub use command::commit;
pub use command::{CommandError, Committed, Retry, commit_with};
pub use domain::{DecisionModel, DomainEvent};

pub use happenstance_core::*;
