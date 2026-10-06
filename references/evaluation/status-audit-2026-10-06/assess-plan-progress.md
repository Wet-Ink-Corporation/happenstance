# Assessment: plan-progress

Commit `1f92d08` (== origin/main), audited 2026-10-06. Read-only. The maps used were plan, history and spec, plus gate-summary.md. I re-checked their load-bearing claims in the worktree, on GitHub (PRs, tags, CI jobs), on crates.io and against the Pages URL.

**Rating: AMBER. Completeness is about 55% of the way to 1.0** by the plan's own effort weights:
- phases 0–12 are about 82 estimated days, and they are done;
- the 0.3.2→1.0 roadmap is 62–75 days, of which about 20% is spent;
- phase 20 is unestimated and not counted.

**Headline.** The project is mid-way through phase 17, the breaking window that produces 0.4.0. It has met 3 of that phase's 10 exit criteria. Meeting exit criteria is what marks a phase done. It has 4 of 47 exit criteria met across everything still on the 1.0 path. The plan of record is structurally sound and lint-checked, but the hand-written parts (handover, log, roadmap) trail the code by one to several merges.

---

## Phase scoreboard

These are the README status rows (`runbook/README.md:78-110`).

| State | Rows |
|---|---|
| done (16) | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10a, 10b, 11 (since retired, ADR-0078), 12, 15, 16 |
| in progress (3) | 17 (3/10 exit), 20 (0/4, session log empty), 22 (0/4, built but not deployed) |
| not started (7) | 17b (0/7), 18 (0/6), 13 (0/9), 14 (1/5), 19a/19b (0/1 shared), 21 (0/6) |

- The exit-criteria counts come from awk over `runbook/phases/*.md`, between `**Exit criteria**` and the next heading.
- On the remaining 1.0 path (17, 17b, 18, 13, 14, 20, 21), **4 of 47** exit criteria are ticked.
- Releases are real: GitHub tags `v0.2.0`, `v0.3.0`, `v0.3.1` and `v0.3.2` exist on origin. crates.io has `happenstance-core` at max_stable `0.3.2`, updated 2026-09-21. No `v0.4.0` exists yet, though `Cargo.toml:24` already reads `0.4.0` and `CHANGELOG.md:35` is `## [Unreleased]`.

## What documents claim vs what is shown

| Claim | Source | Shown |
|---|---|---|
| "PR #34 … Not merged" | `runbook/handover.md:15-16,40` | **False.** #34 merged 2026-10-06T02:54Z as HEAD `1f92d08`. L6b is open as PR #35. |
| L6b will "write ADR-0083" | `handover.md:68` | **False.** `ledgers.md:20` says the next free number is 0079. PR #35 adds `.kb/decisions/0079-a-query-item-binds-a-constant-number-of-parameters.md` and moves the ledger to 0080. |
| "untracked `runbook/phase-15-afk-prompt.md`" | `handover.md:86` | **False.** `git ls-files` lists it; it was added in `1f92d08`. |
| Status table is held to the registry and changelog | `README.md`; `cargo xtask lints` | **True.** The gate ran green (gate-summary.md), and tags plus crates.io agree. |
| 1.0 path is 62–75 working days | `roadmap.md:78-82` | The arithmetic is right: 2-3 + 2 + 25-30 + 8-10 + 5-8 + 12 + 5 + 3-5. But it **excludes phase 20**, which phase 21 depends on (`README.md:110`; the roadmap's own diagram draws 20 into 21). It also excludes the soak. |
| Phase 9 done | `README.md:90` | Its proof artefact was restated to a `node:sqlite` shim. The original criterion "every rule … under workerd" (`RUNBOOK.md:4564`) is **red on main**: CI run 37406385109, job "conformance under workerd, local and deployed" = failure. All other jobs passed. |

---

## Findings

### F1. Phase 17 is about 30% through by exit criteria, with the heaviest unknowns still ahead (high)

- **Exit criteria:** 3 of 10 are ticked (`runbook/phases/17-breaking-window.md:257-283`).
- **Lanes landed:** L0–L6a, PRs #26–#28 and #30–#34. These are about 7 of 13 lane positions in the session-log lane list (`:300-304`).
- **Not started:**
  - the ES-11/ES-12 fence spike on Neon;
  - the ES-17 measurement;
  - the ADR-0022 §9 reproduction;
  - the guard-plan `LIST SUBQUERY` assertion;
  - `ProjectionId`;
  - codec sealing;
  - the SQL-seam statement type;
  - VT-6 mint-per-open;
  - the 0.4.0 release with its semver trace.
- **Still provisional:** ES-11, ES-12 and ES-17 are `PROVISIONAL` in §7.2, but their disposition is freeze-by-17.
- **Spend so far:** about 4 active commit days, 7 calendar days since `acffe1c` (2026-09-29), against a 25–30 day estimate. The estimate itself was already raised from 5–8 (ADR-0072).

### F2. `main` is knowingly red on the workerd job, and the fix is green but unmerged (medium)

- CI run 37406385109 on `1f92d08`:
  - the `workerd` job failed at "Every rule passed under local workerd" and at "The conformance rules on a deployed Durable Object";
  - everything else passed: gate on 3 OSes, live Postgres, live Neon, MSRV and the benchmark compile.
- PR #35 (`lane/p17-workerd-green`, head `32ea2ad`) has **all 20 check runs completed with no failures**, including workerd and semver compatibility. It is not merged.
- On `main`, Cloudflare's published partition constants are still `MAX_QUERY_ARMS_PER_STATEMENT = 400` and `MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000` (`crates/happenstance-cloudflare/src/event_store.rs:402,423`). The measured walls are 5 compound terms and 100 parameters.
- Phase 9's original "done" criterion therefore remains unmet on main.

### F3. The handover is wrong on three points (medium)

1. It says #34 is not merged.
2. It names ADR-0083.
3. It calls a tracked file "untracked".

Its self-staleness rule ("stale if As-of does not name HEAD or its parent", `handover.md:3-6`) does **not** fire. The As-of names `9d99339`, which is HEAD's parent. So the rule fails to detect a content-stale handover.

PR #35's replacement handover says "uncommitted, on `1f92d088`". It will be stale in the same way the moment it merges. The cause is structural: the handover is written in-lane, before merge.

### F4. `log.md` and phase 22's log stop at 2026-09-30 (low-medium)

- `runbook/log.md`'s newest heading is `## 2026-09-30 — happenstance-ladybug retired` (`:13`). There is no entry for L6a (10-01 to 10-05), and none for the #34 merge.
- The phase-17 session log does carry L6a, through 2026-10-02 (`17-breaking-window.md:476-536`).
- Phase 22's session log has one entry, "*uncommitted, `lane/docs-site`*" (2026-09-29, `22-docs-site.md:146`). It does not record the #29 merge or the failed Pages deploy.

### F5. The roadmap omits phase 22 and under-counts the 1.0 path (medium)

- `grep -n 22 runbook/roadmap.md` matches only `:140`, which is ADR-0022. Phase 22 has no row and no place in the diagram, yet README lists it as `in progress`.
- The "62–75 days" figure excludes phase 20 ("run alongside and are not counted", `roadmap.md:81-82`), but phase 21 depends on 20 (`README.md:110`).
- Phase 20's estimate is "Not yet made" (`phases/20-docs-that-teach.md:51`), with 29 stories specified.
- What remains on the critical path, at the plan's own figures, is about **52–64 working days, plus phase 20, plus the soak**.

### F6. Phase 20 is marked "in progress", but there is no recorded progress (medium)

- `README.md:108` says `in progress`.
- The phase file has 0/4 exit criteria, no estimate, and an empty session log (the file ends at `:54`).
- No commit names phase 20.
- It is a 1.0 dependency, and the only docs work recorded anywhere is phase 22's site shell.

### F7. Phase 22's site is built but not deployed, and the blocker is an owner action (low-medium)

- `curl https://wet-ink-corporation.github.io/happenstance/` returns **404** (2026-10-06).
- The unticked owner item reads "The owner sets the repository's Pages source to *GitHub Actions*" (`phases/22-docs-site.md:84`).
- The exit criteria stand at 0/4.

### F8. The frozen monolith's exit boxes do not back the "done" rows (low)

- `RUNBOOK.md`'s unticked exit boxes include phase 6 (0/5), phase 7 (2/8), phase 8 (0/4), phase 9 (1/4) and phase 12 (2/4).
- The artefacts were spot-checked and do exist. Examples:
  - `examples/course-subscriptions/tests/ui.rs`;
  - `CheckpointOnlyStore` in the testkit;
  - registry releases.
- The exception is phase 9's workerd criterion (`RUNBOOK.md:4564`). It was relocated to phase 17, not met.
- This is a record-keeping gap, which `runbook/README.md:71-74` concedes.

### F9. The critical path is long and has a sequencing hazard (medium)

The order after 17 is 17b (8–10), 18 (5–8), 13 (12), 14 (5) and 21 (3–5 plus soak).

**Phase 13 carries 17 of the 34 freezes still owed** (`ledgers.md:270-315`, counted by the plan map). Further:
- ADR-0026 and ADR-0027 are unwritten.
- No `happenstance-sync-testkit` crate exists (`ls crates/`).
- No `sync_peer_conformance` exists.
- SY-27 and SY-28 need phase 14's instrument, but phase 14 runs after 13. The ledger itself flags this as a hazard (`ledgers.md:263-268`).
- D-1 (`roadmap.md:93-95`) puts the least-settled component on the critical path, overriding the record's own recommendation.

### F10. The one measured re-estimate grew 3–5×, and activity is bursty (info)

- Phase 17 went from 5–8 days to 25–30.
- Phases 15 and 16 landed on estimate (2 and 1 active days).
- Commits by ISO week: W37 61, W38 1, W39 2, W40 21, W41 1. Idle gaps run 6–8 days (`git log --date=short`).
- Phases 0–9 cannot be audited from git: root `673dcc5` is a squashed import of 2,538 files.
- **Correction to map-history:** "0 tags" is a shallow-clone artefact. Origin has four release tags.

---

## Strengths

- The status table is **machine-checked** against CHANGELOG and registry (`cargo xtask lints`; status-table and milestone lints at `xtask/src/lints.rs:2205,4253,4263`). The gate ran green on this commit.
- Every phase has a named proof artefact. For the done phases spot-checked, the artefact exists in the tree:
  - `frozen_signatures.rs`;
  - the two `wire.rs` files;
  - `LivePostgresProjectionStore` (`crates/happenstance-postgres/src/live_projection_store.rs:151`);
  - the trybuild case.
- Clause accounting is exact and consistent across spec §7.1, §7.2, §1.3 and the ledgers: 203 clauses, 152 frozen, 32 provisional, 12 deferred, 7 non-normative. Every non-frozen clause on a promised surface has a 1.0 disposition (44 rows).
- **Release cadence is real:** 0.2.0, 0.3.0, 0.3.1 and 0.3.2 are tagged and published within 10 days, and the semver baseline runs against the registry.
- Phase 17 is honestly re-estimated (ADR-0072) and split so that 0.4.0 does not wait on additive work.
- Lanes land as reviewed, one-PR-per-lane units. The phase-17 session log is detailed and cites CI runs (36957404625, 36966608470, 36967951105).
- The workerd measurement work was done on a deployed Durable Object, not only locally, and the fix is already green in PR #35.
