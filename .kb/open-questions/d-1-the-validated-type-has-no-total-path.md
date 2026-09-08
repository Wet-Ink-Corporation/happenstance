---
id: kb-open-question-d-1-no-total-path-001
title: D-1 — a validated identifier has no infallible route, in either direction
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Two faces of one defect, both bearing on VT-18 which is FROZEN, both recorded by phase 7's use of
  the frozen contract and both named by ADR-0020 as defect candidate D-1 with a decision record
  rather than a line edit as their route. Inward: QueryItem::new is fallible even when the caller
  already holds validated EventTypes and Tags, so every derived query carries a Result unreachable
  for a well-formed model; and because Boundary is sealed, that error arm is untestable from
  outside happenstance-core, since no downstream type can be a failing Boundary. Outward:
  DomainEvent::tags returns Tags totally while every route into Tags is fallible, so an implementor
  whose tag values are runtime strings has no total path except to invent a newtype holding the
  validated Tag — which the worked example does, at a cost of 81 lines. What is not decided is
  whether happenstance-core grows an infallible constructor for pre-validated inputs, what it is
  called, and whether one answer serves both faces or only the first; ADR-0020 routed it to a
  decision record and named VT-18 as its nearest clause subject, and no record has been written.
  What forces it is the first API change after 0.1, and the macros verdict, which names this
  defect's settlement with an infallible Tags path as the single condition that would reopen AC-013
  and require its 2026-08-16 measurement to be re-taken.
depends_on: []
related:
  - kb-decision-0020
  - kb-decision-0015
  - kb-decision-0033
  - kb-decision-0059
  - kb-open-question-event-type-positional-mapping-001
  - kb-open-question-scope-coverage-helper-projection-gap-001
source_paths:
  - .kb/_intake/contract-defect-log-phase-7.md
  - .kb/_intake/happenstance-macros-verdict.md
  - references/evaluation/phase-7-contract-defects.md
  - crates/happenstance-core/src/query.rs
  - crates/happenstance-core/src/tag.rs
  - examples/course-subscriptions/src/main.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-17
---

# D-1 — a validated identifier has no infallible route, in either direction

## What is true today

Two findings from the phase-7 contract defect log are one defect, not two: the log itself calls the
second "D-1's other face," both bear on the same `[FROZEN]` clause, VT-18, and both were surfaced by
the same activity — building a real application against the frozen contract for the first time. ADR-0020
(`kb-decision-0020`) already names the pair as defect candidate D-1 and already routes it to a
decision record rather than a line edit; that record has not been written, and this atom exists so
the gap is a findable open question rather than a silent absence.

**Inward.** `QueryItem::new` (`crates/happenstance-core/src/query.rs:48-62`) is fallible even when
the caller already holds a validated `EventType` and validated `Tag`s — constructed, in other words,
from values that have already cleared every check `QueryItem::new` could perform. Every derived query
therefore carries a `Result` whose error arm is unreachable for a well-formed model. The problem is
stronger than "an unreachable arm is untidy": `Boundary` is `sealed`, so this error arm is
**untestable from outside the crate** — no downstream type can construct a failing `Boundary`, which
means the fallibility cannot even be exercised by a caller who wanted to prove it dead code.

**Outward.** `DomainEvent::tags` returns `Tags` totally — `fn tags(&self) -> Tags`, no `Result` — while
every constructor route into `Tags` is fallible. An implementor whose tag values come from runtime
strings (the ordinary case: a course ID, a student ID) has no total path from string to `Tags` and
therefore no way to implement `tags` without either panicking on a value it cannot prove valid ahead
of time, or building a second, hand-rolled newtype that holds an already-validated `Tag` and defers
the fallible step to construction time. The worked example takes the second route, and it costs 81
lines (`examples/course-subscriptions/src/main.rs:102-182`), a cost its own inline documentation
names as a cost.

## What is not decided

Whether `happenstance-core` grows an infallible constructor for pre-validated inputs — an API that
lets a caller who already holds a validated `EventType` and `Tag`s assemble a `QueryItem` or a `Tags`
without re-running fallible validation — and, if so, what it is called and how it is guarded so it
cannot be used to smuggle an unvalidated value past the type. It is also not decided whether one
answer serves both faces (inward and outward) or whether they need two separate infallible routes,
since the inward face is about assembling a query from parts and the outward face is about producing
`Tags` from a `DomainEvent` implementor's own runtime data. ADR-0020 named VT-18 as the nearest clause
this settles against, and routed the whole question to a decision record precisely because the fix
carries alternatives worth weighing rather than being a self-evident line edit — inventing the
constructor without weighing those alternatives would be exactly the kind of silent architectural
commitment this repository's KB discipline exists to prevent.

## What forces it

Two triggers. The first API change made after 0.1 is the natural point to settle this, since VT-18 is
`[FROZEN]` and any infallible-construction API is itself a contract addition that wants the same
scrutiny the rest of `happenstance-core`'s surface got. The second is more concrete: the
`happenstance-macros` scope verdict names this defect's settlement, specifically with an infallible
`Tags` path, as the **single condition** that would reopen its own closed question (AC-013) — a
derive that could also produce tags from runtime values would reach further into the worked example's
current hand-written ceremony than the 40 lines its 2026-08-16 measurement counted, and that
measurement would need to be retaken rather than assumed to still hold.
