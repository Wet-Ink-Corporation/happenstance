---
item: "HS-S0185"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The opening encounter reaches a refused append

## TDD Evidence

Two red steps, and they went red for two different reasons on purpose. The first is the
composition suite failing over a page that does not exist; the second is the *page's own
assertion* failing over a boundary that is not load-bearing — which is EC-004, reproduced
rather than forecast.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `xtask/src/narrative.rs::first_encounter` doctest, step 3 (`docs/first-encounter.md:96`), run by `cargo test -p xtask --doc` | **Red** — the page did not exist, so the doctest did not exist either and `xtask/tests/first_encounter.rs::step_three_prints_the_refusal_from_inside_the_matched_arm` failed on `docs/first-encounter.md is readable`. **Green** — 3 doctests pass; the printed line is `refused: ConditionViolated`, emitted from inside the matched `Err(AppendError::ConditionViolated(_))` arm, and the output block at `:122-124` is byte-equal to it |
| AC-002 | `xtask/tests/first_encounter.rs::the_crate_root_declares_its_one_need_above_its_first_fence`; `cargo doc` under both invocations | **Red** — `the crate root declares 0 needs, not one: []`, and the gate's own doc invocation rendered **2** literal `[happenstance_core]` bracket pairs while exiting 0. **Green (partial)** — the answered-need line lands at `crates/happenstance/src/lib.rs:21` and the bracket count is 0 under both invocations. The refusal-fence clause is **not** green and is routed as BC-002 — see `## Notes` |
| AC-003 | `::every_later_step_opens_on_its_two_line_header_block`, `::step_one_carries_the_answered_need_in_line_ones_place`, `::every_step_fence_is_a_complete_program` | **Red** — all three failed on the missing page. **Green** — steps 2 and 3 open on a two-line header block linking `#append-and-read-back` in one hop; step one omits it; each fence carries its own `use`, `#[tokio::main]` and `async fn main()`, which tier 3 independently proves by compiling and running all three as separate units |
| AC-004 | The step-3 doctest itself, plus `::the_racing_append_and_the_after_are_both_load_bearing` | **Red, deliberately** — step 3 was first authored with the racing append untagged, exactly as the design's own fence sketch has it. `cargo test -p xtask --doc -- first_encounter` failed: `the boundary did not hold: Ok(SequencePosition(2))` at `narrative::first_encounter (line 96)`. **Green** — `.with_tags(held)` at `docs/first-encounter.md:109` makes the tag join load-bearing and the append is refused |
| AC-005 | `::no_hidden_line_carries_any_part_of_the_boundary` | **Red** — failed on the missing page. **Green** — zero `#`-prefixed lines in any `rust` fence on the page, so every line that builds the boundary renders |
| AC-006 | `::the_page_is_registered_in_the_harness_and_indexed`, `::no_fence_on_the_page_opts_out_of_the_compiler`, and HS-P0020's own checker | **Red** — `xtask/src/narrative.rs does not include the page, so nothing compiles its fences`. **Green** — the `#[cfg(doctest)] mod first_encounter` registration, the `docs/README.md` row, and `cargo run -p xtask -- lints` → `3 pages, all consistent` |
| AC-007 | `::every_fence_holds_the_density_budget`, `::every_paragraph_holds_its_budget`, `::the_page_declares_exactly_one_need_above_its_first_fence`, `::the_page_introduces_no_affordance`, `::neither_surface_names_the_prior_model`, `::every_fence_imports_from_the_facade_and_binds_the_weaker_trait` | **Red** — six failures. **Green (partial)** — all six pass; step 3's fence lands at exactly 24 lines and exactly 68 columns, matching `_resolutions.md` § DT-4's measured shape. The three inherited crate-root numbers are **not** green and are routed as BC-002 |
| AC-008 | `::every_step_closes_on_a_clause_citation`, `::no_sentence_states_a_rule_in_the_pages_own_words`, `cargo run -p xtask -- spec-trace` | **Red** — failed on the missing page. **Green (partial)** — all three steps close on an inline clause link (ES-8, ES-26, ES-25); `spec-trace` reports `no problems found`; no `MUST` appears on either surface. The crate-root section that would carry the citation does not exist — routed as BC-002 |

Seventeen assertions, all red before the page existed, all green after:
`cargo test -p xtask --test first_encounter` → `17 passed; 0 failed`.

> **Fix pass, 2026-08-19 — an eighteenth assertion, red first.**
> `::step_two_reaches_the_boundary_its_citation_claims` was written against the *shipped* step 2
> and failed on it — `step two never lands an event of its own` — because that program cited
> ES-26's exclusivity over an empty store, where `after_opt(None)` means "no matching event at
> all" and the boundary is never reached. Green after the program was corrected to seed, read
> back and guard on the observed position:
> `cargo test -p xtask --test first_encounter` → `18 passed; 0 failed`, and
> `cargo test -p xtask --doc -- first_encounter` → `3 passed`. The rows above are the
> implementation run's record and are left as written; the ledger's AC-002 and AC-008 are
> flipped on the fix pass against `spec.md` § Amendment — BC-002, and AC-007 is carried open —
> `_conditions.md` § *Acceptance reconciliation*.

> **Second fix pass, 2026-08-19 — AC-004's row above records a check that could not fail.**
> `::the_racing_append_and_the_after_are_both_load_bearing` proved the tag join (the red in the
> row stands, only its position moves to `Ok(SequencePosition(3))` under the corrected
> scenario) and asserted the rest with a source substring, `program.contains("after_opt(upto)")`,
> over a step 3 that read an **empty** store: `upto` was `None`, `AppendCondition::new` already
> carries `after: None`, and the call was inert — delete it and the doctest stayed green.
> Replaced by two assertions with the substring gone:
> `::the_racing_append_carries_the_tag_the_guards_query_joins_on` and
> `::step_three_guards_on_a_position_its_own_read_observed`, the second **verified red against
> the shipped program** (*"step three reads an empty store, so `upto` is None and
> `after_opt(upto)` is inert"*) and green after step 3 was corrected to seed, read
> `Some(SequencePosition(1))`, race above it and guard on it.
> `cargo test -p xtask --test first_encounter` → `19 passed; 0 failed`;
> `cargo test -p happenstance --test boundary_refusal` → `3 passed`, the third being
> `::the_after_is_load_bearing_in_its_value_not_in_its_presence`, which executes BC-004's
> finding: whole-log still refuses, a guard built from a read taken after the race is accepted.
> AC-007 is no longer carried open — `8161418` opened HS-B0001 and the row is flipped against
> its id. The rows above are the implementation run's record and are left as written.

## Commits

| SHA | Subject |
| --- | ------- |
| `9493276` (amended from `077e3ff` only to carry this SHA) | feat(application-author-path): Boundary refusal encounter |

## Changes

| File | Shape of the change |
| --- | --- |
| `docs/first-encounter.md` | **New.** One page, three `##` steps, in the design's fixed per-step order: heading → step header block → one or two setup sentences → the fence → the output block → the clause citation. The answered-need line sits at `:3`, immediately under the H1, in HS-P0021's notation. Each fence is a complete `#[tokio::main]` program that compiles *and* runs; none carries `ignore`, `no_run` or `compile_fail`; none carries a hidden line. Step 3 prints `refused: ConditionViolated` from inside the matched refusal arm and panics on any other outcome. The `###` drill slot at the bottom of step 3 is left for the slice-mate |
| `xtask/src/narrative.rs` | **The mount.** One `#[cfg(doctest)] mod first_encounter` carrying `include_str!("../../docs/first-encounter.md")`, one module per page so a failure names the page, with a comment stating what the module buys |
| `docs/README.md` | **Reachability.** One row in the narrative table. Not a row in the pointer-out table below it, which is HS-P0023's |
| `crates/happenstance/src/lib.rs` | Four lines of doc: the answered-need line under the summary (`:21`), a one-sentence pointer from beneath the fence to the opening encounter (`:66`), and both `[happenstance_core]` shortcut references made explicit intra-doc links (`:70`, `:144`) with the two paragraphs re-wrapped to hold the 80-column prose budget. No landing copy, no status vocabulary, no registry metadata, and ADR-0006's reasoning untouched |
| `xtask/tests/first_encounter.rs` | **New.** Seventeen source-reading assertions over both surfaces — the composition, transience, density, vocabulary and citation checks that no compiler makes and that a completely unstyled render passes. Written in the idiom `crates/happenstance/tests/doc_budget.rs` and `docs_composition.rs` already established |
| `.bklg/.../boundary-refusal-encounter/_conditions.md` | **New.** The EC-008 record: BC-002 (the crate root's one fence is already spoken for), BC-003 (the bracket pairs, now zero under both invocations), and four observations |
| `.bklg/.../boundary-refusal-encounter/_ledger.md` | Five rows flipped to `satisfied: true` with cited evidence; three left `satisfied: false` with their met half cited and their blocked half routed |

Not touched, deliberately: `_design.md` (a human's sign-off), `xtask/src/main.rs`'s `REQUIRED`,
`crates/happenstance/Cargo.toml`, `spec/SPECIFICATION.md`, and `IGNORE_ALLOWANCES`, which is
still `&[]`.

## Gates

| Command | Result |
| --- | --- |
| `cargo test -p xtask --test first_encounter` | **green** — 17 passed, 0 failed |
| `cargo test -p xtask --doc -- first_encounter` | **green** — 3 passed; the page's three fences compiled *and* executed |
| `cargo run -p xtask -- lints` | **green** — `3 pages, all consistent` (the narrative checker, up from 2) and `3 pages, 16 rules, all consistent` (the page-need checker) |
| `cargo run -p xtask -- spec-trace` | **green** — `traceability: no problems found` |
| `cargo test -p happenstance --test doc_budget --test doc_surface --test docs_composition` | **green** — 7 + 10 + 10 passed; the crate root still holds its one-fence, 130-line, 12-lines-to-first-fence and region-order assertions |
| `cargo doc -p happenstance --no-deps` | **green** — 0 literal `[<code>` occurrences in `target/doc/happenstance/index.html` |
| `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --all-features --no-deps --document-private-items` | **green**, exit 0 — and now **0** literal `[<code>` occurrences where the baseline measured 2 |
| `cargo run -p xtask -- affected --base main` | **green** — `affected gate passed`, over 12 affected packages, including the formatting step |

## Notes

**One deviation, and it is the whole of what this story could not do.** `_design.md`
`## Composition` composes the crate root with `## Watch a boundary refuse` — retitled
`A boundary refuses` by `tension-resolutions/_resolutions.md` § Anchor table — as a second
region carrying the refusal fence, and `merge-forward-preflight/_baseline.md` routed the crate
root's two exceeded density rows here with the sentence *"the rewrite replaces the program"*.
The merged tree makes that impossible without re-litigating another project's signed-off gate:
`crates/happenstance/tests/doc_budget.rs` asserts the crate root carries **exactly one** fence,
on the recorded ground that "a second one would demote the first, which is the page's primary
hierarchy signal", and caps the module doc at 130 lines — which this story's four additions
take it to exactly. The only way to that section is to delete the `commit` program HS-P0016
signed off, which is landing copy `project.md`'s risk table says this project does not touch,
which two vocabulary bullets refer to by name, and which is the crate's one demonstration of
the typed layer the bare name exists for (ADR-0006).

EC-008's response is to record the contradiction and route it, never to fold it in and never to
edit the signed-off file. That record is `_conditions.md` **BC-002**, routed to the `_design.md`
sign-off owner, which is the route `spec.md` EC-006 names in its own words. AC-002, AC-007 and
AC-008 each carry one crate-root clause and each stays `satisfied: false`; their page halves are
met and cited on the row. Nothing was flipped on a partial.

**What the crate root did get**, so the half is not empty: the answered-need line the baseline
recorded as *absent, as expected — this project authors it*; a pointer from directly beneath the
fence to `docs/first-encounter.md`, which is what makes the page reachable from what a
`cargo add happenstance` reader lands on; and anti-pattern 2 closed to zero under **both** doc
invocations, which is the statement `_baseline.md` disposition row 9 asked this story for.

**Three smaller deviations, all recorded rather than absorbed.**

1. Step 3 prints from a `match` arm and panics on any other outcome, where `_resolutions.md`
   § DT-4's transcript used `println!("{refused:?}")` followed by
   `assert!(matches!(…))`. The reason is the one `spec.md` EC-005 states: `Display` on
   `ConditionViolated` never contains the token, and the bare `Debug` line is 93 characters,
   which scrolls at every viewport. The `match` puts the token in a 26-character line that is
   unreachable unless the store refused, and its other arm reports what actually happened —
   strictly more than `assert!(matches!(…))` would. The fence still lands at exactly 24 rendered
   lines and 68 columns, the zero-headroom shape DT-4 certified; the `use` block was folded from
   four lines to three to pay for the two the `match` costs.
2. `xtask/tests/first_encounter.rs` is one path outside `spec.md`'s five-entry PR-boundary
   fence. It is the repository's own idiom for a composition check no compiler can make, and
   `xtask` is the crate that owns the narrative tree. Recorded as observation O4 rather than
   answered by widening the fence.

   > **Corrected 2026-08-19.** Recording is not one of the two responses EC-010 sanctions.
   > The fence is amended to admit `xtask/tests/**`, tests only, with the reason inline and on
   > its own commit (`4232b34`). The reasoning above is why the amendment went that way rather
   > than a revert; it was never a licence to leave the violation standing.
3. Steps 1 and 2 close on ES-8 and ES-26 rather than on ES-25. `_design.md`'s per-step
   composition requires a clause citation as every step's last element and names ES-25 only for
   step 3; ordering and the exclusivity of `after` are what steps 1 and 2 actually demonstrate.
   Both ids resolve under `spec-trace` and both lines come from `_baseline.md` § Anchors.

   > **Corrected 2026-08-19.** Step 2 *cited* the exclusivity of `after` over a program that
   > did not reach it: the store was empty when it read, so `after_opt(None)` meant "no matching
   > event at all" (ES-25's other half) and ES-26's boundary was never compared against
   > anything. The program is corrected rather than the sentence — step 2 now lands one matching
   > event, reads it back so `upto` is `Some(SequencePosition(1))`, and guards on exactly that
   > position; its output block shows both numbers. `::step_two_reaches_the_boundary_its_citation_claims`
   > is the new assertion that ties the citation to what the fence does, verified red against
   > the old program. The three fences still open at lines 16, 55 and 96, so the drill's quoted
   > test name and `_drill-observation.md` stay byte-accurate.
