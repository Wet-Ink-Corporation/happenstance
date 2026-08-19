---
item: "HS-S0038"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — A lazy, paged, snapshot-bounded read

> **STATUS: ten of ten ACs satisfied.** The read path is real, and with it the
> workspace has its **first multi-statement `read`** — which means ES-11's
> ceiling clause has, for the first time, an implementation that could fail it.
>
> The one field the skeleton was missing is now there: `ReadCursor::ceiling`,
> sampled under the same lock acquisition that selects the first page and never
> re-sampled. It is spelled as a three-state enum rather than an
> `Option<Option<_>>` because *not sampled yet* and *sampled, and the store was
> empty* are different facts, and conflating them is exactly how the registered
> `NullHeadPagingStore` defect gets written.

## TDD Evidence

**RED.** `crates/happenstance-sqlite/tests/read.rs` written in full — thirteen
tests — against `ReadCursor::fetch_page`, `head` and `contains_event_id`, all
still `todo!()`.

```text
running 13 tests
test result: FAILED. 1 passed; 12 failed; 0 ignored

thread 'tokio-rt-worker' panicked at crates\happenstance-sqlite\src\event_store.rs:1039:9:
not yet implemented: SQLite event store: page query for All
ReadOptions { from: None, to: None, backwards: false, limit: None }
```

The one that passed at red is `read_polled_outside_a_runtime_yields_an_error_item`
— and it passed for the *wrong* reason, because before the captured handle
landed, `Handle::try_current()` failed for every store. Its green run is what
makes it meaningful: the store now prefers a handle captured at construction, so
the test's store is one built with no runtime at all, and `NoRuntime` is
reachable only in that state.

**GREEN.** The ceiling, the page SQL, the decode half of the row codec, `head`,
`contains_event_id`, and the runtime seam. **13 passed, 0 failed.**

| AC | Test that encodes it | Red → Green |
| --- | --- | --- |
| AC-001 | `::read_executes_nothing_until_polled`, `::read_polled_outside_a_runtime_yields_an_error_item` | `read` inert (proved by appending through the same store while the stream is alive); a missing runtime as one `Err` item |
| AC-002 | `::a_multi_page_drain_repeats_and_drops_nothing` | `todo!("page query")` → 1,300 events, both directions, compared against assigned positions |
| AC-003 | `::a_concurrent_append_mid_drain_is_not_observed` | same panic → a second connection's commit is invisible to a half-drained read |
| AC-004 | `::query_items_share_the_one_ceiling` | same panic → one predicate, two items, 1,400 exactly |
| AC-005 | `::bounds_compose_in_both_directions` | same panic → four bound assertions, including ES-16's named wrong shape |
| AC-006 | `::an_empty_store_yields_nothing_and_does_not_error`, `::read_from_a_gap_position_yields_the_next_event` | same panic → `Ceiling::Empty` is spent, not failed; a real deleted-row gap |
| AC-007 | `::limit_is_a_whole_read_budget` | same panic → after filtering, across items and pages, `Some(0)` yielding nothing |
| AC-008 | `::head_is_a_fresh_query_across_two_handles` | `todo!("head")` → the first handle reports what the second just wrote |
| AC-009 | `::query_semantics_are_served_by_the_page_sql`, `::a_decoded_row_matches_what_was_written_byte_for_byte`, `::contains_event_id_answers_from_the_stored_origin_pair` | same panics → five semantics, byte-for-byte round trip, origin-pair probe |
| AC-010 | `tests/shapes.rs` unchanged, `cargo xtask affected --base main` | 10 passed, gate green |

## Commits

`feat(sqlite-durable-store): A lazy, paged, snapshot-bounded read` — see the
`Story: sqlite-durable-store/lazy-read-with-snapshot-ceiling` trailer.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/src/event_store.rs` | `Ceiling`, the three-state ADR-0011 sample; `ReadCursor` gains `ceiling` and `runtime`; a real `fetch_page` (ceiling, bounds, budget, decode); a real `head` and `contains_event_id`; the store captures a `tokio::runtime::Handle` at construction and `poll_next` prefers it; three new error variants (`UnstampedEvent`, `CorruptTags`, and the reuse of `MalformedIdentity` for a stored origin) |
| `crates/happenstance-sqlite/src/row.rs` | The decode half: `COLUMNS`, `decode_tags`, `to_event` |
| `crates/happenstance-sqlite/tests/read.rs` | **New target.** Thirteen tests, seeded through the adapter's own `append` — the same codec the decode reads back — with no page-size knob anywhere |

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test read` | **13 passed**, 0 failed |
| `cargo test -p happenstance-sqlite --test append` | **17 passed** — the parallel slice-mate still green |
| `cargo test -p happenstance-sqlite --test migration` | **9 passed** |
| `cargo test -p happenstance-sqlite --test shapes` | **10 passed**, unchanged in intent |
| `cargo xtask affected --base main` | **PASSED** |
| `cargo fmt --all` | clean; run last |

## Notes

**The runtime seam is ADR-0022 §9's answer, consumed rather than re-decided.**
Option (a): capture a `Handle` at construction, prefer it, keep
`Handle::try_current()` as the fallback. The reason it matters is two stories
away — the concurrency family drives its contenders on **bare OS threads** under
the testkit's own park-loop `block_on`, with no tokio context at poll time, so a
`try_current`-only store would answer `NoRuntime` for every read in that family
and produce a red result that is not about this adapter's logic. Because (a) won,
`SqliteEventStoreError::NoRuntime` **keeps a real meaning** and the module-doc
paragraph explaining it stays true as written; under (b) both would have had to
be deleted in the same change.

**`contains_event_id` landed here, and the gap is worth naming.** This story's
spec lists it under *"landed by `schema-migration-and-identity` and consumed here
unchanged"*, while `schema-migration-and-identity`'s spec lists it under *"the
slice-mates'"*. It is neither: it fell between the two specs. It is a read-side
probe over the origin pair, so it belongs with `head`, and leaving it `todo!()`
would have made `contains_event_id_reports_membership` unrunnable in the fixture
story two positions later. Implemented here, with its own test.

**No `cfg(test)` page-size knob.** `PAGE_SIZE` stays 512 and stays private; the
multi-page criteria seed 1,300 events so that at least three page statements run.
A knob would have made the tested paging path a different path from the shipped
one, which is the failure this whole project exists to retire.

**Nothing out of boundary.** `git diff --stat HEAD -- crates/happenstance-core
crates/happenstance-testkit spec .kb` is empty: ES-11 and ES-12 keep their
`[PROVISIONAL]` markers (their falsifier is the transport axis, which
`postgres-and-neon-stores` owns), no ADR was authored, no `Cargo.toml` changed
and no dependency was added. `#![allow(clippy::todo)]` is still at `lib.rs:83`,
and the only `todo!()`s removed are `fetch_page`'s, `head`'s and
`contains_event_id`'s — the projection store's four are all still marked.
