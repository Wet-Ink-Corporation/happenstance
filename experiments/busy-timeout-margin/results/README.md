# Results

Written by hand from [`raw/`](raw/), which is one `./run.sh`. Conditions —
machine, OS, filesystem, toolchain, build profile, the core-constraint method and
how it was verified — are in [`../README.md`](../README.md#conditions), and every
row of `raw/` carries its settings string beside it: the affinity mask read off
the live process, the pragmas off the live connection, the handler and the cap.

**Conformance ran before any clock.** `raw/conformance-counting.txt` and
`raw/conformance-default.txt`: **178 tests each — 89 conformance rules × 2 arms —
`178 passed; 0 failed`**, once under this crate's counting busy handler and once
under SQLite's own. Both are needed because this crate *replaces the store's busy
handler*, on the write path, in the exact place contention resolves — a handler
that gave up one entry early, or never gave up, would move every number here and
would look in a timing table simply like a fast one. `raw/handler-unit.txt` adds
the two unit tests that check the transcription itself: that `TOTALS` is the
prefix sum of `DELAYS`, and that the schedule reaches the cap in a bounded 59
entries, which is what lets `run.sh` promise to terminate unattended.

| page | what is on it |
| --- | --- |
| [`busy-timeout-margin.md`](busy-timeout-margin.md) | the control against ADR-0022 §11, what the instrument costs, the five-point core sweep, the per-contender distribution, and the contender sweep where ADR-0022's falsifier fires |
| [`lost-wakeup.md`](lost-wakeup.md) | M-1's nested `block_on` probe — the other way a contended run stops with no diagnostic |

`raw/waits/` holds one file per cell: every contender's own maximum busy-handler
wait, one per line, 640 lines for a 64-contender cell. The printed rows carry
four percentiles; the distribution is what the question is actually about, so the
whole vector is kept and the histograms are built from it by hand.

## The question, and the answer

> In the configuration the gate actually runs — debug, `--test-threads` at its
> default so the three `CONTENDERS = 64` rules overlap — what is the worst
> per-contender wait inside SQLite's busy handler, and how few cores does it take
> to exceed 5,000 ms?

**3,628 ms of the 5,000 ms budget — a margin of 1.38x — and no core count exceeds
the cap, because removing cores makes it better, not worse.**

The gate runs unconstrained, so its own configuration is the 20-core row (3,628
ms, 1.38x); the worst cell anywhere in the sweep is the 8-core row (3,828 ms,
1.31x), and the two are not separable on this host. Call it **1.3x–1.4x on the
plateau**.

The cap is reached on the *contender* axis instead, and it is close: at 96
contenders SQLite's own handler gives up on 2.2% of attempts. At the shipped 64,
one launch in seven already does.

| cores | worst per-contender wait | margin |
| ---: | ---: | ---: |
| 1 | 8 ms | 625x |
| 2 | 628 ms | 8.0x |
| 4 | 2,628 ms | 1.9x |
| 8 | 3,828 ms | 1.31x |
| **20 — the gate's own** | **3,628 ms** | **1.38x** |
| 20, the arm that ships | 3,728 ms | 1.34x |

## Verdict per finding

| finding | verdict | the number |
| --- | --- | --- |
| **W.W-1** — 5,000 ms was measured on a 20-core i9 in `--release`; the gate runs the same rules in debug with default parallelism | **confirms the gap, refutes the mechanism** | the gap is real and now measured; but debug is inside release's noise (3,628 vs 3,228 ms) and **two cores is 8.0x safer than twenty**, so neither half of the proposed cause survives. The proposed instrument, `taskset -c 0,1`, would have measured the safest cell in the matrix |
| **J.J-1** — the margin is 1.43x, inferred from race wall time under the most favourable configuration in the corpus | **confirms, and sharpens the currency** | **1.38x** in the gate's own configuration (1.31x at the sweep's worst), measured as the unluckiest contender's own budget rather than inferred from a race. And the counting handler is optimistic by ~2.6x, so that is a floor |
| **L2.L2-02** — a transient failure is indistinguishable from a semantic violation, and ADR-0022 §11's re-open trigger has no instrument that can fire | **confirms, and fires it** | `busy > 0` for the first time in the tree: **2.2%** of attempts at 96 contenders, 13.6% at 128, 17.0% at 160 — and **one launch in seven** (`rate-n64-r2`) at the shipped `CONTENDERS = 64`. Each one is `Attempt::Failed` and a panicking rule |
| **M.M1** — `block_on` is a bare park-loop with no notified flag, and the testkit nests it inside itself | **confirms, deterministically** | the nested-collision case hangs past a 10 s deadline on every run, with a baseline and a no-collision control that both complete |

### And one thing nobody asked for

**The shipped strategy is not cheaper under contention.** `monotonic-guard` —
what ADR-0022 §4 chose and `happenstance-sqlite` installs — reached 3,728 ms
against `begin-immediate-probe`'s 3,628 ms in the same configuration. The two are
indistinguishable here. **The margin is a property of the herd, not of the
guard**, so no choice of append-condition strategy buys headroom back.

## What none of this shows

Stated once here and again on each page, because a reader who takes only the
table will otherwise assume them away.

1. **It is not `happenstance-sqlite`.** The store is
   `experiments/append-condition`'s measurement candidate — same schema, pragmas,
   `BEGIN IMMEDIATE` and guard SQL, one differing function (`configure`, which
   chooses the handler). The adapter holds a `Mutex` across its transaction and
   does strictly more inside it; neither is reproduced.
2. **Every `wait_ms` figure is a lower bound.** The counting handler raced 2.6x
   faster than SQLite's own at the identical configuration, because Windows
   serves `std::thread::sleep` on a high-resolution timer and `Sleep()` at ~15.6
   ms granularity. Enough to say the figures are optimistic; not enough to say by
   how much.
3. **One host, and a loaded one.** The in-run speedup probe read 5.8 cores of
   throughput available out of 20. Absolute times are inflated — the release
   control lands 1.36x above ADR-0022's recorded row — so only within-run
   comparisons are used.
4. **Nothing quantitative about a 2-vCPU CI container.** An affinity mask
   constrains this process; a small container also has less memory bandwidth, a
   different filesystem, and no twenty other agents. The *direction* of the core
   effect is established; its magnitude elsewhere is not. **Storage was never
   varied**, and storage is what a busy handler ultimately waits on.
5. **`busy = 0` means "not observed", not "safe".** A matrix cell is 640 samples.
   A one-in-ten-thousand tail event is invisible at that size, and the
   distribution is skewed enough to make one plausible.
6. **This is an experiment, never a gate step** (CF-34). It is not a workspace
   member, `cargo xtask ci` cannot see it, and `run.sh` must never be wired to
   anything that can turn a merge red — which matters more here than usual,
   because this harness is *designed* to fail.
