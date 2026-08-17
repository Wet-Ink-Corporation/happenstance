---
item: HS-S0149
stage: spec
created: 2026-08-17T13:16:09.215Z
updated: 2026-08-17T13:16:09.215Z
template_sig: 87bbf1d0
rendered_sig: "929645e6"
---

# Spec — The cite-never-restate rule, the non-author verdict walk, and the paraphrase spot check

## Scope lock

| Level | Path and the part that binds this story |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-09 (`:344`, "a page cites clauses and never restates them"), AC-12 (`:395-397`), **DoD-8** (`:441-444`, the singular-need walk "a reviewer can actually perform rather than one that depends on the author's memory") and **DoD-12** (`:455-457`, "no page has become a second specification") |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the BR-09 split: this project owns the *rule* and the reviewer spot check, HS-P0020 owns the mechanical clause-id resolution |
| Project | `.bklg/docs-that-teach/page-need-discipline/project.md` — **AC-009**, **AC-010**; DR-07, DR-09; Risks row 4 (the discipline becoming a second specification) |
| Briefs (key) | `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` — Architecture brief AC-009 (`:116-119`), AC-010 (`:120-129`), Note 4 (no `rust` fence), Note 6 (no shared abstraction), Note 7 items 1 and 4; UX brief **UX-009** (`:526-532`), **UX-010** (`:533-541`), UX-003, UX-011; Testing brief AC-009 (`:913-923`), AC-010 (`:924-936`) and the fifth tier, *procedural (ledger-recorded)* (`:785-791`) |
| Signed-off design (**BINDING**) | `.bklg/docs-that-teach/page-need-discipline/_design.md` — `## Surfaces` (`rule-atom`, `reviewer-procedure`), `### S5 — reviewer-procedure`, `## Composition` S3 and S5, `## Transience policy` rows S3/S5, `## Density budget`, `## States` row S5, `## Anti-patterns` 13-15, `## Mock` finding 3 |
| Story map | `.bklg/docs-that-teach/page-need-discipline/_storymap.md` — slice `discipline-on-disk`, this story's row (`:48`); Coverage rows for AC-009 and AC-010 (`:116-117`) |
| Dependency's spec (band namespace) | `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` — the band table fixing bands `30` and `40` to this story, and the byte-exact generated-index row contract |
| Grounding | `.bklg/docs-that-teach/page-need-discipline/_grounding.md` — "Clause-id stability (DR-09's textual basis)" (`:240-245`); no Accepted decision atom governs this project (`:291-296`) |
| Roadmap pointer | **None.** `RUNBOOK.md` carries no phase for this initiative. The roadmap of record is the story map's merge order (`_storymap.md`, "Merge order"). |
| Design mock (built, awaiting sign-off) | `.bklg/docs-that-teach/page-need-discipline/design/mock.html` — the `reviewer-procedure` frames and its four-row verdict table |

## One-line PR slice

Write the cite-never-restate rule and the two procedures a byte count cannot
replace — the non-author verdict walk (DR-07) and the paraphrase spot check — and
run each once over the set as it stands, recorded in `_ledger.md`.

## Executive summary

**What this PR lands.** Two rule atoms in the tree the dependency story stood up —
`standards/pages/30-citing-the-specification.md` (a page cites a clause id and never
restates the clause) and `standards/pages/40-reviewing-a-page.md` (the non-author
verdict walk, and the paraphrase spot check) — their rows in the router's filter and
its generated index, one linked fixture page that the walk is calibrated against, and
the recorded runs of both procedures.

**Pointer, not restatement.** The tree, the router, the rank and the announcement are
the dependency's (`router-precedence-and-announcement`); the need set and the
declaration form are `need-vocabulary-and-declaration-form`'s; the fold line is
`fold-line-rule`'s. This story lands **bands `30` and `40` and nothing else in the
tree** — the two rules that exist because a lint cannot reach them, plus the evidence
that each has actually been executed by a person rather than merely written down.

**The delta worth reading twice.** Four decisions in this PR are load-bearing beyond
it, and each is stated as a decision in the Context pack rather than left open:

1. **The procedures must be able to return `fail`.** No governed page exists in this
   worktree, so a walk over "the set" would be vacuous and a vacuous walk proves
   nothing. This PR authors **one fixture page carrying two needs**, outside the
   governed tree and outside the atom namespace, and the non-author walk is run
   against it to a recorded `fail — two needs`.
2. **The vacuous half is recorded as vacuous, not as green.** `_design.md`'s `## States`
   row S5 fixes that behaviour, and the ledger must not let a vacuous sweep stand in
   for the real one — the `RUNBOOK.md:920-925` lesson applied to a procedure.
3. **The spot check's first real corpus is this discipline's own tree.** The rules
   themselves make normative claims and cite clause ids, and "the discipline becomes a
   second specification" is the project's own Risks row 4. Running the rule against
   the tree that states it is the cheapest available falsification.
4. **`_design.md` specifies the verdict table twice and differently** (three rows in
   `## Composition`, four states in `## States`); its `## Mock` finding 3 records the
   discrepancy and its mock draws four. This story implements **four**, and says so.

## Context pack

The decisions this story must honor, stated inline. Everything deeper is a signposted
anchor in the second half of this spec — link, do not paste.

**Two bands, two atoms, and the file names are pinned here.** The dependency's spec
fixed the band namespace: band `30` is *citations — a page cites a clause id and never
restates it*; band `40` is *reviewing a page — the non-author walk and the paraphrase
spot check*. `_design.md` pins band `40`'s file by addressing it directly
(`standards/pages/40-reviewing-a-page.md#the-walk`, `## RP-40-1`); it names **no** file
for band `30`, so this story pins **`standards/pages/30-citing-the-specification.md`**
and it is depended on by value the moment the router's index carries a link to it.
Folding the two into one atom was rejected in the dependency's own band table and the
reason holds: the citation rule is applied while *writing a sentence*, the walk while
*judging a finished page*, and an atom that answers two moments is the failure this
whole project exists to name.

**The `#the-walk` anchor has to resolve, so the walk sits under its own heading.**
`_design.md`'s `## Surfaces` addresses the procedure at
`standards/pages/40-reviewing-a-page.md#the-walk`. Only a heading generates that
anchor, so the ordered walk opens with a `### The walk` heading inside `## RP-40-1.`'s
**Do** section. This is safe against the parser the checker will copy: `rules()` splits
a rule at the next line beginning `"## "` (`xtask/src/lint_constitution.rs:262,275-279`,
proven by `rules_are_split_at_the_next_heading` at `:870-877`), and `### The walk` does
not match that prefix, so the sub-heading stays inside the rule body. An unresolving
anchor would make the signed-off design's own addressing manifest a dead link.

**The citation rule is this repository's existing rule, applied one level out — cite
it, do not re-derive it.** `standards/rust/README.md:32-36` already holds the
constitution to the same test: *could a conformant adapter written in another language
violate this sentence?* Yes → it is a clause and the atom cites it; and *"An atom never
restates a clause's content."* Band `30` states that test for narrative pages and adds
the half UX-010 owns: the clause id is the **visible link text**, so a reader can see
they are being handed to `spec/SPECIFICATION.md` rather than to a paraphrase of it. The
rule can require a *name* rather than a line reference because clause ids are stable and
are never renumbered (`spec/SPECIFICATION.md:280`) — which is also why a citation
survives the sibling branch's divergence, where a line reference would not
(`_grounding.md:240-245`). **Rejected:** a rule that says a page should *prefer*
citations. It is unfalsifiable, it gives the spot check nothing to return `fail` on, and
"use good judgment" is the non-answer this project's own DT-8 resolution refuses by name.

**The mechanical half is not ours and must not be rebuilt.** Whether a cited id
actually resolves is HS-P0020's check, exposed as `pub(crate) fn clause_ids(root: &Path)`
beside `spec_trace::all_rules` (`xtask/src/spec_trace.rs:1746`, confirmed at that line);
architecture brief AC-010 says **do not build a second one**, and Note 6 says the two
checkers are not refactored together (RS-81-3, `standards/rust/81-checks-that-cannot-be-types.md:209`).
This story therefore touches no `xtask/src/**` at all. The seam is not a gap — it is the
exact shape `lint_constitution`'s own module docs already declare one level down: *"It
does not check that a citation's **claim** is true, only that the cited file has that
line and that the anchor text is near it"* (`xtask/src/lint_constitution.rs:20-21`). A
page can cite a real, resolving clause id and restate its content in the paragraph
underneath, and **no byte count sees it** (architecture brief, Note 7 item 4). That
sentence is the reason the paraphrase spot check exists at all, and band `30` must carry
it rather than imply it.

**The walk is executable by a stranger or it is not the deliverable.** `_design.md`'s
`### S5` fixes the form and this story implements it without re-deciding it: a numbered
list a stranger executes top to bottom **from the rendered page alone**, each step a
question with a yes/no answer and a stated consequence, closing in a verdict table. Who
performs it is part of the rule — *not the author* — and what they may consult is closed:
the rendered page, the router, `spec/SPECIFICATION.md`. **Not** the author, and **not**
the page's git history, because DoD-8's whole subject is a check that does not depend on
the author's memory (`initiative.md:441-444`). No step may contain "consider", "use
judgement", "as appropriate" or "if it seems" — `_design.md` anti-pattern 15, and the
`Do`/`Not`/`Rejects.` grammar (`standards/rust/README.md:99-112`) is what forces a named
wrong state instead of an impression. **Rejected, all three from `_design.md`:** a
checklist of adjectives (unfalsifiable and author-flattering); a quiz (it measures recall
of the prose that wrote it, and an author-authored comprehension check is ruled out as
evidence at `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md:430-433`);
delegating to the lint (the lint cannot see the thing this procedure exists for).

**Four verdicts, not three, and the discrepancy is the design's own recorded finding.**
`_design.md` `## Composition` S5 says a three-row verdict table; its `## States` row S5
carries four (`verdict-pass`, `verdict-fail-two-needs`, `verdict-fail-unstated`,
`indeterminate`); `## Mock` finding 3 records that they disagree and states the mock
drew four *because the States block is its addressing manifest*. This story implements
**four**: `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate`.
`indeterminate` is not a soft pass and carries its own rule — the walk could not be
completed from the page alone, which `_design.md` `## States` fixes as **a defect in the
page, not in the procedure**. This is a gap the sign-off named, not a re-decision.

**A procedure that has never returned `fail` is decorative, so this PR ships the page it
fails on.** UX-009's test is literal: *"a person who did not write the page runs it over
a page carrying two needs and reaches the same verdict as the author would"*
(`_decomposition.md:526-532`). There is no such page in this worktree — `docs/` is
deliberately near-empty (`docs/README.md:1-6`) and HS-P0020's narrative tree is not
merged here. So this story authors **`standards/pages/examples/two-needs.md`**, a
specimen page carrying two declarations, and links it from `## RP-40-1` as the worked
example. `_design.md`'s `## Transience policy` requires exactly that shape — *"a link to
a fixture, not an inline fold"*, and the reason given there is that a link is the one
opened-on-demand mechanism this repository does not have to verify (UX-011). Three
properties of that fixture are decisions, not incidentals:

- **It is not a governed page.** It lives under `standards/pages/`, never under the
  narrative tree the checker will walk. A permanently broken page inside the checked
  surface would make the gate red forever; breaking a *real* page happens once, with a
  revert, and it is `declaration-check-seen-to-fail`'s story, not this one.
- **It is not an atom.** The corpus reader the checker copies is non-recursive and takes
  top-level `.md` entries only (`xtask/src/lint_constitution.rs:208-220`), so a file
  under `examples/` is invisible to it and needs no exclusion list — provided the
  checker story does not make that walk recursive. That is stated as a forward
  obligation under "Data and migrations", the same way the dependency stated its
  generated-region contract one slice ahead of the generator.
- **It is the `two-declarations` state of a surface someone else owns.** `_design.md`
  `## Surfaces` lists that state under `page-need-declaration`; this story renders a
  specimen of it, and does not re-decide the declaration form, whose shape is
  `need-vocabulary-and-declaration-form`'s.

**The set as it stands is empty, and the ledger has to say that in those words.** Both
procedures are run twice and the two runs are recorded differently, because they prove
different things:

- **The governed-set walk is vacuous.** There are no governed pages at this merge.
  `_design.md` `## States` row S5 fixes the behaviour: *"no pages to review → the
  procedure states the walk is vacuous and the reviewer records that, rather than
  recording a pass."* The procedure text itself must carry that instruction, so the
  first person to run it on an empty set cannot record green by accident.
- **The calibration run is where the verdict comes from.** A non-author runs the walk
  against the fixture and records `fail — two needs`, with the walker's identity (not
  the author) and the verdict both in `_ledger.md` — the testing brief's *procedural
  (ledger-recorded)* tier, which exists because two of this project's ACs are judgements
  no function signature can carry (`_decomposition.md:785-791,913-923`).
- **The spot check's real corpus at this merge is this discipline's own tree.** Bands
  `00`-`40` are prose that makes normative claims and cites clause ids, and *"the
  discipline becomes a second specification, or grows `MUST`s that belong in a decision
  atom"* is the project's own Risks row 4. Running band `30`'s rule over
  `standards/pages/**` is therefore not a stand-in for the missing corpus — it is the
  first place the rule was always going to have to hold, and the cheapest available
  falsification of the risk the charter names. The result is recorded as what it is: a
  spot check over the rules tree, **not** over the narrative set, which HS-P0025
  re-observes on the assembled tree (project `project.md`, Out of scope).

**Both procedures state their own blind spots, first.** RS-81-1 requires a check's
limits to be documented in its own documentation because *"a check whose limits are
undocumented is read as a guarantee"* (`standards/rust/81-checks-that-cannot-be-types.md:11`,
worked at `xtask/src/lint_constitution.rs:10-28`). The same failure applies to a written
procedure, and harder — nothing re-runs it. Three limits, stated in the atoms
themselves: the walk yields a verdict on the **page's shape** and never a claim about
reader understanding (that is HS-P0024's friction log and comprehension session); the
spot check does **not** resolve clause ids (HS-P0020's `clause_ids` does, and this
project builds no second parser); and **neither procedure is a gate step**, so nothing
re-runs either one — which is why the ledger records who ran it and when, rather than a
green CLI line standing in for it.

**Mounting: an atom that lands without its router row is half-mounted.** The router's
`<!-- BEGIN GENERATED -->` … `<!-- END GENERATED -->` region is the composition root for
this tree, and at this merge it is still authored by hand — the generator arrives with
`page-need-checker-mounted-in-the-gate`. The dependency's spec fixed the row format
byte-exactly, and this story must produce rows identical to what
`generated_region`'s analogue will emit (`xtask/src/lint_constitution.rs:400-420`): a
markdown link to the atom file, the atom's first `Load when` source line, and its
comma-separated rule ids. Two mechanical consequences: each atom's `> **Load when:**`
occupies **one source line**, because `load_when` reads only the first line of the block
and silently drops continuations (`:247-255`); and every `.md` link this story adds must
resolve, because the link half of `check_router` is copied verbatim next slice
(`:343-356`) and it is what discharges half of AC-011's reachability. The filter is
mounted too: each atom takes a `## Start here` row, inside the router's ≤ 12-row ceiling
(`_design.md` `## Density budget`). *"A rule atom that lands without its router row lands
half-mounted"* is the story map's own first "Why the slices fall here" bullet, and it is
the failure this paragraph exists to foreclose.

**The atoms are built from the grammar the checker will enforce.** `# NN — Title` ·
`> **Load when:**` · `> **See also:**` · `---` · then `## RP-NN-N. <imperative sentence>`
per rule, each with the five fixed sections in order — **Why.** · **Do** · **Not** ·
**Rejects.** · **Evidence.** (`standards/rust/00-prime-directives.md:1-9,19,30,54,78`,
enforced by `check_shape` at `xtask/src/lint_constitution.rs:477-509`). Ceilings, all
inherited on purpose so two trees never teach two numbers for the same idea: ≤ **6**
rules per atom, ≤ **16,384** bytes per atom (`:88,95`), ≤ **96** columns for prose lines
outside tables. Two deliberate divergences, both from architecture brief Note 4: fences
are tagged `text` or `markdown` — a `rust`-tagged fence is rejected because nothing in
the workspace compiles this tree (`xtask/src/lib.rs:28` registers only `mod constitution`),
and an untagged fence is rejected too so a later decision to register the tree cannot be
undermined retroactively. One cheap forward hedge: `MIN_REJECTS_CHARS` is **120**
(`xtask/src/lint_constitution.rs:82`) and the checker may inherit it; writing each
`Rejects.` at or above that length costs nothing and forecloses a retroactive failure.

**The persona-journey slice this realizes.** The reader here is **S4, the non-author
reviewer** — *"reach the same verdict the author would, from the page alone"*
(`_decomposition.md:426`) — and the walk is the only instrument they have, because the
lint they might otherwise trust reports on declaration, never on answering
(architecture brief Note 7 item 1). Behind them stands the application author, whose
named fear is *silent wrongness*: a page that quietly answers two needs reads as
complete and is not, which is the same defect one level up (`_decomposition.md:443-446`).
And the discipline's own credibility rides on the seam being real rather than announced:
this repository's one measured documentation defect is an explanation that existed in
three contributor-facing files, *"none of them `crates/happenstance-core/src/store.rs`,
the file the reader is looking at at the moment they need it"*
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`). A
procedure nobody has executed is that defect wearing a verdict table.

**What this story is forbidden from doing, stated as decisions.** No `xtask/src/**` —
not the checker, not a constant, not a test; the whole gate-step surface is
`page-need-checker-mounted-in-the-gate`'s. No second clause-id parser (architecture
brief AC-010). No `standards/rust/**` edit — AC-002 is discharged architecturally by
never opening that tree. No `.kb/**` — the playbook atom is staged by
`playbook-atom-staged-for-ingest`. No page under HS-P0020's narrative tree, and no
`docs/README.md` edit — the announcement was the dependency's. No `<details>`, tab or
accordion anywhere, including on the fixture, while `PERMITTED_FOLD_MECHANISMS` ships
empty (`_design.md` anti-pattern 6). No quiz or authored comprehension check presented as
evidence. And no restatement of a `spec/SPECIFICATION.md` clause anywhere in this diff —
the rule this story writes would be the first thing to fail it.

## Integration contract

**Slice / milestone.** `discipline-on-disk`. Slice-mates, implemented in one context and
mounted as one surface: `need-vocabulary-and-declaration-form`,
`router-precedence-and-announcement` (**this story's dependency**), `fold-line-rule`.

**Archetype.** `capability` — a user-observable slice. The observable user is a reviewer
who did not write the page, arriving at the router and leaving with a verdict.

**Mount point.** **`standards/pages/README.md`** — the router the dependency story
stands up, and the only composition root this tree has. Two regions, both required:

- the generated index between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->` —
  **two new rows**, one per atom, byte-identical to what `generated_region`'s analogue
  will emit next slice (`xtask/src/lint_constitution.rs:400-420`);
- the `## Start here` filter table — a row per atom, inside the ≤ 12-row ceiling.

An atom in the directory with no row in either region is reachable only by someone who
already knows the file name, which is the half-mount `_storymap.md` names in its first
"Why the slices fall here" bullet.

**Wires into** (real siblings, by path):

| Consumed | Path | What this story takes from it |
| --- | --- | --- |
| the router and its two mount regions (**dependency**) | `standards/pages/README.md` — authored by `router-precedence-and-announcement` | the band table that reserves `30` and `40`, the filter table, and the generated region's exact row shape |
| the atom grammar | `standards/rust/00-prime-directives.md:1-9,19,30,54,78` | `# NN — Title`, `> **Load when:**`, `> **See also:**`, `---`, `## RP-NN-N.` and the five fixed sections |
| the shape enforcement it will be measured by | `xtask/src/lint_constitution.rs:477-509` (`check_shape`), `:82,88,95` (the ceilings) | the section order, the rules-per-atom and bytes-per-atom ceilings, and the `Rejects.` length floor worth clearing pre-emptively |
| the generated-row and link contracts | `xtask/src/lint_constitution.rs:400-420`, `:343-356`, `:247-255` | the row format, the link-resolution obligation, and why `Load when:` is one source line |
| the rule the constitution already holds itself to | `standards/rust/README.md:32-36` | the clause-or-atom test, and *"An atom never restates a clause's content"* |
| the normative voice, and its stability guarantee | `spec/SPECIFICATION.md:280` | clause ids are stable names and are never renumbered — why a citation is an id, never a line |
| the mechanical half, **not rebuilt** | `xtask/src/spec_trace.rs:1746` (`all_rules`), and HS-P0020's `clause_ids` sibling | the seam: resolution is theirs, paraphrase is ours |
| the blind-spot obligation | `standards/rust/81-checks-that-cannot-be-types.md:11`; `xtask/src/lint_constitution.rs:10-28,20-21` | that a limit is stated first, and the precedent sentence that a citation's *claim* is unchecked |

**Design-system primitives consumed.** Textual, all from `_design.md` and all already
enforced somewhere: the atom head (`# NN — Title`, `> **Load when:**`, `> **See also:**`,
`---`); the `## RP-NN-N. <imperative sentence>` rule heading with **Why.** · **Do** ·
**Not** · **Rejects.** · **Evidence.**; the router's generated-region row and its
`You are… / Load` filter row; a markdown table for the verdicts; a numbered list for the
walk; a plain relative link for the worked example. No new primitive is invented, and no
widget is added on top of what the renderer already gives (`_design.md` anti-pattern 9).

**Renders surfaces** (ids from `_design.md` `## Surfaces`):

| Surface | What this story does with it | States it is accountable for |
| --- | --- | --- |
| `rule-atom` | **creates two instances** — `standards/pages/30-citing-the-specification.md` and `standards/pages/40-reviewing-a-page.md` | `populated`; and refusing `over-rule-ceiling`, `over-byte-ceiling`, `missing-section`, `rust-tagged-fence` |
| `reviewer-procedure` | **creates it** — `standards/pages/40-reviewing-a-page.md#the-walk`, the ordered walk under `## RP-40-1` plus the verdict table that closes it | `verdict-pass`, `verdict-fail-two-needs`, `verdict-fail-unstated`, `indeterminate` |
| `discipline-router` | **changes it, does not re-decide it** — two generated index rows and the matching `## Start here` rows | `populated`; must not create `generated-region-stale` or `dangling-link` |
| `page-need-declaration` | **one specimen only**, `standards/pages/examples/two-needs.md`, deliberately in its `two-declarations` state and deliberately outside the governed tree | `two-declarations` |

**Conformance rule(s).** **None, and this is not adapter-observable.** No port, no crate,
no feature is in this diff; `happenstance-testkit`'s suite observes stores, and a
markdown rules tree is invisible to it. The instruments that will observe this story are
`xtask`-side and arrive next slice — the checker's `check_shape` and `check_router`
analogues over `standards/pages/` — plus the two *procedural (ledger-recorded)* runs this
story performs itself, which the testing brief names as first-class proof precisely
because AC-009 and half of AC-010 are judgements no `#[test]` carries
(`_decomposition.md:785-791,913-936`). A story that changes a port and names no rule is
a port change nothing can fail; this story changes no port.

**Clause(s).** None discharged, none amended, none restated — and here that is the
subject rather than a formality. This story writes the rule that *forbids* restating a
clause, so a paraphrase inside its own atoms is the sharpest possible self-refutation;
band `30`'s spot check is run over this tree for exactly that reason. `cargo xtask
spec-trace` remains the only writer of `spec/SPECIFICATION.md`'s generated sections
(architecture brief Note 8) and must be green at merge.

**Advances DoD scenario.** Two, each moved to *the instrument exists and has been run*
rather than to green:

- **DoD-8** (`.bklg/docs-that-teach/initiative.md:441-444`) — *"a review pass over the
  set finds no page carrying two [needs] — with the check being one a reviewer can
  actually perform rather than one that depends on the author's memory."* This story
  lands the performable check and proves it discriminates; the pass over the **real**
  set is re-observed by HS-P0025 once pages exist.
- **DoD-12** (`:455-457`) — *"a spot check confirms the page defers to the clause rather
  than restating it."* This story lands the rule and the spot check and runs it over the
  corpus that exists; the resolving half is HS-P0020's and the re-observation is
  HS-P0025's.

## PR boundary

**In this PR**

- `standards/pages/30-citing-the-specification.md` — the citation rule, its visible-link
  half (UX-010), and the statement that resolution is checked elsewhere and paraphrase
  by nothing.
- `standards/pages/40-reviewing-a-page.md` — `## RP-40-1` (who performs it, the
  `### The walk` ordered steps, the four-row verdict table) and `## RP-40-2` (the
  paraphrase spot check), each with the five fixed sections.
- `standards/pages/examples/two-needs.md` — the worked example the walk is calibrated
  against, linked from `## RP-40-1`, outside the governed tree and outside the atom
  namespace.
- `standards/pages/README.md` — **two generated index rows and the matching
  `## Start here` rows, and nothing else in that file.** Its region order, its
  precedence block, its scope paragraph and its `## What checks this tree, and what does
  not` section are the dependency's and are not re-opened.
- `.bklg/docs-that-teach/page-need-discipline/reviewer-and-citation-procedures/**` — this
  spec, the story's stage artifacts, and the `_ledger.md` carrying both recorded runs.

The implementer **may** also touch the composition-root file named in the Integration
contract to mount this slice; that is not scope drift. Here the composition root *is*
`standards/pages/README.md`, and it is already in the list above with its edit scoped to
the two mount regions.

**Explicitly not in this PR**

- **`xtask/src/**` — nothing.** No checker, no constant, no test, no `INERT` entry. The
  whole gate-step surface is `page-need-checker-mounted-in-the-gate`'s (architecture
  brief Note 1, CR-1 through CR-4), and a partial edit here is the half-mount that story
  exists to avoid.
- **A second clause-id parser** — HS-P0020 owns resolution via `clause_ids`
  (`xtask/src/spec_trace.rs:1746` for the sibling shape); architecture brief AC-010
  forbids building another.
- **`standards/rust/**` — nothing**, so `git diff main -- standards/rust/README.md` stays
  empty and AC-002 is discharged by not touching the file.
- **`docs/README.md` — nothing.** The announcement, the table row and the
  `[PROVISIONAL — settles at …]` marker are the dependency's.
- **Bands `00`, `10`, `20`** — the need set, the declaration form and the fold line
  belong to the two other slice-mates. This story cites them; it does not write them.
- **Any page in HS-P0020's narrative tree**, and any link from one — that is
  `governed-page-cites-the-discipline`.
- **`.kb/**` — nothing**, and **`.redkiln/templates/**` — nothing**, so `redkiln doctor`
  still reports exactly six `template-drift` advisories and `redkiln adopt --templates`
  is never run.
- **A fold-checker, a comprehension quiz, and the re-observation of DoD-8/DoD-12 on the
  assembled tree** — HS-P0020's DT-7, HS-P0024, and HS-P0025 respectively.

```
standards/pages/**
.bklg/docs-that-teach/page-need-discipline/reviewer-and-citation-procedures/**
```

**Merge DoD, one line.** Bands `30` and `40` exist, conform to the atom grammar and its
ceilings, are reachable from both router regions with every link resolving, and state
their own blind spots first; a named non-author has run the walk against the fixture to
a recorded `fail — two needs`, the governed-set walk is recorded as **vacuous** rather
than as a pass, and the paraphrase spot check has been run over `standards/pages/**` with
its verdict in `_ledger.md`; `git diff main -- xtask standards/rust docs .kb` is empty
and `cargo xtask ci --fast`, `cargo xtask lints`, `cargo xtask spec-trace` and
`cargo xtask affected --base main` are green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Band `30` exists at a pinned name** | `standards/pages/30-citing-the-specification.md`. `_design.md` names no file for this band, so the name is pinned here and is depended on by value the moment the router links it — the same "renaming it later is not a rename" cost the design records for the directory itself. | `.bklg/docs-that-teach/page-need-discipline/_design.md` `## Surfaces`, `## Sign-off` condition 2; `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` band table |
| **The citation rule is stated as a test, not a preference** | A normative claim on a governed page is a **citation of a clause id**; restating a clause's content is forbidden. The applied test is the constitution's own: *could a conformant adapter written in another language violate this sentence?* Yes → it is a clause, and the page cites it. A rule phrased as "prefer citations" is rejected: the spot check would have nothing to fail. | `standards/rust/README.md:32-36`; project `project.md` DR-09; `.bklg/docs-that-teach/initiative.md:344` (BR-09) |
| **The citation is visible as a citation** | The clause id is the **visible link text**, so the reader can see they are being handed to `spec/SPECIFICATION.md` rather than to a paraphrase. The id — never a line number — is what is written, because ids are stable names and are never renumbered. | UX brief UX-010 (`_decomposition.md:533-541`); `spec/SPECIFICATION.md:280`; `_grounding.md:240-245` |
| **Band `30` states what nothing checks, first** | The atom says, before its rules: resolution is checked elsewhere (HS-P0020's `clause_ids`), and **paraphrase is checked by nothing mechanical** — a page may cite a real, resolving id and restate its content underneath. RS-81-1's obligation applied to prose, with the in-repo sentence it mirrors quoted at its source rather than re-derived. | `standards/rust/81-checks-that-cannot-be-types.md:11`; `xtask/src/lint_constitution.rs:20-21`, `:10-28`; architecture brief Note 7 item 4 (`_decomposition.md:331-333`) |
| **Band `40` exists at the name and anchor the design addresses** | `standards/pages/40-reviewing-a-page.md`, with the ordered walk under a `### The walk` heading inside `## RP-40-1.`'s **Do** section so the surface route's `#the-walk` fragment resolves. The sub-heading is parser-safe: a rule is split at the next line beginning `"## "`, which `###` does not match. | `_design.md` `## Surfaces` (`reviewer-procedure` route); `xtask/src/lint_constitution.rs:262,275-279`, `:870-877` |
| **The walk names its performer and its permitted sources** | One paragraph before the steps: performed by someone who is **not the author**; they may consult the rendered page, the router and `spec/SPECIFICATION.md`; they may **not** consult the author or the page's git history. That closure is what makes DoD-8's check independent of the author's memory. | `_design.md` `## Composition` S5; `.bklg/docs-that-teach/initiative.md:441-444`; UX brief UX-009 (`_decomposition.md:526-532`) |
| **Every step is a question with a yes/no answer and a consequence** | Numbered, executed top to bottom, from the page alone. No step contains "consider", "use judgement", "as appropriate" or "if it seems"; a step that cannot be answered yes or no is not a step. | `_design.md` `### S5`, `## Anti-patterns` 15; the `Do`/`Not`/`Rejects.` grammar at `standards/rust/README.md:99-112` |
| **The walk closes in a four-row verdict table** | `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate`. Four, not the three `## Composition` states, because `## States` is the addressing manifest and `## Mock` finding 3 records the discrepancy with the mock drawing four. `indeterminate` means the walk could not be completed from the page alone and is recorded as **a defect in the page, not in the procedure**. | `_design.md` `## States` row S5, `## Composition` S5, `## Mock` finding 3 |
| **The empty set is a stated outcome, not an accidental pass** | The procedure text itself instructs: no pages to review → record the walk as **vacuous**; never record a pass. At this merge that is the actual state of the governed set — `docs/` is deliberately near-empty and no narrative tree is merged here. | `_design.md` `## States` row S5 ("rather than recording a pass"); `docs/README.md:1-6`; `RUNBOOK.md:920-925` for the cost of the inverse |
| **The procedure has been run, and has returned `fail`** | A named non-author executes the walk against `standards/pages/examples/two-needs.md` and records `fail — two needs`; the walker's identity and the verdict both go in `_ledger.md`. This is the *procedural (ledger-recorded)* tier, which is proof here because no `#[test]` can carry the judgement. | UX brief UX-009 (`_decomposition.md:526-532`); Testing brief AC-009 (`:913-923`) and the fifth tier (`:785-791`) |
| **The worked example is a linked fixture, and it is inert** | `standards/pages/examples/two-needs.md`, linked from `## RP-40-1` rather than inlined. It is **not** a governed page (a permanently broken page inside the checked surface would make the gate red forever) and **not** an atom (the corpus reader is non-recursive and takes top-level `.md` entries only). | `_design.md` `## Transience policy` row "S5 worked example"; `xtask/src/lint_constitution.rs:208-220`; `_storymap.md:50` (breaking a real page belongs to `declaration-check-seen-to-fail`) |
| **The paraphrase spot check is a written procedure with a corpus** | `## RP-40-2`: for each normative sentence, is its authority a visible, resolving clause id, or is the clause's content restated here? It is run once at this merge over `standards/pages/**` — the only prose in the diff that makes normative claims — and its verdict recorded as a check over the **rules tree**, explicitly not over the narrative set. | project `project.md` AC-010 and Risks row 4; Testing brief AC-010 (`_decomposition.md:924-936`); `.bklg/docs-that-teach/initiative.md:455-457` |
| **No second parser, and the seam is stated in both directions** | Whether a cited id resolves is HS-P0020's `clause_ids`, a sibling of `spec_trace::all_rules`. This story builds nothing mechanical and touches no `xtask/src/**`; band `30` says which half is whose, so a reader does not assume the gate covers the other. | `xtask/src/spec_trace.rs:1746`; architecture brief AC-010 (`_decomposition.md:120-129`), Note 6 (`:304-311`); `standards/rust/81-checks-that-cannot-be-types.md:209` |
| **Both atoms obey the shape the checker will enforce** | `# NN — Title` · `> **Load when:**` (one source line) · `> **See also:**` · `---` · `## RP-NN-N.` per rule with **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that order. Ceilings: ≤ 6 rules, ≤ 16,384 bytes, ≤ 96 columns outside tables; `Rejects.` written at ≥ 120 characters so an inherited floor cannot fail it retroactively. | `standards/rust/00-prime-directives.md:1-9,19,30,54,78`; `xtask/src/lint_constitution.rs:477-509`, `:82,88,95`, `:247-255`; `_design.md` `## Density budget` |
| **Fences are `text` or `markdown`; never `rust`, never untagged** | Nothing in the workspace compiles this tree, so a `rust` fence is a Rust claim nothing checks; untagged is refused too, so a later decision to register the tree cannot be undermined retroactively. Both atoms and the fixture obey it. | `xtask/src/lib.rs:28`; architecture brief Note 4; `_design.md` anti-pattern 13 |
| **Both atoms are mounted in both router regions** | Two rows in the generated index — link, first `Load when` line, comma-separated rule ids, in that order and byte-identical to what `--write` will emit next slice — and one `## Start here` row each, inside the ≤ 12-row ceiling. Every `.md` link added resolves. | `xtask/src/lint_constitution.rs:400-420`, `:343-356`; `_design.md` `## Density budget`; `_storymap.md`, "Why the slices fall here", first bullet |
| **Nothing is folded, and nothing is a widget** | No `<details>`, tab strip or accordion anywhere in the diff — including the fixture — while `PERMITTED_FOLD_MECHANISMS` ships empty; the walk's steps and the verdict table are persistent chrome; the only opened-on-demand control is the link to the fixture. | `_design.md` `## Transience policy` rows S5, `## Anti-patterns` 6 and 9; UX-003, UX-011 |
| **`Not` and `Rejects.` name a review that could actually ship** | Each rule names its wrong state: for `## RP-40-1`, the review that reaches "looks fine" without executing the walk; for band `30`, the page that helpfully summarises a clause "for the reader's convenience". An atom with no `Not` or no `Rejects.` is decorative. | `_design.md` `## Anti-patterns` 14, `### S5` ("**Rejects.** naming the review that reaches 'looks fine'"); `standards/rust/README.md:99-112`; `CLAUDE.md`, the decorative-rule corollary |
| **Nothing else moves** | No `xtask` source, no `standards/rust/` file, no `docs/README.md` edit, no `.kb/` file, no `.redkiln/templates/` file, no narrative page, no clause. `redkiln doctor` still reports exactly six `template-drift` advisories. | architecture brief Note 8 (`_decomposition.md:341-361`); `CLAUDE.md`; project `project.md` Definition of done |

## Data and migrations

**N/A — no data and no migration.** This story adds three markdown files and edits two
regions of a fourth. It touches no schema, no serialised format, no wire envelope, no
stored state and no compiled code: `standards/pages/` is deliberately not registered
with the doctest harness (`xtask/src/lib.rs:28`), and nothing in the diff is read by
`rustc`.

Three obligations that are *not* migrations but are the closest this story has to one.
Each is a contract with a story that has not been implemented yet, and each is stated
here so the later story inherits it rather than discovers it:

- **The two generated index rows are a format contract, not stored data.** The generator
  arrives with `page-need-checker-mounted-in-the-gate`, and its first `--write` against
  the router must produce **no diff** — link form, trigger cell, rule-id cell, `|`
  escaping and row order all as `xtask/src/lint_constitution.rs:400-420` builds them.
  The dependency story took the same obligation for the region as a whole; this story
  extends it to its own two rows.
- **The fixture must stay invisible to both walks.** `standards/pages/examples/two-needs.md`
  is safe today because the corpus reader the checker copies is non-recursive and takes
  top-level `.md` entries only (`:208-220`), and because the file is not under the
  governed page tree. If the checker story makes either walk recursive, or if
  `PAGE_DIR` is ever pinned to a root that contains `standards/pages/`, this fixture
  becomes a permanently failing page inside the gate — so that story must either keep
  the walk non-recursive or exclude `standards/pages/examples/**` explicitly, and say
  which.
- **The recorded runs are dated evidence with a named re-observation, not a discharged
  claim.** Both procedures are run once here against the set that exists; HS-P0025
  re-observes DoD-8 and DoD-12 on the assembled tree (project `project.md`, Out of
  scope). The ledger entries must therefore say what corpus was walked and when, so a
  later reader cannot mistake a vacuous walk or a rules-tree spot check for a pass over
  the narrative set.

## Acceptance criteria

Ten criteria. Each is framed from a **reader's intent crossing the whole stack** — from
the router, into one rule file, to a verdict recorded against a real page — because that
traverse is the product here: a procedure that is correct and unperformed satisfies
nothing, and that is the exact defect this initiative measured
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:174-181`). The
personas are the project's: **S4, the non-author reviewer** — *"reach the same verdict the
author would, from the page alone"* — and the **next page author**, mid-sentence, deciding
whether to cite a clause or explain it
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, UX brief UX-009, UX-010).

Every verification is runnable from the worktree root in `git bash`. Where a criterion's
**permanent** regression assertion lands in the next slice — it cannot land here, this PR
touches no `xtask/src/**` — the verification names both the mechanical check that runs
*now* and the forward test that inherits it; see "Clarifications resolved during spec".

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the next page author, mid-sentence, about to write *"a write re-reads what it decided on"* and unsure whether to explain it or point at it, **WHEN** they open band `30` from the router, **THEN** they meet a **falsifiable test** rather than a preference — *could a conformant adapter written in another language violate this sentence? yes → it is a clause, and the page cites it* — **AND** the atom tells them the citation is written as the **stable clause id as visible link text**, never a line number and never a paraphrase, so a later reader can see they are being handed to `spec/SPECIFICATION.md`. | *Mechanical.* `test -f standards/pages/30-citing-the-specification.md`; `rg -n '^# 30 — ' standards/pages/30-citing-the-specification.md` → one hit; `rg -ni 'never restate' standards/pages/30-citing-the-specification.md` → ≥ 1; `rg -ni 'visible link text' standards/pages/30-citing-the-specification.md` → ≥ 1; `rg -n 'clause ids are stable\|never renumbered' standards/pages/30-citing-the-specification.md` → ≥ 1, and the cited authority is `spec/SPECIFICATION.md:280`. *Procedural (ledger).* The author-at-a-sentence walk: take three sentences — one normative, one explanatory, one mixed — apply the atom's test to each from the atom's text alone, and record the three verdicts and which sentence the atom says must be split. Precedent for the test's wording: `standards/rust/README.md:32-36`. |
| **AC-002** | **GIVEN** a contributor who assumes that because this repository has a gate, the gate is watching citations, **WHEN** they read band `30` from the top, **THEN** the atom tells them **first**, before any rule, what nothing checks: that whether a cited id *resolves* is HS-P0020's `clause_ids` and not this tree's, and that whether a page **restates a clause it correctly cites** is checked by **nothing mechanical** — so they leave knowing a green gate is not a claim about paraphrase, and knowing that band `40`'s spot check is the instrument for the rest. | *Mechanical.* The blind-spot statement precedes the first rule: `rg -n 'does not\|not checked\|nothing' standards/pages/30-citing-the-specification.md \| head -n 1` reports a line number **lower** than `rg -n '^## RP-30-1' standards/pages/30-citing-the-specification.md`, captured as the two numbers. `rg -n 'clause_ids' standards/pages/30-citing-the-specification.md` → ≥ 1. *Procedural (ledger).* A reader who has not seen this spec reads only that section and states, in their own words, the two facts above and which project owns each half; recorded verbatim. Bar and precedent: RS-81-1 (`standards/rust/81-checks-that-cannot-be-types.md:11`), worked at `xtask/src/lint_constitution.rs:10-28` and, for this exact sentence, `:20-21`. |
| **AC-003** | **GIVEN** the non-author reviewer, handed a page and asked "is this one need or two?", who has no access to the author and will not read the page's git history, **WHEN** they follow the router to `standards/pages/40-reviewing-a-page.md#the-walk`, **THEN** the fragment **resolves to a heading**, and before the first step they are told who performs the walk (**not the author**) and exactly what they may consult (the rendered page, the router, `spec/SPECIFICATION.md`) and what they may **not** (the author; the page's git history) — so the check is independent of the author's memory, which is the whole subject of DoD-8. | *Mechanical.* `test -f standards/pages/40-reviewing-a-page.md`; `rg -n '^### The walk$' standards/pages/40-reviewing-a-page.md` → **one** hit, and its line number falls between `^## RP-40-1` and `^## RP-40-2` (both captured), proving the sub-heading sits inside the rule body the way `rules()` splits at `"## "` only (`xtask/src/lint_constitution.rs:262,275-279`, proven by `rules_are_split_at_the_next_heading`, `:870-877`). `rg -ni 'not the author' standards/pages/40-reviewing-a-page.md` → ≥ 1; `rg -ni 'git history\|git log' standards/pages/40-reviewing-a-page.md` → ≥ 1, in a forbidding sentence. *Procedural (ledger).* The router→anchor traverse run keyboard-only and recorded as `router row → file → #the-walk`, ending at the performer paragraph. |
| **AC-004** | **GIVEN** that same reviewer, who is a stranger to the page and to this project, **WHEN** they execute the walk top to bottom **from the rendered page alone**, **THEN** every step is a **question with a yes/no answer and a stated consequence**, numbered and executed in order — **AND** no step contains "consider", "use judgement", "as appropriate" or "if it seems", so two different strangers walking the same page reach the same verdict rather than two impressions. | *Mechanical.* Hedging is absent from the walk region: `awk '/^### The walk$/,/^## RP-40-2/' standards/pages/40-reviewing-a-page.md \| rg -ni 'consider\|judgement\|judgment\|as appropriate\|if it seems'` → **no output** (`_design.md` anti-pattern 15). Step shape: `awk '/^### The walk$/,/^## RP-40-2/' standards/pages/40-reviewing-a-page.md \| rg -c '^[0-9]+\. '` → ≥ 3, and every matched step line ends in `?` — captured by reading the extracted block into the ledger verbatim. *Procedural (ledger).* Two different people who did not author the fixture each walk `standards/pages/examples/two-needs.md` and record their step-by-step yes/no answers; the two answer sequences are compared and the comparison recorded. Divergence on any step is a defect in the step, not in the walker. |
| **AC-005** | **GIVEN** a reviewer who has finished the steps and now has to write something down that another person can act on, **WHEN** they reach the end of the walk, **THEN** it closes in a **four-row verdict table** — `pass` · `fail — two needs` · `fail — need not answered` · `indeterminate` — each row saying what the verdict means and what happens next; **AND** `indeterminate` is defined as *the walk could not be completed from the page alone*, recorded as **a defect in the page, not in the procedure**, so a reviewer never has a soft pass available to them. | *Mechanical.* `awk '/^\| *Verdict/,/^$/' standards/pages/40-reviewing-a-page.md \| tail -n +3 \| wc -l` → **4**. Each of the four literal strings present: `rg -n 'fail — two needs\|fail — need not answered\|indeterminate\|^\| *pass' standards/pages/40-reviewing-a-page.md`, all four captured. `rg -ni 'defect in the page' standards/pages/40-reviewing-a-page.md` → ≥ 1. *Procedural (ledger).* The calibration walk (AC-007) lands on exactly one of the four rows and the row's stated "what happens next" is executed or recorded as not-executed with a reason. Authority for four rather than three: `_design.md` `## States` row S5 and `## Mock` finding 3. |
| **AC-006** | **GIVEN** the first person to run this walk over the governed set — which at this merge is **empty**, because no narrative tree is merged in this worktree — **WHEN** they find nothing to review, **THEN** the procedure's own text instructs them to record the walk as **vacuous**, in those words, and **never** as a pass; **AND** the sweep actually performed for this story is recorded in `_ledger.md` as vacuous with the corpus and date named, so no later reader can mistake an empty walk for a clean one. | *Mechanical.* `rg -ni 'vacuous' standards/pages/40-reviewing-a-page.md` → ≥ 1, in an instruction to the reviewer rather than a note about the tree; the surrounding sentence captured verbatim. The emptiness itself is captured: `ls docs/*.md` and the absence of any governed page tree, plus `cat docs/README.md` (`:1-6`, "user documentation. nothing else."). *Procedural (ledger).* One row reading, in substance, *"governed-set walk, <date>, corpus: none — no governed page tree exists at this merge; verdict: vacuous, not pass"*. The failure this forecloses is `RUNBOOK.md:920-925`'s, one medium over: a green record standing in for a check that never ran. |
| **AC-007** | **GIVEN** that a procedure which has never returned `fail` is decorative — the corollary `CLAUDE.md` states for conformance rules, applied to a written one — **WHEN** a **named person who did not author it** executes the walk against `standards/pages/examples/two-needs.md`, a specimen page carrying two declarations and linked from `## RP-40-1` as the worked example, **THEN** they reach `fail — two needs`, and both the walker's identity (not the author) and the verdict are recorded in `_ledger.md`; **AND** the fixture is deliberately inert — outside the governed page tree and outside the atom namespace — so it can be permanently broken without ever making the gate red. | *Mechanical.* `test -f standards/pages/examples/two-needs.md`; the file carries **two** `> **Answers:**` lines — `rg -c '^> \*\*Answers:\*\*' standards/pages/examples/two-needs.md` → `2`. It is linked, not inlined: `rg -n 'examples/two-needs\.md' standards/pages/40-reviewing-a-page.md` → ≥ 1, inside `## RP-40-1`. It is invisible to the corpus reader the checker copies (non-recursive, top-level `.md` only, `xtask/src/lint_constitution.rs:208-220`): `ls standards/pages/*.md` does **not** list it, output captured. *Procedural (ledger).* The named walker, the date, the step-by-step answers and the verdict `fail — two needs`, recorded verbatim per UX-009's own test (`_decomposition.md:526-532`) and the *procedural (ledger-recorded)* tier (`:785-791`, `:913-923`). |
| **AC-008** | **GIVEN** the initiative's own risk that *the discipline becomes a second specification* (project `project.md`, Risks row 4), **WHEN** the paraphrase spot check `## RP-40-2` is run once over the only prose in this repository that currently makes normative claims and cites clause ids — `standards/pages/**`, this discipline's own tree — **THEN** it yields a recorded verdict per file, stating for each normative sentence whether its authority is a **visible, resolving clause id** or a **restatement**; **AND** the verdict is recorded as a check over the **rules tree**, explicitly not over the narrative set, whose walk HS-P0025 re-observes — **AND** no second clause-id parser was built to do it. | *Mechanical, and this is the boundary that makes the criterion honest:* `git diff main -- xtask` → **empty**, output pasted into `_ledger.md`; `git diff main -- spec/` → **empty**. `rg -n '^## RP-40-2' standards/pages/40-reviewing-a-page.md` → one hit, with a **Do** section naming the corpus and the per-sentence question. *Procedural (ledger).* The spot check run file by file over `standards/pages/*.md`, each file's verdict recorded with the sentence count examined; the resolution half explicitly deferred to HS-P0020's `clause_ids` (`xtask/src/spec_trace.rs:1746` for the sibling shape) per architecture brief AC-010 (`_decomposition.md:120-129`) and Testing brief AC-010 (`:924-936`). |
| **AC-009** | **GIVEN** the page author who will load one of these atoms — never the tree — on the task where they need it, **WHEN** either atom is measured rather than asserted, **THEN** it is real composed presentation in this repository's own grammar and inside its budget: `# NN — Title` · a **one-source-line** `> **Load when:**` · `> **See also:**` · `---` · then `## RP-NN-N. <imperative sentence>` per rule with **Why.** · **Do** · **Not** · **Rejects.** · **Evidence.** in that order — ≤ **6** rules, ≤ **16,384** bytes, ≤ **96** columns outside tables, every fence tagged `text` or `markdown`, every `Rejects.` ≥ **120** characters, and **no rule missing its `Not` or its `Rejects.`**, because an atom that never shows a wrong page is decorative. | *Mechanical, every number a command whose output is pasted into `_ledger.md`.* `wc -c < standards/pages/30-citing-the-specification.md` and the same for band `40` → each ≤ `16384`. `rg -c '^## RP-30-' standards/pages/30-citing-the-specification.md` and `rg -c '^## RP-40-'` → each ≤ `6`. `awk '!/^\|/ && length > 96 {print FILENAME":"FNR": "length}' standards/pages/30-citing-the-specification.md standards/pages/40-reviewing-a-page.md` → **no output**. Section presence and order: `rg -n '^\*\*Why\.\*\*\|^\*\*Do\*\*\|^\*\*Not\*\*\|^\*\*Rejects\.\*\*\|^\*\*Evidence\.\*\*' <atom>` prints the five markers in `SECTIONS` order once per rule (`xtask/src/lint_constitution.rs:67-81`, enforced by `check_shape` at `:477-509`). `rg -n -A1 'Load when:' <atom>` → the following source line does **not** begin with `>` (`:247-255`). `rg -n '^```rust' standards/pages/` → no matches; `rg -n '^```$' standards/pages/` → no matches. Each `**Rejects.**` paragraph's character count captured and compared against `MIN_REJECTS_CHARS = 120` (`:82`). |
| **AC-010** | **GIVEN** any of these readers arriving at `standards/pages/README.md` — the tree's only composition root — **WHEN** the router first loads with **nothing clicked**, **THEN** both new atoms are reachable from **both** mount regions: one row each in the generated index between `<!-- BEGIN GENERATED -->` and `<!-- END GENERATED -->`, byte-identical to what `generated_region`'s analogue will emit, and one `## Start here` row each inside the ≤ **12**-row ceiling; **AND** every `.md` link added by this PR resolves; **AND** nothing anywhere in the diff — atoms, fixture, router rows — sits behind a `<details>`, a tab, an accordion or a bespoke navigation widget, and no meaning is carried by colour, an icon or size, so the walk's steps and its verdict table are present on first load and the only opened-on-demand control is the link to the fixture. | *Mechanical.* In-region rows equal the corpus: `sed -n '/BEGIN GENERATED/,/END GENERATED/p' standards/pages/README.md \| grep -c '^\|'` minus 2 equals `ls standards/pages/*.md \| grep -v README \| wc -l`, both numbers captured before and after this PR. Row format pinned to `xtask/src/lint_constitution.rs:400-420`: a derivation record mapping each of the two new rows to link form, first `Load when` line, and comma-separated rule ids, with every interior `\|` escaped and no blank line inside the markers. Filter ceiling: `awk '/^## Start here/,/^## Index/' standards/pages/README.md \| grep -c '^\|'` → ≤ `14`. Links resolve: every `NN-slug.md` and `examples/two-needs.md` target passes `test -f` (the check `:343-356` will make permanent). Absence: `rg -n '<details>\|<summary>\|role="tab"\|\{\{#tab' standards/pages/` → no matches; `rg -n '<small>\|<sub>\|<sup>\|<nav>\|<img' standards/pages/` → no matches. *Procedural (ledger).* Read all three new files through `less` with no renderer and confirm every walk step, the verdict table and both blind-spot statements are legible with nothing lost; then `git checkout -- standards/pages && git status` → clean, no residue. |

**Project-AC coverage.** Project **AC-009** (*the reviewer procedure is non-author-
performable*, DoD-8) ← story **AC-003, AC-004, AC-005, AC-006, AC-007** — the walk exists
at the addressed anchor, its steps are answerable, it yields one of four verdicts, its
empty-set behaviour is stated rather than accidental, and a named non-author has run it to
a `fail`. Project **AC-010** (*the citation rule exists and the spot check runs*, DoD-12) ←
story **AC-001, AC-002, AC-008** — the rule is stated as a falsifiable test, its blind
spots are stated first, and the spot check has been run over the corpus that exists. Story
**AC-009** and **AC-010** are the cross-cutting composition and reachability criteria: they
carry no project AC of their own and instead keep both of the above from landing
half-mounted or unreadable, which is the failure `_storymap.md`'s first "Why the slices
fall here" bullet names.

## Interaction quality

RFC §6.7/D6. Every invariant that applies is an **AC row above**, not a bullet here: this
section says *which id carries which invariant* and how each is verified, so nothing here
is separately gateable and nothing gateable lives only here. The medium is text, so each
invariant is restated in the form it takes with no DOM — that translation is `_design.md`'s
(`## Transience policy`, first paragraph), not invented in this spec.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** — the answer is on the page the reader landed on | **AC-002** (band `30`'s blind spots stated in the atom itself, before its first rule, not in a separate note), **AC-003** (the performer and permitted-sources paragraph sits immediately above the steps, not in the router) | the line-number ordering capture in AC-002; the `^### The walk$` position check in AC-003. The forbidden shape is a procedure whose preconditions live somewhere the reviewer has to go and find |
| **One intended context switch, and the reader chooses it** | **AC-007** (the worked example is a **link to a fixture**, the single opened-on-demand control in this diff), **AC-010** (index→atom links) | `_design.md` `## Transience policy` row "S5 worked example" — *"a link to a fixture, not an inline fold"* — and the `rg -n 'examples/two-needs\.md'` check that it is linked rather than inlined |
| **Non-occlusion — the procedure's output is never hidden behind its input** | **AC-010** (nothing behind `<details>`, a tab or an accordion anywhere in the diff), reinforced by **AC-005** (the verdict table is present on first load) and **AC-004** (the steps are) | the absence greps in AC-010 plus the `less`-with-no-renderer read. `_design.md` `## Transience policy` rows "S5 walk steps" and "S5 verdict table", both **persistent chrome**; anti-patterns 5 and 6 |
| **Preserved focus, scroll and selection** | **AC-010** | discharged by **absence of the mechanism**: this diff authors no disclosure, no tab and no script, so there is nothing that *can* move focus or scroll. AC-010's greps are what keep it that way (UX brief, Note 2, item 5) |
| **Reversibility** | **AC-010** (`git checkout -- standards/pages && git status` → clean, no cache, no generated artifact, no dirty file), **AC-007** (the fixture is inert by construction — reverting it can never leave a red gate behind) | the residue capture in AC-010; the `ls standards/pages/*.md` proof in AC-007 that the fixture is outside every walk the checker will perform |
| **Keyboard reachability** | **AC-003** (the router→`#the-walk` traverse is run keyboard-only), **AC-010** (every added link is a plain markdown link; no widget that could need a pointer) | the recorded traverse and the absence greps. UX-006's own test wording, applied to the rules tree rather than to a governed page |

**Composition family** — taken from the signed-off
`.bklg/docs-that-teach/page-need-discipline/_design.md`, which is **binding** on this story
because it renders the `rule-atom` and `reviewer-procedure` surfaces and changes
`discipline-router`. This story implements it and re-decides none of it.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every control is real composed presentation in the repo's own textual grammar (atom head, five marker-delimited sections, a numbered list, a markdown verdict table, generated-region and filter rows), not bare prose that happens to contain the words | **AC-009** (the atom grammar and its five sections), **AC-004** (the walk is an ordered list), **AC-005** (the verdict is a table, not a sentence), **AC-010** (both router regions) | the `SECTIONS`-order grep in AC-009; the `^[0-9]+\. ` step count in AC-004; the four-row `awk` count in AC-005; the region checks in AC-010. `_design.md` `## Composition` S3 and S5 |
| **Composition and placement** — S5's binding region order: `## RP-40-1.` heading → the performer/permitted-sources paragraph → the ordered walk → the verdict table → a `**Rejects.**` naming the review that reaches "looks fine" | **AC-003** (heading and performer paragraph), **AC-004** (walk), **AC-005** (table), **AC-009** (the `Rejects.` that closes it, present and ≥ 120 chars) | the line-number ordering captures across AC-003/004/005 read as one sequence, compared against `_design.md` `## Composition` "S5 — the reviewer procedure, in the atom that carries it" |
| **Transience** — walk steps and verdict table are **persistent chrome**; `Why.`/`Do`/`Not`/`Rejects.`/`Evidence.` are persistent chrome; the worked example is the one **opened on demand**, and it is a link | **AC-010**, **AC-007**, **AC-009** | `_design.md` `## Transience policy` rows S3 and S5. The reason a link is permitted where a fold is not: it is *"the one 'opened on demand' mechanism whose accessibility this repository does not have to verify"* (UX-011's burden of proof) |
| **Density budget, with its real numbers** — ≤ 6 rules and ≤ 16,384 bytes per atom, ≤ 96 columns outside tables, `Rejects.` ≥ 120 characters, ≤ 12 `## Start here` rows, `Load when:` on one source line | **AC-009** (the atom half), **AC-010** (the router half) | the `wc`/`rg -c`/`awk` measurements in AC-009 and the `awk` row count in AC-010, each output pasted into the ledger (NF-004). `_design.md` `## Density budget` and its per-surface yield order — for S5, *"the worked example yields (it is a link already). The walk's steps never do."* |
| **Hierarchy** — the `## RP-NN-N.` imperative sentence primary, `Do`/`Not` secondary, `Why.`/`Rejects.`/`Evidence.` recessive, carried by **heading level and bold run-in markers**; within S5, the ordered steps primary and the performer paragraph recessive, carried by **numbered-list structure vs prose** — never by colour or size | **AC-009**, **AC-004** | the `SECTIONS` grep plus the `<small>`/`<sub>`/`<sup>` absence grep in AC-010. `_design.md` `## Hierarchy`, rows S3 and S5. "Recessive" means *read third*, never *removed* |
| **Named anti-patterns refused** — 6 (any `<details>`/tab/accordion at all while `PERMITTED_FOLD_MECHANISMS` is empty), 9 (a bespoke navigation widget), 13 (a `rust`-tagged fence anywhere in `standards/pages/`), 14 (a rule atom with no `Not` or no `Rejects.`), 15 (a walk step containing "consider", "use judgement", "as appropriate", "if it seems") | 6, 9 → **AC-010**; 13, 14 → **AC-009**; 15 → **AC-004**; 5 (a never-fold class invisible until clicked) → **AC-010** | the greps named in each AC. Anti-pattern 15 is the one with a *scoped* test — the hedging grep runs over the extracted walk region only, because "consider" is legitimate English in a `Why.` paragraph and forbidden in a step |

**Why this matters here specifically.** An unstyled render satisfies every structural
assertion in this spec: a file containing the five section markers, a numbered list and a
four-row table would pass most of the greps above while being unusable. The composition
rows are what fail it — a `Rejects.` under 120 characters, an atom over 16,384 bytes, a
walk step that hedges, a verdict table with three rows, an atom with no router row. There
is **no perceptual review** to catch what those miss (`design.capture` is absent from
`.redkiln/config.yaml`, and `_design.md` records the skip as deliberate), so `_design.md`
plus these rows are the only instrument this story has.

## Error conditions

| id | condition | required handling |
| --- | --- | --- |
| **EC-001** | The dependency `router-precedence-and-announcement` has not merged, so `standards/pages/README.md` — the mount point — does not exist, or exists without the band table reserving `30` and `40`. | **Stop; do not author the router.** Standing up the tree, its precedence block, its scope paragraph, its `## What checks this tree, and what does not` section and the `docs/README.md` announcement is that story's whole slice, and authoring any of it here creates a merge conflict inside a file whose generated region is compared for **byte equality** one slice later. Bands `30` and `40` may be drafted; the PR does not merge without the router. |
| **EC-002** | The `#the-walk` fragment does not resolve — the walk was written as a bold run-in, a list caption or a paragraph rather than a `### The walk` heading. | **`_design.md`'s own addressing manifest becomes a dead link**, and the surface it names cannot be reviewed. Only a heading generates the anchor. Fix by promoting it to `### The walk` inside `## RP-40-1.`'s **Do** section — which is parser-safe, because a rule is split at the next line beginning `"## "` and `###` does not match (`xtask/src/lint_constitution.rs:262,275-279`). Do **not** fix it by moving the walk to its own `##` rule: that would split `## RP-40-1` and strand its `Rejects.`. |
| **EC-003** | The governed-set sweep is recorded as a **pass** because there was nothing to walk. | **The decorative-gate failure with this project's name on it.** `_design.md` `## States` row S5 fixes the required behaviour — *"the procedure states the walk is vacuous and the reviewer records that, rather than recording a pass"* — and the instruction must be in the **procedure's own text**, not only in this spec, because the spec is not what the next reviewer reads. `RUNBOOK.md:920-925` is what this costs when it is got wrong. |
| **EC-004** | The calibration walk returns `pass`, or is not run at all because "the procedure is obviously correct". | **AC-007 fails outright.** A procedure that has never returned `fail` is decorative — the corollary `CLAUDE.md` states for conformance rules, and the reason this PR ships the page it fails on. If the walk genuinely cannot reach `fail — two needs` on a page carrying two declarations, the defect is in the **steps**, not in the fixture: rewrite the step that failed to discriminate and re-run, recording both runs. |
| **EC-005** | The walker who runs the calibration is the person who authored band `40`. | The verdict is **not evidence** and the row may not be flipped. The whole subject of DoD-8 is a check that does not depend on the author's memory (`.bklg/docs-that-teach/initiative.md:441-444`); an author walking their own procedure over their own fixture is the author-flattering shape `_design.md` rejects by name. Record the walker's identity in the ledger, not merely the verdict. |
| **EC-006** | `standards/pages/examples/two-needs.md` is placed inside the governed page tree, or the checker story later makes either corpus walk recursive, or pins `PAGE_DIR` to a root containing `standards/pages/`. | **The fixture becomes a permanently failing page inside the gate** — red forever, with no revert that fixes it. Today it is safe because the corpus reader is non-recursive and takes top-level `.md` entries only (`xtask/src/lint_constitution.rs:208-220`). This is stated as a forward obligation in "Data and migrations": `page-need-checker-mounted-in-the-gate` must either keep both walks non-recursive or exclude `standards/pages/examples/**` explicitly, **and say which**. Breaking a *real* page is `declaration-check-seen-to-fail`'s story, once, with a revert. |
| **EC-007** | A rule in either atom restates a `spec/SPECIFICATION.md` clause "for the reader's convenience", or cites a clause by **line number** instead of by id. | **The sharpest possible self-refutation**, and AC-008's spot check over `standards/pages/**` is aimed exactly here. Fix by replacing the restatement with the clause id as visible link text; a line reference is rejected because ids are stable and lines are not (`spec/SPECIFICATION.md:280`), which is also why a citation survives the sibling branch's divergence (`_grounding.md:240-245`). |
| **EC-008** | An atom or the fixture carries a `rust`-tagged or an untagged code fence. | Rejected, both. Nothing in the workspace compiles this tree (`xtask/src/lib.rs:28` registers only `mod constitution`), so a `rust` fence is a Rust claim nothing checks — anti-pattern 13. An **untagged** fence is refused too, so a later decision to register the tree cannot be undermined retroactively (architecture brief, Note 4). Tag `text` or `markdown`. |
| **EC-009** | Either atom's `> **Load when:**` block wraps onto a second source line. | The generated index silently carries a **mid-phrase fragment**, because `load_when` reads only the first line of the block and drops continuations (`xtask/src/lint_constitution.rs:247-255`) — the precedent's own defect, visible at `standards/rust/README.md:70-71`, which the dependency's AC-005 exists to stop this tree inheriting. One source line, however long; shorten the trigger phrase rather than wrapping it. |
| **EC-010** | The temptation arrives to add "just a small helper" in `xtask/src/` — a `PAGE_RULE_DIR` const, a test, an `INERT` entry — to make one of the mechanical checks above permanent. | **Out of scope, and the boundary is a merge condition:** `git diff main -- xtask` is empty (AC-008). The whole gate-step surface is `page-need-checker-mounted-in-the-gate`'s (architecture brief, Note 1, CR-1 through CR-4), and a partial edit here is the half-mount that story exists to avoid. RS-81-3 (`standards/rust/81-checks-that-cannot-be-types.md:209`) forbids the shared abstraction over two trees that the helper would grow into. |
| **EC-011** | Band `30` is folded into band `20`, or band `40` into band `30`, on the grounds that "they are both about writing pages". | Rejected in the dependency's own band table and the reason holds: the citation rule is applied while **writing a sentence**, the walk while **judging a finished page**. An atom that answers two moments is the failure this whole project exists to name, one level up from the pages it governs. Changing the band namespace is a change to a signed-off design plus a merged dependency, not an implementer's choice. |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **No dependency changes and no Rust.** No workspace manifest is touched and nothing is added for measurement — every budget in this spec is `wc`, `awk`, `rg`, `sed` or `test -f`, all present. | `git diff main -- '**/Cargo.toml' Cargo.lock` → empty. The testing brief is explicit that no `tempfile` (or any) dependency is added for this project; this story adds no Rust at all, so the question does not arise here and is recorded as foreclosed. |
| **NF-002** | **LF line endings and UTF-8, no BOM**, on all four touched files. | This is a Windows checkout. `check_router` compares the generated region for string equality after trimming the whole block only — a CRLF interior line is **not** equal to an LF one, so a CRLF row would make the checker story's first `--write` produce a whole-region diff nobody intended. Observe with `file standards/pages/*.md` and `git diff --check`. |
| **NF-003** | **Non-ASCII characters match the corpus's existing set** — the em dash `—` and the middle dot `·` only, and only where the corpus already uses them. No smart quotes, no non-breaking spaces, no emoji. | Two files that render the same idiom two ways is a second grammar. `rg -n '[‘’“”… ]' standards/pages/` → no matches. The verdict strings in particular (`fail — two needs`) are typed with the em dash the corpus uses, because AC-005 greps for them literally. |
| **NF-004** | **Every number in this spec is reproducible from a named command, and the command's *output* — not a claim about it — is what enters `_ledger.md`.** | `_design.md` `## Sign-off` condition 4: no pixel budget is claimed, only source columns and bytes, *"which are measurable today"*. A ledger row reading "checked, within budget" without the number is not evidence, and `verify.require_ledger: true` treats a recorded manual verification as first-class proof only when it is actually recorded. |
| **NF-005** | **Every procedural record names the person, the date and the corpus.** Not "a reviewer ran it" — the walker's identity (and that it is not the author), the date, and what was walked. | This is the *procedural (ledger-recorded)* tier's entire load-bearing content (`_decomposition.md:785-791`). Without the identity, EC-005 is undetectable; without the corpus, AC-006's vacuous sweep and AC-008's rules-tree spot check are indistinguishable from a pass over the narrative set, which HS-P0025 has yet to perform. |
| **NF-006** | **`redkiln doctor` still reports exactly six `template-drift` advisories**, and `redkiln adopt --templates` is never run. | `.redkiln/templates/**` is out of scope (PR boundary) and the `backlog` CI job asserts the set is exactly those six (`CLAUDE.md`). A seventh or a fifth means a template moved in this diff. |
| **NF-007** | **`cargo xtask spec-trace` stays green and `spec/SPECIFICATION.md` is unmodified.** | Nothing here writes, restates or renumbers a clause; the rule this story authors requires pages to **cite**. `git diff main -- spec/` → empty. Load-bearing rather than ceremonial: this is the story whose subject is the normative voice's boundary. |
| **NF-008** | **Both atoms are loadable alone.** A reader who opens band `40` and nothing else can execute the walk; a reader who opens band `30` and nothing else can apply the citation test. Neither atom requires the other, and neither requires this spec. | UX-012 — *"a reader loads one rule, not the corpus"* — is the reason the ceilings exist (`xtask/src/lint_constitution.rs:503-508`, *"an agent loading this pays for all of it"*). Observed as part of AC-004's two-stranger walk: the walkers are given the atom, not the spec. |

## Implementation notes (non-prescriptive)

Not instructions — the shape the front half's decisions imply, offered so the implementer
spends their judgement on the prose rather than on rediscovering the constraints.

- **Write the fixture first, then the walk against it.** The fixture is the only thing in
  this PR that can falsify a step. Author `standards/pages/examples/two-needs.md` as a
  plausible page someone would actually write — a real title, two `> **Answers:**` lines
  that each look defensible in isolation, and enough body that the second declaration is
  not the only clue — then write each walk step by asking *what question, answered yes or
  no from this page alone, exposes it?* A step written before the page it must catch is a
  step written against an imagined defect.
- **Write both `Rejects.` sections before their `Do` sections.** `_design.md` `### S5`
  names band `40`'s: *the review that reaches "looks fine" without executing the walk*.
  Band `30`'s is the page that helpfully summarises a clause "for the reader's
  convenience" — a real, friendly, shippable edit, which is what makes it worth rejecting.
  `MIN_REJECTS_CHARS` is 120 for a reason (`xtask/src/lint_constitution.rs:73-82`): naming
  who is misled and when they find out does not fit in less.
- **Write the blind-spot statements early, while they are uncomfortable.** Band `30`'s
  "resolution is elsewhere, paraphrase is checked by nothing" and band `40`'s "neither
  procedure is a gate step" both get softer every hour they sit unwritten. Model them on
  `xtask/src/lint_constitution.rs:9-28`, which states the limit before the mechanism, and
  quote `:20-21` at its source rather than re-deriving the sentence.
- **A useful order of work:** fixture → band `40`'s `## RP-40-1` (performer paragraph →
  `### The walk` → verdict table → `Rejects.`) → `## RP-40-2` (the spot check) → band `30`
  (blind spots → `## RP-30-1` and its rules) → both atom heads and `Load when` lines →
  the two router rows in both regions → then measure, then run both procedures, then fill
  the ledger.
- **Derive the router rows from the atoms, by hand, the way the generator will.** Read each
  atom's first `> **Load when:**` line and its `## RP-NN-N.` ids, then build the row: link
  form, trigger cell, comma-separated ids, every interior `|` escaped, no blank line inside
  the markers (`xtask/src/lint_constitution.rs:400-420`). Keep the derivation as a small
  table in the ledger — the checker story reads it to confirm its first `--write` produces
  no diff, which is what makes AC-010's obligation reachable rather than lucky.
- **Recruit the non-author walker before you need them.** AC-004 wants two people who did
  not author the fixture and AC-007 wants one named non-author for the calibration; the
  cheapest failure in this story is finishing the prose and then discovering the only
  available walker is the author (EC-005). Their answers are the deliverable, not a
  formality — a step two strangers answer differently is a defect the ledger should carry.
- **Do not build the shared helper you will want.** After writing "for each `.md` under
  `standards/pages/`" twice, the instinct is to factor something out with
  `lint_constitution`. RS-81-3 scopes a scanner to the directory whose behaviour it
  constrains (`standards/rust/81-checks-that-cannot-be-types.md:209`, architecture brief
  Note 6), and in any case this PR contains no Rust — `git diff main -- xtask` is a merge
  condition.
- **Run the measurements as you write, not at the end.** Ten ACs, roughly twenty commands;
  collecting them afterwards is how one gets paraphrased, and a paraphrased measurement is
  the exact failure this story's own rule forbids.

## Tests and CI (merge gate)

Tiers as the project's testing brief defines them
(`.bklg/docs-that-teach/page-need-discipline/_decomposition.md`, Testing brief,
"Acceptance Criteria"): **static** (`#[cfg(test)]`), **gate-integration**,
**end-to-end/fixture**, **procedural (ledger-recorded)**. This story is deliberately
weighted to the last — AC-009 and half of AC-010 at the project grain are judgements no
function signature can carry (`:785-791`), which is why that tier exists at all.

| tier | command / path | proves |
| --- | --- | --- |
| **static** | *(none in this PR — by construction)* | The one permanent assertion this story's ACs imply — the testing brief's *"structural-presence test: the discipline's own rule text states 'cite, never restate'"* (`_decomposition.md:924-928`) — is a `#[cfg(test)]` test inside a module that does not exist until `page-need-checker-mounted-in-the-gate`, and this PR's boundary forbids touching `xtask/src/**`. The obligation is recorded here and inherited by that story; its present-tense verification is AC-001's `rg`. Nothing is dropped. |
| **gate-integration** | `cargo xtask lints` | The file-reading lint family is green and **unchanged** — `lint-constitution` still passes over `standards/rust/`, proving this story perturbed neither the tree it copies its grammar from nor the checker it will be measured by (`verify.reachability_static`, project.md DoD). |
| **gate-integration** | `cargo xtask spec-trace` | `spec/SPECIFICATION.md`'s markers and citations still resolve; **NF-007**. Load-bearing here precisely because this is the story about the normative voice's boundary, and because `cargo xtask spec-trace` is the only writer of that file's generated sections (architecture brief, Note 8). |
| **gate-integration** | `cargo xtask ci --fast` | The bar a non-terminal project is held to (`.redkiln/config.yaml`, `verify.integration_scoped`; project.md DoD). Green at merge. |
| **gate-integration** | `cargo xtask ci` | The merge gate of record (`CLAUDE.md`, "Commands"). Run before the story is called done. |
| **gate-integration** | `cargo xtask affected --base main` | The story grain (`verify.affected_gate`). Expected to widen to the whole workspace on a prose-only diff and to be **green**; the widening is recorded as observed and **not** "fixed" by touching `xtask/src/affected.rs` (EC-010, and the dependency's EC-007 for the same reason). |
| **gate-state** | `git diff main -- xtask` | **AC-008, EC-010.** Empty. Nothing inside this diff can prove a *different* tree was never touched except inspecting the diff, which is the testing brief's own reasoning for AC-002's shape (`_decomposition.md:842-850`). |
| **gate-state** | `git diff main -- standards/rust spec docs .kb .redkiln/templates '**/Cargo.toml'` | The PR boundary and NF-001/NF-006/NF-007 in one command. Empty. `standards/rust` in particular: AC-002 at the project grain is discharged architecturally by never opening that tree. |
| **mechanical (procedural, captured)** | the ~20 `test -f` / `rg` / `awk` / `sed` / `wc` / `ls` invocations named in the acceptance table | **AC-001 … AC-010.** Reproducible from the worktree root in `git bash`, and each one's *output* — never a claim about it — enters `_ledger.md` (NF-004). |
| **procedural (ledger-recorded)** | the author-at-a-sentence walk (AC-001); the blind-spot read-back (AC-002); the keyboard router→anchor traverse (AC-003); the **two-stranger** step-answer comparison (AC-004); the vacuous governed-set sweep (AC-006); the **named non-author calibration walk to `fail — two needs`** (AC-007); the file-by-file paraphrase spot check over `standards/pages/**` (AC-008); the plain-pager read (AC-010) | The judgements no command carries — and here they are the product, not the residue. `.redkiln/config.yaml`'s `require_ledger: true` already treats a recorded manual verification as first-class proof, which is why the testing brief made this a named tier rather than an excuse (`_decomposition.md:785-791`, `:913-923`, `:924-936`). Each record carries a person, a date and a corpus (**NF-005**). |
| **end-to-end/fixture** | `standards/pages/examples/two-needs.md` walked to a recorded `fail — two needs` | **AC-007.** This is the story's own fixture tier, and it is the reason the fixture exists: a procedure that has never returned `fail` is decorative. It is deliberately **not** the gate-fixture observation — breaking a real page and watching `cargo xtask ci` name the file and line is `declaration-check-seen-to-fail`'s whole story, and it cannot run before a gate step exists to be watched failing. |
| **procedural (reversibility)** | `git checkout -- standards/pages && git status` | **AC-010.** Nothing this PR adds leaves residue: no cache, no generated artifact, no dirty file. The same shape UX-008 specifies for the checker's own failure, applied to a prose-only diff. |

**Merge gate, one line.** `cargo xtask ci` green; `cargo xtask affected --base main` green
(widened); both `git diff` boundary checks empty; every mechanical measurement captured;
the calibration walk recorded at `fail — two needs` by a named non-author; the governed-set
sweep recorded as **vacuous**; the spot check recorded over `standards/pages/**` — all in
`_ledger.md` with cited evidence.

## Risks and coupling (PR-scoped)

| Risk | Coupling it runs through | Mitigation in this PR |
| --- | --- | --- |
| **The procedures are written, never executed, and the ledger records their existence rather than their result.** This is the initiative's own measured defect wearing a verdict table — an explanation that existed in three files, none of them where the reader was standing. | `_ledger.md` ← this story only; nothing re-runs either procedure, ever | AC-007 requires a **verdict**, a **date** and a **named non-author**; AC-006 requires the word *vacuous*; AC-008 requires a per-file record. NF-005 makes the identity and corpus mandatory, and EC-005 makes an author-run calibration inadmissible rather than merely weak. |
| **The discipline becomes a second specification** — the project's own Risks row 4, and the rule this story writes would be the first thing to fail it. | `spec/SPECIFICATION.md` ← `standards/pages/**`, by citation only | AC-008 runs the spot check over `standards/pages/**` *at this merge*, making this tree the rule's first corpus rather than an exempt one; EC-007 names the two concrete forms (restatement, and citation-by-line); NF-007 keeps `git diff main -- spec/` empty. |
| **The fixture becomes a permanently red page** the moment the checker story makes a walk recursive or pins a root above `standards/pages/`. | `page-need-checker-mounted-in-the-gate` ← this story, by **file placement**, not by an interface | EC-006 plus the forward obligation stated in "Data and migrations": that story must keep both walks non-recursive **or** exclude `standards/pages/examples/**` explicitly, and say which. Today's safety is mechanical and cited (`xtask/src/lint_constitution.rs:208-220`), not assumed. |
| **The two generated router rows disagree with what `--write` will emit**, so the checker's first run produces a surprise diff and that PR's reviewer cannot separate drift from intent. | `page-need-checker-mounted-in-the-gate` ← this story, by **value** (bytes, not an interface) | AC-010 pins the row format to `xtask/src/lint_constitution.rs:400-420` by line and requires a written derivation record in the ledger — the same obligation the dependency took for the region as a whole, extended to these two rows. |
| **Band `30`'s file name is invented here and depended on by value the moment the router links it.** `_design.md` names no file for that band. | `standards/pages/README.md`'s two regions; `governed-page-cites-the-discipline` later links into this tree | The name is pinned in the Context pack and in the PR boundary, typed once and copied; EC-011 records that changing the band namespace reopens a signed-off design plus a merged dependency, and is not an implementer's choice. The dependency's own clarification 4 already assigned band `30` to this story. |
| **The walk drifts into a checklist of adjectives** under review pressure — "is the page focused?" is shorter to write than a yes/no question with a consequence. | `_design.md` `### S5` and anti-pattern 15 | AC-004's hedging grep is scoped to the extracted walk region and returns **no output** or the story does not merge; the two-stranger comparison is what catches a step that is technically yes/no and practically an impression. |
| **`indeterminate` is used as a soft pass**, so a page that cannot be reviewed from itself quietly ships. | `_design.md` `## States` row S5, whose four verdicts this story implements over `## Composition`'s three | AC-005 requires the row to be defined as **a defect in the page, not in the procedure**, in the atom's own words, and greps for the sentence. The discrepancy between the two `_design.md` sections is recorded as its own `## Mock` finding 3 and is carried here as a decision, not resolved silently. |
| **The affected gate widens to the whole workspace and someone "fixes" it** by adding an `INERT` entry. | `xtask/src/affected.rs:249-266` | EC-010 puts `xtask/src/**` out of scope with `git diff main -- xtask` empty as a merge condition; the `INERT` entry lands **paired** with the checker's unconditional-list entry in the next slice, and the entry alone would make a prose-only PR read nothing — the half-mount `_storymap.md` names. |

## Dependencies

**Blocks on** (must be merged before this story's atoms can be mounted):

- **`router-precedence-and-announcement`** — the mount point itself. It stands up
  `standards/pages/`, authors `standards/pages/README.md` with the band table that reserves
  bands `30` and `40` for this story, the `## Start here` filter, the
  `<!-- BEGIN GENERATED -->` … `<!-- END GENERATED -->` region and its byte-exact row
  contract, the precedence block, the `## What checks this tree, and what does not` section
  and the `docs/README.md` announcement. Without it these two atoms have no composition
  root and would land half-mounted (EC-001), which is the failure
  `.bklg/docs-that-teach/page-need-discipline/_storymap.md`'s first "Why the slices fall
  here" bullet names. Its own spec records the hand-off in the other direction: its
  "Unlocks" list says band `30` is the one band `_design.md` names no file for, *"so that
  story inherits an owner rather than a gap"*.

Transitively, through that story: **`need-vocabulary-and-declaration-form`**, whose closed
`NEEDS` set the fixture's two declarations must be drawn from — a fixture declaring
unenumerated tokens would fail for the wrong reason and would not calibrate the walk
against the defect it is built for.

**Unlocks** (each `depends_on` this story, or inherits an obligation stated here):

- **`playbook-atom-staged-for-ingest`** — names this story in its `depends_on` per the story
  map. The playbook atom carries the method and the conditions under which the discipline
  stops holding; both procedures and both recorded runs are the material it distils, and an
  atom staged before the procedures had ever been executed would stage a claim rather than a
  practice.
- **`page-need-checker-mounted-in-the-gate`** — inherits three obligations from this PR,
  stated in "Data and migrations": the two generated rows must survive its first `--write`
  with **no diff**; its corpus walk must stay non-recursive **or** exclude
  `standards/pages/examples/**` explicitly and say which; and its module docs must name
  band `40`'s walk as the instrument for what the checker cannot see (RS-81-1; architecture
  brief AC-009 and Note 7 item 1).
- **HS-P0025 `durable-audience-closeout`** — re-observes DoD-8 and DoD-12 on the assembled
  tree. This story's two records are dated evidence over the corpus that existed, explicitly
  not a discharged claim over the narrative set (project `project.md`, Out of scope).

**Slice-mates** (implemented in one context, mounted as one surface — the
`discipline-on-disk` milestone): `need-vocabulary-and-declaration-form`,
`router-precedence-and-announcement`, `fold-line-rule`.

## Anchors (progressive disclosure)

Load-bearing depth, deferred rather than pasted. Each row says **why** it is load-bearing
and **when** to open it, and is bound to the AC it serves. Every path was confirmed to
exist in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | **Binding, signed off.** `### S5 — reviewer-procedure` fixes the walk's form and its three rejected alternatives; `## Composition` S5 fixes the region order; `## States` row S5 carries the **four** verdicts and the vacuous-sweep instruction; `## Transience policy` rows S3/S5 class the steps and table as persistent chrome and the worked example as a link; `## Density budget` carries the numbers; `## Anti-patterns` 13-15 are the named refusals; `## Mock` finding 3 records the three-vs-four discrepancy this story resolves. This story implements it and may not contradict it. | **Before writing the first line of band `40`**, and again before each measurement. | AC-003, AC-004, AC-005, AC-006, AC-009, AC-010 |
| `xtask/src/lint_constitution.rs` | The grammar this tree is measured by, in code: `SECTIONS` and the ceilings at `:67-95`, `check_shape` at `:477-509`, the rule split at `:262,275-279` with its test at `:870-877` (why `### The walk` is parser-safe), the non-recursive corpus reader at `:208-220` (why the fixture is invisible), the `Load when` first-line-only parser at `:247-255`, the generated-row bytes at `:400-420`, the link check at `:343-356`, and — the sentence band `30` mirrors — the module docs at `:10-28,20-21`. | **Before writing either atom head**, again before authoring the two router rows, and again before placing the fixture. | AC-002, AC-003, AC-007, AC-009, AC-010 |
| `standards/rust/README.md` | `:32-36` is the clause-or-atom test band `30` states one level out, including *"An atom never restates a clause's content"* — cite it, do not re-derive it. `:99-112` is the `Do`/`Not`/`Rejects.` grammar that forces a named wrong state instead of an impression, which is what makes a walk step falsifiable. | **Before writing band `30`'s first rule**, and before writing either `Rejects.`. | AC-001, AC-004, AC-009 |
| `standards/rust/00-prime-directives.md` | `:1-9` is the atom head grammar (`# NN — Title`, the one-line `> **Load when:**`, `> **See also:**`, `---`) and `:19,30,54,78` is the worked five-section rule shape both atoms copy. The reference implementation, not a description of one. | **When writing each atom head and its first rule.** | AC-009 |
| `standards/rust/81-checks-that-cannot-be-types.md` | `:11` is RS-81-1 — a check's blind spot must be stated in its own documentation, because *"a check whose limits are undocumented is read as a guarantee"*. `:209` is RS-81-3, which forbids the shared abstraction over two trees the implementer will be tempted to build. | **Before writing either blind-spot statement**, and the moment a "small helper" in `xtask/src/` starts to look reasonable. | AC-002, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/_decomposition.md` | The three briefs. **UX-009** (`:526-532`) is AC-007's test verbatim; **UX-010** (`:533-541`) is the visible-link-text half of band `30`; UX-003 and UX-011 are the never-occlude and burden-of-proof invariants. **Architecture** AC-009 (`:116-119`), AC-010 (`:120-129`), Note 4 (no `rust` fence), Note 6 (no shared abstraction), Note 7 items 1 and 4 (*"no byte count sees it"*). **Testing** AC-009 (`:913-923`), AC-010 (`:924-936`) and the fifth tier (`:785-791`). | **UX-009/UX-010 before writing the ACs' evidence; architecture Note 7 before the blind-spot statements; the Testing brief before filling the ledger.** | AC-001, AC-002, AC-007, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/router-precedence-and-announcement/spec.md` | The dependency's contract with this story: the band table assigning `30` and `40`, the generated-region row format its AC-004 pins by `diff`, the one-source-line `Load when` constraint its AC-005 states before these atoms are written, and its "Unlocks" paragraph naming what this story inherits. | **Before authoring the two router rows**, and if the band assignment is ever questioned. | AC-010, AC-009 |
| `.bklg/docs-that-teach/initiative.md` | `:344` is BR-09 (*"a page cites clauses and never restates them"*); `:395-397` is AC-12; `:441-444` is **DoD-8** — the phrase *"a check a reviewer can actually perform rather than one that depends on the author's memory"* is the whole reason the walk closes its permitted-sources list; `:455-457` is **DoD-12**. The gold source for what these two ACs are ultimately for. | **Before writing the performer/permitted-sources paragraph**, and when filling the ledger's trace. | AC-003, AC-008 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `:430-433` rules out an author-authored comprehension check as evidence — the reason this procedure yields a verdict on the page's **shape** and never a claim about reader understanding. `:436-443` is the do-not-add-an-affordance-the-medium-already-renders rule behind the link-not-a-fold choice. | **If a quiz, a rubric or a comprehension check starts to look like better evidence than a walk.** | AC-004, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | `:174-181` is this initiative's one **measured** defect: the `E0034` explanation that existed in three contributor-facing files, none of them the one the reader was looking at. A procedure nobody has executed is that defect wearing a verdict table — the argument for why AC-006 and AC-007 are runs, not statements. | **Before deciding how much of this story is prose and how much is evidence**, i.e. before concluding the atoms alone are the deliverable. | AC-006, AC-007 |
| `.bklg/docs-that-teach/page-need-discipline/design/mock.html` | The built contact sheet, self-contained and painted from rustdoc's own emitted stylesheet: the `reviewer-procedure` frames and the **four-row** verdict table it draws, plus density chips computed from each specimen's own bytes rather than asserted. It shows what `verdict-fail-two-needs` and `indeterminate` actually look like. | **When writing the verdict table**, and when a budget measurement comes out close to a ceiling. | AC-005, AC-009 |
| `.bklg/docs-that-teach/page-need-discipline/project.md` | The charter's own words for **AC-009** and **AC-010** — including the requirement that the full-set walk *finds no page carrying two needs* — DR-07 and DR-09, Risks row 4 (the discipline as a second specification), the Definition of done this story's gate commands come from, and the Out-of-scope line handing re-observation to HS-P0025. | **When filling the ledger**, to confirm each project AC is discharged by a story AC rather than mentioned. | AC-006, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/_grounding.md` | `:240-245` is DR-09's textual basis — why a citation is a stable **id** and not a line reference, and why that survives the sibling branch's divergence where a line would not. `:291-296` records that **no Accepted decision atom governs this project**, which is why the authority chain here runs through `_design.md` and the briefs rather than through `.kb/decisions/`. | **Before writing band `30`'s rule about what a citation *is***, and whenever an ADR seems to be missing. | AC-001 |
| `.bklg/docs-that-teach/page-need-discipline/_storymap.md` | The slice boundaries and the Coverage table: this story is the **sole owner** of project AC-009 and AC-010, the mechanical resolution half of AC-010 is HS-P0020's, and DR-07/DR-09 ride along here. Also the "Why the slices fall here" bullet on half-mounting. | **If a rule seems to belong to a slice-mate**, or before deciding whether something is this story's half of an AC. | AC-008, AC-010 |

## Clarifications resolved during spec

1. **The AC set is exactly the ten the front half enumerated** — AC-001 through AC-010,
   none added, none dropped. `_ledger.md` carries ten rows, one per id, each
   `satisfied: false` with empty evidence at planning time.
2. **The verdict table has four rows, and the two halves of `_design.md` that disagree are
   reconciled here rather than silently.** `## Composition` S5 says three
   (`pass` / `fail — two needs` / `fail — need not answered`); `## States` row S5 carries
   four, adding `indeterminate`; `## Mock` finding 3 records the discrepancy and states the
   mock drew four *because the States block is its addressing manifest*. This story
   implements four and AC-005 greps for all four literals. Dropping `indeterminate` would
   leave a reviewer who cannot complete the walk with only a soft `pass` available, which
   is the outcome the States block exists to forbid.
3. **The static test tier is empty in this PR, and that is a decision rather than an
   omission.** The testing brief specifies one static assertion for AC-010 at the project
   grain — a structural-presence test that the rule text states "cite, never restate"
   (`_decomposition.md:924-928`). It is a `#[cfg(test)]` test inside a module that does not
   exist until `page-need-checker-mounted-in-the-gate`, and this story's PR boundary makes
   `git diff main -- xtask` empty a merge condition. Resolution: the assertion is named in
   the Tests and CI table's `static` row as an obligation that story inherits, and its
   present-tense verification is AC-001's `rg`. This is exactly the shape the *procedural
   (ledger-recorded)* tier was created for, plus a stated forward obligation — not an
   unverified AC.
4. **Band `30`'s file name is `standards/pages/30-citing-the-specification.md`, pinned by
   this story.** `_design.md` addresses band `40` directly and names **no** file for band
   `30`; the dependency's spec assigned the band to this story and its own clarification 4
   records that the fold-into-band-`20` alternative lost because the fold line and the
   citation rule are applied by different readers at different moments. Pinning the name
   here is depended on by value the moment the router links it (EC-011).
5. **The fixture is a new artifact this spec introduces, and its three properties are
   decisions.** `standards/pages/examples/two-needs.md` did not exist in any brief. It
   exists because UX-009's test is literal — *a person who did not write the page runs it
   over a page carrying two needs* — and no such page exists in this worktree. It is
   deliberately **not** a governed page (a permanently broken page inside the checked
   surface would make the gate red forever, and breaking a real page once with a revert is
   `declaration-check-seen-to-fail`'s story), **not** an atom (the corpus reader is
   non-recursive, `xtask/src/lint_constitution.rs:208-220`), and **linked, not inlined**
   (`_design.md` `## Transience policy`, and a link is the one opened-on-demand mechanism
   UX-011 does not require this repository to verify).
6. **"Run each once over the set as it stands" resolves to three recorded runs, not two,
   because the set is empty.** The one-line slice says "the set as it stands"; as it stands,
   the governed set is empty. So: the governed-set walk is recorded as **vacuous** (AC-006,
   the `_design.md` States-block behaviour); the **calibration** walk against the fixture is
   what produces a real verdict (AC-007); and the paraphrase spot check's corpus at this
   merge is `standards/pages/**` — this discipline's own tree, the only prose in the diff
   that makes normative claims (AC-008). Each record names its corpus (NF-005) so a later
   reader cannot mistake any of the three for a pass over the narrative set, which HS-P0025
   re-observes.
7. **The walk lives under a `### The walk` heading inside `## RP-40-1.`'s `Do` section, not
   as its own `##` rule.** `_design.md`'s `## Surfaces` addresses the procedure at
   `standards/pages/40-reviewing-a-page.md#the-walk`, and only a heading generates that
   anchor. Promoting it to `##` would split the rule at `rules()`' boundary
   (`xtask/src/lint_constitution.rs:262,275-279`) and strand `## RP-40-1`'s `Rejects.`;
   leaving it as a bold run-in would make the signed-off design's own addressing manifest a
   dead link (EC-002). `###` is parser-safe, proven by
   `rules_are_split_at_the_next_heading` (`:870-877`).
8. **AC-004 requires two walkers, not one.** UX-009's test names a single non-author; the
   two-stranger comparison is this spec's addition, and its justification is that a single
   walker cannot distinguish "the step is answerable" from "this particular person found it
   answerable". The comparison is cheap — the same fixture, the same steps — and a
   divergence on any step is recorded as a defect in the step. AC-007's calibration walk may
   be one of the two.
