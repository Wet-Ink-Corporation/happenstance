---
item: HS-P0012
stage: intake
created: 2026-08-12T03:23:09.868Z
updated: 2026-08-12T03:23:09.868Z
template_sig: ab516678
rendered_sig: 9ade314b
---

# Intake Brief — The first adapter that is not an instrument

## Problem

`happenstance-sqlite` is a skeleton: real associated types, `todo!()` bodies,
`publish = false`, and a scoped `#![allow(clippy::todo)]` naming the phase that
removes it. A `todo!()` body type-checks against any signature, so skeleton-based
proof is decorative by construction (`RUNBOOK.md:91-100`). Nothing in this
workspace has ever *passed* the conformance suite against durable storage, and no
fixture has ever handed out two genuinely independent handles onto one backing
store — every existing one hands out refcount clones of a single in-process
object.

## Desired Outcome

`happenstance-sqlite` is a store that has passed the suite, not one that compiles
against it. We know it worked when `event_store_conformance!` is green against a
real `SqliteFixture`, including the concurrency macro at 64 contenders and an
acknowledged write surviving a **process reopen** with two real handles onto one
file (DoD 3). Every `todo!()` is gone and the scoped `#![allow(clippy::todo)]` is
deleted — the allow's own presence is the marker that the crate is still an
instrument.

## Constraints

- **Depends on** `projection-store-freeze` (the projection suite this crate's
  projection store runs against) and `typed-layer-and-alpha-release` (the phase 7 →
  phase 8 edge: the typed layer is what discovers contract defects, and doing this
  first is precisely the sequence the plan exists to avoid).
- **Driver is settled**: `rusqlite`, decided at phase 2 by building both. The
  **append-condition SQL strategy** is the part still genuinely open — ADR-0022's.
- **Migration 1 columns are fixed**: `EventId` and `recorded_at`, already discharged
  at phase 4 and identical in every store.
- **An adapter that compiles but has not run the conformance suite is not an
  adapter.** No exceptions, including for this project.
- **Benchmarks are never a conformance rule.** `event_store_benchmarks!` (CF-34) is
  inherited by adapters and lives outside the suite.
- **Non-goals**, each naming its owner: the non-serialising and no-cursor far ends →
  `postgres-and-neon-stores`; the `wasm32` / `!Send` far end →
  `cloudflare-durable-object-store`; whether adapter crates are published at all →
  `publication-and-positioning`.

## Open Questions

- **The append-condition SQL strategy** (ADR-0022) — the one genuinely open part of
  the driver decision. Notes are in `crates/happenstance-sqlite/src/`.
- Schema and tag storage shape, and how the append condition reads against it.
- **CF-14's far end**: a store that can lose a write to a *fault*, as distinct from
  losing one to a rejected condition. Nothing in the workspace sits there today.
- **CF-17's rule shape** and **ES-35** — both discharged here or explicitly deferred
  with a reason.
- Whether the second unlike batch shape DoD 7 owes lives here or inside the testkit
  — see the watched edge in `projection-store-freeze`'s brief. If it lives here,
  this project acquires an edge back to `projection-store-freeze`'s exit that the
  DAG does not currently carry.

## Proof artefact

**An acknowledged write surviving a process reopen, observed through two real
handles onto one SQLite file**, with `event_store_conformance!` green including the
concurrency macro at 64 contenders. This would not exist if the design were wrong:
every fixture in the workspace to date hands out refcount clones of one in-process
object, so the `REOPEN` and `SECOND_HANDLE` capabilities have never been exercised
by anything that could fail them. A store that only looked durable would pass every
existing fixture and fail this one.

## Clauses

- **ES-35** `[PROVISIONAL]` — discharged here or deferred with a stated reason.
- **CF-14** `[DEFERRED]` — its far end (loss to a fault) is exercised here; whether
  the deferral survives contact is recorded.
- **CF-17** — the rule shape settled here.
- **CF-34** — `event_store_benchmarks!`, inherited by adapters, never a conformance
  rule.
- **VT-21 – VT-24** `[PROVISIONAL]` — store limits, tested here and again at
  `postgres-and-neon-stores`.
- **PS-*** — the SQLite projection store runs against the suite frozen by
  `projection-store-freeze`; this project consumes those clauses, it does not
  amend them.
- Settled by **ADR-0022** (driver, schema, migration 1, tag storage,
  append-condition strategy).
- Nothing `[FROZEN]` is amended.

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
