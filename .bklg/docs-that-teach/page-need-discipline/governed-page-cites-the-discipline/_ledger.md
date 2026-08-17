---
item: "HS-S0152"
stage: implement
created: "2026-08-17"
updated: "2026-08-17"
---

# Acceptance ledger — A governed page names the discipline as the reason it is shaped as it is

The machine-checkable Definition-of-Done ledger for this story (RFC §6.5). One row per spec
`AC-###`. Planning authors every row with `satisfied: false`; the implementer may only flip a row to
`satisfied: true` and MUST cite real evidence (a `file:line` and/or the verifying test id) — never
edit, remove, or re-word a criterion, and never flip a satisfied row back. `redkiln verify --grain
story` reads the fenced block below and blocks `implement → report` unless every spec AC is present,
`satisfied: true`, and carries non-placeholder evidence (and no AC that was satisfied on the base
branch has regressed). Scope changes are a human decision recorded through `redkiln advance`, not a
quiet ledger edit.

Two of this story's rows are **procedural by design** (AC-002's membership probe, AC-007's
move/restore walk) and one is a **stated blind spot** (AC-008). For those, evidence means the
captured command output or the recorded walk pasted here verbatim — never a paraphrase of it
(spec NF-004). AC-002's fallback: if the checker's walk excludes `README.md`, EC-001's tie-break
fires and this ledger records **which** page carries the citation.

```yaml
- id: AC-001
  criterion: "GIVEN the adapter author, who has landed on docs/README.md because it is the repository's documentation index and who is asking why the page hands them onward instead of explaining anything, WHEN they read the page top to bottom with nothing clicked, THEN one sentence between the opening prose paragraph and the \"Looking for / It is at\" table states that the page routes and does NOT teach because the page discipline says an `orientation` page must not — naming what about the page the rule determines, not merely that a rule exists — with `RP-10-2` as the visible link text, AND following that link reaches the rule in one navigation step, keyboard only."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md"
  verifying_test: "mechanical: rg -n 'RP-10-2' docs/README.md; rg -n '\\[`RP-10-2`\\]\\(' docs/README.md; test -f standards/pages/<band-10 atom>.md — plus the UX-006 one-hop keyboard traverse and the non-author read-back, both recorded in this _ledger.md"
- id: AC-002
  criterion: "GIVEN the non-author reviewer, who must be able to say the citation is not decorative, WHEN they ask whether the citing page is one the discipline actually governs, THEN the citing page is shown to lie under HS-P0020's pinned TREE (docs/) AND to be reported on by the page-need checker's own walk — observed, never assumed — AND if the walk excludes README.md, the pre-decided tie-break has fired and the citation additionally sits on the governed page the walk does report, with _ledger.md recording which page carries it and why."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md"
  verifying_test: "procedural, three captures verbatim: (1) declaration removed → cargo run --locked --quiet -p xtask -- lint-pages names that file; (2) git checkout -- docs/README.md → re-run green with the '{n} pages, {m} rules, all consistent' line; (3) git status clean"
- id: AC-003
  criterion: "GIVEN the same reviewer one step later, asking the only question that can retire the whole story — does the citing page still obey the rule it cites? — WHEN they run the gate and read the page, THEN the page still declares exactly one need (`orientation`), the citation is one sentence, it teaches nothing and restates no rule text, docs/ still holds at most one `orientation` page (RP-10-3), and the added source line is <= 96 columns — so the page does not fail, in the same commit, the rule it has just named."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md"
  verifying_test: "gate-integration: cargo xtask ci green with the 'every page declares one need' step included; mechanical: rg -c '^> \\*\\*Answers:\\*\\*' docs/README.md → 1, awk '!/^\\|/ && length > 96 {print FILENAME\":\"FNR\": \"length}' docs/README.md → no output, added lines pasted verbatim; procedural: the paraphrase spot check recorded here"
- id: AC-004
  criterion: "GIVEN the evaluator standing on standards/pages/README.md, who has read the rules and is asking the reciprocal question — what material do these rules actually govern? — WHEN they read the scope paragraph, THEN it carries one resolving relative link naming docs/ as the governed material, reachable in one keyboard hop; AND the router's composition survives: the binding region order is unchanged, no heading is added, the <!-- BEGIN GENERATED --> region is byte-identical, the file is still <= 8,192 bytes, and no index of governed pages has appeared."
  satisfied: false
  evidence: ""
  mount_point: "standards/pages/README.md (region 2, the scope paragraph); wired from the docs/README.md mount"
  verifying_test: "mechanical: rg -n 'docs/README.md' standards/pages/README.md → 1; wc -c < standards/pages/README.md → <= 8192; rg -n '^#{1,2} ' standards/pages/README.md heading order equal to main; git diff main -- standards/pages/README.md one hunk inside region 2; gate: cargo run --locked -p xtask -- lint-pages green (link half, lint_constitution.rs:343-356) and --write → no diff; procedural: the reverse one-hop keyboard traverse"
- id: AC-005
  criterion: "GIVEN any of the three readers opening the page in a plain-text pager with no renderer, WHEN the page first loads, THEN the citation is composed presentation in the page's own textual grammar — a body sentence carrying the rule id as inline code inside a markdown link, in the voice of the surrounding prose — and it sits below the first prose paragraph and above the routing table; it is NOT a bare URL, a badge, an admonition block, an HTML element or an appended \"See also:\" stub, it is NOT in a footer or an end-of-page note, and the region between the H1 and the `> **Answers:**` declaration is exactly one blank line and nothing else."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md"
  verifying_test: "mechanical: first-20-lines capture (line 2 blank, next non-blank is the declaration); line-number ordering citation_line > first_prose_line and < first_table_line from rg -n; rg -n '<[a-z]|^\\s*!\\[|^> \\[!' docs/README.md → no match; rg -n 'https?://' docs/README.md → no bare URL added; procedural: the less(1) read recorded here"
- id: AC-006
  criterion: "GIVEN the evaluator reading in a pager, printing the page, and reading it in greyscale, WHEN nothing has been clicked, THEN both added sentences are visible in every state — persistent chrome, never behind a <details>, a tab, an accordion or an inactive panel — nothing added is a bespoke navigation widget, no meaning is carried by colour, an icon or size, and nothing added can move focus, scroll or selection, because the only interactive thing either PR touches is a plain markdown link."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md and standards/pages/README.md"
  verifying_test: "mechanical: rg -n '<details>|<summary>|role=\"tab\"|\\{\\{#tab' docs/README.md standards/pages/README.md → no match; rg -n '<small>|<sub>|<sup>|<nav>|<img' both files → no match; icon-standing-in-for-a-word grep → no match; procedural: the plain-pager read and the greyscale print/read of both pages, recorded here"
- id: AC-007
  criterion: "GIVEN the maintainer who relocates standards/pages/ in some future refactor and does NOT touch xtask/src/, WHEN they run the gate, THEN the build fails in the same change — .with_context() naming the pinned path, or the vacuity bail! — rather than the citation rotting quietly into a dead link; AND moving the tree back returns the gate to green with no residue."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md (the citation whose target the pin protects); observed through cargo xtask ci"
  verifying_test: "procedural, both directions captured verbatim: (1) git mv standards/pages standards/pages-moved && cargo xtask ci → fails, message naming the pinned path; (2) git mv back && cargo xtask ci → green; (3) git status → clean. Basis: xtask/src/lint_constitution.rs:212 and :176"
- id: AC-008
  criterion: "GIVEN the contributor who assumes that because this repository has a gate, the gate is watching this citation, WHEN they read _ledger.md and the router's `## What checks this tree, and what does not` section, THEN they are told plainly that at this merge the back-link is mechanically checked by the router's copied link half while the outbound link from docs/README.md is not checked by anything, and that the cross-project confirmation is a one-time procedural record — AND no shared scanner ranging over both trees was built to close that gap."
  satisfied: false
  evidence: ""
  mount_point: "docs/README.md (the unchecked outbound half) and standards/pages/README.md (the checked back-link half)"
  verifying_test: "gate-state: git diff main -- xtask → empty, output pasted here; mechanical: rg -n 'What checks this tree, and what does not' standards/pages/README.md → 1; procedural: the blind-spot row stating the asymmetry plus a non-author read-back, recorded here"
```
