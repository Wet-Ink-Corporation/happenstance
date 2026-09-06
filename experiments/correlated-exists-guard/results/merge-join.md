# The shape was already in SQLite, and it wins both floors

Written by hand from [`raw/windowed-arms.txt`](raw/windowed-arms.txt) and
[`raw/wide-windowed-arms.txt`](raw/wide-windowed-arms.txt), `cargo test --release`
on those two targets on 2026-09-05. 500,000-event stores, one 512-row page per statement, full-row
projection, the returned page compared column-by-column across all four shapes
every round before any time is recorded.

**This page supersedes the recommendation in
[`windowed-arms.md`](windowed-arms.md) and [`wide-arms.md`](wide-arms.md).** They
are kept because they are how this shape was found.

## What was wrong with the previous three candidates

All of them — `in-exists` (what ships), `wrapper-exists` and
`windowed-in-exists` — answer *"which positions match?"* by producing the whole
matched set and then taking a page from it. They differ only in what they do
with that set: materialise it, test it per row, or truncate it per arm. That is
why each is best at one end of some axis and why every new axis produced a new
crossover.

The question a paged read actually asks is *"the next `budget` matching
positions in order"*, and the standard answer to that is a **merge of ordered
streams with early termination**, not a set operation. SQLite has it:

> An alternative method of computing a compound is to run each subquery as a
> co-routine, arrange for their outputs to appear in sorted order, and merge the
> results together. […] because the co-routine doesn't need to run to completion
> before the outer query begins, the first rows appear sooner, and if the overall
> query is abandoned before finishing, less work is done overall.
> — <https://sqlite.org/lang_select.html>

PostgreSQL's planner does the same thing under the name `MergeAppend`, and has
pushed `LIMIT` into `UNION ALL` arms since 2005. **`windowed-in-exists` was a
hand-rolled, worse version of an optimisation both engines already have** — worse
because a per-arm `LIMIT` bounds the work at `budget x arms` where a merge bounds
it at `budget`.

## The shape

```sql
SELECT event.<cols> FROM event JOIN (
    SELECT seed.position AS position FROM event_tag AS seed
    WHERE seed.tag = ? AND seed.position >= ? AND seed.position <= ?
      AND EXISTS (SELECT 1 FROM event_tag AS m0
                  WHERE m0.tag = ? AND m0.position = seed.position)
  UNION
    …one arm per query item…
  ORDER BY position ASC LIMIT ?
) AS m ON m.position = event.position
ORDER BY event.position ASC
```

Three things are load-bearing and each was got wrong at least once here.

* **The `LIMIT` is on the compound, once.** SQLite's grammar allows it nowhere
  else, and that restriction turns out to be the feature: it is what lets the
  planner merge rather than truncate.
* **The arms carry the window and not a limit.** Without `position >= ?` each
  co-routine rewinds to the start of its tag range and a late page pays for the
  prefix beneath it. With a per-arm `LIMIT` the merge is gone.
* **`JOIN`, not `position IN (…)`.** `IN` materialises the compound before
  emitting a row, which throws the early exit away — that is the whole of finding
  I-3.

The plan says `MERGE (UNION)` over a binary tree of `SEARCH event_tag USING
PRIMARY KEY (tag=? AND position>? AND position<?)`. **`USE TEMP B-TREE FOR ORDER
BY` inside the compound means the planner declined**, and the shape has become a
sort of the whole matched set. That is the observable to assert on.

## One arm — `windowed-arms.md`'s two corpora, 500,000 events

| corpus | cell | `in-exists` (ships) | `wrapper-exists` | `windowed` | **`merge-join`** |
| --- | --- | ---: | ---: | ---: | ---: |
| selective | first page | 24,665 | 54,872 | 5,582 | **4,669** |
| selective | mid-replay | 39,030 | 55,801 | **4,131** | 4,872 |
| selective | late replay | 46,169 | 49,639 | 5,544 | **3,993** |
| selective | backwards | 18,952 | 52,527 | **3,975** | 4,785 |
| unselective | first page | 908,541 | 751 | 528 | **527** |
| unselective | mid-replay | 928,931 | 773 | 638 | **595** |
| unselective | late replay | 930,602 | 673 | 514 | **467** |
| unselective | backwards | 866,210 | 829 | 802 | **730** |

## 128 arms — `wide-arms.md`'s two queries, VT-23's floor

| query | cell | `in-exists` (ships) | `wrapper-exists` | `windowed` | **`merge-join`** |
| --- | --- | ---: | ---: | ---: | ---: |
| partition-128 | first page | 654,774 | 87,479 | 326,473 | **7,985** |
| partition-128 | mid-replay | 636,387 | 91,047 | 313,745 | **7,011** |
| partition-128 | late replay | 685,001 | 93,486 | 110,462 | **6,400** |
| partition-128 | backwards | 665,135 | 80,383 | 297,280 | **6,171** |
| needle-128 | first page | 32,477 | 19,779,043 | 14,683 | **12,175** |
| needle-128 | mid-replay | 33,937 | 18,531,612 | 14,466 | **10,733** |
| needle-128 | late replay | 43,452 | 17,502,855 | 10,628 | **10,171** |
| needle-128 | backwards | 33,157 | 21,107,773 | 12,650 | **11,087** |

## 1. It wins sixteen cells out of sixteen, or ties

Against **what ships**: 4.0x–11.6x (selective, 1 arm), 1,187x–1,993x
(unselective, 1 arm), 82x–108x (partition, 128 arms), 2.7x–4.3x (needle, 128
arms).

Against **`wrapper-exists`**, the candidate `read-path.md` preferred: ahead
everywhere, by 11x–15x on the wide broad query and 1,625x–1,904x on the wide
selective one.

Against **`windowed-in-exists`**: 17x–48x on the wide broad query, 1.0x–1.3x
elsewhere — and two cells where it is nominally behind (selective mid-replay and
backwards, 0.8x) which are inside this host's noise and where both shapes are
4x–10x ahead of what ships.

**No cell in either table has a different winner.** That is the first time in
this crate that has been true, and it is what makes a default defensible where
the previous three shapes could only support a threshold.

## 2. Why it does not have a crossover to have

The other three all scale with the **matched set**: 500,000 positions for a broad
query, whatever the page wants. This one scales with `budget` plus the cost of
advancing `arms` cursors, and neither term depends on how much of the log
matches. That is why `partition-128` — the cell that destroyed every previous
shape — is its *cheapest* wide cell at 6,171–7,985 µs.

The residue is arm count: 128 co-routines cost about 1.6x one co-routine
(12,175 against 4,669 on comparable selective cells). That is a bounded,
predictable, monotone cost with no crossing.

## 3. What it costs the adapter

`query_sql::chunks` must take the read's window, direction and budget, which only
the read path has — the guard takes `max(position)` over the matched set and has
no window at all. The two callers of the module they deliberately share stop
passing the same shape of argument.

`fetch_page`'s own `WHERE position >= ? AND position <= ? … LIMIT ?` disappears
into the compound, so the wrapper stops existing rather than being replaced.

## What this page does not show

Two arm counts — 1 and 128 — and nothing between; two corpora at each; one store
size; one page size; one machine; 10 rounds narrow and 5 wide. Two tags and one
type per item.

**Nothing above 400 arms**, which is where `MAX_QUERY_ARMS_PER_STATEMENT` splits
a query across statements and the merge is per chunk rather than per query. A
query of 800 items would merge twice and combine in Rust, and the page budget
would then be spent twice — that is the same soundness argument the adapter
already makes for chunking, but it is *argued* here and not measured.

**No falsifier for the planner's choice.** Every cell here reported
`MERGE (UNION)`; nothing establishes what SQLite does at arm counts, cardinality
ratios or versions not tried, and the shape degrades to a full sort when it
declines. An adapter shipping this needs the `EXPLAIN QUERY PLAN` assertion in
its own tests, not in this crate.

Nothing here is a patch to `happenstance-sqlite`.
