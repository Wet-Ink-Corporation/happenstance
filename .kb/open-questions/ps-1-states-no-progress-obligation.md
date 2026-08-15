---
id: kb-open-question-ps-1-no-progress-obligation-001
title: PS-1's MUST is a coupling, not a progress obligation
kind: open_question
status: superseded
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
  Amended 2026-08-13: the 37-clause pairing sweep answers sub-question 3 - isolated, at 29 sound,
  7 defective and 1 undetermined, with only PS-1 and PS-19 involving §4.11's table - and refutes
  this body's claim that no clause's MUST states progress, since PS-23's "one commit advances
  exactly one ProjectionId" excludes zero, on a PROVISIONAL clause about fan-out scope. The PS-1
  pairing defect itself reproduces independently, so what changes is the candidate repair, now
  scoped as which clause states progress and which rules rest on it; three rules (PS-8, PS-21,
  PS-22) rest on the missing obligation, and the same intent-not-sentence habit recurs outside the
  table on PS-29, which ADR-0019 (kb-decision-0019) names in its own range. Sub-questions 1, 2 and
  4 stay open, owner unchanged.
  Resolved 2026-08-15 by ADR-0030 (kb-decision-0030), which mints PS-38 rather than widening PS-1:
  a successful commit MUST advance id's checkpoint to position. Sub-question 1 is answered a clause
  of its own, sub-question 2 by reattributing commit_advances_the_checkpoint to PS-38, and
  sub-question 4 by the decision itself; PS-1's own text is byte-identical across it.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-post-phase-reconciliation-001
  - kb-open-question-projection-batch-no-apply-001
  - kb-decision-0017
  - kb-decision-0019
  - kb-decision-0030
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/2026-08-13-ps-clause-pairing-sweep.md
  - .kb/_intake/2026-08-13-adr-0019-apply-failure.md
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/evaluation/ps-clause-pairing-sweep.md
  - references/adr/0019-what-happens-when-apply-fails.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-08-15
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

## Amended 2026-08-13 — sub-question 3 answered; one sentence above refuted; the repair rescoped

The paragraphs above are what was known on 2026-08-10 and are left as written, per this layer's
README. Two documents reached this question on 2026-08-13 and both land here.

**Sub-question 3 is answered: isolated.** The scan it asks for was run over all 37 `PS` clauses
against a threshold declared before the count (`references/evaluation/ps-clause-pairing-sweep.md`,
pinned to `2136dde`): 29 `sound`, 7 `defective`, 1 `undetermined`. Three of the seven are
*independent* — an exposing implementation that satisfies every other `PS` `MUST` — and only two of
those three, this one and PS-19 (`kb-open-question-ps-19-scope-narrower-001`), involve a rule
§4.11's table introduced. The hypothesis that the table was populated from one systematic
assumption is **not supported**: the prior cannot be confirmed by the observation that raised it,
and no third case was found inside the table. The scope of the phase-6 ADR should therefore be
three point repairs — PS-1, PS-19, PS-29 — plus one recorded lesson, which is the shape phase 6's
budget already assumes.

**One sentence above is refuted.** *What is true today* says (`:48-51`): "That a successful commit
*advances* anything … is stated by no clause's MUST anywhere in the document." Read literally
against today's clause text, **PS-23 states it**: "One `commit` advances exactly one
`ProjectionId`" (`spec/SPECIFICATION.md:5317-5318`). "Exactly one" excludes zero, so a `commit`
that advances nothing violates PS-23's `MUST`. The obligation is not absent — it is **misfiled**,
twice over. It sits on a clause about fan-out *scope*, whose `Rejects` field names "an adapter with
a single-row checkpoint table" (`:5327-5329`), which is the "not more than one" reading; progress
arrives through the word "exactly", almost certainly unintended. And it sits on a `[PROVISIONAL]`
clause (`:5318-5323`), falsified by a pair of read models that must be mutually consistent at every
observable instant — so the only normative statement that a commit makes progress is scheduled to
be rewritten by an unrelated question.

**The finding itself survives.** PS-1's pairing defect reproduces independently from
`spec/SPECIFICATION.md:4733-4759`: the per-handle backing store the sweep describes satisfies
PS-1's `MUST` through the "or not at all" arm, passes `commit_is_atomic_with_the_read_model`, and
fails `commit_advances_the_checkpoint`. What changes is the **candidate repair**. "Add a sentence
to PS-1" is now one of at least three candidates, alongside minting a clause and splitting PS-23's
two readings so progress lands where it is already half-written — and the question is better scoped
as *which clause states that a successful `commit` advances the checkpoint, and which rules rest on
it*, because three further rules do: `rollback_leaves_both_unchanged` (PS-8),
`commit_accepts_a_position_the_batch_did_not_write` (PS-21) and `commit_rejects_a_regressing_position`
(PS-22) each enforce something no clause's `MUST` states.

**The shape recurs outside the table, which is the lesson.** PS-29 carries exactly PS-1's shape — a
`[FROZEN]` sentence narrower than the rule written beside it — on
`one_poisoned_projection_does_not_stall_the_others`, a rule that lives only in PS-29's own clause
body and is not one of §4.11's seventeen. ADR-0019 (`kb-decision-0019`) met that row inside its own
clause range, named it, scoped it out and repaired nothing, reaching the same conclusion by an
independent route: the cause is a habit of writing the rule to the clause's *intent* rather than to
its *sentence*, distributed across the family, not one table's artefact.

Nothing here resolves the question. Sub-questions 1, 2 and 4 stay open, `status` stays `accepted`,
and the owner is unchanged — phase 6, with the repair itself
`unstable-projection-gate-and-clause-disposition`'s under
`kb-playbook-repair-frozen-clause-001`'s discipline.

## Resolved 2026-08-15 — a clause of its own; `status` superseded

Everything above is the state of knowledge on 2026-08-10 and 2026-08-13 and is left exactly as
written, per this layer's README. ADR-0030, "The checkpoint reports the commits that happened"
(`kb-decision-0030`; full record at
`references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md`), now holds the answer, so
this atom moves to `superseded` rather than `withdrawn`: the question was worth asking, a later atom
answers it, and a reader arriving here needs sending there.

**Sub-question 1 is answered: a clause of its own.** PS-38 is minted `[PROVISIONAL]` in §4.7
(`spec/SPECIFICATION.md:5447-5449`) — *a successful `commit(batch, id, position, authority)` MUST
advance `id`'s checkpoint to `position`, and a `ProjectionId` no successful `commit` has named MUST
read as `Checkpoint::NeverRun`*. PS-1's own `MUST` is **byte-identical** across it (`:4755-4757`),
which is what makes this a repair rather than a widening: no implementation gains or loses
conformance by anything the decision writes. Widening PS-1 lost on two counts — it changes the set of
implementations a `[FROZEN]` clause admits, which is a gap and a decision's rather than an edit's
(`.kb/decisions/README.md:20-22`), and it would put two independently falsifiable propositions,
coupling and progress, in one sentence. Splitting PS-23's *"exactly one"* — the third candidate the
2026-08-13 amendment raised — lost because that clause is `[PROVISIONAL]` on an unrelated fan-out
falsifier, and an obligation three rules rest on cannot live where something else is scheduled to
rewrite it.

**Sub-question 2 is answered: yes, and independently of where the prose lives.**
`commit_advances_the_checkpoint` now reads against `PS-1, PS-38` in §4.11's table
(`spec/SPECIFICATION.md:5846`) — the progress half rests on PS-38, the coupling half still on PS-1
(`:4758-4759`). `fresh_projection_has_no_checkpoint` moves the same way, to `PS-19, PS-38`
(`:5845`), and renders `†` because it is still unwritten. Of the three further rules the amendment
named, PS-21 and PS-22 now have a clause to cite; **PS-8 does not** — its row is recorded in its own
clause and routed to ADR-0017's range, not repaired here.

**Sub-question 4 is answered by the decision itself.** ADR-0030 depends on `kb-decision-0007`,
`kb-decision-0017` and `kb-decision-0018` and names `kb-decision-0019` as related, but settles none
of them; the PS-29 row ADR-0019 met inside its own range stays with ADR-0019's typed-layer deferral.
The 2026-08-13 correction above travels with the repair and is *stated* in PS-1's clause rather than
edited into it (`spec/SPECIFICATION.md:4762-4769`), which is the same discipline that kept the
sentence refuted-in-place here.

**What is open is PS-38's, not this atom's.** PS-38 is provisional against a store answering
`checkpoint` from a replica that may lag its own `commit`, owned by the first projection adapter
over storage this workspace does not control. That is a new question in the decision's keeping.
