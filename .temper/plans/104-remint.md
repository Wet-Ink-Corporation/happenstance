# #104 — Postgres/Neon StoreId re-mint (VT-6, ADR-0086)

## Goal
Both server adapters earn mint-once through VT-6's documented re-mint arm, with
an adapter-provided operation: `remint_identity` on each store, Postgres
stamping from the row inside the append transaction (cache deleted), Neon
refusing an append when the meta row is absent, and a Neon statement builder.

## Invariants
- A re-mint is visible to the next append on every handle and every clone (no
  cached copy of the identity anywhere in Rust).
- No append commits a row with a NULL origin because the meta row is absent;
  it refuses with `MissingIdentity` and writes nothing.
- Events already written keep the origin they were stamped with.

## Non-goals
Mint-per-open (declined), detection (opt-in only, not built), the
`restored_peer_does_not_reissue_identities` rule (#133), schema change
(no new migration: the re-mint is an UPDATE of an existing row).

## API (all additive)
- `PostgresEventStore::remint_identity(&self) -> impl Future<Output = Result<StoreId, PostgresEventStoreError>>`
  (`pub async fn`). `&self`: the pool is already held (ADR-0086 §5). One
  `UPDATE … RETURNING v`, autocommit. No row → `MissingIdentity`.
- `NeonEventStore::<T: SqlTransport>::remint_identity(&self) -> Result<StoreId, NeonError<T::Error>>` (`pub async fn`).
- `happenstance_neon::migration::remint_statement(&NeonConfig) -> SqlStatement`:
  the one spelling of the re-mint, `.query` is plain SQL for branch tooling.
- `NeonError::MissingIdentity` loses its `cfg(feature = "sync")` gate
  (additive on a `#[non_exhaustive]` enum).
- Private: `PostgresEventStore.store_id: Arc<OnceLock<StoreId>>` removed
  (invisible: `#[non_exhaustive]`, private fields).

## Placement
- Postgres: identity read inside `append_once` as the transaction's first
  statement (SELECT, no `FOR SHARE`: a racing re-mint is harmless because the
  sequence is not rewound, ADR-0086 §7.1; a row lock per append is churn).
  Ingest: identity and begin snapshot in one first statement.
- Neon: append's filter gains `EXISTS (meta row)`; stamp stays spelled once in
  `local_origin`. Zero rows without a conflict → `MissingIdentity`.

## Test list
- [x] pg live: `remint_changes_the_id_stamped_by_an_already_open_handle` (rejects the OnceLock cache; reuses the old handle AND a clone)
- [x] pg live: `append_refuses_without_an_identity_row` (zero rows written)
- [x] pg live: `remint_identity_refuses_without_an_identity_row`
- [x] pg live: `a_rewound_store_reissues_a_pair_without_a_remint` (negative control) and `a_rewound_and_reminted_store_does_not_reissue_a_pair`
- [x] pg unit: port `store_id` now reaches the wire (replaces the cache-seeding test; expectation retired by ADR-0086 §7.1)
- [x] neon unit: `remint_statement` names the configured meta table and returns base64
- [x] neon unit: append statement refuses without identity (EXISTS guard present on both shapes)
- [x] neon unit: zero rows, no conflict → `MissingIdentity` (both shapes)
- [x] neon unit: `remint_identity` decodes the returned id / zero rows → MissingIdentity (scripted transport)
- [~] neon live (CI only, compiled locally; runs in live-neon): remint seen by same handle; append refuses without row
- [x] review round 2 (F1) neon unit: `an_unidentified_store_writes_no_compensation_and_is_refused` (c gated on EXISTS own; HS086 raised before HS038)
- [x] review round 2 (F1) neon unit: `an_unidentified_refusal_answer_is_missing_identity`
- [~] review round 2 (F1) neon live (CI only): `an_ingest_into_a_store_without_an_identity_row_is_refused_and_writes_nothing`
- [x] review round 2 (F8) neon unit: `remint_identity_with_a_row_lacking_the_column_is_missing_column`

## HITL
- New migration: none needed (stated in report).
- Test expectation change: the Postgres cache-seeding unit test pins behaviour
  ADR-0086 (accepted) orders removed; replaced, reported.

## Progress
- RED (compile): tests/remint.rs, E0599 no method named `remint_identity` on PostgresEventStore.
- RED (runtime, remint_identity added, cache still present): `remint_changes_the_id_stamped_by_an_already_open_handle` failed "an open handle stamps the re-minted identity" (left old id, right minted); `append_refuses_without_an_identity_row` failed at expect_err (cached id stamped). Negative control and remint_identity_refuses passed. (First test hit a Docker Hub 429 pulling the image; environment, not adapter.)
- GREEN (pg): cache removed, identity read in append_once / ingest first statement; remint target 5/5, lib 75/75 with --include-ignored, whole postgres suite with --include-ignored green (conformance 113/113).
- RED (neon compile): E0432 `migration::remint_statement`, E0599 `remint_identity`.
- RED (neon runtime, remint added, append unchanged): `both_append_shapes_insert_only_where_the_identity_row_exists` (no guard in insert), `an_unconditional_append_that_inserts_nothing_is_missing_identity` and `a_conditional_append_without_an_identity_row_is_missing_identity` both got Store(MissingColumn { column: "position" }).
- GREEN (neon): EXISTS guard in both append shapes, probe reports `identified`, decode maps an empty insert to MissingIdentity; lib 8/8 remint tests. SQL semantics verified by hand against the local Postgres 17.10 (migration 1 rendered, insert/probe/remint run with and without the meta row).
- Added after green: pg `ingest_refuses_without_an_identity_row` (live lib), neon `the_documented_remint_is_the_default_configurations`.
- Gate GREEN at 9974ae200a7837f3 (after two runs lost to ENOSPC). lints, spec-trace, lint-constitution, lint-kb (HS_KB_BASE=origin/phase-13) green. wasm32 check of neon (sync, no-default) green.
- Review round 2: RED `an_unidentified_*` E0432 (UNIDENTIFIED_GUC/SQLSTATE missing); GREEN after the gate on `c`, the second GUC and the HS086 raise. RED `remint_identity_with_a_row_lacking_the_column_is_missing_column` panicked at tests/remint.rs:170 (got MissingIdentity); GREEN after the row/column split in `decode_reminted`.
