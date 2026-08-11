---
id: kb-open-question-ps-1-no-progress-obligation-001
title: PS-1's MUST is a coupling, not a progress obligation
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-1 is FROZEN and says the read-model write and the checkpoint write MUST become durable
  together or not at all. §4.11 assigns it three rules, and the third does not follow from the
  sentence: a commit returning Ok that makes neither durable satisfies the "or not at all" arm,
  passes commit_is_atomic_with_the_read_model, and fails commit_advances_the_checkpoint. That a
  successful commit advances anything is stated by no clause's MUST — PS-22 presupposes it and
  §4.1a asserts it non-normatively. Adding the obligation changes the set of implementations the
  clause admits, so it is an ADR's and not an edit's. Owned by phase 6, which discharges
  PS-1 through PS-37 and settles ADR-0017, ADR-0018 and ADR-0019.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# PS-1's MUST is a coupling, not a progress obligation

## What is true today

PS-1 (`spec/SPECIFICATION.md:4733`) is `[FROZEN]` and reads: "The read-model write and the
checkpoint write MUST become durable together or not at all." §4.11's rule table assigns it three
rules: `commit_is_atomic_with_the_read_model`, `failed_commit_leaves_both_unchanged`, and
`commit_advances_the_checkpoint`.

The third does not follow from the sentence. A `commit` that returns `Ok` and makes *neither* the
read-model row nor the checkpoint durable satisfies the MUST through its "or not at all" arm —
both-absent is one of the two states the "together or not at all" coupling permits — passes
`commit_is_atomic_with_the_read_model` because atomicity says nothing about which of the two
permitted states occurred, and fails `commit_advances_the_checkpoint`, which expects a successful
commit to actually move the checkpoint forward. This is not a contrived counterexample: the clause
as written admits an implementation that is atomic and useless, and the rule table pretends the
clause forbids that.

That a successful commit *advances* anything — as opposed to merely being atomic about whatever it
does — is stated by no clause's MUST anywhere in the document. PS-22 presupposes progress happens;
§4.1a's prose asserts it, but non-normatively, which under this specification's own maturity
vocabulary means it binds no implementation.

## What is not decided

Whether PS-1 gets a sentence added — turning the coupling into a coupling-plus-progress obligation
— or whether a separate clause states the progress requirement and PS-1 stays exactly as written,
with `commit_advances_the_checkpoint` reassigned to the new clause instead of PS-1. Per
`kb-playbook-repair-frozen-clause-001`'s test, this is a gap and not a repair precisely because
either fix changes the set of implementations PS-1 (or the spec as a whole) admits — an
implementation that is atomic but makes no progress is legal today and would not be after either
fix.

## What forces it

Phase 6 ("Freeze `ProjectionStore`", `RUNBOOK.md:3846`), which discharges PS-1 through PS-37 and
settles ADR-0017, ADR-0018 and ADR-0019. The projection store port cannot be frozen honestly while
one of its clauses admits an implementation its own rule table rejects — freezing PS-1 as-is would
mean freezing a mismatch between prose and test.

## Ordered sub-questions

1. Is the progress obligation PS-1's to carry, or does it belong on a new clause, given PS-1 is
   `[FROZEN]` and widening it is exactly the kind of act
   `kb-playbook-repair-frozen-clause-001` requires an ADR for?
2. Does resolving this change which rule `commit_advances_the_checkpoint` is attributed to in
   §4.11's table, independent of where the obligation's prose lives?
3. **Before deciding phase 6's answer for PS-1 specifically, check the other 35 PS clauses for the
   same shape** — this defect and the PS-19 gap
   (`kb-open-question-ps-19-scope-narrower-001`) are both cases where a PS-layer clause's MUST is
   narrower than the rule table assigns it, which may be systematic rather than two isolated
   incidents, and the scope of the ADR should reflect whichever is true.
4. Does the answer here interact with ADR-0017, ADR-0018 or ADR-0019, all three of which phase 6
   is scheduled to settle?
