---
item: "HS-S0038"
stage: implement
created: "2026-08-12"
updated: "2026-08-12"
---

# Acceptance ledger — The real read stream, lazy, paged, and snapshot-bounded

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
  criterion: "Given an application author holding a store handle on a bare OS thread — the shape ADR-0001 exists to keep possible — when they call `read` and have not yet polled the returned stream, then no SQL has run, no connection lock has been taken and no runtime has been looked up; and when they poll that stream with no tokio runtime present, the absence arrives as exactly one `Err` item on the stream and nothing panics."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::read_executes_nothing_until_polled; ::read_polled_outside_a_runtime_yields_an_error_item (or ::read_polled_outside_a_runtime_runs_inline if ADR-0022 chose the inline path)"

- id: AC-002
  criterion: "Given an application author replaying a log far longer than one page, when they drain a `read` over a store seeded with strictly more than `2 × PAGE_SIZE` events so at least three page statements run, then the stream yields every matching event exactly once in ascending position order — no row repeated at a page boundary and none dropped — and the drain never buffers the whole log."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::a_multi_page_drain_repeats_and_drops_nothing"

- id: AC-003
  criterion: "Given the application author's stated fear of a torn log, when a genuinely separate `rusqlite::Connection` appends to the same file while their stream is half-drained, then none of those events appear in that stream, every position it yields is at or below the ceiling captured no later than its first poll, and a fresh `read` issued after the drain does see them."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::a_concurrent_append_mid_drain_is_not_observed"

- id: AC-004
  criterion: "Given an adapter author with a multi-item `Query` — the shape a real decision model produces — when the read is drained across pages while a matching event lands concurrently, then every item is answered against the same ceiling, so no item observes an event another item could not, and the store implements no second isolation mechanism to achieve it."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::query_items_share_the_one_ceiling"

- id: AC-005
  criterion: "Given an application author asking for the newest events before position P, when they set `backwards` with `from` and `to`, then they get the newest-first window they asked for: `from` is the higher starting bound and `to` the lower stopping one, both inclusive, the ceiling bounds the starting end rather than replacing `to`, and a closed forward window [from, to] returns exactly its endpoints and everything between."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::bounds_compose_in_both_directions"

- id: AC-006
  criterion: "Given an adapter author on their first run — the store is empty, which is the state every adapter is in before it works — when they read it, then they get an empty stream and no error; and given a log with gaps, which `AUTOINCREMENT` guarantees after a delete, when they read `from` a position nothing occupies, then they get the next matching event above it (or below, backwards) rather than an error or an empty result."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::an_empty_store_yields_nothing_and_does_not_error; ::read_from_a_gap_position_yields_the_next_event"

- id: AC-007
  criterion: "Given an application author rebuilding a decision model who wants only the first n matching events, when they set `limit`, then they get at most n events counted after filtering and across all query items — never n per item and never n per page — and `Some(0)` yields nothing at all rather than everything."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::limit_is_a_whole_read_budget"

- id: AC-008
  criterion: "Given an application author deriving an `AppendCondition`'s boundary from what the store says it has, when they call `head` — on an empty store, on a populated one, and from a second handle onto the same file after the first handle appended — then they get `None`, the highest visible position, and the position the other handle just wrote, respectively; the value is queried fresh every call and is never a field the store caches."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::head_is_a_fresh_query_across_two_handles"

- id: AC-009
  criterion: "Given an adapter author who wrote an event and now reads it back, when the page SQL filters and the codec decodes, then they get their event: types within an item OR'd, tags within an item AND'd with superset matching, items OR'd, duplicate items not duplicating events, `Query::All` matching untagged events — and the payload, metadata (absent distinguished from empty), canonically-ordered tags, `EventId` reconstructed from the stored origin pair, and `recorded_at` as stored all coming back byte-for-byte."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::query_semantics_are_served_by_the_page_sql; ::a_decoded_row_matches_what_was_written_byte_for_byte"

- id: AC-010
  criterion: "Given an adapter author reading this crate as the worked example of the port — the 'learn when you are finished' journey — when they open the completed read path, then the type-level obligations still hold after the cursor gained a ceiling (`SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, no `Statement`/`Rows`/`Transaction` in any field, both port flavours type-checking with the stream at the top level) and every public item this story completes carries real composed rustdoc — a summary, the laziness and ceiling contract stated where a caller meets it, and an `# Errors` section naming the conditions rather than the error types."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance-sqlite/tests/shapes.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs::read_stream_is_send_and_unpin, ::store_is_send_and_sync, ::send_impl_satisfies_the_bare_bound, ::send_flavour_stream_is_send_in_generic_code, ::read_returns_the_named_stream_type; plus `cargo doc -p happenstance-sqlite` and `cargo clippy -p happenstance-sqlite -- -D warnings` inside `cargo xtask affected --base main`"
```
