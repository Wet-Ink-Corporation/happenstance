# The adversarial corpus: no tag selective

Written by hand from [`raw/unselective-pair.txt`](raw/unselective-pair.txt), one
`./run.sh` on 2026-09-05. 500,000 events, `all:yes` and `shard:cold` **both
matching every event**, 15 round-robin rounds per shape.

## Why this page exists

`chain-exists` wins by making the seed arm the outer loop, so its win should be
proportional to how *small* the seed is. Every cell in
[`guard-cost.md`](guard-cost.md) uses `row:r7`, one event in ninety-seven. Take
the selectivity away and the mechanism has nothing to work with:

* `chain-exists` would do `|log|` index seeks;
* `chain-as-shipped` does one sequential scan to materialise, then probes.

A sequential scan is cheaper per row than a b-tree seek, so **this is the shape
where the correlated form could plausibly lose**. A recommendation that had not
looked here would be a recommendation with an unlooked corner.

## It does not lose. It wins by more.

| scenario | `chain-as-shipped` | `chain-bounded-all-arms` | `grouped-adr0022` | **`chain-exists`** |
| --- | ---: | ---: | ---: | ---: |
| accepted, boundary at head | 1,079,137 | 112 | 401,116 | **122** |
| rejected, **no boundary** | 1,100,102 | **1,131,958** | 423,501 | **128** |

**8,600x the shipped chain**, against 1,617x–1,681x on the selective corpus at
the same size. And `chain-bounded-all-arms` — the previously-best remediation —
is *worse than shipped* with no boundary to push, which is the same conditional
collapse [`guard-cost.md`](guard-cost.md) §2 records, here at 1.13 seconds.

## The prediction was wrong, and the reason matters

`src/chain.rs`'s `exists_sql` predicted a cost of
`|seed tag| x log(|chained tag|)`. Compare across the two corpora at the same
500,000 events, unbounded:

| corpus | seed rows | `chain-exists` |
| --- | ---: | ---: |
| `row:r7` + `shard:cold` | 5,155 | 130 µs |
| `all:yes` + `shard:cold` | **500,000** | **128 µs** |

**The seed grew 97x and the cost did not move.** Whatever `chain-exists` is
doing, it is not one seek per seed row.

The likely mechanism is the `max()` the adapter wraps every guard in
(`event_store.rs:658-672`): `SELECT max(position) FROM (…)`. `(tag, position)`
is the primary key of a `WITHOUT ROWID` table, so the seed arm is already ordered
by position; SQLite can walk it **descending** and stop at the first row whose
`EXISTS` holds. On this corpus every row's `EXISTS` holds, so it stops at the
first one.

That was a hypothesis when this page was written, and
[`read-path.md`](read-path.md) has since **confirmed it**: on a shape that
enumerates rather than taking a maximum, `chain-exists` keeps only 1.7x–1.9x of
its margin on this same corpus, against 1,617x–1,681x for the guard. The early
exit was most of it.

## The limitation that follows, and it is the important one

**The win measured here belongs largely to the guard path.**
`query_sql::chunks` serves both `evaluate` and `read` (`query_sql.rs:1-11`), and
[`read-path.md`](read-path.md) measures the second: on this same unselective
corpus the rewrite is worth **1.85x** per page rather than 8,600x.

It still wins, and it is still worth having. But any figure quoted from *this*
page is a figure about a guard, and a reader sizing a projection runner should be
reading `read-path.md` instead.

## What this page does not show

One store, one size, one boundary pair, 15 rounds. Two tags with *equal*
cardinality — `most_selective_first` is a stable sort, so the tie leaves input
order deciding the seed, and there is no ordering question to ask when no tag is
selective. A corpus with two *moderately* selective tags — say 1/2 and 1/3, where
the intersection is smaller than either — is between this page and
`guard-cost.md` and is measured by neither.
