# `ops/host/`

The machine this repository measures on, as scripts rather than as somebody's
memory of an afternoon.

## Why this tree exists

`benchmarks/README.md` §"What none of this shows" opens with the problem:

> The wall-clock medians on this host moved by up to 40% between runs while the
> allocation counts stayed identical to the digit — which is why the allocation
> columns are the ones to quote.

That instability is load-bearing rather than cosmetic. It is why
`benchmarks/src/paired.rs` exists at all — ratios from an interleaved paired
runner, absolutes from criterion, never mixed in one table — why
`STABLE_DRIFT_MARGIN` is 0.15, "chosen against the 45% and 4× swings ADR-0022
recorded on this class of host", and why `RUNBOOK.md` says outright that *"any
later benchmark in this repository should assume the same instability."*

The answer this repository had was **design**: interleave the arms, quote ratios,
prefer the allocation counts. That was the right answer to have, and it is not
the whole answer. This tree is the other half — a machine whose conditions are
declared, applied and checked, so that a figure is taken under stated conditions
instead of whatever the laptop was doing.

## What it does not do

**Nothing here can turn a merge red.** No `cargo xtask ci` step, no
`.redkiln/config.yaml` `verify:` command and no CI job invokes any of it, and
`xtask/src/affected.rs`'s INERT list names `ops/` — held by
`the_host_provisioning_tree_selects_no_package` — so a diff here selects no
package at all.

`preflight.sh` asserts, and the distinction that keeps it inside CF-34 is worth
stating precisely. CF-34 (`spec/SPECIFICATION.md:8747`) rejects *a benchmark
result* gating a merge: a threshold on a measured number. Every check in
`preflight.sh` reads a value out of sysfs, systemd or a `--version` **before the
first sample exists**; it can fail when nothing has been measured yet, which is
the property that separates it from a budget. `paired.rs:71-73` draws the same
line from the other side — it computes drift and refuses to threshold it,
"because CF-34 forbids a benchmark result gating a merge, and a drift threshold
would be exactly that."

If you add a check whose input is a number this repository measured, you have
written the thing CF-34 forbids. Put it in the report.

## The files

| | |
| --- | --- |
| `host.env` | **the declared conditions, as data.** Both `cpu-tuning.sh` and `preflight.sh` read it, so a condition cannot be applied and asserted differently |
| `00-system.sh` | root half: packages, sleep/lid masking, scheduled maintenance masked, sysctl, installs the unit. `--restore` undoes it |
| `cpu-tuning.sh` | the unit's payload: governor, EPP, optional frequency cap, SMT, THP, clocksource. `--restore` |
| `happenstance-bench-tuning.service` | `Type=oneshot`, `RemainAfterExit=yes` — which is what makes `systemctl is-active` a question the preflight can ask |
| `10-toolchain.sh` | user half: rustup, nightly, `cargo-hack`, `cargo-deny`, `wasm-bindgen-cli`, the Postgres image |
| `preflight.sh` | the assertions. Reads only |
| `bench.sh` | preflight → `taskset` → `benchmarks/run.sh` → the throttle/thermal aftermath |
| `probes/` | the C programs that settled the clocksource question |

**One script writes, the other reads.** `00-system.sh` and `cpu-tuning.sh` only
change the machine; `preflight.sh` only judges it. A script that did both could
always make its own check pass — which is the argument `benchmarks/README.md`
already makes for `tests/instruments_work.rs` being a separate binary from the
instruments it tests.

## Running it

```console
# once, as root
sudo ./00-system.sh                         # packages, masking, installs the unit

# once, as the build user, after the checkout exists
./10-toolchain.sh ~/src/happenstance        # rustup + the tools the gate probes for

# before every measurement
./preflight.sh --report                     # the table, exit 0 whatever it says
./bench.sh --fast                           # preflight --assert, then the harness
```

`preflight.sh` exits **0 with a banner** on any machine that is not the reference
host — absent `/etc/happenstance-bench-host`, or not Linux. That is deliberate:
`benchmarks/run.sh` is `set -euo pipefail` and runs on contributor laptops, so a
preflight that failed there would break the harness for everyone who is not us.

To undo everything: `sudo ./00-system.sh --restore`. The CPU knobs come back from
a snapshot taken before the first change, never from assumed defaults — a restore
that writes `powersave` and `cpuinfo_max_freq` leaves a machine that passes the
next check while not being what it was.

## The host

Lenovo Legion 5 15ACH6 — a **laptop**, which is most of what the tuning is about.
AMD Ryzen 7 5800H, 8 cores / 16 threads, 15 GiB, Ubuntu 24.04.4, kernel 6.8,
ext4 on LVM on NVMe. Reachable over Tailscale only.

Slower than the machine `benchmarks/results/` was taken on (i9-13905H, 14c/20t)
and with fewer cores. That is the right trade: every figure this repository
quotes is a ratio between arms of one run, and a repeatable clock beats a fast
one for that.

### What is tuned, and why each one

| Knob | Setting | Why |
| --- | --- | --- |
| `scaling_governor` | `performance` | **measured not to matter, and kept anyway** — see below |
| `energy_performance_preference` | `performance` | a second, independent lever on this driver — the governor picks the algorithm, EPP biases it |
| `scaling_max_freq` | **uncapped, for now** | capping to the 3.2 GHz base clock would trade peak throughput for a clock that does not vary with die temperature. Deliberately not done yet: capping before the flakiness is measured makes the measurement unable to say whether the cap was needed |
| SMT | **on, for now** | same argument. It is a second variable and it moves in its own regime |
| THP | `madvise` | `always` changes allocator behaviour, and the allocation counts are the columns to quote |
| clocksource | **`hpet`** | see below. This one is not a preference |
| sleep/lid | masked / ignored | a laptop that suspends at minute 12 loses a 40-minute run |
| apt timers, unattended-upgrades, man-db, fstrim | **masked** | the one interference class that fires on a schedule, and therefore lands entirely inside whichever arm was running. Masked rather than disabled: `apt-get install` re-enables a *disabled* timer as a side effect |

## The governor was the wrong suspect, and that is measured

This tree was provisioned under a plan that named the CPU governor as *"the
largest single noise source"* and led with it. That was wrong, and
[`benchmarks/results/flakiness/FLAKINESS.md`](../../benchmarks/results/flakiness/FLAKINESS.md)
is the measurement that says so: five criterion repetitions under each governor
on this host, 14 arms spanning 31 ns to 2 ms.

| regime | worst between-run spread | worst rMAD |
| --- | --- | --- |
| `powersave`, as found | 1.109× | 2.26% |
| `performance` | 1.145× | 1.87% |

Indistinguishable, with medians within 1% of each other. Criterion warms up
before it measures and the workload keeps the core loaded, so `amd-pstate` ramps
to its ceiling under either setting and the governor never gets a chance to
matter.

**What fixed the flakiness was the teardown** — 21 unrelated containers including
an Ollama inference server on `0.0.0.0:11434` and an Immich ML worker, three cron
jobs, and every scheduled apt timer. Between-run spread on the quiet host is
~1–5% against the 40% `benchmarks/README.md` records for the Windows host.

The governor tuning is kept because it is free and reversible, and because an
instrument that takes discrete samples with idle gaps between them — rather than
a warmed-up criterion batch — might yet care. It is kept **on those terms**. Do
not cite it as the thing that worked.

## The clocksource, which is the expensive thing this tree learned

The kernel refuses this machine's TSC at boot:

```
TSC synchronization [CPU#0 -> CPU#2]:
Measured 11225813293 cycles TSC warp between CPUs, turning off TSC clock.
tsc: Marking TSC unstable due to check_tsc_sync_source failed
```

and falls back to `hpet`, where `clock_gettime(CLOCK_MONOTONIC)` leaves the vDSO
fast path and costs a **measured 1,390 ns against 19 ns on tsc**. A 73× tax on
the timer itself, on a host acquired to make measurement better.

That looked like a false positive, and the argument was decent: 11.2e9 cycles is
3.5 seconds of apparent skew between two cores of one die, which is not a
physical quantity, and the CPU advertises both `constant_tsc` and `nonstop_tsc`.
So `tsc=reliable` went on the kernel command line and the machine was rebooted.

**The argument was wrong, and `probes/tsc-migrate.c` is what said so.** It took
three attempts to build an instrument worth believing:

* `probes/tsc-coherence.c` — 16 threads against a shared high-water mark.
  Reported 154,588,122 violations with a 2 ms worst case. The read and the
  compare were separable, so anything scheduled out between them reported its own
  latency as a warp.
* `probes/tsc-pairwise.c` — a ping-pong handshake, and the first version with an
  **SMT-sibling control**: CPU0 and CPU1 are one physical core reading one
  counter, so a backwards read there condemns the instrument. It reported
  299,999 out of 300,000 on that pair. Control fired; result discarded.
* `probes/tsc-migrate.c` — one thread, migrating, comparing consecutive reads.
  No lock, no baton, no shared mutable state, no memory ordering to get wrong.

The third one, same binary and same host:

| clocksource | backwards reads per 1M migrations | worst backstep |
| --- | --- | --- |
| `tsc`, with `tsc=reliable` | 62,493 | 2.0 ms |
| `hpet` | **0** (in 500,000) | 0 ns |

That A/B is what makes the number evidence rather than an artefact — a broken
instrument would have failed both columns.

The proportions say what the shape is. Cycling all 16 CPUs gives 6.25%, which is
1/16: the wrap hop. Four physical cores gives 25%, which is 1/4. **One physical
core gives 50%** — CPU0 and CPU1 disagree by 6.46 ms. So each logical CPU carries
a fixed TSC offset, one direction of every hop is always backwards, and confining
a measurement to a single core does not escape it.

And the mechanism is closed rather than merely suspected: **`tsc_adjust` is not
in `/proc/cpuinfo` on this part.** That is the MSR Linux uses to correct per-CPU
TSC offsets, and without it no kernel parameter can fix this — only firmware.
`tsc=reliable` is therefore removed from `GRUB_CMDLINE_LINUX_DEFAULT`, the
kernel's own check is the standing protection, and `CLOCKSOURCE=hpet` in
`host.env` is the second line.

### What hpet costs, stated rather than hidden

At a 1,390 ns timer and `paired.rs`'s `TIMER_HEADROOM` of 10, arms below roughly
14 µs are timer-dominated. Of the seven arms in `benchmarks/results/raw/overhead.log`
exactly **one** is: `memory`, at a 5.8 µs median. The other six sit between
143 µs and 5.5 ms, where 1.4 µs is under 1%. The criterion targets batch
iterations and amortise it. The allocation counts — which `benchmarks/README.md`
already names as the columns to quote — read no clock at all.

### And it costs the paired runner entirely, which the arithmetic above missed

The estimate above — one arm affected — was too kind, and the harness's own
control is what said so. On `hpet`,
`benchmarks/tests/instruments_work.rs`'s `the_paired_sampler_sees_a_difference_it_was_given`
**fails**: both arms come back at an identical 4,679 ns median, both labelled
`TIMER-DOMINATED`. The paired sampler cannot see a difference it was handed, and
it is the source of every *ratio* in `benchmarks/README.md`'s headline.

So the harness splits in two on this machine:

| instrument | on `hpet` | why |
| --- | --- | --- |
| criterion targets | **usable** — resolves 82 ns at ±0.3% | batches iterations, amortising the clock |
| allocation counts | **usable** | read no clock at all |
| the paired runner | **not usable** | its own control fails |

**A regime that would repair it, not yet taken.** The TSC skew is a *fixed
per-CPU offset*, and Linux CPU affinity is inherited by child threads — so a
single-CPU mask makes a cross-CPU read unreachable. Measured, with `tsc`
selectable: pinned to one logical CPU, `tsc-migrate` finds **0** backwards reads
in 500,000 and the timer costs 20 ns; unpinned on the same boot, 31,250. And the
paired runner is already single-threaded by design — `benchmarks/README.md` says
its contention is "single-thread interleaved, not OS-thread" — so a single-CPU
mask costs it nothing it has.

What that needs is a boot with `tsc=reliable` **and** every measurement pinned to
one logical CPU, with the hazard documented for anything else on the machine that
reads a clock. Switching `current_clocksource` to `tsc` at runtime is **not** a
substitute and was tried: the kernel cleared TSC's high-resolution flag when it
marked it unstable, so the vDSO path is gone and `CLOCK_MONOTONIC` comes back at
millisecond granularity — the arms report medians of `0ns` and tails on
1,000,447 ns boundaries. Fast to call, useless to read.

That trade is not this tree's to make silently, so it is written down here rather
than taken.

`benchmarks/src/cpu.rs` is the one place this is a straight gain rather than a
trade: its `Utilisation` ratio is documented as unquotable on short arms because
"on Windows the accounting granularity is around 15.6 ms", and Linux's
`CLOCK_PROCESS_CPUTIME_ID` has no such floor.
