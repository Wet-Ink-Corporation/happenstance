# The guard, four shapes, three log sizes

Every figure on this page is a row of
[`raw/guard-cost.txt`](raw/guard-cost.txt), from the single `./run.sh` that
finished 2026-09-03 06:27 local. The conditions line is printed beside every row
in the raw file and is read back off the live connection by
`happenstance_sqlite::connection::ConnectionSettings::read_back`, not trusted
from the `PRAGMA` that issued it:

```
journal_mode=wal  synchronous=1  busy_timeout_ms=5000  sqlite=3.53.2  max_arms=400
```

Machine, OS, filesystem, toolchain and build profile are in
[`../README.md`](../README.md#conditions). Everything here is `--release`.

## Conformance ran first, and it is the control

`raw/conformance.txt`:

```
running 356 tests
test result: ok. 356 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.92s
```

89 conformance rules × 4 shapes, green, **before any timer started**. That is the
whole reason the two bounded shapes are allowed on this page at all: they answer
the guard question by asking SQLite a narrower one, which is exactly the shape of
a guard that has stopped noticing conflicts, and a shape that is fast and wrong
wins every table below. Two further controls run inside the timing loop itself:
every round asserts all four shapes returned the **same verdict**
(`guard_cost.rs:319-328`), and every `APPEND` round asserts all five arms named
the **same conflicting position** (`guard_cost.rs:367-374`).

## How to read a cell

The `GUARD` instrument times `BEGIN IMMEDIATE` → `Selectivity::read_for` →
`SELECT max(position) FROM (…)` → `ROLLBACK` — `append_locked` minus the write.
`guard_us` below is that median minus the median of a bare
`BEGIN IMMEDIATE; ROLLBACK` measured in the same rounds, so it is the probe and
not a transaction with a probe somewhere in it. The four shapes are interleaved
round-robin on **one** database file, and which shape goes first rotates every
round.

## 1. The single-tag path is not where any of this lives

`guard_us` median, µs. All four shapes, both single-tag scenarios, every size:

| scenario | size | chain-as-shipped | chain-bounded-seed | chain-bounded-all-arms | grouped-adr0022 |
| --- | ---: | ---: | ---: | ---: | ---: |
| `accepted-1tag-at-head` | 50,000 | 6 | 6 | 6 | 6 |
| | 500,000 | 13 | 16 | 16 | 15 |
| | 1,000,000 | 10 | 10 | 10 | 10 |
| `rejected-1tag-unbounded` | 50,000 | 11 | 12 | 11 | 10 |
| | 500,000 | 4 | 5 | 5 | 4 |
| | 1,000,000 | 6 | 7 | 7 | 6 |

Single-digit microseconds at 10^6 events, and flat in log size, because a
single-tag item is a `tag = ?` seek into the primary key of a `WITHOUT ROWID`
table. Nothing that follows is about this row. Everything that follows is about
the second tag.

## 2. The two-tag guard — `guard_us` median, µs

| scenario | size | chain-as-shipped | chain-bounded-seed | chain-bounded-all-arms | grouped-adr0022 |
| --- | ---: | ---: | ---: | ---: | ---: |
| `accepted-2tag-at-head` | 50,000 | 29,845 | 32,483 | **124** | 19,178 |
| | 500,000 | 296,229 | 282,999 | **143** | 185,224 |
| | 1,000,000 | 559,591 | 570,741 | **179** | 362,385 |
| `rejected-2tag-unbounded` (boundary 0) | 50,000 | 25,204 | 24,405 | 25,366 | 14,085 |
| | 500,000 | 373,858 | 383,439 | 374,662 | 231,501 |
| | 1,000,000 | 545,830 | 554,497 | 534,230 | 349,805 |
| `rejected-2tag-midlog` | 50,000 (b=25,000) | 31,487 | 29,218 | 15,442 | 18,015 |
| | 500,000 (b=250,000) | 338,431 | 328,717 | 169,587 | 214,040 |
| | 1,000,000 (b=500,000) | 593,093 | 549,161 | 254,482 | 319,306 |

Rounds per cell: 120 at 50,000, 30 at 500,000, 15 at 1,000,000. The raw file
carries `us_p10` and `us_max` for every one of them.

### 2a. The shipped chain against the aggregate whose numbers justified it

`chain-as-shipped ÷ grouped-adr0022`, from the table above:

| scenario | 50,000 | 500,000 | 1,000,000 |
| --- | ---: | ---: | ---: |
| `accepted-2tag-at-head` | 1.56x | 1.60x | 1.54x |
| `rejected-2tag-unbounded` | 1.79x | 1.61x | 1.56x |
| `rejected-2tag-midlog` | 1.75x | 1.58x | 1.86x |

Nine cells, all the same direction, 1.54x to 1.86x. **The shipped intersection
chain is slower than ADR-0022 §8's `GROUP BY position HAVING COUNT(DISTINCT tag)
= n` at every size and in every two-tag scenario measured.**
`crates/happenstance-sqlite/src/query_sql.rs:34-54` justifies the chain by
reasoning from the aggregate's own 1.97x single-tag measurement — the chain was
never measured against it, and when it is, it loses.

### 2b. The boundary in the seed arm — the question this experiment was built for

`chain-bounded-seed` relative to `chain-as-shipped`:

| scenario | 50,000 | 500,000 | 1,000,000 |
| --- | ---: | ---: | ---: |
| `accepted-2tag-at-head` | +8.8% | −4.5% | **+2.0%** |
| `rejected-2tag-unbounded` | −3.2% | +2.6% | +1.6% |
| `rejected-2tag-midlog` | −7.2% | −2.9% | −7.4% |

Nine cells, no consistent sign, every one inside its own cell's run-to-run band.
At 10^6 events with the guard anchored at head, `chain-as-shipped` has
`us_p10=485,321` and `us_max=4,685,317`; `chain-bounded-seed` has
`us_p10=516,599`, `us_max=741,044`. A 2.0% difference in medians sits inside both.

**Binding `position > ?` into the seed arm does not change the guard's cost, and
does not change its cost class.** [`query-plans.md`](query-plans.md) says why,
and the reason is the opposite of what the finding predicted: the seed is not the
driving table.

### 2c. The boundary in *every* arm — the control that shows a cost class can move

`chain-bounded-all-arms` relative to `chain-as-shipped`:

| scenario | 50,000 | 500,000 | 1,000,000 | what the mechanism predicts |
| --- | ---: | ---: | ---: | --- |
| `accepted-2tag-at-head` | 241x faster | 2,071x | **3,126x** | boundary at head — the materialisation is empty |
| `rejected-2tag-midlog` | 2.04x | 2.00x | 2.33x | boundary at half — half the materialisation survives |
| `rejected-2tag-unbounded` | 0.99x | 1.00x | 1.02x | boundary at 0 — nothing is discarded, nothing is saved |

Three predictions, three sizes, nine agreements. The 2.0x at midlog and the 1.0x
at boundary 0 are what make the 3,126x credible rather than a suspicious outlier:
the saving is exactly proportional to the fraction of the chained tag's history
the predicate can discard, which is what "the materialisation is what shrinks"
means as an arithmetic claim.

This shape exists nowhere in the repository. It is the only one of the four that
makes `query_sql.rs:48-54`'s *"keeps the boundary pushable"* a true sentence.

## 3. The whole call, and the fifth arm

The `APPEND` instrument drives `EventStore::append` on the rejection path,
including the real `SqliteEventStore` on the same seeded file. Median µs:

| scenario | size | chain | bounded-seed | bounded-all-arms | grouped | **real-adapter** |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `rejected-1tag-unbounded` | 50,000 | 10 | 10 | 10 | 10 | **10** |
| | 500,000 | 9 | 10 | 10 | 9 | **9** |
| | 1,000,000 | 7 | 7 | 7 | 7 | **7** |
| `rejected-2tag-unbounded` | 50,000 | 20,685 | 20,946 | 20,919 | 11,773 | **20,542** |
| | 500,000 | 275,838 | 286,286 | 282,018 | 196,405 | **294,027** |
| | 1,000,000 | 472,063 | 453,628 | 438,462 | 285,622 | **453,243** |
| `rejected-2tag-midlog` | 50,000 | 21,878 | 21,671 | 10,570 | 12,279 | **20,832** |
| | 500,000 | 2,317,702 | 2,200,320 | 1,050,840 | 184,125 | **2,275,476** |
| | 1,000,000 | 472,749 | 450,388 | 221,840 | 274,188 | **464,045** |

The `real-adapter` column is the point of the table: it tracks `chain` to within
7% in every cell where either is above a millisecond. The transcription is not
merely byte-identical in SQL ([`raw/emitted-sql.txt`](raw/emitted-sql.txt)) — it
costs the same end to end.

**The 500,000-event `rejected-2tag-midlog` row is anomalous and no claim on this
page rests on it.** Four of its five arms report medians five to ten times the
`GUARD` instrument's figure for the same cell (338 ms), with `us_p10` an order of
magnitude below the median — 242,252 against 2,317,702 for `chain` — which is a
bimodal sample, not a slower operation. `grouped` in the same interleaved rounds
is unaffected (184,125, p10 146,291, max 298,400), which rules out a simple
ambient-load window and leaves it unexplained. It did not reproduce at either of
the other two sizes. It is left in the table rather than deleted because deleting
an inconvenient row is how a table stops being evidence.

## 4. What this page does not show

* **Nothing here is a throughput number for `happenstance-sqlite`.** Every claim
  is a ratio between shapes taken inside one interleaved run on one file. The
  absolute microseconds are this host, this hour, this page cache.
* **Two tags, one event type.** A guard crossing three or more tags adds chain
  links nothing here measured, and the chain's cost is a function of link count.
* **Warm cache.** Rounds are back-to-back on a 184 MB database against SQLite's
  default 2 MB page cache; the first execution of the shipped chain at 10^6 rows
  cost 3.97 s ([`query-plans.md`](query-plans.md)) against the 0.56 s the
  interleaved median reports. A cold store is worse than every number above.
* **The 50,000-event calibration did not reproduce ADR-0022 §1's 42,399 µs.**
  `grouped-adr0022` — the same SQL shape §1 measured — costs 14,085 µs here on
  the matching cell, 3.0x less. See [`../README.md`](../README.md) for why the
  two are different operating conditions rather than a disagreement about SQL,
  and treat this page's absolutes accordingly.
