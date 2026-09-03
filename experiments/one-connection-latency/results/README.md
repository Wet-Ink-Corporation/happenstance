# Results

Written by hand from [`raw/`](raw/), which is one `./run.sh` that ran
2026-09-03 07:42:15–07:47:27 local. Nothing here is carried across runs.
Conditions — machine, OS, filesystem, toolchain, build profile, and the pragmas
read back off the live connection — are in [`../README.md`](../README.md#conditions),
and the settings string is printed at the top of `raw/reactor-stall.txt` beside
the figures it governs.

**Two controls ran before any clock.** `raw/drift.txt`: the two verbatim copies
re-derived from `crates/happenstance-sqlite/src/` at the live tree, `4 passed;
0 failed`. `raw/conformance.txt`: 89 conformance rules × 4 page sizes,
`356 passed; 0 failed`. A wrong page size is always the fastest — it does not
produce a slow read, it produces a read that stops early or repeats a row at a
page boundary — and the suite is what earns each timed page size a place in a
table.

| page | what is on it |
| --- | --- |
| [`reactor-stall.md`](reactor-stall.md) | instruments (a) and (c): eight arms on one `current_thread` runtime, CONTROL 1, and the residual the seam does not close |
| [`page-lock-hold.md`](page-lock-hold.md) | instrument (b): `PAGE_SIZE ∈ {64,128,512,2048}` × query width ∈ {1,400,1200} over 10^6 events — shape, hold, what a sharing caller pays, and the full-replay extrapolation |
| [`ceiling-residency.md`](ceiling-residency.md) | R-1 head on: one page of 512 events at exactly `MAX_EVENT_DATA_LEN` |
| [`copy-fidelity.md`](copy-fidelity.md) | the control under all three: what is the real crate, what is a copy, and what says the copy has not drifted |

## The question, and the answer

> What does `happenstance-sqlite`'s one-connection-behind-one-mutex shape cost —
> to the reactor, to a caller sharing the handle, and to memory — and does the
> `in_blocking_task` seam the sibling module already has fix it?

**A contended `append` stalls a `current_thread` reactor for 885.761 ms against
an idle floor of 16.177 ms. The seam takes that to 34.812 ms, 25.4x, and takes
`head()` from 717.545 ms to 41.573 ms — and leaves `read()`'s first poll at
635.056 ms, unchanged, because ES-11 puts the ceiling sample on the polling
thread by requirement.**

The headline row, and the control that gives it scale — one
`current_thread` runtime, a 1 ms ticker, a second connection holding
`BEGIN IMMEDIATE` for 750 ms:

| arm | max tick gap | against the idle floor |
| --- | ---: | ---: |
| **0. idle** (Windows timer resolution) | 16.177 ms | — |
| **1. SHIPPED `append`** | **885.761 ms** | **54.8x** |
| 2. CONTROL 1: SHIPPED `SqliteProjectionStore::commit` | 27.525 ms | 1.70x |
| 3. the same append through an `in_blocking_task` seam | 34.812 ms | 2.15x |
| **7. residual: `read()` first poll, after the seam** | **635.056 ms** | **39.3x** |

Arms 1 and 2 wait the same 750 ms for the same lock — their calls take 870.3 ms
and 870.9 ms — and one of them takes the runtime down with it. That is the whole
result in two rows, and without arm 2 the 885.761 ms would be a number with no
scale.

## Verdict per finding

| finding | verdict | the number |
| --- | --- | --- |
| **J.J-2** — `append`, `head` and `contains_event_id` run SQLite on the calling executor thread; the README says the opposite | **confirms** | 885.761 ms max tick gap against a 16.177 ms floor, and the ticker fired 5 times where it fired 54 uncontended. The maximum gap is 99–102% of the call in every affected arm — no partial stall, no yield point |
| **F2.F2-1** — three of four port methods run rusqlite on the executor thread, the option ADR-0022 §9 rejected, the defect the sibling module names by name | **confirms**, with the sibling as the control | the sibling's own `commit` under identical contention: **27.525 ms**. Transcribing its `in_blocking_task` onto the event store's bodies reaches **34.812 ms** — the proposed fix works, measured, at 25.4x |
| **I.I-4** — …and the read's ceiling sample takes the same lock on the executor thread | **confirms**, and the last clause is the one that survives the fix | `head()` 717.545 → 41.573 ms through the seam (17.3x); `read()` first poll **639.290 → 635.056 ms, 0.7%**. See the residual section below |
| **J.J-5** — `PAGE_SIZE = 512` is an unmeasured placeholder and *the only knob controlling how long a read holds the write connection* | **qualifies** — the premise is confirmed and the mechanism is falsified | Over a **32x** page-size range the per-page hold moves **1.06–1.15x** (conc p50). What moves it is query width: 146.6 → 203.3 → **684.2 ms** at page 512 for widths 1, 400, 1,200. But `PAGE_SIZE` does control the aggregate: **286.4 s** of held mutex over a 10^6-event replay at 512 against **99.1 s** at 2,048 and **2,929.2 s** at 64 |
| **R.R-1** — a read page's memory is bounded by row count, not bytes, and the constant is a stated placeholder | **confirms**, to within 0.04% | one page of 512 events at exactly `MAX_EVENT_DATA_LEN` peaks at **537,036,800 live bytes = 512.2 MiB** in one buffer before a single row reaches the caller, and holds the mutex 583.2 ms doing it |

### The residual, stated separately because it is the finding the fix leaves behind

Arms 5 and 7 both drive a `read()` to its first row behind an in-flight append
holding the connection mutex. Arm 5 is the shipped `SqliteEventStore`; arm 7 is
the replica with the sibling append on its own connection. **639.290 ms and
635.056 ms.** The seam moved nothing, and it cannot:

* ES-11 requires the ceiling to be sampled **no later than the first poll**;
* `ReadCursor::sample_ceiling` (`crates/happenstance-sqlite/src/event_store.rs:1239`)
  therefore takes the connection mutex from inside `poll_next`, on the polling
  thread, deliberately;
* a seam on the **write** side changes where the mutex is *held*, not where it is
  *acquired* on the read side.

So the fix that J-2, F2-1 and I-4 all propose — route `append`, `head` and
`contains_event_id` through `in_blocking_task` — is a good fix that closes three
of four, and leaves a caller who issues `read()` on a `current_thread` runtime
parked for as long as any in-flight append holds the mutex: **39.3x the floor,
after the fix**, and up to `BUSY_TIMEOUT_MS = 5_000` in the worst case. Closing
it means making `sample_ceiling` a cooperative re-poll rather than a blocking
acquisition, which is a different and larger change and should be recorded as a
residual rather than absorbed into the seam story.

### And one thing nobody asked for

**J-5 and R-1 point in opposite directions and neither can be settled by a row
count.** The lock-hold table says `PAGE_SIZE` should be *raised* — 512 → 2,048
removes 187 s to 1,028 s of held mutex from a 10^6-event rebuild — because the
per-page hold is flat and the page count is not. The ceiling arm says raising it
is unsafe, because the thing that scales linearly with `PAGE_SIZE` is the one
with no ceiling: at 512 rows a page of ceiling-sized events is already 512 MiB,
and at 2,048 it would be about 2 GiB.

The measured answer is that R-1's byte budget has to land **first**. With one,
`PAGE_SIZE` becomes a number that can be raised on the strength of the lock-hold
table; without one it is a number that cannot safely be moved in either
direction. Doing either alone makes something worse.

A second thing the tables show that no finding asked about: **the cost of sharing
a handle is asymmetric**. At the shipped page size and a 1,200-item query, a page
holds the mutex for 640.7 ms and waits **1.7 ms** for it — 371x. The concurrent
appender's p50 is untouched (0.148 ms against a quiet 0.118 ms) while its p99
goes from 7.727 ms to **799.041 ms**, 103x, with a worst observed append of
945.195 ms. Command latency is quantised by the replay's page time at the tail
only, which is worse than a uniform slowdown: nothing in the median warns anyone.

## What none of this shows

Stated once here and again on each page, because a reader who takes only the
table will otherwise assume them away.

1. **One machine, one run, one observation per cell.** No repetition, no
   confidence interval, no second host. `experiments/append-condition` records
   45% run-to-run variance on this host. Every claim here is about a **ratio
   between arms of one run** and about orders of magnitude — 885 ms against
   27 ms — never about a third significant figure. Nothing should be quoted as
   "885.761 ms of `append`".
2. **The floor is the operating system's, and it is 16 ms.** Windows' default
   system timer resolution is 15.6 ms, which is why the idle arm reads 16.177 ms
   and every arm's p50 is 15.48–16.01 ms. Nothing below about 16 ms is measurable
   by this instrument at all, and the difference between the seam's 34.8 ms and
   CONTROL 1's 27.5 ms is beneath its resolution. **The idle row is the
   resolution, not a stall.**
3. **These arms succeed.** The write-lock holder releases at 750 ms, comfortably
   inside the 5,000 ms busy timeout. The worst case — a stall of the full
   `BUSY_TIMEOUT_MS` followed by `DatabaseBusy` — is arithmetic from
   `connection.rs:62`, not a measurement, because measuring it means measuring a
   failure.
4. **`current_thread` is the worst case for the stall, and it was chosen on
   purpose.** On a multi-threaded runtime the same call parks one worker rather
   than the whole reactor, and the damage is throughput rather than stopped
   timers. That arm was not run.
5. **Peak bytes is live-bytes from a counting global allocator, not RSS.** It
   excludes the system allocator's size-class rounding and its retained free
   pages, and it includes SQLite's page cache. It is a **lower bound** on what a
   page costs the process.
6. **The mechanism behind the flat per-page hold was not measured here.** No
   `EXPLAIN QUERY PLAN` was run in this experiment. The IN-operator explanation
   is imported from `experiments/shipped-append-condition-sql/results/query-plans.md`
   and offered as consistent-with, not established.
7. **The full-replay column is an extrapolation and is labelled one.** No
   configuration was driven to exhaustion; the longest pass was 200 pages or
   15 s. It assumes the sampled `hold_p50` holds for all 15,625 pages, which
   nothing here tested.
8. **128-byte payloads, single-tag query items, no metadata.** The lock-hold
   table says nothing about multi-tag guards (which add cardinality lookups and
   an intersection chain inside the same hold) and nothing about R-1, which has
   its own arm at the other end.
9. **The seam's own cost is stated, not measured.** `spawn_blocking` needs a
   `'static` closure, so `events.to_vec()` deep-clones every `Event` —
   `t + 2` allocations each by AE-1's mechanism, against a shipped `append` that
   takes `&[Event]` and clones nothing. `Bytes` is refcounted so the payload does
   not copy. That is why the fix is a decision rather than an obvious win, and it
   is not on any table here.
10. **This is an experiment, never a gate step** (CF-34). It is not a workspace
    member, `cargo xtask ci` cannot see it, and every assertion in every timed
    target is deliberately weak — it fires only if the instrument stopped
    working. A threshold on any figure here would make this crate a gate step by
    the back door, and `run.sh` must never be wired to anything that can turn a
    merge red.
