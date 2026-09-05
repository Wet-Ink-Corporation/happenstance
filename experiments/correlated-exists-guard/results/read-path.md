# The read path, and I-3's outer wrapper

Written by hand from [`raw/read-path.txt`](raw/read-path.txt), one `./run.sh` on
2026-09-05. Two 500,000-event stores, one page of 512 rows per statement, 10
round-robin rounds per arm, full-row projection.

## Two questions, and the second is the one worth reading

1. **Does the correlated chain's win transfer to the read path?** Partly. The
   guard is wrapped in `SELECT max(position) FROM (…)`, and `max()` over a
   position-ordered index lets SQLite stop at the first satisfying row. `read`
   has no `max()`.
2. **What can be done about the second materialisation the read path adds?**
   That is finding I-3, it is now the read path's floor, and this page measures
   candidates for it rather than choosing between them.

## The measurements

`us_median`, one 512-row page. `IN + exists` is what the adapter emits **today**,
after this crate's findings were adopted; `IN + old chain` is what it emitted
before.

| corpus | matched | page | `IN` + old chain | **`IN` + exists (ships)** | `JOIN` + exists | **`EXISTS` wrapper** |
| --- | ---: | --- | ---: | ---: | ---: | ---: |
| selective | 5,155 | first | 335,531 | **21,373** | 48,257 | 48,521 |
| selective | 5,155 | mid-replay | 344,249 | **36,733** | 24,685 | 53,554 |
| unselective | 500,000 | first | 1,743,431 | 1,023,761 | 424,016 | **940** |
| unselective | 500,000 | mid-replay | 1,757,546 | 1,000,823 | 203,743 | **937** |

## 1. The correlated chain transfers, at a fraction of the size

Against the old chain under the shipped `IN` wrapper: **9.4x–15.7x** on the
selective corpus, **1.7x** on the unselective one — against **1,617x–1,681x**
for the guard at the same store size.

So the early-exit reading [`unselective-pair.md`](unselective-pair.md) proposed
is **confirmed**: most of the guard's margin was `max()` stopping at the first
satisfying row, and the read path cannot do that. The rewrite is still worth
having on both — it never loses a cell — but a reader sizing a projection runner
should quote this page and not `guard-cost.md`.

## 2. Why: the read path adds a second materialisation, and it is not the chain's

`EXPLAIN QUERY PLAN`, unselective corpus, first page, same run.

**`IN` wrapper + old chain** — two materialisations:

```
id=4  parent=0   SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
id=8  parent=0   LIST SUBQUERY 2      <- the read path's own wrapper
id=10 parent=8     SEARCH event_tag USING PRIMARY KEY (tag=? AND position=?)
id=16 parent=8     LIST SUBQUERY 1    <- the chained membership test
id=18 parent=16      SEARCH event_tag USING PRIMARY KEY (tag=?)
```

**`IN` wrapper + correlated chain** — one:

```
id=4  parent=0   SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
id=8  parent=0   LIST SUBQUERY 2      <- STILL THERE
id=12 parent=8     SEARCH seed USING PRIMARY KEY (tag=?)
id=20 parent=8     SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)
```

`LIST SUBQUERY 1` is gone — the rewrite doing exactly what
[`query-plans.md`](query-plans.md) says. **`LIST SUBQUERY 2` is not**, and it is
not the chain's: it is `fetch_page`'s own `WHERE position IN (<matched>)`, which
is uncorrelated whatever sits inside it. **The `LIMIT 512` cannot help**, because
that list is built in full before a single row is emitted — which is why one page
over a broad query costs a second on a half-million-event store.

## 3. The candidates, and the crossover

**`EXISTS` wrapper** drives the rowid range scan over `event` — already in
position order, so it can honour `ORDER BY position ASC LIMIT ?` by stopping —
and tests membership per row:

```
id=6  parent=0   SEARCH event USING INTEGER PRIMARY KEY (rowid>? AND rowid<?)
id=12 parent=0   CORRELATED SCALAR SUBQUERY 1
id=16 parent=12    SEARCH c1 USING PRIMARY KEY (tag=? AND position=?)
id=27 parent=0   SEARCH c0 EXISTS USING PRIMARY KEY (tag=? AND position=?)
```

**No `LIST SUBQUERY` anywhere.** And the result is a clean crossover:

| corpus | `EXISTS` wrapper against what ships |
| --- | --- |
| selective (1 event in 97) | **2.3x worse** (48,521 against 21,373) |
| unselective (every event) | **1,089x better** (940 against 1,023,761) |

That is exactly what the shape predicts. The `EXISTS` wrapper walks `event` until
the page is full, so it is best when almost every row matches and worst when
almost none does. The `IN` wrapper materialises the matched set, so it is best
when that set is small. **Neither is right everywhere.**

`JOIN` is a middle: 2.3x worse than shipped on the selective corpus, 2.4x–4.9x
better on the unselective one. It never wins a corpus outright, so it is the one
candidate this page would drop.

## 4. What an approach looks like

**The asymmetry decides the shape of the rule.** Choosing wrong towards `IN`
costs up to **1,089x**; choosing wrong towards `EXISTS` costs **2.3x**. A rule
does not need to be accurate, only to catch the unselective case — and the
adapter already reads the number it would need, in `Selectivity::read_for`.

Three parts, in increasing order of what they cost to decide:

1. **`Query::all` needs no judgement at all.** `chunks` short-circuits it to
   `SELECT position FROM event`, so `fetch_page` emits
   `WHERE position IN (SELECT position FROM event)` — a literal tautology over
   the whole table. Omitting the wrapper when the chunk matches every event is
   unconditionally correct and needs no cardinality estimate. It is also the
   commonest read in the library: a projection runner catching up.

   **This one shipped**, and measuring it first was worth doing: the reason
   given here — that the tautology is materialised — turned out to be false for
   this statement, because `position` is the rowid and SQLite drives the scan
   from the subquery instead. The real defect is worse. See
   [`all-query-wrapper.md`](all-query-wrapper.md).
2. **A cardinality-conditional wrapper for tagged queries.** Emit the `EXISTS`
   wrapper when the item's most selective tag still matches a large fraction of
   the log, and `IN` otherwise. The threshold wants measuring — this page has two
   points, 1/97 and 1/1, and the crossover is somewhere between them.
3. **Nothing for the middle until (2)'s threshold is measured.** A rule fitted to
   two points is a rule fitted to two points.

**Since this page was written, (2) has become the wrong question.** A fourth
candidate pushes the read's window and page budget **into each arm** instead of
applying them outside the wrapper, which bounds the matched set at
`budget x arms` whatever the corpus. It was then measured on this crate's
harness, and it wins **both** corpora, all three depths and the backwards cell —
so the crossover this section is about does not arise, and no threshold is
needed to navigate it. See [`windowed-arms.md`](windowed-arms.md); §4 above is
kept as written because it is the reasoning that produced the question.

**(1) shipped; (2) and (3) did not.** (2) is a decision about when the adapter
changes its query plan on data it samples, which is ADR-0022's territory and not
an experiment's.

## 5. The arithmetic that says why this matters

500,000 matching events at 512 rows per page is 977 pages. Per-page medians,
multiplied out for one catch-up:

| shape | one full replay |
| --- | ---: |
| `IN` + old chain | ~28 minutes |
| `IN` + exists (ships today) | ~17 minutes |
| **`EXISTS` wrapper** | **~15 seconds** |

The rewrite that shipped is worth a third of that cost. I-3's wrapper is worth
the rest, and it is the difference between a projection rebuild being a coffee
break and being a maintenance window.

## What this page does not show

One machine, one run, 10 rounds per arm, one page size (512, the shipped
`PAGE_SIZE`). No full drain — §5 multiplies per-page medians by hand, and a real
drain would also pay `ReadCursor`'s per-hop `spawn_blocking` and connection
mutex, which `experiments/one-connection-latency/` measures and this does not.
Two tags throughout, and two selectivity points with nothing between them. **No
repair is built:** §4's three parts are named and priced, not implemented.
