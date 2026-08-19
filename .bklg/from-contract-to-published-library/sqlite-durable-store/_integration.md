---
title: Integration audit — The first adapter that is not an instrument
initiative_slug: from-contract-to-published-library
project_slug: sqlite-durable-store
terminal: false
dod_green: true
reachability_ok: true
deferred_scenarios:
  - "DoD 1 — @smoke: cargo run -p course-subscriptions completes the canonical DCB cycle with no todo!() reached"
  - "DoD 2 — @smoke: the compile-fail case for an unhandled event variant is present and green"
  - "DoD 4 — the constrained-runtime store passes the suite on wasm32 under the edge runtime"
  - "DoD 5 — a store that does not serialise its writers passes the suite, visibility cost measured"
  - "DoD 6 — a store with no connection, no interactive transaction and no cursor passes the suite"
  - "DoD 7 — the projection suite discriminates: two structurally unlike batch shapes pass, CheckpointOnlyStore fails by name"
  - "DoD 8 — the written ProjectionStore freeze verdict"
  - "DoD 9 — @smoke: a stranger installs the published crate from the registry and runs a write-then-read cycle"
  - "DoD 10 — the published crate looks finished: rendered docs green under all features, registry page checked by looking"
  - "DoD 11 — the release is diffed against the prior published baseline, not asserted"
  - "DoD 12 — the clause ledger is audited at publish, no provisional clause with an empty falsifier"
  - "DoD 13 — cargo xtask ci (the whole gate) green on the exact tree that was published"
  - "DoD 14 — replication has an answer on disk: an accepted decision atom on ingest re-checking conditions"
  - "DoD 15 — incomplete logs have an answer on disk"
  - "DoD 16 — the audience is durable: persona and journey atoms under .kb/product/, linked by the closeout"
---

# Integration audit — The first adapter that is not an instrument

Non-terminal (feature) project. The bar read here is the one
[`.redkiln/config.yaml`](../../../.redkiln/config.yaml) wires to a non-terminal
project's integration grain — `integration_scoped: cargo xtask ci --fast` at
`:50-55` — plus a reachability audit over every capability the project's fourteen
stories delivered. The whole-initiative Definition of Done in
[`../initiative.md`](../initiative.md)`:354-407` belongs to the terminal project
(`closeout-and-durable-audience`, HS-P0019); its journeys are listed under
*Deferred to terminal project* below and are **not** counted against this project.

One exception, and it is the reason this project exists: **DoD 3 is this
project's own**, assigned to it by [`../_decomposition.md`](../_decomposition.md)
and restated at [`project.md`](project.md)`:66-67, :281-284`. It was executed
here, not deferred.

## Project integration bar

Every scenario below was **executed** on the merged branch
`initiative/from-contract-to-published-library` at `82584ff`, from a clean tree
(`git status --porcelain` empty before and after). Nothing is `fixme`, nothing is
`#[ignore]`d, nothing is gated off.

| # | Scenario this project owns | Command / mount | Result |
| --- | --- | --- | --- |
| 1 | **DoD 3a — `event_store_conformance!` green against a real `SqliteFixture`** | `crates/happenstance-sqlite/tests/conformance.rs:87` | **executed, PASS** — `90 passed; 0 failed; 0 ignored`. Includes `two_fixture_instances_observe_none_of_each_others_appends`, `two_handles_observe_each_others_appends`, `head_advances_across_two_handles`, `append_reports_exceeded_store_limits` and `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` |
| 2 | **DoD 3b — the concurrency case at 64 contenders** | `crates/happenstance-sqlite/tests/concurrency.rs:62`; `crates/happenstance-testkit/src/concurrency.rs:238` — `pub const CONTENDERS: usize = 64` | **executed, PASS** — `9 passed; 0 failed; 0 ignored` in 4.78s, including `exactly_one_of_n_contenders_commits` and `k_disjoint_boundaries_admit_exactly_k_commits` over 64 real `rusqlite::Connection`s on one file. The 8-versus-64 discrepancy was closed by **raising** the constant with a measured reason (`RUNBOOK.md:4278-4289`) — AC-005's first option, not the third one that rots |
| 3 | **DoD 3c — an acknowledged write survives a process reopen** | `dcb_conformance::acknowledged_writes_survive_a_reopen`, `::recorded_time_survives_a_reopen`, `::reopened_store_does_not_reissue_an_event_id`; fixture at `crates/happenstance-sqlite/tests/support/mod.rs:235-247` | **executed, PASS** — all three reopen rules green across a genuine close and WAL-`TRUNCATE` checkpoint of a real file that is never deleted or recreated |
| 4 | **The model family** | `crates/happenstance-sqlite/tests/conformance.rs:89` | **executed, PASS** — `dcb_model_conformance::ops_agree_with_the_model` green, with `proptest` enabled as a dev-dependency feature in `crates/happenstance-sqlite/Cargo.toml` so the family cannot silently not run |
| 5 | **The reopen negative control the rule has lacked since phase 4** | `RestampingFixture`, `crates/happenstance-testkit/tests/mutation_coverage.rs:1233-1258` | **executed, PASS** — declared to fail exactly `recorded_time_survives_a_reopen` with a pinned headline message, driven by `mutation_coverage::mutant_registry_is_exhaustive`, `::every_rule_has_a_mutant` and `::conformant_variants_pass_everything`, all green |
| 6 | **The probe-then-insert wrong implementation is rejected** | `RacingProbeStore`, `crates/happenstance-testkit/tests/mutation_coverage.rs:2598-2624` | **executed, PASS** — driven by `mutation_coverage::the_concurrency_rules_reject_exactly_what_they_claim`. Deviation from the story map recorded **on the row** at `:2588-2597`: the defect fails no *sequential* rule, so a `REGISTRY` row would be rejected by `mutant_registry_is_exhaustive`; it lives in `RACERS`, where a concurrency defect belongs |
| 7 | **The projection store passes the suite it did not write** | `crates/happenstance-sqlite/tests/projection.rs:226` | **executed, PASS** — `24 passed; 0 failed; 0 ignored` against the `projection_store_conformance!` that `projection-store-freeze` froze |
| 8 | **A wide query is chunked, not refused** | `crates/happenstance-sqlite/src/query_sql.rs:154` (`chunks`), consumed on the read path at `src/event_store.rs:648, :1313` and on the append-condition path at `:290` | **executed, PASS** — `crates/happenstance-sqlite/tests/wide_query.rs` `18 passed`, including the negative controls `a_wide_query_actually_crosses_the_chunk_boundary`, `the_union_is_every_chunk_not_the_first`, `a_wide_guard_answers_from_every_chunk_not_the_first` and `limit_applies_across_chunks_not_per_chunk` |
| 9 | **The append is atomic and the ceilings are facts** | `crates/happenstance-sqlite/tests/append.rs` | **executed, PASS** — `19 passed`, including `a_failure_mid_batch_leaves_nothing`, which installs a real `AFTER INSERT … RAISE(ABORT)` trigger through a second connection at `:862-886` and asserts `AppendError::Store` with every row rolled back |
| 10 | **The instrument markers are gone** | `crates/happenstance-sqlite/src/lib.rs:80` | **executed, PASS** — the scoped `#![allow(clippy::todo)]` is deleted and `crates/happenstance-sqlite/tests/front_page.rs` is `7 passed`, three of which were red on the base tree. The one surviving `unimplemented!` is `src/projection_store.rs:762` — `probe_read_through` behind `READS_THROUGH_BATCH = false` — named and permitted by `front_page::no_todo_synonym_stands_in_for_work_not_done` |
| 11 | **The benchmark family exists and is provably not conformance** | `crates/happenstance-testkit/src/bench.rs`, mounted at `crates/happenstance-testkit/tests/memory_benchmarks.rs:82, :88, :96` | **executed, PASS** — `18 passed` inside the gate's `--all-features` test step, and the conformance rule count is unchanged at **112** across four rule files (`cargo xtask lints`: *every stated rule count matches the suite*). See *Note 1* |
| 12 | **The specification and the code still agree** | `cargo xtask spec-trace`, inside `cargo xtask ci --fast` | **executed, PASS** — `201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE), 112 conformance rules, 58 e2e cases, 401 citations checked (80 anchored to their subject, 12 external)`; `traceability: no problems found`. Above the pre-slice baseline of 389 checked / 76 anchored recorded at `spec-and-code-reconciliation/_ledger.md:51` |
| 13 | **The non-terminal project bar** | `cargo xtask ci --fast` | **executed, PASS** — `all required checks passed (--fast: 4 optional step(s) not run)`. Covers fmt, clippy `-D warnings` over `--workspace --all-targets --all-features`, the whole test run, all four mandatory wasm32 steps, docs, `spec-trace`, the five file-reading lints, the `--no-default-features` doc build and the `cargo package --list` licence/README assertion |
| 14 | **Backlog and knowledge base clean** | `redkiln validate --kb && redkiln doctor` | **executed, SPLIT — `validate` PASS, `doctor` FAIL. Corrected 2026-08-18; see *Note 3*.** `redkiln validate --kb` → `validate passed`, exit **0**. `redkiln doctor` → exit **1**: the six expected `template-drift` advisories *and* **nine `unconsumed-foundation` errors** — `HS-S0002`, `HS-S0034`, `HS-S0035`, `HS-S0067`, `HS-S0074`, `HS-S0075`, `HS-S0100`, `HS-S0108`, `HS-S0120` |

**The whole gate, in its own words:**
`all required checks passed (--fast: 4 optional step(s) not run)`.

## Deferred to terminal project

Whole-initiative Definition-of-Done journeys from
[`../initiative.md`](../initiative.md)`:354-407` that this project does **not**
own. Each depends on a project that has not merged, and running it here would fail
by design. They are recorded for traceability and do not count against this
project's bar.

| DoD | Journey | Owner |
| --- | --- | --- |
| 1 | @smoke — `cargo run -p course-subscriptions` completes the DCB cycle with no `todo!()` reached | `typed-layer-and-alpha-release` (HS-P0011), asserted whole at `closeout-and-durable-audience` (HS-P0019) |
| 2 | @smoke — the compile-fail case for an unhandled event variant | `typed-layer-and-alpha-release` (HS-P0011), asserted whole at HS-P0019 |
| 4 | The constrained-runtime store passes the suite on `wasm32` under the edge runtime | `cloudflare-durable-object-store` (HS-P0013) |
| 5 | A store that does not serialise its writers passes, visibility cost measured | `postgres-and-neon-stores` (HS-P0014) |
| 6 | A store with no connection, no interactive transaction and no cursor passes | `postgres-and-neon-stores` (HS-P0014) |
| 7 | The projection suite discriminates — two unlike batch shapes pass, `CheckpointOnlyStore` fails by name | `projection-store-freeze` (HS-P0010) and `ladybug-projection-store` (HS-P0015). **This project contributes the SQL batch shape**, green at `crates/happenstance-sqlite/tests/projection.rs:226`, and per [`project.md`](project.md)`:346-350` must not resolve the second-shape question unilaterally — taking it here would invert the runbook's 6-before-8 order |
| 8 | The written `ProjectionStore` freeze verdict | `ladybug-projection-store` (HS-P0015) |
| 9 | @smoke — a stranger installs the published crate from the registry | `publication-and-positioning` (HS-P0016) |
| 10 | The published crate looks finished, checked by looking | `publication-and-positioning` (HS-P0016) |
| 11 | The release is diffed against the published baseline | `publication-and-positioning` (HS-P0016) |
| 12 | The clause ledger is audited at publish | `publication-and-positioning` (HS-P0016) |
| 13 | `cargo xtask ci` — the whole gate on the assembled, published tree | `closeout-and-durable-audience` (HS-P0019) |
| 14 | Replication has an answer on disk | `replication-identity-and-ingest` (HS-P0017) |
| 15 | Incomplete logs have an answer on disk | `retention-and-incomplete-logs` (HS-P0018) |
| 16 | The audience is durable — persona and journey atoms | `closeout-and-durable-audience` (HS-P0019) |

## Reachability map

"Reachable" here means the capability is compiled and **executed** by
`cargo xtask ci --fast` on this tree, through the composition root this repository
actually has — the crate's public module tree, plus the conformance suites that
`CLAUDE.md`'s *rule that matters* makes the bar for an adapter existing at all.
Nothing below is constructed-but-unmounted, exported-but-unconsumed, or reachable
only through a test the gate does not run.

| Capability (story) | Mount point | Reachable? |
| --- | --- | --- |
| `benchmark-harness` (HS-S0034) | `crates/happenstance-testkit/src/bench.rs`, declared as a feature- and target-gated `pub mod bench;` in `src/lib.rs`, mounted at `crates/happenstance-testkit/tests/memory_benchmarks.rs:82, :88, :96` | **Yes** — 18 tests executed in the gate. Gated `not(target_arch = "wasm32")`, so all four wasm32 steps stay green and DR-08 holds. See *Note 1* on the SQLite arm |
| `adr-0022-append-condition-strategy` (HS-S0035) | `.kb/decisions/0022-append-condition-strategy.md` (`kind: decision`, `status: accepted`) with the long record at `references/adr/0022-append-condition-strategy.md`; measured through `experiments/append-condition/tests/measure.rs:118-148` | **Yes** — the atom is read by `redkiln validate --kb` (passing) and cited from `crates/happenstance-sqlite/src/lib.rs:53-61` and `spec/SPECIFICATION.md`. It carries numbers rather than a preference: 23 and 213 microseconds for the `max(position)` guard against 32 and 311 for the `EXISTS` probe, 10.7 ms against 34.0 and 49.8 ms on tag storage, and `busy = 0` at 64 connections |
| `schema-migration-and-identity` (HS-S0036) | `crates/happenstance-sqlite/src/connection.rs:78` (`open_configured`, a real `Connection::open` at `:79`) behind `src/event_store.rs:357` (`SqliteEventStore::open`), reached by every `SqliteFixture::connect` | **Yes** — `crates/happenstance-sqlite/tests/migration.rs` 14 tests executed, including `store_id_survives_a_close_and_reopen`, `recorded_at_is_returned_as_stored_after_a_reopen` and `module_doc_schema_matches_sqlite_master` |
| `append-atomicity-and-store-limits` (HS-S0037) | `append` in `crates/happenstance-sqlite/src/event_store.rs`; the three ceilings mirrored onto the fixture at `tests/support/mod.rs:189-195` rather than restated, so a number cannot be declared at one value and enforced at another | **Yes** — `tests/append.rs` 19 tests, plus `dcb_conformance::append_is_atomic`, `::append_reports_exceeded_store_limits` and the VT-21 to VT-24 minimum-guarantee rules, all executed |
| `lazy-read-with-snapshot-ceiling` (HS-S0038) | `read` returning `SqliteReadStream` at `crates/happenstance-sqlite/src/event_store.rs:1190`, with the `spawn_blocking` deferred into `poll_next` | **Yes** — `tests/read.rs` 14 tests executed, and the stream is what the conformance read family and `wide_query::read_of_a_wide_query_executes_nothing_off_runtime` drive |
| `wide-query-chunked-not-refused` (HS-S0039) | `crates/happenstance-sqlite/src/query_sql.rs:154`, consumed at `event_store.rs:290, :648, :1313` — one entry point for both the read and the append-condition paths | **Yes** — `tests/wide_query.rs` 18 tests executed. A private module, but consumed by two public paths, so not exported-but-unconsumed |
| `sqlite-fixture-and-whole-suite` (HS-S0040) | `crates/happenstance-sqlite/tests/support/mod.rs:69`, mounted at `tests/conformance.rs:87` | **Yes** — one instance is one fresh temp file (`:87-106`), each `connect()` is a fresh `rusqlite::Connection` through `SqliteEventStore::open` (`:208-216`), and `reopen()` clears every handle then WAL-checkpoints `TRUNCATE` without deleting or recreating the file (`:235-247`). This is the second-handle and durability far end the instrument portfolio had empty |
| `concurrency-family-and-contender-count` (HS-S0041) | `crates/happenstance-sqlite/tests/concurrency.rs:62`; `CONTENDERS = 64` at `crates/happenstance-testkit/src/concurrency.rs:238` | **Yes** — 9 tests executed, and the runtime seam is proved rather than assumed by `a_store_with_no_runtime_anywhere_reports_no_runtime` and `store_serves_a_bare_thread_with_no_ambient_runtime`. The `NoRuntime` risk [`project.md`](project.md)`:330-339` named was answered in the brief, not discovered in a red run |
| `model-family-and-mutant-pass-column` (HS-S0042) | `crates/happenstance-sqlite/tests/conformance.rs:89`; `RacingProbeStore` at `crates/happenstance-testkit/tests/mutation_coverage.rs:2598` | **Yes** — both executed in the gate |
| `reopen-negative-control-and-durability-verdicts` (HS-S0043) | `RestampingFixture` at `crates/happenstance-testkit/tests/mutation_coverage.rs:1233`; the verdicts at `spec/SPECIFICATION.md:4213` for ES-35, `:7990-8012` for CF-17 and `:7874-7922` for CF-14 | **Yes** — the control is driven by the mutation-coverage harness in the gate, and the three clause verdicts are read by `spec-trace`'s falsifier-length check, which is green |
| `projection-store-passes-the-borrowed-suite` (HS-S0044) | `crates/happenstance-sqlite/src/projection_store.rs`, mounted at `tests/projection.rs:226` | **Yes** — 24 tests executed against the frozen suite, on an independently-migrated connection |
| `instrument-markers-removed-and-gate-green` (HS-S0045) | `crates/happenstance-sqlite/src/lib.rs:80` — the attribute is gone; guarded by `tests/front_page.rs` | **Yes** — 7 tests executed. Clippy `-D warnings` is green *because* the exception is gone rather than despite it, and the six scoped allows in the sibling skeleton crates are untouched |
| `crates-io-name-and-packaging-facts` (HS-S0046) | `crates/happenstance-sqlite/Cargo.toml` (description, `readme`), `crates/happenstance-sqlite/README.md`, `crates/happenstance-sqlite/LICENSE-MIT`, `crates/happenstance-sqlite/LICENSE-APACHE`; the generator at `xtask/src/reserve.rs` | **Yes**, and verified against the registry rather than against the ledger: `GET https://crates.io/api/v1/crates/happenstance-sqlite` returns `name: happenstance-sqlite`, `max_version: 0.0.0`, `created_at: 2026-08-18T13:16:06.844473Z`, `yanked: false`, with the manifest's exact description. `publish = false` and `xtask`'s `PUBLISHABLE` are deliberately untouched, and `cargo package --list` still reconciles exactly three crates. See *Note 2* |
| `spec-and-code-reconciliation` (HS-S0047) | `spec/SPECIFICATION.md`, with the pass recorded at [`spec-and-code-reconciliation/_reconciliation.md`](spec-and-code-reconciliation/_reconciliation.md) | **Yes** — `cargo xtask spec-trace` executes in the gate: 401 citations checked, 80 anchored, both above the 389/76 merge-base baseline, no citation deleted, and `git diff xtask/src/spec_trace.rs` empty, so no check was widened or relaxed to buy the green |

### Note 1 — the benchmark family's SQLite arm

Project AC-012 words itself as *"`event_store_benchmarks!` runs against the SQLite
fixture behind a `bench` feature"*. There is no permanent
`event_store_benchmarks!(SqliteFixture::new())` under
`crates/happenstance-sqlite/`, and that is a scope decision taken at planning
rather than a gap discovered here: [`_storymap.md`](_storymap.md)`:49` mounts the
family against `MemoryFixture` and `:50` assigns the real-SQLite measurement to
ADR-0022's experiment. The harness *does* run against real SQLite through real
`rusqlite` connections at `experiments/append-condition/tests/measure.rs:118-148`
— five invocations behind the `bench` feature — and its output is what
`experiments/append-condition/results/append-condition.md` and ADR-0022 quote.
`CLAUDE.md` puts experiments deliberately outside the gate, so this follows the
repository's own convention rather than evading it, and `measure.rs:4` already
names the later in-crate mount as a later addition. Recorded as a residual, not a
blocker: the capability is reachable and executed, and no acceptance criterion in
this project rests its evidence on a gate-resident SQLite benchmark mount.

### Note 2 — `publish = false` stays, knowingly

`RUNBOOK.md:4297`'s phase-8 exit checkbox reads *"`publish = false` removed"* and
it is knowingly **not** ticked. Project AC-015
([`project.md`](project.md)`:268-272`) and the initiative decomposition both say
the opposite and are the current authority: this project makes the crate
*publishable*, and `publication-and-positioning` (HS-P0016) decides that it
publishes. Deleting the line here would promote a crate that `xtask`'s
`PUBLISHABLE` does not name, which is exactly the failure `xtask/src/package.rs`
was written to produce. The supersession is stated at
`crates/happenstance-sqlite/Cargo.toml:16-21` and in
[`crates-io-name-and-packaging-facts/_ledger.md`](crates-io-name-and-packaging-facts/_ledger.md),
so a closeout audit reading phase 8 finds a decision rather than a silence.

### Note 3 — row 14 was wrong, and this is the correction

**As first written, row 14 claimed `doctor` exited 0 "carrying exactly the six expected
`template-drift` advisories … and no seventh". That was false.** The orchestrator ran both halves
against this tree on 2026-08-18, before advancing anything:

```console
$ redkiln validate --kb
redkiln: validate passed.                     # exit 0

$ redkiln doctor
… six template-drift warnings …
… nine "foundation story 'HS-S####' is consumed by no capability slice" errors …
                                              # exit 1
```

The wrong sentence is quoted above rather than deleted, because a proof artifact that silently
acquires a different claim is worth less than one that shows what it got wrong. This is the same
defect class the project's own premise names — something that looks like evidence and is not — landing
in the project's own integration proof, and it was caught by re-running the command rather than by
reading the report.

**What is actually true, and why `dod_green` is retained rather than flipped.**

- The **declared** project-scoped bar is `verify.integration_scoped` in
  [`.redkiln/config.yaml`](../../../.redkiln/config.yaml) — `cargo xtask ci --fast` — and that is
  **row 13**. The orchestrator re-ran it independently at the same commit: exit **0**,
  `all required checks passed (--fast: 4 optional step(s) not run)`. Row 13 is first-hand, not
  relayed.
- `redkiln doctor` is **not** one of this repo's declared `verify:` commands (the four are
  `affected_gate`, `reachability_static`, `integration_scoped`, `e2e`). Row 14 is an extra check
  this run chose to make. Its failure is real and is recorded as a failure — it does not retroactively
  redden the bar the project is actually held to.

**The nine errors are pre-existing and are not this project's code.** They are redkiln
**[#122](https://github.com/Wet-Ink-Corporation/redkiln/issues/122)** — `diagnoseUnconsumedFoundations`
answers a reachability question with a single-hop predicate, so a foundation story feeding capability
work through one intermediate foundation story is indistinguishable from one feeding nothing; seven of
the nine are false positives by transitive reachability. Verified pre-existing three separate times:
by the 2026-08-15 wave (stash-and-reset to `HEAD`, and a second worktree at the same commit), and by
this initiative's orchestrator via `redkiln doctor --cwd` against a tree carrying none of the
intervening changes.

**Two of the nine are this project's own stories** — `HS-S0034` `benchmark-harness` and `HS-S0035`
`adr-0022-append-condition-strategy`. And **CI's `backlog` job asserts the list is empty**
(`.github/workflows/ci.yml:177`), which makes this a **release blocker for the initiative**, owed
before `publication-and-positioning` (HS-P0016) rather than at the PR. It is disclosed here so the
project's review gate is answered with it in view rather than around it.

## Missing dependencies

**None.** No scenario this project owns needed a sibling or a state contract that
does not exist. The two dependencies the decomposition marked as blocking —
`projection-store-freeze` (HS-P0010) for the projection port, its `Batch` shape
and the projection suite, and `typed-layer-and-alpha-release` (HS-P0011) as the
consumer that discovers contract defects before the flagship adapter is written —
both merged on this branch before this project's stories ran, and both are
consumed for real: `crates/happenstance-sqlite/tests/projection.rs:226` runs the
borrowed suite unmodified, and no clause either project froze was amended here.

Two things are **open by design and recorded rather than settled in passing**,
both of which [`project.md`](project.md) forbids this project to resolve:

- **CF-40's clause home.** `.kb/open-questions/cf-40-fixture-limits-ownership.md`
  is still `kind: open_question`. This project needed the *capability* to declare
  numeric ceilings, and used it; it did not pick the clause's owner.
- **ES-35's fault far end.** The marker at `spec/SPECIFICATION.md:4213` is
  restated as `[PROVISIONAL]` with a live, **narrowed** falsifier: a store that
  loses a write to a *fault* rather than to an instruction. `SqliteFixture`
  declines `MID_BATCH_FAULT` **by scope, not by incapacity**, and says so in its
  own words at `crates/happenstance-sqlite/tests/support/mod.rs:173-184` — the
  trigger-based injection is demonstrably available to this adapter and is
  exercised by `tests/append.rs::a_failure_mid_batch_leaves_nothing`, which
  installs an `AFTER INSERT ... RAISE(ABORT)` trigger through a second connection
  and watches the append fail unabsorbed. A declined capability whose rule still
  appears in the run, reporting the fixture's stated reason, is the CF-18
  mechanism working as designed, not a scenario skipped.

## Advisory — not blocking, and owned elsewhere

Recorded so the next run does not rediscover them.

- **Three stories carry stale system frontmatter.** `HS-S0045`, `HS-S0046` and
  `HS-S0047` read `status: ready` / `stage: plan` while their work is implemented,
  reviewed, ledgered and committed (`62a05dd`, `ada4962`, `54df29a`) and their
  slice was sealed approved at `82584ff`. The CLI is the only writer of those
  fields, so this audit does not touch them; the orchestrating command owns the
  transition.
- **`RUNBOOK.md`'s phase-8 status row still reads `not started`** (`:159`) and its
  four exit checkboxes are unticked (`:4293-4297`). Phases 6 and 7 read the same
  way despite having merged on this branch, so this is an initiative-wide pattern
  — the runbook's phase rows are read and updated at closeout by their owner —
  rather than a defect this project introduced. Three of the four criteria are
  observably met on this tree; the fourth is *Note 2*.
- **Two conformance rules are claimed by no clause and owe an ADR**
  (`k_disjoint_boundaries_admit_exactly_k_commits` and `ops_agree_with_the_model`).
  `spec-trace` reports both and stays green, which is its designed behaviour;
  neither was introduced by this project and both are the ADR queue's.

## Integration verdict

**GREEN.** This project's integration bar is met.

- **Reachability: OK.** All fourteen delivered capabilities are mounted into the
  real composition root — the crate's public module tree and the four conformance
  suites the repository's *rule that matters* recognises — and every one of them is
  **executed** by `cargo xtask ci --fast` on this tree. Nothing is
  constructed-but-unmounted or exported-but-unconsumed. The one capability whose
  gate-resident mount is narrower than its criterion's wording — the benchmark
  family's SQLite arm, *Note 1* — is still reachable and executed, in the place
  this repository's own convention puts it.
- **Affected gate: green.** `cargo xtask ci --fast`, the non-terminal bar at
  `.redkiln/config.yaml:50-55` and a strict superset of a package-filtered run over
  `happenstance-sqlite`, `happenstance-testkit` and `xtask`, passed whole: zero
  failed and zero ignored tests across every sqlite and testkit target, and
  `all required checks passed (--fast: 4 optional step(s) not run)`.
- **DoD 3 — this project's own initiative-level journey — was executed and
  passed.** The conformance suite is green against a real file, the concurrency
  family is green at 64 contenders, and an acknowledged write survives a genuine
  reopen through two real handles onto one file. `happenstance-sqlite` is no
  longer an instrument.
- **Fifteen whole-initiative journeys are correctly deferred** to the projects
  that own them, and none of them is a dependency this project should have
  supplied.

Verified on `initiative/from-contract-to-published-library` at `82584ff`, clean
tree, 2026-08-18.
