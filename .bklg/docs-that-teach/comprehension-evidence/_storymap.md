---
item: HS-P0024
stage: storymap
created: 2026-08-17T03:21:39.518Z
updated: 2026-08-17T03:21:39.518Z
template_sig: 1c63534a
rendered_sig: 77442abd
---

# Story Map — Comprehension Evidence

Ten stories in four slices. The spine is `project.md`'s AC-001 … AC-011; the slicing
follows the UX brief's state enumeration and the testing brief's tier split
([`_decomposition.md`](_decomposition.md), `## UX brief` → *The states this surface has
to express*, and `## Testing brief` → the AC/tier table).

This project ships no crate. Its three surfaces are artifacts a person reads — the
assembled material the reader walks, the friction log, and `_design.md`
([`_decomposition.md`](_decomposition.md), `## UX brief`, `### Intent`) — so a "vertical
slice" here means *protocol → observation → record → disposition → hand-off* cutting
through all three, never "write a section of the log". One story, `content-fixes-from-
dispositions`, does touch code, and it is held to the ordinary tiers
(`.redkiln/config.yaml`'s `affected_gate`, then `integration_scoped: cargo xtask ci
--fast`).

## Backbone

The activities, left to right, in the order a person actually performs them. Each is an
outcome someone can observe, not a document section.

| # | Activity | Whose outcome | Ends when |
| --- | --- | --- | --- |
| A1 | **Fix the instrument before anyone is measured** | U2, the facilitator | `_design.md` carries DT-9's resolution, the scenario, the narration mode, the severity scale and the disqualifying criteria, at a commit that predates the session |
| A2 | **Find a reader who is genuinely outside** | U2 | A candidate has declared against the written criteria and the declaration is in the record |
| A3 | **Watch a stranger use the material** | U1, the recruited reader | The reader has completed, or abandoned, the stated scenario against a named tree, and the chronological record exists |
| A4 | **Give every stumble a destination** | U3, the downstream actor | Each severity-marked item resolves to a fix, a recorded acceptance, a routed id, or an escalation carrying a DT id |
| A5 | **Say only what one session supports, and hand it on** | HS-P0025 | The scope sentence is present wherever the claim is made, the second-session question is answered, and one directly observed persona is named with date and tree |

A2 begins while HS-P0023 is still in flight — this is the only project in the tree whose
schedule depends on a person outside the team (`project.md`, `## Risks and coupling
notes`, closing paragraph). A3 cannot start until HS-P0022 and HS-P0023 have merged
forward: *"A friction log run against a half-assembled surface measures the assembly, not
the teaching"* (`../_decomposition.md`, `### Why the load-bearing edges exist`).

## Slices

The thin vertical slices (stories) under each activity, grouped by the milestone/slice they are delivered
with. Stories sharing a Milestone are implemented together in a single context and mounted as ONE integrated
surface (the implement stage runs one milestone at a time). Cross-milestone `depends_on` edges must be acyclic.
Type each story `capability` (a user-observable slice) or `foundation` (real in-tree substrate a capability
slice in this initiative consumes — never owned outside it, never a double/fixme).

| Milestone | Story | Archetype | One-line slice | depends_on | traces_to |
| --------- | ----- | --------- | -------------- | ---------- | --------- |
| `session-protocol` | `dt9-and-fixed-protocol` | capability | Resolve DT-9 in `_design.md` — which persona the session walks, in terms of what that reader is trying to accomplish, with the two rejected personas and why each lost — and fix the scenario, narration mode, severity scale and logger-disqualifying criteria in the same commit, before any candidate is approached | — | AC-001, AC-002 |
| `session-protocol` | `friction-log-skeleton` | foundation | Land the auditable-shape log scaffold in-tree — scenario; logger identity, context and date; an append-only chronological section with stable occurrence-ordered stumble ids; a text-token severity mark on the named scale; a disposition slot at each stumble with a revision block; and the scope-sentence and hand-off sections — so the session fills a shape it did not invent while running | `dt9-and-fixed-protocol` | AC-004 |
| `session-protocol` | `non-insider-recruitment` | capability | Apply the written criteria to a real candidate and record the reader's own declaration, platform, toolchain and assistive technology in the log — including the ineligible-but-used-anyway state, if it occurs, as a stated failure of the artefact rather than a redefinition of the bar | `dt9-and-fixed-protocol`, `friction-log-skeleton` | AC-003 |
| `observed-session` | `session-run-against-pinned-tree` | capability | Run the session against the post-merge assembled tree and record which tree that was, capturing concurrently what the reader did — searches typed, links followed, files opened, commands run — with severity marked inline, facilitator interventions timestamped as entries, keyboard reach recorded, and abandonment accepted as a valid end state | `friction-log-skeleton`, `non-insider-recruitment` | AC-004, AC-005 |
| `dispositions-and-routing` | `disposition-every-stumble` | capability | Give every severity-marked item exactly one disposition — fixed, deliberately accepted with its reason, or routed — readable at the stumble itself, with any later change recorded as a revision beside the original rather than overwriting it | `session-run-against-pinned-tree` | AC-006 |
| `dispositions-and-routing` | `route-and-escalate` | capability | Submit the log to a named owner able to act and record that submission, give every routed item a destination id that resolves to a real item, and record anything that would reopen a sibling's resolved tension as an escalation carrying its DT id instead of absorbing it here | `disposition-every-stumble` | AC-007, AC-011 |
| `dispositions-and-routing` | `content-fixes-from-dispositions` | capability | Land the small content fixes whose disposition is "fixed", composing from the existing primitive layer — doc-comment sections and intra-doc links, `docs/README.md`'s routing-table shape, compiled examples — and prove each one through `cargo xtask affected` and `cargo xtask ci --fast` | `disposition-every-stumble` | AC-006 |
| `scoped-claim-and-handoff` | `scope-the-claim` | capability | State the narrow claim — real stumbles were captured and are traceable — wherever this evidence is summarised, and assert nothing about exhaustiveness anywhere in the log or its summary | `session-run-against-pinned-tree` | AC-008 |
| `scoped-claim-and-handoff` | `second-session-decision` | capability | Assess whether the findings were dominated by a single blocking defect and record the verdict — a second session run, or declined with a stated reason — so silence never stands in for the decision | `disposition-every-stumble` | AC-010 |
| `scoped-claim-and-handoff` | `handoff-note-to-closeout` | capability | Write the named hand-off section stating one directly observed persona, the date, the tree walked and the scope sentence, in a form HS-P0025 can lift without re-deriving anything, so the blanket "none has been directly observed" qualification is replaced by an accurate one | `route-and-escalate`, `scope-the-claim`, `second-session-decision` | AC-008, AC-009 |

### Why the slices fall here

**`session-protocol` is one slice, not three.** The severity scale, the disqualifying
criteria and the log's shape are the same decision seen from three angles: a scale named
in `_design.md` but absent from the log's severity column is a scale that gets
retrofitted, which is precisely the difference research 04 draws between a rankable
record and a diary. AC-002's provenance check — `git log --format=%aI` on `_design.md`
predating the session date (`_decomposition.md`, `## Testing brief`, AC-002 row) — only
means something if all three land before any candidate is approached, so they are
implemented and committed together. The optional cognitive-walkthrough pre-screen is
declared here too, explicitly as a hypothesis generator whose findings stay provisional
until the real session confirms or supersedes them (`project.md`, `## In scope`).

**`friction-log-skeleton` is the one foundation, and it is consumed inside this
initiative.** It is real in-tree substrate — the scaffold `session-run-against-pinned-
tree` fills, `disposition-every-stumble` writes into and `handoff-note-to-closeout`
closes out — never a double and never a fixme. It exists as its own story because the
UX brief's invariants IQ-2, IQ-3 and IQ-4 (nothing exists only in a filtered view; ids
and order stable once written; a revised disposition recorded as a revision) are
properties of the *shape*, and a shape improvised at 40 minutes into a live session will
not have them. Its consumer is one slice later, which is what keeps it from being a
terminal, unproven island. The log's path and heading vocabulary are `_design.md`'s to
fix, not this map's — the briefs are deliberately contract-grain on that
(`_decomposition.md`, `## UX brief`, `### Notes`, *Contract grain, deliberately*).

**`observed-session` is deliberately a single story.** It is the one irreducible act:
a real person, a real tree, once. Splitting "run the session" from "record what
happened" would be the horizontal cut this project exists to avoid, and the testing
brief is explicit that U1 and U2 are never mocked, scripted or simulated — *"a stand-in
reader is not a cheaper version of this instrument; it is a different, useless one"*.
This slice is also the schedule's hinge: it cannot begin until HS-P0022 and HS-P0023
have merged forward, and AC-005 records which tree that produced.

**`dispositions-and-routing` keeps the fix, the route and the escalation together.** The
three are one decision per stumble with three outcomes; separating "decide" from "act"
would let routing become the sink the risk table names. `content-fixes-from-dispositions`
is the only story in the project that touches code, and it is the only reason DoD-7's
`cargo xtask ci --fast` bar bites here rather than being a formality.

**`scoped-claim-and-handoff` is what HS-P0025 actually reads.** It is separated from
dispositioning because its reader is different: not a sibling project owner acting on an
item, but the terminal project deciding what may honestly be promoted into
`.kb/product/`. Nothing in this slice authors or promotes a `.kb/` atom — hand-authoring
outside the ingest path is a named non-goal and the first attempt at it was reverted
(`0269720`).

### Dependency graph

Acyclic; every edge points forward in the merge order below.

```
dt9-and-fixed-protocol
  └─> friction-log-skeleton
        └─> non-insider-recruitment
              └─> session-run-against-pinned-tree
                    ├─> disposition-every-stumble
                    │     ├─> route-and-escalate ─────────────┐
                    │     ├─> content-fixes-from-dispositions │
                    │     └─> second-session-decision ────────┤
                    └─> scope-the-claim ─────────────────────┬┘
                                                             └─> handoff-note-to-closeout
```

`non-insider-recruitment` depends on both protocol stories because the declaration is
checked against criteria that must already be written (AC-003 requires a reviewer to
verify it *without asking the reader anything further*) and is recorded in the log's own
declaration section. `content-fixes-from-dispositions` deliberately does **not** gate the
hand-off: a fix that lands late must not be able to hold up the evidence HS-P0025 needs,
and IQ-4's reversibility means a disposition revised after hand-off is recorded as a
revision rather than as a rewrite of the note.

## Coverage

Every project acceptance criterion is claimed by at least one story, and no two stories
own the same responsibility. Where an AC appears twice, the two stories own different
halves of it and the split is stated.

| AC | Story / stories | Split, where there is one |
| --- | --- | --- |
| AC-001 — persona decided, not defaulted | `dt9-and-fixed-protocol` | sole owner |
| AC-002 — protocol fixed before recruitment | `dt9-and-fixed-protocol` | sole owner |
| AC-003 — logger verifiably non-author, non-insider | `non-insider-recruitment` | sole owner |
| AC-004 — dated log in the auditable shape | `friction-log-skeleton`, `session-run-against-pinned-tree` | the skeleton owns the **shape** (six sections present, ids stable, severity a text token); the session owns the **content** (that the six are truthfully filled by an observed walk) |
| AC-005 — the log names the tree it walked | `session-run-against-pinned-tree` | sole owner |
| AC-006 — every stumble exactly one disposition | `disposition-every-stumble`, `content-fixes-from-dispositions` | the first owns *exactly one disposition per stumble*; the second owns only the **"fixed" arm** — landing the fix a "fixed" disposition asserts, and clearing the gate for it |
| AC-007 — routing is real and lands correctly | `route-and-escalate` | sole owner |
| AC-008 — the claim is scoped wherever it is made | `scope-the-claim`, `handoff-note-to-closeout` | the first owns the sentence and its presence in the log and its summary; the second is bound to carry the same sentence because the hand-off note is the summary HS-P0025 reads, and it is authored later |
| AC-009 — the hand-off to closeout is explicit | `handoff-note-to-closeout` | sole owner |
| AC-010 — a second session is a decision, not an omission | `second-session-decision` | sole owner |
| AC-011 — no settled decision reopened in passing | `route-and-escalate` | sole owner — escalation is a *distinct state* from routing (UX brief, states list) and is owned with it because both are "this item's destination is elsewhere" |

**MECE confirmed.** Eleven criteria, eleven covered, none orphaned. Ten stories, none
without a traced criterion. The two shared criteria (AC-004, AC-006) and the one carried
criterion (AC-008) are split by responsibility with the seam stated above, in the same
discipline `../_decomposition.md` uses for BR-09, BR-11 and BR-12.

**Not represented as stories, on purpose.** The optional cognitive-walkthrough pre-screen
is a clause inside `dt9-and-fixed-protocol`, not a story, because it traces to no AC and
is explicitly never the proof artefact. Persona promotion, `.kb/` atom authorship and the
re-observation of all fifteen DoD scenarios are HS-P0025's (`project.md`, `## Out of
scope`); no story here touches them. The AC-05 fallback — the verdict that the surfaced
`MemoryEventStore` walk-through does not carry an adapter author — is not a story either:
it is a *finding* that, if it occurs, is dispositioned by `route-and-escalate` to
HS-P0023, in the shape HS-P0023's own risk table already declared it would accept.

## Merge order

Slice by slice, foundation before its consumers.

1. **`session-protocol`** — `dt9-and-fixed-protocol` → `friction-log-skeleton`
   (foundation) → `non-insider-recruitment`. The protocol story lands first because the
   named severity scale is what the skeleton's severity column carries; the foundation
   lands before the slice that consumes it, one milestone later. Recruitment starts here
   and may run while HS-P0023 is still in flight. Merge-gated on `_design.md`'s human
   sign-off (`.redkiln/processes/project.yaml`, `design` stage), which is the only record
   DT-9 will ever have.
2. **`observed-session`** — `session-run-against-pinned-tree`. Cannot start until
   HS-P0022 and HS-P0023 have merged forward, per the project DAG. One session, one
   fixture: the pinned commit is this project's analogue of a testkit fixture instance,
   and a second session (AC-010) opens a new one rather than reopening this.
3. **`dispositions-and-routing`** — `disposition-every-stumble` → then
   `route-and-escalate` and `content-fixes-from-dispositions`, which are independent of
   one another and may land in either order. This is the slice where DoD-7's
   `cargo xtask ci --fast` bar actually bites.
4. **`scoped-claim-and-handoff`** — `scope-the-claim` and `second-session-decision` in
   either order, then `handoff-note-to-closeout` last. The hand-off note is the final
   artifact of the project and the sole input HS-P0025 cannot manufacture.

The repository's own gate being green is a precondition for reading any of this
evidence, never a substitute for it (`project.md`, `## Definition of done`).
