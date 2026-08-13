---
item: HS-S0006
stage: implement
created: 2026-08-12T13:46:00.598Z
updated: 2026-08-12T13:46:00.598Z
---

# Acceptance ledger — MemoryProjectionStore as oracle, doctest target and cold-start fix

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, so a row is not flipped on the wrong evidence:

- **The mount is proven by compilation, not by an assertion.**
  `crates/happenstance-core/tests/projection_memory.rs` is an *integration* test file and can name
  only public items, so the file building at all is AC-001's evidence
  (`spec.md`, Clarifications item 3).
- **AC-009 may resolve as a halt.** If the impl does not compile with the concrete batch type
  (`spec.md` EC-007), the row is not flipped and is not worked around by spelling
  `Self::Batch<'_>` — it is reported as a finding against `owned-batch-port-shape` and the story
  stops.

```yaml
- id: AC-001
  criterion: |-
    GIVEN an adapter author who has added `happenstance-core` with default features and is looking for something to copy, WHEN they write `use happenstance_core::MemoryProjectionStore;` and construct one, THEN the type, its batch type and its error type resolve from the crate root under the `memory` feature — the same gate pair as `MemoryEventStore` — and `CHANGELOG.md`'s `[Unreleased] / ### Added` says the store now exists and what it unblocks. A type mounted in `lib.rs` but absent from `Cargo.toml`'s `[features]`, or the reverse, does not satisfy this.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::store_is_reachable_from_the_public_surface (and the file compiling at all)"

- id: AC-002
  criterion: |-
    GIVEN an adapter author copying this store's shape onto a `Send` runtime and, separately, onto `wasm32`, WHEN they read how the one impl serves both, THEN they find `impl SendProjectionStore for MemoryProjectionStore` with the bare flavour derived by `#[trait_variant::make]` and no `#[async_trait]` anywhere; and WHEN they call `begin`, THEN it is neither `async` nor fallible and hands back an owned, empty batch — so a transport that cannot afford a round trip to open a buffer is still able to implement the port.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::begin_is_synchronous_and_infallible; ::send_impl_satisfies_the_bare_bound"

- id: AC-003
  criterion: |-
    GIVEN a rule author about to write `commit_is_atomic_with_the_read_model` and needing a store they can trust to be *right*, WHEN they open a batch against this store, write through it, and inspect the store before committing, THEN neither the rows nor the checkpoint have moved; and WHEN `commit` returns `Ok`, THEN both are visible — installed under one lock acquisition, never rows-then-checkpoint. A store that publishes rows first and the checkpoint second is `CheckpointOnlyStore`'s defect with the timing changed, and the oracle must not model it.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::open_batch_is_invisible_until_commit; ::commit_installs_rows_and_checkpoint_together"

- id: AC-004
  criterion: |-
    GIVEN an operator's projection that is rebuilding, and a sibling that is live, WHEN each commits, THEN the store records `Checkpoint::Rebuilding { through }` for one and `Checkpoint::Live { through }` for the other, an id never committed (or freshly `reset`) reads `Checkpoint::NeverRun`, and neither id's checkpoint moves when the other commits; AND WHEN a batch that wrote nothing is committed at a position, THEN the checkpoint still advances to it — the position is the one *considered*, not the one applied — while a position below the recorded checkpoint returns `CommitError::CheckpointRegression { current, attempted }` and changes nothing.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::commit_records_the_authority_it_was_given; ::empty_batch_still_advances_the_checkpoint; ::regressing_position_is_rejected_and_changes_nothing; ::distinct_projections_advance_independently"

- id: AC-005
  criterion: |-
    GIVEN an adapter author who has two stores in scope and writes `b.commit(a.begin(), …)` by mistake — which the owned batch did **not** make unrepresentable, because a lifetime names a region and not an instance — WHEN the call runs, THEN it returns `CommitError::ForeignBatch` and store `b` is unchanged; the same for `reset` with `ResetError::ForeignBatch`. The identity is stamped per store instance at `begin` and compared as an integer.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::commit_rejects_a_foreign_batch; ::reset_rejects_a_foreign_batch"

- id: AC-006
  criterion: |-
    GIVEN an application author whose apply loop panics or returns early mid-batch, WHEN the batch is dropped bare — no `commit`, no `rollback` — THEN nothing it held is visible, AND the store serves a *subsequent* `begin` → write → `commit` normally. The second half is the clause, not decoration: a reviewer's probe found a store that answers `Busy` forever afterwards. `rollback(batch)` has the same effect, stated explicitly.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::dropped_batch_leaves_store_usable; ::rollback_leaves_both_unchanged"

- id: AC-007
  criterion: |-
    GIVEN an operator rebuilding one projection from zero while its siblings keep serving, WHEN they call `reset` with a batch carrying the deletes — the port has no idea what the read model is, so the caller supplies them — THEN that id's rows and its checkpoint are cleared **in one unit**, the id reads `Checkpoint::NeverRun` again, and every sibling id's rows and checkpoint are untouched. `commit(empty, id, FIRST)` is not this: it silently skips event 1, and the store must not make the two look alike.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::reset_clears_rows_and_checkpoint_together; ::reset_is_scoped_to_one_projection; ::reset_returns_the_projection_to_never_run"

- id: AC-008
  criterion: |-
    GIVEN a rule author who must observe a read model without knowing what a read model is, WHEN they write generic code bound on `ProjectionStore + ProjectionProbe` only — never an inherent method on the concrete store — THEN `probe_write` then `probe_read` round-trips the value against `MemoryProjectionStore`; AND because this store applies on write, `READS_THROUGH_BATCH` is `true` and `probe_read_through` really returns a pending write rather than `unimplemented!()`. Declaring `false` here would make PS-12's rule skippable by everything in the workspace and is a **finding** for `read-through-and-rebuild-rules`, not a quiet flip.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block, under all(feature = \"memory\", feature = \"conformance\")) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::probe_round_trip_through_the_traits_only; ::probe_read_through_sees_a_pending_write; ::probe_delete_all_supports_reset (all #[cfg(feature = \"conformance\")])"

- id: AC-009
  criterion: |-
    GIVEN the adapter author of persona 2's journey step 2, who today writes an impl and is stopped by `error[E0195]: lifetime parameters or bounds on method 'commit' do not match the trait declaration` with nothing in the workspace saying `Self::Batch<'_>` is required, WHEN this story's impl spells the **concrete** batch type in `commit` and `rollback` and the crate compiles, THEN PS-5's claim to have removed the trap is discharged by a compiler rather than by a paragraph — and the port's own rustdoc says so **once, at the item whose unusual shape bought it**, naming the alternative that lost, together with PS-36's `Batch: Send` transitivity, which `type Batch: Send;` cannot express without breaking wasm32. If it does **not** compile, that is a finding against `owned-batch-port-shape` and this story stops.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs (port rustdoc + toy-store doctest, documentation only — signatures are owned-batch-port-shape's)"
  verifying_test: "cargo test -p happenstance-core --all-features --doc over the toy-store doctest in crates/happenstance-core/src/projection.rs; plus cargo check -p happenstance-core --all-features (the compile itself is PS-5's evidence)"

- id: AC-010
  criterion: |-
    GIVEN two different readers — an adapter author who needs a minimal impl to copy that does not force them to enable a feature, and an application author (P1) who needs to see the loop actually run — WHEN they open the two pages, THEN `ProjectionStore`'s rustdoc carries a **toy-store impl** doctest with no dependency on the `memory` feature, and `MemoryProjectionStore`'s rustdoc carries a **runnable walkthrough** (`begin` → write → `commit` → read back the rows *and* the checkpoint) mirroring `crates/happenstance-core/src/memory.rs:36-60`. Both execute; neither is a `no_run` or `ignore` sketch.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/projection.rs (port doctest) + crates/happenstance-core/src/memory_projection.rs (store doctest), reachable through the lib.rs:98-124 export block"
  verifying_test: "cargo test -p happenstance-core --all-features --doc (both doctests run)"

- id: AC-011
  criterion: |-
    GIVEN a `no_std` consumer, a `wasm32` consumer, a consumer who wants `conformance` without `memory`, and docs.rs, WHEN each configuration is built, THEN all of them succeed: no page that renders without `memory` intra-doc-links the store or its module, no page that renders without `conformance` links the probe impl, `cargo doc --no-default-features` (a hard error, not a warning, for a broken link — D13) is green, the nightly `--cfg docsrs` build renders both feature badges, and the host and `wasm32` feature powersets are green over the widened combination set. The crate root gains the store in its feature-flag list and "Getting started" prose, spelled plainly, exactly as `MemoryEventStore` already is.
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-core/src/lib.rs:66-83 (crate-root prose and feature list) + :98-124 (cfg/cfg_attr(docsrs) gate pair) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "cargo doc -p happenstance-core --no-default-features; cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556); cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps (:558-591); nightly --cfg docsrs rustdoc build"
```
