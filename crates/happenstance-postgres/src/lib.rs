//! Postgres adapters for happenstance: an event store and a projection store.
//!
//! # Status: not implemented
//!
//! The crate compiles and its types are real, but every body that would touch a
//! server is `todo!()`. It is `publish = false` until it passes
//! [`happenstance-testkit`](https://docs.rs/happenstance-testkit)'s conformance
//! suite — which is the bar for any adapter in this workspace, not a formality.
//!
//! # Why this crate exists before it is needed
//!
//! It is an instrument first and a target second. Every adapter in the
//! workspace today — `MemoryEventStore`, a `RefCell` store, rusqlite, a Durable
//! Object — assigns positions under a lock it holds until commit, so all four
//! satisfy the specification's ES-10 (*position order is visibility order*) for
//! free, and none of them votes for it. Postgres is the one shape on the roadmap
//! that can violate ES-10 **by construction**: `nextval()` allocates outside the
//! transaction, so a writer that took position 99 can commit after a writer that
//! took position 100, and a reader that has already seen 100 then watches 99
//! appear beneath it. `AppendCondition::after(100)` evaluates `99 <= 100` and
//! reports no violation, so the consistency boundary silently stops enforcing.
//!
//! Nothing here buys the invariant yet. What this crate does today is make the
//! *shape* that can violate it exist in-tree, so that the port is frozen against
//! a networked, pooled, non-serialising adapter rather than against SQLite
//! wearing four hats.
//!
//! # Open decisions
//!
//! * **How ES-10 is bought.** `xid8` + `pg_snapshot_xmin`, a
//!   transaction-scoped advisory lock, or a serialised sequence table. Each
//!   costs something real, the choice is owed a measurement rather than a
//!   preference, and the deciding question is which of them does *not*
//!   serialise writers — because an adapter that serialises its writers has
//!   stopped being the instrument this crate exists to be. See
//!   [`event_store`] for what each one asks of the types.
//! * **Tag matching.** A join table, a `text[]` column with a GIN index, or
//!   `jsonb`. [`Tags`](happenstance_core::Tags) is canonically sorted so that
//!   the containment operator (`@>`) stays available.
//! * **TLS.** Deliberately not selected, and the reason is measured rather than
//!   assumed. `sqlx`'s `tls-rustls` fails the workspace's `cargo deny` licence
//!   allowlist — not on `ring`, which is `Apache-2.0 AND ISC` and passes, but on
//!   `webpki-roots`, the Mozilla CA root bundle, which is `CDLA-Permissive-2.0`.
//!   A crate that connects to nothing does not need a TLS backend; a crate that
//!   connects to RDS or Neon does, and that is the point at which the allowlist
//!   has to either grow a data licence or the adapter has to take its roots from
//!   the platform trust store.
//!
//! # Not the Neon adapter
//!
//! Neon's serverless driver speaks one-shot HTTP to a `/sql` endpoint: no
//! connection, no interactive transaction, no cursor. Everything below — a
//! pool, a server-side `DECLARE CURSOR`, a `Transaction` held across awaits —
//! is unavailable there. That is `happenstance-neon`, a separate crate
//! implementing the bare [`EventStore`](happenstance_core::EventStore), not a
//! feature of this one.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. The allow is scoped to this crate
// rather than left open in the workspace manifest so that it is visible in
// review, and it disappears with the last `todo!()` rather than outliving it.
// Phase 10 removes both the bodies and this line.
#![allow(clippy::todo)]

pub mod error;

#[cfg(feature = "event-store")]
pub mod event_store;

#[cfg(feature = "projection-store")]
pub mod projection_store;

#[cfg(feature = "event-store")]
pub mod read_stream;

/// Re-exported so callers can build a pool without pinning their own `sqlx`
/// version against this crate's.
pub use sqlx;
