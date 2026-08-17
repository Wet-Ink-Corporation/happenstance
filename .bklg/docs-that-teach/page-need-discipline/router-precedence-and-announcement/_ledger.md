---
item: "HS-S0147"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — The rules tree, its router, its rank in the chain, and its announcement

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two standing obligations specific to this story, both stated in `spec.md` and neither a licence to
soften a row:

- **The static tier is empty in this PR by construction** (`spec.md`, Tests and CI, `static` row;
  Clarification 2). This PR's boundary forbids touching `xtask/src/**`, so every AC's *present*
  verification is a mechanical `rg` / `awk` / `wc` / `sed` / `diff` / `test -f` invocation or a
  recorded procedural walk. Paste the command's **output**, never a claim about it (NF-004).
- **`cargo xtask affected --base main` is expected to widen to the whole workspace and to be green**
  (EC-007, NF-005). Record the widening as observed; do not "fix" it by editing
  `xtask/src/affected.rs`.

```yaml
- id: AC-001
  criterion: "**GIVEN** the next page author, about to write a narrative page and unwilling to read a corpus to start, **WHEN** they open `standards/pages/README.md` from a stated intent, **THEN** the page opens with `# Page standards`, one scope paragraph carrying the instruction to load one rule and never the tree, and the five-row band table (`00`, `10`, `20`, `30`, `40`) — and they name their task in one `## Start here` row, open exactly **one** rule file, and stop, without ever loading a second."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the `discipline-router` surface, reached in one hop from the repository index `docs/README.md:12-24`"
  verifying_test: "mechanical, captured verbatim: `test -f standards/pages/README.md`; `rg -n '^# Page standards$' standards/pages/README.md` (one hit); `rg -c 'load one rule, never the tree' standards/pages/README.md` (>= 1); `awk '/^\\| Band \\|/,/^$/' standards/pages/README.md | tail -n +3 | wc -l` (= 5). Plus the procedural UX-012 walk, one per band that exists at this merge, each recorded as `task -> row -> file opened` and each ending at one file."

- id: AC-002
  criterion: "**GIVEN** the non-author reviewer, who has found a page rule and a `spec/SPECIFICATION.md` clause that appear to disagree and does **not** know which wins, **WHEN** they read `## Precedence` on the page they already landed on, **THEN** they learn the discipline sits at the **constitution-atom tier** of the existing five-tier chain, alongside `standards/rust/` and scoped to a different subject, adding no tier — **AND** they reach that answer without opening a second file, **AND** the chain's own statement was never edited to say it."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md `## Precedence` — the rank stated in place; `standards/rust/README.md:23-29` is cited and never opened for writing"
  verifying_test: "gate-state: `git diff main -- standards/rust/README.md` -> empty, output pasted here. Mechanical: `rg -n 'constitution.atom tier' standards/pages/README.md` (>= 1) and `rg -n 'SPECIFICATION clause' standards/pages/README.md` (>= 1). Procedural: a person who did not author the router answers \"does a page rule beat a clause?\" from the router's text alone, verdict recorded. Forward obligation: the `check_shape`-style structural-presence assertion inherited by `page-need-checker-mounted-in-the-gate` (Testing brief, AC-002, \"Static, secondary\")."

- id: AC-003
  criterion: "**GIVEN** a reader whose intent matches **no** `## Start here` row — the case a filter cannot serve — **WHEN** they keep reading the router top to bottom, **THEN** every rule in the tree is still listed, in a complete `## Index` between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, one row per rule atom with a resolving link, its trigger phrase and its rule ids; the index is **on the same page as the filter**, not behind a toggle, a fold or a second page, and it is complete whether or not the filter matched."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the generated `## Index` region, on the same page as the `## Start here` filter it is filtered by"
  verifying_test: "mechanical: in-region row count equals the corpus — `sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md | grep -c '^|'` minus 2 equals `ls standards/pages/*.md | grep -v README | wc -l`; every in-region `NN-slug.md` target passes `test -f standards/pages/<target>`; `rg -n 'show all|<details>|<summary>' standards/pages/README.md` -> no matches. Procedural: name one rule no `## Start here` row points at and reach it by reading top to bottom. Forward: the link half of `check_router` reused verbatim (`xtask/src/lint_constitution.rs:343-356`)."

- id: AC-004
  criterion: "**GIVEN** the implementer of `page-need-checker-mounted-in-the-gate`, one slice later, who runs `--write` against this router for the first time, **WHEN** the generator emits the region from the corpus, **THEN** the output is **byte-identical** to what this PR committed and the run produces **no diff** — so the reader never sees a router whose index silently disagrees with its own tree, and the repair path (`--write`) is the only writer of that region."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the bytes between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, a forward contract on `page-need-checker-mounted-in-the-gate`"
  verifying_test: "mechanical, the decisive one: `diff <(sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/rust/README.md | sed -n '2,3p') <(sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md | sed -n '2,3p')` -> empty (header and separator are the precedent's verbatim, per Clarification 3). Procedural: a row-by-row derivation table mapping each committed row to `generated_region`'s format at `xtask/src/lint_constitution.rs:400-420` — link form, an empty trigger rendered as an em dash, every interior pipe escaped, rows joined with newlines and no blank line inside the markers. Forward: that story's first `--write` run, recorded in its own ledger as producing no diff."

- id: AC-005
  criterion: "**GIVEN** a slice-mate authoring a rule atom into this tree, **WHEN** they read `## The shape of a rule` before writing the atom head, **THEN** they learn the grammar (`# NN — Title`, `> **Load when:**`, `> **See also:**`, `---`, then `## RP-NN-N.` with the five fixed sections) **and** the constraint that `Load when:` occupies exactly **one source line**, with the reason attached — so the reader of the generated index gets a whole trigger phrase instead of the mid-phrase fragment the constitution's own index shows today."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md `## The shape of a rule` — the authoring grammar the `rule-atom` surface's instances (the slice-mates') are written against"
  verifying_test: "mechanical: `rg -n '^## The shape of a rule$' standards/pages/README.md` (one hit); for every `> **Load when:**` match in the tree the following source line does not begin with `>` — `rg -n -A1 'Load when:' standards/pages/` reviewed and captured. Cross-check against the parser that motivates it (`xtask/src/lint_constitution.rs:247-255`) and the truncation it produces (`standards/rust/README.md:70-71`). Consequence check: no in-region trigger cell ends mid-phrase, confirmed by reading the region."

- id: AC-006
  criterion: "**GIVEN** a contributor who assumes that because the repository has a gate, the gate is watching this tree, **WHEN** they read the router, **THEN** `## What checks this tree, and what does not` tells them, **at this merge**, that **no** gate step reads `standards/pages/` yet, names `page-need-checker-mounted-in-the-gate` as the step that adds one, and names what will still be unchecked afterwards — that a page *declares* a need is checkable, that it *answers* one is not, and band `40`'s reviewer walk is the instrument for the rest."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md `## What checks this tree, and what does not` — the blind-spot section, modelled on `standards/rust/README.md:114-127`"
  verifying_test: "mechanical: `rg -n '^## What checks this tree, and what does not$' standards/pages/README.md` (one hit); `rg -n 'page-need-checker-mounted-in-the-gate' standards/pages/README.md` (>= 1). Procedural: a reader who has not seen this spec reads only that section and states, in their own words, the three facts above; recorded. Bar: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`)."

- id: AC-007
  criterion: "**GIVEN** the reader who arrives at the repository's own index — the persona whose measured defect is an answer that existed three documents from where they were standing — **WHEN** they scan `docs/README.md`'s \"Looking for / It is at\" table, **THEN** one row states a question they would actually ask and links to `standards/pages/README.md`, reachable in **one** navigation step, keyboard only; **AND** the gate-read-trees paragraph names the third tree carrying `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]`, so a reader of that paragraph is told the truth at this merge rather than promised a check that does not exist."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md — both regions: the \"Looking for / It is at\" table at `:12-24` (one new row) and the gate-read-trees paragraph at `:25-29` (the third tree named, carrying the `[PROVISIONAL — settles at page-need-checker-mounted-in-the-gate]` marker)"
  verifying_test: "mechanical: `rg -n 'standards/pages' docs/README.md` -> >= 2 hits, one inside the `:12-24` table and one inside the `:25-29` paragraph; the row's link target passes `test -f standards/pages/README.md`; `rg -n 'PROVISIONAL — settles at' docs/README.md` -> one hit naming `page-need-checker-mounted-in-the-gate`. Procedural: the one-hop keyboard traverse from `docs/README.md` to the router (medium recorded, per Clarification 6), and the closing sentence about editing `xtask/src/` in the same change confirmed present and unweakened. Forward: the static \"names three trees, not two\" count assertion inherited by `page-need-checker-mounted-in-the-gate`."

- id: AC-008
  criterion: "**GIVEN** any of the three readers, on a rendered markdown page **or** in a plain-text pager, **WHEN** the router first loads with nothing clicked, **THEN** every region is present as **composed presentation from this repository's own textual grammar** — H1, scope paragraph, band table, precedence blockquote, intent-keyed table, marker-delimited generated region, headed sections — in the binding top-to-bottom order, with the filter above the thing it filters; **AND** nothing is behind a fold, a `<details>`, a tab or an inactive panel, no bespoke navigation widget is added on top of what the renderer already gives, and no region's meaning is carried by colour, an icon or size."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md as a whole — the `discipline-router` surface's `populated` state, in the region order `_design.md` `## Composition` fixes"
  verifying_test: "mechanical: `rg -n '^#{1,2} ' standards/pages/README.md` prints exactly `# Page standards`, `## Precedence`, `## Start here`, `## Index`, `## The shape of a rule`, `## What checks this tree, and what does not`, in that order, with the band table between the scope paragraph and `## Precedence`; `rg -n '<details>|<summary>|role=\"tab\"|\\{\\{#tab' standards/pages/` -> no matches; `rg -n '<small>|<sub>|<sup>|<nav>|<img' standards/pages/` -> no matches. Procedural: read the file through `less` (no renderer) and confirm each region is legible and that no meaning is lost; checked against `_design.md` `## Anti-patterns` 5, 6, 7, 9 and `## Transience policy` rows S2."

- id: AC-009
  criterion: "**GIVEN** the page author who loads this router on **every** page they write — so its cost is paid on every task in the project — **WHEN** the router is measured rather than asserted, **THEN** it is ≤ **8,192 bytes**, every prose line outside a table is ≤ **96 columns**, `## Start here` carries ≤ **12** rows, every fence is tagged `text` or `markdown` (never `rust`, never untagged), and every `.md` link in the file resolves — and if a budget bites, the stated yield order is what gives: `Start here` rows merge, then the scope paragraph shortens; the band table and the `## Index` never yield."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md — the density budget of the `discipline-router` surface (`_design.md` `## Density budget`, S2, and its yield order)"
  verifying_test: "mechanical, every number a command whose output is pasted here (NF-004): `wc -c < standards/pages/README.md` -> <= 8192; `awk '!/^\\|/ && length > 96 {print FILENAME\":\"FNR\": \"length}' standards/pages/README.md` -> no output; `awk '/^## Start here/,/^## Index/' standards/pages/README.md | grep -c '^|'` -> <= 14 (12 rows plus header and separator); `rg -n '^```rust' standards/pages/` -> no matches and `rg -n '^```$' standards/pages/` -> no matches; link resolution as AC-003. Calibration: `xtask/src/lint_constitution.rs:88,95`."
```
