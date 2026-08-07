//! SQLite adapters for happenstance: an event store and a projection store.
//!
//! # Status: not implemented
//!
//! The crate compiles and its shape is fixed, but every operation is
//! `todo!()`. It is `publish = false` until it passes
//! [`happenstance-testkit`](https://docs.rs/happenstance-testkit)'s conformance suite —
//! which is the bar for any adapter in this workspace, not a formality.
//!
//! # Open decisions
//!
//! These are settled in the pass that implements this crate, not before:
//!
//! * **Driver.** `rusqlite` (synchronous, bundles SQLite, the natural fit for a
//!   local-first application) against `sqlx` (async-native, and a path to
//!   Postgres later). The current lean is `rusqlite` with a small connection
//!   pool, with writes marshalled through `spawn_blocking` on native.
//! * **Append-condition strategy.** The append must evaluate the condition and
//!   write in one atomic step. Candidates: `BEGIN IMMEDIATE` plus an
//!   `EXISTS` probe; a conditional `INSERT ... SELECT ... WHERE NOT EXISTS`; or
//!   a monotonic-position guard. Which one wins depends on how tag matching is
//!   indexed.
//! * **Tag storage.** A join table against a canonical serialised blob against
//!   SQLite's JSON1 functions. [`Tags`](happenstance_core::Tags) is canonically
//!   sorted precisely so that the blob option stays open.
//!
//! # Not the Cloudflare adapter
//!
//! A Durable Object's SQLite is reached through the Workers `SqlStorage` API,
//! not through a SQLite driver, and its futures are `!Send`. That is a separate
//! adapter crate implementing
//! [`EventStore`](happenstance_core::EventStore) rather than
//! [`SendEventStore`](happenstance_core::SendEventStore) — not a feature of this
//! one.

#![doc(html_no_source)]
// `clippy::todo` is denied workspace-wide. This crate is the one exception, and
// the exception is scoped here rather than left open in the workspace manifest so
// that it is visible in review and disappears with the last `todo!()` rather than
// outliving it. Phase 8 removes both the bodies and this line.
#![allow(clippy::todo)]

#[cfg(feature = "event-store")]
pub mod event_store;

#[cfg(feature = "projection-store")]
pub mod projection_store;
