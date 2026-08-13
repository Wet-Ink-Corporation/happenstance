---
item: HS-P0014
stage: intake
created: 2026-08-12T03:23:12.009Z
updated: 2026-08-12T03:23:12.009Z
template_sig: ab516678
rendered_sig: 9ef3158c
---

# Intake Brief — The two stores that disagree with the port

## Problem

A port is only as well-designed as the *spread* of what implements it.
`MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all serialise
their writers and assign positions under a lock — four adapters, one storage
shape, and a port frozen against them is frozen against SQLite wearing four hats
(`RUNBOOK.md:669-671`, `695-698`). Postgres sits at the other end of the
position-allocation axis: `nextval()` allocates outside the transaction, so a
Postgres store violates the visibility invariant by construction unless it does
something about it. Neon sits at the other end of the transport axis: no
connection, no interactive transaction, no cursor.

## Desired Outcome

Both far ends are passing implementations, not instruments. We know it worked when
`event_store_conformance!` is green against `happenstance-postgres` including the
concurrency macro on a store that does **not** serialise its writers, with the
position-visibility cost **measured rather than estimated** (DoD 5); and green
against `happenstance-neon` — no connection, no interactive transaction, no cursor
— or the contract is amended by decision record and the suite re-run (DoD 6). Neon
also keeps its `wasm32` build green, because it claims that target in its own
documentation.

## Constraints

- **Depends on** `projection-store-freeze`. Independent of the other two adapter
  projects — 9, 10 and 11 touch disjoint crates and parallelise freely
  (`RUNBOOK.md:239-242`).
- **One project, not two** — decided at the decomposition gate. Neon *is* Postgres
  over one-shot HTTP: it inherits migration 1, tag storage and the append-condition
  SQL, and both are settled by one ADR pass (ADR-0024). Splitting would create a
  horizontal seam (schema below, transport above) rather than a vertical one.
- **The mechanism is measured; the structural costs are not.** `xid8` +
  `pg_snapshot_xmin` benchmarked at 0.99–1.03× baseline
  (`experiments/position-visibility/`), but this project pays the structural bill:
  `head` becomes a frontier, read-your-own-writes does not hold, and staleness is
  bounded by the longest write transaction **in the cluster**.
- **The Neon endpoint cannot be faked.** Substituting a pooled Postgres connection
  destroys the exact axis the adapter exists to test.
- **Never assert on literal position values** — this is the project where gaps stop
  being hypothetical.
- **Non-goals**, each naming its owner: durability and reopen far ends →
  `sqlite-durable-store`; the completeness axis (a store holding only a suffix) →
  `retention-and-incomplete-logs`; whether ES-10 should have been per-boundary
  rather than global — settled at phase 4 in `happenstance-core`, and reopening it
  is a new decision atom and a re-plan, not this project's.

## Open Questions

- **ADR-0024** — how the adapter buys the ES-10 visibility invariant when
  `nextval()` allocates outside the transaction. `xid8` + `pg_snapshot_xmin`,
  transaction-scoped advisory locks, and a serialised sequence table each cost
  something real.
- **`conflicting_position` — a promise every adapter owes, or a hint one may omit?**
  Neon is the forcing case: with no cursor and no interactive transaction, the
  information may simply not be available. This is the most likely place the
  contract gets amended rather than the adapter excused.
- Whether the structural consequences (frontier `head`, no read-your-own-writes,
  cluster-wide staleness bound) are acceptable to state as adapter documentation or
  require a specification clause of their own.
- Carried in: `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
  — read for context, **not** reopened here.

## Proof artefact

**`event_store_conformance!` green against `happenstance-neon` — a store with no
connection, no interactive transaction and no cursor — with the Postgres
position-visibility cost measured on live infrastructure.** This would not exist if
the design were wrong: every adapter written so far could hold a transaction open
and assign positions under a lock, so a port that had quietly assumed both would
pass all four and fail here. If Neon cannot pass, the contract changed by decision
record and the suite re-ran — which is itself the artefact, and a legitimate one.

## Clauses

- **ES-10** — the position-visibility clause the Postgres measurement was run for;
  bought here by whatever ADR-0024 decides. Its removal from the ledger is
  `publication-and-positioning`'s to reconcile.
- **ES-11, ES-12** — far-end discharge.
- **ES-41, ES-42** `[PROVISIONAL]` — recorded by `RUNBOOK.md:622-635` as missing
  from its own table; exercised here, reconciled at publish.
- **VT-21 – VT-24** `[PROVISIONAL]` — store limits, tested here and at
  `sqlite-durable-store`.
- Settled by **ADR-0024**.
- Nothing `[FROZEN]` is amended *by edit*. DoD 6 explicitly contemplates the
  contract changing — by decision record, with the suite re-run.

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
