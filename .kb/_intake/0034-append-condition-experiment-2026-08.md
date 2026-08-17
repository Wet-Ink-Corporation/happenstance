---
id: kb-reference-append-condition-experiment-001
title: The append-condition experiment — three strategies and three tag storages against real SQLite
kind: reference
status: accepted
authority_tier: note
summary: >-
  The phase-8 measurement ADR-0022 rests on, run against real SQLite 3.53.2 under WAL,
  synchronous=NORMAL and a finite 5,000 ms busy timeout, kept in experiments/append-condition/.
  All five measured arms cleared event_store_conformance! first — 445 tests, 89 rules each —
  because a wrong arm is always the fastest. Three append-condition strategies separate only on
  the rejection path, measured round-robin in one process over 400 rounds: the monotonic-position
  guard costs 23 us against the BEGIN IMMEDIATE + EXISTS probe's 32 and the conditional INSERT's
  45 over a 5,000-event log, 213 against 311 and 306 over 50,000, 972 against 1,513 and 1,532 on a
  two-tag boundary at 5,000, and 42,399 against 65,637 and 65,383 at 50,000. On the accepted-append
  path (guard cost 67, 79, 74 us) and under contention at 8 and 64 connections the three are a tie
  within noise and are reported as one. Three tag storages, 50,000 events, 25 rounds round-robin:
  the join table's selective read of 516 of 50,050 costs 10,744 us against a canonical blob's
  33,992 and JSON1's 49,766 — 3.16x and 4.63x against a +/-7% noise floor measured as an unfiltered
  read that touches no tag storage — while its unconditional single-event append costs 862 us
  against 414 and 590. Dropping the GROUP BY aggregate for a single-tag item, measured against its
  own negative control, cut the probe from 1,093 us to 556. A two-tag boundary costs roughly 200x
  a single-tag one at 50,000 events on every strategy. Sixty-four rusqlite connections opened on
  one file on every one of thirty races with busy=0 and failed=0 and exactly one winner each; a
  race costs about 130 ms at 8 contenders and 1.4 to 2.7 s at 64, a factor of 11 to 20. Two
  positive controls fired: the runner refuses to emit a number under synchronous=OFF, and the
  journal mode is read back rather than trusted from the PRAGMA that was issued.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0012
source_paths:
  - .kb/_intake/0034-append-condition-experiment-2026-08.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
last_reviewed: 2026-08-16
---

# Staged: the append-condition experiment

**Staged 2026-08-16** for the next `/redkiln:kb-ingest` wave. **Not an atom yet.**
The frontmatter above is *proposed*. It is the **evidence** half of a pair: the
decision it supports is staged beside it as
`.kb/_intake/0033-adr-0022-append-condition-strategy.md`.

**Why the measurement is its own atom.** `.kb/decisions/README.md` keeps
measurements out of decisions, so that a decision can be **superseded without
invalidating the numbers underneath it**. `.kb/reference/position-visibility-experiment-2026-08.md`
is the precedent shape, and its discipline is the one followed here: the summary
carries the actual ratios and the controls that fired, not adjectives.

## What this is a pointer to

The full instrument — three candidate `EventStore` implementations over
`rusqlite`, three tag storages, one schema, one read path, one identity story,
its fixtures, its two positive controls and every raw result row — lives in
`experiments/append-condition/`, **outside the workspace and outside the gate**.
It carries an empty `[workspace]` table so cargo does not adopt it, it is in no
`verify:` command and no `cargo xtask ci` step, and it adds no dependency to any
workspace manifest.

This atom is the citable summary of what it measured. The conclusions drawn from
it — which strategy the adapter ships, what migration 1 holds, which pragmas are
documented properties — are ADR-0022's, not this atom's.

## The question the experiment answers

`SqliteEventStore::append` is `todo!()` and stays that way until three stories
after ADR-0022 lands, because AC-013 puts the record before the implementation.
So there was nothing in the workspace to measure, and a record that quoted a
preference instead of a figure would have failed the criterion it was written
for. The experiment is where the figure came from.

## The conditions, without which no figure means anything

| | |
| --- | --- |
| Machine | 13th Gen Intel Core i9-13905H, 20 logical cores, 32 GB |
| OS / filesystem | Windows 11 (10.0.26200), NTFS on local NVMe |
| Toolchain | `rustc 1.97.1`, `x86_64-pc-windows-msvc`, `--release` |
| SQLite | 3.53.2, `rusqlite` 0.40 `bundled`, no pool |
| `journal_mode` | `wal` |
| `synchronous` | `normal` |
| `busy_timeout` | 5,000 ms |
| Command | `experiments/append-condition/run.sh`, about five minutes |

Every printed row carries these beside itself, read back off the live connection
rather than trusted from the `PRAGMA` that was issued — SQLite silently ignores a
`journal_mode` it cannot honour, so the statement is not evidence of the setting.

## The controls that fired

**Conformance before measurement.** 445 tests, 89 rules across five arms, all
green, in `tests/candidates_are_conformant.rs`. A wrong arm is always the
fastest; an arm that had not cleared the suite would have had its figure
discarded.

**The durability refusal.** `tests/durability_settings_are_enforced.rs` forces a
connection to `synchronous = OFF` and asserts the runner **refuses to emit a
number** — because `spec/SPECIFICATION.md:7481-7484` names that pragma by name as
a wrong implementation CF-14's reopen rule rejects, so a figure produced under it
is a figure for a store that cannot ship. A control that cannot fire is
decorative; this one is made to fire. The same file asserts the journal mode is
read back rather than assumed.

## What the instrument cannot do

**Two harness figures are noise-dominated on this host.**
`event_store_benchmarks!` emits one `#[test]` per scenario, so each arm gets its
own time slot; on this machine two runs an hour apart disagreed by up to 45%, and
one arm's *unconditional* append — a path no strategy participates in — varied
4x between slots. The harness's timer also covers fixture construction, which at
`k = 64` is sixty-five `connect()` calls.

That is a property of measuring on a shared developer host rather than of the
harness, and **the remedy is the emitter's**, which is what CF-23 makes the
wrapper a parameter for. The two figures ADR-0022 decides on come from
caller-side controls that measure the arms **round-robin inside one process**.
`bench.rs` delegates the threaded half in terms: *"an adapter that wants
thread-level contention supplies it through its own emitter."*

**Nothing here measured the tokio runtime seam.** The experiment has no tokio in
it. It contributes one negative result — sixty-four bare OS threads with no
runtime context anywhere completed correctly — and no cost comparison.

## The transferable findings, beyond this one decision

Three things this experiment learned that are not about SQLite:

1. **On a shared host, measure arms round-robin inside one process.** Three
   sequential tests were written first and thrown away: two runs an hour apart
   disagreed about the *ordering* of a read, because the machine slowed down and
   whichever arm ran late wore it. Interleaving is what makes a ratio mean
   anything; an absolute figure on such a host means much less.
2. **Subtract a baseline, and check the subtraction is resolvable.** Guard cost
   was measured as *conditional append minus unconditional append*. At 5,000
   events that resolves; at 50,000 the commit rises to ~18 ms while the guard
   stays in the tens of microseconds and the difference of two medians three
   orders of magnitude larger is noise. Reporting it as noise is the finding.
3. **Measure the path where the arms structurally differ.** All three strategies
   were indistinguishable until the *rejection* path was timed — where no commit
   happens, so the cost that dominates everything is out of the measurement, and
   where the arms genuinely differ in statement count. A benchmark that only ever
   measures the happy path measures the thing all the candidates share.
