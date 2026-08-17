---
item: HS-S0170
stage: spec
created: 2026-08-17T13:16:22.711Z
updated: 2026-08-17T13:16:22.711Z
template_sig: 87bbf1d0
rendered_sig: aec0ae3f
---

# Spec — Answer the second-session question explicitly

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-14 (line 349); the assumption this story converts into a recorded decision: *"One comprehension session is enough for the narrow claim being made. If the first session's findings are dominated by a single blocking defect, a second may be owed"* (line 637) |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `### Definition of Done` rows 5 and 6 (HS-P0024's two owned scenarios); the AC table's AC-09/AC-10 rows; `## Design tension ownership` (the DT-id vocabulary an escalation would use) |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — **AC-010** is this story's sole traced criterion; derived requirement 10 (from BR-14) is its wording; the risk table's *"One session's findings are dominated by a single blocking defect, making the rest of the evidence thin"* row is the risk it discharges; DoD-7 is its integration bar |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/second-session-decision/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (`#### The states this surface has to express`, the **Second-session question answered** state: *"Run / declined-with-reason. 'Not mentioned' is not one of the states"*; IQ-1, IQ-2, IQ-3, IQ-4; the accessibility floor) and `## Testing brief` (the AC/tier table's AC-010 row: Artifact-evidence, ledger-cited; the *"do not automate into a check that nothing can fail"* note; the pinned-tree-as-fixture paragraph) |
| Grounding | [`../_grounding.md`](../_grounding.md) — lines 72–76, Nielsen & Landauer: one qualitative session finds ~1/3 of problems and each problem a real reader hits is already proven. The stated grounding for both BR-14/AC-008's scoped claim **and** AC-010's "was one session enough" assessment |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, approved by the repository owner 2026-08-17 with no conditions. It declares **no** public API item and **no** rendered UI surface, and (once `dt9-and-fixed-protocol` amends it) carries the severity scale whose blocking end this story's dominance test is expressed against |
| Consumed contract | [`../friction-log-skeleton/spec.md`](../friction-log-skeleton/spec.md) — fixes the log at `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, its eight frozen sections, and the `## Hand-off` slot this story writes: *"the second-session verdict (`run` \| `declined — <reason>`; 'not mentioned' is not a state)"* |
| Roadmap pointer | [`../_storymap.md`](../_storymap.md) — `## Merge order`, step 4 (`scoped-claim-and-handoff`): `scope-the-claim` and **this story** in either order, then `handoff-note-to-closeout` last |

## One-line PR slice

Assess whether the findings were dominated by a single blocking defect and record the verdict — a
second session run, or declined with a stated reason — so silence never stands in for the decision.

## Executive summary

This PR writes **one slot** in an existing file: the second-session verdict in
`_friction-log.md`'s `## Hand-off` section, together with the short assessment the verdict follows
from. It adds no section, renames none, reorders none, and touches no stumble entry.

The delta against the project charter is the part AC-010 leaves open, and it is the only part worth a
story. The charter says the question is "assessed and recorded, with a second session either run or
declined for a stated reason" (`../project.md`, AC-010) — it does not say **what "dominated by a
single blocking defect" means**, and an unstated test is how a verdict becomes a mood. So this PR
fixes a dominance test made of facts the log already carries (how many severity-marked stumbles
exist, how many sit at the blocking end of `_design.md`'s named scale, whether the session ended in
`Kind: abandonment`, whether the scenario's stated goal was reached), requires the verdict to record
the counts it was computed from so a reviewer can recompute rather than defer, and fixes what each
verdict *costs*: a `declined` verdict must state its consequence without overclaiming, and a `run`
verdict opens a **new fixture** rather than reopening this log.

It decides **no** protocol content, **no** disposition, and **no** scope sentence. The scope
sentence is `scope-the-claim`'s (project AC-008); the persona/date/tree hand-off is
`handoff-note-to-closeout`'s (AC-009). This story owns exactly one of the eight states the UX brief
enumerates: *"Second-session question answered. Run / declined-with-reason. 'Not mentioned' is not
one of the states."*

## Context pack

**Read this section and you can start. Everything below `## Anchors` is deferred depth, not optional
depth — open an anchor when its bound AC says to.**

### The decision this story exists to make

**The wrong implementation, named at discovery, is a second session scheduled by default, or refused
by default, rather than decided against what the first one found** (`discover.md`, `## The wrong
implementation`). Both defaults produce the same artefact — a line of text saying "no second
session" or "second session planned" — and neither is a decision. What separates a decision from a
default is that the verdict is *derived* from stated facts about the log, and the reader can
recompute the derivation without asking anyone.

The initiative states the assumption this story tests in the conditional, not the absolute: *"One
comprehension session is enough for the narrow claim being made. If the first session's findings are
dominated by a single blocking defect, a second may be owed"* (`../../initiative.md`, line 637).
"May be owed" is precisely the residual an unassessed project resolves by silence.

### The persona-journey slice this realizes

**U3, the downstream actor** — a sibling-project owner, and above all HS-P0025
`durable-audience-closeout` — is who this slot is written for. The UX brief's three-user table says
what must never happen to them: *"Reaching an item whose destination is a description rather than an
id, or whose disposition is implied by silence."* Applied here: HS-P0025 opens the hand-off, reads
how much evidence it is inheriting, and must be able to tell whether one session was judged
sufficient or merely never questioned. A missing verdict reads identically to a considered "no", and
only one of those is honest.

**U1 and U2 never see this slot.** The reader is gone; the facilitator is no longer facilitating.
This is a post-session act, which is why the story sits in `scoped-claim-and-handoff` and not in
`observed-session`.

### The decisions this story must honor, stated as decisions

1. **The verdict is one of exactly two states, and absence is not a third.** `run` or
   `declined — <reason>`. The UX brief's states enumeration is explicit: *"Second-session question
   answered. Run / declined-with-reason. 'Not mentioned' is not one of the states"*
   (`../_decomposition.md`, `#### The states this surface has to express`). The skeleton landed this
   slot carrying the emptiness token; this story is what replaces the token, and a slot still
   carrying it at hand-off is the failure state, not an in-progress state.

2. **The verdict follows the assessment, and the assessment is recorded with its inputs.** The
   charter's own wording puts the assessment first: *"Whether the findings were dominated by one
   blocking defect **is assessed and recorded**, with a second session either run or declined for a
   stated reason"* (`../project.md`, AC-010). Recording only the conclusion converts a checkable
   claim into an assurance — the exact substitution the testing brief refuses when it defines
   Artifact-evidence as ledger-cited `file:line`, *"not a euphemism for 'someone eyeballs it'"*
   (`../_decomposition.md`, `## Testing brief`).

3. **The dominance test is made of facts the log already carries.** Four, and no more, because each
   is readable off `_friction-log.md` without judgment: (a) the count of severity-marked stumbles;
   (b) how many sit at the **blocking end of the scale `_design.md` named** — with Nielsen 0–4 the
   supplied candidate and 4/3 its blocking end, but the token is whatever the protocol fixed and the
   record names it; (c) whether the session terminated in an entry of `Kind: abandonment`; (d)
   whether the reader reached the stated goal of `## Scenario`. A verdict computed from anything
   else — how the session *felt*, how much time remained, whether a reader is available — is not
   this test, and if it is used it is stated as such rather than dressed as an assessment.

4. **The assessment is computed against a fully dispositioned log, which is why this story blocks on
   `disposition-every-stumble`.** Until every severity-marked item carries exactly one disposition
   (project AC-006), the finding set is still moving, and a dominance ratio computed over a moving
   set is arithmetic about nothing. That single edge is the whole reason for the dependency; it is
   not an ordering convenience.

5. **`declined` states its consequence, and never buys the decline with an overclaim.** BR-14 is the
   binding constraint on the *reason text*: the evidence supports "real stumbles were captured and
   are traceable", never exhaustiveness (`../project.md`, AC-008). So *"one session was enough to
   find the problems"* is a forbidden reason and *"the findings were not dominated by one blocking
   defect; the small-sample basis supports the narrow claim already made"* is the shape a legitimate
   one takes. The grounding for that basis is Nielsen & Landauer, cited in `../_grounding.md:72-76`:
   one session finds ~1/3 of problems, and each problem a real reader hits is already proven worth
   fixing.

6. **A second session opens a new fixture; it does not reopen this one.** The testing brief states
   the discipline directly, borrowing the conformance-suite rule: *"one commit or merge point is one
   session's fixture (AC-005 records which), and a second session (AC-010) opens a new fixture
   instance rather than reopening the first one's"* (`../_decomposition.md`, `## Testing brief`,
   fixtures and seams; `CLAUDE.md`, `## The rule that matters`). Concretely: a second session's
   entries may **not** be appended into this log's `## Chronological record` under the same `FL-###`
   sequence, and this PR does not run one. A `run` verdict records the decision and where the second
   session's own record will live; the running is a separate recorded act.

7. **A second session that is owed but cannot be had is `declined` with *that* as the reason — the
   bar does not move.** This is the same discipline the project applies to the no-reader-found state:
   *"If no reader is found, the artefact is unmet — that is a failure of the initiative, not a reason
   to redefine the bar"* (`../project.md`, risk table, row 1). The honest record says a second
   session was owed, was not run, and that the evidence is correspondingly thinner; it does not
   retro-fit a dominance assessment that concludes the opposite.

8. **Write once, revise beside — never overwrite (IQ-4).** If the verdict changes after it is first
   written, the earlier verdict and the reason for the change stay legible. This is
   [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
   applied to the hand-off: *"the referent may be rewritten, the reasoning may not be erased."*

9. **Touch nothing else in the log (IQ-3).** The eight sections are frozen in name and order; every
   `FL-###` heading line is frozen at write time because two sibling projects have already declared
   they will cite into this log by reference (`../_decomposition.md`, `## UX brief`, `### Notes`,
   *Escalation has a shape already on record*). A re-worded heading breaks a citation with no error
   message.

10. **The verdict lives in one place.** IQ-1 allows an index but forbids requiring the hop; a
    *second* document that also states the verdict is worse than either — it is two claims that can
    drift. `handoff-note-to-closeout` carries the verdict forward **by reference to this slot**, and
    HS-P0025 reads it there.

11. **This story authors nothing under `.kb/`.** Hand-authoring atoms outside the ingest path is a
    named non-goal of the initiative and the first attempt was reverted (`0269720`) (`../project.md`,
    `## Out of scope`). A verdict is not a persona claim and does not become one here.

### What this story is explicitly not deciding

The scope sentence (`scope-the-claim`, project AC-008), the persona/date/tree hand-off
(`handoff-note-to-closeout`, AC-009), any disposition (`disposition-every-stumble`,
`route-and-escalate`), and any content fix (`content-fixes-from-dispositions`). It also does not
decide the severity scale — that is `dt9-and-fixed-protocol`'s, resolved in `_design.md` under the
human sign-off gate, and this story reads the blocking end off it rather than naming one.

### The one ordering constraint that can invalidate this work

`disposition-every-stumble` must be committed first (decision 4). If the assessment is computed while
stumbles are still undispositioned, the counts it records will not match the log a reviewer later
reads, and the recorded derivation — the only thing that makes this a decision rather than an
assertion — becomes false in a way that no gate will catch, because nothing recomputes it
automatically and nothing should.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice: U3 opens the hand-off and finds the question answered (`../_storymap.md`, slice table) |
| **Slice / milestone** | `scoped-claim-and-handoff` |
| **Slice-mates** | `scope-the-claim` (HS-S0169 — independent of this story; either order) and `handoff-note-to-closeout` (HS-S0171 — lands **after** this story and carries its verdict forward by reference). Implemented together in one context and mounted as one integrated surface (`../_storymap.md`, `## Merge order`, step 4) |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` → its `## Hand-off` section, the second-session verdict slot the skeleton landed. That section is the render path HS-P0025 and every downstream reviewer actually read; the log itself is already reachable in one hop from `../project.md`'s `## Companions` list, mounted there by `friction-log-skeleton`. A verdict written anywhere else — a story report, a commit message, a companion note — is constructed-but-unmounted |
| **Wires into** | `_friction-log.md`'s `## Chronological record` (the `FL-###` ids and `Severity:` tokens the assessment counts, cited by id — read, never edited); its `## Scenario` (whether the stated goal was reached); its `## Session record` (the tree and session date the fixture is identified by, per project AC-005); its `## Scope of the claim` (`scope-the-claim`'s sentence, which the decline reason must be compatible with and must not restate); `../_design.md`'s severity scale (the blocking-end token the test is expressed against); `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract, which is what makes citing an `FL-###` anchor worth anything |
| **Design-system primitives consumed** | Document primitives only, per `../_design.md` (*"there is no CSS layer and no token file, and inventing one is out of scope"*): `.redkiln/templates/_design.md`'s `## Shape decision` table — the repository's existing primitive for recording a decision **with the alternative that lost** — and the one-line checkbox from `.redkiln/templates/gates/`, if any box is written. No new heading vocabulary, no fold, no widget, no table of contents |
| **Renders surfaces** | **None as an `_design.md` item id** — `../_design.md`'s `## Items` block is empty (`# no items — no public API surface, no rendered UI surface`) and every shape section reads `N/A — no user-facing surface`. This story changes surface **2** of the three *document* surfaces the UX brief enumerates (*"The friction log — a dated markdown artifact whose reader is a reviewer, a sibling-project owner, and eventually HS-P0025"*), which `_design.md` names in prose rather than as an item |
| **Conformance rule(s) / clause(s)** | None, deliberately. This story adds no `pub` item, compiles nothing, touches no crate under `crates/` and discharges no `spec/SPECIFICATION.md` clause; it is not adapter-observable. Its instruments are the Static and Artifact-evidence tiers the project's own testing brief defines (`../_decomposition.md`, `## Testing brief`, AC-010 row: *"Ledger cites the assessment and its verdict (run / declined-with-reason)"*), plus `cargo xtask affected --base main`'s fall-through to the five file-reading lints and `spec-trace` (`.redkiln/config.yaml:40`) |
| **Advances DoD scenario** | **Initiative DoD scenario 5** — *"A non-author, non-insider completes a stated scenario; auditable log"* (`../../_decomposition.md`, `### Definition of Done`, row 5; owner HS-P0024). This story does not turn it green — `session-run-against-pinned-tree` does — but it closes the last state that scenario's own log can be left in undetermined, and it is what stops the initiative-level assumption at `../../initiative.md:637` from being carried forward unexamined into HS-P0025's terminal re-observation |

**Delivered mounted, not as an isolated component.** The acceptance bar for this PR includes that a
person arriving at HS-P0024 from `redkiln board`, who has never read this story, reaches
`_friction-log.md` in one hop from the project card and finds the second-session question answered
inside `## Hand-off` — not in a story report they would have to know to look for.

## PR boundary

**In this PR**

- The second-session **assessment** and **verdict**, written into `_friction-log.md`'s `## Hand-off`
  section, in the slot `friction-log-skeleton` landed for them — replacing the emptiness token.
- The counts and `FL-###` citations the assessment was computed from, recorded beside it.
- This story's own `_ledger.md` and its stage artifacts.

The implementer **may** also touch the wiring named in the Integration contract above to keep the
slice mounted — that is not scope drift. In practice there is nothing to wire: `friction-log-skeleton`
already mounted the log at `../project.md`'s `## Companions`, and this story writes into a slot that
already exists.

**Explicitly not in this PR**

- **Running a second session.** If the verdict is `run`, that session is a new fixture with its own
  record and its own recorded act (decision 6). Appending its entries into this log's
  `## Chronological record` is forbidden, not merely discouraged.
- **The scope sentence** (`scope-the-claim`, project AC-008) and **the persona/date/tree hand-off**
  (`handoff-note-to-closeout`, AC-009). This story writes neither, and restating either here would
  create a second summary that can drift from the approved one.
- **Any disposition, any revision to one, any routing or escalation.** Those are
  `disposition-every-stumble`'s and `route-and-escalate`'s. If the assessment reveals a stumble whose
  disposition looks wrong, that is a finding routed to those stories, not fixed here.
- **Any edit to an existing `FL-###` entry, any section added, renamed or reordered, any id
  renumbered.** IQ-3; the heading line is a public identifier.
- **Any edit under `crates/`, `docs/`, `examples/`, `spec/` or `standards/`.** Content fixes are
  `content-fixes-from-dispositions`'s.
- **Any file under `.kb/`.** Atom authorship is closeout's, through the ingest path.
- **Any automation that "checks" a verdict exists.** The testing brief forbids exactly this shape by
  name: a script confirming a line is present would pass `"noted"` as readily as a real verdict, and
  so rejects no wrong implementation (`../_decomposition.md`, `## Testing brief`, Notes).

**Merge DoD one-liner** — `_friction-log.md`'s `## Hand-off` carries a second-session verdict of
`run` or `declined — <reason>` with no emptiness token left in that slot, the assessment beside it
records the four dominance inputs with the `FL-###` ids they were counted from, no other line of the
log changed, `cargo xtask affected --base main` green and `redkiln validate --kb && redkiln doctor`
clean at exactly the six standing `template-drift` advisories `CLAUDE.md` documents.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/second-session-decision/**
```

Two paths, and the first is one section of one file. If a change here appears to need a third, the
story has absorbed a slice-mate's work or a second session — split it rather than widening the block.

## Behavior and interfaces

The "interface" here is the shape of a recorded decision: what a reviewer six months later can
recompute, and what they must take on trust.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Where the verdict is written** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, inside `## Hand-off`, in the second-session slot the skeleton landed. Nowhere else, and in no second document. **If `_design.md`'s protocol section or the as-built skeleton fixed a different path or slot name, that wins verbatim** and this row is discarded in its favour | `../friction-log-skeleton/spec.md`, `## Behavior and interfaces` (the `## Hand-off` row and EC-001); `../_storymap.md`, `### Why the slices fall here` (path and heading vocabulary are `_design.md`'s) |
| **The verdict is a two-valued token, and absence is a failure** | Exactly one of `run` or `declined — <reason>`. The emptiness token the skeleton put in this slot is replaced, not supplemented. "Not mentioned", "TBD", "n/a" and a silently deleted slot are all the same failure | `../_decomposition.md`, `#### The states this surface has to express` (*"'Not mentioned' is not one of the states"*); `../project.md`, AC-010 |
| **The assessment is recorded above the verdict, with its inputs** | A short block stating the four dominance inputs as numbers and facts: total severity-marked stumbles; how many carry the blocking-end token of `_design.md`'s named scale (naming the token); whether the session terminated in an entry of `Kind: abandonment`; whether the reader reached the stated goal of `## Scenario`. Each count cites the `FL-###` ids it was computed from, so a reviewer recomputes rather than defers | `../project.md`, AC-010 (*"is assessed and recorded"*); `../_decomposition.md`, `## Testing brief`, AC-010 row and the Artifact-evidence definition |
| **The dominance test, stated once and applied as written** | *The findings are dominated by a single blocking defect when one stumble at the blocking end of the named scale both (a) accounts for the session ending — the log terminates in `Kind: abandonment` at that stumble or the reader never reached `## Scenario`'s stated goal — and (b) leaves the remaining severity-marked stumbles too few to carry the narrow claim on their own.* Written into the log beside the assessment so the test and its application are read together, and so the test cannot be re-worded to fit the verdict afterwards | `../../initiative.md:637`; `../_grounding.md:72-76` (Nielsen & Landauer); `../project.md`, risk table, *"One session's findings are dominated by a single blocking defect"* row |
| **Computed against a fully dispositioned log** | Every severity-marked stumble carries exactly one disposition before the counts are taken (project AC-006, `disposition-every-stumble`'s). A count taken over a moving finding set is arithmetic about nothing | `../project.md`, AC-006; `../_storymap.md`, `### Dependency graph` (this story's only inbound edge) |
| **A `declined` reason states the consequence and does not overclaim** | The reason names what the decline costs: the evidence rests on one session, the narrow claim stands unchanged, and nothing about exhaustiveness is asserted anywhere. *"One session was enough to find the problems"* is a forbidden reason; *"the findings were not dominated by one blocking defect; the small-sample basis supports the narrow claim already made"* is the legitimate shape. The verdict must be compatible with `## Scope of the claim` and must not restate it | `../project.md`, AC-008 and derived requirement 9; `../../initiative.md:349` (BR-14); `../_grounding.md:72-76` |
| **"Owed but not runnable" is `declined`, with that as the stated reason** | The bar does not move to make the record comfortable. The log says a second session was owed, was not run, and that the evidence is correspondingly thinner — the same discipline the project applies when no reader can be recruited | `../project.md`, `## Risks and coupling notes`, row 1 (*"a failure of the initiative, not a reason to redefine the bar"*) |
| **A `run` verdict opens a new fixture** | The second session does **not** append into this log's `## Chronological record` and does not continue its `FL-###` sequence. The verdict records the decision and where the second session's own record will live; running it is a separate recorded act outside this PR | `../_decomposition.md`, `## Testing brief`, fixtures and seams; `CLAUDE.md`, `## The rule that matters`; `../session-run-against-pinned-tree/spec.md:116-117` |
| **Revision, never overwrite (IQ-4)** | If the verdict changes after it is first written, the change appends beside the original with the earlier verdict and the reason still legible. The original text is never edited in place | [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md); `../_decomposition.md`, IQ-4 / UX-AC-008 |
| **Nothing else in the log moves (IQ-3)** | No section added, renamed or reordered; no `FL-###` heading re-worded; no entry, severity or disposition edited. The diff is confined to `## Hand-off` | `../_decomposition.md`, IQ-3 / UX-AC-007; `../session-run-against-pinned-tree/spec.md:536` (the same composition invariant, stated for the session story) |
| **Readable at the hand-off, no hop (IQ-1)** | A reviewer reading `## Hand-off` sees the assessment, the test and the verdict together. Any `FL-###` citation points outward for detail and is optional; the verdict itself never lives behind that pointer | `../_decomposition.md`, IQ-1 / UX-AC-005 |
| **Complete as static text** | The verdict and its assessment read correctly in a `git diff` and under a screen reader: no colour, no emoji, no fold, no `<details>`, no widget, no rendering step. Any checkbox stays on one line, because the gate parser matches line by line and a wrapped box can never match | `../_decomposition.md`, `#### The accessibility floor`; `CLAUDE.md`, `## Where the work lives`; `.redkiln/templates/gates/` |
| **Composed from the `## Shape decision` primitive, not a new format** | The decision is recorded with the alternative that lost — the repository's existing primitive for exactly that is `.redkiln/templates/_design.md`'s `## Shape decision` table. Reuse it rather than inventing a verdict block | `../_decomposition.md`, `#### The primitive layer to compose from — do not hand-roll`; `../_design.md`, `## Items` |
| **`handoff-note-to-closeout` carries it forward by reference** | HS-S0171 cites this slot rather than restating the verdict, so there is one claim and not two that can drift. This story writes nothing in the hand-off note's own persona/date/tree slots | `../_storymap.md`, coverage rows for AC-009/AC-010; `../_decomposition.md`, IQ-1 |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds no crate, no type, no table and no
connection; `../_design.md` records `hasSurface: false` with an empty items block for exactly this
reason, and the whole deliverable is prose inside one section of one markdown file.

Two data-shaped contracts are consumed rather than defined, and both are read-only here:

- **The `FL-###` stumble id.** It behaves as a primary key other documents foreign-key into, is
  assigned in occurrence order, is never reused or reassigned, and its heading line is frozen at write
  time (`../friction-log-skeleton/spec.md`, `## Data and migrations`). This story **cites** those ids
  in its assessment and creates, renames and renumbers none of them. Renumbering is not a migration;
  it is a broken citation with no error message.
- **The session fixture identity** — the commit or merge point recorded in `## Session record` under
  project AC-005. A second session is a *new* fixture instance, so the only "migration" available if
  the verdict is `run` is a new record elsewhere, never an extension of this one.

## Acceptance criteria

Seven, each framed from the intent of the person it serves — above all **U3, the downstream actor**
(a sibling-project owner, and specifically HS-P0025 `durable-audience-closeout`), whose stated
failure mode is *"reaching an item… whose disposition is implied by silence"*
(`../_decomposition.md`, `## UX brief`, the three-user table). Each crosses the whole slice: from the
dispositioned record, through the assessment, to the verdict a stranger reads at the hand-off.

`<log>` throughout is the friction log `friction-log-skeleton` lands at
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; **where `_design.md` or the as-built
skeleton fixed a different path or slot name, that substitutes verbatim in every row below**
(`## Integration contract`; EC-007).

"Verification" follows the project testing brief's tier split (`../_decomposition.md`,
`## Testing brief`): **Static** where the claim is an `rg` / `git` / deletion fact, and
**Artifact-evidence** where it is genuinely reviewer-read and carried by the `_ledger.md`
`file:line` discipline `require_ledger: true` (`.redkiln/config.yaml:67`) already enforces. The
brief's warning binds every row: a check that would pass `"noted"` as readily as a real verdict
rejects no wrong implementation, so every static check below asserts on **content**, not on
structural presence.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** U3 opens `<log>`'s `## Hand-off` to find out how much evidence they are inheriting, and has never read this story, **WHEN** they look for the second-session question, **THEN** the slot carries exactly one of `run` or `declined — <reason>`, readable in place with no hop and no fold, with the skeleton's not-yet-recorded token gone from that slot — and **NOT** absent, blank, deleted, `TBD`, `n/a`, `noted`, "no second session planned", or stated only in a story report, a commit message or a companion note. The verdict appears in exactly one place in the repository; `handoff-note-to-closeout` cites this slot rather than restating it. | **Static (content).** `rg -n "second.session" <log>` returns the slot with one of the two arms; the not-yet-recorded token named in `## Status` no longer matches inside `## Hand-off` (`rg -n "<token>" <log>` shows no hit in that section); `rg -ni "TBD\|not mentioned\|noted\|n/a" ` against the `## Hand-off` block returns nothing. **Artifact-evidence:** ledger cites the verdict line by `file:line`. Traces project **AC-010**. |
| AC-002 | **GIVEN** a reviewer six months out must be able to disagree with the verdict rather than defer to it, **WHEN** they read the block immediately above the verdict, **THEN** the assessment states all four dominance inputs as facts read off this log — (a) the count of severity-marked stumbles, (b) how many carry the blocking-end token of the scale `_design.md` named, naming the token, (c) whether the record terminates in an entry of `Kind: abandonment`, (d) whether the reader reached the stated goal of `## Scenario` — each count citing the `FL-###` ids it was computed from, so the reviewer can recompute without asking the logger anything. It is **NOT** a conclusion presented alone, a mood ("the session went fine"), an appeal to time or reader availability dressed as an assessment, or a count with no ids behind it. | **Static (content).** Each of the four inputs is present with a value; `rg -o "FL-[0-9]{3}" ` over the assessment block returns ids, and every id it returns resolves to an `### FL-` heading in `## Chronological record`. A reviewer recount of `rg -c "^### FL-" <log>` and of the blocking-end token matches the recorded counts. **Artifact-evidence:** ledger cites the assessment block by `file:line`; the reviewer confirms the recomputation succeeded. Traces project **AC-010**. |
| AC-003 | **GIVEN** the verdict must be a decision and not a default (`discover.md`, `## The wrong implementation`), **WHEN** the reviewer reads the assessment, **THEN** the dominance test is written out beside it in one place — *one stumble at the blocking end of the named scale both accounts for the session ending (abandonment at that stumble, or the stated goal never reached) and leaves the remaining severity-marked stumbles too few to carry the narrow claim on their own* — and the recorded verdict follows from applying that test to the four inputs, visibly and in the same block. It is **NOT** a test stated only in this spec, re-worded to fit the conclusion, or replaced by a bare "we judged that…". | **Artifact-evidence (reviewer-read, ledger-cited), with static support.** `rg -ni "dominat" <log>` returns the stated test inside `## Hand-off`; the reviewer reads test → inputs → verdict as one block and confirms the verdict is entailed by the inputs under the test as written. `git log -p --follow <log>` shows the test and the verdict landing together, not the test edited after the verdict. Traces project **AC-010**. |
| AC-004 | **GIVEN** a count taken over a moving finding set is arithmetic about nothing, **WHEN** the assessment is computed, **THEN** `<log>` already carries exactly one disposition arm on every severity-marked stumble — zero entries reading `not yet dispositioned` — and the counts the assessment records match what a reviewer counts in the log as it stands at merge; **NOT** computed against a partially dispositioned record, and **NOT** repaired afterwards by editing a severity mark or a disposition to change a count. | **Static.** `rg -n "Disposition:" <log>` returns zero `not yet dispositioned` arms; the recorded counts equal a fresh `rg`-count over the same log; `git log -p --follow <log>` shows this PR's diff touching no `Severity:` and no `Disposition:` line. **Artifact-evidence:** ledger cites the recount. Guards the `disposition-every-stumble` edge; traces project **AC-010** (and depends on project AC-006). |
| AC-005 | **GIVEN** BR-14 binds every summary of this evidence to "real stumbles were captured and are traceable" and never to exhaustiveness (`../project.md`, AC-008), **WHEN** the verdict is `declined`, **THEN** its reason states what the decline costs — that the evidence rests on one session and the narrow claim stands unchanged — is compatible with `## Scope of the claim` without restating it, and asserts nothing about coverage; and where a second session was *owed but could not be run*, that is the stated reason and the record says the evidence is correspondingly thinner. It is **NOT** *"one session was enough to find the problems"*, *"nothing important was missed"*, a decline justified by cost or scheduling alone, or a quietly relaxed bar. | **Static (grep).** `rg -ni "exhaustiv\|complete coverage\|all the problems\|nothing.{0,20}missed" <log>` returns nothing outside a disclaiming sentence. **Artifact-evidence (reviewer-read, ledger-cited):** the reviewer confirms the reason names its consequence, is compatible with `## Scope of the claim`, and would still read honestly if quoted alone in HS-P0025's promotion pass. Traces project **AC-010**; upholds project AC-008 / BR-14. |
| AC-006 | **GIVEN** one commit or merge point is one session's fixture and a second session opens a new fixture instance (`../_decomposition.md`, `## Testing brief`, fixtures and seams; `CLAUDE.md`, `## The rule that matters`), **WHEN** the verdict is `run`, **THEN** the verdict records the decision and names where the second session's own record will live, and **no** second-session entry is appended to `<log>`'s `## Chronological record`, no `FL-###` id is continued into it, and no second session is run inside this PR — **NOT** reopened as a continuation of the first log. | **Static.** `rg -n "^### FL-" <log>` is unchanged by this PR's diff (`git diff --stat` shows `## Hand-off` only); `git log -p --follow <log>` shows no entry appended under this story. **Artifact-evidence:** where the arm is `run`, the ledger cites the line naming the second record's destination; where the arm is `declined`, the reviewer records that this row is satisfied vacuously and says so. Traces project **AC-010**. |
| AC-007 | **GIVEN** two sibling projects have already declared they will cite into this log by reference, and a re-worded heading breaks a citation with no error message, **WHEN** this PR is diffed, **THEN** the change is confined to the second-session slot inside `## Hand-off`: no section added, renamed or reordered, no `FL-###` heading or body altered, no id renumbered; the verdict is composed from the repository's existing `## Shape decision` primitive (the decision *with the alternative that lost*) rather than a hand-rolled block; it is complete as static text — no colour, emoji, fold, `<details>`, widget or rendering step, and any checkbox on one line; and a verdict revised after first being written **appends** beside the original with the earlier verdict and the reason still legible. It is **NOT** an in-place overwrite, a new heading vocabulary, a bespoke verdict widget, or a tidy-up of the rest of the log. | **Static.** `rg -n "^## " <log>` returns the same eight headings in the same order as before this PR; `git diff -- <log>` shows hunks only inside `## Hand-off`; `rg -n "^### FL-" <log>` unchanged; `git show HEAD:<log>` read with all styling stripped carries the whole verdict; `rg -n "<details>\|<summary>" <log>` returns nothing; any `- [ ]` box matches on one line. **Static (append-only):** `git log -p --follow <log>` shows no in-place rewrite of a written verdict. **Artifact-evidence:** ledger cites the `## Shape decision`-shaped block. Refines UX-AC-004 / UX-AC-007 / UX-AC-008; IQ-1, IQ-2, IQ-3, IQ-4. |

**Coverage of the traced project AC.** Project **AC-010** — *"Whether the findings were dominated by
one blocking defect is assessed and recorded, with a second session either run or declined for a
stated reason"* — is this story's sole traced criterion and is carried in full by six rows: AC-002
and AC-003 own *assessed and recorded*; AC-001 owns *run or declined* as a two-valued state where
absence is a failure; AC-005 owns *for a stated reason*; AC-006 owns what a `run` verdict actually
obliges; AC-004 owns the precondition that makes the assessment arithmetic rather than impression.
AC-007 is not an additional project AC — it is the UX brief's interaction-quality invariants for this
story's own edit, promoted to a table row because a prose bullet is never gated.

## Interaction quality

RFC §6.7/D6. **Every invariant below is already an `AC-###` row above** — this section says *which*
row carries it and how that row is checked, and introduces nothing new. The placement is deliberate:
`redkiln verify` extracts ACs by matching a leading `| AC-001 |` table cell or an `- AC-001:` bullet,
so an invariant stated only as prose here would carry no ledger row, be gated by nothing, and be
tested by nobody.

**Where the composition family comes from.** `../_design.md` is signed off (Ryan Britton, repository
owner, 2026-08-17, no conditions) and declares `hasSurface: false` with an empty `## Items` fence —
this project renders no screen and adds no `pub` item, so there is no surface id to bind to. What
`_design.md` *does* bind, in its `## Items` prose, is the **primitive layer**: there is no CSS layer
and no token file, and inventing one is out of scope. The composition invariants below are therefore
taken from that clause, from the UX brief's `#### The primitive layer to compose from — do not
hand-roll` and `#### The accessibility floor`, and from the `## Hand-off` slot shape
`friction-log-skeleton` fixes. None of them is re-decided here.

### State invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **In place, not a context jump (IQ-1)** — U3 reads the assessment, the test and the verdict together inside `## Hand-off`; any `FL-###` citation points outward for detail only and the hop is optional | **AC-001**, **AC-003** | The verdict and its assessment are lines inside `## Hand-off`, not a pointer into another section or another file. Static: the slot resolves without following a link |
| **Non-occlusion (IQ-2)** — nothing load-bearing sits behind a fold, a collapsed admonition or a derived index; the verdict is visible by default | **AC-001**, **AC-007** | `rg -n "<details>\|<summary>" <log>` returns nothing; deleting `## Dispositions index` and every fold from a scratch copy loses no part of the verdict or its assessment |
| **Preserved position (IQ-3)** — sections, `FL-###` ids and heading lines are unchanged; the diff is confined to one slot | **AC-007** (and **AC-006** for the id space) | `rg -n "^## " <log>` returns the eight headings in the same order; `rg -n "^### FL-" <log>` unchanged; `git diff -- <log>` shows hunks only inside `## Hand-off` |
| **Reversibility (IQ-4)** — a changed verdict appends beside the original, with the earlier verdict and the reason for the change still legible; nothing is overwritten | **AC-007** | `git log -p --follow <log>` shows no diff hunk rewriting a previously written verdict line. Bound by `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Non-occlusion of the finding set** — the verdict may not be made comfortable by editing what it is computed from | **AC-004** | This PR's diff touches no `Severity:` and no `Disposition:` line; the recorded counts equal a fresh recount |
| **Keyboard and assistive reachability** — the slot is reachable with browser find and stable heading anchors alone; no widget, script or rendering tool is required | **AC-007** | Plain-text read of `git show HEAD:<log>`; the `## Hand-off` anchor is unchanged, so an existing citation still lands |
| **Reversibility of the ordering edge** — the assessment is not computed while the finding set is still moving | **AC-004** | Zero `not yet dispositioned` arms at the moment the counts are taken; `disposition-every-stumble` merged first |

### Composition invariants

| Invariant | Carried by | How it is verified |
| --- | --- | --- |
| **Presentation exists at all** — the verdict is a *composed* record, not a bare sentence dropped under a heading: it carries the test, the four inputs with their ids, the arm, and the reason | **AC-002**, **AC-003**, **AC-005** | Each element is present with a real value; a slot carrying only `declined` with no reason and no inputs fails, which is exactly the presence-token wrong implementation the testing brief names |
| **Composition and placement** — the verdict lands in the second-session slot inside `## Hand-off` and nowhere else; the eight sections keep their names and order | **AC-001**, **AC-007** | `rg -n "^## " <log>`; `git diff -- <log>` confined to `## Hand-off`; no second document in the repository states the verdict |
| **Composed from an existing primitive, not hand-rolled** — the `## Shape decision` table shape from `.redkiln/templates/_design.md` (`\| Item \| Chosen shape \| Rejected (and why) \| Evidence \| Resolves \|`), the repository's existing way of recording a decision *with the alternative that lost* | **AC-007** | The block is recognisably that shape; the rejected arm is named with its reason. No new heading vocabulary, no fold, no widget, no table of contents |
| **Transience — persistent chrome vs revealed vs opened on demand** | **AC-001**, **AC-007** | *Persistent chrome*: the verdict, the stated test and the four inputs are visible by default, always. *Revealed*: nothing. *Opened on demand*: only the optional `FL-###` detail a citation points at — never the verdict, never the reason, never an input |
| **Density budget, with its real numbers** | **AC-002** (four inputs), **AC-001** (one arm), **AC-005** (the reason) | Exactly **four** dominance inputs — no fifth, and any other consideration is stated as such rather than dressed as an input. Exactly **one** verdict arm. The reason: **one to three sentences**, naming its consequence. The stated test: **one** sentence or short block, written once. Any checkbox: **one line**, never wrapped — the gate parser matches line by line and a wrapped box can never match |
| **Hierarchy** — the assessment reads before the verdict it produces; `## Chronological record` stays authoritative and this slot is derived from it, never the reverse | **AC-002**, **AC-003** | Reading order in the file: test → inputs → verdict. The assessment cites into the record; the record is not edited to agree with the assessment (AC-004) |
| **Anti-pattern — silence, or a presence token, standing in for a decision** ("not mentioned", `TBD`, `n/a`, "noted") | **AC-001** | Content grep against the `## Hand-off` block, not mere presence of a line |
| **Anti-pattern — colour or emoji as the sole carrier** of the verdict or its severity references | **AC-007** | Plain-text read of `git show HEAD:<log>`: every token is a word |
| **Anti-pattern — a load-bearing item behind a fold, an inactive tab or a collapsed admonition** with no visible-by-default counterpart (`.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`) | **AC-007** | `rg -n "<details>\|<summary>"` returns nothing; the deletion check loses nothing |
| **Anti-pattern — a bespoke widget or a second summary document** layered on a medium that already renders the equivalent for free; two claims that can drift | **AC-001**, **AC-007** | The verdict exists once; `handoff-note-to-closeout` cites it rather than restating it |
| **Anti-pattern — a decline bought with an overclaim** | **AC-005** | `rg -ni "exhaustiv\|nothing.{0,20}missed"` returns nothing outside a disclaiming sentence |
| **Anti-pattern — a second session's entries appended into this log** under the same `FL-###` sequence | **AC-006** | `rg -n "^### FL-" <log>` unchanged by this PR |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | `<log>` still carries entries on the `not yet dispositioned` arm when the assessment is due | **Do not compute.** The counts would describe a finding set that is still moving, and the recorded derivation — the only thing that makes this a decision rather than an assertion — would become false with no gate to catch it. Block on `disposition-every-stumble` (AC-004) and record the blocked state in this story's own notes |
| **EC-002** | The scale `_design.md` named has no unambiguous "blocking end" — a stoplight convention, or a local variant, rather than Nielsen 0–4 | Name the token you are treating as blocking **in the assessment**, quoting the legend in `## Severity scale`, and apply it consistently. Do **not** invent a scale, re-scale existing marks, or substitute Nielsen 0–4 because it is tidier. If the legend genuinely admits no blocking end, that is a protocol defect: log it as a finding and route it to `dt9-and-fixed-protocol`, and state in the verdict that the test was applied against a stated interpretation |
| **EC-003** | The session was abandoned at the first blocker and the log holds one or two stumbles | The assessment still runs, and this is the paradigm case the test was written for — a single blocking stumble that both ended the session and left nothing else to carry the claim. Thinness is not a reason to skip the assessment, and a one-stumble log is evidence (IQ-6; `../session-run-against-pinned-tree/spec.md`, AC-008) |
| **EC-004** | The test says a second session is owed, and none can be run — no reader, no window, no time | The verdict is `declined — a second session was owed and could not be run`, with the consequence stated: the evidence rests on one session and is correspondingly thinner. **The bar does not move.** This is the same discipline `../project.md`'s risk table row 1 applies to the no-reader-found state — *"a failure of the initiative, not a reason to redefine the bar"*. Retro-fitting a dominance assessment that concludes the opposite is the forbidden repair (AC-005) |
| **EC-005** | A second session has already been run, out of order, before this verdict was written | Record the verdict as `run`, dated, stating that it happened and where its record lives. The second session's entries still do **not** enter this log and do not continue its `FL-###` sequence — a new fixture is a new fixture regardless of the order the paperwork arrived in (AC-006) |
| **EC-006** | The verdict must change after it has been written and committed | Append beside it with the earlier verdict, the new one, and the reason for the change, all legible. Never edit in place. `.kb/governance/rewrite-the-referent-never-the-reasoning.md`: the referent may be rewritten, the reasoning may not be erased (AC-007 / IQ-4) |
| **EC-007** | `friction-log-skeleton` landed the log at a different path, or named the `## Hand-off` slot differently from what this spec anticipates | **The as-built artifact wins verbatim.** Widen the `## PR boundary` fenced block *deliberately, before writing* — never at gate time, and never by editing the boundary after a file has already been written outside it. This spec's path rows are then discarded in the as-built's favour (`## Behavior and interfaces`, first row) |
| **EC-008** | Computing the counts reveals a stumble whose disposition or severity now looks wrong | It is a finding **routed** to `disposition-every-stumble` or `route-and-escalate`, not fixed here. Adjusting a severity or a disposition inside this PR changes the number the verdict is computed from, which is the one manipulation the whole assessment exists to make impossible to hide (AC-004) |
| **EC-009** | The decline reason and `## Scope of the claim` disagree — the scope sentence says one thing, the reason implies another | Neither is silently re-worded. `scope-the-claim` owns that sentence (project AC-008); a genuine incompatibility is a blocking finding raised against that story, and the verdict waits. Two summaries that can drift is precisely the failure IQ-1 and decision 10 exist to prevent |
| **EC-010** | Someone proposes a CI check that asserts a second-session line exists | Refuse it. `../_decomposition.md`, `## Testing brief`, Notes forbids exactly this shape: a script confirming a line is present would pass `"noted"` as readily as a real verdict, and so rejects no wrong implementation. Any lightening of the reviewer burden must assert on **content** — an arm token, a citation pattern, a count that matches — or it belongs in the Artifact-evidence tier and not in a new automated one |
| **EC-011** | `_design.md` looks wrong at assessment time and the temptation is to amend it | `_design.md` is signed off, and its commit date is what makes project AC-002's provenance check meaningful. Amending it here destroys that check retroactively. Log the defect and escalate it with its DT id (project AC-011); this story reads the scale, it does not set it |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| **NF-001** | A stranger who has never met the logger can **recompute** the assessment from `<log>` alone, in a few minutes, using `rg` and their own reading — no conversation, no context, no access beyond the repository | This is the operational meaning of "assessed and recorded" (`../project.md`, AC-010) and the U3 bar the UX brief sets: act *without asking the logger what an entry meant* |
| **NF-002** | The verdict and its assessment are readable with **no tool, script, rendering step or network access** — plain markdown, `git diff`, browser find, stable heading anchors, and a screen reader | `../_decomposition.md`, `#### The accessibility floor`. A `git diff` view has no colour at all, which is why every token is a word |
| **NF-003** | **One claim, not two.** The verdict exists in exactly one place in the repository; every downstream mention is a citation to that slot | IQ-1 and decision 10. Two statements of the same verdict is two things to update and one that goes stale — the same reasoning `CLAUDE.md` gives for not duplicating the style guide |
| **NF-004** | The assessment records **the log state it was computed against** — the counts, the ids, and the fact that the log was fully dispositioned at that point | Without it, a later reader who recounts and gets a different number cannot tell whether the verdict was wrong or the log moved. AC-004 makes the precondition true; NF-004 makes it legible afterwards |
| **NF-005** | **No new dependency, tool, format or convention.** No CSS, no token file, no widget, no heading vocabulary not already in the skeleton, no new severity scale | `../_design.md`, `## Items`: inventing a primitive layer is out of scope. The primitives are the named existing files |
| **NF-006** | Repository hygiene is unchanged: `redkiln validate --kb && redkiln doctor` clean at **exactly** the six standing `template-drift` advisories — no more, no fewer — and **zero** files under `.kb/` touched | `CLAUDE.md`, `## Where the work lives`; `../project.md`, DoD-6 and DoD-8. A seventh advisory is an unintended template change; a missing one is a reverted customisation |
| **NF-007** | The verdict introduces no personal detail beyond what the reader's declaration already carries, and names no third party who did not consent to be in a public repository | The log is committed to a repository that will be published; a decline reason that says *"the reader was unavailable"* names a scheduling fact, not a person |

## Implementation notes (non-prescriptive)

Not instructions — the reasoning behind the shape, so the implementer can make the small calls the
criteria do not reach.

**Take the counts first, write the verdict last.** The failure mode this story exists to prevent is
motivated reasoning, and its mechanical signature is a verdict written before the numbers. Run the
counts, paste them in with their ids, then read the test against them. If the order feels backwards,
that is the point.

**Write the test before you know the answer.** The dominance test is stated in the log so it cannot
be re-worded to fit the conclusion (AC-003). The cheapest way to guarantee that is to write it into
`## Hand-off` in the same edit that records the counts, and only then to record the arm.

**Four inputs, and say so when something else mattered.** Time, reader availability and appetite are
real considerations and are frequently decisive — but they are not this test. If one of them drove
the outcome, the honest record says so as a *separate stated factor* beside the assessment, which
keeps the test clean and the reason truthful. Dressing a scheduling constraint as a dominance
finding is the single most likely dishonesty in this PR.

**A `declined` verdict on a thin log is not automatically wrong.** Nielsen & Landauer's basis is
that one session finds roughly a third of problems and that each problem a real reader hits is
already proven worth fixing (`../_grounding.md:72-76`). A short log with three well-spread stumbles
supports the narrow claim; a short log where one blocking stumble ended the session does not. The
test distinguishes them; the length alone does not.

**Do not tidy the log while you are in it.** The pull to fix a typo in an `FL-###` heading or
re-align a table while the file is open is strong and is exactly what breaks a sibling project's
citation with no error message (AC-007, IQ-3). Confine the diff to one slot and let the rest be
imperfect.

**Prefer citing three ids to citing none.** The mechanical value of the assessment is that the next
reader can jump straight to the entries the count came from. An id costs nine characters; its
absence costs the recomputation.

**If the arm is `run`, resist starting.** Deciding is this story; running is a new fixture with its
own record and its own act (AC-006, decision 6). Appending a second session's entries into this log
is the one boundary breach here that cannot be undone by a later edit, because the `FL-###` space
has no forwarding mechanism.

**Reuse the `## Shape decision` table rather than inventing a verdict block.** It already carries
the column this story most needs — *Rejected (and why)* — which is where the arm that lost goes.
That single column is what converts "we declined" into a decision a reviewer can argue with.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`, `## Testing brief` — its AC/tier table (the AC-010 row:
*"Ledger cites the assessment and its verdict (run / declined-with-reason)"*, proving *"silence
never stands in for a decision"*) and its Notes — and in `.redkiln/config.yaml`'s wired grains.
`<log>` is `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` or the path the skeleton
fixed (EC-007). Every static row below asserts on **content**; per the brief, a row that would pass
`"noted"` as readily as a real verdict rejects no wrong implementation and does not belong here.

| tier | command / path | proves |
| --- | --- | --- |
| **Content (static, `rg`)** | `rg -n "second.session" <log>` and a scoped read of the `## Hand-off` block: exactly one arm, `run` or `declined — <reason>`; the skeleton's not-yet-recorded token absent from that slot; `rg -ni "TBD\|not mentioned\|noted\|^n/a"` returning nothing there | **AC-001** — the verdict exists, is two-valued, and absence is caught rather than tolerated |
| **Content (static, `rg`) + recount** | The four inputs present with values; `rg -o "FL-[0-9]{3}"` over the assessment block resolving against `rg -n "^### FL-" <log>`; a fresh count of severity-marked entries and of blocking-end tokens compared to the recorded numbers | **AC-002**, **AC-004** — the assessment is recomputable, and the numbers are the log's, not the author's |
| **Content (static, `rg`)** | `rg -n "Disposition:" <log>` returning zero `not yet dispositioned` arms | **AC-004** — the assessment was computed against a settled finding set, which is the whole content of the `disposition-every-stumble` edge |
| **Content (static, `rg`)** | `rg -ni "exhaustiv\|complete coverage\|all the problems\|nothing.{0,20}missed" <log>` returning nothing outside a disclaiming sentence | **AC-005** — the decline is not bought with an overclaim (BR-14, project AC-008) |
| **Structure (static, `rg` + `git diff`)** | `rg -n "^## " <log>` (eight headings, same order); `rg -n "^### FL-" <log>` (unchanged set); `git diff -- <log>` (hunks inside `## Hand-off` only); `git diff --stat` over the PR (two paths only) | **AC-006**, **AC-007** — no id continued, no section moved, no entry edited, no boundary leak |
| **Plain-text legibility (static)** | `git show HEAD:<log>` read with all styling stripped; `rg -n "<details>\|<summary>" <log>` empty; any `- [ ]` box on one line | **AC-007**, **NF-002** — a screen reader and `git diff` both read the whole verdict; nothing needs a renderer, and a wrapped checkbox can never match the gate parser |
| **Append-only history (static)** | `git log -p --follow <log>` across this story's commits | **AC-003**, **AC-007** / IQ-4 — the test was not edited after the verdict, and no written verdict was rewritten in place |
| **Artifact-evidence (ledger)** | `.bklg/docs-that-teach/comprehension-evidence/second-session-decision/_ledger.md`, enforced by `redkiln verify --grain story` with `require_ledger: true` (`.redkiln/config.yaml:67`) | Every AC row carries a real `file:line` into the log. This is the tier that carries **AC-003** and **AC-005**, which are genuinely reviewer-read and which the brief forbids over-automating |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The repository gate for this diff. Touching no crate, it falls through to the five file-reading lints and `spec-trace` unconditionally rather than passing vacuously on an empty package set |
| **Integration grain** | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7's non-terminal bar. Expected green and unchanged — this story alters no crate, so a failure here is a signal the PR boundary leaked |
| **Backlog hygiene** | `redkiln validate --kb && redkiln doctor` | **NF-006** — clean at exactly the six standing `template-drift` advisories, and no `.kb/` atom authored or edited |
| **Explicitly not run** | `cargo xtask ci` (`.redkiln/config.yaml:60`, `e2e`) | The terminal bar is HS-P0025's (`../project.md`, DoD-7). And per the brief, folding a research instrument into a merge gate confuses two things that falsify different things — a passing gate says nothing about whether one session was enough |
| **Explicitly not added** | Any check that asserts a second-session line merely exists | EC-010. It would pass the wrong implementation `discover.md` names — a default dressed as a decision — and a rule nothing can fail is decorative (`CLAUDE.md`, `## The rule that matters`) |

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation, inside this PR |
| --- | --- | --- |
| The verdict is written first and the assessment reverse-engineered to support it | Medium / High | AC-003 requires the test to be stated in the log and the verdict to follow visibly from the four inputs; the append-only history check shows whether the test landed with the verdict or after it. The implementation note makes counts-first the stated working order |
| The assessment is computed while stumbles are still undispositioned, and the recorded counts later fail to match the log | Medium / High | AC-004 plus EC-001: zero `not yet dispositioned` arms is a static precondition, and the recorded counts are re-counted at review. This is the entire content of the `disposition-every-stumble` edge and the one ordering error nothing else would catch |
| The decline is bought with an overclaim — *"one session was enough to find the problems"* | Medium / High | AC-005's grep plus a reviewer read against `## Scope of the claim`. BR-14 is the binding constraint on the reason text, and HS-P0025 will quote this sentence at promotion, where an overclaim becomes durable |
| A scheduling constraint is dressed as a dominance finding | Medium / Medium | The four inputs are closed (AC-002, density budget); anything else is recorded as a *separate stated factor*. EC-004 gives the honest path — `declined` with "owed but could not be run" and the consequence stated |
| A `run` verdict slides into actually running a second session inside this PR | Low / High | AC-006 plus the `## PR boundary` fenced block, which `redkiln verify --grain story` reads. The `FL-###` space has no forwarding mechanism, so an appended second session is unrecoverable rather than merely untidy |
| The log is tidied while the file is open — a heading re-worded, ids renumbered, a severity adjusted | Medium / High | AC-007's diff-confinement check and AC-004's no-`Severity:`-no-`Disposition:` check. Two sibling projects have already declared they will cite into this log by id, and a broken citation raises no error |
| The verdict is restated in the hand-off note and the two drift | Medium / Medium | NF-003 and decision 10: `handoff-note-to-closeout` cites this slot. Slice-mates are implemented in one context, so the drift is preventable at write time rather than at review |
| `_design.md`'s scale admits no clean blocking end and one is invented at assessment time | Low / Medium | EC-002: state the interpretation in the assessment, route the protocol defect to `dt9-and-fixed-protocol`, and do not re-scale existing marks. Re-scaling would silently move every count |
| The log's home differs from `_friction-log.md` and this spec's paths go stale | Low / Low | `friction-log-skeleton`'s spec binds that path today and mounts the log at `../project.md`'s `## Companions`; EC-007 handles divergence, and the `## PR boundary` glob already covers the whole story folder |

**Coupling, stated once.** This story consumes a finished record and produces one line plus its
derivation; nothing downstream can repair a defect in it, because `handoff-note-to-closeout` carries
the verdict forward *by reference* and HS-P0025 `durable-audience-closeout` reads it as an input it
cannot manufacture. The upstream coupling is a single hard edge — `disposition-every-stumble` — and
it is a correctness edge, not an ordering convenience.

## Dependencies

**Blocks on** — matching this story's `depends_on` exactly:

| Story slug | What it must have landed before this story starts |
| --- | --- |
| `disposition-every-stumble` | Every severity-marked stumble in `<log>` carrying exactly one disposition arm, with zero entries reading `not yet dispositioned` (project AC-006). Until then the finding set is still moving and the counts this story records are arithmetic about nothing (AC-004, EC-001) |

Transitively, through that story: `session-run-against-pinned-tree` (the session that produced the
stumbles, the `Kind: abandonment` terminal state and the `## Scenario` goal outcome this test reads),
`friction-log-skeleton` (the eight sections, the `FL-###` shape and the `## Hand-off` slot itself),
`dt9-and-fixed-protocol` (the severity scale whose blocking end the test is expressed against, fixed
in `../_design.md` under the human sign-off gate), and `non-insider-recruitment`.

**Slice-mate, no edge either way**

| Story slug | Relationship |
| --- | --- |
| `scope-the-claim` | Same slice, `scoped-claim-and-handoff`, in either order (`../_storymap.md`, `## Merge order`, step 4). No dependency, but a real compatibility obligation: this story's decline reason must be compatible with `## Scope of the claim` and must not restate it (AC-005, EC-009) |

**Unlocks**

| Story slug | What it takes from this story |
| --- | --- |
| `handoff-note-to-closeout` | The recorded verdict, cited by reference rather than restated, so the hand-off HS-P0025 reads carries one claim and not two (`../_storymap.md`, `## Coverage`, AC-009/AC-010 rows). It lands last in this slice |

And transitively, through it: **HS-P0025 `durable-audience-closeout`**, which cannot honestly state
how much evidence it is inheriting while the second-session question is answered by silence.

## Anchors (progressive disclosure)

Load-bearing depth, deferred but **not optional**. The `## Context pack` above is self-sufficient to
begin; open each anchor at the stated moment. **Link, never paste in bulk.** Every path below was
confirmed present in this worktree.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes the artifact this story writes into: the log's path, its eight sections in order, and the `## Hand-off` row naming the second-session verdict slot (`run` \| `declined — <reason>`; "not mentioned" is not a state) together with the not-yet-recorded token this story replaces. Its `## Behavior and interfaces` table is the slot-by-slot contract, and the as-built file wins over this spec wherever they differ | Before writing the first character into the log — as a pre-flight that confirms the slot and its token are actually on disk under the names expected | AC-001, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/spec.md` | The one blocking edge, in its own words: the five disposition arms, the rule that `not yet dispositioned` is legal during the session and illegal at hand-off, and that zero and two arms are equal failures. It also fixes that the record itself is not rewritten by a disposition pass, which is why the counts are stable enough to cite | Immediately before taking the counts, to confirm the pass has landed and to know exactly which arm strings to grep for | AC-004 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off protocol of record (`hasSurface: false`, approved 2026-08-17): the severity scale whose blocking end this test is expressed against, and the `## Items` clause binding the primitive layer — no CSS, no token file, no invented format. **Read-only: amending it retroactively destroys project AC-002's provenance check** | Before applying input (b), to read the scale's blocking-end token off the record rather than assuming Nielsen 0–4 | AC-002, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief's states list (*"'Not mentioned' is not one of the states"*), the seven IQ invariants, the accessibility floor in this medium's own terms, the primitive layer to compose from, and the testing brief's AC-010 row plus its standing prohibition on automating a check that nothing can fail | Open the UX brief when composing the block; open the testing brief when deciding how a criterion is actually verified, and whenever a lighter automated check is proposed | AC-002, AC-005, AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | AC-010's exact wording (*"is assessed and recorded"*), derived requirement 10, AC-008's scope constraint that bounds the decline reason, the risk-table row this story discharges, and risk row 1 — *"a failure of the initiative, not a reason to redefine the bar"* — which is the precedent EC-004 applies | When a criterion's intent is ambiguous, and when writing the ledger's `criterion` values | AC-001, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | Lines 72–76: Nielsen & Landauer — one qualitative session finds ~1/3 of problems and each problem a real reader hits is already proven. This is the stated grounding for AC-010's "was one session enough" assessment and for the narrow claim the decline reason must rest on, and it is why a short log is not automatically insufficient | When writing the decline reason, and whenever the pull arrives to justify the decline by coverage rather than by the small-sample basis | AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/session-run-against-pinned-tree/spec.md` | Supplies two of the four inputs by contract: its AC-008 makes the end state explicit (a recorded completion of `## Scenario`, or a `Kind: abandonment` entry), and its AC-004 fixes severity as an inline text token on the named scale. Its EC-005 states plainly that the second-session question is this story's recorded call | Before taking inputs (c) and (d), to know exactly what shape the end state is recorded in | AC-002, AC-003 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | The slice and merge order (`scoped-claim-and-handoff`, step 4: `scope-the-claim` and this story in either order, `handoff-note-to-closeout` last), the dependency graph's single inbound edge, and the `## Coverage` rows giving AC-010 to this story alone | At the start, to confirm the ordering, and whenever the scope of "answer the question" feels like it should absorb a slice-mate | AC-004, AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/handoff-note-to-closeout/spec.md` | The consumer. It carries this verdict forward by reference, which is what makes NF-003's one-claim rule real rather than aspirational — read it to see exactly what it expects to cite, so the slot is written in a form it can point at | While composing the verdict block, and before considering any second statement of the verdict anywhere | AC-001 |
| `.bklg/docs-that-teach/comprehension-evidence/scope-the-claim/spec.md` | Owns `## Scope of the claim`. The decline reason must be compatible with that sentence and must not restate it; an incompatibility is a blocking finding rather than a silent re-wording of either | When drafting a `declined` reason, and if EC-009's disagreement appears | AC-005 |
| `.bklg/docs-that-teach/initiative.md` | Line 349 states BR-14; line 637 states the assumption this story converts into a recorded decision — *"one comprehension session is enough… if the first session's findings are dominated by a single blocking defect, a second may be owed"*. "May be owed" is the residual an unassessed project resolves by silence | When the verdict's framing needs its source, and when writing the assessment's opening sentence | AC-003, AC-005 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding anywhere near this project. It is the reasoning behind write-once/revise-beside: the referent may be rewritten, the reasoning may not be erased | The moment a written verdict appears to need changing (EC-006) | AC-007 |
| `.redkiln/templates/_design.md` | The `## Shape decision` table — the repository's existing primitive for recording a decision *with the alternative that lost*. The verdict composes into that shape rather than a hand-rolled block, and its *Rejected (and why)* column is where the arm that lost goes | While composing the verdict block, before inventing any format | AC-007 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | The anti-pattern list the composition invariants cite by name — a load-bearing item behind a fold or collapsed admonition with no visible-by-default counterpart, and a bespoke widget over a medium that renders the equivalent for free | When tempted to add a fold, a roll-up or a summary block because the hand-off is getting long | AC-007 |
| `.redkiln/config.yaml` | The wired gate grains this PR is actually checked by: `affected_gate` (line 40), `integration_scoped` (line 55), `e2e` (line 60 — explicitly not this project's), `require_ledger` (line 67), `support_initiative` (line 5) | When running the merge gate | AC-004, AC-007 |
| `.redkiln/templates/_ledger.md` | The ledger shape and its rules: planning authors every row `satisfied: false`; the implementer may only flip a row with real evidence and may never re-word a criterion | When filling `_ledger.md` after the verdict is written | all |
| `CLAUDE.md` | Two clauses bind here: `## The rule that matters` (one fixture instance is one isolated backing store — the discipline AC-006 borrows) and `## Where the work lives` (the CLI is the only writer of item frontmatter; the six standing `template-drift` advisories NF-006 must not move; the one-line checkbox constraint) | Before the first edit, and before running the hygiene checks | AC-006, AC-007 |

## Clarifications resolved during spec

**The AC set is exactly the seven the front half enumerated.** AC-001…AC-007, none added and none
dropped, in the same order and with the same subjects as `## Context pack`'s numbered decisions:
decision 1 → AC-001; decision 2 → AC-002; decisions 3 and 5 (the test itself) → AC-003; decision 4 →
AC-004; decisions 5 and 7 → AC-005; decision 6 → AC-006; decisions 8, 9 and 10 → AC-007. The
`_ledger.md` carries exactly these seven rows.

**Why the interaction-quality invariants are concentrated in AC-007 rather than spread.** IQ-3
(preserved position), IQ-4 (reversibility), the plain-text floor and the primitive-composition
obligation all describe **the same edit** — a diff confined to one slot, composed from an existing
shape, that never overwrites what it replaces. Splitting them across four rows would produce four
ledger rows citing the same `git diff`, which is decoration rather than gating. IQ-1 and IQ-2 sit on
AC-001 and AC-007 because they are properties of where the verdict *lives*, and the finding-set
invariant sits on AC-004 because it is a property of what the verdict is computed *from*.

**Where the composition family comes from, given `hasSurface: false`.** `../_design.md` declares no
surface id and an empty `## Items` fence, so there is no signed-off surface to bind to in the usual
sense. Rather than treat that as an exemption, this spec takes the composition family from what
`_design.md` *does* bind in its `## Items` prose — the named document primitive layer and the
explicit ruling that inventing a CSS or token layer is out of scope — joined to the UX brief's
accessibility floor and the `## Hand-off` slot shape `friction-log-skeleton` fixes. Nothing in
`## Interaction quality` is a new decision, and every invariant there is carried by a row above.

**The dominance test is this spec's contribution, and it is bounded deliberately.** The project
charter says the question is assessed and recorded; it does not say what "dominated by a single
blocking defect" means, and an unstated test is how a verdict becomes a mood. The four inputs were
chosen because each is readable off the log without judgment, and the set is closed on purpose
(AC-002's density budget): a fifth input is where "it felt fine" would re-enter. Considerations
outside the four are recorded as *separate stated factors*, which keeps the test falsifiable and the
reason honest.

**`_friction-log.md` does not exist in this worktree yet, and that is expected.** It is landed by
`friction-log-skeleton` and filled by `session-run-against-pinned-tree`, both of which precede this
story in merge order. Every anchor cited above is a file that exists **today**; the log itself is
referenced through the specs that bind it, which is why `friction-log-skeleton/spec.md` and
`disposition-every-stumble/spec.md` are anchors and the log is not. EC-007 covers the case where the
as-built artifact differs from what those specs bind.

**"Verifying test" means a real check, not a test file.** This project has no functions and no test
binary, and the testing brief says so plainly. Each AC's verification is a real `rg` / `git` /
recount check against a real path, plus the ledger's `file:line` discipline — and per the brief's own
warning, each static check asserts on content rather than mere structural presence, so none of them
is a check that nothing can fail.

**Project AC-008 is honoured here but not owned.** AC-005 binds the decline reason to BR-14's narrow
claim and forbids an exhaustiveness assertion, which is a constraint *on this story's text*.
Authoring the scope sentence itself remains `scope-the-claim`'s, and EC-009 keeps a disagreement
between the two a blocking finding rather than a quiet re-wording.

**Running a second session is out of scope even when the verdict is `run`.** The verdict is the
recorded decision; the running is a separate act against a new fixture with its own record
(AC-006, decision 6). This is the same fixture discipline the conformance suite applies to stores,
with a different noun pinned.
