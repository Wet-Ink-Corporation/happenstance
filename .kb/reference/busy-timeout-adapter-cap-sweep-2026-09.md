---
id: kb-reference-busy-timeout-adapter-cap-sweep-001
title: The cap sweep on the adapter that ships — 5,000 ms goes red 7 launches in 8, 15,000 ms 0 in 16
kind: reference
status: accepted
authority_tier: note
summary: >-
  The measurement taken 2026-09-21 from experiments/busy-timeout-margin/results/adapter-cap-sweep.md,
  of what red rate happenstance-sqlite actually has at three candidate values of BUSY_TIMEOUT_MS. It
  is the sibling of kb-reference-busy-timeout-margin-001 and not a re-run of it: that page measured
  how close per-contender waits run to the cap, on experiments/append-condition's measurement
  candidate, with a counting busy handler; this one measures how often the suite therefore goes red,
  on crates/happenstance-sqlite's own concurrency target, unmodified, and reports one bit per launch —
  the bit an adapter author experiences. Running the real adapter is what closes caveat 1 of
  experiments/busy-timeout-margin/README.md, the caveat the sibling atom correctly still carries. At
  --test-threads=1, which serialises the rules so that all 64 contenders get the whole machine and is
  the worst case this host can produce: 5,000 ms went red in 7 launches of 8; 15,000 ms in 0 of 16;
  30,000 ms in 0 of 8. The 5,000 ms cell was run a second time in a parallelism control and went red 8
  of 8, so 15 of 16 between the two runs, and both are reported rather than averaged. The control is
  the other half of that table: at libtest's default parallelism — what cargo xtask ci actually runs —
  5,000 ms went red 1 of 8, the same direction as the sibling's finding that fewer simultaneously-
  runnable contenders measure safer. Raising the cap costs nothing when nothing is contended: passing-
  run target durations were 4.51–5.65 s at 5,000 ms and 4.99–5.40 s at 15,000 ms, indistinguishable on
  a shared host, because the busy handler returns the instant the lock is acquired and the cap bounds
  only the tail. Every failure was a liveness failure — committed was correct in every row, and the
  two rules that went red are the two that require every contender to commit. Conditions, because
  none of it is portable — the same Windows i9-13905H host as every other page in that experiment, not
  the declared-conditions Linux host of kb-decision-0064, debug build as the gate runs it, CONTENDERS
  at the shipped 64, host shared. What this cannot show: one bit per launch discards the margin, a
  launch clearing the cap by 1 ms and one clearing it by 14 s being the same row; one host; and it
  does not show the contended-versus-broken conflation is fixed, because it is not.
depends_on: []
related:
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-reference-one-connection-latency-001
  - kb-decision-0022
  - kb-decision-0065
  - kb-decision-0064
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-testkit-contention-tolerance-001
source_paths:
  - .kb/_intake/2026-09-21-adr-0022-s11-superseded-busy-timeout-is-fifteen-seconds.md
  - experiments/busy-timeout-margin/results/adapter-cap-sweep.md
  - experiments/busy-timeout-margin/README.md
  - experiments/busy-timeout-margin/results/busy-timeout-margin.md
  - crates/happenstance-sqlite/src/connection.rs
  - crates/happenstance-sqlite/tests/concurrency.rs
last_reviewed: 2026-09-21
---

# The cap sweep on the adapter that ships — 5,000 ms goes red 7 launches in 8, 15,000 ms 0 in 16

## What this is a pointer to

The instrument lives at `experiments/busy-timeout-margin/results/adapter-cap-sweep.md`, outside
the workspace and outside the gate. Unlike every other page in that directory, it is not produced
by `run.sh` against `experiments/append-condition`'s measurement candidate — it edits
`crates/happenstance-sqlite/src/connection.rs`'s `BUSY_TIMEOUT_MS` in place, rebuilds, and launches
`crates/happenstance-sqlite`'s own `tests/concurrency.rs` repeatedly, restoring the constant on any
exit including an interrupt. This atom is the citable summary of what it measured. The decision
that used it is `kb-decision-0065`; this atom carries no verdict of its own.

## Why a second instrument, given the sibling already exists

`kb-reference-busy-timeout-margin-001` measured a *margin* — how close a per-contender wait runs to
the cap — using a counting busy handler installed on `experiments/append-condition`'s candidate
store, not the shipped adapter. Its own first caveat says so: the candidate shares schema, pragmas,
`BEGIN IMMEDIATE` and guard SQL with `happenstance-sqlite`, but the adapter holds a `Mutex` across
its transaction and does more inside it, and neither is reproduced. This page closes that caveat by
running the real adapter's own conformance target and giving up the instrumentation in exchange —
no wait vector, no percentiles, one bit per launch: did the suite go red. Read the two together, as
the sweep page itself says: the sibling shows how close the waits run; this shows how often the
suite therefore fails.

## The sweep

At `--test-threads=1` — which serialises the three racing rules so all 64 contenders get the whole
machine simultaneously, the worst case this host can produce, and *not* the gate's own parallelism:

| `BUSY_TIMEOUT_MS` | launches | red | rate |
| ---: | ---: | ---: | ---: |
| 5,000 *(shipped before this)* | 8 | 7 | 88% |
| 15,000 | 16 | 0 | 0% |
| 30,000 | 8 | 0 | 0% |

The 5,000 ms cell was also measured a second time, as the other half of a parallelism control (see
below), and went red 8 of 8 there — 15 red of 16 launches between the two runs of the same cell.
Both numbers are reported rather than averaged into one figure, because a coin that lands heads
most of the time is better shown twice than reconciled once. Nothing downstream turns on which:
15,000 ms cleared 16 of 16 where 5,000 ms cleared 1 of 16 in total.

The rules that go red are exactly the two that require *every* contender to commit —
`positions_are_unique_under_concurrent_appends` on the count, and
`append_returns_the_callers_own_last_position` per contender. `committed` is correct in every row
throughout: nothing here is a semantic failure, only a liveness one.

## The parallelism control

`--test-threads=1` is the worst case this host can produce, not what `cargo xtask ci` runs.
Serialising the rules gives each 64-contender race the whole machine, so all 64 writers run
genuinely simultaneously; letting libtest overlap the three rules spreads them and makes the suite
*less* likely to go red. Both arms measured back-to-back at 5,000 ms, eight launches each:

| parallelism at 5,000 ms | launches | red |
| --- | ---: | ---: |
| `--test-threads=1` | 8 | 8 |
| libtest default (the gate's) | 8 | 1 |

Same direction as `kb-reference-busy-timeout-margin-001`'s core-count finding: fewer simultaneously
runnable contenders measure safer, not the other way a reasonable person might predict.

## What the raise costs when nothing is contended

Nothing measurable. The busy handler returns the instant the lock is acquired, so the cap bounds
only the pathological tail rather than delaying every append. Six launches at the gate's own
parallelism, passing-run target durations: 4.91, 5.47, 5.48, 5.57, 5.65, 4.51 s at 5,000 ms against
4.99, 5.03, 5.23, 5.26, 5.27, 5.40 s at 15,000 ms — indistinguishable on a shared host. What the
raise *is* paid for by: a genuinely stuck writer now reports in 15 s rather than 5 s, a rare path
that still ends in a red rule rather than a hang, because the cap stays finite.

*Erratum, 2026-09-28.* The six durations above run from 4.51 s, not 4.91 s; the
range was misquoted as 4.91–5.65 s in this atom's summary and in
`kb-decision-0065` (lines 29 and 123). The decision atom is accepted and is not
edited in place; the conclusion it draws — raising the cap is free on the healthy
path — does not depend on the lower bound.
## Conditions, and what this cannot show

Windows 11, i9-13905H (14c/20t) — the same host as every other page in this experiment, and
explicitly *not* `kb-decision-0064`'s declared-conditions Linux host. Debug build, as
`cargo xtask ci` runs it. `CONTENDERS` at the shipped 64, `happenstance-sqlite` at `0.3.2`, host
shared with roughly twenty other agent processes, so absolute durations are inflated and only
within-run comparisons are used.

Three things this page cannot show, named rather than assumed away: it is one host; it reports one
bit per launch, so a launch clearing 15,000 ms by 1 ms and one clearing it by 14 s are the same row,
and the margin question stays the sibling atom's, not re-measured at the new cap; and it does not
show that the contended-versus-broken conflation is fixed, because it is not — a contended store and
a broken store are still the same `Attempt`, only met less often.
