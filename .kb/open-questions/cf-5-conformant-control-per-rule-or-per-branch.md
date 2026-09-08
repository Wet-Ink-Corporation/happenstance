---
id: kb-open-question-cf-5-per-rule-or-branch-001
title: Whether CF-5's conformant-control obligation runs per rule or per branch
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0010 mints CF-5 by requiring a conformant variant that passes every rule, as the positive
  control against a mutant-only portfolio proving the rule set is broken rather than working. It
  does not say whether that obligation is discharged once per rule or once per branch inside a
  rule with more than one assertion arm. ES-22's dropped_append_future_leaves_no_partial_batch is
  the first place the difference is visible: its arm 2 (the byte-identical before/after snapshot)
  is reached and passed by two registered stores, but both are filed as mutants for other rules
  rather than as a Kind::ConformantVariant for this one, so nothing in the portfolio is a control
  built to exercise arm 2 specifically. CF-1 read strictly asks for the plausible wrong
  implementation a control rejects, and a conformant control rejects nothing by construction --
  which is CF-5's own justification restated, and does not distinguish the per-rule reading from
  the per-branch one. Unresolved here or anywhere; bigger than ES-22 because it recurs at every
  multi-arm rule the suite already has or will add.
depends_on:
  - kb-decision-0010
related:
  - kb-governance-what-may-refute-a-finding-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/es-22-arm-two-is-reached-the-finding-is-wrong.md
  - .kb/_intake/f2-5-holds-the-release-for-phase-10.md
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-testkit/tests/mutation_coverage.rs
last_reviewed: 2026-09-07
---

# Whether CF-5's conformant-control obligation runs per rule or per branch

## What is true today

ADR-0010 (`kb-decision-0010`) requires a positive control alongside the mutant portfolio: a `Kind::ConformantVariant` store that passes every rule, without which a harness that fails unconditionally would satisfy the suite's other two meta-tests (`every_rule_has_a_mutant`, `mutants_fail_exactly_their_declared_rules`) by accident. That obligation is stated and enforced at the granularity of a **rule** — one entry in `for_each_event_store_rule!` — and every existing rule in the suite has at least one conformant store passing it.

`dropped_append_future_leaves_no_partial_batch` (ES-22) is the first rule where a single-granularity reading stops being obviously sufficient, because the rule itself has two arms with genuinely different content: arm 1 asserts a store that lands one row of two fails at `landed == 1`, and arm 2 asserts byte-identical before/after snapshots when `landed == 0`. A measurement run against the mutation-coverage binary (`es-22-arm-two-is-reached-the-finding-is-wrong.md`) found that arm 2 is reached and passed by exactly two registered stores — `PreCommitPositionStore` and `AwaitAcrossBorrowStore` — on every run, in both feature configurations. Both are legal on ES-22's own axis: each is documented as "indistinguishable from a correct one" when driven to completion, and neither has a defect this rule is meant to catch. But both are registered as `Kind::Mutant` for *other* rules (position visibility and a borrow held across an await, respectively), not as a `Kind::ConformantVariant` for this one. So arm 2 is functionally controlled — two legal stores pass it — while carrying no store whose registry entry says that is what it is for.

Whether that gap matters turns on a question CF-5's text does not answer. Read per rule, ES-22 already has conformant coverage: `GappedPositionStore` and `PagedStreamStore` both complete on the first poll and pass the whole rule, satisfying `conformant_variants_pass_everything` for ES-22 as a unit. Read per branch, arm 2 has no purpose-built control, only two mutants that happen to be legal on this axis — which is coverage in the CF-1 sense (a rule is under-specified only if no wrong store fails it) but not in the CF-5 sense (a rule is over-specified only if a legal store fails it, and detecting that needs a control aimed at the branch, not an incidental pass by a mutant aimed elsewhere).

## What is not decided

Whether adding a `Kind::ConformantVariant` for every distinct assertion arm inside a multi-arm rule is what CF-5 already requires, or whether CF-5's per-rule reading is the intended scope and a multi-arm rule's branches are covered as a side effect of covering the rule. The argument against minting one is CF-1 read strictly: name the plausible wrong implementation such a control rejects, and the honest answer for a conformant control is *none* — it rejects nothing by construction, which is exactly why CF-5 exists as a separate obligation from CF-1 rather than an instance of it, and that same reasoning applies whether the granularity is a rule or a branch.

## What forces it

Any future rule the suite adds with more than one assertion arm inherits the same ambiguity, and ES-22 is only the first place it became visible rather than a special case. It is a precedent question, not a scoped fix.

## Ordered sub-questions

1. Does CF-5's text, read against ADR-0010's own reasoning for requiring the control (proving the rule set catches something rather than fails everything), already imply per-branch coverage, or was the obligation only ever conceived at the rule granularity?
2. If per-branch coverage is adopted, is a purpose-built ~40-line store delegating to `crate::correct` (as `es-22-arm-two-is-reached-the-finding-is-wrong.md` sketches) the right shape, or should an existing mutant that happens to be legal on a branch be *re-registered* with a `Kind::ConformantVariant` entry alongside its mutant one?
3. Does adopting a per-branch reading retroactively obligate an audit of every existing multi-arm rule in the suite, or only bind rules added from the day this is decided?
