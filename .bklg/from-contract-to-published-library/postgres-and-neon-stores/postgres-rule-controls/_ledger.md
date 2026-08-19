---
item: "HS-S0064"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The naive-arm control and the Postgres mutant column

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
  criterion: "**GIVEN** an adapter author who has just read that `PostgresEventStore` passes CF-13 and wants to know what a *failing* Postgres store would look like, **WHEN** they read the merged tree, **THEN** a naive arm of the same adapter exists — `position` allocated by a sequence, plain `INSERT … RETURNING position`, `head` as `SELECT max(position)`, no `xid8` read and no frontier predicate — reachable only through an off-by-default Cargo feature (or a `cfg` chosen in its place), **AND** no deliberately-broken Postgres `Fixture` survives the merge as a maintained instrument: the default build, `cargo test --workspace --all-features` and the live job's shipped-arm run are all unchanged by its existence."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/rule_controls.rs"
  verifying_test: "crates/happenstance-postgres/tests/rule_controls.rs::naive_arm_is_reachable_only_under_its_feature (plus cargo xtask ci's cargo hack --feature-powerset step)"

- id: AC-002
  criterion: "**GIVEN** an adapter author who trusts this workspace's dependency rule (\"no adapter may depend on another adapter\", `CLAUDE.md`) and the promise that `cargo test --workspace` needs no server, **WHEN** the mutant control is added, **THEN** it is driven from `crates/happenstance-postgres/tests/rule_controls.rs` through the public `happenstance_testkit::rules::<name>(open: impl AsyncFn() -> F) -> RuleOutcome` seam, **AND** `happenstance-testkit` gains no dependency on any adapter and `mutation_coverage.rs`'s `REGISTRY` gains no row for a live store — with the §9.1 seam choice taken deliberately, not by default."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/rule_controls.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutant_registry_is_exhaustive, green under `cargo test --workspace --all-features` with no server reachable"

- id: AC-003
  criterion: "**GIVEN** the adapter author's question \"would this rule have noticed?\", **WHEN** the naive arm is driven through `rules::nothing_below_an_observed_position_appears_later` in the live Postgres job, **THEN** the run records a failure attributed to **that rule by name** — captured by `catch_unwind` over a non-capturing probe — **AND** a decode error, a missing column, a schema fault or a connection failure is reported as itself and can never be recorded as a visibility finding."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/rule_controls.rs"
  verifying_test: "crates/happenstance-postgres/tests/rule_controls.rs::naive_arm_fails_nothing_below_an_observed_position_appears_later (live Postgres CI job)"

- id: AC-004
  criterion: "**GIVEN** an evaluator reading the Postgres job's log to decide in one sitting whether \"passes the suite\" is a claim or a coincidence, **WHEN** the job runs, **THEN** one invocation emits a per-rule outcome column for `PostgresEventStore` in which every rule the seam drives reports `pass`, `fail`, or `RuleOutcome::Skipped` carrying the fixture's own stated reason — **AND** CF-13's `fail` for the naive arm and `pass` for the shipped arm are produced by the **same call site in the same run**, so the comparison is not across two harnesses, **AND** a `Skipped` is never rendered or read as a pass."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/rule_controls.rs"
  verifying_test: "crates/happenstance-postgres/tests/rule_controls.rs::postgres_rule_outcome_column, run as `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`"

- id: AC-005
  criterion: "**GIVEN** an adapter author who needs to know whether CF-13 can fail *at all* against a store whose `append` takes several polls — connection acquire, `BEGIN`, execute, `COMMIT` — **WHEN** the mutant harness runs with no server present, **THEN** a poll-padding decorator over `PreCommitPositionStore` pads `append` to *n* `Pending` returns while leaving the defect intact (the sequence still advances outside the publishing transaction), with *n* **measured off HS-S0062's shipped `append`** and the counting method written down, and it carries a `REGISTRY` row and a `for_each_mutant!` entry stating its true verdict — **OR** `.kb/_intake/` carries the written reason it was not built, addressed to ADR-0024, naming what a real multi-poll `append` demonstrated instead."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage/mutants.rs (plus its REGISTRY row and for_each_mutant! entry in crates/happenstance-testkit/tests/mutation_coverage.rs)"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs (for_each_mutant! sweep and mutant_registry_is_exhaustive) under `cargo test -p happenstance-testkit --all-features`; on the excuse branch, the staged .kb/_intake/ file cited here"

- id: AC-006
  criterion: "**GIVEN** ES-10's own clause pre-authorising a rule change and forbidding a clause change, **WHEN** the padded store **passes** (the rule cannot detect a known defect at that poll shape), **THEN** `nothing_below_an_observed_position_appears_later`'s **schedule** changes in the same commit with its reason stated in its rustdoc and a `CHANGELOG.md` entry naming the defect it now detects, its **name** unchanged, `spec/SPECIFICATION.md` unedited and ES-10 still `[FROZEN]` and untouched; **AND** if the padded store is rejected, that observation is recorded instead and no rule changes."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/suite.rs (the rule) and CHANGELOG.md"
  verifying_test: "`cargo xtask lint-changelog` (xtask/src/lints.rs:525) and `cargo xtask spec-trace` check 6 (xtask/src/spec_trace.rs:85-89), both REQUIRED gate steps; plus an empty `git diff --stat spec/SPECIFICATION.md`"

- id: AC-007
  criterion: "**GIVEN** a contributor on a clean checkout with no Docker and no credentials (DR-9, project AC-011), **WHEN** they run `cargo xtask ci --fast`, **THEN** it exits zero: every live-requiring test this story adds is gated at **whole-invocation** grain — `#[ignore]`, a `required-features` gate, or an env read — never a `#[cfg]` hiding a conformance rule out of a macro's expansion (DR-5), **AND** the serverless half (the decorator and its registry row) runs in the ordinary `cargo test --workspace --all-features`, **AND** the live job sweeps the new target up with **no** edit to `.github/workflows/ci.yml`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-postgres/tests/rule_controls.rs (gating attribute) and .redkiln/config.yaml's verify: block"
  verifying_test: "`cargo xtask ci --fast` and `cargo xtask affected --base <base>` on a machine with no Docker and no credentials; the live job's unchanged `cargo test -p happenstance-postgres --all-features -- --ignored --show-output`"

- id: AC-008
  criterion: "**GIVEN** the author of ADR-0024 (`adr-0024-position-visibility-mechanism`, HS-S0065) and, later, the auditor of `publication-and-positioning`, both of whom must be able to cite *why* \"the adapter had to work to pass CF-13\" is true, **WHEN** they open this story's folder, **THEN** they find the §9.1 seam decision written down with both of its structural reasons, the decorator's calibration (*n*, how it was counted) and its verdict, and whatever of that is load-bearing for ADR-0024 staged in `.kb/_intake/` for the runbook's ADR pass — **AND** no ADR is authored here as a side effect."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/postgres-and-neon-stores/postgres-rule-controls/ (this ledger and the story's implementation report) and .kb/_intake/"
  verifying_test: "`redkiln validate --kb && redkiln doctor` green, with no hand-written decision atom under .kb/decisions/; the staged .kb/_intake/ file cited as evidence in this row"
```
