---
item: "HS-S0011"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

> **STATUS: ten of ten ACs satisfied.** Five rules landed in the projection rule
> family, mounted in the one enumeration, emitted by all three harnesses, and
> **every one of them has a registered wrong store that fails it by name**. Six
> new hostile stores, each a single overridden step over the shared correct core;
> three existing declarations grew rather than the new rules being weakened to
> preserve them.
>
> **What the projection suite is after this commit.** Fourteen rules, not nine.
> `reset` is now a checked operation rather than a signature: one unit of work
> covering rows and checkpoint together, scoped to one `(store, ProjectionId)`,
> refusable with no side effect, `NeverRun` for an id nobody has ever committed,
> and — the one this story exists for — **not** `commit(empty, id, FIRST)`. The
> substitute six deployment scenarios out of six reached for now fails a named
> test with a resume-point consequence attached.

## TDD Evidence

The order was the repository's own, and it is the order the spec asks for: the
rules first, so the first red is the instrument *demanding* the stores rather
than a promise that it would. Two reds, each for the behaviour it names and
neither a compile or import failure.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `projection_conformance::reset_clears_rows_and_checkpoint_together` + its cell in `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` | **red**: `every_projection_rule_has_a_mutant` named it among five rules that *"have no mutant … so they certify nothing"*. **green** with `TwoStatementResetStore`, pinned at `"must return this projection's checkpoint to"`. |
| AC-002 | `projection_conformance::reset_is_scoped_to_one_projection` + its cell | **red** in the same run; **green** with `TruncatingResetStore`, pinned at `"resetting one projection moved another one's checkpoint"`. A **second** red on the way there was the informative one — see `## Notes`, EC-004's near miss. |
| AC-003 | `projection_conformance::refused_reset_changes_nothing` + its two cells | **red** in the same run; **green** with `RefusalAsSuccessStore` (pinned at ``"must answer `Err(ResetError::Refused)`"``) and `RefusalAfterTheFactStore` (pinned at `"must leave the read model exactly as it was"`) — the pair AC-003 names, one per half. |
| AC-004 | `projection::a_declined_reset_refusal_is_reported_with_the_fixtures_reason`; `mutation_coverage::projection_capability_skips_are_reported` | **red**: the reference fixture's pinned skip set was `["failed_commit_leaves_both_unchanged"]` and the new rule made it two, which is exactly the equality that list exists to force. **green** when both the unit assertion and the pinned set named `refused_reset_changes_nothing` and its reason was read from the fixture's own `const`. |
| AC-005 | `projection_conformance::fresh_projection_has_no_checkpoint` + its cell | **red** in the same run; **green** with `PresumedLiveCheckpointStore`, pinned at `"a projection this store has never seen must read back as"`. The EC-001 precondition was checked *before* the rule was written: ADR-0018 is `status: accepted`. |
| AC-006 | `projection_conformance::reset_is_not_commit_at_first` + its cell | **red** in the same run; **green** with `CommitAtFirstResetStore`, pinned at `"must be **distinguishable by variant**"`. Its second half — the resume consequence — is asserted through `resumes_over`, and it is what separates this rule from a variant comparison that would not have seen event 1 disappear. |
| AC-007 | the four projection exactness meta-tests | **red twice, for two different right reasons.** First: CF-1 listed all five new rules as decorative. Second, three times over — `projection_mutants_fail_exactly_their_declared_rules` reported `UncommittedTransactionStore`, `TypeStampedBatchStore` and `ValidatingCommitStore` each *"failed a rule it does not declare"*, which is **EC-005 arriving where the spec predicted**. **green** when each declaration grew with a comment saying which repair was refused. |
| AC-008 | the same four meta-tests over the widened rule set | **green** once every one of the five rules had at least one registered store failing it, each store a single overridden `Defect` step. No pass rate is computed or quoted anywhere in the binary or in this report. |
| AC-009 | `no_orphan_projection_rules`; the two host emitters; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | **green**, and the wasm arm was run explicitly because `--fast` omits it. Each host emitter reports `14 passed`. |
| AC-010 | `changelog_names_every_rule`; `cargo xtask spec-trace`; the CF-6 position lint | **red then green on all three.** The changelog lint fails on a rule with no entry of substance; `spec-trace` went red listing five generated rows still marked `†` *not found*, and green after `spec-trace --write` regenerated the generated region — clause text, maturity markers and rule citations untouched. |

The oracle direction was green from the first compile and is deliberately not a
red-then-green row: fourteen rules against `MemoryProjectionStore` on the tokio
and blocking emitters. A conformance rule that failed the reference store on its
first run would be a rule written against a store rather than against a clause.
The direction that had to be earned is the mutant one, and every row above
carries it.

## Commits

`feat(projection-store-freeze): Reset is one unit of work` — one checkpoint
commit, the first of slice `reset-and-rebuild-rules`, on
`initiative/from-contract-to-published-library`, immediately after
`chore(projection-store-freeze): seal slice commit-atomicity-and-mutants
approved`.

Named by subject and by predecessor rather than by hash, for the reason the
earlier reports in this project give: this file is committed *inside* the commit
it describes, so no hash it quoted could survive being written into it.
`git log --grep "Story: projection-store-freeze/reset-rules"` resolves it.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-testkit/src/projection.rs` | **Five rule bodies** (`:1052-1531`) and their five names added to the single `for_each_projection_store_rule!` enumeration (`:1563-1568`) — the composition root for a conformance rule, since a rule not named there is compiled by nothing. Two helpers: `reset_ok`, and `resumes_over` (`:302`), which states PS-20's derivation once for both arms of `reset_is_not_commit_at_first` and is documented as *not* a runner. One unit test, `a_declined_reset_refusal_is_reported_with_the_fixtures_reason` (`:1753`). Module doc corrected twice: the family is now two rules answered by a skip against the oracle rather than one, and the "eight rules unwritten" sentence is three. |
| `crates/happenstance-testkit/src/contract.rs` | One new **provided method** on `ProjectionFixture`: `protect_from_reset(&self, id: &ProjectionId)` (`:705`), whose provided body panics. It is `arm_commit_fault`'s sibling and the mechanism `RESET_REFUSAL` gates — **not a fourth capability**, and the doc says so in those words. `RESET_REFUSAL`'s own doc lost the sentence saying no rule read it yet. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs` | The `Defect` seam grew from eight steps to eleven — `missing_checkpoint` (PS-19), `refusal` (PS-18) and `reset_writes` (PS-16) — each added because *a rule acquired the ability to see a defect there*. `MutantStore` grew one field, `protected: Rc<RefCell<Option<String>>>`, shared by every handle exactly as the fault flag is and deliberately **not** one-shot. `MutantFixture::RESET_REFUSAL` went from declined to `SUPPORTED`, which is the honest answer now that the instrument has a policy to enforce. The correct `reset` **records** an explicit `NeverRun` rather than removing the key, and the step's doc says why: with a removal, a store whose *missing* row resolves to `Live` would fail four rules instead of the one its defect is about. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs` | **Six new hostile stores**, each an `impl Defect` overriding exactly one step: `TwoStatementResetStore`, `TruncatingResetStore`, `CommitAtFirstResetStore`, `RefusalAsSuccessStore`, `RefusalAfterTheFactStore`, `PresumedLiveCheckpointStore`. Each was named defect-first, from §4.11's *Rejects* column and from `RUNBOOK.md:3904-3907`, before it was named at all. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | Six `Declared` rows (`:544-700`), and **three existing rows grown** (`UncommittedTransactionStore`, `TypeStampedBatchStore`, `ValidatingCommitStore`) with a comment on each naming the repair that was refused. Six entries added to `for_each_projection_mutant!`. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `PROJECTION_MUST_REJECT` grew from eight names to twelve; the reference fixture's pinned skip set grew from one rule to two, checked per skip against the fixture's own `const`. The tail of `projection_capability_skips_are_reported` moved into a free function, `assert_reference_projection_declensions`, because the body crossed `clippy::too_many_lines` — the same trade the file already made once. `PROJECTION_MUST_SKIP`'s doc records that its own prediction was wrong and why. |
| `CHANGELOG.md` | One entry per landed rule naming the defect it detects, plus one for the six wrong stores and the three grown declarations. |
| `spec/SPECIFICATION.md` | **The generated region only.** `cargo xtask spec-trace --write`; 5 insertions, 5 deletions, every one of them a `†` removed from PS-16, PS-17, PS-18, PS-19 or PS-20. |
| `standards/rust/{11,13,40,41}-*.md` | Six `file:line` citations into `contract.rs` repaired, because the new provided method shifted the file by 49 lines. Compelled repair, admitted into this boundary the way `aef8990` admitted it into the two before it: the citations name anchors, `cargo xtask lint-constitution` checks them, and leaving them wrong would break the gate for a reason unrelated to this story. |
| `.bklg/…/reset-rules/_ledger.md` | Ten rows flipped `false` → `true` with citations. |

No `Cargo.toml` was edited, no dependency added, no feature flag introduced
(NF-001), and no clause text or maturity marker moved.

## Gates

| Gate | Result |
| ---- | ------ |
| `cargo test -p happenstance-testkit` | green — 14 projection rules on the tokio harness, 14 on the blocking harness, four projection exactness meta-tests, nine event-store meta-tests including `projection_capability_skips_are_reported`. |
| `cargo test -p happenstance-testkit --test projection_mutation_coverage` | green — `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive`, `projection_mutants_fail_exactly_their_declared_rules`, `every_projection_mutant_states_its_provenance`. |
| `cargo xtask ci --fast` | green (fmt, clippy `-D warnings` over all targets and all features, the workspace tests, the proof-artefact checks, docs, `spec-trace`, the retired-rule / clock / literal-position / changelog lints, the testkit version check and `lint-constitution`). |
| `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | green — `projection_conformance_wasm.rs` type-checks the five new rules with no per-harness list and no `wasm32`-only subset. |
| `cargo xtask spec-trace` | green after `--write`; `git diff -- spec/` is the generated region and nothing else. |
| `redkiln validate --kb` | green — the EC-001 precondition, recorded on this tree. |

## Notes

**1. PS-16's failure-injecting half needed no new capability, and did not get
one.** The clause pairs its rule with *"a failure-injecting variant asserting
that a `reset` that errors leaves both halves as they were"*, and the projection
fixture's capability set — `SECOND_HANDLE`, `RESET_REFUSAL`, `COMMIT_FAULT`,
plus `ProjectionProbe::READS_THROUGH_BATCH` — contains nothing that can arm a
failing **reset**. `commit-rollback-and-drop-rules` met the same wall on the
commit side and halted rather than minting a constant inside a rules story;
minting one here would have been the same violation with the precedent already
set against it. The rule reaches the same observation through the port's own
error path instead: a batch begun on a **different store instance** is refused as
`ResetError::ForeignBatch`, which is a `reset` that errored, is reachable by any
caller, and needs no fixture co-operation at all. The cost is one line in an
existing registry row (`TypeStampedBatchStore` now also fails this rule, because
it accepts every instance's batch) and it is recorded there rather than absorbed.

**2. Three existing declarations grew, and each says which repair was refused.**
`UncommittedTransactionStore` fails `reset_is_not_commit_at_first` because a
store that makes nothing durable leaves *both* checkpoints at `NeverRun`, so the
two states this rule requires to be distinguishable are the same state.
`TypeStampedBatchStore` fails `reset_clears_rows_and_checkpoint_together` for the
reason in note 1. `ValidatingCommitStore` fails `reset_is_not_commit_at_first`
because the operator's substitute *is* `commit(empty_batch, id, FIRST)`, and a
store that refuses a commit whose batch applied nothing refuses the substitute
itself — the alternative was to model the substitute with a non-empty batch,
which would have kept the declaration at one rule and stopped modelling what six
deployment scenarios actually typed. In all three cases the repair that was
available and refused is weakening the new rule so the old declaration survived
(EC-005's second sentence).

**3. `reset_is_scoped_to_one_projection` hands `reset` an *empty* batch, and
that is the sharp version rather than a shortcut.** `probe_delete_all` queues
removal of **every** probe row — the port has no idea which rows belong to which
projection, and PS-11's seam deliberately does not teach it — so a scoping rule
that reset one id with a `probe_delete_all` batch would delete the sibling's rows
*by the caller's own instruction* and would fail a conformant store for something
PS-17 does not constrain. With an empty batch the caller asked for no deletes at
all, so every row that disappears and every checkpoint other than the target's
that moves is attributable to the store. The rule's own doc carries this
paragraph, because the first instinct on reading it is to "fix" the missing
`probe_delete_all`.

**4. The correct core's `reset` now records `NeverRun` instead of removing the
key, and that is a deliberate divergence from the oracle.**
`MemoryProjectionStore` removes the key and documents why; both spellings are
conformant. Recording it keeps *what an unseen id reads* and *what a reset leaves
behind* as two seams rather than one, which is what lets
`PresumedLiveCheckpointStore` be a single override that fails the rule its defect
is about instead of four rules its defect merely disturbs. The divergence is
stated on the step.

**5. The two reset-side mutants that model different mistakes fail the same
rules, and the rows say so.** `TwoStatementResetStore` and
`CommitAtFirstResetStore` both leave a `Live` checkpoint behind, so both are seen
by the three rules that look at a checkpoint after a reset. What separates them
is not which rules they fail but what they model — the statement that never ran,
and the statement someone wrote on purpose — and CF-4's provenance is where that
distinction lives. Collapsing them into one store would have discarded one of
the two provenances, which is the part of a mutant a reviewer actually reads.

**6. `PROJECTION_MUST_SKIP` is still empty, and its own doc's prediction was
wrong.** That list was written expecting `RESET_REFUSAL`'s rule to be its first
entry. The rule that arrived spells `must!(F: SECOND_HANDLE)` **first** — a
fixture declining both is failing the contract, and reporting that as a skip
would file a broken fixture under a trade it was entitled to make — so against
the declining instrument it rejects rather than skips, and it went into
`PROJECTION_MUST_REJECT`. The doc now records the wrong prediction and names the
next candidate rather than being quietly rewritten as though it had been right.
