# Map: the plan of record and its state

Audit reader: plan of record. Worktree `audit-wt` at `1f92d08` (== `origin/main`), read on 2026-10-06.
Read-only: no cargo was run. Paths below are relative to the worktree root unless stated.
**Convention:** "CLAIMS" means a document says it. "SHOWS" means code, git, CI or the registry was checked.

---

## 0. Headline findings

1. **The handover is stale by one merge.** `runbook/handover.md:15-16` says "open as PR #34, on `9d99339`… 2026-10-01", and `:40` says "Not merged".
   - `git log`: `1f92d08 Phase 17 L6a: the workerd job, landed red on purpose (#34)` is HEAD. GitHub REST shows PR #34 `merged_at 2026-10-06T02:54:00Z`.
   - Its next action 3, "Merge #34 red, then L6b" (`handover.md:65`), is half done. L6b is **open as PR #35** (`lane/p17-workerd-green`, created 2026-10-06T05:36:59Z, head `32ea2ad`, base `1f92d08`). Its CI run 37419423991 was `in_progress` when checked.
2. **The ADR numbers disagree.**
   - The handover's L6b plan says "write ADR-0083" (`handover.md:68`).
   - The ledger says "The next free number is 0079" (`runbook/ledgers.md:20`).
   - PR #35's body says "D5 ADR: ADR-0079".
   - `ls .kb/decisions/` ends at `0078-happenstance-ladybug-is-retired.md`.
   - So 0083 is wrong, or it assumes reservations that are recorded nowhere. `grep -rn "0079\|0080\|0081\|0082\|0083" runbook/` finds only those two lines.
3. **The `workerd` job is red on `main`, by design.** In CI run 37406385109 (push of `1f92d08`):
   - green: all three `gate` OSes, live Postgres, live Neon, MSRV and benchmarks;
   - red: `conformance under workerd, local and deployed`;
   - skipped: semver, security advisories, and backlog.

   Cloudflare's 1.0 claim is "conformance on the real runtime" (ADR-0066, cited at `phases/17-breaking-window.md:170-179`). It is unmet on `main` until PR #35 merges.
4. **The phase-22 docs site is not deployed.**
   - `curl https://wet-ink-corporation.github.io/happenstance/` returned **404** on 2026-10-06.
   - The Pages run 37406385128 on `main` failed in its `deploy` job: `Failed to create deployment (status: 404) … Ensure GitHub Pages has been enabled`.
   - This matches the unticked owner item "The owner sets the repository's Pages source to *GitHub Actions*" (`phases/22-docs-site.md:84`).
5. **`0.4.0` is not released.**
   - crates.io API: `happenstance-core` max_version `0.3.2`, updated `2026-09-21`.
   - Remote tags are `v0.2.0`, `v0.3.0`, `v0.3.1` and `v0.3.2`, with no `v0.4.0` (`git ls-remote --tags origin`).
   - The workspace manifest already reads `version = "0.4.0"` (`Cargo.toml:24`). The CHANGELOG heading is still `## [Unreleased]` (`CHANGELOG.md:35`), with no semver trace table yet.
6. **The roadmap's "62–75 working days" leaves out a 1.0 dependency.**
   - Phase 21 depends on phase 20 (`runbook/README.md:110`).
   - Phase 20 has no estimate: "Not yet made" (`phases/20-docs-that-teach.md:51`).
   - The roadmap says "Phases 19a and 20 run alongside and are not counted" (`roadmap.md:81-82`).
   - So the 1.0 figure omits a phase that 1.0 waits on. It also omits the soak, which `roadmap.md:76` lists as "3–5, plus a soak".
7. **The monolith's tick boxes do not back its "done" rows for phases 6–9 and 12.** `RUNBOOK.md` exit-criteria boxes for those phases are still unticked (§2.2). The status table says all are `done`, and the proof-artefact evidence exists in code for each one checked. The runbook README (`:71-74`) concedes the monolith's table and logs drifted. This is a record-keeping gap, not evidence that the work is missing.

---

## 1. Sources read

| File | Lines (`wc -l`) | Role |
|---|---|---|
| `runbook/README.md` | 146 | status table (`:76-110`), protocol |
| `runbook/handover.md` | 152 | "now"; claims As-of PR #34 |
| `runbook/roadmap.md` | 161 | order, estimates, D-1–D-4 |
| `runbook/ledgers.md` | 341 | ADR queue, open decisions, clause ledgers, 1.0 dispositions |
| `runbook/log.md` | 390 | dated log, newest first |
| `runbook/phases/*.md` | 11 files, 2,090 lines | 13, 14, 15, 16, 17, 17b, 18, 19 (a+b), 20, 21, 22 |
| `RUNBOOK.md` | 5,704 | frozen monolith, phases 0–12 |

---

## 2. Every phase: status, boxes, proof, estimate, dependencies

### 2.1 Summary table

How the boxes were counted:
- Phases 13 and later: `grep -c '^ *- \[x\]'` and `'^ *- \[ \]'` per file. The exit-criteria counts come from an awk pass over the lines between `**Exit criteria**` and the next `**Estimate`/`**Cases`/`**Session` or `##` heading.
- Phases 0–12: the same awk over `RUNBOOK.md`'s `## Phase N` sections.

Struck items, such as work moved to 17b, still count as `[ ]` lines, and the notes say so.

| # | Name | State (README) | Deps (README) | Exit ticked / total | Work ticked / total | Proof artefact (README) | Artefact exists? | Estimate (days) |
|---|---|---|---|---|---|---|---|---|
| 0 | Ground clear | done (`README.md:80`) | — | 6/6 (`RUNBOOK.md`) | 18/18 | `.crate` with licences+README; spec-trace failing on a broken clause | **Yes (claim backed)**: `xtask/src/package.rs` `reconcile` per CLAUDE.md; spec-trace is a gate step. Not executed here | 2 (`RUNBOOK.md:286`; `:1173` "1 day, plus 1") |
| 1 | The `!Send` proof | done (`:81`) | 0 | 5/5 | 7/7 | provided body under both flavours; all rules green on a `!Send` wasm32 store | Partly checked: `crates/happenstance-testkit/tests/memory_conformance_wasm.rs` exists | 4 |
| 2 | Instrument portfolio | done (`:82`) | 1 | 6/6 | 8/8 | six crates on real targets with real associated types | Yes, by history; ladybug since retired | 5 (`RUNBOOK.md:288`) / 6 (`:1764`) |
| 3 | Suite becomes an instrument | done (`:83`) | 1 | 7/7 | 14/14 | mutant registry | **Yes**: `crates/happenstance-testkit/tests/mutation_coverage{.rs,/}`, `projection_mutation_coverage{.rs,/}` | 6 |
| 4 | Freeze the contract | done (`:84`) | 2, 3 | 12/12 | 1/33 (32 unticked lines in non-exit sections; not reviewed) | `frozen_signatures.rs` + `compile_fail` doctest | **Yes**: `crates/happenstance-core/tests/frozen_signatures.rs`; `compile_fail` at `crates/happenstance-core/src/event.rs:95`, `store.rs:61` | 10 |
| 5 | Freeze the wire format | done (`:85`) | 4 | 7/7 | 13/13 | two `wire.rs` (core + sync) | **Yes**: `crates/happenstance-core/tests/wire.rs`, `crates/happenstance-sync/tests/wire.rs` | 3 |
| 6 | Freeze `ProjectionStore` | done (`:86`) | 4 | **0/5** (`RUNBOOK.md:4086-4092` all `[ ]`) | 0/13 | `CheckpointOnlyStore` failing + two batch shapes passing | **Yes**: `CheckpointOnlyStore` in `crates/happenstance-testkit/src/projection.rs`, used by `tests/projection_mutation_coverage/mutants.rs` | 6 |
| 7 | Typed layer + example | done (`:87`) | 4, 6 | **2/8** (`RUNBOOK.md:4193-4225`) | 0/12 | `trybuild` compile-fail case | **Yes**: `examples/course-subscriptions/tests/ui.rs`, `tests/ui/unhandled_variant.{rs,stderr}`; `trybuild.workspace = true` at `examples/course-subscriptions/Cargo.toml:44` | 8 |
| — | `0.2.0-alpha.1` | released 2026-08-16, yanked (`:88`) | 7 | — | — | — | Predates git history: the first commit is `673dcc5` on 2026-09-07 | — |
| 8 | `happenstance-sqlite` | done (`:89`) | 4, 6, 7 | **0/4** (`RUNBOOK.md:4425-4429`) | 0/8 | concurrency macro at 64 contenders; write survives reopen | Claimed. `CONTENDERS` stays 64 (`phases/17-breaking-window.md:443`). Crate published (registry 0.3.2). Not executed here | 10 |
| 9 | Cloudflare Durable Object | done (`:90`) | 2, 4 | **1/4** (`RUNBOOK.md:4564-4572`; "Every rule runs and passes under `workerd`" unticked) | 2/4 | **Restated**: rules green on wasm32 against a `node:sqlite` shim, "not `workerd`, whose run is phase 17's sibling job (ADR-0066)". The original at `RUNBOOK.md:163` was "every rule green under `workerd`" | Shim: `crates/happenstance-cloudflare/src/host.rs` mentions `node:sqlite`. **The workerd half is red on `main`** (§0.3) | 8 |
| 10a | Postgres event store | done (`:91`) | 2, 4, 6 | 5/5 (phase-10 section, shared with 10b) | 0/7 | concurrency macro on a non-serialising store; visibility cost measured | Claimed. `experiments/position-visibility/` exists | 11 for 10 whole (`RUNBOOK.md:4864`) |
| 10b | Postgres projections, Neon | done (`:92`) | 2, 4, 6 | (shared) | — | no `todo!()`; Neon's declines; `LivePostgresProjectionStore` passing 17 rules | **Yes**: `pub struct LivePostgresProjectionStore` at `crates/happenstance-postgres/src/live_projection_store.rs:151`. Non-comment `todo!()` in `crates/*/src`: **none**. The only live `todo!()` calls are in `crates/happenstance-sync/tests/real_peer_shapes.rs:110,114,194,209,218` (stand-in peers) | (shared) |
| 11 | Ladybug projection store | done (`:93`) | 6 | 3/3 | 0/5 | projection suite green on a non-SQL batch | Crate kept as a frozen record; **retired** at 17 (ADR-0078). `publish = false # RETIRED` at `crates/happenstance-ladybug/Cargo.toml:12` | 6 |
| 12 | Publish `0.2.0` | done (`:94`) | 7, 8, 10a | **2/4** (`RUNBOOK.md:5264-5280`; "Crates live; docs.rs builds green" and the semver baseline unticked) | 5/11 | seven crates on crates.io + semver baseline | **Yes (registry)**: `happenstance-core` max_stable 0.3.2; tags `v0.2.0`–`v0.3.2` on origin; `86a410c` "The registry baseline turns on" | 2 |
| — | `0.2.0` / `0.3.0` / `0.3.1` / `0.3.2` | released 09-10 / 09-11 / 09-11 / 09-20 (`:95-98`) | 12 (+10b for 0.3.0) | — | — | — | Tags on origin. CHANGELOG headings at `CHANGELOG.md:341,373,389,457` | — |
| 15 | Reconcile the record | done (`:99`) | 12 | **6/6** (`phases/15-reconcile.md:136-157`) | 12/12 | status lint failing on the pre-split table, passing now | **Yes**: `fn a_released_version_with_no_row_is_refused` at `xtask/src/lints.rs:4253`, `fn a_milestone_row_holds_its_phase_to_done` at `:4263`, `fn runbook_status_matches_the_registry` at `:2205` | 2–3 |
| 16 | Define 1.0 | done (`:100`) | 15 | **5/5** (`phases/16-define-1-0.md:100-112`) | 10/10 | 1.0 charter + a disposition for every non-frozen clause | **Yes**: `.kb/decisions/0066-what-1-0-promises.md`; the dispositions table at `ledgers.md:270-315` has 44 rows, which equals 32 provisional + 12 deferred in §7.2 (counted, §5) | 2 |
| 17 | Breaking window → `0.4.0` | **in progress** (`:101`) | 16 | **3/10** (`phases/17-breaking-window.md:257-283`) | 6/16 (two of the 10 open lines are struck "moved to 17b") | `0.4.0` released + every semver break traced to a decision | **No**: no `v0.4.0` tag; crates.io max 0.3.2; no trace table in `CHANGELOG.md` | ~~5–8~~ **25–30** (`:285-290`) |
| 17b | After the window | not started (`:102`) | 17 | **0/7** (`phases/17b-after-the-window.md:87-96`) | 0/7 | VT-14, VT-30, ES-7 frozen; minimal-versions + floating-deps jobs seen failing | **No**: VT-14, VT-30 and ES-7 are all `PROVISIONAL` in §7.2; `.github/workflows/` holds only `ci.yml` and `pages.yml` | 8–10 |
| 18 | Typed runner leaves its gate | not started (`:103`) | 17 | **0/6** (`phases/18-typed-runner.md:179-192`) | 0/13 | `rebuilding-read-models` with no `unstable-projection` in its graph | **No**: `unstable-projection = ["dep:futures-core"]` at `crates/happenstance/Cargo.toml:148`; `examples/rebuilding-read-models/Cargo.toml:39` still enables it | 5–8 |
| 13 | `happenstance-sync` + testkit | not started (`:104`) | 5, 8, 9, 10a, 10b, 12, 17, 18 | **0/9** (`phases/13-sync.md:214-231`) | 1/16 (`:124`: placeholder `EventId`/`StoreId` deleted, pulled forward by L1) | one suite green against three peers + byte-identical round trip | **No**: no `sync_peer_conformance` in any `.rs`; no `happenstance-sync-testkit` crate (`ls crates/`) | 12 |
| 14 | Retention and completeness | not started (`:105`) | 13, 17 | **1/5** (`phases/14-retention.md:108-122`; ADR-0028 ticked) | 2/7 | completeness instrument + pass list + reader experiment outcome | **No** (unverified beyond the phase file; nothing found) | 5 |
| 19a | SQLite on wasm32: skeleton | not started (`:106`) | 15 | 0/1 (shared, `phases/19-sqlite-on-wasm.md:74-77`) | 0/5 | wasm32 skeleton in the gate + verdict on driver, storage, CI | **No**: no such crate in `crates/` | 3 |
| 19b | SQLite on wasm32: adapter | not started (`:107`) | 17, 19a | (shared) | 0/4 | both suites green on wasm32 in the gate | **No** | 8–10 |
| 20 | Documentation that teaches | **in progress** (`:108`) | 15 | **0/4** (`phases/20-docs-that-teach.md:42-49`) | 0/1 | the HS-I0007 Definition of Done, re-observed from a clean checkout | The DoD source exists (`.bklg/docs-that-teach/initiative.md`). Not observed. **Session log empty** (`:54` is the last line) | **not yet made** (`:51`) |
| 22 | The documentation site | **in progress** (`:109`) | 15 | **0/4** (`phases/22-docs-site.md:88-93`) | 4/10 | site deployed from `main` with scraped examples + guide from `docs/` | **Built, not deployed**: `site/` and `.github/workflows/pages.yml` exist (merged `09854b3`, #29). The URL returns 404; the Pages deploy job failed (§0.4) | none stated in the file or the roadmap |
| 21 | `1.0.0` | not started (`:110`) | 13, 14, 16, 17, 17b, 18, 20 | **0/6** (`phases/21-one-point-oh.md:78-86`) | 0/9 | promised crates at 1.0.0 + clause audit clean | **No** | 3–5 + soak |

### 2.2 Phases 0–12: the monolith's unticked exit boxes, quoted

These are evidence for headline 7. Each phase is `done` in the status table.

- Phase 6 (`RUNBOOK.md:4086-4092`):
  - `- [ ] ADR-0017, 0018, 0019 written first…`
  - `- [ ] CheckpointOnlyStore fails at least one named rule.`
  - `- [ ] Two implementations green against the projection suite.`
  - …
- Phase 7 (`:4193-4217`): `- [ ] The trybuild compile-fail case exists and is in the gate.` The artefact exists, so the box is simply unticked. `- [ ] **PS-32, PS-33 and PS-35 leave the clause space.**`
- Phase 8 (`:4425-4429`):
  - `- [ ] All four macros green…`
  - `- [ ] A benchmark *number* in ADR-0022…`
  - `- [ ] No todo!() on any SQLite path.` This one holds now: there is no non-comment `todo!()` in `crates/happenstance-sqlite/src`.
  - `- [ ] publish = false removed.` This holds too: `grep '^publish' crates/*/Cargo.toml` lists only ladybug and sync.
- Phase 9 (`:4564-4572`):
  - `- [ ] Every rule runs and passes under workerd…`. This has **moved** to phase 17 per `README.md:90` and is still red on `main`.
  - `- [ ] ES-6 is decided…`. ES-6 has been settled since phase 2 by ADR-0009 (`ledgers.md:100`).
- Phase 12 (`:5264-5265`): `- [ ] Crates live; docs.rs builds green.` and `- [ ] cargo-semver-checks compares against a real published baseline…`. Both have since happened, per the registry and `86a410c`.

**Assessment:** for the items I checked, the boxes are stale. The work exists in the tree or the registry. The exception is phase 9's workerd criterion: it was relocated, not done.

---

## 3. Phase 17 in detail: lanes, work items, what landed

### 3.1 Lane list

The lane order is written once, in the phase-17 session log (`phases/17-breaking-window.md:300-304`):

> kickoff; VT-10; the provided-method spike with ADR-0028; the `apply` and port-clause records; the renames and manifest breaks; `Busy`; the `workerd` job, then Cloudflare's partition; ES-17; ES-11 on Neon; ADR-0022 §9; `ProjectionId`; the codec; the release.

Explicit L-numbers appear only for L1–L6b (session log, `log.md`, handover) and for **L8 = the ES-11 Neon fence** (`handover.md:123`: "until L8 lands its fence"). L7 and L9 onward are **inferred** from the list order and recorded nowhere.

| Lane | Scope | PR | Commit on `main` | State |
|---|---|---|---|---|
| L0 | kickoff, ADR-0072 split | #26 | `acffe1c` (2026-09-29) | **landed** |
| L1 | VT-10 foreign-identity spike, ADR-0073 | #27 | `52aa951` | **landed**. VT-10 `FROZEN` in §7.2 |
| L2 | provided-method spike + ADR-0028 | #28 | `004413a` | **landed** |
| L3 | `apply` record ADR-0074, port clauses ADR-0075 | #30 | `be09a7a` | **landed** |
| L4 | renames + manifest breaks, ADR-0076, 0.4.0 manifests | #31 | `18e6a32` | **landed**. `Cargo.toml:24` reads `0.4.0` |
| L5 | `AppendError::Busy`, ADR-0077, ES-43 | #32 | `159ed70` | **landed**. ES-43 `FROZEN` in §7.2 |
| (ladybug) | retirement, ADR-0078 | #33 | `9d99339` | **landed** |
| (phase 22) | docs site | #29 | `09854b3` | **landed**, but not deployed |
| L6a | `workerd` job, red on purpose | #34 | `1f92d08` (2026-10-06) | **landed**. The `workerd` CI job fails on main as intended |
| L6b | `json_each` rendering; 5 arms / 90 params; ADR-0079 | **#35, open** | — (head `32ea2ad`) | **in review/CI**. The PR body reports local workerd 97/97. Still owed: the deployed transcript and Node 24 on three runners |
| L7 (inferred) | ES-17 measurement or freezing record | — | — | **not started**. No commit matches `ES-17`; ES-17 `PROVISIONAL` |
| L8 | ES-11/ES-12 fence spike on Neon | — | — | **not started**. ES-11 and ES-12 `PROVISIONAL`; the handover trap says Neon flakes until then |
| L9 (inferred) | ADR-0022 §9 reproduction + guard-plan `LIST SUBQUERY` assertion | — | — | **not started**. There is no `LIST SUBQUERY` assertion in tests: `query_sql.rs:91,103,798` are comments, and `event_store.rs:2912` asserts only the page plan |
| L10 (inferred) | `ProjectionId` (+ SY-31 `sync/` reservation) | — | — | **not started**. `.kb/open-questions/projection-id-is-unvalidated.md` is still `status: accepted` (open) |
| L11 (inferred) | codec sealing | — | — | **not started**. `should-codec-be-sealed` is still open |
| L12 (inferred) | release `0.4.0` + semver trace | — | — | **not started** |
| unplaced | SQL-seam statement type; VT-6 mint-per-open for Postgres and Neon | — | — | **not started**. Both open questions are `status: accepted` (open). Neither is in the lane list, though ADR-0072 keeps both in 17 (`.kb/decisions/0072-…md:79-80`) |

`git log --oneline | grep -iE 'phase 17|L[0-9]'` returns only `1f92d08`, `004413a` and `acffe1c`. The other lanes are identified by PR title (REST API) and by the session log.

### 3.2 Phase-17 work checkboxes, quoted

These come from `phases/17-breaking-window.md`.

Ticked `[x]`:
- `:31` ADR-0028
- `:41` ADR-0026's published half (ADR-0073)
- `:49` `Projection::apply` (ADR-0074)
- `:137` ADR-0057 executed
- `:146` core's `unstable-projection` removed
- `:156` `naive-arm`

Open `[ ]`:
- `:61` the breaking open questions. Three sub-items are struck as settled (Busy, cf-23, ES-6 prose). **Four remain:**
  - `should-codec-be-sealed`
  - `projection-batch-sql-seam-statement-type`
  - `projection-id-is-unvalidated`
  - `es-17-two-adapter-measurement-is-unscheduled`
- `:105` ADR-0022 §9 reproduction
- `:117` guard-plan assertion
- `:126` QueryItem, struck and moved to 17b
- `:128` VT-6 mint-per-open
- `:162` ES-11 record
- `:170` `workerd` job. It exists and is red; the measurements are recorded locally and deployed; the box is not yet ticked.
- `:180` minimal-versions, struck and moved to 17b
- `:182` clause follow-ups (ES-11, ES-12 and ES-17 remain)
- `:232` release `0.4.0`

### 3.3 Phase-17 exit criteria, quoted (`:257-283`)

- [x] ADR-0028 accepted… (amended by the owner on 2026-09-29, `:359-365`)
- [x] Foreign-identity question answered (ADR-0073)
- [ ] ADR-0022 §9 reproduction + classification
- [ ] Guard-plan `LIST SUBQUERY` assertion in the tree
- [x] `apply` record accepted (ADR-0074)
- [ ] Every breaking open question answered or closed
- [ ] `0.4.0` released and semver findings fully traced
- [ ] `workerd` job exists, watched failing once, walls + partition widths recorded locally and deployed. **Nearly met on evidence:**
  - the job is on `main`;
  - failing runs exist: 36957404625, 36966608470, 36967951105 and main's 37406385109;
  - the walls are in `experiments/durable-object-limits/results/run-workerd-local-2026-10-01.txt` and `…/run-workerd-deployed-2026-10-02.txt`;
  - the partition widths are set in PR #35, which is unmerged. `crates/happenstance-cloudflare/src/event_store.rs:402,423` on `main` still read 400 and 30_000.
- [ ] Every `freeze-by-17` clause `[FROZEN]`. Three remain: **ES-11, ES-12, ES-17**, all `PROVISIONAL` in §7.2.
- [ ] Spec reconciled + spec-trace passes

### 3.4 Pace versus estimate (arithmetic, not a forecast)

- Phase 17 opened on 2026-09-29 (`acffe1c`) at an estimate of 25–30 days (`:285`). By 2026-10-06 it had run 6 weekdays.
- Landed lanes are about 7 of 13 named list positions. By the record's own weighting, the heaviest items are still ahead or partly done:
  - the workerd harness, about 26 h, mostly done;
  - Busy, about 20 h, done;
  - the ES-11 fence spike, about 16 h, not started.

  The weights come from ADR-0072 (`.kb/decisions/0072-…md:52-53`).
- The exit criteria stand at 3/10.

---

## 4. Decisions pending the owner, the ADR queue, and the KB intake

### 4.1 ADR queue (`ledgers.md:18-45`)

| ADR | Phase | Status per ledger | Evidence |
|---|---|---|---|
| 0026 | 13 (published half settled at 17 by ADR-0073) | unwritten (`:35-36` "0026 and 0027 are still unwritten") | `ls .kb/decisions/` shows no 0026 |
| 0027 | 13 | unwritten | no 0027 |
| 0028 | 14 (decided at 17) | **written** | `.kb/decisions/0028-what-a-store-may-forget.md` |
| 0066 | 16 | **written** | `.kb/decisions/0066-what-1-0-promises.md` |
| unnumbered | 17 (`apply`) | **settled**, ADR-0074 | exists |
| unnumbered | 19b | open: a shared home for the SQLite SQL once a third copy exists | — |
| *(next free)* | — | **0079**, per the ledger | PR #35 claims 0079 for L6b. The handover says 0083 (**conflict**) |

Phase 17 still owes these records, which are not yet in the queue table:
- the ES-11 record that supersedes ADR-0061;
- the ES-17 record or measurement;
- ADR-0022 §9's classification record;
- `ProjectionId`;
- the codec;
- the SQL-seam statement type;
- VT-6.

`ls .kb/decisions/ | wc -l` = 97 entries. That includes `README.md`, the `wi-*`, `sd-*` and other atoms; numbered ADRs run to 0078.

### 4.2 Open decisions (`ledgers.md:53-73`), still open

| Decision | Phase | Status |
|---|---|---|
| `append` batch ownership (ES-17) | 17 | open, `PROVISIONAL` |
| ADR-0022 §9, the runtime seam (captured `Handle`) | 17 | open |
| whole-log vs scoped replication (SY-27, SY-28) | 13 | deferred |
| idempotent bulk ingest (SY-14) | 13 | deferred |
| peer-declared limits (SY-18) | 13 | deferred |
| per-peer watermark placement (SY-31) | 13 | provisional |
| query changed under checkpoint (PS-25) | 18 | provisional |
| snapshotting; tracing/metrics | after 1.0 | deferred |

### 4.3 Owner decisions taken (`roadmap.md:85-144`)

- D-1: sync is inside 1.0 (`wi-40b321`). The recommendation was the opposite and was overridden.
- D-2: `.bklg/` frozen (`wi-016abe`).
- D-3: merged branches deleted (`wi-b9b9ab`).
- D-4: the 1.0 charter (ADR-0066/0067, seven `wi-*` atoms). Nine crates; ladybug since retired.

### 4.4 Waiting on the owner (`handover.md:72-86`)

- **Follow-up PR for PR #33's review (`wi-13bd3b`).**
  - The defect is real in code. `declared_excludes` matches by prefix: `line.trim_start().starts_with("exclude")` at `xtask/src/affected.rs:640`.
  - `wi-13bd3b` exists **only** in `handover.md`, not in `.kb/` or `_intake` (`grep -rln wi-13bd3b`).
- Defaults the lanes will take unless overridden:
  - the `trait-variant` caret;
  - an FNV-1a PS-25 digest;
  - `ProjectionId` refusing the ADR-0015 set;
  - Neon `push` narrowing rides `0.4.0`;
  - VT-30 as a deprecated alias;
  - new CI jobs not required.
- Phase-15 leftovers: the Weigh-In digest, merged `lane/*` branches, and two "untracked" files.
  - The handover is **wrong on one**: `runbook/phase-15-afk-prompt.md` is now tracked. `git ls-files` lists it, added in `1f92d08`. `assets/brand/happenstance-mark.png` is not tracked.
- **Phase 22 owner items:** set the Pages source (`phases/22-docs-site.md:84`). This is the cause of the 404 in §0.4.

### 4.5 Owner decisions decided but not yet written into `.kb/`

These sit in `.kb/_intake/decisions/`. CLAUDE.md says intake atoms are written into `.kb/` by hand.

| Atom | Title |
|---|---|
| `wi-269e5a` | "Follow-up branch" (first how-to pages) |
| `wi-3c4f18` | "DDD aggregates" (prior-model page) |
| `wi-557b41` | "Fix the token, re-run, then merge" |
| `wi-642765` | "Accept as pre-existing" (lint exceptions) |
| `wi-8cdf0a` | "Keep as draft" (personas) |
| `wi-c11a24` | "Keep as built" (namespaced store API) |

### 4.6 Open questions

`.kb/open-questions/` holds 47 `status: accepted` (open) and 31 `superseded` (`grep -h '^status:' | sort | uniq -c`).

---

## 5. Clause state (counted from `spec/SPECIFICATION.md` §7.2)

The §7.2 table starts at line 9802. I counted column 3 per clause row:

**152 FROZEN, 32 PROVISIONAL, 12 DEFERRED, 7 NON-NORMATIVE = 203 clauses.**

- The ledger headings agree: "The 12 `[DEFERRED]`" at `:81` and "The 32 `[PROVISIONAL]`" at `:145`.
- At the split, `roadmap.md:13-14` gave 141/41/12/7 = 201. The difference is ES-43 and CF-41, minted at 17, plus nine freezes.
- `CLAUDE.md` says "202 numbered clauses". That is stale by one, and outside the runbook.

1.0 dispositions by kind (`ledgers.md:270-315`, 44 rows):

| Disposition | Rows |
|---|---|
| freeze-by-13 | 17 |
| freeze-by-18 | 7 |
| freeze-by-14 | 4 |
| freeze-by-17 | 3 (ES-11, ES-12, ES-17) |
| freeze-by-17b | 3 (VT-14, VT-30, ES-7) |
| renew-past-1.0 | 10 |

**Phase 13 carries 17 of the 34 freezes still owed before 1.0**, and it is the least-settled area (D-1's own warning, `roadmap.md:93-95`). Its four rows that depend on phase 14's instrument (SY-27, SY-28, CF-27 and arguably CF-40) are flagged as a sequencing hazard (`ledgers.md:263-268`).

---

## 6. Inconsistencies between the documents

| # | Where | What disagrees | Evidence |
|---|---|---|---|
| 1 | handover vs git | The As-of line names #34 open on `9d99339`; #34 is merged as HEAD `1f92d08`; L6b is PR #35, open | `handover.md:15-16,40`; `git log -1`; REST `pulls?state=all` |
| 2 | handover vs ledger vs PR #35 | L6b ADR is "ADR-0083", against next free 0079 and PR #35's ADR-0079 | `handover.md:68`; `ledgers.md:20`; PR #35 body |
| 3 | handover vs git | "untracked `runbook/phase-15-afk-prompt.md`", but the file is tracked | `handover.md:86`; `git ls-files` |
| 4 | log.md vs git | `log.md`'s newest entry is 2026-09-30 (ladybug). There is **no entry** for L6a (10-01 to 10-05) or for phase 22's merge (#29, `09854b3`). Several entries still read "*Uncommitted at writing*" | `runbook/log.md:13-15`; `grep -n "L6\|workerd" runbook/log.md` hits only lines 145 and 161 |
| 5 | phase 22 file vs git | The session log's only entry is "*uncommitted, `lane/docs-site`*" (2026-09-29). The merge on 10-01 (#29) and the failed deploy are not logged | `phases/22-docs-site.md:146-154` |
| 6 | phase 20 state vs file | The status table says `in progress`, but the file has 0/5 boxes and an **empty session log**, with no estimate | `README.md:108`; `phases/20-docs-that-teach.md:51-54` |
| 7 | roadmap vs README | The roadmap omits phase 22 entirely: no row in its table (`:64-76`) and nothing in its diagram (`:54-63`). The README lists 22 as `in progress` | `grep -n 22 runbook/roadmap.md` matches only `:140` (ADR-0022) |
| 8 | roadmap estimate vs dependency | The 1.0 path of 62–75 days excludes phase 20, which phase 21 depends on, and the soak | `roadmap.md:78-82`; `README.md:110` |
| 9 | phase 21 file vs README | The phase-21 prose says its dependency row is "13, 14, 16, 17, 18 and 20". The README row also includes **17b** | `phases/21-one-point-oh.md:10-11`; `README.md:110` |
| 10 | phase 21 file vs ADR-0072 | It says "phase 17's minimal-versions job", but that job moved to 17b | `phases/21-one-point-oh.md:35`; `phases/17-breaking-window.md:180` |
| 11 | README phase 9 proof vs monolith | The proof artefact was restated from "every rule green under workerd" to "against a `node:sqlite` shim — not workerd". The phase stays `done` while its original criterion is unticked and red on main | `README.md:90` vs `RUNBOOK.md:163,4564` |
| 12 | monolith ticks vs status | Phases 6, 7, 8, 9 and 12 are marked done, but have unticked exit boxes | §2.2 |
| 13 | roadmap "where it starts" | "its four `todo!()` bodies" in `happenstance-sync` (`roadmap.md:19-22`). There are now **zero** non-comment `todo!()` in `crates/happenstance-sync/src`. The crate says so at `crates/happenstance-sync/src/lib.rs:135`, and L1 removed them. The roadmap text is dated 2026-09-28, so this is history and not wrong as written | grep, §2.1 |
| 14 | handover "Waiting on the owner" | `wi-13bd3b` is cited as a record but no atom exists | grep |
| 15 | CLAUDE.md | "202 numbered clauses" against 203 counted | §5 |

No disagreement was found between the README status table and the phase files' own `Depends on` notes for 13, 14, 17b, 18 and 19. The README proof-artefact cells for 14 and 17b match the restated phase-file text.

---

## 7. What remains, by the plan's own estimates

The 1.0 path is 17 (remainder) → 17b → 18 → 13 → 14 → 21.

| Phase | Plan estimate (days) | Note |
|---|---|---|
| 17 (remainder) | 25–30 total; 6 weekdays elapsed | 3/10 exit criteria. ES-11 spike, ES-17, §9, ProjectionId, codec, SQL seam, VT-6 and the release are untouched |
| 17b | 8–10 | 0/7 |
| 18 | 5–8 | 0/6 |
| 13 | 12 | 0/9; 17 clause freezes. ADR-0026 and 0027 are unwritten |
| 14 | 5 | 1/5 |
| 21 | 3–5 + soak | 0/6 |
| 20 | **not estimated** | on the 1.0 path by the README dependency row |
| 22 | not estimated | not on the 1.0 path; blocked on the owner's Pages setting |
| 19a / 19b | 3 / 8–10 | off the 1.0 path |

Summing the plan's figures for the critical path still to run: the remainder of 17 at roughly (25–30) − 6, plus 17b, 18, 13, 14 and 21. That gives roughly **52–64 working days plus phase 20 and the soak**.

This is arithmetic on the plan's own estimates, not a forecast. The record itself notes "estimates being wrong" (`roadmap.md:82-83`). Phase 17 already went from 5–8 to 25–30 (ADR-0072).
