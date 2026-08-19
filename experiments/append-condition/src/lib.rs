//! Three candidate append-condition strategies, three tag storages, one
//! schema — measured before the adapter exists.
//!
//! # Why this is not in `happenstance-sqlite`
//!
//! `SqliteEventStore::append` is `todo!()` and stays that way until three
//! stories from now: AC-013 puts ADR-0022's *record* before the
//! implementation, and a record that quotes a preference instead of a figure is
//! the thing that AC exists to refuse. So the number has to come from somewhere
//! that is not the adapter, and the precedent is
//! `experiments/position-visibility/`, which measured the Postgres
//! position-visibility question one phase before any Postgres adapter existed.
//!
//! This crate is that somewhere. It is **not a workspace member** (see
//! `Cargo.toml`), it is in no `verify:` command and no `cargo xtask ci` step,
//! and it adds no dependency to any workspace manifest.
//!
//! # What is varied, and what is held fixed
//!
//! One schema, one read path, one identity story. Two axes move:
//!
//! * **The append-condition strategy** — [`BeginImmediateProbe`],
//!   [`ConditionalInsert`], [`MonotonicGuard`]. Exactly the three
//!   `crates/happenstance-sqlite/src/lib.rs:56-62` names; a fourth would stop
//!   the record answering the question it was queued for.
//! * **The tag storage** — [`JoinTable`], [`CanonicalBlob`], [`Json1`].
//!   Exactly the three at `lib.rs:63-65`. `Tags` is canonically sorted
//!   *precisely so* the blob arm stays open, so it is measured rather than
//!   dismissed on taste.
//!
//! Nine combinations exist; the README says which were run and why the rest
//! were not.
//!
//! # Every arm is a conformant store before its figure counts
//!
//! `tests/candidates_are_conformant.rs` points `event_store_conformance!` at
//! each arm. A wrong arm is always the fastest, so a figure from an arm that
//! has not cleared the suite is discarded — that is AC-001's content and EC-003's
//! required behaviour.
//!
//! # The durability settings are enforced, not assumed
//!
//! `spec/SPECIFICATION.md:7481-7484` names `PRAGMA synchronous = OFF` **by
//! name** as a wrong implementation CF-14's reopen rule exists to reject. A
//! figure produced under it is a figure for a store that cannot ship, so
//! [`Durability::read_back`] reads `journal_mode` and `synchronous` off the live
//! connection and [`Durability::require_shippable`] refuses to let a
//! measurement start under a setting the adapter may not use. That is the exact
//! analogue of `experiments/position-visibility/setup.sh` aborting under
//! `fsync=off`.
//!
//! # What this crate is deliberately not
//!
//! Not an adapter. There is no pool, no page-budget tuning, no `spawn_blocking`
//! read stream, no projection store and no migration framework — every one of
//! those belongs to a story in `durable-event-store`, and building them here
//! would be this story implementing the thing it exists to decide *before*.
//!
//! # Where the fixture is, and why it is not here
//!
//! `CandidateFixture` lives in `tests/support/mod.rs`, not in `src/`, and that
//! is the shape the testkit's own extension page asks an adapter author for:
//! the fixture belongs in `tests/` and `happenstance-testkit` stays a
//! **dev-dependency**. Putting it in `src/` would drag the suite into this
//! crate's normal dependency graph, which is exactly the edge an adapter must
//! not have.

#![forbid(unsafe_code)]

mod candidate;
mod durability;
mod strategy;
mod tags;

pub use candidate::{CandidateStore, SqliteProbeError};
pub use durability::{Durability, DurabilityRefused, JournalMode, Synchronous};
pub use strategy::{AppendStrategy, BeginImmediateProbe, ConditionalInsert, MonotonicGuard};
pub use tags::{CanonicalBlob, JoinTable, JoinTableGrouped, Json1, TagStorage};
