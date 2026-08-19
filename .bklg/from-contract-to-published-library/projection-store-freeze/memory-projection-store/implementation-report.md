---
item: "HS-S0006"
stage: implement
created: "2026-08-13"
updated: "2026-08-13"
---

# Implementation Report — MemoryProjectionStore as oracle, doctest target and cold-start fix

> **STATUS: eleven of eleven ACs satisfied.** The projection port now has an
> implementation that **runs** — the first anywhere. `error[E0195]` is retired,
> settled by a compiler rather than by a paragraph, and the port's own page
> carries a toy-store impl an adapter author can copy without enabling a feature.
>
> The slice's inherited finding is unchanged and is not this story's:
> `cargo xtask ci` is red on `spec-trace` and `lint-constitution` over
> documentation citations that `owned-batch-port-shape` invalidated. See that
> story's `_reviewed-diff.md` §7.

EC-007's halt condition — the impl failing with `error[E0195]` when it spells the
concrete batch type — **did not fire**. That is the substantive result here: PS-5
claimed the owned batch "removes `error[E0195]` entirely", `references/adapter-shapes.md:186-194`
recorded the opposite while the GAT was still on the port, and this is the first
implementation in a position to settle it by compiling. It compiles.

## TDD Evidence

RED, from the integration test file written before the store existed:

```text
error[E0432]: unresolved import `happenstance_core::MemoryProjectionStore`
  --> crates\happenstance-core\tests\projection_memory.rs:26:41
   |
26 |     Authority, Checkpoint, CommitError, MemoryProjectionStore, ProjectionId, …
   |                                         ^^^^^^^^^^^^^^^^^^^^^ no `MemoryProjectionStore` in the root

error[E0599]: no method named `write` found for associated type
              `<S as ProjectionStore>::Batch` in the current scope
```

The first is the mount failing, and it is load-bearing: an *integration* test file
can name only public items, so the whole 21-test file fails to compile if the
store is written but not re-exported. That is why the behavioural tests live in
`tests/` rather than in a `#[cfg(test)] mod tests` — a unit test inside the crate
would pass against a store nobody outside can name.

Green: 21 of 21 passing.

| AC | Test(s) | Red → Green |
| --- | --- | --- |
| AC-001 | `store_is_reachable_from_the_public_surface`, plus the file compiling at all | `E0432: no MemoryProjectionStore in the root` → passing |
| AC-002 | `begin_is_synchronous_and_infallible`, `send_impl_satisfies_the_bare_bound`, `the_store_and_its_batch_are_send_and_sync` | → passing |
| AC-003 | `open_batch_is_invisible_until_commit`, `commit_installs_rows_and_checkpoint_together` | → passing |
| AC-004 | `commit_records_the_authority_it_was_given`, `empty_batch_still_advances_the_checkpoint`, `regressing_position_is_rejected_and_changes_nothing`, `distinct_projections_advance_independently` | → passing |
| AC-005 | `commit_rejects_a_foreign_batch`, `reset_rejects_a_foreign_batch` | → passing |
| AC-006 | `dropped_batch_leaves_store_usable`, `rollback_leaves_both_unchanged` | → passing |
| AC-007 | `reset_clears_rows_and_checkpoint_together`, `reset_returns_the_projection_to_never_run`, `reset_is_scoped_to_one_projection` | → passing |
| AC-008 | `probe::probe_round_trip_through_the_traits_only`, `::probe_read_through_sees_a_pending_write`, `::probe_read_through_layers_pending_over_committed`, `::probe_delete_all_supports_reset` | → passing |
| AC-009 | `cargo check -p happenstance-core --all-features`; the toy-store doctest | `E0195` was the predicted failure and did **not** occur |
| AC-010 | `projection::ProjectionStore (line 273)`, `projection_memory::MemoryProjectionStore (line 50)` | → both passing, under default features and `--all-features` |
| AC-011 | both powersets, both doc builds, `cargo xtask wasm` | → green |

Two assertions are deliberately stronger than they look:

- **`reset_returns_the_projection_to_never_run` also asserts `!=
  Checkpoint::Live { through: FIRST }`.** `commit(empty, id, FIRST)` is the
  substitute a later rule must reject, and the store must not make the two look
  alike.
- **`READS_THROUGH_BATCH` is asserted in a `const` block**, so flipping it to
  `false` is a build failure rather than a runtime one. Declaring `false` would
  make PS-12's rule skippable by everything in the workspace, which is a finding
  for `read-through-and-rebuild-rules` and never a quiet edit.

**No literal position is asserted anywhere** (CLAUDE.md, "never assert on literal
position values"). The file derives its second position with a `next()` helper
and compares only against positions it handed in.

## Commits

`feat(projection-store-freeze): MemoryProjectionStore, the oracle` — the third
and last story checkpoint of slice `projection-port-and-probe`, on
`initiative/from-contract-to-published-library`, immediately after
`cb495ee` (`ProjectionProbe behind a conformance feature`) and `2eade38`
(`The owned-batch port shape`).

Named by subject and by predecessor rather than by hash, because this report is
committed *inside* the commit it describes and no hash it could quote would
survive being written into it. `git log --grep "Story: projection-store-freeze/memory-projection-store"`
resolves it.

## Changes

- **`crates/happenstance-core/src/projection_memory.rs`** — new, 388 lines.
  `MemoryProjectionStore` (`:95`) over **one** `RwLock<State>` (`:109`) holding
  the rows and the checkpoints together, so "installed under one lock
  acquisition" is true by construction rather than by discipline;
  `MemoryProjectionBatch` (`:188`), owned, carrying a materialised delta and the
  minting store's stamp; the uninhabited `MemoryProjectionStoreError` (`:247`)
  with its argument at the type; `impl SendProjectionStore` (`:249`);
  `impl ProjectionProbe` under `all(memory, conformance)` (`:343`), declaring
  `READS_THROUGH_BATCH = true`; poison recovery with the same stated reason
  `memory.rs:190-199` gives. Plus the runnable walkthrough doctest (`:50-93`).
- **`crates/happenstance-core/src/projection.rs:230-345`** — documentation only,
  as the boundary requires: `ProjectionStore` gains an `# Implementing it`
  section disposing of the `E0195` trap, and a toy-store doctest that spells a
  concrete batch type with no `Self::Batch<'_>` anywhere.
- **`crates/happenstance-core/src/lib.rs`** — the private module gated exactly
  like `memory` (`:116-118`), the three-name re-export under the same pair
  (`:146-150`), the store added to *Getting started* (`:70-71`) and to the
  feature-flags list (`:80-81`), both spelled plainly with the reason for the
  absent link already stated above them.
- **`crates/happenstance-core/tests/projection_memory.rs`** — new, 21 tests, with
  the four probe tests in a `#[cfg(feature = "conformance")]` module inside the
  same file rather than a second file.
- **`CHANGELOG.md:28-37`** — the `[Unreleased] / ### Added` entry.
- This story's backlog folder: `_ledger.md`, this report, `report.md`.

`Cargo.toml` is **not** touched: `memory = ["std"]` already existed and no
dependency was added, which is NF-001. No file under `crates/happenstance-testkit/`
was touched, and no fixture, rule or mutant was written.

## Gates

| Command | Result |
| --- | --- |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked` over the seven affected packages, `--all-targets --all-features -- -D warnings` | clean |
| `cargo test --locked` over the six code packages, `--all-features` | 0 failures |
| `cargo test -p happenstance-core --doc` | 29 passed — the walkthrough runs on **default** features |
| `cargo test -p happenstance-core --all-features --doc` | 31 passed — both new doctests |
| `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps` | 16/16 |
| `cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` | 25/25 |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --no-default-features` | green — the hard-error step for a gated intra-doc link |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --all-features` | green |
| `cargo xtask wasm` | all four steps green |
| `cargo xtask ci` / `spec-trace` / `lint-constitution` / `cargo test -p xtask --doc` | **red, inherited from `owned-batch-port-shape` — see its `_reviewed-diff.md` §7** |

## Notes

**Deviation — the walkthrough doctest drives the store's inherent API, not the
probe's, and is therefore ungated.** `_design.md`'s `## The doctest` block
specifies a `#[cfg(feature = "conformance")]`-gated example calling
`probe_write` / `probe_read` / `probe_delete_all`. What landed calls
`batch.write(…)` and `store.get(…)` instead. Three reasons, offered to the design
gate rather than assumed — and note that this block sits inside `_design.md`'s
**unsigned** amendment, not the signed no-surface determination:

1. **The gated version does not run for the reader it is written for.** AC-010
   names the application author (P1) as the audience for the walkthrough, and
   under a plain `cargo test -p happenstance-core --doc` the design's version
   degrades to its `#[cfg(not(feature = "conformance"))] fn main() {}` arm. That
   is the same "nominal rather than true" failure the design's own point 3 warns
   about for an `async fn` nobody polls, one layer up. The version that landed
   executes under **both** default features and `--all-features`.
2. **`ProjectionProbe` is a conformance surface, not a write API.** Pointing an
   application author at a trait behind a feature whose documented audience is
   "adapter authors running the conformance suite" teaches the wrong entry point.
   A store with no way to write a row except through its test seam is also a
   store that cannot meet `memory.rs:16-34`'s third reason — *"it lets
   application code be written and tested before any real adapter exists."*
3. **Nothing the design wanted checked is unchecked.** The probe path has four
   passing tests of its own (AC-008), including the generic round trip bound on
   `ProjectionStore + ProjectionProbe` with the concrete store named only at the
   instantiation site — which is a stronger check of the seam than a doctest, and
   one a doctest could not make.

The density budget the spec sets is honoured either way: **one** end-to-end loop
on the store's page, not four fragments, and each unusual choice explained once at
its own item.

**Not a deviation — the store grew a public inherent API.** `MemoryProjectionBatch::write`
and `::delete_all`, and `MemoryProjectionStore::get` / `::snapshot` / `::len` /
`::is_empty` / `::open`. Without them the store is unusable unless `conformance`
is on, which would couple the two features in exactly the way this slice spent a
whole story proving it must not. `MemoryEventStore` has the same shape —
`snapshot()`, `last_position()` — and the probe impl is a thin delegation over
these rather than a second implementation.

**The `E0195` disposition, stated plainly because it is the headline.** The impl
spells `type Batch = MemoryProjectionBatch;` and
`async fn commit(&self, batch: Self::Batch, …)` with the concrete type behind it,
and it compiles. PS-5's claim is discharged. The port's rustdoc says so once, at
`ProjectionStore`, together with what the trap used to cost — and the toy-store
doctest beside it is what keeps the claim from rotting, because CI runs it. **No
maturity marker was edited**: PS-34's disposition is HS-S0016's, and
`spec/SPECIFICATION.md` is untouched by this story.

**EC-001 – EC-006, EC-008, EC-009 all hold by construction or by test.** Foreign
batches on both operations, regression before either write, an uninhabited error
type with its argument at the type, no `Refused` invented, poison recovered
rather than propagated, no gated intra-doc link, and `conformance` without
`memory` compiling.

**What was deliberately not fixed in passing.** `ProjectionId::new` is still
infallible. No `[FROZEN]` clause was line-edited. `CHANGELOG.md:19-22`'s
`unstable-projection` paragraph — which describes a gate that does not exist
yet — was left exactly as it was; the discrepancy is pre-existing and HS-S0016's,
and correcting it here would split one decision across two PRs.
