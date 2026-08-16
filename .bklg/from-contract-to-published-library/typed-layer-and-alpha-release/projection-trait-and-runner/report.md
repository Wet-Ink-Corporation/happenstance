---
item: "HS-S0026"
stage: report
created: "2026-08-16"
updated: "2026-08-16"
---

# Report — The application-facing Projection trait and its streaming runner

## Findings Ledger

**Outcome: twelve of twelve ACs satisfied. Nothing deferred, nothing blocked.**

The mount point is `crates/happenstance/src/lib.rs` — composition root 1, and the only render
path this library has. `mod runner;` is declared at `:192` behind
`#[cfg(feature = "unstable-projection")]`, and the four items are re-exported at `:233-235`
with their `doc_cfg` badge, beside the surviving `pub use happenstance_core::*;` at `:237`.
The co-equal mount is `crates/happenstance/Cargo.toml`'s `[features]`, which gains
`unstable-projection = ["happenstance-core/unstable-projection", "dep:futures-core"]` — off by
default, and the first manifest in the workspace to make `CHANGELOG.md:19-22`'s existing claim
true rather than aspirational.

| AC | result | proved by | notes |
| --- | --- | --- | --- |
| AC-001 | satisfied | `projection_runner.rs::derived_query_matches_event_types_and_scope` | The query is derived from `EVENT_TYPES` + `scope()` through the *same* `derive_query` a decision model uses (`runner.rs:420`). One read per run, asserted. `Store` is an associated type (`runner.rs:67`), so a two-store projection is a compile error rather than a warning. |
| AC-002 | satisfied | `::apply_receives_decoded_domain_events` + `doc_surface.rs::no_second_decode_path` | `apply` receives the application's own enum; the fixture's `apply` takes `Stock`, so a `Bytes` signature would not compile against it. The decode goes through `crate::codec::decode_event` and nothing else. |
| AC-003 | satisfied | `::read_model_and_checkpoint_commit_together`, **discriminated by** `::checkpoint_without_rows_is_rejected` | One `begin`, one `commit(batch, id, last_applied, Live)`, no `set_checkpoint`. The wrong runner at `projection_runner.rs:412` commits the checkpoint on an empty batch and **fails** the same oracle — asserted, so the oracle is not decorative. |
| AC-004 | satisfied | `::first_run_starts_from_the_beginning`, `::resume_advances_past_the_checkpoint`, `::tolerates_gapped_positions`, `::a_checkpoint_at_the_last_position_reports_exhausted_key_space` | `NeverRun` means no `from` anchor at all, not position zero. Resume is `through.next()`, whose `None` arm is `ProjectionError::KeySpaceExhausted`, never an `unwrap`. The gap test runs against `GappyMemoryStore` at stride 7 and names no literal position. **EC-005 now has an executed path** (added after review, which found it the only row of the EC table with none): a checkpoint committed at the last representable position through the real `MemoryProjectionStore` — no double stands in for a store — and the arm, its `position()` and its empty `progress()` are asserted. Under `saturating_add` the run would return `Ok` instead, so `event.rs`'s `checked_add` reasoning has an instrument on this side of the port too. |
| AC-005 | satisfied | `::commits_before_the_stream_ends` | The stream is pinned and pulled item by item; the only buffer is the `chunk` window. `PullWatching` records the committed row count at each pull, and a `collect`ing runner records zero everywhere and fails. |
| AC-006 | satisfied | `::decode_failure_names_its_position_and_rolls_back` + `doc_surface.rs::runner_prints_nothing` | `ProjectionError::Decode` carries the position and the concrete `CodecError` as `#[source]`; the chunk is discarded through `rollback`; `error.progress()` returns the partial `Progressed`, verified at chunk 8 (nothing durable) and chunk 2 (checkpoint at the last *good* position). The runner writes nothing to a console. |
| AC-007 | satisfied | the implementation report's *PS-33 / PS-27 / PS-30 evidence* section, plus an empty `git status --porcelain -- spec/ crates/happenstance-core/src` | Each count is a fact about the tree, produced by a command whose output is quoted. **No pump exists in the contract crate and none was written.** No `SkipPolicy`. No fan-out runner. No maturity marker moved. |
| AC-008 | satisfied | `manifest_contract.rs::unstable_projection_is_declared_off_by_default` + `cargo hack check --feature-powerset -p happenstance` | Declared, absent from `default`, forwarding stated in a manifest comment, and gating a real item. The powerset is green and was run by hand, because `--fast` drops it. |
| AC-009 | satisfied | `doc_surface.rs::crate_root_renders_the_projection_surface` + `docs_composition.rs` (three tests) + `doc_budget.rs` + `src/tests.rs::doc_density_budget_holds` | The fifth bullet became a link **in place**, still fifth and last; the Features table gained one recessive row; the `*(planned)*` marker and its explanatory paragraph are gone. Every density number holds: 122 module-doc lines of 130, prose ≤ 80, fences ≤ 72, longest new identifier 15. |
| AC-010 | satisfied | `manifest_contract.rs::docs_rs_metadata_is_declared`, `docs_composition.rs::docsrs_metadata_is_present`, three doc builds | `doc_cfg` badge on the module and on the re-exports; all three rustdoc configurations green, including nightly `--cfg docsrs -D warnings`. |
| AC-011 | satisfied | wasm32 `cargo check`, `doc_surface.rs::no_contract_name_is_shadowed`, `src/tests.rs::shadowing::same_projection_id`, empty `git status` under `crates/happenstance-core/src` | Binds `EventStore`, never `SendEventStore`. One flavour name per module. No `#[async_trait]`. `read` untouched. The glob survives and nothing shadows a contract name — **including the module name**, which is why the module is `runner`. |
| AC-012 | satisfied | `projection_runner.rs:26-32` (the import) + `doc_surface.rs::no_local_projection_store` | HS-P0010's `MemoryProjectionStore` had landed, so EC-009 did not fire. No `impl ProjectionStore for` exists anywhere under `crates/happenstance/`. |

### What a reviewer should look at first

1. **`checkpoint_without_rows_is_rejected`** (`crates/happenstance/tests/projection_runner.rs:443`).
   This is the story's own falsifier. If the atomicity oracle
   (`rows_and_checkpoint_agree`, `:372`) were weakened to make the happy path easier, this
   test goes red, because it asserts the wrong runner **fails** it.
2. **`commits_before_the_stream_ends`** (`:557`). The single assertion that separates a
   streaming runner from a `collect`ing one. Everything else in the file passes against a
   runner that buffers the whole log.
3. **The reference-style link** (`crates/happenstance/src/lib.rs:169-176`). An inline
   `](run_projection)` is a hard rustdoc error in the default build, which is the one a
   casual `cargo doc` runs. The test asserts both conditional spellings.

### Deviations, all recorded in the implementation report

- **`xtask/src/main.rs` changed.** One assertion whose stated premise this story falsifies was
  rewritten into a two-directional one that is strictly harder to satisfy. It was not deleted
  and it was not softened.
- **`futures-core` became an optional dependency** of `happenstance`, enabled by
  `unstable-projection`. Zero new nodes in the dependency graph; the alternative was an edit
  under `crates/happenstance-core/src/**`, which AC-A02 forbids.
- **`Batch` is no longer a GAT**, so the design's `Batch<'_>` spelling in `apply` became
  `Batch`. The port changed under HS-P0010; nothing about the shape of this story moved.

### Handed forward

- **HS-S0027 (`projection-clause-verdicts`)** consumes the PS-33 / PS-27 / PS-30 counts.
  One correction it must act on: **PS-18's subject now exists.** `reset` and
  `ResetError::Refused` landed with HS-P0010 (`crates/happenstance-core/src/projection.rs:497`
  and `:270`), so this story's spec clarification 7 — *"its subject does not exist in the
  tree"* — is out of date, and the documented exclusion it planned needs re-deriving from the
  tree rather than from the spec.
- **HS-S0028 (`polling-cost-measurement`)** measures this runner. The number it is after is
  N views × N reads, and the runner it holds is `happenstance::run_projection` behind
  `unstable-projection` — one read per call, so the amplification is a property of how many
  times a caller calls it.
- **HS-S0031 (`edge-flavour-and-wasm-claim`)** registers the fifth `wasm32` step. The exact
  invocation it will register already passes:
  `cargo check -p happenstance --target wasm32-unknown-unknown --no-default-features
  --features std,json,unstable-projection`.
