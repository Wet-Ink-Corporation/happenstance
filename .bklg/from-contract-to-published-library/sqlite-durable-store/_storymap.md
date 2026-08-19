---
item: HS-P0012
stage: storymap
created: 2026-08-12T03:30:13.774Z
updated: 2026-08-12T03:30:13.774Z
template_sig: 1c63534a
rendered_sig: bdcd9f6b
---

# Story Map — The first adapter that is not an instrument

Fourteen stories, five slices. The spine is `project.md`'s **AC-001 – AC-016**; the
shape comes from the two warranted briefs in
[`_decomposition.md`](_decomposition.md) — the architecture brief's §1 mount table
(*which target actually executes this code, against which sibling's contract*) and
the testing brief's §2 AC-to-tier matrix.

The user here is an **adapter author and the library's consumer**, and the only
observable increment this project can offer either of them is the same one:
*a store that has passed the bar, against durable storage, through two real
handles*. So every capability story below ends at a `cargo test` target under
`crates/happenstance-sqlite/tests/`, never at a module that only `cargo check`
sees (architecture brief AC-A01).

## Backbone

The activities across the top, in the order a reader of this crate meets them.

| # | Activity | The outcome it produces | Stories |
| --- | --- | --- | --- |
| 1 | **Decide before building** | An append-condition strategy chosen on a measured number, not a preference | `benchmark-harness`, `adr-0022-append-condition-strategy` |
| 2 | **Write an event durably** | A batch acknowledged inside one transaction, on a schema that does not serialise the probe | `schema-migration-and-identity`, `append-atomicity-and-store-limits` |
| 3 | **Replay a log** | A lazy, paged, snapshot-bounded read that serves a wide query rather than refusing it | `lazy-read-with-snapshot-ceiling`, `wide-query-chunked-not-refused` |
| 4 | **Clear the bar** | `event_store_conformance!` green against a real file, every rule `Ran` or `Skipped` with a reason | `sqlite-fixture-and-whole-suite` |
| 5 | **Survive a race, and a reopen** | The far ends `RUNBOOK.md:691-692` says are empty: handle multiplicity and durability | `concurrency-family-and-contender-count`, `model-family-and-mutant-pass-column`, `reopen-negative-control-and-durability-verdicts` |
| 6 | **Project into a read model** | The SQL projection store passing a suite it did not write | `projection-store-passes-the-borrowed-suite` |
| 7 | **Hand it over** | No instrument markers, a green `--fast` gate, a manifest a publisher could act on, and a specification that still agrees with the code | `instrument-markers-removed-and-gate-green`, `crates-io-name-and-packaging-facts`, `spec-and-code-reconciliation` |

## Slices

Five slices. The first is pure substrate and is sequenced ahead of everything that
consumes it; `durable-event-store` is deliberately the largest, because splitting
"implement `append`" from "mount the fixture that runs it" would recreate exactly
the compiles-but-never-ran failure this project exists to retire
(`CLAUDE.md`, *The rule that matters*).

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `bench-harness-and-adr` | `benchmark-harness` | foundation | Add `event_store_benchmarks!` to `happenstance-testkit` behind an off-by-default **and** target-gated `bench` feature, with its own enumeration outside `for_each_event_store_rule!` and a caller-supplied emitter, mounted against `MemoryFixture`. | — | AC-012 |
| `bench-harness-and-adr` | `adr-0022-append-condition-strategy` | foundation | Measure the three append-condition candidates and the tag-storage options against real SQLite through that harness, and commit ADR-0022 — atom plus long record — carrying the winner, the losers, the amended schema, the runtime seam, the `index_arms()` rejection and a number. | `benchmark-harness` | AC-013 |
| `durable-event-store` | `schema-migration-and-identity` | foundation | Land migration 1 as ADR-0022 amends it — `event_tag` keyed `(tag, position)` with `event_type` as a covering column, `tag_cardinality`, the `EventId` origin pair `UNIQUE`, `recorded_at`, and a `StoreId` minted once and persisted — idempotent under two concurrent opens, with WAL, a stated `synchronous` and a finite busy timeout. | `adr-0022-append-condition-strategy` | AC-001, AC-004 |
| `durable-event-store` | `append-atomicity-and-store-limits` | capability | Replace `append`'s `todo!()` with preconditions-then-one-`BEGIN IMMEDIATE`: empty batch refused first, the three declared ceilings raised as `ExceedsStoreLimit`, every guard probed by `EXISTS` before a row of the batch is inserted, returning the caller's own last position. | `schema-migration-and-identity` | AC-007, AC-009 |
| `durable-event-store` | `lazy-read-with-snapshot-ceiling` | capability | Replace the read path's `todo!()`s with a still-lazy paged `SqliteReadStream` that captures ADR-0011's position ceiling no later than the first `poll_next`, composes it with `ReadOptions::to` and `backwards`, and keeps `resume_from` inclusive. | `schema-migration-and-identity` | AC-001 |
| `durable-event-store` | `wide-query-chunked-not-refused` | capability | Decompose a wide `Query` into `ceil(arms/N)` prepared statements **privately inside `happenstance-sqlite`** and k-way-merge their cursors, de-duplicating on position and applying the page budget and `ReadOptions::limit` to the merged output. | `lazy-read-with-snapshot-ceiling` | AC-008 |
| `durable-event-store` | `sqlite-fixture-and-whole-suite` | capability | Mount `SqliteFixture` in `crates/happenstance-sqlite/tests/conformance.rs` — one instance is one fresh temp file, each `connect()` a second `rusqlite::Connection` onto it — declaring `SECOND_HANDLE`, `REOPEN` and real ceilings, declining `MID_BATCH_FAULT` with its reason, and take `event_store_conformance!` green whole. | `append-atomicity-and-store-limits`, `wide-query-chunked-not-refused` | AC-001, AC-002, AC-003, AC-004, AC-009 |
| `race-model-and-durability` | `concurrency-family-and-contender-count` | capability | Take `event_store_concurrency_conformance!` green with ADR-0022's runtime seam in place (a captured `Handle`, not `NoRuntime` from every contender), and close the 8-versus-64 discrepancy by raising `CONTENDERS` with a stated reason **or** amending both proof artefacts. | `sqlite-fixture-and-whole-suite` | AC-005, AC-007 |
| `race-model-and-durability` | `model-family-and-mutant-pass-column` | capability | Take `event_store_model_conformance!` green and put `SqliteEventStore` in the mutant harness's pass column, adding the `BEGIN DEFERRED` probe-then-insert row to `mutation_coverage/mutants.rs`'s `REGISTRY` so the racing rules are shown to be falsifiable. | `concurrency-family-and-contender-count` | AC-006, AC-007 |
| `race-model-and-durability` | `reopen-negative-control-and-durability-verdicts` | capability | Give `recorded_time_survives_a_reopen` the negative control it has lacked since phase 4 — a permanent registry row encoding a re-stamped `recorded_at` — and land the recorded verdicts for CF-17's rule shape, ES-35's marker and CF-14's deferral. | `sqlite-fixture-and-whole-suite` | AC-004, AC-010 |
| `sqlite-projection-store` | `projection-store-passes-the-borrowed-suite` | capability | Give `SqliteProjectionStore` real bodies against its owned `Batch` on an independently-migrated connection, and mount it at `crates/happenstance-sqlite/tests/projection.rs` against the projection suite `projection-store-freeze` froze. | `schema-migration-and-identity` | AC-011 |
| `publishable-and-reconciled` | `instrument-markers-removed-and-gate-green` | capability | Delete the last `todo!()` on a SQLite path and the scoped `#![allow(clippy::todo)]` in the same change, correct the crate's known-wrong module-doc sketch, and take `cargo xtask ci --fast` green. | `model-family-and-mutant-pass-column`, `reopen-negative-control-and-durability-verdicts`, `projection-store-passes-the-borrowed-suite` | AC-014 |
| `publishable-and-reconciled` | `crates-io-name-and-packaging-facts` | capability | Reserve `happenstance-sqlite` on crates.io and give the crate a real description, README and both licence files so `cargo package --list --allow-dirty` shows them — while `publish = false` and `PUBLISHABLE` stay untouched. | `instrument-markers-removed-and-gate-green` | AC-015 |
| `publishable-and-reconciled` | `spec-and-code-reconciliation` | capability | Discharge the standing reconciliation criterion: read every clause ADR-0022 discharges against the code as it now stands, compare the phase's clause range against the union of its ADRs' ranges, and show `spec-trace`'s citation count has not fallen. | `instrument-markers-removed-and-gate-green`, `reopen-negative-control-and-durability-verdicts` | AC-016 |

### Why the three foundations are foundations, and what consumes each

Per RFC §6.1/D8 a foundation is real in-tree substrate consumed and demonstrated by
a capability slice in **this** initiative. None of the three is a double, a flag or
a fixme, and each names its consumer:

- **`benchmark-harness`** ships `crates/happenstance-testkit/src/bench.rs` and is
  consumed immediately by `adr-0022-append-condition-strategy`, which cannot quote a
  measured figure without it, and again by the SQLite fixture once it exists. It is
  never a conformance rule (CF-34), which is what makes AC-012's "the rule count is
  unchanged by its arrival" structurally true rather than asserted.
- **`adr-0022-append-condition-strategy`** is the decision substrate every
  implementation story below reads: the append strategy, the tag storage, the
  amended schema, the runtime seam and the busy timeout. AC-013 and DR-06 require it
  **before** the implementation, so it cannot be a capability story sequenced by
  convenience — it is a hard predecessor. Authored through the runbook's ADR queue as
  an atom under `.kb/decisions/` plus the long record under `references/adr/`, per
  `CLAUDE.md`'s two-places rule, never as a side effect of a code change.
- **`schema-migration-and-identity`** is migration 1: not user-observable on its own,
  and consumed by `append-atomicity-and-store-limits`,
  `lazy-read-with-snapshot-ceiling` and
  `projection-store-passes-the-borrowed-suite` in the same project. Its persisted
  `StoreId` and read-back `recorded_at` are the architectural precondition that makes
  the reopen rules askable at all (architecture brief §7).

Nothing in this map depends on substrate owned outside the initiative. The two
external edges are **project-level**, already recorded in `project.md` *Dependencies*:
`projection-store-freeze` (HS-P0010) ships the projection suite
`projection-store-passes-the-borrowed-suite` invokes, and
`typed-layer-and-alpha-release` (HS-P0011) precedes the whole project. Neither is a
story dependency here.

### Slice coherence notes

- **`durable-event-store` is one slice on purpose.** `SqliteFixture` is the
  composition root, and a fixture without `append` and `read` is a mount with nothing
  mounted. `SqliteEventStore::open_in_memory` is the wrong constructor for it — a
  private in-memory database is per-*connection*, so a second `connect()` would open a
  second empty database and `two_handles_observe_each_others_appends`, a MUST, would
  fail (architecture brief §1).
- **`race-model-and-durability` is a separate slice, not an afterthought.** Its three
  stories all run *on top of* a green sequential suite, and the testing brief §4 is
  explicit that a green sequential suite with a skipped or hanging concurrency family
  is not partial credit. The runtime seam ADR-0022 settles is only exercised here.
- **`sqlite-projection-store` is independent of `race-model-and-durability`** — it
  touches `projection_store.rs` and its own connection only, and may be scheduled
  either side of it. It consumes the frozen suite and amends nothing about the port;
  PS-6 and PS-7 stay "not this crate's to settle".
- **`publishable-and-reconciled` is last because AC-014 is a whole-crate claim.** The
  scoped `#![allow(clippy::todo)]` may only be deleted in the same change as the last
  `todo!()` (DR-01), which means after the projection store, not before.

## Coverage

Every project acceptance criterion maps to at least one story, and no story shares a
responsibility with another. Where an AC appears twice the two stories discharge
*different halves* of it, named in the right-hand column.

| Project AC | Stories | Split, where there is one |
| --- | --- | --- |
| AC-001 — the suite runs, whole | `schema-migration-and-identity`, `lazy-read-with-snapshot-ceiling`, `sqlite-fixture-and-whole-suite` | schema makes any rule runnable; read carries the largest rule family; the fixture story owns the whole-run claim (`Ran` or `Skipped{reason}`, never absent) |
| AC-002 — two instances share nothing | `sqlite-fixture-and-whole-suite` | — |
| AC-003 — the second handle is a second connection | `sqlite-fixture-and-whole-suite` | — |
| AC-004 — an acknowledged write survives a reopen | `schema-migration-and-identity`, `sqlite-fixture-and-whole-suite`, `reopen-negative-control-and-durability-verdicts` | schema persists the `StoreId` and reads `recorded_at` back; the fixture declares `REOPEN` and the rules pass; the third proves the rule **can** fail |
| AC-005 — the race is real, its size is a decision | `concurrency-family-and-contender-count` | — |
| AC-006 — model family and mutant harness | `model-family-and-mutant-pass-column` | — |
| AC-007 — atomic append, probe-then-insert rejected | `append-atomicity-and-store-limits`, `concurrency-family-and-contender-count`, `model-family-and-mutant-pass-column` | the implementation; the family that rejects the wrong one; the registry row that proves the rejection is live |
| AC-008 — a wide query is chunked, not refused | `wide-query-chunked-not-refused` | — |
| AC-009 — limits are declared facts | `append-atomicity-and-store-limits`, `sqlite-fixture-and-whole-suite` | `append` enforces the ceilings; the fixture states them, VT-21 – VT-24 run, and CF-40's clause home is recorded as still open |
| AC-010 — durability clauses leave with verdicts | `reopen-negative-control-and-durability-verdicts` | — |
| AC-011 — the projection store passes the borrowed suite | `projection-store-passes-the-borrowed-suite` | — |
| AC-012 — benchmarks exist and are provably not conformance | `benchmark-harness` | — |
| AC-013 — the decision record carries a number | `adr-0022-append-condition-strategy` | — |
| AC-014 — instrument markers gone, gate green | `instrument-markers-removed-and-gate-green` | — |
| AC-015 — publishable, publishing stays elsewhere | `crates-io-name-and-packaging-facts` | — |
| AC-016 — specification and code still agree | `spec-and-code-reconciliation` | — |

**No AC is orphaned**: AC-001 – AC-016 all appear above, each against at least one
story slug from the Slices table. **No responsibility is duplicated**: the five ACs
carried by more than one story are split by clause, not shared wholesale.

Three obligations the briefs raise that are *not* separate stories, and where they
land instead, so they are not mistaken for gaps:

- **AC-T05 / DoD 4 — cited evidence in each story's `_ledger.md`** is a per-story
  obligation of every row above (`require_ledger: true`), not a story of its own.
- **DR-08 — nothing added to the testkit costs the `!Send` flavour anything** is
  discharged inside `benchmark-harness` (feature-gated *and* target-gated) and
  re-checked by `instrument-markers-removed-and-gate-green`'s `--fast` gate.
- **The DoD 7 second-unlike-batch-shape question** is answered in the architecture
  brief §9 *before* code — it does **not** live in this project — so it produces no
  story here. If `projection-store-freeze` assigns it here instead, that is a
  blocking re-plan, not something `projection-store-passes-the-borrowed-suite`
  absorbs.

## Merge order

Slice by slice; foundations before the capability slices that consume them.

1. **`bench-harness-and-adr`** — `benchmark-harness` → `adr-0022-append-condition-strategy`.
   Nothing else may start: AC-013 puts the record before the implementation, and the
   record needs a number the harness produces.
2. **`durable-event-store`** — `schema-migration-and-identity` →
   (`append-atomicity-and-store-limits` ‖ `lazy-read-with-snapshot-ceiling`) →
   `wide-query-chunked-not-refused` → `sqlite-fixture-and-whole-suite`. The two
   middle stories are independent of each other; the fixture story closes the slice
   and is the first green `event_store_conformance!` in the workspace against a file.
3. **`race-model-and-durability`** — `concurrency-family-and-contender-count` →
   `model-family-and-mutant-pass-column`; `reopen-negative-control-and-durability-verdicts`
   in parallel with either. The concurrency story leads because it is where the
   runtime seam and the busy timeout are first exercised, and a hang here is a finding
   about ADR-0022's timeout paragraph rather than something a test may paper over
   (CF-33: no watchdog, anywhere).
4. **`sqlite-projection-store`** — `projection-store-passes-the-borrowed-suite`.
   Depends only on slice 2 and may be pulled forward beside slice 3 if
   `projection-store-freeze` has already shipped its suite.
5. **`publishable-and-reconciled`** — `instrument-markers-removed-and-gate-green` →
   (`crates-io-name-and-packaging-facts` ‖ `spec-and-code-reconciliation`). The
   crates.io *name reservation* inside the first of those two has no code dependency
   and should be done at the project's start per `RUNBOOK.md:4191-4193`; the manifest
   facts it is paired with cannot land until the README can state the adapter's real
   durability settings and enforced ceiling.

Cross-slice edges: 1 → 2 → {3, 4} → 5. Acyclic, and every `depends_on` in the table
above points backwards in this order.
