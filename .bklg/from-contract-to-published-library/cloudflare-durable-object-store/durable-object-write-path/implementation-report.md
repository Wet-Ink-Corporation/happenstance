---
item: "HS-S0050"
stage: implement
created: "2026-08-19"
updated: "2026-08-19"
---

# Implementation Report — Append, head, contains_event_id and migrate against a real Durable Object

**All ten ACs are satisfied.** Four `todo!()` bodies became four real ones, the schema
gained ADR-0014's identity columns and an incarnation row, and the three caller-visible
channels a failure can travel in are decided in one place each. Nothing is blocked and
nothing is deferred; the two remaining `todo!()`s in the file are `render_read` and
`decode_row`, which are `durable-object-read-path`'s.

The delta is small in SQL and large in classification, exactly as the spec predicted. What
took the thinking was **which channel each failure travels in**, and the answer is three
decisions rather than one:

- A DCB conflict is decided by a `SELECT max(position) … WHERE position > ?` probe *before*
  any row is written, so the ordinary rejection path never constructs a
  `CloudflareEventStoreError` at all — which is what makes the adapter's missing
  `ConditionViolated` variant a structural fact rather than a gap.
- A capacity refusal is decided *before any SQL runs*, because a refusal classified after
  the fact cannot say which ceiling was crossed and CF-40 requires it to name one.
- Everything else, object-wide storage exhaustion included, is `AppendError::Store`. The one
  exception is a thrown constraint violation escaping the insert — the `UNIQUE (origin_store,
  origin_position)` index is a second expression of the same conflict — and it is routed by
  a single private classifier (`classify_write`) called from exactly one site.

## TDD Evidence

Every test executes on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`, against a
real Durable Object `SqlStorage` backed by real SQLite (`src/test_object.rs`, landed by
`worker-binding-layer`). 33 tests, 0 failures.

**The Red step was run as a mutation sweep**, because writing a test after the body it
covers proves nothing on its own. Six named wrong implementations were compiled into the
adapter one at a time and the suite re-run; each was rejected, and each rejection names the
test that caught it:

| Mutant | Rejected by |
| --- | --- |
| `PositionOnlyMembershipStore` — `contains_event_id` over `WHERE position = ?` | `a_foreign_origin_is_not_a_member` |
| `ConditionBeforeEmptinessStore` — emptiness and ceilings checked after the condition | `an_empty_batch_is_refused_before_the_condition`, `a_failed_batch_leaves_no_partial_rows`, `a_capacity_refusal_names_a_store_limit` |
| `FlatteningClassifier` — every throw becomes `AppendError::Store` | `a_unique_constraint_message_classifies_as_condition_violated` |
| `ValidateAsYouWriteStore` — the ceilings never checked before the write | `a_failed_batch_leaves_no_partial_rows`, `a_capacity_refusal_names_a_store_limit` |
| `NarrowingPositionStore` — a widened integer truncated rather than reported | `a_position_above_two_pow_53_is_reported_not_narrowed` |
| `ReMintingStore` — a fresh incarnation per handle instead of a read-back | `store_id_is_not_reminted_per_handle` |

| AC | Test | Red → Green |
| --- | --- | --- |
| AC-001 | `append_then_head_round_trips` | RED on `todo!("phase 9: …")` in `migrate`. GREEN through `new(sql)` → `migrate()` → `append()` → `head()` |
| AC-002 | `migrate_is_idempotent`, `schema_carries_the_identity_columns` | RED: the schema had no `origin_store`/`origin_position` at all, so the second test could not even read the columns |
| AC-003 | `store_id_is_read_back_on_a_second_handle`, `store_id_is_not_reminted_per_handle` | RED via mutant 6 |
| AC-004 | `an_empty_batch_is_refused_before_the_condition` | RED via mutant 2 |
| AC-005 | `a_rejected_condition_writes_nothing`, `a_unique_constraint_message_classifies_as_condition_violated`, `the_error_type_has_no_condition_violated_variant` | RED via mutant 3; the third is a compile-time guard rather than a runtime one |
| AC-006 | `a_capacity_refusal_names_a_store_limit`, `exactly_at_the_ceiling_is_accepted` | RED via mutants 2 and 4 |
| AC-007 | `append_returns_the_last_written_position`, `a_failed_batch_leaves_no_partial_rows` | RED via mutant 4 |
| AC-008 | `head_of_an_empty_store_is_none`, `head_is_the_highest_position_this_store_assigned` | RED on `todo!()` in `head` |
| AC-009 | `a_foreign_origin_is_not_a_member`, `a_local_identity_is_a_member` | RED via mutant 1 |
| AC-010 | `a_position_above_two_pow_53_is_reported_not_narrowed`, `a_position_below_one_is_reported`, `the_ceiling_itself_is_a_usable_position` | RED via mutant 5; the third is the control that stops the decoder passing by refusing everything |

## Commits

- `durable-object-write-path` — see the story checkpoint commit carrying
  `Story: cloudflare-durable-object-store/durable-object-write-path`.

## Changes

| Path | Shape of the change |
| --- | --- |
| `crates/happenstance-cloudflare/src/event_store.rs` | The four bodies; the schema (`MIGRATION`) with the identity columns, the `event_type` covering column and `store_meta`; the incarnation mint-and-read-back; `Ceilings` and `check_ceilings`; `evaluate`; `write_batch` and `write_tag_rows`; `classify_write`; the shared `decode_position`; `encode_tags`; `now()` off the Durable Object's own clock; and `mod write_path_tests` |
| `crates/happenstance-cloudflare/src/query_sql.rs` | **New, crate-private.** One `Query` → SQL translation, reached through one entry point, so the append probe and (later) the read path cannot disagree about what "matches" means |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | `SqlError::internal` widened to `pub(crate)` so the event store can report an adapter-side failure with no JS value behind it |
| `crates/happenstance-cloudflare/src/lib.rs` | `mod query_sql;` |
| `.bklg/.../durable-object-write-path/_ledger.md` | Ten rows flipped with cited evidence |

## Gates

| Gate | Result |
| --- | --- |
| `cargo fmt --all --check` | green |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | green |
| `cargo clippy -p happenstance-cloudflare --all-targets --target wasm32-unknown-unknown -- -D warnings` | green |
| `cargo test -p happenstance-cloudflare --target wasm32-unknown-unknown --lib` | **33 passed, 0 failed** |
| `cargo test -p happenstance-cloudflare` (host) | 4 unit + 2 doctests, all pass |
| `cargo xtask affected --base main` | `affected gate passed` |
| `cargo xtask ci --fast` | `all required checks passed` |
| `cargo xtask spec-trace` | `traceability: no problems found` |
| `cargo xtask lint-constitution` | `27 atoms, all consistent` |

## Notes

**Atomicity is structural here, and it is worth saying why there is no `BEGIN`.** A Durable
Object rejects transaction-control statements through `sql.exec()` and offers a callback
form instead, which would put a second seam in the constructor. It is not needed: `append`'s
body contains **no `.await` at all**, so the object cannot yield to its event loop between
the condition probe and the insert, and the runtime's implicit per-turn transaction covers
the batch. The two failures a caller can actually reach — an empty batch and a crossed
ceiling — are both decided before any statement runs, which is what
`a_failed_batch_leaves_no_partial_rows` observes. A genuine mid-batch fault is
`MID_BATCH_FAULT`'s, this story declares nothing, and the schema forecloses nothing: a
`CHECK` constraint or a trigger armed for one write is still addable without a schema
change.

**The condition is probed, not classified from a thrown message — and both paths exist.**
The spec's context pack describes classification from the thrown `message`, on the model of
an append condition implemented as a unique index. This object is single-threaded with
exclusive storage, so a `SELECT max(position)` probe is exact, atomic and *names the
conflicting position*, which a thrown constraint cannot. So the probe decides the ordinary
case and `classify_write` catches the other one: a `UNIQUE (origin_store, origin_position)`
violation — the shape a future replication ingest produces — reaches the caller as
`ConditionViolated` rather than as a transport fault, and
`a_unique_constraint_message_classifies_as_condition_violated` drives exactly that through
`append` with a real SQLite throw rather than a message the test wrote.

**`SELECT max(position)` and not `EXISTS`.** A guard asks "is there anything matching after
this boundary", which reads as an existence question and is an inequality on the *highest*
match. One `max()` answers both halves at once — whether the condition is violated and by
which event — so the rejection path needs no second query, and that is the path a DCB
command loop takes every time it loses a race.

**The 2^53 ceiling is two-sided, and the first side is in the binding layer.**
`sql_storage::decode_value` hands a stored integer past `Number.MAX_SAFE_INTEGER` back as
`SqlValue::Real` rather than narrowing it, and `decode_position` reports that as
`StoredPosition`. The test seeds `9007199254740993` as a SQL **literal** rather than a
binding, because binding it would have widened it on the way *in* and tested nothing.

**One shared decoder, landed here as the spec asked.** `decode_position` is the single
place the `< 1` and `> 2^53` rules are stated; `head` and `contains_event_id`'s neighbours
call it now and `durable-object-read-path`'s `decode_row` consumes it rather than writing a
second.

**`query_sql` is new and crate-private, and that is a deliberate repeat of a decision
`happenstance-sqlite` already took for itself.** `happenstance-core` exposes no query-plan
API and should not mint one until two unlike storage shapes independently need the same
decomposition; one implementor is not a spread. The two adapters reached the same
conclusion separately, which is evidence for a later ADR rather than a reason to share code
between adapters now — the dependency rule forbids that anyway.

**`recorded_at` comes from `Date.now()`**, the Durable Object's own clock, which the runtime
pins to the last I/O. Two events accepted in one turn therefore carry the same millisecond,
which is exactly why `RecordedAt` is not an ordering key.

**One clippy-driven rename.** `Ceilings`' fields are `event_data_len` / `tags_per_event` /
`events_per_batch` rather than `max_*`: `clippy::struct_field_names` rejects a common
prefix, and the type name already says they are ceilings.

**Out of scope and untouched, as the PR boundary says:** `read`, `render_read`,
`decode_row` and `SqlRowStream`'s state machine; the `Fixture`; the numeric ceilings; the
caller-visible reconstruction test; every `.kb/` write. The scoped
`#![allow(clippy::todo)]` stays, because this story is not the one that lands the crate's
last `todo!()`.

## Slice-review repair (`real-worker-bindings`)

**AC-007's all-or-none half was argued in prose and guarded by nothing — and the prose was
wrong.** `a_failed_batch_leaves_no_partial_rows` refuses on the pre-flight ceiling check,
where zero statements have run, so it observes a batch that never *started* rather than one
that started and stopped: it could not see the property at all. And the mechanism the crate
documented — "the runtime's implicit transaction covers the batch" — does not hold for this
adapter's own shape. `write_batch` issues N `INSERT`s, then the tag rows, then the identity
`UPDATE`, and converts every throw into `Err(…)` and returns **normally**, which is exactly
when a Durable Object commits its turn's writes. Isolation is not atomicity, and reading the
first as the second is what happened here.

Three committed guards now observe it, and all three failed against the shipped
implementation before it changed (RED recorded: 3 failed / 74 passed) — a throw armed on
`INSERT INTO event_tag`, a throw armed on `UPDATE event SET origin_store`, and the store's
usability afterwards.

**The fix is real, not a weakened claim.** `write_batch` compensates explicitly, discarding
the position range the failed batch was assigned through `discard_from`. It is exact rather
than best-effort because nothing is awaited mid-batch, so no row outside the batch can sit
in the range; and it does not reset the counter, so a discarded batch leaves a *gap* —
positions are never reused, because an `EventId` is `(store, position)` and a reused
position is two events wearing one identity. `SAVEPOINT` was not an alternative: a Durable
Object rejects transaction control through `sql.exec()`, which is the same reason there is
no `BEGIN`/`COMMIT` here.

**The one remaining failure is reported rather than hidden.**
`CloudflareEventStoreError::PartialBatch` carries both the original cause and the reason the
discard could not run, and it is the only failure of `append` after which a retry is unsafe.
It is reachable rather than defensive — an object out of room fails the `DELETE` as readily
as the `INSERT` — and `a_batch_whose_discard_also_fails_reports_both_failures` reaches it.
That needed one small extension to the test shim: `armThrow` takes an optional count, so a
single arming can fire on both statements.

The module documentation at `event_store.rs:68-95` now says what is true.
