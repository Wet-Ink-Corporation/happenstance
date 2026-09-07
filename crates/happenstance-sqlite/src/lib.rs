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
//! **What a dropped `append` future does** is ES-23's question, and it is
//! answered in [`event_store`]'s own `# Cancellation` section — beside the
//! body it is about.
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
// `docs.rs` builds this crate with `--all-features` and `--cfg docsrs`
// (`Cargo.toml`'s `[package.metadata.docs.rs]`), which means the rendered page
// shows `projection_store` beside `event_store` with nothing to distinguish
// them. Two of this crate's three features are off by default and one of those
// gates an explicitly unstable port, so a reader who cannot see a badge draws
// the wrong conclusion from a page that is otherwise accurate — they add
// `SqliteProjectionStore` to a project and discover the feature flag from a
// compiler error, and the semver exemption never.
//
// `feature(doc_cfg)` is nightly, which is why this is `cfg_attr`-gated on
// `docsrs` rather than written plainly: the flag is set by the docs.rs build and
// by the gate's own nightly rustdoc step, and by nothing a consumer runs.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(any(feature = "event-store", feature = "projection-store"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "event-store", feature = "projection-store")))
)]
pub mod connection;

#[cfg(feature = "event-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "event-store")))]
pub mod event_store;

#[cfg(feature = "event-store")]
mod query_sql;

#[cfg(feature = "event-store")]
mod row;

// No `///` doc on this declaration, and that is a constraint rather than a
// preference: a doc comment written *here* is resolved in **this** module's
// scope, while the module's own `//!` header is resolved in its own. Attaching
// one made every intra-doc link inside `projection_store.rs` — `SendProjectionStore`,
// `SqliteBatch`, `SqliteProjectionStoreError` — fail to resolve, and
// `-D rustdoc::broken-intra-doc-links` turned that into four errors. The
// stability note this used to carry now lives in the module's own header, where
// the names it wants to link to are in scope.
#[cfg(feature = "projection-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "projection-store")))]
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

/// Compiled proof that [`Op::Read`] cannot be built from outside the testkit,
/// and can still be matched.
///
/// **Why here rather than in `happenstance-testkit`.** `#[non_exhaustive]` is
/// inert inside the crate that defines it, so the testkit's own tests cannot
/// fail this and never could — the same reason `happenstance-testkit` hosts the
/// equivalent proof for `happenstance_core`'s `Query::Items` rather than
/// `happenstance-core` doing it. This crate is the first one downstream of the
/// testkit that mounts the model family
/// (`tests/conformance.rs:102`), so it is where the seal is real.
///
/// **What the seal is for.** `Op` is the model family's alphabet, and a
/// downstream crate building an `Op::Read` by hand is building an operation the
/// generator's weighting never produced and the model was never checked
/// against — a `Model::apply` answer nobody has a reason to trust, reported as
/// a conformance result. The variant is also the one that demonstrably grows:
/// `to` arrived at the `0.2.0` pass and `limit`'s own documentation already
/// names VT-28 as the next widening. Sealing it makes that growth additive,
/// which is what `ReadOptions` upstream has had since phase 4 and what this
/// variant, mirroring it option-for-option, did not.
///
/// **Rejects:** an `Op::Read` without `#[non_exhaustive]`. Strike the attribute
/// and the first block below compiles, so the test fails with *"Test compiled
/// successfully, but it's marked `compile_fail`"*.
///
/// ```compile_fail
/// use happenstance_core::Query;
/// use happenstance_testkit::model::{Anchor, Op};
///
/// let op = Op::Read {
///     query: Query::all(),
///     from: Anchor::Unset,
///     to: Anchor::Head,
///     backwards: false,
///     limit: None,
/// };
/// assert!(matches!(op, Op::Read { .. }));
/// ```
///
/// The **twin** is the same block with the one refused expression removed, and
/// it must compile. Every name in the snippet above appears in it — `Query`,
/// `Anchor`, `Op`, the module path, all five field names in the pattern — so a
/// renamed item, a moved module or a feature that stopped being forwarded
/// breaks the twin, and a broken twin is a hard failure rather than a quietly
/// satisfied `compile_fail`. That pairing is not belt and braces: it is
/// measured, in `experiments/wire-format/`, where a type-name typo, a misspelt
/// trait and a wrong crate path all reported ok against a false claim. The
/// error-code annotation does not close it either — rustdoc on 1.97.1 silently
/// ignores one it cannot match, so `compile_fail,E0639` would be the weaker
/// check and not the stricter one.
///
/// The pair was nonetheless checked against the compiler directly rather than
/// argued: compiled as an ordinary integration test in this crate, that block
/// is `error[E0639]: cannot create non-exhaustive variant using struct
/// expression`, and that is the **only** error it produces. Recorded because it
/// is the one thing neither half of the pair can report about itself.
///
/// ```
/// use happenstance_core::Query;
/// use happenstance_testkit::model::{Anchor, Op};
///
/// // The five field values the refused expression wanted. Binding them here is
/// // what proves the block above fails on its construction and not on any of
/// // the names it happens to mention.
/// let (query, from, to, backwards, limit) =
///     (Query::all(), Anchor::Unset, Anchor::Head, false, None::<usize>);
/// assert!(!query.is_all() || from != to || !backwards || limit.is_none());
///
/// // Matching stays legal downstream, which is what variant-level
/// // `#[non_exhaustive]` buys over enum-level: `Op` itself carries no
/// // attribute, so this `match` is still exhaustive over the three variants
/// // and still stops compiling the day a fourth is added — which is the
/// // report the author of that fourth variant wants, and what RS-13-5 is
/// // protecting.
/// let op = Op::Append { events: Vec::new() };
/// let name = match op {
///     Op::Append { .. } => "append",
///     Op::AppendConditional { .. } => "conditional",
///     Op::Read { .. } => "read",
/// };
/// assert_eq!(name, "append");
/// ```
///
/// [`Op::Read`]: happenstance_testkit::model::Op::Read
#[cfg(doctest)]
mod op_read_is_sealed_downstream {}
