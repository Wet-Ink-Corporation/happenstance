# What to expect

**One run, one machine, 2026-09-05**, at commit `4a7ca16` with an uncommitted
`benchmarks/` tree. Conditions:
[`../README.md#conditions`](../README.md#conditions). Raw output:
[`raw/`](raw/). Every figure below is written by hand from it, and nothing is
carried across runs.

**Read these as orders of magnitude and as ratios between arms of one run.**
Nothing here should be quoted to a third significant figure: on this host the
same append's median moved between 278 µs and 335 µs across three runs while the
allocation counts stayed identical to the digit.

---

## 1. What the abstraction costs over raw SQL

From [`raw/overhead.csv`](raw/overhead.csv) — the interleaved paired runner, not
criterion, because a ratio taken from sequential arms on this host is worth
nothing (see
[`../README.md`](../README.md#absolutes-from-criterion-ratios-from-the-paired-runner)).

| Operation | happenstance-sqlite over… | Ratio |
| --- | --- | --- |
| Append, 8 events × 3 tags × 256 B | raw SQL on **its own schema** | **1.7×** |
| Append, same | a **hand-rolled** single-table event log | **4.5×** |
| Replay 2,000 events | raw SQL on its own schema | **7.1×** |

Across three separate runs the append ratio came out at 1.66, 1.68 and 1.73. The
*ratio* is the stable thing; the absolutes underneath it are not.

**How to read the two floors.** The first isolates the cost of the adapter's
Rust — query planning, tag dedup, the connection mutex, the paged read, the
`spawn_blocking` hop — from the schema those decisions were made for. The second
prices the library *and its schema* together, against what an engineer writes on
day one. Both floors are deliberately cheaper than a correct store (no event id,
no recorded time, no `tag_cardinality`, no append condition), which makes each
ratio an **upper** bound.

**Cross-adapter:** `SqliteEventStore` is **111×** `MemoryEventStore` on the same
append. That is the price of durability, not the price of the abstraction.

---

## 2. Throughput

From [`raw/store_append.txt`](raw/store_append.txt), owned regime (the one a
caller is in), 1 KiB payloads, 3 tags.

| Store | 1 event/batch | 128 events/batch (VT-24 floor) |
| --- | --- | --- |
| `MemoryEventStore` | ~3.2 M events/s | ~4.5 M events/s |
| `SqliteEventStore` | ~6.0 K events/s | ~31 K events/s |

Batching is worth about **5×** on SQLite and 1.4× in memory. What SQLite
amortises per batch is one `BEGIN IMMEDIATE`, one commit and one `fsync` under
`synchronous = NORMAL`.

**Replay**, unfiltered, from [`raw/store_replay.txt`](raw/store_replay.txt):

| Log | memory | sqlite | raw floor | sqlite ÷ floor |
| --- | --- | --- | --- | --- |
| 1,000 | 276 µs | 1.17 ms | 176 µs | 6.6× |
| 10,000 | 3.97 ms | 53.9 ms | 4.87 ms | **11.1×** |

The replay overhead **grows with log length** — 6.6× at a thousand events,
11.1× at ten thousand. `replay/page-hops` says the same thing per event, either
side of the 512-row page: **1.9 µs/event at 256, 2.2 at 2,048, 4.4 at 8,192.**
`PAGE_SIZE = 512` describes itself as *"a placeholder until it is measured"*,
and this is the shape a projection runner catching up meets.

---

## 3. Two places the documentation is wrong

### `MemoryEventStore` does not honour `limit`; `SqliteEventStore` does

`limit(1)` against `limit(None)` over `Query::all`, at 10,000 events:

| Store | `limit(1)` | `limit(None)` | Ratio |
| --- | --- | --- | --- |
| memory | 2.63 ms | 3.64 ms | **0.72×** |
| sqlite | 68.9 µs | 42.0 ms | 0.0016× |

Asking for one event out of ten thousand costs **72%** of asking for all of them
in memory, and **0.16%** on SQLite. The allocation counts
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

| Store | `head()` | `backwards().limit(1)` | Ratio |
| --- | --- | --- | --- |
| memory | **13.6 ns**, zero allocations | 2.84 ms | ~209,000× |
| sqlite | **4.7 µs** | 76.0 µs | 16× |

`spec/SPECIFICATION.md:4006` makes `head` a required method on exactly this
argument. It is now a measured decision rather than a plausible one.

---

## 4. The DCB guard, and the one shape that does not scale

The cost of evaluating an append condition against a seeded log and refusing —
the path a losing writer pays on every retry.
[`raw/store_conditional.txt`](raw/store_conditional.txt), `SqliteEventStore`.

| Guard | 1,000 events | 10,000 events | Growth |
| --- | --- | --- | --- |
| single tag, selects one | 9.1 µs | 10.4 µs | **flat** |
| single tag, selects all | 9.0 µs | 9.9 µs | **flat** |
| **two-tag intersection** | **270 µs** | **3.16 ms** | **11.7× for 10× the log** |

Single-tag guards are flat in log length — the tag index does its job. The
**two-tag intersection is linear-or-worse and 305× more expensive at 10,000
events**, and it is evaluated **inside `BEGIN IMMEDIATE`** with every other
writer queued behind it. A 3.2 ms guard is not one caller's latency; it is
3.2 ms that every concurrent writer waits.

This reproduces finding I-1 through the port at affordable log lengths.
`references/evaluation/review-pre-publication-2026-09-03.md:2540` measured the
same shape at **296 ms at 500,000 events and 560 ms at 10^6** — which is where
this curve goes — and `experiments/shipped-append-condition-sql/` has the
statement-level detail.

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

| Prior events | memory | sqlite |
| --- | --- | --- |
| 0 | 1.37 µs | 1.89 ms |
| 10 | 18.3 µs | 1.97 ms |
| 100 | 137 µs | 2.63 ms |
| 1,000 | 1.43 ms | 6.17 ms |

Linear at roughly **1.4 µs per prior event** on the memory arm — the decode and
fold cost, with no I/O under it. DCB has no aggregate to snapshot into, so this
is what an unbounded consistency boundary costs, and it is the number to size a
domain against.

### Codec choice is worth 2–3× on a warm boundary

At a 100-event boundary: **postcard 56 µs, json 143 µs, cbor 170 µs** — json is
2.5× postcard and cbor is 3.0×.

The encoded sizes matter as much as the times, because bytes become rows, pages
and write-ahead log. At a 64 KiB payload: **postcard 65,544 B, cbor 131,119 B,
json 196,667 B** — json is **3.0×** postcard, which is ADR-0016's base64
encoding of `Bytes` arriving on the wire.

`Json` is the default and the one a first program gets. It is the right default,
and it is the most expensive of the three on both axes.

### A lost attempt costs a whole command

`command/retry`: **139 µs** uncontended, **287 µs** when the command loses once
and wins on its second attempt — **2.1×**. A rejected attempt has already paid
for the full replay and every decode before the condition refuses it.

---

## 6. The projection runner

[`raw/projection_runner.txt`](raw/projection_runner.txt), catching up over
10,000 events from `Checkpoint::NeverRun`.

| Chunk | memory models | sqlite models |
| --- | --- | --- |
| 1 | 15.1 ms (661 K/s) | 980 ms (10.2 K/s) |
| 100 | 11.2 ms (893 K/s) | 145 ms (68.8 K/s) |
| 5,000 | 11.9 ms (840 K/s) | 132 ms (75.5 K/s) |

**Chunk size is worth 7.4× on SQLite and nothing at all in memory.** That is
Wattline's concern measured (`references/scenarios/README.md:665-670`): *"chunk
size is the runner's business and every adapter's default is wrong for this."*
Almost all of the win is between 1 and 100; the step from 100 to 5,000 buys a
further 10%.

**Idle poll** — one `run_projection` that finds nothing new: **95.4 µs** in
memory, **5.68 ms** on SQLite. An operator polling a SQLite projection once a
second spends about 0.6% of a core on it; once every 10 ms, about 57%. Multiply
by `experiments/polling-cost/`'s measured fan-out amplification of **32.00 at 32
views** — it found no economy of scale anywhere in the range it swept.

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
