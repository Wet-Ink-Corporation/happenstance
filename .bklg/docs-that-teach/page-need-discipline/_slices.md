---
item: HS-P0021
stage: implementation
created: 2026-08-18T16:05:42.265Z
updated: 2026-08-18T16:05:42.265Z
template_sig: 4c5f37d6
rendered_sig: 0b76f6c1
---

# Slice ledger — Page-Need Discipline

The review verdict of every vertical slice of this project, sealed as it closed.

A slice's stories commit BEFORE its adversarial review runs, so "committed" is not "approved".
This ledger is the second axis: it records which slices actually cleared review, so a re-launched
implement run re-reviews a rejected slice instead of walking past it as finished. Each row is sealed
by a commit carrying a `Slice-Verdict: <project-slug>/<slice> <verdict>` trailer — the ledger is the
human-readable record, the trailer is what resume greps.

## Verdicts

| Slice | Verdict | Story checkpoints | Sealed by |
| ----- | ------- | ----------------- | --------- |
| discipline-on-disk | changes-requested | need-vocabulary-and-declaration-form@55b987b, router-precedence-and-announcement@9dacc7d, fold-line-rule@dec82c7, reviewer-and-citation-procedures@a349e04 | (this commit) |

## Surviving findings

For each slice whose verdict is `changes-requested`, the findings that survived the in-slice fix
pass, with the `file:line` evidence the reviewer cited. These are the prescription a resumed run —
or a human — starts from. They are hypotheses for the next reviewer to verify, not facts to trust.

### discipline-on-disk

- **no-op-seam** — `xtask/src/lint_pages.rs:1689-1700` — the untagged-fence half of
  `no_rust_tagged_and_no_untagged_fence_in_the_rules_tree` counts lines equal to a bare triple-backtick
  fence marker and asserts `openers % 2 == 0`. Those lines are the CLOSERS of correctly tagged fences,
  so an untagged fence adds one opener and one closer and parity never changes. Proved by mutation: a
  bare triple-backtick block appended to `standards/pages/40-reviewing-a-page.md` left all 51
  `lint_pages` tests green, while a `rust`-tagged fence in the same place went red at `:1681` with a
  file:line message. `need-vocabulary-and-declaration-form` AC-014 names the untagged half explicitly
  and names this test as its verification;
  `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/_ledger.md` AC-014
  flips the row citing exactly this parity reasoning ("an even number of closers matching four tagged
  openers").
