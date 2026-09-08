---
id: kb-open-question-reset-refusal-declension-001
title: RESET_REFUSAL can be declared and left un-mechanised, and nothing in the tree would notice
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0042 retracted ProjectionFixture's trait-level "the fixture writes the reason" requirement
  and replaced it with three defaulted declensions, on the promise that the honesty obligation
  moves to a CF-39-shaped clause-level MUST per capability. COMMIT_FAULT already has one — it
  predates the change and failed_commit_leaves_both_unchanged enforces it — and SECOND_HANDLE
  needs none, because claiming it is self-checking: every rule in the family re-opens a handle and
  reads through it, so a false claim fails on the mechanism rather than on a sentence. RESET_REFUSAL
  is the one capability the retraction left with nothing: a fixture may declare it supported,
  override protect_from_reset with an empty body, and refused_reset_changes_nothing passes on a
  store that was never asked to protect anything. The trait's provided protect_from_reset panics,
  so a forgotten override aborts loudly; a written-but-empty one compiles, does nothing, and looks
  like honest code — NoopFaultFixture's exact shape one port over, and this family has no named
  wrong implementation for it. The population that could be lying about RESET_REFUSAL today is
  empty (no projection adapter has run the suite), which is why the cost of leaving this open is
  low rather than zero, and why it should close before the first one does.
depends_on: []
related:
  - kb-decision-0042
  - kb-decision-0034
  - kb-decision-0036
  - kb-decision-0012
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/projection-declension-obligations.md
  - crates/happenstance-testkit/src/contract.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# RESET_REFUSAL can be declared and left un-mechanised, and nothing in the tree would notice

## What is true today

ADR-0042 ratified Option A of the fixture-declension question and it landed: `ProjectionFixture`'s
three capability constants — `SECOND_HANDLE`, `RESET_REFUSAL`, `COMMIT_FAULT` — are now defaulted
declensions in `crates/happenstance-testkit/src/contract.rs`, and the trait-level rule that used to
require every fixture to write its own reason is retracted, with that rule's argument kept as the
case *for* overriding rather than as a requirement. The change's own stated trade was that the
honesty obligation moves off the trait and onto "a CF-39-shaped clause-level MUST per capability" —
one clause per capability, each saying what declaring it commits a fixture to.

Checked against the three capabilities as they stand, only one of the three actually carries that
clause. `COMMIT_FAULT` has it already, and had it before this change: the constant's doc section
states that a store able to absorb every fault its fixture can arm MUST decline the capability with
that as its stated reason, and `failed_commit_leaves_both_unchanged` requires the armed `commit` to
answer `Err` — so a false claim fails a real rule. `SECOND_HANDLE` needs no clause at all, because
declaring it is self-checking: every rule in the family opens a second handle onto the fixture's
store and reads through it, so a fixture that cannot actually support the capability fails on the
mechanism itself rather than on an unverified sentence.

`RESET_REFUSAL` is the hole the retraction opened. A fixture may declare it supported, override
`protect_from_reset` with a body that does nothing, and `refused_reset_changes_nothing` still
reports green — the rule protects, resets, and finds nothing changed, which is exactly what an
empty override also produces. The trait's provided (un-overridden) `protect_from_reset` panics, so
a fixture author who forgets the override at least fails loudly; one who writes an override that
compiles and does nothing passes silently. That is the same shape as `NoopFaultFixture`, the wrong
implementation CF-39 was written against for `COMMIT_FAULT` one port over — and `RESET_REFUSAL` has
no clause and no registered wrong implementation to catch its analogue.

## What is not decided

Whether the clause gets minted now or waits, and by whom. The shape is not in question — mirror
CF-39: a fixture declaring `RESET_REFUSAL` supported MUST, after `protect_from_reset(id)`, cause the
next `reset(id)` to answer `ResetError::Refused`, and MUST state the mechanism; a fixture whose
store refuses no reset MUST decline the capability with that as its stated reason. What is
unsettled is *ownership*: `PS-1` is `[FROZEN]` and the rest of the PS-clause maturity sweep is
already routed to `unstable-projection-gate-and-clause-disposition`, so minting this clause outside
that sweep risks two sweeps colliding over the same family. ADR-0034 already answers who *would*
mint it — a `CF-` clause belongs to whichever decision first needs the capability, not to a single
fixture-contract owner — which settles the mechanism of minting without settling the timing.

A smaller, separable question rides along: whether the default reason strings this change wrote
describe what the testkit knows (the fixture did not answer) or attempt to speak in the fixture's
own voice the way `Fixture::MID_BATCH_FAULT`'s default does. Neither reading has been argued through.

## What forces it

The first projection adapter to run the suite and declare `RESET_REFUSAL` supported. Today the only
declarers are the testkit's own instruments, so the population that could be lying about the
capability is empty; it stops being empty exactly when the port is meant to move toward frozen,
which is the same moment the PS-clause maturity sweep is scheduled to run. The clause and its rule
— plus a `NoopProtectFixture` registered in `tests/projection_mutation_coverage.rs` so the
obligation has a named wrong implementation — should exist before that adapter lands, not after.
