# Assessment: vision-fit

Commit `1f92d08` (== origin/main), 2026-10-06. Read-only. No cargo was run; the measured gate is taken from `gate-summary.md`. CI state was read through the GitHub MCP.

**Rating: AMBER. About 55% of the way to ADR-0066's 1.0.**

**Headline:** The part of the vision about a contract and a conformance suite is delivered and holds up. The part about local-first sync, which the owner made the 1.0 headline, is still mostly design on paper. The edge (Cloudflare) promise has just collided with real platform limits.

## What the vision is (sources)

- **Founding motive.** "a local-first native application syncing with SQLite inside a Durable Object behind a Rust Worker" (`references/adr/0001-async-port-flavours.md:45-46`; `crates/happenstance-sync/src/lib.rs:54-57`).
- **Founding front page** (quoted, since the original README is not in git):
  - "fast, efficient"
  - "with batteries"
  - "widely usable, trusted, respected"
  - Sources: `references/evaluation/review-docs-adr.md:369-374` and `ARCHITECTURAL-EVALUATION.md:885`.
- **Today's README.** It positions the project on DCB plus a published conformance suite (`README.md:3-6`, `:359-361`).
- **The 1.0 definition** (ADR-0066, `.kb/decisions/0066-what-1-0-promises.md`) has four parts:
  - **The crate set.** Nine crates, including `happenstance-sync` and `happenstance-sync-testkit` (§1).
  - **Clause dispositions.** Every non-frozen clause gets exactly one disposition (§2).
  - **Cloudflare.** Its 1.0 claim is conformance on workerd, not on the shim (§6).
  - **The release candidate.** A soak with five conditions, one of which is an outside-reader pass (§8).
- **Sync inside 1.0.** D-1 (`.kb/decisions/wi-40b321-…md`) put sync inside 1.0, "on the premise that replication is the headline for first adopters". The owner made this call overriding the recommended option A.

## Promise ledger (what documents CLAIM vs what the code and tests SHOW)

| Promise | Status | Evidence |
|---|---|---|
| DCB contract + published conformance suite ("the best idea") | **Met, strengthened** | The gate was green: 2,965 tests passed, 0 failed (`gate-summary.md`). The spec has 152 FROZEN clauses (`spec/SPECIFICATION.md:228-230`). On main, CI run 37406385109 has the `live Postgres` and `live Neon` conformance jobs green. |
| Storage-agnostic across shapes that disagree | **Met for 4 SQL shapes** | 7 crates are on crates.io at 0.3.2 (registry API). Postgres (unserialised writers) and Neon (no connection) are the far ends of the axis. |
| `!Send` / wasm32 / Durable Object | **Kept in code; failing on the real runtime** | In the `workerd` job of run 37406385109, the local and deployed legs fail: 97 executed, 96 passed. VT-23 fails (`runbook/handover.md:40-50`). The shipped `MAX_QUERY_ARMS_PER_STATEMENT = 400` and `MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000` (`crates/happenstance-cloudflare/src/event_store.rs:402,423`) sit against measured limits of 5 compound terms and 100 parameters. The fix is open as PR #35. |
| Local-first sync / replication (the 1.0 headline per D-1) | **About 10–15% built** | The crate is `publish = false` with "Not yet implemented." (`crates/happenstance-sync/Cargo.toml:3,12`). There is no `SyncRunner`, and there is no `happenstance-sync-testkit`. 0 of 35 SY clauses have an executable rule. Phase 13 is "not started" (`runbook/README.md:104`). |
| Local-first positioning | **Drifted** | `grep -i 'local-first\|offline' README.md docs crates/*/README.md` returns 0 hits. The local-first/edge persona appears in HS-I0006 but is missing from HS-I0007. |
| Graph projection store (Ladybug) | **Dropped** | ADR-0078; `Cargo.toml` `exclude`. The crates.io name does not exist, even though ADR-0078 claims it stays claimed (`.kb/decisions/0078-…md:25,158`). |
| "with batteries" | **Mostly delivered; the runner is still gated** | `unstable-projection` remains (`crates/happenstance/Cargo.toml:148`), and phase 18 has not started. No page in `docs/` mentions projections (`grep -ci projection docs/*.md` → all 0). Snapshots, tracing and live subscriptions are deferred past 1.0. |
| "fast, efficient" | **Withdrawn honestly** | "This README makes no performance claim, and that is deliberate" (`README.md:285`). `benchmarks/results/GRADES.md` is a measured record kept out of the gate (CF-34). |
| "trusted, delight" | **Unevidenced** | Phase 20's outside-reader evidence has not been built. The Pages site has never deployed (map-docs §4). |

## Findings

1. **HIGH: the 1.0 headline is the least-built part.** D-1 puts sync in 1.0 because "replication is the headline for first adopters". The code has none of the following:
   - a runner;
   - a sync testkit;
   - networked peers (the `todo!()` stand-ins in `crates/happenstance-sync/tests/real_peer_shapes.rs:110-218` are deliberate, compile-only shapes);
   - shipped ingest (`crates/happenstance-sqlite/src/lib.rs:121-122` is `cfg(test)`).

   In the dispositions ledger, 18 `freeze-by-13` mentions are still outstanding. The roadmap sequences sync after 17, 17b and 18 (`runbook/README.md:101-104`).
2. **HIGH: ADR-0066 §6 (Cloudflare conformance on workerd) is currently unmet, and the published adapter over-promises capacity.** CI on main is red on the `workerd` job by design (run 37406385109, job 112084786585). The published constants exceed the measured platform walls by roughly 80× (400 arms against 5) and 300× (30,000 parameters against 100). The Cloudflare README on main says workerd "runs them too" (`crates/happenstance-cloudflare/README.md:115-123`), lists the walls, and does not say that a rule fails. PR #35 (L6b) is open to fix this.
3. **MEDIUM: the positioning has drifted from the founding motive.** The README and docs never say "local-first", while ADR-0001 and D-1 make it load-bearing. The local-first/edge persona was dropped between HS-I0006 and HS-I0007.
4. **MEDIUM: Ladybug was dropped, along with the only non-SQL projection evidence, and a record contradicts the registry.** ADR-0078 says the 0.0.0 reservation "stays claimed", but the crates.io API says the crate does not exist. The sync names are also unclaimed.
5. **MEDIUM: there is still no browser or device store.** Native host SQLite covers the "native application" half of ADR-0001. Phases 19a/19b (SQLite on wasm32) are not started and sit outside 1.0, yet the seed says four of the six scenarios put SQLite on a device (`references/seeds/sqlite-on-wasm.md:44-53`).
6. **MEDIUM: the batteries are incomplete at the projection layer.** The runner is still behind `unstable-projection` (phase 18 not started), and no narrative docs page covers projections. The "with batteries" tagline survives at `crates/happenstance/Cargo.toml:3`.
7. **MEDIUM: the 1.0 timeline is long and still growing.**
   - The roadmap now says "about 62–75 working days" (`runbook/roadmap.md:78`). It was 34–43 when D-1 was taken.
   - Phase 17 grew from 5–8 days to 25–30.
   - The rc condition "outside-reader pass" depends on phase 20 work that has not been built.
8. **LOW: stale front-door and status claims.**
   - `crates/happenstance-core/README.md:10-14` still says "93-rule" and lists "an embedded graph database".
   - `crates/happenstance/README.md:11,29` says "Four ship at `0.2.0`".
   - `README.md:15-21` counts five adapters, including the retired one.
   - `runbook/handover.md` says PR #34 is "Not merged", but it is HEAD.
9. **INFO: the performance claim was withdrawn properly.** The project replaced it with a measured, reproducible record that is barred from gating merges. This is a healthy re-scope, not a dropped promise.

## Strengths

- The core differentiator, a contract plus a published, mutant-checked conformance suite, is delivered. Two live external backends (Postgres and Neon) are green in CI on main.
- Seven crates are published at 0.3.2. The MSRV, licence and security promises are kept.
- ADR-0066 is a concrete 1.0 definition that a lint can enforce: crates are named, and every clause gets a disposition.
- The `!Send`/wasm32 constraint is preserved in code. The edge target is now tested on real workerd, which is why the gap above was found.
- The project is honest about scope: the perf claims were withdrawn, the shim-versus-workerd caveat is stated, and the dropped Ladybug crate is recorded with what it loses.
