---
item: HS-S0131
stage: spec
created: 2026-08-12T13:48:10.691Z
updated: 2026-08-12T13:48:10.691Z
template_sig: 87bbf1d0
rendered_sig: f802e954
---

# Spec — Three personas and four journeys staged in `.kb/_intake/`, the qualification in frontmatter

> **The item card's title says "Four personas" and is stale.** The set is **three persona atoms
> plus four journey atoms**, settled by the repository owner on 2026-08-12 at the `/redkiln:plan`
> spec stage and recorded in both amended artefacts
> (`.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md:150-180`
> and `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md:222-235`).
> The card's frontmatter is the CLI's to write and is not edited here; `_storymap.md:59` already
> carries the same correction inline. See *Context pack* decision 1.

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — *Referenced personas & journeys* (`:227-258`, the four journey names and the "qualification travels with them into the KB" instruction), **AC-15** (`:350-352`), **DoD 16** (`:405-407`), **BR-16** (`:299`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the traceability matrix (BR-16 → HS-P0019), the DAG, *Decisions taken at the gate* |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — **AC-010** (`:225-228`), **DR-8** (`:162-165`), **DR-9** (`:166-170`), **DR-10** (`:171-174`), the risk rows *Hand-authoring the product atoms* and *The kb-ingest glob and a second wave* (`:300-301`) |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` — the one warranted `testing` brief: the **amended DR-10 decision** (`:148-242`) and the AC-007/AC-008/AC-009 tier rows (`:50-52`); `_grounding.md`; `_design.md` (**no public API surface**, approved 2026-08-12) |
| Sibling design that the decision reconciles | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md:214-235` — DT-1's persona resolution and its 2026-08-12 rider |
| Story map row | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:59` (this row, with its inline correction), `:81-84` (why the slice is cut whole), `:149-151` (merge order — slice 4 precedes slice 5) |
| This story's discovery | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/discover.md` — the signal ledger, the escalated question and its resolution, and the named wrong implementation |
| Source material being staged | `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` — the four personas, their journeys, *Cross-persona tensions*, *Risks* (`:349-369`) and *Open questions for the planning team* (`:371-395`) |
| Roadmap pointer | `RUNBOOK.md` — the plan of record; this story adds nothing to it and amends nothing in it |

## One-line PR slice

Stage three personas and four journeys in `.kb/_intake/` under a distinct wave id, each draft
carrying the secondary-evidence sentence in `summary` and its discovery artefacts in `source_paths`.

## Executive summary

This PR lands **seven markdown drafts in `.kb/_intake/` and nothing else** — no `.kb/product/` atom,
no ingest run. It is the `foundation` half of the `durable-audience` slice: real in-tree material
that its slice-mate `product-atom-promotion-via-kb-ingest` consumes for real through
`/redkiln:kb-ingest`, which is the only path `project.md` DR-8 permits into `.kb/product/`.

Delta against what the project already says, in four places where the upstream artefacts stop short:

1. **The set is decided, not inherited.** `project.md` AC-010 requires the evaluator question to be
   settled before any atom is authored; the `testing` brief settled it one way (`_decomposition.md`
   DR-10, four personas) and the sibling `publication-and-positioning/_design.md` settled the *same*
   discovery question the other way (`:214-222`, a stage of Persona 1). Both were amended on
   2026-08-12 into a synthesis, and this spec is where the synthesis becomes a file count: **three
   persona drafts, four journey drafts, the evaluation path a journey of its own linked to Persona 1**
   (*Context pack* 1). Neither prior document is rewritten by this story; the discrepancy is recorded
   as a **routed finding** (*Context pack* 8), which is what the brief's own DR-12 language asked for.
2. **The staged draft carries the target atom's frontmatter, deliberately.** `.kb/_intake/README.md:7-11`
   says staged files are raw material and are *not* held to `KbFrontmatter`, and `:21-27` says
   `redkiln validate --kb` is blind to the whole tree. The ingest **re-authors** rather than copies,
   and adjudicates "biased hard towards merging into an existing atom" — so a qualification that
   lives only in a draft's prose can be lost in extraction without anything failing. The drafts
   therefore carry an explicit proposed-frontmatter block (*Context pack* 3) so DR-9's sentence
   arrives in the promoted atom's `summary` by construction rather than by hope.
3. **The wave is narrowable, and that is a naming decision.** `/redkiln:kb-ingest` with no argument
   ingests **all** of `.kb/_intake/*.md` (`README.md:3-5`) — including the directory's own
   `README.md`, which `project.md`'s risk table names explicitly. Every file this story writes is
   prefixed `product-`, so the slice-mate can pass `.kb/_intake/product-*.md` and leave the README
   where it belongs (*Context pack* 5).
4. **The personas get re-checked, because closeout is the moment their own source demands it.**
   `personas-and-journeys.md:361-369` says these describe an *inferred* audience and that
   `.kb/product/README.md`'s bar "should be re-checked at closeout once real contact… exists to
   confirm or correct them". This initiative built that contact. Each draft therefore carries a short
   confirmed/corrected/untouched note against artefacts from *this* initiative (*Context pack* 6) —
   which is what stops the promotion being a pre-initiative guess wearing a closeout date.

## Context pack

Everything below is a decision this story must honour. It is complete enough to start from; the
deeper artefacts sit behind the anchors and are opened only when a row here sends you to one.

**1. The set is three personas and four journeys, and the evaluation path is a journey linked to
Persona 1.** This settles the open question filed at `personas-and-journeys.md:373-377` and it is a
*synthesis of two signed-off artefacts, not a win for either*. The `testing` brief's DR-10 argued the
evaluator is its own persona; `publication-and-positioning`'s DT-1 argued it is the first fifteen
minutes of the application author's journey. What survived from DR-10 is **argument 1, never
rebutted**: the difference is in the *mechanism* of trust-building — one-shot public evidence versus
revisable contact with the code over weeks — not a difference of degree inside one journey. What did
not survive is the claim that a difference of mechanism makes a different *person*: every
distinguishing property (time-boxed, one-shot, cannot run the suite) is a property of a moment, and
the same human is an application author twenty minutes later. **A moment with its own mechanism is
exactly what a journey atom is for.** DR-10's argument 3 (that folding would make DT-1's option (c)
incoherent) was discounted outright as circular — option (c) *lost*, partly for splitting one person
in half. Both artefacts now say so
(`_decomposition.md:150-180` and `:227-236`; `publication-and-positioning/_design.md:222-235`).
Two consequences an implementer must not soften: a fourth *persona* draft is wrong, **and** so is
folding the evaluation beats into Persona 1's journey draft as a stage — "a stage buried inside
Persona 1's journey would make that mechanism unfindable to the reader it exists to serve, which is
the failure `.kb/product/` is being populated to prevent" (`publication-and-positioning/_design.md:232-234`).

**2. The seven drafts, their kinds and their names.** The journey titles are the charter's own
(`initiative.md:241-250`), not invented here, so the promoted atoms are findable from the document
that flagged them for promotion.

| Draft file (in `.kb/_intake/`) | Kind | `authority_tier` | Source |
| --- | --- | --- | --- |
| `product-persona-application-author.md` | `concept` | `product` | `personas-and-journeys.md:42-110` |
| `product-persona-adapter-author.md` | `concept` | `product` | `personas-and-journeys.md:114-178` |
| `product-persona-local-first-edge-developer.md` | `concept` | `product` | `personas-and-journeys.md:182-245` |
| `product-journey-choose-a-contract-before-a-database.md` | `playbook` | `product` | Persona 1's journey; `initiative.md:243-244` |
| `product-journey-learn-when-you-are-finished.md` | `playbook` | `product` | Persona 2's journey; `initiative.md:245-246` |
| `product-journey-event-source-at-the-edge.md` | `playbook` | `product` | Persona 3's journey; `initiative.md:247-248` |
| `product-journey-decide-in-one-sitting.md` | `playbook` | `product` | the evaluation path; `initiative.md:249-250`, `personas-and-journeys.md:249-313` |

`concept` for a persona and `playbook` for a journey, both at `authority_tier: product`, is
`.kb/product/README.md:6-9`, not a choice made here.

**3. A staged draft is raw material that nevertheless declares its target atom.** `.kb/_intake/README.md:7-11`:
"Nothing staged here is an atom yet… they are not held to `KbFrontmatter`. The ingest run extracts
the claims, adjudicates them… and writes what survives as real atoms." Two properties of that run
make an under-specified draft dangerous rather than merely vague: extraction paraphrases, and
adjudication is biased hard toward **merging into an existing atom**. DR-9 does not ask for a
qualification that *was said*; it asks for one that lands in the promoted atom's `summary` where "a
reader who reads only the frontmatter cannot miss it" (`project.md:166-170`). So each draft opens
with a YAML block giving the proposed `id`, `title`, `kind`, `status`, `authority_tier`, `summary`,
`source_paths` and `related` in the house shape
(`.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30` is the reference for that shape;
ids follow `kb-<kind>-<slug>-001`). This is safe precisely because `validate --kb` skips
`_`-prefixed directories (`.kb/_intake/README.md:21-27`) — a draft cannot fail validation where it
sits, and cannot be mistaken for an atom either.

**4. The qualification is per-atom, in its own voice, and greppable.** DR-9's sentence is written
about "all four personas"; there are now three, and four journeys, so a copy-pasted collective
sentence would be false in an atom's own `summary`. Each draft's `summary` therefore ends with a
sentence in the atom's own voice that names its own evidence classes (download counts, public posts,
issue threads, an adjacent project's postmortem — `personas-and-journeys.md:361-369`) and contains
**both literal strings `secondary evidence` and `directly observed`**. That fixed pair is what makes
project AC-009 a `grep` over seven promoted atoms rather than a reading exercise, and it is why the
journeys carry it too even though DR-9's sentence names only personas: a journey promoted without it
is the one atom in the layer that reads as first-hand.

**5. The wave is distinct and narrowable.** `/redkiln:kb-ingest` run bare consumes all of
`.kb/_intake/*.md` and **a successful ingest clears the directory** (`README.md:13-19`). Two hazards
follow, both named in `project.md`'s risk table (`:301`). The directory's own `README.md` is a `*.md`
file at the top level — hence the mandatory `product-` filename prefix, so the slice-mate's wave can
be `.kb/_intake/product-*.md`. And the wave id must not collide with the two already in history
(`2026-08-10-intake` at `e768413`, `2026-08-10-intake-2` at `72f2c9b`), because the id is what the
commit subject and the telemetry run file are keyed on; this wave is
**`<staging-date>-product-audience`** (e.g. `2026-08-12-product-audience`), declared in this story's
`_ledger.md` so the slice-mate passes the same string rather than inventing one.

**6. The sketches are re-checked against contact this initiative actually made.** The source's own
*Risks* section (`personas-and-journeys.md:361-369`) says none of these personas was interviewed or
observed and that `.kb/product/README.md:23-24`'s bar ("a persona nobody researched is a stock photo
with a name") "should be re-checked at closeout once real contact… exists to confirm or correct
them". Closeout is now. Each draft carries a short **confirmed / corrected / untouched** note naming
the artefact from *this* initiative that bears on it, and where nothing in this initiative touched
the sketch it says exactly that. The note never invents new evidence: an honest "untouched by this
initiative" is a passing answer, a silent omission is not.

**7. A journey is a path through a task, never a transcript of the surface.** `.kb/product/README.md:26-44`
excludes four things from this layer and three of them are live risks here: a screen/flow/interaction
pattern (the crates.io and docs.rs surfaces belong to `publication-and-positioning`'s `_design.md`,
and a journey that transcribes them "goes stale the moment the UI moves"); requirements, scope or
acceptance criteria (those are `.bklg/`); and a market segment. For a library the same trap has a
Rust-shaped form: a journey that names `EventStore`, `event_store_conformance!` or a crate feature
flag has transcribed the API instead of the task. The drafts stay problem-space, matching the
distillation's own declared `scope: problem-space-only` (`personas-and-journeys.md:7`).

**8. The DR-10 / DT-1 discrepancy is recorded as a finding and routed, not absorbed and not
re-litigated.** `project.md` DR-12 and its risk row *The evaluator decision has downstream reach*
(`:306`) both say the consistency of HS-P0016's recorded DT-1 resolution with this decision is "a
finding to route". This story therefore writes the finding — what disagreed, which artefacts, how it
was settled, and who owns the disposition — and hands it to `findings-disposition-register` (AC-013).
It does **not** edit the `testing` brief's DR-10 body beyond the owner's amendment already there, and
it does not reopen DT-1. Absorbing it silently is the inverse of the failure DR-12 exists to prevent:
not fixing a defect here, but entrenching an inconsistency in a durable layer because "the brief
already said so" (`discover.md` *The wrong implementation*).

**9. Nothing is hand-authored into `.kb/product/`.** DR-8 and project AC-008 make the *provenance*
observable, not just the result: the atoms must arrive in the commit range from `.kb/_intake/` via
`/redkiln:kb-ingest`, with staging consumed. The fastest route to a durable audience is to write the
seven atoms directly, and that is exactly what was reverted at `0269720` for "producing the directory
layout of the process without the process". This story's PR leaves `.kb/product/` byte-identical to
its current state (one file, `README.md`).

**10. The persona-journey slice this realises.** The reader `_storymap.md:18-22` names for this
project is someone who must believe the initiative closed honestly without re-deriving the evidence.
For this slice that reader is one initiative later: they open `.kb/product/`, and the question is
whether they inherit an adjudicated audience or a guess with a closeout date on it (AC-15,
`initiative.md:350-352`). This story is where the difference is decided, because the ingest can only
promote what the staging says — every qualification, link and citation the promoted atom will carry
either exists in a draft here or does not exist at all.

**11. Nothing here designs a surface.** `_design.md` is approved with **no public API surface** —
`N/A` against every item block, sign-off recorded 2026-08-12. This story adds, changes and removes
zero public items, so the surface invariants that normally bind a story in this repository are
vacuous and must not be ticked as though they were met.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (seven markdown files at the ingest's own
  input path), consumed inside this same slice by `product-atom-promotion-via-kb-ingest` running
  `/redkiln:kb-ingest` for real. Never a double: nothing here is a placeholder atom, and no
  `.kb/product/` file is written by this story.
- **Slice / milestone**: `durable-audience`. Slice-mate: `product-atom-promotion-via-kb-ingest`
  (the ingest run and the promoted atoms). Implemented in one context and landed as one integrated
  surface — `_storymap.md:81-84`: "Holding the two apart is how `0269720` happened: 'the directory
  layout of the process without the process.'"
- **Mount point**: **`.kb/_intake/`** — the declared default input to `/redkiln:kb-ingest`
  (`.kb/_intake/README.md:3-5`). For a capability whose output is knowledge rather than code, this
  directory *is* the composition root: a draft anywhere else — under this story's folder, under
  `.kb/product/`, or in a subdirectory of `_intake/` — is not mounted, because the wave glob is
  `.kb/_intake/*.md` at the top level and nothing else is read. The mount is demonstrated, not
  asserted: the slice-mate's ingest consumes these exact files and the directory is left cleared.
- **Wires into** (real siblings and contracts consumed, by path):
  - `.kb/_intake/README.md:3-5`, `:7-11`, `:13-19`, `:21-27` — the wave glob, the "not held to
    `KbFrontmatter`" rule, the clears-on-success contract, and the `validate --kb` blindness that
    makes a proposed-frontmatter block safe to stage.
  - `.kb/product/README.md:6-9` (persona = `concept`, journey = `playbook`, both
    `authority_tier: product`), `:11-13` (what each is), `:15-21` (closeout performs the promotion),
    `:23-24` (the evidence bar), `:26-44` (what does not belong).
  - `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30` — the in-tree reference for
    the frontmatter shape and the `kb-<kind>-<slug>-001` id convention; `.kb/playbooks/*.md:1-8` for
    the `playbook` variant.
  - `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` —
    the sole source of persona content, including its own `sourcePaths` list (`:8-21`), which is what
    each draft's `source_paths` is drawn from.
  - `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` —
    the project's convergence artefact, created by `clean-checkout-harness` (slice 1, ahead of this
    one in merge order). This story appends **one row to its *Findings* section** for the DR-10/DT-1
    discrepancy, with the destination cell left to `findings-disposition-register`; it re-shapes no
    section and creates no second record.
  - `.redkiln/config.yaml:67` / `:73` — `require_ledger` and `require_commit_provenance`: this
    story's `_ledger.md` cites the seven staged paths and declares the wave id and glob.
- **Renders surfaces**: **none.** `_design.md` records no public API surface for this project and
  declares `N/A` against `## Items`; this story implements no item id, by an approved determination
  rather than by omission. The nearest thing to a rendered surface anywhere in this initiative is the
  crates.io/docs.rs presentation, which is `publication-and-positioning`'s and explicitly out of this
  project's scope.
- **Conformance rule(s)**: none, and this is **not adapter-observable**. No port, no rule in
  `crates/happenstance-testkit/src/suite.rs`, no behaviour an adapter could implement differently.
  Naming a rule here would be decorative in exactly the sense `CLAUDE.md` forbids.
- **Clause(s)**: none discharged, none amended. No `spec/SPECIFICATION.md` clause is touched, so no
  `[FROZEN]` clause is at risk and no new ADR is owed.
- **Advances DoD scenario**: **DoD 16** — "The audience is durable. Persona and journey atoms exist
  under `.kb/product/` with valid frontmatter and pass validation, and the initiative's closeout
  links them" (`initiative.md:405-407`), and through it **AC-15** (`:350-352`) and **BR-16**
  (`:299`). This story does not turn DoD 16 green on its own — the atoms do not exist until the
  slice-mate's ingest runs — but every property DoD 16 will be checked for (the kinds, the tier, the
  qualification in frontmatter, the citations) is either present in these drafts or unavailable to
  the run that promotes them. It also discharges project **AC-010**, whose observable is that the set
  matches the decision.

## PR boundary

`redkiln verify --grain story` reads the first fenced block under this heading and fails on any file
changed outside it. The set is narrow because this project owns no code (`project.md` *Out of
scope*) and the `testing` brief's AC-013 row makes "zero commits inside this project's own stories
touch code outside `.bklg/`/`.kb/` planning artefacts" an observable criterion.

```
.kb/_intake/product-*.md
.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
```

The third path is the mount wiring named in the *Integration contract* — one appended *Findings*
row — and is not scope drift. `.kb/product/**` is deliberately absent: writing there is the whole
prohibition (*Context pack* 9).

**In this PR**

- Seven drafts under `.kb/_intake/`, named exactly as *Context pack* 2 lists them, each with its
  proposed-frontmatter block, its qualification sentence, its `source_paths`, its confirmed /
  corrected / untouched note, and — for the journeys — its persona link.
- The declared wave id and the narrowing glob, recorded in this story's `_ledger.md` so the
  slice-mate consumes the same wave rather than a bare directory.
- The DR-10/DT-1 finding, written under this story's folder and mounted as one row in the project's
  `_closeout-record.md` *Findings* section with its destination left owed.
- This story's `_ledger.md` (second pass), citing the seven staged paths per `.redkiln/config.yaml:67`.

**Explicitly not in this PR**

- Running `/redkiln:kb-ingest`, creating any file under `.kb/product/`, clearing `.kb/_intake/`, or
  running `redkiln validate --kb` against promoted atoms — all of that is
  `product-atom-promotion-via-kb-ingest` (project AC-007, AC-008, AC-009), in this same slice.
- Flipping `promoted: false` → `true` in
  `_discovery/distillation/personas-and-journeys.md`'s frontmatter. That claim is only true once the
  atoms exist, so it belongs to the slice-mate or to nobody.
- Editing the `testing` brief's DR-10 body, or `publication-and-positioning/_design.md`'s DT-1. Both
  already carry the owner's 2026-08-12 amendment; a downstream story does not silently rewrite a
  signed-off brief (*Context pack* 8).
- Answering the two questions `personas-and-journeys.md:378-391` leaves open (is "adapter author"
  purely internal for this timeframe; is the orphaned-tooling fear owed a proof artefact). They are
  carried into the drafts as stated-open, never silently settled.
- Routing the finding to a destination, opening a `support` item, or re-planning anything — AC-013 is
  `findings-disposition-register`'s.
- Any change to a crate, `xtask/`, `spec/SPECIFICATION.md`, `.github/workflows/ci.yml`,
  `.redkiln/templates/`, or any `.kb/` atom outside `_intake/`.

**Merge DoD one-liner.** A reader who opens `.kb/_intake/` sees exactly seven `product-*.md` drafts
whose count, kinds, links and qualification sentences already match what `.kb/product/` is supposed
to contain — so the ingest that follows has nothing left to decide and nothing left to lose.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B-1 · Seven drafts, three of them personas** | Exactly the seven files of *Context pack* 2, at the top level of `.kb/_intake/`, all prefixed `product-`. Three `concept`/persona drafts (application author, adapter author, local-first/edge developer) and four `playbook`/journey drafts, the fourth being the evaluation path. A fourth persona draft, or six drafts with the evaluation beats folded into Persona 1's journey, both fail: the first is DR-10 unamended, the second is the sibling design's original consequence, and 2026-08-12 overruled each in the same breath. | `_decomposition.md:150-180`, `:227-236`; `publication-and-positioning/_design.md:214-235`; `_storymap.md:59` |
| **B-2 · The evaluation journey is linked to Persona 1 and says why it is not a persona** | `product-journey-decide-in-one-sitting.md` declares the application-author persona in `related` (and reciprocally, Persona 1's draft names it), and its body carries one short paragraph stating the settled reason: the difference is one of *mechanism* (one-shot public evidence versus revisable contact with the code over weeks), which earns a journey atom, while every distinguishing property is a property of a moment rather than of a person — citing both amended artefacts. Without that paragraph the next reader re-opens a question two projects already closed. | `_decomposition.md:150-180`; `publication-and-positioning/_design.md:222-235`; `personas-and-journeys.md:333-338` (*Cross-persona tensions*, Persona 4) |
| **B-3 · Each draft declares its target atom's frontmatter** | A YAML block at the head of the draft giving `id` (`kb-concept-…-001` / `kb-playbook-…-001`), `title`, `kind` (`concept` \| `playbook`), `status`, `authority_tier: product`, `summary`, `source_paths`, `related`, in the shape the corpus already uses. Staged files are not held to `KbFrontmatter` and `validate --kb` is blind to `_`-prefixed directories, so this cannot fail where it sits — it exists so extraction cannot paraphrase away a field the promoted atom is checked for. | `.kb/_intake/README.md:7-11`, `:21-27`; `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30`; `.kb/product/README.md:6-9` |
| **B-4 · The qualification sits in `summary`, per atom, and greps** | Every one of the seven `summary` values ends with a sentence in the atom's own voice naming its own evidence classes and containing both literal strings `secondary evidence` and `directly observed`. Personas name what they rest on (download counts for Persona 1, the project's own six skeletons and the surveyed conformance regimes for Persona 2, one blog post plus one issue thread for Persona 3); the evaluation journey states it is inferred from research framing rather than a named individual's account. A collective "all four personas…" sentence is wrong twice over: there are three personas, and it reads as someone else's claim inside an atom's own summary. | `project.md:166-170` (DR-9); `initiative.md:252-258` (*Flagged for promotion at closeout*); `personas-and-journeys.md:349-355`, `:361-369` |
| **B-5 · `source_paths` names the discovery artefacts, and verifies the referent** | Each draft's `source_paths` lists the distillation file and the specific upstream artefacts its claims came from — drawn from the distillation's own `sourcePaths` (`:8-21`), narrowed to the ones that actually bear on that persona or journey (e.g. research 02 and the torn-reads concept atom for Persona 1; research 03/04 for Persona 2; research 09 for Persona 3; research 01/08 for the evaluation journey). Every path is checked to exist and to contain the attributed content, per the accepted playbook — an address that resolves says nothing about whether the claim is there. The ingest adds the draft's own `.kb/_intake/…` path on promotion (`README.md:13-19`); that extra entry is expected, not a defect. | `personas-and-journeys.md:8-21`; `.kb/playbooks/verify-the-referent-and-report-coverage.md`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` |
| **B-6 · Each draft states whether closeout contact confirmed or corrected the sketch** | One short *What closeout contact showed* note per draft: confirmed, corrected (with what changed), or untouched by this initiative — each naming the artefact it rests on. This is the re-check the source itself asks for at closeout, and an honest "untouched" is a passing answer. What is not permitted is manufacturing evidence: no download figure, adoption claim or user quotation that does not already exist in a cited artefact. | `personas-and-journeys.md:361-369`; `.kb/product/README.md:23-24` |
| **B-7 · The drafts stay problem-space** | No screen, no flow, no crate name, no trait, no feature flag, no acceptance criterion, no market sizing. A journey says what the persona is trying to accomplish and in what order; the registry and docs surfaces belong to `publication-and-positioning`, and requirements belong to `.bklg/`. The distillation's own `scope: problem-space-only` is inherited rather than relaxed. | `.kb/product/README.md:26-44`; `personas-and-journeys.md:7`, `:36-38` |
| **B-8 · The wave id is distinct and the glob is narrow** | Wave id `<staging-date>-product-audience`, distinct from `2026-08-10-intake` (`e768413`) and `2026-08-10-intake-2` (`72f2c9b`); the wave is expressed as `.kb/_intake/product-*.md`. Both are recorded in this story's `_ledger.md`, because the slice-mate needs the same strings and a bare run would sweep `.kb/_intake/README.md` into the wave. | `.kb/_intake/README.md:3-5`, `:13-19`; `project.md:301` (the risk row); `git log` for the two prior waves |
| **B-9 · `.kb/product/` is untouched by this PR** | After this story, `.kb/product/` still contains exactly one file, `README.md`, unchanged. Provenance is the observable that AC-008 checks later — atoms arriving from `.kb/_intake/` in the ingest commit — and a hand-authored atom here would make that check unanswerable no matter how good the file was. | `project.md:162-165` (DR-8), `:300` (the risk row); commit `0269720` |
| **B-10 · The DR-10/DT-1 discrepancy is written down and handed on** | A finding record under this story's folder naming: the two artefacts that disagreed, the dates and stages they were signed off at, the synthesis that settled it, and the residual question of whether HS-P0016's *published* positioning copy is consistent with a three-persona layer. Mounted as one row in `_closeout-record.md`'s *Findings* section with the destination owed to `findings-disposition-register`. The row records; it does not repair, and it does not reopen DT-1. | `project.md:179-183` (DR-12), `:306`; `_storymap.md:62`; `discover.md` *Questions* |
| **B-11 · No public interface changes** | This story exposes no Rust item, no CLI flag and no config key. `_design.md` records **no public API surface** for the project, approved 2026-08-12, so the repository's usual surface invariants are vacuous here and are not ticked. The only interface produced is the draft contract in B-3, which binds the slice-mate rather than any caller. | `_design.md` (*Items*, *Signatures*, *Visibility and stability*) |

## Data and migrations

**N/A — no schema, no store, no migration.** No program reads a persistent structure this story
writes. Three near-misses, each deliberately not a migration:

- **The staged drafts** are markdown in a directory `redkiln validate --kb` is blind to by design
  (`.kb/_intake/README.md:21-27`). They carry a *proposed* `KbFrontmatter` block (B-3), which
  introduces no new field and changes no schema — it reuses the shape already in the corpus
  (`.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30`). The real atoms, and the only
  files that must satisfy the schema, are written by the slice-mate's ingest.
- **`.kb/product/`** gains no file here, so there is no prior atom shape to migrate from and no
  existing atom to merge into — the layer is empty apart from its README
  (`initiative.md:229-233`). An ingest adjudication that *merges* one of these drafts into an
  existing atom would therefore be a signal that something went wrong, and the slice-mate should
  treat it as one.
- **`_closeout-record.md`** is a markdown record whose column contract is fixed by
  `clean-checkout-harness`; this story appends one row inside the existing *Findings* section and
  changes no section, column or ordering, so nothing downstream re-parses.

The staging is fully reversible: deleting the seven files restores the tree exactly, and the
directory is cleared by the successful ingest anyway (`.kb/_intake/README.md:13-19`), with the drafts
preserved in git history as the atoms' cited `source_paths`.

## Acceptance criteria

Three readers are crossing the stack here, and every criterion below is written from one of them.

- **The planner one initiative later.** `.kb/product/README.md:15-21` says a charter cites these
  atoms and *frames its acceptance criteria from them*. That reader is the whole point of AC-15
  (`initiative.md:350-352`): they must inherit an adjudicated audience rather than re-derive one from
  the same secondary evidence — or, worse, re-open a question two projects already closed.
- **The human at the intake approval gate.** `.kb/_intake/` is staging a person approves *before*
  `/redkiln:kb-ingest` promotes anything (`.kb/product/README.md:15-21`; `project.md` risk row
  *The kb-ingest glob and a second wave*, `:301`). They read the seven drafts, not the ingest's
  internals, and they can only approve what the drafts actually say.
- **The evaluator of the library itself**, `personas-and-journeys.md:249-262` — "decide, in a bounded
  amount of research time", cannot run the suite. They never read `.kb/`, but they are the subject of
  the fourth journey draft, and a draft that describes them wrongly is what makes the atom worthless.

A criterion satisfied by "a file with the right name exists" and not by "the reader opening it finds
what they came for, and cannot mistake it for something better evidenced than it is" has not been
met. `_intake/` abbreviates `.kb/_intake/` in the table below; `story dir` abbreviates
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/`.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **The next initiative inherits a settled audience, not a re-opened question.** GIVEN the planner who will frame acceptance criteria from `.kb/product/` and must not have to re-decide whether the evaluator is a person, WHEN the approver reviews `.kb/_intake/` before the ingest, THEN they find **exactly seven** top-level `product-*.md` drafts and nothing else new — three `concept`/persona drafts (application author, adapter author, local-first/edge developer) and four `playbook`/journey drafts whose titles are the charter's own (`initiative.md:243-250`) — with **no** persona draft for the evaluator and **no** draft in which the evaluation beats appear as a *stage* inside Persona 1's journey. Both of those alternatives were live, signed-off positions until 2026-08-12, and each is now wrong (*Context pack* 1, B-1). | `ls _intake/product-*.md` returns exactly the seven names of *Context pack* 2 and no other file is added to `_intake/`; `rg -n "^kind:" _intake/product-*.md` returns three `concept` and four `playbook`; `rg -n "^authority_tier:" _intake/product-*.md` returns seven `product`. Reviewed against `_decomposition.md:150-180` and `publication-and-positioning/_design.md:222-235`, which are the two amendments that fix the count. |
| **AC-002** | **The evaluator's moment is findable by the reader it exists to serve, and the reason it stands alone is on the page.** GIVEN the planner who opens `.kb/product/` looking for "the person deciding in one sitting" and would otherwise have to read Persona 1's journey to the end to find them, WHEN they open `product-journey-decide-in-one-sitting.md`, THEN it declares the application-author persona's atom id in `related` (and Persona 1's draft names it back, so the link survives whichever atom is read first), AND one short paragraph states the settled reason — the difference is one of *mechanism*, one-shot public evidence versus revisable contact with the code over weeks, which earns a journey; every distinguishing property is a property of a moment, not of a person, which is why it is not a persona — citing both amended artefacts by path. The paragraph *records* the decision; it does not re-argue it and does not reopen DT-1. | Reviewer opens the draft: `related` contains the application-author id; `rg -n "decide-in-one-sitting" _intake/product-persona-application-author.md` is non-empty (the reciprocal link); the reason paragraph cites `_decomposition.md` DR-10's amendment and `publication-and-positioning/_design.md:222-235`. Cross-read against `personas-and-journeys.md:333-338` (*Cross-persona tensions*, Persona 4), the un-rebutted mechanism argument. |
| **AC-003** | **What the approver reads is what the promoted atom will carry.** GIVEN the approver who can only approve what the draft says, and an ingest that *re-authors* rather than copies and is "biased hard towards merging into an existing atom" (`.kb/_intake/README.md:7-11`), WHEN they open any of the seven drafts, THEN it opens with a YAML block naming the proposed `id` (`kb-concept-…-001` / `kb-playbook-…-001`), `title`, `kind`, `status`, `authority_tier: product`, `summary`, `source_paths` and `related`, in the shape the corpus already uses — so no field the promoted atom is later checked for exists only as prose an extraction pass can paraphrase away. This cannot fail validation where it sits, by design (`.kb/_intake/README.md:21-27`), which is exactly why it must be checked by eye here. | Each of the seven files begins with a `---`-fenced block carrying all eight keys, checked field-by-field against `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30`; `redkiln validate --kb` still exits zero on the tree **without** having read any of them, which is the observable form of the `_`-prefix skip and is recorded as such rather than mistaken for a pass. |
| **AC-004** | **A reader who reads only the frontmatter cannot mistake an inference for an observation.** GIVEN the planner about to frame an acceptance criterion from an atom, WHEN they read its `summary` and stop there, THEN the last sentence is in **that atom's own voice**, names **that atom's own** evidence classes (download counts for Persona 1; the project's six skeletons and the surveyed conformance regimes for Persona 2; one blog post and one issue thread for Persona 3; research framing rather than a named individual's account for the evaluation journey), and contains **both** literal strings `secondary evidence` and `directly observed`. All seven carry it, journeys included. A copy-pasted collective "All four personas rest on secondary evidence…" fails twice: there are three personas, and it reads as someone else's claim inside an atom's own summary (DR-9, `project.md:166-170`; B-4). | `rg -c "secondary evidence" _intake/product-*.md` and `rg -c "directly observed" _intake/product-*.md` each return 7 files × ≥1; a reviewer confirms for each file that both strings fall **inside** the `summary` scalar and not merely in the body, and that the sentence names evidence classes specific to that draft. `rg -n "All four personas" _intake/product-*.md` returns **zero** hits. |
| **AC-005** | **Every claim can be walked back to the artefact it came from, and the artefact really says it.** GIVEN the planner who distrusts an inherited persona and wants to check one sentence of it, WHEN they follow a draft's `source_paths`, THEN each path exists **and contains the attributed content** — the distillation file plus the specific upstream artefacts bearing on that persona or journey, narrowed from the distillation's own `sourcePaths` (`personas-and-journeys.md:8-21`) rather than copied wholesale, so a path list is a claim about provenance and not a decoration. An address that resolves says nothing about whether the claim is there (`.kb/playbooks/verify-the-referent-and-report-coverage.md`). The extra `.kb/_intake/…` entry the ingest adds on promotion (`README.md:13-19`) is expected and is not a defect. | For every path in every draft's `source_paths`: `test -f <path>` passes, and the referent check is performed and its coverage reported — n paths checked of n cited, per the accepted playbook. Narrowing is visible: no two drafts carry an identical `source_paths` list, and none reproduces all fourteen entries of `personas-and-journeys.md:8-21`. |
| **AC-006** | **A promotion dated at closeout is not a pre-initiative guess wearing a closeout date.** GIVEN the reader who knows `.kb/product/README.md:23-24`'s bar ("a persona nobody researched is a stock photo with a name") and reads the source's own instruction that it "should be re-checked at closeout once real contact… exists to confirm or correct them" (`personas-and-journeys.md:361-369`), WHEN they open any draft, THEN it carries one short *What closeout contact showed* note saying **confirmed**, **corrected** (with what changed), or **untouched by this initiative**, each naming the artefact from *this* initiative it rests on. An honest "untouched" passes; a silent omission does not; and no download figure, adoption number or user quotation appears that is not already in a cited artefact. | Seven drafts, seven notes: `rg -n "What closeout contact showed" _intake/product-*.md` returns 7. Each note's cited artefact is opened and confirmed to bear on the sketch. A reviewer greps the drafts for numerals and quoted speech and traces each to a cited artefact; anything untraceable is manufactured evidence and fails this criterion (EC-4). |
| **AC-007** | **The audience layer stays usable after the API and the registry page move.** GIVEN the planner reading a journey two initiatives from now, WHEN they read any draft end to end, THEN it says what the persona is trying to accomplish and in what order, and contains **no** screen, flow or interaction pattern, **no** crate, trait, method or feature-flag name, **no** acceptance criterion or scope statement, and **no** market sizing — the four exclusions of `.kb/product/README.md:26-44`, in the Rust-shaped form they take here. The distillation's own `scope: problem-space-only` (`personas-and-journeys.md:7`) is inherited, not relaxed; the crates.io and docs.rs surfaces belong to `publication-and-positioning`'s `_design.md`. | `rg -n "EventStore\|event_store_conformance\|SendEventStore\|happenstance-core\|happenstance-testkit\|feature =\|#\[cfg" _intake/product-*.md` returns zero hits; a reviewer reads each draft for the two exclusions a grep cannot catch (an acceptance criterion in prose, a market claim). Where a pain point has an obvious technical fix, the fix is left out and flagged, exactly as `personas-and-journeys.md:36-38` does. |
| **AC-008** | **The ingest that follows has one wave to consume and nothing pre-empted.** GIVEN the slice-mate about to run `/redkiln:kb-ingest`, and a bare run that would sweep `.kb/_intake/README.md` into the wave and clear the directory on success (`README.md:3-5`, `:13-19`), WHEN they read this story's `_ledger.md`, THEN it declares the wave id `2026-08-12-product-audience` — distinct from `2026-08-10-intake` (`e768413`) and `2026-08-10-intake-2` (`72f2c9b`), whose commit subjects and telemetry run files are keyed on theirs — and the narrowing glob `.kb/_intake/product-*.md`, AND `.kb/product/` is byte-identical to its pre-PR state (one file, `README.md`), so AC-008's provenance check downstream still has something to observe. Every draft is at the **top level** of `_intake/`: a subdirectory is not read by the wave glob and is therefore not mounted. | `git diff --stat main -- .kb/product/` is empty and `ls .kb/product/` returns `README.md` only; `_ledger.md` carries both strings verbatim; `git log --oneline` confirms the two prior wave ids are taken; `ls _intake/` shows the seven drafts plus `README.md` and no subdirectory. |
| **AC-009** | **The initiative closes without entrenching a contradiction in a durable layer.** GIVEN the reader of the closeout record who must be able to see that a cross-project disagreement was *found*, not absorbed (`project.md` DR-12, `:179-183`; risk row `:306`), WHEN they open `_closeout-record.md`'s *Findings* section, THEN one row records the DR-10/DT-1 discrepancy — the two artefacts, the stages and dates they were signed off at, the synthesis that settled it, and the residual question of whether HS-P0016's *published* positioning copy is consistent with a three-persona layer — pointing at the finding written under this story's folder, with the **destination cell left owed** to `findings-disposition-register`. The row records; it does not repair, does not reopen DT-1, and does not edit either signed-off artefact beyond the owner's amendment already in them. | `story dir/_finding-dr10-dt1.md` exists and names both artefacts with `file:line`, both sign-off dates, the synthesis and the residual question; `_closeout-record.md`'s *Findings* section gains exactly one row citing it, with a destination cell reading as owed rather than filled; `git diff main -- .bklg/.../closeout-and-durable-audience/_decomposition.md .bklg/.../publication-and-positioning/_design.md` is empty in this PR. |

**Coverage of the traced project AC.** `project.md` **AC-010** ("the evaluator question is decided
before authoring, with the reasoning, and the set of promoted atoms matches that decision") is
discharged across AC-001, AC-002 and AC-009: AC-001 is the *set matches the decision* half made
observable in the staged files, AC-002 is the *with the reasoning* half carried where the promoted
atom will keep it, and AC-009 is the routing the brief's own DR-12 language demanded instead of a
silent overwrite. The remaining six criteria are the properties the slice-mate's promotion can only
inherit — AC-003, AC-004, AC-005 for what project AC-007/AC-009 will be checked for, AC-006 for
`.kb/product/README.md`'s evidence bar, AC-007 for the layer's own exclusions, AC-008 for the
provenance project AC-008 observes.

## Interaction quality

This story renders **no screen and no public API**. `_design.md` is approved with *no public API
surface* — `N/A` against every item block, sign-off 2026-08-12 — so the project's signed-off design
contributes **no composition invariants to inherit**, and inventing some here would be fabricating a
design a human never approved (*Context pack* 11). What this story *does* render is a set of
documents a reader meets: seven drafts at the mount point `.kb/_intake/`, read by the approver
before the ingest and re-read as atoms by every later initiative. The invariants below are what make
them legible rather than merely present.

**Every invariant here is already carried by an AC row in the table above.** None is a prose-only
bullet: `redkiln verify` extracts criteria by matching a leading `| AC-001 |` table cell or an
`- AC-001:` bullet, so an invariant stated only in this section would get no ledger row, would never
be gated, and would never be checked.

**State family.**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not context-jump** — the drafts are added to the one directory the ingest already reads; no new directory, no subdirectory under `_intake/`, no parallel staging area under the story folder | AC-008 | `ls _intake/` shows seven `product-*.md` files plus the pre-existing `README.md` and no subdirectory; the PR boundary's first line is `.kb/_intake/product-*.md` |
| **Non-occlusion** — nothing this story writes hides or displaces what is already there: `.kb/_intake/README.md` stays, and the `product-` prefix is what lets the wave exclude it instead of consuming it | AC-008 | `git diff main -- .kb/_intake/README.md` is empty; the declared glob in `_ledger.md` cannot match it |
| **Preserved selection** (the document analogue) — the journey titles are the charter's own (`initiative.md:243-250`) and the persona names are the distillation's, so a citation written against either survives promotion; nothing is renamed in passing | AC-001, AC-005 | Titles compared literally against `initiative.md:243-250`; `source_paths` entries resolve and bear the attributed content |
| **Reversibility** — deleting the seven files restores the tree exactly; `.kb/product/` is untouched, so nothing downstream has yet consumed anything | AC-008 | *Data and migrations* above; `git diff --stat main -- .kb/product/` empty |
| **Reachable without a tool** (the analogue of keyboard reachability) — every draft is plain UTF-8 markdown readable in any editor; nothing load-bearing is expressible only through `redkiln` output, and `validate --kb` is blind to this tree by design | AC-003 | The drafts are read by eye at the approval gate; `redkiln validate --kb` green *without* having read them is recorded as the skip it is, not as a pass |

**Composition family.** `_design.md` declares none, so these are taken from the layer's own contract
(`.kb/product/README.md:6-13`, `:26-44`), the corpus's atom shape and the two accepted playbooks —
not invented here.

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — each draft is a composed atom-shaped document (frontmatter block, then the body the layer asks for), never a pasted excerpt of `personas-and-journeys.md` with a filename on it | AC-003, AC-007 | The eight frontmatter keys are present in all seven; the body follows the layer's own shape — persona: goal, context, what they do instead, what they fear (`.kb/product/README.md:11-13`); journey: the moment-by-moment beats in order |
| **Composition and placement** — drafts at the top level of `_intake/` (the only place the wave glob reads); the finding under the *story* folder because it is this story's provenance; its one-row summary in the *project*'s `_closeout-record.md` because it is read across stories | AC-008, AC-009 | The three paths in the PR-boundary fenced block are exactly these three locations |
| **Transience** — the drafts are deliberately *transient* chrome: a successful ingest clears them and they survive only in git history as the atoms' cited `source_paths` (`.kb/_intake/README.md:13-19`). The finding and its `_closeout-record.md` row are **persistent** and outlive the wave | AC-008, AC-009 | *Data and migrations* above; the finding is not staged in `_intake/`, so the ingest cannot clear it |
| **Density budget, with its real numbers** — seven drafts; one frontmatter block each with eight keys; exactly one qualification sentence per `summary`, carrying both literal strings; exactly one *What closeout contact showed* note per draft; one `related` link per journey to its persona and back; one finding file; one appended row. Each draft is sized to re-author into a canonical atom of roughly 100 lines (`CLAUDE.md`, on the decision atoms: "canonical, ~100 lines each") — a draft that cannot be is over-stuffed, and the ingest will paraphrase the surplus rather than keep it. An eighth draft, or a second row in *Findings*, means someone re-shaped the story | AC-001, AC-004, AC-006, AC-009 | Counted: 7 files, 7 notes, 7 × 2 literal strings, 1 finding, 1 row |
| **Hierarchy** — the qualification is the **last** sentence of `summary`, so the frontmatter reader meets it last and cannot scroll past it; the closeout note sits after the body it qualifies, not in front of it; the evaluation journey's "why not a persona" paragraph is one paragraph, not a section competing with the journey itself | AC-002, AC-004, AC-006 | Read in order at review; the `summary` scalar's final sentence is the qualification in all seven |
| **Named anti-patterns** — (a) a fourth *persona* draft; (b) the evaluation beats folded as a stage inside Persona 1's journey draft; (c) one collective "All four personas rest on secondary evidence…" sentence pasted into all seven summaries; (d) writing any file under `.kb/product/` (commit `0269720`); (e) a journey that names a crate, trait or feature flag — the API transcribed instead of the task; (f) a closeout note that invents a figure or a quotation; (g) a filename without the `product-` prefix, or a draft in a subdirectory, either of which unmounts it or sweeps `README.md` into the wave | AC-001 (a, b), AC-004 (c), AC-008 (d, g), AC-007 (e), AC-006 (f) | Each is a review check with a named artefact: (a)/(b) against `_decomposition.md:150-180`; (c) `rg -n "All four personas"` returns zero; (d) `git diff --stat main -- .kb/product/` empty; (e) the identifier grep in AC-007; (f) numerals traced to cited artefacts; (g) `ls _intake/` |

## Error conditions

| id | Condition | Required response |
| --- | --- | --- |
| **EC-1** | A source path a draft wants to cite does not exist, or exists but does not contain the attributed content | Do **not** cite it. Narrow the claim to what a real artefact supports, or drop the claim. A `source_paths` list whose referents do not bear the content is the exact failure `.kb/playbooks/verify-the-referent-and-report-coverage.md` was accepted against, and it survives promotion into a durable layer where nobody re-checks it. Report the coverage — paths checked over paths cited — rather than asserting the list is clean. |
| **EC-2** | The distillation is silent on something a draft needs (a persona's fear, a journey's third beat) | Leave it out and say so. `personas-and-journeys.md:371-395` files five questions as open, two of which this story inherits (*Explicitly not in this PR*); a draft carries them as **stated-open**, never silently settled. Inventing the missing beat converts an admitted gap into an inherited claim — the failure `.kb/product/README.md:26-31` calls "giving a guess the standing of a finding". |
| **EC-3** | A draft's content pulls toward naming a crate, trait or registry surface — the obvious way to make a journey concrete | Rewrite in problem-space terms (AC-007). If the beat cannot be stated without the API, it is a `design/` or `publication-and-positioning` concern and does not belong in this layer. `personas-and-journeys.md:36-38` already did this once for the same material and is the precedent to follow. |
| **EC-4** | The closeout re-check produces nothing — this initiative genuinely never made contact bearing on a persona | Write **"untouched by this initiative"** and name what would have counted. That is a passing answer (AC-006, B-6). Manufacturing a download figure, an adoption claim or a user quotation to fill the note is the one response that turns an honest sketch into a fabricated finding, and it is unrecoverable once promoted. |
| **EC-5** | A file lands in `.kb/_intake/` without the `product-` prefix, or inside a subdirectory | Rename or move it before the slice-mate runs. Without the prefix the narrowing glob cannot exclude `README.md`, and the bare run sweeps the directory's own documentation into the wave (`project.md:301`); inside a subdirectory the file is simply never read, and the story's mount claim is false (*Integration contract*, mount point). |
| **EC-6** | The wave id collides with `2026-08-10-intake` or `2026-08-10-intake-2` | Pick a fresh id and record it in `_ledger.md` before the slice-mate runs. The id keys the commit subject and the telemetry run file (`.redkiln/telemetry/events/`), so a collision overwrites the first wave's audit trail — the second half of the risk row at `project.md:301`. |
| **EC-7** | The slice-mate's ingest adjudicates one of these drafts as a **merge into an existing atom** | Treat it as a signal, not an outcome. `.kb/product/` is empty apart from its README (`initiative.md:229-233`), so there is nothing legitimate to merge into; a merge means the adjudication matched something in another layer and the promoted atom will not be the persona atom AC-007 requires. Halt and re-read the draft's frontmatter block before re-running. |
| **EC-8** | The DR-10/DT-1 discrepancy tempts a repair — editing the `testing` brief's DR-10 body, or `publication-and-positioning`'s DT-1 | Record and route (AC-009). Both artefacts already carry the owner's 2026-08-12 amendment; a downstream story rewriting a signed-off brief is the inverse of DR-12's rule, and `findings-disposition-register` owns the disposition (`project.md:179-183`, `:306`). |
| **EC-9** | `redkiln validate --kb` fails while the drafts are staged | It is not the drafts: `_`-prefixed directories are skipped (`.kb/_intake/README.md:21-27`). Investigate elsewhere and route the finding; do **not** "fix" it by deleting or reshaping a staged file, which would hide a real defect behind an unrelated change. |

## Non-functional

| id | Requirement | Why, and how it is checked |
| --- | --- | --- |
| **NF-1** | **Zero code, zero runtime cost.** No crate, no `xtask/`, no CI workflow, no spec clause changes; the library's compile time, binary size and MSRV are untouched. | `cargo xtask affected --base main` names no affected package — the observable form of `project.md` *Out of scope* and of the `testing` brief's AC-013 row ("zero commits inside this project's own stories touch code outside `.bklg/`/`.kb/` planning artefacts"). |
| **NF-2** | **Durability over convenience.** These drafts become atoms "authored once, referenced by many initiatives; never re-derived per story" (`.kb/product/README.md:3-4`). Every sentence is written to be read years later by someone with no access to this initiative's context. | Each claim carries its citation in `source_paths` and its qualification in `summary`; nothing load-bearing lives only in this spec or in a commit message, both of which the promoted atom does not carry. |
| **NF-3** | **Honesty over completeness.** A thinner draft that states what it does not know beats a fuller one that infers. Persona 3 rests on one blog post and one issue thread; the evaluation journey is inferred from research framing rather than a named individual's account (`personas-and-journeys.md:349-355`). Both say so in their own voice. | AC-004's per-atom sentence and AC-006's note; EC-2 and EC-4 are the two ways this is violated and both name the passing alternative. |
| **NF-4** | **The staging is cheap to review.** Seven files, each re-authorable into ~100 lines, readable in one sitting by the approver at the gate. The approval is a human act and a draft nobody can finish reading is a draft nobody approved. | The density budget in *Interaction quality*; `wc -l` on the seven drafts is proportionate to the ~100-line atom target `CLAUDE.md` records for the decision corpus. |
| **NF-5** | **No credential, no private correspondence, no unpublished third-party material** enters a layer intended to be published with the repository. Every persona claim traces to a public research artefact already in the tree. | `source_paths` entries are all in-repo paths under `.bklg/…/_discovery/` or `.kb/`; the drafts quote no private communication. |

## Implementation notes (non-prescriptive)

Not requirements — the shape this spec's author expects, offered so the implementer can disagree with
something concrete.

- **Write the frontmatter block first, then the body against it.** The `summary` is the field the
  promoted atom is checked on (AC-004) and the field an extraction pass is most likely to rewrite;
  drafting it first makes the body an expansion of a settled claim rather than the other way round.
- **Copy the shape, not the content, from `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30`.**
  It is the in-tree reference for the eight keys, the `>-` folded summary and the
  `kb-<kind>-<slug>-001` id convention. `.kb/playbooks/*.md:1-8` is the `playbook` variant.
- **Draft the three personas before the four journeys.** Each journey's `related` needs its persona's
  id, and the evaluation journey needs Persona 1's specifically (AC-002). Writing journeys first
  produces dangling links that get "fixed" by inventing ids.
- **Keep the qualification sentence to one sentence, last in `summary`.** Two sentences invite the
  extraction pass to keep one; a sentence in the middle of the summary is a sentence a skimmer stops
  before. Both literal strings must survive re-wording, which is why AC-004 fixes them literally.
- **A journey's beats are the source's, re-ordered at most.** `personas-and-journeys.md` already
  writes each persona's journey as *today* and *what the initiative should improve*; the promoted
  journey is the moment-by-moment path, and the "should improve" half is initiative-scoped and mostly
  belongs in `.bklg/`, not in the atom.
- **The finding is a document, not a paragraph in this spec.** Write
  `story dir/_finding-dr10-dt1.md` with the two artefacts, their sign-off dates and stages, the
  synthesis, and the residual question about HS-P0016's published copy; the `_closeout-record.md` row
  is a one-line pointer at it under the column contract `clean-checkout-harness` fixed. Do not
  restate the argument in the row.
- **Record the wave id and the glob in `_ledger.md` before the slice-mate starts.** They are the two
  strings the ingest run needs and the two most likely to be re-invented in a second context.
- **Resist the collective sentence.** DR-9's wording ("All four personas rest on secondary
  evidence…") is a *project-level* statement and reads perfectly in `project.md`. Pasted into an
  atom's own summary it is false about the count and wrong about the voice — AC-004 exists because
  this is the single most likely way the qualification lands and still fails.

## Tests and CI (merge gate)

The `testing` brief (`_decomposition.md` *Test mix, summarised by tier*) is explicit that **static**
and **process** are the load-bearing tiers here: this project audits and records, it does not build.
There is no unit test to write, and writing one would mean this story had grown code it is forbidden
to have (NF-1). `_intake/` abbreviates `.kb/_intake/` below.

| Tier | Command / path | Proves |
| --- | --- | --- |
| **Static** | `ls _intake/product-*.md` (7 files, names per *Context pack* 2); `rg -n "^kind:" _intake/product-*.md` (3 × `concept`, 4 × `playbook`); `rg -n "^authority_tier:" _intake/product-*.md` (7 × `product`) | AC-001 — the set matches the amended decision, in kind and in count |
| **Static** | Review of `_intake/product-journey-decide-in-one-sitting.md` for the `related` link and the mechanism paragraph, plus `rg -n "decide-in-one-sitting" _intake/product-persona-application-author.md` for the reciprocal | AC-002 — the evaluation path is a first-class journey linked to Persona 1, with the settled reason on the page |
| **Static** | Field-by-field review of each draft's frontmatter block against `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30` | AC-003 — every field the promoted atom is checked for exists in the draft rather than in prose |
| **Static** | `rg -c "secondary evidence" _intake/product-*.md`, `rg -c "directly observed" _intake/product-*.md` (7 files each), `rg -n "All four personas" _intake/product-*.md` (zero), plus a reviewer confirming both strings sit inside the `summary` scalar | AC-004 — the qualification is per-atom, in-voice, in frontmatter, and greppable at promotion time |
| **Static / process** | `test -f` over every `source_paths` entry, then the referent check with its coverage reported, per `.kb/playbooks/verify-the-referent-and-report-coverage.md` | AC-005 — the citations resolve *and* bear the attributed content |
| **Static** | `rg -n "What closeout contact showed" _intake/product-*.md` (7); each note's cited artefact opened; numerals and quoted speech traced to a cited artefact | AC-006 — the sketches were re-checked at closeout, honestly, including where nothing changed |
| **Static** | `rg -n "EventStore\|event_store_conformance\|SendEventStore\|happenstance-core\|happenstance-testkit\|feature =\|#\[cfg" _intake/product-*.md` (zero hits), plus a read-through for the two exclusions a grep cannot catch | AC-007 — the drafts stay problem-space and outlive the API and the registry page |
| **Process** | `git diff --stat main -- .kb/product/` (empty); `ls .kb/product/` (`README.md` only); `ls _intake/` (7 drafts + `README.md`, no subdirectory); `_ledger.md` carrying the wave id and glob verbatim; `git log --oneline` for the two prior wave ids | AC-008 — nothing was hand-authored into the durable layer, and the wave is distinct, narrow and top-level |
| **Process** | `story dir/_finding-dr10-dt1.md` reviewed for both artefacts, dates, synthesis and residual question; the one appended row in `_closeout-record.md`'s *Findings*; `git diff main` empty for `_decomposition.md` and `publication-and-positioning/_design.md` | AC-009 — the discrepancy is recorded and handed on, not absorbed and not re-litigated |
| **Static (negative)** | `redkiln validate --kb` exits zero with the drafts staged | The `_`-prefix skip holds (`.kb/_intake/README.md:21-27`) — recorded as the *skip* it is, never cited as evidence the drafts are well-formed (AC-003) |
| **E2E (slice-mate, same slice)** | `/redkiln:kb-ingest .kb/_intake/product-*.md` under wave id `2026-08-12-product-audience`, then `redkiln validate --kb` on the promoted atoms | Consumes AC-001…AC-008 for real: the mount is demonstrated by the directory being cleared and seven atoms appearing under `.kb/product/`. Owned by `product-atom-promotion-via-kb-ingest` (project AC-007, AC-008, AC-009); listed here because this story fixes the wave id and glob it must use |
| **Merge gate (story grain)** | `redkiln verify --grain story` — reads `_ledger.md`, the PR-boundary fenced block and commit provenance (`.redkiln/config.yaml:67`, `:73`) | Every AC has a ledger row with cited evidence, and no file changed outside the three-line boundary |
| **Merge gate (affected)** | `cargo xtask affected --base main` | NF-1 — no crate is affected, the observable form of "this project owns no code" |

The four merge-gate commands the `testing` brief reserves for this project (`cargo xtask ci`,
`redkiln validate`, `redkiln validate --kb`, `redkiln doctor --json`) are **not** this story's bar.
The first belongs to `whole-gate-green-on-the-assembled-tree` in slice 1; the last three to
`backlog-and-kb-health-at-closeout` in slice 5, which must run *after* the promoted atoms exist
(`_storymap.md:149-151`). Running them here would not be wrong; citing them as this story's evidence
would be, because this story's claim is about what the staging says, not about what passes.

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **Deferring to the brief and staging four personas** | Named in `discover.md` *The wrong implementation*: it satisfies AC-010's literal first clause, passes a reviewer who checks only that a decision was made, and entrenches a contradiction with a sibling's signed-off, already-published resolution in a layer the next initiative reads as settled. Caught by AC-001, which counts kinds rather than files, and by the two amendments it is reviewed against. |
| **The inverse error: folding the evaluation beats into Persona 1's journey** | Equally live, because `publication-and-positioning/_design.md`'s *original* consequence wording said exactly that, and it names this project by slug as the executor. The 2026-08-12 rider amended it (`:222-235`). A reader who opens only the sibling's pre-amendment text will implement the wrong half of the synthesis. |
| **Coupling to the slice-mate is tight and deliberate** | AC-008's wave id and glob are inputs to a run this story does not perform, and the mount is only *demonstrated* when that run clears the directory. `_storymap.md:81-84` keeps them in one slice for exactly this reason: "Holding the two apart is how `0269720` happened." Do not close this story's ledger on the strength of the strings being *recorded* rather than *consumed*. |
| **The ingest re-authors, and the qualification is the most losable thing in the wave** | Extraction paraphrases and adjudication is biased toward merging (`.kb/_intake/README.md:7-11`). AC-003 and AC-004 are the mitigation — the field exists in the draft and the two literal strings are fixed — but nothing mechanical enforces that the promoted atom kept them. That check is project AC-009's, in the slice-mate, and it is the reason the strings are literal rather than descriptive. |
| **`_closeout-record.md` may not exist yet when this story runs** | It is created by `clean-checkout-harness` (slice 1), which precedes slice 4 in merge order (`_storymap.md:140-151`), and its *Findings* section with a destination column and no fix column is fixed there. If the slices are run out of order, AC-009's row has nowhere to land — write the finding file regardless and append the row once the record exists; do **not** create a second record, which is the fourteen-places failure mode `_storymap.md:24-30` writes against. |
| **Pressure to fix what the finding names** | The project's standing risk (`project.md` risk table, *Pressure to fix what re-observation finds*) lands here as the temptation to tidy a signed-off brief. AC-009 makes recording the deliverable; EC-8 names the repair and forbids it. |
| **Two open questions travel into the drafts unanswered** | Whether "adapter author" is purely internal for this timeframe, and whether Persona 3's orphaned-tooling fear is owed a proof artefact (`personas-and-journeys.md:378-391`). Carried as stated-open (EC-2). The risk is that a draft written for readability quietly settles one; the mitigation is that both are named in the *Explicitly not in this PR* list, so their absence is deliberate rather than an omission. |
| **Evidence thinness is inherited, not created here** | Persona 3 and the evaluation journey are "the two least-tested against real evidence" (`personas-and-journeys.md:349-355`). This story cannot improve that; it can only refuse to hide it (NF-3, AC-004, AC-006). A draft that reads as confidently as Persona 1's is the tell that it was smoothed. |
| **Slice 5 measures the tree this slice changes** | `backlog-and-kb-health-at-closeout` runs `validate --kb` and `doctor --json` after the promoted atoms exist, which is why slice 4 precedes it. A malformed promoted atom surfaces there as a health failure rather than here — one more reason AC-003's frontmatter block is checked by eye at staging, where `validate --kb` deliberately cannot help. |

## Dependencies

**Blocks on:** nothing. `depends_on: []`, matching `_storymap.md:59` and the merge-order note at
`:149-151` ("Independent of slices 1–3"). At the *project* level HS-P0019 depends on all nine
siblings having merged, but that is the project's edge, discharged before any story here runs. The
one soft ordering that matters is `_closeout-record.md`'s existence (slice 1) for AC-009's row —
recorded in *Risks and coupling* rather than as a `depends_on`, because the finding file itself is
writable without it.

**Unlocks:**

- `product-atom-promotion-via-kb-ingest` — same slice, immediately. It consumes exactly these seven
  files under the wave id and glob this story declares, and it is what makes the mount real
  (project AC-007, AC-008, AC-009; `_storymap.md:60`).
- `backlog-and-kb-health-at-closeout` — slice 5. The promoted atoms are inputs to
  `redkiln validate --kb`, which is why slice 4 must precede it (`_storymap.md:149-151`).
- `findings-disposition-register` — slice 5. It routes the DR-10/DT-1 finding this story writes;
  the destination cell is deliberately left owed to it (AC-009, project AC-013).
- `initiative-closeout-readiness` — slice 5. The initiative's own closeout links the promoted product
  atoms (project AC-014; `initiative.md:405-407`), which do not exist until this staging is ingested.

## Anchors (progressive disclosure)

Read the *Context pack* first — it is complete enough to start from. Open a row below only when the
AC it serves is the one being worked. Nothing here is optional detail; it is deferred detail.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The one warranted `testing` brief. `:148-181` is DR-10's **amendment** — the synthesis, what survived (argument 1, the mechanism difference), what did not (that it makes a different person), and why argument 3 was discounted as circular. `:227-236` is the amended consequence that fixes the promoted set at three personas plus four journeys. The superseded reasoning below it is preserved on purpose: the reasons it lost only make sense beside it. | **Before authoring any draft** — this is where the count comes from, and reading only the superseded text produces the exact wrong implementation `discover.md` names | AC-001, AC-002 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The sibling's signed-off DT-1 resolution at `:214-222` and its 2026-08-12 rider at `:222-235`, including the sentence that forbids the *other* wrong implementation: "a stage buried inside Persona 1's journey would make that mechanism unfindable to the reader it exists to serve". Read the rider, not just the resolution — the pre-amendment text names this project as the executor of the opposite decision. | Before writing `product-journey-decide-in-one-sitting.md`, and again before writing the finding | AC-002, AC-009 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The sole source of persona content: Persona 1 at `:42-110`, Persona 2 at `:114-178`, Persona 3 at `:182-245`, the evaluator at `:249-313`; *Cross-persona tensions* `:317-347`; *Risks* `:349-369` (the evidence-quality statement AC-004's sentences must be honest about, and the closeout re-check instruction AC-006 executes); *Open questions* `:371-395` (the two carried in as stated-open); `sourcePaths` at `:8-21`, which each draft's `source_paths` is narrowed from. | Continuously while drafting — it is the material being staged, and nothing may be added to a draft that is not derivable from it | AC-001, AC-004, AC-005, AC-006, AC-007 |
| `.kb/product/README.md` | The layer's own contract: `:6-9` persona = `concept`, journey = `playbook`, both `authority_tier: product`; `:11-13` what each *is*, which is the body shape; `:15-21` that a charter frames acceptance criteria from these atoms and that closeout performs the promotion; `:23-24` the evidence bar; `:26-44` the four exclusions. | Before choosing each draft's `kind`, and again before every read-through for AC-007 | AC-001, AC-003, AC-006, AC-007 |
| `.kb/_intake/README.md` | `:3-5` the wave glob and the bare-run default; `:7-11` "not held to `KbFrontmatter`" plus the two dangerous properties of the run (extraction paraphrases; adjudication is biased hard toward merging); `:13-19` the clears-on-success contract and the `source_paths` back-reference; `:21-27` the `_`-prefix skip that makes a proposed-frontmatter block safe to stage. | Before writing the frontmatter block, and before declaring the wave id and glob | AC-003, AC-008 |
| `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | `:1-30` is the in-tree reference for the frontmatter shape a promoted atom carries — the eight keys, the `>-` folded `summary`, the `kb-<kind>-<slug>-001` id convention, `source_paths` as in-repo paths. Copy the shape; the content is unrelated. | While writing each draft's YAML block | AC-003 |
| `.kb/playbooks/verify-the-referent-and-report-coverage.md` | The accepted playbook behind AC-005: an address that resolves says nothing about whether the attributed content is there, and a check that does not state what fraction it parsed is indistinguishable from one that sees everything. | While assembling every `source_paths` list, and when reporting the check | AC-005 |
| `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | Why a bare `file:line` rots, and why a citation needs a subject string. These drafts become atoms referenced by many initiatives (`.kb/product/README.md:3-4`), so every citation in them is written against a tree that will move. | While writing any in-body citation, and while writing the finding | AC-005, AC-009 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | DR-8 (`:162-165`) the ingest-only path, DR-9 (`:166-170`) the qualification in frontmatter, DR-10 (`:171-174`) decided-before-authoring, DR-12 (`:179-183`) findings routed not absorbed; AC-010 (`:225-228`) the traced criterion; the risk rows at `:300-301` (hand-authoring; the glob and a second wave) and `:306` (the evaluator decision's downstream reach). | Before staging anything, and again before the finding | AC-004, AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/persona-and-journey-intake-staging/discover.md` | *The wrong implementation* — staging four personas because the brief said so — stated more precisely than anywhere else, including why it passes a reviewer; and *Questions*, whose second answer is superseded in place by the owner's 2026-08-12 update. Read both the superseded paragraph and the update: the update only makes sense against what it replaced. | **First**, before any file is written — it is the shortest statement of what failure looks like | AC-001, AC-009 |
| `.bklg/from-contract-to-published-library/initiative.md` | `:227-258` *Referenced personas & journeys*: the four journey titles verbatim (`:243-250`), the promotion flag, and the instruction that the secondary-evidence qualification "travels with them into the KB"; `:350-352` AC-15; `:405-407` DoD 16. | While naming the journey drafts, and when checking AC-004's wording against the charter's own | AC-001, AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | `:59` this story's row with its inline 2026-08-12 correction; `:60` the slice-mate's; `:81-84` why the slice is cut whole and the `0269720` sentence; `:100-106` the *stale* grain note still asserting four personas, which is the last uncorrected copy of the superseded text and must not be followed; `:149-151` merge order. | Before assuming what the slice-mate does, and if the persona count seems contested | AC-001, AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/spec.md` | The slice-mate's own spec — what it will do with these files, and which of its criteria depend on a field being present in the draft rather than inferred by the ingest. Reading it prevents writing a draft that is correct in isolation and unusable to the run that consumes it. | Before finalising the frontmatter block and the wave declaration | AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The approved *no public API surface* determination, sign-off 2026-08-12. Load-bearing **negatively**: it is why the composition family above inherits nothing from a signed-off design and why no surface invariant may be ticked as met. | If a reviewer asks why this spec declares no design-derived invariants | AC-003 |
| `.redkiln/config.yaml` | `:67` `require_ledger` and `:73` `require_commit_provenance` — why `_ledger.md` must cite the seven staged paths and carry the wave id and glob; `:40` `affected_gate`, NF-1's check; `:5` `support_initiative`, one of the destinations the finding may eventually route to. | Before filling `_ledger.md` | AC-008, AC-009 |
| `.bklg/from-contract-to-published-library/_decomposition.md` | The initiative-level traceability matrix (BR-16 → HS-P0019), the DAG and *Decisions taken at the gate*. Confirms this story's material is the initiative's own discovery output and that no other project owns the product layer. | If the scope of "the audience" is questioned, or a second owner is suspected | AC-001 |

## Clarifications resolved during spec

1. **AC count unchanged, and the mapping from the behaviour table is explicit.** The nine ids the
   first pass enumerated (AC-001…AC-009) are exactly the nine written here; none was added or
   dropped. B-1 → AC-001, B-2 → AC-002, B-3 → AC-003, B-4 → AC-004, B-5 → AC-005, B-6 → AC-006,
   B-7 → AC-007, **B-8 and B-9 together → AC-008** (the wave is distinct and narrow *and*
   `.kb/product/` is untouched — one criterion, because both are the same promise: the ingest that
   follows has exactly one wave to consume and nothing pre-empted), B-10 → AC-009. B-11 ("no public
   interface changes") gets no row deliberately: `_design.md` records no surface, so a row asserting
   the absence of one would be the decorative criterion `CLAUDE.md` forbids.
2. **The interaction-quality invariants were mapped onto existing rows rather than given rows of
   their own.** Every one of them is a property of the drafts' composition (AC-003, AC-007), of the
   qualification's placement (AC-004), of the closeout note (AC-006), of the mount (AC-008) or of the
   finding (AC-009) that those criteria already assert. A tenth row would have restated one — and a
   bullet in that section alone would never be extracted by `redkiln verify`, never gated, never
   tested.
3. **The wave id is fixed here, not left to the slice-mate.** `2026-08-12-product-audience`, from the
   `<staging-date>-product-audience` form of *Context pack* 5. Fixed because the id keys the commit
   subject and the telemetry run file, and two contexts inventing it independently is exactly how the
   first wave's audit trail gets overwritten (`project.md:301`, EC-6).
4. **The finding's path is fixed here** as `story dir/_finding-dr10-dt1.md`, inside the PR boundary,
   because `_closeout-record.md`'s row cites it and a renamed artefact is a dead citation
   (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`).
5. **The qualification's two literal strings are fixed, not paraphrased.** `secondary evidence` and
   `directly observed`, both present in each `summary`. Fixing them makes project AC-009 a `grep`
   over seven promoted atoms instead of a reading exercise, and it is the only property of the
   qualification that survives an extraction pass that rewords everything around it.
6. **The journeys carry the qualification too**, although DR-9's sentence names only personas
   (`project.md:166-170`). A journey promoted without it is the one atom in the layer that reads as
   first-hand — and the evaluation journey is the thinnest-evidenced of the seven
   (`personas-and-journeys.md:349-355`), which makes it the worst possible place for the omission.
7. **`_storymap.md:100-106` is stale and is not followed.** The *Grain notes* still say "the promoted
   set is **four** personas plus their journeys" and that "a three-persona staging is wrong against
   the brief". That text is downstream of DR-10's superseded consequence; the row at `:59` carries
   the correction inline. Recorded here so an implementer who reads the story map top-to-bottom does
   not treat the later paragraph as the newer one. Editing the story map is not this story's to do
   (frontmatter and body of a signed-off planning artefact both belong to the stage that produced
   it); the discrepancy is one more line in the finding.
8. **Not decided here, deliberately**: whether "adapter author" is purely internal for this
   timeframe, and whether Persona 3's orphaned-tooling fear is owed a proof artefact
   (`personas-and-journeys.md:378-391`). Both travel into the drafts as stated-open (EC-2). Also not
   decided: the disposition of the DR-10/DT-1 finding, which is `findings-disposition-register`'s
   (project AC-013), and whether HS-P0016's *published* positioning copy needs a change — that is a
   question for the owner of the published surface, and this story only records that it exists.
9. **`promoted: false` in the distillation's frontmatter stays false.** It becomes true only when the
   atoms exist, which is the slice-mate's PR or nobody's; flipping it alongside the staging would be
   a claim about `.kb/product/` made from a story that is forbidden to write there (*Context pack* 9,
   AC-008).
