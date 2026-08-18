---
id: kb-decision-0022
title: The append condition is a max(position) guard inside BEGIN IMMEDIATE, and tags live in a join table keyed (tag, position)
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0022
reversibility: medium
phase: 8
supersedes: null
superseded_by: null
summary: >-
  Adopt the staged summary at .kb/_intake/0033-adr-0022-append-condition-strategy.md:12-38
  verbatim: a SQLite adapter evaluates an append condition as one SELECT max(position) per guard
  inside a transaction opened BEGIN IMMEDIATE, violated when the answer exceeds the guard's
  boundary and reporting that position as the conflicting event; measured against the two
  alternatives on the rejection path (23/213/972/42,399 us against higher costs, a tie on the
  accepted path and under contention, and the record says so rather than manufacturing a margin).
  Tags go in event_tag(tag, position) WITHOUT ROWID with event_type as a covering column and the
  key left (tag, position): a selective read of 516 of 50,050 costs 10.7 ms against a blob's 34.0
  and JSON1's 49.8, paid for with a 1.5x to 2.1x more expensive write. Skipping the GROUP BY ...
  HAVING COUNT(DISTINCT tag) aggregate for a single-tag item halves the probe, 1,093 us to 556,
  which is why tag_cardinality and most-selective-tag-first probing are requirements rather than
  tuning. Three pragma values are fixed — journal_mode WAL, synchronous NORMAL, busy_timeout 5,000
  ms, which absorbed 64-way contention with zero SQLITE_BUSY. The runtime seam captures a tokio
  Handle at construction with try_current as fallback so SqliteEventStoreError::NoRuntime keeps a
  real meaning. Query::index_arms() is rejected: it does not exist in happenstance-core, the
  decomposition stays adapter-private, and the re-open trigger is postgres and neon independently
  needing it. Two subjects are recorded as non-verdicts with owners: ES-17's &[Event] marker is not
  lifted, and CF-40's clause home stays open. rusqlite without a pool is ratified rather than
  decided.
depends_on:
  - kb-decision-0012
  - kb-decision-0010
related:
  - kb-reference-append-condition-experiment-001
  - kb-open-question-cf-40-ownership-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-concept-torn-read-append-boundary-001
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
  - RUNBOOK.md
last_reviewed: 2026-08-17
---

# The append condition is a max(position) guard inside BEGIN IMMEDIATE, and tags live in a join table keyed (tag, position)

## Decision

`happenstance-sqlite` evaluates an `AppendCondition` guard as a single `SELECT max(position)`
against the guard's derived query, run inside a transaction opened `BEGIN IMMEDIATE`. The guard is
violated when the answer exceeds the guard's boundary, and the position returned is reported as the
conflicting event. This wins against the two alternatives measured in
`kb-reference-append-condition-experiment-001` — a `BEGIN IMMEDIATE` plus `EXISTS` probe, and a
conditional `INSERT ... WHERE NOT EXISTS` — specifically on the **rejection path**, the path a DCB
retry loop takes every time it loses a race: 23 us against 32 and 45 over a 5,000-event log, 213
against 311 and 306 over 50,000, and by a similar margin on a two-tag boundary at both scales. On the
accepted-append path and under 8- and 64-connection contention the three strategies tie within
measurement noise, and that tie is reported rather than resolved into a manufactured margin — the
guard's win is real but scoped to the path where it was actually measured to differ.

Tags are stored in `event_tag(tag, position)`, a `WITHOUT ROWID` table with `event_type` carried as
a covering column and the primary key ordered `(tag, position)` rather than `(position, tag)` — the
ordering that makes a per-tag range scan selective. Against a canonical-JSON blob and a JSON1 column,
the join table's selective read (516 of 50,050 rows) costs 10.7 ms against 34.0 ms and 49.8 ms — a
3.16x and 4.63x win against a measured plus-or-minus 7 per cent noise floor — paid for with a write
that costs 1.5x to 2.1x more than either alternative. Reads dominate a DCB workload's steady state;
the trade is taken deliberately on that basis. Skipping the `GROUP BY ... HAVING COUNT(DISTINCT
tag)` aggregate for a single-tag boundary restores predicate pushdown on the guard's own boundary and
halves the probe from 1,093 us to 556 — measured against its own negative control, not assumed — and
because a two-tag boundary costs roughly 200x a single-tag one at 50,000 events on every strategy,
`tag_cardinality` and most-selective-tag-first probing are shipped as requirements the query planner
must satisfy, not as later tuning.

Three pragma values are fixed as documented adapter properties: `journal_mode=WAL`,
`synchronous=NORMAL`, and a finite `busy_timeout` of 5,000 ms, which absorbed 64-way write contention
in the experiment with zero `SQLITE_BUSY` errors. The runtime seam captures a `tokio::runtime::Handle`
at store construction, falling back to `Handle::try_current` when none is supplied, so that
`SqliteEventStoreError::NoRuntime` keeps a real, reachable meaning rather than being dead code behind
a constructor that always succeeds.

`Query::index_arms()` — a proposed decomposition of a `Query` into per-index probe arms — is
rejected for this adapter. It does not exist as a method on `happenstance-core`'s `Query`, and the
decomposition this adapter needs stays adapter-private rather than becoming a contract-level API. The
named re-open trigger is `happenstance-postgres` and `happenstance-neon` independently needing the
same decomposition; one adapter wanting it is not sufficient grounds to widen the contract.

Two subjects are recorded as non-verdicts with named owners rather than settled here. ES-17's
`&[Event]` marker on `append` is not lifted — `kb-open-question-es-17-two-adapter-measurement-001`
records why this experiment's three candidates, which differ in strategy rather than in batch
ownership, do not supply the two-build evidence ADR-0012's falsifier requires. CF-40's clause home —
which document owns a fixture-constant clause — stays open at `kb-open-question-cf-40-ownership-001`.
`rusqlite` without a connection pool, which the backlog queue row already treated as settled before
this ADR was written, is ratified here rather than decided fresh.

## Alternatives rejected

The `EXISTS` probe and the conditional `INSERT` both lost on the rejection path at every scale
measured, with no scale at which either was cheaper; neither is retained as a fallback because the
tie on the accepted-append path means there is no path on which choosing them instead would help. A
canonical-JSON tag blob and a JSON1 column both lost the read benchmark by more than 3x against the
noise floor, and while both write cheaper than the join table, a DCB workload's read pressure — every
guard evaluation is a read — makes that the wrong side of the trade to optimise. `Query::index_arms()`
as a contract-level method was rejected in terms, not merely deferred: it does not exist, and adding
it on the strength of one adapter's convenience would widen `happenstance-core` for a decomposition
nothing outside this crate currently needs.
