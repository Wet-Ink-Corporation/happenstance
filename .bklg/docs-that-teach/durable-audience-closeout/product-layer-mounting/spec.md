---
item: HS-S0179
stage: spec
created: 2026-08-17T13:16:29.746Z
updated: 2026-08-17T13:16:29.746Z
template_sig: 87bbf1d0
rendered_sig: 24dd8697
---

# Spec — Mount every promoted atom at all four mount points

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` — DoD-15 at `## Definition of Done` ("linked from this initiative's closeout"), and `## Referenced personas & journeys`, which today cites the discovery distillation because the product layer is empty |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` — the merge-forward rule (`:89-103`), the parallel-and-reconcile decision (`:116-123`), HS-P0021's carried `.kb/playbooks/` payload (`:106-114`), the terminal flag (`:306-308`) |
| Project | `.bklg/docs-that-teach/durable-audience-closeout/project.md` — DR-9, DR-10, AC-010, AC-011, and the risk row *"Documentation has no functional home in the domain map"* |
| This spec | `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/spec.md` |
| Key briefs | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` — `## Architecture brief` §2 (the composition root and the seven-item mount list, of which four are this story's), AC-A04, AC-A05, AC-A09, AC-A10, §5 ordering 3 ("validation before the links"); `## UX brief` IQ-2, IQ-3, IQ-5, IQ-8 and AC-UX-06, AC-UX-07, AC-UX-08, AC-UX-09; `## Testing brief` Tier 3 (the mount-point walk) and merge-gate steps 1–4 |
| Signed-off design | `.bklg/docs-that-teach/durable-audience-closeout/_design.md` — approved 2026-08-17, `hasSurface: false`, `## Items` empty by decision. Binding on this story by what it *excludes*: there is no signature, visibility, doctest or screen obligation here to invent, and none may be introduced |
| Grounding | `.bklg/docs-that-teach/durable-audience-closeout/_grounding.md` — `## Reconciliation counterpart does not exist in this tree`, and the append-only map rule at `:88` |
| Roadmap pointer | `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md`, `## Merge order` §3 — this story is **last** in `product-layer-promotion`, behind `staged-audience-payload` and `audience-ingest-wave`, and it gates `dod-scenario-ledger` (HS-S0180) and `terminal-gate-run` (HS-S0182) |

## One-line PR slice

Mount every landed atom at all four points — an appended `##` section in
`.kb/maps/domain-map.md` with no deletion line anywhere in the diff, reciprocal
`related` / `depends_on` edges, `links.kb` on HS-P0025 via `redkiln record-links --atom`,
and a `## Knowledge Harvest` row — and record the two-hop reachability walk from
`.kb/README.md`.

## Executive summary

This PR lands **reachability, not content**. Every atom this slice promotes already exists
when this story starts: `staged-audience-payload` wrote the source documents and
`audience-ingest-wave` ran the one composition root that may write `.kb/`. What is still
missing at that point is the only thing that makes a promoted atom differ from a file
nobody will ever open — the four registrations the architecture brief calls the mount
points, and without which "a promoted atom is a component rendered into no tree"
(`_decomposition.md`, `## Architecture brief`, §2).

The delta over the project charter is four shape decisions the briefs deliberately left to
the implementer or that only became visible when the corpus was actually read. This spec
fixes all four rather than handing them down:

1. **What the appended domain section is called and what it covers.** §6 of the
   architecture brief calls the *name* a judgement call. Fixed here as **one section
   covering documentation and its audience together** — because the same wave lands
   HS-P0021's page-need-discipline playbook alongside the persona and journey atoms, so a
   section named after personas alone is already too narrow on the day it is written, and
   the map's own rule is that "a domain is a subject area, not a wave, and sections should
   outlive the ingest that first populated them" (`.kb/maps/domain-map.md:144-150`).
2. **Who appends the map section, and what this story owns when the ingest already did.**
   The domain map is itself an atom whose `summary` says it is "Updated by the Maps phase
   of every kb-ingest wave" (`.kb/maps/domain-map.md:7-12`). So the wave writes; this story
   owns the *shape* of what it wrote, the append-only property of the diff, and the repair
   of anything the Maps phase left incomplete — inside the same wave, on atoms that are not
   decision atoms. Mounting is completion, never authoring.
3. **The fourth mount point cannot be written by this story, and that is structural.**
   `closure.md` does not exist in this project's folder and will not be rendered until the
   project reaches its `closeout` stage — the only project stage carrying `harvest_kb: true`
   (`.redkiln/processes/project.yaml:75-79`). Decided: this story authors the Knowledge
   Harvest rows, in the template's exact columns, in a companion under its own folder, so
   the closeout transcribes rather than re-derives them. Claiming to have written a row
   into a file that does not exist would be the more comfortable lie.
4. **Hop 1 of the two-hop reachability walk has no link today.** `.kb/README.md` carries no
   markdown link to `.kb/maps/domain-map.md` — checked, not assumed — so IQ-5's falsifier
   ("an atom findable only by `ls .kb/product/`") is unavoidable unless one appended bullet
   is added there. Decided: add it, as an addition only, and declare `.kb/README.md` in the
   PR boundary rather than let a later reviewer discover an undeclared touch.

Everything else this PR does is registration under a strict mechanical rule: **no deletion
line anywhere in either map file**, which is the one property that keeps "append-only" a
thing `git diff` can check rather than a thing a reviewer has to believe.

## Context pack

**Mounting *is* the deliverable, and the bar is four references per atom.** The
architecture brief states the tripwire in one sentence: for each promoted atom, name the
map line, the `related` edge, the `links.kb` entry and the `Knowledge Harvest` row that
carry it — "four references, or it is not mounted" (`_decomposition.md`,
`## Architecture brief`, §2, and AC-A04). The failure this story exists to prevent is not a
malformed atom; `redkiln validate --kb` already catches those. It is a **well-formed atom
that is reachable from nothing**: it validates, and the next initiative never finds it,
which is the same outcome as never having promoted it (§2, opening paragraph). That is why
this story is a `capability` rather than bookkeeping, and why it is the last story in the
slice rather than a tidy-up after it.

**This story does not author a single atom, and the distinction is mechanical.**
`/redkiln:kb-ingest` is the only writer of `.kb/product/`, reachable only from
`.kb/_intake/`, and hand-writing the directory layout of the process without the process
was reverted once already at `0269720` (`.kb/_intake/README.md:3-5`; CLAUDE.md, "Where the
work lives"; AC-A01). By the time this story runs, `.kb/_intake/` is **cleared** — the
successful wave removed the staged sources in the same commit that added the atoms
(`.kb/_intake/README.md:13-19`). What remains permitted is completing the *registrations*:
frontmatter edges on atoms the wave already landed, and appended lines in the two map
files. That is not the hand-authoring AC-A01 forbids, which is about an atom **appearing**
under `.kb/` with no staged source in the wave's history. And it is not the immutability
`redkiln validate --kb` enforces either: `.kb/README.md`'s Rules section binds immutability
to a `decision` atom at `status: accepted`, and persona atoms are `concept`, journey atoms
are `playbook`. **If a repair would require editing `.kb/decisions/`, stop** — AC-A09
forbids it outright and T5 makes an accidental touch fail merge-gate step 1 structurally.

**Ordering 3 is load-bearing and it is this story's ordering.** `record-links --atom`
records ids that exist and validate; recording first and fixing afterwards leaves an item
pointing at an atom that changed shape (`_decomposition.md`, `## Architecture brief`, §5,
ordering 3). So the sequence inside this story is fixed: append the map section and wire
the edges, then `redkiln validate --kb`, then `redkiln record-links HS-P0025 --atom`, then
the Tier 3 reachability grain. And the re-run of `validate --kb` is **this story's own
obligation, not a second reading of the ingest wave's**: `_storymap.md`'s `## Coverage`
names the seam directly — the wave proves it green on the atoms as landed, this story
re-proves it after the reciprocal edges and the map section are added, "because those
change the frontmatter the validator reads."

**Append-only is strict, mechanical, and it wins against tidiness.** IQ-3's falsifier is
unambiguous: `git diff .kb/maps/domain-map.md` showing a `-` line **anywhere** outside the
appended region (`_decomposition.md`, `## UX brief`, IQ-3; AC-UX-06; AC-A05; project
AC-010). Two consequences the implementer will meet immediately and must not resolve by
feel. First, `.kb/maps/domain-map.md:35` reads *"This is the map's first wave. One domain
exists so far."* — a sentence this story makes arithmetically false. It is **not corrected
here**: the appended section carries its own orientation, and the stale preamble is
recorded as a routed gap in this story's ledger, because the cost of one stale sentence is
smaller than the cost of "append-only" ceasing to be a property `git diff` can decide.
Second, `last_reviewed: 2026-08-10` in the map atom's frontmatter is **not bumped** — that
is a deletion plus an addition. Adding an id to a `related:` list is an added line and is
fine; rewriting a scalar is not. If the wave's Maps phase itself rewrote the preamble or
the date, that is a deviation to **revert before the wave commits**, not a fait accompli to
accept.

**The domain map's format is a primitive to compose, not a layout to invent.** Append a
`##` section; group entries by kind under bold labels (**Reference**, **Concepts**,
**Governance**, **Playbooks**, **Open questions**); cite each atom's id next to its link;
keep the orientation to one sentence, "because the atom itself is the source of truth, not
this map" (`.kb/maps/domain-map.md:144-150`, with the live format visible at `:105-142`).
The section's *name* is this spec's to fix and is fixed above at decision 1: one subject
area covering documentation and the audience it teaches, sized to hold the persona atoms,
the journey atoms, HS-P0021's carried page-need-discipline playbook and any `open_question`
atom the disposition staged. Inventing a second map format, or a sub-directory under
`.kb/product/`, is on the UX brief's forbidden list (`## UX brief`, "Hand-rolling,
explicitly forbidden here").

**Links in this corpus are two-way, and the reciprocity is the mount.** The exemplar map
atom carries `related: [kb-map-open-questions-index-001, kb-map-decision-001]` in its own
frontmatter (`.kb/maps/domain-map.md:14-16`), and `.kb/README.md`'s Rules make dangling ids
a validation failure: "every `depends_on` / `related` / `supersedes` / `superseded_by` id
must resolve to a real atom". So a journey atom names its persona and **the persona names
its journeys back** (`_decomposition.md`, `## Architecture brief`, §2, mount point 3). A
one-way edge is not a lighter version of the mount; it is the half that leaves the persona
unreachable from the journey a charter author actually starts at.

**The carried playbook is payload: mount it, never edit it.** HS-P0021's page-need
discipline atom rides this wave and its text is HS-P0021's — "this project's diff may add
it via ingest and must not rewrite it" (AC-A10; `.bklg/docs-that-teach/_decomposition.md:106-114`).
The seam is vehicle versus content, and it resolves cleanly for edges too: where a
reciprocal link between that playbook and this project's atoms is wanted, **write the edge
on this project's atom pointing at the playbook**, and leave the playbook's own frontmatter
alone. It still gets its map line, its `links.kb` entry and its Knowledge Harvest row —
three of the four — and the fourth is satisfied from the other end.

**`links.kb` is written by exactly one command, on the project, not on this story.**
`redkiln record-links HS-P0025 --atom <ids>` is the single writer of that item field
(`.redkiln/templates/_retrospective.md:59`; CLAUDE.md, "The CLI is the only writer of an
item's system frontmatter"), a `PreToolUse` hook denies a hand edit, and the flag takes a
**comma-separated list** because a repeated string flag is last-wins under strict
`parseArgs` — recording one id per invocation is how four atoms become one recorded atom.
The target is **HS-P0025**, the project, because AC-A04(c) says so and because the harvest
is the project's, not this story's; this story separately records its own work commit with
`redkiln record-links HS-S0179 --sha` (`require_commit_provenance: true`,
`.redkiln/config.yaml:73`).

**The atom ids are read off the tree, never predicted.** The ingest mints ids; nothing in
this spec fixes them, and a mount that cites an id this spec guessed is a mount that will
dangle. Read the landed frontmatter, then write the same ids into the map lines, the
`related` edges, the `--atom` list and the harvest rows, so all four references name the
same string.

**The fourth mount point is a handoff, and saying so is the honest move.** `closure.md` is
produced by the project's `closeout` stage (`.redkiln/processes/project.yaml:75-79`,
`produces: [closure.md]`, `template: closure`), which is the only project stage carrying
`harvest_kb: true` — the mechanical reason promotion cannot happen earlier, and equally the
reason the harvest table cannot be written now. This story authors the rows in the exact
columns `.redkiln/templates/closure.md` declares (`| KB id | Kind | Summary |`) in a
companion under its own folder, and **does not advance the project**: every stage
transition belongs to the orchestrating command, not to an implementer.

**Reachability is observed from the front door, and the front door has a gap.** IQ-5 asks
that a reader who has never heard of this initiative reaches every atom starting from
`.kb/README.md` in at most two hops, with link text naming the destination at each hop, and
AC-UX-08 asks for that walk to be **recorded** — starting file, hops named, every atom
reached. Checked against the tree: `.kb/README.md` names the `product/` layer and the
`maps/` directory in prose but carries no markdown link to `.kb/maps/domain-map.md`, so the
only route left is `ls .kb/product/` — which is IQ-5's falsifier verbatim. Decided: this
story appends **one bullet** to `.kb/README.md` naming `maps/domain-map.md` as the
subject-matter index, an addition with no `-` line, and declares that file in its PR
boundary. Mounting a capability into the composition root is not scope drift; touching it
silently would be.

**No ADR governs this surface and looking for one is the error.** All sixteen accepted
decision atoms govern the Rust contract, and the charter says outright that "none of the
seventeen decisions concerns documentation" (`.bklg/docs-that-teach/initiative.md:524-525`;
`_decomposition.md`, `## Architecture brief`, §3). What binds instead is
`.kb/governance/rewrite-the-referent-never-the-reasoning.md` — a promoted atom later found
wrong is corrected by a new atom that supersedes it, never by an edit — the single
write-path rule, and `harvest_kb: true` on `closeout` only. Citing an ADR here would be
manufacturing authority.

**The persona-journey slice this realizes.** U1, the next initiative's charter author, is
the reader this whole story exists for: the four mount points are precisely the surfaces U1
travels — the map they browse, the reciprocal edge they follow from a journey to its
persona, the `links.kb` that ties the audience to the initiative that produced it, and the
closure table that says what was harvested. U2, the closeout reviewer, reads the same four
as a checklist and must be able to find the one atom that is missing one of them. Neither
is a documented persona: the three personas are the *subject* of this material and never
its audience (`_decomposition.md`, `## UX brief`, "Who this is actually for").

## Integration contract

- **Archetype**: `capability` — a user-observable slice, where the user is U1/U2 and the
  observable is an atom that can actually be *reached* from the corpus's front door rather
  than found by listing a directory.
- **Slice / milestone**: `product-layer-promotion`. Slice-mates, implemented in the same
  context and mounted as one integrated surface, in this order:
  `staged-audience-payload` (HS-S0177) → `audience-ingest-wave` (HS-S0178) →
  **`product-layer-mounting` (this story)** (`_storymap.md`, `## Merge order`, §3). The
  whole wave lands as **one commit on its own branch** and is reviewed by merging
  (`.kb/_intake/README.md:13-19`; IQ-4), so this story is never delivered alone — and it is
  the story that makes the other two count as promotion rather than filing.
- **Mount point**: **`.kb/maps/domain-map.md`** — the index the corpus renders through
  (`_decomposition.md`, `## Architecture brief`, §2, mount point 1), and the file this story
  actually changes. The composition root that produced the atoms (`/redkiln:kb-ingest`)
  already ran; the render path is the map, and the mount is complete only when all four
  registrations below carry the same atom ids:
  1. `.kb/maps/domain-map.md` — one appended `##` section, entries grouped by kind, each
     atom's id beside its link, one orientation sentence.
  2. Reciprocal `related` / `depends_on` edges between the landed atoms (and, conditionally,
     `.kb/maps/open-questions-index.md` for any `open_question` atom the wave landed).
  3. `links.kb` on **HS-P0025**, written only by `redkiln record-links HS-P0025 --atom`.
  4. The `## Knowledge Harvest` rows, authored here in the closure template's columns and
     transcribed into `closure.md` when the project reaches `closeout`.
- **Wires into**:
  - `.kb/maps/domain-map.md:105-142` (the live section format) and `:144-150` (the
    append-only rule) — the primitive this story composes rather than invents.
  - `.kb/maps/open-questions-index.md:136-143` — the index-bullet shape: status word first,
    then the link, then the id, then one sentence.
  - `.kb/README.md` — the corpus front door and hop 1 of the recorded reachability walk;
    also the Rules section that makes a dangling `related` id a validation failure and
    scopes immutability to accepted `decision` atoms.
  - `.kb/_templates/atom.md` — the frontmatter vocabulary whose `related` / `depends_on`
    keys this story fills in; no key outside it is added.
  - `.kb/product/README.md:6-13` — the tier table, read to confirm the landed atoms carry
    `authority_tier: product` before they are registered anywhere.
  - `.redkiln/templates/closure.md` (`## Knowledge Harvest`, `| KB id | Kind | Summary |`)
    and `.redkiln/templates/_retrospective.md:59` (`redkiln record-links <id> --atom`).
  - `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md`
    (HS-S0178) — the upstream story whose landed atom ids, and whose Maps-phase output, this
    story reads and completes.
  - `.redkiln/config.yaml:48` (`reachability_static: cargo xtask lints && cargo xtask
    spec-trace`), `:40` (the story-grain affected gate), `:67` and `:73`.
- **Renders surfaces**: **none from `_design.md`.** This project's signed-off design records
  `hasSurface: false` with an empty `## Items` block — "no items — no public API surface, no
  rendered UI surface" — so there is no composition, density or chrome decision to inherit
  and none to contradict. The binding surface contract is the UX brief's: IQ-2 (the map
  indexes, it does not gate; supersession annotates rather than deletes), IQ-3 (preserved
  position — no moved anchor), IQ-5 (reachable without prior knowledge), IQ-8 (no state
  carried by presence, absence or ordering), and AC-UX-06, AC-UX-07, AC-UX-08, AC-UX-09.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** This story adds no
  Rust, touches no port and changes no file under `crates/`, so no `suite.rs` rule can
  observe it. Its automated readers are `redkiln validate --kb`, `git diff`, and the Tier 3
  `reachability_static` grain — which is the only automated check in the repository that can
  catch a promoted atom nothing links to (`_decomposition.md`, `## Testing brief`, Tier 3).
- **Clause(s)**: **none.** No `spec/SPECIFICATION.md` clause is discharged or amended and
  none is touched — `spec/` is read-only for this whole project (`## Architecture brief`,
  §1). No `[FROZEN]` clause is in play, so no ADR is owed.
- **Advances DoD scenario**: **DoD-15 — "The audience is durable and reconciled"**
  (`.bklg/docs-that-teach/initiative.md`, `## Definition of Done`, item 15). This story
  turns green the clause the other two slice-mates cannot: *"linked from this initiative's
  closeout"*. It also unblocks DoD re-observation — `dod-scenario-ledger` (HS-S0180) and
  `terminal-gate-run` (HS-S0182) both list this story in `blocked_by`, because a gate run
  taken before the atoms are mounted proves the gate, not the deliverable
  (`## Architecture brief`, §5, ordering 4).

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file
changed outside it.

```
.kb/maps/domain-map.md
.kb/maps/open-questions-index.md
.kb/README.md
.kb/product/*.md
.kb/open-questions/*.md
.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/**
```

**In this PR**

- The appended `##` section in `.kb/maps/domain-map.md` covering documentation and the
  audience it teaches, with one grouped, id-carrying, one-sentence entry per atom the wave
  landed — including HS-P0021's carried playbook.
- Reciprocal `related` / `depends_on` edges on this project's landed atoms: every journey
  names its persona, every persona names its journeys back, no dangling id.
- Appended bullets in `.kb/maps/open-questions-index.md` — **conditional**, only if the wave
  landed an `open_question` atom — under the existing domain section it belongs to, status
  word first.
- One appended bullet in `.kb/README.md` naming `maps/domain-map.md`, **conditional** on the
  hop-1 gap still holding at implementation time. Additions only; no existing line edited.
- `links.kb` on **HS-P0025**, written by a single
  `redkiln record-links HS-P0025 --atom <comma-separated ids>` invocation after
  `redkiln validate --kb` is green.
- The Knowledge Harvest rows, in `.redkiln/templates/closure.md`'s exact columns, authored
  in a companion under this story's own folder for the closeout to transcribe.
- The recorded two-hop reachability walk from `.kb/README.md` — starting file, hops named,
  every landed atom reached.
- This story's `_ledger.md` (`require_ledger: true`, `.redkiln/config.yaml:62-67`) and its
  stage artefacts, including the routed-gap note for the map's stale preamble.
- A work commit recorded via `redkiln record-links HS-S0179 --sha`
  (`require_commit_provenance: true`, `.redkiln/config.yaml:73`).

**Explicitly not in this PR**

- **Authoring, rewording or re-scoping any atom body.** The wave wrote them; this story
  registers them. A body defect found here is reported, not repaired by editing — and a
  standing atom found wrong is corrected by a **new atom that supersedes it**
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **`.kb/decisions/`, in any form.** No addition, no modification, no `related` edge written
  into a decision atom (AC-A09; T5 — `validate --kb` checks every accepted decision against
  `HEAD`, and that tripwire is desired behaviour, not an obstacle).
- **HS-P0021's carried `.kb/playbooks/` atom's own file.** Mounted from the map, `links.kb`
  and the harvest table; its frontmatter and body are HS-P0021's (AC-A10). Edges toward it
  are written on this project's atoms.
- **`.kb/_intake/` in any form.** The wave cleared it and `.kb/_intake/README.md` must stay
  byte-identical; a re-stage here would reopen AC-009 after it was already observed.
- **Correcting `.kb/maps/domain-map.md`'s preamble or bumping its `last_reviewed`.** Both
  produce a `-` line, which is IQ-3's literal falsifier. Recorded as a routed gap instead.
- **Advancing HS-P0025, writing `closure.md`, or editing any item's YAML frontmatter by
  hand.** The CLI is the single writer of system frontmatter and the orchestrating command
  owns every stage transition (CLAUDE.md, "Where the work lives").
- **Running the fifteen DoD re-observations or `cargo xtask ci`.** `dod-scenario-ledger` and
  `terminal-gate-run` own those, and both are downstream of this story by construction.
- **Anything under `crates/`, `spec/`, `xtask/`, `standards/`, `docs/` or `examples/`.**
  Read-only for this project; a needed change there is a routed gap
  (`.redkiln/config.yaml:5`), not a scope extension.

**Merge DoD one-liner** — merged when every atom the wave landed can be named together with
its map line, its reciprocal edge, its `links.kb` entry and its Knowledge Harvest row;
`git diff` over `.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md` and
`.kb/README.md` contains **no deletion line**; `redkiln validate --kb` exits zero on the
tree that carries the edges and the map section; `cargo xtask lints && cargo xtask
spec-trace` is green; the two-hop walk is recorded and reaches every atom; and
`cargo xtask affected --base main` is green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| One appended `##` section in the domain map, covering documentation **and** its audience | The section is a subject area sized to outlive this wave: it holds the persona atoms, the journey atoms, HS-P0021's carried page-need-discipline playbook and any `open_question` atom the wave landed. A section named after personas alone is already too narrow on the day it is written. | `.kb/maps/domain-map.md:144-150`; `_decomposition.md`, `## Architecture brief`, §6 ("What the appended domain section is called"); `.bklg/docs-that-teach/_decomposition.md:106-114` |
| The section composes the map's existing format, not a new one | Entries grouped by kind under bold labels (**Concepts**, **Playbooks**, **Open questions**, …), each atom's id cited next to its link, one sentence of orientation per atom — because the atom is the source of truth and the map is not. | `.kb/maps/domain-map.md:105-142` (live format), `:144-150` (the rule); `_decomposition.md`, `## UX brief`, "Design-system primitives" |
| No deletion line anywhere in either map file, or in `.kb/README.md` | The mechanical form of append-only. It binds the preamble at `.kb/maps/domain-map.md:35` and the map atom's `last_reviewed` scalar as much as it binds a `##` section: adding a list item is an addition, rewriting a scalar is not. | `_decomposition.md`, `## UX brief`, IQ-3 (falsifier) and AC-UX-06; `## Architecture brief`, AC-A05; project AC-010 |
| The stale preamble is a routed gap, not a fix | `"This is the map's first wave. One domain exists so far."` becomes false when the second domain is appended. It is left alone and recorded as a gap, because the cost of one stale sentence is smaller than the cost of append-only ceasing to be decidable by `git diff`. If the wave's Maps phase rewrote it, revert that hunk before the wave commits. | `.kb/maps/domain-map.md:35`; `_decomposition.md`, `## UX brief`, IQ-3; `## Architecture brief`, AC-A05 |
| Reciprocal edges, both directions, no dangling id | Every journey atom names its persona in `depends_on` / `related`; every persona names its journeys back. A one-way edge leaves the persona unreachable from the journey a charter author starts at, and a dangling id fails validation outright. | `_decomposition.md`, `## Architecture brief`, §2, mount point 3; `.kb/maps/domain-map.md:14-16` (exemplar); `.kb/README.md`, `## Rules` ("No dangling links") |
| Edges toward the carried playbook are written on this project's atoms | Vehicle versus content: the playbook's file is HS-P0021's and is not rewritten, so reciprocity is satisfied from this project's end while the payload stays untouched. | `_decomposition.md`, `## Architecture brief`, AC-A10; `.bklg/docs-that-teach/_decomposition.md:106-114` |
| Index bullets are appended under an existing section, status word first | Conditional on the wave landing an `open_question` atom: `Open` / `Withdrawn` / `Superseded`, then the link, then the id, then one sentence; grounding lives in the atom, never in the index. No new `##` if the question's domain already has a section. | `.kb/maps/open-questions-index.md:136-143`; `_decomposition.md`, `## UX brief`, AC-UX-07 and the forbidden-list entry for a new `##` in the index |
| `links.kb` is written once, on HS-P0025, by the CLI | `redkiln record-links HS-P0025 --atom <ids>` with a **comma-separated** list — a repeated string flag is last-wins under strict `parseArgs`, so one invocation per id records one id. Hand-editing `links:` is denied by a `PreToolUse` hook and would be the wrong record anyway. | `.redkiln/templates/_retrospective.md:59`; `redkiln record-links --help`; CLAUDE.md, "The CLI is the only writer of an item's system frontmatter"; `_decomposition.md`, `## Architecture brief`, §2, mount point 4 |
| The link write happens **after** validation, not before | Ordering 3: `record-links --atom` should record ids that exist and validate; recording first and fixing afterwards leaves an item pointing at an atom that changed shape. | `_decomposition.md`, `## Architecture brief`, §5, ordering 3; `## Testing brief`, merge-gate steps 1 and 4 |
| `redkiln validate --kb` is re-run **after** the edges and the map section | This is the AC-011 half this story owns, distinct from the wave's: the reciprocal edges and the map atom's body are frontmatter and content the validator reads, so the wave's green result is not evidence for the mounted tree. | `_storymap.md`, `## Coverage`, AC-011 row; `_decomposition.md`, `## Testing brief`, merge-gate step 1 |
| `redkiln doctor` still reports **exactly six** `template-drift` advisories | A seventh is a template someone changed without deciding to; a missing one is a customisation reverted. Either is this project's problem to flag rather than silence. Never run `redkiln adopt --templates`. | CLAUDE.md, "Where the work lives"; `_decomposition.md`, `## Testing brief`, merge-gate step 2 |
| The Tier 3 reachability grain runs corpus-wide | `cargo xtask lints && cargo xtask spec-trace` — the only automated check that can catch a promoted atom nothing links to, and it is not diff-scoped, which is exactly why it belongs here rather than in the wave. | `.redkiln/config.yaml:48`; `_decomposition.md`, `## Testing brief`, Tier 3 and merge-gate step 3 |
| The Knowledge Harvest rows are authored, not filed | `closure.md` does not exist until the project's `closeout` stage renders it (`produces: [closure.md]`, `harvest_kb: true`, the only project stage carrying it). The rows are written here in the template's exact `\| KB id \| Kind \| Summary \|` columns, in a companion under this story's folder, for transcription. This story advances nothing. | `.redkiln/processes/project.yaml:75-79`; `.redkiln/templates/closure.md`, `## Knowledge Harvest`; `_decomposition.md`, `## Architecture brief`, §2, mount point 5 |
| A recorded two-hop walk from `.kb/README.md` reaches every landed atom | Starting file named, each hop named, each hop's link text naming its destination. Verified against the tree rather than asserted: `.kb/README.md` carries no markdown link to the maps today, so the walk is only two hops once the conditional bullet is appended. | `_decomposition.md`, `## UX brief`, IQ-5 (falsifier: "findable only by `ls .kb/product/`") and AC-UX-08; `.kb/README.md` |
| Atom ids are read off the landed tree, never predicted | The ingest mints ids. The same string appears in the map line, the `related` edge, the `--atom` list and the harvest row; a mount citing an id this spec guessed would dangle. | `.kb/README.md`, `## Atoms` (the `id` field); `_decomposition.md`, `## Architecture brief`, §4 |
| Mounting never becomes authoring | No new file appears under `.kb/` in this story's diff; the only `.kb/` changes are the two map files, the front-door bullet, and `related` / `depends_on` edges on non-decision atoms the wave already landed. `.kb/decisions/` is untouched. | `_decomposition.md`, `## Architecture brief`, AC-A01, AC-A09, T5; `.kb/README.md`, `## Rules` (immutability scoped to accepted `decision` atoms) |
| Every state is a literal word | No emoji, tick, colour word, strikethrough, empty cell, ordering or absence carries meaning in the map section, the index bullets, the harvest rows or the walk record. Corpus precedent is direct. | `_decomposition.md`, `## UX brief`, `### Accessibility floor` and AC-UX-09, IQ-8; `.kb/maps/open-questions-index.md:136-143` |
| Supersession annotates, it does not delete | If the reconciliation superseded a persona, the map entry and the index bullet say so in words and keep the entry; a superseded atom is annotated, never removed from the index. | `_decomposition.md`, `## UX brief`, IQ-2 (third bullet); `.kb/maps/open-questions-index.md:11-13` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds no Rust type, no serialized
form and no persisted state. It writes markdown and YAML frontmatter into files that already
exist, plus one item field written by the CLI.

Three data-shaped obligations do bind, and none of them is a migration:

- **`KbFrontmatter` is a schema this story writes into but does not extend.** It lives in
  the redkiln plugin rather than this repository, so it is validated by *running*
  `redkiln validate --kb`, never by reading a type (`_decomposition.md`,
  `## Architecture brief`, §4). It is `.passthrough()`, so an imported corpus's own keys
  survive validation — which cuts both ways: do not strip a key to make an atom conform, and
  do not invent a tracking key on an atom you are only mounting. The only keys this story
  touches are `related` and `depends_on`, both already in `.kb/_templates/atom.md`.
- **`links.kb` on HS-P0025 is item system state, not a document.** It is appended and
  de-duplicated by `redkiln record-links`, so re-running after one more atom records only
  the new id; there is no reconciliation step to write and no hand-edit path that is
  legitimate.
- **The append-only rule is a data-integrity constraint on two files, not a style
  preference.** Anchors elsewhere in the corpus cite `.kb/maps/domain-map.md` and
  `.kb/maps/open-questions-index.md` by `file:line` — this spec does so four times — so a
  reflow that shifts a cited line is the text-corpus equivalent of a destructive migration.
  Appending is the one edit shape that cannot break a cited anchor, which is why IQ-3 states
  it as a falsifier on `git diff` rather than as guidance.

## Acceptance criteria

Ten criteria, each framed from the reader whose intent this story exists to serve: **U1**,
the next initiative's charter author, who must reach an atom and frame a criterion from it
without re-running discovery; **U2**, the closeout reviewer at the pull request, who must be
able to find the one atom that is missing one of its four references; and **U3**, the future
maintainer who must be able to correct what was promoted without editing it
(`_decomposition.md`, `## UX brief`, "Who this is actually for, framed as intent"). Every
criterion is observable by a reviewer with the tree in front of them, and every verification
names a real command or a real checklist path.

Traceability: project **AC-010** (`.kb/maps/domain-map.md` gains an appended `##` section
and `git diff` shows no modification to any pre-existing section) is carried by AC-001,
AC-002, AC-009 and AC-010. Project **AC-011** (`redkiln validate --kb` exits zero on the
tree that carries the promoted atoms) is carried by AC-003, AC-004 and AC-006 — the half
`_storymap.md`'s `## Coverage` assigns to this story, re-proved *after* the reciprocal edges
and the map section change the frontmatter the validator reads.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN U1 has never heard of this initiative and opens `.kb/maps/domain-map.md` looking for who this library is for, WHEN they scroll to the end of the file, THEN they find exactly one appended `##` section covering documentation and the audience it teaches, placed after the last pre-existing section, listing **every** atom the wave landed — persona atoms, journey atoms, HS-P0021's carried page-need-discipline playbook and any `open_question` atom — each with a link whose text names its destination, its atom id beside the link, and one sentence of orientation, so U1 can choose which atom to open without opening any of them. | Tier 2 content review of the appended section against the landed atom set, read side by side with `.kb/maps/domain-map.md:105-142` (the live format) and `_decomposition.md`, `## UX brief`, AC-UX-07; roll-call recorded in `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md`. Tier 1 cross-check: every atom under `.kb/product/` and every `.kb/open-questions/` atom in this wave's commit appears in the section. |
| AC-002 | GIVEN U3 later cites a fact by `file:line` against the corpus and GIVEN this spec itself cites `.kb/maps/domain-map.md:35`, `:105-142` and `:144-150`, WHEN this story's diff is applied, THEN `git diff` over `.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md` and `.kb/README.md` contains **no deletion line anywhere** — the stale preamble at `.kb/maps/domain-map.md:35` is left standing and recorded as a routed gap rather than corrected, `last_reviewed` is not bumped, and if the wave's Maps phase rewrote either, that hunk is reverted before the wave commits — so no anchor anyone already cited moves. | Tier 1: `git diff .kb/maps/domain-map.md`, `git diff .kb/maps/open-questions-index.md`, `git diff .kb/README.md` — zero lines beginning `-` (excluding the `---` file header). The routed gap for the preamble is present in `_ledger.md`'s gap note and in `_mount-walk.md`. Falsifier is IQ-3's, verbatim (`_decomposition.md`, `## UX brief`, IQ-3; AC-UX-06; AC-A05). |
| AC-003 | GIVEN U1 has landed on a journey atom and wants the persona whose goals that journey serves, WHEN they read the journey atom's frontmatter, THEN it names that persona in `related` / `depends_on` — and GIVEN U1 started at the persona instead, WHEN they read *its* frontmatter, THEN it names its journeys back, so neither direction of the pair is a dead end; every id on both sides resolves to a real atom, and edges toward HS-P0021's carried playbook are written on **this project's** atoms so the payload's own frontmatter stays byte-identical. | Tier 1: `redkiln validate --kb` exits zero (its no-dangling-links rule, `.kb/README.md` `## Rules`). Tier 2: a reciprocity read — for each landed pair, open both files and confirm the edge exists in both directions; recorded per-atom in `_mount-walk.md`. Tier 1 cross-check on the payload: `git diff` over `.kb/playbooks/` shows the carried atom added by the wave and unmodified afterwards (AC-A10). |
| AC-004 | GIVEN U2 opens `.bklg/docs-that-teach/durable-audience-closeout/project.md` at the pull request and asks what this project actually harvested, WHEN they read HS-P0025's `links.kb`, THEN it lists every atom the wave landed — written by exactly one `redkiln record-links HS-P0025 --atom <comma-separated ids>` invocation run **after** `redkiln validate --kb` came back green, never by hand and never one id per invocation, and never on this story's own item. | Tier 1: read `links.kb` on `.bklg/docs-that-teach/durable-audience-closeout/project.md` and compare its id set to the landed atom set; the transcript of the single `record-links` invocation is cited as evidence. Ordering proved by the command transcript sequence in `_mount-walk.md` (validate green, then record-links) per `_decomposition.md`, `## Architecture brief`, §5 ordering 3 and `## Testing brief` merge-gate steps 1 and 4. |
| AC-005 | GIVEN the project later reaches `closeout` and `closure.md` is rendered for the first time, WHEN whoever runs that stage opens this story's folder, THEN they find the `## Knowledge Harvest` rows already written in `.redkiln/templates/closure.md`'s exact three columns (`\| KB id \| Kind \| Summary \|`), one row per landed atom, ready to transcribe rather than re-derive — and THEN they find that this story advanced no stage and wrote no `closure.md`, because that file does not exist yet and claiming a row was written into it would be a fabricated record. | Tier 2: `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_harvest-rows.md` exists, its table header matches `.redkiln/templates/closure.md:18-19` character for character, and it carries one row per landed atom id. Tier 1: no `closure.md` appears in this story's diff and no item frontmatter is modified (`.redkiln/processes/project.yaml:75-79` — `closure.md` is produced by the `closeout` stage alone). |
| AC-006 | GIVEN the wave already proved `redkiln validate --kb` green on the atoms **as landed**, WHEN the reciprocal edges and the appended map section have changed the frontmatter and content the validator reads, THEN `redkiln validate --kb` is re-run on that tree and exits zero — this story's own obligation, not a second reading of the wave's — and `redkiln doctor` still reports **exactly six** `template-drift` advisories, a seventh or a missing one being flagged as a routed gap rather than silenced, and `redkiln adopt --templates` never run. | Tier 1: `redkiln validate --kb && redkiln doctor` (CLAUDE.md, "Commands"; `_decomposition.md`, `## Testing brief`, merge-gate steps 1 and 2), both transcripts cited with the tree sha they ran against. The advisory count is read off `doctor`'s output and stated as a number, not as "as expected". |
| AC-007 | GIVEN a reader who has never heard of this initiative opens `.kb/README.md` — the corpus front door — WHEN they follow links whose text names each destination, THEN they reach **every** atom this wave landed in at most two hops, and the walk is recorded in prose naming the starting file, each hop and each atom reached; and GIVEN `.kb/README.md` carries no markdown link to `.kb/maps/domain-map.md` today, WHEN that gap still holds at implementation time, THEN one bullet naming `maps/domain-map.md` as the subject-matter index is appended there (addition only) so the walk is genuinely two hops rather than `ls .kb/product/`. | Tier 3 mount-point walk, recorded in `_mount-walk.md`: starting file, hop 1, hop 2, and the full list of atoms reached, checked against the landed atom set for equality. Falsifier is IQ-5's verbatim — an atom findable only by `ls .kb/product/` (`_decomposition.md`, `## UX brief`, IQ-5, AC-UX-08). Tier 1: `git diff .kb/README.md` shows an addition and no `-` line. |
| AC-008 | GIVEN U3 wants to correct something this closeout promoted, WHEN they read this story's diff to learn the route, THEN they find that mounting never became authoring: **no new file appears under `.kb/`** in this story's diff, `.kb/decisions/` is untouched in every form (no addition, no modification, no `related` edge written into one), HS-P0021's carried playbook body and frontmatter are unedited, `.kb/_intake/README.md` is byte-identical and `.kb/_intake/` stays cleared, no atom id is renamed or reused — so the only correction route the tree offers is a **new atom that supersedes**, never an edit. | Tier 1: `git diff --stat` over `.kb/` shows changes confined to `.kb/maps/domain-map.md`, `.kb/maps/open-questions-index.md`, `.kb/README.md` and `related` / `depends_on` frontmatter on non-decision atoms the wave landed; `git diff .kb/decisions/` and `git diff .kb/_intake/` are both empty. `redkiln validate --kb` independently fails on any accepted-decision touch, checking each against `HEAD` (CLAUDE.md, "Where the work lives"; AC-A09, T5). Route stated in prose per `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. |
| AC-009 | GIVEN U2 must be able to find the one atom that is missing one of its four references, WHEN they open this story's mount-point roll-call, THEN every landed atom is named once with all four of its registrations quoted — its domain-map line, its reciprocal edge, its `links.kb` entry and its Knowledge Harvest row — with the **same id string** in all four, and THEN every one of those ids was read off the landed frontmatter rather than predicted by this spec, so no reference dangles; an atom missing any one of the four is reported as unmounted rather than counted. | Tier 3 mount-point walk in `_mount-walk.md`: a four-column table, one row per atom, no empty cell and no cell reading "n/a" (the bar is `_decomposition.md`, `## Architecture brief`, §2 and AC-A04 — "four references, or it is not mounted"). Tier 1 cross-check: each quoted id resolves via `redkiln validate --kb` and appears verbatim in `links.kb`. |
| AC-010 | GIVEN U2 reads this story's diff and GIVEN half the corpus's readers will meet it as plain text in a quote or a diff hunk rather than in a renderer, WHEN they read any entry, bullet or row this story added, THEN every state it expresses is a **literal word** — no emoji, tick, colour word, strikethrough, empty cell, ordering or absence carries meaning; a superseded atom is annotated in words and kept, never removed; index bullets state `Open` / `Withdrawn` / `Superseded` first, then the link, then the id, then one sentence, appended under the question's existing domain section rather than a new `##`; the map section groups entries by kind under bold labels and composes the map's existing format rather than a new one; and no `.kb/product/` subdirectory, no frontmatter key outside `.kb/_templates/atom.md` and no bespoke per-atom table appears anywhere. | Tier 2 review against `_decomposition.md`, `## UX brief`, `### Accessibility floor`, AC-UX-07 and AC-UX-09, plus its "Hand-rolling, explicitly forbidden here" list, read against `.kb/maps/open-questions-index.md:136-143` and `.kb/maps/domain-map.md:105-142` as the composed exemplars. Tier 1 cross-check: `rg -n "✅|❌|🟢|~~" .kb/maps .kb/README.md` returns nothing from this diff, and `rg -n "^(kind|authority_tier):" .kb/product` confirms no invented key. |

## Interaction quality

RFC §6.7/D6. **Every invariant below is already carried by an AC-### row in the table
above** — this section only says which row carries which, and how it is verified. Nothing
here is a free-standing bullet, because `redkiln verify` extracts ACs by matching a leading
`| AC-001 |` table cell or a `- AC-001:` bullet, so an invariant stated only as prose in
this section would get no ledger row, would never be gated, and would never be tested.

**The composition family's source, and why it is the UX brief rather than `_design.md`.**
This project's signed-off design (`.bklg/docs-that-teach/durable-audience-closeout/_design.md`)
records `hasSurface: false` with an empty `## Items` block — "no items — no public API
surface, no rendered UI surface". It is binding on this story **by what it excludes**: there
is no screen, no route, no DOM node, no token layer and no chrome decision to inherit, and
none may be invented here. The composition invariants that do bind come from the layer the
UX brief identifies as this corpus's honest primitive layer — the atom template, the
frontmatter vocabulary and the two map formats, which "behave exactly like a token layer"
and have a real validator behind them (`_decomposition.md`, `## UX brief`,
"Design-system primitives — compose these, do not hand-roll").

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1's analogue at the index grain) — U1 decides *which* atom to open from the map entry alone: link text that names the destination, the id, and one sentence of orientation. The entry never substitutes for the atom, and the atom never has to be opened to know what it is. | AC-001, AC-007 | Tier 2 read of every appended entry; a link reading "here", "this" or "see above" fails WCAG 2.4.4 as the accessibility floor states it. |
| **Non-occlusion — the map indexes, it does not gate.** Every promoted atom stays reachable by directory listing and by grep; the map section is *one* path to it, not the path. Supersession annotates and keeps the entry; it never deletes it. Clearing `.kb/_intake/` does not hide the evidence, because `source_paths` still cites both the staging path and the discovery artefact. | AC-002, AC-008, AC-010 | Tier 1 `git diff` (no deletion line); Tier 2 read of any superseded entry for its annotation word; `ls .kb/product/` still enumerates every atom (IQ-2, `.kb/maps/domain-map.md:148-150`, `.kb/maps/open-questions-index.md:11-13`). |
| **Preserved position** — the text analogue of preserved focus, scroll and selection. No cited `file:line` moves, no pre-existing section is reflowed, no atom id is renamed or reused, and a charter that already cited the discovery distillation and one that cites the atom still point at the same audience. | AC-002, AC-008 | Tier 1: the no-`-`-line diff check, plus `git diff` confirming no id rename; IQ-3's falsifier and IQ-6 (`_decomposition.md`, `## UX brief`). |
| **Reversibility** — the whole wave lands as one commit on its own branch and is reviewed by merging; `links.kb` is appended and de-duplicated by the CLI so a re-run records only what is new; and a promoted atom later found wrong is corrected by a **new atom that supersedes it**, never by an edit and never by an ADR written as a closeout by-product. | AC-004, AC-008 | Tier 1: re-running `redkiln record-links HS-P0025 --atom` with the same ids leaves `links.kb` unchanged; the supersede-not-edit route is stated in prose per `.kb/governance/rewrite-the-referent-never-the-reasoning.md` (IQ-4). |
| **Reachable without prior knowledge** — the keyboard-reachability analogue. Two hops from `.kb/README.md`, link text naming the destination at each hop, no atom findable only by knowing the initiative slug or by `ls .kb/product/`. | AC-007 | Tier 3 recorded walk, checked for set equality against the landed atom set (IQ-5, AC-UX-08). |

### Composition invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all.** Every registration this story writes carries real composed presentation, not bare markup: a map entry is a bold kind label, a destination-naming link, the atom id and one orientation sentence — never a bare path dumped in a list; a harvest row is three filled columns — never an id with two empty cells. | AC-001, AC-005, AC-010 | Tier 2 read against the composed exemplars at `.kb/maps/domain-map.md:105-142` and `.kb/maps/open-questions-index.md:136-143`; an empty cell is itself a forbidden state carrier (AC-UX-09). |
| **Composition and placement.** Exactly one appended `##` in the domain map, placed after the last pre-existing section, covering documentation and its audience as one subject area. Index bullets are appended **under the question's existing domain section**; a new `##` in `open-questions-index.md` when that domain already has a section is on the forbidden list. The front-door bullet is appended to `.kb/README.md`'s existing list, not a new section. | AC-001, AC-007, AC-010 | Tier 1 `git diff` shows exactly one added `##` line in the domain map and zero added `##` lines in the index; Tier 2 read for the section's placement (`.kb/maps/open-questions-index.md:136-140`). |
| **Transience — what is persistent chrome, what is opened on demand.** The map section and the index bullets are **persistent chrome**: visible the moment the file is opened, never behind a disclosure. The atom body is **opened on demand** — the map deliberately does not restate it, "because the atom itself is the source of truth, not this map". Nothing this story writes is *revealed* — there is no hover, no fold, no collapsed block, and a caveat placed behind one would be exactly how a guess acquires the standing of a finding. | AC-001, AC-010 | Tier 2 read: no `<details>`, no collapsed block, no trailing "caveats" section anywhere in the appended content (`.kb/maps/domain-map.md:144-150`; IQ-7). |
| **Density budget, with its real numbers.** One orientation sentence per map entry (**1**, not a paragraph). Index bullet = status word + link + id + **1** sentence, in that order. Harvest row = exactly **3** columns. Bold kind labels drawn from the map's existing set of at most **5** (**Reference**, **Concepts**, **Governance**, **Playbooks**, **Open questions**) — no sixth label invented. **4** references per atom, or it is not mounted. **2** hops maximum from the front door. | AC-001, AC-005, AC-007, AC-009, AC-010 | Tier 2 count-and-read against each number; Tier 3 four-column roll-call in `_mount-walk.md` with no empty cell (AC-A04). |
| **Hierarchy.** One `#` per file, sections at `##`, no skipped level; bold kind labels are emphasis inside a section, never headings promoted to `###`. Entries are grouped by kind, so a reader scanning for a playbook never has to read the concepts. | AC-001, AC-010 | Tier 2 read against the accessibility floor's "Heading structure" bullet and the live format at `.kb/maps/domain-map.md:105-142`. |
| **The named anti-patterns, each one refused.** A new map format; a new `##` in the index when the domain already has a section; a subdirectory under `.kb/product/`; a frontmatter key not in `.kb/_templates/atom.md`; a bespoke per-atom table, "Persona card" or "TL;DR"; a status carried by an emoji, a tick, a colour word or an empty cell; an atom written by hand into `.kb/`, whatever its shape. | AC-008, AC-010 | Tier 2 review against `_decomposition.md`, `## UX brief`, "Hand-rolling, explicitly forbidden here"; Tier 1 `rg` sweeps for glyph-carried state and for keys outside the atom template. |
| **Linear readability — plain-text equivalence for load-bearing claims.** Every map entry, index bullet and harvest row is self-contained: it names its atom, its kind and what it is for, with nothing that only makes sense read after the row above. A verdict that exists only as a table cell is a verdict that will be lost in a quote or a diff. | AC-001, AC-005, AC-009, AC-010 | Tier 2: read each row in isolation, out of order, and confirm nothing is lost (`_decomposition.md`, `## UX brief`, `### Accessibility floor`, "Linear readability"). |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| EC-001 | The ingest wave's Maps phase already appended a domain-map section, but under a name narrower than "documentation and its audience", or in a shape that is not the map's existing format. | Re-shape it **inside the same wave, before the wave commits** — this story owns the shape of what the Maps phase wrote (`.kb/maps/domain-map.md:7-12`). Do not add a second section; do not leave two. |
| EC-002 | The Maps phase rewrote `.kb/maps/domain-map.md`'s preamble at `:35`, or bumped `last_reviewed`. | Revert that hunk before the wave commits. Both produce a `-` line, which is IQ-3's literal falsifier; a fait accompli is not an exemption. The stale preamble is then recorded as a routed gap in `_ledger.md`. |
| EC-003 | An atom id cited in a map line, a `related` edge, the `--atom` list or a harvest row does not resolve to a real atom. | `redkiln validate --kb` fails on the dangling id (`.kb/README.md`, `## Rules`). Fix by re-reading the landed frontmatter and correcting the citation — **never** by creating the atom to make the citation true, which would be hand-authoring under AC-A01. |
| EC-004 | A repair the mount seems to need would require editing a file under `.kb/decisions/`. | **Stop.** Record it as a routed gap in prose in this story's ledger and `_mount-walk.md`. AC-A09 forbids the touch outright, and `validate --kb` checks every accepted decision against `HEAD`, so the attempt fails merge-gate step 1 structurally (T5). |
| EC-005 | `redkiln record-links HS-P0025 --atom` was invoked once per id. | Only the last id is recorded — a repeated string flag is last-wins under strict `parseArgs`. Re-invoke **once** with the full comma-separated list and re-read `links.kb` on the project item to confirm the whole set landed. |
| EC-006 | The wave landed no `open_question` atom. | `.kb/maps/open-questions-index.md` stays out of the diff entirely. Do **not** append an empty section or a placeholder bullet — the index bullets are conditional by construction (`## PR boundary`, "conditional"). |
| EC-007 | `.kb/README.md` already links `maps/domain-map.md` by the time this story runs. | The conditional front-door bullet is **not** added; the recorded walk cites the existing link as hop 1. Adding a duplicate bullet would be an unneeded touch of the composition root. |
| EC-008 | `redkiln doctor` reports a seventh `template-drift` advisory, or fewer than six. | Flag it as a routed gap; do not silence it and **never** run `redkiln adopt --templates` — the six customisations are this repository's own gate bars, and `adopt --templates` would overwrite all six silently (CLAUDE.md, "Where the work lives"). |
| EC-009 | A landed atom carries `authority_tier: note` or `guideline` instead of `product`, or a persona landed as `playbook` / a journey as `concept`. | Do not mount it and do not patch the tier as a registration edit. Report to `audience-ingest-wave` and repair **inside the wave** before it commits — the tier is that story's AC-004 bar and this project's single highest-probability defect (`.kb/product/README.md:6-13`; `_storymap.md`, "Notes the implementer needs at the story grain"). |
| EC-010 | `.kb/_intake/` is not cleared, or `.kb/_intake/README.md` differs from its pre-wave bytes. | Halt the mount. AC-009 was already observed by the upstream story and a re-stage here would reopen it; the README's survival is an approval-gate property, not something to repair after the fact (`.kb/_intake/README.md:13-19`, `:29-31`; AC-A02). |
| EC-011 | `cargo xtask lints && cargo xtask spec-trace` fails on a file this story did not touch. | It is corpus-wide, not diff-scoped, so a pre-existing corpus break can surface here. Record which file failed and whether this story's diff caused it; a break outside this project's diff surface is a routed gap (`.redkiln/config.yaml:5`), not a licence to edit `spec/` or `crates/`. |

## Non-functional

| id | Requirement | Why it binds here |
| --- | --- | --- |
| NF-001 | **Anchor stability.** No `file:line` citation anywhere in the repository resolves to different text after this story's diff. | The corpus cites `.kb/maps/domain-map.md` and `.kb/maps/open-questions-index.md` by line — this spec alone does so four times — so a reflow is the text-corpus equivalent of a destructive migration. Appending is the one edit shape that cannot break a cited anchor. |
| NF-002 | **Idempotence of the mount.** Re-running `redkiln record-links HS-P0025 --atom <same ids>` leaves `links.kb` byte-identical; re-walking the two hops yields the same atom set; re-running `redkiln validate --kb` on the unchanged tree yields the same exit code. | The mount is evidence, and evidence a second run would contradict is not evidence. `record-links` appends and de-duplicates, so this is a property to *observe*, not to implement. |
| NF-003 | **Zero compilation cost added.** This story adds no Rust, no dependency and no CI step. The only long-running command it introduces is the corpus-wide Tier 3 grain already declared at `.redkiln/config.yaml:48`. | The project ships no Rust at all; a new gate step here would be scope drift into `xtask/`, which is read-only for this project. |
| NF-004 | **Plain-text legibility.** Every artefact this story writes reads correctly in a diff hunk, a quoted excerpt and a terminal `cat` — no rendering required to recover a state or a verdict. | The accessibility floor's "Plain-text equivalence for load-bearing claims" and AC-UX-09; U2 reviews this at a pull request, in a diff. |
| NF-005 | **No invented tooling.** Every command this story names resolves to a real CLI verb (CLAUDE.md, "Commands"), a real grain in `.redkiln/config.yaml`, or a real target in `xtask/src/main.rs`. | AC-TB-02. A verification step that names a command nobody can run is a decorative check, which is the exact failure the testing brief's tier mapping exists to prevent. |
| NF-006 | **Traceable provenance.** The story's work commit is recorded via `redkiln record-links HS-S0179 --sha`, and every command transcript cited as evidence names the tree sha it ran against. | `require_commit_provenance: true` (`.redkiln/config.yaml:73`), and AC-A06's rule that nothing is observed against an unnamed tree. |

## Implementation notes (non-prescriptive)

These are observations that save time, not instructions that override judgement.

- **Read the tree before writing a single line.** The ingest minted the ids; this spec did
  not. `rg -n "^id:" .kb/product .kb/open-questions .kb/playbooks` over the wave's commit is
  the first command, and its output is the vocabulary every subsequent registration reuses
  verbatim. A mount that cites a guessed id dangles, and `validate --kb` will say so — but
  only after four files already carry the wrong string.
- **Do the roll-call table first, then fill it.** `_mount-walk.md`'s four-column table
  (atom id | map line | reciprocal edge | `links.kb` | harvest row) is easier to build empty
  and fill than to reconstruct afterwards, and building it empty is what makes an unmounted
  atom visible while there is still time to mount it. AC-A04's tripwire is a table by nature.
- **The order is fixed and it is short.** Map section and edges → `redkiln validate --kb`
  → `redkiln record-links HS-P0025 --atom` → Tier 3 walk → `cargo xtask affected --base
  main`. Ordering 3 is the one that costs something if inverted: recording links first
  leaves the project item pointing at an atom that then changed shape.
- **Check the append-only property with `git diff` before you believe it.** A markdown
  editor that trims trailing whitespace or reflows a paragraph on save turns a pure
  addition into an addition plus a deletion, silently. This is the single most likely way
  AC-002 fails, and it fails without anyone intending anything.
- **`.kb/README.md` hop-1 gap is conditional — check it, do not assume it.** The gap held
  when this spec was written; a slice-mate or the Maps phase could close it first. EC-007
  covers the other branch.
- **Where the two map files disagree with each other, the atom wins.** Neither map is the
  source of truth; both are indexes. If the map section and an atom's own frontmatter say
  different things about kind or supersession, the map line is what changes.
- **The harvest rows' `Summary` column wants the atom's own `summary:` compressed to one
  line, not a fresh description.** Two descriptions of one atom is two things to keep in
  sync and one that goes stale — the same reason CLAUDE.md gives for not duplicating the
  style guide.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, `## Testing brief` — its four-tier mix, its five merge-gate
commands in the architecture brief's fixed order, and its AC-###→tier mapping. This project
compiles no new Rust, so Tier 2 is not optional colour: for this story's content-shaped
criteria it is the **only** tier that can fail a plausible wrong implementation.

| tier | command / path | proves |
| --- | --- | --- |
| **1 — static / shape** | `redkiln validate --kb` | AC-003, AC-006. Frontmatter conformance and the no-dangling-links rule on the tree **that already carries the reciprocal edges and the map section** — the AC-011 half `_storymap.md`'s `## Coverage` assigns to this story. Also the free assertion that `.kb/decisions/` was not touched, since every accepted decision is checked against `HEAD` (AC-008, T5). |
| **1 — static / shape** | `redkiln doctor` | AC-006. Exactly six `template-drift` advisories, stated as a number. A seventh or a missing one is flagged, never silenced. |
| **1 — static / shape** | `git diff .kb/maps/domain-map.md` · `git diff .kb/maps/open-questions-index.md` · `git diff .kb/README.md` | AC-002. Zero deletion lines — IQ-3's falsifier applied mechanically, which is what keeps "append-only" decidable by `git diff` rather than believable by a reviewer. |
| **1 — static / shape** | `git diff --stat .kb/` · `git diff .kb/decisions/` · `git diff .kb/_intake/` · `git diff .kb/playbooks/` | AC-008. No new file under `.kb/` in this story's diff; decisions and intake empty; the carried playbook added by the wave and unmodified after (AC-A01, AC-A09, AC-A10). |
| **1 — static / shape** | `rg -n "^id:" .kb/product .kb/open-questions .kb/playbooks` · `rg -n "^(kind\|authority_tier):" .kb/product` | AC-009, AC-010. The landed id set the roll-call is checked against, and confirmation that no frontmatter key outside `.kb/_templates/atom.md` was invented. |
| **1 — static / shape** | `rg -n "✅\|❌\|🟢\|~~" .kb/maps .kb/README.md` | AC-010. No glyph-, colour- or strikethrough-carried state in the appended content (AC-UX-09, the accessibility floor's "Colour, glyph and position never alone"). |
| **2 — content review** (human, against a named checklist) | `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md`, `## UX brief` — IQ-1…IQ-8 and AC-UX-06, AC-UX-07, AC-UX-08, AC-UX-09; read against `.kb/maps/domain-map.md:105-142` and `.kb/maps/open-questions-index.md:136-143` | AC-001, AC-005, AC-010. Whether the section is *composed* rather than dumped, whether each entry is self-contained, whether every state is a literal word, whether the harvest rows match the closure template's columns. Per AC-TB-06, this is the brief's own checklist — no second, divergent list is maintained here. |
| **2 — content review** | `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md` (reciprocity read) | AC-003. For each landed pair, both files opened and both directions confirmed. The named wrong implementation this rejects: a journey atom that names its persona while the persona names nothing back — valid frontmatter, passes Tier 1, and leaves the persona unreachable from the journey a charter author actually starts at. |
| **3 — integration / mount-point** | `cargo xtask lints && cargo xtask spec-trace` (the `reachability_static` grain, `.redkiln/config.yaml:48`) | AC-007, AC-009. Corpus-wide rather than diff-scoped, which is precisely why it belongs to this story: it is the only automated check in the repository that can catch a promoted atom nothing links to. |
| **3 — integration / mount-point** | The four-point walk recorded in `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_mount-walk.md`, plus `links.kb` read off `.bklg/docs-that-teach/durable-audience-closeout/project.md` | AC-004, AC-007, AC-009. Every atom named once with all four registrations quoted and the same id string in all four; the two-hop walk's reached-set equal to the landed set. An atom missing one is reported unmounted, not counted. |
| **3 — integration / mount-point** | `.bklg/docs-that-teach/durable-audience-closeout/product-layer-mounting/_harvest-rows.md` header compared to `.redkiln/templates/closure.md:18-19` | AC-005. The rows are transcribable rather than re-derivable at `closeout`, and no `closure.md` was fabricated. |
| **story grain (this PR's own gate)** | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | The regression net. This story maps to no package, so the value here is the five unconditional file-reading lints and `spec-trace` — which is exactly why the grain runs them regardless of package mapping. |
| **story grain (this PR's own gate)** | `redkiln verify --grain story` over `_ledger.md` and the `## PR boundary` fence | `require_ledger: true` (`.redkiln/config.yaml:62-67`) — every AC-### above carries a row with cited evidence — and no file changed outside the declared boundary. |
| **4 — e2e / whole-tree** | `cargo xtask ci` — **not run by this story** | Deliberately out of scope. `terminal-gate-run` (HS-S0182) owns it and must run it **last**, on the tree that already carries these mounts; a gate run taken here would prove the gate, not the deliverable (`_decomposition.md`, `## Architecture brief`, §5 ordering 4; `## Testing brief`, merge-gate step 5, AC-TB-03). |

**Merge-gate order for this story**, a strict subset of the brief's five steps, in the same
sequence: (1) `redkiln validate --kb`, (2) `redkiln doctor`, (3) `cargo xtask lints &&
cargo xtask spec-trace`, (4) `redkiln record-links HS-P0025 --atom` — which is a
precondition Tier 3's walk checks for, not a test in itself. Step 5 belongs to
`terminal-gate-run`.

## Risks and coupling (PR-scoped)

| Risk | Why it is live here | Mitigation inside this PR |
| --- | --- | --- |
| **The atom ids are unknowable at spec time.** Every registration cites ids the ingest mints, and this spec deliberately fixes none of them. | A spec that guessed would produce four dangling references in one commit. | AC-009 makes "read off the landed tree" an acceptance bar, and `redkiln validate --kb` (AC-006) fails on a dangling id before the wave commits. |
| **The Maps phase and this story write the same file.** `/redkiln:kb-ingest` syncs the maps; this story owns the shape of what it wrote. | Two writers, one file, inside one wave — the classic place a fait accompli gets accepted because it is already there. | EC-001 and EC-002 fix the disposition: re-shape inside the wave, revert any hunk that produced a `-` line, never add a second section. |
| **An editor turns an append into an edit.** Whitespace trimming or paragraph reflow on save produces a deletion line nobody intended. | AC-002's falsifier is mechanical and unforgiving, and this is the failure mode that arrives without intent. | The `git diff` check is a named merge-gate step, run and cited, not an assumption. |
| **`closure.md` does not exist, so the fourth mount point cannot be completed here.** | AC-A04(d) names a row in a file the `closeout` stage renders (`.redkiln/processes/project.yaml:75-79`). Claiming to have written it would be a fabricated record. | AC-005 converts it into an authored, transcribable companion in this story's own folder, and the spec says plainly that this story advances no stage. Residual coupling to `closeout` is declared, not hidden. |
| **`.kb/README.md` is the corpus front door and this story touches it.** | Hop 1 of the two-hop walk has no link today, so IQ-5's falsifier is otherwise unavoidable. | The touch is one appended bullet, declared in the `## PR boundary` fence, conditional (EC-007), and covered by the same no-deletion-line check as the maps. |
| **The Tier 3 grain is corpus-wide.** `cargo xtask lints && cargo xtask spec-trace` can fail on a file this story never touched. | A red gate that is not this story's fault still blocks the merge and tempts an out-of-boundary fix. | EC-011: record what failed and route it; `spec/`, `crates/`, `xtask/`, `standards/`, `docs/` and `examples/` stay read-only for this project. |
| **Downstream coupling is total.** `dod-scenario-ledger` (HS-S0180) and `terminal-gate-run` (HS-S0182) both list this story in `blocked_by`. | A partial mount does not merely under-deliver — it invalidates the DoD re-observation taken on top of it, because DoD-15's "linked from this initiative's closeout" would be observed false. | AC-009's four-reference roll-call is the single artefact both downstream stories read to know the mount is complete; an unmounted atom is reported as such rather than counted. |
| **This story is never delivered alone.** The whole `product-layer-promotion` slice lands as one commit on one branch, reviewed by merging. | A reviewer reading this story's diff in isolation will see registrations for atoms that appear in the same commit — confusing without the slice context. | The `## Integration contract` states the slice and its order; `_mount-walk.md` is written for U2 reading the whole wave at once. |

## Dependencies

**Blocks on** — `audience-ingest-wave` (HS-S0178). Hard, and structural rather than
conventional: this story registers atoms, and until the wave has run there is nothing to
register, no id to cite and no frontmatter to add an edge to. Its own upstream,
`staged-audience-payload` (HS-S0177), reaches this story transitively — all three are
slice-mates in `product-layer-promotion` and land as **one commit on one branch**
(`_storymap.md`, `## Merge order`, §3; `## Slices`, "Why these four and not eleven").

**Unlocks** —

- `dod-scenario-ledger` (HS-S0180), which lists this story in `blocked_by` alongside
  `post-merge-clause-completeness`: DoD-15's clause *"linked from this initiative's
  closeout"* cannot be re-observed as green until the four registrations exist.
- `terminal-gate-run` (HS-S0182), which lists this story in `blocked_by` alongside
  `scenario-two-fault-injection`: ordering 4 requires everything committed before the final
  gate, and a gate run taken before the atoms are mounted "proves the gate, not the
  deliverable" (`_decomposition.md`, `## Architecture brief`, §5, ordering 4).

**Not a dependency, despite proximity** — `charter-open-question-disposition` (HS-S0175).
Whether any `open_question` atom exists for this story to index is decided there, but that
decision reaches this story *through* `staged-audience-payload` and the wave, not directly.
This story's index-bullet obligation is conditional on what the wave landed, which is why
the `## PR boundary` marks `.kb/maps/open-questions-index.md` conditional and EC-006 gives
the empty branch a correct behaviour.

## Anchors (progressive disclosure)

The `## Context pack` above is the must-read core and is self-sufficient for starting. These
are the deeper artefacts — open each at the moment named, and do not pre-load the set.

| anchor (real path) | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/maps/domain-map.md` | The mount point itself. `:105-142` is the live section format this story composes rather than invents; `:144-150` is the append-only rule and the "the atom is the source of truth, not this map" sentence; `:35` is the preamble AC-002 leaves standing; `:14-16` is the exemplar of a map atom carrying `related:` ids. | Before writing the appended section — first file open, after reading the landed ids. | AC-001, AC-002, AC-010 |
| `.kb/maps/open-questions-index.md` | `:136-143` fixes the index-bullet shape (status word, link, id, one sentence) and the rule against a new `##` when the domain already has a section; `:11-13` is the corpus's own statement that a superseded entry stays listed and annotated rather than removed. | Only if the wave landed an `open_question` atom — otherwise EC-006 applies and this file stays out of the diff. | AC-010, AC-002 |
| `.kb/README.md` | The corpus front door and hop 1 of the recorded walk; its `## Rules` section is where the no-dangling-links rule lives and where immutability is scoped to accepted `decision` atoms — the distinction that makes editing a `concept`/`playbook` atom's `related` list legitimate. | Before adding any reciprocal edge (for the Rules), and again when recording the walk (for hop 1). | AC-003, AC-007, AC-008 |
| `.kb/_templates/atom.md` | The frontmatter vocabulary. `related` and `depends_on` are the only two keys this story writes; any key outside this file is the text-corpus equivalent of bespoke CSS. | Immediately before editing any atom's frontmatter. | AC-003, AC-010 |
| `.kb/product/README.md` | `:6-13` is the tier table — persona → `kind: concept`, journey → `kind: playbook`, both `authority_tier: product`. Read to confirm the landed atoms clear the bar **before** registering them anywhere; the tier is this project's single highest-probability defect. | At the start, as a gate on the landed set — EC-009 depends on this read. | AC-009, EC-009 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The binding constraint in the absence of any ADR: a promoted atom later found wrong is corrected by a new atom that supersedes it, never by an edit. This is the route AC-008 requires the tree to offer. | When a body defect is found in a landed atom, or when writing the supersession route into the record. | AC-008 |
| `.redkiln/templates/closure.md` | `:14-20` — the `## Knowledge Harvest` heading and its exact three columns (`\| KB id \| Kind \| Summary \|`). The harvest rows must match this character for character to be transcribable. | When authoring `_harvest-rows.md`, with the template open beside it. | AC-005 |
| `.redkiln/processes/project.yaml` | `:75-79` — `closeout` is the only project stage carrying `harvest_kb: true` and the only one that `produces: [closure.md]`. This is the mechanical reason the fourth mount point is a handoff rather than a write. | When tempted to write `closure.md`, or when explaining in the record why it was not written. | AC-005 |
| `.redkiln/templates/_retrospective.md` | `:59` — the single writer of `links.kb`: `redkiln record-links <id> --atom`. Confirms the verb and the flag before running it against the project item. | Immediately before the `record-links` invocation, after `validate --kb` is green. | AC-004 |
| `.bklg/docs-that-teach/durable-audience-closeout/_decomposition.md` | The three briefs. `## Architecture brief` §2 (the mount points and the "four references, or it is not mounted" tripwire), AC-A01/A04/A05/A09/A10, §5 ordering 3; `## UX brief` IQ-2/IQ-3/IQ-5/IQ-8, AC-UX-06…AC-UX-09, the design-system primitives table and the forbidden-list; `## Testing brief` Tier 3 and merge-gate steps 1–4. | The UX brief's checklist at Tier 2 review time; the architecture brief §2 before building the roll-call table. | AC-001, AC-002, AC-004, AC-009, AC-010 |
| `.bklg/docs-that-teach/durable-audience-closeout/_design.md` | The signed-off design, `hasSurface: false`, `## Items` empty by decision. Load-bearing by exclusion: it forecloses inventing a screen, a token layer or a chrome decision here, and redirects the composition family to the UX brief's primitives. | Once, before writing the `## Interaction quality` obligations into code — to confirm nothing was inherited that this story is contradicting. | AC-010 |
| `.bklg/docs-that-teach/durable-audience-closeout/audience-ingest-wave/spec.md` | The upstream slice-mate. Its landed atom ids, its Maps-phase output and its `.kb/_intake/` clearing are the preconditions this story reads and completes; EC-009 and EC-010 both route defects back into it. | First, before any write — this is where the landed set is established. | AC-009, EC-009, EC-010 |
| `.bklg/docs-that-teach/durable-audience-closeout/_storymap.md` | `## Coverage` names the AC-011 ownership seam between the wave and this story in one sentence — the wave proves validate green on the atoms as landed, this story re-proves it after the edges and the section change what the validator reads. `## Merge order` §3 fixes this story as last in the slice. | When justifying why `validate --kb` is re-run rather than inherited. | AC-006 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | HS-P0025 — the item that carries `links.kb`, and the source of DR-9, DR-10 and project AC-010/AC-011. Also the risk row *"Documentation has no functional home in the domain map"*. | At the `record-links` step (to read `links.kb` back), and when checking the traced project ACs. | AC-004 |
| `.bklg/docs-that-teach/initiative.md` | `:465-468` — DoD-15, whose clause *"linked from this initiative's closeout"* is the one this story turns green, and `:524-525`, the charter's own statement that none of the decisions concerns documentation. | When the temptation to cite an ADR arises, and when writing the DoD-15 evidence line. | AC-005, AC-004 |
| `.kb/_intake/README.md` | `:5-7` (the default glob), `:13-19` (a successful ingest clears the directory and the human reviews by merging) and `:21-27` (`validate --kb` cannot see `.kb/_intake/`). Establishes that the intake is already cleared when this story runs and must stay that way. | Before touching anything under `.kb/`, to confirm the wave's postcondition holds. | AC-008, EC-010 |
| `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | The composed `concept`-atom exemplar — `:23-28` shows `source_paths` citing both the `.kb/_intake/…` staging file and the long-form evidence, and `related:` carried by id. The shape a persona atom's edges should look like from the outside. | When a reciprocal edge's shape is in doubt. | AC-003 |
| `.redkiln/config.yaml` | `:40` (`affected_gate`), `:48` (`reachability_static`, the Tier 3 grain), `:62-67` (`require_ledger`), `:73` (`require_commit_provenance`). The four declarations that make this story's gate what it is. | When assembling the merge-gate command list and the ledger. | AC-006, AC-007, AC-009 |

## Clarifications resolved during spec

1. **AC ids are exactly AC-001 … AC-010, as the front half decided.** None added, none
   dropped. The ledger enumerates the same ten.
2. **The four mount points became ten criteria, not four, and the split is deliberate.**
   AC-A04's four registrations are necessary but not sufficient: the append-only property
   (AC-002), the re-run of validation (AC-006), the reachability walk (AC-007), the
   authoring boundary (AC-008), the roll-call that makes an unmounted atom visible (AC-009)
   and the composition floor (AC-010) each reject a plausible wrong implementation that a
   four-row table would pass. A criterion no implementation can fail is decorative.
3. **The composition family is sourced from the UX brief, not from `_design.md`, and that
   required a decision.** `_design.md` is signed off with `hasSurface: false` and an empty
   `## Items` block, so it declares no surface to inherit. Rather than record "N/A" for the
   whole composition family — which would leave a real presentation surface ungoverned — the
   invariants are taken from the layer the UX brief names as this corpus's honest primitive
   layer (the atom template, the frontmatter vocabulary, the two map formats). `_design.md`
   still binds, by exclusion: no screen, no route, no token layer, no chrome decision is
   invented here.
4. **The density budget's numbers are real, and they were read off the corpus rather than
   chosen.** One orientation sentence per map entry, one sentence per index bullet, three
   harvest columns, at most five bold kind labels, four references per atom, two hops from
   the front door — each traced to `.kb/maps/domain-map.md:105-150`,
   `.kb/maps/open-questions-index.md:136-143`, `.redkiln/templates/closure.md:18-19`,
   AC-A04 and IQ-5 respectively.
5. **Two companion artefacts are introduced, both inside this story's own folder and inside
   the declared PR boundary.** `_mount-walk.md` carries the four-reference roll-call and the
   recorded two-hop walk; `_harvest-rows.md` carries the Knowledge Harvest rows in the
   closure template's exact columns. Neither is a new tree, a new format or a new location —
   `product-layer-mounting/**` is already fenced in the `## PR boundary`.
6. **`cargo xtask ci` is named in the test table and explicitly *not run* by this story.**
   The testing brief's merge-gate step 5 belongs to `terminal-gate-run`; running it here
   would satisfy nothing AC-016 can accept (AC-TB-03) and would burn the ordering the
   architecture brief fixes. The row is kept in the table so the omission is a decision on
   the record rather than a gap.
7. **`redkiln verify` extraction shaped the layout of `## Interaction quality`.** Every
   invariant in that section is a pointer to an AC-### row in the acceptance table, because
   an invariant stated only as a prose bullet there would get no ledger row and would never
   be gated. This is why the section reads as three tables of *carriers* rather than as a
   list of requirements.
8. **The verifying tests are commands and named checklists, not Rust test functions, and
   that is honest rather than a shortfall.** This project compiles no Rust; the testing
   brief's own tier mix says so, and its Tier 2 justification is that "nothing checks whether
   a persona atom is usable". Every `verifying_test` in the ledger therefore names a real
   command, a real grain in `.redkiln/config.yaml`, or a real checklist path in the tree.
