---
id: HS-P0021
uid: be9675
type: project
slug: page-need-discipline
title: Page-Need Discipline
parent: HS-I0007
initiative: docs-that-teach
project: page-need-discipline
status: in-review
process: project
stage: review
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-19T01:03:37.462Z
links:
  pr: null
  commits: []
  kb: []
gate_open: true
schema: 1
process_rev: 7289a0c4
---
# Page-Need Discipline

## One-line objective

Make *which need does this page answer* a declared, singular, machine-readable and
reviewer-checkable property of every narrative page, and write the discipline that
says so down in a tree that binds while the pages governed by it are still being
written.

## How this advances the initiative

The initiative's diagnosis is that this repository can tell a document is *correct*
and cannot tell it *teaches* — the constitution supplies its own green counter-example
on purpose (`standards/rust/70-rustdoc-obligations.md`, RS-70-5). Its sibling
HS-P0020 closes the half a compiler can close: prose that lies about the API stops
building. This project closes the half no compiler reaches. A page that compiles
perfectly and quietly answers three questions is the "open-ended sink" failure named
independently by two traditions in
[`_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md)
(Anti-patterns, first entry), and no Rust-ecosystem peer makes the answered-need a
labelled, checkable property at all — this is a stricter bar than convention, not an
imported one (`.bklg/docs-that-teach/_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md`).

It is second in merge order for a structural reason recorded at the decomposition
gate: *a page authored before the rule exists is a page the rule is then retro-fitted
to, which is how the rule becomes decorative*
([`_decomposition.md`](../_decomposition.md), "Why the load-bearing edges exist").
HS-P0022 and HS-P0023 both author pages; both sit behind this project so that the
discipline binds their first draft rather than their last.

It also carries the initiative's answer to *where a documentation discipline lives*
(BR-12). The gate decision was taken at decomposition: **a new tree pinned by path,
sibling to `standards/rust/` and inside its precedence chain without extending it**,
with a `.kb/playbooks/` atom staged through `/redkiln:kb-ingest` at closeout. That is
the only option in which the rule binds *now* — `.kb/` atoms may only be authored
through the ingest path, and ingest runs at closeout, four projects too late
(`.kb/playbooks/README.md`; the hand-authoring attempt reverted at `0269720`).
`docs/README.md:25-29` and `xtask/src/lint_constitution.rs` are the precedent: a tree
the gate reads is pinned by path, and moving it means editing `xtask/src/` in the
same change.

## In scope (this project)

- **The discipline itself**, authored into its decided home: a router plus rule atoms,
  shaped like `standards/rust/README.md`'s band table so the next author can find one
  rule without loading the corpus.
- **The enumerated set of answered-needs** a page may declare, and whether that set is
  a named external taxonomy adopted literally, only its one-need-per-page rule, or
  adoption with a stated local extension (**DT-2**).
- **Findability and navigation's status inside that set** — a first-class need with its
  own pages, or an implicit byproduct of the chosen structure (**DT-3**).
- **The fold line**: a stated rule for what may never sit behind a fold, tab or
  collapsed panel, versus what may (**DT-8**, and the rule half of BR-11).
- **The declaration mechanism** — the form in which a page states its one need, chosen
  so that both the rendering surface HS-P0020 selects and a lint can read it.
- **The lint that reads it**: a gate step in `xtask/src/`, modelled on
  `xtask/src/lint_constitution.rs`, that fails by name and location when a page
  declares no need, two needs, or a need outside the enumerated set — and that has
  been *seen to fail* before it is trusted.
- **The reviewer procedure** for the judgement a lint cannot make: whether a page that
  declares one need is actually answering only that one. Performable by someone who did
  not write the page (DoD-8).
- **The authoring rule for normative claims** — a page cites a `spec/SPECIFICATION.md`
  clause id and never restates it — and the spot-check procedure that walks the set for
  paraphrase (the rule half of BR-09; AC-12; DoD-12).
- **Staging the playbook atom** for closeout ingest, with `authority_tier: guideline`,
  written into the intake path rather than into `.kb/`.

## Out of scope (this project)

Each excluded thing names the sibling that owns it.

- **The narrative tree itself, its build, its render, and its gate wiring** — including
  which hosting shape wins and which tree is pinned by path for *content*. HS-P0020
  `checked-documentation-surface`. This project's tree is the *rules*; that project's
  is the *pages*.
- **Compiling code fences against the real crates, and the deliberately-broken-page
  falsification of that check** (BR-01, BR-02, AC-03, DoD-1, DoD-2). HS-P0020.
- **The mechanical half of BR-09 and AC-12** — that a cited clause id actually resolves.
  HS-P0020 owns the citation-resolution check; this project owns the rule that a page
  cites rather than restates, and the reviewer spot check. The seam is stated in both
  directions and no responsibility is owned twice
  ([`_decomposition.md`](../_decomposition.md), "Traceability matrix").
- **Whether adapter-scoped content uses hidden panels at all, and the demonstration
  that a hidden branch is inside the checked surface** (DT-7, BR-11's demonstration
  half, DoD-13). HS-P0020. This project states *what may never be folded*; that project
  proves *whether a fold is checked*.
- **Pinning the frozen documentation MUSTs by clause id** (BR-10, DoD-11). HS-P0020.
- **Authoring any teaching content**: the opening encounter, the conceptual bridge,
  which prior mental model is argued against, diagrams, the wrong-model contrast
  (DT-1, DT-4, DT-5, DT-6; BR-03, BR-07, BR-18). HS-P0022 `application-author-path`.
- **Pointer policy and reachability**: the front door into the narrative material, the
  evaluator's second question, and the adapter author's error meeting its explanation
  at `crates/happenstance-core/src/store.rs` (DT-10; BR-08, BR-15, BR-16). HS-P0023
  `reach-and-adapter-path`.
- **Recruiting and running the comprehension session, and dispositioning its stumbles**
  (BR-05, BR-06, BR-14, DT-9). HS-P0024 `comprehension-evidence`.
- **Promoting personas and journeys into `.kb/product/`, and re-observing all fifteen
  Definition-of-Done scenarios on the assembled tree** (BR-13, BR-17, AC-14, DoD-15).
  HS-P0025 `durable-audience-closeout`. This project's playbook atom is *staged* here
  and *ingested* there.
- **Extending the five-tier precedence chain** in `standards/rust/README.md:23-29`. An
  initiative-level non-goal. The discipline sits inside the chain; it does not add a
  tier to it.
- **The published surface's claims** — README landing copy, status vocabulary, registry
  metadata. `publication-and-positioning` (HS-P0016) on the unmerged
  `initiative/from-contract-to-published-library` branch. Not reachable from this
  worktree and not reopened here.

## Derived requirements

Expanded from the initiative requirements this project owns
([`_decomposition.md`](../_decomposition.md), "Traceability matrix"): **BR-04** whole,
**BR-09** and **BR-11** and **BR-12** in the halves named above, plus AC-07, AC-12,
AC-13 and DoD-8, DoD-12, DoD-14.

- **DR-01 (BR-12).** The discipline lands in a decided home — a new tree, sibling to
  `standards/rust/`, pinned by path in `xtask/src/` — and `_design.md` records the
  rejected homes (new constitution atoms; a `.kb/` corpus authored now; a subtree of
  `docs/`; nothing at all) with the cost of each. A tree the gate reads by convention
  rather than by path can be moved without anything noticing (`docs/README.md:25-29`).
- **DR-02 (BR-12).** The discipline's router states its rank inside the existing
  precedence chain — `SPECIFICATION clause > ADR > constitution atom > CLAUDE.md /
  CONTRIBUTING.md summary > references/evaluation/*`
  (`standards/rust/README.md:23-29`) — without editing that chain or adding a tier.
- **DR-03 (BR-04, DT-2).** The set of needs a page may declare is enumerated and closed,
  and the choice between adopting an external taxonomy literally, keeping only its
  one-need-per-page rule, or adopting it with a stated local extension is resolved with
  its rejected options named. The evidence cuts against literal adoption for dense
  conceptual models and against forcing content into a fixed number of buckets
  ([`_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md),
  tension 3 and the eighth anti-pattern) — but the resolution is the design stage's,
  not this charter's.
- **DR-04 (BR-04, DT-3).** Whether findability and routing is a need in its own right is
  decided, because the taxonomy under consideration is documented to be silent on it,
  and an unslotted landing page is then flagged as answering a second need by the very
  rule this project is writing (same source, tension 7).
- **DR-05 (BR-04).** Each page declares exactly one need in a form that is *both*
  human-visible on the rendered page and readable by a lint without parsing prose. The
  form is constrained by HS-P0020's hosting choice and must be agreed with it before
  either side implements — see Risks.
- **DR-06 (BR-04, DoD-8).** A lint in `xtask/src/` rejects a page with zero, two, or an
  unenumerated declared need, naming the file and the line, and joins the gate as an
  ordinary step. It must reject a page that plausibly could ship: a rule no page can
  fail is decorative, so the wrong page is written into the lint's own tests, per
  `CLAUDE.md` and the precedent set by `xtask/src/lint_constitution.rs`'s explicit
  "What this does not verify" section.
- **DR-07 (BR-04, DoD-8).** The judgement the lint cannot make — *is this page actually
  answering only its declared need* — is a written procedure with the same steps for a
  non-author as for the author, so DoD-8's walk does not depend on the author's memory.
- **DR-08 (BR-11 rule half, DT-8).** The discipline states, as a rule rather than as
  reviewer judgement, where the line falls between a safe aside and a load-bearing
  constraint that may never be folded. "Use good judgment" is the non-answer that let a
  code-layer invariant drift here once already (same source, tension 2); the checkable
  form the evidence offers is *if the collapsed section were deleted, would the page
  still teach the constraint correctly?*
- **DR-09 (BR-09, AC-12, DoD-12).** The discipline requires a normative claim to be a
  citation of a `spec/SPECIFICATION.md` clause id and forbids restating clause content,
  mirroring the rule the constitution already holds itself to (`standards/rust/README.md`,
  "Clause or atom?"). Clause ids are stable names and are never renumbered
  (`spec/SPECIFICATION.md:280`), so a citation survives the sibling branch's 521-line
  divergence. A spot-check procedure walks the set for paraphrase.
- **DR-10 (AC-13, DoD-14).** The discipline is *cited*, not merely present: at least one
  page governed by it names it as the reason the page is shaped as it is, and the
  discipline is reachable from the material it governs.
- **DR-11 (AC-13, closeout handoff).** A `playbook` atom carrying `authority_tier:
  guideline` is staged for `/redkiln:kb-ingest` rather than hand-authored into `.kb/`.
  It records the method, the rejected alternatives and the conditions under which the
  discipline stops holding — the three things `.kb/playbooks/README.md` requires — and
  it does not carry the binding *must*, which stays in the gate-read tree, because a
  commitment filed as a playbook is stripped of the immutability that makes it
  enforceable (`.kb/playbooks/README.md`, "What does not belong here").

## Acceptance criteria

Project-grain and testable. These are the spine the story map must cover.

- **AC-001 — The discipline has a decided home and is on disk.** A new tree exists,
  sibling to `standards/rust/`, with a router that lets a reader load one rule rather
  than the corpus. `_design.md` names the rejected homes and the cost of each.
- **AC-002 — It sits inside the precedence chain without extending it.** The router
  states its rank relative to `standards/rust/README.md:23-29`, and a diff over the
  branch shows that file's precedence block unedited.
- **AC-003 — DT-2 is resolved and recorded.** `_design.md` states whether an external
  taxonomy is adopted literally, only its one-need-per-page rule is kept, or it is
  adopted with a stated local extension — with the options not chosen named and the
  reason each lost. The perceptual review is a skip (`design.capture` is absent from
  `.redkiln/config.yaml`), so this written record is the only record.
- **AC-004 — DT-3 is resolved and the need set is closed.** The enumerated set of
  answered-needs is written down, and findability/navigation's status inside or outside
  it is a stated decision rather than a gap discovered when the first index page is
  reviewed.
- **AC-005 — DT-8 is resolved as a stated rule.** The discipline says which classes of
  content may never sit behind a fold, tab or collapsed panel, in a form a reviewer
  applies to a page without consulting the author.
- **AC-006 — Every governed page declares exactly one need.** Walking the narrative tree
  as it stands at this project's merge, every page carries a declaration drawn from the
  enumerated set, in a form both the rendered surface and the lint read.
- **AC-007 — The declaration check has been seen to fail.** A page is edited to declare
  two needs (and, separately, an unenumerated one); the gate is run and **fails**,
  naming the file and the line; the edit is reverted and the gate returns to green. Both
  halves are observed; the failing half is the one that matters.
- **AC-008 — The lint rejects a wrong page that could plausibly ship.** A named wrong
  implementation exists in the lint's own tests, so the rule is not decorative.
- **AC-009 — The reviewer procedure is non-author-performable.** Someone who did not
  write a page can run the written procedure over it and reach a verdict, and a walk of
  the full set under that procedure finds no page carrying two needs (DoD-8).
- **AC-010 — The citation rule exists and the spot check runs.** The discipline states
  that a page cites a clause id and never restates the clause, and a spot check over the
  set confirms no page has become a second specification (DoD-12).
- **AC-011 — The discipline is cited by what it governs.** At least one page names the
  discipline as the reason it is shaped as it is, and the discipline is reachable from
  the material it governs (DoD-14).
- **AC-012 — The playbook atom is staged, not hand-authored.** A `playbook` atom with
  `authority_tier: guideline` and valid KB frontmatter sits in the ingest path for
  HS-P0025 to promote; nothing has been written directly into `.kb/`.

## Definition of done (boundary-level)

This project is **non-terminal**, so its integration bar is the project-scoped gate,
not the whole-initiative end-to-end it could not pass:

- `cargo xtask ci --fast` green on the project boundary (`.redkiln/config.yaml`,
  `verify.integration_scoped`), and `cargo xtask affected --base main` green at the
  story grain (`verify.affected_gate`).
- `cargo xtask lints && cargo xtask spec-trace` green (`verify.reachability_static`) —
  load-bearing here, because this project's deliverable is largely files the compiler
  never reads.
- Every AC-### above discharged, with a `_ledger.md` carrying real cited evidence per
  criterion (`verify.require_ledger: true`) and a recorded work commit
  (`verify.require_commit_provenance: true`).
- **AC-007's failure observed and recorded in the ledger**, not merely asserted. A gate
  that has only ever been green is decorative; this repository has paid for that lesson
  (`RUNBOOK.md:920-924`).
- `redkiln doctor` reports **exactly six** `template-drift` advisories — no more, no
  fewer. Adding a gate step must not touch `.redkiln/templates/`, and
  `redkiln adopt --templates` is never run.
- `redkiln validate --kb` green: nothing has been hand-authored into `.kb/`, and the
  staged atom's frontmatter is valid against the ingest path's expectations.
- All three warranted briefs authored and reviewed — `architecture`, `ux`, `testing`
  ([`_decomposition.md`](../_decomposition.md), "Warranted briefs per project"). No
  `deployment` brief: this project ships no rendered surface.
- DT-2, DT-3 and DT-8 each resolved in this project's `_design.md`, or explicitly and
  reasonably deferred with the deferral recorded. An unowned tension is how a documented
  failure mode ships.

## Dependencies

From the decomposition DAG ([`_decomposition.md`](../_decomposition.md), "Sequencing").
The `blocked_by` / `blocks` frontmatter is the CLI's to write; this section is the
prose of record.

**Depends on**

- **HS-P0020 `checked-documentation-surface`** — the substrate. It decides the hosting
  shape and pins the narrative tree by path; until that resolves there are no pages to
  declare a need, and the *form* of the declaration (DR-05) is constrained by the
  surface it must render on. It also owns the gate-step plumbing this project's lint
  joins.

**Unlocks**

- **HS-P0022 `application-author-path`** — its pages are authored under this discipline
  from their first draft.
- **HS-P0023 `reach-and-adapter-path`** — the same, and DT-3's resolution directly
  shapes whether its front-door and second-question routing content is a first-class
  page or a byproduct of DT-10's pointer policy.

Transitively, HS-P0024 `comprehension-evidence` and HS-P0025 `durable-audience-closeout`
both sit downstream. HS-P0025 ingests the staged playbook atom and re-observes DoD-8,
DoD-12 and DoD-14 on the assembled tree.

**Available parallelism.** This project's design stage runs alongside HS-P0020's
implementation; only DR-05 and the lint's implementation genuinely wait on HS-P0020's
hosting decision.

## Risks and coupling notes

| Risk | Likelihood / Impact | Note or mitigation |
| --- | --- | --- |
| The declaration form is chosen before HS-P0020's hosting shape and turns out to be unrepresentable on it — mdBook, for instance, has no native per-page front matter | High / Medium | DR-05 is agreed jointly with HS-P0020 before either side implements; the design stage records the form *and* the hosting assumption it rests on, so a change in the latter visibly invalidates the former |
| The lint is wired and can never fail, and nobody notices | Medium / High | AC-007 requires observing the failure and the recovery; AC-008 requires a named wrong page in the lint's own tests. This is `CLAUDE.md`'s decorative-rule corollary applied to prose, and `xtask/src/lint_constitution.rs` is the working precedent |
| The lint's limits are read as guarantees — it checks that a need is *declared*, never that the page *answers* it | Medium / High | The lint documents what it does not verify, in the shape `xtask/src/lint_constitution.rs:10-28` already uses; DR-07's reviewer procedure is the instrument for the part a byte count cannot reach |
| The discipline becomes a second specification, or grows `MUST`s that belong in a decision atom | Medium / High | DR-09 forbids restating clause content; the binding rules stay in the gate-read tree and the playbook atom carries method, not commitment (`.kb/playbooks/README.md`) |
| DT-2 resolves late and HS-P0022 authors against a rule that then changes | Medium / High | DT-2 and DT-3 are design-stage items that run in parallel with HS-P0020's implementation, ahead of any content authoring; this is why the project is second in merge order |
| An atom is hand-authored into `.kb/` because closeout feels far away | Medium / High | AC-012 makes staging the deliverable and ingest someone else's; the first attempt at hand-authoring was reverted at `0269720` |
| The gate-step addition perturbs the six-advisory template-drift assertion, or someone runs `redkiln adopt --templates` on `upgrade`'s advice | Low / High | Named in the Definition of done; the `backlog` CI job asserts the set is exactly those six, and a seventh or a missing one both fail |
| A new subject area edits `.kb/maps/domain-map.md` in place rather than appending | Low / Medium | Documentation has no functional home in the map today; a new subject area is an appended section, never an edit (`.kb/maps/domain-map.md`) |
| The discipline is measured by how many rules it has | Medium / Medium | The initiative's non-goal on volume applies to the rules as much as to the pages; the router's job is to let a reader load one to three rules, as `standards/rust/README.md` does for twenty-seven atoms |

**Coupling worth stating plainly.** Three initiative requirements are split between this
project and HS-P0020 by *responsibility, not shared ownership* — BR-09, BR-11 and BR-12.
In each case this project owns the rule and HS-P0020 owns the machine. The seam is
recorded in both projects' non-goals, and no responsibility is owned twice
([`_decomposition.md`](../_decomposition.md), "Traceability matrix").

## Context anchors

Backlog:

- [`../initiative.md`](../initiative.md) — BR-04, BR-09, BR-11, BR-12; AC-07, AC-12,
  AC-13; DoD-8, DoD-12, DoD-14; DT-2, DT-3, DT-8
- [`../_decomposition.md`](../_decomposition.md) — ownership, the DAG, and the BR-12
  gate decision on where the discipline lives
- [`../_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md)
  — tensions 2, 3 and 7, and the anti-pattern list this discipline's rules are drawn from
- `.bklg/docs-that-teach/_discovery/research/01-rust-narrative-docs-prior-art-and-page-need-taxonomy-how-tok.md`
  — that no Rust peer makes the answered-need a checkable property
- `.bklg/docs-that-teach/_discovery/distillation/opportunities.md` — Opportunity 3

Standards and specification:

- `standards/rust/README.md:23-29` — the five-tier precedence chain this discipline sits
  inside and does not extend; also the router shape and the "Clause or atom?" test
- `standards/rust/70-rustdoc-obligations.md` — the only constitution atom on
  documentation, and the source of the green-but-teaching-nothing counter-example
- `spec/SPECIFICATION.md` — the normative voice a page cites and never restates; clause
  ids are stable and never renumbered (`:280`)

Code and gate:

- `xtask/src/lint_constitution.rs` — the working precedent for a gate step that reads a
  tree by path, and for documenting a check's limits so they are not read as guarantees
  (`:10-28`)
- `docs/README.md:25-29` — why a gate-read tree is pinned by path rather than by
  convention, and what moving one costs
- `.redkiln/config.yaml` — `verify.integration_scoped` (`cargo xtask ci --fast`) for a
  non-terminal project, `verify.reachability_static`, `require_ledger`,
  `require_commit_provenance`, and the deliberate absence of `design.capture`
- `RUNBOOK.md:920-924` — the decorative-gate-step precedent AC-007 exists not to repeat

Knowledge base:

- `.kb/playbooks/README.md` — what a playbook must carry, and why a commitment filed as
  one is stripped of its enforceability
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the test for any touch
  of text that already discharges something
- `.kb/maps/domain-map.md` — documentation has no functional home yet; a new subject area
  is an appended section, never an edit

## Companions

Board-invisible drill-down for this card:

- [`_intake-brief.md`](_intake-brief.md) — this project's intake artifact
- [`_storymap.md`](_storymap.md) — the vertical-slice story map, authored by the briefs
  workflow and approved at the story-map review gate
- [`_design.md`](_design.md) — where DT-2, DT-3 and DT-8 are resolved; the perceptual
  review is a skip, so this is the only record of those choices
- The three warranted briefs — `architecture`, `ux`, `testing` — authored into this
  directory by the briefs workflow. No `deployment` brief is warranted
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
- [`../_decomposition.md`](../_decomposition.md) — the decomposition of record
- [`../initiative.md`](../initiative.md) — the initiative charter
