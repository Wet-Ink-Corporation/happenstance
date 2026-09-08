---
id: kb-reference-one-connection-latency-001
title: An inline SQLite write stalls a current_thread reactor for 885 ms, and the projection store's existing seam for 27
kind: reference
status: accepted
authority_tier: note
summary: >-
  An eight-arm reactor-stall measurement from experiments/one-connection-latency/,
  separating SQLite's busy handler (the file write lock, unaffected by an
  in-process seam) from happenstance-sqlite's own process Mutex (held by inline
  execution, removable by a spawn_blocking seam). Shipped append stalls a
  current_thread runtime 885.761 ms against the shipped projection store's own
  seam at 27.525 ms under identical contention; routing append through the same
  seam brings it to 34.812 ms. A second table gives per-page lock-hold and
  residency figures at PAGE_SIZE 64/512/2048, and a third gives connection-sharing
  tail latency (p50 0.118 ms / p99 7.727 ms quiet, p99 799.041 ms at width 1,200).
depends_on: []
related:
  - kb-decision-0058
  - kb-decision-0053
  - kb-reference-busy-timeout-margin-001
  - kb-reference-nested-block-on-lost-wakeup-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-read-page-budget-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md
  - .kb/_intake/remediation-2026-09-04-briefs/read-page-budget-rows-bytes-or-caller.md
  - experiments/one-connection-latency/
last_reviewed: 2026-09-07
---

# An inline SQLite write stalls a current_thread reactor for 885 ms, and the projection store's existing seam for 27

## What this is a pointer to

The instrument, its eight-arm harness and every raw result file live in
`experiments/one-connection-latency/`, outside the workspace and the gate. This
atom is the citable summary of what it measured. Two separate decision briefs
draw on it for two different questions — where the blocking seam should sit on
`happenstance-sqlite`'s write and read paths, and how a read page's byte budget
should be set — and the conclusions each brief reaches belong to the decision
atoms that cite this one, not to this atom.

## The reactor-stall table

One `current_thread` tokio runtime, a 1 ms `tokio::time::interval` as the
canary, a second connection holding `BEGIN IMMEDIATE` for 750 ms — well inside
the crate's 5,000 ms busy timeout, so every arm succeeds:

| arm | call | max tick gap | ticks |
| --- | ---: | ---: | ---: |
| 0. idle (Windows timer resolution) | 762.6 ms | 16.177 ms | 54 |
| 1. shipped `SqliteEventStore::append` | 870.3 ms | **885.761 ms** | 5 |
| 2. control — shipped `SqliteProjectionStore::commit` | 870.9 ms | 27.525 ms | 62 |
| 3. the same append through an `in_blocking_task` seam | 778.9 ms | 34.812 ms | 54 |
| 4. shipped `head()` behind an in-flight append | 873.4 ms | 717.545 ms | 15 |
| 5. shipped `read()` first poll, same contention | 793.4 ms | 639.290 ms | 15 |
| 6. `head()` through the seam | 777.7 ms | 41.573 ms | 54 |
| 7. residual — replica `read()` first poll, after the seam | 789.4 ms | 635.056 ms | 15 |

Arms 1 and 2 wait the same 750 ms for the same lock, and one of them takes the
runtime's other timers and I/O completions down with it for that whole window;
without arm 2 as a same-crate control, 885.761 ms would be a number with no
scale. The harness deliberately separates two distinct causes
(`experiments/one-connection-latency/src/holder.rs`): under WAL a writer does
not block readers, so the holder blocks `BEGIN IMMEDIATE` (what `append` opens)
and does not block `SELECT max(position)` (what `head` and the read path's
ceiling sample run). Arm 1 is therefore SQLite's own busy handler sleeping on
the file write lock with the crate's process `Mutex` free; arms 4, 5 and 7 are
that process `Mutex`, held by the shipped `append` running inline rather than
by anything SQLite itself requires — WAL had already made those stalls
avoidable. Routing `append` and `head` through the crate's existing
`in_blocking_task` seam (already shipped for the projection store) brings arm 1
to 34.812 ms and arm 4 to 41.573 ms, both within about 1.5x of the projection
store's own 27.525 ms control.

## The page-budget tables

Per-page lock hold and aggregate residency at three candidate `PAGE_SIZE`
values, over a real replay:

| | at `PAGE_SIZE` 64 | 512 (shipped) | 2,048 |
|---|---|---|---|
| per-page lock hold, width 1 | 187.5 ms | 146.6 ms | 202.6 ms |
| aggregate held mutex over 10^6 events, width 1 | 2,929.2 s | 286.4 s | 99.1 s |
| one page at the data ceiling | ~64 MiB | 537,036,800 B (512.2 MiB) | ~2 GiB |

The two rows move in opposite directions as `PAGE_SIZE` rises: lock-hold
aggregate falls, but per-page byte residency at the declared data ceiling grows
linearly and without bound — at 2,048 rows a single page of maximal-size events
would be roughly 2 GiB. One knob cannot co-optimise both consequences at any
value. A third table gives connection-sharing tail latency: a caller sharing
the store's handle pays only at the tail, p50 0.118 ms / p99 7.727 ms quiet
against p50 0.148 ms / p99 799.041 ms under a concurrent replay at width 1,200.

## What this does not establish

The measurement is taken on one host (the same conditions
`kb-reference-busy-timeout-margin-001` records: i9-13905H, Windows 11,
NTFS/NVMe) and does not resolve whether a page's byte budget should be stated
by the adapter, by the caller, or derived from a single unit — that trade, and
whichever `PAGE_SIZE` value or seam routing is adopted, is decided by the
citing decision atoms. Neither table measures a Linux host or a real production
disk; both instruments record that limitation themselves.
