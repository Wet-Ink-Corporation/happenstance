---
item: HS-S0178
stage: spec
created: 2026-08-17T13:16:29.278Z
updated: 2026-08-17T13:16:29.278Z
template_sig: 87bbf1d0
rendered_sig: 03beafac
---

# Spec — Run the single closeout ingest wave into .kb/product/

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — DoD-15 at `:465-468` ("Persona and journey atoms exist under `.kb/product/` with valid frontmatter, pass `redkiln validate --kb`"), `## Referenced personas & journeys` (`:275`) and its "Flagged for promotion at closeout" line |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md:106-114` — the `.kb/playbooks/` page-need discipline atom staged through ingest at closeout, and why ingest is the only authoring path (`0269720`) |
| Project | `.bklg/docs-that-teach/durable-audience-closeout/project.md` — DR-3, DR-7, DR-8, DR-10; AC-004, AC-005, AC-008, AC-009, AC-011 |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md` |
| Key briefs | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` — `## Architecture brief` (AC-A01, AC-A02, AC-A03, AC-A05, AC-A08, AC-A09, AC-A10; §2 the composition root and its mount points; §4 the contracts an atom must satisfy; T1 and T4), `## UX brief` (the primitive table at `### Design-system primitives`, IQ-2's fourth bullet, IQ-4, IQ-6; AC-UX-02, AC-UX-05, AC-UX-06), `## Testing brief` (the four tiers, and the AC-### → tier mapping rows for AC-004, AC-005, AC-008, AC-009, AC-011) |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` — approved 2026-08-17, `hasSurface: false`, no public API surface and no rendered surface. Binding on this story by what it *excludes*: there is no signature, visibility or doctest obligation here to invent |
| Grounding | `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md:63-119` (`## Existing patterns this project must follow`) and `:120-134` (`## Reconciliation counterpart does not exist in this tree`) |
| Roadmap pointer | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md`, `## Merge order` §3 — this story is second in `product-layer-promotion`, after `staged-audience-payload` and before `product-layer-mounting`, and the three commit as one wave |

## One-line PR slice

Run the single closeout `/redkiln:kb-ingest` wave against an explicit file list that excludes
`.kb/_intake/README.md`, landing persona atoms as `kind: concept` / journey atoms as
`kind: playbook` — both `authority_tier: product` — plus HS-P0021's carried playbook payload,
with resolving `source_paths`, a cleared intake directory and `redkiln validate --kb` green.

## Executive summary

This PR lands the **atoms**: the one closeout ingest wave, and everything it writes.
`staged-audience-payload` (HS-S0177) wrote the persona and journey documents,
`charter-open-question-disposition` (HS-S0175) wrote zero or more deferral documents, and
HS-P0021's `playbook-atom-staged-for-ingest` wrote
`.kb/_intake/lesson-page-need-declaration-discipline.md`. All three are sitting in one
directory and none of them is an atom. This story is the single act that turns them into
atoms, and it is the only act in this project permitted to write `.kb/` outside `_intake/`.

The delta over the project charter is four decisions the briefs left to the implementer, and
this spec fixes all four:

1. **The wave is enumerated, not globbed.** `/redkiln:kb-ingest` takes `files` as an explicit
   array of repo-relative path strings. AC-A02 is discharged by *passing that array*, not by
   writing a cleverer glob and not by deleting the README afterwards. The array is the
   artefact this story is reviewed on.
2. **The array is a union of three upstream inventories, and the union is this story's to
   assemble.** Nothing else in the project holds a complete list. A staged file omitted from
   the array is not ingested, is not cleared, and is discovered only when someone notices a
   leftover file in `.kb/_intake/` — which the directory's own contract makes readable
   precisely so that it *is* noticed (`.kb/_intake/README.md:13-19`).
3. **The wave lands two tiers, deliberately.** Persona and journey atoms are
   `authority_tier: product`; HS-P0021's carried payload is `authority_tier: guideline` and
   must stay that way. "Set the tier to `product`" applied uniformly across the wave is a
   defect, not a simplification.
4. **`redkiln validate --kb` green is recorded as proving two specific things and no others** —
   frontmatter conformance and accepted-decision immutability. It cannot see `.kb/_intake/`
   and it does not enforce the `authority_tier` vocabulary. Both facts are load-bearing here
   and both are stated on the record rather than discovered by a reviewer.

Everything else this PR does is the run itself and the observation of its output. It decides
no persona content, re-argues no question, and edits no sibling's text.

## Context pack

**One wave, and the wave is the deliverable.** The project runs exactly one
`/redkiln:kb-ingest` at closeout (`_storymap.md`, `## Merge order` §3; project.md,
"Operating the single closeout ingest wave"). Every `.kb/` file this project adds must have
that run in its history — AC-A01 is literal: "no `.kb/` atom appears in the diff without a
corresponding staged source in the same wave's history." Hand-authoring an atom because the
ingest produced something slightly wrong is the exact practice reverted at `0269720` and
named as a non-goal in the charter. If the run's output is wrong, fix the staged input and
re-run; do not patch the output.

**AC-A02 is satisfied at the invocation, by an explicit array — and this is the story's
highest-consequence decision.** `/redkiln:kb-ingest`'s default input is `.kb/_intake/*.md`
(`.kb/_intake/README.md:3-7`), and `.kb/_intake/README.md` is itself a real `.md` file that
is deliberately not an atom (`:29-31`). The run is invoked with a structured `files` argument —
a non-empty array of repo-relative path strings — so the README is excluded by *never being
named*, which is a stronger guarantee than a negated glob. IQ-4 puts the fix here on purpose:
after the wave commits, a README that was ingested as an atom is a supersession problem, not
a deletion (`_decomposition.md`, `## UX brief`, IQ-4; `## Architecture brief`, T4).

**The file list is a union of three inventories, and assembling it is this story's work.**
(a) `staged-audience-payload`'s persona and journey documents, named `persona-<slug>.md` and
`journey-<slug>.md` — a prefix set chosen so it is enumerable and cannot collide with the
README (that story's AC-008 and its Integration contract). (b) `charter-open-question-disposition`'s
staged deferral documents, **zero or more**, listed in that story's handoff block, which by
its AC-006 states each staged file name explicitly so this story does not have to infer them
(`.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md`).
(c) HS-P0021's single carried payload,
`.kb/_intake/lesson-page-need-declaration-discipline.md`
(`.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md`).
That third one is the easiest to lose: its own spec was written expecting the *default* glob
to sweep it ("it needs no registration anywhere"), so narrowing the invocation — which AC-A02
requires — silently drops it unless this story names it. Enumerate it.

**Two tiers land in one wave, and the tier is the single highest-probability defect in this
surface.** `.kb/product/README.md:6-9` fixes persona → `kind: concept` + `authority_tier: product`
and journey → `kind: playbook` + `authority_tier: product`. But `.kb/playbooks/README.md:1-4`
says, in its own first sentence, that playbook atoms carry `authority_tier: guideline` — so an
ingest agent that reasons "this is a playbook, read the playbooks layer README" lands a journey
atom in `.kb/playbooks/` with the wrong tier, and every check except a reviewer passes it. That
is T1 stated concretely (`_decomposition.md`, `## Architecture brief`, T1; `## UX brief`,
"The single highest-probability defect"). **`kind` is not the directory.** A journey is
`kind: playbook` living under `.kb/product/`; HS-P0021's payload is *also* `kind: playbook`,
lives under `.kb/playbooks/`, and keeps `authority_tier: guideline`. Both are correct
simultaneously, and a uniform tier across the wave breaks one of them.

**`redkiln validate --kb` cannot catch that, and saying so is part of the deliverable.**
`authority_tier` is `z.string().min(1)` in the schema — "so `validate --kb` accepts any
non-empty string and nothing enforces the vocabulary" (`.kb/README.md:31-35`). AC-011 green is
therefore *not* evidence for AC-004. The testing brief already routes AC-004 to Tier 1 as
`rg "^(kind|authority_tier):" .kb/product` for exactly this reason
(`_decomposition.md`, `## Testing brief`, `### AC-### → tier mapping`, AC-004 row).

**AC-009 and AC-011 are two observations, not one, and this story owns both.** `validate --kb`
skips every `_`-prefixed directory (`.kb/_intake/README.md:21-27`), so a green run says nothing
about whether `.kb/_intake/` was cleared. The architecture brief calls this out directly:
AC-009 "is a separate, separately-observed criterion and not a by-product of AC-011"
(`## Architecture brief`, §2). The storymap's own implementer note repeats it. Record them as
two rows with two different pieces of evidence.

**Non-occlusion survives the clearing step through `source_paths`, and that makes the citation
mandatory rather than nice.** A successful ingest deletes the staged sources in the same commit
that adds the atoms (`.kb/_intake/README.md:13-19`). The evidence is preserved because every
atom cites **both** its `.kb/_intake/…` staging path and the `.bklg/docs-that-teach/_discovery/…`
artefact behind the claim — the pattern the corpus already uses at
`.kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28`, and the shape the two
previously-ingested playbooks carry (`.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:21`,
`.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md:24`). IQ-2's fourth bullet and
AC-UX-05 both bind here. An atom whose `source_paths` lost the `_intake` entry has had its
provenance deleted by the very step that was supposed to be lossless, and IQ-6 is the other
half: the discovery distillation is cited, never orphaned, so a charter that cited the
distillation and a charter that cites the atom point at one audience rather than two.

**The maps move as a side effect of the run, and append-only binds to that side effect too.**
The ingest's map-sync step writes `.kb/maps/domain-map.md` and, where a deferral atom landed,
`.kb/maps/open-questions-index.md` (`_decomposition.md`, `## Architecture brief`, §1 writer
table). This story does not *author* the product `##` section — `product-layer-mounting` owns
AC-010 and the two-hop reachability walk — but whatever the wave writes must already obey the
maps' own rule: append a `##`, never edit an existing one (`.kb/maps/domain-map.md:144-150`;
AC-A05, AC-UX-06, IQ-3). A deletion line in either map file is a defect this story introduced
and the next story cannot repair, because the anchor someone cited has already moved.

**`.kb/decisions/` is untouchable, and the validator is the tripwire.** `validate --kb` checks
each accepted decision atom against `HEAD` (CLAUDE.md, "Where the work lives"), so an
accidental touch fails the run structurally — T5, and the testing brief notes this makes AC-A09
free rather than needing its own check. Anything the wave surfaces that wants a decision is a
routed gap in prose in the reconciliation record, never an ADR written as closeout exhaust
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).

**No ADR governs this surface, and looking for one is the error.** All sixteen accepted
decision atoms govern the Rust contract; the charter says outright that "none of the seventeen
decisions concerns documentation" (`.bklg/docs-that-teach/initiative.md`,
`## Open questions for the planning team`, second bullet). What binds instead is the
single-write-path rule, `.kb/governance/rewrite-the-referent-never-the-reasoning.md`, and
`harvest_kb: true` on the `closeout` stage only (`.redkiln/processes/project.yaml:76-80`).

**Vehicle versus content.** HS-P0021's payload rides this wave; its *text* is HS-P0021's
(AC-A10, and the initiative decomposition at `:105-114`). This story may cause the atom to be
created and must not rewrite a sentence of it. The same discipline applies to the deferral
documents: their shape is `charter-open-question-disposition`'s, taken from
`.kb/open-questions/README.md`.

**The persona-journey slice this realizes.** U1, the next initiative's charter author, is the
reader whose acceptance this story is finally answerable to — DoD-15's bar is that the atoms
*exist under `.kb/product/` with valid frontmatter and pass `validate --kb`*, which is the
shape half of "the next initiative inherits an audience rather than re-deriving one". U2, the
closeout reviewer, stands at the ingest approval gate: they see the file array before the run
and the one commit after it, and IQ-4 makes reviewing-by-merging the whole review mechanism.
The three documented personas are the *subject* of this material and never its audience
(`_decomposition.md`, `## UX brief`, "Who this is actually for").

## Integration contract

- **Archetype**: `capability` — a user-observable slice, where the user is U1/U2 and the
  observable is the promoted layer as it exists after the run.
- **Slice / milestone**: `product-layer-promotion`. Slice-mates, implemented in the same
  context and mounted as one integrated surface, in this order:
  `staged-audience-payload` → **`audience-ingest-wave`** → `product-layer-mounting`
  (`_storymap.md`, `## Merge order` §3). The whole wave — new atoms *and* the removal of the
  staged sources — lands as **one commit on its own branch** and is reviewed by merging
  (`.kb/_intake/README.md:13-19`; IQ-4). An ingested atom that is not mounted is "a component
  rendered into no tree" (`_decomposition.md`, `## Architecture brief`, §2), so this story is
  never delivered alone.
- **Mount point**: **`.kb/product/`** — the layer written only by `/redkiln:kb-ingest`, reached
  only from `.kb/_intake/`, and governed by `.kb/product/README.md:6-13`. Staging is the input;
  **ingest is the mount** (`## Architecture brief`, §2). This story *is* the mounting act for
  the persona and journey material: before it, the payload is raw text invisible to
  `validate --kb`; after it, the atoms exist in the layer the next initiative's charter cites.
  The onward render path — `.kb/maps/domain-map.md`, the reciprocal `related` edges, `links.kb`
  on HS-P0025 and the `## Knowledge Harvest` row — is `product-layer-mounting`'s to complete,
  and this story hands it the landed atom ids so it does not re-derive them.
- **Wires into**:
  - `/redkiln:kb-ingest` — the composition root and the single writer of `.kb/product/`
    (`.kb/_intake/README.md:3-5`; CLAUDE.md, "Where the work lives"). Invoked with a structured
    argument whose `files` field is a non-empty array of repo-relative path strings; that array
    is this story's principal artefact.
  - `.kb/_intake/` and its three staged inventories: `staged-audience-payload`'s
    `persona-*.md` / `journey-*.md`,
    `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md`'s
    handoff block, and
    `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md`'s
    `.kb/_intake/lesson-page-need-declaration-discipline.md`.
  - `.kb/product/README.md:6-13` — the tier table and the four-slot persona vocabulary the
    landed atoms are checked against. Read the layer README, never the neighbour (T1).
  - `.kb/README.md:16-26` (the required-field table) and `:31-35` (why `authority_tier` is a
    convention the validator does not enforce); `.kb/_templates/atom.md` (the frontmatter
    vocabulary and the `## Context` / `## Body` / `## Consequences / links` skeleton).
  - `.kb/playbooks/README.md:1-4` — read here as the **tier trap**, not as the model for a
    journey atom: it fixes `authority_tier: guideline` for its own layer, and it is where
    HS-P0021's carried payload correctly lands.
  - `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28` — the `source_paths`
    pattern citing both the `.kb/_intake/…` staging file and the long-form evidence.
  - `.kb/maps/domain-map.md:144-150` and `.kb/maps/open-questions-index.md:136-143` — what the
    wave's map-sync step is allowed to do to them (append), and what it is not (edit).
  - `redkiln validate --kb` — the only automated reader of the result, and the accepted-decision
    immutability tripwire that makes AC-A09 free (`## Testing brief`, Notes).
- **Renders surfaces**: **none from `_design.md`.** This project's signed-off design records
  `hasSurface: false` and carries no `## Items` block — "no items — no public API surface, no
  rendered UI surface". The binding surface contract is the UX brief's primitive layer: the
  atom template, the frontmatter vocabulary and the tier tokens
  (`_decomposition.md`, `## UX brief`, `### Design-system primitives`), with AC-UX-02,
  AC-UX-05 and AC-UX-06 as this story's rows.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story adds no Rust,
  touches no port and changes no file under `crates/`, so no rule in `suite.rs` can observe it.
  Its automated readers are `redkiln validate --kb`, `git diff`, `git log`, `rg` and `test -f`;
  its content-shaped failures are caught only by Tier 2
  (`_decomposition.md`, `## Testing brief`, the tier table).
- **Clause(s)**: **none.** No `spec/SPECIFICATION.md` clause is discharged or amended, and none
  is touched — `spec/` is read-only for this whole project (`## Architecture brief`, §1). No
  `[FROZEN]` clause is in play, so no ADR is owed.
- **Advances DoD scenario**: **DoD-15 — "The audience is durable and reconciled"**
  (`.bklg/docs-that-teach/initiative.md:465-468`). This story turns green the two halves DoD-15
  states literally — the atoms *exist under `.kb/product/` with valid frontmatter* and they
  *pass `redkiln validate --kb`* — and leaves the third, *linked from this initiative's
  closeout*, to `product-layer-mounting`. It is also the step at which project AC-008's "no
  atom was hand-authored" becomes a fact about the tree rather than an intention.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file
changed outside it. The boundary is wider than a hand-written story's because the composition
root writes it: a single `/redkiln:kb-ingest` run adds the atoms, deletes the staged sources
and syncs the maps **in one commit**, and a boundary that excluded any of those would fail on a
file this story did not choose to touch.

```
.kb/product/**
.kb/playbooks/**
.kb/open-questions/**
.kb/concepts/**
.kb/maps/domain-map.md
.kb/maps/open-questions-index.md
.kb/_intake/**
.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/**
```

**In this PR**

- The explicit `files` array handed to `/redkiln:kb-ingest`, recorded verbatim in
  `_ledger.md` — the artefact AC-A02 is actually reviewed on.
- One `/redkiln:kb-ingest` run and everything it writes: the persona atoms
  (`kind: concept`, `authority_tier: product`), the journey atoms (`kind: playbook`,
  `authority_tier: product`), HS-P0021's carried playbook atom (`kind: playbook`,
  `authority_tier: guideline`), any `open_question` atoms from the deferral documents, the
  reciprocal-link and map-sync edits the run performs, and the removal of every staged source
  it consumed.
- The recorded observations: `redkiln validate --kb` exiting zero; the post-run state of
  `.kb/_intake/`; `git diff -- .kb/_intake/README.md` empty; the tier/kind sweep over
  `.kb/product/`; `test -f` over every path in every landed atom's `source_paths`.
- The list of landed atom ids, written into `_ledger.md` as the handoff
  `product-layer-mounting` consumes for `redkiln record-links HS-P0025 --atom`, the domain-map
  entries and the `## Knowledge Harvest` rows.
- This story's `_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) and its
  stage artefacts, plus a work commit recorded via `redkiln record-links <id> --sha`
  (`require_commit_provenance: true`, `.redkiln/config.yaml:73`).

**Explicitly not in this PR**

- **Authoring or editing any staged document.** The three inventories are upstream. A staged
  document that is wrong is fixed by its owning story and the wave re-run; editing it here
  makes this story the author of a persona (project non-goal: "Inventing personas") or of a
  sibling's discipline text (AC-A10).
- **Hand-writing, hand-patching or hand-deleting a `.kb/` atom.** Every `.kb/` file in this
  diff is the run's output. Correcting a landed atom by editing it is the reverted practice
  (`0269720`) and, for an accepted decision, is forbidden outright — supersede, never edit
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Anything under `.kb/decisions/`.** No addition, no modification (AC-A09). A reconciliation
  finding that wants a decision is a routed gap in prose in the reconciliation record.
- **`.kb/_intake/README.md`.** Not named in the `files` array, not edited, not deleted, not
  cited by any atom's `source_paths`. It must be byte-identical after the wave (project AC-009).
- **Authoring the domain map's product `##` section, the reciprocal edges as a deliberate act,
  `links.kb`, the `## Knowledge Harvest` row, or the two-hop reachability walk.** All five are
  `product-layer-mounting`'s (project AC-010, AC-011 second half; AC-A04, AC-UX-08). This story
  is responsible only for the map edits the run performs being *appends*.
- **Re-proving AC-011 after the mounting edits.** The storymap's ownership seam is explicit:
  this story proves `validate --kb` green **on the atoms as landed**; the next story re-proves
  it after the reciprocal edges and the map section change the frontmatter the validator reads
  (`_storymap.md`, `## Coverage`, AC-011 row).
- **`cargo xtask ci`.** The terminal `e2e` grain is `terminal-gate-run`'s
  (`.redkiln/config.yaml:60`); a run taken here would prove the gate, not the deliverable.
- **Anything under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `examples/`.**
  Read-only for this project; a needed change there is a routed gap
  (`.redkiln/config.yaml:5`), not a scope extension.

**Merge DoD one-liner** — merged when one `/redkiln:kb-ingest` run, invoked against an
enumerated `files` array that names every staged document from all three upstream inventories
and does not name `.kb/_intake/README.md`, has landed persona atoms as
`kind: concept`/`authority_tier: product` and journey atoms as
`kind: playbook`/`authority_tier: product` under `.kb/product/` while leaving HS-P0021's
payload at `kind: playbook`/`authority_tier: guideline`; every landed atom's `source_paths`
cites its `.kb/_intake/…` path and at least one `.bklg/docs-that-teach/_discovery/…` artefact
and every cited path resolves; `.kb/_intake/` holds only its untouched README; both map files
show no deletion line and `.kb/decisions/` shows no line at all; `redkiln validate --kb` exits
zero; and `cargo xtask affected --base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| Exactly one ingest run | The project operates a single closeout wave. Every `.kb/` file in the project's diff traces to it. A second run is an error condition, not a retry strategy, and if one is unavoidable it carries a distinct wave id so it cannot overwrite the first run's audit trail. | `.bklg/docs-that-teach/durable-audience-closeout/project.md`, "Operating the single closeout ingest wave"; `_decomposition.md`, `## Architecture brief`, AC-A01; `_storymap.md`, `## Merge order` §3 |
| The invocation carries an explicit `files` array | `/redkiln:kb-ingest` is driven with a structured argument whose `files` field is a non-empty array of repo-relative path strings. The default `.kb/_intake/*.md` is not used unmodified. Exclusion of the README is by omission from the array, which cannot be defeated by a glob subtlety. | `.kb/_intake/README.md:3-7`, `:29-31`; `_decomposition.md`, `## Architecture brief`, AC-A02 and T4 |
| The array is the union of three inventories | (a) `persona-*.md` and `journey-*.md` from `staged-audience-payload`; (b) the zero-or-more deferral documents named in `charter-open-question-disposition`'s handoff block; (c) `.kb/_intake/lesson-page-need-declaration-discipline.md`. (c) is the one a narrowed invocation silently drops, because its own spec was written expecting the default glob. | `.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md` (AC-008); `.../charter-open-question-disposition/spec.md` (AC-006, handoff block); `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md` |
| Persona atoms: `kind: concept`, `authority_tier: product` | Under `.kb/product/`, flat — the corpus is flat per layer and no subdirectory is invented. Each atom's body still answers goal, context, what they already do instead, what they are afraid of; the wave must not compress a slot away. | `.kb/product/README.md:6-13`; `_decomposition.md`, `## Architecture brief`, AC-A03; `## UX brief`, AC-UX-01, AC-UX-02 |
| Journey atoms: `kind: playbook`, `authority_tier: product` | Also under `.kb/product/`. `kind` is not the directory: a journey is a `playbook`-kind atom in the product layer, and routing it to `.kb/playbooks/` or giving it `guideline` is the named wrong implementation. | `.kb/product/README.md:6-9`; `.kb/playbooks/README.md:1-4` (the tier that must **not** be copied); `_decomposition.md`, `## UX brief`, "The single highest-probability defect" |
| The carried payload keeps its own tier and text | HS-P0021's atom lands `kind: playbook`, `authority_tier: guideline` in `.kb/playbooks/`, with its body unedited. Vehicle versus content: this project runs the ingest and does not own the discipline's text. | `.bklg/docs-that-teach/_decomposition.md:106-114`; `_decomposition.md`, `## Architecture brief`, AC-A10; `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md` |
| Deferral documents land as `open_question` atoms | Under `.kb/open-questions/`, in the layer's own "what is true today / what is not decided / what forces it" shape, one per staged document. Zero is a valid count if every charter question was answered on the record. | `.kb/open-questions/README.md`; `.../charter-open-question-disposition/spec.md`, AC-005; project DR-13 |
| Every landed atom carries the full required frontmatter | `id`, `title`, `kind`, `status`, `authority_tier`, `summary` (a folded block carrying the argument, not a label), `depends_on`, `related`, `source_paths`, `last_reviewed`. No invented tracking key; `KbFrontmatter` is `.passthrough()`, so an imported key survives and stripping one to force conformance is the wrong repair. | `.kb/README.md:16-26`; `.kb/_templates/atom.md:1-18`; `_decomposition.md`, `## Architecture brief`, §4 |
| `source_paths` cites both the staging path and the discovery evidence | Each atom names its `.kb/_intake/…` source **and** at least one artefact under `.bklg/docs-that-teach/_discovery/`. This is what makes the clearing step lossless and what keeps a prior charter citation and a new atom citation pointed at one audience. | `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28`; `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md:21`; `_decomposition.md`, `## UX brief`, IQ-2 fourth bullet, IQ-6, AC-UX-05 |
| Every cited path resolves on the merged tree | `test -f` over every `source_paths` entry of every landed atom. The verified discovery set is enumerated in the architecture brief; a path that exists only on the sibling branch is not citable here. | `_decomposition.md`, `## Architecture brief`, §4 (`source_paths`, verified set); project AC-005 |
| The intake directory is cleared of this wave's payload | After the run, `.kb/_intake/` contains `README.md` and nothing this wave consumed. A file still sitting there is a file the run did not ingest — that is the directory's contract and it is what makes the leftover readable. | `.kb/_intake/README.md:13-19`; project AC-009, DR-8 |
| `.kb/_intake/README.md` is byte-identical | Not named, not ingested, not cited. `git diff -- .kb/_intake/README.md` is empty, and no landed atom's `source_paths` contains it. | `.kb/_intake/README.md:29-31`; `_decomposition.md`, `## Architecture brief`, AC-A02 |
| Project AC-009 and project AC-011 are observed separately | `validate --kb` skips `_`-prefixed directories, so its exit code is silent about the intake directory. Two rows, two commands, two pieces of evidence. | `.kb/_intake/README.md:21-27`; `_decomposition.md`, `## Architecture brief`, §2; `_storymap.md`, "Notes the implementer needs at the story grain" |
| `validate --kb` green is scoped on the record | It proves frontmatter conformance and accepted-decision immutability against `HEAD`. It does **not** prove the tier vocabulary (`authority_tier` is `z.string().min(1)`) and does not prove the intake directory was cleared. Both limits are written into the ledger beside the green result. | `.kb/README.md:31-35`; CLAUDE.md, "Where the work lives"; `_decomposition.md`, `## Testing brief`, tier mapping AC-004 and AC-009 rows |
| Map edits are appends | Whatever the run's map-sync writes to `.kb/maps/domain-map.md` or `.kb/maps/open-questions-index.md` is an appended `##` section or an appended bullet; `git diff` over both shows no deletion line. Authoring the product section deliberately is `product-layer-mounting`'s. | `.kb/maps/domain-map.md:144-150`; `.kb/maps/open-questions-index.md:136-143`; `_decomposition.md`, `## Architecture brief`, AC-A05; `## UX brief`, IQ-3, AC-UX-06 |
| `.kb/decisions/` is untouched | No addition, no modification. The validator's immutability check against `HEAD` fails structurally on any touch, which is the desired behaviour and is not to be worked around. | `_decomposition.md`, `## Architecture brief`, AC-A09 and T5; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| No `.kb/` file is hand-authored | Every `.kb/` path in the diff is introduced by the wave's commit. `git log`/`git diff` for each added atom shows the ingest wave as its introducing change, with the corresponding staged source removed in the same commit. | `_decomposition.md`, `## Architecture brief`, AC-A01; `## Testing brief`, tier mapping AC-008 row; precedent `0269720` |
| The wave is one commit, reviewed by merging | New atoms, removed staged sources and map syncs commit together on the wave's own branch. That is what makes IQ-4's reversibility real: the reviewer's unit is the merge, not a file. | `.kb/_intake/README.md:13-19`; `_decomposition.md`, `## UX brief`, IQ-4 |
| The landed atom ids are handed forward | The ids are recorded as a list in `_ledger.md` so `product-layer-mounting` can run `redkiln record-links HS-P0025 --atom`, write the domain-map entries and fill the `## Knowledge Harvest` rows without re-deriving them by listing the directory. | `_decomposition.md`, `## Architecture brief`, §2, mount points 3–5; `.redkiln/templates/closure.md:14-20`; `.redkiln/templates/_retrospective.md:56-59` |
| Every state this story records is a literal word | No emoji, tick, colour word, strikethrough, ordering or empty cell carries meaning in the ledger or anywhere else in the diff. | `_decomposition.md`, `## UX brief`, `### Accessibility floor`, AC-UX-09; `.kb/maps/open-questions-index.md:136-143` |

## Data and migrations

**N/A as a schema migration — but this story is the only one in the project that performs a
real data transition, and it is worth stating as one.** No Rust type, no store, no serialized
format and no persisted state is introduced. What moves is a corpus: a set of unschema'd
markdown documents under `.kb/_intake/` becomes a set of `KbFrontmatter`-conformant atoms under
`.kb/product/`, `.kb/playbooks/` and `.kb/open-questions/`, and the sources are deleted in the
same commit.

Three properties make that transition safe, and all three are this story's to preserve:

1. **It is atomic.** The additions and the deletions land in one commit on one branch
   (`.kb/_intake/README.md:13-19`). There is no intermediate tree in which the atoms exist and
   the staged sources also exist, and none in which the sources are gone and the atoms are not.
2. **It is lossless.** Nothing is destroyed that is not recoverable, because the staged
   document stays in git history and every atom cites its `.kb/_intake/…` path in
   `source_paths` (`:17-19`). That citation is therefore not decoration — it is the only
   forward link from the atom to the material it was cut from, and dropping it is the one way
   this transition can actually lose data.
3. **It is forward-only.** Correction is by a new atom that supersedes, never by editing
   (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`), and for an accepted decision
   atom the validator enforces it against `HEAD`. There is no down-migration; the reversal
   available to a reviewer is declining the merge, which is why IQ-4 puts the review at the
   merge and the README filter at the invocation.

The nearest thing to a schema in play is `KbFrontmatter`, and it is minted here rather than
upstream: staged material is explicitly not held to it (`.kb/_intake/README.md:7-11`), so the
first tree in which these documents have ids, kinds and tiers is the tree this story produces.
`redkiln validate --kb` is the migration's only automated post-condition — with the two
documented blind spots named in the Context pack, which is why it is one of several checks
here and not the check.

## Acceptance criteria

Nine criteria. Each is framed from the intent of a reader the UX brief names — **U1**, the
next initiative's charter author; **U2**, the closeout reviewer standing at the ingest
approval gate and at the pull request; **U3**, the future maintainer reconciling a third
persona set (`_decomposition.md`, `## UX brief`, `### Who this is actually for, framed as
intent`). "Test" in this corpus means one of the four tiers the testing brief fixes
(`_decomposition.md`, `## Testing brief`, `### The test mix`), and the tier named in each row
is the one that can actually falsify a wrong implementation of that row.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U2 at the ingest approval gate, whose only cheap veto is refusing the run before it commits, **WHEN** they read the `/redkiln:kb-ingest` invocation this story is about to make, **THEN** they see a structured argument whose `files` field is a non-empty array of repo-relative path strings that names every document from all three upstream inventories — `staged-audience-payload`'s `persona-*.md` and `journey-*.md`, every deferral document listed in `charter-open-question-disposition`'s handoff block (zero is a valid count, and zero must be stated as such rather than left ambiguous), and `.kb/_intake/lesson-page-need-declaration-discipline.md` — and does **not** name `.kb/_intake/README.md`, so the README is excluded by never being named rather than by a glob that could be defeated. | **Tier 3 primary** — the invocation is read before the run, against the three inventories: `.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md` (AC-008), `.../charter-open-question-disposition/spec.md` (AC-006 handoff block), `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md`. **Tier 1 backstop after the run:** `ls .kb/_intake/` shows the array was complete — nothing consumed is left behind, and nothing left behind was unlisted. The array is recorded verbatim in `_ledger.md` as AC-001's evidence (`_decomposition.md`, `## Architecture brief`, AC-A02 and T4). |
| AC-002 | **GIVEN** U1 writing `GIVEN a <persona> <context>, WHEN they <action>, THEN <outcome>` for the next charter and unwilling to re-run discovery, **WHEN** they open one landed persona atom and no other file, **THEN** it is under `.kb/product/` (flat — no subdirectory), carries `kind: concept` and `authority_tier: product`, composes from `.kb/_templates/atom.md`'s own skeleton — one `#`, then `## Context` / `## Body` / `## Consequences / links`, with the ten required frontmatter keys present and `summary:` a folded block carrying the argument rather than a label — and yields all four slots the product README fixes (goal, context, what they already do instead, what they are afraid of) plus its observation-status word, each stated in place, with every link carrying evidence or provenance and never one of the four slots. | **Tier 1:** `rg -n "^(kind\|authority_tier):" .kb/product/` shows `kind: concept` + `authority_tier: product` on every persona atom; `rg -n "^(id\|title\|kind\|status\|authority_tier\|summary\|depends_on\|related\|source_paths\|last_reviewed):" ` over each atom shows all ten keys; `rg -n "^## " ` shows the three-section skeleton and `rg -n "^# " ` shows exactly one; `ls .kb/product/` shows no subdirectory. **Tier 2 primary** (the only tier that can fail slot *content*): read each atom alone against `.kb/product/README.md:6-13` and `_decomposition.md`, `## UX brief`, AC-UX-01 — a slot reading "see `_discovery/distillation/personas-and-journeys.md`" is the named wrong implementation (`## Testing brief`, "A named wrong implementation Tier 2 exists to reject"). |
| AC-003 | **GIVEN** U1 following a journey atom to see the moment-by-moment path, and U3 later asking what standing each atom carries, **WHEN** they list the landed set, **THEN** journey atoms sit under `.kb/product/` with `kind: playbook` **and** `authority_tier: product` — `kind` is not the directory — while HS-P0021's carried payload sits under `.kb/playbooks/` with `kind: playbook` and `authority_tier: guideline`, its body unedited by this story, so the wave lands two tiers deliberately and a uniform tier across the wave is visible as the defect it is. | **Tier 1:** `rg -n "^(kind\|authority_tier):" .kb/product/` shows **no** `guideline` under the product layer; `rg -n "^authority_tier:" .kb/playbooks/lesson-page-need-declaration-discipline.md` shows `guideline`; `ls .kb/playbooks/` shows no journey atom misfiled there. **Tier 2:** `git diff` on the carried payload's staged source versus the landed atom body shows the ingest's structural transformation only, no editorial change (`_decomposition.md`, `## Architecture brief`, T1 and AC-A10; `.kb/playbooks/README.md:1-4` read as the tier trap, never as the model). |
| AC-004 | **GIVEN** U3 reconciling a third persona set two initiatives from now, needing to correct a claim without editing it and therefore needing to reach the material it was cut from, **WHEN** they open any atom this wave landed — after the same commit deleted every staged source — **THEN** its `source_paths` cites **both** its `.kb/_intake/…` staging path and at least one artefact under `.bklg/docs-that-teach/_discovery/`, and every cited path resolves in the merged tree, so the clearing step destroyed nothing and a charter that cited the distillation and a charter that cites the atom point at one audience rather than two. | **Tier 1:** for every landed atom, `rg -n "^source_paths:" -A 6` yields the entry list, and `test -f` over every entry exits zero — a path that exists only on a sibling branch fails here. At least one entry per atom matches `^\s*-\s+\.kb/_intake/` and at least one matches `^\s*-\s+\.bklg/docs-that-teach/_discovery/`. Pattern authority: `.kb/concepts/torn-reads-and-the-append-condition-boundary.md:23-28`. Serves project AC-005; AC-UX-05, IQ-2's fourth bullet, IQ-6. |
| AC-005 | **GIVEN** U2 asking the one question this project's non-goal exists to make answerable — "was any of this written by hand?" — **WHEN** they walk the history of every `.kb/` path added in this project's diff, **THEN** each one's introducing commit is this wave's single commit, that same commit removes the staged source it consumed, and no `.kb/` file appears whose staged source is absent from the wave's history; a landed atom found wrong is corrected by fixing the staged input and re-running, never by editing the output. | **Tier 1:** `git log --diff-filter=A --format=%H -- <each added .kb path>` returns the wave commit for every path; `git show --stat <wave sha>` shows the additions and the `.kb/_intake/` deletions in the same commit. **Tier 3:** no `.kb/` path in `git diff --name-only main...HEAD` lies outside that commit's file list. Serves project AC-008; `_decomposition.md`, `## Architecture brief`, AC-A01; precedent `0269720`. |
| AC-006 | **GIVEN** U2 relying on the intake directory's own contract — that a file still sitting there is a file the run did not ingest — **WHEN** they list `.kb/_intake/` after the wave, **THEN** it contains `README.md` and nothing this wave consumed, `git diff -- .kb/_intake/README.md` is empty, no landed atom's `source_paths` names the README, and this observation is recorded separately from AC-007 because `redkiln validate --kb` skips every `_`-prefixed directory and its exit code is silent about this one. | **Tier 1 and only Tier 1** — `ls .kb/_intake/` and `git diff -- .kb/_intake/README.md`; plus `rg -n "_intake/README" .kb/product/ .kb/playbooks/ .kb/open-questions/` returning nothing. `validate --kb` must **not** be substituted (`.kb/_intake/README.md:21-27`; `_decomposition.md`, `## Testing brief`, tier mapping, AC-009 row: "validate --kb cannot see this tier — do not substitute it"). Serves project AC-009, DR-8. |
| AC-007 | **GIVEN** U1 needing the frontmatter contract to hold before they cite an atom, and U2 needing to know exactly what a green run did and did not prove, **WHEN** `redkiln validate --kb` is run on the tree carrying the landed atoms, **THEN** it exits zero, and the ledger records beside that result the two limits it does not cover — it does not enforce the `authority_tier` vocabulary (`authority_tier` is `z.string().min(1)`), so AC-007 green is not evidence for AC-002 or AC-003; and it cannot see `.kb/_intake/`, so it is not evidence for AC-006. | **Tier 1:** `redkiln validate --kb` exit code zero, captured with its output; paired with `redkiln doctor` still reporting exactly the six permanent `template-drift` advisories and no seventh (CLAUDE.md, "Where the work lives"). The two recorded limits are checked by reading `.kb/README.md:31-35` and `.kb/_intake/README.md:21-27` into the ledger's note. Serves project AC-011 **on the atoms as landed only** — `product-layer-mounting` re-proves it after the reciprocal edges (`_storymap.md`, `## Coverage`, AC-011 row). |
| AC-008 | **GIVEN** U3, and anyone who has already cited a `file:line` in either map, needing the anchor they cited to still mean what it meant, **WHEN** they diff the two map files and the decisions layer after the wave, **THEN** whatever the run's map-sync wrote to `.kb/maps/domain-map.md` or `.kb/maps/open-questions-index.md` is an appended `##` section or appended bullets in each map's own fixed shape — domain-map entries grouped by kind under bold labels with each atom's id beside its link; index bullets with the status word first, then the link, then the id, then one sentence — with **no deletion line** anywhere in either diff, and `.kb/decisions/` shows no line at all, added or modified. | **Tier 1:** `git diff -- .kb/maps/domain-map.md .kb/maps/open-questions-index.md \| rg "^-[^-]"` returns nothing; `git diff --name-only -- .kb/decisions/` returns nothing. **Tier 2:** the appended content read against `.kb/maps/domain-map.md:144-150` and `.kb/maps/open-questions-index.md:136-143` for shape. The decisions half is additionally free: `validate --kb` checks each accepted decision atom against `HEAD` and fails structurally on a touch (`_decomposition.md`, `## Architecture brief`, AC-A05, AC-A09, T5; `## UX brief`, IQ-3, AC-UX-06). |
| AC-009 | **GIVEN** the implementer of `product-layer-mounting`, who must run `redkiln record-links HS-P0025 --atom`, write the domain-map entries and fill the `## Knowledge Harvest` rows without re-deriving the landed set by listing a directory, **WHEN** they open this story's `_ledger.md`, **THEN** they find the complete handoff — for every atom the wave landed, its id, its path, its `kind` and its `authority_tier` — with every one of those states written as a literal word, no emoji, tick, colour word, strikethrough, empty cell or ordering carrying meaning anywhere in this story's diff, and every link naming its destination rather than "here" or "see above". | **Tier 2 primary:** read `_ledger.md`'s handoff list against `ls .kb/product/ .kb/playbooks/ .kb/open-questions/` — a landed atom absent from the list is an unmounted atom the next story will miss. **Tier 1 backstop:** `rg -n "[\x{2705}\x{274C}\x{1F7E2}]|~~" .bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/ .kb/product/` returns nothing. Serves AC-UX-09, IQ-8, and the mount points the architecture brief numbers 3–5 (`_decomposition.md`, `## Architecture brief`, §2; `.redkiln/templates/closure.md:14-20`). |

**Coverage of the traced project criteria.** AC-004 → AC-002 (shape and slots) and AC-003
(the two tiers, including the one that must stay `guideline`). AC-005 → AC-004. AC-008 →
AC-005. AC-009 → AC-006. AC-011 → AC-007, scoped to the atoms as landed. AC-001, AC-008 and
AC-009 carry no project AC alone: AC-001 is the invocation that makes AC-006 and AC-005
satisfiable, AC-008 is the append-only obligation `product-layer-mounting` inherits and
cannot repair retroactively, and AC-009 is the handoff without which the slice's third story
re-derives what this one already knows. All three are brief-grain criteria
(AC-A02, AC-A05, AC-A09, AC-UX-09) discharged inside the story carrying the project AC they
serve, exactly as `_storymap.md`'s `## Coverage` note requires.

## Interaction quality

This project's signed-off design records `hasSurface: false` with no `## Items` block
(`.bklg/docs-that-teach/durable-audience-closeout/_design.md:10-47`), so there is no rendered
screen and no public API signature for the usual composition vocabulary to attach to. It does
**not** follow that composition is vacated: the UX brief fixes an explicit primitive layer for
this corpus — the atom template, the frontmatter vocabulary, the tier tokens and the two map
formats — and states that anything invented beside them "is the equivalent of bespoke CSS in a
repository that has a design system" (`_decomposition.md`, `## UX brief`,
`### Design-system primitives`). Those primitives are what the composition invariants below are
written against.

Every invariant that applies is carried by a row in the acceptance-criteria table above. This
section says **which** row carries it and how it is falsified; it deliberately introduces no
new obligation as a bullet, because a bullet here gets no ledger row and is never gated.

**State invariants**

| Invariant | Carried by | Falsifier |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) | AC-002 | A persona atom whose "what they are afraid of" or "what they already do instead" reads "see `_discovery/distillation/personas-and-journeys.md`". U1's entire reason for existing is not having to open the discovery corpus. |
| **Non-occlusion — the filter must not hide what it filters** (IQ-2, fourth bullet) | AC-004, AC-006 | A landed atom whose `source_paths` lost its `.kb/_intake/…` entry: the clearing step then deleted the only forward link to the material the atom was cut from. Equally: a staged file consumed but not listed in AC-001's array, which leaves a file in `.kb/_intake/` that the directory's contract says was *not* ingested — readable precisely so it is noticed. |
| **Preserved position — the text analogue of preserved focus and scroll** (IQ-3) | AC-008 | `git diff .kb/maps/domain-map.md` showing a `-` line anywhere outside the appended region, or any line at all under `.kb/decisions/`. A moved anchor breaks a `file:line` someone already cited, and the next story cannot repair it. |
| **Preserved selection — prior citations keep meaning what they meant** (IQ-6) | AC-004 | An atom that cites only its staging path, forking the audience into "the distillation's personas" and "the atoms' personas" instead of pointing both at one. |
| **Reversibility** (IQ-4) | AC-001, AC-005 | `.kb/_intake/README.md` appearing anywhere in the diff or in a `source_paths`. Before the commit this is a filter; after it, it is a supersession problem — which is why AC-001 puts the fix at the invocation and AC-005 forbids repairing output by hand. |
| **Reachable without prior knowledge** (IQ-5) | AC-002 (flat layer, no subdirectory), AC-009 (the handoff that lets the next story complete the two-hop walk) | An atom findable only by knowing this initiative's slug or only by `ls .kb/product/`. The recorded two-hop walk itself is `product-layer-mounting`'s (AC-UX-08); what this story owes is not making it impossible. |
| **The qualification travels with the claim** (IQ-7) | AC-002 | An atom that survives ingest with its three evidence qualifications collected into a trailing caveats block a reader scanning the four slots never reaches — the route by which a guess acquires the standing of a finding (`.kb/product/README.md:23-24, 28-31`). |
| **Every state change legible at the moment of reading** (IQ-8) | AC-009 | Any state in this diff expressed by presence, absence, ordering or omission rather than a written word. |

**Composition invariants**

| Invariant | Carried by | The real number or shape, and its falsifier |
| --- | --- | --- |
| **Presentation exists at all** — an atom is composed, not a frontmatter block with prose dumped under it | AC-002 | The template's own skeleton is mandatory: one `#`, then `## Context` / `## Body` / `## Consequences / links`, with `summary:` a folded block carrying the argument rather than a label (`.kb/_templates/atom.md`). A landed file with valid frontmatter and an uncomposed body passes every `rg` assertion and fails this. |
| **Composition and placement** | AC-002, AC-003 | Flat per layer: personas and journeys directly under `.kb/product/`, the carried payload directly under `.kb/playbooks/`, deferral atoms directly under `.kb/open-questions/`. A subdirectory under `.kb/product/` is named as hand-rolling by the UX brief. |
| **Transience — persistent, revealed, opened on demand** | AC-004, AC-006 | Staged documents are transient **by contract**: the successful ingest removes them in the same commit that adds the atoms. What persists is the atom, its `source_paths` entry naming the staging path, and git history. Nothing may be landed whose only copy the clearing step destroys. |
| **Density budget, with its numbers** | AC-002, AC-008 | Ten required frontmatter keys, no invented eleventh (`KbFrontmatter` is `.passthrough()`, so stripping an imported key to force conformance is the wrong repair); four persona slots, none compressed away; three `##` sections per atom and one `#`; the index bullet in exactly four parts — status word, link, id, one sentence; the domain-map entry as a link plus its id plus one sentence of orientation, because the atom is the source of truth and the map is not. |
| **Hierarchy** | AC-002, AC-008 | No skipped heading level anywhere; the map's grouping by kind under bold labels (**Reference**, **Concepts**, **Governance**, **Playbooks**, **Open questions**) is the map's hierarchy and is not re-invented per entry. |
| **Anti-pattern: the tier copied from the neighbour** | AC-003 | The single highest-probability defect in this surface: a journey atom given `authority_tier: guideline` because `.kb/playbooks/README.md:1-4` says so for *its* layer. Copy from `.kb/product/README.md:6-13`, never from the atom you are imitating. |
| **Anti-pattern: a bespoke vocabulary** | AC-002 | A "Persona card", a "TL;DR", a per-atom bespoke table, or a frontmatter key absent from `.kb/_templates/atom.md`. |
| **Anti-pattern: a status carried by a glyph** | AC-009 | An emoji, a tick, a colour word, a strikethrough or "a blank cell means fine" anywhere in the diff. Corpus precedent is literal words, stated first (`.kb/maps/open-questions-index.md:136-143`). |
| **Anti-pattern: an atom written by hand into `.kb/`, whatever its shape** | AC-005 | The practice reverted at `0269720`, and the one this whole project exists to avoid repeating. |
| **Anti-pattern: a new `##` in `open-questions-index.md` where the domain already has a section** | AC-008 | The exemption `charter-open-question-disposition`'s handoff relies on applies **only** because documentation has no section today; the two existing sections stay untouched. |

Accessibility floor items that transfer rather than apply: WCAG AA contrast and reduced motion
are N/A here because this diff authors no colour and no motion, and the obligation transfers
unchanged to HS-P0020's rendered surface (`_decomposition.md`, `## UX brief`,
`### Accessibility floor`). The two that bind **today** in markdown — colour/glyph/position
never alone, and link purpose from link text alone — are AC-009's.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | A file sits in `.kb/_intake/` that appears in none of the three upstream inventories. | **Halt and report blocked.** Do not infer its owner and do not sweep it in "since it is there". An unowned staged file is either a sibling story's uncommitted work or a leftover from an earlier attempt, and both are answered upstream. Ingesting it makes this story the author of content it did not write. |
| EC-002 | `charter-open-question-disposition`'s handoff block is absent, or lists a staged file name that does not exist on disk. | **Halt and report blocked** against `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md` (its AC-006 obliges the block to state each staged file name explicitly). Do not infer the deferral set by pattern-matching `.kb/_intake/`. A handoff block that legitimately lists **zero** deferral documents is not this condition — it is the valid zero case, and AC-001 requires it be recorded as stated-zero rather than left ambiguous. |
| EC-003 | The run lands a journey atom under `.kb/playbooks/`, or any `.kb/product/` atom with a tier other than `product`, or the carried payload with a tier other than `guideline`. | **Do not hand-edit the landed atom.** Fix the staged input's tier declaration and re-run the wave; if a re-run is unavoidable it carries a distinct wave id so it cannot overwrite the first run's audit trail. Editing the output is the practice reverted at `0269720` and would falsify AC-005 in the same stroke. |
| EC-004 | `redkiln validate --kb` exits non-zero. | Read the failure before changing anything. If it is a missing required key, fix the staged source and re-run. **Never** strip an unexpected key to force conformance: `KbFrontmatter` is `.passthrough()`, so an imported key is legal and its removal is data loss dressed as a fix. If the failure names an accepted decision atom, EC-006 is live instead. |
| EC-005 | `.kb/_intake/README.md` appears in `git diff`, or in any landed atom's `source_paths`. | **Stop before the commit.** Pre-commit this is a filter — narrow the array and re-run. Post-commit it is a supersession problem that cannot be cheaply undone (IQ-4), so the check belongs at the approval gate and the halt is not negotiable. |
| EC-006 | The wave's diff touches `.kb/decisions/` in any way. | **Abandon the run and investigate.** `validate --kb` fails structurally on this because it checks each accepted decision atom against `HEAD`, and that failure is the desired behaviour — it is not to be worked around, silenced, or "fixed" by restoring the file and re-running without understanding how the run reached the decisions layer. Anything the wave surfaced that wants a decision is a routed gap in prose, never an ADR written as closeout exhaust. |
| EC-007 | A `source_paths` entry does not resolve — most plausibly a discovery artefact that exists only on the sibling branch, or a staging path whose file name changed between staging and ingest. | **Fix the citation in the staged source and re-run.** Do not delete the unresolvable entry to make `test -f` pass: for the `.kb/_intake/…` entry that deletion is exactly the data loss AC-004 exists to prevent, and for a `_discovery/…` entry it silently narrows the atom's evidence base. If the artefact genuinely only exists on the sibling branch, the merge-forward baseline is the story that owes it. |
| EC-008 | The run's map-sync produces a deletion line in either map file. | **Revert the run; do not hand-repair the map.** A hand-repaired map is a `.kb/` file not produced by the wave, which fails AC-005 while appearing to fix AC-008. Establish why the sync rewrote rather than appended, fix that, and re-run under a distinct wave id. |
| EC-009 | A second ingest run becomes unavoidable after a first has committed. | The second run carries a **distinct wave id** so it cannot overwrite the first's audit trail, and the ledger records both runs with the reason the first was insufficient. "One wave" is the project's operating rule, not a claim that a mistake is unrecoverable — but the recovery is additive and named, never a silent re-run over the same id. |

## Non-functional

| id | requirement | why it binds here |
| --- | --- | --- |
| NF-001 | **Atomicity.** The additions, the staged-source deletions and the map syncs land in **one commit on one branch**. No intermediate tree exists in which the atoms and their staged sources both exist, or in which the sources are gone and the atoms are not. | It is what makes IQ-4's reversibility real: the reviewer's unit is the merge, not a file (`.kb/_intake/README.md:13-19`). |
| NF-002 | **Boundary containment.** Nothing outside the PR-boundary block changes. In particular nothing under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `examples/` — read-only for this whole project. | A needed change there is a routed gap, not a scope extension (`.redkiln/config.yaml:5`; `_decomposition.md`, `## Architecture brief`, §1). It is also what lets `cargo xtask affected --base main` stay cheap and meaningful at the story grain. |
| NF-003 | **Re-runnability of the evidence.** Every observation recorded in `_ledger.md` is a command a later reader can re-run, against a tree named by ref or sha, and get the same answer. No evidence row is a claim about what the implementer saw. | AC-UX-12's discipline applied at the story grain, and the only thing that makes U3's "correct it without editing it" possible two initiatives from now. |
| NF-004 | **No new tooling.** Every command this story runs is `/redkiln:kb-ingest`, `redkiln validate --kb`, `redkiln doctor`, `redkiln record-links`, `git`, `ls`, `rg` or `test -f`. No script is written, no harness is added, no CI job is introduced. | AC-TB-02 — every merge-gate command resolves to a real script or a documented CLI verb, no invented tooling (`_decomposition.md`, `## Testing brief`, `### Acceptance Criteria`). |
| NF-005 | **No Rust is compiled and no Rust behaviour changes.** The gate's Rust-grain steps function here purely as a regression net proving the read-only areas stayed read-only. | `_decomposition.md`, `## Testing brief`, AC-TB-08. Stating it keeps a reviewer from reading a green `cargo` step as evidence *for* this story rather than evidence that this story stayed out of the way. |
| NF-006 | **Review cost stays bounded to one sitting.** The diff is exactly the wave's output; the ledger carries the file array, the landed-atom list and the recorded observations so U2 can review by reading one commit and one ledger. | U2's stated intent is to "disagree with exactly one of them without reading the discovery corpus" (`_decomposition.md`, `## UX brief`, `### Who this is actually for`). A review that requires reconstructing the file array from the diff has already failed that. |

## Implementation notes (non-prescriptive)

A workable order, offered because the sequencing has real failure modes, not because the steps
are prescribed:

**Assemble the array before anything else, and write it down first.** The three inventories are
read, not guessed: `staged-audience-payload`'s spec names the `persona-*.md` / `journey-*.md`
prefix set, `charter-open-question-disposition`'s handoff block names each deferral file, and
HS-P0021's spec names its single payload. Cross-check the assembled array against
`ls .kb/_intake/` **in both directions** — a file on disk missing from the array is EC-001, and
a file in the array missing from disk is EC-002 or an upstream story that did not land.
Recording the array in `_ledger.md` *before* the run rather than after is what makes AC-001
reviewable as a decision rather than as a description of what happened.

**Read `.kb/product/README.md` while composing, and keep `.kb/playbooks/README.md` closed
except to check the carried payload.** This sounds like a triviality and it is the project's
single highest-probability defect. Both READMEs are correct; they say different things; and the
one an author naturally reaches for while composing a `playbook`-kind atom is the wrong one for
a journey.

**Expect the third inventory to be the one you drop.** HS-P0021's payload spec was written
expecting the default `.kb/_intake/*.md` glob to sweep it — "it needs no registration anywhere".
Narrowing the invocation, which AC-001 requires, is exactly what makes that assumption false.

**Observe AC-006 and AC-007 as two separate acts with two separate captures.** Running
`validate --kb` and then writing "and the intake directory is clear" from memory is the failure
this story was split to prevent. `ls .kb/_intake/` is its own command with its own output
pasted into its own ledger row.

**When the output is wrong, the input is what you change.** Every repair route in the error
table runs through the staged source and a re-run. There is no route in this story that ends in
an editor open on a file under `.kb/product/`.

**Hand forward more than the ids if it is cheap.** `product-layer-mounting` needs id, path,
`kind` and `authority_tier` per atom; if the wave also makes the persona↔journey pairing
obvious, record it, because the reciprocal `related` edges are that story's next task and this
run is where the pairing is freshest.

## Tests and CI (merge gate)

The four tiers are the testing brief's (`_decomposition.md`, `## Testing brief`,
`### The test mix, and what each tier can and cannot see`); the ordering is the architecture
brief's data-flow ordering 2 and 3 — ingest before validation, validation before the links.
This story runs steps 1 and 2 of the merge-gate list and the story-grain affected check; steps
3 to 5 belong to `product-layer-mounting` and `terminal-gate-run`.

| tier | command / path | proves |
| --- | --- | --- |
| **1 — static / shape** | `redkiln validate --kb` (merge-gate step 1, `_decomposition.md`, `## Testing brief`, `### Merge-gate commands`) | AC-007: frontmatter conformance across every non-`_` directory under `.kb/`, and accepted-decision immutability against `HEAD` — which is also the free assertion covering AC-008's decisions half. Proves nothing about the tier vocabulary and nothing about `.kb/_intake/`. |
| **1 — static / shape** | `redkiln doctor` (merge-gate step 2) | The six permanent `template-drift` advisories are still exactly six. A seventh or a missing one is this story's to flag rather than silence (CLAUDE.md, "Where the work lives"). |
| **1 — static / shape** | `rg -n "^(kind\|authority_tier):" .kb/product/` and `rg -n "^authority_tier:" .kb/playbooks/lesson-page-need-declaration-discipline.md` | AC-002 and AC-003 — the tier mapping the testing brief routes AC-004 to as its **primary** tier, precisely because `validate --kb` cannot see it. |
| **1 — static / shape** | `ls .kb/_intake/`; `git diff -- .kb/_intake/README.md` | AC-006. The one check that must never be substituted by `validate --kb` (`_decomposition.md`, `## Testing brief`, tier mapping, AC-009 row). |
| **1 — static / shape** | `test -f` over every `source_paths` entry of every landed atom | AC-004 — project AC-005's literal bar, and the tier mapping's primary for it. |
| **1 — static / shape** | `git diff -- .kb/maps/domain-map.md .kb/maps/open-questions-index.md \| rg "^-[^-]"`; `git diff --name-only -- .kb/decisions/` | AC-008 — append-only on both maps, nothing at all under decisions. Both must return empty. |
| **1 — static / shape** | `git log --diff-filter=A -- <each added .kb path>`; `git show --stat <wave sha>` | AC-005 — every `.kb/` path in the diff was introduced by the wave, with its staged source removed in the same commit. |
| **2 — content review** | A human read of each landed persona atom **alone**, against `.kb/product/README.md:6-13` and `_decomposition.md`, `## UX brief`, AC-UX-01 / AC-UX-04 | AC-002's slot content and AC-009's handoff completeness — the only tier that can fail a frontmatter-valid atom whose fourth slot is a link. Uses the UX brief's own checklist, not a second one (AC-TB-06). |
| **2 — content review** | A diff read of the carried payload's staged source against its landed atom body | AC-003's second half — the ingest transformed it and this story did not edit it (AC-A10). |
| **3 — integration / mount-point** | A read of the `/redkiln:kb-ingest` invocation's `files` array against the three upstream inventories, performed **before** the run | AC-001. The seam the testing brief names explicitly: "the seam to isolate is the *invocation*, not a post-hoc filter" (`### Fixtures and seams`, "The ingest glob"). |
| **3 — integration / mount-point** | `cargo xtask affected --base main` — the story grain wired at `.redkiln/config.yaml:40` | NF-002 and NF-005: nothing under `crates/` moved, so the affected set is empty or trivially green. This is the story-grain gate; it is not evidence for any AC-### here. |
| **4 — e2e / whole-tree** | Deferred — `cargo xtask ci` is `terminal-gate-run`'s and runs **last** in the project, on the tree that already carries the atoms, the maps, `links.kb`, the reconciliation record and the DoD ledger | A run taken here "proves the gate, not the deliverable" (`_decomposition.md`, `## Architecture brief`, ordering 4; `## Testing brief`, merge-gate step 5). Named so its absence reads as a decision rather than an omission. |

`require_ledger: true` (`.redkiln/config.yaml:62-67`) makes `_ledger.md` part of the gate: every
AC-### row must carry cited evidence before `implement → report`. `require_commit_provenance:
true` (`:73`) makes the wave commit recordable via `redkiln record-links <id> --sha`.

## Risks and coupling (PR-scoped)

| risk or coupling | likelihood / impact | the seam that holds it |
| --- | --- | --- |
| **The carried payload is dropped by the narrowed invocation.** HS-P0021's spec was written expecting the default glob; AC-001's narrowing is exactly what invalidates that expectation. | Medium / High | AC-001 names it as inventory (c) explicitly, and the Context pack calls it "the easiest to lose". Detected after the fact by `ls .kb/_intake/` showing a leftover — but that is a re-run, not a repair. |
| **A journey atom lands with `authority_tier: guideline`.** Every neighbouring atom an author opens while composing carries `note` or `guideline`, and `.kb/playbooks/README.md` says `guideline` in its first sentence. | High / High | AC-003, whose Tier 1 check is a one-line `rg` — and which exists *because* AC-007's green run cannot see it (`authority_tier` is `z.string().min(1)`). |
| **AC-007 green is read as covering AC-006.** The two are one command apart in the runbook and one directory apart in the tree. | Medium / High | The ACs are separate rows with separate commands and separate evidence, and the storymap, the architecture brief and the testing brief all say so independently. The ledger's AC-007 note records the blind spot beside the green result. |
| **A `source_paths` entry that resolves only on the sibling branch.** The discovery set was verified in the architecture brief, but this story runs after a merge-forward. | Low / High | EC-007 and AC-004's `test -f` sweep. The failure is loud and pre-commit; the wrong repair (deleting the entry) is named in EC-007 so it is not reached for. |
| **Coupling to `staged-audience-payload` (blocks-on).** Its AC-008 fixes the `persona-*.md` / `journey-*.md` prefix set precisely so this story's array can enumerate it, and its AC-004 puts the observation-status word in the staged text this wave carries into the atom. | — | By file: `.kb/_intake/persona-*.md` and `.kb/_intake/journey-*.md`. That story's own Tier 3 check reads *this* spec's invocation, so the coupling is checked from both ends. |
| **Coupling to `charter-open-question-disposition` (same milestone, upstream of the array).** Its handoff block is the only source for the deferral file names, and it may legitimately list zero. | — | EC-002 for the missing/ambiguous case; AC-001 for the stated-zero case. Not a `depends_on` edge for this story: the storymap routes it through `staged-audience-payload`, which itself depends on it. |
| **Coupling to `product-layer-mounting` (unlocks).** It re-proves `validate --kb` after the reciprocal edges and the map section, and it owns the two-hop walk, `links.kb` and the `## Knowledge Harvest` row. | — | AC-009's handoff list, and the explicit AC-011 ownership seam in `_storymap.md`, `## Coverage`. This story must not pre-empt any of the four mount points beyond the appends the run itself performs. |
| **Cross-project input from HS-P0021 (`page-need-discipline`), not a blocks-on edge.** Its staged payload rides this wave; its text is not this story's. | — | AC-003's diff read, and AC-A10's vehicle-versus-content seam. If the payload is not staged when the wave runs, it is EC-001's mirror image: report it rather than authoring a substitute. |
| **The wave is not idempotent and a re-run is not free.** | Low / Medium | EC-009: a second run carries a distinct wave id so the first's audit trail survives, and both are recorded. |

## Dependencies

**Blocks on** — `staged-audience-payload`. This is the story's only `depends_on` edge and it is
a hard one: without the staged persona and journey documents there is nothing for the wave to
ingest, and without that story's prefix-naming rule (its AC-008) the enumerable array AC-001
requires is not assemblable. Its own AC-008 verification reads *this* spec's invocation, so the
two are checked against each other rather than each against its own intention.

Two further inputs are consumed but are **not** `depends_on` edges for this story:
`charter-open-question-disposition`'s handoff block reaches here transitively, because
`staged-audience-payload` already depends on it (`_storymap.md`, `## Slices`); and HS-P0021's
`playbook-atom-staged-for-ingest` is a cross-project input under
`.bklg/docs-that-teach/page-need-discipline/`, already staged, carried rather than blocked on.

**Unlocks** — `product-layer-mounting`, which cannot begin until atoms exist to mount and
consumes AC-009's landed-atom list directly. Downstream of the milestone, `dod-scenario-ledger`
depends on `product-layer-mounting` and therefore on this story, and `terminal-gate-run` runs
last on the tree this wave's commit is part of.

## Anchors (progressive disclosure)

Linked, not pasted. The Context pack above is sufficient to start; each row below is the deeper
artefact whose *detail* is needed at one specific moment.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/_intake/README.md` | The staging contract in four parts: the default input is `.kb/_intake/*.md` (`:3-7`), staged material is not held to `KbFrontmatter` (`:7-11`), a successful ingest clears the directory and the whole wave commits together (`:13-19`), `validate --kb` is blind to this directory (`:21-27`), and the README itself is deliberately not an atom (`:29-31`). Four of this story's nine criteria are graded against those lines. | Before assembling the `files` array, and again before writing the AC-006 and AC-007 ledger rows. | AC-001, AC-006, AC-007 |
| `.kb/product/README.md` | `:6-13` is the tier table and the four-slot persona vocabulary — the authority for `kind: concept` + `authority_tier: product` and for `kind: playbook` + `authority_tier: product`; `:15-21` is why U1 reads these atoms at all; `:23-24, 28-31` is how a guess acquires the standing of a finding. The layer README, never the neighbour. | Open while composing and while checking every landed atom; keep it open for the whole tier sweep. | AC-002, AC-003 |
| `.kb/playbooks/README.md` | Read as the **tier trap**, not as a model: its first sentence fixes `authority_tier: guideline` for its own layer, which is correct there and wrong for a journey atom. It is also where HS-P0021's carried payload correctly lands. | Only when checking the carried payload's tier — and consciously, so its `guideline` is not carried across to a journey. | AC-003 |
| `.kb/README.md` | `:16-26` is the required-field table every landed atom is checked against; `:31-35` states that `authority_tier` is `z.string().min(1)`, which is the reason AC-007 green is not evidence for AC-002 or AC-003 and the reason that limit is written into the ledger. | Before writing AC-007's ledger note, and whenever a frontmatter question arises. | AC-002, AC-007 |
| `.kb/_templates/atom.md` | The composition primitive: the frontmatter vocabulary and the `## Context` / `## Body` / `## Consequences / links` skeleton. It is what makes "presentation exists at all" checkable in a corpus with no CSS. | When reviewing a landed atom's body shape, before the Tier 2 read. | AC-002 |
| `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | `:23-28` is the corpus's own `source_paths` exemplar — citing both the `.kb/_intake/…` staging file and the long-form evidence. It is the pattern AC-004 requires rather than a pattern this story invents. | When checking `source_paths` on the first landed atom, before sweeping the rest. | AC-004 |
| `.kb/maps/domain-map.md` | `:144-150` is the map's own "adding an entry" rule — append a `##`, group by kind under bold labels, cite the id beside the link, one sentence of orientation — and `:105-142` shows the format in use. It is the shape the wave's map-sync output is judged against, and the anchor stability IQ-3 protects. | After the run, when reading the map diff. | AC-008 |
| `.kb/maps/open-questions-index.md` | `:136-143` is the index bullet's fixed shape (status word first, then link, then id, then one sentence) and the rule about starting a new `##` only where the domain has none; `:11-13` is the precedent that a superseded question stays listed rather than removed. | After the run, if any deferral atom landed. | AC-008 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The three briefs this story is graded by, and the only place their detail lives: `## Architecture brief` (AC-A01, AC-A02, AC-A03, AC-A05, AC-A09, AC-A10; §2's mount points and the AC-009/AC-011 separation; T1, T4, T5), `## UX brief` (the primitive table, IQ-1…IQ-8, AC-UX-01…AC-UX-09), `## Testing brief` (the four tiers, the merge-gate order, the fixtures/seams section, and the AC-### → tier mapping rows for AC-004, AC-005, AC-008, AC-009, AC-011). | The UX brief's primitive table before composing; the testing brief's tier mapping before writing any verification; the architecture brief's T1/T4 before the invocation. | AC-001 … AC-009 |
| `.bklg/docs-that-teach/durable-audience-closeout/staged-audience-payload/spec.md` | Inventory (a). Its AC-008 fixes the `persona-<slug>.md` / `journey-<slug>.md` prefix set — the naming rule that makes this story's array enumerable — and its AC-004 fixes the observation-status word this wave carries into the persona atoms unchanged. | Before assembling the array; again during the Tier 2 read, to confirm the slots survived ingest. | AC-001, AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/charter-open-question-disposition/spec.md` | Inventory (b). Its AC-006 obliges the handoff block to name each staged deferral file, its target `##` section in the index and its bullet text — so this story enumerates rather than infers, and the index append has its shape decided upstream. | Before assembling the array; again if EC-002 looks live. | AC-001, AC-008 |
| `.bklg/docs-that-teach/page-need-discipline/playbook-atom-staged-for-ingest/spec.md` | Inventory (c), and the vehicle-versus-content seam. It names the single carried payload and states that its text belongs to HS-P0021 — the reason this story may cause the atom to exist and may not edit a sentence of it. | Before assembling the array (this is the entry most easily dropped), and before the AC-003 diff read. | AC-001, AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/spec.md` | The consumer. It defines what the next story needs from AC-009's handoff and which four mount points remain its own — so this story neither under-delivers the list nor pre-empts the mounting. | When writing the AC-009 handoff list, before committing. | AC-009 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | `## Merge order` §3 fixes this story's position in the wave; `## Coverage`'s AC-011 row fixes the ownership seam that keeps AC-007 scoped to "the atoms as landed"; "Notes the implementer needs at the story grain" repeats the tier trap and the AC-009/AC-011 separation independently. | Before starting, and again before claiming AC-007. | AC-007, AC-009 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The governing atom for every repair route in the error table: correction is a new atom that supersedes, never an edit — and for an accepted decision the validator enforces it against `HEAD`. It is what makes EC-003, EC-004 and EC-006 non-negotiable rather than stylistic. | The moment any landed atom looks wrong and an editor starts to look attractive. | AC-005, AC-008 |
| `.redkiln/config.yaml` | `:40` the story-grain command, `:48` the reachability grain the next story runs, `:60` the terminal grain deliberately not run here, `:62-67` `require_ledger`, `:73` `require_commit_provenance` — the mechanisms that actually gate this story. | Before the final commit and the ledger sign-off. | AC-001 … AC-009 |
| `.bklg/docs-that-teach/initiative.md` | `:465-468` is DoD-15, the scenario this story turns green in two of its three halves; `## Referenced personas & journeys` (`:275`) states why the product layer is "structurally present and functionally empty" today and why hand-authoring is the reverted practice. | When writing the ledger's DoD linkage, and if the question "why not just write the atom" arises. | AC-005, AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The long-form evidence every landed atom must cite in `source_paths`, and the material IQ-6 exists to keep pointed at one audience rather than two. | When verifying AC-004's `_discovery/…` half resolves and points at the right artefact. | AC-004 |

## Clarifications resolved during spec

**The AC set is exactly the nine the first pass enumerated.** None was added, none dropped. The
brief-grain criteria this story also discharges (AC-A01, AC-A02, AC-A03, AC-A05, AC-A09,
AC-A10, AC-UX-01, AC-UX-02, AC-UX-05, AC-UX-06, AC-UX-09) are folded into those nine rather
than enumerated separately, which is what `_storymap.md`'s `## Coverage` note requires of every
story in this project.

**Zero deferral documents is a valid outcome and is now stated, not left to inference.**
`charter-open-question-disposition` may legitimately answer every charter question on the
record and stage nothing. AC-001 therefore requires the zero case be *stated* in the recorded
array rather than showing up as an array that happens to contain no `open_question` files —
otherwise "none were staged" and "I forgot to look" are indistinguishable in the ledger. EC-002
covers the genuinely broken case separately.

**AC-007 is scoped to the atoms as landed, and the scope is written into the criterion.** The
storymap splits project AC-011 across this story and `product-layer-mounting`; without the
scope in the criterion text, a reviewer could read this story's green run as discharging AC-011
outright and the next story's re-run as redundant. It is not: the reciprocal `related` edges
and the map section change the frontmatter the validator reads.

**The two blind spots are ledger content, not commentary.** `validate --kb` not enforcing the
`authority_tier` vocabulary and not seeing `.kb/_intake/` are recorded *beside* the green
result in `_ledger.md`, because a future reader finding a green AC-007 row and no such note
will reasonably assume it covered more than it did. This is why AC-007's criterion text names
both limits rather than the Context pack alone carrying them.

**"Composition" is not vacated by `hasSurface: false`.** The signed-off `_design.md` records no
API surface and no rendered surface, which removes the signature and doctest obligations — it
does not remove the atom template, the tier tokens, the persona vocabulary and the two map
formats, which the UX brief installs explicitly as this corpus's primitive layer. The
composition invariants above are written against those primitives, and each is carried by a
numbered AC rather than a prose bullet, so `redkiln verify` can extract it.

**One front-half table cell was disambiguated, and nothing else in it was touched.** The
`## Behavior and interfaces` row whose first cell read "AC-009 and AC-011 are observed
separately" now reads "Project AC-009 and project AC-011 are observed separately". `redkiln
verify` extracts acceptance criteria by matching a leading `| AC-001 |` table cell, and a row
whose leading cell *starts* with `AC-009` is a row that can be mistaken for this story's own
AC-009 and paired with the wrong criterion. Both ids there are `project.md`'s, not this
spec's; saying so makes the row unambiguous to a reader as well as to the extractor. The
row's meaning, its evidence column and its position are unchanged.

**No ADR is cited because none exists for this surface, and that is recorded as a finding.**
All sixteen accepted decision atoms govern the Rust contract. What binds instead is the
single-write-path rule, `.kb/governance/rewrite-the-referent-never-the-reasoning.md`, and
`harvest_kb: true` on the `closeout` stage. Nothing in this story is owed an ADR, and writing
one as closeout exhaust is a named project non-goal.
