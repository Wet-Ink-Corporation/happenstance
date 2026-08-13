---
item: HS-S0035
stage: spec
created: 2026-08-12T13:46:34.058Z
updated: 2026-08-12T13:46:34.058Z
template_sig: 87bbf1d0
rendered_sig: fedece46
---

# Spec — ADR-0022, written first and carrying a measured number

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) — BR-15 (decisions recorded), DoD 3 |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) — the project DAG; this project's rank and merge position |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) — AC-013, DR-06, DoD 3 |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture §3** (runtime seam), **§5** (`index_arms()`), **§6** (write path, `SQLITE_BUSY`, pragmas), **§7** (schema), **§9** (tensions), **§11** (left to the implementer), **§12** (*what ADR-0022 must carry*); **testing §2** (AC-013 row), **§4** (no watchdog) |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `bench-harness-and-adr`, row `adr-0022-append-condition-strategy` |
| Signed-off design | [`../_design.md`](../_design.md) — records **no user-facing surface** for this project. Nothing in this story renders one, and the no-surface determination is the thing that was approved |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/adr-0022-append-condition-strategy/spec.md` |
| Roadmap pointers | `RUNBOOK.md:4166-4237` (phase 8 in full); `RUNBOOK.md:284-301` (the ADR queue, whose row **0022** is the obligation this story discharges); `RUNBOOK.md:267` (*"an ADR that cannot be stated as one question is two ADRs"*) |

## One-line PR slice

Measure the three append-condition candidates and the tag-storage options against
real SQLite through `event_store_benchmarks!`, and commit ADR-0022 — atom plus long
record — carrying the winner, the losers, the amended schema, the runtime seam, the
`index_arms()` rejection and a number.

## Executive summary

This PR lands the **decision substrate** every implementation story in
`durable-event-store` reads, and the **reproducible measurement** underneath it. It
writes no production code at all.

Delta against what the project already says:

- The project scope says ADR-0022 is "written before the code" and "carries a
  benchmark *number*, not a claim" (`project.md`, *In scope*, first bullet). This
  spec fixes **where the number comes from** given that
  `SqliteEventStore::append` is still `todo!()` when the record must be written:
  three candidate `EventStore` implementations live in a **new out-of-workspace
  experiment crate** at `experiments/append-condition/`, driven by the slice-mate's
  `event_store_benchmarks!` verbatim. The pattern is not invented here — it is
  exactly `experiments/wire-format/Cargo.toml`'s empty-`[workspace]` trick and
  exactly what `experiments/position-visibility/` did for the Postgres visibility
  question one phase before any Postgres adapter existed.
- The architecture brief's §12 lists eight subjects ADR-0022 must carry, while the
  ADR queue's own rule is that a record answers **one** question
  (`RUNBOOK.md:267`). This spec resolves that rather than leaving it: seven of the
  eight are **consequences** of one question and are recorded as such, and the two
  that are not consequences are recorded as **non-verdicts with a named owner**
  (§*Context pack*, D6).
- `CLAUDE.md` forbids hand-writing `.kb/` atoms. So this story **stages** the atom's
  source in `.kb/_intake/` and authors the long record under `references/adr/`; the
  atom itself is minted by a `/redkiln:kb-ingest` run at a human handoff. That
  handoff is a deliverable of this story, not an afterthought (D7).

Nothing else in phase 8 may start until this merges: AC-013 puts the record before
the implementation, and `schema-migration-and-identity` reads the amended schema out
of it.

## Context pack

Read this section before opening anything. Each item is a decision this story must
honor, not a pointer.

**D1 — The one question, and the half of the queue row that was already settled.**
`RUNBOOK.md:301` states row 0022 as *"SQLite: driver, schema, tag storage, and the
append-condition strategy."* The **driver half is stale on arrival**: `rusqlite`,
deliberately without a pool, is already settled in the crate's own module doc
(`crates/happenstance-sqlite/src/lib.rs:47-53`), which says so in terms and names
`sqlx` as `happenstance-postgres`'s end of that axis. ADR-0022 therefore answers one
question — **how does a SQLite adapter evaluate an append condition atomically, and
what schema, tag storage and runtime does that answer force?** — and says the driver
was ratified rather than decided, the way [ADR-0008](../../../../.kb/decisions/0008-one-derivation-for-both-ports.md)
records that the first half of its own queue question was stale when it arrived
(`RUNBOOK.md:286`). Everything else in §12's list hangs off that question as a
consequence; state the hang, do not merely list them.

**D2 — The candidates are named already; do not invent a fourth.**
`crates/happenstance-sqlite/src/lib.rs:56-62` names exactly three: `BEGIN IMMEDIATE`
plus an `EXISTS` probe; a conditional `INSERT ... SELECT ... WHERE NOT EXISTS`; a
monotonic-position guard. `:63-65` names exactly three tag-storage options: a join
table, a canonical serialised blob, SQLite's JSON1. `Tags` is canonically sorted
*precisely so the blob option stays open* — so the blob arm must be measured, not
dismissed on taste. The architecture brief already **recommends** `BEGIN IMMEDIATE`
+ probe (§6) and the amended join table (§7); a recommendation is not a measurement,
and this story's whole reason to exist is that AC-013 refuses a preference.

**D3 — The schema the sketch gets wrong, and why the measurement can see it.** The
sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` keys
`event_tag(tag, position)` with **no type column**, so a `QueryItem`'s type
constraint becomes a join back to `event` — walked *under the `BEGIN IMMEDIATE`
write lock*, which serialises every writer (`RUNBOOK.md:4178-4187`). The correction
is `event_type` as a **covering column** with the key left `(tag, position)` so the
range stays sorted by position, plus a `tag_cardinality` table because multi-tag
arms must be probed most-selective-tag-first and `ANALYZE` stores only an average.
This is currently *published documentation that is known to be wrong*: the ADR is
where the correction becomes citable, and `schema-migration-and-identity` is where
the doc comment is fixed.

**D4 — `synchronous` is this experiment's `fsync`, and weakening it makes every arm
look free.** `experiments/position-visibility/README.md` records that its whole
result stands on `fsync=on`, and that `setup.sh` *aborts* rather than produce a
number under `fsync=off`, because what these mechanisms charge for is the length of
an interval held across a durable commit. The SQLite analogue is exact, and it is
also a **correctness** constraint here, not only an honesty one:
`spec/SPECIFICATION.md:7481-7484` names `PRAGMA synchronous = OFF` **by name** as a
wrong implementation that CF-14's reopen rule exists to reject. The experiment reads
back its own `synchronous` and journal-mode settings and refuses to emit a number
under a setting the adapter may not ship.

**D5 — `SQLITE_BUSY` is an architecture decision the record owes a value for, and an
unbounded busy handler is a hung CI job.** With N connections on one file,
`BEGIN IMMEDIATE` on a busy database returns `SQLITE_BUSY` *immediately* unless a
busy handler is configured, and that error becomes `AppendError::Store` →
`Attempt::Failed` → a red rule that is not about the adapter's logic (architecture
brief §6). There is **no watchdog anywhere in the suite and there must not be one**
(CF-33, `crates/happenstance-testkit/src/concurrency.rs:24-43`), so an *unbounded*
handler converts a livelock into a hang that names no rule. ADR-0022 records a
**finite and generous** timeout value, journal mode, and the `synchronous` setting —
three numbers, each a documented property of the adapter.

**D6 — Two things this record must be careful *not* to settle, and it must say so
out loud.** An accepted decision atom is immutable (`.kb/decisions/README.md`), so a
paragraph written on evidence this story cannot produce cannot be corrected later —
it can only be superseded. Therefore:

- **ES-17 / `&[Event]` versus `Vec<Event>`.** ADR-0012 names phase 8 as its lifting
  measurement, and restates its falsifier so a positive result cannot be
  manufactured — item 1 is *"two builds of the **same** SQLite adapter differing only
  in `append`'s ownership, measured on the same harness"*
  (`references/adr/0012-append-shape-and-preconditions.md:244-266`). Three candidate
  stores in an experiment crate are **not** two builds of the same adapter, so this
  story cannot lift the marker. ADR-0022 records that as an explicit **non-verdict**:
  the falsifier restated, what the experiment *did* observe about multi-row insert
  copy cost, and the statement that no story in this project's map is currently
  assigned to produce item 1 — a **recorded gap escalated to the ADR queue**, never
  absorbed quietly.
- **CF-40's clause home.** ADR-0015 both claims and disclaims the clause that lets a
  fixture declare numeric limits, and
  `.kb/open-questions/cf-40-fixture-limits-ownership.md` names phase 8 as what forces
  it. This project needs the **capability**; it gets no say in the clause's home
  (architecture brief §9). Record and escalate; do not settle in passing.

**D7 — The ADR lives in two places, and only one of them may be hand-written.**
`CLAUDE.md`, *Where the work lives*: the long record under `references/adr/` carries
the transcripts, the rejected alternatives and the measurement tables a summary
cannot hold; the atom under `.kb/decisions/` carries the frontmatter, status and
supersession graph `redkiln validate --kb` enforces. **Atoms are authored by
`/redkiln:kb-ingest` from `.kb/_intake/`, not by hand** — hand-writing them produces
the directory layout of the process without the process, which is why the first
attempt was reverted (`0269720`). So this story writes
`references/adr/0022-append-condition-strategy.md` directly, stages the atom's source
and the measurement's evidence source under `.kb/_intake/`, and **hands off** to a
human-invoked `/redkiln:kb-ingest`. Evidence is its own atom kind: the decision cites
a `reference` atom rather than swallowing the measurement, which is what lets the
decision be superseded without invalidating the numbers underneath it
(`.kb/decisions/README.md`, *What does not belong here*), and
`.kb/reference/position-visibility-experiment-2026-08.md` is the precedent shape.

**D8 — The persona slice.** The reader here is the **adapter author** — the person
who, three stories from now, opens the SQLite crate to replace `append`'s `todo!()`.
What they must be able to do after this merges, without re-deriving anything: read
one atom, learn which SQL shape to write and why the other two lost with the figure
attached, learn what the schema must hold and why the crate's own doc comment
disagrees, learn where `spawn_blocking` gets its runtime, learn the three pragma
values, and learn which two questions were deliberately left open and who owns them.
A record that makes them go and measure it again has failed AC-013 even if every
sentence in it is true.

## Integration contract

- **Archetype**: `foundation` — real in-tree decision substrate, consumed by capability
  slices in this same initiative. It is not a double, a flag or a fixme.
- **Slice / milestone**: `bench-harness-and-adr`. **Slice-mate**: `benchmark-harness`
  (`HS-S0034`), implemented in the same context and mounted as one surface. This
  story is second in that slice and hard-depends on it.
- **Mount point**: `.kb/decisions/0022-append-condition-strategy.md` — the decision
  atom in the corpus every downstream story loads. This story does not hand-write
  that file (D7): it stages `.kb/_intake/0022-append-condition-strategy.md` and the
  measurement's evidence source beside it, and the atom is minted by the
  `/redkiln:kb-ingest` handoff, which also indexes it in `.kb/maps/decision-map.md`.
  An ADR that no map indexes and no downstream story cites is the decision-corpus
  equivalent of a component that was built and never mounted.
- **Wires into**:
  - `crates/happenstance-testkit/src/bench.rs` + the `bench` feature — the slice-mate's
    deliverable. This story is its **first non-`MemoryFixture` consumer**, and is
    therefore the first thing that can show the emitter parameter (CF-23) and the
    workload set are usable by something that is not the reference fixture.
  - `crates/happenstance-testkit/src/contract.rs` — `Fixture`, and the
    limits-are-facts distinction at `:44-54`. The candidate stores are driven through
    real fixtures, one temp file each.
  - `crates/happenstance-sqlite/src/lib.rs:56-65` — the candidate and tag-storage
    lists this record closes, **read** here and edited by
    `instrument-markers-removed-and-gate-green`, not by this story.
  - `crates/happenstance-sqlite/src/event_store.rs:36-54` — the wrong sketch the
    measurement indicts; `:26-32, :174-178` — the `NoRuntime` variant the runtime-seam
    paragraph decides the fate of.
  - `crates/happenstance-testkit/src/concurrency.rs:206` — `CONTENDERS = 8`, and its
    doc's own claim that it is *not a tuning knob*. The contention workload at 64 is
    the number AC-005 needs and this story supplies.
  - `references/adr/` + `.kb/_intake/` + `.kb/maps/decision-map.md` — the two-places
    rule and the index.
  - `RUNBOOK.md:301` — the queue row, struck through and marked **Written** in the
    shape rows 0008, 0009, 0010 and 0016 already use.
- **Renders surfaces**: **none.** `_design.md` records *"N/A — no user-facing
  surface"* for this project and that determination is what was signed off. This
  story renders nothing and must not introduce a surface.
- **Public items**: none. `_design.md`'s `## Items` block is `N/A`; this story adds no
  `pub` item to any published crate.
- **Conformance rule(s)**: **none, and that is the point.** This story is not
  adapter-observable: it adds no rule, changes no rule, and must leave
  `for_each_event_store_rule!`'s count exactly where `benchmark-harness` left it
  (CF-34, and AC-012's structural claim). The behaviours it decides become observable
  three stories later, through `append_returns_last_written_position`,
  `empty_batch_is_refused_before_the_condition_is_evaluated` and the racing rules
  that distinguish `Attempt::Rejected` from `Attempt::Failed`
  (`crates/happenstance-testkit/src/concurrency.rs:214-231`).
- **Clause(s)**: **discharges none, amends none, moves no marker.** It *records
  verdict positions* that later stories in this project discharge: CF-14
  (`spec/SPECIFICATION.md:7471-7492`) via the `synchronous` value it fixes, and CF-17
  / ES-35 (`:7570-7583`, `:4142-4167`) via the durability settings the reopen rules
  will run against. `spec/SPECIFICATION.md` is **not** edited by this story — that is
  `reopen-negative-control-and-durability-verdicts` and
  `spec-and-code-reconciliation`. Changing a `[FROZEN]` clause takes a new ADR, and
  ADR-0022 is not that ADR.
- **Advances DoD scenario**: initiative **DoD 3** — *"the durable store passes the
  suite for real … including the concurrency case at 64 contenders"*
  (`initiative.md:366-369`). This story does not turn DoD 3 green; it makes it
  *reachable*, and its contention measurement is what turns 64 from a number two
  proof artefacts assert into a number something verified. It also directly discharges
  project **DoD 3** (*"ADR-0022 exists as an accepted decision atom … written through
  the runbook's ADR queue rather than as a side effect"*).

## PR boundary

**In this PR**

- `experiments/append-condition/` — a new, out-of-workspace crate: three candidate
  `EventStore` implementations over `rusqlite`, their fixtures, a `run.sh`/README
  that states the machine and the pragma settings, and the recorded results. It
  carries an empty `[workspace]` table so cargo does not adopt it, exactly as
  `experiments/wire-format/Cargo.toml` does, and it adds **no dependency to any
  workspace `Cargo.toml`**.
- `references/adr/0022-append-condition-strategy.md` — the long record.
- `.kb/_intake/` — the staged atom source and the staged evidence source for the
  `/redkiln:kb-ingest` handoff.
- `RUNBOOK.md` — the queue row 0022 marked written; the session log for phase 8.
- This story's own backlog folder, including `_ledger.md`.
- The composition-root/wiring files named in the Integration contract, where mounting
  this slice requires it — that is not scope drift.

**Explicitly not in this PR**

- **Any file under `crates/happenstance-sqlite/src/`.** Not one `todo!()` is
  replaced, not one word of the wrong module-doc sketch is corrected, and
  `#![allow(clippy::todo)]` stays. AC-013's whole content is that the record precedes
  the implementation; a PR that does both has destroyed the ordering it was written
  to prove. The doc-comment correction is `schema-migration-and-identity`'s and
  `instrument-markers-removed-and-gate-green`'s.
- **`.kb/decisions/**` and `.kb/maps/**` written by hand.** The ingest run owns them
  (D7). If a hand-written atom appears in this diff, the story is wrong regardless of
  its content.
- **`spec/SPECIFICATION.md`.** No clause text, no marker, no citation range.
- **`crates/happenstance-testkit/src/**`.** The harness is the slice-mate's; raising
  `CONTENDERS` is `concurrency-family-and-contender-count`'s.
- **`crates/happenstance-sqlite/Cargo.toml`.** The description, README and licence
  files are `crates-io-name-and-packaging-facts`'s; `publish = false` stays
  everywhere.

**Merge DoD**: the long record and its staged atom source exist and quote a measured
figure with its conditions; `cargo xtask affected --base {{base}}` and
`cargo xtask ci --fast` are green on a tree in which no SQLite production file moved;
`redkiln validate --kb && redkiln doctor` are clean with exactly the six expected
`template-drift` advisories.

```
experiments/append-condition/**
references/adr/0022-append-condition-strategy.md
.kb/_intake/**
RUNBOOK.md
.bklg/from-contract-to-published-library/sqlite-durable-store/adr-0022-append-condition-strategy/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The measurement runs where the adapter cannot** | Three candidate `EventStore` implementations — `BEGIN IMMEDIATE` + `EXISTS` probe, conditional `INSERT … SELECT … WHERE NOT EXISTS`, monotonic-position guard — live in a new out-of-workspace crate and are driven by `event_store_benchmarks!` **verbatim**, so the figures are the same workloads a later `event_store_benchmarks!(SqliteFixture::new())` run re-derives. Not in `members`, no gate step, no workspace `Cargo.toml` touched. | `crates/happenstance-sqlite/src/lib.rs:56-62`; `experiments/wire-format/Cargo.toml:1-6` (the empty-`[workspace]` trick); `experiments/position-visibility/README.md:22-26` |
| **Durability settings are stated and enforced, not assumed** | The experiment reads back `journal_mode` and `synchronous` and refuses to emit a number under a setting the shipped adapter may not use. `PRAGMA synchronous = OFF` is named by the specification as a wrong implementation, so a number produced under it is not merely dishonest — it is a number for a store that fails CF-14. | `spec/SPECIFICATION.md:7481-7484`; `experiments/position-visibility/README.md:47-52` |
| **The tag-storage arms are measured, not preferred** | Join table with `event_type` as a covering column and key left `(tag, position)`, against a canonical serialised blob, against JSON1 — under a probe that runs while the write lock is held. `Tags` is canonically sorted so the blob arm stays viable; dismissing it without a number contradicts the reason that sorting exists. | `crates/happenstance-sqlite/src/lib.rs:63-65`; `RUNBOOK.md:4178-4187`; `crates/happenstance-sqlite/src/event_store.rs:36-54` |
| **The contention workload runs at 64 connections on one file** | The runbook's "conditional append under contention" workload, at the contender count both phase-8 proof artefacts already claim, so the file-descriptor and busy-contention cost of raising `CONTENDERS` from 8 to 64 is a measured claim rather than an assumed one. The raise itself is a later story's. | `crates/happenstance-testkit/src/concurrency.rs:200-206`; `RUNBOOK.md:4217-4222`; `RUNBOOK.md:2686-2696` |
| **The long record answers one question and names what lost** | `references/adr/0022-append-condition-strategy.md`, in the established shape: Status / Date / Settles / Rests on and does not settle / the question and the one it has quietly become. The driver half of the queue row is recorded as **stale on arrival**, ratified rather than decided. | `references/adr/0016-the-wire-format.md:1-30`; `RUNBOOK.md:267`, `RUNBOOK.md:286`, `RUNBOOK.md:301`; `crates/happenstance-sqlite/src/lib.rs:47-53` |
| **The record carries the consequences, each with its rejected alternative** | Runtime seam: a `tokio::runtime::Handle` captured at construction with `try_current()` as fallback, against running inline on the calling thread — and if (a) wins, `NoRuntime` keeps a real meaning; if (b) wins, the variant and two module-doc paragraphs must be removed in the implementing story rather than left describing an unreachable state. `index_arms()`: **rejected** — it does not exist in `happenstance-core`, the decomposition stays adapter-private, and the re-open trigger is named (`postgres-and-neon-stores` independently needing it). Pragmas: busy timeout finite and generous, journal mode, `synchronous` — three values. | `../_decomposition.md` architecture §3, §5, §6; `crates/happenstance-sqlite/src/event_store.rs:26-32`; `crates/happenstance-core/src/query.rs:204` (`items()`, and no `index_arms`) |
| **Two non-verdicts are recorded with owners, not silently absorbed** | ES-17's `&[Event]` question is **not** lifted here — falsifier item 1 requires two builds of the same adapter, which no experiment crate can be — and no story in this project's map currently owns producing it, so the gap is escalated to the ADR queue. CF-40's clause home is recorded as still open. | `references/adr/0012-append-shape-and-preconditions.md:244-266`; `.kb/open-questions/cf-40-fixture-limits-ownership.md`; `../_storymap.md` *Coverage* |
| **The atom is minted by the process, not by hand** | `.kb/_intake/` carries the atom source and a separate evidence source; `/redkiln:kb-ingest` mints `.kb/decisions/0022-…`, a `reference` atom for the measurement, and the `.kb/maps/decision-map.md` row. A successful run clears `_intake`, so a file still sitting there is a file the run did not ingest — and the intake `README.md` is dropped at the approval gate rather than ingested. | `CLAUDE.md` *Where the work lives*; `.kb/_intake/README.md:14-20`; `.kb/decisions/README.md:7-24`; `.kb/reference/position-visibility-experiment-2026-08.md` |
| **No production code moves and the gate is unchanged** | No `todo!()` replaced, no `#![allow(clippy::todo)]` deleted, no clause edited, no conformance rule added or renamed. `cargo xtask ci --fast` green on a tree where the only compiled change is outside the workspace. | `crates/happenstance-sqlite/src/lib.rs:75-80`; `.redkiln/config.yaml:40,48,55` |

## Data and migrations

**No migration ships in this PR** — but this story *decides* the migration that
`schema-migration-and-identity` ships, so the shape is stated here as a decision
rather than deferred to a code review.

Migration 1, as ADR-0022 must specify it (architecture brief §7, `RUNBOOK.md:4178-4187`):

- `event(position INTEGER PRIMARY KEY AUTOINCREMENT, event_type, data, metadata, tags, …)`.
  `AUTOINCREMENT` is load-bearing rather than stylistic: positions must never be
  reused after a delete, and plain `rowid` does not guarantee that. It also *permits
  gaps*, which is why no rule and no code may assume `+1`.
- `event_tag(tag, position)` `WITHOUT ROWID`, key left `(tag, position)` so the range
  stays sorted by position, **with `event_type` carried as a covering column** — the
  single correction that stops a type constraint becoming a join back to `event`
  walked under the `BEGIN IMMEDIATE` write lock.
- `tag_cardinality` — multi-tag arms are probed most-selective-tag-first and SQLite
  cannot supply per-value cardinality; `ANALYZE` stores only an average.
- The `EventId` origin pair — origin store and origin position, `UNIQUE`
  **together**: both the index `contains_event_id`'s probe seeks and the constraint
  that stops ingest storing one event twice
  (`crates/happenstance-core/src/identity.rs:97`;
  `crates/happenstance-sqlite/src/event_store.rs:237-244`).
- `recorded_at`, read back rather than re-stamped on reopen.
- The store's own `StoreId`, **minted once at schema creation and persisted**.
  Mint-per-open is permitted by VT-6 in general and is the wrong choice for a
  file-backed store; the testkit's own `DurableFixture` switched to mint-once for
  exactly this reason.

Migration must be **idempotent and safe under a concurrent open**, because
`SqliteEventStore::open` calls `migrate` on every connect
(`crates/happenstance-sqlite/src/event_store.rs:118-124`) and the fixture connects
twice onto one file: `IF NOT EXISTS` inside `BEGIN IMMEDIATE`, and
`INSERT OR IGNORE` then read-back for the `StoreId` row. The projection store's
checkpoint schema migrates independently on its own connection and must be equally
idempotent — an event store and a projection store on one file are two connections,
not one.

**Experiment data is throwaway and self-cleaning.** Each candidate fixture owns one
temporary file, created per instance and removed on `Drop`; nothing under
`experiments/` is a durable artefact except the README, the runner and the recorded
results table. No workspace dependency is added for it — `tempfile` is not in
`[workspace.dependencies]`, and a process-local ordinal plus `Drop` cleanup has
precedent at `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102`.

## Acceptance criteria

The persona is the **adapter author** (`initiative.md:210-214`), on the journey
*"Learn when you are finished"* (`initiative.md:245-246`) — here at its first
moment, three stories before they open `append`'s `todo!()`. Every criterion below
is stated as what that person can do afterwards without re-deriving anything; each
one, taken alone, is a way project **AC-013** can fail.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the adapter author must pick an append-condition SQL shape while `SqliteEventStore::append` is still `todo!()`, so there is nothing in the workspace to measure, **WHEN** they ask where the number came from, **THEN** all three candidates named at `crates/happenstance-sqlite/src/lib.rs:56-62` — `BEGIN IMMEDIATE` + `EXISTS` probe, conditional `INSERT … SELECT … WHERE NOT EXISTS`, monotonic-position guard — exist as real `EventStore` implementations over `rusqlite` in `experiments/append-condition/`, are driven by `event_store_benchmarks!` **verbatim** rather than by a bespoke timing loop, and **each one passes `event_store_conformance!` before its figure is allowed to count** — so no arm wins by being fast and wrong | `experiments/append-condition/tests/candidates_are_conformant.rs` — `event_store_conformance!` green against each of the three candidate fixtures (a red run there disqualifies that arm's number); `experiments/append-condition/tests/measure.rs` invokes `happenstance_testkit::event_store_benchmarks!` per candidate; `git diff --name-only` shows no change to the root `Cargo.toml`, any workspace member's `Cargo.toml`, or `Cargo.lock`, and `experiments/append-condition/Cargo.toml` carries an empty `[workspace]` table (`experiments/wire-format/Cargo.toml:1-6`) |
| **AC-002** | **GIVEN** the specification names `PRAGMA synchronous = OFF` by name as a wrong implementation CF-14's reopen rule exists to reject (`spec/SPECIFICATION.md:7481-7484`), so a figure produced under it is a figure for a store that cannot ship, **WHEN** the runner starts, **THEN** it reads back `journal_mode` and `synchronous` from the live connection and **aborts rather than emit a number** under a setting the shipped adapter may not use, and every figure in the record is printed beside the settings, machine and SQLite version it was produced on | `experiments/append-condition/tests/durability_settings_are_enforced.rs` — a positive control that forces `synchronous = OFF` and asserts the runner refuses (mirroring `experiments/position-visibility/README.md:47-52`, whose `setup.sh` aborts under `fsync=off`); `experiments/append-condition/results/` and the record's conditions block both carry the read-back values; review checks the two agree |
| **AC-003** | **GIVEN** `schema-migration-and-identity` is the very next story and must write migration 1 without re-deriving it — and the crate's own published sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` is *known to be wrong* in a way that serialises every writer, **WHEN** its implementer opens the record, **THEN** all three tag-storage options (`lib.rs:63-65`) were measured under a probe held under the `BEGIN IMMEDIATE` write lock — the canonical blob arm measured rather than dismissed, since `Tags` is sorted precisely to keep it open — and the record specifies migration 1 outright: `event_tag(tag, position)` `WITHOUT ROWID` with **`event_type` as a covering column and the key left `(tag, position)`**, `tag_cardinality`, the `EventId` origin pair `UNIQUE` **together**, `recorded_at` read back not re-stamped, `AUTOINCREMENT`, and a `StoreId` minted once at schema creation — naming what the sketch got wrong and why | `experiments/append-condition/results/tag-storage.md` carries a figure for each of the three arms under identical conditions; review checks every column above appears in `references/adr/0022-append-condition-strategy.md`'s schema block and that `crates/happenstance-sqlite/src/event_store.rs:36-54` is cited there as the wrong sketch; `git diff --name-only` shows that file **unchanged** (its correction is `schema-migration-and-identity`'s) |
| **AC-004** | **GIVEN** project AC-013 refuses a preference and the architecture brief already *recommends* `BEGIN IMMEDIATE` + probe (§6), **WHEN** the adapter author reads the decision paragraph, **THEN** exactly one strategy is stated as chosen, the other two are named as **lost with the measured figure and its conditions attached**, and the driver half of the queue row (`RUNBOOK.md:301`) is recorded as **ratified rather than decided** — stale on arrival, because `rusqlite` without a pool is already settled at `crates/happenstance-sqlite/src/lib.rs:47-53` — so the record answers one question the way `RUNBOOK.md:267` demands | Review against `experiments/append-condition/results/append-condition.md`: the figure quoted in `references/adr/0022-append-condition-strategy.md` is **the same figure**, reproducible by re-running `experiments/append-condition/run.sh`; `RUNBOOK.md:301`'s queue row is struck through and marked **Written** in the shape rows 0008/0009/0010/0016 already use; ledger row cites `references/adr/0022-append-condition-strategy.md:<line>` and `results/append-condition.md:<line>` |
| **AC-005** | **GIVEN** both of phase 8's stated proof artefacts read *64 contenders* while `concurrency::CONTENDERS` is **8** and its own doc says it is not a tuning knob (`crates/happenstance-testkit/src/concurrency.rs:200-206`), **WHEN** `concurrency-family-and-contender-count` later moves that constant, **THEN** it inherits a **measured** claim rather than an assumed one: the contended workload was run at **64 connections on one file**, and the record states what 64 cost — file descriptors, busy contention, the committed/rejected mix and wall time — and whether the raise is supportable; and this story **does not itself touch `concurrency.rs`** | `experiments/append-condition/results/contention-64.md` — the contended scenario at k = 64 with its `ConditionViolated` split, produced by the same `event_store_benchmarks!` scenario (b); `git diff --name-only` shows `crates/happenstance-testkit/**` untouched; review checks the record's `CONTENDERS` paragraph cites that results file and states one of AC-005's two permitted outcomes as a *recommendation to the later story*, not as an edit |
| **AC-006** | **GIVEN** the architecture brief's §12 lists eight subjects while the queue's rule is that a record answers one question, **WHEN** the adapter author reads the record end to end, **THEN** the seven consequences hang visibly off the one question and **each carries the alternative that lost**: the runtime seam as a captured `tokio::runtime::Handle` with `try_current()` as fallback, against running inline on the calling thread — with the fate of `SqliteEventStoreError::NoRuntime` (`crates/happenstance-sqlite/src/event_store.rs:26-32`) spelled out under whichever wins; `index_arms()` **rejected**, because it does not exist in `happenstance-core` (`crates/happenstance-core/src/query.rs:204` has `items()` and no `index_arms`), the decomposition stays adapter-private, and the re-open trigger is named (`postgres-and-neon-stores` independently needing it); and **three pragma values** — a **finite and generous** busy timeout, the journal mode, the `synchronous` setting | Review checklist against `references/adr/0022-append-condition-strategy.md`: one `Settles:` line, seven consequence sections each with a named rejected alternative, three numeric pragma values; `rg -n "index_arms" crates/happenstance-core/src` still returns nothing (no API was minted as a side effect); `rg -n "NoRuntime" references/adr/0022-append-condition-strategy.md` finds the paragraph deciding the variant's fate |
| **AC-007** | **GIVEN** an accepted decision atom is immutable (`.kb/decisions/README.md`), so a paragraph written on evidence this story cannot produce can only ever be *superseded*, **WHEN** the record reaches the two subjects it must not settle, **THEN** each is recorded as an explicit **non-verdict with a named owner**: ES-17 / `&[Event]` vs `Vec<Event>` is **not lifted**, with ADR-0012's falsifier item 1 restated verbatim (*two builds of the same adapter*, `references/adr/0012-append-shape-and-preconditions.md:244-266`), what the experiment *did* observe about multi-row insert copy cost, and the statement that **no story in this project's map currently owns producing it** — escalated to the ADR queue; and CF-40's clause home is recorded as still open, citing `.kb/open-questions/cf-40-fixture-limits-ownership.md`, which this story does not edit | `rg -n "0012\|falsifier\|Vec<Event>" references/adr/0022-append-condition-strategy.md` finds the restated falsifier and the not-lifted verdict; `git diff --name-only` shows `.kb/open-questions/cf-40-fixture-limits-ownership.md` **unmodified**; `RUNBOOK.md`'s ADR queue carries the escalation row for the unowned ES-17 measurement; review checks neither subject is stated as decided |
| **AC-008** | **GIVEN** `CLAUDE.md` forbids hand-writing `.kb/` atoms — the first attempt at that was reverted at `0269720` — and the mount point every downstream story loads is `.kb/decisions/0022-append-condition-strategy.md`, **WHEN** this story finishes, **THEN** the atom's source and a **separate evidence source** are staged under `.kb/_intake/` (the decision cites a `reference` atom rather than swallowing the measurement, so the decision can be superseded without invalidating the numbers — `.kb/reference/position-visibility-experiment-2026-08.md` is the precedent shape), the human-invoked `/redkiln:kb-ingest` handoff is recorded as a named deliverable, and **no `.kb/decisions/**` or `.kb/maps/**` file appears in this diff** | `git diff --name-only` contains no path under `.kb/decisions/` or `.kb/maps/`, and exactly two new paths under `.kb/_intake/` (`.kb/_intake/README.md` is not one of them — it is dropped at the approval gate, `.kb/_intake/README.md:14-20`); both staged files carry valid `KbFrontmatter`; `redkiln validate --kb && redkiln doctor` clean with exactly the six expected `template-drift` advisories |
| **AC-009** | **GIVEN** AC-013's entire content is that the record **precedes** the implementation, so a PR that does both has destroyed the ordering it was written to prove, **WHEN** the diff is read, **THEN** not one file under `crates/happenstance-sqlite/src/` has moved, `#![allow(clippy::todo)]` is still there, no clause in `spec/SPECIFICATION.md` and no marker has changed, the conformance rule count is exactly where `benchmark-harness` left it, and the gate is green on a tree whose only compiled change is **outside the workspace** | `git diff --name-only` against the declared PR boundary — nothing under `crates/**` or `spec/**`; `cargo xtask affected --base {{base}}` green; `cargo xtask ci --fast` green; the rule-name count from `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`) identical either side of the diff |

**Coverage of the traced project AC.** This story traces to project **AC-013**
only, and all nine criteria above serve it, each closing a different way it can be
claimed and be false: *committed before the implementation* → AC-009; *states the
strategy with the alternatives that lost* → AC-004, AC-006; *quotes a measured
benchmark figure rather than a preference* → AC-001, AC-002, AC-003, AC-005; *is an
accepted decision atom written through the runbook's ADR queue* (project DoD 3) →
AC-008; and the immutability tax that makes over-claiming permanent → AC-007.

## Interaction quality

**This story renders no surface, and the no-surface determination is what was
approved.** `_design.md` records `N/A — no user-facing surface` in every block and
was signed off on 2026-08-12 at the `/redkiln:plan` gate, with the `design.capture`
perceptual review a *declared* skip. There is therefore no composed control, no
placement, no transience policy and no density budget to bind to — and inventing one
here would contradict a signed-off design rather than honour it.

RFC §6.7/D6 is still answered rather than waved away: both families are enumerated,
each is marked applicable or not with its reason, and **every invariant that does
apply is carried by an AC-### row in the table above** — there is no invariant in
this section that exists only as a bullet.

| Family | Invariant | Applies? | Carried by |
| --- | --- | --- | --- |
| **State** | In-place vs context-jump; non-occlusion; preserved focus / scroll / selection; keyboard reachability | **No** — nothing is rendered, focused, scrolled or selected. `_design.md`, *Surfaces* | — |
| **State** | Reversibility | **Yes, in its non-UI form.** An accepted decision atom is immutable, so the only "undo" is a superseding atom. That is why a paragraph written on evidence this story cannot produce is a defect rather than a rough edge | **AC-007** (non-verdicts recorded with owners rather than settled), **AC-008** (the atom is minted by the process, so supersession has a graph to live in) |
| **Composition** | Presentation exists at all — every control carries real composed presentation, not bare markup | **Yes, in its non-UI form.** The analogue of an unstyled render is a record whose sentences are all true and which still sends the reader back to measure it again — the failure D8 names explicitly. What makes it *composed* is a figure with its conditions attached and a named loser beside every winner | **AC-004**, **AC-006** |
| **Composition** | Composition / placement — the artifact is reachable from where the reader already is | **Yes, in its non-UI form.** An ADR no map indexes and no downstream story cites is the decision-corpus equivalent of a component built and never mounted (*Integration contract*, mount point) | **AC-008** (the `.kb/maps/decision-map.md` row arrives with the atom, via ingest) |
| **Composition** | Transience — persistent vs revealed vs opened-on-demand | **Yes, in its non-UI form**, and it is exactly `CLAUDE.md`'s two-places rule: the ~100-line atom is the persistent chrome every downstream story loads; the long record under `references/adr/` is opened on demand and holds the transcripts, tables and rejected alternatives a summary cannot | **AC-008** (the atom is staged, not hand-written), **AC-006** (the long record carries the consequences) |
| **Composition** | Density budget, with its real numbers | **Yes, in its non-UI form.** The atom is the ~100-line grain the corpus already uses (`.kb/decisions/`, seventeen atoms); the long record is unbounded — the corpus's longest is 1,508 lines. Three pragma **values**, three candidate arms, three tag-storage arms, one contender count: the record states numbers, not adjectives | **AC-005** (64), **AC-006** (three pragma values), **AC-003** (three arms) |
| **Composition** | Hierarchy | **Yes, in its non-UI form.** One question at the top; seven consequences beneath it; two non-verdicts fenced off from both, so a reader cannot mistake an open question for a decision | **AC-006**, **AC-007** |
| **Composition** | The design's named anti-patterns | **N/A as written** — `_design.md`'s *Anti-patterns* block is `N/A — no user-facing surface`. The equivalent prohibitions this story is held to come from `CLAUDE.md` and the briefs instead: hand-writing an atom, settling an open question in passing, and shipping a record without a number | **AC-008**, **AC-007**, **AC-004** |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The runner reads back `synchronous = OFF`, or a journal mode the adapter will not ship | **Abort before measuring.** No results file is written and no partial table is emitted. A number under those settings is a number for a store CF-14 rejects (`spec/SPECIFICATION.md:7481-7484`), and emitting it with a caveat is how a caveat becomes a citation |
| **EC-002** | A candidate arm returns `SQLITE_BUSY` under the 64-contender workload | Record it as a **measurement**, not as an error to retry away. The busy timeout stays finite and generous; an *unbounded* handler is forbidden, because there is no watchdog anywhere in the suite (CF-33, `crates/happenstance-testkit/src/concurrency.rs:24-43`) and it would convert a livelock into a hung run naming no rule. A run that hangs is evidence for the record's timeout paragraph |
| **EC-003** | A candidate is fast and fails `event_store_conformance!` | Its figure is **discarded, and the failure is reported in the record** as what that arm costs. A wrong arm is always the fastest; AC-001 exists so that fact cannot decide anything |
| **EC-004** | The arms land within measurement noise of one another | The record says so **and names the tie-break it then used** — correctness surface, diff size, or how many existing comments in the crate stay true — never a fabricated margin. A tie is a finding; a manufactured winner is a lie with a table attached |
| **EC-005** | `/redkiln:kb-ingest` fails validation, or `.kb/_intake/` is non-empty after the run | The story is **not done**. A successful run clears `_intake` (`.kb/_intake/README.md`), so a file still sitting there is a file the run did not ingest, and the mount point named in the Integration contract does not exist |
| **EC-006** | `benchmark-harness` (HS-S0034) has not merged, or its emitter/scenario surface differs from what this spec assumes | **Hard stop and re-plan** — do not fork a private timing loop. The whole point of driving `event_store_benchmarks!` verbatim is that a later `event_store_benchmarks!(SqliteFixture::new())` re-derives the same workloads; a bespoke loop makes the figure unreproducible by the adapter it was measured for |
| **EC-007** | 64 `rusqlite::Connection`s onto one file exhausts a file-descriptor or SQLite connection limit on the measuring machine | That is **the AC-005 finding**, recorded with the limit and the platform, not worked around by quietly measuring at 32. `concurrency-family-and-contender-count` needs to know this before it raises a workspace-wide constant |

## Non-functional

| id | requirement | why it is here |
| --- | --- | --- |
| **NF-001** | **Reproducible by a second person.** `experiments/append-condition/README.md` states the machine, OS, SQLite version, pragma settings and the exact command; `run.sh` re-derives `results/` from a clean checkout | `experiments/position-visibility/README.md:22-45` is the shape; a figure nobody else can re-produce is a claim with a table attached |
| **NF-002** | **Zero blast radius on the workspace.** No entry in `members`, no workspace `Cargo.toml` or `Cargo.lock` change, no `cargo xtask ci` step, no `.redkiln/config.yaml` `verify:` change, no measurement crate (`criterion`, `divan`, …) anywhere in the workspace graph | `experiments/wire-format/Cargo.toml:1-6`; the emitter is a caller-supplied parameter precisely so measurement dependencies stay out of the testkit (CF-23) |
| **NF-003** | **The run terminates.** Every arm's busy timeout is finite; the whole `run.sh` completes unattended within a stated wall-clock budget recorded in the README | CF-33 — no watchdog anywhere means nothing else will notice a hang |
| **NF-004** | **Two artefacts, two densities.** The staged atom source is the corpus's ~100-line grain; the long record is unbounded and carries the transcripts, the rejected alternatives and the tables | `CLAUDE.md`, *Where the work lives*: deleting the long form would discard ~78% of the corpus, and `spec-trace` cites line ranges that only exist there |
| **NF-005** | **The record is legible to the person three stories away**, not only to its author: one question, the answer, the losers with figures, the schema block, the seam, the pragmas, the two open items and their owners — in that order | D8; a record that makes the adapter author measure it again has failed AC-013 even if every sentence is true |
| **NF-006** | **KB and backlog stay clean**: `redkiln validate --kb && redkiln doctor` report exactly the six expected `template-drift` advisories, no seventh and none missing | project DoD 6; the `backlog` CI job asserts the set is exactly those six |

## Implementation notes (non-prescriptive)

Calls the compiler and the measurement are better placed to make than this document
— stated as shape, not as instruction.

- **Order of work.** Conformance first, measurement second. Write the three candidate
  stores until `experiments/append-condition/tests/candidates_are_conformant.rs` is
  green, *then* run `tests/measure.rs`. Inverting that order produces a table you
  cannot use and will not want to throw away.
- **The candidates are small on purpose.** Each is a straight-line `rusqlite`
  implementation of `EventStore` — no pool, no page budget tuning, no read-path
  cleverness beyond what the benchmark scenarios need. `MemoryEventStore`
  (`crates/happenstance-core`) and `crates/happenstance-testkit/src/fixtures.rs`'s
  `MemoryFixture` are the reference shapes for the fixture side; one instance is one
  temp file, each `connect()` a second `rusqlite::Connection` onto it.
- **The tag-storage arms multiply the candidate arms only where it is interesting.**
  The probe shape and the tag storage interact under the write lock, so the arm the
  measurement actually needs is the *chosen* append strategy crossed with all three
  tag storages, plus the two losing strategies at one tag storage each. Say in the
  README which cross you ran and why the others were not.
- **Warm-up and repetition are the runner's, not the harness's.** `event_store_benchmarks!`
  asserts completion and a finite record and never a timing (CF-34); discarding a
  first iteration and reporting a median over *n* belongs in your emitter and in the
  results table.
- **`PAGE_SIZE` is not this story's number.** `crates/happenstance-sqlite/src/event_store.rs:81`
  calls the current 512 "a placeholder until it is measured", and it stays a
  placeholder here (architecture brief §11). If the replay scenario incidentally says
  something about it, record it as an observation for a later story, not as a decision.
- **Write the long record first, stage the atom source from it.** The atom is a
  distillation of the record, not an independent draft; two drafts diverge, and the
  one under `.kb/` is the one that becomes immutable.
- **Read `references/adr/0016-the-wire-format.md:1-30` for the record's head shape**
  (Status / Date / Settles / Rests on and does not settle) and
  `.kb/reference/position-visibility-experiment-2026-08.md` for the evidence atom's
  summary discipline — a summary that carries the actual ratios, not adjectives.
- **The handoff is a step, not a footnote.** Finish with `.kb/_intake/` staged and the
  `/redkiln:kb-ingest` invocation written down for the human who runs it; that command
  is user-invoked by design and must not be called from inside the implementation.

## Tests and CI (merge gate)

Grounded in the testing brief §1 (the tier table), §2 (AC-013's row: *review, not a
test tier* — named there so testing does not silently exclude this project's largest
single piece of required evidence) and §6 (merge-gate commands by grain).

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Experiment conformance** (out of workspace, out of the gate) | `cargo test --manifest-path experiments/append-condition/Cargo.toml` → `tests/candidates_are_conformant.rs` | Every arm whose figure is quoted is a *conformant* store. AC-001, and the precondition for AC-003 and AC-005 meaning anything |
| **Experiment durability control** | `cargo test --manifest-path experiments/append-condition/Cargo.toml --test durability_settings_are_enforced` | The runner aborts under a setting the shipped adapter may not use. AC-002 |
| **Experiment measurement** | `experiments/append-condition/run.sh` → `experiments/append-condition/results/` | The figures themselves, reproducibly. AC-001, AC-003, AC-004, AC-005. **Never a gate step** — CF-34, and NF-002 |
| **Static** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | No clause moved, no citation range broke, the file-reading lints still pass. AC-009 |
| **Story grain (affected)** | `cargo xtask affected --base {{base}}` (`affected_gate`, `.redkiln/config.yaml:40`) | A story whose deliverable maps to no workspace package still gets its lints and `spec-trace` run — the case that block's own comment names. AC-009 |
| **Integration grain** | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`; project DoD 5) | The workspace is exactly as it was: fmt, clippy `-D warnings`, tests, four wasm32 steps, docs, `spec-trace`, packaging assertions. AC-009 |
| **Rule-count check** | The rule-name count from `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`), identical either side of the diff | This story is not adapter-observable and did not become so by accident. AC-009, and it keeps `benchmark-harness`'s AC-012 claim true |
| **Backlog / KB** | `redkiln validate --kb && redkiln doctor` | Staged intake files carry valid `KbFrontmatter`; no accepted atom body was edited; exactly six `template-drift` advisories. AC-008, NF-006 |
| **Review (the tier AC-013 actually lives in)** | `references/adr/0022-append-condition-strategy.md` read against `experiments/append-condition/results/`, and `_ledger.md`'s nine rows read against both | Every figure in the record exists in `results/`; every consequence names a loser; the two non-verdicts are fenced. AC-004, AC-006, AC-007 |

Two things this table deliberately does **not** contain: a threshold assertion on any
timing (CF-34 forbids a benchmark failing a merge, and
`crates/happenstance-testkit/src/concurrency.rs`'s module docs say why nothing here
reads a clock into an assertion), and any new `verify:` entry — the four commands
above are the repository's own, wired from `.redkiln/config.yaml`, not typed fresh
for this story (testing brief AC-T02).

## Risks and coupling (PR-scoped)

- **The slice-mate is a hard predecessor and its surface is still in flight.**
  `benchmark-harness` (HS-S0034) is being specified in parallel; if its emitter arms
  or its three scenarios land differently, this story's `tests/measure.rs` changes
  shape. Mitigation: bind only what the slice-mate's own spec states as its public
  entry point — the three macro arms and the caller-supplied emitter — and treat any
  gap as EC-006 rather than as licence to write a private timer.
- **A SQLite number is a number about a machine.** Unlike the Postgres experiment,
  which ran in a pinned container, this one runs on whatever host is to hand, and
  filesystem and OS differences move SQLite write latency by more than the arms may
  differ. Mitigation: NF-001's stated conditions, a reported spread rather than a
  single figure, and EC-004 — a tie is a legitimate outcome that changes how the
  decision is justified, not something to resolve by re-running until it separates.
- **The immutability tax is charged at merge, not later.** Every over-claimed
  sentence in the atom can only be corrected by a superseding atom
  (`.kb/decisions/README.md`). The pressure to write "and this also settles ES-17" is
  real and AC-007 is what resists it.
- **This is the largest adapter project on the trunk and its first story is a
  document.** Ten runbook-days sit behind it with `publication-and-positioning`
  blocked at the far end (`project.md`, risks). The temptation is to start
  `schema-migration-and-identity` "in parallel" — which inverts exactly the ordering
  AC-013 exists to prove. AC-009's diff check is the mechanical answer.
- **Coupling to `schema-migration-and-identity` runs through prose, not code.** If the
  record's schema block is vague, the next story either re-derives it or implements
  the wrong sketch — the most likely way to ship a slow, conformant adapter
  (`project.md`, risks). AC-003 requires the block to be specific enough to implement
  from, and `RUNBOOK.md:4178-4187` is the correction it must carry.
- **Coupling to `concurrency-family-and-contender-count` runs through a number this
  story measures and does not apply.** Raising `CONTENDERS` is workspace-wide — every
  fixture, including `MemoryFixture` and the mutants, re-runs at 64. This story
  supplies the evidence and stops; a diff that also edits `concurrency.rs:206` has
  taken the other story's decision without its ACs.
- **`/redkiln:kb-ingest` is user-invoked and cannot be called from here** (all seven
  Redkiln SDLC commands carry `disable-model-invocation`). So this story's mount point
  is reached in two moves, and a story that "finishes" without the handoff written
  down leaves the corpus without the atom every downstream story loads.

## Dependencies

**Blocks on**

- **`benchmark-harness`** (HS-S0034, same slice `bench-harness-and-adr`) — ships
  `crates/happenstance-testkit/src/bench.rs`, the `bench` feature and
  `event_store_benchmarks!`. This story is its **first non-`MemoryFixture` consumer**
  and cannot quote a measured figure without it. Matches this story's
  `blocked_by: [HS-S0034]`.

No other story blocks this one. The two project-level edges — `projection-store-freeze`
(HS-P0010) and `typed-layer-and-alpha-release` (HS-P0011) — are recorded in
`project.md`, *Dependencies*, and are not story dependencies here.

**Unlocks**

- **`schema-migration-and-identity`** (HS-S0036, `blocks` in this story's
  frontmatter) — reads migration 1's shape, the pragmas and the identity columns
  straight out of the record (AC-003).
- **`append-atomicity-and-store-limits`** — implements the chosen append-condition
  strategy and the `BEGIN IMMEDIATE` transaction the record decided (AC-004).
- **`concurrency-family-and-contender-count`** — consumes the runtime seam (AC-006)
  and the 64-contender measurement (AC-005).
- **`instrument-markers-removed-and-gate-green`** — corrects the crate's known-wrong
  module-doc sketch against what the record says (AC-003).
- **`spec-and-code-reconciliation`** — reads every clause ADR-0022 discharges against
  the code as it then stands; the record is its input.

Transitively, the whole of the `durable-event-store` slice: `_storymap.md`'s merge
order states that nothing else may start until this merges.

## Anchors (progressive disclosure)

Load-bearing depth is deferred, not optional. Open each at the moment named — the
Context pack above already carries the decisions; these carry the evidence.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `crates/happenstance-sqlite/src/lib.rs` | `:47-53` settles the driver (`rusqlite`, no pool) — the half of the queue row that is stale on arrival; `:56-62` names exactly the three append-condition candidates and `:63-65` exactly the three tag-storage options. This is the list the record closes, and inventing a fourth arm is how the record stops answering the question it was queued for | First, before writing a line of the experiment crate — it defines the arms | AC-001, AC-004 |
| `crates/happenstance-sqlite/src/event_store.rs` | `:36-54` is the schema sketch that is *published and known wrong* — no type column on `event_tag`, so a type constraint becomes a join back to `event` under the write lock; `:26-32` and `:174-178` are the `NoRuntime` variant whose fate the runtime-seam paragraph decides; `:81` marks `PAGE_SIZE = 512` a placeholder that is not this story's to move | Before writing the schema block (AC-003) and again before the runtime-seam paragraph (AC-006) | AC-003, AC-006 |
| `RUNBOOK.md` | `:301` is the queue row this story discharges and marks Written; `:267` is the one-question rule; `:286` is the precedent for recording half a row as stale on arrival; `:4166-4237` is phase 8 in full; `:4178-4187` is the schema correction; `:4217-4222` and `:2686-2696` are the 64-versus-8 discrepancy and its two permitted outcomes | Open `:4166-4237` before drafting the record; open `:2686-2696` before writing the `CONTENDERS` paragraph | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` | The architecture brief's **§12** is the eight-subject checklist the record is read against; **§3** is the runtime seam with its recommended (a) and its rejected (b) spelled out; **§5** is the `index_arms()` rejection with its named re-open trigger; **§6** is the write path, `SQLITE_BUSY` and the pragmas; **§7** the schema; **§9** the tensions that must stay open; **§11** what is deliberately the implementer's | §12 as the drafting checklist; §3 and §5 while writing the consequences; §9 before writing the non-verdicts | AC-003, AC-006, AC-007 |
| `references/adr/0012-append-shape-and-preconditions.md` | `:244-266` restates ADR-0012's falsifier so a positive result cannot be manufactured — item 1 is *two builds of the same adapter differing only in `append`'s ownership*. Three candidate stores in an experiment crate are not that, which is exactly why ES-17 cannot be lifted here | Before writing the ES-17 non-verdict — quote item 1, do not paraphrase it | AC-007 |
| `.kb/open-questions/cf-40-fixture-limits-ownership.md` | The accepted open question that names phase 8 as what forces CF-40's clause home. This project needs the capability and gets no say in the clause's home; the atom is cited, never edited | While writing the second non-verdict | AC-007 |
| `experiments/position-visibility/README.md` | The precedent for every structural choice here: `:22-26` states this is not a crate, not a member, not a gate step and adds no dependency; `:29-45` is the conditions table; `:47-52` records that `setup.sh` *aborts* under `fsync=off` because what these mechanisms charge for is the interval held across a durable commit — the exact analogue of `synchronous` | Before laying out `experiments/append-condition/` and before writing its abort path | AC-001, AC-002 |
| `experiments/wire-format/Cargo.toml` | `:1-6` is the empty-`[workspace]` trick, with the comment explaining that it stops cargo walking up and adopting the crate as a member — the mechanism NF-002 depends on | When creating `experiments/append-condition/Cargo.toml` | AC-001 |
| `spec/SPECIFICATION.md` | `:7471-7492` is CF-14, which names `PRAGMA synchronous = OFF` **by name** in its `Rejects:` list. That is what makes AC-002 a correctness constraint and not merely an honesty one. The file is read here and edited by nobody in this story | Before choosing the `synchronous` value the record fixes | AC-002, AC-009 |
| `crates/happenstance-testkit/src/concurrency.rs` | `:24-43` is CF-33 — no watchdog anywhere, which is why an unbounded busy handler is a hung run naming no rule; `:200-206` is `CONTENDERS = 8` with its own doc saying it is not a tuning knob; `:214-231` is `Attempt::Rejected` versus `Attempt::Failed`, the distinction the chosen strategy has to make three stories later | Before the busy-timeout paragraph and before the contention run | AC-005, AC-006 |
| `crates/happenstance-testkit/src/contract.rs` | `Fixture` — one instance is one isolated store, each `connect()` one handle onto it — and `:44-54`'s limits-are-facts distinction. The candidate fixtures are real fixtures, so `event_store_conformance!` can be pointed at them unchanged | While writing the candidate fixtures | AC-001 |
| `crates/happenstance-core/src/query.rs` | `:204` is `Query::items()`, and the absence of `index_arms` anywhere in the file is the evidence for the rejection. The runbook's own work item names an API that does not exist, so a reader will otherwise think it was forgotten | While writing the `index_arms()` paragraph | AC-006 |
| `.kb/decisions/README.md` | States that an accepted decision atom is immutable and that measurements do not belong inside a decision — the two rules that produce AC-007's fencing and AC-008's separate evidence atom | Before staging anything under `.kb/_intake/` | AC-007, AC-008 |
| `.kb/reference/position-visibility-experiment-2026-08.md` | The precedent shape for the evidence atom: a `reference` atom whose `summary` carries the actual ratios and the controls that fired, `related:` to the decision it supports rather than swallowed by it | When drafting the staged evidence source | AC-008 |
| `.kb/_intake/README.md` | `:14-20` — the ingest glob and the approval gate; a successful run clears `_intake`, and the README itself is dropped at the gate rather than ingested | When staging, and again when writing the handoff note | AC-008 |
| `references/adr/0016-the-wire-format.md` | `:1-30` is the established head shape of a long record — Status / Date / Settles / Rests on and does not settle — the format the corpus reads and `spec-trace` cites into | Before drafting `references/adr/0022-append-condition-strategy.md` | AC-004, AC-006 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` | `:95-102` — the process-local ordinal plus `Drop` cleanup that owns a temporary path with **no new dependency**, which is how the candidate fixtures get their files without `tempfile` entering any manifest | While writing the candidate fixtures | AC-001, NF-002 |
| `.redkiln/config.yaml` | `:40`, `:48`, `:55` are the three `verify:` commands this story is measured by, and `:62-67` is `require_ledger: true`. They are wired, not typed fresh (testing brief AC-T02) | Before running the merge gate | AC-009 |

## Clarifications resolved during spec

1. **The nine AC ids are exactly the ones the front half decided** — AC-001 through
   AC-009, one per row of the *Behavior and interfaces* table, with the contention
   row bound to **AC-005** as that section states. None was added and none dropped;
   `_ledger.md` carries the same nine.
2. **Where the number comes from, given that `append` is `todo!()`.** Resolved in
   favour of a new out-of-workspace crate at `experiments/append-condition/` driven by
   `event_store_benchmarks!` verbatim, rather than (a) waiting for the adapter, which
   inverts AC-013, or (b) a private timing loop, which produces a figure the adapter
   can never re-derive. The precedent is `experiments/position-visibility/`, which
   measured the Postgres visibility question one phase before any Postgres adapter
   existed.
3. **Eight subjects versus one question.** §12's list and `RUNBOOK.md:267` are
   reconciled rather than traded off: one question, seven consequences that hang off
   it (AC-006), two non-verdicts fenced off from both (AC-007).
4. **The driver half of `RUNBOOK.md:301` is stale on arrival** and is recorded as
   ratified rather than decided, following the precedent at `RUNBOOK.md:286`.
5. **ES-17 is not lifted here, and the gap is escalated rather than absorbed.**
   ADR-0012 names phase 8 as its lifting measurement, but its falsifier item 1
   requires two builds of the *same* adapter; three candidate stores are not that. The
   record states that no story in this project's map currently owns producing item 1
   — a recorded gap for the ADR queue, not a silence.
6. **CF-40's clause home is not settled here.** The project needs the capability, not
   the clause; the open-question atom is cited and left untouched.
7. **The 64-contender figure is measured, not applied.** `concurrency::CONTENDERS`
   stays 8 in this diff; the raise, and the choice between AC-005's two permitted
   outcomes, belongs to `concurrency-family-and-contender-count`.
8. **The mount point is reached in two moves.** `.kb/decisions/0022-…` is minted by a
   human-invoked `/redkiln:kb-ingest` from staged `.kb/_intake/` sources; the handoff
   is a deliverable of this story (AC-008), and a hand-written atom in this diff is a
   defect regardless of its content.
9. **No surface, and no invented one.** `_design.md`'s signed-off `N/A` determination
   binds; the *Interaction quality* section enumerates both invariant families,
   marks each applicable or not with a reason, and carries every applicable one as an
   AC row rather than as a bullet.
