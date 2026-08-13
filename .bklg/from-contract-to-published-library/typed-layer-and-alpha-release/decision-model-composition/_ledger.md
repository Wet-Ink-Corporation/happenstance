---
item: HS-S0021
stage: implement
created: 2026-08-12T13:46:17.300Z
updated: 2026-08-12T13:46:17.300Z
---

# Acceptance ledger — Consistency boundaries compose at compile time

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN P1 has two decision models over one event vocabulary — say seat capacity and one student's own history — WHEN they need both boundaries checked by the same append, THEN they write the tuple `(seats, history)` and hand it where a single model went: no macro invocation, no builder, no extra import, no new crate-root name. Arities 2 through 8 all compose; a 9-tuple is an ordinary \"trait not implemented\" error on the caller's own line."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::a_tuple_is_a_boundary"

- id: AC-002
  criterion: "GIVEN P1 composes two models, WHEN the composed boundary is read with, THEN it selects every event either member would have selected alone — the composite's `Query` items are the members' own items, in member order, unaltered — so no event inside a member's boundary can be missed by the composite. A composite that dedups, merges or re-sorts selects fewer events than its members did and is a lost update with no diagnostic."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::composed_query_is_the_union_of_member_queries (paired with ::a_narrowing_union_is_rejected)"

- id: AC-003
  criterion: "GIVEN one member of P1's tuple constrains neither event types nor tags, WHEN the composite's `query()` is called, THEN the caller gets that member's `InvalidQuery` unchanged rather than a silently widened boundary: the first `Err` propagates, `Query::all()` is never substituted, and a failing member is never dropped so the rest can carry on. The refusal is the feature — an unconstrained boundary is the whole log and must be said out loud."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::a_members_invalid_query_is_the_composites_error"

- id: AC-004
  criterion: "GIVEN P1's two models share an event type but not a boundary, WHEN read events are absorbed into the composite, THEN each member folds only the events its own query nominated, so a sibling's nomination can never corrupt a member's state. An event no member nominated is skipped silently; an event a member did nominate and cannot decode returns `CodecError::UnknownEventType`, because that is a real disagreement between `EVENT_TYPES` and the fold."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::each_member_absorbs_only_what_it_nominated (paired with ::routing_by_arrival_is_rejected)"

- id: AC-005
  criterion: "GIVEN P1 mistypes a tuple — nine members, or two members with different `Event` types — WHEN they compile, THEN the diagnostic's `-->` span points at their line, never inside a macro body, because every path the expansion emits is `$crate::`-qualified and the arms are ordered most-literal-first. And GIVEN a third party wants to hand-maintain a query, THEN they cannot: the same expansion emits `impl sealed::Sealed` beside every `impl Boundary`, and neither the macro nor `Sealed` is exported."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::the_seal_holds_for_tuples and crates/happenstance/tests/doc_budget.rs::every_emitted_path_is_crate_qualified"

- id: AC-006
  criterion: "GIVEN P4 lands on `happenstance`'s docs.rs page in a bounded sitting, WHEN they reach the vocabulary region and ask \"how do I check two boundaries at once?\", THEN the `DecisionModel` bullet answers in one line and links to `Boundary`, where a compiling example composes two models — presentation that actually renders: a resolved intra-doc link and a rustdoc-collected fence, not a bare sentence. No new name joins the crate root, `pub use happenstance_core::*;` survives, and nothing shadows `Query`, `EventStore`, `Tags` or `Event`."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/doc_budget.rs::the_glob_reexport_survives_and_nothing_shadows_it; cargo test -p happenstance --doc (the composing example on impl Boundary for (B1, B2)); RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance --no-deps"

- id: AC-007
  criterion: "GIVEN P4 reads that page at 1024x768, WHEN this story's line and example are added, THEN the page still opens with one summary line and the first program's fence above the fold, the added line sits in region 4 as one line of a one-line-per-bullet list, the example's fence never scrolls horizontally, and the signed-off budgets hold: doc prose <= 80 columns, code inside a doc fence <= 72, the crate-root module doc <= 130 lines, <= 12 prose lines to the first fence, every doc comment's first sentence <= 80 characters. Tuple composition stays revealed — one click from the page, never a sixth bullet or a new root item."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/doc_budget.rs::doc_prose_stays_within_eighty_columns, ::doc_fences_stay_within_seventy_two_columns, ::the_module_doc_stays_under_its_line_budget, ::the_first_fence_is_within_twelve_prose_lines, ::first_sentences_fit_the_item_table"

- id: AC-008
  criterion: "GIVEN P1 builds and inspects a composed boundary before committing to anything, WHEN they compose, fold and re-fold, THEN nothing has happened: composing performs no I/O and touches no store, the composed `Query` is a value they can hold and assert on rather than private machinery inside `read`, a clone taken before absorption is still pristine and re-folds identically after a `ConditionViolated` retry, and the link this story adds resolves with `--no-default-features` as well as with defaults."
  satisfied: false
  evidence: ""
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::composing_is_pure_and_a_clone_refolds_identically and ::the_composed_query_is_inspectable; RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance --no-default-features --no-deps"
```
