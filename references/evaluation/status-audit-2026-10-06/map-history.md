# Map: git history and velocity (happenstance @ 1f92d08, audited 2026-10-06)

Worktree: scratchpad/audit-wt (detached at 1f92d08 == origin/main). All commands run read-only.

## 1. Headline numbers
| Metric | Value | Command |
|---|---|---|
| Commits reachable | **86** (also 86 on origin/main in /home/user/happenstance) | `git log --format=%h \| wc -l`; `git rev-list --count origin/main` |
| First commit | 673dcc5, 2026-09-07 "Repair three status claims that had gone stale..." | `git log --reverse` |
| Last commit | 1f92d08, 2026-10-05 "Phase 17 L6a: the workerd job, landed red on purpose (#34)" | `git log -1` |
| Calendar span | 29 days (09-07 .. 10-05) | |
| Distinct commit dates (active days) | **12** | `git log --format=%ad --date=short \| sort -u \| wc -l` |
| Git tags | **0** (also 0 in the main checkout) | `git tag -l` |
| `(#N)` PR subjects | 28 on main, #7..#34, plus 2 older "Merge pull request #5/#6" merges | grep |
| Files tracked / Rust files / Rust LOC | 2,833 / 354 / 191,821 | `git ls-files`, `xargs cat \| wc -l` |

**CAVEAT, critical.** Root commit 673dcc5 is a squashed import: `git show --stat 673dcc5` = 2,538 files, +685,860 lines. `git rev-parse --is-shallow-repository` = true in the worktree. So the history of phases 0-9 (and the creation of everything up to 2026-09-07) is NOT in git; it exists only in RUNBOOK.md prose. Roughly 354 Rust files exist at 673dcc5 already (`git ls-tree -r 673dcc5 --name-only | grep -c '\.rs$'` = 354, i.e. all of them: no Rust file count change since). Whether the squash was deliberate or the clone truncated is unverified. Consequence: "velocity" can only be measured for 2026-09-07 onward; phases 0-9 estimate-vs-actual is not measurable from git.

Also: the 28 PR numbers #7..#34 imply roughly #1-#6 existed before; PRs #1-#4 are not visible, #5/#6 are merge commits (54044ac 2026-09-08, beaf048 2026-09-10). Missing numbers #15 (not seen in subjects; #14 then #16) - minor, unverified.

## 2. Commits per day and per ISO week
Per day (`git log --format=%ad --date=short | sort | uniq -c`):
09-07:18, 09-08:18, 09-09:14, 09-10:7, 09-11:4, 09-20:1, 09-21:2, 09-28:10, 09-29:6, 09-30:4, 10-01:1, 10-05:1. (Sum 86.)

Per ISO week (`--date=format:'%G-W%V'`): W37 (09-07..13) 61; W38 (09-14..20) 1; W39 (09-21..27) 2; W40 (09-28..10-04) 21; W41 (10-05..) 1.
Gaps: 09-12..09-19 (8 days, one commit on 09-20), 09-22..09-27 (6 days), 10-02..10-04 and 10-06 (none). Today 10-06 has no commit yet.
Burst pattern: 61 of 86 commits (71%) landed in the first week, many being telemetry/KB-intake chores (chore(telemetry), "docs(kb-intake)"). Since 09-28 the pattern is PR-per-lane, 1-10 commits/day.

## 3. Merged PRs (from subjects) with dates
| PR | Date | Hash | Subject (truncated) |
|---|---|---|---|
| #5 (merge) | 09-08 | 54044ac | lane/0.2.0-closeout |
| #6 (merge) | 09-10 | beaf048 | lane/0.2.0-closeout |
| #7 | 09-11 | 967f963 | 0.3.0: projection port frozen |
| #8 | 09-11 | 60f0081 | KB intake wave, ADR-0062/63/64 atoms |
| #9 | 09-11 | 5de44e8 | three crate descriptions catch up |
| #10 | 09-11 | 41ae2ad | 0.3.1 description-only release |
| #11 | 09-20 | 55288c5 | 0.3.2 dependency-advisory release |
| #12 | 09-21 | 142b558 | testkit can produce a busy store |
| #13 | 09-21 | f89e184 | busy timeout fifteen seconds (ADR-0022 s11) |
| #14 | 09-28 | 65253fc | runbook becomes a directory, road to 1.0 |
| #16-#22 | 09-28 | c0df525..4c538e7 | Phase 15 lanes (registry, ingest re-check, lint-kb #19, KB wave #20, open questions #21, adapter pages #22) |
| #23 | 09-29 | 3dcba41 | Phase 15 done |
| #24, #25 | 09-29 | 230065f, d6e42df | Phase 16 done (ADR-0066); log record |
| #26 | 09-29 | acffe1c | Phase 17 opens, split (ADR-0072) |
| #27 | 09-29 | 52aa951 | VT-10 frozen (ADR-0073) |
| #28 | 09-29 | 004413a | ADR-0028 forgetting as refusal (P17 L2) |
| #29 | 09-30 | 09854b3 | Phase 22 documentation site |
| #30 | 09-30 | be09a7a | apply record, projection 1.0 clauses (ADR-0074/75) |
| #31 | 09-30 | 18e6a32 | first breaking PR: 0.4.0 manifests |
| #32 | 09-30 | 159ed70 | AppendError::Busy (ES-43, ADR-0077) |
| #33 | 10-01 | 9d99339 | ladybug retired (ADR-0078) |
| #34 | 10-05 | 1f92d08 | Phase 17 L6a workerd job, **landed red on purpose** |
Command: `git log --date=short --format='%ad %h %s' | grep -E '\(#[0-9]+\)'`. 28 matches, 23 PRs on 7 days listed above (counts include #25).

## 4. Releases
No git tags exist, so releases are dated by commit subject only:
| Release | Evidence | Date |
|---|---|---|
| 0.2.0 | closeout lane merges #5 (09-08), #6 (09-10); CLAUDE.md says shipped 2026-09-10 (docs claim; no tag) | 2026-09-10 |
| 0.3.0 | 967f963 | 2026-09-11 |
| 0.3.1 | 41ae2ad | 2026-09-11 |
| 0.3.2 | 55288c5 | 2026-09-20 |
| 0.4.0 | not released; manifests bumped by 18e6a32 (#31, 09-30) per subject | pending |
CLAUDE.md claims registry state "measured 2026-09-29: all seven crates published at 0.2.0..0.3.2" - a docs claim; not verifiable from git (no tags, no network checks here).

## 5. Phase to date-range timeline
Source: commit subjects plus runbook/log.md dated headings (log.md lists 2026-09-28..09-30 entries only) and phase files.
| Phase | Date range | Evidence | Active days |
|---|---|---|---|
| 0-9 | before 2026-09-07 | not in git (squashed root); RUNBOOK.md only | n/a |
| 10 (split 10a/10b) | split recorded 09-07 | b17a316 "Split phase 10 into 10a and 10b" | n/a (work pre-dates visible history) |
| 11 (Ladybug) | frozen 09-08 (b20eb4f) ; retired 10-01 (#33, ADR-0078, phase 17) | | |
| 12 (publish 0.2.0) | 09-07 .. 09-10 closeout lanes (#5, #6) | 4adbb97 certification review 09-09 | 4 (09-07..10, dates with commits 09-07,08,09,10) |
| 0.3.0 projection freeze (ADR-0062/63) | 09-11 | 967f963 | 1 |
| 0.3.1 / 0.3.2 patch releases | 09-11, 09-20 | | |
| 15 Reconcile | 09-28 .. 09-29 | #16-#23; log "phase 15 done" 09-29 | 2 |
| 16 Define 1.0 | 09-29 | 230065f | 1 |
| 17 Breaking window | 09-29 kickoff (acffe1c) .. ongoing | #26-#28,#30-#34 minus #29 | 4 so far (09-29, 09-30, 10-01, 10-05) |
| 22 Docs site | 09-30 | 09854b3 (#29) | 1 (inside 17 days) |
| 13, 14, 17b, 18, 19, 20, 21 | not started in git | no commits with those phase names (`grep -iE 'phase (13|14|17b|18|19|20|21)'` = none) | 0 |

## 6. Estimate vs actual
Estimates: RUNBOOK.md `**Estimate.**` lines and runbook/roadmap.md table. Actuals only where visible in git.
| Phase | Estimate (days) | Actual calendar span | Active days | Ratio (active/est) | Note |
|---|---|---|---|---|---|
| 0 | 1 (+1) | unmeasurable | - | - | pre-history |
| 1 | 4 | unmeasurable | - | - | |
| 2 | 6 | unmeasurable | - | - | |
| 3 | 6 | unmeasurable | - | - | |
| 4 | 10 | unmeasurable | - | - | |
| 5 | 3 | unmeasurable | - | - | |
| 6 | 6 | unmeasurable | - | - | |
| 7 | 8 | unmeasurable | - | - | |
| 8 | 10 | unmeasurable | - | - | |
| 9 | 8 | unmeasurable | - | - | |
| 10 | 11 | unmeasurable | - | - | |
| 11 | 6 | unmeasurable | - | - | |
| 12 | 2 | 09-07..09-10 visible (4 days) | 4 | ~2.0x (partial; start may pre-date 09-07) | upper bound only |
| 15 | 2-3 | 09-28..09-29 (2 cal. days) | 2 | 0.7-1.0x | on estimate |
| 16 | 2 | 09-29 (1 day) | 1 | 0.5x | under |
| 17 | 5-8 originally; **25-30** re-est. | 09-29..10-05 so far (7 cal. days) | **4 of 25-30** | 4/25=16%..4/30=13% consumed; vs original 5-8 already 0.5-0.8x | in progress |
| 13 | 12 | not started | 0 | - | |
| 14 | 5 | not started | 0 | - | |
| 17b | 8-10 | not started | 0 | - | |
| 18 | 5-8 | not started | 0 | - | |
| 19a / 19b | 3 / 8-10 | not started | 0 | - | |
| 21 | 3-5 plus soak | not started | 0 | - | |
Sum of pre-13 measured phases is the only honest ratio: 15+16 = est 4-5 days, actual 3 active days (09-28, 09-29 with phase 17 kickoff on 09-29 shared). Caveat: "days" in the estimates are working days of solo effort; active days (a commit date) are a lower bound on effort since one commit date can hold a long session (09-28 has 10 commits, 09-07/08 have 18 each).

### Phase 17 re-estimate (ADR-0072)
- Original 5-8 days; re-estimated to 25-30 at start 09-29 (roadmap.md table row 17; phases/17-breaking-window.md:285). log.md:143 says a read-only research pass put the unsplit phase at ~275 hours; ADR-0072 split off 17b (8-10 days), so 17 keeps 25-30.
- Roadmap line 78: solo 1.0 path ~62-75 working days; was 34-43 before the re-estimate. 1.0 path = 15,16,17,17b,18,13,14,21.
- Consumed so far: kickoff acffe1c (09-29) through 1f92d08 (10-05): **4 active days** (09-29, 09-30, 10-01, 10-05), 7 calendar days, 12 commits since 09-29 (`git log --since=2026-09-29 --oneline | wc -l` = 12 = 6+4+1+1; the 6 on 09-29 include phase 16 close). Remaining at the estimate: ~21-26 active days, i.e. 84-87% of phase 17 left. Lanes landed per phase file: L1 VT-10, L2 ADR-0028, apply/port clauses, 0.4.0 manifests, Busy, ladybug retirement (side effect), workerd job (red on purpose). Lanes remaining named in log.md:~145 list: Cloudflare partition, ES-17, ES-11 on Neon, ADR-0022 s9, ProjectionId, codec, release (unverified how many are done; check phase file).
- Remaining 1.0 path after phase 17 per roadmap: 17b 8-10, 18 5-8, 13 12, 14 5, 21 3-5 = 33-40 days, so total remaining ~54-66 active days at the roadmap's own numbers (arithmetic from the roadmap table; the roadmap itself says "Every figure is an estimate").

## 7. Lines of code over time
Cheap view only (`git log --shortstat`): the root import is +685,860 lines (includes many non-Rust files: docs, ADRs, .bklg, .redkiln, references). Post-root churn:
| Month | insertions | deletions | method |
|---|---|---|---|
| 2026-09 | 1,476,056 - 685,860 root = ~790,196 | 29,286 | shortstat sum, all files |
| 2026-10 | 6,886 | 848 | same |
Caution: 790k insertions in 3 weeks post-root is dominated by non-code (telemetry events, KB intake, benchmark run records, generated docs); not a measure of code. Rust today: 191,821 lines in 354 files (`git ls-files '*.rs' | xargs cat | wc -l`); the same 354 .rs file count at root, so net new Rust files since 09-07 = 0 (unverified for renames/deletions; count equality only). Not computed: per-crate LOC.

## 8. Observations
1. Git shows only 29 days; phases 0-9 estimates cannot be audited from git. Recommend not claiming an estimate-accuracy figure for them.
2. Roadmap estimate quality: the one measurable re-estimate (phase 17) rose 3-5x; phases 15 and 16 landed on time. The estimate of record was wrong where the scope included unknown instrument work (workerd, Busy across three adapters).
3. Zero tags and 0 GitHub-release evidence in git versus claims of four published releases: process gap, or tags live elsewhere (unverified).
4. Last commit lands a CI job "red on purpose" (1f92d08): known-red main as of 10-05; whether the gate currently passes is out of scope here.
5. Velocity dropped from 61 commits (W37) to ~1-2/week in W38-W39, then 21 in W40, then 1 in W41; activity is bursty, with 5 idle gaps of 3+ days.
