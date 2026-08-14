---
item: "HS-S0010"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — Commit, rollback and dropped-batch rules, each with the store that fails it

## Findings Ledger

**Amended 2026-08-14 — ten of ten ACs satisfied.** This report first read *nine of
ten; AC-001 is BLOCKED, and it is blocked on a decision this story is not entitled
to make.* That was the correct answer at the time and the escalation worked: the
decision came back (`_slices.md`, *commit-atomicity-and-mutants — human decision*),
DT-3 was **amended** to add a commit-fault capability rather than the rule being
descoped, and the seventh rule landed with the capability, the gate and the mutant
the amendment obliged. The block is kept in the row below rather than erased,
because the record of *how* the rule arrived is the point: it arrived by a scope
decision, not by an implementer deciding a signed-off enumeration was one constant
short.

Seven conformance rules landed, each with a registered wrong implementation that
fails **exactly** it, each green against the oracle on both host emitters and
type-checked on `wasm32`. One of the seven —
`failed_commit_leaves_both_unchanged` — is answered by a **reported skip** against
the reference fixture rather than by a pass, because the oracle has no fault to
arm and says so in the line it prints.

**The delta this story is.** Before it the projection suite proved one thing: a
store that drops the read-model write is caught. After it the whole commit path is
differential — an explicit rollback, a batch dropped bare, a batch begun on the
wrong store, a position the batch never wrote, a position that goes backwards, and
two projections that must not share a checkpoint row — plus, after the amendment,
a commit that reported failure and left a half behind. Two rules became nine; two
hostile stores became nine.

**The claim this story does not make.** The projection port is not frozen. Eight of
§4.11's seventeen rules are still owed (`fresh_projection_has_no_checkpoint` and
the four reset rules to `reset-rules`, the three read-through and rebuild rules to
theirs), and CF-5's conformant variant is still deferred. **PS-1's second conjunct
is now enforced** — that sentence replaces this paragraph's original *"is
unenforced"* — but with one honest qualification a freeze verdict must carry: no
fixture in this workspace except the mutant harness can arm a commit fault, so the
rule is *demonstrated* against `PartialCommitStore` and *skipped* against the
oracle. It becomes a rule an adapter actually runs when the first adapter that can
inject a fault declares the capability, which is `happenstance-sqlite`'s to do.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — was BLOCKED, now SATISFIED by amendment.** The block was real: the rule must be gated on a fixture capability that can arm a commit fault, in `MID_BATCH_FAULT`'s mould, and the signed-off capability set had none. It was reported (EC-002) rather than closed locally, the human took the fourth path — **amend DT-3** — and the rule landed under it. | The amendment is recorded in the artefact it amends: `_design.md`'s DT-3 table gains a fourth row, dated, with what forced it and why a fourth constant under the existing policy is not the second declension policy `cf-40-fixture-limits-ownership.md` forbids. `ProjectionFixture::COMMIT_FAULT` + `arm_commit_fault` at `crates/happenstance-testkit/src/contract.rs:605-666`; the rule at `crates/happenstance-testkit/src/projection.rs:440-560`; `PartialCommitStore` at `tests/projection_mutation_coverage/mutants.rs:105-144`, registered at `projection_mutation_coverage.rs:361-389`; the reported skip asserted as a `RuleOutcome::Skipped` **value** at `tests/mutation_coverage.rs:3527-3577`. | **Two things a later reader should know.** (1) The capability is **required, not defaulted**, deliberately: a default would carry a testkit-written reason, and DT-3 permits exactly one of those. (2) The rule folds CF-39's anti-vacuity demand in as its own precondition rather than splitting it into a second rule; the residue — a fixture whose `arm_commit_fault` has an empty body — is caught by the rule's first assertion but has no registered *fixture*-level mutant, which the registry's own scope note names. |
| **AC-002 — an explicit rollback preserves both halves.** | `crates/happenstance-testkit/src/projection.rs:394-448`; green on both host emitters; red against `UnrolledBackStore` at `"must leave the read model as it was"`. | None. The checkpoint is compared against what a fresh handle saw *before* the rollback, so the rule asserts preservation and claims nothing about progress. |
| **AC-003 — a dropped batch rolls back AND leaves the store usable.** | `projection.rs:454-505`, non-vacuity anchor at `:480-491`; red against `PooledConnectionStore` at `"commit should succeed"`. The connection is genuinely modelled (`correct.rs:305-336`, `:364-380`), not asserted about. | None. Project AC-010 discharged. |
| **AC-004 — a foreign batch is refused and neither store moves.** | `projection.rs:522-577`, two `open()` calls and deliberately no `SECOND_HANDLE` gate; red against `TypeStampedBatchStore`. | **Worth a reviewer's eye:** this is the one projection rule absent from `PROJECTION_MUST_REJECT`, which makes its *pass* against a fixture declining everything an assertion rather than an accident. Adding it there "for safety" would make that assertion unreachable. |
| **AC-005 — a commit may name a position the batch did not write.** | `projection.rs:583-624`; red against `ValidatingCommitStore`, the store the specification itself names for this rule. | None. |
| **AC-006 — a regressing position is refused with both fields, and nothing moves.** | `projection.rs:630-692`; red against `UnconditionalCheckpointStore`. Positions from `SequencePosition::FIRST` and `after` (`:214-227`); CF-6 lint green over 4 rule files. | **Review check discharged:** no assertion anywhere about an *equal* position — PS-22 permits accepting one. |
| **AC-007 — projections advance independently.** | `projection.rs:698-756`; red against `SingleRowCheckpointStore`. | None. |
| **AC-008 — all seven rules have a mutant that fails exactly them; no pass rate anywhere.** (Amended: six at first report, seven after the DT-3 amendment landed `failed_commit_leaves_both_unchanged` with `PartialCommitStore`.) | The four projection exactness meta-tests green over the widened 9×9 matrix, plus the event-store family's four beside them. RED first: CF-1 named all six new rules as decorative before any mutant landed. | **EC-003 obtained and was honoured.** `CheckpointOnlyStore`'s declaration grew to three rules and `UncommittedTransactionStore`'s to five, **in this PR**, each with a comment naming the refused repair. The five-rule row is inflation rather than vacuity, and it meets the event-store registry's stated bar: no rule is covered by that store alone except `commit_advances_the_checkpoint`, which is the one it exists for. |
| **AC-009 — every rule present on every runtime, reported at its own name.** | Single enumeration + `no_orphan_projection_rules`; `9 passed` on each host emitter (8 at first report); `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` run explicitly and clean. | **Stated cost, restated so the gate is not over-read:** the wasm step covers the *rules* on that target and none of the nine mutants — the mutant binary is gated off `wasm32` because `catch_unwind` cannot catch without an unwinder. |
| **AC-010 — one skip vocabulary, one enumeration, one registry, and a human-readable record.** | `RuleOutcome` and `skip_line` reused unchanged — the `contract.rs` diff adds the new capability constant and its arming method and touches no line of either; **+7** rule names (2 → 9, the planned end state), **+7** registry rows (2 → **9 stores**, the planned end state), **+1** fixture capability constant — the DT-3 amendment itself, **+0** skip vocabularies, **+0** harness files, **+0** gate steps, **+0** dependencies, **+0** public items. `CF-29: all 104 rules in 4 file(s) have a changelog entry`. `spec-trace` green after `--write`: 6 insertions, 6 deletions, all inside the generated region. | **EC-005 checked, not assumed:** `RULE_FILES` is `[&str; 4]` and already contained the projection rules module, so no edit was needed. The gate counting **4 rule files and 104 rules** is what proves the four sweeps reach it rather than printing green over nothing. |

**One correction of an inherited claim.** The `CHANGELOG.md` entry for
`commit_is_atomic_with_the_read_model`, and two rustdoc paragraphs in
`crates/happenstance-testkit/src/projection.rs`, described `CheckpointOnlyStore` as
a carried debt arriving with a later story. That stopped being true one commit ago;
all three now say the rejection is demonstrated. Those files were outside the
previous story's PR boundary and inside this one's, which is why the correction
lands here.

## Acceptance

| AC | Status | Verified by |
| -- | ------ | ----------- |
| AC-001 | **BLOCKED — not satisfied** | no test, no rule, nothing stubbed. Missing dependency: a commit-fault `Capability` on `ProjectionFixture` (`_design.md:276-280` fixes the set at three constants, none of which arms a fault) |
| AC-002 | satisfied | `projection_conformance{,_blocking}::rollback_leaves_both_unchanged` + its cell in `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` |
| AC-003 | satisfied | `…::dropped_batch_leaves_store_usable` + its cell |
| AC-004 | satisfied | `…::commit_rejects_a_foreign_batch` + its cell |
| AC-005 | satisfied | `…::commit_accepts_a_position_the_batch_did_not_write` + its cell |
| AC-006 | satisfied | `…::commit_rejects_a_regressing_position` + its cell |
| AC-007 | satisfied | `…::distinct_projections_advance_independently` + its cell |
| AC-008 | satisfied for the six landed rules | the four projection exactness meta-tests, green over 8 rules × 8 stores |
| AC-009 | satisfied | `no_orphan_projection_rules`; both host emitters; the explicit `wasm32 --tests` check |
| AC-010 | satisfied | `changelog_names_every_rule`; `cargo xtask spec-trace`; diff review against the density counts |

Merge gate: `cargo xtask affected --base main` **green**, `cargo xtask ci --fast`
**green** (CF-6, CF-29, CF-33 included), `cargo xtask spec-trace` **exit 0**,
`cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown`
**clean**, `cargo fmt --check` **clean**.

**The story gate will not pass with AC-001 at `satisfied: false`, and that is
correct.** The row is not flipped, no evidence is invented for it, and the
decision it waits on is named above.

## Knowledge Harvest

- **A conformance rule can be blocked by a capability set rather than by code.**
  This is the first time in the project that a rule could not be written because
  the *fixture contract* had no honest gate for it — not because the port was
  wrong, not because the clause was unclear. The transferable observation is that
  a fixture's capability set is a **rule budget**: it bounds which obligations the
  suite can ever enforce, and freezing it before the rule set is complete leaves
  clauses stranded. Candidate: a concept atom beside `cf-40-fixture-limits-ownership`.
- **A rule with no capability gate is an assertion, if the declining instrument
  runs it.** `commit_rejects_a_foreign_batch` passes against a fixture that
  declines everything, and that pass is only meaningful because the rule is
  *absent* from `PROJECTION_MUST_REJECT`. Absence-as-assertion is fragile and
  wants its reason written at the list rather than at the rule; it now is.
  Candidate: a playbook line.
- **Adding a rule re-opens every existing mutant's exactness claim, and the cost
  is quadratic and worth paying.** Six new rules re-interrogated two existing
  stores and grew both declarations. The registry caught it immediately and named
  the cell; the temptation it creates — weaken the new rule so the old declaration
  survives — is the repair to refuse, and both grown rows now carry a comment
  saying so. Candidate: an amendment to ADR-0010's practice notes.
