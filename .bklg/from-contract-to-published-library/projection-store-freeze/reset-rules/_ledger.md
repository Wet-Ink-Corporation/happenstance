---
item: HS-S0011
stage: implement
created: 2026-08-12T13:46:05.705Z
updated: 2026-08-12T13:46:05.705Z
---

# Acceptance ledger — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

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
  criterion: "GIVEN an adapter author whose reset is the runbook procedure — two statements on two connections, the shape Norvant ran at 02:46:31 before the pod died at 02:46:33, leaving sixty-one events applied into an empty table and the runner reporting healthy (spec/E2E-CASES.md:458-481) — WHEN they run projection_store_conformance! against their own fixture, THEN reset_clears_rows_and_checkpoint_together fails by name, and a store that applies the caller's deletes and returns the checkpoint to NeverRun as one unit passes it: observed through a fresh handle, every key the rule wrote reads None and checkpoint(id) is Checkpoint::NeverRun — both, or neither, because the pairing is the claim and not either half. The interrupted half is asserted where the fixture can express a failing reset and is a reported skip where it cannot; it is never a silently absent assertion."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration"
  verifying_test: "reset_clears_rows_and_checkpoint_together, emitted by the projection harnesses (crates/happenstance-testkit/tests/projection_conformance*.rs, wasm sibling projection_conformance_wasm.rs); red against its two-statement mutant in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-002
  criterion: "GIVEN Kestrel Cold Chain, whose one projection store holds van_stock (rebuilt several times a day across 138 devices) and fgas_ledger (a hash chain a regulator already holds), WHEN the author's reset is a SqliteProjectionStore::reset() that truncates the checkpoint table, THEN reset_is_scoped_to_one_projection fails by name — because the rule commits probe rows and a checkpoint under two ProjectionIds in one store, resets one, and asserts the sibling's rows and checkpoint are untouched as well as the target's being gone. The sibling assertion is the rule; without it the same code passes and the regulatory ledger is destroyed in the field."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration"
  verifying_test: "reset_is_scoped_to_one_projection, emitted by the projection harnesses; red against TruncatingResetStore in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-003
  criterion: "GIVEN an adapter author whose store must protect one projection from reset (the fgas_ledger shape), WHEN they declare that capability and run the suite, THEN refused_reset_changes_nothing proves the refusal is real on both halves: the call returns Err(ResetError::Refused) and is never reported as success, and through a fresh handle the rows and the checkpoint are exactly as they were before the attempt. A store that reports the refusal correctly after deleting the rows fails, and so does one that reports success."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration"
  verifying_test: "refused_reset_changes_nothing, emitted by the projection harnesses; red against both the refusal-as-success and refusal-after-the-fact mutants in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-004
  criterion: "GIVEN an adapter author whose store cannot refuse a reset at all, WHEN they run the suite, THEN refused_reset_changes_nothing still appears in their output as a test and reports RuleOutcome::Skipped carrying their fixture's own stated reason — never #[cfg]-ed out, never a pass, never a second skip vocabulary invented for the projection family. This is initiative AC-05 in full for this story: a guarantee that does not apply says so, with a reason, in the same line shape every other declined capability uses."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/contract.rs:473,500-507 — RuleOutcome and skip_line, reused unchanged"
  verifying_test: "the projection sibling of capability_skips_are_reported in crates/happenstance-testkit/tests/projection_mutation_coverage.rs, asserting on RuleOutcome values rather than stdout"
- id: AC-005
  criterion: "GIVEN an operator asking the store about a projection it has never seen, WHEN the store answers, THEN fresh_projection_has_no_checkpoint requires the Checkpoint::NeverRun variant — and a store answering Live { through: FIRST } for that id fails by name. The assertion is a variant match; comparing against any SequencePosition is simultaneously a CF-6 violation and the exact value the defective store writes, so a rule written that way cannot tell the two mutants apart and both walk free."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration"
  verifying_test: "fresh_projection_has_no_checkpoint, emitted by the projection harnesses; red against the Live-for-an-unseen-id mutant. Precondition recorded with its sha: redkiln validate --kb green with the PS-19 repair atom accepted under .kb/decisions/"
- id: AC-006
  criterion: "GIVEN the operator at 03:18 who \"resets properly\" by writing commit(empty_batch, id, SequencePosition::FIRST) — the substitute all six deployment scenarios reached for and all six got wrong (RUNBOOK.md:3904-3907) — WHEN the runner resumes, THEN reset_is_not_commit_at_first rejects it: the rule resets one id and commits-at-FIRST on a sibling id in the same store, asserts the two checkpoints differ by variant (NeverRun vs Live { .. }), and then derives a resume point from each under the port's own rule — strictly after a Live position, inclusive from the store's first position when NeverRun — asserting the event at the first position is included in the reset case and excluded in the commit-at-FIRST case. Event 1 skipped permanently and silently is the defect; the variant check alone would not see the resume consequence, and the resume check alone would not see the state."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration"
  verifying_test: "reset_is_not_commit_at_first, emitted by the projection harnesses; red against the commit-at-first substitute store registered in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-007
  criterion: "GIVEN an adapter author who wants to know whether reset_is_scoped_to_one_projection can actually fail, WHEN they open the projection mutant binary, THEN TruncatingResetStore is registered there with a fails list naming exactly that rule and a non-empty provenance describing the real mistake it models — a SqliteProjectionStore::reset() truncating the checkpoint table, cheap and obvious, which destroys the append-only regulatory ledger sharing the file. It is correct in every other step, so the rule that goes red goes red for the defect and not for the store."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs — the projection REGISTRY: &[Declared]"
  verifying_test: "the three projection exactness meta-tests (CF-1/CF-2/CF-3 siblings) plus the CF-4 provenance meta-test in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-008
  criterion: "GIVEN that a rule nobody has checked is a rule that proves nothing, WHEN this story merges, THEN every one of its five rules has at least one registered wrong store that fails it — the commit-at-first substitute (declaring reset_is_not_commit_at_first, provenance: six scenarios out of six), TruncatingResetStore, and one each for the remaining three in the shapes §4.11 pre-specifies (the two-statement runbook procedure; a refusal reported as success; a store answering Live { through: FIRST } for an id it has never seen) — and each wrong store is a single overridden step over the shared correct-steps module, never a hand-written store. A store that differs in two ways makes a red rule red for the wrong reason, and the exactness meta-test would then be catching a defect in the instrument."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs — the projection REGISTRY: &[Declared] and its shared correct-steps module"
  verifying_test: "every_projection_rule_has_a_mutant, projection_mutant_registry_is_exhaustive and projection_mutants_fail_exactly_their_declared_rules in crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
- id: AC-009
  criterion: "GIVEN initiative AC-04 — the adapter author is told when they are finished, \"with no rule silently absent from the run\" — WHEN they invoke the one entry point, THEN all five rule names are in the single for_each_projection_store_rule! enumeration, so the tokio, blocking and wasm32 harnesses each emit five more tests with no per-harness list to maintain, and a rule present in the module but absent from the enumeration is a build failure rather than an omission. The edge developer's runtime is not a separately maintained subset: projection_conformance_wasm.rs type-checks the same five in the same cargo xtask ci run."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the for_each_projection_store_rule! enumeration; crates/happenstance-testkit/tests/projection_conformance_wasm.rs"
  verifying_test: "the projection sibling of no_orphan_rules (crates/happenstance-testkit/src/registry.rs:412-424), plus cargo xtask ci's mandatory wasm32 step: cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown"
- id: AC-010
  criterion: "GIVEN an evaluator (initiative AC-08) reading the release notes to decide whether the suite is worth trusting, WHEN they look up any of these five rules, THEN CHANGELOG.md names the defect it detects rather than listing the rule, the gate's own rule lints see all five (they read RULE_FILES, so a projection rules module absent from that list makes them pass vacuously — reported, never patched here), and git diff over spec/SPECIFICATION.md is empty: no [FROZEN] clause text changed and no maturity marker moved to make anything pass."
  satisfied: false
  evidence: ""
  mount_point: "CHANGELOG.md; xtask/src/spec_trace.rs:85-89 (RULE_FILES) — the file list the gate's rule checks read"
  verifying_test: "cargo xtask spec-trace (check 6), the CF-29 changelog lint at xtask/src/lints.rs:511-531, the CF-6 position lint over the same file set, and git diff --stat main...HEAD -- spec/ crates/happenstance-core/ (empty)"
```
