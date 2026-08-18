---
item: HS-P0012
stage: review
title: "Review — The first adapter that is not an instrument"
initiative_slug: from-contract-to-published-library
project_slug: sqlite-durable-store
terminal: false
created: 2026-08-18
updated: 2026-08-18
overall: 3
dod_green: true
rubric:
  ac-coverage: 2
  integration-reachability: 3
  test-integrity: 3
  gate-greenness: 3
  brief-fidelity: 3
  intent-fidelity: 3
  presentation-fidelity: 0
---

# Review — The first adapter that is not an instrument

- [x] Every project acceptance criterion (AC-001 – AC-016) is met by real, reachable, committed behaviour — with one recorded residual on AC-012's SQLite arm
- [x] Every delivered capability is mounted into a real consumer the gate reaches — nothing constructed-but-unmounted
- [x] No test gutted, skipped, `#[ignore]`d, flag-gated off, or replaced by a double
- [x] The affected-package gate is genuinely green, formatter included — re-run by this review at HEAD, not taken on trust
- [x] This project's applicable Definition-of-Done bar is green (`_integration.md`, `dod_green: true`, `reachability_ok: true`)
- [x] Fifteen whole-initiative journeys are correctly deferred to the projects that own them
- [ ] Presentation reviewed — **DOES NOT APPLY and was NEVER OBSERVED**; see the Rubric note

## Verdict

**approved.**

I tried to break this and could not. The two claims the project exists to retire
— that a `todo!()` body proves nothing, and that no fixture in the workspace has
ever opened a *second connection* or survived a *real reopen* — are retired by
mechanism rather than by prose. The fixture is where three criteria are won or
lost, and it is honest: `SqliteFixture::new` mints a fresh temporary file per
instance and unlinks stale sidecars
(`crates/happenstance-sqlite/tests/support/mod.rs:87-106`); `connect()` calls
`SqliteEventStore::open`, which reaches a real `rusqlite::Connection::open` at
`crates/happenstance-sqlite/src/connection.rs:79`, and keeps the returned handle
only so `reopen()` has something to close; and `reopen()` drops every handle and
WAL-checkpoints `TRUNCATE` **without deleting or recreating the file**, which is
the single line that stops `acknowledged_writes_survive_a_reopen` from being
vacuous (`tests/support/mod.rs:235-247`).

The two things most likely to be escape hatches are not. The lone
`unimplemented!` (`crates/happenstance-sqlite/src/projection_store.rs:762`) sits
behind `READS_THROUGH_BATCH = false`, a contract constant that **predates this
project** (present at merge-base `90cbca5` in
`crates/happenstance-core/src/projection.rs:583`) and whose declension path the
borrowed suite already carried
(`crates/happenstance-testkit/src/contract.rs:836`); it is narrowly allowlisted
by path *and* enclosing function name in `tests/front_page.rs`, so it cannot
spread. And `MID_BATCH_FAULT` is declined **by scope, not by incapacity**, in the
adapter's own words — an earlier spelling claimed the store had no way to fail
mid-batch, that sentence was false, and commit `9a10dbf` replaced it with one
that names the mechanism the adapter demonstrably has. Both fault rules are still
**in the binary**, which I verified mechanically rather than by reading: a
`--list` pass over the conformance target emits
`dcb_conformance::append_is_atomic_under_a_mid_batch_fault` and
`::arming_a_mid_batch_fault_makes_the_append_fail`. That is CF-18 working, not a
scenario skipped.

One residual, recorded and not blocking: AC-012's literal wording asks for
`event_store_benchmarks!` against **the SQLite fixture**, and no such invocation
exists under `crates/happenstance-sqlite/`. See *Residual* under Evidence.

## Rubric

Each dimension scored 0 (absent) to 3 (excellent). `approved` requires every dimension >= 2, with
`gate-greenness` = 3, `integration-reachability` = 3, `intent-fidelity` >= 2, and the applicable
Definition-of-Done bar green.

`presentation-fidelity` is the single exception to "every dimension >= 2", and the exemption is
narrow. Here the design review **DOES NOT APPLY**: `_design.md`'s fenced `yaml` surfaces block
(lines 37-38) is empty and every section reads "N/A — no user-facing surface", signed off
2026-08-12; and `.redkiln/config.yaml` declares no `design:` block at all, so `design.capture` is
absent by deliberate design — *"there is no app to screenshot"* (`CLAUDE.md`). It scores 0 and is
**exempt from the bar, never from the record**.

| Dimension | Score | Rationale |
| --------- | ----- | --------- |
| ac-coverage | 2 | Fifteen of sixteen project ACs are met by real, reachable, committed behaviour, each traced in the map below and re-verified against the code rather than the report. AC-012 is met in substance and **partially in letter**: the harness exists, is feature- and target-gated, is mounted, and executes 18 tests inside the gate (`crates/happenstance-testkit/tests/memory_benchmarks.rs`), and its "provably not conformance" half is *mechanically* checked — `cargo xtask lints` reports 112 rules across four rule files, unchanged, and `bench.rs` is not a rule file. But no `event_store_benchmarks!(SqliteFixture::new())` exists under `crates/happenstance-sqlite/`; the real-SQLite arm lives at `experiments/append-condition/tests/measure.rs:118-148` (five invocations over real `rusqlite` connections), and `experiments/*` is not a workspace member (`Cargo.toml:3`), so that arm sits outside the gate by the repository's own convention. Pre-planned at `_storymap.md:49-50`, not discovered late. Docked one point for the gap between the criterion's words and its mount. |
| integration-reachability | 3 | All fourteen delivered capabilities are mounted into the composition root this repository actually has — the crate's public module tree plus the four suites `CLAUDE.md`'s *rule that matters* makes the bar for an adapter existing at all — and every one is **executed**, which I confirmed by running them: conformance 90 passed, projection 24, append 19, wide_query 18, migration 14, read 14, concurrency 9, front_page 7, all with `0 failed`. Nothing is constructed-but-unmounted or exported-but-unconsumed: `query_sql::chunks` is private but consumed by *two* public paths (`event_store.rs:290` on the append-condition guard path and `:648, :1313` on the read path), and the guard path was wired by repair `9a10dbf` after review caught that only `read` chunked. `_integration.md` reports `dod_green: true` and `reachability_ok: true` with zero fixme, zero `#[ignore]`, nothing gated off. The fifteen deferred journeys are whole-initiative DoD items belonging to named sibling projects; DoD 3, which *is* this project's own, was executed here. |
| test-integrity | 3 | Nothing was gutted. The cumulative diff removes **no** test function and **no** assertion from `crates/happenstance-testkit/` or `crates/happenstance-core/`; the contract crate is untouched entirely, and the borrowed `crates/happenstance-testkit/src/projection.rs` is absent from the changed-file set, so the suite this project had to pass was not edited to pass it. The one modified test outside the affected set (`crates/happenstance/tests/projection_clauses.rs`) is **strengthened**: it stopped pinning a staged `.kb/_intake/` file that ingest was always going to consume, and now requires the accepted atom plus a `status: accepted` read-back — I verified `.kb/decisions/0031-the-runner-collapses-upward.md` exists at `status: accepted`. Two genuine negative controls were added where the runbook said falsifiers were missing, both carrying **pinned headline messages** so a failure at the wrong anchor is reported as the wrong failure instead of counted as a pass: `RestampingFixture` (`mutation_coverage.rs:1233-1258`, which must fail exactly `recorded_time_survives_a_reopen` and pass the two survival rules beside it, or it is `LosingFixture` renamed) and `RacingProbeStore` (`:2598-2624`). The only `ignored` tests in the scoped run are pre-existing `ignore`-annotated doctests in `crates/happenstance-testkit/src/lib.rs`; no `#[ignore]`, `cfg(false)` or `todo!()` was added anywhere under `crates/`. |
| gate-greenness | 3 | Re-run by this review at HEAD on a clean tree (porcelain status empty), affected-scoped, never the unfiltered whole-repo script. Formatter check: exit 0. Scoped tests over the three affected packages with `--all-features`: exit 0, **zero failed** across every target. Scoped clippy over `--all-targets --all-features` with `-D warnings`: exit 0, which is meaningful precisely because the scoped `#![allow(clippy::todo)]` is gone rather than still absorbing it. `cargo xtask lints`: green, every stated rule count matching the suite. `cargo xtask spec-trace`: green, `traceability: no problems found`, `401 citations checked (80 anchored to their subject, 12 external)`, reproducing `_integration.md`'s figures exactly. `cargo xtask affected --base main` — the project's own configured runner at `.redkiln/config.yaml:40` — printed **`affected gate passed`**, exit 0. The formatter is green and was not hand-coaxed. |
| brief-fidelity | 3 | No Accepted KB decision is deviated from. `CLAUDE.md` constraint 3 (ADR-0001, ADR-0008) holds: `read` returns `SqliteReadStream` at the top level and is not `async`, with `spawn_blocking` deferred into `poll_next` (`event_store.rs:1190`). DR-08 holds exactly as the brief demanded — `pub mod bench` carries `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]`, and the comment at `crates/happenstance-testkit/src/lib.rs:311-320` shows the author reasoning about the one gate step that would have caught the omission rather than copying the pattern blindly. ADR-0022 was committed at `791b929`, **before** the first implementation story `8381c89`, as an accepted atom (`.kb/decisions/0022-append-condition-strategy.md`, `kind: decision`, `status: accepted`) with the 619-line record beside it. The known-wrong schema sketch was corrected rather than implemented. Out-of-scope was respected under schedule pressure: CF-40's clause home is still `kind: open_question`, DoD 7's second batch shape was left to its owner, `publish = false` and `xtask`'s three-crate `PUBLISHABLE` set are deliberately untouched, and nothing `[FROZEN]` was amended. |
| intent-fidelity | 3 | The UX/interaction lens does not apply — this project renders no surface (see the exemption note). The design-intent lens is what binds, and the diff reflects what the anchors *decide*, not merely that a test is green. Three places where the letter could have been satisfied cheaply and was not: (1) AC-005's contender count was closed by **raising** `CONTENDERS` to 64 with a measured reason and a stated workspace-wide cost (`crates/happenstance-testkit/src/concurrency.rs:199-238`), the runbook's first option rather than "the third option and it is the one that rots"; (2) the fixture's ceilings are **mirrored from the adapter's own constants** (`tests/support/mod.rs:189-195`) rather than restated, so a number cannot be declared at one value and enforced at another — the fixture-pinned-assertion failure mode, closed by construction; (3) `RacingProbeStore` was deliberately placed in `RACERS` rather than `REGISTRY` because a probe-then-insert defect fails no *sequential* rule and `mutant_registry_is_exhaustive` would reject the row, and the deviation from `_storymap.md` is recorded **on the row** at `:2588-2597` where the next reader meets it. The `spec_trace.rs` `BARE_NAME_MAP` addition is a disambiguation forced by a new file, landed in `23bc776` — the very commit that created `tests/append.rs` — not a late relaxation bought to turn a check green. |
| presentation-fidelity | 0 | **Presentation was NEVER OBSERVED.** No perceptual evidence exists, and none could: this project declares no user-facing surface (`_design.md` lines 10-38, where the fenced `yaml` surfaces block is empty and every section reads "N/A"), no `_design-review.md` was produced or owed, and the repository declares no `design:` block in `.redkiln/config.yaml`, which makes `design.capture` a **declared skip** rather than a silent pass. Scored 0 as absence of evidence, not evidence of absence. **DOES NOT APPLY**, therefore exempt from the >= 2 bar — exempt from the bar, not from the record. |

`intent-fidelity` scores BEHAVIOR from the diff; `presentation-fidelity` scores FORM from the
perceptual evidence in `_design-review.md`. No such evidence exists here and none was owed.

## Evidence

### Project AC coverage map

| AC | Met by reachable behaviour? | Evidence |
| --- | --- | --- |
| AC-001 — the suite runs, whole | **Yes** | `crates/happenstance-sqlite/tests/conformance.rs:87`; re-run gives `90 passed; 0 failed; 0 ignored`. Rule presence verified mechanically via a `--list` pass rather than asserted: the two declined-capability rules `dcb_conformance::append_is_atomic_under_a_mid_batch_fault` and `::arming_a_mid_batch_fault_makes_the_append_fail` are **in the binary**, reporting the fixture's stated reason |
| AC-002 — two instances share nothing | **Yes** | `tests/support/mod.rs:87-106` — process id plus atomic ordinal in the temp path per instance, stale sidecars unlinked; `two_fixture_instances_observe_none_of_each_others_appends` green among the 90 |
| AC-003 — the second handle is a second connection | **Yes** | Observable in the fixture's body, which is what the criterion demands: `connect()` calls `SqliteEventStore::open(&self.path)` (`tests/support/mod.rs:208-216`), reaching `open_configured` and a real `rusqlite::Connection::open` (`src/connection.rs:79`). Not a `Clone` of one store; the clone that *is* retained is of the newly-opened handle, so `reopen` has something to close |
| AC-004 — an acknowledged write survives a reopen | **Yes** | `REOPEN = Capability::SUPPORTED`; `reopen()` clears every handle then WAL-checkpoints `TRUNCATE`, and the **file is never deleted or recreated** (`tests/support/mod.rs:235-247`). The negative control the rule has lacked since phase 4 now exists: `RestampingFixture`, `crates/happenstance-testkit/tests/mutation_coverage.rs:1233-1258`, one rule, pinned headline |
| AC-005 — the race is real and its size is a decision | **Yes** | `CONTENDERS = 64` at `crates/happenstance-testkit/src/concurrency.rs:238`, raised with a measured reason and a stated cost at `:199-237`; `tests/concurrency.rs` re-run gives `9 passed; 0 failed` in 3.72s. The runbook's first option, not the third one that rots |
| AC-006 — model family and mutant pass column | **Yes**, with a recorded deviation | `event_store_model_conformance!` at `tests/conformance.rs:89`, green. Registering `SqliteEventStore` inside the testkit's own `conformant_variants_pass_everything` was **refused** for two mechanical reasons — a published crate would dev-depend on its own consumer, and `rusqlite` plus a tokio runtime would be dragged into a harness deliberately built to need neither — and the refusal, together with where the obligation is honestly discharged, is recorded in `tests/conformance.rs`'s module doc where a reviewer holding the runbook meets it |
| AC-007 — append atomic, probe-then-insert rejected | **Yes** | One `BEGIN IMMEDIATE` per append (`src/event_store.rs:455, :513, :549`); `tests/append.rs` `19 passed`, including `a_failure_mid_batch_leaves_nothing`, which installs a real `AFTER INSERT ... RAISE(ABORT)` trigger through a second connection. The named wrong implementation is rejected by name: `RacingProbeStore` fails exactly `exactly_one_of_n_contenders_commits` and `k_disjoint_boundaries_admit_exactly_k_commits` with pinned messages (`mutation_coverage.rs:2598-2624`) |
| AC-008 — a wide query is chunked, not refused | **Yes** | `src/query_sql.rs:154` (`chunks`), a single entry point consumed by both the read path (`event_store.rs:648, :1313`) and the append-condition guard path (`:290`); `tests/wide_query.rs` `18 passed`, including four negative controls a first-chunk-only implementation would fail |
| AC-009 — limits are declared facts, and the rule runs | **Yes** | Ceilings mirrored from `SqliteEventStore`'s own constants rather than restated (`tests/support/mod.rs:189-195`); `append_reports_exceeded_store_limits` present in the binary and green. CF-40's clause home is left open: `.kb/open-questions/cf-40-fixture-limits-ownership.md` is still `kind: open_question` |
| AC-010 — durability clauses leave with verdicts | **Yes** | ES-35 restated `[PROVISIONAL]` with a narrowed, live falsifier at `spec/SPECIFICATION.md:4213`; CF-17 confirmed at `:7990-8012`; CF-14's deferral confirmed and narrowed at `:7874-7922`. `spec-trace`'s falsifier-length check is green in my own re-run |
| AC-011 — the projection store passes the suite it did not write | **Yes** | `tests/projection.rs:226`, `24 passed; 0 failed`, on an independently-migrated connection. Critically, the borrowed suite was **not edited to pass it**: `crates/happenstance-testkit/src/projection.rs` is absent from this project's changed-file set |
| AC-012 — benchmarks exist and are provably not conformance | **Partially — substance yes, letter no** | The second half is fully met and *checked rather than asserted*: `cargo xtask lints` reports 112 rules across four rule files, unchanged. The first half: the harness is at `crates/happenstance-testkit/src/bench.rs`, feature- and target-gated, mounted and executing 18 tests in the gate at `tests/memory_benchmarks.rs`. But the SQLite arm is not gate-resident — see *Residual* below |
| AC-013 — the decision record carries a number | **Yes** | `.kb/decisions/0022-append-condition-strategy.md` (`kind: decision`, `status: accepted`) plus the 619-line record at `references/adr/0022-append-condition-strategy.md`, committed at `791b929` **before** the first implementation story `8381c89`. Numbers rather than preferences: 23 and 213 microseconds for the `max(position)` guard against 32 and 311 for the `EXISTS` probe, 10.7 ms against 34.0 and 49.8 ms on tag storage, and `busy = 0` at 64 connections |
| AC-014 — instrument markers gone, gate green | **Yes** | Verified directly: no `todo!(` anywhere under `crates/happenstance-sqlite/src/`, and `#![allow(clippy::todo)]` survives only as the string its guard test greps for (`tests/front_page.rs:62`). `7 passed`. Scoped clippy `-D warnings` is green **because** the exception is gone rather than despite it |
| AC-015 — publishable, publishing stays someone else's decision | **Yes** | Verified against the registry independently of the ledger: the crates.io API for `happenstance-sqlite` returns `"max_version":"0.0.0"`, `"yanked":false`, and a description byte-identical to `Cargo.toml:3`. README and both `LICENSE-MIT` and `LICENSE-APACHE` are present. `publish = false` and the three-crate `PUBLISHABLE` set (`xtask/src/package.rs:86`) are deliberately untouched, which is exactly what the criterion asks for |
| AC-016 — the specification and the code still agree | **Yes** | My own `cargo xtask spec-trace` run: `201 clauses (137 FROZEN, 47 PROVISIONAL, 12 DEFERRED, 5 NON-NORMATIVE), 112 conformance rules, 401 citations checked (80 anchored, 12 external)`, `traceability: no problems found` — reproducing `_integration.md` line 62 exactly and clearing the recorded merge-base baseline of 389/76. No citation was deleted; the pass is written up at `spec-and-code-reconciliation/_reconciliation.md` |

### Gate and DoD results

Every check below was executed by this review at HEAD on a clean tree, scoped to the affected
packages (`happenstance-sqlite`, `happenstance-testkit`, `xtask`) — never the unfiltered
whole-repo script.

| Check | Result |
| --- | --- |
| Formatter check across the workspace | **exit 0** — green, and the non-negotiable bar is cleared |
| Scoped tests over the three packages, `--all-features` | **exit 0** — zero failed across every target |
| Scoped clippy, `--all-targets --all-features`, `-D warnings` | **exit 0** |
| `cargo xtask lints` | **green** — 112 rules, every stated count matching the suite |
| `cargo xtask spec-trace` | **green** — 401 checked / 80 anchored, `no problems found` |
| `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | **`affected gate passed`**, exit 0 |

DoD: `_integration.md` records `dod_green: true` and `reachability_ok: true`, with
`cargo xtask ci --fast` — the non-terminal project bar at `.redkiln/config.yaml:50-55` — reported as
`all required checks passed (--fast: 4 optional step(s) not run)`. **DoD 3 is this project's own**
and was executed here rather than deferred: the conformance suite green against a real file, the
concurrency family green at 64 contenders, and an acknowledged write surviving a genuine reopen
through two real handles onto one file. The fifteen entries under `deferred_scenarios` are
whole-initiative journeys owned by named sibling projects and are not held against this project.

### Escape-hatch, unmounted and scope-drift findings

**None blocking.** Four candidates were examined adversarially and all four cleared.

1. **`unimplemented!` at `src/projection_store.rs:762`** — not a stub standing in for work not
   done. `READS_THROUGH_BATCH` is a contract constant present at merge-base `90cbca5`
   (`crates/happenstance-core/src/projection.rs:583`), the suite's declension path predates this
   project (`crates/happenstance-testkit/src/contract.rs:836`), and the panic is the honest answer:
   any value returned would be a claim about pending writes SQLite was never told about, which
   PS-12 forbids. It is allowlisted by path **and** enclosing function name, so it cannot spread.
2. **`MID_BATCH_FAULT` declined** — a declension, not a skip. Both fault rules remain in the binary
   reporting the fixture's stated reason. The reason names the injection mechanism as *available*
   rather than denying it, after repair `9a10dbf` replaced an earlier spelling that falsely claimed
   incapacity. AC-010 explicitly permits ES-35 to remain provisional with exactly this falsifier.
3. **`xtask/src/spec_trace.rs` changed** (`BARE_NAME_MAP` grew from 6 entries to 7), which
   `_integration.md` describes as an empty diff — true for the final story, not for the cumulative
   project diff, so I checked it. The entry landed in `23bc776`, the same commit that created
   `crates/happenstance-sqlite/tests/append.rs` and thereby broke that basename's uniqueness for 24
   pre-existing citations, every one of which predates the file and names the contract. It follows
   the map's own established pattern (an identical projection entry sits directly above it) and the
   anchor check remains the backstop. A disambiguation, not a relaxation.
4. **`crates/happenstance/tests/projection_clauses.rs` modified outside the affected set** — the
   assertion is strengthened rather than weakened: it now demands an accepted atom carrying
   `status: accepted` instead of a staged intake file that ingest was always going to consume.

**No scope drift.** The three things the project was forbidden to resolve unilaterally are all still
open and owned elsewhere: CF-40's clause home, DoD 7's second batch shape, and the decision to
publish. `happenstance-core` is untouched.

### Residual — AC-012's SQLite arm (recorded, not blocking)

AC-012 words itself as *"`event_store_benchmarks!` runs against the SQLite fixture behind a `bench`
feature"*. No such invocation exists under `crates/happenstance-sqlite/`. The harness *does* run
against real SQLite over real `rusqlite` connections — five invocations behind the `bench` feature
at `experiments/append-condition/tests/measure.rs:118-148` — and those runs produced the numbers
ADR-0022 quotes, which is what AC-013 needed and got. `experiments/*` is not a workspace member
(`Cargo.toml:3`), so that arm sits outside the gate, where `CLAUDE.md` deliberately puts
experiments.

This was a pre-code planning decision rather than a late discovery: `_storymap.md:49` mounts the
family against `MemoryFixture` and `:50` assigns the real-SQLite measurement to ADR-0022's
experiment, and `measure.rs:4` already names the in-crate mount as a later addition. Nothing was
faked to buy a green, no assertion was pinned, and no criterion in this project rests its evidence
on a gate-resident SQLite benchmark. It costs `ac-coverage` one point and is left for whichever
project adds the permanent mount.

### Advisories carried forward (not this project's to fix)

- `HS-S0045`, `HS-S0046` and `HS-S0047` read `status: ready` / `stage: plan` while their work is
  implemented, reviewed, ledgered and committed. The CLI is the only writer of those fields, so
  neither the integration audit nor this review touches them; the orchestrating command owns the
  transition.
- `RUNBOOK.md`'s phase-8 status row still reads `not started` (`:159`) and its exit checkboxes are
  unticked. Phases 6 and 7 read the same way despite having merged on this branch, so this is an
  initiative-wide closeout pattern rather than a defect this project introduced.
- Two conformance rules are claimed by no clause and owe an ADR
  (`k_disjoint_boundaries_admit_exactly_k_commits` and `ops_agree_with_the_model`). `spec-trace`
  reports both and stays green, which is its designed behaviour; neither was introduced here.

### Per-story checkpoint SHAs

Derived from git rather than from the seed — `git log 90cbca5..HEAD --grep "Story: sqlite-durable-store/"`
— which reproduces the fifteen workflow-reported checkpoints exactly: fourteen story checkpoints
plus the final slice seal.

| Story | Checkpoint |
| --- | --- |
| `benchmark-harness` | `2665883` |
| `adr-0022-append-condition-strategy` | `791b929` |
| `schema-migration-and-identity` | `8381c89` |
| `append-atomicity-and-store-limits` | `23bc776` |
| `lazy-read-with-snapshot-ceiling` | `0c6ce2b` |
| `wide-query-chunked-not-refused` | `11596b4` |
| `sqlite-fixture-and-whole-suite` | `41a2064` |
| `concurrency-family-and-contender-count` | `995b987` |
| `model-family-and-mutant-pass-column` | `0ad702f` |
| `reopen-negative-control-and-durability-verdicts` | `2d08e0d` |
| `projection-store-passes-the-borrowed-suite` | `1afb47b` |
| `instrument-markers-removed-and-gate-green` | `62a05dd` |
| `crates-io-name-and-packaging-facts` | `ada4962` |
| `spec-and-code-reconciliation` | `54df29a` |
| slice seal — `publishable-and-reconciled` | `82584ff` |

**Out-of-band repairs: none.** No commit in the range carries a `Baseline-Repair:` or
`Slice-Repair: sqlite-durable-store` trailer, which matches the seed. Seven in-slice review-fix
commits do exist and are expected rather than drift — they are the adversarial loop working, and
two of them repaired real cross-slice defects: `9a10dbf` (the append-condition guard path did not
chunk, only `read` did) and `897ae70` (a negative control that could not fail). The remaining five
are `2172b40`, `587fd78`, `c510468`, `67365be` and `4ad58d0`.

## Required Changes

None — the verdict is `approved`.

Carried forward as a note for the initiative rather than a change owed by this project: AC-012's
permanent `event_store_benchmarks!(SqliteFixture::new())` mount, which `_storymap.md:50` and
`experiments/append-condition/tests/measure.rs:4` both already name as a later addition.
