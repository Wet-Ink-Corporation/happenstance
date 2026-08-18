---
item: "HS-S0139"
stage: implement
created: "2026-08-18T00:20:00.000Z"
updated: "2026-08-18T00:20:00.000Z"
---

# Implementation Report — No fence opts out of the check unnoticed

## TDD Evidence

Sixteen tests were written into `xtask/src/lint_narrative.rs`'s existing
`#[cfg(test)] mod tests` — the module its slice-mate created one commit earlier — against a
`check_fences` and a `check_allowances` with empty bodies and a `fences()` parser copied from
`lint_constitution` *including* its unpaired-opener hole. Empty bodies rather than absent
functions, so the red run is an assertion failure per row instead of the compile error a new
symbol would produce.

Red run: `cargo test --locked -p xtask --bin xtask lint_narrative::` → **31 passed; 16
failed** (the 31 are the slice-mate's, still green — this story regressed nothing). Green
run: **47 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `an_untagged_fence_is_rejected`, `a_text_tagged_fence_is_neither_compiled_nor_flagged` | RED: `got: []` — the walk returned nothing. GREEN with the message asserted verbatim and the line asserted at 3. The `text` companion was green throughout and is the negative half: it exists to fail a future walk that starts rejecting non-Rust tags. |
| AC-002 | `an_unrecognised_info_string_part_is_a_problem`, `the_enumerated_info_string_parts_are_accepted`, `an_error_code_without_compile_fail_is_a_problem`, `a_compile_fail_claiming_a_code_the_prose_never_names_is_a_problem` | RED: `got: []` on three plausible *novel* spellings — `ignore_me`, `norun`, `edition2027` — rather than a known one, per RS-81-2. GREEN on the closed match whose only non-accepting arm is `_ => recognised = false`. |
| AC-003 | `an_unlisted_ignore_fence_is_rejected`, `a_comment_above_an_ignore_fence_does_not_permit_it`, `a_listed_ignore_fence_passes`, `an_anchored_allowance_survives_an_insertion_above_the_fence`, `an_allowance_with_an_empty_reason_is_a_problem` | RED: `got: []` on the two rejection tests. `a_listed_ignore_fence_passes` was green trivially before the rule existed — the spec names it as the exception, and it is only meaningful paired with the rejection test in the same commit. The comment test is the named wrong implementation: the copied parser carries no `preceding` field, so the comment form cannot rescue anything by construction. |
| AC-004 | `a_stale_allowance_is_a_problem`, `an_allowance_for_a_fence_that_no_longer_opts_out_is_a_problem`, `a_malformed_allowance_is_a_problem`, `a_duplicate_allowance_is_a_problem` | RED: all four `got: []`. GREEN on the `Usage` bookkeeping recorded during the forward pass, which gives the reverse sweep, EC-005's specific message and EC-004's duplicate detection out of one walk. |
| AC-005 | `four_backtick_fences_are_not_examples`, `a_problem_names_the_page_and_the_fence_line`, `an_unterminated_fence_is_a_problem` | The four-backtick test was green from the parser copy and is asserted through the **whole walk**, not only `fences()`, so it fails a future check that re-reads the raw text. `a_problem_names_the_page_and_the_fence_line` RED: `got: []`; GREEN at `docs/adapters/sqlite.md:4`, inside the 48-column budget. `an_unterminated_fence_is_a_problem` RED against the verbatim copy — that is EC-002, an opt-out route inherited by copying, fixed in the copy only. |
| AC-006 | `problems_are_reported_in_source_order`, `every_problem_is_reported_not_the_first` | RED: `got: []`. GREEN with three problems across two pages in path-then-line order, and 40 problems printing as 40 lines with nothing containing `more`. |
| AC-007 | `the_module_docs_state_the_text_limit_and_the_divergence`, `a_clean_tree_still_reports_one_line_after_the_fence_walk` | RED: `the module docs must state \`tagged \`text\`\``. GREEN once the `text` back door joined `# What this does not verify` and the divergence section landed. The test reads the module's own source with `include_str!` — a bin-crate `//!` doc is not compiled by `cargo test --doc`, so a doctest could not have carried this. |

**One test was found to be wrong and was fixed rather than weakened.**
`a_compile_fail_claiming_a_code_the_prose_never_names_is_a_problem` went green against a
`page.text.contains(code)` copied from the precedent — because the fence's own info string
`` ```rust,compile_fail,E0277 `` contains the code, so the rule was satisfied by the very
line it was checking. `names_outside_a_fence_marker` (`:576-581`) now excludes fence
delimiter lines. This is a latent weakness in `lint_constitution`'s own `check_fences`; it
was fixed **in the copy only**, per the standing instruction not to touch that module.

## Commits

| SHA | Subject |
| --- | ------- |
| `<checkpoint>` | feat(checked-documentation-surface): Fence discipline and allowance list |

Recorded as first written; the reachable commit is in the slice digest.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `xtask/src/lint_narrative.rs` | `IGNORE_ALLOWANCES` (`:170`) — empty, `(page path, line-or-anchor, reason)`, with the anchor form recommended on the `const` itself and the "grows by review, never as a repair" line. `CHECKER` (`:152`), the file a stale allowance is a defect *in*. `Fence` (`:383`) with a `closed` flag and no `preceding` field. `Usage` (`:396`). `fences()` (`:407`) — the precedent's four-backtick step-over plus the EC-002 fix. `check_fences` (`:463`), `check_allowances` (`:620`), `check_page` (`:667`) and the helpers `names_outside_a_fence_marker` (`:576`), `is_error_code` (`:583`), `allowance_names` (`:596`), `is_line_key` (`:609`). `Page` gained `text`, read once at enumeration (`:250-256`) with EC-001's hard-error posture. Module docs gained the `text` limit (`:28-34`) and the divergence section (`:81-100`). |
| `.bklg/…/fence-discipline-and-allowance-list/` | `_ledger.md` flipped with evidence; this report and `report.md`. |

No file outside `xtask/src/lint_narrative.rs` and this story's backlog folder was touched. No
`Step`, no subcommand, no banner, no dependency, no `docs/` page.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask lint_narrative::` | 47 passed; 0 failed (red: 31 passed, 16 failed) |
| `cargo clippy --locked -p xtask --all-targets --all-features -- -D warnings` | clean, no new `#![allow]` |
| `cargo test --locked -p xtask` | 321 across four targets; 0 failed; 3 ignored. `lint_constitution`'s own tests unchanged and green. |
| `cargo fmt --all --check` | clean, run last |
| `cargo run --locked --quiet -p xtask -- narrative` (tree clean) | `  1 pages, all consistent`, exit 0 |
| `cargo run --locked --quiet -p xtask -- narrative` (three-defect page) | the transcript below, exit 1 |
| `cargo run --locked --quiet -p xtask -- ci --fast` | green (run again after story 3; see the slice digest) |

**The observed failure surface.** The Merge DoD asks for the fence checks visible in the
step's real output rather than only in a test. A three-defect page was written to
`docs/three-defects.md`, observed, and **deleted in the same session** — the PR boundary
forbids a page under `docs/`, and a page carrying these defects would fail the gate forever:

```
  docs/three-defects.md:7 — an untagged fence is compiled as Rust; tag it `rust` or `text`
  docs/three-defects.md:13 — an `ignore` fence needs an `IGNORE_ALLOWANCES` entry naming it; a comment above the fence does not permit it, because a comment is reviewable only in the diff that introduced it
  docs/three-defects.md:19 — unrecognised fence info string `rust,norun`
  xtask/src/narrative.rs — does not include three-defects.md; its examples are never compiled
  xtask/src/narrative.rs — no `mod three_defects`; one module per page is what keeps a doctest failure's line number relative to the page

xtask failed: 5 problem(s) in docs
```

Read against `_design.md` `## Composition`: two-space indent, `{path}:{line}` first, an em
dash, the message; every problem in source order — the tree, then the harness that registers
it; the count last carrying the directory; nothing truncated; exit non-zero. `git status
docs/` is clean afterwards.

## Notes

**The walk is a per-page pure function over `(page path, page text)`**, exactly as the
implementation notes asked, and `check_page` sorts a page's problems by line before composing
them. That sort is what lets `hidden-content-resolution` add a check to the same walk and have
its problems interleave with the fence problems in line order rather than land in a second
block.

**The allowance sweep reports against `xtask/src/lint_narrative.rs`, not against the page.**
The precedent's reverse half reports a stale registration against the *harness* — the file
that carries the mistake — and a stale allowance's mistake is in this module's `const`, not in
the page it names. The message still names the entry by page and key so the reader can find
both.

**`is_markdown` was hardened** from a byte-slice suffix comparison to `str::get`, so a file
name whose last three bytes fall inside a multi-byte character cannot panic. EC-006 asks for
no panic path in the walk; the precedent's `has_ext` has the same latent hazard and was left
alone.

**What this story did not do.** It added no gate step, banner, subcommand or dependency: the
mount is the slice-mate's and this story consumes it. `IGNORE_ALLOWANCES` ships empty — every
allowance-path test constructs its own list — because a `const` that ships with an entry ships
with a precedent for adding the second. `lint_constitution.rs` is untouched, including the two
weaknesses found in it (the `compile_fail` code check and the dropped unpaired opener), both
fixed only in the copy.
