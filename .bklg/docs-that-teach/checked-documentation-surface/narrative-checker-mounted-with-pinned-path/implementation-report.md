---
item: "HS-S0138"
stage: implement
created: "2026-08-17T23:40:00.000Z"
updated: "2026-08-17T23:40:00.000Z"
---

# Implementation Report — The checker is mounted, the tree is pinned by constant, and no page is an orphan

## TDD Evidence

The whole `#[cfg(test)] mod tests` was written first, into a `xtask/src/lint_narrative.rs`
whose production half was the *named wrong implementations* rather than empty bodies — a
`pages()` that swallowed the read and returned `Ok(vec![])` (AC-001's "reports zero pages"),
a `guard_not_vacuous()` that always returned `Ok(())` (AC-002's "pinned constant with no
vacuity guard"), an empty `check_registration` (a checker that reports nothing in either
direction) and an empty `check_paths`. That is what makes the red run an *assertion* failure
per row rather than a compile error, which a brand-new module would otherwise produce.

Red run: `cargo test --locked -p xtask --bin xtask lint_narrative::` →
**5 passed; 20 failed**. Green run, after the bodies and the five mount sites landed:
**25 passed; 0 failed**.

The five that were green from the start are the *negative* halves of directional pairs, and
they are named here rather than hidden: `a_tree_with_one_page_passes_the_guard`,
`the_index_is_not_expected_to_be_registered`,
`the_same_harness_with_the_page_present_yields_no_problem`,
`the_module_name_is_one_derivation_for_both_directions` and `the_step_is_named_as_a_claim`.
Each exists to fail a *future* over-broad implementation (a guard that always bails, a
registration check that demands the index, a reverse sweep that fires on a live module), and
none of them is evidence on its own — the spec says as much for the same shape in the
slice-mate.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `a_missing_tree_is_an_error_naming_the_pinned_path` | RED: `a missing tree must be an error, never an empty page list` — the wrong implementation returned `Ok([])`. GREEN once the `read_dir` was `?`-propagated with `.with_context(\|\| format!("reading {here}"))`; the `{:#}` chain now contains `docs`. |
| AC-002 | `an_empty_tree_is_a_hard_error`, `a_tree_holding_only_its_index_is_still_vacuous` | RED: both returned `Ok(())`. GREEN on `bail!("{TREE} holds no pages, so every check below is vacuous")`. The index test is the one that rejects counting *files*: a tree holding only `docs/README.md` holds nothing the harness registers. |
| AC-003 | `a_page_the_harness_does_not_include_is_a_problem`, `a_page_the_harness_does_not_declare_is_a_distinct_problem` | RED: `assertion left == right failed: got: []` on both — the checker reported nothing. GREEN as two separate problems, kept separate exactly as `check_harness` keeps them. |
| AC-004 | `a_registration_naming_no_page_is_a_problem` | RED: 0 problems over a harness carrying `mod renamed_away {`. GREEN on the reverse loop. This direction is unreachable from a real gate run once tree and harness agree, which is why it is driven by a harness string. |
| AC-005 | `the_checker_step_is_required_and_unprobed`, `the_checker_step_follows_the_narrative_compile_step`, `the_checker_step_passes_locked_and_names_its_subcommand`, `the_gate_can_select_the_checker_step_by_name`, `the_checker_step_is_a_lint_step`, `the_subcommand_is_dispatched_and_listed_in_the_help`, `the_checker_joins_the_unconditional_file_reading_list` | RED: `step_index` and `steps_named` both panicked with `REQUIRED must contain the \`every narrative page is checked\` step` — which is EC-005's intended failure, observed. GREEN after the five mount sites plus the `affected.rs` call. Backed by four recorded runs, under Gates. |
| AC-006 | `every_problem_is_reported_in_source_order_and_none_is_elided`, `a_problem_is_a_composed_line` | RED: `got: []` and `no em dash separator`. GREEN with four problems returned from one call, in source order, none containing `more`, and the location/em-dash/message split asserted. |
| AC-007 | `a_green_run_prints_exactly_one_summary_line` | RED: `summary()` returned `""`. GREEN on `  1 pages, all consistent` — one line, count equal to the pages enumerated, no `skipped`. |
| AC-008 | `the_path_budget_is_enforced_at_its_four_corners` | RED: `docs/aaaaaaaaaaaaaaaaaaaaaaaaa.md should yield 1 problem(s), got: []`. GREEN at all four corners: 31 passes, 33 fails, `docs/a/b/c.md` fails, `docs/adapters/sqlite.md` passes. |
| AC-009 | `the_module_states_its_limits_first`, `the_module_states_its_own_limits_and_its_one_divergence`, `nothing_in_the_module_claims_a_page_teaches` | RED: `this module's docs carry no headings at all`. GREEN once `# What this does not verify` opened the docs, the `affected::run` divergence paragraph landed, and the diff carried no `verified`/`badge`/`shield`. The third test reads only the production half of the file, because `include_str!` of the whole module otherwise trips on its own assertion literals — a real defect found in the first green run and fixed in the test, not by weakening it. |

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Narrative checker mounted with pinned path |

The row records the checkpoint **as first written**. A commit cannot contain its own SHA, so
the reachable commit is reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_narrative.rs` | **New**, ~380 production lines plus tests. Module docs opening with `# What this does not verify` (`:10-42`), then the module/subcommand asymmetry (`:44-59`), the `affected::run` divergence (`:61-75`) and the constants-as-contracts note (`:77-83`). `TREE`, `INDEX`, `HARNESS`, `PATH_BUDGET`, `STEP` (`:92-127`). `Page` + `Page::new` (`:129-158`) and the one pure `module_name` (`:164-168`). I/O at two edges only: `pages`/`collect` (`:179-224`) and the harness read inside `run` (`:350-351`). Pure checks: `guard_not_vacuous` (`:244`), `check_paths` (`:258`), `check_registration` (`:285`), composed by `problems` (`:321`) and `summary` (`:334`). |
| `xtask/src/main.rs` | The five mount sites: `mod lint_narrative;` (`:66`), the `REQUIRED` Step (`:526-553`), the dispatch arm (`:746`), five `print_help` lines (`:834-838`) and `lint_steps` membership (`:882`). |
| `xtask/src/affected.rs` | One call added to the unconditional file-reading list (`:125-131`), with the divergence from `lint-constitution`'s absence noted at the call site and argued in full in the new module's docs. The selection arm at `:212-260` is untouched. |
| `xtask/src/narrative.rs` | **Module names only were in scope and none needed to move** — milestone 1's `append_conditions` agrees with the derivation this story pins. What did move is the *prose* that states the derivation: it claimed an `NN-` ordering prefix is stripped and said nothing about nested pages, which is a second spelling of a rule this story makes computable. Re-pointed at `xtask::lint_narrative::module_name` rather than restated. |
| `docs/README.md` | One paragraph (`:39-42`) naming `cargo xtask narrative` as the tree's second gate-side reader. Milestone 1 had not named the subcommand, so the sentence was owed rather than skipped. |
| `standards/rust/51-…`, `52-…`, `70-…`, `80-the-gate.md` | Nine `xtask/src/main.rs:NNN` citations re-pointed. Line numbers only; no rule, prose or anchor text changed. See Notes. |
| `.bklg/…/narrative-checker-mounted-with-pinned-path/` | `_ledger.md` flipped with evidence; this report and `report.md`. |

## Gates

Affected package: `xtask` only.

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` | 25 passed; 0 failed (red: 5 passed, 20 failed) |
| `cargo clippy --locked -p xtask --all-targets --all-features -- -D warnings` | clean. Two findings were fixed rather than allowed: `case_sensitive_file_extension_comparisons` became an explicit `is_markdown` helper (`:230`), and one `doc_markdown` backtick. No new `#![allow]`. |
| `cargo test --locked -p xtask` | 231 passed; 0 failed; 3 ignored, across the four targets — including `affected::tests` unchanged and green, so adding the checker to the unconditional list did not widen `INERT`. |
| `cargo fmt --all --check` | clean. Run last, after the clippy fixes. |
| `cargo run --locked --quiet -p xtask -- narrative` | `  1 pages, all consistent`, exit 0. One line, no banner of its own, no progress output. |
| `cargo run --locked --quiet -p xtask -- lints` | Selects the step by name: `=== every narrative page is checked ===` / `  1 pages, all consistent`. |
| `cargo run --locked --quiet -p xtask -- affected --base main` | The checker runs inside `=== the file-reading checks ===`, before any package selection; then `xtask` selected, then `affected gate passed`. |
| `cargo run --locked --quiet -p xtask -- ci --fast` | `all required checks passed (--fast: 4 optional step(s) not run)`. The banner appears after `=== the constitution's examples compile ===`; no `skipped:` line anywhere. **This is the story's merge bar.** |

## Notes

**Two deviations, both forced, both recorded rather than absorbed.**

**1. The step sits after `the constitution's examples compile`, not immediately after the
narrative compile step.** The spec asked for the latter. It is not available: milestone 1's
`narrative_doctests::tests::the_narrative_step_precedes_the_constitution_step` asserts
`step_index(STEP) + 1 == step_index("the constitution's examples compile")`, and that
adjacency carries a real argument — the constitution's compile step is unfiltered and
compiles the narrative pages too, and `run_steps` bails at the first failure, so the ordering
is the whole of what keeps a broken narrative fence under the narrative banner. Inserting
between the two would have required weakening a sibling story's assertion from `==` to `<` to
satisfy a placement preference. What the spec actually wants from the placement —
*compile-then-check*, so the banner alone says which half failed — survives the move intact,
and this story's own `the_checker_step_follows_the_narrative_compile_step` pins it. The
reasoning is written at the `Step` itself so the next reader does not re-litigate it.

**2. Nine line-number citations in `standards/rust/` were re-pointed**, which the PR boundary
lists under "explicitly not in this PR". Adding ~40 lines to `xtask/src/main.rs` moved every
anchor below the insertion past `lint_constitution`'s 10-line `ANCHOR_SLACK`, and
`cargo xtask lints` failed on nine citations before anything else could be observed. The
change is line numbers only — no rule, no prose, no anchor text — and it is the same
mechanical consequence milestone 1 absorbed in `7020c4c`, which touched the same four atoms
for the same reason. Not doing it would have left the merge bar red.

**Three things the spec asked for that were re-derived rather than copied.**

*Source order* is defined explicitly, in the module's own `problems` doc: the tree first —
the pinning check before any other line, per AC-008 — then each page, then the files that
register the tree. Every problem is still one composed line and the count is still last. The
alternative, sorting composed strings, puts `:100` before `:12`.

*`docs/README.md` is a page for some checks and not for others.* It is enumerated (so the
slice-mates' fence and marker walks reach it) and carries an `index` flag, which excludes it
from the registration check — registering the routing table would turn its links into
intra-doc links — and from the vacuity count, so a tree holding only its own index is still
vacuous. The shape is `lint_constitution`'s `ROUTER`, one directory over.

*Page naming was left free by the architecture brief and pinned by this spec*: the
tree-relative path minus `.md`, with `/` and `-` mapped to `_`. It is computed in exactly one
place and both directions of the orphan check compare that string, which is why
`xtask/src/narrative.rs`'s prose statement of the same rule had to stop being a second
spelling of it.

**What this story did not claim.** No end-to-end falsification: project AC-003 requires
observing `cargo xtask ci` itself fail on a deliberately broken page and then recover, and
`observed-failure-falsification` owns it. The unit tests here are not evidence for it. The
six-item contents of the limits section remain
`documented-blind-spots-and-their-proofs`'; this story wrote only the limits its own checks
create. No ADR was written, and none is owed.
