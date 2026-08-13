---
item: HS-S0009
stage: implement
created: 2026-08-12T13:46:03.590Z
updated: 2026-08-12T13:46:03.590Z
---

# Acceptance ledger — CheckpointOnlyStore fails the suite by name

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four notes specific to this story, so no row is flipped on the wrong evidence:

- **AC-009 is a review-time row.** "No pass rate is quoted" is a property of what the binary does
  **not** print, and there is nothing to assert against an absence except reading it (`spec.md`
  Clarifications item 3; project Testing brief, AC-003 row). Its evidence is a `file:line` into the
  new binary's module doc **plus** the reviewer's note — never a green test id alone.
- **AC-004's evidence must record a decision, not just a location.** Whether `Declared` / `Kind` /
  `FailureMode` are shared via a `#[path]` module or duplicated is deliberately left open
  (`spec.md` Clarifications item 4); the reasoning belongs in this row's evidence.
- **AC-005 may resolve as a halt.** If `for_each_projection_store_rule!` / `__emit_rule_names` is not
  in the shape this story assumes, the row is **not** flipped and is **not** worked around by
  hand-writing a second rule list (`spec.md` EC-013) — it is reported as a finding against
  `projection-suite-entry-point` and the story stops.
- **AC-011 needs the wasm step run explicitly.** `cargo xtask ci --fast` omits the mandatory wasm32
  conformance-harness check; this story adds the file that could break it, so the row's evidence is
  a run of `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`
  (`spec.md` Clarifications item 6).

```yaml
- id: AC-001
  criterion: |-
    GIVEN an adapter author who has just wired a store that advances the checkpoint but forgets to flush its read-model writes, WHEN they run `projection_store_conformance!` against it, THEN the run fails at a **named** rule — `commit_is_atomic_with_the_read_model` — and the failure message identifies the store, not merely "1 test failed". Concretely: `CheckpointOnlyStore` is a real `ProjectionStore` + `ProjectionProbe` + `ProjectionFixture` implementation driven `begin` → `probe_write` → `commit` → *fresh handle* → `probe_read` + `checkpoint`, and it fails at the both-present-or-both-absent assertion and nowhere else.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (cargo test-target auto-discovery over tests/*.rs; no [[test]] entry, no autotests = false)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules (declared direction, CheckpointOnlyStore × commit_is_atomic_with_the_read_model)"

- id: AC-002
  criterion: |-
    GIVEN the same author, WHEN the suite reports that failure, THEN they can trust it is a *diagnosis* and not noise — `CheckpointOnlyStore` **passes** every projection rule it does not declare, so the one red rule is the one true statement about the defect. A skip is never counted as one of those passes: a rule that skipped for any reason other than a `(capability, reason)` pair this fixture itself declined fails the check as a hole in the map.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules (undeclared direction)"

- id: AC-003
  criterion: |-
    GIVEN an author reading `spec/SPECIFICATION.md` §4.11 and seeing an em-dash where `commit_advances_the_checkpoint`'s rejecting store should be, WHEN they ask what would fail that rule, THEN the registry answers with a store rather than a shrug: a `commit` that returns `Ok` and makes **neither** write durable — which satisfies PS-1's "or not at all" arm, therefore **passes** `commit_is_atomic_with_the_read_model` and **fails** `commit_advances_the_checkpoint`. It is the one store that separates the two rules. Its `provenance` records that this shape is also the evidence `.kb/open-questions/ps-1-states-no-progress-obligation.md` describes, **without** proposing a clause change.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (REGISTRY row + the enumerated store type)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules (both cells for the neither-write-durable store) + review that no diff touches spec/SPECIFICATION.md PS-1"

- id: AC-004
  criterion: |-
    GIVEN an author who already knows the event-store mutant registry, WHEN they open the projection one, THEN they read *one* shape, not a second dialect: a hand-written `const REGISTRY: &[Declared]` carrying `name`, `kind`, `fails`, `provenance`, `mode`, `expect` with the same semantics, `name` matching the store's own `Subject::NAME`, `fails` exact and empty **iff** `Kind::ConformantVariant`, and `expect: &[(rule, substring)]` keyed **by rule** from the first commit rather than `Option<&'static str>` retrofitted later. It is never generated from observed outcomes and never an associated const on the store's own impl.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (REGISTRY const + the Declared / Kind / FailureMode shape, shared or duplicated — record which and why)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutant_registry_is_exhaustive (the expect-pin-names-a-declared-rule assertion) + review against crates/happenstance-testkit/tests/mutation_coverage.rs:125-186"

- id: AC-005
  criterion: |-
    GIVEN the author's slice-mate about to add a new projection rule, WHEN they add it to `for_each_projection_store_rule!` without a store that fails it, THEN the build goes **red before their PR merges**, naming every uncovered rule (all of them, not the first) and pointing at where to write the wrong implementation. The rule universe comes from the single enumeration via `__emit_rule_names`, never a second hand-kept list, and the test has **no exemption list and must never acquire one**.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (rule universe taken from for_each_projection_store_rule! + __emit_rule_names)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::every_projection_rule_has_a_mutant"

- id: AC-006
  criterion: |-
    GIVEN an author adding a store to the projection binary, WHEN they get one of the three edits wrong — the type is enumerated but unregistered, registered but not enumerated, named twice, enumerated twice, declared to fail a misspelled rule, or pinned to a rule it does not declare — THEN the registry says which row and which field, rather than the store silently going undriven or a claim silently going unchecked. `kind` must also agree with `fails` being empty or not.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutant_registry_is_exhaustive"

- id: AC-007
  criterion: |-
    GIVEN an author who suspects a mutant is "failing for the wrong reason", WHEN the exactness meta-test runs, THEN it distinguishes *the rule's assertion fired* from *the store panicked* **positively** — under `FailureMode::Assertion` the panic origin must be inside the projection rules file and the message must not match `RUNTIME_PANICS`; under `FailureMode::StorePanic` the origin must be outside it and the message must contain the declared `expect` needle. A denylist over panic text is not sufficient and is not what is written: deleting a mutant's modelled defect once made the panic come from the fixture contract's own "not implemented" body, which a denylist passed silently. Assertions are on `Verdict` values, never on stdout.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (the Origin positive check, naming the projection rules module's real path)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules (Origin + RUNTIME_PANICS arms)"

- id: AC-008
  criterion: |-
    GIVEN a reviewer asking whether the mutant set is *representative* or merely *large*, WHEN they read any row, THEN they find a non-empty `provenance` naming the adapter shape or scenario that makes the defect plausible — a mistake an author would really make — because `struct AlwaysWrong` satisfies CF-1 mechanically and proves nothing. This holds for conformant-variant rows too: a variant owes an account of why it is *legally* different or it is a second copy of the reference store.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (REGISTRY provenance fields)"
  verifying_test: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs::projection_mutation_coverage::every_projection_mutant_states_its_provenance"

- id: AC-009
  criterion: |-
    GIVEN an author or reader trying to judge the projection suite's strength, WHEN they read the registry's module doc or any output of the binary, THEN they are told **which defects the set covers and which axes it leaves uncovered**, and are never handed a fraction: ADR-0010's prohibition is stated in the module doc and nothing in the binary computes or prints a pass rate over the mutant set. In the same doc, CF-5's projection half is a **named** deferral — `Kind::ConformantVariant` exists in the shape, no row uses it yet, `buffering-conformant-variant` is named as the story that closes it, and neither the "at least one variant is registered" assertion nor a `conformant_variants_pass_everything` sibling is landed in a form that would pass over an empty set.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (module doc on the REGISTRY const)"
  verifying_test: "Static review of the module doc at crates/happenstance-testkit/tests/projection_mutation_coverage.rs, corroborated by a repo grep finding no fraction/percentage computed over the projection REGISTRY (project Testing brief, AC-003 row: an absence has nothing to assert against)"

- id: AC-010
  criterion: |-
    GIVEN an author who reads the event-store registry first (it is the older and larger one), WHEN they reach its scope note, THEN it no longer tells them the projection port "has neither a suite nor a mutant yet" — a sentence this PR makes false — but points at `tests/projection_mutation_coverage.rs` while keeping the "Scope: the event-store port only" framing intact.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/mutation_coverage.rs:188-206 (the REGISTRY scope note — the story's second wiring point)"
  verifying_test: "Static review of the diff at crates/happenstance-testkit/tests/mutation_coverage.rs:188-206, plus `cargo test --workspace --all-features` proving the event-store binary still passes"

- id: AC-011
  criterion: |-
    GIVEN the same author targeting Cloudflare Workers, WHEN the workspace gate runs, THEN the new binary does not break their target: it carries one crate-level `#![cfg(not(target_arch = "wasm32"))]` — because `catch_unwind` cannot catch where there is no unwinder, and a meta-test that cannot observe a panic is the whole mechanism gone — **and states the cost**, that the stores in it are never type-checked for `wasm32`. In the same breath, the hostile stores reuse `Capability` and `RuleOutcome` unchanged and declare **no** borrowing GAT anywhere (`type Store<'a> where Self: 'a` on a foreign trait is one ingredient of a rustc ICE this repository already minimised and which still reproduces on 1.97.1).
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/projection_mutation_coverage.rs (crate-level cfg gate) + crates/happenstance-testkit/src/contract.rs:355-537 left undiffed"
  verifying_test: "`cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` (the mandatory wasm conformance-harness step in xtask/src/main.rs), plus `cargo xtask ci --fast` green and review that no borrowing GAT appears"
```
