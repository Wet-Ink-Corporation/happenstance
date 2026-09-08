---
id: kb-decision-0042
title: Every future fixture capability lands defaulted, and honesty moves to a clause
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0042
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  No fixture trait in happenstance-testkit ever grows a required associated item
  again: every future capability constant or method lands defaulted, on both
  Fixture and ProjectionFixture. ProjectionFixture's three capability constants
  (SECOND_HANDLE, RESET_REFUSAL, COMMIT_FAULT), previously required with the stated
  policy that the fixture writes its own reason, are now defaulted declensions, and
  that trait-level policy is retracted. The honesty obligation a required item was
  carrying moves to a CF-39-shaped clause-level MUST written per capability, stating
  what declaring the capability commits a fixture to and what a store with nothing
  to offer must say instead. Grounds: five-for-five precedent on the event-store
  side (every capability added since the two founding consts has been defaulted),
  a measured finding (L1-2, over the required REOPEN const) that requiredness
  compels an answer but not a true one, and the fact that ConcurrentFixture's
  blanket impl makes a required item permanently unsatisfiable there -- E0046 on the
  blanket impl itself, E0119 for any adapter that tries to supply it directly. The
  retraction left one hole -- RESET_REFUSAL has no clause yet -- which this decision
  identifies and defers rather than filling.
depends_on:
  - kb-decision-0034
related:
  - kb-decision-0012
  - kb-open-question-cf-40-ownership-001
  - kb-open-question-reset-refusal-declension-001
  - kb-open-question-read-fault-rule-no-clause-001
  - kb-open-question-projection-module-exemption-scope-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/fixture-declension-policy.md
  - .kb/_intake/remediation-2026-09-04-briefs/projection-declension-obligations.md
  - .kb/_intake/remediation-2026-09-04-briefs/read-fault-clause-and-capability.md
  - .kb/_intake/remediation-2026-09-04-briefs/stated-only-defects-and-the-reopen-must.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# Every future fixture capability lands defaulted, and honesty moves to a clause

## Decision

No fixture trait in `happenstance-testkit` — `Fixture`, `ProjectionFixture`, or
any that follows — ever grows a required associated constant or method again.
`ProjectionFixture`'s three capability constants (`SECOND_HANDLE`,
`RESET_REFUSAL`, `COMMIT_FAULT`) move from required to defaulted, and the
trait-level rule that had justified requiring them — "the fixture writes the
reason" — is retracted from `contract.rs`, with its argument kept as the case
for *overriding* the default rather than as a compiler-enforced obligation.

The honesty a required item used to compel moves to a clause-level MUST,
written per capability in the specification, stating what declaring the
capability commits a fixture to and what a store with nothing to offer must
say instead. This is the shape CF-39 already uses for `MID_BATCH_FAULT`.

## Why this is a decision and not a documentation tidy

The trait is published API — `happenstance-testkit` has been on crates.io
since `0.2.0-alpha.1` — and the projection port is expected to move: the next
capability it needs is scheduled, not hypothetical. Two fixture families were
already running opposite policies on the same document review, with no clause
anywhere governing which a release may take. A commitment about what a minor
version of a testkit crate may do to every out-of-tree fixture is exactly the
shape this corpus routes to a decision atom rather than a playbook.

## The precedent, measured rather than asserted

**Five for five, on the side that has adapters.** Every capability added to
`Fixture` since its two founding consts (`SECOND_HANDLE`, `REOPEN`) has
shipped defaulted: `MID_BATCH_FAULT` and the three `Option<usize>` ceiling
constants. The projection family's opposite rule was one documentation
paragraph with zero adapter experience behind it — no projection adapter had
run the suite, and the port is not frozen.

**The nearest capability of the same shape is already decided the other way,
in the frozen specification.** `COMMIT_FAULT`'s closest relative is
`MID_BATCH_FAULT`: both are trades rather than facts, both have a
store-specific reason for declining. `MID_BATCH_FAULT` is defaulted, its
default reason is written in the fixture's own voice, and CF-39 recovers the
honesty as a clause-level MUST including "the fixture MUST state the
mechanism" and a MUST-decline for a store that can absorb every fault it is
able to arm. That is this decision's shape, already shipped, for the case
`ProjectionFixture` departed from without arguing what the departure bought.

**Requiredness compelled a sentence, not a true one.** `REOPEN` is a required
const on `Fixture`. The compiler forced an answer; `NoopReopenFixture` gave a
false one and scored higher (86 passed, 3 skipped) than an honest fixture (83
passed, 6 skipped), and no rule on the surface as it stood could reject it.
The defect could not be turned into a rule at all — the scenario meant to
distinguish an honest `reopen` from an empty one instead separated two
*styles* of honest implementation (a durable file-backed store and one that
replaces its log), with `happenstance-sqlite` on the side a naive rule would
have rejected. What survives is a record, not a rule: `NoopReopenFixture` is
driven through the whole suite and its escape is asserted and named rather
than concealed behind a passing green.

**The blanket impl forecloses one shape permanently.** `ConcurrentFixture` is
`impl<F> ConcurrentFixture for F where F: Fixture, F::Store: Send {}`. A new
required item on that trait is `E0046` on the blanket impl itself — nowhere to
write the value — and an adapter cannot supply it directly either, because the
blanket impl already covers it (`E0119`, conflicting implementations). Any
capability the concurrency rules ever need has to land on `Fixture` instead,
so "required, but only there" is not an available escape hatch.

## What retracting the trait-level rule leaves owed

Moving the obligation off the trait is a trade, and checking that the honesty
side of it has somewhere to land is part of this decision. Of
`ProjectionFixture`'s three capabilities: `SECOND_HANDLE` needs no clause,
because a false claim fails on the mechanism (every rule in the family opens a
second handle and reads through it) rather than on a promise about it.
`COMMIT_FAULT` already had its obligation written on the constant before this
change, with a rule that requires the armed commit to answer `Err` — nothing
was lost. `RESET_REFUSAL` is the hole: a fixture may declare it, override
`protect_from_reset` with an empty body, and pass the suite's reset-refusal
rule vacuously. That hole existed before this decision — the trait's provided
body panics on a forgotten override but not on an empty one — and this
decision changes its visibility, not its presence.

That hole is not closed here. Minting the clause and its rule belongs to the
specification's own maturity sweep for the projection family; adding one here
would collide with that sweep in progress.

## What this does not decide

Whether `Fixture::SECOND_HANDLE` and `Fixture::REOPEN` — the event-store
family's two remaining required constants — should also become defaulted is
left open. Whether `kb-decision-0034`'s clause-ownership finding or the
projection port's gating exemption reach `ProjectionFixture` itself is
untouched by this decision. A capability that genuinely cannot be defaulted is
not forbidden either: it lands as a correctly-classified breaking release
rather than as a required item slipped into a minor.

## Alternatives rejected

A minor release adding a required item, with the changelog stating that its
minors are source-breaking. Rejected because at `0.x` a minor and a major that
a caret cannot resolve are the same release event, so this bought no
mechanical difference over correctly labelling the release a major, while
keeping the projection family's policy without arguing for it. Quarantining
new capabilities on an opt-in extension trait with its own macro. Rejected
because it does not work for `ConcurrentFixture` at all, and a fixture that
never invokes the second macro emits no test and no skip line — the silent
omission this corpus's skip-reporting discipline forbids.
