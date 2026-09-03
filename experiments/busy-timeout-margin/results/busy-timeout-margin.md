# What the busy handler costs, and what is left of 5,000 ms

Written by hand from [`raw/`](raw/), which is one `./run.sh`. Conditions — the
machine, the toolchain, the build profile, the core-constraint method and how it
was verified — are in [`../README.md`](../README.md#conditions), and every row of
`raw/` carries its own settings string beside it: the affinity mask read off the
live process, the pragmas off the live connection, the handler and the cap.

**Conformance ran before any clock.** 178 tests — 89 rules × 2 arms — passing
under the counting handler (`raw/conformance-counting.txt`) *and* under SQLite's
own (`raw/conformance-default.txt`), plus two unit tests asserting the handler
transcribes `sqliteDefaultBusyCallback`'s tables rather than inventing a
schedule. This crate replaces the store's busy handler on the write path, so a
figure from it only counts if the store still passes.

## The question, and the answer

> In the configuration the gate actually runs, what is the worst per-contender
> wait inside SQLite's busy handler at `CONTENDERS = 64`, and how few cores does
> it take to exceed 5,000 ms?

**3,628 ms of the 5,000 ms budget — a margin of 1.38x — and no core count
exceeds the cap, because removing cores makes it better, not worse.**

Two numbers, because they answer two questions and conflating them overstates the
result. `cargo xtask ci` runs **unconstrained**, so the gate's own configuration
is the 20-core row: **3,628 ms, 1.38x**. The worst cell anywhere in the sweep is
the 8-core row at **3,828 ms, 1.31x** — and 8 and 20 are not separable on this
host, so the defensible statement is *a margin of roughly 1.3x–1.4x on the
plateau*.

The cap is not reached on the *core* axis at all. It is reached on the
**contender** axis, and the distance is short: at 96 contenders SQLite's own
handler gives up on 2.2% of attempts.

| | worst per-contender wait | margin |
| --- | ---: | ---: |
| 1 core | 8 ms | 625x |
| 2 cores | 628 ms | 8.0x |
| 4 cores | 2,628 ms | 1.9x |
| 8 cores | **3,828 ms** | **1.31x** |
| 20 cores | 3,628 ms | 1.38x |
| 20 cores, the arm that ships | 3,728 ms | 1.34x |

## The control, and what the instrument costs

`--release`, every core, one racing test alone — the shape ADR-0022 §11's row was
produced under (`experiments/append-condition/results/raw/contention.txt:24`).

| | median µs | min µs | max µs | committed | rejected | busy |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| ADR-0022 §11 (recorded) | 2,724,759 | 1,970,474 | 3,496,577 | 10 | 630 | 0 |
| control, SQLite's own handler | 3,697,053 | 2,918,558 | 4,664,995 | 10 | 630 | **0** |
| control, this crate's counting handler | 1,439,930 | 762,385 | 3,250,315 | 10 | 630 | **0** |

**The shape reproduces exactly; the timing is inflated by a loaded host, and the
run measures that rather than asserting it.** `committed = 10`, `rejected = 630`,
`busy = 0`, `failed = 0` were identical to the record in every one of five
complete runs — those are what a broken transcription breaks first. The median is
1.36x the record here (1.31x–1.72x across runs), and the same binary's speedup
probe read **5.8 cores of throughput available out of 20** on the same launch
(`raw/cores-debug-c20.txt`). A third of the machine explains a third more wall
clock. Every comparison on this page is therefore *within* one run.

**The counting handler is optimistic, and by an uncalibrated amount.** The two
control rows differ in one thing — which handler is installed — and the counted
race resolves **2.6x faster** on the median. The cause is timer resolution:
SQLite sleeps through `Sleep()` at ~15.6 ms granularity, `std::thread::sleep`
uses a high-resolution timer, and a tighter poll resolves a thundering herd
sooner. Across five runs the ratio was 2.18x, 2.27x, 2.40x, **0.66x** and 2.57x —
directionally consistent, numerically unstable, and the inverted run is reported
rather than dropped.

This does not put `wait_ms` in the wrong units. SQLite applies its cap against
`TOTALS`, *a table of scheduled milliseconds*, not against a clock — so the
budget a contender burns is commensurable under either handler. What it means is
that **every `wait_ms` figure here is a floor**: a counted contender waited inside
a less contended race than the shipped handler creates. That is exactly why the
headroom sweep below gives up the accounting and asks SQLite's own handler the
one-bit question instead.

## The core sweep, and why it runs backwards

Each cell is the worst of three concurrently racing rules, each contributing 640
samples (10 rounds × 64 contenders) — the maximum of 1,920.

| cores | measured speedup | worst max | p99 | p90 | p50 | retries |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1.0 | **8** | 3 | 3 | 1 | 913 |
| 2 | 1.7 | **628** | 428 | 103 | 3 | 2,357 |
| 4 | 3.7 | **2,628** | 2,028 | 1,028 | 78 | 5,771 |
| 8 | 5.2 | **3,828** | 3,228 | 1,828 | 328 | 8,817 |
| 20 | 5.8 | **3,628** | 3,028 | 1,928 | 428 | 9,269 |
| 20, `monotonic-guard` | 5.8 | **3,728** | 3,128 | 1,728 | 178 | 7,775 |

`busy = 0` and `exhausted = 0` in all eighteen rows. Across five runs the worst
cell ranged 3,128–3,828 ms; **8 and 20 cores are not distinguishable** and traded
places between runs, so the honest shape is a steep rise from 1 to 4 and a
plateau after.

**The hypothesis this sweep was built to test is refuted in its direction.** A
2-core container is the *safest* configuration in the matrix, not the most
dangerous, and `taskset -c 0,1` would have reported an 8.0x margin as though it
were the hazard. The mechanism is in the `retries` column — 913 entries into the
handler at one core, 8,817 at eight. With one core the scheduler serialises the
herd *before* it reaches SQLite's file lock; with eight, SQLite's lock is the only
thing left to serialise it. **The busy handler only does work when contenders
overlap in time, and cores are what let them overlap.**

The build profile barely enters either. Release at 20 cores gave 3,228 ms alone
and 3,328 ms with three rules concurrent — inside the same band as debug's 3,628.
The time is spent *asleep*, so `--release` has nothing to recover. Both halves of
the finding's proposed mechanism — debug overhead, few cores — fail.

The shipped arm is not cheaper: `monotonic-guard` reached 3,728 ms against
`begin-immediate-probe`'s 3,628 ms. **The margin is a property of the herd, not of
the guard.**

## The distribution, not the mean

The cap is exhausted by one unlucky contender, so the tail *is* the measurement.
SQLite's handler is a back-off **poll, not a queue** — no FIFO, no fairness, the
lock goes to whoever happens to retry when it is released. The unluckiest wait is
not `63 × T_commit`; it is the tail of a skewed distribution.

640 samples, the worst cell in the matrix (8 cores):

| wait (ms) | contenders | |
| --- | ---: | --- |
| 0 | 10 | the round's winner, which never waits |
| 1–100 | 197 | ▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇ |
| 101–500 | 156 | ▇▇▇▇▇▇▇▇▇▇▇▇▇▇▇ |
| 501–1,000 | 85 | ▇▇▇▇▇▇▇▇ |
| 1,001–2,000 | 142 | ▇▇▇▇▇▇▇▇▇▇▇▇▇▇ |
| 2,001–3,000 | 41 | ▇▇▇▇ |
| 3,001–4,000 | **9** | ▇ |
| 4,001–5,000 | 0 | |
| over 5,000 | **0** | |

Median 328 ms, maximum 3,828 — a **12x** spread inside one cell, and bimodal
rather than long-tailed. A mean would have reported about 600 ms and hidden the
question entirely. Nine contenders in 640 (1.4%) passed 3,000 ms; none reached
4,000.

At two cores it is not merely smaller, it is a different shape:

| wait (ms) | contenders |
| --- | ---: |
| 0 | 10 |
| 1–100 | 555 |
| 101–500 | 71 |
| 501–1,000 | 4 |
| over 1,000 | **0** |

Every value above is on the schedule's own grid — `1, 3, 8, 18, 33, 53, 78, 103,
128, 178, 228`, then `228 + 100n` — because the cap is applied against `TOTALS`
rather than a clock. A value off that grid would mean the transcription is wrong.
It also bounds the retry budget: 5,000 ms is **59 handler entries** in one lock
event, and the worst contender observed reached 47 of them — **80% of the retry
budget**.

## The headroom above 64 — ADR-0022 §11's falsifier firing

`references/adr/0022-append-condition-strategy.md:615-616` says to re-open the
busy timeout *"if any run ever reports `busy > 0`"*. **Nothing in the tree has
ever reported a busy count**, so the trigger has been unfalsifiable since it was
written. This sweep is the first thing that can fire it, and it fires.

Debug, every core, three racing rules — the gate's own shape — under **SQLite's
own handler**, so `busy` is the one bit that cannot be an artefact of the
instrument. Five rounds per cell.

| contenders | attempts | busy, by rule | rate |
| ---: | ---: | --- | ---: |
| **64** | 960 | 0, 0, 0 | 0% |
| **96** | 1,440 | 3, 15, 13 | **2.2%** |
| **128** | 1,920 | 82, 105, 75 | **13.6%** |
| **160** | 2,400 | 108, 178, 123 | **17.0%** |

`CONTENDERS` is 64 today, so the headroom above the shipped value is **under
1.5x**.

**And at 64 itself the answer is not "never" — it is "sometimes".** One launch is
one sample of a tail, so the shipped configuration was launched seven times in
this run: once as `headroom-n64` and six more as `rate-n64-r1` … `rate-n64-r6`,
each five rounds of three concurrent rules under SQLite's own handler.

| launch | busy, by rule |
| --- | --- |
| `headroom-n64` | 0, 0, 0 |
| `rate-n64-r1` | 0, 0, 0 |
| **`rate-n64-r2`** | **1, 2**, 0 |
| `rate-n64-r3` | 0, 0, 0 |
| `rate-n64-r4` | 0, 0, 0 |
| `rate-n64-r5` | 0, 0, 0 |
| `rate-n64-r6` | 0, 0, 0 |

**One launch in seven turns the concurrency suite red at the contender count that
ships today, on the fastest machine in the corpus.** In `rate-n64-r2` three
contenders across two rules exhausted the full 5,000 ms; its worst race ran
5,522,382 µs. Every other launch cleared it. That is what a margin of 1.31x on a
skewed distribution looks like from the outside: mostly fine, and occasionally
not.

That is not a slow run. `crates/happenstance-testkit/src/concurrency.rs:263` maps
every error that is not `ConditionViolated` to `Attempt::Failed`, and
`exactly_one_of_n_contenders_commits` panics on it at `:409-414`:

```text
SQLITE_BUSY → AppendError::Store → Attempt::Failed → the rule panics
```

— a red concurrency suite telling an adapter author their store is wrong when
nothing about their store is wrong. `committed` stays at 5 in every row, so the
*semantic* property holds throughout. What fails is liveness, in the one place
CF-33 guarantees the suite cannot diagnose it.

## What none of this shows

Stated here, and again in [`../README.md`](../README.md), because these are the
sentences most likely to be dropped when a figure is quoted.

1. **It is not `happenstance-sqlite`.** The store is
   `experiments/append-condition`'s measurement candidate — same schema, pragmas,
   `BEGIN IMMEDIATE` and guard SQL, one differing function. The adapter holds a
   `Mutex` across its transaction and does more inside it (tag inserts,
   cardinality upserts); neither is reproduced.
2. **Every `wait_ms` figure is a lower bound**, by 2.6x on one measured
   configuration and by an amount that is not calibrated in general.
3. **One host, and a busy one.** The speedup probe read 5.8 of 20 cores. Absolute
   times are inflated; only within-run comparisons are used.
4. **It says nothing quantitative about a 2-vCPU CI container.** An affinity mask
   constrains this process; a small container also has less memory bandwidth, a
   different filesystem and no twenty other agents. The *direction* of the core
   effect is established; its magnitude elsewhere is not. **Storage was never
   varied at all**, and storage is what a busy handler ultimately waits on.
5. **`busy = 0` means "not observed", not "safe".** Each matrix cell is 640
   samples; a one-in-ten-thousand tail event is invisible at that size, and the
   distribution is skewed enough to make one plausible.
6. **One seed, one log size, one tag storage** — 5,000 events, join-table tags.
   The guard's cost grows with the log, and the guard runs inside the lock.
7. **It proposes no value.** Whether `BUSY_TIMEOUT_MS` moves, whether
   `CONTENDERS` moves, and whether the testkit grows a third `Attempt`
   classification are decisions this directory supplies a number to, not ones it
   makes.
