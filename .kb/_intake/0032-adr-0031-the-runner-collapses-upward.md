# Staged: the projection runner collapses upward into `happenstance`

**Staged 2026-08-16** for the next `/redkiln:kb-ingest` wave. **Not an atom.**

This document supersedes an **accepted** decision, so the ingest must write a *new*
atom rather than edit the old one: `.kb/decisions/0007-projection-runner-decodes.md`
is immutable and `redkiln validate --kb` checks it against `HEAD`
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md:50-59`).

Long-form record to amend in the same wave:
[`references/adr/0007-projection-runner-decodes.md`](../../references/adr/0007-projection-runner-decodes.md).
That file is **not** immutable and may be edited directly; the atom may not.

Two clauses are waiting on this document and cite it by filename:
`spec/SPECIFICATION.md`'s **PS-33** (the falsifier, now fired) and **PS-32** (the
Context correction, recorded as owed since phase 6). One wave discharges both,
because both are about the same three sentences of the same ADR.

---

## The falsifier fired, and it was evaluated rather than left to lapse

ADR-0007 set its own falsifier and named the exit that would evaluate it:

> Falsifier: if the core pump has acquired no caller but the typed one when phase 7
> exits, collapse it upward and supersede this decision.
> — `.kb/decisions/0007-projection-runner-decodes.md:23-25`

Phase 7 is the typed layer, and it has exited: `happenstance::Projection` and
`happenstance::run_projection` landed behind `unstable-projection` in
`crates/happenstance/src/runner.rs`.

**The count, taken over the tree rather than recalled.** `happenstance-core`
publishes exactly two module-level free functions in the whole crate:

```
crates/happenstance-core/src/store.rs:285:pub async fn collect<S, T, E>(stream: S) -> Result<Vec<T>, E>
crates/happenstance-core/src/store.rs:321:pub async fn read_decision_model<S>(
```

Neither is a checkpoint pump. **There is no pump function in the contract crate at
all** — the module holds `ProjectionId`, `Checkpoint`, `Authority`, `CommitError`,
`ResetError`, the `ProjectionStore` port, `ProjectionProbe` and, behind `memory`,
the reference store. Nothing that runs.

So the count is not "zero callers of an existing pump". It is stronger: the pump was
allocated to `happenstance-core` by an ADR, three phases passed, and it was never
written — because at every point the thing an application actually needed was the
typed runner, and the typed runner can drive the port directly. Both readings fire
the same falsifier, and the ADR's own instruction is unambiguous about what follows.

## Decision to record

**One runner, in `happenstance`.** `happenstance-core` keeps the `ProjectionStore`
port, the transactional invariant stated on its module doc, and nothing that runs.
`happenstance::run_projection` is the only runner: it reads the checkpoint, derives
the query, streams the replay, decodes through `Codec`, applies in chunks, and hands
each chunk's write set and last applied position to the port's single `commit`.

**What survives from ADR-0007, untouched.** The discriminator — encoding, not
orchestration — and all three shape decisions that rode with the split: a projection
nominates its events with `Query`, `Projection::Store` is an associated type, and
checkpoints stay per `(store, ProjectionId)`. Every one of them is implemented as
written. What is superseded is *only* the allocation of a checkpoint pump to
`happenstance-core`, which is the half the falsifier was aimed at.

**The cost, stated rather than waved past.** ADR-0007 rejected this alternative for
one reason and it is still the true reason: *"it puts the checkpoint invariant in a
crate that sits above the port that states it."* An adapter author reading
`happenstance-core` alone sees the invariant documented on `ProjectionStore` and no
code enforcing it. That is mitigated by where the enforcement is checkable rather
than by moving it back — the projection conformance suite in `happenstance-testkit`
drives the port through `ProjectionProbe` and can fail a store that commits the two
halves apart, which is a stronger guard than a pump the adapter never calls. The
mitigation is worth naming because it is what makes the collapse safe, and it did
not exist when ADR-0007 was written.

**A second consequence, which is a saving rather than a cost.** The pump's signature
typed its callback's error as the *projection store's*, so a decode failure had no
representable home — the application would have had to forge one into the adapter's
`#[non_exhaustive]` error enum, or panic (E2E-26, `spec/E2E-CASES.md:677-696`). The
collapsed runner has no such problem, because `CodecError` is concrete and
`ProjectionError` carries a decode arm with the position it failed at.

## Correcting ADR-0007's Context in the same wave (PS-32)

PS-32 has recorded since phase 6 that one sentence of ADR-0007's Context is wrong and
that correcting it is a superseding atom's job:

> The runner ADR-0006 relocated therefore cannot be written against the port as it
> stands — in either crate.
> — `references/adr/0007-projection-runner-decodes.md`, Context

It can. A callback-driven pump was compiled with a real body during the pressure
test, because the closure's caller knows the concrete `Batch`; what could not be
written against the port as it stood was the *conformance suite*, which is a
different claim and the one that was actually load-bearing. The superseding atom
should carry the correction so that a reader who follows PS-32's citation lands on a
document that says what is true, and `references/adr/0007-projection-runner-decodes.md`
should be amended directly in the same wave.

## What must not happen in the ingest

- **Do not edit `.kb/decisions/0007-projection-runner-decodes.md`.** It is accepted
  and immutable; write a new atom whose `supersedes` names it, and set the old atom's
  `superseded_by` through the CLI rather than by hand.
- **Do not widen the supersession.** ADR-0006's naming decision and ADR-0007's three
  shape decisions are untouched. Only the pump allocation moves.
- **Do not delete PS-32 or PS-33 from `spec/SPECIFICATION.md`.** Both IDs are
  retained as `[NON-NORMATIVE]`; deleting either breaks citations in `RUNBOOK.md`
  and in §7.3.

## Provenance

Produced by `typed-layer-and-alpha-release/projection-clause-verdicts` (HS-S0027),
the story that owns PS-33's evaluation, from the tree left by
`typed-layer-and-alpha-release/projection-trait-and-runner` (HS-S0026). The counts
above are re-derivable and are held by an executed test at
`crates/happenstance/tests/projection_clauses.rs`, which fails if a pump appears in
the contract crate after this document was written.
