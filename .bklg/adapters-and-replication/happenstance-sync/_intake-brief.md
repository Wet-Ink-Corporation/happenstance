---
item: HS-P0008
stage: intake
created: 2026-08-10T02:59:46.919Z
updated: 2026-08-10T02:59:46.919Z
template_sig: 56ad54cb
rendered_sig: 6c46d959
---

# Intake Brief — Phase 13: `happenstance-sync` and its testkit

## Problem

`SequencePosition` is meaningful only within one store, so positions cannot be
replicated as-is — and whether ingest re-checks append conditions is the central
question nothing has answered. §5 carries an indicative shape and a bound decision
that ingest never rejects, with the falsifier still owed. The port crate is a
skeleton and its conformance suite, `happenstance-sync-testkit`, does not exist.

## Desired Outcome

A replication port proved the only way a port can be: two unlike implementations and
an oracle. Its own conformance suite, and a payload that survives the round trip
byte-identically.

## Constraints

- **This needs three real stores at once**, which is why it depends on phases 8, 9,
  10 and 12 and cannot be parallelised the way 9, 10 and 11 can.
- **Positions do not cross a store boundary.** Anything that replicates one is wrong
  by construction, whatever it measures.
- **The wire format is frozen and private to happenstance** (ADR-0016), which is what
  makes every reversal in it free rather than breaking.
- **`happenstance-sync` stays out of the contract crate**, so that publishing
  `happenstance-core` never waits on replication.

## Open Questions

- Does ingest re-check append conditions? (`.kb/open-questions/replication-semantics.md`)
- Are hub-and-spoke and peer-to-peer one abstraction or two?
- Where does a per-peer watermark live?
- The peer port's own shape, and `SyncError`. (ADR-0026, ADR-0027)

## Proof artefact

**One suite green against three peers, two of them unlike, and a byte-identical
payload round trip.**

Two unlike implementations and an oracle is the minimum that proves a port rather
than a program. The byte-identical round trip is what discharges ADR-0003's remaining
provisional half: the payload has been opaque all along, and this is the first time
anything carries one across a boundary and back.

## Clauses

Discharges **SY-1 – SY-30** and ADR-0003's `provisional` marker.

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
