---
item: "HS-S0034"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — event_store_benchmarks! in the testkit, provably not conformance

## Findings Ledger

**Eight of eight ACs satisfied, each by real reachable behaviour with a real
test.** 18 integration tests in the mount, 5 unit tests in the module, 23
doctests, both feature powersets on both targets, and the `wasm32` powerset run
explicitly because `--fast` skips the only step that compiles `bench` there.

**Mount point:** `crates/happenstance-testkit/tests/memory_benchmarks.rs` — the
harness is *executed* by `cargo test -p happenstance-testkit --features bench`
and by the gate's own `cargo test --locked --workspace --all-features` step, not
merely compiled. Composition root for the capability itself:
`crates/happenstance-testkit/src/lib.rs:315-323` (the gated `pub mod bench;`)
and `crates/happenstance-testkit/Cargo.toml`'s `[features]`.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — the adapter author inherits benchmarks the way they inherit conformance | **Met** | One line — `event_store_benchmarks!(MemoryFixture::new())`, `tests/memory_benchmarks.rs:82` — expands to three tests that reach the store only through `Fixture::connect`. 18/18 passing; the generated tests name themselves `dcb_benchmarks::append_throughput` and siblings. `all_three_scenarios_complete_against_the_reference_fixture` (:133) calls the same three directly, so a record nobody inspected cannot pass. No measurement crate anywhere: `[dependencies]` unchanged, `Cargo.lock` untouched |
| **AC-002** — "benchmarks are not the bar" is checkable, not trusted | **Met** | `git diff --name-only HEAD` over `suite.rs`, `registry.rs`, `model.rs`, `concurrency.rs`, `projection.rs`, `xtask/`, `spec/` and `Cargo.lock` returns **empty**. 89 registered rule names either side. `registry::no_orphan_rules` green; `cargo xtask spec-trace` reports the same 112 rules and *no problems found*; `cargo xtask lints` reports the same five counts. The family carries its own enumeration, `bench.rs:726` |
| **AC-003** — the edge developer keeps their runtime, including under `--all-features` | **Met** | `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]`, `lib.rs:315-323`, copied from `model` rather than written fresh. `cargo hack check … --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` — 12/12 green, four of them with `bench` on. The family binds `Fixture` and never `SendEventStore`; no bound, dependency or item reaches the `!Send` flavour |
| **AC-004** — no measurement dependency is imposed, and the timer is the caller's | **Met** | `emit_timed_csv!` (`tests/memory_benchmarks.rs:51`) is written in the *caller's* crate, uses `std::time::Instant` — which the testkit's `src/` may not spell — and drives all three scenarios as `dcb_benchmarks_timed_csv::*`. The shipped blocking emitter is the second. `git diff` of `Cargo.toml` is one `[features]` line; `Cargo.lock` is unchanged |
| **AC-005** — the decision-maker's three measurements, at their own numbers | **Met** | `BenchmarkParams` (`bench.rs:106-186`) hoisted behind `__benchmark_params()` exactly as the fixture is. Three parameter sets in the mount: SMOKE (8/4/16), `new(6, 5, 21)`, `new(12, 7, 33)`. `the_append_scenario_appends_the_callers_batch_size` asserts `events == n` at n = 1 and 9; the replay record carries **both** a `replay-all` and a `replay-tagged` pass, the filtered one a proper subset |
| **AC-006** — a run where nobody collides is visible as such | **Met** | `Outcome` keeps `Committed` / `Rejected` (`ConditionViolated`) / `Refused` (`ExceedsStoreLimit`) / `Failed` apart, the way `concurrency::Attempt` does; `is_well_formed` is `attempts == committed + rejected + refused + failed`. `a_contended_run_accounts_for_every_contender` (:242) asserts `committed == 1`, `rejected == k - 1`, `failed == 0` and the sum, at k = 2 and k = 8 against the serialising reference store. No timing asserted |
| **AC-007** — a benchmark result can never turn a merge red | **Met** | `cargo run -p xtask -- lint-clock`: *11 file(s) … read no clock* — 11 rather than 10, so `bench.rs` was in scope. No threshold, no watchdog, no `Duration` in the record at all. Non-occlusion: one line per scenario, asserted twice (`a_record_summarises_itself_in_one_line`, `bench::tests::a_summary_is_one_line`) |
| **AC-008** — the reader meets the boundary before the macro | **Met** | Module docs open with *a benchmark is not a conformance rule* and *the testkit never reads a clock* (`bench.rs:1-82`); crate docs `lib.rs:146-171`, with `bench` in backticks and **not** an intra-doc link, and the D13 paragraph widened to three modules; README `:136-176`, a family table whose benchmark row reads *Checks: nothing — it measures / Can fail a merge: no*; `CHANGELOG.md:25-58`. Both `cargo doc` runs green under `-D warnings` |

**Deferred: nothing.** No AC is blocked, no test is skipped, no behaviour is
stubbed. The one thing this story deliberately does **not** do is write ADR-0022
or touch `crates/happenstance-sqlite/` — that is the slice-mate's, and a diff
doing both would destroy the ordering AC-013 exists to prove.

## Acceptance

All eight `AC-###` rows in `_ledger.md` are `satisfied: true` with cited
evidence — a `file:line` and a named passing test each. `redkiln verify --grain
story` reads that block.

Error conditions, all six answered in code and four of them tested:

| EC | Answered by |
| --- | --- |
| EC-001 (`bench` reaching wasm32) | `lib.rs:315-323`'s double `cfg`; the wasm32 powerset run explicitly, 12/12 |
| EC-002 (a clock under `src/`) | `lint-clock` green over 11 files; the record carries counts and no duration |
| EC-003 (degenerate parameters) | `BenchmarkParams::new` refuses each by name; three `#[should_panic(expected = "<name>")]` tests |
| EC-004 (a stated ceiling below *n*) | `BatchCeilingFixture` (`tests/memory_benchmarks.rs:108`) states `MAX_EVENTS_PER_BATCH = Some(2)`; the append is counted `refused`, never `committed`, and never panics |
| EC-005 (transport failure is not a rejection) | `Outcome::of_error` (`bench.rs:203-215`); `bench::tests::the_three_refusal_kinds_stay_apart` |
| EC-006 (an emitter that is not in scope) | The path is substituted verbatim (`bench.rs:846-853`); a misspelt emitter is a compile error naming the missing item in the caller's crate, with no fallback |

## Knowledge Harvest

Three things worth promoting at closeout, none of them settled here:

1. **A fourth macro family costs nine `file:line` citations.** Inserting a module
   declaration near the top of `lib.rs` moved every line below it and failed
   `lint-constitution`, which is part of the story gate. The lint is doing its
   job; what is durable is the *shape* of the cost, and it is an argument for
   anchor-text-relative citations over absolute line numbers in
   `standards/rust/`. Candidate for a `playbook` atom, not a decision.
2. **CF-33 and CF-23 land on the same seam, and that is a design result rather
   than a workaround.** "The testkit may not read a clock" and "the wrapper is a
   parameter" independently force the same architecture: the harness produces
   counts, the caller's emitter produces durations. This is the sentence a
   future benchmark family in any other port crate should start from.
3. **Interleaving reaches a real rejection mix without a `Send` bound.** `k`
   futures built before any is polled, driven round-robin on one thread, gives
   `committed == 1, rejected == k - 1` against a serialising store — which is
   what the concurrency family buys with `F::Store: EventStore + Send`. The
   limits are honestly narrower (an adapter with real I/O interleaves
   differently, and OS-thread contention is still the emitter's to supply), and
   both belong in whatever atom records the two-flavour discipline.

Input handed to the slice-mate (**HS-S0035**), per the spec's instruction that a
finding goes to the ledger and the slice-mate rather than into `.kb/`:

- The public entry point is `event_store_benchmarks!` with five arms; the one an
  experiment crate wants is
  `(mod_name = …, emit = …, params = …, fixture = …)`.
- Scenario names, which are also the generated test names:
  `append_throughput`, `conditional_append_under_contention`,
  `replay_with_and_without_a_tag_filter`.
- The record reports counts only. A duration is the emitter's; `results/` tables
  come from an emitter like `tests/memory_benchmarks.rs:51`'s, and warm-up and
  repetition belong there too.
- `BenchmarkPass::rejected()` is the `ConditionViolated` count ADR-0012 asked
  for, and `refused()`/`failed()` are deliberately not it.
