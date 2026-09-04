//! SQLite adapters for happenstance: an event store and a projection store.
//!
//! # Status: an adapter, and it has run the suite
//!
//! Every operation that touches SQL has a real body — migration, `append`,
//! [`read`](happenstance_core::SendEventStore::read), `head`,
//! `contains_event_id`, `checkpoint` and `commit` — and each of them is executed
//! against a real file on disk rather than against something standing in for
//! one. [`happenstance-testkit`](https://docs.rs/happenstance-testkit)'s
//! conformance suite is mounted three times: `tests/conformance.rs` carries the
//! event-store family and the model family, `tests/projection.rs` the
//! projection-store family, and `tests/concurrency.rs` the concurrency family.
//! Passing that suite is what makes an adapter in this workspace rather than an
//! instrument, and it is the bar this crate has now cleared.
//!
//! Whether the crate is *published* is a different question with a different
//! owner. `publish = false` is gone from its manifest, and its absence is half
//! of an atomic pair: `PUBLISHABLE` (`xtask/src/package.rs`) names this crate,
//! and `reconcile` fails on either half alone. So this crate is packaged by the
//! gate and is in the `0.2.0` release set — but having passed the suite and
//! being live on a registry are still two claims, and only the first is made
//! here. Only the registry can say whether that release has happened yet.
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

/// Re-exported so a caller can name the driver types this crate's own
/// signatures name — `Connection` on both constructors, `rusqlite::Error` and
/// `types::Value` inside the error enums and the projection batch — without
/// adding a second `rusqlite` of their own for the resolver to fork on. A
/// consumer already carrying `rusqlite` for their own tables, at a requirement
/// that does not overlap this crate's, writes
/// `if let SqliteEventStoreError::Sqlite(e) = err` and meets `error[E0308]`
/// over two types that print identically — on the error path, long after
/// `open` and `open_in_memory` let them build and append without naming a
/// foreign type at all.
///
/// See `reexported_paths` below for what this guarantee is and is not.
pub use rusqlite;

/// Re-exported because the contract is in this crate's public signatures rather
/// than merely behind them: `impl EventStore for SqliteEventStore` names
/// `Query`, `ReadOptions`, `SequencedEvent`, `Event`, `AppendCondition` and
/// `AppendError`, and `StoreLimit` is a field of two error variants.
pub use happenstance_core;

/// Compiled proof that every path this crate promises a caller actually
/// resolves from outside it — and the statement of what that promise is not.
///
/// The guarantee is the one `happenstance_core`'s own `reexported_paths` states:
/// **type identity and discoverability**, so that the `rusqlite::Error` a caller
/// matches on is the one this adapter's error enum actually carries rather than
/// a second copy that prints the same. It is never a substitute for a
/// consumer's own dependency.
///
/// **`tokio` is the crate deliberately left out of that set, and the omission is
/// that qualification made concrete.** `tokio::task::JoinError` and
/// `tokio::runtime::TryCurrentError` are variants of this crate's error enums,
/// so by RS-40-4's own arithmetic `tokio` belongs here, and it was re-exported
/// until this release. The feature half is what removed it: this crate takes
/// `tokio` at `features = ["rt"]` (`Cargo.toml:37`), so what arrived through
/// `happenstance_sqlite::tokio` was a **partial** `tokio` — no `macros`, no
/// `rt-multi-thread`, no `time`. A consumer who reached it only through that
/// path and then wrote `#[tokio::main]` met an `error[E0433]` one layer further
/// from its cause than the `error[E0308]` the re-export existed to prevent. A
/// path that has to be read twice before it is safe is not a shorter route to
/// the type.
///
/// What that costs, stated rather than glossed: a caller matching on `JoinError`
/// writes a `tokio` line of their own, and identity then rests on cargo unifying
/// the two rather than on this crate guaranteeing it. Semver-compatible
/// requirements unify, which covers every consumer who already had a `tokio`
/// line; a consumer who pins a different *major* than this crate resolves gets
/// the two-types-that-print-identically failure the re-export set exists to
/// prevent, and `cargo tree -d` is what names it.
///
/// And the omission is compiled too, so that re-adding the re-export cannot pass
/// unnoticed: this is the path a reader following the old documentation would
/// take, and it must not resolve.
///
/// ```compile_fail,E0433
/// fn joined(e: happenstance_sqlite::tokio::task::JoinError)
///     -> happenstance_sqlite::tokio::task::JoinError { e }
/// # fn main() {}
/// ```
///
/// ```
/// fn open(c: happenstance_sqlite::rusqlite::Connection)
///     -> happenstance_sqlite::rusqlite::Connection { c }
/// # fn main() {}
/// ```
///
/// ```
/// fn param(v: happenstance_sqlite::rusqlite::types::Value)
///     -> happenstance_sqlite::rusqlite::types::Value { v }
/// # fn main() {}
/// ```
///
/// The contract is in this crate's public signatures rather than merely behind
/// them — `impl EventStore for SqliteEventStore` names `Query`, `ReadOptions`,
/// `SequencedEvent`, `Event`, `AppendCondition` and `AppendError` — so it is
/// reachable under its own name here too:
///
/// ```
/// fn bound<S: happenstance_sqlite::happenstance_core::EventStore>(_s: S) {}
/// # fn main() {}
/// ```
#[cfg(doctest)]
mod reexported_paths {}
