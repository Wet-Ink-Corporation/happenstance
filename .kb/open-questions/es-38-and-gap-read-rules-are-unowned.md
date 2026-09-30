---
id: kb-open-question-es-38-and-gap-read-unowned-001
title: Two frozen clauses name rules that are unwritten and unscheduled
kind: open_question
status: accepted
authority_tier: note
summary: >-
  A FROZEN marker binds the design; it does not assert that anything checks it. Two clauses demonstrate the difference. ES-38's rule, positions_are_not_reused_after_removal, cannot be written today: it needs a store capable of removing events, the fixture declares no such capability, and the completeness instrument it depends on is deferred with nothing planned before phase 14. And read_from_a_gap_position, which ES-9 is owed, is now named by two accepted decisions — ADR-0013 and ADR-0011 — and scheduled by neither; ADR-0011 claims ES-9 itself but assigns no owner to this rule, and ADR-0013 says explicitly that '0013 covered it' must not become the reason it goes unclaimed. What is not decided is who writes each, and whether a frozen clause may name a rule with no owning phase at all. Owner for ES-38's rule: phase 14. Owner for read_from_a_gap_position: unassigned, which is the point. Related in shape but not in subject to the ES-6 question, which is about a rule that cannot be written rather than one nobody has been asked to write.
  Amended 2026-09-28: the gap-read half is answered — read_from_a_gap_position landed at phase 4
  (d480446), ES-9 defines its behaviour, and FromIsAnOffsetStore and BackwardsIgnoredStore fail
  it. The ES-38 half stays open, owned by phase 14, and its sub-question 3 is unanswered.
  Amended 2026-09-29 by kb-decision-0028: sub-question 3 is answered. The instrument decision is
  taken at phase 17, and phase 14 builds rather than decides. It writes
  positions_are_not_reused_after_removal against CF-27's decorator and against every real adapter,
  through a new Fixture capability, defaulted to declined, that removes events outside the port.
  Its named wrong implementation is a SQLite table without AUTOINCREMENT. The rule stays unwritten
  and phase 14's, which is now a scheduled obligation, not an unowned one.
  Sub-question 2, whether a frozen clause may name a rule with no owning phase, stays open, so
  this question does too.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0011
  - kb-open-question-es-6-unwritable-rule-001
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-open-question-es-18-byte-identical-conformance-001
  - kb-decision-0028
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - .kb/_intake/0011-read-laziness-and-isolation.md
  - references/adr/0013-position-assignment-and-visibility.md
  - spec/SPECIFICATION.md
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-09-29
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

## Amended 2026-09-28 — the gap-read half is answered; the question stays open

`read_from_a_gap_position` exists. It landed at phase 4 in `d480446`, is registered in the
testkit's suite, is failed by `FromIsAnOffsetStore` and `BackwardsIgnoredStore` in
`tests/mutation_coverage.rs`, and ES-9 now states the behaviour it checks;
`spec/SPECIFICATION.md` strikes it from the list of unwritten rules. Sub-question 1 is answered.

The ES-38 half is not. ES-38 records `positions_are_not_reused_after_removal` as examined at phase
4 and deliberately not written, blocked on CF-27's completeness instrument, and names phase 14 as
its owner; `runbook/phases/14-retention.md` carries it as an open work item. Sub-question 3 —
instrument first or rule first — is still unanswered, and is phase 14's. Sub-question 2 is answered
in part by mechanism: `spec_trace`'s `UNRESOLVABLE_RULE_NAMES` makes every unwritten rule a clause
names carry a declared reason, though the reason is a sentence rather than a phase.

## Amended 2026-09-29 — sub-question 3 answered by `kb-decision-0028`

ADR-0028 takes the written refusal for retention and, with it, the instrument decision this
question left to phase 14. **Sub-question 3 is answered: phase 14 does not need its own instrument
decision first.** `positions_are_not_reused_after_removal` is written at phase 14 against two
things:

- **CF-27's completeness instrument.** This is a testkit decorator over any `EventStore`, holding an
  arbitrary retained set and reporting what it withholds through its own inherent API.
- **Every real adapter**, through a new `Fixture` capability. The capability is defaulted to
  declined, on `MID_BATCH_FAULT`'s precedent, and through it a fixture removes events outside the
  port with a raw `DELETE`. The capability's name is phase 14's.

The rule's named wrong implementation is a SQLite table declared `INTEGER PRIMARY KEY` without
`AUTOINCREMENT`, which hands the deleted tail's highest rowid out again. Every published adapter
allocates by `AUTOINCREMENT` or a sequence, so each should pass.

Sub-question 2 stays answered in part by mechanism, as the 2026-09-28 amendment says. The broader
policy it asks for is not taken here. ES-38's rule is now owned and scheduled rather than unowned,
which is the gap this question recorded. `runbook/phases/14-retention.md` carries it as a work item.

**This question stays open** for sub-question 2's policy: whether a frozen clause may name a rule
with no owning phase at all. Amended by hand in phase 17, not closed. No accepted decision was
edited.
