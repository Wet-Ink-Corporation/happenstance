---
id: kb-decision-0047
title: An under-tagged model is refused at commit, not silently admitted
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0047
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  commit_with rejects a commit whose decided events are not matched by the boundary's own
  derived query, reusing the Query::matches predicate absorb already calls rather than
  inventing a second filter vocabulary. This closes the lost-update direction ADR-0020 named by
  a different route — DomainEvent::tags and DecisionModel::scope were declared independently
  and nothing related them — and between it and ES-20's empty-batch refusal no write from an
  under-tagged model can land. Two doctests that taught the violation were repaired; the
  additive scope-coverage helper was held back.
depends_on:
  - kb-decision-0020
related:
  - kb-decision-0046
  - kb-decision-0008
  - kb-open-question-scope-coverage-helper-projection-gap-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/tags-scope-agreement.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
last_reviewed: 2026-09-07
---

# An under-tagged model is refused at commit, not silently admitted

## Decision

Between `command.rs:305` and `:311`, `commit_with` now rejects a commit when a decided event's
`(event_type, tags)` is not matched by the boundary's own derived query — the same check
`Boundary::absorb` already performs on events coming back from the store, via
`Query::matches` (`crates/happenstance-core/src/query.rs`). No second filter vocabulary is
invented; landing this without reusing `matches` is exactly the "second place the event set is
named" objection ADR-0020's own design comments raise against itself. Two rendered doctests that
returned `Tags::empty()` from `tags()` while their model's `scope()` was non-empty — teaching the
violation on `DecisionModel`'s own page and on the composition page — were repaired in the same
change. Landed at `0ae10ab`, in one commit with `kb-decision-0046`.

## What gap this closes, and why ADR-0020 did not already close it

ADR-0020 (`kb-decision-0020`) made it unwritable for a fold to interpret an *event type* the
query never selected: `EVENT_TYPES` is declared once, the fold is exhaustive over the same enum,
and the query is derived from it on a sealed trait the caller cannot override. It never touched
*tags*: `Boundary::query` derives the query's tag half from `DecisionModel::scope()`, the store
matches it against `DomainEvent::tags()`, and the two are declared independently, in two traits,
by two separate methods, with nothing in the workspace relating them. A model whose events do
not carry its own scope reads an empty fold on every attempt while the append condition built
from that same query matches nothing — present, evaluated, and unconditional. That is the exact
state `happenstance-core`'s own `AppendCondition` documentation already names and seals by the
*other* route (a query with zero items): "a conditional append that is silently unconditional,
which is a lost update with no diagnostic anywhere." The zero-item route was already structurally
unreachable; the tag-mismatch route reached the identical state and was wide open until this
decision. `kb-decision-0020` is `accepted` and immutable and is neither amended nor superseded:
its Decision section settled `scope()`'s signature and never the coverage relation, so this is a
new record with 0020 as its parent rather than its target.

## Why an empty decision was never the risk this closes

The check only sees events a decision actually emitted; an under-tagged model whose correct
answer is "emit nothing" folds empty and decides empty, and that class never reaches this check
at all. It does not need to: `kb-decision-0012` (transcribing ES-20 `[FROZEN]`) already refuses
an empty batch before any condition is evaluated, so an empty decision cannot land regardless of
whether its tags agree with its scope. The corrupting direction ADR-0020 named — an append that
lands while the condition protects less than the handler assumes — requires an append to land at
all, and every decision that can land is a decision this check inspects. Between the two rules,
no write produced by an under-tagged model can land. What the empty-decision class loses instead
is a diagnostic, not a write: the caller still reads "an append must contain at least one event"
without any mention of tag coverage, which is `kb-decision-0046`'s subject, not this one's.

## What was held back, and what remains open

The recommendation reused `Query::matches`, which for a composite boundary is a union: an event
selected by *any* member of a tuple model passes the check, never necessarily by the member whose
invariant motivated it, so the guarantee this decision buys is real but grain-limited on
composites — a stronger per-member check is possible only if `Boundary` gains a way to enumerate
its members, which it does not today. An additive, enum-total test helper
(`assert_scope_covered`, quantifying over every variant of a `DomainEvent` the way
`assert_domain_event` already quantifies over `EVENT_TYPES`) was endorsed on its merits and
deliberately not bundled into this change, because it adds a second public helper to
`happenstance::testing` while what that module's remit should be is still an open question, and
because its second use — giving the empty-decision class a diagnostic that names tag coverage —
only becomes concrete once `kb-decision-0046`'s outcome shape existed to hang it on. `ADR-0008`
(`kb-decision-0008`) states "one derivation, both ports" as the standing expectation, and this
decision reaches only the command path: `runner.rs`'s identical `Projection::scope()` /
`DomainEvent::tags()` pair is untouched, so the two ports are now visibly unequal, named as an
open question rather than resolved here.
