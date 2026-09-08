---
id: kb-open-question-poll-count-rule-strength-001
title: The rule that checks the visibility invariant has an uncalibrated window
kind: open_question
status: superseded
superseded_by: kb-open-question-off-poll-visibility-defect-001
authority_tier: note
summary: >-
  nothing_below_an_observed_position_appears_later is the only rule checking the invariant
  ADR-0013 lifted to frozen, and its discriminating power varies with something nobody has
  measured: the adapter's poll shape. The technique builds two append futures from one handle and
  polls them out of order, which works because a Rust future is cold until polled — but the window
  it opens is bounded by the number of polls the adapter's append needs, and an adapter needing
  three or more may slip through it. What is not decided is how to calibrate it. The direct repair,
  a POLL_BUDGET capability on the fixture, was considered and rejected at phase 4: it would be the
  first non-boolean capability, it sits close to the no-clock rule, and calibrating it needs a real
  I/O-bound adapter that does not exist. Refuted by a poll-padding decorator over
  PreCommitPositionStore that the rule fails to reject, calibrated against a three-poll adapter.
  Owned by phase 10. If it fires, only the rule changes — the clause it checks does not.
  Amended 2026-08-20: ADR-0034 records that the fixture contract has no single owning document,
  so ADR-0013's "whoever owns the fixture contract" has no referent — a POLL_BUDGET capability
  would be minted by the decision that needs it, and this question is that position's named next
  test. Resolved 2026-09-06 at phase 10, which built the poll-padding decorator, calibrated it
  against a measurement rather than a choice, and watched it fire. n comes from the adapter:
  happenstance-postgres's shipped append needs three polls under one-millisecond pacing against a
  live PostgreSQL 17.10, which is the number the specification already named. The repair changed
  only the rule's schedule — poll the slow writer once to take its number, then drive the fast
  writer to completion — so ES-10 is untouched and still FROZEN, exactly as ADR-0013 predicted.
  POLL_BUDGET is moot rather than deferred: a schedule that counts no polls needs no budget, so
  ADR-0013's CF-33 tension never has to be resolved and kb-decision-0034's named next test never
  runs. Superseded rather than withdrawn because the recalibrated rule has a blind spot on the
  other side — an adapter advancing off-poll — which is
  kb-open-question-off-poll-visibility-defect-001's, owned by ADR-0024.
depends_on: []
related:
  - kb-decision-0013
  - kb-playbook-cold-future-hand-polling-001
  - kb-decision-0010
  - kb-decision-0034
  - kb-open-question-testkit-contention-tolerance-001
  - kb-open-question-off-poll-visibility-defect-001
  - kb-decision-0024
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - references/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
  - .kb/_intake/2026-09-06-poll-count-calibrated-and-a-second-limitation.md
  - crates/happenstance-postgres/tests/poll_shape.rs
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-09-07
---

# The rule that checks the visibility invariant has an uncalibrated window

## What is true today

ADR-0013 lifts ES-10 — the global visibility invariant — to `[FROZEN]`, and decision §8 states
plainly that the entire lift rests on one rule: `nothing_below_an_observed_position_appears_later`.
The rule "creates two `append` futures from one handle and polls them by hand in the order A, B, B,
A, reading after every step, so that the writer which started second is resumed first — the
reversal is what models the slow transaction that took a low number and published it late." This
works at all because a Rust future is cold: constructing an `async fn`'s future runs none of its
body, so building two futures is not starting two writers, it is building two suspended state
machines the rule fully controls (`suite.rs:2766-2772`), using `pin!` to make them pollable in
place without a heap allocation.

The rule's own comment states its declared limitation honestly: "an adapter needing three polls is
not let off. Asserting 'two polls was enough' would be over-specification." ADR-0013 restates this
as a real gap rather than a documented non-issue: "against a store whose `append` needs three
polls, the interleaving window never opens where the rule looks, and the rule cannot fail. Its
strength varies silently with the adapter." `MemoryEventStore` passes trivially because its
`append` contains no `.await` at all — correctly, since a store with no suspension point cannot
have a window bug — "but the same mechanism means a real store's window can sit outside the
schedule."

`Fixture` declares exactly three capability constants — `SECOND_HANDLE`, `REOPEN`, and
`MID_BATCH_FAULT` — and none of them is a poll budget, so nothing in the harness lets a rule ask
"how many polls does this adapter's `append` actually need" before deciding how hard to test it.

## What is not decided

How — or whether — to calibrate the rule's window against a real adapter's poll count. ADR-0013
names the direct repair and explains why it was rejected rather than adopted at phase 4: "The
obvious repair — a `POLL_BUDGET` associated constant on `Fixture` that the schedule consumes —
would be the first capability that is not a yes/no answer, in a contract five of whose six clauses
(CF-16, CF-18, CF-19, CF-20, CF-21) are already `[FROZEN]`." It also sits close to CF-33's
prohibition on a rule asserting on an operation count — "a budget the schedule *consumes* is
arguably not an assertion — but that distinction is exactly the kind of reasoning CF-33 exists to
keep out of the suite, and it should be made by whoever owns the fixture contract, with an adapter
in front of them." And calibration itself needs an adapter with real I/O-bound suspension, which
does not exist yet: "an author choosing *n* is the reference-store failure mode with one more
step — so the calibration waits for an adapter with real I/O."

## What forces it

Phase 10, named explicitly as the owner, "rather than phase 8, because what the calibration wants
is a store whose `append` genuinely suspends more than once, and phase 10 is where the
`sqlx`-backed one lands; if phase 8's adapter turns out to suspend, phase 8 may take it earlier."
The instrument that would settle it is already named and is cheap to build: "a poll-padding
decorator over `PreCommitPositionStore` that inserts *n* `Pending` returns into `append`, and the
observation is whether the rule still rejects it." What that decorator alone cannot supply is the
right `n` to calibrate against, which is why a real I/O-bound adapter is the actual blocker rather
than the decorator's existence.

## Ordered sub-questions

1. Does phase 8's adapter (if it turns out to genuinely suspend across `.await` inside `append`)
   pull this calibration forward, per ADR-0013's own conditional — "if phase 8's adapter turns out
   to suspend, phase 8 may take it earlier"?
2. Is a `POLL_BUDGET` capability the right eventual shape, or does the CF-33 tension ADR-0013 flags
   mean the fixture contract needs a different mechanism entirely — one that isn't shaped like an
   assertion the schedule "consumes"?
3. Once the poll-padding decorator exists, what value of `n` is the right first calibration target,
   and does it come from `happenstance-postgres`'s actual `append` implementation once phase 10
   builds it, or from a synthetic worst case chosen independently?
4. If the decorator does find an `n` the rule fails to reject, does the fix change only
   `nothing_below_an_observed_position_appears_later`'s schedule (as ADR-0013 predicts — "the rule
   changes and ES-10 does not"), or does discovering a real three-poll adapter also change anything
   about the poll-count limitation's status as a phase-10-owned concern versus something that
   should have gated the freeze itself?

## Resolved 2026-09-06 — status `superseded`; the residual moves out

Everything above is the state of knowledge on 2026-08-20 and is left exactly as it was written,
including the parts that describe a repair nobody had to build. Phase 10 built the instrument
ADR-0013 named, calibrated it against a measurement rather than a preference, ran it, and it fired.
No accepted decision was edited to produce any of it, and `spec/SPECIFICATION.md` is not touched.

**Sub-question 1 is answered by events.** Phase 8's adapter did not pull the calibration forward;
phase 10 took it, which is where ADR-0013 put it and for the reason ADR-0013 gave — the calibration
wanted a store whose `append` genuinely suspends more than once, and the `sqlx`-backed one is the
first that does.

**Sub-question 3 is answered from the adapter, not from a synthetic worst case.** This is the half
ADR-0013 cared about, because "an author choosing *n* is the reference-store failure mode with one
more step". `crates/happenstance-postgres/tests/poll_shape.rs` polls the *shipped* `append` against
a live PostgreSQL 17.10 and reports **three** polls to completion at one millisecond between polls
— the number `spec/SPECIFICATION.md` already names as the case the old schedule cannot fail. The
instrument that consumed it is `PollPaddedPositionStore`: `PreCommitPositionStore` with one extra
`Pending` in `append`, same defect, two polls become three, registered in `for_each_mutant!` with a
`REGISTRY` row naming the rules its defect implies. Under the old `A, B, B, A` schedule it
**passed**.

**Sub-question 4 is answered as ADR-0013 predicted, in both halves.** The fix changed the rule's
schedule and nothing else: poll the slow writer once so it takes its number, then drive the fast
writer *to completion* rather than a fixed number of times — asking for what the rule actually needs
(that the fast writer has committed) instead of a proxy that implies it only where a store suspends
exactly once. The rule's name, its assertions and its `observe` helper are unchanged; every in-tree
adapter still passes; both `PreCommitPositionStore` and `PollPaddedPositionStore` now fail it.
ES-10 stays `[FROZEN]` and unedited. And nothing about a real three-poll adapter turning up argues
the limitation should have gated the freeze — the clause was right and the instrument was missing.

**Sub-question 2 is moot, which is a different outcome from deferred.** The new schedule counts no
polls, so there is nothing for a `POLL_BUDGET` to budget. The first non-boolean fixture capability
is not minted, ADR-0013's CF-33 tension does not have to be resolved, and `kb-decision-0034`'s
position — a `CF-` clause is minted by the decision that first needs the capability — keeps its
named next test *unrun* rather than passing it. That position is untested, not confirmed.

**Why this is `superseded` and not `withdrawn`.** The recalibrated rule is stronger against the
failure this atom described and still blind from the other side. An adapter whose `append` hands its
work to a runtime advances **off-poll**: the same measurement that gives three polls at one
millisecond gives roughly 25,000 in a tight loop, because those polls are only asking whether the
work has finished. `happenstance-postgres` is that shape necessarily — `sqlx` wants a runtime in
thread-local scope and the suite's contenders run on raw threads that have none — and its
deliberately naive arm passes the recalibrated rule anyway. Padding a poll-driven state machine
yields a slower poll-driven one, never an off-poll one, so no schedule-based instrument reaches it.
That residual is `kb-open-question-off-poll-visibility-defect-001`, owned by ADR-0024
(`kb-decision-0024`).
