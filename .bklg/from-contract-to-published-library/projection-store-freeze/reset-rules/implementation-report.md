---
item: "HS-S0011"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

> **AMENDED TWICE. Read the amendments in order — they are the record of a rule
> that shipped, was withdrawn, and came back against a different clause.**
>
> **First amendment, 2026-08-14 (the fix pass).** AC-005 was blocked, not
> satisfied: EC-001 fired, the PS-19 repair atom its precondition names did not
> exist, and `fresh_projection_has_no_checkpoint` and
> `PresumedLiveCheckpointStore` were **held** rather than shipped. Reasoning in
> `_slices.md`, *reset-and-rebuild-rules — fix pass 2026-08-14*.
>
> **Second amendment, 2026-08-15 (run 6). AC-005 is satisfied and the hold is
> over.** `.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md`
> is `status: accepted` on this branch, merged at `9efda45`. It did **not** widen
> `[FROZEN]` PS-19 — byte-identical across the decision — it minted **PS-38**,
> whose second sentence is the obligation the rule needed: *"a `ProjectionId` no
> successful `commit` has named MUST read as `Checkpoint::NeverRun`"*. The rule is
> back in the module and the enumeration, cited to PS-38; the mutant is back after
> re-checking that PS-38's text genuinely rejects it; and a **conformant variant**
> on the same seam (`AbsentAfterResetStore`) landed with them, because the absence
> of one was why CF-5's positive control could not see the original over-conviction.
> The corrected rows are in `_ledger.md` and `report.md`; `_slices.md`, *Run 6's
> record*, carries what was routed onward.
>
> **STATUS AS WRITTEN AT THE IMPLEMENT COMMIT: ten of ten ACs satisfied.** Five
> rules landed in the projection rule family, mounted in the one enumeration,
> emitted by all three harnesses, and **every one of them has a registered wrong
> store that fails it by name**. Six new hostile stores, each a single overridden
> step over the shared correct core; three existing declarations grew rather than
> the new rules being weakened to preserve them. That status was premature on
> 2026-08-14 and is true on 2026-08-15 — for a reason nothing in this story could
> supply, which is the part worth remembering.
>
> **What the projection suite is after this commit.** Fourteen rules, not nine —
> thirteen while the fix pass held one, and seventeen once the restoration and the
> slice-mate's three are counted. `reset` is now a checked operation rather than a
> signature: one unit of work covering rows and checkpoint together, scoped to one
> `(store, ProjectionId)`, refusable with no side effect, and — the one this story
> exists for — **not** `commit(empty, id, FIRST)`. The substitute six deployment
> scenarios out of six reached for now fails a named test with a resume-point
> consequence attached. *"`NeverRun` for an id nobody has ever committed"* is in
> that list too, now that PS-38 says so.

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
| AC-005 | `projection_conformance::fresh_projection_has_no_checkpoint` + its cell in `projection_mutants_fail_exactly_their_declared_rules` | **Withdrawn 2026-08-14, restored 2026-08-15, and the sequence is the evidence.** *Withdrawn:* EC-001 fired and this row's original claim was false — the precondition is an accepted atom widening PS-19, none existed, and the row had cited ADR-0018, a different atom that repairs nothing here (`.kb/decisions/0018-…:113-116`). The rule and `PresumedLiveCheckpointStore` were held. *Restored:* ADR-0030 is accepted on this branch (`9efda45`) and mints **PS-38**, whose second sentence states the obligation. **red**: with the rule back in the enumeration and no mutant, `every_projection_rule_has_a_mutant` named it as certifying nothing; with `PresumedLiveCheckpointStore` back, `projection_mutants_fail_exactly_their_declared_rules` is what proves the rule *rejects* it — pinned at `"a projection this store has never seen must read back as"`. **green** on both, and on `projection_conformant_variants_pass_everything` over the new `AbsentAfterResetStore`, which is the assertion that the rule does **not** over-convict the legal store on the other arm of the same seam. |
| AC-006 | `projection_conformance::reset_is_not_commit_at_first` + its cell | **red** in the same run; **green** with `CommitAtFirstResetStore`, pinned at `"must be **distinguishable by variant**"`. Its second half — the resume consequence — is asserted through `resumes_over`, and it is what separates this rule from a variant comparison that would not have seen event 1 disappear. |
| AC-007 | the four projection exactness meta-tests | **red twice, for two different right reasons.** First: CF-1 listed all five new rules as decorative. Second, three times over — `projection_mutants_fail_exactly_their_declared_rules` reported `UncommittedTransactionStore`, `TypeStampedBatchStore` and `ValidatingCommitStore` each *"failed a rule it does not declare"*, which is **EC-005 arriving where the spec predicted**. **green** when each declaration grew with a comment saying which repair was refused. |
| AC-008 | the same four meta-tests over the widened rule set | **green** once every one of the five rules had at least one registered store failing it, each store a single overridden `Defect` step. No pass rate is computed or quoted anywhere in the binary or in this report. |
| AC-009 | `no_orphan_projection_rules`; the two host emitters; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | **green**, and the wasm arm was run explicitly because `--fast` omits it. Each host emitter reports `17 passed` as of the 2026-08-15 restoration (`14` while AC-005's rule was held). |
| AC-010 | `changelog_names_every_rule`; `cargo xtask spec-trace`; the CF-6 position lint | **red then green on all three.** The changelog lint fails on a rule with no entry of substance; `spec-trace` went red listing generated rows still marked `†` *not found*, and green after `spec-trace --write` regenerated the generated region — clause text, maturity markers and rule citations untouched, across the withdrawal (which restored a `†`) and the restoration (which cleared it from PS-19's row and PS-38's). |

The oracle direction was green from the first compile and is deliberately not a
red-then-green row: every rule against `MemoryProjectionStore` on the tokio and
blocking emitters, seventeen of them now. A conformance rule that failed the
reference store on its first run would be a rule written against a store rather
than against a clause.
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
| `crates/happenstance-testkit/src/projection.rs` | **Five rule bodies** (`:1122-1633`) and their five names added to the single `for_each_projection_store_rule!` enumeration (`:1907-1911`) — the composition root for a conformance rule, since a rule not named there is compiled by nothing. Two helpers: `reset_ok`, and `resumes_over` (`:302`), which states PS-20's derivation once for both arms of `reset_is_not_commit_at_first` and is documented as *not* a runner. One unit test, `a_declined_reset_refusal_is_reported_with_the_fixtures_reason` (`:2100`), whose doc points at `assert_reference_projection_declensions` as the authority for the reference skip set rather than restating a count. Module doc corrected: the family is now two rules answered by a skip against the oracle rather than one, and — at the 2026-08-15 restoration — every rule §4.11 assigns to an adapter's own suite is written, with the withdrawal-and-return recorded in the paragraph rather than erased. |
| `crates/happenstance-testkit/src/contract.rs` | One new **provided method** on `ProjectionFixture`: `protect_from_reset(&self, id: &ProjectionId)` (`:705`), whose provided body panics. It is `arm_commit_fault`'s sibling and the mechanism `RESET_REFUSAL` gates — **not a fourth capability**, and the doc says so in those words. `RESET_REFUSAL`'s own doc lost the sentence saying no rule read it yet. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs` | The `Defect` seam grew from eight steps to eleven — `missing_checkpoint` (PS-19), `refusal` (PS-18) and `reset_writes` (PS-16) — each added because *a rule acquired the ability to see a defect there*. `MutantStore` grew one field, `protected: Rc<RefCell<Option<String>>>`, shared by every handle exactly as the fault flag is and deliberately **not** one-shot. `MutantFixture::RESET_REFUSAL` went from declined to `SUPPORTED`, which is the honest answer now that the instrument has a policy to enforce. The correct `reset` **records** an explicit `NeverRun` rather than removing the key, and the step's doc says why: with a removal, a store whose *missing* row resolves to `Live` would fail four rules instead of the one its defect is about. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs` | **Five new hostile stores** after the fix pass, each an `impl Defect` overriding exactly one step: `TwoStatementResetStore`, `TruncatingResetStore`, `CommitAtFirstResetStore`, `RefusalAsSuccessStore`, `RefusalAfterTheFactStore`. Each was named defect-first, from §4.11's *Rejects* column and from `RUNBOOK.md:3904-3907`, before it was named at all. A **sixth**, `PresumedLiveCheckpointStore` (`:465`, one override of `missing_checkpoint`), was held on 2026-08-14 and **restored on 2026-08-15**: it is conformant with PS-19 alone, and PS-38 — minted by ADR-0030, not a widening of PS-19 — is what makes it a mutant. Its own doc carries that sequence, because a registry row that hides why it was withdrawn teaches nothing. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | Six `Declared` rows (`:554-748`) plus one `Kind::ConformantVariant` row for `AbsentAfterResetStore` (`:834`), and **three existing rows grown** (`UncommittedTransactionStore`, `TypeStampedBatchStore`, `ValidatingCommitStore`) with a comment on each naming the repair that was refused. Seven entries added to `for_each_projection_mutant!`. The module doc's *"CF-5's half"* section grew from two conformant rows to three and records why the third was bought at full price. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/variants.rs` | **`AbsentAfterResetStore`** (`:127`), CF-5's third conformant projection row and the one this whole episode is the argument for: a `reset` that *removes* its checkpoint row instead of overwriting it with an explicit `NeverRun`. Both answers are legal — it is `MemoryProjectionStore`'s own shape, and the correct core in this binary deliberately does the other one — so it is a variant and not a mutant. It occupies the arm of the missing-checkpoint seam the registry had **no** legal store on, which is exactly why `projection_conformant_variants_pass_everything` could not catch the rule that over-convicted `PresumedLiveCheckpointStore`. |
| `crates/happenstance-testkit/src/fixtures.rs`, `…/tests/projection_conformance.rs`, `…/src/lib.rs`, `crates/happenstance-core/src/projection.rs`, `_design.md` | **Doc repairs, 2026-08-15.** Five sites said a reference run prints *one* `SKIP` line; `assert_reference_projection_declensions` pins **two**, and this story is what made it two. Each now names `COMMIT_FAULT`'s rule and `RESET_REFUSAL`'s and **points at that assertion as the authority** rather than restating a count nothing in the gate reads — the count had been wrong four times. Three further sites said the seventeenth rule was held pending a PS-19 widening; they now say what happened instead. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `PROJECTION_MUST_REJECT` grew from eight names to twelve; the reference fixture's pinned skip set grew from one rule to two, checked per skip against the fixture's own `const`. The tail of `projection_capability_skips_are_reported` moved into a free function, `assert_reference_projection_declensions`, because the body crossed `clippy::too_many_lines` — the same trade the file already made once. `PROJECTION_MUST_SKIP`'s doc records that its own prediction was wrong and why. |
| `CHANGELOG.md` | One entry per landed rule naming the defect it detects, plus one for the six wrong stores and the three grown declarations. The fix pass's *non-delivery* entry was **replaced** on 2026-08-15 by `fresh_projection_has_no_checkpoint`'s own entry: a shipped changelog telling an adapter author the contract does not oblige something it now obliges is worse than silence. |
| `spec/SPECIFICATION.md` | **The generated region only.** `cargo xtask spec-trace --write`; every hunk a `†` removed from PS-16 – PS-20 as its rule started existing — one restored on PS-19's row by the fix pass, and cleared again on 2026-08-15 from PS-19's row and PS-38's. No clause text, maturity marker or rule citation moved; PS-19 is byte-identical across the whole episode. |
| `standards/rust/{11,13,40,41}-*.md` | Six `file:line` citations into `contract.rs` repaired, because the new provided method shifted the file by 49 lines. Compelled repair, admitted into this boundary the way `aef8990` admitted it into the two before it: the citations name anchors, `cargo xtask lint-constitution` checks them, and leaving them wrong would break the gate for a reason unrelated to this story. |
| `.bklg/…/reset-rules/_ledger.md` | Ten rows flipped `false` → `true` with citations. AC-010's criterion text was **re-synced** on 2026-08-15 to `spec.md:382`'s amended wording, which `5f2cf02` had changed in the spec and not in the ledger — the row the gate reads and the criterion the spec states were two different sentences. |

No `Cargo.toml` was edited, no dependency added, no feature flag introduced
(NF-001), and no clause text or maturity marker moved.

## Gates

| Gate | Result |
| ---- | ------ |
| `cargo test -p happenstance-testkit` | green — 17 projection rules on the tokio harness, 17 on the blocking harness, 18 on the buffering harness, seven projection meta-tests, ten event-store meta-tests including `projection_capability_skips_are_reported`. |
| `cargo test -p happenstance-testkit --test projection_mutation_coverage` | green — `every_projection_rule_has_a_mutant`, `projection_mutant_registry_is_exhaustive`, `projection_mutants_fail_exactly_their_declared_rules`, `every_projection_mutant_states_its_provenance`. |
| `cargo xtask ci --fast` | green (fmt, clippy `-D warnings` over all targets and all features, the workspace tests, the proof-artefact checks, docs, `spec-trace`, the retired-rule / clock / literal-position / changelog lints, the testkit version check and `lint-constitution`). |
| `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | green — `projection_conformance_wasm.rs` type-checks the five new rules with no per-harness list and no `wasm32`-only subset. |
| `cargo xtask spec-trace` | green after `--write`; `git diff -- spec/` is the generated region and nothing else. |
| `redkiln validate --kb` | green — but **green on this command was never the EC-001 precondition**, and reading it as one is the defect the fix pass corrected. EC-001 asks whether a *specific atom* exists and is accepted; `validate --kb` asks whether the atoms that do exist are well-formed. It answers yes to the second question over a tree missing the first. |

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
