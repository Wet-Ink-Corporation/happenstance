# Between-run spread, measured — and what turned out not to cause it

`README.md`'s [What none of this shows](../../README.md#what-none-of-this-shows)
has always opened with this:

> **One machine, one run per cell.** … The wall-clock medians on this host moved
> by up to 40% between runs while the allocation counts stayed identical to the
> digit — which is why the allocation columns are the ones to quote.

It is the most consequential sentence in that file. `benchmarks/src/paired.rs`
exists because of it; `STABLE_DRIFT_MARGIN` is 0.15 because of it; `RUNBOOK.md`
tells every later benchmark to *"assume the same instability"*; and four
`[PROVISIONAL]` clauses name the measurement it makes impossible.

And it had never had an instrument behind it, for a mundane reason: `run.sh`
tee'd every artefact to a fixed filename, so the evidence for run N was destroyed
by run N+1. `RESULTS_SUFFIX` (`run.sh:66`) fixes that. This is the first
measurement of the thing.

**Nothing here is a threshold and nothing here gates anything.** CF-34
(`spec/SPECIFICATION.md:8747`). The numbers are printed and a human reads them.

## What was run

Host B — `britton-ai`, the dedicated measurement host
([`ops/host/README.md`](../../../ops/host/README.md)), after 21 unrelated
containers, three cron jobs and every scheduled apt timer were removed from it.

Two regimes, five runs each, `taskset -c 0,2,4,6` (one thread per physical core),
`cargo bench --bench typed_codec -- --warm-up-time 1 --measurement-time 3`:

* **A — `powersave`**, the governor as found. `amd-pstate-epp`, 400 MHz–4.46 GHz.
* **B — `performance`**, governor *and* `energy_performance_preference`, applied
  by `happenstance-bench-tuning.service`.

Regime A was produced by `systemctl stop happenstance-bench-tuning`, which
restores the governor from the snapshot the unit took before its first change —
so A is the machine as found, not an approximation of it.

`criterion` rather than the paired runner, and the reason given here at the time
was wrong. It said the paired runner "does not work at all" on this host, because
`the_paired_sampler_sees_a_difference_it_was_given` was failing. The ratio
assertion in that control was in fact **passing**, at 6.44×; what failed was
`is_above_the_timer()`, a self-check on the control's own arm sizes, which were
hard-coded for a host whose timer costs 56 ns against this one's 2,924 ns. The
control has since been calibrated against the measured timer and both hosts pass.
`benchmarks/README.md` carries the whole correction.

So criterion here is a choice rather than a necessity. It remains the right one
for *this* measurement: the question is between-run dispersion of many arms, and
criterion's batching makes even the 1.4 µs arms reproduce to 1.003×, where the
paired runner's 29 µs floor on this host would put a third of them under it.
Nothing below depends on the wrong reason; the runs were criterion runs either
way.

## The result

Full table in [`comparison.txt`](comparison.txt); the raw estimates are the
twenty `typed_codec-*.txt` files beside it. Summary, over 14 arms × 5 runs:

| | worst spread (max/min) | worst rMAD | typical spread |
| --- | --- | --- | --- |
| **A — powersave** | 1.109× | 2.26% | 1.01–1.05× |
| **B — performance** | 1.145× | 1.87% | 1.01–1.03× |
| *Host A, Windows, from `README.md`* | *~1.4×* | *—* | *—* |

## Two conclusions, and the second is the one worth having

**The instability was real, and it is gone.** Between-run spread on a quiet host
is ~1–5%, worst case 14.5%, against the 40% `README.md` records. That is the
result the host was acquired for.

**The CPU governor did not cause it, and tuning the governor did not fix it.**
A and B are indistinguishable — the `faster` column runs 0.98× to 1.01×, and
regime A is marginally *tighter* at the worst case than regime B. Criterion warms
up before it measures and the workload keeps the core loaded, so `amd-pstate`
ramps to its ceiling under either setting and the governor never gets a chance to
matter.

That is a falsification, and it falsifies this work's own opening argument. The
plan this host was provisioned under named the governor as *"the largest single
noise source"* and led with it. Measured: it is not a noise source on this
workload at all. **What fixed the flakiness was removing the other 21 containers**
— an Ollama inference server on `0.0.0.0:11434`, an Immich ML worker, a
six-container note stack and a three-minute git-sync cron — not any CPU knob.

The governor tuning is kept, because it is free, reversible and may yet matter
for an instrument that takes discrete samples with idle gaps between them rather
than a warmed-up batch. But it is kept **on those terms**, and this file is here
so nobody later cites it as the thing that worked.

## Is the paired runner still needed here? No — and that settles the clocksource

The paired runner exists for one recorded reason. `RUNBOOK.md`:

> the sequential design this criterion specified — arms first, baseline re-run
> last to bound drift — **failed**: the baseline moved 2.7× at one client and
> 3.0× at 64, larger than two of the three effects.

That is why `README.md` rules *"absolutes from criterion, ratios from the paired
runner, never mixed in one table"*. The rule is a consequence of a measurement,
so another measurement can falsify it — and on this host it does.

**`store_append`, three runs at criterion defaults, 82 arms matched by name**
(`store_append-named-{1,2,3}.tsv`, analysis in
[`sequential-drift.txt`](sequential-drift.txt),
tool at `ops/host/sequential-drift.py`):

| statistic | median | p90 | p99 | worst |
| --- | --- | --- | --- | --- |
| per-arm spread across runs | 1.043× | 1.083× | — | 1.356× |
| **arm-to-arm ratio spread** | **1.059×** | 1.106× | 1.395× | **1.485×** |

**Of 3,321 arm pairs, zero moved more than 2.7×.** The worst pair in the suite is
1.485×. Sequential drift of the kind the discarded pass recorded does not happen
on this host.

The earlier `typed_codec` runs say the same thing from another angle. Grouping
arm pairs by how far apart they sit in criterion's sequence — distance 1, 3, 6,
9, 13 — gives ratio spreads of 1.031×, 1.028×, 1.033×, 1.030×, 1.044×. **There
is no gradient.** If the machine were moving underneath the run, the pairs
furthest apart in time would carry it and the adjacent pairs would not.

### The floor does not apply to criterion, and that was my error

`paired.rs`'s `TIMER_HEADROOM` of 10 puts a "timer-dominated" floor at ~14 µs
against `hpet`'s 1,390 ns clock, and it was reasonable to expect the 25 arms
below that floor to be the noisy ones. Measured, they are the **quiet** ones:

| | arms | median spread |
| --- | ---: | ---: |
| below the ~14 µs floor | 25 of 82 | **1.012×** |
| above it | 57 of 82 | 1.063× |

An arm at 1.43 µs reproduces to 1.003×. The floor is a **`paired.rs` concept and
not a criterion one**, because criterion does not read the clock once per
operation — it batches iterations between two reads and divides, which is what
`README.md`'s own `TIMER-DOMINATED` note tells the reader to do. The residual
noise in this suite is the SQLite arms doing real I/O, not the clock.

### Therefore

**Keep `hpet`. The clocksource does not need fixing.** Criterion works at every
scale this suite reaches, from 1.43 µs to 5 ms, and reproduces ratios to well
inside the 20% resolution `README.md` already claims for itself. And the paired
runner works too, once three host assumptions were taken out of the harness
(`benchmarks/README.md` lists them): a full `run.sh` completes here, and its
headline ratios reproduce against Host A at **1.66× / 4.05× / 5.53×** where Host
A read 1.7× / 3.9× / 6.2×.

What `hpet` costs is one arm. At a 29 µs floor the `memory` append at 5,727 ns is
timer-dominated and reported as such; the other six clear it.

Two things that follow, and neither is silent:

* Re-taking `results/GRADES.md` on this host would **change instrument** for
  §1's ratios, from the paired runner to criterion. That is a documented
  substitution, not a free one.
* The paired runner still earns its keep on a *busy* host, which is what it was
  built for. Nothing here retires it, and it runs here.

## What this does not show

1. **One benchmark, five runs, one host.** `typed_codec`'s 14 arms span 31 ns to
   2 ms, which is a useful spread, but it is one target. The SQLite arms touch a
   filesystem and could behave differently.
2. **Criterion only.** The paired runner is the source of every *ratio* in
   `README.md`'s headline and it is unusable here. A regime that repairs it — a
   boot with `tsc=reliable` plus single-CPU pinning, which `ops/host/README.md`
   analyses — is not measured in this table.
3. **It is not a comparison with Host A.** The 40% row above is quoted from
   `README.md`, taken on different hardware under a different OS with a different
   clock. Cross-host absolutes are forbidden here for the usual reason; the two
   numbers are the same *statistic* about different machines, not a ratio.
4. **Five runs, not ten** for the governor comparison, and **three** for the
   sequential-drift one. Enough to see a 40%-to-5% change and to rule out a
   2.7× drift by two orders of magnitude. Not enough to separate 1.109× from
   1.145×, which is exactly why the governor paragraph says the two regimes are
   indistinguishable rather than that A is better.
5. **One target for the drift test.** `store_append` is the heaviest and most
   I/O-bound of the seven, which makes it the right one to try first; it is
   still one. The `typed_codec` runs agree, and they are the other end of the
   weight range.
