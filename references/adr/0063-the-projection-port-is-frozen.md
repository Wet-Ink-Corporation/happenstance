# ADR-0063 — The projection port is frozen, and the gate comes off

- **Status:** proposed
- **Date:** 2026-09-11
- **Phase:** 12, after the `0.2.0` release
- **Acts on:** [ADR-0062](0062-the-probe-seam-moves-and-the-far-end-is-built.md), which met PS-2's bar and deliberately did not take this decision
- **Resolves the decision of:** [ADR-0036](../../.kb/decisions/0036-the-projection-port-ships-gated.md), reaffirmed by [ADR-0060](../../.kb/decisions/0060-ps-2s-axis-re-evaluated.md) — both records said *gate the port*; this one says the reason has been discharged
- **Amends:** PS-3 (retired), PS-4, PS-5 and PS-12 (frozen), PS-34 (retired) in `spec/SPECIFICATION.md` §4; §1.3's census and its account of what `[FROZEN]` means for a `PS` clause
- **Evidence:** `crates/happenstance-postgres/tests/live_projection.rs`, 20 of 20 against a live PostgreSQL; `crates/happenstance-postgres/tests/projection.rs`, 21 of 21; ADR-0062's record of what the two ends disagreed about

## The question

PS-3 said the port SHOULD ship behind `unstable-projection` *until PS-2's bar is
met*. ADR-0062 met it: the probe seam and `begin` moved, `happenstance-postgres`
built a store whose batch is a `sqlx` transaction, and it passed all seventeen
rules with `READS_THROUGH_BATCH = true` declared truthfully — the two ends of
the batch-shape axis, distinguishable to the suite for the first time, and
disagreeing about nothing the port had to move for. ADR-0062 then stopped,
because lifting the gate is a semver promise on a published crate and the thing
it had found one layer up — `Projection::apply` is synchronous — belonged in the
record that made the promise, not in one about a probe seam.

**Does the gate come off, on what exactly, and what does the typed layer do?**

## What is being promised, precisely

`happenstance-core`'s `ProjectionStore`, `SendProjectionStore`, `Batch`'s
ownedness, `begin`/`checkpoint`/`commit`/`reset`/`rollback` as they stand after
ADR-0062, the value types `ProjectionId`, `Checkpoint`, `Authority`,
`CommitError`, `ResetError`, and `MemoryProjectionStore` behind `memory`. Also
`ProjectionProbe` behind `conformance`, which is inside the promise for the same
reason `happenstance-testkit`'s rules are: an adapter author implements it, and
a changed signature there is a changed obligation.

A signature change on any of these is now a breaking change with a decision
record behind it. That is what *frozen* means in this workspace, and it is the
same thing it means for `EventStore`.

## Decision

### §1 The gate comes off `happenstance-core`

`pub mod projection;` and its re-exports are unconditional. `projection_memory`
is behind `memory` alone, like its event-store twin. `conformance` implies
nothing again — it implied `unstable-projection` only because the probe was
defined inside the module that feature closed.

### §2 The feature name stays, empty

`unstable-projection = []` remains declared on `happenstance-core`, off by
default, gating nothing. Removing a Cargo feature is a breaking change, a
`0.2.0` manifest names this one, and the lift is meant to be a minor. The two
`xtask` tests that held the gate on — one on the manifest, one on the crate root
— are inverted rather than deleted: `the_retired_projection_feature_is_still_declared_and_empty`
and `the_port_is_mounted_unconditionally`, so that re-gating the port or
deleting the name is a red gate rather than a quiet edit.

Every in-tree crate that forwarded `happenstance-core/unstable-projection` stops:
the adapters' `projection-store` flags gate their own modules and forward
nothing, and `no_crate_forwards_the_retired_feature_to_the_contract_crate` holds
that. A forward of an empty feature compiles, which is exactly why a test is
needed — nothing else would notice a manifest telling its reader the port is
gated.

### §3 The typed layer keeps a gate of the same name, for a different reason

`happenstance`'s `unstable-projection` still gates the runner — `Projection`,
`run_projection`, `Progressed`, `ProjectionError` — and no longer forwards to
the contract crate. The contract's projection items are mounted on the facade
unconditionally, as the rest of the contract is.

The reason is the one ADR-0062 found and this record is where it is weighed:
`Projection::apply(&mut self, event, batch: &mut StoreBatch<Self>)` is
synchronous, because buffering a row does not await. A projection can therefore
push into a buffered batch and cannot issue a statement into a batch that is a
live transaction — so the far end the port was just proved against is one the
runner cannot drive for a projection that writes rows. Freezing `apply` as it
stands would be freezing a shape that has not met its own far end, which is the
mistake ADR-0060 refused to make for the port. The runner's feature keeps its
name because renaming a feature is as breaking as removing one, and the name
still says the true thing: the surface behind it makes no semver promise yet.

### §4 The clauses that move, and the ones that do not

- **PS-3** is `[NON-NORMATIVE]`, discharged and retired. Its falsifier — PS-2's
  bar met *before* 0.1 — can no longer fire in either direction. The ID stays so
  citations resolve.
- **PS-4** (a batch MUST NOT be *required* to be a live transaction) and
  **PS-5** (`type Batch;`, no lifetime) are `[FROZEN]`. Their falsifier was
  Ladybug, which held at phase 11; the live store then arrived — an adapter
  that *does* acquire a handle before the first write, binding a
  `sqlx::Transaction<'static, Postgres>` as its batch — and needed neither a
  MUST nor a lifetime. Both ends stand on the clause.
- **PS-12** (reads through an open batch reflect its pending writes, or there is
  no read path) is `[FROZEN]`. Both arms now have an adapter over a real
  database: four decline the read path and their rule is a reported skip; the
  live store offers it and its rule ran.
- **PS-34** is `[NON-NORMATIVE]`. Its own marker said *dead the moment
  `type Batch;` lands*; that landed at phase 6, and with PS-5 frozen the
  condition it hangs on cannot recur.
- **PS-6** stays `[PROVISIONAL]` under ADR-0062, against an adapter that must
  reserve something from its server *and* cannot afford the round trip `begin`
  now permits. Nothing names one.
- **PS-9, PS-11, PS-15, PS-16, PS-22 – PS-25, PS-38** stay where they are.
  Each has its own falsifier, stated in its own marker — a second generic
  consumer, a zero-cost instance-naming construction, the first real rebuild —
  and none of them is "the far end of the batch-shape axis". The RUNBOOK's
  ledger row that grouped seven of them under *PS-2 alone* was a simplification
  the clauses' own text never made, and it is narrowed to say so.

§1.3's census moves from 138/46/12/5 to 141/41/12/7, and §7.2 is regenerated
rather than edited.

## What this costs, stated

- **A consumer of the port drops a feature; a consumer of the runner keeps
  one.** `happenstance-core = { version = "0.2", features = ["unstable-projection"] }`
  keeps compiling and now means nothing; the recipe on `ProjectionProbe`'s
  page, the testkit's copy of it and `examples/outside-projection-adapter` all
  write the bare dependency line, and `crates/happenstance-core/tests/projection_recipe.rs`
  fails if the recipe names a feature the port does not need.
- **`cargo-semver-checks` now polices the projection surface.** The next
  breaking change to `ProjectionStore` is a major, not a note in a module
  header.
- **The runner's gate is now the odd one out**, a feature on `happenstance`
  that forwards nothing. That is the honest shape: the instability is the
  runner's, and the crate that owns the runner carries the flag.

## Alternatives rejected

- **Lift both gates.** Freezes `Projection::apply` at a shape proved at one end
  of its axis. The port's own history is the argument against.
- **Rename the typed layer's feature** to something like `unstable-runner`.
  Breaking to every manifest that names it, for a distinction the crate's
  documentation can carry.
- **Remove `unstable-projection` from `happenstance-core`** now that it gates
  nothing. Breaking to every `0.2.0` manifest, for a name that costs nothing to
  keep.
- **Freeze the whole question-1 cluster including PS-6.** PS-6's falsifier fired
  under ADR-0062 and its rewritten MUST is one day old; a clause frozen the day
  it was rewritten has been proved by nothing.
- **Freeze PS-9, PS-11 and PS-15 because the ledger said they waited on PS-2.**
  The clauses win over the ledger, and each names a falsifier that is still
  open.

## Falsifier

This record is reopened by a breaking change to `ProjectionStore` that turns
out to be needed by an adapter the suite passes — which is what a frozen port
being wrong looks like, and which would be a major rather than a quiet edit —
or by `Projection::apply` moving to a shape that requires the port to move with
it. It is not reopened by the runner staying gated for another release: that is
the state §3 chose.
