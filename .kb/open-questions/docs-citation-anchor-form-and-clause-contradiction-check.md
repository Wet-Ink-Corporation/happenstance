---
id: kb-open-question-docs-citation-anchor-contradiction-001
title: A docs/ page cited a real clause and said the opposite of it, and nothing mechanical would have caught that
kind: open_question
status: accepted
authority_tier: note
summary: >-
  docs/append-conditions.md carried three defects at once and each fell inside a checker's own
  stated blind spot rather than being missed by accident: a citation of ES-40 whose surrounding
  sentence claimed the reverse of what ES-40 says, a dead file:line citation that resolved to the
  wrong paragraph of an unrelated feature's doc comment, and a bare `:122` shorthand citation that
  parse_citation cannot recognise at all because it requires a slash. citation_ranges_resolve checks
  only that a citation's range exists and is not obviously nonsense — never that the cited content
  agrees with the sentence around it — and both lint_pages.rs and lint_narrative.rs say so in their
  own `What this does not verify` sections, just not for the contradiction case specifically. The
  brief that found this proposes and weighs four options (extend the anchored path:line (anchor)
  form from standards/rust/ into docs/ and examples/; check the sentence against the clause instead,
  which only a human reviewer's walk can do; also scan .kb/_intake/, which the checker currently
  excludes as transient staging; or just write the blind spot down in both lint modules) and
  recommends the anchored form plus writing the gap down, explicitly declining to build a mechanical
  contradiction check because none exists. A wave-integrator postscript later measured that the
  intake-scan exclusion's own stated re-examination trigger has already fired: 57 of 699 bare
  path:line citations into live files across sixteen staged briefs had drifted, and the moment that
  matters is not ingest but whenever an owner reads a brief to ratify it — a moment no checker
  reaches at all.
depends_on: []
related:
  - kb-open-question-rustdoc-citation-form-001
  - kb-playbook-anchoring-citations-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/docs-citation-form-and-clause-content.md
last_reviewed: 2026-09-07
---

# A docs/ page cited a real clause and said the opposite of it, and nothing mechanical would have caught that

## What is true today

`docs/append-conditions.md` shipped with three separate citation defects, and
the pattern connecting them is that each sat inside a documented instrument
limit rather than outside every check. The first and worst: the page's whole
argument cited ES-40 as its authority, and ES-40 says the *opposite* of the
sentence's claim about vacuous passes over pruned history.
`xtask/src/lint_narrative.rs`'s `check_citations` resolves the id against
`spec/SPECIFICATION.md` and stops there — its own doc comment already states
the limit for the milder case, restating a clause in the surrounding prose,
but the shipped defect was worse than restatement: it was contradiction, which
the stated limit does not name at all. A reader who follows the page's own
citation discipline into ES-40 either loses trust in the page or walks away
believing a completeness caveat does not exist.

The second defect was a citation into `crates/happenstance-core/src/lib.rs:103`
that lands mid-paragraph inside an unrelated feature's doc comment rather than
at the private `memory` module's declaration, which sits at `:134`.
`citation_ranges_resolve` (`xtask/src/lints.rs`) passed it, because its whole
check is that the range exists and its first line is not the wrong *kind* of
place — never that it is the wrong *place*. Retargeting the same citation at
line 99999 does fail the step by name, which demonstrates the citation was
inside the checker's sight the entire time; only a line number smaller than
the file's length separated it from a report. The third defect was a bare
`` `:122` `` shorthand meaning "same file as the previous citation," which
`parse_citation` does not recognise at all because it requires a slash in the
path — a documented gap for the leading-citation case that was doing more work
than its doc comment claims, since the shorthand was invisible to the one
check that could have flagged it.

The anchored `path:line (anchor)` form that `lint_constitution.rs` already
requires and verifies for `standards/rust/` is scoped to that one tree; `docs/`
and `examples/` carry 77 bare `path:line` citations with no equivalent. The
brief's own repair — while fixing the page — found two more citations gone
stale from a single unrelated section insertion, in a brief sitting inside
`.kb/_intake/`, which the citation scan deliberately excludes as transient
staging that `/redkiln:kb-ingest` clears.

## What is not decided

Whether `docs/` and `examples/` citations should adopt the anchored form (the
brief's Option A, recommended, sequenced after the exact-anchor tolerance
question is settled — see `what-the-exact-anchor-rule-still-leaves-open.md`);
whether `.kb/_intake/` should lose its scan exclusion given that briefs staged
there are cited by each other and go stale while waiting (Option C, declined
on merge-coupling cost between concurrent worktrees, with its own decline
carrying a named re-examination trigger); and — the one the brief says
plainly cannot be made mechanical — whether anything can ever catch a citation
that resolves cleanly, points at real content, and states the reverse of what
that content says (Option B, which the brief converts into a human reviewer's
procedure rather than a checker, on the ground that `lint_pages.rs` is
explicit its walk is the instrument for exactly this and "never a byte
count").

A wave-integrator postscript measured that the Option C decline's own trigger
condition has already fired, earlier than the decline anticipated: across
`.kb/_intake/` at one measured commit pair, 57 of 699 resolvable bare
`path:line` citations into live (non-`references/`) files had drifted since
being written, spread over sixteen briefs, 25 of them mechanically
recoverable and the rest requiring a human. The refuted claim was narrow —
not that the drift is large, but that "the gap closes itself at the moment it
starts to matter" assumes that moment is ingest, when a citation would enter
`CITATION_SCAN_DIRS`. A decision brief's citations are load-bearing at a
strictly earlier and unchecked moment: when an owner reads the brief to
ratify the decision it carries.

## What forces it

The anchored-form question is sequenced explicitly behind
`what-the-exact-anchor-rule-still-leaves-open.md`'s tolerance settlement, so it
activates once that lands or is revisited. The `.kb/_intake/` scan question is
already forced by the postscript's measurement — 24 findings currently sit
unratified behind briefs with unrepointed citations — independent of any
future trigger. The contradiction-check question has no forcing event named;
it stays a standing gap unless a second contradiction is found in production
documentation, which is the condition that would make "the honest thing is to
say so" look inadequate in hindsight the way it already looks in hindsight for
this one.

## Ordered sub-questions

1. Does the anchored `path:line (anchor)` form extend to `docs/` and
   `examples/` once `spec_trace`/`lint_constitution`'s tolerance question is
   settled, adopting whatever anchor-strictness rule that settlement lands on
   rather than picking a second number?
2. Given the postscript's measurement, is the `.kb/_intake/` scan exclusion
   re-examined now rather than waiting for a future ingest, and does the fix
   look like scanning intake after all, a pre-ratification repointing step, or
   something narrower?
3. Is there any mechanical proxy for "this citation's surrounding sentence
   contradicts the clause it cites" — for example flagging citations adjacent
   to negation words for closer human review — or does this stay permanently a
   reviewer's-walk-only defect, as `lint_pages.rs`'s own limits section
   currently implies?
