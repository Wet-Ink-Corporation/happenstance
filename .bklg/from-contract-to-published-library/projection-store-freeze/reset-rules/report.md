---
item: "HS-S0011"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

## Findings Ledger

**Ten of ten ACs satisfied, and the tenth arrived a day late by design.** Five
rules, six wrong stores, one conformant variant, three grown declarations, and one
new provided method on the projection fixture trait that is a mechanism rather
than a capability. No clause text edited, nothing stubbed.

**Read AC-005's row as a sequence, not as a row that was always green.** The rule
shipped once (`10ace94`), was found to convict adapters of an obligation no
clause stated, and was **withdrawn** rather than argued around (`064687a`) — its
mutant with it, because the store was *conformant*. The repair was never
available inside this story: it was an accepted decision atom, and the wave that
produces one is the human's to invoke. ADR-0030 landed at `9efda45`, minting
PS-38 rather than widening `[FROZEN]` PS-19, and the rule returned cited to the
new clause. PS-19 is byte-identical across the whole episode, which was the
point.

| AC | Result | What proves it | Where it is mounted |
| -- | ------ | -------------- | ------------------- |
| **AC-001** | **satisfied** | `projection_conformance::reset_clears_rows_and_checkpoint_together` (green against the oracle on the tokio and blocking harnesses) and its cell in `projection_mutants_fail_exactly_their_declared_rules` (red against `TwoStatementResetStore`, pinned at `"must return this projection's checkpoint to"`). Both halves are read through a **fresh handle**; PS-16's failure-injecting half is asserted through `ResetError::ForeignBatch` rather than left absent, and `TypeStampedBatchStore` fails there. | `for_each_projection_store_rule!`, `crates/happenstance-testkit/src/projection.rs:1907` |
| **AC-002** | **satisfied** | `reset_is_scoped_to_one_projection` — two ids in one store, the sibling's rows *and* checkpoint compared against a fresh-handle before-state, and the reset id's own checkpoint back to `NeverRun` as the non-vacuity anchor. Red against `TruncatingResetStore` **and** `SingleRowCheckpointStore`. | `…/projection.rs:1908` |
| **AC-003** | **satisfied** | `refused_reset_changes_nothing` — `Err(ResetError::Refused)` **and** both halves unchanged through a fresh handle. Red against the pair AC-003 names: `RefusalAsSuccessStore` (refusal reported as success) and `RefusalAfterTheFactStore` (refusal reported after the deletes). | `…/projection.rs:1909` |
| **AC-004** | **satisfied** | `projection::a_declined_reset_refusal_is_reported_with_the_fixtures_reason` asserts the `RuleOutcome` **value** — `Skipped { capability: "RESET_REFUSAL", reason }` — with the reason read from `MemoryProjectionFixture`'s own `const`, never from a repeated literal and never from stdout. `mutation_coverage::projection_capability_skips_are_reported` pins the reference fixture's whole skip set to exactly two rules and checks each one's capability and reason. No projection-local skip type was invented. | `crates/happenstance-testkit/src/projection.rs:2100`; `…/tests/mutation_coverage.rs:3481`, whose `assert_reference_projection_declensions` (`:3553`) pins the set |
| **AC-005** | **satisfied — precondition unmet at the first attempt, met now** | **Then:** AC-005's verification column requires an accepted atom widening PS-19. On 2026-08-14 none existed; ADR-0018 is a different atom that named this defect *"as a gap, not repaired"* (`.kb/decisions/0018-…:113-116`), so the rule and `PresumedLiveCheckpointStore` were **withdrawn**, each with the reason stated at the point it would sit, the `†` restored on §7.2, and the non-delivery named in `CHANGELOG.md`. No clause was line-edited. **Now:** `.kb/decisions/0030-the-checkpoint-reports-the-commits-that-happened.md` is `status: accepted` on this branch (merged `9efda45`, wave `2026-08-15-adr-0030-checkpoint-progress`). It did not widen PS-19 — byte-identical, `:74` — it **minted PS-38**, whose second sentence is the obligation verbatim: *"a `ProjectionId` no successful `commit` has named MUST read as `Checkpoint::NeverRun`"* (`spec/SPECIFICATION.md:5447-5449`). `fresh_projection_has_no_checkpoint` is written against **that** clause, asserts the `Checkpoint::NeverRun` **variant** with no position compared anywhere (CF-6 green), and is red against the restored `PresumedLiveCheckpointStore` at the pinned assertion. Its conformant neighbour on the same seam, `AbsentAfterResetStore`, landed with it — CF-5's control had no occupant there, which is why the over-convicting rule went out unseen. | `for_each_projection_store_rule!`, `crates/happenstance-testkit/src/projection.rs:1910`; rule body at `:1467` |
| **AC-006** | **satisfied** | `reset_is_not_commit_at_first` — both halves. The **state** half compares `core::mem::discriminant` of the two checkpoints; the **consequence** half derives a resume point from each through `resumes_over` (strictly after a recorded position, inclusive from the first position when `NeverRun`) and asserts the event at the first position is applied after the reset and *excluded* after the substitute. Red against `CommitAtFirstResetStore`. No runner was built. | `…/projection.rs:1911` |
| **AC-007** | **satisfied** | `TruncatingResetStore`'s row: `fails: &["reset_is_scoped_to_one_projection"]`, non-empty provenance naming the truncating `SqliteProjectionStore::reset()` and the `van_stock` / `fgas_ledger` consequence, and an `expect` pin. Held by all four projection exactness meta-tests. | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:620` |
| **AC-008** | **satisfied — all five rules** | Every one of the five has at least one registered wrong store failing it, and each of the six new stores overrides **one** `Defect` step: `reset_writes` (three), `refusal` (two), `missing_checkpoint` (one). The shapes are the ones §4.11 pre-specifies, including *"a store answering `Live { through: FIRST }` for an id it has never seen"*. No pass rate is computed or quoted, here or in the binary. | `…/tests/projection_mutation_coverage/mutants.rs:301-483`; rows at `…/projection_mutation_coverage.rs:554-748` |
| **AC-009** | **satisfied** | All five names are in the one enumeration, so the tokio, blocking and `wasm32` harnesses each gained five tests with no per-harness list, and **no rule exists in `projection::rules` that a harness fails to run**. Each host harness reports 17 rules. `no_orphan_projection_rules` makes an unmounted rule a test failure; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` type-checks the wasm harness in the same gate run. | `…/projection.rs:1907-1911`; `…/tests/projection_conformance_wasm.rs` |
| **AC-010** | **satisfied** (criterion re-synced to `spec.md:382`'s amended wording, which the ledger had not carried since `5f2cf02`) | `cargo xtask spec-trace` green — check 6 resolves all five names out of `RULE_FILES`, which already named `projection.rs`, so EC-002 did not fire. CF-29's lint green (*"all 112 rules in 4 file(s) have a changelog entry"*) over five entries each naming the defect its rule detects; the fifth **replaced** the non-delivery notice the withdrawal had left in the release notes, because a shipped changelog telling an adapter author the contract does not oblige something it now obliges is worse than silence. CF-6's position lint green. `git diff -- spec/` is **two lines**, both inside the `BEGIN/END GENERATED` region: the `†` off PS-19's row and off PS-38's. No clause text, no maturity marker, no rule citation. | `CHANGELOG.md`; `spec/SPECIFICATION.md` generated region |

**Deferred: nothing. Blocked: nothing.**

**What the episode cost and what it bought, since a green row hides both.** It
cost a day and two commits. It bought three things that outlive it: PS-38, a
clause the specification did not have and which three *already-shipped* rules were
silently resting on (`commit_advances_the_checkpoint`,
`commit_accepts_a_position_the_batch_did_not_write`,
`commit_rejects_a_regressing_position` — ADR-0030 §Decision); a registered
conformant variant on the missing-checkpoint seam, without which CF-5's positive
control was blind to exactly this class of over-conviction; and the demonstrated
path for repairing a `[FROZEN]` clause's rule pairing without touching the clause.
The rule was never dropped and never line-edited around: it was held, in the open,
with the argument at every point it would have sat.

**Two things checked before the rule was restored, because restoring it wrongly
was the available second mistake.** First, that PS-38's text genuinely rejects
`PresumedLiveCheckpointStore` — it does, by its second sentence, and its
`Rejects:` field names the wider version of the same defect (a store whose backing
state lives per *handle* rather than per store). Second, that this was not a
one-rule repair, which `_slices.md`'s run-6 instruction warned about explicitly.
It is not, and **the re-citing was already done by the ADR wave rather than owed
here**: PS-1 records the answer at `spec/SPECIFICATION.md:4755-4758`, PS-21 at
`:5379` and PS-22 at `:5407`, each naming PS-38 as the owner of the progress
obligation its rule rests on, and each `MUST` byte-identical. §7.2's generated
table lists `commit_advances_the_checkpoint` against PS-38 because that is PS-38's
own `Rule:` field; the other two stay filed under their own clauses, which is what
those clause bodies say should happen. Nothing further was owed and nothing further
was taken.

**One thing reported rather than absorbed**, and it is the only place this story
touched a decision it does not own. PS-16's rule is paired by the clause with a
failure-injecting variant, and the projection fixture's signed-off capability set
has nothing that can arm a failing `reset`. Rather than mint a fifth constant
inside a rule-writing story — the move `commit-rollback-and-drop-rules` refused,
and the reason `_design.md` carries a dated amendment — the rule uses the port's
own `ResetError::ForeignBatch`, which any caller can produce. If a later story
decides a `RESET_FAULT` capability is owed, this rule is where it would be spent;
nothing here pre-empts that decision, and nothing here waits on it.

**Two observations a reviewer should not have to re-derive.** First, three
existing mutant declarations grew, and every one of them grew rather than the new
rule being weakened — the alternative repair is named in each row's comment.
Second, `TwoStatementResetStore` and `CommitAtFirstResetStore` fail the same
three rules; they are kept as two stores because CF-4's provenance is the part a
reviewer reads, and collapsing them would discard one of two real deployment
mistakes.
