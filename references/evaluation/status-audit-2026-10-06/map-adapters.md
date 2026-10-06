# Audit map: storage adapters and the workerd runtime harness

Worktree: `audit-wt` at `1f92d08` (== `origin/main`), read 2026-10-06. Read-only. No cargo build/test was run;
CI evidence comes from `gh run view` on the latest `main` CI run **37406385109** (push of `1f92d08`,
2026-10-06T02:54Z) and from files in the tree. "Docs claim" vs "code/CI shows" is kept separate throughout.

---

## 1. At a glance

| Adapter | Roles implemented | src LOC | tests LOC | Conformance families invoked | Where it runs in CI | Latest main CI (run 37406385109) | Required check? |
|---|---|---|---|---|---|---|---|
| `happenstance-sqlite` | EventStore (`SendEventStore`), ProjectionStore (`SendProjectionStore`) | 6,868 | 6,272 | event-store, model, concurrency, projection | `gate` (3 OSes), no external service | gate green on ubuntu/macos/windows | yes (gate) |
| `happenstance-cloudflare` | EventStore only (bare `EventStore`, `!Send`) | 8,758 | 3,649 | event-store only (wasm32) | `gate` (node:sqlite shim, wasm32) + `workerd` job (real DO, local + deployed) | gate green; **`workerd` red (by design)** | gate yes; **workerd no** |
| `happenstance-postgres` | EventStore + **two** ProjectionStores (buffered `PostgresProjectionStore`, `LivePostgresProjectionStore`) | 4,093 | 2,746 | event-store, concurrency, model, projection ×2 | `live-postgres` job (testcontainers, Docker) | green: 108/108 event-store, 21/21 projection | yes |
| `happenstance-neon` | EventStore + ProjectionStore (generic over `SqlTransport`) | 4,900 | 1,857 | event-store, concurrency, model, projection | `live-neon` job (needs `NEON_CONNECTION` secret) | green this run: 108/108, 19/19 | yes |
| `harness/workerd` | none (test harness, `publish = false`) | 798 Rust + 184 JS/TS (+3,593-line npm lockfile) | — | dispatches every event-store rule by name inside a real Durable Object | `workerd` job | local 96/97, deployed 95/96 — VT-23 fails on both | no |

Sources: `wc -l` over `src/*.rs` and `tests/**/*.rs` per crate; `grep -rn 'impl.*EventStore for\|impl.*ProjectionStore for'`
(sqlite `event_store.rs:1674`, `projection_store.rs:623`; cloudflare `event_store.rs:1093`; postgres `event_store.rs:462`,
`projection_store.rs:397`, `live_projection_store.rs:174`; neon `event_store.rs:775`, `projection_store.rs:367`).
Required checks: `gh api repos/Wet-Ink-Corporation/happenstance/rulesets/22926481` lists `gate (×3)`, `conformance against a live Neon endpoint`,
`conformance against a live Postgres`, `minimum supported Rust version`, `semver compatibility`, `the benchmark crate still compiles` —
**not** the `workerd` job.

Publishing: workspace `version = "0.4.0"` (`Cargo.toml:24`, unreleased); none of the four adapters carries a `publish` key
(`grep -n '^publish' crates/*/Cargo.toml` returns only ladybug and sync; `harness/workerd/Cargo.toml:11` is `publish = false`).
Docs claim all four are published at 0.3.2 (`runbook/handover.md`, "Seven crates are published at `0.3.2`") — not re-verified against crates.io here.

---

## 2. Conformance invocation, per adapter

Rule-family sizes (counted by regex over the enumeration macros): event-store ≈95 (`crates/happenstance-testkit/src/registry.rs:94`),
projection 20 (`projection.rs:1937`), concurrency 6 (`concurrency.rs:1391`), model 1 rule `ops_agree_with_the_model` (`model.rs:813-820`).
The workerd runner collected 97 tests locally and the deployed leg listed 96 (CI log, see §6).

| Adapter | Invocation(s) | Emitter / gating |
|---|---|---|
| sqlite | `tests/conformance.rs:100` `event_store_conformance!(SqliteFixture::new())`; `:102` `event_store_model_conformance!`; `tests/concurrency.rs:62` `event_store_concurrency_conformance!`; `tests/projection.rs:226` `projection_store_conformance!` | Plain tokio emitter, runs in the default gate. Blocking emitter deliberately not used because `NoRuntime` would redden rules (`tests/conformance.rs:59-71`, `tests/concurrency.rs:21-35`). Projection target needs `--all-features` (`tests/conformance.rs:46-56`). |
| cloudflare | `tests/durable_object_conformance.rs:69-73` `event_store_conformance!(emit = emit_wasm, fixture = CloudflareFixture::new())`, `#![cfg(target_arch = "wasm32")]` | Runs under `wasm-bindgen-test-runner` against a **Node `node:sqlite` shim**, "not under `workerd`" (`:1-10`). Gate step "wasm32 run of the conformance rules" (`xtask/src/main.rs` ~`:486-499`) is *probed* — skipped locally if `wasm-bindgen-test-runner` is absent, installed in CI. Concurrency + model families not invoked (`!Send`, no threads on wasm32; `:46-53`). |
| postgres | `tests/postgres_conformance.rs:118` event-store, `:136` concurrency, `:144` model; `tests/projection.rs:87`; `tests/live_projection.rs:64` | Custom emitters adding `#[ignore = "needs a live Postgres…"]` (`:59-116`); run with `-- --ignored`. Container `postgres:17.10`, `max_connections=200`, schema per fixture (`tests/support/mod.rs:80-81,137,177`). |
| neon | `tests/neon_conformance.rs:131` event-store, `:146` concurrency, `:154` model; `tests/neon_projection.rs:83` | `#[ignore]` emitters; reads `NEON_CONNECTION` (`tests/support/transport.rs:232-237`); must run `--test-threads=1` because the visibility frontier is held back by sibling rules (`README.md` "Running its tests"). |
| workerd harness | `harness/workerd/src/dispatch.rs` expands `for_each_event_store_rule!` into a name→rule `match`; `test/conformance.test.ts` (30 lines) iterates names; `deployed.mjs` (79 lines) drives the deployed Worker | Strict: exits non-zero on any rule failure (`deployed.mjs:10-18,75-78`); retries only Cloudflare's literal `500 Worker not found.` once (`:35,54-56`). |

---

## 3. Capability declines (what is skipped, and why)

Capability constants live on `Fixture` (`crates/happenstance-testkit/src/contract.rs:172,243,300,334,379`) and `ProjectionFixture`
(`RESET_REFUSAL`, `COMMIT_FAULT`). A declined capability turns its rule(s) into a reported `SKIP` that still counts as "passed" in libtest.

### Event-store fixtures

| Capability | sqlite (`tests/support/mod.rs`) | cloudflare (`tests/support/mod.rs`) | postgres (`tests/support/mod.rs`) | neon (`tests/support/mod.rs`) | workerd (`src/fixture.rs`) |
|---|---|---|---|---|---|
| `SECOND_HANDLE` | supported `:135` | supported `:179` | supported `:422` | supported `:122` | supported `:110` |
| `REOPEN` | supported `:140` | supported `:191` | supported `:431` | supported `:133` | supported `:115` |
| `READ_YOUR_OWN_WRITES` | default (supported) | default | **declined** `:451-456` — `xact_id < pg_snapshot_xmin(...)` frontier; "a durability promise, not a visibility one" | **declined** `:145-151` — same server frontier | default |
| `MID_BATCH_FAULT` | **declined "by scope, not by incapacity"** `:173-184` (the trigger mechanism exists and is exercised by `tests/append.rs::a_failure_mid_batch_leaves_nothing`) | supported `:244` (trigger in object's SQL) | supported `:473` | supported `:173` (txn-local GUC counter) | supported `:116` |
| `READ_FAULT` | **declined by incapacity** `:199-207` (one rusqlite statement held open, no page fetch to fail) | **declined by scope** `:258-264` (read is not paged across an await) | supported `:488` | supported `:198` | **declined by scope** `:117` |

**Consequence the docs state and CI confirms:** declining `READ_YOUR_OWN_WRITES` makes the model family (`ops_agree_with_the_model`) skip
(`crates/happenstance-testkit/src/model.rs:693-695`). The CI log for run 37406385109 prints `SKIP ops_agree_with_the_model: fixture declines
READ_YOUR_OWN_WRITES` on both live-postgres and live-neon. So **the generated/property-based check runs only against SQLite** (and the
in-memory store) on every CI run; Postgres and Neon never execute it.

### Projection fixtures

| Capability / probe | sqlite | postgres buffered | postgres live | neon |
|---|---|---|---|---|
| `RESET_REFUSAL` | declined `tests/projection.rs:162-171` (no protection policy) | declined `support/mod.rs:759` | declined `:879` | declined `support/mod.rs:393` |
| `COMMIT_FAULT` | supported `:181` | supported `:776` | supported `:887` | supported `:410` |
| `READS_THROUGH_BATCH` (store const) | `false` `src/projection_store.rs:803` | `false` `src/projection_store.rs:634` | **`true`** `src/live_projection_store.rs:372` | `false` `src/projection_store.rs:666` |

With `READS_THROUGH_BATCH = false`, `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant` are skipped (CI log, live-postgres
and live-neon, run 37406385109). `probe_read_through` is `unimplemented!()` on the three buffered stores by design
(sqlite `projection_store.rs:868`, postgres `projection_store.rs:695`, neon `projection_store.rs:756`) — reachable only if the const were wrong.

**Gap (code shows):** `LivePostgresProjectionStore` is the *only* real-database store on which those two rules run
(`tests/live_projection.rs:22-29`), and it is the instrument ADR-0063 froze the projection port against. **No CI job runs
`--test live_projection`**: `grep -n -- '--test ' .github/workflows/ci.yml` lists only `postgres_conformance`, `projection`, `rule_controls`,
`naive_arm_probe`, `neon_conformance`, `neon_projection`. Its 6 `#[ignore]`d tests run only when someone runs them by hand. The same is
true of `tests/poll_shape.rs` (1 ignored test). The spec claims it "passes" the suite (`spec/SPECIFICATION.md:405`) — that is a claim about a
local run, not something CI re-observes.

---

## 4. Tests that need live services, and whether CI runs them

| Service | Tests | CI job & gating | What actually happens |
|---|---|---|---|
| Docker / testcontainers Postgres 17.10 | postgres: `postgres_conformance` (14 ignore attrs), `projection` (7), `live_projection` (6), `poll_shape` (1), `rule_controls`/`naive_arm_probe` (under `--cfg happenstance_naive_arm`) | `live-postgres` (`ci.yml:412-678`), no secret, runs on forks too | Runs `postgres_conformance` + `projection` + naive-arm pair; asserts listed == executed, 0 ignored, no failures (`ci.yml:497-607`). Steps run under `bash -e -o pipefail` (confirmed in job log), so a failing `cargo test | tee` fails its step. `live_projection` and `poll_shape` **not run**. Event-store count floor is `>= 100` (`ci.yml:444`); actual 108. |
| Neon endpoint | neon: `neon_conformance` (15 ignore attrs), `neon_projection` (4) | `live-neon` (`ci.yml:693-860`), `NEON_CONNECTION` secret via job-level env | Fork PR: every step skipped, job green "proves NOTHING" (notice, `ci.yml:716-718`). Same-repo run with no secret: hard error (`:719-723`). Floors: ≥100 event-store, ≥17 projection (`:744,748`). Latest main: 108/108 and 19/19. |
| Cloudflare account | workerd deployed leg | `workerd` job (`ci.yml:876-1041`), `CLOUDFLARE_API_TOKEN` + `CLOUDFLARE_ACCOUNT_ID` | Per-run Worker deployed with `wrangler deploy --var HARNESS_TOKEN:…`, deleted at end (`:993-1041`). Fork: notice + skip; same-repo without secrets: error. |
| Node + wasm-bindgen-test-runner | cloudflare wasm32 conformance | `gate` step (probed) | Runs in CI on all 3 runners; locally skipped if the runner binary is absent (`xtask/src/main.rs` comment ~`:463-484`). |
| local workerd (vitest-pool-workers 0.22.0) | harness local leg | `workerd` job | No credentials needed; runs on forks. |

---

## 5. Code-hygiene counts (production `src/`, excluding comment lines)

Workspace lints: `unsafe_code = "forbid"` (`Cargo.toml:213`), `clippy::unwrap_used = "deny"` (`:239`), `clippy::todo = "deny"` (`:245`); CI runs clippy with `-D warnings`.

| Crate | `todo!()` | `#![allow(clippy::todo)]` | `unimplemented!()` | `.unwrap()` | `.expect(` | `panic!(` | Notes |
|---|---|---|---|---|---|---|---|
| sqlite | 0 | 0 | 1 (probe, by design) | 126, all under `#[cfg(test)]` modules with scoped `allow(clippy::unwrap_used)` (`event_store.rs:2413,3065`, `query_sql.rs:878`, `ingest_spike.rs:79`, module gated `lib.rs:121`) | 26 | 5 | |
| cloudflare | 0 | 0 (two hits are prose) | 0 | 0 | 215 | 12 | `expect` is not denied. Most sit in test modules (`#[cfg(test)]` at `event_store.rs:334+`, `namespace.rs:173`, …); `host.rs:271` (`expect` on the Node shim) is host-only test support. Not exhaustively classified. |
| postgres | 0 (one hit is prose, `event_store.rs:470`) | 0 | 1 (probe) | 0 | 12 | 2 | |
| neon | 0 | 0 | 1 (probe) | 39, all after the first `#[cfg(test)]` | 19 | 8 | |
| harness/workerd | 0 | 0 | 0 | 0 | 0 | 0 | |

Stale comment (code shows): `crates/happenstance-postgres/tests/postgres_conformance.rs:153` still says "`append`, `head` and `contains_event_id`
are still `todo!()`, so every rule in the expansion above would panic if run" — false; real bodies at `src/event_store.rs:517,599,618`, and
CI runs 108 of them green.

---

## 6. Known open issues

### 6.1 workerd job "landed red on purpose" (`1f92d08`, PR #34) — what is red and why

**CI shows (run 37406385109, job 112084786585):** steps "The conformance rules under local workerd" and "Every rule passed under local
workerd" fail — `collected 97; executed 97 (passed 96, failed 1)`; the deployed leg fails — `executed 96 of 96; 1 failed`. The single
failure on both legs is `store_evaluates_a_query_at_the_guaranteed_minimum_item_count` (VT-23, spec `SPECIFICATION.md:1615`, `[PROVISIONAL]`
per `:9830`) — `Rust panic … read should succeed, … too many SQL variables`.

**Why:** a real Durable Object's SQLite is far tighter than stock SQLite. Measured (`experiments/durable-object-limits/results/run-workerd-local-2026-10-01.txt`,
`run-workerd-deployed-2026-10-02.txt`): compound `SELECT` terms **5**, bound parameters **100**, statement **100,000 B**, expression depth **100**;
row payload 2,199,995 B (local workerd 1.20260815.1) vs **8,388,637 B** deployed. Through the adapter: **at most 5 query items, 5
append-condition items, and 45 tags in one query item**. VT-23 requires 128 items.

**The adapter's published constants are wrong for the real runtime.** `CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT = 400`
(`src/event_store.rs:402`) and `MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000` (`:423`) are justified in their docs by stock SQLite defaults
(`SQLITE_MAX_COMPOUND_SELECT` 500, `SQLITE_MAX_VARIABLE_NUMBER` 32,766; `:381-422`) and were measured only against the shim. The
gate's shim "does not enforce the platform's documented caps at all" (`README.md` limits section) — which is why the gate stayed green.

**Remedy is planned, not done:** lane L6b — render with `json_each` so each item binds constant parameters, set the arm width ≤5 and the
parameter budget <100, write ADR-0083, record a green run (`runbook/handover.md` "Next action" 3; commit message of `1f92d08`). Also still
owed: the vacuity control (drop one dispatch name, watch the job go red) (`handover.md` Next action 2; phase-17 log 2026-10-02).
`workerd` is not a required check, so main merged red; `main`'s CI run is red today because of it.

What the deployed leg *does* prove: 95 of 96 rules pass in a real deployed Durable Object (run 36967951105 per `runbook/phases/17-breaking-window.md:529-535`,
and 95/96 again in run 37406385109). Not covered: eviction and hibernation (`crates/happenstance-cloudflare/README.md` "workerd runs them too").

Side additions in `1f92d08`: `CloudflareEventStore::namespaced` + `TableNamespace` (`src/namespace.rs`, 537 lines, `MAX_LEN = 64` at `:68`),
flagged as a new 1.0 promise in the commit message.

### 6.2 ES-11 on Neon (transport fence)

**Docs claim:** the crate "ships carrying one conformance rule it does not pass" — `read_result_is_stable_under_concurrent_append` reddens
intermittently because a read and an append are two independent HTTP requests (`crates/happenstance-neon/src/lib.rs:17-24`;
`README.md:64-96` "this crate does not claim ES-11"). ES-11 is `[PROVISIONAL]` (`spec/SPECIFICATION.md:9874`); ES-12's
`query_items_share_one_snapshot` is "not immune" (`:3214`). No failure rate is quoted because no raw log was retained (`README.md:72-75`).

**CI shows:** the job is kept strict (no tolerance for the named failure, `ci.yml:794-860`), so it is **intermittently red**, and it is a
**required check**. Handover trap: "Live Neon flakes on the ES-11/ES-12 race … until L8 lands its fence. Re-run it" (`runbook/handover.md`).
Latest main run happened to pass 108/108.

**Plan:** phase 17 work item "The ES-11 record, settling ES-11 and ES-12 together" is unchecked (`runbook/phases/17-breaking-window.md:162-168`);
the owner chose to spike a fence rather than take a named exception (`:296`). Not started in tree (unverified beyond the unchecked box).

Related hazard: `ProbeThenWriteStore` — a deliberately wrong (lost-update) `EventStore` kept as a negative control — is a **`pub struct`
in the published crate's public `event_store` module** (`src/event_store.rs:1475-1486`; `lib.rs:147` `pub mod event_store`), not
`#[cfg(test)]` or `#[doc(hidden)]`. It is documented as wrong (`:1441-1473`), but it is public API a 1.0 would promise.

### 6.3 ADR-0022 §9: runtime `Handle` captured at construction

**Code shows:** `SqliteEventStore::with_store_id` captures `Handle::try_current().ok()` (`crates/happenstance-sqlite/src/event_store.rs:512`),
as do `SqliteProjectionStore` (`projection_store.rs:234`) and `PostgresEventStore` (`crates/happenstance-postgres/src/event_store.rs:314`,
falling back to `try_current` at `:367-368` and erroring `NoRuntime`). Under a non-tokio executor or a dropped runtime, reads fail with
`NoRuntime` (`sqlite event_store.rs:1557`, `postgres error.rs:118`); the test files document that the blocking emitter would therefore
redden every read (`sqlite tests/concurrency.rs:21-35`).

**Status:** open. Phase-17 item "ADR-0022 §9's reproduction" and its exit criterion are unchecked (`17-breaking-window.md:105-116, 267-269`);
`.kb/open-questions/adr-0022-falsifiers-have-fired.md:251-266` says §9 "stays open, owned by phase 17". No reproduction test found
(`grep -rln 'stranded|outlives.*runtime' crates/happenstance-{sqlite,postgres}/tests` → nothing). If the remedy changes `open`/`new`, it is
a breaking change that must land in 0.4.0.

### 6.4 `AppendError::Busy` (ES-43, ADR-0077)

**Done (code shows):** commit `159ed70` (PR #32). Reported by sqlite (`event_store.rs:1093`, when `BEGIN IMMEDIATE` outlasts the 15 s
`BUSY_TIMEOUT_MS`, `connection.rs:120`), postgres (`event_store.rs:565`), neon (`event_store.rs:844`), each with a test
(sqlite `:2783`, neon `:2143`). Cloudflare does not emit it (single-threaded actor; not needed — inference, not verified by a doc statement).
Phase 17 marks the question superseded (`17-breaking-window.md:75-80`).

### 6.5 Other open items touching these adapters

- Guard-plan `LIST SUBQUERY` assertion for sqlite (ADR-0068) — unchecked (`17-breaking-window.md:117-125, 270`).
- VT-6 store-id restore detection for Postgres/Neon (mint-per-open or not) — unchecked (`:129-137`; `.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md`).
- Postgres visibility frontier is server-global: a reader can be held back by an unrelated open transaction anywhere on the server —
  "0.688 ms unloaded and 4010.719 ms behind a five-second write elsewhere" (`crates/happenstance-postgres/tests/support/mod.rs:440-450`).
  ADR-0071 keeps ES-10 global (per CLAUDE.md). This is a real operational characteristic, not a bug.
- SQLite concurrency rule "flakes on Windows, worse with more CPU" (`runbook/handover.md` Traps). Gate on windows-latest was green in 37406385109.
- Handover is stale: its "As of" names PR #34 as open on `9d99339` (`runbook/handover.md` "As of"); PR #34 is merged as `1f92d08`.
- Out of topic but seen in the same run: the **Pages** workflow's deploy fails — `Failed to create deployment (status: 404) … Ensure GitHub
  Pages has been enabled` (run 37406385128). The docs site is not being published.

---

## 7. Production-readiness verdicts (candid)

### happenstance-sqlite — the most mature adapter; production-viable for single-node/embedded use
- Evidence for: every family runs in the default gate on three OSes, including the concurrency family and the only property-based (model)
  run against a real store (§2, §3). Both roles. Durability settings are public constants (WAL, `synchronous=NORMAL`, 15 s busy timeout;
  `README.md` "Durability"). Limits enforced and tested from both sides (1 MiB / 128 tags / 256 events; `src/event_store.rs:326-342`).
- Caveats: one `rusqlite::Connection` behind a `Mutex` per handle (`src/event_store.rs:240`) — single writer, serialised per handle;
  tokio-bound (`NoRuntime`) and §9 still open, so constructors may change in 0.4.0; `synchronous=NORMAL` can lose the tail on power loss
  (documented); projection store cannot read through its batch.

### happenstance-postgres — conformant and CI-verified; beta-grade for production
- Evidence for: required `live-postgres` job green, 108/108 event-store tests (event-store + concurrency at 64 contenders per CLAUDE.md +
  model skip) and 21/21 projection, on pinned `postgres:17.10` (CI log; `tests/support/mod.rs:80-81`). Fault injection supported for mid-batch
  and read faults. Busy reported.
- Caveats: visibility frontier is global and can lag by seconds behind unrelated long transactions (§6.5) — so the model family is skipped;
  only one Postgres version tested; the live-transaction projection store (the port's freeze instrument) is not run in CI; §9 open;
  benchmarks only compile in CI (`ci.yml:1232-1246`, `cargo check --benches`), so no performance/soak evidence in CI. Stale `todo!()` comment (§5).

### happenstance-neon — works, but carries a known, intermittent consistency failure; not production-ready for read-then-decide on hot keys
- Evidence for: required `live-neon` job; latest main green 108/108 + 19/19; real bodies throughout; Busy reported; append is a single
  SERIALIZABLE batch so conditional appends are protected (`src/event_store.rs:1466-1473`).
- Against: ES-11 not satisfied (intermittent `read_result_is_stable_under_concurrent_append`), fence spike not yet done; a required check
  that flakes; fork PRs prove nothing; tests must run serially; lower limits (128 KiB payload, 128 events, 64 MiB hard response ceiling,
  `README.md` limits); projection store must point at a primary (documented only, not testable); `ProbeThenWriteStore` publicly exported.

### happenstance-cloudflare — not production-ready beyond small queries
- Evidence for: 95 of 96 conformance rules pass in a **real deployed Durable Object** (§6.1); every capability supported except `READ_FAULT`;
  the gate runs the full family on wasm32 against a shim.
- Against: on the real platform it can evaluate only **5 query/condition items and 45 tags per item**, below the specification's 128-item floor
  (VT-23 red), while its published constants advertise 400 arms / 30,000 parameters; fix (L6b) not merged. Event store only — no projection
  store. Positions bounded by 2^53 (`README.md`). Eviction/hibernation untested. The `workerd` job that catches this is not a required check, and
  `main` is red because of it. The README itself calls it "the release with the thinnest evidence of the seven" (`README.md` status block).

### harness/workerd — a sound instrument, newly landed
- Strict counting (collected ≥97, none pending, executed == total; `ci.yml` workerd steps), the same enumeration feeds dispatcher and runner,
  panics relayed to JS before the wasm trap (commit message `1f92d08`). Owed: the vacuity control; L6b to turn it green; it is not a required check.

---

## 8. Commands run (for reproduction)

- `git log -1` → `1f92d08…` "Phase 17 L6a: the workerd job, landed red on purpose (#34)".
- `wc -l` per crate src/tests (table §1).
- `grep -rn 'conformance!' crates/happenstance-{sqlite,cloudflare,postgres,neon}/tests harness/workerd/src`.
- `grep -n -A4 'const [A-Z_]*: Capability'` over each fixture file.
- `grep -n -- '--test ' .github/workflows/ci.yml` (shows no `live_projection`).
- `gh run list -R Wet-Ink-Corporation/happenstance --branch main` → CI 37406385109 failure, Pages 37406385128 failure.
- `gh run view 37406385109 --log --job <id>` for live-postgres (112084786414), live-neon (112084786531), workerd (112084786585).
- `gh api repos/Wet-Ink-Corporation/happenstance/rulesets/22926481` → required status checks list.
