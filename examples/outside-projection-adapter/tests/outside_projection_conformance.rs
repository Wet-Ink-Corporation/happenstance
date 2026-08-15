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

mod support;

use support::OutsideFixture;

happenstance_testkit::projection_store_conformance!(OutsideFixture::new());
