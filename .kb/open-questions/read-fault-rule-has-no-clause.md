---
id: kb-open-question-read-fault-rule-no-clause-001
title: The read-fault rule checks a real hazard and belongs to no clause
kind: open_question
status: accepted
authority_tier: note
summary: >-
  arming_a_read_fault_makes_the_stream_yield_an_error is a live conformance rule -- arm a fault
  mid-stream after seeding and anchor-reading four events, and require the stream to yield an Err
  item rather than end silently -- landed with a defaulted Fixture::READ_FAULT capability and a
  SwallowedReadFaultStore mutant that fails exactly it. No clause in spec/SPECIFICATION.md obliges
  it: ES-2 governs read's signature -- the stream returned at the top level, the method not async
  -- and says nothing about what the Err arm must contain. The rule is not decorative: against
  SwallowedReadFaultStore with the fault unarmed it fails 0 of 89 rules, and with the fault armed
  by hand it fails 22, which is a hole in the suite rather than a preference about it. It is
  disposed of through UNCLAIMED_PENDING_ADR, printed on every green cargo xtask spec-trace run,
  because minting the clause is the specification owner's act and the lane that landed the rule
  did not take it. Sibling shape to disjoint-boundaries-have-no-clause and
  model-family-rule-has-no-clause: a third permanent-until-claimed entry on a list whose own
  documentation says it can only shrink.
depends_on: []
related:
  - kb-open-question-disjoint-boundaries-no-clause-001
  - kb-open-question-model-family-rule-no-clause-001
  - kb-open-question-postgres-read-fault-declension-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/read-fault-clause-and-capability.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-testkit/src/suite.rs
last_reviewed: 2026-09-07
---

# The read-fault rule checks a real hazard and belongs to no clause

## What is true today

`arming_a_read_fault_makes_the_stream_yield_an_error` is a live conformance rule, landed as part of L3-01's response to a gap in the pre-publication review: seed four events, read them once unarmed as an anchor, arm a fault, read again, and require the resulting stream to surface an `Err` **item** rather than simply end. It ships with `Fixture::READ_FAULT` and `Fixture::arm_read_fault`, both defaulted under the fixture-declension-policy shape already adopted elsewhere, so no existing fixture had to move to keep compiling. `SwallowedReadFaultStore` is registered as the mutant that fails exactly this rule; `PagedStreamStore`, meeting the same armed failure, is shown passing it, so the rule is demonstrated as both failable and passable rather than being decorative.

No clause in `spec/SPECIFICATION.md` obliges it. ES-2 is the obvious candidate and is the wrong one: it governs `read`'s *signature* — that the stream comes back at the top level and the method is not `async` — and says nothing about what a store must do with a failure encountered partway through. `cargo xtask spec-trace`'s check 6 exists precisely for this asymmetry: a rule with no clause is treated as seriously as a clause with no rule, because CF-24's discipline runs in both directions. The rule was landed anyway, on the strength of a measurement rather than a preference: `SwallowedReadFaultStore` fails **0 of 89** rules with the fault left unarmed and **22** with it armed by hand — a real hole in the suite's coverage, not a hypothetical one. What could not be done in the same change is write the normative sentence, because minting a specification clause is the specification owner's act and the lane implementing the rule did not hold that role.

The rule is now listed in `xtask/src/spec_trace.rs`'s `UNCLAIMED_PENDING_ADR`, which prints on every green `spec-trace` run — the same disposition mechanism already carrying `k_disjoint_boundaries_admit_exactly_k_commits` (`kb-open-question-disjoint-boundaries-no-clause-001`) and `ops_agree_with_the_model` (`kb-open-question-model-family-rule-no-clause-001`). This is a third entry on a list whose own documentation states it "can only shrink," and it differs in shape from both siblings: the disjoint-boundaries gap is one proposition no clause states, and the model-family gap is several propositions belonging to no single clause; this one is a single, narrow proposition about the `Err` arm of one method, and the sentence needed to close it is not contested — only who is authorised to write it, and when.

## What is not decided

Three shapes were named for closing it: mint a new ES-clause stating that a store must surface a mid-`read` failure as an `Err` stream item and must not report it as end-of-stream (landing `[PROVISIONAL]` at most, since no adapter has yet armed a real fault); widen ES-2 to carry the obligation, which is strictly more expensive because ES-2 is `[FROZEN]` and a frozen clause cannot be edited without the repair-frozen-clause discipline; or leave it in `UNCLAIMED_PENDING_ADR` indefinitely, which is honest and free but leaves the suite's newest rule as the only one an adapter author could contest on the ground that nothing obliges it.

## What forces it

The specification owner's attention before phase 12, on the reasoning already recorded against Option C: an unclaimed rule is advisory in a suite whose entire argument is that rules are not advisory, and every additional cycle it stays unclaimed is a cycle in which an adapter author can correctly say no clause requires what the rule demands of them.

## Ordered sub-questions

1. Does the specification owner mint the new clause (Option A) before phase 12, as recommended, or does the presence of two prior `UNCLAIMED_PENDING_ADR` entries argue for batching all three into one ADR pass rather than resolving this one alone?
2. Should `arm_read_fault` take an index argument the way `arm_mid_batch_fault(after)` does, so a rule can distinguish "failed at the first poll" from "failed mid-stream" — and if so, what does a non-paging adapter do with an index that names a page boundary it doesn't have?
3. Does `LogError::ReadFailed`, added beside `WriteFailed` as the second variant a conformant store in the mutation binary produces, belong in the shared correct core as a contract-level fact, or does it stay a test-support convenience?
4. Should `happenstance-cloudflare` or `happenstance-neon` arm a real read fault over a real medium before phase 12, so the rule's coverage stops being entirely in-process — the same question `kb-open-question-postgres-read-fault-declension-001` raises for `happenstance-postgres` specifically?
