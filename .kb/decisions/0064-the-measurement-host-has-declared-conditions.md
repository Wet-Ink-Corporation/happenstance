---
id: kb-decision-0064
title: The measurement host has declared conditions, and its preflight is unreachable from the gate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0064
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  A dedicated Linux host is the reference machine for benchmarks and for
  build/test, and its conditions are data rather than prose: declared in
  ops/host/host.env, applied by a systemd unit (00-system.sh, cpu-tuning.sh),
  and asserted by ops/host/preflight.sh. Two properties are load-bearing: one
  script writes and the other only reads, so a script that did both could
  always make its own check pass; and --restore restores captured values,
  never assumed defaults, so a restore cannot silently leave the machine in a
  state that merely passes the next check. The decision is forced by
  instability written down three times independently (benchmarks/README.md,
  RUNBOOK.md, references/adr/0022-append-condition-strategy.md:120-144); the
  existing answer — interleave arms, quote ratios, prefer allocation counts —
  makes numbers comparable without making the machine quiet. The CF-34
  boundary (spec/SPECIFICATION.md:9021): preflight.sh asserts on the
  environment, using only values readable before the first sample exists,
  which is what separates it from a budget. Structurally nothing in xtask's
  step table, .redkiln/config.yaml's verify: block or .github/workflows/
  invokes it, and ops/ is on xtask/src/affected.rs's INERT list. Residual
  risk named: registering this host as a self-hosted CI runner would make the
  preflight reachable from a merge-blocking job, and that is a decision, not
  a configuration change. Not decided: frequency cap and SMT stay as-found;
  benchmarks/results/ is not re-taken; RESOLVABLE_FLOOR stays documented
  against the Windows host; a Windows absolute is never compared with a
  Linux absolute.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-host-clocksource-tsc-hpet-001
  - kb-playbook-control-fires-on-instrument-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-append-condition-experiment-001
source_paths:
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - ops/host/README.md
  - ops/host/host.env
  - ops/host/preflight.sh
  - ops/host/00-system.sh
  - ops/host/cpu-tuning.sh
  - ops/host/happenstance-bench-tuning.service
  - xtask/src/affected.rs
  - benchmarks/src/paired.rs
  - benchmarks/src/cpu.rs
  - benchmarks/README.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-11
---

# The measurement host has declared conditions, and its preflight is unreachable from the gate

## Decision

A dedicated Linux host is the reference machine for this repository's
benchmarks and its build/test. Its conditions are **data**, not prose in a
README and a ritual in somebody's shell history: `ops/host/host.env`
declares them, `00-system.sh` and `cpu-tuning.sh` apply them, and
`ops/host/preflight.sh` asserts them before a run is trusted.

Two properties are load-bearing. **One script writes, the other only
reads** — the apply scripts change the machine, `preflight.sh` only judges
it, and never both in one script, because a script that did both could
always make its own check pass. That is the same argument
`benchmarks/README.md` already makes for keeping
`benchmarks/tests/instruments_work.rs` separate from the instruments it
checks. And **`--restore` restores captured values, never assumed
defaults** — the first `--apply` snapshots what it is about to change and
never overwrites that snapshot, so a restore that wrote back a plausible
default (`powersave`, a guessed `cpuinfo_max_freq`) instead of what was
actually there would leave a machine that passes the next check while no
longer being what it was.

## Why this is forced

Not a preference: benchmark instability on developer machines is written
down three times independently before this host existed —
`benchmarks/README.md`'s §"What none of this shows" (wall-clock medians
moving up to 40% while allocation counts held to the digit), `RUNBOOK.md`
(a sequential baseline moving 2.7x at one client and 3.0x at 64), and
`references/adr/0022-append-condition-strategy.md:120-144` (45% disagreement
an hour apart, and one unconditional-append arm — on a path no strategy
touches — varying 4x between slots). The existing answer, `paired.rs`
interleaving arms and quoting ratios over absolutes, is correct and
incomplete: it makes the numbers comparable without making the machine
quiet. A declared-conditions host is the other half.

## The CF-34 boundary

CF-34 (`spec/SPECIFICATION.md:9021`) forbids a benchmark *result* gating a
merge. `preflight.sh` asserts on the *environment* — every value it reads
comes from sysfs, systemd, or a `--version` check, all available before the
first sample exists, so it can fail with nothing measured yet. That is what
separates it from a budget. `paired.rs:71-73` draws the identical line from
the measurement side: it computes drift and refuses to threshold it,
because a drift threshold would be exactly the result-gating CF-34
forbids.

Structurally, nothing wires the preflight into anything that blocks a merge:
it is absent from `xtask/src/main.rs`'s step table and
`.redkiln/config.yaml`'s `verify:` block, and `ops/` sits on
`xtask/src/affected.rs`'s `INERT` list, held by the test
`the_host_provisioning_tree_selects_no_package`. **Residual risk, named
rather than dismissed:** registering this host as a self-hosted CI runner
would make the preflight reachable from a merge-blocking job, at which point
this paragraph stops being true. That is itself a decision, not a
configuration change, and is not being made here.

## What this decision does not settle

The frequency cap and SMT stay at their as-found settings — capping and
halving the machine before flakiness is measured would make any later
improvement unattributable. `benchmarks/results/` is not re-taken; it stays
sourced from the Windows host, and the figures four `[PROVISIONAL]` clauses
and two doc comments reason from are unchanged. `RESOLVABLE_FLOOR`
(`benchmarks/src/cpu.rs:58`) stays documented against the Windows host's
~15.6 ms scheduler-accounting granularity, which the Linux host's
`CLOCK_PROCESS_CPUTIME_ID` does not share; lowering it wants its own
measurement. A Windows absolute is never compared with a Linux absolute —
only same-run ratios cross that boundary. The clocksource finding that
motivated part of this work is `kb-reference-host-clocksource-tsc-hpet-001`;
the method that produced it is `kb-playbook-control-fires-on-instrument-001`.
