---
item: "HS-S0146"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The closed need set and the declaration form, landed once

## TDD Evidence

Which tests went red, then green, mapped to each AC. The unit tests live in
`xtask/src/lint_pages.rs`'s `#[cfg(test)] mod tests` (24 tests). The red run is
`cargo test -p xtask --bin xtask lint_pages`: **7 passed; 17 failed**, every failure a
read of a rule atom that did not exist yet — `reading …\standards/pages\00-one-need.md:
The system cannot find the path specified. (os error 3)` — which is an assertion about
missing behaviour (the two atoms this story owes), not a compile or import error. The
green run after both atoms landed is **24 passed; 0 failed**.

The seven that passed on the red run are the `const`-and-accessor half. Those were
carried into this worktree by an interrupted earlier run of this same story (uncommitted,
on this branch), so their own red step was not re-observed at authoring time. They are not
taken on trust: the ceiling assertion that guards the set was **seen to fail** in this
session (AC-003 below), which is the one assertion in that half that a deleted test could
otherwise hide.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `needs_holds_the_four_tokens_in_design_order` | Passing on the red run against the carried-in `const`. Its bite is proved by AC-003's mutation: three extra members made the module refuse to compile at all, so the enumeration is load-bearing rather than decorative. |
| AC-002 | `reference_is_not_a_member`, `band_ten_states_why_reference_was_subtracted` | RED: the second panicked reading `standards/pages/10-the-need-set.md` — the atom did not exist. GREEN once band 10 stated the subtraction *and its consequence* (`spec/SPECIFICATION.md`, "second specification"), not merely the absence. |
| AC-003 | the `const _: () = assert!(NEEDS.len() <= MAX_NEEDS, …)` item, exercised by `cargo check -p xtask` | SEEN TO FAIL: three extra `Need` members appended → `error[E0080]: evaluation panicked: the need set is closed. A page that strains against every token is answering more than one need: split the page, never add a token.` at `xtask\src\lint_pages.rs:115:15`, `error: could not compile 'xtask'`. Reverted → `Finished 'dev' profile`. `git status --short` clean of residue. |
| AC-004 | `the_accessor_accepts_each_of_the_four_tokens`, `the_accessor_is_exact_and_case_sensitive` | Passing on the red run. Both pin behaviour on the near misses an author actually types — `reference`, `guide`, `Explanation`, `explanation ` (trailing space), `how_to`. |
| AC-005 | `rule_dir_holds_this_storys_two_atoms`, `router_is_not_created_by_this_story` | RED: the first panicked on `standards/pages\00-one-need.md` not existing. GREEN once both atoms landed. The second asserts `ROUTER`'s **absence** on purpose and is inverted by `router-precedence-and-announcement` inside this same slice. |
| AC-006 | `module_docs_open_with_what_this_does_not_verify` | Passing on the red run; reads the module's own source via `include_str!` and asserts `# What this does not verify` precedes every other `//!` heading, with all six limits in order and limit 1 naming DR-07. |
| AC-007 | `module_docs_carry_the_hosting_assumption` | Passing on the red run. Procedural half done here: the paragraph read against `_design.md:148-156` and sign-off condition 1 (`:718-721`) — the design's own words, not a paraphrase. |
| AC-008 | `cargo check -p xtask`, `cargo test -p xtask`, `cargo xtask ci --fast` | The mount (`xtask/src/main.rs:67`) was already in the tree from the interrupted run; it is what makes every `cargo check` in this report a statement about a compiled module. Exactly one scoped `allow` at `:51-56`, naming the story that deletes it. |
| AC-009 | `band_zero_fixes_the_declaration_grammar`, `band_zero_states_the_overflow_diagnosis` | RED: both panicked reading `standards/pages/00-one-need.md`. GREEN after band 00 landed — and then RED again on the second, because the sentence "the overflow is the diagnosis" had been line-wrapped across a newline; re-wrapped, GREEN. That is the assertion earning its keep: a contains-check over prose is a check the author can break by rewrapping. |
| AC-010 | `band_zero_forbids_occlusion_and_defers_the_mechanism_list`, `the_rules_tree_contains_no_disclosure_markup` | RED: file-missing. GREEN with the never-fold sentence present, band 20 named **by number**, no permitted-mechanism list, and neither atom carrying `<details`, `<summary`, `role="tab"` or `{{#tab`. |
| AC-011 | `band_ten_names_every_needs_token_and_no_other`, `band_ten_names_the_three_rejected_options`, `band_ten_states_the_two_part_amendment_rule` | RED: file-missing, then a second RED on `band 10 names the option that lost and why: open set` — "an open set" had wrapped across a line. Re-wrapped, GREEN. The token-table test compares row by row against `NEEDS` and names *which* token moved (RS-81-5). |
| AC-012 | `band_ten_carries_both_orientation_ceilings` | RED: file-missing. GREEN with `## RP-10-2.` (links only, "It teaches nothing", split remedy) and `## RP-10-3.` (one `orientation` page per directory level). |
| AC-013 | `both_atoms_carry_the_atom_head_grammar`, `both_atoms_carry_the_five_sections_in_order`, `both_atoms_are_inside_the_rule_and_byte_ceilings`, `prose_lines_stay_within_ninety_six_columns`, `every_rejects_section_names_a_wrong_page` | RED: all five file-missing. GREEN at 3 rules / 5,352 bytes and 4 rules / 7,909 bytes against ceilings of 6 and 16,384; no prose line over 96 columns; every `**Rejects.**` over 120 characters. |
| AC-014 | `no_rust_tagged_and_no_untagged_fence_in_the_rules_tree`, `evidence_sections_cite_the_repository_first` | RED: file-missing. GREEN with all eight fences tagged `text`, no untagged opener, and every `**Evidence.**` block opening on a repository `path:line` span before the discovery dossier. |

No test was weakened, deleted or skipped to reach green. The two RED-after-GREEN
episodes above were both fixed in the prose the assertion is about, never in the
assertion.

## Commits

| SHA | Subject |
| --- | ------- |
| `STORY-1-SHA` | `feat(page-need-discipline): Need vocabulary and declaration form` |

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_pages.rs` | **New**, 745 lines. Module docs with `# What this does not verify` first (six limits) and the hosting-assumption paragraph; one module-level `#![allow(dead_code, reason = …)]` naming `page-need-checker-mounted-in-the-gate`; `struct Need { token, job }`; `const NEEDS` (four members); `MAX_NEEDS` and the `const _` ceiling assertion; `RULE_DIR` and `ROUTER`; `fn need`; and a 24-test `#[cfg(test)] mod tests` that reads the module's own source and the two atoms by name (never `read_dir`). |
| `xtask/src/main.rs` | **One line**: `mod lint_pages;` at `:67`, alphabetically between `mod lint_narrative;` and `mod lints;`. This is the mount — an `xtask/src/*.rs` file absent from that list is compiled by nothing. |
| `standards/pages/00-one-need.md` | **New**, band 00. Three rules: RP-00-1 (exactly one declaration), RP-00-2 (the grammar, the position, the 96/80 budget, the yield order), RP-00-3 (never occluded, with the mechanism question deferred to band 20 **by number**). |
| `standards/pages/10-the-need-set.md` | **New**, band 10. The four-token table, the `reference` subtraction and the `orientation` addition with their causes, the three rejected shapes, and four rules: RP-10-1 (exact spelling), RP-10-2 and RP-10-3 (the two `orientation` ceilings), RP-10-4 (the two-part amendment commit). |
| `…/need-vocabulary-and-declaration-form/_ledger.md` | Fourteen rows flipped `false → true`, each carrying a `file:line` and the passing test id. No criterion re-worded. |

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test -p xtask --bin xtask lint_pages` | **RED** 7 passed / 17 failed → **GREEN** 24 passed / 0 failed |
| `cargo check -p xtask` | green (and observed failing on the ceiling mutation, then green again on revert) |
| `cargo clippy -p xtask --locked --all-targets --all-features -- -D warnings` | green |
| `cargo test -p xtask` | green — 24 module tests, 63 doctests |
| `cargo fmt --all --check` | red on first run (five rustfmt diffs in the carried-in module); `cargo fmt --all` run as the **last** pass, then green |
| `cargo xtask lints` | green — `27 atoms, all consistent`; `2 pages, all consistent` |
| `cargo xtask ci --fast` | green — `all required checks passed (--fast: 4 optional step(s) not run)` |
| `cargo xtask affected --base main` | green, and **wide** — every workspace member selected, exactly as EC-007 predicts for an unrecognised `standards/pages/` path |
| `redkiln doctor` | `no problems`, and exactly **six** `template-drift` advisories |
| `git diff main -- standards/rust/README.md .kb .redkiln/templates xtask/Cargo.toml crates/` | all empty |

## Notes

**Deviation 1 — `> **Load when:**` and `> **See also:**` are two blockquote blocks, not
one.** The constitution keeps them in a single blockquote
(`standards/rust/00-prime-directives.md:3-9`). Here a blank line separates them, because
`both_atoms_carry_the_atom_head_grammar` asserts that the line *after* `Load when:` does
not begin with `>` — the mechanical form of the one-source-line rule the router's
`## The shape of a rule` will state, and the reason the constitution's own generated index
carries mid-phrase trigger cells today. The alternative was to weaken the assertion so it
tolerated a `>` continuation, which would have re-admitted exactly the defect the rule
exists to refuse. Rendered, the two blockquotes stack; nothing is lost.

**Deviation 2 — RP-00-3's `Do` and `Not` are prose, not fences.** The wrong page this rule
names is a declaration inside a collapsed panel, and writing that fragment would have put
`<details`/`<summary>` into the tree — anti-pattern 6 applied to the atom that writes the
rule against it, and a failure of `the_rules_tree_contains_no_disclosure_markup`. The atom
says so in the rule itself rather than leaving the reader to wonder why the shape is
described instead of shown.

**Not a deviation, recorded because a reader will check it.** AC-001's repo-state form —
`rg -n '"how-to"' xtask/src` returns exactly one line — returns **two**:
`xtask/src/lint_pages.rs:95` (the enumeration) and `:287` (the order assertion inside this
module's own test). The second is required by the same AC's static verification, which
asks the test to assert "each token verbatim". The substance holds: there is one
enumeration and no second parser.

**`cargo xtask affected --base main` is correct and slow here, and was not "fixed".**
`standards/pages/` is unrecognised by `affected_packages` and absent from `INERT`, so the
run widens to the whole workspace (`xtask/src/affected.rs:222-223`). The repair is a
*pair* of edits that only makes sense with the checker, and it belongs to
`page-need-checker-mounted-in-the-gate` (EC-007, CR-4).

**Provenance.** `xtask/src/lint_pages.rs` and the `xtask/src/main.rs` mount were present
uncommitted in this worktree at the start of this session, left by an interrupted earlier
run of this same story on this branch. They were read in full, verified against the spec
and `_design.md`, exercised red-then-green over the two atoms this session authored, and
mutation-checked at the ceiling. Nothing was accepted on trust.
