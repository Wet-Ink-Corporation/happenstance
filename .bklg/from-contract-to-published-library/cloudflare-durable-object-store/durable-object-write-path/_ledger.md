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
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `impl EventStore for CloudflareEventStore` (:145-196), reached through `CloudflareEventStore::new(sql)` (:70-87) and `migrate()` (:79-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::append_then_head_round_trips (plus `cargo clippy --workspace --all-targets --all-features -- -D warnings` and the four probe tests at crates/happenstance-cloudflare/src/lib.rs:137-259)"

- id: AC-002
  criterion: "GIVEN an adapter author whose Durable Object is evicted and re-created between requests, WHEN `migrate()` runs on every open, THEN the first call creates `event` (carrying `origin_store` and `origin_position`), `event_tag` and the incarnation row, every later call is a no-op, and the intended-schema comment at `crates/happenstance-cloudflare/src/event_store.rs:8-28` names exactly the columns the DDL creates."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `CloudflareEventStore::migrate()` (:79-87), the schema seam of the single construction root"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::migrate_is_idempotent; ::write_path_tests::schema_carries_the_identity_columns"

- id: AC-003
  criterion: "GIVEN a constrained-runtime developer holding two handles onto one Durable Object, WHEN the second handle opens the store and asks `contains_event_id` about an id the first minted seconds earlier, THEN the answer is `true` — the incarnation `StoreId` is minted once at first `migrate`, persisted through `exec`, and read back on every later open rather than re-minted per handle."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `migrate()` (:79-87) mints and persists; every `CloudflareEventStore::new(sql)` (:70-87) reads it back"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::store_id_is_read_back_on_a_second_handle; ::write_path_tests::store_id_is_not_reminted_per_handle"

- id: AC-004
  criterion: "GIVEN an application author whose retry loop branches on `AppendError::is_condition_violated`, WHEN they accidentally call `append(&[], Some(&condition))` against a store the condition matches, THEN they receive `AppendError::NoEvents` and never `ConditionViolated` — because an empty batch is refused before the condition is evaluated, so the loop terminates instead of retrying a batch that will still be empty next time."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) in the existing impl block"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::an_empty_batch_is_refused_before_the_condition"

- id: AC-005
  criterion: "GIVEN two writers racing to append against the same DCB condition on one object, WHEN one loses, THEN the loser receives `AppendError::ConditionViolated` — classified from the thrown value's `message` text *before* any `CloudflareEventStoreError` is constructed — the store is left byte-for-byte as it was, and `CloudflareEventStoreError` still carries no `ConditionViolated` variant."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) and the error type `CloudflareEventStoreError` (:89-143), which gains no variant"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_unique_constraint_message_classifies_as_condition_violated; ::write_path_tests::a_rejected_condition_writes_nothing; ::write_path_tests::the_error_type_has_no_condition_violated_variant"

- id: AC-006
  criterion: "GIVEN a sync runner that must tell \"this will never fit here, park it and tell a human\" from \"the disk is full, retry\", WHEN a batch crosses a capacity ceiling this store declares, THEN it is refused as `AppendError::ExceedsStoreLimit { limit, len }` naming the matching `StoreLimit` and the offending count — never `AppendError::Store`, never truncated, never clamped where a chunk was meant — and a batch at exactly the ceiling is accepted."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177), refusing before the insert against a ceiling seam `measured-store-limits` later fills in"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_capacity_refusal_names_a_store_limit; ::write_path_tests::exactly_at_the_ceiling_is_accepted"

- id: AC-007
  criterion: "GIVEN a developer appending a five-event batch that models one decision, WHEN `append` returns, THEN it returns the position of the **last** event in slice order, positions follow slice order, and either every event in the batch is visible or none is — the condition probe and the `INSERT … RETURNING position` have nothing awaited between them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::append` (:171-177) over the synchronous `SqlStorage::exec` seam (src/sql_storage.rs:1-12)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::append_returns_the_last_written_position; ::write_path_tests::a_failed_batch_leaves_no_partial_rows"

- id: AC-008
  criterion: "GIVEN an application author resuming a projection after the object was evicted, WHEN they call `head()` on an empty store and again after appending, THEN they get `None` and then the highest visible position, decoded through the one shared position decoder, with no `await` taken while the storage handle is held."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::head` (:179-185) and the shared position decoder landed by this story"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::head_of_an_empty_store_is_none; ::write_path_tests::head_is_the_highest_position_this_store_assigned"

- id: AC-009
  criterion: "GIVEN a replication ingest asking whether an event it just received is already stored here, WHEN the `EventId` carries a **foreign** `StoreId` at a position this store *did* assign, THEN `contains_event_id` answers `false`, and answers `true` only when both halves match — the query is over `origin_store` **and** `origin_position`, never over `position` alone."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — `EventStore::contains_event_id` (:187-195) over the identity columns `migrate()` creates"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_foreign_origin_is_not_a_member; ::write_path_tests::a_local_identity_is_a_member"

- id: AC-010
  criterion: "GIVEN a constrained-runtime developer on a store whose position column holds a value Workers SQL cannot round-trip through a JS number, WHEN `head` or `contains_event_id` decodes that row, THEN the caller receives `CloudflareEventStoreError::StoredPosition { raw }` — never a narrowed or truncated position, never a panic — so a ceiling that is real on this runtime is reported rather than silently passed."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the shared position decoder behind `head` (:179-185) and `contains_event_id` (:187-195), reporting `CloudflareEventStoreError::StoredPosition` (:135-142)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::write_path_tests::a_position_above_two_pow_53_is_reported_not_narrowed; ::write_path_tests::a_position_below_one_is_reported"
```
