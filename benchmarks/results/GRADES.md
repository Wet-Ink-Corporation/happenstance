# What to expect

**Sections 0 to 7 are one run, one machine, 2026-09-06**, at commit `b3c8d84`
with an uncommitted `benchmarks/results/` tree. Conditions:
[`../README.md#conditions`](../README.md#conditions). Raw output:
[`raw/`](raw/). Every figure below is written by hand from it, and nothing is
carried across runs.

**Read these as orders of magnitude and as ratios between arms of one run.**
Nothing here should be quoted to a third significant figure.

> ### Four figures on this page are false at HEAD
>
> Two later runs exist — `6c7a7a8` on the same Windows host (2026-09-08) and
> `54044ac` on the Linux measurement host (2026-09-09, `raw/*-linux.*`) — and
> comparing all three shows that the **code moved under this page**, not just
> the machine. Where the Linux run agrees with `6c7a7a8` against `b3c8d84`, the
> figure below is the outlier, and a quiet host is the tiebreak.
>
> | figure | `b3c8d84` (below) | `6c7a7a8` | Linux `54044ac` | |
> | --- | ---: | ---: | ---: | --- |
> | `memory` `limit(1)` @ 10k | 2.853 ms | 291 ns | **135 ns** | §3 is wrong |
> | `memory` `backwards().limit(1)` @ 10k | 3.629 ms | 277 ns | **136 ns** | §3 is wrong |
> | `sqlite` `limit(1)` @ 10k | 93.8 µs | 539 µs | **469 µs** | 5× slower than §3 says |
> | `sqlite` idle poll | 184 µs | 682 µs | **557 µs** | 3× slower than §6 says |
>
> The allocation counts settle the first two beyond argument: §3 records
> `limit(1)` costing **50,014 heap operations**, and the Linux run measures
> **7**. `MemoryEventStore` now honours `limit`. §3 and §6 carry corrections in
> place; the rest of the page has not been re-derived and its absolutes are the
> 2026-09-06 host's.

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

## 3. The read path, and one finding this page has outlived

### ~~`MemoryEventStore` does not honour `limit`~~ — it does now

**This finding is retired, and the thing it argued for was fixed.** What stood
here said `limit(1)` at 10,000 events cost 87% of `limit(None)` in memory, from
2.85 ms against 3.28 ms, and that `memory.rs`'s `read` materialised the whole
matched set before `limit` truncated it. That was true at `b3c8d84`. It is not
true at HEAD, and the allocation counts — the column this page tells you to
prefer — say so without needing a clock:

| `memory`, `limit(1)` @ 10,000 events | heap operations |
| --- | ---: |
| at `b3c8d84`, as §3 recorded it | 50,014 |
| Linux run at `54044ac` (`raw/allocations-linux.csv`) | **7** |

Fifty thousand heap operations to seven. The timings agree: 2.853 ms →
291 ns on the same Windows host at `6c7a7a8`, → **135 ns** on Linux.

**The current read path**, Linux host `54044ac`, at 10,000 events
(`raw/collect-linux.csv`, criterion):

| Store | `head()` | `limit(1)` | `backwards().limit(1)` | `limit(None)` |
| --- | ---: | ---: | ---: | ---: |
| memory | 6.1 ns | 135 ns | 136 ns | 3.198 ms |
| sqlite | 4.72 µs | 469 µs | 468 µs | 9.956 ms |

`limit(1)` in memory is now **0.004%** of `limit(None)`, against the 87% this
section was built on. It is a reduction, which is what it always documented
itself as being.

**`SqliteEventStore` went the other way and nobody has explained it.**
`limit(1)` at 10,000 events was 93.8 µs at `b3c8d84` and is 469 µs on Linux — a
**5× regression**, reproduced on the intervening Windows run at 539 µs, so it is
not one host's noise. It is still 4.7% of the unlimited read and the shape of
the finding survives; the level does not. Nothing in this suite explains it and
it is left visible rather than smoothed.

### ES-30's `head()` still earns its keep, by less than it used to

| Store | `head()` | `backwards().limit(1)` | Ratio | at `b3c8d84` |
| --- | ---: | ---: | ---: | ---: |
| memory | **6.1 ns** | 136 ns | **22×** | ~283,000× |
| sqlite | **4.72 µs** | 468 µs | **99×** | 12.8× |

The memory ratio collapsed by four orders of magnitude for the same reason the
row above it did — `backwards().limit(1)` stopped materialising the log — and
the SQLite ratio grew by 8×. `spec/SPECIFICATION.md:4006` makes `head` a
required method on this argument, and 22× and 99× still carry it. **The
"one to five orders of magnitude" in the old heading does not.**

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

> **The 184 µs is stale.** The same arm reads 682 µs at `6c7a7a8` on this host
> and **557 µs** on Linux at `54044ac` — the two later runs agree with each
> other and not with this one, so the figure below it is the outlier and the
> real SQLite idle poll is around half a millisecond. The 31× improvement over
> the run before is not in question; its endpoint is. Redo the operator
> arithmetic that follows at 557 µs: polling once a second is about 0.06% of a
> core rather than 0.02%, and once every 10 ms about **5.6%** rather than 1.8%.
> Still two orders of magnitude better than the 57% it replaced, and no longer
> the number quoted here.

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

## 8. The same suite on Linux

`54044ac`, 2026-09-09, on the dedicated measurement host
([`../../ops/host/README.md`](../../ops/host/README.md)) — 21 unrelated
containers removed, governor and EPP pinned, scheduled maintenance masked. Raw
output is `raw/*-linux.*`; the history entry is
`history/2026-09-09-54044ac-dirty.json`.

**Read this as a second host, never as a delta against the tables above.** The
absolutes are not comparable across hosts, which is the rule this whole page is
written under. What *is* comparable is a ratio taken inside one run, and those
are given below beside their Windows counterparts so the shapes can be checked
against each other.

### The instrument, and what it took to get it running here

`run.sh` **now completes on this host**, and getting it there took three separate
fixes, none of them to the library. All three were host assumptions baked into
the harness by the machine it was written on.

1. **`the_paired_sampler_sees_a_difference_it_was_given` was mis-sized.**
   `spin(20_000)` was chosen when the timer pair cost 56 ns; here it costs
   2,924 ns. The ratio assertion was passing at 6.44× — the sampler saw the
   difference perfectly well — while `is_above_the_timer()`, a self-check on the
   control's own arms, failed. It now calibrates to the measured timer.
2. **`the_processor_clock_tells_working_from_waiting` failed 1 run in 5.** With
   the TSC marked unstable, `sched_clock` runs on a per-CPU fallback and a task
   that migrates mid-region has its runtime mis-accounted: a busy 256 ms read as
   0 ms. Measured 2/10 unpinned, 4/10 on four CPUs, **0/10 on one**. The control
   pins itself for its span.
3. **`ulimit -n` was Ubuntu's 1024.** The SQLite arms create a database per
   iteration, and SQLite reports EMFILE as `CannotOpen` — a disk error for an fd
   problem. `/tmp` had 861 GB free at the time.

None of that was visible from Host A, and none of it is visible from CI, whose
runners set the fd limit high and whose clock is in the vDSO.

### What the abstraction costs — the paired runner, on a second machine

The table below is now the **paired runner's** own answer, not a criterion
substitution. Same instrument as §1, different machine.

| Ratio | Host B, paired | Host A, paired (§1) |
| --- | ---: | ---: |
| append: sqlite ÷ raw SQL on its own schema | **1.66×** | 1.7× |
| append: sqlite ÷ hand-rolled single-table log | **4.05×** | 3.9× |
| replay: sqlite ÷ raw SQL on its own schema | **5.53×** | 6.2× |
| append: sqlite ÷ memory | **32.1×** | 120× |

Three of the four reproduce inside 10%, from a run on different hardware under a
different OS with a 52× slower clock. **That is the strongest evidence on this
page that §1 measures the library and not the machine** — and it is the check
`README.md` has been asking for since it wrote "one machine, one run per cell".

The fourth does not, and the reason is stated rather than smoothed: the
cross-adapter ratio divides by the `memory` arm, whose 5,727 ns median sits under
this host's 29 µs timer floor and is reported `TIMER-DOMINATED`. **That is the
one arm `hpet` costs**, it is the numerator of the one row that disagrees, and it
is why the row is quoted with its caveat instead of as a finding about durability
getting cheaper.

### Replay

| Log | memory | sqlite | raw floor | sqlite ÷ floor | ÷ floor, Windows §2 |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1,000 | 175 µs | 847 µs | 149 µs | **5.70×** | 5.8× |
| 10,000 | 4.01 ms | 9.94 ms | 2.34 ms | **4.25×** | 2.8× |

The **shape** reproduces — overhead falls as the log grows, which is the
correction §2 made — and the level does not. 4.25× against 2.8×.

Per-event, either side of the 512-row page: **931 / 862 / 979 ns** at 256 /
2,048 / 8,192 events. A 1.14× spread across a 32× range, matching the 1.16×
§2 recorded. **Flat in page depth**, independently on a second machine — and at
roughly a third of the Windows per-event cost, which is a host difference and
not a claim about the code.

### The DCB guard

| Guard | 1,000 events | 10,000 events | Growth |
| --- | ---: | ---: | --- |
| single tag, selects one | 19.6 µs | 20.0 µs | **flat** |
| single tag, selects all | 19.8 µs | 19.8 µs | **flat** |
| two-tag intersection | 33.0 µs | 33.3 µs | **flat** |

Two-tag is **1.67×** a single-tag guard and flat in log length, against §4's
1.3× and flat. The correlated `EXISTS` rewrite holds on a second machine, which
is the claim §4 actually makes. `MemoryEventStore` is flat at 14–17 ns.

**Contention**, at *k* = 1, 8, 64 on SQLite: 9.26 ms, 10.27 ms, 17.47 ms.

### The typed layer

| Prior events | memory | sqlite |
| --- | ---: | ---: |
| 0 | 3.93 µs | 1.685 ms |
| 10 | 16.1 µs | 1.651 ms |
| 100 | 122 µs | 1.912 ms |
| 1,000 | 1.168 ms | 3.964 ms |

Linear at ~1.16 µs per prior event in memory, against §5's 1.4 µs. **Codec
choice**: postcard 42.5 µs, json 121 µs, cbor 226 µs at a 100-event boundary —
json 2.8× postcard, cbor 5.3×, against §5's 2.7× and 3.6×. **A lost attempt**:
122 µs uncontended against 244 µs when it loses once — **2.0×**, against §5's
1.9×.

### The projection runner

| Chunk | memory models | sqlite models |
| --- | ---: | ---: |
| 1 | 10.75 ms (930 K/s) | 565 ms (17.7 K/s) |
| 100 | 9.84 ms (1,016 K/s) | 58.5 ms (171 K/s) |
| 5,000 | 9.57 ms (1,044 K/s) | 51.4 ms (195 K/s) |

Chunk size is worth **11.0×** on SQLite here and 1.12× in memory, against §6's
17.3× and nothing. The gradient is the finding and it survives; its steepness
does not transfer between hosts.

**Idle poll**: 75.4 µs in memory, **557 µs** on SQLite — the figure that
supersedes §6's 184 µs, and the one to do operator arithmetic with.

### What did not change, because it cannot

`raw/allocations.csv` and `raw/allocations-linux.csv` are **byte-identical over
every data row** — `diff` returns nothing. Two operating systems, two target
triples (`x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu`), two commits,
and not one counter differs. `Event::clone()` at the 64-tag floor is 66 heap
operations owned and 1 interned on both.

That is the strongest single result on this page, and it is not about speed. It
is the control that makes every timing above legible: when two hosts whose
wall-clock medians differ by 2–3× agree to the digit on the allocation counts,
the counts are measuring the program and the timings are measuring the program
*and the machine*. It is why §7 says to quote the counts.

## What none of this shows

[`../README.md#what-none-of-this-shows`](../README.md#what-none-of-this-shows),
in full; all eight points apply to every table above. The three that bite
hardest here:

- **No comparison against a peer library.** Declined deliberately, not deferred.
- **The floors are cheaper than a correct store.** §1's ratios are upper bounds,
  and are not a claim that a hand-rolled store would be correct.
- **One run per cell.** Where two arms differ by less than about 20%, this suite
  cannot tell them apart.
