---
id: kb-playbook-declared-page-need-001
title: Making the need a page answers a declared, singular, checkable property
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  A prose tree teaches unevenly and nothing sees it: the need each page answers is
  implicit and therefore arguable. The method makes it explicit — one need per page,
  declared in the page's own visible body text, spelled from a closed enumerated set,
  checked mechanically — then the check documents the judgement it cannot make, and a
  written non-author walk supplies it. Front matter, an HTML comment, a filename
  convention, a sidecar manifest and a badge all lost as declaration forms; an open set
  lost because no check tests membership in one; a persona taxonomy lost because a persona
  is a property of the reader. The discriminator is that a check sees a declaration and
  never an answer, so any form that hides it from the reader or makes it unenumerable
  hands the discipline back to judgement.
depends_on:
  - kb-playbook-verify-referent-report-coverage-001
related:
  - kb-playbook-ratchet-gate-landing-001
  - kb-playbook-anchoring-citations-001
  - kb-governance-referent-not-reasoning-001
source_paths:
  - .kb/_intake/lesson-page-need-declaration-discipline.md
  - standards/pages/README.md
  - standards/pages/00-one-need.md
  - standards/pages/10-the-need-set.md
  - standards/pages/20-the-fold-line.md
  - standards/pages/40-reviewing-a-page.md
  - xtask/src/lint_pages.rs
  - .bklg/docs-that-teach/page-need-discipline/_design.md
  - .bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md
last_reviewed: 2026-09-02
---

# Making the need a page answers a declared, singular, checkable property

## The situation

Nothing in a prose tree records which reader's question a page answers, so a page
that quietly answers two reads as complete and is not: whoever came for the second
question leaves with a partial answer and no signal of it. Reviewers see that; no
instrument does. Measured, not assumed: an evaluator found the material good until
the second question and had nowhere to go. The shape recurs wherever an artefact's
purpose is a matter of opinion.

## The method, in four steps

**1. Declare the need in the artefact's own visible body text.** One line under the
title, carrying a token and the reader's question in the author's words. Prose the
reader sees, not metadata. The count and the position are both load-bearing, and
both are cited rather than repeated here. Evidence: RP-00-1 (the count) and RP-00-2
(the position, and the reader's test it protects) in
`standards/pages/00-one-need.md`; the hosting assumption is written beside the
parser — the medium renders CommonMark blockquotes in document order, and nothing
else (`xtask/src/lint_pages.rs`).

**2. Spell the token from a closed, enumerated set held in one place.** The
enumeration is a constant the check reads, its argument is one atom a reader can
disagree with, and a compile-time assertion caps the set, so a straining page is
split, not granted a new token. Evidence: `xtask/src/lint_pages.rs` carries `NEEDS`,
`MAX_NEEDS` and a `const _: () = assert!(...)` failing at `cargo check`, not in a
test the token's author could delete; in `standards/pages/10-the-need-set.md`,
RP-10-1 governs how the token is spelled and RP-10-4 how the set is changed.

**3. Check the declaration mechanically, and document what the check cannot see.**
Report zero declarations, two, a malformed line and an out-of-set token — all of
them, by file and line, in one run — and open the check's documentation with its
limits. Evidence: `xtask/src/lint_pages.rs` opens `# What this does not verify`, its
first limit being that a need is *declared* and never *answered*. Watched failing:
`docs/append-conditions.md:4 — declares 'explanation' and 'how-to'; a page answers
one need`. The general rule is RS-81-1, cited in full below.

**4. Supply a written non-author procedure for the judgement the check cannot
make.** An ordered walk of yes/no questions closing in an enumerated verdict with no
soft pass, calibrated against a specimen that makes it return a failure. Evidence:
in `standards/pages/40-reviewing-a-page.md`, RP-40-1 carries the walk, its four
verdicts and the constraints on who may run it and what they may consult; RP-40-2
the paraphrase spot check. The specimen is inert: it stays broken without a gate
going red.

## The alternatives that lost

**The declaration's form — five rejected** (`_design.md:128-147`). Front matter
renders as literal text or a rule in the renderers in play, and reads as machine
chrome. An HTML comment is machine-readable and invisible — the sidecar failure in
an inline costume. A filename convention is invisible on the rendered page and
forces the taxonomy into the tree. A sidecar manifest is two artefacts that have to
agree, one of which goes stale. A badge or coloured admonition carries meaning by
colour alone.

**The need set — three rejected** (`_design.md:188-202`). A four-box taxonomy
adopted literally lost because the cited critique names a dense interrelated model
as where the split strains, and it would have left an empty bucket. Dropping the
enumeration lost because no check tests membership in an open set, which reduces the
discipline to a convention. A persona-keyed taxonomy lost because one reader
consumes explanations and how-tos alike, so nearly every page would declare two.

**Findability — two rejected** (`_design.md:231-237`). Routing as an implicit
byproduct reproduces the measured defect — an implicit responsibility is nobody's; a
bespoke navigation widget lost because the gap is a missing link, not a missing
widget.

**The fold line — two rejected** (`_design.md:275-280`). Reviewer judgement per page
drifts, which is the failure the tree exists to replace; a permanent ban on
collapsible content is over-broad and would be re-litigated at the first legitimate
case. What shipped is RP-20-1's test, RP-20-2's closed class list, RP-20-3's empty
mechanism table and RP-20-4's asymmetry between the two — folding forbidden in
practice today, with the evidence that would lift it named
(`standards/pages/20-the-fold-line.md`).

**The tree's home — four rejected** (`_design.md:88-96`). Atoms in the
code-standards tree would drag prose rules into a doctest harness; a hand-authored
knowledge-base corpus was tried here once and reverted, atoms being an ingest
process's to author; a subtree of the user documentation is pinned by nothing;
nothing at all leaves the rule to be retrofitted, which is how it becomes
decorative.

## What makes it portable

The transferable half is the pairing, not the vocabulary: a mechanical check that
verifies a declaration, a written non-author procedure for the part it cannot
verify, and the check's own documentation naming the seam between them. It recurs
one tree over on a subject that is not documentation — RS-81-1 states it for any
check (`standards/rust/81-checks-that-cannot-be-types.md:11`) — and
`kb-playbook-verify-referent-report-coverage-001` is the earlier instance: a check
that verified an address and reported its coverage instead of what it could not see.

## Where this is the wrong instrument

A medium that does not render the declaration in document order. A renderer that
strips leading blockquotes, relocates them into a metadata layer, or demands front
matter invalidates the form, and the decision re-opens rather than quietly
contradicting itself.

A corpus small enough that the router costs more than it saves. Below a dozen
artefacts a reader finds the right one by listing the directory.

A subject whose reference surface is not already owned elsewhere. The set here
subtracts a `reference` token because two reference surfaces already existed; where
they do not, the subtraction is wrong and the set is a different set.

A corpus nobody may edit. The remedy for a straining page is splitting it. Where the
artefacts are immutable — accepted decisions, published clauses — the check can only
report: `kb-playbook-ratchet-gate-landing-001`'s problem, not this one.
