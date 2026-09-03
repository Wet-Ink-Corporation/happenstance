//! What the append-condition SQL `happenstance-sqlite` actually emits costs, and
//! whether pushing the guard's boundary into it changes the cost class.
//!
//! # Why this exists beside `experiments/append-condition`
//!
//! That experiment measured three tag storages and three strategies and decided
//! ADR-0022. Its general multi-tag arm was
//! `GROUP BY position HAVING COUNT(DISTINCT tag) = n`, and §8's two published
//! figures — the single-tag fast path at 1,093 µs → 556 µs (1.97x) and a two-tag
//! boundary at roughly 200x a single-tag one, 42-66 ms over a 50,000-event log —
//! were both taken against **that** shape.
//!
//! `crates/happenstance-sqlite/src/query_sql.rs:195-233` does not emit that
//! shape. It emits a correlated **intersection chain** seeded by the most
//! selective tag, and `query_sql.rs:48-54` justifies the chain by analogy with
//! the aggregate's measurement rather than with one of its own:
//!
//! > Multi-tag items take an **intersection chain seeded by the most selective
//! > tag**, which keeps the boundary pushable for the same reason […]
//!
//! The chain was never built as an arm and never timed. ADR-0022 §16's falsifier
//! for §8 therefore cannot fire, because the shape it would fire on does not
//! exist anywhere a clock can reach it. This crate builds it.
//!
//! # The four shapes
//!
//! [`Shape`], and every one of them is a conformant store before its figure
//! counts:
//!
//! * [`Shape::Chain`] — the chain, transcribed from `query_sql` and **checked
//!   against the statements a running adapter emits** (`tests/emitted_sql.rs`).
//! * [`Shape::ChainBoundedSeed`] — the same chain with `AND position > ?` bound
//!   into the seed arm. This is finding I-2's one-line fix.
//! * [`Shape::ChainBoundedAllArms`] — the boundary bound into the seed *and*
//!   into every chained membership subquery. It is here because the seed-only
//!   edit turned out not to be the interesting one.
//! * [`Shape::Grouped`] — ADR-0022 §8's aggregate, so that the figures it
//!   produced have something to be compared against **in the same run** rather
//!   than across runs on a host where two slots an hour apart disagree by 45%.
//!
//! # Conformance first, and it is the whole control
//!
//! **A shape that answers a boundary question faster by answering it less
//! completely wins every timing.** That is not a remark, it is the named wrong
//! implementation for this experiment, and it is the specific risk the two
//! bounded shapes carry: a predicate that restricts the matched set is exactly
//! the shape of a guard that has stopped noticing conflicts. So
//! `tests/arms_are_conformant.rs` runs all 89 rules against all four shapes —
//! 356 tests — and `run.sh` runs it **first**, before any timer. A shape whose
//! figure appears in `results/` cleared the suite or its figure was discarded.
//!
//! # What this crate is deliberately not
//!
//! Not an adapter, and not a second one. There is no `spawn_blocking` read
//! stream, no declared ceilings, no projection store. Where a question is about
//! the *real* adapter — the paged read's plan, the statements it emits — the real
//! adapter is used, on a database this crate seeded, because
//! [`store`]'s schema is `MIGRATION_1` verbatim.

#![forbid(unsafe_code)]

pub mod chain;
mod conditions;
pub mod seed;
pub mod probe_store;

pub use chain::{Selectivity, Shape};
pub use conditions::{Conditions, Refused};
pub use probe_store::{ProbeStore, ProbeStoreError};
