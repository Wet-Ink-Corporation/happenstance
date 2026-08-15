---
id: kb-open-question-ps-19-scope-narrower-001
title: PS-19's MUST is scoped after a reset; its second rule asks about an unseen id
kind: open_question
status: superseded
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
  Amended 2026-08-13: the 37-clause pairing sweep answers sub-question 2 - the systematic
  §4.11-table hypothesis is not supported, there being two defects inside the table and both
  already known, so three point repairs and one recorded lesson is the shape - and the finding
  itself reproduces against a sharper exposing store, so the atom is confirmed rather than
  corrected. ADR-0018 (kb-decision-0018) scoped this defect out of its own range by name and
  repaired nothing; sub-questions 1 and 3 stay open, owner unchanged.
  Resolved 2026-08-15 by ADR-0030 (kb-decision-0030): PS-19 keeps its post-reset scope and the
  never-seen-id obligation becomes PS-38's second sentence, a ProjectionId no successful commit has
  named MUST read as Checkpoint::NeverRun. Sub-question 1 is answered a new clause, sub-question 3 by
  section 4.7 rather than adjacency to PS-19; fresh_projection_has_no_checkpoint is now listed
  against both clauses and still does not exist, so it renders †. Sub-question 2's isolated verdict
  from 2026-08-13 stands untouched.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-decision-0018
  - kb-decision-0030
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/2026-08-13-adr-0018-reset.md
  - .kb/_intake/2026-08-13-ps-clause-pairing-sweep.md
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - references/adr/0018-returning-a-projection-to-never-run.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-15
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

## Amended 2026-08-13 — sub-question 2 answered; the finding confirmed; ADR-0018 scoped it out

The paragraphs above are what was known on 2026-08-10 and are left as written. Two documents
reached this question on 2026-08-13 by different routes, and both land here.

**Sub-question 2 is answered: the systematic hypothesis is not supported.** The scan it asks for
was run over all 37 `PS` clauses against a threshold declared before the count
(`references/evaluation/ps-clause-pairing-sweep.md`, pinned to `2136dde`): 29 `sound`, 7
`defective`, 1 `undetermined`. Three of the seven are *independent* — an exposing implementation
that satisfies every other `PS` `MUST` — and only two of those three, PS-1 and this one, involve a
rule §4.11's table introduced. A single pass over the table is therefore **not** cheaper than point
fixes, because there is nothing table-wide to fix: two defects inside it, both already known.
Three point repairs — PS-1, PS-19, PS-29 — plus one recorded lesson is the shape, and it is the
shape phase 6's budget already assumes.

The lesson is the qualification that changes the repair's framing: the shape **recurs outside the
table**. PS-29 carries it on `one_poisoned_projection_does_not_stall_the_others`, a rule that lives
only in PS-29's own clause body and is not one of §4.11's seventeen. The cause is a habit of
writing the rule to the clause's *intent* rather than to its *sentence*, distributed across the
family, not one table's population.

**This atom's own finding is confirmed, not corrected.** The sweep re-derived it from
`spec/SPECIFICATION.md:5218-5223` before re-reading this atom, and named the exposing store more
sharply: `checkpoint` is `SELECT position, authority FROM checkpoints WHERE id = ?`, the missing
row resolves through `.unwrap_or(Checkpoint::Live { through: FIRST })` — the cheapest default,
`SequencePosition` being `NonZeroU64` with `FIRST` as its minimum — and `reset` writes an explicit
`NeverRun` sentinel row. It satisfies every other `PS` `MUST`, PS-22 included, since nothing can
regress below `FIRST`. The bar this atom set for itself — the natural implementation, not a
contrivance — is met.

**ADR-0018 (`kb-decision-0018`) met the same defect inside its own clause range (PS-16 – PS-20),
named it, scoped it out, and repaired nothing** — widening the clause changes the set of
implementations it admits, so the repair is a decision's, not a record's. It lands at
`unstable-projection-gate-and-clause-disposition` under `kb-playbook-repair-frozen-clause-001`'s
discipline.

Nothing here resolves the question. Sub-questions 1 and 3 stay open, `status` stays `accepted`, and
the owner is unchanged.

## Resolved 2026-08-15 — a new clause, in §4.7; `status` superseded

Everything above is the state of knowledge on 2026-08-10 and 2026-08-13 and is left exactly as
written, per this layer's README. ADR-0030, "The checkpoint reports the commits that happened"
(`kb-decision-0030`; full record at
`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`), now holds the answer, so
this atom moves to `superseded` rather than `withdrawn` — the question was worth asking, and a
reader arriving here needs sending on.

**Sub-question 1 is answered: a new clause, not a widening.** PS-19 keeps its scope, *after a
successful `reset`*, and its `MUST` is byte-identical across the decision
(`spec/SPECIFICATION.md:5288-5290`, with phase 6's answer recorded beneath it at `:5311-5317`). The
never-seen-id half becomes PS-38's **second** sentence — *a `ProjectionId` no successful `commit`
has named MUST read as `Checkpoint::NeverRun`* (`:5447-5449`) — which is why the
`.unwrap_or(Checkpoint::Live { through: FIRST })` store this atom named is now rejected by a clause
rather than only by a rule reaching past one. Widening PS-19 lost for the reason *What is not
decided* anticipated: it changes the set of implementations a `[FROZEN]` clause admits, which is a
gap and a decision's rather than an edit's.

**Sub-question 3 is answered by §4.7, not by adjacency.** The new clause does not sit beside PS-19.
It sits in §4.7 with the progress obligation from the PS-1 gap
(`kb-open-question-ps-1-no-progress-obligation-001`) — the second of the two outcomes sub-question 3
offered, and one clause rather than two, because *a commit is visible in the checkpoint* and
*nothing else is* are one proposition about what a fresh projection is owed.

**`fresh_projection_has_no_checkpoint` is now listed against both clauses and still does not
exist.** §4.11's table reads it against `PS-19, PS-38` (`:5845`), and it is daggered on both rows in
§7.2 (`:8833`, `:8852`) because nobody has written it yet. The rule this atom was about is
therefore correctly attributed and still unimplemented; writing it is the projection suite's, not
this question's.

**Sub-question 2 is untouched.** The 2026-08-13 verdict — *isolated*, from the 37-clause sweep —
stands as recorded above. Nothing in ADR-0030 reopens it; the decision fixes four of the sweep's
rows and routes PS-8, PS-13, PS-28 and PS-29 to their own owners rather than folding them in.
