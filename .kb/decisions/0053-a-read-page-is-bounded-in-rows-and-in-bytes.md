---
id: kb-decision-0053
title: A read page is bounded in rows and in bytes, and the caller states neither
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0053
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Two adapter-stated constants bound a read page: PAGE_SIZE stays private at 512 rows, and the new
  public MAX_PAGE_BYTES_PER_STATEMENT bounds it at eight times MAX_EVENT_DATA_LEN. The connection
  is taken per statement rather than held per page. A caller-stated budget is not taken for 0.2.0
  and stays open as a public-surface question — does the budget belong to the read, the store, or
  the contract. An unmeasured 8 MiB bound binds only workloads above every measured one, which is
  better than no bound.
depends_on:
  - kb-decision-0011
related:
  - kb-open-question-read-page-budget-001
  - kb-reference-one-connection-latency-001
  - kb-open-question-event-metadata-no-floor-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/read-page-budget-rows-bytes-or-caller.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# A read page is bounded in rows and in bytes, and the caller states neither

## Decision

`happenstance-sqlite`'s read path is bounded by two adapter-stated constants
rather than one. `PAGE_SIZE` stays private and unchanged at `512` rows —
bounding cardinality — and a new public
`SqliteEventStore::MAX_PAGE_BYTES_PER_STATEMENT`, set to `8 *
MAX_EVENT_DATA_LEN`, bounds one page's payload residency in bytes. Neither is
configurable: there is no `ReadOptions`-adjacent knob and no builder. Ratified
as "A for `0.2.0`; B open," meaning Option A — two adapter-stated constants — is
what ships now, and a caller-stated budget (Option B) is deliberately left as
an open question rather than decided on the strength of one machine's run.

A third, non-decision change landed alongside these for free: the connection
is now taken per statement rather than held for the whole page. That has no
public surface and was previously the correctness gap making `PAGE_SIZE`'s own
documentation false — it said "one `spawn_blocking` hop" while the code
actually held the mutex across the selectivity lookup, every statement of the
plan, and the merge. It is sound under ADR-0011's ceiling: every statement
carries `position <= H`, so whatever commits between two statements of one
read is invisible to all of them regardless of how the connection is acquired.

## Why two axes, not one

The audit had found `PAGE_SIZE` and the page's byte residency pulling in
opposite directions on a single knob: the lock-hold table favoured raising
`PAGE_SIZE` (measured aggregate lock hold over 10⁶ events at width 1 drops from
2,929.2s at `PAGE_SIZE` 64 to 99.1s at 2,048), while the residency arm found the
same raise unsafe, because one page's byte size scales linearly with row count
and has no independent ceiling — at 2,048 rows and payloads near the data
ceiling, one page would be roughly 2 GiB. One knob in the wrong unit cannot
co-optimise both consequences for any value. `MAX_PAGE_BYTES_PER_STATEMENT`
supplies the missing second axis: without a byte budget, nothing bounds a
page's residency except SQLite's own near-gigabyte blob limit, and a limit
nothing measures and nothing constrains is a number, not a bound. The row
budget is kept rather than deriving everything from the byte budget alone,
because it is not decorative either — it is what `Vec::with_capacity` sizes
against and what binds `LIMIT ?`, and a page of a million tiny events would
otherwise be one statement returning a million rows to spend the 8 MiB.

## Why the number is unmeasured, and why that is still better than nothing

`8 * MAX_EVENT_DATA_LEN` is stated in the same register `MAX_EVENT_DATA_LEN`
itself is — a fact about this adapter's ceiling, not a trade weighed against a
workload. No replay was run at 4 MiB or 32 MiB and compared against 8 MiB; the
value was chosen for how it reads ("eight maximal payloads") rather than
derived from a measurement, which is the same category of placeholder the old
undocumented `PAGE_SIZE` comment used to apologise for. What answers that
objection only partly, and is stated as a partial answer rather than a full
one: the two constants are not the same kind of claim. `PAGE_SIZE`'s value
changes outcomes for every workload — 64 versus 2,048 moves the aggregate lock
hold thirty-fold — while `MAX_PAGE_BYTES_PER_STATEMENT` moves nothing for any
workload below it, and every workload in this repository's own tests and
examples sits below it. It binds only payloads near the data ceiling, where
the prior behaviour was to allocate without limit at all. A wrong bound stated
plainly is judged strictly better than no bound, and it is the *value* that
stays open for revision, not the presence of a mechanism.

## What stays open

Whether the budget belongs to the read, the store, or the contract — a
caller-stated budget on or beside `ReadOptions` would put the number where the
knowledge of a deployment's memory ceiling actually lives, but `ReadOptions` is
`happenstance-core`'s, so an adapter-specific knob cannot live there without
becoming a contract obligation every adapter must answer, including buffering
ones with no comparable constraint. A `SqliteEventStore`-local builder is the
alternative and makes construction fallible or stateful in a way it is not
today. Both are real public-surface decisions with an ADR's worth of
consequences, and neither is taken here. Also open: whether `PAGE_SIZE` itself
should become public, since a caller today can compute the byte bound on a page
but not its row bound; whether a wide query spanning several statements should
share one budget across them rather than each statement getting its own (
rejected as a mechanism here, not a measurement — the first statement would
spend the shared budget and every later one would be cut to a single row); and
`happenstance-sqlite`'s write-side gap this decision does not touch — nothing
bounds `metadata` at `append`, so the read-side byte budget protects a read
against a row that the write path never refused in the first place.
