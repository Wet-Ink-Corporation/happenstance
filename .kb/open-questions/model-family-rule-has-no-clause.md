---
id: kb-open-question-model-family-rule-no-clause-001
title: The model family's rule checks the composition of seven clauses and belongs to none
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ops_agree_with_the_model is the model family's single conformance rule; it replays generated
  operation sequences against a model and therefore checks the composition of ES-8, ES-9, ES-11,
  ES-14, ES-15, ES-18 and ES-25 over inputs no clause enumerates. §6.4 names it only as CF-22's
  illustration, and CF-22's MUST is where the rule list lives rather than what any rule asserts.
  Distinct from the disjoint-boundaries gap: that is one proposition no clause states, this is
  several belonging to no single clause. Settled by an ADR minting a cross-clause property clause,
  or by deciding how cross-clause rules are disposed of — which becomes the precedent for every
  future property-based rule. Owner unassigned; held in UNCLAIMED_PENDING_ADR.
depends_on: []
related:
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-open-question-disjoint-boundaries-no-clause-001
source_paths:
  - .kb/_intake/gaps-owed-a-decision.md
  - crates/happenstance-testkit/src/model.rs
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
last_reviewed: 2026-08-10
---

# The model family's rule checks the composition of seven clauses and belongs to none

## What is true today

`ops_agree_with_the_model` (`crates/happenstance-testkit/src/model.rs:598`) is the model family's
single conformance rule. It replays a generated sequence of appends, conditional appends and reads
against a model implementation and compares every answer the store under test gives against what
the model gives. What it checks is therefore the **composition** of ES-8, ES-9, ES-11, ES-14,
ES-15, ES-18 and ES-25 over inputs no clause enumerates — the rule is a property test, not a fixed
scenario, so the inputs it exercises are generated rather than written down anywhere.

§6.4 does name the rule, but only as CF-22's illustration of a per-family enumeration; CF-22's MUST
is where the rule *list* lives, not an assertion about what any individual rule in that list
checks. Claiming `ops_agree_with_the_model` under any single one of the seven clauses it exercises
would misstate what that one clause's rule table covers.

This gap differs in shape from the disjoint-boundaries gap
(`kb-open-question-disjoint-boundaries-no-clause-001`), and the difference matters for how each is
settled. That gap is one proposition no clause states. This one is a rule stating *several*
propositions at once and belonging to no single clause for exactly that reason — the two cannot be
closed by the same ADR move, and a reader who resolves one should not assume the other is resolved
too. It is the second of two entries held in `UNCLAIMED_PENDING_ADR`
(`xtask/src/spec_trace.rs:1968-1997`), which prints both on every green `cargo xtask spec-trace`
run.

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
2. If a clause is minted, is it scoped to the seven clauses this specific rule composes, or stated
   generally enough to cover future property rules over a different clause set?
3. Whichever way this resolves, does it change how the disjoint-boundaries gap should be framed —
   should the two be settled by one ADR, given both are UNCLAIMED_PENDING_ADR entries with related
   but non-identical shapes, or does bundling them risk conflating a one-proposition gap with a
   many-proposition one?
