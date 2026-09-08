---
id: kb-reference-model-family-case-cliff-001
title: MODEL_COVERAGE's table is only true at 192 cases against a default of 256
kind: reference
status: accepted
authority_tier: note
summary: >-
  The per-store Rejected/Agreed table `crates/happenstance-testkit/tests/mutation_coverage.rs`
  publishes as fact about the model family is a fact only at PROPTEST_CASES >= 192, a cliff
  between 176 and 192 measured both before the `to` generator change (11f6f47) and at tip
  (a835e7c), leaving a 1.33x margin on the default of 256. The `to` change moved which store
  sits on the cliff, not the cliff itself.
depends_on: []
related:
  - kb-open-question-model-family-rule-no-clause-001
  - kb-decision-0010
  - kb-open-question-model-family-rule-no-clause-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/model-family-case-floor.md
last_reviewed: 2026-09-07
---

# MODEL_COVERAGE's table is only true at 192 cases against a default of 256

## What this is a pointer to

`crates/happenstance-testkit/tests/mutation_coverage.rs` carries `MODEL_COVERAGE`, an 88-row
table claiming, per mutant store, whether `ops_agree_with_the_model` rejects it — every store
rather than only the caught ones, so the table answers *what the rule is blind to* and goes red
when the answer changes. `ops_agree_with_the_model` (`crates/happenstance-testkit/src/model.rs`)
takes its `proptest` case count from `Config::default()`, which reads `PROPTEST_CASES` from the
environment. This atom is the measurement of how much of that count the table's truth actually
depends on. It does not decide anything; see `kb-open-question-model-family-rule-no-clause-001` for
the clause-ownership question it sits beside.

## The measurement

`cargo test -p happenstance-testkit --all-features --test mutation_coverage the_model_rule`, run
in this working tree with `PROPTEST_CASES` set explicitly, both columns run rather than
transcribed:

| `PROPTEST_CASES` | at `11f6f47` (before the `to` change) | at `a835e7c` (tip) |
|---|---|---|
| 64 | — | FAILED — `ForwardPagingBudgetStore` |
| 128 | — | FAILED — `PayloadDedupStore` |
| 160 | FAILED — `LimitPerItemStore` | FAILED — `PayloadDedupStore` |
| 176 | FAILED — `LimitPerItemStore` | FAILED — `PayloadDedupStore` |
| 192 | ok | ok |
| 256 (default) | — | ok |
| 512 | — | ok |

The failure is always the same shape: `MODEL_COVERAGE` claims `Rejected` for a store and the
model answers `Agreed`, because the case count never drew the sequence that would have caught
it. **The cliff sits between 176 and 192 cases in both columns**; the `to` generator change did
not move the cliff, it moved which store stands on it (`LimitPerItemStore` before, `PayloadDedupStore`
at tip).

## Two consequences the pass/fail table cannot see

`any_to_anchor`'s 4-in-5 skew toward `Anchor::Unset` bought `LimitPerItemStore` coverage at a
measured cost nobody had reported: `PayloadDedupStore`'s margin fell from 64 cases to 192 — a 4x
margin under the default collapsing to 1.33x. And the model-only mutant
`UnparenthesisedToPredicateStore` needs 128 of the 256 available cases, half the budget, where
`from`, `backwards` and `limit` defects are typically caught inside the first few dozen — the
arithmetic cost of sampling one option in five at one fifth the rate.

## Why 256 rather than another number

256 is the shipped default and sits 1.33x above the measured cliff. 512 cases reject the same 43
rows as 256 in the per-store depth sweep this atom's source brief cites, so the extra budget is
measured to buy nothing at double the runtime. 192 is the cliff itself and a number sitting on
its own cliff has no margin.

## What this does not establish

Whether `MODEL_COVERAGE` should record a per-store depth (cases to first rejection) rather than
pass/fail — that would have made the `to` change's 4x-to-1.33x loss visible in the diff that
caused it, but a depth number moves on every generator change and needs a tolerance with an
owner. Also unmeasured: whether `any_to_anchor`'s skew is the right one, and what
`ops_agree_with_the_model`'s real runtime is against a store backed by actual I/O rather than a
`Vec` behind an `Rc` — the number that would justify a floor lower than 256.

## Conditions

Measured in this working tree; conditions not otherwise pinned by the source brief beyond the two
named commits, `11f6f47` and `a835e7c`.
