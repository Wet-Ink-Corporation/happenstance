---
item: "HS-S0050"
stage: report
created: "2026-08-19"
updated: "2026-08-19"
---

# Report — Append, head, contains_event_id and migrate against a real Durable Object

## Findings Ledger

**Outcome: ten of ten ACs satisfied. Nothing blocked, nothing deferred, no conformance rule
added or changed, no `.kb/` write, and no `[FROZEN]` clause touched.** Two `todo!()`s
remain in the file and both are `durable-object-read-path`'s; the scoped
`#![allow(clippy::todo)]` therefore stays, as the story map's AC-001 row provides for.

| AC | Result | Proved by | Mount point |
| --- | --- | --- | --- |
| AC-001 | satisfied | `write_path_tests::append_then_head_round_trips`; `cargo clippy … -D warnings` | `impl EventStore for CloudflareEventStore`, reached through `CloudflareEventStore::new(sql)` |
| AC-002 | satisfied | `::migrate_is_idempotent`, `::schema_carries_the_identity_columns` | `CloudflareEventStore::migrate` and `MIGRATION` |
| AC-003 | satisfied | `::store_id_is_read_back_on_a_second_handle`, `::store_id_is_not_reminted_per_handle` | `migrate`'s mint-and-read-back plus `store_id()`'s cache |
| AC-004 | satisfied | `::an_empty_batch_is_refused_before_the_condition` | `append`'s refusal order |
| AC-005 | satisfied | `::a_rejected_condition_writes_nothing`, `::a_unique_constraint_message_classifies_as_condition_violated`, `::the_error_type_has_no_condition_violated_variant` | `evaluate` and `classify_write` |
| AC-006 | satisfied | `::a_capacity_refusal_names_a_store_limit`, `::exactly_at_the_ceiling_is_accepted` | `Ceilings` + `check_ceilings` |
| AC-007 | satisfied | `::append_returns_the_last_written_position`, `::a_failed_batch_leaves_no_partial_rows` | `write_batch` |
| AC-008 | satisfied | `::head_of_an_empty_store_is_none`, `::head_is_the_highest_position_this_store_assigned` | `head` |
| AC-009 | satisfied | `::a_foreign_origin_is_not_a_member`, `::a_local_identity_is_a_member` | `contains_event_id` |
| AC-010 | satisfied | `::a_position_above_two_pow_53_is_reported_not_narrowed`, `::a_position_below_one_is_reported`, `::the_ceiling_itself_is_a_usable_position` | `decode_position` |

### Why these tests can fail

Six named wrong implementations were compiled into the adapter one at a time and the wasm32
suite re-run. Every one was rejected, and the mapping is the useful part of this report:

| Mutant | Rejected by |
| --- | --- |
| `contains_event_id` over `WHERE position = ?` | `a_foreign_origin_is_not_a_member` |
| emptiness and ceilings checked after the condition | `an_empty_batch_is_refused_before_the_condition`, `a_failed_batch_leaves_no_partial_rows`, `a_capacity_refusal_names_a_store_limit` |
| every throw flattened into `AppendError::Store` | `a_unique_constraint_message_classifies_as_condition_violated` |
| ceilings never checked before the write | `a_failed_batch_leaves_no_partial_rows`, `a_capacity_refusal_names_a_store_limit` |
| a widened integer truncated rather than reported | `a_position_above_two_pow_53_is_reported_not_narrowed` |
| a fresh incarnation per handle | `store_id_is_not_reminted_per_handle` |

### Two decisions a reviewer should look at

**The condition is probed rather than classified from a thrown message — and both paths
exist.** The spec's context pack describes reading the thrown `message` and classifying it,
which is the shape an append condition implemented as a unique index takes. This object is
single-threaded with exclusive storage, so a `SELECT max(position) … WHERE position > ?`
probe is exact, atomic, and *names the conflicting position*, which a thrown constraint
cannot. The probe therefore decides the ordinary case; `classify_write` — one private
function, one call site, the shape the implementation notes asked for — catches the other
one, so a `UNIQUE (origin_store, origin_position)` violation reaches the caller as
`ConditionViolated` rather than as a transport fault. The test drives that through `append`
with a real SQLite throw, not a message it wrote itself.

**There is no `BEGIN`, and the atomicity claim rests on the runtime rather than on a
transaction.** A Durable Object rejects transaction-control statements through `sql.exec()`.
It is not needed: `append`'s body contains no `.await` at all, so the object cannot yield
between the probe and the insert, and the implicit per-turn transaction covers the batch.
The two failures a caller can actually reach are decided before any statement runs. A
genuine mid-batch fault is `MID_BATCH_FAULT`'s; this story declares nothing and forecloses
nothing — a `CHECK` constraint or a one-shot trigger is still addable without a schema
change.

### What was added that the spec did not name

`crates/happenstance-cloudflare/src/query_sql.rs`, crate-private: one `Query` → SQL
translation with a single entry point, so the append probe and the read path cannot come to
disagree about what "matches" means. `happenstance-sqlite` reached the same conclusion for
itself and its module documentation says why a public API for it would be premature — one
implementor is not a spread. Two adapters now agreeing, independently, is evidence for a
later ADR rather than a reason to share code between adapters, which the dependency rule
forbids in any case.

### Deliberately not here

`read`, `render_read`, `decode_row` and the stream's state machine; the `Fixture`, the
Durable Object host and the conformance target; the three numeric ceilings and any
`MID_BATCH_FAULT` claim; the caller-visible error reconstruction test; every `.kb/` write.
`CloudflareEventStoreError` gained no `ConditionViolated` variant and is still genuinely
`!Send` — the four probes pass on the host and on `wasm32`.
