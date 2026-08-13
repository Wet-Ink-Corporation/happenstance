---
item: "HS-S0043"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The reopen rule gets a negative control, and the durability clauses get verdicts

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two obligations this story's testing brief adds on top of the schema, both discharged in the
`evidence` fields rather than in extra rows:

- **AC-T05** — every row's evidence is a real `file:line` and/or test id, never "green gate".
  A green gate is a precondition for reading these criteria, never a substitute
  (`RUNBOOK.md:38-42`).
- **AC-T06** — the evidence for **AC-001** must state *which* of the two negative-control options
  was used. This story chose the permanent registry row over a reverted local mutation, per testing
  brief §5's stated preference; the evidence cites the `Declared` row's `file:line`.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author reading `recorded_time_survives_a_reopen` to learn whether their own store's reopen is good enough, WHEN they run the mutant harness, THEN a registered subject fails that rule at its headline assertion — the `recorded_at` comparison, not the survival anchor — and its `Declared` row's `expect` entry pins the headline message, so a failure at an earlier anchor is reported as the wrong failure rather than counted as a pass."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules"

- id: AC-002
  criterion: "GIVEN the same adapter author re-running the harness on a fast machine and on a slow one, WHEN the control reopens, THEN the stamp it recomputes is a deterministic function of a per-fixture reopen generation — never re-spent from `correct::stamp`'s constant, never read from a clock — so the failure is identical run to run and can never be silently invisible or intermittently absent."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules"

- id: AC-003
  criterion: "GIVEN an evaluator judging in one sitting whether the three reopen rules are independently falsifiable, WHEN they read the registry, THEN the new row declares exactly one rule — `recorded_time_survives_a_reopen` — and the subject passes `acknowledged_writes_survive_a_reopen` and `reopened_store_does_not_reissue_an_event_id`, so what bites is the stamp sentence alone and not the survival sentences `LosingFixture` already covers."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutants_fail_exactly_their_declared_rules, ::every_rule_has_a_mutant"

- id: AC-004
  criterion: "GIVEN a maintainer who adds a fixture and forgets one of the lists it must appear in, WHEN the gate runs, THEN the control is present in all four places — the type in `mutation_coverage/mutants.rs`, the type in `for_each_mutant!`, the `Declared` row in `REGISTRY` with a real provenance sentence, and the `MODEL_COVERAGE` row — and the model doc comment's miss count and its \"defects that are only visible across a reopen\" bullet both name the second subject, so the file's own answer to what is this test blind to stays true rather than going stale in prose."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/mutation_coverage.rs::mutant_registry_is_exhaustive, ::every_mutant_states_its_provenance, ::the_model_rule_rejects_exactly_what_it_claims (run with --all-features)"

- id: AC-005
  criterion: "GIVEN an evaluator who reads a [PROVISIONAL] marker to decide adopt-or-decline, WHEN they read ES-35, CF-17 and CF-14 after this PR, THEN each carries a written verdict against this adapter's evidence — ES-35's remaining falsifier restated as a store that loses a write to a fault rather than to an instruction, CF-17's rule shape confirmed against a real file-backed adapter, CF-14's deferral confirmed and narrowed to the two implementations that have not answered — every sentence that has become factually false is corrected, and no marker changes level; any promotion is recorded as a named ADR-queue item in this story's folder with its evidence attached, never made here."
  satisfied: false
  evidence: ""
  mount_point: "spec/SPECIFICATION.md"
  verifying_test: "cargo xtask spec-trace (xtask/src/spec_trace.rs) plus review of spec/SPECIFICATION.md:4142-4180, :7471-7500, :7570-7596 and the clause-status rows at :8617, :8725, :8728"

- id: AC-006
  criterion: "GIVEN the application author who was told an acknowledged write survives a reopen, WHEN the SQLite conformance target is run after the control lands, THEN `recorded_time_survives_a_reopen` and `acknowledged_writes_survive_a_reopen` are green against `SqliteFixture` and the `MID_BATCH_FAULT`-gated rule is reported as Skipped carrying the fixture's stated reason — so the pass is a pass of a rule now demonstrably failable, and the fault far end stays visibly open rather than looking covered."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/conformance.rs"
  verifying_test: "crates/happenstance-sqlite/tests/conformance.rs — event_store_conformance!(SqliteFixture::new())'s recorded_time_survives_a_reopen and acknowledged_writes_survive_a_reopen, plus the MID_BATCH_FAULT-gated rule's reported skip"

- id: AC-007
  criterion: "GIVEN a reader who never opens a test file, WHEN this change merges, THEN `CHANGELOG.md` carries an entry naming the defect the new control encodes in CF-29's stated shape, and the story-grain gate is green with `spec-trace`'s citation count not fallen — so the change to what the suite can prove is discoverable from the record alone."
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md"
  verifying_test: "cargo xtask affected --base {{base}} (.redkiln/config.yaml:40) and cargo xtask lints && cargo xtask spec-trace (:48); the CHANGELOG.md lint of CF-29 in xtask/src/lints.rs"
```
