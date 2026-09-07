//! The in-memory arm, and one thing it is careful not to hide.
//!
//! `happenstance_testkit::fixtures::MemoryFixture` is already the reference
//! implementation of `Fixture`, so there is nothing to write: this module
//! re-exports it and states what a reader of the results table needs to know
//! about the arm.
//!
//! # What this arm is for, and what it is not
//!
//! It is the arm with no I/O, so a memory-versus-SQLite ratio separates what
//! the *contract* costs from what *storage* costs. `MemoryFixture::connect` is
//! an `Arc` refcount bump returning `core::future::ready`, so fixture setup is
//! effectively free and does not need excluding from a timed region the way
//! SQLite's `open` does.
//!
//! It is **not** an arm whose absolute numbers say anything about scale, and
//! two measured facts say why:
//!
//! * `MemoryEventStore::read` materialises the entire matched result into a
//!   `Vec<SequencedEvent>` under the read lock, cloning every event, before
//!   returning anything (`crates/happenstance-core/src/memory.rs:296-336`). It
//!   is not lazy, despite the port's contract permitting laziness.
//! * Because of that, `limit` truncates *after* the clone. On a store of a
//!   million two-tag events,
//!   `references/evaluation/review-pre-publication-2026-09-03.md:2792` measured
//!   `limit(1)` against a fully-matching query at **4,000,020 heap operations
//!   requesting 229,995,520 bytes** — and `limit(1)` against `limit(None)` at
//!   **1.0000× to four significant figures**. Asking for one event out of a
//!   million costs what asking for all of them costs.
//!
//! The store documents itself as not built for scale and nobody is entitled to
//! be surprised that it is slow. What the suite reports is narrower and worth
//! reporting: that its *snapshot* is priced as cheap and its `limit` as a
//! reduction, and that neither is true. `benches/store_replay.rs` carries the
//! arm that shows it.
//!
//! # Condition evaluation is a linear scan
//!
//! `append` evaluates its condition with `stored.iter().find(...)` over the
//! whole log (`memory.rs:373`), and `contains_event_id` is a full scan with no
//! index (`:414`). Both are documented in the adapter as existing "to be
//! correct and readable, not fast". The contended arm's absolute numbers on
//! this store are therefore a function of log length, and every table carrying
//! them says at what length they were taken.

pub use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};
