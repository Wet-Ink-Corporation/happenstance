---
item: HS-S0140
stage: implement
created: "2026-08-17T13:16:03.191Z"
updated: "2026-08-17T13:16:03.191Z"
---

# Acceptance ledger — Hidden content is inside the check, or absent

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two notes for whoever flips these rows.

**The mount point is the same for all six, and that is the point.** Every criterion is reachable
through `xtask/src/main.rs`'s bin-crate narrative checker module — declared at `:64-70` alongside
`mod lint_constitution;`, reached from its `REQUIRED` entry, its dispatch arm and its `lint_steps()`
membership — and specifically from *inside* the fence walk `fence-discipline-and-allowance-list`
wrote, reported by that walk's single `bail!`. A row whose evidence cites a second step, a second
banner or a second `bail!` is not satisfied; it is a different design.

**The verifying tests live in `xtask/src/lint_narrative.rs`'s `#[cfg(test)] mod tests`**, the module
`narrative-checker-mounted-with-pinned-path` creates. Test names below are the intended names, not
sacred: rename freely, but cite the name that actually exists.

```yaml
- id: AC-001
  criterion: "GIVEN persona 2, the adapter author, whose goal is understanding *why* the port is shaped as it is and who therefore meets the per-adapter fanout as the natural tab strip (`personas-and-journeys.md:148-166`), WHEN any contributor writes scoped divergence into a page under `docs/` using a disclosure marker — a fold, a tab directive or a collapsed admonition — THEN `cargo xtask narrative` and `cargo xtask ci` fail, naming the file and the line, from inside the fence walk that already exists rather than from a step of this story's own; and THEN the contributor's remaining problems on other pages are reported in the same run, because the marker problem is pushed onto that walk's existing `Vec<String>` and counted by its single `bail!`"
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:727-740 (`check_hidden_markers`), called from the existing fence walk at :750 and reported by the module's single `bail!` at :420; tests `every_hidden_marker_is_reported_once_per_occurrence` (all seven tokens, one problem each, `{path}:{line}` prefix), `two_markers_on_one_line_are_two_problems` (EC-004), `marker_and_fence_problems_arrive_in_one_list_in_source_order`. Gate: `cargo xtask narrative` and `cargo xtask ci --fast` both exit 0 over the real tree, printing `  1 pages, all consistent` - mounted and not vacuous. Observed failing on a temporary `docs/folded.md`: `  docs/folded.md:7 - `<details` is a hidden panel; DT-7 forbids it in docs`"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::every_hidden_marker_is_reported_by_file_and_line + ::tests::a_marker_and_a_fence_problem_are_reported_in_one_run; gate: `cargo xtask narrative` and `cargo xtask ci --fast` green over the real tree"
- id: AC-002
  criterion: "GIVEN the named wrong implementation this story exists to reject — \"a `<details>` block whose inner claim is broken and the gate stays green\" (`hidden-content-resolution/discover.md:55`) — WHEN the `_design.md` `## The doctest` fixture page is wrapped in a disclosure block around its scope band, THEN the checker reports it by file and line, and WHEN that wrapper alone is removed, THEN the same page produces no problem at all. Both halves are required: without the failing half the rule is decorative (`_design.md` `## The doctest`, closing paragraph; CLAUDE.md, \"A rule that no adapter can fail is decorative\"), and without the clean half the rule cannot be distinguished from one that rejects every page"
  satisfied: true
  evidence: "The `_design.md` `## The doctest` fixture page held as `FIXTURE_PAGE` in `#[cfg(test)] mod tests`, with the wrapped form derived from it at runtime rather than hand-copied; tests `the_wrapped_fixture_page_fails_by_file_and_line` (exactly two problems, the `<details` and `<summary` lines, each naming its own line) and `the_same_fixture_page_without_its_wrapper_is_clean` (zero). The fixture is never a file under `docs/`: a committed wrapper would fail `cargo xtask ci` forever, which is exactly what makes it the named wrong implementation"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_wrapped_fixture_page_fails_by_file_and_line + ::tests::the_unwrapped_fixture_page_is_clean"
- id: AC-003
  criterion: "GIVEN that this repository has already been bitten by \"elaborate spellings\" of an opt-out getting past a naive match (`project.md`, and the risk row \"`ignore` returns by the back door\"), WHEN an author reaches for a disclosure marker in any spelling a renderer accepts — `<Details>`, `<DETAILS open>`, `{{#tabs`, a `{{#tab ` directive, `{{#endtabs`, an ```admonish fence or an `<!-- tab` comment — or hides one inside a fenced block or inside `docs/README.md` itself, THEN every one is a problem, because HTML tag names are case-insensitive, a marker quoted in a fence still renders as a page telling a reader to fold something, and there is no file-level, page-level or fence-level exemption anywhere in the design (`_design.md` `## Shape decision` row 3)"
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:728-729 (`to_ascii_lowercase` per line, then a match per token) and the line-based scan over the whole page at :728; tests `a_hidden_marker_is_matched_whatever_its_case` (`<details>`, `<Details>`, `<DETAILS open>`, `{{#TABS}}`), `a_marker_inside_a_fence_is_still_reported` (EC-003 - a fence-aware scan is the plausible refinement and it is refused), `the_real_index_carries_no_hidden_marker` (reads `docs/README.md`'s actual bytes). Gate: `cargo xtask narrative` green over the tree as it stands"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::marker_matching_is_ascii_case_insensitive + ::tests::a_marker_inside_a_text_fence_is_still_reported + ::tests::the_real_docs_readme_is_clean"
- id: AC-004
  criterion: "GIVEN a future maintainer who finds one token inconvenient and deletes it, WHEN `HIDDEN_MARKERS` is shrunk, re-ordered or has a token upper-cased, THEN `cargo test -p xtask` fails with a message naming *which* token moved rather than only that two numbers disagree — so DT-7, which a human closed on 2026-08-17 with no conditions (`_design.md` `## Sign-off`), cannot be reopened by an edit to a `const`, only by a new design record (`_design.md` `## Visibility and stability`)"
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:208-216 (the seven tokens) with the doc comment at :180-207 naming DT-7, citing `_design.md`, and saying that shrinking the set requires a new design record; tests `the_hidden_marker_set_is_pinned_to_the_design` - RS-81-5, comparing the const against a hand-written pin in both directions and naming the token that moved - and `every_hidden_marker_is_already_lowercase`, which is what makes the case-folding in AC-003 correct rather than accidentally correct. RED against a one-token `HIDDEN_MARKERS`"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_hidden_marker_set_matches_its_hand_written_intention (RS-81-5, naming which token moved in both directions) + ::tests::every_hidden_marker_token_is_lowercase"
- id: AC-005
  criterion: "GIVEN the contributor at the gate, whose whole task on a red run is deciding where to look first, WHEN a run reports hidden-marker problems — one, or forty — THEN each is one stderr line composed from the existing problem-line primitive: two-space indent, `{path}:{line}` first, an em dash, then \"`<token>` is a hidden panel; DT-7 forbids it in docs\"; the list is in source order, path then line, interleaved with the walk's other problem kinds; it is never truncated with \"… and N more\"; the single terminal `bail!(\"{n} problem(s) in docs\")` is the last line; and no banner, step, subcommand, spinner, progress line or success chatter of this story's own is added anywhere. AND the module's rustdoc leads with what this check does not verify, before what it does"
  satisfied: true
  evidence: "xtask/src/lint_narrative.rs:735-738 (the message, `_design.md` `## States` verbatim) composed into `  {path}:{line} - {message}` at :742-757; tests `a_marker_problem_is_the_composed_line_the_design_specifies` (exact-string, the whole line), `forty_markers_print_as_forty_lines` (40 lines, no elision token), `marker_problems_are_ordered_by_page_then_line`, `the_marker_scan_adds_no_step_or_banner_of_its_own` (the composition root mentions no marker at all - `REQUIRED` gains no entry and the dispatch gains no arm in this diff). Module docs lead with the three limits this check creates (:35-47), asserted by `the_module_docs_state_the_marker_scans_limits` to precede the first check. `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` both still pass"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_problem_line_matches_the_designed_message_form (exact string) + ::tests::forty_occurrences_print_forty_lines_untruncated + ::tests::problems_are_reported_in_source_order; static: `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` pass"
- id: AC-006
  criterion: "GIVEN persona 1 and persona 3, who will one day read this material and must not be told a green gate means a page teaches (`project.md` DoD item 8), and GIVEN HS-P0021, which owns DT-8's aside/constraint line (`initiative.md:492`), WHEN this story's diff is reviewed, THEN no message, doc comment, ledger row or report line asserts that the surface proves comprehension, no badge or \"verified\" mark exists (`_design.md` anti-pattern 9), and THEN no allowance list, environment variable, `#[cfg]`, feature or commented-out hook exists by which a hidden marker could be permitted — the absence is the decision, and a future non-normative use petitions in its own change with its own falsification (`_design.md` D2, `## Open questions` item 3)"
  satisfied: true
  evidence: "Tests `no_input_makes_a_hidden_marker_pass` - six shapes including a comment claiming an exemption, a page naming an allowance, a `text` fence and a four-backtick block, all still problems - and `the_marker_scan_has_no_allowance_environment_or_cfg_hook`, which reads the scan's own body and asserts it contains no `IGNORE_ALLOWANCES`, `env::var`, `cfg(` or `feature =`. `rg` over the diff finds no badge, tick, shield or teachability claim; `nothing_in_the_module_claims_a_page_teaches` gates the same thing mechanically over the module's production half and was RED once during this story, on the word `unverified` in a doc comment, which was reworded rather than the test weakened"
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::no_input_makes_a_hidden_marker_pass; plus the reviewed `rg` sweep over this story's diff for allowance/`cfg(`/`env::var` escapes and for teachability claims (`project.md` DoD item 8)"
```
