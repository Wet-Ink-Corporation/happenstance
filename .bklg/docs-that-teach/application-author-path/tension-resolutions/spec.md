---
item: HS-S0184
stage: spec
created: 2026-08-17T13:16:32.581Z
updated: 2026-08-17T13:16:32.581Z
template_sig: 87bbf1d0
rendered_sig: 1703cc07
---

# Spec — DT-1, DT-4, DT-5 and DT-6 resolved and recorded

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project | `.bklg/docs-that-teach/application-author-path/project.md` |
| This spec | `.bklg/docs-that-teach/application-author-path/tension-resolutions/spec.md` |
| Key briefs | `.bklg/docs-that-teach/application-author-path/_decomposition.md` — UX brief `:11-447` (IQ invariants `:229-315`, UX-001…UX-015 `:317-378`), testing brief `:448-650` (test mix `:475-513`, AC-to-tier map `:515-541`) |
| Signed-off design (binding) | `.bklg/docs-that-teach/application-author-path/_design.md` — approved 2026-08-17 (`:1037-1039`) |
| Design mock (evidence, not synced) | `.bklg/docs-that-teach/application-author-path/design/mock.html` |
| Grounding | `.bklg/docs-that-teach/application-author-path/_grounding.md` |
| Story map / merge order | `.bklg/docs-that-teach/application-author-path/_storymap.md` — this story's row `:56`, slice rationale `:66-69`, archetype note `:95-98`, merge order `:138-140` |
| Slice-mate's spec (read first) | `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` |
| Roadmap pointer | `RUNBOOK.md` — this initiative is documentation work and adds no phase; the roadmap is not amended here |

## One-line PR slice

Certify DT-1, DT-4 and the joint DT-5+DT-6 resolution against the merged tree and fix the
one citable location — heading text, emitted fragment id, and per-story consumption map —
so every later page in this project points at the same anchor instead of re-deriving it.

## Executive summary

This PR lands **one resolution record and zero pages**, and its delta is not the four
resolutions — those already exist.

The pointer is project AC-001/AC-002/AC-003 (`project.md:232-241`), which the story map
assigns here and which the AC-to-tier map scores at tier 5, "`_design.md` sign-off, DoD item
1" (`_decomposition.md:515-541`). Read literally, that has already happened:
`_design.md` carries all four resolutions under `## Pattern decision` (`:76-262`) and the
repository owner approved the file on 2026-08-17 (`:1037-1039`), on the second pass, after
rejecting it once.

So the honest delta is what the sign-off does **not** yet carry, and it is three things a
page author would otherwise hit one at a time, mid-authoring:

1. **The approval is conditional, and one condition is unabsorbed.** The approver's row
   records that `crate-root-encounter` is rustdoc-framed but "the three step-page surfaces
   render as markdown under HS-P0020's approved 'the markdown is the render' decision, so
   their chrome assumptions do not carry" (`_design.md:1039`). The binding sections above it
   still state rustdoc selectors for all four surfaces (`:39-72`) and a `##` heading budget
   derived entirely from rustdoc's 200px sidebar (`:575-584`). Which surfaces that budget
   binds is unstated, and four pages are about to be written under it.
2. **The design mandates retitles it does not supply.** `_design.md:575-585` states that
   "Eleven of the sixteen headings this design names exceed" the 22-character budget and that
   "the retitles in `## Composition` are structural, not cosmetic" — but `## Composition`
   (`:408-504`) still names the long ones. Measured here: `## Watch a boundary refuse` is 23
   characters on the one surface where the rustdoc budget certainly applies, and the DT-1
   anchor's own heading, `## Where your streams went`, is also 23. A retitle moves the slug,
   and the slug is the single string DT-1's resolution hands to every other page
   (`:108-114`). Deciding it four times, once per page story, is the exact defect BR-07 and
   DR-07 exist to prevent (`project.md:191-197`).
3. **Two resolutions make claims the merged tree can now falsify, and only now.** DT-6
   commits to **zero** entries on HS-P0020's allowance list and to a wrong-side fence that
   "compiles, runs and passes" while asserting the append is *accepted* (`_design.md:206-244`,
   `:1055-1058`). DT-4 commits to three steps "each a complete runnable program" inside a
   24-line, 68-column budget. Both were reasoned against this worktree's 75-line crate root;
   `merge-forward-preflight` replaces it with the sibling's 237-line file. The design itself
   states the consequence: "if a fence turns out to be genuinely unwritable as compiled code,
   this design must be reopened rather than an exemption written" (`:1055-1058`).

The deliverable is therefore a **resolution record** that certifies each resolution with a
probe that could fail, fixes the anchor table once, and routes anything falsified to the
sign-off owner as a reopen condition — rather than absorbing it silently, which is what the
design gate already rejected this project for once (`_design.md:1041-1048`).

## Context pack

**What this story may and may not decide.** `_design.md` is signed off and binding
(`:10-12`, approver row `:1039`). This story **executes** it and does not re-open it: it
supplies values the design mandates but leaves blank, measures claims the design states as
facts, and records the result. Where a measurement contradicts a binding statement, the
response is the same one the slice-mate uses — record it with a disposition and route it
(substrate to HS-P0020, pointer and reach to HS-P0023, comprehension to HS-P0024, incidental
bugs to the `support` initiative, `.redkiln/config.yaml:5`) per project DoD item 9
(`project.md:300-302`). Re-deciding DT-1, DT-4, DT-5 or DT-6 here would re-litigate a human
sign-off inside a preflight PR, and `merge-forward-preflight` has already declined to do it
once for the same reason.

**The three resolutions, stated as the decisions they are — not as a reading list.**

- **DT-1 is invariant-first, with the stream-per-entity prior named exactly once, at the
  seam where it bites** — formally option (c) bounded by a single located use of (b)
  (`_design.md:81-120`). The reader does not arrive holding a *pain*; they arrive holding an
  invariant they cannot place, which is already what AC-008's bridge is made of. The prior is
  named at precisely the moment the reflex question — *which stream does this go in?* — is the
  next thing the reader will think. The DDD-aggregate anchor lost on statistics, not taste:
  EventStoreDB "presents stream-per-entity as an unmarked default", making "one stream per
  entity" the more common prior (`interaction-patterns.md:547-566`). **The consequence that
  binds every other surface**: `crate-root-encounter` and `opening-encounter` name no prior
  model at all, and anti-pattern 6 makes that checkable by forbidding the words "aggregate",
  "your aggregates", "one stream per entity" and "which stream" on them (`_design.md:852-854`).
- **DT-4 is staged, minimal-first: three steps, each a complete runnable program, with no
  fourth "whole program" artifact** (`_design.md:122-168`). The three steps are already fixed
  by content and anchor (`:133-139`). The literature's named failure mode — a reader arriving
  mid-sequence from search past the setup — is answered structurally, not advisorily: **no
  step is a fragment** (the steps are cumulative in teaching, never in execution, which
  removes the interdependence rather than warning about it); **an in-body step header block**,
  two lines, because rustdoc's sidebar TOC is removed entirely below 700px and a mitigation
  that disappears on a phone is not one; and **the payoff sits in the step most likely to be
  landed on cold** — step 3 is the refusal, so cold arrival becomes the best available
  landing rather than a signposted loss. Flattening this to "staged disclosure, three steps"
  loses the whole resolution: the mitigation *is* the decision.
- **DT-5 + DT-6 are one resolution.** DT-5: **narrate the cycle** — a three-column mapping
  table in a fixed column order plus a four-step narration using the same four names
  everywhere, *tag, query, fold, guard*, because synonym drift across pages is how a narration
  loses to a diagram. **No diagram ships**, and AC-013 is discharged explicitly by the
  reviewer recording that it went that way, not vacuously by silence (`_design.md:170-204`,
  `_storymap.md:120`). DT-6: **real, compiled, executed code in the narrowest form the
  contrast can take** — one fence differing from the correct one by a single expression, a
  guard **tagged too narrowly** (scoped to what the command *writes* rather than to the
  invariant it must *hold*), asserting the outcome the reader should fear: the append is
  *accepted* and the invariant is violated with no error. **This is the mirror of CF-7, not
  CF-7 itself, and the distinction is the whole point**: CF-7 (`spec/SPECIFICATION.md:7266`,
  `[FROZEN]`) names the *broadening* failure, which over-refuses and would make an `is_ok()`
  assertion fail. The hazard worth teaching is the quiet one (`_design.md:206-244`). DT-6
  therefore puts **zero** entries on HS-P0020's allowance list — a stronger commitment than
  AC-003 requires, which removes a dependency and, in exchange, means an unwritable fence
  reopens the design instead of buying an exemption (`:1055-1058`).

**The persona-journey slice this serves.** Backbone activity A1 — *"Give me a baseline I am
not about to redo"* (`_storymap.md:41`) — the half the slice-mate does not cover. The reader
never sees this PR. What they see is that the same prior model is named the same way in the
same place on every page they walk, which is initiative AC-08's whole content and the thing
the initiative's bar states as "once, consistently, rather than differently on each page"
(`project.md:194-197`). Persona 1's stated fear is "a mental model that looks right, compiles,
runs, and is quietly wrong" (`personas-and-journeys.md:99-105`); a teaching that anchors
differently on each page is that fear applied to the teaching itself.

**Why the anchor must be fixed here rather than by the page that renders it.** DT-1's
resolution names one reader-facing home — the `conceptual-bridge` heading **"Where your
streams went"**, cited as `#where-your-streams-went` (`_design.md:108-114`) — and requires
every other page to *link to that heading* rather than re-argue it. But the page that renders
that heading is `invariant-to-appendcondition-bridge`, which is two slices downstream, while
`boundary-refusal-encounter` (next slice) already needs the string. A fragment decided by the
page that renders it and consumed by pages written earlier is a link that is correct only by
luck. Fixing the heading text and its emitted id here is what makes UX-009 and AC-009
checkable by `answered-need-and-anchor-review` against a table instead of a memory.

**The measured contradiction this story must reconcile, stated as a fact rather than as work
to redo.** The 22-character `##` budget exists because rustdoc's sidebar TOC is 200px wide
and clips with `white-space:nowrap; text-overflow:ellipsis` — verified in the rendered
stylesheet, and re-derived at the design gate from finding F-2 (`_design.md:575-584`,
`:996`, `:1013`). Sign-off condition 2 says the three step-page surfaces are markdown, not
rustdoc (`:1039`). Those two statements do not disagree; nobody has written down what falls
out. Both `## Watch a boundary refuse` (23 characters, on the one surface where the budget
certainly binds) and `## Where your streams went` (23, on a surface where it may not) sit one
character over. Reconciling scope, then fixing final titles for every `##` the design names,
is executing `_design.md:582-584`'s own instruction that the retitles are structural.

**The gate bar.** This project is `terminal: false` (`project.md:17`), so the story-grain bar
is `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) and the project bar is
`cargo xtask ci --fast` (`:55`); `reachability_static` runs `cargo xtask lints && cargo xtask
spec-trace` (`:48`), which is the grain that matters for a story whose deliverable maps to no
package. This story authors no Rust, so `affected` will find little — which is why its
certification probes must be **run and transcribed**, not asserted. A tier-5 story whose
evidence is "the design says so" is the decorative check CLAUDE.md names.

**What must not happen in this PR.** No page, no fence authored into any surface, no edit to
`_design.md`, no edit under `crates/`, `examples/`, `spec/`, `xtask/` or `docs/`. The probes
compile against the merged tree; they do not land in it.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate (the certified resolutions and the
  fixed anchor table) consumed by the capability slices in this same project, not a double
  and not a fixme. `_storymap.md:95-98` states the demonstration explicitly: every page story
  consumes this record, and `answered-need-and-anchor-review` proves the consumption by
  checking that every page cites it.
- **Slice / milestone**: `preflight-and-anchor`. Slice-mate: `merge-forward-preflight`
  (HS-S0183), which this story blocks on and is delivered with as one integrated slice. The
  slice is one slice and not two because both stories gate the same thing — authoring — and
  splitting them permits a page written against a merged tree with an unsettled anchor, or
  against a settled anchor on a stale tree (`_storymap.md:66-69`).
- **Mount point**: `.bklg/docs-that-teach/application-author-path/_design.md`. It is the
  composition root of this project in the literal sense the repurposed design stage means:
  the single binding artifact every page story loads before it writes a line, and the file
  whose `## Pattern decision` sub-sections (`:76-262`) this story certifies and whose blank
  values (heading titles, emitted fragment ids, budget scope) it fills. This story does not
  edit it — it mounts **onto** it by publishing the record the design's own sign-off
  conditions leave outstanding, at a path the four downstream specs open by name. The mount
  is observable downstream rather than in `target/`: it has happened when a page story can
  write `[Where your streams went](…#where-your-streams-went)` without opening `_design.md`
  to guess the slug, and it has not happened if any page story re-derives one.
- **Secondary mount, for the shipped tree**: the reader-facing anchor
  `conceptual-bridge § "Where your streams went"` (`_design.md:108-114`, composition slot 3,
  `:478-480`). This story fixes the string; `invariant-to-appendcondition-bridge` renders the
  heading and `boundary-refusal-encounter` is the first page to link it. That two-step —
  decided once here, rendered once there, linked from everywhere — *is* AC-009's mechanism.
- **Wires into**:
  - `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/_baseline.md` —
    the slice-mate's record. `§ Anchors` supplies the merged clause lines this record cites
    id-first, `§ Dispositions` supplies the four merge contradictions this record must carry
    a per-resolution status for, and `§ Composition baseline` supplies the re-measured
    heading, bracket, hidden-line and fence figures the budget reconciliation is computed
    against. This story does not re-run that archaeology.
  - `crates/happenstance/src/lib.rs` (merged) and `crates/happenstance-core/src/store.rs`,
    `crates/happenstance-core/src/append.rs` — read-only, as the API the DT-4 and DT-6 probes
    compile against. Design-gate finding F-5 is the standing warning: the first binding fence
    shape named `EventStore::head` with an argument and `Guard::new`, neither of which exists
    (`_design.md:354-364`, `:999`).
  - `spec/SPECIFICATION.md` — ES-25 (the taught refusal), VT-30 (`[PROVISIONAL]`, the shape
    the bridge lands on and must not imply is frozen) and CF-7 (`[FROZEN]`, whose *mirror*
    DT-6 ships). Cited by id; lines taken from `§ Anchors`.
  - `.redkiln/config.yaml` — `affected_gate` (`:40`), `reachability_static` (`:48`),
    `integration_scoped` (`:55`), and `support_initiative` (`:5`) as a routing target.
  - `.kb/decisions/0006-bare-name-to-the-typed-layer.md` — the Accepted atom that makes
    `use happenstance::{…}` the taught vocabulary (UX-011, IQ-9, anti-pattern 13). Every
    heading and every probe in the record uses that vocabulary or seeds the wrong word into
    five downstream stories.
- **Renders surfaces**: **none rendered; three constrained.** This story ships no page, so no
  surface id from `_design.md:39-72` is rendered here. It *fixes binding values* on
  `crate-root-encounter`, `opening-encounter` and `conceptual-bridge` — their `##` heading
  texts and emitted fragment ids — which the stories that render them must use verbatim.
  `worked-example-handoff` is touched only by the citation contract's per-story consumption
  map. Claiming a rendered surface here would be claiming a page that does not exist.
- **Conformance rule(s)**: **none, and it is not adapter-observable.** No port, no store, no
  fixture, no `suite.rs` rule. The DT-6 probe *reads* CF-7 as the reason its wrong side is
  the mirror rather than the copy, but it neither adds nor amends a rule; naming one would be
  decorative by CLAUDE.md's own test. What observes this story is the record's transcripts
  plus the tier-5 reviewer walk (`_decomposition.md:475-513`).
- **Clause(s)**: **none discharged, none amended.** The initiative is additive and discharges
  no clause (`project.md:155-157`). ES-25, VT-30 and CF-7 are cited and their `[FROZEN]` /
  `[PROVISIONAL]` markers recorded; changing one would take a new ADR and nothing here comes
  near it.
- **Advances DoD scenario**: project Definition-of-done **item 1** — "`_design.md` is signed
  off carrying resolutions for DT-1, DT-4, DT-5 and DT-6" (`project.md:281-285`) — which this
  story completes by turning a signed file into a certified, consumable one. Toward the
  initiative's own scenarios it advances DoD **3** (`initiative.md:421-424`) and DoD **4**
  (`:425-428`) without reaching either: both are `boundary-refusal-encounter`'s and
  `boundary-falsification-drill`'s to run, and this story is what makes the shape they run
  fixed before they start.

## PR boundary

```
.bklg/docs-that-teach/application-author-path/tension-resolutions/**
```

That fence is the whole of it, and it is honestly narrow rather than defensively narrow: this
story's deliverable is a record, and the record's only legitimate home is the story folder.
`redkiln verify --grain story` reads the first fenced block above and fails on any file
changed outside it. If a probe needs a scratch crate to compile against the merged tree, it is
built outside the repository (or under an untracked path) and only its **transcript** is
committed — a probe binary that lands in the tree is a page nobody reviewed.

**In this PR**

- `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` — the
  record, with the sections named in the Behavior table below.
- The transcripts the probes produce, inline in that record: the DT-6 fence's compile-and-run
  output, the DT-4 step-3 line/column measurement, and the fragment-id read-off.
- `spec.md` (this file) and the `_ledger.md` the second pass authors.

**Explicitly not in this PR**

- Any page, fence, output block, mapping table, step header block or answered-need line.
  Authoring is gated on this story *and* on `merge-forward-preflight` (`_design.md:1033-1036`).
- Any edit to `_design.md`. Its sign-off row is a human's. Contradictions are recorded beside
  it and routed, never folded into it — the failure mode the design gate rejected this
  project for once (`_design.md:1041-1048`), and the one the slice-mate's own AC-004 guards
  with a byte-identity check.
- Any edit to `crates/**`, `examples/**`, `spec/SPECIFICATION.md`, `xtask/**` or `docs/**`.
  The crate-root rewrite is `boundary-refusal-encounter`'s; the `overview.md` extraction is
  `surface-course-subscriptions`'s; the pinned tree and the allowance list are HS-P0020's.
- Re-deciding any of the four tensions, or writing an exemption. DT-6's zero-allowance
  commitment means an unwritable fence is a **reopen**, escalated to the sign-off owner, not
  a locally-granted exception (`_design.md:1055-1058`).
- Repairing the stale line citations inside `project.md`, `_grounding.md`, `_storymap.md` and
  `_decomposition.md`. Those belong to stages already passed; the slice-mate's `§ Anchors`
  supersedes them and the closeout's reference reconciliation repairs them
  (`_design.md:736-741`, HS-P0025).

**Merge DoD one-liner** — the record exists at a stable path, each of the three resolutions
carries a certification with a transcribed probe that could have failed, every `##` heading
the design names has a final title and an emitted fragment id fixed once, and every sign-off
condition and merge disposition carries a per-resolution status of `holds`, `reconciled` or
`raised as reopen condition` — with `_design.md` byte-identical.

The implementer MAY also touch the composition-root / wiring files named in the Integration
contract to mount this slice; that is not scope drift. Here that permission is deliberately
narrow, because the mount point is a signed-off artifact: it extends to *reading* every file
named under **Wires into** and to committing probe transcripts, and to nothing that edits
`_design.md`.

## Behavior and interfaces

The record this story authors is
`.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md`. Its
section names — `§ DT-1`, `§ DT-4`, `§ DT-5+DT-6`, `§ Anchor table`, `§ Consumption map`,
`§ Conditions and dispositions` — are fixed here so the ledger's evidence column can point at
a path *and* a heading rather than at a promise. The filename is this spec's choice, matching
the `_baseline.md` companion beside it in the slice; the leading underscore marks it a
companion rather than a stage artifact.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Each resolution is certified, and a certification names the probe that could have failed** | Three certifications, one per decision (DT-5 and DT-6 are one). A certification states: the decision as `_design.md` records it, the probe run against the merged tree, the transcript, and a verdict of `holds` / `reconciled, with the reconciliation stated` / `raised as reopen condition`. A certification with no runnable probe is recorded as such with its reason — the tier-5 rows in `_decomposition.md:515-541` are reviewer checks by design, and pretending otherwise would be the decorative-check failure. | `_design.md:76-262`; `_decomposition.md:475-513` |
| **DT-1 certification: one decision, one location, one prior named once** | Confirms `_design.md:81-120` records a single resolution with reasoning and the evidence considered (`interaction-patterns.md:547-566`), that option (c) was treated as a real answer rather than a failure to choose (DR-06, `project.md:191-193`), and that the one reader-facing location is a **heading on `conceptual-bridge`**, not this file — `_design.md` is the decision's provenance; the heading is its single reader-facing statement (`:108-114`). Probe: the words anti-pattern 6 forbids ("aggregate", "your aggregates", "one stream per entity", "which stream") are searched for across every surface this project will author, and the expected result is zero outside the bridge's one section — recorded now as the baseline the later audit runs against. | `_design.md:81-120`, `:852-854`; `project.md:232-235`; `_decomposition.md:317-378` (UX-009) |
| **DT-4 certification: the mitigation is the decision, and it must survive the budget** | Confirms the disclosure shape (staged, minimal-first, three steps) *and* that the named failure mode is answered by all three structural mitigations, not by a warning sentence (`_design.md:140-159`). Probe: step 3's program — the binding case at roughly 20 lines — is written against the **merged** API and measured for rendered line count against the 24-line step budget and widest line against the 68-column budget (`_design.md:552-574`). Finding F-4 is the precedent that makes this non-decorative: the crate-root program measured 31 lines against a 24-line ceiling and forced an exemption. If a step program cannot be both complete-and-runnable and inside budget, DT-4's own premise is in tension with the density budget and the tension is raised, not split the difference. | `_design.md:122-168`, `:552-574`, `:1015`; `project.md:236-238`; `_decomposition.md:317-378` (UX-005, IQ-3) |
| **DT-5+DT-6 certification: the joint resolution, and the fence that could fail** | Confirms the two are recorded as one resolution covering both whether the shift is drawn and how honest the old side is (AC-003's own wording), that no diagram ships and AC-013 is discharged explicitly rather than by silence, and that the mapping table's fixed three-column shape and the four fixed step names (*tag, query, fold, guard*) are stated. Probe, and the load-bearing one: the wrong-side fence from `_design.md:373-384` — a guard tagged to the entity the command writes rather than to the invariant it holds — is compiled and run against the merged API, and `assert!(accepted.is_ok())` must **pass**. If it refuses, DT-6's construction is backward for the second time and the design is reopened, not patched; that exact error was found once already at the design gate as finding F-6. | `_design.md:170-262`, `:373-384`, `:1000`, `:1017`; `spec/SPECIFICATION.md:7266` (CF-7, `[FROZEN]`); `project.md:239-241` |
| **DT-6's zero-allowance commitment is verified, not restated** | The record states that this project puts **zero** entries on HS-P0020's enumerated allowance list and ships zero uncompiled fences, and that the consequence is a reopen rather than an exemption. UX-015 (`_decomposition.md:317-378`) is therefore recorded as **vacuously satisfied with its reason**, not omitted — an obligation dropped because it did not fire is indistinguishable from one forgotten. | `_design.md:206-208`, `:1055-1058`; `_decomposition.md:317-378` (UX-015) |
| **The anchor table fixes every `##` heading once, with its emitted fragment id** | One row per `##` heading `_design.md`'s `## Composition` (`:408-504`) names, across all four surfaces: final heading text, character count, the surface's applicable budget, and the fragment id the renderer actually emits. Slugging is **read off a render**, not predicted — `_design.md:110-112` verifies rustdoc's own behaviour by example (`id="using-it-today"`), and where the renderer is HS-P0020's markdown one the id is recorded as provisional against that substrate gap rather than guessed. This table is what `boundary-refusal-encounter` links against before the bridge that renders the target exists. | `_design.md:408-504`, `:108-114`; `_storymap.md:56`; `_decomposition.md:317-378` (UX-009, IQ-4) |
| **The retitles the design mandates are supplied here, once** | `_design.md:575-585` states eleven of sixteen named headings exceed the 22-character budget and that the retitles are structural. It does not supply them. This story does, in the anchor table, for every heading whose surface the budget actually binds — measured examples: `## Watch a boundary refuse` (23) on `crate-root-encounter`, `## Where your streams went` (23) on `conceptual-bridge`, `## A condition that refuses` as step 3's anchor. Supplying a mandated value is executing the design; changing which surfaces the budget binds is not, and that half goes to the reconciliation row below. | `_design.md:575-585`, `:693`, `:845-851`; slice-mate `§ Composition baseline` |
| **The budget's scope is reconciled against sign-off condition 2, and the reconciliation is stated** | The 22-character budget, anti-pattern 5 (sidebar TOC ellipsis) and the `Long label` state all derive from rustdoc's 200px sidebar. Sign-off condition 2 records that only `crate-root-encounter` is rustdoc-framed and the three step-page surfaces render as markdown (`_design.md:1039`, `:1018`). The record states, per surface, whether the budget binds and why — with the consequence for anti-pattern 5, which on a markdown surface is a check that can never fire and is therefore decorative by CLAUDE.md's own test unless it is scoped. It records the scoping; it does not relax a number, which is what the design gate rejected. | `_design.md:575-585`, `:693`, `:845-851`, `:1039`; CLAUDE.md, "A rule that no adapter can fail is decorative" |
| **The consumption map says which sections each downstream story loads** | One row per remaining story in this project — `boundary-refusal-encounter`, `boundary-falsification-drill`, `invariant-to-appendcondition-bridge`, `surface-course-subscriptions`, `fence-inventory-and-clause-audit`, `answered-need-and-anchor-review` — naming the `_design.md` sections it is bound by, the anchor-table rows it must render or link, and the one citation string it uses for the DT-1 decision. This is what turns AC-009's "each page that relies on it cites the one place" into a check a reviewer runs against a table. | `_storymap.md:56-62`; `project.md:260-262`; `_decomposition.md:317-378` (UX-009) |
| **Every sign-off condition and merge disposition carries a per-resolution status** | Two conditions (`_design.md:1039`) and the four dispositions the slice-mate records, each mapped to the resolutions it touches with a status of `holds`, `reconciled` or `raised as reopen condition`. The `tokio` disposition is the worked example of the discipline: its row status changes from `added` to `unchanged`, and the *reason* the design gave — a fence must execute, not merely type-check (AC-004, IQ-7) — is untouched and still binding. A disposition is three fields, not a paragraph: what the design says, what the tree says, and the status. | slice-mate `§ Dispositions`; `_design.md:284-306`, `:1039`; `project.md:300-302` |
| **A falsified resolution escalates; it does not get absorbed** | If any probe fails, the record states the failure, names the resolution it falsifies, and routes it to the sign-off owner as a condition on the story that would have consumed it — the same mechanism the slice-mate uses. It does not write an exemption, does not relax a budget, and does not edit `_design.md`. Nothing downstream starts on a resolution recorded as falsified. | `_design.md:1041-1048`, `:1055-1058`; `project.md:300-302`; `.redkiln/config.yaml:5` |
| **Zero teaching content ships** | No fence lands on any surface, no answered-need line, no mapping table, no step header block, no control outside the medium's own chrome (anti-pattern 8). Confirmed by the PR boundary fence holding under `redkiln verify --grain story` and by the authored diff being confined to this story's folder. The probes' source exists only as transcripts inside the record. | `_storymap.md:56`; `_design.md:858-860`; `.redkiln/config.yaml:40` |
| **The record's own vocabulary is the taught vocabulary** | `happenstance`, never `happenstance_core`, wherever the record names the crate a reader installs; every probe fence imports from `happenstance` (ADR-0006, UX-011, IQ-9, anti-pattern 13). A record that seeds the wrong word feeds it to six downstream stories at once. | `.kb/decisions/0006-bare-name-to-the-typed-layer.md`; `_design.md:871-874`; `_decomposition.md:317-378` (UX-011) |

**Interfaces, explicitly.** This story defines, changes and consumes **no Rust interface**.
It *reads* the merged public surface — `EventStore`, `Query`, `QueryItem`, `Tags`,
`AppendCondition`, `Guard`, `read_decision_model`, `AppendError::ConditionViolated` — as the
vocabulary its probes must compile against, and it binds `EventStore` rather than
`SendEventStore` in every probe, per CLAUDE.md binding constraint 4 and `_design.md:776-780`:
a fence that bound the `Send` flavour would teach the application author the bound that closes
off `wasm32`. Its actual interface is three artifacts and one guarantee: the record, the anchor
table, the consumption map, and the guarantee that the four pages downstream are written
against resolutions that were tested rather than merely approved.

## Data and migrations

**N/A — no schema, no store, no data.** Nothing in this story reads or writes an event store,
a projection store or a checkpoint in the tree. The DT-6 probe constructs a `MemoryEventStore`
in a scratch program to observe an append being accepted, but that program is never committed
and the store is in-memory and per-run; no fixture, no migration, no persisted state. No
`Cargo.toml`, feature flag or dependency is added or removed by an authored edit — the merged
`[dev-dependencies]` table arrives by the slice-mate's parentage, and dev-dependencies are not
resolved by downstream consumers (`_design.md:761`).

The one migration-shaped thing here is **anchor migration**, and it is deliberately performed
before there is anything to migrate. Fragment ids are the only identifiers this project mints,
and unlike clause ids — which are stable and never renumbered (`spec/SPECIFICATION.md:280`) —
a fragment id changes whenever its heading text changes. IQ-4 already requires anchors to be
stable and non-numbered so that inserting a step breaks no inbound link
(`_decomposition.md:229-315`). Fixing every heading and its emitted id *here*, before the first
page renders one, is what makes that requirement cheap: the migration cost of the retitles
`_design.md:575-585` mandates is zero today and grows with every page authored against an
unfixed title.

## Acceptance criteria

Five criteria, framed from the intent of Persona 1 — the application author whose stated fear
is "a mental model that looks right, compiles, runs, and is quietly wrong"
(`.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md:99-105`) — even though
this PR ships them no page. That reader is downstream of every row below by exactly one hop:
each criterion is discharged in
`.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md`, and the
reader meets it as the shape of the four pages written against that file. A criterion phrased
as "the record contains a section" would be satisfiable by a heading; every row below names the
probe that could have failed.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | GIVEN Persona 1 arrives holding a cross-entity invariant they cannot place, and the initiative's bar is that the prior model is named "once, consistently, rather than differently on each page" (`project.md:194-197`), WHEN the four page stories are authored against this record, THEN `§ DT-1` certifies the resolution as `_design.md:81-120` records it — option (c), invariant-first, bounded by a **single located use** of the stream-per-entity prior at the seam where the reflex question *which stream does this go in?* arises, decided on the EventStoreDB frequency evidence (`interaction-patterns.md:547-566`) and treating "explicitly none" as a real answer (DR-06, `project.md:191-193`) — names the one reader-facing home as a **heading on `conceptual-bridge`**, not this file, and records the baseline for anti-pattern 6: the words "aggregate", "your aggregates", "one stream per entity" and "which stream" appear on no other surface. | Reviewer walk of `_resolutions.md § DT-1` against `_design.md:81-120`, `:108-114` and `:852-854`, recorded as project DoD item 1's observation (`project.md:281-284`); probe transcribed into `§ DT-1` — `rg -n -i "aggregate|your aggregates|one stream per entity|which stream" crates/happenstance/src/lib.rs docs/ examples/course-subscriptions/src/main.rs` run on the merged tree, output pasted, expected zero on every surface this project will author; a cold read by the `boundary-refusal-encounter` implementer confirming they can cite the anchor without opening `_design.md`. |
| AC-002 | GIVEN a reader who lands on step 3 cold from a search result — the literature's named failure mode for staged disclosure (`interaction-patterns.md:161-166`) — WHEN `boundary-refusal-encounter` is authored against this record, THEN `§ DT-4` certifies the disclosure shape *with its three structural mitigations intact and not collapsed to a warning sentence*: no step is a fragment (cumulative in teaching, never in execution), a **two-line in-body step header block** carries the mid-sequence signal because rustdoc's sidebar TOC is removed entirely below 700px (`_design.md:394`, `:521`), and the payoff sits in step 3 so cold arrival is the best available landing — and the certification carries a **measured** verdict on whether step 3's program is simultaneously "a complete runnable program" and inside the 24-line / 68-column step budget, with the seven-element per-step composition (`_design.md:454-471`, `:591`) stated as the order the page must render. | Probe transcribed into `§ DT-4`: step 3's program written against the **merged** `crates/happenstance/src/lib.rs` API, rendered via `cargo doc -p happenstance --no-deps` (`_design.md:1025-1027`), its rendered line count read off `target/doc/happenstance/index.html` against the 24-line step ceiling and its widest line against 68 columns (`_design.md:552-574`); finding F-4's precedent (31 lines against a 24-line ceiling) cited as the reason the measurement is non-decorative; a verdict of `holds` / `reconciled` / `raised as reopen condition` per `_design.md:1055-1058`. Reviewer walk recorded as DoD item 1 (tier 5, `_decomposition.md:523`). |
| AC-003 | GIVEN the hazard worth teaching is the **quiet** one — a guard that compiles, runs, and accepts an append it should have refused — and Persona 1's fear is exactly that shape, WHEN `invariant-to-appendcondition-bridge` is authored against this record, THEN `§ DT-5+DT-6` certifies DT-5 and DT-6 as **one** resolution: no diagram ships and AC-013 is discharged **explicitly, by the reviewer recording that it went that way**, not vacuously by silence (`_storymap.md:120`); the narration is four fixed names — *tag, query, fold, guard* — with a three-column mapping table in fixed column order and never four columns (`_design.md:592-595`); and the wrong side is **one compiled, executed fence** differing from the correct one by a single expression, a guard **tagged to what the command writes rather than to the invariant it holds**, whose `assert!(accepted.is_ok())` must **pass** — the mirror of CF-7 (`spec/SPECIFICATION.md:7266`, `[FROZEN]`), which names the *broadening* failure and would make that assertion fail. Zero entries go on HS-P0020's allowance list, and UX-015 is recorded as **vacuously satisfied with its reason** rather than omitted. | Probe transcribed into `§ DT-5+DT-6`: the fence from `_design.md:373-384` compiled and **run** against the merged public API in a scratch program outside the tree, its stdout and its assertion result pasted verbatim, binding `EventStore` and not `SendEventStore` (CLAUDE.md constraint 4, `_design.md:776-780`) and importing from `happenstance` (ADR-0006). A refusal is a **falsification**: finding F-6 (`_design.md:1000`, `:1017`) recorded the same construction backward once already, so the response is `raised as reopen condition` to the sign-off owner, never a patch. Reviewer walk against `_design.md:170-262` recorded as DoD item 1 (tier 5, `_decomposition.md:524`). |
| AC-004 | GIVEN a page author mid-authoring needs the exact string `#where-your-streams-went` and must not decide it a second time — the defect BR-07 and DR-07 exist to prevent (`project.md:191-197`) — WHEN any of the six downstream stories opens this record, THEN `§ Anchor table` carries one row per `##` heading `_design.md`'s `## Composition` names across all four surfaces (`:408-504`) with **final heading text, character count, the surface's applicable budget, and the fragment id the renderer actually emits** — ids **read off a render**, never predicted, and marked provisional where the renderer is HS-P0020's markdown one; the retitles `_design.md:575-585` mandates but does not supply are supplied here for every heading whose surface the 22-character budget binds (`## Watch a boundary refuse` at 23, `## Where your streams went` at 23); the budget's scope is **reconciled** against sign-off condition 2 (`_design.md:1039`) per surface with the consequence for anti-pattern 5 stated; and `§ Consumption map` names, per downstream story, the `_design.md` sections binding it, the anchor rows it renders or links, and the one citation string it uses for DT-1. | `cargo doc -p happenstance --no-deps`, then the emitted `id="…"` attributes read out of `target/doc/happenstance/index.html` and pasted into `§ Anchor table` (the method `_design.md:110-112` already uses by example, `id="using-it-today"`); every count re-derived rather than asserted; `cargo xtask spec-trace` and `cargo xtask lints` green (`.redkiln/config.yaml:48`); reviewer walk confirming one row per named `##` and one map row per remaining story (`_storymap.md:56-62`), recorded as the tier-5 check `_decomposition.md:530` assigns to AC-009's mechanism. |
| AC-005 | GIVEN the design gate rejected this project once for absorbing findings its binding sections still contradicted (`_design.md:1041-1048`), and project DoD item 9 requires that nothing found is absorbed silently (`project.md:300-302`), WHEN this story's checkpoint is cut, THEN `§ Conditions and dispositions` carries a per-resolution status of `holds`, `reconciled` (with the reconciliation stated) or `raised as reopen condition` for **both** sign-off conditions (`_design.md:1039`) and **all four** merge dispositions from the slice-mate's `§ Dispositions` — each as three fields (what the design says, what the tree says, the status), each preserving the design's original *reason* even where the row's status changes, each escalation routed to a real item (HS-P0020 / HS-P0023 / HS-P0024 / the `support` initiative, `.redkiln/config.yaml:5`) — while `_design.md` is **byte-identical** to its signed-off state, the authored diff is confined to this story's folder, and **zero teaching content** ships: no fence on any surface, no answered-need line, no mapping table, no step header block, no control outside the medium's own chrome. | `git diff <base> HEAD -- .bklg/docs-that-teach/application-author-path/_design.md` empty; `redkiln verify --grain story` holding the PR boundary fence; `cargo xtask affected --base main` green at the checkpoint (`.redkiln/config.yaml:40`); reviewer walk of `§ Conditions and dispositions` against `_design.md:1039`, the slice-mate's `_baseline.md § Dispositions` and `project.md:300-302`, recorded as the DoD item 1 observation; every `raised as reopen condition` row cross-checked to exist as a named condition on the story that would have consumed it. |

**Coverage of the traced project ACs.** AC-001 (`project.md:232-235`) is discharged by AC-001
above, with AC-004 supplying the "citable location a later page can point at" half that DR-07
adds to it. AC-002 (`:236-238`) is discharged by AC-002 above; its "states how the chosen
shape's documented failure mode is handled" clause is why that row certifies the three
mitigations individually rather than the shape alone. AC-003 (`:239-241`) is discharged by
AC-003 above, including its "if any uncompiled code ships" branch — which resolves to *none
ships*, recorded as a commitment with a consequence rather than as an absence. AC-004 and AC-005
are the two obligations the sign-off left outstanding and the story map assigns nowhere else:
without them the traced three are approved but not consumable, which is the state this story
exists to end. No sixth criterion, because project AC-009 and AC-013 are owned downstream
(`_storymap.md:116`, `:120`) and claiming them here would double-own them.

## Interaction quality

This story renders **no surface** — it ships no page (`§ Integration contract`, "Renders
surfaces"). It does *fix binding values* on three surfaces others will render, so both families
apply, and both are carried by rows in the table above rather than by bullets here. This section
says **which AC row carries each invariant and how it is checked**; a bullet here with no AC id
would get no ledger row, would never be gated and would never be tested.

**STATE invariants.**

- **In place, not a context jump — AC-004.** The whole mechanism of `§ Consumption map` and
  `§ Anchor table` is that a page author resolves a fragment id, a heading text and a binding
  design section from *this record*, inside their own story, rather than jumping to
  `_design.md`, to a render, or to a page that does not exist yet. IQ-1's shape applied one
  level up. Verified by AC-004's cold-read check and by the map having a row per remaining
  story.
- **Non-occlusion — AC-005 and AC-003.** Two forms. The record must not occlude the human
  sign-off it sits beside: contradictions are recorded *next to* `_design.md`, never folded
  into it, and AC-005's byte-identity check is what makes that observable rather than promised.
  And the resolution AC-003 certifies is itself a non-occlusion rule (IQ-2.3): the wrong-side
  fence is marked at both ends, is never last code on the page, and never precedes the correct
  version — fixed here so no page re-decides it.
- **Preserved selection / prior state — AC-005.** The approver's row, its two conditions and
  the two calls made on their behalf (`_design.md:1039`) are the "selection" this story must
  not clear. They survive verbatim; sign-off condition 2 is *absorbed by reconciliation*, which
  is AC-004's budget-scope row, not by rewriting the condition.
- **Reversibility — AC-005 and AC-003.** The PR is one record in one folder, so backing it out
  is a single revert with nothing downstream depending on it yet — which is exactly why the
  anchor decisions are cheap today and expensive after four pages (`§ Data and migrations`).
  Reversibility of the *design* is AC-003's clause: a falsified probe reopens the design rather
  than buying an exemption (`_design.md:1055-1058`).
- **Keyboard reachability — n/a, recorded rather than dropped.** This story introduces no
  control on any surface. IQ-6's test is "enumerate the interactive affordances this project
  introduced" and the correct answer is none (`_decomposition.md:287-291`); anti-pattern 8
  forbids adding one. AC-005's "no control outside the medium's own chrome" clause is what keeps
  that true here instead of assumed.

**COMPOSITION invariants** (from `_design.md`, binding — `:10-12`, sign-off row `:1039`).

- **Presentation exists at all — AC-004.** The documentation analogue of an unstyled render
  passing every ARIA assertion is an anchor table full of *predicted* slugs: every string looks
  right, every citation resolves in review, and the first page to link one lands on the page top.
  AC-004 therefore forbids prediction — ids are read off `target/doc/happenstance/index.html`
  after a real `cargo doc` run, the same method `_design.md:110-112` uses by example, and where
  the renderer is HS-P0020's markdown one the id is recorded as *provisional against a named
  substrate gap* rather than guessed.
- **Composition and placement — AC-004 and AC-002.** AC-004 fixes heading text and order for
  `crate-root-encounter` (`_design.md:415-445` — the fence moves up, the roadmap moves down),
  `conceptual-bridge`'s seven-section reading order (`:473-491`, where the order is load-bearing:
  §6's wrong side strictly after §5, §7 so the last code is correct) and
  `worked-example-handoff`'s five (`:494-503`). AC-002 fixes the per-step order for
  `opening-encounter` (`:454-471`): heading, step header block, setup, fence, output block,
  citation — and the drill as a `###` under step 3.
- **Transience — AC-002, AC-003, AC-005.** AC-002 carries the load-bearing one: the step header
  block is **persistent, in-body** and not chrome, because chrome disappears at 700px
  (`_design.md:521`). AC-003 carries that the wrong-side contrast is **persistent, never
  opened-on-demand** (`:525`) and that hidden doctest lines are **binary in this medium** —
  absent from the DOM with no toggle that recovers them, therefore forbidden for any `Query`,
  `Tags`, `Guard`, `AppendCondition`, the append call or any assertion (`:524`), which is a
  constraint the DT-4 and DT-6 probe programs must satisfy as written. AC-005 carries that the
  answered-need line and every fence stay persistent by this story shipping none of either.
- **Density budget, with its real numbers — AC-002 and AC-004.** Fence width **68 columns**
  hard (72 is where `overflow-x` engages on the 696px fence at 1024×768); fence height **24
  rendered lines** on a step page, **32 on `crate-root-encounter` alone** (F-4's stated
  exemption); `##` heading length **22 characters** before the sidebar TOC truncates with an
  ellipsis; paragraph length **435 characters**; mapping table **three columns, never four**;
  per-step budget **seven elements** (`_design.md:532-620`). AC-002 measures step 3's program
  against the first two. AC-004 applies the third to every named heading and reconciles *which
  surfaces it binds*. Neither relaxes a number — relaxing one is what the design gate rejected.
- **Hierarchy — AC-001, AC-003, AC-004.** AC-001 preserves the DT-1 anchor as **secondary**,
  carried by heading level and by position ahead of the code (`_design.md:657-659`) — one
  section on one page, not a topic promoted onto every page. AC-003 preserves the deliberate
  inversion: the wrong-side contrast is *visually equal* to the correct fence and
  *hierarchically below* it, and the only channels that can carry "below" are position and
  framing, so it gets both (`:660-664`). AC-004's retitles must not disturb the crate root's
  primary/secondary/recessive ordering (`:632-641`), where the refusal is primary by position.
- **Named anti-patterns — AC-001 (6), AC-003 (1, 7, 14), AC-004 (5), AC-002 (9, 15), AC-005
  (the rest, recorded).** Anti-pattern 6 is AC-001's zero-occurrence baseline. Anti-pattern 1
  (a block labelled not-compiled) and 14 (a diagram) are what DT-6's zero-allowance and DT-5's
  no-diagram commitments make checkable; 7 is the wrong-block placement rule. Anti-pattern 5 is
  the one AC-004 must **scope**: on a markdown surface with no 200px sidebar it is a check that
  can never fire, and a check no page can fail is decorative by CLAUDE.md's own test — so AC-004
  records its scope rather than deleting it or leaving it universal. Anti-patterns 2, 3, 4, 8,
  10, 11, 12 concern authored page content and cannot fire in a PR that authors none; AC-005's
  zero-teaching-content clause is what keeps that true instead of assumed.

## Error conditions

| id | condition | required response |
| --- | --- | --- |
| EC-001 | The DT-6 wrong-side fence **refuses** — `assert!(accepted.is_ok())` fails — when compiled and run against the merged API. | **Falsification, not a bug.** Record the transcript in `§ DT-5+DT-6`, mark the resolution `raised as reopen condition`, and route it to the sign-off owner as a blocking condition on `invariant-to-appendcondition-bridge`. Do not adjust the fence until it passes, do not substitute CF-7's own broadening failure, and do not edit `_design.md`. This exact construction was backward once already (finding F-6, `_design.md:1000`, `:1017`); a second silent correction would make the design's record untrue twice. |
| EC-002 | The DT-6 fence cannot be **written at all** against the merged API — no expression expresses "tagged to what the command writes" as a single-expression delta. | Reopen, do not exempt. DT-6 commits to zero allowance-list entries precisely so this case escalates (`_design.md:1055-1058`). Record what was attempted, name the API shape that blocks it (`crates/happenstance-core/src/append.rs`, `crates/happenstance-core/src/query.rs`), route to the sign-off owner, and — if the blocker is a substrate gap rather than a design one — additionally to HS-P0020 per `project.md:300-302`. |
| EC-003 | Step 3's program cannot be both "a complete runnable program" and inside 24 rendered lines / 68 columns. | Record the measurement in `§ DT-4` and raise the tension; do **not** split the difference by hiding a line that carries a boundary construct (forbidden, `_design.md:524`), by marking the fence `no_run` (forbidden, `:878-882`), or by quietly granting a second exemption. The `## Density budget` yield order (`:601-617`) is the only permitted reduction path and it stops before the boundary. If it does not fit after that, DT-4's premise and the budget are in genuine tension and the sign-off owner decides. |
| EC-004 | A `##` heading cannot be brought under 22 characters without losing its meaning, on a surface where the budget binds. | Record the heading, its count, the alternatives tried and the meaning each lost, and raise it as a reopen condition rather than shipping a heading the sidebar truncates mid-phrase. Note that AC-004's scope reconciliation may legitimately establish that the budget does **not** bind that surface (sign-off condition 2) — that is a reconciliation with a stated reason, not a relaxation, and the two must not be confused. |
| EC-005 | The slice-mate's `_baseline.md` is missing a section this record depends on (`§ Anchors`, `§ Dispositions`, `§ Composition baseline`), or its dispositions do not number four. | **Stop.** This story blocks on `merge-forward-preflight` (`_storymap.md:56`) and re-running its archaeology here would produce a second, competing set of numbers — the precise defect its `§ Anchors` exists to prevent. Report the gap against that story rather than filling it. |
| EC-006 | An emitted fragment id cannot be read off a render because the surface's renderer does not exist yet (HS-P0020's markdown substrate, gap 1 and 2 at `_design.md:891-899`). | Record the id as **provisional against that named gap**, with the slugging rule assumed and the command that will re-derive it once the renderer is pinned, and route the dependency to HS-P0020. Do not guess silently: a guessed id that later moves is a broken inbound link on every page authored in between. |
| EC-007 | The implementer finds a contradiction inside `_design.md`'s binding sections while certifying. | Record it in `§ Conditions and dispositions` with its measurement and route it; leave `_design.md` byte-identical. Folding a finding into a signed-off file is what this project was rejected for at the design gate (`_design.md:1041-1048`), and the slice-mate's AC-004 already guards the same file with the same check. |
| EC-008 | A probe requires a scratch crate and the implementer is tempted to add it to the workspace to make `cargo` cooperate. | Build it outside the repository, or under an untracked path, and commit only the transcript. A probe crate inside the tree is a page nobody reviewed, would appear in `cargo xtask ci`'s feature powerset forever, and breaks the PR boundary fence under `redkiln verify --grain story`. |

## Non-functional

| id | requirement | why |
| --- | --- | --- |
| NF-001 | Every certification names a probe **that was run**, with its transcript, or states in the same row that no runnable probe exists and why. | A tier-5 story whose evidence is "the design says so" is the decorative check CLAUDE.md names. The testing brief scores these ACs at tier 5 *by mechanism*, not as permission to assert (`_decomposition.md:475-513`). |
| NF-002 | `_resolutions.md` is self-sufficient cold: a page author resolves the heading text, the fragment id and the binding sections for their own story from it alone, without opening `_design.md`. | The record replaces work, or it is a second thing to read. Progressive disclosure only pays if the disclosed layer is complete at the point of disclosure — and `_design.md` is 1,065 lines. |
| NF-003 | `§ Anchor table` and `§ Consumption map` stay **tables**, one row per heading and one row per story. | Six implementers will open them mid-authoring under time pressure. Prose that must be read to be searched is a context jump wearing a different hat. |
| NF-004 | Every anchor into `spec/SPECIFICATION.md` is stated **id-first, line-second**, taken from the slice-mate's `§ Anchors`. | Ids are stable and never renumbered (`spec/SPECIFICATION.md:280`); lines are not, and this project already has a corpus of line citations that rotted once. |
| NF-005 | The record's own vocabulary is the taught vocabulary: `happenstance`, never `happenstance_core`, wherever it names the crate a reader installs, and every probe fence imports from `happenstance` and binds `EventStore` rather than `SendEventStore`. | ADR-0006 gave the bare name to the typed layer for exactly this reader; CLAUDE.md constraint 4 is why the weaker bound is the correct one. A record that seeds the wrong word feeds it to six downstream stories at once. |
| NF-006 | No probe artifact, scratch crate, `Cargo.toml` edit, feature flag or dependency lands in the tree. | The gate's feature powerset and `cargo package --list` assertions are workspace-wide; a scratch crate added for a documentation probe is a permanent cost paid for a one-time measurement. |
| NF-007 | Every escalation names a **real item id** and the story it conditions. | "Routed" without a target is absorption with better manners, and project DoD item 9 exists because that is the failure mode this initiative keeps finding in other people's documentation. |

## Implementation notes (non-prescriptive)

- **Order that keeps every failure attributable.** Read the slice-mate's `_baseline.md` end to
  end first; then run the DT-6 probe (it is the one that can fail hardest and the one whose
  failure changes the shape of everything after it); then the DT-4 measurement; then the anchor
  table off a render; then the dispositions; then write the record. Writing the record first
  turns every probe into a formality performed against a conclusion.
- **The DT-6 probe is the load-bearing half hour of this story.** Take the fence verbatim from
  `_design.md:373-384`, correct only what F-5 already corrected in spelling
  (`read_decision_model(&store, &seats)`, `AppendCondition::new(seats).after_opt(upto)` —
  `EventStore::head` takes no argument and `Guard` is `#[non_exhaustive]` with no constructor,
  `_design.md:999`, `:1016`), and change nothing else. If it does not behave as the design says,
  that is the finding, and the finding is worth more than a working fence.
- **Read slugs, do not derive them.** `cargo doc -p happenstance --no-deps` then grep
  `target/doc/happenstance/index.html` for `id="`. Rustdoc's own behaviour is already verified
  by example at `_design.md:110-112`; anything not verified that way is recorded as provisional
  with the gap named, not silently normalised.
- **A disposition is three fields, not a paragraph** — what the design says, what the tree says,
  and one of `holds` / `reconciled` / `raised as reopen condition`. Anything longer starts
  re-arguing a signed-off decision.
- **Count characters mechanically.** `## Watch a boundary refuse` and `## Where your streams
  went` are both 23 characters of heading text (excluding the `## ` marker), one over budget —
  measured here so the implementer starts from a number rather than a guess, and re-measures
  rather than trusting it.
- **The `_resolutions.md` filename is this spec's choice**, not a redkiln convention. It lives
  in the story folder because that is the only place this story may author, and the leading
  underscore marks it a companion rather than a stage artifact — matching `_baseline.md` beside
  it in the slice and `_ledger.md` beside it here.
- **When a heading retitle and a fragment id disagree with a section in `## Composition`, the
  design wins on *intent* and this record wins on *string*.** The design mandated the retitles
  and declined to supply them (`_design.md:582-584`); supplying them is execution. Changing what
  a section is *for* would be re-opening, and is not available here.

## Tests and CI (merge gate)

Grounded in the project testing brief's five tiers (`_decomposition.md:480-486`), which this
story rides rather than extends. Tiers not exercised are stated n/a with a reason rather than
omitted, per that brief's own rule (`:517-518`).

| tier | command / path | proves |
| --- | --- | --- |
| 5 — design / review sign-off (this story's assigned tier, `_decomposition.md:522-524`) | Reviewer walk of `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md` against `_design.md:76-262` and `project.md:281-284` (DoD item 1) | AC-001, AC-002, AC-003 — each resolution is certified with its probe and its verdict, and AC-013's no-diagram outcome is recorded explicitly rather than left to silence. |
| 3 — executed (borrowed, out-of-tree) | The DT-6 wrong-side fence compiled and **run** against the merged API in a scratch program; stdout and assertion result transcribed into `§ DT-5+DT-6` | AC-003 — the resolution's central claim ("compiles, runs and passes while the append is accepted") is a result, not a prediction. Tier 3's usual home is a doctest; here the code deliberately does not land in the tree, so the transcript is the artifact. |
| 1 — structural, render | `cargo doc -p happenstance --no-deps`, then `id="…"` and rendered line counts read off `target/doc/happenstance/index.html` | AC-002, AC-004 — fragment ids and the density measurements come off a real render rather than from arithmetic; `_design.md:1025-1027` names this as the cheapest instrument for exactly these numbers. |
| 1 — structural, citations | `cargo xtask spec-trace` — REQUIRED step (`xtask/src/main.rs`) | AC-001, AC-004 — every clause id this record hands downstream (ES-25, VT-30, CF-7) resolves on the merged text. |
| static reachability (redkiln `reachability_static`) | `cargo xtask lints && cargo xtask spec-trace` (`.redkiln/config.yaml:48`) | AC-004 — the file-reading lints and the trace both run on a story whose deliverable maps to no package, which is exactly this one. |
| story grain (redkiln `affected_gate`) | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | AC-005 — the checkpoint does not break what its diff could reach. It will find little, which is the point: this story's evidence is its transcripts, not this command. |
| integration grain, non-terminal (redkiln `integration_scoped`) | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | AC-005 — the project bar, already green on the slice-mate's merge commit; re-run here only to prove this story's authored diff changed nothing about it. |
| 2 — compiled fence | **n/a.** No fence is authored onto any surface, and HS-P0020's compiled-fence step does not exist yet (`_design.md:891-908`, gaps 1 and 4). The DT-6 probe compiles outside the tree by design (`§ PR boundary`). | — |
| 4 — falsification drill | **n/a.** DoD item 3's drill belongs to `boundary-falsification-drill` and depends on a page that does not exist. Running it here would prove nothing. | — |
| ledger gate | `redkiln verify --grain story` over `_ledger.md`, and the PR boundary fence | all five — every AC present, satisfied, and carrying non-placeholder evidence before `implement → report`; and the authored diff confined to this story's folder. |

## Risks and coupling (PR-scoped)

| risk | likelihood / impact | mitigation in this PR |
| --- | --- | --- |
| The implementer treats a signed-off `_design.md` as evidence and writes a record of assertions with no probe run — the tier-5 rows read as permission. | High / High | NF-001 and every AC's verification cell name a transcript. The Executive summary states the honest delta up front: the sign-off already happened, so a record that only restates it delivers nothing. |
| The DT-6 fence refuses, falsifying the resolution late, after page stories have started. | Medium / High | The probe runs **in this story**, which is why the story exists in the preflight slice and why nothing downstream may start on a resolution recorded as falsified. EC-001 fixes the response before the temptation arrives. |
| The implementer, meeting a contradiction, fixes `_design.md` "while they are in there". | Medium / High | AC-005's byte-identity check, EC-007, and the PR boundary's explicit exclusion. The design gate rejected this project once for absorbing findings (`_design.md:1041-1048`); the slice-mate guards the same file the same way. |
| Fragment ids are predicted rather than read, and every page authored in between links to the page top. | Medium / High | AC-004 forbids prediction and EC-006 defines the provisional-with-a-named-gap form for surfaces whose renderer does not exist yet. |
| The 22-character budget's scope is *relaxed* rather than *reconciled*, quietly turning a design decision into an implementer's convenience. | Medium / High | AC-004 requires the reconciliation to be stated per surface with its reason and forbids changing the number; EC-004 separates "the budget does not bind this surface" (legitimate, sign-off condition 2) from "this heading is too long" (escalates). |
| A scratch probe crate lands in the workspace and becomes permanent. | Low / Medium | NF-006, EC-008 and the PR boundary paragraph: only transcripts are committed. |
| This record and the slice-mate's `_baseline.md` both carry composition numbers, and they drift. | Medium / Medium | Single source: `§ Composition baseline` is the slice-mate's and this record cites it rather than re-measuring, except for the two measurements it is scoped to take (step 3's program, and the heading counts the design left unsupplied). |
| Downstream stories consume `_design.md` directly and never open this record, so the anchor decision is re-derived anyway. | Medium / High | `§ Consumption map` names each story's rows explicitly, and `answered-need-and-anchor-review` (AC-009, AC-012) proves consumption by checking every page cites the one recorded location (`_storymap.md:62`, `:95-98`). |

## Dependencies

**Blocks on:** `merge-forward-preflight` (HS-S0183) — `depends_on: ["merge-forward-preflight"]`,
matching `_storymap.md:56` and the merge order at `:138-140`. The dependency is not procedural:
three of this story's five criteria consume that story's artifact directly. AC-002's and
AC-003's probes compile against the **merged** `crates/happenstance/src/lib.rs`; AC-004's
character counts and budget reconciliation are computed against `_baseline.md § Composition
baseline`; AC-005's dispositions are per-resolution statuses on the four contradictions
`_baseline.md § Dispositions` records. Starting before it produces measurements against a file
the merge replaces — the exact rework `_grounding.md` §1 and DR-13 exist to prevent.

**Unlocks:**

- `boundary-refusal-encounter` (HS-S0185) — blocks on this story and on the slice-mate
  (`_storymap.md:57`). It is the first page to *link* the DT-1 anchor before the page that
  renders it exists, so it consumes `§ Anchor table` and `§ DT-4` in full.
- `invariant-to-appendcondition-bridge` (HS-S0187) — blocks on this story (`:59`). It renders
  the `## Where your streams went` heading whose text and id this story fixes, and it ships the
  DT-5+DT-6 material AC-003 certifies, including the wrong-side fence.
- `answered-need-and-anchor-review` (HS-S0190) — blocks on this story (`:62`). Its whole check
  is that every page cites the one recorded location, which is `§ Consumption map` read back.
- Transitively, `boundary-falsification-drill`, `surface-course-subscriptions` and
  `fence-inventory-and-clause-audit` — none may be authored until `_design.md`'s first sign-off
  condition and AC-001 are both discharged (`_design.md:1033-1036`).

**Not a DAG edge, deliberately:** HS-P0020 (`checked-documentation-surface`). DT-6's
zero-allowance commitment removes the allowance-list coupling entirely (`_design.md:904-908`,
gap 4); what remains is a *routing target* for substrate gaps, not a blocker on this story.

## Anchors (progressive disclosure)

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/application-author-path/_design.md` | The mount point and the binding artifact. `## Pattern decision` (`:76-262`) is the text every certification quotes; the wrong-side fence to run is at `:373-384`; `## Composition` (`:408-504`) names every heading the anchor table must carry; the density budget with its real numbers is `:532-620`; the anti-patterns are `:831-885`; the sign-off row and its two conditions are `:1039`, and the reopen-not-exempt commitment is `:1055-1058`. | Before writing any certification — and never with an editor open on it. Read-only is a hard rule here (AC-005). | AC-001, AC-002, AC-003, AC-004, AC-005 |
| `.bklg/docs-that-teach/application-author-path/merge-forward-preflight/spec.md` | The slice-mate's spec, and through it this story's direct input: it fixes the path and the six sections of `_baseline.md` (`§ Clarifications resolved during spec`, item 2), which does not exist until that story lands. From that record, `§ Anchors` supplies the merged clause lines cited id-first, `§ Dispositions` supplies the four contradictions AC-005 must status, and `§ Composition baseline` supplies the re-measured heading, bracket, hidden-line and fence figures AC-004 computes against. | First, before any probe — read the spec now, the record the moment it exists. If a section is missing, EC-005 applies and this story stops rather than re-running the archaeology. | AC-002, AC-004, AC-005 |
| `crates/happenstance/src/lib.rs` | The merged crate root: the API surface the DT-4 and DT-6 probes compile against, and the render source whose emitted `id="…"` attributes AC-004 reads. 75 lines on this branch, 237 after the merge — reading it is the check that the probes are being written against the tree that will ship. | Immediately after the slice-mate lands, before writing a line of probe code. | AC-002, AC-003, AC-004 |
| `crates/happenstance-core/src/append.rs` | `AppendCondition` and `Guard` — `Guard` is `#[non_exhaustive]` with no constructor and `AppendCondition::new` takes a `Query`. Finding F-5 (`_design.md:999`) recorded that the design's first fence sketch called two things that do not exist; this file is what makes the probe's spelling verifiable instead of remembered. | While writing the DT-6 probe, at the moment of spelling the guard. | AC-003 |
| `crates/happenstance-core/src/store.rs` | `EventStore`, and `head` taking no argument (the other half of F-5). Also the reason CLAUDE.md constraint 4 binds: the probe binds `EventStore`, not `SendEventStore`, and importing both names into one module makes the call ambiguous. | Alongside `append.rs`, while writing the probe. | AC-003 |
| `spec/SPECIFICATION.md` | CF-7 at `:7266` (`[FROZEN]`) is the clause the DT-6 resolution is the **mirror** of, not a copy — the distinction the whole resolution turns on. ES-25 is the taught refusal; VT-30 is `[PROVISIONAL]` and must never be implied frozen. `:280` states that clause ids are stable and never renumbered, which is why NF-004 cites id-first. | When writing `§ DT-5+DT-6`'s framing, and whenever a clause is cited; open by id search, never by scrolling to a remembered line. | AC-001, AC-003 |
| `.bklg/docs-that-teach/application-author-path/_decomposition.md` | The UX brief (`:11-447`) carries the IQ invariants (`:229-315`) and UX-001…UX-015 (`:317-378`) — UX-009 is the anchor obligation, UX-015 the one recorded vacuously-with-reason, IQ-3 and IQ-4 the mid-sequence and anchor-stability rules. The testing brief (`:448-650`) carries the tier table (`:480-486`) and the AC-to-tier map that scores these three ACs at tier 5 (`:522-524`). | Before writing the record's obligations list, and before the Tests table is acted on. | AC-002, AC-004 |
| `.bklg/docs-that-teach/application-author-path/project.md` | AC-001, AC-002 and AC-003 verbatim (`:232-241`), DR-06/DR-07 on the single citable location (`:191-197`), the nine Definition-of-done items with item 1 (`:281-284`) and item 9's routing rule (`:300-302`). It is the source of both the obligation and the routing targets. | First, before opening `_design.md` — it is what the design was written to satisfy. | AC-001, AC-005 |
| `.bklg/docs-that-teach/application-author-path/_storymap.md` | This story's row and one-line slice (`:56`), the slice rationale (`:66-69`), the archetype note stating that every page story consumes this record (`:95-98`), the AC-to-story ownership table (`:108-121`) and the merge order (`:138-140`). It is what makes `§ Consumption map` a fixed list rather than a judgement call. | While writing `§ Consumption map`, and when confirming what unlocks on completion. | AC-004 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The evidence DT-1 and DT-4 were decided on: the stream-per-entity frequency argument (`:547-566`) that beat the DDD-aggregate anchor on statistics rather than taste, and staged disclosure's named mid-sequence failure mode (`:161-166`). A certification that cannot restate the evidence is a restatement of the conclusion. | While writing `§ DT-1` and `§ DT-4`, when stating what the decision was made against. | AC-001, AC-002 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | Persona 1's stated fear — "a mental model that looks right, compiles, runs, and is quietly wrong" (`:99-105`) — is the user intent every AC above is framed from, and the exact reason DT-6 teaches the quiet failure rather than the loud one. | Once, before framing the record; again if any AC's user-intent framing is questioned. | AC-001, AC-003 |
| `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | The Accepted decision atom behind the taught vocabulary. `use happenstance::{…}`, not `happenstance_core`, in the record's prose and in every probe fence (UX-011, IQ-9, anti-pattern 13). Accepted atoms are immutable and this one is not up for reinterpretation here. | Before choosing any crate name in the record or in a probe. | AC-003 |
| `.redkiln/config.yaml` | `affected_gate` (`:40`), `reachability_static` (`:48`) and `integration_scoped` (`:55`) are the commands redkiln runs at this story's and this project's grains whether or not anyone types them; `:5` names the `support` initiative that incidental bugs route to under DoD item 9. | Before the gate run, and again when routing anything found. | AC-005 |
| `.bklg/docs-that-teach/application-author-path/design/mock.html` | The instrument that produced findings F-1…F-7, deliberately **not** synced to the corrected figures (`_design.md:1020-1027`) — it still shows the pre-correction numbers because it is the evidence they rest on. Its `overflow-fence` frame carries a column ruler at 72/84/100, which is how the 68-column budget is read off a render rather than argued. | When a density measurement is in doubt, and never as the source of a current number. | AC-002, AC-004 |
| `.bklg/docs-that-teach/initiative.md` | The gold source. DoD scenarios 3 (`:421-424`) and 4 (`:425-428`) are what this story makes reachable without reaching, and initiative AC-08 is the "once, consistently" bar the anchor decision serves. | Once, before writing the record's opening paragraph, to keep it from drifting into a design summary. | AC-001, AC-004 |
| `.bklg/docs-that-teach/application-author-path/_grounding.md` | The measured evidence behind DR-13 and the stale-tree problem, and the record that a repo-wide `rg -n "aggregate" .kb` returns nothing (`:151-158`) — which is why DT-1 had no prior grounding to look up and why the frequency evidence had to come from outside. | If anyone questions why DT-1 needed resolving at all, or why the probes must run post-merge. | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the five ids the first pass decided** — AC-001 through AC-005. None
   added, none dropped. The ledger matches, one row per id.
2. **The record's path and sections are fixed here**:
   `.bklg/docs-that-teach/application-author-path/tension-resolutions/_resolutions.md`, with
   `§ DT-1`, `§ DT-4`, `§ DT-5+DT-6`, `§ Anchor table`, `§ Consumption map` and
   `§ Conditions and dispositions`. A ledger row needs a path *and* a heading to point at, so
   both are settled rather than left to the implementer.
3. **This story certifies; it does not re-decide.** The traced project ACs read as "`_design.md`
   records a resolution", and read literally that already happened on 2026-08-17. The delta is
   the three things the sign-off does not carry — an unabsorbed condition, mandated retitles it
   does not supply, and two claims the merged tree can now falsify. Every AC above is written
   against that delta, which is why none of them says "record a resolution".
4. **The four resolutions map to three certifications, not four.** DT-5 and DT-6 are one joint
   resolution by project AC-003's own wording (`project.md:239-241`) and by `_design.md:170-262`.
   Splitting them into two ACs would contradict the criterion this story traces to.
5. **The DT-6 probe compiles outside the tree and only its transcript is committed.** The PR
   boundary fence admits nothing under `crates/`, and a probe crate added to the workspace would
   be carried by the feature powerset and the packaging assertions forever. EC-008 states the
   response to the temptation.
6. **Composition invariants are carried as AC rows, not as prose.** Every invariant in
   `## Interaction quality` names the AC id that gates it. The two that would otherwise have gone
   unowned are the fragment ids being *read rather than predicted* (AC-004 — the documentation
   analogue of an unstyled render passing every assertion) and anti-pattern 5's *scope* (AC-004
   again — a check no page can fail is decorative by CLAUDE.md's own test, so it is scoped rather
   than deleted or left universal).
7. **Keyboard reachability is recorded n/a with its reason** rather than dropped: this story adds
   no control, IQ-6's correct answer is none, and anti-pattern 8 forbids adding one. Dropping it
   silently would leave a reader unable to tell whether it was considered.
8. **Testing tiers 2 and 4 are stated n/a with reasons**, per the testing brief's own rule
   (`_decomposition.md:517-518`). Claiming a compiled-fence tier this PR does not exercise would
   be the decorative-check failure CLAUDE.md names.
9. **A falsified probe is a first-class outcome, not a failure of the story.** EC-001 through
   EC-004 all resolve to *record and escalate*. The story is complete when every resolution
   carries a status, including `raised as reopen condition`; it is the downstream page stories
   that may not start on a falsified one.
