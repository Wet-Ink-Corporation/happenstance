// The README's code blocks are compiled as doctests. `cfg(doctest)` keeps the
// prose out of the rendered documentation — it would otherwise appear twice, once
// here and once in the module docs below — while still type-checking every
// example. A README example that does not compile is worse than no example: it
// is the first thing a reader tries, and the first impression the crate makes.
// (D10)
#![cfg_attr(doctest, doc = include_str!("../README.md"))]
//! The happenstance contract: DCB-compliant event sourcing types and storage ports.
//!
//! This crate defines *what* an event store is and nothing about *how* one is
//! built. It holds no I/O, opens no connections, and takes no opinion on
//! serialisation. Everything else in the happenstance ecosystem — SQLite, Ladybug,
//! replication — depends on it, so it is deliberately the smallest and most
//! stable crate in the workspace.
//!
//! It implements the [Dynamic Consistency Boundary specification][spec].
//!
//! [spec]: https://dcb.events/specification/
//!
//! # DCB in a paragraph
//!
//! Classical event sourcing draws consistency boundaries ahead of time, as
//! aggregates, and every decision must fit inside one. DCB draws them per
//! decision instead. A command handler reads whatever events it needs — across
//! any number of entities — and then appends conditioned on *nothing matching
//! that same query having appeared since*. The boundary is whatever the handler
//! actually looked at, so invariants that span entities stop requiring either an
//! oversized aggregate or a saga.
//!
//! # The shape of the API
//!
//! | Concern | Type |
//! |---|---|
//! | What happened | [`Event`], [`EventType`], [`Tag`], [`Tags`] |
//! | Where it sits in the log | [`SequencePosition`], [`SequencedEvent`] |
//! | What to read | [`Query`], [`QueryItem`], [`ReadOptions`] |
//! | What must not have changed | [`AppendCondition`] |
//! | Storage seams | [`EventStore`], [`ProjectionStore`] |
//!
//! # Design notes
//!
//! **Illegal states are unrepresentable.** [`Query`] is an enum rather than a
//! vector because the specification permits "at least one item" or "match
//! everything" and nothing else. [`SequencePosition`] wraps a [`NonZeroU64`],
//! so position zero cannot exist and `Option<SequencePosition>` costs no extra
//! space. [`Tags`] is canonically sorted at construction, so it can never be
//! observed out of order.
//!
//! [`NonZeroU64`]: core::num::NonZeroU64
//!
//! **Payloads are opaque.** [`Event::data`] is [`Bytes`](bytes::Bytes) with no
//! `serde` bound in sight. Adapters stay free of domain knowledge, and
//! replication can forward an event byte-for-byte without deserialising it.
//! Encoding is a job for the layer above.
//!
//! **The concurrency signal is in the type system.** An append returns
//! [`AppendError`], which separates [`ConditionViolated`] — routine under
//! contention, and a cue to retry — from adapter-specific failures. No caller
//! has to match on a string to tell them apart.
//!
//! **There are two flavours of each port.** [`EventStore`] carries no `Send`
//! bound so it can be implemented on `wasm32`; [`SendEventStore`] is derived
//! from it for native use. Implementing the latter gives you the former. See
//! the [`store`] module documentation for the full rationale.
//!
//! # Getting started
//!
//! Enable the `memory` feature (on by default) and use `MemoryEventStore` — see
//! its documentation for a runnable walkthrough of the read-decide-append loop.
//!
//! The name is deliberately not a link here. It would be a broken one whenever
//! the feature is off, and `cargo doc --no-default-features` treats a broken
//! intra-doc link as a hard error rather than a warning (D13).
//!
//! # Feature flags
//!
//! * **`std`** *(default)* — standard library support.
//! * **`memory`** *(default)* — the `MemoryEventStore` reference
//!   implementation. Implies `std`.
//! * **`serde`** — `Serialize`/`Deserialize` for the wire types. Off by
//!   default so the contract crate carries no serialisation opinion; enabled by
//!   replication adapters that need one.
//! * **`conformance`** — `ProjectionProbe`, the write seam the projection
//!   conformance suite drives an adapter's read model through. For adapter
//!   authors running that suite against their own store; nothing in the runtime
//!   path needs it. Off by default, pulls in no dependency, and implies
//!   nothing — not `std`, not `memory`.
//!
//!   The name is deliberately not a link here, for the reason given under
//!   *Getting started*: a link into a `cfg`-gated item is a hard error when the
//!   feature is off, and `cargo doc --no-default-features` is a gate step.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod append;
mod error;
mod event;
mod identity;
mod limits;
mod query;
mod tag;
mod validate;

pub mod projection;
pub mod store;

#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
mod memory;

pub use append::{AppendCondition, Guard};
pub use error::{AppendError, ConditionViolated, InvalidEventType, InvalidQuery, InvalidTag};
pub use event::{
    Event, EventParts, EventType, MAX_EVENT_TYPE_LEN, SequencePosition, SequencedEvent,
};
pub use identity::{EventId, RecordedAt, StoreId};
pub use limits::{
    MIN_SUPPORTED_EVENT_DATA_LEN, MIN_SUPPORTED_EVENTS_PER_BATCH, MIN_SUPPORTED_QUERY_ITEMS,
    MIN_SUPPORTED_TAGS_PER_EVENT, StoreLimit,
};
pub use projection::{
    Authority, Checkpoint, CommitError, ProjectionId, ProjectionStore, ResetError,
    SendProjectionStore,
};
pub use query::{Query, QueryItem, ReadOptions};
pub use store::{EventStore, SendEventStore, collect, read_decision_model};
pub use tag::{MAX_TAG_LEN, Tag, Tags};

#[cfg(feature = "conformance")]
#[cfg_attr(docsrs, doc(cfg(feature = "conformance")))]
pub use projection::ProjectionProbe;

#[cfg(feature = "memory")]
#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]
pub use memory::{MemoryEventStore, MemoryStoreError};

/// Re-exported so adapters and callers can name payload types without adding a
/// direct dependency on a specific `bytes` version.
pub use bytes;
