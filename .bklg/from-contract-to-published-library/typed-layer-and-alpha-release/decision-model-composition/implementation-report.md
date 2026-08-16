---
item: "HS-S0021"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — Consistency boundaries compose at compile time

**All eight ACs are satisfied.** The public delta is exactly what the design
licensed: **no new name**. Seven `impl Boundary` blocks, seven matching
`impl sealed::Sealed` blocks, one internal `macro_rules!`, and one line inside an
existing crate-root bullet.

The slice-mate landed `Boundary` sealed and blanket-implemented; a caller could
express one consistency boundary. After this PR they write `(seats, history)` and
have one — with the composed query the **union** of the members' own, and each
member folding only what its own query nominated.

## TDD Evidence

The two wrong implementations this spec names by name were **installed in the real
macro** for the RED step, run, and then replaced. A third — swallowing a member's
`Err` and widening to `Query::all()` — was installed for one further RED cycle
because AC-003's propagation is not discriminated by either of the first two.
Nothing was weakened afterwards; the tests are the ones written before the fix.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `composition.rs::a_tuple_is_a_boundary` | RED: `assertion left == right failed` — the narrowing union collapsed the 2-, 3- and 8-tuples' item counts. GREEN: 2, 3 and 8 items, and an eight-tuple absorbs |
| AC-002 | `composition.rs::composed_query_is_the_union_of_member_queries` | RED: `assertion left == right failed` against the oracle built from each member's own `query()`. GREEN: the composite's items **are** the members' items, in member order, unaltered |
| AC-002 | `composition.rs::a_narrowing_union_is_rejected` | RED: `the narrowing composite must fail the union assertion` — with the dedup in production, the hand-rolled narrowing composite was indistinguishable from it. GREEN: it differs, and the test shows exactly what it loses (the second member's boundary stops matching) |
| AC-003 | `src/tests.rs::a_members_invalid_query_is_the_composites_error` | RED (third cycle, with `unwrap_or_else(\|_\| Query::all())` in the macro): `expected the member's own error, got Ok(All)`. GREEN: the member's `InvalidQuery` unchanged, and on two 3-tuples the *first* failing member in member order decides |
| AC-004 | `composition.rs::each_member_absorbs_only_what_it_nominated` | RED: `the student model folded its own two` — with the composite absorbing into its first member only. GREEN: two models sharing an event type across disjoint tags each fold exactly their own two, out of four events |
| AC-004 | `composition.rs::routing_by_arrival_is_rejected` | RED: `assertion left == right failed` — the honest composite and the hand-rolled route-by-arrival one agreed, because the honest one was broken. GREEN: `(1, 1)` by nomination against `(2, 2)` by arrival |
| AC-005 | `composition.rs::the_seal_holds_for_tuples` | RED: `assertion left == right failed` on the nested tuple's item count. GREEN: a nested `((B1, B2), B3)` derives three items — the answer above arity 8 |
| AC-005 | `doc_budget.rs::every_emitted_path_is_crate_qualified` | green from the first run, and non-vacuous by construction: it asserts each of the ten names was **found** in the macro body before asserting each occurrence is `::`-rooted |
| AC-006 | `doc_budget.rs::the_glob_reexport_survives_and_nothing_shadows_it`, `cargo test -p happenstance --doc` | green from the first run for the glob; the composing example is new and went from *absent* to collected — doctest count went 3 → 4 |
| AC-007 | `doc_budget.rs`'s five budget tests | RED on authoring: `composition.rs:…: a fence line is 74 columns, over 72` and two first-sentence overruns, each fixed by rewriting the line rather than by moving the budget. GREEN, including `the crate root has exactly one fence` |
| AC-008 | `composition.rs::composing_is_pure_and_a_clone_refolds_identically` | passed in RED as well as GREEN, and honestly so: both folds were equally wrong, so it discriminates purity, not correctness — which is what it is for |
| AC-008 | `composition.rs::the_composed_query_is_inspectable` | RED: `assertion left == right failed` on the item count under the narrowing union. GREEN: two items, `Debug`-printable, not `Query::all()` |

## Commits

One checkpoint commit, on `initiative/from-contract-to-published-library`,
identified by its trailer rather than by a literal SHA — a commit's SHA cannot be
written inside a file that commit contains. `redkiln record-links <item> --sha
<sha>` puts it on the item, which is the queryable place.

| Commit | Subject |
| ------ | ------- |
| `git log --grep "Story: typed-layer-and-alpha-release/decision-model-composition"` | feat(typed-layer-and-alpha-release): Decision model composition |

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance/src/composition.rs` | New. One `macro_rules! impl_boundary_for_tuple` with **one arm**, emitting `impl sealed::Sealed` and `impl Boundary` per arity; seven invocations, 2 through 8. `query` concatenates the members' items in member order after deriving all of them (so the first `Err` wins); `absorb` asks each member, each of which consults its own query. Every emitted item path is `$crate::`- or `::`-rooted |
| `crates/happenstance/src/lib.rs` | `#[macro_use] mod composition;` declared **first**, because a `macro_rules!` is in scope only after its definition in source order; plus the one-line change to the `DecisionModel` vocabulary bullet, which now answers *"how do I check two boundaries at once?"* and links to `Boundary` |
| `crates/happenstance/src/tests.rs` | Adds AC-003's test and the two adversarial members it needs (`Unconstrained`, `Itemless`) — the only place in the workspace that can write a `Boundary` whose `query()` fails |
| `crates/happenstance/tests/composition.rs` | New. Eight tests in a **downstream** crate: one event vocabulary, one `Tally` model reused under different scopes, a real JSON codec, and the two hand-rolled wrong implementations |
| `crates/happenstance/tests/doc_budget.rs` | New. Seven source-reading tests: the five density budgets, the `$crate`-qualification assertion over the macro definition, and the surviving-glob / no-shadowing assertion |

`crates/happenstance/Cargo.toml` is **not** edited by this story (NF-001): a
`macro_rules!` needs nothing, and the manifest is M3's mount.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance --test composition` | 8 passed |
| `cargo test -p happenstance --test doc_budget` | 7 passed |
| `cargo test -p happenstance` (all targets) | 18 unit + 8 + 7 + 4 doctests + 2 compile-fail doctests — all pass |
| `cargo clippy -p happenstance --all-targets --all-features -- -D warnings` | clean |
| `cargo fmt -p happenstance -- --check` | clean; the `--write` pass ran last |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps` | warning-free |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps --no-default-features` | warning-free — AC-008's manual check, cited because the gate's own feature-off doc step covers `happenstance-core` only |
| `cargo xtask affected --base main` | **passed** (227 passed, 0 failed) — the story grain, and this slice's merge gate |
| `cargo xtask ci --fast` | **green**, after the pre-existing citation drift was routed and repaired in its own checkpoint. See *Notes* |

## Notes

**`cargo xtask ci --fast` was red on a pre-existing citation drift, outside this
slice — now routed, repaired and green.** The failing step was *the Rust
constitution is internally consistent*:

```
standards/rust/60-what-a-test-must-prove.md:68 — crates/happenstance-testkit/tests/mutation_coverage/harness.rs:454 no longer has `catch_unwind(probe.run)` within 10 lines
standards/rust/60-what-a-test-must-prove.md:69 — …/harness.rs:330 no longer has `function pointers are unconditionally` within 10 lines
standards/rust/60-what-a-test-must-prove.md:163 — …/harness.rs:296 no longer has `LAST_ORIGIN.with_borrow_mut` within 10 lines
```

`git diff d4aedfc..HEAD --name-only` shows this slice touched **nothing** under
`standards/` or `crates/happenstance-testkit/`; `harness.rs` was last changed by
`90421d0 fix(typed-layer-and-alpha-release): repair contaminated baseline test`,
which shifted the lines those three citations point at by twelve.

Reporting it was not enough: `cargo xtask ci --fast` **is** this slice's declared
merge bar (story 1's Merge DoD one-liner, project DoD 6), and a slice cannot merge
claiming a bar it has not cleared. The repair is therefore routed to the story that
owns both paths — `misbehaving-testkit-stores`, the `90421d0` neighbourhood — and
landed as **its own checkpoint commit**, carrying `Story:
typed-layer-and-alpha-release/misbehaving-testkit-stores` and `Baseline-Repair:
typed-layer-and-alpha-release` so the attribution is in the history rather than in
a report. Three line numbers changed and nothing else: `harness.rs:466`, `:342`,
`:308`; the cited anchor text is untouched. `cargo xtask lint-constitution` now
reports *27 atoms, all consistent*, and `cargo xtask ci --fast` was then run once
for the slice.

**Three implementation notes worth keeping.**

1. **A blanket impl over a *local* trait does not conflict with tuple impls.**
   `impl<M: DecisionModel> Boundary for M` and
   `impl<B1: Boundary, B2: Boundary<Event = B1::Event>> Boundary for (B1, B2)`
   coexist, because the orphan rule forbids any downstream `impl DecisionModel for
   (A, B)` and rustc can therefore prove no overlap. This was probed with a
   standalone `rustc --edition 2024` file **before** a line of the macro was
   written; a guaranteed `E0119` would have forced a different composition shape,
   and finding that out mid-implementation is the expensive way.
2. **Doc comments can be passed *into* a `macro_rules!` invocation.** `///` is
   lexed into `#[doc = "…"]` before expansion, so `$(#[$page:meta])*` in the
   matcher lets the arity-2 invocation carry the prose and the composing doctest
   while the other six arities carry none — which is what *"name the alternative
   once, not on all seven"* requires, without a second arm duplicating the body.
3. **Members are named twice in the invocation — `B1 b1`.** `self.0` cannot be
   generated from a type-parameter repetition, so the expansion destructures
   (`let (b1, b2) = self;`) and needs a value identifier per member. Reusing the
   type parameter as the binding name would work and would trip `non_snake_case`
   on every emitted impl.

**Not done, by boundary:** the 9-tuple's refusal is *stated*, not tested —
`trybuild` has no home in this tree and a `compile_fail` doctest does not
discriminate, so `compile-fail-proof-artefact` (M6) owns it. Diagnostic spans are
the same story's. Nothing here is async, nothing binds a store, no clause is cited,
and `spec-trace` is green unchanged.
