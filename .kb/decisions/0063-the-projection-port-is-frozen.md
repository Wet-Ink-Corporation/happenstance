---
id: kb-decision-0063
title: The projection port is frozen, and the typed layer keeps a gate of the same name
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0063
reversibility: low
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Recorded from the 2026-09-11 brief for a decision taken on
  lane/projection-probe-seam; at this worktree's HEAD (86a410c) the long form
  the brief names is absent, crates/happenstance-core/Cargo.toml:110 still
  reads conformance = ["unstable-projection"], crates/happenstance/Cargo.toml:142
  still forwards the feature to the contract crate, and SPECIFICATION.md's
  census still reads 138/46/12/5 — this atom states what the lane binds when
  it lands. The decision: the port is frozen. happenstance-core's projection
  module and its re-exports are unconditional, MemoryProjectionStore is
  behind memory alone, conformance implies nothing again, and a signature
  change on ProjectionStore, its value types or ProjectionProbe is now a
  breaking change with a decision record behind it. The feature name stays,
  empty: unstable-projection = [] remains on happenstance-core, off by
  default, gating nothing, because removing a Cargo feature is breaking and a
  0.2.0 manifest names it. The typed layer keeps a gate of the same name for
  a different reason: happenstance's unstable-projection still gates the
  runner and no longer forwards to the contract crate, because
  Projection::apply (crates/happenstance/src/domain.rs:249) is synchronous,
  so the far end the port was just proved against is one the runner cannot
  drive for a projection that writes rows. ADR-0036 and ADR-0060 are
  discharged rather than superseded: both said gate the port until PS-2's bar
  was met, both were right when written, and ADR-0062 met the bar.
depends_on:
  - kb-decision-0062
  - kb-decision-0036
  - kb-decision-0060
  - kb-decision-0017
related:
  - kb-decision-0007
  - kb-open-question-provisional-falsifiers-001
  - kb-open-question-projection-module-exemption-scope-001
  - kb-open-question-projection-runner-chunk-observation-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/2026-09-11-adr-0063-the-projection-port-is-frozen.md
  - crates/happenstance-core/Cargo.toml
  - crates/happenstance/Cargo.toml
  - crates/happenstance/src/domain.rs
  - spec/SPECIFICATION.md
  - RUNBOOK.md
last_reviewed: 2026-09-11
---

# The projection port is frozen, and the typed layer keeps a gate of the same name

## Provenance note

This atom is minted from the 2026-09-11 intake brief for a decision taken on
`lane/projection-probe-seam`. At this worktree's `HEAD` (`86a410c`) the long
form the brief names — `references/adr/0063-the-projection-port-is-frozen.md`
— is not present in this checkout; `crates/happenstance-core/Cargo.toml:110`
still reads `conformance = ["unstable-projection"]`,
`crates/happenstance/Cargo.toml:142` still forwards that feature to the
contract crate, and the census `spec/SPECIFICATION.md` carries still reads
138/46/12/5. This record states what the lane binds when it lands, on the
same footing as `kb-decision-0062`.

## The one question

PS-3 said the port SHOULD ship gated *until PS-2's bar is met*. ADR-0062
met that bar and deliberately stopped there, because lifting a gate on a
published crate is itself a semver promise. Does the gate come off, on
what exactly, and what does the typed layer do?

## The decision

**The port is frozen.** `happenstance-core`'s `projection` module and its
re-exports become unconditional; `MemoryProjectionStore` stays behind
`memory` alone; `conformance` implies nothing again. A signature change to
`ProjectionStore`, its value types, or `ProjectionProbe` is now a breaking
change requiring a decision record — the same thing *frozen* already means
for `EventStore`.

**The feature name stays, empty.** `unstable-projection = []` remains
declared on `happenstance-core`, off by default and gating nothing, because
removing a Cargo feature outright is itself breaking, and a `0.2.0` manifest
names it publicly. Two `xtask` tests that used to hold the gate on are
inverted rather than deleted, and a third test is added asserting that no
in-tree crate still forwards the retired feature — a forward of an empty
feature compiles silently, which is why that test is needed rather than
assumed.

**The typed layer keeps a gate of the same name, for a different reason.**
`happenstance`'s `unstable-projection` still gates the projection runner and
no longer forwards to the contract crate. `Projection::apply`
(`crates/happenstance/src/domain.rs:249`) is still synchronous, so the far
end the port was just proved against — a store whose batch is a live
transaction — is one the runner cannot drive for a projection that writes
rows through it. Freezing `apply` now would freeze a shape proved at only
one end of its own axis, which is exactly the mistake ADR-0060 refused to
make for the port itself; the runner's gate protects the typed layer from
repeating it one layer up.

## Clauses

PS-3 and PS-34 retire to `[NON-NORMATIVE]` with their IDs kept. PS-4, PS-5
and PS-12 freeze, standing on both ends of the axis now built. PS-6 stays
`[PROVISIONAL]` under ADR-0062, whose rewritten MUST it now names. PS-9,
PS-11, PS-15 and the rebuild cluster stay on their own separate falsifiers —
the RUNBOOK ledger row that had grouped them under "PS-2 alone" was a
simplification the clauses themselves never made, and this decision narrows
it back to what the clauses say. The brief records the census moving to
141/41/12/7.

## Discharged, not superseded

ADR-0036 and ADR-0060 both said *gate the port until PS-2's bar is met*, and
both were correct when written — the bar genuinely was not met. ADR-0062
met it. This is the same shape the corpus has used before (ADR-0037 over
ADR-0004): the earlier decisions are not wrong in retrospect and their
bodies are not touched; the reason they gave has simply run out.

## Rejected

Lifting both gates at once (freezes `apply` unproved); renaming the runner's
feature (breaking, for a distinction documentation can carry instead);
removing the now-empty contract-crate feature (breaking, for a free name);
freezing PS-6 the same day its MUST was rewritten (no time to observe it
holding); freezing PS-9/11/15 on the RUNBOOK ledger's grouping rather than on
their own clause text.

## Falsifier

Reopened by a breaking change to `ProjectionStore` that an adapter the suite
already passes turns out to need, or by `Projection::apply` moving to a
shape that requires the port to move with it. Not reopened by the runner
staying gated for another release.
