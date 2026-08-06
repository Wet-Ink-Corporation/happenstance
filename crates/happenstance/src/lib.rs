//! DCB-compliant event sourcing, with batteries.
//!
//! # Status: a facade over [`happenstance_core`]
//!
//! Today this crate re-exports the contract crate and adds nothing. It is
//! published from day one anyway, so that `cargo add happenstance` is true
//! throughout — including before the typed layer exists — and so that the name
//! never has to move once people depend on it.
//!
//! [ADR-0006](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/adr/0006-bare-name-to-the-typed-layer.md)
//! is why the bare name is here rather than on the contract: an application
//! programs against typed events and decision models, and the crate an
//! application reaches for first should be the one it uses. `happenstance-core`
//! is what *adapter* authors pin, and adapter authors are the ones who read
//! version numbers carefully.
//!
//! # What arrives here, and what stays below
//!
//! The discriminator is **encoding**. [`happenstance_core`] deals in opaque
//! bytes on purpose — that is what keeps adapters free of domain knowledge and
//! lets replication forward events without deserialising them. Anything that
//! knows how a payload is *shaped* belongs here, so that the contract crate
//! never grows a `serde` dependency in its default feature set.
//!
//! Planned, and specified in
//! [`docs/architecture/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/architecture/SPECIFICATION.md):
//!
//! * **`Codec`** — payload encoding. Events carry a codec tag so one store can
//!   hold more than one encoding at a time, which is what makes a migration
//!   possible.
//! * **`DomainEvent`** — a Rust type's mapping to its
//!   [`EventType`](happenstance_core::EventType) and
//!   [`Tags`](happenstance_core::Tags).
//! * **`DecisionModel`** — folds read events into decidable state and produces
//!   the matching [`Query`](happenstance_core::Query). Composing several into
//!   one query is the mechanism that makes a dynamic consistency boundary
//!   *dynamic*.
//! * **The command loop** — read, decide, append, retry on
//!   [`ConditionViolated`](happenstance_core::AppendError::ConditionViolated).
//! * **The typed projection runner** — decoded events, over the checkpoint pump
//!   that stays in the contract crate
//!   ([ADR-0007](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/adr/0007-projection-runner-decodes.md)).
//!
//! # Using it today
//!
//! Everything the contract crate exports is available here under the same
//! paths, so nothing has to be rewritten when the typed layer lands:
//!
//! ```
//! use happenstance::{EventStore, MemoryEventStore, Query, ReadOptions, collect};
//!
//! # async fn example() -> Result<(), Box<dyn core::error::Error>> {
//! let store = MemoryEventStore::new();
//! let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
//! assert!(events.is_empty());
//! # Ok(())
//! # }
//! ```
//!
//! Adapter authors should depend on [`happenstance_core`] directly rather than
//! on this crate: it is the smaller semver surface, and it is the one the
//! conformance suite is written against.

#![doc(html_no_source)]

pub use happenstance_core::*;
