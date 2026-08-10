---
item: HS-P0009
stage: intake
created: 2026-08-10T02:59:47.349Z
updated: 2026-08-10T02:59:47.349Z
template_sig: 56ad54cb
rendered_sig: f23fe077
---

# Intake Brief — Phase 14: Retention, deletion and completeness

## Problem

Every clause so far assumes a store holds its whole log. Real stores do not: retention
policies, crypto-shredding and lawful deletion all produce a store that holds only a
suffix, or a log with holes that are not the gaps VT-11 permits. §3.7 says what such a
store may look like; nothing implements it, and nothing fails against it.

## Desired Outcome

A completeness story that is checkable — a store that legitimately holds only part of
its own log, and a runner that notices and fails loudly instead of silently producing
a wrong read model.

## Constraints

- **Phase 13 first.** Retention across a peer set is a different question from
  retention in one store, and the second is meaningless without the first.
- **Failing loudly is the requirement.** A projection built from a truncated log is
  wrong in a way no type can catch, so the failure has to be at the seam.
- **A deliberate crypto-shred needs a fourth failure option** — "skip and record" —
  that neither retry, halt nor dead-letter covers.

## Open Questions

- What a store that has been deleted from may promise, and what a reader may assume.
  (ADR-0028, §3.7)
- How retention interacts with a peer set that has already replicated the deleted
  events.

## Proof artefact

**A store that holds only a suffix of its own log, and a runner that fails loudly
against it.**

A runner that keeps going is the wrong implementation this phase exists to reject,
and it is the default behaviour of every runner written so far — so the artefact is
one that could not exist if the design were wrong, rather than one that merely
records success.

## Clauses

Discharges **§3.7**'s completeness clauses and the retention half of §5.10.

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
