---
id: kb-governance-what-may-refute-a-finding-001
title: What may refute a finding, and how an accepted decision's currency is computed
kind: governance
status: accepted
authority_tier: guideline
summary: >-
  The precedence ladder this repository already declares - SPECIFICATION clause above accepted
  ADR above constitution atom above CLAUDE.md above references/evaluation/* - applied to a verb
  it was not written for. standards/rust/README.md:23-43 tells an author which document wins when
  two disagree; this atom tells an auditor what may overturn a finding, and three consequences
  follow that the ladder alone does not give. First, references/evaluation/* and RUNBOOK.md may
  annotate a finding and may never refute one: they are dated evidence rather than rules, and a
  review that lets them refute suppresses true findings using documents the repository itself
  declares non-binding - which is RS-01-3's rule read from the auditing end. Second, a refutation
  needs a quoted sentence answering the same question. A filename is not a refutation and a
  section number is not a refutation, and the absence of a citation in a corpus of roughly forty
  thousand lines is a search failure rather than a verdict. Third, an accepted decision's currency
  is computable rather than a matter of judgement: git log <adr-commit>..HEAD -- <the files the
  decision cites> returns empty when the decision was taken against exactly this code, and it
  refutes; when it returns commits, the decision refutes only if a line at HEAD still implements
  what it decided, and where that line has moved or vanished the correct verdict is that an
  accepted decision may have silently drifted - the highest-value class available and the one a
  naive "the ADR covers this" reading discards. Specification clauses are exempt from the
  computation because cargo xtask spec-trace is a gate step, so their citations resolve at HEAD by
  construction.
depends_on: []
related:
  - kb-governance-referent-not-reasoning-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-verify-referent-report-coverage-001
  - kb-decision-0022
  - kb-open-question-adr-0022-falsifiers-fired-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - standards/rust/README.md
  - standards/rust/01-standard-of-evidence.md
last_reviewed: 2026-09-04
---

# What may refute a finding, and how an accepted decision's currency is computed

## What this adds to the existing ladder

`standards/rust/README.md:23-43` already states this repository's precedence ladder: a
SPECIFICATION clause outranks an accepted ADR, which outranks a constitution atom, which outranks
`CLAUDE.md`, which outranks `references/evaluation/*`. That ladder answers one question — which
document wins when two disagree about what is true. It was written for an *author* deciding what
to write next. This atom answers a different question the same ladder implies but never states:
what is allowed to **overturn a finding** someone has already made about the tree. The distinction
matters because a review that gets the second question wrong can suppress a true finding while
citing the very ladder that would, read correctly, have supported it.

## Three consequences, derived rather than assumed

**Dated evidence may annotate, and may never refute.** `references/evaluation/*` and
`RUNBOOK.md` sit at the bottom of the ladder by design — they are recorded observations about a
past state of the tree, not rules the tree is held to. An auditor who lets a citation from either
override a fresh finding has inverted the ladder: they are using the repository's own
lowest-authority documents to suppress something that may be true right now. Such documents may
still **annotate** a finding — noting that a prior review already looked at the same area, say —
but annotation is not refutation, and the difference is whether the citation changes what is true
or only adds context to it.

**A refutation needs a quoted sentence answering the same question.** The failure mode this
guards against is citing an artefact rather than its content: a filename, a section number, or "see
ADR-0022" with nothing quoted. None of those is a refutation, because none of them states what the
cited material actually says about the question at hand. In a corpus of roughly forty thousand
lines, failing to find a sentence that answers the question is evidence the search was incomplete,
not evidence the finding is wrong — the two are easy to conflate under time pressure, and only the
first is a defensible verdict.

**An accepted decision's currency is a computation, not a judgement call.** For a finding that
appears to contradict an accepted decision, run `git log <adr-commit>..HEAD -- <the files the
decision's rationale cites>`. An empty result means the decision was taken against exactly the code
that exists now, and it stands as a full refutation. A non-empty result does not settle anything by
itself — it means the cited files moved since the decision was written, and the decision refutes
the finding only if a line at `HEAD` still implements what the decision actually decided. Where
that line has moved or been removed, the correct verdict is **an accepted decision may have
silently drifted from the code it once described** — which is the class of finding the naive
reading ("the ADR already covers this, dismiss it") reliably discards, because that reading stops
at the citation resolving rather than checking what it resolves *to*.

## The one exemption, and why it needs none of this

Specification clauses do not need the git-log computation applied to them. `cargo xtask spec-trace`
is a standing gate step that re-verifies every clause's citations against `HEAD` on every run, so a
clause's references are current by construction rather than by an auditor's separate check — the
guarantee this atom asks an auditor to build by hand for a decision is already built into the gate
for a clause.
