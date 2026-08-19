---
item: HS-S0086
stage: spec
created: 2026-08-12T13:47:24.047Z
updated: 2026-08-12T13:47:24.047Z
template_sig: 87bbf1d0
rendered_sig: 83721c90
---

# Spec — The MSRV stops being a preference and becomes a promise

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-08 (`:291`), AC-12 (`:341-343`), DoD 10 (`:387-389`) |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` — the ADR-number allocation `0017–0028` to sibling projects (`:116-121`). Cited below as `../_decomposition.md`, to keep it distinct from the project's brief file of the same name |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — AC-006 (`:246-250`), DR-6 (`:176-183`), DR-13 (`:208-212`) |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/msrv-promise-atom/spec.md` |
| Key briefs | `.bklg/.../publication-and-positioning/_decomposition.md` — `ux` U6 (`:87`) and AC-UX-010 (`:361-366`); `testing` AC-006 instrument row (`:469`); `deployment` MSRV consequence (`:743-747`), AC-DEP-006 (`:770-776`), and its explicit non-amendment note (`:777-780`) |
| Signed-off design | `.bklg/.../publication-and-positioning/_design.md` — `## Visibility and stability` (`:566-580`), transience row (`:429`), Guarantees budget (`:470`), cost-of-depending (`:591-595`) |
| Story map row | `.bklg/.../publication-and-positioning/_storymap.md`:58, merge order `:159-160`, foundation-consumer table `:106` |
| Grounding | `.bklg/.../publication-and-positioning/_grounding.md`:8-36 — the two accepted atoms, and the `ls .kb/decisions/` evidence that `0030` and up is free |
| Roadmap pointer | `RUNBOOK.md`:163 (phase 12's row), `RUNBOOK.md`:4448-4451 (phase 12's goal, and the four-crate set this project's sibling rejected) |

## One-line PR slice

Author, through `.kb/_intake/` and `/redkiln:kb-ingest`, a new decision atom at `0030`-or-above
stating the 1.97.1 floor as a promise, why it is where it is, and what an MSRV bump costs a
consumer under 0.x — leaving `.kb/decisions/0004-edition-and-msrv.md` and
`0029-msrv-raised-to-1-97-1.md` byte-identical and `redkiln validate --kb` clean.

## Executive summary

Two accepted atoms already say everything about the *number*. `.kb/decisions/0004-edition-and-msrv.md`:83-86
says the floor is *"a preference until first publish (phase 12) and a promise to downstream
consumers afterward"*; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:94-95 closes with the standing
charge *"Phase 12 must revisit the floor at first publish."* This project is phase 12, and this
story is the revisit.

**The delta is one new sibling atom, and nothing else.** The number does not move (`Cargo.toml`:8
and `rust-toolchain.toml`:2 both say 1.97.1 and stay saying it). What changes is its *status*: an
internal preference that could be traded against a dependency becomes a commitment to a consumer
who may never build `happenstance-sqlite`, the crate that forced it. The atom is the artefact of
record for that conversion — the thing `guarantees-and-docs-rs-presentation` links from the one
Guarantees bullet a `cargo add` evaluator actually reads, and the thing `redkiln validate --kb`
holds immutable from the moment it is accepted.

Three things make it real work rather than a file copy. It must be **new**, because both existing
atoms are `status: accepted` and their bodies are checked against `HEAD`. It must be **authored
through the ingest path**, because hand-writing atoms was tried and reverted (`0269720`). And it
must be **self-sufficient at one hop**, because the surface that links it is allowed exactly one
link per bullet (`_design.md`:470) — an atom that says "see ADR-0004 and ADR-0029" spends a hop
budget the design has already allocated.

## Context pack

The decisions this story must honour. Each is settled elsewhere and is not re-opened here.

**1 — The floor is 1.97.1 and this story does not move it.** `Cargo.toml`:8 carries
`rust-version = "1.97.1"`; `rust-toolchain.toml`:2 pins the same number. Changing either is a
different decision with a different ADR. This story converts a status, not a value. If the
implementer believes the floor is wrong, that is a *finding* that blocks and re-plans (project
`project.md`:217-219, DR-15's shape), not an edit made in passing.

**2 — The promise is made by a NEW sibling atom. Not an edit. Not a supersession.**
`.kb/decisions/README.md`:7-13 states the rule and `redkiln validate --kb` enforces it against
`HEAD`. `.kb/maps/decision-map.md`:76-79 records the exact relationship 0004 and 0029 already have —
an **amendment**, with `superseded_by` deliberately `null` on 0004 and the edge carried by 0029's
`depends_on`. This story adds a third node in that lineage under the same shape: the new atom
carries `depends_on: [kb-decision-0004, kb-decision-0029]`, and **neither existing atom's frontmatter
or body is touched** — not even to add a `superseded_by`. A promise is not a reversal; nothing in
either atom stops being true.

**3 — The atom is written by the ingest run, not by hand.** `CLAUDE.md` is explicit that
hand-authoring atoms "produces the directory layout of the process without the process, which is
why the first attempt at this was reverted (`0269720`)"; the project restates it as DR-13
(`project.md`:208-212). So the implementer's *authored artefact* is a staged document under
`.kb/_intake/`, and `/redkiln:kb-ingest` is what mints the atom, the frontmatter, the map row and
the wave record. The staged document is raw material held to no schema (`.kb/_intake/README.md`:7-11),
and a successful run **clears the directory** (`:13-19`) — which is why the long form is not lost:
the wave is committed together, so the staged source stays in git history and every atom it fed
cites its `.kb/_intake/…` path in `source_paths`.

**4 — The number is `0030` or above, and it must not collide with a slice-mate.**
`../_decomposition.md`:116-121 allocates `0017–0028` to sibling projects and `0029` is on disk;
`_grounding.md`:33-36 confirms `.kb/decisions/` holds exactly `0001–0016` and `0029`. This slice
authors **four** atoms in one ingest wave (`_storymap.md`:73-78) — `crate-set-decision`,
`projection-port-ship-shape`, this story, `first-contact-design-resolutions` — so "take the next
free number" is a race between four files staged into the same directory. Claim the number by the
staged filename's prefix and check it against the other three before the wave runs.

**5 — The filename is a permanent public URL, so choose the slug once.** The Guarantees bullet that
will link this atom uses an absolute GitHub blob URL, because a relative repo link does not resolve
on a rendered crates.io page — see the existing ADR-0029 bullet at `crates/happenstance/README.md`:46-49.
Once `publish-0-2-0` lands, that README is frozen for `0.2.0` forever: `cargo yank` removes a version
from resolution and leaves every rendered page exactly as it was (`_storymap.md`:91-96). A rename
after publish is a permanently dead link on a page nobody can edit.

**6 — The atom must be self-sufficient at one hop.** `_design.md`:429 places the MSRV promise as
*revealed on scroll, in Guarantees; the atom is 1 hop*, and `:470` budgets the Guarantees list at
**≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per bullet**. A reader arriving at this atom
has spent their hop. Everything they came for — the number, why it is that number, and what a bump
would cost them — is restated **in this atom's own body**, not delegated onward to ADR-0004 or
ADR-0029. Citing them for provenance is right; requiring them to answer the consumer's question is
not.

**7 — The verification claim has to be honest, and today's is not.**
`crates/happenstance/README.md`:46 currently reads *"MSRV 1.97.1, checked in CI."* The `msrv` job at
`.github/workflows/ci.yml`:241-260 runs `cargo hack check --workspace --no-dev-deps --rust-version`
on a toolchain **pinned to the same 1.97.1 the gate already uses** — and ADR-0029:63-69 says in its
own words that the job is kept "with a comment naming why it is currently vacuous… it proves nothing
until the pin and the floor next diverge." The atom states what is actually checked, what is not,
and the condition (pin ≠ floor) under which the job starts meaning something. This tree already has
a worked example of an MSRV sentence going stale on a published-shaped surface — `CHANGELOG.md`:1031-1033
records two READMEs promising "MSRV 1.85, checked in CI" for a phase after the floor moved.

**8 — What a bump costs a consumer is the question DR-6 actually asks.** `project.md`:176-183
requires the atom to state *"whether an MSRV bump remains a minor bump now that the policy at
`.kb/decisions/0004-edition-and-msrv.md`:83-86 binds a real consumer."* ADR-0004's Policy says a
bump is a minor version bump called out in the changelog. Under 0.x the minor bump **is** the
breaking-change boundary (`project.md`:141-142, `_design.md`:566-580), so "a bump is a minor bump"
and "a bump is breaking" are the same sentence here, and the atom must say so rather than leave the
reader to compose it. The deployment brief (`_decomposition.md`:743-747) is explicit that the atom
is the artefact of record and the brief only carries the release-path consequence.

**9 — The persona slice is U6, and this story is its evidence, not its sentence.**
`_decomposition.md`:87 states U6 as *"Tell me what depending on you costs me, and when that cost can
change"*, owned by Persona 4, the evaluator — one-shot and time-boxed. The sentence they read is
written by `guarantees-and-docs-rs-presentation` under AC-UX-010 (`_decomposition.md`:361-366), which
blocks on this story (`story.md` frontmatter `blocks: [HS-S0094]`). This story therefore owes that
story **quotable, budget-shaped text**: three parts (the floor as a promise, what a bump means, and
that under 0.x the minor bump is the signal), each expressible in ≤ 3 rendered lines.

**10 — A decision without its rejected options is indistinguishable from an accident.**
`.kb/decisions/README.md`:31-33. The forks that are live here: keeping the floor a preference or
promising only "latest stable"; lowering the published crates back to 1.85 by pinning `rusqlite`
0.37 (ADR-0029:78-81 rejected this at phase 2 for a *different* reason — it must be re-weighed
against a consumer, not inherited); a per-package `rust-version` so the three published crates
promise less than the workspace (ADR-0029:83-87, same caveat); and a moving-window policy
(e.g. "N-2 stable") in place of a fixed floor. Name the ones considered and why they lost.

**11 — Two ingest hazards this repository has already met.** `/redkiln:kb-ingest` with no argument
ingests all of `.kb/_intake/*.md` (`.kb/_intake/README.md`:3-5), and that glob includes the
directory's own README — drop it at the approval gate rather than ingesting it. And the wave record
is a dated directory under `.kb/_governance/integration-waves/`, which already holds
`2026-08-10-intake` and `2026-08-10-intake-2`: a new wave takes a **new** id so it cannot overwrite
either audit trail.

**12 — The gate this story is measured by is the KB gate, not the compiler.** `redkiln validate --kb`
(KbFrontmatter conformance plus accepted-decision immutability) and `redkiln doctor` — which must
report exactly the six expected `template-drift` advisories and no `dependency-cycle`
(`project.md`:297-298). The testing brief names the wrong implementation this catches
(`_decomposition.md`:469): *"an atom that edits `.kb/decisions/0004-edition-and-msrv.md` or
`0029-msrv-raised-to-1-97-1.md` in place."* No Rust changes, so `cargo xtask affected` has nothing
to say here; the story is still expected to leave `cargo xtask ci --fast` green because it must not
break anything.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this project by
  `guarantees-and-docs-rs-presentation` (`_storymap.md`:106). It is not an unproven island: the
  atom it lands is the link target of a bullet a later story in this same project writes.
- **Slice / milestone**: `release-decisions`. Slice-mates, implemented in one context and mounted
  as one integrated change: `crate-set-decision`, `projection-port-ship-shape`,
  `first-contact-design-resolutions`. All four produce ingested decision atoms and the slice runs
  **one** `/redkiln:kb-ingest` wave, not four (`_storymap.md`:73-78). Merge order inside the slice
  puts this story third and unordered with the first two (`_storymap.md`:159-160).
- **Mount point**: **`.kb/maps/decision-map.md`** — the corpus's decision index and the one place
  the whole supersession graph is visible (`:24-28`). An atom with no row there is an atom nobody
  navigating the KB can reach, which is the KB equivalent of a component constructed but never
  rendered. `:81-86` states how a row is added: ADR-number order, under a **new** `##` section for
  the wave that introduced it, never deleting a row. The atom file itself under `.kb/decisions/` is
  the second half of the mount — it is what `redkiln validate --kb` walks.
- **Wires into**:
  - `.kb/_intake/` — the authoring path (`README.md`:3-19); the staged document is this story's
    hand-written artefact and the atom's `source_paths` entry.
  - `.kb/decisions/0004-edition-and-msrv.md`, `.kb/decisions/0029-msrv-raised-to-1-97-1.md` —
    **read-only**. Consumed as `depends_on` edges and as citations; byte-identical afterwards.
  - `.kb/_governance/integration-waves/<new dated wave>/` — the wave's audit trail, alongside
    `2026-08-10-intake` and `2026-08-10-intake-2`.
  - `.kb/maps/domain-map.md` — if the ingest run's Maps phase groups the new atom by subject, as it
    did for the seventeen imported atoms (`:88`).
  - `CLAUDE.md`'s fifth binding constraint — the prose that currently promises this work in the
    future tense (*"it loses the marker at phase 12, when first publish turns the MSRV into a
    promise"*). ADR-0029 set the precedent of rewriting that constraint in the same change that
    made it wrong.
  - `crates/happenstance/README.md`:41-49 — **consumed, not edited here.** The Guarantees slot is
    `guarantees-and-docs-rs-presentation`'s to write; this story supplies the text and the link
    target and must fit its budget.
- **Renders surfaces**: **none.** The seven surfaces in `_design.md`:104-176 are rendered by
  `published-surface-copy` and read by `rendered-page-preflight`. This story renders no surface and
  claims no entry in `_design.md`'s `## Items` block (`:49-75`), whose two entries belong to
  `projection-port-ship-shape` and `guarantees-and-docs-rs-presentation`. It is bound by
  `_design.md`:429 and `:470` as the **link target** of one bullet on `crates-io-happenstance`, and
  by `## Visibility and stability` (`:574-580`), which names the stability promise a designed
  surface object rather than a policy.
- **Public items**: none. No Rust item, no feature, no manifest key.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** The MSRV is a promise about
  the compiler a consumer builds with, not about store behaviour; no fixture can observe it and no
  rule in `happenstance-testkit`'s `suite.rs` can fail because of it. `spec/SPECIFICATION.md`
  contains no MSRV or `rust-version` clause at all (verified by search), which is why this story
  discharges no clause and amends none.
- **Clause(s)**: none discharged, none amended, nothing `[FROZEN]` touched.
- **Advances DoD scenario**: initiative **AC-12** — *"A consumer knows which compiler they need, and
  why it moved"* (`initiative.md`:341-343) — is the criterion this story is the record for. Toward
  green on **DoD 10** (*"the published crate looks finished"*, `initiative.md`:388-390), through the
  Guarantees bullet its dependent story writes, and on the project's own DoD item 4 (`validate --kb`
  and `doctor` clean, `project.md`:297-298). It turns no DoD scenario green by itself; a foundation
  that claimed to would be claiming its consumer's work.

## PR boundary

**In this PR**

- A staged intake document under `.kb/_intake/`, number-prefixed to claim its ADR number, carrying
  the full MSRV-promise argument: the floor, why it is that number, what a bump costs a consumer
  under 0.x, how the promise is actually verified today, and the alternatives that lost.
- The ingest run's output: **one** new `decision` atom under `.kb/decisions/` at a free number
  `≥ 0030` with valid `KbFrontmatter`, its `.kb/maps/decision-map.md` row under a new wave section,
  any `domain-map.md` grouping the Maps phase makes, the dated wave record under
  `.kb/_governance/integration-waves/`, and the cleared `_intake` directory.
- The one-sentence correction to `CLAUDE.md`'s fifth binding constraint, so it stops describing this
  conversion as future work and points at the new atom.
- This story's own backlog folder — the ledger and, later, the report.
- The implementer may also touch the mount files named in the Integration contract
  (`.kb/maps/decision-map.md`, `domain-map.md`, the wave directory) to mount this atom into the
  corpus. That is the mount, not scope drift.

**Explicitly not in this PR**

- **Any edit to `.kb/decisions/0004-edition-and-msrv.md` or `0029-msrv-raised-to-1-97-1.md`**,
  including a frontmatter `superseded_by` flip. Both stay byte-identical (`project.md`:246-250).
- **The consumer-facing sentence.** The Guarantees bullets in `crates/happenstance/README.md`:41-49
  and the other packaged READMEs belong to `guarantees-and-docs-rs-presentation` (AC-UX-010).
- **Any change to the floor**, to `Cargo.toml`:8, to `rust-toolchain.toml`, or to the `msrv` job at
  `.github/workflows/ci.yml`:241-260. The atom may *say* what would make that job non-vacuous; it
  does not make it so.
- **A `CHANGELOG.md` release note.** The 0.2.0 entry is `publish-0-2-0`'s, on the tree it publishes.
- **A long-form record under `references/adr/`.** Nothing in `spec/SPECIFICATION.md` cites one for
  this decision, and the staged `_intake` document is the long form, preserved in git history by the
  wave commit (`.kb/_intake/README.md`:13-19). Adding one is a deliberate later choice, not a
  by-product of this story.
- **The other three atoms in this slice.** Their content is their own stories'; only the shared
  number space and the shared wave are coordinated here.
- **Resolving `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md`.** That is
  AC-013's decision pass, and `project.md`:359-364 forbids settling it in passing.

**Merge DoD**: the new atom is on disk, reachable from `.kb/maps/decision-map.md`, produced by an
ingest wave rather than by hand; `git diff` shows ADR-0004 and ADR-0029 unchanged; and
`redkiln validate --kb` and `redkiln doctor` are clean with exactly the six expected
`template-drift` advisories.

```
.kb/_intake/**
.kb/decisions/**
.kb/maps/decision-map.md
.kb/maps/domain-map.md
.kb/_governance/integration-waves/**
CLAUDE.md
.bklg/from-contract-to-published-library/publication-and-positioning/msrv-promise-atom/**
```

A glob can express "may touch `.kb/decisions/`" but not "may only *add* to it". The immutability of
the two existing atoms is carried by an acceptance criterion and by `validate --kb`, not by this
boundary.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| A staged intake document exists and is the authored artefact | Number-prefixed under `.kb/_intake/`, held to no schema, consumed and cleared by the wave; cited in the resulting atom's `source_paths` | `.kb/_intake/README.md`:3-19; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:28-30 shows the resulting `source_paths` shape |
| Exactly one new `decision` atom is minted, by the ingest run | `kind: decision`, `authority_tier: decision`, `status: accepted`, `adr_id: ADR-00NN`, `phase: 12`, `reversibility`, `supersedes: null`, `superseded_by: null`, `depends_on: [kb-decision-0004, kb-decision-0029]`, `last_reviewed` the wave's date | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-35 (the field-by-field precedent); `project.md`:208-212 (DR-13, the ingest path) |
| The number is `≥ 0030`, free, and unique within the wave | `0017–0028` are allocated to sibling projects, `0029` is on disk; three slice-mates stage atoms into the same directory in the same wave | `../_decomposition.md`:116-121; `_grounding.md`:33-36; `_storymap.md`:73-78 |
| The atom states the floor as a **promise**, with the number unchanged | 1.97.1, matching `rust-version` and the toolchain pin, and says plainly that a consumer on an older toolchain cannot build `0.2.0` — the conversion ADR-0004's Policy schedules for phase 12 | `Cargo.toml`:8; `rust-toolchain.toml`:2; `.kb/decisions/0004-edition-and-msrv.md`:83-86; `_design.md`:591-595 |
| The atom states **why the floor is where it is**, self-sufficiently | A dependency's build script (`libsqlite3-sys` 0.38.1's `cfg_select!`), and five of five database crates declaring no `rust-version` so neither `cargo hack --rust-version` nor `resolver = "3"` could see it coming; `rust-version` and `rust-toolchain.toml` stay two different facts | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:41-53, :63-69; `_design.md`:470 (the one-hop budget that forces restatement rather than delegation) |
| The atom states **what a bump costs a consumer under 0.x** | An MSRV bump is a minor version bump called out in the changelog, and under 0.x the minor bump *is* the breaking-change signal — one sentence, not two facts left to be composed; answers DR-6's explicit question of whether that policy survives a real consumer | `project.md`:176-183; `.kb/decisions/0004-edition-and-msrv.md`:83-86; `project.md`:141-142; `_design.md`:566-580 |
| The atom describes verification **honestly** | Names the `msrv` CI job, says it is currently vacuous because the pinned toolchain equals the floor, and names the condition (pin ≠ floor) under which it stops being; does not restate the unbacked *"checked in CI"* phrasing | `.github/workflows/ci.yml`:241-260; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:63-69; `crates/happenstance/README.md`:46; `CHANGELOG.md`:1031-1033 |
| The atom names the alternatives that lost | At minimum: keep it a preference / promise "latest stable"; lower the published crates to 1.85 by pinning `rusqlite` 0.37; a per-package `rust-version`; a moving-window policy. The two ADR-0029 already rejected are re-weighed against a *consumer*, not inherited | `.kb/decisions/README.md`:31-33; `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:76-92 |
| The atom is quotable at the Guarantees budget | Supplies three parts — floor-as-promise, what a bump means, minor-bump-is-breaking — each expressible in ≤ 3 rendered lines with one link, for AC-UX-010's bullets | `_design.md`:429, :470, :574-580; `_decomposition.md`:361-366 |
| ADR-0004 and ADR-0029 are byte-identical afterwards | No body edit, no frontmatter edit, no `superseded_by` flip: this is a sibling, and the amendment lineage stays as the map records it | `.kb/decisions/README.md`:7-13; `.kb/maps/decision-map.md`:30-34, :76-79; `project.md`:246-250 |
| The atom is mounted on the decision map | A row in ADR-number order under a **new** `##` wave section, with the amendment/dependency relationship in the last column; no existing row deleted | `.kb/maps/decision-map.md`:81-86, :47-65 |
| The wave leaves a non-destructive audit trail and a clean `_intake` | New dated wave directory beside `2026-08-10-intake` and `2026-08-10-intake-2`; `_intake` cleared of the staged document with its README retained and not ingested | `.kb/_governance/integration-waves/`; `.kb/_intake/README.md`:3-5, :13-19, :29-30 |
| `CLAUDE.md`'s fifth constraint stops describing this as future work | The sentence promising the `provisional` marker comes off "at phase 12" now names the new atom and reads in the past tense; no other binding constraint changes | `CLAUDE.md` (Binding constraints, item 5); `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:71-74 (the precedent for rewriting it in the same change) |
| The KB gate is the acceptance instrument | `redkiln validate --kb` (KbFrontmatter + accepted-decision immutability against `HEAD`) and `redkiln doctor` clean, exactly six `template-drift`, no `dependency-cycle` | `_decomposition.md`:469; `project.md`:297-298; `CLAUDE.md` (Commands) |
| No API, no clause, no rule | No Rust item changes, no `spec/SPECIFICATION.md` clause is discharged or amended, no conformance rule observes this | `_design.md`:49-75 (`## Items` — neither entry is this story's); `spec/SPECIFICATION.md` (no MSRV or `rust-version` clause) |

## Data and migrations

**N/A for runtime data.** This story adds no schema, no table, no serialised format and no code. The
release-grain posture is already recorded by the deployment brief as N/A with its reason — first
publish, no prior compatible version (`_decomposition.md`, AC-DEP-004) — and this story does not
change it.

The only data with a shape to get wrong is **corpus data**, and it is checked by
`redkiln validate --kb`:

| Field | Value for this atom | Why |
| --- | --- | --- |
| `id` | `kb-decision-00NN`, matching the filename's number | The convention every atom in `.kb/decisions/` follows |
| `kind` / `authority_tier` | `decision` / `decision` | A commitment is a decision atom, never a playbook or concept — those carry no immutability (`.kb/decisions/README.md`:26-29) |
| `status` | `accepted` | Which makes it immutable from the wave commit onward |
| `adr_id` | `ADR-00NN`, `≥ 0030`, not in `0017–0028`, not `0029` | `../_decomposition.md`:116-121; `_grounding.md`:33-36 |
| `phase` | `12` | The phase that owned the decision (`RUNBOOK.md`:163) |
| `supersedes` / `superseded_by` | `null` / `null` | Nothing is reversed; this is the amendment-lineage shape, not a supersession (`.kb/maps/decision-map.md`:76-79) |
| `depends_on` | `[kb-decision-0004, kb-decision-0029]` | The edges carry the lineage, exactly as 0029's `depends_on` carries its edge to 0004 |
| `source_paths` | the staged `.kb/_intake/…` file, `Cargo.toml`, `rust-toolchain.toml`, `.github/workflows/ci.yml` | The wave commit is what preserves the first of these |
| `last_reviewed` | the wave's date | — |

**Migration of the corpus itself**: additive only. One new file, one new map row under a new wave
section, one new wave directory. Nothing is renamed, nothing is deleted, and the two existing MSRV
atoms are the explicit non-migration this story is measured on.

## Acceptance criteria

The persona is **Persona 4, the evaluator** (`.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md`:249-288):
a Rust developer who has *not* adopted anything, is time-boxed and one-shot, and whose only evidence
is what the project chose to publish. Their intent here is U6 — *"tell me what depending on you costs
me, and when that cost can change"* (`_decomposition.md`:87). They arrive at this atom having spent
the single link the Guarantees bullet is allowed. Every criterion below is written from that arrival,
or from the maintainer who must still be able to find and trust the record a year later.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **The promise exists as an atom the corpus minted, not one a hand wrote.** GIVEN `.kb/decisions/` may only hold atoms produced by an ingest wave (DR-13, `project.md`:208-212; `CLAUDE.md`, and the reverted hand-authoring attempt `0269720`), WHEN this story lands, THEN exactly **one** new `decision` atom exists under `.kb/decisions/` with valid `KbFrontmatter` — `kind: decision`, `authority_tier: decision`, `status: accepted`, `phase: 12`, a stated `reversibility`, `supersedes: null`, `superseded_by: null`, `depends_on: [kb-decision-0004, kb-decision-0029]` — its `source_paths` cites the staged `.kb/_intake/…` document it was authored from, and that staged document and the atom appear in the **same** wave commit; and the atom is real composed prose in the corpus's shape, not a schema-valid stub | `redkiln validate --kb` (KbFrontmatter conformance); `git log --stat` on the wave commit shows the staged `_intake` source and the new atom together; `.kb/_governance/integration-waves/` gains exactly one new dated directory beside `2026-08-10-intake` and `2026-08-10-intake-2`; field-by-field read against `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-35 |
| AC-002 | **The number and the filename are chosen once, because they become a permanent public URL.** GIVEN `0017–0028` are allocated to sibling projects (`../_decomposition.md`:116-121), `0029` is on disk (`_grounding.md`:33-36), and three slice-mates stage atoms into the *same* wave (`_storymap.md`:73-78), WHEN the wave runs, THEN this atom's `adr_id` is `≥ ADR-0030`, is none of `0017–0028` or `0029`, and is distinct from every other atom in the wave; and its filename slug is the exact target of the absolute GitHub blob URL `guarantees-and-docs-rs-presentation` will publish, in the form the existing ADR-0029 bullet already uses at `crates/happenstance/README.md`:46-49, chosen before publish because `cargo yank` leaves every rendered page exactly as it was | `ls .kb/decisions/` before and after; the four staged filenames' number prefixes compared in one place before the run; content review against `_storymap.md`:73-78; downstream, the AP-6 dead/relative-link check at `rendered-page-preflight` resolves the published URL onto this filename |
| AC-003 | **The evaluator learns the floor as a promise, inside their budget, before deciding to depend.** GIVEN Persona 4 has followed the one link the Guarantees bullet is allowed (`_design.md`:429, `:470`), WHEN they land on the atom, THEN its opening states that the `0.2.0` release requires Rust **1.97.1**, that this is now a **promise to a consumer** rather than an internal build setting, and plainly that a consumer on an older toolchain cannot build the release — expressed so it is quotable as AC-UX-010's first Guarantees bullet at **≤ 3 rendered lines with exactly 1 link**, positioned as a cost read *after* interest exists rather than as identity copy; and `Cargo.toml`:8 and `rust-toolchain.toml`:2 both still read `1.97.1` | content review against `_design.md`:591-595, `:429`, `:470`; `git diff` shows `Cargo.toml` and `rust-toolchain.toml` unchanged; downstream, the bullet `guarantees-and-docs-rs-presentation` writes is drawn from this text rather than composed independently |
| AC-004 | **The evaluator's question is answered here and not one hop further on.** GIVEN their hop budget is already spent, WHEN they read the atom with ADR-0004 and ADR-0029 closed, THEN the atom's own body still answers *"which compiler, and why that one"* — naming that the floor was forced by a **dependency's build script** rather than by this workspace's code, that five of five database crates in the workspace declare no `rust-version` at all so neither `cargo hack --rust-version` nor `resolver = "3"` could see it coming, and that `rust-version` and `rust-toolchain.toml` remain two different facts; ADR-0004 and ADR-0029 are cited for provenance and are **not** required reading | content review: the atom read standalone with both predecessors closed; cross-checked against `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:41-53 for factual agreement (not for delegation) |
| AC-005 | **The evaluator learns what a bump would cost them, as one statement and not two facts to compose.** GIVEN U6 asks *when the cost can change* and DR-6 asks explicitly whether the policy survives now a real consumer is bound (`project.md`:176-183), WHEN they read the atom's policy section, THEN it states that an MSRV bump is a **minor version bump called out in the changelog** *and* that under 0.x the **minor bump is itself the breaking-change signal**, joined in one statement rather than left to the reader — and it says whether that policy stands unchanged now that it binds someone; the statement is quotable at **≤ 3 rendered lines with exactly 1 link** for AC-UX-010's remaining bullets | content review against `project.md`:176-183, `.kb/decisions/0004-edition-and-msrv.md`:83-86, `project.md`:141-142 and `_design.md`:566-580; downstream, AC-UX-010's bullets in `guarantees-and-docs-rs-presentation` |
| AC-006 | **What is actually checked is stated, so the promise is never larger than its evidence.** GIVEN `crates/happenstance/README.md`:46 today says *"MSRV 1.97.1, checked in CI"* while the `msrv` job at `.github/workflows/ci.yml`:241-260 runs on a toolchain pinned to the floor itself, and this tree has already shipped a stale MSRV sentence (`CHANGELOG.md`:1031-1033), WHEN the atom describes verification, THEN it names the job, says in words that the job is **currently vacuous** because the pin equals the floor, names the condition (**pin ≠ floor**) under which it starts proving something, and does not restate the unbacked *"checked in CI"* phrasing; the status is carried by words, never by a glyph or a badge alone | content review against `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:63-69 and `.github/workflows/ci.yml`:241-260; re-read for truth-at-the-publish-commit (IQ-7) by `rendered-page-preflight` |
| AC-007 | **A reader a year from now can tell this was a fork, not an accident.** GIVEN `.kb/decisions/README.md`:31-33 requires the rejected options, WHEN the atom is read, THEN it names at minimum four live alternatives and why each lost — keep the floor a preference / promise only *"latest stable"*; lower the published crates to 1.85 by pinning `rusqlite` 0.37; a per-package `rust-version` so the three published crates promise less than the workspace; and a moving-window policy such as *N-2 stable* in place of a fixed floor — with the two ADR-0029 already rejected (`:76-92`) **re-weighed against a consumer** rather than inherited | content review; the presence bar the testing brief applies to this project's other recorded decisions (`_decomposition.md`, the AC-011 instrument row) — a resolution that states a winner without stating what lost fails it |
| AC-008 | **The two atoms that already carry the number come through byte-identical.** GIVEN both are `status: accepted` and `redkiln validate --kb` checks accepted bodies against `HEAD` (`.kb/decisions/README.md`:7-13; `.kb/maps/decision-map.md`:30-34), WHEN the wave lands, THEN a diff against the branch point shows **zero changed bytes** in `.kb/decisions/0004-edition-and-msrv.md` and `.kb/decisions/0029-msrv-raised-to-1-97-1.md` — body *and* frontmatter, including any `superseded_by` flip, because this is an amendment lineage and not a supersession (`.kb/maps/decision-map.md`:76-79) — and every existing inbound `file:line` citation into either atom still lands where it did | `git diff --exit-code <base> -- .kb/decisions/0004-edition-and-msrv.md .kb/decisions/0029-msrv-raised-to-1-97-1.md`; `redkiln validate --kb`; `redkiln doctor` clean with exactly the six expected `template-drift` advisories and no `dependency-cycle` (`project.md`:297-298) |
| AC-009 | **The record is reachable by someone navigating the corpus, and nothing still calls this future work.** GIVEN an atom with no index row is the KB equivalent of a component constructed but never rendered, WHEN the wave lands, THEN the atom has a row on `.kb/maps/decision-map.md` in ADR-number order under a **new** `##` wave section with its 0004/0029 dependency named in the relationship column and **no existing row deleted or displaced** (`:81-86`); `.kb/_intake/` is cleared to its README alone, with that README not ingested; and `CLAUDE.md`'s fifth binding constraint names the new atom in the past tense instead of promising the conversion *"at phase 12"* | read `.kb/maps/decision-map.md` for the new row and for the unchanged prior rows; `ls .kb/_intake/` shows `README.md` only; `git diff` on `CLAUDE.md`; `redkiln doctor` |

**Coverage of the traced project ACs.** Project **AC-006** (*the MSRV is a promise with a recorded
justification*, `project.md`:246-250) is carried by AC-001 and AC-003 through AC-008 — the atom
exists, states the floor, its reason, the consumer cost, the honest verification and the losers, with
the two predecessors untouched and `validate --kb` passing. Project **AC-013** (*every answer this
project settled is on disk as a decision atom*, `project.md`:273-276) is carried by AC-001 (authored
through the ingest path), AC-002 (a number free of the `0017–0028` allocation and of `0029`), AC-007
(the alternatives that lost) and AC-009 (mounted where the corpus can find it). Initiative **AC-12**
(`initiative.md`:341-343) is the roll-up of both.

## Interaction quality

**This story renders no surface of its own.** The seven surfaces in `_design.md`:104-176 belong to
`published-surface-copy` and `rendered-page-preflight`, and neither entry in `_design.md`'s `## Items`
block (`:49-75`) is this story's. What this story renders is a **read surface at exactly one hop** —
the atom a `cargo add` evaluator opens from one Guarantees bullet — so `_design.md` binds it as the
*link target* whose composition the bullet's budget already spent. Every invariant below is therefore
carried by an AC row in the table above; none of them lives only here.

**STATE invariants**

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In-place, not a context jump** — the reader's question is answered where they landed; no second hop is required to learn which compiler and why | **AC-004** | the atom read standalone with ADR-0004 and ADR-0029 closed |
| **Non-occlusion** — mounting this atom hides nothing already on the index: a new `##` wave section, a row in number order, no existing row deleted or pushed out of its lineage grouping | **AC-009** | `.kb/maps/decision-map.md` read for the new row *and* the intact prior rows (`:81-86`) |
| **Preserved selection / anchors** — every existing inbound `file:line` citation into ADR-0004 and ADR-0029 still resolves to the same line after this change | **AC-008** | `git diff --exit-code` on both atoms (zero bytes changed means every line number survives) |
| **Reversibility** — the atom carries a stated `reversibility`, and reversing it is a **new superseding atom**, never an edit; the filename, by contrast, is reversible only *before* publish, because `cargo yank` leaves rendered pages exactly as they were | **AC-001** (the field and the supersession shape), **AC-002** (the slug chosen once) | `redkiln validate --kb`; the slug compared against the URL form at `crates/happenstance/README.md`:46-49 |
| **Reachable without prior knowledge** — the keyboard-reachability analogue: a reader who does not know the filename still reaches the atom by navigating the decision index | **AC-009** | the map row is the index entry; `redkiln doctor` |

**COMPOSITION invariants** — from the signed-off `_design.md`, which is binding on this story as the
target of one Guarantees bullet.

| Invariant | Design source | Carried by | How it is verified |
| --- | --- | --- | --- |
| **Presentation exists at all** — a real atom in the corpus's composed shape with the frontmatter the schema requires, not a file that merely validates | `.kb/decisions/README.md`:26-33 | **AC-001** | `redkiln validate --kb` plus a read against ADR-0029's shape (`:1-35`) |
| **Placement** — the MSRV promise lives in **Guarantees**, not in the identity line, the status callout or the disambiguation triad | `_design.md`:429 | **AC-003** | content review against the transience table |
| **Transience** — *revealed on scroll* in Guarantees, with the atom itself **opened on demand at exactly 1 hop**; nothing the reader needs sits behind a second hop | `_design.md`:429, `:470` | **AC-003** (the reveal), **AC-004** (no second hop) | content review; the standalone read |
| **Density budget, with its real numbers** — the Guarantees list is **≤ 7 bullets, each ≤ 3 rendered lines, exactly 1 link per bullet**, and this project spends **4** of the 7; this story supplies three of those four as quotable text rather than as prose someone must compress | `_design.md`:470, `:591-595` | **AC-003** (floor-as-promise), **AC-005** (bump-means / minor-is-breaking) | the three parts are counted and measured when `guarantees-and-docs-rs-presentation` writes the bullets; a part that will not fit is a defect in this story, not that one |
| **Hierarchy** — the stability promise is a designed surface object subordinate to identity and status: it is a cost question asked *after* interest exists, and must not be written as first-screen lead copy | `_design.md`:574-580, `:429` | **AC-003** | content review against `## Visibility and stability` |
| **AP-6 — no dead or relative link** — the published bullet uses an absolute blob URL, so the atom's filename must be its permanent target and must not be renamed after publish | `_design.md`:656-657 (AP-6) | **AC-002** | `rendered-page-preflight`'s mechanical link check resolves onto this filename |
| **AP-7 — no raw HTML, inline style, script or meaning-bearing image** in the atom body, which renders on GitHub one hop from a published page | `_design.md`:658-659 (AP-7) | **AC-001** | content review of the staged document and the minted atom |
| **AP-2 — no signal carried by a glyph or colour alone** — the verification status is stated in words, never as a ✅ or a badge | `_design.md`:648-649 (AP-2) | **AC-006** | content review |

**The unstyled-render trap, in this medium.** An atom can satisfy `KbFrontmatter` perfectly, carry
every `depends_on` edge, pass `validate --kb`, sit on the decision map — and still be useless to the
evaluator, because it delegates the answer to two atoms they will not open, or because its argument
cannot be compressed into three rendered lines. AC-004 and the density rows above are what make that
fail; the schema checks alone never could.

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | **Number collision inside the wave** — two of the four slice-mates claim the same `≥ 0030` prefix | Numbers are reconciled in one place *before* any staged file is written; a collision discovered after the run is a re-plan of the wave, not a rename of an accepted atom (`_storymap.md`:73-78) |
| EC-002 | **The ingest adjudicator proposes MERGE or AMEND into ADR-0004 or ADR-0029.** The run is biased hard towards merging into an existing atom (`.kb/_intake/README.md`:7-11), and here that bias would edit an accepted body | Refuse at the approval gate and re-stage the document so the wave mints a **new** sibling atom. This is the single most likely way this story fails silently; it is what AC-008 exists to catch (`_decomposition.md`:469 names it as the wrong implementation) |
| EC-003 | **The no-argument glob ingests `.kb/_intake/README.md`** — it matches `.kb/_intake/*.md` (`README.md`:3-5) | Drop it at the approval gate; the README is a directory description, never an atom |
| EC-004 | **Wave id collides with `2026-08-10-intake` or `2026-08-10-intake-2`** | Take a new, suffixed id; overwriting either directory destroys an existing audit trail |
| EC-005 | **`redkiln validate --kb` fails on the new atom's frontmatter** | Fix the atom against `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-35's precedent. Never relax the check, and never satisfy it by editing a predecessor |
| EC-006 | **The implementer concludes 1.97.1 is the wrong floor** | This blocks and re-plans (the DR-15 shape, `project.md`:217-219). It is not an edit to `Cargo.toml`:8 or `rust-toolchain.toml` made in passing, and it is not absorbed into this atom |
| EC-007 | **A staged file is still in `.kb/_intake/` after a "successful" run** — the contract is that a successful ingest clears the directory (`README.md`:13-19) | Treat as a failed run: a file still sitting there is a file that run did not ingest. Do not hand-move it into `.kb/decisions/` |
| EC-008 | **`redkiln doctor` reports a seventh `template-drift` advisory, or any `dependency-cycle`** | Not this story's to absorb. A seventh drift means a template changed without a decision (`CLAUDE.md`); report and stop |
| EC-009 | **The three quotable parts will not fit `_design.md`:470's budget** | Rewrite the atom's statements until they do. The budget is `guarantees-and-docs-rs-presentation`'s constraint but this story's obligation — that story may not fix it by adding an eighth bullet or a second link |

## Non-functional

| id | requirement | source |
| --- | --- | --- |
| NF-001 | **Quotability.** Three parts — floor-as-promise, what a bump means, minor-bump-is-breaking-under-0.x — each expressible in **≤ 3 rendered lines with exactly 1 link**, within a Guarantees list capped at 7 bullets of which this project spends 4 | `_design.md`:470, `:574-580`; `_decomposition.md`:361-366 |
| NF-002 | **Sized like its siblings.** Roughly 100 lines, in the band the corpus already occupies (`.kb/decisions/` atoms run ~4.5–8.4 KB; ADR-0029 is 5.5 KB). The long form is the staged `_intake` document, preserved by the wave commit — not a second file under `references/adr/` | `CLAUDE.md` (*"~100 lines each"*); `.kb/_intake/README.md`:13-19 |
| NF-003 | **Self-sufficient at one hop.** No onward reading is *required* to answer the evaluator's question; citations are provenance | `_design.md`:429, `:470` |
| NF-004 | **URL durability.** The filename is a permanent public address from the moment `publish-0-2-0` lands; `cargo yank` does not re-render a page | `crates/happenstance/README.md`:46-49; `_storymap.md`:91-96 |
| NF-005 | **Zero build cost.** No Rust item, no feature, no manifest key, no compile-time or runtime effect; the story must still leave `cargo xtask affected` and `cargo xtask ci --fast` green | `.redkiln/config.yaml` (`affected_gate`, `integration_scoped`); `project.md`:299-303 |
| NF-006 | **Reads correctly in plain markdown.** Meaningful link text, no meaning carried by colour or glyph, no raw HTML or images — the atom is read on GitHub, one hop from a published page | `_design.md` AP-2, AP-7; `_decomposition.md` accessibility floor |
| NF-007 | **Additive to the corpus only.** One new file, one new map row under a new wave section, one new wave directory; nothing renamed, nothing deleted | `.kb/maps/decision-map.md`:81-86 |

## Implementation notes (non-prescriptive)

- **Reconcile the four ADR numbers before writing a word.** All four slice-mates stage into one
  directory and one wave; the number is claimed by the staged filename's prefix, so the cheapest
  place to resolve EC-001 is a single list agreed at the start of the slice.
- **Draft the three quotable parts first, then the argument around them.** NF-001 is the hardest
  constraint in the story; writing it last means rewriting the atom to fit it.
- **Read `.kb/decisions/0029-msrv-raised-to-1-97-1.md` field-by-field for the frontmatter, not the
  schema.** It is the nearest precedent, it is the atom this one depends on, and it already models
  the `depends_on`-carries-the-amendment-edge shape (`.kb/maps/decision-map.md`:76-79).
- **Write the staged `_intake` document as the long form.** It is the only place the compiler
  transcript, the full alternatives table and the reasoning have room; the wave commit is what keeps
  it, and the atom's `source_paths` is what points back at it.
- **Leave the `CLAUDE.md` sentence until the number is real**, so it names the actual atom rather
  than a placeholder — and make that edit in one place for the whole slice if the slice-mates also
  touch the file, because it is the shared conflict surface.
- **One wave, four atoms.** Run `/redkiln:kb-ingest` once for the slice, drop the intake README at
  the approval gate, and check the wave id against the two directories already under
  `.kb/_governance/integration-waves/`.
- **The atom may say what would make the `msrv` CI job non-vacuous; it must not make it so.** Changing
  the job or the pin is out of this PR's boundary.

## Tests and CI (merge gate)

The testing brief types project AC-006 as **static (process, not code)** with `redkiln validate --kb`
as its instrument and *"an atom that edits `.kb/decisions/0004-edition-and-msrv.md` or
`0029-msrv-raised-to-1-97-1.md` in place"* as the wrong implementation it must reject
(`_decomposition.md`:469); AC-013 is typed the same way, with *"an atom hand-written directly under
`.kb/decisions/` rather than staged through `.kb/_intake/`"* as its wrong implementation. There is no
compiled test here, and inventing one would be decorative — the gate is the KB gate.

| tier | command / path | proves |
| --- | --- | --- |
| static (process) | `redkiln validate --kb` | KbFrontmatter conformance on the new atom **and** accepted-decision immutability against `HEAD`. Rejects the wrong implementation the testing brief names: an in-place edit to ADR-0004 or ADR-0029. Covers AC-001, AC-008 |
| static (process) | `redkiln doctor` | Exactly the six expected `template-drift` advisories and no `dependency-cycle` (`project.md`:297-298). Rejects a seventh drift smuggled in with the wave, and a `depends_on` edge that closes a cycle. Covers AC-008, AC-009 |
| static (diff) | `git diff --exit-code <branch point> -- .kb/decisions/0004-edition-and-msrv.md .kb/decisions/0029-msrv-raised-to-1-97-1.md` | Byte identity of both predecessors, frontmatter included — the check `validate --kb` would still pass if a `superseded_by` were flipped *and* the map updated to match. Covers AC-008 |
| static (tree) | `ls .kb/decisions/`, `ls .kb/_intake/`, `ls .kb/_governance/integration-waves/` | The number is free and `≥ 0030`; `_intake` is cleared to its README alone (the contract at `.kb/_intake/README.md`:13-19); exactly one new dated wave directory, overwriting neither existing one. Covers AC-002, AC-009 |
| static (read) | `.kb/maps/decision-map.md` | The atom is mounted: a row in ADR-number order under a **new** `##` wave section, the 0004/0029 relationship named, no prior row deleted (`:81-86`). Rejects an atom on disk that nobody navigating the corpus can reach. Covers AC-009 |
| static (read) | `git diff -- CLAUDE.md` | The fifth binding constraint no longer describes this conversion in the future tense and names the new atom. Covers AC-009 |
| content review | the minted atom, read standalone with ADR-0004 and ADR-0029 closed | The four content criteria a schema cannot see: the floor stated as a promise (AC-003), self-sufficiency at one hop (AC-004), the consumer cost joined into one statement (AC-005), the honest verification claim (AC-006), and the alternatives that lost (AC-007). This is the same tier the testing brief assigns the project's other recorded decisions — *"a content review, not a compiled test"* |
| story grain (auto) | `cargo xtask affected --base {{base}}` (`.redkiln/config.yaml`, `affected_gate`) | This story maps to no workspace package, and the command is wired precisely so such a story is still gated: it runs the five file-reading lints and `spec-trace` unconditionally. Proves the KB-only change breaks nothing and leaves no dangling specification citation. Covers NF-005 |
| integration grain (auto) | `cargo xtask ci --fast` (`.redkiln/config.yaml`, `integration_scoped`) | The project's non-terminal bar is green with the wave applied — including all four mandatory `wasm32` steps. Covers NF-005 |
| downstream (not this PR) | AC-UX-010's Guarantees bullets in `guarantees-and-docs-rs-presentation`; the AP-6 link check in `rendered-page-preflight` | The three parts actually fit `_design.md`:470's budget and the published absolute URL resolves onto this atom's filename. A failure there is a defect *here* (EC-009). Covers NF-001, NF-004 |

## Risks and coupling (PR-scoped)

- **The ingest run's merge bias is the highest risk in the story.** `/redkiln:kb-ingest` is
  deliberately biased towards folding a claim into an existing atom rather than spawning a
  near-duplicate (`.kb/_intake/README.md`:7-11) — and the nearest existing atoms are the two this
  story may not touch. The mitigation is a human refusal at the approval gate (EC-002) plus AC-008's
  diff; neither is automatic.
- **Number and wave contention with three slice-mates.** All four stage into one directory and one
  wave, and all four may touch `.kb/maps/decision-map.md`, `.kb/maps/domain-map.md` and `CLAUDE.md`.
  The slice is implemented in one context for exactly this reason; splitting it re-introduces the
  conflict.
- **A permanent link with a one-way door.** The filename becomes a public URL at `publish-0-2-0`, and
  `cargo yank` does not re-render a page. Renaming later is a dead link on a page nobody can edit.
- **A downstream story is blocked on text that does not exist yet.**
  `guarantees-and-docs-rs-presentation` (HS-S0094) blocks on this atom. If the three quotable parts
  are not written here, that story will invent them, and the atom stops being the record for what the
  README says.
- **Scope pressure towards `references/adr/`.** The corpus convention is atom-plus-long-form, and it
  will feel natural to write both. Nothing in `spec/SPECIFICATION.md` cites a long form for this
  decision, and the staged `_intake` document already is one; adding a `references/adr/` record is a
  deliberate later choice, not a by-product.
- **Silent settlement of an open question.**
  `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` is adjacent to this work and
  `project.md`:359-364 forbids resolving it in passing; it belongs to AC-013's decision pass.
- **A truthful atom next to an untruthful README.** `crates/happenstance/README.md`:46 currently says
  *"checked in CI"*. This story deliberately does not fix that string — AC-006 only obliges the atom
  not to repeat it — so between this PR and `guarantees-and-docs-rs-presentation` the tree holds both
  claims. That is a known, bounded window, closed by AC-UX-010 and re-read at `rendered-page-preflight`
  under IQ-7.

## Dependencies

**Blocks on:** none. `depends_on: []` — this story has no prerequisite inside or outside the slice
(`story.md` frontmatter `blocked_by: []`; `_storymap.md`:159-160 places it third in the slice and
explicitly **unordered** with `crate-set-decision` and `projection-port-ship-shape`).

**Unlocks:** `guarantees-and-docs-rs-presentation` (HS-S0094) — the only consumer, and the reason
this foundation is not an unproven island (`_storymap.md`:106). It writes the Guarantees bullets under
AC-UX-010 and links this atom.

**Coordinated with, but not ordered against:** `crate-set-decision`, `projection-port-ship-shape`,
`first-contact-design-resolutions` — the three slice-mates in `release-decisions`. They share the ADR
number space and the single `/redkiln:kb-ingest` wave; there is no ordering edge between them and this
story, only the shared wave.

## Anchors (progressive disclosure)

Everything above is sufficient to start. These are the deeper artefacts, each with why it is
load-bearing and the moment to open it. Link them; do not paste them.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.kb/_intake/README.md` | The ingest contract in the repository's own words: the no-argument glob, "nothing staged here is an atom yet", and the clearing rule that explains why the staged long form is not lost | Before staging the document — it is the authoring path AC-001 is measured on, and it is where EC-003 and EC-007 come from | AC-001 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | Carries the four-atom, one-wave shape of the `release-decisions` slice, its merge order, and the foundation→consumer table that names HS-S0094 | At the start of the slice, when reconciling the four ADR numbers in one place | AC-002 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md` | The verified evidence that `.kb/decisions/` holds exactly `0001–0016` and `0029`, so `0030` and above is free — the fact AC-002 rests on | With the story map, before claiming a number | AC-002 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | The signed-off design: the transience row that places the MSRV promise in Guarantees at 1 hop (`:429`), the density budget with its real numbers (`:470`), `## Visibility and stability` (`:566-580`) and the cost-of-depending list (`:591-595`) | Before drafting the three quotable parts — the budget is what shapes them | AC-003 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The nearest precedent, three ways: the field-by-field frontmatter shape (`:1-35`), the evidence for *why 1.97.1* that AC-004 must restate rather than delegate (`:41-53`), and its own admission that the `msrv` job is vacuous (`:63-69`) plus the two options it rejected (`:76-92`) | Before writing the staged document; keep it open through the frontmatter | AC-004 |
| `.kb/decisions/0004-edition-and-msrv.md` | The Policy paragraph (`:83-86`) that makes the floor *"a preference until first publish and a promise afterward"* and states that a bump is a minor bump called out in the changelog — the sentence DR-6 asks this story to re-weigh | When writing AC-005's statement about what a bump costs | AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The briefs: `ux` U6 (`:87`) and AC-UX-010 (`:361-366`) — the consumer of this atom's quotable text — and the `testing` instrument row for AC-006 (`:469`) naming the wrong implementation | When drafting the three parts, and again before declaring the story done | AC-005 |
| `.github/workflows/ci.yml` | The `msrv` job as it actually is (`:241-260`): `cargo hack check --no-dev-deps --rust-version` on a toolchain pinned to the same 1.97.1 the gate uses. AC-006 must describe this job, not an idealised one | While writing the verification paragraph | AC-006 |
| `CHANGELOG.md` | The worked example (`:1031-1033`) of two READMEs promising *"MSRV 1.85, checked in CI"* after the floor had moved — the failure mode AC-006 exists to avoid repeating | Alongside `ci.yml`, when deciding how strongly to word the verification claim | AC-006 |
| `.kb/decisions/README.md` | The corpus's own rules: the immutability rule and the repair-vs-amendment distinction (`:7-24`), what belongs in the decisions layer (`:26-29`), and *"a decision recorded without its rejected options is indistinguishable from an accident"* (`:31-33`) | Before the wave runs, and at the approval gate when EC-002 is decided | AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | DR-6 in full (`:176-183`) — including the question about whether a bump is still a minor bump now a consumer is bound — project AC-006 (`:246-250`), AC-013 (`:273-276`) and DoD item 4 (`:297-298`) | At the start, and again when checking the story against its traced ACs | AC-008 |
| `.kb/maps/decision-map.md` | The mount point: the immutability rule restated (`:30-34`), the exact 0004/0029 amendment shape this atom joins (`:76-79`), and how a row is added (`:81-86`) | Immediately before and after the wave runs, to confirm the mount | AC-009 |
| `crates/happenstance/README.md` | The Guarantees slot as it exists today (`:41-49`), including the existing ADR-0029 bullet's absolute GitHub blob URL form (`:46-49`) and the *"checked in CI"* string this story must not repeat (`:46`) | When choosing the filename slug, and when checking the quotable parts against the real slot | AC-002 |

## Clarifications resolved during spec

- **The AC set is exactly the nine the front half enumerated.** None added, none dropped; the ledger
  matches AC-001 … AC-009.
- **Quotability is not a tenth AC.** `_design.md` names *three* parts to the stability promise
  (`:574-580`); AC-003 owns the first and AC-005 the second and third, so the density budget is
  written into those two criteria rather than floated into a separate row that would double-count the
  same text. The Interaction quality section maps each invariant to the id that carries it.
- **"Byte-identical" is measured against the branch point, not against a moving `HEAD`.** Mid-
  implementation `HEAD` includes this story's own commits; the meaningful comparison for AC-008 is the
  tree the project received.
- **The ADR number cannot be fixed at spec time.** `0030` is free today, but three slice-mates stage
  into the same wave, so the spec fixes the *constraint* (`≥ 0030`, none of `0017–0028`, not `0029`,
  unique within the wave) and not the value. The ledger therefore records the criterion, not a
  literal number.
- **No long-form record under `references/adr/`.** The corpus convention is atom-plus-record, but
  `spec/SPECIFICATION.md` cites no line range in a long form for this decision, and the staged
  `_intake` document is the long form, preserved by the wave commit.
- **This story renders no surface**, so it claims no entry in `_design.md`'s `## Items` block and no
  clause in `spec/SPECIFICATION.md` — which contains no MSRV or `rust-version` clause at all. The
  design binds it as a link target, which is why the composition invariants are real rather than
  vacuous.
- **`cargo xtask affected` is still required despite there being no Rust change.** The
  `affected_gate` in `.redkiln/config.yaml` runs the file-reading lints and `spec-trace`
  unconditionally precisely so a story that maps to no package is still gated on something.
- **The stale `"checked in CI"` string in `crates/happenstance/README.md` is deliberately left
  standing by this PR.** Fixing it is AC-UX-010's, in `guarantees-and-docs-rs-presentation`. This
  story's obligation is only that the atom does not repeat the claim.
