---
id: HS-P0024
uid: f8cf33
type: project
slug: comprehension-evidence
title: Comprehension Evidence
parent: HS-I0007
initiative: docs-that-teach
project: comprehension-evidence
status: in-review
process: project
stage: design
automation: HITL
severity: null
blocked_by: []
blocks: []
terminal: false
owner: ryan-britton
created: 2026-08-17
updated: 2026-08-17T05:42:20.888Z
links:
  pr: null
  commits: []
  kb: []
gate_open: false
schema: 1
process_rev: 7289a0c4
---
# Comprehension Evidence

## One-line objective

Put the assembled teaching material in front of one genuine non-author, non-insider
reader, record what stopped them in an auditable shape, and give every stumble a
disposition — so the initiative holds evidence that a reader was taught, not only a
gate that says the pages compile.

## How this advances the initiative

This is the **second, non-substitutable instrument**. The initiative's own framing is
that a green documentation gate and a comprehension record falsify different things
and neither may stand in for the other
([`../initiative.md`](../initiative.md), `## Risks`, first row). The gate that
HS-P0020 `checked-documentation-surface` builds proves a page's code still compiles
against the real crates; the research angle this project rests on says plainly that
*every* tool surveyed is blind to code that still compiles and no longer demonstrates
the claim the prose makes
([`../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`](../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md),
findings on rustdoc/`mdbook test` and on friction logging versus acceptance testing).

The initiative therefore carries two proof artefacts. HS-P0020 owns the first. This
project owns the second, and it is the only place in the six-project tree where a
reader who is not the author touches the material at all. Without it, three of the
initiative's own assumptions — that the three named audiences are the right ones, that
sequencing and reach dominate content volume, and that this project's vocabulary
answers rather than reproduces the field's "no replacement noun" stall point — remain
restated rather than tested (`../initiative.md`, `## Assumptions`, `## Open questions
for the planning team`).

It also supplies the one input HS-P0025 `durable-audience-closeout` cannot manufacture.
`.kb/product/README.md` states the bar directly: *"A persona nobody researched is a
stock photo with a name."* Every persona in this initiative today carries the
qualification that **none has been directly observed**
([`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md),
`## Risks`). This project is what converts exactly one of those qualifications into an
observation, and the decomposition makes the hand-off explicit: the promotion "must
record precisely which persona the friction log *did* directly observe rather than
shipping the blanket 'none has been directly observed' qualification unchanged"
(`../_decomposition.md`, `### Why the load-bearing edges exist`).

## In scope (this project)

- **Resolving DT-9** — which persona the comprehension session walks — in this
  project's `_design.md`, with the reason recorded and the other two personas' evidence
  status explicitly left marked as inferred (`../_decomposition.md`, `## Design tension
  ownership`).
- **The session protocol, fixed before the reader is recruited**: the scenario stated in
  the reader's own terms, the comprehension technique chosen from the published
  paraphrase / plus-minus / task-based taxonomy and justified against the persona
  chosen, the narration mode (concurrent versus retrospective) declared, and the
  severity scale named (research 04, findings on the friction-log template, the
  three-technique taxonomy, Hertzum's think-aloud meta-analysis, and Nielsen's 0–4
  scale).
- **Recruitment and the non-insider precondition.** Stating, in checkable terms, what
  disqualifies a candidate — having authored the material under test, having read
  `crates/happenstance-core/`'s source, `references/adr/` or `references/evaluation/` —
  and recording the recruited reader's declared status in the log itself. Research 04
  treats non-authorship as the *mechanism*, not a nicety: insiders unconsciously route
  around the rough spots the exercise exists to find.
- **Running the session against the assembled material** and producing the dated log in
  the auditable shape: scenario, logger identity and context, a chronological record
  including search terms typed and links followed, reactions captured as they occur,
  and severity marked inline.
- **Dispositioning every stumble** — fixed, deliberately accepted with a recorded
  reason, or routed with a destination id — and routing the log to a named owner able to
  act on it. Research 04 is explicit that a log with no destination is opinion by
  another name.
- **Scoping the claim in writing** to what the method supports: real stumbles were
  captured and are traceable. Never exhaustiveness (BR-14).
- **The optional cognitive-walkthrough pre-screen** the authors may run themselves
  before spending the reader's time — explicitly a hypothesis generator, never the proof
  artefact (research 04, on Wharton et al.'s four-question protocol).
- **The evidence hand-off note** that tells HS-P0025 exactly which persona was directly
  observed, on what date, against which tree.
- **Small content fixes arising from dispositions** where the fix does not reopen a
  settled design tension. Anything that would reopen one is escalated, not absorbed
  (`../initiative.md`, `## Assumptions`, on the publication project's design gate).
- **The AC-05 fallback**: if the log shows the surfaced `MemoryEventStore` walk-through
  does not carry an adapter author, a fuller account is owed, and that finding is
  dispositioned here (`../_decomposition.md`, `**AC-05's scope**`).

## Out of scope (this project)

- **The gate that compiles narrative prose, the pinned tree, and the
  deliberately-broken-page falsification.** HS-P0020 `checked-documentation-surface`
  owns BR-01, BR-02, BR-10, BR-11's demonstration and BR-12's pinning-by-path. This
  project consumes that surface and does not modify it.
- **The one-need-per-page rule, the taxonomy decision, and the discipline's written
  home.** HS-P0021 `page-need-discipline` owns BR-04, DT-2, DT-3, DT-8 and AC-13. A
  stumble that says a page answers two needs is routed there, not rewritten here as a
  new rule.
- **The opening encounter, the prior-model anchor, the diagram decision and the
  wrong-model contrast.** HS-P0022 `application-author-path` owns BR-03, BR-07, BR-18
  and DT-1/4/5/6. This project may find that the staged encounter fails a real reader;
  the remedy is dispositioned, and re-deciding DT-1 is an escalation.
- **The front door, the evaluator's second question, and the `store.rs` error site.**
  HS-P0023 `reach-and-adapter-path` owns BR-08, BR-15, BR-16, AC-04/05/06/11 and DT-10.
- **Persona reconciliation and promotion into `.kb/product/`.** HS-P0025
  `durable-audience-closeout` owns BR-13, BR-17, AC-14 and DoD-15. This project produces
  the observation; it does **not** author, promote or reconcile any persona atom.
  Hand-authoring `.kb/` atoms outside the ingest path is a named non-goal of the
  initiative, and the first attempt at it was reverted (`0269720`).
- **Re-observing all fifteen Definition-of-Done scenarios from a clean checkout.** That
  is HS-P0025's terminal obligation (`../_decomposition.md`, `### Definition of Done`).
- **Quizzes or inline recall checks.** Ruled out on evidence at discovery: they measure
  recall of the author's own prose. Settled, not reopenable here
  ([`../_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md),
  `## Anti-patterns`).
- **Incidental bugs found by the reader in the library itself.** They route to the
  `support` initiative per `.redkiln/config.yaml:5` (`support_initiative: support`), as
  a disposition — not as an in-scope fix.

## Derived requirements

Expanded from the three initiative business requirements this project owns (BR-05,
BR-06, BR-14) plus DT-9, per `../_decomposition.md`'s traceability matrix.

**From BR-05 — a dated friction log from a non-author, non-insider reader.**

1. The disqualifying criteria for a logger are written down *before* recruitment, name
   the specific artefacts that confer insider knowledge in this repository, and are
   recorded alongside the recruited reader's declared status.
2. The log is dated, names the reader's platform/toolchain context, and identifies the
   exact tree it walked — the assembled material as merged from HS-P0020 through
   HS-P0023, at a named commit.
3. The log records what the reader *did*, not whether they liked it: search terms typed,
   links followed, files opened, code copied, commands run.

**From BR-06 — auditable shape, and routed to someone who can act.**

4. The scenario is one to two sentences in the reader's own terms and is fixed before
   the session starts; research 04 calls the scenario "the most crucial part", failing
   in both directions (too narrow proves nothing, too esoteric is dismissed as an edge
   case).
5. Reactions are captured concurrently rather than summarised afterwards, and the
   narration mode is a stated methodological choice — the meta-analytic evidence shows
   concurrent and retrospective narration surface measurably different findings, and
   that concurrent narration inflates task time by roughly 17–20%, which is recorded
   rather than treated as a comparable metric.
6. Severity is marked inline as the log is written, on a named scale, never retrofitted.
7. Every marked stumble carries exactly one disposition — fixed, deliberately accepted,
   or routed — with the fix, the reason, or the destination id attached to it.
8. The log is submitted to a named owner with authority to act, and that submission is
   recorded. A log that is written and filed is a diary.

**From BR-14 — the claim scoped to what the method supports.**

9. Every summary of this evidence states the narrow claim (real stumbles were captured
   and are traceable) and never claims exhaustiveness. The small-sample basis that makes
   one session defensible — a single qualitative session surfaces roughly a third of a
   design's problems, and each problem a reader hits is already proven worth fixing —
   supports the narrow claim only.
10. If the session's findings are dominated by a single blocking defect, a second
    session is an explicit decision — run, or declined with a recorded reason — rather
    than a default (`../initiative.md`, `## Assumptions`).

**From DT-9 — which persona the session walks.**

11. The persona is chosen and the choice is recorded in `_design.md` with its reason,
    together with the comprehension technique that fits it, and an explicit statement
    that the other two personas' evidence remains inferred. `design.capture` is
    deliberately absent from `.redkiln/config.yaml`, so the written `_design.md` is the
    only record this decision will ever have.

## Acceptance criteria

Project-grain and testable. Each is checkable by a reviewer reading a named artefact,
not by asking the author what they remember.

- **AC-001 — The persona choice is decided, not defaulted.** `_design.md` names which
  persona the session walks, why, which comprehension technique fits it, and states that
  the other two personas' evidence stays inferred. (DT-9)
- **AC-002 — The protocol is fixed before recruitment.** `_design.md` carries the stated
  scenario, the narration mode, the severity scale, and the disqualifying criteria for a
  logger — all authored before any reader is approached, and traceable to a commit that
  precedes the session date.
- **AC-003 — The logger is verifiably not an author and not an insider.** The log
  carries the reader's own declaration against the criteria in AC-002, and a reviewer can
  check that declaration without asking the reader anything further.
- **AC-004 — A dated log exists in the auditable shape.** It carries: scenario; logger
  identity, context and date; a chronological record including search terms and links
  followed; reactions captured as they occurred; severity marked inline. A reviewer can
  tick all six against the file.
- **AC-005 — The log names the tree it walked.** The commit or merge point of the
  assembled material (HS-P0020 → HS-P0023) is recorded in the log, so a later reader can
  tell whether a stumble still applies.
- **AC-006 — Every stumble has exactly one disposition.** Reading the log end to end,
  each severity-marked item resolves to a fix, a recorded deliberate acceptance with its
  reason, or a routed item with a destination id. Zero undispositioned items; zero items
  with two dispositions.
- **AC-007 — Routing is real and lands correctly.** The log is submitted to a named
  owner able to act, that submission is recorded, and each routed item's destination is
  the correct one: library bugs to the `support` initiative (`.redkiln/config.yaml:5`),
  page and structure defects to the sibling project that owns the requirement, deliberate
  deferrals to a recorded open question.
- **AC-008 — The claim is scoped in writing wherever it is made.** The log and its
  summary each state that the evidence supports "real stumbles were captured and are
  traceable" and neither asserts exhaustiveness. (BR-14)
- **AC-009 — The hand-off to closeout is explicit.** A named section states which single
  persona was directly observed, on what date, against which tree — in the form
  HS-P0025 needs to replace the blanket "none directly observed" qualification with an
  accurate one.
- **AC-010 — A second session is a decision, not an omission.** Whether the findings
  were dominated by one blocking defect is assessed and recorded, with a second session
  either run or declined for a stated reason.
- **AC-011 — No settled decision was reopened in passing.** Every disposition that would
  change a resolved design tension owned by a sibling project is recorded as an
  escalation with the tension's DT id, rather than absorbed as a fix here.

## Definition of done (boundary-level)

1. **DoD-5 of the initiative is observed.** A reader who is neither the author nor an
   insider completes the stated scenario, and the session produces the dated log in the
   auditable shape (AC-002 through AC-005).
2. **DoD-6 of the initiative is observed.** Every stumble in that log carries a
   disposition, checked by reading the log end to end, with none left undispositioned
   (AC-006, AC-007).
3. **DT-9 is resolved in `_design.md` and the design review gate is approved by a
   human** — the project process makes that gate a human review with `approved` /
   `changes-requested` verdicts (`.redkiln/processes/project.yaml`, `design` stage), and
   the perceptual review is a skip, so the written resolution is the whole record.
4. **The scope statement is present wherever the evidence is cited** (AC-008), including
   in whatever HS-P0025 will read at closeout.
5. **The hand-off note exists and names one directly observed persona** (AC-009).
6. **No `.kb/` atom was hand-authored by this project.** Promotion happens through the
   ingest path at closeout, and is HS-P0025's.
7. **The non-terminal integration bar is green** on this project's tree: `cargo xtask ci
   --fast`, wired as `integration_scoped` in `.redkiln/config.yaml:55`. This project is
   `terminal: false`, so the whole-gate `e2e` bar is HS-P0025's, not this one's.
8. **`redkiln validate` and `redkiln doctor` are clean** for this project's artifacts,
   with no new `template-drift` beyond the six standing advisories CLAUDE.md documents.

The repository's own gate being green is a **precondition** for reading this project's
evidence, never a substitute for it.

## Dependencies

From the DAG in `../_decomposition.md`, `### Dependency graph`.

**Depends on**

- **HS-P0022 `application-author-path`** — the opening encounter and the conceptual
  bridge. A session run before these exist measures an absence.
- **HS-P0023 `reach-and-adapter-path`** — the front door, the evaluator's second
  question, and the `store.rs` error site. Two of the three candidate personas for DT-9
  have nothing to walk until this merges.

Transitively: HS-P0020 `checked-documentation-surface` (the pinned tree and the gate
step) and HS-P0021 `page-need-discipline` (the rule the pages were authored under).
The decomposition's reason for this edge is stated directly: *"A friction log run
against a half-assembled surface measures the assembly, not the teaching."*

**Unlocks**

- **HS-P0025 `durable-audience-closeout`** — which cannot state which persona was
  directly observed, and therefore cannot promote the audience honestly, until this log
  exists.

**Merge order position:** fifth of six (`../_decomposition.md`, `### Merge order`).

## Risks and coupling notes

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| No genuinely non-insider reader can be recruited in time, and an insider is used instead | Medium / High | Non-insider status is a stated precondition with written disqualifying criteria (AC-002, AC-003), not a nicety. The evidence implicates insider knowledge, not authorship, as what routes a logger around rough spots. If no reader is found, the artefact is unmet — that is a failure of the initiative, not a reason to redefine the bar (`../initiative.md`, `## Assumptions`) |
| The evidence is overclaimed as exhaustive downstream, especially at promotion | Medium / Medium | AC-008 requires the scope statement wherever the claim is made, and AC-009's hand-off note carries it into HS-P0025's input rather than trusting the reader of the log to re-derive it |
| Dispositions become a sink: everything is "routed" and nothing is fixed | Medium / Medium | AC-006 requires exactly one disposition per stumble with the destination id attached; AC-007 requires the destination to be the correct owner. A routed item with no id fails the criterion |
| The session finds a defect that reopens a settled design tension (DT-1's anchor model, DT-6's wrong-model contrast) | Medium / High | AC-011 makes reopening an *escalation with the DT id*, never an absorbed fix. The four content tensions were signed off in HS-P0022's design review; re-deciding one here would put half a decision in each |
| The session walks material that is about to be replaced by the sibling branch merge | Medium / High | `crates/happenstance/src/lib.rs` is 75 lines here and 237 there, with the `Tags::empty()` defect intact in their version (`../_decomposition.md`, `## Decisions taken at the gate`). HS-P0022 merges forward *before* it implements; this project's session must run against the post-merge tree and AC-005 records which tree that was |
| One session's findings are dominated by a single blocking defect, making the rest of the evidence thin | Medium / Medium | AC-010 forces the second-session question to be answered explicitly rather than by silence |
| The severity marks drift into a mood report | Low / Medium | A named published scale is fixed in `_design.md` before the session (AC-002) and applied inline (AC-004), which is the difference research 04 draws between a rankable record and a diary |
| A disposition rewrites a `happenstance-core` doc comment that discharges a frozen clause | Low / High | Any such fix is bound by `.kb/governance/rewrite-the-referent-never-the-reasoning.md` and sits behind HS-P0020's clause-id pin in the DAG; the referent may be rewritten, the reasoning may not |

**Coupling notes.** This project's output flows *backwards* into projects that have
already merged. That is deliberate — a friction log is worthless if it cannot cause a
change — but it means the story map must say where a fix lands: small content fixes as
stories under this project, citing the sibling's material; anything larger, or anything
touching a resolved tension, routed or escalated with an id. The second coupling is
temporal: this project is the only one whose schedule depends on a person outside the
team being available, so recruitment should begin while HS-P0023 is still in flight even
though the session cannot run until it merges.

## Context anchors

Initiative and plan of record:

- [`../initiative.md`](../initiative.md) — BR-05, BR-06, BR-14; AC-09, AC-10; DoD-5 and
  DoD-6; DT-9; the two-proof-artefacts framing and the assumptions this project tests
- [`../_decomposition.md`](../_decomposition.md) — this project's owned ids, the DAG
  edge that puts it after all content, the AC-05 fallback clause, and the warranted
  briefs (`ux`, `testing`; no `architecture`, no `deployment`)

Discovery corpus — the method this project implements:

- [`../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md`](../_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md)
  — the friction-log template, non-authorship as mechanism, routing as the non-optional
  final step, the paraphrase / plus-minus / task-based taxonomy, cognitive walkthrough as
  a pre-screen, the small-sample basis, the 0–4 severity scale, and concurrent versus
  retrospective narration
- [`../_discovery/distillation/personas-and-journeys.md`](../_discovery/distillation/personas-and-journeys.md)
  — the three candidate personas for DT-9, each with its fitting technique named, and the
  explicit risk that the friction-log reader is not automatically identical to any one of
  them
- [`../_discovery/distillation/interaction-patterns.md`](../_discovery/distillation/interaction-patterns.md)
  — quizzes ruled out as a substitute; the anti-pattern that a comprehension claim must
  trace to a non-author reader's dated record

Knowledge base and repository constraints:

- `.kb/product/README.md` — the bar this project's evidence exists to clear, and the rule
  that an unevidenced sketch stays in `_discovery/` until closeout promotes it
- `.kb/governance/rewrite-the-referent-never-the-reasoning.md` — binding on any
  disposition that edits a doc comment already discharging a clause
- `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` — the existing correct
  account of the mechanism a paraphrase-style check would ask the reader to restate
- `.redkiln/config.yaml:5` — `support_initiative: support`, the destination for a routed
  library bug
- `.redkiln/config.yaml:55` — `integration_scoped: cargo xtask ci --fast`, this
  non-terminal project's integration bar
- `.redkiln/processes/project.yaml` — the `design` stage's human review gate, which is
  where DT-9 is signed off

Surfaces the session may walk (depending on DT-9):

- `docs/README.md` — the signpost tree reserved for a using-the-library reader, and the
  statement of why a gate-read tree is pinned by path
- `examples/course-subscriptions/` — the one worked example, and the artefact the
  application author's journey stalls on today
- `crates/happenstance-core/src/store.rs` — the file the adapter author is looking at
  when the trait-resolution error fires

## Companions

Board-invisible drill-down for this card:

- [`_intake-brief.md`](_intake-brief.md) — this project's intake artifact
- [`_decomposition.md`](_decomposition.md) — the project-grain briefs (primary)
- [`_grounding.md`](_grounding.md) — the grounding brief authored at the same stage
- [`_storymap.md`](_storymap.md) — the vertical-slice story map, authored by the briefs
  workflow and approved at the story-map review gate
- [`../initiative.md`](../initiative.md) — the initiative charter
- [`../_decomposition.md`](../_decomposition.md) — the initiative decomposition of record
- [`../_plan.md`](../_plan.md) — the initiative planning rollup
