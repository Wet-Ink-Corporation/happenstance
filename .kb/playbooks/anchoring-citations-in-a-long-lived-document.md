---
id: kb-playbook-anchoring-citations-001
title: Anchoring citations in a document whose targets move under it
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How to check file:line citations against a source tree that moves, without a checker that
  either passes forever or fires on every ordinary edit. Two properties decide the design — it
  must detect a moved target, and inserting lines above a cited item must not red the gate — which
  rules out a content hash and an exact line match and leaves a windowed search for a short subject
  string. Compares the explicit anchor spelling against the derived one with the cost of each,
  records four measured attempts that took the false-report rate from 118/316 to 2/262, and states
  the discriminator that made it work: a heuristic that cannot tell its own mistakes from the
  corpus's must decline rather than guess.
depends_on:
  - kb-playbook-verify-referent-report-coverage-001
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-reference-phase-8-spec-reconciliation-001
  - kb-playbook-declared-page-need-001
  - kb-playbook-assert-execution-not-discovery-001
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/lesson-anchoring-citations-in-a-long-lived-document.md
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - references/evaluation/phase-7-contract-defects.md
  - references/evaluation/README.md
  - xtask/src/spec_trace.rs
  - xtask/src/lint_constitution.rs
  - spec/SPECIFICATION.md
  - standards/rust/README.md
  - references/evaluation/phase-4-5-reconciliation.md
  - references/evaluation/review-citation-drift.md
last_reviewed: 2026-09-04
---

# Anchoring citations in a document whose targets move under it

This is the mechanism `kb-playbook-verify-referent-report-coverage-001`'s first obligation —
verify the referent, not just the address — is discharged by in this repository, against a
document whose 358 `file:line` citations sit over a source tree that moves on every commit.

## Two properties any solution must have

- **P1 — it must detect a moved target.** A function that moves far away, citation left behind,
  must red the gate.
- **P2 — inserting lines above a cited item must not red the gate.** This is the property that
  decides the design. A growing doc comment, an added import, a clause inserted upstream — none
  of these invalidate a claim, and a check that fires on them trains a reflex nobody reads,
  which launders drift into the document rather than catching it.

A content hash fails P2 outright, as does an exact line match. What satisfies both is a
**windowed search for a short subject string** around the cited range: it survives insertion up
to the window width and fails when the item genuinely moves.

## Two spellings of an anchor, and their costs

**Explicit** — what `standards/rust/` does: the citation itself carries the anchor,
`` `path:line (anchor)` ``, checked by `parse_citation`/`check_citations`
(`xtask/src/lint_constitution.rs:690-735`) with `ANCHOR_SLACK = 10`. An unparseable citation is a
hard failure. Its cost is real: every existing site would need editing to carry an anchor before
the check could be whole. Right when a corpus is being authored; an expensive retrofit onto one
that already exists.

**Derived** — what `spec_trace` does: the specification's citation idiom already places a
backticked identifier beside the citation, so the anchor is derived from the surrounding prose at
no per-site cost (`fn subject_before`, `xtask/src/spec_trace.rs:2997-3043`). Coverage went from
84 parsed to 358 checked with no edits to the 358 sites. Neither spelling is universally right;
explicit buys certainty at authoring cost, derived buys free coverage with a lower ceiling.

## Four measured attempts

Walking back up to four backtick spans to find a subject over-reached: **118 false reports out of
316 anchored**. Two refinements — reach back only one line, and only the immediately preceding
span — took it to **70 of 262**; a strict-adjacency rule to **10**; an absent-subject
discriminator to **2**, both of which were real defects. The transferable shape, not the numbers:
nearly every false report was one sentence pattern with no backticked subject at all, where
reaching back far enough always finds *some* identifier belonging to the previous sentence.

Three refinements did the work:

- **Reach back at most one line**, and only the span immediately before the citation
  (`xtask/src/spec_trace.rs:2988-2990`) — the document wraps at 80 columns, so a one-line
  reach-back is necessary and two is already a guess.
- **Decline `.md` targets entirely** — a citation into Markdown supports a passage, not a
  definition site, and anchoring them produced a third of the check's first-run false reports.
- **Search the whole cited range**, not just its start — a span like `404-427` may place its
  subject anywhere inside it; the window is `[line - (SLACK+1), line_end + SLACK]`.

## The discriminator that made it work

> If the subject appears nowhere in the cited file, the derivation picked the wrong word. If it
> appears in the file but far from the cited line, that is drift. Only the second is worth
> reporting.

This is the load-bearing idea and it generalises past citations: **a heuristic that cannot tell
its own mistakes from the corpus's mistakes must decline, not guess.** The same commitment shows
in smaller rejections — a span that is not a bare identifier is declined rather than cleaned up,
`Type::method` anchors on the last segment, and names under four characters or starting
upper-case are declined, because a type name is a poor anchor even when it is the grammatical
subject.

## The cost, and where it stops holding

Of 358 citations checked, 69 are anchored to their subject; the other 289 are verified for
addressing only — the direct cost of declining wherever derivation is uncertain, printed rather
than hidden. Deriving an anchor from prose is worth it only where derivation is certain; where it
is not, paying the explicit-spelling retrofit is a real option, not a defeat.

**Line-numbered citations across documents are a standing tax**, not a one-time cost: every edit
to `crates/**` or `xtask/**` during this pass broke citations in `standards/rust/`, three
separate times, each caught by `cargo xtask lint-constitution`. The check works; the tax should be
priced in before adopting line-numbered cross-references at all. An anchor-only citation with no
line number avoids the tax at the price of ambiguity when a name recurs.

A dated residual defect this mechanism left behind — the two `ANCHOR_SLACK` constants disagreeing
with each other — is recorded in `kb-reference-phase-4-5-spec-reconciliation-001`, not here: it is
true of one constant on one afternoon, not a property of the method.

## A second instance, and an error class no line check can see (2026-09-03)

The pre-publication review found this defect in the document it had modelled its own format on.
`references/evaluation/phase-7-contract-defects.md`'s *Lifecycle* bullet cites `README.md:83-85`
for the lifecycle's one permitted in-place change; in `references/evaluation/README.md` that range
is the **erratum** exception, and the repointing rule is a different paragraph well below it. The
citation resolves, so nothing fires — the addressing-versus-referent gap
`kb-playbook-verify-referent-report-coverage-001` names, which the anchor check above closes only
where an anchor exists.

What is new is the **propagation mechanism**. That bullet is a header template, and later
evaluation documents copy it rather than re-derive it. The 2026-09-03 review copied it and
repaired the number to `:228-231` — correct at its pinned commit `56ef6c5`, wrong by the time it
landed, because the commit that recorded the review inserted exactly 40 lines above that paragraph
and carried it to `:268-270`. **An error that spreads by imitation cannot be caught by checking
lines**, because each copy is independently well-formed: the first resolves to the wrong passage,
the second resolved to the right one on the day it was written. The defect lives in the relation
between copies, which no per-citation check has in view. Neither instance is repaired here;
repointing belongs in the evaluation tree, not in an atom about it.

That instance also states the coverage this playbook has been pricing. Explicit anchors are
enforced over `standards/rust/` alone (`lint_constitution`'s `ATOM_DIR`); derived anchors reach
`spec/SPECIFICATION.md` and `spec/E2E-CASES.md` alone (`spec_trace`'s `SPEC` and `CASES`).
`references/evaluation/` — twenty-nine documents, dense in `file:line`, and the tree an auditor
reads first — has neither. Both instances were found there, which is what an unchecked citation
habit looks like from the outside.
