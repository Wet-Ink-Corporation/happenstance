---
item: "HS-S0044"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — SqliteProjectionStore against the suite it did not write

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three of these rows are *findings*, not code: **AC-011** is the row that keeps PS-2 unclaimed, records
PS-18's falsifier as now asked, and records PS-12's `false` arm as first exercised outside the testkit.
Its evidence is prose written here with cited `file:line`, plus a clean `git diff --stat` over
`spec/`, `.kb/`, `crates/happenstance-core/` and `crates/happenstance-testkit/`.

```yaml
- id: AC-001
  criterion: "GIVEN an adapter author who has only ever been told by the type checker that `SqliteProjectionStore` *could* implement the port, WHEN they run `cargo test -p happenstance-sqlite --all-features`, THEN `projection_store_conformance!(SqliteProjectionFixture::new())` executes at `crates/happenstance-sqlite/tests/projection.rs` against a real temporary SQLite **file**, and **every** rule the suite enumerates appears in the run as `Ran` or as `Skipped { capability, reason }` — never absent, and never against an in-memory stand-in."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — projection_store_conformance!(SqliteProjectionFixture::new()); command `cargo test -p happenstance-sqlite --all-features --test projection`"

- id: AC-002
  criterion: "GIVEN a consumer who must trust that two projection stores in one process do not silently share a database, WHEN the suite builds two fixture instances and opens handles from each, THEN one fixture instance is one fresh file and each `connect()` is a real `rusqlite::Connection` onto it — never an `Arc` clone — so `commit_rejects_a_foreign_batch` is exercised against two genuinely isolated stores rather than two names for one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — commit_rejects_a_foreign_batch and distinct_projections_advance_independently, plus reviewed SqliteProjectionFixture::connect body"

- id: AC-003
  criterion: "GIVEN an author whose read model must never be ahead of, or behind, its checkpoint after a crash, WHEN `commit` is called with a batch of queued statements, THEN the replay, the checkpoint upsert and the `COMMIT` happen inside **one** `BEGIN IMMEDIATE` on one connection; and WHEN the commit fails, or the batch is rolled back, or the batch is simply dropped, THEN read model and checkpoint are both unchanged and the store still accepts and commits the next batch."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — commit_is_atomic_with_the_read_model, commit_advances_the_checkpoint, failed_commit_leaves_both_unchanged, rollback_leaves_both_unchanged, dropped_batch_leaves_store_usable"

- id: AC-004
  criterion: "GIVEN a consumer deciding whether the rows they are about to read can be trusted, WHEN they call `checkpoint(id)`, THEN they get `NeverRun` for a projection that has never committed, `Live { through }` after an authoritative commit and `Rebuilding { through }` after a rebuild commit — read back from a stored authority column, never inferred from the absence of a row — and a stored position that is not a valid `SequencePosition` surfaces as `InvalidPosition` rather than a silent zero."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — fresh_projection_has_no_checkpoint, rebuilding_is_distinguishable_from_live, commit_advances_the_checkpoint, plus the adapter-private corrupt-row test asserting SqliteProjectionStoreError::InvalidPosition"

- id: AC-005
  criterion: "GIVEN two runners that both believe they own a projection, WHEN each commits, THEN the loser is refused with a **distinct** `CommitError::CheckpointRegression { current, attempted }` — decided by a read taken *inside* the same `BEGIN IMMEDIATE` that writes, so the two cannot interleave and both pass — while a commit carrying a position **ahead of** anything the batch itself wrote is accepted, because a projection legitimately advances past events it filtered out."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — commit_rejects_a_regressing_position and commit_accepts_a_position_the_batch_did_not_write; reviewed commit body in crates/happenstance-sqlite/src/projection_store.rs"

- id: AC-006
  criterion: "GIVEN an author holding batches from two stores, WHEN they hand store A's batch to store B, THEN B refuses it with `CommitError::ForeignBatch` **before any SQL is issued** — decided by a per-store-instance stamp minted in the constructor and compared in `commit`, `reset` and `rollback`, not by a lifetime — leaving both stores untouched; and WHEN they try to build a batch by hand, THEN they cannot: `SqliteBatch::new()` and its `Default` are withdrawn so `begin` is the only mint, while cloning the store (it is `Clone` over an `Arc`) does **not** change the stamp, because a clone is the same store."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — commit_rejects_a_foreign_batch, plus the adapter-private test asserting a cloned store accepts its origin's batch; reviewed absence of SqliteBatch::new/Default in crates/happenstance-sqlite/src/projection_store.rs"

- id: AC-007
  criterion: "GIVEN an author who must rebuild a read model from scratch, WHEN they queue their own deletes into a batch and call `reset(batch, id)`, THEN those deletes and the removal of the checkpoint happen as one unit scoped to that one projection id, leaving every other projection's checkpoint and rows untouched, and afterwards `checkpoint(id)` reports `NeverRun` — distinguishable from `commit(empty, id, FIRST, Live)`. The adapter deletes nothing it chose itself: the read model is the caller's."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — reset_clears_rows_and_checkpoint_together, reset_is_scoped_to_one_projection, reset_is_not_commit_at_first; reviewed reset body containing no adapter-chosen DELETE FROM"

- id: AC-008
  criterion: "GIVEN an adapter author following the same route for their own crate, WHEN they look for where the suite's read/write seam is implemented, THEN they find `impl ProjectionProbe for SqliteProjectionStore` in `crates/happenstance-sqlite/src/projection_store.rs` behind `all(feature = \"projection-store\", feature = \"conformance\")` — not in `tests/`, where the orphan rule rejects it — with the probe's table created by `migrate` under that same gate and absent from any production schema; and WHEN the store is opened twice concurrently on one path, THEN `migrate` is idempotent on this store's **own** connection, carrying its own version marker, folded into neither the event store's migration nor its connection."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — the whole suite run (every rule writes through the probe) plus the adapter-private concurrent-open test; `cargo build -p happenstance-sqlite --no-default-features --features conformance`"

- id: AC-009
  criterion: "GIVEN an evaluator reading the run to decide what this adapter actually proved, WHEN a capability is declined, THEN the rule still appears in the binary as a reported skip carrying this fixture's **own stated reason** — `READS_THROUGH_BATCH = false` because a buffer that has issued no statement has nothing to read through and answering from committed state is forbidden; the reset-refusal capability declined because no projection here is protected — and neither reason is a shrug: each names the trade and the alternative that lost (the shadow map inside the batch, rejected on the record)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/projection.rs — batch_reads_reflect_pending_writes and refused_reset_changes_nothing present in the run output as Skipped { capability, reason }; reviewed reason strings on SqliteProjectionFixture"

- id: AC-010
  criterion: "GIVEN a consumer who will `cargo add` this crate with an arbitrary feature selection and read its docs, WHEN the gate runs, THEN `crates/happenstance-sqlite/src/projection_store.rs` contains **no `todo!()`**, `cargo hack` is clean over the widened powerset (`conformance` without `projection-store` compiles; `projection-store` forwards `happenstance-core/unstable-projection`; `--no-default-features` builds neither), `tests/shapes.rs` still holds after the stamp and any new field, and the module documentation no longer describes a port that is gone — every public item this story changes carrying rustdoc with an `# Errors` section naming conditions rather than types."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs (`cargo test -p happenstance-sqlite --test shapes`); `cargo hack check -p happenstance-sqlite --feature-powerset`; `cargo doc -p happenstance-sqlite --all-features`; `rg -n \"todo!\" crates/happenstance-sqlite/src/projection_store.rs` empty"

- id: AC-011
  criterion: "GIVEN a reader of the eventual freeze verdict, WHEN they open this story's `_ledger.md`, THEN they find PS-2 recorded as **not cleared** — this run is a *third* replay-at-commit shape, not the live-transaction end of the batch-shape axis, and the stronger finding that under `type Batch;` that end may be unreachable by *any* rusqlite adapter — PS-18's falsifier recorded as now asked and answered, and PS-12's `false` arm recorded as first exercised by a non-testkit adapter; and THEN `spec/SPECIFICATION.md` is unedited, no maturity marker has moved, no projection conformance rule was added, renamed or reordered, and no ADR was authored in this PR."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/projection.rs"
  verifying_test: "cargo xtask spec-trace (citation count not fallen, inside `cargo xtask affected --base main`); `git diff --stat` empty over spec/, .kb/, crates/happenstance-core/, crates/happenstance-testkit/; the three findings written into this ledger with cited file:line"
```
