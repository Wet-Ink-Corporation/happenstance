---
id: kb-decision-0058
title: The SQLite write path stays inline, and the crate stops claiming otherwise
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0058
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  append, head and contains_event_id keep running inline under a blocking mutex rather than being routed through the projection store's in_blocking_task seam. The README sentence claiming every statement runs on a blocking task is corrected, and a Cancellation section states today's true answer: the absent await is what makes a dropped append provably commit nothing, and routing through spawn_blocking would flip that to the may-have-committed shape ES-23 names as the trap. The measured cost is real — 885 ms of reactor stall against the seam's 27 — and the two routing options stay unripe pending an uncontended-latency measurement.
depends_on:
  - kb-decision-0022
related:
  - kb-reference-one-connection-latency-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-es-11-sqlite-ceiling-sample-cost-001
  - kb-open-question-es-23-adapter-half-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# The SQLite write path stays inline, and the crate stops claiming otherwise

## Decision

`SqliteEventStore::append`, `head` and `contains_event_id` keep running inline, under a
blocking `std::sync::Mutex`, on whatever thread polls them
(`crates/happenstance-sqlite/src/event_store.rs:1396-1431`, `:1447-1462`, `:1474-1490`).
They are **not** routed through `in_blocking_task`, the `spawn_blocking` seam
`SqliteProjectionStore::commit` already uses (`projection_store.rs:342-361`), even though
the projection store's own module doc calls a crate answering the same question two ways
"the defect" (`projection_store.rs:97-98`).

**The cost is real and measured, not disputed.** `experiments/one-connection-latency/results/raw/reactor-stall.txt`,
on a `current_thread` runtime with a second connection holding `BEGIN IMMEDIATE` for 750 ms
(well inside the 5,000 ms busy timeout, so every arm succeeds): the shipped `append` stalls
the reactor for **885.761 ms** against a same-crate control — `SqliteProjectionStore::commit`
under identical contention — of **27.525 ms**. Routing the same `append` through the seam
measures at **34.812 ms**, within 1.5x of the control. `head` behind an in-flight append
shows the same pattern: 717.545 ms shipped against 41.573 ms through the seam.

**Two published claims are corrected regardless of which routing wins.**
`crates/happenstance-sqlite/README.md:58` — the crate's crates.io front page — asserts
"every statement runs on a blocking task"; only the read path's deferred `spawn_blocking`
is true today, and the sentence is corrected to say so. And the crate gains the
`# Cancellation` section ES-23 `[FROZEN]` requires and does not have
(`spec/SPECIFICATION.md:3636-3660` names "Rule: none," so no gate check catches its
absence): today's true answer is that a dropped `append` future provably commits nothing,
because `append` contains no `.await` anywhere in its body — the same omission that causes
the stall is this adapter's current ES-23 answer.

**Routing the write path through the seam is not taken now, because it is not free the way
it first looked.** `spawn_blocking` demands `'static`, and `SendEventStore::append` takes
`events: &[Event]` and `condition: Option<&AppendCondition>` by borrow — unlike
`ProjectionStore::commit`, which takes its batch by value — so the seam can only be entered
by cloning the batch first, unconditionally, including on the rejection path. ES-17's own
stated ground for the borrow (`spec/SPECIFICATION.md:3373-3374`) is that "a rejected append
clones nothing," and rejection is the routine outcome under contention; routing through the
seam breaks that ground on this adapter specifically. Worse, it flips this adapter's ES-23
answer from "provably committed nothing" to "may have committed, caller told nothing" — the
exact shape ES-23 and the port method's own doc name as the trap a cancellation-safe-looking
adapter must not be. Neither cost was priced when this routing was first proposed as the
audit's remedy.

An inline-fallback variant (hop through the seam when a runtime is reachable, run inline
otherwise) is also not taken now: it contradicts `kb-decision-0022`'s accepted summary
sentence about `NoRuntime` keeping a real meaning across the crate, so it would need a
superseding atom rather than an amending one, and it gives the crate a *third* answer
(event store falls back, projection store does not) unless the projection store is changed
identically — a wider change than anything measured here.

## Alternatives rejected

`tokio::task::block_in_place` is rejected on mechanism, not taste: it panics on a
`current_thread` runtime, which is the exact flavour every measured arm uses and the
default `#[tokio::test]` flavour. A `try_lock` heuristic — run inline only when the
connection mutex is free — is rejected because the measured 885.761 ms stall is SQLite's
own busy handler waiting on the file write lock while the process mutex is free; a
`try_lock` cannot see that contention at all.

## What this does not settle

Whether the write path is ever routed through the seam, and on which shape (full seam or
fallback), is left open pending two things this decision does not supply: an uncontended
`append`-latency measurement (inline against seam, at tag counts 0/8/64 and batch sizes
1/256, on both the accepted and rejected append paths) and a written `# Cancellation`
answer for each candidate routing. Whether the read path's ceiling-sample stall (a related
but separate ~635 ms cost under contention) is addressed by cooperative re-polling or a
second dedicated connection is explicitly left undecided, entangled with an unrelated
`Clone`/`connect()` question this record does not own. Whether an explicit-`Handle`
constructor is added alongside the unconditional capture at construction is also open,
though it is additive either way and carries no deadline.
