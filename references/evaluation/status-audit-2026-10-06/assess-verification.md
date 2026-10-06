# Assessment — verification (test and conformance evidence)

Commit `1f92d08` (== origin/main), 2026-10-06. Read-only. Sources: `gate-summary.md` (authoritative), `gate2.log`, map-spec / map-core / map-adapters, and direct spot-checks in `audit-wt` plus GitHub Actions (REST via `gh`).

**Rating: AMBER. Completeness against the 1.0 bar: about 65%.**

**Headline:** The event-store and projection-store contracts are tested to an unusually high standard, including mutation-tested conformance rules that run against live databases in CI. The replication contract has no executable checks yet, and three verification legs are red or skipped. A real Durable Object fails one rule on main. Neon fails a provisional rule intermittently. The local gate exercises none of the Postgres or Neon tests.

## What the measured evidence SHOWS

| Area | Evidence | Strength |
|---|---|---|
| Whole workspace, local | `cargo xtask ci`: 31 required steps green, **2,965 passed / 0 failed / 288 ignored** (gate-summary.md) | strong |
| Gate on 3 OSes | CI run 37406385109: `gate (ubuntu/macos/windows)` all success | strong |
| Event-store rules self-proven | `tests/mutation_coverage.rs`: 84 `Kind::Mutant` + 2 conformant variants, 7 racers. `every_rule_has_a_mutant` (`:3275`) has "no exemption list" (`:3265`) | very strong |
| Projection rules self-proven | `tests/projection_mutation_coverage.rs`: 18 mutants + 3 conformant variants for 17 rules (`for_each_projection_store_rule!`) | strong |
| Postgres, live | `conformance against a live Postgres` job success. Map reports 108/108 event-store and 21/21 projection, with the listed-equals-executed accounting at `ci.yml:441-607` | strong |
| Neon, live | `live Neon` job success on main (map reports 108/108 and 19/19) | strong, with a known flake (F3) |
| Cloudflare on a real DO | workerd job: local `collected 97; executed 97 (passed 96, failed 1)`, deployed `executed 96 of 96; 1 failed` | good but red (F2) |
| Spec traceability | §7.2 has 203 rows, regenerated and diff-checked by `spec-trace`. The map resolved 21 of 21 sampled rule names | strong |
| Send/!Send constraints | `memory.rs:628` `send_flavour_stream_is_send_in_generic_code`, `memory.rs:657` `spawns_from_generic`, and `happenstance/tests/flavours.rs` | strong |

What the 288 ignored tests are (measured from `gate2.log`):
- Postgres live tests: 108 (postgres_conformance), 21 (projection), 20 (live_projection) and 1 (poll_shape).
- Neon live tests: 108 (neon_conformance) and 19 (neon_projection).
- About 11 ignored doctests.

The local gate therefore executed **none** of the Postgres or Neon adapter's conformance rules.

## Findings

### F1 (high) The replication (SY) contract is frozen on design alone
- Evidence: in the §7.2 generated table, 32 of 35 SY rule cells are daggered (†). By family, the daggered rows are SY 32, ES 9, PS 7 and CF 1. 21 SY clauses are FROZEN.
- SY-8 cites "the whole of `happenstance-sync-testkit`", and that crate does not exist (`ls crates` lists only `happenstance-sync`).
- `xtask/src/spec_trace.rs:1423-1460` exempts the family from resolution.
- `happenstance-sync` still has `todo!()` in `lib.rs`, `memory.rs` and `ingest.rs`, and it is `publish = false`.
- Result: about 14% of all clauses, and the whole replication story, carry no executable check. Phase 13 is not started.

### F2 (high) The real-platform Cloudflare check is red on main, and the gate's shim hid the defect
- Evidence: CI job 112084786585 fails `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (VT-23) on both legs with `too many SQL variables`.
- The gate's wasm32 run is "**not under `workerd`**" (`crates/happenstance-cloudflare/tests/durable_object_conformance.rs:1-10`). It uses a Node shim with "none of the platform's storage ceilings", so the gate stayed green.
- The published constants say `MAX_QUERY_ARMS_PER_STATEMENT = 400` and `MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000` (`event_store.rs:402,423`). The real platform caps bound parameters at 100.
- The `workerd` job is "Not a required status check" (`ci.yml:875`), so main merged red.
- A fix is in flight but not on main: open PR #35 "Phase 17 L6b … (workerd green)", whose CI run 37419423991 is success.

### F3 (medium) Neon fails a required check intermittently on ES-11/ES-12
- Evidence: of the 54 non-cancelled CI runs since 2026-09-28, 4 failed the live-Neon job:
  - run 36528489181 failed both rules;
  - run 36596302787 failed `query_items_share_one_snapshot` (`suite.rs:6250`);
  - runs 36619371439 and 36775667956 failed `read_result_is_stable_under_concurrent_append` (`suite.rs:6147`).
- The spec records that the clause's falsifier has fired (`SPECIFICATION.md:3087`). The Neon README says "this crate does not claim ES-11" (`README.md:89`).
- This is honest, but a published adapter does not pass the suite. The settling record (phase 17) is not written.

### F4 (medium) The live-service evidence exists only in CI
- Evidence: locally, 277 adapter tests were ignored as "needs a live Postgres" or "needs a live Neon endpoint" (`ignored-reasons.txt` and `gate2.log`).
- The live-Neon job on fork PRs skips everything and goes green ("proves NOTHING", `ci.yml:716-718`).
- A green `cargo xtask ci`, the "before saying done" bar in CLAUDE.md, therefore says nothing about two of the four published adapters.

### F5 (medium) The store the projection port was frozen against is not run in CI
- Evidence: the `--test` targets in `ci.yml` (lines 441, 491, 524, 531, 647, 738, 740, 784, 786) are `postgres_conformance`, `projection`, `rule_controls`, `naive_arm_probe`, `neon_conformance` and `neon_projection`. **There is no `live_projection` target.**
- `crates/happenstance-postgres/tests/live_projection.rs:20-30` says it is the only real-database store on which `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant` execute rather than skip.
- The spec claims it "passes" the suite (`SPECIFICATION.md:405`). That claim rests on a manual run.

### F6 (medium) FROZEN ES/PS clauses that have no executable rule
- Daggered or Scheduled, with no rule found in the repo:
  - ES-6 `store_error_crosses_a_join_handle`, which `spec_trace.rs:1167-1173` argues is "unwritable against today's port";
  - ES-29;
  - ES-31;
  - ES-38 `positions_are_not_reused_after_removal`;
  - the runner rules PS-26, PS-28 and PS-29 (Scheduled for "the projection runner's own rules").
- `*(none — see clause)*`: ES-23, ES-37, PS-9, PS-31, PS-36 and PS-37.
- Adding the 21 SY clauses gives about 34 of 152 FROZEN clauses (about 22%) with no executable check. The runner's failure policy, fan-out and panic handling are among them.

### F7 (medium) Generative and contention evidence covers only the serialised-writer shape
- `ops_agree_with_the_model` (the proptest model check) skips whenever `READ_YOUR_OWN_WRITES` is declined (`crates/happenstance-testkit/src/model.rs:693-697`). Postgres and Neon decline it, so the model check runs only on the memory store and SQLite.
- Cloudflare does not invoke the concurrency or model families (`durable_object_conformance.rs:46`).
- `racing_conditional_appends_elect_one_winner` (`suite.rs:5837`) is sequential on one handle. Real-thread contention lives only in the opt-in concurrency family.
- Result: the strongest differential checks miss exactly the axis that CLAUDE.md says ports must be checked against.

### F8 (low) There is no integration layer, and the case catalogue has no status
- `spec/E2E-CASES.md:22` puts the 9 integration-level cases in `tests/e2e/`, "not yet created", and that directory is confirmed absent.
- The catalogue has no implemented/pending marker per case, and several "blocked" markers are stale (map-spec §5).
- Prose counts are stale: the spec says "89 conformance rules" (`SPECIFICATION.md:404`) while the registry has 95.

### F9 (low) Durability-fault and performance evidence is out of scope until after 1.0
- CF-14 is DEFERRED (renew-past-1.0) and ES-35 is PROVISIONAL. Reopen is tested, but crash or power-loss fault injection is not built.
- The benchmark job only compiles.
- Local gate: cargo-hack, cargo-deny and the nightly docs.rs builds were skipped for lack of the tools (gate-summary.md).
- DO eviction and hibernation are untested.
- One unrelated SQLite projection test flaked on Windows (`concurrent_opens_of_one_path_all_succeed`, run 36962066186).

## Strengths
- No failures in 2,965 tests. The full gate is green locally and on three OSes in CI.
- Conformance rules are mutation-tested, and the meta-test forbids an undecorated (unfalsifiable) rule outright.
- CI accounting is strict: the live jobs assert listed == executed, 0 ignored and 0 failed.
- Spec-to-rule traceability is enforced by a gate step. Daggers honestly mark rules that are not written.
- Documentation is candid about failures: the Neon ES-11 disclaimer, workerd "landed red on purpose", and reported skips with reasons.
