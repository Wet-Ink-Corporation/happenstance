---
id: kb-reference-busy-timeout-margin-001
title: The busy-timeout margin at 64 contenders is 1.3x to 1.4x, and fewer cores is not safer
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-12 measurement taken 2026-09-03 from experiments/busy-timeout-margin/, of how much of
  ADR-0022's fixed 5,000 ms busy_timeout the shipped conformance configuration actually consumes.
  At the shipped CONTENDERS = 64 the worst per-contender wait is 3,628 ms of the 5,000 ms budget,
  a margin of 1.38x; the worst cell anywhere in the matrix is 3,828 ms at 8 cores, 1.31x. The
  defensible claim is 1.3x to 1.4x on the plateau, and not more, because two mechanisms a
  reasonable person would predict are refuted by the matrix. Fewer cores is not safer: 1 core
  gives 8 ms, 2 gives 628, 4 gives 2,628, 8 gives 3,828 and 20 gives 3,628, a roughly 450-fold
  swing, because SQLite's busy handler is a back-off poll rather than a queue and the pathology
  needs contenders that are simultaneously runnable — so constraining cores measures the safest
  cell in the matrix rather than a conservative one. And the build profile barely enters: debug's
  3,628 ms sits inside release's 3,228 and 3,328 ms band, because the time is spent asleep and
  --release has nothing to recover. Every wait_ms is a lower bound: the counting handler the
  experiment installs resolves the same race about 2.6x faster than SQLite's own, uncalibrated,
  with one of five ratios inverted. Storage was never varied, and storage is what a busy handler
  ultimately waits on. Conditions, because none of it is portable: i9-13905H (14c/20t), Windows
  11, NTFS/NVMe, rustc 1.97.1 msvc, SQLite 3.53.2, wal plus synchronous=normal read back off the
  live connection, on a host running about twenty other agent processes. This measurement records
  busy > 0 at 64 contenders; kb-reference-append-condition-experiment-001 recorded busy = 0 at the
  same count on 2026-08-16, and both are true of their own moment.
depends_on: []
related:
  - kb-decision-0022
  - kb-reference-append-condition-experiment-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-nested-block-on-lost-wakeup-001
  - kb-open-question-testkit-contention-tolerance-001
  - kb-reference-one-connection-latency-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
---

# The busy-timeout margin at 64 contenders is 1.3x to 1.4x, and fewer cores is not safer

## What this is a pointer to

The instrument — a harness that opens N `rusqlite` connections against one file under
`busy_timeout = 5000`, races them on a single row, and records each contender's actual wait — lives
in `experiments/busy-timeout-margin/`, outside the workspace and outside the gate. This atom is the
citable summary of what it measured against ADR-0022's fixed 5,000 ms budget
(`crates/happenstance-sqlite/src/event_store.rs`). The conclusion of whether the budget needs to
move is not this atom's; see `kb-open-question-adr-0022-falsifiers-fired-001`.

## The question

ADR-0022 fixed `busy_timeout` at 5,000 ms and shipped the conformance suite's contention rule at
`CONTENDERS = 64`, with a stated falsifier: re-open if any run ever reports `busy > 0`. This
experiment answers a narrower, prior question — assuming no run yet fails, how much of the budget
is actually being spent getting there.

## The headline number, and why it is not tighter

At the shipped configuration the worst observed per-contender wait is 3,628 ms of 5,000, a margin
of 1.38x. The single worst cell in the full core-count matrix is 3,828 ms at 8 cores, 1.31x. Stated
as a range rather than a point because the matrix does not sit on a single plateau value — 1.3x to
1.4x is what the data supports, and a single "the margin is 1.35x" figure would claim precision the
harness does not have.

## Two refuted predictions

**Fewer cores is not safer.** Naively, fewer runnable threads should mean less contention. Measured:
1 core → 8 ms, 2 → 628, 4 → 2,628, 8 → 3,828, 20 → 3,628 — roughly a 450-fold swing, and the *worst*
cell sits at 8 cores, not at 20. The mechanism: SQLite's busy handler is a back-off poll, not a
wait queue, and the pathology it exposes needs contenders that are simultaneously schedulable. One
core forecloses that by construction, so a single-core run measures the safest cell available, not
a conservative stand-in for the real one.

**The build profile barely enters.** Debug's worst wait (3,628 ms) sits inside release's band (3,228
to 3,328 ms). The time being measured is time asleep in the OS scheduler, and `--release` has
nothing to optimise there.

## What makes every number here a lower bound

The experiment's own busy handler counts retries rather than reproducing SQLite's built-in backoff
curve, and it resolves the same race about 2.6x faster than SQLite's own handler does, uncalibrated
— with one of the five measured ratios inverted from the other four. So every `wait_ms` in this
atom understates what a run under SQLite's real handler would show. Storage was never varied across
the matrix, and storage is the resource a busy handler is ultimately waiting on — the whole
experiment ran on one NVMe device.

## Conditions, and the sibling this disagrees with

i9-13905H (14c/20t), Windows 11, NTFS/NVMe, rustc 1.97.1 msvc, SQLite 3.53.2, WAL plus
`synchronous=NORMAL` read back off the live connection rather than trusted from the pragma
statement, on a host running roughly twenty other agent processes at measurement time.
`kb-reference-append-condition-experiment-001` recorded `busy=0, failed=0` across thirty races at
64 connections on 2026-08-16; this measurement recorded `busy > 0` at the same contender count on
2026-09-03. Both are correct statements about their own moment, and the disagreement is itself the
finding: `reference/README.md`'s dating rule exists so that a later, contradicting measurement
supersedes nothing and simply sits alongside the earlier one.
