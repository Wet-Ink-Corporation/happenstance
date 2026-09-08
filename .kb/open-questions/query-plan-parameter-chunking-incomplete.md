---
id: kb-open-question-query-plan-parameter-chunking-001
title: PARAMETER_BUDGET governs one of the two axes the query plan chunks on
kind: open_question
status: superseded
authority_tier: note
summary: >-
  happenstance-sqlite's query planner chunked on two independent axes and applied its parameter
  budget to one of them. crates/happenstance-sqlite/src/event_store.rs declared
  PARAMETER_BUDGET = 30_000 and carried the arithmetic deriving a statement's row capacity from
  it; its only caller was the tag-row insert. The other axis was UNION arm width, chunked through
  query_sql::chunks at a width of 400 on both the read path and the wide-guard path, documented as
  chunked and merged, never refused, because there is no MAX_QUERY_ITEMS to refuse against. Those
  two decompositions were computed independently, so a query whose arm count was inside the width
  and whose bound-parameter count was not had nothing bounding it. What was not decided was whether
  the parameter budget was owed on the arm-chunking paths as well — making the effective width the
  minimum of an arm bound and a parameter bound — or whether 400 arms was already conservative
  enough that the parameter bound could never bind first, in which case the answer was an assertion
  and a comment rather than two more callers. Resolved 2026-09-04 by the X-1 lane and ratified
  2026-09-07 by ADR-0052 (kb-decision-0052), on a measurement rather than a preference: 400 items
  at MAX_TAGS_PER_EVENT tags apiece is 400 arms and 51,200 bound parameters against
  SQLITE_MAX_VARIABLE_NUMBER's 32,766, so the conservative-enough branch is refuted rather than
  merely unchosen. query_sql::partition is now greedy and order-preserving over both limits and
  takes a per-arm extra cost from its caller; MAX_QUERY_PARAMETERS_PER_STATEMENT is public beside
  MAX_QUERY_ARMS_PER_STATEMENT and planned_statement_count reports the two-axis partition.
  tests/wide_tags.rs guards both directions, including the narrow-item case where the arm bound is
  still the only thing that splits. Superseded rather than withdrawn: the lane also found a third
  failure this atom did not name — Selectivity::read_for, unpartitioned and failing first, inside
  BEGIN IMMEDIATE — and that half of the repair is contingent on the ADR-0022 §8 arm-shape record,
  which is kb-open-question-adr-0022-falsifiers-fired-001's.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0015
  - kb-decision-0052
  - kb-decision-0053
  - kb-reference-shipped-append-condition-sql-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-query-union-rule-unowned-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/remediation-2026-09-04-briefs/query-partition-public-surface.md
  - .kb/_intake/remediation-2026-09-04-briefs/append-condition-sql-shape.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-07
---

# PARAMETER_BUDGET governs one of the two axes the query plan chunks on

## What was true when this was asked

`happenstance-sqlite`'s query planner had two places that decided how much of a query it could send
SQLite in one statement, and they were not the same computation.

The first was `PARAMETER_BUDGET`, `30_000`, sitting below `SQLITE_MAX_VARIABLE_NUMBER` (32,766)
with headroom because the arithmetic that matters is done once in `happenstance-core`'s `limits.rs`
and an adapter binding one extra parameter per row should not be within rounding distance of the
wall. Its only caller was `write_tag_rows`, which divides the budget by `TAG_ROW_PARAMETERS` (3) to
get `rows_per_statement` and chunks the `(tag, position, event_type)` insert to that width. It
bounded exactly one statement shape: the tag-row write.

The second was arm width. `MAX_QUERY_ARMS_PER_STATEMENT` is `400`, rationalised by
`SQLITE_MAX_COMPOUND_SELECT`'s 500 terms, and both the ordinary read path and the wide-guard path
route through the single entry point `query_sql::chunks` to split a query's tag disjunction into
UNION arms of at most that width, merging the per-arm results back. The policy is explicit:
**chunked and merged, never refused** — there is no `MAX_QUERY_ITEMS` an over-wide query is
rejected against, because the chunker is supposed to absorb it.

These were computed independently. Arm-chunking sizes a query by how many `OR` branches it has;
parameter-chunking sized a write by how many rows it inserts. Nothing tied 400 disjuncts to the
number of bound values those disjuncts carry, and no comment or test connected the two ceilings.

## What was not decided, and what decided it

Two live answers: give the arm-chunking paths a second bound, so the effective width becomes
`min(arm_bound, parameter_bound)`; or assert that 400 arms can never reach 30,000 parameters for
any SQL shape the adapter emits, in which case the answer is documentation and the constants stay
independent on purpose. Nobody had measured which shipped shape carries the most parameters per
arm.

The measurement settled it against the second answer. One item of one tag is one arm and one
parameter, so 400 narrow items is nowhere near the budget; the same 400 items at
`MAX_TAGS_PER_EVENT` tags apiece is still 400 arms and **51,200** bound parameters, past
`SQLITE_MAX_VARIABLE_NUMBER` — reached at the adapter's own documented chunk width carrying the
adapter's own documented maximum tag count, and recorded in
`kb-reference-shipped-append-condition-sql-001`'s instrument. `planned_statement_count` reported
that plan as `1` and `prepare` refused the statement it counted.

So `chunks` takes `max_arms` and `max_parameters`, and `partition` is greedy and order-preserving
over both. The per-item cost is one bound parameter per distinct tag and one per type. The fourth
argument, `per_arm_extra`, is what the *caller's* shape spends on every arm beyond the item itself:
the guard path binds its boundary once per arm and passes `1`, `page_statements` binds `lo` and
`hi` and passes `2`, and it subtracts the compound's per-chunk `LIMIT` from the budget before
calling rather than pricing it per arm — because a per-chunk cost is not a per-arm one, and folding
them together is how a ceiling ends up in the wrong unit. `PARAMETER_BUDGET` keeps its original
caller and gains a second: `MAX_QUERY_PARAMETERS_PER_STATEMENT` re-exports it publicly, which is
`kb-decision-0052`.

The arm width did not become decorative, and `tests/wide_tags.rs` says so in both directions:
`the_parameter_partition_splits_one_parameter_over_the_budget_and_not_before`, and
`the_arm_partition_still_binds_on_narrow_items`, whose regression is a partition that swapped one
limit for the other instead of taking both.

## What this atom got wrong, and what rides along

This atom framed the gap as two ceilings that never meet on one statement — a read query is never
routed through the tag-row write, and a tag-row write never goes through `chunks` — with the only
worry being that arms and parameters might one day collide there. That understated the defect. A
third site failed *earlier* than either: `Selectivity::read_for` accumulated every
distinct tag of every multi-tag item across the whole query into a single `IN (…)` and was not
partitioned at all. It needs no wide item to break (16,500 two-tag items is 33,000 parameters), and
it runs before `chunks` on both callers — including the guard path, with the write lock held.

That half of the repair is the one that does not stand on its own. The ADR-0022 §8 record asked for
this lane to be sequenced *after* it and the lane ran first; the `chunks` half survives that, because
the partition arithmetic is one parameter per tag and one per type whether the arm is an
intersection chain or an aggregate, and 400 is a fact about `SQLITE_MAX_COMPOUND_SELECT` either way.
The `read_for` half is contingent: under the aggregate option there is no `Selectivity::read_for` to
chunk, and the width given to it is discarded with it. That residual belongs to
`kb-open-question-adr-0022-falsifiers-fired-001`, not here.

Two things this did not settle, both named rather than absorbed. The testkit still cannot reach
either axis: VT-23's rule builds 128 items at one tag each — 128 parameters — while its own
assertion speaks of an adapter sending only its first chunk of bound parameters, and widening it
across the tag axis is a testkit change every adapter takes involuntarily. That sits with
`kb-open-question-query-union-rule-unowned-001`, the nearest owner of an unwritten chunking rule.
And phase 12 still turns `MIN_SUPPORTED_QUERY_ITEMS` into a promise a published store must clear;
what changed is that the adapter's arithmetic is now honest before that deadline rather than after.
