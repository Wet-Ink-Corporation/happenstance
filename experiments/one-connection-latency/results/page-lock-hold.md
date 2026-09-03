# Page lock-hold and residency — instrument (b)

Written by hand from [`raw/page-lock-hold.txt`](raw/page-lock-hold.txt).
Conditions are in [`../README.md`](../README.md#conditions).

> **The question.** How long does one read page hold the connection mutex, what
> does `PAGE_SIZE` actually control, and what does a caller sharing the handle
> pay for it?

## The construction

One million events on one file, one handle, and a replay driven page by page
while a second task appends **through the same handle**. Twelve configurations:
`PAGE_SIZE ∈ {64, 128, 512, 2048}` crossed with query width `∈ {1, 400, 1200}`
items. 1,000,000 events, 128-byte payload, two tags each, seeded in batches of
256 in 85.6 s.

**The width axis is not a selectivity axis**, and the log is shaped to keep it
that way (`src/workload.rs`): every event carries `all:1`, so the 1-item query
matches all 10^6; every event carries `w:<n mod 1200>`, so the 1,200-item query
also matches all 10^6 and the 400-item query matches a third. What width changes
is the **plan** — `ceil(arms / MAX_QUERY_ARMS_PER_STATEMENT)` statements per
page, which is 1, 1 and 3. Both are single-tag query items, which keeps
`Selectivity::read_for` out of the timed region entirely (it issues no statement
for a query whose every item names at most one tag).

**Two passes per configuration, because one number would be two.** The
peak-live-bytes counter is a process-global `#[global_allocator]` — it has to be,
because the page runs inside a `spawn_blocking` closure — so a peak taken while
an appender is running is a number about the appender. The residency pass
therefore runs the replay alone, and the latency pass takes no residency figure.
`run.sh` passes `--test-threads=1` for the same reason.

**Three numbers per page, answering three questions.** *wait* is how long
`fetch_page`'s `connection.lock()` blocked — what the page pays for sharing.
*hold* is how long the guard was live — what the page costs everyone else, and
the number J-5 is about. *peak* is live bytes at the worst instant inside the
hold — the number R-1 is about. A single "time in `fetch_page`" histogram would
fold wait into hold, and on a busy handle the wait can be the larger half, at
which point the figure says nothing about `PAGE_SIZE` at all.

A configuration stops at 200 pages or 15 s, whichever comes first, and every row
prints the page count it actually reached.

## Shape — what `PAGE_SIZE` and width buy a page

| page | width | stmts | rows/page | peak bytes | bytes/row |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 64 | 1 | 1 | 64 | 30,720 | 480 |
| 128 | 1 | 1 | 128 | 61,440 | 480 |
| **512** | 1 | 1 | 512 | 245,826 | 480 |
| 2048 | 1 | 1 | 2,048 | 983,220 | 480 |
| 64 | 400 | 1 | 64 | 101,330 | 1,583 |
| 128 | 400 | 1 | 128 | 122,834 | 960 |
| **512** | 400 | 1 | 512 | 251,748 | 492 |
| 2048 | 400 | 1 | 2,048 | 982,490 | 480 |
| 64 | 1200 | **3** | **192** | 179,180 | 933 |
| 128 | 1200 | **3** | **384** | 227,688 | 593 |
| **512** | 1200 | **3** | **1,536** | 811,100 | 528 |
| 2048 | 1200 | **3** | **6,144** | 3,244,482 | 528 |

**The `ceil(arms / 400) × PAGE_SIZE` multiplier is real and it is exactly 3.**
`rows/page` is the merge buffer's length at its maximum, sampled *before*
`merged.truncate(budget)` — 192, 384, 1,536 and 6,144 against page sizes of 64,
128, 512 and 2,048. J-5's remediation says the lock hold is
`ceil(arms/400) × PAGE_SIZE` rows and not `PAGE_SIZE` rows; here it stops being
arithmetic.

And the extra rows are **discarded**. Each of the three chunk statements carries
its own `LIMIT budget`, the results are merged, sorted, deduplicated and then
truncated back to `budget`, and the cursor resumes from the last row it *kept*.
So a 1,200-item query decodes three rows through `row::to_event` for every one it
yields, and re-decodes the other two on the next page. The residency column shows
the same 3x: 811,100 bytes at page 512 width 1,200 against 245,826 at width 1.

`bytes/row` is 480 B for a 128-byte payload with two tags at every page size on
the one-item query — 3.75x the payload, which is what a `SequencedEvent` and its
decoded tag set cost in the merge buffer. The inflated figures at the narrow ×
wide cells (1,583 B/row at page 64 × width 400) are a **fixed** per-page cost —
the statement text and its bound parameters — divided by very few rows; by page
512 it has amortised to 492 and by 2,048 to 480. That decomposition is an
inference from the shape of the column, not a separate measurement: the counter
reports live bytes and does not itemise them.

## Hold — how long one page owns the connection mutex

| page | width | pages | quiet p50 | conc p50 | conc p99 | conc max |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 64 | 1 | 97 | 187.468 ms | 148.584 ms | 211.494 ms | 211.494 ms |
| 128 | 1 | 92 | 142.018 ms | 151.188 ms | 248.534 ms | 248.534 ms |
| **512** | 1 | 94 | **146.560 ms** | **151.786 ms** | 225.811 ms | 225.811 ms |
| 2048 | 1 | 85 | 202.576 ms | 171.090 ms | 269.621 ms | 269.621 ms |
| 64 | 400 | 65 | 223.911 ms | 232.777 ms | 341.714 ms | 341.714 ms |
| 128 | 400 | 67 | 252.771 ms | 215.047 ms | 299.682 ms | 299.682 ms |
| **512** | 400 | 65 | **203.339 ms** | **227.495 ms** | 341.807 ms | 341.807 ms |
| 2048 | 400 | 62 | 213.235 ms | 238.210 ms | 333.116 ms | 333.116 ms |
| 64 | 1200 | 22 | 680.729 ms | 678.309 ms | 849.471 ms | 849.471 ms |
| 128 | 1200 | 23 | 722.847 ms | 657.242 ms | 944.689 ms | 944.689 ms |
| **512** | 1200 | 23 | **684.192 ms** | **640.683 ms** | 858.856 ms | 858.856 ms |
| 2048 | 1200 | 22 | 631.711 ms | 666.190 ms | 866.684 ms | 866.684 ms |

**This is the surprising table, and it says `PAGE_SIZE` is not the knob.**

Across a **32x** change in page size — 64 to 2,048 — the per-page hold does not
move:

| width | conc p50 at 64 | at 128 | at 512 | at 2048 | spread |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 148.6 ms | 151.2 ms | 151.8 ms | 171.1 ms | **1.15x** |
| 400 | 232.8 ms | 215.0 ms | 227.5 ms | 238.2 ms | **1.11x** |
| 1200 | 678.3 ms | 657.2 ms | 640.7 ms | 666.2 ms | **1.06x** |

The quiet column is noisier (1.43x at width 1, and non-monotonic — page 64 is
*slower* than page 512) but tells the same story. What does move the hold is
query width: at the shipped 512, **146.6 → 203.3 → 684.2 ms** for widths 1, 400
and 1,200, a 4.67x rise driven by the statement count going 1 → 1 → 3.

So the per-row cost collapses as the page grows: at width 1, page 64 spends
148.584 ms on 64 rows (2.32 ms/row) and page 2,048 spends 171.090 ms on 2,048
rows (0.084 ms/row) — **28x cheaper per row**. Nearly all of a page's cost is a
fixed per-page cost that `LIMIT` does not touch.

**This experiment did not measure the mechanism** — no `EXPLAIN QUERY PLAN` was
run here. It is consistent with what the sibling experiment recorded:
`experiments/shipped-append-condition-sql/results/query-plans.md` finds that
`fetch_page`'s `position IN (SELECT position FROM event)` clause makes SQLite
drive the query from the IN-operator instead of from the `position` range it
already has, costing 846x at page 1,000. A per-page cost that is nearly
independent of `LIMIT` is what that plan predicts. Stated as consistent-with, not
as proven here.

## Cost — what a caller sharing the handle pays

Quiet appender baseline, no replay running: **p50 0.118 ms, p99 7.727 ms,
max 15.647 ms** over 11,141 appends in 3 s.

| page | width | appends | append p50 | **append p99** | append max | page wait p50 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 64 | 1 | 1,174 | 0.100 ms | 184.339 ms | 447.438 ms | 0.921 ms |
| 128 | 1 | 1,177 | 0.099 ms | 202.897 ms | 281.856 ms | 1.365 ms |
| **512** | 1 | 985 | **0.103 ms** | **186.261 ms** | 226.395 ms | 0.836 ms |
| 2048 | 1 | 781 | 0.108 ms | 211.136 ms | 270.119 ms | 0.774 ms |
| 64 | 400 | 546 | 0.107 ms | 299.072 ms | 525.941 ms | 0.772 ms |
| 128 | 400 | 1,029 | 0.094 ms | 251.117 ms | 452.655 ms | 1.216 ms |
| **512** | 400 | 603 | **0.105 ms** | **259.072 ms** | 642.135 ms | 1.000 ms |
| 2048 | 400 | 577 | 0.129 ms | 281.923 ms | 333.548 ms | 0.580 ms |
| 64 | 1200 | 261 | 0.109 ms | 819.580 ms | 852.902 ms | 1.572 ms |
| 128 | 1200 | 201 | 0.169 ms | 782.339 ms | **945.195 ms** | 0.924 ms |
| **512** | 1200 | 280 | **0.148 ms** | **799.041 ms** | 859.825 ms | 1.726 ms |
| 2048 | 1200 | 163 | 0.150 ms | 841.332 ms | 867.250 ms | 0.593 ms |

**The median is untouched and the tail is destroyed.** Append p50 stays at
0.094–0.169 ms in every one of the twelve cells, against a quiet baseline of
0.118 ms. Append p99 goes from 7.727 ms to:

| at the shipped `PAGE_SIZE = 512` | p99 | against the quiet p99 |
| --- | ---: | ---: |
| width 1 | 186.261 ms | **24.1x** |
| width 400 | 259.072 ms | **33.5x** |
| width 1,200 | 799.041 ms | **103.4x** |

Worst observed append anywhere in the run: **945.195 ms**.

That is J-5's wrong-outcome sentence — *"command latency becomes quantised by the
replay's page time"* — confirmed precisely, and sharpened. It is quantised at the
tail, not at the median: an append that arrives while no page is in flight is as
fast as ever, and an append that arrives while one is in flight waits for the
whole of it. Which of the two happens is a coin flip the application does not
control.

**And the cost is asymmetric.** At page 512 × width 1,200 the page holds the
mutex for 640.683 ms (conc p50) and waits **1.726 ms** for it. The reader imposes
371x what it pays. The appender is short, so it rarely makes a page wait; the
page is long, so it routinely makes the appender wait.

The `appends` column is a throughput proxy, and the collapse is visible in it. At
page 512 × width 1: 985 appends over a pass of roughly 94 × 151.8 ms = 14.3 s, so
about 69/s, against the quiet baseline's 11,141 / 3 s = 3,714/s — **54x fewer**.
At page 512 × width 1,200: 280 appends over roughly 23 × 640.7 ms = 14.7 s, about
19/s, **195x fewer**. Both are derived from two measured columns rather than
measured directly, and the arithmetic is shown so a reader can check it.

## Extrapolation — a full 10^6-event replay

**Labelled extrapolation everywhere it appears.** `pages_total × quiet hold_p50`.
`pages_total` is `ceil(10^6 / PAGE_SIZE)`, because a page *yields* `PAGE_SIZE`
rows however many it fetched.

| page | width | pages total | held mutex |
| ---: | ---: | ---: | ---: |
| 64 | 1 | 15,625 | 2,929.2 s |
| 128 | 1 | 7,813 | 1,109.6 s |
| **512** | 1 | 1,954 | **286.4 s** |
| 2048 | 1 | 489 | 99.1 s |
| 64 | 400 | 15,625 | 3,498.6 s |
| 128 | 400 | 7,813 | 1,974.9 s |
| **512** | 400 | 1,954 | **397.3 s** |
| 2048 | 400 | 489 | 104.3 s |
| 64 | 1200 | 15,625 | 10,636.4 s |
| 128 | 1200 | 7,813 | 5,647.6 s |
| **512** | 1200 | 1,954 | **1,336.9 s** |
| 2048 | 1200 | 489 | 308.9 s |

**Here `PAGE_SIZE` does control something, and it is the aggregate.** Because the
per-page hold is flat and the page *count* is `10^6 / PAGE`, total held-mutex
time over a whole replay scales as `1/PAGE`:

| width | 512 → 2048 | 512 → 64 |
| ---: | ---: | ---: |
| 1 | 286.4 s → 99.1 s, **2.9x better** | → 2,929.2 s, **10.2x worse** |
| 400 | 397.3 s → 104.3 s, **3.8x better** | → 3,498.6 s, **8.8x worse** |
| 1200 | 1,336.9 s → 308.9 s, **4.3x better** | → 10,636.4 s, **8.0x worse** |

So the two halves of the table have to be read together, and the answer to J-5 is
neither "512 is fine" nor "512 is wrong":

* Per page, `PAGE_SIZE` buys nothing. A caller's *worst single wait* is set by
  query width, not by the page size, and no value of `PAGE_SIZE` in this range
  brings it under 148 ms.
* Per replay, `PAGE_SIZE` buys almost everything. Raising it from 512 to 2,048
  removes 187 s of held mutex from a 10^6-event rebuild at width 1, and 1,028 s
  at width 1,200.
* And raising it is exactly what R-1 says is unsafe, because the thing that grows
  is unbounded in bytes. See [`ceiling-residency.md`](ceiling-residency.md): at
  512 rows of ceiling-sized events, one page is already 512 MiB.

**The two findings are in direct tension and neither can be settled by a row
count.** The measured answer is that `PAGE_SIZE` should be raised *and* a byte
budget added beside it — R-1's remediation — because raising the row budget
without one raises exactly the number that has no ceiling. Doing either alone
makes something worse.

## What this page does not show

1. **One machine, one run, one cell each.** No repetition, no confidence
   interval. `experiments/append-condition` records 45% run-to-run variance on
   this host, and the quiet-hold column's non-monotonicity (page 64 slower than
   page 512 at width 1) is inside that. The claims here are about ratios and
   orders of magnitude between cells of one interleaved run.
2. **128-byte payloads.** Every figure on this page is taken over a log of
   128-byte events. The residency column says nothing about R-1, which lives at
   the other end and has its own page.
3. **Single-tag query items only.** Every item names one tag, which is the common
   consistency-boundary shape and the one `query_sql.rs` documents a fast path
   for — and it keeps `Selectivity::read_for` out of the timed region entirely.
   A multi-tag query issues cardinality lookups and an intersection chain inside
   the same lock hold, and none of that is measured here.
4. **The mechanism behind the flat hold was not measured.** No `EXPLAIN QUERY
   PLAN` was run in this experiment. The IN-operator explanation is imported from
   `experiments/shipped-append-condition-sql` and is offered as consistent-with,
   not as established here.
5. **Peak bytes is live-bytes from a counting allocator, not RSS.** It is the sum
   of `layout.size()` over allocations live at the worst instant of the region. It
   excludes the system allocator's size-class rounding and its retained free
   pages, and it *includes* SQLite's page cache, which is `malloc`'d through the
   same allocator. It is a lower bound on what the page costs the process.
6. **The extrapolation column is arithmetic, not a run.** No configuration was
   driven to exhaustion; the longest pass was 200 pages or 15 s. It assumes the
   sampled `hold_p50` holds for all 15,625 pages, which nothing here tested — page
   cost may well rise as the cursor moves deeper into the log.
7. **The appender is four 64-byte events with one tag.** A realistic command
   handler's append is larger and its own transaction is longer, so the p50 it
   would show quietly is higher than 0.118 ms and the ratios above would compress.
8. **This is an experiment, never a gate step** (CF-34).
