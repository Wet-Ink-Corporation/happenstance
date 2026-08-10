---
item: HS-I0004
stage: intake
created: 2026-08-10T02:55:02.895Z
updated: 2026-08-10T02:55:02.895Z
template_sig: 9b551827
rendered_sig: 5883729f
---

# Intake Brief — Adapters and replication: the post-0.2.0 spread

## Problem

At 0.2.0 the contract has been proved against one storage shape. `MemoryEventStore`,
the `RefCell` store, rusqlite and a Durable Object all serialise their writers and
assign positions under a lock — four implementations, one shape, and a port frozen
against them is frozen against SQLite wearing four hats. Two of the workspace's
skeletons exist precisely to sit at the other end of that axis and neither has been
built. Replication is worse than unbuilt: `SequencePosition` is meaningful only
within one store, so positions cannot be replicated as-is, and whether ingest
re-checks append conditions is still the central unanswered question.

## Desired Outcome

The contract measured against implementations that disagree with each other — a
store that does not serialise its writers, a store with no connection and no cursor,
a `!Send` store on `wasm32`, and a non-SQL projection batch — and then a replication
port proved the only way a port can be: two unlike implementations and an oracle.

## Constraints

- **Phases 9, 10 and 11 touch disjoint crates and parallelise freely.** Phase 13
  does not: it needs three real stores at once.
- **Phase 13 depends on 8, 9, 10 and 12** — a cross-initiative dependency, recorded
  as `blocked_by` rather than as prose.
- **Positions do not cross a store boundary.** Anything that replicates one is
  wrong by construction, whatever it measures.
- **Non-goal.** No new port is invented here. `SyncPeer`'s shape is §5's, and §5
  is indicative rather than frozen.

## Open Questions

- Does ingest re-check append conditions? §5.1 records a bound decision that it
  never rejects; the falsifier is still owed.
  (`.kb/open-questions/replication-semantics.md`)
- How does a Postgres adapter buy position visibility, in a binding form? The
  measurement exists and names `xid8` + `pg_snapshot_xmin` at no measurable
  throughput cost; ADR-0024 is what turns a measurement into a decision.
  (`.kb/open-questions/postgres-position-visibility.md`)
- Are hub-and-spoke and peer-to-peer one abstraction or two?
- What does retention across a peer set do to a store that holds only a suffix of
  its own log?

## Proof artefact

- **Phase 9** — every rule green under `workerd`, and a real `worker::Error`-carrying
  error type that either loses information the caller needs or demonstrably does not.
- **Phase 10** — the concurrency macro green on a store that does **not** serialise
  its writers, with the visibility cost measured rather than preferred.
- **Phase 11** — the projection suite green on a non-SQL batch, and a written verdict
  on whether phase 6's freeze held. The verdict is the artefact: a phase that only
  reports success cannot report that the freeze was wrong.
- **Phase 13** — one suite green against three peers, two of them unlike, and a
  byte-identical payload round trip.
- **Phase 14** — a store that holds only a suffix of its own log, and a runner that
  fails loudly against it.

## Clauses

- **PS-\*** — frozen by the previous initiative; phase 11 is where that freeze is
  tested by something structurally unlike the two it was frozen against.
- **SY-1 – SY-30** — indicative, settled across phases 13 and 14 by ADR-0026 and
  ADR-0027.
- **ES-\*** completeness clauses (§3.7) — what a store that has been deleted from
  may look like; phase 14.
- **ADR-0003** loses its `provisional` marker at phase 13, when a payload
  round-trips between two stores.

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

The fifth box is the whole point of this initiative rather than a formality: the
axis is *does the store serialise its writers and own its transaction*, and the
things at the other end of it are `happenstance-postgres` (positions assigned
outside the transaction) and `happenstance-neon` (no connection, no interactive
transaction, no cursor).
