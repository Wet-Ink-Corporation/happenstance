//! Three instruments on one handle: what an `append` costs the reactor, what a
//! read page costs everybody else, and what `PAGE_SIZE` actually controls.
//!
//! # The question
//!
//! `crates/happenstance-sqlite` puts one `rusqlite::Connection` behind one
//! `std::sync::Mutex` and hands it to every method of both ports. Three findings
//! in the pre-publication review say that shape costs something specific, and
//! all three say it *without a number*:
//!
//! * **J-2 / F2-1 / I-4** — `append`, `head` and `contains_event_id` run
//!   synchronous SQLite on whatever task polled them
//!   (`crates/happenstance-sqlite/src/event_store.rs:1018-1053`, `:1069-1078`,
//!   `:1090-1106`), so on a `current_thread` runtime an append under contention
//!   stalls the whole reactor. The magnitude decides whether that is a release
//!   blocker or a documentation correction.
//! * **J-5** — `PAGE_SIZE = 512` (`event_store.rs:141`) says of itself *"The
//!   value is a placeholder until it is measured"*, and it is the only knob
//!   sizing how long `fetch_page` holds the connection mutex.
//! * **R-1** — a page's memory is bounded by row count and not by bytes, so its
//!   cost is `PAGE_SIZE × (whatever the largest rows happen to be)`.
//!
//! # What is measured against what
//!
//! **CONTROL 1** is `SqliteProjectionStore::commit`
//! (`crates/happenstance-sqlite/src/projection_store.rs:592-618`). It is the
//! conformant arm — it already routes through `in_blocking_task` — and its
//! maximum tick gap is the floor the event store should reach. Without it a
//! stall number has no scale.
//!
//! **CONTROL 2** is the conformance suite. Every `PAGE_SIZE` this crate times
//! has first run `happenstance_testkit::event_store_conformance!` at that page
//! size (`tests/replica_is_conformant.rs`). A page size that drops rows at a
//! page boundary is fast and wrong.
//!
//! A third control sits under both: the **idle** row. Every reactor-stall table
//! carries the same ticker with no contention and no store call at all, because
//! on Windows the default system timer resolution is 15.6 ms and a 1 ms interval
//! does not produce 1 ms gaps on an idle reactor either.
//!
//! # Real code and faithful copy
//!
//! This crate is **not** allowed to instrument `crates/happenstance-sqlite` in
//! place, so it does both. Which is which:
//!
//! | Measured | Where the code is |
//! | --- | --- |
//! | Instrument (a), every arm | the **real** `SqliteEventStore`, by path dependency |
//! | CONTROL 1 | the **real** `SqliteProjectionStore` |
//! | The second connection's `BEGIN IMMEDIATE` | the **real** `happenstance_sqlite::connection::open_configured` |
//! | The four store ceilings and the 400-arm chunk width | the **real** constants on `SqliteEventStore` |
//! | Instrument (b), the paged read | [`replica`] — a copy, because `PAGE_SIZE`, `fetch_page` and the mutex are all private |
//! | Instrument (c), the seam | [`seam`] — the projection store's `in_blocking_task` transcribed onto the event store's bodies |
//! | The query translation | [`query_sql`] — a **byte-for-byte** copy |
//! | The row codec | [`row`] — a copy under one documented rename |
//!
//! `tests/the_copy_has_not_drifted.rs` re-derives the last two from the
//! originals at the live tree and fails on any difference, and checks the one
//! private constant the copy restates. CONTROL 2 is what holds [`replica`]
//! honest, since no textual check can.
//!
//! # What this crate is not
//!
//! Not a benchmark suite, not a gate step, and not a dependency of anything.
//! CF-34: performance is measured by a separate harness which is not part of the
//! conformance bar. See `run.sh`.

pub mod blocking;
pub mod counting;
pub mod holder;
pub mod probe;
mod query_sql;
pub mod replica;
mod row;
pub mod seam;
pub mod ticker;
pub mod workload;

/// The counting allocator, installed for every test binary this crate links.
///
/// It has to be a `#[global_allocator]` — there is no other way to see what a
/// page fetch inside a `spawn_blocking` closure allocated — and a global
/// allocator is a per-binary singleton. That is the reason this crate is outside
/// the workspace rather than a preference about where it is filed; see
/// `Cargo.toml`.
#[global_allocator]
static ALLOCATOR: counting::Counting = counting::Counting;
