---
item: "HS-S0051"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — A lazy read that is still one sample: ADR-0011's ceiling-and-page

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "A replay exists at all. GIVEN an adapter author with a Durable Object holding events written through this crate's one construction root, WHEN they call EventStore::read with Query::all() and default ReadOptions and drain the returned stream to exhaustion, THEN they receive every stored event as a SequencedEvent in ascending position order — no todo!() is reached, nothing panics, and the replay half of the backbone activity \"store and replay events inside a Durable Object\" is real rather than an instrument. Traces project AC-001."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path::read_all_replays_every_appended_event and ::read_reaches_the_store_only_through_new (cargo test -p happenstance-cloudflare); plus the clippy -D warnings step of `cargo xtask ci --fast` over the three read-path items"

- id: AC-002
  criterion: "The shape a caller depends on survives the diff. GIVEN a caller writing generic code that binds EventStore — the weaker bound that accepts both flavours — and spawns work over the stream it gets back, WHEN this story's read compiles, THEN read is still a non-async method returning impl Stream at the top level with no + Send and no #[async_trait] anywhere, so the derived Send flavour keeps its stream bound; the state machine stays hand-written and the stream stays !Send by its fields. Traces project AC-001; enforces CLAUDE.md constraints 1, 3 and 4 (ADR-0001, ADR-0008)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/lib.rs:180-259 (the_store_is_not_send, the_stream_is_not_send, the_error_type_is_not_send, the_probe_is_not_vacuous) and crates/happenstance-cloudflare/src/send_shape.rs; plus crates/happenstance-core/src/memory.rs::send_flavour_stream_is_send_in_generic_code and ::spawns_from_generic unedited and green (cargo test -p happenstance-cloudflare -p happenstance-core)"

- id: AC-003
  criterion: "One read is one sample, or the failure is escalated rather than absorbed. GIVEN a constrained-runtime developer replaying a stream inside a re-entrant Durable Object while another future appends to the same object, WHEN they poll that read to exhaustion across several pages, THEN the events they see are exactly the events that existed at one moment no later than the first poll — a ceiling H is captured then and every statement after the first is bounded by position <= H (>= H under backwards) — so nothing appended mid-drain appears part-way through; AND if that ceiling cannot be afforded against the real SqlStorageCursor, the compiled reason is recorded and escalated as ES-11/ES-12 falsifier evidence (ADR-0023 material), never absorbed as a declined capability and never softened into a weaker promise. Traces project AC-007(a); discharges ES-11."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path::read_is_stable_under_an_interleaved_append and ::the_ceiling_is_captured_no_later_than_the_first_poll, with the ceiling-less paging negative control (cargo test -p happenstance-cloudflare); standing later as read_result_is_stable_under_concurrent_append (crates/happenstance-testkit/src/registry.rs:213, crates/happenstance-testkit/src/suite.rs:5592)"

- id: AC-004
  criterion: "A multi-item DCB query is one sample, not several. GIVEN an application author whose decision boundary is expressed as a Query with several QueryItems, WHEN the adapter issues more than one statement to serve it, THEN all items share the one ceiling — an event matching an earlier item that lands between two statements has a position above H and is excluded by the same predicate that bounds every other statement — so the caller never sees a boundary assembled from two different moments, and no second isolation mechanism is introduced to achieve it. Traces project AC-007(a); discharges ES-12 (which reduces to ES-10 plus the ceiling)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path::all_items_of_one_query_share_one_ceiling, including the assertion that exactly one ceiling capture occurs per read (cargo test -p happenstance-cloudflare); standing later as query_items_share_one_snapshot (crates/happenstance-testkit/src/registry.rs:214, crates/happenstance-testkit/src/suite.rs:5696)"

- id: AC-005
  criterion: "A live replay never wedges the object. GIVEN a Durable Object that is single-threaded but re-entrant, WHEN a read stream is suspended mid-drain and an append is issued on the same handle — or a read is issued while an append on that handle is itself suspended — THEN both complete: the stream holds no RefCell borrow and no live cursor across a poll boundary, and a genuinely re-entered borrow is reported as SqlError::AlreadyBorrowed rather than panicking inside borrow_mut. Traces project AC-007(a); honours ES-36 and the read-during-suspended-append obligation no clause states (spec/SPECIFICATION.md:4178-4250)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path::a_live_read_stream_does_not_block_an_append_on_one_handle, ::a_read_issued_during_a_suspended_append_completes and ::re_entrancy_is_reported_not_panicked, with the held-cursor negative control (cargo test -p happenstance-cloudflare); standing later as a_live_read_stream_does_not_block_an_append (crates/happenstance-testkit/src/registry.rs:210)"

- id: AC-006
  criterion: "The window the caller asked for is the window they get — even though paging exists underneath. GIVEN an application author who has expressed a decision boundary as a DCB query with ReadOptions, WHEN they read, THEN the algebra is rendered, not approximated — types OR within an item, tags AND within an item, items OR across the query, supersets match and partial overlaps do not, an item naming no tags still matches an untagged event, and no event is yielded twice across items; from is a threshold and not a seek (a position nothing occupies yields the next match above it, or below it under backwards, and never an error); to is inclusive and composes with the internal ceiling as the tighter of the two, with from the higher bound and to the lower under backwards; limit is spent on matched events after ordering — a page that scanned excluded rows must not consume it — and limit(0) yields nothing; and a store nobody has written to yields an empty stream and no error, with the options exercised against it rather than short-circuited. Traces project AC-001; satisfies ES-9, ES-13, ES-14, ES-16 on this runtime."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path — the table-driven tests mirroring crates/happenstance-testkit/src/registry.rs:112-138 (read_from_is_inclusive, read_from_a_gap_position, read_backwards_from_with_limit, read_to_is_inclusive, read_from_and_to_bound_a_closed_window, read_to_under_backwards_bounds_the_older_end, read_limit_applies_after_filtering, read_backwards_limit_applies_after_filtering, read_limit_zero_yields_nothing, limit_applies_across_items_not_per_item, query_union_is_item_concatenation, reading_an_empty_store_yields_nothing) plus the null-head ceiling negative control (cargo test -p happenstance-cloudflare)"

- id: AC-007
  criterion: "Nothing is lost or reinterpreted on the way out. GIVEN a stored row this runtime cannot faithfully hand back — a position outside 1..=2^53 because Workers SQL widens integers through a JS number, a column count that disagrees with the SELECT, an undecodable SqlValue, or a stored event type that fails validation — WHEN the caller polls the stream, THEN they receive the corresponding CloudflareEventStoreError (StoredPosition, RowShape, ColumnType, StoredEventType) as an error item on the stream, never a silently narrowed position and never a panic; AND for every row that is representable, data and metadata travel as opaque blobs that are never inspected, parsed or re-encoded, with metadata: None and Some(<empty>) still two distinguishable values and no serde entering this crate on the read path. Traces project AC-007(b) (read side only); honours ADR-0003."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-cloudflare/src/event_store.rs — the `impl EventStore for CloudflareEventStore` block (:145-196), reached through the crate's single construction root `CloudflareEventStore::new(sql)` (:70-87)"
  verifying_test: "crates/happenstance-cloudflare/src/event_store.rs::read_path::decode_row_reports_row_shape, ::decode_row_reports_column_type, ::decode_row_reports_stored_event_type, ::decode_row_reports_stored_position_at_the_boundary (condition constructed, never a bare literal position) and ::metadata_none_and_some_empty_stay_distinguishable (cargo test -p happenstance-cloudflare); dependency surface observed by the --no-default-features and cargo hack steps of cargo xtask ci"
```
