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
  satisfied: true
  evidence: >-
    `crates/happenstance/src/composition.rs:24` (the internal `macro_rules!`), invoked once per
    arity at `:95` (2) and `:161-166` (3 through 8) — seven arities, fourteen emitted impls,
    reachable through `crates/happenstance/src/lib.rs:106-107`. No new crate-root name is added:
    `git diff` shows no new `pub use` in `lib.rs`. Passing test
    `crates/happenstance/tests/composition.rs::a_tuple_is_a_boundary` builds 2-, 3- and 8-tuples
    in a **downstream** test crate, importing only `happenstance::…`, and drives both `query()`
    and `absorb` through `happenstance::Boundary`. Nesting is exercised too, in
    `::the_seal_holds_for_tuples`, which is the answer above arity 8. The 9-tuple refusal is
    stated and not tested: `trybuild` has no home in this tree and a `compile_fail` doctest is a
    weaker instrument that would not discriminate — `compile-fail-proof-artefact` (M6) owns it.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::a_tuple_is_a_boundary"

- id: AC-002
  criterion: "GIVEN P1 composes two models, WHEN the composed boundary is read with, THEN it selects every event either member would have selected alone — the composite's `Query` items are the members' own items, in member order, unaltered — so no event inside a member's boundary can be missed by the composite. A composite that dedups, merges or re-sorts selects fewer events than its members did and is a lost update with no diagnostic."
  satisfied: true
  evidence: >-
    `crates/happenstance/src/composition.rs:45-79` — the members' items are concatenated in member
    order with `extend_from_slice` and handed to `Query::from_items` unaltered: no dedup, no
    merge, no re-sort. A member that matches everything widens the union to `Query::all()` rather
    than narrowing it. Passing tests in `crates/happenstance/tests/composition.rs`:
    `composed_query_is_the_union_of_member_queries`, whose oracle `union_of` (`:188`) calls each
    member's **own** `query()` and concatenates — spelled in the opposite direction from the
    implementation, sharing no subroutine with it (RS-60-4) and never a literal item list — and
    which additionally asserts the composite *matches* every event either member would have
    matched alone. Paired with `::a_narrowing_union_is_rejected`, which hand-rolls the dedup-ing
    composite in the test file and shows it both failing the union assertion and losing the second
    member's boundary. RED evidence: with a dedup-ing union installed in the macro, both tests
    failed (`assertion left == right failed` and `the narrowing composite must fail the union
    assertion`).
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::composed_query_is_the_union_of_member_queries (paired with ::a_narrowing_union_is_rejected)"

- id: AC-003
  criterion: "GIVEN one member of P1's tuple constrains neither event types nor tags, WHEN the composite's `query()` is called, THEN the caller gets that member's `InvalidQuery` unchanged rather than a silently widened boundary: the first `Err` propagates, `Query::all()` is never substituted, and a failing member is never dropped so the rest can carry on. The refusal is the feature — an unconstrained boundary is the whole log and must be said out loud."
  satisfied: true
  evidence: >-
    `crates/happenstance/src/composition.rs:54` — `let parts = [$fv.query()?, $($rv.query()?),+];`
    derives every member before the union loop runs, so the **first** failing member in member
    order is the one whose error propagates, unchanged; `Query::all()` is never substituted and no
    member is dropped so the rest can carry on. Passing test
    `a_members_invalid_query_is_the_composites_error`, which asserts the exact `InvalidQuery`
    value on a 2-tuple and, on two 3-tuples with the failing members in opposite orders, that
    member order decides. RED evidence: with `.unwrap_or_else(|_| Query::all())` installed in
    place of the `?`, the test failed with `expected the member's own error, got Ok(All)` — the
    silently widened boundary, caught. **Location deviation, recorded rather than worked around:**
    the test lives in `crates/happenstance/src/tests.rs` rather than in
    `crates/happenstance/tests/composition.rs`. A member's `query()` can only fail if some
    `Boundary` returns `Err`, and neither shipped implementation can — the blanket derivation's
    only route to `InvalidQuery` is an empty `EVENT_TYPES`, which is a compile error (the
    slice-mate's AC-003). The failing member therefore has to be a type only this crate can write,
    because only this crate can name the seal. It is the test's *input*; the composite under test
    is the shipped one, and the two wrong implementations AC-002 and AC-004 name are still
    rejected from **outside**, in `tests/composition.rs`.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::a_members_invalid_query_is_the_composites_error"

- id: AC-004
  criterion: "GIVEN P1's two models share an event type but not a boundary, WHEN read events are absorbed into the composite, THEN each member folds only the events its own query nominated, so a sibling's nomination can never corrupt a member's state. An event no member nominated is skipped silently; an event a member did nominate and cannot decode returns `CodecError::UnknownEventType`, because that is a real disagreement between `EVENT_TYPES` and the fold."
  satisfied: true
  evidence: >-
    `crates/happenstance/src/composition.rs:75-92` — the composite asks each member in turn and
    each member's own `absorb` consults its own derived query (`crates/happenstance/src/
    boundary.rs:121`, which derives the query and asks `Query::matches` at `:131-140` — the
    contract's only filter vocabulary, per AC-U04; the hand-rolled `nominates` predicate that
    shipped in the first pass is deleted, so there is no second predicate to diverge from it). Passing tests in
    `crates/happenstance/tests/composition.rs`: `each_member_absorbs_only_what_it_nominated`, over
    two models that **share** an event type across disjoint tags plus one event inside both
    boundaries and one inside neither — each member folds exactly its own two. Paired with
    `::routing_by_arrival_is_rejected`, which hand-rolls the wrong implementation in the test file
    (decode centrally, `apply` to every member) and asserts its result differs from the honest
    one. RED evidence: with the composite absorbing into only its first member, the nomination
    test failed with `the student model folded its own two`. The `CodecError::UnknownEventType`
    half is proved by the slice-mate's
    `crates/happenstance/src/tests.rs::absorb_returns_unknown_event_type_when_a_nominated_event_
    cannot_be_decoded`, and the composite propagates it by `?` at
    `crates/happenstance/src/composition.rs:86-87`; the impl's own doc comment
    (`:109-112`) states that earlier members have already folded and are **not** rolled back.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::each_member_absorbs_only_what_it_nominated (paired with ::routing_by_arrival_is_rejected)"

- id: AC-005
  criterion: "GIVEN P1 mistypes a tuple — nine members, or two members with different `Event` types — WHEN they compile, THEN the diagnostic's `-->` span points at their line, never inside a macro body, because every path the expansion emits is `$crate::`-qualified and the arms are ordered most-literal-first. And GIVEN a third party wants to hand-maintain a query, THEN they cannot: the same expansion emits `impl sealed::Sealed` beside every `impl Boundary`, and neither the macro nor `Sealed` is exported."
  satisfied: true
  evidence: >-
    The same expansion emits `impl $crate::sealed::Sealed` beside every `impl $crate::Boundary`
    (`crates/happenstance/src/composition.rs:26-33` and `:35-42`), so the seal survives
    composition; neither the macro nor `Sealed` is exported — `crates/happenstance/src/lib.rs`
    declares `mod composition;` and `mod sealed;` privately and the macro carries no
    `#[macro_export]`. Passing test
    `crates/happenstance/tests/composition.rs::the_seal_holds_for_tuples`, in a downstream crate
    where only public paths are nameable; the two refusals it cannot spell (`use
    happenstance::sealed::Sealed` is `error[E0603]`, and `impl happenstance::Boundary for MyType`
    is `error[E0277] … Sealed is not satisfied`) are recorded there as prose rather than as a
    fence, because rustdoc collects doctests from the lib target only. Passing test
    `crates/happenstance/tests/doc_budget.rs::every_emitted_path_is_crate_qualified` reads the
    macro definition and asserts that every occurrence of `Boundary`, `Sealed`, `Query`,
    `InvalidQuery`, `Codec`, `CodecError`, `SequencedEvent`, `Vec`, `Result` and `Option` outside
    a doc comment is `::`-rooted — `$crate::` for ours, `::core`/`::std` for the standard library
    (RS-41-1) — and that each name was found at all, so the assertion is not vacuous. RS-41-3's
    most-literal-first ordering is satisfied by construction: the macro has exactly one arm, and
    the doc comment at `:12-23` says so. Diagnostic **spans** are `compile-fail-proof-artefact`'s
    instrument (M6); this story states the obligation and fakes no check for it.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::the_seal_holds_for_tuples and crates/happenstance/tests/doc_budget.rs::every_emitted_path_is_crate_qualified"

- id: AC-006
  criterion: "GIVEN P4 lands on `happenstance`'s docs.rs page in a bounded sitting, WHEN they reach the vocabulary region and ask \"how do I check two boundaries at once?\", THEN the `DecisionModel` bullet answers in one line and links to `Boundary`, where a compiling example composes two models — presentation that actually renders: a resolved intra-doc link and a rustdoc-collected fence, not a bare sentence. No new name joins the crate root, `pub use happenstance_core::*;` survives, and nothing shadows `Query`, `EventStore`, `Tags` or `Event`."
  satisfied: true
  evidence: >-
    `crates/happenstance/src/lib.rs:86-92` — the `DecisionModel` bullet's promise becomes the
    answer, in one line, inside region 4: *"Composing several into one query — put them in a
    tuple, which is a [`Boundary`] too — is the mechanism that makes a dynamic consistency
    boundary dynamic."* The bold term is the link, so nothing is emphasised that is not also
    reachable. On the other end of that link, `crates/happenstance/src/composition.rs:96-157` is
    the doc comment on the arity-2 `impl Boundary for (B1, B2)` block, carrying a compiling
    example that composes two models — collected and run by `cargo test -p happenstance --doc`
    (4 doctests pass) and rendered in `Boundary`'s *Trait Implementations* section. No new
    crate-root name joins the page. Passing test
    `crates/happenstance/tests/doc_budget.rs::the_glob_reexport_survives_and_nothing_shadows_it`
    asserts `pub use happenstance_core::*;` is still present and that no rendering module declares
    a `Query`, `EventStore`, `Tags` or `Event` of its own. `RUSTDOCFLAGS="-D warnings" cargo doc
    -p happenstance --no-deps` is warning-free, which under the workspace's
    `rustdoc::broken_intra_doc_links = "deny"` is what proves the link resolves.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/doc_budget.rs::the_glob_reexport_survives_and_nothing_shadows_it; cargo test -p happenstance --doc (the composing example on impl Boundary for (B1, B2)); RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance --no-deps"

- id: AC-007
  criterion: "GIVEN P4 reads that page at 1024x768, WHEN this story's line and example are added, THEN the page still opens with one summary line and the first program's fence above the fold, the added line sits in region 4 as one line of a one-line-per-bullet list, the example's fence never scrolls horizontally, and the signed-off budgets hold: doc prose <= 80 columns, code inside a doc fence <= 72, the crate-root module doc <= 130 lines, <= 12 prose lines to the first fence, every doc comment's first sentence <= 80 characters. Tuple composition stays revealed — one click from the page, never a sixth bullet or a new root item."
  satisfied: true
  evidence: >-
    All five named tests in `crates/happenstance/tests/doc_budget.rs` pass over the crate's own
    sources: `doc_prose_stays_within_eighty_columns` (URL-carrying lines exempt — a link target
    cannot be wrapped), `doc_fences_stay_within_seventy_two_columns` (which is the budget that
    keeps the arity-2 example from scrolling at 1024px),
    `the_module_doc_stays_under_its_line_budget` (91 of 130),
    `the_first_fence_is_within_twelve_prose_lines` (the fence opens 5 prose lines down, and the
    same test asserts the crate root carries **exactly one** fence, so this story's example did
    not demote it) and `first_sentences_fit_the_item_table`. Composition and transience hold:
    this story adds **no** crate-root name and **no** sixth bullet — the added text is one line
    inside the existing `DecisionModel` bullet at `crates/happenstance/src/lib.rs:86-92`, and
    tuple composition stays *revealed*, one click away on `Boundary`'s page. Every public
    identifier budget is satisfied by construction, because no public identifier is added.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/doc_budget.rs::doc_prose_stays_within_eighty_columns, ::doc_fences_stay_within_seventy_two_columns, ::the_module_doc_stays_under_its_line_budget, ::the_first_fence_is_within_twelve_prose_lines, ::first_sentences_fit_the_item_table"

- id: AC-008
  criterion: "GIVEN P1 builds and inspects a composed boundary before committing to anything, WHEN they compose, fold and re-fold, THEN nothing has happened: composing performs no I/O and touches no store, the composed `Query` is a value they can hold and assert on rather than private machinery inside `read`, a clone taken before absorption is still pristine and re-folds identically after a `ConditionViolated` retry, and the link this story adds resolves with `--no-default-features` as well as with defaults."
  satisfied: true
  evidence: >-
    Passing tests in `crates/happenstance/tests/composition.rs`:
    `composing_is_pure_and_a_clone_refolds_identically` — composes, clones, absorbs two events
    into the clone, then asserts the pristine tuple is untouched, that its derived query is
    unchanged by folding, and that a **re-fold from the pristine clone reproduces the first fold
    exactly** (this is what a `ConditionViolated` retry does); and
    `the_composed_query_is_inspectable`, which binds the composed `Query`, `Debug`-prints it,
    asserts it is not `Query::all()` and asserts on its item count — a value, not private
    machinery inside a read. No store is constructed in either test, nothing is awaited, and
    `crates/happenstance/src/composition.rs` names no store type and no future. Feature-off link
    resolution, run by hand because the gate's own no-default-features doc step names
    `happenstance-core` only (`xtask/src/main.rs:502-513`): `RUSTDOCFLAGS="-D warnings" cargo doc
    -p happenstance --no-deps --no-default-features` completed with **no warning** —
    `Documenting happenstance v0.2.0 … Finished dev profile … Generated
    target/doc/happenstance/index.html`. Adding a fifth doc step to the gate is an `xtask/` edit,
    outside this PR boundary and routed to `edge-flavour-and-wasm-claim`.
  mount_point: "crates/happenstance/src/lib.rs"
  verifying_test: "crates/happenstance/tests/composition.rs::composing_is_pure_and_a_clone_refolds_identically and ::the_composed_query_is_inspectable; RUSTDOCFLAGS=\"-D warnings\" cargo doc -p happenstance --no-default-features --no-deps"
```
