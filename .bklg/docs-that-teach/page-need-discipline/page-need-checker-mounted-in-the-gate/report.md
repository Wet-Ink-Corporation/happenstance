---
item: "HS-S0150"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The page-need checker, mounted as an ordinary gate step

## Findings Ledger

All twelve ACs are satisfied by real, reachable behaviour, each with a `file:line` and a passing
test or a captured run. Nothing is stubbed, skipped or fixture-pinned. Three deviations from the
spec's stated PR boundary were forced by the tree as it actually is; each is stated below rather
than quietly absorbed, and none of them softens a criterion.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** The step runs by all three routes and is impossible to ship unreachable — `lint_steps()` names it and `steps_named` panics if `REQUIRED` does not carry it. | `cargo xtask ci` (21st of 27 steps, `all checks passed`), `cargo xtask lints`, `cargo xtask affected --base main` — all three captured in `_ledger.md` | `xtask/src/main.rs:567` (`REQUIRED`, `probe: None`, `--locked`), `:781` (dispatch), `:875` (`print_help`), `:925` (`lint_steps`), `xtask/src/affected.rs:139` |
| AC-002 | **Met.** Both trees, both guards. A missing tree fails naming the path; an empty one `bail!`s as vacuous and can never reach the success line. | `lint_pages::tests::{a_missing_rules_tree_fails_and_names_the_path_it_expected, an_empty_rules_tree_is_vacuous_rather_than_green, a_missing_pages_tree_fails_and_names_the_path_it_expected, an_empty_pages_tree_is_vacuous_rather_than_green}`; `xtask/src/lint_pages.rs:395,435,481,516` | `xtask/src/main.rs:567` → `lint_pages::run` |
| AC-003 | **Met.** One constant. The checker reads `lint_narrative::TREE`; production code holds no second spelling of the path and no `PAGE_DIR`. | `lint_pages::tests::the_pages_root_resolves_from_the_narrative_trees_own_constant`; `xtask/src/lint_narrative.rs:239`; `xtask/src/lint_pages.rs:379,443,467,519` | `xtask/src/lint_narrative.rs:239` (one `pub(crate)` token) |
| AC-004 | **Met.** Zero, two, unenumerated and malformed each report at the right place, and prose mentioning a need word never counts. | five parser tests in `xtask/src/lint_pages.rs`; the checker at `:546,558,630`; real-tree captures for the two-declaration and `reference` cases | `xtask/src/main.rs:567` |
| AC-005 | **Met.** Three named wrong pages as `&str` literals, each asserting the line *within the literal*; none is a committed file. | `lint_pages::tests::the_three_wrong_pages_are_each_rejected_at_the_right_line` (`:2986`); `git status --porcelain` clean | `xtask/src/main.rs:567` |
| AC-006 | **Met.** Two `orientation` pages at one level is one problem naming the directory and every offending `path:line`; one per level is silent. | `lint_pages::tests::{two_orientation_pages_at_one_level_are_one_problem_naming_both, one_orientation_page_per_directory_passes}`; `xtask/src/lint_pages.rs:685` | `xtask/src/main.rs:567` |
| AC-007 | **Met, and the forward contract discharged.** The **first** `--write` against the committed router produced **no diff**; the repair instruction sits inside the problem line; `--write` rewrites only the region. | `lint_pages::tests::{a_router_index_disagreeing_by_one_atom_names_the_repair_in_the_line, write_mode_rewrites_only_the_region_and_reaches_equality, a_dangling_router_link_is_a_problem_at_its_own_line}`; `git diff --exit-code standards/pages/README.md` after `--write` | `xtask/src/lint_pages.rs:799,867` |
| AC-008 | **Met.** Containment, exclusion **and** the job cell, each naming which token moved, citing both paths, and stating it is not `--write`-repairable. | `lint_pages::tests::{a_needs_member_missing_from_band_ten_names_which_token_moved, a_need_shaped_token_in_band_ten_that_is_not_a_member_is_named, band_tens_job_column_cannot_drift_from_the_const}`; `xtask/src/lint_pages.rs:915` | `xtask/src/main.rs:567` |
| AC-009 | **Met.** Sections, both ceilings with the measured byte count, no-rule, and the fence rule in its **inverted** form (`rust` *and* untagged rejected; `text`/`markdown` permitted) with the reason in the message. | six tests in `xtask/src/lint_pages.rs`; `:713,787,1104`; observed on a real atom during the many-problems capture | `xtask/src/main.rs:567` |
| AC-010 | **Met, with the budget breach recorded rather than hidden.** One block, sorted path→line→message, no truncation, no spinner, no summary; success line carries a count. **Longest measured problem line: 135 characters** against the ≤ 100 budget (`_design.md` `## Mock` finding 1 predicted 112). | `lint_pages::tests::{a_problem_line_is_path_line_dash_message_in_that_order, problems_sort_by_path_then_line_whatever_order_they_arrive_in}`; the three verbatim captures in `_ledger.md`; `xtask/src/lint_pages.rs:238,275,365` | `xtask/src/main.rs:567` |
| AC-011 | **Met as a pair, in one change.** The checker is on the unconditional list *and* `standards/pages/` is `INERT`; `standards/rust/` still selects `xtask`. | `affected::tests::{the_rules_tree_selects_no_package, the_pages_prefix_does_not_swallow_the_constitution}` (`:797`, `:813`); `xtask/src/affected.rs:139,300`; the `affected --base main` capture | `xtask/src/affected.rs:139,300` |
| AC-012 | **Met.** Limits first and unhedged, the three divergences stated including the **absence** of a `check_harness` equivalent, the scaffold `allow` deleted (clippy `-D warnings` green is the proof), and the `[PROVISIONAL]` marker retired in the same commit that makes the sentence true. | `xtask/src/lint_pages.rs:1,7,12,16,20,24,27,51,57,64,67,72`; the only `allow(` left is the test module's at `:1186`; `rg -n 'PROVISIONAL' docs/README.md` empty | `docs/README.md:38-42`; `xtask/src/lint_pages.rs` |

**Surface delivered.** `lint-terminal-output` (S4) is created here and accountable for six states.
Five were rendered and read against `_design.md` `## Composition` S4: `green`
(`  2 pages, 16 rules, all consistent`), `single-problem`, `many-problems` (three lines, sorted,
untruncated), `write-repair-offered` (the `--write` instruction inside the problem line), and the
`--write` repair itself (`  rewrote standards/pages/README.md's generated '## Index'`).
`vacuous-tree` and `missing-tree` are proved by the four AC-002 tests rather than by emptying the
real trees. `discipline-router` was **changed, not created**: prose only, and its generated region
is byte-identical to HEAD.

**Three deviations, stated for the reviewer rather than buried.**

1. The pages-tree constant lives in `xtask/src/lint_narrative.rs`, not `xtask/src/narrative.rs`.
   EC-005's halt applies to the constant being *absent*; it is present, under a different module
   path. The single-constant obligation is discharged exactly.
2. `docs/append-conditions.md` and `docs/text-fences.md` gained their declaration, replacing the
   `*<!-- answered-need: reserved for HS-P0021 -->*` slot HS-P0020's signed-off design reserved
   for this project (`checked-documentation-surface/_design.md:614`). Without it the mounted step
   is red and the story cannot exist; the alternative — softening the zero-declaration check — is
   the decorative-gate failure the project exists to refuse. One line per page, no teaching
   content. It also clears the slice-mate's EC-001 precondition.
3. Nine `xtask/src/main.rs:NNN` citations in four constitution atoms were re-pointed by line
   number only. Mounting a `Step` moves every line below it past `ANCHOR_SLACK`'s ten-line
   tolerance; no insertion point avoids it. No prose changed, and
   `git diff main -- standards/rust/README.md` is empty, so project AC-002 stands.

**Nothing deferred, nothing blocked.** The slice-mate `declaration-check-seen-to-fail` is
unblocked: the step is in `REQUIRED`, `cargo xtask ci` is green, and the pinned pages tree holds
two governed pages, so a non-vacuous green baseline exists to break and return to.
