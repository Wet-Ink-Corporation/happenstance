---
item: "HS-S0138"
stage: report
created: "2026-08-17T23:40:00.000Z"
updated: "2026-08-17T23:40:00.000Z"
---

# Report — The checker is mounted, the tree is pinned by constant, and no page is an orphan

## Findings Ledger

Nine ACs, all satisfied by reachable behaviour with a `file:line` citation and a test or a
recorded run behind each. Nothing stubbed, nothing skipped, nothing deferred out of this
story's own set. Two deviations from the spec are recorded below as findings rather than
buried in a note.

| Finding | Evidence | Follow-up |
| ------- | -------- | --------- |
| **AC-001 — the tree is pinned by a constant, and a moved tree is an error naming the path.** `pages()` `?`-propagates `read_dir` through `.with_context()`, so the failure says which path was expected rather than reporting a tree with nothing in it. | `xtask/src/lint_narrative.rs:96` (`const TREE`), `:179-224`; `::tests::a_missing_tree_is_an_error_naming_the_pinned_path` (red: returned `Ok([])`) | None. The `docs/README.md:39-42` paragraph is the prose mirror and now names the second reader. |
| **AC-002 — an empty tree is a hard error before any check runs**, and a tree holding only its own index counts as empty. | `:244-249`, called at `:348`; `::tests::an_empty_tree_is_a_hard_error`, `a_tree_holding_only_its_index_is_still_vacuous`, `a_tree_with_one_page_passes_the_guard` | None. The guard is the half a pinned constant alone does not buy: without it a green run can mean "there was nothing to read". |
| **AC-003 — a page nobody registered is named, as two distinct problems.** A missing `include_str!` says the examples are never compiled; a missing `mod` says the failure's line number stops being relative to the page. | `:285-302`; `::tests::a_page_the_harness_does_not_include_is_a_problem`, `a_page_the_harness_does_not_declare_is_a_distinct_problem` (both red: `got: []`) | None. This is the silent-pass shape that survived milestone 1, because `cfg(doctest)` makes an unregistered page produce no failure at all. |
| **AC-004 — a registration nobody deleted is named**, which is the direction that earns the check its keep and is unreachable from a real gate run once tree and harness agree. | `:304-315`; `::tests::a_registration_naming_no_page_is_a_problem` (red: 0 problems) with its companion `the_same_harness_with_the_page_present_yields_no_problem` | None. A forward-only checker is the plausible wrong implementation CLAUDE.md's rule asks to be named; it is named, and it fails this row. |
| **AC-005 — the checker is reachable on all four wired paths, under its own claim-sentence banner.** Five mount sites plus the `affected` entry, all observed running rather than only asserted structurally. | `xtask/src/main.rs:66`, `:526-553`, `:746`, `:834-838`, `:882`; `xtask/src/affected.rs:125-131`. Runs: `ci --fast` (banner + `all required checks passed`), `lints` (selected by name), `affected --base main` (inside the file-reading block, then `affected gate passed`), `narrative` (exit 0). Seven tests listed in the implementation report | The step's **position** deviates: see the finding two rows below. |
| **AC-006 — every problem in one run, composed, in source order, never truncated.** One `Vec<String>`, no early return, one line per problem at a two-space indent, count and directory last. | `:321-327`, `:356-361`; `::tests::every_problem_is_reported_in_source_order_and_none_is_elided` (four problems from one call, tree before harness, nothing containing `more`), `a_problem_is_a_composed_line` | The ordering contract is stated on `problems` rather than left implicit, because the two problem families key on different files. Fence and marker problems join it in the slice-mates. |
| **AC-007 — a green run prints exactly one line**, whose count is the pages enumerated. No spinner, no per-page progress, and the `skipped:` line is structurally unreachable. | `:334-337`, printed at `:353`; `::tests::a_green_run_prints_exactly_one_summary_line`; observed `  1 pages, all consistent` from `cargo xtask narrative`. `probe: None` at `xtask/src/main.rs:552` | None. `RUNBOOK.md:918-925` is the precedent this row exists to not repeat. |
| **AC-008 — the density rule is a gate rule, at the pinning check.** A page path over 32 characters, or one nested a third directory level under `docs/`, is a problem before any other check reads a file. | `:118` (`PATH_BUDGET`, carrying the 48-less-16 arithmetic), `:258-281`, ordered first at `:323`; `::tests::the_path_budget_is_enforced_at_its_four_corners` | The page-length and H1 budgets deliberately stay review rules — gating them would take HS-P0021's job. |
| **AC-009 — the limits are the first thing in the module's docs, the one divergence is stated, and nothing claims teaching.** | `:10-42` (`# What this does not verify`), `:61-75` (the `affected::run` divergence, argued as a convention and naming `lint-constitution`); `::tests::the_module_states_its_limits_first`, `the_module_states_its_own_limits_and_its_one_divergence`, `nothing_in_the_module_claims_a_page_teaches`. `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` both pass | The six-item contents of the section are `documented-blind-spots-and-their-proofs`'. This story wrote only the limits its own checks create. |
| **Deviation — the step sits after `the constitution's examples compile`, not immediately after the narrative compile step.** The spec asked for the latter; milestone 1's `the_narrative_step_precedes_the_constitution_step` pins the two compile steps as adjacent, and that adjacency is what keeps a broken narrative fence under the narrative banner. Compile-then-check, which is what the placement was for, survives the move. | `xtask/src/main.rs:526-539` (the reasoning at the `Step`); `::tests::the_checker_step_follows_the_narrative_compile_step` | Reviewer decision, if any: weakening the sibling assertion from `==` to `<` would buy the literal placement and lose a real invariant. Recorded here so the trade is visible rather than discovered. |
| **Deviation — nine `xtask/src/main.rs:NNN` citations in `standards/rust/` were re-pointed.** Line numbers only. Adding the mount moved every anchor below it past `ANCHOR_SLACK`, and `cargo xtask lints` was red on nine citations until they moved. | `standards/rust/51-features-and-no-std.md:235`, `52-wasm32-and-target-cfg.md:108`, `70-rustdoc-obligations.md:148`, `80-the-gate.md:93`, `:176-177`, `:316`, `:371`; `git show --stat 7020c4c` shows milestone 1 absorbing the identical consequence | None. No rule, prose or anchor text moved; `cargo xtask lint-constitution` is green. |

**Mount point.** `xtask/src/main.rs` — the bin crate's composition root, at five sites:
`mod lint_narrative;` (`:66`), the `REQUIRED` `Step` with `probe: None` and `--locked`
(`:526-553`), the `Some("narrative")` dispatch arm (`:746`), five `print_help` lines
(`:834-838`), and `lint_steps` membership (`:882`). Plus `xtask/src/affected.rs:125-131`, the
unconditional file-reading list that makes the checker run on the story grain
`.redkiln/config.yaml:40` wires. The surface rendered is `gate-narrative-checker-step` in its
`pass`, `fail-one`, `fail-many`, `fail-empty-tree` and `fail-long-path` states;
`fail-hidden-marker` belongs to `hidden-content-resolution`.

**Deferred, and named so nothing is quietly claimed.** The fence walk, `IGNORE_ALLOWANCES`
and its reverse sweep (`fence-discipline-and-allowance-list`). `HIDDEN_MARKERS` and DT-7's
enforcement (`hidden-content-resolution`). Clause-id resolution and the frozen-MUST pin
(milestone `specification-pin`). The observed `cargo xtask ci` failure-then-recovery that
project AC-003 requires (`observed-failure-falsification`) — this story's unit tests are not
evidence for it and are not offered as such. The six-item limits list
(`documented-blind-spots-and-their-proofs`).

**No ADR is owed.** `_grounding.md`'s "Precedence and non-goals" verified that no Accepted
decision atom governs gate structure, documentation trees or fence compiling. The one
divergence from in-repo precedent — joining `affected::run`'s unconditional list when
`lint-constitution` does not — is a convention, discharged by a paragraph in the new module's
docs at `xtask/src/lint_narrative.rs:61-75`.
