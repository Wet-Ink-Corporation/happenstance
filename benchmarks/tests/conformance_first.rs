//! Conformance first, measurement second.
//!
//! # Why this file is the precondition for every number in `results/`
//!
//! **A wrong arm is always the fastest.** A store that skips the append
//! condition, drops rows at a page boundary, or hands out a refcount clone
//! where the adapter opens a connection will post better figures than the real
//! thing — and no timing can tell the difference, because both produce a
//! number.
//!
//! So both fixtures this crate times are run against the full
//! `event_store_conformance!` suite here, and `run.sh` runs this file **before**
//! anything timed. `experiments/append-condition/results/append-condition.md:9-11`
//! states the house rule the same way: *"All three arms passed
//! `event_store_conformance!` first — 89 rules each. An arm that had not would
//! have had its figure discarded."*
//!
//! # What this proves and what it does not
//!
//! It proves that [`SqliteFixture`] — a *copy* of the fixture in
//! `crates/happenstance-sqlite/tests/support/mod.rs`, because that one is
//! `pub(crate)` in a test target and cannot be imported — has not drifted into
//! being a different thing from the fixture the adapter's own CI uses.
//!
//! It does **not** prove the fixture is well-designed. Three properties would
//! pass a wrong fixture and are argued in `src/fixtures/sqlite.rs`'s own
//! documentation rather than here: that `connect` opens a second connection,
//! that `new` mints a fresh file per instance, and that the ceilings are
//! mirrored from the adapter's constants rather than restated.
//!
//! # No benchmark runs in this file
//!
//! It is a conformance target that happens to live in a benchmark crate.
//! Nothing here reads a clock, and CF-34 is untouched: these are the adapter's
//! own rules, executed against the arms, and they were a bar before this crate
//! existed.

use happenstance_benchmarks::fixtures::memory::MemoryFixture;
use happenstance_benchmarks::fixtures::sqlite::SqliteFixture;

// The SQLite arm. `SqliteFixture::new()` mints one temporary file; the suite
// connects onto it as many times as each rule needs.
happenstance_testkit::event_store_conformance!(
    mod_name = sqlite_arm_is_conformant,
    fixture = SqliteFixture::new()
);

// The in-memory arm. `MemoryFixture` is the testkit's own reference
// implementation, so this is not checking the fixture — it is checking that the
// contract crate this suite compiled against is the one whose rules pass, which
// is what makes a memory-versus-SQLite ratio a comparison of two conformant
// stores rather than of one store and one artefact.
happenstance_testkit::event_store_conformance!(
    mod_name = memory_arm_is_conformant,
    fixture = MemoryFixture::new()
);
