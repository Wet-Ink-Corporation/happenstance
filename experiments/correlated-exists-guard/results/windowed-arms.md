# The window belongs inside the arm — at one arm

> **Superseded by [`merge-join.md`](merge-join.md).** A fourth shape — the arms
> merged as a compound with the budget on the compound rather than on each arm,
> which is SQLite's own co-routine merge — wins every cell in both this table
> and the wide one. The pages below are kept because they are how that shape was
> found, and because each records a claim that had to be run to be refuted.

> **Corrected by [`wide-arms.md`](wide-arms.md), which was run afterwards.**
> Every cell on this page is a query of **one item**. At VT-23's floor of 128
> items the conclusion in §1 — that one shape wins both ends, so nothing has to
> be chosen — **does not hold**: the windowed arm is still better than what ships
> in all eight wide cells, but it is 1.4x–3.4x worse than `wrapper-exists` on a
> broad 128-item query, because `budget x arms` is 65,536 positions rather than
> 512. Read this page for the mechanism and that one for what it costs at width.

Written by hand from the `raw/windowed-arms.txt` recorded at commit `429cf05`.
That file has since been **regenerated** by the same test with a fourth shape
added, so the working copy carries a later run than the three columns below; this
page's run is in git, and the current one is in
[`merge-join.md`](merge-join.md). Two 500,000-event
stores, one 512-row page per statement, 10 rounds per cell with the leading
shape rotated, full-row projection.

**The control is the returned page** — every column of every row, in order —
compared across all three shapes every round before any time is recorded. It
held in all eight cells, backwards included.

## The question this answers that [`read-path.md`](read-path.md) did not

That page priced three candidates and found a **crossover**: each is best at one
end of the selectivity axis, so choosing between them means a rule, a threshold,
and an adapter that changes its query plan on data it samples.

All three argue about which side of the join to drive from. None asks why the
read's window — `resume_from`, the ceiling, the direction, the page budget —
stops at the subquery boundary. `fetch_page` applies all four *outside* the
membership test. **The adapter builds that subquery.** It can put them in:

```sql
SELECT position FROM (
  SELECT seed.position AS position FROM event_tag AS seed
  WHERE seed.tag = ? AND seed.position >= ? AND seed.position <= ?
    AND EXISTS (SELECT 1 FROM event_tag AS m0
                WHERE m0.tag = ? AND m0.position = seed.position)
  ORDER BY seed.position ASC LIMIT 512)
```

The inner `SELECT` is required rather than stylistic: SQLite rejects a bare
`LIMIT` on a compound arm — *"LIMIT clause should come after UNION not before"*.

## The measurement

`us_median`, one 512-row page. `in-exists` is what the adapter emits today;
`wrapper-exists` is [`read-path.md`](read-path.md)'s best broad candidate.

| corpus | cell | `in-exists` (ships) | `wrapper-exists` | **`windowed-in-exists`** |
| --- | --- | ---: | ---: | ---: |
| selective | first page | 26,788 | 61,234 | **5,665** |
| selective | mid-replay | 42,206 | 68,473 | **5,530** |
| selective | late replay | 63,656 | 74,103 | **7,264** |
| selective | backwards from head | 33,892 | 77,501 | **6,570** |
| unselective | first page | 1,178,687 | 1,294 | **920** |
| unselective | mid-replay | 1,264,879 | 1,278 | **906** |
| unselective | late replay | 1,250,408 | 751 | 722 |
| unselective | backwards from head | 1,208,409 | 1,246 | **1,070** |

## 1. One shape wins both corpora at this arm count

Against what ships: **4.7x–8.8x** on the selective corpus, **1,129x–1,732x** on
the unselective one.

Against `wrapper-exists`, which is the candidate the crossover was about:
**10.2x–12.4x** better on the selective corpus, and 1.04x–1.4x on the
unselective one — ahead in every cell, though the unselective margin is inside
this host's noise and is not a claim.

**That is the finding, and its scope is one arm.** `read-path.md`'s crossover
was 1,089x one way and 2.3x the other, and a rule to navigate it needs a
threshold, a cardinality estimate and an adapter whose query plan depends on data
it sampled. At this arm count the shape needs none of those, because it is not
better *at one end* — it is better at both.

[`wide-arms.md`](wide-arms.md) then took the same three shapes to VT-23's
128-item floor, and the second half of that sentence stops being true there: the
crossover returns, narrowed from 1,089x/2.3x to 1,747x/3.4x and moved onto a
different pair of shapes. This section is left as written, because it is what
this page's evidence supports and it is the claim the next page had to be run to
check.

## 2. Why: the window reaches the index instead of stopping outside it

`EXPLAIN QUERY PLAN`, unselective corpus, first page:

```
in-exists            SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
                     LIST SUBQUERY 2
                       SEARCH seed USING PRIMARY KEY (tag=?)
                       SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)

windowed-in-exists   SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
                     LIST SUBQUERY 3
                       SEARCH seed USING PRIMARY KEY (tag=? AND position>? AND position<?)
                       SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)
```

One term changes: `(tag=?)` becomes `(tag=? AND position>? AND position<?)`.
`event_tag`'s primary key is `(tag, position)`, so the window is a **seek into
the interior of one contiguous range** rather than a walk from its start, and
the arm's own `LIMIT` stops it after `budget` matches.

The `LIST SUBQUERY` is still there and is now *harmless*: it materialises at most
`budget x arms` positions instead of every matching position in the log. That is
the whole mechanism. Nothing was removed; something was **bounded**.

It also explains the shape of the selective column — 5,665 / 5,530 / 7,264
against 26,788 / 42,206 / 63,656. The windowed arm is flat in replay depth where
the shipped one grows, because the shipped one re-walks the prefix beneath each
page and the windowed one seeks past it.

## 3. Soundness, which is prior to speed

The claim is that bounding each arm by the page budget cannot lose a row the page
needed: an event in the merged top *b* of the union is in some arm, and its rank
within that arm is no worse than its rank in the union.

`fetch_page` **already relies on exactly this**, one level up, to bound each
*chunk* by the page budget and merge the results. Bounding each *arm* is the same
claim one level down.

The returned-page control is what turns that from an argument into evidence, and
it is why every column is compared rather than a position list: an arm ordered
`ASC` under a page ordered `DESC` keeps the oldest `budget` positions instead of
the newest, and returns a page that is **short** rather than obviously wrong. The
backwards cell exists to fire that, and did not.

## 4. What it costs the adapter, which is the part that is not free

`query_sql::chunks` would need the read's window and direction. Today it takes
an append-condition `bound` and nothing else, because the *guard* has no window:
it takes `max(position)` over the whole matched set.

So this is an interface change to the one module both callers share, and the two
callers would stop passing the same shape of argument. That is a design decision
about `query_sql`'s seam, not a local optimisation — which is why it is measured
here and recorded in `references/seeds/adr-0022-shipped-shape-drift.md` rather
than shipped.

## What this page does not show

One machine, one run, 10 rounds per cell, one page size (512, the shipped
`PAGE_SIZE`), one store size, two tags per item, one item per query. **No
multi-item query is measured**, and that is the gap that matters most: the
soundness argument in §3 is about a union of arms, and every cell here has a
union of one. VT-23's 128-item floor is where `budget x arms` is 65,536
positions rather than 512. **That gap has since been measured**, and it changes
the conclusion — [`wide-arms.md`](wide-arms.md), and the banner at the top of
this page.

No full drain — the per-page medians are not multiplied out, because the shipped
shape's growth with depth and the windowed shape's flatness would make any
single multiplier misleading for one of them.

Nothing here is a patch to `happenstance-sqlite`. The shape is built in this
crate only.
