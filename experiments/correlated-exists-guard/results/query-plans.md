# The plans, which are the mechanism

Written by hand from [`raw/query-plans.txt`](raw/query-plans.txt), one `./run.sh`
on 2026-09-05. One 10^6-event store, boundary 500,000, two-tag guard.

`first_us` is the first execution against a cold page cache and `second_us` the
second. **Only `chain-as-shipped` ran first**, so it alone paid to warm the file
and the other five cold figures are not a like-for-like comparison. The warm
column is.

| shape | plan | cold µs | warm µs |
| --- | --- | ---: | ---: |
| `chain-as-shipped` | seek + **`LIST SUBQUERY`** | 5,905,754 | 897,411 |
| `chain-bounded-seed` | seek + **`LIST SUBQUERY`** | 677,574 | 693,426 |
| `chain-bounded-all-arms` | seek + **`LIST SUBQUERY`**, bounded | 342,973 | 345,439 |
| `grouped-adr0022` | `CO-ROUTINE` + temp b-tree | 367,140 | 330,583 |
| **`chain-exists`** | **two seeks, nothing materialised** | 108 | **43** |
| **`chain-exists-bounded-seed`** | **two seeks, bounded** | 54 | **40** |

The shipped chain's **first** execution against a cold cache is **5.9 seconds**,
inside `BEGIN IMMEDIATE`, with every other writer queued behind it. That is the
figure an application meets after a restart.

## The two plans side by side

**`chain-as-shipped`** — the uncorrelated subquery:

```
id=3  parent=0  SEARCH event_tag USING PRIMARY KEY (tag=? AND position=?)
id=9  parent=0  LIST SUBQUERY 1
id=11 parent=9  SEARCH event_tag USING PRIMARY KEY (tag=?)
```

Read it in the order SQLite runs it, not the order the SQL is written. The
chained `position IN (…)` references nothing from the enclosing row, so it is
uncorrelated and SQLite materialises it **once and in full** — `LIST SUBQUERY 1`
over every row carrying `shard:cold`, which is the whole log. It then drives from
that list, doing one `(tag, position)` seek per entry. **The seed arm is not the
outer loop; it is the probe side.** The cost is therefore proportional to the
*chained* tag's cardinality and the seed's selectivity is irrelevant — which is
[`seed-ordering.md`](seed-ordering.md)'s finding, arrived at from the other
direction.

**`chain-exists`** — the correlated subquery:

```
id=5  parent=0  SEARCH seed USING PRIMARY KEY (tag=?)
id=13 parent=0  SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)
```

Two lines, and **no `LIST SUBQUERY` anywhere**. `m0.position = seed.position`
references the outer row, so the subquery cannot be hoisted out of the loop.
`(tag, position)` is the primary key of a `WITHOUT ROWID` table, so each
evaluation is one point seek into it. The seed arm is now the outer loop.

**`chain-exists-bounded-seed`** adds the boundary to the outer arm:

```
id=5  parent=0  SEARCH seed USING PRIMARY KEY (tag=? AND position>?)
id=16 parent=0  SEARCH m0 EXISTS USING PRIMARY KEY (tag=? AND position=?)
```

`id=5` gains `AND position>?` — a seek into the interior of one contiguous range
rather than a scan of all of it — and the warm figure falls from 43 µs to 40 µs.

## Why `chain-bounded-all-arms` is not enough

Its plan still contains `LIST SUBQUERY 1`; what changes is that `id=11` gains
`AND position>?`, so the materialisation *shrinks*. At a boundary of 500,000 over
10^6 rows it shrinks by half and the figure roughly halves. At a boundary of zero
it shrinks by nothing, and neither does the figure — which is
[`guard-cost.md`](guard-cost.md) §2's 619,234 µs.

**Removing the materialisation is not the same repair as making it smaller.**

## What this page does not show

One execution per shape per column — this is the *mechanism*, and
[`guard-cost.md`](guard-cost.md) is where repetition and round-robin ordering
live. The plans are stable text; the two microsecond columns beside them are
single observations, here to show which line of the plan the cost sits on rather
than to be quoted as timings.

**These plans are for the selective corpus only.**
[`unselective-pair.md`](unselective-pair.md) measures a corpus where no tag is
selective and finds `chain-exists` costs the same there despite a seed 97x
larger — which no plan on this page explains, and which raises a limitation that
page states.
