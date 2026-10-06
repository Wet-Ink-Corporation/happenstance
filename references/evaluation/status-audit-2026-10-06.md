<!--
Status-quo audit, 2026-10-06, pinned to 1f92d08 (== main at the time).
Dated and pinned: like the other review records in this directory it is
immutable evidence, superseded rather than edited. The executive summary is
status-audit-2026-10-06.html beside it; the evidence it cites is under
status-audit-2026-10-06/.
-->

# happenstance: independent audit at `1f92d08`

Audit date 2026-10-06. Subject: `1f92d08` (equal to `origin/main`), the merge of PR #34, "Phase 17 L6a: the workerd job, landed red on purpose".
Read-only audit. The gate was run once in an isolated worktree (section 5). Every other figure comes from the files, git, the GitHub API or the crates.io API, as cited.

**How to read this report.**
- Every finding carries an ID from `verified.json` and a status.
  - **confirmed**: two verifiers agreed.
  - **plausible**: verifiers judged it partly true. The wording used here is the verifier-corrected `statement`, not the assessor's original.
  - **unverified-low**: no verifier checked it. These findings sit in Appendix A, are hedged wherever they appear, and support no headline conclusion.
- Percentages written "by judgement" are estimates, not measurements.

---

## 1. Bottom line

The storage contract (`happenstance-core`), its conformance suite (`happenstance-testkit`) and three adapters are real, and they are verified against live databases:
- SQLite runs on three operating systems.
- Postgres 17.10 passes 108/108.
- Neon passes 108/108.

The measured local gate is green: 2,965 tests passed and 0 failed. The project discloses its own defects unusually candidly.

Three things stand between it and a credible 1.0:

1. **The edge adapter fails on its real platform, and `main` is red.**
   - `happenstance-cloudflare` fails VT-23's 128-item rule on a real Durable Object.
   - Its published limit constants are 80x and 300x above the platform's walls.
   - The fix is open, green and unmerged (PR #35).
2. **Sync is inside 1.0 but is the least-built part.**
   - No SY conformance rule exists. Of 35 SY rules, 0 are executable.
   - There is no `SyncRunner`, no `happenstance-sync-testkit`, and no networked peer.
   - Phase 13 is about a third of the remaining path.
3. **No outsider has used the API that 1.0 would freeze.**
   - `happenstance-core`'s 6 reverse dependencies are all in this workspace. The repo has 1 star.
   - The docs site has never deployed.
   - Phase 20's outside-reader work has not started.

**Estimate to 1.0.0** (section 7): **66–127 working days, central about 90**, plus a 1–3 week rc soak.
- Converting that to dates needs a cadence assumption, which is not a measurement. Under one, 1.0 most likely lands between late February and late May 2027.
- Taking scope levers 1, 2 and 4 brings the path to about 34–64 days.
- `0.4.0` is about late October 2026 at sprint pace, or late October to mid-November at long-run pace.

---

## 2. Scorecard

All eight dimensions are rated **amber**. The table gives the rating and `completeness_pct` exactly as they appear in `verified.json`.

| Dimension | Rating | Completeness % | One-line reason |
|---|---|---|---|
| vision-fit | amber | 55 | The contract and suite are delivered. Sync, the 1.0 headline, is mostly design, and the edge adapter fails on its real runtime. |
| plan-progress | amber | 55 | Phase 17 is at 3/10 exit criteria. The plan is lint-checked, but the hand-written records lag. |
| architecture | amber | 70 | The core is well designed and its constraints are enforced by tests. The weak points are at the edges: Neon ES-11, Cloudflare VT-23, and the SY port frozen on paper. |
| implementation | amber | 65 | SQLite, Postgres and Neon are real and live-tested. Cloudflare is red on workerd. Sync is mostly unbuilt. |
| verification | amber | 65 | Mutation-tested rules run against live databases. SY has no executable checks, and 34 of 152 FROZEN clauses have no executable check. |
| docs-dx | amber | 40 | The first-encounter path works. There is no projection guide and no migration guide, the site has never deployed, and no outside reader has tried the docs. |
| process-governance | amber | 65 | Release and CI machinery are sound. Two workflows are red on `main`, a liveness fix is held, and the governance load is large. |
| risk | amber | 40 | Sync sits on the critical path, the project has a bus factor of one, and no outside adopter exists. |

**Overall: amber.** The overall figure is the unweighted mean of the eight `completeness_pct` values: (55+55+70+65+65+40+65+40)/8 = 455/8 ≈ **57**. It is not weighted by importance and adds no information beyond the table.

---

## 3. Headline findings (high severity)

Seven confirmed-high findings, plus the plausible-high findings that describe the same failures. Wording follows each finding's verified `statement`, condensed where marked "...".

### 3.1 `main` is red: Cloudflare fails VT-23 on a real Durable Object (VF-2, A2, IMPL-1 confirmed-high; V2, PG-1 plausible-high)

- **VF-2 (confirmed, high).** "At 1f92d08 (main), the 'conformance under workerd, local and deployed' job in CI run 37406385109 fails on both the local and the deployed leg. The only rule that fails is VT-23's 128-item query, against the platform's walls of 5 compound terms and 100 bound parameters ... The shipped constants MAX_QUERY_ARMS_PER_STATEMENT = 400 and MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000 (event_store.rs:402 and :423) date from 0.2.0. They are 80x and 300x above those walls, so a published 0.3.x Cloudflare store will render queries of more than 5 items that a real Durable Object refuses. ADR-0066 §6's pre-1.0 condition, Cloudflare conformance on real workerd, is therefore not yet met."
- **A2 (confirmed, high).** Locally the adapter passes 96 of 97 with "too many SQL variables" (128 parameters against 100). On a deployed Durable Object it passes 95 of 96: CI run 37406385109, job 112084786585, steps 9, 11 and 15 failed. "The gate stayed green because it runs the adapter over a node:sqlite shim at those defaults."
- **IMPL-1 (confirmed, high).** The workerd job is not in required-checks ruleset 22926481, which is why PR #34 could merge red.
- **V2 (plausible, high)** and **PG-1 (plausible, high)** describe the same failure.
  - The red was not hidden from the project. It was landed deliberately (owner decision wi-557b41) and documented in commit `1f92d08` and in `runbook/handover.md:39-50`.
  - PR #35 (L6b) changes the constants to 5 arms and 90 parameters. Its CI run 37419423991 is green, workerd included. Its deployed-leg result is still owed in the record.
- **Evidence:** `crates/happenstance-cloudflare/src/event_store.rs:402,423` (re-read for this report); `experiments/durable-object-limits/README.md:154-155`; CI run 37406385109 / job 112084786585.
- **Phase 9 consequence.** Phase 9's original exit criterion was "every rule green under `workerd`" (`RUNBOOK.md:163`, unticked at `RUNBOOK.md:4564`). `runbook/README.md:90` restated it as "every rule green on `wasm32` against a `node:sqlite` shim — not `workerd`, whose run is phase 17's sibling job". **Phase 9's `done` status therefore rests on a restated criterion. The original was moved, not met, and is red on `main`.**

### 3.2 Published `happenstance-neon` does not meet ES-11 (A1, confirmed-high)

- `happenstance-neon` is published (0.2.0 onward) and by its own documentation does not satisfy ES-11 (`lib.rs:17-24`; `spec/SPECIFICATION.md:3087`; ADR-0061). `read_result_is_stable_under_concurrent_append` fails intermittently, because a read and an append are separate HTTP requests.
- ES-11 and ES-12 remain [PROVISIONAL].
- The phase-17 record that would settle them, `runbook/phases/17-breaking-window.md:162`, is unchecked. Before 1.0 it has to choose one of: a named Neon exception, a one-shot-HTTP shape that satisfies ES-11, or weakening the clause or dropping the crate.

**Flake frequency.** This reconciles V3, IMPL-4, R6 and A1, and supersedes A1's "no red run was observed" clause, which the run IDs below falsify.

> The live-Neon job is a required status check (ruleset 22926481). It failed in 4 runs:
> - 36528489181, 36596302787 and 36619371439: pushes to `main` on 2026-09-29;
> - 36775667956: a `pull_request` run on `lane/p17-busy`, 2026-09-30.
>
> Two rules were involved: `read_result_is_stable_under_concurrent_append` (suite.rs:6147) and `query_items_share_one_snapshot` (suite.rs:6250). The job has been green in every run since, including at `1f92d08`.

- The run dates and events were re-checked through the GitHub API for this report.
- The denominator is 4 failures in 54–55 non-cancelled CI runs since 2026-09-28. V3's statement says 55 and its evidence says 54; the GitHub API showed 55 non-cancelled at the time of writing, so the figure depends on the cut-off.
- `runbook/handover.md:123` tells maintainers to re-run the flake rather than chase it.

### 3.3 No external adopter (R4, confirmed-high)

- `happenstance-core`'s crates.io reverse dependencies are 6, all of them this workspace's own crates.
- Lifetime downloads run from about 101 to 1,176 per crate. The repo has 1 star, 0 forks and no non-PR issue ever filed.
- ADR-0066:319-322 admits this, and drops the rc soak's time floor for exactly this reason.
- So 1.0 would freeze an API that no outsider has used.

### 3.4 The documentation site has never deployed (docs-dx F1, confirmed-high)

- All three Pages runs on `main` failed: 36810591479, 36863802395, and 37406385128 at HEAD.
- In each, `build` succeeded and `actions/deploy-pages` failed with "Failed to create deployment (status: 404) ... Ensure GitHub Pages has been enabled".
- The fix is a repository-admin setting. The owner item for it, `runbook/phases/22-docs-site.md:84`, is unticked, as is the exit criterion at `:88`.
- PG-1 (plausible) adds that commit `1f92d08`'s message misattributes the Pages red to a nightly deprecation.

### 3.5 Phase 17 is at 3 of 10 exit criteria (plan-progress F1, confirmed-high)

- `runbook/phases/17-breaking-window.md:257-282` has 3 of 10 boxes ticked.
- Nine items have no commit or branch yet:
  - the ES-11/ES-12 fence record on Neon;
  - the ES-17 measurement or record;
  - the ADR-0022 §9 reproduction;
  - the LIST SUBQUERY guard-plan assertion;
  - ProjectionId;
  - codec sealing;
  - the SQL-seam statement type;
  - VT-6;
  - the 0.4.0 release with its semver trace.
- ES-11, ES-12 and ES-17 remain [PROVISIONAL].
- **Time used, with both measures stated:** 6 weekdays have elapsed since kickoff (2026-09-29 to 2026-10-06), of which about 4–5 were active days by session log. This is against a 25–30 day estimate.

### 3.6 Sync is in 1.0 and is the least-built piece (VF-1, IMPL-2, R1, A3, V1: all plausible-high)

These are plausible, not confirmed. They are stated with measured proxies rather than a percentage. No measured completion percentage exists. If a figure is wanted, sync is about 10–15% built **by judgement**.

- **The crate.** `happenstance-sync` is `publish = false` and describes itself as "Not yet implemented."
  - `src/` is real: port traits, identity types, the wire format and `MemorySyncPeer`, about 1,570 lines.
  - `src/` contains **no `todo!()`**. The `todo!()` stand-ins are only in `tests/real_peer_shapes.rs:110-218`.
- **Missing pieces.** There is no `SyncRunner` and no `happenstance-sync-testkit`. Neither sync crate name exists on crates.io.
- **Rules.** **0 of 35 SY rules are executable.** 32 rows are daggered, and the other 3 cite the missing testkit. All 21 FROZEN SY clauses bind design only.
- **Allowlist.** `spec_trace.rs`'s `CF36_UNDISCHARGED` allowlist covers 9 SY clauses. It does not exempt the whole SY family. V1's evidence field says otherwise; its corrected statement is used here.
- **Schedule.** Phase 13 is not started. It runs after 17, 17b and 18, so after the 0.4.0 breaking window (A3, R5). It carries 17 of the 34 freeze-by-N rows in the 1.0 ledger.

Decision D-1 (wi-40b321) put sync inside 1.0 against the plan's own recommendation.

### 3.7 Other plausible-high findings

- **R2: schedule.** The roadmap's 62–75 days leaves out phase 20 and the soak.
  - Phase 17 grew about 5–6x: from 5–8 days to 35–45 unsplit (ADR-0072:13-19), or 33–40 days across 17 and 17b.
  - R2's own caveat: under one squash-merge per lane, commit dates are a poor throughput measure, so a calendar multiplier drawn from them is speculative.
- **R3: bus factor of one.** One person authored all 86 commits in the visible history. The governance load is large (section 8.7).
- **PG-2: governance size.** About 3.5 times the published crates' source: 8.8 MB of governance prose against 2.4 MB of `src` for the seven published crates.
  - Counting the frozen `.bklg` and `.redkiln` trees, the ratio is about 16.5:1.
  - That the governance load is the main brake on throughput is **an opinion that was not measured**.
- **docs-dx F2: no projection guide.** None of the 6 guide pages in `docs/` mentions projections or read models.

---

## 4. Material medium findings

Medium findings that a summary tends to drop. Status is given for each.

| ID | Status | Finding | Evidence | What it means |
|---|---|---|---|---|
| **IMPL-5** | confirmed | `happenstance-neon` publicly exports `ProbeThenWriteStore`, a deliberately lost-update event store. It has a `pub` constructor and an `EventStore` impl, is in `pub mod event_store`, and is re-exported at the crate root. It has no `cfg(test)` and no `doc(hidden)`. | `crates/happenstance-neon/src/event_store.rs:1475`; `lib.rs:147`, `lib.rs:186` (re-read for this report) | **Release-blocking API-surface item for the 0.4.0 breaking window.** If it ships in 1.0, a known-wrong store becomes semver-promised API. Hide it, gate it, or move it to the testkit's mutants before 0.4.0. |
| **A6** | confirmed | The two-flavour `Send` derivation rests on `trait-variant = "0.1.3"`, a caret requirement. Consumers resolve without the lockfile, so the in-source shape guards see only the workspace's `--locked` resolve. `store.rs:153-155` calls this an uncaught residual risk. | `Cargo.toml:151`; `crates/happenstance-core/src/store.rs:147-157`; `.kb/open-questions/trait-variant-caret-resolves-past-the-locked-gate.md` | Deferred to 17b as additive. It sits under binding constraint 1, the core design. |
| **V5** | confirmed | No CI job runs `crates/happenstance-postgres/tests/live_projection.rs`, which tests `LivePostgresProjectionStore`, the store ADR-0063 froze the projection port against. Its tests are all `#[ignore]`. | `ci.yml` `--test` targets omit `live_projection`; `spec/SPECIFICATION.md:405` | The claim that it passes all seventeen rules against a live server has no CI record. |
| **V6** | confirmed | 34 of 152 FROZEN clauses, about 22%, have no executable check: 21 SY, 4 ES (ES-6, ES-29, ES-31, ES-38), 3 PS runner rules (PS-26, PS-28, PS-29), and 6 deliberate "none" cells. | `spec/SPECIFICATION.md` §7.2; `xtask/src/spec_trace.rs:1167-1173` | Frozen behaviour that nothing executes. |
| **A7** | plausible | Seven frozen ES/PS clauses name rules listed as `Unresolvable::Scheduled`. ES-6's rule `store_error_crosses_a_join_handle` is argued **unwritable** against today's port. | `crates/happenstance-cloudflare/src/send_shape.rs:14,95`; `.kb/open-questions/es-6-names-an-unwritable-rule.md` | A frozen clause whose rule may never exist needs a record, not a dagger. |
| **PG-4** | plausible | The SQLite busy-timeout liveness fix, 5 s to 15 s, landed 2026-09-21 in `f89e184`. It is held in unreleased 0.4.0, so **the published 0.3.2 still ships 5 s**. See section 4.1 for the measurement behind it. | `CHANGELOG.md:39-60`; `experiments/busy-timeout-margin/README.md:6-13,208` | Not cutting 0.3.3 is a recorded owner decision (wi-052920). |
| **docs-dx F4** | confirmed | The CHANGELOG's [Unreleased] section has 6 distinct BREAKING entries (`:92, :120, :137, :175, :304, :317`) and there is no 0.3 to 0.4 migration guide. Each entry carries its own migration note, which is existing practice. | `CHANGELOG.md:35-340` | An upgrader has to assemble the guide from six entries. |
| **R6 / V3** | plausible / confirmed | The required live-Neon check flaked in 4 of 54–55 non-cancelled runs, all between 09-29 and 09-30. It has been green since. Section 3.2 reconciles the figures. | Run IDs in section 3.2 | Institutionalised as "re-run, don't chase" (`handover.md:123`). |
| **R8** | unverified-low in `verified.json`; registry re-checked for this report | `happenstance-sync` and `happenstance-sync-testkit` are not reserved on crates.io, although ADR-0066 §1 promises both at 1.0. The `happenstance-ladybug` 0.0.0 reservation that ADR-0078:158 says "stands" does not exist. VF-4 (confirmed) records the ladybug half. | crates.io API, 2026-10-06: all three return "does not exist"; `xtask/src/reserve.rs:110-112` | **Risk:** the names can be squatted before 1.0. See section 8.8. |
| **A5 / IMPL-6 / R5** | confirmed / plausible / plausible | Six breaking-change candidates are still open inside the 0.4.0 window: ADR-0022 §9 (the runtime `Handle` captured at construction), ES-17, ProjectionId, codec sealing, the SQL-seam statement type, and VT-6. | `runbook/phases/17-breaking-window.md:61-136,267-282` | After 0.4.0, a break that sync uncovers late has no sanctioned outlet (R5). |
| **A4 / IMPL-3** | confirmed | The typed `Projection::apply` is synchronous (`runner.rs:95-99`), and its rustdoc argues for that shape. Accepted ADR-0074 requires an async apply handed a `Delivered<E>`, and no `Delivered` type exists. The runner is still behind `unstable-projection`. | `crates/happenstance/src/runner.rs:50-59,95-99`; `Cargo.toml:148` | Shipped rustdoc contradicts an accepted decision. Phase 18 closes it. |
| **VF-4** | confirmed | Retiring ladybug removed the workspace's only non-SQL projection adapter. | ADR-0078:104-110,146-152 | Partly offset by `outside-projection-adapter` (section 5.3). |
| **docs-dx F6** | confirmed | The Cloudflare user docs do not state the workerd query limit in user terms: at most 5 query items and 45 tags per item. The rustdoc (`src/lib.rs:191-210`) says wide queries are chunked at 400 arms and never refused. The fix is planned in lane L6b as **ADR-0079 (PR #35)**. The handover's "ADR-0083" is wrong (plan-progress F3, `ledgers.md:20`). | `crates/happenstance-cloudflare/README.md:113-122` | Users would expect far more capacity than the real runtime serves. |
| **IMPL-7 / V7** | confirmed / plausible | The proptest model rule runs only on memory and SQLite. Postgres is tested only at 17.10. The benchmark job only compiles the harness. | `crates/happenstance-testkit/src/model.rs:693-697`; `ci.yml` | There is no generative coverage on the non-serialising axis. |

### 4.1 PG-4 is a measured claim, not only a stated one

`experiments/busy-timeout-margin/README.md` gives the measurement:
- "In the configuration `cargo xtask ci` actually runs, the worst per-contender wait is **3,628 ms of the 5,000 ms budget** measured directly, and **≈5.5 s** under the busy handler that actually ships — and **one launch in seven turns the concurrency suite red today**" (`:6-11`; also `:208`).
- The CHANGELOG entry (`CHANGELOG.md:39-60`) gives a worst case of 7 launches in 8 at the old value.
- Both sources call it a liveness defect, not unsoundness: a red test or a refused append, not a wrong result.
- **So the published 0.3.2 `happenstance-sqlite` has a measured liveness margin problem under contention.**

**Recommendation.** Weigh a **0.3.3 patch carrying only the 15 s timeout** against wi-052920's "No 0.3.3". The patch is additive and small. 0.4.0 is at least about two weeks away (section 7).

---

## 5. Gate and CI (measured)

### 5.1 Local gate, verbatim

The following is quoted verbatim from `gate-summary.md` lines 2–6:

> - `cargo xtask ci` run 1: all REQUIRED steps up to "no accepted decision's body has changed" passed; that step failed ONLY because this cloud clone is shallow (environmental). 9m34s wall on 4 CPUs.
> - Run 2 with HS_KB_BASE=origin/main: **all checks passed, exit 0**. 31 required steps green (fmt, clippy -D warnings, tests, phase proof artefacts, 7 wasm32 steps incl. wasm32 run of conformance rules, docs x3 configs, spec-trace, conformance-suite lints, constitution consistency+examples compile, narrative docs checks, packaging of 7 publishable crates, workflow pinning, lint-kb).
> - SKIPPED (tool absent in this container, not failures): cargo-hack feature powerset, wasm32 feature powerset, cargo-deny licences/advisories, two nightly docs.rs builds.
> - Tests: 2,965 passed, 0 failed, 288 ignored (summed over `test result:` lines; panics in log are expected should_panic tests). Ignored include 24 tests needing live Postgres (Docker) and 12 needing a live Neon endpoint — NOT exercised here. Deployed-Cloudflare workerd leg also not exercised.
> - Examples run: course-subscriptions, transfers-on-sqlite, rebuilding-read-models, handles-and-quotas, telemetry-across-codecs — all exit 0. (tickets-over-http and outside-projection-adapter not run as binaries.)

### 5.2 Caveats on the local result

- **Run 1 failed lint-kb** only because the clone is shallow. Run 2 passed with `HS_KB_BASE=origin/main`.
- **288 ignored tests in total.** Of these, 36 are live-database tests by `#[ignore]` reason: 24 Postgres and 12 Neon.
  - V4 (plausible) counts by test target: **277 integration tests are ignored that only CI runs**. They are postgres_conformance 108, projection 21, live_projection 20, poll_shape 1, neon_conformance 108 and neon_projection 19.
  - The 36 is a count of reason annotations. The 277 is a count of tests carrying them. A green local gate says nothing about live-database behaviour.
- **Skipped steps:** cargo-hack powerset, wasm32 powerset, cargo-deny, and two nightly docs.rs builds.
- **Not run locally:** the deployed-Cloudflare workerd leg, and the local-workerd leg, which is a CI sibling job and not a gate step.
- **The MSRV job proves nothing while the MSRV equals the toolchain pin.** Both are 1.97.1, which CLAUDE.md itself concedes.

### 5.3 All seven examples were exercised

`gate-summary.md`'s "not run as binaries" is accurate but incomplete. The gate ran the other two examples' integration tests (`gate2.log`):

- **tickets-over-http.** `tests/two_processes.rs` ran 2 tests, both ok: `nothing_is_oversold_under_contention` and `two_processes_share_one_file` (`gate2.log:5703-5713`, `9600-9609`).
- **outside-projection-adapter.** 4 test targets ran (`gate2.log:5537-5619`): capability_skip, conformance, discrimination and manifest. `outside_projection_conformance` ran **18 projection rules**.

So 5 examples ran as binaries and 2 through their integration tests in the gate.

`outside-projection-adapter` is evidence that **the projection extension surface can be implemented from the published documentation alone**, in a crate where the orphan rule and the non-dev graph behave as they do for a stranger. That partly offsets VF-4's loss of the non-SQL projection adapter.

There is a caveat (docs-dx F7). It was written inside the project, so it is not outside-reader evidence.

### 5.4 CI at HEAD: run 37406385109 on `1f92d08`

| Job | Result |
|---|---|
| gate (ubuntu, macOS, Windows) | green |
| conformance against live Postgres (required) | green, 108/108 |
| conformance against a live Neon endpoint (required) | green, 108/108 |
| MSRV | green (proves nothing while MSRV = pin) |
| benchmark crate still compiles | green (compile only) |
| **conformance under workerd, local and deployed** (not required) | **red**: job 112084786585, VT-23 |
| semver, security advisories, backlog | skipped |
| Pages (separate workflow, run 37406385128) | **red**: deploy 404, Pages not enabled |

### 5.5 Verification gap: no recorded registry-only install test

`scripts/stranger-install-smoke.sh` (DR-8) is the only check that the published crates compile for a consumer who has only the registry.
- Inside the workspace, crates resolve by path. That ignores the `include` list, files left out of the `.crate`, and feature mistakes.
- It is deliberately not a gate or CI step.
- `RUNBOOK.md:5209` still reads `- [ ] **Run scripts/stranger-install-smoke.sh**`.
- No input records it being run for 0.2.0, 0.3.0, 0.3.1 or 0.3.2.
- ADR-0066 §8's rc condition "two examples build against the registry rc" is the same kind of check.

**Recommendation.** Run the script against 0.3.2 now and record the result. Make it a named release step for 0.4.0 and every rc.

---

## 6. Phase coverage: every phase

Sources: `runbook/README.md:80-110`, map-plan §2.1, and the phase files.
- "Proof checked" means an auditor located the README's named proof artefact in the tree, the registry or CI. It does not mean the artefact was re-executed beyond the gate.
- "Estimate" is this audit's independent figure in working days, or "off path".

| Phase | Status (README) | Exit boxes | Proof artefact | Proof checked? | Estimate |
|---|---|---|---|---|---|
| 0 Ground clear | done | 6/6 | `.crate` with licences + README; spec-trace failing on a broken clause | yes (packaging and spec-trace are gate steps, green) | done |
| 1 `!Send` proof | done | 5/5 | both flavours; rules green on `!Send` wasm32 | yes (`memory_conformance_wasm.rs`; wasm32 gate steps green) | done |
| 2 Instrument portfolio | done | 6/6 | six crates on real targets | yes, by history (ladybug since retired) | done |
| 3 Suite becomes an instrument | done | 7/7 | mutant registry | yes (`mutation_coverage`, meta-test) | done |
| 4 Freeze the contract | done | 12/12 | `frozen_signatures.rs` + `compile_fail` doctest | yes | done |
| 5 Freeze the wire format | done | 7/7 | two `wire.rs` | yes (both ran in the gate) | done |
| 6 Freeze `ProjectionStore` | done | **0/5** in RUNBOOK.md (`:4086-4092`) | `CheckpointOnlyStore` failing; two batch shapes passing | yes; the boxes are stale | done |
| 7 Typed layer + example | done | **2/8** (`:4193-4225`) | `trybuild` compile-fail case | yes (`examples/course-subscriptions/tests/ui.rs`) | done |
| 8 `happenstance-sqlite` | done | **0/4** (`:4425-4429`) | concurrency at 64; write survives reopen | partly (crate published; gate green; `CONTENDERS = 64`) | done |
| 9 Cloudflare DO | done | **1/4** (`:4564-4572`) | **restated**: shim, not workerd (`README.md:90` vs `RUNBOOK.md:163`) | shim yes; **original workerd criterion: no, red on `main`** | done on a restated criterion; remainder in 17 |
| 10a Postgres event store | done | 5/5 (shared) | concurrency on a non-serialising store | yes (live-postgres CI green) | done |
| 10b Postgres projections + Neon | done | (shared) | no `todo!()`; `LivePostgresProjectionStore` passing 17 | type exists (`live_projection_store.rs:151`); **its 17-rule run has no CI record (V5)** | done |
| 11 Ladybug | done, then retired (ADR-0078) | 3/3 | projection suite on a non-SQL batch | historical; crate excluded, not built | done (retired) |
| 12 Publish 0.2.0 | done | **2/4** (`:5264-5280`) | seven crates on crates.io + semver baseline | yes (registry max 0.3.2; tags v0.2.0–v0.3.2) | done |
| 15 Reconcile the record | done | 6/6 | status lint | yes (`xtask/src/lints.rs:2205,4253,4263`) | done |
| 16 Define 1.0 | done | 5/5 | ADR-0066 + 44 dispositions | yes (`ledgers.md:270-315`, 44 rows) | done |
| **17** Breaking window, 0.4.0 | in progress | **3/10** | 0.4.0 released + semver trace | **no** (no v0.4.0; registry max 0.3.2) | **9–18** remaining |
| 17b After the window | not started | 0/7 | VT-14, VT-30, ES-7 frozen; new jobs seen failing | no | **6–11** |
| 18 Typed runner | not started | 0/6 | rebuild example with no unstable feature | no (`unstable-projection` still enabled) | **8–15** |
| 13 Sync + testkit | not started | 0/9 | one suite green on three peers | no (no sync-testkit) | **22–40** |
| 14 Retention | not started | 1/5 | completeness instrument | no | **6–11** |
| 19a SQLite wasm32 skeleton | not started | 0/1 (shared) | wasm32 skeleton in the gate | no | off path (3–5) |
| 19b SQLite wasm32 adapter | not started | (shared) | both suites on wasm32 | no | off path (8–14) |
| 20 Docs that teach | in progress (nothing recorded, F5) | 0/4 | DoD re-observed from a clean checkout | no | **10–22** + reader wait |
| 22 Docs site | in progress | 0/4 | site deployed from `main` | **no**: built, never deployed (docs-dx F1) | off path (1–3 + owner toggle) |
| 21 1.0.0 | not started | 0/6 | promised crates at 1.0.0 + clause audit | no | **5–10** + 1–3 week soak |

**Notes on the table.**
- **Phase 9's `done` status rests on a restated criterion.** The original, every rule under `workerd` (`RUNBOOK.md:163,4564`), was moved to phase 17 rather than met. It is red on `main` until PR #35 merges.
- **Phases 6, 7, 8, 9 and 12** have unticked exit boxes in the frozen RUNBOOK.md (`:4086-4092`, `:4193-4225`, `:4425-4429`, `:4564-4572`, `:5264-5280`). For the artefacts spot-checked, the work exists and the boxes are stale. Phase 9 is the exception.
- **Phase 22 has no row or node in `runbook/roadmap.md`** (plan-progress F4). It is off the 1.0 path, because phase 21 does not depend on it.
- **Phase 20 is on the 1.0 path** (`README.md:110`) but has no estimate in the plan (`phases/20-docs-that-teach.md:51`).

---

## 7. Effort and calendar to 1.0.0

Full method in `estimate.md`. The arithmetic below is corrected from earlier drafts.

| On-path phase | Low | High |
|---|---|---|
| 17 remainder | 9 | 18 |
| 17b | 6 | 11 |
| 18 | 8 | 15 |
| 13 | 22 | 40 |
| 14 | 6 | 11 |
| 20 | 10 | 22 |
| 21 (effort) | 5 | 10 |
| **Total** | **66** | **127** |

- **Central figure: about 90 working days.**
- **Roadmap comparison.** The roadmap's own remaining path, without phase 20 and without the soak, is 52–64 days.
- **Biggest single error: phase 13.** Its 12-day figure is the pre-phase-16 number, and phase 16 then added scope to it.
- **Calibration signal.** Phase 17 grew about 5–6x when re-estimated: from 5–8 days to 35–45 unsplit (ADR-0072:13-19), or 33–40 days across 17 and 17b.

**Calendar.**
- These dates rest on a **cadence assumption, not a measurement**. R2's caveat applies: under one squash-merge per lane, git commit dates are a poor throughput measure, and a multiplier drawn from them is speculative.
- Dates count weekdays from 2026-10-06, are rounded, and ignore holidays.

| Cadence assumption | Low (66 d) | Central (~90 d) | High (127 d) |
|---|---|---|---|
| Sprint pace (~5 active days/week, as since 09-28) | ~2027-01-06 | ~2027-02-09 | ~2027-04-01 |
| Long-run pace (~3.1 days/week) | ~2027-03-04 | ~2027-04-27 | ~2027-07-20 |
| Add the rc soak | +1–3 weeks | +1–3 weeks | +1–3 weeks |

- **Most likely window, under these cadence assumptions: late February to late May 2027.** Treat it as an assumption-driven range.
- **0.4.0:** about late October 2026 at sprint pace (9–18 days lands 2026-10-19 to 10-30), or late October to mid-November at long-run pace.

**Scope levers** (`estimate.md` §5):
1. **Ship 1.0 without sync**, revisiting D-1. Saves phase 13.
2. **Renew ES-39 and defer phase 14.**
3. **Take phase 18's fallbacks**, PS-30 outside 1.0 and PS-18 renewed. Saves 2–6 days. The cost of dropping the fan-out runner is measured in `experiments/polling-cost/README.md:11-14`: the polling runner delivers each event once per view, "**delivery amplification of 32.00 at 32 views**, exactly the fan-out, with no economy of scale anywhere in the range" (1.00 where the views do not overlap). Without PS-30, an application with N overlapping views pays N times the deliveries.
4. **Narrow phase 20** to the outside-reader pass that 21 needs. Saves 5–12 days.

Levers 1, 2 and 4 together bring the path to **about 34–64 days**:
- late November 2026 to early January 2027 at sprint pace;
- late December 2026 to late February 2027 at long-run pace.

---

## 8. Findings by dimension

Each dimension lists its headline, then its findings (ID, severity, status). Unverified-low findings are listed only by ID here; see Appendix A.

### 8.1 Vision-fit (amber, 55)

The contract and suite are the delivered differentiator. Local-first sync is mostly paper, and the edge adapter fails its real runtime.

- VF-1 high, plausible: sync least built (section 3.6).
- VF-2 high, confirmed (section 3.1).
- VF-3 medium, plausible: the words "local-first" and "offline" are absent from the README, `docs/` and the crate READMEs. Two docs.rs crate roots still use "local-first".
- VF-4 medium, confirmed: ladybug retired; reservation absent (section 4).
- VF-5 medium, confirmed: there is no browser or WASI store. 19a and 19b are off the 1.0 path, so 1.0 sync could ship with no browser-capable spoke.
- VF-6 medium, confirmed: the "batteries" claim in `crates/happenstance/Cargo.toml:3` is unmet at the projection layer. None of the **6 guide pages plus a README (649 lines in all)** in `docs/` mentions projections.
- VF-7 medium, confirmed: the 1.0 path is long and growing, and the trusted/delight evidence (HS-P0024) is unbuilt.
- Unverified-low: VF-8, VF-9.

**Benchmarks.** VF-9 is unverified-low; its claim is that performance is presented honestly as "no claim". It cites `benchmarks/results/GRADES.md` as the measured record.
- That record is **one run on 2026-09-10 at `54044ac`** (`GRADES.md:3`). It is about four weeks old.
- It predates the busy-timeout change (`f89e184`, 09-21), VT-10 (09-29), ADR-0074 (09-30) and `AppendError::Busy` (`159ed70`, 09-30).
- The CI benchmark job only compiles the harness.
- The measurement host's tuning scripts are in `ops/host/` (`00-system.sh`, `cpu-tuning.sh`, `bench.sh` and others). No assessor examined them.
- **Recommendation:** re-run the benchmark record on the current tree before the 0.4.0 or 1.0 docs quote it.

### 8.2 Plan-progress (amber, 55)

Phase 17 is at 3/10. Across everything left before 1.0, 4 of 47 exit criteria are met.

- F1 high, confirmed (section 3.5).
- F2 medium, confirmed: the workerd job fails on `main` by design. PR #35 has 20 check runs, none failed.
- F3 medium, confirmed: the handover is wrong on three points: PR #34 described as open, "ADR-0083" where it should be 0079, and a tracked file listed as untracked. Its staleness rule cannot catch this.
- F4 medium, plausible: the roadmap has no phase 22 row, and its 62–75 days excludes phase 20 and the soak.
- F5 medium, plausible: phase 20 is marked "in progress", but nothing has been recorded under it since 09-28.
- F6 medium, confirmed: the critical path runs through phase 13. SY-27 and SY-28 need a phase-14 instrument, a sequencing hazard the runbook itself flags (`ledgers.md:263-268`).
- **History note (from PG-6, plausible).** The local clone is shallow. History before 09-07 exists on GitHub: `673dcc5`'s parent is `b0d9e67`, re-checked with `git cat-file` for this report. It was not read. The phase 0–12 pace comes from RUNBOOK.md's dated session logs. This replaces the estimate's earlier "squashed import" wording and F10's "cannot be audited from git".
- Unverified-low: F7, F8, F9, F10.

### 8.3 Architecture (amber, 70)

All binding constraints hold:
- no `#[async_trait]`;
- `serde` is not a default feature of `happenstance-core`;
- `read` is non-async and returns its stream at the top level, with both guard tests present (`memory.rs:628,657`);
- no adapter depends on another adapter.

The ports are small and were tested against storage shapes that differ.

- A1 high, confirmed (section 3.2).
- A2 high, confirmed (section 3.1).
- A3 high, plausible: the replication port is frozen on design only, and its build comes after the 0.4.0 window. ADR-0073 partly mitigates this, and there is no CF-25 violation.
- A4 medium, confirmed: the typed `Projection::apply` contradicts accepted ADR-0074.
- A5 medium, confirmed: six breaking candidates are open.
- A6 medium, confirmed: the caret-pinned `trait-variant`.
- A7 medium, plausible: ES-6's rule may be unwritable.
- Unverified-low: A8, A9.

### 8.4 Implementation (amber, 65)

- Core, the typed layer and the testkit: about 30k `src` lines with 0 `unsafe`, 0 `todo!()` and 0 production `unwrap`.
- Testkit rule counts: 95 event-store, 17 projection, 6 concurrency and 1 model rule, with 84 + 18 named mutants.

Findings:
- IMPL-1 high, confirmed (section 3.1).
- IMPL-2 high, plausible (section 3.6).
- IMPL-3 medium, confirmed (= A4).
- IMPL-4 medium, plausible: Neon's ES-11 exception. Its "never failed in the 15 most recent runs" is true for that window only; see the reconciled figure in section 3.2.
- IMPL-5 medium, confirmed: `ProbeThenWriteStore` is public, and it is release-blocking for 0.4.0 (section 4).
- IMPL-6 medium, plausible: ADR-0022 §9 is unreproduced.
- IMPL-7 medium, confirmed: real-database coverage gaps.
- Unverified-low: IMPL-8, IMPL-9.

### 8.5 Verification (amber, 65)

- V1 high, plausible: the SY contract has no executable checks. Only the corrected statement is used here: there is no `todo!()` in `src/`, and the allowlist covers 9 SY clauses.
- V2 high, plausible (section 3.1).
- V3 medium, confirmed (section 3.2).
- V4 medium, plausible: 277 live tests run only in CI.
- V5 medium, confirmed: `live_projection.rs` is never run in CI.
- V6 medium, confirmed: 34 of 152 FROZEN clauses have no executable check.
- V7 medium, plausible: generative checks cover only stores that serialise their writers.
- Section 5.5 adds a gap no assessor raised: there is no recorded registry-only install test.
- Unverified-low: V8, V9.

### 8.6 Docs and developer experience (amber, 40)

- F1 high, confirmed: the site has never deployed (section 3.4).
- F2 high, plausible: there is no projection guide.
- F3 medium, confirmed: five planned how-tos, the bridge page and site search are all absent. `docs/` holds 7 files, a README plus 6 guide pages.
- F4 medium, confirmed: there is no migration guide for the 6 BREAKING entries.
- F5 medium, plausible: the `happenstance` and `happenstance-core` crates.io READMEs are stale.
- F6 medium, confirmed: the workerd limits are not stated in user terms. The fix is planned as ADR-0079 in PR #35.
- F7 medium, confirmed: no outside reader has validated the docs.
- Unverified-low: F8, F9.

Additional DX items no assessment raised:

- **A rustc ICE that third-party adapter authors can hit (medium; new in this report).**
  - **What happens.** `experiments/rustc-ice-gat-foreign-trait/README.md:1-25,117-145` records an internal compiler error that "this architecture reaches by construction". An outside author who writes a projection store or test fixture that **borrows** its database, so that the store type carries a lifetime, gets a compiler panic instead of the lifetime error they should see.
  - **Scope.** It reproduces on 1.85.1, on 1.97.1 (the pinned toolchain) and on 1.99.0-nightly. It is not fixed. The upstream issue, rust-lang/rust#158983, is open.
  - **What the docs already do.** The port has been reshaped so the obvious path avoids it: `ProjectionStore::Batch` is no longer a GAT, and the testkit's `Fixture::Store` is an owned type.
    - `crates/happenstance-core/src/projection.rs:76-85` notes in module docs that a store carrying a lifetime "made rustc 1.97.1 *ICE*".
    - `crates/happenstance-testkit/src/contract.rs:108-118,600-610` explains the owned-`Store` choice and links the experiment.
    - `docs/README.md:33` lists "a rustc ICE" among the measurements.
  - **What is missing.** No guide page in `docs/`, and nothing on the `ProjectionStore` trait's own "Implementing it" section that a stranger reads first, says in plain terms: "do not make your store borrow its connection; if rustc panics, this is why".
  - **Why it matters.** Phases 20 and 21 depend on outside authors.
  - **Recommendation:** add that sentence where an adapter author lands, and in the planned `pass-the-conformance-suite` how-to.
- **`SECURITY.md`'s scope must grow from 7 crates to 9 at phase 21.** It lists the seven published crates (`SECURITY.md:24-32`). ADR-0066 §1 promises nine at 1.0, adding `happenstance-sync` and `happenstance-sync-testkit`, and `runbook/phases/21-one-point-oh.md:63` carries the rewrite.
- **Internal records in the repository root.** These are `HANDOVER.md` (420 lines, describing the 0.2.0 state), `REMEDIATION-HANDOVER.md` (376) and `SESSION-DECISIONS-0.2.0.md` (979). They sit beside the user-facing README. This is docs-dx F8, unverified-low. If F8 is accepted, move them under `references/`.
- **Not assessed:** `CONTRIBUTING.md` (369 lines) and `CODE_OF_CONDUCT.md` (136 lines). No assessor read them for an outside contributor's path (section 11).

### 8.7 Process and governance (amber, 65)

**Headline, corrected.** The release and CI machinery works. `main` is red on two workflows (workerd and Pages). A liveness fix has been held for 15 days (PG-4). The governance prose is **about 3.5 times the published crates' source** (8.8 MB against 2.4 MB). Whether that load limits throughput was not measured.

- PG-1 high, plausible: `main` is red on two workflows.
- PG-2 high, plausible: the governance size ratio, 3.6:1 by PG-2's figures. R3's figures are about 8.5 MB against about 2.5 MB of `src`, or about 1.8x against all crate `.rs` files.
- PG-3 medium, plausible: citations by line number carry a recurring repointing cost.
- PG-4 medium, plausible (section 4.1).
- PG-5 medium, plausible: plan-of-record documents drift.
- PG-6 medium, plausible: the estimates are lower bounds.
- Unverified-low: PG-7, PG-8, PG-9.

**Supply chain (low; new in this report).**
- `harness/workerd/package-lock.json` is 3,593 lines and is installed with `npm ci` in CI (`ci.yml:907`).
- No advisory scan covers the npm tree:
  - `cargo-deny` covers only Cargo;
  - `.github/` holds `ci.yml`, `pages.yml`, issue templates and a PR template;
  - there is no `dependabot.yml` and no `npm audit` step.
- `cargo-deny` itself was skipped in the local gate (tool absent). It runs in CI's weekly advisories job.
- **Credit:** the Cloudflare credentials are scoped to the individual steps that need them (`ci.yml:968-976`). The comment there names `npm ci`'s install scripts as the reason, so a compromised npm dependency cannot read them during install.

### 8.8 Risk (amber, 40)

- R1 high, plausible: sync is inside 1.0.
- R2 high, plausible: the schedule. The multiplier is about 5–6x on the basis given in section 7, and R2's calendar caveat applies.
- R3 high, plausible: bus factor of one.
- R4 high, confirmed: no external adopter.
- R5 medium, plausible: 0.4.0 is the last breaking window, but sync, the runner and retention all come after it.
- R6 medium, plausible: the Neon flake (section 3.2).
- R7: unverified-low.
- R8 (unverified-low; registry re-checked): unclaimed names.

**Added risks:**
- **Name squatting before 1.0 (R8).**
  - `happenstance-sync` and `happenstance-sync-testkit` are promised by ADR-0066 but unreserved.
  - `happenstance-ladybug`'s recorded reservation does not exist.
  - Likelihood is low and the impact is a forced rename of a promised crate. Reserving the names is cheap.
- **The rustc ICE** for outside adapter authors (section 8.6).
- **The published 0.3.2 SQLite busy timeout** (section 4.1).

---

## 9. Inventory: what was checked for each component

Legend:
- **B**: built in the gate.
- **T**: tests ran in the local gate.
- **L**: live-tested in CI.
- **R**: run as a binary.
- **N**: not examined.

| Component | Kind | Checked |
|---|---|---|
| `happenstance-core` | published 0.3.2 | B, T (unit, memory conformance, wasm32 build, docs x3 configurations) |
| `happenstance` | published 0.3.2 | B, T (typed layer; runner behind `unstable-projection`) |
| `happenstance-testkit` | published 0.3.2 | B, T (mutation-coverage meta-tests; conformance rules on wasm32) |
| `happenstance-sqlite` | published 0.3.2 | B, T (full suite); CI gate on 3 OSes |
| `happenstance-cloudflare` | published 0.3.2 | B, T over a `node:sqlite` shim on wasm32; L under workerd, **red** |
| `happenstance-postgres` | published 0.3.2 | B, T (unit and non-gated only); L (live-postgres green, 108/108); `live_projection.rs` **not run anywhere** (V5) |
| `happenstance-neon` | published 0.3.2 | B (host + wasm32), T (unit only); L (live-neon green at HEAD; 4 flaky runs) |
| `happenstance-sync` | `publish = false` (unfinished) | B, T (wire.rs, probe tests, `real_peer_shapes.rs` type-checks) |
| `happenstance-ladybug` | retired, excluded from the workspace | N (records only; not built) |
| `examples/course-subscriptions` | example | B, T (trybuild), R |
| `examples/transfers-on-sqlite` | example | B, R |
| `examples/rebuilding-read-models` | example | B, R |
| `examples/handles-and-quotas` | example | B, R (gate-summary only; no assessor reviewed its design) |
| `examples/telemetry-across-codecs` | example | B, R (gate-summary only; no assessor reviewed its design) |
| `examples/tickets-over-http` | example (lib target) | B, T (`two_processes.rs`, 2 tests ok); not R |
| `examples/outside-projection-adapter` | example (falsifier) | B, T (4 targets; 18 projection rules); not R |
| `harness/workerd` | workspace member; npm lockfile | B, T (Rust unit tests in the gate); L (CI workerd job, red); not run locally |
| `xtask` | workspace member, about 33k lines | B, T (unit tests); run (it is the gate). Examined only as governance load (PG-2, R3), not reviewed for correctness |
| `benchmarks/` | not a workspace member | CI compiles only; not run; record dated 2026-09-10 (section 8.1) |

---

## 10. Stale-record items

These are listed together so that one pass can fix them. Each is marked by how it was established.

| Record | Says | Is | Status |
|---|---|---|---|
| `CLAUDE.md:404` | "202 numbered clauses" | **203** (`SPECIFICATION.md:228-236`, `:9791-9800`; re-read for this report) | confirmed (PG-5, R3) |
| `runbook/handover.md:15-16,40` | PR #34 open, not merged | merged as HEAD `1f92d08` | confirmed (F3) |
| `runbook/handover.md:68` | "write ADR-0083" | 0079 (`ledgers.md:20`; PR #35) | confirmed (F3) |
| `runbook/handover.md:86` | `phase-15-afk-prompt.md` untracked | tracked | confirmed (F3) |
| `spec/SPECIFICATION.md:404,409` | "89" / "eighty-nine" rules | 95 in the registry | re-read for this report; listed as unverified-low A9/V8 |
| `spec/SPECIFICATION.md:9775,9782` | ES 76% frozen; PS 51% frozen "because it has no implementation at all" | ES 35/43 = 81%; PS 26/38 = 68% (generated §7.1) | re-read for this report; listed as unverified-low A9 |
| `CLAUDE.md:11` | "a five-line facade" | about 5,027 src lines | unverified-low (A9, IMPL-8) |
| `CLAUDE.md:133` | "101 of 101" | the suite is now 95+6+1; live Postgres runs 108 | unverified-low (IMPL-8) |
| `crates/happenstance/README.md:11` | "Four ship at `0.2.0`" | seven published | plausible (docs-dx F5) |
| `crates/happenstance-core/README.md:10-14` | five adapters including Ladybug | Ladybug retired | plausible (docs-dx F5) |
| `runbook/log.md:13` | newest entry 2026-09-30 | L6a and the #34 merge are unlogged there | plausible (PG-5) |
| ADR-0078:158, `xtask/src/reserve.rs` | ladybug 0.0.0 reservation stands | no such crate on crates.io | confirmed (VF-4) |
| Commit `1f92d08` message | Pages red because of a nightly deprecation | Pages not enabled (deploy 404) | plausible (PG-1) |

---

## 11. Sources read, and sources not read

**Read by the assessors:**
- `runbook/` and its phase files, and RUNBOOK.md (phases 0–12);
- `spec/SPECIFICATION.md` §1.3 and §7, and `spec/E2E-CASES.md`;
- the crate sources and manifests, `.kb/decisions` and `.kb/open-questions`;
- `references/adr/` as cited;
- `docs/`, the root README and the crate READMEs;
- `.github/workflows/ci.yml` and `pages.yml`;
- the CHANGELOG;
- the GitHub Actions and REST APIs, and the crates.io API;
- the measured gate logs, `gate.log` and `gate2.log`.

**Experiments.**
- Five of `experiments/`' 17 directories were cited by assessors: append-condition, durable-object-limits, position-visibility, provided-method-spike and wire-format.
- This report additionally read three, folded in above:
  - `busy-timeout-margin` (section 4.1);
  - `polling-cost` (section 7, lever 3);
  - `rustc-ice-gat-foreign-trait` (section 8.6).

**Not read by any assessor or by this report:**
- The other 9 experiment directories. Three matter most:
  - `apply-shape` is ADR-0074's evidence base;
  - `suite-against-wrong-adapters` and `gate-vacuity` bear on how strong the suite is.
  - The other six are correlated-exists-guard, event-clone-allocations, ladybug-driver-probes, live-handle-projection-batch, one-connection-latency and shipped-append-condition-sql.
- `ops/host/`, the measurement host's tuning scripts.
- `CONTRIBUTING.md` (369 lines) and `CODE_OF_CONDUCT.md` (136 lines).
- The bodies of `references/evaluation/` (the fourteen reviews) and `references/scenarios/`.
- `standards/rust/` beyond its router.
- `site/` beyond its config.
- `xtask/` source, except for line counts and the specific lints cited.
- Git history before 2026-09-07. It exists on GitHub (parent `b0d9e67`) but the local clone is shallow.

---

## Appendix A: Unverified-low findings

These 19 findings were not checked by a verifier. Their numbers are as the assessors reported them and **are hedged here**. None supports a rating or a headline above, except where this report re-checked a fact itself, which is noted.

| ID | Sev. | Claim (hedged) |
|---|---|---|
| VF-8 | low | Crate READMEs and the handover reportedly describe superseded states. The README's "five adapters" reportedly counts the retired Ladybug. |
| VF-9 | info | "Fast" was withdrawn in favour of a measured benchmark record. The record is dated 2026-09-10 (section 8.1). |
| F7 | low | `log.md` reportedly stops at 2026-09-30, and phase 22's session log reportedly omits the #29 merge. |
| F8 (plan) | low | Docs site built, not deployed. The Pages URL reportedly returned 404 (consistent with confirmed docs-dx F1). |
| F9 (plan) | low | Monolith exit boxes unticked for phases 6–9 and 12 (the counts match map-plan §2.2; used in section 6). |
| F10 | info | Reported commits by ISO week: W37 61, W38 1, W39 2, W40 21, W41 1. Its "3–5x" and "cannot be audited from git" are superseded by section 7 and PG-6. |
| A8 | low | Conformance depth reportedly uneven on the real-database adapters (overlaps confirmed V5 and IMPL-7). |
| A9 | low | Hand-written spec maturity prose contradicts the generated table. The specific lines were re-read for this report (section 10). |
| IMPL-8 | low | Stale code comments and CLAUDE.md figures ("five-line facade", "101 of 101"). |
| IMPL-9 | low | No Cloudflare projection store; no `DomainEvent` derive; no command-loop backoff. |
| V8 | low | `tests/e2e/` reportedly does not exist. E2E-CASES carries no per-case status. Spec says "89" rules (re-read). |
| V9 | low | No crash-fault, performance or platform-lifecycle evidence. One Windows SQLite flake reported (run 36962066186). |
| F8 (docs) | low | Front-door prose is reportedly dense, and the root holds internal record files (line counts re-checked in section 8.6). |
| F9 (docs) | low | No example has a README. All seven examples are `publish = false` (re-checked). |
| PG-7 | low | Reportedly about 47 open questions against 31 resolved. "About 20" reportedly concern governance; that count is a title-based judgement. |
| PG-8 | low | Reportedly the scheduled CI was red in 3 of the last 5 runs. The backlog job is `if: false`. MSRV is redundant while equal to the pin. |
| PG-9 | info | Reportedly about 12 of 31 gate steps check prose rather than behaviour. |
| R7 | low | Cloudflare's real-runtime limits sit below the spec floor (overlaps confirmed VF-2, A2 and IMPL-1). |
| R8 | low | Sync names unclaimed and ladybug reservation absent. **The registry facts were re-checked for this report on 2026-10-06.** |

## Appendix B: Reconciliations against `verified.json`

Where verified findings disagree with each other or with their own evidence fields, this report uses the following:

- **Neon flake:** the single sentence in section 3.2. It supersedes A1's "no red run was observed", IMPL-4's 15-run window, R6's push-only count, and V3's 55-versus-54.
- **V1:** the corrected statement, not the evidence field. There is no `todo!()` in `happenstance-sync/src`, and the allowlist covers 9 SY clauses.
- **docs-dx F6:** "ADR-0079 (PR #35)", not "ADR-0083".
- **Phase 17 growth:** about 5–6x, on the stated basis. It replaces the "4–5x" in `estimate.md`, the "slipped 4–5x" in R2's title, and the "grew 3-5x" in F10's title.
- **Sync percentage:** "by judgement about 10–15%". It never appears in a table or headline. The VF-1 and IMPL-2 titles overstate it.
- **Governance ratio:** about 3.5x. It replaces the process-governance headline's "about five times".
- **Docs page count:** 6 guide pages plus a README, 649 lines in all. It replaces "seven narrative pages" and "7 guide pages".
- **Estimate totals:** 66–127. They replace 67 and 128, along with the calendar rows and the lever totals derived from those figures.