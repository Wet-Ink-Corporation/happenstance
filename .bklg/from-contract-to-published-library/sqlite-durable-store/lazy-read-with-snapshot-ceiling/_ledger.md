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
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:955-977 — `read` still builds a cursor and returns; no SQL, no lock, no runtime lookup. Two tests PASSED: `crates/happenstance-sqlite/tests/read.rs::read_executes_nothing_until_polled`, which appends through the SAME store while the un-polled stream is alive (a guard parked in the cursor would deadlock rather than fail); and `::read_polled_outside_a_runtime_yields_an_error_item`, a plain `#[test]` with no runtime anywhere, which polls the stream with `Waker::noop()` and asserts exactly one `SqliteEventStoreError::NoRuntime` item rather than a panic. ADR-0022 chose option (a), so `NoRuntime` keeps a real meaning: the store prefers the `Handle` captured at construction (crates/happenstance-sqlite/src/event_store.rs:1469-1475, the field at :167-180) and the variant is reachable only when there was no runtime at construction AND none at poll — which is the state that test builds"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::read_executes_nothing_until_polled; ::read_polled_outside_a_runtime_yields_an_error_item (or ::read_polled_outside_a_runtime_runs_inline if ADR-0022 chose the inline path)"

- id: AC-002
  criterion: "Given an application author replaying a log far longer than one page, when they drain a `read` over a store seeded with strictly more than `2 × PAGE_SIZE` events so at least three page statements run, then the stream yields every matching event exactly once in ascending position order — no row repeated at a page boundary and none dropped — and the drain never buffers the whole log."
  satisfied: true
  evidence: "Test `crates/happenstance-sqlite/tests/read.rs::a_multi_page_drain_repeats_and_drops_nothing` PASSED. 1,300 events — strictly more than `2 x PAGE_SIZE`, so at least three page statements run — drained forwards and backwards, with the collected positions compared against the positions the store actually assigned rather than against literals. There is no `cfg(test)` page-size knob anywhere: the paging path under test is the shipped one. `resume_from` keeps its inclusive sense and `advance()` is unchanged (crates/happenstance-sqlite/src/event_store.rs:1407-1430), which is what makes a repeated or dropped row at a 512-boundary visible"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::a_multi_page_drain_repeats_and_drops_nothing"

- id: AC-003
  criterion: "Given the application author's stated fear of a torn log, when a genuinely separate `rusqlite::Connection` appends to the same file while their stream is half-drained, then none of those events appear in that stream, every position it yields is at or below the ceiling captured no later than its first poll, and a fresh `read` issued after the drain does see them."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:1239-1253 — the ceiling is sampled under the same lock acquisition that selects the first page, before those rows are selected, and never re-sampled; `Ceiling` is a three-state enum precisely so that *not sampled yet* and *sampled, store empty* cannot be conflated. Every later statement carries `AND position <= ?` (:1354-1360). Test `crates/happenstance-sqlite/tests/read.rs::a_concurrent_append_mid_drain_is_not_observed` PASSED: 1,300 events seeded, 600 drained to force the sample, then a **second `SqliteEventStore` handle — a second real connection onto the same file** — commits two events, and the rest of the drain is asserted to equal exactly what was there before, while a fresh read afterwards does see them"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::a_concurrent_append_mid_drain_is_not_observed"

- id: AC-004
  criterion: "Given an adapter author with a multi-item `Query` — the shape a real decision model produces — when the read is drained across pages while a matching event lands concurrently, then every item is answered against the same ceiling, so no item observes an event another item could not, and the store implements no second isolation mechanism to achieve it."
  satisfied: true
  evidence: "Test `crates/happenstance-sqlite/tests/read.rs::query_items_share_the_one_ceiling` PASSED: a two-item query over 1,400 events drained across page boundaries while a second connection appends an event matching only the second item; the drain returns exactly 1,400. The mechanism is that every item of a chunk is `UNION`ed into one statement (crates/happenstance-sqlite/src/query_sql.rs:178-191) and the single ceiling predicate is carried by every statement the page issues (crates/happenstance-sqlite/src/event_store.rs:1354-1360, inside the per-chunk loop) — ES-12 discharged by ES-11's ceiling and by no second mechanism, which is what the review of the page SQL confirms. At this story's own commit there was exactly one statement; the slice-mate `wide-query-chunked-not-refused` split a wide query across several, and its AC-005 is what re-checks the ceiling survived the split"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::query_items_share_the_one_ceiling"

- id: AC-005
  criterion: "Given an application author asking for the newest events before position P, when they set `backwards` with `from` and `to`, then they get the newest-first window they asked for: `from` is the higher starting bound and `to` the lower stopping one, both inclusive, the ceiling bounds the starting end rather than replacing `to`, and a closed forward window [from, to] returns exactly its endpoints and everything between."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:1325-1360 — `resume_from` and `to` each pick their comparison from `options.backwards` rather than copying `position <= ?` into both branches, and the ceiling is a separate `position <= ?` term that COMPOSES with `to` rather than replacing it. Test `crates/happenstance-sqlite/tests/read.rs::bounds_compose_in_both_directions` PASSED with four assertions: a closed forward window returning its endpoints and everything between; the same window backwards, newest-first and inclusive at both ends; a `to` above everything the store holds narrowed to the ceiling rather than ignored; and a backwards `from` above the ceiling starting AT the ceiling rather than yielding nothing"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::bounds_compose_in_both_directions"

- id: AC-006
  criterion: "Given an adapter author on their first run — the store is empty, which is the state every adapter is in before it works — when they read it, then they get an empty stream and no error; and given a log with gaps, which `AUTOINCREMENT` guarantees after a delete, when they read `from` a position nothing occupies, then they get the next matching event above it (or below, backwards) rather than an error or an empty result."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:1247-1251 — a `NULL` `max(position)` becomes `Ceiling::Empty`, which means the read is **spent**, not that it failed; no arithmetic is performed on the bound, which is the registered defect (`NullHeadPagingStore`). Two tests PASSED: `::an_empty_store_yields_nothing_and_does_not_error` (forwards, backwards, and `head()` = `None` on a brand-new file) and `::read_from_a_gap_position_yields_the_next_event`, which deletes a real row to make a real gap and reads `from` the now-unoccupied position in both directions, getting the next matching event above (and below) it"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::an_empty_store_yields_nothing_and_does_not_error; ::read_from_a_gap_position_yields_the_next_event"

- id: AC-007
  criterion: "Given an application author rebuilding a decision model who wants only the first n matching events, when they set `limit`, then they get at most n events counted after filtering and across all query items — never n per item and never n per page — and `Some(0)` yields nothing at all rather than everything."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:1293-1299 and :1368-1375 — the per-page budget is `min(remaining, PAGE_SIZE)` and is applied as the statement `LIMIT` **beneath** `ReadOptions::limit`, which `advance()` decrements across pages; `Some(0)` returns an exhausted empty page rather than everything. Test `crates/happenstance-sqlite/tests/read.rs::limit_is_a_whole_read_budget` PASSED over a 1,300-event store: a limit smaller than one page (7, compared against the first seven assigned positions), one spanning several page boundaries (1,100), one larger than the log, `Some(0)` yielding nothing, and a limit over a two-item query returning 5 across the items rather than 5 per item"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::limit_is_a_whole_read_budget"

- id: AC-008
  criterion: "Given an application author deriving an `AppendCondition`'s boundary from what the store says it has, when they call `head` — on an empty store, on a populated one, and from a second handle onto the same file after the first handle appended — then they get `None`, the highest visible position, and the position the other handle just wrote, respectively; the value is queried fresh every call and is never a field the store caches."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/event_store.rs:1055-1078 — `head` is `SELECT max(position) FROM event` asked of the database on every call, with `NULL` as the `None` arm; there is no memoised field anywhere on `SqliteEventStore` (its only fields are the `Arc<Mutex<Connection>>`, the `StoreId` and the runtime `Handle`). Test `crates/happenstance-sqlite/tests/read.rs::head_is_a_fresh_query_across_two_handles` PASSED: `None` on a new file, the assigned maximum after an append, and — the load-bearing third — the position a **second** `SqliteEventStore::open` on the same path just wrote, reported by the FIRST handle"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::head_is_a_fresh_query_across_two_handles"

- id: AC-009
  criterion: "Given an adapter author who wrote an event and now reads it back, when the page SQL filters and the codec decodes, then they get their event: types within an item OR'd, tags within an item AND'd with superset matching, items OR'd, duplicate items not duplicating events, `Query::All` matching untagged events — and the payload, metadata (absent distinguished from empty), canonically-ordered tags, `EventId` reconstructed from the stored origin pair, and `recorded_at` as stored all coming back byte-for-byte."
  satisfied: true
  evidence: "crates/happenstance-sqlite/src/query_sql.rs:159-208 (types OR within an item, tags AND with superset matching, items `UNION`ed so duplicates do not duplicate, `Query::all` short-circuiting to `event` so untagged events match) and crates/happenstance-sqlite/src/row.rs:95-132 (`to_event`, the decode half of the one shared codec, which is where the file ends). Two tests PASSED: `::query_semantics_are_served_by_the_page_sql` — all five semantics plus the type-plus-tag shape the covering column exists for — and `::a_decoded_row_matches_what_was_written_byte_for_byte`, which round-trips a 256-byte payload, canonically-ordered tags, metadata absent vs empty (kept apart), the `EventId` reconstructed from the **stored origin pair** rather than from the local incarnation, and `recorded_at` compared against the value in the file. `::contains_event_id_answers_from_the_stored_origin_pair` covers the probe that seeks the same pair"
  mount_point: "crates/happenstance-sqlite/tests/read.rs"
  verifying_test: "crates/happenstance-sqlite/tests/read.rs::query_semantics_are_served_by_the_page_sql; ::a_decoded_row_matches_what_was_written_byte_for_byte"

- id: AC-010
  criterion: "Given an adapter author reading this crate as the worked example of the port — the 'learn when you are finished' journey — when they open the completed read path, then the type-level obligations still hold after the cursor gained a ceiling (`SqliteReadStream: Send + Unpin`, `SqliteEventStore: Send + Sync`, no `Statement`/`Rows`/`Transaction` in any field, both port flavours type-checking with the stream at the top level) and every public item this story completes carries real composed rustdoc — a summary, the laziness and ceiling contract stated where a caller meets it, and an `# Errors` section naming the conditions rather than the error types."
  satisfied: true
  evidence: "`cargo test -p happenstance-sqlite --test shapes` PASSED with the file unchanged — all ten assertions still hold after `ReadCursor` gained a `Ceiling` and a `tokio::runtime::Handle`, both of which are `Send + Unpin`, and after `SqliteEventStore` gained the same handle. No `Statement`, `Rows` or `Transaction` reached a field. Every public item this story completed carries composed rustdoc with the laziness and ceiling contract stated where a caller meets it and an `# Errors` section naming conditions: `fetch_page` (crates/happenstance-sqlite/src/event_store.rs:1276-1405, with the ceiling contract stated on `sample_ceiling` at :1239-1259), `head` (:1055-1078), `contains_event_id` (:1080-1089). `cargo xtask affected --base main` PASSED whole — fmt, `clippy -D warnings` over all targets and all features, the five file-reading lints, spec-trace and the tests. **It runs no rustdoc build at all** (xtask/src/affected.rs:38-44), so this row previously credited it with a check it never performed; the rustdoc obligation is carried separately and was RED until the intra-doc link at event_store.rs:39 was repaired. Observed 2026-08-17: `RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance-sqlite --no-deps` clean, and the `documentation` step of `cargo xtask ci --fast` green inside a run that reports `all required checks passed` end to end"
  mount_point: "crates/happenstance-sqlite/tests/shapes.rs"
  verifying_test: "crates/happenstance-sqlite/tests/shapes.rs::read_stream_is_send_and_unpin, ::store_is_send_and_sync, ::send_impl_satisfies_the_bare_bound, ::send_flavour_stream_is_send_in_generic_code, ::read_returns_the_named_stream_type; plus `cargo clippy -p happenstance-sqlite -- -D warnings` inside `cargo xtask affected --base main`, and — separately, because `affected` runs no rustdoc build — `RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance-sqlite --no-deps` and the `documentation` step of `cargo xtask ci --fast`"
```
