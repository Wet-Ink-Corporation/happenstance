---
id: kb-open-question-ps-19-scope-narrower-001
title: PS-19's MUST is scoped after a reset; its second rule asks about an unseen id
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-19 is FROZEN and scoped to what checkpoint(id) returns after a successful reset, while §4.11
  additionally assigns it fresh_projection_has_no_checkpoint, which asks about an id never seen.
  The implementation that exposes the gap is the natural one: reset writes an explicit NeverRun
  sentinel and checkpoint(id) resolves a missing row with unwrap_or(Live { through: FIRST }) —
  satisfying the MUST verbatim and failing the rule. No clause obliges an unseen id to read
  NeverRun. Settled by an ADR widening PS-19 or minting a clause. Owned by phase 6. Interacts with
  the PS-1 gap: both are PS-layer clauses whose MUST is narrower than the rule table assigns them,
  so whoever takes either should first check the other 35 PS clauses for the same shape.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# PS-19's MUST is scoped after a reset; its second rule asks about an unseen id

## What is true today

PS-19 (`spec/SPECIFICATION.md:5218`) is `[FROZEN]` and reads: "After a successful `reset`,
`checkpoint(id)` MUST return `Checkpoint::NeverRun`, and this MUST be distinguishable from
`commit(empty_batch, id, SequencePosition::FIRST, Live)`." §4.11 assigns it two rules:
`reset_is_not_commit_at_first` and `fresh_projection_has_no_checkpoint`. The second asks about an
id that has **never been seen**, while the MUST is scoped entirely to the state *after a successful
`reset`* — those are different points in a projection's lifecycle.

The implementation that exposes the gap is the natural one, not a contrivance. A store whose
`reset` writes an explicit `NeverRun` sentinel row, and whose `checkpoint(id)` resolves a *missing*
row with `.unwrap_or(Checkpoint::Live { through: FIRST })`, answers `NeverRun` after a reset —
satisfying the MUST verbatim, and distinguishable from a commit at `FIRST` exactly as required — and
answers `Live` for an id it has never seen, because that id has no row and the `unwrap_or` default
fires. This passes `reset_is_not_commit_at_first` and fails `fresh_projection_has_no_checkpoint`.
No clause's MUST obliges an unseen id to read as `NeverRun`; the rule table assumes an obligation
the clause never states.

## What is not decided

Whether PS-19 widens to also cover the never-seen-id case, or a new clause states that obligation
separately and `fresh_projection_has_no_checkpoint` is reattributed to it. Widening a `[FROZEN]`
clause is an ADR-scale act under `kb-playbook-repair-frozen-clause-001`'s test — it changes which
implementations PS-19 admits, since the `unwrap_or(Live)` implementation above is legal under the
clause as written today and would not be after either fix.

## What forces it

Phase 6 ("Freeze `ProjectionStore`", `RUNBOOK.md:3846`), the same forcing event as the PS-1 gap —
freezing the port while one of its clauses admits an implementation its own rule table rejects
would freeze the mismatch along with the port.

## Ordered sub-questions

1. Does the never-seen-id obligation belong on PS-19 (widened) or on a new clause, given PS-19's
   own text is scoped to post-reset behavior and stretching it to cover a state reset never
   touches may be the wrong shape regardless of the ADR-scale cost?
2. **Before deciding, check the other 35 PS clauses for the same shape.** This gap and the PS-1
   gap (`kb-open-question-ps-1-no-progress-obligation-001`) are both PS-layer clauses whose MUST is
   narrower than the rule table assigns them — that pairing at a similarity score of 40, the
   closest of the six gaps in this batch, suggests §4.11's table may have been populated with a
   systematic assumption that does not match the clause text it sits beside, rather than two
   isolated defects. Whoever takes either gap should scan the remainder before scoping the ADR,
   since a single pass fixing the table's assumption may be cheaper than two point fixes.
3. If a new clause is minted for the never-seen-id case, does it sit adjacent to PS-19 in the
   specification's structure, or wherever the phase-6 ADR work groups the progress-obligation
   clause from the PS-1 gap — since both may be settled by clauses about what a *fresh* projection
   is owed?
