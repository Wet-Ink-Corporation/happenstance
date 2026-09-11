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
//! > **Answers:** `tutorial` — How do I decide, write, and hold an invariant?
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
//! const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");
//! impl DomainEvent for Seat {
//!     const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
//!     fn event_type(&self) -> EventType { SEAT_TAKEN }
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
//! let retry = Retry::attempts(core::num::NonZeroU32::new(3).unwrap());
//! let take =
//!     |_: &Seats| Ok::<_, core::convert::Infallible>(vec![Seat::Taken]);
//! let done = commit(&store, seats, retry, take).await?;
//! assert_eq!(done.committed().expect("a seat was taken").attempts, 1);
//! # Ok::<(), Box<dyn Error>>(()) }
//! ```
//!
//! To watch that boundary *refuse* a write instead, in three runnable steps: [the opening encounter](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/first-encounter.md).
//!
//! # What arrives here, and what stays below
//!
//! The discriminator is **encoding**. [`happenstance_core`] deals in opaque
//! bytes on purpose — that is what keeps adapters free of domain knowledge and
//! lets replication forward events without deserialising them. Anything that
//! knows how a payload is *shaped* belongs here, so the contract crate never
//! grows a `serde` dependency in its default feature set.
//!
//! [ADR-0006](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0006-bare-name-to-the-typed-layer.md)
//! is why the bare name is here: an application programs against typed events
//! and decision models, so the crate it reaches for first should be the one it
//! uses. `happenstance-core` is what *adapter* authors pin, and they are the
//! ones who read version numbers carefully.
//!
//! Everything the contract crate exports is available here under the same
//! paths.
//!
//! # The vocabulary
//!
//! Every term below is an item you can use today. The last one asks for a
//! feature first, and the feature is named after what is unfinished beneath
//! it rather than after what it turns on.
//!
//! * [**`Codec`**](Codec) — payload encoding. `Json` is on by default;
//!   `Postcard` and `Cbor` arrive with the features below. Events carry a codec
//!   tag, so one store can hold more than one encoding at a time, which is what
//!   makes a payload migration possible.
//! * [**`DomainEvent`**](DomainEvent) — a Rust type's mapping to its
//!   [`EventType`] and [`Tags`]. Its page carries the `decode` guard this
//!   first program leaves out.
//! * [**`DecisionModel`**](DecisionModel) — folds read events into decidable
//!   state and produces the matching [`Query`], through [`Boundary`]. Its item
//!   page carries the program above over two variants. Composing several into
//!   one query — put them in a tuple, which is a [`Boundary`] too — is what
//!   makes a dynamic consistency boundary *dynamic*.
//! * [**The command loop**][command-loop] — read, decide, append, retry on
//!   [`ConditionViolated`](happenstance_core::AppendError::ConditionViolated),
//!   bounded by a [`Retry`] you pass in and re-deciding from a pristine model
//!   on every attempt. Its own page carries the policy.
//! * [**The typed projection runner**][projection-runner] — decoded events into
//!   a read model, in chunks, with the rows and the checkpoint moving in one
//!   commit. Behind `unstable-projection` — not for the port beneath it, which
//!   is frozen since ADR-0063, but for [`Projection::apply`] being synchronous:
//!   a projection can push into a buffered batch and cannot issue a statement
//!   into a live one, and whether that shape survives is the runner's own open
//!   axis. One call drives one projection, and it never buffers the replay.
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
//! | `unstable-projection` | the projection runner; `apply` is unproved |
//!
//! # Testing without a database
//!
//! *Does the domain model work* and *pick a database* are two decisions, and
//! only the first is due now. [`happenstance::testing`][testing-module] answers
//! it: `given(model).event(a)?.when(..).await?` folds your boundary over an
//! in-memory store and hands back a `Decision` to assert on — no connection
//! string, no fixture, one `await`.
//!
//! For a store that *misbehaves* on demand — an append refused without naming
//! the conflict, positions that are not dense — add `happenstance-testkit` as a
//! dev-dependency. A test double belongs in a second crate rather than in an
//! application's own dependency graph.
//!
//! Adapter authors should depend on [`happenstance_core`]
//! directly rather than on this crate: it is the smaller semver surface, and
//! it is the one the conformance suite is written against.
//!

// The vocabulary's command-loop reference resolves to whichever door this build
// has. A plain `](commit)` would be an unresolved link — a *hard* rustdoc error,
// not a warning — in every build without `json`, which is the failure the
// design's mock caught before any of this was written.
#![cfg_attr(feature = "json", doc = "[command-loop]: commit")]
#![cfg_attr(not(feature = "json"), doc = "[command-loop]: commit_with")]
// The same trick, for the same reason: the testing module exists only where
// both features that build it are on, and an intra-doc link that resolves in
// only some configurations is a hard rustdoc error rather than a warning. The
// region above stays on the page either way — a region that disappeared with a
// feature would leave a reader who turned one off wondering what they lost.
#![cfg_attr(
    all(feature = "memory", feature = "json"),
    doc = "[testing-module]: testing"
)]
#![cfg_attr(
    not(all(feature = "memory", feature = "json")),
    doc = "[testing-module]: https://docs.rs/happenstance/latest/happenstance/testing/"
)]
// And once more for the runner, whose feature is the only one of the three that
// is **off** by default — so the configuration where the local target does not
// exist is the one a casual `cargo doc` builds. The bullet stays on the page
// either way: a vocabulary that lost an entry when a feature went off would
// leave a reader unable to tell what they turned off.
#![cfg_attr(
    feature = "unstable-projection",
    doc = "[projection-runner]: run_projection"
)]
#![cfg_attr(
    not(feature = "unstable-projection"),
    doc = "[projection-runner]: https://docs.rs/happenstance/latest/happenstance/fn.run_projection.html"
)]
#![doc(html_no_source)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
mod composition;

mod boundary;
mod codec;
mod command;
mod domain;
// `runner`, not `projection`. The contract crate already publishes a
// `projection` module, and this crate re-exports it as
// `happenstance::projection`; a private module of the same name shadows it
// silently, which is a breaking change to a facade whose whole promise is that
// the contract's paths still work here.
#[cfg(feature = "unstable-projection")]
mod runner;
mod sealed;

// The one deliberate exception to root re-export: a **named module**, not a
// handful of `pub use`s. `given`, `Decision` and `assert_domain_event` are
// test-time vocabulary, and mixing them into the root's item table doubles the
// page a reader scans for the four names they actually need.
//
// Two features, not one. The design's item table says `memory` — the store
// `given` builds — and that is incomplete rather than wrong: `Given::event`
// seeds through the codec `commit` writes with, which is `Json`, which is
// `json`'s. Both are in `default`, so the item a `cargo add happenstance` user
// meets is the item the design signed off; what the second gate buys is a
// feature powerset that still compiles.
#[cfg(all(feature = "memory", feature = "json"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "memory", feature = "json"))))]
pub mod testing;

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
pub use command::{CommandError, CommandOutcome, Committed, Retry, commit_with};
pub use domain::{DecisionModel, DomainEvent};
// Off by default, and the badge is what says so on the rendered page: an
// item that only exists behind a feature and does not name it is the reader
// finding out from a compiler error instead of from the documentation.
#[cfg(feature = "unstable-projection")]
#[cfg_attr(docsrs, doc(cfg(feature = "unstable-projection")))]
pub use runner::{Progressed, Projection, ProjectionError, run_projection};

// The contract's surface, mounted here so a reader who installed the facade can
// still type the paths the contract's own documentation uses.
//
// Written out rather than globbed, and the glob is what shipped: `pub use
// happenstance_core::*;` re-exports whatever the *compiled* contract crate
// exposes, and while the contract's projection items were gated on **its**
// `unstable-projection` that meant `happenstance-testkit` — this crate's
// dev-dependency, which enabled the feature unconditionally — resolved the
// whole unfrozen port at `happenstance::` under `cargo test` with this crate's
// own feature off. The port is unconditional now (ADR-0063); the list stays
// written out because the second cost below has not gone anywhere.
//
// The glob cost a second thing that never showed up as a failure: every future
// addition to the contract crate was an addition to this crate's public surface
// with nobody reviewing it, and a name added to both crates was a hard break in
// a crate that had not changed. An explicit list turns that collision into
// `error[E0255]` here, at home, in the commit that causes it.
//
// The gates below are the contract's own, feature for feature.
// `tests/contract_surface.rs` derives them from `happenstance-core`'s crate root
// and fails if the two lists stop agreeing.
pub use happenstance_core::store;
pub use happenstance_core::{AppendCondition, Guard};
pub use happenstance_core::{AppendError, ConditionViolated, InvalidEventType};
pub use happenstance_core::{Event, EventParts, EventType, MAX_EVENT_TYPE_LEN};
pub use happenstance_core::{EventId, RecordedAt, StoreId};
pub use happenstance_core::{EventStore, SendEventStore, collect, read_decision_model};
pub use happenstance_core::{InvalidQuery, InvalidTag};
pub use happenstance_core::{MAX_TAG_LEN, Tag, Tags};
pub use happenstance_core::{
    MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_QUERY_ITEMS,
    MIN_SUPPORTED_TAGS_PER_EVENT, StoreLimit,
};
pub use happenstance_core::{Query, QueryItem, ReadOptions};
pub use happenstance_core::{SequencePosition, SequencedEvent};

// The two crates the contract re-exports so that a caller names one `Bytes` and
// one `Stream` rather than two that print identically (RS-40-4). Passed through
// for the same reason they are re-exported there: a facade that stopped at the
// contract's *types* would send a reader back to their own manifest to add a
// `bytes` line, and the version they picked is the one the compiler complains
// about.
pub use happenstance_core::{bytes, futures_core};

#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use happenstance_core::{MemoryEventStore, MemoryStoreError};

// The port, frozen since ADR-0063 and mounted unconditionally like the rest of
// the contract. The module is on the list because a *module* is a name too, and
// `src/tests.rs`'s `same_projection_id` is the witness that
// `happenstance::projection` still resolves to the contract's. What stays
// behind this crate's `unstable-projection` is the *runner* above, not the
// port it runs over.
pub use happenstance_core::projection;
pub use happenstance_core::{Authority, Checkpoint, CommitError, ProjectionId};
pub use happenstance_core::{ProjectionStore, ResetError, SendProjectionStore};

#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use happenstance_core::{
    MemoryProjectionBatch, MemoryProjectionStore, MemoryProjectionStoreError,
};
// Deliberately absent: `ProjectionProbe`. The contract gates it on
// `conformance`, a feature this crate does not forward and should not — it is a
// test double, and a test double belongs in `happenstance-testkit` rather than
// in an application's own dependency graph. It reached `happenstance::` through
// the glob anyway, which is the plainest statement of what the glob was doing:
// mounting a name no consumer of this crate could turn on *or* off.

/// The contract crate under its own name, beside the list above.
///
/// The list re-exports every *item*; it does not re-export the **crate**, so
/// `happenstance_core::EventStore` — the spelling in the contract's own
/// documentation, in every adapter's, and in every diagnostic — does not
/// resolve through this facade without it. It adds a path, not a type: the
/// list already guarantees there is one contract crate here, and this makes
/// it nameable.
pub use happenstance_core;

/// Compiled proof that the contract crate is nameable here.
///
/// Reachable under its own name, that is, and not only through the list
/// above.
///
/// The list puts every *item* in this crate's root; it does not put the
/// **crate** there. A signature copied out of `happenstance-core`'s own
/// documentation, or out of an adapter's, is written
/// `happenstance_core::EventStore` — and a reader who installed `happenstance`
/// rather than the contract should not have to rewrite it to compile it.
///
/// ```
/// fn bound<S: happenstance::happenstance_core::EventStore>(_s: S) {}
/// # fn main() {}
/// ```
///
/// ```
/// use happenstance::happenstance_core::bytes::Bytes;
/// fn payload(b: Bytes) -> usize { b.len() }
/// # fn main() {
/// #     assert_eq!(payload(Bytes::from_static(b"{}")), 2);
/// # }
/// ```
#[cfg(doctest)]
mod reexported_paths {}
