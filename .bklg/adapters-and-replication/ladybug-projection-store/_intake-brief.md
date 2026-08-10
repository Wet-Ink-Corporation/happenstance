---
item: HS-P0007
stage: intake
created: 2026-08-10T02:59:46.454Z
updated: 2026-08-10T02:59:46.454Z
template_sig: 56ad54cb
rendered_sig: a6bc9332
---

# Intake Brief — Phase 11: Ladybug projection store

## Problem

Phase 6 freezes `ProjectionStore` against two implementations — an in-memory oracle
and a rusqlite transaction. Both are SQL-shaped or memory-shaped. A graph store is
neither, and if the freeze is wrong about batch shape this is where it shows.

## Desired Outcome

The projection suite green against a non-SQL batch, and a written verdict on whether
phase 6's freeze held.

## Constraints

- **Phase 6 first** — there is nothing to implement against until the port is frozen.
- **Projection store only.** This crate implements no `EventStore`.
- **Non-goal.** Not on the 0.2.0 path.

## Open Questions

- Checkpoint placement, how a projection expresses graph mutations, and the blocking
  API. (ADR-0025)

## Proof artefact

**The projection suite green on a non-SQL batch, and a written verdict on whether
phase 6's freeze held.**

The verdict is the artefact, not a formality. A phase that can only report success
cannot report that the freeze was wrong — and "the freeze did not hold" is a
legitimate outcome here that must have somewhere to be written down.

## Clauses

Tests **PS-\*** as frozen at phase 6 against the batch shape furthest from the two it
was frozen against.

## Gate: Intake

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/intake.md` and will not leave `intake` until
every one is ticked.

- [x] The problem and desired outcome are stated.
- [x] Constraints and non-goals are recorded.
- [x] Open questions are captured for distillation.
- [x] The proof artefact is named, and it would not exist if the design were wrong.
- [x] For a port freeze: the axis it is most likely to be wrong about is named, and something in the workspace sits at the other end of it.
- [x] The clauses this work discharges or amends are listed by id.
- [x] Where this brief and `SPECIFICATION.md` disagree, the specification wins.
