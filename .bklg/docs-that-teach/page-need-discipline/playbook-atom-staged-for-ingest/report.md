---
item: "HS-S0153"
stage: report
created: "2026-08-18T00:00:00.000Z"
updated: "2026-08-18T00:00:00.000Z"
---

# Report — The playbook atom staged under .kb/_intake/, never hand-authored into .kb/

## Findings Ledger

**Outcome: delivered, eleven of eleven ACs satisfied, none blocked and none deferred as behaviour.**
Two residuals are the *identity of a walker* rather than missing work, and both are named below.

**Mount point.** `.kb/_intake/lesson-page-need-declaration-discipline.md` — 8,184 bytes, sitting
inside `/redkiln:kb-ingest`'s default glob. There is no code composition root here, and inventing
one would be the anti-pattern: `redkiln validate --kb` is blind to this tree on purpose, so presence
in the glob's directory *is* the mount and the ledger's field-by-field walk is what makes it
observable.

### AC by AC

| AC | result | what proves it |
| --- | --- | --- |
| **AC-001** — exactly one payload file, swept by the default glob, registered nowhere | **met** | `git diff --name-status --cached HEAD -- .kb/_intake` → one `A` line; `ls .kb/_intake` → two files; the staged path named in full for HS-P0025's approval gate |
| **AC-002** — `kind`/`authority_tier`/`status` readable in the first six lines | **met** | `2:id:`, `3:title:`, `4:kind: playbook`, `5:status: proposed`, `6:authority_tier: guideline`, each with the reason `accepted` and `draft` lost |
| **AC-003** — a ten-row field-by-field frontmatter walk, not a green CLI run | **met** | ten rows in `implementation-report.md` §Frontmatter walk, each citing `<atom>:NN`; a grep of the AC-003 evidence string for `validate --kb` returns nothing |
| **AC-004** — a `summary` that adjudicates before the body is opened | **met** | all four elements quoted out of `<atom>:7-17`; four sentences, against corpus exemplars at five and four |
| **AC-005** — every `source_paths` entry and every id resolves | **met** | eight paths listed and returned with no error; four ids each resolving to one canonical atom |
| **AC-006** — method, evidence and stop conditions all visible in document order | **met** | `rg -n '<details>\|<summary>\|<!--'` → exit 1; `## Where this is the wrong instrument` at `<atom>:121`, two conditions quoted verbatim |
| **AC-007** — five rejection families, each cited, no sixth invented | **met** (walker residual) | five rows, each with an atom line and a `_design.md` line cite: `:81`/`128-147`, `:88`/`188-202`, `:95`/`231-237`, `:99`/`275-280`, `:105`/`88-96` |
| **AC-008** — no commitment filed as a playbook | **met** | `rg -n '\b(must\|shall\|MUST)\b' <atom>` → **zero hits**; the atom names no `RP-` id and so restates none; hedge sweep also exit 1 |
| **AC-009** — the subject is the shape, with a recurrence outside this initiative, describing the shipped tree | **met** (walker residual) | `## What makes it portable` at `<atom>:111`; RS-81-1 and `kb-playbook-verify-referent-report-coverage-001` named; §Divergence lists **three** real divergences rather than "none observed" |
| **AC-010** — nothing landed in the checked KB, nothing durable points at the file | **met** | `git diff HEAD --stat -- .kb ':!.kb/_intake'` empty; `git diff HEAD --stat -- .kb/maps` empty; `rg -n "_intake/lesson-page-need" -g '!.bklg/**'` exit 1; `validate --kb` passed and `doctor` reports exactly six template advisories |
| **AC-011** — composed in the corpus's playbook shape, inside the measured envelope | **met, and red first** | `wc -c` → **8,184** ≤ 8,192 (first draft 10,324); max column **92** ≤ 100 (first draft 101); six headings in the corpus's order; `rg -n '^> \*\*Answers:\*\*'` → exit 1 |

### The two residuals, stated plainly

AC-007's rejection cross-walk and AC-009's portability walk both call for a **non-author**. Both were
performed by the implementer, who drafted the atom, and both are labelled as such in the ledger and
in `implementation-report.md`. What is complete is their *content*: five families mapped to five
`_design.md` line cites with no sixth invented, and a portability verdict with a named recurrence
outside this initiative. What is owed is the *walker's independence*, at this story's review and
integration stages. This is the same residual the project recorded one slice back in
`reviewer-and-citation-procedures/_ledger.md`, and recording it is what makes the obligation visible
rather than assumed. Neither row rests on the walker's authority alone — each sits beside a
mechanical transcript.

### What a reviewer should push on

1. **The 8-byte headroom.** `<atom>` is 8,184 bytes against an 8,192 ceiling. Any later edit to this
   file is a re-measurement, not a typo fix. The overrun was fixed rather than waived: the first
   draft was 10,324 bytes and 101 columns, and three compression passes removed prose and no
   obligation.
2. **§Divergence is not empty, and that is the point.** Three places where the shipped tree and
   `_design.md` disagree are recorded rather than reconciled in prose, per
   `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the fold-mechanism list is a table
   and deliberately not a `const`; the router's link check is `lint_pages.rs`'s own copy rather than
   `lint_constitution.rs`'s; and the governed set excludes `README.md`, which the design does not
   discuss and which the slice-mate observed.
3. **`status: proposed`, not `accepted`.** Acceptance is the ingest wave's to confer. Asserting it in
   the one directory nothing validates would claim a state no gate granted.

### Boundary promises

`git diff HEAD --stat -- .kb ':!.kb/_intake'` is empty and `.kb/maps/` is untouched, so EC-001's
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
