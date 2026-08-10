---
item: HS-P0006
stage: intake
created: 2026-08-10T02:59:46.058Z
updated: 2026-08-10T02:59:46.058Z
template_sig: 56ad54cb
rendered_sig: 2a844ab9
---

# Intake Brief — Phase 10: `happenstance-postgres` and `happenstance-neon`

## Problem

Every store that has passed the suite serialises its writers and assigns positions
under a lock. A port frozen against them is frozen against one storage shape wearing
several hats. These two crates exist to sit at the other end of that axis and neither
has been built: Postgres assigns positions **outside** the transaction, and Neon has
no connection, no interactive transaction and no cursor.

## Desired Outcome

Both adapters through the conformance suite, with the cost of position visibility
measured rather than preferred — and a contract that survives contact with a store
whose writers genuinely run in parallel.

## Constraints

- **`nextval()` allocates outside the transaction**, so a Postgres store breaks ES-10
  by construction unless it does something about it. A writer takes 99, a writer that
  started later takes 100 and commits first, and a reader that has already observed
  100 later sees 99 appear beneath it.
- **The choice is owed a measurement.** It now has one:
  `.kb/reference/experiment-position-visibility.md` finds `xid8` +
  `pg_snapshot_xmin` buys ES-10 as written at no measurable throughput cost, two
  mechanisms buy it by serialising every writer at 16× and 30×, and advisory locks
  keyed by tags are cheap *because they do not buy ES-10 at all*.
- **Neon must build for host and wasm32 both.** The pair of checks proves the bare
  flavour on each target, which is not the same as satisfying both flavours.
- **Non-goal.** Not on the 0.2.0 path.

## Open Questions

- Which mechanism the adapter takes, in a **binding** form. The measurement answers
  the empirical question and does not by itself constitute the decision — ADR-0024 is
  the instrument for that.
  (`.kb/open-questions/postgres-position-visibility.md`)

## Proof artefact

**The concurrency macro green on a store that does not serialise its writers, with
the visibility cost measured.**

A store that serialises its writers passes the concurrency family for a reason that
has nothing to do with the contract being right. This is the first run where it does
not, and the measurement is what stops "we chose `xid8`" from being a preference
wearing a number.

## Clauses

Discharges **ES-10** against an implementation that can actually violate it, and
tests the ES freeze against the axis it is most likely to be wrong about.

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
