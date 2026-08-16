---
item: "HS-S0021"
stage: report
created: "2026-08-15"
updated: "2026-08-15"
---

# Report — Consistency boundaries compose at compile time

## Findings Ledger

**Outcome: eight of eight ACs satisfied.** One test moved file, one gate step is
red for a reason this slice did not create, and both are stated below rather than
smoothed over.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| A tuple is a boundary, arities 2 through 8, with no new crate-root name | `crates/happenstance/src/composition.rs:24` (one `macro_rules!`, one arm), invoked at `:95` and `:161-166`; `tests/composition.rs::a_tuple_is_a_boundary` drives 2-, 3- and 8-tuples from a downstream crate | above arity 8, nest — proved by `::the_seal_holds_for_tuples` |
| The composed query is the union, and the narrowing composite is rejected by a named test | oracle `union_of` at `tests/composition.rs:188` calls each member's own `query()`; `::a_narrowing_union_is_rejected` hand-rolls the dedup-ing composite and shows it losing the second member's boundary | none. A later "tidy" with a `HashSet` fails a named test rather than passing quietly |
| Each member folds only what its own query nominated | `composition.rs:75-92` asks each member; `boundary.rs:97` is where nomination is checked. `::each_member_absorbs_only_what_it_nominated` uses two models that **share** an event type across disjoint tags — the shape a disjoint-tag test cannot discriminate | none |
| The first failing member's `InvalidQuery` propagates unchanged | `composition.rs:54` derives every member before the union loop; RED cycle showed `Ok(All)` when the `?` was replaced by a widening fallback | none |
| **AC-003's test lives in `src/tests.rs`, not `tests/composition.rs`** | a member's `query()` can only fail if some `Boundary` returns `Err`, and neither shipped impl can: the blanket derivation's only route to `InvalidQuery` is an empty `EVENT_TYPES`, which is a compile error. The failing member has to be a type that can name the seal, and only this crate can | worth one sentence in `_design.md`'s *residual* section at closeout: D-1 does not merely make the `Result` unreachable, it makes it **untestable from outside the crate** |
| The seal survives composition | the macro emits `impl $crate::sealed::Sealed` beside every `impl $crate::Boundary` (`composition.rs:26-33`, `:35-42`); no `#[macro_export]`, `mod composition` and `mod sealed` both private | none |
| Every emitted item path is rooted | `tests/doc_budget.rs::every_emitted_path_is_crate_qualified` over ten names, and it asserts each was found before asserting each is rooted, so it cannot pass vacuously | none |
| The crate root still carries exactly **one** fence | `tests/doc_budget.rs::the_first_fence_is_within_twelve_prose_lines` asserts both the distance (5 of 12) and the count — this story's example lives on `Boundary`'s item page, where the transience policy already put tuple composition | none |
| **`cargo xtask ci --fast` is red on a step this slice does not touch** | *the Rust constitution is internally consistent* — three citations in `standards/rust/60-what-a-test-must-prove.md` (`:68`, `:69`, `:163`) point at lines `crates/happenstance-testkit/tests/mutation_coverage/harness.rs` no longer has. `git diff d4aedfc..HEAD --name-only` shows this slice touched neither file; `harness.rs` was last changed by `90421d0` | `misbehaving-testkit-stores`' neighbourhood. Repairing it here would put the change outside both stories' declared PR boundaries |

## Acceptance

| AC | Status | Verified by |
| -- | ------ | ----------- |
| AC-001 | satisfied | `composition.rs::a_tuple_is_a_boundary` (2-, 3-, 8-tuples through the public root); the 9-tuple refusal is stated, not faked |
| AC-002 | satisfied | `::composed_query_is_the_union_of_member_queries` with an oracle spelled in the opposite direction (RS-60-4), paired with `::a_narrowing_union_is_rejected` |
| AC-003 | satisfied | `src/tests.rs::a_members_invalid_query_is_the_composites_error` — exact error value, and member order proved on two 3-tuples. Location deviation recorded above and in the ledger evidence |
| AC-004 | satisfied | `::each_member_absorbs_only_what_it_nominated` over two models sharing an event type, paired with `::routing_by_arrival_is_rejected` |
| AC-005 | satisfied | `::the_seal_holds_for_tuples` plus `doc_budget.rs::every_emitted_path_is_crate_qualified`; one arm, so RS-41-3 is satisfied by construction. Diagnostic spans stay M6's instrument |
| AC-006 | satisfied | the one-line vocabulary answer at `lib.rs:97-99`, the composing doctest on the arity-2 impl (doctest count 3 → 4), `::the_glob_reexport_survives_and_nothing_shadows_it`, and a warning-free `RUSTDOCFLAGS="-D warnings" cargo doc` |
| AC-007 | satisfied | all five `doc_budget.rs` budget tests, including *exactly one fence on the crate root*. No new public identifier, so that budget holds by construction |
| AC-008 | satisfied | `::composing_is_pure_and_a_clone_refolds_identically`, `::the_composed_query_is_inspectable`, and the cited feature-off `cargo doc` run |

**Deferred, and named rather than faked:** the 9-tuple compile error and the
`-->` span obligation (`compile-fail-proof-artefact`, M6); a fifth
`--no-default-features` doc step for `happenstance` in `xtask`
(`edge-flavour-and-wasm-claim`, M7); rewriting `examples/course-subscriptions`
onto the typed layer (`worked-example-on-typed-layer`, M6) — this story exists so
that rewrite has something to be written onto, and deliberately does not do it.

## Knowledge Harvest

* **Coherence permits a blanket impl over a local trait alongside tuple impls.**
  `impl<M: DecisionModel> Boundary for M` and `impl … Boundary for (B1, B2)` do
  not overlap, because the orphan rule forbids a downstream
  `impl DecisionModel for (A, B)`. Probed with a standalone `rustc` file before
  the macro was written — the opposite answer would have forced a different
  composition shape on the whole slice. This belongs beside
  `standards/rust/13-sealing-and-exhaustiveness.md`.
* **`///` can be passed into a `macro_rules!` invocation and matched as
  `$(#[$m:meta])*`.** It is how one arity out of seven carries prose and a
  doctest without a second arm duplicating the body — directly serving RS-70-5's
  *name the alternative once, where the reader is*. A candidate rule for
  `standards/rust/41-declarative-macros.md`.
* **Tuple field access cannot be generated from a type-parameter repetition.**
  The expansion destructures instead, which means the invocation must supply a
  value identifier per member (`B1 b1`). Reusing the type parameter as the binding
  name compiles and trips `non_snake_case` on every emitted impl.
* **D-1 is stronger than recorded.** `happenstance-core`'s lack of an infallible
  `QueryItem` constructor does not only leave an unreachable `Result` — combined
  with the seal, it makes that `Result`'s error arm **untestable from outside the
  crate**, because no downstream type can be a failing `Boundary`. Worth adding to
  the defect entry AC-012 collects.
