---
id: kb-open-question-dagger-convention-vs-maturity-markers-001
title: Is the dagger convention superseded by the maturity markers, now that the guard it switched is gone?
kind: open_question
status: accepted
authority_tier: note
summary: >-
  This is the surviving third of kb-open-question-no-ps-rule-name-resolved-001, which described a
  mechanism the tree no longer has. Check 4's loop guard — c.schedules_new || !has_suite(&c.id) —
  is retired and the schedules_new field is deleted, so nothing in spec-trace reads a dagger as a
  switch any more; measured over the real specification at 9b06836, the bare-dagger term had
  already been firing on zero clauses, and dropping it newly failed none. Sub-questions 1 and 2 of
  the predecessor are therefore answered, in kb-reference-spec-trace-unresolved-declarations-001.
  What is not decided is whether the dagger convention is superseded by the maturity markers the
  clauses already carry. The shape of that question has changed with the mechanism: no clause's
  Rule: line carries a hand-authored dagger any more, and all 59 daggers in spec/SPECIFICATION.md
  are either section 7.2's generated cells or the legend prose defining them. So the two markers
  now sit in adjacent columns of the same generated row answering different questions — does this
  rule exist, versus how settled is this clause — and whether that is redundancy or a division of
  labour is an ADR's to say, not a lane's. Forced by whoever next regenerates section 7.2's
  committed region.
depends_on:
  - kb-reference-spec-trace-unresolved-declarations-001
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-open-question-cf-17-cf-14-markers-001
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-repair-frozen-clause-001
  - kb-open-question-es-6-unwritable-rule-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/prose-guard-retired-and-what-it-owes.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
  - experiments/gate-vacuity/results/raw/spec-trace-drop-dagger.txt
supersedes:
  - kb-open-question-no-ps-rule-name-resolved-001
last_reviewed: 2026-09-07
---

# Is the dagger convention superseded by the maturity markers?

## Why this atom exists rather than an edit

Its predecessor, `kb-open-question-no-ps-rule-name-resolved-001`, was accepted while the dagger
was a *switch*: a bare `†` in a clause's `Rule:` line set `Rules::schedules_new`, and check 4's
loop continued past any clause where `c.schedules_new || !has_suite(&c.id)` held, resolving
nothing. That code is gone. Two of the predecessor's citations pointed at the guard expression and
at the parse site that set the field, and neither can be repointed by anchor, because the anchor is
what was deleted. Editing the body to describe deleted code is what
`kb-governance-referent-not-reasoning-001` forbids, so the predecessor keeps its body, takes
`status: superseded`, and this atom carries the one sub-question that survives.

## What is true today

The guard is retired and `schedules_new` is deleted. All that remains of it in
`xtask/src/spec_trace.rs` are three doc comments recording the removal: check 4's own
(`:793-815`), which explains that the sentence saying a rule was unwritten *was* the switch that
stopped the checker looking, and `Rules`' (`:2406-2422`), which notes the field had exactly one
reader and that deleting it also retired three terms — the dagger, `meta-test` and a leading
`new ` — that fired on nothing at all. `UNRESOLVABLE_RULE_NAMES` replaced it with a
per-`(clause, rule)` table, so a sentence about one name can no longer switch off the check for the
three beside it; the census that table now holds is
`kb-reference-spec-trace-unresolved-declarations-001`.

The predecessor's sub-questions 1 and 2 are answered by that measurement. The dagger was not doing
work `has_suite` could not: instrumenting `rules_of` over the real specification at `9b06836` shows
the bare-dagger term firing on **zero** clauses, and dropping it newly failing **zero** of the
seventeen daggered `PS` clauses — independently reproducing
`experiments/gate-vacuity/results/raw/spec-trace-drop-dagger.txt`. The term that actually did the
`PS` family's work was the backticked `` `new` ``.

The convention's shape has changed with the mechanism, and this is the part a reader of the
predecessor would get wrong. `spec/SPECIFICATION.md` carries 59 daggers and **none** of them is in
a clause's `Rule:` line; every one is inside §7.2's generated table or the legend prose defining
it (`spec/SPECIFICATION.md:9155-9164`). A `†` there is now the checker's own answer — *looked for
in `suite.rs`, and in the two `wire.rs` files for a `wire::`-qualified name, and not found* —
rather than an authored claim, and §7.2 deliberately refuses to print one where it could not
resolve the target, because a dagger there would assert an absence nothing checked.

## What is not decided

Whether the dagger convention is superseded by the maturity markers. On today's evidence the two
answer different questions in adjacent columns of the same generated row: `Maturity` says how
settled the clause is and names an owner, the `†` says whether the rule it cites exists. That reads
as a division of labour rather than a duplication — but it is a reading, not a decision, and the
predecessor's framing (a redundant marker naming no owner where the maturity marker does) was
written against a hand-authored dagger that no longer exists anywhere in the file. Whether the
generated marker should survive under its own name, be folded into the maturity column, or be
retired in favour of `UNRESOLVABLE_RULE_NAMES`'s three explicit kinds, is the question.

## What forces it

Whoever next regenerates §7.2's committed region. The open follow-up from the same lane — widening
`resolvable` to include `happenstance-core`'s `src/` and `tests/` items, which would turn 26
`Elsewhere` declarations into resolved names — moves the generated table, and the dagger's meaning
moves with it: a rule that is only `†` because no resolution source reads its file stops being one.
That edit and this question are the same pass.

## Ordered sub-questions

1. Does the generated `†` say anything the three `UNRESOLVABLE_RULE_NAMES` kinds do not already say
   with more precision?
2. If it does not, is the right removal deleting the column or widening `resolvable` first, so the
   column empties itself rather than being suppressed?
3. Does anything outside §7.2 still read a dagger as authored input — and if not, is the legend
   prose at `spec/SPECIFICATION.md:9155-9164` the only remaining edit the retirement needs?
