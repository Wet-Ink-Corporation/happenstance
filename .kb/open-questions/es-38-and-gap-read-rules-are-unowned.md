---
id: kb-open-question-es-38-and-gap-read-unowned-001
title: Two frozen clauses name rules that are unwritten and unscheduled
kind: open_question
status: accepted
authority_tier: note
summary: >-
  A FROZEN marker binds the design; it does not assert that anything checks it. Two clauses demonstrate the difference. ES-38's rule, positions_are_not_reused_after_removal, cannot be written today: it needs a store capable of removing events, the fixture declares no such capability, and the completeness instrument it depends on is deferred with nothing planned before phase 14. And read_from_a_gap_position, which ES-9 is owed, is now named by two accepted decisions — ADR-0013 and ADR-0011 — and scheduled by neither; ADR-0011 claims ES-9 itself but assigns no owner to this rule, and ADR-0013 says explicitly that '0013 covered it' must not become the reason it goes unclaimed. What is not decided is who writes each, and whether a frozen clause may name a rule with no owning phase at all. Owner for ES-38's rule: phase 14. Owner for read_from_a_gap_position: unassigned, which is the point. Related in shape but not in subject to the ES-6 question, which is about a rule that cannot be written rather than one nobody has been asked to write.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0011
  - kb-open-question-es-6-unwritable-rule-001
  - kb-reference-phase-4-5-spec-reconciliation-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-08-10
---

# Two frozen clauses name rules that are unwritten and unscheduled

## What is true today

`[FROZEN]` records a design decision. It does not record that anything checks
it, and two clauses in the position/read neighbourhood show the gap between
the two in different shapes.

**ES-38** (`[FROZEN]`, settled by ADR-0013) says a store from which events
have been removed by any means outside the port keeps every promise about the
events it still holds: no position is reused, survivors stay unique and
monotonic, `Query::all()` still means everything the store holds. Its rule,
`positions_are_not_reused_after_removal`, cannot be written today. It needs a
store capable of removing events; `Fixture` declares no such capability. The
completeness instrument ES-38 depends on is `CF-27`, which is `[DEFERRED]`
with nothing planned before phase 14. ADR-0013 records this as a limitation
of its own lift rather than a gap it introduces: "This ADR transcribes ES-38
into the port's documentation and records that its rule waits. **Owner: phase
14.**"

**`read_from_a_gap_position`** is different: it is not blocked on an
instrument, it is blocked on nobody having been asked to write it. It is
ES-9's owed rule — ES-9 (`[FROZEN]`) is what makes `ReadOptions::from` a
threshold rather than a seek, so a resume at an unoccupied position yields the
next matching event rather than an error, and `checkpoint.next()` is sound
over gaps only because of it. VT-13 names the rule only parenthetically, as
"(ES-9's name for it)". ADR-0013 says outright: "ADR-0011 takes the
twenty-nine-ID extension and claims ES-9 — but it schedules no owner for this
rule, so as of this ADR the rule is named by two documents and owned by
neither. It is listed here so that fact is written down before phase 4
closes, not to claim it." ADR-0011, for its part, lists ES-8, ES-9 and ES-15
among the clauses it "confirms unchanged, and owes no amendment" — it looked
at ES-9, changed nothing in its text, and did not assign the rule an owner
either. Two accepted decisions have now looked directly at this rule and
neither took it.

## What is not decided

Who writes each rule, and — the broader question the pair raises together —
whether a `[FROZEN]` clause may name a rule with no owning phase at all and
stay frozen indefinitely. ES-38's case has an owner and a reason it cannot be
written sooner; `read_from_a_gap_position`'s case has neither, which is a
different and arguably worse kind of gap: not "not yet buildable" but "nobody
has claimed it."

## What forces it

For ES-38: `CF-27` landing at or before phase 14, and an adapter capable of
removing events to test against. For `read_from_a_gap_position`: nothing
forces it today, and that absence of a forcing function is itself the
finding — a rule two ADRs have each looked at and declined to own can sit
unclaimed indefinitely with no gate step noticing, the same way ES-6's rule
sat unclaimed under `spec_trace`'s `(new)`/`†` escape hatch.

## Ordered sub-questions

1. Does `read_from_a_gap_position` get an explicit owner in a future ADR (a
   phase 4/5 reconciliation pass, or ADR-0013/ADR-0011's own successors), or
   does it wait for whichever ADR next touches ES-9's neighbourhood to notice
   it a third time?
2. Should a general policy require every `[FROZEN]` clause's named rule to
   carry an owning phase, closing the same class of gap ES-6's question
   raises for `(new)`/`†` markers?
3. When `CF-27` lands, does `positions_are_not_reused_after_removal` get
   written against whatever removal-capable fixture arrives with it, or does
   phase 14 need its own instrument decision first?
