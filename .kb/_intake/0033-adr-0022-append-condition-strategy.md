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
  A SQLite adapter evaluates an append condition as one SELECT max(position) per guard inside a
  transaction opened BEGIN IMMEDIATE, violated when the answer exceeds the guard's boundary and
  reporting that position as the conflicting event. Measured against the two alternatives on the
  rejection path — the path a DCB loop takes whenever it loses a race — the guard costs 23 us
  against the BEGIN IMMEDIATE + EXISTS probe's 32 and the conditional INSERT ... WHERE NOT EXISTS's
  45 over a 5,000-event log, and 213 against 311 and 306 over 50,000; on a two-tag boundary 972
  against 1,513 and 1,532, and 42,399 against 65,637 and 65,383. On the accepted-append path and
  under contention at 8 and 64 connections the three arms are a tie within measurement noise and
  the record says so rather than manufacturing a margin. Tags go in event_tag(tag, position)
  WITHOUT ROWID with event_type as a covering column and the key left (tag, position): a selective
  read of 516 events out of 50,050 costs 10.7 ms against a canonical blob's 34.0 and JSON1's 49.8,
  a factor of 3.16 and 4.63 against a +/-7% noise floor, paid for with a 1.5x to 2.1x more
  expensive write. Skipping the GROUP BY ... HAVING COUNT(DISTINCT tag) aggregate for a single-tag
  item restores predicate pushdown of the guard's boundary and halves the probe, 1,093 us to 556;
  a two-tag boundary costs roughly 200x a single-tag one, which is why tag_cardinality and
  most-selective-tag-first probing are requirements rather than tuning. Three pragma values are
  fixed — journal_mode WAL, synchronous NORMAL, and a finite busy timeout of 5,000 ms, which
  absorbed 64-way contention with zero SQLITE_BUSY. The runtime seam captures a tokio Handle at
  construction with try_current as fallback, so SqliteEventStoreError::NoRuntime keeps a real
  meaning. Query::index_arms() is rejected: it does not exist in happenstance-core, the
  decomposition stays adapter-private, and the re-open trigger is postgres-and-neon-stores
  independently needing it. Two subjects are recorded as non-verdicts with owners: ES-17's
  &[Event] marker is not lifted, because ADR-0012's falsifier item 1 requires two builds of the
  same adapter and no story in this project's map produces it; and CF-40's clause home stays open.
  The driver half of the queue row was stale on arrival — rusqlite without a pool was already
  settled — and is ratified rather than decided.
depends_on:
  - kb-decision-0012
  - kb-decision-0010
related:
  - kb-reference-append-condition-experiment-001
  - kb-open-question-cf-40-fixture-limits-ownership-001
source_paths:
  - .kb/_intake/0033-adr-0022-append-condition-strategy.md
  - references/adr/0022-append-condition-strategy.md
  - experiments/append-condition/
  - RUNBOOK.md
last_reviewed: 2026-08-16
---

# Staged: ADR-0022, the append-condition strategy

**Staged 2026-08-16** for the next `/redkiln:kb-ingest` wave. **Not an atom yet.**
The frontmatter above is *proposed*: a starting point for the wave to adjudicate
and author from, not a shape this document is asserting. `redkiln validate --kb`
skips `_`-prefixed directories, so nothing here is checked against the schema.

**Hand-written atoms are forbidden** (`CLAUDE.md`, *Where the work lives*): the
first attempt at that was reverted at `0269720`, because it produced the
directory layout of the process without the process. This file is the input to
the process, and `.kb/decisions/0022-append-condition-strategy.md` is the mount
point every downstream story loads once the wave has minted it.

**The handoff.** A human runs:

```console
/redkiln:kb-ingest
```

The wave should mint **two** atoms from the two files staged in this wave — this
decision, and the `reference` atom beside it — index the decision in
`.kb/maps/decision-map.md`, and clear both source files. A file still sitting in
`_intake/` after a run is a file that run did not ingest.

**Why the measurement is its own atom rather than a paragraph in this one.**
`.kb/decisions/README.md` keeps measurements out of decisions so that a decision
can be **superseded without invalidating the numbers underneath it**. The
precedent shape is `.kb/reference/position-visibility-experiment-2026-08.md`,
which ADR-0013 cites rather than swallows. This decision cites
`kb-reference-append-condition-experiment-001` in the same way.

---

## The long-form record

[`references/adr/0022-append-condition-strategy.md`](../../references/adr/0022-append-condition-strategy.md)
carries the whole of it — the question, the seven consequences with the
alternative that lost beside each, the two non-verdicts, the measured tables and
the falsifiers. It is **not** immutable and may be edited directly; the atom the
wave mints may not.

Sections a downstream story reads, in the order it will want them:

| section | what it decides |
| --- | --- |
| §4 | the append-condition strategy, and the SQL it is |
| §5 | the two that lost, with the figures and their conditions |
| §6 | the tag storage, with the figures |
| §7 | **migration 1 in full**, and what the published sketch gets wrong |
| §8 | the single-tag fast path, and why `tag_cardinality` is a requirement |
| §9 | the runtime seam, and the fate of `NoRuntime` under the option that won |
| §10 | `index_arms()` rejected, with its named re-open trigger |
| §11 | three pragma values, each with the alternative that lost |
| §12 | 64 contenders: supportable, measured, and **not applied** |
| §13, §14 | the two non-verdicts and their owners |
| §16 | the falsifier for every verdict above |

## What the wave must not do

- **Do not fold the measurement into the decision.** Two atoms, and the decision
  cites the reference.
- **Do not edit `.kb/decisions/0012-append-shape-and-preconditions.md`.** ES-17's
  marker does not lift here; §13 records why, quoting ADR-0012's falsifier item 1
  verbatim.
- **Do not edit `.kb/open-questions/cf-40-fixture-limits-ownership.md`.** §14
  cites it and leaves it open.
- **Do not mint a clause.** This record discharges none, amends none and moves no
  marker; `spec/SPECIFICATION.md` is untouched by the change that staged it.

## The gap this record escalates to the ADR queue

ADR-0012 names **phase 8** as the measurement that could lift `append`'s
`&[Event]` marker, and its falsifier item 1 requires *"two builds of the same
SQLite adapter differing only in `append`'s ownership, measured on the same
harness."* Three candidate stores in an experiment crate are not that, and **no
story in this project's map is currently assigned to produce it** — the four
implementation stories after this one build one adapter, not two builds of one.

That is a phase-8 obligation phase 8 as planned does not discharge, and it is
recorded here rather than absorbed. It needs a queue row of its own or an
explicit deferral with a new owner; either way it is a decision somebody has to
take rather than a silence.
