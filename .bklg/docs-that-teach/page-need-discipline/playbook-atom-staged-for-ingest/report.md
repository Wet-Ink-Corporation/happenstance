---
item: "HS-S0153"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

## Findings Ledger

**Outcome: delivered, eleven of eleven ACs satisfied, none blocked and none deferred as behaviour.**
The two walker residuals this report first carried have been discharged by a non-author at review,
and one AC — **AC-008** — was found unmet at review, repaired, and re-verified. Both are below.

**Mount point.** `.kb/_intake/lesson-page-need-declaration-discipline.md` — 8,161 bytes, sitting
inside `/redkiln:kb-ingest`'s default glob. There is no code composition root here, and inventing
one would be the anti-pattern: `redkiln validate --kb` is blind to this tree on purpose, so presence
in the glob's directory *is* the mount and the ledger's field-by-field walk is what makes it
observable.

### AC by AC

| AC | result | what proves it |
| --- | --- | --- |
| **AC-001** — exactly one payload file, swept by the default glob, registered nowhere | **met** | `git diff --name-status main -- .kb/_intake` → one `A` line (and the commit-scoped diff agrees); `ls .kb/_intake` → two files; the staged path named in full for HS-P0025's approval gate |
| **AC-002** — `kind`/`authority_tier`/`status` readable in the first six lines | **met** | `2:id:`, `3:title:`, `4:kind: playbook`, `5:status: proposed`, `6:authority_tier: guideline`, each with the reason `accepted` and `draft` lost |
| **AC-003** — a ten-row field-by-field frontmatter walk, not a green CLI run | **met** | ten rows in `implementation-report.md` §Frontmatter walk, each citing `<atom>:NN`; a grep of the AC-003 evidence string for `validate --kb` returns nothing |
| **AC-004** — a `summary` that adjudicates before the body is opened | **met** | all four elements quoted out of `<atom>:7-17`; four sentences, against corpus exemplars at five and four |
| **AC-005** — every `source_paths` entry and every id resolves | **met** | eight paths listed and returned with no error; four ids each resolving to one canonical atom |
| **AC-006** — method, evidence and stop conditions all visible in document order | **met** | `rg -n '<details>\|<summary>\|<!--'` → exit 1; `## Where this is the wrong instrument` at `<atom>:123`, two conditions quoted verbatim |
| **AC-007** — five rejection families, each cited, no sixth invented | **met, non-author walked** | five rows, each with an atom line and a `_design.md` line cite: `:82`/`128-147`, `:89`/`188-202`, `:96`/`231-237`, `:100`/`275-280`, `:107`/`88-96`; walked by the adversarial slice reviewer, verdict **PASS** |
| **AC-008** — no commitment filed as a playbook, and no cited rule restated | **repaired at review, now met** | `rg -n '\b(must\|shall\|MUST)\b' <atom>` → **zero hits**; **ten** `RP-NN-N` ids now cited and each diffed against the shipped rule text — all cites, no restatement; hedge sweep also exit 1 |
| **AC-009** — the subject is the shape, with a recurrence outside this initiative, describing the shipped tree | **met, non-author walked** | `## What makes it portable` at `<atom>:113`; RS-81-1 and `kb-playbook-verify-referent-report-coverage-001` named; portability verdict **PASS** by the adversarial slice reviewer, identity recorded; §Divergence lists **three** real divergences rather than "none observed" |
| **AC-010** — nothing landed in the checked KB, nothing durable points at the file | **met** | `git diff --stat main -- .kb ':!.kb/_intake'` empty (commit-scoped diff agrees); `git diff --stat main -- .kb/maps` empty; `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` exit 1; `validate --kb` passed and `doctor` reports exactly six template advisories |
| **AC-011** — composed in the corpus's playbook shape, inside the measured envelope | **met, and red twice** | `wc -c` → **8,161** ≤ 8,192 (first draft 10,324, first merge 8,184); max column **91** ≤ 100 (first draft 101); six headings in the corpus's order; `rg -n '^> \*\*Answers:\*\*'` → exit 1 |

### What the review pass changed

**AC-008 was found unmet, and it was a real defect rather than a paperwork one.** The atom cited
`standards/pages/*.md` paths but not one `RP-NN-N` id, so AC-008's second clause — *every `RP-` id in
the atom checked against the shipped rule text for restatement* — had nothing to run over. It was
hiding a genuine restatement: RP-00-2's own italicised reader's test, reproduced verbatim from
`standards/pages/00-one-need.md:52-53`, with RP-00-2's imperative paraphrased in the same paragraph.
Ten ids are now cited (`RP-00-1`, `RP-00-2`, `RP-10-1`, `RP-10-4`, `RP-20-1` … `RP-20-4`, `RP-40-1`,
`RP-40-2`), the restated sentence and the paraphrase are gone, and the ten-row audit AC-008 asks for
is in `implementation-report.md` §Commitment audit.

**The two walker residuals are discharged.** AC-007's rejection cross-walk and AC-009's portability
walk both call for a **non-author**; both were re-run by the adversarial slice reviewer for
`binding-beyond-this-project` on 2026-08-18 — an actor that drafted no part of the atom, the ledger
or the report — and both **PASS**. The author-run captures have been dropped from the ledger rows and
kept in `implementation-report.md` under **What this replaced, and why**, exactly as the precedent one
slice back does it (`reviewer-and-citation-procedures/_ledger.md:84`).

### What a reviewer should push on

1. **The headroom, which the review pass has now spent once.** `<atom>` is 8,161 bytes against an
   8,192 ceiling. The first draft was 10,324 bytes and 101 columns; three compression passes brought
   it to 8,184 / 92 — **8 bytes of headroom**, which the report warned was a re-measurement rather
   than a typo fix for whoever edited next. Adding the ten rule citations proved that warning
   correct and cost a fourth compression pass. Both budgets were re-measured from scratch.
2. **§Divergence is not empty, and that is the point.** Three places where the shipped tree and
   `_design.md` disagree are recorded rather than reconciled in prose, per
   `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the fold-mechanism list is a table
   and deliberately not a `const`; the router's link check is `lint_pages.rs`'s own copy rather than
   `lint_constitution.rs`'s; and the governed set excludes `README.md`, which the design does not
   discuss and which the slice-mate observed.
3. **`status: proposed`, not `accepted`.** Acceptance is the ingest wave's to confer. Asserting it in
   the one directory nothing validates would claim a state no gate granted.

### Boundary promises

`git diff --stat main -- .kb ':!.kb/_intake'` is empty on both the `main` base and the commit-scoped
base, and `.kb/maps/` is untouched on both, so EC-001's
failure — the one this repository already made and reverted at `0269720` — did not recur. No ingest
was run; `.kb/_intake/` is deliberately non-empty at merge, which is the desired state. Nothing
under `standards/pages/**` or `xtask/**` moved. `.kb/_intake/` holds exactly two files, so
HS-P0025's approval gate stays a one-line decision.

### What this discharges

Project **AC-012** in full — its mechanical-negative half by AC-010, its frontmatter half by AC-002 +
AC-003 + AC-005, and its unstated-but-real *is this actually a playbook* half by AC-004, AC-006,
AC-007, AC-008, AC-009 and AC-011. Initiative **AC-13** is satisfied directly: someone writing the
next page has a written discipline to cite rather than this initiative's output to reverse-engineer.
No initiative Definition-of-Done scenario is claimed by this story on its own; it is the payload the
closeout ingest wave carries when HS-P0025 runs DoD-15.
