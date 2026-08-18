---
item: "HS-S0137"
stage: implement
created: "2026-08-17T22:45:00.000Z"
updated: "2026-08-17T22:45:00.000Z"
---

# Implementation Report — A prose-only change selects the package that compiles it

## TDD Evidence

Which tests went red then green, mapped to each AC. All four unit tests were written
into `xtask/src/affected.rs`'s existing `mod tests` before any production line moved,
beside the constitution pair they mirror rather than at the end of the module — the
file's own comment at `:711-733` is written to be read as the second half of a pair,
and adjacency is what makes the both-directions obligation visible to the next reader.
The red run was `cargo test --locked -p xtask --bin xtask affected::`: **19 passed;
2 failed**. The green run is **21 passed; 0 failed**.

| AC | Test | Red → Green |
| -- | ---- | ----------- |
| AC-001 | `a_narrative_page_selects_xtask` | RED: `assertion 'left == right' failed / left: {} / right: {"xtask"}` — a page under the narrative tree selected **nothing**, which is the empty-selection failure the story exists to close. GREEN after the third disjunct landed on the arm at `xtask/src/affected.rs:219`. |
| AC-002 | `the_narrative_arm_does_not_widen_to_all_prose` | Green from the moment it was written, and deliberately so: it is the negative half of a directional pair, and its job is to fail a *future* over-broad arm (`path.ends_with(".md")`, say) rather than the present one. Verified as a real instrument by the counter-observation recorded below, where widening the arm's reach did change what `affected_packages` returns. |
| AC-002 | `a_top_level_prose_file_selects_nothing` (renamed from `a_docs_only_change_selects_nothing`) | Green throughout, by construction: the assertion is on `RUNBOOK.md` and never moved. Only the name was false. Renamed, not re-pointed and not deleted — it is one of the two anchors of this module's widening posture. |
| AC-003 | `the_narrative_tree_is_no_longer_inert` | RED: `assertion failed: !is_inert("docs/append-conditions.md")`. GREEN after `"docs/"` was **removed** from `INERT` rather than left shadowed behind the arm. This is the test the other three cannot substitute for: they all pass while the prefix sits unreachable on the list, waiting for the next reordering of the `else if` chain to re-arm it. |
| AC-004 | (review, per RS-81-1) | No `#[test]` can assert that a comment is true, which is why RS-81-1 splits the obligation into a test plus a statement. The tests are AC-001–AC-003; the four rewritten sites are listed under Changes, and `rg -n '"docs/"' xtask/src/affected.rs` is the mechanical half — the arm's literal and two comments, no `INERT` entry. |
| AC-005 | (gate-integration, recorded) | Run and captured verbatim to `_observed-affected-run.md` in this story's folder. The evidence is the parsed output naming the package, not the exit status (RS-81-4). |

## Commits

| SHA | Subject |
| --- | ------- |
| `dad0b86` | feat(checked-documentation-surface): A prose-only change selects the package that compiles it |

The row records the checkpoint **as first written**, before this row and the story's
own artifacts were folded into it. A commit cannot contain its own SHA, so writing the
SHA down amends the commit and gives it a new one; the reachable commit is that
successor, reported in the slice digest and recorded on the item by
`redkiln record-links --sha` at the advance seam.

## Changes

One file of production code, exactly as the PR boundary allows, plus this story's own
folder.

| Site | Shape of the change |
| ---- | ------------------- |
| `xtask/src/affected.rs:217-220` | The arm gains a third disjunct, `path.starts_with("docs/")`. One condition rather than a separate `else if` branch: the three paths are selected for one reason, and one comment should state it once. It stays **inside** the existing `None` branch, so `WORKSPACE_WIDE`'s early return at `:198-200` keeps precedence and `the_lockfile_selects_everything` stays green (EC-005). |
| `xtask/src/affected.rs:221-244` | The arm's comment: why all three prose paths select `xtask`; why `"docs/"` is a literal here and will be `xtask::narrative`'s pinned `TREE` on the other side (a lib-target module and a bin-target module cannot share a private constant, and a `pub` seam across the two targets costs more than seven characters of duplication); and, in `xtask/src/lint_constitution.rs:9-28`'s register, that **selection is not compilation** — naming `xtask` means the package is built and tested, not that any fence compiled, and nothing here says anything about whether a page teaches. |
| `xtask/src/affected.rs:276-293` | `"docs/"` removed from `INERT`. Removed, not shadowed: a prefix left on the list but unreachable is one the next reordering re-arms. |
| `xtask/src/affected.rs:267-274` | `is_inert`'s doc comment extended rather than restarted elsewhere, because it now has **two** deliberately-absent trees; stating them together is what stops the next contributor reading the `standards/rust/` paragraph as the sole exception. |
| `xtask/src/affected.rs:212-216` | The tree list that claimed `docs/` "reaches no package" now names `spec/`, `.github/`, `.bklg/` and `.kb/`. |
| `xtask/src/affected.rs:136-142` | The empty-selection branch's justification. It used to read "a docs-only story genuinely has no package to compile"; it now names the trees the emptiness still covers and records that the narrative tree is not among them. |
| `xtask/src/affected.rs:676-686, 735-768` | The rename, and the three new tests, ordered beside the constitution pair. |
| `.bklg/.../narrative-tree-story-grain-selection/_observed-affected-run.md` | AC-005's transcript, with what it is evidence of, and one honest limit. |

Not touched, and named as out of scope: `docs/README.md` (the slice-mate's paragraph),
`xtask/src/narrative.rs`, `xtask/src/lib.rs`, `REQUIRED`, `print_help`, and
`affected::run`'s unconditional file-reading list at `:120-125` — that last is a live
open decision belonging to the story that creates the checker.

## Gates

| Command | Result |
| ------- | ------ |
| `cargo test --locked -p xtask --bin xtask affected::` (red run) | 19 passed; **2 failed** — `a_narrative_page_selects_xtask` and `the_narrative_tree_is_no_longer_inert`. |
| `cargo test --locked -p xtask --bin xtask affected::` (green run) | 21 passed; 0 failed. |
| `cargo test --locked -p xtask --all-features` | 43 bin-target unit tests, 168 lib/integration, 63 doctests — 0 failed, 3 ignored. |
| `rg -n '"docs/"' xtask/src/affected.rs` | Three hits: the arm's literal at `:219`, and two comments. No `INERT` entry. |
| `cargo run -p xtask -- lint-constitution` | `  27 atoms, all consistent` (DoD-7). |
| `cargo test -p xtask --doc` | 63 passed; 0 failed (DoD-7). |
| `cargo fmt --all --check` | Clean (exit 0), run last, after clippy. |
| `cargo clippy --locked -p xtask --all-targets --all-features -- -D warnings` | Clean (exit 0). |
| `cargo xtask affected --base HEAD` on a tree-confined working tree | `  xtask`, then fmt / clippy / 231 tests, then `affected gate passed`. Transcript in `_observed-affected-run.md`. |
| `cargo xtask ci --fast` | Green, with `=== the narrative tree's examples compile ===` and `  1 page(s)' examples enumerated` still in place — this story added no `Step`, so `steps_named` has nothing new to panic on. |

## Notes

Two deviations, both stated.

1. **AC-005's transcript was taken with `--base HEAD`, not `--base main`.** On this
   branch the merge-base diff against `main` is the whole initiative — 345 files,
   including every planning artifact — so no `--base main` invocation here can present
   a tree-confined change, and one that claimed to would be observing the branch
   rather than the change. `changed_files` (`xtask/src/affected.rs:547-571`) unions
   four sources; `--base HEAD` collapses the first to nothing and leaves exactly the
   one working-tree edit, which is the diff shape the story-grain gate actually meets
   mid-story. Same command, same code path, same `affected_packages` call. The
   transcript says all of this in its own preamble rather than presenting a
   `--base HEAD` run as if it were a `--base main` one.
2. **The full pre-story counter-transcript could not be produced, and is recorded as
   unobtainable rather than omitted.** Reconstructing "arm absent **and** `"docs/"`
   back on `INERT`" requires editing `xtask/src/affected.rs`, which is itself a change
   inside a member directory — so the run then selects `xtask` for the wrong reason
   and reports two changed files instead of one. What *was* observed is the partial
   counter-case: removing only the arm makes the same page path unrecognised, and the
   selection widens to all six members rather than narrowing to none. That is the
   module's stated posture firing, and it demonstrates that the arm and the `INERT`
   removal are jointly load-bearing rather than one being cosmetic. The mechanical
   statement of the pre-story behaviour is the red run of `a_narrative_page_selects_xtask`
   (`left: {}`), which drives the same function with no git repository at all.

One thing deliberately **not** done: no `Step`, no `REQUIRED` member, no subcommand,
no `probe`, and no edit to `print_help`. RS-80-1 demands a `Step` reachable by name for
a *check*; a package selector is not a check, and the mount at `xtask/src/main.rs:64`,
`:683-688` and `:729` was already complete. Adding one would have produced a second,
redundant gate entry for a mapping that has no output of its own.
