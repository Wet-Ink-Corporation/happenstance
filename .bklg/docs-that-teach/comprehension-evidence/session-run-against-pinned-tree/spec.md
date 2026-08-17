---
item: HS-S0165
stage: spec
created: 2026-08-17T13:16:19.552Z
updated: 2026-08-17T13:16:19.552Z
template_sig: 87bbf1d0
rendered_sig: 4d538568
---

# Spec — Run the session against the named assembled tree

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/docs-that-teach/initiative.md` |
| Initiative decomposition | `.bklg/docs-that-teach/_decomposition.md` |
| Project (the spine this story serves) | `.bklg/docs-that-teach/comprehension-evidence/project.md` |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/spec.md` |
| Briefs (ux + testing, one `##` each; no architecture, no deployment) | `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` |
| **Signed-off design — binding** | `.bklg/docs-that-teach/comprehension-evidence/_design.md` |
| Grounding (verified sibling-project state, verified anchors, one refuted anchor) | `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` |
| Story map (slice cut, merge order, coverage split) | `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` |
| This story's discover artifact | `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/discover.md` |
| Roadmap pointer | none. `RUNBOOK.md` sequences library phases, not documentation projects; this initiative's ordering is `_decomposition.md`'s dependency graph and `_storymap.md`'s merge order. |

`_design.md` is **binding and already signed off** (Ryan Britton, 2026-08-17, no
conditions). It records `hasSurface: false` explicitly rather than being skipped: this
project ships no `pub` item, no screen and no crate change. What it *does* durably
record — and what `dt9-and-fixed-protocol` writes into it before this story may run —
is the protocol: DT-9's resolved persona, the scenario, the narration mode, the
severity scale and the logger-disqualifying criteria. **This story does not re-decide
any of them.** Where this spec and `_design.md` appear to disagree after that story
lands, `_design.md` wins and this spec is wrong.

## One-line PR slice

Run the comprehension session once — one real non-insider reader, the post-merge
assembled tree, the protocol as `_design.md` fixed it — and land the filled friction
log: the tree named as a resolvable commit, what the reader searched, opened, ran and
followed captured *while it happened*, severity marked inline as a text token,
facilitator interventions timestamped as entries, keyboard reach recorded either way,
and abandonment accepted as a valid end state.

## Executive summary

**What this PR lands.** Content, into a shape that already exists. `friction-log-skeleton`
lands the scaffold — the six sections, the stumble-id convention, the severity column,
the disposition slot with its revision block. This story fills it by running the
session and writing what happened, and adds exactly two things the scaffold cannot
carry until a session has occurred: the **session date** and the **tree identifier**.

**The delta.** Before: a scaffold with empty sections and a `_design.md` protocol,
committed at a date that predates any candidate being approached. After: the same file
with a resolvable commit or merge SHA in its tree slot, a filled context block naming
the reader's platform, toolchain, browser and any assistive technology, and an
append-only chronological section of occurrence-ordered entries, each stumble carrying
a text-token severity mark on the named scale and an empty disposition slot. The
dispositions are **not** this story's — `disposition-every-stumble` fills them, and it
is blocked on this.

**Why this is a vertical slice and not "write the log".** Three things make it one.
The reader is real and is not the author, which is the mechanism the whole project
rests on, so nothing here is simulatable — the testing brief says a stand-in reader
"is not a cheaper version of this instrument; it is a different, useless one". The
tree is a fixture that must be *assembled first*: run it a week early and the log
measures HS-P0022 and HS-P0023's absence rather than their teaching. And the record's
auditable properties — occurrence-ordered stable ids, inline severity, timestamped
interventions — are only true if they are true *while the session runs*; every one of
them is unrecoverable by 18:00 the same day.

**What this story is not.** It does not disposition, route, escalate, scope the claim,
write the hand-off note, or fix a single line of documentation. Each of those is a
named sibling story. See `## PR boundary`.

## Context pack

The load-bearing decisions, distilled. Everything deeper sits behind the signposted
anchors and is opened just-in-time, not read up front.

### The persona-journey slice this realizes

**U1, the recruited reader** — whichever of Persona 1 (application author), Persona 2
(adapter author) or Persona 3 (evaluator) `_design.md`'s DT-9 resolution selected — is
here to get *their own* job done: model a cross-entity boundary, implement the storage
contract, or decide inside twenty minutes whether to depend on this. They are **not
evaluating documentation and must never be told they are**. The UX brief names the
three things that must never happen to them, and each converts evidence into
agreement: being told the answer, being asked to grade a page, being made to feel the
session is a test of them.

**U2, the facilitator/logger**, is the one this story actually gives work to: capture
what happened — searches typed, links followed, files opened, commands run — *while it
is happening*, without steering it. The failure state the brief names for U2 is
precise: discovering afterwards that the shape they needed was never fixed, and
reconstructing it from memory. `friction-log-skeleton` and `dt9-and-fixed-protocol`
exist to make that impossible, and this story's obligation is to *use* what they
landed rather than improvise at minute forty.

**U3, the downstream actor**, does not appear in this story's session and is entirely
the reason its record must be legible: a sibling-project owner opening the log six
months later must find their items and act without asking the logger what an entry
meant.

### Decision 1 — the tree is a fixture, and it is pinned after the merge, not before

`_storymap.md`'s merge order puts this slice second and states the precondition
flatly: the session **cannot start** until HS-P0022 `application-author-path` and
HS-P0023 `reach-and-adapter-path` have merged forward. The decomposition's reason is
the one sentence to remember — *"A friction log run against a half-assembled surface
measures the assembly, not the teaching."*

This is not schedule hygiene, it is a fixture identity. The testing brief makes the
analogy explicit and it is the useful mental model: **one commit or merge point is one
session's fixture instance**, exactly as `CLAUDE.md`'s conformance rule says one
fixture instance is one isolated backing store. A second session (AC-010, and
`second-session-decision`'s) opens a *new* fixture rather than reopening this one.

There is a concrete, verified reason the pin has to be post-merge rather than
"whatever is on the branch". `crates/happenstance/src/lib.rs` is **75 lines in this
worktree** and 237 lines on `initiative/from-contract-to-published-library`, with the
`Tags::empty()` defect — this initiative's central piece of evidence — present only in
*their* version (`.bklg/docs-that-teach/_decomposition.md:81-103`). A session walking
the front door on this branch's stale copy would produce findings about a file that
was already replaced.

### Decision 2 — concurrent narration is a *methodological choice already made*, and it costs time on purpose

`_design.md` declares the narration mode. If it declares **concurrent** — which is the
friction-log template's own default and the mode the research favours for surfacing
findings — then two consequences bind this story and neither is negotiable at session
time. Reactions are captured **as they occur**, never summarised afterward; the
evidence is that concurrent and retrospective narration are measurably *different
instruments*, not an early and a late version of one. And task time inflates by
roughly **17–20%**, which is **recorded as a stated property of the run, never treated
as a comparable metric** — a session that reports its duration as if it were clean
task-time data has manufactured a number.

Whichever mode `_design.md` fixed, this story honours it and says so in the log. It
does not switch mode mid-session because the reader went quiet.

### Decision 3 — severity is a text token, applied inline, and never retrofitted

The severity scale is named in `_design.md` before the session (project AC-002); this
story *applies* it. Two properties are load-bearing and both are checkable by a
stranger:

- **Inline, at the moment.** A severity mark added in a tidy-up pass after the session
  is the exact difference the research draws between a rankable record and a diary.
- **A text token — the word or the number on the named scale — never colour and never
  an emoji carrying the meaning alone.** The accessibility floor states why in this
  medium's own terms: a markdown log rendered in a diff view has no colour at all, and
  a screen reader and `git diff` must both read the severity. Note the trap — the
  stoplight convention the source template uses is red/yellow/green **by name**, so
  "use the stoplight scale" is satisfied by the words and violated by three coloured
  squares.

### Decision 4 — the facilitator does not rescue the reader, and every time they do it is an entry

IQ-5, non-interference: the facilitator does not answer the reader's question, does
not point at the file, and does not narrate on the reader's behalf. The reasoning is
the same finding non-authorship rests on — *insiders unconsciously route around the
rough spots the exercise exists to find* — and a facilitator who rescues the reader
reintroduces exactly that from the other chair.

The rule is not "never intervene". It is: **if the session must be unblocked to
continue, the intervention is itself a timestamped entry.** UX-AC-009 closes the loop
in the direction that makes this falsifiable rather than decorative: *a session log
containing zero interventions asserts that none occurred.* Silence is a claim here,
not an absence.

### Decision 5 — abandonment is a valid end state, and a short log is still evidence

IQ-6. The reader may stop at any point. A session that ends at the first blocker has
**produced a finding, not a void** — the small-sample basis for running one session at
all is that each problem a real reader actually hits is already proven worth fixing,
without needing to recur. So the log records *where* and *why* it stopped, and is
neither discarded nor silently re-run. Re-running is `second-session-decision`'s
explicit, recorded call (project AC-010), and it opens a new fixture.

The wrong move this rules out is the tempting one: coaching the reader past the
blocker to "get more data". That converts the one finding you had into agreement.

### Decision 6 — record what they *did*, not whether they liked it

Project derived requirement 3 (from BR-05) is the whole discipline in one line: search
terms typed, links followed, files opened, code copied, commands run. Verbatim where
verbatim is possible — a search string is the *string*, not "they searched for the
error". This matters mechanically downstream: a stumble whose entry records the exact
query a reader typed is a stumble a sibling project can act on with `rg`; a stumble
recorded as "couldn't find it" is opinion.

Keyboard reach is part of the same record, and the accessibility floor states it in
both directions: if the reader could not reach something without a pointer, **that is
a finding, logged**, not a session defect to work around.

### Decision 7 — the record is append-only during the session, with stable occurrence-ordered ids

IQ-3, preserved position. Stumbles carry ids assigned in the order they occurred; the
chronological section is append-only *during* the session; ids are never reassigned.
The reason is downstream citation: `route-and-escalate` will hand sibling projects a
stumble id, and a later reader landing on a cited anchor must find the item that was
cited. Renumbering to close gaps, reordering by severity in place of chronology, or
re-heading a section breaks every citation already made into the log.

The scaffold owns the id *convention*; this story owns not violating it under time
pressure. If two stumbles land within the same minute, they still get distinct ids in
occurrence order — no merging.

### Decision 8 — this project ships no code, and that does not lower the bar

`_design.md` declares no `pub` item, no signature, no doctest and no screen, and the
perceptual review is a genuine skip because `design.capture` is deliberately absent
from `.redkiln/config.yaml`. **No Accepted decision atom under `.kb/decisions/`
governs documentation, personas or comprehension methodology** — all seventeen concern
port flavours, crate naming, opaque payloads, MSRV, error traits, append conditions,
position assignment, event identity, validated identifiers and the wire format. This
spec therefore cites **no decision atom for its core subject, because none exists**,
and states that rather than manufacturing one. The single KB atom that binds anywhere
near this project is `.kb/governance/rewrite-the-referent-never-the-reasoning.md`, and
it binds at the *disposition* seam — which is a sibling story's, not this one's.

What does not relax: this story's artifact is the proof artifact, so its evidence is
`file:line` citations into the log itself, and `require_ledger: true`
(`.redkiln/config.yaml`) blocks `implement → report` until every AC row carries one.

### Standing constraints inherited, not re-decided

- **Nothing under `.kb/` is written by this story.** Hand-authoring atoms outside the
  ingest path is a named non-goal of the initiative and the first attempt was reverted
  (`0269720`). Promotion is HS-P0025's, at closeout.
- **No persona is promoted, reconciled or renamed.** This story *observes* one; the
  three personas remain drafts carrying their "none has been directly observed"
  qualification until HS-P0025 replaces it.
- **Quizzes and inline recall checks are settled out on evidence**, not available as a
  lighter-weight alternative if the session runs long.
- **No sibling project's content is edited here** — not a doc comment, not
  `docs/README.md`, not the example. A fix is a *disposition*, and it lands in
  `content-fixes-from-dispositions`.
- **One-line checkbox.** Any checklist this story writes keeps each box on one line;
  the gate parser matches line by line and a wrapped box can never match. This is the
  closest thing to a hard token constraint in this repository.
- From `CLAUDE.md`'s binding constraints: none is touched by this story, and none may
  be disturbed. No `#[async_trait]`; no `serde` in `happenstance-core`'s defaults;
  `EventStore::read` returns the stream at the top level; generic code binds
  `EventStore`, not `SendEventStore`.

## Integration contract

This story is delivered **mounted** — the session's record lands in the project's real
friction-log artifact, the one `disposition-every-stumble` and `handoff-note-to-closeout`
read next. A private notes file, a scratch document in the story folder, or a screen
recording nobody transcribed is **not** a delivery.

- **Archetype**: `capability` — a user-observable slice. The observable is a real
  reader's walk and the auditable record it leaves; `_storymap.md` types it so.
- **Slice / milestone**: `observed-session`. **Slice-mates: none — this milestone is
  deliberately a single story.** `_storymap.md` says why: it is "the one irreducible
  act: a real person, a real tree, once", and splitting "run the session" from "record
  what happened" would be exactly the horizontal cut this project exists to avoid.
- **Mount point**: the **friction log** landed by `friction-log-skeleton`, at the path
  `_design.md` binds. Expected at
  `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, following the
  `_`-prefixed companion convention this directory already uses (`_grounding.md`,
  `_decomposition.md`, `_storymap.md`, `_design.md`). **This story does not choose that
  path** — the UX brief is explicit that the log's path and heading vocabulary are
  `_design.md`'s to fix, deliberately not the story map's. If the skeleton binds a
  different home (`references/evaluation/` is the plausible alternative, being where
  this repository keeps evidence held for citation), this story follows it and the PR
  boundary below is widened **here, deliberately, before the session** — never at gate
  time.
- **Wires into**:
  - `.bklg/docs-that-teach/comprehension-evidence/_design.md` — the protocol this
    session executes: DT-9's persona, the scenario in the reader's own words, the
    narration mode, the severity scale, the disqualifying criteria. Consumed, never
    amended by this story.
  - The friction log's **declaration section**, filled by `non-insider-recruitment`
    with the reader's own statement against those criteria. This story asserts it is
    present and filled *before* the session starts; it does not write it.
  - The friction log's **chronological section, stumble-id convention, severity column
    and disposition slot** — `friction-log-skeleton`'s shape, filled here.
  - The friction log's **disposition slots** — left empty and correctly shaped for
    `disposition-every-stumble`, which is blocked on this story.
  - The material walked, which this story **reads and never edits**: `docs/README.md`
    (36 lines today — the signpost tree), `examples/course-subscriptions/` (the one
    worked example), `crates/happenstance-core/src/store.rs` (the adapter author's
    error site), and the rendered rustdoc of `crates/happenstance-core/` and
    `crates/happenstance/` — all as assembled post-merge from HS-P0020 through
    HS-P0023.
  - `.redkiln/config.yaml:40` (`affected_gate`) and `:55` (`integration_scoped`) — the
    story-grain and integration-grain commands this story's PR is checked by. Because
    it touches nothing under `crates/`, `cargo xtask affected` falls through to the
    file-reading lints and `spec-trace` unconditionally rather than passing vacuously
    on an empty package set.
- **Renders surfaces**: **no `_design.md` surface id, because `_design.md` declares
  none** — its `## Items` block is an empty `yaml` fence with the reason stated, and
  the design review recorded `hasSurface: false` rather than skipping the project. The
  artifact this story renders is the UX brief's **surface 2, the friction log** — "a
  dated markdown artifact whose reader is a reviewer, a sibling-project owner, and
  eventually HS-P0025", whose legibility the brief calls "the whole deliverable". It
  renders and changes nothing else; it explicitly does not render surface 1 (the
  material walked) or surface 3 (`_design.md`).
- **Conformance rule(s)**: **none, and this is not adapter-observable.** No rule in
  `crates/happenstance-testkit/src/suite.rs` can observe a friction log; no port,
  signature or bound changes here, so there is nothing an adapter could implement
  wrongly. Stating this rather than leaving it blank is the point — `CLAUDE.md`
  requires a story that changes a port to name a rule, and this story's answer is that
  it changes no port. It adds no rule, and adds no literal position assertion anywhere.
- **Clause(s)**: **discharges and amends none.** No `SPECIFICATION.md` clause is
  edited, no `[FROZEN]` clause changes, no line-anchored citation moves, so no ADR is
  owed and `cargo xtask spec-trace` has nothing to re-anchor. If a *finding* implies a
  clause is wrong, that is a stumble with a disposition, routed by `route-and-escalate`
  — never a clause edit inside this PR.
- **Advances DoD scenario**: initiative **DoD-5** — *"A reader who is not the author
  and not an insider completes a stated scenario, and it is recorded... a dated log in
  the auditable shape."* This story lands four of DoD-5's five elements (the
  chronological record of what was searched, opened and tried; reactions captured as
  they occurred; severity marked inline; the date), consuming the stated scenario from
  `_design.md` and the declared status from `non-insider-recruitment`. It also lands
  project DoD item 1 and the *precondition* for **DoD-6** — there is nothing to
  disposition until this log has stumbles in it — but DoD-6 itself is
  `disposition-every-stumble`'s and `route-and-escalate`'s.

## PR boundary

### In this PR

- The **session, run once**, against the post-merge assembled tree, by U1 with U2
  facilitating, under `_design.md`'s protocol unmodified.
- The **tree identifier** written into the log: the commit or merge SHA of the
  assembled material, plus which sibling projects' work that SHA contains and a
  human-readable statement of how it was obtained.
- The **session date** and the reader's **run context**: platform, toolchain, browser,
  and any assistive technology actually in use during the session.
- The **chronological record**: append-only, occurrence-ordered stable stumble ids,
  search terms verbatim, links followed, files opened, commands run and what they
  emitted, reactions captured as they occurred.
- **Inline severity marks** on `_design.md`'s named scale, as text tokens.
- **Facilitator interventions** as timestamped entries, or the explicit statement that
  none occurred.
- **Keyboard-reach observations**, in both directions — reached, or reached only with
  a pointer and therefore logged as a stumble.
- The **end state**: completed, or abandoned at a named point with the reason.
- This story's own backlog folder: the `_ledger.md` with `file:line` evidence per AC,
  and any session-conduct notes.

### Explicitly not in this PR

- **The protocol itself** — DT-9's persona, the scenario, the narration mode, the
  severity scale, the disqualifying criteria. `dt9-and-fixed-protocol`'s, and already
  committed at a date that must precede this session for project AC-002 to hold. This
  story consumes them and may not edit `_design.md`.
- **The log's shape** — section set, id convention, severity column, disposition slot,
  revision block. `friction-log-skeleton`'s. If the shape proves wrong mid-session, the
  session finishes inside the shape it has and the shape defect is itself logged.
- **The reader's declaration and the eligibility verdict** —
  `non-insider-recruitment`'s, and a precondition of this story, not an output of it.
- **Any disposition, route, escalation or destination id** —
  `disposition-every-stumble`'s and `route-and-escalate`'s. Disposition slots are left
  empty and correctly shaped. Writing "will fix" beside a stumble during the session is
  the failure mode this boundary exists to prevent.
- **Any content fix** to `docs/README.md`, a doc comment,
  `examples/course-subscriptions/` or any crate — `content-fixes-from-dispositions`'s,
  and only for stumbles whose disposition is "fixed". The reader's walk must not be
  edited out from under the record of it.
- **The scope sentence, the second-session verdict and the hand-off note** —
  `scope-the-claim`, `second-session-decision` and `handoff-note-to-closeout`'s.
- **Anything under `.kb/`**, including staging a persona or an open question. Not this
  project's, at any stage.
- **A second session.** If this one is dominated by a single blocking defect, that is
  an input to `second-session-decision`, not an unrecorded re-run.

### Merge DoD

The log carries a tree SHA that resolves from this worktree; the chronological section
is non-empty and occurrence-ordered; every stumble carries a text-token severity mark
and an empty, correctly-shaped disposition slot; the six AC-004 elements tick against
the markdown alone with no recording needed; `cargo xtask affected --base main` green
(falling through to the file lints and `spec-trace`, this story touching no crate);
`redkiln validate && redkiln doctor` clean with no new `template-drift`; and the
`_ledger.md` carries a real `file:line` per AC.

### Paths

`redkiln verify --grain story` reads the first fenced block under this heading and
fails on any file changed outside it. The log's home is `_design.md`'s to bind (see
`## Integration contract`); the glob below covers every home inside this project's
folder. If the skeleton lands it outside — `references/evaluation/` being the plausible
alternative — this boundary is widened here, deliberately, **before** the session runs.

```
.bklg/docs-that-teach/comprehension-evidence/**
```

## Behavior and interfaces

The observable behaviour, element by element. "Evidence path" is where the claim is
grounded today — the thing to read, not the thing to copy.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **the tree is named as an identifier, not a description** | The log records the commit or merge SHA of the assembled material (HS-P0020 → HS-P0023 merged forward), what that SHA contains, and how it was obtained. "The current main" and "the docs branch" both fail: a later reader must be able to resolve it and decide whether a stumble still applies. | `project.md` AC-005 · `_decomposition.md` `## Testing brief`, AC-005 row (`git show`/`git log` resolution) · `_storymap.md` merge order §2 |
| **the tree is post-merge, verified before the session, not assumed** | HS-P0022 and HS-P0023 are confirmed merged forward *into the walked tree* before the reader is seated. Verified, because the one file this initiative's central evidence lives in is 75 lines here and 237 on the sibling branch with the `Tags::empty()` defect intact only there. | `.bklg/docs-that-teach/_decomposition.md:81-103` · `crates/happenstance/src/lib.rs` (75 lines, this worktree) · `project.md` risk table, "walks material that is about to be replaced" |
| **the protocol is consumed, not written** | Scenario, narration mode, severity scale and disqualifying criteria are read out of `_design.md` and executed. Nothing in `_design.md` is edited by this story, which is what keeps project AC-002's provenance check (`git log --format=%aI` on `_design.md` predating the session date) meaningful. | `.bklg/docs-that-teach/comprehension-evidence/_design.md` · `_decomposition.md` `## Testing brief`, AC-002 row |
| **the declaration is present before the reader starts** | `non-insider-recruitment` has filled the declaration section, and it is checkable against `_design.md`'s criteria without asking the reader anything further. If the recruited reader is ineligible-but-used-anyway, the log says so as a **stated failure of the artefact**, never as a redefinition of the bar. | `project.md` AC-003 · `_decomposition.md` `## UX brief`, states list ("Candidate declared, eligibility resolved") · `project.md` risk table, row 1 |
| **the session date and run context are recorded** | Date; platform; toolchain; browser; any assistive technology actually in use. A stumble is only reproducible against the context that produced it. | `project.md` derived requirement 2 · `_decomposition.md` UX-AC-004 · accessibility floor, "The reader's own context is part of the record" |
| **what the reader did, verbatim where verbatim is possible** | Search terms as typed (the string, not a paraphrase), links followed, files opened, code copied, commands run and what they emitted. Not whether they liked it. | `project.md` derived requirement 3 · `_grounding.md` `## The friction-log method`, the template's chronological-record element |
| **reactions are captured as they occur, in the declared mode** | Concurrent unless `_design.md` says otherwise; never summarised afterward, and the mode is not switched mid-session. Where concurrent, the ~17–20% task-time inflation is recorded as a property of the run and **not** presented as a comparable metric. | `project.md` derived requirement 5 · `_grounding.md`, Hertzum 2024 meta-analysis bullet · research 04 |
| **severity is a text token on the named scale, inline** | The word or the number, applied at the moment the stumble is written — never retrofitted in a tidy-up pass, never colour or emoji alone. Readable with all styling stripped, in `git diff`, and by a screen reader. | `_decomposition.md` UX-AC-003 · accessibility floor, "Colour is never the only carrier" · `project.md` derived requirement 6 |
| **stumble ids are stable and occurrence-ordered; the section is append-only** | Ids assigned in occurrence order, never reassigned, no renumbering to close gaps, no reordering by severity in place of chronology, no re-heading. Two stumbles in the same minute still get distinct ids. | `_decomposition.md` IQ-3 / UX-AC-007 · `_storymap.md`, why the skeleton is its own story |
| **facilitator interventions are timestamped entries** | The facilitator does not answer, point or narrate on the reader's behalf. Where the session had to be unblocked, that intervention is an entry with its timestamp. Zero interventions is a positive assertion that none occurred, not a blank. | `_decomposition.md` IQ-5 / UX-AC-009 · research 04, non-authorship as mechanism |
| **keyboard reach is recorded either way** | Whether the reader reached what they needed using the medium's own affordances — rustdoc keyboard search, browser find, intra-doc links. A pointer-only reach is logged **as a stumble**, not smoothed over. | `_decomposition.md` UX-AC-010 · accessibility floor, "Keyboard reachability, both sides" |
| **abandonment is a valid, recorded end state** | The reader may stop at any point; the partial log is evidence. Where it stopped and why are recorded; the session is not discarded and not silently re-run. Coaching the reader past a blocker to "get more data" is the forbidden move. | `_decomposition.md` IQ-6 · `_grounding.md`, Nielsen & Landauer bullet · `project.md` AC-010 (the re-run is a *recorded decision*, elsewhere) |
| **the record is complete as static text** | AC-004's six elements tick against the markdown alone — no recording, tool or rendering step required. A screen or audio recording may exist as a supplement and is never the evidence of record. | `_decomposition.md` UX-AC-004 · accessibility floor, "Static text is the evidence of record" |
| **nothing exists only in a filtered or collapsed view** | Any roll-up, severity extract or per-owner view this session writes is additive. Delete every one of them and the chronological record still carries every stumble. | `_decomposition.md` IQ-2 / UX-AC-006 · `interaction-patterns.md` anti-pattern (load-bearing item behind a fold) |
| **disposition slots are left empty and correctly shaped** | The session marks severity; it does not decide destinations. A "will fix" written beside a stumble during the session pre-empts `disposition-every-stumble` and starts the double-disposition failure project AC-006 forbids. | `_storymap.md` coverage, AC-006 row · `_decomposition.md` UX brief states list ("Stumble recorded, undispositioned" — legal during the session) |
| **no scope sentence, no hand-off, no second-session verdict here** | Those three are separately owned and separately gated. This story writes none of them, and writing one early would create a second summary that can drift from the approved one. | `_storymap.md` slices table, `scoped-claim-and-handoff` rows |
| **nothing else moves** | No `.kb/` file, no persona promotion, no doc comment, no `docs/README.md` edit, no crate, no `SPECIFICATION.md` clause, no conformance rule, no `_design.md` amendment. No quiz or recall check is introduced as a lighter-weight substitute if time runs short. | `project.md` `## Out of scope` · `_decomposition.md` `### Notes`, quizzes settled out · `CLAUDE.md` binding constraints |

### The acceptance criteria this story enumerates

Nine, in the order the behaviour above lands them, framed from the reader's and the
downstream actor's intent rather than from the artifact that serves them: **AC-001**
(the tree is a resolvable identifier, pinned post-merge) · **AC-002** (session date and
run context recorded, and the declaration present before the reader starts) ·
**AC-003** (what the reader did, verbatim, captured as it happened in the declared
narration mode) · **AC-004** (severity a text token on the named scale, marked inline,
never retrofitted) · **AC-005** (stumble ids stable and occurrence-ordered; the section
append-only) · **AC-006** (facilitator interventions timestamped, or none asserted) ·
**AC-007** (keyboard reach recorded either way; a pointer-only reach is a stumble) ·
**AC-008** (abandonment accepted and recorded as a valid end state) · **AC-009** (the
record is complete as static text and nothing lives only in a filtered view, with
disposition slots empty and correctly shaped for the next story).

## Data and migrations

**N/A for schema and runtime state — and not N/A for one identifier space, which is
handled above rather than here.**

There is no database, no serialized format, no wire envelope, no persisted runtime
artefact and no migration in scope. This story adds no `pub` item, compiles nothing,
and touches no crate under `crates/`; `happenstance-core` carries no `serde` in its
default features by binding constraint and nothing here goes near that. The one
code-shaped obligation this project carries — small content fixes arising from
dispositions, and the `cargo xtask ci --fast` bar that bites for them — belongs to
`content-fixes-from-dispositions`, not to this story.

The one thing that *is* data-shaped is the **stumble id space**, and its rules are
stated in `## Behavior and interfaces` rather than duplicated here because they are
observable session behaviour, not a stored schema. It is worth naming what makes them
migration-like: stumble ids become external references the moment `route-and-escalate`
hands one to a sibling project, and there is no forwarding mechanism — no redirect, no
alias table, nothing that would let a renumbering be absorbed. So the "migration
policy" for this identifier space is simply **there is none, and none is needed,
because ids are append-only and never reassigned** (IQ-3). Gaps are permitted and
expected; closing them is the forbidden operation, in the same spirit as this
repository's rule that a conformance test must never assert literal position values
because the specification permits gaps.

## Acceptance criteria

Nine, framed from the intent of the person the criterion serves — U1 the recruited
reader, U2 the facilitator, U3 the downstream actor
(`.bklg/docs-that-teach/comprehension-evidence/_decomposition.md`, `## UX brief`, the
three-user table) — each crossing the whole slice from protocol through session to
record. The log referred to throughout is the one `friction-log-skeleton` lands at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` with the eight sections
and the `FL-###` entry shape it fixes; where `_design.md` fixes a different path, that
path substitutes verbatim in every row below.

"Verification" here follows the project testing brief's tier split
(`_decomposition.md`, `## Testing brief`): **Static** where a claim is a `git`/`rg`/
`test -f` fact, **Artifact-evidence** where it is genuinely reviewer-read and carried by
the `_ledger.md` `file:line` discipline `require_ledger: true` (`.redkiln/config.yaml:67`)
already enforces. Neither tier is a euphemism for the other, and per the brief no check
below passes a wrong implementation as readily as a right one.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3 opens the log six months from now and needs to know whether a stumble still applies, **WHEN** they read the `## Session record`, **THEN** the tree walked is a commit or merge SHA that resolves from this repository — together with which sibling projects' work that SHA contains and how it was obtained — and **NOT** a description such as "current main" or "the docs branch". The SHA is the post-merge assembled tree: HS-P0022 `application-author-path` and HS-P0023 `reach-and-adapter-path` are confirmed merged forward *into that tree* before the reader is seated, verified rather than assumed. | **Static (provenance).** `git log --oneline -1 <sha>` and `git show --no-patch --format=%H <sha>` resolve from this worktree; `git log --oneline <sha> -- crates/happenstance/src/lib.rs docs/README.md` shows the sibling merges present. Cross-checked against `.bklg/docs-that-teach/_decomposition.md:81-103` (the 75-vs-237-line divergence with `Tags::empty()` present only on the sibling branch). **Artifact-evidence:** ledger cites the `## Session record` tree line by `file:line`. Traces project AC-005. |
| AC-002 | **GIVEN** U2 must produce a record a stranger can reproduce a stumble against, **WHEN** the session opens, **THEN** the `## Session record` already carries the session date and the reader's real run context — platform, toolchain, browser, and any assistive technology actually in use — and the reader's own non-insider declaration is present and resolved (eligible / ineligible / ineligible-but-used-anyway) *before the first task is read out*, **NOT** back-filled afterwards. Where the state is ineligible-but-used-anyway, the log says so as a stated failure of the artefact and the bar is **not** redefined. | **Static (provenance + presence).** `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` predates the recorded session date; `rg -n "Platform\|Toolchain\|Browser\|Assistive\|Declaration" <log>` returns a filled value for each, with no not-yet-recorded token surviving. **Artifact-evidence:** ledger cites the declaration line and confirms it is checkable against `_design.md`'s disqualifying criteria without asking the reader anything further (project AC-003's own wording). Traces project AC-004. |
| AC-003 | **GIVEN** U1 is trying to get *their own* job done against the library and is never told they are grading it, **WHEN** they search, click, open, copy or run something, **THEN** the log's `## Chronological record` carries it as an `FL-###` entry written *at that moment* in `_design.md`'s declared narration mode — search strings verbatim as typed, links followed, files opened, commands run and what they emitted, reactions as they occurred — and **NOT** a post-session summary, a paraphrase ("they searched for the error"), or an opinion about whether they liked it. Where the declared mode is concurrent, the ~17–20% task-time inflation is recorded as a stated property of the run and never presented as a comparable metric. | **Artifact-evidence (reviewer-read, ledger-cited).** A reviewer reads the record end to end and confirms entries are behavioural and verbatim: at least one entry carries a quoted search string, at least one carries a command and its output. **Static support:** `rg -n "^### FL-" <log>` is non-empty and the narration-mode line in `## Session record` matches `_design.md`'s verbatim. Traces project AC-004. |
| AC-004 | **GIVEN** U3 needs to rank what to fix without asking the logger what an entry meant, **WHEN** they read any stumble with all styling stripped — in `git diff`, in a plain-text pager, or through a screen reader — **THEN** its `Severity` field is a word or number drawn from the `## Severity scale` legend `_design.md` named, applied inline as the entry was written, and **NOT** a colour, an emoji, a swatch carrying the meaning alone, or a mark added in a tidy-up pass after the session. | **Static.** `git show HEAD:<log>` read as plain text: every `Severity` value on an `FL-###` entry of a stumble kind matches a token in the legend table; no severity value is emoji-only or colour-only. **Artifact-evidence:** ledger cites one entry per distinct severity token and the reviewer confirms inline application (marks are interleaved with entries in chronological position, not appended in a block). Traces project AC-004; refines UX-AC-003. |
| AC-005 | **GIVEN** `route-and-escalate` will hand a sibling project a stumble id and a later reader will land on that cited anchor, **WHEN** the log is finalised, **THEN** every `FL-###` id is the one assigned in occurrence order at write time, ids are never reassigned, no gap is closed, no entry is reordered by severity in place of chronology, no heading line is re-worded, and nothing already written during the session is edited — a correction is an appended entry — **NOT** a tidied, renumbered or severity-sorted record. Two stumbles inside the same minute still get distinct ids. | **Static.** `rg -n "^### FL-" <log>` yields strictly increasing, unique ids with no duplicates; `git log -p --follow <log>` shows the session's commits as appends — no diff hunk rewrites a previously written `### FL-` heading or its body. **Artifact-evidence:** ledger cites the first and last entry lines. Traces project AC-004; refines UX-AC-007 / IQ-3. |
| AC-006 | **GIVEN** the finding rests on the reader being genuinely unrescued, **WHEN** the facilitator answers a question, points at a file, narrates on the reader's behalf, or unblocks the session so it can continue, **THEN** that moment is an `FL-###` entry of `Kind: intervention` carrying its timestamp — and where none occurred, the log states positively that none occurred, **NOT** leaving the reader to infer it from an absence. | **Static.** `rg -n "Kind: intervention" <log>` — either returns timestamped entries, or the `## Session record` carries the explicit zero-intervention assertion, and the reviewer confirms exactly one of the two is true. **Artifact-evidence:** ledger cites the interventions or the assertion by `file:line`. Refines UX-AC-009 / IQ-5. |
| AC-007 | **GIVEN** U1 may be reaching the material with keyboard only or with assistive technology, **WHEN** they need to find something using the medium's own affordances — rustdoc keyboard search, browser find, intra-doc links — **THEN** the log records whether they reached it, and a reach that required a pointer is written **as a stumble with a severity mark**, **NOT** smoothed over as a session mechanics detail or omitted because it "wasn't about the docs". | **Artifact-evidence (reviewer-read, ledger-cited), with static support.** `rg -ni "keyboard\|browser find\|rustdoc search\|pointer" <log>` returns at least one recorded observation; the reviewer confirms it is stated in one of the two directions, and that any pointer-only reach carries a `Severity` token rather than sitting as a bare note. Refines UX-AC-010; accessibility floor, "Keyboard reachability, both sides". |
| AC-008 | **GIVEN** the small-sample basis says every problem a real reader hits is already proven worth fixing, **WHEN** U1 stops — completing the scenario or abandoning at a blocker — **THEN** the log records the end state explicitly: completed, or abandoned at a named point with the reason, as an `FL-###` entry of `Kind: abandonment`; the partial log is kept as evidence and is **NOT** discarded, silently re-run, or rescued by coaching the reader past the blocker to "get more data". | **Static.** The log terminates in either a recorded completion of the `## Scenario` or an entry with `Kind: abandonment` (`rg -n "Kind: abandonment" <log>`), and `git log --oneline <log>` shows one session's worth of commits — no second session's entries appended under this story. **Artifact-evidence:** ledger cites the terminal entry. Refines IQ-6; feeds project AC-010, which is `second-session-decision`'s recorded call. |
| AC-009 | **GIVEN** U3 must find their items and act without a tool, a rendering step or a conversation, **WHEN** they open the log as plain markdown, **THEN** all six of project AC-004's elements tick against the text alone; every stumble is present in the visible-by-default `## Chronological record` so that deleting the whole `## Dispositions index` and every fold or extract would lose nothing; and every stumble's `Disposition` field is present, on the `not yet dispositioned` arm, with its `Revisions` slot present and defaulting to `none` — **NOT** pre-filled with a "will fix", a destination, or a "noted", which would pre-empt `disposition-every-stumble` and start the double-disposition failure project AC-006 forbids. | **Static (deletion check + presence).** In a scratch copy, delete `## Dispositions index` and every collapsed block: `rg -c "^### FL-" ` is unchanged. Every `FL-###` entry carries a `Disposition` line; `rg -n "Disposition:" <log>` shows only the `not yet dispositioned` arm — zero `fixed:`, `accepted:`, `routed:`, `escalated:` values. No recording, script or widget is referenced as required reading. **Artifact-evidence:** ledger cites the six AC-004 elements by `file:line`. Refines UX-AC-004 / UX-AC-006 / IQ-2; guards project AC-006. |

**Coverage of the traced project ACs.** Project **AC-004** (*a dated log exists in the
auditable shape — scenario; logger identity, context and date; chronological record
including search terms and links followed; reactions as they occurred; severity marked
inline*) is carried by AC-002, AC-003, AC-004, AC-005 and AC-009, which together own the
*content* half of the split `_storymap.md` states — the skeleton owns the shape. Project
**AC-005** (*the log names the tree it walked*) is carried by AC-001, sole owner.
AC-006, AC-007 and AC-008 are not additional project ACs; they are the UX brief's
interaction-quality invariants for this story's own surface, promoted to table rows
because a bullet is never gated.

## Interaction quality

RFC §6.7/D6. Every invariant below is already an `AC-###` row above — this section says
**which** row carries it and how the row is checked, and adds nothing new. That placement
is deliberate: `redkiln verify` extracts ACs by matching a leading `| AC-001 |` table cell
or an `- AC-001:` bullet, so an invariant stated only as prose here would carry no ledger
row, be gated by nothing, and be tested by nobody.

A note on where the composition family comes from.
`.bklg/docs-that-teach/comprehension-evidence/_design.md` is signed off and declares
`hasSurface: false` with an empty `## Items` fence — this project renders no screen and no
`pub` item, so there is no surface id to bind to. What `_design.md` *does* bind, in its
`## Items` prose, is the **primitive layer**: *"there is no CSS layer and no token file,
and inventing one is out of scope"*, and everything these artifacts need is drawn from
named repository document primitives that already exist. The composition invariants below
are therefore taken from that clause plus the UX brief's `#### The primitive layer to
compose from — do not hand-roll` and `#### The accessibility floor`, and from the entry
shape `friction-log-skeleton` fixes. They are not re-decided here.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump (IQ-1)** — a reviewer reading a stumble sees its disposition state at the stumble; an index may exist in addition, one hop maximum and the hop optional | **AC-009** | The `Disposition` field is a line inside the `FL-###` entry, not a pointer into `## Dispositions index`. Static: every entry matched by `rg -n "^### FL-"` has a `Disposition:` line within its own block |
| **Non-occlusion (IQ-2)** — a filter, roll-up or extract never hides what it filters; nothing exists only in a collapsed or derived view | **AC-009** | The deletion check: remove `## Dispositions index` and every fold from a scratch copy; the stumble count is unchanged |
| **Preserved position (IQ-3)** — ids and chronological order are stable once written; the section is append-only during the session | **AC-005** | `rg -n "^### FL-"` gives unique, strictly increasing occurrence-ordered ids; `git log -p --follow` shows appends only, no rewritten heading or body |
| **Reversibility (IQ-4)** — nothing written during the session is overwritten; a correction is an appended entry, and the `Revisions` slot exists on every entry from first write, defaulting to `none` | **AC-005** (append-only) and **AC-009** (the slot is present and correctly shaped) | Static: every entry carries a `Revisions` line; the diff history shows no in-place edit of a written entry. The *revision arm itself* is `disposition-every-stumble`'s, bound by `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Keyboard reachability** — the walk is completable with the medium's own affordances, and a pointer-only reach is a logged finding rather than a workaround | **AC-007** | The log states it in one of the two directions; a pointer-only reach carries a severity token |
| **Non-interference (IQ-5)** — the facilitator does not answer, point or narrate for the reader; an unblocking intervention is a timestamped entry, and zero interventions is a positive assertion | **AC-006** | `rg -n "Kind: intervention"` returns entries, or the explicit zero-intervention assertion is present — exactly one of the two |
| **Abandonment is a valid end state (IQ-6)** — the reader may stop; the partial log is evidence | **AC-008** | A recorded completion or a `Kind: abandonment` entry terminates the record; the log is neither discarded nor silently re-run |

### Composition invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — every entry is composed from the skeleton's fixed field set (`Time` · `Kind` · `Severity` · `What happened` · `Disposition` · `Revisions`), never a bare paragraph of prose dropped under a heading | **AC-003** (the behavioural `What happened` content) and **AC-009** (the field set is present and correctly shaped) | Static: each `### FL-###` block carries all six labelled fields; an entry missing `Severity` or `Disposition` fails |
| **Composition and placement** — the log keeps the skeleton's eight sections in their fixed order (`## Status` · `## Scenario` · `## Session record` · `## Severity scale` · `## Chronological record` · `## Dispositions index` · `## Scope of the claim` · `## Hand-off`); session content lands in `## Session record` and `## Chronological record` and nowhere else | **AC-002**, **AC-003**, **AC-009** | `rg -n "^## "` returns the eight headings in order; the diff shows no section added, renamed or reordered by this story |
| **Transience — persistent chrome vs revealed vs opened-on-demand** | **AC-004** and **AC-009** | *Persistent chrome*: the `## Status` line (replaced by session date and tree, never deleted), the `## Severity scale` legend and the `## Scope of the claim` sentence are visible by default at all times. *Revealed*: nothing. *Opened on demand*: nothing — no fold, no collapsed admonition, no inactive tab, and no external recording is required to read any element |
| **Density budget, with its real numbers** | **AC-002** (six context fields), **AC-003** and **AC-009** (six one-line entry fields), **AC-004** (one token) | Scenario 1–2 sentences, transcribed verbatim, unedited by this story. `## Session record`: exactly the six context fields, one line each. Every `FL-###` entry: exactly the six labelled fields, each on one line. `Severity`: exactly one token from the legend. Any checkbox this story writes: one line, never wrapped — the gate parser matches line by line and a wrapped box can never match |
| **Hierarchy** — the chronological record is authoritative and the index is derived; the log is self-contained for a reader who has not opened `_design.md` (the severity legend is reproduced as a token→meaning table naming `_design.md` as source of record) | **AC-004**, **AC-009** | The legend is present and every applied token appears in it; `## Dispositions index` states in its own first line that it is derived and that the record is authoritative |
| **Anti-pattern — colour or emoji as the sole carrier** of severity | **AC-004** | Plain-text read of `git show HEAD:<log>`: every severity value is a legend word or number |
| **Anti-pattern — a load-bearing item behind a fold, an inactive tab or a collapsed admonition** with no visible-by-default counterpart (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`) | **AC-009** | The deletion check above |
| **Anti-pattern — a bespoke navigation widget layered on a medium that renders the equivalent for free**; and inventing a CSS layer, a token file or a new heading vocabulary | **AC-009** | The log requires only browser find and stable heading anchors; the diff introduces no script, no widget, no style file and no new section heading |
| **Anti-pattern — retrospective reconstruction presented as concurrent narration** | **AC-003** | The narration-mode line matches `_design.md` verbatim and entries carry per-moment timestamps rather than a single post-session block |
| **Anti-pattern — a pre-filled disposition** ("will fix", "noted", a destination) written beside a stumble during the session | **AC-009** | `rg -n "Disposition:"` shows only the `not yet dispositioned` arm |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | The assembled tree does not actually contain HS-P0022 and HS-P0023 — the merge has not happened, or happened only on a sibling branch | **The session does not start.** `_storymap.md`'s merge order states the precondition flatly, and the decomposition's reason is the operative one: a log run against a half-assembled surface measures the assembly. Record the blocked state in the story's own notes and stop; do not substitute "close enough" material. Running anyway would produce findings about `crates/happenstance/src/lib.rs` at 75 lines when 237 is what ships |
| **EC-002** | The recruited reader turns out to be ineligible against `_design.md`'s disqualifying criteria, and is used anyway because no one else is available | The **ineligible-but-used-anyway** state is representable and is recorded as such in `## Session record`. The log states plainly that the artefact is **unmet** — this is a failure of the initiative, not a reason to redefine the bar (`project.md`, risk table, row 1). Downstream summaries inherit that statement; the hand-off note may not quietly drop it |
| **EC-003** | Mid-session, the skeleton's shape proves wrong — a needed field has no slot | The session **finishes inside the shape it has**, and the shape defect is itself logged as an `FL-###` entry with a severity mark. Improvising a new field at minute forty is exactly the U2 failure state the UX brief names, and re-shaping the log mid-run breaks AC-005's stability guarantee for every entry already written |
| **EC-004** | The reader asks the facilitator a direct question, or stalls in a way that would end the session prematurely | IQ-5 governs: do not answer, do not point, do not narrate for them. If the session genuinely must be unblocked to continue, the unblocking is performed **and logged as `Kind: intervention` with its timestamp** (AC-006). Silent rescue is the one unrecoverable failure here — it corrupts the finding and leaves no trace that it did |
| **EC-005** | The reader abandons at the first blocker and the session is five minutes long | Valid outcome, not an error state to recover from. AC-008 applies: record where and why, keep the log, do not coach past the blocker and do not re-run silently. Whether a second session follows is `second-session-decision`'s recorded call (project AC-010) and opens a new fixture |
| **EC-006** | The reader hits a genuine library bug rather than a documentation problem | It is still an `FL-###` entry with a severity mark. Its destination is the `support` initiative (`.redkiln/config.yaml:5`), but the destination is **not** written here — the `Disposition` field stays on `not yet dispositioned` and `route-and-escalate` assigns the id |
| **EC-007** | A finding implicates a `[FROZEN]` clause in `spec/SPECIFICATION.md` or a design tension a sibling project already resolved | Log the finding; change nothing. A clause edit or a re-decision inside this PR would put half a decision in each of two projects. Escalation with the DT id is `route-and-escalate`'s (project AC-011) |
| **EC-008** | Note-taking falls behind — the reader moves faster than the record | The record's auditable properties are unrecoverable after the fact, so the correct recovery is to **slow the session, not the record**: pause the reader's task narration rather than reconstruct entries afterwards. A reconstructed block is a different instrument and, if any part of the record is reconstructed, that fact is stated in `## Session record` rather than left implicit |
| **EC-009** | Two stumbles occur inside the same minute, or a single moment reads as both an observation and a reaction | Distinct `FL-###` ids in occurrence order; no merging, no shared id. Where the kind is ambiguous, write two entries rather than one composite — merging is unrecoverable, splitting is not |
| **EC-010** | `friction-log-skeleton` landed the log somewhere other than `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (`references/evaluation/` being the plausible alternative) | This story follows the skeleton, and the `## PR boundary` fenced block is widened **here, deliberately, before the session runs** — never at gate time, and never by editing the boundary after a file has already been written outside it |
| **EC-011** | No reader can be seated at all within the window | Terminal failure state of the project, recorded as such. Not a prompt to seat an insider (EC-002), and not a prompt to simulate one — the testing brief is explicit that a stand-in reader *"is not a cheaper version of this instrument; it is a different, useless one"* |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | The recorded session duration is a **stated property of the run, never a comparable metric**. Where the declared narration mode is concurrent, the ~17–20% task-time inflation is named alongside it | Reporting a concurrently-narrated duration as if it were clean task-time data manufactures a number. `project.md` derived requirement 5; `_grounding.md`, `## The friction-log method` |
| **NF-002** | The record is complete **before the facilitator leaves the session** — timestamps, verbatim search strings, intervention entries and inline severity are all written live, and the log is committed the same day | Every one of those properties is unrecoverable by evening. This is the concrete form of the U2 failure state, and it is the only "performance" requirement this story has |
| **NF-003** | The log is readable with **no tool, script, rendering step or network access** — plain markdown, browser find, stable heading anchors | AC-009 states it as a criterion; NF-003 states it as a standing property of the artifact for every later reader, including a `git diff` view with no colour at all |
| **NF-004** | The reader's identity is recorded only to the degree needed to make the declaration checkable — role, relationship to the repository, and the context fields — and no credential, access token, private path or third-party personal detail enters the log | The log is committed to a repository that will be public. The declaration must be verifiable without the artifact becoming a personal record |
| **NF-005** | The session and the record introduce **no new dependency, tool, format or convention** — no CSS, no token file, no widget, no heading vocabulary not already in the skeleton | `_design.md`, `## Items`: inventing a primitive layer is out of scope. The primitives are the named existing files |
| **NF-006** | Repository hygiene is unchanged: `redkiln validate --kb && redkiln doctor` clean at **exactly** the six standing `template-drift` advisories, no more and no fewer | `CLAUDE.md`, `## Where the work lives` — a seventh is an unintended template change, a missing one is a reverted customisation. This story changes no template and must move neither number |
| **NF-007** | The evidence is legible to a stranger six months out: a reviewer who has never met the logger can read any entry and act on it without asking what it meant | The UX brief's stated bar for U3, and the reason `What happened` records behaviour verbatim rather than impressions |

## Implementation notes (non-prescriptive)

Not instructions — the reasoning behind the shape, so the implementer can make the
hundred small calls the criteria do not reach.

**Do the pre-flight the day before, not the hour of.** Three things are cheap to verify
early and expensive to discover late: that the merge is actually in the tree walked
(EC-001), that the skeleton's eight sections and `FL-###` shape are on disk, and that the
declaration section is filled and resolved. Each is a `git`/`rg` check taking seconds;
each, discovered at minute zero with a reader waiting, costs the session.

**Write the tree SHA into the log before the reader arrives.** It is the one field that
cannot be reconstructed later with confidence — a branch moves, a merge lands, and "the
tree we used" becomes a guess. Capture `git rev-parse HEAD` of the exact checkout the
reader will walk, together with a one-line human statement of what it contains.

**Prefer more entries to richer entries.** Two entries at the same minute cost nothing;
one composite entry that later needs splitting costs an id that a sibling project may
already have cited (AC-005, EC-009). The same asymmetry governs kinds: an `observation`
mis-typed as a `reaction` is a cosmetic error, a merged pair is a structural one.

**A verbatim string is worth more than a good paraphrase.** The mechanical downstream
value of an entry is that `content-fixes-from-dispositions` can run `rg` on what the
reader actually typed. Capture the string even when it is misspelled — especially then,
because a misspelling that finds nothing *is* the finding.

**Leave the severity token slightly rough rather than deferring it.** A token applied in
the moment and later judged one notch off is a rankable record; a token applied in a
tidy-up pass is a diary with a column. The scale is `_design.md`'s and is not
re-negotiated at the table.

**Resist writing the disposition.** The strongest pull in a live session is to write
"will fix" next to something obvious. It costs the project AC-006's guarantee and creates
a second disposition that can drift from the approved one. The `not yet dispositioned`
arm exists precisely so the pull has somewhere legal to go.

**Do not fix anything the reader stumbles on, however small, in this PR.** Editing
`docs/README.md` mid-session edits the reader's walk out from under the record of it.
The fix is `content-fixes-from-dispositions`'s and is gated by `cargo xtask ci --fast`
there.

**When in doubt about whether a moment belongs in the log, write it.** The record is
append-only and gaps are permitted; an entry that turns out to be uninteresting costs a
line. An unwritten one costs the finding.

## Tests and CI (merge gate)

Grounded in `_decomposition.md`, `## Testing brief` — its AC/tier table and its
content-fix tier — and in `.redkiln/config.yaml`'s wired grains. `<log>` is
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` (or the path `_design.md`
fixed, per `## Integration contract`). The testing brief's warning applies to every row
below: a check that would pass `"noted"` as readily as a real value rejects no wrong
implementation, so every static check here asserts on **content**, not merely on
structural presence.

| tier | command / path | proves |
| --- | --- | --- |
| **Provenance (static)** | `git log --oneline -1 <tree-sha>` and `git log --oneline <tree-sha> -- crates/happenstance/src/lib.rs docs/README.md`, run from this worktree against the SHA in `<log>`'s `## Session record` | AC-001 — the tree is a resolvable identifier and it contains the sibling merges, not a description of one |
| **Provenance (static)** | `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` compared against the recorded session date | AC-002 — the protocol could not have been written to fit what the reader did. This is project AC-002's own mechanism, honoured rather than re-owned |
| **Structure (static, `rg`)** | `rg -n "^## " <log>` (the eight sections, in order); `rg -n "^### FL-" <log>` (unique, strictly increasing ids) | AC-005 and the composition invariants — placement and id stability |
| **Content (static, `rg`)** | Per-entry field presence with a real value: `Time`, `Kind`, `Severity`, `What happened`, `Disposition`, `Revisions` on every `FL-###` block; `Severity` matched against the `## Severity scale` legend tokens | AC-003, AC-004, AC-009 — the entry is composed, not bare prose, and the severity token is on the named scale |
| **Content (static, `rg`)** | `rg -n "Disposition:" <log>` returns only the `not yet dispositioned` arm — zero `fixed:` / `accepted:` / `routed:` / `escalated:` | AC-009 — no disposition was pre-empted during the session |
| **Content (static, `rg`)** | `rg -n "Kind: intervention" <log>`, or the explicit zero-intervention assertion in `## Session record`; `rg -n "Kind: abandonment" <log>` or a recorded completion | AC-006, AC-008 — silence is never allowed to stand for either |
| **Plain-text legibility (static)** | `git show HEAD:<log>` read with all styling stripped | AC-004, NF-003 — a screen reader and `git diff` both read the severity; nothing needs a renderer |
| **Deletion check (static)** | In a scratch copy, delete `## Dispositions index` and every fold; compare `rg -c "^### FL-" ` before and after | AC-009 / IQ-2 — no stumble exists only inside a derived or filtered view. This is the UX brief's own stated method, applied verbatim |
| **Append-only history (static)** | `git log -p --follow <log>` across this story's commits | AC-005 / IQ-3, IQ-4 — no written entry was rewritten in place |
| **Artifact-evidence (ledger)** | `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/_ledger.md`, enforced by `redkiln verify --grain story` with `require_ledger: true` (`.redkiln/config.yaml:67`) | Every AC row carries a real `file:line` into the log. This is the tier that carries AC-003 and AC-007, which are genuinely reviewer-read and which the brief forbids over-automating |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The repository gate for this diff. Touching no crate, it falls through to the five file-reading lints and `spec-trace` unconditionally rather than passing vacuously on an empty package set |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7's non-terminal bar. Expected green and unchanged — this story alters no crate, so a failure here is a signal the PR boundary leaked |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | NF-006 — clean at exactly the six standing `template-drift` advisories, and no `.kb/` atom authored |
| **Explicitly not run** | `cargo xtask ci` (`.redkiln/config.yaml:60`, `e2e`) | The terminal bar is HS-P0025's. And per the brief: the session is a research instrument, not a CI step — a passing gate says nothing about comprehension, and folding the session into the merge gate would confuse two instruments that falsify different things |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, inside this PR |
| --- | --- | --- |
| The session runs against a tree that does not yet contain HS-P0022 / HS-P0023, and the findings describe an absence rather than a teaching failure | Medium / High | EC-001 blocks the start; AC-001's verification is a `git log` against the SHA touching `crates/happenstance/src/lib.rs` specifically, because that file is the one with a verified 75-vs-237-line divergence carrying `Tags::empty()` only on the sibling branch (`.bklg/docs-that-teach/_decomposition.md:81-103`) |
| The facilitator rescues the reader without noticing, and the finding quietly becomes agreement | Medium / High | AC-006 makes zero interventions a positive assertion rather than a blank, so the claim is falsifiable. The mitigation is structural, not diligence-based: the entry kind exists in the skeleton, so logging is the path of least resistance |
| Severity marks get applied in a tidy-up pass, and the record becomes a mood report with a column | Medium / Medium | AC-004 checks inline application (marks interleaved in chronological position, not appended as a block) plus a plain-text token match; the legend is transcribed from `_design.md` before the session so there is nothing to decide at the table |
| A disposition leaks into the session — a "will fix" or a destination written beside a stumble | Medium / Medium | AC-009 asserts the `not yet dispositioned` arm exclusively. This is the single most likely boundary breach in this PR because the pull is strongest at the moment of the finding |
| The PR boundary widens after a file has been written outside it — most plausibly a "tiny" fix to `docs/README.md` | Low / High | The `## PR boundary` fenced block is read by `redkiln verify --grain story`, and EC-010 requires any widening to be a deliberate pre-session edit. A content fix is `content-fixes-from-dispositions`'s, and editing the walked material invalidates the record of the walk |
| The reader is ineligible and the bar is quietly relaxed to salvage the session | Low / High | EC-002 keeps ineligible-but-used-anyway a representable state whose consequence is a **stated unmet artefact**, carried forward into every summary. `project.md`'s risk table already fixed this direction; this story does not get to soften it |
| Findings are dominated by one blocking defect and the rest of the evidence is thin | Medium / Medium | Not this story's to resolve, and deliberately so: AC-008 keeps the short log as evidence, and the second-session question is `second-session-decision`'s explicit recorded verdict (project AC-010). Coaching past the blocker is the forbidden repair |
| `_design.md` looks wrong at session time and the temptation is to amend it | Low / High | `_design.md` is signed off (Ryan Britton, 2026-08-17) and its commit date is what makes project AC-002's provenance check meaningful. Amending it inside this PR destroys that check retroactively. The defect is logged as a finding and escalated by `route-and-escalate` |
| The log's home differs from the expected `_friction-log.md` and this spec's paths go stale | Low / Low | `friction-log-skeleton`'s spec currently binds `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` and mounts it at `project.md`'s `## Companions`, so the two agree today; EC-010 covers divergence, and the `## PR boundary` glob already covers the whole project folder |

**Coupling, stated once.** Everything downstream of this story is blocked on it and
nothing downstream can repair a defect in it: `disposition-every-stumble` cannot invent a
stumble, `scope-the-claim` cannot narrow a claim about a session that did not happen, and
`handoff-note-to-closeout` supplies HS-P0025 the one input it cannot manufacture. The
upstream coupling is temporal and outside the team's control — a person's availability —
which is why recruitment began a slice earlier while HS-P0023 was still in flight.

## Dependencies

**Blocks on** (both are `depends_on` in `_storymap.md`, and both must have merged before
this story starts):

| Story slug | What it must have landed before the reader is seated |
| --- | --- |
| `friction-log-skeleton` | The log on disk at the path it binds, with all eight sections, the `FL-###` id convention, the six-field entry shape, the severity column, the `not yet dispositioned` disposition arm and the `Revisions` slot, mounted in `project.md`'s `## Companions`. Without it, the shape is improvised at minute forty and IQ-2/IQ-3/IQ-4 are not properties the record has |
| `non-insider-recruitment` | The reader's own declaration in `## Session record`, resolved to eligible / ineligible / ineligible-but-used-anyway against `_design.md`'s criteria — checkable without asking the reader anything further. AC-002 asserts it is present *before* the session, and this story does not write it |

Transitively, through `friction-log-skeleton`: `dt9-and-fixed-protocol`, which fixed
DT-9's persona, the scenario, the narration mode, the severity scale and the
disqualifying criteria in `_design.md` at a commit that must predate the session date.

Beyond the story graph, the project DAG adds a hard precondition this story cannot
proceed without: **HS-P0022 `application-author-path` and HS-P0023 `reach-and-adapter-path`
merged forward** (`_storymap.md`, `## Merge order`, item 2; EC-001; AC-001).

**Unlocks** — directly:

| Story slug | What it takes from this story |
| --- | --- |
| `disposition-every-stumble` | The severity-marked stumbles themselves, each with an empty, correctly-shaped disposition slot. There is nothing to disposition until this lands |
| `scope-the-claim` | A session that happened, with a known end state, for the narrow claim to be about |

And transitively, through those two: `route-and-escalate`,
`content-fixes-from-dispositions`, `second-session-decision`, and
`handoff-note-to-closeout` — which is the sole input HS-P0025 `durable-audience-closeout`
cannot manufacture.

## Anchors (progressive disclosure)

Load-bearing depth, deferred but not optional. The `## Context pack` above is
self-sufficient to begin; open these at the stated moment. **Link, never paste in bulk.**
Every path below was confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off protocol of record and the only durable record of DT-9: the persona, the scenario verbatim, the narration mode, the severity scale and the disqualifying criteria. It also fixes the primitive layer this story's composition invariants are drawn from, and declares `hasSurface: false` with the reason stated | Immediately before the session, and again the moment any protocol question arises at the table. **Read-only — amending it retroactively destroys project AC-002's provenance check** | AC-002, AC-003, AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes the exact shape this story fills: the eight sections in order, `### FL-###` occurrence-ordered ids with frozen heading lines, the six one-line entry fields, the five disposition arms, the `Revisions` slot defaulting to `none`, and the log's path. Its `## Behavior and interfaces` table is the field-by-field contract | Before writing the first entry — and before the session, as part of the pre-flight that confirms the scaffold is actually on disk | AC-003, AC-004, AC-005, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/non-insider-recruitment/spec.md` | Owns the declaration section and the eligibility verdict this story asserts is already present and resolved, including the ineligible-but-used-anyway state and what it obliges the log to say | During pre-flight, to confirm the declaration is filled and resolved before the reader is seated | AC-002 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief's seven interaction-quality invariants (IQ-1…IQ-7), the accessibility floor stated in this medium's own terms, the three-user table, and the testing brief's AC/tier table and its warning against over-automating a check that rejects nothing | Open the UX brief when composing an entry or judging whether an observation is a stumble; open the testing brief when deciding how an AC is actually verified | AC-003, AC-004, AC-006, AC-007, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | The verified grounding: `## The friction-log method` (the source template's shape, the severity scale, the small-sample basis) and `## Headline finding` — that no Accepted decision atom governs this subject, which is why this spec cites none | When a methodological claim needs its source, particularly the narration-mode and severity-scale evidence | AC-003, AC-004, AC-008 |
| `.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` | The primary evidence: non-authorship as *mechanism* rather than nicety, the concurrent-vs-retrospective meta-analysis and its ~17–20% task-time inflation, the friction-log template, and the finding that a log with no destination is a diary | When the pull to summarise afterwards, rescue the reader, or switch narration mode arrives mid-session — this is the argument that answers it | AC-003, AC-006 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three candidate personas with their fitting techniques, and the explicit risk that the recruited reader is not automatically identical to any one of them. All three carry the "none has been directly observed" qualification this session exists to convert for exactly one | Before the session, to hold the reader's intent in mind as *their* goal rather than a documentation evaluation. **Not to be promoted, renamed or edited — that is HS-P0025's** | AC-002, AC-003 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The anti-pattern list this story's composition invariants cite by name — a load-bearing item behind a fold or collapsed admonition, a bespoke widget over a medium that renders the equivalent free — plus the settled ruling against quizzes and recall checks | When tempted to add a roll-up, a fold, or a "quicker" comprehension check because the session is running long | AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | The project ACs this story traces (AC-004, AC-005), the eleven derived requirements, and the risk table rows this spec inherits — including the ineligible-reader row and the about-to-be-replaced-material row | When a criterion's intent is ambiguous, and when writing the ledger's `criterion` values | AC-001, AC-002, AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | The merge-order precondition in its own words ("cannot start until HS-P0022 and HS-P0023 have merged forward"), why `observed-session` is deliberately one story, and the AC-004 split between the skeleton's shape and this story's content | During pre-flight, and whenever the scope of "run the session" feels like it should be split | AC-001, AC-003 |
| `.bklg/docs-that-teach/_decomposition.md` | The initiative DAG and the verified 75-vs-237-line divergence of `crates/happenstance/src/lib.rs` with `Tags::empty()` present only on the sibling branch — the concrete reason the pin must be post-merge | When verifying the tree SHA, to know what specifically to check is present | AC-001 |
| `crates/happenstance/src/lib.rs` | The file whose two versions differ; reading it in the walked tree is the fastest confirmation that the merge is actually in | Pre-flight, alongside the `git log` check | AC-001 |
| `docs/README.md` · `examples/course-subscriptions/` · `crates/happenstance-core/src/store.rs` | The material the reader walks — the signpost tree, the one worked example, and the adapter author's error site. **Read to know what the reader is looking at; never edited by this story** | While observing, to record what was opened accurately | AC-003 |
| `.redkiln/config.yaml` | The wired gate grains this PR is actually checked by: `affected_gate` (line 40), `integration_scoped` (line 55), `e2e` (line 60, explicitly not this project's), `require_ledger` (line 67), and `support_initiative` (line 5, the destination a library bug will later route to) | When running the merge gate, and when a finding turns out to be a library bug rather than a docs problem | AC-009 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom that binds anywhere near this project. It binds at the *disposition* seam, not here — but it is the reasoning behind the append-only and revisions discipline this story must not break | If a written entry appears to need correcting mid-session | AC-005 |
| `.redkiln/templates/_ledger.md` | The ledger shape and its rules: planning authors every row `satisfied: false`; the implementer may only flip a row with real evidence and may never re-word a criterion | When filling `_ledger.md` after the session | all |

## Clarifications resolved during spec

**The AC set is exactly the nine the front half enumerated.** None added, none dropped.
AC-001…AC-009 above are the same ids, in the same order, with the same subjects as
`## Behavior and interfaces`' closing paragraph, promoted to full GIVEN/WHEN/THEN
criteria. The `_ledger.md` carries exactly these nine rows.

**Where the composition invariants come from, given `hasSurface: false`.** `_design.md`
declares no surface id and an empty `## Items` fence, so there is no signed-off surface to
bind to in the usual sense. Rather than treat that as an exemption, this spec takes the
composition family from what `_design.md` *does* bind in its `## Items` prose — the named
document primitive layer, and the explicit ruling that inventing a CSS or token layer is
out of scope — joined to the UX brief's accessibility floor and the entry shape
`friction-log-skeleton` fixes. Nothing in `## Interaction quality` is a new decision, and
every invariant there is carried by a table row above.

**Project AC-003 is a precondition here, not a co-owned criterion.** AC-002 asserts the
declaration is present and resolved before the session starts. `_storymap.md` gives
AC-003 solely to `non-insider-recruitment`, so this story checks the state and never
writes or re-judges it. What this story *does* own is the consequence: if the state is
ineligible-but-used-anyway, the log states the artefact is unmet (EC-002).

**Project AC-010 is touched but not owned.** AC-008 makes abandonment a valid, recorded
end state and EC-005 forbids a silent re-run — both of which produce the *input* to the
second-session question. The verdict itself is `second-session-decision`'s, and running a
second session inside this PR would open a new fixture under a story that owns one.

**The log path is confirmed, not assumed.** `friction-log-skeleton`'s spec binds
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` and mounts it at
`project.md`'s `## Companions`, which matches the front half's expectation. EC-010 keeps
the divergence case handled without weakening the boundary, and the `## PR boundary` glob
already covers the whole project folder either way.

**"Verifying test" means a real check, not a test file.** This project has no functions
and no test binary, and the testing brief says so plainly. Each AC's verification is
therefore a real `git`/`rg`/deletion check against a real path, plus the ledger's
`file:line` discipline — and per the brief's own warning, each static check asserts on
content rather than mere structural presence, so none of them is a check that nothing can
fail.

**No decision atom is cited for this story's core subject because none exists.** All
seventeen Accepted atoms under `.kb/decisions/` concern port flavours, crate naming,
opaque payloads, MSRV, error traits, append conditions, position assignment, event
identity, validated identifiers and the wire format. Stating the absence is the honest
form; manufacturing an ADR number would not be.
