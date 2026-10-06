# Assessment: risk to a credible 1.0

Commit `1f92d08` (== origin/main), assessed 2026-10-06. Read-only. No cargo was run; gate status comes from `gate-summary.md`.
"CLAIM" means a document says it. "SHOWS" means code, git, CI or the registry was checked during this pass.

**Rating: AMBER, close to red on schedule.** Completeness against the 1.0 target for this dimension is about 40%.

**Headline.** The engineering is sound and the gate is green. The risk is concentrated elsewhere:
- The unbuilt replication crate sits on the critical path to 1.0.
- One maintainer works in bursts and carries a governance apparatus larger than the library it governs.
- No outside user has yet tried the API that 1.0 would freeze.

## How this was checked

These are the spot-checks run in this pass. They override the maps where the two disagree.

**Git**
- `git log --format='%an <%ae>' | sort | uniq -c` shows a single author, Ryan Britton, under two emails (56 + 30 commits).
- `git log --since=2026-09-07 --format=%ad | sort -u` shows 12 active days in 29 calendar days.

**GitHub REST**
- PR #35 (`32ea2ad`) is open. Every check is **green**, including `conformance under workerd, local and deployed`. This is newer than the maps, which recorded its CI as in progress.
- Repo: 1 star, 0 forks, 0 issues ever filed (`issues?state=all` with PRs excluded returns 0).
- Main-branch CI history: of the 13 `push` runs on `main` listed since 2026-09-29, 4 failed:
  - Three failed **only** on `conformance against a live Neon endpoint`: runs 36528489181, 36596302787 and 36619371439.
  - The fourth is the intentional `workerd` red, run 37406385109.
  - The Neon job logs show the failing tests were `dcb_conformance::read_result_is_stable_under_concurrent_append` (ES-11) and `query_items_share_one_snapshot` (ES-12), panicking at `crates/happenstance-testkit/src/suite.rs:6147` and `:6250`.

**crates.io API**
- All seven published crates are at `max_stable_version 0.3.2`, last updated 2026-09-21.
- Lifetime downloads are 99–980 per crate.
- The reverse dependencies of `happenstance-core` are 6 crates, and every one is this workspace's own.
- These names are **absent** from the registry: `happenstance-sync`, `happenstance-sync-testkit` and `happenstance-ladybug`.

**In the tree**
- `crates/happenstance-cloudflare/src/event_store.rs:402,423` reads `400` / `30_000` on `main`.
- `crates/happenstance-neon/src/lib.rs:17-21` says the crate "ships carrying one conformance rule it does not pass".
- `crates/happenstance-sqlite/src/event_store.rs:512` captures `Handle::try_current().ok()`.
- `crates/happenstance-neon/src/event_store.rs:1475` defines `pub struct ProbeThenWriteStore`.
- `.kb/decisions/wi-40b321-*.md:8,59-60` records the sync-in-1.0 decision.
- `.kb/decisions/0066-what-1-0-promises.md:124-125,319-322` says the sync names are unclaimed and the soak has no time floor because nobody external uses the crates.
- `.kb/decisions/0072-*.md:117-119` says there is no `0.5.0`.
- `xtask/src/reserve.rs:110-112` and ADR-0078:158 say the ladybug reservation stands.
- Size counts:
  - xtask is 33,774 lines.
  - `happenstance-core` + `happenstance` `src` is 13,217 lines.
  - `docs/` is 649 lines.
  - 47 open questions are still open (`status: accepted`).

## Ranked risks

| # | Risk | Kind | Likelihood | Impact |
|---|---|---|---|---|
| 1 | Sync is inside 1.0 and is the least-built piece | scope / technical | High | High |
| 2 | The 1.0 schedule is long, under-counted, and already slipped 4–5x once | schedule | High | High |
| 3 | Bus factor of one, plus process overhead that outweighs the code | bandwidth | High | High |
| 4 | No external adopter; 1.0 would freeze an API nobody outside has used | adoption / credibility | High | Med-High |
| 5 | 0.4.0 is declared the last breaking window, but sync, runner and retention come after it | technical / sequencing | Medium | High |
| 6 | Neon fails ES-11/ES-12 intermittently, and its required check flakes on `main` | technical / quality | High (already happening) | Medium |
| 7 | Cloudflare's real-runtime limits sit below the specification's floor (fix pending merge) | technical | Low-Med (mitigating) | Medium |
| 8 | Record drift and unclaimed names | governance / supply | Medium | Low-Med |

### 1. Sync is inside 1.0, and it is the least-built piece (High / High)

**What the documents CLAIM**
- D-1 put `happenstance-sync` inside 1.0, overriding the recommendation, "accepting that if wrong: 1.0 slips about 17 working days on the least-settled piece" (`.kb/decisions/wi-40b321-*.md:8,59-60`; `runbook/roadmap.md:86-95`).
- ADR-0066 counts nine crates, sync and sync-testkit among them (`:118-125`).

**What the code SHOWS**
- `crates/happenstance-sync/Cargo.toml:3,12` is `publish = false`, "Not yet implemented".
- There is no `SyncRunner` anywhere.
- There is no `happenstance-sync-testkit` crate (`ls crates/`).
- Of the 35 SY index rows, 32 carry a dagger, meaning the rule must still be written. The other three have no rule name to dagger. **No executable check exists for any of the 21 SY clauses marked `[FROZEN]`** (map-sync §1.7).
- The two networked peers are `todo!()` stand-ins (`tests/real_peer_shapes.rs:110,114,194,209,218`).
- The only real ingest is a `cfg(test)` spike (`happenstance-sqlite/src/lib.rs:121-122`).
- ADR-0026 and ADR-0027 are unwritten.
- Phase 13 carries 17 of the 34 freezes still owed before 1.0 (`runbook/ledgers.md:270-315`).

**Why it ranks first.** This is the largest block of undesigned-in-practice work, and it is gated behind phases 17 and 18. The decision's own flip condition ("replication is the headline for first adopters") cannot be tested, because there are no first adopters (risk 4).

### 2. The 1.0 schedule is long, under-counted, and already slipped once (High / High)

**Estimates**
- Phase 17 went from 5–8 to **25–30** days at its start (`runbook/phases/17-breaking-window.md:285-290`; ADR-0072). That re-estimate followed a research pass that put the unsplit phase at about 275 hours.
- The roadmap gives "about 62–75 working days" (`roadmap.md:78`).
- That figure excludes phase 20, which phase 21 depends on (`README.md:110`) and which has no estimate ("Not yet made", `phases/20-docs-that-teach.md:51`). It also excludes the soak.
- After the 6 days already spent, the remaining critical path is about **52–64 working days, plus phase 20 and the soak**, by the plan's own figures.

**Progress so far**
- Phase 17 stands at 3 of 10 exit criteria (`17-breaking-window.md:257-283`).
- The heavy items are still ahead: the ES-11 fence spike on Neon, ES-17, ADR-0022 §9, `ProjectionId`, the codec, the SQL seam, VT-6, and the release with its semver trace table.

**Cadence (SHOWS)**
- 12 active commit days in 29 calendar days.
- Gaps of 8 and 6 days.
- 61 of 86 commits landed in the first week.

If "working days" means days on which work happens, the observed cadence of about 40% puts the plan's remaining figures at roughly two to three times their length in calendar time. That is arithmetic, not a forecast.

### 3. Bus factor of one, and process overhead that outweighs the code (High / High)

**Who does the work (SHOWS)**
- One human author.
- 109 `Co-Authored-By: Claude` trailers across 86 commits (map-governance §9).
- `CLAUDE.md` says the owner is new to idiomatic Rust.

**How heavy the apparatus is**
- xtask, which mostly polices documents, is 33.8k lines. The core contract plus the typed layer `src` is 13.2k lines.
- Governance prose runs to about 8.8 MB, against 1.7 MB of published library source (map-governance §9).
- 47 open questions remain open, and roughly 20 of them concern the bookkeeping machinery itself.
- `RUNBOOK.md` cannot shrink because about 2,250 line citations point into it.

**The records drift within days**
- `runbook/handover.md:15-16,40` still says PR #34 is unmerged.
- `handover.md:68` names ADR-0083, while `runbook/ledgers.md:20` gives the next free number as 0079, and so does PR #35.
- `log.md` has no entry for L6a or for phase 22's merge.
- `CLAUDE.md` says 202 clauses; 203 were counted.

Every change pays a lint, citation and record-keeping tax. If the owner's availability drops, nobody else can carry the project.

### 4. No external adopter: 1.0 would freeze an API no outsider has used (High / Med-High)

**What the evidence SHOWS**
- `happenstance-core` has no reverse dependencies outside the workspace (crates.io).
- No issue has ever been filed. The repo has 1 star and 0 forks.
- ADR-0066 itself says so, and drops the soak's time floor "because the registry shows no one external to find anything in it" (`0066-what-1-0-promises.md:319-322`).

**The paths a newcomer would use are thin or missing**
- The docs site has never deployed. Three of three `pages.yml` runs on `main` failed with "Ensure GitHub Pages has been enabled", which is an owner action (`phases/22-docs-site.md:84`).
- Phase 20 is at 0 of 4 exit criteria, with an empty session log.
- There is no projection guide in `docs/`.
- There is no 0.3 → 0.4 migration guide.

**Positioning disagrees with the 1.0 premise.** `README.md` never says "local-first", while D-1's premise is replication.

The result is that 1.0 can be technically correct and still not *credible*: no outside reader has ever confirmed it is usable, which is the outside-reader pass phase 21 itself requires.

### 5. 0.4.0 is declared the last breaking window, but sync, runner and retention follow it (Medium / High)

**What the documents CLAIM.** ADR-0072:117-119 and `handover.md:128-129` say a breaking item "goes back to phase 17 … not into a `0.5.0`".

**Why that is fragile**
- Phases 18, 13 and 14 all run **after** the 0.4.0 release.
- Phase 13 explicitly comes after 17 "because 17 may change the core surface a peer is built against" (`roadmap.md:70`).
- A core change that sync discovers late therefore has no planned home.

**Breaking questions still open inside the window**
- ADR-0022 §9: the runtime `Handle` captured at construction, at `sqlite event_store.rs:512` and `postgres event_store.rs:314`. Changing what `open` / `new` capture is a break, per `17-breaking-window.md:105-116`.
- VT-6 mint-per-open.
- Codec sealing.
- `ProjectionId` validation, together with SY-31's reserved `sync/` prefix.
- The SQL-seam statement type.
- ES-17.

**Also on the 1.0 surface.** `ProbeThenWriteStore`, a deliberately wrong store, is public API in a published crate (`neon event_store.rs:1475`). Unless something hides it, 1.0 would promise it.

### 6. Neon fails ES-11/ES-12 intermittently, and its required check flakes on `main` (High / Medium)

**What CI SHOWS**
- Three of the 13 recent `main` push runs were red **only** because of live Neon: 36528489181, 36596302787 and 36619371439.
- The failing rules were the ES-11 rule `read_result_is_stable_under_concurrent_append` and the ES-12 rule `query_items_share_one_snapshot`.

**What the crate CLAIMS.** It admits it ships failing a rule (`neon/src/lib.rs:17-21`).

**What is still owed.** The L8 fence spike has not started (`17-breaking-window.md:162`, unticked), and the job is a required check (rulesets).

**Possible outcomes**
- The spike works.
- ES-11 takes a named exception for one-shot HTTP.
- Neon leaves the promised set.

Each outcome is a 1.0 promise change. Until one lands, the flakes train the maintainer to re-run red builds.

### 7. Cloudflare's real-runtime limits sit below the specification's floor (Low-Med / Medium; mitigating)

**What is red on `main` (SHOWS)**
- The `workerd` job is red on `main` (run 37406385109).
- A real Durable Object allows only 5 compound terms and 100 bound parameters. Through the adapter that means 5 query items and 45 tags, against VT-23's floor of 128.
- The published constants on `main` still advertise 400 arms and 30,000 parameters (`cloudflare/src/event_store.rs:402,423`).

**The fix**
- PR #35 is **fully green, `workerd` included**, as of 2026-10-06. It uses `json_each` rendering.
- Once it merges, this risk drops to residuals:
  - eviction and hibernation are untested;
  - `workerd` is not a required check;
  - the vacuity control is owed (`handover.md:61-62`);
  - VT-23 is `[PROVISIONAL]`, and its resolution shapes Cloudflare's 1.0 claim.

### 8. Record drift and unclaimed names (Medium / Low-Med)

**The sync names are unclaimed.** `happenstance-sync` and `happenstance-sync-testkit` are absent on crates.io, while ADR-0066 promises both for 1.0. That is name-squat exposure on crates the charter promises.

**The ladybug reservation contradicts the registry.**
- ADR-0078:158 and `xtask/src/reserve.rs:110-112` CLAIM the `happenstance-ladybug` `0.0.0` reservation "stands, unyanked".
- crates.io SHOWS no such crate.

**Handover drift.** The stale handover and the ADR-number conflict from risk 3 mean the next session can start from a wrong plan.

## Strengths (what reduces risk)

- **The gate is green and broad.** Measured: 31 required steps, 2,965 tests passed and 0 failed (`gate-summary.md`). CI runs live Postgres, live Neon, a three-OS matrix, the MSRV and semver jobs, and a deployed Durable Object leg.
- **The contract is mostly frozen.** 152 of 203 clauses are `[FROZEN]`. The projection port is frozen (ADR-0063). Four adapters are conformant across genuinely different storage shapes.
- **The risk register is unusually honest.** D-1 names its own failure mode. ADR-0066 admits there are no external users. The Neon crate states the rule it fails. The `workerd` job was landed red on purpose rather than hidden.
- **The breaking work is being front-loaded deliberately.** ADR-0072 split additive work into 17b, and `Busy`, VT-10, `apply` and ADR-0028 have already landed in the window.
- **The workerd fix exists.** PR #35 is green and ready, so risk 7 is one merge from residual.

## What would move this to green

1. Re-decide D-1 with an evidence test, or ship 1.0 without sync and keep sync on a `0.x` line. This removes about 17+ days and the least-settled scope from the critical path. Alternatively, put a timebox on phase 13.
2. Claim the two sync names now, and correct the ladybug reservation record.
3. Recruit an outside user or reviewer before rc. Enable GitHub Pages. Write the projection guide and the migration guide.
4. Settle ES-11 on Neon (fence, or a named exception) and stop the required check from flaking.
5. Write down what happens when phase 13 or 18 needs a core break after 0.4.0, instead of "no 0.5.0".
6. Cut the record-keeping tax: fewer line-anchored citations and fewer governance lints on the critical path.
