---
item: HS-S0177
stage: spec
created: 2026-08-17T13:16:28.581Z
updated: 2026-08-17T13:16:28.581Z
template_sig: 87bbf1d0
rendered_sig: 5af87e82
---

# Spec — Stage the persona and journey documents for the closeout wave

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — BR-13, BR-17, AC-14, DoD-15, the four named journeys under `## Referenced personas & journeys`, and the evaluator open question at `## Open questions for the planning team` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — `**Persona reconciliation (BR-13)**` (run in parallel, reconcile at closeout) and `### Merge order` (`**HS-P0025 last**`, which states the observation correction this story carries) |
| Project | `.bklg/docs-that-teach/durable-audience-closeout/project.md` — DR-3 … DR-7, AC-003, AC-006, AC-007 |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md` |
| Key briefs | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` — `## UX brief` (IQ-1, IQ-7, IQ-8; AC-UX-01, AC-UX-04, AC-UX-05, AC-UX-09), `## Architecture brief` (§2 composition root, §4 the contracts an atom must satisfy, §6 what is deliberately not prescribed), `## Testing brief` (the staged-input-vs-ingested-output seam, and Tier 2 as the only tier that can fail AC-003, AC-006 or AC-007) |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` — approved 2026-08-17, `hasSurface: false`, no public API surface and no rendered surface. Binding on this story by what it *excludes*: there is no signature, visibility or doctest obligation here to invent |
| Grounding | `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` — `## Reconciliation counterpart does not exist in this tree` |
| Roadmap pointer | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md`, `## Merge order` §3 — this story is first in `product-layer-promotion`, ahead of `audience-ingest-wave` and `product-layer-mounting` |

## One-line PR slice

Author the staged persona and journey documents under `.kb/_intake/` — four slots per
persona, the evaluator decision transcribed, all three evidence qualifications adjacent to
the claims they qualify, and the blanket "none has been directly observed" replaced by
naming the persona HS-P0024's session walked.

## Executive summary

This PR lands **staged source material only**: a set of hand-authored documents under
`.kb/_intake/`, one per atom the next story's ingest wave is intended to produce, plus this
story's own `_ledger.md`. Nothing under `.kb/product/`, `.kb/open-questions/`, `.kb/maps/`
or `.kb/decisions/` moves — those are `/redkiln:kb-ingest`'s to write, and this story is the
one that hands it something to read.

The delta over the project charter is a set of shape decisions the briefs deliberately left
open (`_decomposition.md`, `## Architecture brief`, `#6. Deliberately not prescribed`) and
the story map did not fix. This spec fixes three of them:

1. **Granularity** — one staged document per intended atom, not one adjudicated blob. The
   ingest merges or splits either way; one-to-one is chosen so each landed atom's
   `source_paths` gets a single, unambiguous `.kb/_intake/…` entry, and so the reviewer at
   the ingest approval gate reads one document per decision they might disagree with.
2. **Naming** — every file this story writes is `persona-<slug>.md` or `journey-<slug>.md`.
   That is not cosmetic: it makes the next story's narrowed invocation a glob that *cannot*
   match `.kb/_intake/README.md`, which is the live risk the architecture brief flags as T4
   and the project carries as AC-009.
3. **Cardinality** — four journey documents always; persona documents are two or three, and
   which it is falls out of the evaluator decision this story's dependency already made. The
   count is a consequence, never a choice made here a second time.

Everything else this PR does is transcription under discipline: it carries decisions made
upstream (`reconciliation-ledger`, `charter-open-question-disposition`, HS-P0024's hand-off
note) into prose an atom can be cut from, without re-deciding any of them and without
letting a qualification fall off in transit.

## Context pack

**This story writes staged material, and staged material is not an atom.** `.kb/_intake/`
is explicitly raw input — "not held to `KbFrontmatter`" (`.kb/_intake/README.md:7-11`) — and
`redkiln validate --kb` skips every `_`-prefixed directory (`:21-27`). Two consequences bind
immediately: a green `validate --kb` says *nothing* about this story's output, so it is not
evidence here; and inventing atom frontmatter on a staged file is a category error that
makes the ingest's own adjudication harder, not easier. Write the argument; let the ingest
mint the id.

**`/redkiln:kb-ingest` is the composition root, and `.kb/_intake/` is the only door into
it.** It is the single writer of `.kb/product/` (`.kb/_intake/README.md:3-5`; CLAUDE.md,
"Where the work lives"), and hand-writing the directory layout of the process without the
process was reverted once already at `0269720`. This story therefore *cannot* produce an
atom and must not try; it produces the thing the writer reads. Staging is the input, ingest
is the mount (`_decomposition.md`, `## Architecture brief`, §2).

**The staged side and the ingested side are tested independently, and this story owns the
staged side.** The testing brief is explicit: Tier 2 reads the staged documents *before*
ingest and asks whether the four persona slots and the three qualifications exist in the
source material at all; Tier 1 reads the landed atoms after and asks whether the shape
survived. "A pass on one side is not evidence about the other"
(`_decomposition.md`, `## Testing brief`, `### Fixtures and seams`). A slot missing here
cannot be repaired by the ingest — it can only be invented by it, which is the failure this
project exists to prevent.

**The four slots are non-negotiable and non-delegable.** Every persona document states goal,
context, **what they already do instead**, and what they are afraid of
(`.kb/product/README.md:11-13`). IQ-1's falsifier is precise and is the named wrong
implementation for this story's tier: a document whose "what they are afraid of" or "what
they already do instead" reads *"see `_discovery/distillation/personas-and-journeys.md`"*.
Links carry evidence and provenance; they never carry a slot. The whole reason the promoted
layer exists is that the next initiative's charter author (U1) should not have to open the
discovery corpus to frame `GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>`.

**The evaluator decision is transcribed, never re-made.** `charter-open-question-disposition`
(this story's dependency, HS-S0175) decides whether the evaluator is a persona in its own
right or an earlier stage of the application author's journey, and marks the charter's
question at `.bklg/docs-that-teach/initiative.md:538-540` answered. This story lifts that
decision *and its reasoning* into the staged text, because AC-003 requires the decision to
appear in the atoms, not only in the backlog record — and requires it to appear **once**. A
second, differently-worded statement of the same decision anywhere in the payload is the
defect, not a redundancy. The staged set's cardinality follows: decided as its own persona →
three persona documents; decided as a stage → two, and the "Survive the second question"
journey names the application author at their evaluating stage. Either way there are **four**
journey documents, because the charter names four journeys
(`.bklg/docs-that-teach/initiative.md`, `## Referenced personas & journeys`).

**The observation qualification is corrected against a named upstream artefact, not
improvised.** The distillation's blanket *"None of these three personas has been directly
observed by this project"* (`_discovery/distillation/personas-and-journeys.md`, `## Risks`,
first bullet) must not be copied forward. The replacement text comes from HS-P0024's hand-off
note — its AC-009 requires "which single persona was directly observed, on what date, against
which tree, in the form HS-P0025 needs"
(`.bklg/docs-that-teach/comprehension-evidence/project.md`, AC-009), authored by
`handoff-note-to-closeout` and resting on DT-9 as resolved in
`.bklg/docs-that-teach/comprehension-evidence/_design.md`. Lift it; do not choose a persona
here. And carry the *stated choice* with it: the distillation's own fourth-persona note warns
that the friction-log reader "is not automatically identical to any one of the three"
(`## Risks`, second bullet), so the staged text must say the identification was decided at
DT-9 rather than assumed. The other personas are marked `still inferred`, in those words.

**Three qualifications travel, and they travel *adjacent to the claim*, not in a footer.**
(a) Per-persona observation status, per the previous paragraph. (b) The adapter author's
"no third-party adapter exists to read" claim rests on the seed's internal audit alone and
is corroborated by no research digest (`## Risks`, third bullet) — that sentence belongs on
that claim, in the adapter-author document. (c) The HS-S0131 overlap: a separately staged
four-persona set on an unmerged branch overlaps Personas 1 and 2 "almost exactly"
(`## Risks`, fourth bullet), and `reconciliation-ledger` (HS-S0174) has already adjudicated
it pair by pair — the surviving outcome, and which merge-order case held, travel into the
documents. IQ-7 is the bar: a caveat collected in a trailing "caveats" section that a reader
scanning the four slots never reaches is how `.kb/product/README.md:23-24, 28-31` says a
guess acquires the standing of a finding.

**The counterpart's default case is "it never ran".** `.bklg/` in this tree holds
`docs-that-teach` and `support` and nothing else — confirmed by listing, not inferred
(`_grounding.md`, `## Reconciliation counterpart does not exist in this tree`). The staged
text therefore states this initiative's set as the authored one **by default**, with the
HS-S0131 overlap as a named, unreconciled-in-fact qualification — never as a paragraph bolted
on if the merge turns out empty. If `reconciliation-ledger` recorded the other case, the
staged text follows *its* record; it does not re-read the sibling branch itself.

**No ADR governs this surface, and looking for one is the error.** All sixteen accepted
decision atoms govern the Rust contract — port flavours, opaque payloads, the wire format,
the MSRV — and the charter says so outright: "none of the seventeen decisions concerns
documentation" (`.bklg/docs-that-teach/initiative.md:524-525`). Citing an ADR here would be
manufacturing authority, which is the same defect as manufacturing a persona. What binds
instead is `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (correct by
superseding, never by editing), the single-write-path rule, and `harvest_kb: true` on the
`closeout` stage only.

**What must not travel into a staged document**, because the ingest will faithfully carry it
into an atom: requirements, scope or acceptance criteria (they stay in `.bklg/`); a screen, a
flow or an interaction pattern (a journey says what the persona is trying to accomplish and
in what order, never which control they click); a market segment; and the reconciliation
record itself. That last one is the live temptation the architecture brief names directly
(§4): the record is a project artefact, not durable knowledge about an audience. Its
*outcome* travels into the documents' bodies; the record stays under
`.bklg/docs-that-teach/durable-audience-closeout/`.

**Every state is a word.** No emoji, no tick, no colour word, no strikethrough, no empty
cell meaning "fine", no state inferable only from a row's absence. The corpus precedent is
literal — the open-questions index writes `Open` / `Withdrawn` / `Superseded` as words, first
(`.kb/maps/open-questions-index.md:136-143`). Here the words that matter are
`directly observed` and `still inferred`.

**The persona-journey slice this realizes.** U1, the next initiative's charter author, is the
reader — not the three documented personas, who are the *subject* of this material and never
its audience (`_decomposition.md`, `## UX brief`, "Who this is actually for"). U2, the
closeout reviewer standing at the ingest approval gate, is the falsification-shaped reader:
one document per decision is what lets them disagree with exactly one thing without reading
the discovery corpus.

## Integration contract

- **Archetype**: `capability` — a user-observable slice, where the user is U1/U2 and the
  observable is the staged payload a reviewer reads at the ingest approval gate.
- **Slice / milestone**: `product-layer-promotion`. Slice-mates, implemented in the same
  context and mounted as one integrated surface, in this order:
  `staged-audience-payload` → `audience-ingest-wave` → `product-layer-mounting`
  (`_storymap.md`, `## Merge order`, §3). The whole wave lands as **one commit on its own
  branch** and is reviewed by merging (`.kb/_intake/README.md:13-19`; IQ-4) — staging without
  ingest is not an atom, so this story is never delivered alone.
- **Mount point**: **`.kb/_intake/`** — the input directory the composition root
  `/redkiln:kb-ingest` reads, and the only path by which anything reaches `.kb/product/`
  (`.kb/_intake/README.md:3-5`). This story mounts by writing files the wave's narrowed
  invocation will name. Concretely, the handoff is consumed by
  `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md`, whose
  AC-A02 obligation ("invoked against an explicit file list or a glob that excludes
  `.kb/_intake/README.md`") is satisfiable *because* every file this story writes carries the
  `persona-` or `journey-` prefix. A staged document this story writes under a name that
  cannot be enumerated by that wave is unmounted.
- **Wires into**:
  - `.kb/product/README.md:6-13` — the tier table and the four-slot persona vocabulary the
    staged text must be cuttable into. Read the layer README, never the neighbouring atom
    (T1: every atom an author will look at while composing carries `authority_tier: note` or
    `guideline`; the product layer requires `product`).
  - `.kb/_templates/atom.md` — the frontmatter vocabulary and the
    `## Context` / `## Body` / `## Consequences / links` skeleton. Compose to it; do not
    invent a "Persona card", a "TL;DR" or a bespoke per-document table.
  - `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:1-30` (concept exemplar)
    and `.kb/playbooks/one-decision-per-adr-title.md` (playbook exemplar) — for **shape
    only**; the tier differs.
  - `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` — the substance,
    and the three qualifications in its `## Risks` section.
  - `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md` (HS-S0174)
    — the adjudicated outcome and the merge-order case.
  - `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md`
    (HS-S0175) — the evaluator decision and its reasoning.
  - `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md` and
    `.bklg/docs-that-teach/comprehension-evidence/_design.md` — the directly-observed persona,
    its date and its tree, and the DT-9 resolution behind the identification.
- **Renders surfaces**: **none from `_design.md`.** This project's signed-off design records
  `hasSurface: false` and carries no `## Items` block — "no items — no public API surface, no
  rendered UI surface". The binding surface contract for this story is therefore the UX
  brief's, not the design's: IQ-1 (in place, not a context jump), IQ-2's fourth bullet
  (clearing `.kb/_intake/` must not hide the evidence), IQ-6 (prior citations keep meaning
  what they meant), IQ-7 (the qualification travels with the claim), IQ-8 (no state expressed
  by presence, absence or ordering), and AC-UX-01, AC-UX-04, AC-UX-05, AC-UX-09.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story adds no
  Rust, touches no port and changes no `crates/` file, so there is no `suite.rs` rule that
  could observe it. Its automated readers are `git diff` and `test -f`; its only tier that can
  fail a plausible wrong implementation is Tier 2, human content review against IQ-1…IQ-8
  (`_decomposition.md`, `## Testing brief`, tier mapping for AC-003, AC-006, AC-007).
- **Clause(s)**: **none.** No `spec/SPECIFICATION.md` clause is discharged or amended, and
  none is touched — `spec/` is read-only for this whole project
  (`_decomposition.md`, `## Architecture brief`, §1). No `[FROZEN]` clause is in play, so no
  ADR is owed.
- **Advances DoD scenario**: **DoD-15 — "The audience is durable and reconciled"**
  (`.bklg/docs-that-teach/initiative.md`, `## Definition of Done`, item 15). This story does
  not turn it green on its own: it supplies the material `audience-ingest-wave` turns into
  atoms and `product-layer-mounting` links from the closeout. It is the step at which the
  *content* of DoD-15 becomes real; the remaining two are shape and reachability.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file
changed outside it.

```
.kb/_intake/persona-*.md
.kb/_intake/journey-*.md
.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/**
```

**In this PR**

- The staged persona documents — one per persona the evaluator decision leaves standing
  (two or three), each carrying the four slots, its observation-status word, and the
  qualifications that attach to its own claims.
- The four staged journey documents, one per journey the charter names, each naming the
  persona it belongs to so the reciprocal `related` edge `product-layer-mounting` writes has
  both ends available.
- The evaluator decision and its reasoning, transcribed once, in the one document the
  decision makes it belong to.
- This story's `_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) and its
  stage artefacts under its own folder.
- A work commit recorded via `redkiln record-links <id> --sha`
  (`require_commit_provenance: true`, `.redkiln/config.yaml:73`).

**Explicitly not in this PR**

- **Running the ingest.** `audience-ingest-wave` owns the invocation, the narrowed file
  list, the landed atoms, the cleared directory and `redkiln validate --kb`
  (project AC-004, AC-005, AC-008, AC-009, AC-011).
- **Any file under `.kb/product/`, `.kb/open-questions/`, `.kb/maps/` or `.kb/decisions/`.**
  The first two are the ingest's to write; the maps are `product-layer-mounting`'s; and
  `.kb/decisions/` is untouchable by this project at all (AC-A09 — a reconciliation finding
  that wants a decision is a routed gap in prose, never an ADR written as closeout exhaust).
- **`.kb/_intake/README.md`.** Not edited, not deleted, not cited in any staged document's
  evidence list. It is not an atom (`.kb/_intake/README.md:29-31`), and the whole point of
  the naming rule above is that it can never be swept into the wave.
- **Staged files belonging to a sibling story.** `charter-open-question-disposition` may
  stage an `open_question` document into the same directory; its files are its own and this
  story neither writes nor edits them. Likewise HS-P0021's carried `.kb/playbooks/` payload —
  vehicle versus content (AC-A10).
- **Deciding the evaluator question, deciding the reconciliation outcome, or deciding which
  persona was observed.** All three are upstream. Transcription with a citation, never a
  fresh judgement.
- **Anything under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `examples/`.**
  Read-only for this project; a needed change there is a routed gap
  (`.redkiln/config.yaml:5`), not a scope extension.

**Merge DoD one-liner** — merged when every staged document under `.kb/_intake/persona-*.md`
and `.kb/_intake/journey-*.md` yields, opened alone, the four slots, the transcribed evaluator
decision, the three qualifications adjacent to their claims and an observation-status word
that is `directly observed` or `still inferred`; `.kb/_intake/README.md` is byte-identical;
nothing outside the fenced boundary changed; and `cargo xtask affected --base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| One staged document per intended atom | Granularity is fixed here because the briefs left it open. Each file is the sole `.kb/_intake/…` source the corresponding atom will cite, so `source_paths` resolves one-to-one and the approval-gate reviewer reads one document per decision they might reject. | `_decomposition.md`, `## Architecture brief`, `#6. Deliberately not prescribed` ("How many staged intake documents"); `.kb/_intake/README.md:17-19` |
| File naming is `persona-<slug>.md` / `journey-<slug>.md` | Structural, not stylistic: the prefix set is disjoint from `README.md`, so the next story's invocation is narrowable by a glob that cannot match the README. It also keeps this story's files distinguishable from a sibling's staged `open_question` document in the same directory. | `.kb/_intake/README.md:5`; `_decomposition.md`, `## Architecture brief`, T4 and AC-A02 |
| Persona documents: two or three, decided upstream | Three if the evaluator was decided a persona in its own right; two if decided an earlier stage of the application author's journey. The count is read off `charter-open-question-disposition`'s record — never chosen here. | `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md`; `.bklg/docs-that-teach/initiative.md:538-540` |
| Journey documents: exactly four, always | *The first fifteen minutes*; *Model my invariant in your words*; *Walk the adapter path, not just the recipe*; *Survive the second question*. If the evaluator folded, the fourth names the application author at their evaluating stage rather than disappearing. | `.bklg/docs-that-teach/initiative.md`, `## Referenced personas & journeys` |
| Each persona document answers four questions without a link | Goal; context; what they already do instead; what they are afraid of. A slot whose text is a pointer into `_discovery/` is the named wrong implementation for this story. | `.kb/product/README.md:11-13`; `_decomposition.md`, `## UX brief`, IQ-1 and AC-UX-01 |
| Each journey document is a path, not a surface | Moment-by-moment steps through a task, written so a later reader can tell whether a build improved it. Never a screen, a flow, an interaction pattern or a control name — a journey that transcribes today's surface goes stale when the surface moves. | `.kb/product/README.md:33-36`; `_decomposition.md`, `## Architecture brief`, §4 ("Journey body") |
| Each journey document names its persona | So `product-layer-mounting` can write reciprocal `related` / `depends_on` edges without inferring the pairing. Links are two-way in this corpus. | `_decomposition.md`, `## Architecture brief`, §2, mount point 3 |
| The evaluator decision appears exactly once, with reasoning | Transcribed verbatim in substance from the disposition record, placed in the document the decision makes it belong to. A second, differently-worded statement elsewhere in the payload fails AC-003's "one place". | `.bklg/docs-that-teach/durable-audience-closeout/project.md`, AC-003; `_decomposition.md`, `## Testing brief`, AC-003 row (Tier 2 primary, Tier 1 `rg` for a contradicting statement) |
| Observation status is a word, per persona | Exactly one persona carries `directly observed`, with the date and the tree from HS-P0024's hand-off note and a sentence saying the identification was decided at DT-9 rather than assumed. Every other persona carries `still inferred`. The string "none has been directly observed" appears nowhere. | `.bklg/docs-that-teach/comprehension-evidence/project.md`, AC-009; `.bklg/docs-that-teach/comprehension-evidence/_design.md` (DT-9); `_discovery/distillation/personas-and-journeys.md`, `## Risks`, bullets one and two |
| Qualification (b) sits on the adapter author's claim | The "no third-party adapter exists to read" statement carries, in the same place, that it rests on the seed's internal audit alone and is corroborated by no research digest. | `_discovery/distillation/personas-and-journeys.md`, `## Risks`, third bullet; project DR-4 |
| Qualification (c) sits on the reconciliation-derived claim | The HS-S0131 overlap is stated where the audience's provenance is stated, together with the merge-order case `reconciliation-ledger` recorded — with the counterpart-never-ran case as the default shape, not an afterthought. | `_grounding.md`, `## Reconciliation counterpart does not exist in this tree`; `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md` |
| No qualification is collected in a trailing block | IQ-7: a caveat a reader scanning the four slots never reaches is how a guess acquires the standing of a finding. Adjacency is the contract; a "Caveats" heading carrying any of the three is the falsifier. | `_decomposition.md`, `## UX brief`, IQ-7; `.kb/product/README.md:23-24`, `:26-31` |
| Evidence is named inline so the ingest can lift it | Each document names the `_discovery/` artefacts its claims come from, plus the upstream records for transcribed decisions. Every named path resolves at staging time; the ingest adds this file's own `.kb/_intake/…` path on the far side. | `_decomposition.md`, `## Architecture brief`, §4 (`source_paths`, and the verified discovery set); `_decomposition.md`, `## UX brief`, AC-UX-05 |
| Staged files claim no atom identity | No `id:`, no `kind:`, no `authority_tier:`, no invented tracking keys. Staged material is not held to `KbFrontmatter`, and asserting a tier here is how the wrong tier gets copied forward. | `.kb/_intake/README.md:7-11`; `_decomposition.md`, `## Architecture brief`, T1 |
| Backlog-shaped content stays in the backlog | No requirements, scope, acceptance criteria or market segment in a staged document; the reconciliation record is not staged as an atom — its outcome travels, the record does not. | `.kb/product/README.md`, `## What does not belong here`; `_decomposition.md`, `## Architecture brief`, §4 |
| Every state is a literal word | No emoji, tick, colour word, strikethrough, ordering or empty cell carries meaning anywhere in this story's diff. | `_decomposition.md`, `## UX brief`, `### Accessibility floor` and AC-UX-09; `.kb/maps/open-questions-index.md:136-143` |
| Nothing this story writes moves an existing anchor | Append-only in effect: no existing file is edited, so no `file:line` anyone already cited shifts. `.kb/_intake/README.md` is byte-identical after this PR. | `_decomposition.md`, `## UX brief`, IQ-3; `.kb/_intake/README.md:29-31` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story writes plain markdown into
`.kb/_intake/`, a directory `redkiln validate --kb` deliberately does not read
(`.kb/_intake/README.md:21-27`), and it introduces no Rust type, no serialized form and no
persisted state. The nearest thing to a schema in play is `KbFrontmatter`, and it is
deliberately *not* applied here: staged material is raw input, and the frontmatter is minted
on the far side of the ingest by `audience-ingest-wave`.

The one data-shaped obligation that does bind is a lifecycle rather than a migration: a
successful ingest **clears** `.kb/_intake/`, and the whole wave — the new atoms and the
removal of these staged sources — commits together (`.kb/_intake/README.md:13-19`). Nothing
is lost, because the staged document stays in git history and every atom it fed cites its
`.kb/_intake/…` path. That is why this story must not treat its own files as durable
artefacts to be preserved, and why it must not stage anything whose only copy would be
destroyed by the clearing step.

## Acceptance criteria

Each criterion is framed from the reader's intent, not from a capability. The readers are the
ones the UX brief names: **U1**, the next initiative's charter author, who must be able to frame
`GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>` from one open file; **U2**, the
closeout reviewer standing at the ingest approval gate, who must be able to disagree with exactly
one decision without reading the discovery corpus; and **U3**, the future maintainer reconciling a
third persona set, who must be able to see what was decided, against which tree, on what evidence
(`_decomposition.md`, `## UX brief`, "Who this is actually for").

"Tier N" below is the testing brief's tier, not an invented scale
(`_decomposition.md`, `## Testing brief`, "The test mix"). Tier 2 is a human reading one claim at
a time against the checklist the UX brief already carries; it is named as primary wherever the
failure mode is content rather than shape, per AC-TB-01.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN U1, who has never opened this initiative's discovery corpus, WHEN they open one staged persona document under `.kb/_intake/persona-<slug>.md` and read nothing else, THEN that file states, as prose in itself, the persona's goal, their context, what they already do instead, and what they are afraid of — four slots, none of them discharged by a link — so U1 can write a `GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>` criterion from that one file. | **Tier 2 primary**, one document at a time against `_decomposition.md`, `## UX brief`, IQ-1 and AC-UX-01, recorded per document in `_ledger.md`. **Tier 1 backstop:** `rg -n "already do instead" .kb/_intake/persona-*.md` and `rg -n "afraid of" .kb/_intake/persona-*.md` each hit every persona file; `rg -n "_discovery/distillation/personas-and-journeys" .kb/_intake/persona-*.md` shows no hit on a line that is itself one of the four slots. |
| AC-002 | GIVEN U1 choosing which journey a new initiative will improve, WHEN they list `.kb/_intake/journey-*.md`, THEN there are exactly four documents — *The first fifteen minutes*, *Model my invariant in your words*, *Walk the adapter path, not just the recipe*, *Survive the second question* — and each reads as a moment-by-moment path through a task, names in its own text the persona it belongs to, and contains no screen, control, route or interaction-pattern name, so a later reader can tell whether a build improved it rather than whether a surface moved. | **Tier 2 primary** against `.kb/product/README.md:33-36` and `_decomposition.md`, `## Architecture brief`, §4 ("Journey body"). **Tier 1 backstop:** `ls .kb/_intake/journey-*.md` returns exactly four paths; `rg -n "persona" .kb/_intake/journey-*.md` hits every one. Cross-read against `.bklg/docs-that-teach/initiative.md`, `## Referenced personas & journeys`, for the four titles. |
| AC-003 | GIVEN U2 at the ingest approval gate asking whether the evaluator was settled or quietly left open in two distillations, WHEN they read the staged payload end to end, THEN the evaluator decision appears **exactly once**, in the document the decision itself makes it belong to, carrying the reasoning transcribed from `charter-open-question-disposition` rather than re-argued here — and the persona-document count that follows from it (three if a persona in its own right, two if an earlier stage of the application author's journey) is what is actually on disk. | **Tier 2 primary:** one read of the whole staged set plus the disposition record, against project AC-003. **Tier 1 backstop:** `rg -n -i "evaluator" .kb/_intake/persona-*.md .kb/_intake/journey-*.md` yields exactly one passage stating the decision, and no second, differently-worded statement of it; `ls .kb/_intake/persona-*.md` returns the count the disposition record implies. Cross-check `.bklg/docs-that-teach/initiative.md:538-540` is the question being answered. |
| AC-004 | GIVEN U1 deciding which claims they may frame a criterion from and which they must re-check first, WHEN they read any staged persona document, THEN that document carries an observation-status **word** — `directly observed` or `still inferred` — and exactly one document across the set carries `directly observed`, with the session date and the tree ref lifted from HS-P0024's hand-off note and a sentence saying the identification was decided at DT-9 rather than assumed; the blanket "none has been directly observed" appears nowhere. | **Tier 2 primary** against `_decomposition.md`, `## UX brief`, AC-UX-04, and project AC-007. **Tier 1 backstop:** `rg -n "directly observed" .kb/_intake/persona-*.md` hits exactly one file; `rg -n "still inferred" .kb/_intake/persona-*.md` hits every other; `rg -n "none has been directly observed" .kb/_intake/` returns nothing; the date and tree ref match `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md`'s slot table verbatim. |
| AC-005 | GIVEN U2 scanning the four slots of a persona document and stopping there, WHEN they reach the claim each qualification qualifies, THEN the qualification is already in front of them — the observation status beside the persona's evidence, "rests on the seed's internal audit alone, corroborated by no research digest" beside the adapter author's "no third-party adapter exists to read", and the HS-S0131 overlap beside the statement of where this audience came from — and no staged document collects any of the three in a trailing caveats block a scanning reader never reaches. | **Tier 2 primary**, and it is the only tier that can fail this: read each qualification and check the claim it sits next to, per `_decomposition.md`, `## UX brief`, IQ-7 and AC-UX-04, and `.kb/product/README.md:23-24`, `:26-31`. **Tier 1 backstop:** `rg -n -i "^#+ .*caveat" .kb/_intake/` returns nothing; `rg -n "internal audit" .kb/_intake/persona-*.md` and `rg -n "HS-S0131" .kb/_intake/` each hit. |
| AC-006 | GIVEN U3 asking a year from now whether this audience was reconciled or merely authored, WHEN they read the staged document that states where this audience came from, THEN it states the merge-order case `reconciliation-ledger` actually recorded — with the counterpart-never-ran case as the payload's default written shape, not a paragraph bolted on — names the merged tree by the sha that record names, and does not re-read or re-adjudicate the sibling branch here; and the reconciliation record itself is nowhere staged as an atom, only its outcome. | **Tier 2 primary**, the parametric read the testing brief describes under "The unrun counterpart": the stated case must match what `ls .bklg` shows, **and** the prose must still parse sensibly under the other case. **Tier 1 backstop:** `rg -n "HS-S0131\|counterpart" .kb/_intake/` hits; the sha string in the staged text matches the one in `reconciliation-ledger`'s record; no staged file is a copy of that record. |
| AC-007 | GIVEN U2 opening any staged document expecting the corpus's shared shape rather than this story's invention, WHEN they read it, THEN it composes the repository's own primitives — the `# title` plus the atom template's `## Context` / `## Body` / `## Consequences / links` skeleton, and the product README's exact four-slot vocabulary — and hand-rolls none of the alternatives the UX brief forbids: no "Persona card", no "TL;DR", no bespoke per-document table, no frontmatter key absent from `.kb/_templates/atom.md`, no `id:` / `kind:` / `authority_tier:` claiming an atom identity the ingest has not minted, and no subdirectory under `.kb/_intake/`. | **Tier 2 primary** against `_decomposition.md`, `## UX brief`, "Design-system primitives — compose these, do not hand-roll" and its forbidden list. **Tier 1 backstop:** `rg -n "^## " .kb/_intake/persona-*.md .kb/_intake/journey-*.md` shows the three skeleton headings and nothing bespoke; `rg -n "^(id|kind|authority_tier|last_reviewed):" .kb/_intake/persona-*.md .kb/_intake/journey-*.md` returns nothing; `ls -d .kb/_intake/*/` returns nothing. |
| AC-008 | GIVEN the next story's implementer needing an invocation that provably cannot sweep `.kb/_intake/README.md` into the wave, WHEN they enumerate what this story staged, THEN every file is `.kb/_intake/persona-<slug>.md` or `.kb/_intake/journey-<slug>.md` — a prefix set disjoint from `README.md` and from a sibling's staged `open_question` document — and `.kb/_intake/README.md` is byte-identical to its pre-PR state, so the one act review cannot cheaply undo is prevented at invocation rather than repaired after the commit. | **Tier 1:** `ls .kb/_intake/` shows only `README.md` plus this story's prefixed files and any sibling's own; `git diff -- .kb/_intake/README.md` is empty. **Tier 3 handoff check:** the file list here is exactly what `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md`'s narrowed invocation enumerates, per `_decomposition.md`, `## Architecture brief`, AC-A02 and T4. |
| AC-009 | GIVEN a maintainer reading a landed atom after the ingest has cleared `.kb/_intake/`, WHEN they follow the evidence the atom inherited, THEN it resolves — because every staged document named its `_discovery/` artefacts inline at the claims they support, every named path resolved at staging time, and no staged document cited `.kb/_intake/README.md` as evidence; a charter that already cited the distillation and a charter that cites the promoted atom are pointed at the same audience rather than two. | **Tier 1:** `test -f` over every path cited in every staged document, all pass; `rg -n "_intake/README" .kb/_intake/persona-*.md .kb/_intake/journey-*.md` returns nothing. **Tier 2:** the cited artefact actually supports the claim it sits beside, against `_decomposition.md`, `## UX brief`, IQ-6 and AC-UX-05, and the verified discovery set in `## Architecture brief`, §4. |
| AC-010 | GIVEN a reader consuming this payload one line at a time — in a diff, a quote, or a screen reader — WHEN they encounter any state this story expresses, THEN it is a literal word: no emoji, tick, colour word, strikethrough, empty cell, ordering or absence carries meaning anywhere in this story's diff; each document has one `#` heading, sections at `##`, no skipped level, and link text naming its destination rather than "here" or "see above"; and nothing outside this story's own folder and its new `.kb/_intake/` files is modified, so no `file:line` anyone already cited moves. | **Tier 1:** `git diff --stat` lists only new `.kb/_intake/persona-*.md` / `journey-*.md` files and this story's own folder; `git diff` shows no `-` line outside them. **Tier 2** against `_decomposition.md`, `## UX brief`, `### Accessibility floor`, AC-UX-09 and IQ-3/IQ-8, and the corpus precedent at `.kb/maps/open-questions-index.md:136-143`. |

**Coverage of the traced project criteria.** Project AC-003 is carried by AC-003 (the transcription
half of the seam `_storymap.md`, `## Coverage` names — `charter-open-question-disposition` decides,
this story transcribes). Project AC-006 is carried by AC-005 with AC-004 and AC-006 supplying two of
its three qualifications in their own right. Project AC-007 is carried by AC-004. The remaining six
criteria are the invariants that make those three survive the ingest rather than pass on a document
nobody can use: AC-001, AC-002 and AC-007 are what make the payload cuttable into the shape project
AC-004 later demands; AC-008 is what makes `audience-ingest-wave`'s AC-A02 satisfiable; AC-009 is
what makes project AC-005 achievable on the far side; AC-010 is the accessibility floor the UX brief
binds today rather than on a rendered page.

## Interaction quality

This project's signed-off design records `hasSurface: false` and carries no `## Items` block
(`_design.md`, "no items — no public API surface, no rendered UI surface"), so there is no
composition, density or chrome decision to inherit from it and none to contradict. What binds
instead — and the design says so by excluding itself — is the UX brief's **primitive layer**: the
atom template, the frontmatter vocabulary and the product README's four-slot vocabulary are a real
shared shape with a real validator behind them, which is what makes "compose, do not hand-roll" a
testable instruction here rather than a metaphor (`_decomposition.md`, `## UX brief`, Notes,
"'Design system' is being used honestly").

Every invariant below is already an `AC-###` row in the table above. This section says only **which
row carries which invariant and how it is falsified**; nothing here is a bullet that escapes the
ledger.

**State invariants.**

| Invariant | Carried by | Falsifier |
| --- | --- | --- |
| In place, not a context jump (IQ-1) | AC-001, AC-002 | A slot whose text reads "see `_discovery/distillation/personas-and-journeys.md`". U1 opens one file and cannot write a criterion from it. |
| Non-occlusion: a qualification is not hidden behind a disclosure (IQ-2, IQ-7) | AC-005, AC-006 | Any of the three qualifications living in a trailing "Caveats" heading, or the merge-order case stated only in the reconciliation record and not in the payload. |
| Preserved position — the text analogue of preserved focus, scroll and selection (IQ-3, IQ-6) | AC-009, AC-010 | A `-` line in `git diff` outside this story's own new files; or a promoted claim that forks the audience by citing nothing the earlier charter cited. |
| Reversibility (IQ-4) | AC-008 | `.kb/_intake/README.md` appearing in `git diff`, or a staged filename the wave's narrowed invocation cannot enumerate — either turns a pre-commit filter into a post-commit supersession problem. |
| Reachable without prior knowledge — the keyboard-reachability analogue (IQ-5) | AC-002, AC-008 | A journey document that never names its persona, leaving `product-layer-mounting` to infer the reciprocal `related` edge; or a staged file reachable only by knowing this story's slug. |
| Every state change legible at the moment of reading (IQ-8) | AC-004, AC-010 | An observation status expressed by a persona's absence from a list, by ordering, or by an empty cell rather than by the words `directly observed` / `still inferred`. |

**Composition invariants.** Taken from the primitive layer, since the design declares no surface.

| Invariant | Carried by | The real number or shape |
| --- | --- | --- |
| Presentation exists at all — every document carries real composed shape, not bare prose | AC-007 | `# title`, then `## Context`, `## Body`, `## Consequences / links` — the `.kb/_templates/atom.md` skeleton, three `##` sections, in that order. |
| Composition and placement | AC-007, AC-008 | Flat `.kb/_intake/`, no subdirectory; the four slots inside `## Body` using the product README's own words so they are greppable; evidence and links in `## Consequences / links`. |
| Transience — what is persistent, what is revealed, what is opened on demand | AC-008, AC-009 | Staged documents are **transient by contract**: the successful ingest removes them in the same commit that adds the atoms. What persists is the atom plus its `source_paths` entry naming this file, and git history. Nothing may be staged whose only copy the clearing step would destroy. |
| Density budget | AC-001, AC-002, AC-003, AC-005 | Exactly 4 slots per persona document. Exactly 4 journey documents. 2 or 3 persona documents, decided upstream. Exactly 1 statement of the evaluator decision across the whole payload. Exactly 3 qualifications, each inside the document carrying the claim it qualifies. Total staged payload: 6 or 7 files. `.kb/_intake/README.md`: 0 diff lines. |
| Hierarchy | AC-007, AC-010 | One `#` per file, sections at `##`, sub-structure at `###`, no skipped level — which is also what keeps the `## Context` / `## Body` skeleton legible. |
| Named anti-patterns | AC-004, AC-005, AC-007, AC-010 | The UX brief's forbidden list, one to one: a "Persona card"; a "TL;DR"; a bespoke per-document table; a frontmatter key absent from `.kb/_templates/atom.md`; a status carried by an emoji, tick, colour word or empty cell; a subdirectory under the layer; and — this story's own two — the blanket "none has been directly observed", and a trailing caveats block. |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | `charter-open-question-disposition` has not recorded the evaluator decision, or its record is ambiguous about which way it went. | Halt and report blocked. Do **not** decide it here and do not pick the cardinality that makes the payload easier to write — AC-003's whole seam is that this story transcribes. The dependency edge exists for this. |
| EC-002 | HS-P0024's hand-off note is missing a slot AC-004 needs — the persona, the date, or the tree ref. | Halt and report blocked against `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md`. Do **not** infer which persona the session walked from the friction log's contents, and do not fall back to the blanket phrase. Inferring here re-opens DT-9, which was resolved elsewhere. |
| EC-003 | `reconciliation-ledger` recorded the counterpart-**ran** case rather than the default unrun one. | Follow its record. The staged text states the case that record states and names its sha; this story does not re-read the sibling branch, does not re-adjudicate a pair, and does not add a paragraph the record does not support. |
| EC-004 | A `_discovery/` path a staged document wants to cite does not resolve in the merged tree. | Fix the citation against the verified discovery set (`_decomposition.md`, `## Architecture brief`, §4). Never drop the claim to avoid the citation, and never cite a path that exists only on the sibling branch — the charter already records two such paths as unreachable here. |
| EC-005 | A filename this story wants collides with a sibling's staged document already in `.kb/_intake/`. | Rename this story's file to a distinct `persona-` / `journey-` slug. Never overwrite or edit a file this story did not write; a sibling's staged `open_question` document is its own (PR boundary, "Explicitly not in this PR"). |
| EC-006 | `git diff` shows `.kb/_intake/README.md`, or any file outside the fenced PR boundary, as modified. | Revert the change before committing. `redkiln verify --grain story` fails on it, and after the wave commits, a README ingested as an atom is a supersession problem rather than a deletion (IQ-4). |
| EC-007 | Two staged documents each state the evaluator decision, in different words. | Delete one and leave the statement in the document the decision makes it belong to. Two statements is the defect AC-003 names, not a helpful redundancy — a later reader cannot tell which is authoritative, and the ingest would faithfully carry both into two atoms. |

## Non-functional

| id | Requirement | Why it is here rather than assumed |
| --- | --- | --- |
| NF-001 | **Self-sufficiency on first load.** A persona document is usable by U1 with no second file open. This is a property of the writing, not of the frontmatter, and nothing automated can observe it. | The UX brief's own Notes: "nothing checks whether a persona atom is *usable*". It is why Tier 2 exists at all. |
| NF-002 | **No new tooling, no Rust, no gate step.** This story adds no command, no script and no CI job; `cargo xtask affected --base main` runs and stays green because nothing it compiles changed. | `_decomposition.md`, `## Testing brief`, AC-TB-02 and AC-TB-08 — the Rust-grain steps function here as a regression net proving `crates/`, `spec/`, `xtask/` and `standards/` stayed untouched. |
| NF-003 | **Plain-text equivalence.** Every load-bearing verdict — an observation status, a merge-order case, the evaluator decision — is also stated in a sentence, not only as a table cell or a heading, so it survives being quoted or diffed out of context. | `_decomposition.md`, `## UX brief`, `### Accessibility floor`, "Plain-text equivalence for load-bearing claims". |
| NF-004 | **Linear readability.** Every row and every slot is self-contained: no passage that only makes sense read after the one above it. A reviewer consuming one claim at a time loses nothing. | Same floor; and it is what makes U2's "disagree with exactly one thing" possible. |
| NF-005 | **Reviewable in one sitting, per decision.** Each staged document holds one decision surface a reviewer might reject. A document that grows into a second document's subject defeats the one-file-per-decision granularity this spec fixed. | Executive summary, decision 1; `.kb/_intake/README.md:17-19`. |
| NF-006 | **ASCII-safe and diff-stable.** No character whose meaning depends on rendering; no trailing-whitespace or line-reflow churn in the files this story adds, since they are read in `git diff` at the approval gate as often as in an editor. | IQ-3's preserved-position rule, applied to this story's own additions. |

## Implementation notes (non-prescriptive)

These are hints, not obligations; the acceptance criteria above are the bar.

- **Read the two upstream records before writing a word.** `reconciliation-ledger`'s merge-order
  case and `charter-open-question-disposition`'s evaluator decision jointly fix the payload's
  cardinality and one whole document's contents. Writing first and reconciling after is how a
  second, differently-worded statement of the evaluator decision gets into the set (EC-007).
- **A plausible drafting order** is: the persona documents first (they carry the four slots and two
  of the three qualifications), then the journey documents (each naming a persona that now exists),
  then a last pass reading only for adjacency — walking each of the three qualifications back to the
  claim it sits beside. That last pass is the one AC-005 is written for, and it is easier as a
  separate read than as a running check.
- **Compose from the exemplars for shape and from the layer README for vocabulary.** The concept
  exemplar shows how a folded `summary` carries an argument rather than a label; the playbook
  exemplar shows the same skeleton a journey takes. Neither is a tier model — both carry
  `authority_tier: note` or `guideline`, and copying a tier from a neighbour is T1, the single
  highest-probability defect in this surface. Since staged files carry no frontmatter at all
  (AC-007), the tier trap is inherited by `audience-ingest-wave`, not sprung here; leaving the
  frontmatter off is the cleanest way to avoid handing it a wrong answer to copy.
- **The evaluator decision's home follows the decision.** Decided as a persona in its own right, it
  belongs in that persona's document. Decided as an earlier stage of the application author's
  journey, it belongs in the application author's document and the *Survive the second question*
  journey names them at that stage. Either way, one place.
- **Slug the files after what they are, not after this story.** `persona-application-author.md`,
  `journey-first-fifteen-minutes.md` and the like read as themselves at the approval gate and in
  every `source_paths` entry they later appear in; a slug carrying the story id would outlive the
  story and mean nothing to U3.
- **Do not stage the reconciliation record.** The temptation is real and the architecture brief
  names it directly: the record is a project artefact, its outcome is durable knowledge. Lift a
  sentence; leave the table.
- **`redkiln validate --kb` is not evidence here** and running it proves nothing about this story —
  it skips every `_`-prefixed directory. If it is run at all, run it to confirm nothing else in
  `.kb/` moved.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, `## Testing brief` — its four tiers, its AC-to-tier mapping and its
merge-gate ordering. No tier or command below is invented for this story; the two tiers this story
can actually be failed by are 1 and 2, and Tier 2 is primary wherever the failure mode is content.

| Tier | Command or path | Proves |
| --- | --- | --- |
| **1 — static / shape** | `ls .kb/_intake/` and `git diff -- .kb/_intake/README.md` | AC-008: only prefixed files were added, and the README is byte-identical. The one check that must not be substituted by `validate --kb`, which cannot see this directory. |
| **1 — static / shape** | `rg -n "already do instead" .kb/_intake/persona-*.md`; `rg -n "afraid of" .kb/_intake/persona-*.md`; `rg -n "none has been directly observed" .kb/_intake/`; `rg -n "directly observed" .kb/_intake/persona-*.md` | AC-001 backstop and AC-004: every persona file carries both hard slots by their real words; the forbidden blanket phrase is absent; exactly one file carries the observed status. |
| **1 — static / shape** | `rg -n "^(id\|kind\|authority_tier\|last_reviewed):" .kb/_intake/persona-*.md .kb/_intake/journey-*.md`; `rg -n "^## " .kb/_intake/persona-*.md .kb/_intake/journey-*.md`; `ls -d .kb/_intake/*/` | AC-007: no atom identity is claimed, the skeleton is the template's three sections, and the layer stayed flat. |
| **1 — static / shape** | `test -f` over every path cited in every staged document | AC-009: `source_paths` on the far side of the ingest will resolve, because the citations resolved at staging time. |
| **1 — static / shape** | `git diff --stat` and `git diff` over the whole tree | AC-010: additions only, confined to the fenced PR boundary; no pre-existing `file:line` moved. |
| **2 — content review** (the "unit" analogue; a human, one claim at a time) | The UX brief's own checklist — IQ-1 … IQ-8 and AC-UX-01, AC-UX-04, AC-UX-05, AC-UX-09 — read against each staged document, outcome recorded per AC in `_ledger.md` | AC-001 … AC-007 and AC-009's substance. This is the **only** tier that can fail the named wrong implementation: a document with valid shape whose "what they are afraid of" slot links out to the discovery corpus. Per AC-TB-06 this story maintains no second checklist. |
| **2 — content review**, parametric | Read the payload's stated merge-order case against what `ls .bklg` shows, then re-read the prose under the other case | AC-006: the record is coherent whichever way the counterpart went, which is the only form DR-2 accepts. |
| **3 — integration / mount-point** | The file list this story produces, checked against the invocation in `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md` | AC-008's mount half: every staged file is enumerable by the narrowed wave, so nothing this story wrote is unmounted and `.kb/_intake/README.md` cannot be swept in. |
| **story gate** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | NF-002: the diff maps to no workspace package, and the five file-reading lints plus `spec-trace` run unconditionally — the reason a documentation-only story is not a trivial green. |
| **story gate** | `redkiln verify --grain story` with `require_ledger: true` and `require_commit_provenance: true` (`.redkiln/config.yaml:62-73`) | Every AC above has a `_ledger.md` row flipped with cited evidence, no file changed outside the fenced boundary, and the work commit is recorded via `redkiln record-links <id> --sha`. |

**Deliberately not run as evidence for this story:** `redkiln validate --kb` (blind to
`.kb/_intake/` by design, `.kb/_intake/README.md:21-27`) and `cargo xtask ci` (the terminal grain,
owned by `terminal-gate-run`, and a run taken here would prove the gate rather than the deliverable).

## Risks and coupling (PR-scoped)

| Risk | Scope | Mitigation in this PR |
| --- | --- | --- |
| A slot is discharged by a link, and the payload passes every machine check | This PR | The named wrong implementation for AC-001. Tier 1 cannot see it; Tier 2's per-document read is the whole defence, and the `_ledger.md` row for AC-001 must cite the file and line where each of the four slots is answered — not merely that the document exists. |
| A qualification is written but drifts into a trailing block during editing | This PR | AC-005's separate adjacency pass, done as its own read rather than as a running check. The falsifier is mechanical enough to grep for a "Caveats" heading, but the real check is where each of the three sentences sits. |
| The evaluator decision gets restated "for clarity" in a second document | This PR | EC-007. One statement, in one document, chosen by where the decision itself lands. The reviewer's Tier 1 `rg` for the word is cheap and catches it. |
| A persona is chosen for `directly observed` because the hand-off note was ambiguous | This PR, and it propagates | EC-002 halts rather than infers. The distillation's own fourth-persona note is why: the friction-log reader is not automatically identical to any of the three, so the identification must arrive as a decision (DT-9) rather than as an assumption. |
| The payload is written against the pre-merge worktree copy of the discovery corpus | Upstream coupling | Ordering 1 — merge before everything. `merge-forward-baseline` produces the sha, `reconciliation-ledger` names it, and this story's citations must resolve in that tree, not this one. EC-004. |
| A staged filename cannot be enumerated by the next story's narrowed invocation | Downstream coupling | AC-008 is written as a mount criterion for exactly this reason. An unenumerable file is unmounted; and worse, an unnarrowed invocation reaches the approval gate with `.kb/_intake/README.md` already treated as content (T4). |
| This story's output is treated as the deliverable and reviewed alone | Slice coupling | It is not deliverable alone — staging without ingest is not an atom. The slice lands as one commit reviewed by merging; `audience-ingest-wave` and `product-layer-mounting` are implemented in the same context, in that order. |
| The reconciliation record is staged wholesale because it reads like knowledge | This PR | AC-006's last clause and the PR boundary. Its outcome travels; the record stays in `.bklg/`. `.kb/product/README.md`'s "what does not belong here" is the authority, not a preference. |

## Dependencies

**Blocks on**

- `reconciliation-ledger` — supplies the adjudicated outcome and the merge-order case AC-006
  transcribes, and the merged sha the payload's provenance statement names. Without it, this story
  would have to re-adjudicate, which is the failure the project exists to prevent.
- `charter-open-question-disposition` — supplies the evaluator decision and its reasoning, and with
  them the persona-document cardinality. Without it, AC-003 has nothing to transcribe and EC-001
  fires.

**Unlocks**

- `audience-ingest-wave` — consumes this story's file list as its narrowed invocation and turns the
  staged documents into `.kb/product/` atoms. It cannot start without a payload, and its AC-A02
  obligation is satisfiable only because of AC-008's naming rule.
- `product-layer-mounting` — transitively, through the wave. Its reciprocal `related` / `depends_on`
  edges are writable without inference only because AC-002 required each journey document to name
  its persona.

**Outside the story DAG**

- HS-P0024 `comprehension-evidence`, via `handoff-note-to-closeout` — a project-grain dependency
  already recorded on `project.md`'s `## Dependencies`, not a story edge this spec adds. It supplies
  AC-004's persona, date and tree ref, in the liftable form its own AC-009 required.

## Anchors (progressive disclosure)

Each row is a deeper artefact this spec deliberately did not paste. Open it at the moment named, for
the criterion named — not as background reading.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.kb/product/README.md` | The four-slot persona vocabulary (`:11-13`), the journey definition (`:33-36`) and the "what does not belong here" list are the exact words the staged text must be cuttable into. Read the **layer** README, never a neighbouring atom — that is T1. | Before drafting the first persona document, and again before the AC-005 adjacency pass. | AC-001, AC-002, AC-005, AC-007 |
| `.kb/_intake/README.md` | Establishes that staged material is not held to `KbFrontmatter` (`:7-11`), that a successful ingest clears the directory and the wave commits together (`:13-19`), that `validate --kb` is blind here (`:21-27`), and that the README itself is not an atom (`:29-31`). Every one of those is a constraint this story is graded on. | Before creating the first file, and again before writing the `_ledger.md` evidence for AC-008. | AC-007, AC-008, AC-009 |
| `.kb/_templates/atom.md` | The `## Context` / `## Body` / `## Consequences / links` skeleton and the frontmatter vocabulary the composition invariant binds to. It is also the list of keys a staged file must *not* invent. | While composing each document's structure. | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The substance of all three personas, and — in `## Risks` — the exact wording of the blanket observation claim to be corrected, the fourth-persona warning behind DT-9, the adapter author's internal-audit-only basis, and the HS-S0131 overlap. All three qualifications originate here. | While writing each persona's four slots, and immediately before the adjacency pass. | AC-001, AC-004, AC-005, AC-006, AC-009 |
| `.bklg/docs-that-teach/initiative.md` | `## Referenced personas & journeys` names the four journeys verbatim, and `:538-540` is the evaluator open question `charter-open-question-disposition` marks answered. Quote the journey titles from here rather than paraphrasing. | Before creating the journey files, and when placing the evaluator statement. | AC-002, AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md` | The evaluator decision and its reasoning, and therefore the persona-document cardinality. This story transcribes it; if the record is not there or is ambiguous, EC-001 fires rather than a judgement being made. | First, before any file is created — it decides how many exist. | AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/reconciliation-ledger/spec.md` | The merge-order case that actually held, the pair-by-pair outcome, and the merged sha. AC-006's default written shape is the counterpart-never-ran case; this record is what overrides it if the other case holds. | Before writing the provenance passage in whichever document carries it. | AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md` | Defines the five liftable slots of the `## Hand-off` section — persona, date, tree, scope sentence, second-session verdict — written so HS-P0025 can lift them without re-deriving. AC-004's date and tree ref come from here verbatim. | When writing the observation-status sentence, and again if EC-002 looks live. | AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | Carries the DT-9 resolution behind *which* persona the session walked — the decision this story must cite rather than assume, given the distillation's warning that the friction-log reader is not automatically one of the three. | Alongside the hand-off note, when writing the "decided at DT-9" sentence. | AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | Three briefs in one file: `## UX brief` (IQ-1…IQ-8, the forbidden hand-rolling list, the accessibility floor, AC-UX-01/04/05/09) is the Tier 2 checklist; `## Architecture brief` §4 fixes what an atom must satisfy and §6 names what was left open; `## Testing brief` fixes the tier per AC. | `## UX brief` before the Tier 2 review pass; `## Architecture brief` §4 while drafting; `## Testing brief` when filling `_ledger.md`. | AC-001, AC-002, AC-005, AC-007, AC-009, AC-010 |
| `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md` | The consumer. Its narrowed invocation must be able to enumerate exactly this story's files and exclude `.kb/_intake/README.md`; a name it cannot enumerate is an unmounted file. | Before choosing filenames, and again before committing. | AC-008 |
| `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` | `## Reconciliation counterpart does not exist in this tree` is the verified-by-listing basis for AC-006's default shape. It is why "no counterpart staged" is the payload's written case rather than a paragraph added if the merge turns out empty. | When drafting the provenance passage, before assuming the counterpart ran. | AC-006 |
| `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | Concept-atom exemplar, for **shape only**: how a folded `summary` carries a whole argument rather than a label, and how `source_paths` cites both the `.kb/_intake/…` staging path and the long-form evidence. Its `authority_tier` is not the model. | When a document's argument is written but its shape feels invented. | AC-007, AC-009 |
| `.kb/playbooks/one-decision-per-adr-title.md` | Playbook-atom exemplar — the shape a journey atom takes on the far side of the ingest, so the staged journey text is cuttable into it. Shape only; the tier differs. | While drafting the four journey documents. | AC-002, AC-007 |
| `.kb/maps/open-questions-index.md` | Lines `:136-143` are the corpus precedent for status-as-a-word — `Open` / `Withdrawn` / `Superseded`, stated first. The direct model for `directly observed` / `still inferred`. | When writing any status word. | AC-004, AC-010 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The accepted governance atom that makes "correct by superseding, never by editing" binding here in the absence of any documentation ADR. It is what forbids both editing a decision and inventing one as closeout exhaust. | If a correction to an upstream record seems necessary while drafting. | AC-006, AC-010 |
| `.redkiln/config.yaml` | `:40` the story-grain command, `:62-67` `require_ledger`, `:73` `require_commit_provenance` — the three mechanisms that actually gate this story. | Before the final commit. | AC-008, AC-010 |

## Clarifications resolved during spec

1. **Ten acceptance criteria, and no change to the front half's enumeration.** AC-001 … AC-010 as
   the first pass declared them. Three carry the traced project criteria directly (AC-003 → project
   AC-003; AC-004 → project AC-007; AC-005 with AC-004 and AC-006 → project AC-006); the other six
   are the invariants without which those three pass on a payload nobody can use. None was added or
   dropped.
2. **The interaction-quality invariants are AC rows, not prose.** Every state and composition
   invariant that applies is carried by a row in the acceptance-criteria table; `## Interaction
   quality` names which row carries which and how each is falsified. A bullet in that section would
   never have reached the ledger and never been gated.
3. **The composition contract comes from the UX brief's primitive layer, not from `_design.md`.**
   The signed-off design records `hasSurface: false` and no `## Items`, so it binds this story by
   what it excludes. Rather than declare the composition family N/A, this spec binds the real shared
   shape the repository has — the atom template, the four-slot vocabulary, the forbidden
   hand-rolling list — with real numbers in the density budget. That is the honest reading of
   "binding on any story that renders a surface" for a story whose surface is a text corpus.
4. **Tier 2 is named primary wherever the failure is content.** AC-001, AC-002, AC-003, AC-005,
   AC-006 and AC-007 all carry Tier 2 as primary with Tier 1 only as a backstop, matching the
   testing brief's AC-TB-01 prohibition on citing a tier that cannot falsify the criterion. The
   `_ledger.md` rows for those criteria must cite a `file:line` in the staged document, not a
   command's exit code.
5. **`redkiln validate --kb` is explicitly excluded as evidence for every AC here.** It skips
   `_`-prefixed directories, so a green run says nothing about this story's output. This is stated
   in the merge-gate table rather than left to be rediscovered, because a green validator is the
   most available false comfort on this surface.
6. **The observation-status trigger for a halt is EC-002, not a fallback.** Where the hand-off note
   is incomplete, this story blocks rather than inferring which persona the session walked. The
   alternative — picking the most likely persona — would re-open DT-9 silently and manufacture
   exactly the kind of unmarked assumption the project exists to prevent.
7. **Density has real numbers.** Four slots, four journeys, two or three persona documents, one
   evaluator statement, three qualifications, six or seven staged files, zero diff lines on
   `.kb/_intake/README.md`. They are written into the composition table so a reviewer can count
   rather than judge.
