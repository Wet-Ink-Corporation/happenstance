---
item: "HS-S0006"
stage: report
created: "2026-08-13"
updated: "2026-08-13"
---

# Report — MemoryProjectionStore as oracle, doctest target and cold-start fix

## Findings Ledger

**Eleven of eleven ACs satisfied, each by real reachable behaviour with a real
test.** Twenty-one integration tests, two runnable doctests, both feature
powersets and all four `wasm32` steps.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — mounted at both halves, announced in the changelog | **Met** | The store, its batch and its error re-exported at `lib.rs:146-150` under the `memory` gate pair; `memory = ["std"]` unchanged and no dependency added. The mount is proven by construction: `tests/projection_memory.rs` is an integration file, so all 21 tests fail to compile if the re-export is missing — which is the RED transcript this story started from. `CHANGELOG.md:28-37` |
| **AC-002** — one impl, two flavours; `begin` costs nothing | **Met** | `impl SendProjectionStore` with the bare flavour derived; no `async_trait` anywhere. `begin_is_synchronous_and_infallible` binds the batch with no `.await` and no `?`; `send_impl_satisfies_the_bare_bound`; `the_store_and_its_batch_are_send_and_sync` |
| **AC-003** — atomicity, the invariant the store exists to demonstrate | **Met** | One `RwLock<State>` holding rows *and* checkpoints, taken once in `commit` with no return between the two writes. `open_batch_is_invisible_until_commit`, `commit_installs_rows_and_checkpoint_together` |
| **AC-004** — authority, position-considered, regression, per-id scope | **Met** | Four tests, including one asserting the exact `CheckpointRegression { current, attempted }` value *and* that nothing moved. No literal position anywhere — the file derives its second position from its first |
| **AC-005** — the foreign batch the owned type did not make unrepresentable | **Met** | Per-*instance* stamp compared before any guard is taken. `commit_rejects_a_foreign_batch`, `reset_rejects_a_foreign_batch`, each asserting both the error arm and that the receiving store is untouched |
| **AC-006** — `rollback`, and the half a store actually fails | **Met** | `dropped_batch_leaves_store_usable` drops a written batch **bare**, then opens and commits a second batch on the same handle and asserts it took effect — the half a reviewer's probe once found a store failing by answering `Busy` forever. `rollback_leaves_both_unchanged` |
| **AC-007** — `reset` as one scoped unit | **Met** | The checkpoint key is *removed* rather than set to a sentinel. `reset_clears_rows_and_checkpoint_together`, `reset_returns_the_projection_to_never_run` (which also asserts `!= Live { through: FIRST }`, so `commit(empty, id, FIRST)` cannot masquerade as it), `reset_is_scoped_to_one_projection` |
| **AC-008** — the probe seam, and an honest `READS_THROUGH_BATCH` | **Met** | Four gated tests. The round trip is bound `S: ProjectionStore + ProjectionProbe` with the concrete store named only at instantiation. `READS_THROUGH_BATCH = true` is asserted in a **`const` block**, so flipping it is a build failure rather than a runtime one |
| **AC-009** — the `E0195` disposition | **Met, and it is the headline** | The impl spells `type Batch = MemoryProjectionBatch;` and `commit(&self, batch: Self::Batch, …)` with the concrete type behind it, and **it compiles**. EC-007's halt did not fire. Documented once, at `ProjectionStore`, with PS-36's `Batch: Send` transitivity at `type Batch` |
| **AC-010** — two doctests, two jobs | **Met, with one recorded deviation** | The port's page carries a toy-store impl with no dependency on `memory`; the store's page carries the runnable walkthrough asserting rows **and** checkpoint. Both execute — 29 doctests on default features, 31 on `--all-features`; neither is `no_run` or `ignore`. Deviation below |
| **AC-011** — every configuration, and docs.rs | **Met** | 16 host and 25 `wasm32` powerset combinations; `--no-default-features` doc build green (the hard-error step); `--all-features` doc build green; all four `cargo xtask wasm` steps; both new gates carry `#[cfg_attr(docsrs, doc(cfg(…)))]` |

### The one deviation, offered to the design gate rather than assumed

`_design.md`'s `## The doctest` block specifies a `conformance`-gated walkthrough
driving `ProjectionProbe`'s methods. What landed drives the store's **inherent**
`batch.write` / `store.get` and is ungated. The block sits inside `_design.md`'s
**unsigned** amendment, not the signed no-surface determination, and the three
reasons are:

1. **The gated version does not run for the reader it is written for.** AC-010
   names the application author as its audience, and under a plain
   `cargo test -p happenstance-core --doc` the design's version degrades to its
   `#[cfg(not(feature = "conformance"))] fn main() {}` arm — the same "nominal
   rather than true" failure the design's own point 3 warns about for an
   `async fn` nobody polls.
2. **`ProjectionProbe` is a conformance surface, not a write API.** A store with
   no way to write a row except through its test seam cannot meet
   `memory.rs:16-34`'s third reason for existing — *"it lets application code be
   written and tested before any real adapter exists."*
3. **Nothing the design wanted checked is unchecked.** The probe path has four
   passing tests of its own, including a generic round trip that is a stronger
   check of the seam than a doctest could be.

**Nothing was stubbed, skipped or fixture-pinned.** Every body in the new module
is real; there is no `todo!()`, no `unimplemented!()` and no `ignore`d test.

### Inherited, and not this story's

`cargo xtask ci` is red on `spec-trace` (7 problems) and `lint-constitution`
(16 problems, plus 3 failing compiled examples). All of it comes from
`owned-batch-port-shape`'s ADR-0017-mandated file move and the growth of
`projection.rs`; the inventory with the exact re-points is that story's
`_reviewed-diff.md` §7. Nothing in this story contributes to it, and every gate
step this story owns is green.

## Acceptance

**Recommended: accept.**

The projection port now has an implementation that runs — the first one, and the
thing whose absence made every projection clause a guess. Three claims are worth
separating out, because they are what a reviewer should check rather than take:

**The `E0195` trap is retired, and by a compiler.** PS-5 claimed the owned batch
"removes `error[E0195]` entirely"; `references/adapter-shapes.md:186-194` recorded
the opposite while the GAT was still on the port. This is the first implementation
in a position to settle it, it spells its concrete batch type in `commit`, and it
compiles. The toy-store doctest on `ProjectionStore`'s own page is what keeps the
claim from rotting back, because CI runs it and it deliberately names no
feature-gated item.

**The oracle is obviously correct rather than fast, and that is a design
constraint met rather than a description.** One lock over one struct holding both
halves, taken once in `commit`, with the regression check inside the same guard
and before either write — so there is no path that mutates and then fails. When a
conformance rule fails, this store is presumed right; its correctness had to be
readable in one sitting, and it is.

**The two features stay independent.** `memory` without `conformance`,
`conformance` without `memory`, and `--no-default-features` all compile, across
16 host and 25 `wasm32` combinations. That is the check the whole of
`projection-probe-conformance-feature` argued for, now exercised by a store that
actually implements both surfaces.

What is not green is inherited and named above. It is a planning decision, not
rework here.

## Knowledge Harvest

**An integration test file is the library's mount check.** `tests/` can name only
public items, so "the store is exported" needs no separate assertion — the 21
tests failing to compile *is* the assertion, and the first RED transcript was
literally `no MemoryProjectionStore in the root`. A `#[cfg(test)] mod tests`
inside the crate would have passed against a store nobody outside could name.
This generalises: for a library, put the behavioural tests where only the public
surface is visible and the mount checks itself.

**Assert a capability constant in a `const` block, not with `assert!`.**
`READS_THROUGH_BATCH = true` is load-bearing — declaring `false` on both batch
shapes would make PS-12's rule skippable by everything in the workspace and the
pair vacuous. In a `const` block, flipping it is a build failure; in a runtime
`assert!`, it is a test failure someone can `#[ignore]`. Clippy's
`assertions_on_constants` pointed straight at the better form.

**A reference implementation needs an inherent API, or it is only a test
fixture.** The probe is behind `conformance`, whose documented audience is
adapter authors running the suite. If the *only* way to write a row is
`probe_write`, then an application author cannot use the store at all without
enabling a feature built for somebody else — which quietly couples two features
the slice-mate spent a whole story proving must stay independent. The probe impl
is now a thin delegation over the inherent API, which is also what makes it
credible as a demonstration rather than a special case.

**Removing a key is how you return to "never run".** Storing a sentinel position
would reintroduce exactly the `(None, true)` ambiguity the three-variant
`Checkpoint` exists to forbid, one layer down in the storage rather than in the
type. `reset_returns_the_projection_to_never_run` asserts both the positive
(`NeverRun`) and the negative (`!= Live { through: FIRST }`), because
`commit(empty, id, FIRST)` is the substitute a later rule has to reject and the
oracle must not make the two look alike.
