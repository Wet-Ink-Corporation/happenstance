---
id: kb-open-question-query-plan-parameter-chunking-001
title: PARAMETER_BUDGET governs one of the two axes the query plan chunks on
kind: open_question
status: accepted
authority_tier: note
summary: >-
  happenstance-sqlite's query planner chunks on two independent axes and applies its parameter
  budget to one of them. crates/happenstance-sqlite/src/event_store.rs:577 declares
  PARAMETER_BUDGET = 30_000 and carries the arithmetic that derives a statement's row capacity from
  it; :760 is its only caller, sizing the tag-row insert. The other axis is UNION arm width, chunked
  through query_sql::chunks at a width of 400 on both the read path and the wide-guard path,
  documented as chunked and merged, never refused, because there is no MAX_QUERY_ITEMS to refuse
  against. Those two decompositions are computed independently, so a query whose arm count is
  inside the width and whose bound-parameter count is not has nothing bounding it. What is not
  decided is whether the parameter budget is owed on the arm-chunking paths as well — which would
  make the effective chunk width the minimum of an arm bound and a parameter bound — or whether the
  arm width of 400 is already conservative enough that the parameter bound can never bind first, in
  which case the answer is an assertion and a comment rather than two more callers. Nobody has
  measured which. Forced by phase 12, after which MIN_SUPPORTED_QUERY_ITEMS is a public promise a
  store must clear, and by any adapter raising the arm width.
depends_on: []
related:
  - kb-decision-0022
  - kb-decision-0015
  - kb-open-question-query-union-rule-unowned-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-09-04
---

# PARAMETER_BUDGET governs one of the two axes the query plan chunks on

## What is true today

`happenstance-sqlite`'s query planner has two places that decide how much of a query it can send
SQLite in one statement, and they are not the same computation.

The first is `PARAMETER_BUDGET`, declared at `crates/happenstance-sqlite/src/event_store.rs:577`
as `30_000`, sitting below `SQLITE_MAX_VARIABLE_NUMBER` (32,766) with headroom because, as the
constant's own doc comment says, the arithmetic that matters is done once in `happenstance-core`'s
`limits.rs` and an adapter binding one extra parameter per row should not be within rounding
distance of the wall. Its only caller is `write_tag_rows` at `:760`, which divides the budget by
`TAG_ROW_PARAMETERS` (3) to get `rows_per_statement` and chunks the `(tag, position, event_type)`
insert to that width. `PARAMETER_BUDGET` bounds exactly one statement shape: the tag-row write.

The second is arm width. `MAX_QUERY_ARMS_PER_STATEMENT` is `400` (`:277`), and both the ordinary
read path and the wide-guard path route through the single entry point `query_sql::chunks`
(`query_sql.rs:154`) to split a query's tag disjunction into UNION arms of at most that width,
merging the per-arm results back (`:622`, `:648`, `:1313`). The doc comment at `:270` is explicit
about the policy this implements: **chunked and merged, never refused** — there is no
`MAX_QUERY_ITEMS` an over-wide query is rejected against, because the chunker is supposed to
absorb it.

These two decompositions are computed independently. Arm-chunking sizes a query by how many `OR`
branches it has; parameter-chunking sizes a write by how many rows it inserts. A read query is never
routed through `write_tag_rows`, and a tag-row write never goes through `query_sql::chunks` — so
today, in the code that ships, the two never collide on the same statement. But the arm width of 400
is a count of disjuncts, not of bound parameters, and each arm can itself carry more than one bound
value (a tag plus a boundary, say). Nothing currently proves that 400 arms, at whatever parameters
per arm the shipped SQL uses, stays under 30,000 bound parameters — the two ceilings were set by
different reasoning at different times, and no comment or test ties them together.

## What is not decided

Whether the query-read path is missing a parameter check it happens not to need yet, or whether it
already can't need one. Two live answers, and they cost different things:

- The arm-chunking paths gain a second bound, so the effective chunk width becomes
  `min(arm_bound, parameter_bound)` — which is two more call sites doing the same kind of division
  `write_tag_rows` already does, and a rule the testkit could exercise once a store's arm width is
  configurable enough to make it bind.
- The arm width of 400 is asserted, with the arithmetic shown in a comment, to always keep the
  per-statement parameter count under `PARAMETER_BUDGET` for every SQL shape the adapter emits —
  in which case the answer is documentation, not code, and the two constants stay independent on
  purpose.

Nobody has measured which shipped SQL shape has the most parameters per arm, so nobody has checked
whether 400 arms can ever reach 30,000 parameters on that shape. This is the same kind of gap
`kb-open-question-query-union-rule-unowned-001` names for the union-chunking conformance rule
itself — that question is whether a rule exists at all to check chunking behaves as documented;
this one is narrower, about a second bound the existing chunking may or may not need.

## What forces it

Phase 12, when `MIN_SUPPORTED_QUERY_ITEMS` (documented today at 128 alongside the 400 chunk width)
becomes a promise a published store must clear rather than an internal constant — at which point an
adapter's chunking arithmetic is something a downstream crate can rely on rather than something
this workspace can quietly change. It is also forced sooner by any adapter, in this workspace or
outside it, that raises the arm-chunk width above 400: that is the change most likely to make the
parameter bound the tighter one, and whoever makes it needs to know which of the two questions
above they are answering.
