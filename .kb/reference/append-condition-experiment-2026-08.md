---
id: kb-reference-append-condition-experiment-001
title: The append-condition experiment — three strategies and three tag storages against real SQLite
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-8 measurement ADR-0022 rests on, run 2026-08-16 against real SQLite 3.53.2 under WAL,
  synchronous NORMAL and a finite 5,000 ms busy timeout, kept in experiments/append-condition/
  outside the workspace and outside the gate. All five measured arms cleared
  event_store_conformance! first (445 tests, 89 rules each) because a wrong arm is always the
  fastest. Three append-condition strategies separate only on the rejection path, measured
  round-robin in one process over 400 rounds: the monotonic-position guard costs 23 us against the
  BEGIN IMMEDIATE plus EXISTS probe's 32 and the conditional INSERT's 45 over a 5,000-event log,
  213 against 311 and 306 over 50,000, 972 against 1,513 and 1,532 on a two-tag boundary at 5,000,
  and 42,399 against 65,637 and 65,383 at 50,000. On the accepted-append path and under contention
  at 8 and 64 connections the three are a tie within noise and are reported as one rather than
  given a manufactured margin. Three tag storages, 50,000 events, 25 rounds round-robin: the join
  table's selective read of 516 of 50,050 costs 10,744 us against a canonical blob's 33,992 and
  JSON1's 49,766 — 3.16x and 4.63x against a plus-or-minus 7 per cent noise floor — while its
  unconditional single-event append costs 862 us against 414 and 590. Dropping the GROUP BY
  aggregate for a single-tag item, against its own negative control, cut the probe from 1,093 us to
  556. A two-tag boundary costs roughly 200x a single-tag one at 50,000 events on every strategy.
  Sixty-four rusqlite connections opened on one file on every one of thirty races with busy 0 and
  failed 0 and exactly one winner each. Two positive controls fired: the runner refuses to emit a
  number under synchronous OFF, and the journal mode is read back rather than trusted from the
  PRAGMA that was issued. Two harness figures are noise-dominated on a shared developer host, two
  runs an hour apart disagreeing by up to 45 per cent, and nothing here measured the tokio runtime
  seam.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0012
  - kb-reference-position-visibility-experiment-001
  - kb-open-question-es-17-two-adapter-measurement-001
  - kb-reference-busy-timeout-margin-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-shipped-append-condition-sql-001
source_paths:
  - .kb/_intake/0034-append-condition-experiment-2026-08.md
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
last_reviewed: 2026-08-17
---

# The append-condition experiment — three strategies and three tag storages against real SQLite

## What this is a pointer to

The full instrument — three candidate `EventStore` implementations over `rusqlite`, three tag
storages, one schema, one read path, its fixtures, two positive controls and every raw result row
— lives in `experiments/append-condition/`, outside the workspace and outside the gate: an empty
`[workspace]` table, no `verify:` command, no `cargo xtask ci` step, no workspace dependency. This
atom is the citable summary of what it measured. The conclusions drawn from it — which strategy the
adapter ships, what migration 1 holds, which pragmas become documented properties — are ADR-0022's
(`kb-decision-0022`), not this atom's. The precedent shape for splitting evidence from decision this
way is `kb-reference-position-visibility-experiment-001`, cited rather than copied.

## The question the experiment answers

`SqliteEventStore::append` was `todo!()` at measurement time, with no implementation in the
workspace to benchmark, so the record needed a figure that did not yet exist in production code.
Three append-condition strategies and three tag-storage shapes were built as standalone candidates
purely to be measured, cleared against `event_store_conformance!` first — 445 tests, 89 rules
across five arms, all green — because an unconformant arm is always the fastest one on the clock.

## The three append-condition strategies

All three are indistinguishable on the accepted-append path and under contention; they separate
only on the **rejection path**, where a `SELECT max(position)` guard is compared against a `BEGIN
IMMEDIATE` plus `EXISTS` probe and a conditional `INSERT ... WHERE NOT EXISTS`. Round-robin over 400
rounds in one process: 23 / 32 / 45 us over a 5,000-event log, 213 / 311 / 306 over 50,000, 972 /
1,513 / 1,532 on a two-tag boundary at 5,000, and 42,399 / 65,637 / 65,383 at 50,000 — the guard
wins every rejection-path row it was measured on. On the accepted-append path and at 8 and 64
connections the three arms tie within noise, reported as a tie deliberately rather than manufactured
into a margin.

## The three tag storages

Measured over 50,000 events, 25 rounds round-robin. A join table (`event_tag(tag, position)`) reads
selectively — 516 of 50,050 rows — at 10,744 us, against a canonical-JSON blob's 33,992 and JSON1's
49,766: 3.16x and 4.63x cheaper against a measured plus-or-minus 7 per cent noise floor. The join
table pays for that read with a more expensive write: 862 us unconditional single-event append
against the blob's 414 and JSON1's 590. Dropping the `GROUP BY ... HAVING COUNT(DISTINCT tag)`
aggregate for a single-tag boundary, checked against its own negative control, halves the probe from
1,093 us to 556 — restoring predicate pushdown on the guard's boundary rather than tuning an
unrelated constant. A two-tag boundary costs roughly 200x a single-tag one at 50,000 events, on
every strategy, which is the shape that makes single-tag fast-pathing worth doing rather than
optional.

## Positive controls

Two fired, which is what makes a passing measurement worth citing. Conformance-before-measurement:
all five arms green before a single timing number was recorded. The durability refusal: a connection
forced to `synchronous = OFF` makes the runner refuse to emit a number at all, and the journal mode
is read back off the live connection rather than trusted from the `PRAGMA` statement that was issued
— SQLite silently ignores a `journal_mode` it cannot honour, so the statement alone is not evidence.
Contention: sixty-four `rusqlite` connections opened on one file across thirty races each recorded
`busy=0`, `failed=0`, and exactly one winner.

## What the instrument cannot do

Two harness figures are noise-dominated on a shared developer host — two runs an hour apart
disagreed by up to 45 per cent, and one arm's unconditional-append figure varied 4x between slots —
which is why the two figures ADR-0022 actually decided on came from round-robin, single-process
measurement rather than the sequential harness. Nothing here exercises the tokio runtime seam: the
experiment ran on bare OS threads and contributes no cost comparison for it, only the negative result
that sixty-four threads with no runtime context completed correctly.
