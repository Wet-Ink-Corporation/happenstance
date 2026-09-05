//! The other half of "the suite passed": a store this author wrote wrong, and
//! the rule that catches it.
//!
//! A suite no adapter can fail is decorative, and an outside author has no way
//! to know which kind they are holding unless they try. So this file drives the
//! *same* published rule functions the macro drives — `projection::rules`, which
//! the module documentation says outright may be called directly — against a
//! store that commits the checkpoint and drops the read-model write.
//!
//! Two of the three tests are the point. `commit_advances_the_checkpoint`
//! **passes** against the wrong store: watching only the checkpoint certifies a
//! projection whose rows never arrived, which is precisely the trap of mistaking
//! "it compiles" for "it is correct".
//! `commit_is_atomic_with_the_read_model` is the rule that convicts it, by name.
//!
//! The failure is caught rather than emitted, because a conformance run over a
//! deliberately wrong store must not turn this workspace's own gate red. The
//! panic is what the suite reports a failure with; catching it and asserting on
//! it is reading the same signal a red CI run would have shown.

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

use std::future::Future;
use std::panic::{self, AssertUnwindSafe};

use happenstance_testkit::{RuleOutcome, block_on, projection::rules};
use support::{CheckpointOnlyFixture, OutsideFixture};

async fn open_wrong() -> CheckpointOnlyFixture {
    CheckpointOnlyFixture::new()
}

async fn open_right() -> OutsideFixture {
    OutsideFixture::new()
}

/// Runs one rule to completion and returns its failure message, or `None` if it
/// did not fail.
///
/// `block_on` is the testkit's own runtime-free driver, published for exactly
/// this: a caller who needs to run a rule outside an emitter. The panic hook is
/// swapped for the duration so that an *expected* failure does not print a
/// backtrace into a green run's output and read as a real one.
fn failure_of<F>(rule: impl FnOnce() -> F) -> Option<String>
where
    F: Future<Output = RuleOutcome>,
{
    let previous = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| block_on(rule())));
    panic::set_hook(previous);

    match outcome {
        Ok(_) => None,
        Err(payload) => Some(
            payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_else(|| "<non-string panic payload>".to_owned()),
        ),
    }
}

/// The trap, stated as a passing test so it cannot be waved away: a store that
/// never writes a row satisfies the rule that watches only the checkpoint.
#[test]
fn the_checkpoint_only_store_passes_the_rule_that_watches_only_the_checkpoint() {
    assert_eq!(
        failure_of(|| rules::commit_advances_the_checkpoint(open_wrong)),
        None,
        "a store that drops every read-model write still moves its checkpoint, \
         so this rule cannot be the one that catches it — if it starts failing \
         here, the demonstration below is no longer about what it claims"
    );
}

/// The discrimination, by name.
#[test]
fn commit_is_atomic_with_the_read_model_fails_the_checkpoint_only_store() {
    let message = failure_of(|| rules::commit_is_atomic_with_the_read_model(open_wrong)).expect(
        "`commit_is_atomic_with_the_read_model` must reject a store that commits \
         the checkpoint and drops the read-model write: if it passes, the port is \
         not frozen and the suite is decorative",
    );

    assert!(
        !message.trim().is_empty(),
        "the rule failed without saying anything, which is a failure an adapter \
         author cannot act on"
    );
}

/// The control. The same rule, the same entry point, the conformant store — so
/// the red above is a fact about the store rather than about the harness.
#[test]
fn commit_is_atomic_with_the_read_model_passes_the_conformant_store() {
    assert_eq!(
        failure_of(|| rules::commit_is_atomic_with_the_read_model(open_right)),
        None,
        "the conformant store must pass the rule its wrong sibling fails, or the \
         demonstration proves nothing about either"
    );
}
