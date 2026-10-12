---
id: kb-decision-0091
title: then(&[]) asserts the decision emitted nothing, decide gets no third channel, and no then_nothing() is added
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0091
reversibility: medium
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-then-empty-emission-idiom-001, the two residuals ADR-0046 left, as the
  owner's call (wi-378078, "all as recommended", 2026-10-12). Limb 1, the test DSL: then(&[])
  asserts the decision emitted exactly nothing. That is what Decision::then has always done,
  since it compares the emission against the expected slice, and the in-tree call sites the open
  question read as "not asserting" all decide Vec::new(), so each one asserts what its closure
  does. The meaning is now stated on then, and pinned by then_empty_asserts_that_nothing_was_emitted,
  whose second half fails the wrong reading (an empty slice asserts nothing) by mutation. A
  refusal compared against &[] still panics. No then_nothing() is added: it would be a second
  name for the same comparison, and it is additive at any date if a reader ever needs it. Limb 2,
  the command loop: decide gets no third channel. The closure stays
  FnMut(&B) -> Result<Vec<B::Event>, D>, and CommandOutcome stays two-armed and exhaustive. A
  deliberate no-op and an under-tagged fold's empty decision keep one representation, because a
  fold that missed events cannot know it missed them, so a "nothing to do" channel would carry
  the same answer from both and would label the bug rather than detect it. The defences against
  the under-tagged fold stay where they are: the derived query (ADR-0020), assert_domain_event,
  and the DSL's "seeded but NOT selected" region. Nothing breaks. The scope-coverage helper
  question, which waited on this one, is free to answer on its own.
depends_on:
  - kb-decision-0046
  - kb-decision-0066
related:
  - kb-decision-0012
  - kb-decision-0020
  - kb-decision-0046
  - kb-decision-0066
  - kb-decision-0072
  - kb-open-question-then-empty-emission-idiom-001
  - kb-decision-wi-378078
source_paths:
  - crates/happenstance/src/testing/mod.rs
  - crates/happenstance/src/command.rs
  - crates/happenstance/tests/dsl_failure_message.rs
  - .kb/open-questions/then-empty-emission-idiom-and-the-nothing-to-do-channel.md
last_reviewed: 2026-10-12
---

# then(&[]) asserts the decision emitted nothing, decide gets no third channel, and no then_nothing() is added

**Accepted by the owner on 2026-10-12** as `kb-decision-wi-378078` ("all as recommended", for
#93's "option A on both questions"). No long-form record exists; this atom is the record. It
lands as a `door:one-way` change, so the owner merges it.

## Context

ADR-0046 gave `commit` a two-armed, `#[must_use]` outcome, `CommandOutcome::Committed` or
`CommandOutcome::Nothing`, and left two questions open, which
`kb-open-question-then-empty-emission-idiom-001` holds. The first is what `then(&[])` means in the
test DSL now that an empty decision has its own arm. The second is whether `decide` should say
"nothing to do" apart from "refused", or apart from a fold that silently saw too little. ADR-0066
classified both as breaking-if-answered. A third `CommandOutcome` variant breaks a type that is
deliberately not `#[non_exhaustive]` (`crates/happenstance/src/command.rs:112-115`), and so does
widening the closure's return type. ADR-0072 placed the closure in phase 17b.

## Decision

1. **`then(&[])` asserts the decision emitted exactly nothing.** `Decision::then` compares the
   emission against `expected` (`crates/happenstance/src/testing/mod.rs:300`, `pub fn then(`), so
   an empty slice is a claim like any other: a decision that emitted one event fails it, and the
   failure renders that event under `actual:`. The open question counted three in-tree call sites
   where `&[]` seemed to mean "not asserting". All three decide `Vec::new()`
   (`crates/happenstance/src/testing/tests.rs:286`, `:346`, and
   `crates/happenstance/tests/dsl_failure_message.rs:437`), so each asserts what its closure does.
   The doc on `then` now says this, and `then_empty_asserts_that_nothing_was_emitted`
   (`crates/happenstance/tests/dsl_failure_message.rs:573`) pins both halves. Its wrong
   implementation is `then` passing whenever `expected` is empty, and that turns it red. A refusal
   compared against `&[]` still panics naming the refusal
   (`then_on_a_refused_decision_panics_naming_the_refusal`).
2. **No `then_nothing()`.** It would be a second spelling of `then(&[])`, and two spellings of one
   assertion invite a reader to look for the difference between them. It is additive at any date,
   so declining it now costs nothing to reverse.
3. **`decide` gets no third channel.** The closure stays `FnMut(&B) -> Result<Vec<B::Event>, D>`,
   and `CommandOutcome` keeps its two arms with no `#[non_exhaustive]`. A deliberate no-op and an
   under-tagged model's empty decision keep one representation. That is the honest one: a fold
   that never saw an event cannot report having missed it. A distinct "nothing to do" signal would
   be returned by the buggy closure exactly as by the correct one, so it would label the defect
   and not detect it. The defences against an under-tagged fold stay where they act:
   - the query is derived from `EVENT_TYPES`, so it cannot be hand-narrowed (ADR-0020);
   - `assert_domain_event` checks a domain enum's variants against its declaration;
   - a failed `then` renders the events that were seeded and not selected.

## Consequences

- No signature, behaviour or variant changes. The published surface is the surface `0.4.1`
  shipped. The change is a doc paragraph on `then`, one test and this record.
- From `1.0.0`, a third `CommandOutcome` variant or a wider `decide` return type is a major
  release and needs a record superseding this one.
- `scope-coverage-helper-and-the-projection-port-gap.md` sequenced its additive helper after this
  question so it would not have to rationalise a two-function DSL surface. This record adds no
  function, so that question is free to add its own.

## What reopens it

A shipped defect that a distinct "nothing to do" channel would have caught, and that the derived
query, `assert_domain_event` and the DSL's fourth region all missed. For limb 1, a reader who
misread a passing `then(&[])` as "not checked".

## Alternatives that lost

- **`then(&[])` refuses an empty emission, with `then_nothing()` as the replacement.** That is a
  behaviour break on the default `happenstance::testing` module, and it buys a second name for an
  assertion the first already makes.
- **Add `then_nothing()` beside `then(&[])`.** It was the phase-16 note's likeliest answer. It is
  free and additive, and it is declined because it adds a name without adding a check.
- **A `Decided::Nothing` arm in the closure's return type, or a richer `D`.** Either breaks
  `commit`'s signature, and neither can tell a deliberate no-op from a blind fold, because the
  closure is the only witness and it is the party that does not know.
