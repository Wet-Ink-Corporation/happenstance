---
item: "HS-S0020"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — DomainEvent and DecisionModel, mounted at the crate root

**All eight ACs are satisfied, and every budget inside AC-007 is met against the
design's own number.** The first pass shipped a 44-line crate-root fence and moved
the test's threshold to 44 to match; that is reverted in the review-fix pass — the
fence is 34 visible lines, `FENCE_LINES` is 35 again, and defect **D-2** is closed
as fixed rather than routed. See *Notes*.

`crates/happenstance/src/lib.rs` was 76 lines with one `pub use`. It is now the
mount for four public items — `DomainEvent`, `DecisionModel`, the sealed `Boundary`
and the `Codec`/`CodecError` vocabulary — plus a private `sealed` module, with the
surviving `pub use happenstance_core::*;` untouched at `:124`.

## TDD Evidence

Rust cannot go red on a trait that does not exist — the test file simply fails to
compile, which is a typo-shaped failure, not a behavioural one. So the RED step
landed the **whole type surface with two deliberately wrong bodies**, both of them
implementations this spec names by name:

* `Boundary::query` returned `Ok(Query::all())` — the *silently widened boundary*;
* `Boundary::absorb` decoded and folded **every** arriving event — *routing by
  arrival*, which passes any test whose models have disjoint tags.

The const assertion was absent, so the `compile_fail` fences compiled and failed as
doctests. That run produced nine failing tests, each panicking on the missing
behaviour. The GREEN step replaced the two bodies and added the per-monomorphisation
`const`; nothing was deleted, weakened or skipped.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `tests.rs::fold_is_exhaustive_over_domain_enum`, `::scope_returns_the_models_own_tags`, `::decision_model_is_clone_not_default` | green from the first run — the trait shapes were correct in RED; their discriminating power is the *compiler's* (`apply` has no `_ =>` arm, `assert_clone` is a bound witness), so a green-first result is the honest one |
| AC-002 | `tests.rs::query_is_derived_from_event_types_and_scope` | RED: `the derivation constrains something` — `Query::all()` has no items. GREEN: one item, types = the model's own sorted `EVENT_TYPES`, tags = `scope()` |
| AC-002 | `tests.rs::query_is_a_value_a_test_can_assert_on` | RED: `a derived query is not Query::all()`. GREEN: bound, `Debug`-printed, and matching only the in-scope event |
| AC-002 | doctest `boundary_cannot_be_implemented_outside_the_crate` (`boundary.rs:34`) | RED: the fence compiled, so the `compile_fail` doctest failed. GREEN: `error[E0277]: the trait bound Divergent: happenstance::sealed::Sealed is not satisfied`, with rustc's own "sealed trait" note. **Re-measured in the review-fix pass**, because the first draft's `type Event = Divergent;` also failed `type Event: DomainEvent` and would have kept the fence red with the seal deleted — it did not discriminate what it existed to prove. The fence now carries a complete `impl DomainEvent for Ev`, the unspelled fence emits exactly one diagnostic (`error: aborting due to 1 previous error`) and it is the seal's, and the negative control confirms it: strike `: crate::sealed::Sealed` from `pub trait Boundary` and the doctest goes `FAILED` |
| AC-003 | doctest `empty_event_types_does_not_compile` (`domain.rs:23`) | RED: compiled (no const assertion existed), so the fence failed. GREEN: `error[E0080]: evaluation panicked: a DomainEvent must declare at least one event type in EVENT_TYPES` + `note: erroneous constant encountered --> boundary.rs:94` + `note: … while instantiating fn <Empty as Boundary>::query` |
| AC-003 | `tests.rs::unconstrained_boundary_is_an_error_not_query_all` | RED: `assertion failed: !derived.is_all()`. GREEN: the refusal is `Err(InvalidQuery::UnconstrainedItem)` and a well-formed derivation is never `Query::all()` |
| AC-004 | `tests.rs::absorb_skips_an_unnominated_event` | RED: `an unnominated event is skipped, not an error: UnknownEventType { event_type: EventType("SomethingElse") }` — routing by arrival reached the decoder. GREEN: `Ok(())`, state untouched, for both the wrong-scope and the wrong-type case |
| AC-004 | `tests.rs::absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded` | green in both, and deliberately so: it is the *loud* half, and the pair with the row above is what discriminates nomination from arrival |
| AC-004 | `tests.rs::absorb_applies_a_decoded_event_to_the_fold` | as above — the positive control |
| AC-004 | `tests.rs::codec_error_carries_a_typed_source` | RED: `a typed source, never a String` — `#[error(transparent)]` forwards the *inner* error's source, so the box fell out of the chain. GREEN: a condition message plus an explicit `#[source]`; the chain downcasts to `serde_json::Error` |
| AC-002/004 | `tests.rs::nomination_agrees_with_the_derived_query` | RED: `the fold and the query select the same events — left: false, right: true`. GREEN: the two predicates agreed. **Re-armed in the review-fix pass**: `absorb` now asks `Query::matches` directly and the second predicate is gone, so the test became a 16-cell table over {each declared type, one undeclared type} x {in-scope, superset, disjoint, empty tags} comparing `absorb`'s observable routing against the derived query. Its discriminating power was measured by mutation, not asserted: routing by arrival (`if false`) fails it, and a tags-only predicate that ignores event types fails it |
| AC-005 | `tests.rs::shadowing::contract_names_are_not_shadowed` | green from the first run; it is a compile-time witness and its failure mode is a build break, which is the point |
| AC-006 | `cargo doc` (both feature configurations) + `rg -n "Planned, and specified"` | RED: the page carried `# Status: a facade over happenstance_core` and *"adds nothing"* beside four shipped items. GREEN: the two bullets are intra-doc links in place, the three unbuilt ones keep the promise, and both doc builds are warning-free |
| AC-007 | `tests.rs::doc_density_budget_holds`, `::public_identifiers_fit_the_item_table`, `::the_crate_root_page_fits_above_the_fold` | RED, in four successive failures that each named a real over-run: `codec.rs:1: doc prose is 81 columns`, `boundary.rs:54: a first sentence is 91 characters`, `sealed.rs:11: … 94 characters`, `lib.rs:58: a doc fence line is 77 columns … so the fence scrolls at 1024px`. GREEN after each line was rewritten. **The line budget is the exception, and it is now honest**: the first pass met `FENCE_LINES` by raising it from 35 to 44, which is the budget being moved. In the review-fix pass the constant went back to 35, the test went RED at `the first fence is 44 visible lines, over the 35 the signed-off design budgets`, and it went GREEN by shortening the program to 34 |
| AC-008 | `tests.rs::nothing_in_the_vocabulary_touches_a_store` | RED: `assertion failed: !query.is_all()`. GREEN: the whole vocabulary exercised with no runtime, no store and no `await` |
| AC-008 | `tests.rs::every_variant_event_type_is_declared` | green from the first run; it records the residual the compiler cannot see, in both directions |

## Commits

One checkpoint commit, on `initiative/from-contract-to-published-library`. It is
identified here by its trailer rather than by a literal SHA, for the reason the
template gives: a commit's SHA cannot be written inside a file that commit
contains. `redkiln record-links <item> --sha <sha>` writes it onto the item, which
is the queryable place.

| Commit | Subject |
| ------ | ------- |
| `git log --grep "Story: typed-layer-and-alpha-release/domain-event-and-decision-model"` | feat(typed-layer-and-alpha-release): DomainEvent and DecisionModel |
| `git log --grep "Slice: typed-layer-and-alpha-release/typed-vocabulary"` | fix(typed-layer-and-alpha-release): answer the typed-vocabulary slice review |

The second is the review-fix checkpoint and spans both stories of the slice; a
third, `fix(typed-layer-and-alpha-release): repair three stale constitution
citations`, carries `Story:
typed-layer-and-alpha-release/misbehaving-testkit-stores` and is not this story's.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance/src/lib.rs` | **The mount.** Module doc restructured to `_design.md`'s region order: summary, one sentence, the staged region-2 fence, the discriminator (now carrying ADR-0006's paragraph), `# The vocabulary`, adapter pointer last. The `# Status: a facade …` region is deleted; the `DomainEvent`/`DecisionModel`/`Codec` bullets become intra-doc links in place — `Codec` because the trait ships here, so *(planned)* beside it was the state AC-U03 forbids; the promise moves onto the concrete codecs and their feature flags, which really are M3's. The fence is the one-variant program (34 visible lines against the budget's 35). Adds `#![cfg_attr(docsrs, feature(doc_cfg))]`, four private `mod` declarations and three `pub use` lines beside the surviving glob |
| `crates/happenstance/src/domain.rs` | New. `DomainEvent` (`const EVENT_TYPES`, `event_type` by value, `tags`, `encode`, `decode`) and `DecisionModel` (`type Event`, `scope(&self) -> &Tags`, `apply(&mut self, Self::Event)`, supertrait `Clone`). Carries the bare `compile_fail` fence and its compiling companion |
| `crates/happenstance/src/boundary.rs` | New. The sealed `Boundary` with `type Event`, `query` and `absorb`; the blanket `impl<M: DecisionModel> Boundary for M`; `AtLeastOneType::<E>::CHECKED`, the per-monomorphisation `const`; and `pub(crate) fn derive_query`. `absorb` asks `Query::matches` on the derived query — there is no second filter predicate in this crate |
| `crates/happenstance/src/codec.rs` | New. `Codec` (`const TAG`, `encode`, `decode`; no associated `Error` type) and `#[non_exhaustive] CodecError` with a typed `#[source]` on both codec-side variants |
| `crates/happenstance/src/sealed.rs` | New, private. `pub trait Sealed {}` with the blanket impl for every `DecisionModel`, shaped so the slice-mate can add tuple impls beside it rather than having to re-open it |
| `crates/happenstance/src/tests.rs` | New, `#[cfg(test)]`. The `Ticket`/`Gate` fixture domain (deliberately not the doctests' `Seat`), a real `Json` codec, eighteen tests, and the six named budget constants — `FENCE_LINES` among them, at the design's 35 |
| `crates/happenstance/Cargo.toml` | Adds `serde` and `thiserror` (both already `[workspace.dependencies]`), a `serde_json` **dev**-dependency for the test codec, `std` forwarding for the two new deps, and `[package.metadata.docs.rs]`. The existing `serde` *feature* keeps its forwarding meaning, unrepurposed |

Not touched, by boundary: `crates/happenstance-core/**`, `examples/**`, `xtask/**`,
`spec/SPECIFICATION.md`, `CHANGELOG.md`, `crates/happenstance/README.md`.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance --all-features` | 18 lib tests, 8 in `tests/composition.rs`, 7 in `tests/doc_budget.rs`, 4 doctests, 2 compile-fail doctests — all pass |
| `cargo clippy -p happenstance --all-targets --all-features -- -D warnings` | clean (workspace `unwrap_used = "deny"`, `missing_docs`, `unreachable_pub`, pedantic) |
| `cargo fmt -p happenstance -- --check` | clean; the `--write` pass ran last, after the clippy fixes |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps` | warning-free |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps --no-default-features` | warning-free |
| `cargo xtask affected --base main` | **passed** — fmt, `clippy -D warnings`, tests for `happenstance` and its dependents, plus the file-reading lints and `spec-trace` (227 passed, 0 failed) |

`cargo xtask ci --fast` is the M2 **slice** bar and is run once, after the
slice-mate, per `decision-model-composition`'s CI table.

## Notes

**Four notes, each recorded rather than absorbed.** Items 3 and 4 were written in
the review-fix pass and replace what the first pass recorded there.

1. **`_design.md`'s `CodecError` does not compile as written.** `## Signatures`
   spells the codec-side variants `#[error(transparent)] Encode(#[source] Box<…>)`;
   thiserror answers `error: transparent variant can't contain #[source]`, measured
   here. Dropping the `#[source]` is not the fix either — `transparent` forwards the
   *inner* error's `source()`, so the box falls out of the chain and
   `codec_error_carries_a_typed_source` fails. The variants therefore carry a
   condition message and keep the explicit typed `#[source]` RS-30-2 asks for. The
   design's intent — *a typed source, never a `String`, and no associated `Error`
   type* — is preserved exactly; only the attribute spelling changed.

2. **`Boundary` carries `type Event: DomainEvent`,** which `## Signatures` does not
   show. It is required by two other signed-off rows: `commit`'s where-clause
   (`F: FnMut(&B) -> Result<Vec<B::Event>, D>`, `_design.md:514-524`) and the
   slice-mate's tuple bound `B2: Boundary<Event = B1::Event>`. RS-40-1 forbids adding
   a required item to a sealed trait after publication, so it lands now.

3. ~~**Defect D-2, for AC-012's log and AC-013's measurement.**~~ **Withdrawn and
   fixed.** The first pass met AC-007's line budget by setting the test's
   `FENCE_LINES` to 44 — the value the shipped fence measured — and logging the
   shortfall against the design's 35 as a defect. That is a gate calibrated to the
   implementation: `the_crate_root_page_fits_above_the_fold` could not fail on the
   criterion it exists to check, and 44 would have read as blessed precedent to the
   next story. Two further things made it wrong to route rather than fix. The story
   spec's *What this story must not do* forbids re-opening a signed-off `_design.md`
   row, and `_design.md`'s `## Mock` finding 1 records its own 81-line overrun as
   *"a decision the approver is being asked to take"* — it never amended the
   *Density budget* table, which still reads **35**. And AC-012's log is for defects
   in the `[FROZEN]` **contract**; a doc page over its own budget is not one, so
   there was no honest entry to write.

   The fix is (a) of the two the review offered: shorten the fence without hiding
   anything. The crate root now carries a one-variant `Seat` plus the derived-query
   assertion — 34 visible lines, 1 hidden (`# Ok::<(), …>(())`, harness), 71 columns
   at its widest — and the two-variant walkthrough it used to carry already lived
   verbatim on `DecisionModel`'s item page (`crates/happenstance/src/domain.rs:123-167`),
   where the vocabulary bullet now points. `FENCE_LINES` is 35, is documented as the
   only number it may hold, and `FENCE_LINES_DESIGNED` is deleted. Nothing moved
   behind `# ` (anti-pattern 4) and the 72-column budget still holds (anti-pattern
   3). **Nothing is routed to AC-012's log from this story**, and the routing that
   the first pass declared is withdrawn rather than left dangling for M7's
   reconciliation table.

4. **`Boundary::absorb` asks the contract's filter instead of re-implementing it.**
   The first pass carried a `pub(crate) fn nominates(types, scope, event)` spelling
   `types.contains(..) && tags.contains_all(..)` — a second filter predicate beside
   `Query::matches`, which composition's AC-U04 calls the contract's *only* filter
   vocabulary ("there is no second one"). Two predicates that agree today are a
   divergence waiting for whichever one changes first, and the agreement test that
   guarded them exercised two candidates. `nominates` is deleted; `absorb` derives
   the query and asks `Query::matches`. The cost is one derivation per event, which
   is the trade NF-002 already took against caching — a cached query makes the
   answer depend on when the boundary was built. The agreement test is now the
   16-cell table described above, and it survives because it is the thing that
   rejects a re-introduced predicate.

**The `Err` arm of `query()` is unreachable, and the test says so.** With an empty
`EVENT_TYPES` a compile error, no well-formed model can reach
`InvalidQuery::UnconstrainedItem` through `Boundary::query`. That is D-1 restated,
and `unconstrained_boundary_is_an_error_not_query_all` asserts the arm at the
derivation `query()` delegates to — `derive_query(&[], &Tags::empty())` — plus the
`Query::all()` half at the trait method itself. The alternative would have been an
`unwrap` or a `query_unchecked`; both are forbidden.

**One thing this PR could not fix, did not touch, and has now routed by name.**
`crates/happenstance/README.md:6` still reads *"Status: early, and this crate is
currently a facade. It re-exports `happenstance-core` and adds nothing yet."* That
is false as of this PR, and `crates/happenstance/src/lib.rs:10` compiles the README
into the doctest set, so the contradiction ships on a published-facing page.
`crate-readme` is **`publish-0-2-0-alpha-1`**'s surface (M7), whose AC-003 already
requires the blockquote at `:6-11` **deleted** rather than annotated and the word
*facade* absent from the page. A note naming the exact line has been added to that
story's card so the alpha is not cut with the crate root and the README disagreeing
about whether the typed layer exists. Raising it only in this report was the gap the
slice review named; the report is not a queue anyone reads.

**The slice's own integration bar was red on arrival, and the repair is routed and
landed.** `cargo xtask ci --fast` — story 1's Merge DoD one-liner and project DoD 6
— failed on three stale citations in `standards/rust/60-what-a-test-must-prove.md`
(`:68-69`, `:163`) pointing into
`crates/happenstance-testkit/tests/mutation_coverage/harness.rs`. The attribution is
not this slice's: `git log -1 --format=%H -- .../harness.rs` is `90421d0`, the
baseline repair that landed before both slice commits and moved the cited lines by
twelve, and neither slice commit touches `standards/` or the testkit. It is fixed in
its own checkpoint commit carrying `Story:
typed-layer-and-alpha-release/misbehaving-testkit-stores` — the story that owns both
paths — so the slice is not credited with a bar it did not clear on its own, and
`cargo xtask ci --fast` is then run once for the slice.

**Nothing async, nothing `Send`-bound, no conformance rule, no clause.**
`spec-trace` is green unchanged, as the spec requires.
