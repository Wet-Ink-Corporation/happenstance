---
id: kb-open-question-model-only-kind-memberless-001
title: Kind::ModelOnlyMutant has no members; keep it dormant, withdraw it, or make the emptiness a standing check
kind: open_question
status: accepted
authority_tier: note
summary: >-
  Kind::ModelOnlyMutant records a store with a defect no event-store-family rule can see, and it
  carried exactly one member — UnparenthesisedToPredicateStore — until read_to_composes_with_multi_item_query
  landed and turned that row into an ordinary Kind::Mutant, leaving the kind memberless.
  the_model_only_bar_rejects_a_store_with_no_defect's !MODEL_ONLY_WITNESSES.is_empty() assertion,
  right while the kind had a member, became wrong once it did not, so the landing lane replaced it
  with an equivalence: the witness table is empty exactly when the kind has no members, checked in
  both directions. That edit is the minimum needed to keep the tree honest under any answer and is
  not itself the decision. Three options remain open: keep the variant dormant with documentation
  saying why (recommended at medium confidence); withdraw roughly 200 lines of machinery that
  currently drives nothing; or keep it and add a test that turns filing a new member into a
  deliberate, justified act. The recommendation is held to medium confidence on its own terms and
  is explicitly reviewed at phase 12, because the shape this kind exists for — a defect needing a
  query and a read option together, where the option's own rules all issue Query::all() — has now
  recurred twice (CF-12's `from` at phase 3, this store's `to` today), which argues against
  withdrawing a vocabulary entry for a shape that keeps recurring.
depends_on: []
related:
  - kb-decision-0010
  - kb-concept-mutation-kind-tethered-bar-001
  - kb-open-question-cf-17-cf-14-markers-001
  - kb-open-question-read-to-backwards-limit-composition-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/model-only-kind-has-no-members.md
last_reviewed: 2026-09-07
---

# Kind::ModelOnlyMutant has no members; keep it dormant, withdraw it, or make the emptiness a standing check

## What is true today

`Kind::ModelOnlyMutant` (`crates/happenstance-testkit/tests/mutation_coverage/`) records a store
with a defect that no rule of the event-store family can see, which the generative *model* family
has to catch instead. It carried exactly one member since it landed: `UnparenthesisedToPredicateStore`
(`WHERE a OR b AND position <= ?`, the upper bound conjoined with the last disjunct alone), and
its own documentation said in terms what would end its filing: *"If someone writes the missing
rule, this row becomes an ordinary `Kind::Mutant` with one entry in `fails` and the meta-tests say
so."* `read_to_composes_with_multi_item_query` landed that rule. The row is now an ordinary
`Kind::Mutant`, and the kind has no members.

`the_model_only_bar_rejects_a_store_with_no_defect` opened with
`assert!(!MODEL_ONLY_WITNESSES.is_empty(), …)` — correct while the kind had a member, and wrong
the instant it did not, since a control over an empty iteration controls nothing and a bare
non-empty assertion demands a witness for a store that must not have one. The landing lane
replaced it with an equivalence checked in both directions: the witness table is empty exactly
when the kind has no members. That keeps a row without a member (an orphan scenario) and a member
without a row (the hazard `HidingPlaceStore` exploited) both detectable, and it is deliberately
the *minimum* fix — reversible under any of the three options below, and not itself a decision
about which one is right.

## What is not decided

**Option A — keep the variant, dormant.** The `Kind::ModelOnlyMutant` variant, `Witness`,
`MODEL_ONLY_WITNESSES`, `HidingPlaceStore`, and both meta-tests stay with no rows to drive; the
documentation says so and why. Costs a reader a vocabulary entry with no live example and costs
nothing else — none of it is public API, and `tests/mutation_coverage.rs` is a test target of a
crate that publishes no test targets. Buys the next defect of this shape a place to land with the
three obligations already written and already checked.

**Option B — withdraw the variant.** Delete all of it, roughly 200 lines, none of it currently
driving anything. Buys a `Kind` that is only ever "a store fails rules or it does not," with no
row whose content is "no rule rejects it." Costs the next author who meets the shape a full
re-derivation of the three obligations, and the July `HidingPlaceStore` review — the one that
found a kind borrowing another family's bar can have that bar compiled out under
`--no-default-features` — would have to happen again.

**Option C — keep it, and assert the emptiness as the standing measurement.** Option A plus a
named test that turns green-to-red the moment a store is filed here, forcing the filing to be
argued rather than quiet. Buys a kind that cannot silently accumulate members; costs an inversion
of what the machinery means (something to be argued *out of* rather than a category) and a check
built to be deleted the moment a legitimate member arrives.

## Recommendation and its own discount

Option A, at medium confidence, revisited at phase 12 rather than settled now. The argument is
asymmetric cost: keeping a memberless variant costs a paragraph of documentation; withdrawing
loses a non-obvious design (the tethered `Witness`/`Defect::select` repair) if the shape recurs,
and it has recurred once already in nine phases. The strongest objection on file is that
`Kind::StatedOnlyDefect` was withdrawn on this exact memberless-versus-dormant question one day
earlier and the tree is better for it — but that withdrawal was for a bar shown **unsound** (two
string literals satisfied it), not for being empty; `Kind::ModelOnlyMutant`'s bar was attacked in
the same review and **survived** because of the `Witness` tether. A bar that has been attacked and
held is a different asset from one shown unsatisfiable, which is why this brief does not read the
sibling withdrawal as controlling here.

## What forces it

Phase 12's testkit public-surface pass, named as the review point regardless of outcome. If a
third store of this shape has not appeared by then, "the shape recurs" will have been a prediction
that did not pay, and Option B becomes the stronger answer on its own terms.
