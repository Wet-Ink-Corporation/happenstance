---
id: kb-reference-nested-block-on-lost-wakeup-001
title: park carries one token per thread, so a nested block_on in the testkit loses its wakeup
kind: reference
status: accepted
authority_tier: note
summary: >-
  The deterministic reproduction taken 2026-09-03 in experiments/busy-timeout-margin/tests/lost_wakeup.rs
  of a second way a conformance run can stop and name no rule. The testkit's own executor,
  crates/happenstance-testkit/src/registry.rs:338-342, drives a rule by polling and calling
  std::thread::park on Poll::Pending, with no notified flag of its own. std's park and unpark
  carry a single token per thread, so an unpark delivered while the thread is not parked is
  coalesced rather than queued: a second block_on nested inside a rule already driven by one —
  the shape at crates/happenstance-testkit/src/concurrency.rs:890 calling through to :980 — can
  consume the token the outer loop was waiting for. Measured over four runs: the baseline
  completes, the nested case without a collision completes, and the nested case with a collision
  hangs past ten seconds. This matters beyond the one call site because CF-33 is [FROZEN] and
  forbids a conformance rule a clock, a watchdog or an elapsed-time assertion, so a hang produces
  a stopped CI job that names no rule at all. spec/SPECIFICATION.md:4302-4322 already records one
  mechanism with that signature — an adapter holding an exclusive resource across its append's
  suspension point, where the read blocks on what the suspended append still holds. This is a
  second, and it lives in the suite rather than in an adapter.
  crates/happenstance-sqlite/tests/concurrency.rs:45-47 currently instructs the reader that a
  hang is evidence about ADR-0022's busy-timeout paragraph and should be escalated there, which
  was sound while that was the sole candidate and is a dated statement now.
depends_on: []
related:
  - kb-decision-0010
  - kb-decision-0022
  - kb-playbook-cold-future-hand-polling-001
  - kb-reference-busy-timeout-margin-001
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/tests/lost_wakeup.rs
  - crates/happenstance-testkit/src/registry.rs
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-sqlite/tests/concurrency.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
---

# park carries one token per thread, so a nested block_on in the testkit loses its wakeup

## What this is a pointer to

The reproduction — three deterministic cases (baseline, nested without a collision, nested with a
forced collision) built as a standalone test file rather than a change to the testkit — lives at
`experiments/busy-timeout-margin/tests/lost_wakeup.rs`, outside the workspace and outside the gate.
This atom is the citable summary of what it demonstrated: a real hazard in the testkit's own
executor, found by reasoning about a primitive `happenstance_testkit::registry::block_on` already
uses, not by observing a failure in CI.

## The mechanism

`registry.rs:338-342` implements `block_on` as a hand-rolled poll loop: on `Poll::Pending` it calls
`std::thread::park()` directly, with no flag of its own recording whether a wakeup already arrived.
This is a deliberate, documented choice — the comment at the call site says parking rather than
spinning lets a rule that awaits real I/O make progress without burning a core — and it is exactly
the shape that `standards/rust/`'s cold-future playbook already names as fragile.

The primitive it rests on is where the hazard comes from. `std::thread::park` and `unpark` carry
**one token per thread**, not a counter and not a queue. If `unpark` is called while the target
thread is not currently parked, the token is recorded once; a *second* `unpark` before the thread
parks again is coalesced into the same single token rather than queued as a second wakeup. That is
fine for a single, flat poll loop. It stops being fine the moment a second `block_on` runs *nested*
inside a rule that is itself being driven by an outer `block_on` on the same thread — the shape at
`concurrency.rs:890` through `:980`, where a rule spawns further polling while already inside the
registry's own loop. A wakeup meant for the inner future's waker and one meant for the outer loop's
waker are, from `park`'s point of view, indistinguishable tokens on the same thread: the inner
call can consume the token the outer loop needed, and the outer loop then parks with nothing left
to wake it.

## The measurement

Four runs, deterministic: the baseline (no nesting) completes; the nested case with no wakeup
collision completes; the nested case with a forced collision **hangs past ten seconds**, every
time. The hang is not a race in the sense of "usually fine" — once the collision is forced, it is
unconditional.

## Why it is durable, not a one-off

CF-33 is `[FROZEN]`: no conformance rule may read a clock, measure elapsed time, or carry a
watchdog. That constraint exists so a rule's outcome is a message about the store, not a timing
artefact — but its cost is that a genuine hang inside the suite produces a stopped CI job that
names no rule at all, because nothing in the design is permitted to say "this took too long."
`spec/SPECIFICATION.md:4302-4322` already documents one mechanism with exactly that signature: an
adapter holding an exclusive resource across its `append`'s suspension point, so a concurrent
`read` blocks on what the suspended `append` still holds and the executor parks forever. This atom
records a second mechanism with the same observable shape — a hung run, no rule named — except
this one lives inside the testkit's own executor rather than in an adapter under test, so no
adapter needs to be at fault for it to fire.

## What this atom does not do

`crates/happenstance-sqlite/tests/concurrency.rs:45-47` tells the reader that a hang is evidence
about ADR-0022's busy-timeout paragraph and should be escalated there. That was the correct and
only candidate when it was written. It is not rewritten here: the comment is dated rather than
wrong, and the repair belongs to whoever next edits that file with both mechanisms in view.
