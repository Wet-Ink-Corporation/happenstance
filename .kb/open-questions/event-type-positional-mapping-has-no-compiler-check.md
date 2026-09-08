---
id: kb-open-question-event-type-positional-mapping-001
title: Both worked examples spell event_type with an unchecked positional mapping, and ADR-0033's reopen condition is denominated in a currency too coarse to see it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Both course-subscriptions and transfers-on-sqlite implement DomainEvent::event_type as a match
  pairing each variant with Self::EVENT_TYPES[n] by hand-written index - six subscript expressions
  across the tree, none checked by the compiler. Reorder EVENT_TYPES and every event a store
  accepts is written under the wrong type, silently, on the write path; nothing in the workspace
  rejects it, as clause, conformance rule, or lint. kb-decision-0033 accepted happenstance-macros
  out of scope for 0.1 on a mechanical ceremony-ratio measurement, and its reopen condition is
  denominated entirely in that ratio's own currency - lines, percentages, a threshold - which is
  the right instrument for the question it answers and structurally the wrong one for this defect,
  because the six positional-mapping lines are 0.5 per cent of the two files' combined ceremony and
  the reopen threshold is 1.0. A criterion made of volume cannot see a six-line hazard regardless of
  its consequence. kb-decision-0033 is accepted and immutable; this ground does not amend it, since
  the decision never claimed to cover unchecked mapping obligations and its reopen condition
  excludes this ground by construction rather than by omission. What is not decided is whether the
  positional mapping should be fixed at all, and if so how - a derive is one answer, a const
  assertion pairing each variant with its index is a narrower one that needs no macro crate - and
  separately, whether ADR-0033's reopen condition should eventually be amended to admit
  non-volume evidence, which needs its own decision record and is deliberately not filed here.
  Forced by whoever next reaches ADR-0033 with this evidence and finds a condition it cannot
  satisfy, or by kb-open-question-d-1-no-total-path settling with an infallible Tags path, which is
  ADR-0033's own named reopen trigger and shares no mechanism with this ground.
depends_on: []
related:
  - kb-decision-0033
  - kb-open-question-d-1-no-total-path-001
  - kb-decision-0059
  - kb-reference-macros-ceremony-second-example-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/adr-0033-reopen-ground.md
  - examples/course-subscriptions/src/main.rs
  - examples/transfers-on-sqlite/src/main.rs
  - .kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md
last_reviewed: 2026-09-07
---

# Both worked examples spell `event_type` with an unchecked positional mapping, and ADR-0033's reopen condition is denominated in a currency too coarse to see it

## What is true today

Both worked examples in this workspace spell `DomainEvent::event_type` the
same way, independently:

```rust
fn event_type(&self) -> EventType {
    match self {
        Self::AccountOpened { .. } => Self::EVENT_TYPES[0].clone(),
        Self::Deposited { .. } => Self::EVENT_TYPES[1].clone(),
        Self::Withdrawn { .. } => Self::EVENT_TYPES[2].clone(),
    }
}
```
— `examples/course-subscriptions/src/main.rs:195-201` and
`examples/transfers-on-sqlite/src/main.rs:288-294`, three subscript
expressions each, six in the tree total.

Each arm pairs an enum variant with a hand-written index into
`EVENT_TYPES`, and nothing checks that the pairing is correct or that it
stays correct as either list is edited. Reorder `EVENT_TYPES` and every event
the store subsequently accepts is written under the wrong type — silently, on
the write path, with no compile error, no clause violation, no conformance
rule and no lint catching it. This is not one example's local shortcut: it is
the pattern both independently-written worked examples converge on, which is
itself evidence that nothing in the contract or the constitution currently
steers an implementor away from it.

`kb-decision-0033` — accepted, immutable — decided `happenstance-macros` out
of scope for 0.1 on a specific, mechanical instrument: whether the rewritten
worked example carries more mapping ceremony than domain logic, measured as a
line-range ratio against a 1.0 threshold. Measured against
`course-subscriptions/src/main.rs` at `78a2170`, both extreme assignments of
the one contested block (0.50:1 and 0.12:1) land under the threshold, so the
verdict is "out" either way. A second measurement, taken 2026-09-04 against
both worked examples on `lane/rendered-pages` at `b87b17b`, strengthens the
same verdict rather than weakening it: the `impl DomainEvent` block's share of
the file *fell* on the second example (35 of 663 lines, 5.3%, against 40 of
532, 7.5%), because the impl is a near-fixed cost while the surrounding domain
logic grew.

That measurement is the right instrument for the question it was built to
answer, and it is the wrong instrument, by construction, for this ground. The
six positional-mapping lines are roughly 0.5% of the two files' combined 74
lines of `impl DomainEvent` ceremony — the reopen threshold is 1.0, denominated
in the same volume currency the verdict itself uses. A criterion made entirely
of line-count ratios cannot register a six-line hazard regardless of what that
hazard costs when it fires, and ADR-0033's actual reopen condition
(`.kb/decisions/0033-happenstance-macros-out-of-scope-for-0-1.md`, *Reopen
condition*) is denominated exactly that way: reopen "if and only if" D-1
settles with an infallible `Tags` path, because even the full 85-line
contested block would not cross 1.0 from 0.50:1. The condition was written to
answer "does more ceremony change the verdict," and it does that faithfully.
It was never written to answer "is there an unchecked mapping obligation
worth fixing regardless of ceremony volume," and excludes that ground by the
same construction that makes it correct for its own question.

## What is not decided

Two separable things, deliberately kept apart here because conflating them is
exactly the failure mode this atom exists to avoid. First: whether the
positional mapping should be fixed at all, and if so how — a derive is one
answer (and would reopen the macros question), but a narrower fix needs no
macro crate at all: a `const` assertion pairing each variant with its
`EVENT_TYPES` index, checked at compile time, is a candidate that was never
priced against ADR-0033's ceremony ratio because it produces no ceremony to
measure. Second: whether ADR-0033's reopen condition should itself eventually
be widened to admit non-volume evidence — an amendment in the shape ADR-0029
used on ADR-0004, record-that-amends rather than edit-in-place. That second
question is explicitly not resolved here; filing it as an open question rather
than an ADR amendment is the deliberate, cheaper half of the choice the
source brief considered and did not spend on a decision nobody is currently
contesting.

## What forces it

Whoever next reaches `kb-decision-0033` carrying evidence about this ground
and finds a reopen condition their evidence does not satisfy — a comment, an
issue, or a review entry citing the decision against soundness ground rather
than ceremony volume is the observable trigger, per this repository's own
standard for when a finding raised more than once is a missing check rather
than a coincidence. Independently, `kb-open-question-d-1-no-total-path-001`
settling with an infallible `Tags` path is ADR-0033's own named reopen
trigger, and shares no mechanism with this ground — D-1 settling does not
touch the positional-mapping hazard, and this ground settling does not touch
D-1.

## Ordered sub-questions

1. Is a `const`-assertion-style fix (pairing each variant with its
   `EVENT_TYPES` index at compile time) sufficient, or does the mapping need
   the fuller guarantee a derive would give?
2. If a narrow fix lands in both worked examples without a macro crate, does
   this open question simply close, or does it still leave a residual case
   (a hand-written `DomainEvent` outside the two worked examples) worth
   documenting as a pattern to avoid?
3. Separately and only if raised on its own merits: should ADR-0033's reopen
   condition be amended to admit non-volume evidence in general, and who
   takes that ADR pass?
