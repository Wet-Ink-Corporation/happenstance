//! SQLite adapters for happenstance: an event store and a projection store.
//!
//! # Status: an instrument, not yet an adapter
//!
//! Every operation that touches SQL is `todo!()` — migration, `append`, the
//! page query, `checkpoint` and `commit`. Two are **not**, and the exception is
//! worth stating rather than rounding off:
//! [`begin`](happenstance_core::SendProjectionStore::begin) and
//! [`rollback`](happenstance_core::SendProjectionStore::rollback) have real
//! bodies, because an
//! owned buffer batch is created and discarded without the database being
//! involved at all. That is a consequence of the batch shape this crate adopted,
//! so a blanket "every operation is `todo!()`" would hide the one place the
//! shape already shows through. The **types are real**: a live
//! [`rusqlite::Connection`], error enums that wrap [`rusqlite::Error`], a read
//! stream that is a genuine state machine, and a batch that owns
//! [`rusqlite::types::Value`]. That distinction is the whole point — a skeleton
//! that stubs its associated types has stubbed the only part of it a type
//! checker can disagree with, so the associated types are exactly what is not
//! stubbed here.
//!
//! It is `publish = false` until it passes
//! [`happenstance-testkit`](https://docs.rs/happenstance-testkit)'s conformance suite —
//! which is the bar for any adapter in this workspace, not a formality.
//!
//! # The shape this crate represents
//!
//! **Serialising, `Send`, native.** One connection behind one [`Mutex`](std::sync::Mutex),
//! so writers queue by construction and positions are assigned under a lock. It
//! implements [`SendEventStore`](happenstance_core::SendEventStore) and
//! [`SendProjectionStore`](happenstance_core::SendProjectionStore), and it is
//! deliberately *one* point in the portfolio rather than the reference: a port
//! frozen against this shape alone would be frozen against SQLite wearing four
//! hats.
//!
//! # What the type checker has already decided
//!
//! Two results, both compiled rather than reasoned:
//!
//! * A `rusqlite::Transaction<'a>` batch is **not available** on the `Send`
//!   flavour, for two independent reasons that each fire on their own. See
//!   [`projection_store`] for both, and for the owned batch that replaces it.
//! * While the port declared a generic associated type, binding an owned type
//!   to it did **not** free an implementer from spelling the parameter
//!   `Self::Batch<'_>` literally; writing the concrete type was still
//!   `error[E0195]`. ADR-0017 removed the lifetime, so `type Batch =
//!   SqliteBatch;` and `commit(&self, batch: Self::Batch, …)` now compile —
//!   this crate is one of the impls that shows it.
//!
//! # Open decisions
//!
//! The **driver** is no longer one of them: `rusqlite` is what this crate is
//! built on, chosen for the synchronous, bundled, local-first shape it gives —
//! and it is `rusqlite` *without* a pool, because one `Mutex`-guarded connection
//! is the serialising instrument the portfolio needs. `sqlx` is not discarded;
//! it is where `happenstance-postgres` sits, at the other end of that axis.
//!
//! These are settled in the pass that implements this crate, not before:
//!
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

#[cfg(any(feature = "event-store", feature = "projection-store"))]
pub mod connection;

#[cfg(feature = "event-store")]
pub mod event_store;

#[cfg(feature = "event-store")]
mod query_sql;

#[cfg(feature = "event-store")]
mod row;

#[cfg(feature = "projection-store")]
pub mod projection_store;
