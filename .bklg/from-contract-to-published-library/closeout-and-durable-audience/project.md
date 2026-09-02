---
id: HS-P0019
uid: d57bf2
type: project
slug: closeout-and-durable-audience
title: The whole gate on the assembled library, and a durable audience
parent: HS-I0006
initiative: from-contract-to-published-library
project: closeout-and-durable-audience
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: true
owner: ryan-britton
created: 2026-08-12
updated: 2026-08-12T12:57:19.560Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The whole gate on the assembled library, and a durable audience

## One-line objective

Run the whole gate once on the assembled library from a clean checkout, re-observe
the initiative's sixteen Definition-of-Done scenarios **as a set** rather than as
sixteen separately-remembered ticks, and leave the audience durable in `.kb/product/`
so the next initiative inherits it instead of re-deriving it.

## How this advances the initiative

This is the initiative's **terminal** project (`_decomposition.md` Children row
HS-P0019; DAG sink at rank 6) and the only one wired to `verify.e2e:
cargo xtask ci` — the whole gate, which `.redkiln/config.yaml:56-60` reserves for
exactly one project and whose own comment calls it "this repository's Definition of
Done". Every sibling ran `cargo xtask ci --fast`, the non-terminal bar, against a
tree containing only its own work and whatever preceded it. A genuine interaction
between two independently-green projects is therefore invisible until the whole
thing is assembled and run once, and a clean checkout additionally catches what a
warm working tree hides: uncommitted files, stale build artefacts, a path dependency
that should have been a version.

It also closes the second gap the charter named and no code project can close. The
durable product layer is structurally present and functionally empty —
`.kb/product/README.md` and `.kb/design/README.md` are template READMEs and no
`authority_tier: product` atom exists in this tree. The charter's *Referenced
personas & journeys* section says so explicitly and flags the four personas for
promotion at closeout. Promoting them here, **through the ingest path**, is what
turns a durable audience from a paragraph in a closed initiative into something
`redkiln validate --kb` will keep honest.

Owned per `_decomposition.md`'s traceability matrix: **BR-16**, **AC-15**, **DoD 13**,
**DoD 16**, the **audit** half of BR-15, and initiative exit criteria 7 and 8. It
exercises (does not own) BR-03, BR-12, AC-01, AC-02, AC-03, AC-07 and DoD 1–12 and
14–15 — exercising them is precisely the re-observation this project exists for.

## In scope (this project)

- **The whole gate on the assembled tree.** `cargo xtask ci` — fmt, clippy `-D
  warnings`, tests, the four `wasm32` steps, docs, `cargo xtask spec-trace`, the
  `--no-default-features` doc build and the `cargo package --list` licence/README
  assertion, plus `cargo hack`, `cargo deny` and the nightly `--cfg docsrs` build
  where the tool resolves (`CLAUDE.md` *Commands*; `xtask/src/main.rs`) — run green
  from a clean checkout.
- **The re-observation record** for DoD 1–12 and 14–15: one line per scenario, each
  carrying a cited artefact observed on *this* tree, drawn from the producing
  project's own `_ledger.md` (`.redkiln/config.yaml:67` makes those ledgers mandatory
  and cited) but re-run rather than re-read.
- **The honest caveat on DoD 13.** The charter asks for the gate green "on the exact
  tree that was published", and `replication-identity-and-ingest` and
  `retention-and-incomplete-logs` land *after* `publication-and-positioning` in merge
  order. The closeout tree is therefore not byte-identical to the published one. That
  delta is stated and bounded, not left implicit.
- **The BR-15 audit.** That every answer this initiative settled landed as a decision
  atom under `.kb/decisions/` from the runbook's ADR queue (`RUNBOOK.md:262-284`),
  that each names the alternatives that lost, and that every consumed open-question
  atom is **resolved, annotated and still on disk** — the standing instruction in
  `.kb/maps/open-questions-index.md:11-13` is that a withdrawn or superseded question
  stays listed rather than removed.
- **Persona and journey promotion** into `.kb/product/` through `.kb/_intake/` and
  `/redkiln:kb-ingest`, carrying the secondary-evidence qualification, per
  `.kb/product/README.md:15-24`.
- **Backlog and knowledge-base health at closeout**: `redkiln validate`,
  `redkiln validate --kb` and `redkiln doctor` clean, with the `template-drift`
  advisory set exactly the six asserted at `.github/workflows/ci.yml:179-186`.
- **The disposition of anything the re-observation finds** — routed, recorded, and
  never silently absorbed.

## Out of scope (this project)

Each exclusion names the sibling that owns it. Per `_decomposition.md` *Scope seams*:
"`closeout-and-durable-audience` does not own any code, adapter or release — every
sibling above."

- **Any store or projection adapter, and any conformance work.** `sqlite-durable-store`
  (HS-P0012), `cloudflare-durable-object-store` (HS-P0013), `postgres-and-neon-stores`
  (HS-P0014), `ladybug-projection-store` (HS-P0015), `projection-store-freeze` (HS-P0010).
- **The typed layer, the worked example and `0.2.0-alpha.1`.**
  `typed-layer-and-alpha-release` (HS-P0011).
- **Any release, registry presentation, MSRV promise, semver diff, clause-ledger audit
  or positioning claim.** `publication-and-positioning` (HS-P0016) — including BR-17
  and DT-1/4/5/6. If a peer has moved since that project ran, this project records the
  finding; it does not restate the positioning.
- **The `ProjectionStore` freeze and its verdict.** `projection-store-freeze` (HS-P0010)
  and `ladybug-projection-store` (HS-P0015) respectively.
- **The two silences.** `replication-identity-and-ingest` (HS-P0017) and
  `retention-and-incomplete-logs` (HS-P0018) each write their own decision atom; this
  project audits only that they exist and that their open-question atoms are resolved.
- **Design of anything.** `_decomposition.md` *Warranted briefs* gives this project
  **one** brief (`testing`) deliberately: "it verifies and records; it designs
  nothing. Forcing `architecture` or `ux` onto it would produce exactly the stub the
  right-sizing rule exists to prevent."
- **Fixing incidental defects.** They route to the `support` initiative per
  `.redkiln/config.yaml:5`.

## Derived requirements

Expanded from BR-03(e), BR-12(e), BR-15(audit), BR-16, AC-15, DoD 13, DoD 16 and
exit criteria 7–8.

- **DR-1 — Clean-checkout assembly.** The gate runs against a checkout with no
  untracked or ignored residue and no path dependency standing in for a published
  version. A warm working tree does not satisfy this.
- **DR-2 — The whole gate, not the fast one.** `cargo xtask ci` — the terminal grain
  at `.redkiln/config.yaml:60`, not `--fast` (`:55`). Any step that reports `skipped`
  is accounted for by a missing tool probe, not by a tool that ran and was ignored.
- **DR-3 — Re-observation as a set.** DoD 1–12 and 14–15 are each re-run or
  re-inspected on this tree and recorded together in one artefact, with the command
  and the artefact path. A scenario marked from the memory of the project that
  produced it does not count.
- **DR-4 — The DoD 13 delta is stated.** The record names the published `0.2.0` tag,
  the projects that landed after it (HS-P0017, HS-P0018), and why that delta does not
  change the published surface — the gate decision at `_decomposition.md` *Decisions
  taken at the gate* item 4 constrains retention's answer to what needs no
  published-surface change precisely so this remains true.
- **DR-5 — `!Send` survived to the end.** The four `wasm32` steps inside the gate are
  green on the assembled tree, so BR-12's "exercised end to end by everything that
  ships" is observed at closeout rather than assumed from HS-P0013's own run
  (ADR-0001; `CLAUDE.md` binding constraint 1).
- **DR-6 — Decision-atom audit.** One row per answer this initiative settled, naming
  its atom under `.kb/decisions/`, its accepted status, and the alternatives it
  records as having lost. Any ADR number the queue reserved and nobody wrote is listed
  with the reason it was not needed (`RUNBOOK.md:276-283` establishes that a reserved
  number staying empty can be correct).
- **DR-7 — Open questions resolved, never deleted.** Every open-question atom this
  initiative consumed — including `cf-40-fixture-limits-ownership.md`,
  `projection-store-batch-has-no-apply-seam.md`,
  `es-38-and-gap-read-rules-are-unowned.md`,
  `global-versus-per-boundary-visibility-invariant.md`,
  `ps-1-states-no-progress-obligation.md` and
  `human-readable-payload-encoding-on-a-constrained-peer.md` — is still on disk with a
  status reflecting its resolution and an annotated bullet in
  `.kb/maps/open-questions-index.md`.
- **DR-8 — Promotion runs through the ingest path.** Persona and journey atoms are
  staged in `.kb/_intake/` and authored by `/redkiln:kb-ingest`, never hand-written.
  This is the constraint the reverted commit `0269720` exists to remember, and
  `.kb/product/README.md` is explicit that closeout performs the promotion.
- **DR-9 — The qualification travels in frontmatter.** "All four personas rest on
  secondary evidence — download counts, public posts, issue threads, an adjacent
  project's postmortem — and none was directly observed" appears where a reader who
  reads only the frontmatter cannot miss it: in `summary`, with `source_paths` naming
  the discovery artefacts it came from.
- **DR-10 — The evaluator question is decided first.** Whether the evaluator is its
  own persona or an earlier stage of the application author's journey is settled in
  this project's `testing` brief *before* any atom is authored — carried unresolved
  from the charter and from `_decomposition.md` *Carried into the briefs*.
- **DR-11 — Tooling health.** `redkiln validate`, `redkiln validate --kb` and
  `redkiln doctor` clean, with zero `problems`, zero `process-drift`, and a
  `template-drift` file set exactly the six in `.github/workflows/ci.yml:179-186`.
  `redkiln adopt --templates` is never run.
- **DR-12 — Findings are dispositioned, not absorbed.** Anything the re-observation
  surfaces gets a recorded disposition: incidental defect → `support`
  (`.redkiln/config.yaml:5`); a genuine cross-project interaction → a new item against
  the owning sibling, not a fix here; anything touching a `[FROZEN]` clause → a new
  decision atom and a re-plan, per the charter's *Out of scope*.
- **DR-13 — The initiative can close.** Every child HS-P0010…HS-P0018 is closed out,
  the initiative's own closeout links the promoted product atoms, and what was learned
  is harvested rather than left in the backlog (exit criteria 7–8).

## Acceptance criteria

Project-grain and observable. Each is the spine a story must cover.

- **AC-001 — The whole gate is green on the assembled tree from a clean checkout.**
  `cargo xtask ci` exits zero on a checkout with no untracked or ignored residue; the
  run records the commit SHA it ran against, and every step that printed `skipped` is
  listed with the absent tool that caused it.
- **AC-002 — The four `wasm32` steps are green in that same run.** The gate's
  `happenstance-core` build, the conformance-harness check and the
  `happenstance-cloudflare` and `happenstance-neon` builds all pass on the assembled
  tree, evidencing BR-12 at closeout rather than at HS-P0013's own merge.
- **AC-003 — DoD 1–12 and 14–15 are re-observed as a set.** One record lists all
  fourteen, each with the command run on this tree and the artefact path it produced.
  No entry cites only the producing project's ledger; a ledger may be the pointer,
  never the evidence.
- **AC-004 — The DoD 13 delta is stated, not glossed.** The record names the published
  `0.2.0` tag, the commits between it and the closeout tree, the projects responsible,
  and the reason the published surface is unchanged across that delta.
- **AC-005 — Every answer has a decision atom, and the atom names what lost.** An
  audit table maps each settled question to an atom under `.kb/decisions/` whose
  status is accepted and whose body records the rejected alternatives; every reserved
  ADR number that was not written carries a stated reason.
- **AC-006 — No open-question atom was deleted.** A diff of `.kb/open-questions/`
  between the initiative's start commit and the closeout tree shows zero deletions;
  every atom whose question this initiative answered has a resolution-bearing status
  and an annotated bullet in `.kb/maps/open-questions-index.md`.
- **AC-007 — The audience exists as durable atoms.** `.kb/product/` contains a
  `concept` / `authority_tier: product` atom per promoted persona and a `playbook` /
  `authority_tier: product` atom per journey, and `redkiln validate --kb` passes on
  them.
- **AC-008 — The atoms were produced by the ingest path.** The commit range shows the
  atoms arriving from `.kb/_intake/` via `/redkiln:kb-ingest`, with the intake
  staging consumed and cleared; no product atom appears in a hand-authored commit.
- **AC-009 — The secondary-evidence qualification is unmissable.** Each promoted atom
  states in its `summary` that its evidence is secondary and no persona was directly
  observed, and lists the discovery artefacts it came from in `source_paths`.
- **AC-010 — The evaluator question is decided before authoring.** This project's
  `testing` brief records whether the evaluator is its own persona or a stage of the
  application author's journey, with the reasoning, and the set of promoted atoms
  matches that decision.
- **AC-011 — Backlog and knowledge base are clean, with exactly six drift
  advisories.** `redkiln validate`, `redkiln validate --kb` pass and `doctor --json`
  reports zero problems, zero `process-drift`, and a `template-drift` file list equal
  to the six at `.github/workflows/ci.yml:179-186`.
- **AC-012 — A seventh advisory, or a missing sixth, is dispositioned rather than
  accepted.** If the drift set differs, the difference is recorded with a decision —
  revert, or adopt deliberately with a stated reason — and `redkiln adopt --templates`
  is not run in either case.
- **AC-013 — Findings are routed, not absorbed.** Every defect surfaced by the
  re-observation appears in the record with its destination: `support`, a new item
  against the owning sibling, or a decision atom plus a re-plan. Zero are fixed inside
  this project.
- **AC-014 — The initiative can be closed.** All nine sibling projects are closed out
  and the initiative's closeout links the promoted product atoms, evidenced by
  `redkiln status` showing no open child of HS-I0006.

## Definition of done (boundary-level)

Observed at this project's boundary, not inside its stories.

1. `cargo xtask ci` is green on a clean checkout of the closeout tree, and the run
   is recorded with its SHA (AC-001, AC-002 → DoD 13).
2. The re-observation record exists, covers DoD 1–12 and 14–15 with cited artefacts,
   and states the published-tree delta (AC-003, AC-004).
3. The audit table exists: every settled answer has an accepted decision atom naming
   its rejected alternatives, and `.kb/open-questions/` shows zero deletions
   (AC-005, AC-006 → BR-15).
4. Persona and journey atoms exist under `.kb/product/`, arrived via the ingest path,
   carry the secondary-evidence qualification in frontmatter, and pass
   `redkiln validate --kb` (AC-007, AC-008, AC-009 → DoD 16, AC-15, BR-16).
5. `redkiln validate`, `redkiln validate --kb` and `redkiln doctor` are clean with
   exactly the six expected `template-drift` advisories, and any deviation is
   dispositioned (AC-011, AC-012 → exit criterion 7).
6. Every finding is routed with a recorded destination and none was fixed here
   (AC-013).
7. All nine sibling projects are closed and the initiative's closeout links the
   promoted atoms (AC-014 → exit criterion 8).

## Dependencies

From `_decomposition.md` *Dependency DAG* and *Merge order*.

**Depends on**

- `retention-and-incomplete-logs` (HS-P0018) — the direct `blocked_by` edge; position
  9 of 10 in merge order.
- Transitively, the whole serial trunk: `projection-store-freeze` (HS-P0010) →
  `typed-layer-and-alpha-release` (HS-P0011) → `sqlite-durable-store` (HS-P0012) →
  `publication-and-positioning` (HS-P0016) → `replication-identity-and-ingest`
  (HS-P0017) → HS-P0018; plus `cloudflare-durable-object-store` (HS-P0013),
  `postgres-and-neon-stores` (HS-P0014) and `ladybug-projection-store` (HS-P0015),
  all of which block HS-P0016 and so precede this project.

In practice this project depends on **all nine siblings**: it is the DAG's single
sink at rank 6, and it cannot re-observe a scenario whose producing project has not
merged.

**Unlocks**

- Nothing downstream inside this initiative — `blocks` is empty by construction.
- Outside it: the initiative's own closeout stage, and the next initiative's ability
  to cite an adjudicated audience instead of re-deriving one from the same secondary
  evidence.

## Risks and coupling notes

| Risk / coupling | Note |
| --- | --- |
| **The gate is long and fires on both sides of the stage** | `.redkiln/config.yaml:50-55` explains that a command gate runs on entry and exit of its stage. The terminal `e2e` grain is the full `cargo xtask ci` including two feature powersets, `cargo deny` and a nightly rustdoc build. Budget the wall-clock; do not be tempted to substitute `--fast`, which is the bar every *other* project already met. |
| **DoD 13's phrase cannot be satisfied literally** | "the exact tree that was published" is false by construction once HS-P0017 and HS-P0018 land after HS-P0016. Mitigated by DR-4 / AC-004 stating the delta, and upstream by the gate decision constraining retention to an answer that needs no published-surface change (`_decomposition.md` *Decisions taken at the gate*, item 4). If that constraint was broken upstream, this project reports it — it does not absorb a `0.3.0` the exit criteria do not contemplate. |
| **Pressure to fix what re-observation finds** | The whole value of this project is that it is the first tree where a cross-project interaction is visible. That makes it the project most tempted to absorb a fix. AC-013 makes routing the deliverable. |
| **Hand-authoring the product atoms** | The fastest route to `.kb/product/` is to write the files. That is exactly what was reverted at `0269720` for "producing the directory layout of the process without the process". DR-8 and AC-008 make the *provenance* observable, not just the result. |
| **The kb-ingest glob and a second wave** | `/redkiln:kb-ingest` consumes `.kb/_intake/` wholesale; a README left in the staging directory is picked up with the content, and a second wave needs a distinct id so it does not overwrite the first's audit trail. Watch both at the approval gate. |
| **A seventh `template-drift` advisory** | Six are deliberate and asserted by name in CI. A seventh means a template changed without a decision; a missing one means a customisation was reverted — which is what `adopt --templates` does. Never run it (`CLAUDE.md`; `.github/workflows/ci.yml:160-187`). |
| **Evidence quality is inherited** | This project can only re-observe cheaply if each sibling's stories carry real cited evidence. `.redkiln/config.yaml:67` (`require_ledger: true`) and `:73` (`require_commit_provenance: true`) are what make that true; a thin ledger upstream becomes re-derivation work here. |
| **Positioning dates** | The charter's assumptions say anything positioned against a named live peer needs re-checking at closeout. That check is cheap; acting on it is HS-P0016's, not this project's. Record the finding and route it. |
| **The `!Send` flavour quietly dropped** | Low likelihood, high impact, and the closeout gate run is the last place it can be caught before the initiative is called shipped. AC-002 makes it explicit rather than folded into "the gate was green". |
| **The evaluator decision has downstream reach** | It changes what is promoted (DR-10) and whether DT-1's "two entry points, one per audience" option was even coherent. It is decided in the `testing` brief, but its consequence for HS-P0016's already-recorded DT-1 resolution is a finding to route, not a re-litigation. |

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — the charter: BR-15/BR-16, AC-15, DoD 13/16, exit criteria 7–8
- [`../_decomposition.md`](../_decomposition.md) — the traceability matrix, the DAG, the four gate decisions, the warranted-brief right-sizing
- [`_intake-brief.md`](_intake-brief.md) — the approved problem, constraints, open questions and proof artefact
- [`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md) — the four personas and their journeys, the source for promotion
- [`../_discovery/distillation/opportunities.md`](../_discovery/distillation/opportunities.md) — the archetypes behind them

Knowledge base:

- `.kb/product/README.md` — what a persona and a journey are, and that closeout performs the promotion (lines 15-24)
- `.kb/design/README.md` — the sibling layer, harvested from `_design.md` at closeout
- `.kb/maps/open-questions-index.md` — the nineteen open questions, and the standing rule that a resolved one stays listed and annotated rather than removed (lines 11-13)
- `.kb/maps/decision-map.md`, `.kb/maps/domain-map.md`
- `.kb/decisions/README.md` — accepted atoms are immutable; supersede, never edit
- `.kb/open-questions/cf-40-fixture-limits-ownership.md`, `.kb/open-questions/projection-store-batch-has-no-apply-seam.md`, `.kb/open-questions/es-38-and-gap-read-rules-are-unowned.md`, `.kb/open-questions/global-versus-per-boundary-visibility-invariant.md`, `.kb/open-questions/ps-1-states-no-progress-obligation.md`, `.kb/open-questions/human-readable-payload-encoding-on-a-constrained-peer.md` — atoms this initiative consumes and must resolve rather than delete

Process, gate and specification:

- `.redkiln/config.yaml:56-60` — `integration_scoped` vs `e2e`; the terminal grain this project alone carries
- `.redkiln/config.yaml:62-73` — `require_ledger` and `require_commit_provenance`, the upstream evidence this project consumes
- `.redkiln/config.yaml:5` — `support_initiative`, where findings route
- `.github/workflows/ci.yml:142-187` — the `backlog` job: `validate`, `validate --kb`, and the exact six-file `template-drift` assertion
- `xtask/src/main.rs` — the gate, defined once; `cargo xtask ci` is exactly what CI runs
- `spec/SPECIFICATION.md:219-222` — 200 clause IDs, 198 normative (139 frozen, 49 provisional, 10 deferred, 2 non-normative); the figure the ledger audit reconciles against
- `RUNBOOK.md:262-284` — the ADR queue this project audits for completeness
- `RUNBOOK.md:244-258` — the ordering constraints the DAG carries
- `CLAUDE.md` — binding constraints, the `adopt --templates` prohibition, and the six deliberate template customisations

## Companions

Board-invisible drill-down for this card:

- [`_intake-brief.md`](_intake-brief.md) — the approved intake for this project
- [`_testing.md`](_testing.md) — the one warranted brief, authored by `plan-briefs`; it also carries the evaluator-persona decision (DR-10)
- [`_storymap.md`](_storymap.md) — the vertical-slice story map over AC-001…AC-014, authored by `plan-briefs`
- [`../_decomposition.md`](../_decomposition.md) — the initiative's decomposition of record
- [`../initiative.md`](../initiative.md) — the charter
- [`../_plan.md`](../_plan.md) — the initiative-wide plan rollup
