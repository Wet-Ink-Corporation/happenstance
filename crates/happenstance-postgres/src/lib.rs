//! Postgres adapters for happenstance: an event store and a projection store.
//!
//! # Status
//!
//! The **event store is implemented**, and run rather than asserted: 107 of 107
//! gated tests pass against a live PostgreSQL 17.10, including the concurrency
//! family at `CONTENDERS = 64`. That is the first time an adapter in this
//! workspace has cleared that family against a store whose writers are **not**
//! serialised, which is the whole reason the crate is in the tree.
//!
//! **The projection store beside it is implemented too**, and the crate carries
//! no `todo!()` and no `#![allow(clippy::todo)]` any more — the allow left with
//! the last stub, which is the contract it was written under.
//!
//! It carries **two** projection stores, and they are not two ways of doing the
//! same thing. [`projection_store`]'s batch is an owned statement list replayed
//! at commit — the shape an application uses, because the typed layer's
//! `Projection::apply` is synchronous and can push into a buffer. The skeleton
//! had declared `sqlx::Transaction<'static, Postgres>` instead, and that binding
//! could not be discharged while `begin` was total, synchronous and infallible;
//! a skeleton did not report it because `todo!()` has type `!` and coerces to
//! anything. ADR-0062 moved `begin` and the probe seam, and
//! [`live_projection_store`] is that binding discharged: a batch that **is** a
//! live transaction, standing at the far end of PS-2's batch-shape axis and
//! passing the suite there — an instrument for the freeze rather than a product.
//!
//! **The crate ships in `0.2.0`.** The release set was five, decided, and the
//! owner re-opened it on this crate's evidence at `e597c34`; the manifest carries
//! no `publish` key. This paragraph said the opposite until the release pass,
//! which would have told a docs.rs reader that the crate they were reading was
//! unpublished.
//!
//! **What a dropped `append` future does here is answered in
//! [`event_store`]'s `# Cancellation` section**, which ES-23 obliges this
//! adapter to carry. The short version, because a caller should not have to
//! click to learn it: the future cannot be cancelled and the batch may already
//! have committed.
//!
//! # Why this crate exists
//!
//! It is an instrument first and a target second. Every other adapter in the
//! workspace — `MemoryEventStore`, a `RefCell` store, rusqlite, a Durable
//! Object — assigns positions under a lock it holds until commit, so all four
//! satisfy the specification's ES-10 (*position order is visibility order*) for
//! free, and none of them votes for it. Postgres is the one shape on the roadmap
//! that can violate ES-10 **by construction**: `nextval()` allocates outside the
//! transaction, so a writer that took position 99 can commit after a writer that
//! took position 100, and a reader that has already seen 100 then watches 99
//! appear beneath it. `AppendCondition::after(100)` evaluates `99 <= 100` and
//! reports no violation, so the consistency boundary silently stops enforcing.
//!
//! This crate buys the invariant back rather than avoiding the shape, and the
//! next section is what that costs the caller.
//!
//! # What this store costs a caller
//!
//! Read this before choosing the adapter. Everything below is a **capability
//! limit** — a thing the store does not do — and not a defect to report, not a
//! setting, and not something an operator can tune down.
//!
//! ES-10 is bought with `xid8` + `pg_snapshot_xmin`. Every row stamps the
//! transaction that wrote it, and every read admits only rows beneath
//! `pg_snapshot_xmin(pg_current_snapshot())` — the transaction id below which
//! nothing can still be in flight. Writers are never serialised in the sense that
//! matters: this store takes no lock that makes disjoint boundaries queue behind
//! one another, which is the property it exists to have.
//!
//! **Most of the bill is paid on the read side, and there is a fourth item that
//! is not.** The read-side costs are the three below. The write-side one is this:
//! an *unconditional* append runs at the pool's default `READ COMMITTED` and pays
//! nothing extra, but a *conditional* append runs `SERIALIZABLE`, so two
//! conditional appends whose predicates overlap are adjudicated at commit and one
//! is aborted with `40001`. This adapter re-runs the loser up to
//! `SERIALISATION_ATTEMPTS` times with exponential, capped, fully-jittered
//! backoff — so a writer already losing a serialisation fight can pay several
//! round trips and a fraction of a second of waiting before it gets its answer.
//! That is retry inside the adapter and is invisible to the caller as anything
//! but latency. The constant's own doc block records that its value was adopted
//! from the Neon lane rather than measured here, and that a re-measurement is
//! owed.
//!
//! Three read-side things follow, and the crate does not soften them.
//!
//! **`head()` reports a visibility frontier, not `max(position)`, and the
//! frontier trails the maximum.** A head is a promise that nothing at or below
//! it will appear later; `max(position)` cannot make that promise here, because
//! it can name a row whose predecessors are still uncommitted. So `head()`
//! returns the highest position beneath the frontier, which legitimately sits
//! below the highest position this store has already assigned. The conformance
//! rule for a head asserts a *bound* rather than an equality for exactly this
//! reason.
//!
//! **Read-your-own-writes does not hold, and is not claimed.** `append`
//! returning `Ok(P)` does **not** promise that the next `head()` is at or above
//! *P*, and a read issued immediately after a successful append may not contain
//! the event that append just wrote. Nothing is lost — the row is committed and
//! will become visible — but a workflow that writes and then reads back through
//! this store to confirm the write is not one this adapter serves.
//!
//! **Staleness is bounded by the longest open write transaction anywhere on the
//! cluster.** Not on this table and not in this database: any write transaction
//! held open by anything connected to the same server holds the frontier where
//! it is. A migration, a batch job, an idle-in-transaction connection or an
//! unrelated tenant is enough, and it is a bound a consumer of this crate
//! neither controls nor can necessarily observe.
//!
//! **What to budget for.** Sub-millisecond when nothing else is holding a write
//! transaction open, and **the whole remaining duration of the longest one that
//! is** otherwise. The remainder of the holder, not a fraction of it.
//!
//! Measured against this adapter, nine samples per cell: a median of 0.593 ms
//! with no holder, and 4,799 ms behind a five-second held write transaction
//! writing to its own unrelated table — while the same adapter with the
//! visibility predicate removed is unaffected by the identical hold, at
//! 0.595 ms. Same server, same hold, same load; only the predicate differs. So
//! the figure to plan for is whatever your cluster's longest write transaction
//! is, and if you cannot bound that, you cannot bound this.
//!
//! ```no_run
//! use happenstance_core::{Event, SendEventStore};
//! use happenstance_postgres::event_store::PostgresEventStore;
//!
//! # async fn confirm(
//! #     store: &PostgresEventStore,
//! #     events: &[Event],
//! # ) -> Result<(), Box<dyn std::error::Error>> {
//! let written = store.append(events, None).await?;
//!
//! // `written` is where the event landed. It is not a position `head()`
//! // promises to have reached, so this is a bound and never an equality —
//! // `head() == Some(written)` is the assertion a caller must not write.
//! assert!(store.head().await? <= Some(written));
//! # Ok(())
//! # }
//! ```
//!
//! Three other mechanisms were measured and lost, and naming them once is
//! cheaper than having them re-proposed: a serialised sequence table (0.062 of
//! baseline throughput at 64 writers) and a constant advisory lock (0.033) are
//! both correct and both buy ES-10 by deleting the reason to reach for Postgres,
//! while a tag-keyed advisory lock is nearly free (0.935) and lost on the
//! **invariant** rather than on cost — it buys a per-boundary property where
//! ES-10 states a global one. `references/adr/0024-position-visibility-mechanism.md`
//! in this crate's repository carries the argument and the measurements.
//!
//! # Still open
//!
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

// `--cfg docsrs` is set by this crate's `[package.metadata.docs.rs]`, and it is
// what makes the `doc(cfg(...))` badges below render a feature gate rather than
// nothing. The manifest asserted that was happening while neither the feature
// nor a single attribute existed, so every gated item rendered as ordinary API.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_no_source)]
pub mod error;

#[cfg(feature = "event-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "event-store")))]
pub mod event_store;

// Both roles' schemas live here, each item gated on its own feature. The module
// is reachable from either, because a `projection-store`-only build needs
// `apply_projection` and has no event log at all — and a module gated on
// `event-store` would have hidden it, which is the defect this line exists to
// avoid rather than a preference about layout.
#[cfg(any(feature = "event-store", feature = "projection-store"))]
pub mod migration;

#[cfg(feature = "event-store")]
mod query_sql;

#[cfg(feature = "projection-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "projection-store")))]
pub mod projection_store;

#[cfg(feature = "projection-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "projection-store")))]
pub mod live_projection_store;

#[cfg(feature = "event-store")]
#[cfg_attr(docsrs, doc(cfg(feature = "event-store")))]
pub mod read_stream;

/// Re-exported so callers can build a pool without pinning their own `sqlx`
/// version against this crate's.
pub use sqlx;

/// Re-exported for ADR-0044's reason, and the trigger it named has arrived.
///
/// That decision states its own trigger as *"the moment `xtask/src/package.rs`'s
/// `reconcile` check would otherwise let the crate onto the registry without
/// it"*, and `0.2.0` is that moment. The contract is in this crate's public
/// signatures rather than merely behind them: `impl SendEventStore for
/// PostgresEventStore` names `Query`, `ReadOptions`, `SequencedEvent`, `Event`,
/// `AppendCondition` and `AppendError`, so a caller cannot use the store without
/// naming types it does not otherwise depend on. The dependency is
/// unconditional, so this needs no `cfg`.
pub use happenstance_core;
