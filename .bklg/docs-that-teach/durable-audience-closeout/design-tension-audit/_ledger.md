---
item: HS-S0176
stage: implement
created: "2026-08-17T13:16:27.636Z"
updated: "2026-08-17T13:16:27.636Z"
---

# Acceptance ledger — Audit DT-1 … DT-10 for resolution, deferral or gap

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Three notes for whoever flips these rows.

**The mount point is the same for all seven, and it is a link, not a build step.** Every criterion is
reachable through `.bklg/docs-that-teach/durable-audience-closeout/project.md`'s `## Companions` list
(`:291-301`) — the composition root a reviewer reaches this project's artefacts *through*. A record
sitting in this story folder that nothing links to is this corpus's analogue of a component rendered
into no tree, and because `design.capture` is deliberately absent from `.redkiln/config.yaml`
(`:75-83`) no perceptual gate would ever notice. Evidence that cites only `_audit.md` and not the
companion bullet is not satisfied.

**There is no automated verifying test, and inventing one is a defect, not diligence.** The project's
testing brief classifies AC-018 as Tier 2 content review with the note "no automated reader exists for
this table" (`.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:878`), AC-TB-01 forbids
citing Tier 1 alone for it (`:824-826`), and AC-TB-02 forbids invented tooling (`:827-830`). The
`verifying_test` field below therefore names the **Tier 2 review** — a human read against the UX
brief's own checklist, IQ-1 … IQ-8 (`:145-232`) and AC-UX-09 / AC-UX-10 (`:266-273`) — with the Tier 1
commands a reviewer *runs* named beside it as support. Those commands are not added to the tree or to
CI.

**Evidence must name the merged sha.** Every row is observed against the commit
`merge-forward-baseline` (HS-S0172) produced and recorded in
`.bklg/docs-that-teach/durable-audience-closeout/_baseline.md`. A row flipped on an observation taken
against this worktree's pre-merge copy is invalid however true it looks (`_storymap.md:144-147`,
ordering 1).

```yaml
- id: AC-001
  criterion: "**GIVEN** U2 opens `_audit.md` intending to disagree with exactly one tension's adjudication, **WHEN** they scan it once, **THEN** they find exactly ten body rows keyed `DT-1` … `DT-10`, each carrying the charter's own tension text in short form, with no id absent, no id invented, and no two tensions merged into one row because they share an owner — and any summary sentence sits *above* the table rather than in place of a row."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 content review of .bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md against .bklg/docs-that-teach/initiative.md:483-494, one row at a time; Tier 1 support (never alone, AC-TB-01): `rg -c '^\\| DT-' .bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` = 10 and ten distinct ids"
- id: AC-002
  criterion: "**GIVEN** U2 wants to route a finding to the team that owns it, **WHEN** they read any row's owner cell, **THEN** it names the real backlog project by id *and* slug — `HS-P0020 checked-documentation-surface`, `HS-P0021 page-need-discipline`, `HS-P0022 application-author-path`, `HS-P0023 reach-and-adapter-path`, `HS-P0024 comprehension-evidence` — matching `_decomposition.md`'s ownership map exactly, with the charter's superseded descriptive labels (\"Conceptual bridge\", \"Page-need discipline\", \"First encounter\", \"Checked prose\", \"Comprehension evidence\", \"Reach and placement\") appearing nowhere as an owner."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review of each owner cell against .bklg/docs-that-teach/_decomposition.md:148-159 (DT-1/DT-4/DT-5/DT-6 all resolve to HS-P0022, per :29-32 and :52-58); Tier 1 support: `rg -n 'Conceptual bridge|First encounter|Reach and placement' .bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` finds no match in an owner cell"
- id: AC-003
  criterion: "**GIVEN** U2 refuses to accept \"resolved\" as a claim, **WHEN** they follow any row's evidence cell, **THEN** it is a `file:line` into the **owning** project's `_design.md` that resolves in the merged tree and, when opened, actually contains the resolution the row asserts — and where a tension is discussed in a *non-owning* project's design (the live instance is `checked-documentation-surface/_design.md:267-269` on DT-8, which is HS-P0021's) that discussion is recorded in the note as a cross-reference and does **not** move the outcome to `resolved`."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review: the reviewer opens every cited file:line in .bklg/docs-that-teach/{application-author-path,page-need-discipline,checked-documentation-surface,comprehension-evidence,reach-and-adapter-path}/_design.md and confirms the text found there is the resolution claimed — the falsifier named at design-tension-audit/discover.md:53-55; Tier 1 support: `test -f` over every distinct cited path"
- id: AC-004
  criterion: "**GIVEN** U2 must be able to tell a settled tension from an unowned one without inference, **WHEN** they read any outcome cell, **THEN** it holds exactly one of the three words `resolved` / `recorded deferral` / `gap` spelled out in full; a `recorded deferral` names where the deferral is written and what it defers to; and a `gap` row names the owning project, the file the resolution would be written into, and — for DT-6, DT-7 and DT-9 — the Definition-of-Done scenario that rests on it, then **stops**: no option chosen, no sibling `_design.md` edited, no ADR opened."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review against .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:70 (three-state vocabulary) and .bklg/docs-that-teach/durable-audience-closeout/project.md:219-220 (no sibling requirement re-opened); Tier 1 support: every outcome cell matches ^(resolved|recorded deferral|gap)$, `git diff --name-only` shows no sibling `_design.md` and no `.kb/decisions/` path, and `redkiln validate --kb` exits zero"
- id: AC-005
  criterion: "**GIVEN** U3 returns months later and asks \"against which tree, on what evidence\", **WHEN** they read the record's provenance header and any single row, **THEN** both name the merged tree: the header states the branch, the full 40-hex merged sha copied from `_baseline.md`, the sibling branch merged, the observer and the date; and each row repeats the observer and the short sha in its own cell, so a row lifted out of the table still says what it was observed against. No observation is taken against this worktree's pre-merge copy."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review that the header's sha is the one .bklg/docs-that-teach/durable-audience-closeout/_baseline.md records, cited rather than re-derived from `git log` (merge-forward-baseline/spec.md:98, :126); Tier 1 support: the sha string matches in both files and `git cat-file -t <merged-sha>` returns `commit`"
- id: AC-006
  criterion: "**GIVEN** a reader who has never heard of this story opens `project.md`, **WHEN** they look for this project's artefacts, **THEN** they reach `_audit.md` in **one hop** from the `## Companions` list via link text that names the destination — and `git diff .bklg/docs-that-teach/durable-audience-closeout/project.md` shows a pure append: one added bullet, zero `-` lines, no reflow of any pre-existing section, and no change whatsoever to the YAML frontmatter the CLI owns."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review: the one-hop walk is performed from .bklg/docs-that-teach/durable-audience-closeout/project.md:291-301, not asserted; Tier 1 support: `rg -n '_audit' .bklg/docs-that-teach/durable-audience-closeout/project.md` matches inside `## Companions`, and `git diff -U0 -- .bklg/docs-that-teach/durable-audience-closeout/project.md | rg '^-[^-]'` returns nothing (the IQ-3 falsifier form, _decomposition.md:180-188)"
- id: AC-007
  criterion: "**GIVEN** U2 reads the record in a plain terminal with no colour, **WHEN** they take any single row out of context, **THEN** it is fully understandable on its own — tension id, owner, evidence `file:line`, outcome word, observer and sha, note — with no cell reading \"same as above\", \"ditto\", \"see row 3\" or left empty; no state anywhere carried by colour, emoji, glyph, tick, strikethrough, ordering or an empty cell; the record composed only from primitives already in this corpus (the `| … |` markdown table, the `file:line` citation form, the `## Companions` bullet) with no legend, key, fold, `<details>`, tab or collapsed panel; and the density budget held — one provenance header of at most 12 lines, exactly 10 body rows, exactly the 7 columns named in Implementation notes, at most 60 words per note cell, and at most one further `## Routed gaps` section."
  satisfied: false
  evidence: ""
  mount_point: ".bklg/docs-that-teach/durable-audience-closeout/project.md — the `## Companions` list (:291-301), reached in one hop to `design-tension-audit/_audit.md`"
  verifying_test: "Tier 2 review against .bklg/docs-that-teach/durable-audience-closeout/_decomposition.md:266-269 (AC-UX-09, the a11y floor, which names the DT audit table explicitly) and :270-273 (AC-UX-10, self-contained rows); Tier 1 support: `rg -n 'same as above|ditto|see row|<details>|<summary>'` returns nothing, `rg -n '\\|\\s*\\|'` finds no empty cell, a non-ASCII scan finds no emoji or glyph outside prose, and the row/column/word counts are taken"
```
