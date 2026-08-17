---
item: HS-S0176
stage: spec
created: 2026-08-17T13:16:27.636Z
updated: 2026-08-17T13:16:27.636Z
template_sig: 87bbf1d0
rendered_sig: 1945db46
---

# Spec — Audit DT-1 … DT-10 for resolution, deferral or gap

## Scope lock

| What | Where |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — `## Open design tensions` (`:483-494`) is the canonical DT-1 … DT-10 table; `## Exit criteria for this initiative` (`:664-666`) is the bar this story discharges |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `## Design tension ownership` (`:148-159`) maps each DT to a real project id; `:161` records that this project owns none |
| Project (mandate + ACs) | [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../project.md) — AC-018 (`:207`), DR-14 (`:179-182`), and the risk row that says this project owns no tension (`:252`) |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` names the audit table as artefact 4 (`:32-33`), fixes its three-state vocabulary (`:70`) and its row shape (AC-UX-10, `:270-273`) and its a11y floor (AC-UX-09, `:266-269`); `## Testing brief` puts AC-018 at Tier 2 content review with **no automated reader** (`:878`) and forbids claiming Tier 1 alone for it (AC-TB-01, `:824-826`) |
| Grounding | [`../_grounding.md`](../_grounding.md) — what was verified against this worktree, including that no Accepted ADR binds this project's diff surface |
| Signed-off design (BINDING) | [`../_design.md`](../_design.md) — `hasSurface: false`, `# no items` (`:45-47`), approved by the repository owner 2026-08-17 (`:91-94`). It names the DT-1 … DT-10 audit table as one of this project's four artefacts (`:29`) |
| Story map row | [`../_storymap.md`](../_storymap.md) — this story's one-line slice (`:52`), the `closeout-adjudication` slice rationale (`:67-69`), the story-grain notes (`:99-104`) |
| Roadmap pointer | None. This project ships no Rust; `RUNBOOK.md` phases do not carry it |

## One-line PR slice

Produce the DT-1 … DT-10 audit table over the merged tree: for each tension, the owning project, its
`_design.md` resolution or recorded deferral, and — where it has neither — an explicit `gap` row rather
than a silent pass.

That sentence is the story map's own, not a paraphrase of it (`../_storymap.md:52`). The map was reviewed
and approved by a human on 2026-08-17; restating it in different words would create a second description
that can drift from the approved one.

## Executive summary

This PR lands **one file**: a ten-row audit record in this story's own folder, plus the one-line append to
`project.md`'s `## Companions` that mounts it. Nothing else.

The delta against the artifacts already in the tree is not "write down what the tensions are" — the charter
already enumerates them (`../../initiative.md:483-494`) and the decomposition already assigns each an owner
(`../../_decomposition.md:148-159`). The delta is that **nobody has yet opened all six owning `_design.md`
files on one tree and checked that the resolution the ownership table promises is actually there**. The
initiative's exit criterion is written in exactly those terms — resolved *in its owning project's
`_design.md`*, or explicitly deferred with the deferral recorded, *"none is left silently unowned"*
(`../../initiative.md:664-666`) — and DR-14 adds the reason it can only be discharged by reading: with
`design.capture` deliberately absent from `.redkiln/config.yaml` (`:75-83`), *"the written resolution is the
only record there will ever be"* (`../project.md:179-182`).

There is no checker to write here and none may be invented. The testing brief classifies AC-018 as Tier 2
content review with the note *"no automated reader exists for this table"* (`../_decomposition.md:878`), and
AC-TB-01 forbids citing Tier 1 alone for it (`:824-826`). The whole deliverable is a human-read table whose
every row carries a `file:line` a reviewer can follow.

## Context pack

Everything below is a decision this story must honor. Deeper material is behind the signposted anchors; do
not go looking for it before you need it.

**The audit is an audit. It resolves nothing.** If a tension turns out to have no resolution and no recorded
deferral, this story writes a `gap` row and routes it — it does **not** decide the tension. Deciding one here
would re-open a sibling project's *signed-off* design (every owning `_design.md` carries a dated human
sign-off), which the project's own Definition of Done forbids in as many words: *"No sibling project's
requirement was re-opened, re-decided or re-implemented here"* (`../project.md:219-220`). The same rule
carries a second edge with teeth: if a gap looks like it wants an ADR, record and route the gap — an accepted
decision atom is immutable and supersession is a deliberate act, never a closeout by-product
(`.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `../_storymap.md:99-101`).

**The ids come from the charter; the owners come from the decomposition. Never the reverse.** The charter's
tension table names its owning projects *descriptively* — "Conceptual bridge", "Page-need discipline",
"First encounter", "Checked prose", "Comprehension evidence", "Reach and placement" — and says
`/redkiln:plan` assigns the real ids (`../../_decomposition.md:29-32`). The decomposition then did assign
them, and **changed the cut** while doing so: the six descriptive owners collapse to five real projects, and
`application-author-path` ends up carrying four tensions (DT-1, DT-4, DT-5, DT-6) because they are one
coupled decision rather than four independent ones (`:52-58`). An audit row that names "Conceptual bridge"
as an owner has cited the superseded label; the binding map is `_decomposition.md:148-159`:

| DT | Owning project | Where its resolution must be found |
| --- | --- | --- |
| DT-1, DT-4, DT-5, DT-6 | HS-P0022 `application-author-path` | `.bklg/docs-that-teach/application-author-path/_design.md` |
| DT-2, DT-3, DT-8 | HS-P0021 `page-need-discipline` | `.bklg/docs-that-teach/page-need-discipline/_design.md` |
| DT-7 | HS-P0020 `checked-documentation-surface` | `.bklg/docs-that-teach/checked-documentation-surface/_design.md` |
| DT-9 | HS-P0024 `comprehension-evidence` | `.bklg/docs-that-teach/comprehension-evidence/_design.md` |
| DT-10 | HS-P0023 `reach-and-adapter-path` | `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` |

**Do not read the dossier's numbering as DT numbering.** The tensions originate in
`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Open design tensions`
(`:461-465`), where they are an unnumbered-in-DT-terms ordered list: dossier item **1** is DT-**7** (tabs and
hidden panels), item **2** is DT-**8** (the fold line), item **3** is DT-**2** (the Diátaxis taxonomy). Any
row that cites "tension 3" from the dossier as evidence for DT-3 has cited the wrong tension and the audit
has silently passed something it never looked at. Cite the charter row for the id and the owning `_design.md`
for the resolution; the dossier is background only.

**The outcome vocabulary is fixed at three words, and the third one is the point.** The UX brief's state
table fixes it: a design tension is `resolved` / `recorded deferral` / **`gap`**, with the note *"a tension
with neither is reported as a gap, not passed over"* (`../_decomposition.md:70`). Three states, spelled in
words. AC-UX-09 (`:266-269`) forbids carrying any state in this project's diff by colour, emoji, glyph,
strikethrough, ordering or an empty cell — and it names the DT audit table explicitly as covered. An empty
outcome cell is therefore not "not yet checked"; it is a defect.

**Every row must stand alone.** AC-UX-10 (`:270-273`) requires each row of this audit to be self-contained —
the tension id, the owning project, the observer, the tree ref or sha, and the outcome — *"with no row that
depends on the row above to be understood"*. A row reading "same as above" fails the criterion the brief
already wrote.

**A resolution counts only where the exit criterion says it counts.** The bar is the *owning project's*
`_design.md`. Two near-misses are live in this tree and both must be recorded as what they are rather than
laundered into a pass:

- A tension can be discussed in a **non-owning** project's design. `checked-documentation-surface`'s
  `_design.md:267-269` states outright what its DT-7 answer *does not* decide about DT-8, and observes that
  its hidden-marker ban makes DT-8 moot *inside the pinned tree* only. That is a cross-reference worth
  recording in the row's note; it is not DT-8's resolution, which is HS-P0021's (`page-need-discipline/_design.md:240-288`).
- A tension can be resolved in a **story spec** rather than in the design file. DT-9's resolution is owned by
  `comprehension-evidence`, whose `_design.md` names DT-9 as *"the sole durable record"* of that choice
  (`:44`, `:103-109`) — and, as this spec is written, does not yet state which persona was chosen; the story
  that appends it is `comprehension-evidence/dt9-and-fixed-protocol` (`spec.md:27`, `:208`). So DT-9 is the
  one row whose outcome genuinely depends on what has landed by audit time. Expect `resolved` if that story
  shipped its append, `gap` if it did not — and check the file, not the story's status.

**The audit is performed on the merged tree, after the merge, and names it.** `merge-forward-baseline` is
this story's only dependency (`story.md`, `blocked_by: [HS-S0172]`) and it exists to produce one named commit
that every downstream record cites (`../_storymap.md:48`, `:144-147`). The sibling
`initiative/from-contract-to-published-library` owns none of these ten tensions, so the merge is *expected*
to leave all ten resolutions untouched — which is precisely why the audit must be taken after it rather than
before. "Expected" is not "observed", and an audit run on this worktree's pre-merge copy is an audit of a
tree that will not ship.

**The persona-journey slice this serves.** The reader of this record is not any of the three documented
personas — they are the subject of this project's other artefacts, never its audience. It is **U2, the
closeout reviewer**, reading this project's written records in one pass (`../_decomposition.md`, `## UX
brief`, "Who this is actually for"; `../_storymap.md:66-69`). U2's job is to disagree per row, which is why
the row — not the table — is the unit that must carry its own citation, its own observer and its own tree
ref.

**What this project's `_design.md` binds you to, and what it does not.** It records `hasSurface: false` and
`# no items` (`../_design.md:45-47`) and states, correctly, that this project owns **no** design tension
(`:96-98`). That is not licence to skip the design contract: it is the reason the audit table is one of the
four artefacts the design file *does* name (`:29`), and the reason its quality bar lives in the UX brief's
AC-UX-* criteria rather than in a `## Signatures` block. There is no screen, no token, no component, and
inventing one is the anti-pattern.

## Integration contract

- **Archetype**: `capability` (`story.md`, `archetype: capability`) — a user-observable slice, where the user
  is U2 and the observable is a record they can act on.
- **Slice / milestone**: `closeout-adjudication`. Slice-mates, implemented in the same context and mounted as
  one integrated surface: `reconciliation-ledger`, `charter-open-question-disposition`
  (`../_storymap.md:50-52`). All three are written records under this project's folder, read by the same
  reviewer in the same pass (`:66-69`).
- **Mount point**: **`.bklg/docs-that-teach/durable-audience-closeout/project.md`** — its `## Companions`
  list (`:291-301`). That list is this project's real composition root for documents: it is what a reviewer
  opening the project reaches its artefacts *through*. The audit record is mounted when a new companion line
  points at it and is reachable in one hop from `project.md`; a record sitting in a story folder that
  nothing links to is this corpus's exact analogue of a component rendered into no tree
  (`../_storymap.md:70-75`). `design.capture` is deliberately absent from `.redkiln/config.yaml` (`:75-83`),
  so there is no perceptual gate that would find it unmounted — the link is the whole mounting mechanism.
- **The artefact this story creates**:
  `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md`. Underscore prefix, matching
  the corpus's convention for non-stage companions (`_ledger.md`, `_review.md`, `_grounding.md`); it is not a
  redkiln stage artifact and must never be given stage frontmatter.
- **Wires into** (read-only, every one verified present):
  - `.bklg/docs-that-teach/initiative.md:483-494` — the canonical DT id + tension text.
  - `.bklg/docs-that-teach/_decomposition.md:148-159` — the DT → project-id ownership map.
  - `.bklg/docs-that-teach/application-author-path/_design.md` — DT-1 (`:81-120`), DT-4 (`:122-168`),
    DT-5 + DT-6 (`:170-262`).
  - `.bklg/docs-that-teach/page-need-discipline/_design.md` — DT-2 (`:168-212`), DT-3 (`:214-238`),
    DT-8 (`:240-288`).
  - `.bklg/docs-that-teach/checked-documentation-surface/_design.md` — DT-7 (`:200`), and `:267-269` for what
    it explicitly does not decide.
  - `.bklg/docs-that-teach/comprehension-evidence/_design.md` — DT-9 (`:44`, `:103-109`).
  - `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` — DT-10 (`:87`, `:475`).
  - The merged baseline ref and sha produced by `merge-forward-baseline` (HS-S0172).
- **Document primitives it composes** (compose, do not hand-roll — `../_decomposition.md`, "Design-system
  primitives"): the `| … |` markdown table already used by every `project.md` risk table and by the charter's
  own tension table; the `file:line` citation form used throughout this initiative's planning corpus; the
  `## Companions` bullet form at `../project.md:291-301`. No new notation, no legend, no key.
- **Renders surfaces**: **none**. The project's `_design.md` declares `hasSurface: false` and
  `# no items — no public API surface, no rendered UI surface` (`../_design.md:45-47`); there is no surface id
  to claim. The audit table is artefact 4 of that file's prose surface list (`:29`), which is a document, not
  a render path.
- **Conformance rule(s)**: none, and this is not an omission. This story adds no Rust, touches no port, and
  is not adapter-observable; `happenstance-testkit` has nothing to say about a markdown table. The testing
  brief's own row for AC-018 records that no automated reader exists (`../_decomposition.md:878`), and
  AC-TB-01 makes claiming a machine tier alone here a defect (`:824-826`).
- **Clause(s)**: none. No `spec/SPECIFICATION.md` clause is discharged or amended, and nothing `[FROZEN]` is
  touched — this project's diff does not reach `spec/` at all (`../_storymap.md:21-23`).
- **Advances DoD scenario**: **none of the fifteen, directly, and that is deliberate.** The tension audit is
  an *exit criterion* (`../../initiative.md:664-666`), not a Definition-of-Done scenario; the fifteen are
  enumerated at `:413-468` and none of them is this. What the audit does do is protect three of them from
  passing on a resolution that quietly evaporated: DoD-4 rests on DT-6's compiled wrong-side contrast, DoD-5
  on DT-9's chosen persona, DoD-13 on DT-7's hidden-content answer. A `gap` row against any of those three is
  an escalation, not a footnote. Say so in the row.

This story must be delivered **mounted**: the record exists *and* `project.md`'s `## Companions` reaches it.

## PR boundary

**In this PR**

- `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` — the ten-row audit
  record, its provenance header (baseline ref + sha + observer + date), and per-row citations.
- `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_ledger.md` — required, because this
  spec declares AC-###, and `require_ledger: true` is on (`.redkiln/config.yaml:62-67`). One row per AC with
  a real `file:line`. Authored by this spec's second pass and filled at implementation.
- `.bklg/docs-that-teach/durable-audience-closeout/project.md` — **body only**, one appended `## Companions`
  bullet pointing at `_audit.md`. This is the mount named in the Integration contract, so touching it is not
  scope drift. The YAML frontmatter is the CLI's and must not be edited (`CLAUDE.md`, "The CLI is the only
  writer of an item's system frontmatter").

**Explicitly not in this PR**

- **Any sibling project's `_design.md`.** Not a correction, not a clarifying sentence, not a missing
  resolution supplied helpfully. Each is human-signed-off; a gap is reported and routed, never filled here
  (`../project.md:219-220`).
- **Any file under `crates/`, `spec/`, `xtask/`, `standards/` or `docs/`.** *"A story that proposes to edit
  [them] has left this project"* (`../_storymap.md:21-23`).
- **Anything under `.kb/`.** No atom, no map append, no `.kb/decisions/` edit. The product-layer promotion
  and its ingest wave belong to `staged-audience-payload` / `audience-ingest-wave` /
  `product-layer-mounting`, and `.kb/decisions/` is touched by no story in this project
  (`../_storymap.md:99-101`). A design finding that wants a `.kb/design/` atom is a routed gap in prose, not
  an atom authored here.
- **The reconciliation record, the charter open-question dispositions, the DoD ledger, the clause-completeness
  statement and the terminal gate run.** Slice-mates and downstream stories own each
  (`../_storymap.md:48-58`).
- **A checker, a lint or an xtask step for this table.** None exists, none is wanted, and adding one would
  put Rust in a project that ships none.

```
.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/**
.bklg/docs-that-teach/durable-audience-closeout/project.md
```

**Merge DoD (one line).** `_audit.md` carries ten rows for DT-1 … DT-10, each with its owning project id, a
`file:line` into that project's `_design.md`, one of `resolved` / `recorded deferral` / `gap` spelled in
words, and the merged baseline sha — and `project.md`'s `## Companions` reaches it in one hop.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| The audit is taken on the merged tree and names it | A provenance header states the branch ref, the commit sha `merge-forward-baseline` produced, the observer, and the date. Nothing in the table is observed against this worktree's pre-merge copy. | `../_storymap.md:48`, `:144-147`; `../project.md:201` (AC-012's framing of "names the merged tree"); `story.md` `blocked_by: [HS-S0172]` |
| Exactly ten rows, ids DT-1 … DT-10 | Ids and tension text are taken from the charter table. Not nine, not eleven, no renaming, no merging of two rows because two tensions share an owner. | `../../initiative.md:483-494` |
| Owner column carries the real project id and slug | `HS-P0020 checked-documentation-surface`, `HS-P0021 page-need-discipline`, `HS-P0022 application-author-path`, `HS-P0023 reach-and-adapter-path`, `HS-P0024 comprehension-evidence`. The charter's descriptive labels are superseded and must not appear as owners. | `../../_decomposition.md:148-159`, `:29-32` |
| Each row cites the owning `_design.md` by `file:line` | The row's evidence is a path a reviewer follows, not a claim. This is the falsifier the discover stage named: *"an audit that counts ten resolutions without checking each is in its owning project's `_design.md`"*. | `discover.md:53-55`; `../_decomposition.md:270-273` |
| Outcome is one of three words | `resolved` / `recorded deferral` / `gap`, spelled out. No tick, no colour, no emoji, no empty cell, no ordering-carries-meaning. | `../_decomposition.md:70`, `:266-269` |
| A resolution recorded outside the owning project does not count as `resolved` | It is recorded in the row's note as a cross-reference, and the row's outcome still reflects the owning file. The live instance is `checked-documentation-surface/_design.md:267-269` discussing DT-8, which is HS-P0021's. | `../../initiative.md:664-666`; `.bklg/docs-that-teach/checked-documentation-surface/_design.md:267-269`; `.bklg/docs-that-teach/page-need-discipline/_design.md:240-288` |
| A `recorded deferral` must be *recorded*, and reasonable | The exit criterion's wording is "explicitly and reasonably deferred with the deferral recorded". A deferral inferable from silence is a `gap`. The row names where the deferral is written and what it defers to. | `../../initiative.md:664-666`; `../project.md:179-182` |
| A `gap` row names a route and stops there | Who owns closing it (the owning project), where it would be written (that project's `_design.md`), and — for DT-6, DT-7, DT-9 — that a Definition-of-Done scenario rests on it. The audit does not choose an option, does not edit the owning file, and does not open an ADR. | `../project.md:219-220`, `:124-127`; `../_storymap.md:99-101`; `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| DT-9 is checked in the file, not inferred from a story's status | `comprehension-evidence/_design.md` claims to be DT-9's sole durable record but, as of this spec, does not state the chosen persona; `comprehension-evidence/dt9-and-fixed-protocol` is the story that appends it. Read the file at audit time. | `.bklg/docs-that-teach/comprehension-evidence/_design.md:44`, `:103-109`; `.bklg/docs-that-teach/comprehension-evidence/dt9-and-fixed-protocol/spec.md:27`, `:208` |
| Every row is self-contained | Tension id, owning project, observer, tree ref/sha, outcome, citation — readable without the row above it. | `../_decomposition.md:270-273` (AC-UX-10) |
| The record is mounted, not merely written | One appended bullet in `project.md`'s `## Companions`, reaching `_audit.md` in one hop. Body only; frontmatter untouched. | `../project.md:291-301`; `CLAUDE.md`, "The CLI is the only writer of an item's system frontmatter" |
| Verification is Tier 2 content review; no machine tier is claimed alone | A reviewer follows every citation. No checker is added, and `redkiln validate --kb` / `cargo xtask ci` are cited for nothing here — they cannot read this table. | `../_decomposition.md:878`, `:824-826` |
| Composition uses primitives already in the corpus | The markdown table shape used by every `project.md` risk table and by the charter's tension table; the `file:line` citation form; the `## Companions` bullet form. No legend, no key, no new notation. | `../_decomposition.md`, "Design-system primitives — compose these, do not hand-roll"; `../../initiative.md:483-484` |

**Interfaces.** None in the Rust sense: no `pub` item, no trait, no feature gate, no doctest. The project's
signed-off design records `hasSurface: false` and `# no items` (`../_design.md:45-47`), so there is no
signature to match and nothing for `## Signatures`, `## Visibility and stability` or `## The doctest` to
bind. The only "interface" this story exposes is the audit record's column set, which is fixed by AC-UX-10
above and is consumed by one reader, U2.

## Data and migrations

**N/A.** This story adds no schema, no persisted state, no store, no migration and no frontmatter. It writes
one markdown file and appends one bullet to another. The only structured data in the repository this story
comes near is the YAML frontmatter of `project.md`, which it must **not** touch — the `redkiln` CLI is the
single writer of an item's system fields (`id`, `stage`, `status`, `updated`, `links`), a `PreToolUse` hook
denies the edit, and the prose body beneath the closing `---` is the only part this story may append to
(`CLAUDE.md`, "The CLI is the only writer of an item's system frontmatter"). `_audit.md` is a plain companion
document with no frontmatter at all; giving it stage frontmatter would make it look like a redkiln stage
artifact the CLI never rendered.

## Acceptance criteria

Every criterion below is framed from the reader whose goal it serves. The primary reader is **U2, the
closeout reviewer** — "show me every adjudication you made, including the ones that changed nothing, so I
can disagree with exactly one of them without reading the discovery corpus" (`../_decomposition.md`,
`## UX brief`, reader table). **U3, the future maintainer**, appears wherever a row must stay re-checkable
against a named tree. All seven trace to project **AC-018** (`../project.md:207`) and, through it, to the
initiative's exit criterion (`../../initiative.md:664-666`).

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U2 opens `_audit.md` intending to disagree with exactly one tension's adjudication, **WHEN** they scan it once, **THEN** they find exactly ten body rows keyed `DT-1` … `DT-10`, each carrying the charter's own tension text in short form, with no id absent, no id invented, and no two tensions merged into one row because they share an owner — and any summary sentence sits *above* the table rather than in place of a row. | Tier 2 content review of `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` against `../../initiative.md:483-494`, one row at a time. Tier 1 support: `rg -c '^\| DT-' .bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` returns `10`, and `rg -o 'DT-([1-9]\|10)' … \| sort -u` returns ten distinct ids. Tier 1 is never cited alone (AC-TB-01, `../_decomposition.md:824-826`). |
| AC-002 | **GIVEN** U2 wants to route a finding to the team that owns it, **WHEN** they read any row's owner cell, **THEN** it names the real backlog project by id *and* slug — `HS-P0020 checked-documentation-surface`, `HS-P0021 page-need-discipline`, `HS-P0022 application-author-path`, `HS-P0023 reach-and-adapter-path`, `HS-P0024 comprehension-evidence` — matching `_decomposition.md`'s ownership map exactly, with the charter's superseded descriptive labels ("Conceptual bridge", "Page-need discipline", "First encounter", "Checked prose", "Comprehension evidence", "Reach and placement") appearing nowhere as an owner. | Tier 2 review of each owner cell against `../../_decomposition.md:148-159`; DT-1/DT-4/DT-5/DT-6 must all resolve to `HS-P0022`, which is where the approved cut differs from the charter's six descriptive owners (`:29-32`, `:52-58`). Tier 1 support: `rg -n 'Conceptual bridge\|First encounter\|Reach and placement' …/_audit.md` returns nothing in the owner column. |
| AC-003 | **GIVEN** U2 refuses to accept "resolved" as a claim, **WHEN** they follow any row's evidence cell, **THEN** it is a `file:line` into the **owning** project's `_design.md` that resolves in the merged tree and, when opened, actually contains the resolution the row asserts — and where a tension is discussed in a *non-owning* project's design (the live instance is `checked-documentation-surface/_design.md:267-269` on DT-8, which is HS-P0021's) that discussion is recorded in the note as a cross-reference and does **not** move the outcome to `resolved`. | Tier 2 review: the reviewer opens every cited `file:line` and confirms the text found there is the resolution claimed. This is the falsifier `discover.md:53-55` names — "an audit that counts ten resolutions without checking each is in its owning project's `_design.md`". Tier 1 support: `test -f` over every distinct path cited, plus `rg -n ':[0-9]' …/_audit.md` showing no evidence cell without a line reference. |
| AC-004 | **GIVEN** U2 must be able to tell a settled tension from an unowned one without inference, **WHEN** they read any outcome cell, **THEN** it holds exactly one of the three words `resolved` / `recorded deferral` / `gap` spelled out in full; a `recorded deferral` names where the deferral is written and what it defers to; and a `gap` row names the owning project, the file the resolution would be written into, and — for DT-6, DT-7 and DT-9 — the Definition-of-Done scenario that rests on it, then **stops**: no option chosen, no sibling `_design.md` edited, no ADR opened. | Tier 2 review against `../_decomposition.md:70` (the three-state vocabulary) and `../project.md:219-220` (no sibling requirement re-opened). Tier 1 support: every outcome cell matches `^(resolved\|recorded deferral\|gap)$`; `git diff --name-only` shows no path under any sibling project's `_design.md` and no path under `.kb/decisions/`; `redkiln validate --kb` exits zero, whose accepted-decision immutability check makes an accidental `.kb/decisions/` touch structurally fail (`../_decomposition.md`, `## Testing brief`, Notes). |
| AC-005 | **GIVEN** U3 returns months later and asks "against which tree, on what evidence", **WHEN** they read the record's provenance header and any single row, **THEN** both name the merged tree: the header states the branch, the full 40-hex merged sha copied from `_baseline.md`, the sibling branch merged, the observer and the date; and each row repeats the observer and the short sha in its own cell, so a row lifted out of the table still says what it was observed against. No observation is taken against this worktree's pre-merge copy. | Tier 2 review that the sha in the header is the one `_baseline.md` records (not re-derived from `git log`, per `merge-forward-baseline/spec.md:98`, `:126`). Tier 1 support: `rg -n '<merged-sha>' .bklg/docs-that-teach/durable-audience-closeout/_baseline.md .bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` matches in both, and `git cat-file -t <merged-sha>` returns `commit` (the AC-012 "also tier 4" check, `../_decomposition.md`, tier mapping). |
| AC-006 | **GIVEN** a reader who has never heard of this story opens `project.md`, **WHEN** they look for this project's artefacts, **THEN** they reach `_audit.md` in **one hop** from the `## Companions` list via link text that names the destination — and `git diff .bklg/docs-that-teach/durable-audience-closeout/project.md` shows a pure append: one added bullet, zero `-` lines, no reflow of any pre-existing section, and no change whatsoever to the YAML frontmatter the CLI owns. | Tier 2 review: the reviewer performs the one-hop walk from `project.md:291-301`. Tier 1 support: `rg -n '_audit' .bklg/docs-that-teach/durable-audience-closeout/project.md` matches inside `## Companions`; `git diff -U0 -- .bklg/docs-that-teach/durable-audience-closeout/project.md \| rg '^-[^-]'` returns nothing (the IQ-3 falsifier form, `../_decomposition.md:180-188`, `:254-257`). |
| AC-007 | **GIVEN** U2 reads the record in a plain terminal with no colour, **WHEN** they take any single row out of context, **THEN** it is fully understandable on its own — tension id, owner, evidence `file:line`, outcome word, observer and sha, note — with no cell reading "same as above", "ditto", "see row 3" or left empty; no state anywhere carried by colour, emoji, glyph, tick, strikethrough, ordering or an empty cell; the record composed only from primitives already in this corpus (the `\| … \|` markdown table, the `file:line` citation form, the `## Companions` bullet) with no legend, key, fold, `<details>`, tab or collapsed panel; and the density budget held — one provenance header of at most 12 lines, exactly 10 body rows, exactly the 7 columns named in Implementation notes, at most 60 words per note cell, and at most one further `## Routed gaps` section. | Tier 2 review against `../_decomposition.md:266-269` (AC-UX-09, the a11y floor, which names the DT audit table explicitly) and `:270-273` (AC-UX-10, self-contained rows). Tier 1 support: `rg -n 'same as above\|ditto\|see row\|<details>\|:white_check_mark:' …/_audit.md` returns nothing; `rg -n '\|\s*\|' …/_audit.md` finds no empty cell; a non-ASCII scan finds no emoji or glyph outside prose. |

**Coverage.** AC-018 is the only project AC traced, and it decomposes without residue: AC-001 gives it its
ten rows, AC-002 its owners, AC-003 its "in the owning project's `_design.md`" clause, AC-004 its
`resolved` / `recorded deferral` / `gap` disposition and its "reported as a gap rather than passed over"
clause, AC-005 the merged tree it is observed against, AC-006 its reachability, AC-007 the row shape and
a11y floor the UX brief already fixed for it.

## Interaction quality

This project's signed-off design records `hasSurface: false` and `# no items — no public API surface, no
rendered UI surface` (`../_design.md:45-47`), and its `## Anti-patterns` section is `N/A — no public
surface`. That is not an exemption from RFC §6.7: the same file names the DT-1 … DT-10 audit table as
artefact 4 of this project's real user-facing surface (`:29`) and states that its bar is "a prose/schema
bar", located in the UX brief. So the composition family below is taken from the **UX brief's**
`IQ-1 … IQ-8` and `AC-UX-09` / `AC-UX-10` (`../_decomposition.md:145-232`, `:266-273`), which the design
file defers to, rather than invented here.

Every invariant that applies is carried by an AC row in the table above. Nothing in this section is a new
requirement; it is the map from invariant to id, so a reviewer can check that none was dropped.

**State family**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the row answers *which tension, whose, settled how* on first load; links carry evidence, never one of the required cells | AC-007 (self-contained row), AC-003 (the link is the *evidence*, not the substance) | Tier 2: a row is read in isolation, cover the rest of the table, and must still be actionable |
| **Non-occlusion** (IQ-2) — a summary must not replace a row; a resolution recorded elsewhere must not be laundered into a pass | AC-001 (ten rows, summary above the table only), AC-003 (cross-reference recorded, outcome unmoved) | Tier 2: row count against the charter's ten; the `checked-documentation-surface`/DT-8 instance checked by name |
| **Preserved position** (IQ-3) — nothing this story lands moves an anchor someone else already cited | AC-006 (`project.md` append-only, zero `-` lines), AC-004 (no sibling `_design.md` edited, so no cited `file:line` in the corpus shifts) | Tier 1: `git diff -U0 … \| rg '^-[^-]'` empty on `project.md`; `git diff --name-only` shows no sibling design file |
| **Reversibility** (IQ-4) — a wrong finding is corrected by superseding, never by editing the thing it describes | AC-004 (a gap is routed, never filled; no ADR as a closeout by-product) | Tier 2 against `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; Tier 1 `redkiln validate --kb` |
| **Reachable without prior knowledge** (IQ-5, the keyboard-reachability analogue) — one hop, link text naming the destination | AC-006 | Tier 2: the walk is performed from `project.md`, not asserted |
| **Preserved selection** (IQ-6) — a prior citation keeps meaning what it meant | AC-003 (rows cite the owning file rather than restating it), AC-005 (every observation names its tree) | Tier 2: each cited `file:line` still says what the row claims on the merged sha |
| **Every state legible at the moment of reading** (IQ-8) — no state by presence, absence, ordering or omission | AC-004 (three words spelled out), AC-007 (a11y floor) | Tier 1 regex over the outcome column; Tier 2 for the empty-cell-as-"not checked" failure |

**Composition family** (from `../_design.md:29` deferring to `../_decomposition.md`, "Design-system
primitives — compose these, do not hand-roll")

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the record is a composed table with a provenance header, not ten prose sentences or a bare bullet list; every cell carries real composed presentation in the corpus's own notation | AC-007, AC-005 (the header) | Tier 2: the record is read beside `../../initiative.md:483-494` and `../project.md`'s risk table — it must look like a member of the same corpus |
| **Composition and placement** — the record lives in this story's folder and is reached through `project.md`'s `## Companions`, the corpus's real composition root for documents | AC-006 | Tier 1 `rg` for the bullet + Tier 2 one-hop walk |
| **Transience: persistent, never revealed or opened on demand** — no `<details>`, no fold, no tab, no collapsed panel. DT-7's own tension is whether hidden panels are inside the checked surface (`../../initiative.md:483-494`); a record *about* that tension must not itself hide content | AC-007 | Tier 1: `rg '<details>\|<summary>'` returns nothing; Tier 2 confirms no fold-shaped prose ("expand for the full reasoning") |
| **Density budget, with its real numbers** — 1 provenance header ≤ 12 lines; exactly 10 body rows; exactly 7 columns; ≤ 60 words per note cell; at most 1 further `## Routed gaps` section, ≤ 120 words per routed gap; no nested table, no second data table | AC-007 | Tier 1: row count, column count, word count per note cell. The budget exists because U2's stated goal is one pass — a note that grows into an essay reproduces the discovery corpus U2 opened this record to avoid |
| **Hierarchy** — provenance header first (it scopes everything below it), then the ten-row table, then routed gaps if any exist. Nothing above the header; no appendix below the gaps | AC-005, AC-007 | Tier 2 read order |
| **Named anti-patterns** — (a) inventing a screen, token, legend or key for a text corpus that has none (`../_design.md:20-45`); (b) a summary sentence standing in for rows (IQ-2); (c) an outcome carried by tick, emoji or an empty cell (AC-UX-09); (d) "same as above" (AC-UX-10); (e) resolving a tension while auditing it (`../project.md:219-220`); (f) citing dossier item *n* as if it were DT-*n* | AC-007 (a, c, d), AC-001 (b), AC-004 (e), AC-003 (f) | Tier 2 against the checklist; Tier 1 regex for (c) and (d) |

An unstyled render satisfies every path-resolves and row-count assertion perfectly. AC-007 is what makes
that fail: a ten-row table with correct citations, an empty outcome column and a `✅` where a word should
be passes every Tier 1 check in this spec and is a defect.

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | `merge-forward-baseline` has not landed, or `.bklg/docs-that-teach/durable-audience-closeout/_baseline.md` does not exist / carries no sha when the audit is about to be taken | **Stop.** Do not audit this worktree's pre-merge copy and do not write a placeholder sha. The story is `blocked_by: [HS-S0172]` (`story.md`) and ordering 1 forbids observing anything against the pre-merge tree (`../_storymap.md:144-147`). Report the block; the human sequences it. |
| EC-002 | An owning project's `_design.md` is absent, renamed or moved in the merged tree | The row's outcome is `gap`, its note states *the file the ownership map names does not exist at this sha*, and the route names the owning project. It is never inferred as `resolved` from the decomposition's ownership table, which records intent, not the tree. |
| EC-003 | A cited `file:line` no longer lands on the text it claims, because a sibling story reflowed the file after this audit was drafted | Re-take the citation against the merged sha and re-record it; additionally cite the nearest stable `##`/`###` heading beside the line number, so the row degrades to "findable" rather than "wrong" if the file moves again. Do **not** edit the sibling file to restore the line. |
| EC-004 | DT-9's persona choice has not been appended to `comprehension-evidence/_design.md` at audit time | Outcome is `gap`, not `recorded deferral` — a story that has not shipped is not a recorded deferral. The note names `comprehension-evidence/dt9-and-fixed-protocol` as the route and flags that DoD-5 rests on it (`../../initiative.md:413-468`). Read the file; a story marked done whose append is absent is still a gap. |
| EC-005 | A gap plainly wants an ADR, or a sibling's resolution is plainly wrong | Record and route in prose. Writing or superseding a decision atom here is forbidden twice over: accepted atoms are immutable and supersession is deliberate (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`), and an ADR authored as a closeout by-product is the practice this corpus exists to refuse (`../project.md`, risk table, "A reconciliation decision that really wants an ADR"). |
| EC-006 | The merge left an owning `_design.md` conflicted or modified by the sibling branch | Record the conflict in the row's note with the sha, and cite `_baseline.md`'s conflict record rather than re-adjudicating it — `merge-forward-baseline` owns the conflict statement (`merge-forward-baseline/spec.md:92`). |
| EC-007 | A slice-mate has already appended to `project.md`'s `## Companions` and the two appends collide | Both bullets land; neither replaces the other. The slice is implemented in one context (`../_storymap.md:50-52`), so the collision is a same-file ordering question, not a merge conflict — append below, never reflow the list. |
| EC-008 | The audit finds all ten tensions resolved and the record looks trivially clean | That is a permitted outcome and must not be dressed up, but it is also the highest-risk one: re-run AC-003's follow-every-citation pass before accepting it. Ten `resolved` rows with unfollowed citations is precisely the wrong implementation `discover.md:53-55` names. |

## Non-functional

| id | Requirement | Why, and how it is judged |
| --- | --- | --- |
| NF-001 | **One-pass reviewability.** U2 reaches a per-row verdict without opening the discovery corpus; only the owning `_design.md` citations are followed. | The reader table's stated goal (`../_decomposition.md`, `## UX brief`). Judged by the density budget in AC-007 and by IQ-1's falsifier: a note cell reading "see `interaction-patterns.md`" for the substance is a failure. |
| NF-002 | **Durability against a moving tree.** The record stays interpretable after the branch advances: it names a sha, not "the current tree", and never says "today" or "the latest". | AC-012's framing and IQ-4/AC-UX-12 (`../_decomposition.md:278-282`). Judged by `rg` for relative time words. |
| NF-003 | **Zero tooling footprint.** No new script, lint, xtask step, CI job or dependency; no file under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `.kb/` in this diff. | `../_storymap.md:21-23`; AC-TB-02 forbids invented tooling (`../_decomposition.md:827-830`); AC-TB-08 records that no new Rust coverage is implied (`:846-851`). Judged by `git diff --name-only`. |
| NF-004 | **Plain-text legibility.** The record reads identically through `cat`, through `git diff`, and through a markdown renderer — no construct whose meaning depends on rendering. | AC-UX-09's a11y floor (`../_decomposition.md:266-269`), and the practical fact that U2 will meet this file in a diff before they meet it rendered. |
| NF-005 | **Correction by supersession.** If a row is later found wrong, the route is a new statement naming both shas — never a silent edit of the row. | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; AC-UX-12. Stated in the record's own header so the next reader inherits the rule. |
| NF-006 | **No sibling latency.** Auditing must not require any sibling project to do work. Every input is already on disk at the merged sha. | The project depends on HS-P0024 and transitively on the rest (`../project.md`, `## Dependencies`); if an input is missing, that is EC-002 or EC-004 — a `gap` row — not a wait. |

## Implementation notes (non-prescriptive)

These are notes, not instructions. Where one conflicts with an AC above, the AC wins.

**The column set that satisfies AC-UX-10.** Seven columns, in this order, is one shape that works and is
worth starting from:

`DT` | `Tension (short)` | `Owning project` | `Resolution or deferral — file:line` | `Outcome` |
`Observed by / against` | `Note or route`

`Observed by / against` folds the observer and the short sha into one cell (`ryan-britton @ 1a2b3c4`) so
every row is self-contained without a column that repeats the same value ten times in two places. The full
40-hex sha lives once, in the provenance header, and `_baseline.md` remains its source.

**A workable order of operations.** Read `_baseline.md` first and copy the sha *before* opening anything
else — an audit that starts by reading designs and adds the sha afterwards is how a pre-merge observation
gets a post-merge label. Then work the ownership map (`../../_decomposition.md:148-159`) project by project
rather than tension by tension: HS-P0022 answers four tensions from one file, so four rows are settled in
one read. Write the row's citation at the moment you find the text, not from memory.

**The four cross-checks worth doing explicitly**, because each has a live near-miss in this tree:

1. DT-8 — read `page-need-discipline/_design.md:240-288` for the outcome, and record
   `checked-documentation-surface/_design.md:267-269` in the note as a cross-reference only.
2. DT-9 — open `comprehension-evidence/_design.md` and look for the *chosen persona*, not for the word
   DT-9. The file claims to be DT-9's sole durable record (`:44`, `:103-109`); the claim is not the record.
3. DT-5 and DT-6 — both live inside HS-P0022's single `:170-262` block. Two rows, two citations, two
   outcomes; one shared block is not one shared verdict.
4. Any row you are about to source from `_discovery/distillation/interaction-patterns.md:461-465` — stop.
   Dossier item 1 is DT-7, item 2 is DT-8, item 3 is DT-2. The dossier is background; the charter is the id.

**Mounting is one line.** Append a single bullet to `project.md`'s `## Companions` (`:291-301`) in the same
form as the bullets already there, with link text naming the destination (IQ-5). Body only — the frontmatter
is the CLI's and a `PreToolUse` hook denies the edit (CLAUDE.md, "Where the work lives").

**What not to build.** No checker, no `rg`-based lint committed to the tree, no xtask step, no legend, no
status key, no colour convention, no second summary table. The Tier 1 commands in this spec are things a
reviewer *runs*, not things this story *adds*.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`, `## Testing brief` — its four-tier ladder, its AC-018 row ("Tier 2 —
content review… no automated reader exists for this table", `:878`) and AC-TB-01, which makes claiming
Tier 1 alone for AC-018 a defect (`:824-826`).

| tier | command / path | proves |
| --- | --- | --- |
| **2 — content review** (primary, and the only tier that can falsify AC-018) | A human read of `.bklg/docs-that-teach/durable-audience-closeout/design-tension-audit/_audit.md` against the UX brief's own checklist — IQ-1 … IQ-8 (`../_decomposition.md:145-232`), AC-UX-09 (`:266-269`), AC-UX-10 (`:270-273`) — following **every** cited `file:line` into its owning `_design.md` | AC-001, AC-002, AC-003, AC-004, AC-005, AC-006, AC-007. AC-TB-06 requires this checklist and forbids a second divergent one (`:840-842`) |
| **1 — static** (support only; never cited alone for AC-018) | `rg -c '^\| DT-' …/_audit.md` = 10; `rg -o 'DT-([1-9]\|10)' …/_audit.md \| sort -u` = 10 distinct ids | AC-001's completeness, mechanically |
| **1 — static** | `rg -n 'Conceptual bridge\|First encounter\|Checked prose\|Reach and placement' …/_audit.md` finds no match in an owner cell | AC-002 — the superseded descriptive labels are not being used as owners |
| **1 — static** | `test -f` over every distinct path cited in the evidence column (five sibling `_design.md` files) | AC-003's paths resolve at the merged sha. It does **not** prove the cited *line* says what the row claims — that is Tier 2's job and the reason AC-TB-01 exists |
| **1 — static** | Every outcome cell matches `^(resolved\|recorded deferral\|gap)$`; `rg -n '\|\s*\|' …/_audit.md` finds no empty cell; a non-ASCII scan finds no emoji, tick or glyph | AC-004's vocabulary and AC-007's a11y floor |
| **1 — static** | `rg -n '<merged-sha>' .bklg/docs-that-teach/durable-audience-closeout/_baseline.md …/_audit.md` matches in both; `git cat-file -t <merged-sha>` = `commit` | AC-005 — the named tree is the baseline's tree and it resolves (the "also tier 4" cell of AC-012's row, `../_decomposition.md`, tier mapping) |
| **1 — static** | `rg -n '_audit' .bklg/docs-that-teach/durable-audience-closeout/project.md` matches inside `## Companions`; `git diff -U0 -- .bklg/docs-that-teach/durable-audience-closeout/project.md \| rg '^-[^-]'` returns nothing | AC-006 — mounted, and mounted by appending (IQ-3's own falsifier form) |
| **1 — static** | `rg -n 'same as above\|ditto\|see row\|<details>\|<summary>' …/_audit.md` returns nothing | AC-007 — self-containment and the no-disclosure rule |
| **1 — static** | `git diff --name-only` contains no path under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/`, `.kb/`, or any sibling project's `_design.md` | NF-003 and AC-004's "audits, does not resolve" boundary |
| **3 — integration** | `redkiln validate --kb` (merge-gate step 1, `../_decomposition.md`, Notes) | Not evidence for AC-018. It is cited here for one thing only: its accepted-decision immutability check makes an accidental `.kb/decisions/` touch fail structurally, which is AC-004's last clause proved as a side effect |
| **3 — integration** | `redkiln doctor` | The six expected `template-drift` advisories and nothing new; this story adds no item and changes no template |
| **4 — e2e** | `cargo xtask ci` — **not claimed by this story.** It is `terminal-gate-run`'s (AC-016), run last on the tree that already carries this record (`../_storymap.md`) | Named here so no reviewer accepts a gate run as this story's evidence. AC-TB-03 forbids a gate run taken before the records are in the tree from counting (`:831-834`) |

**What this story adds to CI: nothing.** No job, no step, no script. The gate's Rust-grain steps continue
to function as a regression net proving `crates/`, `examples/`, `spec/` and `standards/` stayed untouched
(AC-TB-08, `:846-851`) — which is the only relationship this story has to them.

## Risks and coupling (PR-scoped)

| Risk | Why it is live in *this* PR | Mitigation, already an AC where possible |
| --- | --- | --- |
| **The auditor resolves instead of auditing.** A gap is one sentence away from being filled, and the fill would look helpful. | Every input file is open in front of the implementer, five of them signed off by a human. | AC-004's "then stops"; `../project.md:219-220`; Tier 1 `git diff --name-only` showing no sibling `_design.md`. The PR boundary lists sibling designs as explicitly out. |
| **Cited line numbers go stale.** Sibling projects are still implementing; a `_design.md` reflowed after the audit is drafted moves every line cited into it. | The merge lands mid-initiative and slice-mates run concurrently. | EC-003: re-take citations against the merged sha and cite the nearest stable heading beside the line. Take the citations *after* the merge, never before. |
| **`project.md`'s `## Companions` is touched by three stories in one slice.** `reconciliation-ledger` and `charter-open-question-disposition` mount there too (`../_storymap.md:50-52`). | Same file, same section, same context. | EC-007: three bullets, appended, no reflow. AC-006's zero-`-`-lines check catches a reflow that ate a sibling's bullet. |
| **DT-9's outcome depends on a story landing.** `comprehension-evidence/dt9-and-fixed-protocol` may or may not have appended the chosen persona by audit time. | It is the single row whose verdict is genuinely time-dependent (`spec.md:27`, `:208` of that story). | EC-004: read the file, not the story status; `gap` if absent, with DoD-5 flagged. |
| **The charter's descriptive owner labels are read as authoritative.** They are the first thing anyone sees in the tension table. | `../../initiative.md:483-494` still carries them, correctly — it predates the id assignment. | AC-002 makes them a checkable defect; the Context pack states the map inline so no lookup is needed. |
| **Dossier numbering is mistaken for DT numbering.** Item 3 is DT-2. | The dossier is the most detailed source and the most tempting to cite. | Called out in the Context pack and in Implementation notes cross-check 4; AC-003's Tier 2 pass catches a citation that lands on the wrong tension. |
| **A clean all-`resolved` audit is accepted without following citations.** | It is the cheapest outcome and looks like success. | EC-008; AC-003's verification is defined as *following* the citation, not checking it exists. |
| **Coupling to `merge-forward-baseline` is the only hard edge.** If HS-S0172 slips, this story cannot start. | `blocked_by: [HS-S0172]`. | EC-001: stop and report; do not audit the pre-merge tree. |

**Coupling out.** None to `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `.kb/`. Coupling *in* is
read-only against five sibling `_design.md` files and `_baseline.md`. This PR cannot break the build,
because it changes nothing the build reads.

## Dependencies

**Blocks on**

- `merge-forward-baseline` (HS-S0172) — the only dependency, recorded as a real `blocked_by` link on the
  item (`story.md`). It produces the merged commit and writes
  `.bklg/docs-that-teach/durable-audience-closeout/_baseline.md`, which is where this story's provenance
  header copies its sha from (`merge-forward-baseline/spec.md:48`, `:98`, `:237`). Ordering 1 of the story
  map — "merge before everything; nothing after this point may be observed against this worktree's
  pre-merge copy" (`../_storymap.md:144-147`) — is the reason this edge exists, and EC-001 is what happens
  if it is ignored.

**Slice-mates** (`closeout-adjudication`, implemented in one context, mounted as one integrated surface —
`../_storymap.md:50-52`, `:66-69`)

- `reconciliation-ledger` and `charter-open-question-disposition`. Neither is a dependency of this story:
  `design-tension-audit` runs **in parallel with either** (`../_storymap.md`, "Merge order", milestone 2).
  The shared surface is `project.md`'s `## Companions`, which is EC-007's concern and not an ordering one.

**Unlocks**

- No story declares `design-tension-audit` in its `depends_on` (`../_storymap.md`, slice table), so it
  unlocks nothing formally. It is nonetheless a **precondition on the tree** for `terminal-gate-run`:
  AC-016's gate run must be taken on "the exact tree that already carries … the DT audit"
  (`../_storymap.md`, `dod-reobservation` row), and AC-TB-03 makes a gate run taken before the records
  exist inadmissible (`../_decomposition.md:831-834`). Landing late does not block another story; it
  invalidates the terminal one's evidence.

## Anchors (progressive disclosure)

Everything load-bearing that is *not* in the Context pack. Open each at the moment named, not before.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/initiative.md` | `:483-494` is the canonical DT-1 … DT-10 table — the only authority for the ten ids and their tension text. `:664-666` is the exit criterion whose exact wording ("in its owning project's `_design.md`", "explicitly and reasonably deferred", "none is left silently unowned") this audit discharges. `:413-468` enumerates the fifteen DoD scenarios, needed only to name DoD-4/5/13 in a gap row. | Before writing the first row; again when a gap row must name the scenario resting on it. | AC-001, AC-004 |
| `.bklg/docs-that-teach/_decomposition.md` | `:148-159` is the binding DT → project-id ownership map; `:29-32` and `:52-58` explain why the charter's six descriptive owners became five real projects and why HS-P0022 carries four tensions. Getting an owner wrong routes a gap to a team that cannot close it. | Before writing the owner column; again if any owner looks like it should be a sixth project. | AC-002 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The project's three briefs. `:70` fixes the three-state vocabulary; `:145-232` is IQ-1 … IQ-8, the checklist Tier 2 review is *required* to use; `:266-273` is AC-UX-09/AC-UX-10, the a11y floor and the self-contained-row rule; `:824-826` and `:878` are why Tier 1 may never be claimed alone here. | Before drafting the table shape (`:266-273`), and again at review time (`:145-232`). | AC-004, AC-007, and the verification column of every AC |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The signed-off design. `:45-47` records `hasSurface: false` and `# no items`; `:29` names this audit table as artefact 4 and defers its bar to the UX brief; `:91-94` is the dated human sign-off. It is why there is no screen, no token and no `## Signatures` obligation — and why inventing one is the anti-pattern. | Once, before deciding anything about presentation, if there is any temptation to design a surface. | AC-007 |
| `.bklg/docs-that-teach/application-author-path/_design.md` | Where DT-1 (`:81-120`), DT-4 (`:122-168`) and DT-5 + DT-6 (`:170-262`) are resolved or deferred. Four of the ten rows are settled by one read of this file. | When filling rows DT-1, DT-4, DT-5, DT-6. | AC-003 |
| `.bklg/docs-that-teach/page-need-discipline/_design.md` | Where DT-2 (`:168-212`), DT-3 (`:214-238`) and DT-8 (`:240-288`) are resolved or deferred. `:240-288` is DT-8's *only* authoritative locus, which is what makes the `checked-documentation-surface` cross-reference a note rather than a pass. | When filling rows DT-2, DT-3, DT-8. | AC-003 |
| `.bklg/docs-that-teach/checked-documentation-surface/_design.md` | DT-7's resolution (`:200`), and `:267-269`, which states what its answer explicitly does *not* decide about DT-8 and that its hidden-marker ban makes DT-8 moot only inside the pinned tree. The live instance of "discussed in a non-owning design". | When filling row DT-7, and again when filling DT-8's note. | AC-003 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | DT-9's claimed sole durable record (`:44`, `:103-109`). The claim is checkable and, at spec time, the chosen persona is not yet stated in it. | When filling row DT-9 — read for the *persona*, not for the string "DT-9". | AC-003, AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/dt9-and-fixed-protocol/spec.md` | `:27` and `:208` identify the story that appends DT-9's persona choice, i.e. the route a DT-9 `gap` row must name. | Only if DT-9 reads as a gap. | AC-004 |
| `.bklg/docs-that-teach/reach-and-adapter-path/_design.md` | DT-10's resolution (`:87`, `:475`) — the last of the ten. | When filling row DT-10. | AC-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/merge-forward-baseline/spec.md` | Defines `_baseline.md`'s contents and its rule that downstream records **cite** the file rather than re-deriving the sha from `git log` (`:48`, `:98`, `:126`, `:237`), and who owns the conflict record (`:92`). `_baseline.md` itself does not exist until that story lands. | First, before any observation is taken. | AC-005, EC-001, EC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | `:52` is this story's approved one-line slice; `:144-147` is ordering 1 (merge before everything); `:21-23` is the diff-surface boundary; `:99-101` is the no-`.kb/decisions/` rule; `:50-52` and `:66-69` are the slice and its rationale. | When a scope question arises — "may this PR touch X". | AC-004, AC-006, NF-003 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | `:207` is AC-018 verbatim; `:179-182` is DR-14 and the reason the written resolution is the only record there will ever be; `:219-220` is the Definition-of-Done clause forbidding re-opening a sibling's requirement; `:291-301` is the `## Companions` list this story mounts into. | At mount time, and whenever a gap tempts a fix. | AC-004, AC-006 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The corpus's supersede-never-edit governance. It is what makes "record and route the gap" the only legal move when a finding wants an ADR. | Only when a gap looks like it needs a decision written. | AC-004, NF-005 |
| `.kb/design/README.md` | Where a `_design.md` resolution is harvested to at closeout and what altitude it must keep. Relevant only to confirm that *this* story harvests nothing — the promotion belongs to `staged-audience-payload` / `audience-ingest-wave`. | Only if tempted to write a `.kb/design/` atom from a finding. | AC-004 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `:461-465` is where the tensions originate, in an ordered list whose numbering is **not** DT numbering (item 1 = DT-7, item 2 = DT-8, item 3 = DT-2). Background only; useful for understanding a tension, never for citing one. | Only when a tension's *meaning* is unclear — and never as a row's evidence. | AC-003 |
| `.redkiln/config.yaml` | `:62-67` is `require_ledger: true`, which is why `_ledger.md` is in this PR; `:75-83` is `design.capture`'s deliberate absence, which is why no perceptual gate would catch an unmounted record; `:60` is the terminal `e2e` grain this story does not claim. | When asked why a ledger is required, or why mounting must be checked by hand. | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the seven the first pass declared** — AC-001 … AC-007. None was added and none
   dropped. AC-018 decomposed cleanly across them (see the Coverage note under the acceptance criteria) and
   the ledger enumerates the same seven ids.
2. **How a `hasSurface: false` project can carry composition invariants.** Resolved in favour of honesty:
   the project's `_design.md` has no `## Anti-patterns` to inherit (`../_design.md`, "`N/A — no public
   surface`"), but it *names this audit table as artefact 4* and defers its bar to the UX brief (`:29`). The
   composition family is therefore taken from `../_decomposition.md`'s IQ-1 … IQ-8 and AC-UX-09/AC-UX-10,
   not invented, and its "density budget" is expressed in the units a text corpus actually has — rows,
   columns, header lines, words per cell. Inventing a token, a legend or a colour convention to satisfy the
   section's shape would have contradicted the signed-off design.
3. **Whether the outcome for a tension resolved in a non-owning project is `resolved`.** No. The exit
   criterion says *its owning project's* `_design.md` (`../../initiative.md:664-666`), and the live instance
   (`checked-documentation-surface/_design.md:267-269` on DT-8) explicitly disclaims deciding it. Recorded
   as a cross-reference in the note; the outcome still reflects the owning file. Written into AC-003.
4. **Whether DT-9 may be marked `resolved` on the strength of its owning story being complete.** No —
   EC-004. `comprehension-evidence/_design.md` claims to be DT-9's sole durable record but does not, at spec
   time, state the chosen persona. The audit reads files, not statuses.
5. **Whether the audit's ten rows may be collapsed where an owner is shared.** No — AC-001. HS-P0022 owns
   four tensions and they are four rows with four citations and four outcomes; one shared `_design.md`
   block (`:170-262` covers DT-5 and DT-6 together) is not one shared verdict.
6. **Whether a `git`-based or `rg`-based checker should be committed for this table.** No. The testing
   brief records that no automated reader exists (`../_decomposition.md:878`), AC-TB-01 forbids claiming
   Tier 1 alone (`:824-826`), and AC-TB-02 forbids invented tooling (`:827-830`). The Tier 1 commands in
   this spec are things a reviewer runs; none is added to the tree or to CI.
7. **Where the merged sha comes from.** `_baseline.md`, cited — not re-derived from `git log`, per
   `merge-forward-baseline/spec.md:98`, `:126`. That file does not exist until HS-S0172 lands, which is
   exactly EC-001's trigger and why it is cited through its owning spec in the anchors table rather than by
   its own path.
8. **Whether this story advances a DoD scenario.** No — the audit is an exit criterion, not one of the
   fifteen. It is recorded in the Integration contract as advancing none, with the note that a `gap` row
   against DT-6, DT-7 or DT-9 is an escalation because DoD-4, DoD-13 and DoD-5 respectively rest on them.
