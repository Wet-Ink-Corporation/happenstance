//! A projection fixture that answers **no** capability at all, and what the
//! suite then says about it.
//!
//! # Why this file exists
//!
//! `ProjectionFixture`'s three capability constants were **required** — no
//! defaults — under a stated trait-level policy: *"the projection family's
//! declension policy is that the fixture writes the reason"*
//! (`contract.rs`, on `RESET_REFUSAL`). That policy is retracted. The
//! repository owner ratified Option A of
//! `.kb/_intake/remediation-2026-09-04-briefs/fixture-declension-policy.md`:
//! every capability on either fixture trait lands **defaulted**, and the
//! honesty it gives up is recovered as a CF-39-shaped clause-level MUST per
//! capability rather than by the trait's requiredness.
//!
//! The mechanical half of that is a compile-time fact, and a compile-time fact
//! is exactly the kind that rots silently: nothing in a passing test suite
//! notices that a `const` acquired or lost a default. [`Silent`] below is that
//! notice. It implements `ProjectionFixture` and writes **not one** capability
//! constant, so it stops compiling the moment any of the three goes back to
//! being required — with `error[E0046]: not all trait items implemented`, which
//! is the diagnostic an adapter author on a minor bump would have met.
//!
//! # What it does *not* claim
//!
//! It does not claim the defaults are good sentences. A default reason is
//! testkit prose printed in an adapter's CI log as though it were the adapter's
//! own account of itself, and that cost is real and is argued at length in the
//! brief. What this file holds is the two things a test can hold: that a
//! fixture may stay silent, and that staying silent about a **MUST** is still
//! rejected — loudly, by the rule an adapter's own CI runs, rather than by a
//! compiler an adapter author on a minor bump would never have met either.
//!
//! Native only: the assertions below drive a rule through
//! [`happenstance_testkit::block_on`] and catch its panic, and
//! `catch_unwind` is not how a wasm32 target reports one.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use std::panic;

use happenstance_testkit::fixtures::{MemoryProjectionFixture, MemoryProjectionHandle};
use happenstance_testkit::{ProjectionFixture, block_on};

/// A projection fixture that declares **no** capability constant.
///
/// It delegates everything to [`MemoryProjectionFixture`], so the only
/// difference between the two is the thing under test: this one answers nothing
/// and takes whatever the trait provides.
#[derive(Debug, Default)]
struct Silent(MemoryProjectionFixture);

impl ProjectionFixture for Silent {
    type Store = MemoryProjectionHandle;

    // Deliberately no `SECOND_HANDLE`, no `RESET_REFUSAL`, no `COMMIT_FAULT`.
    // The absence is the assertion; see the module documentation.

    fn connect(&self) -> impl Future<Output = Self::Store> {
        self.0.connect()
    }
}

/// Every capability on `ProjectionFixture` is defaulted, and every default is a
/// declension carrying a reason.
///
/// The *compile* is half the test — an impl naming none of the three has to be
/// accepted — and the assertions are the other half: a default that was
/// `SUPPORTED` would let a fixture claim a capability by saying nothing, which
/// is the one answer worse than the testkit writing its reason for it.
#[test]
fn a_projection_fixture_may_answer_nothing_and_declines_everything() {
    for (name, capability) in [
        (
            "SECOND_HANDLE",
            <Silent as ProjectionFixture>::SECOND_HANDLE,
        ),
        (
            "RESET_REFUSAL",
            <Silent as ProjectionFixture>::RESET_REFUSAL,
        ),
        ("COMMIT_FAULT", <Silent as ProjectionFixture>::COMMIT_FAULT),
    ] {
        let reason = capability.reason();
        assert!(
            reason.is_some(),
            "`{name}`'s default is `SUPPORTED`, so a fixture that says nothing \
             claims the capability. A default may decline on an author's behalf; \
             it may never declare on their behalf"
        );
        assert!(
            !reason.unwrap().trim().is_empty(),
            "`{name}`'s default declension carries no reason, so the suite would \
             print a shrug (CF-18)"
        );
    }
}

/// Defaulting a MUST does not make it skippable.
///
/// `SECOND_HANDLE` is the projection family's one MUST, and the argument for
/// leaving it *required* was that the compiler then forces an answer. Under
/// Option A the compiler no longer does, so the whole of the enforcement is
/// `must!` inside the rules — on the path an adapter's own CI executes rather
/// than in a meta-test that never runs there. This asserts that path is intact:
/// a fixture that says nothing is **rejected**, not skipped, and the failure
/// quotes the stated reason back.
#[test]
fn saying_nothing_about_the_one_must_still_fails_the_rules() {
    let previous = panic::take_hook();
    // The rule below is *expected* to panic; without this the expected
    // backtrace is sprayed across a passing run.
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(|| {
        block_on(
            happenstance_testkit::projection::rules::commit_advances_the_checkpoint(async || {
                Silent::default()
            }),
        )
    });
    panic::set_hook(previous);

    let payload = outcome.expect_err(
        "a projection fixture that declares no `SECOND_HANDLE` was accepted by \
         `commit_advances_the_checkpoint`. Defaulting the constant moved the \
         enforcement of a MUST off the compiler and onto `must!`; if `must!` \
         does not fire, nothing does, and a fixture that cannot observe PS-1 \
         at all reports a green rule",
    );
    let message = payload
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| payload.downcast_ref::<&str>().copied())
        .unwrap_or("<non-string panic payload>");
    assert!(
        message.contains("SECOND_HANDLE"),
        "the rejection must name the constant the author can change; got: {message}"
    );
    assert!(
        message.contains(
            <Silent as ProjectionFixture>::SECOND_HANDLE
                .reason()
                .unwrap()
        ),
        "the rejection must quote the stated reason, which is CF-18's whole \
         argument — the reason is now the testkit's, and printing it is what \
         tells an author it was written for them; got: {message}"
    );
}
