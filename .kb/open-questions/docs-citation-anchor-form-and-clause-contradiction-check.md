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
  reaches at all. Sub-question 2 has an answer as of 2026-09-08, and the exclusion's cost has been
  paid once: the 2026-09-07 ingest wave promoted kb-decision-0058 with three ranges inherited from
  a 2026-09-04 brief, one landing on a blank line, and cargo xtask ci was red on main from 6acdf24
  until the 0.2.0 closeout session — the ingest, not only ratification, is a moment where unchecked
  citations become checked ones. The owner chose re-anchor at promotion: /redkiln:kb-ingest
  resolves every path:line against HEAD as it authors and refuses the wave rather than the atom
  when one does not resolve; scanning _intake as a warning was declined because it puts the check
  on the wrong side of the boundary. The fix lives in the redkiln plugin (fix-kb-ingest-defects),
  not in .redkiln/, so until it lands the interim is a habit: run the gate immediately after an
  ingest wave merges. Two of the files that carried this finding into the 2026-09-11 wave had
  themselves drifted while staged (xtask/src/lints.rs:2003 and :2007 for a constant now at :2488;
  CF-34 at :8747 for a clause now at :9021); neither would redden the gate, which is the milder
  half of what this atom already records.
depends_on: []
related:
  - kb-open-question-rustdoc-citation-form-001
  - kb-playbook-anchoring-citations-001
  - kb-reference-intake-citation-drift-census-001
  - kb-decision-0058
  - kb-open-question-immutability-check-pre-commit-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/docs-citation-form-and-clause-content.md
  - .kb/_intake/2026-09-08-intake-is-outside-the-citation-scan.md
  - .kb/_intake/2026-09-09-the-measurement-host-and-its-clock.md
  - xtask/src/lints.rs
last_reviewed: 2026-09-11
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
staging that `/redkiln:kb-ingest` clears
(`CITATION_SCAN_EXCLUDE`, `xtask/src/lints.rs:2488`).

### The exclusion's cost has now been paid once

The postscript's prediction arrived, and at a moment the postscript had
argued was the *less* important one. The 2026-09-07 ingest wave (`025f300`,
merged at `6acdf24`) promoted `kb-decision-0058` carrying three line ranges
inherited whole from a brief authored 2026-09-04, against a tree the
merge-join read path had since moved. One of them opened on a blank line.
`a4616ca` — the repointing commit for what that merge moved — had touched
nothing under `.kb/`, and correctly so: nothing there was in scope. So
`cargo xtask ci` went red on `main` at `6acdf24` and stayed red until the
`0.2.0` closeout session's first baseline run found it, because nobody ran
the gate between an ingest merge and the next session. The citation was
stale on arrival, not drifted afterwards; the ingest is the exact step at
which a document leaves a directory nothing checks for one that is checked,
and it is the step at which nobody is looking at line numbers, because the
work in front of them is adjudication. Ratification is the earlier unchecked
moment; ingest is the later one, and it is the one that reddens the gate.

The repair of the atom itself (`4e13ee2`) ran into a second instrument
limit — the immutability check refuses an uncommitted referent repair and
passes a committed reversal — which is its own question and lives at
`accepted-atom-immutability-check-is-pre-commit-only.md`, not here.

The pattern repeated inside the very wave that carried this finding. Two of
the 2026-09-11 intake files cited `CITATION_SCAN_EXCLUDE` at
`xtask/src/lints.rs:2003` and `:2007`, and the constant had moved to `:2488`
under them while they waited; the measurement-host brief cited CF-34 at
`spec/SPECIFICATION.md:8747` for a clause now at `:9021`. Neither would
have reddened the gate — each line still exists and is not obviously the
wrong kind of place — which is the milder, wrong-*place* half of the first
defect above rather than the blank-line half. The 2026-09 census
(`kb-reference-intake-citation-drift-census-001`) is the dated measurement
of how common that milder case is across staged briefs; it stays a snapshot
at its stated commit pair and is not extended by these two.

## What is not decided

Whether `docs/` and `examples/` citations should adopt the anchored form (the
brief's Option A, recommended, sequenced after the exact-anchor tolerance
question is settled — see `what-the-exact-anchor-rule-still-leaves-open.md`);
and — the one the brief says plainly cannot be made mechanical — whether
anything can ever catch a citation that resolves cleanly, points at real
content, and states the reverse of what that content says (Option B, which
the brief converts into a human reviewer's procedure rather than a checker,
on the ground that `lint_pages.rs` is explicit its walk is the instrument for
exactly this and "never a byte count").

The `.kb/_intake/` scan question (Option C) is no longer open in the form
the brief posed it. At the 2026-09-08 review the owner chose the second of
two repairs: **re-anchor at promotion**. `/redkiln:kb-ingest` resolves every
`path:line` against `HEAD` as it authors an atom, and refuses the *wave*
rather than the atom when one does not resolve, which closes the hole at the
only point where the unchecked and the checked directory meet and fails the
wave rather than the next person to run the gate. The first repair — bring
`_intake` into scope as a warning, never a failure — was declined, not
because it would fail to work but because it puts the check on the wrong
side of the boundary: a warning in this repository about documents that tool
is about to promote makes happenstance responsible for noticing redkiln's
defect on every run, forever. Two things follow. The fix is not a change to
this repository: the ingest workflow lives in the redkiln plugin, not in
`.redkiln/` (which holds config, the pinned process pack, templates and
telemetry, and no workflow), so it sits on redkiln's own backlog under its
`fix-kb-ingest-defects` project. And until it lands the hole stays open and
the next wave can redden the gate the same way, so the interim is a habit
rather than a code change: run `cargo xtask ci` immediately after an ingest
wave merges, rather than trusting the pre-merge green. What remains open on
this sub-question is only whether the promotion-time check should also
*re-anchor* a citation it can recover mechanically, or only refuse.

## What forces it

The anchored-form question is sequenced explicitly behind
`what-the-exact-anchor-rule-still-leaves-open.md`'s tolerance settlement, so it
activates once that lands or is revisited. The `.kb/_intake/` scan question
was forced by the postscript's measurement and then by the red gate; it is
now waiting on a fix in another repository, and the forcing event for the
interim habit is every ingest wave until then. The contradiction-check
question has no forcing event named; it stays a standing gap unless a second
contradiction is found in production documentation, which is the condition
that would make "the honest thing is to say so" look inadequate in hindsight
the way it already looks in hindsight for this one.

## Ordered sub-questions

1. Does the anchored `path:line (anchor)` form extend to `docs/` and
   `examples/` once `spec_trace`/`lint_constitution`'s tolerance question is
   settled, adopting whatever anchor-strictness rule that settlement lands on
   rather than picking a second number?
2. Answered 2026-09-08 in substance — re-anchor at promotion, in
   `/redkiln:kb-ingest`, refusing the wave — and not yet landed. What stays
   open is narrower: whether promotion-time resolution refuses only, or also
   repoints the mechanically recoverable cases the census counted, and
   whether the post-merge gate run stays a habit or becomes a step the
   ingest command itself performs.
3. Is there any mechanical proxy for "this citation's surrounding sentence
   contradicts the clause it cites" — for example flagging citations adjacent
   to negation words for closer human review — or does this stay permanently a
   reviewer's-walk-only defect, as `lint_pages.rs`'s own limits section
   currently implies?
