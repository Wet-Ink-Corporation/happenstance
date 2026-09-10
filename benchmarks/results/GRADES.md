# What to expect

**One run, one machine, 2026-09-10**, at commit `54044ac` on the dedicated Linux
measurement host — 21 unrelated containers removed, CPU regime pinned, scheduled
maintenance masked. Conditions:
[`../README.md#conditions`](../README.md#conditions), Host B. Raw output:
[`raw/*-linux.*`](raw/). History entry:
[`history/2026-09-10-54044ac-dirty.json`](history/). Every figure below is
written by hand from it, and nothing is carried across runs.

**Read these as orders of magnitude and as ratios between arms of one run.**
Nothing here should be quoted to a third significant figure.

> **This page changed host at `54044ac`.** Sections 1–7 were previously the
> 2026-09-06 Windows run at `b3c8d84`, and four of their figures had gone false
> as the code moved under them. [§8](#8-the-windows-record-and-what-it-is-still-for)
> keeps that record, what it is still for, and the cross-host check that is now
> the strongest single result here.

---

## 1. What the abstraction costs over raw SQL

From [`raw/overhead-linux.csv`](raw/overhead-linux.csv) — the **interleaved
paired runner**, not criterion, because a ratio taken from sequential arms is
worth less than one taken round-robin in a single process. Absolutes from
criterion, ratios from here, never mixed in one table.

| Operation | happenstance-sqlite over… | Ratio | Host A |
| --- | --- | ---: | ---: |
| Append, 8 events × 3 tags × 256 B | raw SQL on **its own schema** | **1.66×** | 1.7× |
| Append, same | a **hand-rolled** single-table event log | **4.05×** | 3.9× |
| Replay 2,000 events | raw SQL on its own schema | **5.53×** | 6.2× |

**How to read the two floors.** The first isolates the cost of the adapter's
Rust — query planning, tag dedup, the connection mutex, the paged read, the
`spawn_blocking` hop — from the schema those decisions were made for. The second
prices the library *and its schema* together, against what an engineer writes on
day one. Both floors are deliberately cheaper than a correct store: no event id,
no recorded time, no `tag_cardinality`, no append condition. That makes each
ratio an **upper** bound.

The `Host A` column is a different machine under a different OS with a 52× slower
clock, and three of these reproduce inside 10%. That is not a claim about either
host; it is the reason to believe the ratios describe the library. §8 has it in
full.

**Cross-adapter, with its caveat:** `SqliteEventStore` is **32.1×**
`MemoryEventStore` on the same append. Quote this one with care — it divides by
the `memory` arm, whose 5,727 ns median sits under this host's 29 µs timer floor
and is reported `TIMER-DOMINATED`. It is the one arm the `hpet` clocksource
costs. Host A, whose clock is 52× faster, read 120× on the same ratio, and the
disagreement is the clock rather than the adapters.

---

## 2. Throughput, and replay

From [`raw/store_append-linux.txt`](raw/store_append-linux.txt), owned regime
(the one a caller is in), 1 KiB payloads, 3 tags.

| Store | 1 event/batch | 128 events/batch (VT-24 floor) |
| --- | --- | --- |
| `MemoryEventStore` | ~665 K events/s | ~4.8 M events/s |
| `SqliteEventStore` | ~6.9 K events/s | ~57 K events/s |

Batching is worth about **8.2×** on SQLite and **7.2×** in memory. What SQLite
amortises per batch is one `BEGIN IMMEDIATE`, one commit and one `fsync` under
`synchronous = NORMAL`; what *memory* amortises is the per-call lock and the
`Vec` growth, which is why the two gains are closer here than the mechanism
suggests.

**Replay**, unfiltered, from [`raw/store_replay-linux.txt`](raw/store_replay-linux.txt):

| Log | memory | sqlite | raw floor | sqlite ÷ floor |
| --- | --- | --- | --- | ---: |
| 1,000 | 178 µs | 860 µs | 152 µs | **5.66×** |
| 10,000 | 4.09 ms | 9.79 ms | 2.38 ms | **4.11×** |

**Replay overhead falls as the log grows.** 5.66× at a thousand events against
4.11× at ten thousand — the read path stopped materialising a matched set per
page, and what that removed was a cost that grew with page depth.

`replay/page-hops` says it per event, either side of the 512-row page:

| Page count | ns/event |
| --- | ---: |
| 256 events (under one page) | 945 |
| 2,048 events (four pages) | 853 |
| 8,192 events (sixteen pages) | 979 |

A **1.15× spread across a 32× range of log lengths** — flat in page depth, which
is what a projection runner catching up actually meets. It is not *free*: 0.85–0.98 µs
an event is row materialisation, the decode and the per-hop `spawn_blocking`.
`PAGE_SIZE = 512` still describes itself as *"a placeholder until it is
measured"*, and on this evidence the case for measuring it is weak.

---

## 3. The read path

### `limit` is honoured, and `head()` is free

`limit(1)` against `limit(None)` over `Query::all`, at 10,000 events:

| Store | `head()` | `limit(1)` | `backwards().limit(1)` | `limit(None)` |
| --- | ---: | ---: | ---: | ---: |
| memory | **6.1 ns** | 132 ns | 135 ns | 3.23 ms |
| sqlite | **4.70 µs** | 469 µs | 472 µs | 9.97 ms |

`limit(1)` costs **0.004%** of `limit(None)` in memory and **4.7%** on SQLite.
The allocation counts ([`raw/allocations-linux.csv`](raw/allocations-linux.csv))
are exact and say it without a clock:

| Read | heap operations | peak live bytes |
| --- | ---: | ---: |
| `limit(None)` | 50,026 | 5,897,482 |
| `limit(1)` | **7** | 1,267 |
| `backwards().limit(1)` | **7** | 1,270 |
| `head()` | **0** | **0** |

> **This is a finding that reversed.** Through `b3c8d84` this section read
> *"`MemoryEventStore` does not honour `limit`"*, on 2.85 ms against 3.28 ms and
> 50,014 heap operations for `limit(1)`. It was true when written and the defect
> was fixed; §8 has the trace across three runs. Seven heap operations is what
> the fix looks like.

**The other half of that finding survives, and is still worth stating.**
`crates/happenstance-core/src/memory.rs:30-31` prices a snapshot as *"bumps
refcounts rather than copying data"*. A full read still costs **50,026 heap
operations and 5.9 MB of peak live bytes for 10,000 events** — five operations
per event, which is a clone of every one. `limit` no longer pays that; an
unbounded read still does, and the doc comment is still describing something
cheaper than what happens.

### ES-30's `head()` earns its keep

`head()` against the composed `backwards().limit(1)`, at 10,000 events:

| Store | `head()` | `backwards().limit(1)` | Ratio |
| --- | ---: | ---: | ---: |
| memory | **6.1 ns**, zero allocations | 135 ns | **22×** |
| sqlite | **4.70 µs** | 472 µs | **101×** |

`spec/SPECIFICATION.md:4006` makes `head` a required method on exactly this
argument. It is a measured decision rather than a plausible one — though note
that on Host A, before `limit` was honoured, the memory ratio was ~283,000×.
Fixing `limit` took four orders of magnitude off the case for `head`, and 22×
is what is left of it.

---

## 4. The DCB guard

The cost of evaluating an append condition against a seeded log and refusing —
the path a losing writer pays on every retry.
[`raw/store_conditional-linux.txt`](raw/store_conditional-linux.txt),
`SqliteEventStore`.

| Guard | 1,000 events | 10,000 events | Growth |
| --- | --- | --- | --- |
| single tag, selects one | 19.70 µs | 19.78 µs | **flat** |
| single tag, selects all | 19.68 µs | 19.76 µs | **flat** |
| **two-tag intersection** | **33.71 µs** | **33.96 µs** | **flat** |

The two-tag intersection is **1.72× a single-tag guard and flat in log length**.
That is the correlated `EXISTS` rewrite arriving through the port, confirmed here
on a second machine: the experiment measured the same change at statement level
and at sizes this suite cannot reach — 579,883 µs to 33 µs for a two-tag guard at
10⁶ events (`experiments/correlated-exists-guard/results/guard-cost.md`).

There was a version of this suite where that row read 270 µs at 1,000 events and
3.16 ms at 10,000 — linear-or-worse, and 305× a single-tag guard. It is not the
curve this adapter is on.

`MemoryEventStore` is flat at **14–18 ns** for all three, because its condition
check short-circuits on the first match.

**Contention** is clean on both arms. At *k* = 1, 8 and 64 contenders the
rejection mix is always exactly one winner, *k*−1 `ConditionViolated` rejections,
and **zero** failures. Both stores serialise their writers, as their designs say
they do. The cost of the contended round: SQLite 7.5 ms, 8.6 ms and 15.9 ms;
memory 4.0 µs, 4.3 µs and 7.6 µs.

---

## 5. The typed layer

### One command is a full replay plus one decode per prior event

[`raw/typed_command-linux.txt`](raw/typed_command-linux.txt), Json. Every
iteration starts from a restored boundary.

| Prior events | memory | sqlite |
| --- | ---: | ---: |
| 0 | 3.94 µs | 1.664 ms |
| 10 | 15.6 µs | 1.629 ms |
| 100 | 121 µs | 1.878 ms |
| 1,000 | 1.156 ms | 3.988 ms |

Linear at roughly **1.15 µs per prior event** on the memory arm — the decode and
fold cost, with no I/O under it. DCB has no aggregate to snapshot into, so this
is what an unbounded consistency boundary costs, and it is **the number to size a
domain against**. The SQLite arm is flat until 100 and then follows, because
below that the fixed cost of the transaction dominates.

### Codec choice is worth 2.8–5.2× on a warm boundary

At a 100-event boundary: **postcard 42.5 µs, json 121 µs, cbor 221 µs** — json is
2.8× postcard and cbor is 5.2×.

The encoded sizes matter as much as the times, because bytes become rows, pages
and write-ahead log. At a 64 KiB payload, from
[`raw/typed_codec-linux.txt`](raw/typed_codec-linux.txt):

| Codec | encoded bytes |
| --- | ---: |
| postcard | 65,544 |
| cbor | 131,119 |
| json | 196,667 |

json is **3.0×** postcard, which is ADR-0016's base64 encoding of `Bytes`
arriving on the wire. These are deterministic and reproduce byte-for-byte across
both hosts.

`Json` is the default and the one a first program gets. It is the right default,
and it is the most expensive of the three on both axes.

### A lost attempt costs a whole command

`command/retry`: **121 µs** uncontended, **243 µs** when the command loses once
and wins on its second attempt — **2.0×**. A rejected attempt has already paid
for the full replay and every decode before the condition refuses it.

---

## 6. The projection runner

[`raw/projection_runner-linux.txt`](raw/projection_runner-linux.txt), catching up
over 10,000 events from `Checkpoint::NeverRun`.

| Chunk | memory models | sqlite models |
| --- | --- | --- |
| 1 | 10.8 ms (928 K/s) | 590 ms (16.9 K/s) |
| 100 | 9.85 ms (1.02 M/s) | 59.0 ms (169 K/s) |
| 5,000 | 9.87 ms (1.01 M/s) | **52.8 ms (189 K/s)** |

**Chunk size is worth 11.2× on SQLite and nothing at all in memory.** Chunk 1 is
dominated by per-hop cost — one `spawn_blocking` and one transaction per event —
and no query change reaches it. So the gradient is the finding: chunk size is the
runner's business and every adapter's default is wrong for somebody.

**Idle poll** — one `run_projection` that finds nothing new: **76.6 µs** in
memory, **555 µs** on SQLite.

The operator arithmetic follows from that 555 µs: polling once a second is about
**0.06%** of a core; once every 10 ms is about **5.6%**. `experiments/polling-cost/`'s
fan-out amplification of **32.00 at 32 views** multiplies whatever the per-poll
cost is — it found no economy of scale anywhere in the range it swept — so
thirty-two views polling at 10 ms is not a configuration this adapter supports.

There was a version of this suite where the idle poll cost 5.68 ms, because it
emitted `WHERE position IN (SELECT position FROM event) AND position >= ?` and
the `IN`-driven search could not take the range bound: it re-walked the whole log
to discover there was nothing after the end of it.

---

## 7. Allocation, exactly

[`raw/allocations-linux.csv`](raw/allocations-linux.csv). These are reproducible
to the digit; the timings above are not. **They are the columns to quote.**

**`Event::clone()`, steady state** (the second clone — the first pays one more
operation for `Bytes`' shared header):

| Tags | owned | interned (control) |
| --- | --- | --- |
| 0 | 1 op, 17 B | 0 ops, 0 B |
| 1 | 3 ops, 50 B | 1 op, 24 B |
| 8 | 10 ops, 281 B | 1 op, 192 B |
| **64** (VT-22 floor) | **66 ops, 2,129 B** | **1 op, 1,536 B** |

`t + 2` in the owned regime and **flat at 1** in the interned one. This is why
every table in this suite carries a regime column: the interned arm is the one a
benchmark author writes *without choosing to*, and it would report the clone as
66× cheaper than a caller actually pays.

**This does not lift ES-17's marker.**
`.kb/open-questions/es-17-two-adapter-measurement-is-unscheduled.md` records what
that falsifier asks for — two builds of one adapter differing only in `append`'s
ownership, on one harness — and this suite is not that. What it supplies is the
numerator, and the warning that goes with it.

---

## 8. The Windows record, and what it is still for

Sections 1–7 were taken on a Windows laptop through 2026-09-08, at commits
`4a7ca16`, `b3c8d84` and `6c7a7a8`. That record is kept — `raw/` without the
`-linux` suffix, and three entries in `history/` — for two reasons.

### It is what the cross-host check is against

Same instrument, different hardware, different OS, a 52× slower clock:

| Ratio, paired runner | Host B (this page) | Host A |
| --- | ---: | ---: |
| append: sqlite ÷ raw same-schema | **1.66×** | 1.7× |
| append: sqlite ÷ hand-rolled | **4.05×** | 3.9× |
| replay: sqlite ÷ raw same-schema | **5.53×** | 6.2× |
| append: sqlite ÷ memory | 32.1× | 120× |

Three of four reproduce inside 10%. **That is the strongest evidence on this
page, and it is not a performance result** — it is what licenses reading §1 as a
property of the library rather than of a laptop, which is the thing
`../README.md`'s *"one machine, one run per cell"* has always conceded it could
not do. The fourth disagrees because it divides by the timer-dominated `memory`
arm; see §1.

And the allocation counts do better than reproduce. `raw/allocations.csv` and
`raw/allocations-linux.csv` are **byte-identical over every data row** — `diff`
returns nothing across two operating systems, two target triples and two commits.
When hosts whose wall-clock medians differ by 2–3× agree to the digit, the counts
are measuring the program and the timings are measuring the program *and the
machine*.

### It is the trace of four figures going false

The Windows runs bracket a set of code changes, and comparing them is how the
staleness in the previous version of this page was found. Per figure, at
10,000 events:

| figure | `b3c8d84` | `6c7a7a8` | Linux `54044ac` |
| --- | ---: | ---: | ---: |
| `memory` `limit(1)` | 2.853 ms | 291 ns | **132 ns** |
| `memory` `backwards().limit(1)` | 3.629 ms | 277 ns | **135 ns** |
| `sqlite` `limit(1)` | 93.8 µs | 539 µs | **469 µs** |
| `sqlite` idle poll | 184 µs | 682 µs | **555 µs** |

The first two are the `limit` fix, corroborated exactly by the allocation counts
(50,014 heap operations → 7). The second two moved the *other* way and nothing in
this suite explains either; they are left visible rather than smoothed. In all
four the Linux run agrees with `6c7a7a8` against `b3c8d84`, which is what makes
the older figure the outlier rather than the newer one — **a quiet third host as
a tiebreak between two runs on a busy one**.

### What it is not for

Comparing a Host A absolute with a Host B absolute. The clocks differ by 52×,
the page-hop cost by roughly 3×, and neither difference says anything about the
code.

---

## What none of this shows

[`../README.md#what-none-of-this-shows`](../README.md#what-none-of-this-shows),
in full; all eight points apply to every table above. The four that bite hardest
here:

- **One run per cell.** Where two arms differ by less than about 20%, this suite
  cannot tell them apart. The cross-host agreement in §8 is a check on the
  ratios, not a second sample of them.
- **No comparison against a peer library.** Declined deliberately, not deferred.
- **The floors are cheaper than a correct store.** §1's ratios are upper bounds,
  and are *not* a claim that a hand-rolled store would be correct.
- **One arm is timer-dominated on this host.** The `memory` append, at 5,727 ns
  against a 29 µs floor. It is the numerator of §1's cross-adapter row and the
  only figure here the `hpet` clocksource costs; `../README.md` records why that
  clocksource cannot be changed.
