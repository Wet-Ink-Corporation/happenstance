---
item: "HS-S0038"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — A lazy, paged, snapshot-bounded read

## Findings Ledger

**Ten of ten ACs satisfied.** This is the workspace's first multi-statement
`read`, which makes it the first implementation that could fail ES-11 and ES-12 —
and the first that can produce the torn log the application author's stated fear
names. It does not.

**Mount point:** `crates/happenstance-sqlite/tests/read.rs` — a new integration
target, auto-discovered and therefore run by `cargo xtask affected --base main`
and `cargo xtask ci --fast`. It is **not** retired when `tests/conformance.rs`
arrives: it asks three questions the suite cannot, because the suite has no way
to know how many statements a `read` issued.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — `read` executes nothing; a missing runtime is an item | **Met** | `read.rs::read_executes_nothing_until_polled` appends through the **same store** while the un-polled stream is alive — a guard parked in the cursor would deadlock rather than fail. `::read_polled_outside_a_runtime_yields_an_error_item` is a plain `#[test]`, no runtime anywhere, polling with `Waker::noop()` and asserting one `NoRuntime` item |
| **AC-002** — a multi-page drain repeats and drops nothing | **Met** | `read.rs::a_multi_page_drain_repeats_and_drops_nothing`: 1,300 events — past `2 × PAGE_SIZE`, so at least three page statements — drained both directions and compared against the positions the store actually assigned. **No `cfg(test)` page-size knob**, so the tested paging path is the shipped one |
| **AC-003** — the ceiling is a ceiling | **Met** | `read.rs::a_concurrent_append_mid_drain_is_not_observed`: 600 events drained to force the sample, a **second real connection** commits two, and the rest of the drain equals exactly what was there before — while a fresh read afterwards does see them |
| **AC-004** — every item shares that one ceiling | **Met** | `read.rs::query_items_share_the_one_ceiling`: a two-item query over 1,400 events with a concurrent append matching only the second item, returning exactly 1,400. The mechanism is one `UNION`ed statement carrying one ceiling predicate — ES-12 discharged by ES-11's ceiling and by **no second mechanism** |
| **AC-005** — the bounds compose in both directions | **Met** | `read.rs::bounds_compose_in_both_directions`, four assertions including the two that reject ES-16's named wrong shape: a backwards window inclusive at both ends and newest-first, and a backwards `from` above the ceiling starting *at* the ceiling rather than yielding nothing |
| **AC-006** — the empty store and the gap | **Met** | `::an_empty_store_yields_nothing_and_does_not_error` and `::read_from_a_gap_position_yields_the_next_event`, the latter deleting a real row to make a real gap. `Ceiling::Empty` means *spent*, never *failed*; no arithmetic is done on a `None` head, which is the registered `NullHeadPagingStore` defect |
| **AC-007** — `limit` is a whole-read budget | **Met** | `read.rs::limit_is_a_whole_read_budget` over 1,300 events: smaller than a page, spanning several page boundaries, larger than the log, `Some(0)` yielding nothing, and 5 across a two-item query rather than 5 per item |
| **AC-008** — `head` is a fresh query | **Met** | `read.rs::head_is_a_fresh_query_across_two_handles`. The load-bearing third assertion is the first handle reporting the position the **second** handle just wrote — which a memoised field passes every single-handle test and fails only here |
| **AC-009** — the store's own event comes back | **Met** | `::query_semantics_are_served_by_the_page_sql` (five semantics plus the type-plus-tag shape the covering column exists for) and `::a_decoded_row_matches_what_was_written_byte_for_byte` — 256-byte payload, canonical tags, metadata absent vs empty kept apart, `EventId` from the **stored** origin pair, `recorded_at` as stored |
| **AC-010** — the type-level obligations and the rustdoc | **Met** | `cargo test --test shapes` green unchanged after `ReadCursor` gained a `Ceiling` and a `Handle`; no `Statement`, `Rows` or `Transaction` in any field; `cargo doc` and `clippy -D warnings` green inside the gate |

## What the runtime seam cost, and what it bought

ADR-0022 §9 chose to capture a `tokio::runtime::Handle` at construction, prefer
it, and keep `Handle::try_current()` as the fallback. That decision is spent
here, not re-opened, and its consequence is two stories away: the concurrency
family drives contenders on **bare OS threads** with no tokio context at poll
time, so a `try_current`-only store would answer `NoRuntime` for every read in
that family and go red for a reason that is not about this adapter.

Because option (a) won, `SqliteEventStoreError::NoRuntime` keeps a real meaning —
a store both *constructed* and *driven* with no runtime anywhere — and
`read_polled_outside_a_runtime_yields_an_error_item` is the test that builds
exactly that state. Under option (b) the variant and its module-doc paragraph
would both have had to be deleted in the same change.

## A gap between two specs, closed here and named

`contains_event_id` is listed by **this** story's spec as *"landed by
`schema-migration-and-identity` and consumed here unchanged"*, and by **that**
story's spec as *"the slice-mates'"*. It was neither: it fell between them. It is
a read-side probe over the origin pair, so it landed here beside `head`, with its
own test — and had it not, `contains_event_id_reports_membership` would have been
unrunnable in `sqlite-fixture-and-whole-suite` two positions later, which is the
sort of gap that surfaces as a red conformance run naming no author.

## Deferred, and to whom

Nothing in this story's scope. ES-11 and ES-12 keep their `[PROVISIONAL]`
markers, deliberately: their falsifier is the **transport** axis, whose far end
is `postgres-and-neon-stores`', not this adapter's. `git diff --stat HEAD --
crates/happenstance-core crates/happenstance-testkit spec .kb` is empty.

## What is still `todo!()`, on purpose

The projection store's `migrate`, `checkpoint`, `commit` and `rollback` bodies.
Every `todo!()` on the **event store** path is gone; `#![allow(clippy::todo)]` is
still at `crates/happenstance-sqlite/src/lib.rs:83` and dies with the last of
those four, in `instrument-markers-removed-and-gate-green`.
