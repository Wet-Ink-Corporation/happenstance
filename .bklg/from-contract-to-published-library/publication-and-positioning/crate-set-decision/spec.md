---
item: HS-S0084
stage: spec
created: 2026-08-12T13:47:22.342Z
updated: 2026-08-12T13:47:22.342Z
template_sig: 87bbf1d0
rendered_sig: fa06be86
---

# Spec — The crate set for 0.2.0 is a recorded decision, not a default

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-05⁰, BR-15; DoD 9, DoD 13 |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md`:302-309 — the *decide early* flag on the crate set; `:115-121` — the 0017–0028 ADR allocation |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — DR-1 (`:154-162`), DR-13 (`:207-212`), AC-001 (`:225-228`), AC-013 (`:273-276`) |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/spec.md` |
| Key brief | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md`:574-613 — the **Deployment brief's crate-set decision**, which is where this answer was settled; `:749-755` — AC-DEP-001 |
| Grounding | `.bklg/from-contract-to-published-library/publication-and-positioning/_grounding.md`:96-125 — the tension as first flagged, and the instruction not to inherit the runbook's four silently |
| Design (binding, but silent here) | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md`:49-79 — `## Items` carries no item this story adds |
| Story map row | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md`:56, and the consumed-by table at `:105` |
| Discover stage | `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/discover.md` — the signal ledger, the decision, and the named wrong implementation (`:38-40`) |
| Roadmap pointer | `RUNBOOK.md`:4448-4499 — phase 12. Its goal line (`:4450-4451`) is the source this story overrides |

## One-line PR slice

Record, as an ingested decision atom, that `0.2.0` ships exactly `happenstance-core`, `happenstance`,
`happenstance-testkit`, naming the four-crate alternative at `RUNBOOK.md`:4450-4451 as rejected and
its cost, and assert `xtask/src/package.rs`'s derived set and its `PUBLISHABLE` intention list agree
with no reconciliation failure.

## Executive summary

**What already exists, and is not changed:** `xtask/src/package.rs`:86 holds `PUBLISHABLE =
["happenstance-core", "happenstance", "happenstance-testkit"]`; `reconcile` (`:172-218`) fails the
gate when that list disagrees with the set `cargo metadata` derives (`:226-241`); the six adapter
skeletons, the example and `xtask` all carry `publish = false`, so the derived set is those same
three names today.

**The delta this PR lands is not a change to what publishes — it is a change to the *status* of that
fact.** Three crates is currently an unexamined constant with one corroborating sentence in
`CLAUDE.md`:280 and a *contradicting* four-crate goal in `RUNBOOK.md`:4450-4451. After this PR the
set is an accepted decision atom in `.kb/decisions/` (number 0030 or above), authored through
`.kb/_intake/` and `/redkiln:kb-ingest`, naming the four-crate alternative and what it would have
cost; the runbook's stale goal carries a pointer to it so the plan of record cannot be acted on as
if nothing decided; and the atom's claim of agreement is backed by a **captured run** of `cargo
xtask package-check` at the decision commit rather than by a sentence that assumes it.

Three sibling stories block on this landing (`projection-port-ship-shape`, `registry-surface-diff`,
`landing-copy-and-status-truth`) and `publish-0-2-0` publishes exactly the set named here, which is
why it is first in merge order and why it is a foundation rather than a capability.

## Context pack

Everything in this section is a decision already taken. This story executes it; it does not
re-litigate it. Depth stays behind the anchors table below.

**1. The set is three, and the names are fixed.** `0.2.0` ships `happenstance-core`, `happenstance`
and `happenstance-testkit` — the deployment brief's decision
(`publication-and-positioning/_decomposition.md`:574-583), matching `xtask/src/package.rs`:86 and
`CLAUDE.md`:280. `happenstance` is the crate a consumer installs and the one holding the bare name
(`.kb/decisions/0006-bare-name-to-the-typed-layer.md`); `happenstance-core` is what an adapter author
pins; `happenstance-testkit` is the bar an adapter clears. That is a set of three publishable
artifacts, not a preference about emphasis.

**2. The alternative that lost is four crates, and it must be named with its cost.** `RUNBOOK.md`
phase 12 states the goal as *"`happenstance-core`, `happenstance`, `happenstance-testkit` **and
`happenstance-sqlite`** on crates.io"* (`:4450-4451`). It is rejected on the deployment brief's three
grounds (`publication-and-positioning/_decomposition.md`:585-608), and those grounds are **copied
forward, not re-derived**:

- `PUBLISHABLE` and `CLAUDE.md` already encode three as the intention `reconcile` checks against;
  a fourth is a change to that constant, which is a decision this project must take deliberately.
- A fourth crate owes `LICENSE-MIT`, `LICENSE-APACHE` and `README.md` inside its own package
  directory to clear `REQUIRED_FILES` (`xtask/src/package.rs`:88-94), plus a status-table row, a
  disambiguation entry and a docs.rs configuration on the UX surface — uncosted scope this project's
  `_intake-brief.md` does not carry. `sqlite-durable-store` being upstream and conformant is not the
  same claim as having a publish-ready package surface.
- `RUNBOOK.md`'s phase-12 section is independently known stale: it still cites a superseded
  `#the-46-provisional-clauses` anchor (`RUNBOOK.md`:4495) against the ledger heading corrected to 49
  (`RUNBOOK.md`:622-635, `spec/SPECIFICATION.md`:220). It is the source overridden here — not
  `CLAUDE.md` and not `xtask/src/package.rs`.

**3. Agreement is a thing you run, not a thing you write.** AC-001's *"no reconciliation failure"* is
a property of executing `reconcile`, and the story's named wrong implementation is precisely an atom
that reads correctly while nobody ran the check (`discover.md`:38-40). `PUBLISHABLE` could already
have drifted — a `publish = false` dropped from an adapter, a member added under a new path — and
prose would not notice. The evidence this story produces is the literal stdout of `cargo xtask
package-check` at the decision commit, including the line `reconcile` prints on success
(`xtask/src/package.rs`:213-217).

**4. A reconciliation failure is a stop, not a repair.** `reconcile` names the direction of the drift
because the two directions are two different bugs (`xtask/src/package.rs`:163-204). If it reports a
*promoted* crate, that is a `publish = false` gone missing or a promotion nobody costed; if it
reports a *withdrawn* one, the constant is stale. Neither is fixed by editing `PUBLISHABLE` to make
the gate green — a crate-set change is a re-plan that moves `PUBLISHABLE`, the project's
`_intake-brief.md` scope and the deployment brief together
(`publication-and-positioning/_decomposition.md`:610-613, `_decomposition.md`:302-309).

**5. The atom is ingested, never hand-written, and `/redkiln:kb-ingest` is a human handoff.** Atoms
are authored by `/redkiln:kb-ingest` from `.kb/_intake/`; hand-authoring produces the directory
layout of the process without the process, and the first attempt at it was reverted (`0269720`,
`CLAUDE.md`:103-105). The implementer's deliverable is therefore a **staged intake document** plus
the handoff — the redkiln SDLC commands are user-invoked and cannot be called from inside the
implementing context. Two operational facts about that wave: the ingest glob picks up
`.kb/_intake/README.md`, which must be dropped at the approval gate rather than ingested; and a
successful wave **clears** `_intake/` — a file still sitting there afterwards is a file that run did
not ingest (`.kb/_intake/README.md`:3-5, `:13-19`).

**6. The number is 0030 or above, and the slice shares one wave.** `.kb/decisions/` holds 0001–0016
and 0029; 0017–0028 are allocated to sibling projects (`_decomposition.md`:115-121). So this
project's atoms take the next free numbers above the corpus (`project.md`:207-212). The
`release-decisions` milestone deliberately produces four atoms in **one** ingest pass rather than
four (`_storymap.md`:73-78), so this story — first in merge order — states the slice-wide allocation
in its intake document so the wave cannot collide with itself.

**7. Nothing accepted may be edited, and nothing here is a supersession.** `redkiln validate --kb`
checks every `status: accepted` decision atom against `HEAD`
(`.kb/decisions/README.md`:7-13). Nothing in `.kb/` currently mentions the publishable set at all —
grep finds no atom naming `PUBLISHABLE` or `package-check` — so there is nothing to merge into and
nothing to supersede. The nearest neighbour is `kb-decision-0006`, which decided *which crate holds
the bare name*, not *which crates publish*; it is a `related` edge, not a `supersedes` edge.

**8. No persona meets this story, and that is stated rather than papered over.** Backbone activity A1
has no user intent of its own — every story in it is a foundation consumed by A3 and A4
(`_storymap.md`:45-50). The evaluator persona meets this decision's *consequence*: three crates.io
pages rather than four, and a "Which crate do I want?" block with three answers. The named consumers
are `registry-surface-diff` (what it diffs), `landing-copy-and-status-truth` (the status table and
the disambiguation triad) and `publish-0-2-0` (what it publishes) (`_storymap.md`:105).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate (a decision atom of record plus a verified reconciliation), consumed inside this project by three named stories rather than left as an unproven island (`_storymap.md`:100-108) |
| **Slice / milestone** | `release-decisions`. Slice-mates, implemented in one context and mounted as one integrated wave: `projection-port-ship-shape`, `msrv-promise-atom`, `first-contact-design-resolutions` |
| **Mount point** | `xtask/src/package.rs` — the `PUBLISHABLE` constant (`:86`) reached through the **mandatory** `package-check` step registered in `xtask/src/main.rs`'s `REQUIRED` list (`:515-532`, dispatched at `:680`). That is the real composition root here: the decision is only mounted when the gate that reconciles it runs on this tree. Second mount, in the knowledge tree: the new atom at `.kb/decisions/00NN-…md` plus its row in `.kb/maps/decision-map.md` — an atom with no map row is an atom the corpus cannot see (`.kb/maps/decision-map.md`:81-86) |
| **Wires into** | `reconcile` (`xtask/src/package.rs`:172-218) and `publishable_members` (`:226-241`) — the derived-vs-intention pair; `REQUIRED_FILES` (`:88-94`) — what a fourth crate would owe; the six `publish = false` manifests (`crates/happenstance-{sqlite,cloudflare,ladybug,postgres,neon,sync}/Cargo.toml`:12) that make the derived set three; `.kb/decisions/README.md`'s immutability rule; `.kb/_intake/` as the ingest inlet; `.redkiln/config.yaml`'s story-grain `affected_gate` (`:40`) |
| **Renders surfaces** | **none.** `_design.md`'s `## Items` (`:49-79`) carries no item this story adds and `## Surfaces` (`:104-176`) names no surface it renders. The honest coupling is one level down: the three `crates-io-*` and three `docs-rs-*` surfaces exist as three pairs *because* the set is three — a fourth crate would add a surface pair the signed-off design does not carry |
| **Conformance rule(s)** | **none, and it is not adapter-observable.** No `crates/happenstance-testkit/src/suite.rs` rule can observe which crates publish; the observing instrument is the gate step named above. Stated explicitly because a story that names no rule and changes no port must say which of the two it is |
| **Clause(s)** | **none amended.** No `spec/SPECIFICATION.md` clause states the publishable set, and nothing `[FROZEN]` is touched — AC-015 and DR-15 hold trivially here and the diff is expected to prove it |
| **Advances DoD scenario** | Initiative **DoD 9** — *"a stranger can install it"* — by deciding, on the record, which crate the stranger installs and which three pages must be finished; and supports **DoD 13** (*the gate green on the assembled whole*) by keeping `package-check` green on a tree whose intention list is now a decision. At project grain it advances DoD 1 (AC-001), DoD 4 (`validate --kb` / `doctor` clean) and DoD 6 (AC-001 and AC-013 covered) |

## PR boundary

```
.kb/_intake/**
.kb/decisions/**
.kb/maps/decision-map.md
.kb/_governance/integration-waves/**
.redkiln/telemetry/**
RUNBOOK.md
.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/**
```

**In this PR**

- The staged intake document under `.kb/_intake/`, and the atom, decision-map row and wave record
  the `/redkiln:kb-ingest` run writes from it (`.kb/decisions/`, `.kb/maps/decision-map.md`,
  `.kb/_governance/integration-waves/`, and the telemetry that wave commits).
- One pointer line at `RUNBOOK.md`'s phase-12 **Goal** paragraph (`:4450-4451`) naming the atom as
  the source in force, so the plan of record cannot be read as authorising a fourth crate.
- The captured `cargo xtask package-check` output, and this story's `spec.md` / `_ledger.md`.

**Explicitly not in this PR**

- **Any edit to `xtask/src/package.rs`.** `PUBLISHABLE` already says three; the story records why,
  it does not change the constant. A change there means the decision moved, which is a re-plan.
- **Any manifest change** — no `publish = false` is removed, no crate is promoted, no adapter
  project gains a package surface. That is the whole cost the four-crate alternative was rejected on.
- **Any other region of `RUNBOOK.md`** — in particular the provisional falsifier ledger at
  `:622-635`, which `falsifier-ledger-repair` owns and must land before anything reads it
  (`_storymap.md`:179-184).
- **The three sibling atoms** of this slice (PS-3's ship shape, the MSRV promise, DT-1/4/5/6) — this
  story only reserves their numbers so one wave does not collide.
- **Any published-surface copy.** The status table, the disambiguation triad and the
  "Which crate do I want?" block are `landing-copy-and-status-truth`'s.
- **A long-form record under `references/adr/`.** That directory holds the seventeen imported ADRs
  whose line ranges `spec/SPECIFICATION.md` cites; nothing cites a long form for this decision, and
  the atom is canonical for it.

**Merge DoD (one line).** The atom exists and is accepted, names the four-crate alternative and its
cost, `redkiln validate --kb` and `redkiln doctor` are clean, and a captured `cargo xtask
package-check` run at the head commit shows the derived set and `PUBLISHABLE` agreeing on exactly
the three names the atom states.

## Behavior and interfaces

**There is no Rust interface change.** No public item is added, removed or re-signatured — `_design.md`'s
`## Items` block is empty of anything this story owns, and `project.md`'s Out of scope reserves any API
surface change for an upstream project and a re-plan. The interfaces this story does touch are the gate's
(`package-check`) and the knowledge base's (`KbFrontmatter`).

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **B1 — the set is stated as three names** | The atom's decision sentence names `happenstance-core`, `happenstance`, `happenstance-testkit` as the exact set published at `0.2.0`, and states that the set is a *decision* whose change is a re-plan rather than an edit | `xtask/src/package.rs`:86; `CLAUDE.md`:280; `publication-and-positioning/_decomposition.md`:574-583 |
| **B2 — the alternative that lost is named with its cost** | An `Alternatives rejected` section names the four-crate goal *verbatim from the runbook*, carries the three rejection grounds unchanged (intention-list change; licence/README/status-row/docs.rs scope for a fourth crate; the runbook section's independently-known staleness), and states which source is being overridden | `RUNBOOK.md`:4450-4451, `:4495`; `publication-and-positioning/_decomposition.md`:585-608; `_grounding.md`:96-125 |
| **B3 — the atom reaches `.kb/` through the ingest path only** | The implementer writes one staged document at `.kb/_intake/<nnnn>-the-0-2-0-crate-set.md` and then **hands off**: the human runs `/redkiln:kb-ingest`, whose wave authors the atom, syncs the maps, wires backlinks, runs `redkiln validate --kb`, clears `_intake/` and commits on its own branch. `.kb/_intake/README.md` is dropped at the wave's approval gate, never ingested | `CLAUDE.md`:103-105; `.kb/_intake/README.md`:3-5, `:13-19` |
| **B4 — the atom's frontmatter is valid `KbFrontmatter` of the decision shape** | `kind: decision`, `authority_tier: decision`, `status: accepted`, `adr_id: ADR-00NN`, `phase: 12`, `reversibility` stated, `supersedes: null`, `superseded_by: null`, `related` carrying `kb-decision-0006` (which crate holds the bare name — adjacent, not superseded), `source_paths` citing the intake file, `xtask/src/package.rs` and `RUNBOOK.md` | `.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33 as the shape of record; `.kb/decisions/README.md`:27-33 |
| **B5 — the number is free, and the wave's allocation is stated once** | The intake document requests the lowest number ≥ 0030 not taken by the 0017–0028 sibling allocation or by 0029, and records the `release-decisions` allocation (this story first, then the ship shape, the MSRV promise and the first-contact resolutions) so one wave does not collide. The id the wave actually assigns is what the ledger cites | `project.md`:207-212; `_decomposition.md`:115-121; `_storymap.md`:73-78, `:156-160` |
| **B6 — agreement is executed and captured, not asserted** | `cargo xtask package-check` is run at the decision commit and its stdout committed under this story's folder: the `reconcile` success line naming the three crates, plus the three per-crate `N files packaged, including LICENSE-MIT, LICENSE-APACHE, README.md` lines. The exit status is part of the evidence | `xtask/src/package.rs`:103-161, `:213-217`; `discover.md`:38-40 |
| **B7 — a reconciliation failure stops the story** | If `reconcile` reports a promoted or withdrawn crate, the story does **not** edit `PUBLISHABLE` or a manifest to clear it. The failure's named direction is recorded, and a genuine crate-set change becomes a re-plan touching `PUBLISHABLE`, `_intake-brief.md` and the deployment brief together | `xtask/src/package.rs`:163-204; `publication-and-positioning/_decomposition.md`:610-613 |
| **B8 — the derived set is three because six crates decline to publish** | The evidence records *why* the derived set is what it is: `publish = false` at `crates/happenstance-{sqlite,cloudflare,ladybug,postgres,neon,sync}/Cargo.toml`:12, plus `examples/course-subscriptions` and `xtask`. This is what makes "a `publish = false` gone missing" a real, checkable failure mode rather than a hypothetical | `crates/happenstance-sqlite/Cargo.toml`:12 and the five siblings; `xtask/src/package.rs`:24-42 |
| **B9 — the overridden source carries a pointer** | `RUNBOOK.md`'s phase-12 Goal paragraph gains one line naming the atom as the source in force for the crate set, so a reader of the plan of record meets the correction where the stale claim lives. The runbook's prose is otherwise untouched, and the falsifier ledger at `:622-635` is not entered | `RUNBOOK.md`:4448-4451; `_storymap.md`:179-184 |
| **B10 — nothing accepted, and nothing else, moves** | `.kb/decisions/0001`…`0016` and `0029` are byte-identical after the wave; `xtask/src/package.rs`, every `Cargo.toml` and `CLAUDE.md` are unmodified; the diff contains only paths inside the PR boundary | `.kb/decisions/README.md`:7-13; `.redkiln/config.yaml`:40 (story-grain affected gate) |
| **B11 — the corpus can find it** | `.kb/maps/decision-map.md` carries a row for the new atom under a new wave section, in ADR-number order, with no superseded row deleted; `redkiln validate --kb` and `redkiln doctor` are clean, `doctor` reporting exactly the six expected `template-drift` advisories and no `dependency-cycle` | `.kb/maps/decision-map.md`:81-86; `project.md`:297-298; `CLAUDE.md` (the six-advisory assertion) |
| **B12 — the decision is readable by the stories that block on it** | The atom states the set as the three names its consumers can quote directly — `registry-surface-diff` diffs exactly this set, `landing-copy-and-status-truth` writes a three-answer disambiguation block, `publish-0-2-0` publishes exactly it — so none of the three re-derives the answer | `_storymap.md`:105; `story.md` frontmatter `blocks:` |

## Data and migrations

**N/A — no schema, no migration, no backfill.** This library defines no storage schema of its own
(each adapter owns its own), this story writes no code and touches no manifest, and `0.2.0` is the
first real publish of these three crates: the reserved `0.0.0` names are a registry reservation with
no compatible predecessor, so there is nothing a consumer could have depended on in a way this
release must migrate (`publication-and-positioning/_decomposition.md`:643-660,
`CONTRIBUTING.md`:299-300).

The only durable state this story writes is **knowledge-base state**, and it has its own one-way
discipline rather than a migration: the ingest wave adds an atom, adds a decision-map row, records
the wave under `.kb/_governance/integration-waves/`, and clears `.kb/_intake/`. There is no reverse
migration for an accepted decision atom — the correction path is a new atom carrying `supersedes`,
never an edit (`.kb/decisions/README.md`:7-13). Rollback of this PR before the wave is merged is an
ordinary branch discard; after it, the crate-set answer changes by superseding atom plus a re-plan
that moves `PUBLISHABLE`, `_intake-brief.md` and the deployment brief together.

## Acceptance criteria

Each criterion is written from the intent of the person on the other side of it. Two people are on
the other side of this story. The **repository owner deciding once** (`_storymap.md`:40 — *"the
maintainer, deciding once"*) is the actor: they need the crate-set answer to stop being a constant
someone can quietly disagree with, and they need the answer to be true of the tree rather than
merely written down. **Persona 4, the evaluator** (`_discovery/distillation/personas-and-journeys.md`:249-313,
`:333-338`) is downstream: they get one sitting, and what they meet is the *consequence* — three
registry pages and a three-answer *"Which crate do I want?"* triad, not four of either. Neither of
them ever reads this atom directly; both of them are wrong-footed if it is absent or false.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** the repository owner has decided once that `0.2.0` ships exactly `happenstance-core`, `happenstance` and `happenstance-testkit`, **WHEN** they (or any later reader) open `.kb/decisions/`, **THEN** an accepted decision atom at a number ≥ 0030 states that set — **exactly three names, listed** — in the decision-atom shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33): valid `KbFrontmatter` with `kind: decision`, `authority_tier: decision`, `status: accepted`, `supersedes: null`, `superseded_by: null`, a `related` edge to `kb-decision-0006` and `source_paths` naming the intake file, `xtask/src/package.rs` and `RUNBOOK.md`; **AND** the atom defines the vocabulary it uses — *intention list* (`PUBLISHABLE`) and *derived set* (`cargo metadata`) — where it uses them rather than sending the reader elsewhere for the definition; **AND** the set is stated as three so the signed-off `≤ 3 entries` disambiguation-triad budget (`_design.md`:474, region R3 at `:359`) is satisfiable by the downstream copy story without re-deciding anything | `redkiln validate --kb` exits 0 with the new atom present; a content check that the atom names all three crates, carries the six frontmatter fields above, and defines both terms inline; `git ls-files .kb/decisions/` shows exactly one added path and its number is ≥ 0030 and not in 0017–0028 |
| AC-002 | **GIVEN** `RUNBOOK.md`:4450-4451 states the phase-12 goal as four crates including `happenstance-sqlite`, **WHEN** the owner or a future planner reads the new atom, **THEN** an *Alternatives rejected* section quotes that four-crate goal verbatim, names it as the source being overridden, and carries its three costs unchanged from the deployment brief (`publication-and-positioning/_decomposition.md`:585-608) — the intention-list change, the `LICENSE-MIT` / `LICENSE-APACHE` / `README.md` plus status-row plus docs.rs scope a fourth crate owes (`xtask/src/package.rs`:88-94), and the phase-12 section's independently-known staleness — **AND** the cost is readable *in the atom* with the runbook citation as the only hop needed to check it (IQ-1's 0-hops-to-a-claim, ≤ 1-hop-to-evidence budget, `publication-and-positioning/_decomposition.md`:229-237) | a content check that the atom contains the verbatim four-crate goal string, the three named grounds, and the words identifying `RUNBOOK.md` as overridden; a link/citation check that every `file:line` in that section resolves in the tree at HEAD |
| AC-003 | **GIVEN** the atom claims the intention list and the manifests agree, **WHEN** the owner needs that claim to be checked rather than trusted, **THEN** `cargo xtask package-check` has been run at the decision commit and its **stdout is a committed artefact** under this story's folder, carrying the commit sha, the date, the literal command line, the exit status, `reconcile`'s success line `publishable set agrees with the manifests: happenstance-core, happenstance, happenstance-testkit` (`xtask/src/package.rs`:213-217) and the three per-crate lines naming `LICENSE-MIT`, `LICENSE-APACHE` and `README.md`; **AND** the atom cites that artefact rather than restating `PUBLISHABLE`'s contents — no claim on it has *"run the gate yourself"* as its only evidence (IQ-5) | the committed capture exists, exit status is 0, and its `reconcile` line names exactly the three crates in the atom; `cargo xtask package-check` re-run at HEAD reproduces it; `cargo test -p xtask package::tests` (the three tests at `xtask/src/package.rs`:408-457) is green |
| AC-004 | **GIVEN** the named wrong implementation is an atom that reads correctly while nobody ran the check (`discover.md`:38-40), **WHEN** the check is seeded with the exact drift it exists to catch — `publish = false` deleted from `crates/happenstance-sqlite/Cargo.toml`:12 in the working tree only — **THEN** `cargo xtask package-check` fails, and its message names the direction as *promoted* and tells the reader to copy the three required files or restore the missing `publish = false` (`xtask/src/package.rs`:186-196); **AND** the seeded edit is reverted and appears in no commit; **AND** if the *unseeded* run had failed in either direction, the story stops and records it rather than editing `PUBLISHABLE` or a manifest to make the gate green (reversibility is bought before the irreversible act, not after it — IQ-4) | the seeded-failure transcript is committed alongside AC-003's passing one; `git diff --quiet -- crates/` and `git diff --quiet -- xtask/` at HEAD prove the seed was reverted and no manifest or `xtask` source moved |
| AC-005 | **GIVEN** an accepted decision atom is immutable and the corpus is only navigable through its map, **WHEN** the wave that authors this atom lands, **THEN** `.kb/maps/decision-map.md` carries a row for it in ADR-number order under a new wave section with no superseded row deleted (`.kb/maps/decision-map.md`:81-86); **AND** `.kb/decisions/0001`…`0016` and `0029` are byte-identical to HEAD~, `xtask/src/package.rs` and every `Cargo.toml` are unmodified, and the diff contains only paths inside the PR boundary; **AND** `redkiln validate --kb` and `redkiln doctor` are clean, `doctor` reporting exactly the six expected `template-drift` advisories and no `dependency-cycle`; **AND** every sentence in the atom is true of the tree at that commit (IQ-7) | `redkiln validate --kb` and `redkiln doctor` exit 0 with the six-advisory/zero-cycle shape; `git diff --stat` against the merge base is a subset of the PR boundary; `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) green |
| AC-006 | **GIVEN** the plan of record still states a four-crate goal, **WHEN** a reader arrives at `RUNBOOK.md`'s phase-12 **Goal** paragraph, **THEN** they meet a single added pointer line naming the atom as the source in force for the crate set — the stale sentence is **annotated, not deleted** (IQ-2's *annotate, never subtract*), no heading is renamed and no anchor moves (IQ-3), and no other region of `RUNBOOK.md` — the provisional falsifier ledger at `:622-635` above all — is entered; **AND** the three stories that block on this one (`registry-surface-diff`, `landing-copy-and-status-truth`, `publish-0-2-0`, `_storymap.md`:105) can quote the set from the atom without re-deriving it, which is what keeps the evaluator's first contact three pages and a three-answer triad rather than four | `git diff RUNBOOK.md` shows added lines only, inside `:4448-4460`, with no heading line changed and no line removed; a heading-anchor check that every `#`-anchor `RUNBOOK.md` exposed before the edit still exists; the atom's decision sentence is quotable verbatim as the three consumers' input |

**Coverage of the traced project ACs.** Project **AC-001** (`project.md`:225-228 — *the set matches a
recorded decision that names the rejected alternative, and the derived set and intention list agree
with no reconciliation failure*) is carried by AC-001 (the recorded decision), AC-002 (the named
alternative) and AC-003 + AC-004 (agreement, executed and proven non-decorative). Project **AC-013**
(`project.md`:273-276 — *authored through the ingest path, numbered free of 0017–0028 and 0029*) is
carried by AC-001's numbering and shape check and AC-005's `validate --kb` / immutability check.

## Interaction quality

This story **renders no surface** — `_design.md`'s `## Items` (`:49-79`) carries nothing it adds and
`## Surfaces` (`:104-176`) names nothing it renders. So the composition family does not apply *as
authorship*; it applies as a **constraint this decision hands to the design that is already signed
off**, and that constraint is written into AC-001 rather than left as prose here. The state family
does apply, because the UX brief's IQ-1…IQ-7 are stated for this medium — *"the reader's
'interaction' is navigating a claim to its evidence and back inside one sitting"*
(`publication-and-positioning/_decomposition.md`:226-310) — and the reader here is the next person to
open the plan of record.

Every invariant below is carried by an `AC-###` row in the table above. This section says **which row
carries which**, and nothing is asserted here that is not gated there.

### State invariants

| Invariant (UX brief) | Carried by | How it is verified |
| --- | --- | --- |
| **IQ-1 — in place, not a context jump.** The decision and its cost are readable in the atom; the runbook and the deployment brief are the ≤ 1 hop, and each hop lands on a line range, never on a document root | AC-001, AC-002 | the content check for the three grounds *in the atom*, plus the `file:line` resolution check in AC-002 |
| **IQ-2 — non-occlusion: annotate, never subtract.** The rejected four-crate alternative stays visible *with its reason* instead of being deleted from the record; the runbook's stale goal is annotated with a pointer, not silently rewritten. Deleting the losing option would be the filter hiding what it filters | AC-002, AC-006 | `git diff RUNBOOK.md` shows **added lines only**; the atom's *Alternatives rejected* section is present and non-empty |
| **IQ-3 — preserved position: inbound anchors survive.** The runbook edit renames no heading and moves no anchor; the decision map gains a row and deletes none, including superseded rows | AC-005, AC-006 | the heading-anchor check on `RUNBOOK.md`; the decision-map row-count assertion (rows only added) |
| **IQ-4 — reversibility bought before the act.** An accepted atom is irreversible in the same way a publish is: the correction path is a superseding atom, never an edit (`.kb/decisions/README.md`:7-13). So the reconciliation run and the seeded-failure proof both happen **before** the ingest handoff, not after it | AC-003, AC-004 | the passing capture and the seeded-failure transcript are both committed, and both predate the ingest wave's commit |
| **IQ-5 — reachable without running anything.** The atom's evidence is a committed artefact with a sha and a date, not *"run `cargo xtask ci`"* | AC-003 | the capture is a file in the tree carrying commit sha, date, command line and exit status |
| **IQ-6 — no orphan vocabulary.** *Intention list*, *derived set* and `PUBLISHABLE` are defined in the atom where they are used | AC-001 | the content check for both inline definitions |
| **IQ-7 — truth at the commit.** No sentence in the atom is false of the tree it describes | AC-003, AC-005 | the `reconcile` line in the capture equals the set the atom names; `git diff --stat` bounded by the PR boundary |
| **Keyboard reachability / focus / scroll / selection** | **not applicable, stated rather than skipped** | there is no focusable surface: the artefacts are markdown files and a captured stdout. The medium's analogue — *does the reader keep their place* — is IQ-3, which is gated above |

### Composition invariants (from the signed-off `_design.md`)

This story authors none of them and must not contradict any. It **constrains exactly one**, and that
constraint is a criterion, not a bullet:

- **The disambiguation triad's `≤ 3 entries` budget** (`_design.md`:474; region R3 at `:359`; the
  14-rendered-line first screen at `:452-456`) exists *because* the published set is three — the
  design says so in the budget's own rationale: *"A fourth entry means a fourth published crate,
  which AC-DEP-001 has decided against."* **AC-001** therefore requires the decided set to be
  exactly three named crates, which is what makes that budget satisfiable by
  `landing-copy-and-status-truth` without re-opening the design.
- **Presentation exists at all**, in this medium, means the atom is a *composed* decision atom in the
  shape of record (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`:1-33) with real frontmatter, a map
  row and a supersession graph — not bare prose in a file with a heading. **AC-001** and **AC-005**
  carry it; an atom with no map row is an atom the corpus cannot see.
- **Named anti-patterns.** `_design.md`'s AP-1…AP-15 (`:641-676`) all attach to rendered surfaces
  this story does not author, so none is assertable here. The nearest miss is **AP-11** (the 8-row
  status table on a packaged page) — it is `landing-copy-and-status-truth`'s to hold, and this story
  must not pre-empt it by writing surface copy. That is already an explicit exclusion in the PR
  boundary.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `reconcile` reports a **promoted** crate — `cargo metadata` derives a name `PUBLISHABLE` does not carry (`xtask/src/package.rs`:186-196) | **Stop.** A `publish = false` has gone missing, or a crate was promoted without costing its licence/README/status-row/docs.rs surface. Record the message verbatim in the story folder and halt: do **not** add the name to `PUBLISHABLE` and do **not** land the atom. A genuine promotion is a re-plan that moves `PUBLISHABLE`, `_intake-brief.md` and the deployment brief together (`publication-and-positioning/_decomposition.md`:610-613) |
| **EC-002** | `reconcile` reports a **withdrawn** crate — `PUBLISHABLE` names something Cargo will not publish (`xtask/src/package.rs`:198-204) | **Stop**, and note that this is the *other* bug: the constant is stale or a crate gained a `publish = false`. Same prohibition — the fix is not an edit that makes the gate green. The two directions are recorded separately, because the message exists to say which one moved |
| **EC-003** | `cargo metadata` fails, or emits a `publish` shape `scan_publishable` does not understand (`xtask/src/package.rs`:452-458) | The gate stops and says so; the schema is not guessed at. Treat as a blocked story and report the toolchain/schema drift — this is the failure mode the scanner's third unit test exists for |
| **EC-004** | The `/redkiln:kb-ingest` wave assigns an ADR number that collides with a slice-mate's atom (`projection-port-ship-shape`, `msrv-promise-atom`, `first-contact-design-resolutions`) | The intake document states the whole `release-decisions` allocation up front (`_storymap.md`:73-78) precisely so one wave cannot collide with itself. If it happens anyway, the **assigned** id is authoritative and every citation — this story's `_ledger.md` above all — is updated to it; no atom is renamed after acceptance |
| **EC-005** | After the wave, a file is still sitting in `.kb/_intake/`, or `.kb/_intake/README.md` was itself ingested | A file left behind is a file that run did not ingest (`.kb/_intake/README.md`:13-19); the README is dropped at the wave's approval gate, never ingested. Neither is repaired by hand-editing `.kb/`: re-run the wave over the remaining file |
| **EC-006** | `redkiln validate --kb` reports a diff against an accepted atom | An accepted body was edited, which is exactly what the check exists to catch (`.kb/decisions/README.md`:7-13). Revert the edit. If the content genuinely needs correcting, that is a **new atom carrying `supersedes`**, and it is out of this story's scope |
| **EC-007** | `redkiln doctor` reports a seventh `template-drift` advisory, or a `dependency-cycle` | Stop: a seventh is a template changed without a decision, and this story changes no template. Do not run `redkiln adopt --templates` — `CLAUDE.md` forbids it, and it would overwrite all six deliberate customisations |

## Non-functional

| id | requirement | why, and how it is observed |
| --- | --- | --- |
| **NF-001** | **Zero Rust and zero manifest change.** No `.rs`, no `Cargo.toml`, no `Cargo.lock` appears in the diff | The whole cost the four-crate alternative was rejected on is package-surface scope; a story that records that rejection and then edits a manifest has spent the cost anyway. Observed by `git diff --name-only` against the merge base being a subset of the PR boundary |
| **NF-002** | **The story-grain gate is green and is not trivially green.** `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) passes, and it is meaningful because it runs the file-reading lints and `spec-trace` unconditionally even when the diff touches no package | A story whose whole deliverable is markdown must still be gated on something; that is the reason the unconditional half of `affected` exists (`publication-and-positioning/_decomposition.md`, *Merge-gate commands*) |
| **NF-003** | **Durability is one-way and stated.** After acceptance, the crate-set answer changes only by a superseding atom plus a re-plan; there is no reverse migration | `.kb/decisions/README.md`:7-13; already written into *Data and migrations* above, and enforced by `validate --kb` at every later checkpoint (AC-TEST-003, `publication-and-positioning/_decomposition.md`:533-535) |
| **NF-004** | **The evidence artefact is self-dating.** The captured run carries the commit sha, the ISO date, the literal command line and the exit status | A transcript without its commit proves nothing about *this* tree; the project's DoD item 3 requires committed artefacts *with a date*, not remembered observations (`project.md`, Definition of done) |
| **NF-005** | **Reproducible on a clean checkout.** `cargo xtask package-check` reproduces the captured output from a fresh clone at the decision commit, with no network access beyond what `cargo metadata --no-deps --locked` needs | `--locked` is already in the command (`xtask/src/package.rs`:230-232); a capture that only reproduces on the author's machine is not evidence |

## Implementation notes (non-prescriptive)

**Order matters here more than technique, because one of the steps is irreversible.**

1. **Run the check first, before writing a word of the atom.** `cargo xtask package-check` at the
   commit you intend to be the decision commit. If it fails, EC-001/EC-002 apply and the story stops
   — the atom is never written against a tree that disagrees with it.
2. **Capture, do not paraphrase.** Redirect stdout *and* stderr and the exit status into a file under
   this story's folder; prepend the sha, the date and the command line. The `reconcile` success line
   is the load-bearing sentence (`xtask/src/package.rs`:213-217).
3. **Prove the check is not decorative.** Delete `publish = false` from
   `crates/happenstance-sqlite/Cargo.toml`:12 **in the working tree only**, re-run, capture the
   failure, then `git checkout --` the manifest. This is `CLAUDE.md`'s *"a rule that no adapter can
   fail is decorative"* corollary applied one level up, to a gate step rather than a conformance
   rule, and it is the discipline the testing brief asks of every instrument in this project.
4. **Write one staged intake document**, not an atom, at `.kb/_intake/<nnnn>-the-0-2-0-crate-set.md`.
   Hand-authoring under `.kb/decisions/` is prohibited and was reverted once already (`0269720`,
   `CLAUDE.md`:103-105). Model its body on the shape `0029-msrv-raised-to-1-97-1.md` has, since that
   is the most recent atom the ingest path produced.
5. **State the slice's number allocation inside that document.** This story is first in merge order,
   so it is the one that says which numbers the other three `release-decisions` atoms take. The wave
   assigns the real ids; whatever it assigns is what the ledger cites.
6. **Then stop and hand off.** `/redkiln:kb-ingest` is user-invoked and cannot be called from the
   implementing context. The wave authors the atom, syncs `.kb/maps/decision-map.md`, wires
   backlinks, runs `redkiln validate --kb`, clears `.kb/_intake/` and commits on its own branch.
7. **The runbook pointer is the last edit**, and it is one line inside `:4448-4460`. It cannot be
   written before the wave, because it names the atom's real id. Stay out of `:622-635` entirely —
   that region is `falsifier-ledger-repair`'s and it lands separately (`_storymap.md`:179-184).

Two things worth naming so they are not discovered late. The atom's `related` edge points at
`kb-decision-0006`, which decided *which crate holds the bare name* — adjacent, not superseded; a
`supersedes` edge there would be wrong and `validate --kb` will not catch a wrong edge, only a
malformed one. And nothing in `.kb/` currently mentions the publishable set at all, so there is
nothing to merge into: this is a new atom, and the ingest adjudication's usual MERGE-over-new
preference has nothing to prefer.

## Tests and CI (merge gate)

Grounded in the testing brief's own mapping for project AC-001 — *unit + static, `reconcile` plus the
`package-check` gate step, rejecting `PUBLISHABLE` naming a crate `cargo metadata` does not derive
(or the reverse)* — and AC-013 — *static (process), `redkiln validate --kb` over every atom, each
from the ingest path* (`publication-and-positioning/_decomposition.md`:460-480).

| tier | command / path | proves |
| --- | --- | --- |
| **static (gate step)** | `cargo run -p xtask -- package-check`, i.e. the mandatory step registered at `xtask/src/main.rs`:515-532 and dispatched at `:680` | AC-003: the derived set and `PUBLISHABLE` agree, and the three artifacts each contain both licences and a README. This is the observing instrument — the decision is only *mounted* when this step runs on this tree |
| **unit** | `cargo test -p xtask package::tests` — the three tests at `xtask/src/package.rs`:408-457 | AC-003: `scan_publishable` reads all three `publish` spellings, ignores dependency-depth and workspace-`metadata` decoys, and **fails** on a shape Cargo does not emit. The precedent shape the testing brief holds every new instrument to (AC-TEST-002) |
| **seeded negative (manual, working tree only)** | delete `publish = false` from `crates/happenstance-sqlite/Cargo.toml`:12 → `cargo run -p xtask -- package-check` → revert | AC-004: the gate is not decorative. The failure must name the *promoted* direction, which is the half of the message no other gate step could ever produce |
| **static (process)** | `redkiln validate --kb` | AC-001, AC-005: `KbFrontmatter` conformance on the new atom, and every accepted atom byte-identical to `HEAD`. This is the check that catches an edited `0001`…`0016`/`0029` |
| **static (process)** | `redkiln doctor` | AC-005, EC-007: exactly six `template-drift` advisories and zero `dependency-cycle`, per `CLAUDE.md`'s backlog-CI assertion. Required at *this* checkpoint, not only at project end (AC-TEST-003) |
| **static (diff)** | `git diff --name-only <merge-base>..HEAD` asserted against the PR boundary; `git diff --quiet -- crates/ xtask/ CLAUDE.md` | NF-001, AC-004, AC-005: no manifest, no `xtask` source and no `CLAUDE.md` moved, and the seeded edit was reverted |
| **static (content)** | a read of the accepted atom against AC-001/AC-002's checklist; `file:line` resolution of every citation in its *Alternatives rejected* section | AC-001, AC-002: the three names, the six frontmatter fields, both inline definitions, the verbatim four-crate goal and the three unchanged grounds. A content review, which is the tier the testing brief assigns to exactly this class of AC (`_decomposition.md`, AC-011/AC-012 rows) |
| **story-grain gate (automatic)** | `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) | NF-002: fmt/clippy/tests for anything the diff could reach, plus the file-reading lints and `spec-trace` unconditionally — so a markdown-only story is still gated |
| **not run here** | `cargo xtask ci` (full), the four `wasm32` steps, `event_store_conformance!` | Stated so the absence is a decision: this project is not terminal (`.redkiln/config.yaml`:55 wires `cargo xtask ci --fast` at integration grain), the full gate belongs to `publish-0-2-0`'s AC-016, and no `EventStore` fixture is exercised by this project at all (`publication-and-positioning/_decomposition.md`:507-511) |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation, in this PR |
| --- | --- | --- |
| **`reconcile` already fails on the tree this story receives** — a `publish = false` dropped, or a member added under a new path | low / high. It would invert the story: the delta stops being paperwork and becomes a re-plan | The check is step 1, before the atom is written (EC-001/EC-002). The story halts and reports rather than editing `PUBLISHABLE`; the deployment brief already names the three artefacts a real crate-set change must move together (`publication-and-positioning/_decomposition.md`:610-613) |
| **The ingest wave lands on its own branch, out of this story's control** | medium / medium. The atom's real id does not exist until the wave runs, and the runbook pointer and the `_ledger.md` both cite it | The pointer edit and the ledger's evidence are sequenced *after* the handoff (implementation note 7). Everything the story can produce alone — the intake document, both captures — is committed first, so the story is never blocked on the wave for its evidence, only for its id |
| **Number collision inside the `release-decisions` slice** — four atoms, one wave | medium / low | This story is first in merge order and states the whole allocation in its intake document (`_storymap.md`:73-78, `:156-160`); EC-004 makes the wave's assigned id authoritative if it still happens |
| **A `RUNBOOK.md` conflict with `falsifier-ledger-repair`** | low / medium. Both stories edit `RUNBOOK.md` | Disjoint regions by construction: this story is confined to `:4448-4460`, that one owns `:622-635`, and the PR boundary says so explicitly. They are also in different milestones (`_storymap.md`:179-184) |
| **Three siblings block on this and one of them is in the same slice** — `projection-port-ship-shape` depends on it and is implemented in the same context | high / low | It is a *reading* dependency, not a code one: the sibling needs the crate set stated, which AC-001 delivers. Nothing in this story's mount point is edited by any sibling |
| **The atom reads correctly and nobody ran the check** — the story's named wrong implementation (`discover.md`:38-40) | this is the whole risk the story exists against | AC-003 (captured passing run) and AC-004 (captured seeded failure) are both ledger rows requiring cited evidence; a `_ledger.md` row cannot be flipped on prose |
| **Scope creep into published copy** — the triad, the status table, the *"Which crate do I want?"* block are one keystroke away from this decision | medium / medium | Named as an explicit PR-boundary exclusion, and re-stated in the composition section: AP-11 and the R3 budget are `landing-copy-and-status-truth`'s to hold. This story constrains that budget (three entries) and writes none of it |

## Dependencies

**Blocks on:** *nothing* — `depends_on: []` (`_storymap.md`:56). This story is first in the
`release-decisions` slice and first in the project's merge order, which is why it is a foundation:
it produces a fact three other stories read rather than a capability a user meets.

**Unlocks** (by story slug, each with what it actually consumes):

| story slug | what it reads from this one |
| --- | --- |
| `projection-port-ship-shape` | slice-mate; needs the crate set stated before it decides how the projection port ships *within* that set (`_storymap.md`, slices table) |
| `registry-surface-diff` | the set the surface diff runs over — *what it diffs* (`_storymap.md`:105) |
| `landing-copy-and-status-truth` | the three answers of the *"Which crate do I want?"* triad and the status table's shape (`_storymap.md`:105); the `≤ 3 entries` budget at `_design.md`:474 is only satisfiable because this story decided three |
| `publish-0-2-0` | *what it publishes* — the terminal act runs over exactly this set (`_storymap.md`:105) |

## Anchors (progressive disclosure)

Open these when the row's *when* says to, not before. Everything load-bearing enough to be an
obligation is already stated above; these carry the depth, the transcripts and the exact wording.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `xtask/src/package.rs` | The mount point. Module docs `:1-58` argue *why* a derived set and a hand list are both kept and name the decorative-gate failure the story is defending against; `PUBLISHABLE` `:86`; `REQUIRED_FILES` `:88-94`; `reconcile` `:163-217` including both failure messages verbatim; `scan_publishable`'s tests `:408-457` | Before running the check, and again when writing the atom's rationale — the module docs are the best available prose for "intention vs derivation" and should be echoed, not re-invented | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` | The deployment brief's crate-set decision `:574-613` is where this answer was actually settled, with the three rejection grounds in their original wording; the UX brief's IQ-1…IQ-7 `:226-310`; the testing brief's AC-001/AC-013 rows `:460-480` and merge-gate commands `:481-506` | Open `:574-613` before drafting the atom's *Alternatives rejected* section — the grounds are copied forward, not re-derived | AC-002, AC-003 |
| `RUNBOOK.md` | Phase 12 `:4448-4499` is the source being overridden; the Goal at `:4450-4451` is the exact four-crate sentence to quote, and `:4495`'s superseded `#the-46-provisional-clauses` anchor is the evidence that the section is independently stale | Open twice: when quoting the goal verbatim (AC-002), and at the very end when adding the pointer line (AC-006). Do not scroll to `:622-635` | AC-002, AC-006 |
| `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | The shape of record — the most recent atom the ingest path produced, `:1-33` carrying the frontmatter fields AC-001 enumerates, and a body demonstrating how an amendment states what it does and does not reverse | Before writing the intake document, as the template for its body and frontmatter | AC-001 |
| `.kb/decisions/README.md` | `:7-13` is the immutability rule `validate --kb` enforces; `:27-33` the frontmatter contract | Before the ingest handoff, and again if `validate --kb` reports anything | AC-001, AC-005 |
| `.kb/_intake/README.md` | The ingest inlet's own operating notes — `:3-5` and `:13-19` state that the wave clears `_intake/` and that a file left behind is a file that run did not ingest | When staging the intake document, and when checking the wave landed cleanly | AC-001, AC-005 |
| `.kb/maps/decision-map.md` | `:81-86` is *Adding a row* — ADR-number order, a new `##` section per wave, never delete a superseded row. An atom with no row here is invisible to the corpus | After the wave, when verifying the map sync rather than assuming it | AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/discover.md` | `:38-40` states the named wrong implementation in full — the atom that reads correctly while nobody ran the check — and `:16-27` the signal ledger behind every claim in the context pack | First, before anything else, if you are tempted to treat AC-003 as a formality | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` | `:474`'s `≤ 3 entries` triad budget and its rationale (*"a fourth entry means a fourth published crate"*), region R3 at `:359`, the 14-line first screen at `:452-456`, and the AP-1…AP-15 anti-patterns at `:641-676` this story must not pre-empt | When writing AC-001's "exactly three names" claim, and any time surface copy feels in reach | AC-001, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` | AC-001 `:225-228` and AC-013 `:273-276` verbatim, DR-13's numbering constraint `:207-212`, and the boundary-level Definition of done | At the start, to confirm the two traced ACs, and at the end when filling the ledger | AC-001, AC-005 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The nearest neighbour in the corpus — it decided *which crate holds the bare name*, not *which crates publish*. The distinction is what makes the edge `related` rather than `supersedes` | When writing the atom's link graph | AC-001 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` | `:56` this story's row; `:73-78` why `release-decisions` is one slice and one ingest wave; `:105` the three named consumers and what each takes | When stating the slice's number allocation, and when checking the atom is quotable by its consumers | AC-006 |
| `.redkiln/config.yaml` | `:40` the story-grain `affected_gate` that fires automatically at this story's transition; `:55` the non-terminal integration gate this project runs instead of the full one | Before claiming the gate is green, to know which command redkiln will actually have run | AC-005 |
| `.bklg/from-contract-to-published-library/_discovery/distillation/personas-and-journeys.md` | Persona 4 `:249-313` and the one-sitting constraint `:333-338` — the evaluator who never reads this atom but meets its consequence as three pages and three answers | When the "why does a paperwork story have a persona" question comes up, and when writing anything that would add a fourth entry anywhere | AC-006 |

## Clarifications resolved during spec

1. **The AC set is exactly the six the first pass decided** — AC-001…AC-006 — and none was added or
   dropped. Each of B1…B12 in *Behavior and interfaces* lands in one of them: B1/B4/B5 → AC-001,
   B2 → AC-002, B6/B8 → AC-003, B7 → AC-004, B10/B11 → AC-005, B9/B12 → AC-006, and B3 (the ingest
   path) is carried jointly by AC-001's shape check and AC-005's `validate --kb` check because it is
   a property of *how* the atom arrived, observable only through those two.
2. **"No reconciliation failure" was split into two criteria on purpose.** AC-003 proves the check
   passed; AC-004 proves the check *can* fail. One without the other is precisely the story's named
   wrong implementation (`discover.md`:38-40) or a green transcript nobody can distinguish from a
   no-op. The seeded edit is working-tree-only and its absence from the diff is itself asserted.
3. **The composition family is constraint, not authorship.** Rather than declare the whole family
   inapplicable because no surface is rendered, one composition invariant — the `≤ 3 entries` triad
   budget at `_design.md`:474 — is written into AC-001, because that budget's own rationale is this
   story's decision. Everything else in `_design.md`'s composition sections stays with
   `landing-copy-and-status-truth`, and this story's PR boundary forecloses touching it.
4. **The keyboard/focus/scroll/selection invariants are recorded as not applicable rather than
   silently omitted**, with the medium's real analogue (IQ-3, inbound anchors surviving) named and
   gated instead. That is the same *a skip is reported, never silent* discipline
   `.kb/decisions/0010-the-suite-must-prove-itself.md` binds in the testkit.
5. **The atom's ADR number is deliberately not fixed in this spec.** The intake document requests
   the lowest free number ≥ 0030 and states the slice's allocation; the wave assigns the real id, and
   EC-004 makes that assignment authoritative. Writing a number here that the wave then declined
   would leave a citation nobody could resolve — the failure mode the anchors table exists to avoid.
6. **`RUNBOOK.md` is annotated, not corrected.** The tempting alternative was to rewrite the
   phase-12 Goal to say three. It loses on IQ-2: a plan of record whose losing option has been
   quietly deleted cannot show a later reader that a decision happened at all. One added pointer
   line, no removals — and `git diff` showing added lines only is how AC-006 checks it.
