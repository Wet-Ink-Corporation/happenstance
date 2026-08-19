---
item: "HS-S0050"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — Append, head, contains_event_id and migrate against a real Durable Object

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes specific to this story, because they change what counts as evidence:

- **The conformance rules are not this story's evidence.** The twelve rules named in the spec's
  Integration contract observe these same facts, but they execute only once
  `durable-object-host-and-fixture` and `every-rule-under-workerd` land. Every `verifying_test` below
  is therefore a test in this crate's own tree, which is what an implementer of *this* story can run.
- **The test module's `cfg` may move; its names may not.** If the `worker` swap stops
  `cargo test -p happenstance-cloudflare` from linking on the host, `write_path_tests` moves behind
  `#[cfg(all(test, target_arch = "wasm32"))]` and re-emits through `wasm_bindgen_test`
  (`_decomposition.md`, Testing brief Notes §2). The names below stay valid either way.

```yaml
- id: AC-001
  criterion: "GIVEN a constrained-runtime developer who wants an event store inside a Durable Object without hand-rolling one, WHEN they construct `CloudflareEventStore::new(state.storage().sql())`, call `migrate()` and then `append(&events, None)`, THEN real SQL executes and a `SequencePosition` comes back — no `todo!()` remains in `migrate`, `append`, `head` or `contains_event_id`, and the port shape is untouched (bare `EventStore`, no `#[async_trait]`, `read` still non-`async` with the stream at the top level)."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::append_then_head_round_trips` — the store is built through
    `CloudflareEventStore::new(sql)` (`crates/happenstance-cloudflare/src/event_store.rs:196-204`),
    `migrate()` (`:250`) applies the schema and `append` (`:589`) returns a real `SequencePosition` that
    `head` (`:622`) agrees with. All 33 wasm32 tests pass under `wasm-bindgen-test-runner`. `rg -n
    'todo!' crates/happenstance-cloudflare/src/event_store.rs` returns the two read-path sites only
    (`render_read`, `decode_row`). Port shape untouched: bare `EventStore`, no `#[async_trait]`, `read`
    still non-`async` with the stream at the top level, and the four `!Send` probes still pass on the
    host and on `wasm32`. `cargo clippy --workspace --all-targets --all-features -- -D warnings` green.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `impl EventStore for CloudflareEventStore` (:145-196), reached through `CloudflareEventStore::new(sql)` (:70-87) and `migrate()` (:79-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::append_then_head_round_trips (plus `cargo clippy --workspace --all-targets --all-features -- -D warnings` and the four probe tests at crates/happenstance-cloudflare/src/lib.rs:137-259)"

- id: AC-002
  criterion: "GIVEN an adapter author whose Durable Object is evicted and re-created between requests, WHEN `migrate()` runs on every open, THEN the first call creates `event` (carrying `origin_store` and `origin_position`), `event_tag` and the incarnation row, every later call is a no-op, and the intended-schema comment at `crates/happenstance-cloudflare/src/event_store.rs:8-28` names exactly the columns the DDL creates."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::migrate_is_idempotent` (second and third `migrate()` are no-ops, the
    incarnation is unchanged, and the schema still accepts writes) and
    `::schema_carries_the_identity_columns`, which appends and then reads `origin_store` and
    `origin_position` back out through raw SQL, asserting the blob is this object's incarnation and the
    integer is the position the row was actually given — the columns are proven writable and readable,
    not merely commented. The module's schema block (`crates/happenstance-
    cloudflare/src/event_store.rs:9-40`) and the DDL it documents (`:105-125`) name the same columns.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `CloudflareEventStore::migrate()` (:79-87), the schema seam of the single construction root"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::migrate_is_idempotent; ::write_path_tests::schema_carries_the_identity_columns"

- id: AC-003
  criterion: "GIVEN a constrained-runtime developer holding two handles onto one Durable Object, WHEN the second handle opens the store and asks `contains_event_id` about an id the first minted seconds earlier, THEN the answer is `true` — the incarnation `StoreId` is minted once at first `migrate`, persisted through `exec`, and read back on every later open rather than re-minted per handle."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::store_id_is_read_back_on_a_second_handle` — handle A migrates and
    appends, handle B opens the same object and answers `contains_event_id` `true` about the `EventId` A
    minted. `::store_id_is_not_reminted_per_handle` adds the three-handle case (including a handle that
    never migrated) and the negative control that two *different* objects do not share an incarnation.
    Mechanism: `migrate` writes `INSERT OR IGNORE INTO store_meta (k, v) VALUES (?, randomblob(16))` and
    then **reads the row back** (`event_store.rs:250-274`); `store_id()` (`:276`) caches what storage
    says rather than what this handle would have generated. RED: a `store_id()` that mints instead of
    reading is rejected by `store_id_is_not_reminted_per_handle` (mutation sweep, mutant 6).
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `migrate()` (:79-87) mints and persists; every `CloudflareEventStore::new(sql)` (:70-87) reads it back"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::store_id_is_read_back_on_a_second_handle; ::write_path_tests::store_id_is_not_reminted_per_handle"

- id: AC-004
  criterion: "GIVEN an application author whose retry loop branches on `AppendError::is_condition_violated`, WHEN they accidentally call `append(&[], Some(&condition))` against a store the condition matches, THEN they receive `AppendError::NoEvents` and never `ConditionViolated` — because an empty batch is refused before the condition is evaluated, so the loop terminates instead of retrying a batch that will still be empty next time."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::an_empty_batch_is_refused_before_the_condition` — the store already
    holds a matching event and the condition would be violated, so a store that evaluated the condition
    first answers `ConditionViolated` and sends the caller round the loop again. The order is fixed at
    `crates/happenstance-cloudflare/src/event_store.rs:589-600`: emptiness, then ceilings, then
    identity, then the condition probe, then the insert. RED: moving the emptiness check after the
    condition (mutant 2, `ConditionBeforeEmptinessStore`) is rejected by this test.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) in the existing impl block"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::an_empty_batch_is_refused_before_the_condition"

- id: AC-005
  criterion: "GIVEN two writers racing to append against the same DCB condition on one object, WHEN one loses, THEN the loser receives `AppendError::ConditionViolated` — classified from the thrown value's `message` text *before* any `CloudflareEventStoreError` is constructed — the store is left byte-for-byte as it was, and `CloudflareEventStoreError` still carries no `ConditionViolated` variant."
  satisfied: true
  evidence: >-
    Three tests. `event_store::write_path_tests::a_rejected_condition_writes_nothing` — a three-event
    batch against a matching condition is refused as `ConditionViolated` and the log still reads
    `["Existing"]`. `::a_unique_constraint_message_classifies_as_condition_violated` — a row already
    occupies the identity the next append would stamp itself with, so SQLite's own `UNIQUE constraint
    failed:` text is thrown out of `worker::SqlStorage::exec` and reaches the caller as
    `AppendError::ConditionViolated`, not `AppendError::Store`.
    `::the_error_type_has_no_condition_violated_variant` — an exhaustive `match` over
    `CloudflareEventStoreError`, so adding the variant stops the crate compiling. The classification is
    one private function called from one site (`classify_write`, `event_store.rs:686`), and it runs
    *before* `Self::Error` reaches the caller. RED: flattening the classifier to
    `AppendError::Store(error)` (mutant 3) is rejected by the second test.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) and the error type `CloudflareEventStoreError` (:89-143), which gains no variant"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_unique_constraint_message_classifies_as_condition_violated; ::write_path_tests::a_rejected_condition_writes_nothing; ::write_path_tests::the_error_type_has_no_condition_violated_variant"

- id: AC-006
  criterion: "GIVEN a sync runner that must tell \"this will never fit here, park it and tell a human\" from \"the disk is full, retry\", WHEN a batch crosses a capacity ceiling this store declares, THEN it is refused as `AppendError::ExceedsStoreLimit { limit, len }` naming the matching `StoreLimit` and the offending count — never `AppendError::Store`, never truncated, never clamped where a chunk was meant — and a batch at exactly the ceiling is accepted."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::a_capacity_refusal_names_a_store_limit` drives all three limits
    through a test-visible ceiling and asserts the exact `AppendError::ExceedsStoreLimit { limit, len
    }`: `EventDataLen`/9, `TagsPerEvent`/3, `EventsPerBatch`/4 — never `AppendError::Store`, never
    truncated. `::exactly_at_the_ceiling_is_accepted` proves the other direction with a batch sitting on
    all three ceilings at once, so `measured-store-limits` inherits a store that can pass rather than
    one that can only fail. The seam is `Ceilings` (`crates/happenstance-
    cloudflare/src/event_store.rs:147-176`), whose `UNMEASURED` value refuses nothing — this story
    states no public numeric limit. RED: mutant 4 (`ValidateAsYouWriteStore`, ceilings never checked) is
    rejected by both tests.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177), refusing before the insert against a ceiling seam `measured-store-limits` later fills in"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_capacity_refusal_names_a_store_limit; ::write_path_tests::exactly_at_the_ceiling_is_accepted"

- id: AC-007
  criterion: "GIVEN a developer appending a five-event batch that models one decision, WHEN `append` returns, THEN it returns the position of the **last** event in slice order, positions follow slice order, and either every event in the batch is visible or none is — the condition probe and the `INSERT … RETURNING position` have nothing awaited between them."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::append_returns_the_last_written_position` — a three-event batch,
    compared against the positions the store *actually assigned* (read back through raw SQL) and against
    slice order by event type; no literal position value appears anywhere.
    `::a_failed_batch_leaves_no_partial_rows` — a four-event batch whose **third** event crosses the
    ceiling leaves zero rows, which rejects a store that validates as it writes. `write_batch` (`:454`)
    issues one `INSERT … RETURNING position` per event because SQLite leaves the row order of a
    multi-row `RETURNING` undefined.
    CORRECTED AT SLICE REVIEW, and the correction is the substance of this row. The all-or-none half was
    argued in prose and guarded by nothing: `a_failed_batch_leaves_no_partial_rows` refuses on the
    pre-flight ceiling check, where **zero** statements have run, so it observes a batch that never
    started rather than one that started and stopped. The claim at the old `event_store.rs:68-83` — that
    "the runtime's implicit transaction covers the batch" — was **false**, and the adapter was the shape
    it named: N `INSERT`s, then tag rows, then the identity `UPDATE`, with every throw converted to
    `Err(…)` and returned normally, which is exactly when a Durable Object commits the turn's writes.
    Three committed guards now observe it, and all three failed against the previous implementation
    (RED recorded: 3 failed / 74 passed): `::a_batch_that_throws_after_its_first_row_leaves_nothing_behind`
    (`:1851`) arms a throw on `INSERT INTO event_tag` and asserts both `event` and `event_tag` are empty;
    `::a_batch_that_throws_while_stamping_identity_leaves_nothing_behind` (`:1885`) does the same at the
    `UPDATE … SET origin_store`; `::a_discarded_batch_does_not_wedge_or_rewind_the_store` (`:1921`)
    shows the next append lands and does **not** reuse the discarded position. The fix is explicit
    compensation rather than a weakened claim — `write_batch` (`:454`) discards the range the batch was
    assigned through `discard_from` (`:578`), exact because nothing is awaited mid-batch — and the
    remaining failure is reported rather than hidden: `CloudflareEventStoreError::PartialBatch` (`:705`)
    carries both the original cause and the reason the discard could not run, reached by
    `::a_batch_whose_discard_also_fails_reports_both_failures` (`:1960`). `SAVEPOINT` was not an
    alternative: a Durable Object rejects transaction control through `sql.exec()`. The module
    documentation now says isolation is not atomicity instead of claiming the runtime provides it
    (`crates/happenstance-cloudflare/src/event_store.rs:68-95`). All 81 cases execute in the gate.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) over the synchronous `SqlStorage::exec` seam (src/sql_storage.rs:1-12)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::append_returns_the_last_written_position; ::write_path_tests::a_failed_batch_leaves_no_partial_rows; ::write_path_tests::a_batch_that_throws_after_its_first_row_leaves_nothing_behind; ::write_path_tests::a_batch_that_throws_while_stamping_identity_leaves_nothing_behind; ::write_path_tests::a_batch_whose_discard_also_fails_reports_both_failures; ::write_path_tests::a_discarded_batch_does_not_wedge_or_rewind_the_store"

- id: AC-008
  criterion: "GIVEN an application author resuming a projection after the object was evicted, WHEN they call `head()` on an empty store and again after appending, THEN they get `None` and then the highest visible position, decoded through the one shared position decoder, with no `await` taken while the storage handle is held."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::head_of_an_empty_store_is_none` and
    `::head_is_the_highest_position_this_store_assigned`, the second comparing `head` against the
    positions two `append` calls returned rather than against a number, and asserting monotonicity.
    `head` is `SELECT max(position) AS position FROM event` decoded through the one shared decoder
    (`decode_position`, `crates/happenstance-cloudflare/src/event_store.rs:730`), and its body takes no
    `await` while the storage handle is held — the `Sync` bound the provided form would have wanted is
    never needed.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::head` (:179-185) and the shared position decoder landed by this story"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::head_of_an_empty_store_is_none; ::write_path_tests::head_is_the_highest_position_this_store_assigned"

- id: AC-009
  criterion: "GIVEN a replication ingest asking whether an event it just received is already stored here, WHEN the `EventId` carries a **foreign** `StoreId` at a position this store *did* assign, THEN `contains_event_id` answers `false`, and answers `true` only when both halves match — the query is over `origin_store` **and** `origin_position`, never over `position` alone."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::a_foreign_origin_is_not_a_member` — every byte of the store's own
    `StoreId` flipped, at a position this store *did* assign, reads `false`.
    `::a_local_identity_is_a_member` is the positive half plus a never-assigned position, so the pair
    rejects both a store that answers `true` to everything and one that answers `false`. The query is
    `WHERE origin_store = ? AND origin_position = ? LIMIT 1` (`crates/happenstance-
    cloudflare/src/event_store.rs:643-658`). RED: mutant 1 (`PositionOnlyMembershipStore`, `WHERE
    position = ?`) is rejected by `a_foreign_origin_is_not_a_member`.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::contains_event_id` (:187-195) over the identity columns `migrate()` creates"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_foreign_origin_is_not_a_member; ::write_path_tests::a_local_identity_is_a_member"

- id: AC-010
  criterion: "GIVEN a constrained-runtime developer on a store whose position column holds a value Workers SQL cannot round-trip through a JS number, WHEN `head` or `contains_event_id` decodes that row, THEN the caller receives `CloudflareEventStoreError::StoredPosition { raw }` — never a narrowed or truncated position, never a panic — so a ceiling that is real on this runtime is reported rather than silently passed."
  satisfied: true
  evidence: >-
    `event_store::write_path_tests::a_position_above_two_pow_53_is_reported_not_narrowed` seeds
    `9007199254740993` as a SQL **literal** — binding it would already have widened it on the way in —
    and `head()` returns `CloudflareEventStoreError::StoredPosition`.
    `::a_position_below_one_is_reported` seeds `0` and gets the same variant. Both assert on the
    **variant**, never on a number. `::the_ceiling_itself_is_a_usable_position` is the control that
    stops the decoder passing by refusing everything. The mechanism is two-sided:
    `sql_storage::decode_value` hands a stored integer past the JS safe range back as `SqlValue::Real`
    rather than narrowing it, and `decode_position` (`crates/happenstance-
    cloudflare/src/event_store.rs:730-753`) reports that as `StoredPosition`. RED: mutant 5
    (`NarrowingPositionStore`, truncating the `Real`) is rejected by the first test.
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the shared position decoder behind `head` (:179-185) and `contains_event_id` (:187-195), reporting `CloudflareEventStoreError::StoredPosition` (:135-142)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_position_above_two_pow_53_is_reported_not_narrowed; ::write_path_tests::a_position_below_one_is_reported"
```
