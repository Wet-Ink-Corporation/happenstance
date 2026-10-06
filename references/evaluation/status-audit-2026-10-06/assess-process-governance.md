# Assessment: process and governance (release, CI, governance machinery)

Commit `1f92d08` (== origin/main), assessed 2026-10-06. Read-only. Sources: map-governance, map-plan, map-history, gate-summary, plus direct spot-checks in `audit-wt` and GitHub Actions/PR/tag state via the read-only GitHub MCP.

**Rating: AMBER. Completeness against the 1.0 bar for this dimension: about 65%.**

**Headline:** The release and CI machinery works: four tagged releases, a multi-OS gate that passes, live-database legs, semver checks and pinned actions. But `main` is red on two workflows. The bookkeeping lags the code. And the governance apparatus is several times larger than the library it governs, which is out of proportion for one maintainer.

## Claims versus what was shown

| Claim (document) | Shown (code / CI / registry) |
|---|---|
| `cargo xtask ci` is the whole gate and is green (CLAUDE.md) | **Confirmed.** The local gate exits 0 with 31 required steps (gate-summary.md). CI `gate` passes on ubuntu, windows and macos for run 37406385109. |
| Seven crates are published at 0.3.2 (CLAUDE.md, handover.md:19) | **Consistent.** Remote tags are `v0.2.0`, `v0.3.0`, `v0.3.1` and `v0.3.2` (GitHub `list_tags`). The plan map read crates.io max_version 0.3.2. There are no local tags because the clone is shallow. |
| The workerd job is "landed red on purpose" (commit `1f92d08`) | **Confirmed red.** In run 37406385109, job "conformance under workerd, local and deployed" failed, at step 9/11 (local) and step 15 (deployed). |
| The docs site is deployed from main (phase 22 proof artefact, README.md) | **Not true.** All three Pages push runs on main failed in `deploy`: 36810591479, 36863802395 and 37406385128. The site has never deployed. |
| The handover names HEAD or its parent (handover.md:4-6 rule) | **Violated.** The handover says "open as PR #34, on `9d99339`… Not merged" (handover.md:15-16,40). #34 is HEAD, and L6b is open as PR #35. |
| 1.0 path is 62–75 working days (roadmap.md:78) | **Understated by the roadmap's own rules.** It excludes phase 20, which phase 21 depends on (README.md:110 vs roadmap.md:81-82). It also excludes the soak. |

## Strengths

- **The gate is defined once and CI runs the same thing.** `cargo xtask ci` is green locally (31 required steps, 2,965 tests passing, 0 failing). It is also green on three OSes in CI for HEAD (run 37406385109).
- **Real-infrastructure legs exist and pass on main.** Live Postgres and live Neon both pass in run 37406385109. A deployed Cloudflare Durable Object leg exists. All of this is unusual rigour for a solo library.
- **Release hygiene is sound.** Tags `v0.2.0`–`v0.3.2` exist on the remote. Alphas are yanked. The CHANGELOG carries dated sections for each release (CHANGELOG.md:341,373,389,457).
- **Semver is checked two ways.** A registry baseline runs for all seven crates (ci.yml:1162-1175). A base-commit check is advisory only while the version is unpublished (ci.yml:1133).
- **Supply-chain controls are on:**
  - `cargo-deny`, with `yanked = deny`, `wildcards = deny` and async-trait banned with named exemptions (deny.toml);
  - SHA-pinned actions, enforced by a gate step;
  - a weekly advisories cron;
  - CodeQL, which caught and got fixed a stack-trace exposure in #34.
- **Decision immutability is enforced mechanically,** not by convention. `lint-kb` is a gate step and passed with `HS_KB_BASE=origin/main` (gate-summary.md).
- **The red main was deliberate and is being worked.** It is a recorded owner decision (wi-557b41). The fix is already open as PR #35, created 2026-10-06T05:36:59Z, and its Pages PR run 37419424003 is green.

## Findings

### PG-1 (high): `main` is red on two workflows, and one has never been green on main

- **CI:** the `workerd` job fails on HEAD (run 37406385109). This is intentional: VT-23 hits workerd's 100-parameter and 5-term limits, and the fix is PR #35.
- **Pages:** the workflow has failed on every push to main since it landed:
  - 36810591479 (09854b3, 10-01);
  - 36863802395 (9d99339);
  - 37406385128 (1f92d08).
- **Two distinct causes of the Pages failures:**
  - A nightly deprecation broke the build from 10-01. Commit `1f92d08` says the Pages workflow "has been red on main since 2026-10-01".
  - Now `deploy` fails because Pages is not enabled. This is the unticked owner item at `runbook/phases/22-docs-site.md:84`.
- **The gate missed the nightly break:** "rustdoc does not lint function bodies" (same commit message).
- **Consequences:**
  - A consumer or contributor sees red badges.
  - Cloudflare's 1.0 claim, "conformance on the real runtime", is unmet on main.
  - The phase-22 proof artefact, "the site deployed from main", is unmet.
  - Both are one owner action or one merge away. But red-on-purpose plus red-by-configuration teaches people to ignore red.

### PG-2 (high): The process apparatus is several times larger than the library it governs

Measured with `du -sb` and `wc -l` in the worktree:

**Governance prose versus shipped source**
- Governance prose is about 8.8 MB:
  - `.kb` 3.32 MB
  - `references` 3.44 MB
  - `spec` 0.74 MB
  - `RUNBOOK.md` 0.40 MB
  - `standards` 0.36 MB
  - `CHANGELOG` 0.26 MB
  - `runbook` 0.23 MB
- Published-library `src` is 1.72 MB, or 39,409 lines across 7 crates excluding the testkit, about half of them comments. That makes the ratio **about 5:1**.
- Add the frozen `.bklg` (30.2 MB) and `.redkiln` (1.36 MB) and it is **about 23:1**.

**Tooling that polices documents**
- `xtask/src` is 32,981 lines, about 84% of the shipped `src`.
- Four document-policing modules make up about 59% of xtask: `lint_pages.rs` 5,587, `lint_narrative.rs` 5,167, `lints.rs` 4,601 and `spec_trace.rs` 4,223, totalling 19,578.

**Individual oversized artefacts**
- `.github/workflows/ci.yml` has 699 comment lines against 487 lines of YAML.
- The 0.2.0 CHANGELOG section alone is 1,853 lines (CHANGELOG.md:457-2310).
- CLAUDE.md is 467 lines and loads on every task.

**What the reader actually gets**
- User docs: `docs/` is 7 files and 649 lines.

For one maintainer at 0.x, this is the main threat to throughput. Every change pays a tax to keep the apparatus consistent.

### PG-3 (medium): Line-anchored citations make the record expensive to change

- About 2,022 `RUNBOOK.md:N` references across `.md` and `.rs` files (`grep -c 'RUNBOOK.md:' -r`) freeze a 5,704-line monolith. CLAUDE.md says "never shrink it".
- 53 commit-message lines in 86 visible commits mention citations or repointing (`git log --format=%B | grep -ciE 'repoint|citation'`). Examples:
  - #29: "33 path:N citations repointed";
  - #34: "Citations into ci.yml are repointed";
  - #34: "The diff is line-neutral so no `path:N` citation moves".
- The retired `happenstance-ladybug` stays in the tree because "the specification, the ADRs and the constitution cite its files by line" (CLAUDE.md; ADR-0078).
- The design choice to cite by line, rather than by anchor or symbol, now constrains editing, deletion and refactoring.

### PG-4 (medium): The `0.4.0` release bundles a liveness fix that has sat unreleased for 15 days

- The manifests read `version = "0.4.0"` (Cargo.toml:24). The registry and tags stop at `0.3.2`, and `CHANGELOG.md:35` is `## [Unreleased]` with about 305 pending lines.
- The first pending entry is a behavioural fix to a published crate: `happenstance-sqlite`'s busy timeout goes from 5 s to 15 s (CHANGELOG.md:39-60, merged #13, `f89e184`, 2026-09-21). Its own text calls the old value a liveness defect that "goes red 7 launches in 8" under contention.
- `0.4.0` is gated on phase 17, whose exit criteria are 3 of 10 (`runbook/phases/17-breaking-window.md:257-283`) and whose estimate is 25–30 days.
- So the fix reaches consumers only after the breaking window closes. There has been no `0.3.3` patch. No document records a policy for backporting fixes during a breaking window.

### PG-5 (medium): The plan-of-record documents drift from git within days

- **The handover breaks its own staleness rule.** It is "As of… PR #34… Not merged", but #34 is HEAD (handover.md:15-16,40; `git log -1`). It also calls `runbook/phase-15-afk-prompt.md` untracked, but `git ls-files` lists it.
- **ADR numbers conflict.**
  - The handover says the L6b record is "ADR-0083" (handover.md:68).
  - The ledger says the next free number is 0079 (`runbook/ledgers.md:20`).
  - PR #35 uses ADR-0079.
  - `.kb/decisions/` ends at 0078.
- **`runbook/log.md` has no entries after 2026-09-30** (log.md:13). L6a appears only in the phase-17 session log (`17-breaking-window.md:476-529`).
- **Monolith tick-boxes are stale.** `RUNBOOK.md` exit boxes for phases 6, 7, 8, 9 and 12 are unticked while the status table says `done` (e.g. RUNBOOK.md:4086-4092, 5264-5265). The artefacts exist, so this is a records gap.
- **CLAUDE.md is stale.** It says "202 numbered clauses"; the plan map counts 203 in `spec/SPECIFICATION.md` §7.2.
- **The KB has unpromoted and missing records.**
  - Six owner decisions sit unpromoted in `.kb/_intake/decisions/`: wi-269e5a, wi-3c4f18, wi-557b41, wi-642765, wi-8cdf0a and wi-c11a24. CLAUDE.md says intake is written into `.kb/` by hand and then removed.
  - The owed review follow-up `wi-13bd3b` exists only in handover.md.

None of these is harmful alone. Together they show the record-keeping load is higher than one person sustains between sessions.

### PG-6 (medium): Estimates are unreliable, and the headline 1.0 figure omits a dependency

- **Phase 17 grew about 4–5x at kickoff,** from "~~5–8~~ 25–30" days (roadmap.md:68; ADR-0072, from a roughly 275-hour research pass).
- **The 1.0 path is understated.** It is "about 62–75 working days" (roadmap.md:78) and excludes:
  - phase 20, which is "to be estimated" (roadmap.md:75) yet a dependency of phase 21 (README.md:110);
  - the soak (roadmap.md:76).
- **Calendar implications are unstated.** Git shows 12 active commit-days in 29 calendar days (map-history). At that cadence the remaining roughly 52–64 working days is several calendar months, and nothing in the record says so.
- **Only small phases can be checked against git.** Phases 15 and 16 landed on estimate (2–3 and 2 days estimated, about 3 active days actual). Phases 0–9 cannot be checked: the root commit `673dcc5` is a squashed import of 685,860 lines.
- The roadmap concedes the problem itself (roadmap.md:81-83).

### PG-7 (low): Governance is turning inward

- `.kb/open-questions/` holds 47 open atoms (`status: accepted`) against 31 resolved.
- By title, roughly 20 of the 47 concern the governance machinery rather than library behaviour, for example:
  - `adr-status-vocabulary-exceeds-the-schema`;
  - `rustdoc-citations-relative-or-url-shaped`;
  - `references-adr-in-place-correction-policy`;
  - `gate-step-first-check-hides-its-second`;
  - `is-the-dagger-convention-superseded-by-maturity-markers`;
  - `experiment-raw-output-eaten-by-the-ignore-rule`.
- The decision corpus is 94 accepted and 2 superseded atoms (`grep -m1 '^status:'`).
- Each accepted decision is immutable, and corrections spawn new atoms. The apparatus therefore generates its own backlog.

### PG-8 (low): Dead and redundant CI weight is carried by design

- **The `backlog` job is `if: false`** (ci.yml:255), with a 60-plus-line justification comment (ci.yml:188-255), for a tool retired on 2026-09-28.
- **The `msrv` job duplicates the gate.** It runs the same 1.97.1 compiler as the gate, and CLAUDE.md admits it "proves nothing until the pin and the floor diverge".
- **Scheduled CI was red three of five times** (09-08, 09-15, 09-22). The 09-22 run 35728235725 failed only on the live Neon leg ("Every gated test passed" step). This matches the flake rates recorded in ci.yml's comments (about l.830) and the handover's warning that Neon flakes until the L8 fence lands.
- A weekly job that is red for reasons nobody acts on is the failure mode ci.yml's own comments warn against.

### PG-9 (info): The gate's size is proportionate in runtime but not in scope

- **Runtime is acceptable.** The gate takes 9m34s locally on 4 CPUs (gate-summary.md) and about 12–17 minutes per OS in CI (run 37406385109 timings).
- **Scope is heavy.** Of 31 required steps, about 12 police prose, citations, changelog structure, page declarations or the KB rather than library behaviour (step list, map-governance §4).
- **The local gate is not complete.** Four optional steps skip when tools are absent: cargo-hack powerset, wasm32 powerset, cargo-deny and nightly docs.rs. The gate also missed the nightly break that kept Pages red (PG-1). "Green local gate" is therefore weaker than CLAUDE.md implies.

## Bottom line

The machinery that protects consumers is real and working: conformance on real databases, semver baselines, deny, pinned actions and tagged releases. What is out of proportion is everything around it:

- a governance corpus about 5x the shipped code;
- line-anchored citations that freeze files;
- a self-generating backlog of meta-questions;
- run records that a solo maintainer cannot keep current.

Main is red today, with a deliberate red and a configuration red. The next release is blocked behind a phase that is 30% through its exit criteria and holding a liveness fix.

None of this is fatal. Merging PR #35, enabling Pages and promoting the intake atoms would clear the visible red and staleness within a day. The structural cost will persist unless the citation and record-keeping model is lightened.
