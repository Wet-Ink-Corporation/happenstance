# Gate: Intake

REQUIRED checklist to leave the intake stage. Every box must be ticked in the
stage's produced artifact (`_intake-brief.md`) before `redkiln advance` will pass.

- [ ] The problem and desired outcome are stated.
- [ ] Constraints and non-goals are recorded.
- [ ] Open questions are captured for distillation.

## This repository's additions

The four below are `CLAUDE.md`'s and `RUNBOOK.md`'s standing bars, moved to the
moment they are cheap to satisfy. Each exists because it was violated once and
cost a phase.

- [ ] **The proof artefact is named, and it would not exist if the design were
      wrong.** Not "the gate is green" — a green gate is a precondition for
      *looking* at the exit criteria, never one of them. Four of the previous
      runbook's fifteen proof artefacts would have existed unchanged had the
      design under test been wrong; the worst was a skeleton of `todo!()` bodies,
      which type-checks against any signature because `!` coerces to everything.
- [ ] **For a port freeze: the axis it is most likely to be wrong about is named,
      and something in the workspace sits at the other end of it.** A port frozen
      against one storage shape is shaped like that shape. Four adapters that all
      serialise their writers and assign positions under a lock are one adapter
      wearing four hats.
- [ ] **The clauses this work discharges or amends are listed by id**, with their
      maturity markers. A `[FROZEN]` clause changes by new ADR, never by edit.
- [ ] **Where this brief and `SPECIFICATION.md` disagree, the specification
      wins** — and the disagreement is either fixed here or recorded as an open
      question, not left for the implementer to arbitrate.
