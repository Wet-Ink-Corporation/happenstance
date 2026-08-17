---
item: "HS-S0039"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Implementation Report — A wide query is chunked, not refused

> **STATUS: eight of eight ACs satisfied.** A `Query` wider than SQLite will
> compile into one statement is now decomposed into
> `ceil(arms / 400)` statements and k-way merged, rather than refused.
>
> **The RED run is the whole argument for this story existing.** Before the
> chunking, fourteen of sixteen tests failed with SQLite's own words:
>
> ```text
> Sqlite(SqliteFailure(Error { code: Unknown, extended_code: 1 },
> Some("too many terms in compound SELECT")))
> ```
>
> That is the pushdown limit the runbook's work item names, arriving as a real
> driver error against a real database — and **nothing in the conformance suite
> can reach it**, because VT-23's floor is 128 items and the compound-`SELECT`
> ceiling is 500.

## TDD Evidence

**RED.** `crates/happenstance-sqlite/tests/wide_query.rs` written in full —
sixteen tests — with `planned_statement_count` added as the seam AC-002
requires.

```text
running 16 tests
test result: FAILED. 2 passed; 14 failed; 0 ignored
```

The two that passed are the ones that do not cross the boundary: the 128-item
minimum, and the off-runtime construction. Every other one failed with
`too many terms in compound SELECT`.

**GREEN.** `query_sql::statement_count` and `query_sql::chunks`, and a
`fetch_page` that loops the plan and merges. **16 passed, 0 failed.**

| AC | Test that encodes it | Red → Green |
| --- | --- | --- |
| AC-001 | `::a_query_at_the_guaranteed_minimum_item_count_is_served`, `::a_query_far_above_the_minimum_is_served` | green at 128 already; `too many terms` at 900 → served |
| AC-002 | `::a_wide_query_actually_crosses_the_chunk_boundary` | `todo!()` on the seam → `> 1` statements, and `ceil(arms/width)` |
| AC-003 | `::overlapping_arms_yield_each_event_once`, `::item_order_does_not_change_the_result_set`, `::the_union_is_every_chunk_not_the_first`, `::items_with_similar_types_but_different_tags_are_not_collapsed` | `too many terms` → one sort, one dedup, the whole union |
| AC-004 | `::limit_applies_across_chunks_not_per_chunk`, `::limit_applies_after_filtering_forwards`, `::limit_applies_after_filtering_backwards` | same → the budget applied to the merged output |
| AC-005 | `::a_concurrent_append_between_chunks_is_not_seen`, `::a_concurrent_append_between_pages_is_not_seen` | same → the ceiling on every chunk of every hop |
| AC-006 | `::paging_a_wide_query_repeats_no_row_and_drops_none`, `::paging_backwards_repeats_no_row_and_drops_none` | same → 600 matches over a page boundary and a 3-statement plan |
| AC-007 | `git diff --stat` over the contract crate and the testkit | empty |
| AC-008 | `::read_of_a_wide_query_executes_nothing_off_runtime`, `::a_wide_read_streams_from_a_tokio_spawn`, `tests/shapes.rs` | green |

## Commits

`feat(sqlite-durable-store): A wide query is chunked, not refused` — see the
`Story: sqlite-durable-store/wide-query-chunked-not-refused` trailer.

## Changes

| File | Shape of the change |
| --- | --- |
| `crates/happenstance-sqlite/src/query_sql.rs` | `statement_count` and `chunks` — the decomposition, crate-private |
| `crates/happenstance-sqlite/src/event_store.rs` | `MAX_QUERY_ARMS_PER_STATEMENT` and `planned_statement_count` (the two new public items, documented); `fetch_page` loops the plan, and merges with one direction-aware sort, one dedup on position, an `exhausted` computed before truncation, and the budget applied to the merged output; the module-doc status banner corrected now that every event-store path is real |
| `crates/happenstance-sqlite/tests/wide_query.rs` | **New target.** Sixteen tests, every width derived from the crate's own constant rather than a literal |

## Gates

| Gate | Result |
| --- | --- |
| `cargo test -p happenstance-sqlite --test wide_query` | **16 passed**, 0 failed |
| `cargo test -p happenstance-sqlite` (all five targets) | **65 passed** across `migration`, `append`, `read`, `wide_query`, `shapes` |
| `cargo xtask affected --base main` | **PASSED** |
| `cargo fmt --all` | clean; run last |

Two `clippy::pedantic` findings (`unnecessary_sort_by`, `assertions_on_constants`)
were fixed at the source rather than suppressed.

## Notes

**The chunk width is 400 because SQLite's ceiling is 500.**
`SQLITE_MAX_COMPOUND_SELECT` defaults to 500 terms, and a `UNION` of *n* arms is
one compound `SELECT` of *n* terms. The runbook proposed `ceil(arms/400)` and
that is what landed, with the headroom stated in the constant's own rustdoc.

**The merge is sound rather than approximate, and the reason is that every chunk
statement is identical in shape** — same ceiling, same `resume_from`, same `to`,
same direction, same `LIMIT budget`. The merged top-`budget` is therefore a
subset of the union of the per-chunk tops, so bounding each chunk loses nothing.
`exhausted` is computed **before** truncation, and the argument is written into
the code: a chunk's own rows are already distinct, so a chunk that filled its
budget puts that many distinct positions into the merge — meaning a short merge
implies every chunk was short.

**The one deviation from AC-007, stated rather than hidden.** AC-007 asks for no
new public item; AC-002 asks for the chunk boundary to be observable from an
**integration** target, which can only reach public API, through a seam free of
`#[doc(hidden)]` and `cfg(test)`. Those two cannot both be met literally, and
AC-007's own verification note says which way it goes: *"the boundary observation
of AC-002 must therefore be designed, not smuggled."* So `happenstance-sqlite`
gains exactly two documented public items — `MAX_QUERY_ARMS_PER_STATEMENT` and
`planned_statement_count` — and the invariant AC-007 exists to protect holds
exactly: `git diff --stat HEAD -- crates/happenstance-core
crates/happenstance-testkit spec .kb` is **empty**. `planned_statement_count` is
not a test hook: it is the function the read path itself plans with, so it cannot
report a boundary the code does not take.

**`index_arms()` stays rejected, and the re-open trigger is recorded rather than
remembered.** ADR-0022 §10's reasoning is written into
`crates/happenstance-sqlite/src/query_sql.rs`'s module doc: the API does not
exist in `happenstance-core`, minting it would add public surface to a frozen
contract crate for the benefit of one implementor, and the evidence that would
earn it is `postgres-and-neon-stores` independently needing the same
decomposition — as its own ADR then, not as a side effect of this one.

**Corrected after slice review: "chunk and merge, never refuse" held on the read
path only.** `query_sql`'s module doc opened by asserting the translation is
asked "in two places" and that "both callers use it" — and the code had two
spellings. `read` went through `chunks`; `evaluate`, the append-condition guard
at `crates/happenstance-sqlite/src/event_store.rs:627`, went through an
unchunked `match_sql`. A guard carrying more than `MAX_QUERY_ARMS_PER_STATEMENT`
arms therefore failed with SQLite's own *"too many terms in compound SELECT"*
wrapped as `AppendError::Store`: the refusal at the pushdown limit that AC-008
names as its wrong implementation, arriving on the **write** path, where
`crates/happenstance-core/src/limits.rs:46-52` gives it no variant to be reported
through. Nothing in the suite could see it — `MIN_SUPPORTED_QUERY_ITEMS` is 128
and the chunk width is 400 — so it was latent rather than red.

`evaluate` now plans with `chunks` and folds the per-chunk `max(position)`
results with `Option<i64>`'s own ordering (`None` sorts below every `Some`, so an
empty chunk contributes nothing and there is no special case for "no match yet").
The fold is **exact**, not an approximation of the unchunked query: a guard is an
inequality on the *highest* match, and `max(max(a), max(b))` is `max(a ∪ b)`.

`match_sql` is **deleted** rather than left beside `chunks`. A single-chunk plan
is the narrow case of the wide one, so the removed spelling could say nothing the
survivor cannot — and two spellings of one question is precisely how a module doc
came to claim a property half the crate did not have.

Two tests, both red before the change with the literal SQLite message and green
after: `tests/wide_query.rs::a_wide_append_condition_guard_is_not_refused` and
`::a_wide_guard_answers_from_every_chunk_not_the_first`. The second is the
sharper one: it puts matching items either side of the chunk partition and sets
the guard's boundary at the **lower** of them, so an implementation that answered
from the first chunk alone would find nothing above the boundary and *accept* the
append — a silent wrong answer rather than a loud one.
