---
item: "HS-S0149"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The cite-never-restate rule, the non-author verdict walk, and the paraphrase spot check

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Four standing obligations specific to this story, all stated in `spec.md` and none a licence to
soften a row:

- **The static tier is empty in this PR by construction** (`spec.md`, Tests and CI, `static` row;
  Clarification 3). The PR boundary forbids touching `xtask/src/**` — `git diff main -- xtask` is a
  merge condition — so every AC's *present* verification is a mechanical `rg` / `awk` / `wc` / `sed`
  / `ls` / `test -f` invocation or a recorded procedural walk. Paste the command's **output**, never
  a claim about it (NF-004).
- **Every procedural record names the person, the date and the corpus** (NF-005). "A reviewer ran
  it" is not evidence. For the calibration walk the walker must be **someone other than the author**
  of band `40`, or the row may not be flipped (EC-005).
- **Three runs are recorded, not two** (Clarification 6): the governed-set walk as **vacuous**
  (AC-006), the calibration walk against the fixture to **`fail — two needs`** (AC-007), and the
  paraphrase spot check over **`standards/pages/**`** (AC-008). Each names its corpus, so no later
  reader mistakes any of the three for a pass over the narrative set — which HS-P0025 re-observes.
- **`cargo xtask affected --base main` is expected to widen to the whole workspace and to be green**
  on this prose-only diff (EC-010). Record the widening as observed; do not "fix" it by editing
  `xtask/src/affected.rs`.

```yaml
- id: AC-001
  criterion: "**GIVEN** the next page author, mid-sentence, about to write *\"a write re-reads what it decided on\"* and unsure whether to explain it or point at it, **WHEN** they open band `30` from the router, **THEN** they meet a **falsifiable test** rather than a preference — *could a conformant adapter written in another language violate this sentence? yes → it is a clause, and the page cites it* — **AND** the atom tells them the citation is written as the **stable clause id as visible link text**, never a line number and never a paraphrase, so a later reader can see they are being handed to `spec/SPECIFICATION.md`."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the `discipline-router` composition root, band `30` row in the `## Start here` filter and in the generated index, reaching standards/pages/30-citing-the-specification.md"
  verifying_test: "mechanical, captured verbatim: `test -f standards/pages/30-citing-the-specification.md`; `rg -n '^# 30 — ' standards/pages/30-citing-the-specification.md` (one hit); `rg -ni 'never restate' <atom>` (>= 1); `rg -ni 'visible link text' <atom>` (>= 1); the stability sentence citing `spec/SPECIFICATION.md:280`. Plus the procedural author-at-a-sentence walk: three sentences (normative, explanatory, mixed), the atom's test applied to each from the atom's text alone, three verdicts recorded."

- id: AC-002
  criterion: "**GIVEN** a contributor who assumes that because this repository has a gate, the gate is watching citations, **WHEN** they read band `30` from the top, **THEN** the atom tells them **first**, before any rule, what nothing checks: that whether a cited id *resolves* is HS-P0020's `clause_ids` and not this tree's, and that whether a page **restates a clause it correctly cites** is checked by **nothing mechanical** — so they leave knowing a green gate is not a claim about paraphrase, and knowing that band `40`'s spot check is the instrument for the rest."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/30-citing-the-specification.md — the blind-spot section above the first `## RP-30-1` rule, reached from standards/pages/README.md"
  verifying_test: "mechanical: the blind-spot statement's first line number is lower than `rg -n '^## RP-30-1' standards/pages/30-citing-the-specification.md`, both numbers captured; `rg -n 'clause_ids' <atom>` (>= 1). Procedural: a reader who has not seen the spec reads only that section and states, in their own words, the two facts and which project owns each half; recorded verbatim. Bar: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`), precedent sentence at `xtask/src/lint_constitution.rs:20-21`."

- id: AC-003
  criterion: "**GIVEN** the non-author reviewer, handed a page and asked \"is this one need or two?\", who has no access to the author and will not read the page's git history, **WHEN** they follow the router to `standards/pages/40-reviewing-a-page.md#the-walk`, **THEN** the fragment **resolves to a heading**, and before the first step they are told who performs the walk (**not the author**) and exactly what they may consult (the rendered page, the router, `spec/SPECIFICATION.md`) and what they may **not** (the author; the page's git history) — so the check is independent of the author's memory, which is the whole subject of DoD-8."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — band `40` row in both mount regions, reaching standards/pages/40-reviewing-a-page.md#the-walk"
  verifying_test: "mechanical: `test -f standards/pages/40-reviewing-a-page.md`; `rg -n '^### The walk$' <atom>` (exactly one hit) whose line number falls between `^## RP-40-1` and `^## RP-40-2`, all three numbers captured (parser safety: `xtask/src/lint_constitution.rs:262,275-279`, proven by `rules_are_split_at_the_next_heading` at `:870-877`); `rg -ni 'not the author' <atom>` (>= 1); `rg -ni 'git history|git log' <atom>` (>= 1, in a forbidding sentence). Procedural: the router -> anchor traverse run keyboard-only, recorded as `router row -> file -> #the-walk`."

- id: AC-004
  criterion: "**GIVEN** that same reviewer, who is a stranger to the page and to this project, **WHEN** they execute the walk top to bottom **from the rendered page alone**, **THEN** every step is a **question with a yes/no answer and a stated consequence**, numbered and executed in order — **AND** no step contains \"consider\", \"use judgement\", \"as appropriate\" or \"if it seems\", so two different strangers walking the same page reach the same verdict rather than two impressions."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/40-reviewing-a-page.md — the `### The walk` ordered list inside `## RP-40-1`, reached from standards/pages/README.md"
  verifying_test: "mechanical, scoped to the walk region: `awk '/^### The walk$/,/^## RP-40-2/' standards/pages/40-reviewing-a-page.md | rg -ni 'consider|judgement|judgment|as appropriate|if it seems'` -> **no output** (_design.md anti-pattern 15); the same extraction piped to `rg -c '^[0-9]+\\. '` (>= 3) with the extracted block pasted here verbatim, every step line ending in `?`. Procedural: two people who did not author the fixture each walk `standards/pages/examples/two-needs.md`, their step-by-step yes/no answers recorded and compared; any divergence recorded as a defect in the step."

- id: AC-005
  criterion: "**GIVEN** a reviewer who has finished the steps and now has to write something down that another person can act on, **WHEN** they reach the end of the walk, **THEN** it closes in a **four-row verdict table** — `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate` — each row saying what the verdict means and what happens next; **AND** `indeterminate` is defined as *the walk could not be completed from the page alone*, recorded as **a defect in the page, not in the procedure**, so a reviewer never has a soft pass available to them."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/40-reviewing-a-page.md — the verdict table closing `## RP-40-1`, reached from standards/pages/README.md"
  verifying_test: "mechanical: `awk '/^\\| *Verdict/,/^$/' standards/pages/40-reviewing-a-page.md | tail -n +3 | wc -l` -> `4`; all four literals present via `rg -n 'fail — two needs|fail — need not answered|indeterminate' <atom>` plus the `pass` row, outputs captured; `rg -ni 'defect in the page' <atom>` (>= 1). Procedural: the AC-007 calibration walk lands on exactly one of the four rows and that row's stated \"what happens next\" is executed or recorded as not-executed with a reason. Authority for four rather than three: `_design.md` `## States` row S5 and `## Mock` finding 3."

- id: AC-006
  criterion: "**GIVEN** the first person to run this walk over the governed set — which at this merge is **empty**, because no narrative tree is merged in this worktree — **WHEN** they find nothing to review, **THEN** the procedure's own text instructs them to record the walk as **vacuous**, in those words, and **never** as a pass; **AND** the sweep actually performed for this story is recorded in `_ledger.md` as vacuous with the corpus and date named, so no later reader can mistake an empty walk for a clean one."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/40-reviewing-a-page.md — the empty-set instruction inside `## RP-40-1`, and this ledger's own governed-set sweep row"
  verifying_test: "mechanical: `rg -ni 'vacuous' standards/pages/40-reviewing-a-page.md` (>= 1) with the surrounding sentence captured verbatim, and it must read as an instruction to the reviewer rather than a note about the tree; the emptiness itself captured via `ls docs/*.md` and `docs/README.md:1-6`. Procedural: one record reading, in substance, \"governed-set walk, <date>, corpus: none — no governed page tree exists at this merge; verdict: vacuous, not pass\". The failure this forecloses is `RUNBOOK.md:920-925`'s."

- id: AC-007
  criterion: "**GIVEN** that a procedure which has never returned `fail` is decorative — the corollary `CLAUDE.md` states for conformance rules, applied to a written one — **WHEN** a **named person who did not author it** executes the walk against `standards/pages/examples/two-needs.md`, a specimen page carrying two declarations and linked from `## RP-40-1` as the worked example, **THEN** they reach `fail — two needs`, and both the walker's identity (not the author) and the verdict are recorded in `_ledger.md`; **AND** the fixture is deliberately inert — outside the governed page tree and outside the atom namespace — so it can be permanently broken without ever making the gate red."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/40-reviewing-a-page.md `## RP-40-1` — the worked-example link (the one opened-on-demand control in this diff), reaching standards/pages/examples/two-needs.md"
  verifying_test: "end-to-end/fixture plus procedural. Mechanical: `test -f standards/pages/examples/two-needs.md`; `rg -c '^> \\*\\*Answers:\\*\\*' standards/pages/examples/two-needs.md` -> `2`; `rg -n 'examples/two-needs\\.md' standards/pages/40-reviewing-a-page.md` (>= 1, inside `## RP-40-1`); `ls standards/pages/*.md` does **not** list it, output captured (non-recursive corpus reader, `xtask/src/lint_constitution.rs:208-220`). Procedural: the named walker, the date, the step-by-step answers and the verdict `fail — two needs`, recorded verbatim per UX-009 (`_decomposition.md:526-532`) and the procedural tier (`:785-791`, `:913-923`). An author-run walk is inadmissible (EC-005)."

- id: AC-008
  criterion: "**GIVEN** the initiative's own risk that *the discipline becomes a second specification* (project `project.md`, Risks row 4), **WHEN** the paraphrase spot check `## RP-40-2` is run once over the only prose in this repository that currently makes normative claims and cites clause ids — `standards/pages/**`, this discipline's own tree — **THEN** it yields a recorded verdict per file, stating for each normative sentence whether its authority is a **visible, resolving clause id** or a **restatement**; **AND** the verdict is recorded as a check over the **rules tree**, explicitly not over the narrative set, whose walk HS-P0025 re-observes — **AND** no second clause-id parser was built to do it."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/40-reviewing-a-page.md `## RP-40-2` — the spot-check procedure, reached from standards/pages/README.md; its corpus at this merge is standards/pages/**"
  verifying_test: "gate-state: `git diff main -- xtask` -> **empty** and `git diff main -- spec/` -> **empty**, both outputs pasted here. Mechanical: `rg -n '^## RP-40-2' standards/pages/40-reviewing-a-page.md` (one hit) with a **Do** section naming the corpus and the per-sentence question. Procedural: the spot check run file by file over `standards/pages/*.md`, each file's verdict recorded with the sentence count examined; the resolution half deferred to HS-P0020's `clause_ids` (sibling shape at `xtask/src/spec_trace.rs:1746`) per architecture brief AC-010 (`_decomposition.md:120-129`) and Testing brief AC-010 (`:924-936`)."

- id: AC-009
  criterion: "**GIVEN** the page author who will load one of these atoms — never the tree — on the task where they need it, **WHEN** either atom is measured rather than asserted, **THEN** it is real composed presentation in this repository's own grammar and inside its budget: `# NN — Title` · a **one-source-line** `> **Load when:**` · `> **See also:**` · `---` · then `## RP-NN-N. <imperative sentence>` per rule with **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that order — ≤ **6** rules, ≤ **16,384** bytes, ≤ **96** columns outside tables, every fence tagged `text` or `markdown`, every `Rejects.` ≥ **120** characters, and **no rule missing its `Not` or its `Rejects.`**, because an atom that never shows a wrong page is decorative."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/30-citing-the-specification.md and standards/pages/40-reviewing-a-page.md — both instances of the `rule-atom` surface, reached from standards/pages/README.md"
  verifying_test: "mechanical, every number's output pasted here: `wc -c < <atom>` (<= 16384, each); `rg -c '^## RP-30-' <atom>` and `rg -c '^## RP-40-' <atom>` (<= 6, each); `awk '!/^\\|/ && length > 96 {print FILENAME\":\"FNR\": \"length}' <both atoms>` -> no output; `rg -n '^\\*\\*Why\\.\\*\\*|^\\*\\*Do\\*\\*|^\\*\\*Not\\*\\*|^\\*\\*Rejects\\.\\*\\*|^\\*\\*Evidence\\.\\*\\*' <atom>` prints the five markers in `SECTIONS` order once per rule (`xtask/src/lint_constitution.rs:67-81`, `check_shape` at `:477-509`); `rg -n -A1 'Load when:' <atom>` (following line does not begin with `>`, `:247-255`); a grep for a rust-tagged fence opener at line start under standards/pages/ and a grep for a bare untagged fence opener at line start under standards/pages/ -> no matches for either (anti-pattern 13; architecture brief Note 4); each `**Rejects.**` paragraph's character count captured against `MIN_REJECTS_CHARS = 120` (`:82`)."

- id: AC-010
  criterion: "**GIVEN** any of these readers arriving at `standards/pages/README.md` — the tree's only composition root — **WHEN** the router first loads with **nothing clicked**, **THEN** both new atoms are reachable from **both** mount regions: one row each in the generated index between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, byte-identical to what `generated_region`'s analogue will emit, and one `## Start here` row each inside the ≤ **12**-row ceiling; **AND** every `.md` link added by this PR resolves; **AND** nothing anywhere in the diff — atoms, fixture, router rows — sits behind a `<details>`, a tab, an accordion or a bespoke navigation widget, and no meaning is carried by colour, an icon or size, so the walk's steps and its verdict table are present on first load and the only opened-on-demand control is the link to the fixture."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — both mount regions: the `## Start here` filter table and the block between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`"
  verifying_test: "mechanical: `sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md | grep -c '^|'` minus 2 equals `ls standards/pages/*.md | grep -v README | wc -l`, both numbers captured before and after; a written derivation record mapping each new row to link form, first `Load when` line and comma-separated rule ids per `xtask/src/lint_constitution.rs:400-420`, every interior pipe escaped, no blank line inside the markers; `awk '/^## Start here/,/^## Index/' standards/pages/README.md | grep -c '^|'` (<= 14); every added `.md` target passes `test -f` (the check `:343-356` makes permanent); `rg -n '<details>|<summary>|role=\"tab\"|\\{\\{#tab' standards/pages/` and `rg -n '<small>|<sub>|<sup>|<nav>|<img' standards/pages/` -> no matches. Procedural: read all three new files through `less` with no renderer, confirming every walk step, the verdict table and both blind-spot statements are legible; then `git checkout -- standards/pages && git status` -> clean, no residue."
```
