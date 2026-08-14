---
item: "HS-S0010"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — Commit, rollback and dropped-batch rules, each with the store that fails it

## Findings Ledger

**Nine of ten ACs satisfied. AC-001 is BLOCKED, and it is blocked on a decision
this story is not entitled to make.** Six conformance rules landed, each with a
registered wrong implementation that fails **exactly** it, each green against the
oracle on both host emitters and type-checked on `wasm32`. The seventh rule,
`failed_commit_leaves_both_unchanged`, is not in the tree.

**The delta this story is.** Before it the projection suite proved one thing: a
store that drops the read-model write is caught. After it the whole commit path is
differential — an explicit rollback, a batch dropped bare, a batch begun on the
wrong store, a position the batch never wrote, a position that goes backwards, and
two projections that must not share a checkpoint row. Two rules became eight; two
hostile stores became eight.

**The claim this story does not make.** The projection port is not frozen. Nine of
§4.11's seventeen rules are still owed (the reset and read-through families),
CF-5's conformant variant is still deferred, and — the new one — **PS-1's second
conjunct is unenforced**, because the rule that would enforce it cannot be gated
honestly against the capability set DT-3 signed off. That is stated here rather
than left for the freeze verdict to discover.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — BLOCKED. `failed_commit_leaves_both_unchanged` is not landed.** The rule must be gated on a fixture capability that can arm a commit fault, in `MID_BATCH_FAULT`'s mould; the projection fixture has none, and that is a signed-off decision rather than an omission. | DT-3's resolution enumerates the projection capability set as exactly `RESET_REFUSAL`, `SECOND_HANDLE` and `READS_THROUGH_BATCH` (`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:276-280`), and `crates/happenstance-testkit/src/contract.rs:519-596` implements exactly that set. The same table lists this rule as gated by `SECOND_HANDLE`, which cannot arm anything. | **Owed: a decision.** Either DT-3 is amended to add a commit-fault capability (`projection-api-design-record`'s ground), or PS-1's second conjunct is recorded as unenforceable by the suite and its maturity marker reconsidered by `unstable-projection-gate-and-clause-disposition`. **Not** to be closed by minting a capability inside a rules story — that is the second declension policy `.kb/open-questions/cf-40-fixture-limits-ownership.md` and `project.md`'s risk register both forbid. |
| **AC-002 — an explicit rollback preserves both halves.** | `crates/happenstance-testkit/src/projection.rs:394-448`; green on both host emitters; red against `UnrolledBackStore` at `"must leave the read model as it was"`. | None. The checkpoint is compared against what a fresh handle saw *before* the rollback, so the rule asserts preservation and claims nothing about progress. |
| **AC-003 — a dropped batch rolls back AND leaves the store usable.** | `projection.rs:454-505`, non-vacuity anchor at `:480-491`; red against `PooledConnectionStore` at `"commit should succeed"`. The connection is genuinely modelled (`correct.rs:305-336`, `:364-380`), not asserted about. | None. Project AC-010 discharged. |
| **AC-004 — a foreign batch is refused and neither store moves.** | `projection.rs:522-577`, two `open()` calls and deliberately no `SECOND_HANDLE` gate; red against `TypeStampedBatchStore`. | **Worth a reviewer's eye:** this is the one projection rule absent from `PROJECTION_MUST_REJECT`, which makes its *pass* against a fixture declining everything an assertion rather than an accident. Adding it there "for safety" would make that assertion unreachable. |
| **AC-005 — a commit may name a position the batch did not write.** | `projection.rs:583-624`; red against `ValidatingCommitStore`, the store the specification itself names for this rule. | None. |
| **AC-006 — a regressing position is refused with both fields, and nothing moves.** | `projection.rs:630-692`; red against `UnconditionalCheckpointStore`. Positions from `SequencePosition::FIRST` and `after` (`:214-227`); CF-6 lint green over 4 rule files. | **Review check discharged:** no assertion anywhere about an *equal* position — PS-22 permits accepting one. |
| **AC-007 — projections advance independently.** | `projection.rs:698-756`; red against `SingleRowCheckpointStore`. | None. |
| **AC-008 — six of seven rules have a mutant that fails exactly them; no pass rate anywhere.** | The four projection exactness meta-tests green over the widened 8×8 matrix, plus the event-store family's four beside them. RED first: CF-1 named all six new rules as decorative before any mutant landed. | **EC-003 obtained and was honoured.** `CheckpointOnlyStore`'s declaration grew to three rules and `UncommittedTransactionStore`'s to five, **in this PR**, each with a comment naming the refused repair. The five-rule row is inflation rather than vacuity, and it meets the event-store registry's stated bar: no rule is covered by that store alone except `commit_advances_the_checkpoint`, which is the one it exists for. |
| **AC-009 — every rule present on every runtime, reported at its own name.** | Single enumeration + `no_orphan_projection_rules`; `8 passed` on each host emitter; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` run explicitly and clean. | **Stated cost, restated so the gate is not over-read:** the wasm step covers the *rules* on that target and none of the eight mutants — the mutant binary is gated off `wasm32` because `catch_unwind` cannot catch without an unwinder. |
| **AC-010 — one skip vocabulary, one enumeration, one registry, and a human-readable record.** | `RuleOutcome` reused unchanged (no diff to `contract.rs`); **+6** rule names (2 → 8, not 9 — AC-001 is blocked), **+6** registry rows (2 → **8 stores**, the planned end state), **+0** skip vocabularies, **+0** harness files, **+0** gate steps, **+0** dependencies, **+0** public items. `CF-29: all 103 rules in 4 file(s) have a changelog entry`. `spec-trace` green after `--write`: 6 insertions, 6 deletions, all inside the generated region. | **EC-005 checked, not assumed:** `RULE_FILES` is `[&str; 4]` and already contained the projection rules module, so no edit was needed. The gate counting **4 rule files and 103 rules** is what proves the four sweeps reach it rather than printing green over nothing. |

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
