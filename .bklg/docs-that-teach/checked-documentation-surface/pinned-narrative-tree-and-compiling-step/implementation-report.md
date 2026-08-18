---
item: "HS-S0136"
stage: implement
created: "2026-08-17T22:30:00.000Z"
updated: "2026-08-17T22:30:00.000Z"
---

# Implementation Report — The narrative tree exists and every fence in it compiles, mandatorily

## TDD Evidence

Which tests went red then green, mapped to each AC. All thirteen unit tests were
written first, in `xtask/src/narrative_doctests.rs`'s `mod tests`, against a module
carrying only signatures — `narrative_pages` returning an empty set and
`enumerated_pages` returning it unchecked — so every failure below is an assertion
about missing behaviour rather than a compile or import error. The red run was
`cargo test --locked -p xtask --bin xtask`: **27 passed; 13 failed**. The green run,
after the six production edits, is **40 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `the_fixture_page_carries_no_hidden_marker_and_fits_the_path_budget` | RED: `reading docs/append-conditions.md: The system cannot find the file specified` — the page did not exist. GREEN after `docs/append-conditions.md` landed: no `HIDDEN_MARKERS` token present, 25-character path inside the 32-character budget. |
| AC-002 | `cargo test --locked -p xtask --doc -- narrative::` (the mechanism, not a test about it) | RED: `--list` named no `narrative::` doctest at all. GREEN: `xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9) ... ok`. Observed-fail on purpose: renaming `store.len()` → `store.length()` in the fence produced `error[E0599]: no method named 'length' found`, located at `docs/append-conditions.md:13:18`, and the step exited non-zero; reverted. |
| AC-003 | `a_listing_with_no_narrative_doctest_is_a_problem` | RED: `enumerated_pages` returned `Ok` over a constitution-only listing, so the CR-1 mis-mount was accepted. GREEN: it bails naming `xtask/src/narrative.rs`, `xtask/src/lib.rs` and the `xtask/src/main.rs` mis-mount. Confirmed end to end by commenting out `mod narrative;` in `xtask/src/lib.rs` and watching `cargo run -p xtask -- narrative-doctests` fail with that message; restored. |
| AC-004 | `the_narrative_step_is_required_and_unprobed`, `the_narrative_step_precedes_the_constitution_step`, `the_narrative_step_carries_rustdocflags_in_its_own_env`, `the_narrative_step_passes_locked` | RED: all four panicked with `REQUIRED must contain the 'the narrative tree's examples compile' step`. GREEN once the `Step` landed at `xtask/src/main.rs:481-509`, immediately before the constitution's at `:510`. |
| AC-005 | `an_empty_listing_is_a_problem`, `a_listing_with_no_narrative_doctest_is_a_problem`, `a_listing_with_one_narrative_doctest_is_accepted_and_counted` | RED: the empty listing was accepted (`Ok`), and the one-page listing counted zero pages. GREEN: empty and constitution-only listings are errors; the one-page listing counts **one page** from two doctest lines, which is what makes the printed count a fact about the tree rather than about rustdoc. |
| AC-006 | `the_gate_can_select_the_narrative_step_by_name`, `the_narrative_step_is_not_a_lint_step` | RED: the first panicked inside `steps_named` (`xtask/src/main.rs:852`) — the designed loud failure for a half-mount; the second failed its precondition that the step exists in `REQUIRED`. GREEN after the `REQUIRED` entry, the dispatch arm (`:712`) and the `print_help()` entry (`:795-799`) landed together. |
| AC-007 | `the_index_names_the_narrative_tree_as_gate_read`, `the_narrative_table_precedes_the_pointer_out_table` | RED: `docs/README.md still says two trees are gate-read; the narrative tree is a third`, and `docs/README.md carries no narrative routing table`. GREEN after the two-column narrative table went in above the pointer-out table and the pin-by-path paragraph was rewritten. |
| AC-008 | `the_new_modules_state_their_limits_first` | RED: `reading xtask/src/narrative.rs: The system cannot find the file specified`. GREEN: `# What this does not verify` is the first heading in both modules' docs, and both carry the unhedged teaching sentence. |
| AC-009 | `cargo xtask ci --fast`, `cargo xtask affected --base main`, `cargo test -p xtask --all-features`, `cargo xtask lint-constitution` | RED at the gate tier, and not in the way expected: inserting the `Step` shifted `xtask/src/main.rs` by 29 lines and broke nine `file:line` citations the Rust constitution makes into that file, so `lint-constitution` reported `9 problem(s) in standards/rust`. GREEN after repointing exactly those nine line numbers — see Notes. |

## Commits

| SHA | Subject |
| --- | ------- |
| `91aca56` | feat(checked-documentation-surface): The narrative tree exists and every fence in it compiles, mandatorily |

The row records the checkpoint **as first written**, before this row was added to it.
A commit cannot contain its own SHA, so writing the SHA down amends the commit and
gives it a new one; the reachable commit is that successor, which is identical in
content except for this table cell. It is reported in the slice digest and is what
`redkiln record-links --sha` records on the item at the advance seam.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `docs/append-conditions.md` | **New.** The fixture page, composed as `_design.md` `## The doctest` writes it: H1 (26 chars), the reserved answered-need slot present and empty, the `ES-40` citation inline in the sentence that depends on it, one `rust` fence immediately after that sentence, and the two-scope level-3 band last. 28 source lines, longest line 86 columns. |
| `xtask/src/narrative.rs` | **New.** The doctest harness. Module docs open with `# What this does not verify` and six limits; then `#[cfg(doctest)] mod append_conditions` carrying an **inner** `#![doc = include_str!("../../docs/append-conditions.md")]` — one module per page, so a failure names the page rather than a line counted from the first page. |
| `xtask/src/lib.rs` | `mod narrative;` beside `mod constitution;` (`:36`), with the reason: `cargo test --doc` compiles the **lib** target's doctests, so the same line in `main.rs` would compile clean and check nothing. |
| `xtask/src/narrative_doctests.rs` | **New.** The vacuity guard and the filtered run, in `xtask/src/proof.rs:191-240`'s shape: `list()` enumerates and hard-errors with its own message on an unreadable enumeration; `narrative_pages` parses page modules out of the listing by the `narrative::` **substring** (never a path prefix — the listing is backslashed on Windows); `enumerated_pages` bails on an empty set naming the harness, the doctest root and the mis-mount; `run()` prints one count line and runs the filtered tests. Plus the thirteen tests. |
| `xtask/src/main.rs` | Four mounts, together, because a partial mount is EC-006: `mod narrative_doctests;` (`:67`), the `REQUIRED` entry (`:481-509`) immediately before `the constitution's examples compile`, the dispatch arm (`:712`), and the `print_help()` entry (`:795-799`). Not added to `lint_steps()`. |
| `docs/README.md` | The two-column narrative table above the existing pointer-out table (whose ten rows are unchanged), the pin-by-path paragraph rewritten from two gate-read trees to three, and one opening sentence corrected — see Notes. |
| `standards/rust/{51,52,70,80}-*.md` | Nine cited `xtask/src/main.rs:NNN` line numbers repointed. Mechanical, forced by the diff — see Notes. |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask` (red run) | 27 passed; **13 failed** — the thirteen new tests, each on its own assertion. |
| `cargo test --locked -p xtask --bin xtask` (green run) | 40 passed; 0 failed. |
| `cargo test --locked -p xtask --all-features` | 231 passed; 0 failed; 3 ignored. Includes `affected::tests::a_docs_only_change_selects_nothing`, still green because it asserts over `RUNBOOK.md`. |
| `cargo test --locked -p xtask --doc -- --list` | Names `xtask\src\../../docs/append-conditions.md - narrative::append_conditions (line 9)`. |
| `cargo run -p xtask -- narrative-doctests` | `  1 page(s)' examples enumerated`, then the filtered run: `1 passed`. |
| `cargo run -p xtask -- lint-constitution` | `  27 atoms, all consistent`. |
| `cargo fmt --all --check` | Clean (exit 0), run last, after clippy. |
| `cargo clippy --locked -p xtask --all-targets --all-features -- -D warnings` | Clean (exit 0). |
| `cargo xtask affected --base main` | `=== affected packages ===`, `xtask`, then fmt / clippy / tests — `affected gate passed`. |
| `cargo xtask ci --fast` | Green. `=== the narrative tree's examples compile ===` followed by `  1 page(s)' examples enumerated`, immediately above `=== the constitution's examples compile ===`, **no `skipped` line anywhere**, closing on `all required checks passed (--fast: 4 optional step(s) not run)`. |
| `git diff --stat -- xtask/Cargo.toml Cargo.lock` | Empty. No dependency was added, and no `book.toml`, `book/`, `site/` or `.css` path appears in the diff. |

One transient to record rather than hide: the **first** `cargo xtask affected --base main`
run reported `230 passed; 1 failed` in the doctest target with no failure body, while
the same invocation immediately before and twice after reported `231 passed; 0 failed`.
It is a build-directory race between the outer `cargo run` and the doctest target it
spawns, not a property of this change; it is recorded here so that nobody re-diagnoses
it as one.

## Notes

Three deviations, all stated rather than smuggled.

1. **The fixture page omits one sentence from `_design.md`'s fenced block, and
   reorders the two scopes.** The sentence — "Corrected here from the mock's finding 1
   … this fixture is the literal artifact `pinned-narrative-tree-and-compiling-step`
   and `observed-failure-falsification` build against" — sits inside the design's
   ```` ```markdown ```` fence, but the spec's own Context pack quotes it as the thing
   `_design.md` **calls** the fixture, i.e. as design voice about the artifact rather
   than as page content. It names backlog story slugs and a mock finding; shipping it
   would put process leakage on a page addressed to someone who has run
   `cargo add happenstance`. The scope band is ordered `happenstance-postgres` then
   `happenstance-sqlite` because AC-001 and `_design.md`'s own caption both require the
   band to be **alphabetical**, and the fenced block contradicts its caption on that
   point. Both are corrections toward the design's stated properties, not away from
   them; if either is judged wrong, the fix is an amendment to `_design.md`
   `## The doctest` by its owner, and the page follows.
2. **Nine `standards/rust/**` citations were repointed, and that tree is named as out
   of scope in the PR boundary.** Inserting a 29-line `Step` into `xtask/src/main.rs`
   moved every line the constitution cites below it, and `cargo xtask lint-constitution`
   failed with nine `the citation points at the wrong place` problems. AC-009 requires
   that step green, so the citations had to move with the code — which is precisely the
   pin-by-path mechanism working, in the direction it is usually read backwards. Only
   line numbers changed: no rule, anchor, example or router entry was touched, and the
   step still reports `27 atoms, all consistent`.
3. **`docs/README.md`'s opening sentence was corrected as well as its pin paragraph.**
   The spec's implementation note 7 calls this two edits; it is three. The existing
   sentence read "It is deliberately near-empty … nothing has yet been written that
   belongs here instead", which is false the moment a page and a routing table sit
   under it. The replacement states what the tree now is, and deliberately does **not**
   claim that *every* page's examples are compiled — the page-to-module registration
   check that would make an exhaustive claim true is
   `narrative-checker-mounted-with-pinned-path`'s.

Two things deliberately **not** done, both named in the PR boundary and both still
open: `xtask/src/affected.rs` is untouched (`docs/` stays on `INERT` here, which the
slice-mate `narrative-tree-story-grain-selection` changes immediately after this
story), and no pinned `TREE`/`HARNESS`/`HIDDEN_MARKERS` constants, fence walk or
bidirectional registration check were added — `xtask/src/narrative_doctests.rs` names
the two paths its own failure message must print and says so where they are declared,
and AC-001's marker list is a test-local literal for the one page this story authors.
