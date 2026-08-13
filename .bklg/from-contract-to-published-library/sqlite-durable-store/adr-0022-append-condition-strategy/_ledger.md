---
item: HS-S0035
stage: implement
created: 2026-08-12T13:46:34.058Z
updated: 2026-08-12T13:46:34.058Z
---

# Acceptance ledger — ADR-0022, written first and carrying a measured number

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN the adapter author must pick an append-condition SQL shape while `SqliteEventStore::append` is still `todo!()`, so there is nothing in the workspace to measure, WHEN they ask where the number came from, THEN all three candidates named at `crates/happenstance-sqlite/src/lib.rs:56-62` — `BEGIN IMMEDIATE` + `EXISTS` probe, conditional `INSERT … SELECT … WHERE NOT EXISTS`, monotonic-position guard — exist as real `EventStore` implementations over `rusqlite` in `experiments/append-condition/`, are driven by `event_store_benchmarks!` verbatim rather than by a bespoke timing loop, and each one passes `event_store_conformance!` before its figure is allowed to count — so no arm wins by being fast and wrong"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — minted by the /redkiln:kb-ingest handoff from .kb/_intake/0022-append-condition-strategy.md; the figures it quotes originate in experiments/append-condition/results/"
  verifying_test: "experiments/append-condition/tests/candidates_are_conformant.rs (event_store_conformance! green against each of the three candidate fixtures) plus experiments/append-condition/tests/measure.rs, run by cargo test --manifest-path experiments/append-condition/Cargo.toml"

- id: AC-002
  criterion: "GIVEN the specification names `PRAGMA synchronous = OFF` by name as a wrong implementation CF-14's reopen rule exists to reject (`spec/SPECIFICATION.md:7481-7484`), so a figure produced under it is a figure for a store that cannot ship, WHEN the runner starts, THEN it reads back `journal_mode` and `synchronous` from the live connection and aborts rather than emit a number under a setting the shipped adapter may not use, and every figure in the record is printed beside the settings, machine and SQLite version it was produced on"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the conditions block travelling with every quoted figure, staged at .kb/_intake/0022-append-condition-strategy.md"
  verifying_test: "experiments/append-condition/tests/durability_settings_are_enforced.rs — the positive control that forces synchronous = OFF and asserts the runner refuses; cross-read against experiments/append-condition/README.md and experiments/append-condition/results/"

- id: AC-003
  criterion: "GIVEN `schema-migration-and-identity` is the very next story and must write migration 1 without re-deriving it — and the crate's own published sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` is known to be wrong in a way that serialises every writer, WHEN its implementer opens the record, THEN all three tag-storage options (`lib.rs:63-65`) were measured under a probe held under the `BEGIN IMMEDIATE` write lock — the canonical blob arm measured rather than dismissed, since `Tags` is sorted precisely to keep it open — and the record specifies migration 1 outright: `event_tag(tag, position)` `WITHOUT ROWID` with `event_type` as a covering column and the key left `(tag, position)`, `tag_cardinality`, the `EventId` origin pair `UNIQUE` together, `recorded_at` read back not re-stamped, `AUTOINCREMENT`, and a `StoreId` minted once at schema creation — naming what the sketch got wrong and why"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the schema block `schema-migration-and-identity` (HS-S0036) reads migration 1 out of; long form at references/adr/0022-append-condition-strategy.md"
  verifying_test: "experiments/append-condition/results/tag-storage.md (a figure per arm under identical conditions) plus experiments/append-condition/tests/candidates_are_conformant.rs; review check that crates/happenstance-sqlite/src/event_store.rs is absent from git diff --name-only"

- id: AC-004
  criterion: "GIVEN project AC-013 refuses a preference and the architecture brief already recommends `BEGIN IMMEDIATE` + probe (§6), WHEN the adapter author reads the decision paragraph, THEN exactly one strategy is stated as chosen, the other two are named as lost with the measured figure and its conditions attached, and the driver half of the queue row (`RUNBOOK.md:301`) is recorded as ratified rather than decided — stale on arrival, because `rusqlite` without a pool is already settled at `crates/happenstance-sqlite/src/lib.rs:47-53` — so the record answers one question the way `RUNBOOK.md:267` demands"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the atom every downstream story loads, indexed in .kb/maps/decision-map.md by the ingest run"
  verifying_test: "Review tier (testing brief §2, AC-013's row): references/adr/0022-append-condition-strategy.md read against experiments/append-condition/results/append-condition.md — the quoted figure must be the same figure, reproducible by experiments/append-condition/run.sh; RUNBOOK.md:301 struck through and marked Written"

- id: AC-005
  criterion: "GIVEN both of phase 8's stated proof artefacts read 64 contenders while `concurrency::CONTENDERS` is 8 and its own doc says it is not a tuning knob (`crates/happenstance-testkit/src/concurrency.rs:200-206`), WHEN `concurrency-family-and-contender-count` later moves that constant, THEN it inherits a measured claim rather than an assumed one: the contended workload was run at 64 connections on one file, and the record states what 64 cost — file descriptors, busy contention, the committed/rejected mix and wall time — and whether the raise is supportable; and this story does not itself touch `concurrency.rs`"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the CONTENDERS paragraph concurrency-family-and-contender-count reads before raising the constant"
  verifying_test: "experiments/append-condition/results/contention-64.md (event_store_benchmarks! scenario (b) at k = 64 with its ConditionViolated split) plus git diff --name-only showing crates/happenstance-testkit/** untouched"

- id: AC-006
  criterion: "GIVEN the architecture brief's §12 lists eight subjects while the queue's rule is that a record answers one question, WHEN the adapter author reads the record end to end, THEN the seven consequences hang visibly off the one question and each carries the alternative that lost: the runtime seam as a captured `tokio::runtime::Handle` with `try_current()` as fallback, against running inline on the calling thread — with the fate of `SqliteEventStoreError::NoRuntime` (`crates/happenstance-sqlite/src/event_store.rs:26-32`) spelled out under whichever wins; `index_arms()` rejected, because it does not exist in `happenstance-core` (`crates/happenstance-core/src/query.rs:204` has `items()` and no `index_arms`), the decomposition stays adapter-private, and the re-open trigger is named (`postgres-and-neon-stores` independently needing it); and three pragma values — a finite and generous busy timeout, the journal mode, the `synchronous` setting"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the consequences the durable-event-store and race-model-and-durability slices implement; long form at references/adr/0022-append-condition-strategy.md"
  verifying_test: "Review checklist against references/adr/0022-append-condition-strategy.md (one Settles: line, seven consequence sections each naming a rejected alternative, three numeric pragma values) plus rg -n \"index_arms\" crates/happenstance-core/src returning nothing and rg -n \"NoRuntime\" references/adr/0022-append-condition-strategy.md finding the deciding paragraph"

- id: AC-007
  criterion: "GIVEN an accepted decision atom is immutable (`.kb/decisions/README.md`), so a paragraph written on evidence this story cannot produce can only ever be superseded, WHEN the record reaches the two subjects it must not settle, THEN each is recorded as an explicit non-verdict with a named owner: ES-17 / `&[Event]` vs `Vec<Event>` is not lifted, with ADR-0012's falsifier item 1 restated verbatim (two builds of the same adapter, `references/adr/0012-append-shape-and-preconditions.md:244-266`), what the experiment did observe about multi-row insert copy cost, and the statement that no story in this project's map currently owns producing it — escalated to the ADR queue; and CF-40's clause home is recorded as still open, citing `.kb/open-questions/cf-40-fixture-limits-ownership.md`, which this story does not edit"
  satisfied: false
  evidence: ""
  mount_point: ".kb/decisions/0022-append-condition-strategy.md — the fenced non-verdict section, with the escalation row in RUNBOOK.md's ADR queue"
  verifying_test: "rg -n \"0012|falsifier|Vec<Event>\" references/adr/0022-append-condition-strategy.md finding the restated falsifier and the not-lifted verdict; git diff --name-only showing .kb/open-questions/cf-40-fixture-limits-ownership.md unmodified; review of RUNBOOK.md's queue escalation row"

- id: AC-008
  criterion: "GIVEN `CLAUDE.md` forbids hand-writing `.kb/` atoms — the first attempt at that was reverted at `0269720` — and the mount point every downstream story loads is `.kb/decisions/0022-append-condition-strategy.md`, WHEN this story finishes, THEN the atom's source and a separate evidence source are staged under `.kb/_intake/` (the decision cites a `reference` atom rather than swallowing the measurement, so the decision can be superseded without invalidating the numbers — `.kb/reference/position-visibility-experiment-2026-08.md` is the precedent shape), the human-invoked `/redkiln:kb-ingest` handoff is recorded as a named deliverable, and no `.kb/decisions/**` or `.kb/maps/**` file appears in this diff"
  satisfied: false
  evidence: ""
  mount_point: ".kb/_intake/0022-append-condition-strategy.md plus the staged evidence source — the two files /redkiln:kb-ingest mints .kb/decisions/0022-append-condition-strategy.md and its .kb/maps/decision-map.md row from"
  verifying_test: "redkiln validate --kb && redkiln doctor clean with exactly the six expected template-drift advisories; git diff --name-only carrying no path under .kb/decisions/ or .kb/maps/ and exactly two new paths under .kb/_intake/ (not .kb/_intake/README.md, per .kb/_intake/README.md:14-20)"

- id: AC-009
  criterion: "GIVEN AC-013's entire content is that the record precedes the implementation, so a PR that does both has destroyed the ordering it was written to prove, WHEN the diff is read, THEN not one file under `crates/happenstance-sqlite/src/` has moved, `#![allow(clippy::todo)]` is still there, no clause in `spec/SPECIFICATION.md` and no marker has changed, the conformance rule count is exactly where `benchmark-harness` left it, and the gate is green on a tree whose only compiled change is outside the workspace"
  satisfied: false
  evidence: ""
  mount_point: "The PR boundary itself — experiments/append-condition/**, references/adr/0022-append-condition-strategy.md, .kb/_intake/**, RUNBOOK.md and this story's backlog folder, and nothing under crates/** or spec/**"
  verifying_test: "cargo xtask affected --base {{base}} (affected_gate, .redkiln/config.yaml:40) and cargo xtask ci --fast (integration_scoped, :55), both green; the rule-name count from for_each_event_store_rule! (crates/happenstance-testkit/src/registry.rs:94) identical either side of the diff"
```
