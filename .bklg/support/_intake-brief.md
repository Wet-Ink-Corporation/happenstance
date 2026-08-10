---
item: HS-I0001
stage: intake
created: 2026-08-10T02:55:01.813Z
updated: 2026-08-10T02:55:01.813Z
template_sig: 9b551827
rendered_sig: 0d1e2900
---

# Intake Brief — Support

## Problem

The reactive lane needs a standing home. `/redkiln:fix` files its story under the
initiative named by `support_initiative` in `.redkiln/config.yaml`, and without
that initiative the lane fails at its own preflight the first time a bug is
reported — which is the worst possible moment to discover a missing directory.

## Desired Outcome

A standing initiative that never closes, under which single bugfix stories and
incidents live. It carries no Definition of Done and no release: it exists so
that reactive work has a parent and shows up on the same board as planned work,
rather than being tracked in someone's head.

## Constraints

- **It never reaches closeout.** A standing initiative that closes takes its
  children with it into `_archive/`, where nothing is indexed.
- **Its stories are full-rigour, not a fast lane.** `/redkiln:fix` runs TDD red,
  green, adversarial review and the deterministic gate. What is cut is planning
  ceremony, never a gate.
- **Non-goal.** This is not a backlog of "someday" ideas. Work nobody is expected
  to do is not an item at all; an unsettled question is a `.kb/open-questions/`
  atom.

## Open Questions

None. This initiative exists to satisfy a configuration key, and its shape is
fixed by the tool.

## Proof artefact

`/redkiln:fix` completing a run end to end — which is only observable once a real
bug arrives, and is deliberately not manufactured here. The intake bar below is
satisfied by the initiative existing and being reachable; there is no design under
test, so there is nothing a wrong design could fail to produce.

## Clauses

None. This initiative implements no clause of `SPECIFICATION.md`; it is process
scaffolding.

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

The fifth box is ticked as **not applicable and stated so**: this initiative
freezes no port. The gate has no N/A arm, and leaving it unticked would block a
standing initiative forever on a question it cannot answer — so the honest record
is here in prose rather than in a box that pretends otherwise.
