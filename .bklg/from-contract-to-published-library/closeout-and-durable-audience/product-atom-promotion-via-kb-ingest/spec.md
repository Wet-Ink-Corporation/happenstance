---
item: HS-S0132
stage: spec
created: 2026-08-12T13:48:11.607Z
updated: 2026-08-12T13:48:11.607Z
template_sig: 87bbf1d0
rendered_sig: 54723f17
---

# Spec — The audience promoted into .kb/product/ through the ingest path

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-16, AC-15, DoD 16, *Referenced personas & journeys* (`:227-250`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — traceability matrix, DAG, warranted briefs |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-007, AC-008, AC-009; DR-8, DR-9, DR-10; risk rows *Hand-authoring the product atoms* and *The kb-ingest glob and a second wave* |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/spec.md` |
| Key briefs | `.bklg/.../closeout-and-durable-audience/_decomposition.md` — the one warranted (`testing`) brief, and the **amended** evaluator-persona decision that governs the promoted set; `_grounding.md` — what is actually on disk; `_design.md` — **no public API surface**, signed off 2026-08-12 |
| Roadmap pointer | `.bklg/.../closeout-and-durable-audience/_storymap.md` — slice `durable-audience`, merge order position 4 of 5 |

Everything below is scoped to the `durable-audience` slice. This story writes **no Rust**,
touches **no crate**, and changes **no clause** in `spec/SPECIFICATION.md`.

## One-line PR slice

Promote the staged persona and journey drafts into `.kb/product/` **through
`/redkiln:kb-ingest`**, leaving the intake consumed and cleared, a `concept` atom per
persona and a `playbook` atom per journey at `authority_tier: product`, those atoms
indexed in `.kb/maps/domain-map.md`, and `redkiln validate --kb` green on the result.

## Executive summary

`persona-and-journey-intake-staging` (HS-S0131) leaves raw drafts sitting in
`.kb/_intake/`. Drafts are not atoms — `.kb/_intake/README.md` (*"Nothing staged here is
an atom yet"*) and `redkiln validate --kb` both skip `_`-prefixed directories entirely,
so at the end of that story the durable product layer is still exactly what
`initiative.md:227-233` calls it: **structurally present and functionally empty**.

This PR closes that gap and is the only story in the initiative that does. The delta it
lands:

- Seven new atoms under `.kb/product/` — three personas (`kind: concept`) and four
  journeys (`kind: playbook`), all `authority_tier: product` — where today there are
  zero and only a template README.
- A wave audit trail under `.kb/_governance/integration-waves/<waveId>/`, `.kb/_intake/`
  cleared back to its README, and a single wave commit whose diff *is* the provenance
  AC-008 checks for.
- A new domain section in `.kb/maps/domain-map.md`, which today says *"This is the map's
  first wave. One domain exists so far."* (`:35`) — the audience is the second.

What it deliberately does **not** land: any decision about *who* the personas are. That
was settled upstream and this story executes it (see Context pack). Nor does it link the
atoms from the initiative charter — that is `initiative-closeout-readiness` (AC-014).

## Context pack

The load-bearing decisions, stated as decisions. Read this before touching anything;
the deeper artefacts stay behind the anchors table the second pass appends.

**1. The promotion runs through the ingest path, and hand-authoring is the failure mode
the project exists to avoid.** DR-8 makes provenance itself the deliverable: atoms are
staged in `.kb/_intake/` and authored by `/redkiln:kb-ingest`, never typed into
`.kb/product/`. The precedent is commit `0269720`, which reverted a correctly-shaped
hand-written `.kb/` tree because hand-authoring "is not an accelerated version of that
process, it is a different process wearing its directory layout." A seven-file commit
that produces byte-identical atoms without a wave behind it **fails this story**, and it
fails it in a way `redkiln validate --kb` cannot detect — only the commit graph can.
This is also why the remediation path for a wrong atom is a corrected draft and a
*second wave*, never an edit (see decision 6).

**2. Three personas, four journeys — not four personas.** The project's `testing` brief
(`_decomposition.md`, *The evaluator-persona decision (DR-10, resolving AC-010)*) was
**amended by the repository owner on 2026-08-12** and the amendment governs. DR-10's
original consequence ("four persona atoms… any step that assumes three personas is wrong")
is **superseded**. What survived is DR-10 argument 1 — the evaluator's trust-building
mechanism is genuinely different (one-shot public evidence versus revisable contact with
the code over weeks) — which earns the evaluation path a **first-class atom of its own**.
What did not survive is the claim that this makes them a different *person*: every
distinguishing property is a property of a moment, and the same human is an application
author twenty minutes later. So the evaluation path is a **journey**, linked to the
application-author persona, not a fourth persona and *not* a stage buried inside Persona
1's journey either. Do not collapse either half of that: "its own atom" and "not its own
persona" are both binding. Argument 3 (DT-1 option (c) coherence) was discounted
outright and must not be revived as a reason to shape the corpus.

The set, matching the four journeys the charter already names at `initiative.md:241-250`:

| Draft | Atom kind | The journey the charter names |
| ----- | --------- | ----------------------------- |
| Persona 1 — the application author | `concept` | *Choose a contract before a database* |
| Persona 2 — the adapter author | `concept` | *Learn when you are finished* |
| Persona 3 — the local-first / edge Rust developer | `concept` | *Event-source at the edge without hand-rolling it* |
| (no persona of its own) | `playbook` | *Decide in one sitting* — the evaluation path, `related` to the application-author persona |

**3. The secondary-evidence qualification travels in `summary`, per atom, honestly
worded.** AC-009 requires each promoted atom to state *in its own `summary` field* that
its evidence is secondary and no persona was directly observed, with `source_paths`
naming the discovery artefacts it came from. A reader who reads only frontmatter must
not be able to miss it. Two traps: (a) DR-9 quotes a literal sentence beginning "All
four personas…", which the DR-10 amendment made false — AC-009's per-atom formulation is
the governing one, and the atom must say something true of itself; (b) the evidence is
not uniformly thin, and flattening it into one boilerplate clause discards what
`personas-and-journeys.md` *Risks* (`:347-370`) actually recorded — Persona 1 rests on
download counts, Persona 3 on one blog post and one issue thread, and the evaluator is
inferred from research framing rather than any named individual. The brief's argument 4
settles the balance: the evaluation journey gets **the same** disclaimer as its
siblings, not a stronger one — "only the same one, honestly worded."

**4. The mount point is the domain map, not the directory.** An atom under
`.kb/product/` that no index lists is a component that was built and never wired in: a
reader who does not already know the file name will never reach it.
`.kb/maps/domain-map.md`'s own `summary` (`:10-12`) says it is "updated by the Maps phase
of every kb-ingest wave that adds a new canonical concept or domain path", and its body
(`:35`) currently declares one domain. This wave adds the second. Mounted means: a domain
section for the audience, one bullet per atom with its id and a resolvable relative link,
and reciprocal `related` links that `redkiln validate --kb` resolves in both directions.

**5. `validate --kb` will not catch the things this story is most likely to get wrong.**
Three specific gaps, all verified against redkiln 0.19.0:
- `authority_tier` is `z.string().min(1)` — a free-form string. `product` is a *first
  use* of that tier in this corpus (every existing atom is `note`, `guideline` or
  `decision`, per `.kb/_governance/integration-waves/2026-08-10-intake-2/03-integration-summary.md`).
  A typo validates silently. The authority for the value is `.kb/product/README.md:6-9`,
  and it must be asserted, not assumed.
- `KbFrontmatter` is a passthrough object: an invented key validates silently and is
  read as corpus fact by every later wave. Author only the ten fields
  `.kb/_templates/atom.md` lists.
- README files and `_`-prefixed directories are skipped by the atom walk entirely, so a
  draft left behind in `.kb/_intake/` after the run is invisible to validation. The
  contract is `.kb/_intake/README.md`'s: *"a file still sitting here after a run is a
  file that run did not ingest."*

**6. Wave hygiene: the glob, the id, and the branch.** Three concrete mechanics the
project's risk table flags and this story must handle at the Stage-A gate:
- **The glob.** `/redkiln:kb-ingest` with no argument resolves *all* of
  `.kb/_intake/*.md`, and `README.md` matches. Pass an explicit narrowed glob or file
  list so the staging README is not ingested as content — and is not deleted by the
  clear either.
- **The wave id.** Two waves already exist on disk (`2026-08-10-intake`,
  `2026-08-10-intake-2`), each owning a five-file directory under
  `.kb/_governance/integration-waves/`. This wave takes a **distinct** id or it
  overwrites a previous wave's audit trail — which is also the audit trail AC-008 reads.
- **The branch.** The ingest command creates its own worktree and branch
  (`worktree-kb-intake-<date>`), copies the staged files across because they are
  untracked at `HEAD`, commits the whole wave there, and **does not merge**. The atoms
  are not on the closeout tree until that branch is merged onto
  `initiative/from-contract-to-published-library`. Merge it `--no-ff`: the wave commit is
  the object AC-008's `git log --diff-filter=A` provenance check follows, and a squash
  destroys it.

**7. Accepted decisions stay immutable — but a backlink is not an edit.** If a persona
atom cites `kb-decision-0006`, the Maps phase may wire the reciprocal `related` entry
onto that accepted decision. That is allowed: `validate-kb`'s frozen signature covers
`title`, `kind`, `summary`, `authority_tier`, `depends_on`, `adr_id`, `reversibility`,
`phase` and the body — **`related` is deliberately outside it**. Nothing else about an
accepted decision may move (`.kb/decisions/README.md`).

**8. This project fixes nothing.** If the wave surfaces a defect — an inconsistency with
`publication-and-positioning`'s recorded DT-1 resolution, a stale citation in a draft —
it is routed to `findings-disposition-register`, not repaired here
(`project.md` DR-12; `.redkiln/config.yaml:5`).

## Integration contract

- **Archetype**: `capability`. The observable outcome is a reader of the *next*
  initiative inheriting an adjudicated audience instead of re-deriving one from the same
  secondary evidence (AC-15).
- **Slice / milestone**: `durable-audience`. Slice-mate: `persona-and-journey-intake-staging`
  (HS-S0131, `foundation`, `blocked_by` this story's dependency edge). The two are
  implemented in one context and land as one integrated surface — holding them apart is
  exactly how `0269720` happened.
- **Mount point**: **`.kb/maps/domain-map.md`** — the corpus's subject-matter index and
  the only render path by which a reader who does not know the filenames reaches these
  atoms. The wave adds an audience domain section listing all seven atoms. An atom
  present in `.kb/product/` and absent from this file is constructed-but-unmounted.
- **Wires into**:
  - `.kb/product/README.md:6-9` — the layer contract this story's atoms must satisfy
    (`concept`/`product` for a persona, `playbook`/`product` for a journey); `:15-24` —
    that closeout performs the promotion and that every claim is cited in `source_paths`.
  - `.kb/_intake/` and `/redkiln:kb-ingest` — the authoring mechanism (DR-8). The staged
    drafts are HS-S0131's output; this story consumes them and clears the directory.
  - `.kb/_templates/atom.md` — the `KbFrontmatter` field set atoms are authored against.
  - `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`
    and `.../opportunities.md` — the evidence the atoms cite in `source_paths`.
  - `.kb/_governance/integration-waves/2026-08-10-intake-2/` — the shape of the wave
    record this run must also produce (five files: corpus match, claims, placement,
    integration summary, retrospective).
  - Downstream: `backlog-and-kb-health-at-closeout` (HS-S0133) runs `validate --kb` over
    the tree these atoms are now part of; `initiative-closeout-readiness` (HS-S0135)
    links them from the initiative closeout.
- **Renders surfaces**: **none**. The project's signed-off `_design.md` records *no
  public API surface* and `N/A` for every item, signature, state and doctest section —
  a determination approved by the repository owner on 2026-08-12 and grounded three
  times upstream. This story adds, changes and removes zero public items in any crate.
- **Conformance rule(s)**: none, and this is not adapter-observable. No port, value type
  or testkit rule is touched; nothing an adapter could implement differently is changed.
- **Clause(s)**: none. `spec/SPECIFICATION.md` is untouched — no clause is discharged or
  amended, and no `[FROZEN]` clause is approached.
- **Advances DoD scenario**: initiative **DoD 16** — *"The audience is durable. Persona
  and journey atoms exist under `.kb/product/` with valid frontmatter and pass
  validation, and the initiative's closeout links them"* (`initiative.md:405-407`). This
  story lands the first two thirds; the closeout link is AC-014's. It also moves project
  **DoD 4** (AC-007/008/009 → DoD 16, AC-15, BR-16) to green.

## PR boundary

```
.kb/product/**
.kb/maps/**
.kb/_governance/integration-waves/**
.kb/_intake/**
.bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/**
```

**In this PR**

- The Stage-A resolution of the intake set (narrowed glob), the wave id, and the human
  approval gate; the `/redkiln:kb-ingest` run itself.
- Seven new atoms under `.kb/product/`, the wave record under
  `.kb/_governance/integration-waves/<waveId>/`, and `.kb/_intake/` cleared to its README.
- The domain-map mount and whatever reciprocal `related` backlinks the Maps phase wires
  (including onto accepted decision atoms under `.kb/decisions/**` and concept/playbook
  atoms elsewhere under `.kb/` — `related` is outside the frozen signature, so this is a
  legal edit; it is inside the boundary via `.kb/maps/**` only for the map itself, so a
  backlink landing outside these globs is a **deliberate, recorded widening** of this
  block, never a silent one).
- The merge of `worktree-kb-intake-<date>` onto the initiative branch with `--no-ff`, and
  this story's own `_ledger.md`.

**Explicitly not in this PR**

- Any Rust, any crate manifest, any file under `crates/`, `xtask/`, `examples/` or
  `spec/`. This project owns no code (`project.md` *Out of scope*).
- Hand-authoring, hand-editing or hand-correcting any file under `.kb/product/` outside
  the wave commit — the prohibited move, not merely an undesirable one.
- Editing the body or frozen frontmatter of any accepted decision atom.
- Linking the atoms from `initiative.md` (`initiative-closeout-readiness`, AC-014).
- Running `redkiln validate`/`doctor` as the closeout health assertion
  (`backlog-and-kb-health-at-closeout`, AC-011/AC-012) — this story runs `validate --kb`
  only as its own correctness check.
- Fixing anything the wave surfaces (`findings-disposition-register`, AC-013).
- `redkiln adopt --templates`, ever (`CLAUDE.md`).

**Merge DoD**: seven `authority_tier: product` atoms are on the closeout tree, every one
of them introduced by a single `/redkiln:kb-ingest` wave commit, listed in
`.kb/maps/domain-map.md`, carrying the secondary-evidence qualification in `summary`, with
`.kb/_intake/` cleared and `redkiln validate --kb` green.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The intake set is narrowed before the wave runs** | Stage A resolves `.kb/_intake/*.md` by default and `README.md` matches that glob. Pass an explicit file list or a glob that excludes it, so the staging README is neither ingested as content nor removed by the clear. Confirm the resolved list at the approval gate. | `.kb/_intake/README.md` (*"the default input to `/redkiln:kb-ingest`"*, *"`_`-prefixed directories are reserved"*); `project.md` risk row *The kb-ingest glob and a second wave* |
| **The wave id is distinct from every prior wave** | `.kb/_governance/integration-waves/` already holds `2026-08-10-intake` and `2026-08-10-intake-2`. The new id (`<date>-intake`, suffixed if it collides) gets its own five-file directory; reusing an id overwrites the audit trail AC-008 reads. | `.kb/_governance/integration-waves/2026-08-10-intake-2/`; commit `72f2c9b` (*"38 ops, 2026-08-10-intake-2"*) |
| **Seven atoms are created, none merged away or deferred** | Three `concept` personas + four `playbook` journeys. The ingest workflow biases hard toward MERGE-AND-LINK over new atoms and may `defer_open_question`; for a domain the corpus does not yet have, seven creates is the correct plan. A manifest reporting fewer than seven `newAtoms` under `.kb/product/`, a non-empty `degraded`, or any of the seven in `deferredAtoms` means **discard the branch and re-stage** — never hand-complete the set. | `.kb/_governance/integration-waves/2026-08-10-intake-2/03-integration-summary.md` (the *Atoms created* table shape); `project.md` risk row *Hand-authoring the product atoms* |
| **Kind and tier are as the layer contract states** | Persona → `kind: concept`; journey → `kind: playbook`; both `authority_tier: product`. `authority_tier` is a free-form string in `KbFrontmatter`, so a typo passes validation — assert the value, do not infer it from a green `validate --kb`. | `.kb/product/README.md:6-9`; redkiln 0.19.0 `src/schema/kb.ts` (`authority_tier: z.string().min(1)`) |
| **The promoted set matches the amended evaluator decision** | Exactly three persona atoms; exactly four journey atoms, the fourth being the evaluation path as a first-class atom `related` to the application-author persona — not a fourth persona, not a stage inside Persona 1's journey. | `_decomposition.md` *The evaluator-persona decision (DR-10, resolving AC-010)*, amendment of 2026-08-12; `initiative.md:241-250` (the four journeys, already named) |
| **The qualification is in `summary`, per atom, and true of that atom** | Each atom's `summary` states that its evidence is secondary and that no persona was directly observed; `source_paths` names the discovery artefacts. Same disclaimer strength across all seven; wording honest about what each rests on rather than a copied sentence that counts four personas. | `project.md` DR-9 / AC-009; `personas-and-journeys.md:347-370` (*Risks* — the differential evidence, and the "none directly interviewed or observed" bullet); `.kb/product/README.md:23-24` |
| **Only the ten authored frontmatter keys appear** | `id`, `title`, `kind`, `status`, `authority_tier`, `summary`, `depends_on[]`, `related[]`, `source_paths[]`, `last_reviewed`. `KbFrontmatter` is passthrough, so an invented key validates silently and becomes corpus fact. Ids follow the corpus convention: `kb-concept-<slug>-001`, `kb-playbook-<slug>-001`. | `.kb/_templates/atom.md`; existing ids in `.kb/_governance/integration-waves/2026-08-10-intake-2/03-integration-summary.md` |
| **The atoms are mounted in the domain map** | A new domain section in `.kb/maps/domain-map.md` listing all seven with id and relative link, alongside the existing *Specification governance & conformance* domain. Reciprocal `related` links resolve both ways. | `.kb/maps/domain-map.md:10-12` (Maps phase updates it), `:35` (one domain so far) |
| **Accepted decisions are not edited** | A reciprocal `related` backlink onto an accepted decision atom is permitted; nothing else is. `validate --kb`'s frozen signature covers title/kind/summary/authority_tier/depends_on/adr_id/reversibility/phase and body — `related` is outside it by design. | `.kb/decisions/README.md`; redkiln 0.19.0 `src/store/validate-kb.ts` (`frozenSignature`) |
| **Intake is consumed and cleared** | After the run `.kb/_intake/` contains its README and nothing else. A draft still sitting there is a draft that was not ingested. | `.kb/_intake/README.md` *A successful ingest clears this directory* |
| **`redkiln validate --kb` is green, and is the authority** | Run it from inside the ingest worktree and again on the merged closeout tree. Relay its verdict verbatim; do not hand-check frontmatter in its place, and do not report a problem it did not report. `--kb` adds to the default run, so it also re-validates config, pack and `.bklg`. | `/redkiln:kb-ingest` Stage B step 7; `project.md` AC-007 |
| **Provenance survives to the closeout tree** | Merge `worktree-kb-intake-<date>` with `--no-ff`. `git log --diff-filter=A -- .kb/product/` must attribute every atom to the wave commit, and no other commit in the range may add or modify a file under `.kb/product/`. | `project.md` AC-008 / DR-8; `.redkiln/config.yaml:73` (`require_commit_provenance`) |
| **A wrong atom is fixed by a second wave, not a keystroke** | If an atom lands with a wrong `summary`, a missing `source_paths` entry or the wrong tier, correct the *intake draft* and run a second wave under a further-distinct id. Editing the atom in place would put a hand-authored commit on a `.kb/product/` path and fail AC-008 permanently — the commit graph does not forget. | `project.md` DR-8; commit `0269720` |
| **Findings are routed, not repaired** | Anything the wave surfaces — a stale citation, an inconsistency with `publication-and-positioning`'s recorded DT-1 resolution — is recorded for `findings-disposition-register`. | `project.md` DR-12, AC-013; `.redkiln/config.yaml:5` |

## Data and migrations

No database, no schema migration, no code. There *is* a data contract, and it is the
whole deliverable: seven markdown atoms whose frontmatter must satisfy `KbFrontmatter`
and the `.kb/product/` layer convention simultaneously — the first is machine-checked,
the second is not.

**Destination shape.** One file per atom under `.kb/product/`, named by slug, each
carrying:

| Field | Value for these atoms |
| --- | --- |
| `id` | `kb-concept-<slug>-001` (persona) / `kb-playbook-<slug>-001` (journey), unique across the corpus — duplicate ids are rejected by `validate --kb` |
| `kind` | `concept` (persona) / `playbook` (journey) |
| `status` | `accepted` — these are promoted findings, not drafts; a draft belongs in `_intake/` |
| `authority_tier` | `product` — **first use of this tier in this corpus**, and unchecked by the schema |
| `summary` | carries the secondary-evidence qualification (AC-009) |
| `depends_on` / `related` | the journey↔persona links, including the evaluation journey's link to the application-author persona; every id must resolve to a real atom |
| `source_paths` | the discovery artefacts (`.bklg/.../_discovery/distillation/personas-and-journeys.md`, `.../opportunities.md`) **and** the `.kb/_intake/…` draft path the wave consumed, per the ingest contract |
| `last_reviewed` | the wave date |

**The one destructive operation** is the intake clear: `.kb/_intake/*.md` is removed as
part of the wave commit. It is reversible by construction — the staged sources stay in
git history on the initiative branch, and every atom cites its `.kb/_intake/…` path in
`source_paths`. The one thing that must survive it is `.kb/_intake/README.md`, which is
scaffolding rather than staged content and is why the glob is narrowed at Stage A.

**No backfill, no compatibility shim.** `.kb/product/` is empty today, so nothing
pre-existing needs reconciling, and no existing atom's frozen content changes — the only
edits outside `.kb/product/` are the domain-map section and reciprocal `related` entries,
both additive.

## Acceptance criteria

The "user" of this story is the reader of the **next** initiative — the planner who opens
`.kb/product/` expecting an adjudicated audience, and the reviewer who has to believe it
arrived honestly. Both are named in the charter (`initiative.md:250-253`, AC-15) and both
are why `.kb/product/README.md:24` says a persona nobody researched "is a stock photo with
a name". Every criterion below is framed from one of them crossing the whole path —
staged draft → wave → merged tree → the index they will actually navigate by.

`<base>` throughout is the initiative branch's merge-base for this story's PR;
`_promotion-record.md` is this story's companion transcript, authored beside this spec at
`.bklg/from-contract-to-published-library/closeout-and-durable-audience/product-atom-promotion-via-kb-ingest/_promotion-record.md`
and cited by the ledger. It does not exist yet — the implementer creates it, and it is the
artefact, not the proof: every row below names a command whose output it records verbatim.

| id | criterion | verification |
| -- | --------- | ------------ |
| AC-001 | **GIVEN** a planner opening the next initiative who has never read this initiative's `_discovery/`, **WHEN** they list `.kb/product/` on the closeout tree, **THEN** they find seven atoms — three `kind: concept` personas and four `kind: playbook` journeys, every one `status: accepted` and `authority_tier: product` — and `redkiln validate --kb` exits zero over the corpus containing them, so what they inherit is a checked layer and not a folder of drafts. (project AC-007) | **Static.** `redkiln validate --kb` on the merged closeout tree, exit code and verdict transcribed verbatim into `_promotion-record.md` §Validation; plus a per-file frontmatter assertion over `.kb/product/*.md` (`rg -n "^(id\|kind\|status\|authority_tier):" .kb/product`) showing seven files, three `concept`, four `playbook`, seven `product` — asserted, not inferred from a green run, because `authority_tier` is a free-form string in `KbFrontmatter`. Recorded in `_promotion-record.md` §Atoms. |
| AC-002 | **GIVEN** the repository owner's amendment of 2026-08-12, which held that the evaluator's *mechanism* of trust-building earns a first-class atom while the evaluator remains the same human as the application author, **WHEN** a reader looks for the evaluation path, **THEN** they find it as a **journey atom of its own** whose `related` names the application-author persona atom — not a fourth persona, and not a stage folded inside *Choose a contract before a database* — and the persona set is exactly the application author, the adapter author and the local-first / edge developer. (project AC-007; honours AC-010's already-discharged decision) | **Static.** Enumeration table in `_promotion-record.md` §Set, one row per atom mapping file → `kind` → the charter journey it realises (`initiative.md:241-250`), checked against `_decomposition.md` *The evaluator-persona decision*, amendment. The evaluation journey's `related` edge to the application-author persona id must resolve — `redkiln validate --kb` fails an unresolvable id, so a green run over an enumeration of seven is the check for both halves. |
| AC-003 | **GIVEN** a reviewer who does not trust a correctly-shaped `.kb/` tree on sight, because `0269720` was exactly that and was reverted, **WHEN** they run `git log --diff-filter=A --format='%h %s' <base>..HEAD -- .kb/product/`, **THEN** every one of the seven atoms is attributed to a **single `/redkiln:kb-ingest` wave commit**, and no other commit in the range adds or modifies a file under `.kb/product/` — the provenance is the deliverable, not the file shapes. (project AC-008; DR-8) | **Process.** The two `git log` runs (`--diff-filter=A`, then unfiltered `-- .kb/product/`) with their full output in `_promotion-record.md` §Provenance, plus the merge command used (`git merge --no-ff worktree-kb-intake-<date>`), because a squash collapses the wave commit this check follows. `.redkiln/config.yaml:73` (`require_commit_provenance`) ties the same commit to this story's links. |
| AC-004 | **GIVEN** the `_intake` contract that "a file still sitting here after a run is a file that run did not ingest" (`.kb/_intake/README.md:13-19`), **WHEN** the reviewer lists `.kb/_intake/` after the merge, **THEN** it contains `README.md` and nothing else — the staging README survived the narrowed glob and was neither ingested as content nor deleted by the clear — and each of the seven atoms cites the `.kb/_intake/…` draft path it consumed in `source_paths`, so the drafts are recoverable from history rather than merely gone. (project AC-008) | **Process + static.** `git ls-files .kb/_intake/` on the merged tree (exactly one path); `rg -n "_intake" .kb/product` showing a staged-source citation in every atom; and the Stage-A resolved input list — the explicit file list or narrowed glob confirmed at the approval gate — pasted into `_promotion-record.md` §Wave inputs. |
| AC-005 | **GIVEN** a reader who reads only frontmatter — the failure mode DR-9 was written against — **WHEN** they read any one of the seven `summary` fields, **THEN** that atom tells them its evidence is secondary and that no persona was directly observed, and its `source_paths` names the discovery artefacts it rests on; the sentence is true *of that atom* (no summary asserts a four-persona set), the disclaimer is the same strength across all seven including the evaluation journey, and it is not in the body only. (project AC-009) | **Static.** `rg -n "^summary:" -A 6 .kb/product/*.md` with all seven summaries quoted in `_promotion-record.md` §Qualification, each checked for (a) the secondary-evidence statement, (b) the not-directly-observed statement, (c) no "all four personas" claim; plus `rg -n "source_paths" -A 4 .kb/product` showing `personas-and-journeys.md` / `opportunities.md` cited. `redkiln validate --kb` covers the field's presence and shape, never its truth — the quoted transcript is what covers the rest. |
| AC-006 | **GIVEN** a reader who does not already know the filenames — the only reader an index exists for — **WHEN** they open `.kb/maps/domain-map.md`, **THEN** a new `##` audience domain section lists all seven atoms with id and a relative link that resolves, appended rather than folded into an existing section per the map's own *Adding a domain* rule (`:144-150`), and the reciprocal `related` edges resolve in both directions. An atom present in `.kb/product/` and absent here is constructed-but-unmounted and fails this AC. (project AC-007 — the mount) | **Static.** Diff of `.kb/maps/domain-map.md` in the wave commit showing a new `##` section (never an edit that absorbs the audience into *Specification governance & conformance*); a link-resolution pass over that section's seven relative links (`git ls-files` each target); `redkiln validate --kb` for the id resolution in both directions. Recorded in `_promotion-record.md` §Mount. |
| AC-007 | **GIVEN** an auditor at closeout who reads wave records to reconstruct how the corpus grew, **WHEN** they list `.kb/_governance/integration-waves/`, **THEN** a **third** directory under an id distinct from `2026-08-10-intake` and `2026-08-10-intake-2` holds this wave's five-file record in the shape the previous wave produced, and the two existing wave directories are byte-unchanged — a reused id would silently overwrite the audit trail AC-003 reads. (project AC-008) | **Process.** `git diff --stat <base>..HEAD -- .kb/_governance/integration-waves/2026-08-10-intake .kb/_governance/integration-waves/2026-08-10-intake-2` must be **empty**; `ls` of the new wave directory showing the five files (`00-corpus-match.md` … `04-retrospective.md`). Both in `_promotion-record.md` §Wave record. |
| AC-008 | **GIVEN** the two prohibitions this project is most likely to breach under time pressure — hand-completing a partial wave, and editing an accepted decision — **WHEN** the reviewer reads the whole PR diff, **THEN** the only changes outside `.kb/product/` and the wave record are the domain-map section and additive `related:` backlinks, no accepted decision atom's frozen signature (`title`, `kind`, `summary`, `authority_tier`, `depends_on`, `adr_id`, `reversibility`, `phase`, body) has moved, and every defect the wave surfaced appears in `_promotion-record.md` §Findings routed to `findings-disposition-register` with **zero** fixed here. (project AC-008; DR-12) | **Process.** `git diff <base>..HEAD -- .kb/decisions/` reviewed line by line — only `related:` list entries may appear; `git diff --stat <base>..HEAD` with every path outside the PR boundary named and justified (a Maps-phase backlink landing outside is a recorded widening, never silent); the findings list with a destination per row, and the assertion that no commit in the range repairs one. Recorded in `_promotion-record.md` §Diff review and §Findings. |

Coverage: project **AC-007** → AC-001, AC-002, AC-006; project **AC-008** → AC-003,
AC-004, AC-007, AC-008; project **AC-009** → AC-005. No story AC is orphaned and none
belongs to a project AC this story does not carry.

## Interaction quality

**Composition family: not applicable, and declared rather than skipped.** The project's
signed-off `_design.md` records **no public API surface** and `N/A` for every items,
signatures, states, anti-patterns and doctest section, approved by the repository owner on
2026-08-12 at the plan design gate, with the determination grounded three times upstream
(`_design.md:10-39`, `:89-100`). This story renders no screen, adds no public Rust item,
and `design.capture` is deliberately absent from `.redkiln/config.yaml:75-83`, which makes
the perceptual review a declared skip. There is therefore no density budget, no transience
policy and no named visual anti-pattern to bind — asserting one here would invent a design
a human never approved.

**State family: applicable, translated to this story's medium.** The medium is a durable
markdown corpus that readers navigate by index and reviewers navigate by diff, and the
state invariants have exact analogues in it. Every one that applies is carried by a row in
the table above — none is stated only here, because `redkiln verify` extracts ACs from
`| AC-### |` cells and a bullet in this section would never be gated.

| Invariant | Carried by | How it is verified |
| --------- | ---------- | ------------------ |
| **Non-occlusion** — new work must not paint over existing state | **AC-007** (a distinct wave id; the two prior wave directories byte-unchanged), **AC-006** (a new `##` section appended, never an existing domain rewritten to absorb the audience), **AC-008** (accepted decision bodies untouched) | `git diff --stat` over the prior wave dirs is empty; the domain-map diff shows an appended section; `.kb/decisions/` diff shows only `related:` lines |
| **Preserved context across a destructive step** — the one destructive operation is the intake clear | **AC-004** (`.kb/_intake/README.md` survives it; every atom cites the draft path it consumed) | `git ls-files .kb/_intake/` returns exactly the README; `rg "_intake" .kb/product` finds a citation in all seven |
| **Reachability** — the artefact must be reachable by someone who does not already know where it is | **AC-006** (the domain-map mount, with links that resolve) | link-resolution pass over the new section; `redkiln validate --kb` for the reciprocal ids |
| **Reversibility** — a wrong result is undone by the mechanism, not by hand | **AC-003** and **AC-008** (a wrong atom is corrected by a corrected draft and a *second* wave under a further-distinct id; an in-place edit would put a hand-authored commit on a `.kb/product/` path and fail AC-003 permanently) | `git log --diff-filter=A -- .kb/product/` still attributing every atom to a wave commit after any correction |
| **In-place vs context-jump** — the reader's frame of reference must not move under them | **AC-002** (the evaluation path lands as its own atom in the place a reader looks for a journey, rather than being buried mid-way through another persona's journey where only someone who already knew would find it) | the enumeration table: one atom per journey the charter names, none nested inside another |
| **Non-occlusion of the record's own evidence** — a claim must not hide the run behind it | **AC-001, AC-003, AC-005** (verbatim command output in `_promotion-record.md`, never a paraphrase or a tick) | the transcript sections named in each row |

## Error conditions

| id | Condition | Required handling |
| -- | --------- | ----------------- |
| **EC-001** | `/redkiln:kb-ingest` invoked with no argument resolves all of `.kb/_intake/*.md`, and `README.md` matches — the staging README is ingested as content and removed by the clear. | Narrow the wave at Stage A to an explicit file list or a glob that excludes `README.md`, and confirm the **resolved** list at the approval gate before the wave runs (`.kb/_intake/README.md:3-5`). If it was ingested, discard the branch — do not delete the resulting atom by hand. |
| **EC-002** | The chosen wave id collides with `2026-08-10-intake` or `2026-08-10-intake-2`, overwriting a prior wave's five-file record. | Pick `<date>-intake`, suffixed until distinct, and check the directory does not exist *before* the run. If a prior record was overwritten, restore it from `<base>` and re-run under a new id; AC-007 fails until `git diff --stat` over both prior directories is empty. |
| **EC-003** | The ingest manifest reports fewer than seven `newAtoms` under `.kb/product/`, any of the seven in `deferredAtoms`, or a non-empty `degraded` — the workflow's hard bias toward MERGE-AND-LINK misfiring on a domain the corpus does not yet have. | **Discard the branch and re-stage.** Never hand-complete the set: a hand-written seventh atom is the `0269720` failure with six-sevenths of an alibi, and AC-003 detects it while `validate --kb` cannot. |
| **EC-004** | The wave branch is merged with `--squash` or rebased flat, destroying the single wave commit. | Merge `worktree-kb-intake-<date>` with `--no-ff`. If the commit is already gone, the provenance is not recoverable by editing history on a shared branch — re-run the wave from the staged drafts under a further-distinct id and merge that correctly. |
| **EC-005** | `authority_tier` is misspelled (`Product`, `products`, `produce`). `KbFrontmatter` types it `z.string().min(1)`, so `validate --kb` passes and every later wave reads the typo as corpus fact. | Assert the literal value across all seven files (AC-001's `rg` check). This is the **first use** of the `product` tier in this corpus — no existing atom would contradict a typo. |
| **EC-006** | An invented frontmatter key (`persona:`, `journey_of:`, `evidence_tier:`) is authored. `KbFrontmatter` is a passthrough object, so it validates silently. | Author only the ten fields `.kb/_templates/atom.md` lists; the decision-only fields (`adr_id`, `reversibility`, `phase`, `supersedes`, `superseded_by`) are deleted for non-decision kinds, as that template instructs. |
| **EC-007** | A `related` / `depends_on` id does not resolve, or an atom id duplicates one already in the corpus. | `redkiln validate --kb` fails and is the authority — relay its message verbatim, fix the **draft**, re-run the wave. Ids follow `kb-concept-<slug>-001` / `kb-playbook-<slug>-001`. |
| **EC-008** | The wave proposes to amend an accepted decision atom's body or frozen frontmatter (its adjudication step may prefer AMEND). | Refuse the amendment and let the wave supersede or link instead. Only a reciprocal `related` entry may land on an accepted decision — `related` sits outside the frozen signature by design (`.kb/decisions/README.md`). |
| **EC-009** | An atom lands with a wrong `summary`, a missing `source_paths` entry or the wrong tier, discovered after the merge. | Correct the intake **draft** and run a second wave under a further-distinct id. Editing the atom in place fails AC-003 permanently — the commit graph does not forget. |
| **EC-010** | The staged drafts are absent or stale because `persona-and-journey-intake-staging` has not merged onto the initiative branch. | Stop. The ingest command creates its own worktree and copies the staged files across because they are untracked at `HEAD`; running it against an empty or partial `_intake/` produces a wave that is green and wrong. Verify the seven drafts are present and current before Stage A. |
| **EC-011** | The wave surfaces a defect — a stale citation in a draft, an inconsistency with `publication-and-positioning`'s recorded DT-1 resolution, a sibling planning artefact still counting four personas. | Record it in `_promotion-record.md` §Findings with its destination and **do not fix it** (`project.md` DR-12, AC-013; `.redkiln/config.yaml:5`). A repair inside this story fails AC-008 and the project's own AC-013. |

## Non-functional

| id | Requirement | Why, and where it bites |
| -- | ----------- | ---------------------- |
| **NF-001** | This story's diff maps to **no workspace package**. `cargo xtask affected --base main` (the story grain wired at `.redkiln/config.yaml:40`) must still exit zero: it runs the five file-reading lints and `spec-trace` unconditionally, precisely so a story whose whole deliverable is markdown is not "green by compiling nothing" (`.redkiln/config.yaml:36-39`). | A `.kb/` path that breaks a cross-reference the lints read would otherwise land invisibly. |
| **NF-002** | Each atom is **self-sufficient to a reader with no access to this backlog**. `.bklg/` paths appear in `source_paths` as provenance, never as the only place the claim is legible; the atom states the persona's goal, context, what they do instead and what they fear (`.kb/product/README.md:11-13`) in its own body. | The whole point of promotion is that the next initiative inherits the audience without re-reading a closed initiative's discovery (AC-15). |
| **NF-003** | **One idea per atom.** One persona per `concept` atom, one journey per `playbook` atom; no atom bundles two personas or narrates two journeys, and no journey atom transcribes an interface — a journey says what the persona is trying to accomplish and in what order, never which control they click (`.kb/product/README.md:32-36`). | The corpus's own norm; also what keeps the evaluation journey a *journey* rather than a persona in disguise. |
| **NF-004** | The wave runs behind its **human approval gate** with the resolved input list, the wave id and the target branch shown before execution. This story is `automation: HITL` like every item in this repository (`.redkiln/config.yaml:3`). | EC-001 and EC-002 are both catchable only at that gate; after the run they are cleanup. |
| **NF-005** | The result is **re-verifiable by a third party from the merged tree alone** — every command in the Tests table re-runs on a fresh clone with no local state, tool or credential, and yields the same verdict. | `backlog-and-kb-health-at-closeout` re-runs `validate --kb` over this tree, and the closeout reader must be able to repeat it. |
| **NF-006** | No `.kb/` file outside the wave commit is touched by a human in this PR — including the domain map. The map is edited **by the Maps phase of the wave**, which is what its own `summary` says maintains it (`.kb/maps/domain-map.md:10-12`). | A hand-edited map in a separate commit is the same category error as a hand-written atom, one directory over. |

## Implementation notes (non-prescriptive)

Not prescriptive; the shape below is what the constraints leave standing, and a better
route that satisfies the ACs is welcome.

- **Sequence.** Confirm the seven drafts are present on the initiative branch (EC-010) →
  choose and verify a distinct wave id (EC-002) → resolve the narrowed input list and read
  it back at the approval gate (EC-001) → run `/redkiln:kb-ingest` → read the returned
  manifest against the seven-atom expectation (EC-003) → `redkiln validate --kb` inside the
  ingest worktree → merge `--no-ff` → re-run `redkiln validate --kb` on the closeout tree →
  capture the commands in `_promotion-record.md` as they run, not from memory afterwards.
- **The manifest is the first gate, not the diff.** `forge-kb-ingest` returns
  `{ newAtoms, deferredAtoms, mutatedAtoms, maps, degraded }`. Reading it before reading the
  files is what turns EC-003 from a post-merge discovery into a pre-merge decision.
- **Expect the Maps phase to want backlinks.** Reciprocal `related` entries onto atoms
  outside `.kb/product/` are legal and are the reason the audience is reachable from more
  than one direction. Each one that lands outside the PR boundary globs is named in
  `_promotion-record.md` §Diff review — widening the boundary deliberately is fine;
  widening it silently is what AC-008 catches.
- **`.kb/maps/domain-map.md:35` says "This is the map's first wave. One domain exists so
  far." while the file already carries two `##` domain sections (`:37`, `:82`).** The
  audience is the third. The sentence is stale prose in a map atom, and a map atom is
  wave-maintained — if the Maps phase corrects it, it does so inside the wave commit; a
  hand-edit outside the wave is NF-006's violation, and leaving it stale is a finding for
  EC-011, not a defect this story repairs by keystroke.
- **Where the record lives.** `_promotion-record.md` sits in this story's own directory
  rather than the project's `_closeout-record.md`: that companion is where the DoD set, the
  delta, the audit tables and the findings converge (`_storymap.md:24-30`), and this story
  is not one of them. Its findings row is what reaches `findings-disposition-register`.
- **Do not run `redkiln adopt --templates`, ever** (`CLAUDE.md`), and do not run
  `redkiln advance` or `redkiln new` — the orchestrating command owns every transition.

## Tests and CI (merge gate)

Grounded in the project's `testing` brief (`_decomposition.md` *Testing brief →
Acceptance Criteria*, AC-007/008/009 rows, and *Notes → Test mix, summarised by tier*).
"Static" reads frontmatter/config without executing code under test; "process" reads the
commit graph or backlog state; both are load-bearing tiers for a project whose deliverable
is verification. This story adds **no new automated test** and no code — it runs existing
checks on the tree it produced and records what they showed.

| Tier | Command / path | Proves |
| ---- | -------------- | ------ |
| Static | `redkiln validate --kb` — run inside the ingest worktree **and** again on the merged closeout tree; verdict transcribed verbatim | AC-001 (schema conformance of all seven), AC-002 and AC-006 (every `related`/`depends_on` id resolves), EC-007. The authority — never hand-check frontmatter in its place, and never report a problem it did not report |
| Static | `rg -n "^(id\|kind\|status\|authority_tier):" .kb/product` over the seven files | AC-001, EC-005 — three `concept`, four `playbook`, seven `product`, seven `accepted`, ids unique and following `kb-concept-<slug>-001` / `kb-playbook-<slug>-001` |
| Static | `rg -n "^summary:" -A 6 .kb/product/*.md` and `rg -n "source_paths:" -A 4 .kb/product` | AC-005 — the qualification is in `summary` itself and true of that atom; `source_paths` names `_discovery/distillation/personas-and-journeys.md` and `.../opportunities.md`, and AC-004's staged-draft path |
| Static | Field-set diff of each atom's frontmatter against `.kb/_templates/atom.md`'s ten fields | EC-006 — no invented key riding in on the passthrough schema |
| Static | Link-resolution pass over the new `##` section of `.kb/maps/domain-map.md` (`git ls-files` each relative target) | AC-006 — mounted and navigable, not merely present |
| Process | `git log --diff-filter=A --format='%h %s' <base>..HEAD -- .kb/product/`, then the same range unfiltered | AC-003 — one wave commit adds all seven; nothing else in the range touches `.kb/product/` |
| Process | `git ls-files .kb/_intake/` on the merged tree | AC-004 — exactly `README.md`; the intake is consumed and cleared |
| Process | `git diff --stat <base>..HEAD -- .kb/_governance/integration-waves/2026-08-10-intake .kb/_governance/integration-waves/2026-08-10-intake-2` (must be empty) plus `ls` of the new wave directory | AC-007 — a distinct id, five wave files, no prior audit trail overwritten |
| Process | `git diff <base>..HEAD -- .kb/decisions/` reviewed line by line; `git diff --stat <base>..HEAD` with every out-of-boundary path named | AC-008, EC-008 — only `related:` additions on accepted atoms; boundary widening recorded, not silent |
| Process | `_promotion-record.md` §Findings — one destination per finding, zero repaired | AC-008, EC-011 — routing is the deliverable (`project.md` DR-12) |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | NF-001 — the file-reading lints and `spec-trace` run even though this diff maps to no package |
| Story gate | `redkiln verify --grain story` over `_ledger.md` (`.redkiln/config.yaml:67`, `:73`) | Every AC-### above carries cited evidence and a recorded work commit before `implement → report` |
| Project gate (not this story's bar) | `cargo xtask ci` — the terminal grain at `.redkiln/config.yaml:60`, owned by `whole-gate-green-on-the-assembled-tree`; `redkiln validate`/`doctor` owned by `backlog-and-kb-health-at-closeout` | Named so the implementer does not re-run them here and does not mistake this story's `validate --kb` for the project's closeout health assertion (AC-011/AC-012) |

## Risks and coupling (PR-scoped)

| Risk / coupling | Note |
| --- | --- |
| **The fastest route is the forbidden one** | Seven markdown files can be typed in twenty minutes and would pass `validate --kb` perfectly. `0269720` is the precedent for why that is a different process wearing the same directory layout, and AC-003 is the only check that can see the difference. The risk peaks *after* a partially-successful wave (EC-003), where hand-writing one missing atom feels like finishing rather than forging. |
| **Slice coupling with `persona-and-journey-intake-staging`** | The two stories are one slice implemented in one context. The drafts are untracked at `HEAD` until that story's commit lands, and the ingest command copies staged files into its own worktree — so the ordering is real, not ceremonial (EC-010). If the staged set is three personas and three journeys, the evaluation journey is missing and AC-002 fails; check the set before the wave, not after. |
| **The wave runs on a branch this PR must merge** | `/redkiln:kb-ingest` commits on `worktree-kb-intake-<date>` and **does not merge**. Until that branch is merged `--no-ff` onto `initiative/from-contract-to-published-library`, the atoms do not exist for any downstream story, and every check in the Tests table reports honestly that they do not (EC-004). |
| **Maps-phase backlinks reach outside the declared globs** | Reciprocal `related` entries may land on `.kb/decisions/**`, `.kb/concepts/**` or `.kb/playbooks/**`. That is legal (the `related` field is outside the frozen signature) and desirable, but it widens the PR boundary — AC-008 requires each such path to be named, which converts a silent widening into a recorded one. |
| **First use of `authority_tier: product`** | Every existing atom is `note`, `guideline` or `decision`. Nothing in the corpus contradicts a typo and the schema accepts any non-empty string (EC-005). This is the one field where a green `validate --kb` is actively misleading. |
| **Downstream stories consume this tree** | `backlog-and-kb-health-at-closeout` (HS-S0133) runs `validate --kb` over a corpus that now includes these atoms, so a schema defect here surfaces as a health failure there and costs two stories. `initiative-closeout-readiness` (HS-S0135) links these atoms from the initiative closeout, so their ids are load-bearing outside this PR and must not be renamed after the merge — a rename is a supersession, not an edit. |
| **Sibling planning artefacts still count four personas** | `_storymap.md` *Backbone* row D ("Four persona atoms") and *Grain notes* ("the evaluator is its own persona, so the promoted set is **four** personas") predate the 2026-08-12 amendment; the storymap's own row for this story was corrected, those two were not. The amendment governs (`_decomposition.md` *The evaluator-persona decision*). Correcting a sibling planning artefact is outside this PR's boundary — record it as a finding (EC-011). |
| **`validate --kb` cannot see the things most likely to be wrong** | Provenance, tier spelling, invented keys, an intake file left behind, and whether a `summary` sentence is *true*. Each is carried by an explicit AC above rather than delegated to the tool, because the tool's green is the same green either way. |

## Dependencies

**Blocks on**

- `persona-and-journey-intake-staging` (HS-S0131, `foundation`, same `durable-audience`
  slice) — its staged drafts under `.kb/_intake/` are this story's only input, and they
  must be on the initiative branch before the wave runs (EC-010). This is the story's sole
  `depends_on` edge and matches `story.md`'s `blocked_by: [HS-S0131]`.

**Unlocks**

- `backlog-and-kb-health-at-closeout` (HS-S0133, `closeout-health-and-disposition`) — runs
  `redkiln validate`, `validate --kb` and `doctor --json` over the tree these atoms are now
  part of; the storymap sequences slice 4 before slice 5 for exactly this reason
  (`_storymap.md:149-151`).
- `initiative-closeout-readiness` (HS-S0135) — AC-014's "the initiative's closeout links
  the promoted product atoms" has nothing to link until this story merges.

**Not a dependency, deliberately**: the `assembled-tree-gate`, `dod-re-observation` and
`answers-audit` slices. This story shares no artefact with them and can merge before or
after; the storymap's ordering keeps the two `.kb/`-writing slices adjacent so one
`validate --kb` at the head of slice 5 covers both.

## Anchors (progressive disclosure)

Deferred, not optional. The Context pack above is sufficient to start; open each of these
at the moment named.

| Anchor | Why it is load-bearing | When to open | Serves |
| ------ | ---------------------- | ------------ | ------ |
| `.kb/product/README.md` | The layer contract: persona → `concept`, journey → `playbook`, both `authority_tier: product` (`:6-9`); closeout performs the promotion and every claim is cited in `source_paths` (`:15-24`); and what does **not** belong here — an unevidenced sketch, a screen or flow, requirements, a market segment (`:26-44`), which is the bar each atom body must clear | Before authoring or reviewing any atom body; again when checking a journey has not transcribed an interface | AC-001, AC-005, NF-002, NF-003 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` | The `testing` brief. Its 2026-08-12 **amendment** to DR-10 (`:148-181`, consequence at `:227-242`) is the governing statement of the promoted set — three personas, four journeys, argument 3 discounted; its AC-007/008/009 rows (`:50-52`) define the tier each check belongs to | Before AC-002's enumeration, and before writing any test that counts personas | AC-002, AC-005 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | The evidence the atoms cite and must not overstate. *Risks* (`:347-370`) records the differential evidence — Persona 3 on one blog post and one issue thread, Persona 4 inferred from research framing, "none directly interviewed or observed" — which is what makes AC-005's per-atom wording honest rather than boilerplate | While drafting each `summary`, and when checking AC-005's transcript | AC-005, AC-002 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/opportunities.md` | The archetypes behind the personas; the second `source_paths` citation every atom carries | While filling `source_paths` | AC-005 |
| `.kb/_intake/README.md` | The ingest contract in its own words: `_intake/*.md` is the default input and a glob narrows it (`:3-5`); a successful ingest **clears** the directory and a file still sitting there was not ingested (`:13-19`); `_`-prefixed trees are invisible to `validate --kb` (`:21-27`), which is why a leftover draft is undetectable by the tool | At Stage A when resolving the input list, and again when checking the cleared directory | AC-004, EC-001 |
| `.kb/_templates/atom.md` | The exact ten authored frontmatter fields, and the instruction to delete the decision-only block for non-decision kinds. `KbFrontmatter` is passthrough, so this file — not the validator — is the field-set authority | Before the drafts are finalised; again for the field-set diff | AC-001, EC-006 |
| `.kb/maps/domain-map.md` | The mount point. Its `summary` (`:7-12`) states the Maps phase of a wave maintains it; *Adding a domain* (`:144-150`) says append a new `##` section, group by kind, cite each atom's id beside its link, keep the orientation line to one sentence — the shape AC-006 is checked against; `:35` is the stale count noted in the implementation notes | When reviewing the wave's map edit | AC-006, NF-006 |
| `.kb/_governance/integration-waves/2026-08-10-intake-2/03-integration-summary.md` | The previous wave's integration summary — the *Atoms created* table shape this wave's record must match, and the corpus's existing id and `authority_tier` conventions against which `product` is a first use | When reading the manifest and reviewing the new wave record | AC-007, AC-001 |
| `.kb/decisions/README.md` | Why an accepted atom is immutable and what the frozen signature covers — the reason a reciprocal `related` backlink is legal and any other edit is not | Only if the wave proposes to amend a decision atom (EC-008) | AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` | AC-007/008/009 verbatim, DR-8/DR-9/DR-12, and the risk rows *Hand-authoring the product atoms* and *The kb-ingest glob and a second wave* this story's error conditions expand | When a criterion's intent is disputed, or before routing a finding | AC-003, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` | Slice membership, merge order (slice 4 of 5, `:149-151`), and the note that the two `durable-audience` stories land as one surface. Its *Backbone* row D and *Grain notes* bullet still count four personas and are stale against the amendment | When sequencing the slice, or when a sibling artefact appears to contradict AC-002 | AC-002 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` | The signed-off no-public-API-surface determination (`:10-39`, `:89-100`) — the authority for the composition family being N/A rather than skipped | Only if a reviewer asks why no composition invariants are bound | Interaction quality |
| `.redkiln/config.yaml` | `:5` `support_initiative` (where findings route), `:36-40` the story grain and why it lints unconditionally, `:60` the terminal grain this story must not run, `:67`/`:73` `require_ledger` and `require_commit_provenance`, `:75-83` the declared design-capture skip | When wiring the ledger, and when deciding which gate command belongs to this story | NF-001, AC-003, AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` | *Referenced personas & journeys* (`:227-253`): the durable layer is "structurally present and functionally empty", the four journeys by name, and the flag for promotion at closeout — the charter text AC-002's enumeration is checked against | Before AC-002's enumeration table | AC-002, AC-001 |
| `CLAUDE.md` | The `.kb/` vs `.bklg/` split, that atoms are authored by `/redkiln:kb-ingest` and not by hand ("the directory layout of the process without the process", `0269720`), the CLI-is-the-only-writer rule, and the standing `adopt --templates` prohibition | Before the first write of the story, and any time a shortcut looks attractive | AC-003, AC-008 |

## Clarifications resolved during spec

1. **The AC set is exactly the eight the first pass enumerated** — AC-001…AC-008, none
   added, none dropped. They partition the three traced project ACs as: AC-007 →
   AC-001/AC-002/AC-006, AC-008 → AC-003/AC-004/AC-007/AC-008, AC-009 → AC-005.
2. **Three personas, four journeys, seven atoms.** DR-10's original "four persona atoms"
   consequence is superseded by the repository owner's amendment of 2026-08-12
   (`_decomposition.md` *The evaluator-persona decision*). Both halves are binding: the
   evaluation path is a **first-class atom** *and* **not a persona**. Where `_storymap.md`
   *Backbone* row D and *Grain notes* still say four personas, they are stale — recorded
   here and routed as a finding (EC-011), not repaired in this PR.
3. **AC-009's per-atom formulation governs over DR-9's literal sentence.** DR-9 quotes a
   sentence beginning "All four personas…", which the amendment made false. Each atom's
   `summary` must say something **true of that atom**; the disclaimer is the same strength
   across all seven (the brief's argument 4: "only the same one, honestly worded"), while
   the wording stays honest about the differential evidence `personas-and-journeys.md`
   *Risks* records.
4. **Interaction quality: composition family N/A, state family translated.** The project's
   signed-off `_design.md` records no public API surface and `design.capture` is absent
   from `.redkiln/config.yaml`, so there is no approved composition, density budget or
   transience policy to bind — declared, not silently skipped. The state-family invariants
   do apply to a durable corpus and are each carried by a table row (AC-004, AC-006,
   AC-007, AC-008, AC-002, AC-003), never by a prose bullet, so `redkiln verify` gates
   them.
5. **`verifying_test` for a story that writes no code** is a command plus the real path
   where its verbatim output is recorded. The project's `testing` brief names static and
   process as first-class tiers here (`_decomposition.md` *Notes → Test mix*), and AC-008
   is explicitly *E2E (process)* in that brief because the mechanism itself is what is
   under test.
6. **The verification transcript is `_promotion-record.md` in this story's directory**, not
   the project's `_closeout-record.md`. That companion is where the DoD set, the delta, the
   audit tables and the findings converge (`_storymap.md:24-30`); this story is not one of
   those four, and its only outbound obligation is the findings row.
7. **`.kb/maps/domain-map.md:35` ("This is the map's first wave. One domain exists so
   far.") is stale** — the file already carries two `##` domain sections (`:37`, `:82`), so
   the audience is the third. AC-006 requires a new appended section and says nothing about
   the count sentence: the Maps phase may correct it inside the wave commit, a hand-edit
   outside the wave violates NF-006, and leaving it is a finding rather than a defect this
   story fixes.
8. **The ingest branch merge is inside this PR.** `/redkiln:kb-ingest` deliberately does
   not merge. Because AC-003 reads the commit graph of the *closeout tree*, the `--no-ff`
   merge is part of this story's deliverable rather than a follow-up, and a squash is
   EC-004.
