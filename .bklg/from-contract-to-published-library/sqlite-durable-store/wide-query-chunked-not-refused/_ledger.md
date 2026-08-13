---
item: HS-S0039
stage: implement
created: 2026-08-12T13:46:37.634Z
updated: 2026-08-12T13:46:37.634Z
---

# Acceptance ledger — A wide Query is chunked and merged, never refused

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
  criterion: "GIVEN an application author whose DCB decision model has grown to the 128 query items every store must evaluate (crates/happenstance-core/src/limits.rs:32) and beyond, WHEN they call `read` on `SqliteEventStore` with that `Query`, THEN they receive every matching event — not a `SqliteEventStoreError`, not an empty stream, and not a silently truncated prefix — and there is no query-width refusal path anywhere in the crate to reach: no `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant, no early `return Err` keyed on item or arm count."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::a_query_at_the_guaranteed_minimum_item_count_is_served and ::a_query_far_above_the_minimum_is_served, plus the negative guard `rg -n \"MAX_QUERY_ITEMS|QueryItems\" crates/happenstance-sqlite/src` returning nothing; run by cargo xtask affected --base main (.redkiln/config.yaml:40)"

- id: AC-002
  criterion: "GIVEN an adapter author who needs to know the merge *ran* rather than that the suite was green, WHEN they run this crate's tests, THEN at least one test issues a `Query` wide enough that `ceil(arms / N) > 1` and observes in-crate that the boundary was crossed — the chunk width is reachable from the test rather than an unreadable private literal, and the test fails if the read were served by a single statement. A merge that never executes is dead code behind a green suite, which is the failure mode this initiative exists to retire."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::a_wide_query_actually_crosses_the_chunk_boundary — asserts the observed chunk count > 1 and would fail at == 1; cargo test -p happenstance-sqlite --test wide_query"

- id: AC-003
  criterion: "GIVEN an application author whose wide query has two items that both match the same event, and whose items were written in whatever order the domain suggested, WHEN they read, THEN they get the union of every item's matches — never only the first chunk's — with each event yielded exactly once, in ascending position order regardless of how items were partitioned into chunks or ordered in the `Query`, and with no two items collapsed because their type lists looked alike (VT-31 licenses reorder/dedup only where the match set is preserved)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::overlapping_arms_yield_each_event_once, ::item_order_does_not_change_the_result_set, ::the_union_is_every_chunk_not_the_first, ::items_with_similar_types_but_different_tags_are_not_collapsed; sibling rules crates/happenstance-testkit/src/suite.rs:553, :636, :710, :780 once sqlite-fixture-and-whole-suite mounts the fixture"

- id: AC-004
  criterion: "GIVEN an application author who asked for `ReadOptions::limit = n` over a wide query, in either direction, WHEN they read, THEN they receive exactly `min(n, matches)` events counted after filtering and across the merged output — never `n` per chunk and never `n` per item — and the backwards read returns the highest `n` matching positions rather than whatever a per-chunk `LIMIT` happened to keep. ES-14 is [FROZEN] and names the per-statement-`LIMIT` adapter as its rejected implementation (spec/SPECIFICATION.md:3111-3145)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::limit_applies_across_chunks_not_per_chunk, ::limit_applies_after_filtering_forwards, ::limit_applies_after_filtering_backwards; sibling rules crates/happenstance-testkit/src/suite.rs:1402 and :1052 with its backwards mirror (crates/happenstance-testkit/src/registry.rs:130-131)"

- id: AC-005
  criterion: "GIVEN an application author replaying a wide query while another writer is appending, WHEN the read spans several chunk statements and several `spawn_blocking` hops, THEN every one of those statements is bounded by the same snapshot ceiling, `ReadOptions::to`, direction and `resume_from` — so an event appended between chunk 1 and chunk k, or between hop 1 and hop 2, is above the ceiling and absent from the whole read. ES-12 is discharged by ES-11's ceiling and by no second mechanism (spec/SPECIFICATION.md:3020-3025)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::a_concurrent_append_between_chunks_is_not_seen and ::a_concurrent_append_between_pages_is_not_seen; sibling rule query_items_share_one_snapshot (crates/happenstance-testkit/src/suite.rs:5696, registered crates/happenstance-testkit/src/registry.rs:214)"

- id: AC-006
  criterion: "GIVEN an application author paging a wide replay whose page boundary lands in the middle of a chunk's rows, WHEN the stream fetches the next page, THEN no row is repeated and none is dropped: `resume_from` keeps its inclusive sense, per-chunk resume state stays inside one hop, and `advance()` folds only the merged page into `resume_from` and `remaining`. The historical `resume_after` bug — an inclusive seed advanced by an exclusive step, re-reading one row per page boundary — must not return in per-chunk form (crates/happenstance-sqlite/src/event_store.rs:313-330)."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — new integration target, reached through the public SendEventStore::read (crates/happenstance-sqlite/src/event_store.rs:198-215)"
  verifying_test: "crates/happenstance-sqlite/tests/wide_query.rs::paging_a_wide_query_repeats_no_row_and_drops_none (more than PAGE_SIZE = 512 matching events across a multi-chunk query, compared against the positions the store actually assigned) and ::paging_backwards_repeats_no_row_and_drops_none; guard: git diff of crates/happenstance-sqlite/src/event_store.rs shows advance() unchanged"

- id: AC-007
  criterion: "GIVEN an adapter author or an evaluator reading `happenstance-core`'s public surface after this PR, WHEN they diff it, THEN it is byte-identical: no `index_arms()`, no `arm_count()`, no `IndexArm`, no `MAX_QUERY_ITEMS`, no fourth `StoreLimit` variant — and `happenstance-testkit` gains no rule, no registry entry and no mutant row either. Everything this story builds (arm planning, the chunk width, the merge) is `pub(crate)` or private inside `happenstance-sqlite`; the crate's only surface change is documentation. The re-open trigger is recorded, not forgotten: two unlike storage shapes needing the same decomposition is what earns a public `index_arms()`, in its own ADR."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/wide_query.rs — an integration target by construction, so it can only reach public API; plus the crate boundary itself (crates/happenstance-sqlite/src/event_store.rs)"
  verifying_test: "`git diff --stat crates/happenstance-core crates/happenstance-testkit` empty on this branch; `rg -n \"index_arms|arm_count|IndexArm|MAX_QUERY_ITEMS\" crates/` finds nothing new; `cargo doc -p happenstance-sqlite --no-deps` shows no new public item; compilation of crates/happenstance-sqlite/tests/wide_query.rs is the standing enforcement"

- id: AC-008
  criterion: "GIVEN a constrained caller — a bare OS thread with no tokio runtime in scope, and a `tokio::spawn`ed task that holds the stream across an await — WHEN they call `read` with a wide query and then poll it, THEN `read` still executes no SQL and prepares no statement (it cannot: `spawn_blocking` panics with no runtime, which is why the absent-runtime case is `SqliteEventStoreError::NoRuntime` and not a panic), the returned `SqliteReadStream` is still `Send + Unpin`, no `rusqlite::Statement`, `Rows` or `Transaction` has become a field, and the first page arrives without the whole result being buffered — the materialised `position IN (… UNION …)` plan is rejected because it buffers before the first row, not merely because it measured 970× slower."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/shapes.rs (existing type-level target) and crates/happenstance-sqlite/tests/wide_query.rs — both reached through the public SendEventStore::read"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs green unchanged in intent (cargo test -p happenstance-sqlite --test shapes); crates/happenstance-sqlite/tests/wide_query.rs::read_of_a_wide_query_executes_nothing_off_runtime and ::a_wide_read_streams_from_a_tokio_spawn; cargo xtask lints inside cargo xtask affected --base main"
```
