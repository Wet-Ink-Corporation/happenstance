---
item: HS-S0162
stage: spec
created: 2026-08-17T13:16:17.924Z
updated: 2026-08-17T13:16:17.924Z
template_sig: 87bbf1d0
rendered_sig: 6b76ab99
---

# Spec — Resolve DT-9 and fix the protocol before recruitment

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-05, BR-06, BR-14; DoD-5, DoD-6; DT-9 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `## Design tension ownership`, line 158: DT-9 is HS-P0024's |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — AC-001, AC-002; derived requirements 1, 4, 5, 6, 11 |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/dt9-and-fixed-protocol/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (the three users, the eight states, the primitive layer, the accessibility floor, IQ-1…IQ-7, UX-AC-001/002/003) and `## Testing brief` (the AC/tier table; AC-002 is a `git` provenance fact, not a reviewer opinion) |
| Signed-off design (BINDING) | [`../_design.md`](../_design.md) — `hasSurface: false`, `# no items`, and a human sign-off dated 2026-08-17 that names DT-9 as the one decision this file durably records |
| Story map | [`../_storymap.md`](../_storymap.md) — slice `session-protocol`, row `dt9-and-fixed-protocol`; `### Why the slices fall here`, first paragraph |
| Grounding | [`../_grounding.md`](../_grounding.md) — confirmed mechanics behind every citation the briefs make |

## One-line PR slice

Resolve DT-9 in `_design.md` — which persona the session walks, in terms of what that reader is
trying to accomplish, with the two rejected personas and why each lost — and fix the scenario, the
narration mode, the severity scale and the logger-disqualifying criteria in the same commit, before
any candidate is approached.

## Executive summary

`_design.md` today says, in its `## Sign-off` section, that *"the only decision this file itself
durably records — DT-9, which persona the comprehension session walks — is sign-off by the human
reading this file, since no capture/perceptual gate exists to record it elsewhere"*
([`../_design.md`](../_design.md), lines 103–116). The file makes that promise and does not yet
keep it: there is no persona named anywhere in it, no scenario, no narration mode, no severity
scale and no disqualifying criteria. Every `##` heading from `## Signatures` to `## The doctest`
reads `N/A — no user-facing surface`, which is correct about the *API* question the bundled template
asks and silent about the one question this project's design stage actually had to answer.

This PR lands that content, and nothing else. It appends the protocol to `_design.md` — the DT-9
resolution composed into the template's existing `## Shape decision` table shape
(`.redkiln/templates/_design.md:55,64`), then the scenario, the narration mode, the severity scale,
the disqualifying criteria, the log's path and heading vocabulary, and the optional
cognitive-walkthrough pre-screen — in one commit, dated before any candidate is approached, so that
AC-002's provenance check is a `git log --format=%aI` fact rather than an assurance.

The delta is therefore additive and bounded: no `pub` item, no crate under `crates/`, no `.kb/`
atom, no reader approached, no log content. `hasSurface: false` and the `# no items` block stay
exactly as signed off; what changes is that the file now asserts a persona choice it previously only
promised, and that assertion is re-signed as an amendment *beneath* the existing sign-off rather
than by rewriting it.

## Context pack

The load-bearing decisions this story must honor. Read this section and you can start; everything
deeper is a signposted anchor below.

### The mount point is a file that is already signed off, and the amendment discipline is binding

`_design.md` carries a human sign-off — *"approved by Ryan Britton (repository owner), 2026-08-17,
with no conditions… Recorded via `redkiln advance HS-P0024 --verdict approved --stay --apply`"*
([`../_design.md`](../_design.md), lines 111–116). This story adds content to that file. The rule
from [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
applies directly, and its test is the one to run on every edit: **ask whether the edit changes what
the document asserts, not whether it changes the document** (that atom, `## The test`).

Concretely, three consequences:

- **The existing sign-off paragraph is never edited.** It made a true statement about a review that
  happened on a file with no persona in it. Rewriting it to read as if it had approved the persona
  choice would convert a true claim into a false one — exactly the ADR-0002 failure that atom
  records (`## Three worked instances, all from one week`, second instance).
- **The new protocol content is appended as new sections, and gets its own sign-off line beneath
  the existing one.** The addition *does* change what the document asserts, so it is an amendment,
  not a rename-in-place, and it goes to the human at `.redkiln/processes/project.yaml`'s `design`
  gate on its own terms.
- **`hasSurface: false` and the `# no items` fenced block stay.** This story adds no public API and
  no rendered screen. Flipping either would contradict a signed-off design and would be false: the
  protocol is prose a person reads, which is what `_design.md`'s `## Items` section already
  established at lines 12–61.

### DT-9 is genuinely open, and this story is where it closes — but the *shape* of the answer is fixed

Every upstream artifact deliberately refused to pre-empt it. `../_decomposition.md`'s UX brief
Notes say so in as many words — *"DT-9 is deliberately unresolved here… the choice belongs to
`_design.md` and its human sign-off gate"* — and *"Contract grain, deliberately. Nothing above
prescribes a file layout for the log, a heading vocabulary, or which severity scale wins."* So the
implementer is not looking for a decision someone already made; they are making it. What is already
fixed is the shape it must take:

- **Stated as intent, not as a label.** The resolution names what that reader is trying to
  *accomplish* — model a cross-entity consistency boundary correctly on the first real attempt /
  understand why the storage contract is shaped as it is / decide inside twenty minutes whether to
  depend on this — not "Persona 2" (UX-AC-001).
- **With the comprehension technique that fits that intent**, drawn from the published three-way
  taxonomy and justified against the persona chosen, not asserted: **paraphrase** for a mental-model
  goal, **plus-minus** for a confidence/trust goal, **task-based/scenario** for a find-and-use goal
  (research 04, `## Findings`, third bullet, and `## Implications for the idea`, second bullet).
- **With both rejected personas named and why each lost**, composed into the `## Shape decision`
  table primitive that already exists at `.redkiln/templates/_design.md:64`
  (`| Item | Chosen shape | Rejected (and why) | Evidence | Resolves |`). This is the repository's
  existing way of recording a decision *with* its alternatives; do not invent a new one.
- **With an explicit statement that the other two personas' evidence remains inferred.** The
  qualification in [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md),
  `## Risks`, first bullet — *"none of these three personas has been directly observed"* — is
  narrowed by this initiative to exactly one persona, and the other two keep it. HS-P0025 lifts this
  statement verbatim at closeout; getting it wrong here mis-promotes an audience.

Two facts should weigh on the choice and are easy to miss. **Persona 2's recruitment is the hardest
of the three** — its readers today are *"exclusively the project's own team"*
(`personas-and-journeys.md`, `## Persona 2 — The adapter author`), and the disqualifying criteria
this same story writes are what would rule most of them out; a persona nobody eligible can be
recruited for is a protocol that produces no evidence. And **Persona 2's evidence base is the
thinnest of the three** on the "no third-party adapter" claim (`## Risks`, third bullet). Neither
fact decides DT-9. Both belong in the `Rejected (and why)` cell if Persona 2 loses, and in the
`Evidence` cell if it wins anyway.

### Everything in this story is fixed *before* recruitment, and that ordering is the evidence

AC-002 is a provenance claim, not a quality claim: the protocol *could not have been written to fit
what the reader did*. The testing brief makes the mechanism explicit — `git log --format=%aI --
<path to _design.md>` must predate the session date recorded in the log
(`../_decomposition.md`, `## Testing brief`, AC-002 row). Two things follow for the implementer:

- **One commit, all five elements.** Persona, scenario, narration mode, severity scale,
  disqualifying criteria. Splitting them across commits means the latest of them is the date a
  reviewer must use, and a scale added after the session is precisely the retrofit research 04 says
  separates a rankable record from a diary.
- **No candidate is approached in this PR.** Recruitment is HS-S0164 `non-insider-recruitment`,
  one story later in the same slice. This story writes the bar; it does not apply it.

### The slice consumes this file, so the vocabulary decided here is an interface

`session-protocol` is one slice and not three because *"the severity scale, the disqualifying
criteria and the log's shape are the same decision seen from three angles: a scale named in
`_design.md` but absent from the log's severity column is a scale that gets retrofitted"*
([`../_storymap.md`](../_storymap.md), `### Why the slices fall here`). Concretely, the two
slice-mates read this file as their input:

- **HS-S0163 `friction-log-skeleton`** builds the scaffold whose severity column carries the scale
  named here, whose section headings are the heading vocabulary named here, and which lives at the
  path named here. The briefs deliberately left path and heading vocabulary to `_design.md`; if this
  story does not fix them, the skeleton invents them and the slice has no contract.
- **HS-S0164 `non-insider-recruitment`** checks a real candidate's declaration against the
  disqualifying criteria written here, and AC-003 requires a reviewer to verify that declaration
  *without asking the reader anything further* — so the criteria must be checkable statements about
  artefacts, not a judgement call.

### The four protocol elements, and the bar each must clear

- **The scenario** — one to two sentences, in the reader's own terms, naming their goal and not this
  documentation's structure; readable by someone who does not know this repository exists.
  Research 04 calls the scenario the most crucial part and names both failure directions: too narrow
  proves nothing, too esoteric is dismissed as an edge case (UX-AC-002; `project.md` derived
  requirement 4).
- **The narration mode** — concurrent or retrospective, declared as a *methodological* choice with
  its reason. The meta-analytic finding is that the two surface measurably different findings, and
  that concurrent narration inflates task time by roughly 17–20%; the protocol records that
  inflation and states that elapsed time is therefore not a comparable metric across sessions,
  rather than quietly reporting it as one (research 04, `## Findings`, sixth bullet; `project.md`
  derived requirement 5).
- **The severity scale** — a *named published* scale, marked inline as the log is written, never
  retrofitted. Nielsen's 0–4 (not a problem / cosmetic / minor / major / catastrophe) is the named
  candidate the research supplies; the choice is this file's. Binding regardless of which scale
  wins: **every mark is a text token** — the word, or the number on the named scale — and never an
  emoji or a colour swatch carrying the meaning alone, because a markdown log on a diff view has no
  colour at all and a screen reader and a `git diff` must both read the severity
  (`../_decomposition.md`, `#### The accessibility floor`, first bullet; UX-AC-003).
- **The disqualifying criteria** — written in checkable terms that name the specific artefacts
  conferring insider knowledge *in this repository*: having authored the material under test, having
  read `crates/happenstance-core/`'s source, `references/adr/` or `references/evaluation/`
  (`project.md`, `## In scope`, third bullet). Research 04 treats non-authorship as the *mechanism*,
  not a nicety — insiders unconsciously route around the rough spots the exercise exists to find.
  The criteria must also make room for the **ineligible-but-used-anyway** state: it is a real state
  the surface has to express, and if it occurs the artefact is *unmet* and the log says so — it is
  never a reason to redefine the bar (`../_decomposition.md`, `#### The states this surface has to
  express`, second bullet; `project.md` risk table, row 1).

### The pre-screen is declared here and is never the proof

The optional cognitive walkthrough — the authors themselves walking Wharton et al.'s four questions
against a page before spending a real reader's time — is declared in this file, explicitly as a
*hypothesis generator*. Its own literature is clear that a "no" answer is a plausible defect, not a
proven one (research 04, `## Findings`, second bullet). Anything it alone justifies stays
**provisional** until the real session confirms or supersedes it
(`../_decomposition.md`, `## Testing brief`, `**Fixtures and seams**`, second bullet). It traces to
no AC and is not a story — that is why it is a clause here (`../_storymap.md`, `## Coverage`,
`**Not represented as stories, on purpose**`).

### What this story must not do

Compose from the repository's existing document primitives; invent no new format. The one hard
token-like constraint is real: **any checklist keeps each box on one line**, because the gate parser
matches line by line and a wrapped box can never match (`CLAUDE.md`; `.redkiln/templates/gates/`).
And nothing here settles a sibling project's tension — DT-1, DT-4, DT-5, DT-6 belong to HS-P0022 and
DT-10 to HS-P0023, and a protocol that pre-judges what the reader will find in their material is not
an instrument.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` (story frontmatter, `archetype: capability`) |
| **Slice / milestone** | `session-protocol`. Slice-mates, implemented in the same context and mounted as one integrated surface: `friction-log-skeleton` (HS-S0163), `non-insider-recruitment` (HS-S0164). Both are `blocks:` edges on this story's frontmatter. |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/_design.md` — the project's real composition root for this slice. `design.capture` is deliberately absent from `.redkiln/config.yaml` (the block at lines 15–81), so the perceptual review is a skip and this written file is the only record DT-9 will ever have (`project.md`, derived requirement 11). The protocol is *mounted* when the slice's two consumers read their inputs out of it, not when it is merely written. |
| **Wires into** | `.redkiln/templates/_design.md:55,64` (the `## Shape decision` table primitive: `\| Item \| Chosen shape \| Rejected (and why) \| Evidence \| Resolves \|`) and `:130` (the `## Sign-off` primitive); `.redkiln/templates/gates/` (the one-line-checkbox primitive); `.redkiln/templates/briefs/brief.md` (the Intent / Acceptance Criteria / Notes spine); `docs/README.md`'s two-column routing-table shape; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (the amendment discipline for an already-signed file); `.redkiln/processes/project.yaml`, `design` stage (the human review gate that approves the amendment). |
| **Renders surfaces** | **None** in `_design.md`'s `## Items` sense — the signed-off design declares `hasSurface: false` and `# no items — no public API surface, no rendered UI surface`, and this story does not change that. The artifact surface it *changes* is `_design.md` itself, which `../_decomposition.md`'s UX brief names as this project's third user-facing artifact (`## UX brief`, `### Intent`, item 3). |
| **Advances DoD scenario** | Initiative **DoD-5** — *"A reader who is not the author and not an insider completes a stated scenario, and it is recorded"* (`../../initiative.md`, `## Definition of Done`, item 5). This story lands the two halves DoD-5 requires to exist *in advance*: the **stated scenario**, and the criteria against which non-authorship and non-insider status become declarable. It also discharges project **DoD-3** — DT-9 resolved in `_design.md` with the human design gate approved. |
| **Conformance rules / clauses** | None. This story is not adapter-observable and amends no `SPECIFICATION.md` clause: it touches no crate, adds no `pub` item and changes no port. Stated rather than omitted, per `CLAUDE.md`'s rule that a story changing a port and naming no rule is a port change nothing can fail — this is the other case, and it is genuinely the other case. |

## PR boundary

**In this PR**

- The DT-9 resolution appended to `_design.md`, composed into the `## Shape decision` table shape,
  with the chosen persona stated as intent, the fitting comprehension technique justified, both
  rejected personas with reasons, and the explicit statement that the other two personas' evidence
  remains inferred.
- The four fixed protocol elements in the same file and the same commit: scenario; narration mode
  with its reason and the task-time caveat; the named severity scale with its text-token rule; the
  logger-disqualifying criteria naming this repository's insider artefacts, including the
  ineligible-but-used-anyway state.
- The log's **path and heading vocabulary**, fixed here because the briefs deliberately left them to
  this file and the slice-mate skeleton consumes them.
- The optional cognitive-walkthrough pre-screen, declared as a hypothesis generator whose findings
  stay provisional.
- The amendment sign-off line beneath the existing 2026-08-17 sign-off, leaving that sign-off
  legible and unedited.
- This story's own backlog folder: `spec.md`, `_ledger.md`, and the stage artifacts the process
  renders.

**Explicitly not in this PR**

- **No candidate is approached, contacted or screened.** That is HS-S0164, and doing it here
  destroys AC-002's provenance.
- **No friction-log file is created or filled.** The scaffold is HS-S0163; its content is
  `session-run-against-pinned-tree`.
- **No `.kb/` atom is authored, promoted or reconciled.** Hand-authoring outside the ingest path is a
  named non-goal and the first attempt was reverted (`0269720`); persona promotion is HS-P0025's
  (`project.md`, `## Out of scope`).
- **No file under `crates/`, `docs/`, `examples/`, `spec/` or `standards/` is touched.** Content
  fixes are `content-fixes-from-dispositions`, two slices later, and they require a disposition that
  does not exist yet.
- **`hasSurface: false`, the `# no items` block, and every `N/A — no user-facing surface` heading are
  left as signed off.** No `pub` item, no doctest, no signature.
- **No sibling project's design tension is decided.** DT-1/4/5/6 are HS-P0022's, DT-10 is HS-P0023's.
- **No item frontmatter is edited and no `redkiln advance`/`new` is run.** The CLI is the single
  writer of system fields (`CLAUDE.md`, `## Where the work lives`).

**Merge DoD (one line).** `_design.md` carries the DT-9 resolution and all four protocol elements in
a single commit whose author date precedes any recruitment contact, the amendment is signed off at
the `design` gate with the prior sign-off intact, and `redkiln validate --kb && redkiln doctor` are
clean with no new `template-drift` beyond the six standing advisories.

The implementer may also touch the composition-root and wiring files named in the Integration
contract in order to mount this slice; that is not scope drift.

```
.bklg/docs-that-teach/comprehension-evidence/_design.md
.bklg/docs-that-teach/comprehension-evidence/dt9-and-fixed-protocol/**
```

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **DT-9 resolves to one persona, stated as intent** | A new `## DT-9 — which persona the comprehension session walks` section in `_design.md` names the chosen persona by *what that reader is trying to accomplish* (model a cross-entity consistency boundary on the first real attempt / understand why the storage contract is shaped as it is / decide inside twenty minutes whether to depend on this), never by bare label. | [`../_decomposition.md`](../_decomposition.md) `## UX brief` → UX-AC-001; [`../../_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) `## Persona 1/2/3`, each persona's `**Goal.**` paragraph |
| **The rejected two personas are named with reasons, in the existing table primitive** | The resolution is composed into `\| Item \| Chosen shape \| Rejected (and why) \| Evidence \| Resolves \|`. `Resolves` carries `DT-9`. The `Rejected` cell carries a reason per persona, not a shared sentence. Recruitability and evidence thinness are legitimate reasons and must appear if they were weighed. | [`.redkiln/templates/_design.md`](../../../../.redkiln/templates/_design.md) lines 55, 64; `personas-and-journeys.md` `## Risks`, bullets 1 and 3 |
| **The comprehension technique is chosen from the published taxonomy and justified against the persona** | Exactly one of paraphrase (mental-model goal) / plus-minus (confidence-and-trust goal) / task-based (find-and-use goal), with the fit argued rather than asserted. A technique that does not match the chosen persona's goal verb is a defect. | `../../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` `## Findings` bullet 3; `## Implications for the idea` bullet 2 |
| **The other two personas keep the inferred-evidence qualification, in writing** | One sentence in the same section stating that the other two personas' evidence remains inferred, in the form HS-P0025 can lift. Absence of this sentence lets closeout promote three observed personas from one session. | `project.md` AC-001; `../_decomposition.md` `### Why the load-bearing edges exist` (quoted in `project.md`, `## How this advances the initiative`) |
| **The scenario is one to two sentences in the reader's own terms** | States the reader's goal, not the documentation's structure; comprehensible to someone who has never seen this repository. Neither so narrow it proves nothing nor so esoteric it is dismissed as an edge case. | `project.md` derived requirement 4; `../_decomposition.md` UX-AC-002; research 04 `## Findings` bullet 1 (scenario as "the most crucial part") |
| **The narration mode is declared with its reason and its cost** | Concurrent or retrospective, stated as a methodological choice; the ~17–20% task-time inflation of concurrent narration is recorded, together with the statement that elapsed time is therefore not a comparable metric across sessions. | `project.md` derived requirement 5; research 04 `## Findings` bullet 6, `## Evidence & citations` row on Hertzum TOCHI 2024 |
| **The severity scale is named, published, and text-token-only** | The scale is named before the session (Nielsen 0–4 is the supplied candidate) and its marks are words or numbers on that scale. No mark may carry meaning by colour or emoji alone; the log must read correctly as plain text and under a screen reader. Applied inline as the log is written, never retrofitted. | `project.md` derived requirement 6; `../_decomposition.md` `#### The accessibility floor` bullet 1 and UX-AC-003; research 04 `## Findings` bullet 4 |
| **The disqualifying criteria name this repository's insider artefacts** | Checkable statements, not judgement calls: authored the material under test; read `crates/happenstance-core/`'s source; read `references/adr/`; read `references/evaluation/`. A reviewer must be able to check a declaration against them without asking the reader anything further. | `project.md` `## In scope` bullet 3 and AC-003; research 04 `## Findings` (non-authorship as mechanism) |
| **The ineligible-but-used-anyway state is representable and is a failure** | The criteria section states that if an ineligible reader is used, the artefact is unmet and the log records that — the bar is never redefined to fit the recruit. | `../_decomposition.md` `#### The states this surface has to express` bullet 2; `project.md` `## Risks and coupling notes` row 1 |
| **The log's path and heading vocabulary are fixed here** | `_design.md` names the file path the friction log will occupy and the heading vocabulary for its sections (scenario; logger identity, context and date; chronological record; disposition; scope sentence; hand-off), so HS-S0163 fills a shape it did not invent. Stable heading anchors are required — IQ-3 makes a sibling's citation into a stumble id resolve after finalisation. | `../_decomposition.md` UX brief `### Notes`, *Contract grain, deliberately*; IQ-3 and UX-AC-007; `../_storymap.md` `### Why the slices fall here`, second paragraph |
| **The cognitive-walkthrough pre-screen is declared and demoted in the same clause** | Declared as available to the authors, explicitly a hypothesis generator; anything it alone justifies is marked provisional until the real session confirms or supersedes it. Never the proof artefact. | `project.md` `## In scope`, pre-screen bullet; research 04 `## Findings` bullet 2; `../_decomposition.md` `## Testing brief`, `**Fixtures and seams**` bullet 2 |
| **All of the above land in one commit, before recruitment** | A single commit containing every element, whose author date (`git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md`) precedes the session date the log will later record and precedes any candidate contact. | `project.md` AC-002; `../_decomposition.md` `## Testing brief`, AC-002 row |
| **The prior sign-off survives, and the amendment is signed on its own terms** | The 2026-08-17 sign-off paragraph is left byte-identical; a new sign-off line is appended below it covering the protocol addition. `hasSurface: false` and the `# no items` block are unchanged. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md` `## The test`; [`../_design.md`](../_design.md) lines 63–65, 103–116; `.redkiln/processes/project.yaml`, `design` stage |
| **Composition is from existing primitives only** | The `## Shape decision` table, the `## Sign-off` section, the Intent/AC/Notes spine, the two-column routing table, the one-line checkbox. No new format, no new heading convention invented for this project. Each checklist box stays on one line — the gate parser matches line by line. | `../_decomposition.md` `#### The primitive layer to compose from — do not hand-roll`; `CLAUDE.md`, six-customised-templates paragraph |

## Data and migrations

**N/A — no data and no migration.** This story adds no schema, no persisted state, no store and no
file format that anything reads programmatically. Its entire deliverable is prose appended to one
markdown artifact under `.bklg/`, and the project ships no crate at all
([`../_design.md`](../_design.md), `## Items`: *"It adds no `pub` item, compiles nothing new, and
touches no crate under `crates/`"*).

Two adjacent obligations that are *not* migrations but are the nearest thing, recorded here so they
are not mistaken for absent:

- **The item frontmatter of `_design.md` is not a data structure this story writes.** The `redkiln`
  CLI is the single writer of item system fields; only the markdown body beneath the closing `---`
  is authored here (`CLAUDE.md`, `## Where the work lives`).
- **The heading vocabulary this story fixes is a forward-compatibility surface, not a schema.** Once
  HS-S0163's skeleton and any sibling project cite a heading anchor or a stumble id, renaming it
  breaks those citations — IQ-3's stability rule is the migration constraint, and the remedy is to
  choose the vocabulary carefully now rather than to write a migration later
  (`../_decomposition.md`, IQ-3; UX-AC-007).

## Acceptance criteria

Ten criteria. Each is framed from what a real person is trying to accomplish — U1 the recruited
reader, U2 the facilitator/logger, U3 the downstream actor
([`../_decomposition.md`](../_decomposition.md), `## UX brief`, *Who the three users are*) — and
crosses the whole of this story's stack: the decision, the file it lands in, and the slice-mate that
consumes it. Together they discharge project **AC-001** (via AC-001…AC-003) and project **AC-002**
(via AC-004…AC-009), with AC-010 carrying the amendment and composition invariants that make the
file itself trustworthy.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U2 the facilitator must choose which reader the session serves, and the three candidates want measurably different things — model a cross-entity consistency boundary correctly on the first real attempt, understand *why* the storage contract is shaped as it is, or decide inside twenty minutes whether to depend on this — **WHEN** a reviewer opens `_design.md` at its new DT-9 section, **THEN** it names exactly one chosen reader by that reader's own goal and never by bare label, and names exactly one comprehension technique from the published paraphrase / plus-minus / task-based taxonomy with the fit argued against that goal rather than asserted, so a technique mismatched to the goal verb is visibly a defect. | Static presence: `rg -n "^## DT-9" .bklg/docs-that-teach/comprehension-evidence/_design.md` and `rg -n "paraphrase|plus-minus|task-based" <same file>`. Artifact-evidence: `_ledger.md` AC-001 cites the `file:line` of the goal sentence and of the technique justification. Reviewer-read at the `design` gate (`.redkiln/processes/project.yaml`, `design` stage). |
| AC-002 | **GIVEN** U3 reading this file six months later must be able to tell a decision from a default, **WHEN** they read the DT-9 row, **THEN** it is composed into the repository's existing `## Shape decision` table primitive with all five columns filled (`Item` / `Chosen shape` / `Rejected (and why)` / `Evidence` / `Resolves`, with `Resolves` carrying `DT-9`), and the `Rejected` cell carries a distinct reason per rejected persona — including, where they were weighed, Persona 2's recruitability and the thinness of its evidence base — never one shared sentence covering both. | Static shape: `rg -n "Chosen shape" .bklg/docs-that-teach/comprehension-evidence/_design.md` matches the five-column header at `.redkiln/templates/_design.md:64`; `rg -n "DT-9" <same file>` finds it in the `Resolves` column. Artifact-evidence: ledger cites the row's `file:line`; the `design`-gate reviewer confirms two distinct reasons, not one shared sentence. |
| AC-003 | **GIVEN** HS-P0025 must at closeout replace the blanket *none of these three personas has been directly observed* qualification with an accurate one rather than promoting three personas from one session, **WHEN** it lifts a sentence out of this section, **THEN** one sentence in that same section states which single persona this initiative will have directly observed and that the other two personas' evidence remains inferred, in a form liftable verbatim without re-derivation. | Static presence: `rg -n -i "remains inferred|stays inferred" .bklg/docs-that-teach/comprehension-evidence/_design.md`. Artifact-evidence: ledger cites the sentence's `file:line`; reviewer checks it names the other two personas explicitly rather than saying "the others". |
| AC-004 | **GIVEN** U1 the recruited reader must be able to start work from the scenario alone, having never seen this repository, **WHEN** the facilitator reads it to them at the session, **THEN** `_design.md` carries it as one to two sentences in the reader's own terms, naming that reader's goal and not this documentation's structure — neither so narrow that completing it proves nothing nor so esoteric that it is dismissed as an edge case. | Static presence: `rg -n -i "scenario" .bklg/docs-that-teach/comprehension-evidence/_design.md` locates the clause; the one-to-two-sentence budget is checked by reading. Artifact-evidence: ledger cites `file:line`. Reviewer-read against `../_decomposition.md` UX-AC-002 — the reviewer must be able to read it cold, without knowing this repository exists. |
| AC-005 | **GIVEN** U2 must not later report an inflated elapsed time as though it were comparable across sessions, **WHEN** a reviewer reads the narration clause, **THEN** it declares concurrent or retrospective as a stated methodological choice with its reason, records that concurrent narration inflates task time by roughly 17–20%, and states that elapsed time is therefore not a comparable metric across sessions. | Static presence: `rg -n -i "concurrent|retrospective|17" .bklg/docs-that-teach/comprehension-evidence/_design.md` finds the mode and the inflation figure. Artifact-evidence: ledger cites the clause's `file:line`; reviewer confirms all three parts are present — the mode, its reason, and the non-comparability statement — not only the mode. |
| AC-006 | **GIVEN** U3 opens the eventual log on a `git diff` view that has no colour at all, or through a screen reader, **WHEN** they reach a severity mark, **THEN** the scale named in `_design.md` is a named, published scale (Nielsen's 0–4 is the supplied candidate; the choice is this file's), every mark on it is a text token — the word, or the number on that scale — with no emoji or colour swatch carrying the meaning alone, and the clause states the mark is applied inline as the log is written and never retrofitted. | Static presence: `rg -n -i "severity" .bklg/docs-that-teach/comprehension-evidence/_design.md` finds the named scale and its levels. Static accessibility: the severity clause contains no emoji and no colour swatch, checked by reading the file as plain text with styling stripped (`../_decomposition.md`, `#### The accessibility floor`, bullet 1; UX-AC-003). Artifact-evidence: ledger cites the scale's `file:line`. |
| AC-007 | **GIVEN** a reviewer in HS-S0164 must check a real candidate's declaration without asking the reader anything further, **WHEN** they hold that declaration against this file, **THEN** `_design.md` lists the disqualifying criteria as checkable statements about artefacts — having authored the material under test; having read `crates/happenstance-core/`'s source, `references/adr/`, or `references/evaluation/` — never a judgement call, and states that if an ineligible reader is used anyway the artefact is unmet and the log records that, which is never a reason to redefine the bar. | Static presence: `rg -n "happenstance-core|references/adr|references/evaluation" .bklg/docs-that-teach/comprehension-evidence/_design.md`. Static existence: `test -d crates/happenstance-core && test -d references/adr && test -d references/evaluation` confirms every named artefact is real and citable. Artifact-evidence: ledger cites the criteria block and the ineligible-but-used-anyway sentence by `file:line`. |
| AC-008 | **GIVEN** HS-S0163 must build a log scaffold it did not invent, and sibling projects will later cite heading anchors and stumble ids into that log, **WHEN** its implementer opens `_design.md`, **THEN** the file names the log's repo-relative file path and fixes its heading vocabulary (scenario; logger identity, context and date; chronological record; disposition; scope sentence; hand-off) as stable anchors, so a citation made into a heading or a stumble id still resolves after the log is finalised, and the log so described is navigable by browser find alone — no widget, script or rendering tool required to read it. | Static presence: `rg -n "friction" .bklg/docs-that-teach/comprehension-evidence/_design.md` yields a concrete repo-relative path, and the six heading names appear as a list. Artifact-evidence: ledger cites the path line and the vocabulary block. Consumption check: HS-S0163's own ledger must be able to cite these exact lines as its input (`../_storymap.md`, `### Why the slices fall here`). |
| AC-009 | **GIVEN** project AC-002 is a provenance claim — the protocol could not have been written to fit what the reader did — and not a quality claim, **WHEN** a reviewer runs `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md`, **THEN** a single commit carries all five elements (persona, scenario, narration mode, severity scale, disqualifying criteria) and its author date precedes both the session date the log will record and any contact with any candidate; no candidate is approached, contacted or screened in this PR. | Static provenance: `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` yields one date for this change, and `git log --oneline -- <same path>` shows the five elements arrived together rather than across commits. Static negative: `git diff --name-only main..HEAD` contains no recruitment artefact and no file outside the two paths in the PR boundary. Ledger cites the commit sha and its author date. |
| AC-010 | **GIVEN** the human who approved `_design.md` on 2026-08-17 approved a file with no persona in it, and rewriting that approval to read as though it had covered the persona would convert a true claim into a false one, **WHEN** they review this amendment at the `design` gate, **THEN** the existing sign-off paragraph is unchanged, the protocol is appended as new sections beneath the template's existing heading spine with its own sign-off line below the original, `hasSurface: false` and the `# no items` fenced block are untouched, the addition is composed only from the repository's existing document primitives — the `## Shape decision` table, the `## Sign-off` section, the Intent/AC/Notes spine, the two-column routing table, the one-line checkbox — with no fold, no widget and no new format invented, and every checklist box added stays on one line. | Static diff: `git diff -- .bklg/docs-that-teach/comprehension-evidence/_design.md` shows no removed line inside the sign-off paragraph (lines 103–116) or the `# no items` block (lines 63–65). Static composition: `rg -n "details>|summary>" <file>` returns nothing; `rg -n "\- \[ \]" <file>` shows every box on one line. Human gate: `.redkiln/processes/project.yaml`, `design` stage, `gate: { kind: review, approver: human, verdicts: [approved, changes-requested] }`, verdict `approved`. |

**Traceability.** Project **AC-001** (*the persona choice is decided, not defaulted*) is discharged
by AC-001 + AC-002 + AC-003. Project **AC-002** (*the protocol is fixed before recruitment*) is
discharged by AC-004 + AC-005 + AC-006 + AC-007 + AC-009, with AC-008 carrying the slice-interface
half the briefs deliberately left to this file (`../_decomposition.md`, `## UX brief`, `### Notes`,
*Contract grain, deliberately*). AC-010 is the invariant that keeps all nine legible and
re-reviewable rather than a rewrite of an already-signed document.

## Interaction quality

RFC §6.7/D6. This project renders **no screen and no public API** — the signed-off design records
`hasSurface: false` and `# no items — no public API surface, no rendered UI surface`
([`../_design.md`](../_design.md), lines 63–65) — but it does render an **artifact surface a person
reads**, and `../_decomposition.md`'s UX brief names `_design.md` as this project's third
user-facing artifact (`## UX brief`, `### Intent`, item 3). The invariants below therefore bind in
the medium this repository actually has: markdown, `git diff`, browser find and stable heading
anchors. There is **no CSS layer and no token file, and inventing one is out of scope**
(`../_design.md`, `## Items`), so *composition* here means **which existing document primitive the
content is composed into** — and since an unstyled render is the only render there is, the
composition rows below are the ones that make a structurally-correct-but-hand-rolled file fail.

Every invariant that applies is carried by an `AC-###` **row in the acceptance-criteria table
above**. This section maps which row carries which, and how each is checked; it declares nothing on
its own.

**State invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the decision, its rejected alternatives and its evidence are readable *at the decision*, in one row, with no hop to a footnote or another file | AC-002 | The five-column `## Shape decision` row is itself the check: `Rejected (and why)` and `Evidence` are cells of the same row, so a reviewer never leaves the decision to learn why it was made |
| **Non-occlusion** — no load-bearing content exists only inside a fold, a collapsed admonition or a filtered view; deleting every fold would leave the record complete (IQ-2; named as an anti-pattern in `../../_discovery/distillation/interaction-patterns.md`) | AC-010 | `rg -n "details>|summary>"` over `_design.md` returns nothing, and all five protocol elements are visible-by-default prose |
| **Preserved position — stable anchors and ids** (IQ-3) | AC-008, AC-010 | The log's heading vocabulary is fixed *before* HS-S0163 cites it, and the existing `_design.md` regions are not re-headed or renumbered, so citations already made into the file still resolve |
| **Reversibility** (IQ-4) — the amendment is appended and the prior sign-off stays legible with its reasoning intact rather than being overwritten; a later revision of the protocol is likewise recorded beside the original | AC-010 | `git diff` shows additions only; the governance test from `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (*does the edit change what the document asserts?*) is applied per hunk at the `design` gate |
| **Keyboard reachability** — the artifact is completable and auditable with only what the medium provides: browser find, plain-text reading, `git diff`. No pointer, widget, script or rendering tool is required, and nothing autoplays or depends on motion | AC-006, AC-008 | The severity clause reads correctly as plain text with styling stripped, and the log shape AC-008 fixes is navigable by browser find and stable heading anchors alone (`../_decomposition.md`, `#### The accessibility floor`, bullets 1–3) |

**Composition invariants** — taken from the signed-off [`../_design.md`](../_design.md) and the
primitive layer it points at (`../_decomposition.md`, `#### The primitive layer to compose from — do
not hand-roll`).

| Invariant | Carried by | The real rule or number |
| --- | --- | --- |
| **Presentation exists at all** — the DT-9 resolution is composed into a real primitive, not dumped as a bare paragraph or a bullet list | AC-002 | The `## Shape decision` table at `.redkiln/templates/_design.md:64`, **all five columns filled**. A four-column row, or a row whose `Rejected` cell is empty, fails |
| **Composition and placement** — the protocol sits as new `##` sections inside the template's existing heading spine, and the amendment sign-off goes beneath the original, never in place of it | AC-010 | `.redkiln/templates/_design.md:55` (`## Shape decision`) and `:130` (`## Sign-off`) are the placement anchors; `../_design.md` lines 103–116 are the paragraph that must survive |
| **Transience** — in this medium the axis is *visible-by-default vs folded*, and everything here is persistent chrome: nothing is revealed on demand, because a load-bearing item behind a fold is a named anti-pattern | AC-010 | Zero `<details>`/`<summary>` elements, zero collapsed admonitions, zero tabbed sections |
| **Density budget** | AC-004, AC-006, AC-007, AC-009 | Scenario: **1–2 sentences** (UX-AC-002). Severity scale: **one** named published scale, Nielsen's **0–4** = five levels, as the supplied candidate. Disqualifying criteria: **four** named artefacts (authored material; `crates/happenstance-core/` source; `references/adr/`; `references/evaluation/`) plus the ineligible-but-used-anyway clause. Log heading vocabulary: **six** headings. Commits: **exactly one**, carrying **five** elements. Checklist boxes: **one line each** |
| **Hierarchy** — DT-9 first because it determines everything after it, then scenario, narration mode, severity scale, disqualifying criteria, the log's path and vocabulary, the pre-screen clause, and the amendment sign-off last | AC-010 | The order a reader needs, matching the slice's own ordering (`../_storymap.md`, `### Why the slices fall here`) |
| **Named anti-patterns** — inventing a CSS layer or token file; a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold; colour or emoji as the sole carrier of severity; a wrapped checklist box; a quiz or inline recall check as a lighter substitute for the session | AC-006, AC-010 | `../_design.md` `## Items` (no CSS layer); `../../_discovery/distillation/interaction-patterns.md` `## Anti-patterns` (bespoke widget, fold, quizzes); `../_decomposition.md` `#### The accessibility floor` bullet 1 (colour-only); `CLAUDE.md` (the one-line box, and why the parser cannot match a wrapped one) |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The chosen persona turns out to be the hardest to recruit for — Persona 2's readers today are *exclusively the project's own team*, and this story's own disqualifying criteria would rule most of them out | Not grounds to defer DT-9. Decide, and record the recruitability weighing in the `Rejected (and why)` cell if it lost, or the `Evidence` cell if it won anyway (AC-002). A protocol nobody eligible can be recruited for produces no evidence, and that consequence belongs in the record rather than in a postponement |
| EC-002 | A candidate has already been approached when the implementer reaches this story | AC-009's provenance is **unrecoverable**, not repairable. Record it as unmet, never backdate a commit, and escalate. `project.md`'s risk table, row 1: if no eligible reader is found the artefact is unmet — *a failure of the initiative, not a reason to redefine the bar* |
| EC-003 | No published severity scale seems to fit, and a bespoke one is tempting | A bespoke scale fails AC-006's *named, published* requirement. Nielsen's 0–4 is the supplied candidate; choosing a different **published** scale is legitimate and must be cited. Inventing one is precisely the retrofit research 04 separates from a rankable record |
| EC-004 | A disqualifying criterion is written as a judgement (*seems familiar with the codebase*) rather than an artefact fact | HS-S0164 could not then verify a declaration *without asking the reader anything further*, which is project AC-003's own text. Rewrite it as an artefact statement before the commit lands |
| EC-005 | An edit lands inside the existing 2026-08-17 sign-off paragraph — even a reword | Revert that hunk and append instead. The test is *does the edit change what the document asserts, not whether it changes the document* (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`, `## The test`). Rewording an approval so it appears to have covered a persona it never saw converts a true claim into a false one |
| EC-006 | The heading vocabulary needs to change after HS-S0163 or a sibling has cited it | Before those citations exist it is free; after, IQ-3 binds and a rename breaks every citation made into it. Choose carefully now; if a change is genuinely required later, **add** rather than rename, leaving the cited anchor resolving |
| EC-007 | The `design` gate returns `changes-requested` on the amendment | The protocol is **not** fixed and recruitment must not begin. HS-S0163 and HS-S0164 are `blocks:` edges on this story's frontmatter for exactly this reason; the slice does not proceed on an unapproved instrument |
| EC-008 | An edit is attempted on `_design.md`'s YAML frontmatter, or `redkiln advance`/`new` is reached for | The `PreToolUse` hook denies the frontmatter edit, and is right to. Only the markdown body beneath the closing `---` is this story's to write; every state transition belongs to the orchestrating command (`CLAUDE.md`, `## Where the work lives`) |
| EC-009 | A stumble-shaped finding about a sibling project's material surfaces while writing the protocol | Not this story's to absorb or to pre-judge — a protocol that pre-decides what the reader will find is not an instrument. DT-1/4/5/6 are HS-P0022's and DT-10 is HS-P0023's (`project.md`, `## Out of scope`; `../../_decomposition.md`, `## Design tension ownership`) |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| NF-001 | The whole addition reads correctly as **plain text with all styling stripped**, under a screen reader and on a colourless `git diff` | The accessibility floor for a medium with no CSS: the obligation is that the *text* is complete, not that an animation is polite (`../_decomposition.md`, `#### The accessibility floor`, bullets 1–2) |
| NF-002 | `redkiln validate --kb && redkiln doctor` stay clean, with **no new `template-drift` advisory beyond the six standing ones** | `project.md` DoD-8. A seventh advisory is a template changed without a decision, and a missing one is a customisation reverted (`CLAUDE.md`). Never run `redkiln adopt --templates` |
| NF-003 | The scenario is legible to a reader who has never seen this repository or its discovery corpus | AC-004's own test is that a reviewer can read it cold. A scenario that only parses against `personas-and-journeys.md` has been written in the documentation's terms, not the reader's |
| NF-004 | The optional **cognitive-walkthrough pre-screen** is declared here and demoted in the same clause: available to the authors, explicitly a *hypothesis generator*, and anything it alone justifies stays **provisional** until the real session confirms or supersedes it | `project.md`, `## In scope`; research 04's finding that a "no" to one of Wharton et al.'s four questions is a *plausible* defect, not a proven one. Deliberately **not** an AC — `../_storymap.md`, `## Coverage`, *Not represented as stories, on purpose*, states it traces to no acceptance criterion and is never the proof artefact. Its presence in the file is still checked by the merge-gate `rg` pass below |
| NF-005 | **No new file format, heading convention or navigation device** is introduced by this story, in `_design.md` or anywhere else | `../_design.md` `## Items`: *there is no CSS layer and no token file, and inventing one is out of scope*. The equivalent obligation for documents is to compose from primitives that already exist |
| NF-006 | The diff stays **reviewable in one sitting** — one file changed under `.bklg/…/comprehension-evidence/`, plus this story's own folder | AC-009 requires one commit; a commit a human cannot review defeats the only gate this decision will ever get, since `design.capture` is deliberately absent from `.redkiln/config.yaml` |

## Implementation notes (non-prescriptive)

Guidance, not instruction. The implementer owns the shape within the constraints above.

- **Write DT-9's row before anything else.** Every other element depends on it: the technique follows
  the persona's goal verb, the scenario is *that* reader's task, and the disqualifying criteria are
  the artefacts that would spoil *that* reader. Writing the scenario first tends to produce a persona
  reverse-engineered from a scenario someone already liked.
- **Read all three persona sections before choosing.** `personas-and-journeys.md` lines 56, 148 and
  225 each carry a `**Goal.**` paragraph — those are the sentences AC-001 wants restated as intent.
  `## Cross-persona tensions` and `## Risks` are where the two facts that should weigh on the choice
  live.
- **A sanity check on the technique.** Paraphrase fits a *mental-model* goal, plus-minus fits a
  *confidence and trust* goal, task-based/scenario fits a *find-and-use* goal. If the justification
  reads as "we picked X because it is the most rigorous", the fit has not been argued.
- **Compose against the template, not against memory.** Open `.redkiln/templates/_design.md` at lines
  55, 64 and 130 and copy the primitive's exact header row. A table differing by a column is not the
  primitive.
- **Draft it as one commit from the start.** Staging across five elements and amending is fine;
  landing three, then two, is not — the later date is the one AC-009's reviewer must use.
- **State the log path as a repo-relative path, not a description.** HS-S0163 consumes it literally;
  "somewhere under the project folder" is the prose-destination failure IQ-7 names, one artifact
  upstream.
- **Say what the amendment sign-off covers.** The original sign-off approved a file with no persona;
  the new line approves the protocol. Both statements are then true, and the document's history stays
  legible — which is what makes HS-P0025's eventual promotion auditable rather than assertive.

## Tests and CI (merge gate)

Grounded in [`../_decomposition.md`](../_decomposition.md), `## Testing brief` — its AC/tier table
and its `**Content-fix tier**` block. This story ships **no code**, so the doctest tier has nothing
to compile; the tiers that bite are static and provenance checks over the artifact, plus the ledger
discipline `.redkiln/config.yaml:67`'s `require_ledger: true` already imposes.

| tier | command / path | proves |
| --- | --- | --- |
| **Artifact-evidence (ledger)** | [`_ledger.md`](_ledger.md) in this story folder, read by `redkiln verify --grain story` | Every AC-001…AC-010 row is present, `satisfied: true`, and cites a real `file:line` in `_design.md`. `implement → report` is blocked until then (`.redkiln/templates/_ledger.md`, lines 10–17). Most `evidence` values are citations into the proof artifact itself — *not a gap in rigor, the shape rigor takes when the deliverable is a record rather than a function* |
| **Static — provenance** | `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` | **AC-009.** One commit, one author date, preceding any recruitment contact and the session date the log will later record. A real date comparison, not an assurance (`../_decomposition.md`, `## Testing brief`, AC-002 row) |
| **Static — presence** | `rg -n -i "^## DT-9|paraphrase|plus-minus|task-based|remains inferred|concurrent|retrospective|severity|references/adr|references/evaluation" .bklg/docs-that-teach/comprehension-evidence/_design.md` | **AC-001, AC-003, AC-005, AC-006, AC-007** — each element is in the file rather than in someone's intention. Presence only; adequacy stays reviewer-read, per the testing brief's warning against a check nothing can fail |
| **Static — primitive shape** | `rg -n "Chosen shape" .bklg/docs-that-teach/comprehension-evidence/_design.md`, compared against `.redkiln/templates/_design.md:64` | **AC-002.** The resolution is composed into the existing five-column primitive rather than hand-rolled |
| **Static — every named artefact exists** | `test -d crates/happenstance-core && test -d references/adr && test -d references/evaluation` | **AC-007.** A criterion naming a path that does not exist is uncheckable by HS-S0164. The same discipline `_grounding.md` used to *fail* `examples/outside-projection-adapter/` as unreachable, applied here to confirm |
| **Static — composition and accessibility** | `rg -n "details>|summary>"` returns nothing; `rg -n "\- \[ \]"` shows one box per line; the severity clause carries no emoji or colour swatch | **AC-006, AC-010.** The non-occlusion, one-line-checkbox and text-token invariants. `CLAUDE.md` is explicit that the gate parser matches line by line and a wrapped box can never match |
| **Static — amendment diff** | `git diff -- .bklg/docs-that-teach/comprehension-evidence/_design.md`; `git diff --name-only main..HEAD` | **AC-010** and the PR boundary: no removed line inside the existing sign-off paragraph or the `# no items` block, and no file changed outside `_design.md` and this story's folder |
| **Static — pre-screen clause** | `rg -n -i "cognitive walkthrough|provisional" .bklg/docs-that-teach/comprehension-evidence/_design.md` | **NF-004.** Declared and demoted in the same clause. Not an AC by design (`../_storymap.md`, `## Coverage`), but its absence is still caught before merge |
| **Story grain — affected gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The diff touches no workspace package, so this falls through to the five file-reading lints and `spec-trace` unconditionally — a real check rather than a vacuous pass on an empty package set (`../_decomposition.md`, `## Testing brief`, `### Notes`) |
| **Project grain — integration** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`, `integration_scoped`) | DoD-7's non-terminal bar. Nothing in this story can break it, and that is the point: a green gate is a **precondition** for reading this evidence, never a substitute for it |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | **NF-002.** Clean, with no new `template-drift` beyond the six standing advisories |
| **Human gate** | `.redkiln/processes/project.yaml`, `design` stage — `gate: { kind: review, approver: human, verdicts: [approved, changes-requested] }` | **AC-010** and project DoD-3. `design.capture` is deliberately absent, so the perceptual review is a **skip** and this human read is the whole record DT-9 will ever have |

**Not owned here.** `cargo xtask ci` (the terminal `e2e` bar, `.redkiln/config.yaml:60`) is
HS-P0025's. And the friction-log session itself is a research instrument, not a CI step — the
initiative's framing is that the gate and the comprehension record *falsify different things and
neither may stand in for the other*. Do not fold the session into a merge gate, and do not read a
passing `cargo xtask ci --fast` as saying anything about comprehension.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, in this PR |
| --- | --- | --- |
| DT-9 resolves to the persona with the thinnest evidence base and the hardest recruitment (Persona 2), and the slice stalls at HS-S0164 | Medium / High | Both facts are on record and must be weighed in the table's `Rejected` or `Evidence` cell (AC-002). EC-001 forbids deferring the decision; it does not forbid choosing Persona 2 — it forbids choosing it silently |
| The protocol over-specifies the log and leaves HS-S0163 nothing but transcription — or under-specifies it and the skeleton invents a shape | Medium / Medium | AC-008 fixes exactly two things: the **path** and the **heading vocabulary**. Everything structural — append-only ids, the revision block, the disposition slot — is the skeleton's, per `../_storymap.md`'s split of project AC-004 |
| The five elements are split across commits, or the commit is amended after a candidate is contacted | Medium / High | AC-009 is a single-commit criterion checked by `git log`, and EC-002 states plainly that the provenance is unrecoverable rather than repairable |
| The amendment rewrites the signed paragraph so the file reads coherently | Medium / High | AC-010 plus EC-005; the governance test is applied per hunk at the `design` gate. This is the exact failure `.kb/governance/rewrite-the-referent-never-the-reasoning.md` records from ADR-0002 |
| The severity scale named here does not match the column HS-S0163 builds | Low / High | This is why `session-protocol` is one slice and not three: *a scale named in `_design.md` but absent from the log's severity column is a scale that gets retrofitted* (`../_storymap.md`). The two stories are implemented in one context |
| The protocol quietly pre-judges what the reader will find, and stops being an instrument | Low / High | EC-009; and `project.md`, `## Out of scope` — DT-1/4/5/6 are HS-P0022's, DT-10 is HS-P0023's |
| A `.kb/` atom is hand-authored to record the persona choice | Low / High | Named non-goal; the first attempt was reverted (`0269720`). Persona promotion is HS-P0025's, through the ingest path (`project.md`, `## Out of scope`) |

**Coupling.** This story is the head of the project's dependency graph and both of its slice-mates
are `blocks:` edges on its frontmatter (`story.md`, `blocks: [HS-S0163, HS-S0164]`). Nothing
downstream can start against an unapproved instrument, which makes the `design` gate's verdict the
schedule's real hinge — and makes recruitment, which `project.md` says should begin while HS-P0023
is still in flight, dependent on this landing first.

## Dependencies

**Blocks on:** *none.* `depends_on: []`, matching this story's row in
[`../_storymap.md`](../_storymap.md) (`depends_on: —`) and `story.md`'s `blocked_by: []`. It is
first in the `session-protocol` slice and first in the project's merge order.

**Unlocks:**

- **`friction-log-skeleton`** (HS-S0163) — consumes the severity scale, the log path and the heading
  vocabulary fixed here (AC-006, AC-008). Its `depends_on` names this story.
- **`non-insider-recruitment`** (HS-S0164) — applies the disqualifying criteria written here
  (AC-007) to a real candidate, and depends on both protocol stories.

Transitively, every remaining story in the project: `session-run-against-pinned-tree`,
`disposition-every-stumble`, `route-and-escalate`, `content-fixes-from-dispositions`,
`scope-the-claim`, `second-session-decision` and `handoff-note-to-closeout`
(`../_storymap.md`, `### Dependency graph`).

**Project-level.** HS-P0024 itself depends on HS-P0022 and HS-P0023 having merged forward, but that
edge gates the **session** (`observed-session`), not this story. Protocol and recruitment are
explicitly permitted to run while HS-P0023 is in flight (`../_storymap.md`, `## Merge order`,
item 1).

## Anchors (progressive disclosure)

Everything above is enough to start. Open these when the bound AC comes up — link, never paste in
bulk.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| [`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md`](../../_discovery/distillation/personas-and-journeys.md) | The three candidate personas with their `**Goal.**` paragraphs (lines 56, 148, 225) — the sentences AC-001 wants restated as intent — plus `## Cross-persona tensions` and the `## Risks` bullets carrying Persona 2's recruitability and evidence thinness | **Before writing the DT-9 row.** Read all three goals and both risk bullets before choosing; nothing else holds them | AC-001, AC-002, AC-003 |
| [`.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`](../../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md) | The published evidence the protocol is built on: the paraphrase / plus-minus / task-based taxonomy, the scenario as *the most crucial part* with both failure directions, the concurrent-vs-retrospective meta-analysis and its 17–20% task-time inflation, Nielsen's 0–4 scale, non-authorship as *mechanism*, and Wharton et al.'s four questions as a hypothesis generator | **Before writing the technique justification, the scenario, the narration clause and the severity clause.** AC-004…AC-006 each cite a specific finding here; asserting one from memory is how a citation rots | AC-001, AC-004, AC-005, AC-006, AC-007 |
| [`.bklg/docs-that-teach/comprehension-evidence/_decomposition.md`](../_decomposition.md) | The UX brief's three users, eight states, primitive layer, accessibility floor and IQ-1…IQ-7 with UX-AC-001…UX-AC-013; and the testing brief's AC/tier table, `**Fixtures and seams**`, and its warning against automating a check nothing can fail | **Before writing any verification and before relying on an accessibility claim.** UX-AC-001/002/003 are the refined form of AC-001/AC-004/AC-006, and the tier table is where the merge-gate mechanisms come from | AC-004, AC-006, AC-007, AC-008 |
| [`.bklg/docs-that-teach/comprehension-evidence/_design.md`](../_design.md) | **The mount point.** Lines 12–61 establish `hasSurface: false` and the primitive layer; 63–65 are the `# no items` block; 103–116 are the sign-off paragraph that must survive unedited | **Immediately before the first edit, and again before committing.** Every constraint in AC-010 is a statement about specific lines of this file | AC-010, and every AC — it is where they all land |
| [`.redkiln/templates/_design.md`](../../../../.redkiln/templates/_design.md) | The `## Shape decision` primitive at line 55 with its exact five-column header at line 64, and the `## Sign-off` primitive at line 130 | **When composing the DT-9 row and the amendment sign-off.** Copy the header row from here; a table differing by a column is not the primitive | AC-002, AC-010 |
| [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) | The only KB atom binding on this story. `## The test` is the per-hunk question, and `## Three worked instances, all from one week` carries the ADR-0002 case this amendment could otherwise repeat | **Before touching anything at or above `_design.md` line 103**, and on every hunk of the final diff | AC-010 |
| [`.bklg/docs-that-teach/comprehension-evidence/_storymap.md`](../_storymap.md) | `### Why the slices fall here` explains why the scale, the criteria and the log's shape are one commit; `## Coverage` states the pre-screen traces to no AC; `## Merge order` item 1 permits recruitment to begin while HS-P0023 is in flight | **When deciding what belongs in this commit versus HS-S0163's.** The AC-004 split — skeleton owns shape, session owns content — is stated here | AC-006, AC-008, AC-009 |
| [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) | Derived requirements 1, 4, 5, 6 and 11 are the source of AC-007, AC-004, AC-005, AC-006 and AC-001; `## In scope` bullet 3 names the four insider artefacts verbatim; the risk table's row 1 is the ineligible-but-used-anyway rule | **Before writing the disqualifying criteria and the narration clause** — these are the exact words the criteria must name | AC-001, AC-004, AC-005, AC-006, AC-007 |
| [`.bklg/docs-that-teach/comprehension-evidence/_grounding.md`](../_grounding.md) | The confirmed mechanics behind every citation the briefs make, including the finding that no Accepted decision atom governs documentation, personas or comprehension methodology, and a worked example of a citation being *failed* as unreachable | **When a citation looks convenient.** Apply its discipline before adding any new path to `_design.md` | AC-007, AC-008 |
| [`.redkiln/processes/project.yaml`](../../../../.redkiln/processes/project.yaml) | The `design` stage at line 26 — `gate: { kind: review, approver: human, verdicts: [approved, changes-requested] }` — and the comment explaining why the stage exists at all for a project with no surface | **Before requesting the amendment sign-off**, and when EC-007 fires | AC-010 |
| [`.redkiln/config.yaml`](../../../../.redkiln/config.yaml) | `support_initiative: support` (line 5); `affected_gate` (40); `reachability_static` (48); `integration_scoped: cargo xtask ci --fast` (55); `e2e` (60); `require_ledger: true` (67); and the commented-out `design.capture` block that makes the perceptual review a skip | **When wiring the merge-gate commands**, and whenever tempted to treat the perceptual review as coverage | AC-010, and the merge gate |
| [`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`](../../_discovery/distillation/interaction-patterns.md) | The anti-pattern list: a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold, tab or collapsed admonition; quizzes ruled out on evidence as a substitute for a non-author reader's dated record | **Before adding any structural device to `_design.md`**, and if a lighter-weight alternative to the session is ever proposed | AC-006, AC-010 |
| [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) | BR-05, BR-06, BR-14; DoD-5 and DoD-6; DT-9's original statement; and `## Risks` first row — the gate and the comprehension record falsify different things and neither may stand in for the other | **If the scope of the protocol is in doubt**, and when writing the merge-gate rationale | AC-009, and the DoD framing |
| [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) | `## Design tension ownership`, line 158 — DT-9 is HS-P0024's, and DT-1/4/5/6 and DT-10 are not. This is the table an escalation's DT id is checked against | **When a finding looks like it belongs to a sibling** (EC-009), and before naming any DT id | AC-001, and EC-009 |

## Clarifications resolved during spec

1. **Ten AC ids, exactly as the front half enumerated.** AC-001…AC-010, none added, none dropped; the
   ledger matches one-for-one.
2. **The cognitive-walkthrough pre-screen is carried as NF-004, not as an acceptance criterion.**
   `../_storymap.md`, `## Coverage`, states it *"traces to no AC and is not a story… explicitly never
   the proof artefact"*, while the front half lists it as an in-PR deliverable. Both are honoured: it
   is a required clause in the file with an `rg` check in the merge-gate table, and it gates no
   acceptance criterion. Promoting it to an AC would turn a hypothesis generator into evidence, which
   is the precise inversion the research warns against.
3. **The interaction-quality invariants are carried by AC rows, not by prose bullets.** IQ-1 lands on
   AC-002; IQ-2 and IQ-4 on AC-010; IQ-3 on AC-008 and AC-010; keyboard reachability on AC-006 and
   AC-008. `redkiln verify` extracts ACs from the acceptance-criteria table, so an invariant stated
   only in the Interaction quality section would get no ledger row and would never be gated — that
   section therefore maps rather than declares.
4. **"Composition" is read in the medium this repository actually has.** `../_design.md` records
   `hasSurface: false` and states there is no CSS layer and no token file. The composition invariants
   are therefore about *which existing document primitive the content is composed into* — the
   five-column `## Shape decision` table above all — and the density budget's numbers are real ones
   (1–2 sentences; five severity levels; four named artefacts; six log headings; one commit; one line
   per checkbox) rather than pixel budgets borrowed from a medium that is not present.
5. **AC-002 requires a distinct reason per rejected persona, not one shared sentence.** The front
   half's Behavior table already required it; it is restated as an AC because a single sentence
   covering both rejections is the most likely way that row degrades while still looking filled in.
6. **AC-008 fixes two things and no more — the log's path and its heading vocabulary.** Everything
   else about the log's structure is HS-S0163's, per `../_storymap.md`'s split of project AC-004
   (*the skeleton owns the shape; the session owns the content*). This was the one place this story
   could plausibly have absorbed its slice-mate.
7. **No conformance rule, no `SPECIFICATION.md` clause, no `.kb/` atom.** Stated rather than omitted:
   this story touches no crate, adds no `pub` item and changes no port, so there is genuinely nothing
   for `cargo xtask spec-trace` to trace. Persona promotion into `.kb/product/` is HS-P0025's, and
   hand-authoring an atom outside the ingest path is a named non-goal.
8. **The `verifying_test` values in `_ledger.md` are real commands, not test-function ids.** This
   project ships no functions; the testing brief sanctions `file:line` evidence and static/provenance
   mechanisms explicitly, calling that *not a gap in rigor, the shape rigor takes when the deliverable
   is a record rather than a function*.
