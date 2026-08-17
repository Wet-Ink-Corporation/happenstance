---
id: HS-P0025
uid: cdcf20
type: project
slug: durable-audience-closeout
title: The Durable, Reconciled Audience
parent: HS-I0007
initiative: docs-that-teach
project: durable-audience-closeout
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: true
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-17T05:42:21.488Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# The Durable, Reconciled Audience

## One-line objective

Turn this initiative's three inferred personas into an adjudicated, durable audience
under `.kb/product/` — reconciled once with the separately staged persona set rather
than authored twice — and, as terminal project, re-observe all fifteen of the
initiative's Definition-of-Done scenarios on the assembled tree from a clean checkout.

## How this advances the initiative

Two of the initiative's goals land nowhere else. *"A durable, citable discipline"* and
*"the next initiative inherits an audience rather than re-deriving one"* are both
closeout obligations by construction: `.kb/` atoms may only be authored through the
ingest path, and the project process runs that harvest at its `closeout` stage
(`.redkiln/processes/project.yaml:76-80`, `harvest_kb: true`). The durable product
layer is structurally present and functionally empty today — `.kb/product/README.md`
and `.kb/design/README.md` are layer READMEs with no `authority_tier: product` atom
beneath them — so the initiative's `## Referenced personas & journeys` section cites
its own discovery distillation and explicitly flags promotion for closeout
(`.bklg/docs-that-teach/initiative.md`, "Flagged for promotion at closeout").

The decomposition made this project the **terminal DoD owner** and the only project
created with `--terminal` (`.bklg/docs-that-teach/_decomposition.md:25`, `:306-308`).
Every sibling is held to `cargo xtask ci --fast`; this one is held to the whole gate,
which `.redkiln/config.yaml:60` wires as the terminal `e2e` grain and which
`CLAUDE.md` calls the repository's Definition of Done. Fourteen of the fifteen DoD
scenarios are *made true* by a sibling; all fifteen are **re-observed here** on the
assembled result (`.bklg/docs-that-teach/_decomposition.md:236-239`). That is a
verification obligation, not a second ownership.

It is also the last honest moment for two corrections. The promotion must record which
persona the friction log *actually* observed rather than shipping the blanket "none has
been directly observed" qualification unchanged (`:283-285`), and DoD-11's
specification cross-reference must be re-run after the sibling branch merges forward,
because the residual risk the gate accepted is that the sibling *adds* documentation
MUSTs and leaves HS-P0020's pinned clause-id set incomplete (`:89-96`).

## In scope (this project)

- **Reconciling the audience (BR-13).** Adjudicating this initiative's three personas
  (`_discovery/distillation/personas-and-journeys.md`) against HS-S0131's four staged
  ones, persona by persona and journey by journey, and stating supersession explicitly
  — whichever order the branches actually merge in
  (`.bklg/docs-that-teach/_decomposition.md:116-123`).
- **Adjudicating the evaluator.** Whether Persona 3 is a persona in its own right or an
  earlier stage of the application author's journey. The decomposition resolved that
  the *initiative* plans against three, and deferred the durable question to promotion,
  to be settled jointly with HS-S0131's set rather than twice
  (`.bklg/docs-that-teach/_decomposition.md:134-139`).
- **Promoting the audience (BR-17, AC-14, DoD-15).** Authoring persona atoms as
  `kind: concept` / `authority_tier: product` and journey atoms as `kind: playbook` /
  `authority_tier: product`, per `.kb/product/README.md:6-9`, with `source_paths` citing
  the real discovery evidence and the three evidence qualifications travelling intact.
- **Operating the single closeout ingest wave.** Staging under `.kb/_intake/`, running
  the ingest, and carrying whatever `.kb/` payload the siblings staged for closeout —
  notably the `.kb/playbooks/` atom for the page-need discipline, whose *content* is
  HS-P0021's (`.bklg/docs-that-teach/_decomposition.md:106-114`).
- **Keeping the promoted layer discoverable.** Appending — never editing — a `##`
  section to `.kb/maps/domain-map.md` (`:144-150`), which today has two domains, both
  architectural, and no functional home for documentation.
- **Merging the sibling branch forward before the pull request**, and performing the
  reconciliation and the clause-id re-check against the *merged* tree rather than this
  worktree's stale copy (`.bklg/docs-that-teach/_decomposition.md:89-103`).
- **Re-observing all fifteen DoD scenarios** on the assembled tree from a clean
  checkout, with per-scenario evidence, including watching scenario 2 fail and recover.
- **Discharging the initiative's remaining exit criteria** that are closeout-shaped:
  every charter open question answered on the record or converted into an
  `open_question` atom, and an audit that each of the ten design tensions carries a
  resolution or a recorded deferral in its owning project's `_design.md`.

## Out of scope (this project)

- **Pinning the frozen documentation MUSTs by clause id (BR-10).** HS-P0020
  `checked-documentation-surface` owns the pin; this project only re-runs the
  cross-reference over the merged tree and reports whether the pinned set is still
  complete and still discharged.
- **The pinned narrative tree, the gate step, the deliberately-broken-page
  falsification and the hidden-content demonstration (BR-01, BR-02, BR-11, DoD-1, 2,
  13).** HS-P0020.
- **Authoring the page-need discipline's content, its one-need-per-page rule, the
  safe-aside line, and the "no page has become a second specification" spot check
  (BR-04, BR-09, BR-12, DoD-8, 12, 14).** HS-P0021 `page-need-discipline`. This project
  supplies the ingest vehicle, not the text.
- **The opening encounter, the conceptual bridge, and the prior-model decision (BR-03,
  BR-07, BR-18, DoD-3, 4).** HS-P0022 `application-author-path`.
- **Reachability, the front door pointer, the evaluator's second question and the
  adapter author's error site (BR-08, BR-15, BR-16, DoD-7, 9, 10).** HS-P0023
  `reach-and-adapter-path`.
- **Running the friction-log session and dispositioning its stumbles (BR-05, BR-06,
  BR-14, DoD-5, 6).** HS-P0024 `comprehension-evidence`. This project *consumes* the
  log's finding about which persona was directly observed; it does not run the session
  and does not re-open its dispositions.
- **Inventing personas.** The failure mode this project exists to prevent. Nothing new
  is authored here that is not traceable to `_discovery/distillation/` or to HS-S0131.
- **Writing an ADR as a side effect of promotion.** If reconciliation surfaces a
  decision that warrants one, the gap is recorded and routed; an accepted decision atom
  is immutable and superseding is a deliberate act, not a closeout by-product
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Incidental bugs found while re-observing.** They route to the `support` initiative
  per `.redkiln/config.yaml:5`.

## Derived requirements

Expanded from the initiative requirements this project owns (BR-13, BR-17, AC-14,
DoD-15) plus its terminal verification obligation.

- **DR-1 — Reconciliation is a written adjudication, not an assertion.** For each
  persona and journey on both sides, the record states one of: merged (with the
  surviving atom named), superseded (with the superseded source named), or kept
  distinct (with the distinguishing goal/fear named). Silence on a pair is a defect.
- **DR-2 — Reconciliation works in either merge order.** HS-S0131 was verified at the
  decomposition gate as still `stage: plan`, `status: ready` — unrun, on an unmerged
  branch. If it has still not run, the record says so and states that this initiative's
  set stands as the authored one; if it has, the record adjudicates against its output.
  Blocking on it is forbidden (`.bklg/docs-that-teach/_decomposition.md:116-123`).
- **DR-3 — Atoms carry the shape `.kb/product/README.md` requires.** Persona atoms are
  `kind: concept`, journey atoms `kind: playbook`, both `authority_tier: product`, both
  with `source_paths` citing real discovery evidence; and each persona states goal,
  context, what they already do instead, and what they are afraid of.
- **DR-4 — The three evidence qualifications survive promotion.** Direct-observation
  status per persona; the adapter author's "no third-party adapter exists to read"
  resting on internal audit alone; and the HS-S0131 overlap. Dropping one converts a
  guess into a finding, which is precisely what `.kb/product/README.md:26-31` forbids.
- **DR-5 — The observation qualification is corrected, not copied.** The blanket "none
  has been directly observed" is replaced by naming the persona HS-P0024's session
  actually walked and marking the rest as still inferred.
- **DR-6 — The evaluator question is answered once.** The atoms record a decision, with
  its reasoning, rather than leaving the question open in two distillations.
- **DR-7 — Atoms reach `.kb/` only through the ingest path.** Staged in `.kb/_intake/`
  and ingested; hand-authoring the directory layout of the process without the process
  was reverted once already (`0269720`, cited in the initiative's non-goals).
- **DR-8 — The ingest wave leaves `.kb/_intake/` clear of its own payload.**
  A successful ingest clears the directory (`.kb/_intake/README.md:14-19`), and the
  default glob is `.kb/_intake/*.md` (`:5-7`) — which includes that README, a file that
  is not an atom and must not be ingested as one.
- **DR-9 — The promoted layer is indexed.** `.kb/maps/domain-map.md` gains an appended
  `##` section; existing sections are not edited (`:144-150`).
- **DR-10 — Validation passes.** `redkiln validate --kb` is green on the resulting tree,
  which is DoD-15's literal bar.
- **DR-11 — All fifteen DoD scenarios are re-observed with recorded evidence** on the
  assembled tree from a clean checkout, and the whole gate (`cargo xtask ci`) is green
  on the exact tree that carries them.
- **DR-12 — DoD-11 is re-run post-merge.** `cargo xtask spec-trace` passes over the
  merged tree and the pinned clause-id set is checked for *completeness* against it, not
  only for discharge. Clause ids are stable names and are never renumbered
  (`spec/SPECIFICATION.md:280`), so the risk is addition, not renumbering.
- **DR-13 — No charter open question is left silently unanswered.** Each is answered on
  the record or becomes an `open_question` atom with a bullet appended to
  `.kb/maps/open-questions-index.md` (`:27-31`).
- **DR-14 — No design tension is left silently unowned.** Each of the ten DTs is audited
  for a resolution or a recorded deferral in its owning project's `_design.md`. With
  `design.capture` deliberately absent from `.redkiln/config.yaml:75-83`, the written
  resolution is the only record there will ever be.

## Acceptance criteria

Project-grain and observable. This is the spine the story map must cover.

| ID | Criterion |
| --- | --- |
| AC-001 | A written reconciliation record exists that pairs every persona and journey in `_discovery/distillation/personas-and-journeys.md` with HS-S0131's staged set, and marks each pair merged, superseded or kept distinct with a named reason. No pair is unaddressed. |
| AC-002 | The reconciliation record states which merge order actually occurred, and reads coherently under it — including the case where HS-S0131 has still not run and this initiative's set stands as the authored one. |
| AC-003 | The evaluator question is decided in one place: the atoms state whether the evaluator is a persona in its own right or an earlier stage of the application author's journey, with the reasoning, and the initiative charter's corresponding open question is marked answered. |
| AC-004 | Persona atoms exist under `.kb/product/` with `kind: concept`, `authority_tier: product`; journey atoms with `kind: playbook`, `authority_tier: product`; each persona atom states goal, context, current alternative and fear, per `.kb/product/README.md:6-13`. |
| AC-005 | Each promoted atom's `source_paths` cites at least one real discovery artefact under `.bklg/docs-that-teach/_discovery/`, and every cited path resolves in the tree at closeout. |
| AC-006 | All three evidence qualifications appear in the promoted atoms: per-persona observation status, the adapter author's internal-audit-only basis, and the HS-S0131 overlap. Removing any one is a review-blocking defect. |
| AC-007 | The promoted atoms name the persona HS-P0024's friction-log session directly observed, and mark the remaining personas as still inferred. No atom carries an unqualified blanket "none has been directly observed". |
| AC-008 | Every `.kb/` atom this project lands was produced by an ingest run from `.kb/_intake/`; no atom under `.kb/` in this project's diff was hand-authored. |
| AC-009 | After the ingest run, `.kb/_intake/` contains no staged payload from this wave, and `.kb/_intake/README.md` is still present and was not ingested as an atom. |
| AC-010 | `.kb/maps/domain-map.md` carries a new appended `##` section covering the product layer, and `git diff` shows no modification to any pre-existing section of that file. |
| AC-011 | `redkiln validate --kb` exits zero on the tree that carries the promoted atoms. |
| AC-012 | The sibling branch has been merged forward, and both the reconciliation record and the clause-id re-check name the merged tree (by ref or sha) as what they were performed against. |
| AC-013 | `cargo xtask spec-trace` passes on the merged, assembled tree, and a written statement records whether the sibling added any documentation MUST absent from HS-P0020's pinned set — with any addition either pinned or explicitly routed. |
| AC-014 | All fifteen initiative Definition-of-Done scenarios have been re-run on the assembled tree from a clean checkout, each with recorded evidence naming who ran it and what was seen. None is marked "inherited from a sibling" without a fresh observation. |
| AC-015 | Scenario 2 is re-observed in both halves at closeout: the deliberately broken page makes the gate **fail by name**, and reverting it returns the gate to green. The failing half is recorded, not just the recovery. |
| AC-016 | `cargo xtask ci` — the terminal `e2e` grain wired at `.redkiln/config.yaml:60` — is green on the exact tree carrying every artefact above. |
| AC-017 | Every open question in the initiative charter's `## Open questions for the planning team` is either answered on the record or exists as an `open_question` atom, with a bullet appended to `.kb/maps/open-questions-index.md`. None is silently dropped. |
| AC-018 | An audit table records, for each of DT-1 … DT-10, the owning project's `_design.md` resolution or its recorded deferral. Any tension with neither is reported as a gap rather than passed over. |

## Definition of done (boundary-level)

- The reconciliation record, the promoted atoms and the DoD re-observation ledger all
  exist in the tree, and the initiative's closeout links them.
- `redkiln validate --kb` and `cargo xtask ci` are both green on that same tree, in that
  order — the gate being green is a precondition for reading the evidence, never a
  substitute for it (initiative charter, `## Definition of Done` preamble).
- The sibling branch is merged forward and the merged tree is what everything above was
  observed against.
- Nothing in `.kb/product/` traces to an assumption that is not marked as one.
- No sibling project's requirement was re-opened, re-decided or re-implemented here; each
  entry in the re-observation ledger cites the project that made it true.

## Dependencies

**Depends on:** HS-P0024 `comprehension-evidence` — directly, per the decomposition's
dependency graph (`.bklg/docs-that-teach/_decomposition.md:257-264`). Transitively on
HS-P0020, HS-P0021, HS-P0022 and HS-P0023 through it. The direct edge is load-bearing
twice over: the friction log supplies which persona was *actually observed* (DR-5,
AC-007), and a DoD re-observation run against a half-assembled surface would measure the
assembly rather than the teaching.

**Also depends on, outside the DAG:** the merge-forward of
`initiative/from-contract-to-published-library`. This is a *sequencing* obligation, not a
blocking dependency — the gate resolved explicitly that this initiative runs in parallel
and merges forward before the pull request, because blocking would put an unschedulable
cross-branch dependency on the terminal project
(`.bklg/docs-that-teach/_decomposition.md:89-96`, `:116-123`).

**Unlocks:** nothing. Terminal, and the only project created with `--terminal`
(`:306-308`).

## Risks and coupling notes

| Risk | Note |
| --- | --- |
| HS-S0131 never runs, and "reconciliation" quietly becomes "authoring" | Verified unrun at the decomposition gate. DR-2 requires the record to state the case explicitly; an unstated single-sided authoring is the charter's named failure mode wearing a reconciliation's clothes. |
| The ingest glob sweeps `.kb/_intake/README.md` | The default input is `.kb/_intake/*.md` (`.kb/_intake/README.md:5-7`) and the README is a real `.md` file that is deliberately *not* an atom (`:28-30`). AC-009 makes surviving it observable; drop it at the ingest approval gate rather than after the fact. |
| Promotion launders a guess into a finding | `.kb/product/README.md:23-24` and `:26-31` are explicit: a persona nobody researched is a stock photo with a name, and promoting an unevidenced sketch early makes every later initiative inherit the guess without the caveat. DR-4 and AC-006 are the counterweight. |
| The sibling adds documentation MUSTs after HS-P0020 pinned the set | The named residual risk at the gate (`:94-96`). Clause ids are stable (`spec/SPECIFICATION.md:280`), so the failure is an *incomplete* pin, not a stale one — which only a post-merge re-run finds. AC-013. |
| Fifteen re-observations become fifteen ticked boxes | The re-observation is a verification obligation on top of sibling ownership (`:236-239`), and AC-014 forbids "inherited from a sibling" as an evidence value. Scenario 2 in particular must be seen to **fail** (AC-015). |
| The closeout ingest wave carries a sibling's payload it does not own | HS-P0021's `.kb/playbooks/` discipline atom is staged for this wave (`:106-114`). The seam is vehicle versus content: this project runs the ingest and does not edit the discipline's text. The story map must express that as a handoff, not an authoring story. |
| A reconciliation decision that really wants an ADR | Record the gap and route it. An accepted decision atom is immutable and supersession is deliberate (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`); an ADR written as a side effect of a closeout is exactly the practice the corpus was built to refuse. |
| This project owns no design tension, so its `_design.md` looks vacuous | The design stage runs for every project (`.redkiln/processes/project.yaml:26-46`), and this one carries a `ux` brief (`:322` of the decomposition): its surface is the promoted atoms themselves — what a persona atom must state so the next initiative can frame acceptance criteria from it. Say that explicitly rather than passing in one line. |
| Documentation has no functional home in the domain map | Two domains today, both architectural (`.kb/maps/domain-map.md:130-150`). DR-9 appends; editing an existing section to make room would violate the map's own rule. |

## Context anchors

Initiative and decomposition:

- [`../initiative.md`](../initiative.md) — BR-13, BR-17, AC-14, DoD-15, the fifteen DoD
  scenarios, `## Referenced personas & journeys` and `## Exit criteria`
- [`../_decomposition.md`](../_decomposition.md) — this project's mandate (`:25`), the
  parallel-and-reconcile decision (`:116-123`), the merge-forward rule (`:89-103`), the
  re-observation obligation (`:236-239`), the terminal flag (`:306-308`)
- [`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md)
  — the three personas, their journeys, their risks, and the four open questions this
  project closes
- [`../_discovery/grounding/product-functional-alignment.md`](../_discovery/grounding/product-functional-alignment.md)
  and [`../_discovery/grounding/backlog-adjacency.md`](../_discovery/grounding/backlog-adjacency.md)

Knowledge base:

- `.kb/product/README.md` — the atom shapes, the closeout-performs-promotion rule, and
  the bar an unevidenced sketch fails
- `.kb/design/README.md` — where a `_design.md` resolution is harvested to, and what
  altitude it must keep
- `.kb/_intake/README.md` — the ingest glob, the clearing contract, and why the README
  itself is not an atom
- `.kb/maps/domain-map.md` — append a section, never edit one
- `.kb/maps/open-questions-index.md` — where a deferred question gets its bullet
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — supersede, never edit

Process and gate:

- `.redkiln/processes/project.yaml` — the `design` stage that runs for every project, and
  the `closeout` stage with `harvest_kb: true`
- `.redkiln/config.yaml:60` — `e2e: cargo xtask ci`, the terminal grain this project is
  held to; `:5` — where incidental bugs route; `:75-83` — why the perceptual review skips
- `spec/SPECIFICATION.md:280` — clause ids are stable and never renumbered
- `xtask/src/main.rs` — the single definition of the gate `cargo xtask ci` runs

## Companions

- [`_intake-brief.md`](_intake-brief.md) — this project's approved intake
- [`_decomposition.md`](_decomposition.md) — this project's briefs artifact
- [`_grounding.md`](_grounding.md) — the grounding companion to it
- The three briefs this project warrants (`../_decomposition.md:322`) are **sections
  of `_decomposition.md`**, not separate files: `## Architecture brief`, `## UX brief`
  and `## Testing brief`. There is no `briefs/` directory and no `_architecture.md`.
- [`_storymap.md`](_storymap.md) — the vertical-slice story map
- [`../initiative.md`](../initiative.md) — the parent charter
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
