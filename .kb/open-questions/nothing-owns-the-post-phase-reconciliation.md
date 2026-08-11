---
id: kb-open-question-post-phase-reconciliation-001
title: Nothing owns the specification reconciliation at a phase's exit
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Phases 4 and 5 were the two largest changes to the contract in the plan, and neither carried an
  item obliging anyone to read spec/SPECIFICATION.md back against the tree it had just changed;
  the cost was measured in August 2026 by an unscheduled reconciliation pass. The rule that would
  have caught it was already written in RUNBOOK.md three times and is implemented nowhere. The
  pass added a standing exit criterion, and it is attached to no phase and to no tool: what is
  open is whether it becomes a per-phase checkbox, a gate step, or both; who computes a phase's
  clause range against the union of its ADRs' ranges when neither is machine-readable; and what a
  disagreement between the two numbers obliges. Forced by phase 6's exit, which freezes
  ProjectionStore and discharges PS-1 through PS-37, and secondarily by first publish at phase 12.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-ps-1-no-progress-obligation-001
  - kb-open-question-ps-19-scope-narrower-001
  - kb-open-question-provisional-falsifiers-001
source_paths:
  - .kb/_intake/open-question-nothing-owns-the-post-phase-reconciliation.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - references/evaluation/phase-4-5-reconciliation.md
last_reviewed: 2026-08-10
---

# Nothing owns the specification reconciliation at a phase's exit

## What is true today

`spec/SPECIFICATION.md` holds 200 numbered clauses stating what is true of the contract now.
Phases 4 and 5 were, by the runbook's own account, "the two largest changes to the contract in
the plan." Neither carried an item obliging anyone to read the specification back against the
tree it had just changed.

The consequence was measured in August 2026 and is recorded in `RUNBOOK.md` under the heading
"Between 5 and 6 — the reconciliation nothing owned," which opens: "Not a phase. A pass that had
to happen and that this plan had not scheduled, recorded here so the next one is scheduled rather
than noticed." The counted defects that pass found are the census owned by
`kb-reference-phase-4-5-spec-reconciliation-001`; this atom does not restate them — a question
carrying its own copy of a census is a second census, and the two would drift.

**The rule that would have caught this was already written down, in the same file, and nothing
implements it.** Stated three separate times in `RUNBOOK.md`: "The rule this leaves behind, for
every later phase: a phase's clause range and the union of its ADRs' clause ranges are two
numbers, and nothing checks that they are equal. Compute both at the phase's exit." Three
appearances and zero enforcement is itself part of the finding — a rule stated only in prose has
the same survival odds whether written once or three times.

The reconciliation pass responded with a standing exit criterion in the runbook, applying to every
phase from 6 onward before its box is ticked: every clause a phase's ADRs discharge has been read
against the code as it now stands, not as it stood when written; the phase's clause range and the
union of its ADRs' clause ranges are computed and compared; and `cargo xtask spec-trace`'s
citation count has not fallen, with any clause the phase froze naming a rule that exists or is
marked `†`.

## What is not decided

**The criterion exists as prose in one section of the runbook and is attached to no phase and to
no tool.** Three separable questions follow:

Whether it becomes an item in each phase's own work list — phase 6's section enumerates its work
as checkboxes and does not carry these three, so a rule stated once, several thousand lines above
the phase that must obey it, has today's same enforcement profile as the rule that already failed.

Whether any of it is mechanisable in `cargo xtask spec-trace`. The citation-count check is
closest: the tool already prints the count, so "has not fallen" needs only a stored baseline, and
`†` handling already exists. The clause-range comparison becomes checkable only once phase and ADR
clause ranges are written in a parseable form; today each lives as prose, so the comparison is a
human diff of two paragraphs — skipped precisely when the phase is large, which is when it matters
most. The read-the-clauses-against-the-code bullet is probably not mechanisable at all, which
argues for mechanising the other two so human effort lands only where it is required.

What a disagreement between the two computed numbers obliges. The runbook says "computed and
compared" and does not say what happens next; a comparison with no stated consequence is a
comparison that stops being run.

## What forces it

**Phase 6's exit.** Phase 6 freezes `ProjectionStore` and discharges PS-1 through PS-37 — more
clauses than any prior phase, in the layer already carrying two recorded MUST-versus-rule gaps
(see `kb-open-question-ps-1-no-progress-obligation-001` and
`kb-open-question-ps-19-scope-narrower-001`). If the exit criterion is not attached to something
concrete before phase 6 closes, the same unscheduled reconciliation runs again, at larger scale.

A secondary forcing event is first publish at phase 12, when the specification becomes a promise
to downstream consumers rather than an internal document. A clause describing a superseded
implementation is a documentation defect today and a support burden then.

## Ordered sub-questions

1. Does the standing criterion become a per-phase checkbox, a gate step, or both? Logically first
   because it determines whether the remaining questions are about wording or about code.
2. If a gate step: where does the citation-count baseline live? A committed number is a
   self-referential count, which `xtask/src/spec_trace.rs` already records as its own known
   failure mode for a different count — the array-length comment that cannot come to disagree
   with the array beneath it. `kb-playbook-ratchet-gate-landing-001`'s rule against ever writing a
   count beside a list applies directly here.
3. Are phase clause ranges and ADR clause ranges made machine-readable? This is what would turn
   the range comparison from a diff of two paragraphs into an assertion a tool can run.
4. What is the obligation when the two numbers disagree — block the phase's exit, or merely flag
   it for a human to adjudicate?
5. Does this become its own ADR, or is it runbook process only? It changes no contract clause and
   admits no new implementation, so it is arguably process; but it is the third place the same
   rule has been written down with zero prior effect, which is itself evidence that prose is the
   wrong instrument for it.

## Why this is a question and not a task

There is a genuine tradeoff inside it: a mechanised gate step cannot check the bullet that matters
most (has a human actually read the clauses against the code), and a per-phase checklist has
already been demonstrated not to survive contact with a large phase. The real answer is probably
some combination whose shape is not yet chosen, and filing this as a plain backlog item would
smuggle that design decision into whoever happened to pick up the ticket.

## Owner

Unassigned.
