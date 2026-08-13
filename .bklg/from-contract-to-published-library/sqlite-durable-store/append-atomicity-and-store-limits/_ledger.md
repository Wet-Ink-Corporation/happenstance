---
item: "HS-S0037"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — append: preconditions, then one BEGIN IMMEDIATE transaction

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two evidence notes specific to this story, both from `spec.md`:

- **AC-002's evidence cites run output, not a green binary.** The failure mode project AC-009
  names is a rule that *skips* while everything stays green (`discover.md`, *The second mutant*),
  so the ceiling assertions must be shown to have run.
- **AC-010 is partly a negative claim.** Its evidence includes the diff check that
  `crates/happenstance-core/**`, `crates/happenstance-testkit/**`, `spec/SPECIFICATION.md` and
  `.kb/**` are untouched — CF-40's clause home stays open.

```yaml
- id: AC-001
  criterion: "GIVEN an application author whose retry loop branches on `is_condition_violated` and rebuilds its decision model on every rejection, WHEN it calls `append` with an empty slice and a condition that a matching event in the store would violate, THEN it receives `AppendError::NoEvents` and not `ConditionViolated`, so the loop terminates instead of rebuilding a model that will produce the same empty batch forever."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::empty_batch_is_refused_before_any_condition_is_looked_at, plus happenstance_testkit::rules::append_rejects_empty_batch (crates/happenstance-testkit/src/suite.rs:2841) and ::empty_batch_is_refused_before_the_condition_is_evaluated (:2870) invoked from the same target"

- id: AC-002
  criterion: "GIVEN an adapter author who must tell a sync runner apart a batch that will never fit in this store from a disk that is momentarily full, WHEN a batch exceeds any of the three ceilings this adapter declares, THEN `append` returns `AppendError::ExceedsStoreLimit { limit, len }` carrying the matching `StoreLimit` variant and the offending magnitude — never `AppendError::Store`, never a truncation — and a value at exactly the ceiling is still accepted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::each_ceiling_is_exact_at_both_ends (all three StoreLimit variants, at the ceiling and at ceiling + 1), mirroring crates/happenstance-testkit/src/suite.rs:4272-4360"

- id: AC-003
  criterion: "GIVEN an application author whose second, independent reader must never build an answer from a torn log, WHEN any `append` is refused — empty batch, any ceiling, or a violated guard — THEN a **second raw `rusqlite::Connection`** opened on the same file afterwards finds no row of the refused batch in `event`, `event_tag` or `tag_cardinality`, so the refusal cost the log nothing."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::a_refused_append_leaves_the_file_unchanged — all three tables counted and compared through a connection the store never held"

- id: AC-004
  criterion: "GIVEN an adapter author who cannot trust a green single-threaded run, WHEN two handles onto one file decide from the same state and both attempt a conditional append, THEN the loser is refused as `AppendError::ConditionViolated` rather than surfacing a driver error, because the guard probe and the insert happen inside one `BEGIN IMMEDIATE` transaction that takes the write lock at the top and holds it to commit."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::two_handles_racing_one_condition_yield_one_winner_and_one_rejection — the losing attempt asserted ConditionViolated and not Store, mirroring the Attempt::Rejected / Attempt::Failed split at crates/happenstance-testkit/src/concurrency.rs:214-231"

- id: AC-005
  criterion: "GIVEN an application author who modelled one consistency boundary as several guards, WHEN `append` evaluates an `AppendCondition`, THEN a guard is violated only by a match at a position **strictly greater** than its `after`, a guard whose `after` is `None` is violated by any match at all, any one violated guard refuses the whole append, and no event of the batch being written is ever evaluated against the batch's own condition."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::guard_after_is_exclusive_at_the_boundary, ::a_guard_without_after_sees_the_whole_log, ::any_violated_guard_refuses_the_whole_batch, ::a_batch_never_conflicts_with_itself (ES-21, spec/SPECIFICATION.md:3516)"

- id: AC-006
  criterion: "GIVEN an application author who records the position their own write landed at, WHEN `append` succeeds while other connections are committing, THEN the returned `SequencePosition` is the one assigned to the last event of **this** batch in slice order — not the store head, and never assumed to be a predecessor plus one."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::append_returns_the_callers_own_last_position (with a second handle committing in between) and ::batch_positions_follow_slice_order — strict ascent, no literal position values"

- id: AC-007
  criterion: "GIVEN an application author whose second reader identifies an event across stores and whose sync runner must reject a duplicate on ingest, WHEN a batch is appended, THEN every row carries `origin_store` = this store's persisted `StoreId`, `origin_position` = its own assigned position and a `recorded_at` stamped exactly once here, its `event_tag` rows carry the covering `event_type`, and `tag_cardinality` is incremented for each tag written."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::every_appended_row_carries_its_identity_and_stamp and ::tag_cardinality_is_maintained_by_append — asserted through the raw second connection"

- id: AC-008
  criterion: "GIVEN an adapter author who declared a batch ceiling above the specification's 128-event floor, WHEN a batch at exactly that ceiling is appended, THEN it lands whole in one transaction — the insert is split into parameter-budget-sized statements, the transaction is not split, and no partially-applied batch is ever observable."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::a_batch_at_the_declared_ceiling_lands_whole (full ceiling at maximum tags per event) and ::a_failure_mid_batch_leaves_nothing (failure forced after the first chunk, zero rows asserted)"

- id: AC-009
  criterion: "GIVEN an adapter author running a suite that has no watchdog anywhere in it by design, WHEN several connections contend for the write lock, THEN contention is a bounded **wait** rather than an `AppendError::Store`, because the finite busy timeout `schema-migration-and-identity` configured is consumed rather than replaced — and no timeout, watchdog, `sleep` or retry loop is introduced anywhere in this diff."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/append.rs::contention_waits_rather_than_erroring, plus the reviewer check `rg -n \"timeout|sleep|retry\" crates/happenstance-sqlite/` over the diff finding nothing added around a test (testing brief §4)"

- id: AC-010
  criterion: "GIVEN an evaluator reading this adapter's public surface in one sitting, WHEN they open `SqliteEventStore`, THEN the three ceilings are documented public constants stating their unit and that they are facts about this adapter, `append`'s rustdoc carries an `# Errors` section naming conditions rather than error types, `crates/happenstance-sqlite/tests/shapes.rs` still holds, the feature powerset still compiles clean — and nothing beyond this adapter has been decided: no `StoreLimit` variant added, no item added to `happenstance-core`, no `spec/SPECIFICATION.md` edit, no ADR authored, and CF-40's clause home still recorded as open."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/append.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs via `cargo test -p happenstance-sqlite --test shapes`; `cargo xtask affected --base main` (docs + clippy -D warnings) and `cargo xtask ci --fast` (feature powerset); reviewer diff check that crates/happenstance-core/**, crates/happenstance-testkit/**, spec/SPECIFICATION.md and .kb/open-questions/cf-40-fixture-limits-ownership.md are untouched"
```
