# Independent effort estimate: happenstance to 1.0.0

Audit of `1f92d08` (== origin/main), 2026-10-06. Read-only; no cargo run (gate status from `gate-summary.md`: all required steps green, 2,965 passed / 0 failed).
Unit: **working days at the pace this repository has actually shown** (one owner plus AI-assisted lanes). The roadmap uses "solo working days"; where the two differ it is called out.

## 1. Bottom line

| | Low | High | Central |
|---|---|---|---|
| Roadmap's own remaining path (17 rem. 19–24, 17b 8–10, 18 5–8, 13 12, 14 5, 21 3–5) | 52 | 64 | — |
| …plus phase 20 (unestimated) and the soak | not counted | not counted | — |
| **This estimate, 1.0 path incl. phase 20, excl. soak calendar** | **66** | **127** | **~90** |

The roadmap's "62–75 working days" (`runbook/roadmap.md:78`) leaves out two things 1.0 waits on: phase 20, which phase 21 depends on (`runbook/README.md:110`; `phases/20-docs-that-teach.md:51` "Not yet made"), and the soak (`roadmap.md:76`). The biggest error is phase 13. Its 12-day figure is the pre-phase-16 monolith number (`RUNBOOK.md:5596`), and phase 16 then added five items and 17 clause freezes to it (`phases/13-sync.md:10-17`) without re-estimating it. Phase 17 sat in exactly that state before its research pass raised it about 5–6x (5–8 days to 35–45 unsplit, ADR-0072; or 33–40 across 17 and 17b).

## 2. Calibration signals (and how they were reconciled)

Two signals point in opposite directions, and both are real.

1. **Un-researched estimates miss scope by about 5–6x** (5–8 days to 35–45 unsplit, ADR-0072:16-17; or 33–40 across 17 and 17b).
   - Phase 17 was 5–8 days. A research pass put it at about 275 h, or 35–45 days unsplit (`.kb/decisions/0072-phase-17-is-split-at-the-release.md:13-19,50-53`). It was then split into 17 (25–30 days) and 17b (8–10 days).
   - Three items the original estimate never carried account for most of the gap: the workerd harness at about 26 h, Busy at about 20 h, and the ES-11 fence at about 16 h.
   - Freeze attempts also tend to fail: "Thirteen of fourteen proposed freezes were refuted" (`runbook/handover.md:151-152`).
2. **Once a phase is scoped, execution beats the estimate unit by roughly 2x.**
   - **Phases 0–12.** The estimates sum to about 82 days (`RUNBOOK.md:1173…5352`). Dated session logs run from 2026-08-05 (`RUNBOOK.md:1223`) to the 0.2.0 release on 2026-09-10, about 37 calendar days. Phases 0–3, estimated at 18 days, closed between 08-05 and 08-08 (`RUNBOOK.md:1177,1464,1768,2593`).
   - **Phases 15 and 16.** These were estimated at 2–3 and 2 days. They took 2 and 1 active days (map-history §6).
   - **Phase 17.** Lanes L0–L5 merged as PRs #26–#28 and #30–#32 on 09-29 and 09-30. L6a (PR #34) was opened 10-02 and merged 2026-10-06T02:54Z. L6b (PR #35) is open, created 2026-10-06 (GitHub API). By my judgement, roughly 40–55% of phase 17's weighted work landed in 6 weekdays elapsed (09-29 to 10-06), of which about 4–5 were active days by session log, against a 25–30-day estimate. Busy and most of the workerd harness, the heaviest items, are done.
   - **Counter-measure.** Only 3/10 of phase 17's exit criteria are ticked (`phases/17-breaking-window.md:257-283`), and the items still open are the decision-heavy ones.

**How they combine:**
- **Phases that had a research pass (17, 17b):** about 0.5–1.0x the plan figure.
- **Phases that did not (13, 14, 18, 20):** about 1.5–3.5x, scaled by how much of each phase is new instrument work rather than records. In each case the scope multiplier outweighs the execution discount.

## 3. Per-phase estimate

| Phase | Roadmap days | Independent (low–high) | On 1.0 path | Remaining work evidence | Rationale |
|---|---|---|---|---|---|
| 17 remainder | 25–30 total; 6 weekdays used | **9–18** | yes | 8 open work lines; 3/10 exit criteria (`17-breaking-window.md:61-232,257-283`). Lanes left: L6b (PR #35, open), ES-17, ES-11/12 Neon fence, ADR-0022 §9 + guard-plan, ProjectionId, codec, SQL-seam type, VT-6, release with semver trace table | Lanes have run at about 1 day each for records and 3–5 days for instruments. Remaining: 2 instrument-type items (ES-17 measurement, ES-11 fence), about 6 records, and a 7-crate release. Variance comes from three places. The ES-11 fence may fail ("ask again if it fails", `handover.md:33-34`). ES-17 may change `append` (`:288-291`). §9 may force a breaking constructor change across two adapters (`:105-115`). |
| 17b | 8–10 | **6–11** | yes | 0/7 exit criteria (`17b-after-the-window.md:87-96`) | Research-estimated, so trust it with a small execution discount. The tail risk is the minimal-versions job with its wasm32 leg: five DB crates declare no `rust-version` (CLAUDE.md constraint 5). CF-40 also touches core, the testkit and cloudflare, each with mutants. |
| 18 | 5–8 | **8–15** | yes | 0/6 exit, 0/13 work (`18-typed-runner.md:21-192`). Covers 7 freezes and three rules with mutants (PS-18, PS-27, PS-30) | Not research-estimated. ADR-0074's spike settled the design, but the list is long: async `apply` across 12+ impls, the error type, `on_error`, a live savepoint test, the FNV-1a derived id with golden values, the fan-out runner with a benchmark, a refusable reset, `Chunk`, and the convergence declaration that 13 consumes. |
| 13 | 12 | **22–40** | yes (D-1) | 0/9 exit, 1/16 work (`13-sync.md:58-231`). **0 of 35 SY rules executable**: 32 of 35 rows are daggered (map-sync §1.7). No `SyncRunner`, no `happenstance-sync-testkit`, no networked peers (only `todo!()` stand-ins at `tests/real_peer_shapes.rs:110-218`). ADR-0026 and 0027 are unwritten. 17 freeze-by-13 rows (`ledgers.md:270-315`). The CF-25 portfolio check is unbuilt | The least-settled area (`roadmap.md:93-95`), and its estimate is in the same state 17's was before re-estimation. Raw sum: a new testkit crate with macro flavours (3–5), two unlike networked peers (a Durable Object over a socket needs a workerd-class harness, which L6a took about 4 days to build; Postgres over HTTP), about 30 rules with mutants, a filtered-subset instrument, a KV-capped peer, the VT-6 restore gap, the runner, and 2 ADRs. That comes to about 36–66 raw days, discounted for execution. |
| 14 | 5 | **6–11** | yes (via 13 and D-1) | 1/5 exit (`14-retention.md:108-122`) | ADR-0028's refusal shrank the decision. Still to build: a removal capability on `Fixture` across 4 adapters (raw DELETE on Cloudflare and Neon is non-trivial), the completeness instrument, the reader experiment, and the SY-32 120-day case across peers. |
| 20 | not estimated | **10–22** (+ calendar wait for outside readers) | yes (`README.md:110`) | 0/4 exit, empty session log. 3 unbuilt projects, 29 specified stories: reach-and-adapter-path 8, comprehension-evidence 10, durable-audience-closeout 11 (`.bklg/docs-that-teach/*/`); 3 more projects still to confirm | Story cost is about 0.3–0.8 day. HS-P0024 needs real readers who are not the author, so its calendar time is gated by other people. |
| 21 + soak | 3–5 + soak | **5–10** effort, plus **1–3 calendar weeks** soak | yes | 0/6 exit (`21-one-point-oh.md:78-86`) | Choosing release tooling, nine-crate PUBLISHABLE, SECURITY.md, the MSRV text, rc.1. The soak has no time floor (`roadmap.md:129-131`), but it needs all five ADR-0066 §8 conditions on the rc (`21-one-point-oh.md:55-62`): docs.rs renders for all 9, two examples build against the registry rc, a clean semver baseline, an outside-reader pass, and no rc defect needing an API change. Any API change means rc.N+1 and the checks start again. |
| 19a / 19b | 3 / 8–10 | 3–5 / 8–14 | **no** (not in 21's row) | not started | A candidate offline spoke for 13; it competes for the same owner. |
| 22 | none | 1–3 + owner action | **no** | 4/10 work; built but not deployed. The Pages deploy returns 404 until the owner enables Pages (`22-docs-site.md:84`) | Off path. Its main cost is that it already took a day inside phase 17 (PR #29). |

**On-path totals:** low 9+6+8+22+6+10+5 = **66**; high 18+11+15+40+11+22+10 = **127**. Phase 13 alone is about a third of the path.

## 4. Calendar projection (from 2026-10-06)

**Cadence observed:**
- **Git, whole history:** 12 distinct commit dates over 09-07..10-05, so 12/29 = about 2.9 active days per week.
- **Why git understates:** squash merges hide working days. PR #34 was worked on 10-02 (deployed run 36966608470) with no commit on `main` that day.
- **Runbook era (09-28 onward):** about 7 working days out of 9 calendar days, roughly 5 per week.
- **Earlier:** a near-idle stretch from 09-12 to 09-27.

| Cadence assumption (an assumption, not a measurement; dates rounded, weekdays from 2026-10-06, holidays ignored) | Low (66 d) | Central (~90 d) | High (127 d) |
|---|---|---|---|
| Sprint pace held (~5 active days/week, as since 09-28) | ~2027-01-06 | ~2027-02-09 | ~2027-04-01 |
| Long-run pace (~3.1 days/week, git days plus hidden days) | ~2027-03-04 | ~2027-04-27 | ~2027-07-20 |
| Add the soak | +1–3 weeks | +1–3 weeks | +1–3 weeks |

**Most likely window (under these cadence assumptions):** 1.0.0 lands between **late February and late May 2027**. Per R2, commit dates are a poor throughput measure under one squash-merge per lane, so this window is speculative, not measured. Mid-January 2027 is possible only if the sprint pace holds and phase 13 lands near its low end. The roadmap's own numbers at sprint pace would point to late December 2026. **0.4.0** (9–18 days) is about late October 2026 at sprint pace (2026-10-19 to 10-30), or late October to mid-November at long-run pace.

## 5. Scope levers (largest first)

1. **Revisit D-1: ship 1.0 without sync.** This was the original recommendation, overridden on 2026-09-28 (`roadmap.md:90-95`, `wi-40b321`).
   - **What it saves:** phase 13 (22–40 days) and most of 14's SY-32 work, roughly 30–45% of the remaining path. 1.0 becomes seven crates, with sync on its own 0.x line.
   - **What it costs:** the freeze-by-13 rows on core and adapter surfaces (VT-6, VT-9, VT-21, VT-24, CF-40) must be re-dispositioned by a record. VT-6's restore gap still matters for Postgres and Neon as plain stores.
2. **Renew ES-39 past 1.0 and defer phase 14.** Phase 14's own text allows it: firing is additive (`14-retention.md:91-93`). This saves 6–11 days if lever 1 is taken.
3. **Take phase 18's pre-recorded fallbacks.** The cheap version is PS-30 outside 1.0 (no fan-out runner) and PS-18 renewed (`18-typed-runner.md:146-156`); that saves 2–6 days. The last resort is "runner exempt from semver at 1.0" (`:172-173`).
4. **Narrow phase 20 to what 21 needs.** Phase 21 already requires one outside-reader pass against the rc. HS-P0024 and HS-P0025 could be cut to that, and 20 decoupled from 21's dependency row. This saves 5–12 days. Recruit the reader now, because that wait is calendar time, not effort.
5. **Timebox ES-11.** If the Neon fence spike fails, take the named, documented exception instead of iterating. This caps the largest 17-remainder variance at about 3 days.
6. **Keep 19a, 19b and 22 off the path in practice, not only on paper.** Phase 22 already took a day inside phase 17. Only the owner's Pages toggle is cheap enough to do now.

Levers 1, 2 and 4 together bring the path to about **34–64 days**: late November 2026 to early January 2027 at sprint pace, or late December 2026 to late February 2027 at long-run pace.

## 6. Claims versus evidence

- **The "62–75 days" figure is a document claim** (`roadmap.md:78`). It omits phase 20 and the soak (README dependency row 21 vs `roadmap.md:81-82`).
- **The phase 17 overrun is a scope overrun against the original 5–8 days, not an execution overrun against the re-estimate.** Against 25–30 days it is ahead of pace on weighted lanes, and behind on exit criteria (3/10). Weighted progress is my judgement; ADR-0072 gives hours for only three items.
- **Active-day counts come from git and understate effort.** The phase 0–12 pace comes from dated session logs in RUNBOOK.md (a claim), because the local clone is shallow: history before 09-07 exists on GitHub (`673dcc5`'s parent is `b0d9e67`) but was not read.
- **0.4.0 is unreleased.** crates.io max is 0.3.2, the remote tags stop at `v0.3.2`, and the workerd job is red on `main` until PR #35 merges (map-plan §0).
