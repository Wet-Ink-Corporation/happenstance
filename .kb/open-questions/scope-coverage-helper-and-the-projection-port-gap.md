---
id: kb-open-question-scope-coverage-helper-projection-gap-001
title: A scope-coverage test helper for DomainEvent::tags vs DecisionModel::scope is endorsed but deliberately unscheduled, and the projection port has the identical unchecked pair
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0020 made it unwritable for a DecisionModel's fold to interpret an event type its own query
  never selected, but it left an adjacent hole open on tags: scope() and tags() are declared
  independently by two traits and nothing in the workspace relates them, so a model whose events
  under-declare its scope folds on a narrower log than the handler believes while its append
  condition, built from the same under-scoped query, matches nothing and becomes silently
  unconditional. The record resolving this (P-2, tags-scope-agreement) recommends a run-time refusal
  in commit_with plus fixing two rendered doctests that currently teach the violation, and endorses
  but explicitly declines to bundle a third, additive test helper (assert_scope_covered, an
  enum-total sibling to the existing assert_domain_event) because it would add a second public
  function to happenstance::testing while a separate open question about that module's remit is
  still unsettled, and adding the sibling first would hand that question a two-function surface to
  rationalize rather than a design to evaluate. Separately and not touched by the recommended fix at
  all: happenstance's projection runner derives its query from Projection::scope() against
  DomainEvent::tags() by the identical two-independent-declarations pattern, on a port ADR-0008
  established should share one derivation with the command path — after the recommended fix lands,
  the command path is checked and the projection path is not, which is a visible asymmetry against
  that expectation and is left unaddressed because it was outside the writable surface of the lane
  that produced the fix.
depends_on: []
related:
  - kb-decision-0020
  - kb-decision-0008
  - kb-open-question-d-1-no-total-path-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/tags-scope-agreement.md
last_reviewed: 2026-09-07
---

# A scope-coverage test helper is endorsed but unscheduled, and the projection port has the identical unchecked pair

## What is true today

ADR-0020 closed one half of a two-halved problem: a DCB handler names its
event set twice, once in the query and once in the fold, and the decision
made `EVENT_TYPES`/fold agreement unwritable by construction — the fold is
exhaustive over the same sealed enum the query derives from. It never touched
the tag half. `DomainEvent::tags()` and `DecisionModel::scope()` are declared
independently, on two different traits, and nothing in the workspace relates
one to the other, even though `scope()`'s own doc comment claims "the tags
every event inside this boundary carries" — a claim about `tags()`, written on
`scope()`, enforced nowhere. A model whose events under-declare the scope
lands exactly on the row ADR-0020's own long form names as the corrupting
direction: the fold reads a narrower log than the handler believes, and the
append condition built from that same query is a lost update with no
diagnostic, because it protects less than the code assumes. Measured: eight
rendered doctests in the workspace implement `tags()`, all return
`Tags::empty()`, and two of them — on `DecisionModel`'s own documentation page
and on the composition page — pair that empty-tags implementation with a
non-empty `scope()` on the same page, teaching the exact violation to a reader
following the crate's own examples. Both worked examples in the repository
(`course-subscriptions`, `transfers-on-sqlite`) get it right, which is part of
why this reads as a gap rather than a misunderstanding.

The record resolving this — after two rounds of adversarial review each
falsified a premise the previous draft rested on — recommends a run-time
check in `commit_with` (reusing the existing `Query::matches` predicate, so no
second filter vocabulary is invented) plus repairing the two teaching
doctests. It separately identifies, evaluates, and **endorses on the merits**
a third fix: an additive `assert_scope_covered` helper alongside the existing
`assert_domain_event`, which would be enum-total (checking every variant an
author hands it, at authoring time, rather than only the variants one
decision happened to emit) and would reach the per-member grain a
union-query runtime check structurally cannot, since `Boundary`'s derived
query for a composite model is a union and passing it proves an event is
selected by *some* member, never by the specific member whose invariant
motivated the check.

## What is not decided

Whether and when the additive helper is actually added. It is deliberately
**not bundled** with the recommended fix, on the ground that it would be a
new public function in `happenstance::testing` — a module in the crate's
published default feature set — landing while a separate, unrelated question
about what `assert_domain_event` and its module are *for* remains open; adding
a second helper first would hand that question a two-function surface to
justify after the fact rather than a live design question to answer on its
own terms. Separately, and not resolved by any option the record considers:
`happenstance`'s projection runner has the structurally identical pair —
`Projection::scope()` against `DomainEvent::tags()`, feeding the same
`derive_query` function the command path uses — and the recommended
`commit_with` check does not reach it, because the runner has no append
condition and so the same failure surfaces differently (a projection silently
applies nothing while its own checkpoint still advances, rather than a lost
update). ADR-0008 established one derivation shared by both ports as the
standing expectation; leaving one port checked and the other not is a
departure from that expectation that the record names explicitly rather than
resolving, because it sat outside the writable surface of the lane that
produced the fix.

## What forces it

The additive helper's own sequencing condition is stated by the record
itself: it should land after the separate question governing
`happenstance::testing`'s remit is settled, and after the related question
about what an empty decision's outcome type looks like
(`then-empty-emission-idiom-and-the-nothing-to-do-channel.md`), because that
outcome shapes what a coverage helper's second intended use — giving an
under-tagged model's empty-fold class a diagnostic that actually names tag
coverage, rather than the currently misattributed "an append must contain at
least one event" — would need to look like. The projection-port asymmetry has
no forcing event named; it is recorded so that whoever next touches
`runner.rs`'s query derivation, or next revisits ADR-0008's one-derivation
expectation, finds it written down rather than rediscovers it.

## Ordered sub-questions

1. Does the module-remit question get settled first, and does its answer
   determine whether `assert_scope_covered` is the right *shape* for the
   helper, or whether the remit question reshapes `assert_domain_event`
   itself in a way the sibling would have to match?
2. Once the empty-decision outcome question settles, does the coverage
   helper's diagnostic get wired to name tag-coverage failures specifically,
   distinguishing them from the ordinary "forgot to push" case the outcome
   type does not otherwise distinguish?
3. Does the projection port get the same run-time check `commit_with` gained,
   a different remedy suited to its different failure mode (silent
   no-op-with-advancing-checkpoint rather than lost update), or a stated
   decision that the two ports are allowed to diverge here — and who owns
   raising that as its own decision record given ADR-0008's expectation?
