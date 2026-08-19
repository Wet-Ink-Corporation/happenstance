---
item: "HS-S0012"
stage: report
created: "2026-08-14"
updated: "2026-08-14"
---

# Report — Read-your-writes, chunk-size invariance and rebuild authority

## Findings Ledger

**Eight of eight ACs satisfied. Nothing deferred, nothing stubbed, no clause text
edited, and `crates/happenstance-core/**` untouched.** Three rules, three
mutants, one conformant variant, and the projection registry's first CF-5
positive control.

| AC | Result | What proves it | Where it is mounted |
| -- | ------ | -------------- | ------------------- |
| **AC-001** | **satisfied** | `batch_reads_reflect_pending_writes` asserts `probe_read_through(&batch, k) == Some(v)` for the value written, **before** commit — so a `None`-vs-`Some` technicality cannot satisfy it. Green against the oracle on both host harnesses; red against `CommittedReadBatchStore`, pinned at `"must let an open batch see"`. | `for_each_projection_store_rule!`, `crates/happenstance-testkit/src/projection.rs:1878` |
| **AC-002** | **satisfied** | `rebuild_is_chunk_size_invariant` replays `a, a, b, a, b, a` at chunk sizes **1, 3 and whole-log** against **three isolated stores** (`open()` per run), every step a read-modify-write **through the batch**, comparing read models across runs. Red against `CommittedReadBatchStore` **and** `FirstWriteWinsBatchStore`. The body increments; a plain `set` would be chunk-insensitive by construction and is what this row exists to forbid. | `…/projection.rs:1879` |
| **AC-003** | **satisfied** | `rebuilding_is_distinguishable_from_live` asserts the `Checkpoint::Rebuilding` **variant** after each of two rebuilding commits and `Checkpoint::Live` after the live one, comparing no `through` value anywhere, and asserting nothing immediately after the opening reset. Red against `LiveOnlyCheckpointStore`, which preserves `NeverRun` for an unseen id and so does not also trip the reset family's rule. | `…/projection.rs:1880` |
| **AC-004** | **satisfied** | Each new rule has at least one registered wrong store with an exact `fails` set and non-empty provenance naming the real mistake — the round-trip batch `get`, the `entry().or_insert(…)` dedup buffer, the single-position-field rebuild. All four projection exactness meta-tests green. **No pass rate is quoted anywhere**; nothing in the binary divides. | `crates/happenstance-testkit/tests/projection_mutation_coverage.rs:719, :747, :765` |
| **AC-005** | **satisfied** | `NoBatchReadStore` — `READS_THROUGH_BATCH = false`, `probe_read_through` `unimplemented!()` — is registered as `Kind::ConformantVariant` with `fails: &[]` and passes every projection rule. The new positive control `projection_conformant_variants_pass_everything` drives it through the whole enumeration and **asserts its own non-vacuity** (at least one variant driven), so it cannot pass over an empty set. `Kind::ConformantVariant`'s `#[expect(dead_code)]` came off in the same change — the trip-wire fired, which is the mechanism working. | `…/tests/projection_mutation_coverage/variants.rs:57`; row at `…/projection_mutation_coverage.rs:784` |
| **AC-006** | **satisfied** | `a_batch_with_no_read_path_is_reported_as_a_skip` asserts the skip set against the declining store is **exactly two rules** in enumeration order, and that each skip's `capability` **and** `reason` equal the exported `NO_BATCH_READ_PATH` (`"ProjectionProbe::READS_THROUGH_BATCH"`) and `NO_BATCH_READ_PATH_REASON` — compared against the constants, never literals repeated in the test, and asserted on the `RuleOutcome` value rather than on stdout. | `…/projection_mutation_coverage.rs:1315`; consts at `crates/happenstance-testkit/src/contract.rs:836,852` |
| **AC-007** | **satisfied** | All three names in the one enumeration → tokio, blocking and `wasm32` harnesses each gained three tests with no per-harness list (17 passed per host harness; the wasm harness type-checks in `cargo xtask ci`'s mandatory step). Mounted at all four points: rules module, enumeration, three harnesses, and the mutant registry's `REGISTRY` + `for_each_projection_mutant!`. `no_orphan_projection_rules` makes an unmounted rule a failure. | `…/projection.rs:1878-1880`; `…/tests/projection_conformance_wasm.rs` |
| **AC-008** | **satisfied** (criterion amended 2026-08-15; see below) | `git diff -- spec/` is the **generated §7.1–§7.2 region only** (four `†` removed for PS-12, PS-13, PS-14, PS-24 by `spec-trace --write`) — no clause text, no maturity marker, no rule citation. No `ProjectionStore` / `ProjectionProbe` / `Checkpoint` / `Authority` / `MemoryProjectionStore` definition changed. `Capability`, `RuleOutcome`, the `Declared` shape and the three emitters are unchanged; the two new constants sit beside `NO_STORE_LIMITS` and add no new skip type or line shape. No assertion compares a position or a `through` value to a literal. `cargo xtask spec-trace` green; the event-store suite including `GappedPositionFixture` still green. **The three gate-compelled exits are now disclosed** in `implementation-report.md`, *Compelled exits from the PR boundary*. | `CHANGELOG.md`; `spec/SPECIFICATION.md` generated region; `standards/rust/{11,13,40,41}-*.md` line numbers |

**Deferred: nothing.** **Blocked: nothing.**

**AC-006's authority, so the row is not read as a near-miss.** Who writes the
reason a declined batch-read skip carries is decided by the signed-off
`_design.md`, and it is decided the way the code does it: the capability table
(`_design.md:280`) assigns `READS_THROUGH_BATCH`'s reason to the **testkit**, and
note 1 (`:306-310`) calls it *"the one place the projection family adds a
testkit-written reason … the same argument `NO_CEILING_REASON` already won"*.
`NO_BATCH_READ_PATH_REASON` is that decision executed, so the AC is met rather
than excused. The AC's phrase *"the fixture's own stated reason"* predates the
port landing the switch as a probe-level `bool` and describes the family's other
two capabilities. The residual **port-shape** question — should the probe const
carry a reason at all — is routed to `unstable-projection-gate-and-clause-disposition`
and recorded in `_slices.md`, *Run 6's record*, rather than parked inside a
green DoD row.

**Three edits landed outside this story's fenced boundary block, and this report
was silent on all three until 2026-08-15.** Each is compelled by a gate step —
`spec-trace --write`'s generated region, CF-29's changelog lint,
`lint-constitution`'s line-number citations — and each is now named in the
boundary block, in AC-008's criterion, and in the implementation report's table.
The fault was the silence, not the exits: the pre-amendment criterion (*"`cargo
xtask spec-trace` … unaffected"*) could not describe an exit the gate demands, so
nothing in the record had a place to put them.

**One residual, stated rather than hidden**, because a skip that reads as
coverage is worse than a red rule. An adapter declaring
`READS_THROUGH_BATCH = false` gets **two** reported skips, and what they do not
say is that the adapter is safe for read-modify-write projections: chunk-size
invariance is *unverified* for a write-behind store, because such a store has no
read path for a projection's `apply` to use in the first place. The sentence is
in the rule's own doc comment and in `NO_BATCH_READ_PATH_REASON`, so it reaches a
CI log rather than only a spec.

**Two observations a reviewer should not have to re-derive.** First, the
alternative to gating `rebuild_is_chunk_size_invariant` — running a *blind* write
sequence for declining adapters so the rule reports `Ran` — was considered and
rejected: blind writes are chunk-size invariant by construction, which is a rule
no adapter can fail arriving through the back door. Second, `NoBatchReadStore` is
deliberately **not** the buffering replay-at-commit variant `buffering-conformant-variant`
owes; it is the minimal declining instrument, and its doc says so, so that this
story cannot be read as having claimed the second batch shape one slice early.
