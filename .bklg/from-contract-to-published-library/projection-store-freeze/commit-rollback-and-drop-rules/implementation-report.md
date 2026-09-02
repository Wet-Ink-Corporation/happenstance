---
item: "HS-S0010"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — Commit, rollback and dropped-batch rules, each with the store that fails it

> **STATUS: nine of ten ACs satisfied. AC-001 is BLOCKED and reported, not
> worked around.** Six of the seven planned rules landed, each with a registered
> wrong implementation that fails **exactly** it. The seventh —
> `failed_commit_leaves_both_unchanged` — is not in the tree, because the
> capability it has to be gated on does not exist and minting one here is the
> failure EC-002 names by name. The detail, with citations, is the first entry
> under `## Notes`.
>
> **What the projection suite is after this commit.** Eight rules, not two. The
> whole commit path is differential: a batch rolled back, a batch dropped on the
> floor, a batch begun on the wrong store, a position the batch never wrote, a
> position that goes backwards, and two projections that must not share a
> checkpoint row. Eight hostile stores, each failing exactly what it declares.
> Project DoD 1 — *`CheckpointOnlyStore` fails by name* — is observable at the
> end of this slice.

## TDD Evidence

Two reds, each for the behaviour it names and neither a compile or import
failure. The order was the one the spec asks for: the rules first, so that the
first red is the instrument *demanding* the stores rather than a promise that it
would.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | — | **BLOCKED.** No test was written, no rule was landed, and nothing was stubbed to make the row flippable. See `## Notes`. |
| AC-002 | `projection_conformance::rollback_leaves_both_unchanged` + its cell in `projection_mutation_coverage::projection_mutants_fail_exactly_their_declared_rules` | **red**: `every_projection_rule_has_a_mutant` named it among six rules that "have no mutant … so they certify nothing". **green** once `UnrolledBackStore` landed, failing at the pinned substring `"must leave the read model as it was"`. |
| AC-003 | `projection_conformance::dropped_batch_leaves_store_usable` + its cell in the exactness meta-test | **red** in the same run; **green** with `PooledConnectionStore`, pinned at `"commit should succeed"`. The second red this rule produced was the *interesting* one — see EC-003 below. |
| AC-004 | `projection_conformance::commit_rejects_a_foreign_batch` + its cell | **red** in the same run; **green** with `TypeStampedBatchStore`, pinned at ``"must be rejected as `CommitError::ForeignBatch`"``. |
| AC-005 | `projection_conformance::commit_accepts_a_position_the_batch_did_not_write` + its cell | **red** in the same run; **green** with `ValidatingCommitStore` — the store the specification itself names for this rule — pinned at `"commit should succeed"`. |
| AC-006 | `projection_conformance::commit_rejects_a_regressing_position` + its cell | **red** in the same run; **green** with `UnconditionalCheckpointStore`, pinned at ``"must be refused as `CommitError::CheckpointRegression`"``. |
| AC-007 | `projection_conformance::distinct_projections_advance_independently` + its cell | **red** in the same run; **green** with `SingleRowCheckpointStore`, pinned at `"one commit advances exactly one projection"`. |
| AC-008 | the four projection exactness meta-tests | **red twice, for two different right reasons.** First: CF-1 listed all six new rules as decorative. Second, and the one worth recording — `projection_mutants_fail_exactly_their_declared_rules` reported ``\`CheckpointOnlyStore\` failed \`dropped_batch_leaves_store_usable\`, which it does not declare``, which is **EC-003 arriving exactly where the spec predicted**. **green** when both existing declarations grew (never by weakening the new rules). |
| AC-009 | `no_orphan_projection_rules`; the two host emitters; `cargo check --tests --target wasm32-unknown-unknown` | **green**, and the wasm arm was run explicitly because `--fast` omits it. Each emitter reports `8 passed`. |
| AC-010 | `changelog_names_every_rule`; `cargo xtask spec-trace` | **red then green on both.** The changelog lint fails on a rule with no entry; `spec-trace` went red listing six rows still marked `†` *not found*, and green after `spec-trace --write` regenerated the generated region. |

The oracle direction was green from the first compile and is not a red-then-green
row: eight rules against `MemoryProjectionStore` on the tokio and blocking
emitters. That is deliberate — a conformance rule that failed the reference store
on its first run would be a rule written against a store, not against a clause.
The direction that had to be earned is the mutant one, and every row above
carries it.

## Commits

`feat(projection-store-freeze): Commit, rollback and drop rules` — one checkpoint
commit, the second of slice `commit-atomicity-and-mutants`, on
`initiative/from-contract-to-published-library`, immediately after
`feat(projection-store-freeze): The mutant registry` (this slice's first story,
`projection-mutant-registry`).

Named by subject and by predecessor rather than by hash, for the reason the
earlier reports in this project give: this file is committed *inside* the commit
it describes, so no hash it quoted could survive being written into it.
`git log --grep "Story: projection-store-freeze/commit-rollback-and-drop-rules"`
resolves it.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-testkit/src/projection.rs` | **Six rule bodies** (`:394-756`) and their six names added to the single `for_each_projection_store_rule!` enumeration (`:772-792`) — the composition root for a conformance rule, since a rule not named there is compiled by nothing. Two helpers: `rollback_ok` and `after`, the latter taking `SequencePosition::next()`'s `None` arm as a panic with a message rather than a silent `unwrap`. Three new probe constants, so a rule asserting about two rows can tell them apart. Module doc and two rule docs corrected: they described `CheckpointOnlyStore` as a debt arriving with a later story, which stopped being true one commit ago. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs` | The `Defect` seam grew from one step to seven — `mint_stamp`, `checkpoint_key`, `regression`, `validate_position`, `commit_writes`, `rollback`, `release_the_connection` — one per commit-path obligation the suite now enforces, and each added because *a rule acquired the ability to see a defect there*. `MutantError` went from uninhabited to two variants, which is a visible withdrawal of the claim "the correct core cannot fail". The store and its batch now model **one connection**: an `Rc<Cell<bool>>` the store holds and the batch shares, released explicitly by `commit`/`rollback` and by the batch's `Drop` otherwise. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs` | **Six new hostile stores**, each an `impl Defect` overriding exactly one step: `UnrolledBackStore`, `PooledConnectionStore`, `TypeStampedBatchStore`, `ValidatingCommitStore`, `UnconditionalCheckpointStore`, `SingleRowCheckpointStore`. Each was named defect-first, from §4.11's *Rejects* column, before it was named at all. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | Six `Declared` rows (`:353-450`), and the two existing rows grown (`:264-300`) with a comment on each saying which repair was refused. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `PROJECTION_MUST_REJECT` grew from two names to seven (`:3339-3366`). The eighth rule is **absent on purpose** and the doc says why: `commit_rejects_a_foreign_batch` spells no gate, so its `Verdict::Passed` against a fixture declining everything is what this list asserts by omission. |
| `CHANGELOG.md` | One entry per landed rule naming the defect it detects, plus one for the registry itself; and the sentence claiming `CheckpointOnlyStore`'s rejection was a carried debt corrected in place. |
| `spec/SPECIFICATION.md` | **The generated region only.** `cargo xtask spec-trace --write`; 6 insertions, 6 deletions, every one of them a `†` removed from PS-7, PS-8, PS-15, PS-21, PS-22 or PS-23. |
| `.bklg/…/commit-rollback-and-drop-rules/_ledger.md` | Nine rows flipped `false` → `true` with citations. **AC-001 left `false` with empty evidence**, which is what a blocked row looks like. |

No `Cargo.toml` was edited, no dependency added, no public item created, no
`Capability` or `RuleOutcome` changed, and no `[FROZEN]` clause hand-edited.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance-testkit --all-features` | green — `projection_conformance` `8 passed`, `projection_conformance_blocking` `8 passed`, `projection_mutation_coverage` `4 passed`, `mutation_coverage` `10 passed` |
| `cargo test --workspace --all-features` | green across every target; nothing outside the testkit moved |
| `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | clean — **run explicitly**, because `cargo xtask ci --fast` omits the mandatory conformance-harness step and AC-009's constrained-runtime arm rests on it |
| `cargo xtask ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)`, including `CF-6: no position-shaped literals in 4 rule file(s)`, `CF-33: 8 file(s) … read no clock`, and `CF-29: all 103 rules in 4 file(s) have a changelog entry` |
| `cargo xtask spec-trace` | red before regeneration, listing six rows still marked `†`; **exit 0** after `spec-trace --write`, `traceability: no problems found; §7.1–§7.2 matches the checker` |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo fmt --check` | clean. The formatter's `--write` pass ran **last**, after clippy. |

## Notes

**AC-001 is blocked, and this is the finding.** `failed_commit_leaves_both_unchanged`
has to be gated on a fixture capability that can arm a commit fault, in
`MID_BATCH_FAULT`'s mould — nothing in the port lets an outside caller make a
conformant store's `commit` fail, which is the whole argument that made
`MID_BATCH_FAULT` a capability rather than testkit machinery
(`crates/happenstance-testkit/src/contract.rs:196-211`). **The projection fixture
carries no such capability, and that is a decision rather than an omission.**
DT-3's resolution in the signed-off `_design.md` enumerates the projection
capability set as exactly three constants — `RESET_REFUSAL` and `SECOND_HANDLE`
on the fixture, `READS_THROUGH_BATCH` on `ProjectionProbe`
(`.bklg/from-contract-to-published-library/projection-store-freeze/_design.md:276-280`)
— and `crates/happenstance-testkit/src/contract.rs:519-596` implements exactly
that set. The same design table lists `failed_commit_leaves_both_unchanged` as
gated by `SECOND_HANDLE`, which cannot arm anything.

EC-002 is unambiguous about what to do: *"Report, do not invent. Which constant
exists is `_design.md`'s call … Minting a second declension policy here is the
failure `.kb/open-questions/cf-40-fixture-limits-ownership.md` names."* So the
rule is not landed. Three routes were considered and all three are gaming:

1. **Mint a `COMMIT_FAULT` capability here.** Contradicts a signed-off design
   record and mints the second declension policy the project charter's own risk
   register forbids in those words.
2. **Write the rule using a failure the port already reaches** — a foreign batch,
   or a regressing position. Both are already asserted to leave everything
   unchanged by `commit_rejects_a_foreign_batch` and
   `commit_rejects_a_regressing_position`, so the "new" rule would be a duplicate
   with no mutant that fails it *differently*. CF-1 could be satisfied by
   declaring an existing mutant twice, and the result would be exactly the
   decorative rule ADR-0010 exists to make unwriteable.
3. **Land the rule as an unconditional skip.** A rule that can never run is a
   rule absent from the binary wearing a `SKIP` line, which is the shape CF-18
   names as the thing to prevent.

What is owed, and by whom: a decision — on `projection-api-design-record`'s
ground, or as an amendment to DT-3 — about whether the projection fixture gains a
commit-fault capability. PS-1's *second conjunct* is unenforced until it does, and
that is a gap in the freeze evidence rather than in this story: `project.md`'s own
Risks section anticipated it, and `unstable-projection-gate-and-clause-disposition`
is where a clause whose rule cannot be written gets its maturity marker
reconsidered.

**EC-003 obtained, and was honoured rather than worked around.** The spec predicted
`dropped_batch_leaves_store_usable` would legitimately catch `CheckpointOnlyStore`,
and it did — at the assertion that the *second* batch's row is present, which is
the rule's non-vacuity anchor. It caught more than that once the matrix was
complete: `CheckpointOnlyStore` now fails three rules and
`UncommittedTransactionStore` five. Both declarations grew **in this PR**, each
with a comment naming the refused repair (weakening the new rule so the old
declaration survived). The five-rule row is worth a reviewer's attention and its
comment says so: a store that makes nothing durable trips every rule whose
*anchor* is a committed state, at that anchor rather than at the property the rule
is named for. That is **inflation, not vacuity**, and it meets the bar the
event-store registry sets for tolerating it — no rule is covered by that store
alone except `commit_advances_the_checkpoint`, which is the one it was written for
and the one §4.11's table leaves an em-dash against.

**EC-005 did not obtain, and it was checked rather than assumed.**
`xtask/src/spec_trace.rs:88-93` is `RULE_FILES: [&str; 4]` and already contains
`crates/happenstance-testkit/src/projection.rs`, added by
`projection-suite-entry-point`. No edit was needed and none was made. The gate's
own output is the confirmation that the sweep reaches the projection module rather
than printing green over nothing: `CF-6: no position-shaped literals in 4 rule
file(s)` and `CF-29: all 103 rules in 4 file(s) have a changelog entry` — 103 is
the count *with* the six new rules in it.

**One rule deliberately spells no capability gate, and that is asserted rather
than merely intended.** `commit_rejects_a_foreign_batch` wants two *isolated
stores*, not two handles onto one, so it does not write `must!(F: SECOND_HANDLE)`.
The consequence is checkable: it is the one projection rule absent from
`PROJECTION_MUST_REJECT`, which means
`mutation_coverage::projection_capability_skips_are_reported` requires it to
**pass** against a fixture that declines everything. Adding it to that list "for
safety" would make the assertion unreachable, and the list's doc comment now says
so.

**PS-1's scope gap was not absorbed.** Writing the rules next to a clause whose
MUST is a coupling rather than a progress obligation makes closing it feel like
diligence. It is `ps-clause-pairing-sweep`'s, PS-1 is `[FROZEN]`, and the only
thing this story does about it is register the store that demonstrates it — which
`projection-mutant-registry` already did. `git diff spec/SPECIFICATION.md` is six
lines, all inside the machine-generated region.

**The `wasm32` blind spot is inherited and restated so nobody reads the gate as
covering it.** The projection mutant binary is gated off that target, so the eight
hostile stores are never type-checked for `wasm32`. The mandatory
conformance-harness check covers the *rules* on that target — which is AC-009's
arm — and covers none of the mutants. That is the stated cost of being able to
`catch_unwind` at all, not a defect to fix here.
