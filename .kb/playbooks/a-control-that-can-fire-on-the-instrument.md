---
id: kb-playbook-control-fires-on-instrument-001
title: A control that can fire on the instrument, not only on the fault
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A control that can only fire on a machine fault is worth less than one that
  can fire on the instrument itself. Learned building three TSC probes on the
  measurement host (ops/host/probes/): the first — sixteen threads against a
  shared high-water mark — reported 154,588,122 "violations" because its read
  and its compare were separable and anything scheduled out between them
  reported its own latency as a warp; the second — a ping-pong handshake —
  carried the control that mattered, an SMT-sibling row (CPU0 and CPU1 are one
  physical core reading one counter and cannot disagree), reported 299,999 of
  300,000 on that pair, and was discarded on its own evidence; the third — one
  thread, migrating, comparing consecutive reads, no lock, no shared state —
  had nothing to get wrong, and its tsc/hpet A/B separated the columns. The
  method: before believing a probe, find a row whose outcome the physics
  forces and check the probe agrees; a row that merely usually passes is a
  threshold, not a control.
depends_on: []
related:
  - kb-reference-host-clocksource-tsc-hpet-001
  - kb-decision-0010
  - kb-decision-0022
  - kb-playbook-assert-execution-not-discovery-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0064
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/probes/tsc-coherence.c
  - ops/host/probes/tsc-pairwise.c
  - ops/host/probes/tsc-migrate.c
  - benchmarks/tests/controls_fire.rs
  - benchmarks/tests/instruments_work.rs
last_reviewed: 2026-09-11
---

# A control that can fire on the instrument, not only on the fault

## The claim

A control built to catch a fault in the system under test is only doing half
its job if the only way it can fail is that fault occurring. The other half —
easy to skip, because it costs nothing when the instrument happens to be
right — is a row whose outcome is forced by something other than the system
under test, so a broken instrument fails it regardless of whether the fault
it was built to catch ever shows up. Without that row, a control that always
passes and a control that has never been exercised look identical.

This is not about clocks. It came out of building three probes for one clock
question, but the method transfers to any control: benchmark harnesses,
concurrency assertions, conformance rules — anywhere a check is trusted to
say "the thing I'm watching is fine."

## The measured instance

Three probes were built to settle whether the measurement host's TSC could be
trusted after overriding the kernel's refusal to use it
(`kb-reference-host-clocksource-tsc-hpet-001`).

The first — sixteen threads racing against a shared high-water mark in
`ops/host/probes/tsc-coherence.c`'s ancestor — reported 154,588,122
"violations." It had no forced row: its read of the mark and its compare
against a thread's own last-seen value were two separate operations, so
any thread scheduled out between them would report its own descheduling
latency as a backwards clock read. The number was real; the conclusion it
implied was not.

The second, a ping-pong handshake, is the version that mattered:
`ops/host/probes/tsc-pairwise.c` pairs CPU0 and CPU1, which on this part are
SMT siblings — two hardware threads of one physical core, reading one
physical counter. Two reads of one counter from siblings of one core cannot
disagree; if the probe reports otherwise, the probe is wrong, independent of
anything the TSC itself is doing. It reported 299,999 backwards reads out of
300,000 on that pair, and was discarded on its own evidence rather than
believed about the hardware.

The third, `ops/host/probes/tsc-migrate.c`, was built with nothing left to
misreport: one thread, migrating between CPUs, comparing only its own
consecutive reads, no lock, no shared mutable state, no memory-ordering
question to get wrong. Its tsc/hpet A/B (62,493 backwards reads per million
migrations vs. 0 in 500,000) is what the finding actually rests on, precisely
because the first two probes had already been caught failing rows the
hardware could not have caused.

## Where the habit already lives, and where it did not yet

`benchmarks/tests/controls_fire.rs` already encodes this for the paired
benchmark harness's own arms — a control the harness's instruments cannot
fail is treated as decorative there too. The gap was one layer down, in a
probe rather than in the harness, and it took two wrong answers before it was
added.

## Boundary

This stops holding when no row of the instrument's own behavior is
analytically forced — when every plausible row depends on the same
uncertain thing the probe is meant to test. In that case a control cannot be
built this way, and the honest statement is that the probe is unverified,
not that it passed.
