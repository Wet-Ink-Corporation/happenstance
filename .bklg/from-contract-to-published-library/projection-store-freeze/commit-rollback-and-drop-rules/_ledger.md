---
item: HS-S0010
stage: implement
created: 2026-08-12T13:46:04.770Z
updated: 2026-08-12T13:46:04.770Z
---

# Acceptance ledger — Commit, rollback and dropped-batch rules, each with the store that fails it

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Six notes specific to this story, so no row is flipped on the wrong evidence:

- **Every rule row needs evidence in both directions.** A green run against `MemoryProjectionStore`
  is half of it; the other half is the rule's registered mutant failing at the pinned `expect`
  substring. A row flipped on the green half alone certifies a rule no adapter can fail, which is the
  decorative-rule failure `CLAUDE.md` names outright (`spec.md` AC-008, EC-008).
- **AC-001 may legitimately resolve as a reported skip.** If the landed `ProjectionFixture` carries a
  commit-fault capability the memory fixture declines, the row's evidence is the
  `RuleOutcome::Skipped { capability, reason }` **value** assertion plus the mutant direction — not a
  green pass. If it carries **no** such capability at all, the row is **not** flipped and **not**
  worked around by minting a second declension policy: it is reported (`spec.md` EC-002).
- **AC-008's evidence must name any `CheckpointOnlyStore` declaration growth.** A `fails` list that
  grew in this PR is the exactness meta-test working; record which rule caused it and why the rule was
  not weakened instead (`spec.md` EC-003).
- **AC-009 and AC-010 are the interaction-quality rows and are not satisfied by a green suite.**
  AC-009 needs the `wasm32` arm run explicitly — `cargo xtask ci --fast` omits the mandatory
  conformance-harness check — and AC-010 needs the diff-review counts (+7 names, +7 rows, +0 skip
  vocabularies, +0 harness files, +0 gate steps) recorded alongside the lint and `spec-trace` results.
- **AC-010's `spec-trace` evidence must be a regeneration, not an edit.** `cargo xtask spec-trace
  --write` over the generated §7.1–§7.2 region only; a hand-edited clause, maturity marker or
  `[FROZEN]` line invalidates the row (`spec.md` EC-004).
- **`RULE_FILES` must be checked before any lint row is flipped.** CF-6, CF-29, CF-33 and spec-trace's
  check 6 all sweep that list; a projection rules module outside it makes four gate steps print green
  over nothing. The report says which case obtained (`spec.md` EC-005, Clarifications item 5).

```yaml
- id: AC-001
  criterion: |-
    GIVEN an adapter author whose store can make one write of a batch fail (a trigger raising on the third insert, a `CHECK` armed for one write, a connection killed mid-statement), WHEN they run the projection suite, THEN they are told whether a `commit` that *reports failure* left the read model and the checkpoint exactly as they were; AND GIVEN a fixture with no fault to inject, the rule still runs, reports the fixture's **own stated reason**, and is never mistaken for a pass.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "failed_commit_leaves_both_unchanged (projection rules module), emitted into the tokio and blocking projection harnesses under crates/happenstance-testkit/tests/; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (partial-apply-reports-failure mutant), and the RuleOutcome::Skipped { capability, reason } value assertion for the declined arm"

- id: AC-002
  criterion: |-
    GIVEN an adapter author who must call `rollback` explicitly because Rust has no `async Drop`, WHEN they run the suite, THEN they learn whether an explicit rollback left **both halves** — read model and checkpoint — exactly as the last successful commit left them, rather than only whether it returned `Ok`.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "rollback_leaves_both_unchanged (projection rules module) green against MemoryProjectionStore under the tokio and blocking harnesses; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the rollback-discards-only-its-buffer mutant)"

- id: AC-003
  criterion: |-
    GIVEN an adapter author whose batch holds a pooled connection, WHEN a batch is dropped bare — no `commit`, no `rollback` — THEN they are told not merely that the work rolled back but that **the store is still usable afterwards**: a second batch opens on the same handle, commits, and its row reads back. (Project AC-010; the second half is the rule.)
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "dropped_batch_leaves_store_usable (projection rules module), asserting first key absent, second key present and the checkpoint advanced; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the Drop-returns-the-pooled-connection-to-nothing mutant that answers Busy forever)"

- id: AC-004
  criterion: |-
    GIVEN an adapter author whose application holds more than one store instance, WHEN a batch begun on store A is committed on store B, THEN they are told it is rejected as `CommitError::ForeignBatch` **and** that neither store was disturbed — so the run-time stamp is proven to be a check and not a comment.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_rejects_a_foreign_batch (projection rules module), building two isolated stores from two open() calls on the impl AsyncFn() -> F rule argument (crates/happenstance-testkit/src/registry.rs:45-49) and deliberately not using SECOND_HANDLE; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the batch-carries-no-per-instance-stamp mutant)"

- id: AC-005
  criterion: |-
    GIVEN a narrow projection that considered a range and applied nothing from it, WHEN its author commits an empty batch at that range's end, THEN the suite tells them the checkpoint advanced — a high-water mark of *consideration*, not of application — so their projection does not re-scan the same range forever.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_accepts_a_position_the_batch_did_not_write (projection rules module); plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test against ValidatingCommitStore, the store spec/SPECIFICATION.md:5680-5685 names for this rule"

- id: AC-006
  criterion: |-
    GIVEN an adapter author whose runner restarts and replays, WHEN a commit names a position strictly below the current checkpoint, THEN they are told it is refused as `CheckpointRegression { current, attempted }` with both fields naming what the store was actually given, and that neither half moved — AND they are never told that an *equal* position must be refused, because the clause permits accepting it.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_rejects_a_regressing_position (projection rules module), asserting the variant, both field values derived from SequencePosition::FIRST/.next(), and both halves unchanged; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the unconditional UPDATE checkpoint SET position = ? mutant). Review check: no assertion anywhere on the equal-position case"

- id: AC-007
  criterion: |-
    GIVEN an adapter author running several projections over one store, WHEN each advances at its own rate, THEN they are told whether one projection's commit disturbed another's checkpoint or rows — the failure a single-row checkpoint table produces silently.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "distinct_projections_advance_independently (projection rules module), two ProjectionIds committed at different positions with distinct probe rows; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the single-row-checkpoint-table mutant)"

- id: AC-008
  criterion: |-
    GIVEN an adapter author who has just gone green, WHEN they ask what a green run is worth, THEN each of the seven new rules has a **registered wrong implementation that fails exactly it**, with provenance naming a real adapter shape and an `expect` pin naming the exact assertion — and **no pass rate is quoted anywhere over the mutant set**. (Project AC-003.)
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration, plus the projection REGISTRY: &[Declared] and its mutants module landed by projection-mutant-registry"
  verifying_test: "the projection siblings of every_rule_has_a_mutant, mutant_registry_is_exhaustive and mutants_fail_exactly_their_declared_rules (shape at crates/happenstance-testkit/tests/mutation_coverage.rs:141-186, :2734, :2754, :2889) green over the widened nine-rule × eight-store matrix, including any CheckpointOnlyStore fails growth; plus a review-time file:line into the registry module doc for the no-pass-rate prohibition"

- id: AC-009
  criterion: |-
    STATE invariants. GIVEN an adapter author reading one run's output, WHEN the suite executes, THEN every one of the seven rules is *present in the run on every runtime* — enumerated once, emitted by the tokio, blocking and `wasm32` emitters alike, with none silently absent, none reachable only behind an extra feature flag, and a failure reported **at the failing rule's own name** rather than as an anonymous harness abort; a declined capability yields a reported skip that **occludes nothing** — every other rule's outcome still appears — and each rule leaves the store usable for the next, so no rule's effect and no run ordering can change another's verdict.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration and its tokio / blocking / wasm32 emitters"
  verifying_test: "the projection orphan meta-test over the rules module (landed by projection-suite-entry-point); an explicit run of cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown (xtask/src/main.rs:231-243); and the projection mutants_fail_exactly_their_declared_rules meta-test, whose positive panic-origin check is what proves a failure names the rule rather than the harness"

- id: AC-010
  criterion: |-
    COMPOSITION invariants. GIVEN the only surface this project has — the text a run prints and the record of what changed (`_design.md` declares `surfaces: []`) — WHEN the seven rules land, THEN every reported outcome carries **real composed presentation** through the one existing renderer rather than bare output: one skip vocabulary and one line shape, SKIP {rule}: fixture declines `{capability}` — {reason}, naming the capability constant the author can actually change and carrying the fixture's own words; AND the density budget holds — **one** enumeration gains exactly seven names (2 → 9), **one** registry gains exactly seven rows (1 → 8 stores), **zero** new skip types, **zero** new harness files, **zero** new gate steps; AND the human-readable record exists — a `CHANGELOG.md` entry naming each rule and the defect it detects, and a regenerated §7.1–§7.2 so no rule is left marked † *not found*.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration; presentation reused unchanged from crates/happenstance-testkit/src/contract.rs:500-537 (RuleOutcome::report / skip_line)"
  verifying_test: "changelog_names_every_rule with MIN_CHARS_PER_RULE = 120 (xtask/src/lints.rs:525, :446) and cargo xtask spec-trace green over the region regenerated by spec-trace --write (xtask/src/spec_trace.rs:200-201); plus the RuleOutcome value assertion for the skip line shape and capability identifier, and a recorded diff review against the five density counts"
```
