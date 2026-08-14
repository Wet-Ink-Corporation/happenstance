---
item: "HS-S0011"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — reset is one unit of work, scoped, refusable, and not commit-at-FIRST

## Findings Ledger

**Nine of ten ACs satisfied. AC-005 is BLOCKED — EC-001 fired, and the first
version of this report said it had not.** Four rules, five wrong stores, three
grown declarations, and one new provided method on the projection fixture trait
that is a mechanism rather than a capability. One rule and its store are **held**,
not dropped: the reason is below and in the tree at both points they would sit.
No clause text edited, nothing stubbed.

| AC | Result | What proves it | Where it is mounted |
| -- | ------ | -------------- | ------------------- |
| **AC-001** | **satisfied** | `projection_conformance::reset_clears_rows_and_checkpoint_together` (green against the oracle on the tokio and blocking harnesses) and its cell in `projection_mutants_fail_exactly_their_declared_rules` (red against `TwoStatementResetStore`, pinned at `"must return this projection's checkpoint to"`). Both halves are read through a **fresh handle**; PS-16's failure-injecting half is asserted through `ResetError::ForeignBatch` rather than left absent, and `TypeStampedBatchStore` fails there. | `for_each_projection_store_rule!`, `crates/happenstance-testkit/src/projection.rs:1564` |
| **AC-002** | **satisfied** | `reset_is_scoped_to_one_projection` — two ids in one store, the sibling's rows *and* checkpoint compared against a fresh-handle before-state, and the reset id's own checkpoint back to `NeverRun` as the non-vacuity anchor. Red against `TruncatingResetStore` **and** `SingleRowCheckpointStore`. | `…/projection.rs:1565` |
| **AC-003** | **satisfied** | `refused_reset_changes_nothing` — `Err(ResetError::Refused)` **and** both halves unchanged through a fresh handle. Red against the pair AC-003 names: `RefusalAsSuccessStore` (refusal reported as success) and `RefusalAfterTheFactStore` (refusal reported after the deletes). | `…/projection.rs:1566` |
| **AC-004** | **satisfied** | `projection::a_declined_reset_refusal_is_reported_with_the_fixtures_reason` asserts the `RuleOutcome` **value** — `Skipped { capability: "RESET_REFUSAL", reason }` — with the reason read from `MemoryProjectionFixture`'s own `const`, never from a repeated literal and never from stdout. `mutation_coverage::projection_capability_skips_are_reported` pins the reference fixture's whole skip set to exactly two rules and checks each one's capability and reason. No projection-local skip type was invented. | `crates/happenstance-testkit/src/projection.rs:2042`; `…/tests/mutation_coverage.rs:3481` |
| **AC-005** | **BLOCKED — EC-001 fired** | **The precondition was NOT met.** AC-005's verification column requires *"the PS-19 repair atom from `projection-decision-atoms` is `status: accepted` under `.kb/decisions/`"*. **No such atom exists** — `.kb/decisions/` holds 0001–0019 and 0029, and none widens PS-19. This report's first version cited ADR-0018's own `status: accepted` as satisfaction; that is a different atom answering a different question, and ADR-0018 says so about itself at `.kb/decisions/0018-returning-a-projection-to-never-run.md:113-116`: PS-19's pairing defect *"sits inside this clause range and is named here as a gap, not repaired"*. `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md:122-129` agrees — *"ADR-0018 … scoped this defect out of its own range by name and repaired nothing … sub-questions 1 and 3 stay open"* — and routes the repair to `unstable-projection-gate-and-clause-disposition`. The slice-1 story says it in as many words (`projection-decision-atoms/spec.md:72`), and so does the specification about itself (`spec/SPECIFICATION.md:5238-5239`). **Handling taken, EC-001's own remedy:** `fresh_projection_has_no_checkpoint` and `PresumedLiveCheckpointStore` are **held** — removed from the rules module, the enumeration, `mutants.rs`, the `REGISTRY` and `for_each_projection_mutant!`, each with the reason stated at the point it would sit — the clause is **not** line-edited, and the rule is **not** dropped silently: `CHANGELOG.md` names the non-delivery and `spec-trace --write` restored `†` on §7.2's PS-19 row. | held; the enumeration comment at `…/projection.rs:1861` marks where it belongs |
| **AC-006** | **satisfied** | `reset_is_not_commit_at_first` — both halves. The **state** half compares `core::mem::discriminant` of the two checkpoints; the **consequence** half derives a resume point from each through `resumes_over` (strictly after a recorded position, inclusive from the first position when `NeverRun`) and asserts the event at the first position is applied after the reset and *excluded* after the substitute. Red against `CommitAtFirstResetStore`. No runner was built. | `…/projection.rs:1865` |
| **AC-007** | **satisfied** | `TruncatingResetStore`'s row: `fails: &["reset_is_scoped_to_one_projection"]`, non-empty provenance naming the truncating `SqliteProjectionStore::reset()` and the `van_stock` / `fgas_ledger` consequence, and an `expect` pin. Held by all four projection exactness meta-tests. | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:591` |
| **AC-008** | **satisfied for the four rules delivered** | Every one of the four has at least one registered wrong store failing it, and each of the five new stores overrides **one** `Defect` step: `reset_writes` (three), `refusal` (two). The sixth — `PresumedLiveCheckpointStore`, one override of `missing_checkpoint` — is held with AC-005's rule, and `commit_rejects_a_foreign_batch`, the other rule it failed, keeps `TypeStampedBatchStore`, so CF-1 leaves no rule uncovered. No pass rate is computed or quoted, here or in the binary. | `…/tests/projection_mutation_coverage/mutants.rs:301-435`; rows at `…/projection_mutation_coverage.rs:554-686` |
| **AC-009** | **satisfied** | Every rule this story delivers is in the one enumeration — four names — so the tokio, blocking and `wasm32` harnesses each gained four tests with no per-harness list, and **no rule exists in `projection::rules` that a harness fails to run**, which is what this AC forbids. `no_orphan_projection_rules` makes an unmounted rule a test failure; `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` type-checks the wasm harness in the same gate run. The fifth name's absence is AC-005's row, not a mounting failure. | `…/projection.rs:1858-1865`; `…/tests/projection_conformance_wasm.rs` |
| **AC-010** | **satisfied** | `cargo xtask spec-trace` green (check 6 resolves the four delivered names and marks the held one `†`; `RULE_FILES` already named `projection.rs`, so EC-002 did not fire); the CF-29 changelog lint green over four entries each naming a defect, plus one naming a **non**-delivery; the CF-6 position lint green. `git diff -- spec/` is **the generated §7.1–§7.2 region and nothing else** — `†` markers added and removed by `spec-trace --write` as rule names started and stopped existing, no clause text, no maturity marker, no rule citation. | `CHANGELOG.md`; `spec/SPECIFICATION.md` generated region |

**Deferred: nothing. Blocked: AC-005**, on an accepted decision atom widening
PS-19 (or minting the clause that says it), owned by
`unstable-projection-gate-and-clause-disposition`. Until it lands, the suite
would otherwise be asserting an obligation the contract does not state, and the
store that exposes the gap — `.unwrap_or(Checkpoint::Live { through: FIRST })` —
is the natural implementation rather than a contrivance. CF-5's positive control
cannot catch that, because no conformant variant in the registry models the legal
store, which is a second reason the rule could not be left standing on the
strength of a green run.

**Path (b) is still open to a human and is not taken here.** The reviewer's
alternative was a recorded human decision to proceed, in the shape `_slices.md`
already carries for the DT-3 amendment of 2026-08-14. Nobody authorised
proceeding, so this run took EC-001 as written rather than inventing an
authorisation. What path (b) would cost, if a human takes it, is written into
`_slices.md` beside this finding.

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
