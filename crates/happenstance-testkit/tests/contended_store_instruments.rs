//! The busy store, and what the concurrency rules do with it.
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
//! store returned. When this file was written the port offered a caller exactly
//! one classifier, [`AppendError::is_condition_violated`], which answers
//! `false` for a busy store exactly as it does for a broken one, so the
//! per-error arm could not be built in the testkit without a contract change
//! first.
//!
//! **The contract change has been made, and the per-error arm was chosen**
//! (ADR-0077, ES-43). `contend_next` now refuses through `AppendError::Busy`,
//! and `a_retry_loop_gated_on_the_dcb_signal_never_retries_a_busy_store` shows
//! both halves: a loop that asks only the DCB question still stalls, and one
//! that also asks [`AppendError::is_busy`] gets through.
//!
//! # What this file used to pin, and what it pins now
//!
//! It was written to build the instrument and to **pin the behaviour of the
//! time**, including the part that was arguably wrong. A test named
//! `two_rules_reject_a_store_that_is_merely_contended` asserted that two rules
//! rejected a conformant-but-busy store, and its own documentation said it
//! would be *deleted or inverted* by whichever remedy the open question
//! settled on.
//!
//! ADR-0077 settled it, and the test is inverted.
//! `every_concurrency_rule_accepts_a_store_that_is_merely_busy` drives every
//! rule of the family against a store that refuses some contenders as `Busy`,
//! and requires each rule to pass. It also checks that the refusals really
//! happened inside the rule, since a pass is worth nothing if they did not.
//! This is the deterministic arm of `a_busy_append_left_nothing_behind` too:
//! that rule only observes the `Busy` answers a store happens to give, and here
//! the answers are certain. The floors have a test of their own,
//! `a_store_that_is_busy_for_every_contender_has_made_no_progress`, which
//! drives every rule the same way, because a rule that accepts `Busy` without
//! a floor would be measuring luck.
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
///
/// Cloning it clones the prototype and the connect counter, not the store, so a
/// clone is the same fixture. That is what lets a test hand a rule one clone and
/// then look at the store afterwards through another.
#[derive(Clone)]
struct ContendedFixture {
    /// The one prototype every handle is cloned from.
    prototype: SendFaultyStore<MemoryHandle>,
    /// How many handles have been opened, for [`Self::refusing_the_race`].
    connects: Arc<AtomicUsize>,
    /// Refusals armed at the second `connect`, if any.
    deferred: u32,
}

impl ContendedFixture {
    /// A fixture whose next `n` appends are refused as contended.
    fn refusing(n: u32) -> Self {
        let handle = block_on(MemoryFixture::new().connect());
        Self {
            prototype: SendFaultyStore::new(handle).contend_next(n),
            connects: Arc::new(AtomicUsize::new(0)),
            deferred: 0,
        }
    }

    /// A fixture that refuses `n` appends **of the race**, not of its setup.
    ///
    /// Several rules append a setup batch through their first handle before any
    /// contender exists, and an arming spent there makes the rule panic in its
    /// setup instead of meeting a busy contender. Every rule opens its setup
    /// handle first, if it has one, and then opens all its contenders before it
    /// starts the race. So arming at the **second** `connect` lands after the
    /// setup and before any contender has appended, whichever rule runs.
    fn refusing_the_race(n: u32) -> Self {
        Self {
            deferred: n,
            ..Self::refusing(0)
        }
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
        if self.connects.fetch_add(1, Ordering::Relaxed) == 1 && self.deferred > 0 {
            // `contend_next` writes the shared arming behind the prototype's
            // `Arc`, so arming a clone arms every handle.
            drop(self.prototype.clone().contend_next(self.deferred));
        }
        self.prototype.clone()
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
        matches!(refused, Err(AppendError::Busy(FaultyStoreError::Contended))),
        "a contended refusal must arrive as AppendError::Busy, the channel ES-43 \
         gives a refusal that wrote nothing. Got: {refused:?}"
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
            if let Err(AppendError::Busy(FaultyStoreError::Contended)) =
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

/// A caller that retries on the busy signal gets through; one that retries
/// only on the DCB signal does not.
///
/// `is_condition_violated` answers `false` for a busy store, because no
/// condition was evaluated, so a retry loop written against it alone stalls on
/// a store that was merely contended. `is_busy` is the question that loop was
/// missing.
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

    // Gated on the busy signal as well: one retry, and it lands.
    let store = SendFaultyStore::new(MemoryEventStore::new()).contend_next(1);
    let mut attempts = 0_u32;
    let mut outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    while outcome.as_ref().is_err_and(AppendError::is_busy) {
        attempts += 1;
        outcome = SendEventStore::append(&store, &[event("Appended")], None).await;
    }
    assert_eq!(attempts, 1);
    assert!(outcome.is_ok());
}

// ---------------------------------------------------------------------------
// What the suite does with it
// ---------------------------------------------------------------------------

/// How many contenders each rule below is made to refuse as busy.
///
/// Three, which leaves a winner behind in every rule, including the one with
/// the fewest contenders per boundary.
const REFUSED: u32 = 3;

/// **Every concurrency rule accepts a store that is merely busy.**
///
/// This inverts the test this file was written with,
/// `two_rules_reject_a_store_that_is_merely_contended`. That test pinned
/// `positions_are_unique_under_concurrent_appends` failing on *"every contender
/// must commit"* and `append_returns_the_callers_own_last_position` failing on
/// *"did not commit an unconditional append"*. Both were right about the rules
/// as they stood, and both became wrong when ES-43 made `Busy` an answer rather
/// than a failure (ADR-0077).
///
/// Every rule is driven, not only the ones that were pinned, because the count
/// of three affected rules had missed two:
/// `k_disjoint_boundaries_never_conflict` and
/// `a_concurrent_reader_never_sees_a_partial_batch` also failed on any refusal.
/// The enumeration is the family's own, so a new rule is driven here as soon as
/// it exists.
///
/// Each run checks that the arming was **spent inside the rule**. The check is
/// a probe append after the rule has returned: if the refusals were still armed,
/// the probe would be refused. Without it, a pass would also be what a fixture
/// that never armed anything produces.
#[test]
fn every_concurrency_rule_accepts_a_store_that_is_merely_busy() {
    macro_rules! drive {
        ($($rule:ident),* $(,)?) => {
            $(
                let fixture = ContendedFixture::refusing_the_race(REFUSED);
                let handed = fixture.clone();
                let outcome = block_on(happenstance_testkit::concurrency::rules::$rule(
                    || async { handed.clone() },
                ));
                assert_eq!(
                    outcome,
                    RuleOutcome::Ran,
                    "`{}` must run, and pass, against a store that refused {REFUSED} \
                     contenders as `Busy`",
                    stringify!($rule)
                );

                let probe = block_on(fixture.connect());
                assert!(
                    block_on(SendEventStore::append(&probe, &[event("Probe")], None)).is_ok(),
                    "`{}` passed, but the arming was not spent inside it, so the \
                     pass says nothing about a busy store",
                    stringify!($rule)
                );
            )*
        };
    }

    happenstance_testkit::for_each_concurrency_rule!(drive);
}

/// **The floor: a store that is busy for every contender has made no progress.**
///
/// The other half of the inversion above, and the half that keeps it from being
/// a tolerance. If `Busy` were accepted with no floor, a store that refused
/// everybody would pass every rule, and the suite would be measuring luck. Each
/// rule therefore requires at least one commit. That requirement is structural
/// and not a share: it names no fraction, so CF-34's `Rejects:` is not engaged.
///
/// Every rule of the family is driven, through the family's own enumeration,
/// because each carries its own floor and each floor is what a rule's
/// documentation cites as the thing that keeps it from passing vacuously —
/// `a_busy_append_left_nothing_behind`'s anchor has nothing to read without
/// one. Deleting any one of them must turn this test red, and a test that
/// drove one rule would only hold one.
///
/// Every contender is refused, four times over: more arming than any rule has
/// appends to spend it on, so no rule can reach a commit. Each panic message is
/// matched against the floor it is expected to fail, rather than being checked
/// with a bare `#[should_panic]`, which ADR-0010 rejects by name. Two phrasings
/// are accepted because the family has two shapes of contender:
/// `a_concurrent_reader_never_sees_a_partial_batch` races batches rather than
/// deciders, and says so.
#[test]
fn a_store_that_is_busy_for_every_contender_has_made_no_progress() {
    let everyone = u32::try_from(happenstance_testkit::concurrency::CONTENDERS * 4).unwrap();

    macro_rules! drive {
        ($($rule:ident),* $(,)?) => {
            $(
                let outcome = catch_unwind(AssertUnwindSafe(|| {
                    block_on(happenstance_testkit::concurrency::rules::$rule(
                        || async { ContendedFixture::refusing_the_race(everyone) },
                    ))
                }));
                let Err(payload) = outcome else {
                    panic!(
                        "`{}` passed against a store that refused every contender \
                         as busy and committed nothing; its floor must say so",
                        stringify!($rule)
                    );
                };
                let message = panic_message(&payload);
                assert!(
                    message.contains("no contender committed")
                        || message.contains("no batch committed"),
                    "`{}` must fail on its floor rather than for some unrelated \
                     reason. Got: {message}",
                    stringify!($rule)
                );
            )*
        };
    }

    happenstance_testkit::for_each_concurrency_rule!(drive);
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
