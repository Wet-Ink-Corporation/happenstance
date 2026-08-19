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
  satisfied: true
  evidence: "`docs/README.md:13-15`, the whole of the addition, pasted verbatim: `It routes rather than teaches: the library is explained on the pages below and / not here, because [`RP-10-2`](../standards/pages/10-the-need-set.md) is the rule / that shapes a page whose job is orientation.` It is a *reason*, not a see-also: the property named is that the library's teaching is on the pages below and not on this page, and RP-10-2 is named as what makes that so. Mechanical, outputs verbatim: `rg -n 'RP-10-2' docs/README.md` -> `14:not here, because [`RP-10-2`](../standards/pages/10-the-need-set.md) is the rule` (1 hit). `rg -n '\\[`RP-10-2`\\]\\(' docs/README.md` -> the same single line, so the id is inside the *link text* and not only in the target. `test -f standards/pages/10-the-need-set.md` -> `PRESENT standards/pages/10-the-need-set.md`; RP-10-2 is at `standards/pages/10-the-need-set.md:77` and the router's generated index names that atom for it (`standards/pages/README.md:52`). UX-006 traverse, keyboard only, recorded: docs/README.md -> Tab to the `RP-10-2` link at `:14` -> Enter -> standards/pages/10-the-need-set.md. **One** step, no mouse; recorded as `page -> link -> file`. **Non-author read-back, performed and PASSED.** Walker: the **adversarial slice reviewer for `binding-beyond-this-project`** — the review pass that produced this slice's findings, which wrote no part of `docs/README.md`, of this ledger or of the implementation report. Date: **2026-08-18**. Verdict transcribed verbatim: *from `docs/README.md:13-15` alone the property RP-10-2 dictates is nameable (the page routes; the library is explained on the pages below, not here) — it is a reason, not a see-also.* **What this replaced:** the first-merge read-back was performed by the implementer, who wrote the sentence, so it is not evidence for a read-back whose whole point is a reader who did not write it; it is superseded here rather than kept in the row, per the precedent one slice back (`reviewer-and-citation-procedures/_ledger.md:84`)."
  mount_point: "docs/README.md"
  verifying_test: "mechanical: rg -n 'RP-10-2' docs/README.md; rg -n '\\[`RP-10-2`\\]\\(' docs/README.md; test -f standards/pages/10-the-need-set.md — plus the UX-006 one-hop keyboard traverse and the non-author read-back, both recorded in this _ledger.md"
- id: AC-002
  criterion: "GIVEN the non-author reviewer, who must be able to say the citation is not decorative, WHEN they ask whether the citing page is one the discipline actually governs, THEN the citing page is shown to lie under HS-P0020's pinned TREE (docs/) AND to be reported on by the page-need checker's own walk — observed, never assumed — AND if the walk excludes README.md, the pre-decided tie-break has fired and the citation additionally sits on the governed page the walk does report, with _ledger.md recording which page carries it and why."
  satisfied: true
  evidence: "**The walk excludes README.md, EC-001 fired, and this ledger's `## EC-001 fired` section records which page carries what and why.** Both citing pages lie under HS-P0020's pinned TREE — `const TREE: &str = \"docs\";` consumed by value through `lint_narrative::TREE` at `xtask/src/lint_pages.rs:467`, no second const declared here. Capture (1), the membership probe against the tie-break page `docs/append-conditions.md` with its `> **Answers:**` declaration temporarily deleted: `cargo run --locked --quiet -p xtask -- lint-pages` -> `  docs/append-conditions.md — no `> **Answers:**` line; see standards/pages/00-one-need.md` then `xtask failed: 1 problem(s) in standards/pages + docs`. The walk names that file, so that page is inside the governed set. Capture (1a), the inverse probe against `docs/README.md`, run first: an unenumerated token `probe` inserted under its H1 produced `  2 pages, 16 rules, all consistent` — total silence, so README.md is outside the set; the exclusion is `xtask/src/lint_pages.rs:496`. Capture (2), both probes reverted and re-run: `cargo run --locked --quiet -p xtask -- lint-pages` -> `  2 pages, 16 rules, all consistent`. Capture (3): `git status --short` -> only ` M .redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`, ` M docs/README.md`, ` M docs/append-conditions.md`, ` M standards/pages/README.md` — the three intended edits and the session telemetry that was already modified at the story's start; no probe residue. This is a one-off *membership* probe and is not cited as evidence for `declaration-check-seen-to-fail`'s AC-007, which owns the two-needs and unenumerated-token demonstrations against `cargo xtask ci`."
  mount_point: "docs/README.md (in place, at the index) and docs/append-conditions.md (EC-001's tie-break: the governed page the walk reports)"
  verifying_test: "procedural, three captures verbatim: (1) declaration removed → cargo run --locked --quiet -p xtask -- lint-pages names that file; (2) git checkout -- docs/README.md → re-run green with the '{n} pages, {m} rules, all consistent' line; (3) git status clean"
- id: AC-003
  criterion: "GIVEN the same reviewer one step later, asking the only question that can retire the whole story — does the citing page still obey the rule it cites? — WHEN they run the gate and read the page, THEN the page still declares exactly one need (`orientation`), the citation is one sentence, it teaches nothing and restates no rule text, docs/ still holds at most one `orientation` page (RP-10-3), and the added source line is <= 96 columns — so the page does not fail, in the same commit, the rule it has just named."
  satisfied: true
  evidence: "Gate: `cargo xtask ci` -> `all checks passed`, with `=== every page declares one need ===` -> `  2 pages, 16 rules, all consistent` — that step is what asserts the single declaration and the per-directory `orientation` count (`xtask/src/lint_pages.rs:630` check_declarations, `:685` check_orientation_ceiling). Declaration counts, verbatim: `rg -c '^> \\*\\*Answers:\\*\\*' docs/append-conditions.md docs/text-fences.md` -> `docs/append-conditions.md:1` and `docs/text-fences.md:1` — the governed citing page still declares **exactly one** need after the edit. `rg -c '^> \\*\\*Answers:\\*\\*' docs/README.md` -> no output, exit 1: the index carries none, because it is outside the walk (`xtask/src/lint_pages.rs:496`) and giving it one would claim a governance no checker extends — the divergence is recorded in this ledger's `## EC-001 fired` section rather than smoothed over. `docs/` holds **zero** `orientation` declarations among the walked pages, so RP-10-3's ceiling of one is met with room. Column ceiling: `awk '!/^\\|/ && length > 96 {print FILENAME\":\"FNR\": \"length}' docs/README.md docs/append-conditions.md standards/pages/README.md` -> **no output**. The added blocks, pasted verbatim rather than counted by claim — `docs/README.md:13-15`: `It routes rather than teaches: the library is explained on the pages below and / not here, because [`RP-10-2`](../standards/pages/10-the-need-set.md) is the rule / that shapes a page whose job is orientation.` — one sentence, one full stop. `docs/append-conditions.md:9-10`: `It answers the one question its declaration names and no second one, because / [`RP-00-1`](../standards/pages/00-one-need.md) is the rule that shaped it.` — one sentence, one full stop. Paraphrase spot check (RP-40-2, `standards/pages/40-reviewing-a-page.md:68`) run over both sentences: RP-10-2's text is `Keep an `orientation` page to links and one sentence each.` / `An `orientation` page carries links, and at most one sentence per destination saying what that destination answers. It teaches nothing.` — the citation reproduces none of it; the only shared token is the need name `orientation`, which is the thing being named. RP-00-1's text is `Declare exactly one need on every governed page.` — the citation reproduces none of it either. Both sentences take their authority from the cited id rather than asserting a rule in their own voice, which is the discriminator that check applies (`xtask/src/lint_pages.rs:466-470`'s general form: one of two copies goes stale). **The spot check was re-run by a NON-AUTHOR and PASSED.** Walker: the **adversarial slice reviewer for `binding-beyond-this-project`**, the review pass that produced this slice's findings, which wrote neither citation. Date: **2026-08-18**. Verdict transcribed verbatim: *neither citation reproduces RP-10-2's or RP-00-1's rule text; the only shared token is the need name being pointed at.* **What this replaced:** the first-merge run of RP-40-2's spot check was performed by the implementer, who wrote both sentences, and RP-40-1's own bar is a reader who is not the author; that capture is superseded here rather than kept in the row, per the precedent one slice back (`reviewer-and-citation-procedures/_ledger.md:84`)."
  mount_point: "docs/append-conditions.md (the governed citing page, where the single-declaration bar is observed); docs/README.md (the index citation's own measurements)"
  verifying_test: "gate-integration: cargo xtask ci green with the 'every page declares one need' step included; mechanical: rg -c '^> \\*\\*Answers:\\*\\*' docs/README.md → 1, awk '!/^\\|/ && length > 96 {print FILENAME\":\"FNR\": \"length}' docs/README.md → no output, added lines pasted verbatim; procedural: the paraphrase spot check recorded here"
- id: AC-004
  criterion: "GIVEN the evaluator standing on standards/pages/README.md, who has read the rules and is asking the reciprocal question — what material do these rules actually govern? — WHEN they read the scope paragraph, THEN it carries one resolving relative link naming docs/ as the governed material, reachable in one keyboard hop; AND the router's composition survives: the binding region order is unchanged, no heading is added, the <!-- BEGIN GENERATED --> region is byte-identical, the file is still <= 8,192 bytes, and no index of governed pages has appeared."
  satisfied: true
  evidence: "`standards/pages/README.md:8` — `rules govern is the narrative tree at [`docs/`](../../docs/README.md).`, and `rg -n 'docs/README.md' standards/pages/README.md` returns **exactly that one line**. It is inside region 2, the scope paragraph: `git diff HEAD -- standards/pages/README.md` is **one hunk at `@@ -4,7 +4,8 @@`**, one line replaced by two, between the H1 and the `| Band | Owns |` table — no region moved, no heading added, nothing between the generated markers. Heading order, captured both sides: after -> `1:# Page standards / 18:## Precedence / 32:## Start here / 44:## Index / 59:## The shape of a rule / 79:## What checks this tree, and what does not`; before (`git show HEAD:standards/pages/README.md | rg -n '^#{1,2} '`) -> the same six headings in the same order at `1/17/31/43/58/78`, shifted by the one added line and nothing else. Budget: `wc -c < standards/pages/README.md` -> `5429` (ceiling 8192), also asserted by `lint_pages::tests::router_is_inside_its_budgets`. No index of governed pages appeared — the diff is two prose lines. **The back-link is mechanically checked, and it was seen to fail rather than assumed:** with the target temporarily mistyped as `../../docs/READMEX.md`, `cargo run --locked --quiet -p xtask -- lint-pages` -> `  standards/pages/README.md:8 — link `../../docs/READMEX.md` resolves to no file` then `xtask failed: 1 problem(s) in standards/pages + docs`. Restored, `cargo run --locked --quiet -p xtask -- lint-pages` -> `  2 pages, 16 rules, all consistent`. That also discharges EC-004's forward obligation as *observed*: the copied link half resolves `root.join(RULE_DIR).join(target)` (`xtask/src/lint_pages.rs:817`) and was **not** narrowed to targets inside RULE_DIR, so a `../../` target outside the rules tree resolves correctly. `cargo run --locked -p xtask -- lint-pages --write` -> `  2 pages, 16 rules, all consistent` and `git status --short` afterwards shows no change to `standards/pages/README.md` beyond this story's own hunk — **no diff**. Reverse UX-006 traverse, keyboard only, recorded: standards/pages/README.md -> Tab to the `docs/` link at `:8` -> Enter -> docs/README.md. **One** hop, no mouse."
  mount_point: "standards/pages/README.md (region 2, the scope paragraph); wired from the docs/README.md mount"
  verifying_test: "mechanical: rg -n 'docs/README.md' standards/pages/README.md → 1; wc -c < standards/pages/README.md → <= 8192; rg -n '^#{1,2} ' standards/pages/README.md heading order equal to main; git diff main -- standards/pages/README.md one hunk inside region 2; gate: cargo run --locked -p xtask -- lint-pages green (link half, lint_constitution.rs:343-356) and --write → no diff; procedural: the reverse one-hop keyboard traverse"
- id: AC-005
  criterion: "GIVEN any of the three readers opening the page in a plain-text pager with no renderer, WHEN the page first loads, THEN the citation is composed presentation in the page's own textual grammar — a body sentence carrying the rule id as inline code inside a markdown link, in the voice of the surrounding prose — and it sits below the first prose paragraph and above the routing table; it is NOT a bare URL, a badge, an admonition block, an HTML element or an appended \"See also:\" stub, it is NOT in a footer or an end-of-page note, and the region between the H1 and the `> **Answers:**` declaration is exactly one blank line and nothing else."
  satisfied: true
  evidence: "**Composed, not marked up.** Both additions are body sentences in the surrounding prose's voice, opening with `It` exactly as the paragraph above each of them opens with a noun phrase, and each carries its rule id as **inline code inside a markdown link** — `[`RP-10-2`](../standards/pages/10-the-need-set.md)`, `[`RP-00-1`](../standards/pages/00-one-need.md)`. Neither is a bare URL, a badge, an admonition, an HTML element or an appended `See also:` stub, and neither sits in a footer. Ordering on `docs/README.md`, all three numbers from `rg -n`: first prose line **3** (`**This directory is for user documentation** — …`), citation line **13**, first table line **17** (`| Page | Read it at |`), routing table line **22** (`| Looking for | It is at |`). So `13 > 3` and `13 < 17`. Ordering on `docs/append-conditions.md`: first prose line **5**, citation line **9**, next block (the `rust` fence) line **12**. **The H1/declaration region, captured as the first 20 lines of the governed citing page** (`sed -n '1,20p' docs/append-conditions.md`): line 1 `# Appending under a condition`, line 2 **blank**, line 3 `> **Answers:** `explanation` — Why does a write re-read what it decided on?` — exactly one blank line and nothing else, so anti-pattern 1 / EC-007 is refused and the citation sits four lines *below* the declaration, never between it and the H1. The same capture for `docs/README.md` shows line 1 `# Documentation`, line 2 blank, line 3 the opening prose paragraph, citation at 13 — the index carries no declaration at all (see the `## EC-001 fired` section), so the between-H1-and-declaration bar is observed on the page that has one. Markup greps, both files: `rg -n '<[a-z]|^\\s*!\\[|^> \\[!' docs/README.md docs/append-conditions.md` -> **exit 1, no match** (no HTML element, no badge image, no admonition); `rg -n 'https?://' docs/README.md docs/append-conditions.md standards/pages/README.md` -> **exit 1, no match** (no bare URL added). Plain-pager read: both files read top to bottom as UTF-8 text with no renderer; each citation reads as a body sentence in the flow of the page, not as chrome bolted beside it."
  mount_point: "docs/README.md and docs/append-conditions.md"
  verifying_test: "mechanical: first-20-lines capture (line 2 blank, next non-blank is the declaration); line-number ordering citation_line > first_prose_line and < first_table_line from rg -n; rg -n '<[a-z]|^\\s*!\\[|^> \\[!' docs/README.md → no match; rg -n 'https?://' docs/README.md → no bare URL added; procedural: the less(1) read recorded here"
- id: AC-006
  criterion: "GIVEN the evaluator reading in a pager, printing the page, and reading it in greyscale, WHEN nothing has been clicked, THEN both added sentences are visible in every state — persistent chrome, never behind a <details>, a tab, an accordion or an inactive panel — nothing added is a bespoke navigation widget, no meaning is carried by colour, an icon or size, and nothing added can move focus, scroll or selection, because the only interactive thing either PR touches is a plain markdown link."
  satisfied: true
  evidence: "Fold-mechanism grep over all three edited files: `rg -n '<details>|<summary>|role=\"tab\"|\\{\\{#tab' docs/README.md docs/append-conditions.md standards/pages/README.md` -> **exit 1, no match**. Size/widget grep: `rg -n '<small>|<sub>|<sup>|<nav>|<img' docs/README.md docs/append-conditions.md standards/pages/README.md` -> **exit 1, no match**. Icon-standing-in-for-a-word grep: `rg -n '[🟢🔴⚠️✅]' docs/README.md docs/append-conditions.md standards/pages/README.md` -> **exit 1, no match**. The router is additionally held to the same bar by `lint_pages::tests::router_regions_are_in_the_binding_order`, which asserts the absence of `<details`, `<summary`, `role=\"tab\"`, `{{#tab`, `<small>`, `<sub>`, `<sup>`, `<nav>` and `<img` on every run of `cargo xtask ci`. Persistent chrome is discharged **by removing the mechanism**: all three additions are plain markdown body text plus one inline link each, so nothing exists that *can* move focus, scroll or selection — there is no authored disclosure and no script in the diff. Plain-pager read: all three files read as text with every added sentence visible on first load, nothing clicked. Greyscale read: each addition's only emphasis carriers are inline code (monospace) and link text — both survive greyscale and both survive printing; no colour, icon or size carries any distinction. Basis: the citation states a constraint on the page, never-fold class 3, and `PERMITTED_FOLD_MECHANISMS` ships empty (`standards/pages/20-the-fold-line.md`, the `| Mechanism |` table with zero data rows, asserted by `lint_pages::tests::band_twenty_ships_an_empty_permitted_mechanism_table`), so the mechanism is forbidden outright rather than merely unused."
  mount_point: "docs/README.md, docs/append-conditions.md and standards/pages/README.md"
  verifying_test: "mechanical: rg -n '<details>|<summary>|role=\"tab\"|\\{\\{#tab' docs/README.md standards/pages/README.md → no match; rg -n '<small>|<sub>|<sup>|<nav>|<img' both files → no match; icon-standing-in-for-a-word grep → no match; procedural: the plain-pager read and the greyscale print/read of both pages, recorded here"
- id: AC-007
  criterion: "GIVEN the maintainer who relocates standards/pages/ in some future refactor and does NOT touch xtask/src/, WHEN they run the gate, THEN the build fails in the same change — .with_context() naming the pinned path, or the vacuity bail! — rather than the citation rotting quietly into a dead link; AND moving the tree back returns the gate to green with no residue."
  satisfied: true
  evidence: "Both directions, captured verbatim. (1) `git mv standards/pages standards/pages-moved` -> `ls standards` -> `pages-moved` and `rust`. `cargo run --locked --quiet -p xtask -- lint-pages` -> `xtask failed: reading standards/pages: The system cannot find the path specified. (os error 3)` — that is the `.with_context(|| format!(\"reading {RULE_DIR}\"))` at `xtask/src/lint_pages.rs:395` naming the **pinned path**, not a skip and not a silent pass. The whole gate fails with it: `cargo xtask ci --fast` -> `test result: FAILED. 217 passed; 44 failed; 0 ignored; 0 measured; 0 filtered out` then `xtask failed: tests failed with exit code: 101`, the 44 including `lint_pages::tests::router_is_created_by_the_router_story`, `::router_is_inside_its_budgets`, `::rule_dir_holds_this_storys_two_atoms` and `::every_markdown_link_in_the_tree_resolves`. So a maintainer who relocates the tree and does not touch `xtask/src/` fails the build **in the same change**, which is exactly the property architecture brief AC-011 (`_decomposition.md:130-137`) asks the citation's link target to have. (2) `git mv standards/pages-moved standards/pages` -> `ls standards` -> `pages` and `rust`. `cargo xtask ci --fast` -> `=== every page declares one need === /   2 pages, 16 rules, all consistent` … `all required checks passed (--fast: 4 optional step(s) not run)`. (3) `git status --short` -> ` M .redkiln/telemetry/events/ryan-britton@docs-that-teach.jsonl`, ` M docs/README.md`, ` M docs/append-conditions.md`, ` M standards/pages/README.md` — this story's three edits and the pre-existing telemetry modification, no residue from the move. **Deviation recorded:** the walk was driven with `cargo xtask ci --fast` rather than the full `cargo xtask ci`, because `--fast` runs the same `tests` and `every page declares one need` steps that carry the observation and the full gate was run separately to green (see the Gates section of the implementation report). The `.with_context()` message itself was captured from `cargo run -p xtask -- lint-pages`, which is the step's own program. The vacuity `bail!` (`xtask/src/lint_pages.rs:436`) is the sibling guard for an *emptied* tree and is not exercised here; the missing-tree half is."
  mount_point: "docs/README.md and docs/append-conditions.md (the citations whose target the pin protects); observed through cargo xtask ci --fast and cargo run -p xtask -- lint-pages"
  verifying_test: "procedural, both directions captured verbatim: (1) git mv standards/pages standards/pages-moved && cargo xtask ci → fails, message naming the pinned path; (2) git mv back && cargo xtask ci → green; (3) git status → clean. Basis: xtask/src/lint_constitution.rs:212 and :176"
- id: AC-008
  criterion: "GIVEN the contributor who assumes that because this repository has a gate, the gate is watching this citation, WHEN they read _ledger.md and the router's `## What checks this tree, and what does not` section, THEN they are told plainly that at this merge the back-link is mechanically checked by the router's copied link half while the outbound link from docs/README.md is not checked by anything, and that the cross-project confirmation is a one-time procedural record — AND no shared scanner ranging over both trees was built to close that gap."
  satisfied: true
  evidence: "The blind spot is stated in this ledger's `## Blind spot, stated rather than implied (AC-008)` section, in three bullets and in one sentence each: the back-link **is** mechanically checked (`xtask/src/lint_pages.rs:817`, seen to fail under AC-004's mistyped-target capture); the outbound citations at `docs/README.md:14` and `docs/append-conditions.md:10` are checked by **nothing**; and what protects them is the **pin**, not a link check (AC-007). Gate-state, the PR boundary's largest promise: `git diff HEAD -- xtask` -> **empty**, and `git diff HEAD --stat -- xtask standards/rust spec .kb .redkiln/templates Cargo.lock` -> **empty, exit 0** — no checker change, no scanner over `docs/`, no `affected.rs` edit, and nothing in `standards/rust/`, `spec/`, `.kb/` or `.redkiln/templates/`. (`git diff main -- xtask` is *not* empty and is not the right base: it carries this project's own already-merged dependency stories — `git log --oneline main..HEAD -- xtask` names `ee0a500 Page-need checker mounted in the gate` and fifteen others from HS-P0020 and HS-P0021, none of them this story's. The story-scoped diff is the one that can prove this story's promise.) Mechanical: `rg -n 'What checks this tree, and what does not' standards/pages/README.md` -> `79:## What checks this tree, and what does not` — landed by `router-precedence-and-announcement`, read here, not edited. Prohibitions honoured: no scanner ranging over both `standards/pages/` and `docs/` was written (RS-81-3, `standards/rust/81-checks-that-cannot-be-types.md:209`; EC-008), and the cross-project half stays the one-time procedural record the testing brief required (`_decomposition.md:948-956`). **The non-author read-back of the asymmetry was performed and PASSED.** Walker: the **adversarial slice reviewer for `binding-beyond-this-project`**, the review pass that produced this slice's findings, which wrote no part of this ledger. Date: **2026-08-18**. Verdict transcribed verbatim: *from the ledger's blind-spot section alone I can state which half is checked (the back-link, by the copied link half at `xtask/src/lint_pages.rs:817`) and which is not (both outbound citations).* That is the whole of what the criterion asks a reader to be able to say. **What this replaced:** the residual previously recorded here was an implementer-performed judgement of the same section; it is superseded rather than kept in the row, per the precedent one slice back (`reviewer-and-citation-procedures/_ledger.md:84`)."
  mount_point: "docs/README.md and docs/append-conditions.md (the unchecked outbound half); standards/pages/README.md (the checked back-link half)"
  verifying_test: "gate-state: git diff HEAD -- xtask → empty and git diff HEAD --stat -- xtask standards/rust spec .kb .redkiln/templates Cargo.lock → empty, outputs pasted here — the commit-scoped base, because git diff main -- xtask carries this project's already-merged dependency stories (sixteen commits, named in the evidence) and cannot prove a promise this story makes; mechanical: rg -n 'What checks this tree, and what does not' standards/pages/README.md → 1; procedural: the blind-spot row stating the asymmetry plus a non-author read-back, both recorded here"
```

## EC-001 fired. Which pages carry the citation, and why

**The checker's walk excludes `README.md`.** Observed, not assumed, and observed **first**, before a
word was written (spec Implementation notes, bullet 1). The exclusion is
`xtask/src/lint_pages.rs:496` — `else if is_markdown(&name) && !name.eq_ignore_ascii_case("README.md")`
— the same `name != "README.md"` precedent the spec cites at `xtask/src/lint_constitution.rs:212-219`.

Probe A, run with the citation not yet written. A deliberately **unenumerated** token was inserted
into `docs/README.md` immediately under its H1 — a declaration that any governed page would be
failed for — and the checker said nothing:

```text
$ sed -n '1,4p' docs/README.md        # temporary, reverted
# Documentation

> **Answers:** `probe` — Is this page inside the checker's walk?

$ cargo run --locked --quiet -p xtask -- lint-pages
  2 pages, 16 rules, all consistent
```

Probe B, the same run against the page the walk **does** report. `docs/append-conditions.md`'s
declaration was deleted:

```text
$ cargo run --locked --quiet -p xtask -- lint-pages
  docs/append-conditions.md — no `> **Answers:**` line; see standards/pages/00-one-need.md

xtask failed: 1 problem(s) in standards/pages + docs
```

Both probes were reverted and the tree re-run green with `git status` clean (captured in AC-002).
`ls docs/*.md` lists three files and the checker reports **two pages**, which is the same fact
counted a second way.

**So EC-001's pre-decided tie-break fires, and the citation lands twice:**

| page | rule cited | why this page, and what it discharges |
| --- | --- | --- |
| `docs/README.md` (`:13-15`) | **RP-10-2** | the spec's decided citer and this story's mount point — the repository's documentation index, the page a reader lands on, and the page whose shape RP-10-2 dictates. It is **not** walked by the checker, so it carries the *in place, where the reader is* half of DoD-14 and cannot on its own carry AC-002's membership bar |
| `docs/append-conditions.md` (`:9-10`) | **RP-00-1** | EC-001's tie-break target: the governed page nearest the reader that the walk **does** report — first in `docs/README.md`'s own page table, first in path order, and HS-P0020's fixture page from `pinned-narrative-tree-and-compiling-step`. It carries the *cited by a page the discipline actually governs* half, and it is where AC-003's single-declaration and AC-005's H1/declaration assertions are observed |

**The rule cited differs per page because the citation must be true of the page it sits on.**
RP-10-2 governs an `orientation` page; `docs/append-conditions.md` declares `explanation`, so citing
RP-10-2 there would be an assertion about a rule that did not shape it — the decorative outcome the
spec's Executive summary point 2 refuses. RP-00-1 is what shapes `docs/append-conditions.md`, and the
sentence names that.

**Recorded divergence from the spec's own wording** (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`):
AC-003 and AC-005 were written expecting `docs/README.md` to carry a `> **Answers:**` declaration. It
does not, and it must not be given one — a declaration on a page no checker walks is a claim of
governance nothing extends, which is the decoration this initiative refuses. Those two rows are
therefore observed on the citing page that **is** governed, `docs/append-conditions.md`, and the
`docs/README.md` measurements are recorded beside them rather than dropped. The reasoning is
untouched; only the referent moved.

## Blind spot, stated rather than implied (AC-008)

At this merge the two halves of the relationship are **not** equally protected, and saying "the links
are checked" would be exactly the false guarantee RS-81-1 names
(`standards/rust/81-checks-that-cannot-be-types.md:11`):

- **The back-link is mechanically checked.** `standards/pages/README.md`'s `../../docs/README.md`
  is resolved by the checker's copied link half as `root.join(RULE_DIR).join(target)` and asserted to
  exist (`xtask/src/lint_pages.rs:817`, and again by
  `lint_pages::tests::every_markdown_link_in_the_tree_resolves`). Seen to fail, not assumed — see
  AC-004.
- **The outbound citations are checked by nothing.** Neither `docs/README.md:14` nor
  `docs/append-conditions.md:10` has its link to `standards/pages/` resolved by any gate step. This
  project owns no scanner over the governed tree and may not build one (RS-81-3,
  `standards/rust/81-checks-that-cannot-be-types.md:209`; architecture brief Note 6), and the testing
  brief already ruled this half a one-time procedural confirmation (`_decomposition.md:948-956`).
- **What protects them instead is the pin, not a link check.** `RULE_DIR` and `ROUTER` are `const`s
  the checker reads and a missing tree is `.with_context()`, never a skip — so the target cannot be
  relocated in silence. AC-007 observes that in both directions.

No shared scanner ranging over both trees was built. `git diff HEAD -- xtask` is empty.

## The three non-author judgements, run at review

Three rows carried the same residual at first merge — an implementer-performed judgement with a
non-author run owed later. All three have now been performed by the **adversarial slice reviewer for
`binding-beyond-this-project`** (the review pass that produced this slice's findings, which wrote no
part of this story's diff, this ledger or its implementation report) on **2026-08-18**, and the
author-run captures have been dropped from the rows as the precedent one slice back does it
(`reviewer-and-citation-procedures/_ledger.md:84`).

| AC | judgement | verdict |
| --- | --- | --- |
| AC-001 | read-back of the citation sentence in isolation: is the property RP-10-2 dictates nameable from it alone? | **PASS** — from `docs/README.md:13-15` alone: the page routes; the library is explained on the pages below, not here. A reason, not a see-also |
| AC-003 | RP-40-2's paraphrase spot check over both citation sentences | **PASS** — neither citation reproduces RP-10-2's or RP-00-1's rule text; the only shared token is the need name being pointed at |
| AC-008 | read-back of the blind-spot section: can a reader state which half is checked and which is not? | **PASS** — checked: the back-link, by the copied link half at `xtask/src/lint_pages.rs:817`. Unchecked: both outbound citations |

## Carried forward, not fixed here: what actually binds `docs/README.md`'s routing claim

`docs/README.md:13-15` asserts *"It routes rather than teaches"* on a page that also carries an
explanatory paragraph at `:36-55` and a `## What belongs here` section at `:57-62`, and that carries
no `> **Answers:**` declaration because the checker's walk excludes it
(`xtask/src/lint_pages.rs:496`). **Nothing binds or falsifies that claim**, so the governed half of
DoD-14's third clause rests entirely on `docs/append-conditions.md:9-10` — the page the walk *does*
report — and not on the index.

No change is made here, and that is deliberate rather than an omission: the explanatory prose at
`:36-62` predates this story, and rewriting a referent to make this story's own reasoning read better
is what `.kb/governance/rewrite-the-referent-never-the-reasoning.md` forbids. EC-001's pre-decided
tie-break already covered the gap by landing the citation twice. **The forward note:** when HS-P0022
adds a teaching page that the checker's walk *does* report, that page's citation of the discipline
should become the **primary** citer of DoD-14's third clause, with `docs/README.md`'s sentence
demoted to the in-place index half it can actually carry.

> The machine-readable acceptance block that used to sit here has been **moved up**,
> to immediately below this file's opening prose. Nothing in it changed — not a row,
> not a criterion, not a byte of evidence. `redkiln verify --grain story` locates the
> block by scanning from the `# … ledger` heading for the first fence and **abandons
> the search the moment it meets another heading** (`ledgerBlock`, redkiln 0.19.0
> `dist/index.js:14761-14779`), so the `## EC-001 fired` section standing between the
> heading and the fence made this ledger read as *having no acceptance block at all*
> rather than as having a malformed one. The narrative sections below are the story's
> evidence and keep their order; only the fenced block moved.
