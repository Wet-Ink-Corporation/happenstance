---
id: kb-open-question-no-ps-rule-name-resolved-001
title: No PS rule name is resolved, because the dagger that marks it unwritten is also what disables the check
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-38 is FROZEN, and PS-27 and PS-30 are the visible symptoms of a check
  that does not run against the projection family. The intake recorded the
  cause as has_suite excluding PS, and that half is now wrong: has_suite
  admits PS- since the 2026-08-15 wave, held by a test, and
  kb-reference-spec-trace-has-suite-001 records the flip. The guard has two
  terms. Check 4's loop continues on c.schedules_new || !has_suite(&c.id),
  and schedules_new is set by a bare dagger in the clause's Rule line, so
  every PS clause marked with a dagger — the seventeen section 7.2 prints —
  is skipped before its rule name is looked up. The conclusion the intake
  reached therefore stands and its stated mechanism does not, and the
  correction is what joins the two halves of the finding: the dagger is
  simultaneously the marker that duplicates the clause's own maturity
  marker while naming no owner where the clause does, and the switch that
  disables the check on the clauses carrying it. What is not decided is
  whether a dagger should suppress rule resolution at all, whether the
  dagger convention is superseded by maturity markers that carry owners,
  and whether removing it from the guard would report a wall of true
  positives that some other decision must absorb first. Forced by the next
  PS clause that cites a rule name nobody has written, which no gate in
  this repository would report.
depends_on: []
related:
  - kb-reference-spec-trace-has-suite-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-contract-defects.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
---

# No PS rule name is resolved, because the dagger that marks it unwritten is also what disables the check

## What is true today

CF-38 is `[FROZEN]`, and PS-27 and PS-30 are the visible symptoms named
against it: two clauses whose rule names are never resolved by `cargo xtask
spec-trace`. The phase-7 defect log recorded the cause as `has_suite`
excluding the `PS` family outright, at `spec_trace.rs:692-694`. Verified
against this worktree, that half of the finding is now wrong: `has_suite`
(`xtask/src/spec_trace.rs:2501`) admits the `PS-` prefix as of the
2026-08-15 wave, a change held by its own test
(`xtask/src/spec_trace.rs:3477-3484`, asserting `has_suite("PS-1")` and
`has_suite("PS-37")`), and recorded at `kb-reference-spec-trace-has-suite-001`.

The guard that actually gates check 4's rule-name resolution has two terms,
not one: the loop continues past a clause — skipping it, resolving nothing
— on `c.schedules_new || !has_suite(&c.id)` (`xtask/src/spec_trace.rs:699`).
`has_suite` is now true for `PS`. `schedules_new` is not: it is set by
`Rules::parse` (`xtask/src/spec_trace.rs:1630-1634`) whenever the clause's
`Rule:` line contains a bare `†`, among a few other markers. Every `PS`
clause in section 7.2 that carries that dagger — seventeen of them, by the
specification's own print — is therefore skipped before its rule name is
ever looked up, regardless of what `has_suite` now says. The intake's
**conclusion** — no `PS` rule name is resolved — stands. Its **stated
mechanism** was `has_suite`'s exclusion, and that mechanism no longer holds;
the actual mechanism is the dagger.

This correction does more than swap one cause for another: it merges what
looked like two separable findings — "the family switch excludes PS" and
"the dagger marker is redundant with the clause's own maturity annotation"
— into one, because the dagger is simultaneously both things at once. It is
the marker that duplicates the clause's `[FROZEN]`/`[PROVISIONAL]` status
while naming no owner where the maturity marker does, **and** it is the
literal switch in `schedules_new` that disables rule-name resolution on
every clause carrying it. `kb-reference-spec-trace-has-suite-001`'s own text
already anticipated this shape, describing the seventeen dagger marks as
"an accurate statement rather than a checked one" — accurate about the rule
being unwritten, unchecked as to whether the check itself ever runs.

## What is not decided

Whether a dagger should suppress rule-name resolution at all — the design
intent may have been "don't fail on a rule that doesn't exist yet," which
is defensible, but it currently also means "don't ever check again," which
forecloses the clause from ever moving out of the daggered state via this
gate. Whether the dagger convention should be retired in favour of the
maturity markers the clauses already carry, which do name an owner. And
whether removing `schedules_new` from the guard would immediately report a
wall of true positives — seventeen or more newly-failing checks — that some
other decision needs to absorb first, rather than landing as a surprise CI
break.

## What forces it

The next `PS` clause that cites a rule name nobody has written. No gate in
this repository currently reports that state, because the dagger that marks
it unwritten is the same signal that stops the check from looking.

## Ordered sub-questions

1. Does `schedules_new` need to exist as a guard term at all, given
   `has_suite` already answers "does a conformance suite exist for this
   family" — is the dagger doing work `has_suite` cannot?
2. If the dagger is retired from the guard, how many of the seventeen
   daggered `PS` clauses newly fail, and does that number change the
   answer to sub-question 1?
3. Is the dagger convention itself superseded by the maturity markers, in
   which case removing it from `spec_trace.rs` and from the seventeen
   clauses' `Rule:` lines are two edits to the same decision rather than
   two separate ones?
