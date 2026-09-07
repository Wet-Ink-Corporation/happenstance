//! The whole projection suite, reached from outside through nothing but the
//! published surface.
//!
//! One line. No rule is named here, no rule is vendored, and the rule set is not
//! edited: `projection_store_conformance!` expands
//! `for_each_projection_store_rule!` in *this* crate and emits one
//! `#[tokio::test]` per rule, so a failure names the rule that broke rather than
//! reporting "conformance failed".
//!
//! The expansion is also the only thing in this workspace that exercises the
//! testkit's `__private` re-export for real. Every other invocation lives in a
//! crate that already has the fixture trait in scope under its own name; this
//! one does not, and would not compile if the re-export were missing.

// Behind this crate's own `conformance` flag, and it has to be: the flag is
// what compiles `impl ProjectionProbe for OutsideProjectionStore` in `src/`,
// and `ProjectionFixture::Store` is bound on that trait — so without it this
// file does not compile, and a bare `cargo test -p outside-projection-adapter`
// must therefore not see it at all. The gate runs `--all-features`, which is
// what makes the target run rather than quietly configure out. This is the
// shape `crates/happenstance-sqlite/tests/projection.rs` already lives in, and
// it arrived here only when the manifest started following the published
// recipe: while `conformance` was on unconditionally in `[dependencies]`, the
// coupling existed and nothing expressed it.
#![cfg(feature = "conformance")]

mod support;

use support::OutsideFixture;

happenstance_testkit::projection_store_conformance!(OutsideFixture::new());
