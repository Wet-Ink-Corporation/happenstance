---
item: "HS-S0012"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — Read-your-writes, chunk-size invariance and rebuild authority

> **STATUS: eight of eight ACs satisfied.** Three rules landed in the projection
> rule family, mounted at all four composition points, each with a registered
> wrong store that fails it by name — and the family's **first conformant
> variant**, which is what makes the two gated rules' skip arm a branch something
> executes rather than a claim.
>
> **What the projection suite is after this commit.** Seventeen rules. Every rule
> §4.11 assigns to an adapter's own suite is written; what is still owed is the
> six runner-dependent ones CF-36 moves to the workspace e2e crate. Both arms of
> `READS_THROUGH_BATCH` have a fixture behind them in the same run — the
> reference store holds `true`, `NoBatchReadStore` holds `false` — which is the
> structural guard that keeps a gated rule from quietly becoming dead code.

## TDD Evidence

Same order as the slice-mate, and the same reason: the rules first, so the first
red is CF-1 *demanding* the stores.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `projection_conformance::batch_reads_reflect_pending_writes` + its cell in `projection_mutants_fail_exactly_their_declared_rules` | **red**: `every_projection_rule_has_a_mutant` named it among three rules that *"have no mutant … so they certify nothing"*. **green** with `CommittedReadBatchStore`, pinned at `"must let an open batch see"`. |
| AC-002 | `projection_conformance::rebuild_is_chunk_size_invariant` + two cells | **red** in the same run; **green** with `CommittedReadBatchStore` **and** `FirstWriteWinsBatchStore`, both pinned at `"must produce the same read model however it is"`. The two diverge for different reasons — reads from committed state, and `or_insert` keeping the first staged value — which is why the rule needs an increment rather than a `set`. |
| AC-003 | `projection_conformance::rebuilding_is_distinguishable_from_live` + its cell | **red** in the same run; **green** with `LiveOnlyCheckpointStore`, pinned at `"must leave a checkpoint a reader can recognise as a rebuild"`. |
| AC-004 | the four projection exactness meta-tests over the widened rule set | **red a second time, for the right reason**: `projection_mutants_fail_exactly_their_declared_rules` reported ``\`UncommittedTransactionStore\` failed \`rebuilding_is_distinguishable_from_live\`, which it does not declare``. **green** when that declaration grew with a comment — never by weakening the new rule. |
| AC-005 | `projection_mutation_coverage::projection_conformant_variants_pass_everything` | **red** the moment `NoBatchReadStore` was registered without the positive control existing: the row used `Kind::ConformantVariant`, whose `#[expect(dead_code)]` then went **unfulfilled** and failed the build — which is precisely the trip-wire that attribute was left as. **green** with the attribute removed and the control landed, including its own non-vacuity assertion that at least one variant was driven. |
| AC-006 | `projection_mutation_coverage::a_batch_with_no_read_path_is_reported_as_a_skip` | **red** before `require_read_through!` existed (the rules ran against a store whose `probe_read_through` is `unimplemented!()` and panicked with *"not yet implemented"*, which the harness classifies as the store falling over). **green** with the gate, asserting the skip set is exactly the two gated rules and both fields of each `RuleOutcome::Skipped` against the exported constants. |
| AC-007 | `no_orphan_projection_rules`; both host emitters; `cargo check --tests --target wasm32-unknown-unknown` | **green**, 17 tests per host harness, and the wasm arm run explicitly because `--fast` omits it. |
| AC-008 | `cargo xtask spec-trace`; `git diff --stat` over the PR boundary | **red then green**: `spec-trace` listed four generated rows still marked `†` *not found* and went green after `--write`. Clause text, maturity markers and rule citations are untouched, and `crates/happenstance-core/**` has no diff from this story at all. |

The oracle direction was green from the first compile and is not a red-then-green
row, for the slice-mate's reason.

## Commits

`feat(projection-store-freeze): Read-through and rebuild rules` — one checkpoint
commit, the second and last of slice `reset-and-rebuild-rules`, on
`initiative/from-contract-to-published-library`, immediately after
`feat(projection-store-freeze): Reset is one unit of work`.

Named by subject and by predecessor rather than by hash, for the reason the
earlier reports in this project give: this file is committed *inside* the commit
it describes. `git log --grep "Story: projection-store-freeze/read-through-and-rebuild-rules"`
resolves it.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-testkit/src/projection.rs` | **Three rule bodies** and their three names in the single `for_each_projection_store_rule!` enumeration (`:1878-1880`). One new gate macro, `require_read_through!` (`:204`), which is `require!`'s third sibling and reads `<F::Store as ProjectionProbe>::READS_THROUGH_BATCH` — a `const` on the **store's probe**, so there is no identifier for `require!` to `stringify!`. One new module const, `REBUILD_SEQUENCE`, at module scope rather than in the rule body because `clippy::items_after_statements` is denied. Module doc updated twice: the family is complete apart from CF-36's six runner-dependent rules, and a third switch that can produce a skip is now documented beside the two fixture capabilities. |
| `crates/happenstance-testkit/src/contract.rs` | Two new exported constants beside `NO_STORE_LIMITS` and `NO_CEILING_REASON`: `NO_BATCH_READ_PATH` (`:836`), which names the probe const **with its path** so a skip sends an author to a constant that exists in their code, and `NO_BATCH_READ_PATH_REASON` (`:852`), which is testkit-written for `NO_CEILING_REASON`'s reason and is the second and last instance of that exception. Its doc states what a skip carrying it does *not* cover. |
| `crates/happenstance-testkit/src/lib.rs` | The two constants re-exported at the crate root. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/correct.rs` | The `Defect` seam grew from eleven steps to fifteen: `READS_THROUGH_BATCH` (a `const`, because the gate reads it before a store exists), `stage_write`, `probe_read_through` and `checkpoint_for`. The `ProjectionProbe` impl and `commit` now route through them, so a wrong store overrides one step rather than reimplementing a path. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/mutants.rs` | **Three new hostile stores**, each overriding exactly one step: `CommittedReadBatchStore`, `FirstWriteWinsBatchStore`, `LiveOnlyCheckpointStore`. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/variants.rs` | **New file**, mirroring the event-store family's: the projection stores that are legally *different* rather than wrong. It holds `NoBatchReadStore`, whose two overrides — the `const` and the `unimplemented!()` — are one property, and whose doc says why it is not the buffering replay-at-commit variant a later story owes. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | Four `Declared` rows (three mutants, one conformant variant), four entries in `for_each_projection_mutant!`, `Kind::ConformantVariant`'s `#[expect(dead_code)]` removed, one existing declaration grown (`UncommittedTransactionStore`), and **two new meta-tests**: `projection_conformant_variants_pass_everything` (CF-5's positive control, with its own non-vacuity assertion) and `a_batch_with_no_read_path_is_reported_as_a_skip`. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/harness.rs` | `declines` now includes the probe-side switch when a subject declares it `false`, reading the testkit's own two constants. Without that a conformant variant's skip would be one the meta-tests could not account for — the "a rule stopped running and nothing noticed" hole `SubjectReport::declines` exists to close. |
| `crates/happenstance-testkit/tests/mutation_coverage.rs` | `PROJECTION_MUST_REJECT` grew by one (`rebuilding_is_distinguishable_from_live`); its doc now explains why the other two new rules are absent — one reads through the same handle that owns the batch, the other opens a fixture per run. |
| `CHANGELOG.md` | One entry per landed rule naming the defect it detects, plus one for the conformant variant and the two new constants. |
| `spec/SPECIFICATION.md` | **The generated region only.** `spec-trace --write`; 4 insertions, 4 deletions, every one a `†` removed from PS-12, PS-13, PS-14 or PS-24. |
| `.bklg/…/read-through-and-rebuild-rules/_ledger.md` | Eight rows flipped `false` → `true` with citations. |

`crates/happenstance-core/**` is untouched by this story: no port, probe,
`Checkpoint`, `Authority` or `MemoryProjectionStore` change, and no
`READS_THROUGH_BATCH` flipped to make a rule convenient.

## Gates

| Gate | Result |
| ---- | ------ |
| `cargo test -p happenstance-testkit` | green — 17 projection rules on each host harness; six projection meta-tests; nine event-store meta-tests. |
| `cargo test -p happenstance-testkit --test projection_mutation_coverage` | green — including the two new ones. |
| `cargo xtask ci --fast` | green (fmt, clippy `-D warnings`, tests, proof artefacts, docs, `spec-trace`, the four rule lints, the testkit version check, `lint-constitution`). |
| `cargo check -p happenstance-testkit --tests --target wasm32-unknown-unknown` | green — the wasm harness type-checks all seventeen rules. |
| `cargo xtask spec-trace` | green after `--write`; the diff over `spec/` is the generated region and nothing else. |

## Notes

**1. The gate is on the probe, and that is why `require!` could not serve it.**
`require!` expands to `<F as ProjectionFixture>::$capability.reason()` and
`stringify!`s the const's own identifier. `READS_THROUGH_BATCH` is not a
`Capability` and not on the fixture — it is a `bool` on the store's
`ProjectionProbe` impl, because whether a batch can be read through is a property
of the batch *type* rather than of the fixture's environment. So the capability
name is a `&'static str` carrying the **path**, `ProjectionProbe::READS_THROUGH_BATCH`,
which is the `NO_STORE_LIMITS` precedent applied to its second case: a skip
naming a fixture const that does not exist in the author's code sends them
looking for something they cannot find. No second skip type, no second line
shape, one `RuleOutcome::Skipped` rendered by the same `skip_line`.

**2. Both gated rules skip together, and the residual is documented rather than
hidden.** `rebuild_is_chunk_size_invariant`'s prescribed shape defines each step
as an increment of what the batch can see, so it needs `probe_read_through` and
is gated on the same const. The alternative — a *blind* write sequence for
declining adapters, so the rule reports `Ran` — was rejected: blind writes are
chunk-size invariant by construction, which is a rule no adapter can fail
arriving through the back door. What that leaves is real and is written into the
rule's own doc and into `NO_BATCH_READ_PATH_REASON`: **chunk-size invariance is
unverified for a write-behind adapter**, because such an adapter has no read path
for a projection's read-modify-write to use in the first place.

**3. `Kind::ConformantVariant`'s `#[expect(dead_code)]` was the trip-wire it was
left as, and it fired.** The attribute's own text said it *"goes unfulfilled —
and therefore red — on the day a row uses it, which is the day the deferral
ends"*. Registering `NoBatchReadStore` made that day today, the build went red,
and the repair was to remove the attribute and land the positive control the same
comment named. That is the mechanism working, not an obstacle: without it the
first conformant variant could have landed with no assertion that a conformant
store passes anything.

**4. The positive control asserts its own non-vacuity.** `projection_conformant_variants_pass_everything`
counts the variants it drove and fails if the count is zero. Over an empty set it
would otherwise pass while asserting nothing, which is the vacuity CF-5 exists to
prevent reintroduced one level up — the exact reason the registry's module doc
gave for *not* landing it earlier.

**5. `UncommittedTransactionStore` grew a seventh declared failure.** A store
that makes nothing durable has no checkpoint for a reader to recognise a rebuild
in, so the first `Rebuilding` assertion sees `NeverRun`. The repair refused, as
in the slice-mate, was weakening the new rule so the old declaration survived.

**6. What this story deliberately did not do.** It did not touch
`crates/happenstance-core/**`, flip `MemoryProjectionStore`'s
`READS_THROUGH_BATCH`, change `Capability`, `RuleOutcome`, the `Declared` shape
or any emitter, or present `NoBatchReadStore` as the buffering replay-at-commit
conformant variant. That variant is a different and larger instrument at the far
end of §6's batch-shape axis and belongs to `buffering-conformant-variant`; if it
also declares `false` when it lands, that is a second instance rather than a
duplicate to delete — the skip arm must never end up with zero fixtures behind
it.
