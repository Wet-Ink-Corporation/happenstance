//! The typed layer that sits above the [`happenstance`] contract.
//!
//! # Status: not implemented
//!
//! This crate is a **named seam**, not working code. It exists so that the
//! boundary is visible in the workspace and so that `happenstance` never
//! quietly accretes the responsibilities listed below. It is `publish = false`
//! until it does something.
//!
//! Even the crate name is provisional.
//!
//! # What belongs here
//!
//! `happenstance` deals in opaque bytes on purpose — that is what keeps
//! adapters free of domain knowledge and lets replication forward events
//! without deserialising them. But applications do not want bytes, so
//! *something* has to map between the two. That something is this crate, and
//! keeping it separate is what stops the contract crate from growing a `serde`
//! dependency in its default feature set.
//!
//! Planned contents:
//!
//! * **`Codec`** — payload encoding, with JSON, CBOR and postcard
//!   implementations. Events carry a codec tag so a store can hold more than
//!   one encoding at a time, which matters during a migration.
//! * **`DomainEvent`** — maps a Rust type to its
//!   [`EventType`](happenstance::EventType) and its
//!   [`Tags`](happenstance::Tags). A derive macro in a future
//!   `happenstance-macros` crate would generate the boilerplate; that crate is
//!   deliberately absent until there is something for it to derive.
//! * **`DecisionModel`** — folds a projection of read events into the state a
//!   command handler decides on, and produces the matching
//!   [`Query`](happenstance::Query). Composing several of these into one query
//!   is the mechanism that makes a dynamic consistency boundary *dynamic*.
//! * **Command handling** — the read-decide-append loop with retry on
//!   [`AppendError::ConditionViolated`](happenstance::AppendError::ConditionViolated).
//! * **Projection runner** — pumps events from an
//!   [`EventStore`](happenstance::EventStore) into a
//!   [`ProjectionStore`](happenstance::ProjectionStore), honouring the
//!   checkpoint invariant described in that trait's module documentation.
//!
//! [`happenstance`]: happenstance

#![doc(html_no_source)]

/// Placeholder so the crate has a public item and its lint configuration is
/// exercised by CI.
///
/// Replaced by the real API in the pass that implements this crate.
pub const STATUS: &str = "unimplemented: see crate documentation";
