# Assessment: implementation (per-crate completeness and production-readiness)

Commit `1f92d08` (== origin/main), 2026-10-06. Read-only audit. Sources are the maps `map-core.md`,
`map-adapters.md` and `map-sync.md`, the measured gate summary, and direct spot-checks in the worktree
and in GitHub Actions/REST. No cargo commands were run. The gate summary is treated as authoritative.

**Rating: AMBER.** **Rough completeness against the 1.0 target: about 65%.**

**Headline:** The contract, the conformance suite and the three server/embedded adapters (SQLite,
Postgres, Neon) are real, clean and verified against live databases in CI. Two parts are not ready.
The Cloudflare adapter fails the specification's 128-item query floor on the real platform, and
`main` is red because of it. Replication (`happenstance-sync`) is mostly unbuilt.

## Measured baseline
- Local gate at `1f92d08`: all 31 required steps green, with 2,965 tests passed, 0 failed and 288 ignored
  (`gate-summary.md`). Not exercised locally: 24 live-Postgres tests, 12 live-Neon tests, the workerd legs,
  cargo-hack, cargo-deny and the nightly docs.rs builds. These were skipped because the tools are absent, not because they failed.
- `main` CI run 37406385109 (push of `1f92d08`): every required job passed, including gate ×3, live Postgres,
  live Neon and MSRV. The non-required job `conformance under workerd, local and deployed` **failed**
  (`gh run view 37406385109`).

## Per-crate verdict (claims vs what code and tests show)

| Crate | src / tests LOC | What code and tests show | Verdict |
|---|---|---|---|
| happenstance-core | 8,190 / 4,519 | Both ports are frozen. `read` is non-async with the stream at the top level (`src/store.rs:184-188`). Both Send tests are present (`src/memory.rs:628,657`). Production code has 0 unsafe, 0 todo and 0 unwrap. | ~95%, sound |
| happenstance (typed layer) | 5,027 / 7,617 | It has a command loop with bounded `Retry`, codecs (JSON, postcard, CBOR), tuple boundaries and a test DSL. The runner is behind `unstable-projection` (`Cargo.toml:148`). `Projection::apply` is still **synchronous** (`src/runner.rs:95-99`), and its rustdoc still argues for that design (`:50-59`) even though ADR-0074 is accepted the other way. There is no `Delivered` in src. | ~60%. The command side is usable; the projection side is unstable. |
| happenstance-testkit | 16,652 / 22,498 | 95 event-store rules (`registry.rs:94`), 17 projection rules (`projection.rs:1937-1968`, counted), 6 concurrency rules and 1 model rule. It also has a mutation registry of 84 + 18 named wrong implementations. Some rules are unwritten: ES-38/39/40, CF-27, and all SY rules. | ~85%, the strongest asset |
| happenstance-sqlite | 6,868 / 6,272 | Both roles are implemented, and every family runs in the gate on three OSes. It is the only real store that runs the model (property) rule. Weak points: one connection behind a Mutex per handle, tokio-bound (`Handle::try_current().ok()` at `src/event_store.rs:512`), and ADR-0022 §9 is still open. | ~85%, production-viable for single-node use |
| happenstance-postgres | 4,093 / 2,746 | The required `live-postgres` job is green against `postgres:17.10`. Both projection stores are implemented. The model rule is skipped because `READ_YOUR_OWN_WRITES` is declined. `LivePostgresProjectionStore`, the store the projection port was frozen against, is **not run in CI** (`grep -- '--test ' ci.yml` has no `live_projection`). | ~85%, beta-grade |
| happenstance-neon | 4,900 / 1,857 | The required `live-neon` job is green. The crate itself says it ships **failing ES-11 intermittently** (`src/lib.rs:17-24`). `ProbeThenWriteStore`, a deliberately lost-update store, is a `pub struct` in a public module (`src/event_store.rs:1475`; `lib.rs:147`). | ~75%, with a known consistency hole |
| happenstance-cloudflare | 8,758 / 3,649 | Event store only; there is no `ProjectionStore` impl (grep returns nothing). Under real workerd it passes 96 of 97 rules locally and 95 of 96 deployed. VT-23 fails with "too many SQL variables" (CI log). The published constants (`MAX_QUERY_ARMS_PER_STATEMENT = 400`, `MAX_QUERY_PARAMETERS_PER_STATEMENT = 30_000`) assume stock SQLite. | ~60%, not production-ready for non-trivial queries |
| happenstance-sync | 1,573 / 1,273 | `publish = false`. It contains the port traits, the wire envelope and an in-memory peer. There is no `SyncRunner` (grep returns nothing), no `happenstance-sync-testkit` crate, and no real peers (5 `todo!()` stand-ins in `tests/real_peer_shapes.rs`). Ingest is a `cfg(test)` spike in sqlite (`happenstance-sqlite/src/lib.rs:121-122`). There are 0 executable SY rules. | ~10-15% |

## Findings

1. **HIGH: the Cloudflare adapter fails the 128-item query floor on a real Durable Object, and `main` is red.**
   CI run 37406385109, job `conformance under workerd`: "Tests 1 failed | 96 passed (97)". The failure is
   `too many SQL variables … SQLITE_ERROR`, raised in VT-23's rule. Measured platform walls are 5 compound terms
   and 100 bound parameters (`experiments/durable-object-limits/results/`). The adapter still advertises 400 arms and 30,000
   parameters (`crates/happenstance-cloudflare/src/event_store.rs:402,423`). The gate stayed green only because
   its Node shim does not enforce the caps. The workerd job is not a required check (ruleset 22926481), so the
   merge went through red. The remedy, lane L6b, exists as **open PR #35** (`lane/p17-workerd-green`), and its CI run on
   2026-10-06T05:37Z shows workerd=success. It is not merged at the audited commit.
2. **HIGH: replication is a skeleton, so the local-first half of the vision does not exist as working code.**
   `happenstance-sync` has no runner, no testkit crate, no networked peer and no shipped ingest. All 35 SY clauses
   lack an executable rule, 32 of them daggered in `spec/SPECIFICATION.md`. Phase 13 is "not started" and depends on 17
   and 18 (`runbook/README.md:104`). There is also no store that runs on wasm32 outside Cloudflare (phase 19a/b not started).
3. **MEDIUM: the typed projection runner is still gated, and its shipped shape contradicts an accepted ADR.**
   `apply` is synchronous and takes a bare event (`crates/happenstance/src/runner.rs:95-99`), and its rustdoc argues
   against async (`:50-59`). ADR-0074 (accepted) makes it async with a `Delivered<E>`. Phase 18 is not started
   (`runbook/README.md:103`). The runner also has no observability, no lease or fencing, and no default chunk size
   (its own docs, `runner.rs:445-494`). Applications therefore have no stable read-model path from the typed layer.
4. **MEDIUM: Neon ships knowingly failing ES-11.** `crates/happenstance-neon/src/lib.rs:17-24` says
   `read_result_is_stable_under_concurrent_append` "reddens intermittently". The fence spike, phase 17's "ES-11 record",
   is unchecked (`runbook/phases/17-breaking-window.md:162`). A different reading: the last 15 CI runs (2026-10-01 to 2026-10-06)
   all had live-neon=success, so the flake rate is low or the race window is narrow. No rate is recorded anywhere.
5. **MEDIUM: Neon exports a known-wrong store as public API.** `pub struct ProbeThenWriteStore` (`crates/happenstance-neon/src/event_store.rs:1475`)
   sits in `pub mod event_store` (`lib.rs:147`) without `#[cfg(test)]` or `#[doc(hidden)]`. It is a lost-update
   negative control that a 1.0 release would promise.
6. **MEDIUM: ADR-0022 §9, runtime-handle capture, is open, and its remedy may break constructors in 0.4.0.**
   `Handle::try_current().ok()` is captured at construction (`happenstance-sqlite/src/event_store.rs:512`,
   `projection_store.rs:234`, and the postgres equivalent). The phase-17 item and its exit criterion are unchecked
   (`17-breaking-window.md:105,267`). No reproduction test exists yet.
7. **MEDIUM: coverage gaps against real databases.** The model (property) rule runs only on SQLite and memory.
   Postgres and Neon skip it because they decline `READ_YOUR_OWN_WRITES`. `LivePostgresProjectionStore` and `poll_shape` are never run by CI
   (`ci.yml` `--test` list). Only one Postgres version (17.10) is tested, and benchmarks are only compiled in CI. Fork PRs to
   live-neon "prove NOTHING" (`ci.yml:716-718`).
8. **LOW: stale records next to the code.** `crates/happenstance-postgres/tests/postgres_conformance.rs:153` still says
   `append`, `head` and `contains_event_id` "are still `todo!()`". In fact CI runs 108 of them green. CLAUDE.md calls `happenstance` "a five-line
   facade", but it has 5,027 src lines. CLAUDE.md's "101 of 101" predates the current 95 + 6 + 1. `map-adapters.md` says the projection family has 20 rules, but the
   enumeration has **17**; the 21/19 CI counts include other tests.
9. **LOW: some functionality is missing outright.** Cloudflare has no projection store. There is no graph or non-SQL projection store since
   ladybug was retired (ADR-0078). There is no `DomainEvent` derive (ADR-0033). Retry has no backoff.

## Strengths
- The core contract is frozen and clean. Across about 30k src lines in core, typed layer and testkit there is no unsafe, no todo and no production unwrap.
  The binding constraints (no async_trait, non-async `read`, both Send tests) are enforced by tests and `deny.toml`.
- The testkit can be falsified: 84 event-store and 18 projection mutants, a meta-test that every rule has a mutant, and
  conformant controls with gapped positions.
- Three adapters run the full suite against real storage in required CI jobs: SQLite on 3 OSes, Postgres 17.10 at 108/108,
  and Neon at 108/108.
- The adapters disclose their own limits honestly (Neon ES-11, the Postgres visibility frontier, Cloudflare's "thinnest evidence").
- The workerd harness is a real, strict instrument. It is the check that exposed the Cloudflare limits failure, and a fix (PR #35) is already green on its branch.
- The local gate is green at 2,965 tests with 0 failures. Five runnable examples exit 0.
