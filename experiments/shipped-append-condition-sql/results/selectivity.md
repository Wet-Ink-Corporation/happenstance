# The two query-planning costs that need no rows at all

Rows from [`raw/selectivity.txt`](raw/selectivity.txt) — `--release`, three
tests, no seeded log. The shape measured is **the specification's own floor**,
not a corner: VT-23 requires every store to evaluate at least 128 query items,
nothing anywhere bounds tags per query item, and `MAX_TAGS_PER_EVENT` is 128. So
128 items × 128 tags is a query a conformant caller may build, and it presents
16,384 tags to the planning path.

That path runs once per read page (`event_store.rs:1301`) and once per append
guard (`event_store.rs:645`) — and the append one runs **inside
`BEGIN IMMEDIATE`**, with the write lock held.

## 1. The deduplication quadratic

25 rounds, median µs. `distinct_tags_in_query` is the size of the accumulated
`wanted` vector.

| shape | distinct tags | `planned_statement_count` (public, shipped) | `read_for` accumulation (`Vec::contains`) | the same via `BTreeSet` | ratio |
| --- | ---: | ---: | ---: | ---: | ---: |
| `distinct-tags` | 16,384 | 4,595 | **257,690** | 6,424 | **40.1x** |
| `shared-tags` (control) | 128 | 5,033 | 6,150 | 4,746 | 1.30x |

**257.7 ms per call.** The `BTreeSet` counterfactual is asserted to return
byte-identical output before either is timed (`selectivity_cost.rs:125-130`), so
this is a replacement and not a different function wearing the same name.

The `shared-tags` row is the control and it is what makes the figure readable.
Its 128 items name the *same* 128 tags, so `wanted` stops growing after the first
item and the cross-item quadratic disappears — 257,690 µs collapses to 6,150 µs —
while `planned_statement_count` barely moves (4,595 → 5,033). Two quadratics
live on this path and the control separates them:

* the **cross-item** one, in `Selectivity::read_for`'s `wanted.contains` — 250 ms
  of the 258 ms, and the only one the control removes;
* the **per-item** one, in `query_sql::distinct_tags` — reached by the public
  `SqliteEventStore::planned_statement_count`, and about 4.6–5.0 ms either way.

Only the second is measured through a shipped public entry point.
`planned_statement_count` passes `Selectivity::default()`
(`event_store.rs:288-291`), so it never reaches `read_for` at all; the 257.7 ms
is a line-for-line transcription of a crate-private function, timed here because
there is no other seam that can reach it from outside the crate.

## 2. Chunked by item count, never by parameter count

`SQLITE_MAX_VARIABLE_NUMBER` is 32,766. `MAX_QUERY_ARMS_PER_STATEMENT` is 400 and
bounds arms. Nothing bounds parameters, at two sites. Both are shown here as a
`prepare` (or a `query`) that **actually fails**, with the parameter count that
did it:

| site | items | tags/item | parameters | limit | SQL bytes | outcome |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `Selectivity::read_for` | 300 | 128 | 38,400 | 32,766 | — | `too many SQL variables` |
| `query_sql::chunks` | 400 | 128 | 51,600 | 32,766 | 3,229,593 | `too many SQL variables` |

Three things make the second row worse than the first.

* `chunks` reports the plan as **valid**: `plan.len() == 1`, and the shipped
  public `SqliteEventStore::planned_statement_count(&query)` agrees it is `1`
  (`selectivity_cost.rs:190,208`). The statement it counted cannot be prepared.
* 400 items is exactly `MAX_QUERY_ARMS_PER_STATEMENT` — the boundary the adapter
  chose for itself. A caller at the documented chunk width, carrying the
  documented maximum tags, produces an unpreparable statement.
* The string is **3.2 MB** long, built before the failure.

`Selectivity::read_for` fails first, because it runs before `chunks` on both
callers — and on the append path it runs inside `BEGIN IMMEDIATE`, so the failure
arrives as `AppendError::Store` with the write lock held and the caller's
decision already made.

## 3. What this page does not show

* **These are CPU-time figures for one host** — 13th Gen Intel Core i9-13905H,
  `--release`, allocator warmed once before sampling. The ratio is the durable
  part; the milliseconds are not.
* **No claim that any application builds this query.** VT-23's 128 items is a
  conformance floor a store must survive, not a workload profile. What the page
  establishes is that a query the specification requires to work costs a quarter
  of a second of planning per call, and that a slightly wider one cannot be
  prepared at all.
* **The parameter-limit rows are `prepare`/`query` failures on the transcription
  and on `chunks`, not an `append` round trip.** The claim that this reaches a
  caller as `AppendError::Store` is read off the call sites, not measured
  end to end.
* **`SQLITE_MAX_VARIABLE_NUMBER` is a compile-time constant** and this is
  `rusqlite`'s `bundled` SQLite 3.53.2 at its default. A build with a raised
  limit moves the threshold; it does not create a bound where there is none.
