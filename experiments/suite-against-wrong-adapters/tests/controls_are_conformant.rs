//! **Conformance before the count.** The two controls, mounted exactly as an
//! adapter mounts the suite.
//!
//! This runs first in `run.sh`, and it is not ceremony. `tests/census.rs` reaches
//! the rule set through the testkit's `for_each_event_store_rule!` enumeration
//! and a hand-written probe harness, because a failing rule has to become *data*
//! rather than a dead binary. That harness could be subtly wrong — a rule that
//! never ran, a fixture never constructed, a panic caught in the wrong place —
//! and every one of those failure modes produces the same output as "the store
//! passed".
//!
//! So the same fixtures are also mounted through `event_store_conformance!`, the
//! one-line macro `CLAUDE.md` says an adapter must invoke before it is considered
//! to exist. Ninety-odd `#[tokio::test]`s, libtest's own reporting, no harness of
//! this crate's in the path. If the census's controls are green and these are
//! red, the census is wrong and its number is discarded.
//!
//! The four wrong stores are deliberately **absent** from this file. A binary
//! whose purpose is to be red cannot also be the check that the machinery is
//! green; they are driven by `tests/census.rs`, which converts a rejection into a
//! row.

mod support;

use support::wrong_fixtures::CorrectFixture;
use support::harness::Subject;

happenstance_testkit::event_store_conformance!(
    mod_name = memory_reference,
    fixture = happenstance_testkit::fixtures::MemoryFixture::new()
);

happenstance_testkit::event_store_conformance!(
    mod_name = correct_core,
    fixture = CorrectFixture::open()
);
