---
id: kb-open-question-read-to-backwards-limit-composition-001
title: read_to composes with limit going forwards only; the backwards-plus-window-plus-budget case is unwritten on purpose
kind: open_question
status: accepted
authority_tier: note
summary: >-
  read_to_composes_with_limit exercises a forwards read with an upper bound and a row budget
  together, but not the combination of backwards, a window (both from and to bounds present) and
  a LIMIT budget in the same read. A wrong implementation combining all three is constructible — a
  windowed statement written ascending with the LIMIT inside it and direction applied only by an
  outer ORDER BY, which returns the wrong end of the window under a tight budget — and it was
  deliberately not asserted, because doing so in this rule would make three already-registered
  mutants (BackwardsToIsAnUpperBoundStore, BackwardsIgnoredStore, FetchOneExtraStore) fail it for
  reasons three other rules already own, muddying which rule caught what. Whether that composition
  deserves a fourteenth read-option rule of its own, or is an accepted gap, is unsettled and is the
  rule set owner's call. It is a residual from the same lane that closed Kind::ModelOnlyMutant's
  only member (read_to_composes_with_multi_item_query) and has no other home. Forced before the
  first SQL adapter ships a windowed, direction-sensitive, budget-limited query — happenstance-sqlite
  and happenstance-postgres are the two in the workspace where this shape is buildable at all.
depends_on: []
related:
  - kb-decision-0010
  - kb-open-question-model-only-kind-memberless-001
  - kb-decision-0022
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/model-only-kind-has-no-members.md
last_reviewed: 2026-09-07
---

# read_to composes with limit going forwards only; the backwards-plus-window-plus-budget case is unwritten on purpose

## What is true today

The event-store read-option conformance rules cover several two-way compositions of `Query`,
direction, bounds and `limit`, and `read_to_composes_with_limit` is one of them: a read with an
upper bound (`to`) and a row budget together, run forwards. That rule was written and landed in
the same change that closed `Kind::ModelOnlyMutant`'s only member
(`read_to_composes_with_multi_item_query`, see `kb-open-question-model-only-kind-memberless-001`),
because both residuals came out of the same pass over `read_to`'s composition surface.

What the rule does not cover, and was not written to cover, is the **three-way** composition:
backwards direction, a full window (both a `from` and a `to` bound present), and a `limit` budget,
all in one read. The wrong implementation this would need to reject is constructible and was
described rather than built: a statement written as an ascending, windowed `SELECT … LIMIT ?`
with direction applied only by an outer `ORDER BY … DESC` re-sort of the already-limited rows —
which returns the wrong end of the window whenever the budget is tighter than the window's size,
because the row selection happened before the direction was applied.

## Why it was not asserted

Three mutants already registered in the suite —
`BackwardsToIsAnUpperBoundStore`, `BackwardsIgnoredStore`, and `FetchOneExtraStore` — each
already fail on a narrower composition that this three-way rule would also catch. Writing the
rule as described would make all three fail it too, for reasons that are already each mutant's
own rule's job to report. The lane judged that outcome worse than the gap: a reader diagnosing a
failing gate would see one new rule reject three already-explained mutants and have to
re-establish which existing rule was the "real" one for each, rather than the new rule adding
signal. CLAUDE.md's own standard for a conformance rule — name a plausible wrong implementation it
rejects that nothing else already rejects — argues the same way once the three existing rejections
are accounted for.

## What is not decided

Whether the backwards/window/budget composition is worth a **fourteenth** read-option rule
written narrowly enough to avoid double-rejecting the three existing mutants (for instance, by
constructing a fourth wrong implementation that passes all three existing rules and fails only on
this composition), or whether the gap is accepted permanently on the ground that the three
existing rules already provide adequate coverage of every wrong implementation anyone has found in
this space. Nobody has proposed a fourth mutant that separates cleanly from the existing three,
which is itself weak evidence for the second answer — but the search for one has not been
seriously attempted, only judged not worth attempting yet.

## What forces it

The first SQL adapter that ships a windowed, direction-sensitive, budget-limited read against a
real store where the statement shape matters — `happenstance-sqlite` (ADR-0022's append-condition
SQL strategy already lives in this crate) and `happenstance-postgres` are the two candidates in
the workspace. An adapter author hand-writing that statement is exactly the person who would reach
for the ascending-then-resort shape this gap describes, and exactly the person with no rule
telling them it is wrong.
