---
item: HS-P0003
stage: intake
created: 2026-08-10T02:59:44.867Z
updated: 2026-08-10T02:59:44.867Z
template_sig: 56ad54cb
rendered_sig: 963218f7
---

# Intake Brief — Phase 8: `happenstance-sqlite`

## Problem

Every implementation of `EventStore` in this workspace is either in memory or a
skeleton whose bodies are `todo!()` — and a `todo!()` body type-checks against any
signature, because `!` coerces to everything. So the contract frozen at phase 4 has
never met a database. Durability and concurrency are the two properties an in-memory
store gets for free and a real one does not, and neither has been tested.

## Desired Outcome

The flagship adapter: `happenstance-sqlite` on rusqlite, having invoked
`happenstance_testkit::event_store_conformance!` and passed it. Until then it is not
an adapter, whatever it compiles into.

## Constraints

- **Phase 7 first** — the typed layer is the consumer that discovers contract
  defects.
- **`EventId` and `recorded_at` are columns in migration 1.** Settled at phase 4 for
  exactly this reason.
- **No literal position assertions anywhere in the rules this touches.** The
  specification permits gaps and a conformant adapter may leave them.
- **Non-goal.** No projection store here beyond what the crate already skeletons; no
  publication.

## Open Questions

- The schema, tag storage, and the **append-condition SQL strategy** — how a
  condition becomes SQL, what it costs under contention, and how it behaves when the
  store leaves gaps. (ADR-0022;
  `.kb/open-questions/append-condition-sql-strategy.md`)

## Proof artefact

**The concurrency macro green at 64 contenders, and an acknowledged write surviving a
process reopen.**

Both are properties memory gives away free. A wrong locking story fails the first; a
wrong durability story fails the second; and neither can be satisfied by a store that
merely compiles.

## Clauses

Exercises **ES-\*** and **VT-\*** as frozen at phase 4 — this is the first time that
freeze meets a real database rather than a skeleton. A clause that has to move here
needs a new ADR, and finding one is a success of this phase rather than a failure.

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
