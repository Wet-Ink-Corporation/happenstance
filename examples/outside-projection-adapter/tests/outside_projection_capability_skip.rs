//! Declining a capability without opting out of the bar.
//!
//! This store genuinely cannot do one thing a rule asks for, and the honest way
//! to say so is a declined `Capability` carrying its own reason. What matters to
//! an outside author is that saying so does **not** make the rule disappear: it
//! is still emitted, it still returns a value, and the value carries the words
//! this fixture wrote.
//!
//! The assertions below are on the **returned `RuleOutcome`**, never on captured
//! stdout. The `SKIP …` line the harness prints is legibility — reachable under
//! the gate's `--show-output` and worth having — but a promise checked only by
//! stdout is a promise checked by nobody.

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

use happenstance_testkit::{ProjectionFixture, RuleOutcome, projection::rules};
use outside_projection_adapter::OutsideProjectionStore;
use support::OutsideFixture;

async fn open() -> OutsideFixture {
    OutsideFixture::new()
}

/// The declined capability: the rule runs, returns a skip, and the skip carries
/// this fixture's own sentence.
#[tokio::test]
async fn a_declined_capability_returns_a_skip_carrying_this_fixtures_own_reason() {
    // Read off the fixture's own constant rather than repeated as a literal. A
    // literal here would let this test go on passing while the fixture told CI
    // something else entirely.
    let stated = <OutsideFixture as ProjectionFixture>::RESET_REFUSAL
        .reason()
        .expect("this fixture declines RESET_REFUSAL, so the capability carries a reason");

    let outcome = rules::refused_reset_changes_nothing(open).await;

    assert_eq!(
        outcome,
        RuleOutcome::Skipped {
            capability: "RESET_REFUSAL",
            reason: stated,
        },
        "a declined capability must come back as a reported skip naming the \
         constant an author can go and change, and carrying their own reason"
    );

    // And the reason really is the store's account of itself, not a sentence
    // invented in the fixture.
    assert_eq!(stated, OutsideProjectionStore::NO_RESET_PROTECTION);

    // Legibility, not the check: the one line a human reads in the CI log.
    let line = outcome
        .skip_line("refused_reset_changes_nothing")
        .expect("a skipped rule has a line worth printing");
    assert!(line.contains("refused_reset_changes_nothing"));
    assert!(line.contains("RESET_REFUSAL"));
    assert!(line.contains(stated));
}

/// The other side of the same contract, and the reason the skip above is a
/// report rather than an opt-out: a capability this fixture **declares** gets
/// the rule run for real.
#[tokio::test]
async fn a_declared_capability_runs_the_rule_it_gates() {
    assert_eq!(
        rules::failed_commit_leaves_both_unchanged(open).await,
        RuleOutcome::Ran,
        "this fixture declares COMMIT_FAULT supported, so the rule it gates must \
         actually drive the store rather than report a skip"
    );
}

/// A rule that needs no capability at all still returns `Ran`, so nothing above
/// is an artefact of how this fixture was written.
#[tokio::test]
async fn an_ungated_rule_runs() {
    assert_eq!(
        rules::commit_advances_the_checkpoint(open).await,
        RuleOutcome::Ran
    );
}
