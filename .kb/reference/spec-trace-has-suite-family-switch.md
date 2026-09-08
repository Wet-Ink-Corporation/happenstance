---
id: kb-reference-spec-trace-has-suite-001
title: spec-trace's has_suite is a per-family switch, and PS sat outside it for two slices
kind: reference
status: accepted
authority_tier: note
summary: >-
  As of 2026-08-15, cargo xtask spec-trace decides per clause family whether to check the
  conformance-rule names a clause cites, through has_suite in xtask/src/spec_trace.rs; a family
  absent from that switch is a family whose rule citations nothing checks. PS was excluded when the
  projection suite did not exist, the exclusion outlived the suite by two slices, and until it was
  flipped a PS clause could cite a rule that had never existed with every gate in the repository
  green. The flip is now held by
  spec_trace::tests::the_projection_family_is_checked_against_its_suite and the one family that
  still abstains, SY, is held by the test beside it, so widening the switch to everything is a build
  failure rather than a judgement call. The transferable shape is an exclusion written for a true
  reason, kept after the reason expired, and structurally invisible because the thing it disables is
  itself a check — the same shape the ES-7 and VT-9 markers carry one level up, and the property a
  ratchet's fail-on-discharge exemption list exists to supply and this switch did not have.
depends_on: []
related:
  - kb-decision-0030
  - kb-open-question-provisional-falsifiers-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-reference-phase-4-5-spec-reconciliation-001
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-open-question-no-ps-rule-name-resolved-001
  - kb-reference-phase-8-spec-reconciliation-001
  - kb-open-question-cf-38-case-naming-no-clause-001
  - kb-open-question-dagger-convention-vs-maturity-markers-001
  - kb-reference-spec-trace-unresolved-declarations-001
source_paths:
  - .kb/_intake/2026-08-15-adr-0030-checkpoint-progress.md
  - references/adr/0030-the-checkpoint-reports-the-commits-that-happened.md
  - xtask/src/spec_trace.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
---

# spec-trace's has_suite is a per-family switch, and PS sat outside it for two slices

## What is true today

`cargo xtask spec-trace`'s check 4 — does every conformance rule a clause cites actually exist —
does not run against every clause family uniformly. It runs only against families `has_suite`
admits (`xtask/src/spec_trace.rs:2501-2506`):

```rust
fn has_suite(clause_id: &str) -> bool {
    clause_id.starts_with("ES-")
        || clause_id.starts_with("VT-")
        || clause_id.starts_with("WF-")
        || clause_id.starts_with("PS-")
}
```

The check's own loop (`:699-701`) skips a clause outright when `has_suite` returns `false`: "for a
`wire::`-qualified name … looked for in the wire test files rather than in a suite that could never
define it." A family absent from `has_suite` is not partially checked — it is not checked at all,
and check 4 reports nothing wrong, because abstaining and passing look identical in the printed
summary.

`PS` was added to that list only recently. Before it was, a `PS` clause could cite a rule name that
existed nowhere in the tree — a typo, a rule renamed out from under its citation, a rule never
written — and `cargo xtask spec-trace` would report no problem, because `PS` sat outside the set
`has_suite` checked. The exclusion was written for a true reason: the projection suite
(`crates/happenstance-testkit/src/projection.rs`) did not exist yet, and checking `PS` citations
against a suite that could never define them would report every one of them as a false positive,
"noise indistinguishable from a real typo" (`xtask/src/spec_trace.rs:2497-2499`). The suite was
then written, and the exclusion was not removed — it outlived its own justification by two slices
before this workspace noticed.

## What holds the fix

`spec_trace::tests::the_projection_family_is_checked_against_its_suite`
(`xtask/src/spec_trace.rs:3476-3484`) now asserts `has_suite("PS-1")` and `has_suite("PS-37")`
directly, so a future regression — narrowing the prefix list back to exclude `PS` — fails the build
rather than silently reopening the blind spot. The companion test,
`the_replication_family_still_abstains_and_the_rest_do_not` (`:2378-2394`), asserts the reverse for
the one family still legitimately excluded: `has_suite("SY-1")` must stay `false`, because
`happenstance-sync-testkit` does not exist and checking `SY` rule names against suites that do exist
would manufacture exactly the false-positive noise the original `PS` exclusion was written to avoid
(`:2387-2391`). It also pins `CF-1`'s abstention, on the different ground that `CF` clauses are about
the suite rather than checked by it. Widening `has_suite` to admit every family unconditionally is
therefore a build failure today, not a judgement call left to the next editor.

## The transferable shape

An exclusion written for a true reason, kept after the reason expired, and structurally invisible
because the thing it disables is itself a check — nothing fails when the exclusion goes stale,
because the exclusion's job is to prevent failures. `kb-open-question-provisional-falsifiers-001`
records the identical shape one level up, on specification-clause maturity markers rather than a
tool's internal switch: ES-7 and VT-9 each name a falsifier that has already occurred without
falsifying anything, and `spec-trace` "can see that a marker exists and cite it, but has no way to
evaluate whether the condition the marker names has been met." `has_suite` is the same defect wearing
different clothes — a named exemption list, silent when an exemption is discharged.

That is also the property `kb-playbook-ratchet-gate-landing-001`'s ratchet pattern is built to
supply and `has_suite` did not have before the two tests above were added: a ratchet's exemption
list is fatal from day one and shrinks only by an explicit, checked edit, so a discharged exemption
that is never removed fails the build rather than sitting there unnoticed. `has_suite` was, until
`the_projection_family_is_checked_against_its_suite` landed, an exemption list with no such
guarantee — `PS`'s entry became discharged and nothing failed.
