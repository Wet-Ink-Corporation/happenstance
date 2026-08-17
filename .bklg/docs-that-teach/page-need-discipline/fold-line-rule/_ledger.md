---
item: "HS-S0148"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — DT-8 resolved as a rule a reviewer can apply without the author

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
  Clarification 2). The PR boundary forbids touching `xtask/src/**`, so every AC's *present*
  verification is a mechanical `rg` / `awk` / `wc` / `grep` / `sed` / `test -f` invocation or a
  recorded procedural walk. Paste the command's **output**, never a claim about it (NF-004).
- **Every procedural row records who walked it and what verdict they reached** — and the walker is
  **not** the author of the atom. Project AC-005's bar is *"in a form a reviewer applies to a page
  without consulting the author"*; a walk performed by the author proves the opposite of the
  criterion.
- **`PERMITTED_FOLD_MECHANISMS` ships with zero data rows** (AC-004, EC-004). A row added to make
  the table "look finished" fails AC-004 and AC-005 at once; the disagreement goes to `_design.md`
  `## Sign-off` condition 3, not into this diff.
- **`cargo xtask affected --base main` is expected to widen to the whole workspace and to be green**
  (EC-010, NF-005). Record the widening as observed; do not "fix" it by editing
  `xtask/src/affected.rs`.

```yaml
- id: AC-001
  criterion: "**GIVEN** the non-author reviewer, holding a page whose \"what this does not verify\" paragraph sits inside a `<details>` labelled *Limitations*, and who cannot ask the author what they intended, **WHEN** they load `standards/pages/20-the-fold-line.md` — one hop from the router, `# 20 — The fold line`, a one-source-line `> **Load when:**`, a `> **See also:**` naming sibling bands **by number**, then `---` — and apply **RP-20-1**, **THEN** the atom hands them the deletion test verbatim (*if the collapsed region were deleted, would the page still teach the constraint correctly?*) as a question with a **yes/no** answer and a stated consequence, and they reach a **verdict**, not an impression — with no sentence anywhere in the atom reading \"consider\", \"use judgement\", \"as appropriate\" or \"if it seems\"."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the `rule-atom` surface, reached in one hop from `standards/pages/README.md`'s `## Start here` row"
  verifying_test: "mechanical, captured verbatim: `test -f standards/pages/20-the-fold-line.md`; `rg -n '^# 20 — The fold line$' standards/pages/20-the-fold-line.md` (one hit); `rg -n -A1 'Load when:' standards/pages/20-the-fold-line.md` (the next source line does not begin with `>`); `rg -n 'See also' standards/pages/20-the-fold-line.md` (names 00, 30, 40 as numbers); `rg -n 'would the page still teach the constraint correctly' standards/pages/20-the-fold-line.md` (>= 1); `rg -in 'consider|use judge?ment|as appropriate|if it seems' standards/pages/20-the-fold-line.md` (no matches). Plus the procedural non-author verdict walk over a specimen page whose limitations paragraph sits in a `<details>` — walker, rule id and verdict all recorded."

- id: AC-002
  criterion: "**GIVEN** the same reviewer facing an edge case the deletion test alone would leave to judgement — the drift the tension names in its own words — **WHEN** they read **RP-20-2**, **THEN** they find **five** enumerated never-fold classes: the page's own `> **Answers:**` declaration; any sentence carrying a normative modal or citing a `spec/SPECIFICATION.md` clause id; any statement of an invariant, constraint or precondition; the only occurrence of a code fence the reader is expected to run; any statement of what a check does *not* verify — **AND** the atom states in its own text that **no reviewer may grant an exception** and that the list is **closed**, so an edge case grows the *mechanism* list and never shrinks the class list, growing the class list being a disagreement with `_design.md` sign-off condition 3 rather than an authoring choice."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the `## RP-20-2.` rule block inside the `rule-atom` surface"
  verifying_test: "mechanical: `awk '/^## RP-20-2\\./,/^## /' standards/pages/20-the-fold-line.md | grep -c '^[0-9]\\.'` (= 5); `rg -n 'no reviewer may grant an exception' standards/pages/20-the-fold-line.md` (>= 1); the closedness sentence present and naming `_design.md` sign-off condition 3 as the amendment path. Plus the procedural three-fragment classification (a `MUST` sentence, a clause citation, a runnable fence) by a non-author, each mapping to exactly one class."

- id: AC-003
  criterion: "**GIVEN** the page author who has just read `standards/pages/00-one-need.md` and found the declaration's never-occluded rule with *which mechanisms count* explicitly deferred, **WHEN** they follow the deferral, **THEN** band 20 pays it — class 1 of RP-20-2 **is** the declaration — and the pairing is findable from either side, because band 20's `> **See also:**` names `00` by number, as the constitution spells it, **AND** `standards/pages/00-one-need.md` is not edited by this PR to say so."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md's `> **See also:**` line, paired with the already-merged `standards/pages/00-one-need.md` — the band 00 <-> band 20 round trip"
  verifying_test: "mechanical, both directions: `rg -n 'See also' standards/pages/20-the-fold-line.md` (names 00); `rg -n '20' standards/pages/00-one-need.md` (the slice-mate's own See also, present); `git diff main -- standards/pages/00-one-need.md standards/pages/10-the-need-set.md` (empty). Plus the procedural round trip walked once in each direction, keyboard only, one hop each way."

- id: AC-004
  criterion: "**GIVEN** the author who has cleared RP-20-2 — their content is on no never-fold class and is therefore *eligible* — **WHEN** they read **RP-20-3**, **THEN** they learn eligibility is not permission: a fold is permitted only if its mechanism appears on `PERMITTED_FOLD_MECHANISMS`, a table **in this atom**, which **ships with zero data rows** — and the atom says in those words that **folding is therefore forbidden in practice**, names what would lift it (HS-P0020's DT-7 demonstration earning the first entry), and so the author reaches \"not yet, and here is who can change that\" instead of a silence they resolve in their own favour."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the `PERMITTED_FOLD_MECHANISMS` table inside the `## RP-20-3.` rule block"
  verifying_test: "mechanical: `awk '/PERMITTED_FOLD_MECHANISMS/,/^$/' standards/pages/20-the-fold-line.md | grep -c '^|'` (= 2 — header and separator, no data row); `rg -n 'forbidden in practice' standards/pages/20-the-fold-line.md` (>= 1); `rg -n 'DT-7' standards/pages/20-the-fold-line.md` (>= 1). Plus the procedural read-back by a non-author answering, from RP-20-3 alone: may I fold today, what would have to happen first, and who owns it."

- id: AC-005
  criterion: "**GIVEN** a future contributor who has read that `mdbook-tabs` documents its panels as accessible and wants to add the first mechanism row on that basis, **WHEN** they read the entry procedure in **RP-20-3**, **THEN** it holds them to **four recorded observations in this repository** — the content (i) sits correctly in the accessibility tree, (ii) is keyboard operable, (iii) is found by Ctrl-F, (iv) is found by print — and states that **upstream documentation is not an observation** and that an **unverified property counts as unmet**, so the row is not added and the honest gap stays named rather than closed by assurance."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the entry procedure inside the `## RP-20-3.` rule block, governing the empty `PERMITTED_FOLD_MECHANISMS` table"
  verifying_test: "mechanical: all four observations present inside the `## RP-20-3.` block (`rg -n 'accessibility tree' / 'keyboard' / 'Ctrl-F' / 'print' standards/pages/20-the-fold-line.md`); `rg -n 'unverified' standards/pages/20-the-fold-line.md` (the counts-as-unmet sentence); `rg -n 'upstream' standards/pages/20-the-fold-line.md` (the not-an-observation sentence). Plus the procedural entry-procedure dry run: the walker states what they would have to produce and confirms none of it exists in this worktree."

- id: AC-006
  criterion: "**GIVEN** a reader of a rustdoc-hosted page, where rustdoc has wrapped the whole document in `<details class=\"toggle top-doc\" open>` and ships `#toggle-all-docs` to close it — so never-fold class 1 already sits inside a disclosure nobody has observed — **WHEN** they consult band 20, **THEN** it answers in **both halves**: RP-20-2 binds **the author's own markup**, so a renderer-supplied, open-by-default wrapper does not put a page in breach; **AND** because the reader can close it and nothing here has checked what survives that, the wrapper is recorded in the atom as a **named unmet property owed by HS-P0020's hosting decision** — so no page is retroactively non-conformant and no unchecked property is quietly promoted to \"fine\"."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the renderer-wrapper block scoping `## RP-20-2.`, answering `_design.md` `## Mock` finding 2"
  verifying_test: "mechanical: `rg -n 'toggle.top-doc|toggle-all-docs' standards/pages/20-the-fold-line.md` (>= 1); both sentences present and adjacent — the author's-markup scope sentence and the unmet-property sentence — pasted into this ledger verbatim. Plus the procedural two-answer read-back: is a rustdoc-hosted page in breach today (no), and is the wrapper's behaviour known (no, and it is owed by HS-P0020)."

- id: AC-007
  criterion: "**GIVEN** a contributor who assumes a rule written into a gate-read repository is a rule something checks, **WHEN** they reach band 20's closing \"what this rule does not do\" statement — itself unfolded, because it is never-fold class 5 applied to the atom that wrote the class — **THEN** they learn that this rule is applied by a **reader** and by **no gate step**; that `PERMITTED_FOLD_MECHANISMS` is a table in the rules tree and deliberately **not** a `const` in `xtask/src/lint_pages.rs`, because an unenforced const beside an enforced one reads as a check that exists; that whether a hidden branch sits inside the checked surface is HS-P0020's DT-7 and is not answered here; and that a page can satisfy every class and still teach badly, which is band 40's walk and HS-P0024's evidence."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the closing \"what this rule does not do\" statement, after the final rule's `**Evidence.**`; and the absence of any `xtask/src/**` change"
  verifying_test: "mechanical, negative half decisive: `git diff main -- xtask` (empty); `rg -n 'PERMITTED_FOLD_MECHANISMS' xtask/` (no matches); the closing statement present with all four limits named. Plus the procedural four-limits read-back by a reader who has not seen this spec, stated in their own words."

- id: AC-008
  criterion: "**GIVEN** any of those readers, on a rendered markdown page **or** in a plain-text pager, **WHEN** the atom first loads with nothing clicked, **THEN** every rule is real **composed presentation in this repository's own textual grammar** — a `## RP-20-N.` imperative sentence followed by exactly five sections in fixed order (**Why.** · **Do** · **Not** · **Rejects.** · **Evidence.**), each `Not` naming a wrong page that could plausibly ship and each `**Rejects.**` clearing **120 characters** by saying who is misled and when they find out — **AND** the atom is inside every ceiling (≤ **6** rules, ≤ **16,384** bytes, prose ≤ **96** columns outside tables), every fence is tagged `text` or `markdown` (never `rust`, never untagged), and the atom contains **no `<details>`, tab strip or accordion of its own**, because an atom about folding that folds is anti-pattern 6 self-applied."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/20-the-fold-line.md — the whole `rule-atom` surface in state `populated`, and explicitly not in `over-rule-ceiling`, `over-byte-ceiling`, `missing-section` or `rust-tagged-fence`"
  verifying_test: "mechanical, every number an output pasted here: `grep -c '^## RP-20-' standards/pages/20-the-fold-line.md` (<= 6); `wc -c < standards/pages/20-the-fold-line.md` (<= 16384); `awk '!/^\\|/ && length > 96 {print FNR\": \"length}' standards/pages/20-the-fold-line.md` (no output); the five section markers in order per rule block (awk capture); each `**Rejects.**` body >= 120 chars (awk length per rule); `rg -n '^```rust' standards/pages/` (no matches); `rg -n '^```$' standards/pages/` (no matches); `rg -n '<details|<summary|role=\"tab\"|\\{\\{#tab|<small>|<sub>|<img' standards/pages/` (no matches). Plus the procedural plain-pager read through `less`, confirming no meaning is lost and no wrong page is missing."

- id: AC-009
  criterion: "**GIVEN** the next page author who reaches this tree from a stated intent — *about to put something behind a `<details>`, a tab or a collapsed panel* — and the implementer of `page-need-checker-mounted-in-the-gate` one slice later, **WHEN** the first opens `standards/pages/README.md` and the second runs `--write` against it for the first time, **THEN** the author finds **one** `## Start here` row keyed to that intent (table still ≤ **12** rows, router still ≤ **8,192** bytes) that routes them to exactly one file, **AND** the generated `## Index` region carries **one** row for `20-the-fold-line.md` — a markdown link, the atom's first `Load when` source line, its comma-separated rule ids, interior `|` escaped, no blank line inside the markers — **byte-identical** to what the generator emits, so the `--write` produces **no diff**."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the generated `## Index` region between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->` (one row) and the `## Start here` filter table (one row); the `discipline-router` surface, changed not created"
  verifying_test: "mechanical: `rg -n '20-the-fold-line' standards/pages/README.md` (exactly 2 hits, one in the generated region and one in `## Start here`); `test -f standards/pages/20-the-fold-line.md` (the link target resolves); in-region row count equals `ls standards/pages/*.md | grep -v README | wc -l`; `awk '/^## Start here/,/^## Index/' standards/pages/README.md | grep -c '^|'` (<= 14); `wc -c < standards/pages/README.md` (<= 8192); the index row's trigger cell is not mid-phrase. Plus the procedural row-by-row derivation against `generated_region` (`xtask/src/lint_constitution.rs:400-420`) and the one-hop keyboard traverse intent -> row -> file -> stop. Forward: the no-diff `--write` recorded in `page-need-checker-mounted-in-the-gate`'s own ledger."
```
