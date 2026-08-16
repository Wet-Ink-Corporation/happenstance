---
item: "HS-S0020"
stage: implement
created: "2026-08-15"
updated: "2026-08-15"
---

# Implementation Report — DomainEvent and DecisionModel, mounted at the crate root

**All eight ACs are satisfied.** One budget inside AC-007 is met by a recorded,
cited exception rather than by the design's number, and the exception is stated in
full below rather than hidden — see *Notes*, defect **D-2**.

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
| AC-002 | doctest `boundary_cannot_be_implemented_outside_the_crate` (`boundary.rs:26`) | RED: the fence compiled, so the `compile_fail` doctest failed. GREEN: `error[E0277]: the trait bound Divergent: happenstance::sealed::Sealed is not satisfied`, with rustc's own "sealed trait" note |
| AC-003 | doctest `empty_event_types_does_not_compile` (`domain.rs:23`) | RED: compiled (no const assertion existed), so the fence failed. GREEN: `error[E0080]: evaluation panicked: a DomainEvent must declare at least one event type in EVENT_TYPES` + `note: erroneous constant encountered --> boundary.rs:94` + `note: … while instantiating fn <Empty as Boundary>::query` |
| AC-003 | `tests.rs::unconstrained_boundary_is_an_error_not_query_all` | RED: `assertion failed: !derived.is_all()`. GREEN: the refusal is `Err(InvalidQuery::UnconstrainedItem)` and a well-formed derivation is never `Query::all()` |
| AC-004 | `tests.rs::absorb_skips_an_unnominated_event` | RED: `an unnominated event is skipped, not an error: UnknownEventType { event_type: EventType("SomethingElse") }` — routing by arrival reached the decoder. GREEN: `Ok(())`, state untouched, for both the wrong-scope and the wrong-type case |
| AC-004 | `tests.rs::absorb_returns_unknown_event_type_when_a_nominated_event_cannot_be_decoded` | green in both, and deliberately so: it is the *loud* half, and the pair with the row above is what discriminates nomination from arrival |
| AC-004 | `tests.rs::absorb_applies_a_decoded_event_to_the_fold` | as above — the positive control |
| AC-004 | `tests.rs::codec_error_carries_a_typed_source` | RED: `a typed source, never a String` — `#[error(transparent)]` forwards the *inner* error's source, so the box fell out of the chain. GREEN: a condition message plus an explicit `#[source]`; the chain downcasts to `serde_json::Error` |
| AC-002/004 | `tests.rs::nomination_agrees_with_the_derived_query` | RED: `the fold and the query select the same events — left: false, right: true`. GREEN: `nominates` and `Query::matches` agree on every candidate |
| AC-005 | `tests.rs::shadowing::contract_names_are_not_shadowed` | green from the first run; it is a compile-time witness and its failure mode is a build break, which is the point |
| AC-006 | `cargo doc` (both feature configurations) + `rg -n "Planned, and specified"` | RED: the page carried `# Status: a facade over happenstance_core` and *"adds nothing"* beside four shipped items. GREEN: the two bullets are intra-doc links in place, the three unbuilt ones keep the promise, and both doc builds are warning-free |
| AC-007 | `tests.rs::doc_density_budget_holds`, `::public_identifiers_fit_the_item_table`, `::the_crate_root_page_fits_above_the_fold` | RED, in four successive failures that each named a real over-run: `codec.rs:1: doc prose is 81 columns`, `boundary.rs:54: a first sentence is 91 characters`, `sealed.rs:11: … 94 characters`, `lib.rs:58: a doc fence line is 77 columns … so the fence scrolls at 1024px`. GREEN after each line was rewritten — **not** after the budget was moved |
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

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `crates/happenstance/src/lib.rs` | **The mount.** Module doc restructured to `_design.md`'s region order: summary, one sentence, the staged region-2 fence, the discriminator (now carrying ADR-0006's paragraph), `# The vocabulary`, adapter pointer last. The `# Status: a facade …` region is deleted; the `DomainEvent`/`DecisionModel` bullets become intra-doc links in place. Adds `#![cfg_attr(docsrs, feature(doc_cfg))]`, four private `mod` declarations and three `pub use` lines beside the surviving glob |
| `crates/happenstance/src/domain.rs` | New. `DomainEvent` (`const EVENT_TYPES`, `event_type` by value, `tags`, `encode`, `decode`) and `DecisionModel` (`type Event`, `scope(&self) -> &Tags`, `apply(&mut self, Self::Event)`, supertrait `Clone`). Carries the bare `compile_fail` fence and its compiling companion |
| `crates/happenstance/src/boundary.rs` | New. The sealed `Boundary` with `type Event`, `query` and `absorb`; the blanket `impl<M: DecisionModel> Boundary for M`; `AtLeastOneType::<E>::CHECKED`, the per-monomorphisation `const`; and the two `pub(crate)` halves of the derivation, `derive_query` and `nominates` |
| `crates/happenstance/src/codec.rs` | New. `Codec` (`const TAG`, `encode`, `decode`; no associated `Error` type) and `#[non_exhaustive] CodecError` with a typed `#[source]` on both codec-side variants |
| `crates/happenstance/src/sealed.rs` | New, private. `pub trait Sealed {}` with the blanket impl for every `DecisionModel`, shaped so the slice-mate can add tuple impls beside it rather than having to re-open it |
| `crates/happenstance/src/tests.rs` | New, `#[cfg(test)]`. The `Ticket`/`Gate` fixture domain (deliberately not the doctests' `Seat`), a real `Json` codec, seventeen tests |
| `crates/happenstance/Cargo.toml` | Adds `serde` and `thiserror` (both already `[workspace.dependencies]`), a `serde_json` **dev**-dependency for the test codec, `std` forwarding for the two new deps, and `[package.metadata.docs.rs]`. The existing `serde` *feature* keeps its forwarding meaning, unrepurposed |

Not touched, by boundary: `crates/happenstance-core/**`, `examples/**`, `xtask/**`,
`spec/SPECIFICATION.md`, `CHANGELOG.md`, `crates/happenstance/README.md`.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p happenstance --all-features` | 17 unit tests, 3 doctests, 2 compile-fail doctests — all pass |
| `cargo clippy -p happenstance --all-targets --all-features -- -D warnings` | clean (workspace `unwrap_used = "deny"`, `missing_docs`, `unreachable_pub`, pedantic) |
| `cargo fmt -p happenstance -- --check` | clean; the `--write` pass ran last, after the clippy fixes |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps` | warning-free |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p happenstance --no-deps --no-default-features` | warning-free |
| `cargo xtask affected --base main` | **passed** — fmt, `clippy -D warnings`, tests for `happenstance` and its dependents, plus the file-reading lints and `spec-trace` (227 passed, 0 failed) |

`cargo xtask ci --fast` is the M2 **slice** bar and is run once, after the
slice-mate, per `decision-model-composition`'s CI table.

## Notes

**Three deviations, each recorded rather than absorbed.**

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

3. **Defect D-2, for AC-012's log and AC-013's measurement.** *The signed-off
   `<= 35`-visible-line budget for the crate-root fence is not satisfiable under the
   signed-off `DomainEvent` signature.* The mapping ceremony — `EVENT_TYPES`,
   `event_type`, `tags`, `encode`, `decode` — is 22 lines before any domain exists;
   the smallest honest two-variant vocabulary program is 44. `_design.md`'s own mock
   measured the program it budgeted at **81** lines and **107** columns (`## Mock`,
   finding 1) and the approver accepted that as DT-2's measurement. What is *not*
   conceded is the column budget: the fence here holds 72 columns, so it does not
   scroll at 1024px (anti-pattern 3), and no line was hidden behind `# ` to shorten
   it (anti-pattern 4). Both numbers live in `tests.rs` as `FENCE_LINES` and
   `FENCE_LINES_DESIGNED` so the gap is visible where someone would try to widen it.

**The `Err` arm of `query()` is unreachable, and the test says so.** With an empty
`EVENT_TYPES` a compile error, no well-formed model can reach
`InvalidQuery::UnconstrainedItem` through `Boundary::query`. That is D-1 restated,
and `unconstrained_boundary_is_an_error_not_query_all` asserts the arm at the
derivation `query()` delegates to — `derive_query(&[], &Tags::empty())` — plus the
`Query::all()` half at the trait method itself. The alternative would have been an
`unwrap` or a `query_unchecked`; both are forbidden.

**One thing this PR could not fix and did not touch.** `crates/happenstance/README.md:6`
still reads *"Status: early, and this crate is currently a facade."* That is now
false, and it is `crate-readme`'s surface, explicitly outside this PR's globs
(M5–M7). Raised here so the next reader of the README knows why it disagrees with
the crate root.

**Nothing async, nothing `Send`-bound, no conformance rule, no clause.**
`spec-trace` is green unchanged, as the spec requires.
