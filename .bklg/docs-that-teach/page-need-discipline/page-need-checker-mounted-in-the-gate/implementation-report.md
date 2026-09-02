---
item: "HS-S0150"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The page-need checker, mounted as an ordinary gate step

## TDD Evidence

The unit tests live in `xtask/src/lint_pages.rs`'s `#[cfg(test)] mod tests` and
`xtask/src/affected.rs`'s `mod tests`. The Red step was taken against *compiling* stubs, not
against absent items: every new function was introduced with its real signature and an
empty body (`Vec::new()`, `String::new()`, `Ok(())`), so the failures are assertions about
missing behaviour rather than `E0425`s. That distinction is the whole point — a compile error
proves the test was written, not that the behaviour is absent.

**Red run** — `cargo test -p xtask --bin xtask lint_pages`: **64 passed; 17 failed**. Every
failure is an empty-result assertion: `assertion 'left == right' failed: three wrong pages,
three problems: []`, `one problem for the directory: []`, `` `rust` fence: [] ``, and
`left: "| Atom | Load when | Rules |\n|---|---|---|" right: ""` for the generated index.

**Green run** — the same command: **81 passed; 0 failed**. Whole crate: `cargo test -p xtask`
→ **261 passed; 0 failed** plus 168 + 63 doctests.

Nine of the new tests passed on the Red run and are named here rather than hidden. Six are the
directory-reader tests (AC-002, AC-003): `rule_atoms`, `governed_pages` and the two guards were
written for real in the same pass, because a stub of a directory walk has no failure mode to
assert. Three are *negative* tests — `prose_that_mentions_a_need_word_is_never_a_declaration`,
`one_orientation_page_per_directory_passes`, `a_well_formed_atom_reports_nothing` — which a stub
satisfies vacuously; each earns its keep only beside the positive test in the same AC row, and
each of those went red first.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | gate-integration, three routes | RED: `cargo xtask lints` had no such step; `steps_named` would have panicked had `lint_steps()` named it before `REQUIRED` carried it. GREEN: the step prints in `cargo xtask ci` (21st of 27), in `cargo xtask lints`, and in `cargo xtask affected --base main` under `=== the file-reading checks ===`. |
| AC-002 | `a_missing_rules_tree_fails_and_names_the_path_it_expected`, `an_empty_rules_tree_is_vacuous_rather_than_green`, `a_missing_pages_tree_fails_and_names_the_path_it_expected`, `an_empty_pages_tree_is_vacuous_rather_than_green` | Green on the Red run — the readers and both guards were written as real code in the same pass, since an empty-bodied directory walk cannot fail for the right reason. Both trees, both guards; no `tempfile`. |
| AC-003 | `the_pages_root_resolves_from_the_narrative_trees_own_constant`, `the_index_is_not_a_governed_page` | RED, once, at the *compiler*: `error[E0603]: constant 'TREE' is private` at three call sites. That is the seam the story exists to open, and the repair was the one `pub(crate)` token in `xtask/src/lint_narrative.rs:239`, never a second `PAGE_DIR`. |
| AC-004 | `one_declaration_parses_to_its_token_and_its_line`, `a_page_with_no_declaration_is_one_problem_naming_band_zero`, `a_second_declaration_is_reported_at_the_second_ones_line`, `a_malformed_declaration_is_its_own_problem_not_a_missing_one`, `prose_that_mentions_a_need_word_is_never_a_declaration` | RED: the parser returned `[]`, so the first four asserted `0 != 1` / `[] != [...]`. GREEN once `declarations` + `declaration` + `check_declarations` landed. The fifth was green throughout and is the false-positive guard. |
| AC-005 | `the_three_wrong_pages_are_each_rejected_at_the_right_line` | RED: `three wrong pages, three problems: []`. GREEN with all three at the right line inside the literal — `:4`, file-level, `:3`. No committed broken page: `git status --porcelain` clean of `docs/` breaks after the captures. |
| AC-006 | `two_orientation_pages_at_one_level_are_one_problem_naming_both`, `one_orientation_page_per_directory_passes` | RED: `one problem for the directory: []`. GREEN with the directory as the problem's path and every offending `path:line` in the message. |
| AC-007 | `a_router_index_disagreeing_by_one_atom_names_the_repair_in_the_line`, `write_mode_rewrites_only_the_region_and_reaches_equality`, `a_dangling_router_link_is_a_problem_at_its_own_line` | RED: the generator returned `""`, so the equality test failed against the committed header. GREEN, and then the forward contract discharged for real: the **first** `cargo run --locked -p xtask -- lint-pages --write` against the router `router-precedence-and-announcement` committed produced **no diff**. EC-008 did not fire. |
| AC-008 | `a_needs_member_missing_from_band_ten_names_which_token_moved`, `a_need_shaped_token_in_band_ten_that_is_not_a_member_is_named`, `band_tens_job_column_cannot_drift_from_the_const` | RED: `check_need_set` pushed nothing, so all three asserted `0 != 1`. GREEN with containment, exclusion and the job cell, each naming which token moved and citing both paths. |
| AC-009 | `a_well_formed_atom_reports_nothing`, `an_atom_missing_a_section_is_one_problem_at_the_rules_line`, `an_atom_with_no_rule_at_all_is_a_problem`, `an_atom_over_the_rule_ceiling_is_a_problem_naming_the_ceiling`, `an_atom_over_the_byte_ceiling_carries_the_measured_count`, `text_and_markdown_fences_pass_while_rust_and_untagged_fail` | RED for five of six (the well-formed one is the vacuous pass). The fence test's Red is the sharpest: `` `rust` fence: [] `` — the check that a `rust` fence is *rejected* here, where `lint_constitution` rejects the opposite. GREEN with the byte count measured into the message. |
| AC-010 | `a_problem_line_is_path_line_dash_message_in_that_order`, `problems_sort_by_path_then_line_whatever_order_they_arrive_in` | Both green from the first run — `Problem` and its derived `Ord` were written before the checks that push onto it, deliberately, because the composition contract is what every other test's assertion string depends on. The three verbatim captures are the perceptual instrument; the measured longest line is **135 characters**. |
| AC-011 | `the_rules_tree_selects_no_package`, `the_pages_prefix_does_not_swallow_the_constitution` | RED: `standards/pages/README.md should reach no package` failed — the path was unrecognised and widened to every member, which is the correct-but-slow behaviour this story replaces. GREEN once `"standards/pages/"` joined `INERT` **in the same change** as the unconditional-list entry. |
| AC-012 | `module_docs_open_with_what_this_does_not_verify`, `cargo clippy … -D warnings` | The docs test was green throughout (the six limits were the foundation story's). The bite is the `allow` deletion: with `#![allow(dead_code, …)]` gone, clippy is the test, and it went **red** on `Need::job` having no consumer until `check_need_set` gained the job-cell comparison — which is exactly what band 10 claims about both of that struct's fields. |

## Commits

| SHA | Subject |
| --- | ------- |
| `ee0a500` | `feat(page-need-discipline): Page-need checker mounted in the gate` |

`ee0a5000955bbe7230874ce5897e46eee99d863a`. A commit cannot record its own SHA, so this row was
written one commit later, in the slice-mate `declaration-check-seen-to-fail`'s checkpoint — which
is the only other commit in this slice and the one whose five captures were all taken at exactly
this SHA.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_pages.rs` | The module became a checker. Added: `Mode`, `STEP`, `Problem` with its `render` and derived `Ord`, `Atom`, `Page`, `Declaration`, `pub(crate) fn run`, `report`, `rule_atoms`, `governed_pages`, `collect_pages`, both vacuity guards, `declarations`/`declaration`, `check_declarations`, `check_orientation_ceiling`, `check_atom_shape`, `check_atom_fences`, `check_router`, `generated_index`, `region`, `check_need_set`, and the ceilings/`SECTIONS` copies. Nine helpers that existed only inside `mod tests` (`load_when`, `rules`, `table_rows`, `markdown_link_targets`, `Fence`/`fences`, the fence rule, the need-table reader, `SECTIONS`) were **promoted** to the module proper and deleted from the test module, so the checker and the tests read one implementation rather than two. The module docs gained the three divergences (fence rule inverts, the generated region carries more weight, no shared abstraction) and the stated absence of a `check_harness` equivalent. **`#![allow(dead_code, reason = …)]` deleted.** 30 new tests. |
| `xtask/src/main.rs` | Four mounts: the `Step` in `REQUIRED` (`:567`, `probe: None`, `--locked`), the dispatch arm with its `--write` and unknown-flag branches (`:781`), the `print_help()` block (`:875`), and the name in `lint_steps()` (`:925`). |
| `xtask/src/affected.rs` | The pair, in one change: `crate::lint_pages::run(Mode::Check)` on the unconditional file-reading list (`:139`) and `"standards/pages/"` on `INERT` (`:300`), plus the two guard tests (`:797`, `:813`). |
| `xtask/src/lint_narrative.rs` | One token: `const TREE` → `pub(crate) const TREE` (`:239`), with a doc paragraph saying why (two scanners agreeing on where one tree is; RS-81-3 forbids one scanner over two trees, not this). |
| `docs/append-conditions.md`, `docs/text-fences.md` | The `*<!-- answered-need: reserved for HS-P0021 -->*` placeholder — reserved for this project by HS-P0020's own `_design.md:614` — replaced by the real declaration. One line each; no teaching content authored. See Notes. |
| `docs/README.md` | The `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker removed and the paragraph made unconditionally true, naming the dedicated step and the fact that it reads `docs/` from the other side. The closing sentence about moving a tree meaning an `xtask/src/` edit is preserved verbatim. |
| `standards/pages/README.md` | Prose only, and only the `## What checks this tree, and what does not` section: the sentence *"No **dedicated** gate step reads this tree yet"* became false the moment the step landed. Smallest legal edit, reasoning preserved. **The generated region is byte-identical to HEAD** — the first `--write` produced no diff. |
| `standards/rust/51-*.md`, `52-*.md`, `70-*.md`, `80-the-gate.md` | Line numbers only, in nine `xtask/src/main.rs:NNN` citations that the mount pushed past `lint_constitution`'s 10-line anchor slack. No prose changed. `git diff main -- standards/rust/README.md` is empty. See Notes. |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p xtask` | 261 passed, 0 failed; doctests 168 passed / 3 ignored and 63 compile-fail passed |
| `cargo run --locked --quiet -p xtask -- lint-pages` | `  2 pages, 16 rules, all consistent`, exit 0 |
| `cargo run --locked -p xtask -- lint-pages --write` then `git diff --exit-code standards/pages/README.md` | no diff on the generated region — AC-007's forward contract |
| `cargo run --locked --quiet -p xtask -- lints` | green; `=== every page declares one need ===` present |
| `cargo run --locked --quiet -p xtask -- affected --base main` | `affected gate passed`; the checker runs under `=== the file-reading checks ===`, `standards/pages/` selects no package |
| `cargo run --locked --quiet -p xtask -- ci` | **`all checks passed`** — 27 steps, the new one 21st, nothing skipped that a tool could answer |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean, with the scaffold `allow` deleted |
| `redkiln doctor` | no problems; exactly **six** `template-drift` advisories |
| `redkiln validate --kb` | `validate passed` — nothing hand-authored into `.kb/` |
| `git diff main --stat -- crates/`, `-- xtask/Cargo.toml`, `-- spec/`, `-- .kb`, `-- standards/rust/README.md` | all empty |

One flake observed and re-run rather than worked around: the first
`cargo xtask affected --base main` reported a single doctest failure in
`standards/rust/12-manual-impls-and-derive-traps.md (line 39)` under the merged-doctest
compilation; `cargo test -p xtask --doc` on its own was green, and the immediately following
affected run was green. Recorded here rather than left out, because a flake nobody wrote down
becomes a defect nobody can reproduce.

## Notes

**Three deviations from the spec's PR boundary, each forced and each recorded rather than
quietly taken.**

1. **`xtask/src/narrative.rs` does not hold the pages-tree constant; `xtask/src/lint_narrative.rs`
   does.** The spec (and EC-005) expected `xtask::narrative::TREE`. HS-P0020 landed the pinned
   constant as `TREE` in the *checker* module (`xtask/src/lint_narrative.rs:231` before this
   change), while `xtask/src/narrative.rs` is the lib-target doctest harness. EC-005's halt
   condition is that the constant is **absent** — it is not; only the module path differs. The
   obligation the rule exists for (one constant, referenced, never re-declared) is discharged
   exactly, and the one-token `pub(crate)` change landed in the module that actually holds it.
   Not a block.

2. **Two pages in HS-P0020's tree gained their declaration.** The spec excludes "any page in
   HS-P0020's narrative tree". Without this the story cannot exist: the checker mounted in
   `REQUIRED` reports `docs/append-conditions.md — no > **Answers:** line` and
   `docs/text-fences.md — no > **Answers:** line`, and `cargo xtask ci` is red — so either the
   step is not mounted (the story's whole deliverable) or the zero-declaration check is softened
   (the decorative-gate failure this project exists to refuse). The minimal real version was
   built in-slice, and it is not an invention: both pages carry
   `*<!-- answered-need: reserved for HS-P0021 -->*`, a slot HS-P0020's own signed-off design
   reserved **for this project** (`.bklg/docs-that-teach/checked-documentation-surface/_design.md:614`).
   One line replaced per page; no teaching content authored, no page added, no page removed.
   The slice-mate's EC-001 is thereby also cleared: the tree now holds two governed pages, so
   `declaration-check-seen-to-fail` has a non-vacuous green baseline to break and return to.

3. **Nine `xtask/src/main.rs:NNN` citations in four constitution atoms were re-pointed.** The
   spec says `standards/rust/**` — nothing. Mounting a `Step` in `REQUIRED` moves every line
   below it, and `lint_constitution`'s anchor slack is ten lines (`ANCHOR_SLACK`,
   `lint_constitution.rs:111`); the mount moved them by 27-43. There is no insertion point in
   `main.rs` that avoids this, so the citations *must* move or the `the Rust constitution is
   internally consistent` step stays red. Only the numbers changed; every anchor phrase is
   unedited and `lint-constitution` re-verifies each one against the new line. **Project AC-002's
   own assertion is intact: `git diff main -- standards/rust/README.md` is empty.**

**One design reading, not a new decision.** `_design.md`'s S1 route is `<PAGE_DIR>/**/*.md`, which
would make `docs/README.md` a governed page. It is not treated as one, and the authority is the
pinned tree's own checker rather than this module's preference: `xtask/src/lint_narrative.rs:232-241`
calls the index *"the only file under `TREE` that is not a page"*, and its own vacuity guard at
`:486-491` says *"a tree holding only its own routing table holds nothing to check"*. Treating the
index as a page would have made the two checkers disagree about what a page is, in the same tree.

**One stated blind spot added rather than discovered later.** The declaration parser skips fenced
regions, so a page that *teaches* the declaration form — band 00 does exactly this — is not read as
declaring one. The cost is that a second declaration hidden inside a fence is invisible to the
check. It is written into the parser's own doc comment as a limit with band 40's non-author walk
named as its instrument, rather than left for a reader to find by experiment.

**The 100-character budget was breached and routed, not amended.** The longest real problem line
measured **135 characters**. `_design.md` `## Mock` finding 1 predicted 112 against an inherited
111, so the direction was known; the size was not. The yield order was applied as written — the
location and the repair pointer were never candidates, and AC-004 requires the enumerated set in
the message — so the number is recorded in `_ledger.md` and the design is untouched.
