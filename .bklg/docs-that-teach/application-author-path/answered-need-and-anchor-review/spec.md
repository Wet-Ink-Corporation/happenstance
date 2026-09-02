---
item: HS-S0190
stage: spec
created: 2026-08-17T13:16:37.043Z
updated: 2026-08-17T13:16:37.043Z
template_sig: 87bbf1d0
rendered_sig: 713c5612
---

# Spec — Each page's answered need and every anchor reviewed

## Scope lock

| | Path and the part that binds this story |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — **AC-08** (`:383-385`, the prior model named "once, consistently, rather than differently on each page"), AC-07, AC-12, and **DoD scenario 8** (`:435-438`, "a review pass over the set finds no page carrying two — with the check being one a reviewer can actually perform rather than one that depends on the author's memory") |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the DAG; why HS-P0021 owns the rule and this project only obeys it |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` — **AC-009** (`:260-262`), **AC-012** (`:269-271`), DR-07, DR-14, DoD item 7 (`:296-297`) and DoD item 9's routing (`:300-302`) |
| This spec | `.bklg/docs-that-teach/application-author-path/answered-need-and-anchor-review/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` — UX brief IQ-8 (`:299-303`), **UX-008** (`:348-349`), **UX-009** (`:350-352`), the primitives table (`:112-190`) and its "no second notation" rule (`:186-188`); testing brief tier 5 (`:486`), the AC→tier map rows for AC-009 and AC-012 (`:530`, `:533`) |
| Signed-off design (**BINDING**) | `.bklg/docs-that-teach/application-author-path/_design.md` — DT-1's single recorded location (`:108-118`), the four surfaces (`:45-65`), the answered-need row of `## Transience policy` (`:520`), anti-patterns 6, 11, 12 and 13 (`:852-874`), gap 3 (`:900-903`), sign-off (`:1039`) |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` — this story's row (`:62`), why `page-set-assurance` is last and set-wide (`:79-84`), the cross-cutting obligations that are deliberately not stories (`:128-132`) |
| Discipline owner (procedure borrowed, never invented) | `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` (the closed `NEEDS` set and the declaration form, `:71-107`) and `.bklg/docs-that-teach/page-need-discipline/reviewer-and-citation-procedures/spec.md` (the non-author walk, the four verdicts, the paraphrase spot check) |
| Checker owner (mechanical half, consumed) | `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md` — the step `every page declares one need`, its corpus `xtask::narrative::TREE`, and the halt-loudly rule when that const is absent (`:305-306`) |
| Substrate owner (tree, harness) | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — `TREE = "docs"`, the harness, the registration states |
| Slice-mate | `.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/spec.md` — the other half of `page-set-assurance`; the split is by mechanism, not by subject |
| Roadmap pointer | `RUNBOOK.md` — documentation work, no phase added. The roadmap of record is `_storymap.md`'s merge order. |

## One-line PR slice

Walk the whole page set with a non-author reviewer applying HS-P0021's own check —
exactly one named answered-need per page, above the first fence — confirm the DT-1
anchor decision is applied identically everywhere with each relying page citing the one
recorded location, and record the result as project DoD item 7.

## Executive summary

**What this PR lands.** One executed review and its evidence: a per-page verdict for all
four surfaces of this project's page set, a per-page statement of *what actually looked at
that page* (the gate, or a person), the one-line repairs the walk found, and the recorded
result that discharges project DoD item 7 (`project.md:296-297`). It lands **no teaching
content**, no rule text, no checker and no page.

**Pointer, not restatement.** The obligations are project AC-009 and AC-012; the
procedure is HS-P0021's (`page-need-discipline/reviewer-and-citation-procedures/spec.md`);
the notation is HS-P0021's (`need-vocabulary-and-declaration-form/spec.md`); the single
recorded location of the DT-1 decision is `_design.md:108-118` and its reader-facing home
is the bridge page's `#where-your-streams-went`, which
`invariant-to-appendcondition-bridge` ships. This story invents none of the four and is a
defect the moment it does — DR-14 names a second notation as the failure, and the same
sentence covers a second procedure and a second recorded home.

**The delta this spec adds, over the storymap's one-liner.** Three things, each a decision
rather than a task:

1. **A walk that has never returned `fail` is decorative** — CLAUDE.md's own rule for
   conformance rules, applied to a written procedure. This story therefore does not only
   sweep the set; it *calibrates*, in both directions, against a page of this project's own
   material temporarily carrying two declarations, and records the failure and the revert.
2. **The set is not uniformly machine-visible, and saying so is the deliverable.**
   HS-P0021's checker sweeps `xtask::narrative::TREE` and nothing else
   (`page-need-checker-mounted-in-the-gate/spec.md:305`), which is `docs/`
   (`checked-documentation-surface/_design.md`, `TREE`). Three of the four surfaces are in
   it. `crate-root-encounter` is `crates/happenstance/src/lib.rs` — a rustdoc doc comment
   with no markdown `# Title` for a declaration to sit under — and **no mechanism in this
   repository sees its answered-need line at all**. An audit that reports "4/4 declare one
   need" without that column is a reassurance, which is the artifact this initiative exists
   to refuse. The gap is recorded and routed, never counted as coverage.
3. **The anchor half is a link claim, not an opinion.** AC-009 decomposes into two
   mechanically checkable facts — the prior model is *named* on exactly one page, and every
   page relying on it *links* to that one heading — plus one that only a person can settle:
   whether a page re-argued the decision in its own words instead of citing it. The first
   two are run as commands and the outputs pasted; only the third is judgement.

## Context pack

Read this and you can start. Everything deeper is a signposted anchor in the second half of
this spec — link, never paste.

### The decisions this story is downstream of, stated as decisions

**The notation is HS-P0021's and is consumed exactly.** The declaration is a single
blockquote line, `> **Answers:** \`token\` — <the reader's question>?`, placed immediately
after the page's `# Title` with one blank line above and below, and **nothing may be
inserted between the H1 and it** — no badge row, no table of contents, no "last updated"
line — because the check is *read nothing but the region above the first prose paragraph
and name the need*, and any interposed element makes that region ambiguous
(`need-vocabulary-and-declaration-form/spec.md:92-107`). The token is one member of the
closed set `orientation` · `tutorial` · `how-to` · `explanation` (`:71-82`). The three
tree pages have already chosen theirs and this story does not re-open them: `first-encounter`
is the encounter's, `docs/carry-your-invariant.md` declares `explanation`
(`invariant-to-appendcondition-bridge/spec.md:374`), `docs/read-the-worked-example.md`
declares `orientation` (`surface-course-subscriptions/spec.md:293`). **Inventing a second
notation is DR-14's named defect** (`project.md:224-226`); so is inventing a second *check*
of it.

**Position and cardinality are this project's design and stand regardless.** Exactly one
declaration per surface, above everything and therefore above the first fence in reading
order (`_design.md:520`; IQ-8 at `_decomposition.md:299-303`; UX-008 at `:348-349`). Where
HS-P0021's form is unavailable, position and cardinality are still binding and the *form*
is blocked on HS-P0021 rather than substituted (`_design.md:900-903`). Anti-pattern 12 is
the reviewer-facing spelling: *"two answered-need statements on one page, or one that
appears below the first code block"* (`_design.md:869-870`).

**The DT-1 decision has exactly one provenance and exactly one reader-facing home, and
they are different files on purpose.** `_design.md:108-118` is the provenance — it records
*why* the teaching is invariant-first with the stream-per-entity prior named once. The
reader-facing statement is the bridge page's `## Where your streams went`, slug
`#where-your-streams-went`, and *"every other page that relies on the anchor decision links
to that heading and does not re-argue it"* (`_design.md:108-114`). That discharges UX-009,
DR-07 and AC-009 with one anchor rather than a repeated paragraph, and the repeated
paragraph is precisely the defect BR-07 exists to prevent
(`project.md:336`, Risks row 2). **A second recorded home is the same defect as a second
notation.**

**The consequence that makes AC-009 checkable rather than judged.** The design binds the
other surfaces: `crate-root-encounter` and `opening-encounter` name **no** prior model at
all, and anti-pattern 6 states the check as words a person can grep for — *"the words
'aggregate', 'your aggregates', 'one stream per entity' or 'which stream' on the crate root
or on any step of the opening encounter"* — the prior model is named on exactly one page
and nowhere else (`_design.md:852-854`, `:116-118`). The bridge's own spec already commits
to the mirror of it: those four phrases appear *"on this page and nowhere else in this
project's output"* (`invariant-to-appendcondition-bridge/spec.md:430`).

**The procedure is HS-P0021's non-author walk, and its shape is fixed.** The walk lives at
`standards/pages/40-reviewing-a-page.md#the-walk` under `## RP-40-1`; the performer is
**not the author**; the reviewer may consult the rendered page, the router and
`spec/SPECIFICATION.md`, and may **not** consult the author or the page's git history; every
step is a question with a yes/no answer, and the walk closes in a **four-row** verdict table
— `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate`, where
`indeterminate` means the walk could not be completed from the page alone and is recorded as
**a defect in the page, not in the procedure**
(`reviewer-and-citation-procedures/spec.md:139-144`, `:424`, `:491-493`). There is no soft
pass available, and this story does not create one.

**An empty or unreachable corpus is recorded as vacuous, never as green.** HS-P0021's
procedure says so in its own text, and its ledger records the sweep it could actually
perform as vacuous rather than as a pass
(`reviewer-and-citation-procedures/spec.md:180-184`, `:494`). The same rule binds here one
level out: if a page of this set does not exist at the time of the walk — because a
blocking story has not landed — that page's row reads *not yet authored*, and the story does
not complete. `RUNBOOK.md:920-925` is the recorded cost of the inverse.

**The mechanical half is HS-P0021's and must not be rebuilt.** `cargo xtask lint-pages`,
mounted as the `REQUIRED` step **`every page declares one need`**, reads
`xtask::narrative::TREE` and *"declares no `PAGE_DIR` of its own"* — the *three lists that
must agree* defect, foreclosed rather than documented
(`page-need-checker-mounted-in-the-gate/spec.md:305`, `:369`). This story adds no `xtask`
code, no second corpus and no second checker. Where it needs a mechanical answer it runs
that step and pastes the output.

**What this story may not do.** Author a page. Edit `_design.md` (signed off, `:1039`).
Re-decide DT-1, DT-4, DT-5 or DT-6. Write a rule atom under `standards/pages/**` — bands
`00`–`40` are HS-P0021's and adding a fifth is not this project's mandate. Add a lint,
including the content-level accessibility lint `_design.md:909-914` explicitly routes to
HS-P0021. Where a page it walks is defective, it repairs at **one-line grain** — a missing or
duplicated declaration line, a re-argued paragraph replaced by a link to
`#where-your-streams-went` — and anything larger is a stop-and-route under DoD item 9
(`project.md:300-302`), not a rewrite performed here.

### The seam this story is the first to have to state

Three of the four surfaces are markdown under `docs/`, which is exactly the corpus HS-P0021's
checker sweeps. The fourth is not, and it is not an oversight: `crate-root-encounter` lives
in `crates/happenstance/src/lib.rs` because *"the docs.rs reader who ran `cargo add
happenstance` lands there first and the whole measured defect is what that page shows them"*
(`_design.md:707-711`), and it cannot be moved into the tree because `include_str!` resolves
against the file tree at compile time and a path escaping the package would not resolve once
published (`crates/happenstance/src/lib.rs:7-9`). Two consequences follow and both belong in
the recorded result rather than in a footnote:

1. **Nothing mechanical reads the crate root's answered-need line.** Its verdict is a
   person's, and the audit says so per page instead of averaging it away. This is the exact
   shape the slice-mate found for fences — *"for the crate-root fence the answer is no: an
   `ignore` there makes rustdoc skip it and the gate stays green… saying so in the inventory
   is the difference between an audit and a reassurance"*
   (`fence-inventory-and-clause-audit/spec.md`, `## Executive summary` item 2). The two
   stories mount together and their coverage tables must not disagree about the same file.
2. **The declaration's anchoring element differs there.** HS-P0021's form is anchored to a
   markdown `# Title`; a rustdoc crate-root doc comment has no `#` H1 — rustdoc renders the
   crate name as the page's heading. The design already fixed the *position* independently of
   the form: immediately below the one-line crate summary and above every `##`, therefore
   above the first fence by construction (`_design.md:428-431`). This story records how the
   form landed against that position and, if the two cannot both be honoured, routes the
   question to HS-P0021 as a form gap rather than settling it by writing a variant.

### The persona-journey slice this realizes

Backbone activity **A6** — *"I want to trust the whole set, not one page"*
(`_storymap.md:46`). Persona 1 never reads this story's output. What reaches them is that
every page they open tells them what it is for before they invest in it, and that the one
question they will reflexively ask — *which stream does this go in?* — is answered in one
place they can point at rather than answered slightly differently three times. Persona 1's
stated fear is silent wrongness, *"a mental model that looks right, compiles, runs, and is
quietly wrong"* (`_decomposition.md:28-30`); a page set that reads as complete while quietly
answering two needs, or that anchors against a prior model on page two and against a
different one on page three, is that fear at the level of the set. The reviewer this story
actually serves is HS-P0021's **S4, the non-author reviewer**, whose success condition is
*"reach the same verdict the author would, from the page alone"*
(`reviewer-and-citation-procedures/spec.md:244-245`).

## Integration contract

- **Archetype**: `capability` — user-observable, where the user is the reader of the whole
  set and what they observe is the absence of a defect. That is why the calibration is
  load-bearing: an absence is demonstrated by showing the instrument can detect a presence,
  not by asserting it.
- **Slice / milestone**: `page-set-assurance`. **Slice-mate**: `fence-inventory-and-clause-audit`
  (HS-S0189), independent of this story and delivered in the same slice
  (`_storymap.md:149-151`). The split is by mechanism, not by subject: that story is what
  `spec-trace` and the allowance list can decide, this one is what only a reviewer can
  (`_storymap.md:82-84`). Both mount last, together, over the assembled set.
- **Mount point**: **`xtask/src/narrative.rs`** — HS-P0020's registration harness, whose
  `TREE` const is the single in-tree definition of which pages are governed at all, and which
  HS-P0021's checker reads rather than declaring a second `PAGE_DIR`
  (`page-need-checker-mounted-in-the-gate/spec.md:305`). It is the real composition root for
  this story because *membership in the set this story makes a claim about is decided there*:
  a page under `docs/` that the harness does not name is unregistered, and a page outside
  `TREE` is invisible to the `every page declares one need` step no matter how correct its
  declaration is. The mount is observable two ways — every page in this project's set is
  either named there or is explicitly recorded in the coverage table as outside the corpus
  with its reviewer named. Where the walk finds a page of this set missing a registration
  line, this story adds the line; that is the mount, not scope drift. **If HS-P0020 pinned a
  different filename, use the file it pinned and record the deviation — never stand up a
  parallel harness** (`_decomposition.md:118-131`).
- **Wires into**:
  - `xtask/src/main.rs` — the `REQUIRED` array. The step **`every page declares one need`**
    (`cargo run --locked --quiet -p xtask -- lint-pages`, `probe: None`) is HS-P0021's mount
    and this story's mechanical instrument; also `"documentation"` (`:290-301`, `cargo doc`
    under `RUSTDOCFLAGS=-D warnings`, which is what fails an inbound intra-doc link to
    `#where-your-streams-went` that does not resolve) and `"tests"` (`:143-155`).
  - `standards/pages/00-one-need.md`, `standards/pages/10-the-need-set.md` and
    `standards/pages/40-reviewing-a-page.md#the-walk` — HS-P0021's rule atoms and the walk
    itself, read and executed, never edited.
  - `standards/pages/examples/two-needs.md` — HS-P0021's inert two-declaration specimen,
    deliberately outside the governed tree, walked here as the second calibration point
    (`reviewer-and-citation-procedures/spec.md:495`).
  - `crates/happenstance/src/lib.rs`, `docs/first-encounter.md`,
    `docs/carry-your-invariant.md`, `docs/read-the-worked-example.md` — the four authored
    surfaces, produced by the five stories this one blocks on; read, and repaired only at
    one-line grain.
  - `docs/README.md` — the tree's narrative index, whose rows are how a page is reachable at
    all; read, and used to enumerate the set the walk must cover so a page cannot be missed
    by being forgotten.
  - `.redkiln/config.yaml` — `affected_gate` (`:40`), `reachability_static` (`:48`),
    `integration_scoped` (`:55`) and `require_ledger` (`:67`): the commands redkiln runs at
    this story's and this project's grains whether or not anyone types them.
- **Design-system primitives consumed**: HS-P0021's declaration blockquote and its closed
  `NEEDS` token set; the four-row verdict table; the `docs/README.md:12-24` two-column table
  shape (read only — the pointer policy is HS-P0023's). No new primitive is introduced, and
  the correct answer to "enumerate the affordances this story introduced" is none (IQ-6,
  `_decomposition.md:287-291`).
- **Renders surfaces**: **none newly.** This story walks all four ids from `_design.md`'s
  `## Surfaces` block (`:45-65`) — `crate-root-encounter`, `opening-encounter`,
  `conceptual-bridge`, `worked-example-handoff` — and **changes** one only where the walk
  finds a defect on it. Composition, hierarchy, density and states are the signed-off
  design's and are not reopened.
- **Public items**: **none.** `_design.md`'s `## Items` block records that this project adds,
  changes and removes no public Rust API item (`:268`), and this story adds no code at all.
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No port, store,
  fixture or `suite.rs` rule is touched; `happenstance-testkit`'s suite observes stores and
  cannot see a markdown declaration line. Naming a rule here would be decorative by
  CLAUDE.md's own test. What observes this story is the gate — `every page declares one
  need`, `"documentation"`, and `cargo xtask affected --base main` at the checkpoint — plus
  the procedural tier, which is a person and is recorded as one.
- **Clause(s)**: **none discharged, none amended.** The initiative is additive and discharges
  no clause (`project.md:155-157`). Clause ids are *cited* by the pages this story walks;
  whether a citation resolves and whether a clause is restated is the slice-mate's and
  HS-P0021's, not this story's (`page-need-discipline/project.md` AC-010).
- **Advances DoD scenario**: initiative **DoD-8** (`initiative.md:435-438`) — *"Every page's
  answered need is stated and singular… with the check being one a reviewer can actually
  perform rather than one that depends on the author's memory."* This story is the half that
  makes "actually perform" true of *this project's* set. It also discharges project **DoD
  item 7** (`project.md:296-297`) and is the tier-5 proof the testing brief assigns to AC-009
  and AC-012 (`_decomposition.md:530`, `:533`).

## PR boundary

```
crates/happenstance/src/lib.rs
docs/first-encounter.md
docs/carry-your-invariant.md
docs/read-the-worked-example.md
docs/README.md
xtask/src/narrative.rs
.bklg/docs-that-teach/application-author-path/answered-need-and-anchor-review/**
```

**In this PR.**

- The executed non-author walk over the four surfaces, per HS-P0021's procedure, with a
  verdict from the four-row table for each page.
- The **coverage column**: for each page, which instrument actually observed its declaration
  — the `every page declares one need` step, or a named person — and the pasted output of
  that step.
- The **calibration, in both directions**: HS-P0021's inert specimen walked to
  `fail — two needs`, and a second declaration temporarily injected into one page of *this*
  project's set, the walk (and, for a `docs/` page, the gate step) observed to fail by name,
  then reverted and observed to pass. The tree ends clean.
- The **anchor sweep**: the phrase search proving the prior model is named on
  `docs/carry-your-invariant.md` and nowhere else in this project's output; the link check
  proving every page relying on the decision resolves into `#where-your-streams-went`; and
  the reviewer's judgement on whether any page re-argues rather than cites.
- **One-line repairs only** where the walk finds a defect: a missing, duplicated or
  mispositioned declaration line; a re-argued paragraph replaced by a link to the one
  recorded location; a missing harness registration line for a page of this set.
- The recorded result, dated and attributed to a named non-author, discharging project DoD
  item 7 — including every row that is *vacuous*, *blocked* or *indeterminate*, recorded in
  those words.

**Explicitly not in this PR.**

- **Authoring or re-composing any page.** The four surfaces are the five blocking stories';
  a defect larger than one line stops and routes.
- **The fence inventory and the clause-citation audit.** The slice-mate's (HS-S0189), same
  slice, different mechanism.
- **Any rule text, atom or band under `standards/pages/**`, and any `xtask` code.** The
  notation, the walk, the checker and the corpus const are HS-P0021's and HS-P0020's. No
  second notation, no second procedure, no second `PAGE_DIR`, no new lint — including the
  accessibility lint `_design.md:909-914` routes to HS-P0021.
- **Editing `_design.md`** (signed off, `:1039`) or re-deciding DT-1/DT-4/DT-5/DT-6.
- **Any addition to `docs/README.md:12-24`'s pointer-out table.** DT-10 and every pointer
  policy are HS-P0023's; `docs/README.md` is in the boundary only for the narrative index row
  a repair may need, and for reading the set's membership.
- **Amending or discharging a `spec/SPECIFICATION.md` clause**, and any `.kb/` atom —
  promotion is HS-P0025's and no `.kb/` atom is hand-authored in this project
  (`project.md:144-147`).

**Merge DoD one-liner** — a named non-author has walked all four surfaces with HS-P0021's own
procedure and recorded a verdict for each; the walk is demonstrated able to return `fail` and
the tree is clean afterwards; the prior model is named on exactly one page with every relying
page resolving into `#where-your-streams-went`; every page whose declaration no machine can
see is recorded as such and routed; and `cargo xtask affected --base main` is green at the
checkpoint with `cargo xtask ci --fast` green at the project bar.

## Behavior and interfaces

This story ships no API. Its "interfaces" are a borrowed procedure, a borrowed gate step, and
two artifacts whose shape is fixed here so review is against the thing rather than a
description of it.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The set is enumerated before it is walked, and enumeration is not from memory** | The page set is the four surface ids of `_design.md`'s `## Surfaces` block resolved to real paths: `crates/happenstance/src/lib.rs` (`crate-root-encounter`), `docs/first-encounter.md` (`opening-encounter`), `docs/carry-your-invariant.md` (`conceptual-bridge`), `docs/read-the-worked-example.md` (`worked-example-handoff`). It is cross-checked two ways — the `docs/README.md` narrative index rows, and `git diff main --name-only` over the five blocking stories' output — so a page authored and then forgotten cannot pass by absence. A fifth page found by either method is a finding, not a row to add quietly. | `_design.md:45-65`, `:707-747`; `boundary-refusal-encounter/spec.md:303-304`; `invariant-to-appendcondition-bridge/spec.md:304`; `surface-course-subscriptions/spec.md:239` |
| **The walk is HS-P0021's, executed by a non-author, and nothing else is executed** | `standards/pages/40-reviewing-a-page.md#the-walk` under `## RP-40-1`, top to bottom, from the rendered page alone. The walker is not the author of the page and does not consult the author or the page's git history. Each step is answered yes/no; the answers are recorded per page, not summarised. | `reviewer-and-citation-procedures/spec.md:491-492`; `_decomposition.md:486` (tier 5) |
| **Every page lands on exactly one of four verdicts** | `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate`. Four, not three. `indeterminate` means the walk could not be completed from the page alone and is a **defect in the page**, not in the procedure — there is no soft pass and this story does not add one. A verdict of `fail` or `indeterminate` on any page blocks this story until repaired or routed. | `reviewer-and-citation-procedures/spec.md:139-144`, `:424`, `:493`; `_design.md` `## States` row S5 as cited there |
| **Exactly one declaration per page, in the exact form, above the first fence** | `> **Answers:** \`token\` — <question>?` as a single blockquote line, immediately after the page's H1 with nothing interposed, token from the closed set `orientation`/`tutorial`/`how-to`/`explanation`, question in the reader's voice ending in `?`. Count is exactly one; position is above everything and therefore above the first fence. A second notation, however it renders, is a defect. | `need-vocabulary-and-declaration-form/spec.md:71-107`; `_decomposition.md:348-349` (UX-008), `:299-303` (IQ-8); `_design.md:520`, `:869-870` (anti-pattern 12) |
| **The mechanical half is run, not assumed, and its corpus is stated** | `cargo run --locked --quiet -p xtask -- lint-pages` (the `REQUIRED` step `every page declares one need`) is executed and its output pasted. Its corpus is `xtask::narrative::TREE`; the three `docs/` pages are inside it and `crates/happenstance/src/lib.rs` is not. No second corpus is declared and no second checker is written. | `page-need-checker-mounted-in-the-gate/spec.md:305`, `:320`, `:369`; `xtask/src/main.rs` `REQUIRED` |
| **Coverage is reported per page, never averaged** | One row per surface with three columns: the instrument that observed the declaration (gate step / named person), whether that instrument *could have failed* on this page, and the verdict. `crate-root-encounter`'s middle column is **no** — nothing in the repository reads its declaration — and that cell is the reason the row exists. The slice-mate's inventory reaches the same conclusion about the same file for fences; the two tables must not disagree. | `fence-inventory-and-clause-audit/spec.md` `## Executive summary` item 2; `_design.md:707-726` |
| **The procedure is demonstrated able to return `fail`, in both directions** | Two calibrations. (a) HS-P0021's inert specimen `standards/pages/examples/two-needs.md`, walked to `fail — two needs`. (b) A second declaration temporarily injected into one page of this project's own set: the walk returns `fail — two needs` and, for a page under `docs/`, the gate step fails naming that page by `path:line`; the injection is reverted and both return to green, with the reverted tree shown clean. A procedure that has never returned `fail` is decorative — CLAUDE.md's rule for conformance rules, applied to a written one. | `reviewer-and-citation-procedures/spec.md:495`, `:256`; CLAUDE.md, "A rule that no adapter can fail is decorative" |
| **The prior model is named in exactly one place** | A case-insensitive search for `aggregate`, `your aggregates`, `one stream per entity` and `which stream` over this project's authored spans returns hits **only** in `docs/carry-your-invariant.md`'s `## Where your streams went`. Anti-pattern 6 is the reviewer-facing spelling of the same check and is performable on a rendered page. | `_design.md:116-118`, `:852-854`; `invariant-to-appendcondition-bridge/spec.md:430` |
| **Every relying page cites the one recorded location, by a resolving anchor** | Each page that depends on the DT-1 decision links to `docs/carry-your-invariant.md#where-your-streams-went` — the slug is stable, human-readable and unnumbered, so inserting a section breaks no inbound link (IQ-4). Resolution is proven, not assumed: the heading exists with that exact text, each link target resolves, and `cargo doc` under `RUSTDOCFLAGS=-D warnings` fails any unresolved intra-doc link from the crate root. | `_design.md:108-114`; `invariant-to-appendcondition-bridge/spec.md:372`, `:460`; `_decomposition.md:269-276` (IQ-4); `xtask/src/main.rs` `"documentation"` step |
| **"Applied identically" includes not re-arguing it, and that half is judgement** | The reviewer reads each relying page and answers one yes/no question: does this page *state* the prior-model decision in its own words, or does it *cite* the one location? A restated version is a second recorded home even when it agrees, which is the defect BR-07 exists to prevent. This is the only step in the story a command cannot settle, and it is recorded as a judgement with the sentence it turned on. | `project.md:260-262` (AC-009), `:336` (Risks row 2); `_decomposition.md:350-352` (UX-009) |
| **Repairs are one line, and anything larger stops** | Permitted in-tree edits: add/remove/reposition a declaration line; replace a re-argued sentence or paragraph with a link to `#where-your-streams-went`; add a missing harness registration line. Everything else — a page needing a new section, a token needing to change, a form that cannot be expressed on the crate root — is recorded and routed under project DoD item 9 (substrate → HS-P0020, notation and rule → HS-P0021, pointer and reach → HS-P0023, comprehension → HS-P0024, incidental bugs → the `support` initiative). Nothing is absorbed silently. | `project.md:300-302`; `_storymap.md:158-163`; `.redkiln/config.yaml` |
| **Vacuous, blocked and unauthored states are recorded in those words** | If a page does not exist when the walk runs, its row reads *not yet authored* and this story does not complete. If HS-P0021's atoms or checker are absent, the mechanical column reads *no instrument* against every row and the gap is routed — the form's absence blocks the criterion rather than licensing a placeholder notation. A green record standing in for a check that never ran is the failure `RUNBOOK.md:920-925` already paid for. | `reviewer-and-citation-procedures/spec.md:180-184`, `:494`; `_design.md:900-903`; `boundary-refusal-encounter/spec.md:563` (EC-003) |
| **The recorded result is the deliverable, and it is dated and attributed** | A per-page table carrying: surface id, path, declared token, verdict, instrument, walker identity (a named non-author), date, and — where applicable — the routing destination. It is the artifact project DoD item 7 asks for, and it is written into this story's `_ledger.md` against the AC ids below rather than into a prose summary a later reader cannot audit. | `project.md:296-297`; `.redkiln/config.yaml:67` (`require_ledger`) |

## Data and migrations

**N/A — no data, no schema, no migration.** This story adds no Rust code, no `const`, no
file format and no persisted state; its only in-tree writes are single lines of markdown or
doc comment inside files that already exist, plus at most one `include_str!` registration
line in the harness. The one thing that behaves like a schema is HS-P0021's declaration
grammar, and it is **consumed, never extended**: the token set is closed
(`need-vocabulary-and-declaration-form/spec.md:71-82`), the line shape is fixed (`:92-107`),
and a page needing a token outside the set is a routed finding for HS-P0021, not a local
addition. The nearest thing to a migration in scope — moving the crate root's declaration
into a form a machine can read — is deliberately *not* performed here: it would require
either relocating a page out of `crates/happenstance/src/lib.rs`, which `_design.md:707-726`
forbids with a stated reason, or widening the checker's corpus, which is HS-P0021's const to
change. Both are recorded and routed instead.

## Acceptance criteria

Eight criteria. Each is framed from the reader's or the reviewer's intent rather than as a
capability, because every one of them is a property of a *set* that a single page cannot
demonstrate. `Mechanical` verification is a command whose output is pasted into `_ledger.md`;
`Procedural (ledger-recorded)` is the testing brief's fifth tier (`_decomposition.md:486`),
which is proof here because no `#[test]` carries a judgement.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the reviewer is about to make a claim about *the whole set* — the thing Persona 1 actually depends on, per backbone activity A6 (`_storymap.md:46`) — and a page authored and then forgotten would pass by being absent rather than by being correct, **WHEN** the walk begins, **THEN** the set is enumerated *before* it is walked, from `_design.md`'s four surface ids (`:45-65`) resolved to real paths, and cross-checked twice against sources that cannot both be stale — `docs/README.md`'s narrative index rows, and `git diff main --name-only` over the five blocking stories' output — **AND** a page found by either cross-check and missing from the enumeration is recorded as a **finding**, never quietly appended to the table. | *Mechanical.* All three lists captured verbatim: the four surface ids and their paths; `rg -n '\]\(' docs/README.md`; `git diff main --name-only -- docs crates/happenstance/src/lib.rs`. The two set differences are computed and printed — both empty, or each difference named with its disposition. *Procedural (ledger).* One row stating the enumeration method, the date, and the walker; a fifth page, if any, carried into `## Clarifications` and routed, not absorbed. |
| AC-002 | **GIVEN** Persona 1, whose stated fear is silent wrongness (`_decomposition.md:28-30`) and who must be able to tell what a page is for *before* investing in it, **WHEN** a **named non-author** executes HS-P0021's walk (`standards/pages/40-reviewing-a-page.md#the-walk`, `## RP-40-1`) top to bottom **from the rendered page alone**, consulting neither the author nor the page's git history, **THEN** every surface in AC-001's enumeration carries **exactly one** declaration in HS-P0021's exact form — `> **Answers:** \`token\` — <question>?` as a single blockquote line, the **first element under the page's H1** with nothing interposed (no badge row, no table of contents, no "last updated" line), the token one member of the closed set `orientation`/`tutorial`/`how-to`/`explanation`, the question in the reader's voice ending in `?` — and therefore **above the first fence in reading order**; **AND** each page lands on **exactly one** of the four verdicts `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate`, with the per-step yes/no answers recorded rather than summarised; **AND** no page is at `fail` or `indeterminate` when this story completes. | *Mechanical.* Per `docs/` page: `rg -c '^> \*\*Answers:\*\*' <page>` → **1**; the first 12 source lines captured, showing `# Title`, one blank line, the declaration, one blank line, and nothing between; the page's **first fenced-block opening line**, located with `rg -n` over the fence marker, reports a line number **greater** than the declaration's, both numbers captured. Token membership checked against `need-vocabulary-and-declaration-form/spec.md:71-82`. *Procedural (ledger).* The walker's identity (declared non-author of that page), the date, the step-by-step answers, and the single verdict, per page — four rows, per `reviewer-and-citation-procedures/spec.md:491-493`. |
| AC-003 | **GIVEN** a later reader deciding whether to trust this record, for whom "4/4 declare one need" without a coverage column is the reassurance this initiative exists to refuse, **WHEN** the result is written, **THEN** it carries a **per-page coverage table** — never an average — with three columns per surface: the instrument that actually observed the declaration (the `every page declares one need` REQUIRED step, or a **named person**), whether that instrument **could have failed on this page**, and the verdict; **AND** `crate-root-encounter`'s middle column reads **no**, because `crates/happenstance/src/lib.rs` is outside `xtask::narrative::TREE` and **nothing in this repository reads its answered-need line**, with that gap routed rather than counted as coverage; **AND** the mechanical step is *run*, its corpus named, and its output pasted — no second corpus is declared and no second checker is written. | *Mechanical.* `cargo run --locked --quiet -p xtask -- lint-pages` executed and its full output pasted (the `REQUIRED` step `every page declares one need`, `probe: None`). The corpus is shown, not assumed: `rg -n 'TREE' xtask/src/narrative.rs` and the checker's read of it (`page-need-checker-mounted-in-the-gate/spec.md:305`, `:369`). Boundary proof: `git diff main --stat -- xtask/src` shows no new corpus const and no new checker. *Procedural (ledger).* The three-column table, one row per surface, with the `crate-root-encounter` row's **no** cell carrying its routing destination; the row is cross-read against the slice-mate's inventory row for the same file and the two are recorded as agreeing. |
| AC-004 | **GIVEN** that a procedure which has never returned `fail` is decorative — CLAUDE.md's own corollary for conformance rules, applied to a written one — and that this story's deliverable is an **absence**, which can only be demonstrated by showing the instrument detects a presence, **WHEN** the calibration is run in **both directions**, **THEN** (a) HS-P0021's inert two-declaration specimen `standards/pages/examples/two-needs.md` is walked to `fail — two needs`, **AND** (b) a second declaration is temporarily injected into one page of *this project's own* set, the walk returns `fail — two needs` and — for a page inside `TREE` — the gate step fails naming that page by `path:line`, **AND** the injection is reverted and both the walk and the step return to green with the working tree observed **clean**. | *Mechanical.* (b) is four pasted outputs in order: the injected diff; `cargo run --locked --quiet -p xtask -- lint-pages` failing with the offending `path:line`; `git checkout -- <page> && git status --porcelain` → **empty**; the same step green. (a) is `rg -c '^> \*\*Answers:\*\*' standards/pages/examples/two-needs.md` → **2**, and `ls standards/pages/*.md` **not** listing it, proving the fixture is inert (`reviewer-and-citation-procedures/spec.md:495`). *Procedural (ledger).* Both walks recorded step by step with the walker named; a `pass` on either is EC-006. |
| AC-005 | **GIVEN** the reader arriving with the stream-per-entity prior — who asks *which stream does this go in?* reflexively and will ask it again on every page until something answers it — and given that answering it slightly differently three times is initiative AC-08's named failure (`initiative.md:383-385`), **WHEN** the reviewer sweeps this project's authored spans, **THEN** the prior model is **named on exactly one page**: hits for `aggregate`, `your aggregates`, `one stream per entity` and `which stream` fall **only** inside `docs/carry-your-invariant.md`'s `## Where your streams went` span, and **nowhere** on the crate root or on any step of the opening encounter. | *Mechanical.* `rg -ni 'aggregate\|your aggregates\|one stream per entity\|which stream' crates/happenstance/src/lib.rs docs/first-encounter.md docs/carry-your-invariant.md docs/read-the-worked-example.md docs/README.md` — full output with line numbers pasted, and every hit's line number shown to fall inside the `## Where your streams went` section's start/end lines, which are captured separately. *Procedural (ledger).* Anti-pattern 6 (`_design.md:852-854`) restated as the reviewer-performable form and run against the rendered pages, so the check survives a phrase the grep did not anticipate. |
| AC-006 | **GIVEN** the reader two pages downstream who needs the anchor decision but must not be made to read it twice, **WHEN** any page relies on the DT-1 decision, **THEN** it **links** to the one recorded reader-facing location — `docs/carry-your-invariant.md#where-your-streams-went`, a real `##` heading whose slug is stable, human-readable and unnumbered so inserting a section breaks no inbound link (IQ-4) — **AND** that heading exists exactly once with exactly that text, **AND** every such link is proven to **resolve** rather than assumed to: the intra-doc link from the crate root is denied by `RUSTDOCFLAGS=-D warnings` if it does not, and no literal `[bracketed]` code-styled word (anti-pattern 2, the visible signature of an unresolved reference) survives on any page this project touches. | *Mechanical.* `rg -n '^## Where your streams went$' docs/carry-your-invariant.md` → **1**. `rg -n 'where-your-streams-went' crates/happenstance/src/lib.rs docs/` — every relying page enumerated, each target resolved by hand and the resolution pasted. The `"documentation"` REQUIRED step (`xtask/src/main.rs:290`) run green under `RUSTDOCFLAGS=-D warnings`. Anti-pattern 2: `rg -n '\[\`[a-z_:]+\`\]' crates/happenstance/src/lib.rs` → no unresolved pair. *Procedural (ledger).* The router→anchor traverse run **keyboard-only** from the crate root and recorded as `crate root → bridge → #where-your-streams-went`, landing on the heading. |
| AC-007 | **GIVEN** that a page which *re-states* the decision — even in agreement — has become a second recorded home, which is exactly the defect BR-07 exists to prevent (`project.md:336`), and that no command can tell a citation from a paraphrase, **WHEN** the reviewer reads each relying page, **THEN** they answer one yes/no question per page — *does this page state the prior-model decision in its own words, or does it cite the one location?* — **AND** the answer is recorded as a **judgement with the sentence it turned on quoted verbatim**, never as a tick; **AND** any page found re-arguing is repaired at one-line grain by replacing the restatement with a link to `#where-your-streams-went`, with the before and after both recorded. | *Procedural (ledger), and this is the only step in the story a command cannot settle.* One row per relying page: the page, the verdict `cites` / `re-argues`, and the sentence quoted verbatim that the verdict turned on. Where a repair was made, the one-line diff is pasted and AC-005's sweep is **re-run** afterwards, since a repair changes the phrase corpus. Authority: `project.md:260-262` (AC-009); `_decomposition.md:350-352` (UX-009); the mirror commitment at `invariant-to-appendcondition-bridge/spec.md:430`. |
| AC-008 | **GIVEN** project DoD item 7 (`project.md:296-297`) asks for *a recorded result* and not for a green feeling, and given that `RUNBOOK.md:920-925` is the recorded cost of a green record standing in for a check that never ran, **WHEN** the walk closes, **THEN** the result exists as a **composed per-page table** — surface id, path, declared token, verdict, instrument, walker identity (a named non-author), date, and routing destination where applicable — written into this story's `_ledger.md` against the AC ids above rather than into prose a later reader cannot audit; **AND** every row that is *vacuous*, *blocked*, *not yet authored* or *indeterminate* is recorded **in those words**; **AND** every in-tree repair is a single line, with anything larger stopped and routed under project DoD item 9 (`project.md:300-302`); **AND** the working tree is clean and `cargo xtask affected --base main` is green at the checkpoint. | *Mechanical.* `git diff main --stat` confined to the PR boundary, showing only single-line hunks in the four surfaces plus at most a `docs/README.md` index row and a `xtask/src/narrative.rs` registration line; `git status --porcelain` → empty; `cargo xtask affected --base main` green, output pasted; `redkiln verify --grain story` passing with `require_ledger` (`.redkiln/config.yaml:67`). *Procedural (ledger).* The result table itself, and a routed-findings list where each entry names its destination (HS-P0020 substrate, HS-P0021 notation/rule, HS-P0023 pointer/reach, HS-P0024 comprehension, `support` for incidental bugs). Nothing absorbed silently. |

**Project-AC coverage.** Project **AC-012** (*exactly one named answered-need per page in
HS-P0021's form, and a reviewer applying HS-P0021's check finds no page carrying two*,
`project.md:269-271`) ← story **AC-001, AC-002, AC-003, AC-004** — the set is enumerated so
the claim is about all of it, the form and cardinality are walked, the coverage is reported
per page instead of averaged, and the check is demonstrated able to fail. Project **AC-009**
(*the DT-1 anchor decision applied identically, each relying page citing the one recorded
place*, `project.md:260-262`) ← story **AC-005, AC-006, AC-007** — named once, linked
everywhere, and not re-argued. **AC-008** is the recorded result that discharges project DoD
item 7 and carries every other criterion's evidence.

## Interaction quality

RFC §6.7/D6. This story renders **no new surface**; it walks four that the signed-off design
already composed, and changes them only at one-line grain. The invariants below therefore bind
in two directions at once: they are what the walk **checks**, and they are what a repair must
not **break**. Every applicable invariant is carried by an `AC-###` **row in the table above** —
this section says only which id carries which and how it is verified, because a bullet here
would get no ledger row and would never be gated.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the reader learns what the page is for on the page they landed on | **AC-002** (the declaration is the first element under the H1, not in an index, not in a sidebar, not on a hub page) | the first-12-lines capture and the fence-position comparison in AC-002. The forbidden shape is a set whose "what is this for" lives one page away — the reason `_design.md:452-455` refuses an index page in front of the opening encounter |
| **One intended context switch, and the reader chooses it** | **AC-006** (the single link to `#where-your-streams-went` is the one hop this design spends, and it is a plain link) | the enumerated link targets and the keyboard-only traverse in AC-006. `_design.md:108-114` — one anchor rather than a repeated paragraph |
| **Non-occlusion** — the declaration is never hidden behind a control | **AC-002**, **AC-003** | `_design.md` `## Transience policy`, the answered-need row (`:520`): *persistent*, never inside a fold, never a tooltip, never a badge in a corner. On the crate root the binding is stronger and already verified: everything this project writes lives inside `details.toggle.top-doc` rendered `open`, and **no content may land inside a `details` that is not** — anti-pattern 3. AC-003's coverage table is the instrument that catches a declaration a machine cannot see |
| **Preserved focus, scroll and selection** | **AC-006**, **AC-008** | discharged by **absence of mechanism** plus one positive property. This story authors no script, no disclosure and no CSS (UX-012), so nothing *can* move focus — AC-008's `git diff --stat` boundary is what keeps it that way. The positive half is AC-006's slug: unnumbered and human-readable, so inserting a section above it does not move where an inbound link lands (IQ-4, `_decomposition.md:269-276`) |
| **Reversibility** | **AC-004** (the injected duplicate is reverted and both instruments return to green), **AC-008** (`git status --porcelain` → empty at the checkpoint) | the four pasted outputs of AC-004's direction (b), and AC-008's residue capture. Reversibility is a state, not a footnote (`_design.md` `## States`, "Boundary restored") |
| **Keyboard reachability** | **AC-006** | the keyboard-only router→anchor traverse, recorded. Every reference this story adds or repairs is a plain markdown or intra-doc link; no widget that could need a pointer exists to reach (IQ-6's answer is *none*) |

### Composition invariants — from the signed-off `_design.md`, binding

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every artifact is real composed presentation in this repository's own textual grammar, not bare markup or a paragraph that happens to contain the words | **AC-002** (the declaration is a blockquote in the exact grammar — token in backticks, em-dash, question ending in `?` — not a sentence saying what the page is about), **AC-003** and **AC-008** (the coverage and result are **tables with named columns**, not prose) | the form checks in AC-002; the three-column and eight-column table shapes required by AC-003 and AC-008. An unstyled render satisfies "a need is stated somewhere" perfectly; only the grammar and position checks fail it |
| **Composition and placement** — the binding region order per surface | **AC-002** (H1 → declaration → prose, nothing interposed, above the first fence, on all four surfaces), **AC-006** (`## Where your streams went` is region 3 of the bridge's fixed order: rule-in-your-words, *then* the prior model, *then* the four-step narration) | AC-002's ordered line capture; AC-006's single-heading check read against `_design.md:487-503` (the bridge's reading order) and `:428-431` (the crate root's position for the declaration, fixed independently of the form) |
| **Transience** — persistent chrome vs revealed vs opened-on-demand | **AC-002**, **AC-003** (the declaration is **persistent**, on every surface), **AC-006** (the anchor is the one *opened-on-demand* control, and it is a link — the mechanism whose accessibility this repository does not have to verify) | `_design.md` `## Transience policy` rows for the answered-need line (`:520`) and for hidden doctest lines; anti-pattern 3. A repair that moved a declaration inside a `<details>` would satisfy every count check and fail here |
| **Density budget, with its real numbers** | **AC-002** (the declaration is **one source line**; no fence is added, so the **68-column** hard fence budget is untouched — and a one-line repair must not push a fence line over it), **AC-006** (the **22-character** `##` sidebar budget, `_design.md:575`, `:693`) | AC-002's line capture, plus `awk 'length > 68' ` over any fence line a repair touches → no output. AC-006's heading measures **23** characters against a 22-character budget: that is **EC-004** — measured, recorded and routed, never silently renamed, because the rename would break every inbound link AC-006 exists to prove resolves |
| **Hierarchy** — carried by structure, never by colour or size | **AC-002** (H1 primary, declaration recessive by *blockquote structure*; "recessive" means read second, never removed), **AC-003** (verdict primary, instrument recessive, carried by column order) | the ordered line capture in AC-002. No `<small>`, `<sub>`, `<sup>` or colour is introduced anywhere — there is no CSS in this project's mandate (UX-012) |
| **Named anti-patterns refused** — 12, 6, 3, 2, 5 | 12 (*two answered-need statements on one page, or one below the first code block*, `_design.md:869-870`) → **AC-002**, and **AC-004** proves the check that catches it can fire; 6 (*the four prior-model phrases outside the one page*, `:852-854`) → **AC-005**; 3 (*a collapsed disclosure triangle in this project's content*) → **AC-002**/**AC-003**; 2 (*a literal bracketed code word — an unresolved intra-doc link*) → **AC-006**; 5 (*a sidebar entry clipped with an ellipsis*) → **AC-006** via EC-004 | the greps and captures named in each AC. Anti-patterns 1, 7, 10, 13, 14 and 15 are **fence-shaped** and belong to the slice-mate (`fence-inventory-and-clause-audit`); they are named here so a reader can see they are covered in the slice, not double-counted in this story |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | A surface in AC-001's enumeration **does not exist** when the walk runs, because a blocking story has not landed. | The row reads **not yet authored**, in those words, and **this story does not complete**. It is not recorded as `pass`, not recorded as vacuous, and not silently dropped from the table — the set the claim is about is fixed by `_design.md:45-65`, not by what happens to be on disk. Escalate to the slice, not to the record. |
| EC-002 | HS-P0021's atoms, walk or checker are **absent** at the time of the walk. | The mechanical column reads **no instrument** against every row and the gap is routed to HS-P0021. Position and cardinality are still binding — they are this project's design (`_design.md:520`, `:900-903`) — and are checked by a person. The **form** half of AC-002 is **blocked**, recorded as blocked, and **no placeholder notation is invented**: a second notation is DR-14's named defect (`project.md:224-226`) and inventing one to unblock a criterion is the worst version of it. |
| EC-003 | `xtask::narrative::TREE` is absent, or `cargo xtask lint-pages` is not mounted in `REQUIRED`. | **Halt loudly** — HS-P0021's own rule for its checker (`page-need-checker-mounted-in-the-gate/spec.md:305-306`) — and record it. Do **not** declare a local `PAGE_DIR`, a second corpus or a substitute checker; that is the *three lists that must agree* defect this initiative already foreclosed. |
| EC-004 | The one recorded location's heading, `## Where your streams went`, measures **23 characters** against `_design.md`'s own **22-character** `##` budget (`:575`, `:693`), whose stated failure is an ellipsis-truncated sidebar entry (anti-pattern 5, finding F-2 at `:996`). | **Record the measurement and route it. Do not rename the heading.** The slug `#where-your-streams-went` is the single citable target AC-006 proves every relying page resolves into; renaming it breaks every inbound link to relieve a one-character overrun. `_design.md` is signed off (`:1039`) and this story may not edit it, and gap 2 (`:895-899`) already notes the 22-character figure is rustdoc's 200px sidebar and moves if HS-P0020's DT-7 pins a different renderer. Destination: `_design.md`'s owner via project DoD item 9, with HS-P0020 as the render-shape input. |
| EC-005 | HS-P0021's declaration form cannot be honoured on `crates/happenstance/src/lib.rs`, because a rustdoc crate-root doc comment has no markdown `# Title` for the form to anchor to. | Record **how the form landed against the position the design already fixed** — immediately below the one-line crate summary and above every `##` (`_design.md:428-431`), which puts it above the first fence by construction. If form and position cannot both hold, **route it to HS-P0021 as a form gap** and leave AC-002's form half blocked for that row. Do **not** write a variant notation, and do **not** move the page out of the crate root: `include_str!` resolves against the file tree at compile time and a path escaping the package would not resolve once published (`crates/happenstance/src/lib.rs:7-9`, `_design.md:707-726`). |
| EC-006 | The calibration walk returns `pass` on the two-declaration specimen, or is skipped because "the procedure is obviously correct". | **AC-004 fails outright.** A procedure that has never returned `fail` is decorative. If the walk genuinely cannot reach `fail — two needs` on a page carrying two declarations, the defect is in the **step that failed to discriminate**, not in the fixture — and the repair is HS-P0021's, routed with both runs recorded, not a step rewritten here. |
| EC-007 | The walk finds a defect **larger than one line** — a page needing a new section, a token that should change, a re-argued passage that cannot be replaced by a link without losing the argument. | **Stop and route** under project DoD item 9 (`project.md:300-302`). Record the finding, its page, its destination and the reason it exceeded one-line grain. This story does not re-author or re-compose a page; that authority belongs to the five blocking stories and to the signed-off design. |
| EC-008 | The only available walker **authored** one of the pages in the set. | The walk is **void for that page**. HS-P0021's procedure fixes the performer as *not the author* and forbids consulting the author or the page's git history (`reviewer-and-citation-procedures/spec.md:491-492`); a self-walk reproduces exactly the author's-memory dependency DoD-8 exists to eliminate (`initiative.md:435-438`). Record the row as **blocked — no non-author walker** and escalate; do not soften the rule. |
| EC-009 | AC-005's phrase sweep and AC-007's judgement **disagree** — the greps are clean but the reviewer finds a page arguing the prior model without using any of the four phrases. | The **reviewer wins**, and the finding is recorded with the sentence quoted. The four phrases are anti-pattern 6's *screenshot-checkable* spelling, not the definition of the defect. A phrase added to the sweep is a proposal to HS-P0022's design owner, recorded in `## Clarifications`, not an edit to `_design.md`. |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| NF-001 | **No `xtask` code, no second corpus, no second checker, no rule atom.** `git diff main -- xtask/src` contains at most registration lines in `xtask/src/narrative.rs`; `git diff main -- standards/` is **empty**; `git diff main -- spec/` is **empty**. | The notation, the walk, the checker and the corpus const are HS-P0021's and HS-P0020's. Duplicating any of them is DR-14's defect and produces the *three lists that must agree* failure one level out (`page-need-checker-mounted-in-the-gate/spec.md:369`). |
| NF-002 | **Every measurement is a pasted command output, never a claim.** A criterion whose evidence reads "verified" is not satisfied. | `.redkiln/config.yaml:67` (`require_ledger`) exists because a green suite proves something works, never that the criteria the story was written to satisfy are the things that work. The precedent for pasting numbers rather than asserting them is the sibling discipline's AC-009 (`reviewer-and-citation-procedures/spec.md:497`). |
| NF-003 | **Reproducible by a stranger.** A second person, given only this spec, the rendered pages and HS-P0021's walk, reaches the same verdict per page. | The walk's own bar: *"reach the same verdict the author would, from the page alone"* (`reviewer-and-citation-procedures/spec.md:244-245`). Divergence on any step is a defect in the step, routed to HS-P0021. |
| NF-004 | **No residue.** After the calibration, `git status --porcelain` is empty and the four surfaces are byte-identical to their pre-injection state apart from deliberate one-line repairs. | The injection is an instrument, not a change. `_design.md` `## States`, "Boundary restored" — reversibility is a state that gets observed, not assumed. |
| NF-005 | **The record is durable and machine-read.** The result lives in `_ledger.md` rows keyed to AC ids, not in a prose summary. | Project DoD item 7 asks for a *recorded* result and `redkiln verify --grain story` reads the ledger. A prose summary is unauditable by the mechanism that exists to audit it. |
| NF-006 | **Zero runtime, dependency, feature or MSRV movement.** No `Cargo.toml` in the workspace is touched; the MSRV floor and `rust-toolchain.toml` are untouched. | This story adds no code. Moving the floor in silence is what ADR-0029 forbids (CLAUDE.md, constraint 5), and there is no reason here to move it at all. |
| NF-007 | **Bounded reviewer cost.** The walk is four pages plus two calibration walks; each page is three steps at one screen (`_design.md` density budget). If the walk cannot be completed in one sitting, the *set* has grown past what a reviewer can hold — which is a finding about the set, recorded, not a reason to sample. | The initiative's non-goal is explicit: *"Volume. Pages written is not the measure"* (`_decomposition.md:31-33`). Sampling a set is how a reviewer's pass becomes the author's memory by another name. |

## Implementation notes (non-prescriptive)

The order below is the one that keeps each step's evidence valid; deviate with a reason
recorded, not silently.

1. **Enumerate first, walk second.** Build AC-001's three lists before opening any page. Doing
   it the other way round makes "did I miss a page?" unanswerable, because the reviewer's
   memory of what they walked becomes the enumeration.
2. **Choose the walker before choosing the pages.** EC-008 is cheapest to discover now. If the
   walker authored the bridge, walk the bridge with someone else or record the row as blocked;
   do not reason that "it's obvious anyway".
3. **Run the mechanical step before the human walk, and paste it either way.** `cargo run
   --locked --quiet -p xtask -- lint-pages` is fast, and its output tells you which of the four
   rows already has a machine behind it. It is *not* a substitute for the walk on those rows —
   the step can only see cardinality and position; it cannot see whether the question the
   declaration asks is the one the page answers, which is `fail — need not answered`.
4. **Calibrate against the inert specimen first, the live page second.** Walking
   `standards/pages/examples/two-needs.md` costs nothing and can never make the gate red
   (`reviewer-and-citation-procedures/spec.md:495`). Only once that returns `fail — two needs`
   is it worth injecting into a live page — and inject into a `docs/` page, not the crate root,
   because the crate root has no mechanical instrument to observe failing (AC-003).
5. **Revert with `git checkout -- <page>` and prove it with `git status --porcelain`.** A
   revert claimed rather than shown is exactly the shape of defect this story is about.
6. **Run AC-005's sweep last among the mechanical steps, and again after any AC-007 repair.**
   A repair that replaces a re-argued paragraph with a link changes the phrase corpus; a sweep
   run before the repair proves nothing about the tree that merges.
7. **Coordinate the `crates/happenstance/src/lib.rs` row with the slice-mate before writing
   it.** Both stories land a coverage claim about that file in the same slice — this one for
   its declaration, `fence-inventory-and-clause-audit` for its fences — and both reach the same
   conclusion for the same structural reason. Two tables in one merge that disagree about
   whether a machine watches that file is a worse outcome than either table being wrong alone.
8. **When you find something larger than a line, stop writing and start routing.** The
   temptation at the end of a slice is to fix it while you are there. EC-007 exists because the
   five blocking stories own their pages and a repair made here has no spec behind it.
9. **Write the ledger rows as you go, not at the end.** Each AC's evidence is a command output
   that is easier to paste when you run it than to reconstruct afterwards, and NF-002 makes an
   unpasted output equivalent to an unrun command.

## Tests and CI (merge gate)

Grounded in the project testing brief's five tiers (`_decomposition.md:475-486`) and its AC→tier
map, which assigns both of this story's project ACs to **tier 5** (`:530`, `:533`). Tier 3 is
stated as `n/a` rather than omitted, per that brief's own auditability rule.

| tier | command / path | proves |
| --- | --- | --- |
| **1. Structural** | `cargo xtask spec-trace`; the `"documentation"` REQUIRED step (`xtask/src/main.rs:290`) — `cargo doc` under `RUSTDOCFLAGS=-D warnings`, which denies `broken_intra_doc_links` | **AC-006**: the link into `#where-your-streams-went` from the crate root *resolves*; an unresolved reference fails the gate rather than rendering as literal brackets (anti-pattern 2). Also guards that a one-line repair did not break a citation the slice-mate is auditing |
| **2. Compiled-fence / page checker** | `cargo run --locked --quiet -p xtask -- lint-pages` — the REQUIRED step **`every page declares one need`**, `probe: None`, corpus `xtask::narrative::TREE` | **AC-002** (mechanical half) and **AC-003**: exactly one declaration, positioned, on the three `docs/` surfaces; and the pasted output is the evidence that the instrument was *run*. **AC-004(b)**: the same step observed **failing** by `path:line` under an injected duplicate, then green after revert |
| **3. Executed** | **n/a — stated, not omitted.** This story adds no Rust code, no doctest and no `#[test]`; there is nothing to execute. The executed tier belongs to `boundary-refusal-encounter` and `boundary-falsification-drill` | recorded so the absence is a decision rather than a hole. Naming a test here that no page needs would be decorative by CLAUDE.md's own rule |
| **4. Human-observed falsification (recorded once)** | The two-direction calibration: inert specimen → `fail — two needs`; injected duplicate → walk fails, step fails, revert, both green, `git status --porcelain` empty | **AC-004**. This is the tier that makes the absence claim demonstrable rather than asserted. Its shape is the project's own DoD-4 drill (`project.md`, DoD item 3) applied to a written procedure instead of to a boundary |
| **5. Design / review sign-off** | The non-author walk of `standards/pages/40-reviewing-a-page.md#the-walk` over the four surfaces, plus AC-007's judgement, recorded in `_ledger.md` with walker identity and date | **AC-001, AC-002** (judgement half)**, AC-005** (the reviewer-performable form of anti-pattern 6)**, AC-007, AC-008**. `design.capture` is absent from `.redkiln/config.yaml`, so this written record is the **only** record these checks will ever have — the same reason `_design.md` itself exists |
| **Story grain (redkiln)** | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | **AC-008**: the tree this story leaves is green at the story grain. It runs the five file-reading lints and `spec-trace` unconditionally, which is exactly why a documentation-only diff is not a free pass here |
| **Integration grain (redkiln)** | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `:48`); `cargo xtask ci --fast` (`integration_scoped`, `:55`) | the project bar this story closes the slice against. This project is `terminal: false`, so `--fast` is the applicable bar and HS-P0025 owns the whole-initiative re-observation (`project.md`, DoD item 5) |
| **Ledger gate** | `redkiln verify --grain story` with `require_ledger: true` (`:67`) | every `AC-###` above is present in `_ledger.md`, `satisfied: true`, and carries non-placeholder evidence. This is the mechanism that stops "the gate is green" from standing in for "the criteria are met" |

## Risks and coupling (PR-scoped)

| Risk | Why it is real here | Mitigation, in this PR |
| --- | --- | --- |
| **The walk becomes a formality.** Four pages, an author-adjacent reviewer, and eight rows of `pass` written in fifteen minutes. | This is the exact failure DoD-8 names — *"a check a reviewer can actually perform rather than one that depends on the author's memory"* (`initiative.md:435-438`). Nothing about a `pass` row is falsifiable on its own. | **AC-004** is the whole mitigation: the instrument must be shown detecting a presence before its report of an absence means anything. **EC-006** makes a skipped calibration a hard failure, and **EC-008** makes a self-walk void. |
| **The crate root's gap is averaged away.** "4/4 pages declare one need" is true and misleading, because one of the four has no machine behind it. | An audit that reads as complete while one quarter of it rests on a single person's reading is the initiative's own subject, reproduced in the artifact meant to close it. | **AC-003**'s middle column, per page, never averaged — and the row's **no** is the reason the column exists. Cross-read against the slice-mate's identical finding about the same file. |
| **A repair grows into an edit.** The reviewer is in the file, the fix is obvious, and one line becomes a paragraph. | The five blocking stories own their pages and their specs; a paragraph written here has no spec behind it and no reviewer after it. | **EC-007** and the `## PR boundary`'s explicit exclusion; **AC-008**'s `git diff --stat` check that every hunk in the four surfaces is a single line. |
| **A blocking story slips and the walk runs on a partial set.** Five stories block this one, and `page-set-assurance` is deliberately last (`_storymap.md:79-84`). | A walk over three of four pages that reports `pass` is a false claim about the set, and the missing page is the one nobody re-checks. | **EC-001**: the row reads *not yet authored* and the story does not complete. The set is fixed by `_design.md:45-65`, not by what is on disk. |
| **HS-P0021 has not landed, and the pressure is to invent the notation.** This story consumes a form, a walk and a checker it does not own. | Inventing any of them is DR-14's named defect and would produce a second notation the whole initiative then has to reconcile (`project.md:224-226`). | **EC-002** and **EC-003**: block and route, never substitute. **NF-001** makes it mechanically visible — `git diff main -- standards/` must be empty. |
| **The slice-mate and this story disagree about the same file.** Both land coverage claims about `crates/happenstance/src/lib.rs` in one merge. | Two tables in one PR contradicting each other about whether a machine watches a file is worse than one wrong table, because a reader cannot tell which to believe. | Implementation note 7: coordinate that row before writing it. Both stories reach the same conclusion for the same structural reason, and the agreement is recorded. |
| **EC-004 gets "fixed" by renaming the heading.** A 23-character heading against a 22-character budget looks like a one-word edit. | The slug is the single citable target the entire anchor half of AC-009 rests on; renaming it silently invalidates every link **AC-006** just proved resolves, and edits a signed-off design by side effect. | **EC-004** states the disposition explicitly: measure, record, route. `_design.md` is signed off (`:1039`) and gap 2 (`:895-899`) already flags the budget as renderer-dependent. |

## Dependencies

**Blocks on** — all five must have landed before the walk can run at all, because the walk's
corpus *is* their output:

| Story slug | What this story cannot do without it |
| --- | --- |
| `tension-resolutions` | `_design.md` itself — the four surface ids AC-001 enumerates, the DT-1 single recorded location AC-005/AC-006 check, the transience and density rows the composition invariants cite. Without it there is no decision to check identical application of. |
| `boundary-refusal-encounter` | `docs/first-encounter.md` — the `opening-encounter` surface, one of the four rows, and the page anti-pattern 6 forbids naming a prior model on. |
| `boundary-falsification-drill` | the drill section under step 3 of that same page; the page is not final until the drill's `###` is on it, and a walk over a page still being written measures nothing. |
| `invariant-to-appendcondition-bridge` | `docs/carry-your-invariant.md` and, critically, its `## Where your streams went` — **the one recorded location** every other criterion in the anchor half points at. AC-005 and AC-006 are unrunnable before it exists. |
| `surface-course-subscriptions` | `docs/read-the-worked-example.md` — the `worked-example-handoff` surface and the fourth row of every table in this spec. |

**Also consumed, and owned outside this project** (not `depends_on` edges in this project's map,
but hard preconditions recorded here so a missing one is diagnosed rather than worked around):
HS-P0021's `need-vocabulary-and-declaration-form` (the notation), `reviewer-and-citation-procedures`
(the walk and the four verdicts), `declaration-check-seen-to-fail` and
`page-need-checker-mounted-in-the-gate` (the mechanical step and its corpus); HS-P0020's
`xtask/src/narrative.rs` and its `TREE`. Their absence is **EC-002** and **EC-003**, not a licence
to substitute.

**Unlocks** — nothing inside this project. `page-set-assurance` is the terminal slice
(`_storymap.md:79-84`), and this story's only downstream consumers are outside it:

- **Project DoD item 7** (`project.md:296-297`) is discharged by **AC-008**, and DoD item 9's
  routing table is fed by every finding this story records rather than fixes.
- **Initiative DoD scenario 8** (`initiative.md:435-438`) gets its `application-author-path`
  half here; HS-P0025 re-observes it across the whole initiative.
- **HS-P0021** receives every notation and procedure gap this walk surfaces (EC-002, EC-004,
  EC-005, EC-006, EC-009) as input to its own rules, which is the only way a written procedure
  learns from being executed.

**Slice-mate, not a dependency**: `fence-inventory-and-clause-audit` (HS-S0189) — same slice,
independent of this story, mounted together over the assembled set (`_storymap.md:82-84`).

## Anchors (progressive disclosure)

Load the `## Context pack` above and you can start. Open these **when** the column says, not
before — each is deeper than the pack on purpose, and pasting any of them in bulk would defeat
the reason they are anchors.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The **signed-off, binding** design. `:45-65` is the authoritative list of the four surfaces AC-001 enumerates; `:108-118` is DT-1's single recorded location and the sentence "*every other page … links to that heading and does not re-argue it*"; `:520` is the answered-need transience row; `:575` and `:693` are the 22-character budget behind EC-004; `:852-874` are anti-patterns 6, 11, 12 and 13; `:900-903` is gap 3 (the form is HS-P0021's, position and cardinality are ours); `:1039` is the sign-off that makes it unamendable here. | Before writing AC-001's enumeration, and again before writing any composition-invariant evidence. Re-open at `:575`/`:693` the moment EC-004 fires. | AC-001, AC-002, AC-005, AC-006 |
| `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` | The table this story checks the four surfaces **against**. `§ Anchor table` is the one place every rendered heading, its character count, its emitted fragment id and whether the 22-character budget binds its surface were decided — ids read off a render, never predicted — including the per-surface scoping that settles EC-004 (`Where your streams went` at 23 needs no retitle, because the budget's mechanism is rustdoc's 200px sidebar and the bridge is markdown) and the consequence that anti-pattern 5 cannot fire on a markdown surface. `§ Consumption map` is the list of stories that must not re-derive the DT-1 anchor, which is AC-007's judgement made checkable, and `§ DT-1` is the decision itself. | Before the first walk, and open beside the rendered page for every AC-005 and AC-006 row. A rendered heading that disagrees with that table is the finding — the table is not re-decidable here. | AC-005, AC-006, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/reviewer-and-citation-procedures/spec.md` | The **procedure this story executes and must not reinvent**: the non-author performer and permitted/forbidden sources (`:491-492`), the four-row verdict table and `indeterminate` as a defect in the page (`:493`), the vacuous-never-pass rule (`:494`), and the inert two-declaration fixture used for calibration (`:495`). | Immediately before the first walk — read the performer paragraph and the verdict table, then execute. Re-open before recording any row that is not `pass`. | AC-002, AC-004, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/need-vocabulary-and-declaration-form/spec.md` | The **exact notation** — the closed four-token `NEEDS` set (`:71-82`) and the declaration line's grammar and position rule, including the nothing-interposed clause (`:92-107`). Checking a form from memory is how a second notation gets invented. | Before running AC-002's form checks on the first page, and again if any page's token looks like it might be outside the set. | AC-002 |
| `.bklg/docs-that-teach/page-need-discipline/page-need-checker-mounted-in-the-gate/spec.md` | The **mechanical half's contract**: the step name, that it reads `xtask::narrative::TREE` and declares no `PAGE_DIR` of its own (`:305`), the halt-loudly rule when the const is absent (`:305-306`), and the *three lists that must agree* defect it forecloses (`:369`). | Before running `lint-pages`, and immediately if the step is missing or its corpus is not what you expected (EC-003). | AC-003, AC-004 |
| `.bklg/docs-that-teach/application-author-path/project.md` | The two project ACs this story traces to — **AC-009** (`:260-262`) and **AC-012** (`:269-271`) — DoD item 7's exact wording (`:296-297`), DoD item 9's routing table (`:300-302`), DR-14 (`:224-226`) and the Risks row BR-07 rests on (`:336`). | Before writing AC-007 (the re-argument judgement) and before routing any finding under EC-007. | AC-007, AC-008 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The briefs. IQ-8 (`:299-303`) and UX-008 (`:348-349`) are position and cardinality; UX-009 (`:350-352`) is the cite-don't-re-argue obligation; IQ-4 (`:269-276`) is the stable-slug rule AC-006 rests on; the testing brief's five tiers (`:475-486`) and the AC→tier rows for AC-009 and AC-012 (`:530`, `:533`); Persona 1's fear of silent wrongness (`:28-30`). | When writing the verification column of any AC, and before filling the `## Tests and CI` table's "proves" cells. | AC-002, AC-006, AC-007 |
| `.bklg/docs-that-teach/application-author-path/invariant-to-appendcondition-bridge/spec.md` | The blocking story that **ships the one recorded location**. `:372` and `:460` are the heading and its slug; `:430` is the mirror commitment that the four prior-model phrases appear on that page *and nowhere else in this project's output*; `:374` is the page's declared token. AC-005 and AC-006 are checking a promise made there. | Before running AC-005's sweep and AC-006's link check — so you are verifying the commitment that was actually made, not one you inferred. | AC-005, AC-006 |
| `.bklg/docs-that-teach/application-author-path/fence-inventory-and-clause-audit/spec.md` | The **slice-mate**, which lands a coverage claim about `crates/happenstance/src/lib.rs` in the same merge. Its `## Executive summary` item 2 reaches the same structural conclusion about that file for fences that AC-003 reaches for declarations. | Before writing AC-003's `crate-root-encounter` row — implementation note 7. Two tables in one PR must not disagree about one file. | AC-003 |
| `.bklg/docs-that-teach/initiative.md` | The gold source. **AC-08** (`:383-385`) — *"once, consistently, rather than differently on each page"* — is the intent AC-005/AC-006/AC-007 decompose; **DoD scenario 8** (`:435-438`) is the sentence AC-004 exists to make true. | Once, before writing the acceptance evidence, to check that what you are about to record answers the initiative's question and not a narrower one. | AC-004, AC-005 |
| `xtask/src/main.rs` | The real `REQUIRED` array: `"documentation"` at `:290` is the `RUSTDOCFLAGS=-D warnings` step AC-006's resolution proof rides; `"tests"` at `:143-155`; `"specification traceability"` at `:315`. This is where the gate is defined once, and where the `every page declares one need` step will be mounted. | When assembling AC-006's evidence, and to confirm the page step is actually in `REQUIRED` rather than merely written. | AC-003, AC-006 |
| `crates/happenstance/src/lib.rs` | The `crate-root-encounter` surface itself, and `:7-9`'s `include_str!` constraint — the compile-time reason the page cannot be relocated into the governed tree, which is what makes EC-005 a routing decision rather than a move. | Before writing AC-003's coverage row for it and before considering any remedy for its unwatched declaration. | AC-003 |
| `.redkiln/config.yaml` | The commands redkiln runs at this story's and this project's grains whether or not anyone types them: `affected_gate` (`:40`), `reachability_static` (`:48`), `integration_scoped` (`:55`), `require_ledger` (`:67`) — the last being why AC-008's record must be ledger rows, not prose. | Before the checkpoint commit, and when writing `_ledger.md`. | AC-008 |
| `CLAUDE.md` | The corollary this story borrows wholesale — *"A rule that no adapter can fail is decorative. Before adding one, name a plausible wrong implementation it rejects"* — applied to a written procedure instead of a conformance rule. It is the entire justification for AC-004 existing. | Before deciding whether the calibration is worth the effort. It is; this is why. | AC-004 |
| `RUNBOOK.md` | `:920-925` is this repository's own recorded cost of a green record standing in for a check that never ran. It is the concrete precedent behind EC-001 and NF-002. | When tempted to write `pass` on a row whose evidence you did not actually gather. | AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the front half enumerated** — AC-001 … AC-008. None added,
   none dropped. AC-001–AC-004 carry project AC-012; AC-005–AC-007 carry project AC-009; AC-008
   is the recorded result that discharges project DoD item 7 and holds the others' evidence.
2. **Four verdicts, not three, and no soft pass.** `indeterminate` is a *defect in the page*,
   not a neutral outcome (`reviewer-and-citation-procedures/spec.md:493`). A page at
   `indeterminate` blocks this story exactly as a `fail` does; the story does not invent a
   fourth disposition to let itself complete.
3. **The calibration injects into a `docs/` page, not the crate root.** Both are walked, but
   only a page inside `TREE` has a mechanical instrument that can be observed failing, and
   direction (b) of AC-004 is about observing *both* instruments. Injecting into the crate root
   would exercise only the human half and prove less.
4. **EC-004 is a genuine finding surfaced while writing this spec, and it is deliberately not
   fixed here.** `## Where your streams went` measures 23 characters against `_design.md`'s own
   22-character `##` budget (`:575`, `:693`), while `:693` also asserts *"Every heading this
   project ships is inside the budget"*. The two cannot both be true. The disposition is
   **record and route**, because the slug is the single citable target AC-006 proves every
   relying page resolves into, `_design.md` is signed off (`:1039`), and gap 2 (`:895-899`)
   already notes the budget is rustdoc-specific and moves if HS-P0020 pins a different renderer.
   Renaming the heading to relieve one character would break every inbound link this story
   exists to verify.
5. **Anti-patterns are split with the slice-mate by mechanism, not by page.** 12, 6, 3, 2 and 5
   are this story's; 1, 7, 10, 13, 14 and 15 are fence-shaped and are
   `fence-inventory-and-clause-audit`'s. Named in both specs so the slice's coverage is legible,
   counted in one.
6. **AC-005's phrase list is anti-pattern 6's spelling, not the definition of the defect.**
   EC-009 settles the disagreement in the reviewer's favour and routes any proposed phrase
   addition to `_design.md`'s owner rather than editing a signed-off file.
7. **`docs/README.md` is in the PR boundary for reading and for at most one index row.** Every
   pointer-out policy is HS-P0023's (DT-10); this story does not add to `docs/README.md:12-24`'s
   pointer table, and a reachability gap found during enumeration is routed there.
8. **Tier 3 is recorded as `n/a` rather than omitted.** The testing brief's own auditability
   rule; naming a test this story does not need would be decorative by CLAUDE.md's test.
9. **The mount point is `xtask/src/narrative.rs` even though this story adds no code**, because
   membership in the set this story makes a claim about is decided there. If HS-P0020 pinned a
   different filename, use the file it pinned and record the deviation — never stand up a
   parallel harness (`_decomposition.md:118-131`).
