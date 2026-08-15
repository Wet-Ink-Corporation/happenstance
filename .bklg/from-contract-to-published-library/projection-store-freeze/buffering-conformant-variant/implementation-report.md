---
item: "HS-S0013"
stage: implement
created: "2026-08-14"
updated: "2026-08-14"
---

# Implementation Report — A second, structurally unlike batch shape passes the whole suite

> **STATUS: seven of seven ACs satisfied. Nothing blocked, nothing stubbed.**
> One store, one fixture, one registry row, one enumeration entry, one harness
> target, one observation note. **Zero conformance rules added, zero edited,
> zero new capabilities, nothing under `crates/happenstance-core/src/`, no new
> dependency and no new feature.**
>
> **What the projection suite is after this commit.** Sixteen rules, driven
> against **two** fixtures inside one `cargo xtask ci` — `MemoryProjectionFixture`
> over a materialised-delta batch, and `BufferingProjectionFixture` over a
> replayable op journal that holds nothing between `begin` and `commit`. The
> family's CF-5 positive control now asserts both halves: no variant rejected,
> and every rule executed against a variant.
>
> **One finding, reported rather than absorbed**, and it is the interesting
> product of the run: `MemoryProjectionStore` is *itself* already a deferred
> write set, so the axis this pair actually spans is narrower than PS-2's. It is
> written up in `two-shape-observation.md` and is the substance
> `ps3-batch-shape-finding` composes into PS-3's evidence. No rule moved because
> of it.

## TDD Evidence

Three reds, each a real assertion failure rather than a compile error, and each
recorded with the message it produced.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-004 | `projection_mutation_coverage::projection_conformant_variants_pass_everything` | **Red first, before any store existed.** The second assertion — *every projection rule executed against a variant* — was added to the control and immediately failed: ``` `batch_reads_reflect_pending_writes` never ran against a conformant variant — every variant skipped it — so nothing here says the rule **accepts** a legal store ```. With `NoBatchReadStore` the only variant, two rules had never been *accepted* by anything legally different from the reference store. **Green** when `BufferingProjectionStore` — `READS_THROUGH_BATCH = true`, every fixture capability supported — was registered. This red is also the non-vacuity demonstration AC-004 asks for, run in the other direction: the guard was observed failing with the row absent, so no uncommitted deletion was needed. |
| AC-003 | `projection_mutation_coverage::projection_mutant_registry_is_exhaustive` | **Red** with the enumeration entry added and the `Declared` row deliberately withheld: ``` `BufferingProjectionStore` is enumerated in `for_each_projection_mutant!` but has no `REGISTRY` row, so it is driven and nothing is claimed about it ```. **Green** when the hand-written row landed with `kind: Kind::ConformantVariant`, `fails: &[]`, `expect: &[]` and a provenance naming the Workers / Neon shape. |
| AC-001 | `projection_conformance_buffering::the_read_model_changes_only_at_commit` | **Red** with `probe_write` temporarily applying straight to committed rows — the vacuous-variant shape EC-005 names — at *"a row staged in an open batch must not be visible in committed state"*, `left: Some(9)`, `right: None`. **Green** with the line reverted and the write appended to the journal only. The same experiment measured what the suite alone can see: five of the sixteen rules went red with it (`rollback_leaves_both_unchanged`, `dropped_batch_leaves_store_usable`, `failed_commit_leaves_both_unchanged`, `commit_rejects_a_foreign_batch`, `commit_rejects_a_regressing_position`), which is why this assertion exists — the *compensating* apply-on-write store passes all sixteen. |
| AC-002 | `projection_conformance_buffering::projection_conformance::*` (16 emitted tests) + `projection_mutation_coverage::the_second_batch_shape_answers_every_rule_with_a_pass` | **Green from the first compile** for the sixteen emitted tests and reported as such, per this project's convention for the oracle direction. The `RuleOutcome`-grounded meta-test is new and was written against the same run: outcome set equals the enumeration in order, every `Verdict` is `Passed`, `declines` empty. |
| AC-005 | `cargo xtask ci`, whole, clean tree | **Green**, `all checks passed`, with both fixtures named in the one `tests` step. Run in full rather than `--fast` because AC-005's second half is the wasm32 conformance-harness check, which `--fast` omits. |
| AC-006 | reviewed diff over `crates/happenstance-testkit/tests/**` | **Green.** One file defines the store and the fixture; two `#[path]` includes name the same path string. Not compiler-checkable — clippy finds a *dead* duplicate, not a used one — so it is a named diff assertion. |
| AC-007 | `two-shape-observation.md` + diff-scope assertion | **Green.** Written while the run was live; specific, not a shrug. |

## Commits

`feat(projection-store-freeze): The second, unlike batch shape` — one checkpoint
commit, the first of slice `second-batch-shape-and-evidence`, on
`initiative/from-contract-to-published-library`.

Named by subject rather than by hash for the reason the earlier reports in this
project give: this file is committed *inside* the commit it describes.
`git log --grep "Story: projection-store-freeze/buffering-conformant-variant"`
resolves it, and `ps3-batch-shape-finding` pins that sha in the evidence
document it writes next.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/buffering.rs` | **New, and the whole of the store.** `Op` (a two-variant journal entry), `replay`, `Staged`/`staged` (the backwards scan a queued-statement adapter can answer without a server), `BufferingBatch` (stamp + journal, owning nothing of the store's), `BufferingProjectionStoreError::CommitFault`, `Committed` (rows and checkpoints behind **one** `RefCell`), `BufferingProjectionStore` and `BufferingProjectionFixture`. Bare `ProjectionStore` flavour, `Rc`-backed, no `#[async_trait]`, no borrowing GAT. The module doc states the one axis it differs on, cites PS-4 and PS-5 for its legality, states what it deliberately does **not** differ on (`READS_THROUGH_BATCH = true`, and why that is load-bearing rather than a softening), and states what a green run against it does not prove (PS-2). |
| `crates/happenstance-testkit/tests/projection_conformance_buffering.rs` | **New harness target**, beside `projection_conformance.rs` and mirroring it: same `#![cfg(not(target_arch = "wasm32"))]` discipline, one `projection_store_conformance!` invocation, **no rule name anywhere in the file**. Plus `the_read_model_changes_only_at_commit`, the one assertion the suite cannot make about itself. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage.rs` | The `#[path]` include (`:84-85`), the enumeration entry (`:863`), the hand-written `Declared` row (`:802-830`), `run_buffering_store()` (`:919`), the second assertion in `projection_conformant_variants_pass_everything` (`:1350-1361`) and the new `the_second_batch_shape_answers_every_rule_with_a_pass` (`:1446`). Two stale doc sections corrected: the module doc's *"CF-5's half is deferred by name"* (which had said **no row uses it yet** since before the previous story) and the `REGISTRY` doc's *not covered* bullet, which now names the axis that genuinely has no row — a batch whose commit crosses a real process boundary. |
| `crates/happenstance-testkit/tests/projection_mutation_coverage/variants.rs` | `impl ProjectionSubject for BufferingProjectionFixture`, with the reason it lives here rather than in `buffering.rs`: the trait is the mutant binary's, and `buffering.rs` must compile in a target that cannot see it. `NoBatchReadStore`'s *"what it is not"* paragraph updated — the buffering variant has landed, declares the switch `true`, and the two are opposite arms of one gate rather than duplicates. |
| `crates/happenstance-testkit/tests/projection_conformance.rs` | Module doc only: one paragraph naming the sibling harness and what the pair buys. No change to the invocation, the fixture, the emitter or the target discipline. |
| `.bklg/…/buffering-conformant-variant/two-shape-observation.md` | **New.** AC-007's deliverable. |
| `.bklg/…/buffering-conformant-variant/_ledger.md` | Seven rows flipped `false → true` with cited evidence. No criterion re-worded. |

**Not touched, and checked:** `crates/happenstance-core/**` (no diff at all),
`crates/happenstance-testkit/src/**`, `crates/happenstance-testkit/Cargo.toml`,
`spec/SPECIFICATION.md`, `memory_conformance.rs`, and every `happenstance-sqlite`
/ `-postgres` / `-ladybug` / `-neon` crate.

## Gates

| Gate | Command | Result |
| ---- | ------- | ------ |
| Story inner loop | `cargo test -p happenstance-testkit --test projection_conformance_buffering` | 17 passed, 0 failed |
| Story inner loop | `cargo test -p happenstance-testkit --test projection_mutation_coverage` | 7 passed, 0 failed |
| Story inner loop | `cargo test -p happenstance-testkit --test projection_conformance --test projection_conformance_blocking` | 16 passed each, 0 failed |
| Lint | `cargo clippy -p happenstance-testkit --all-targets --all-features -- -D warnings` | clean (two findings fixed on the way: `clippy::option_option` on the read-through helper, which became the three-state `Staged` enum, and one `clippy::needless_borrow`) |
| Format | `cargo fmt --all -- --check` | clean, and run **last**, after the lint fixes |
| Whole gate | `cargo xtask ci` | `all checks passed` — fmt, clippy, tests, four wasm32 steps, docs, `spec-trace`, the `--no-default-features` doc build, the packaging assertion, `cargo hack` feature-powerset, `cargo deny` and the nightly `--cfg docsrs` build |

Both fixtures inside that one run: `Running tests\projection_conformance.rs` and
`Running tests\projection_conformance_buffering.rs`, in the same `tests` step.
No `#[ignore]`, no `required-features`, no second command.

## Notes

**Three deviations from the plan, each deliberate and each reported rather than
absorbed.**

**1. The spec's premise about `MemoryProjectionStore` is wrong, and the story is
delivered anyway.** The spec (and §4.11's own sentence) calls the reference store
*apply-on-write*. It is not: `begin` returns an owned `MemoryProjectionBatch`
holding a delta and `commit` applies it
(`crates/happenstance-core/src/projection_memory.rs:280-330`), so it already sits
at PS-4's deferred-write-set end. Rather than reshape the variant toward the
reference store — the forbidden response — or reach for a live-handle store the
spec explicitly rules out (`begin` acquires nothing), the store was built to the
spec's *literal* description: `begin` allocates a write set and holds nothing,
**every probe write appends an op**, `commit` replays them. That op journal is a
real structural difference from a collapsed delta, and the residual narrowing of
the axis is written up in `two-shape-observation.md` for `ps3-batch-shape-finding`
rather than papered over. EC-005 is not triggered: AC-001's direct assertion
holds, and it holds for the right reason.

**2. Two meta-tests changed.** `projection_conformant_variants_pass_everything`
gained the "every rule executed against a variant" assertion — AC-004 requires it
in as many words, and the event-store family has carried it since phase 4
(`tests/mutation_coverage.rs:3128-3137`) — and
`the_second_batch_shape_answers_every_rule_with_a_pass` is new. Neither is a
conformance rule: no adapter can pass or fail either, so CF-29's
mutant-plus-changelog obligation is not incurred, which is the cross-check the
spec names for this exact drift.

**3. The `ProjectionSubject` impl is in `variants.rs`, not in `buffering.rs`.**
The spec asks for the store and fixture in one file reached by two consumers.
`ProjectionSubject` is declared in the mutant binary's `harness.rs`, and the
conformance target cannot see it without dragging `correct.rs`, `harness.rs` and
their dead code into a binary that would then fail `-D warnings`. Splitting the
*registration* from the *store* keeps the store itself a single definition, which
is what AC-006 is actually about.

**Two things deliberately not done.** The buffering fixture supports
`COMMIT_FAULT` and `RESET_REFUSAL` rather than declining them, because a
declining variant would leave those two rules unexecuted against every conformant
variant — the same hole AC-004 exists to close, one level down. And no wasm32
harness target was added for this fixture: the density budget is +1 harness, the
store reaches for nothing host-only, and the existing
`projection_conformance_wasm.rs` already proves the emitter path for this family.
