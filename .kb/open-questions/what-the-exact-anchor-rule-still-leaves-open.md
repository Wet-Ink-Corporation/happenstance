---
id: kb-open-question-exact-anchor-residue-001
title: The exact-anchor citation checker closed the tolerance and left three questions standing
kind: open_question
status: accepted
authority_tier: note
summary: >-
  citation-anchor-slack.md measured that ANCHOR_SLACK = 10 in lint_constitution.rs let 101 of 323
  constitution citations resolve to the wrong line while reporting green, three of them to a
  provably wrong line (a closing brace, an `allow = [`, the line above a field). The repository owner
  ratified Option C: the window is closed to zero, a miss now names the correct line, and an
  ambiguous anchor is refused rather than guessed at. That landing settles the constitution's own
  tolerance and leaves three things it deliberately did not reach. First, spec_trace.rs keeps its
  own ANCHOR_SLACK = 12 for spec/SPECIFICATION.md, on the documented ground that its citations point
  at evidence rather than at anchor text and so cannot satisfy an exact match by construction — a
  reasoned keep, not an oversight, but the number itself has still never been derived from anything.
  Second, no rule requires an anchor to be unique in its target file, even though sixteen anchors
  were sharpened in the same lane specifically because ambiguity was found costly (one anchor
  occurred 70 times). Third, a shipped `--repoint` was never built — Option C shipped only the
  line-naming message — so the idempotence hazard a sibling lane hit while hand-repointing citations
  (a content check that passes on coincidentally-identical lines and re-shifts an already-correct
  citation on a second run) is sidestepped rather than answered, and would still have to be solved by
  whoever builds an automatic repair.
depends_on: []
related:
  - kb-decision-0013
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-es-6-unwritable-rule-001
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/citation-anchor-slack.md
  - .kb/_intake/remediation-2026-09-04-briefs/query-partition-public-surface.md
  - .kb/_intake/remediation-2026-09-04-briefs/sole-evidence-pins-and-moved-file-citations.md
last_reviewed: 2026-09-07
---

# The exact-anchor citation checker closed the tolerance and left three questions standing

## What is true today

`citation-anchor-slack.md` measured `xtask/src/lint_constitution.rs`'s
`ANCHOR_SLACK` — the number of lines a citation's anchor may drift from its
stated line before the checker calls it stale — against the corpus it governs
for the first time. At `ANCHOR_SLACK = 10`, 101 of 323 `standards/rust/`
citations resolved only inside the tolerance, not at the stated line, and the
drift did not cluster near zero: a second lobe sat at +4, a third at +9, and
two citations sat at exactly +10, one inserted line from going red. Three of
the 101 had drifted onto the *wrong* line entirely — a closing brace, a
`deny.toml` `allow = [`, and the line above a field — each reported green for
as long as it was wrong, which is the failure mode the constitution's own
`81-checks-that-cannot-be-types.md` names in the abstract, discovered inside
the checker that atom is cited by. A companion brief (`sole-evidence-pins-and-
moved-file-citations.md`) found the same shape one tree over: `spec-trace`'s
own tolerance let a citation into `spec/SPECIFICATION.md:8656` stay green for
one commit while pointing at the wrong line, because the replacement prose had
been written to a line budget specifically to keep it inside the window.

The repository owner ratified Option C — exact match, with the checker
searching the whole file on a miss and naming the correct line, refusing to
guess where the anchor is ambiguous — and it landed in the constitution tree:
323 citations repointed by anchor (never by offset), sixteen anchors sharpened
to be less ambiguous, and the window removed from `lint_constitution.rs`
entirely. That closes the question `citation-anchor-slack.md` posed for
`standards/rust/`.

## What is not decided

Three residues the ratification explicitly left open, in the ratified record's
own words. **Whether `spec_trace.rs`'s `ANCHOR_SLACK = 12` should move to
match.** It was kept deliberately: a `standards/rust/` citation carries its
anchor as literal quoted text beside the line number, so "the cited line
contains that text" is a claim exact match can satisfy by construction, while
a `spec_trace` citation's range points at the *evidence* for a clause — usually
prose bullets — with the identifier the specification names often sitting just
outside that range. Repointing onto the identifier would move the citation off
the prose it evidences and onto a signature that states nothing. The two
checkers no longer differ *silently* — each now documents and cites the other
— but the 12 itself is still a number nobody has derived from a measurement,
only defended by analogy to a different problem.

**Whether an anchor must be unique in its target file.** Sixteen anchors were
sharpened in the exact-match lane because an ambiguous anchor left the wrong
occurrence pinned by line number alone — the extreme case occurred 70 times in
one file. The checker now lists every candidate and refuses to choose when an
anchor is ambiguous, which is a nudge toward sharpening rather than a rule
requiring it, so the next non-unique anchor added to the corpus is not
rejected, only flagged at the moment someone's citation happens to miss.

**Whether a shipped, idempotent `--repoint` is buildable at all.** Option C
shipped the message a human types the line number from, not an automatic
rewriter. A different lane's hand-built repointing script hit exactly the
hazard a rewriter would have to solve: it derived each new line from `git diff
-U0` hunks and verified the repair by comparing line content before and after,
which is sound once — but running the same script twice re-shifted eleven
already-correct citations, because the already-corrected line happened to have
content identical to a nearby line (a blank line, a bare `///`, a lone `}`),
so the content check passed on coincidence and reapplied a shift that was
already done. The discipline that actually held was "restore from git and run
exactly once," which is not a property a shipped tool can assume of its
caller.

## What forces it

`spec_trace.rs`'s tolerance is re-examined naturally the next time
`spec/SPECIFICATION.md`'s clause citations are audited for drift, the same way
the constitution's was. The uniqueness question is forced the next time an
ambiguous anchor is added and a citation lands on the wrong occurrence of it
before anyone notices. The `--repoint` question is forced the first time
someone tries to automate a repair at a scale where "run it exactly once by
hand" stops being a credible discipline — the sixteen-brief, 57-citation drift
`docs-citation-anchor-form-and-clause-contradiction-check.md` measured across
`.kb/_intake/` is one plausible trigger, since that population is exactly the
kind a tool would be asked to fix in bulk.

## Ordered sub-questions

1. Should `spec_trace.rs`'s `ANCHOR_SLACK = 12` be measured the way
   `lint_constitution.rs`'s was, even though the two checkers' anchors are
   structurally different — and if the measurement supports 12, should that
   derivation be written into the doc comment that currently only argues by
   analogy?
2. Should citation anchors be required to be unique in their target file as a
   standing rule, closing the gap the sixteen sharpened anchors narrowed but
   did not close?
3. Is a `--repoint` that is safe to run twice buildable at all for a citation
   form with no anchor (bare `path:line`), or does automatic repair require
   the anchored form everywhere it is attempted — which would bear directly on
   `docs-citation-anchor-form-and-clause-contradiction-check.md`'s Option A?
