---
item: "HS-S0151"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The declaration check watched failing, and recovering

## Findings Ledger

All eight ACs are satisfied by a recorded, reproducible procedure — five full `cargo xtask ci` runs
at one commit, captured verbatim into `_ledger.md`. Nothing was asserted in place of an observation,
nothing was paraphrased, and no `#[test]` stands in for any row. Zero lines of Rust were written
(NF-002); the breaks were working-tree edits reverted inside the run, and **no broken page reached
any commit**.

| AC | Result | Proved by | Mount point |
| -- | ------ | --------- | ----------- |
| AC-001 | **Met.** Green baseline with a **non-zero** page count — `  2 pages, 16 rules, all consistent` — so the before-state is attributable and EC-001 did not fire. | Capture 0, `_ledger.md`; exit 0, `all checks passed`, HEAD `ee0a500` | `xtask/src/main.rs:567` (`REQUIRED`), reached by `run_ci` |
| AC-002 | **Met.** `  docs/append-conditions.md:4 — declares `explanation` and `how-to`; a page answers one need` — file **and** the line of the *second* declaration, both tokens, remedy in the same line. | Capture 1; exit 1, failing on the page-need step by name | same |
| AC-003 | **Met.** `  docs/text-fences.md:3 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md` — file, line, offending token, the enumerated set, and where the set is argued. | Capture 2, taken on a tree confirmed clean by `git status --porcelain` | same |
| AC-004 | **Met.** Three problems, **one** invocation, one block sorted path → line → message, `bail!` count `3`, no truncation and no `and others`. | Capture 3 | same |
| AC-005 | **Met.** `git checkout --` → exit 0, `all checks passed` → `git status --porcelain` prints nothing. No cache, no `--write` residue, no manual step; EC-005 did not fire. | Capture 4, plus the interstitial reverts recorded between captures 1→2 and 2→3 | same |
| AC-006 | **Met.** Five capture blocks, each carrying the exact command line, the exit status, the step's own name line, the problem lines unedited, and `git rev-parse HEAD`. Other steps elided only with an explicit marker, never inside the page-need block. This story's diff is confined to its own directory plus one bookkeeping line in the slice-mate's report. | `_ledger.md` `## Captures`; `git status --porcelain` empty before the commit | — |
| AC-007 | **Met, with the overrun measured and routed.** Every problem line begins `path:line — ` except the one file-level form `_design.md` `## Composition` S4's own sample permits for a missing declaration; the repair is inside the line in every case and appears in no footer. **Longest: 141 characters** (139 without the gate indent), against the ≤ 100 budget whose finding 1 predicted 112. | The measured-length table in `_ledger.md` | — |
| AC-008 | **Met.** Captures 0 and 4 show the step's own name line and a success line carrying a count; all five show no per-file progress, no spinner, no box drawing, no summary and no `next steps`. Colour disabled (NF-003), so no ANSI escape reaches the ledger. | All five captures, read against `design/mock.html`'s `green`, `single-problem` and `many-problems` frames | — |

**Project AC-007 discharged.** Both failing halves observed (two declarations; an unenumerated
`reference`), the recovery observed, and all three recorded verbatim rather than asserted — which is
the project DoD's own wording. The initiative's **DoD scenario 2** moves to green on its page-need
half; the *"a claim is no longer true of the library"* half remains HS-P0020's.

**Nothing routed under EC-004.** Every captured message named its file, named its line where an
offending line exists, and carried its repair inside the line. No defect landed back in the
slice-mate's boundary and the observation was not re-run.

**Two things a reviewer should look at, stated rather than buried.**

1. **Capture 3 induces three failure modes across two paths, not three.** The pinned pages tree holds
   exactly two governed pages. `spec.md`'s implementation notes anticipate this and require the
   constraint to be *recorded* rather than worked around by authoring a page into HS-P0020's tree, so
   `docs/append-conditions.md` carries two of the three wrongs (a second declaration, and that second
   one naming `reference`) and `docs/text-fences.md` carries the third (no declaration). The report-all
   property is fully exercised — including the harder half of the ordering, since the two same-line
   problems break their tie on the message text (EC-011).
2. **The ≤ 100-character budget is breached by 41 characters and is not amended here.** The cause is
   concrete and is written down so a later amendment has something to decide: the slice-mate's AC-004
   requires the offending token, the whole enumerated set *and* the atom pointer in one line, while the
   density budget's yield order forbids cutting the location or the repair pointer. Under the criteria
   as they stand there is nothing left to shorten. Decision 9 and the PR boundary forbid amending
   `_design.md` on the strength of a measurement, so the number is recorded and the finding routed.

**Not blocked.** EC-001 was the story's top risk and it did not fire, because the slice-mate filled
the `*<!-- answered-need: reserved for HS-P0021 -->*` slot HS-P0020's own design reserved for this
project. Had it not, this story would have halted here with a block against `depends_on` rather than
fabricating a green.
