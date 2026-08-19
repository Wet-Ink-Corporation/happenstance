---
id: kb-decision-0031
title: One runner, in happenstance — the checkpoint pump collapses upward
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0031
reversibility: medium
phase: 7
supersedes: null
superseded_by: null
summary: >-
  Partly supersedes ADR-0007's runner allocation while leaving its discriminator and all three of
  its shape decisions untouched and standing. ADR-0007 set its own falsifier — if the core pump has
  acquired no caller but the typed one when phase 7 exits, collapse it upward and supersede this
  decision — and it fired in a stronger form than it anticipated: phase 7 exited with
  happenstance-core publishing exactly two module-level free functions, collect and
  read_decision_model at store.rs:285 and :321, neither a pump, and no pump function in the
  contract crate at all. The pump was allocated by an ADR, three phases passed, and it was never
  written, because at every point what an application needed was the typed runner and the typed
  runner drives the port directly. The decision: one runner, in happenstance. happenstance-core
  keeps the ProjectionStore port and the transactional invariant stated on its module doc and
  nothing that runs; happenstance::run_projection reads the checkpoint, derives the query, streams
  the replay, decodes through Codec, applies in chunks, and hands each chunk's write set and last
  applied position to the port's single commit. The cost ADR-0007 named for this alternative is
  still the true one — it puts the checkpoint invariant in a crate above the port that states it —
  and it is mitigated by something that did not exist when ADR-0007 was written: the projection
  conformance suite drives the port through ProjectionProbe and can fail a store that commits the
  two halves apart, a stronger guard than a pump no adapter calls. A second consequence is a
  saving: the old pump typed its callback's error as the projection store's, leaving a decode
  failure no representable home (E2E-26), where the collapsed runner has CodecError concrete and a
  ProjectionError decode arm carrying the failing position. The same record corrects ADR-0007's
  Context per PS-32: a callback-driven pump can be written against the port as it stands, compiled
  during the pressure test; what could not be written was the conformance suite. What is not
  superseded is everything else — the discriminator is encoding not orchestration, a projection
  nominates its events with Query, Projection::Store is an associated type, and checkpoints stay
  per store and projection id. All three are implemented as written, which is why kb-decision-0007
  stays accepted and superseded_by stays null.
depends_on:
  - kb-decision-0007
related:
  - kb-decision-0006
  - kb-decision-0010
  - kb-decision-0017
  - kb-decision-0019
  - kb-decision-0030
  - kb-open-question-ps-32-adr-0007-correction-owed-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/0032-adr-0031-the-runner-collapses-upward.md
  - references/adr/0007-projection-runner-decodes.md
  - references/evaluation/PRESSURE-TEST.md
  - crates/happenstance/src/runner.rs
  - crates/happenstance/tests/projection_clauses.rs
  - crates/happenstance-core/src/store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
---

# One runner, in happenstance — the checkpoint pump collapses upward

## Decision

ADR-0007 allocated a checkpoint pump to `happenstance-core` and set its own falsifier: *if the core
pump has acquired no caller but the typed one when phase 7 exits, collapse it upward and supersede
this decision.* Phase 7 is the typed layer, and it has exited — `happenstance::Projection` and
`happenstance::run_projection` landed behind `unstable-projection` in `crates/happenstance/src/runner.rs`.
The falsifier fired in a stronger form than it names: `happenstance-core` publishes exactly two
module-level free functions in the entire crate — `collect` at `store.rs:285` and
`read_decision_model` at `store.rs:321` — and neither is a checkpoint pump. There is no pump function
in the contract crate at all. The pump was allocated by ADR-0007, three phases passed, and it was
never written, because at every point the thing an application actually needed was the typed runner
driving the port directly.

**One runner, in `happenstance`.** `happenstance-core` keeps the `ProjectionStore` port and the
transactional invariant stated on its module doc, and nothing that runs. `happenstance::run_projection`
is the only runner: it reads the checkpoint, derives the query, streams the replay, decodes through
`Codec`, applies in chunks, and hands each chunk's write set together with the last applied position
to the port's single `commit`. The counts backing this are re-derivable and held by an executed test,
`crates/happenstance/tests/projection_clauses.rs`, which fails if a pump function reappears in the
contract crate.

**The cost, named rather than waved past.** ADR-0007 rejected this same shape for one reason, and
that reason is still true: putting the runner above the port that states the checkpoint invariant
means an adapter author reading `happenstance-core` alone sees the invariant documented on
`ProjectionStore` with no code enforcing it. What changed since ADR-0007 was written is the
mitigation, not the cost: the projection conformance suite in `happenstance-testkit` now drives the
port through `ProjectionProbe` and fails a store that commits the checkpoint and the write set apart
— a stronger guard in practice than a pump function no adapter ever called.

**A saving, not merely a wash.** The old pump's callback typed its error as the projection store's,
so a decode failure had no representable home — the application had to forge one into the adapter's
`#[non_exhaustive]` error enum, or panic (E2E-26). The collapsed runner has no such gap:
`CodecError` is concrete in `happenstance`, and `ProjectionError` carries a decode arm holding the
position the decode failed at.

**PS-32's correction, folded into this same atom.** PS-32 has recorded since phase 6 that one
sentence of ADR-0007's Context is wrong: *"the runner ADR-0006 relocated therefore cannot be written
against the port as it stands — in either crate."* It can. `references/evaluation/PRESSURE-TEST.md`
section 3.4 built the ADR's own indicative callback-driven pump against
`crates/happenstance-testkit/src/projection.rs` unchanged, because the closure's caller — not the
port — knows the concrete `Batch`. What could not be written against the port as it stood was the
**conformance suite**, a narrower and different claim from the one ADR-0007's Context makes, and the
one that was actually load-bearing for the collapse decided here.

## What survives untouched

The discriminator ADR-0006 drew — encoding, not orchestration, is what separates `happenstance-core`
from `happenstance` — and all three shape decisions that rode with ADR-0007's original split: a
projection nominates the events it wants with `Query`, `Projection::Store` is an associated type, and
checkpoints stay keyed per `(store, ProjectionId)`. Every one of the three is implemented as written
and none is touched by this record. Only the allocation of a runnable pump to `happenstance-core` is
superseded — which is why `kb-decision-0007` stays `accepted` with `superseded_by: null` rather than
being flipped: the supersession here is partial, and this repository's convention for a partial
supersession is a `depends_on` edge plus prose explaining the split, not a status flip that would
retire three still-standing, still-implemented decisions along with the one that actually changed.

## Alternatives rejected

Flipping `kb-decision-0007` to `status: superseded` was considered and rejected: the metadata flip is
the only edit an accepted decision atom permits, but doing it here would retire the discriminator and
all three shape decisions along with the pump allocation, one of which (`Projection::Store` as an
associated type) `kb-decision-0030` itself depends on. The flip remains available as a one-edit
reversal if a human reviewer prefers the fuller supersession spelling; it was not taken by default.
