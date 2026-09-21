//! The busy store, and the two conformance rules that cannot tell it from a
//! broken one.
//!
//! Every store this workspace can test against is always available.
//! `MemoryEventStore` is a `Vec` behind an `RwLock` and never refuses a writer;
//! `FaultyStore`'s two older arms produce a *rejection* (`ConditionViolated`)
//! and a *read* failure, neither of which is the shape in question. So until
//! [`SendFaultyStore::contend_next`] existed, **no implementation anywhere in
//! this tree could produce a transient append refusal**, and the question of
//! what the suite does with one had no executable answer at all.
//!
//! That gap is not hypothetical. `experiments/busy-timeout-margin` recorded
//! `busy > 0` at the shipped `CONTENDERS = 64` — the first nonzero busy count
//! in this tree — which is ADR-0022 §11's own re-open condition firing
//! (`references/adr/0022-append-condition-strategy.md:614-616`). The open
//! questions that own the consequence are
//! `.kb/open-questions/adr-0022-falsifiers-have-fired.md` and
//! `.kb/open-questions/no-fixture-tolerance-for-transient-contention.md`, and
//! both name this instrument as the thing they are gated on.
//!
//! # How it discriminates the two proposed remedies
//!
//! It does not merely *gate* them, it tells them apart. A **per-fixture
//! capability** would relax a rule on what the fixture declared, and
//! `ContendedFixture` below is a fixture that could declare one. A **per-error
//! channel** — `AppendError::Busy` in the contract — would relax it on what the
//! store returned, and that arm has a cost this file makes visible rather than
//! argues: `a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store`
//! shows that the port offers a caller exactly one classifier today,
//! [`AppendError::is_condition_violated`], and it answers `false` for a busy
//! store exactly as it does for a broken one. So the per-error arm cannot be
//! built in the testkit at all; it needs a contract change first, and the
//! per-fixture arm does not. That asymmetry is a fact about the two designs,
//! not a preference between them.
//!
//! # What this file does and does not decide
//!
//! It builds the instrument and **pins today's behaviour**, including the part
//! that is arguably wrong: `two_rules_reject_a_store_that_is_merely_contended`
//! asserts that the rules reject a conformant-but-busy store. That test is
//! written to be *deleted or inverted* by whichever remedy the open question
//! settles on — a fixture-side tolerance, `AppendError::Busy` in the contract,
//! or an explicit ratification that a busy store is non-conformant. Nothing
//! here takes that decision; it makes it measurable, which is the step that was
//! missing.
//!
//! Native only: the harness is `#[tokio::test]` and the concurrency
//! demonstration needs [`std::panic::catch_unwind`], which cannot catch on a
//! target with no unwinder — the gate `tests/mutation_coverage.rs` carries for
//! the same reason.

#![cfg(not(target_arch = "wasm32"))]
#![allow(clippy::unwrap_used)]

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use happenstance_core::{
    AppendError, Event, MemoryEventStore, Query, ReadOptions, SendEventStore, collect,
};
use happenstance_testkit::fixtures::{MemoryFixture, MemoryHandle};
use happenstance_testkit::{
    Capability, FaultyStoreError, Fixture, RuleOutcome, SendFaultyStore, block_on,
};

// ---------------------------------------------------------------------------
// Arrangement
// ---------------------------------------------------------------------------

fn event(event_type: &str) -> Event {
    Event::new(event_type, &b"{}"[..]).unwrap()
}

/// How many contenders the demonstrations race.
///
/// Deliberately **not** `happenstance_testkit::concurrency::CONTENDERS`. That
/// constant is the subject of an unsettled question about what it should be,
/// and a test that moved with it would stop meaning what it was written to
/// mean the moment somebody answered that question. Sixteen is enough for a
/// race and small enough to be quick.
const RACERS: usize = 16;

/// One `MemoryEventStore` behind a wrapper that can be told to be busy.
///
/// It delegates isolation to [`MemoryFixture`] for the reason
/// `tests/faulty_store_conformance.rs`'s fixture does, and holds **one**
/// prototype cloned per `connect` rather than wrapping a fresh handle each
/// time: `SendFaultyStore`'s arming lives behind an `Arc`, so one prototype is
/// one fixture, one arming, any number of handles. A fixture that wrapped per
/// `connect` would give every contender its own counter, and an arming of `n`
/// would refuse `n` callers *each* — which is not a contended store, it is
/// `RACERS * n` broken ones.
struct ContendedFixture(SendFaultyStore<MemoryHandle>);

impl ContendedFixture {
    /// A fixture whose next `n` appends are refused as contended.
    fn refusing(n: u32) -> Self {
        let handle = block_on(MemoryFixture::new().connect());
        Self(SendFaultyStore::new(handle).contend_next(n))
    }
}

impl Fixture for ContendedFixture {
    type Store = SendFaultyStore<MemoryHandle>;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;

    const REOPEN: Capability = Capability::declined(
        "the wrapped store is a MemoryEventStore — a Vec behind an RwLock — so \
         there is no durable medium to reopen over, and FaultyStore adds none",
    );

    const MID_BATCH_FAULT: Capability = Capability::declined(
        "FaultyStore wraps a store from outside and fails whole calls; a \
         contended refusal never reaches the inner store at all, so it cannot \
         land between two rows of a batch",
    );

    const READ_FAULT: Capability = Capability::SUPPORTED;

    async fn connect(&self) -> Self::Store {
        self.0.clone()
    }
}

// ---------------------------------------------------------------------------
// The instrument itself
// ---------------------------------------------------------------------------

/// The refusal arrives on the store channel, not as a rejection.
///
/// The conformant sibling is the append that follows: the same call, the same
/// store, succeeding once the arming is spent. Without it this test would pass
/// against a wrapper that had simply broken.
#[tokio::test]
async fn a_contended_refusal_is_not_a_violated_condition() {
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(1);

    let refused = SendEventStore::append(&store, &[event("Appended")], None).await;

    assert!(
        matches!(
            refused,
            Err(AppendError::Store(FaultyStoreError::Contended))
        ),
        "a contended refusal must arrive as AppendError::Store, because that is \
         the channel a real busy store uses and the one the suite cannot \
         classify. Got: {refused:?}"
    );
    assert!(
        !refused.unwrap_err().is_condition_violated(),
        "a contended store evaluated no condition — it never took the lock — so \
         claiming a violation would be the fixture reporting an answer it could \
         not have computed"
    );

    // The conformant sibling: transient means the retry lands.
    assert!(
        SendEventStore::append(&store, &[event("Appended")], None)
            .await
            .is_ok(),
        "the arming is spent, so the wrapper must delegate and the append must \
         land; a permanent refusal would be a poisoned fixture rather than a \
         contended one"
    );
}

/// A refused append writes nothing.
///
/// The failure this rejects is a wrapper that delegates first and reports
/// contention afterwards, which would be a fixture lying about the atomicity
/// the port promises — and would silently corrupt every count in this file.
#[tokio::test]
async fn a_refused_append_never_reaches_the_inner_store() {
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(1);

    let _refused = SendEventStore::append(&store, &[event("Appended")], None).await;

    let landed = collect(SendEventStore::read(
        &store,
        &Query::all(),
        ReadOptions::new(),
    ))
    .await
    .unwrap();
    assert!(
        landed.is_empty(),
        "a refused append must not have written; the store holds {landed:?}"
    );
    assert_eq!(
        SendEventStore::head(&store).await.unwrap(),
        None,
        "and it must not have allocated a position"
    );
}

/// `Contended` and `Injected` are different answers to different questions.
///
/// They are both this wrapper's own, so both report no `source`; what separates
/// them is meaning, and a caller that collapsed them would retry a fixture that
/// was broken on purpose.
#[tokio::test]
async fn contended_is_distinguishable_from_injected() {
    let contended: FaultyStoreError<core::convert::Infallible> = FaultyStoreError::Contended;
    let injected: FaultyStoreError<core::convert::Infallible> = FaultyStoreError::Injected;

    assert_ne!(contended, injected);
    assert!(
        contended.to_string().contains("retry"),
        "the transient one should say so: {contended}"
    );
    assert!(
        !injected.to_string().contains("retry"),
        "and the hard one should not: {injected}"
    );
}

/// Arming zero arms nothing, matching the older arms' contract.
#[tokio::test]
async fn arming_zero_contentions_arms_nothing() {
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(0);

    assert!(
        SendEventStore::append(&store, &[event("Appended")], None)
            .await
            .is_ok()
    );
}

/// Exactly `min(n, racers)` callers are refused, however the threads interleave.
///
/// This is the property that lets a rule assert on the count without reading a
/// clock, which CF-33 forbids. The wrong implementation it rejects is a
/// saturating or unsynchronised counter: one that wrapped below zero, or that
/// read-then-wrote without `fetch_update`, would refuse a number of callers
/// that varied run to run and would make every count in this file a coin flip.
#[tokio::test(flavor = "multi_thread")]
async fn the_refusal_count_is_exact_under_a_race() {
    const ARMED: u32 = 3;

    let fixture = ContendedFixture::refusing(ARMED);
    let refusals = Arc::new(AtomicUsize::new(0));

    let mut joins = Vec::with_capacity(RACERS);
    for _ in 0..RACERS {
        let store = fixture.connect().await;
        let refusals = Arc::clone(&refusals);
        joins.push(tokio::spawn(async move {
            if let Err(AppendError::Store(FaultyStoreError::Contended)) =
                SendEventStore::append(&store, &[event("Appended")], None).await
            {
                refusals.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }
    for join in joins {
        join.await.unwrap();
    }

    assert_eq!(
        refusals.load(Ordering::Relaxed),
        ARMED as usize,
        "an arming of {ARMED} against {RACERS} racers must refuse exactly \
         {ARMED} of them"
    );

    let observer = fixture.connect().await;
    let landed = collect(SendEventStore::read(
        &observer,
        &Query::all(),
        ReadOptions::new(),
    ))
    .await
    .unwrap();
    assert_eq!(
        landed.len(),
        RACERS - ARMED as usize,
        "and every caller it did not refuse must have committed"
    );
}

/// A caller that retries on the store channel gets through; one that retries
/// only on the DCB signal does not.
///
/// The second half is the point. `is_condition_violated` is the discrimination
/// the port offers a caller today, and it answers `false` for a busy store — so
/// a retry loop written against it stalls on a store that was merely contended.
#[tokio::test]
async fn a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store() {
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(1);

    // Gated on the DCB signal: sees `false`, gives up, never retries.
    let mut attempts = 0_u32;
    let mut outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    while let Err(err) = &outcome {
        if !err.is_condition_violated() {
            break;
        }
        attempts += 1;
        outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    }
    assert_eq!(attempts, 0, "the loop must not have retried");
    assert!(
        outcome.is_err(),
        "so it must still be holding the refusal it could not classify"
    );

    // Gated on the store channel instead: one retry, and it lands.
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(1);
    let mut attempts = 0_u32;
    let mut outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    while matches!(
        outcome,
        Err(AppendError::Store(FaultyStoreError::Contended))
    ) {
        attempts += 1;
        outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    }
    assert_eq!(attempts, 1);
    assert!(outcome.is_ok());
}

// ---------------------------------------------------------------------------
// What the suite does with it — today's behaviour, pinned
// ---------------------------------------------------------------------------

/// **Today, two concurrency rules reject a store that is merely contended.**
///
/// This is the named wrong implementation both open questions said did not
/// exist. It is not asserted with a bare `#[should_panic]`, which records only
/// that *something* failed — the shape ADR-0010 rejects by name: each rule is
/// driven separately and its panic message is matched against the assertion it
/// was expected to fail.
///
/// `positions_are_unique_under_concurrent_appends` asserts
/// `committed.len() == CONTENDERS`, and
/// `append_returns_the_callers_own_last_position` panics per contender on any
/// non-commit. One transient refusal is therefore indistinguishable, to both,
/// from a store that lost a write.
///
/// **When the open question is settled this test changes or goes.** If a
/// tolerance is bought, invert it; if a busy store is ratified as
/// non-conformant, keep it as the record of that decision.
#[test]
fn two_rules_reject_a_store_that_is_merely_contended() {
    // `positions_are_unique_under_concurrent_appends` fails on the *count*:
    // `committed.len() == CONTENDERS`.
    let positions = catch_unwind(AssertUnwindSafe(|| {
        block_on(
            happenstance_testkit::concurrency::rules::positions_are_unique_under_concurrent_appends(
                || async { ContendedFixture::refusing(1) },
            ),
        )
    }));
    let message = panic_message(&positions.expect_err(
        "a store that refuses one contender must, today, fail this rule: it \
         requires every contender to commit",
    ));
    assert!(
        message.contains("every contender must commit"),
        "it must fail on the all-commit assertion rather than for some \
         unrelated reason. Got: {message}"
    );

    // `append_returns_the_callers_own_last_position` fails per contender, on a
    // different assertion — which is why a tolerance is a change to what these
    // rules assert rather than one new arm on a private enum.
    let returned = catch_unwind(AssertUnwindSafe(|| {
        block_on(
            happenstance_testkit::concurrency::rules::append_returns_the_callers_own_last_position(
                || async { ContendedFixture::refusing(1) },
            ),
        )
    }));
    let message = panic_message(
        &returned.expect_err("and the same store must fail this one too, by a different route"),
    );
    assert!(
        message.contains("did not commit an unconditional append"),
        "it must fail on the per-contender commit assertion. Got: {message}"
    );
}

/// The conformant sibling: with nothing armed, the same fixture passes.
///
/// Without this, the test above would pass against a fixture that was broken in
/// some way having nothing to do with contention.
#[test]
fn the_same_fixture_passes_the_rule_when_it_is_not_contended() {
    let outcome = block_on(
        happenstance_testkit::concurrency::rules::positions_are_unique_under_concurrent_appends(
            || async { ContendedFixture::refusing(0) },
        ),
    );

    // Asserted rather than discarded: a rule that *skipped* would also not
    // panic, and a silent skip here would make the test above look like it had
    // isolated contention when it had only found a declined capability.
    assert_eq!(
        outcome,
        RuleOutcome::Ran,
        "the unarmed fixture must actually run the rule, not skip it"
    );
}

/// Extracts a panic payload as a string, for the message assertions above.
fn panic_message(panic: &Box<dyn core::any::Any + Send>) -> String {
    panic.downcast_ref::<String>().map_or_else(
        || {
            panic.downcast_ref::<&str>().map_or_else(
                || "<non-string panic payload>".to_owned(),
                |s| (*s).to_owned(),
            )
        },
        Clone::clone,
    )
}
