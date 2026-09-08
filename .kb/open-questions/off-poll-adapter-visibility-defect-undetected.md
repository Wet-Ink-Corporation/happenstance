---
id: kb-open-question-off-poll-visibility-defect-001
title: The conformance suite has no instrument for an off-poll adapter's visibility defect
kind: open_question
status: accepted
authority_tier: note
summary: >-
  nothing_below_an_observed_position_appears_later was recalibrated at phase 10: the schedule now
  polls the slow writer once, then drives the fast writer to completion, which discharges the
  poll-count limitation ADR-0013 flagged and fires against both PreCommitPositionStore and its
  padded successor. But happenstance-postgres advances off-poll — sqlx hops onto a captured
  runtime Handle, so its append's suspension is scheduled by the runtime rather than counted in
  polls — and a deliberately naive arm with the frontier predicate removed passes the recalibrated
  rule anyway. naive_arm_probe.rs shows the inversion by direct observation: the naive arm's
  reader sees position 2 before position 1 commits; the shipped store never does. The port exposes
  no suspension point between allocation and commit for a rule to wedge into, so no schedule-based
  instrument can reach this defect family. What is not decided is whether the suite should be able
  to detect an off-poll visibility defect at all, and with what instrument, given the thing that
  does catch it today — holding a transaction open — is adapter-specific and lives in the
  adapter's own tests rather than in the testkit. Owner: ADR-0024.
depends_on:
  - kb-decision-0013
related:
  - kb-decision-0024
  - kb-open-question-poll-count-rule-strength-001
  - kb-open-question-postgres-arm-c-cost-001
source_paths:
  - .kb/_intake/2026-09-06-poll-count-calibrated-and-a-second-limitation.md
  - .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-postgres/tests/poll_shape.rs
  - crates/happenstance-postgres/tests/naive_arm_probe.rs
last_reviewed: 2026-09-07
---

# The conformance suite has no instrument for an off-poll adapter's visibility defect

## What is true today

The poll-count question the suite carried since ADR-0013 is closed. `nothing_below_an_observed_position_appears_later`'s discriminating power used to vary silently with an adapter's poll shape — an `append` needing three or more suspensions could slip through a schedule calibrated on two. Phase 10 measured the real number against `happenstance-postgres`'s shipped `append` on a live PostgreSQL 17.10 (`crates/happenstance-postgres/tests/poll_shape.rs`): **three** polls under realistic timing, which matches what `spec/SPECIFICATION.md` already names as the boundary case. The fix changed only the rule's schedule — poll the slow writer once to take its number, then drive the fast writer *to completion* rather than a fixed number of times — and both `PreCommitPositionStore` and a poll-padding decorator built to calibrate the old schedule now fail it. ES-10 stays `[FROZEN]` and unedited; the specification's own disposition applied: the rule changed, the clause did not.

That closure surfaced a second, different limitation. `happenstance-postgres` does not advance by polling at all in the sense the rule models: `sqlx` needs a runtime in thread-local scope, so the store captures a `Handle` and hops onto it, and the suite's own concurrency contenders run on raw threads with none. Its transaction opens and commits on the runtime's schedule, not on a count of `.await` points the rule's hand-polling can control. `crates/happenstance-postgres/tests/naive_arm_probe.rs` demonstrates the consequence directly, by building the inversion the recalibrated rule is supposed to reject:

```
naive   | after fast commit: [2]      <- reader observes 2
naive   | after slow commit: [1, 2]   <- 1 appears BENEATH it
shipped | after fast commit: []
shipped | after slow commit: [1, 2]   <- never inverts
```

The naive arm — the shipped store with the frontier predicate removed, reachable only behind the off-by-default `naive-arm` feature — passes CF-13 (`nothing_below_an_observed_position_appears_later`) even after the schedule change. A poll-padding decorator cannot reach this defect family at all: padding a poll-driven state machine produces a slower poll-driven state machine, which is a different shape from a store whose suspension is handed to a runtime.

## What is not decided

Whether the conformance suite should be able to detect an off-poll adapter's visibility defect at all, and if so with what instrument. The `EventStore::append` port exposes no suspension point between position allocation and commit that a hand-polled schedule can wedge into — the technique that makes `nothing_below_an_observed_position_appears_later` work (building two cold futures from one handle and resuming them out of order) depends on the adapter's suspension being visible to the caller as a poll, and an off-poll adapter's is not. The thing that does catch the naive arm's defect today — holding a transaction open across an unrelated write and measuring staleness — is adapter-specific, lives in `happenstance-postgres`'s own `tests/`, and is not a portable conformance rule any other adapter could run unmodified.

## What forces it

ADR-0024, which already owns the sibling question of `happenstance-postgres`'s position-visibility mechanism and its structural cost. Any future adapter that hops onto a runtime the way `happenstance-postgres` does — `happenstance-neon`'s one-shot-HTTP shape is a candidate — inherits the same blind spot, so the question compounds with every off-poll adapter added rather than staying scoped to one.

## Ordered sub-questions

1. Is a portable instrument possible at all, or is "hold a transaction open and measure staleness under load" inherently adapter-specific because it depends on knowing what the adapter's storage medium can be made to hold open?
2. If a portable instrument is possible, does it belong in the testkit as a new conformance rule, or as a documented technique adapter authors are expected to apply in their own test suite the way `happenstance-postgres` did?
3. Does the `Fixture` capability model need a new declaration — analogous to `MID_BATCH_FAULT` — so that an off-poll adapter can name a way to hold its own transaction open for a rule to drive, the way `happenstance-postgres`'s own tests already do informally?
