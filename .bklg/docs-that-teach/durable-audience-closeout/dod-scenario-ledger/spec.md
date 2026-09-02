---
item: HS-S0180
stage: spec
created: 2026-08-17T13:16:30.285Z
updated: 2026-08-17T13:16:30.285Z
template_sig: 87bbf1d0
rendered_sig: 412630be
---

# Spec — Re-observe the fifteen DoD scenarios from a clean checkout

## Scope lock

| Layer | Path | What it fixes for this story |
| --- | --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` | The fifteen Definition-of-Done scenarios themselves (`:413-468`), the preamble that makes them *run and observed* rather than argued (`:406-411`), and the exit criterion requiring every one observed on the assembled result from a clean checkout (`:655-657`) |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` | Which project *makes* each scenario true (`:219-235`) and the sentence that turns that into this project's separate obligation: all fifteen are additionally re-observed by HS-P0025 on the assembled result, "a verification obligation, not a second ownership" (`:236-239`) |
| Project (this project's mandate) | `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-014 (`:203`), AC-012 (`:201`), DR-11 (`:169-171`), the boundary-level DoD requiring each ledger entry to cite the project that made it true (`:219-220`), and the named risk this story exists to defeat — "fifteen re-observations become fifteen ticked boxes" (`:249`) |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/spec.md` | The story an implementer loads |
| Key briefs | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | UX brief: the two states a DoD row may express (`:71-72`), AC-UX-09/10/11/12 (`:266-281`), the accessibility floor for markdown (`:115-145`), the primitive inventory and the hand-rolling ban (`:74-107`); Architecture brief: AC-A06 (`:383-387`), AC-A07 (`:388-391`), AC-A08 (`:392-395`), ordering 4 (`:638-640`), the mutation-must-not-survive rule (`:642-647`), the composition-root/mount analogue (`:429-436`); Testing brief: the four tiers (`:704-711`), the clean-checkout seam and the fault-injection seam (`:789-802`), AC-TB-04/05/08 (`:833-839`, `:846-851`), AC-014's tier row (`:874`) |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | `## Items` is empty by decision — no `pub` item, no rendered surface (`:10-20`, `:45-47`); sign-off recorded `:91-93`. The DoD re-observation ledger is named there as one of the project's four user-facing artefacts (`:26-35`) |
| Grounding | `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | The clean-checkout tension stated before planning began: this worktree cannot satisfy AC-014 and cannot be made to (`:148-155`) |
| Story map / roadmap pointer | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | This story's row (`:56`), backbone activity A4 (`:35`), why `dod-reobservation` is one surface (`:76-78`), the AC-014 ownership seam against `scenario-two-fault-injection` (`:127`), the per-story-ledger-versus-re-observation-ledger distinction (`:94-98`), and merge order 4 (`:156-160`) |
| Upstream evidence this story consumes | `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/spec.md`, `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/` | The two `blocked_by` edges, each existing so one scenario row has something fresh to cite: scenario 11 and scenario 15 |

## One-line PR slice

Create a fresh checkout off the merged branch and re-observe all fifteen initiative DoD
scenarios there, recording per scenario the observer, the checkout path and sha, and what
was seen — with "inherited from a sibling" inadmissible as an outcome.

## Executive summary

Fourteen of the fifteen Definition-of-Done scenarios were *made true* by a sibling project
(`.bklg/docs-that-teach/_decomposition.md:219-235`). This PR lands the artefact that says
they are true **on the assembled tree** — one ledger, fifteen rows, each naming a human, a
checkout, a sha, an instrument and what came back.

The delta over what already exists is precisely the word *fresh*. Every scenario already has
an owner who proved it once, in its own project, on its own branch, with its own gate run.
None of that is evidence about the merged tree, which is why the initiative made this a
separate obligation rather than a roll-up: *"All fifteen are additionally re-observed by
HS-P0025 as terminal DoD owner, on the assembled result from a clean checkout. That is a
verification obligation, not a second ownership"* (`:236-239`). So this story adds no
capability, changes no page, compiles no Rust, and fixes no defect. It adds the one thing no
sibling could produce and no command emits: a written observation taken somewhere the
building did not happen.

Three things make that harder than a checklist, and they are the reason this spec is long.
**The checkout is a fixture, not a location** — it must be a fresh `git clone` or a new
`git worktree add` off the *merged* branch, never `.claude/worktrees/docs-that-teach`
(`_decomposition.md:388-391`, `:789-794`; `_grounding.md:148-155`), and it only ever contains
*committed* state, which fixes when it may be taken. **Three scenarios are mutation-shaped,
not read-shaped** — 2, 4 and 13 are only honestly re-observed by breaking something and
watching a check go red; 2 belongs to the slice-mate, 4 and 13 belong here, and none of the
three edits may ever be a committed state of the tree (`:642-647`, `:795-802`). And **seven
scenarios have no command at all** — 5, 6, 8, 9, 10, 12 and 14 are walks and reads, which is
exactly where the story's named wrong implementation lives: *a ledger asserting fifteen
scenarios pass, with evidence for the cheap ones only* (`discover.md:55`).

## Context pack

Twelve decisions. They are decisions, not references; the deeper artefacts sit behind the
anchors the second pass enumerates.

**1. The checkout is the fixture, and this worktree is disqualified by construction.** AC-014
requires the observation to run on a fresh clone or a new `git worktree add` off the **merged**
branch, and the ledger names that checkout's path and its sha alongside every scenario
(`project.md:203`; AC-A07, `_decomposition.md:388-391`). The tension was recorded before
planning began — the worktree this planning ran in "does not satisfy it and cannot be made to"
(`_grounding.md:148-155`; `_decomposition.md:306-310`, `:899-902`). Treat *which checkout* as
the fixture the testing brief says it is (`:789-794`): Tier 4 does not start until it exists.
Convenient and verified: `.gitignore`'s final entry ignores `.claude/` wholesale, with the
reason written out — worktrees are "full checkouts of this repository, each with its own
`target/`" — so a sibling worktree created under `.claude/worktrees/` cannot pollute this PR's
diff.

**2. A fresh checkout sees only committed state, which fixes when it may be taken.** This is
the consequence implementers miss and it is why both `blocked_by` edges exist. The atoms, the
map appends, `links.kb`, the reconciliation record, the DT audit and the clause-completeness
statement must be **committed on the merged branch** before the checkout is created, or the
checkout observes their absence and the ledger records fifteen honest failures. Ordering 4 —
everything committed before the final gate (`_decomposition.md:638-640`) — has a precondition
one step earlier: everything committed before the *observation*.

**3. The observation happens in the clean checkout; the ledger is written in the initiative
worktree.** They are two trees and the distinction is load-bearing. The clean checkout is
read-only apart from the deliberate, reverted mutations of decision 6; nothing this PR ships
is authored inside it, because a file written there is a file this pull request does not
carry. The ledger, its companion and this story's `_ledger.md` are written in the initiative
worktree, under this story's own directory.

**4. One checkout, one sha, fifteen rows — and if that breaks, the ledger says so per row.**
The cheapest correct shape is a single checkout taken once, with every row naming the same
sha. If a re-observation forces a second checkout — a scenario had to be re-run after a fix,
or a slice-mate committed in between — the ledger names *both* checkouts and every row
carries the sha it was actually observed on. It never carries a sha it was not observed on,
and it never leaves the reader to infer one from the row above (AC-UX-10,
`_decomposition.md:270-273`; AC-UX-12, `:278-281`).

**5. "Inherited from a sibling" is not an outcome, and the citation of the owner is a
*separate* obligation from the observation.** AC-014 forbids the phrase as an evidence value
(`project.md:203`), AC-UX-11 makes its presence a review-blocking defect
(`_decomposition.md:274-277`), and AC-TB-04 asks that the string appear nowhere in the ledger
(`:833-836`). But the project's boundary-level DoD *also* requires each entry to "cite the
project that made it true" (`project.md:219-220`). Those pull in opposite directions only if
the owner citation is allowed to stand in for the observation. It cannot: every row carries
**both** — the owning project (provenance, so nobody re-opens a sibling's requirement here)
and a fresh observation with its own instrument, its own artefact and what was seen.

**6. Three scenarios are mutation-shaped, and two of them are this story's.** Scenario 2 (break
a page, gate fails by name, revert, green) belongs to the slice-mate `scenario-two-fault-injection`
(`_storymap.md:57`, `:127`). Scenario 4 — *"removing the boundary from the example makes it
fail"* (`initiative.md:425-428`) — and scenario 13 — *"by breaking a hidden claim and observing
the gate fail"* (`:458-461`) — are this story's, and they are the same shape: a live edit, a red
check captured verbatim, a revert, a green re-run. The architecture brief's rule binds all
three identically — the broken edit "is never a committed state of the tree the final
`cargo xtask ci` runs on" (`_decomposition.md:642-647`, `:795-802`). Scenario 13 has a second
admissible arm the initiative itself names — *"or no such content carries a load-bearing
claim"* — and that arm must be **shown** (the folded/tabbed/collapsed content enumerated, each
one's claims read) rather than asserted, because an unshown disjunct is the ticked box the
project's risk table names.

**7. Seven scenarios have no command, and that is where the defect lives.** Scenarios 5, 6, 8,
9, 10, 12 and 14 are artefact reads and reader walks. The story's own named wrong
implementation is *"a ledger asserting fifteen scenarios pass, with evidence for the cheap
ones only"* (`discover.md:55`), and it is the project risk *"fifteen re-observations become
fifteen ticked boxes"* (`project.md:249`) at story grain. The counterweight is structural,
not exhortative: every row names the **instrument** it used, and every non-command row cites a
real artefact at a real `file:line` or records the walk hop by hop. A row whose evidence is a
sentence of confidence is not a row.

**8. Scenario 11 is why `post-merge-clause-completeness` blocks this story.** That story
produces a written completeness statement over the frozen documentation MUSTs on the merged
tree, and its own spec says it is written "so a single row of that ledger can cite it by path
and line without paraphrasing it"
(`post-merge-clause-completeness/spec.md`, `## Integration contract`, and its AC-006). Scenario
11's row therefore cites that statement at a real `file:line` — it does not re-derive the
clause-id set, does not re-run the comparison, and does not paraphrase the verdict. Citing a
sibling *record* that was itself a fresh observation on this tree is not "inheriting from a
sibling"; inheriting would be citing HS-P0020's original proof on its own branch.

**9. Scenario 15 is why `product-layer-mounting` blocks this story.** DoD-15 requires persona
and journey atoms under `.kb/product/` with valid frontmatter, passing `redkiln validate --kb`,
reconciled rather than duplicated, **and linked from this initiative's closeout**
(`initiative.md:465-468`). Every clause of that is only observable after the
`product-layer-promotion` milestone has landed: the atoms come from `audience-ingest-wave`, and
`product-layer-mounting` lands all four mount points including `links.kb` on HS-P0025 and the
`## Knowledge Harvest` row (AC-A04, `_decomposition.md:372-378`). Scenario 15's row is the one
row where this project observes its own output, so it is the row most at risk of being written
from memory rather than from the checkout. Run `redkiln validate --kb` **in the clean
checkout** and record its exit and output there.

**10. Scenario 1 is a gate run, and it is not `terminal-gate-run`'s gate run.** DoD-1 is *"from
a fresh clone with no local state, the full gate runs green and the narrative material builds
and renders as part of it, not as a separate manual step"* (`initiative.md:413-415`). That is
two claims: the gate is green in the fresh checkout, **and** the narrative build is a step
inside it. The second is answered by locating the step in `xtask/src/main.rs`'s `REQUIRED` list
(`:105`) on the merged tree and citing it at the line it occupies — located by search and the
search recorded, never cited from a plan, because HS-P0020's landing shape is not visible from
this worktree. The gate run this story takes is an *observation in the fixture*;
`terminal-gate-run` later takes a different run, last, on the final tree carrying this ledger
(`_storymap.md:156-160`; AC-016, `project.md:205`). Two runs, two purposes; conflating them
produces a gate run that proves the gate, not the deliverable.

**11. The fifteen-scenario ledger is a companion file; `_ledger.md` is this story's per-AC
ledger.** The architecture brief left the shape open and told the story map to settle it
(`_decomposition.md:669-674`), and the story map settled it: the per-story `_ledger.md` that
`require_ledger: true` mandates (`.redkiln/config.yaml:67`) "is distinct from
`dod-scenario-ledger`'s fifteen-scenario re-observation ledger" (`_storymap.md:94-98`; testing
brief, `_decomposition.md:768-774`). So: the fifteen rows live in a companion under this
story's own directory (working name `_dod-reobservation.md`; the name is the implementer's, the
directory is not), and `_ledger.md` carries one row per `AC-###` in this spec, each citing the
companion at a real `file:line`. That citation is the mount — a record in a story directory
that nothing cites is this project's analogue of a component rendered into no tree
(`:429-436`).

**12. This is markdown with no automated reader, so the surface obligations bind as written.**
Every state is a literal **word**, never a tick, an emoji, a colour, a strikethrough, an
ordering or an empty cell (AC-UX-09, `_decomposition.md:266-269`, `:120-130`); every row is
self-contained and loses nothing read alone (AC-UX-10, `:270-273`); the load-bearing verdict is
also a **sentence**, because a verdict that exists only in a table cell is lost in a quote or a
diff (`:142-145`); every link names its destination, never "here" or "see above"
(`:133-137`). The corpus precedent is the open-questions index's literal status words
(`.kb/maps/open-questions-index.md:136-143`). Nothing here is stylistic: `redkiln validate --kb`
does not read this file, so a reviewer is the only reader it has.

### The fifteen, their owners, and the instrument each re-observation demands

The scenario texts are `initiative.md:413-468`; the owners are
`.bklg/docs-that-teach/_decomposition.md:219-235`. The instrument column is this spec's
decision, and it exists so that a cheap row cannot be written where an expensive one is owed.

| # | Scenario (abbreviated) | Owner | Instrument this re-observation must use | Row owner |
| --- | --- | --- | --- | --- |
| 1 | Clean checkout; gate green; narrative builds and renders as part of it | HS-P0020 | Gate run in the fixture, plus the narrative step located in `xtask/src/main.rs`'s `REQUIRED` list (`:105`) and cited at its landed line | this story |
| 2 | A deliberately broken page fails the gate by name, then recovers | HS-P0020 | Fault injection, two halves, failing transcript captured verbatim | `scenario-two-fault-injection` |
| 3 | The opening encounter runs and demonstrates a boundary | HS-P0022 | Run the path end to end; the refusal is visible in program output, not only in the page | this story |
| 4 | The boundary claim is checked, not narrated | HS-P0022 | **Mutation**: remove the boundary from the example, capture the red check, revert, re-run green | this story |
| 5 | A non-author, non-insider completes a stated scenario; auditable log | HS-P0024 | Read the dated log in the fixture against its five required shape elements | this story |
| 6 | Every stumble in that log has a disposition | HS-P0024 | Walk every marked item to a fix, a recorded acceptance or a routed item; cite each | this story |
| 7 | The reader reaches the teaching from the front door | HS-P0023 | Walk from what a developer sees after installing the crate; record the hops | this story |
| 8 | Every page's answered need is stated and singular | HS-P0021 | Walk the full page set; per page, the named need; the review pass finding no page with two | this story |
| 9 | The evaluator's second question is walked | HS-P0023 | Follow at least two plausible second questions to answering pages; no dead ends | this story |
| 10 | The adapter author's error meets its explanation | HS-P0023 | Reproduce the trait-resolution failure; reach the explanation from the file and message | this story |
| 11 | The frozen documentation MUSTs are still discharged | HS-P0020 | Cite `post-merge-clause-completeness`'s statement at a real `file:line`; do not re-derive | this story |
| 12 | No page has become a second specification | HS-P0021 | Spot check: each normative claim is a citation that resolves and defers to the clause | this story |
| 13 | Nothing load-bearing is hidden from the check | HS-P0020 | **Mutation**, or the disjunct *shown*: the folded content enumerated and its claims read | this story |
| 14 | The discipline is on disk and cited | HS-P0021 | Locate it in its decided home; follow the reachability link; find the page that cites it | this story |
| 15 | The audience is durable and reconciled | HS-P0025 | `redkiln validate --kb` run **in the fixture**; atoms, reconciliation and closeout link read there | this story |

## Integration contract

- **Archetype**: `capability` (`story.md` frontmatter, `archetype: capability`;
  `_storymap.md:56`). Its observable output is a written record a human reads, not a `pub`
  item — which is what `_design.md` means by declaring no items at all (`:45-47`).
- **Slice / milestone**: **`dod-reobservation`**. Slice-mates, implemented in the same context
  and mounted as one surface: **`scenario-two-fault-injection`** (second) and
  **`terminal-gate-run`** (last). This story is first in the milestone
  (`_storymap.md:156-160`). The three are one surface because they all run *in the same fresh
  checkout*, and because the fault injection is a live mutation of that checkout "whose revert
  the terminal gate run proves" (`:76-78`).
- **Mount point**:
  **`.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/_ledger.md`** — this
  story's own evidence ledger, mandatory for any story declaring `AC-###`
  (`require_ledger: true`, `.redkiln/config.yaml:67`) and read by `redkiln verify --grain
  story`. The fifteen-scenario re-observation ledger itself is a companion in this same
  directory (working name `_dod-reobservation.md`), and `_ledger.md` is the file that cites it
  per criterion with a real `file:line`. That citation is the mount: a ledger written into a
  story folder and referenced by nothing is this project's analogue of a component rendered
  into no tree (`_decomposition.md:429-436`).
  **Downstream mounts, load-bearing and named here so they cannot be missed**: the slice-mate
  `scenario-two-fault-injection` writes scenario 2's two half-rows **into this companion**, so
  its row shape must be fixed and its scenario-2 slots must exist and be visibly awaiting their
  evidence rather than silently absent; and `terminal-gate-run` takes `cargo xtask ci` on the
  tree that already carries this file (`_storymap.md:58`, `:156-160`).
- **Wires into** (all read-only unless stated):
  - **The merged commit** produced by `merge-forward-baseline` and consumed as a value: this
    story runs no `git merge` and derives no sha of its own (AC-A06,
    `_decomposition.md:383-387`).
  - **The clean checkout**, created by `git clone` or `git worktree add` off that merged
    branch — the fixture the testing brief names (`:789-794`), whose path and sha every row
    carries.
  - **`post-merge-clause-completeness`'s completeness statement**, cited by scenario 11's row
    at a real `file:line`
    (`.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/`).
  - **`product-layer-mounting`'s four mount points** — the appended `##` section in
    `.kb/maps/domain-map.md`, the reciprocal `related` / `depends_on` edges, `links.kb` on
    HS-P0025, and the `## Knowledge Harvest` row — read in the fixture as scenario 15's
    evidence (AC-A04, `_decomposition.md:372-378`).
  - **`redkiln validate --kb`** (`.redkiln/config.yaml`, and CLAUDE.md's "Commands"), run in the
    fixture for scenario 15; **`cargo xtask ci`** (`.redkiln/config.yaml:60`), run in the
    fixture for scenario 1; **`cargo xtask affected --base main`** (`:40`), the story-grain
    check this prose-only diff still runs.
  - **`xtask/src/main.rs`'s `REQUIRED` step list** (`:105`) — read, to locate the narrative
    build step scenario 1 requires to be *inside* the gate. Never modified: no step, subcommand
    or `REQUIRED` entry is added.
  - No workspace crate, no port, no `Send` bound, no feature, no dependency. ADR-0001 and
    ADR-0003 are untouched by construction, and nothing is compiled that was not compiled
    before.
- **Renders surfaces**: **none.** `_design.md` records `# no items — no public API surface, no
  rendered UI surface` (`:45-47`). This story produces one instance of the project's fourth
  named artefact class — "a DoD re-observation ledger plus the DT-1…DT-10 audit table"
  (`:26-35`) — specifically the ledger half; the DT audit is `design-tension-audit`'s. No
  signed-off surface is changed. "No surface" is not "no obligations": the UX brief's
  accessibility floor and interaction-quality invariants bind this markdown as written
  (context decision 12).
- **Public items**: none. No `pub` item is added, changed or removed, and no row of
  `_design.md`'s `## Items` block is claimed, because that block is empty by decision
  (`:10-20`, `:45-47`).
- **Conformance rule(s)**: **none, and this is not adapter-observable.** Nothing here touches
  `crates/happenstance-testkit/`, a port, a value type or a fixture. Stated explicitly because
  a story that changes a port and names no rule is a port change nothing can fail — this
  changes no port and compiles no Rust (AC-TB-08, `_decomposition.md:846-851`). The instruments
  are the already-wired gate, the siblings' own checks, and a human running fifteen
  observations.
- **Clause(s)**: **none discharged, none amended.** `spec/SPECIFICATION.md` is read-only in this
  project's diff surface (`_decomposition.md:422-427`) and the initiative "discharges no clause
  and amends none" (`initiative.md:689-690`). No `[FROZEN]` clause is changed, so no ADR is
  owed. Scenario 11's row *cites* a statement about clause ids; it does not touch one.
- **Advances DoD scenario**: **all fifteen** — this is the story that moves the initiative's
  entire Definition of Done from "made true by a sibling" to "observed on the assembled
  result" (`.bklg/docs-that-teach/_decomposition.md:236-239`), and it is the artefact the
  initiative's exit criterion demands (`initiative.md:655-657`). It owns fourteen rows and the
  ledger's shape outright; scenario 2's two half-rows are its slice-mate's, filled into the
  shape this story fixes (`_storymap.md:127`).

## PR boundary

The paths this story is allowed to touch, as globs. `redkiln verify --grain story` reads the
first fenced block under this heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**
```

One glob, and it is honestly narrow rather than narrowed to look good: this story observes
fifteen scenarios and writes them down. The clean checkout it observes in is a separate tree
(and, if placed under `.claude/`, is ignored outright — `.gitignore`'s last entry); the
mutations of scenarios 4 and 13 happen there and are reverted there, never committed anywhere.

**In this PR**

- The fifteen-scenario re-observation ledger, a companion under this story's directory: the
  checkout's path and sha named once at the top and again on every row, and one self-contained
  row per scenario carrying its id, the abbreviated scenario, the owning project, the observer,
  the instrument, what was seen, and the outcome as a literal word.
- Scenario 2's two half-row slots, named and visibly awaiting the slice-mate's evidence, so the
  shape is fixed by this story and filled by `scenario-two-fault-injection`.
- The verbatim outputs the command-shaped scenarios produced — `cargo xtask ci` in the fixture
  (scenario 1), `redkiln validate --kb` in the fixture (scenario 15), the red and green runs of
  the scenario 4 mutation, and of scenario 13's if its mutation arm is taken — recorded as
  transcripts rather than summarised as "green" or "failed".
- The hop-by-hop record of the walk-shaped scenarios (7, 9, 10, 14) and the artefact citations
  of the read-shaped ones (5, 6, 8, 12), each landing on a real `file:line` in the fixture.
- Where the narrative build step was found in `xtask/src/main.rs`'s `REQUIRED` list, and by what
  search, so a later reader can repeat the location step without knowing HS-P0020's internals.
- The verdict, stated as a sentence as well as carried by the table.
- This story's `_ledger.md` (second pass) and its implementation report.

**Explicitly not in this PR**

- **Scenario 2's two rows and its fault injection** — `scenario-two-fault-injection`
  (`_storymap.md:57`, `:127`). This story fixes the shape those rows go in; it does not break
  the page.
- **The terminal `cargo xtask ci` run on the final tree** — `terminal-gate-run`
  (`:58`, `:156-160`). Scenario 1's gate run in the fixture is a different run for a different
  purpose and is not offered as AC-016's evidence.
- **Any fix to anything a scenario reveals.** A failed re-observation is a *recorded finding
  with a named owner*, routed to the owning project or to the `support` initiative
  (`.redkiln/config.yaml:5`). Re-implementing a sibling's requirement here is forbidden by the
  project's own DoD (`project.md:219-220`).
- **Any commit containing a mutation.** Scenarios 4 and 13's edits are reverted in the fixture
  before anything is committed anywhere (`_decomposition.md:642-647`, `:795-802`).
- **Any edit to `crates/`, `spec/`, `xtask/`, `standards/` or `docs/`** — read-only in this
  project's diff surface (`:410-427`). Locating a step is a read.
- **Any `.kb/` write** — no atom, no map append, no `_intake/` staging. Those belong to the
  `product-layer-promotion` milestone (`_storymap.md:53-55`), and `.kb/decisions/` is untouched
  by anyone in this project (AC-A09, `_decomposition.md:396-400`).
- **The DT-1…DT-10 audit table** (`design-tension-audit`), **the reconciliation record**
  (`reconciliation-ledger`) and **the clause-completeness statement**
  (`post-merge-clause-completeness`). This story cites the last of these; it authors none of
  them.
- **An ADR**, in any form, as a by-product of a finding.

The implementer MAY touch the wiring named in the Integration contract to mount this slice.
Here that wiring is this story's own `_ledger.md` and the companion it cites; both are inside
the single glob above, and citing the companion from the ledger is the point of the story
rather than scope drift.

**Merge DoD**: a clean checkout exists off the merged branch and is not this worktree; its path
and sha are named; fifteen scenario entries exist, fourteen of them carrying a fresh observation
with a named observer, a named instrument and cited evidence, and scenario 2's two slots present
for its slice-mate; the string "inherited from a sibling" appears nowhere; scenarios 4 and 13
carry either a captured red-then-green transcript or, for 13, a shown disjunct; no mutation is
present in any commit; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is green on
the prose-only diff; and no file outside
`.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**` is changed.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The observation runs in a fresh checkout off the merged branch | A `git clone` or a new `git worktree add` off the merged branch, at a path that is not `.claude/worktrees/docs-that-teach`. The ledger names the path and the sha; nothing is observed in the tree the planning and authoring happened in. A checkout placed under `.claude/` cannot pollute the diff. | `project.md:203`; `_decomposition.md:388-391` (AC-A07), `:789-794`, `:306-310`, `:899-902`; `_grounding.md:148-155`; `.gitignore` (`.claude/` entry) |
| The checkout is taken only after every observed artefact is committed | A fresh checkout carries committed state only. The atoms, the map appends, `links.kb`, the reconciliation record, the DT audit and the clause-completeness statement are on the branch before the checkout is created; otherwise the fixture observes their absence. | `_decomposition.md:638-640` (ordering 4), `:621-624` (data flow); `_storymap.md:156-160` |
| Observation happens in the fixture; authoring happens in the initiative worktree | The ledger, its companion and `_ledger.md` are written under this story's directory in the initiative worktree. Nothing this PR ships is authored inside the clean checkout, which is read-only apart from the reverted mutations below. | `_decomposition.md:789-794`; this spec's PR boundary |
| Every row names its own checkout and sha | One checkout and one sha is the expected shape. Where a second checkout was needed, both are named and each row carries the sha it was actually observed on — never one inferred from the row above. | `project.md:201` (AC-012); `_decomposition.md:278-281` (AC-UX-12), `:270-273` (AC-UX-10) |
| Every row carries both a fresh observation and the owning project | The owner is provenance and stops a sibling's requirement being re-opened here; the observation is the deliverable. Neither substitutes for the other, and "inherited from a sibling" is not an admissible outcome value anywhere. | `project.md:203`, `:219-220`; `_decomposition.md:71-72`, `:274-277` (AC-UX-11), `:833-836` (AC-TB-04) |
| Every row names the instrument it used | Command run, mutation, artefact read or reader walk — stated per row, so a read-shaped scenario cannot be discharged by a command-shaped sentence and a walk cannot be discharged by an assertion. This is the structural counterweight to "evidence for the cheap ones only". | `discover.md:55`; `project.md:249`; the instrument table in this spec's Context pack |
| Scenario 1 is a gate run in the fixture **plus** a located narrative step | DoD-1 makes two claims: the gate is green from a fresh checkout, and the narrative build is a step inside it rather than a separate manual one. The second is answered by locating the step in `xtask/src/main.rs`'s `REQUIRED` list on the merged tree, citing it at its landed line, and recording the search. This run is not `terminal-gate-run`'s run. | `initiative.md:413-415`; `xtask/src/main.rs:105`; `_storymap.md:156-160`; `.redkiln/config.yaml:60` |
| Scenarios 4 and 13 are mutations, captured red then green, never committed | Remove the boundary from the example (4) or break a hidden claim (13); capture the failing output verbatim; revert; re-run green. The edit is never a committed state of the tree — the same discipline the architecture brief fixes for scenario 2. | `initiative.md:425-428`, `:458-461`; `_decomposition.md:642-647`, `:795-802`, `:837-839` (AC-TB-05) |
| Scenario 13's non-mutation arm must be shown, not asserted | The initiative admits "or no such content carries a load-bearing claim". Taking that arm means enumerating the folded, tabbed or collapsed content in the shipped material and recording that its claims are not load-bearing — an enumeration, not a sentence of confidence. | `initiative.md:458-461`; `project.md:249` |
| Scenario 11 cites the sibling record; it does not re-derive the clause set | `post-merge-clause-completeness` writes its statement to be citable whole. Scenario 11's row cites it at a real `file:line`. No second enumeration of the frozen documentation MUSTs is produced here, in code or in prose. | `initiative.md:452-454`; `post-merge-clause-completeness/spec.md` (`## Integration contract`, AC-006); `project.md:102-105` |
| Scenario 15 is observed in the fixture, including the closeout link | Atoms present with valid frontmatter, `redkiln validate --kb` run **in the clean checkout** with its output recorded, reconciliation stated rather than duplicated, and the closeout link present — the four mount points `product-layer-mounting` lands, read where they landed. | `initiative.md:465-468`; `_decomposition.md:372-378` (AC-A04); `.redkiln/config.yaml:67` |
| A failed re-observation is recorded and routed, never fixed here | The ledger is allowed to report red. A finding names its owning project or routes to the `support` initiative with a reason; re-implementing a sibling's requirement inside this closeout is forbidden by the project's boundary-level DoD. | `project.md:219-220`; `.redkiln/config.yaml:5`; `_decomposition.md:422-427` |
| Scenario 2's slots exist and are visibly awaiting the slice-mate | This story fixes the ledger's row shape and leaves scenario 2's two named halves — `failed by name` and `recovered` — present and explicitly pending, so their absence can never read as an oversight or as a pass. | `_storymap.md:127`, `:57`; `_decomposition.md:72`, `:274-277` |
| Every row is self-contained and every state is a word | Each row names its scenario id, owner, observer, checkout, sha, instrument, what was seen and its outcome, and loses nothing read alone. No tick, emoji, colour, glyph, strikethrough, ordering or empty cell carries meaning; the corpus precedent is the open-questions index's literal status words. | `_decomposition.md:266-273` (AC-UX-09, AC-UX-10), `:120-130`, `:138-141`; `.kb/maps/open-questions-index.md:136-143` |
| The verdict is also a sentence | Whether the assembled tree meets the initiative's Definition of Done is stated in prose as well as in the table, because a verdict that exists only in a cell is lost in a quote or a diff. | `_decomposition.md:142-145` |
| The ledger is composed from the corpus's own primitives | One `#`, sections at `##`, no skipped level, links naming their destinations, transcripts in fenced blocks under their own heading rather than interleaved with verdicts. No bespoke per-artefact vocabulary, no invented status glyphs. | `_decomposition.md:74-107`, `:133-137` |
| The companion is reachable from `_ledger.md` and citable downstream | `_ledger.md` cites the companion per criterion with a real `file:line`, and the companion is written so `terminal-gate-run` and the closeout can cite its rows without paraphrase. | `.redkiln/config.yaml:67`; `_storymap.md:94-98`; `_decomposition.md:429-436` |
| Nothing this story writes moves an anchor someone else cited | The diff adds files under this story's directory. No pre-existing section is reflowed, no id renamed, no cited `file:line` shifted. | `_decomposition.md:180-187` (IQ-3) |

## Data and migrations

**N/A, in the strict sense: no schema, no store, no persisted state, no serialised envelope, no
released artifact, and no `.kb/` frontmatter.** This story compiles no Rust — `_design.md`
records that this project changes no public API and adds no `pub` item (`:10-20`, `:45-47`) — so
there is no semver promise and nothing in anyone's dependency graph to migrate. It reads
`.kb/product/` as scenario 15's evidence and writes nothing there; every `.kb/` write in this
project belongs to the `product-layer-promotion` milestone (`_storymap.md:53-55`), and
`.kb/decisions/` is untouched by anyone here (AC-A09, `_decomposition.md:396-400`).

Three adjacent facts are worth stating so they are not mistaken for migrations.

**The clean checkout is a fixture, not state.** It is created for the observation and carries no
data this repository owns afterwards. If it is placed under `.claude/worktrees/`, `.gitignore`'s
final entry — written for exactly this reason, because worktrees are "full checkouts of this
repository, each with its own `target/`" — keeps it and its build output out of every diff.
Nothing needs cleaning up in the tracked tree, and `git worktree remove` afterwards is hygiene
rather than an obligation this spec imposes.

**Two mutations are performed and both are reverted in place.** Scenarios 4 and 13 change files
in the fixture and change them back. That is a transient edit, not a migration: no committed
state of any branch ever contains it, per the architecture brief's rule that the broken edit is
never a committed state of the tree the final gate runs on (`_decomposition.md:642-647`). If a
revert cannot be proven clean, the correct recovery is to discard the fixture and take a new
checkout — it is disposable by construction — rather than to repair it.

**The only frontmatter this story is near, it does not write.** `_ledger.md` and the companion
carry the item frontmatter the `redkiln` CLI owns; the prose body is this story's to author and
the system fields are not (CLAUDE.md, "Where the work lives"). `links.kb` on HS-P0025 is
`product-layer-mounting`'s to write via `redkiln record-links --atom` (AC-A04,
`_decomposition.md:372-378`); scenario 15 reads it as evidence and never sets it.

## Acceptance criteria

Seven criteria, each framed from the intent of a reader this project actually has —
**U2**, the closeout reviewer at the pull request, and **U3**, the future maintainer
reconciling against this tree (`_decomposition.md:45-56`). U1, the next initiative's charter
author, is served indirectly: scenario 15's row is what tells U1 the atoms they are about to
frame criteria from were seen to exist somewhere other than where they were written.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3, a future maintainer who wants to know *against which tree* this initiative was called done (`_decomposition.md:49`), **WHEN** they open the re-observation ledger, **THEN** its head names a checkout that is a fresh `git clone` or a new `git worktree add` off the **merged** branch and is demonstrably not `.claude/worktrees/docs-that-teach`, together with that checkout's absolute path and the sha it resolved to — **AND** that sha is the merged sha `merge-forward-baseline` produced and this story consumed as a value rather than derived (AC-A06, `_decomposition.md:383-387`), **AND** the checkout was created only after every artefact any scenario observes was committed on that branch, so the fixture could not observe an absence the branch does not have. *(serves AC-014; AC-A07 `:388-391`, AC-012 `project.md:201`)* | **Tier 4** — in the fixture: `git rev-parse HEAD` and `git rev-parse --show-toplevel` recorded verbatim; the toplevel is not the initiative worktree. **Tier 1** — the recorded sha resolves in the initiative worktree (`git cat-file -t <sha>` is `commit`) and `git log --oneline -1 <sha>` matches the head the ledger names. **Tier 2** — for each of the fifteen rows, the artefact it reads was in the tree at that sha (`git ls-tree -r <sha> --name-only` contains it), which is the check that catches a checkout taken one commit too early. |
| AC-002 | **GIVEN** U2, the closeout reviewer, who must be able to find the one observation they disagree with without reading the discovery corpus (`_decomposition.md:48`), **WHEN** they read the ledger end to end, **THEN** all fifteen initiative scenarios (`initiative.md:413-468`) are present, none merged away and none silently dropped; fourteen carry a **fresh** observation with a named human observer, the checkout path and sha it was taken on, the instrument used, what was seen, and an outcome stated as a literal word; scenario 2 occupies **two** named halves (`failed by name`, `recovered`) that exist and are visibly pending its slice-mate rather than absent — **AND** each row *additionally* names the project that made the scenario true, as provenance — **AND** the string `inherited from a sibling` appears nowhere in the file. *(serves AC-014; AC-UX-10 `:270-273`, AC-UX-11 `:274-277`, AC-TB-04 `:833-836`, `project.md:203`, `:219-220`)* | **Tier 1** — `rg -c "inherited from a sibling"` over the companion returns no match; a row count finds sixteen data rows (fifteen scenarios, scenario 2 as two). **Tier 2** — the reviewer's read: each row against the scenario text at `initiative.md:413-468` and the owner at `.bklg/docs-that-teach/_decomposition.md:219-235`; a row missing observer, checkout, sha, instrument, evidence or outcome word is a defect. **Tier 4** — the observations themselves. |
| AC-003 | **GIVEN** U2, who knows that a claim nothing can falsify is decorative (CLAUDE.md, "The rule that matters"), **WHEN** they read the two mutation-shaped rows this story owns — scenario 4, *removing the boundary from the example makes it fail* (`initiative.md:425-428`), and scenario 13, *nothing load-bearing is hidden from the check* (`:458-461`) — **THEN** each carries a captured **red** transcript from the check run against the deliberately broken state and a **green** transcript from the re-run after the revert, both verbatim rather than summarised as "failed" and "green"; **OR**, for scenario 13 only, the initiative's second admissible arm is *shown* — the folded, tabbed or collapsed content in the shipped material enumerated item by item with each one's claims read — never asserted in a sentence; **AND** neither mutation is present in any commit on any branch, so `git status` in the fixture is clean before anything is committed anywhere. *(serves AC-014; `_decomposition.md:642-647`, `:795-802`, AC-TB-05 `:837-839`)* | **Tier 4** — run the mutation, capture, revert, re-run; the red transcript names the failing item, not merely a non-zero exit. **Tier 1** — `git status --porcelain` in the fixture is empty at the end of each mutation pair, and `git log -p` on the branch contains neither edit. **Tier 2** — for the shown-disjunct arm, the enumeration is checked to *be* an enumeration: N items named at real `file:line`, not a count and a claim. |
| AC-004 | **GIVEN** U2 reading the seven scenarios that have **no command at all** — 5, 6, 8, 9, 10, 12 and 14, plus the walk-shaped 3 and 7 — and knowing this story's own named wrong implementation is *"a ledger asserting fifteen scenarios pass, with evidence for the cheap ones only"* (`discover.md:55`), **WHEN** they read those rows, **THEN** every one names the **instrument** it used from the fixed vocabulary — command run, mutation, artefact read, reader walk — and carries evidence of that instrument's shape: a walk records its hops in order with each hop's landing point at a real `file:line` in the fixture, and a read cites the artefact at a real `file:line` in the fixture; **AND** no row's evidence is a sentence of confidence, a restatement of the scenario, or a citation of the owning project's own prior proof. *(serves AC-014; `project.md:249`, the instrument table in this spec's Context pack)* | **Tier 2** — the reviewer's read, one row at a time, against the instrument column: a row whose instrument says `reader walk` and whose evidence has no hops fails; a row whose instrument says `artefact read` and whose evidence has no `file:line` fails. **Tier 1** — every `file:line` cited by a non-command row resolves in the fixture at the named sha (`test -f` plus a line-count bound). **Tier 4** — the walks themselves, performed in the fixture. |
| AC-005 | **GIVEN** U2 checking the three rows whose evidence comes from somewhere other than this story's own reading — scenario 1 (the gate), scenario 11 (the sibling statement) and scenario 15 (this project's own output) — **WHEN** they read them, **THEN** scenario 1 carries **both** of DoD-1's claims: a `cargo xtask ci` run taken **in the fixture** with its result recorded, and the narrative build step located inside `xtask/src/main.rs`'s `REQUIRED` list (`:105`) on the merged tree, cited at the line it actually occupies with the search that found it recorded — and that run is explicitly *not* offered as AC-016's evidence, which is `terminal-gate-run`'s (`_storymap.md:156-160`); scenario 11 cites `post-merge-clause-completeness`'s completeness statement at a real `file:line`, re-deriving no clause-id set and paraphrasing no verdict (that story's AC-006); and scenario 15 records `redkiln validate --kb` run **in the fixture** with its exit and output, plus the four mount points `product-layer-mounting` landed — the appended `##` in `.kb/maps/domain-map.md`, the reciprocal `related` / `depends_on` edges, `links.kb` on HS-P0025 and the `## Knowledge Harvest` row — read where they landed rather than from memory. *(serves AC-014; `initiative.md:413-415`, `:452-454`, `:465-468`; AC-A04 `_decomposition.md:372-378`)* | **Tier 4** — both commands run in the fixture, transcripts captured. **Tier 3** — the mount-point walk for scenario 15: from `.kb/README.md` in the fixture to each landed atom, and from HS-P0025's `links.kb` back; the walk is recorded in the row. **Tier 1** — `xtask/src/main.rs:105`'s `REQUIRED` list is read at the merged sha and the cited line contains the step named; the `file:line` scenario 11 cites resolves inside `post-merge-clause-completeness`'s companion. |
| AC-006 | **GIVEN** U2 quoting one row of this ledger into a review comment, and U3 reading one row two years later with no other file open, **WHEN** either reads a single row in isolation, **THEN** it loses nothing: it names its scenario id, its owning project, its observer, its checkout and sha, its instrument, what was seen and its outcome, and depends on no row above it; **AND** every state is a literal **word** — no tick, emoji, colour, glyph, strikethrough, ordering or empty cell carries meaning anywhere in the file, the precedent being the open-questions index's literal `Open` / `Withdrawn` / `Superseded` (`.kb/maps/open-questions-index.md:136-143`); **AND** the load-bearing verdict — whether the assembled tree meets the initiative's Definition of Done — is stated as a **sentence** in prose as well as carried by the table, because a verdict that exists only in a cell is lost in a quote or a diff; **AND** the file is composed from the corpus's own primitives — one `#`, sections at `##`, no skipped level, every link naming its destination rather than "here" or "see above", transcripts in fenced blocks under their own heading rather than interleaved with verdicts — with no invented status vocabulary, no bespoke per-artefact table format and no new heading grammar. *(serves AC-014; AC-UX-09 `:266-269`, AC-UX-10 `:270-273`, a11y floor `:115-145`, primitives and the hand-rolling ban `:74-107`)* | **Tier 2** — the presentation review, run as a read of the rendered markdown rather than of its source: a reviewer covers each row with the ones above it and confirms it still reads, then scans for any state expressed by anything other than a word. **Tier 1** — `rg` over the companion for emoji, `~~`, and `[ ]`/`[x]` used as status; a heading pass confirming exactly one `#` and no skipped level; a link-text pass confirming no bare "here"/"this"/"see above". |
| AC-007 | **GIVEN** the three downstream consumers this ledger exists for — `scenario-two-fault-injection`, which must fill scenario 2's two halves into a shape it did not design; `terminal-gate-run`, which runs the gate on the tree that already carries this file; and the initiative closeout, whose exit criterion is that every scenario was observed on the assembled result (`initiative.md:655-657`) — **WHEN** any of them reaches for this ledger, **THEN** it is *mounted*: this story's `_ledger.md` cites the companion **per criterion** with a real `file:line` rather than a bare filename, so the record is reachable and not merely present (`_decomposition.md:429-436`); scenario 2's two slots are named and structurally identical to the fourteen owned rows, so the slice-mate fills rather than redesigns; every row is citable without paraphrase; **AND** where a re-observation came back red, the row records the finding with its owning project or its route to the `support` initiative (`.redkiln/config.yaml:5`) and the finding is **not fixed here** — re-implementing a sibling's requirement inside this closeout is forbidden by the project's boundary-level DoD (`project.md:219-220`) — **AND** `git diff --name-only` lists no path outside `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**`. *(serves AC-014; `.redkiln/config.yaml:67`, `_storymap.md:94-98`, `:127`)* | **Tier 3** — the mount-point walk: open `_ledger.md`, follow each row's `evidence` `file:line`, land on the sentence or row in the companion it claims; then open the companion and confirm scenario 2's two slots carry the same columns as row 1. **Tier 1** — `git diff --name-only` against the merge base lists only paths under the story glob; `git diff` shows no `-` line in any pre-existing file; `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) green on the prose-only diff. **Tier 2** — every red row is checked to carry a named owner or route, and the diff is checked to contain no fix to a sibling's artefact. |

**Coverage of the traced project AC.** AC-014 — *"all fifteen DoD scenarios re-observed, clean
checkout"* (`project.md:203`; tier row `_decomposition.md:874`) — is carried by all seven: AC-001
is the *clean checkout* half, AC-002 the *all fifteen* half, AC-003/AC-004/AC-005 are the three
evidence classes that stop "re-observed" degrading into "asserted", AC-006 is the shape that makes
a row readable and quotable, and AC-007 is what makes the ledger a record something downstream can
consume rather than a file in a folder. AC-015 is deliberately **not** traced here: scenario 2's
evidence is `scenario-two-fault-injection`'s, and this story owns only the shape its rows go in
(`_storymap.md:127`).

## Interaction quality

The blocking interaction-quality invariants (RFC §6.7/D6), in two families. Every one of them is
carried by an `AC-###` **row in the table above** — this section names *which* row and *what
falsifies it*, and adds no obligation that is not already an AC.

A word on the composition family, because this story's situation is unusual and the honest reading
matters. The signed-off `_design.md` declares **no surface at all** — `# no items — no public API
surface, no rendered UI surface` (`:45-47`) — and states that this repository has no CSS, no
`.tsx`, and no design-token layer anywhere outside markdown prose (`:30-35`). So the composition
family does not bind through a component library here. It binds through the thing the project's UX
brief identifies as this corpus's actual primitive layer: *"the atom template, the frontmatter
vocabulary, and the two map formats — and they behave exactly like a token layer"*
(`_decomposition.md:74-81`), with an explicit hand-rolling ban (`:97-107`). That is not a weaker
obligation. This ledger has exactly one reader — a human — and no `redkiln validate --kb`, no
compiler and no lint reads the file at all (context decision 12), so presentation is the only thing
standing between a real observation and a ticked box.

### State invariants

| Invariant | Carried by | What falsifies it |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1, `_decomposition.md:155-157`) | AC-006 | A row whose "what was seen" cell reads *see the transcripts section* with no anchor, or whose outcome can only be determined by opening the owning project's own record. U2 must be able to act on one row from one open file. |
| **Non-occlusion — a summary must not replace a row** (IQ-2, `:162-168`) | AC-002 | A ledger with a "fourteen of fifteen observed" headline and thirteen rows. A summary sentence may precede the fifteen; it may never stand in for one. |
| **Non-occlusion — a red outcome stays visible** (IQ-2, `:170-173`) | AC-007 | A scenario that came back red, quietly re-run after an off-ledger fix and recorded only as green. Supersession annotates; it does not delete. The red observation, its finding and its route all stay in the file. |
| **Preserved position** (IQ-3, `:180-187`) | AC-007 | `git diff` showing a `-` line in any pre-existing file, or any reflow that shifts a `file:line` another artefact already cites. This story adds files; it moves nobody's anchor. |
| **Reversibility** (IQ-4, `:189-199`) | AC-003 | A mutation that survives into a commit, or a fixture repaired by hand after a failed revert instead of being discarded and retaken. The fixture is disposable by construction; the commit history is not. |
| **Reachable without prior knowledge — the keyboard-reachability analogue** (IQ-5, `:201-205`) | AC-007 | The companion cited from `_ledger.md` by filename with no line, or not cited at all, so `terminal-gate-run` and the closeout must paraphrase a verdict instead of citing it. Findable only by knowing the story slug is unreachable. |
| **Preserved selection — prior citations keep meaning what they meant** (IQ-6, `:207-212`) | AC-005 | Scenario 11 re-deriving the clause-id set, producing a second enumeration beside `post-merge-clause-completeness`'s. Two verdicts about one question is the forked-audience defect in another medium. |
| **The qualification travels with the claim** (IQ-7, `:214-219`) | AC-002, AC-004 | A row stating `re-observed` whose caveat — a partial walk, an unavailable artefact, a second checkout — sits in a trailing notes block a scanning reviewer never reaches. The qualification belongs in the row. |
| **Every state change is legible at the moment of reading** (IQ-8, `:221-224`) | AC-006 | A state expressed by presence, absence, ordering or omission: an empty outcome cell meaning "fine", or a scenario 2 slot simply missing rather than named and pending. |

### Composition invariants

| Invariant | Carried by | What falsifies it |
| --- | --- | --- |
| **Presentation exists at all** | AC-006 | Fifteen bare bullet lines with a sha at the top. The composed presentation this corpus mandates is a table whose columns are the states the reader needs to see (`_decomposition.md:63-72`), plus prose that states the verdict — not markup that happens to render. |
| **Composition and placement** | AC-007 | The companion written into this story's directory and referenced by nothing. A record nothing cites is this project's analogue of a component rendered into no tree (`:429-436`); the citation from `_ledger.md` is the mount. |
| **Transience — persistent chrome, revealed, opened on demand** | AC-006 | Persistent chrome: the checkout path, the sha and the verdict sentence, stated at the head and never reachable only from mid-file. Revealed per row: observer, instrument, outcome word, evidence pointer. Opened on demand: the transcripts, under their own `##` heading and pointed *to* from a row — never pasted into a table cell, the shape that makes fourteen rows unreadable to keep one honest. |
| **Density budget, with its numbers** | AC-006 | This spec fixes the budget, since no design system does: **sixteen data rows** (fifteen scenarios, scenario 2 as two named halves); **at most nine columns** — scenario id, abbreviated scenario, owning project, observer, checkout, sha, instrument, what was seen, outcome; **at most one sentence per cell**, anything longer moved under the transcripts heading and pointed to; **exactly one `#`**, sections at `##`, `###` only beneath the transcripts section. A tenth column, a multi-paragraph cell, or an inlined transcript falsifies it. |
| **Hierarchy** | AC-006 | The verdict sentence buried below fifteen rows and four transcripts. The order is: what tree this is → the verdict in prose → the sixteen-row table → the transcripts → the findings and their routes. A reader who stops after the second section has the answer; a reader who continues has the evidence. |
| **The design's named anti-patterns** (`_decomposition.md:97-107`) | AC-006, AC-002 | Any of: a status carried by an emoji, a tick, a colour word or an empty cell; a bespoke per-artefact vocabulary invented for this file; a new table or heading format where the corpus already has one; the literal string `inherited from a sibling` as an outcome value, which AC-UX-11 (`:274-277`) makes a review-blocking defect rather than a wording preference. |

An unstyled render satisfies every path-resolves and string-absent assertion in this spec
perfectly. The composition rows above are what make that fail.

## Error conditions

| id | Condition | Required handling |
| --- | --- | --- |
| EC-001 | The observation is taken in `.claude/worktrees/docs-that-teach`, or in any tree that is not a fresh checkout off the merged branch | Inadmissible. AC-001 fails outright and the ledger is not amended to explain the exception. The tension was recorded before planning began and is not negotiable at implementation time (`_grounding.md:148-155`; AC-A07, `_decomposition.md:388-391`). |
| EC-002 | The checkout was taken before every observed artefact was committed, so the fixture legitimately cannot see them | Do **not** hand-correct the affected rows from the initiative worktree. Commit what is missing on the merged branch, discard the fixture, take a new checkout, re-run the affected observations, and record the new sha on every row re-observed against it. |
| EC-003 | A re-observation genuinely comes back red on the assembled tree | Record it red, with the transcript or walk that showed it, the finding stated plainly, and the owning project named or a route to the `support` initiative (`.redkiln/config.yaml:5`). Do not fix it here (`project.md:219-220`), and do not re-run to green after an off-ledger repair — that erases the observation (IQ-2, `_decomposition.md:170-173`). |
| EC-004 | A scenario 4 or scenario 13 mutation cannot be proven cleanly reverted (`git status` is not empty, or the green re-run differs from the pre-mutation baseline) | Discard the fixture and take a new checkout. It is disposable by construction; repairing it risks exactly what the architecture brief forbids — a broken edit reaching a committed state (`:642-647`). |
| EC-005 | `post-merge-clause-completeness`'s statement does not exist, or exists but carries no line-addressable verdict | Scenario 11's row is **blocked**, not improvised. Do not re-derive the clause-id set and do not paraphrase a verdict from HS-P0020's original proof. Record the block against the blocking story and escalate; the `blocked_by` edge exists for exactly this (`_storymap.md:56`). |
| EC-006 | `redkiln validate --kb` exits non-zero in the fixture | Scenario 15 is red. Capture the output verbatim, name `audience-ingest-wave` or `product-layer-mounting` as owner per what the failure is about, and route it. Note that a green `validate --kb` says nothing about `.kb/_intake/` (`.kb/_intake/README.md:21-27`), so it is not evidence about the staging directory either way. |
| EC-007 | A second checkout becomes necessary mid-run | Name both checkouts at the head of the ledger and carry the actually-observed sha on every row. Never let a row inherit a sha from the row above; that is precisely what AC-UX-10's self-containment forbids (`_decomposition.md:270-273`). |
| EC-008 | The companion is written but `_ledger.md` cites it by bare filename, or not at all | Unmounted. AC-007 fails. The citation must be per criterion and carry a real `file:line`, because that is the walk `terminal-gate-run` and the closeout will perform (`:429-436`). |
| EC-009 | The narrative build step cannot be located in `xtask/src/main.rs`'s `REQUIRED` list at the merged sha | Scenario 1's *second* claim is red — the narrative material is not built as part of the gate — and is recorded as such against HS-P0020. A green gate run alone does not discharge DoD-1 (`initiative.md:413-415`); recording only the green half is this story's named wrong implementation in miniature. |
| EC-010 | `git status` in the fixture is not clean at the moment scenario 1's gate run is taken | The run is inadmissible as scenario 1's evidence: it is a gate run on a mutated tree. Revert, confirm clean, re-run, and record the clean-status check alongside the transcript. |
| EC-011 | A finding tempts an ADR | Out of scope in every form. Record the gap in prose and route it; an ADR written as a closeout by-product is a named non-goal of this project (`project.md`, "Out of scope"; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`). |

## Non-functional

| id | Requirement | How it is met |
| --- | --- | --- |
| NF-001 | The diff compiles nothing and changes no behaviour | Prose only, under one directory. `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) is the story-grain check and is green; no crate, port, feature or dependency is touched, so ADR-0001 and ADR-0003 are untouched by construction. |
| NF-002 | Every observation is independently repeatable | A later reader with the recorded sha can recreate the fixture and re-run the row: commands recorded verbatim with their invocation, walks recorded hop by hop, reads cited at `file:line`. Repeatability is the property AC-012's name-the-tree requirement exists to buy (`_decomposition.md:278-281`). |
| NF-003 | No path outside the story glob changes, and no pre-existing anchor moves | `git diff --name-only` lists only `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**`; `git diff` shows no `-` line in any pre-existing file (IQ-3, `:180-187`). |
| NF-004 | No `.kb/` write of any kind | `.kb/` is read as scenario 15's evidence and never written; `.kb/decisions/` is untouched by anyone in this project (AC-A09, `:396-400`), which `redkiln validate --kb`'s accepted-decision immutability check would catch structurally in any case. |
| NF-005 | The fixture never pollutes a diff | If placed under `.claude/worktrees/`, `.gitignore`'s final entry ignores `.claude/` wholesale — written for this reason, because worktrees are full checkouts each with their own `target/`. If placed elsewhere, it is outside the repository and cannot appear in `git status` here. |
| NF-006 | Transcripts are whole enough to identify what happened by name | A red transcript names the failing item (page, claim, check) and carries its exit status; a green transcript carries the command line and its result. "It failed" and "it passed" are not transcripts, and AC-TB-05 (`:837-839`) makes that explicit for the mutation-shaped rows. |
| NF-007 | The ledger remains readable after the fixture is gone | Nothing in the file depends on the checkout still existing: every claim is a sha, a `file:line` inside the repository at that sha, or a captured transcript. `git worktree remove` afterwards is hygiene, and removing it must not invalidate a single row. |
| NF-008 | The record is legible to a reader with no colour perception and no rendering | Every state is a word; the verdict is a sentence; heading structure is one `#` and `##` sections; every link names its destination (a11y floor, `:115-145`). |

## Implementation notes (non-prescriptive)

Not a procedure — the notes below are where the cost actually lives, and an implementer who finds
a better order should take it as long as the ACs still hold.

**Take the checkout once, and take it late.** The whole cost of this story is paid in one place: if
the checkout is taken before the branch carries every artefact, every downstream row is suspect and
the honest recovery is EC-002's — discard and retake. It is worth reading `git log --oneline` on the
merged branch against the fifteen scenarios' artefacts *before* creating the fixture rather than
discovering the gap on scenario 15.

**Write the ledger's skeleton before running anything.** The row shape is an obligation to a
slice-mate (`scenario-two-fault-injection` fills scenario 2's halves into it) and to downstream
consumers (`terminal-gate-run` and the closeout cite its rows). Fixing the columns and the two
scenario 2 slots first turns fifteen observations into fifteen fills, and it is what stops the cheap
rows being written in a shape the expensive ones cannot fit.

**Run the two command-shaped scenarios early, the two mutation-shaped ones last.** Scenario 1's gate
run and scenario 15's `redkiln validate --kb` are long and tell you immediately whether the fixture
is sound. Scenarios 4 and 13 dirty the fixture; doing them last means a failed revert costs one
retake rather than invalidating a morning of walks.

**The seven no-command scenarios are the work.** 5, 6, 8, 9, 10, 12 and 14 have no transcript to
paste and are where the wrong implementation lives. A practical counterweight: write the instrument
cell *before* the evidence cell. Committing to "reader walk" first makes an evidence cell with no
hops visibly wrong to the person writing it, which is cheaper than catching it at review.

**Scenario 13's disjunct is a search, not a judgement.** Deciding "no folded content carries a
load-bearing claim" honestly means enumerating the folded, tabbed and collapsed content that exists
in the shipped material — a search over the documentation surface for the collapse constructs it
actually uses — and reading each one's claims. The enumeration is the evidence; the conclusion is a
line beneath it.

**The companion's filename is yours; its directory is not.** `_dod-reobservation.md` is a working
name. Whatever it is called, it lives under this story's directory and `_ledger.md` cites it per
criterion at a real `file:line`.

**Record the search that located the narrative build step.** Scenario 1's second claim is answered
by a line in `xtask/src/main.rs`'s `REQUIRED` list (`:105`) whose landed shape is not visible from
this worktree. Recording the search that found it lets a later reader repeat the location step
without knowing HS-P0020's internals, and makes EC-009 distinguishable from "we did not look hard
enough".

## Tests and CI (merge gate)

Grounded in the project's testing brief — the four tiers at `_decomposition.md:706-711`, the
clean-checkout and fault-injection seams at `:789-802`, and AC-014's tier row at `:874`, which
assigns it **Tier 4 primary with no "also" tier**: no machine check can substitute for the
observation.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **1 — static / shape** | `git diff --name-only` and `git diff` against the merge base; `git cat-file -t <sha>`; `git ls-tree -r <sha> --name-only`; `rg -c "inherited from a sibling"`; `test -f` over every path a row cites | NF-003, NF-004 and AC-007's boundary half: nothing outside `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/**` changed, no `-` line in any pre-existing file, no `.kb/` write. AC-001's sha resolves and carried the artefacts observed. AC-002's forbidden string is absent. AC-004's cited `file:line`s resolve. |
| **1 — static / shape (in the fixture)** | `git rev-parse HEAD`, `git rev-parse --show-toplevel`, `git status --porcelain` in the checkout | AC-001: the fixture is a different tree at the merged sha. AC-003 and EC-010: the tree is clean before each gate-shaped run and after each revert, so no mutation is live when an admissible observation is taken. |
| **2 — content review** (the "unit" analogue — a human, one claim at a time, against a named checklist) | The reviewer's read of the companion against the UX brief's own checklist — IQ-1…IQ-8 (`_decomposition.md:147-224`) and AC-UX-09/10/11/12 (`:266-281`) — not a second list invented here (AC-TB-06, `:840-842`) | AC-002, AC-004, AC-006: each row self-contained; each instrument matched by evidence of that instrument's shape; every state a literal word; the verdict a sentence. This is the **only** tier that can fail the named wrong implementation, *"a ledger asserting fifteen scenarios pass, with evidence for the cheap ones only"* (`discover.md:55`) — no machine can tell a hop-by-hop walk from a confident sentence. |
| **3 — integration / mount-point** | The walk: `_ledger.md` → each row's `evidence` `file:line` → the sentence or row in the companion. Plus, inside the fixture, the four-mount-point walk for scenario 15 from `.kb/README.md` outward and from HS-P0025's `links.kb` back | AC-007 and AC-005: the record is reachable and citable rather than merely present, and the atoms `product-layer-mounting` landed are reachable in the fixture. This is the same walk `terminal-gate-run` and the closeout will perform. |
| **4 — e2e / whole-tree** (the terminal grain) | In the fixture: `cargo xtask ci` (`.redkiln/config.yaml:60`) for scenario 1; `redkiln validate --kb` for scenario 15; the scenario 4 and scenario 13 mutation pairs; the nine walk- and read-shaped scenarios performed by a human | AC-001 through AC-005 — the observations themselves. Per AC-014's tier row (`_decomposition.md:874`) this tier has no substitute. **Not** AC-016: that is `terminal-gate-run`'s later run on the final tree carrying this ledger (`_storymap.md:156-160`). |
| **Story-grain gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | NF-001: the prose-only diff breaks nothing. Expected to be trivially green — recorded because `.redkiln/config.yaml` wires it at story grain whether or not anyone types it (CLAUDE.md, "Commands"). |
| **Ledger gate** | `redkiln verify --grain story` reading `_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:67`) | That every `AC-###` in this spec has a ledger row, is `satisfied: true`, and carries non-placeholder evidence before `implement → report`. It checks the *presence* of evidence, never its quality — Tier 2 is what checks that. |

**What this story does not run.** `cargo xtask ci` on the *final* tree (AC-016, `terminal-gate-run`),
`cargo xtask lints && cargo xtask spec-trace` corpus-wide (`post-merge-clause-completeness` and the
`reachability_static` grain, `:48`), and any Rust test. `cargo xtask ci`'s Rust-grain steps run
inside scenario 1's fixture run as a regression net proving `crates/`, `examples/`, `spec/` and
`standards/` stayed untouched, not as tests of new behaviour (AC-TB-08, `:846-851`).

## Risks and coupling (PR-scoped)

| Risk | Why it bites here | Mitigation in this PR |
| --- | --- | --- |
| **Fifteen re-observations become fifteen ticked boxes** (`project.md:249`; `discover.md:55`) | Seven scenarios have no command, and a confident sentence costs nothing to write and nothing to review if the row does not demand otherwise. This is the story's whole reason for existing, so it is the risk that matters most. | Structural, not exhortative: AC-004 makes the **instrument** a required column and binds each instrument to a shape of evidence — hops for a walk, a `file:line` for a read — and Tier 2's review runs against the UX brief's own falsifiers rather than a fresh checklist. |
| **The checkout is taken too early** | A fresh checkout sees committed state only. If the atoms, the map appends, `links.kb`, the reconciliation record or the clause-completeness statement are not yet on the branch, the fixture observes their absence and up to four rows record honest failures of a tree that is actually fine. | AC-001 requires the artefact each row observes to be present at the named sha (`git ls-tree`), which catches the off-by-one-commit case; EC-002 makes discard-and-retake the required recovery rather than hand-correcting rows. |
| **Scenario 1's gate run is offered as AC-016's evidence** | Both are `cargo xtask ci`. Conflating them yields a gate run taken before the records are in the tree, which "proves the gate, not the deliverable" (`_decomposition.md:638-640`), and leaves AC-016 with no real run at all. | AC-005 requires scenario 1's row to state explicitly that its run is an observation in the fixture and not AC-016's; `terminal-gate-run` is last in the milestone by construction (`_storymap.md:156-160`). |
| **The slice-mate is blocked by a shape it cannot change** | `scenario-two-fault-injection` fills scenario 2's two halves into this ledger. If the row shape has no room for a captured failing transcript, or the slots are simply absent, the slice-mate must either redesign the file or bolt its evidence on beside it. | AC-007 requires the two slots to exist, be named, be visibly pending, and carry the *same* columns as the fourteen owned rows; the transcripts section exists from the start so the failing half has somewhere to land (AC-TB-05, `_decomposition.md:837-839`). |
| **Scenario 11 re-derives what a sibling already decided** | The temptation is real: enumerating clause ids is mechanical and feels like better evidence than a citation. It produces a second verdict about one question, and the two can disagree. | AC-005 forbids re-derivation and requires a `file:line` citation into `post-merge-clause-completeness`'s statement — which that story wrote to be citable whole, as its own AC-006 states. IQ-6 is the invariant being protected. |
| **A mutation leaks into a commit** | Scenarios 4 and 13 edit the fixture. A stray `git add -A` in the wrong tree, or a revert that half-worked, puts a knowingly broken state into history, and the final gate then runs against it. | AC-003 requires `git status --porcelain` clean after each pair and no mutation in `git log -p`; EC-004 makes a doubtful revert a discard rather than a repair. The fixture is a separate tree, which limits the blast radius to a tree this PR does not carry. |
| **Coupling: `product-layer-mounting`** | Scenario 15 is the one row where this project observes its own output, so it is the row most likely to be written from what the author knows rather than from the fixture. | The `blocked_by` edge, plus AC-005 requiring `redkiln validate --kb` to be *run in the fixture* with its output recorded, and the four mount points read where they landed (AC-A04, `_decomposition.md:372-378`). |
| **Coupling: `post-merge-clause-completeness`** | Scenario 11 has nothing to cite until that statement exists at a citable line. | The `blocked_by` edge, and EC-005 making the absence a recorded block rather than an improvised re-derivation. |
| **A finding turns into work** | Fourteen scenarios owned by four sibling projects, observed by someone with the tree open and the ability to fix things. Fixing one here re-opens a sibling's requirement inside a closeout. | AC-007 requires a red row to carry a named owner or a route to the `support` initiative (`.redkiln/config.yaml:5`); the PR boundary's single glob makes any such fix a boundary violation `redkiln verify --grain story` fails on. |

## Dependencies

**Blocks on** — both are real `blocked_by` edges on the item, and both exist so that one row of this
ledger has something *fresh* to cite:

| Story slug | What this story consumes from it | What breaks without it |
| --- | --- | --- |
| `post-merge-clause-completeness` | The written completeness statement over the frozen documentation MUSTs on the merged tree, authored to be citable by path and line without paraphrase (that story's `## Integration contract` and its AC-006) | Scenario 11 has no fresh observation to cite. The wrong recovery — re-deriving the clause-id set here — produces a second verdict; the right one is EC-005. It also transitively supplies the merged sha, since it depends on `merge-forward-baseline`. |
| `product-layer-mounting` | The four landed mount points — the appended `##` in `.kb/maps/domain-map.md`, the reciprocal `related` / `depends_on` edges, `links.kb` on HS-P0025 and the `## Knowledge Harvest` row (AC-A04, `_decomposition.md:372-378`) — and, behind it, the atoms `audience-ingest-wave` landed | Scenario 15 cannot be observed at all: DoD-15 requires atoms present, valid, `validate --kb` green, reconciled **and linked from the closeout** (`initiative.md:465-468`), and the last clause only exists once mounting has landed. |

**Unlocks** — both are slice-mates in `dod-reobservation`, implemented in this same context and
mounted as one surface (`_storymap.md:156-160`):

| Story slug | What it takes from this story |
| --- | --- |
| `scenario-two-fault-injection` | The ledger's fixed row shape and scenario 2's two named, pending half-slots, which it fills with the failing-by-name transcript and the recovery run. It does not design the file it writes into (`_storymap.md:57`, `:127`). |
| `terminal-gate-run` | The tree that already carries this ledger — the terminal `cargo xtask ci` must run *after* it, on the tree containing it, or it proves the gate rather than the deliverable (`_decomposition.md:638-640`; AC-016, `project.md:205`). |

Beyond the milestone, the initiative's closeout consumes this ledger directly: the exit criterion is
that every one of the fifteen scenarios was observed on the assembled result from a clean checkout
(`initiative.md:655-657`), and this file is the artefact that says so.

## Anchors (progressive disclosure)

The Context pack above is the must-read core and is complete on its own. Everything below is
deferred depth — open it at the named moment, do not preload it, and never paste it in bulk.

| Anchor | Why it is load-bearing | When to open it | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/initiative.md` | The fifteen scenario texts verbatim (`:413-468`), the preamble that makes them *run and observed* rather than argued (`:406-411`), and the exit criterion this ledger discharges (`:655-657`). The abbreviations in this spec's instrument table are abbreviations — the scenario's own wording is what each row must answer. | Before writing any row, one scenario at a time. Re-open `:458-461` before choosing scenario 13's arm. | AC-002, AC-003, AC-005 |
| `.bklg/docs-that-teach/_decomposition.md` | Which project *made* each scenario true (`:219-235`) — the provenance column every row carries — and the sentence making this a separate obligation rather than a roll-up: re-observation is "a verification obligation, not a second ownership" (`:236-239`). | Before filling the owning-project column, and again if tempted to fix something a scenario reveals. | AC-002, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-014 in its own words (`:203`), AC-012 (`:201`), the boundary-level DoD requiring each entry to cite the project that made it true (`:219-220`), and the named risk this story exists to defeat (`:249`). | Before the first row, and before deciding what to do with a red observation. | AC-002, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The three briefs in one file: the architecture brief's clean-checkout requirement (AC-A07, `:388-391`), merged-sha-as-value (AC-A06, `:383-387`), the mutation-never-committed rule (`:642-647`), ordering 4 (`:638-640`) and the mount analogue (`:429-436`); the UX brief's states table (`:63-72`), primitives and hand-rolling ban (`:74-107`), a11y floor (`:115-145`), IQ-1…IQ-8 (`:147-224`) and AC-UX-09/10/11/12 (`:266-281`); the testing brief's four tiers (`:706-711`), the two seams (`:789-802`), AC-TB-04/05/06/08 (`:833-851`) and AC-014's tier row (`:874`). | The architecture rules before creating the fixture; the UX block before composing the ledger; the testing block before claiming a tier in `_ledger.md`. | AC-001, AC-003, AC-004, AC-006, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The binding sign-off and what it binds: `## Items` empty by decision, `# no items — no public API surface, no rendered UI surface` (`:45-47`), the reasoning that this repository has no CSS, no `.tsx` and no token layer outside markdown (`:30-35`), and the ledger named as one of the project's four user-facing artefacts (`:26-35`). | Before putting the Composition invariants into practice — it is why they route through the corpus's own primitives rather than a component library. | AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | This story's row (`:56`), backbone activity A4 (`:35`), why `dod-reobservation` is one surface (`:76-78`), the AC-014 ownership seam against `scenario-two-fault-injection` (`:127`), the per-story-ledger versus re-observation-ledger distinction (`:94-98`), and merge order 4 (`:156-160`). | Before fixing the row shape (because a slice-mate fills it) and before taking any gate run. | AC-005, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | The clean-checkout tension stated before planning began: this worktree does not satisfy AC-014 and cannot be made to (`:148-155`). | The moment creating a separate checkout feels like unnecessary ceremony. | AC-001 |
| `.bklg/docs-that-teach/durable-audience-closeout/post-merge-clause-completeness/spec.md` | The upstream contract for scenario 11: that story's mount point (`:169-175`) and its AC-006, written in *this* story's terms — the statement must be citable by path and `file:line` without paraphrase. | Before writing scenario 11's row, to find what to cite and at what granularity. | AC-005 |
| `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/spec.md` | The upstream contract for scenario 15: the four mount points as that story actually lands them, so scenario 15 reads what was built rather than what was planned. | Before observing scenario 15 in the fixture, to know exactly what to look for and where. | AC-005 |
| `xtask/src/main.rs` | The `REQUIRED` step list (`:105`) — where scenario 1's second claim is answered. Read at the **merged** sha, since HS-P0020's landed shape is not visible from this worktree. Read only; no step, subcommand or entry is added. | While observing scenario 1, after the gate run and before writing its row. | AC-005 |
| `.redkiln/config.yaml` | The real grains and their lines: `:40` `affected_gate`, `:48` `reachability_static`, `:60` the terminal `e2e`, `:67` `require_ledger`, `:5` the `support` initiative — the routing destination for a finding this story may not fix. | Before naming any command in a row, and before routing any red observation. | AC-005, AC-007 |
| `.kb/maps/open-questions-index.md` | The corpus precedent for literal status words, first: `Open` / `Withdrawn` / `Superseded` (`:136-143`). The shape the outcome column imitates rather than inventing. | Before choosing the outcome vocabulary. | AC-006 |
| `.kb/product/README.md` | What the product layer is for and who reads it (`:6-13`, `:38-43`) — the shape scenario 15's row is observing, and the reminder that the documented personas are this layer's subject, never its audience. | While observing scenario 15's atoms in the fixture. | AC-005 |
| `.kb/_intake/README.md` | That `redkiln validate --kb` cannot see `.kb/_intake/` (`:21-27`), so a green run is not evidence about the staging directory. | If scenario 15's observation is tempted to read `validate --kb` green as covering more than it does. | AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | Why a finding is corrected by a new record rather than an edit — the governing atom behind EC-003's "record and route, never repair in place" and EC-011's ADR prohibition. | When a red observation makes an edit look like the obvious fix. | AC-007 |
| `.redkiln/templates/_ledger.md` | The exact shape `redkiln verify --grain story` reads: the fenced `yaml` block, one row per `AC-###`, `satisfied` / `evidence` / `mount_point` / `verifying_test`. | Before flipping any row in this story's `_ledger.md`. | AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/dod-scenario-ledger/discover.md` | The named wrong implementation in its original words (`:55`) and the signal ledger behind the slice (`:19-27`). | Before writing the seven no-command rows — it is the sentence AC-004 exists to make false. | AC-004 |

## Clarifications resolved during spec

**The seven AC ids are exactly the ones the first pass declared.** AC-001 … AC-007, none added and
none dropped, and the `_ledger.md` authored beside this spec carries exactly those seven rows.

**AC-014 is the only project AC traced, and AC-015 is deliberately not.** The story map's coverage
table splits AC-014 across two stories with an explicit seam: this story "owns fourteen scenarios
and the ledger's shape"; `scenario-two-fault-injection` "owns scenario 2's two rows, which AC-015
makes a separate obligation" (`_storymap.md:127`). Claiming AC-015 here would make the slice-mate's
evidence this story's to produce, which is not the cut a human approved.

**The composition family binds through the corpus's primitives, not through a component library.**
`_design.md` declares no surface at all (`:45-47`). Rather than skip the family — which would leave
presentation ungated on a file whose only reader is a human — this spec routes it through the layer
the project's UX brief identifies as this corpus's token analogue: the atom template, the
frontmatter vocabulary and the two map formats, with the hand-rolling ban at
`_decomposition.md:97-107`. No new design decision is made; the invariants are the brief's, applied
to this artefact.

**The density budget's numbers are this spec's decision, and are recorded as such.** Sixteen data
rows, at most nine columns, at most one sentence per cell, one `#`, `##` sections, `###` only under
the transcripts heading. Nothing upstream fixes these numbers — the architecture brief explicitly
left the ledger's shape to the story map (`:669-674`), and the story map settled only that the
fifteen-scenario ledger is a companion distinct from `_ledger.md` (`_storymap.md:94-98`). A budget
with no numbers is not a budget, so this spec supplies them; an implementer who needs a tenth column
should say why in the implementation report rather than silently take it.

**Scenario 3 and scenario 4 are separate rows with separate instruments, not one row.** Scenario 3
is a run — the refusal is visible in program output (`initiative.md:421-424`). Scenario 4 is a
mutation — removing the boundary makes it fail (`:425-428`). Collapsing them loses exactly the
distinction DoD-4 exists to draw, between an example that demonstrates a claim and one that merely
accompanies it.

**Scenario 13's non-mutation arm is admissible but must be shown.** The initiative offers "or no
such content carries a load-bearing claim" (`:458-461`). This spec does not narrow the initiative's
own disjunction; it requires the second arm to be an enumeration of the folded, tabbed and collapsed
content with each one's claims read, because an unshown disjunct is precisely the ticked box
`project.md:249` names.

**Scenario 1's gate run is not AC-016's gate run, and the ledger must say so.** Two runs of the same
command for two purposes: an observation in the fixture, and the terminal run on the final tree
carrying this ledger (`_storymap.md:156-160`). AC-005 requires the distinction to be written into
scenario 1's row rather than left to a reader who happens to know the milestone order.

**The companion's filename is not fixed.** `_dod-reobservation.md` is a working name used throughout
this spec for readability. The directory is fixed and the citation from `_ledger.md` is fixed; the
basename is the implementer's.

**No ADR is owed and none may be written.** Nothing here changes a `[FROZEN]` clause, a port or a
public item; `spec/SPECIFICATION.md` is read-only in this project's diff surface
(`_decomposition.md:422-427`) and the initiative "discharges no clause and amends none"
(`initiative.md:689-690`). A finding that wants a decision is recorded in prose and routed (EC-011).
