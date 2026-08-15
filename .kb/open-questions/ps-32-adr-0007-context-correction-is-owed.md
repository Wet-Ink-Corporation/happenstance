---
id: kb-open-question-ps-32-adr-0007-correction-owed-001
title: ADR-0007's record overstates what cannot be written, and only a superseding atom may fix it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  PS-32 is FROZEN and states that ADR-0007's Context MUST be corrected: a callback-driven pump can be
  written against the ProjectionStore port as it stands, and what cannot be written is the
  conformance suite. The sentence it rejects is at references/adr/0007-projection-runner-decodes.md
  line 37 — "the runner ADR-0006 relocated therefore cannot be written against the port as it stands,
  in either crate" — and it was falsified by compilation rather than by argument: PRESSURE-TEST
  section 3.4 builds the ADR's own indicative pump against projection.rs unchanged, because the
  callback's caller knows the concrete Batch. What is not decided is who performs the correction and
  in what atom. ADR-0007 is accepted and immutable, and the governance test asks whether an edit
  changes what a document asserts rather than whether it changes the document, so this is a
  superseding decision's act and never an edit; the KB atom kb-decision-0007 does not repeat the
  wrong sentence, so what is defective is the long-form record and the phase-2 work item in
  RUNBOOK.md that derives from it. ADR-0017 records the correction as owed and states its shape
  without performing it, ADR-0030's phase-6 clause disposition changes nothing about that, and this
  is the third wave to carry it forward unperformed. Forced by whoever writes the runner, since the
  sentence being corrected is about work RUNBOOK.md still schedules on its strength.
depends_on: []
related:
  - kb-decision-0007
  - kb-decision-0017
  - kb-decision-0030
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0007-projection-runner-decodes.md
  - references/adr/0017-what-a-projection-batch-owns.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - references/evaluation/PRESSURE-TEST.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-15
---

# ADR-0007's record overstates what cannot be written, and only a superseding atom may fix it

## What is true today

`spec/SPECIFICATION.md` §4.9 states `[FROZEN]` clause PS-32: "ADR-0007's Context MUST be corrected:
a callback-driven pump *can* be written against the port as it stands. What cannot be written is the
conformance suite." The sentence PS-32 rejects lives in the long-form record,
`references/adr/0007-projection-runner-decodes.md:36-38`: "`ProjectionStore::Batch` carries no trait
bounds, so generic code can `begin` a batch and hand it back to `commit`, and cannot write to it.
There is no `apply`. The runner ADR-0006 relocated therefore cannot be written against the port as it
stands — in either crate." That sentence is false, and PS-32 records how it was falsified: not by
counter-argument but by compilation. `references/evaluation/PRESSURE-TEST.md` section 3.4 builds the
ADR's own indicative pump — `checkpoint` → `begin` → per-event `apply` → `commit`/`rollback` — against
`crates/happenstance-testkit/src/projection.rs` unchanged, because the closure's caller, not the
port, knows the concrete `Batch`. The generic-code claim was never tested; the pump was.

## What is not decided

Who performs the correction, and in which atom. ADR-0007 is `accepted` and immutable — `redkiln
validate --kb` checks an accepted decision's body against `HEAD`, so the fix cannot be an edit to
`references/adr/0007-projection-runner-decodes.md`. `.kb/governance/rewrite-the-referent-never-the-
reasoning.md:50-59` states the applicable test: whether a change alters what a document *asserts*,
not whether it touches the file, and correcting a load-bearing factual claim in the Context clears
that bar. The correction is therefore a **superseding** decision atom's act, carrying
`supersedes: [kb-decision-0007]` and a corrected Context, never a hand-edit.

One finding this atom adds past what the intake staged: the KB atom `kb-decision-0007` itself does
not repeat the defective sentence — its frontmatter summary is silent on the generic-code claim, and
only the long-form record and the phase-2 work item in `RUNBOOK.md:204-208` derived from it carry the
error. So what is undecided is narrower than "supersede ADR-0007": it is whether the eventual
superseding atom is a full supersession or a partial one that corrects the record and the runbook item
while leaving the rest of ADR-0007's split-at-the-decode-boundary decision untouched, and it inherits
the corresponding question of whether the same atom, or a different one, is the one that finally
writes ADR-0007's own long-deferred callback-driven pump.

## What forces it

Whoever writes the runner. `RUNBOOK.md`'s phase-2 work item exists *because* of the sentence PS-32
rejects, so the correction is owed the moment someone picks that item up on the strength of a claim
that no longer holds. `ADR-0017` (`kb-decision-0017`) already records the correction as owed and
states its shape at `references/adr/0017-what-a-projection-batch-owns.md:356-361`, without performing
it; `ADR-0030` (`kb-decision-0030`) met the same row in its own clause range and its phase-6 clause
disposition explicitly changes nothing about the correction. This is the third wave in sequence —
0007's own, ADR-0017's, and now ADR-0030's — to carry the obligation forward unperformed, and
`spec/SPECIFICATION.md:5695` names this wave's own intake file as the staging note for whichever wave
finally writes the superseding atom.

## What a future wave should carry

Per `spec/SPECIFICATION.md`'s own §4.9 disposition: what cannot be written against the port as it
stands is the **conformance suite**, not the runner — §4.3 is where that distinction lives, and the
superseding atom should correct the Context to say exactly that rather than simply deleting the false
sentence.
