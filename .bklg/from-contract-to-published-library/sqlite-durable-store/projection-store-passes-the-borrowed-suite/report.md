---
item: "HS-S0044"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — SqliteProjectionStore against the suite it did not write

## Findings Ledger

**Eleven of eleven ACs satisfied. Nothing blocked, nothing deferred, nothing
skipped that was not skipped on the record with a stated reason.**

`happenstance-sqlite` now holds the workspace's **first projection store that has
passed a suite it did not write**, against a real temporary file on disk. Of the
seventeen rules `projection-store-freeze` ships, **fourteen `Ran`** and **three are
reported skips**; none is absent from the binary, which is the failure mode the
whole mount contract exists to forbid.

**Mount point:** `crates/happenstance-sqlite/tests/projection.rs` — a new `cargo
test` target, reachable from the repository's own merge-gate command
(`cargo xtask affected --base main`, which passes `--all-features` and therefore
compiles the `conformance`-gated target rather than quietly skipping it). The
mount is one line, `:226`:
`happenstance_testkit::projection_store_conformance!(SqliteProjectionFixture::new())`.

**Second wiring point:** `crates/happenstance-sqlite/src/projection_store.rs:692`
— `impl happenstance_core::ProjectionProbe for SqliteProjectionStore`, in `src/`
behind `all(projection-store, conformance)`, because `tests/` is a different crate
where the orphan rule answers `error[E0117]`. It is the seam every rule in the
family writes and reads the read model through, so a wrong probe is a red suite
rather than a silent pass.

| Finding | Evidence | Follow-up |
| --- | --- | --- |
| **The suite runs whole, against a file** — 24 tests, 17 rules, 0 absent | `cargo test -p happenstance-sqlite --all-features --test projection`; the file is asserted to exist on disk by `the_fixture_backs_the_suite_with_a_real_file_and_real_connections` (tests/projection.rs:257) | — |
| **PS-1's second conjunct is demonstrated, not skipped** — a commit that reported failure left both halves exactly as they were | `failed_commit_leaves_both_unchanged` **`Ran`**; the fixture arms `BEFORE INSERT`/`BEFORE UPDATE` triggers on `projection_checkpoint` through its own connection (tests/projection.rs:212-225) | This is the first fixture in the workspace to declare `COMMIT_FAULT`. `projection-store-freeze` may want it named in PS-1's evidence |
| **PS-2 is NOT cleared** | This is a *third* replay-at-commit shape beside `MemoryProjectionStore` and the testkit's buffering variant (src/projection_store.rs:13-17) | **HS-P0016 / DoD 8.** Do not read a green run here as the second end of the batch-shape axis |
| **The stronger PS-2 finding, first observable here** | Under `type Batch;` the live-transaction end may be unreachable by **any** rusqlite adapter on the `Send` flavour: `rusqlite::Transaction<'_>` is `!Send`, so `commit` is rejected on the batch *parameter* even when the store is wrapped to be `Sync` (src/projection_store.rs:19-43). A live-transaction rusqlite adapter could only implement the **bare** flavour | `publication-and-positioning` (HS-P0016) and `projection-store-freeze`'s freeze verdict. This is a fact about PS-2's stated bar, not a scheduling note |
| **PS-18's falsifier has been asked and answered** | PS-18 says it is *"evaluated by asking whether the SQLite adapter implemented it"*. It did not: this store holds no protection policy, and the fixture declines `RESET_REFUSAL` with that reason and with the rejected alternative named (tests/projection.rs:143-176) | `projection-store-freeze` owns the marker. **It was not moved here** |
| **PS-12's `false` arm is exercised outside the testkit for the first time** | `READS_THROUGH_BATCH = false` (src/projection_store.rs:708) with the shadow-map alternative rejected on the record; `batch_reads_reflect_pending_writes` and `rebuild_is_chunk_size_invariant` are reported skips, never absent | `projection-store-freeze`; the clause is `[PROVISIONAL]` and stays so |
| **`spec/SPECIFICATION.md:383` is now stale about this crate** | It says `SqliteProjectionStore` carries real bodies "in `begin` and `rollback`" only, and that "none has run a suite because none exists". Both stopped being true with this commit. The spec was **not** edited; the citation was re-anchored from the code side instead | **`spec-and-code-reconciliation` (HS-S0047)** — named in this story's Unlocks, and this is the input it was promised |
| **Two compelled excursions, admitted rather than disclosed** | `tests/migration.rs`'s own assertion message asked for its edit; two `standards/rust` citations drifted because cited code moved. Both are now entries in this story's PR-boundary block on `benchmark-harness`'s terms (`e020276`) | — |
| **Three error variants added, each reachable and tested** | `InvalidAuthority`, `UnsupportedSchemaVersion`, `ForeignBatch` (src/projection_store.rs:466-513), asserted by three targeted tests | `crates-io-name-and-packaging-facts` if the surface is ever published; `#[non_exhaustive]` means no caller's match breaks |

## Acceptance

| AC | Status | Verification |
| --- | --- | --- |
| **AC-001** — the suite runs whole, against a real file, every rule present | **Met** | 17/17 rules in the binary: 14 `Ran`, 3 `Skipped { capability, reason }`. tests/projection.rs:226 |
| **AC-002** — one instance is one file; each `connect` is a real connection | **Met** | tests/projection.rs:189 and :83-98; observed at :257, plus green `commit_rejects_a_foreign_batch` and `distinct_projections_advance_independently` |
| **AC-003** — one `BEGIN IMMEDIATE`; failure, rollback and drop all leave both halves unchanged | **Met** | src/projection_store.rs:823 `commit_locked`; five green rules including `failed_commit_leaves_both_unchanged` |
| **AC-004** — the three-state checkpoint read back from a stored authority; corrupt positions reported | **Met** | src/projection_store.rs:775, :897, :911; `fresh_projection_has_no_checkpoint`, `rebuilding_is_distinguishable_from_live`, and two targeted corruption tests |
| **AC-005** — regression refused by a read *inside* the transaction, as a distinct arm | **Met** | src/projection_store.rs:834-846 and :614-617; `commit_rejects_a_regressing_position`, `commit_accepts_a_position_the_batch_did_not_write` |
| **AC-006** — an instance stamp compared before any SQL; `begin` the only mint; `Clone` keeps it | **Met** | src/projection_store.rs:600-603/:649-652/:673-676, :415, :390-403; `commit_rejects_a_foreign_batch` plus two targeted tests |
| **AC-007** — `reset` applies the caller's deletes and returns to `NeverRun`, scoped to one id | **Met** | src/projection_store.rs:859 `reset_locked`; three green reset rules |
| **AC-008** — the probe impl in `src/` behind both features, its table under the same gate; idempotent migration with its own marker | **Met** | src/projection_store.rs:691-692, :146-151, :283; `concurrent_opens_of_one_path_all_succeed` (8 threads), `a_newer_projection_schema_is_refused`; `--no-default-features --features conformance` builds |
| **AC-009** — a declined capability is a reported skip carrying the adapter's own reason | **Met** | Three `SKIP` lines under `--show-output`, reasons at src/projection_store.rs:693-708 and tests/projection.rs:143-176. `COMMIT_FAULT` deliberately not declined |
| **AC-010** — no `todo!()`, clean powerset, `shapes.rs` holds, docs corrected | **Met** | 0 `todo!()`; `cargo hack` clean over 10 combinations; `shapes.rs` 9 passed; `cargo doc` clean under `-D warnings`; the wrong `# Intended schema` sketch replaced at src/projection_store.rs:45-89 |
| **AC-011** — the three findings recorded; spec unedited, no marker moved, no rule touched, no ADR | **Met** | `_ledger.md`'s AC-011 row; `git diff --stat` empty over `spec/`, `.kb/`, `crates/happenstance-core/`, `crates/happenstance-testkit/`; `spec-trace` 389 citations checked, identical to base |

## Knowledge Harvest

Candidates for `.kb/` at closeout. **None is authored here** — no ADR is a side
effect of an implementation change.

- **A buffering adapter's foreign-batch defence is a minted ordinal plus a
  withdrawn constructor, and the two are one decision.** The stamp is worthless
  while `SqliteBatch::new()` and `Default` exist, because a caller can mint a
  batch no store owns. `publish = false` is what made that a rename rather than a
  semver event; an adapter doing this after publication cannot. Concept, or a
  paragraph in `standards/rust/91-adapter-authoring-recipe.md`.
- **`type Batch;` may have closed the live-transaction end of the batch-shape
  axis for a whole driver family.** Removing the GAT freed implementers, and the
  same change means a `rusqlite::Transaction<'_>` batch is unreachable on the
  `Send` flavour for reasons that have nothing to do with the lifetime. PS-2 asks
  for two adapters at opposite ends of an axis one end of which may not be
  occupiable by this driver at all. **Decision-shaped, and HS-P0016's.**
- **A version marker nothing reads is decoration.** The projection schema's marker
  earns its keep only because `migrate` reads it back in the same transaction and
  refuses a newer file. Reusable as a `standards/rust` line about migrations.
- **The citation-drift lints have a second repair, and it is the code side.**
  When a `file:line` citation drifts and the citing document may not be edited,
  the honest repair is a *true* sentence placed where the anchor now looks — and
  the drifted claim then has to leave as a finding, or the repair has hidden it.
  Playbook material for `spec-and-code-reconciliation`.
