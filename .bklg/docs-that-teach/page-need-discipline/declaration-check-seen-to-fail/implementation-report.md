---
item: "HS-S0151"
stage: implement
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Implementation Report — The declaration check watched failing, and recovering

## TDD Evidence

This story ships **zero lines of Rust** (NF-002), and its testing tier is *procedural
(ledger-recorded)* rather than `#[test]` — deliberately, because a unit test that calls the
checker's own functions never invokes `cargo xtask ci` and therefore says nothing about whether the
step is in `REQUIRED`. So the Red/Green here is not a test suite; it is the **gate itself**, watched
going red on purpose and recovering, five full `cargo xtask ci` runs of record.

The Red is real and it is the point: the same command that printed `all checks passed` in capture 0
printed `xtask failed: every page declares one need failed with exit code: 1` in captures 1, 2 and
3, each time naming the file and — for the two cases where an offending line exists — the line. The
Green is capture 4, reached by `git checkout --` alone.

| AC | Instrument | Red → Green |
| -- | ---------- | ----------- |
| AC-001 | Capture 0, `cargo xtask ci` on a clean tree at `ee0a500` | GREEN before anything was touched. `=== every page declares one need ===` then `  2 pages, 16 rules, all consistent` — a **non-zero** count, so EC-001's vacuous-tree halt did not fire and the before-state is attributable. |
| AC-002 | Capture 1, Break A | RED, exit 1, on the page-need step **by name**: `  docs/append-conditions.md:4 — declares `explanation` and `how-to`; a page answers one need`. Line 4 is the *second* declaration, not the first. |
| AC-003 | Capture 2, Break B, on a tree confirmed clean first | RED, exit 1: `  docs/text-fences.md:3 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md`. File, line, offending token, the whole set, and the atom where the set is argued. |
| AC-004 | Capture 3, three breaks, **one** invocation | RED, exit 1, **three** problems in one block, sorted path → line → message, closed by `xtask failed: 3 problem(s) in standards/pages + docs`. No truncation, no `and others`. |
| AC-005 | Capture 4, `git checkout --` then re-run | GREEN, exit 0, `all checks passed`, and `git status --porcelain` printing nothing. No cache, no `--write` residue, no manual step. |
| AC-006 | The five capture blocks in `_ledger.md` | Each carries the exact command line, the exit status, the step's own name line, the problem lines unedited, and `git rev-parse HEAD`. Other steps elided only with an explicit marker, never inside the page-need block. |
| AC-007 | `awk '{ print length }'` over each capture's problem block | Every line begins `path:line — ` except the one file-level form the design permits for a missing declaration. Longest: **141 characters** (139 without the gate indent). Recorded and routed. |
| AC-008 | Read across all five captures | The step is visible in the two green ones (name + count) and carries nothing else in any of the five: no progress, no spinner, no box drawing, no summary, no `next steps`. |

**Nothing was routed under EC-004.** Every captured message named its file, named its line where an
offending line exists, carried its repair inside the line, and was actionable without opening a
second document. The observation therefore did not have to be re-run, and no defect landed back in
the slice-mate's boundary.

## Commits

| SHA | Subject |
| --- | ------- |
| `ee0a500` | `feat(page-need-discipline): Page-need checker mounted in the gate` — the slice-mate. Every capture in this story was taken at this commit; it is the `git rev-parse HEAD` printed in all five blocks. |
| `PENDING` | `feat(page-need-discipline): Declaration check seen to fail` — this story's own checkpoint, carrying `_ledger.md`, this report and the report-stage artifact. |

The second row's SHA cannot be written inside the commit it names; it is reported in the slice
digest and is findable with
`git log --grep "Story: page-need-discipline/declaration-check-seen-to-fail"`.

## Changes

| File | Shape of the change |
| ---- | ------------------- |
| `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md` | Eight rows flipped to `satisfied: true`, each pointing at the capture that discharges it; the `## Captures` section filled with five verbatim blocks and the measured-line-length table. |
| `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/implementation-report.md` | This file. |
| `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/report.md` | The report-stage artifact's `## Findings Ledger`. |
| `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/implementation-report.md` | One bookkeeping line: the slice-mate's `## Commits` row now carries its real SHA, which could not be written inside the commit it names. |

**No Rust. No page. No rule atom. No router edit.** Every break was a working-tree edit made,
observed and reverted inside the run; `git status --porcelain` was empty before this story's commit
and `git diff main --stat` shows no `docs/` change beyond the two declarations the slice-mate landed.

## Gates

| Command | Result |
| ------- | ------ |
| `CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci` × 5 | captures 0-4: green, red, red, red, green — exactly as the procedure requires |
| `git status --porcelain` after every revert and before the commit | empty |
| `git rev-parse HEAD` at every capture | `ee0a5000955bbe7230874ce5897e46eee99d863a`, unchanged throughout |
| `cargo run --locked --quiet -p xtask -- affected --base main` | `affected gate passed`; this story's `.bklg/`-only diff selects no package, and the checker still ran under `=== the file-reading checks ===` — NF-004's incidental second observation |
| `redkiln doctor` · `redkiln validate --kb` | no problems, exactly six `template-drift` advisories; `validate passed` |

## Notes

**One constraint recorded rather than worked around.** UX-007's stated test is *three pages, three
ways, one run*. The pinned pages tree holds exactly **two** governed pages. The spec's own
implementation notes anticipated this — *"break the same page in sequence rather than inventing
pages … a directory with fewer than three pages is EC-001 territory in miniature — record the
constraint rather than manufacturing pages around it"* — so capture 3 induces the three ways across
two paths: `docs/append-conditions.md` carries a second declaration **and** that second declaration
names `reference`, and `docs/text-fences.md` loses its declaration entirely. Three problems, three
distinct failure modes, one run, and no page authored into HS-P0020's tree to reach a third path.
What the two-page tree cannot demonstrate is three *distinct* paths in the sort; the sort is still
exercised, because the two same-line problems on one path break their tie on the message text
(EC-011), which is the harder half of the ordering property.

**EC-001 did not fire, and the reason is the slice-mate's second deviation.** Before the slice-mate
landed, the pinned pages tree held no *governed* page at all — both pages carried
`*<!-- answered-need: reserved for HS-P0021 -->*`, the slot HS-P0020's own design reserved for this
project. Had that reservation been left unfilled, this story would have halted here under EC-001
with a block against `depends_on`, because there would have been no non-vacuous green to return to.
It was filled in the slice-mate's boundary, where it belongs, and this story observed the result
rather than manufacturing it.

**The 100-character budget is breached and routed (Decision 9).** 141 characters measured against a
100-character budget whose own finding 1 predicted 112. The cause is stated so a later amendment has
something concrete to decide about: the slice-mate's AC-004 requires the offending token, the
**whole** enumerated set, and the atom pointer in one line, while the density budget's yield order
forbids cutting the location or the repair pointer — so under the criteria as written there is
nothing left to shorten. The number is in `_ledger.md`; `_design.md` is untouched.

**Five full gate runs, within NF-005's budget.** One flake was seen during the slice's earlier
affected run (a merged-doctest failure in `standards/rust/12-manual-impls-and-derive-traps.md` that
did not reproduce) and none during these five; every capture of record is a full `cargo xtask ci`,
never the step alone.
