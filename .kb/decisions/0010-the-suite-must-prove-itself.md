---
id: kb-decision-0010
title: The conformance suite's own proof obligation
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0010
reversibility: medium
phase: 3
supersedes: null
superseded_by: null
summary: >-
  Twenty-seven conformance rules had never been shown to reject anything, and four plausible wrong
  implementations passed the suite as written. So a rule may not be added until a store exists
  that fails it: every rule owes a mutant with stated provenance, naming the real implementation
  mistake it models, and three meta-tests enforce it — every rule has a mutant, each mutant fails
  exactly its declared rules and passes every other, and conformant variants pass everything. No
  pass rate is ever quoted over the mutant set. The fixture contract changes from a fresh empty
  store to a factory that can be asked more than once for a handle onto the same backing store,
  and a capability an adapter declines is skipped with its stated reason rather than vanishing — a
  skip must be reported, never silent. The runtime wrapper stays a parameter, ratified from phase
  1. Its fourth section was amended five times in place: three rules were proposed for retirement
  and none was retired, and MID_BATCH_FAULT was minted because a fault must be injected by the
  fixture rather than by a decorator. Rejected: cargo-mutants over the reference store, a
  mechanical mutant per rule, and rejection without exactness.
depends_on: []
related:
  - kb-reference-port-traits-compiled-findings-001
  - kb-playbook-repair-frozen-clause-001
source_paths:
  - .kb/_intake/0010-the-suite-must-prove-itself.md
  - references/adr/0010-the-suite-must-prove-itself.md
  - crates/happenstance-testkit/src/fixtures.rs
  - crates/happenstance-testkit/src/registry.rs
  - crates/happenstance-testkit/src/suite.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-10
---

# The conformance suite's own proof obligation

## Context

`happenstance-testkit` decides whether an adapter exists, and it carried twenty-seven rules that
had never been shown to reject anything — written by reading the contract and asserting what it
says, which is how every conformance suite starts and is not sufficient, because the failure mode
is silent: a rule nothing plausible gets wrong passes forever and is indistinguishable from a good
rule until an adapter with the matching bug passes it too. This repository had already met that
shape three times in its own tooling rather than in a rule, and a reviewer's own measurement made
it concrete: four plausible wrong implementations — `LIMIT` applied before the tag filter, an
OR-ed query returning a matching event twice, `COUNT(*)+1` position allocation, and probe-then-
insert outside the transaction — passed the suite as it stood.

## Decision

A rule may not be added until a store exists in the testkit's own `tests/` that fails it, and
declares which rules it fails. Three meta-tests hold the obligation as the phase's proof artefact:
`every_rule_has_a_mutant` makes a decorative rule impossible to add, since adding one fails this
test until its wrong implementation is named; `mutants_fail_exactly_their_declared_rules` requires
a mutant to fail the rules it declares and pass every other — the load-bearing half, because a
mutant failing everything proves the store is broken rather than that a rule works; and
`conformant_variants_pass_everything` is the positive control, without which the first two are
satisfied by a harness that fails unconditionally. Each mutant states its provenance — the real
implementation mistake it models — because a mutant invented to fail a rule is circular, and no
pass rate is ever quoted over the mutant set, since it is an author-chosen bug sample rather than a
measure of the suite's quality.

The fixture contract changes from a fresh empty store to a factory that can be asked more than
once for a handle onto the same backing store, landing before phase 8 writes the first real
conformance file rather than after, since the fixture shape is the suite's public API. An adapter
that cannot honour repeated handles declares so, and the rules needing it are skipped with their
stated reason — a skip must be reported, never silent, because a suite quietly running twenty-one
of twenty-seven rules while printing green actively misleads, worse than one that runs twenty-one
and says so. The runtime wrapper stays a parameter, ratified rather than newly decided: phase 1
built `for_each_event_store_rule!` as the sole rule enumeration, taking a macro path, and nothing
has argued with it since.

This ADR's own fourth section was amended five times in place while it was being written, and the
amendment history is itself the demonstration of its rule: `append_is_atomic` was first proposed
for retirement on the ground that a condition-check rejection never reaches the write path, then
the ground was found false once `WriteThenCheckStore` — a real probe-then-insert mutant — was
written and failed by it, so the retirement reversed. Two further rules proposed for retirement
were reclaimed the same way. The running count of actual retirements is zero, not two, and the
successor rule `append_is_atomic_under_a_mid_batch_fault` needed a fault injected by the fixture,
not a decorator over an arbitrary store, because nothing outside one transaction can reach between
two rows of it — so `MID_BATCH_FAULT` was minted as a third `Fixture` capability constant.

## Consequences and alternatives rejected

Good: the suite acquires the discriminating property it exists to give adapters, and a decorative
rule becomes unwriteable rather than merely discouraged. Bad and the real cost: every future rule
now costs a rule and a mutant, roughly doubling the price of a conformance rule — the intended
effect, since the cheapest rule to write is the one most likely to be decorative. Mutants are a
second in-tree implementation of the port that must keep compiling as the port changes.

Rejected: `cargo-mutants` over the reference store, which measures whether the suite pins
`MemoryEventStore`'s code rather than whether it catches a different storage engine's mistakes — a
SQL `LIMIT` bug has no in-memory analogue to mutate into; a mechanical mutant per rule, which makes
the mutant-existence test trivially satisfiable; and rejection without exactness, which a mutant
failing everything satisfies by accident while proving nothing about which rule caught it.
