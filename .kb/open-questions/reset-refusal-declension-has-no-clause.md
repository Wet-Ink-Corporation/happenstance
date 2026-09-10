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
  wrong implementation for it. The window this atom originally named has since closed and the
  question has not. Its cost argument rested on no projection adapter having run the suite and on
  the clause landing before the first one did; four storage adapters now invoke
  projection_store_conformance! — happenstance-sqlite, happenstance-postgres, happenstance-neon
  and happenstance-ladybug — so the first has arrived. That neither weakens the finding nor
  answers it. All four decline RESET_REFUSAL, and a declining adapter cannot be lying about a
  capability it never claimed, so the population is still empty and the family still has no named
  wrong implementation. ADR-0025 pre-registered RESET_REFUSAL declined among four capability
  predictions committed before any body was written, so they could not be fitted to the outcome,
  and it held. What has changed is only the cost of leaving this open: the next adapter to declare
  the capability supported will do so with nothing checking it, and there is no longer a
  first-adapter deadline to act before.
depends_on: []
related:
  - kb-decision-0042
  - kb-decision-0034
  - kb-decision-0036
  - kb-decision-0012
  - kb-decision-0025
  - kb-reference-ladybug-driver-probes-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/projection-declension-obligations.md
  - .kb/_intake/2026-09-08-adr-0025-ladybug-projection-adapter.md
  - crates/happenstance-testkit/src/contract.rs
  - crates/happenstance-ladybug/tests/projection.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-09
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

## What forces it, and why the deadline has already passed

What forces it is the first projection adapter to run the suite and declare `RESET_REFUSAL`
supported. Half of that has now happened. Four storage adapters invoke
`projection_store_conformance!` — `happenstance-sqlite`, `happenstance-postgres`,
`happenstance-neon` and `happenstance-ladybug` — so the sentence this section used to carry, that
the only declarers are the testkit's own instruments, is no longer true.

The finding survives intact, because all four **decline** the capability. `happenstance-ladybug`'s
decline is the most explicit of them and reads as the pattern: the store owns one node table,
`__hs_checkpoint`, and the transaction that carries it, is never told which labels the read model
uses, and so returns `Refused` on no path at all — with the cheap alternative that would have
manufactured a `Ran` (a `__hs_protected` node table consulted by `reset`) named and rejected for
putting a domain policy into an adapter designed to keep the domain out. A fixture that declines
cannot be lying about a mechanism it never claimed, so the population that could be lying is still
empty, and `refused_reset_changes_nothing` still has no adapter it could be wrong about.

ADR-0025 (`kb-decision-0025`) is worth citing for *how* that came out rather than only that it did.
Its verdict was **pre-registered** — four capability predictions committed before any body was
written, so they could not be fitted to the outcome — and `RESET_REFUSAL` declined was one of the
four. It held. That is evidence about the default rather than about one adapter: a store that owns
only its checkpoint has no protection policy to expose, and declining is the honest answer rather
than the lazy one.

What has changed is this atom's own cost argument, and it has changed for the worse. The reason for
tolerating an unminted clause was that it should close *before* the first adapter arrived; that
deadline has passed without the clause. The PS-clause maturity sweep is now the only schedule hook
left holding it, and it is no longer racing an adapter it can be timed against. The clause, its
rule, and a `NoopProtectFixture` registered in `tests/projection_mutation_coverage.rs` so the
obligation has a named wrong implementation are all still owed — and the next adapter to declare
`RESET_REFUSAL` supported will do so with nothing checking it, with no advance warning of which
adapter that will be.
