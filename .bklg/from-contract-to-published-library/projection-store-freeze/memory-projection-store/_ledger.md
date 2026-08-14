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
  satisfied: true
  evidence: |
    `MemoryProjectionStore` at crates/happenstance-core/src/projection_memory.rs:95, `MemoryProjectionBatch` at :188, `MemoryProjectionStoreError` at :247, all three re-exported from crates/happenstance-core/src/lib.rs:146-150 under the same `#[cfg(feature = "memory")]` + `#[cfg_attr(docsrs, doc(cfg(feature = "memory")))]` pair `MemoryEventStore` carries at :142-144, with the private module gated identically at :116-118. `memory = ["std"]` is unchanged and no dependency was added. The mount is proven by construction rather than by inspection: crates/happenstance-core/tests/projection_memory.rs is an *integration* test file, so it can name only public items and the whole file — 21 tests — fails to compile if the re-export is missing, which is exactly the RED transcript this story started from (`error[E0432]: no MemoryProjectionStore in the root`). ::store_is_reachable_from_the_public_surface constructs one and reads a checkpoint. CHANGELOG.md:28-37 carries the `[Unreleased] / ### Added` entry naming the store, its feature and what it unblocks.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::store_is_reachable_from_the_public_surface (and the file compiling at all)"

- id: AC-002
  criterion: |-
    GIVEN an adapter author copying this store's shape onto a `Send` runtime and, separately, onto `wasm32`, WHEN they read how the one impl serves both, THEN they find `impl SendProjectionStore for MemoryProjectionStore` with the bare flavour derived by `#[trait_variant::make]` and no `#[async_trait]` anywhere; and WHEN they call `begin`, THEN it is neither `async` nor fallible and hands back an owned, empty batch — so a transport that cannot afford a round trip to open a buffer is still able to implement the port.
  satisfied: true
  evidence: |
    `impl SendProjectionStore for MemoryProjectionStore` at crates/happenstance-core/src/projection_memory.rs:249 — one impl, with the bare flavour derived by `#[trait_variant::make(SendProjectionStore: Send)]`; `rg -n "async_trait" crates/happenstance-core/` finds nothing. `fn begin(&self) -> Self::Batch` at :259 returns the owned batch directly. Verified by crates/happenstance-core/tests/projection_memory.rs::begin_is_synchronous_and_infallible (passing), which binds the batch with no `.await` and no `?`, so restoring either for symmetry with `EventStore` is a compile error; ::send_impl_satisfies_the_bare_bound (passing), a generic `fn` bound on the weaker `ProjectionStore` accepting `&MemoryProjectionStore`; and ::the_store_and_its_batch_are_send_and_sync (passing). `cargo clippy --locked` over all seven affected packages with `-D warnings` green.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::begin_is_synchronous_and_infallible; ::send_impl_satisfies_the_bare_bound"

- id: AC-003
  criterion: |-
    GIVEN a rule author about to write `commit_is_atomic_with_the_read_model` and needing a store they can trust to be *right*, WHEN they open a batch against this store, write through it, and inspect the store before committing, THEN neither the rows nor the checkpoint have moved; and WHEN `commit` returns `Ok`, THEN both are visible — installed under one lock acquisition, never rows-then-checkpoint. A store that publishes rows first and the checkpoint second is `CheckpointOnlyStore`'s defect with the timing changed, and the oracle must not model it.
  satisfied: true
  evidence: |
    One guard over one struct holding both halves: `State` at crates/happenstance-core/src/projection_memory.rs:109 holds `rows` and `checkpoints` together, `commit` at :272 takes `write_guard()` exactly once at :287 and does not return between the two writes at :306-307. The regression check at :289-298 happens inside that guard and before either write, so there is no path that mutates and then fails. Verified by crates/happenstance-core/tests/projection_memory.rs::open_batch_is_invisible_until_commit (passing) — neither the row nor the checkpoint has moved while a written batch is open — and ::commit_installs_rows_and_checkpoint_together (passing), which asserts both are visible after `Ok`.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::open_batch_is_invisible_until_commit; ::commit_installs_rows_and_checkpoint_together"

- id: AC-004
  criterion: |-
    GIVEN an operator's projection that is rebuilding, and a sibling that is live, WHEN each commits, THEN the store records `Checkpoint::Rebuilding { through }` for one and `Checkpoint::Live { through }` for the other, an id never committed (or freshly `reset`) reads `Checkpoint::NeverRun`, and neither id's checkpoint moves when the other commits; AND WHEN a batch that wrote nothing is committed at a position, THEN the checkpoint still advances to it — the position is the one *considered*, not the one applied — while a position below the recorded checkpoint returns `CommitError::CheckpointRegression { current, attempted }` and changes nothing.
  satisfied: true
  evidence: |
    `Authority::Live` records `Checkpoint::Live { through }` and `Authority::Rebuilding` records `Checkpoint::Rebuilding { through }` at crates/happenstance-core/src/projection_memory.rs:300-305; an absent key reads `Checkpoint::NeverRun` at :266-271. Four passing tests in crates/happenstance-core/tests/projection_memory.rs: ::commit_records_the_authority_it_was_given (a live and a rebuilding projection side by side), ::empty_batch_still_advances_the_checkpoint (position considered, not applied), ::regressing_position_is_rejected_and_changes_nothing (asserts the exact `CommitError::CheckpointRegression { current, attempted }` value *and* that neither the row nor the checkpoint moved), ::distinct_projections_advance_independently. No literal position is asserted anywhere: the file's `next()` helper at :31-33 derives the second position from the first, and every assertion compares against a position handed in.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::commit_records_the_authority_it_was_given; ::empty_batch_still_advances_the_checkpoint; ::regressing_position_is_rejected_and_changes_nothing; ::distinct_projections_advance_independently"

- id: AC-005
  criterion: |-
    GIVEN an adapter author who has two stores in scope and writes `b.commit(a.begin(), …)` by mistake — which the owned batch did **not** make unrepresentable, because a lifetime names a region and not an instance — WHEN the call runs, THEN it returns `CommitError::ForeignBatch` and store `b` is unchanged; the same for `reset` with `ResetError::ForeignBatch`. The identity is stamped per store instance at `begin` and compared as an integer.
  satisfied: true
  evidence: |
    `begin` stamps the store instance's identity into the batch (crates/happenstance-core/src/projection_memory.rs:126-132, from `next_stamp()` at :382); `commit` compares it at :278-280 and `reset` at :318-320, before taking any guard. Verified by crates/happenstance-core/tests/projection_memory.rs::commit_rejects_a_foreign_batch and ::reset_rejects_a_foreign_batch (both passing), each constructing two stores, beginning a batch on one, handing it to the other, and asserting **both** the error arm (`CommitError::ForeignBatch` / `ResetError::ForeignBatch`) and that the receiving store's rows and checkpoint are untouched. The stamp is per *instance* rather than per type — a per-type stamp would compare equal everywhere and make the test unwritable.
    CORRECTION, 2026-08-13 (slice review). The evidence above was true through `new()` and
    **false through `Default`**, which is a public constructor of the same store. The struct
    carried `#[derive(Debug, Default)]`, and the derive fills `stamp` with `0` while
    `next_stamp()`'s counter starts at `1` — so two `MemoryProjectionStore::default()` instances
    shared identity `0`, `b.commit(a.begin(), ..)` was *accepted*, and rows and checkpoint were
    mutated by a batch `b` never opened. `reset` had the identical hole. The two tests cited
    above both construct with `new()`, so neither could see it.

    Fixed: `Default` is hand-written as `Self::new()`
    (crates/happenstance-core/src/projection_memory.rs:108-121), mirroring
    `MemoryEventStore` at crates/happenstance-core/src/memory.rs:78-82, which is hand-written for
    this exact reason. Two regression tests were added and were confirmed **red against the
    derive** before the fix — `a batch begun by another `default()` store is refused: ()` on both
    — and green after:
    crates/happenstance-core/tests/projection_memory.rs::commit_rejects_a_foreign_batch_from_default_stores
    and ::reset_rejects_a_foreign_batch_from_default_stores. Both assert the error arm *and* an
    unchanged receiver, matching the `new()` pair.
    `cargo test --locked -p happenstance-core --test projection_memory`: 19 passed, 0 failed.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::commit_rejects_a_foreign_batch; ::reset_rejects_a_foreign_batch"

- id: AC-006
  criterion: |-
    GIVEN an application author whose apply loop panics or returns early mid-batch, WHEN the batch is dropped bare — no `commit`, no `rollback` — THEN nothing it held is visible, AND the store serves a *subsequent* `begin` → write → `commit` normally. The second half is the clause, not decoration: a reviewer's probe found a store that answers `Busy` forever afterwards. `rollback(batch)` has the same effect, stated explicitly.
  satisfied: true
  evidence: |
    A dropped batch holds nothing the store knows about, so dropping it is the rollback (crates/happenstance-core/src/projection_memory.rs:186-187 documents this at the type; `rollback` at :336 is the same discard). Verified by crates/happenstance-core/tests/projection_memory.rs::dropped_batch_leaves_store_usable (passing), which drops a written batch **bare** — no commit, no rollback — asserts nothing it held is visible, and then opens and commits a *second* batch on the same handle and asserts it took effect. That second half is the clause rather than decoration: it is the half a reviewer's probe once found a store failing, by answering `Busy` forever afterwards. ::rollback_leaves_both_unchanged (passing) covers the explicit call.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::dropped_batch_leaves_store_usable; ::rollback_leaves_both_unchanged"

- id: AC-007
  criterion: |-
    GIVEN an operator rebuilding one projection from zero while its siblings keep serving, WHEN they call `reset` with a batch carrying the deletes — the port has no idea what the read model is, so the caller supplies them — THEN that id's rows and its checkpoint are cleared **in one unit**, the id reads `Checkpoint::NeverRun` again, and every sibling id's rows and checkpoint are untouched. `commit(empty, id, FIRST)` is not this: it silently skips event 1, and the store must not make the two look alike.
  satisfied: true
  evidence: |
    `reset` at crates/happenstance-core/src/projection_memory.rs:313 applies the caller's own batch to the rows and *removes* the checkpoint key in the same guard (:331-334) — removing rather than writing a sentinel is what returns the projection to `NeverRun` without reintroducing the ambiguity the three-variant enum forbids. Three passing tests in crates/happenstance-core/tests/projection_memory.rs: ::reset_clears_rows_and_checkpoint_together, ::reset_returns_the_projection_to_never_run (which also asserts `!= Checkpoint::Live { through: FIRST }`, so `commit(empty, id, FIRST)` cannot masquerade as it), ::reset_is_scoped_to_one_projection (a sibling id's checkpoint is untouched). This store never returns `ResetError::Refused` and the reason is documented at :323-326: it has no protection policy, and an oracle that refused arbitrarily would be a worse oracle.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::reset_clears_rows_and_checkpoint_together; ::reset_is_scoped_to_one_projection; ::reset_returns_the_projection_to_never_run"

- id: AC-008
  criterion: |-
    GIVEN a rule author who must observe a read model without knowing what a read model is, WHEN they write generic code bound on `ProjectionStore + ProjectionProbe` only — never an inherent method on the concrete store — THEN `probe_write` then `probe_read` round-trips the value against `MemoryProjectionStore`; AND because this store applies on write, `READS_THROUGH_BATCH` is `true` and `probe_read_through` really returns a pending write rather than `unimplemented!()`. Declaring `false` here would make PS-12's rule skippable by everything in the workspace and is a **finding** for `read-through-and-rebuild-rules`, not a quiet flip.
  satisfied: true
  evidence: |
    `impl ProjectionProbe for MemoryProjectionStore` at crates/happenstance-core/src/projection_memory.rs:343, gated `#[cfg(feature = "conformance")]` — the two features stay independent and the powerset proves it. `READS_THROUGH_BATCH = true` at :346, honest rather than convenient: the batch is a materialised delta and `probe_read_through` at :359 really layers pending writes over committed state (`MemoryProjectionBatch::read_through`, :209-218). Four passing tests in crates/happenstance-core/tests/projection_memory.rs's `probe` module, all gated: ::probe_round_trip_through_the_traits_only, whose helper is bound `S: ProjectionStore + ProjectionProbe` with the concrete store named only at the instantiation site; ::probe_read_through_sees_a_pending_write, which asserts the constant in a **`const` block** so declaring `false` is a build failure rather than a runtime one; ::probe_read_through_layers_pending_over_committed; ::probe_delete_all_supports_reset, which clears the read model without knowing its shape.
  mount_point: "crates/happenstance-core/src/lib.rs:98-124 (export block, under all(feature = \"memory\", feature = \"conformance\")) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "crates/happenstance-core/tests/projection_memory.rs::probe_round_trip_through_the_traits_only; ::probe_read_through_sees_a_pending_write; ::probe_delete_all_supports_reset (all #[cfg(feature = \"conformance\")])"

- id: AC-009
  criterion: |-
    GIVEN the adapter author of persona 2's journey step 2, who today writes an impl and is stopped by `error[E0195]: lifetime parameters or bounds on method 'commit' do not match the trait declaration` with nothing in the workspace saying `Self::Batch<'_>` is required, WHEN this story's impl spells the **concrete** batch type in `commit` and `rollback` and the crate compiles, THEN PS-5's claim to have removed the trap is discharged by a compiler rather than by a paragraph — and the port's own rustdoc says so **once, at the item whose unusual shape bought it**, naming the alternative that lost, together with PS-36's `Batch: Send` transitivity, which `type Batch: Send;` cannot express without breaking wasm32. If it does **not** compile, that is a finding against `owned-batch-port-shape` and this story stops.
  satisfied: true
  evidence: |
    **`error[E0195]` is retired, and by a compiler rather than a paragraph.** crates/happenstance-core/src/projection_memory.rs:255 spells `type Batch = MemoryProjectionBatch;` and :272-280 spells `async fn commit(&self, batch: Self::Batch, …)` with the concrete type behind it; the crate compiles (`cargo check -p happenstance-core --all-features` green, `cargo clippy … -D warnings` green over all seven affected packages). EC-007's halt condition did not fire. The comment at :250-254 records what that line used to cost. The port's own rustdoc says so once, at the item whose unusual shape bought it: crates/happenstance-core/src/projection.rs:230-247 carries the trap and its disposition on `ProjectionStore`'s `# Implementing it` section, and the toy-store doctest below it (:249-345) spells a concrete batch type with no `Self::Batch<'_>` anywhere — so `cargo test -p happenstance-core --all-features --doc` re-proves it on every run (passing as `projection::ProjectionStore (line 273)`). PS-36's `Batch: Send` transitivity is documented once, at `type Batch` (projection.rs:264-272) and in the module doc (:38-47). No `compile_fail` doctest was written, so the bare-versus-`E0308` spelling hazard does not arise.
  mount_point: "crates/happenstance-core/src/projection.rs (port rustdoc + toy-store doctest, documentation only — signatures are owned-batch-port-shape's)"
  verifying_test: "cargo test -p happenstance-core --all-features --doc over the toy-store doctest in crates/happenstance-core/src/projection.rs; plus cargo check -p happenstance-core --all-features (the compile itself is PS-5's evidence)"

- id: AC-010
  criterion: |-
    GIVEN two different readers — an adapter author who needs a minimal impl to copy that does not force them to enable a feature, and an application author (P1) who needs to see the loop actually run — WHEN they open the two pages, THEN `ProjectionStore`'s rustdoc carries a **toy-store impl** doctest with no dependency on the `memory` feature, and `MemoryProjectionStore`'s rustdoc carries a **runnable walkthrough** (`begin` → write → `commit` → read back the rows *and* the checkpoint) mirroring `crates/happenstance-core/src/memory.rs:36-60`. Both execute; neither is a `no_run` or `ignore` sketch.
  satisfied: true
  evidence: |
    Two doctests, two jobs, both executing. **The port's page** carries a toy-store impl at crates/happenstance-core/src/projection.rs:249-345 with **no dependency on the `memory` feature** — it defines its own `ToyStore`/`ToyBatch` and names no gated item — passing as `projection::ProjectionStore (line 273)` and again as `projection::SendProjectionStore (line 273)`. **The store's page** carries the runnable walkthrough at crates/happenstance-core/src/projection_memory.rs:50-93: `begin` → `write` → `commit` → read back **both** the rows and the checkpoint, then `reset` back to `NeverRun`, mirroring memory.rs:36-60's shape and driven by `#[tokio::main(flavor = "current_thread")]` so every assertion actually executes rather than sitting in a future nobody polls. Passing as `projection_memory::MemoryProjectionStore (line 50)` under both `cargo test -p happenstance-core --doc` (29 passed) and `--all-features --doc` (31 passed). Neither is `no_run` or `ignore`. DEVIATION from `_design.md`'s `## The doctest` block, recorded in the implementation report: the walkthrough drives the store's **inherent** `batch.write` / `store.get` rather than `ProjectionProbe`'s methods, and is therefore ungated. The design's version is `#[cfg(feature = "conformance")]`-gated, so under a plain `cargo test -p happenstance-core --doc` it degrades to `fn main() {}` — nominal rather than true for the reader AC-010 names, the application author. The probe path it wanted checked is checked, by the four gated tests of AC-008.
  mount_point: "crates/happenstance-core/src/projection.rs (port doctest) + crates/happenstance-core/src/memory_projection.rs (store doctest), reachable through the lib.rs:98-124 export block"
  verifying_test: "cargo test -p happenstance-core --all-features --doc (both doctests run)"

- id: AC-011
  criterion: |-
    GIVEN a `no_std` consumer, a `wasm32` consumer, a consumer who wants `conformance` without `memory`, and docs.rs, WHEN each configuration is built, THEN all of them succeed: no page that renders without `memory` intra-doc-links the store or its module, no page that renders without `conformance` links the probe impl, `cargo doc --no-default-features` (a hard error, not a warning, for a broken link — D13) is green, the nightly `--cfg docsrs` build renders both feature badges, and the host and `wasm32` feature powersets are green over the widened combination set. The crate root gains the store in its feature-flag list and "Getting started" prose, spelled plainly, exactly as `MemoryEventStore` already is.
  satisfied: true
  evidence: |
    Host powerset `cargo hack check -p happenstance-core --feature-powerset --no-dev-deps`: 16/16 green, including `memory` without `conformance`, `conformance` without `memory`, and `--no-default-features` (`no_std`). wasm32 powerset over `happenstance-core`, `happenstance-neon` and `happenstance-testkit`: 25/25 green. `cargo xtask wasm`: all four steps green, including the mandatory plain `wasm32` build of the contract crate — the new module uses `std::sync::RwLock` and `std::collections::BTreeMap` only, no threads, no clock. `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --no-default-features` green, which is the **hard error** an intra-doc link into a `cfg`-absent module would produce; `--all-features` doc build green. No page that renders without `memory` links the store or its module — crates/happenstance-core/src/lib.rs:68-75 names it in the *Getting started* prose with the link deliberately absent and the reason stated inline, and :80-81 in the feature-flags list — and no page that renders without `conformance` links the probe impl. `[package.metadata.docs.rs]` already sets `all-features = true` and `--cfg docsrs`, and both new gates carry `#[cfg_attr(docsrs, doc(cfg(…)))]` so the badges render.
    CORRECTION, 2026-08-13 (slice review). The sentence above — "no page that renders
    without `conformance` links the probe impl" — was **false when written**, and the two doc
    configurations cited are exactly the two that cannot see it.
    crates/happenstance-core/src/projection_memory.rs:38 carried
    `[`ProjectionProbe::READS_THROUGH_BATCH`](crate::ProjectionProbe::READS_THROUGH_BATCH)`
    inside `MemoryProjectionStore`'s rustdoc: that page renders whenever `memory` is on,
    `ProjectionProbe` exists only under `conformance`, and `rustdoc::broken_intra_doc_links` is
    `deny` — so `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance-core --no-deps` on the
    **default** feature set was `error: unresolved link ... error: could not document
    happenstance-core`. `--all-features` resolved it (the gate is open) and
    `--no-default-features` never rendered the page (no `memory`), which is why both cited runs
    were green over a hard error.

    Fixed: the probe's name is now spelled plainly with the reason inline
    (crates/happenstance-core/src/projection_memory.rs:38-49), exactly as
    crates/happenstance-core/src/lib.rs:70-75 and :91-93 already do. A pre-existing
    `redundant_explicit_links` hard error on the same page (`[`CommitError`](crate::CommitError)`,
    which only fires under `--document-private-items`) was fixed with it. The blind spot is
    closed by a third gate step, `documentation (default features)`
    (xtask/src/main.rs:515-540), so the claim is now checked rather than asserted.

    Re-verified green, all three configurations, 2026-08-13:
    `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps --document-private-items`;
    `... cargo doc --locked -p happenstance-core --no-default-features --no-deps`;
    `... cargo doc --locked -p happenstance-core --no-deps --document-private-items`.
    `cargo xtask ci --fast` green whole, with the new step running.
  mount_point: "crates/happenstance-core/src/lib.rs:66-83 (crate-root prose and feature list) + :98-124 (cfg/cfg_attr(docsrs) gate pair) + crates/happenstance-core/Cargo.toml [features]"
  verifying_test: "cargo doc -p happenstance-core --no-default-features; cargo hack check --workspace --feature-powerset --no-dev-deps (xtask/src/main.rs:546-556); cargo hack check -p happenstance-core -p happenstance-neon -p happenstance-testkit --target wasm32-unknown-unknown --feature-powerset --no-dev-deps (:558-591); nightly --cfg docsrs rustdoc build"
```
