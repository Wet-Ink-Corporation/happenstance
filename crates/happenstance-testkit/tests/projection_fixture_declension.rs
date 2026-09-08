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

/// The silence this file permits is now **reported**, on every default run.
///
/// This module's own documentation has said since it was written that a default
/// reason is *"testkit prose printed in an adapter's CI log as though it were
/// the adapter's own account of itself, and that cost is real"*. It was argued
/// at length and enforced by nothing. CF-18's `Rejects:` paragraph asked for the
/// enforcement in as many words — *"requiring a non-empty reason string
/// alongside each `false` puts the trade in the log where a reviewer and a user
/// of the adapter can both see it"* — and until `0.2.0` the only thing standing
/// there was `Capability::declined`'s empty-string assertion, which no fixture
/// can trip because no fixture writes an empty reason. It writes nothing at all.
///
/// [`Silent`] is therefore the wrong implementation
/// `assert_projection_declensions_are_stated` exists to reject, and this is the
/// test that says so. **It is what stops the check being decorative**: the
/// emitted rule is green against all eleven fixtures in this workspace, so
/// without a fixture that fails it there would be no evidence it can fail at
/// all.
///
/// Note what it does not assert: that the reason is *good*. A fixture writing
/// `"n/a"` passes, here and everywhere. Nothing mechanical distinguishes a
/// considered account from a plausible one, and a check that tried would be a
/// style gate wearing a conformance rule's clothes.
#[test]
fn a_fixture_that_says_nothing_is_rejected_by_the_emitted_check() {
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(|| {
        happenstance_testkit::__private::assert_projection_declensions_are_stated(async || {
            Silent::default()
        });
    });
    panic::set_hook(previous);

    let payload = outcome.expect_err(
        "a fixture that declares no capability at all was accepted by \
         `assert_declensions_are_stated`. Every reason it prints is this \
         crate's prose, so the check has nothing left to reject and CF-18's \
         reporting obligation is met by a sentence about a store nobody wrote \
         it for",
    );
    let message = payload
        .downcast_ref::<String>()
        .map_or("", String::as_str)
        .to_owned();

    // All three, in one message. A fixture that inherited one declension has
    // usually inherited the rest, and reporting them a build at a time turns one
    // edit into three red runs.
    for capability in ["SECOND_HANDLE", "RESET_REFUSAL", "COMMIT_FAULT"] {
        assert!(
            message.contains(capability),
            "the refusal names no `{capability}`, so an author fixing it learns \
             about one capability per run: {message}"
        );
    }
}

/// A fixture that names the constant it agrees with is accepted.
///
/// The positive control, and it is not ceremony: without it the test above
/// passes against a check that rejects *every* fixture, which is a bar nobody
/// can clear rather than a bar. It also pins the escape hatch the refusal
/// message offers — *"if the inherited wording is exactly right, name the
/// `UNSTATED_*` constant"* — as a lie, deliberately, because naming the constant
/// is exactly what the check rejects.
///
/// So the hatch is a paraphrase, and this fixture takes it: it says the same
/// thing in its own words, about its own store. That is the whole distinction
/// the check draws, and the reason it can be drawn mechanically at all.
#[test]
fn a_fixture_that_states_its_own_reasons_is_accepted() {
    #[derive(Debug, Default)]
    struct Stated(MemoryProjectionFixture);

    impl ProjectionFixture for Stated {
        type Store = MemoryProjectionHandle;

        const SECOND_HANDLE: happenstance_testkit::Capability =
            happenstance_testkit::Capability::SUPPORTED;

        const RESET_REFUSAL: happenstance_testkit::Capability =
            happenstance_testkit::Capability::declined(
                "this store holds no protection policy, so there is no projection \
                 it could decline to reset",
            );

        const COMMIT_FAULT: happenstance_testkit::Capability =
            happenstance_testkit::Capability::declined(
                "this store applies the read model and the checkpoint under one \
                 write lock, so it has no write that can be made to fail",
            );

        fn connect(&self) -> impl Future<Output = Self::Store> {
            self.0.connect()
        }
    }

    happenstance_testkit::__private::assert_projection_declensions_are_stated(async || {
        Stated::default()
    });
}
