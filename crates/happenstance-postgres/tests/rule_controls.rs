//! The controls: proof that the rule this adapter exists to pass can fail.
//!
//! # Why a passing suite is not by itself evidence
//!
//! `nothing_below_an_observed_position_appears_later` (CF-13) is the rule the
//! whole visibility mechanism was built for. A green tick against it means
//! nothing unless something can turn it red — CLAUDE.md states the general form:
//! *"A rule that no adapter can fail is decorative. Before adding one, name a
//! plausible wrong implementation it rejects."* Here the wrong implementation is
//! not plausible, it is the one the specification names in ES-10's `Rejects:`
//! and the one this adapter would have been if nobody had done anything.
//!
//! So the naive arm is driven through the same rule, in the same run, from the
//! same call site as the shipped arm, and the two outcomes are compared. Anything
//! less is two harnesses agreeing with themselves.
//!
//! # Why this drives the public `rules::` seam
//!
//! `happenstance_testkit::rules::<name>` is public for exactly this
//! (`crates/happenstance-testkit/src/lib.rs`). The alternative — a row in the
//! testkit's own `mutation_coverage.rs` `REGISTRY` — was considered and
//! rejected, for two structural reasons rather than one preference:
//!
//! 1. **It inverts the dependency direction.** `happenstance-testkit` would have
//!    to depend on `happenstance-postgres` to name its store, and CLAUDE.md's
//!    rule is that everything depends on the contract crate and no adapter is
//!    depended upon. A conformance suite that imports an adapter cannot be the
//!    thing that adapter is measured against.
//! 2. **It drags a live server into `cargo test --workspace`.** The registry
//!    sweep runs in the ordinary gate, on every contributor's machine, and the
//!    promise that the default gate needs no Docker is the one this project
//!    checks hardest.
//!
//! The cost of driving the seam from here instead: this file must do by hand
//! what the registry does by construction — catch the failure, and attribute it.
//! That is what `catch_unwind` below is for.
//!
//! # Running it
//!
//! Swept up by the live job's existing invocation with no workflow edit:
//!
//! ```console
//! cargo test -p happenstance-postgres --all-features -- --ignored --show-output
//! ```

// The whole target is behind the feature, so a default build does not contain
// the naive arm at all -- which is the strongest form of "off by default" and
// what AC-001 asks for. Under `--all-features`, which is what the default gate's
// clippy and test steps and the live job all run, it compiles; its
// server-touching tests are `#[ignore]`d on top of that, so the default gate
// still needs no Docker.
#![cfg(all(feature = "naive-arm", not(target_arch = "wasm32")))]
#![allow(clippy::unwrap_used)]

mod support;

use std::panic::{AssertUnwindSafe, catch_unwind};

use happenstance_testkit::RuleOutcome;
use support::PostgresFixture;

/// What driving one rule against one arm produced.
#[derive(Debug)]
enum Control {
    /// The rule ran and every assertion held.
    Passed,
    /// The rule ran and an assertion failed. Carries the message, so the failure
    /// can be attributed rather than merely counted.
    Failed(String),
    /// The rule declined to run. Never rendered as a pass.
    ///
    /// The two fields are carried rather than discarded because a skip has to be
    /// *readable* as a skip — CF-18's argument is that a rule silently omitted is
    /// indistinguishable from one that passed, and a skip whose reason is thrown
    /// away is the same loss with an extra step. `Debug` is what prints them.
    Skipped {
        capability: &'static str,
        reason: &'static str,
    },
}

impl Control {
    /// The outcome as the column prints it: `pass`, `fail`, or a skip that names
    /// the capability and the fixture's own words.
    ///
    /// Three renderings for three variants, so that a `Skipped` can never be
    /// read as a pass by anyone scanning the log.
    fn column(&self) -> String {
        match self {
            Self::Passed => "pass".to_owned(),
            // The message is carried into the column rather than dropped: a
            // `fail` with no reason sends the reader to a backtrace, and the
            // evaluator this column is written for is reading a log.
            Self::Failed(message) => format!("fail: {message}"),
            Self::Skipped { capability, reason } => format!("SKIP {capability}: {reason}"),
        }
    }
}

/// The panic message, however it was carried.
///
/// `catch_unwind` hands back `Box<dyn Any>`, and a failed `assert_eq!` puts a
/// `String` in it while a bare `panic!("literal")` puts a `&'static str`. Reading
/// only one of the two is how an attribution check quietly stops attributing.
fn panic_message(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| {
            payload
                .downcast_ref::<&'static str>()
                .map(|s| (*s).to_owned())
        })
        .unwrap_or_else(|| "a panic carrying neither a String nor a &str".to_owned())
}

/// Drives `nothing_below_an_observed_position_appears_later` against one arm.
///
/// The rule panics on failure — that is how the suite reports, and it is why the
/// emitted tests carry a message and a backtrace. To *observe* a failure rather
/// than suffer it, this catches the unwind.
///
/// # Why this runs on its own OS thread
///
/// The rule has to be driven to completion **inside** `catch_unwind`, and an
/// `.await` cannot cross that boundary — so its future is blocked on rather than
/// awaited. Blocking needs a runtime, and a runtime cannot be started from
/// inside another one: the first draft of this ran under `#[tokio::test]` and
/// every call came back `Cannot start a runtime from within a runtime`.
///
/// A fresh thread has no ambient runtime, so the one built on it is the only one
/// there. That is the same shape the testkit's own concurrency harness uses, for
/// the same reason.
///
/// The first draft is worth keeping in the record for a second reason: the
/// attribution check below **caught it**. The runtime error was reported as an
/// environment fault and explicitly refused as a visibility finding — the
/// property AC-003 asks for, demonstrated by accident before it was demonstrated
/// on purpose.
fn drive_visibility_rule(arm: Arm) -> Control {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("a broken test environment: could not build a runtime for the control");

        catch_unwind(AssertUnwindSafe(|| {
            runtime.block_on(async {
                happenstance_testkit::rules::nothing_below_an_observed_position_appears_later(
                    || async { arm.open() },
                )
                .await
            })
        }))
    })
    .join()
    .map_or_else(
        |payload| Control::Failed(panic_message(payload.as_ref())),
        |caught| match caught {
            Ok(RuleOutcome::Ran) => Control::Passed,
            Ok(RuleOutcome::Skipped { capability, reason }) => {
                Control::Skipped { capability, reason }
            }
            Err(payload) => Control::Failed(panic_message(payload.as_ref())),
        },
    )
}

/// Which arm of the adapter a control is driving.
#[derive(Debug, Clone, Copy)]
enum Arm {
    /// The one that ships, with the frontier predicate.
    Shipped,
    /// The one that exists to be rejected.
    Naive,
}

impl Arm {
    fn open(self) -> PostgresFixture {
        match self {
            Self::Shipped => PostgresFixture::new(),
            Self::Naive => PostgresFixture::naive(),
        }
    }
}

/// CF-13 does **not** reject the naive arm, and this test asserts that.
///
/// # Read this before treating it as a bug
///
/// The naive arm is genuinely broken. `tests/naive_arm_probe.rs` builds the
/// inversion by hand and watches it: after the fast writer commits a reader sees
/// `[2]`, and after the slow writer commits it sees `[1, 2]` — position 1
/// appearing beneath a position already observed, which is exactly what ES-10's
/// `Rejects:` describes. The shipped arm shows `[]` then `[1, 2]` and never
/// inverts.
///
/// So the defect is real and the rule cannot see it. The reason is the second
/// limitation recorded on the rule itself: this adapter's `append` hands its work
/// to a runtime and therefore advances **off-poll**, and a schedule built out of
/// polls cannot decide when an off-poll transaction commits.
/// `tests/poll_shape.rs` measures the shape — 3 polls at a one-millisecond
/// cadence, 25,096 in a tight loop — and those two numbers agreeing is what it
/// would mean for this adapter to be poll-driven.
///
/// Phase 10 did close the *other* limitation, which was the one ADR-0013 and the
/// specification named: the rule's schedule now drives the fast writer to
/// completion instead of counting polls at it, and `PollPaddedPositionStore` —
/// the same defect one poll wider — is rejected where it used to pass.
///
/// This test is written as an assertion rather than a comment so that the day
/// the rule *does* catch this arm, it fails and says so. That is good news
/// arriving through a red test, which is the only way a control can deliver it.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn cf_13_does_not_yet_reject_the_naive_arm() {
    let control = drive_visibility_rule(Arm::Naive);

    assert!(
        matches!(control, Control::Passed),
        "CF-13 now rejects the naive arm ({control:?}). That is an improvement, not a \
         failure — but it means this control, the rule's own rustdoc and the ledger's \
         recorded limitation are all describing a state that no longer holds. Update \
         them together."
    );
}

/// The shipped arm passes CF-13.
///
/// Weaker evidence than it looks, and the pairing above says why: the rule
/// passes both arms, so this pass is not by itself proof the mechanism works.
/// What proves that is `tests/naive_arm_probe.rs`, which distinguishes them.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn shipped_arm_passes_nothing_below_an_observed_position_appears_later() {
    let control = drive_visibility_rule(Arm::Shipped);
    assert!(
        matches!(control, Control::Passed),
        "the shipped arm did not pass CF-13: {control:?}"
    );
}

/// The outcome column, from one call site in one run.
///
/// Prints `pass`, `fail` or a skip carrying the fixture's own words for each arm,
/// so that an evaluator reading the live job's log sees what the tick had to
/// survive rather than only that it was green. The two arms agreeing here is the
/// finding, not the absence of one.
#[test]
#[ignore = "needs a live Postgres; run with `-- --ignored` (starts a container)"]
fn postgres_rule_outcome_column() {
    let naive = drive_visibility_rule(Arm::Naive);
    let shipped = drive_visibility_rule(Arm::Shipped);

    println!("CF-13 | naive arm   | {}", naive.column());
    println!("CF-13 | shipped arm | {}", shipped.column());

    // A `Skipped` must never be read as a pass. Asserting the variant rather than
    // the rendering is what keeps that true when someone edits `column`.
    for (arm, outcome) in [("naive", &naive), ("shipped", &shipped)] {
        assert!(
            !matches!(outcome, Control::Skipped { .. }),
            "the {arm} arm SKIPPED CF-13, which is not a pass and must not be read as one: \
             {outcome:?}"
        );
    }
}

/// The naive arm is unreachable without its feature.
///
/// A compile-time claim rather than a runtime one, which is why it is a `const`
/// block: clippy correctly points out that the assertion has a constant value,
/// and the fix is to make that explicit rather than to assert it at run time.
/// The whole target is behind `#![cfg(feature = "naive-arm")]`, so this either
/// compiles — proving the feature is on — or does not exist at all.
///
/// What it actually buys: `cargo hack --feature-powerset` compiles the
/// combination that includes the feature, and this is the item that makes that
/// combination contain something.
#[test]
fn naive_arm_is_reachable_only_under_its_feature() {
    const {
        assert!(
            cfg!(feature = "naive-arm"),
            "this target is compiled only with `naive-arm` on"
        );
    }
}
