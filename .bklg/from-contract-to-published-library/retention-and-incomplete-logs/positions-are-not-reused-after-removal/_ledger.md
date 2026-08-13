---
item: HS-S0116
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — ES-38's owed rule, and the store that renumbers on compaction

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes specific to this story, so a row is not flipped on the wrong evidence:

- **The mount is one line.** Every row's `mount_point` resolves to
  `crates/happenstance-testkit/src/registry.rs:94` — the rule's name inside `for_each_event_store_rule!`,
  in the *Sequence positions* group at `:139-141`. A rule body in `suite.rs` that is absent from that macro
  is invisible to every emitter, and evidence citing only `suite.rs` is citing the wrong half.
- **`crates/happenstance-testkit/tests/completeness_instrument.rs` is HS-S0114's file and does not exist
  until that story lands.** AC-004's evidence must come from that target. If it is missing when this story
  starts, that is EC-009 — halt and report a dependency failure; a bespoke stand-in store is never evidence.
- **A skip is evidence for AC-005 and for nothing else.** `MemoryFixture` reporting
  `RuleOutcome::Skipped` with a stated reason satisfies AC-005; it does **not** satisfy AC-003 or AC-004,
  whose evidence must come from a run in which the rule actually executed its assertions.

```yaml
- id: AC-001
  criterion: |-
    The pruning adapter author is told which rule they broke, by name. GIVEN an adapter author whose store compacts — a device saving space, a regulated purge — runs `happenstance_testkit::event_store_conformance!` against their own fixture, WHEN that store re-derives positions from the events it kept, THEN the run fails a test named `positions_are_not_reused_after_removal` rather than reporting "conformance failed", because the rule body exists in `crates/happenstance-testkit/src/suite.rs` and its name appears **exactly once** inside `for_each_event_store_rule!` (`crates/happenstance-testkit/src/registry.rs:94`), in the *Sequence positions* group beside `positions_are_unique` and `positions_are_strictly_monotonic` (`:139-141`), so all four emitters and `for_each_mutant!` carry it without a second registration anywhere.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — the rule name inside `for_each_event_store_rule!`, Sequence positions group (:139-141); rule body at crates/happenstance-testkit/src/suite.rs"
  verifying_test: "no_orphan_rules (crates/happenstance-testkit/src/registry.rs:412); cargo test -p happenstance-testkit --test mutation_coverage — CompactingRestoreStore's failure reported under this rule's module name; the rule's module listed in memory_conformance, local_conformance, memory_conformance_blocking, memory_conformance_wasm, fixture_instruments and completeness_instrument"

- id: AC-002
  criterion: |-
    The seam arrives without breaking one existing `Fixture` impl. GIVEN an adapter author who already implements `Fixture` in their own crate against `happenstance-testkit` `0.2.x`, WHEN they take the version carrying this rule, THEN their impl compiles **untouched** and their store is simply reported as declining removal — because the seam is a **defaulted** `Capability` associated const (default `Capability::declined(<reason>)`) plus a **defaulted** method taking a **set** of positions (`&[SequencePosition]`), in the exact shape `MID_BATCH_FAULT` set (`crates/happenstance-testkit/src/contract.rs:207-211`, `:297-307`); no required item is added, `SECOND_HANDLE` (`:161`) and `REOPEN` (`:173`) are unchanged, and no floor-shaped `remove_below` seam exists to pre-decide ES-39 inside the testkit.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs — the defaulted Capability const and defaulted method on `Fixture`, beside MID_BATCH_FAULT (:200-211) and arm_mid_batch_fault (:290-307); consumed at the rule mount crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "cargo test -p happenstance-testkit --all-targets green with MemoryFixture, DurableFixture, DecliningFixture and every MutantFixture<D> unedited for the const; cargo-semver-checks at the PR grain (CONTRIBUTING.md:291-296) reporting both items as additive with defaults"

- id: AC-003
  criterion: |-
    The rule catches renumbering using only values the store itself said. GIVEN the rule is run against a store whose positions are neither dense nor starting at 1, WHEN it appends across more than one batch, removes a **scattered** subset that excludes the highest assigned position, and appends once more, THEN it passes or fails purely on captured values — (1) the removed positions are no longer readable, (2) the newly assigned position equals **no** position captured earlier including the removed ones, (3) it is strictly greater than the maximum of every position ever assigned, and (4) the positions `Query::all()` reports are unique, strictly increasing and a subset of what the store assigned plus the new one — and **no integer list literal and no `SequencePosition` built from a literal appears anywhere in the rule body**, in particular not the instrument's arbitrary first retained position.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — the registered rule, emitted against GappedPositionFixture through for_each_mutant! (crates/happenstance-testkit/tests/mutation_coverage.rs:2056); rule body in crates/happenstance-testkit/src/suite.rs"
  verifying_test: "cargo test -p happenstance-testkit --test mutation_coverage — the rule green against GappedPositionFixture (crates/happenstance-testkit/tests/mutation_coverage/variants.rs:229-260); cargo xtask lints — no_position_literals (xtask/src/lints.rs:628)"

- id: AC-004
  criterion: |-
    The honest forgetting store passes, in both retained configurations. GIVEN the repository owner runs the suite against HS-S0114's instrument in the **suffix** and the **scattered** configurations (DA-1), WHEN this rule runs — the instrument's `ForgettingFixture` declaring the capability and removing by **narrowing its retained set**, inner `MemoryEventStore` untouched and `restore` called nowhere — THEN it is green in both, so any later red under this rule name is attributable to a store that *renumbers*, never to a store that merely *forgets*.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/completeness_instrument.rs — the `event_store_conformance!` invocations on ForgettingFixture in the suffix and scattered configurations, carrying the rule via crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "cargo test -p happenstance-testkit --test completeness_instrument — this rule's generated module present and green in both configurations; grep confirming MemoryEventStore::restore (crates/happenstance-core/src/memory.rs:163) is absent from that target"

- id: AC-005
  criterion: |-
    The reference fixture says *why* it cannot help, and the silence is accounted for. GIVEN a reader of the conformance output for `MemoryFixture` (and for `DurableFixture`, `DecliningFixture` and every `MutantFixture<D>`), WHEN this rule reaches them, THEN a test carrying the rule's name **still exists in the binary** and reports `RuleOutcome::Skipped { capability, reason }` with the fixture's own stated reason — never `#[cfg]`-ed out, the arrangement `crates/happenstance-testkit/src/contract.rs:31-42` rejects by name — and the skip is *accountable*: `harness.rs`'s hand-written `declines()` gains its line and `MUST_SKIP` gains this rule's name, so no other mutant's outcome becomes unexplainable and a future deletion of the `require!` gate cannot pass in silence.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs:94 — the registered rule, gated by require! (crates/happenstance-testkit/src/suite.rs:37-46); accounted for at crates/happenstance-testkit/tests/mutation_coverage/harness.rs:523-570 (declines()) and crates/happenstance-testkit/tests/mutation_coverage.rs:3157 (MUST_SKIP)"
  verifying_test: "cargo test -p happenstance-testkit --test mutation_coverage — capability_skips_are_reported (:3184, including :3288-3296 and :3306-3314) and mutants_fail_exactly_their_declared_rules (:2889, whose :2907-2925 unaccounted-skip check fails without the declines() line); cargo test -p happenstance-testkit --test memory_conformance showing the rule's module present and reporting a skip with a stated reason"

- id: AC-006
  criterion: |-
    The wrong implementation the specification names is in the tree, and it fails exactly this rule. GIVEN ES-38's `Rejects:` names *"an adapter that renumbers on compaction"* (`spec/SPECIFICATION.md:4321-4323`), WHEN `cargo test -p happenstance-testkit --test mutation_coverage` runs, THEN `CompactingRestoreStore` exists as a `Defect` whose **single** overridden step re-derives the survivors' positions from the survivors, is listed in `for_each_mutant!` and carries a `REGISTRY` row with a non-empty `fails: &["positions_are_not_reused_after_removal"]`, a real `provenance` citing `crates/happenstance-core/src/memory.rs:277-282`, `:386-389`, `FailureMode::Assertion`, and a per-rule `expect` substring pinning **which** of the rule's four assertions did the rejecting — and it fails this rule and **no other**.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs:324 (REGISTRY row) and :2056 (for_each_mutant!), with the Defect impl in crates/happenstance-testkit/tests/mutation_coverage/mutants.rs; run against the rule registered at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "cargo test -p happenstance-testkit --test mutation_coverage — every_rule_has_a_mutant (:2734), mutant_registry_is_exhaustive (:2754) and mutants_fail_exactly_their_declared_rules (:2889) green in both directions; the FailureMode::Assertion origin check (:2965-2975)"

- id: AC-007
  criterion: |-
    The release tells an adapter author what just turned their CI red, in a sentence. GIVEN an adapter author who took a minor bump of `happenstance-testkit` and found a new red test, WHEN they open `CHANGELOG.md` at `## [Unreleased] / ### Added`, THEN they find an entry naming this rule **and the defect it detects** in prose — a store that renumbers when it compacts, and what that costs a reader holding a checkpoint or a replicated `after` — rather than the rule's name listed beside others in a bullet.
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md:25-27 — the `## [Unreleased] / ### Added` section; the rule it names is the one mounted at crates/happenstance-testkit/src/registry.rs:94"
  verifying_test: "cargo xtask lints — the CF-29 changelog check (xtask/src/lints.rs:525-580) with MIN_CHARS_PER_RULE (:446); also run inside cargo xtask affected --base main (.redkiln/config.yaml:40)"

- id: AC-008
  criterion: |-
    Nothing published breaks, and the story that proves it inherits an accurate list. GIVEN `surface-diff-and-the-ac-012-escalation` must report this initiative's surface delta against the `0.2.0` baseline, WHEN it diffs this PR, THEN it finds exactly **three** additive public items — one `pub async fn` in `suite::rules` (reachable via `crates/happenstance-testkit/src/lib.rs:189`), one defaulted `Capability` const and one defaulted method on `Fixture` — with `happenstance-core` untouched, `spec/SPECIFICATION.md` **byte-identical**, no `Cargo.toml` edit and no version bump; and this story's implementation report hands over that enumeration **together with** the second sentence AC-011 is owed: additive under semver *and* capable of turning a passing adapter's CI red (`crates/happenstance-testkit/Cargo.toml:4-14`).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/lib.rs:189 (`pub use suite::rules`, the re-export that makes the rule public) plus the two defaulted items in crates/happenstance-testkit/src/contract.rs; the diff boundary is the PR-boundary glob list in spec.md"
  verifying_test: "cargo-semver-checks at the PR grain (CONTRIBUTING.md:291-296) reporting no breaking change; git diff --stat confined to the PR-boundary globs; git diff -- spec/SPECIFICATION.md crates/happenstance-core crates/happenstance-testkit/Cargo.toml empty; cargo xtask spec-trace green; content review of implementation-report.md for the three-item enumeration and both AC-011 sentences"
```
