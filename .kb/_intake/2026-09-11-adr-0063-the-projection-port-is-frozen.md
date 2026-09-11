# ADR-0063 is written and wants an atom

**Date:** 2026-09-11
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Long form:** `references/adr/0063-the-projection-port-is-frozen.md`
**Acts on:** ADR-0062, which met PS-2's bar and deliberately did not take this decision
**Resolves the decision of:** ADR-0036 and ADR-0060 — both said *gate the port*; this record says the reason is discharged. Neither is superseded in its reasoning, which was right when written.
**Amends:** PS-3 retired, PS-4 / PS-5 / PS-12 frozen, PS-34 retired; §1.3's census 138/46/12/5 → 141/41/12/7
**Depends on:** kb-decision-0062 (to be minted from `2026-09-10-adr-0062-the-probe-seam-moves.md`), kb-decision-0036, kb-decision-0060, kb-decision-0017

## The one question

PS-3 said the port SHOULD ship gated *until PS-2's bar is met*. ADR-0062 met it
and stopped, because lifting the gate is a semver promise on a published crate.
**Does the gate come off, on what exactly, and what does the typed layer do?**

## The decision

**The port is frozen.** `happenstance-core`'s `projection` module and its
re-exports are unconditional; `MemoryProjectionStore` is behind `memory` alone;
`conformance` implies nothing again. A signature change on `ProjectionStore`,
its value types or `ProjectionProbe` is now a breaking change with a decision
record behind it — the same thing *frozen* means for `EventStore`.

**The feature name stays, empty.** `unstable-projection = []` remains on
`happenstance-core`, off by default, gating nothing: removing a Cargo feature is
breaking and a `0.2.0` manifest names it. The two `xtask` tests that held the
gate on are inverted rather than deleted, and a third holds that no in-tree
crate still forwards the retired feature — a forward of an empty feature
compiles, which is why a test is needed.

**The typed layer keeps a gate of the same name for a different reason.**
`happenstance`'s `unstable-projection` still gates the runner and no longer
forwards to the contract crate. `Projection::apply` is synchronous, so the far
end the port was just proved against is one the runner cannot drive for a
projection that writes rows; freezing `apply` now would freeze a shape proved
at one end of its axis, which is the mistake ADR-0060 refused for the port.

**Clauses.** PS-3 and PS-34 retired (`[NON-NORMATIVE]`, IDs kept); PS-4, PS-5,
PS-12 frozen on both ends of the axis standing on them; PS-6 stays provisional
under ADR-0062; PS-9, PS-11, PS-15 and the rebuild cluster stay on their own
falsifiers — the RUNBOOK ledger row that grouped them under *PS-2 alone* was a
simplification the clauses never made, and is narrowed.

## Alternatives rejected

Lifting both gates (freezes `apply` unproved); renaming the runner's feature
(breaking for a distinction docs can carry); removing the empty feature from
the contract crate (breaking for a free name); freezing PS-6 the day its MUST
was rewritten; freezing PS-9/11/15 on the ledger's word against the clauses'.

## Falsifier

Reopened by a breaking change to `ProjectionStore` an adapter the suite passes
turns out to need, or by `Projection::apply` moving to a shape that requires the
port to move with it. Not reopened by the runner staying gated another release.
