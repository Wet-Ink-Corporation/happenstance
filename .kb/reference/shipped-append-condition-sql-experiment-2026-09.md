---
id: kb-reference-shipped-append-condition-sql-001
title: The shipped intersection chain loses to the aggregate in nine of nine cells, and the seed ordering is 38x backwards
kind: reference
status: accepted
authority_tier: note
summary: >-
  Four multi-tag guard shapes built on one schema in experiments/shipped-append-condition-sql/,
  each conformance-cleared (356 passed, 0 failed across four arms x 89 rules) and
  checked byte-for-byte against a sqlite3_trace_v2 callback on a real
  SqliteEventStore. The shipped intersection chain loses to the GROUP BY
  aggregate ADR-0022 originally measured in nine of nine cells (1.54x-1.86x warm,
  11.8x cold), and its most-selective-tag-first seed ordering is measured
  38.1x-44.0x backwards on the two-tag case across two independent runs. A
  fourth arm, binding the boundary into every chained subquery rather than only
  the outer comparison, wins the commonest DCB path by up to 2,024x. I-2's
  proposed remedy (binding position > ? into the seed arm alone) is measured at
  +/-2-9% with no consistent sign and is falsified.
depends_on: []
related:
  - kb-decision-0022
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-append-condition-experiment-001
  - kb-open-question-query-plan-parameter-chunking-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/append-condition-sql-shape.md
  - experiments/shipped-append-condition-sql/
last_reviewed: 2026-09-07
---

# The shipped intersection chain loses to the aggregate in nine of nine cells, and the seed ordering is 38x backwards

## What this is a pointer to

The instrument, four guard-shape implementations, raw benchmark output and query
plans live in `experiments/shipped-append-condition-sql/`, outside the workspace
and the gate. This atom is the citable summary of what it measured against the
shipped `happenstance-sqlite` multi-tag guard. Which shape the adapter should
adopt, and whether ES-27's `[FROZEN]` `Rejects:` prose needs a clause-level
repair, are decisions for the atom that cites this one, not for this atom.

## Why a re-measurement, and what it checks itself against

ADR-0022 §8's original figures — "1,093 µs → 556" and "roughly 200x" — were
measured on `GROUP BY position HAVING COUNT(DISTINCT tag) = n`, a shape the
shipped adapter does not emit. The shipped builder
(`crates/happenstance-sqlite/src/query_sql.rs`) instead chains one
`position IN (SELECT position FROM event_tag WHERE tag = ?)` subquery per
additional tag, with no aggregate anywhere and no `position > ?` bound into any
arm. The experiment builds four shapes on one schema — the shipped chain, the
originally-measured aggregate, the chain with an inverted (least-selective-first)
seed, and the chain with the boundary bound into every arm — and verifies its
own transcription of the shipped shape is byte-for-byte what the real adapter
emits, via a `sqlite3_trace_v2` callback. All four shapes pass the full
conformance suite: 356 passed, 0 failed, four arms times 89 rules each.

## Chain against aggregate — nine of nine, one direction

`guard_us_median`, chain-as-shipped ÷ grouped-adr0022, across three log sizes
and three tag-distribution scenarios:

| scenario | events | chain µs | grouped µs | ratio |
| --- | ---: | ---: | ---: | ---: |
| accepted-2tag-at-head | 50,000 | 29,845 | 19,178 | 1.56 |
| accepted-2tag-at-head | 1,000,000 | 559,591 | 362,385 | 1.54 |
| rejected-2tag-unbounded | 50,000 | 25,204 | 14,085 | 1.79 |
| rejected-2tag-unbounded | 1,000,000 | 545,830 | 349,805 | 1.56 |
| rejected-2tag-midlog | 50,000 | 31,487 | 18,015 | 1.75 |
| rejected-2tag-midlog | 1,000,000 | 593,093 | 319,306 | 1.86 |

(Full nine-row table, including the 500,000-event row per scenario, is in the
raw results.) The chain loses in all nine cells measured, one direction. Cold
cache — the figure an application meets after a restart, at 10^6 events,
boundary 500,000 — widens the gap: chain `first_us=3,971,425` against the
aggregate's `first_us=335,498`, 11.8x cold against 2.6x warm in the same run.
The single-tag path is untouched by any of this: 4-16 µs at every size on all
four shapes, because the aggregate arm keeps the shipped fast path for
`tags.len() == 1`.

## Seed ordering — 38.1x to 44.0x, backwards

One 500,000-event store, one connection, one statement text with only bound
parameters swapped, alternating running order, every round asserting both
orderings return the identical position:

| scenario | most-selective-first µs | least-selective-first µs | ratio | replication |
| --- | ---: | ---: | ---: | ---: |
| accepted-2tag-at-head | 511,054 | 11,839 | 43.2x | 39.5x |
| rejected-2tag-unbounded | 553,328 | 13,340 | 41.5x | 41.5x |
| rejected-2tag-midlog | 634,239 | 14,399 | 44.0x | 38.1x |

The query plans show why: the chained subquery is uncorrelated, so SQLite
materialises it as a list subquery, and the shipped ordering places the
*larger* set there. The shipped requirement — probe most-selective-tag-first —
is measured inverted on the shape that ships.

## The fourth arm, and the falsified remedy

`chain-bounded-all-arms` binds the boundary into the seed and into every
chained membership subquery, rather than only the outer comparison. It wins the
commonest DCB path (read to head, append with `after = head`) by up to 2,024x
at 10^6 events against the aggregate, and beats the aggregate cold as well
(`first_us=236,988` against the aggregate's 335,498). It loses to the aggregate
only at `boundary = 0`, where the bound discards nothing. Separately, the
narrower remedy audit finding I-2 proposed — binding `position > ?` into the
guard's seed arm alone — was measured directly: +8.8%, -4.5%, +2.0%, -3.2%,
+2.6%, +1.6%, -7.2%, -2.9%, -7.4% across nine cells, no consistent sign, none
outside the run's own spread, and `EXPLAIN QUERY PLAN` shows the two plans
identical line for line. I-2's remedy is falsified.

## What this does not establish

Every figure is warm-cache, one host, two tags, one event type, with a
uniformly-spread selective tag; the experiment states this limitation itself
and none of the n≥3 tag case, a clustered selective tag, or cold cache as a
first-class column is measured. The write-side cost of maintaining
`tag_cardinality` (up to 32,768 upserts per append at declared ceilings) is
bounded arithmetically here, not measured, because all four arms share one
schema and all maintain the table identically. Which shape to adopt, and
whether that requires superseding part of `kb-decision-0022`, is left to the
decision record this atom feeds.
