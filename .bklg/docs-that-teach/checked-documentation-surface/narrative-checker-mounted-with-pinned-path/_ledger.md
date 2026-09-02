---
item: HS-S0138
stage: implement
created: "2026-08-17T13:17:03.389Z"
updated: "2026-08-17T13:17:03.389Z"
---

# Acceptance ledger — The checker is mounted, the tree is pinned by constant, and no page is an orphan

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes for the implementer, both drawn from `spec.md` rather than added here:

- `xtask/src/lint_narrative.rs` does not exist yet — it is this story's new module, and its
  `#[cfg(test)] mod tests` is authored in the house shape at
  `xtask/src/lint_constitution.rs:827-878`. The `verifying_test` values below are therefore the
  real paths those tests must land at, not paths that already resolve.
- AC-006, AC-007 and AC-008 are the **composition** criteria taken from the signed-off
  `_design.md`. A checker that satisfies AC-001 through AC-005 while printing a raw `anyhow` chain,
  or nothing at all on success, satisfies every functional assertion and fails this story.

```yaml
- id: AC-001
  criterion: "GIVEN a contributor who has moved or renamed the narrative tree — the way `docs/` itself was once reorganised (`docs/README.md:8-10`) — and has not touched `xtask/src/`, WHEN they run `cargo xtask ci`, THEN the run fails with a message naming the path the gate expected, so they learn the tree is pinned by a constant rather than discovering weeks later that nothing was being read. A message reporting *zero pages* instead of a missing directory does not satisfy this."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:96 (`const TREE`), :179-224 (`pages`/`collect`, `?`-propagated through `.with_context()` naming the expected path); test `lint_narrative::tests::a_missing_tree_is_an_error_naming_the_pinned_path` - RED: returned `Ok([])` over a missing root; GREEN: the `{:#}` chain contains `docs`"
  mount_point: "xtask/src/main.rs (the `REQUIRED` Step at :105, reached from `cargo xtask ci`)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: the missing-tree error chain names the pinned path"
- id: AC-002
  criterion: "GIVEN a contributor whose change has emptied the narrative tree — every page moved, deleted, or relocated under a directory the constant no longer names — WHEN the gate runs, THEN the step fails before any check executes, saying the tree holds no pages and that every check below would be vacuous, so a green run can never mean \"there was nothing to read\". A pinned constant with no vacuity guard is the named wrong implementation this row rejects."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:244-249 (`guard_not_vacuous`, `bail!('{TREE} holds no pages, so every check below is vacuous')`), called at :348 before any check runs; tests `an_empty_tree_is_a_hard_error`, `a_tree_holding_only_its_index_is_still_vacuous`, `a_tree_with_one_page_passes_the_guard`"
  mount_point: "xtask/src/main.rs (the `REQUIRED` Step at :105, reached from `cargo xtask ci`)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: the vacuity guard bails on an empty page slice and passes a one-page slice"
- id: AC-003
  criterion: "GIVEN a contributor who has added a markdown page to the tree and forgotten to register it in the harness — the silent-pass shape that survives milestone 1 because `cfg(doctest)` makes an unregistered page produce no failure at all, only silence (`xtask/src/constitution.rs:20-25`) — WHEN they run `cargo xtask narrative` or the gate, THEN the page is named as a problem, saying its examples are never compiled, so the page cannot ship unchecked."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:285-302 (the forward half of `check_registration`); tests `a_page_the_harness_does_not_include_is_a_problem` and `a_page_the_harness_does_not_declare_is_a_distinct_problem` - the two halves kept separate - plus `the_index_is_not_expected_to_be_registered`"
  mount_point: "xtask/src/main.rs (the `narrative` dispatch arm mirroring :689, and the `REQUIRED` Step at :105)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: a page with no `include_str!` is a problem, and a page with no `mod {module} {` is a distinct problem"
- id: AC-004
  criterion: "GIVEN a contributor who has renamed or deleted a page and left its `mod` behind in the harness, WHEN the gate runs, THEN the stale registration is reported as a problem naming it — because `cfg(doctest)` hides a leftover module from every step but `cargo test` (`xtask/src/lint_constitution.rs:445-446`), and a reviewer reading the harness cannot tell a live registration from a dead one. A checker implementing only AC-003's direction does not satisfy this row."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:304-315 (the reverse half of `check_registration`); tests `a_registration_naming_no_page_is_a_problem` (RED: 0 problems) and its companion `the_same_harness_with_the_page_present_yields_no_problem`; `the_module_name_is_one_derivation_for_both_directions` pins the shared derivation at :164-168"
  mount_point: "xtask/src/main.rs (the `narrative` dispatch arm mirroring :689, and the `REQUIRED` Step at :105)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: a harness `mod` naming no existing page is a problem; the same harness with the page present yields none"
- id: AC-005
  criterion: "GIVEN a contributor on a clean clone with no tool installed and nothing to remember, WHEN they run any of the four invocation paths this repository actually wires — `cargo xtask ci`, `cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask affected --base main` — THEN the narrative check runs on every one of them, under its own claim-sentence banner `=== every narrative page is checked ===`, and is discoverable by name from `cargo xtask` with no arguments. A step reachable from `ci` but invisible to `--fast` or to the story grain is the failure RS-80-1 names, and `.redkiln/config.yaml:40` makes the story grain the gate every later story in this initiative is actually held to."
  satisfied: true
  evidence: "Five mount sites - xtask/src/main.rs:66 (`mod lint_narrative;`), :526-553 (the `REQUIRED` Step, `probe: None`, `--locked`), :746 (dispatch arm), :834-838 (`print_help`), :882 (`lint_steps`) - plus xtask/src/affected.rs:125-131 (the unconditional file-reading list). Observed: `cargo xtask ci --fast` printed `=== every narrative page is checked ===` then `all required checks passed`; `cargo xtask lints` selected it by name; `cargo xtask affected --base main` ran it in the file-reading block and then `affected gate passed`; `cargo xtask narrative` exits 0 standalone. Tests: `the_checker_step_is_required_and_unprobed`, `the_checker_step_follows_the_narrative_compile_step`, `the_checker_step_passes_locked_and_names_its_subcommand`, `the_gate_can_select_the_checker_step_by_name`, `the_checker_step_is_a_lint_step`, `the_subcommand_is_dispatched_and_listed_in_the_help`, `the_checker_joins_the_unconditional_file_reading_list`"
  mount_point: "xtask/src/main.rs — five sites: mod list :65, REQUIRED :105, dispatch arm :689, print_help :718, lint_steps :799; plus xtask/src/affected.rs:118-125"
  verifying_test: "gate-integration: `cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask affected --base main`, `cargo xtask narrative` — each run and the banner/selection observed; xtask/src/main.rs:816-826 (`steps_named`) panics on a drifted name"
- id: AC-006
  criterion: "GIVEN a reviewer reading a failing run in a CI log they cannot re-run, with several pages broken at once, WHEN the checker fails, THEN they get every problem in one run — indented two spaces, in source order (path then line), each line beginning with `{path}:{line}` before an em dash and its message, with the count and the directory last — and never a truncated list. Fail-fast turns one review cycle into six (`_decomposition.md:218-219`); a `… and N more` is `_design.md` anti-pattern 8; a first visual row that does not begin with `path:line` is anti-pattern 7."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:321-327 (`problems` accumulates and never short-circuits) and :356-361 (one `eprintln!('  {problem}')` per problem, then `bail!('{} problem(s) in {TREE}')`); tests `every_problem_is_reported_in_source_order_and_none_is_elided` (four problems, all returned, tree before harness, nothing elided) and `a_problem_is_a_composed_line` (the composed `{path}:{line} - {message}` form with its em dash asserted)"
  mount_point: "xtask/src/main.rs (banner at :864, step failure at :887, `xtask failed:` at :712 — the composed stderr surface `gate-narrative-checker-step`)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: three distinct problems all returned, in source order, each matching `  {path}:{line} — {message}`, nothing elided; read against _design.md `## Composition`"
- id: AC-007
  criterion: "GIVEN a contributor watching a green gate scroll past, WHEN the narrative step passes, THEN it prints exactly one line — `  {n} pages, all consistent` — and nothing else: no spinner, no dot ticker, no per-page progress line, and never the `skipped:` line. Zero output is indistinguishable from a check that did not run, which is precisely `RUNBOOK.md:918-925`'s failure; ten lines trains people to skip the output (`_design.md` `## Transience policy`). The banner printed at `xtask/src/main.rs:864` is the loading state and no second one is added."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:334-337 (`summary`), printed once at :353; test `a_green_run_prints_exactly_one_summary_line` asserts one line, `  1 pages, all consistent`, and no `skipped`. Observed: `cargo xtask narrative` prints exactly that line; the `skipped:` line is structurally unreachable because `probe: None` (xtask/src/main.rs:552)"
  mount_point: "xtask/src/main.rs (the `REQUIRED` Step at :105 with `probe: None`, banner at :864)"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: the success path emits exactly one line whose count equals the enumerated pages; plus `cargo xtask narrative` observed printing one summary line and no `skipped:` line"
- id: AC-008
  criterion: "GIVEN a contributor who has nested a page one directory level too deep, or given it a long filename, WHEN the gate runs, THEN that page is reported as a problem *before any other line is emitted*, because a page whose repo-relative path exceeds **32 characters** — or that sits at a third directory level under `TREE` — pushes the location off the first visual row of an 80-column log once the 16-character `xtask\\src\\../../` doctest prefix is counted, starving the surface a reviewer reads (`_design.md` finding 3's disposition; `## Density budget`; anti-pattern 11). The page-length and H1 budgets deliberately stay review rules."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:118 (`PATH_BUDGET = 32`, with the 48-less-16 arithmetic on the const) and :258-281 (`check_paths`, called first at :323); test `the_path_budget_is_enforced_at_its_four_corners` - 31 characters passes, 33 is a problem, `docs/a/b/c.md` is a problem, `docs/adapters/sqlite.md` passes"
  mount_point: "xtask/src/lint_narrative.rs (the pinning check, ordered before any problem line is emitted), reached through xtask/src/main.rs:105"
  verifying_test: "xtask/src/lint_narrative.rs — #[cfg(test)] mod tests: a 31-character path passes, a 33-character path is a problem, `docs/a/b/c.md` is a problem, `docs/adapters/sqlite.md` passes"
- id: AC-009
  criterion: "GIVEN a contributor or a downstream project reading this new check for the first time, WHEN they open the module, THEN the *first* thing in its docs is `# What this does not verify`, stating at minimum that registration proves a page is compiled and not that it is correct, and that a compile failure names the harness rather than the markdown; the module also states in one paragraph why it joins `affected::run`'s unconditional list when `lint-constitution` does not; and nowhere in the module, its output, or `docs/README.md` does anything claim the surface proves a page teaches. A check whose limits are undocumented is read as a guarantee (`xtask/src/lint_constitution.rs:9-13`, RS-81-1), and a green step read as evidence of teachability is the initiative's top-ranked risk (`project.md`, risk table)."
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:10-42 (`# What this does not verify`, the first heading in the module docs) and :61-75 (the `affected::run` divergence, stated as a convention and naming `lint-constitution`); tests `the_module_states_its_limits_first`, `the_module_states_its_own_limits_and_its_one_divergence`, `nothing_in_the_module_claims_a_page_teaches`. `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` both pass; docs/README.md:39-42 adds the second-reader sentence and claims nothing about teaching"
  mount_point: "xtask/src/lint_narrative.rs module docs (declared from xtask/src/main.rs:65), plus docs/README.md:25-29"
  verifying_test: "compile: `cargo test --locked -p xtask --doc` and `cargo xtask lint-constitution` both pass over the new module docs; reviewed for section-first ordering, the `affected` divergence paragraph, and the absence of any teaching claim (project DoD 8, _design.md anti-pattern 9)"
```
