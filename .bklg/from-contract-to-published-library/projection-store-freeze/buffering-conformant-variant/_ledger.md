---
item: HS-S0013
stage: implement
created: 2026-08-12T13:46:07.787Z
updated: 2026-08-12T13:46:07.787Z
---

# Acceptance ledger — A second, structurally unlike batch shape passes the whole suite

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
  criterion: "GIVEN an adapter author whose storage holds no connection, transaction or lock between opening a write set and committing it (the Workers SqlStorage / Neon-over-one-shot-HTTP shape, references/adapter-shapes.md), WHEN a BufferingProjectionStore written in exactly that shape is driven through begin → probe writes → commit, THEN the store's committed read model and checkpoint are observably unchanged at every point before commit returns Ok, and change only at that moment — so the second shape is unlike MemoryProjectionStore on the batch-shape axis by assertion, not by claim in a doc comment."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/ — the buffering store's own source file, included by both the projection mutant binary and the new conformance harness via the #[path = \"…\"] mod convention"
  verifying_test: "the store's own before/after-commit visibility test, in the buffering store's file under crates/happenstance-testkit/tests/ — open a batch, probe-write, read committed state out of band and assert unchanged; commit and assert changed. Asserted independently of the conformance suite, and never against a literal position value."
- id: AC-002
  criterion: "GIVEN an adapter author who wants 'correct' to be executable rather than interpreted (personas-and-journeys.md:125-127), WHEN they read the gate output for the buffering fixture, THEN they see one named test per projection rule, zero failures, and any rule whose capability the fixture declines still present as a test reporting RuleOutcome::Skipped with the fixture's stated reason — never absent from the binary and never indistinguishable from a pass."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/ — a new projection_store_conformance! harness target beside crates/happenstance-testkit/tests/memory_conformance.rs"
  verifying_test: "the full set of projection rule tests emitted by projection_store_conformance!(<buffering fixture>) in the new harness target (crates/happenstance-testkit/tests/projection_conformance*.rs), run by cargo test -p happenstance-testkit; skip-versus-pass asserted on RuleOutcome values per crates/happenstance-testkit/src/contract.rs:458-537, with skip text from RuleOutcome::skip_line (contract.rs:500-507) unchanged"
- id: AC-003
  criterion: "GIVEN an adapter author who must trust that 'this store is legal' is a claim someone committed to rather than a run that happened to be green, WHEN they open the projection mutant registry, THEN they find exactly one new hand-written Declared row — kind: Kind::ConformantVariant, fails: &[], expect: &[], and a non-empty provenance naming the real adapter shape that makes buffering plausible — and no row generated from observed outcomes."
  satisfied: false
  evidence: ""
  mount_point: "the projection mutant registry's REGISTRY: &[Declared] and its for_each_…_mutant! enumeration, wherever projection-mutant-registry landed them (crates/happenstance-testkit/tests/projection_mutation_coverage.rs, or the single mutation_coverage.rs binary — this story does not re-decide it)"
  verifying_test: "the projection siblings of mutant_registry_is_exhaustive (CF-2, crates/happenstance-testkit/tests/mutation_coverage.rs:2751-2860) and every_mutant_states_its_provenance (CF-4), plus a reviewed diff confirming the row is hand-written and not generated (mutation_coverage.rs:125-141)"
- id: AC-004
  criterion: "GIVEN an adapter author whose fear is a rule that asserts more than the specification requires and rejects their legal store in the field (personas-and-journeys.md:150-165), WHEN the projection family's CF-5 positive control runs, THEN it asserts two things it could not assert before this story — that no variant was rejected by any rule, and that every projection rule executed against a variant — so the control stops being a test over an empty set."
  satisfied: false
  evidence: ""
  mount_point: "the projection mutant registry binary under crates/happenstance-testkit/tests/ — the projection sibling of conformant_variants_pass_everything, reading the same REGISTRY the AC-003 row was added to"
  verifying_test: "the projection sibling of conformant_variants_pass_everything (modelled on crates/happenstance-testkit/tests/mutation_coverage.rs:3071-3120, vacuity guard at :2856-2858, CF-6 failure message at :3094-3108); non-vacuity demonstrated by locally deleting the row and observing the guard fail — that deletion is not committed"
- id: AC-005
  criterion: "GIVEN a reviewer, and the adapter author reading over their shoulder, who must not have to reconcile two cargo test invocations by hand, WHEN they read one cargo xtask ci run, THEN both batch shapes are green inside it with both fixtures named in the output — because the new harness is an ordinary tests/ target with no #[ignore], no opt-in feature and no separate command — and the run's mandatory wasm32 --tests check stays green over the widened target set."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/ — the new harness target as an ordinary tests/ target of happenstance-testkit, picked up by cargo xtask ci's tests step with no gate-step change (xtask/src/main.rs)"
  verifying_test: "cargo xtask ci run whole on a clean tree — its tests step running both the new buffering harness and crates/happenstance-testkit/tests/memory_conformance.rs, plus the existing 'wasm32 check of the conformance harnesses' step (cargo check --locked -p happenstance-testkit --tests --target wasm32-unknown-unknown, xtask/src/main.rs:231-243)"
- id: AC-006
  criterion: "GIVEN an adapter author reading the registry to learn what the suite has actually been proven against, WHEN they follow the row to the store, THEN the store and its fixture are found once, in one source file, reached by both the mutant binary and the conformance harness — so the registry describes the same code the harness ran, and a second drifting copy cannot exist."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/tests/ — one source file holding the store and fixture, included by both consumers via the #[path = \"…\"] mod convention (crates/happenstance-testkit/tests/mutation_coverage.rs:64-65) and satisfying the Subject: Fixture + Sized harness convention (crates/happenstance-testkit/tests/mutation_coverage/harness.rs:60-63)"
  verifying_test: "reviewed diff over crates/happenstance-testkit/tests/** asserting exactly one definition of the buffering store type, alongside cargo clippy --workspace --all-targets --all-features -- -D warnings (which catches a dead duplicate but not a used one, which is why the diff assertion is named rather than inferred)"
- id: AC-007
  criterion: "GIVEN ps3-batch-shape-finding (this story's slice-mate), which must write PS-3's evidence from what this run showed rather than from memory, WHEN this story completes, THEN it hands over a written two-shape observation — which projection rules, if any, needed different handling for the two shapes and where, including the 'they agreed everywhere' outcome, which is the finding and not a silent success — and the diff proves the CF-6 discipline was kept: zero rules added, zero rules edited, zero new capabilities minted to gate a rule, and nothing under crates/happenstance-core/src/."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/from-contract-to-published-library/projection-store-freeze/buffering-conformant-variant/ — the story's own backlog folder, which is where ps3-batch-shape-finding reads from; and the PR-boundary globs, which are where the discipline half is asserted"
  verifying_test: "review of the companion observation note in the story folder (specific rules named, or an explicit 'agreed everywhere' with what was exercised, and no pass rate quoted per .kb/decisions/0010-the-suite-must-prove-itself.md), plus a diff-scope assertion over crates/happenstance-testkit/tests/** — zero conformance rules added or edited, zero new capabilities, nothing under crates/happenstance-core/src/, and no CF-29 mutant-plus-changelog obligation incurred (spec/SPECIFICATION.md:8141-8143)"
```
