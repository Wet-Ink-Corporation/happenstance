# The append-condition strategy, measured

Every figure here is a row in [`raw/`](raw/), produced by one `./run.sh` on the
machine [`../README.md`](../README.md) names, under `journal_mode=wal`,
`synchronous=normal`, `busy_timeout_ms=5000`, SQLite 3.53.2, `--release`. The
runner prints the settings beside every row, read back off the live connection
rather than trusted from the `PRAGMA` that was issued.

All three arms passed `event_store_conformance!` first — 89 rules each,
`raw/conformance.txt`. An arm that had not would have had its figure discarded
and the failure recorded as what that arm costs.

Tag storage is held at the join table throughout, so the only thing varying is
the strategy.

## 1. The rejection path — where the arms actually differ

Four hundred rounds per arm, **round-robin inside one process**, at two log
sizes. Each timed operation is a conditional append whose guard is unbounded
over a query the log already satisfies, so it is violated every time: the
transaction rolls back, no commit happens, and the commit cost that dominates
and swamps everything else is out of the measurement.

It is a real path rather than a synthetic one. This is exactly what a DCB
command loop runs when it loses a race and has to re-decide.

Medians in microseconds, with the 10th percentile beside them.

| strategy | 1-tag @ 5,000 | 2-tag @ 5,000 | 1-tag @ 50,000 | 2-tag @ 50,000 |
| --- | --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + `EXISTS` probe | 32 (26) | 1,513 (1,425) | 311 (198) | 65,637 (49,895) |
| conditional `INSERT … WHERE NOT EXISTS` | 45 (35) | 1,532 (1,443) | 306 (206) | 65,383 (47,784) |
| **monotonic-position guard** | **23 (17)** | **972 (913)** | **213 (143)** | **42,399 (30,926)** |

Source: `raw/contention.txt`, the six `SEQUENTIAL` lines.

**The monotonic guard wins every cell, at both scales and both boundary shapes,
and the ordering held in every run.** Against the `EXISTS` probe it is 1.39x at
5,000 events and 1.46x at 50,000 on a single-tag boundary, 1.56x on a two-tag
one at both. Against the conditional insert, 1.96x and 1.44x.

**Why, and it reverses the prediction the arm was written under.** `max()`
cannot stop at the first hit, so the guard was expected to lose as the matching
set grew. It does not, for two reasons that both come from the schema:
`event_tag` is keyed `(tag, position)`, so `max(position)` over one tag's range
is a **seek to the end of that range** rather than a walk; and answering with a
*position* rather than with a boolean means the rejection path needs **no second
query** to name the conflicting event, where both other arms need one.

**A two-tag boundary costs about 200 times a single-tag one at 50,000 events**
— 42 ms to 66 ms against 0.2 ms to 0.3 ms — on every arm. That is not a strategy
finding, it is a schema finding, and it is `tag_cardinality`'s whole reason for
existing: the `GROUP BY … HAVING COUNT(DISTINCT tag)` form materialises the
union of both tags' position sets, so probing **most-selective-tag-first** is a
requirement rather than a tuning knob.

## 2. The commit path — a tie, and it is reported as one

The same rounds, timing an *accepted* conditional append against an
unconditional one of the same event; `guard = conditional − unconditional`
isolates the guard from the commit it sits inside.

| strategy | conditional | unconditional | **guard** |
| --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + probe | 288 | 221 | 67 |
| conditional insert | 304 | 225 | 79 |
| monotonic guard | 299 | 225 | 74 |

At 50,000 events the same subtraction is **not resolvable at all**: the commit
rises to about 18 ms while the guard stays in the tens of microseconds, and the
difference of two medians three orders of magnitude larger than the quantity
being measured produced 0, 11,919 and 284 µs on one run — noise, and reported as
noise rather than as a result.

**So the commit path is a tie.** Sixty-seven against seventy-nine and
seventy-four is a spread of 18% where the same three numbers moved by more than
that between runs. Nothing is decided here; §1 is where the arms separate.

## 3. Under contention — also a tie

Ten races per arm per count, round-robin. Each race opens *n*
`rusqlite::Connection`s onto one file, releases them through a
`std::sync::Barrier`, and has each attempt the same conditional append anchored
at the head as it stood before the race. Wall time per race, microseconds.

| strategy | contenders | median | range | committed | rejected | busy | failed |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + probe | 8 | 135,948 | 127,973 – 153,998 | 10 / 10 | 70 | 0 | 0 |
| conditional insert | 8 | 122,702 | 70,228 – 152,042 | 10 / 10 | 70 | 0 | 0 |
| monotonic guard | 8 | 131,636 | 62,586 – 154,251 | 10 / 10 | 70 | 0 | 0 |
| `BEGIN IMMEDIATE` + probe | **64** | 2,724,759 | 1,970,474 – 3,496,577 | 10 / 10 | 630 | 0 | 0 |
| conditional insert | **64** | 1,878,294 | 1,427,292 – 2,452,566 | 10 / 10 | 630 | 0 | 0 |
| monotonic guard | **64** | 1,437,206 | 349,134 – 2,405,583 | 10 / 10 | 630 | 0 | 0 |

Source: `raw/contention.txt`, the six `CONTEND` lines. The 64-contender table is
also in [`contention-64.md`](contention-64.md), which is where the
`CONTENDERS` question is answered.

**At 8 the three medians span 11% and every range overlaps every other.** At 64
the medians order the same way §1 does — monotonic guard fastest — but the
ranges overlap almost completely and one arm's own spread is 7x. Across earlier
runs the 64-contender ordering changed twice. It is consistent with §1 and it is
not independent evidence for it.

**What every row does say, unambiguously.** All three strategies are *correct*
under contention at both counts: exactly one contender committed in every race,
every loser learned it lost as a `ConditionViolated` rather than as an adapter
error, and `committed + rejected + busy + failed` accounts for every contender in
every round — asserted in the test rather than eyeballed here.

## 4. The harness scenarios, for the record

`event_store_benchmarks!` run verbatim at `n = 512`, `k = 64`, `N = 5000`, five
runs after a discarded warm-up, medians in microseconds. Source:
`raw/harness.txt`.

**Do not read a strategy verdict off this table**, and the reason is in the first
column: append throughput is an *unconditional* batch append in which no
strategy participates at all, and it varies by more between arms than any
conditional figure does. Each arm is timed in its own `#[test]`, so each gets its
own time slot on a shared host, and the slot noise on this machine reached 4x.

What the table is here for is the **counts**, which are exact and which are the
harness's own contribution: every contended run reported `contend attempts=64
committed=1 rejected=63 refused=0 failed=0` — the rejection mix ADR-0012 asked
this instrument for — and every replay reported 5,000 events unfiltered against
1,667 behind the tag filter.

The interleaved controls in §1 to §3 exist because of exactly this, and the
remedy is the emitter's rather than the harness's: CF-23 makes the wrapper a
parameter so a caller who needs interleaving can write one, and this crate did.

## 5. The verdict, and what it rests on

**Chosen: the monotonic-position guard**, `BEGIN IMMEDIATE` followed by one
`SELECT max(position)` over the guard's query, violated when the answer exceeds
the guard's boundary.

* It wins the **rejection path** — the only axis on which the three arms
  separated reproducibly — at both log sizes and both boundary shapes, by 1.39x
  to 1.96x.
* It ties on the commit path and under contention, where nothing separated.
* It is one query on both paths where the other two are two on the rejection
  path, which is a smaller correctness surface rather than only a faster one.
* It shares `BEGIN IMMEDIATE` with the probe arm, so it inherits that arm's
  concurrency story unchanged: the write lock is taken before the condition is
  read, so the snapshot the guard sees is the snapshot the insert writes into.

**Lost: the conditional `INSERT … SELECT … WHERE NOT EXISTS`**, at 45 µs and
1,532 µs on the 5,000-event rejection path against the guard's 23 and 972, and
306 µs and 65,383 µs at 50,000 against 213 and 42,399. It is also the only arm
whose transaction is **deferred**, so it begins as a reader and upgrades at the
`INSERT` — a distinct and more failure-prone concurrency story bought for no
measured gain.

**Lost: `BEGIN IMMEDIATE` + `EXISTS` probe**, at 32 µs and 1,513 µs against 23
and 972 at 5,000 events, and 311 µs and 65,637 µs against 213 and 42,399 at
50,000. It is the architecture brief's recommendation and it is a close second
on every axis; what it cannot avoid is the second query it needs on the
rejection path to name the conflicting position, which the guard gets for free.

**The falsifier.** This verdict rests on `max(position)` over a single tag's
range being a seek rather than a walk, which is a property of the join table's
`(tag, position)` key. Re-open it if the tag storage changes, or if a
measurement over a boundary whose matching set is a large *contiguous* range —
where `EXISTS` can stop at the first row past the boundary and `max()` still
cannot — shows the probe arm ahead.
