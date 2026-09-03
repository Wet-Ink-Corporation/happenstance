# The plans behind the numbers, and the paged read

Rows from [`raw/query-plans.txt`](raw/query-plans.txt), one `./run.sh`,
`--release`, on a freshly seeded 10^6-event store (183,623,680 bytes on disk,
2,000,000 `event_tag` rows, 0 unstamped, `shard:cold` = 1,000,000, `row:r7` =
10,310).

```
journal_mode=wal  synchronous=1  busy_timeout_ms=5000  sqlite=3.53.2
```

`EXECUTED first_us` / `second_us` are two consecutive executions of the same
prepared statement — the first cold against SQLite's default 2 MB page cache, the
second warm. Both are printed because the gap between them is a result.

## 1. Why the boundary in the seed arm buys nothing

`tag_cardinality` at 10^6 events: `shard:cold = 1,000,000`, `row:r7 = 10,310`.
So most-selective-first seeds on `row:r7` and chains on `shard:cold`.

**`chain-as-shipped`**, boundary 500,000:

```sql
SELECT max(position) FROM (SELECT position FROM event_tag WHERE tag = ?
  AND event_type IN (?) AND position IN (SELECT position FROM event_tag WHERE tag = ?))
```
```
id=3  parent=0  SEARCH event_tag USING PRIMARY KEY (tag=? AND position=?)
id=9  parent=0  LIST SUBQUERY 1
id=11 parent=9  SEARCH event_tag USING PRIMARY KEY (tag=?)
EXECUTED first_us=3971425 second_us=774290 rows=1
```

Read the plan in the order SQLite runs it, not in the order the SQL is written.
The chained `position IN (…)` subquery is **uncorrelated**, so SQLite materialises
it once and in full — `LIST SUBQUERY 1` over the whole of `shard:cold`, one
million rows — and then drives from that list, performing one
`(tag = 'row:r7', position = <entry>)` primary-key seek per entry. The seed arm is
not the outer loop. It is the probe side.

**`chain-bounded-seed`**, the same boundary:

```sql
… WHERE tag = ? AND event_type IN (?) AND position > ? AND position IN (SELECT position FROM event_tag WHERE tag = ?)
```
```
id=3  parent=0  SEARCH event_tag USING PRIMARY KEY (tag=? AND position=?)
id=9  parent=0  LIST SUBQUERY 1
id=11 parent=9  SEARCH event_tag USING PRIMARY KEY (tag=?)
EXECUTED first_us=447012 second_us=406180 rows=1
```

**Byte for byte the same plan.** `position > ?` lands on the arm that is already
being probed one row at a time; the million-entry materialisation it would have
to shrink is `id=11`, which the predicate cannot reach. This is the mechanism
behind [`guard-cost.md`](guard-cost.md) §2b's nine cells of nothing.

**`chain-bounded-all-arms`**:

```sql
… AND position > ? AND position IN (SELECT position FROM event_tag WHERE tag = ? AND position > ?)
```
```
id=11 parent=9  SEARCH event_tag USING PRIMARY KEY (tag=? AND position>?)
EXECUTED first_us=236988 second_us=259839 rows=1
```

One line of the plan changes — `id=11` gains `AND position>?` — and that line is
the materialisation. `(tag, position)` is the primary key of a `WITHOUT ROWID`
table, so the bounded form is a seek into the interior of one contiguous range
instead of a scan of all of it. At this boundary half the range survives, and the
figure halves.

**`grouped-adr0022`**, for comparison:

```sql
SELECT max(position) FROM (SELECT position FROM event_tag WHERE tag IN (?,?)
  AND event_type IN (?) GROUP BY position HAVING COUNT(DISTINCT tag) = 2)
```
```
id=2  parent=0  CO-ROUTINE (subquery-1)
id=8  parent=2  SEARCH event_tag USING PRIMARY KEY (tag=?)
id=28 parent=2  USE TEMP B-TREE FOR GROUP BY
EXECUTED first_us=335498 second_us=296556 rows=1
```

A co-routine, so nothing is materialised into a list at all — which is why it
beats the chain by 1.5x–1.9x in every two-tag cell despite paying for a temp
B-tree. `GROUP BY` is the optimisation barrier ADR-0022 §8 names, and it is also
what stops the planner from choosing the plan above.

### The cold/warm gap, stated because every other page is warm

| shape | `first_us` (cold) | `second_us` (warm) | gap |
| --- | ---: | ---: | ---: |
| `chain-as-shipped` | 3,971,425 | 774,290 | 5.1x |
| `chain-bounded-seed` | 447,012 | 406,180 | 1.1x |
| `chain-bounded-all-arms` | 236,988 | 259,839 | 0.9x |
| `grouped-adr0022` | 335,498 | 296,556 | 1.1x |

The shipped chain's first execution against a cold cache is **3.97 seconds**,
inside `BEGIN IMMEDIATE`, with every other writer queued behind it. Only
`chain-as-shipped` ran first in this test, so it alone paid to warm the file and
the other three rows are not a like-for-like cold comparison — but the 3.97 s is
a real figure for a real statement on a real store, and it is the one an
application meets after a restart.

## 2. The paged read carries a tautology

`fetch_page` at page 1,000 of a `Query::all` replay over the same 10^6-event
store. `query_sql` returns the bare `SELECT position FROM event` as the arm for
`Query::all`, and `fetch_page` wraps it:

```sql
SELECT position, event_type, data, metadata, tags, origin_store, origin_position, recorded_at
FROM event WHERE position IN (SELECT position FROM event)
  AND position >= ? AND position <= ? ORDER BY position ASC LIMIT ?
-- bound: resume_from=511489 ceiling=1000000 limit=512
```
```
id=4 parent=0  SEARCH event USING INTEGER PRIMARY KEY (rowid=?)
id=7 parent=0  USING ROWID SEARCH ON TABLE event FOR IN-OPERATOR
EXECUTED first_us=56770 second_us=60092 rows=512
```

The control is the same statement with the `position IN (…)` clause deleted,
returning the identical 512 rows:

```sql
… FROM event WHERE position >= ? AND position <= ? ORDER BY position ASC LIMIT ?
```
```
id=4 parent=0  SEARCH event USING INTEGER PRIMARY KEY (rowid>? AND rowid<?)
EXECUTED first_us=91 second_us=71 rows=512
```

| | with `position IN (SELECT position FROM event)` | without | ratio |
| --- | ---: | ---: | ---: |
| page 1,000, warm, 512 rows | 60,092 µs | 71 µs | **846x** |

`position IN (SELECT position FROM event)` is a tautology over the table it
selects from — every position in `event` is in `event`. It cannot remove a row.
What it does is take the plan away from the range scan the statement already
carries (`rowid>? AND rowid<?`, one seek and a walk) and give it to the
IN-operator, which is then driven per rowid.

And it is not one statement:

```
REPLAY  size=1000000  query=all  statements=1954  events=1000000  wall_ms=97233  us_per_event=97.23
```

A full `Query::all` replay of 10^6 events through the **real adapter** — not a
transcription — issues 1,954 paged statements and takes **97.2 s**, which is
49.8 ms per page for everything the adapter does: the statement, the decode, the
`spawn_blocking` hop and the stream plumbing. The isolated statement at page
1,000 costs 60.1 ms on its own and its control costs 0.071 ms, so 1,954 pages of
the control is 0.14 s. The two numbers bracket rather than subtract — 49.8 ms of
measured per-page total against a 60.1 ms isolated statement means the statement
is cheaper inside the replay than measured alone, not that the rest of the page
is free — but they bracket in only one direction: **the clause is not a
contributor to the 97.2 s, it is very nearly all of it.**

## 3. What this page does not show

* **The plans are this SQLite (3.53.2) and these statistics.** No `ANALYZE` is
  run anywhere in the adapter or here, so the planner is working from its
  built-in estimates plus the `WITHOUT ROWID` primary key. A different version, or
  a store with `sqlite_stat1` populated, may choose differently.
* **`first_us` is a single sample.** The cold column above is one execution, not a
  median; it establishes an order of magnitude, not a figure to quote to three
  digits.
* **The replay figure includes everything the adapter does per page** — decoding,
  `spawn_blocking`, stream plumbing — so the 117 s attribution is an upper bound
  derived from the per-page statement cost, not a subtraction of two measured
  replays. Removing the clause and re-measuring the replay is the experiment that
  would close that gap, and it was not run.
* **`Query::all` is the worst case for this clause and also the commonest.** A
  query with real tag items emits a genuinely selective arm, and the IN-operator
  then has something to do. Nothing here measures that case.
