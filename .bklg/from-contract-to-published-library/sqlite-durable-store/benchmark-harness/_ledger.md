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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — `cargo test -p happenstance-testkit --features bench -- --show-output`"

- id: AC-002
  criterion: |-
    **The evaluator can check "benchmarks are not the bar" instead of trusting it.** GIVEN an evaluator reading this diff (initiative *Decide in one sitting*), WHEN they compare the conformance rule set either side of it, THEN the rule-name list produced by `for_each_event_store_rule!(__emit_rule_names)` is **identical**, `suite.rs` has gained no `pub async fn`, `xtask/src/spec_trace.rs`'s `RULE_FILES` is still a three-element array, and no clause's `Rule:` line has changed — so CF-34's claim is a property of the tree rather than a sentence in a report.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/src/registry.rs:412 `no_orphan_rules` — `cargo test -p happenstance-testkit no_orphan_rules`; plus `cargo xtask spec-trace` and `cargo xtask lints`"

- id: AC-003
  criterion: |-
    **The edge developer keeps their runtime, including under `--all-features`.** GIVEN the constrained-runtime developer whose target is `wasm32-unknown-unknown`, WHEN the testkit is checked on that target across the *whole* feature powerset — the combination `--all-features` actually produces, because a Cargo feature is not target-scoped — THEN `bench` compiles away entirely, the four mandatory wasm32 steps are unchanged, and nothing in the `!Send` flavour has gained a bound, a dependency or an item (DR-08, architecture brief AC-A06).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs — the gated `pub mod bench;` declaration"
  verifying_test: "xtask/src/main.rs:564-591 `wasm32 feature powerset` — `cargo hack check -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps`; plus `cargo xtask wasm`"

- id: AC-004
  criterion: |-
    **No measurement dependency is imposed on anybody, and the timer is the caller's.** GIVEN an adapter author who wants `criterion`, `divan` or a CSV row, WHEN they invoke the third arm with `emit = <their emitter>`, THEN their own wrapper surrounds each scenario and their dependency stays in *their* `dev-dependencies` — while `happenstance-testkit`'s `[dependencies]` remain exactly `happenstance-core` + `futures-core` and `Cargo.lock` is unchanged. Two distinct emitters are demonstrated in the mount, not described (CF-23, `[FROZEN]`).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs — the second invocation, `emit = …`"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs (both invocations) plus `git diff -- crates/happenstance-testkit/Cargo.toml Cargo.lock`"

- id: AC-005
  criterion: |-
    **The decision-maker measures the three things the decision needs, at their own numbers.** GIVEN ADR-0022's author (the slice-mate, HS-S0035) about to choose an append-condition strategy, WHEN they run the harness against real SQLite, THEN they get (a) append throughput over a batch of *n*, (b) conditional append under *k* contenders, and (c) replay of *N* events run **both** unfiltered and behind a tag filter — with *n*, *k* and *N* supplied by them at the call site, because no constant in the testkit can be right for both a `Vec` behind an `RwLock` and a file under `BEGIN IMMEDIATE`.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — parameterised invocations at two different parameter sets; the replay scenario reports a filtered and an unfiltered record"

- id: AC-006
  criterion: |-
    **A contended run in which nobody collides is visible as such, not averaged away.** GIVEN the same author, whose evidence obligation is a *"realistic batch and rejection mix"* (`.kb/decisions/0012-append-shape-and-preconditions.md:52-60`), WHEN the contended scenario runs with *k* contenders over one append condition, THEN the record reports the committed and the rejected counts **separately**, distinguishes a `ConditionViolated` rejection from a transport-level failure the way `concurrency::Attempt` does, and accounts for every contender (`committed + rejected + failed == k`) — so a run in which every contender wins is legible as a measurement of the wrong thing.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "crates/happenstance-testkit/tests/memory_benchmarks.rs — assertion on the contended record's committed/rejected/failed fields against `MemoryFixture`"

- id: AC-007
  criterion: |-
    **A benchmark result can never turn a merge red, and the testkit never reads a clock.** GIVEN any contributor on a loaded CI runner, WHEN the mounted target runs inside the gate's existing `cargo test --locked --workspace --all-features -- --show-output` step, THEN it passes on completion alone: there is no threshold, no `assert!(… < …)` on a duration, no watchdog and no sleep anywhere in the family, `cargo run -p xtask -- lint-clock` stays green over `crates/happenstance-testkit/src` — which is where `bench.rs` lands — and the default emitter prints at most one line per scenario so the CF-18 `SKIP <rule>: …` lines that step exists to surface are not buried under benchmark output.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/bench.rs, executed via crates/happenstance-testkit/tests/memory_benchmarks.rs"
  verifying_test: "xtask/src/lints.rs:223-281 — `cargo run -p xtask -- lint-clock` (REQUIRED gate step, xtask/src/main.rs:361); plus `cargo xtask ci --fast`"

- id: AC-008
  criterion: |-
    **A reader meets the boundary before they meet the macro.** GIVEN an adapter author or evaluator arriving at docs.rs or the crate README, WHEN they read the benchmark family's entry, THEN the module docs and the crate-doc paragraph state what a benchmark is *not* (CF-34) and how an emitter is supplied (CF-23), the README's family table carries a benchmark row saying it is not the bar, `CHANGELOG.md` has an `[Unreleased] / Added` entry naming the feature and the boundary, every public item carries docs and every fallible public fn an `# Errors` section — and the crate-doc mention of `bench` is **not** an intra-doc link, so `cargo doc` still succeeds under default features and on `wasm32`, which is the D13 failure this workspace has already paid for once (`crates/happenstance-testkit/src/lib.rs:112-131`).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs (crate docs + module declaration), crates/happenstance-testkit/README.md, CHANGELOG.md"
  verifying_test: "xtask/src/main.rs:290-301 `documentation` step (`cargo doc --locked --workspace --all-features --no-deps --document-private-items`, RUSTDOCFLAGS=-D warnings) plus `cargo doc -p happenstance-testkit --no-deps` with default features"
```
