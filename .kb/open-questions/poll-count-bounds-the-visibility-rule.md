---
id: kb-open-question-poll-count-rule-strength-001
title: The rule that checks the visibility invariant has an uncalibrated window
kind: open_question
status: accepted
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
  test.
depends_on: []
related:
  - kb-decision-0013
  - kb-playbook-cold-future-hand-polling-001
  - kb-decision-0010
  - kb-decision-0034
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - references/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/fixtures.rs
  - .kb/_intake/cf-40-fixture-contract-ownership-resolution.md
last_reviewed: 2026-08-20
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
