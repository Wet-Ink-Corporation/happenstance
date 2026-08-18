---
item: HS-S0151
stage: implement
created: 2026-08-17
updated: 2026-08-17
---

# Acceptance ledger — The declaration check watched failing, and recovering

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

**This story's evidence is captured terminal output, and it is captured verbatim.** Each `evidence`
field points at the capture block in this file that discharges the row; the capture blocks
themselves carry the exact command line, the exit status, the step's own name line, the problem lines
unedited and un-re-flowed, and `git rev-parse HEAD`. Paraphrase is the failure `RUNBOOK.md:918-928`
already cost this repository once, and line length is itself evidence (AC-007), so a re-flowed
capture is not a capture.

```yaml
- id: AC-001
  criterion: "GIVEN the slice-mate's step is in `REQUIRED` and the pinned pages tree carries at least one governed page, WHEN a maintainer who is about to trust this gate runs `cargo xtask ci` on a clean tree before touching anything, THEN the page-need step prints its own name and a success line carrying a non-zero page count (`{n} pages, {m} rules, all consistent`), the gate exits `0`, and that run plus `git rev-parse HEAD` is captured as the attributable before-state. A green captured over an empty tree is not a baseline — it is the `0 pages, all consistent` failure with this project's name on it."
  satisfied: true
  evidence: "Capture 0 in this file's `## Captures` section. `CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci` on a clean tree at HEAD `ee0a5000955bbe7230874ce5897e46eee99d863a`, exit status 0, closing `all checks passed`. The page-need step prints its own name line `=== every page declares one need ===` and the success line `  2 pages, 16 rules, all consistent` — a **non-zero** page count, so the before-state is attributable and EC-001's vacuous-tree halt did not fire. `git status --porcelain` was empty before the run."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831)"
  verifying_test: "procedural (end-to-end/fixture): capture 0 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — `cargo xtask ci` on a clean tree, success line with a non-zero page count, read against xtask/src/lint_constitution.rs:192 and :174-176"

- id: AC-002
  criterion: "GIVEN an author mid-edit who has left a second `> **Answers:**` line on a governed page — the plausible accident, not a synthetic one — WHEN they run `cargo xtask ci`, THEN the gate exits non-zero and prints a problem line that names that file and the line number of the offending declaration, says the page answers more than one need, and tells them what to do — so they can repair it without opening a second document. A red gate that names the file but not the line does not discharge this."
  satisfied: true
  evidence: "Capture 1. Break A — a second `> **Answers:** `how-to` — How do I append under a condition?` line left on `docs/append-conditions.md`. `cargo xtask ci` exit status 1, failing **on the page-need step by name** (`xtask failed: every page declares one need failed with exit code: 1`), with the problem line `  docs/append-conditions.md:4 — declares `explanation` and `how-to`; a page answers one need` — file **and** line 4, which is the offending *second* declaration and not the first, the two tokens named, and the remedy stated in the same line. No second document is needed to act on it."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831)"
  verifying_test: "procedural (end-to-end/fixture): capture 1 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — Break A applied to one governed page, `cargo xtask ci`, non-zero exit, problem line judged against .bklg/docs-that-teach/page-need-discipline/_design.md States table S1 \"Error\" row"

- id: AC-003
  criterion: "GIVEN an author who, in good faith, declares `reference` — the token DT-2 removed on purpose because rustdoc and `spec/SPECIFICATION.md` already own that surface — WHEN they run `cargo xtask ci` on an otherwise clean tree, THEN the gate fails naming the file, the line, and the offending token, and points at where the closed set is written, so the author learns the set exists rather than guessing a fifth word."
  satisfied: true
  evidence: "Capture 2. Break B — `docs/text-fences.md`'s declaration changed to `` `reference` ``, the token DT-2 removed on purpose, applied to an otherwise clean tree (`git status --porcelain` empty after Break A was reverted). `cargo xtask ci` exit status 1 on the page-need step, with `  docs/text-fences.md:3 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md` — the file, the line, the offending token, the whole enumerated set, and the atom where the set is argued. The author learns the set exists rather than guessing a fifth word."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831)"
  verifying_test: "procedural (end-to-end/fixture): capture 2 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — Break B on a clean tree, `cargo xtask ci`, judged against .bklg/docs-that-teach/page-need-discipline/_design.md \"S1 vocabulary — DT-2\" and anti-pattern 16"

- id: AC-004
  criterion: "GIVEN a review cycle in which three governed pages are wrong in three different ways at once — two declarations, an unenumerated token, and no declaration at all — WHEN the author runs `cargo xtask ci` once, THEN all three problems print in that single run, as one block sorted by path then line with no blank lines and no per-problem heading, with no truncation, no ellipsis and no \"and others\", and a `bail!` line carrying the count `3` — so one review cycle stays one cycle instead of becoming three."
  satisfied: true
  evidence: "Capture 3. Three breaks, **one** `cargo xtask ci` invocation, three problems: `  docs/append-conditions.md:4 — `reference` is not a need: …`, `  docs/append-conditions.md:4 — declares `explanation` and `reference`; a page answers one need`, and `  docs/text-fences.md — no `> **Answers:**` line; see standards/pages/00-one-need.md` — then `xtask failed: 3 problem(s) in standards/pages + docs`. One block, sorted by path then line then message (the two same-line problems break their tie on the message, EC-011), no blank line between problems, no per-problem heading, no ellipsis, no `and others`, and the `bail!` count is 3, matching the three printed lines. **Constraint recorded rather than manufactured around:** the pinned pages tree holds exactly two governed pages, so the three ways were induced across two paths — two declarations *and* an unenumerated token on `docs/append-conditions.md`, no declaration at all on `docs/text-fences.md` — per `spec.md`'s own implementation note (*\"break the same page in sequence rather than inventing pages … record the constraint rather than manufacturing pages around it\"*). No page was authored into HS-P0020's tree to reach three paths."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831)"
  verifying_test: "procedural (end-to-end/fixture): capture 3 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — Breaks A+B+C simultaneously, one `cargo xtask ci` invocation, problem-line count compared to the `bail!` count; UX-007's stated test in .bklg/docs-that-teach/page-need-discipline/_decomposition.md"

- id: AC-005
  criterion: "GIVEN an author who has seen the gate go red and now wants their tree back, WHEN they run `git checkout -- <page>` for each break and re-run `cargo xtask ci`, THEN the gate returns green and `git status --porcelain` prints nothing — no cache to clear, no generated router region left dirty, no `--write` residue, no manual step. Every state the author can enter, they can leave, in one documented command."
  satisfied: true
  evidence: "Capture 4. `git checkout -- docs/append-conditions.md docs/text-fences.md`, then `git status --porcelain` printing **nothing** (shown as an empty block, not described), then `cargo xtask ci` exit status 0 with `  2 pages, 16 rules, all consistent` and `all checks passed`. HEAD unchanged at `ee0a5000955bbe7230874ce5897e46eee99d863a`. No cache cleared, no generated router region left dirty, no `--write` residue, no manual step — EC-005 did not fire. The same revert-and-verify was performed between captures 1→2 and 2→3 and is recorded in each block."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831); git itself as the reversibility instrument"
  verifying_test: "procedural (end-to-end/fixture) + gate-state: capture 4 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — `git checkout -- <page>`, `cargo xtask ci` exit 0, and an empty `git status --porcelain` shown as an empty fenced block; UX-008's stated test"

- id: AC-006
  criterion: "GIVEN a reader of this repository six months from now who has only the ledger and no ability to re-run anything, WHEN they open `_ledger.md`, THEN they find every capture verbatim — exact command line, exit status, the step's own name line, the problem lines unedited and un-re-flowed, and `git rev-parse HEAD` for each — and the story's committed diff contains no broken page anywhere, only files under this story's own directory. A document that vouches for a check is not evidence the check runs; `RUNBOOK.md:918-928` is this repository's own receipt for that."
  satisfied: true
  evidence: "The `## Captures` section of this file. Each of the five blocks carries all five required elements: the exact command line (`CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci`), the exit status, the step's own name line, the problem lines unedited and un-re-flowed, and `git rev-parse HEAD`. Other gate steps' output is elided only with an explicit marker and never inside the page-need step's own block. The committed diff for this story is confined to `.bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/**` plus the one bookkeeping line recording the slice-mate's SHA; `git status --porcelain` was empty before the commit and **no broken page reached any commit** — every break was a working-tree edit reverted with `git checkout --` inside the run."
  mount_point: ".bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md — the capture blocks below; required by .redkiln/config.yaml:67 (`require_ledger`) and :73 (`require_commit_provenance`)"
  verifying_test: "procedural + gate-state: every capture in this file read for all five elements (command, exit status, step-name line, unedited problem lines, head sha), then `git status --porcelain` empty and `git diff main --stat` confined to .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/**"

- id: AC-007
  criterion: "GIVEN the S3 reader scanning a column of failures, WHEN they read any captured problem line, THEN it is one line beginning `{path}:{line} — `, then what is wrong, then why it matters or what to do, with the repair inside the line and never in a footer paragraph or a \"next steps\" section — location first, remedy third, never removed. The longest captured line's character count is measured and recorded against _design.md's ≤ 100-character budget, whose own finding 1 predicts 112 for a realistic path plus citation and 111 for the inherited spelling at `lint_constitution.rs:369-378`. The measurement is evidence; the budget is not amended here."
  satisfied: true
  evidence: "Read across captures 1, 2 and 3. Every problem line begins `{path}:{line} — ` except the one file-level form the design permits — `docs/text-fences.md — no `> **Answers:**` line` — which is the missing-declaration case and matches `_design.md` `## Composition` S4's own third sample line. The repair is **inside** the line in every case (`see standards/pages/10-the-need-set.md`, `see standards/pages/00-one-need.md`, `a page answers one need`); no footer paragraph, no `next steps` section, and no repair anywhere outside a problem line appears in any capture. MEASURED: longest captured problem line = **141 characters** including the two-space gate indent (**139** without it), on `  docs/append-conditions.md:4 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md` in capture 3; capture 2's longest is 135/133 and capture 1's is 92/90. Against `_design.md`'s ≤ 100-character budget, whose own finding 1 predicted 112 for a realistic path plus citation and 111 for the inherited spelling at `lint_constitution.rs:369-378`. **Routed, not settled**: the overrun is recorded here and the density budget, the citation form and the no-wrap rule are untouched (Decision 9). The overrun's cause is stated so a later amendment has something to decide about — AC-004 of the slice-mate requires the offending token, the *whole* enumerated set, and the atom pointer in one line, and the yield order forbids cutting the location or the repair pointer."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831); the message grammar inherited from xtask/src/lint_constitution.rs:169-198 and :375-379"
  verifying_test: "procedural, measured: every problem line in captures 1–3 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md read for the `path:line — ` prefix (file-level form permitted only for the missing-declaration case) and for the absence of a footer repair, with `awk '{ print length }'` over the captured block and the maximum recorded against the ≤ 100 budget in .bklg/docs-that-teach/page-need-discipline/_design.md"

- id: AC-008
  criterion: "GIVEN the same reader watching a gate that passes, WHEN they look at capture 0 and capture 4, THEN the step is visible — its own name line and a success line with a count are present — and nothing else is: no per-file progress, no spinner, no box drawing, no summary section and no \"next steps\" paragraph in any capture. A green run that printed nothing is indistinguishable from a step that did not run, which is the `RUNBOOK.md:920-925` incident in one sentence; and every decorative line is a line the third problem hides behind."
  satisfied: true
  evidence: "Captures 0 and 4 both show the step **visible**: its own name line `=== every page declares one need ===` followed by `  2 pages, 16 rules, all consistent` — a count, not silence (anti-pattern 11). Read across all five captures, the page-need step's block contains **nothing else**: no per-file progress, no spinner or cursor-control sequence, no box drawing, no summary section and no `next steps` paragraph. Colour was disabled (`CARGO_TERM_COLOR=never`, NF-003) so no ANSI escape reaches this file and the character measurement in AC-007 is true. Compared against `design/mock.html`'s `green`, `single-problem` and `many-problems` frames: the real output matches the composition — step name above, problem block with no separators, count below — and diverges only in line width, which is AC-007's recorded measurement."
  mount_point: "xtask/src/main.rs — the `REQUIRED` array at :105, reached through `cargo xtask ci` (`run_ci`, :828-831); the Step machinery that prints each step's own name"
  verifying_test: "procedural: captures 0 and 4 in .bklg/docs-that-teach/page-need-discipline/declaration-check-seen-to-fail/_ledger.md read for the step name plus a counted success line (anti-pattern 11), and all five captures read for the absence of progress output, spinners, box drawing and summary paragraphs, compared against the `green`, `single-problem` and `many-problems` frames in .bklg/docs-that-teach/page-need-discipline/design/mock.html"
```

## Captures

The implementer pastes each capture here, unmodified, and points the matching `evidence` field at it.
Five are expected; a sixth is added only if a defect is routed under EC-004 and the observation is
re-run. Elide other gate steps' output **only** with an explicit marker, and never inside the
page-need step's own block.

- **Capture 0 — green baseline, non-zero page count.** `cargo xtask ci` on a clean tree.
- **Capture 1 — Break A, two declarations on one page.** `cargo xtask ci`, non-zero.
- **Capture 2 — Break B, an unenumerated need (`reference`).** `cargo xtask ci`, non-zero.
- **Capture 3 — Breaks A + B + C together, one run, three problems.** `cargo xtask ci`, non-zero.
- **Capture 4 — recovery.** `git checkout -- <page>` per break, `cargo xtask ci` green, and
  `git status --porcelain` with empty output.

Each block carries: the exact command line, the exit status, the step's own name line, the problem
lines unedited, and `git rev-parse HEAD`. Run with colour disabled (`CARGO_TERM_COLOR=never`) per
NF-003, so no ANSI escape reaches this file and the `awk` length measurement AC-007 depends on stays
true.

### Capture 0 — green baseline, non-zero page count

```text
$ git rev-parse HEAD
ee0a5000955bbe7230874ce5897e46eee99d863a

$ git status --porcelain

$ CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci
… 20 preceding gate steps elided; the page-need step's own block is verbatim …
=== every page declares one need ===
  2 pages, 16 rules, all consistent

… 6 following gate steps elided …
all checks passed

$ echo exit=$?
exit=0
```

### Capture 1 — Break A, two declarations on one page

Break applied: a second declaration line added to `docs/append-conditions.md`, so lines 3 and 4 read

```text
> **Answers:** `explanation` — Why does a write re-read what it decided on?
> **Answers:** `how-to` — How do I append under a condition?
```

```text
$ git rev-parse HEAD
ee0a5000955bbe7230874ce5897e46eee99d863a

$ CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci
… 20 preceding gate steps elided; the page-need step's own block is verbatim …
=== every page declares one need ===
  docs/append-conditions.md:4 — declares `explanation` and `how-to`; a page answers one need

xtask failed: 1 problem(s) in standards/pages + docs

xtask failed: every page declares one need failed with exit code: 1

$ echo exit=$?
exit=1
```

Reverted before capture 2:

```text
$ git checkout -- docs/append-conditions.md

$ git status --porcelain

$ cargo run --locked --quiet -p xtask -- lint-pages
  2 pages, 16 rules, all consistent
```

### Capture 2 — Break B, an unenumerated need

Break applied: `docs/text-fences.md` line 3 changed to

```text
> **Answers:** `reference` — Which blocks on these pages does the gate never compile?
```

```text
$ git rev-parse HEAD
ee0a5000955bbe7230874ce5897e46eee99d863a

$ CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci
… 20 preceding gate steps elided; the page-need step's own block is verbatim …
=== every page declares one need ===
  docs/text-fences.md:3 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md

xtask failed: 1 problem(s) in standards/pages + docs

xtask failed: every page declares one need failed with exit code: 1

$ echo exit=$?
exit=1
```

Reverted before capture 3:

```text
$ git checkout -- docs/text-fences.md

$ git status --porcelain
```

### Capture 3 — three breaks, one run, three problems

Breaks applied together, on a tree confirmed clean first. The pinned pages tree holds exactly two
governed pages, so the three ways were induced across two paths rather than by authoring a third
page into HS-P0020's tree (`spec.md`, *Implementation notes*, "Choosing the pages to break"):

- `docs/append-conditions.md` — a second declaration, and that second one naming `` `reference` ``,
  so the page is wrong in **two** of the three ways at once;
- `docs/text-fences.md` — its declaration line deleted entirely.

```text
$ git rev-parse HEAD
ee0a5000955bbe7230874ce5897e46eee99d863a

$ CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci
… 20 preceding gate steps elided; the page-need step's own block is verbatim …
=== every page declares one need ===
  docs/append-conditions.md:4 — `reference` is not a need: orientation, tutorial, how-to, explanation; see standards/pages/10-the-need-set.md
  docs/append-conditions.md:4 — declares `explanation` and `reference`; a page answers one need
  docs/text-fences.md — no `> **Answers:**` line; see standards/pages/00-one-need.md

xtask failed: 3 problem(s) in standards/pages + docs

xtask failed: every page declares one need failed with exit code: 1

$ echo exit=$?
exit=1
```

Three problems, one invocation, one block: sorted by path, then line, then message text — the two
`docs/append-conditions.md:4` problems break their tie on the message, which is what makes the order
stable across re-runs (EC-011). No blank line between problems, no per-problem heading, no ellipsis,
no `and others`, and the `bail!` count is `3`, matching the three lines printed above it.

### Capture 4 — recovery, and no residue

```text
$ git checkout -- docs/append-conditions.md docs/text-fences.md

$ git status --porcelain

$ CARGO_TERM_COLOR=never cargo run --locked --quiet -p xtask -- ci
… 20 preceding gate steps elided; the page-need step's own block is verbatim …
=== every page declares one need ===
  2 pages, 16 rules, all consistent

… 6 following gate steps elided …
all checks passed

$ echo exit=$?
exit=0

$ git rev-parse HEAD
ee0a5000955bbe7230874ce5897e46eee99d863a
```

The `git status --porcelain` block above is empty because the command printed nothing — the
un-narrowed form, so residue anywhere in the tree (including the router's generated region) would
have shown. Nothing was cleared, nothing regenerated, no `--write` run was needed.

### Measured line length

`awk '{ print length }'` over each capture's problem block, longest first:

| Capture | Longest problem line | Without the two-space gate indent |
| ------- | -------------------- | --------------------------------- |
| 3 | **141** | 139 |
| 2 | 135 | 133 |
| 1 | 92 | 90 |

Against `_design.md`'s ≤ 100-character budget, whose finding 1 predicted 112. Recorded and routed;
the budget, the citation form and the no-wrap rule are unamended (Decision 9).

## Blocks and routed findings

Recorded here rather than resolved in this story's boundary.

- **EC-001 / EC-002 / EC-003 blocks** — if the pinned pages tree is empty or missing, or the step is
  not in `REQUIRED`, record the condition and the halt here against `depends_on:
  page-need-checker-mounted-in-the-gate`. Do not fabricate a green and do not author a page into
  HS-P0020's tree.
- **EC-004 / EC-005 routed defects** — a substandard message or revert residue is fixed under
  `page-need-checker-mounted-in-the-gate`'s boundary in the same slice context; note the defect, the
  fix, and the fact that the observation was re-run from capture 0.
- **The measured problem-line length** — recorded against the ≤ 100-character budget in
  `.bklg/docs-that-teach/page-need-discipline/_design.md`, whose finding 1 predicts the budget is
  unreachable for this tree. The number goes here; the design amendment does not (Decision 9).
