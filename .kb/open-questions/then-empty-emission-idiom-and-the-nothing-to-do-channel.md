---
id: kb-open-question-then-empty-emission-idiom-001
title: What does then(&[]) mean once an empty decision has its own outcome arm, and can decide say "nothing to do" separately from "refused"?
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0046 (the ratified successor to the empty-decision-outcome brief) settled that commit grows a
  two-armed, #[must_use] outcome so a decision that emits nothing is unignorable at compile time
  rather than a silent Ok or a runtime error, and it deliberately left two residuals for later. The
  crate's own test DSL has five in-tree `.then(&[])` call sites where the empty slice means "I am not
  asserting on this decision" rather than "I assert it emitted nothing," and three of those five are
  green passes that would have to change if `then` on an empty emission started rejecting it the way
  it already rejects comparing a refusal against `&[]`; nobody has designed what replaces them
  (a `then_nothing()` method, a panic naming the ambiguity, or a different idiom entirely). Separately,
  decide's closure returns Result<Vec<Event>, D>, which gives an under-tagged model's empty fold and a
  handler's genuine "no action needed" the identical representation as whatever ADR-0046 chose for
  emitting nothing, and no design distinguishes a deliberate no-op from a fold that silently missed
  events it should have seen. Both are named explicitly as unsettled by the accepted decision itself
  and are one design area because an answer to the DSL question likely shapes what a distinct
  "nothing to do" channel would need to look like on the decide side.
depends_on: []
related:
  - kb-decision-0012
  - kb-decision-0020
  - kb-decision-0031
  - kb-decision-0030
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/empty-decision-outcome.md
last_reviewed: 2026-09-07
---

# What does then(&[]) mean once an empty decision has its own outcome arm, and can decide say "nothing to do" separately from "refused"?

## What is true today

`happenstance`'s command loop had two disagreeing answers to one question: a
decision closure that emits nothing reaches `store.append(&[], …)`, which the
store correctly refuses under ES-9's sibling clause ES-20 (`[FROZEN]` — an
empty batch is refused, and refused before any condition is checked) — but
the crate's own test DSL treats the identical decision as a green pass,
because `.then(&[])` compares an emission against an expected slice and an
empty emission against an empty expectation matches. The audit that surfaced
this (Y-1) found no clause governs the caller-side question at all —
`spec/SPECIFICATION.md` puts the typed layer outside the clause space
entirely — and no decision atom mentioned `Committed` anywhere in `.kb/`.

The record that resolved it (ADR-0046, superseding the empty-decision-outcome
brief's earlier draft after adversarial review reversed its own first
recommendation) chose a two-armed, `#[must_use]` success outcome for `commit`:
committed-with-position, or nothing-to-do-with-attempt-count, forcing every
call site to name which arm it is in rather than letting the common case
silently absorb the uncommon one. That choice is loud at compile time, at
every call site, which is the property the brief's own analysis found Option 1
(a runtime-only error) could not match. It explicitly did not resolve — and
said so in its own "what this does not settle" section — two further
questions, merged here because they are one design area with one shared
deadline: the published-surface break the outcome-type change already spends.

## What is not decided

**How `then(&[])` reads once the caller side has its own outcome type.** The
DSL's `then` method already distinguishes one silent pass deliberately —
comparing a refusal against `&[]` panics, naming the fact, because the doc
comment says exactly that shape is the silent pass the method exists to
prevent. The empty-*emission* pass is a case that doc does not cover, and five
in-tree call sites currently use `.then(&[])` for it: two are already inside
`caught(|| …)` blocks expecting a panic for unrelated reasons, but three are
plain green passes where `&[]` means "I am not asserting on the decision,"
not "I assert it emitted nothing." Making `then(&[])` reject an empty emission
would require rewriting those three into whatever new idiom is chosen —
a `then_nothing()` method, a panic pointing at this same ambiguity, or
something else — and nobody has designed which.

**Whether `decide` can express "nothing to do" and "refused" as two distinct
things.** Every option the settled decision considered pushes both into the
same channel from the closure's point of view: the closure's `Result<Vec<Event>,
D>` return type gives an under-tagged model's silently-empty fold (a
correctness bug — a decision made on a narrower log than the handler
believes) and a handler's deliberate no-op the same representation on the way
in, however `commit` chooses to represent the outcome on the way out. The
settled decision's own recommendation stated plainly that its two-armed
outcome "still does not distinguish 'deliberate no-op' from 'forgot to
push'" — it converts a silent bug into an unignorable value with an extra
arm, but a caller who matches that arm with a bare `_ => ()` has paid the
ceremony and bought none of the safety the ambiguity would otherwise cost
them.

## What forces it

Neither residual has an independent deadline; both inherit ADR-0046's own,
because the outcome-type change that created the need for both is a
behaviour-breaking change free only while `0.2.0-alpha.1` is a pre-release
and permanent the moment `0.2.0` ships. A separate, adjacent decision
(`scope-coverage-helper-and-the-projection-port-gap.md`) explicitly sequences
its own additive test helper *after* this question is settled, on the ground
that landing the helper first would hand this question a two-function surface
to rationalise rather than a live question to answer — so this question also
gates that one.

## Ordered sub-questions

1. Does `then(&[])` on an empty emission become a designed idiom
   (`then_nothing()` or similar) before or after the outcome-type change
   ships, given that the three plain-pass call sites will not compile against
   a stricter `then` without some replacement existing first?
2. If `decide` gains a distinct "nothing to do" signal, does it live in the
   closure's return type (widening it beyond `Result<Vec<Event>, D>`), in a
   richer `D`, or somewhere else entirely — and does that choice constrain or
   free the DSL answer to sub-question 1?
3. Once both are answered, does the scope-coverage helper question
   (`scope-coverage-helper-and-the-projection-port-gap.md`) inherit a specific
   shape to rationalize against, or does it remain free to add its own
   function independently?
