---
id: kb-decision-0052
title: Both query-partition constants stay public, and planned_statement_count counts the real partition
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0052
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  MAX_QUERY_ARMS_PER_STATEMENT and the new MAX_QUERY_PARAMETERS_PER_STATEMENT both stay public: a
  caller doing the arithmetic needs both, and the argument against — that each is a promise about
  this build of bundled SQLite — applies equally to the constant that was already public.
  planned_statement_count now reports the two-axis partition rather than arms alone; that is a
  changed meaning of an existing public function, invisible to cargo-semver-checks, and free only
  because the crate had never really been released.
depends_on: []
related:
  - kb-decision-0022
  - kb-open-question-query-plan-parameter-chunking-001
  - kb-open-question-workerd-runner-absent-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/query-partition-public-surface.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# Both query-partition constants stay public, and planned_statement_count counts the real partition

## Decision

`happenstance-sqlite`'s query-arm partition became two independent ceilings
rather than one, and this settles the two public-surface consequences of that
fix: `MAX_QUERY_ARMS_PER_STATEMENT` (`SqliteEventStore::MAX_QUERY_ARMS_PER_STATEMENT
= 400`, unchanged) stays public alongside a new
`MAX_QUERY_PARAMETERS_PER_STATEMENT` (bound to SQLite's compiled parameter
limit), and `planned_statement_count` changes what it counts — the number of
statements the read path will actually prepare — rather than what it used to
count, the arm chunking alone. Ratified as `1A`/`2A`.

## Why two ceilings, and why both are public

Before the fix, 400 items at one tag each partitioned correctly at 400 arms, but
the same 400 items at the maximum tags-per-event partitioned as a plan of `1`
statement of 51,200 bound parameters — a statement `prepare` refuses, against a
compiled limit of 32,766. `planned_statement_count` reported that query as
fitting in one statement when it could not run at all. The two axes are
independent: at one tag per item, 900 items is 900 parameters and nowhere near
the parameter ceiling, so the arm limit is the only thing that splits the
query — it is not decorated over by the parameter axis, and a standing test
guards specifically against a fix that swapped one limit for the other instead
of keeping both.

Both constants are public because a caller — chiefly a test computing an
under/at/over-boundary case rather than hard-coding a literal that silently
stops crossing the boundary when a width changes — needs to do arithmetic over
both, and `planned_statement_count` alone gives a count with no gradient: it
cannot say how much room is left or which axis a query is closer to. The
strongest argument against publishing either is that each is a promise about
*this build* of bundled SQLite: `SQLITE_MAX_COMPOUND_SELECT` and
`SQLITE_MAX_VARIABLE_NUMBER` are both compile-time constants of the linked
library, not general facts about SQLite, so freezing either at `0.2.0` freezes
a fact about a dependency's build configuration onto this crate's surface,
foreclosing free exploitation of a future `rusqlite` with a raised limit. That
argument is accepted at face value and applies with equal force to the
constant that was *already* public before this fix — it is an argument against
the status quo ante, not one this decision introduces, and it was outweighed by
the cost of a test that cannot compute its own boundary.

## Why planned_statement_count changed meaning rather than gaining a sibling

The function's own documentation says it reports "the same call the read path
makes." Keeping that sentence true, rather than adding a second function
alongside a name that continues to describe an abandoned plan, is why the
existing signature now returns the two-axis count instead of the arm-only
count. This is stated plainly as a different kind of change from everything
else this fix put on the public surface: two new constants are additive and
cost a reader nothing to ignore, while this is **a changed meaning of an
existing public function with the same name, same signature, same
visibility** — invisible to `cargo-semver-checks`, because the surface itself
did not move.

A caller sizing a decision model against the adapter is strictly better off:
the old answer of `1` for a 51,200-parameter plan was actively wrong, describing
a statement that cannot run. A caller who had been deriving the arm width back
out of the old number (`planned_statement_count(&q) * MAX_QUERY_ARMS_PER_STATEMENT`
as an upper bound on items) is broken silently for wide-item queries, with no
error — the repair is to read `MAX_QUERY_ARMS_PER_STATEMENT` directly rather
than infer it, which is exactly why that constant stays public under this same
decision. This is free only because `happenstance-sqlite` has never carried a
released version with the old meaning: the registry holds only a `0.0.0`
name-reservation placeholder, so no real caller exists to be broken. That
ends at first stable publish, after which the same change could not be made
without a version bump no tool would catch as necessary.

## What this does not decide

Whether the two shipping adapters (`happenstance-sqlite` and
`happenstance-cloudflare`) are *required* to agree on their declared query
ceilings, as opposed to happening to publish the same numbers today. Whether
`store_evaluates_a_query_at_the_guaranteed_minimum_item_count` in the testkit,
which currently exercises only the arm axis, should be widened to cross the
parameter axis too — a testkit change with its own release-timing question.
Whether `happenstance-cloudflare`'s widths should be lower to reflect an
isolate memory ceiling nothing has yet measured a wall for.
