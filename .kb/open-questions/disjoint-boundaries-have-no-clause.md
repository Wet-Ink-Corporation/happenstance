---
id: kb-open-question-disjoint-boundaries-no-clause-001
title: The DCB independence proposition is enforced by a rule and stated by no clause
kind: open_question
status: accepted
authority_tier: note
summary: >-
  k_disjoint_boundaries_admit_exactly_k_commits is a live conformance rule enforcing the
  independence proposition Dynamic Consistency Boundary exists for — commands sharing no
  consistency boundary do not conflict — and no clause in spec/SPECIFICATION.md states it; the
  word "disjoint" occurs zero times, verified by grep on 2026-08-10. ES-25 is the wrong attachment
  point: its only-if half forbids the false-positive direction and does not assert the positive
  proposition, so attaching there would claim a FROZEN clause contains something it does not.
  Settled by an ADR that widens ES-25 or mints a clause. Owner unassigned; held meanwhile in
  UNCLAIMED_PENDING_ADR, which prints it on every green gate run.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-model-family-rule-no-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - crates/happenstance-testkit/src/concurrency.rs
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
last_reviewed: 2026-08-10
---

# The DCB independence proposition is enforced by a rule and stated by no clause

## What is true today

`k_disjoint_boundaries_admit_exactly_k_commits`
(`crates/happenstance-testkit/src/concurrency.rs:436`) is a live conformance rule that every
event-store fixture in this workspace runs. It enforces the proposition that commands sharing no
consistency boundary do not conflict — the independence property Dynamic Consistency Boundary
exists to provide in the first place. **No clause in `spec/SPECIFICATION.md` states that
proposition.** The word "disjoint" occurs zero times in the document, verified by grep against
the working tree on 2026-08-10.

ES-25 is the obvious candidate for where this rule should attach, and it is the wrong one. ES-25's
*only if* half forbids the false-positive direction — it says a non-overlapping append must not be
rejected for the wrong reason — and does not state the positive proposition that *k* disjoint
commands all commit. Attaching the rule to ES-25 would assert that a `[FROZEN]` clause contains a
proposition it does not contain, which is a defect strictly worse than the missing attribution
because no automated check could ever see it: `cargo xtask spec-trace` verifies that a named rule
exists and that a clause's citations resolve, not that the clause's prose actually says what the
rule tests.

This is one of two entries (with the model-family gap) held in `UNCLAIMED_PENDING_ADR`
(`xtask/src/spec_trace.rs:1968-1997`), a list that prints on every green `cargo xtask spec-trace`
run. It has a mechanism keeping it visible; most gaps found in the same pass do not.

## What is not decided

Whether the independence proposition gets a clause of its own, or ES-25 is widened to state it.
Widening a `[FROZEN]` clause is itself an ADR-scale act — per
`kb-playbook-repair-frozen-clause-001`'s test, changing what a frozen clause admits is a gap, not a
repair — so the two options differ in cost less than they first appear to. Neither has been
weighed against the other; this record exists so that weighing happens once, deliberately, rather
than by whichever engineer next needs a place to hang a citation.

## What forces it

Nothing forces it today beyond the standing pressure of `UNCLAIMED_PENDING_ADR` printing on every
gate run. It becomes urgent the first time someone needs to cite spec authority for
`k_disjoint_boundaries_admit_exactly_k_commits` — in a review, in a new adapter's documentation, or
in an ADR for a different port — and finds there is nothing to cite. It is also the sharpest of the
six gaps this pass found: it is the library's central claim, tested exhaustively and stated
nowhere.

## Ordered sub-questions

1. Does the independence proposition belong on ES-25 (widened) or on a new clause? Answering this
   first determines whether the ADR is a one-line amendment or a new normative sentence with its
   own maturity marker and rule table entry.
2. If a new clause, where in `spec/SPECIFICATION.md`'s structure does it sit — adjacent to ES-25 in
   the append-condition family, or as its own top-level property alongside the model-family gap?
3. Does resolving this gap change how `UNCLAIMED_PENDING_ADR` reports the remaining entry (the
   model-family rule), given the two are related but distinct in shape?
