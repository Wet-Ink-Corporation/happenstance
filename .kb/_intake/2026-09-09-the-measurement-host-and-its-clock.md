# The measurement host is a machine with declared conditions, and its clock is `hpet`

**Date:** 2026-09-09
**Kind:** decision record, staged for `/redkiln:kb-ingest`
**Evidence:** `ops/host/`, `ops/host/probes/`, `ops/host/README.md`
**Source paths:** `ops/host/*`, `xtask/src/affected.rs`, `benchmarks/run.sh`,
`benchmarks/README.md` §Conditions

## Why this is a brief and not the atom

`.kb/decisions/` atoms are authored by `/redkiln:kb-ingest` from staged
documents. Hand-writing one produces the directory layout of the process without
the process, which is why the first attempt at that here was reverted
(`0269720`).

Two atoms are probably owed rather than one, and the ingest should adjudicate:
the **decision** (a declared-conditions host, and what it may not be wired to),
and a **reference** atom for the clocksource finding, which is a fact about one
machine rather than a policy.

## The problem, and that this is its third recorded occurrence

Benchmark instability in this repository is not an impression. It is written down
three times, independently, before any of this work:

* `benchmarks/README.md` §"What none of this shows" — *"The wall-clock medians on
  this host moved by up to 40% between runs while the allocation counts stayed
  identical to the digit."*
* `RUNBOOK.md` — the sequential design *"failed: the baseline moved 2.7× at one
  client and 3.0× at 64, larger than two of the three effects"*, closing with
  *"any later benchmark in this repository should assume the same instability."*
* `references/adr/0022-append-condition-strategy.md:120-144` — *"on a shared
  developer machine two runs an hour apart disagreed by up to 45%, and one arm's
  unconditional append — a path no strategy participates in — varied by 4× between
  slots."*

Three independent instances is what makes this a decision rather than a
preference. The repository's existing answer was **design**: interleave the arms
(`benchmarks/src/paired.rs`), quote ratios not absolutes, prefer the allocation
counts. That answer is right and it is not the whole answer, because it makes the
numbers *comparable* without making the machine *quiet*.

## The decision

A dedicated Linux host is the reference machine for benchmarks and for
build/test. Its conditions are **data** in `ops/host/host.env`, applied by a
systemd unit, and asserted by `ops/host/preflight.sh` — rather than prose in a
README and a ritual in somebody's shell history.

Two properties are load-bearing:

**One script writes, the other reads.** `00-system.sh` and `cpu-tuning.sh` only
change the machine; `preflight.sh` only judges it. A script that did both could
always make its own check pass — the argument `benchmarks/README.md` already
makes for `tests/instruments_work.rs` being separate from the instruments.

**`--restore` restores captured values, never assumed defaults.** The first
`--apply` snapshots what it is about to change and never overwrites the snapshot.
A restore that writes `powersave` and `cpuinfo_max_freq` back leaves a machine
that passes the next check while not being what it was.

## The CF-34 boundary, which is the clause a future reader will re-litigate

CF-34 (`spec/SPECIFICATION.md:8747`) rejects *a benchmark result* gating a merge.
`preflight.sh` asserts on the **environment**, and every value it reads comes out
of sysfs, systemd or a `--version` **before the first sample exists** — it can
fail when nothing has been measured, which is the property that separates it from
a budget. `paired.rs:71-73` draws the same line from the other side: it computes
drift and refuses to threshold it, *"because CF-34 forbids a benchmark result
gating a merge, and a drift threshold would be exactly that."*

Structurally: nothing in `xtask/src/main.rs`'s step table, `.redkiln/config.yaml`'s
`verify:` block or `.github/workflows/` invokes any of it, and `ops/` is on
`is_inert`'s INERT list, held by `the_host_provisioning_tree_selects_no_package`.

**The residual risk, named:** if this host is later registered as a self-hosted CI
runner, the preflight becomes reachable from a merge-blocking job and this
paragraph stops being true. That is a decision, not a configuration change.

## The clocksource, and the part worth carrying whole

This is the finding, and its value is mostly in how it was got wrong first.

The kernel refuses this machine's TSC at boot — *"Measured 11225813293 cycles TSC
warp between CPUs, turning off TSC clock"* — and falls back to `hpet`, where
`clock_gettime(CLOCK_MONOTONIC)` leaves the vDSO fast path and costs a **measured
1,390 ns against 19 ns on tsc**. A 73× tax on the timer, on a host acquired to
make measurement better.

The argument for overriding it was decent and wrong: 11.2e9 cycles is 3.5 seconds
of apparent skew between two cores of one die, which is not a physical quantity,
and the part advertises `constant_tsc` and `nonstop_tsc`. `tsc=reliable` went on
the kernel command line.

**Three instruments were needed, and the first two failed their own controls.**

1. 16 threads against a shared high-water mark — 154,588,122 "violations". The
   read and the compare were separable, so anything scheduled out between them
   reported its own latency as a warp.
2. A ping-pong handshake, and the first version carrying the control that mattered:
   CPU0 and CPU1 are SMT siblings, one physical core reading one counter, so a
   backwards read *there* condemns the instrument. It reported 299,999 of 300,000
   on that pair. Result discarded.
3. `ops/host/probes/tsc-migrate.c` — one thread, migrating, comparing consecutive
   reads. No lock, no shared mutable state, no memory ordering to get wrong.

Same binary, same host, per million migrations: **62,493 backwards reads on `tsc`
(worst 2.0 ms), and 0 in 500,000 on `hpet`.** That A/B is what makes it evidence
rather than an artefact — a broken instrument would have failed both columns.

The proportions give the shape: 16 CPUs → 6.25% (1/16, the wrap hop), four cores
→ 25%, **one physical core → 50%**, with CPU0 and CPU1 disagreeing by 6.46 ms. A
fixed per-CPU offset, so confining a measurement to one core does not escape it.

And the mechanism is closed rather than suspected: **`tsc_adjust` is not in
`/proc/cpuinfo` on this part** — the MSR Linux uses to correct per-CPU TSC
offsets. No kernel parameter can fix this; only firmware could.

**The transferable lesson is not about clocks.** It is that a control which can
only fire on a machine fault is worth less than one which can fire on the
instrument. The SMT-sibling row exists because two threads of one core reading
one counter *cannot* disagree — and it is what caught both wrong answers before
either was believed. `benchmarks/tests/controls_fire.rs` already encodes this
habit for the harness; the same habit was needed one layer down, and was not
there until it was needed twice.

## What is not decided

* **The frequency cap and SMT stay at their as-found settings** (uncapped, SMT
  on). Capping the clock and halving the machine before the flakiness is measured
  makes the measurement unable to attribute the improvement, and "we changed five
  things and it got better" is the shape of finding this repository rejects.
* **`benchmarks/results/` is not re-taken.** Every figure there was produced on
  the Windows host in `README.md` §Conditions, and four `[PROVISIONAL]` clauses
  and two doc comments reason from those numbers. The host is made capable of
  re-taking them and stops there.
* **`RESOLVABLE_FLOOR` in `benchmarks/src/cpu.rs`** is documented against
  Windows's ~15.6 ms scheduler accounting granularity, which Linux's
  `CLOCK_PROCESS_CPUTIME_ID` does not have. Lowering it is a real change to a
  published caveat and wants its own measurement.
* **Comparing a Windows absolute with a Linux absolute stays forbidden.** Only
  same-run ratios cross the boundary.

## Note to the ingest

`.kb/_intake` is outside the citation scan (`xtask/src/lints.rs:2007`), and
`.kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md` records exactly
that failure. Every `path:line` above is promoted into `.kb/decisions/`, which
**is** scanned — re-resolve each against `HEAD` at ingest or the wave reddens the
gate.
