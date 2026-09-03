# Does `tag_cardinality`'s most-selective-first ordering buy the shipped chain anything?

It costs it about **40x**.

## The claim under test

`crates/happenstance-sqlite/src/event_store.rs:91-96`, in the crate's own
**public** module documentation:

> **`tag_cardinality` is a requirement rather than a convenience.** Multi-tag
> items must be probed most-selective-tag-first…

ADR-0022 §8 makes it a migration-1 requirement
(`references/adr/0022-append-condition-strategy.md:374-379`). Every sentence
stating it was written about the `GROUP BY` form, where the ordering question
does not arise — `tag IN (?,?)` is order-independent — and inherited by the
intersection chain without being re-measured on it.

## The instrument

One 500,000-event store, one connection, one SQL builder, one statement text.
`Selectivity::inverted()` is the entire difference: it flips the sign of the
cardinality counts so `most_selective_first` orders the two tags the other way.
The statement text is identical in both arms; only the bound parameters swap.

```
statement (identical for both orderings):
  SELECT max(position) FROM (SELECT position FROM event_tag WHERE tag = ?
    AND event_type IN (?) AND position IN (SELECT position FROM event_tag WHERE tag = ?))
most-selective-first bound (what ships): ["row:r7", "Seeded", "shard:cold"]
least-selective-first bound (the control): ["shard:cold", "Seeded", "row:r7"]
```

`CARDINALITY shard_cold=Some(500000) row_r7=Some(5155)` — so `row:r7` is 97x the
more selective tag and is what the shipped ordering puts in the seed arm.

Which ordering runs first **alternates every round**, so neither median is about
going second. Every round asserts the two orderings returned the identical
position before either time is recorded — ordering cannot change a conjunction's
result, and the assertion is what proves the two arms are the same question.

## The result, and its replication

25 rounds per arm, `--release`, median µs.
`journal_mode=wal synchronous=1 busy_timeout_ms=5000 sqlite=3.53.2`.

[`raw/seed-ordering.txt`](raw/seed-ordering.txt) — the `./run.sh` run:

| scenario | boundary | most-selective-first (**ships**) | least-selective-first (control) | penalty |
| --- | ---: | ---: | ---: | ---: |
| `accepted-2tag-at-head` | 500,000 | 511,054 | 11,839 | **43.2x** |
| `rejected-2tag-unbounded` | 0 | 553,328 | 13,340 | **41.5x** |
| `rejected-2tag-midlog` | 250,000 | 634,239 | 14,399 | **44.0x** |

[`raw/seed-ordering-replication.txt`](raw/seed-ordering-replication.txt) — an
independent re-run, fresh store, same host, later:

| scenario | boundary | most-selective-first | least-selective-first | penalty |
| --- | ---: | ---: | ---: | ---: |
| `accepted-2tag-at-head` | 500,000 | 646,124 | 16,352 | **39.5x** |
| `rejected-2tag-unbounded` | 0 | 654,719 | 15,764 | **41.5x** |
| `rejected-2tag-midlog` | 250,000 | 565,698 | 14,859 | **38.1x** |

Six measurements, two runs, one direction: **38x to 44x.** The absolute figures
move by up to 26% between runs — this host's documented behaviour — and the ratio
does not.

## Why, and it is the same mechanism as everything else on this experiment

[`query-plans.md`](query-plans.md) §1: SQLite materialises the **chained**
subquery as an uncorrelated `LIST SUBQUERY` and drives from it, seeking
`(tag = <seed>, position = <entry>)` once per entry.

So the cost is the size of the **chained** tag's history, not the seed's:

| ordering | seed arm | chained (materialised) | list entries | measured |
| --- | --- | --- | ---: | ---: |
| most-selective-first (ships) | `row:r7` (5,155) | `shard:cold` | 500,000 | ~511–646 ms |
| least-selective-first | `shard:cold` (500,000) | `row:r7` | 5,155 | ~12–16 ms |

500,000 ÷ 5,155 = 97x of predicted work against 38–44x of measured time. The
policy the documentation states as a requirement puts the **larger** set on the
side the planner materialises, every time, by construction.

This also explains [`guard-cost.md`](guard-cost.md) §2's whole two-tag column
without any further assumption: the shipped chain's 296 ms at 500,000 events and
560 ms at 10^6 are half a million and a million primary-key seeks respectively.

## What this page does not show

* **Two tags.** With three or more, "least selective first" is not obviously the
  right policy either — the plan has more than one materialisation to choose
  between and the arithmetic above no longer has a single term. This page
  falsifies the stated rule for the two-tag case; it does not supply the
  replacement rule.
* **Nothing here says the ordering is wrong for the `GROUP BY` form**, which is
  what the sentence was originally measured on and where it is a no-op.
* **`Selectivity::read_for` is outside the timed region here** (both arms build
  their SQL before the clock starts), unlike `guard-cost.md`, where it is inside
  because it is inside the transaction in the adapter. That is why this page's
  most-selective-first figures are not directly comparable with that page's, and
  the two differ by up to 1.7x.
* **The seed is uniform.** `row:r7` matches every 97th position, spread evenly.
  A clustered selective tag — all its events in one region of the log — would
  change how many pages each seek touches, and nothing here measures that.
