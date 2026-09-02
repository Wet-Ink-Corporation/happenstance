---
item: "HS-S0146"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The closed need set and the declaration form, landed once

## Findings Ledger

The story's outcome as the review gate reads it. All fourteen ACs are satisfied by real,
reachable behaviour with a `file:line` citation and a passing test. Nothing is stubbed,
fixture-pinned or deferred; one AC's repo-state wording is honoured in substance and not
literally, and that is stated below rather than quietly flipped.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** One enumeration, four members in the design's render order, over `struct Need { token, job }` with both fields and the struct documented. | `xtask/src/lint_pages.rs:70,85`; `::tests::needs_holds_the_four_tokens_in_design_order` | `xtask/src/main.rs:67` |
| AC-002 | **Met.** `reference` is not a member, and band 10 states the *reason* for the subtraction, not the absence. | `xtask/src/lint_pages.rs:138`; `standards/pages/10-the-need-set.md:26-32`; `::tests::reference_is_not_a_member`, `::tests::band_ten_states_why_reference_was_subtracted` | `xtask/src/main.rs:67` |
| AC-003 | **Met, and seen to fail.** The ceiling is a compile-time fact. Three extra members produced `error[E0080]: evaluation panicked: the need set is closed…` at `xtask\src\lint_pages.rs:115:15`; reverted; green; `git status` clean. | `xtask/src/lint_pages.rs:110-116`; `cargo check -p xtask` | `xtask/src/main.rs:67` |
| AC-004 | **Met.** One pure membership accessor, exact and case-sensitive; the near misses are pinned by test, not by hope. | `xtask/src/lint_pages.rs:138`; `::tests::the_accessor_accepts_each_of_the_four_tokens`, `::tests::the_accessor_is_exact_and_case_sensitive` | `xtask/src/main.rs:67` |
| AC-005 | **Met.** Both path pins are `const`s introduced together; `ROUTER`'s absence is asserted as a scheduled obligation with the story that lifts it named in the doc comment. | `xtask/src/lint_pages.rs:123,131`; `::tests::rule_dir_holds_this_storys_two_atoms`, `::tests::router_is_not_created_by_this_story` | `xtask/src/main.rs:67` |
| AC-006 | **Met.** Limits first, six of them, item 1 unhedged and naming DR-07's reviewer procedure. | `xtask/src/lint_pages.rs:1-30`; `::tests::module_docs_open_with_what_this_does_not_verify` | `xtask/src/main.rs:67` |
| AC-007 | **Met.** The hosting assumption travels with the decision, in the design's own words, including *re-opens DR-05*. | `xtask/src/lint_pages.rs:40-49`; `_design.md:148-156`, `:718-721`; `::tests::module_docs_carry_the_hosting_assumption` | `xtask/src/main.rs:67` |
| AC-008 | **Met.** Mounted in the bin crate's module list in alphabetical position; exactly one module-level scoped `allow` naming the story that deletes it; `ci --fast` green. | `xtask/src/main.rs:67`; `xtask/src/lint_pages.rs:51-56` | `xtask/src/main.rs:67` |
| AC-009 | **Met.** The declaration is fixed as composed, in-place presentation: literal template, position, the interposition prohibition, the 96/80 budget and the yield order. | `standards/pages/00-one-need.md:50-99`; `::tests::band_zero_fixes_the_declaration_grammar`, `::tests::band_zero_states_the_overflow_diagnosis` | `xtask/src/main.rs:67` → `RULE_DIR` |
| AC-010 | **Met.** Never-fold stated; *what counts as a mechanism* deferred to band 20 by number; neither atom carries disclosure markup of its own. | `standards/pages/00-one-need.md:5,101-125`; `::tests::band_zero_forbids_occlusion_and_defers_the_mechanism_list`, `::tests::the_rules_tree_contains_no_disclosure_markup` | `xtask/src/main.rs:67` → `RULE_DIR` |
| AC-011 | **Met.** The token table, the three named losers, and the two-part amendment rule are all on the page a contributor will actually open. | `standards/pages/10-the-need-set.md:12-17,39-47,149-167`; `::tests::band_ten_names_every_needs_token_and_no_other`, `::tests::band_ten_names_the_three_rejected_options`, `::tests::band_ten_states_the_two_part_amendment_rule` | `xtask/src/main.rs:67` → `RULE_DIR` |
| AC-012 | **Met.** Both `orientation` ceilings are rules a reviewer applies without the author. | `standards/pages/10-the-need-set.md:77-147`; `::tests::band_ten_carries_both_orientation_ceilings` | `xtask/src/main.rs:67` → `RULE_DIR` |
| AC-013 | **Met.** Composed in the repository's enforced atom grammar, inside every ceiling: 3 rules / 5,352 bytes and 4 rules / 7,909 bytes; no prose line over 96 columns; every `**Rejects.**` over 120 characters. | both atoms; `::tests::both_atoms_carry_the_atom_head_grammar`, `::both_atoms_carry_the_five_sections_in_order`, `::both_atoms_are_inside_the_rule_and_byte_ceilings`, `::prose_lines_stay_within_ninety_six_columns`, `::every_rejects_section_names_a_wrong_page` | `xtask/src/main.rs:67` → `RULE_DIR` |
| AC-014 | **Met.** Eight fences, all tagged `text`; no untagged opener; every `**Evidence.**` opens on a repository `path:line`. | both atoms; `::tests::no_rust_tagged_and_no_untagged_fence_in_the_rules_tree`, `::tests::evidence_sections_cite_the_repository_first` | `xtask/src/main.rs:67` → `RULE_DIR` |

**Presentation, and why the tests alone would not have caught a bare render.** Both atoms
are composed in the constitution's own document grammar rather than emitted as prose that
happens to contain the right words: `# NN — Title`, a one-source-line `> **Load when:**`,
a `> **See also:**` naming sibling bands **by number** so nothing dangles before bands 20
and 40 exist, `---`, then `## RP-NN-N.` imperatives each carrying **Why.** · **Do** ·
**Not** · **Rejects.** · **Evidence.** in that fixed order. Hierarchy is carried by heading
level and bold run-in markers, never by colour; the four-token table is the only tabular
region and every token is a spelled word, never a colour or an icon. `less` renders both
files with nothing lost.

**Deferred / not this story.** The router (`standards/pages/README.md`) and its generated
index, the band-20 fold rule, bands 30 and 40, the checker, the gate step, the `INERT`
entry and the `docs/README.md` announcement are all named in the PR boundary and belong to
slice-mates or to the next slice. `::tests::router_is_not_created_by_this_story` is the
scheduled obligation that makes the router's absence legible rather than accidental; it is
inverted by `router-precedence-and-announcement` in this same slice.

**One honest mismatch, not a silent flip.** AC-001's repo-state form asks that
`rg -n '"how-to"' xtask/src` return exactly one line; it returns two, the second being the
order assertion inside this module's own test — which the same AC's static verification
requires. There is one enumeration and no second parser, so the criterion is met in
substance; the literal count is recorded here and in the implementation report's Notes.
