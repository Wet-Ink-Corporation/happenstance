# What to expect

**One run, one machine, 2026-09-06**, at commit `b3c8d84` with an uncommitted
`benchmarks/results/` tree. Conditions:
[`../README.md#conditions`](../README.md#conditions). Raw output:
[`raw/`](raw/). Every figure below is written by hand from it, and nothing is
carried across runs.

**Read these as orders of magnitude and as ratios between arms of one run.**
Nothing here should be quoted to a third significant figure.

## What changed since the 2026-09-05 run, and how to read the difference

The previous version of this page was produced at `4a7ca16`, before three
changes to `happenstance-sqlite`'s query SQL: the append-condition chain became
a correlated `EXISTS` carrying the guard's boundary, and the read path stopped
wrapping a matched set in `WHERE position IN (…)` and now merges windowed arms
with the page budget on the compound. `experiments/correlated-exists-guard/`
measured each before it shipped.

**The two runs' absolutes are not comparable, and the arms this change cannot
reach say so.** `MemoryEventStore`'s append went 3.2 M to 1.9 M events/s and the
single-tag SQLite guard went 9.1 µs to 18.2 µs — neither path contains a line
this change touched, so **this host ran about 1.7× slower than the one that
produced the previous page**. Every absolute below is under that discount, and
every improvement is therefore understated.

What *is* comparable is a ratio taken inside one run, and those are quoted
side by side throughout. The allocation counts in §7 are the control that makes
this legible: they are identical to the digit across both runs while every
timing moved.

---

## 0. The read path, against the previous run

Same-run ratios, so the host difference above cannot flatter them.

| Ratio, measured within its own run | 2026-09-05 | **2026-09-06** |
| --- | ---: | ---: |
| Replay 10,000 events: sqlite ÷ raw SQL floor | 11.1× | **2.8×** |
| Replay 1,000 events: sqlite ÷ raw SQL floor | 6.6× | **5.8×** |
| Two-tag guard ÷ single-tag guard, 10,000 events | 304× | **1.3×** |
| Two-tag guard: 10,000 events ÷ 1,000 events | 11.7× | **0.8× (flat)** |
| Projection idle poll: sqlite ÷ memory | 59.5× | **2.2×** |
| `limit(None)` ÷ `limit(1)`, sqlite, 10,000 events | 610× | 203× |

Two claims the previous page made are now **false**, and are corrected in place
below: that replay overhead grows with log length, and that the two-tag guard is
linear-or-worse in it. Both were true of the SQL that shipped when they were
written.

---

## 1. What the abstraction costs over raw SQL

From [`raw/overhead.csv`](raw/overhead.csv) — the interleaved paired runner, not
criterion, because a ratio taken from sequential arms on this host is worth
nothing (see
[`../README.md`](../README.md#absolutes-from-criterion-ratios-from-the-paired-runner)).

| Operation | happenstance-sqlite over… | Ratio | 2026-09-05 |
| --- | --- | ---: | ---: |
| Append, 8 events × 3 tags × 256 B | raw SQL on **its own schema** | **1.7×** | 1.7× |
| Append, same | a **hand-rolled** single-table event log | **3.9×** | 4.5× |
| Replay 2,000 events | raw SQL on its own schema | **6.2×** | 7.1× |

Across four separate runs the append ratio came out at 1.66, 1.68, 1.71 and
1.73. The *ratio* is the stable thing; the absolutes underneath it are not — and
the append ratio not moving is the expected result, because nothing in the write
path changed.

The 2,000-event replay improves least of any read on this page, and the reason
is worth stating: 2,000 events is four pages, and what the read-path change
removed was a cost that grew with **page depth**. At four pages there is almost
none of it to remove, and what is left — per-event row materialisation, the
connection mutex, one `spawn_blocking` hop per page — the change does not touch.
§2's ten-thousand-event figure is where the difference lives.

**How to read the two floors.** The first isolates the cost of the adapter's
Rust — query planning, tag dedup, the connection mutex, the paged read, the
`spawn_blocking` hop — from the schema those decisions were made for. The second
prices the library *and its schema* together, against what an engineer writes on
day one. Both floors are deliberately cheaper than a correct store (no event id,
no recorded time, no `tag_cardinality`, no append condition), which makes each
ratio an **upper** bound.

**Cross-adapter:** `SqliteEventStore` is **120×** `MemoryEventStore` on the same
append (111× on the previous run). That is the price of durability, not the price
of the abstraction, and it is a write-path figure that this round of changes had
no way to move.

---

## 2. Throughput

From [`raw/store_append.txt`](raw/store_append.txt), owned regime (the one a
caller is in), 1 KiB payloads, 3 tags.

| Store | 1 event/batch | 128 events/batch (VT-24 floor) |
| --- | --- | --- |
| `MemoryEventStore` | ~1.9 M events/s | ~2.7 M events/s |
| `SqliteEventStore` | ~4.8 K events/s | ~19 K events/s |

Batching is worth about **4×** on SQLite and 1.4× in memory. What SQLite
amortises per batch is one `BEGIN IMMEDIATE`, one commit and one `fsync` under
`synchronous = NORMAL`.

Every number in this table is lower than the previous run's, and none of it is
this change: the write path is untouched, and the memory arm — which shares no
code with anything that moved — fell by the same factor. See *"how to read the
difference"* above. **This is the table to distrust across runs and the ratio in
the row below it to trust.**

**Replay**, unfiltered, from [`raw/store_replay.txt`](raw/store_replay.txt):

| Log | memory | sqlite | raw floor | sqlite ÷ floor | ÷ floor, 2026-09-05 |
| --- | --- | --- | --- | ---: | ---: |
| 1,000 | 190 µs | 884 µs | 152 µs | 5.8× | 6.6× |
| 10,000 | 3.61 ms | 12.5 ms | 4.43 ms | **2.8×** | **11.1×** |

**The previous version of this page said the replay overhead grows with log
length. It no longer does — it falls.** 5.8× at a thousand events against 2.8×
at ten thousand, where the same two cells were 6.6× and 11.1× before the read
path stopped materialising a matched set per page.

`replay/page-hops` says it per event, either side of the 512-row page:

| Page count | ns/event, this run | 2026-09-05 |
| --- | ---: | ---: |
| 256 events (under one page) | 2,402 | 1,900 |
| 2,048 events (four pages) | 2,316 | 2,200 |
| 8,192 events (sixteen pages) | **2,689** | **4,400** |

A 1.16× spread across a 32× range of log lengths, where it was 2.3×. The
per-event cost is now **flat in page depth**, which is what a projection runner
catching up actually meets. It is not *free* — 2.3–2.7 µs an event is the row
materialisation, the decode and the per-hop `spawn_blocking` — but it no longer
compounds. `PAGE_SIZE = 512` still describes itself as *"a placeholder until it
is measured"*, and the case for measuring it is now much weaker than it was.

---

## 3. Two places the documentation is wrong

### `MemoryEventStore` does not honour `limit`; `SqliteEventStore` does

`limit(1)` against `limit(None)` over `Query::all`, at 10,000 events:

| Store | `limit(1)` | `limit(None)` | Ratio |
| --- | --- | --- | --- |
| memory | 2.85 ms | 3.28 ms | **0.87×** |
| sqlite | 93.8 µs | 19.0 ms | 0.0049× |

Asking for one event out of ten thousand costs **87%** of asking for all of them
in memory, and **0.5%** on SQLite. The memory ratio moved from 0.72× to 0.87×
and the conclusion does not depend on which: both say `limit` is not a
reduction there. The SQLite ratio moved because its denominator — the full
replay — got 2.2× cheaper, not because `limit(1)` got worse. The allocation counts
([`raw/allocations.csv`](raw/allocations.csv)) are exact and say it more
plainly:

| Read | heap operations | peak live bytes |
| --- | --- | --- |
| `limit(None)` | 50,026 | 5,897,482 |
| `limit(1)` | **50,014** | **3,538,186** |
| `head()` | **0** | **0** |

`crates/happenstance-core/src/memory.rs:30-31` prices a snapshot as *"bumps
refcounts rather than copying data"*, and `read` materialises the whole matched
set — cloning every event — before `limit` truncates it (`:296-336`). The store
documents itself as not built for scale and nobody is entitled to be surprised
that it is slow. What these figures say is narrower: its snapshot is priced as
*cheap* and its `limit` as a *reduction*, and neither is true.

### ES-30's `head()` earns its keep, by one to five orders of magnitude

`head()` against the composed `backwards().limit(1)`, at 10,000 events:

| Store | `head()` | `backwards().limit(1)` | Ratio | 2026-09-05 |
| --- | --- | --- | ---: | ---: |
| memory | **12.8 ns**, zero allocations | 3.63 ms | ~283,000× | ~209,000× |
| sqlite | **5.2 µs** | 66.2 µs | 12.8× | 16× |

`spec/SPECIFICATION.md:4006` makes `head` a required method on exactly this
argument. It is now a measured decision rather than a plausible one.

---

## 4. The DCB guard, and the shape that used not to scale

The cost of evaluating an append condition against a seeded log and refusing —
the path a losing writer pays on every retry.
[`raw/store_conditional.txt`](raw/store_conditional.txt), `SqliteEventStore`.

| Guard | 1,000 events | 10,000 events | Growth |
| --- | --- | --- | --- |
| single tag, selects one | 18.2 µs | 25.4 µs | **flat** |
| single tag, selects all | 22.0 µs | 22.6 µs | **flat** |
| **two-tag intersection** | **41.6 µs** | **34.1 µs** | **flat** |

**This section's heading used to end "the one shape that does not scale".** The
previous version of this page read *"the two-tag intersection is linear-or-worse
and 305× more expensive at 10,000 events"*, at 270 µs and 3.16 ms. It is now
**1.3× a single-tag guard** and flat in log length.

That is the correlated `EXISTS` rewrite arriving through the port. The
experiment measured the same change at statement level and at sizes this suite
cannot reach — 579,883 µs to 33 µs for a two-tag guard at 10^6 events
(`experiments/correlated-exists-guard/results/guard-cost.md`) — and this table
is the independent confirmation that it survives the trip through
`SqliteEventStore`, `BEGIN IMMEDIATE` and the connection mutex.

Finding I-1 and the projection at
`references/evaluation/review-pre-publication-2026-09-03.md:2540` — **296 ms at
500,000 events and 560 ms at 10^6** — described the shape that shipped until
2026-09-05. They are no longer the curve this adapter is on.

The single-tag rows roughly doubled against the previous run. Nothing in that
path changed, the experiment's own single-tag control found all six candidate
shapes indistinguishable at 8–10 µs, and the memory arm moved by the same factor
— so this is the host, not the guard. It is left visible rather than smoothed.

`MemoryEventStore` is flat at ~19 ns for all three, because its condition check
short-circuits on the first match.

**Contention** is clean on both arms: at k = 1, 8 and 64 the rejection mix is
always exactly one winner, k−1 `ConditionViolated` rejections, and **zero**
failures. Both stores serialise their writers, as their designs say they do.

---

## 5. The typed layer

### One command is a full replay plus one decode per prior event

[`raw/typed_command.txt`](raw/typed_command.txt), Json. Every iteration starts
from a restored boundary — see that file's module docs for why that took work.

| Prior events | memory | sqlite | sqlite, 2026-09-05 |
| --- | --- | --- | ---: |
| 0 | 1.37 µs | 1.75 ms | 1.89 ms |
| 10 | 16.7 µs | 1.84 ms | 1.97 ms |
| 100 | 123 µs | 2.35 ms | 2.63 ms |
| 1,000 | 1.53 ms | 5.27 ms | 6.17 ms |

Linear at roughly **1.4 µs per prior event** on the memory arm — the decode and
fold cost, with no I/O under it. DCB has no aggregate to snapshot into, so this
is what an unbounded consistency boundary costs, and it is the number to size a
domain against.

### Codec choice is worth 2–3× on a warm boundary

At a 100-event boundary: **postcard 46 µs, json 126 µs, cbor 166 µs** — json is
2.7× postcard and cbor is 3.6×.

The encoded sizes matter as much as the times, because bytes become rows, pages
and write-ahead log. At a 64 KiB payload: **postcard 65,544 B, cbor 131,119 B,
json 196,667 B** — json is **3.0×** postcard, which is ADR-0016's base64
encoding of `Bytes` arriving on the wire.

`Json` is the default and the one a first program gets. It is the right default,
and it is the most expensive of the three on both axes.

### A lost attempt costs a whole command

`command/retry`: **155 µs** uncontended, **290 µs** when the command loses once
and wins on its second attempt — **1.9×**. A rejected attempt has already paid
for the full replay and every decode before the condition refuses it.

---

## 6. The projection runner

[`raw/projection_runner.txt`](raw/projection_runner.txt), catching up over
10,000 events from `Checkpoint::NeverRun`.

| Chunk | memory models | sqlite models | sqlite, 2026-09-05 |
| --- | --- | --- | ---: |
| 1 | 13.5 ms (742 K/s) | 1.00 s (10.0 K/s) | 980 ms |
| 100 | 11.1 ms (898 K/s) | 86.4 ms (116 K/s) | 145 ms |
| 5,000 | 11.2 ms (891 K/s) | **57.9 ms (173 K/s)** | 132 ms |

**Chunk size is now worth 17.3× on SQLite and nothing at all in memory**, up
from 7.4×. Chunk 1 did not move — it is dominated by per-hop cost, one
`spawn_blocking` and one transaction per event, and no query change reaches it —
while chunks of 100 and 5,000 got 1.7× and 2.3× faster. So the *gradient*
steepened: Wattline's concern (`references/scenarios/README.md:665-670`,
*"chunk size is the runner's business and every adapter's default is wrong for
this"*) is now a stronger claim than when it was written, not a weaker one.

**Idle poll** — one `run_projection` that finds nothing new: **82.7 µs** in
memory, **184 µs** on SQLite, against **5.68 ms** on the previous run. **A 31×
improvement, and the largest single change on this page.**

An idle poll reads from the checkpoint forward and finds nothing. It used to
emit `WHERE position IN (SELECT position FROM event) AND position >= ?`, whose
`IN`-driven search cannot take the range bound, so it re-walked the whole log to
discover there was nothing after the end of it. That is exactly the mechanism
`experiments/correlated-exists-guard/results/all-query-wrapper.md` isolates.

The operator arithmetic changes with it: polling once a second was about 0.6% of
a core and is now about 0.02%; once every 10 ms was about **57%** and is now
about **1.8%**. `experiments/polling-cost/`'s fan-out amplification of **32.00 at
32 views** still multiplies whatever the per-poll cost is — it found no economy
of scale anywhere in the range it swept — but the thing being multiplied is two
orders of magnitude smaller.

---

## 7. Allocation, exactly

[`raw/allocations.csv`](raw/allocations.csv). These are reproducible to the
digit; the timings above are not.

**`Event::clone()`, steady state** (the second clone — the first pays one more
operation for `Bytes`' shared header):

| Tags | owned | interned (control) |
| --- | --- | --- |
| 0 | 1 op | 0 ops |
| 1 | 3 ops | 1 op |
| 8 | 10 ops | 1 op |
| **64** (VT-22 floor) | **66 ops, 2,129 B** | **1 op, 1,536 B** |

`t + 2` in the owned regime and **flat at 1** in the interned one. This
reproduces `review-pre-publication-2026-09-03.md:2724` exactly, and it is why
every table in this suite carries a regime column: the interned arm is the one a
benchmark author writes without choosing to, and it would report the clone as
66× cheaper than a caller actually pays.

**This does not lift ES-17's marker.**
`.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` records what
that falsifier asks for — two builds of one adapter differing only in `append`'s
ownership, on one harness — and this suite is not that. What it supplies is the
numerator, and the warning that goes with it.

---

## What none of this shows

[`../README.md#what-none-of-this-shows`](../README.md#what-none-of-this-shows),
in full; all eight points apply to every table above. The three that bite
hardest here:

- **No comparison against a peer library.** Declined deliberately, not deferred.
- **The floors are cheaper than a correct store.** §1's ratios are upper bounds,
  and are not a claim that a hand-rolled store would be correct.
- **One run per cell.** Where two arms differ by less than about 20%, this suite
  cannot tell them apart.
