# Gate: Intake

REQUIRED checklist to leave the intake stage. Every box must be ticked in the
stage's produced artifact (`_intake-brief.md`) before `redkiln advance` will pass.

**Each box below is one line, deliberately.** The engine enumerates the required
items out of *this* file and matches them against the artifact, and it parses a
checklist line-by-line — so a box whose text wraps is required under its truncated
first line and can never be matched. Rationale goes in the prose under the list,
never inside a box.

- [ ] The problem and desired outcome are stated.
- [ ] Constraints and non-goals are recorded.
- [ ] Open questions are captured for distillation.
- [ ] The proof artefact is named, and it would not exist if the design were wrong.
- [ ] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [ ] The clauses this work discharges or amends are listed by id.
- [ ] Where this brief and `SPECIFICATION.md` disagree, the specification wins.

## Why the last four exist

They are `CLAUDE.md`'s and `RUNBOOK.md`'s standing bars, moved to the moment they
are cheap to satisfy. Each was violated once and cost a phase.

**The proof artefact.** Not "the gate is green" — a green gate is a precondition
for *looking* at the exit criteria, never one of them. Four of the previous
runbook's fifteen proof artefacts would have existed unchanged had the design under
test been wrong; the worst was a skeleton of `todo!()` bodies, which type-checks
against any signature because `!` coerces to everything.

**The axis, for a port freeze.** A port frozen against one storage shape is shaped
like that shape. Four adapters that all serialise their writers and assign
positions under a lock are one adapter wearing four hats.

**The clauses.** By id, with their maturity markers. A `[FROZEN]` clause changes by
new ADR, never by edit — so name the ADR.

**Who wins.** Where this brief and the specification disagree, the specification
does, and the disagreement is fixed here or recorded as an open question rather
than left for the implementer to arbitrate.
