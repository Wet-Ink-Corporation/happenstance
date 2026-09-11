---
id: kb-reference-host-clocksource-tsc-hpet-001
title: The measurement host's TSC skews by a fixed per-CPU offset, and hpet costs its timer 73x
kind: reference
status: accepted
authority_tier: note
summary: >-
  Measured 2026-09 on the dedicated Linux measurement host (ops/host/README.md):
  the kernel refuses the TSC at boot ("Measured 11225813293 cycles TSC warp
  between CPUs") and falls back to hpet, where clock_gettime(CLOCK_MONOTONIC)
  costs 1,390 ns against 19 ns on tsc — a 73x tax. Overriding with tsc=reliable
  was tried and refuted by the third of three instruments
  (ops/host/probes/tsc-migrate.c, one migrating thread comparing consecutive
  reads): 62,493 backwards reads per million migrations on tsc, worst 2.0 ms,
  versus 0 in 500,000 on hpet. Proportions 1/16 on 16 CPUs, 1/4 on four cores,
  1/2 on one physical core (CPU0 and CPU1 disagree by 6.46 ms): a fixed
  per-CPU offset, so pinning to one core does not escape it. tsc_adjust is
  absent from /proc/cpuinfo on this part, so no kernel parameter can fix it.
  On hpet the paired runner's own control fails (both arms 4,679 ns, both
  TIMER-DOMINATED), so criterion targets and allocation counts stay usable on
  this host and the ratio half of the harness is not; a pinned-single-CPU
  regime that would repair it is measured (0 in 500,000, 20 ns) and not taken.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-playbook-control-fires-on-instrument-001
  - kb-decision-0064
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/host.env
  - ops/host/probes/tsc-coherence.c
  - ops/host/probes/tsc-pairwise.c
  - ops/host/probes/tsc-migrate.c
  - benchmarks/src/paired.rs
  - benchmarks/tests/instruments_work.rs
last_reviewed: 2026-09-11
---

# The measurement host's TSC skews by a fixed per-CPU offset, and hpet costs its timer 73x

## What was measured

The dedicated Linux measurement host acquired to make benchmark results more
trustworthy boots with its TSC disabled: the kernel logs *"Measured
11225813293 cycles TSC warp between CPUs, turning off TSC clock"* and falls
back to `hpet`. On `hpet`, `clock_gettime(CLOCK_MONOTONIC)` leaves the vDSO
fast path — a measured **1,390 ns** per call against **19 ns** on `tsc`, a
73x tax on every timer read, on a host whose whole purpose is measurement.

The part advertises `constant_tsc` and `nonstop_tsc`, and 11.2e9 cycles is
only 3.5 seconds of apparent skew across the whole boot — not obviously a
physical quantity — so `tsc=reliable` was tried on the kernel command line.

## Why it took three instruments

The first two probes failed their own controls before either answer could be
trusted. A 16-thread race against a shared high-water mark reported
154,588,122 "violations" — its read and its compare were separable, so any
thread scheduled out between them reported its own latency as a clock warp.
A ping-pong handshake between CPU0 and CPU1 — SMT siblings sharing one
physical core, so two threads there reading one counter cannot disagree —
reported 299,999 of 300,000 backwards reads on that pair and was discarded on
its own evidence: the instrument, not the clock, was still wrong.

The third probe, `ops/host/probes/tsc-migrate.c`, has nothing left to get
wrong: one thread, migrating between CPUs, comparing consecutive reads, no
lock, no shared mutable state. Same binary, same host, per million
migrations: **62,493 backwards reads on `tsc`** (worst case 2.0 ms) against
**0 in 500,000 on `hpet`**. That A/B is what makes the finding evidence
rather than artefact — a broken instrument would have failed both columns,
and this one failed only the column that was actually wrong.

## The shape of the offset

Proportions scale with how many CPUs share the race: 16 CPUs → 1/16 (the
wrap hop), four cores → 1/4, one physical core → 1/2, with CPU0 and CPU1
disagreeing by 6.46 ms on that last case. The offset is fixed per CPU, so
confining a measurement to a single core does not escape it — pinning
changes which two clocks disagree, not whether they do. `tsc_adjust`, the
MSR Linux uses to correct exactly this, is absent from `/proc/cpuinfo` on
this part: the mechanism that would let software repair it does not exist
here, so only firmware could.

## What this leaves the harness with

`benchmarks/tests/instruments_work.rs`'s own paired-sampler control —
`the_paired_sampler_sees_a_difference_it_was_given` — fails on this host
under `hpet`: both arms measure 4,679 ns and both are reported
`TIMER-DOMINATED`, because the timer's own resolution swamps the difference
the control is designed to detect. Criterion targets and allocation counts,
which do not depend on wall-clock resolution at this grain, stay usable; the
ratio half of `benchmarks/src/paired.rs` does not, on this host, as
configured. A pinned-single-CPU regime was measured as a repair — 0 in
500,000 backwards reads, 20 ns per call — and is not taken as part of this
finding; `benchmarks/results/` stays sourced from the Windows host regardless
(kb-decision-0064).
