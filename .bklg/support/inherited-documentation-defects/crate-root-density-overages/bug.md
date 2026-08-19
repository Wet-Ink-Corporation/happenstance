---
id: HS-B0001
uid: 634b65
type: bug
slug: crate-root-density-overages
title: The crate-root fence and headings exceed the documentation density budget
parent: HS-P0026
initiative: support
project: inherited-documentation-defects
status: pending
process: bug
stage: triage
automation: HITL
severity: medium
blocked_by: []
blocks: []
owner: ryan-britton
created: 2026-08-19
updated: 2026-08-19T14:51:00.896Z
links:
  pr: null
  commits: []
  kb: []
schema: 1
process_rev: "92524823"
---
# The crate-root fence and headings exceed the documentation density budget

## Summary

Three measured density overages on `crates/happenstance/src/lib.rs` — the page a
`cargo add happenstance` reader lands on. They are properties of HS-P0016's `commit`
landing program and its headings, not of any page `docs-that-teach` authored.

They were found by `HS-S0185 boundary-refusal-encounter` while measuring its own
surfaces, measured again by `merge-forward-preflight/_baseline.md` before that story
began, and struck from that story's AC-007 by `spec.md § Amendment — BC-002`. They are
recorded here as **owed**, not waived: they are real overages against budgets the
`docs-that-teach` initiative set and met everywhere it was allowed to.

**Origin:** `.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/_conditions.md`
§ BC-002 and § "BC-002 — RESOLVED 2026-08-19", and
`.bklg/docs-that-teach/application-author-path/boundary-refusal-encounter/spec.md`
§ "Amendment — BC-002".

## Reproduction

The three findings, verbatim from `_conditions.md:207-209`:

| id | finding | budget, and where it comes from | how to re-measure |
| --- | --- | --- | --- |
| F-1 | the crate-root fence is **35 rendered lines** | 32, the crate-root exemption in `_design.md` § Density budget | count the non-hidden lines of the fence in the module doc of `crates/happenstance/src/lib.rs:26-64` |
| F-2 | **70 columns** on two lines | 68; 72 is where `overflow-x` engages on the 696px fence at 1024×768 | widest rendered fence line in `target/doc/happenstance/index.html` |
| F-3 | two of four `##` headings exceed **22 characters**, at 39 and 26 | 22; the 200px sidebar TOC clips with an ellipsis and never wraps | `git grep -n "^//! # " -- crates/happenstance/src/lib.rs`, then count each title |

## Impact

Reader-facing, on the crate's front door, and small but not cosmetic:

- **F-2** puts two fence lines past the point where `overflow-x` engages at 1024×768, so a
  reader on a laptop scrolls horizontally to read the landing program.
- **F-3** clips two of the four sidebar TOC entries with an ellipsis. The 200px sidebar
  never wraps, so the clipped half of a 39-character heading is simply not readable.
- **F-1** is the mildest: eight rendered lines over a budget chosen so the fence fits one
  screen without scrolling.

None of the three blocks anything. All three sit on the one page the
`docs-that-teach` initiative most wanted to be exemplary and was least able to touch.

## Acceptance Criteria

Deliberately not enumerated as `AC-###` rows yet — this item is opened to give the
overages an owner, not to schedule them. Whoever picks it up should decide whether the
right fix is shortening the program, shortening the headings, or amending the budget with
a stated reason, and only then write the rows.

What is fixed here must keep the crate root's own signed-off assertions green:
`crates/happenstance/tests/doc_budget.rs:157` requires **exactly one** fence on the page,
and `MODULE_DOC_LINES = 130` is currently met at exactly 130.

## Notes

**This is not `docs-that-teach`'s to fix, and that is the whole reason the item exists.**
`.bklg/docs-that-teach/application-author-path/project.md`'s risk table places the seam
with HS-P0016 at "purpose, not paragraph. This project does not touch landing copy". The
landing program is also the crate's one demonstration of the typed layer
[ADR-0006](../../../.kb/decisions/0006-bare-name-to-the-typed-layer.md) gave the bare name
to, and it is named from two vocabulary bullets and from the file's own opening comment.

**The fix belongs with whoever owns the landing program in
`crates/happenstance/src/lib.rs` once `initiative/from-contract-to-published-library`
merges.** That branch was merged *forward* into `initiative/docs-that-teach` at `a5c0f30`
so this project could author against the real typed layer, but it has not merged to
`main`, so HS-P0016 is not a destination reachable from this branch. That is precisely
why the overages needed an item of their own rather than a prose sentence naming a
project on another branch — flipping an acceptance row against an unreachable owner is
the defect `_conditions.md:229-232` was written to refuse.

**Opened 2026-08-19** by `/redkiln:implement`, at the human's direction, to unblock
`HS-S0185`'s AC-007. The parent project `HS-P0026` was created in the same act for the
same reason: `redkiln new` will not place a bug directly under an initiative, and
`support` (`HS-I0005`) carried no projects.
