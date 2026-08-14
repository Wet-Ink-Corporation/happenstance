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
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:394-448` (`rollback_leaves_both_unchanged`), named at `:784` in `for_each_projection_store_rule!`. Green against `MemoryProjectionStore` under both host emitters — test ids `projection_conformance::rollback_leaves_both_unchanged` and `projection_conformance_blocking::rollback_leaves_both_unchanged`. Red against its mutant: `UnrolledBackStore` (`crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs:119-125`), a `rollback` that releases its connection without issuing `ROLLBACK`, registered at `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:353-367` and asserted by `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` at the pinned substring 'must leave the read model as it was'. Read-back is through a **fresh handle** and `probe_read`, never through the rolled-back batch (`projection.rs:425-447`). Both halves are asserted: the row AND the checkpoint, the latter compared against what a fresh handle saw BEFORE the rollback rather than against a position, so the rule asserts preservation and makes no progress claim. Clause PS-8 `[FROZEN]`."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "rollback_leaves_both_unchanged (projection rules module) green against MemoryProjectionStore under the tokio and blocking harnesses; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the rollback-discards-only-its-buffer mutant)"

- id: AC-003
  criterion: |-
    GIVEN an adapter author whose batch holds a pooled connection, WHEN a batch is dropped bare — no `commit`, no `rollback` — THEN they are told not merely that the work rolled back but that **the store is still usable afterwards**: a second batch opens on the same handle, commits, and its row reads back. (Project AC-010; the second half is the rule.)
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:454-505` (`dropped_batch_leaves_store_usable`), named at `:785`. Green against the oracle under both host emitters. The non-vacuity anchor is present and is the second assertion — `probe_read(SECOND_KEY) == Some(SECOND_VALUE)` at `:480-491`, the claim a 'rolls back' -only rule cannot make. Red against `PooledConnectionStore` (`mutants.rs:137-145`), whose batch `Drop` returns the store's one connection to nothing so the next `commit` answers `Busy`; registered at `projection_mutation_coverage.rs:368-380`, pinned at 'commit should succeed'. The connection is genuinely modelled rather than asserted about: `MutantStore` holds an `Rc<Cell<bool>>` and the batch holds a share in it (`correct.rs:305-336`, `:364-380`, `:421-441`), because without a resource a batch can fail to give back, PS-7's second half is true by construction and the rule would be decorative. Two further stores fail it and say so in their rows (EC-003, see AC-008). Clause PS-7 `[FROZEN]`."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "dropped_batch_leaves_store_usable (projection rules module), asserting first key absent, second key present and the checkpoint advanced; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the Drop-returns-the-pooled-connection-to-nothing mutant that answers Busy forever)"

- id: AC-004
  criterion: |-
    GIVEN an adapter author whose application holds more than one store instance, WHEN a batch begun on store A is committed on store B, THEN they are told it is rejected as `CommitError::ForeignBatch` **and** that neither store was disturbed — so the run-time stamp is proven to be a check and not a comment.
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:522-577` (`commit_rejects_a_foreign_batch`), named at `:786`. It builds **two isolated stores from two `open()` calls** on the `impl AsyncFn() -> F` every rule is handed (`:534-537`) and deliberately spells **no** `must!(F: SECOND_HANDLE)` — PS-15 says the rule wants two isolated stores rather than two handles onto one, and the rule's doc records that. It asserts the `CommitError::ForeignBatch` variant AND both stores' checkpoint and probe reads (`:552-575`), so the stamp is proven to be a check rather than a comment. Green against the oracle under both host emitters. Red against `TypeStampedBatchStore` (`mutants.rs:158-166`), whose batch carries an identity minted per *type*, registered at `projection_mutation_coverage.rs:381-397` and pinned at 'must be rejected as `CommitError::ForeignBatch`'. Because it spells no gate, it also runs and **passes** against `DecliningProjectionFixture`, which is asserted by omission from `PROJECTION_MUST_REJECT` (`crates/happenstance-testkit/tests/mutation_coverage.rs:3339-3366`) through the `Verdict::Passed` arm of `assert_projection_declension`. Clause PS-15, case E2E-19."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_rejects_a_foreign_batch (projection rules module), building two isolated stores from two open() calls on the impl AsyncFn() -> F rule argument (crates/happenstance-testkit/src/registry.rs:45-49) and deliberately not using SECOND_HANDLE; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the batch-carries-no-per-instance-stamp mutant)"

- id: AC-005
  criterion: |-
    GIVEN a narrow projection that considered a range and applied nothing from it, WHEN its author commits an empty batch at that range's end, THEN the suite tells them the checkpoint advanced — a high-water mark of *consideration*, not of application — so their projection does not re-scan the same range forever.
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:583-624` (`commit_accepts_a_position_the_batch_did_not_write`), named at `:787`. It commits an **empty** batch and asserts the checkpoint advanced to the position it named. Green against the oracle under both host emitters. Red against `ValidatingCommitStore` (`mutants.rs:184-193`) — the store the specification itself names for this rule (`spec/SPECIFICATION.md:5680-5685`) — registered at `projection_mutation_coverage.rs:398-415` and pinned at 'commit should succeed'. That mutant is what makes the rule non-decorative: validating the position is a *reasonable* misreading that would be equally conformant without the rule, so two backends could disagree and both pass. `UncommittedTransactionStore` also fails it, at the checkpoint assertion, and its row says so. Clause PS-21 `[FROZEN]`, case E2E-23."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_accepts_a_position_the_batch_did_not_write (projection rules module); plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test against ValidatingCommitStore, the store spec/SPECIFICATION.md:5680-5685 names for this rule"

- id: AC-006
  criterion: |-
    GIVEN an adapter author whose runner restarts and replays, WHEN a commit names a position strictly below the current checkpoint, THEN they are told it is refused as `CheckpointRegression { current, attempted }` with both fields naming what the store was actually given, and that neither half moved — AND they are never told that an *equal* position must be refused, because the clause permits accepting it.
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:630-692` (`commit_rejects_a_regressing_position`), named at `:788`. It asserts the variant AND both field values — `(current, attempted) == (later, earlier)` at `:650-659` — and then that neither half moved (`:678-691`). Every position is derived from `SequencePosition::FIRST` and the `after` helper at `:214-227`, which takes `next()`'s `None` arm as a panic with a message rather than a silent `unwrap` (NF-001); the CF-6 lint confirms it: `cargo xtask ci` prints 'CF-6: no position-shaped literals in 4 rule file(s)'. Green against the oracle under both host emitters. Red against `UnconditionalCheckpointStore` (`mutants.rs:207-217`), registered at `projection_mutation_coverage.rs:416-432`, pinned at 'must be refused as `CommitError::CheckpointRegression`'. **Review check discharged: there is no assertion anywhere in the body about an equal position** — no commit in it names a position the store already holds, which is what PS-22 permits accepting. Clause PS-22."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "commit_rejects_a_regressing_position (projection rules module), asserting the variant, both field values derived from SequencePosition::FIRST/.next(), and both halves unchanged; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the unconditional UPDATE checkpoint SET position = ? mutant). Review check: no assertion anywhere on the equal-position case"

- id: AC-007
  criterion: |-
    GIVEN an adapter author running several projections over one store, WHEN each advances at its own rate, THEN they are told whether one projection's commit disturbed another's checkpoint or rows — the failure a single-row checkpoint table produces silently.
  satisfied: true
  evidence: "Integration + Unit, both directions. Rule: `crates/happenstance-testkit/src/projection.rs:698-756` (`distinct_projections_advance_independently`), named at `:789`. Two `ProjectionId`s committed at different positions with distinct probe keys, then four assertions: each id reads back its own checkpoint and each row is where it was left (`:721-755`). Green against the oracle under both host emitters. Red against `SingleRowCheckpointStore` (`mutants.rs:231-239`), a checkpoint table with one row and no key, registered at `projection_mutation_coverage.rs:433-449` and pinned at 'one commit advances exactly one projection'. `CheckpointOnlyStore` and `UncommittedTransactionStore` also fail it — at the row half and the checkpoint half respectively — and their rows say so. Clause PS-23, case E2E-28."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration"
  verifying_test: "distinct_projections_advance_independently (projection rules module), two ProjectionIds committed at different positions with distinct probe rows; plus its cell in the projection mutants_fail_exactly_their_declared_rules meta-test (the single-row-checkpoint-table mutant)"

- id: AC-008
  criterion: |-
    GIVEN an adapter author who has just gone green, WHEN they ask what a green run is worth, THEN each of the seven new rules has a **registered wrong implementation that fails exactly it**, with provenance naming a real adapter shape and an `expect` pin naming the exact assertion — and **no pass rate is quoted anywhere over the mutant set**. (Project AC-003.)
  satisfied: true
  evidence: "**Six of seven, and the seventh is blocked rather than quietly dropped**: `failed_commit_leaves_both_unchanged` is not landed (AC-001, EC-002), so this row certifies the six rules that did land. Each has a registered wrong implementation that fails **exactly** it, with provenance naming a real adapter shape and an `expect` pin naming the exact assertion — registry rows at `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:353-450`. The four projection exactness meta-tests are green over the widened 8-rule x 8-store matrix: `projection_mutation_coverage::every_projection_rule_has_a_mutant`, `::projection_mutant_registry_is_exhaustive`, `::projection_mutants_fail_exactly_their_declared_rules`, `::every_projection_mutant_states_its_provenance`, and the event-store family's own four are still green beside them. RED first, for the right reason: adding the six rule names to the enumeration turned CF-1 red naming all six — `[\"rollback_leaves_both_unchanged\", \"dropped_batch_leaves_store_usable\", \"commit_rejects_a_foreign_batch\", \"commit_accepts_a_position_the_batch_did_not_write\", \"commit_rejects_a_regressing_position\", \"distinct_projections_advance_independently\"]` — and green only when all six mutants landed. **EC-003 obtained and was honoured, not worked around**: `dropped_batch_leaves_store_usable` and `distinct_projections_advance_independently` legitimately catch `CheckpointOnlyStore`, and four rules catch `UncommittedTransactionStore`, so both `fails` lists grew IN THIS PR (`:264-300`) rather than the new rules being weakened to preserve the old declarations. Both growths carry a comment saying which repair was refused and why. No pass rate appears in code, doc or changelog: `grep` for a fraction over `REGISTRY` returns nothing, and the registry's own doc carries ADR-0010's prohibition at `:235-240`."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration, plus the projection REGISTRY: &[Declared] and its mutants module landed by projection-mutant-registry"
  verifying_test: "the projection siblings of every_rule_has_a_mutant, mutant_registry_is_exhaustive and mutants_fail_exactly_their_declared_rules (shape at crates/happenstance-testkit/tests/mutation_coverage.rs:141-186, :2734, :2754, :2889) green over the widened nine-rule × eight-store matrix, including any CheckpointOnlyStore fails growth; plus a review-time file:line into the registry module doc for the no-pass-rate prohibition"

- id: AC-009
  criterion: |-
    STATE invariants. GIVEN an adapter author reading one run's output, WHEN the suite executes, THEN every one of the seven rules is *present in the run on every runtime* — enumerated once, emitted by the tokio, blocking and `wasm32` emitters alike, with none silently absent, none reachable only behind an extra feature flag, and a failure reported **at the failing rule's own name** rather than as an anonymous harness abort; a declined capability yields a reported skip that **occludes nothing** — every other rule's outcome still appears — and each rule leaves the store usable for the next, so no rule's effect and no run ordering can change another's verdict.
  satisfied: true
  evidence: "**STATE invariants, all four verified.** *Present on every runtime, enumerated once*: the six names are in the single `for_each_projection_store_rule!` at `crates/happenstance-testkit/src/projection.rs:772-792` and nowhere else; the orphan meta-test `no_orphan_projection_rules` (`:911-936`) is green in both directions, and it fails loudly rather than open if its source scan ever matches nothing. *On the constrained target*: `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` run explicitly (it is the mandatory conformance-harness step `cargo xtask ci --fast` omits) — clean, so `projection_conformance_wasm.rs` type-checks all eight rules with no separately maintained subset and no extra feature flag. *A failure names the failing rule rather than aborting the harness*: one libtest test per rule per emitter — `projection_conformance` and `projection_conformance_blocking` each report `8 passed` — and the positive panic-origin check in `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` asserts every mutant's rejection was raised inside `happenstance-testkit/src/projection.rs`, which is what proves a red names the rule and not the harness. *Non-occlusion and independence*: every rule produces its own `RuleOutcome`, and independence is by construction — each rule opens its own fixture through `impl AsyncFn() -> F`, uses its own `ProjectionId` derived from its own name, and the exactness meta-test drives all eight stores through all eight rules in one process without ordering assumptions."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration and its tokio / blocking / wasm32 emitters"
  verifying_test: "the projection orphan meta-test over the rules module (landed by projection-suite-entry-point); an explicit run of cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown (xtask/src/main.rs:231-243); and the projection mutants_fail_exactly_their_declared_rules meta-test, whose positive panic-origin check is what proves a failure names the rule rather than the harness"

- id: AC-010
  criterion: |-
    COMPOSITION invariants. GIVEN the only surface this project has — the text a run prints and the record of what changed (`_design.md` declares `surfaces: []`) — WHEN the seven rules land, THEN every reported outcome carries **real composed presentation** through the one existing renderer rather than bare output: one skip vocabulary and one line shape, SKIP {rule}: fixture declines `{capability}` — {reason}, naming the capability constant the author can actually change and carrying the fixture's own words; AND the density budget holds — **one** enumeration gains exactly seven names (2 → 9), **one** registry gains exactly seven rows (1 → 8 stores), **zero** new skip types, **zero** new harness files, **zero** new gate steps; AND the human-readable record exists — a `CHANGELOG.md` entry naming each rule and the defect it detects, and a regenerated §7.1–§7.2 so no rule is left marked † *not found*.
  satisfied: true
  evidence: "**COMPOSITION invariants, with the counts as they actually landed.** *One skip vocabulary, one line shape*: `RuleOutcome::report` / `skip_line` are reused **unchanged** — this story's diff contains no change to `crates/happenstance-testkit/src/contract.rs`, and the projection family's `must!` (`crates/happenstance-testkit/src/projection.rs:74-94`) is the one that existed before it. No `require!` was written, so no second declension path was minted. *Density budget*: **+6** rule names (2 -> 8, not the planned 2 -> 9, because AC-001's rule is blocked and reported rather than landed), **+6** registry rows (2 -> **8 stores**, which is the planned end state), **+0** skip vocabularies, **+0** harness files, **+0** gate steps, **+0** dependencies, **+0** public items. *The human-readable record exists and is machine-checked*: `CHANGELOG.md` gains one entry per landed rule naming the defect it detects plus one for the registry itself, and `cargo xtask ci` prints `CF-29: all 103 rules in 4 file(s) have a changelog entry` with `MIN_CHARS_PER_RULE = 120`; the stale sentence claiming `CheckpointOnlyStore` was a carried debt was corrected in the same entry it appears in. *No rule is left marked as not found*: `cargo xtask spec-trace --write` regenerated the **generated region only** — `git diff spec/SPECIFICATION.md` is 6 insertions and 6 deletions, all inside `<!-- BEGIN GENERATED -->`, each removing a `†` from PS-7, PS-8, PS-15, PS-21, PS-22 and PS-23; no clause prose, no maturity marker and nothing `[FROZEN]` was hand-edited (EC-004). *`RULE_FILES` was checked before any lint row was flipped* (EC-005): `xtask/src/spec_trace.rs:88-93` is `[&str; 4]` and **already contains** `crates/happenstance-testkit/src/projection.rs`, added by `projection-suite-entry-point`, so no edit was needed and none was made — the CF-6, CF-29, CF-33 and check-6 sweeps all reach the projection rules module, which the gate output confirms by counting 4 rule files and 103 rules."
  mount_point: "crates/happenstance-testkit/src/registry.rs — the single for_each_projection_store_rule! enumeration; presentation reused unchanged from crates/happenstance-testkit/src/contract.rs:500-537 (RuleOutcome::report / skip_line)"
  verifying_test: "changelog_names_every_rule with MIN_CHARS_PER_RULE = 120 (xtask/src/lints.rs:525, :446) and cargo xtask spec-trace green over the region regenerated by spec-trace --write (xtask/src/spec_trace.rs:200-201); plus the RuleOutcome value assertion for the skip line shape and capability identifier, and a recorded diff review against the five density counts"
```
