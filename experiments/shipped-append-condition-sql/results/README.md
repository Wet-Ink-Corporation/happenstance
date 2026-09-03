# Results

Written by hand from [`raw/`](raw/), which is one `./run.sh` that finished
2026-09-03 06:27 local, plus one targeted replication
([`raw/seed-ordering-replication.txt`](raw/seed-ordering-replication.txt)).
Conditions — machine, OS, filesystem, toolchain, build profile, pragmas read back
off the live connection — are in [`../README.md`](../README.md#conditions), and
every row of `raw/` carries its settings string beside it.

**Conformance ran before any clock.** `raw/conformance.txt`: 356 tests — 89
conformance rules × 4 shapes — `356 passed; 0 failed`. Two of the four shapes
answer the guard question by asking SQLite a narrower one, which is the named
wrong implementation for this experiment; the suite is what earns them a place in
a table. Two further controls run inside the timing loops: all four shapes must
return the same verdict every round, and all five append arms must name the same
conflicting position every round.

| page | what is on it |
| --- | --- |
| [`guard-cost.md`](guard-cost.md) | the four shapes × five scenarios × three log sizes, interleaved, and the real adapter as a fifth arm |
| [`query-plans.md`](query-plans.md) | `EXPLAIN QUERY PLAN` for each shape, the cold/warm gap, and the paged read at 10^6 rows |
| [`seed-ordering.md`](seed-ordering.md) | what most-selective-first costs the shipped chain, and its replication |
| [`selectivity.md`](selectivity.md) | the deduplication quadratic and the unbounded parameter count, neither needing a row |

`raw/selectivity.txt` and `raw/run-log.txt` are each about 3.3 MB, and almost all
of it is one line. SQLite's `too many SQL variables` error quotes the statement it
refused, and the statement `query_sql::chunks` built for 400 items × 128 tags is
3,229,593 bytes long. The size of those two files is itself a result; they are
kept whole rather than trimmed, because a raw file someone edited is not raw.

## The question, and the answer

> What does the intersection chain the adapter actually emits cost, and does
> binding `position > ?` into the seed arm change the guard's cost class?

**It costs 1.54x–1.86x the aggregate whose measurements were used to justify it,
and binding the boundary into the seed arm changes nothing — not the number, not
the cost class, not one line of the plan.**

The headline pair, `guard_us` median at 10^6 events, two-tag guard anchored at
head, 15 interleaved rounds:

| shape | µs | against `chain-as-shipped` |
| --- | ---: | ---: |
| `chain-as-shipped` | 559,591 | — |
| `chain-bounded-seed` | 570,741 | **+2.0%** (inside both cells' p10→max bands) |
| `chain-bounded-all-arms` | **179** | 3,126x faster |
| `grouped-adr0022` | 362,385 | 1.54x faster |

`chain-bounded-all-arms` is the control that proves the null result is a real
null and not a broken instrument: a cost class *can* move here, and moving it
takes binding the boundary into the materialised subquery, which the seed arm is
not.

## Verdict per finding

| finding | verdict | the number |
| --- | --- | --- |
| **I.I-1** — the shipped multi-tag query is an intersection chain; ADR-0022's figures measured a `GROUP BY` aggregate the crate never emits | **confirms**, and sharpens | the chain loses to the aggregate in 9 of 9 two-tag cells, 1.54x–1.86x |
| **I.I-2** — the guard never pushes `position > boundary` into SQL | **qualifies** | the fact is confirmed (`raw/emitted-sql.txt`); the proposed fix is falsified — seed-arm binding is +2.0% at 10^6, and only all-arm binding moves anything |
| **I.I-3** — no measurement covers the paged read path | **confirms** | `position IN (SELECT position FROM event)` costs 846x at page 1,000; a 10^6-event `Query::all` replay through the real adapter is 97.2 s |
| **I.I-5** — tag deduplication is quadratic, once per read page and once per append guard | **confirms** | 257.7 ms against a `BTreeSet`'s 6.4 ms, 40.1x, at VT-23's own floor |
| **X.X-1** — the plan is chunked by item count, never by parameter count | **confirms** | 400 items × 128 tags = 51,600 parameters against a limit of 32,766; `prepare` fails, and `planned_statement_count` calls the plan valid |

### And one thing nobody asked for

`crates/happenstance-sqlite/src/event_store.rs:91-96` states in the crate's
public documentation that multi-tag items *"must be probed
most-selective-tag-first"*, and ADR-0022 §8 makes it a migration-1 requirement.
On the shape that actually ships, that policy costs **38x–44x** — measured twice,
on two fresh stores, six cells, one direction. It puts the larger set on the side
SQLite materialises. See [`seed-ordering.md`](seed-ordering.md).

## What none of this shows

Stated once here and again on each page, because a reader who takes only the
table will otherwise assume them away.

1. **Every claim is a ratio between shapes inside one interleaved run on one
   file.** None of these numbers is a throughput figure for `happenstance-sqlite`,
   and none is a claim about any machine but this one. `experiments/append-condition`
   records 45% run-to-run variance on this host; the replication above moved
   absolutes by 26% and ratios by 4%.
2. **Warm cache.** The first execution of the shipped chain at 10^6 rows cost
   3.97 s against the 0.56 s the interleaved median reports. Every table here is
   the favourable case.
3. **Two tags, one event type, a uniformly spread selective tag.** Three-tag
   guards add chain links, and a clustered tag changes the page count per seek.
   Neither was measured.
4. **The 50,000-event calibration did not reproduce ADR-0022 §1's 42,399 µs.**
   The same SQL shape costs 14,085 µs here — 3.0x less — for reasons of cache
   state between measurements that [`../README.md`](../README.md) sets out. The
   absolutes on these pages are warm-cache absolutes; the ratios are what carry.
5. **This is an experiment, never a gate step** (CF-34). It is not a workspace
   member, `cargo xtask ci` cannot see it, and `run.sh` must never be wired to
   anything that can turn a merge red.
