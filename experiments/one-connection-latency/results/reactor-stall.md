# Reactor stall — instruments (a) and (c)

Written by hand from [`raw/reactor-stall.txt`](raw/reactor-stall.txt). Conditions
are in [`../README.md`](../README.md#conditions).

> **The question.** On a `current_thread` runtime, how long is the executor
> unavailable to do anything else while a contended `SqliteEventStore` call is in
> flight — and how much of that does the `in_blocking_task` seam the sibling
> module already has take away?

## The construction

One `current_thread` tokio runtime — the flavour `#[tokio::test]` defaults to,
the flavour `examples/transfers-on-sqlite` uses, and the flavour a local-first
deployment is most likely to be on. On it, a task ticks a 1 ms
`tokio::time::interval` and records every gap. Beside it, a bare OS thread opens
a **second connection** onto the same file through the shipped adapter's own
`connection::open_configured` and holds `BEGIN IMMEDIATE` for 750 ms. Each arm
then does one thing, and the ticker's largest gap is the answer.

`MissedTickBehavior::Delay`, not the default `Burst`: under `Burst` a 700 ms
stall is followed by 700 immediate catch-up ticks separated by nanoseconds, and
every percentile below the maximum becomes a fiction about the recovery rather
than about the stall.

750 ms is well under `BUSY_TIMEOUT_MS = 5_000` (`connection.rs:62`), and that is
the point: these are arms that **succeed**. The figures below are what a
contended append costs on a good day, not what the timeout costs on a bad one.

## The table

Settings, read back off the live connection at the top of the raw file:
`journal_mode=wal synchronous=1 busy_timeout_ms=5000`.

| # | arm | call | **max gap** | p99 | p50 | ticks |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 0 | **idle** — the ticker alone, no contention, no call | 762.6 ms | **16.177 ms** | 16.177 | 15.516 | 54 |
| 1 | **SHIPPED** `SqliteEventStore::append`, write lock held | 870.3 ms | **885.761 ms** | 885.761 | 16.008 | **5** |
| 2 | **CONTROL 1** — SHIPPED `SqliteProjectionStore::commit`, write lock held | 870.9 ms | **27.525 ms** | 27.525 | 15.484 | 62 |
| 3 | (c) the same append through an `in_blocking_task` seam | 778.9 ms | **34.812 ms** | 34.812 | 15.504 | 54 |
| 4 | **SHIPPED** `head()`, behind an in-flight append on the same handle | 873.4 ms | **717.545 ms** | 717.545 | 15.860 | 15 |
| 5 | **SHIPPED** `read()`: first poll samples the ceiling on this thread | 793.4 ms | **639.290 ms** | 639.290 | 15.567 | 15 |
| 6 | (c) `head()` through the seam, behind an in-flight inline append | 777.7 ms | **41.573 ms** | 41.573 | 15.479 | 54 |
| 7 | (c) **residual** — replica `read()` first poll, behind the same append | 789.4 ms | **635.056 ms** | 635.056 | 15.771 | 15 |

## Row 0 is the floor, and it is the operating system's

**16.177 ms is the Windows default timer resolution, not a stall.** The system
timer ticks at 15.6 ms unless something in the process has called
`timeBeginPeriod`; a 1 ms `tokio::time::interval` on an idle `current_thread`
reactor therefore produces ~15.5 ms gaps (the p50 column, which is 15.48–16.01 ms
in *every* row of the table, contended or not). Nothing below about 16 ms is
measurable by this instrument at all, and no figure here should be quoted without
this row beside it.

That the p50 is flat across all eight arms is itself a control: it says the
ticker kept the same rhythm everywhere, and that what the max-gap column varies
is a single interruption rather than a change in cadence.

## Two contentions, because WAL makes them different

Under WAL a writer does not block readers. So the arms split:

* **The file write lock** stalls `append`, which opens `BEGIN IMMEDIATE`. Arms
  1, 2 and 3.
* **The connection mutex** stalls everything else on the same handle. `head`,
  `contains_event_id` and `ReadCursor::sample_ceiling` all run plain `SELECT`s
  that WAL would happily serve concurrently — they wait because the *adapter*
  serialises them, not because SQLite does. Arms 4–7 hold an `append` in flight
  on another OS thread and measure that.

Measuring only the first would have produced two rows of zeroes and the wrong
conclusion about `head` and about the read's first poll.

## The stall is the whole of the call

The arithmetic that makes these figures more than eight numbers. Arms 4–7 sleep
150 ms (`SETTLE`) before the measured call, so that the sibling `append` has
certainly reached the mutex:

| arm | call | call − settle | max gap | gap as a share of the unavoidable window |
| --- | ---: | ---: | ---: | ---: |
| 1 | 870.3 | — | 885.761 | **102%** |
| 2 | 870.9 | — | 27.525 | 3.2% |
| 3 | 778.9 | — | 34.812 | 4.5% |
| 4 | 873.4 | 723.4 | 717.545 | **99%** |
| 5 | 793.4 | 643.4 | 639.290 | **99%** |
| 6 | 777.7 | 627.7 | 41.573 | 6.6% |
| 7 | 789.4 | 639.4 | 635.056 | **99%** |

In arms 1, 4, 5 and 7 the maximum gap is the *entire* call. There is no partial
stall, no yield point, nothing that fires halfway: the reactor is unavailable
from the moment the call begins until the moment it returns. Arm 1's 102% is not
an error — a gap runs from the last tick before the stall to the first after it,
so it may exceed the call by up to one tick period, which is 15.6 ms here.

The tick counts say the same thing in the other direction. Arm 1 fired **5**
timers over its window; the idle arm fired 54 over a shorter one. About 91% of
the ticks that should have fired did not.

## What CONTROL 1 buys

Arm 2 is the *real* `SqliteProjectionStore::commit`
(`crates/happenstance-sqlite/src/projection_store.rs:592`) under exactly the same
contention: same file, same holder, same 750 ms write lock. Its call takes
**870.9 ms** — as long as arm 1's, to within 0.7% — and its maximum tick gap is
**27.525 ms**, 1.70x the idle floor.

That is the whole result in two rows. Both calls wait the same 750 ms for the
same lock; one of them takes the runtime down with it and the other does not. The
difference is `in_blocking_task` (`projection_store.rs:342`), which the sibling
module has and the event store does not — and whose own module doc names the
asymmetry as a defect in advance of the event store having it.

Without arm 2, 885.761 ms is a number with no scale: a reader could reasonably
conclude that waiting 750 ms for a write lock is simply what contention costs.
Arm 2 says it is not.

## Instrument (c): what the proposed fix moves

`src/seam.rs` transcribes `in_blocking_task` onto the event store's three
synchronous bodies — the fix J-2, F2-1 and I-4 all propose — over the replica's
connection, so that the seam arm and its contention are on the *same* mutex.

| what | shipped | through the seam | ratio | against the 16.177 ms floor |
| --- | ---: | ---: | ---: | ---: |
| contended `append` | 885.761 ms | **34.812 ms** | **25.4x** | 2.15x |
| `head()` behind an in-flight append | 717.545 ms | **41.573 ms** | **17.3x** | 2.57x |
| `read()` first poll behind the same append | 639.290 ms | **635.056 ms** | **1.007x** | 39.3x |

The first two land within a factor of 1.5 of CONTROL 1's 27.525 ms, which is what
"the seam works" looks like: the residual is the cost of getting onto and off a
blocking thread, and it is small enough to sit in the same order of magnitude as
the timer granularity.

**The seam is not free, and the figure does not show its cost.**
`spawn_blocking` demands a `'static` closure, so the batch has to be *owned* by
it: `events.to_vec()` deep-clones every `Event`, which by AE-1's mechanism is
`t + 2` allocations each. The shipped `append` takes `&[Event]` and clones
nothing. `Bytes` is refcounted so the payload itself does not copy, but the
remediation has to book the rest. That is why this is a decision rather than an
obvious win, and it is not measured here.

## Row 7 — the residual the obvious fix does not close

**This is the most interesting row in the table and the easiest to bury, so it
gets its own section.**

Arms 5 and 7 both drive a `read()` to its first row behind an in-flight append
holding the connection mutex. Arm 5 is the shipped `SqliteEventStore`; arm 7 is
the replica, with the sibling append on the replica's own connection. They
measure **639.290 ms** and **635.056 ms** — 0.7% apart.

The seam moved nothing. It cannot:

* ES-11 requires the ceiling to be sampled **no later than the first poll**.
* `ReadCursor::sample_ceiling` (`crates/happenstance-sqlite/src/event_store.rs:1239`)
  therefore takes the connection mutex from inside `poll_next`, deliberately, on
  the polling thread. The `spawn_blocking` hop that the rest of `read` already
  uses happens *after* it — `read` is correct about pages and wrong about its
  first poll.
* A seam on the **write** side moves where the mutex is *held*. It does not move
  where it is *acquired* on the read side, because that acquisition is on the
  polling thread by requirement, not by oversight.

So a fix that routes `append`, `head` and `contains_event_id` through
`in_blocking_task` — the fix all three findings propose, and a good one — leaves
a caller who issues `read()` on a `current_thread` runtime parked for as long as
any in-flight append holds the mutex, up to `BUSY_TIMEOUT_MS = 5_000` in the
worst case. **39.3x the floor, after the fix.**

This is a residual to record, not a reason to reject the fix. It is also a
constraint on what any complete remediation looks like: it has to make
`sample_ceiling` a cooperative re-poll (returning `Pending` and waking when the
mutex frees) rather than a blocking acquisition, which is a different and larger
change than adding a seam — and one that turns a 5,000 ms hard stall into a
scheduling question rather than eliminating it.

### A second thing row 7 shows, for free

Arms 5 and 7 are two different code paths — the shipped adapter and
`src/replica.rs` — measured under the same construction, and they agree to
**0.7%**. That is a fidelity check on the copy that no textual diff could
produce, and it sits alongside CONTROL 2's 89 rules × 4 page sizes.

## What this page does not show

1. **Windows, one machine, one run, one arm each.** There is no repetition and no
   confidence interval. These are single observations, and the claim they carry
   is a claim about *order of magnitude* — 885 ms against 27 ms — not about the
   third significant figure. Nothing here should be quoted as 885.761 ms of
   anything except this run.
2. **The floor is 16 ms and it is the operating system's.** No arm's difference
   from another arm is meaningful below about 16 ms, and the seam arms' 34.8 ms
   and 41.6 ms are only about 2x the floor. Whether the seam costs 19 ms or 25 ms
   over CONTROL 1 is beneath this instrument's resolution.
3. **These arms succeed.** The holder releases at 750 ms, comfortably inside the
   5,000 ms busy timeout. The worst case — a stall of the full `BUSY_TIMEOUT_MS`
   followed by `DatabaseBusy` — is arithmetic in `connection.rs:62`, not a
   measurement here, because measuring it means measuring a failure.
4. **`current_thread` is the worst case for this defect and it was chosen on
   purpose.** On a multi-threaded runtime the same call parks one worker thread
   rather than the whole reactor, and the observable damage is throughput rather
   than stopped timers. That arm was not run.
5. **The seam's allocation cost is stated, not measured.** `events.to_vec()`'s
   `t + 2` allocations per event are AE-1's mechanism applied, not a figure taken
   here.
6. **Arm 7's sibling append is inline, not through the seam.** The residual would
   be the same either way — the mutex is held for the same span whichever thread
   holds it — but this run did not build the arm that proves that separately.
7. **This is an experiment, never a gate step** (CF-34). The one assertion in the
   test file checks only that arm 1 exceeded arm 0, so that a broken instrument
   fails loudly; a threshold on any figure here would make this crate a gate step
   by the back door.
