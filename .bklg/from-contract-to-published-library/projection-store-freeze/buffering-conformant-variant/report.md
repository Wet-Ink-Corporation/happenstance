---
item: "HS-S0013"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — A second, structurally unlike batch shape passes the whole suite

## Findings Ledger

**Seven of seven ACs satisfied. Nothing deferred, nothing stubbed, no
conformance rule added or edited, `crates/happenstance-core/**` untouched.** One
store, one fixture, one hand-written registry row, one enumeration entry, one
harness target, one observation note — and the projection family's CF-5 control
now makes a claim over the whole rule set rather than over the part of it a
single variant happened to reach.

| AC | Result | What proves it | Where it is mounted |
| -- | ------ | -------------- | ------------------- |
| **AC-001** | **satisfied** | `the_read_model_changes_only_at_commit` opens a batch, stages three ops (`write`, `delete_all`, `write`) and reads committed rows **and** the checkpoint out of band through a *fresh* handle: all unchanged. After `commit`, all changed, with the queued `DeleteAll` replayed in its journal position — which a collapsed delta plus a flag would have had to decide at stage time. Red with `probe_write` applying straight to committed rows (`left: Some(9)`, `right: None`); green reverted. No literal position: the checkpoint is compared against the two positions the test handed the store. | `crates/happenstance-testkit/tests/projection_conformance_buffering.rs:73`; store at `…/projection_mutation_coverage/buffering.rs:252` |
| **AC-002** | **satisfied** | One `projection_store_conformance!` invocation, **no rule name in the file**, emitting one `#[tokio::test]` per rule from the single enumeration: 17 passed / 0 failed. Asserted on `RuleOutcome`-derived `Verdict` values rather than stdout by `the_second_batch_shape_answers_every_rule_with_a_pass` — outcome set equals `all_projection_rules()` in enumeration order and exactly once each, every verdict `Passed`, `declines` empty. The skip machinery is consumed unchanged and still exercised in the same run by the reference fixture's two `RuleOutcome::skip_line` outputs; no skip string is re-spelled locally. | `…/tests/projection_conformance_buffering.rs:52`; meta-test at `…/tests/projection_mutation_coverage.rs:1446` |
| **AC-003** | **satisfied** | Exactly one new `Declared` row, hand-written in the same `&[Declared]` literal as every other: `kind: Kind::ConformantVariant`, `fails: &[]`, `expect: &[]`, and a provenance naming the Workers `SqlStorage` / Neon-over-one-shot-HTTP shape with the PS-4 and §4.11 citations that make it a claim rather than a plausible sentence. Nothing in the binary derives a row from an observed outcome. Red first: enumerated without a row, `projection_mutant_registry_is_exhaustive` said so by name. | `…/tests/projection_mutation_coverage.rs:802-830` (row), `:863` (enumeration) |
| **AC-004** | **satisfied** | `projection_conformant_variants_pass_everything` now asserts **both** halves: no declared variant rejected by any rule (CF-6's message unchanged in substance), and every rule in the enumeration answered `Passed` by at least one variant. The second half was **red before this story's store existed** — `batch_reads_reflect_pending_writes` had never been accepted by anything legally different from the reference store — which is the non-vacuity demonstration, run in the direction that leaves no uncommitted deletion behind. The empty-set guard is still there above it. | `…/tests/projection_mutation_coverage.rs:1306`, second assertion at `:1350-1361`, guard at `:1317-1323` |
| **AC-005** | **satisfied** | One `cargo xtask ci`, clean tree, `all checks passed`. Both fixtures named in its single `tests` step: `Running tests\projection_conformance.rs` and `Running tests\projection_conformance_buffering.rs`. The new target is ordinary — no `#[ignore]`, no `required-features`, no new feature, `Cargo.toml` byte-identical — and the mandatory wasm32 `--tests` check over the widened target set stayed green with **no new gate step**. | `…/tests/projection_conformance_buffering.rs:31` (target discipline); `xtask/src/main.rs:231-243` (the step that needed no change) |
| **AC-006** | **satisfied** | One source file holds `BufferingProjectionStore`, `BufferingBatch`, its error and `BufferingProjectionFixture`; two consumers reach it by the **same `#[path]` string**. `rg 'struct Buffering' crates/happenstance-testkit` returns two hits, both in that file. The `ProjectionSubject` registration sits in `variants.rs` — the trait is the mutant binary's, and the store must compile in a target that cannot see it — which is what keeps the *store* a single definition rather than two that drift. | `…/tests/projection_mutation_coverage.rs:84-85` and `…/tests/projection_conformance_buffering.rs:47-48`; registration at `…/projection_mutation_coverage/variants.rs:103` |
| **AC-007** | **satisfied** | `two-shape-observation.md`, written while the run was live. Not a shrug: fourteen rules agreed, **two differ** (`failed_commit_leaves_both_unchanged` and `refused_reset_changes_nothing` run against the buffering fixture and report `RuleOutcome::Skipped` against the reference one, because the fixtures' `COMMIT_FAULT` / `RESET_REFUSAL` declarations differ), and the note names the framing error it found. Discipline half, as a diff claim: 0 rules added, 0 rules edited, 0 new capabilities, 0 files under `crates/happenstance-core/src/`, 0 new dependencies or features, 0 edits to `spec/SPECIFICATION.md` — hence no CF-29 obligation incurred. No pass rate quoted anywhere. | `.bklg/…/buffering-conformant-variant/two-shape-observation.md` |

**Deferred: nothing. Blocked: nothing.**

### The finding this story was built to be able to make, and made

**`MemoryProjectionStore` is already a deferred write set.** Its `begin` returns
an owned batch holding a delta and its `commit` applies it
(`crates/happenstance-core/src/projection_memory.rs:280-330`); it holds no
handle, transaction or lock across the batch. The spec for this story — and
§4.11's own sentence assigning CF-5's projection variant — both describe it as
*apply-on-write*. That is a framing error in the plan, not in the code, and it
narrows what this pair of shapes spans.

The response was the one EC-001 and the CF-6 discipline require: **report it, do
not absorb it.** No rule was loosened, no capability was minted to gate one, and
the variant was not reshaped toward the reference store. It was built to the
spec's literal description — `begin` holds nothing, every probe write **appends
an op**, `commit` replays them — which is a real structural difference (an
ordered journal collapsed by nothing, versus a delta collapsed at stage time) and
is exactly the statement list a Workers or Neon adapter queues.

What the pair therefore proves is narrower than PS-2's axis and still worth
having: **no projection rule assumes a lock is held, a transaction is open or a
connection exists between `begin` and `commit`**, which is the sentence PS-4's
`Rejects:` clause asks for. The full account, including the third shape that
*does* hold a connection (`MutantStore`, driven through all sixteen rules by the
mutant harness rather than by the conformance macro), is in
`two-shape-observation.md` and is what `ps3-batch-shape-finding` composes.

### Two residuals, stated rather than left to be inferred

**The comparison is asymmetric by two rules.** The buffering run asserts sixteen
rules; the reference run asserts fourteen and reports two honest skips. Calling
that "agreed everywhere" would overstate it, so it is classified rather than
smoothed. The asymmetry is in the *fixtures'* capability declarations, not in the
rules — and it is load-bearing: had the buffering fixture also declined, those
two rules would be unexecuted against every conformant variant, which is the hole
AC-004's new assertion refuses.

**Every store the projection rules have met is in this process.** Both ends of
the batch-representation axis now have a row, and a third shape models a
checked-out connection, but nothing here crosses a transaction manager, a pool
under contention or a server that can refuse half a request. That is why PS-2
asks for two *adapters*, and it is why nothing in this story clears PS-2's
`[FROZEN]` bar.

### What is explicitly not claimed

PS-2 is **not** cleared. Its bar is two adapters at opposite ends of the
batch-shape axis and its `Rejects:` clause names the two-instrument monoculture
verbatim (`spec/SPECIFICATION.md:4760-4775`); two testkit instruments are not two
adapters. No verdict on the `unstable-projection` exposure is made — that is
`publication-and-positioning`'s (HS-P0016). No maturity marker was touched, no
`.kb/` atom written, and no clause edited. No pass rate is quoted over the rule
or mutant set (ADR-0010).
