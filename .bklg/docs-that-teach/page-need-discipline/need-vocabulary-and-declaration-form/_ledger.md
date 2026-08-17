---
item: "HS-S0146"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The closed need set and the declaration form, landed once

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

```yaml
- id: AC-001
  criterion: "GIVEN the checker story and the router story will each need to know which needs exist, and the repository has already paid for a vocabulary spelled in two places (xtask/src/spec_trace.rs:107-121, 'three lists that must agree'), WHEN either implementer looks for the enumeration, THEN they find exactly one: `const NEEDS: &[Need]` in xtask/src/lint_pages.rs, four members in the design's render order — `orientation`, `tutorial`, `how-to`, `explanation` — over `struct Need { token, job }` with both fields carrying doc comments and the struct carrying a doc comment naming everything that must agree with it (the membership test, the router's generated row, band 10's table); and no second enumeration of these tokens exists anywhere in xtask/src/"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs"
  verifying_test: "xtask/src/lint_pages.rs::tests::needs_holds_the_four_tokens_in_design_order (plus repo-state: `rg -n '\"how-to\"' xtask/src` returns exactly one line)"

- id: AC-002
  criterion: "GIVEN the evaluator has twenty minutes and no second attempt, and this workspace already has two authoritative reference surfaces (rustdoc and spec/SPECIFICATION.md) that a third would quietly compete with, WHEN an author reaches for `reference` as a page's need, THEN the membership accessor returns `None` for it, and standards/pages/10-the-need-set.md states why it was subtracted — not that it is absent, but that a reference bucket in a narrative tree is either permanently empty or becomes a second specification — so the author reads a reason rather than guessing at an oversight"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs; standards/pages/10-the-need-set.md"
  verifying_test: "xtask/src/lint_pages.rs::tests::reference_is_not_a_member; xtask/src/lint_pages.rs::tests::band_ten_states_why_reference_was_subtracted"

- id: AC-003
  criterion: "GIVEN DT-2's named failure mode is bucket proliferation — content straining to be two things and the answer being 'add a fifth token' instead of splitting the page — WHEN any future contributor adds a seventh member to `NEEDS`, THEN `cargo check -p xtask` fails on `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, ...)` before a single test runs, with a message that says splitting the page is the remedy; the ceiling is a compile-time fact nobody can delete by deleting a test"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs, the `const _` ceiling assertion"
  verifying_test: "compile-time: `cargo check -p xtask` over the `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, ...)` item; plus the procedural capture (add a seventh member, capture the failure, revert, re-run green, `git status` clean) recorded verbatim in this ledger"

- id: AC-004
  criterion: "GIVEN the checker story must judge a page's token and the router story must render it, and two parsers that agree today drift tomorrow, WHEN either needs to know whether a token is a member, THEN there is exactly one function to call — `fn need(token: &str) -> Option<&'static Need>`, exact, case-sensitive, no trimming, no aliasing, no plural form — and its behaviour on the near misses an author actually types is pinned by tests rather than by hope"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs"
  verifying_test: "xtask/src/lint_pages.rs::tests::the_accessor_accepts_each_of_the_four_tokens; xtask/src/lint_pages.rs::tests::the_accessor_is_exact_and_case_sensitive"

- id: AC-005
  criterion: "GIVEN _design.md sign-off condition 2 records that renaming this directory later is not a rename — it costs three xtask/src/main.rs edits, an INERT entry and two affected tests — WHEN the checker and router stories need the tree's address, THEN it is a `const` here and not a convention: `RULE_DIR = \"standards/pages\"` and `ROUTER = \"standards/pages/README.md\"`, in the shape of ATOM_DIR/ROUTER (xtask/src/lint_constitution.rs:55-58); RULE_DIR resolves to a real directory holding this story's two atoms, and ROUTER is documented as deliberately absent until router-precedence-and-announcement creates it, so its absence at this checkpoint reads as a scheduled obligation rather than a broken pin"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs, the RULE_DIR / ROUTER consts"
  verifying_test: "xtask/src/lint_pages.rs::tests::rule_dir_holds_this_storys_two_atoms; xtask/src/lint_pages.rs::tests::router_is_not_created_by_this_story"

- id: AC-006
  criterion: "GIVEN the application author's stated fear is silent wrongness — a model that looks right and is quietly wrong — and a check whose limits are undocumented is read as a guarantee (xtask/src/lint_constitution.rs:11-13), WHEN anyone opens xtask/src/lint_pages.rs, THEN the very first thing in the module docs is `# What this does not verify`, above any description of what it does, with item 1 unhedged — it checks that a need is declared, never that the page answers it — naming DR-07's reviewer procedure as the instrument for the rest, followed by the other five limits the architecture brief requires (the set is not judged; the fold rule binds only textual markers; clause ids are not resolved; an empty tree passes everything below the vacuity guard; length is not quality)"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs module docs"
  verifying_test: "xtask/src/lint_pages.rs::tests::module_docs_open_with_what_this_does_not_verify"

- id: AC-007
  criterion: "GIVEN HS-P0020's hosting choice is still open and the declaration form rests on exactly one assumption about the medium, WHEN someone later picks a renderer, THEN the module docs already tell them what breaks: a named paragraph stating the form assumes only that the medium renders CommonMark blockquotes as visible body text in document order, assumes nothing about front matter or directory-derived navigation, and that a renderer which strips or relocates leading blockquotes, or requires front matter, invalidates this decision and re-opens DR-05 — in those words, so a hosting change is visibly a re-opening rather than a silent contradiction"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> xtask/src/lint_pages.rs module docs"
  verifying_test: "xtask/src/lint_pages.rs::tests::module_docs_carry_the_hosting_assumption; plus the procedural read against .bklg/docs-that-teach/page-need-discipline/_design.md 'Hosting assumption...' and sign-off condition 1"

- id: AC-008
  criterion: "GIVEN an xtask/src/*.rs file absent from the bin crate's module list is compiled by nothing, and the gate runs clippy with -D warnings over --all-targets, WHEN this story's checkpoint is taken, THEN `mod lint_pages;` sits in xtask/src/main.rs:64-70 in alphabetical position, `cargo check -p xtask` and `cargo test -p xtask` both compile and run the new module, `cargo xtask ci --fast` is green, and the module carries exactly one module-level `#![allow(dead_code, reason = \"...\")]` whose reason names page-need-checker-mounted-in-the-gate as the story that deletes it — not `#![expect]`, not crate-level, not per-item, not a bare allow"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`)"
  verifying_test: "gate-integration: `cargo check -p xtask`, `cargo test -p xtask`, `cargo xtask ci --fast`; repo-state: `rg -c 'allow\\(dead_code' xtask/src/lint_pages.rs` returns 1 and `rg -n 'page-need-checker-mounted-in-the-gate' xtask/src` returns the reason string"

- id: AC-009
  criterion: "GIVEN UX-001's test is 'read nothing but the region above the first prose paragraph and name the need', and the adapter author's measured defect is an answer that existed three documents from where they were standing, WHEN a page author opens standards/pages/00-one-need.md, THEN it fixes the declaration as composed, in-place presentation and not as a bare capability: exactly one per page; the literal line `> **Answers:** `token` — <question>?`; immediately after the `# Title` with one blank line above and below; nothing may be interposed — no badge row, no table of contents, no admonition, no 'last updated' line; <= 96 characters hard and <= 80 target; and when the question will not fit, the rule states that the page is answering more than one need and the overflow is the diagnosis"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/00-one-need.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::band_zero_fixes_the_declaration_grammar; xtask/src/lint_pages.rs::tests::band_zero_states_the_overflow_diagnosis"

- id: AC-010
  criterion: "GIVEN the declaration is never-fold class 1 and the mock found that rustdoc already wraps every page in an open `<details>` nobody has observed, WHEN a reviewer applies band 00, THEN it states that the declaration may never sit behind a fold, tab, inactive panel or `<details>` — persistent chrome in every state, in every medium — and it defers what counts as a mechanism, including a renderer-supplied open-by-default wrapper, to band 20 by number in `> **See also:**`, without settling DT-8 Part 3 in passing; and the atoms themselves contain no `<details>`, no tab strip and no accordion, so the tree does not violate on its own pages the rule it is writing"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/00-one-need.md and standards/pages/10-the-need-set.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::band_zero_forbids_occlusion_and_defers_the_mechanism_list; xtask/src/lint_pages.rs::tests::the_rules_tree_contains_no_disclosure_markup"

- id: AC-011
  criterion: "GIVEN AC-003 of the project requires the resolution and its rejected options to be recorded because the perceptual review is a confirmed skip and this written record is the only record, WHEN a future contributor asks why the set is what it is, THEN standards/pages/10-the-need-set.md carries the four tokens as a table (token / the page's job / success for the reader, the third column prose-only and deliberately absent from the const), names the three losers and why each lost — literal Diataxis (the evidence names this project's exact kind of subject as where the four-box split strains), drop-the-enumeration (a lint cannot check membership in an open set), persona-keyed (a need is a property of the page, a persona of the reader) — and states the amendment rule: changing the set is a two-part commit, the const and this atom together, never a one-part one"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/10-the-need-set.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::band_ten_names_every_needs_token_and_no_other (failure names which token moved, RS-81-5); xtask/src/lint_pages.rs::tests::band_ten_names_the_three_rejected_options; xtask/src/lint_pages.rs::tests::band_ten_states_the_two_part_amendment_rule"

- id: AC-012
  criterion: "GIVEN DT-3's stated cost is a category the source taxonomy does not have, and the mitigation is that the category cannot grow into a sink, WHEN the evaluator's landing page is written, THEN band 10 carries both ceilings as rules a reviewer applies without the author: RP-10-2 — an `orientation` page contains links and at most one sentence per destination saying what that destination answers, teaches nothing, and the moment it explains, instructs or references it is answering a second need and must be split; and RP-10-3 — at most one `orientation` page per directory level"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/10-the-need-set.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::band_ten_carries_both_orientation_ceilings"

- id: AC-013
  criterion: "GIVEN the next page author must be able to load one rule rather than the corpus (UX-012), and an unstyled, shapeless atom satisfies every content assertion above perfectly, WHEN either new atom is opened, THEN it is composed in the repository's own enforced atom grammar and not merely correct: `# NN — Title`, `> **Load when:** ...`, `> **See also:** ...` naming sibling bands by number, not by link (so no dangling link exists before those bands land), `---`, then each `## RP-NN-N. <imperative sentence>` followed by **Why.** / **Do** / **Not** / **Rejects.** / **Evidence.** in that fixed order — with <= 6 rules and <= 16,384 bytes per atom, prose <= 96 columns outside tables, and every **Rejects.** naming a wrong page that could actually ship in >= 120 characters"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/00-one-need.md and standards/pages/10-the-need-set.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::both_atoms_carry_the_atom_head_grammar; ::both_atoms_carry_the_five_sections_in_order; ::both_atoms_are_inside_the_rule_and_byte_ceilings; ::prose_lines_stay_within_ninety_six_columns; ::every_rejects_section_names_a_wrong_page"

- id: AC-014
  criterion: "GIVEN this tree is deliberately not registered with the doctest harness (xtask/src/lib.rs:28, CR-0), so a `rust` fence here would be a Rust claim nothing in the workspace compiles, WHEN either atom shows an example, THEN every fence is tagged `text` or `markdown` — a `rust`-tagged fence and an untagged fence are both wrong, the second so that a future decision to register the tree cannot be undermined retroactively — and every **Evidence.** section orders its citations repository path:line first, then the discovery dossier, then external URLs with a *(checked ...)* stamp"
  satisfied: false
  evidence: ""
  mount_point: "xtask/src/main.rs:64-70 (`mod lint_pages;`) -> standards/pages/00-one-need.md and standards/pages/10-the-need-set.md, read by xtask/src/lint_pages.rs's tests"
  verifying_test: "xtask/src/lint_pages.rs::tests::no_rust_tagged_and_no_untagged_fence_in_the_rules_tree; xtask/src/lint_pages.rs::tests::evidence_sections_cite_the_repository_first"
```
