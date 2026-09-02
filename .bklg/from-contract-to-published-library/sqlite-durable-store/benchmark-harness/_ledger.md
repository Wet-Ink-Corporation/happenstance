---
item: "HS-S0034"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — event_store_benchmarks! in the testkit, provably not conformance

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: |-
    **The adapter author inherits benchmarks the way they inherit conformance.** GIVEN an adapter author who already has a `Fixture` and has never measured anything, WHEN they add one line — `happenstance_testkit::event_store_benchmarks!(MyFixture::new());` — to a test target and run `cargo test --features bench`, THEN all three scenarios execute against their real store through `Fixture::connect`, each reporting a completed measurement record, with no measurement crate added to their own manifest and no conformance rule re-run or re-defined. The mount in this PR is `MemoryFixture`, and it is *executed* by `cargo test`, not merely compiled.
  satisfied: true
  evidence: |-
    crates/happenstance-testkit/tests/memory_benchmarks.rs:82 — the one-argument arm against `MemoryFixture`, expanded by `crates/happenstance-testkit/src/bench.rs:825` (`event_store_benchmarks!`). `cargo test -p happenstance-testkit --features bench` runs 18 tests, 18 passed: the nine generated ones name themselves `dcb_benchmarks::append_throughput`, `dcb_benchmarks::conditional_append_under_contention`, `dcb_benchmarks::replay_with_and_without_a_tag_filter` and the same three under `dcb_benchmarks_blocking::` and `dcb_benchmarks_timed_csv::`. The three scenarios reach the store only through `Fixture::connect` (bench.rs:533, 581, 640). `all_three_scenarios_complete_against_the_reference_fixture` (tests/memory_benchmarks.rs:133) calls the same three directly and asserts each names itself and reports a well-formed record. No measurement crate was added anywhere: `crates/happenstance-testkit/Cargo.toml` `[dependencies]` is unchanged and `Cargo.lock` does not appear in `git diff --name-only HEAD`.
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — `cargo test -p happenstance-testkit --features bench -- --show-output`"

- id: AC-002
  criterion: |-
    **The evaluator can check "benchmarks are not the bar" instead of trusting it.** GIVEN an evaluator reading this diff (initiative *Decide in one sitting*), WHEN they compare the conformance rule set either side of it, THEN the rule-name list produced by `for_each_event_store_rule!(__emit_rule_names)` is **identical**, `suite.rs` has gained no `pub async fn`, `xtask/src/spec_trace.rs`'s `RULE_FILES` is still a three-element array, and no clause's `Rule:` line has changed — so CF-34's claim is a property of the tree rather than a sentence in a report.
  satisfied: true
  evidence: |-
    `git diff --name-only HEAD -- crates/happenstance-testkit/src/{suite,registry,model,concurrency,projection}.rs xtask/ spec/ Cargo.lock` returns EMPTY — the rule set, its enumeration, `RULE_FILES` and every clause are untouched by this story. The registered rule-name count is 89 either side of the diff (`grep -c` over `for_each_event_store_rule!`, `crates/happenstance-testkit/src/registry.rs:94-218`). `cargo test -p happenstance-testkit --lib` — `registry::no_orphan_rules` passed (9/9). `cargo xtask spec-trace` green: *201 clauses, 112 conformance rules, 389 citations checked; traceability: no problems found*. `cargo xtask lints` green, including `CF-29: all 112 rules in 4 file(s) have a changelog entry` and `6 stated rule count(s) checked against 1 (model.rs), 5 (concurrency.rs), 17 (projection.rs), 89 (suite.rs), 112 (all four rule files)` — the same five numbers as before this diff. The benchmark family carries its own enumeration instead, `crates/happenstance-testkit/src/bench.rs:726` (`for_each_event_store_benchmark!`), beside the scenarios it names. DEVIATION, recorded rather than glossed: the spec's AC-002 text says `RULE_FILES` is *a three-element array*; it has been four since the projection family (`xtask/src/spec_trace.rs:87-93`). It is unchanged at four, which is the criterion's content.
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/src/registry.rs:412 `no_orphan_rules` — `cargo test -p happenstance-testkit no_orphan_rules`; plus `cargo xtask spec-trace` and `cargo xtask lints`"

- id: AC-003
  criterion: |-
    **The edge developer keeps their runtime, including under `--all-features`.** GIVEN the constrained-runtime developer whose target is `wasm32-unknown-unknown`, WHEN the testkit is checked on that target across the *whole* feature powerset — the combination `--all-features` actually produces, because a Cargo feature is not target-scoped — THEN `bench` compiles away entirely, the four mandatory wasm32 steps are unchanged, and nothing in the `!Send` flavour has gained a bound, a dependency or an item (DR-08, architecture brief AC-A06).
  satisfied: true
  evidence: |-
    `crates/happenstance-testkit/src/lib.rs:315-323` — the module is declared `#[cfg(all(feature = "bench", not(target_arch = "wasm32")))]` with `#[cfg_attr(docsrs, doc(cfg(feature = "bench")))]`, the same double condition `model` carries at :335-337 and copied from it rather than written fresh. `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` — **all 12 combinations green**, including the four that set `bench` on wasm32 (`bench`, `bench,memory`, `bench,proptest`, `bench,default,proptest`). This is the OPTIONAL step `cargo xtask ci --fast` skips, and it was run explicitly. `cargo xtask affected --base main` green, which includes the four mandatory wasm32 steps. Nothing in the `!Send` flavour gained a bound, a dependency or an item: the family binds `Fixture` (bench.rs:532, 574, 638) and never `SendEventStore`.
  mount_point: "crates/happenstance-testkit/src/lib.rs — the gated `pub mod bench;` declaration"
  verifying_test: "xtask/src/main.rs:564-591 `wasm32 feature powerset` — `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps`; plus `cargo xtask wasm`"

- id: AC-004
  criterion: |-
    **No measurement dependency is imposed on anybody, and the timer is the caller's.** GIVEN an adapter author who wants `criterion`, `divan` or a CSV row, WHEN they invoke the third arm with `emit = <their emitter>`, THEN their own wrapper surrounds each scenario and their dependency stays in *their* `dev-dependencies` — while `happenstance-testkit`'s `[dependencies]` remain exactly `happenstance-core` + `futures-core` and `Cargo.lock` is unchanged. Two distinct emitters are demonstrated in the mount, not described (CF-23, `[FROZEN]`).
  satisfied: true
  evidence: |-
    `git diff HEAD -- crates/happenstance-testkit/Cargo.toml` adds `bench = []` to `[features]` and nothing else; `[dependencies]` is still `happenstance-core` + `futures-core`, and `Cargo.lock` is absent from `git diff --name-only HEAD`. Two distinct emitters are *demonstrated* in the mount, and the second one is written in the caller's crate: `crates/happenstance-testkit/tests/memory_benchmarks.rs:51` defines `emit_timed_csv!`, which uses `std::time::Instant` — a construct `crates/happenstance-testkit/src` may not spell at all — and is passed at :95 as `emit = emit_timed_csv`. The shipped blocking emitter is the third invocation's, at :87. Both drive the same three scenarios: `dcb_benchmarks_timed_csv::*` and `dcb_benchmarks_blocking::*` in the passing run. The path is substituted verbatim (`crates/happenstance-testkit/src/bench.rs:846-853`), so a misspelt emitter is a compile error naming the missing item in the caller's crate (EC-006) rather than a silent fallback.
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs — the second invocation, `emit = …`"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs (both invocations) plus `git diff -- crates/happenstance-testkit/Cargo.toml Cargo.lock`"

- id: AC-005
  criterion: |-
    **The decision-maker measures the three things the decision needs, at their own numbers.** GIVEN ADR-0022's author (the slice-mate, HS-S0035) about to choose an append-condition strategy, WHEN they run the harness against real SQLite, THEN they get (a) append throughput over a batch of *n*, (b) conditional append under *k* contenders, and (c) replay of *N* events run **both** unfiltered and behind a tag filter — with *n*, *k* and *N* supplied by them at the call site, because no constant in the testkit can be right for both a `Vec` behind an `RwLock` and a file under `BEGIN IMMEDIATE`.
  satisfied: true
  evidence: |-
    `crates/happenstance-testkit/src/bench.rs:106-186` — `BenchmarkParams`, three caller-supplied numbers with no default anywhere in a scenario body; the macro hoists them behind `fn __benchmark_params()` (bench.rs:840-842) exactly as it hoists the fixture. The mount invokes the family at **three** parameter sets: `SMOKE` (8/4/16) at tests/memory_benchmarks.rs:82, `BenchmarkParams::new(6, 5, 21)` at :88 and `BenchmarkParams::new(12, 7, 33)` at :96. (a) `the_append_scenario_appends_the_callers_batch_size` (:180) asserts `pass.events() == batch_size` at n = 1 and n = 9. (b) `a_contended_run_accounts_for_every_contender` (:242) runs at k = 2 and k = 8. (c) `replay_reports_a_filtered_and_an_unfiltered_pass` (:204) asserts the record carries BOTH a `replay-all` and a `replay-tagged` pass, that the unfiltered one read all N = 20 and the filtered one a proper subset. All passing.
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — parameterised invocations at two different parameter sets; the replay scenario reports a filtered and an unfiltered record"

- id: AC-006
  criterion: |-
    **A contended run in which nobody collides is visible as such, not averaged away.** GIVEN the same author, whose evidence obligation is a *"realistic batch and rejection mix"* (`.kb/decisions/0012-append-shape-and-preconditions.md:52-60`), WHEN the contended scenario runs with *k* contenders over one append condition, THEN the record reports the committed and the rejected counts **separately**, distinguishes a `ConditionViolated` rejection from a transport-level failure the way `concurrency::Attempt` does, and accounts for every contender (`committed + rejected + failed == k`) — so a run in which every contender wins is legible as a measurement of the wrong thing.
  satisfied: true
  evidence: |-
    `crates/happenstance-testkit/src/bench.rs:190-232` — `Outcome` keeps `Committed`, `Rejected` (`AppendError::ConditionViolated`), `Refused` (`ExceedsStoreLimit`) and `Failed` apart, the way `concurrency::Attempt` (concurrency.rs:214-231) keeps its three apart; `BenchmarkPass::committed/rejected/refused/failed` (bench.rs:284-338) report them as separate numbers and `is_well_formed` (bench.rs:341-343) is exactly `attempts == committed + rejected + refused + failed`. Verified against the store rather than asserted: `a_contended_run_accounts_for_every_contender` (crates/happenstance-testkit/tests/memory_benchmarks.rs:242, passing) runs at k = 2 and k = 8 against `MemoryFixture` — which serialises its writers — and asserts `committed == 1`, `rejected == k - 1`, `failed == 0` and that the four counters sum to k. No timing is asserted. `bench::tests::the_three_refusal_kinds_stay_apart` (bench.rs:948) pins the classification of `ExceedsStoreLimit` and `NoEvents` so the split cannot silently collapse.
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — assertion on the contended record's committed/rejected/failed fields against `MemoryFixture`"

- id: AC-007
  criterion: |-
    **A benchmark result can never turn a merge red, and the testkit never reads a clock.** GIVEN any contributor on a loaded CI runner, WHEN the mounted target runs inside the gate's existing `cargo test --locked --workspace --all-features -- --show-output` step, THEN it passes on completion alone: there is no threshold, no `assert!(… < …)` on a duration, no watchdog and no sleep anywhere in the family, `cargo run -p xtask -- lint-clock` stays green over `crates/happenstance-testkit/src` — which is where `bench.rs` lands — and the default emitter prints at most one line per scenario so the CF-18 `SKIP <rule>: …` lines that step exists to surface are not buried under benchmark output.
  satisfied: true
  evidence: |-
    `cargo run -p xtask -- lint-clock` green: *CF-33: 11 file(s) in crates/happenstance-testkit/src read no clock* — 11 rather than 10, so `bench.rs` was in scope and passed. `rg -n 'assert!\(.*<|Instant|elapsed|std::time|sleep|timeout|Duration' crates/happenstance-testkit/src/bench.rs` returns exactly **one** hit, `bench.rs:240`, and it is inside a doc comment saying that the elapsed time of a pass is the caller's to produce — the lint's scanner strips comments and blanks string contents (`xtask/src/lints.rs:116-199`), so no code line spells any of them. There is no threshold, no watchdog and no clock at any budget. The shipped emitters assert well-formedness only (`BenchmarkRecord::report`, bench.rs:432-440). Non-occlusion: `BenchmarkRecord::summary` (bench.rs:403-421) joins every pass with `; ` into one line and `report` prints exactly one `println!`; `a_record_summarises_itself_in_one_line` (tests/memory_benchmarks.rs:338) and `bench::tests::a_summary_is_one_line` (bench.rs:930) both assert the line contains no `\n`. `cargo xtask affected --base main` green (227 doctests + every affected package's tests).
  mount_point: "crates/happenstance-testkit/src/bench.rs, executed via crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "xtask/src/lints.rs:223-281 — `cargo run -p xtask -- lint-clock` (REQUIRED gate step, xtask/src/main.rs:361); plus `cargo xtask ci --fast`"

- id: AC-008
  criterion: |-
    **A reader meets the boundary before they meet the macro.** GIVEN an adapter author or evaluator arriving at docs.rs or the crate README, WHEN they read the benchmark family's entry, THEN the module docs and the crate-doc paragraph state what a benchmark is *not* (CF-34) and how an emitter is supplied (CF-23), the README's family table carries a benchmark row saying it is not the bar, `CHANGELOG.md` has an `[Unreleased] / Added` entry naming the feature and the boundary, every public item carries docs and every fallible public fn an `# Errors` section — and the crate-doc mention of `bench` is **not** an intra-doc link, so `cargo doc` still succeeds under default features and on `wasm32`, which is the D13 failure this workspace has already paid for once (`crates/happenstance-testkit/src/lib.rs:112-131`).
  satisfied: true
  evidence: |-
    Module docs: `crates/happenstance-testkit/src/bench.rs:1-82` open with *a benchmark is not a conformance rule* (CF-34) and *the testkit never reads a clock, the caller's emitter does* (CF-23 + CF-33) before a single item is named. Crate docs: `crates/happenstance-testkit/src/lib.rs:146-159`, spelling `bench` in **backticks and not as an intra-doc link**, with :161-171 extended to say why all three of `model`, `concurrency` and `bench` are plain text — `bench` is behind a feature *and* a target gate, so a link would break both doc configurations (the D13 failure). README: `crates/happenstance-testkit/README.md:136-176`, a four-row family table whose benchmark row reads *Checks: **nothing** — it measures / Can fail a merge: **no***. `CHANGELOG.md:25-58` — an `## [Unreleased]` / `### Added` entry naming the feature, the CF-34 boundary and the caller-owned clock. Every public item carries docs under `missing_docs = "warn"` + `-D warnings`; no public fn in the family returns `Result`, so no `# Errors` section is owed, and the two that can panic carry `# Panics` (`BenchmarkParams::new` bench.rs:126-136, `BenchmarkRecord::report` bench.rs:426-430). Both doc builds green: `RUSTDOCFLAGS='-D warnings' cargo doc -p happenstance-testkit --no-deps` (default features) and `RUSTDOCFLAGS='-D warnings' cargo doc -p happenstance-testkit --all-features --no-deps --document-private-items`.
  mount_point: "crates/happenstance-testkit/src/lib.rs (crate docs + module declaration), crates/happenstance-testkit/README.md, CHANGELOG.md"
  verifying_test: "xtask/src/main.rs:290-301 `documentation` step (`cargo doc --locked --workspace --all-features --no-deps --document-private-items`, RUSTDOCFLAGS=-D warnings) plus `cargo doc -p happenstance-testkit --no-deps` with default features"
```
