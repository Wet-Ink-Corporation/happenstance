---
id: HS-P0022
uid: 63196f
type: project
slug: application-author-path
title: The Application Author's Path
parent: HS-I0007
initiative: docs-that-teach
project: application-author-path
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
updated: 2026-08-19T20:13:31.563Z
links:
  pr: null
  commits: []
  kb: []
gate_open: true
schema: 1
process_rev: 7289a0c4
---
# The Application Author's Path

## One-line objective

Make the application author's first encounter with `happenstance` teach the one
thing the library exists for — a consistency boundary refusing an append — and
build the bridge from whichever prior mental model that reader actually arrives
holding, with both the boundary demonstration and the bridge's code checked by
the repository rather than asserted by prose.

## How this advances the initiative

This is the persona-1 vertical slice of *From Accurate to Teachable*
([`.bklg/docs-that-teach/initiative.md`](../initiative.md)), and it is the slice the
initiative's own evidence ranks first. The distillation calls it "the single
highest-confidence, lowest-new-machinery opportunity in the whole set — a content
and sequencing fix backed by a named literature, not a new proof mechanism"
(`.bklg/docs-that-teach/_discovery/distillation/opportunities.md`, Opportunity 1),
and the conceptual bridge is Opportunity 4 in the same document.

Two of the initiative's four reader-journeys are this project's: *the first fifteen
minutes* and *model my invariant in your words*. It owns three business requirements
(BR-03, BR-07, BR-18), three acceptance criteria (AC-01, AC-02, AC-08), two
Definition-of-Done scenarios (3 and 4) and four design tensions (DT-1, DT-4, DT-5,
DT-6) — the largest tension load of the six projects, and deliberately so: the
decomposition records that DT-5 and DT-6 are one coupled decision about how the
aggregate-to-boundary shift is drawn and how honest its old side is, and that DT-1's
anchor model must be settled *before* DT-4 stages anything against it
([`.bklg/docs-that-teach/_decomposition.md`](../_decomposition.md), "Why this cut").

It sits third in merge order. It consumes HS-P0020's checked narrative surface (a
page here that could opt out of the check is the failure this initiative exists to
prevent) and HS-P0021's page-need discipline (a page authored before the rule exists
is a page the rule is retro-fitted to). It unlocks HS-P0023, whose evaluator walk
needs pages to walk to, and HS-P0024, whose friction log measures the assembly rather
than the teaching if it runs against a half-built surface.

The gap this closes is measured, not predicted. This branch's crate root
(`crates/happenstance/src/lib.rs:58-67`) opens with a doctest that constructs a
`MemoryEventStore` and asserts the log is empty, while the prose thirty lines above
it states that composing decision models "is the mechanism that makes a dynamic
consistency boundary *dynamic*" (`crates/happenstance/src/lib.rs:44-46`). Both
statements are true; together they teach syntax while claiming semantics. The one
artefact in the workspace that *does* demonstrate the boundary —
`examples/course-subscriptions/src/main.rs`, whose `subscribe` handler spans a course
definition, every seat held and one student's own history in a single query
(`:113-174`) — is `publish = false` (`examples/course-subscriptions/Cargo.toml:8`),
carries no test target, and is reachable only from an unpublished README.

## In scope (this project)

- **Settling DT-1 in `_design.md`**: which prior mental model the teaching argues
  against — the DDD-aggregate reader, the stream-per-entity reader, or explicitly
  none — stated once, with the evidence that the incumbent store most readers come
  from presents stream-per-entity as an unmarked default
  (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, tension 6).
- **Settling DT-4 in `_design.md`**: whether the opening encounter uses staged,
  minimal-first disclosure or one complete program with commentary, against the
  training-wheels evidence and its named failure mode (a reader arriving mid-sequence
  from search with no signal they missed the setup).
- **Settling DT-5 and DT-6 together in `_design.md`**, because the dossier states
  they are coupled: whether the shift is taught with a diagram, prose narration or a
  mapping table; and whether a "wrong model" contrast ships as real compiled code, as
  an explicitly-exempt illustrative snippet with a stated narrow exemption, or not at
  all.
- **Authoring the opening encounter** on the surface HS-P0020 pins, in which an
  append is *refused* because a boundary held, and the refusal is visible in what the
  reader sees rather than only in what the page says.
- **Authoring the conceptual-bridge material**: a cross-entity rule stated in the
  reader's existing vocabulary carried through to its expression in `Query`,
  `QueryItem`, `Tags` and `AppendCondition`, without the reader needing
  `spec/SPECIFICATION.md` or the source to close the gap.
- **Bringing the existing worked example within reach of this reader.** The bet the
  initiative states is that sequencing and reach dominate content volume;
  `examples/course-subscriptions/`'s module doc (`src/main.rs:1-19`) is one of the
  three strongest explanations this project has written and is currently unpublished
  and unlinked.
- **Making the boundary claim checked rather than narrated (DoD-4).** Removing the
  boundary from the demonstrated scenario must make something in the repository fail.
  `cargo run -p course-subscriptions` is not a step in `REQUIRED`
  (`xtask/src/main.rs:105-`), and the example carries no `#[test]`, so its `bail!` on
  an unexpectedly-accepted append (`src/main.rs:43,54,61`) is reachable only by a
  human who runs the binary. Closing that is this project's, and it is what earns the
  `testing` brief.
- **Applying HS-P0021's discipline to the pages authored here** — each carries its
  one named answered-need — and citing `spec/SPECIFICATION.md` clauses rather than
  restating them.
- **Merging forward from `initiative/from-contract-to-published-library` before
  implementing.** The decomposition records this as an operational rule, not a
  preference: `crates/happenstance/src/lib.rs` is 76 lines here and 237 there, with
  the `Tags::empty()` defect intact in their version. Authoring against this branch's
  stale copy means redoing the work at merge.

## Out of scope (this project)

- **The pinned narrative tree, the gate step that builds and renders it, the
  compiled-fence mechanism, and the deliberately-broken-page falsification (DoD-2).**
  HS-P0020 `checked-documentation-surface`. This project authors pages *onto* that
  surface and does not build it. DT-7 (hidden panels) is HS-P0020's too, and the
  hosting shape — docs.rs-only versus a separate rendered surface — is inseparable
  from which tree is pinned by path, so it is theirs as well.
- **The page-need rule itself**, its taxonomy question (DT-2), where findability
  lives in it (DT-3), the safe-aside / load-bearing line (DT-8), and the written
  discipline's home. HS-P0021 `page-need-discipline`. This project *obeys* the rule; it
  does not author it.
- **The front door pointer into the narrative material, the evaluator's second
  question, the adapter author's trait-resolution error meeting its explanation at
  `crates/happenstance-core/src/store.rs`, and the surfaced `MemoryEventStore`
  reasoning account.** HS-P0023 `reach-and-adapter-path`, which owns DT-10 and every
  pointer policy. A page authored here states its own need and cites its clauses; who
  points *at* it is HS-P0023's.
- **Recruiting the non-author reader, running the friction-log session, and
  dispositioning what it finds.** HS-P0024 `comprehension-evidence`, which also owns
  DT-9. If the log shows this project's bridge does not land, the disposition routes
  through HS-P0024, not back into this charter silently.
- **Promoting personas and journeys into `.kb/product/`, reconciling with HS-S0131,
  and re-observing the fifteen Definition-of-Done scenarios on the assembled tree.**
  HS-P0025 `durable-audience-closeout`. No `.kb/` atom is hand-authored here; the
  first attempt at that was reverted (`0269720`).
- **Pinning the frozen documentation MUSTs by clause id (BR-10).** HS-P0020, and this
  project sits transitively behind the pin in the DAG.
- **README landing copy, status-truth vocabulary, the maturity census and registry
  metadata.** `publication-and-positioning` (HS-P0016) on the unmerged sibling branch.
  The seam is purpose, not paragraph: that project asks whether a claim is true, this
  one asks whether a reader can be taught. This project does not reopen its resolved
  design gate.
- **Amending or discharging any `spec/SPECIFICATION.md` clause.** The initiative is
  additive and discharges none.
- **Rewriting `happenstance-core` doc comments.** Not needed for this slice, and
  HS-P0023 is the project that does it, behind HS-P0020's pin. Where this project
  rewrites `crates/happenstance/src/lib.rs`'s crate-root doc, the referent may be
  rewritten and the reasoning may not
  (`.kb/governance/rewrite-the-referent-never-the-reasoning.md`).
- **Teaching a reader who knows neither event sourcing nor DCB.** Settled out on
  evidence at intake.

## Derived requirements

Expanded from the initiative requirements this project owns (BR-03, BR-07, BR-18).

**From BR-03 — the opening encounter demonstrates a boundary doing its job:**

- DR-01. The opening encounter must contain at least one append that is *refused*
  because an `AppendCondition` held, and the refusal must be observable in the
  reader's own output — not described in adjacent prose.
- DR-02. No code path in the opening encounter may construct an empty consistency
  boundary at a point where a real one is what the surrounding prose claims. Where an
  empty tag set is genuinely correct, the page must say why.
- DR-03. The scenario the encounter demonstrates must be exercised by the repository
  such that deleting the boundary makes the exercise fail. A demonstration nothing
  runs is a claim, and the initiative's central risk is a page that compiles forever
  while quietly ceasing to demonstrate its own claim.
- DR-04. The encounter's disclosure shape (DT-4) must be resolved in `_design.md`
  before authoring, and the resolution must address the documented failure mode of
  the chosen option — for staged disclosure, what a reader who arrives mid-sequence
  from search sees.
- DR-05. Every fenced code block in this project's pages must be exercised by
  HS-P0020's mechanism. An `ignore` fence, or any block that silently opts out, is a
  defect in this project's output regardless of whether the gate is green.

**From BR-07 — the teaching states which prior model it argues against:**

- DR-06. DT-1 must be resolved in `_design.md` as a single stated decision with its
  reasoning, and the option "explicitly none, teach from the invariant" must be
  treated as a real answer rather than a failure to choose.
- DR-07. Every page this project authors must apply that decision consistently. The
  initiative's bar is "once, consistently, rather than differently on each page", so
  the decision needs a citable location a later page can point at.
- DR-08. The bridge must start from a cross-entity rule stated in the reader's
  existing vocabulary and reach its expression in this library's vocabulary
  (`Query`, `QueryItem`, `Tags`, `AppendCondition`, `read_decision_model`) without
  requiring the reader to open `spec/SPECIFICATION.md` or the source.
- DR-09. Where the bridge makes a normative claim, it must cite the clause rather
  than restate it. Two sources of normative truth is how a guide becomes a second,
  weaker specification.

**From BR-18 — whether the shift is taught with a picture:**

- DR-10. DT-5 must be resolved in `_design.md` with a stated decision, including the
  option of narrating the write cycle in prose or a mapping table, which the dossier
  records as convergent across three independent sources and zero-cost.
- DR-11. If a diagram ships, the *arriving* model's mechanism — the
  query/append-condition cycle — must be drawn, not only the retiring model. Drawing
  only the old side reproduces the gap the specification's own site has.
- DR-12. DT-6 must be resolved jointly with DT-5. If a "wrong model" contrast ships
  as uncompiled code, the exemption must be written down, scoped narrowly, and
  visibly marked on the page as not-checked; if it ships as real code, it is checked
  by the same gate as everything else and maintained forever.

**Cross-cutting, from the decomposition's gate decisions:**

- DR-13. This project merges forward from
  `initiative/from-contract-to-published-library` before implementation begins, and
  the merge is recorded, because `crates/happenstance/src/lib.rs` has been replaced
  rather than merely diverged there.
- DR-14. Each page authored here carries the one named answered-need HS-P0021's
  discipline defines, in the form that discipline defines — this project does not
  invent a second notation for it.

## Acceptance criteria

Project-grain and testable. These are the spine the story map must cover.

- **AC-001.** `_design.md` records a single resolution for DT-1 naming which prior
  mental model the teaching argues against (or that it assumes none), with its
  reasoning and the evidence considered, and is signed off before any page in this
  project is authored.
- **AC-002.** `_design.md` records a resolution for DT-4 that names the disclosure
  shape of the opening encounter and states how the chosen shape's documented failure
  mode is handled.
- **AC-003.** `_design.md` records DT-5 and DT-6 as one joint resolution, covering
  both whether the shift is drawn and how honest the "old model" side is, and — if
  any uncompiled code ships — the exact, narrow basis of the exemption.
- **AC-004.** Following the opening encounter end to end from its first step, a
  reader reaches a running program in which an append is refused because a
  consistency boundary held, and the refusal appears in the program's own output.
  (Initiative DoD-3.)
- **AC-005.** Removing the boundary from that scenario — deleting or emptying the
  query the append condition is built from — makes a check the repository runs *fail*,
  observed once as a failure and once as a recovery after reverting. (Initiative
  DoD-4.)
- **AC-006.** No page authored by this project constructs an empty consistency
  boundary at a point where the surrounding prose claims a real one; where an empty
  one is correct, the page states why.
- **AC-007.** Every fenced code block on every page this project authors is exercised
  by HS-P0020's mechanism. An inventory of this project's blocks exists and shows zero
  opted out, or names each exception against AC-003's stated exemption.
- **AC-008.** Starting from a cross-entity invariant written in ordinary
  event-sourcing vocabulary, the bridge material carries it to a `Query`,
  a fold and an `AppendCondition` in this library's vocabulary, with no step requiring
  the reader to open `spec/SPECIFICATION.md` or the crate source. (Initiative AC-02.)
- **AC-009.** The DT-1 anchor decision is applied identically on every page this
  project authors, and each page that relies on it cites the one place the decision
  is recorded. (Initiative AC-08.)
- **AC-010.** `examples/course-subscriptions/` is reachable by the application author
  from this project's own material, and its module-level explanation
  (`src/main.rs:1-19`) is surfaced rather than paraphrased.
- **AC-011.** Every normative claim on a page authored here is a citation to a
  `spec/SPECIFICATION.md` clause that resolves, and a spot check confirms no clause is
  restated in the page's own words.
- **AC-012.** Every page authored here carries exactly one named answered-need in the
  form HS-P0021 defines, and a reviewer applying HS-P0021's check to this project's
  page set finds no page carrying two.
- **AC-013.** If DT-5 resolves to shipping a diagram, the query/append-condition cycle
  itself is drawn; the project does not ship a drawing of only the retiring model.
- **AC-014.** The merge forward from
  `initiative/from-contract-to-published-library` is completed and recorded before the
  first page is authored, and the opening encounter is written against the merged
  `crates/happenstance/src/lib.rs`, not this branch's copy.

## Definition of done (boundary-level)

1. `_design.md` is signed off carrying resolutions for DT-1, DT-4, DT-5 and DT-6 —
   four resolutions, with DT-5 and DT-6 joined. `design.capture` is deliberately
   absent from `.redkiln/config.yaml`, so the perceptual review is a skip and this
   written record is the *only* record these choices will ever have.
2. Initiative DoD scenario 3 is run and observed: the opening encounter reaches a
   running program in which an append is refused because a boundary held.
3. Initiative DoD scenario 4 is run and observed in both directions: the boundary is
   removed, a repository check fails, the removal is reverted, the check passes.
4. Every page this project authored builds and renders as part of `cargo xtask ci`
   through HS-P0020's step — not as a separate manual action.
5. `cargo xtask ci --fast` is green on this project's branch. This project is
   `terminal: false`, so the project-scoped integration bar applies; HS-P0025 owns the
   whole-initiative re-observation.
6. `cargo xtask spec-trace` passes over the tree as this project leaves it, and every
   clause citation added by this project resolves.
7. A reviewer walks this project's page set against HS-P0021's answered-need check and
   records the result.
8. The merge forward is recorded, and no page in this project's output was authored
   against the pre-merge `crates/happenstance/src/lib.rs`.
9. Anything this project found and did not fix is routed: incidental bugs to the
   `support` initiative per `.redkiln/config.yaml`, comprehension doubts to HS-P0024,
   pointer and reach gaps to HS-P0023. Nothing is absorbed silently.

## Dependencies

**Depends on** (from the decomposition DAG):

- **HS-P0020 `checked-documentation-surface`** — the pinned tree, the gate step, and
  the compiled-fence mechanism every page here consumes; also BR-10's clause-id pin,
  which this project sits transitively behind. AC-005 and AC-007 are unmeetable until
  HS-P0020's step exists.
- **HS-P0021 `page-need-discipline`** — the answered-need rule this project's pages
  obey (AC-012, DR-14) and the load-bearing/fold line (DT-8) that constrains how this
  project may present an aside.

**Unlocks:**

- **HS-P0023 `reach-and-adapter-path`** — its DoD-9 walks the evaluator's second
  question *to pages that answer it*, and its AC-11 needs narrative material to reach.
  Its DT-10 pointer policy can be resolved in design while this project is still
  authoring; only its implementation waits. Its adapter half (BR-15, AC-05) does not
  depend on this project at all.
- **HS-P0024 `comprehension-evidence`** — a friction log against a half-assembled
  surface measures the assembly, not the teaching.

**Cross-branch, not a DAG edge:** the merge forward from
`initiative/from-contract-to-published-library` (DR-13, AC-014). It is a precondition
on this project's implementation, not a blocking dependency on another Redkiln item —
that branch's schedule is its own and nothing here waits on its merge to `main`.

## Risks and coupling notes

| Risk | Likelihood / Impact | Note |
| --- | --- | --- |
| The opening encounter compiles forever and quietly stops demonstrating its claim | Medium / High | The named blind spot of every tool the initiative surveyed. AC-005 is the only instrument against it, and it is this project's alone — HS-P0020's broken-page falsification proves the *mechanism*, not the *claim* |
| DT-1 is deferred and each page anchors differently by accident | Medium / High | The exact defect BR-07 exists to prevent, and the dossier states DT-1 is upstream of DT-4. AC-001 sequences the sign-off before authoring |
| DT-5 absorbs effort disproportionate to its evidence | Medium / Medium | Ranked lowest-confidence at distillation, carried as a `Could` (BR-18) and explicitly optional relative to everything above. Prose narration is recorded as convergent, zero-cost and sufficient |
| A compiled "wrong model" side becomes a permanent maintenance liability | Medium / Medium | DT-6's own trade. If it ships as a real artifact it is `publish = false` and gate-checked forever; if it ships uncompiled it reproduces the `ignore`-fence shape this repository has already found once in its own doctests |
| Authoring against the stale `crates/happenstance/src/lib.rs` | Medium / High | DR-13 and AC-014. The sibling has replaced, not merely diverged from, the file this project is fixing |
| This project and HS-P0016 touch the same files | Medium / Medium | The seam is purpose, not paragraph. This project does not touch landing copy, status vocabulary or registry metadata, and does not reopen HS-P0016's resolved design gate |
| Rewriting the crate-root doc loses reasoning that still stands | Low / High | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`: the test is whether the edit changes what the document asserts. ADR-0006's argument for why the bare name is on the typed layer (`crates/happenstance/src/lib.rs:20-25`) is reasoning that still stands |
| The bridge is authored against a persona nobody has observed | High / Medium | Stated as an initiative assumption. All persona evidence is this project's own audit plus secondary evidence; HS-P0024 is the first contact with a real reader, and its dispositions may return work here |
| The project is measured in pages written | Medium / Medium | The cross-persona finding is that this is a findability-and-sequencing problem. AC-010 surfaces existing material rather than replacing it |

**Coupling worth stating explicitly.** DT-5 and DT-6 are one decision wearing two
numbers, and the dossier says so. DT-1 is upstream of DT-4. AC-005 is coupled to
HS-P0020's choice of what the gate runs over the narrative tree — if that mechanism
turns out not to be able to run the boundary scenario, this project owes a check of
its own (an integration test against `MemoryEventStore` in the manner of
`examples/course-subscriptions/src/main.rs:200-224`) rather than a weaker criterion.

## Context anchors

Initiative and plan:

- [`.bklg/docs-that-teach/initiative.md`](../initiative.md) — BR-03, BR-07, BR-18;
  AC-01, AC-02, AC-08; DoD 3 and 4; the DT table this project owns four rows of
- [`.bklg/docs-that-teach/_decomposition.md`](../_decomposition.md) — the DAG, the
  merge-forward rule, and why four tensions land here
- `.bklg/docs-that-teach/_discovery/distillation/opportunities.md` — Opportunity 1
  (the first fifteen minutes), Opportunity 4 (the conceptual bridge), Opportunity 8
  (the diagram gap, and why it is optional)
- `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` — Persona 1,
  their measured journey and their stated fear
- `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` — staged
  disclosure and its failure mode; tensions 4, 5 and 6; the anti-pattern list this
  project's pages are checkable against
- `.bklg/docs-that-teach/_discovery/research/03-teaching-the-aggregate-to-boundary-shift-how-dcb-events-disi.md`
- `.bklg/docs-that-teach/_discovery/research/05-interaction-pattern-prior-art.md`

Code and existing surfaces:

- `crates/happenstance/src/lib.rs` — the crate root this project rewrites; the
  boundary-free opening doctest at `:58-67`, the dynamic-boundary claim at `:44-46`,
  and ADR-0006's reasoning at `:20-25` which is preserved, not rewritten
- `examples/course-subscriptions/src/main.rs` — the one artefact that demonstrates the
  boundary: the three invariants at `:1-19`, the multi-entity query at `:113-125`, the
  fold at `:129-166`, and the append-under-condition at `:200-224`
- `examples/course-subscriptions/Cargo.toml:8` — `publish = false`, and no test target
- `xtask/src/main.rs:105-` — the `REQUIRED` gate steps, which do not include running
  the worked example

Knowledge base and standards:

- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` — an existing correct
  account of the mechanism the current opening encounter fails to demonstrate; useful
  as the bridge's destination, and a caution that the condition is derived from the
  read rather than independent of it
- `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — which crate a piece of
  teaching is actually describing
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — the test for any touch
  of a doc comment whose reasoning still stands
- `standards/rust/70-rustdoc-obligations.md` — the only constitution atom on
  documentation, and the source of the counter-example that is green under the whole
  workspace
- `standards/rust/README.md` — the five-tier precedence chain this work sits inside and
  does not extend
- `docs/README.md:25-29` — why a tree the gate reads is pinned by path
- `spec/SPECIFICATION.md` — the normative voice a page cites and never restates
- `.redkiln/config.yaml` — `design.capture` absent (the perceptual review is a skip);
  incidental bugs route to the `support` initiative

## Companions

- [`_intake-brief.md`](_intake-brief.md) — this project's intake artifact
- `_design.md` — where DT-1, DT-4, DT-5 and DT-6 are resolved; the only
  record those choices will have. Authored at the `design` stage, so it does not
  exist yet — deliberately not linked until it does.
- [`_storymap.md`](_storymap.md) — the vertical-slice story map covering AC-001…AC-014
- The warranted briefs for this project are `ux` and `testing`, per the brief table in
  [`../_decomposition.md`](../_decomposition.md); `architecture` and `deployment` are
  not warranted here. The `testing` brief is earned by AC-005 and by DT-6's possible
  compiled wrong side.
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_decomposition.md`](../_decomposition.md) — the decomposition of record
