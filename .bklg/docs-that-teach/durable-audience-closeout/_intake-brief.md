---
item: HS-P0025
stage: intake
created: 2026-08-17T03:17:22.081Z
updated: 2026-08-17T03:17:22.081Z
template_sig: ab516678
rendered_sig: 5048f76b
---

# Intake Brief — The Durable, Reconciled Audience

## Problem

The problem or opportunity in one paragraph.

## Desired Outcome

What success looks like and how we will know.

## Constraints

Known constraints, dependencies, and non-goals.

## Open Questions

Questions to resolve during distillation.

## Proof artefact

The named artefact this body of work must produce — the one that **would not exist
if the design were wrong**. `cargo xtask ci` being green is a precondition for
looking at the exit criteria, never one of them.

## Clauses

The `SPECIFICATION.md` clause ids this discharges or amends, with their maturity
markers. A `[FROZEN]` clause changes by new ADR, never by edit — name the ADR.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` — where each box's rationale is
written — and will not leave `intake` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
