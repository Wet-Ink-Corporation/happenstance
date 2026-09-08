---
id: kb-open-question-model-family-rule-no-clause-001
title: The model family's rule checks a composition of clauses and belongs to none
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ops_agree_with_the_model is the model family's single conformance rule; it replays generated
  operation sequences against a model and therefore checks the composition of ES-8, ES-9, ES-11,
  ES-14, ES-15, ES-16, ES-18 and ES-25 over inputs no clause enumerates. ES-16 joined that list
  when the `to` generator landed, which is the first evidence that the composed set is not fixed
  and grows with the generator. §6.4 names the rule only as CF-22's illustration, and CF-22's MUST
  is where the rule list lives rather than what any rule asserts. Distinct from the
  disjoint-boundaries gap: that is one proposition no clause states, this is several belonging to
  no single clause. A further complication is now measured rather than suspected — the rule's
  discriminating power is a function of PROPTEST_CASES, with a cliff at 192 against a default of
  256, so any clause minted for it asserts something whose truth is environment-dependent.
  Settled by an ADR minting a cross-clause property clause, or by deciding how cross-clause rules
  are disposed of — which becomes the precedent for every future property-based rule. Owner
  unassigned; held in UNCLAIMED_PENDING_ADR, which now carries three entries rather than two.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-read-fault-rule-no-clause-001
  - kb-reference-model-family-case-cliff-001
  - kb-decision-0048
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - .kb/_intake/remediation-2026-09-04-briefs/op-read-non-exhaustive.md
  - .kb/_intake/remediation-2026-09-04-briefs/model-family-case-floor.md
  - crates/happenstance-testkit/src/model.rs
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
last_reviewed: 2026-09-07
---

# The model family's rule checks a composition of clauses and belongs to none

## What is true today

`ops_agree_with_the_model` (`crates/happenstance-testkit/src/model.rs:598`) is the model family's
single conformance rule. It replays a generated sequence of appends, conditional appends and reads
against a model implementation and compares every answer the store under test gives against what
the model gives. What it checks is therefore the **composition** of ES-8, ES-9, ES-11, ES-14,
ES-15, **ES-16**, ES-18 and ES-25 over inputs no clause enumerates — the rule is a property test,
not a fixed scenario, so the inputs it exercises are generated rather than written down anywhere.

**ES-16 is a correction, and it carries a lesson the original enumeration did not.** This atom
listed seven clauses; the `to` generator landed in the 2026-09-04 remediation wave, `Op::Read`
grew a `to` field, and the rule now exercises ES-16 — *an upper bound on a read* — as well. The
correction was handed over by the lane that made the change
(`kb-decision-0048`'s brief names it explicitly rather than leaving it to be discovered). The
lesson is that **the composed set is not a fixed property of the rule**: it is whatever the
generator currently emits, and it grew by one without anybody editing a clause. That bears
directly on sub-question 2 below, because a clause scoped to an enumerated set of clauses would
have gone stale on the day the `to` field landed.

**And the rule's discriminating power depends on a number nobody had measured.** The composition
is only checked over sequences actually drawn, and the draw count is `PROPTEST_CASES` from the
adapter author's environment: `kb-reference-model-family-case-cliff-001` records the cliff at 192
against a default of 256, measured both before and after the `to` change. The point for *this*
gap is not the floor — that is a testkit decision with its own owner — but that a clause minted
for this rule would assert a property whose truth is environment-dependent, and would have to say
what it is quantified over or be false at `PROPTEST_CASES=100`.

§6.4 does name the rule, but only as CF-22's illustration of a per-family enumeration; CF-22's MUST
is where the rule *list* lives, not an assertion about what any individual rule in that list
checks. Claiming `ops_agree_with_the_model` under any single one of the eight clauses it exercises
would misstate what that one clause's rule table covers.

This gap differs in shape from the disjoint-boundaries gap
(`kb-open-question-disjoint-boundaries-no-clause-001`), and the difference matters for how each is
settled. That gap is one proposition no clause states. This one is a rule stating *several*
propositions at once and belonging to no single clause for exactly that reason — the two cannot be
closed by the same ADR move, and a reader who resolves one should not assume the other is resolved
too. It is the second of the **three** entries now held in `UNCLAIMED_PENDING_ADR`
(`xtask/src/spec_trace.rs`), which prints each of them on every green `cargo xtask spec-trace`
run; the third, `kb-open-question-read-fault-rule-no-clause-001`, is a third distinct shape again
— one narrow proposition about a single method's `Err` arm.

## What is not decided

Whether the model family earns a clause of its own — *a store agrees with the contract over
arbitrary operation sequences, not only over the examples the suite enumerates* — or whether the
absence of a clause for a cross-clause rule is accepted as a category the spec does not try to
cover, with `ops_agree_with_the_model` disposed of some other way. Neither option is obviously
better: minting the clause gives the property a citable home but risks stating something CF-22
already implies; declining to mint one is honest about scope but sets the precedent for every
future property-based rule this suite ever adds.

## What forces it

The same pressure as the disjoint-boundaries gap: `UNCLAIMED_PENDING_ADR` printing on every gate
run, and the moment someone needs to cite spec authority for `ops_agree_with_the_model` and finds
nothing to point at. It is also forced by scale — this suite is exactly the kind of place new
property-based rules get added as adapters mature, and each one will face the same "which clause
owns this" question until a precedent exists.

## Ordered sub-questions

1. Does a cross-clause property rule need a clause at all, or is CF-22's per-family enumeration
   sufficient spec coverage for rules of this shape? This is the fork the whole gap turns on.
2. If a clause is minted, is it scoped to the eight clauses this specific rule composes, or stated
   generally enough to cover future property rules over a different clause set? The `to` change
   has already answered half of this empirically: an enumerated scope went stale once, without a
   clause edit, in the ordinary course of improving the generator.
3. If a clause is minted, what is it quantified over? Over *every* sequence the generator can
   emit, which is honest but is not what any run checks; or over the sequences a stated case
   count draws, which makes the clause's truth depend on `PROPTEST_CASES` and couples a
   specification clause to a testkit knob.
4. Whichever way this resolves, does it change how the other two gaps should be framed — should
   all three be settled by one ADR, given each is an UNCLAIMED_PENDING_ADR entry with a related
   but non-identical shape, or does bundling them risk conflating a one-proposition gap, a
   many-proposition one, and a single-`Err`-arm one?
