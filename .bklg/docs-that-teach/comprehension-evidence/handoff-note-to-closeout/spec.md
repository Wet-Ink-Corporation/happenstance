---
item: HS-S0171
stage: spec
created: 2026-08-17T13:16:23.376Z
updated: 2026-08-17T13:16:23.376Z
template_sig: 87bbf1d0
rendered_sig: ed34d191
---

# Spec — The hand-off note naming one directly observed persona

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-14 (the scoped claim), BR-13/BR-17 (the audience this evidence eventually feeds), AC-09, AC-14 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `### Definition of Done` (rows 5, 6, 15), `### Dependency graph` (the HS-P0024 → HS-P0025 edge), `### Why the load-bearing edges exist` (the sentence that names this story's obligation verbatim) |
| Project (the spine this story serves) | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — **AC-009 is this story's sole-owned criterion; AC-008 is carried, not re-owned**; DoD items 4 and 5; risk-table row 2 |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md` |
| Key briefs (ux + testing; no architecture, no deployment) | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (U3's row, the `Handed off` state, IQ-1/IQ-2/IQ-3/IQ-4/IQ-7, **UX-AC-013**) and `## Testing brief` (the AC-008/AC-009/AC-010 tier rows, and the "do not automate AC-006 or AC-009 into a check that nothing can fail" note) |
| Grounding | [`../_grounding.md`](../_grounding.md) — `## The friction-log method: cited research, not invention` (Nielsen & Landauer, the small-sample basis for the scope sentence); `## Routing destinations for AC-007 — verified against real project files` |
| Signed-off design — **binding** | [`../_design.md`](../_design.md) — `hasSurface: false`, signed off by Ryan Britton on 2026-08-17 with no conditions; it declares no `pub` item and no rendered screen, and it is where `dt9-and-fixed-protocol` writes the protocol (including the persona DT-9 resolved) |
| Story map | [`../_storymap.md`](../_storymap.md) — the `scoped-claim-and-handoff` slice row, `## Coverage` (the AC-008 split), `## Merge order` step 4 (**this story lands last**) |
| This story's discover artifact | [`discover.md`](discover.md) — the named wrong implementation this spec must reject |
| Consumer's card (the reader this note is written for) | [`.bklg/docs-that-teach/durable-audience-closeout/project.md`](../../durable-audience-closeout/project.md) — DR-5 and its AC-007, the two obligations this note exists to make dischargeable by lifting |
| Roadmap pointer | none. `RUNBOOK.md` sequences library phases, not documentation projects; this initiative's ordering is `../../_decomposition.md`'s dependency graph and `../_storymap.md`'s merge order |

## One-line PR slice

Write the named hand-off section stating one directly observed persona, the date, the tree walked and
the scope sentence, in a form HS-P0025 can lift without re-deriving anything, so the blanket "none has
been directly observed" qualification is replaced by an accurate one.

## Executive summary

**What this PR lands.** The `## Hand-off` section of the friction log, filled — the last empty section
in the project's proof artifact — plus the one-line pointer in HS-P0025's project card that makes it
findable from the consumer's side.

**Pointer plus delta, not a restatement.** `friction-log-skeleton` landed the section as five empty
slots carrying the `NOT-YET-RECORDED` token; `session-run-against-pinned-tree` filled the tree and the
date into `## Session record`; `scope-the-claim` fixed the scope sentence; `second-session-decision`
recorded the verdict; `route-and-escalate` closed the last routed destination. **Every fact this story
writes already exists somewhere in that log.** The delta is that they are, for the first time, in one
place, in one order, addressed to one reader who has not read the log — and that exactly one persona is
named as directly observed while the other two are named as still inferred.

**Why that is a story and not a paragraph.** HS-P0025's AC-007 says no promoted atom may carry an
unqualified blanket "none has been directly observed", and its DR-5 says the qualification is
*corrected, not copied* (`../../durable-audience-closeout/project.md`). Today the only written statement
of persona evidence status is `../../_discovery/distillation/personas-and-journeys.md`'s `## Risks`
bullet, which says none of the three has been observed. If this note does not exist, HS-P0025 discharges
DR-5 by reading the whole log and inferring which persona the session walked — which is precisely the
re-derivation the decomposition forbids: the promotion *"must record precisely which persona the
friction log did directly observe rather than shipping the blanket 'none has been directly observed'
qualification unchanged"* (`../../_decomposition.md`, `### Why the load-bearing edges exist`).

**What it does not land.** No persona atom, no `.kb/` file, no edit to the distillation's `## Risks`
bullet, no new finding, and no disposition. Those are HS-P0025's, and the correction of the blanket
qualification is HS-P0025's act — this story delivers the input it acts on.

## Context pack

**Read this section and you can start.** Everything under `## Anchors` is deferred depth, not optional
depth: open an anchor at the moment its bound AC names it.

### The decision this story exists to make

**The hand-off is a *derived, addressed summary*, not a second home for evidence.** Two failure modes
sit either side of it and this story is the narrow path between them.

- **Underclaiming** — the note ships the blanket qualification unchanged, or hedges ("a reader walked
  some of the material"), and HS-P0025 either promotes a guess or re-derives the answer by reading the
  whole log. This is `discover.md`'s named wrong implementation, verbatim: *"A handoff that states no
  persona was directly observed, when the session observed one — the blanket qualification shipped
  unchanged."*
- **Overclaiming** — the note reads as a research finding about the audience rather than as one
  session's observation, and the scope sentence is dropped on the way out because "the log already says
  it". `../project.md`'s risk table row 2 names this as the project's second-largest risk and states the
  mitigation in the same breath: *"AC-009's hand-off note carries it into HS-P0025's input rather than
  trusting the reader of the log to re-derive it."*

Both are avoided by the same rule: **every sentence in the hand-off resolves to something already
written elsewhere in the log, and the scope sentence travels with the claim.**

### The persona-journey slice this realizes

**U3, the downstream actor — specifically HS-P0025's implementer, six months and one merge later.**
The UX brief states what they are trying to do and what must never happen to them: *"Open the log, find
the items that are theirs, and act — without reading the whole thing and without asking the logger what
an entry meant"*; and never *"reaching an item whose destination is a description rather than an id, or
whose disposition is implied by silence"* (`../_decomposition.md`, `## UX brief`, the three-user table).
Applied to this story, that reader's item is one sentence — *which persona was actually observed* — and
"a description rather than an id" here means a persona named in prose this note invented instead of the
name `../../_discovery/distillation/personas-and-journeys.md` already uses.

The state being reached is the brief's `Handed off`: *"One persona named as directly observed, with date
and tree (AC-009). Absent this state, HS-P0025 cannot replace the blanket 'none has been directly
observed' qualification honestly."* U1, the recruited reader, never sees this section; U2, the
facilitator, writes it after the session is over and after every disposition is closed.

### The decisions this story must honor, stated as decisions

1. **Name the persona in the distillation's own vocabulary, and name the other two as still inferred.**
   The three are `Persona 1 — The application author`, `Persona 2 — The adapter author` and
   `Persona 3 — The evaluator` (`../../_discovery/distillation/personas-and-journeys.md`, its three `##`
   headings). Which one the session walked is DT-9's resolution in `../_design.md`, cross-checked against
   what the session actually recorded — this story **reads** that decision and never re-decides it. A
   fourth, invented label ("a Rust developer new to the crate") is not liftable: HS-P0025's AC-007 has to
   pair the observation with a persona it is promoting, and a name that matches nothing pairs with
   nothing.

2. **Exactly one persona is claimed, and the claim is the weaker of the two available.** The distillation
   itself warns that *"the friction-log reader described in the intake brief… is not automatically
   identical to any one of the three personas"*. Where the recruited reader fits the chosen persona
   imperfectly, the note says which persona the session was **designed to walk** and states the
   divergence — it does not upgrade a partial fit into a clean observation, and it does not silently
   claim two.

3. **The scope sentence travels with the claim, verbatim.** BR-14 and project AC-008 require the narrow
   claim — *real stumbles were captured and are traceable* — **wherever the evidence is summarised**, and
   this note is the summary HS-P0025 reads. `../_storymap.md`'s `## Coverage` states the split directly:
   `scope-the-claim` owns the sentence and its presence in the log and its summary; **this story is bound
   to carry the same sentence** because it is authored later. Same string, not a paraphrase — a
   re-worded scope sentence is a second claim with no owner.

4. **Nothing in the note asserts exhaustiveness, including by implication.** The testing brief's AC-008
   mechanism is an `rg` for the scope sentence plus `rg -i "exhaustiv"` returning nothing outside a
   disclaiming sentence (`../_decomposition.md`, `## Testing brief`). Softer overclaims — "the
   documentation was validated", "readers can now follow the material" — fail the same criterion without
   tripping that grep, and are the wording a reviewer must actually watch for.

5. **The date and the tree are transcribed, never re-derived.** `../project.md` AC-005 is
   `session-run-against-pinned-tree`'s and the tree id lives in the log's `## Session record`. This
   section copies it so that a reader of the hand-off alone can tell whether a stumble still applies —
   *"a later reader can tell whether a stumble still applies"* is AC-005's stated purpose. A tree id
   that disagrees with `## Session record` is a defect in *this* section, because the session record is
   authoritative and this is the derived view.

6. **The second-session verdict is transcribed, and "not mentioned" is not one of its values.** The UX
   brief's state list is explicit: `run` / `declined-with-reason`, and *"'Not mentioned' is not one of
   the states (AC-010)"*. `second-session-decision` owns the assessment; this note carries its verdict
   so the reader of the summary learns it without opening the log.

7. **Hand-off has a precondition, and the precondition is checked, not assumed.** The brief marks
   `Stumble recorded, undispositioned` as *"Legal during the session, illegal at hand-off"*, and
   AC-006 as exactly one disposition — *"zero and two are both failures"*. This story is the last one in
   the project and is therefore where that becomes checkable end to end. If a stumble is
   undispositioned or a routed destination does not resolve, **the hand-off does not get written** — it
   is not a note that mentions the gap in passing.

8. **The note is derived and deletable (IQ-2).** *"If the collapsed or filtered view were deleted, would
   the record still carry every stumble and its disposition?"* — the hand-off section is a view, so
   deleting it must lose nothing. Concretely: no stumble, no severity, no disposition and no reason may
   exist **only** here. Anything the note needs that is not yet in the chronological record belongs in
   the chronological record first.

9. **Citations into the log are by stable stumble id and heading anchor (IQ-3).** Ids are `FL-###`,
   occurrence-ordered, frozen heading line included, because *"a later reader landing on a cited anchor
   must find the item that was cited"*. This note is the first document that cites them from outside the
   session, so it is where a renumbering would first hurt.

10. **A hand-off is not final, and a later correction appends (IQ-4).** `content-fixes-from-dispositions`
    deliberately does not gate this story: *"a fix that lands late must not be able to hold up the
    evidence HS-P0025 needs, and IQ-4's reversibility means a disposition revised after hand-off is
    recorded as a revision rather than as a rewrite of the note"* (`../_storymap.md`, `### Dependency
    graph`). Authority is
    [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md):
    the referent may be rewritten, the reasoning may not be erased.

11. **This story authors no `.kb/` atom and stages none.** Persona promotion is HS-P0025's
    (`../project.md`, `## Out of scope`), hand-authoring atoms outside the ingest path is a named
    non-goal of the initiative, and the first attempt at it was reverted (`0269720`). The note is written
    so that an atom *can* be authored from it — goal, context, observation status — without being one.

12. **Compose from the repository's existing document primitives.** There is no CSS layer and no token
    file, and inventing one is out of scope (`../_design.md`, quoting `../_decomposition.md`,
    `#### The primitive layer to compose from — do not hand-roll`). The hand-off is labelled one-line
    fields plus prose; any checkbox stays on one line, because the gate parser matches line by line and a
    wrapped box can never match (`CLAUDE.md`).

### What this story is explicitly not deciding

Which persona DT-9 chose (`dt9-and-fixed-protocol`, in `../_design.md`); the scope sentence's wording
(`scope-the-claim`); the second-session verdict (`second-session-decision`); any disposition
(`disposition-every-stumble`, `route-and-escalate`); whether a persona is promoted, how it is
reconciled against HS-S0131, and whether the evaluator is a persona in its own right (all HS-P0025,
`../../durable-audience-closeout/project.md`, AC-001/AC-003/AC-004).

### The ordering constraint that can invalidate this work

**This story lands last in the project** (`../_storymap.md`, `## Merge order`, step 4). Its three
dependencies are not stylistic: written before `route-and-escalate` closes, the note asserts a
precondition that is not yet true; written before `scope-the-claim`, it invents a scope sentence that
then diverges from the one the log carries; written before `second-session-decision`, its verdict slot
is a guess. Two of the three are its slice-mates and are implemented in the same context, in either
order, before it.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice. The observable is that a reader who has never opened the log can state which persona was directly observed, when, against what tree, and under what claim (`../_storymap.md`, slice table) |
| **Slice / milestone** | `scoped-claim-and-handoff` |
| **Slice-mates** | `scope-the-claim`, `second-session-decision` — both land **before** this story, in either order (`../_storymap.md`, `## Merge order`, step 4). All three are implemented together in one context and mounted as one integrated surface |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` → its **`## Hand-off`** section, section 8 of the eight `friction-log-skeleton` fixed. That file is the project's proof artifact and is already mounted in `../project.md`'s `## Companions` list, so the render path a human reaches from `redkiln board` is: project card → `## Companions` → the log → `## Hand-off`. A hand-off written into this story's own folder, a scratch note, or a commit message is **not** a delivery |
| **Second mount** | `.bklg/docs-that-teach/durable-audience-closeout/project.md` → a one-line pointer in its `## Dependencies` section naming the hand-off section by path and heading anchor. Body prose only. Without it, HS-P0025's card names HS-P0024 as a dependency but not the artifact it must lift from, and the note is reachable only by someone who already knows it exists |
| **Wires into** | `_friction-log.md`'s `## Session record` (session date, the tree as a resolvable id, the reader's declared context — transcribed, never re-derived); its `## Chronological record` (`FL-###` ids, cited by anchor); its `## Dispositions index` and disposition slots (the precondition this story checks); its `## Scope of the claim` (the sentence carried verbatim); `../_design.md`'s protocol section (DT-9's resolved persona, the severity scale's name); `../../_discovery/distillation/personas-and-journeys.md`'s three `##` persona headings (the naming vocabulary, read-only); `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract (`require_ledger: true`, `.redkiln/config.yaml:67`) |
| **Design-system primitives consumed** | The document primitives `../_decomposition.md`'s `#### The primitive layer to compose from` names and `../_design.md` re-states: `.redkiln/templates/briefs/brief.md`'s Intent/AC/Notes spine, `.redkiln/templates/_design.md`'s `## Shape decision` table and `## Sign-off` section, `docs/README.md`'s two-column routing table, and the one-line checkbox from `.redkiln/templates/gates/`. No new heading vocabulary beyond the slots the skeleton already fixed; no navigation widget |
| **Renders surfaces** | **None by id** — `../_design.md`'s `## Items` block is an empty `yaml` fence (`# no items — no public API surface, no rendered UI surface`) and every shape section reads `N/A — no user-facing surface`; the design review recorded `hasSurface: false` explicitly rather than skipping the project. This story completes the UX brief's **surface 2, the friction log** — *"a dated markdown artifact whose reader is a reviewer, a sibling-project owner, and eventually HS-P0025"* — and renders neither surface 1 (the material walked) nor surface 3 (`_design.md`) |
| **Conformance rule(s) / clause(s)** | **None, and not adapter-observable.** No `pub` item, no port, no bound, nothing under `crates/`; no rule in `crates/happenstance-testkit/src/suite.rs` can observe a hand-off note, and no `spec/SPECIFICATION.md` clause is discharged or amended, so no ADR is owed. Stated rather than left blank, because `CLAUDE.md` requires a story that changes a port to name a rule and this story's answer is that it changes none. It adds no rule and asserts no literal position value anywhere |
| **Advances DoD scenario** | Initiative **DoD-15** — *"The audience is durable and reconciled"* (`../../_decomposition.md`, `### Definition of Done`, row 15; owner HS-P0025). This story does not turn it green — promotion is HS-P0025's — it supplies the single input HS-P0025 cannot manufacture, discharging its DR-5 and making its AC-007 liftable rather than derivable. It also **completes project DoD items 4 and 5** (`../project.md`) and closes the `Handed off` state of the UX brief's state list. DoD-5 and DoD-6 are already green by this point; this story asserts them as its own precondition rather than advancing them |

**Delivered mounted, not as an isolated component.** The acceptance bar includes that a person who has
never seen this story reaches the filled `## Hand-off` section in two hops from `redkiln board`, and
that HS-P0025's own card points at it by anchor.

## PR boundary

**In this PR**

- The **`## Hand-off` section of `_friction-log.md`, filled**: one persona named as directly observed
  with the other two named as still inferred, the session date, the tree as a resolvable id, the scope
  sentence verbatim, and the second-session verdict — every `NOT-YET-RECORDED` token in that section
  replaced.
- The **precondition check, recorded**: zero undispositioned stumbles, zero double-dispositioned
  stumbles, every routed destination id and every escalated DT id resolving — stated in the section as a
  checked fact with the date it was checked, not asserted in a commit message.
- The **one-line pointer** into `.bklg/docs-that-teach/durable-audience-closeout/project.md`'s
  `## Dependencies` (body prose only — the CLI owns the frontmatter and a `PreToolUse` hook denies the
  edit).
- This story's own `_ledger.md` and its stage artifacts.

**Explicitly not in this PR**

- **Any persona atom, any file under `.kb/`, any staged file under `.kb/_intake/`.** Promotion,
  reconciliation against HS-S0131 and the evaluator question are HS-P0025's
  (`../project.md`, `## Out of scope`; `../../durable-audience-closeout/project.md`, AC-001/AC-003/AC-004).
- **Any edit to `../../_discovery/distillation/personas-and-journeys.md`**, including its `## Risks`
  bullet. Correcting the blanket qualification is HS-P0025's act; this story produces the input for it.
  Editing discovery here would put half a correction in each project.
- **Any new finding, severity mark, disposition or revision.** If writing the note surfaces something
  the log does not carry, it is a stumble for the chronological record or a disposition for
  `route-and-escalate` — never a fact that exists only in the summary (decision 8).
- **Any re-decision** of DT-9's persona, the scope sentence's wording, or the second-session verdict.
- **Any edit under `crates/`, `docs/`, `examples/`, `spec/` or `standards/`.** Content fixes are
  `content-fixes-from-dispositions`'s and do not gate this story.
- **Any automation that "checks" the hand-off exists.** The testing brief forbids it by name: a script
  confirming a `Persona:` line is present passes `Persona: TBD` as readily as a real one, and rejects no
  wrong implementation (`../_decomposition.md`, `## Testing brief`, Notes).

**Merge DoD one-liner** — the log's `## Hand-off` carries one named persona, a date, a resolvable tree
id, the scope sentence verbatim and a `run`/`declined — <reason>` verdict with zero `NOT-YET-RECORDED`
tokens remaining in the section; HS-P0025's card points at it by anchor; `cargo xtask affected --base
main` is green and `redkiln validate --kb && redkiln doctor` clean at exactly the six standing
`template-drift` advisories `CLAUDE.md` documents.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under this
heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/durable-audience-closeout/project.md
.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/**
```

Three paths, and the middle one is the mount rather than a content edit. If a change here appears to
need a fourth — the distillation, a `.kb/` atom, a sibling project's charter — that is the signal this
story has absorbed HS-P0025's work; stop and route it, do not widen the block.

## Behavior and interfaces

The "interface" here is the shape of one section, read by one downstream implementer who has not read
the log. Every row below is a contract between what this story writes and what
`../../durable-audience-closeout/project.md` will lift.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Where it lives** | The `## Hand-off` section of `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — section 8 of the eight `friction-log-skeleton` fixed, already carrying the five named slots as `NOT-YET-RECORDED`. This story fills them; it adds no section, renames none, and reorders none | `../friction-log-skeleton/spec.md`, `## Behavior and interfaces` (the eight sections and the `## Hand-off` slots) |
| **Slot 1 — the observed persona** | One name, drawn verbatim from `../../_discovery/distillation/personas-and-journeys.md`'s `##` headings (`Persona 1 — The application author`, `Persona 2 — The adapter author`, `Persona 3 — The evaluator`), with one sentence of *what that reader was trying to accomplish* so the name is not a label. Cross-referenced to DT-9's resolution in `../_design.md` | `../../_discovery/distillation/personas-and-journeys.md:56,148,225`; `../_decomposition.md`, UX-AC-001 (persona as intent, not label) |
| **Slot 1b — the other two, explicitly still inferred** | Both named, both marked as **not directly observed**, in the same slot. This is what makes HS-P0025's AC-007 a lift rather than an inference: *"the promoted atoms name the persona HS-P0024's friction-log session directly observed, and mark the remaining personas as still inferred"* | `../../durable-audience-closeout/project.md`, AC-007 and DR-5; `../project.md`, AC-009 |
| **Slot 1c — the fit, if it is imperfect** | Where the recruited reader matched the chosen persona partially, the divergence is stated in one sentence rather than smoothed. The distillation warns that the friction-log reader *"is not automatically identical to any one of the three personas"* | `../../_discovery/distillation/personas-and-journeys.md`, `## Risks`, second bullet |
| **Slot 2 — the date** | The session date, transcribed from `## Session record`. Not the authoring date of this note, and not a range | `../project.md`, AC-004, AC-009 |
| **Slot 3 — the tree walked** | The commit or merge id from `## Session record`, as a resolvable ref, so a later reader can tell whether a stumble still applies. Transcribed; a disagreement with `## Session record` is a defect in this section | `../project.md`, AC-005; `../_decomposition.md`, `## Testing brief`, AC-005 row (`git show` resolves it) |
| **Slot 4 — the scope sentence, verbatim** | The same string `scope-the-claim` wrote into `## Scope of the claim`: real stumbles were captured and are traceable, with no assertion of exhaustiveness. Copied, not paraphrased — a re-worded second claim has no owner | `../project.md`, AC-008; `../_storymap.md`, `## Coverage` (the AC-008 split); `../_grounding.md` (Nielsen & Landauer, the small-sample basis) |
| **Slot 5 — the second-session verdict** | Exactly one of `run` or `declined — <reason>`, transcribed from `second-session-decision`'s record. "Not mentioned" is not a value of this slot | `../_decomposition.md`, `## UX brief`, the states list (`Second-session question answered`); `../project.md`, AC-010 |
| **The precondition, stated as a checked fact** | One line recording that at hand-off: every severity-marked stumble carries exactly one disposition (zero and two both failures), every `routed:` id resolves to a real item, every `escalated:` id is a DT id from `../../_decomposition.md`'s ownership table — with the date the check was performed. `Stumble recorded, undispositioned` is *"legal during the session, illegal at hand-off"* | `../_decomposition.md`, `## UX brief`, the states list; `../project.md`, AC-006, AC-007, AC-011 |
| **Derived, never authoritative** | Every claim in the section resolves to something already written elsewhere in the log — a `FL-###` entry, a `## Session record` field, the `## Scope of the claim` sentence. Deleting the whole `## Hand-off` section loses no stumble, no severity, no disposition and no reason (IQ-2's deletion check) | `../_decomposition.md`, IQ-2 / UX-AC-006 |
| **Citations are stable anchors** | Where the note cites a stumble it uses the frozen `FL-###` heading anchor, never a re-description ("the search problem"). This note is the first document to cite those ids from outside the session, and a re-worded heading kills the citation with no error message | `../_decomposition.md`, IQ-3 / UX-AC-007; `../friction-log-skeleton/spec.md`, AC-005 |
| **Legible to someone who has not read the log** | UX-AC-013 is the bar: the section states persona, date, tree and scope sentence *"in a form HS-P0025 can lift without re-deriving anything"*. A reader who opens the log at this section and reads nothing else can write HS-P0025's DR-5 sentence | `../_decomposition.md`, UX-AC-013 |
| **Complete as static text** | No recording, script, rendering step or widget; navigation by browser find and stable heading anchors only; nothing carries meaning by colour or emoji alone; any checkbox on one line | `../_decomposition.md`, `#### The accessibility floor`; `CLAUDE.md` (the line-by-line parser) |
| **Corrections append, never overwrite** | If a disposition is revised after hand-off — `content-fixes-from-dispositions` may land later — the note gains a dated revision line beside the original statement; the earlier text stays legible with its reason | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `../_decomposition.md`, IQ-4 / UX-AC-008; `../_storymap.md`, `### Dependency graph` |
| **Mounted at the consumer's card** | One line in `.bklg/docs-that-teach/durable-audience-closeout/project.md`'s `## Dependencies`, naming `_friction-log.md` and the `## Hand-off` anchor as what DR-5 and AC-007 are discharged from. Body prose only; `git diff` shows no frontmatter hunk | `CLAUDE.md`, `## Where the work lives` (the CLI is the only writer of system frontmatter); `../../durable-audience-closeout/project.md`, `## Dependencies` |
| **No `.kb/` file, and none staged** | The note is written so an atom *can* be authored from it later — persona name, what they were trying to do, observation status, evidence path — without being one. Zero files under `.kb/` in this story's diff | `../project.md`, `## Out of scope`; `CLAUDE.md`, `## Where the work lives` (`0269720`) |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds no crate, no type, no table and no
connection; `../_design.md` records `hasSurface: false` with an empty items block for exactly this
reason. It edits one markdown section and adds one line of markdown prose.

Two identifier contracts are worth naming as data, because other documents foreign-key into them and
this story is the first outside consumer of both:

- **`FL-###`, the stumble id.** Assigned in occurrence order, never reused, never reassigned, heading
  line frozen at write time (`../friction-log-skeleton/spec.md`, `## Data and migrations`). Any
  citation this note makes is against that anchor. The only "migration" the log can undergo is an
  append; renumbering is not a migration, it is a broken citation with no error message.
- **The persona name.** It behaves as a foreign key into
  `../../_discovery/distillation/personas-and-journeys.md`'s three `##` headings and, after HS-P0025's
  ingest run, into the `.kb/product/` atoms promoted from them. A name invented here resolves to
  nothing on either side, which is why decision 1 fixes the vocabulary rather than leaving it to
  wording.

Nothing else in this PR is versioned, parsed or read by a tool: `redkiln verify --grain story` reads the
PR-boundary fence and the ledger, `cargo xtask affected --base main` falls through to the file-reading
lints and `spec-trace` on an empty package set (`.redkiln/config.yaml:40`), and no schema exists for
either to migrate.

## Acceptance criteria

Eight criteria. Each is framed from a persona's intent crossing the whole artifact — **U3, the
downstream actor**, and specifically HS-P0025's implementer six months and one merge later, plus **U2,
the facilitator** closing the record out (`../_decomposition.md`, `## UX brief`, the three-user table).
A criterion framed as a bare capability ("the hand-off names a persona") is satisfied by
`Persona: TBD`, which is exactly the check the testing brief forbids by name (`../_decomposition.md`,
`## Testing brief`, Notes).

Throughout: **the log** means `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; **the
section** means its `## Hand-off`, section 8 of the eight `friction-log-skeleton` fixed; **the token**
means `NOT-YET-RECORDED`, the single greppable emptiness marker that story fixed
(`../friction-log-skeleton/spec.md`, `## Clarifications resolved during spec`, item 1).

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** HS-P0025's implementer opens the log at `## Hand-off` and reads nothing else, **WHEN** they need to write DR-5's sentence — *the observation qualification is corrected, not copied* — **THEN** the section names exactly **one** persona as directly observed, spelled character-for-character as one of the three `##` headings in `../../_discovery/distillation/personas-and-journeys.md`, with one sentence of what that reader was trying to accomplish; names the **other two** and marks each **not directly observed**; and, where the recruited reader fitted the chosen persona only partially, states the divergence in one sentence rather than smoothing it | Static: the persona string in the section matches `rg -n "^## Persona" ../../_discovery/distillation/personas-and-journeys.md` (lines 56, 148, 225) exactly; all three names appear; exactly one carries the directly-observed marker. Artifact-evidence: ledger-cited `file:line` at the slot. **Rejects `discover.md`'s named wrong implementation** — *"A handoff that states no persona was directly observed, when the session observed one"* — and a fourth invented label ("a Rust developer new to the crate"), which pairs with nothing on either side of HS-P0025's promotion |
| **AC-002** | **GIVEN** a reader of the hand-off alone asks whether a stumble still applies today, **WHEN** they look for what was walked and when, **THEN** the section carries the **session date** and the **tree** as a resolvable commit or merge id, both transcribed from the log's `## Session record` and agreeing with it character-for-character — not the authoring date of the note, not a date range, not a branch name that will move | Static provenance: `git show <the cited id>` resolves from this worktree — the same discipline `../_grounding.md` used to *fail* an unreachable citation, applied here to confirm one; a diff of the two strings against `## Session record` is empty. Rejects the note's most natural defect: re-deriving the tree from `git log` at authoring time instead of transcribing, which silently records a *later* tree than the one the reader walked |
| **AC-003** | **GIVEN** the initiative's BR-14 forbids a claim wider than one session supports, **WHEN** HS-P0025 lifts this section into a promoted atom, **THEN** the scope sentence `scope-the-claim` wrote into `## Scope of the claim` appears in the section **as the same string**, and nothing in the section asserts exhaustiveness — not by the word, and not by the softer forms ("the documentation was validated", "readers can now follow the material") | Static: `rg -F "<the scope sentence>"` returns a hit in both `## Scope of the claim` and `## Hand-off`; `rg -i "exhaustiv"` returns nothing in the section outside a disclaiming sentence — the mechanism `../_decomposition.md`'s `## Testing brief` states for AC-008. Artifact-evidence: a reviewer reads the section for the soft overclaims, which pass the grep and fail the criterion. Rejects a paraphrased scope sentence — a second claim with no owner, when `../_storymap.md`'s `## Coverage` assigns the sentence to one story |
| **AC-004** | **GIVEN** the UX brief's state list admits only `run` and `declined-with-reason`, and states that *"'Not mentioned' is not one of the states"*, **WHEN** a reader of the summary asks whether a second session was owed, **THEN** the section carries exactly one of `run` or `declined — <reason>`, transcribed from `second-session-decision`'s record and agreeing with it, with **zero** occurrences of the token left anywhere in the section | Static: `rg -c "NOT-YET-RECORDED"` scoped to the section returns 0; the verdict line matches one of the two forms. Artifact-evidence: ledger-cited `file:line`, cross-read against `../second-session-decision/spec.md`'s verdict vocabulary. Rejects the three shapes that are the same failure — "not mentioned", "TBD", and a slot silently deleted rather than filled |
| **AC-005** | **GIVEN** the brief marks `Stumble recorded, undispositioned` as *"legal during the session, illegal at hand-off"*, **WHEN** U2 sits down to close the record out, **THEN** the section states as a **dated, checked fact** that every severity-marked stumble carries exactly one disposition (zero and two are both failures), every `routed:` id resolves to a real item and every `escalated:` id is a DT id from `../../_decomposition.md`'s ownership table — and if any of those is false the hand-off **is not written at all**, rather than written with the gap mentioned in passing | Static: per-entry disposition-arm count over `rg -n "^### FL-"` and its `Disposition:` fields yields exactly one arm each; `test -f` on every cited destination (`.bklg/support/initiative.md`, `.bklg/docs-that-teach/application-author-path/project.md`, `.bklg/docs-that-teach/reach-and-adapter-path/project.md`); `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` covers every escalated id. Artifact-evidence: the check line with its date, ledger-cited. Rejects a hand-off that ships alongside an open item and notes it as "outstanding" — which converts AC-006 from a bar into a comment |
| **AC-006** | **GIVEN** a reviewer deletes the entire `## Hand-off` section, **WHEN** they re-read the log, **THEN** not one stumble, severity, disposition or reason has been lost (IQ-2's deletion check) — because every claim in the section resolves to something already written elsewhere in the log; every citation the section makes into the record uses a frozen `FL-###` heading anchor rather than a re-description (IQ-3); the disposition of any cited stumble remains readable **at that stumble**, so the section is at most one optional hop (IQ-1); and any post-hand-off correction appends a dated revision line beside the original with the earlier text still legible (IQ-4) | Artifact-evidence, ledger-cited: for each sentence in the section, the `file:line` elsewhere in the log it resolves to — the deletion check performed and its result recorded. Static: every citation in the section matches an existing `^### FL-` heading; `rg -n "<details>\|<summary>\|<script>"` returns nothing. Authority for the revision rule: `.kb/governance/rewrite-the-referent-never-the-reasoning.md`. Rejects the two failures decision 8 and decision 9 name — a severity, reason or destination that exists **only** in the summary, and a citation by prose ("the search problem"), which dies silently when a heading is tidied |
| **AC-007** | **GIVEN** U3 reads the log as a raw `git diff` or through a screen reader, with every style stripped, **WHEN** they land on the section, **THEN** it is composed from this repository's existing **document** primitives — labelled one-line fields on the brief spine, the `## Shape decision` / routing-table shapes, one-line checkboxes — as five named slots plus the precondition line, in the reading order HS-P0025 needs (persona, date, tree, scope sentence, verdict); it introduces no new heading, no fold, no widget and no script; nothing carries meaning by colour or emoji alone; and it is complete and navigable by browser find and stable heading anchors alone | Static: the section adds no `^## ` or `^### ` heading beyond the slots the skeleton fixed; every checkbox line matches a single-line `- [ ] …` (`CLAUDE.md` — the parser matches line by line and a wrapped box can never match); `git show HEAD:<the log>` read as raw bytes carries no emoji or colour-swatch glyph in a meaning-bearing position; the section renders identically as plain text. Artifact-evidence: a reviewer maps each slot to one named primitive (`../_decomposition.md`, `#### The primitive layer to compose from — do not hand-roll`). Rejects the anti-pattern `interaction-patterns.md` names — a bespoke navigation widget over a medium that renders the equivalent for free — and a hand-off written as one undifferentiated paragraph, which satisfies every presence check and leaves HS-P0025 re-deriving the fields |
| **AC-008** | **GIVEN** a person who has never seen this story reaches HS-P0024 from `redkiln board`, and separately a person reaching HS-P0025 from the same board, **WHEN** each looks for the evidence the closeout promotion rests on, **THEN** the first reaches the filled section in **two hops** (project card → `## Companions` → the log, then the in-page `## Hand-off` anchor), and the second finds a one-line pointer in `.bklg/docs-that-teach/durable-audience-closeout/project.md`'s `## Dependencies` naming the log by path and the section by heading anchor — with **no** frontmatter hunk in either file's diff and **no** file under `.kb/` or `.kb/_intake/` anywhere in this story's diff | Static: `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/durable-audience-closeout/project.md` returns both the existing Companions row and the new pointer; `git diff` shows no hunk above either closing `---`; `git diff --name-only` returns no path under `.kb/`. Rejects the two named failures — a hand-off written into this story's own folder, a scratch note or a commit message, which is not a delivery; and a correct note that HS-P0025's card never points at, reachable only by someone who already knows it exists |

**Coverage of the traced project ACs.** `../project.md` **AC-009** — *the hand-off to closeout is
explicit* — is this story's sole-owned criterion and is carried by AC-001, AC-002, AC-004, AC-005,
AC-007 and AC-008 together: the persona, the date, the tree, the verdict, the precondition and the
reachability are the whole of what "explicit" means here. **AC-008** — *the claim is scoped in writing
wherever it is made* — is **carried, not re-owned**: AC-003 above binds this story to the same string
`scope-the-claim` authored, and `../_storymap.md`'s `## Coverage` states that split. No row above
claims project AC-006 or AC-007; AC-005 **asserts them as a precondition** it checks and refuses to
proceed without, which is a different obligation from owning them.

## Interaction quality

The blocking invariants, in two families. **Every one of them is carried by an `AC-###` row in the
table above** — this section says only which row carries which and how each is verified. Nothing here
is a free-floating bullet: `redkiln verify` extracts ACs by matching a leading `| AC-001 |` table cell
or an `- AC-001:` bullet, so an invariant stated only as prose here would get no ledger row, would
never be gated and would never be tested.

### State invariants

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — a stumble's disposition is readable at the stumble; the hand-off is an *additional*, optional hop and never the only place a disposition is legible | **AC-006** | Every citation the section makes points *outward* to an `FL-###` entry that already carries its own `Disposition:` field; the section adds no disposition of its own |
| **Non-occlusion** (IQ-2) — a derived view must not hide or own what it summarises | **AC-006** | The deletion check, stated verbatim in `../_decomposition.md`: delete the whole `## Hand-off` section and the record still carries every stumble and its disposition |
| **Preserved position** (IQ-3) — a cited anchor still resolves after the log is finalised | **AC-006** | Citations are frozen `FL-###` heading anchors, never re-descriptions. This note is the first document to cite them from *outside* the session, so it is where a renumbering or a "tidied" label would first hurt |
| **Reversibility** (IQ-4) — a hand-off is not final; a later correction appends | **AC-006** | A disposition revised after hand-off (`content-fixes-from-dispositions` may land later) gains a **dated revision line beside** the original statement, the earlier text still legible with its reason. Authority: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Keyboard reachability** — the section is navigable with browser find and stable heading anchors alone; no pointer-only affordance, no widget, no script, no rendering step | **AC-007** | Static-text completeness; `rg` finds no `<details>`, `<summary>` or `<script>`; the section reads identically in a `git diff` |
| **An illegal state is refused, not annotated** — `Stumble recorded, undispositioned` is *"legal during the session, illegal at hand-off"*; the `Handed off` state is only reachable from a closed record | **AC-005** | The precondition is a dated checked fact, and its failure withholds the note rather than qualifying it |
| **No fact is created by the act of summarising** — the note is derived; anything it needs that the record lacks goes into the record first | **AC-006** | Every sentence resolves to a `file:line` elsewhere in the log, recorded in the ledger's evidence column |

### Composition invariants

`../_design.md` is signed off (Ryan Britton, 2026-08-17, no conditions) with `hasSurface: false` and
an empty items block: **no rendered surface and no public API item**, so there is no pixel density
budget, no token file and no component — and inventing one is out of scope in its own words (*"there
is no CSS layer and no token file, and inventing one is out of scope"*, quoting `../_decomposition.md`,
`#### The primitive layer to compose from — do not hand-roll`). What it **does** carry is the named
document-primitive layer, and that is binding on this story exactly as a component library would be.

| Invariant | Real numbers / named source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — every slot carries real composed presentation, not bare markup. A `## Hand-off` heading with an undifferentiated paragraph under it is the document-medium equivalent of an unstyled render: it passes every presence check and leaves HS-P0025 re-deriving the four fields it came for | Five labelled one-line slots (persona / date / tree / scope sentence / verdict) plus the dated precondition line plus the revisions default — each a labelled field, not prose to be parsed | **AC-007**, **AC-001** |
| **Composition and placement** — every element maps to a named existing primitive; nothing hand-rolled, no new heading vocabulary | `.redkiln/templates/briefs/brief.md`'s Intent/AC/Notes spine; `.redkiln/templates/_design.md`'s `## Shape decision` table; `docs/README.md`'s two-column routing table; `.redkiln/templates/gates/discover.md`'s one-line checkbox. Placement is fixed: section **8 of 8**, last, because it is the derived view of the seven before it | **AC-007** |
| **Transience** — what is persistent chrome, what is revealed, what is opened on demand | Persistent and visible-by-default: the whole section, and the `## Chronological record` it derives from. Revealed: **nothing**. Opened on demand: **nothing** — no fold, no tab, no collapse in this file, by rule. The section is itself deletable without loss, which is the inverse property to hiding | **AC-006**, **AC-007** |
| **Density budget, with its numbers** | Exactly **5** named slots + **1** dated precondition line; **3** personas named, of which exactly **1** is marked directly observed and **2** still inferred; **2** legal verdict values; **1** scope sentence, copied not re-worded; **1** date; **1** resolvable tree id; **0** emptiness tokens remaining; **0** folds, widgets and scripts; **1** line per checkbox; **≤ 2** hops from `redkiln board`; **1** added line in the consumer's card | **AC-004**, **AC-007**, **AC-008** |
| **Hierarchy** — the reading order is the order HS-P0025 needs the facts in, so DR-5's sentence can be written top to bottom without scanning back | Persona (with the two inferred) → date → tree → scope sentence → second-session verdict → the precondition line. The scope sentence sits **before** the verdict deliberately: the qualification must be read before the decision it qualifies | **AC-001**, **AC-003**, **AC-007** |
| **Named anti-patterns** — a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold, an inactive tab or a collapsed admonition; colour or emoji as the sole carrier of meaning; a wrapped checkbox; a plausible-looking placeholder. Plus this story's own three: a fourth invented persona label; a paraphrased scope sentence; a fact that exists only in the summary | `../_decomposition.md`, `#### The primitive layer…` and `#### The accessibility floor`; `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`; `CLAUDE.md` on the line-by-line parser; decisions 1, 3 and 8 of the context pack | **AC-007**, **AC-001**, **AC-003**, **AC-006** |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | At hand-off, a severity-marked stumble carries zero dispositions, or two | **Stop; the hand-off is not written.** `../project.md` AC-006 is "exactly one" and the brief marks the undispositioned state illegal at hand-off. Route it back to `disposition-every-stumble` / `route-and-escalate` and close it there. A note that ships alongside an open item and mentions it as "outstanding" converts a bar into a comment |
| **EC-002** | A `routed:` id does not resolve to a real item, or an `escalated:` id is not in `../../_decomposition.md`'s DT ownership table | Same: **stop**, and return it to `route-and-escalate`, which owns project AC-007 and AC-011. `../_grounding.md`'s `## Routing destinations for AC-007 — verified against real project files` is the verified list; a destination outside it is unproven, not merely unusual |
| **EC-003** | The log's `## Session record` still carries the token in its date or tree field, or no session has been run | **Stop.** This story transcribes; it cannot manufacture. The blocking edge is `session-run-against-pinned-tree` (transitively, through all three named dependencies), and writing a plausible date here would be the fabricated observation `../project.md`'s risk-table row 2 exists to prevent |
| **EC-004** | `scope-the-claim` has not landed, so `## Scope of the claim` is empty | **Stop; do not invent a sentence.** Both stories are in the `scoped-claim-and-handoff` slice and implemented sequentially in one context (`../_storymap.md`, `## Merge order`, step 4). A sentence invented here and reconciled later is two claims with one owner between them |
| **EC-005** | `second-session-decision` has not landed, so no verdict exists to transcribe | **Stop.** "Not mentioned" is not a value of the slot (`../_decomposition.md`, the states list), and neither is a guess. Same slice, same sequential rule as EC-004 |
| **EC-006** | DT-9's resolved persona in `../_design.md` and what the recruited reader actually did diverge | **Do not re-decide DT-9 here.** Name the persona the session was *designed to walk*, state the divergence in one sentence (slot 1c), and let the imperfect fit be visible — the distillation already warns that the friction-log reader *"is not automatically identical to any one of the three personas"*. If the divergence is total — the reader matched none of the three — that is a **finding** for the chronological record and an escalation carrying **DT-9**, not a silent re-choice |
| **EC-007** | Writing the note surfaces a fact the log does not carry — an unrecorded reaction, an unstated reason, a severity nobody marked | It belongs in the **chronological record first**, through the story that owns it. Nothing may exist only in the summary (decision 8, IQ-2). If the record cannot be reopened at this point, the fact is omitted from the note and raised as its own item — never smuggled in as summary prose |
| **EC-008** | The second mount appears to require a change above the closing `---` of `.bklg/docs-that-teach/durable-audience-closeout/project.md` | It does not, and a `PreToolUse` hook denies the edit. `## Dependencies` is body prose; `redkiln` is the only writer of `id`, `stage`, `status`, `updated` and `links` (`CLAUDE.md`, `## Where the work lives`). A pointer that seems to need a `blocked_by` entry is a pointer being written as a link — write the prose line |
| **EC-009** | A change appears to need a fourth path — the distillation's `## Risks` bullet, a `.kb/` atom, a `.kb/_intake/` file, a sibling charter | The story has **absorbed HS-P0025's work**. Stop and route it; do not widen the fenced path block. `redkiln verify --grain story` reads that block and fails on any file changed outside it, and correcting the blanket qualification is HS-P0025's act by `../project.md`'s `## Out of scope` |
| **EC-010** | A disposition is revised after the hand-off is written — `content-fixes-from-dispositions` lands late | **Append, never overwrite.** A dated revision line goes beside the original statement with the earlier text still legible and its reason attached (IQ-4; `.kb/governance/rewrite-the-referent-never-the-reasoning.md`). This is why that story deliberately does not gate this one (`../_storymap.md`, `### Dependency graph`) |
| **EC-011** | `redkiln doctor` reports a seventh `template-drift` advisory, or fewer than six | A template was changed without a decision, or a customisation was reverted. Investigate; **never run `redkiln adopt --templates`**, which would overwrite all six customisations silently and then fail the CI assertion on the absence it created (`CLAUDE.md`) |
| **EC-012** | Someone proposes a script that "checks the hand-off exists" | Forbidden by name in the testing brief: a check confirming a `Persona:` line is present passes `Persona: TBD` as readily as a real name and rejects no wrong implementation. Every static check in this spec names the wrong implementation it rejects; a check that names none belongs in the Artifact-evidence tier, not in a new automated one |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| **NF-001** | The section is complete and legible as raw text in a `git diff`, at ordinary terminal width, with no rendering step and no recording | `../_decomposition.md`'s `#### The accessibility floor`: a screen reader and a `git diff` must both read the record. This is the reduced-motion floor stated for a medium with no motion — the obligation is that the text is complete, not that an animation is polite |
| **NF-002** | Zero occurrences of the token remain in `## Hand-off`, and the token keeps its single spelling `NOT-YET-RECORDED` | AC-004's check is an `rg` count. A second spelling makes the count meaningless and a half-filled hand-off indistinguishable from a complete one (`../friction-log-skeleton/spec.md`, NF-002) |
| **NF-003** | A reader who opens the log at this section and reads nothing else can write HS-P0025's DR-5 sentence without opening another file | The operational form of UX-AC-013 — *"in a form HS-P0025 can lift without re-deriving anything"*. It is the only quality bar that distinguishes this story from a paragraph, and it is what AC-001 through AC-004 exist to make true |
| **NF-004** | The diff is small enough to review in one sitting: one filled section plus one line of prose in a sibling card | The PR boundary is three paths on purpose. A large diff here is the signal that the story has absorbed HS-P0025's promotion work (EC-009) |
| **NF-005** | The persona name is durable as an identifier: it must survive HS-P0025's ingest pass without being re-worded, because it foreign-keys into the distillation's three headings today and into the `.kb/product/` atoms promoted from them afterwards | `.kb/product/README.md` — *"A persona nobody researched is a stock photo with a name."* A name invented here resolves to nothing on either side, which is why decision 1 fixes the vocabulary rather than leaving it to wording |
| **NF-006** | Zero files under `.kb/` and zero staged under `.kb/_intake/` in this story's diff | `../project.md`, `## Out of scope`; hand-authoring atoms outside the ingest path is a named non-goal of the initiative and the first attempt at it was reverted (`0269720`). The note is written so an atom *can* be authored from it — goal, context, observation status, evidence path — without being one |
| **NF-007** | The story adds no dependency, no script, no generated file and no new tool. `cargo xtask affected --base main` falls through to the five file-reading lints and `spec-trace` on an empty package set | `.redkiln/config.yaml:40`'s `affected_gate` comment block, and the testing brief's Notes: a story whose whole deliverable is markdown still gets a real check rather than a vacuous pass |

## Implementation notes (non-prescriptive)

Shape only — the implementer owns the wording.

- **Do the precondition check first, before writing a single slot.** It is the one thing that can stop
  the story (EC-001, EC-002), and discovering an undispositioned stumble *after* drafting a hand-off
  creates pressure to qualify the note rather than close the item. Walk `## Chronological record` end
  to end, count arms, resolve every id, and write the dated check line first.
- **Transcribe with the source open, never from memory.** Date, tree, scope sentence and verdict are
  all copies. Open `## Session record`, `## Scope of the claim` and `second-session-decision`'s record
  side by side and copy; a re-typed tree id that differs by one character is a defect that no reader
  will ever catch by reading.
- **Write the persona slot last, and write it as a sentence about a person.** "Persona 2 — The adapter
  author, who was trying to implement the storage contract against `crates/happenstance-core/`" is
  liftable; "Persona 2" alone is a label, and UX-AC-001's whole point is that the choice reads as an
  intent. The other two get one clause each: named, and marked not directly observed.
- **Say the imperfect fit plainly if there is one.** The temptation is to smooth it because the note
  reads better; the cost is that HS-P0025 promotes a cleaner observation than the evidence supports,
  which is the overclaiming failure mode the context pack names.
- **Do the mount before the polish.** The one-line pointer in HS-P0025's `## Dependencies` is two
  minutes of work and is the difference between a delivered artifact and an orphan. Write it early —
  as the last commit it gets forgotten, and AC-008 fails on a note that is otherwise perfect.
- **Keep the section a *view*.** If a sentence being written cannot be pointed at somewhere else in
  the log, that is EC-007 firing, not a writing problem. The test is mechanical: draft the ledger's
  evidence column as you write, and a slot with no `file:line` to cite is a slot inventing a fact.
- **Resist the urge to explain the log.** The reader is not being taught how the session worked; they
  are being handed four facts and a qualification. Every additional paragraph is another sentence a
  reviewer must check resolves to the record, and another chance to imply exhaustiveness by accident.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`'s `## Testing brief` — its AC/tier table (the AC-008 and AC-009
rows specifically), its Artifact-evidence definition (ledger-cited `file:line`, not eyeballing), and
its explicit instruction not to automate AC-009 into a check that nothing can fail.

| tier | command / path | proves |
| --- | --- | --- |
| Static — persona vocabulary | `rg -n "^## Persona" .bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` (lines 56, 148, 225), compared against the name written in the section | The persona is one of the three, spelled identically, and the other two are named — not a fourth invented label (AC-001) |
| Static — transcription | a diff of the section's date and tree strings against the log's `## Session record`; `git show <the cited tree id>` | The date and tree are transcribed and the tree resolves from this worktree, so a later reader can tell whether a stumble still applies (AC-002) |
| Static — scope sentence | `rg -F "<the scope sentence>"` over `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` returns hits in both `## Scope of the claim` and `## Hand-off`; `rg -i "exhaustiv"` returns nothing in the section outside a disclaiming sentence | The narrow claim travels with the summary as the same string, and nothing overreaches (AC-003) — the exact mechanism the testing brief's AC-008 row specifies |
| Static — emptiness | `rg -c "NOT-YET-RECORDED"` scoped to `## Hand-off` returns 0 | Every slot the skeleton left marked is filled; a half-filled hand-off cannot pass as a complete one (AC-004) |
| Static — precondition | per-entry arm count over `rg -n "^### FL-"`; `test -f .bklg/support/initiative.md`; `test -f .bklg/docs-that-teach/application-author-path/project.md`; `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md`; `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` | Exactly one disposition per stumble, every routed destination real, every escalated DT id a real tension — checked, not assumed (AC-005) |
| Static — anchors and plain text | every citation in the section matches an existing `^### FL-` heading; `rg -n "<details>\|<summary>\|<script>"` returns nothing; every checkbox line matches a single-line `- [ ] …`; `git show HEAD:<the log>` read as raw bytes | Citations resolve, nothing is behind a fold, the line-by-line gate parser can match, and no meaning rides on colour or emoji (AC-006, AC-007) |
| Static — mounts | `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/durable-audience-closeout/project.md`; `git diff` carries no frontmatter hunk; `git diff --name-only` returns no path under `.kb/` | The section is two hops from the board and HS-P0025's card points at it; the CLI's frontmatter is untouched and no atom was hand-authored (AC-008, NF-006) |
| Artifact-evidence | `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/_ledger.md` — one row per AC, each citing a real `file:line` into the log | The claims a reviewer must actually read: that each sentence resolves elsewhere in the record (the IQ-2 deletion check), that the fit divergence is stated, that the `declined` reason does not overclaim (AC-001, AC-003, AC-005, AC-006). `require_ledger: true` at `.redkiln/config.yaml:67` |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | Falls through to the five file-reading lints and `spec-trace` on an empty package set — a markdown-only story still gets a real check rather than a vacuous pass |
| Story verify | `redkiln verify --grain story` | No file changed outside the three-path fenced block, and every ledger row satisfied with non-placeholder evidence before `implement → report` |
| Backlog hygiene | `redkiln validate --kb && redkiln doctor` | Clean, at exactly the six standing `template-drift` advisories `CLAUDE.md` documents (`../project.md`, DoD-8) |
| Project integration bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7. **Not this story's to turn green** — it touches no crate — but it must remain green; a story that breaks `spec-trace` has broken something it never should have touched |

**Not run here, deliberately.** `cargo xtask ci` (the terminal `e2e` bar, `.redkiln/config.yaml:60`)
is HS-P0025's — this project is `terminal: false`. And the friction-log *session* is never folded into
a CI step: U1 and U2 are never mocked, scripted or simulated, because non-authorship is the mechanism
rather than a convenience (`../_decomposition.md`, `## Testing brief`, fixtures and seams). A passing
`cargo xtask ci --fast` says nothing about comprehension, and this story must not imply otherwise.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| The note is written before a dependency closes, and asserts a precondition that is not yet true | Medium / High | EC-001 through EC-005 make each a **stop** condition, not a fix-later. Two of the three dependencies are slice-mates implemented sequentially in one context; AC-005's dated check catches the third after the fact if the ordering is broken anyway |
| Overclaiming — the note reads as a research finding about the audience rather than one session's observation | Medium / High | `../project.md`'s risk-table row 2 verbatim. AC-003 carries the scope sentence as the same string and greps for `exhaustiv`; the Artifact-evidence tier exists because the *soft* overclaims ("the documentation was validated") pass the grep and fail the criterion |
| Underclaiming — the blanket qualification ships unchanged, or the persona is hedged into uselessness | Medium / High | `discover.md`'s named wrong implementation, rejected by AC-001's exact-match check plus the requirement that the other two are *marked* inferred rather than left unmentioned. A note that names no persona fails the story's only sole-owned criterion |
| A fourth, invented persona label is used because it describes the reader better | Medium / High | AC-001's check is a character-for-character match against the three `##` headings. NF-005 states the reason: the name foreign-keys into the atoms HS-P0025 will promote, and a name that matches nothing pairs with nothing |
| The note becomes a second home for evidence — a severity, reason or destination that exists only here | Medium / Medium | AC-006's deletion check, run and recorded, plus the ledger discipline of citing a `file:line` per sentence. EC-007 routes the surplus fact into the record instead |
| This story absorbs HS-P0025's correction of the distillation's `## Risks` bullet | Medium / High | The three-path fenced block plus EC-009. Editing discovery here would put half a correction in each project, and `redkiln verify --grain story` fails on the fourth path |
| A disposition is revised after hand-off and the note is quietly rewritten to match | Low / High | EC-010 and IQ-4: append a dated revision line, never overwrite. This is exactly why `content-fixes-from-dispositions` was deliberately left out of this story's `depends_on` (`../_storymap.md`, `### Dependency graph`) |
| The note is correct but unmounted, and HS-P0025 never finds it | Medium / Medium | AC-008 is a first-class criterion with a static check on both mounts, and the implementation notes put the mount early rather than last |
| Backwards coupling — the wording chosen here constrains what HS-P0025 may honestly write in a promoted atom | Medium / Medium | Deliberate and bounded: the note supplies four facts and a qualification in the vocabulary already on record, and asserts nothing about promotion, reconciliation against HS-S0131 or the evaluator question, all of which are HS-P0025's (AC-001/AC-003/AC-004 of `../../durable-audience-closeout/project.md`) |
| A well-meant automation is added that confirms a `Persona:` line exists | Low / Medium | Forbidden by name in the PR boundary, in EC-012 and in the testing brief's Notes: it passes `Persona: TBD` and rejects no wrong implementation |

## Dependencies

**Blocks on** — all three are hard edges, and none is stylistic (`../_storymap.md`, `## Merge order`,
step 4; `### Dependency graph`).

- **`route-and-escalate`** — closes the last routed destination and the last escalation. Written
  before it lands, the note asserts a precondition (AC-005) that is not yet true. It is the only one
  of the three that is **not** a slice-mate: it sits in `dispositions-and-routing`, one slice earlier.
- **`scope-the-claim`** — authors the sentence AC-003 carries verbatim. Written before it lands, the
  note invents a scope sentence that then diverges from the one the log carries, and `../_storymap.md`'s
  `## Coverage` gives the sentence exactly one owner. **Slice-mate**, lands first, either order with
  the next.
- **`second-session-decision`** — records the verdict AC-004 transcribes. Written before it lands, the
  verdict slot is a guess, and "not mentioned" is not one of the states. **Slice-mate**, lands first.

Transitively, through all three: `session-run-against-pinned-tree` (the date and the tree),
`disposition-every-stumble` (the dispositions AC-005 checks), `friction-log-skeleton` (the eight
sections and the five `## Hand-off` slots this story fills), `non-insider-recruitment` and
`dt9-and-fixed-protocol` (the persona AC-001 names).

**Deliberately not a dependency**

- **`content-fixes-from-dispositions`** — *"a fix that lands late must not be able to hold up the
  evidence HS-P0025 needs, and IQ-4's reversibility means a disposition revised after hand-off is
  recorded as a revision rather than as a rewrite of the note"* (`../_storymap.md`, `### Dependency
  graph`). EC-010 is the mechanism that makes the omission safe.

**Unlocks**

- **No story in this project.** This is the last story of HS-P0024 (`../_storymap.md`, `## Merge
  order`, step 4) and nothing downstream of it lives here.
- **HS-P0025 `durable-audience-closeout`**, at the project grain — its **DR-5** (the observation
  qualification is corrected, not copied) and its **AC-007** (the promoted atoms name the persona
  HS-P0024's session directly observed, and mark the remaining personas as still inferred) become
  liftable rather than derivable. That is the whole reason this story exists.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Open each at the moment its row names; do not preload the corpus.
Every path below was confirmed present in this worktree.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes the log's path, its eight sections in order, the five `## Hand-off` slots this story fills, the `FL-###` id contract and the `NOT-YET-RECORDED` token. This story writes *into* a shape it does not get to change | **First, before writing a single slot** — everything below assumes the shape this file fixed | AC-004, AC-006, AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/personas-and-journeys.md` | The three `##` persona headings at lines 56, 148 and 225 are the naming vocabulary AC-001 matches character-for-character; its `## Risks` (line 317) is the warning that the recruited reader is not automatically identical to any one of them | Before writing slot 1 and slot 1c — the name is copied from here, and the fit caveat is judged against here | AC-001 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off design (Ryan Britton, 2026-08-17, no conditions): `hasSurface: false`, the empty items block, the "no CSS layer, no token file, inventing one is out of scope" bound — and, once `dt9-and-fixed-protocol` has landed, the record of **which persona DT-9 resolved to**, which this story reads and never re-decides | Before AC-001 (the persona is read from here), and again before AC-007 (the composition bound) | AC-001, AC-007 |
| `.bklg/docs-that-teach/durable-audience-closeout/project.md` | The consumer's obligations this note exists to make dischargeable: **DR-5** at line 153 and **AC-007** at line 196. Its `## Dependencies` (line 222) is the second mount point | Before AC-001, to know exactly what shape the lift must take; again at the mount, before AC-008 | AC-001, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief holds IQ-1…IQ-7 verbatim, the states enumeration (`Stumble recorded, undispositioned` illegal at hand-off; `Handed off`; the two verdict values), the accessibility floor, the named primitive layer and **UX-AC-013**, the bar this story is measured against. The Testing brief holds the AC-008/AC-009 rows and the do-not-over-automate note | Before AC-003, AC-005, AC-006 and AC-007 — the invariant wording is quoted, not paraphrased | AC-003, AC-005, AC-006, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | **AC-009** is this story's sole-owned criterion and **AC-008** the carried one; risk-table row 2 is the overclaiming risk AC-003 answers; DoD items 4 and 5 are what this story completes; `## Companions` is the first mount's render path | Before enumerating evidence for AC-003 and AC-005; again when confirming the two-hop reach | AC-003, AC-005, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | `## Coverage` states the AC-008 split — `scope-the-claim` owns the sentence, this story carries it — and `## Merge order` step 4 fixes that this story lands **last**, which is what makes AC-005's precondition checkable end to end | Before starting, to confirm the boundary; again if the work starts to feel like it should re-author the scope sentence | AC-003, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/second-session-decision/spec.md` | Fixes the verdict vocabulary (`run` \| `declined — <reason>`), states that "not mentioned"/"TBD"/a deleted slot are the same failure, and names the forbidden `declined` reasons that buy the decline with an overclaim | Before writing slot 5 — the verdict is transcribed from what this story recorded, and its reason must not reintroduce an exhaustiveness claim | AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/spec.md` | The owner of the scope sentence and of the `rg` mechanism AC-003 reuses. The string is copied from what this story wrote, never re-derived from BR-14 | Before writing slot 4 | AC-003 |
| `.bklg/docs-that-teach/comprehension-evidence/route-and-escalate/spec.md` | Defines what "every routed destination resolves" means in practice and the id vocabulary the precondition check walks; it is the third blocking edge and the one that is not a slice-mate | Before performing AC-005's precondition check | AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | Lines 72–76 are Nielsen & Landauer's small-sample basis — the cited reason the narrow claim is defensible and exhaustiveness is not; `## Routing destinations for AC-007 — verified against real project files` (line 94) is the verified destination list AC-005 checks against | Before AC-003 (why the sentence says what it says) and before AC-005 (which destinations are already proven real) | AC-003, AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding on this work: the referent may be rewritten, the reasoning may not be erased. It is the authority for appending a dated revision line rather than editing the note when a disposition changes after hand-off | Before implementing the revision shape, and immediately if `content-fixes-from-dispositions` lands late | AC-006 |
| `.bklg/docs-that-teach/_decomposition.md` | `### Definition of Done` row 15 is the initiative scenario this story feeds; `### Why the load-bearing edges exist` carries this story's obligation verbatim; `## Design tension ownership` is the DT-id vocabulary AC-005 validates escalations against | Before AC-005's escalation check, and when writing the executive framing of what HS-P0025 must not re-derive | AC-001, AC-005 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `## Anti-patterns` names the bespoke navigation widget over a medium that renders the equivalent for free, and the load-bearing item behind a fold — the two composition failures AC-007 forbids | Before adding any navigational affordance, index or fold to the section — the answer is almost always a link | AC-007 |
| `.kb/product/README.md` | *"A persona nobody researched is a stock photo with a name."* States the four fields a persona atom must carry (goal, context, current alternative, fear) — the note is shaped so those are liftable **without** authoring an atom here | Before writing slot 1, to know what HS-P0025 will need from it; and whenever the temptation to write the atom appears | AC-001 |
| `.redkiln/config.yaml` | Line 5 `support_initiative: support` (a routed destination AC-005 resolves), line 40 `affected_gate`, line 55 `integration_scoped`, line 67 `require_ledger: true` | At the precondition check, and again when running the gate | AC-005, AC-008 |
| `.redkiln/templates/gates/discover.md` | Lines 9–13 are the one-line checkbox primitive; the gate parser matches line by line and a wrapped box can never match | Only if the precondition check is written as checkboxes rather than a sentence | AC-007 |
| `docs/README.md` | The live two-column routing table — the existing primitive any tabular slot in the section reuses rather than replaces | While composing the section's layout | AC-007 |

## Clarifications resolved during spec

1. **The AC ids are exactly the eight the front half enumerated** — AC-001 … AC-008 — with none added
   and none dropped. The mapping is: AC-001 the persona (and the two still inferred, and the fit),
   AC-002 the date and tree, AC-003 the scope sentence carried verbatim, AC-004 the second-session
   verdict, AC-005 the precondition checked and recorded, AC-006 derived/deletable/anchored/appending,
   AC-007 composition and static-text legibility, AC-008 both mounts. The ledger carries the same eight.

2. **The emptiness token is inherited, not re-decided.** `NOT-YET-RECORDED` is fixed by
   `../friction-log-skeleton/spec.md`'s own clarification 1; this spec neither renames it nor adds a
   second spelling, and NF-002 requires zero of them left in `## Hand-off`.

3. **Slot 1c is conditional, and its absence is meaningful.** The fit-divergence sentence is written
   only where the recruited reader matched the chosen persona partially. Its **absence therefore
   asserts a clean fit**, which is stated here so that a missing sentence is not read as an oversight
   — the same discipline `friction-log-skeleton` applied to a log with zero interventions.

4. **"Two hops from `redkiln board`" is counted explicitly.** Hop one is the project card's
   `## Companions` row; hop two is the log itself. The `## Hand-off` heading is an **in-page anchor**,
   not a third hop. AC-008 is written against that count so a reviewer and an implementer cannot
   disagree about whether the bar was met.

5. **The precondition may be written as a sentence or as checkboxes; if checkboxes, one line per box.**
   The spec does not prescribe which, because `_design.md` names both the brief spine and the one-line
   checkbox as available primitives. What is fixed is that the check carries the **date it was
   performed** — an undated assertion cannot be distinguished from an assumption.

6. **The second mount is body prose, and it does not create a link.** The pointer in HS-P0025's
   `## Dependencies` is a sentence; it is not a `blocked_by` entry, because the CLI is the only writer
   of an item's system frontmatter and a `PreToolUse` hook denies the edit (`CLAUDE.md`). A reviewer
   should expect `git diff` to show no hunk above that file's closing `---`.

7. **No verifying test is a compiled test, and that is honest rather than a shortfall.** This story
   adds no `pub` item and touches no crate, so every criterion is verified by the Static and
   Artifact-evidence tiers the project's own testing brief defines, plus `cargo xtask affected --base
   main`'s fall-through. The brief's warning is respected: every static check above names the wrong
   implementation it rejects, and the one check that could reject nothing — "a `Persona:` line
   exists" — is deliberately not written (EC-012).

8. **This story renders no `_design.md` item id, because `_design.md` declares none.** It completes
   **surface 2** of the three *document* surfaces the UX brief enumerates — the friction log. The
   integration contract already records this; it is restated so a reviewer does not read the empty
   items block as a missed binding.

9. **`## Hand-off` is filled, not extended.** The section already exists with five named slots and
   the token in each. This story replaces the tokens and adds the dated precondition line; it adds no
   `##` or `###` heading, renames none, and reorders none. A sixth slot would be a change to the
   skeleton's contract and belongs to that story, not this one.
