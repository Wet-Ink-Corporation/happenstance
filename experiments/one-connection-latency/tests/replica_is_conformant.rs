//! **CONTROL 2.** Every `PAGE_SIZE` this crate times has first run the
//! conformance suite at that page size.
//!
//! # Why this control and not a spot check
//!
//! A page size is not a tuning knob that can only be slow. `PAGE_SIZE` sizes the
//! `LIMIT` on every per-chunk statement, seeds `budget`, and is the number
//! `exhausted = merged.len() < budget` is computed against — so a wrong page
//! size does not produce a slow read, it produces a read that stops early or
//! repeats a row at a page boundary. `crates/happenstance-sqlite`'s own
//! `ReadCursor::resume_from` doc records exactly that bug having happened once
//! already, in the shipped adapter, at the page boundary
//! (`event_store.rs:1196-1207`).
//!
//! **A page size that drops rows at a boundary is fast and wrong**, and it wins
//! every table in `results/`. So the suite runs at 64, 128, 512 and 2048 —
//! every value later timed, including the shipped 512 — and a figure taken at a
//! page size whose column here is red is discarded.
//!
//! The suite's own multi-page criteria seed past `2 × PAGE_SIZE` rows, so at
//! `PAGE = 64` these runs cross a page boundary many times over; at `PAGE =
//! 2048` most of them do not cross one at all. That asymmetry is the point:
//! between them the four columns cover both regimes.
//!
//! This is also the only check that holds `src/replica.rs` to being a faithful
//! copy. `tests/the_copy_has_not_drifted.rs` can compare text for the two files
//! that were copied verbatim; nothing can compare text for a copy that was
//! deliberately modified, so what stands in its place is 89 rules per page size.
//!
//! Run it with `cargo test --manifest-path
//! experiments/one-connection-latency/Cargo.toml --test replica_is_conformant`.

mod support;

use support::ReplicaFixture;

happenstance_testkit::event_store_conformance!(
    mod_name = page_size_64,
    fixture = ReplicaFixture::<64>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = page_size_128,
    fixture = ReplicaFixture::<128>::new()
);

// The shipped value (`crates/happenstance-sqlite/src/event_store.rs:141`).
happenstance_testkit::event_store_conformance!(
    mod_name = page_size_512,
    fixture = ReplicaFixture::<512>::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = page_size_2048,
    fixture = ReplicaFixture::<2048>::new()
);
