# `Query::all`'s wrapper: the cost was not the one the shape suggests

Written by hand from [`raw/all-query-wrapper.txt`](raw/all-query-wrapper.txt),
`cargo test --release --test all_query_wrapper` on 2026-09-05. One
500,000-event store, one 512-row page per statement, 10 interleaved rounds per
cell, full-row projection. **The control is the returned page** — same rows,
same order — asserted every round before either time is recorded.

This is [`read-path.md`](read-path.md) §4's part 1, measured before it shipped
and then measured again because the first run refuted the reason for doing it.

## The measurement

`us_median`, one page, `Query::all`.

| page (`resume_from`) | `WHERE position IN (SELECT position FROM event)` | **`WHERE 1` (ships)** | ratio |
| --- | ---: | ---: | ---: |
| first (1) | 122 | **100** | 1.2x |
| half-way (250,000) | 22,955 | **147** | **156x** |
| 90% in (450,000) | 56,310 | **182** | **309x** |

## 1. The prediction was wrong, and the plan says why

The change was proposed on the reading that an uncorrelated `IN (…)` is
materialised in full before a row is emitted — which is what
[`query-plans.md`](query-plans.md) established for the *tagged* chain, and what
[`read-path.md`](read-path.md) §2 shows as `LIST SUBQUERY 2`. Predicted, before
the run: a large flat cost, the same at every page.

`EXPLAIN QUERY PLAN`, same run, all three pages, identical:

```
wrapper-in       id=4 SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
                 id=7 USING ROWID SEARCH ON TABLE event FOR IN-OPERATOR
wrapper-omitted  id=4 SEARCH event USING INTEGER PRIMARY KEY (rowid>? AND rowid<?)
```

**No `LIST SUBQUERY`.** `position` *is* the rowid, so SQLite does not materialise
this list at all — it drives the scan **from** the subquery, in the subquery's
order, starting at its first row.

That is a different defect with a different shape:

* the omitted form gets `rowid>? AND rowid<?` — `resume_from` and the ceiling
  become the range the scan runs over, so it starts where the page starts;
* the wrapped form gets `rowid=?` fed by the `IN` list. **A range predicate
  cannot be pushed into a search driven by an `IN` list**, so the page walks the
  whole prefix of the log below its own starting point and discards it.

The measured cost **grows with depth**, and the omitted form does not
(100 / 147 / 182 against 122 / 22,955 / 56,310). How it grows is not settled by
three points: 1.8x the offset cost 2.45x here and 1.75x on a first run of the
same test, so the data are consistent with linear-or-worse and resolve no
further. Nothing below needs it to be resolved — the *most* favourable reading
is the one used.

## 2. Which makes a full replay quadratic in the length of the log

A replay pages forward, so page *k* re-walks `k × 512` positions that the page
before it already walked. Summed over 977 pages of a 500,000-event log, at the
per-page medians above:

| shape | one full `Query::all` replay |
| --- | ---: |
| wrapped | ~22 s |
| **omitted** | **~0.15 s** |

Taking the half-way page's 0.092 µs per skipped position as the constant, and
assuming the growth is exactly linear — the kindest reading of §1's two runs.

The ratio at one page is anywhere from 1.2x to 309x depending where the page
sits, which is why the per-page number is the wrong one to quote. The figure
that matters is the exponent: **O(N²) against O(N)**, and superlinear growth per
page would only make the first worse. At 5,000,000 events the same arithmetic
gives ~37 minutes against ~1.5 seconds, and that is the `references/scenarios/`
shape this bears on — Cold Chain rebuilds 4.1M events inside a 30-second
ceiling.

## 3. What this does *not* say about finding I-3's 846x

Finding I-3 measured 846x for this wrapper. This page neither confirms nor
refutes that number: it is a different cell — a different store, and a page
whose depth the review does not state. What this page establishes is that the
*mechanism* the fix was reasoned from is not the mechanism at work here, and
that the fix is worth having anyway, for a different and worse reason.

Both readings agree on the action and disagree about why, which is the case
where writing the reason down wrong is most expensive: the next person to touch
`fetch_page` would have reached for `LIMIT` pushdown, and `LIMIT` was never the
problem.

## What this page does not show

One machine, one run, 10 rounds per cell, one page size (512, the shipped
`PAGE_SIZE`), one store size. §2 multiplies per-page medians by hand rather than
draining a replay, and a real drain would also pay `ReadCursor`'s per-hop
`spawn_blocking` and connection mutex — `experiments/one-connection-latency/`
measures those and this does not. Three depths establish that the cost grows
with depth and are not enough to say how; §2's constant is taken from one of
them and is an order of magnitude, not a coefficient.

Only `Query::all` is measured here. The **tagged** wrapper is a different
statement with a different plan and a genuine crossover between its candidates;
that is [`read-path.md`](read-path.md), and none of its candidates shipped.
