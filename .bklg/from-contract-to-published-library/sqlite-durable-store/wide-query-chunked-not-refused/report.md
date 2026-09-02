---
item: "HS-S0039"
stage: report
created: "2026-08-17"
updated: "2026-08-17"
---

# Report — A wide query is chunked, not refused

## Findings Ledger

**Eight of eight ACs satisfied.** A `Query` wider than SQLite will compile into
one compound `SELECT` is decomposed into `ceil(arms / 400)` statements and merged
in Rust. There is no query-width refusal path anywhere in the crate to reach.

**Mount point:** `crates/happenstance-sqlite/tests/wide_query.rs` — a new
integration target, auto-discovered and run by `cargo xtask affected --base
main`. It is an *integration* target deliberately: it can only reach public API,
so the boundary observation had to be designed rather than smuggled.

| AC | Result | What proves it |
| --- | --- | --- |
| **AC-001** — a wide query is served | **Met** | `wide_query.rs::a_query_at_the_guaranteed_minimum_item_count_is_served`, building `MIN_SUPPORTED_QUERY_ITEMS` items (imported, never the literal 128) with the **only** matching item last; and `::a_query_far_above_the_minimum_is_served` at 900. The negative guard returns one hit and it is the rustdoc sentence saying no such constant exists |
| **AC-002** — the boundary is crossed, and observed | **Met** | `wide_query.rs::a_wide_query_actually_crosses_the_chunk_boundary`, asserting `planned_statement_count > 1`, equal to `ceil(arms/width)`, and `== 1` for a narrow query so the number means something. Every width in the file is derived from the crate constant, so changing it cannot silently stop the file crossing the boundary |
| **AC-003** — the union, each event once, in order | **Met** | Four tests, of which two are the sharp ones: `::the_union_is_every_chunk_not_the_first` puts every matching item in the **last** chunk, and `::item_order_does_not_change_the_result_set` reverses all 900 items so each matching one lands in a different chunk |
| **AC-004** — `limit` across the merge | **Met** | `::limit_applies_across_chunks_not_per_chunk` (10 over a 3-statement plan, with the failure message naming the 30 a per-chunk limit would have returned), plus the forwards and backwards filtering cases. ES-14's named rejected implementation is the per-statement `LIMIT`, and the backwards case is where it shows |
| **AC-005** — one ceiling over every chunk and hop | **Met** | `::a_concurrent_append_between_chunks_is_not_seen` and `::a_concurrent_append_between_pages_is_not_seen`: a wide query whose item list **names** the event a second real connection then appends, drained partly before the commit and finished after |
| **AC-006** — paging repeats nothing, drops nothing | **Met** | `::paging_a_wide_query_repeats_no_row_and_drops_none` — 600 matches (above `PAGE_SIZE`) across a 3-statement plan, compared against the positions the appends actually assigned — and its backwards twin. Per-chunk state stays inside one hop; `advance()` folds only the merged page |
| **AC-007** — nothing outside this crate moved | **Met, with one stated deviation** | `git diff --stat HEAD -- crates/happenstance-core crates/happenstance-testkit spec .kb` is **empty**. `happenstance-sqlite` itself gains two documented public items, which AC-002 requires — see below |
| **AC-008** — the constrained callers | **Met** | `::read_of_a_wide_query_executes_nothing_off_runtime` (no runtime, no panic, no prepared statement — `spawn_blocking` would have panicked) and `::a_wide_read_streams_from_a_tokio_spawn` (the stream held across an await inside a real multi-threaded spawn). `tests/shapes.rs` green unchanged |

## The RED run is this story's whole justification

Before the chunking landed, fourteen of sixteen tests failed with SQLite's own
words — `too many terms in compound SELECT`. That is the pushdown limit the
runbook names, arriving as a real driver error against a real database. **The
conformance suite cannot reach it**: VT-23's floor is 128 items and the
compound-`SELECT` ceiling is 500, so shipping the chunking without this target
would have left the merge as dead code behind a green suite — the exact failure
mode this initiative exists to retire.

## The one deviation, and why it is the right way round

AC-007 asks for no new public item. AC-002 asks for the chunk boundary to be
observable from an **integration** target — which can only reach public API —
through a seam free of `#[doc(hidden)]` and `cfg(test)`. Both cannot be met
literally, and AC-007's own verification note settles it: *"the boundary
observation of AC-002 must therefore be designed, not smuggled."*

So `happenstance-sqlite` gains exactly two documented public items,
`MAX_QUERY_ARMS_PER_STATEMENT` and `planned_statement_count`. The second is not a
test hook — it is the function the read path itself plans with, so it cannot
report a boundary the code does not take. The invariant AC-007 exists to protect
holds exactly: the frozen contract crate and the shared testkit are byte-identical.

## Deferred, and to whom

Nothing. `index_arms()` stays **rejected** per ADR-0022 §10, with its re-open
trigger recorded in `crates/happenstance-sqlite/src/query_sql.rs`'s module doc
rather than left to memory: two unlike storage shapes needing the same
decomposition is the evidence that would earn a public name, and
`postgres-and-neon-stores` is the project that would supply it — as its own ADR
then, not as a side effect of this one.
