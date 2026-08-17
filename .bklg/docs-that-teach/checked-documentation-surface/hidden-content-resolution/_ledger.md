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
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::every_hidden_marker_is_reported_by_file_and_line + ::tests::a_marker_and_a_fence_problem_are_reported_in_one_run; gate: `cargo xtask narrative` and `cargo xtask ci --fast` green over the real tree"
- id: AC-002
  criterion: "GIVEN the named wrong implementation this story exists to reject — \"a `<details>` block whose inner claim is broken and the gate stays green\" (`hidden-content-resolution/discover.md:55`) — WHEN the `_design.md` `## The doctest` fixture page is wrapped in a disclosure block around its scope band, THEN the checker reports it by file and line, and WHEN that wrapper alone is removed, THEN the same page produces no problem at all. Both halves are required: without the failing half the rule is decorative (`_design.md` `## The doctest`, closing paragraph; CLAUDE.md, \"A rule that no adapter can fail is decorative\"), and without the clean half the rule cannot be distinguished from one that rejects every page"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_wrapped_fixture_page_fails_by_file_and_line + ::tests::the_unwrapped_fixture_page_is_clean"
- id: AC-003
  criterion: "GIVEN that this repository has already been bitten by \"elaborate spellings\" of an opt-out getting past a naive match (`project.md`, and the risk row \"`ignore` returns by the back door\"), WHEN an author reaches for a disclosure marker in any spelling a renderer accepts — `<Details>`, `<DETAILS open>`, `{{#tabs`, a `{{#tab ` directive, `{{#endtabs`, an ```admonish fence or an `<!-- tab` comment — or hides one inside a fenced block or inside `docs/README.md` itself, THEN every one is a problem, because HTML tag names are case-insensitive, a marker quoted in a fence still renders as a page telling a reader to fold something, and there is no file-level, page-level or fence-level exemption anywhere in the design (`_design.md` `## Shape decision` row 3)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::marker_matching_is_ascii_case_insensitive + ::tests::a_marker_inside_a_text_fence_is_still_reported + ::tests::the_real_docs_readme_is_clean"
- id: AC-004
  criterion: "GIVEN a future maintainer who finds one token inconvenient and deletes it, WHEN `HIDDEN_MARKERS` is shrunk, re-ordered or has a token upper-cased, THEN `cargo test -p xtask` fails with a message naming *which* token moved rather than only that two numbers disagree — so DT-7, which a human closed on 2026-08-17 with no conditions (`_design.md` `## Sign-off`), cannot be reopened by an edit to a `const`, only by a new design record (`_design.md` `## Visibility and stability`)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_hidden_marker_set_matches_its_hand_written_intention (RS-81-5, naming which token moved in both directions) + ::tests::every_hidden_marker_token_is_lowercase"
- id: AC-005
  criterion: "GIVEN the contributor at the gate, whose whole task on a red run is deciding where to look first, WHEN a run reports hidden-marker problems — one, or forty — THEN each is one stderr line composed from the existing problem-line primitive: two-space indent, `{path}:{line}` first, an em dash, then \"`<token>` is a hidden panel; DT-7 forbids it in docs\"; the list is in source order, path then line, interleaved with the walk's other problem kinds; it is never truncated with \"… and N more\"; the single terminal `bail!(\"{n} problem(s) in docs\")` is the last line; and no banner, step, subcommand, spinner, progress line or success chatter of this story's own is added anywhere. AND the module's rustdoc leads with what this check does not verify, before what it does"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::the_problem_line_matches_the_designed_message_form (exact string) + ::tests::forty_occurrences_print_forty_lines_untruncated + ::tests::problems_are_reported_in_source_order; static: `cargo xtask lint-constitution` and `cargo test --locked -p xtask --doc` pass"
- id: AC-006
  criterion: "GIVEN persona 1 and persona 3, who will one day read this material and must not be told a green gate means a page teaches (`project.md` DoD item 8), and GIVEN HS-P0021, which owns DT-8's aside/constraint line (`initiative.md:492`), WHEN this story's diff is reviewed, THEN no message, doc comment, ledger row or report line asserts that the surface proves comprehension, no badge or \"verified\" mark exists (`_design.md` anti-pattern 9), and THEN no allowance list, environment variable, `#[cfg]`, feature or commented-out hook exists by which a hidden marker could be permitted — the absence is the decision, and a future non-normative use petitions in its own change with its own falsification (`_design.md` D2, `## Open questions` item 3)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs — the bin-crate narrative checker module declared at :64-70 alongside `mod lint_constitution;`, reached from its `REQUIRED` entry, dispatch arm and `lint_steps()` membership; the marker scan is called from that module's existing fence walk and reported by its single `bail!`"
  verifying_test: "xtask/src/lint_narrative.rs::tests::no_input_makes_a_hidden_marker_pass; plus the reviewed `rg` sweep over this story's diff for allowance/`cfg(`/`env::var` escapes and for teachability claims (`project.md` DoD item 8)"
```
