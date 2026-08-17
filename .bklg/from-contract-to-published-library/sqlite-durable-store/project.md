---
id: HS-P0012
uid: f1c63c
type: project
slug: sqlite-durable-store
title: The first adapter that is not an instrument
parent: HS-I0006
initiative: from-contract-to-published-library
project: sqlite-durable-store
status: implementing
process: project
stage: implementation
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-17T00:53:30.326Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The first adapter that is not an instrument

## One-line objective

Turn `happenstance-sqlite` from a skeleton whose every SQL body is `todo!()` into
the workspace's first adapter that has **passed** the conformance suite against
durable storage — two real connections onto one file, and an acknowledged write
surviving a genuine process reopen.

## How this advances the initiative

The initiative's whole premise is that six things look like evidence and are not,
and this project retires two of them.

**A `todo!()` body type-checks against any signature**, so `happenstance-sqlite`
has told the type checker nothing about whether it can implement what it declares
(`RUNBOOK.md:91-100`; `crates/happenstance-sqlite/src/lib.rs:1-24`). BR-03 says
nothing counts toward an outcome until it has passed the suite, and this project
is the first place in the initiative where that becomes true of a real database.

**Two of the instrument portfolio's seven axes are empty at the far end because
of this crate**, and `RUNBOOK.md:691-692` names both: *handle multiplicity* —
"every fixture still hands out refcount clones of one in-process object, so no
*connection* has been opened twice" — and *durability* — "Nothing yet loses a
write to a *fault* rather than to an instruction". A far end is a **passing
implementation**, not a skeleton and not a fixture (CF-26, `RUNBOOK.md:695-699`).
Filling these two is this project's contribution to BR-02: it supplies the
*durable* member of the three implementations that must genuinely disagree, with
the non-serialising and no-cursor members owned by `postgres-and-neon-stores` and
the `!Send` member by `cloudflare-durable-object-store`.

It is also the first adapter that can *exercise* the suite's own claims rather
than assert them: AC-04 (told when you are finished), AC-05 (a declined
capability reports its reason) and AC-06 (your storage shape is not quietly
assumed) are owned elsewhere and are put under load here for the first time by an
implementation that could fail them.

Per the decomposition, this project owns BR-02¹, BR-03 and DoD 3, contributes to
BR-12, BR-13, AC-04, AC-05, AC-06 and DoD 7, and writes its own decision record
under BR-15 (`.bklg/from-contract-to-published-library/_decomposition.md`).

## In scope (this project)

- **ADR-0022, written before the code** — driver (settled: `rusqlite`, and
  deliberately without a pool), schema, tag storage, migration 1 including the
  `EventId` and `recorded_at` columns settled at phase 4, and the **append-condition
  SQL strategy**, which is the genuinely open part
  (`RUNBOOK.md:301`, `RUNBOOK.md:4195-4197`;
  `crates/happenstance-sqlite/src/lib.rs:47-65`). The record carries a benchmark
  *number*, not a claim (`RUNBOOK.md:4227-4228`).
- **The schema as amended by the evaluation** — `event_type` carried as a covering
  column on `event_tag` with the key left `(tag, position)`, plus a
  `tag_cardinality` table, because the sketch at
  `crates/happenstance-sqlite/src/event_store.rs:36-54` forces a join back to
  `event` under the write lock and serialises every writer
  (`RUNBOOK.md:4178-4187`). `AUTOINCREMENT` is load-bearing: positions are never
  reused after a delete.
- **The real `read` stream**, with `spawn_blocking` deferred into `poll_next`
  because `EventStore::read` is not `async` and must not spawn on the caller's
  thread (`crates/happenstance-sqlite/src/event_store.rs:11-32`; ADR-0001,
  ADR-0008 per `CLAUDE.md` constraint 3).
- **The real `append`** — condition evaluation and write in one transaction,
  returning the caller's own last position (`RUNBOOK.md:4199-4201`).
- **`SqliteProjectionStore`** against the owned `Batch` frozen by
  `projection-store-freeze`, running that project's projection suite
  (`crates/happenstance-sqlite/src/projection_store.rs`).
- **A `SqliteFixture`** that is the first in the workspace to mean what
  `Fixture` says: one instance is one file, each `connect` is a second real
  `rusqlite::Connection` onto it, and two instances share nothing
  (`crates/happenstance-testkit/src/contract.rs:12-23, 69-75`). It declares
  `SECOND_HANDLE` and `REOPEN` as available, and declares its real numeric
  ceilings.
- **Query arm chunking** past SQLite's pushdown limit — `ceil(arms/400)` prepared
  statements merged in Rust rather than a refusal, since `Query` bounds nothing by
  design and a hard cap in the contract would be wrong (`RUNBOOK.md:4203-4206`).
- **`event_store_benchmarks!(fixture)` in the testkit behind a `bench` feature**,
  inherited by adapters the way conformance is — and never a conformance rule
  (CF-34, `RUNBOOK.md:4207-4213`).
- **A decision on the contender count.** Both of phase 8's stated proof artefacts
  read *64 contenders*; `concurrency::CONTENDERS` is **8** and the parallelism is
  `std::thread::scope` rather than a runtime. `RUNBOOK.md:2686-2696` gives exactly
  two options — raise the constant and say why, or amend both proof artefacts —
  and names leaving it as "the third option and it is the one that rots".
- **The clause work this adapter forces**: CF-17's rule shape, ES-35's
  `[PROVISIONAL]` durability marker, and a recorded verdict on whether CF-14's
  deferral survives contact (`_intake-brief.md` *Clauses*;
  `RUNBOOK.md:605`, `RUNBOOK.md:4175-4176`). VT-21 – VT-24 are tested here and
  again at `postgres-and-neon-stores`.
- **Removing the instrument markers**: every `todo!()` on a SQLite path, and the
  scoped `#![allow(clippy::todo)]` whose own presence is the marker that the crate
  is still an instrument (`crates/happenstance-sqlite/src/lib.rs:76-80`).
- **Reserving `happenstance-sqlite` on crates.io**, per phase 0's rule that a name
  is claimed when its phase starts (`RUNBOOK.md:4191-4193`), and leaving the
  manifest in a state a publisher could act on.
- **The standing reconciliation criterion** every phase from 6 onward carries
  (`RUNBOOK.md:3810-3820`): clauses read against the code as it now stands, clause
  ranges compared, `spec-trace`'s citation count not fallen.

## Out of scope (this project)

Each exclusion names the sibling that owns it.

- **The non-serialising far end and the no-connection/no-cursor far end** —
  `postgres-and-neon-stores` (HS-P0014). This adapter serialises its writers by
  construction and says so (`crates/happenstance-sqlite/src/event_store.rs:83-96`);
  that is the shape it represents, not a defect to fix here.
- **The `wasm32` / `!Send` far end, and running the suite under `workerd`** —
  `cloudflare-durable-object-store` (HS-P0013).
- **Freezing `ProjectionStore`, authoring the projection suite, the
  capability-declension *policy* (DT-3) and the adapter-author-bar cut (DT-8)** —
  `projection-store-freeze` (HS-P0010). This project *consumes* those clauses and
  that policy; it does not amend them.
- **The graph-shaped third batch shape and the written freeze verdict** —
  `ladybug-projection-store` (HS-P0015).
- **The typed layer, the worked example, the compile-fail case and
  `0.2.0-alpha.1`** — `typed-layer-and-alpha-release` (HS-P0011). Its edge into
  this project is not negotiable (`RUNBOOK.md:254-258`).
- **Whether adapter crates are published at all, the MSRV promise, the semver
  diff, the clause-ledger audit and registry presentation** —
  `publication-and-positioning` (HS-P0016). This project makes the crate
  *publishable*; it does not decide that it publishes.
- **The completeness axis — ES-39, ES-40, what a store may forget** —
  `retention-and-incomplete-logs` (HS-P0018).
- **Replication identity and ingest** — `replication-identity-and-ingest`
  (HS-P0017).
- **The whole gate on the assembled library, and persona promotion** —
  `closeout-and-durable-audience` (HS-P0019). This project's bar is
  `cargo xtask ci --fast` (`.redkiln/config.yaml:50-55`).
- **Amending anything `[FROZEN]`.** If this adapter's contact with a real database
  says a frozen clause is wrong, that is a new decision atom and a re-plan, per the
  initiative charter's *Out of scope*.
- **General performance tuning.** A benchmark harness and one number in ADR-0022
  are in scope because a decision depends on them; an optimisation pass is not.

## Derived requirements

Expanded from the initiative requirements this project owns or exercises.

- **DR-01 (BR-03).** Every SQL path in `happenstance-sqlite` has a real body, and
  the scoped `#![allow(clippy::todo)]` is deleted in the same change as the last
  `todo!()` rather than outliving it.
- **DR-02 (BR-02¹, DoD 3).** All four macros are green against a real
  `SqliteFixture`: conformance, model, concurrency, and the reopen rules
  (`RUNBOOK.md:4226`).
- **DR-03 (BR-02¹).** The fixture supplies handle multiplicity for real — a second
  `connect` opens a second connection onto the same file, not a refcount clone of
  one in-process object — which is what `RUNBOOK.md:691` says no fixture has ever
  done.
- **DR-04 (BR-02¹).** The fixture supplies durability for real — an acknowledged
  write survives a genuine reopen, which `RUNBOOK.md:692` says nothing in the
  workspace can currently express.
- **DR-05 (BR-13, exercised).** Every capability this fixture declines carries a
  stated reason and the rule still appears in the run; every limit it states is a
  fact rather than a trade, per the distinction at
  `crates/happenstance-testkit/src/contract.rs:44-54`.
- **DR-06 (BR-15).** ADR-0022 is written first, answers one question, and names
  the append-condition alternatives that lost — the three candidates at
  `crates/happenstance-sqlite/src/lib.rs:56-62` and the tag-storage options at
  `:63-65`.
- **DR-07 (BR-06, feeds).** ES-35 and CF-14 leave this project with a recorded
  verdict — frozen, restated with a live falsifier, or deferred with a stated
  reason — so that `publication-and-positioning`'s clause-ledger audit reads a
  decision rather than a silence.
- **DR-08 (BR-12, exercised).** Nothing this project adds to the shared testkit
  costs the `!Send` flavour anything: the benchmark harness is feature-gated and
  the concurrency family stays opt-in with its `F::Store: EventStore + Send`
  bound (`crates/happenstance-testkit/src/concurrency.rs:70-88`).
- **DR-09 (DoD 7, contributes).** The SQLite projection store passes the projection
  suite frozen by `projection-store-freeze`, and the question of whether the second
  unlike batch shape DoD 7 owes lives here or inside the testkit is answered in
  this project's architecture brief before any code is written.

## Acceptance criteria

Project-grain and testable. This is the spine the story map must cover.

- **AC-001 — the suite runs, whole.** `event_store_conformance!(SqliteFixture::new())`
  is green, and every rule in `for_each_event_store_rule!` appears in the run as
  `Ran` or as `Skipped` with the fixture's stated reason. No rule is absent from
  the binary (`crates/happenstance-testkit/src/contract.rs:31-37`).
- **AC-002 — two fixture instances share nothing.** The isolation rule passes
  against a fixture that opens a fresh temporary file per instance; pointing every
  instance at one directory is the adapter mistake the fixture contract exists to
  catch (`crates/happenstance-testkit/src/contract.rs:17-23`) and it fails here.
- **AC-003 — the second handle is a second connection.** `SECOND_HANDLE` is
  available, and `two_handles_observe_each_others_appends` passes through two
  distinct `rusqlite::Connection`s onto one file — observable in the fixture's own
  code, not merely asserted. A fixture that returns a `Clone` of one store still
  passes the rule and does not satisfy this criterion.
- **AC-004 — an acknowledged write survives a reopen.** `REOPEN` is available and
  `acknowledged_writes_survive_a_reopen` passes across a genuine close-and-reopen
  of the file. `recorded_time_survives_a_reopen`, which has had no negative control
  reaching its headline assertion since phase 4 (`RUNBOOK.md:3205-3207`), gets one.
- **AC-005 — the race is real and its size is a decision.** The concurrency family
  is green, and the contender count is recorded either as a raised
  `concurrency::CONTENDERS` with a stated reason or as an amendment to **both**
  stated proof artefacts (`RUNBOOK.md:159` and `RUNBOOK.md:4217-4222`). Leaving the
  discrepancy is a failure of this criterion, not a deferral
  (`RUNBOOK.md:2686-2696`).
- **AC-006 — the model family and the mutant harness both accept it.**
  `event_store_model_conformance!` is green, and the phase-3 mutant harness is
  re-run with `SqliteEventStore` in the pass column (`RUNBOOK.md:4221-4222`).
- **AC-007 — the append is atomic, and a probe-then-insert is rejected.** `append`
  evaluates its condition and writes in one transaction and returns the caller's
  own last position; the named wrong implementation is a read-then-write adapter,
  which passes the sequential rule forever and fails the concurrency family within
  a handful of iterations (`RUNBOOK.md:4217-4219`).
- **AC-008 — a wide query is chunked, not refused.** A `Query` whose
  `index_arms()` exceeds SQLite's pushdown limit is served by merged cursors over
  `ceil(arms/400)` statements, and VT-23's 128-item minimum passes. The named wrong
  implementation is one that returns an error at the limit.
- **AC-009 — the store's limits are declared facts, and the rule that needs them
  runs.** The fixture states its real numeric ceilings, VT-21 – VT-24 pass, and
  `append_reports_exceeded_store_limits` runs rather than skipping. Where CF-40's
  clause *home* ends up is recorded as still open, not settled in passing
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md`, which names phase 8 as
  what forces it).
- **AC-010 — the durability clauses leave with verdicts.** CF-17's rule shape is
  settled; ES-35 is frozen on this adapter's evidence or restated as provisional
  with a falsifier naming what is still missing (a store that loses a write to a
  *fault* rather than to an instruction); CF-14's deferral is confirmed or
  withdrawn with a reason.
- **AC-011 — the projection store passes the suite it did not write.**
  `SqliteProjectionStore` is green against `projection-store-freeze`'s projection
  suite, and this project's architecture brief records — before code — whether the
  second unlike batch shape DoD 7 owes lives here or in the testkit, since the
  latter is the only answer that does not invert the 6 → 8 order.
- **AC-012 — benchmarks exist and are provably not conformance.**
  `event_store_benchmarks!` runs against the SQLite fixture behind a `bench`
  feature, and the conformance rule count is unchanged by its arrival — CF-34's
  own claim, checked rather than asserted.
- **AC-013 — the decision record carries a number.** ADR-0022 is committed before
  the implementation, states the append-condition strategy with the alternatives
  that lost, and quotes a measured benchmark figure rather than a preference
  (`RUNBOOK.md:4227-4228`).
- **AC-014 — the instrument markers are gone and the gate is green.** No `todo!()`
  on any SQLite path, `#![allow(clippy::todo)]` deleted from
  `crates/happenstance-sqlite/src/lib.rs`, and `cargo xtask ci --fast` green
  (`.redkiln/config.yaml:50-55`).
- **AC-015 — the crate is publishable, and publishing stays someone else's
  decision.** The name is reserved on crates.io, and the manifest carries a
  description, README and both licence files such that `CLAUDE.md`'s
  `cargo package --list` assertion would hold if `publication-and-positioning`
  adds it to the publishable set. Whether it does is that project's.
- **AC-016 — the specification and the code still agree.** The standing criterion
  at `RUNBOOK.md:3810-3820` is discharged: every clause ADR-0022 discharges is read
  against the code as it now stands, the phase's clause range and the union of its
  ADRs' ranges are computed and compared, and `cargo xtask spec-trace`'s citation
  count has not fallen.

## Definition of done (boundary-level)

1. **DoD 3 of the initiative is observed, not argued.** `event_store_conformance!`
   green against a real `SqliteFixture`, the concurrency family green at the
   contender count AC-005 settles, and an acknowledged write surviving a process
   reopen through two real handles onto one file.
2. **All four macros green** — conformance, model, concurrency, reopen
   (`RUNBOOK.md:4226`) — plus the projection suite for `SqliteProjectionStore`.
3. **ADR-0022 exists as an accepted decision atom** under `.kb/decisions/`, written
   through the runbook's ADR queue rather than as a side effect, with the
   open-question atoms it touches resolved rather than deleted.
4. **Every AC-001 – AC-016 has cited evidence in its story's `_ledger.md`**;
   `require_ledger: true` is on (`.redkiln/config.yaml:62-67`), and a green suite is
   a precondition for looking at the criteria, never a substitute for them
   (`RUNBOOK.md:38-42`).
5. **`cargo xtask ci --fast` green** — the non-terminal project bar
   (`.redkiln/config.yaml:50-55`). The whole gate on the assembled library is
   `closeout-and-durable-audience`'s.
6. **`redkiln validate --kb && redkiln doctor` clean**, with the six expected
   `template-drift` advisories and no seventh.

## Dependencies

From the decomposition's DAG
(`.bklg/from-contract-to-published-library/_decomposition.md`, *Sequencing*).
Rank 2; merge position 3.

**Depends on**

- **HS-P0010 `projection-store-freeze`** — the projection port, its `Batch` shape
  and the projection suite `SqliteProjectionStore` runs against. The runbook's
  6 → 7 → 8 chain.
- **HS-P0011 `typed-layer-and-alpha-release`** — *"the typed layer is the consumer
  that discovers contract defects, and discovering them after the flagship adapter
  is written is the sequence this plan exists to avoid"* (`RUNBOOK.md:254-258`).
  The decomposition marks this edge **not negotiable**.

**Unlocks**

- **HS-P0016 `publication-and-positioning`** — `0.2.0` waits for all four adapter
  projects, a fork decided at the decomposition gate rather than inherited from the
  runbook (`_decomposition.md`, *Decisions taken at the gate* 1).

No sibling adapter blocks this one and this one blocks no sibling adapter:
`cloudflare-durable-object-store`, `postgres-and-neon-stores` and
`ladybug-projection-store` touch disjoint crates and parallelise freely
(`RUNBOOK.md:239-242`).

## Risks and coupling notes

- **The concurrency harness may not be able to drive this store's read path.**
  Contenders run on bare OS threads under the testkit's own `block_on`, outside any
  ambient reactor, and `concurrency.rs:61-68` names `sqlx` as the case that breaks
  — but this crate's read stream defers `spawn_blocking` into `poll_next` and turns
  a missing runtime into `SqliteEventStoreError::NoRuntime`
  (`crates/happenstance-sqlite/src/event_store.rs:26-32`). Any concurrency rule that
  *reads* may therefore hit `NoRuntime` rather than a conformance failure. Either
  the spawn becomes a harness-supplied parameter, as `concurrency.rs:61-68`
  anticipates for phase 10, or this project establishes that no concurrency rule
  reaches the read path. It must be answered in the architecture brief, not
  discovered in a red run.
- **There is no watchdog anywhere in the suite, and there must not be one**
  (CF-33). A store that deadlocks hangs the binary and only the CI job timeout
  notices — a cost `RUNBOOK.md:2669-2674` states *"rather than discovered at phase
  8"*, which is this project. `BEGIN IMMEDIATE` plus a lock held to commit is
  precisely where that risk lives.
- **The DoD 7 watched edge.** `projection-store-freeze` owes two unlike batch
  shapes passing while the only SQL projection store ships from here, two merge
  positions later. If the second shape is taken to live in this project, the DAG
  acquires an edge it does not carry and 6 → 8 inverts. Named in the decomposition
  as unsettled and assigned to `projection-store-freeze`'s architecture brief; this
  project must not resolve it unilaterally.
- **CF-40's ownership contradiction is forced here.** ADR-0015 both claims and
  disclaims the clause that lets a fixture declare numeric limits, and the
  open-question atom names phase 8 — this project — as what forces it
  (`.kb/open-questions/cf-40-fixture-limits-ownership.md:77-85`). This project needs
  the *capability*; it does not get to pick the clause's home. Record, escalate to
  the ADR queue, do not settle in passing.
- **The schema sketch in the crate's own docs is known to be wrong** in a way that
  serialises every writer, and the correction is in the runbook rather than in the
  code (`RUNBOOK.md:4178-4187` against
  `crates/happenstance-sqlite/src/event_store.rs:36-54`). Implementing the doc
  comment as written is the most likely way to ship a slow, conformant adapter.
- **`recorded_time_survives_a_reopen` currently has no real negative control.**
  Until this adapter exists, the rule's headline assertion is unfalsified
  (`RUNBOOK.md:3205-3207`). If it passes on the first try, that is a claim to check,
  not a result to accept — per `CLAUDE.md`'s "a rule that no adapter can fail is
  decorative".
- **Positions may contain gaps and no rule may assert on literal positions.** The
  specification permits them, two DCB-labelled stores already disagree publicly
  (initiative charter, BR-14), and `AUTOINCREMENT` produces them after a delete.
  Compare against positions the store actually assigned.
- **This is the largest single adapter on the trunk** — ten runbook-days
  (`RUNBOOK.md:4235`) sitting between the alpha and `0.2.0`, with
  `publication-and-positioning` blocked behind it. Schedule pressure here lands
  directly on the release date, and the temptation it creates is to declare the
  crate done while a capability is quietly declined.

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — the charter: BR-02, BR-03, BR-12, BR-13,
  BR-15, AC-04 – AC-06, DoD 3
- [`../_decomposition.md`](../_decomposition.md) — the DAG, the traceability matrix,
  the scope seams and the four gate decisions
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, proof artefact and
  clause list for this project

Knowledge base:

- `.kb/open-questions/cf-40-fixture-limits-ownership.md` — forced by this phase
- `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`
- `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`
- `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`
- `.kb/decisions/0008-one-derivation-for-both-ports.md` — why `read` returns the
  stream at the top level
- `.kb/decisions/0009-error-send-sync.md` — what the store's `Error` may carry
- `.kb/maps/decision-map.md`, `.kb/maps/open-questions-index.md`

Plan and specification:

- `RUNBOOK.md:4166-4237` — phase 8 in full: goal, the two schema amendments, work
  items, proof artefact, exit criteria
- `RUNBOOK.md:2686-2696` — the 64-versus-8 contender discrepancy and the two
  options for closing it
- `RUNBOOK.md:685-699` — the instrument portfolio: the durability and
  handle-multiplicity rows this project fills
- `RUNBOOK.md:605-606` — CF-17, CF-34 and ES-35 as `[PROVISIONAL]` rows owned by
  phase 8
- `RUNBOOK.md:254-258` — 7 before 8, and why
- `RUNBOOK.md:3810-3820` — the standing reconciliation criterion for every phase
  from 6 onward
- `spec/SPECIFICATION.md:215-221` — the clause census the audit reads

Code:

- `crates/happenstance-sqlite/src/lib.rs` — the status banner, the open decisions,
  and the `#![allow(clippy::todo)]` this project deletes
- `crates/happenstance-sqlite/src/event_store.rs` — the real types, the deferred
  `spawn_blocking`, and the schema sketch the runbook amends
- `crates/happenstance-sqlite/src/projection_store.rs` — the owned batch
- `crates/happenstance-testkit/src/contract.rs` — `Fixture`, `Capability`,
  `RuleOutcome`, and the limits-are-facts distinction
- `crates/happenstance-testkit/src/concurrency.rs` — the opt-in family, its bound,
  and the bare-thread limitation named for phase 10
- `crates/happenstance-testkit/src/fixtures.rs` — `MemoryFixture`, the reference
  implementation to read before writing `SqliteFixture`
- `crates/happenstance-sqlite/Cargo.toml` — `publish = false` and the two features
- `.redkiln/config.yaml:28-73` — the verify grains this project is measured by
- `CLAUDE.md` — the rule that matters, and the two corollaries about decorative
  rules and the spread of implementations

## Companions

Board-invisible drill-down for this card:

- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`_decomposition.md`](_decomposition.md) — this project's warranted briefs
  (**architecture** and **testing**, per the decomposition's brief table),
  authored by `plan-briefs`
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering
  AC-001 – AC-016, authored by `plan-briefs`
- [`../_decomposition.md`](../_decomposition.md) — the initiative's project DAG and
  traceability matrix
- [`../initiative.md`](../initiative.md) — the initiative charter
