---
item: "HS-S0012"
stage: implement
created: "2026-08-12T13:46:06.709Z"
updated: "2026-08-12T13:46:06.709Z"
---

# Acceptance ledger — Read-your-writes, chunk-size invariance and rebuild authority

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
  criterion: "GIVEN an adapter author whose storage lets an open batch see its own pending writes, and whose `ProjectionProbe` therefore declares `READS_THROUGH_BATCH = true`, WHEN they invoke `projection_store_conformance!` with their fixture, THEN a test named `batch_reads_reflect_pending_writes` runs, writes a value through the open batch and asserts `probe_read_through(&batch, k) == Some(v)` for that value before commit — so a store whose batch `get` answers from committed state fails it by name rather than passing on a `None`-vs-`Some` technicality."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single enumeration `for_each_projection_store_rule!` (plus the projection rules module, the tokio/blocking/wasm32 harnesses, and mutation_coverage.rs's projection REGISTRY + for_each_mutant!)"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules (CommittedReadBatchStore row) + the tokio/blocking projection harnesses running `batch_reads_reflect_pending_writes` against MemoryProjectionStore"
- id: AC-002
  criterion: "GIVEN an adapter author who will rebuild a projection in production at whatever chunk size fits their memory budget, and who must not have to discover that the chunk size changed the answer, WHEN the suite runs against their store, THEN `rebuild_is_chunk_size_invariant` replays one fixed sequence (`a, a, b, a, b, a`) at chunk sizes 1, 3 and whole-log against three isolated stores (`open()` per run), each step a read-modify-write through the batch (`probe_read_through(…).unwrap_or(0) + 1`), and asserts the three runs' read models are identical per key — so a store that cannot read through its own batch diverges (`a = 2` at size 3, `a = 1` at whole-log) instead of quietly agreeing."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — `for_each_projection_store_rule!` (plus the rules module, the three harnesses, and the mutant registry)"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules (CommittedReadBatchStore and FirstWriteWinsBatchStore rows) + `rebuild_is_chunk_size_invariant` against MemoryProjectionStore in the tokio/blocking harnesses"
- id: AC-003
  criterion: "GIVEN a reader who must decide whether the rows in front of them are authoritative, WHEN a store commits two chunks under `Authority::Rebuilding` and then one under `Authority::Live`, THEN `rebuilding_is_distinguishable_from_live` asserts `checkpoint(id)` reads the `Checkpoint::Rebuilding` variant after each of the first two and `Checkpoint::Live` after the third — comparing variants only, never the `through` value — and a store that rebuilds in place behind a single position field fails it. The rule asserts nothing immediately after `reset`, because a rebuild that has committed nothing correctly reads `NeverRun`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — `for_each_projection_store_rule!` (plus the rules module, the three harnesses, and the mutant registry)"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules (LiveOnlyCheckpointStore row, failing exactly this rule) + `rebuilding_is_distinguishable_from_live` against MemoryProjectionStore"
- id: AC-004
  criterion: "GIVEN an adapter author deciding whether to trust this suite at all, and knowing that a rule no implementation can fail is decorative (`CLAUDE.md`, The rule that matters), WHEN they read the mutant registry, THEN each of the three new rules has at least one registered wrong implementation with an exact `fails` set and non-empty `provenance` naming the real adapter mistake it models — the round-trip batch `get`, the `entry().or_insert(…)` dedup buffer, the single-position-field rebuild — and no pass rate is quoted anywhere over the mutant set."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs — the projection `REGISTRY: &[Declared]` and `for_each_mutant!`"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::every_rule_has_a_mutant, ::mutant_registry_is_exhaustive and ::mutants_fail_exactly_their_declared_rules over the three new rules and four new stores"
- id: AC-005
  criterion: "GIVEN an adapter author whose storage genuinely cannot answer a read from an uncommitted batch — the write-behind or buffering shape PS-12 explicitly permits — WHEN they declare `READS_THROUGH_BATCH = false` and run the suite, THEN the whole projection suite is green, not red: `NoBatchReadStore` is registered as a `Kind::ConformantVariant` with `fails: &[]`, exposes no read path on the open batch (`probe_read_through` is `unimplemented!()`), and passes every projection rule — so declining a capability honestly is a supported outcome rather than a failure, and the skip arm of both gated rules has a fixture behind it in the same run that `MemoryProjectionStore` holds the `true` arm."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage/variants.rs + the projection `REGISTRY`/`for_each_mutant!` in crates/happenstance-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutant_registry_is_exhaustive (NoBatchReadStore row, `fails: &[]`) + `projection_store_conformance!` driven against NoBatchReadFixture with zero failures"
- id: AC-006
  criterion: "GIVEN that same author reading a CI log, WHEN the two gated rules run against their declining fixture, THEN each returns `RuleOutcome::Skipped` whose `capability` field is the string `ProjectionProbe::READS_THROUGH_BATCH` — the constant they can actually go and change, not a fixture const that does not exist — and whose `reason` is the fixture's own non-empty stated reason, and the run reports two skips, not one and not silence. The assertion is on the `RuleOutcome` value, not on stdout."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the gated rules in `for_each_projection_store_rule!`; the gate branch lives in the rule body, never in an emitter"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs — the new value assertion beside ::capability_skips_are_reported, asserting both fields of `RuleOutcome::Skipped` for both gated rules against NoBatchReadFixture"
- id: AC-007
  criterion: "GIVEN an adapter author who invokes exactly one macro and expects the whole bar, and GIVEN P3, who runs that same bar on `wasm32-unknown-unknown`, WHEN the gate runs, THEN all three new rules are mounted at all four composition points — the projection rules module, the single enumeration `for_each_projection_store_rule!`, the three harnesses (tokio / blocking / `#![cfg(target_arch = \"wasm32\")]`), and the mutant registry's `for_each_mutant!` + `REGISTRY` — so each rule is emitted as a named test on every target in one run, with no wasm-specific subset and no new gate step. A rule present in the module but absent from the enumeration is caught by the orphan meta-test."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — `for_each_projection_store_rule!`, driving all three harnesses through the existing emitters"
  verifying_test: "crates/happenstance-testkit/src/registry.rs::no_orphan_rules (projection sibling) + `cargo xtask ci`'s mandatory wasm32 conformance-harness step (xtask/src/main.rs:231-240)"
- id: AC-008
  criterion: "GIVEN the repository owner reviewing this PR against a project whose whole point is that the port is not yet frozen, WHEN they read the diff, THEN nothing outside this story's boundary moved: no `spec/SPECIFICATION.md` clause text, maturity marker or rule citation changed (PS-13 and PS-14 are [FROZEN] and are implemented, not amended); no `ProjectionStore`, `ProjectionProbe`, `Checkpoint`, `Authority` or `MemoryProjectionStore` definition changed; `Capability`, `RuleOutcome`, the three emitters and the `Declared` shape are byte-identical; no assertion anywhere compares a position or a checkpoint's `through` to a literal; and `cargo xtask spec-trace` is green and unaffected."
  satisfied: false
  evidence: ""
  mount_point: "the PR-boundary block — crates/happenstance-testkit/src/**, crates/happenstance-testkit/tests/**, and this story's own .bklg folder"
  verifying_test: "cargo xtask spec-trace (mandatory gate step, xtask/src/main.rs) + the pre-existing event-store suite and GappedPositionFixture rules still green in `cargo test --workspace --all-features`"
```
