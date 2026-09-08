---
id: kb-decision-0045
title: A citation anchor matches its line exactly, or the lint refuses to guess
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0045
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  lint_constitution.rs's ANCHOR_SLACK is removed entirely: a citation must exact-match its
  anchor on the cited line; on a miss the checker searches the file and names the line the
  citation should carry; where the anchor is not unique it lists every candidate and refuses
  to choose; where it is nowhere it asks for a human. All 88 of 323 previously slack-only
  citations were repointed by anchor, never by offset, and 16 anchors were sharpened.
  spec_trace.rs's ANCHOR_SLACK = 12 is deliberately kept, because its anchor is a derived
  identifier pointing at evidence prose rather than a quoted string.
depends_on: []
related:
  - kb-playbook-anchoring-citations-001
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-open-question-exact-anchor-residue-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/citation-anchor-slack.md
  - .kb/_intake/2026-09-07-ratifications-discharged-and-what-execution-changed.md
  - .kb/_intake/remediation-2026-09-04-briefs/sole-evidence-pins-and-moved-file-citations.md
last_reviewed: 2026-09-07
---

# A citation anchor matches its line exactly, or the lint refuses to guess

## Decision

`xtask/src/lint_constitution.rs`'s `ANCHOR_SLACK` — the ten-line window inside which a
citation's stated line could drift from its anchor and still read as green — is removed
entirely, over `standards/rust/`. A citation must now exact-match: the line it names must
contain the quoted anchor text, or the check fails. On a miss the checker searches the whole
target file and **names the line the citation should carry**; where the anchor occurs more than
once it lists every candidate and **refuses to choose**, on the ground that sharpening the
anchor is the repair, not a guess at which occurrence was meant; where the anchor is nowhere in
the file it says so and asks for a human, because no mechanical repair can find a line that does
not exist. All 323 anchored citations in the corpus were re-measured at the landing commit: 235
were already exact, 88 were green only on the slack, 0 were missing. All 88 were repointed by
searching for the anchor and never by applying a numeric offset; 70 had an anchor unique in
their target file and were mechanical, 18 needed a human to choose among candidates, and those
choices are recorded in the landing commit message. Sixteen anchors were sharpened in the same
pass — `impl Defect for` occurred 70 times in `mutants.rs` and is now `impl Defect for
InnerJoinTagStore` — which is the durable half of the repair: a citation whose anchor is unique
in its file cannot silently re-attach to the wrong occurrence the next time a line moves.

## Why the tolerance had to go rather than shrink

The ten-line window's own rationale predicted a distribution concentrated near zero, decaying
with distance — ordinary editing drifts a little, a function moving drifts a lot and should
turn the gate red. Measured against the corpus, the distribution did not decay: it piled up in
lobes at +4, +7 and +9, with citations sitting at exactly +10, one inserted line from red. Three
citations inside the tolerance were already pointing at the wrong thing entirely — a closing
brace, an `allow = [` line, the line above a field — and each read as passing. A scratch
demonstration moved a real anchor by ten lines to a deliberately false line and the step still
printed "27 atoms, all consistent." The tolerance's own strongest defense — that in this corpus
nothing drifted far enough to matter — was itself falsified by the same measurement that
motivated this decision: the corpus's worst blind spot was not inside the window at all, it was
a class of citation (into files without a slash in their path) the checker had never examined,
found and fixed in the same lane. A green that is read as coverage and is not coverage is worse
than no check, because it retires the reader's own vigilance — exactly the failure this repo's
own README claimed the step already prevented and, at a tolerance of ten, could not.

## What stays out of scope, and why

`xtask/src/spec_trace.rs` keeps its own `ANCHOR_SLACK = 12` over `spec/SPECIFICATION.md`,
unmoved by this decision. The two checkers do not share an anchor concept: a constitution atom's
citation carries its anchor as quoted text beside the line number, so exact-match is satisfiable
by construction — the claim being made is literally "the cited line contains this text." A
specification clause's citation carries no such text; it derives its anchor from an identifier
the specification's prose happened to reach for, and the citation's range points at the
*evidence* for a clause, usually a doc-comment bullet, with the signature it names sitting just
outside the cited range. Closing that window to zero would repoint 21 of 80 anchored citations
onto a signature that states nothing, moving the citation off the prose it is evidence for to
make two numbers match — a worse citation bought for consistency. The two constants no longer
differ silently: each now carries a doc comment stating the argument and citing the other.

## What this does not settle

Whether `spec_trace`'s window is right at twelve is unchanged — it is now *stated* rather than
silent, and nothing has ever derived the number from measurement. Whether an anchor must be
unique in its target file is not a rule; the checker nudges toward sharpening one when it meets
an ambiguous case, and nothing requires the next contributor to comply. A shipped `--repoint`
rewriter is not built — this decision ships the message naming the correct line, not an
automated rewrite — so the idempotence hazard a sibling lane found (a content-based repair
script re-shifting eleven already-corrected citations on a second run) is sidestepped rather
than answered, and would still need its own answer if a rewriter is ever built.
