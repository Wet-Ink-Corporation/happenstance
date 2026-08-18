//! SQLite adapters for happenstance: an event store and a projection store.
//!
//! # Status: an adapter, and it has run the suite
//!
//! Every operation that touches SQL has a real body — migration, `append`,
//! [`read`](happenstance_core::SendEventStore::read), `head`,
//! `contains_event_id`, `checkpoint` and `commit` — and each of them is executed
//! against a real file on disk rather than against something standing in for
//! one. [`happenstance-testkit`](https://docs.rs/happenstance-testkit)'s
//! conformance suite is mounted twice, at `tests/conformance.rs` for the event
//! store and `tests/projection.rs` for the projection store, with the
//! concurrency and model families beside them at `tests/concurrency.rs`. Passing
//! that suite is what makes an adapter in this workspace rather than an
//! instrument, and it is the bar this crate has now cleared.
//!
//! Whether the crate is *published* is a different question with a different
//! owner: it still carries `publish = false`, and lifting that is the
//! publication pass's decision rather than this crate's. Having passed the suite
//! and being on a registry are two claims, and only the first is made here.
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
//! # What is settled, and what is still open
//!
//! The **driver** is `rusqlite`, chosen for the synchronous, bundled,
//! local-first shape it gives — and it is `rusqlite` *without* a pool, because
//! one `Mutex`-guarded connection is the serialising instrument the portfolio
//! needs. `sqlx` is not discarded; it is where `happenstance-postgres` sits, at
//! the other end of that axis.
//!
//! The **append-condition strategy** and **tag storage** were the two decisions
//! this page listed as open, and ADR-0022 settled both against measurement
//! rather than preference. A guard is one `SELECT max(position)` over the
//! guard's derived query inside a transaction opened `BEGIN IMMEDIATE`; tags
//! live in `event_tag(tag, position)`, `WITHOUT ROWID`, with `event_type`
//! carried as a covering column and a `tag_cardinality` table supplying the
//! per-value selectivity SQLite's `ANALYZE` cannot. [`Tags`](happenstance_core::Tags)
//! is still canonically sorted, and [`event_store`] carries the schema those
//! decisions produced with the measurements that chose it.
//!
//! Two subjects the same ADR left open are recorded here rather than answered,
//! each with an open-question atom of its own in the repository's knowledge
//! base: whether ES-17's `&[Event]` marker on `append` can be lifted, which
//! wants a second adapter's measurement this crate cannot supply alone
//! (`es-17-two-adapter-measurement-is-unscheduled`), and which document owns a
//! fixture-constant clause (`cf-40-fixture-limits-ownership`). Neither is a
//! property of this crate's code, and neither is waiting on it.
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
