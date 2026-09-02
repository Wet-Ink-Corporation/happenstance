---
item: HS-S0167
stage: spec
created: 2026-08-17T13:16:20.677Z
updated: 2026-08-17T13:16:20.677Z
template_sig: 87bbf1d0
rendered_sig: f2b8fc2f
---

# Spec — Route to real destination ids, escalate anything that reopens a tension

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-06 (routing is the non-optional final step); AC-10 *"Someone acted on what that reader found"*; DoD scenario 6; the assumption at lines 645–647 (*"the publication project's design gate stays closed… if it turns out that it does, that is escalated, not absorbed"*) |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `## Design tension ownership` (lines 148–159: the DT-1…DT-10 vocabulary the escalated arm is typed to), `### Definition of Done` row 6, and the BR/AC ownership tables that make "the sibling project owning the requirement" a lookup rather than a judgement |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — **AC-007** and **AC-011** are this story's traced criteria; derived requirement 8 (a log that is written and filed is a diary); the risk-table rows on routing-as-a-sink and on reopening a settled tension; DoD-7 is its integration bar |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/route-and-escalate/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (U3's row; the states list, where **escalated** is a *distinct state* from routed; IQ-1, IQ-2, IQ-3, IQ-4, IQ-7; UX-AC-011) and `## Testing brief` (the AC-007 and AC-011 tier rows; the "do not automate into a check that nothing can fail" note) |
| Grounding | [`../_grounding.md`](../_grounding.md) — `## Routing destinations for AC-007 — verified against real project files` (lines 94–152): all three destination classes checked against real files, plus the reciprocal declarations from HS-P0022 and HS-P0023 |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, signed off 2026-08-17 with no conditions. No public API item, no rendered UI surface, no token file; the composition bound is the named document-primitive layer |
| Upstream story spec | [`../friction-log-skeleton/spec.md`](../friction-log-skeleton/spec.md) — fixes the log's path, its eight `##` sections, the `FL-###` id scheme, the five disposition arms and the revisions slot this story writes into |
| Roadmap pointer | [`../_storymap.md`](../_storymap.md) — `## Merge order`, step 3 (`dispositions-and-routing`); `## Coverage`, the AC-007 and AC-011 rows (sole owner of both) |

## One-line PR slice

Submit the log to a named owner able to act and record that submission, give every routed item a
destination id that resolves to a real item, and record anything that would reopen a sibling's
resolved tension as an escalation carrying its DT id instead of absorbing it here.

## Executive summary

This PR turns the log from a record into a **dispatch**. `disposition-every-stumble` (HS-S0166) has
just guaranteed that every severity-marked item carries exactly one arm; this story makes the two
outward-pointing arms — `routed:` and `escalated:` — *land*, and records the one act that research 04
calls the non-optional final step: the log reaching a named person with authority to act.

The delta against the project charter is narrow and specific. `project.md` AC-007 already says routing
must be "real and correct"; this story decides **what "correct" is checkable against** — the
ownership tables in [`../../_decomposition.md`](../../_decomposition.md), so a destination is derived
from who owns the requirement rather than from who seems likely to care. `project.md` AC-011 already
says a reopened tension is an escalation; this story decides **where the escalation is written, in
what shape, and what makes it disjoint from a fix** — because the slice-mate
`content-fixes-from-dispositions` (HS-S0168) is landing fixes in the same context, and the failure
mode is not that someone refuses to escalate but that a small fix quietly re-decides DT-1 or DT-6 on
the way past.

Three decisions this PR makes that no earlier artifact made:

1. **The submission record lives in a slot that survives the deletion of the dispositions index.** It
   is evidence, not a roll-up, and IQ-2's deletion test would otherwise erase it.
2. **A routed item carries the *reason* the destination is correct**, as one line naming the id the
   owner owns (a BR, an AC, or a DT from the initiative tables). A bare `routed: HS-P0022` is real and
   still unfalsifiable.
3. **`escalated:` and `fixed:` are disjoint sets, and zero escalations is an assertion.** A log with no
   escalation section says none were owed; it does not leave the question unasked.

## Context pack

**Read this section and you can start. Everything below `## Anchors` is deferred depth, not optional
depth — open an anchor when its bound AC says to.**

### The decision this story exists to make

**A log with no destination is opinion by another name, and a destination that is a description is no
destination at all.** Research 04's finding is quoted in the project charter without softening: routing
is the *final, non-optional step* of the method, and derived requirement 8 states the failure in one
line — *"The log is submitted to a named owner with authority to act, and that submission is recorded.
A log that is written and filed is a diary"* (`../project.md`, `## Derived requirements`). The UX
brief turns that into a hard shape rule, IQ-7: *"'Route to the docs team' is not a destination.
`HS-P0022`, `HS-P0023`, the `support` initiative (`.redkiln/config.yaml`, `support_initiative:
support`, line 5) or a named deferral are. A routed item without an id fails AC-006 as surely as an
undispositioned one."*

The second half of the story is the mirror image. This project's output flows **backwards** into
projects that have already merged and already had their design tensions signed off. The initiative
assumes the publication project's design gate stays closed, and states what happens if it does not:
*"If it turns out that it does, that is escalated, not absorbed"* (`../../initiative.md`, lines
645–647). AC-011 is that assumption made checkable.

### The persona-journey slice this realizes

**U3, the downstream actor** — a sibling-project owner, the `support` initiative, or HS-P0025 — is the
only user this story serves, and the UX brief states both what they are trying to do and what must
never happen to them: *"Open the log, find the items that are theirs, and act — without reading the
whole thing and without asking the logger what an entry meant"*, and never *"Reaching an item whose
destination is a description rather than an id, or whose disposition is implied by silence"*
(`../_decomposition.md`, `## UX brief`, the three-user table).

Two of those downstream actors have **already written down, in their own charters, that they will
accept work from here** — which is what makes this an integration and not a hand-wave:

- HS-P0022 `application-author-path`, `## Out of scope` (lines 140–143): *"If the log shows this
  project's bridge does not land, the disposition routes through HS-P0024, not back into this charter
  silently."*
- HS-P0023 `reach-and-adapter-path`, risk table (line 294): *"If the friction log shows the surfaced
  version does not carry an adapter author, a new account is owed and is dispositioned through
  HS-P0024 — not absorbed here."* Its line 292 separately concedes that its own DoD-9 second questions
  are author-chosen and that *"HS-P0024 is the instrument that can falsify the choice."*

This story routes findings **out in exactly the shape those two projects declared they would accept
them** (`../_decomposition.md`, `## UX brief`, `### Notes`, *Escalation has a shape already on
record*). Inventing a different escalation shape here would be inventing a protocol for a recipient
who has already published theirs.

### The decisions this story must honor, stated as decisions

1. **A destination is an id that resolves to a file on disk, and the three destination classes are
   fixed by the charter — not chosen per item.** `project.md` AC-007 enumerates them: *library bugs to
   the `support` initiative (`.redkiln/config.yaml:5`), page and structure defects to the sibling
   project that owns the requirement, deliberate deferrals to a recorded open question.* All three are
   verified real (`../_grounding.md`, lines 94–152): `.bklg/support/initiative.md` is `HS-I0005`;
   `.bklg/docs-that-teach/application-author-path/project.md` is `HS-P0022` and
   `.bklg/docs-that-teach/reach-and-adapter-path/project.md` is `HS-P0023`; `.kb/open-questions/`
   holds nineteen atoms today as the shape a deferral is staged toward.

2. **"The sibling project that owns the requirement" is a lookup, not an opinion.**
   [`../../_decomposition.md`](../../_decomposition.md) carries three ownership tables — business
   requirements (lines 173–192), acceptance criteria (lines 200–215) and design tensions (lines
   148–159) — and every BR, AC and DT has exactly one owner. So a routed item names the id it is
   routed to **and** the initiative id that owner owns which makes it theirs. This is the mechanism
   that answers the project's own risk-table row: *"Dispositions become a sink: everything is 'routed'
   and nothing is fixed… A routed item with no id fails the criterion."*

3. **Escalation is a distinct state from routing, and it carries a DT id.** The UX brief's states list
   is explicit: *"**Escalated.** A distinct state from routed: a disposition that would reopen a
   resolved design tension carries the **DT id** and does not become a fix here (AC-011)."* The two
   arms take different vocabularies — an item id versus a DT id — which is exactly why
   `friction-log-skeleton` listed `escalated:` as a fifth arm rather than folding it into `routed:`
   (its `## Clarifications resolved during spec`, item 7): folding them would make the mechanical
   DT-id check impossible.

4. **The DT id must exist.** The testing brief's AC-011 row makes this the one mechanical half of an
   otherwise reviewer-read criterion: *"Any escalation's DT id is checked against
   `../_decomposition.md`'s `## Design tension ownership` table (`rg "^\| DT-" ../_decomposition.md`);
   the escalation's substance stays reviewer-read… An 'escalation' citing a DT id that does not exist
   is caught mechanically, not trusted."* Six of the ten tensions are owned by projects this story can
   route to (DT-1/4/5/6 → HS-P0022, DT-2/3/8 → HS-P0021, DT-7 → HS-P0020, DT-10 → HS-P0023); **DT-9 is
   this project's own** and an "escalation" naming DT-9 is a category error — this project resolves
   DT-9, it does not escalate it.

5. **`escalated:` and `fixed:` are disjoint, and the seam is with a story landing in the same
   context.** `content-fixes-from-dispositions` (HS-S0168) is a slice-mate and lands the *fixed* arm.
   The project charter draws the line: small content fixes are in scope *"where the fix does not reopen
   a settled design tension. Anything that would reopen one is escalated, not absorbed"* (`../project.md`,
   `## In scope`). The risk this addresses is stated in the project risk table as Medium/High: *"The
   four content tensions were signed off in HS-P0022's design review; re-deciding one here would put
   half a decision in each."* So the escalation set is written down as a set, and the fix story's inbox
   is checked against it rather than trusted to be disjoint by good intentions.

6. **The submission is an act with a record, and the record is not derivable.** Derived requirement 8
   asks for a named owner *with authority to act* and a recorded submission. IQ-2 says any roll-up must
   survive its own deletion — so the submission record cannot live in `## Dispositions index`, which is
   explicitly the derived and deletable section (`../friction-log-skeleton/spec.md`, `## Behavior and
   interfaces`, the `## Dispositions index` row: *"Deleting the whole section must lose nothing"*). The
   submission record therefore lands in a **persistent** section, as a labelled block inside
   `## Hand-off`.

7. **No ninth `##` section.** `friction-log-skeleton`'s AC-001 is checked by
   `rg -n "^## " <scaffold>` returning **exactly** its eight headings in that order. Adding a top-level
   section for routing or escalation breaks a criterion an earlier story already banked. Everything
   this story writes goes inside the existing eight — the disposition and revisions slots of each
   `### FL-###` entry, the `## Dispositions index`, and `## Hand-off`.

8. **In place, not a context jump (IQ-1).** The destination id is written **at the stumble**, in that
   entry's `Disposition:` field. `## Dispositions index` is a per-owner extract that exists *in
   addition* — one optional hop, never the only home of a destination. An index that is the only place
   a route is recorded has already failed IQ-1 and IQ-2 simultaneously.

9. **A change of destination appends; it never overwrites (IQ-4).** A re-route, a withdrawn escalation
   or a destination corrected after the owner says "not mine" is written into the entry's `Revisions:`
   slot with its date and reason, and the original stays legible. The authority is
   [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
   *"applied to dispositions: the referent may be rewritten, the reasoning may not be erased"*, and the
   brief names it as what makes escalation safe — *"an escalation that later proves unnecessary can be
   withdrawn without the trail vanishing"* (IQ-4).

10. **Ids cited outward are public identifiers (IQ-3).** The moment a routed item names `FL-004` in a
    submission to HS-P0022, that anchor is a citation someone else holds. Ids are never reassigned, the
    chronological section stays append-only, and the heading line — id **and** label — is frozen. *"A
    later reader landing on a cited anchor must find the item that was cited."*

11. **A deferral stages material; this project writes nothing under `.kb/`.** The third AC-007
    destination is "a recorded open question", and `.kb/open-questions/` is atom-shaped — but
    `CLAUDE.md`'s binding rule is that atoms are authored only through `/redkiln:kb-ingest` or
    closeout, and the first attempt at hand-authoring was reverted (`0269720`). *"A deferral
    disposition therefore stages the question for ingest or for HS-P0025's promotion pass, with the id
    of that hand-off recorded. This project edits nothing under `.kb/`"* (`../_decomposition.md`,
    `## UX brief`, `### Notes`).

12. **`support` is real and structurally empty — say so, do not imply a backlog.**
    `.bklg/support/initiative.md` exists as `HS-I0005` at `stage: intake` with zero projects. Routing a
    library bug there is correct; describing it as an active backlog with stories to slot into is not
    (`../_decomposition.md`, `## UX brief`, `### Notes`).

13. **Do not automate this into a check nothing can fail.** The testing brief forbids by name a script
    that merely confirms a `Disposition:` line exists — *"it would pass `noted` as readily as `routed to
    HS-P0022, see stumble #4`"*. Where this story adds a check, the check asserts on **content**: an id
    pattern that resolves to a real path, a DT id present in the ownership table, a disjointness test
    between two named sets.

### What this story is explicitly not deciding

Whether each stumble *has* a disposition at all — that is `disposition-every-stumble`'s (project
AC-006) and lands immediately before this story in the same slice. What the fix *is* for any `fixed:`
arm — that is `content-fixes-from-dispositions`'s. The scope sentence, the second-session verdict and
the hand-off note's persona/date/tree slots — those are `scope-the-claim`, `second-session-decision`
and `handoff-note-to-closeout`'s, one slice later. And no story here re-opens the log's shape: the
eight sections, the arm list and the revisions slot are `friction-log-skeleton`'s, and if `_design.md`
fixed a different vocabulary, **that wins verbatim**.

### The one ordering constraint that can invalidate this work

`disposition-every-stumble` (HS-S0166) must be committed first. Routing before every stumble carries
exactly one arm means routing an unstable set: an item that gains a disposition afterwards is a routed
item nobody submitted, and an item that turns out to be double-dispositioned is a destination sent to
two owners. Both stories are in the `dispositions-and-routing` slice and are implemented sequentially
in one context (`../_storymap.md`, `## Merge order`, step 3).

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `capability` — a user-observable slice through the whole artifact, for U3 the downstream actor (`../_storymap.md`, slice table) |
| **Slice / milestone** | `dispositions-and-routing` |
| **Slice-mates** | `disposition-every-stumble` (HS-S0166, lands **before** this story — hard edge), `content-fixes-from-dispositions` (HS-S0168, independent of this story and may land either side of it; its inbox is constrained by this story's escalation set). All three are implemented together in one context and mounted as one integrated surface (`../_storymap.md`, `## Merge order`, step 3) |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` — the project's real render path, landed and mounted by `friction-log-skeleton` and reachable in one hop from `project.md`'s `## Companions` list. Two seams inside it: **`## Dispositions index`** (the per-owner extract U3 arrives at) and **`## Hand-off`** (the persistent home of the submission record). Destinations are written at each `### FL-###` entry's `Disposition:` field; the index is additive |
| **Wires into** | `.redkiln/config.yaml:5` `support_initiative: support` (the library-bug destination); [`../../_decomposition.md`](../../_decomposition.md)'s three ownership tables — BR (173–192), AC (200–215), DT (148–159) — which are the lookup that makes a destination *correct* rather than plausible; the five disposition arms and the `Revisions:` slot fixed by [`../friction-log-skeleton/spec.md`](../friction-log-skeleton/spec.md); [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) at the revision seam; `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract, which is what a stable `FL-###` anchor is *for* |
| **Design-system primitives consumed** | [`docs/README.md`](../../../../docs/README.md)'s two-column routing table — "what you are looking for" against "where it is" — reused for `## Dispositions index` rather than replaced; the `\| Risk \| Likelihood / Impact \| Mitigation \|` table shape already used across this initiative; the one-line checkbox from [`.redkiln/templates/gates/`](../../../../.redkiln/templates/gates/) if any checklist is written. No new format, no new heading vocabulary, no navigation widget (`../_decomposition.md`, `#### The primitive layer to compose from — do not hand-roll`) |
| **Renders surfaces** | **None as an `_design.md` item id** — [`../_design.md`](../_design.md)'s `## Items` block is empty (`# no items — no public API surface, no rendered UI surface`) and every shape section reads `N/A`. This story changes surface **2** of the three *document* surfaces the UX brief enumerates (the friction log), which `_design.md` names in prose rather than as an item |
| **Conformance rule(s) / clause(s)** | **None, and deliberately.** This story adds no `pub` item, compiles nothing, touches no crate under `crates/` and discharges no `SPECIFICATION.md` clause; it is not adapter-observable. Its instruments are the Static (existence / id-resolution) and Artifact-evidence tiers the project's `## Testing brief` defines for AC-007 and AC-011, plus `cargo xtask affected --base main`'s fall-through to the five file-reading lints and `spec-trace` |
| **Advances DoD scenario** | **Initiative DoD scenario 6** — *"Every stumble in that log has a disposition… each marked item is traceable to a fix, a recorded deliberate acceptance, or an item **routed elsewhere**"* (`../../initiative.md`, DoD 6; `../../_decomposition.md`, `### Definition of Done`, row 6; owner HS-P0024). `disposition-every-stumble` makes every item carry an arm; **this story is what makes the routed arm traceable**, which is the half of scenario 6 that the word "elsewhere" carries. It also discharges initiative **AC-10** — *"Someone acted on what that reader found"* — whose failure mode the charter names as "merely archived" |

**Delivered mounted, not as an isolated component.** The acceptance bar includes that a named human
outside this story received the log and that the receipt is on disk, and that U3 arriving at
`project.md` from `redkiln board` reaches the items that are theirs in at most two hops — one to the
log, one optional hop through the index to the entry.

## PR boundary

**In this PR**

- Every `routed:` arm in `_friction-log.md` completed to an id that resolves to a real file, each with
  its one-line ownership reason.
- Every `escalated:` arm completed to a DT id that exists in the ownership table, with its owning
  project and what would have to be re-decided.
- The escalation set written down as a set, so the fix story's inbox can be checked against it.
- The submission record — named owner, their authority to act, the date, and what was submitted at
  which commit — as a labelled block inside the log's existing `## Hand-off` section.
- The `## Dispositions index` filled as a derived per-owner extract in `docs/README.md`'s two-column
  shape, stating in its own first line that the chronological record is authoritative.
- Any `Revisions:` appends this story's own re-routing produces.
- This story's own `_ledger.md` and stage artifacts.

**Explicitly not in this PR**

- **Deciding whether an item has a disposition at all.** That is `disposition-every-stumble`'s
  (project AC-006) and lands first.
- **Any content fix.** The `fixed:` arm is `content-fixes-from-dispositions`'s; this story's only
  relationship to it is the disjointness constraint.
- **Any edit to a sibling project's charter.** HS-P0022 and HS-P0023 have already declared in their own
  files that they accept dispositions from here; routing names their id, it does not write into their
  scope sections. A destination that seems to require editing the destination is an escalation being
  performed instead of recorded.
- **Any file under `.kb/`** — including `.kb/open-questions/`. A deferral stages material with the id
  of the hand-off recorded; atom authorship is closeout's, through the ingest path (`CLAUDE.md`;
  `0269720`).
- **Any new `##` section in the log**, any new heading vocabulary, any navigation widget, and any
  reordering or renumbering of `### FL-###` entries.
- **Any `redkiln new` / `redkiln advance` invocation**, and any edit to item frontmatter. Routing
  records a destination id; it does not create the destination's backlog items.
- **Any automation that checks a `Disposition:` line merely exists.** Forbidden by name in the testing
  brief: it passes `noted` as readily as a real routed id.

The implementer **may** also touch the composition-root/wiring seams named in the Integration
contract — `_friction-log.md`'s `## Dispositions index` and `## Hand-off` — to mount this slice; that
is not scope drift.

**Merge DoD one-liner** — every routed item names an id that `test -f` resolves, every escalation names
a DT id present in `../../_decomposition.md`'s ownership table with an owning project that is not
HS-P0024, the escalation and fix sets are disjoint, the submission record names a real person and a
date and survives deletion of the index, and `cargo xtask affected --base main` plus
`redkiln validate --kb && redkiln doctor` are clean at exactly the six standing `template-drift`
advisories.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under this
heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/route-and-escalate/**
```

Two paths, narrower than the upstream foundation story's four, and deliberately so: this story writes
into a file that already exists and mounts nothing new. If a change here appears to need a third path
— a sibling charter, a `.kb/` atom, a new backlog item — that is the signal that an escalation is being
*absorbed* rather than *recorded*, which is precisely what AC-011 forbids.

## Behavior and interfaces

The "interface" here is the destination vocabulary and the two records that make routing checkable:
what a routed entry says, and what the submission block says.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The routed arm's id vocabulary is closed** | Exactly three destination classes, from `project.md` AC-007: **(a)** a library bug → the `support` initiative, `HS-I0005`, per `.redkiln/config.yaml:5`; **(b)** a page or structure defect → the sibling **project id** that owns the requirement (`HS-P0020` … `HS-P0023`, or `HS-P0025` for anything about the audience model); **(c)** a deliberate deferral → a **staged** open question with the id of the hand-off that will carry it to ingest or to HS-P0025's promotion pass. Nothing else is a destination; a prose destination is not expressible | `../project.md`, AC-007; `.redkiln/config.yaml:5`; `../_grounding.md:94-152`; `../_decomposition.md`, IQ-7 / UX-AC-011 |
| **A routed entry carries the reason it is that owner's** | One line beside the id naming the initiative id the destination owns which makes the item theirs — a BR, an AC or a DT from `../../_decomposition.md`'s tables (e.g. *"routed: HS-P0021 — owns BR-04, the answered-need rule this page breaks"*). Without it, the correct-owner half of AC-007 is unfalsifiable and the routing table becomes the sink the risk register names | `../../_decomposition.md`, lines 148–159 / 173–192 / 200–215; `../project.md`, `## Risks and coupling notes`, row 3 |
| **The escalated arm's id vocabulary is the DT table** | `escalated: DT-<n>` where `DT-<n>` appears in `../../_decomposition.md`'s `## Design tension ownership` table, plus the **owning project** from that same row and one line stating **what would have to be re-decided**. `DT-9` is excluded by construction: it is HS-P0024's own tension, resolved in `../_design.md`, and naming it as an escalation is a category error | `../../_decomposition.md:148-159`; `../project.md`, AC-011; `../_decomposition.md`, `## Testing brief`, AC-011 row |
| **Escalation shape matches what the recipients published** | The escalation reads as the mirror of the recipient's own declaration — HS-P0022's *"routes through HS-P0024, not back into this charter silently"* and HS-P0023's *"a new account is owed and is dispositioned through HS-P0024 — not absorbed here."* The symmetry is the point: this project routes findings out in exactly the shape those two declared they would accept them | `.bklg/docs-that-teach/application-author-path/project.md:140-143`; `.bklg/docs-that-teach/reach-and-adapter-path/project.md:292,294`; `../_decomposition.md`, `## UX brief`, `### Notes` |
| **Escalated and fixed are disjoint sets** | The escalation set is written down as a set (in `## Dispositions index`, derived from the entries) so `content-fixes-from-dispositions` can be checked against it. An item appearing in both is a settled tension being re-decided inside a content fix — the Medium/High risk the project charter names, where *"re-deciding one here would put half a decision in each"* | `../project.md`, `## In scope` (fixes only where no settled tension is reopened) and `## Risks and coupling notes`, row 4 |
| **Zero escalations is an assertion, not a silence** | If no stumble would reopen a tension, the log says so explicitly, in the same way a session log with zero facilitator interventions *asserts* that none occurred (IQ-5's discipline applied to AC-011). "Not mentioned" is never one of this project's states | `../_decomposition.md`, `## UX brief`, `#### The states this surface has to express`; UX-AC-009's shape |
| **The submission record — five fields, in a persistent slot** | A labelled block inside the log's existing `## Hand-off`: **owner** (a named person, not a team); **authority** (one line on why that person can act — the role or the item they own); **date**; **what was submitted** (the log at a named commit); **how** (the channel, so a reviewer can check the claim rather than take it). It lives in `## Hand-off` and **not** in `## Dispositions index`, because the index is derived and deletable and the submission record is evidence | `../project.md`, AC-007 and derived requirement 8; `../friction-log-skeleton/spec.md`, `## Behavior and interfaces` (the index is derived; deleting it must lose nothing); `../_decomposition.md`, IQ-2 |
| **No ninth `##` heading** | Everything written by this story lands inside the eight sections `friction-log-skeleton` fixed. Its AC-001 check is `rg -n "^## " <scaffold>` returning exactly those eight in order; a new top-level section breaks a criterion an earlier story banked | `../friction-log-skeleton/spec.md`, AC-001; `../_storymap.md`, `### Why the slices fall here` |
| **Destination at the stumble; index additive** | The id is written in the entry's own `Disposition:` field. `## Dispositions index` is a per-owner extract in `docs/README.md`'s two-column shape, whose first line states that it is derived and that the chronological record is authoritative. Deleting it must lose no destination, no reason and no escalation | `../_decomposition.md`, IQ-1 / IQ-2 / UX-AC-005 / UX-AC-006; `docs/README.md` (the primitive) |
| **Re-routes append, never overwrite** | A destination corrected after an owner declines it, or an escalation withdrawn as unnecessary, is a dated line in that entry's `Revisions:` slot carrying the earlier destination and the reason for the change. The original text is never edited | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `../_decomposition.md`, IQ-4 / UX-AC-008 |
| **Cited anchors stay resolvable** | `FL-###` ids are never reassigned, the chronological section stays append-only, and heading lines stay frozen — because the moment an id is named in a submission it is a citation someone else holds, and a re-worded label kills it with no error message | `../_decomposition.md`, IQ-3 / UX-AC-007; `../friction-log-skeleton/spec.md`, AC-005 |
| **A deferral stages; it never writes under `.kb/`** | The deferral destination records the question and the id of the hand-off that carries it to `/redkiln:kb-ingest` or to HS-P0025's promotion pass. `.kb/open-questions/` holds nineteen atoms today as the *shape*; this project authors none of them | `../_decomposition.md`, `## UX brief`, `### Notes` (*A deferral is staged material*); `CLAUDE.md`, `## Where the work lives`; `0269720` |
| **`support` is described accurately** | Routing there is correct per `.redkiln/config.yaml:5`, and the log says plainly that `HS-I0005` is at `stage: intake` with zero projects — real, structurally empty. A routed bug must not read as though it landed in a populated backlog | `.bklg/support/initiative.md`; `../_grounding.md:101-108`; `../_decomposition.md`, `### Notes` |
| **Complete as static text, one line per checkbox** | Everything above is readable in a `git diff` with no rendering step; no colour, emoji or fold carries meaning; any checkbox stays on one line because the gate parser matches line by line | `../_decomposition.md`, `#### The accessibility floor`; `CLAUDE.md`; `.redkiln/templates/gates/` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds no crate, no type, no table and no
connection; it edits one existing markdown file. `../_design.md` records `hasSurface: false` with an
empty items block for exactly this reason.

Two things behave enough like data contracts to be named, because a later reader will foreign-key into
both:

- **The destination id is a foreign key into the backlog.** Its referential integrity is checkable with
  `test -f`: `HS-I0005` → `.bklg/support/initiative.md`; `HS-P0020` … `HS-P0025` → the six
  `.bklg/docs-that-teach/<slug>/project.md` files; `DT-1` … `DT-10` → a row in
  `../../_decomposition.md`'s ownership table. There is no fourth namespace, and an id outside these
  three is a dangling reference, not a new destination class.
- **The `FL-###` stumble id is the primary key on the other side.** Once a submission names `FL-004`,
  that anchor is held by someone outside this repository's control of the log. The only legal mutation
  is an append — a `Revisions:` line. Renumbering is not a migration; it is a broken citation with no
  error message (`../_decomposition.md`, IQ-3).

The nearest thing to a "migration" this story can produce is a **re-route**: a destination that the
receiving owner declines. It is handled by the reversibility rule rather than by an edit — the new
destination is appended with its date and reason, the earlier one stays legible, and no id changes.

## Acceptance criteria

Eight criteria. Each is framed from the intent of the person it serves crossing the whole artifact —
**U3 the downstream actor** (a sibling-project owner, the `support` initiative, HS-P0025), **U2 the
facilitator** who has to make the submission actually happen, and the reviewer six months later with
nobody left to ask — because a criterion framed as a bare capability ("routed items have ids") is
satisfied by an id that points nowhere. The personas are
[`../_decomposition.md`](../_decomposition.md)'s `## UX brief` three-user table; the journey they cut
through is [`../_storymap.md`](../_storymap.md)'s backbone **A4 — give every stumble a destination**.

Throughout, **the log** means `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, **the
token** means the emptiness marker `NOT-YET-RECORDED` that `friction-log-skeleton` fixed, **the
ownership tables** means the three tables in `.bklg/docs-that-teach/_decomposition.md` (BR 173–192,
AC 200–215, DT 150–159), and **the submission block** means the `### Submission record` sub-block
inside the log's existing `## Hand-off` section, fixed under `## Clarifications resolved during spec`.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** derived requirement 8 states that a log which is written and filed is a diary, **WHEN** a reviewer six months later asks who received this log and whether that person could actually do anything about it, **THEN** `## Hand-off` carries a `### Submission record` block of five one-line labelled fields — **Owner** (a named person, never a team), **Authority** (one line on why that person can act: the role, or the item they own), **Date**, **Submitted** (the log at a named commit), **Channel** (how, so a reviewer can check the claim rather than take it) — and deleting the whole of `## Dispositions index` leaves that block untouched, because the submission is evidence and not a roll-up | Static: `rg -n "^### Submission record" <log>` returns exactly one hit, and it falls after the `## Hand-off` heading; `rg -n "^## " <log>` still returns the skeleton's eight headings in that order; no `NOT-YET-RECORDED` remains inside the block; the commit named in **Submitted** resolves via `git show <commit>:<log>`. Plus the IQ-2 deletion check performed against a scratch copy. Artifact-evidence: the ledger cites the block by `file:line`. Rejects the three failures the charter names — a team in the Owner field ("route to the docs team" wearing a different hat), a submission recorded in the derived-and-deletable index where IQ-2's own test erases it, and a record with a date but no channel, which asserts an act nobody can verify |
| **AC-002** | **GIVEN** U3 opens the log to find the items that are theirs without reading the whole thing, **WHEN** they read a routed entry's `Disposition:` field, **THEN** it names an id from the closed three-class vocabulary — `HS-I0005`, the `support` initiative per `.redkiln/config.yaml:5`; a sibling project id (`HS-P0020` … `HS-P0023`, or `HS-P0025` for anything about the audience model); or a named staged deferral carrying the id of the hand-off that will carry it — and every one of those ids resolves to a real file on disk, so not one destination in the log is a description | Static existence, exactly as `../_decomposition.md`'s `## Testing brief` AC-007 row specifies: `test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/application-author-path/project.md`, `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md`, and a `test -f` for every other id the sweep finds. Static shape: `rg -n "^Disposition: routed:" <log>` piped against the id pattern returns no payload without a resolving id. Rejects IQ-7's own named failure, "route to the docs team", and the subtler one — a real-looking id (`HS-P0026`, `HS-P0019`) that no file answers to |
| **AC-003** | **GIVEN** "the sibling project that owns the requirement" is a lookup in the ownership tables rather than a judgement about who seems likely to care, **WHEN** the receiving owner asks *why is this mine*, **THEN** the same one-line `Disposition:` field carries, beside the id, the initiative id that owner owns which makes the item theirs — a **BR**, an **AC** or a **DT** from the ownership tables — so the *correct-owner* half of project AC-007 is falsifiable against a table instead of believed | Static: every cited id appears in its table (`rg "^\| BR-"`, `rg "^\| AC-"`, `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md`) **and** that row's owner is the same project the item was routed to — a two-sided check, because a real BR id owned by a different project is the exact wrong routing this criterion exists to catch. Artifact-evidence: the ledger cites each routed entry by `file:line`. Rejects a bare `routed: HS-P0022`, which is real and still unfalsifiable, and a "reason" that names a topic ("docs stuff") rather than an owned id |
| **AC-004** | **GIVEN** a library bug and a deliberate deferral are the two destinations with nothing on the other end able to object, **WHEN** U3 or HS-P0025 follows one of them, **THEN** a `support`-routed item states plainly that `HS-I0005` is at `stage: intake` with zero projects — real, and structurally empty — and a deferral records the question together with the id of the hand-off that will carry it to `/redkiln:kb-ingest` or to HS-P0025's promotion pass, with the diff showing not one file added or changed anywhere under `.kb/` | Static: `git diff --name-only` for this PR contains no `.kb/` path (and the `## PR boundary` fenced block makes one impossible); `rg -n "HS-I0005" <log>` shows the emptiness statement alongside the routing. Artifact-evidence: the ledger cites the deferral line and its hand-off id. Rejects a routed bug written as though it landed in a populated backlog with stories to slot into, and a deferral that hand-authors an open-question atom — the move `CLAUDE.md` forbids and that was reverted once already (`0269720`) |
| **AC-005** | **GIVEN** the initiative assumes the publication project's design gate stays closed and states that *"if it turns out that it does, that is escalated, not absorbed"*, **WHEN** the session finds a stumble whose remedy would change a tension already signed off in a sibling's design review, **THEN** its `Disposition:` field reads `escalated: DT-<n>` where `DT-<n>` is a row in `.bklg/docs-that-teach/_decomposition.md`'s `## Design tension ownership` table, names that row's owning project, and states in one line what would have to be re-decided — and `DT-9` never appears, because DT-9 is HS-P0024's own tension and resolving it is not escalating it | Static id existence, the one mechanical half the testing brief's AC-011 row specifies: every escalated DT id is a member of the set `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` yields; the owning project named in the entry equals that row's owner; `DT-9` appears in no escalation. Artifact-evidence: the escalation's substance stays reviewer-read with a `file:line` per escalation. Rejects an "escalation" citing a DT id that does not exist — *"caught mechanically, not trusted"* — one naming DT-9 as a category error, and one that names a tension without saying what would have to be re-decided, which gives the recipient nothing to act on |
| **AC-006** | **GIVEN** `content-fixes-from-dispositions` is landing fixes in the same slice and the project risk table rates re-deciding a signed-off tension Medium/High — *"re-deciding one here would put half a decision in each"* — **WHEN** that story picks up its inbox, **THEN** the escalated set and the fixed set share no `FL-###` id, `## Dispositions index` carries the escalation roll-up derived from the entries so the two sets can be compared without re-reading the log, and if no stumble would reopen a tension the log **says so** in that roll-up rather than leaving the question unasked | Static set intersection: the `FL-###` ids from `rg -n "^Disposition: escalated:" <log>` and from `rg -n "^Disposition: fixed:" <log>` intersect empty. Static assertion: where the escalated set is empty, an explicit zero-escalations sentence is `rg`-findable in the roll-up. Artifact-evidence: the ledger cites the roll-up and, if non-empty, each member. Rejects an item carrying both arms, and an absent escalation section read by a later reader as "none were owed" — the same discipline UX-AC-009 applies to zero facilitator interventions |
| **AC-007** | **GIVEN** a destination is declined by its owner ("not mine"), or an escalation later proves unnecessary, **WHEN** the correction is made, **THEN** the entry's `Revisions:` field gains a dated line carrying the earlier destination and the reason for the change, the original `Disposition:` text is not edited, and no `FL-###` id and no heading label anywhere in `## Chronological record` moves — so a submission that already cited `FL-004` outward still lands on the item it cited | Static: `git diff` on the log confines every hunk to `Disposition:` lines, `Revisions:` lines, the `## Dispositions index` block and the submission block; `rg -n "^### FL-" <log>` returns byte-identical output before and after this PR. Artifact-evidence: the ledger cites each appended revision line. Authority: [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md) — the referent may be rewritten, the reasoning may not. Rejects an in-place overwrite of a declined destination, and a heading label "tidied" during review, which kills a live citation with no error message |
| **AC-008** | **GIVEN** U3 arrives at HS-P0024 from `redkiln board` needing only the items that are theirs, **WHEN** they open the project card, **THEN** one hop from `../project.md`'s `## Companions` list reaches the log and at most **one optional** further hop through `## Dispositions index` reaches the entry; every destination, ownership reason and escalation is readable at its own entry; the index is a two-column table in `docs/README.md`'s existing shape whose first line states it is derived and that the chronological record is authoritative; the log still carries exactly the skeleton's eight `##` sections; and everything this story added is static text — no fold, no widget, no script, nothing carrying meaning by colour or emoji alone, every checkbox on one line | Static: `rg -n "^## " <log>` returns exactly the eight headings in order (no ninth section); `rg -n "<details>\|<summary>\|<script>" <log>` returns nothing; every checkbox line matches a single-line `- [ ] …`; `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md` returns the Companions row and `git diff` shows no frontmatter hunk. Plus the IQ-2 deletion check: delete the index and no destination, no reason and no escalation is lost. Rejects a destination that lives only in the index (IQ-1 and IQ-2 failed at once), a ninth `##` section — which breaks `friction-log-skeleton`'s already-banked AC-001 — and the bespoke navigation widget `interaction-patterns.md` names as an anti-pattern for a medium that renders the equivalent for free |

**Coverage of the traced project ACs.** `../project.md` **AC-007** — *"Routing is real and lands
correctly"* — is carried by **AC-001** (the submission to a named owner able to act, and its record),
**AC-002** (each destination is a resolving id), **AC-003** (it is the *correct* owner, checkable
against a table) and **AC-004** (the two destination classes with no counterparty are described
truthfully), with **AC-008** carrying the reach half of "lands". `../project.md` **AC-011** — *"No
settled decision was reopened in passing"* — is carried by **AC-005** (a real DT id, its owner, and
what would be re-decided) and **AC-006** (disjoint from the fix set, and zero is an assertion).
**AC-007** carries the reversibility and anchor-stability invariants both traced criteria depend on:
a destination that changes without a trail, or an id that moves, silently invalidates every claim
above it. No row claims project AC-006 — *whether* each stumble has a disposition at all is
`disposition-every-stumble`'s and lands immediately before this story in the same slice.

## Interaction quality

The blocking invariants, in two families. **Every one of them is carried by an `AC-###` row in the
table above** — this section says only which row carries which, and how each is verified. Nothing
here is a free-floating bullet: `redkiln verify` extracts ACs by matching a leading `| AC-001 |`
table cell or an `- AC-001:` bullet, so an invariant stated only as prose here would get no ledger
row, would never be gated, and would never be tested.

### State invariants

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the destination is readable at the stumble; the index is additive and the hop through it is optional and at most one | **AC-008** (reach), **AC-003** (the reason travels with the id, on the same line) | Every routed and escalated payload lives in the entry's own `Disposition:` field; a reviewer reading one entry learns the destination *and* why it is that owner's without navigating |
| **Non-occlusion** (IQ-2) — a filter must not hide what it filters; nothing exists only in a roll-up | **AC-008** (destinations), **AC-001** (the submission record), **AC-006** (the escalation roll-up) | The deletion check stated verbatim in `../_decomposition.md`'s IQ-2: delete `## Dispositions index` and no destination, reason, escalation or submission record is lost. The submission block lives in `## Hand-off` precisely because it must survive that deletion |
| **Preserved position** (IQ-3) — ids and order stable once written; a cited anchor still resolves | **AC-007** | `rg -n "^### FL-"` is byte-identical before and after this PR; no id reassigned, no heading label re-worded, no entry reordered — the anchors are public identifiers the moment a submission names one |
| **Reversibility** (IQ-4) — a changed destination is recorded *as a change*, the earlier one still legible with its reason | **AC-007** | A declined destination or a withdrawn escalation appends a dated `Revisions:` line; the original `Disposition:` text is never edited. Authority: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **Keyboard reachability** — the log stays navigable by browser find and stable heading anchors alone; no pointer-only affordance, no widget, no script | **AC-008** | Static-text completeness; `rg -n "<details>\|<summary>\|<script>"` returns nothing; nothing added requires a rendering tool to read |
| **Zero is an assertion, never a silence** (IQ-5's discipline, applied to AC-011) | **AC-006** | An empty escalation set is stated explicitly in the roll-up. "Not mentioned" is not one of this surface's states (`../_decomposition.md`, the states enumeration) |

### Composition invariants

[`../_design.md`](../_design.md) is signed off (2026-08-17, no conditions) with `hasSurface: false`
and an empty items block — it declares **no rendered surface and no public API item**, so there is no
pixel density budget, no token file and no component to compose from, and inventing one is out of
scope by its own words (*"there is no CSS layer and no token file, and inventing one is out of
scope"*). The composition constraints it **does** carry are the named repository *document*
primitives and the one-line checkbox, and those bind this story exactly as a component library would.

| Invariant | Real numbers / named source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — every control carries real composed presentation, not bare markup. A `Disposition:` field holding a bare id, or a `## Hand-off` heading with a sentence under it, is the document-medium equivalent of an unstyled render: it satisfies every structural check and tells the receiving owner nothing | The submission record is a **five-field labelled block**, not a sentence; a routed payload is **id + one-line ownership reason**; an escalated payload is **DT id + owning project + what would be re-decided**; the index is a **two-column table**, not a list of ids | **AC-001**, **AC-003**, **AC-005**, **AC-008** |
| **Composition and placement** — each thing this story writes maps to a named existing primitive, none hand-rolled | `docs/README.md`'s two-column routing table (the index); the one-line labelled-field shape `friction-log-skeleton` fixed for the stumble entry (the submission block reuses it); `.redkiln/templates/gates/discover.md`'s one-line checkbox if any checkbox is written. No new format, no new heading vocabulary, no navigation widget | **AC-001**, **AC-008** |
| **Transience** — what is persistent chrome, what is revealed, what is opened on demand | Persistent and visible-by-default: every entry's `Disposition:` and `Revisions:` fields, and the `### Submission record` block in `## Hand-off`. Derived and deletable: the whole of `## Dispositions index`, including the escalation roll-up. Opened on demand: **nothing** — no fold, no tab, no collapse, by rule | **AC-001**, **AC-006**, **AC-008** |
| **Density budget, with its numbers** | Exactly **3** destination classes; **5** submission fields; **9** legal escalation ids (DT-1 … DT-10 minus DT-9); **1** ownership-reason clause per routed item, on the same line as the id; **2** columns in the index; **8** `##` sections in the log, unchanged; **1** line per field and per checkbox; **0** folds and **0** scripts | **AC-002**, **AC-001**, **AC-005**, **AC-003**, **AC-008** |
| **Hierarchy** — the authoritative record is the chronological one, and the derived view says so in its own first line | `## Dispositions index`'s first line states it is derived and that `## Chronological record` is authoritative; the submission record sits in `## Hand-off`, the last section, because it is the act that closes the log rather than a finding within it | **AC-008**, **AC-001** |
| **Named anti-patterns** — a prose destination ("route to the docs team"); a destination or escalation living only in the index; a ninth `##` section; a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold; colour or emoji as the sole carrier of meaning; a wrapped checkbox; a `support` route implying a populated backlog; an "escalation" citing a DT id that does not exist | `../_decomposition.md`, IQ-7, `#### The primitive layer…`, `#### The accessibility floor` and `### Notes`; `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`; `CLAUDE.md` on the line-by-line parser; `../friction-log-skeleton/spec.md`, AC-001 | **AC-002**, **AC-008**, **AC-004**, **AC-005** |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | An entry still reads `not yet dispositioned`, or carries two arms, when this story starts | **Stop; do not route.** `disposition-every-stumble` owns project AC-006 and is the hard edge in this slice. Routing an unstable set produces a routed item nobody submitted, or one destination sent to two owners. Finish the upstream story in the same context first (`../_storymap.md`, `## Merge order`, step 3) |
| **EC-002** | The owning project for a stumble is ambiguous — two ownership-table rows appear to cover it, or none does | Route to the owner of the **requirement** the stumble breaks, not the owner of the page it was found on, and cite the id in the reason (AC-003). Where genuinely no row covers it, that is itself a finding: append it as an entry and disposition it. **Never invent a fourth destination class** — the vocabulary is closed by `../project.md` AC-007 |
| **EC-003** | The destination owner declines the item after submission | Append a dated `Revisions:` line with the earlier destination and the reason, then write the new destination (AC-007). The original `Disposition:` text is not edited. A decline is evidence about the routing, not an embarrassment to erase |
| **EC-004** | An item dispositioned `fixed:` turns out to reopen a settled tension | Revise it to `escalated: DT-<n>` through the `Revisions:` slot and remove it from the fix inbox. The two sets stay disjoint (AC-006); an item carrying both arms is the Medium/High risk the project charter names, half a decision in each place |
| **EC-005** | An escalation names `DT-9` | Category error. DT-9 is HS-P0024's own tension, resolved in `../_design.md` and signed off 2026-08-17. Re-classify: either it is a finding about this project's own protocol — appended and dispositioned here — or it belongs to one of the nine tensions another project owns |
| **EC-006** | A cited DT id is not a row in `.bklg/docs-that-teach/_decomposition.md`'s ownership table | Caught mechanically by AC-005's check, and a hard failure rather than a formatting nit: an escalation whose tension does not exist routes to nobody. Correct the id against the table, or the disposition is not an escalation at all |
| **EC-007** | Routing an item appears to require editing a sibling project's charter, creating a backlog item, or opening a `.kb/` atom | That is an escalation being **performed** instead of **recorded** — precisely what AC-011 forbids. The `## PR boundary` fenced block is two paths and `redkiln verify --grain story` fails on a third. Record the destination id; the destination's owner creates their own work |
| **EC-008** | No owner with authority to act can be named, or the submission cannot be made before the story closes | The artefact is **unmet**, and `## Hand-off` says so in those words. Do not substitute a team, a mailing list or "the repository owner, presumably". This mirrors the project risk table's first row: a failure of the artefact is not a reason to redefine the bar |
| **EC-009** | A deferral looks like it needs an `.kb/open-questions/` atom written now | Stage the question and record the id of the hand-off that carries it to `/redkiln:kb-ingest` or HS-P0025's promotion pass. `.kb/` atom authorship is closeout's, through the ingest path; the first attempt at hand-authoring was reverted (`0269720`) |
| **EC-010** | `friction-log-skeleton` already landed a submission slot inside `## Hand-off` | Use it **verbatim** rather than adding a second block. The skeleton's shape wins where the two overlap (`../friction-log-skeleton/spec.md`, `## Behavior and interfaces`); a duplicate slot is two homes for one fact and IQ-2's deletion test cannot tell which is authoritative |
| **EC-011** | `redkiln doctor` reports a seventh `template-drift` advisory, or fewer than six | A template was changed without a decision, or a customisation was reverted. Investigate; **never run `redkiln adopt --templates`**, which would overwrite all six customisations silently (`CLAUDE.md`) |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| **NF-001** | Everything this story writes is complete and legible as raw text in a `git diff`, at ordinary terminal width, with no rendering step and no recording | `../_decomposition.md`'s `#### The accessibility floor`: a screen reader and a `git diff` must both read the record. The reduced-motion floor stated for a medium with no motion — the obligation is that the text is complete |
| **NF-002** | One spelling per id, everywhere: `HS-P0022`, never "the application-author project" or "P22"; `DT-6`, never "the wrong-model tension" | AC-002, AC-003 and AC-005 are `rg` + `test -f` checks. A second spelling makes the sweep miss the item, and a missed routed item is indistinguishable from an unrouted one |
| **NF-003** | The story adds no compile cost, no dependency, no script and no generated file. `cargo xtask affected --base main` falls through to the five file-reading lints and `spec-trace` on an empty package set | `.redkiln/config.yaml:40`'s `affected_gate` and the testing brief's Notes: a story whose whole deliverable is markdown still gets a real check rather than a vacuous pass |
| **NF-004** | Destination ids and `FL-###` ids are treated as public identifiers from the moment they are written — durability, not tidiness | Two sibling projects have already declared in their own charters that they accept dispositions from here, and `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract is what makes a stable anchor worth having |
| **NF-005** | The submission record names a person and the basis of their authority, and nothing more — no contact details, no personal information beyond what makes the authority checkable | The log is a committed artifact in a repository that will be read by people outside this project. AC-001 needs the claim to be *checkable*, which a role and an owned item supply; anything further is data the artefact did not need to collect |
| **NF-006** | The diff is small enough to review in one sitting: two `Disposition:`-line edits per routed entry at most, one index block, one submission block | The `## PR boundary` is two paths on purpose. A large diff here is the signal that an escalation is being absorbed, or that a content fix has leaked in from the slice-mate |

## Implementation notes (non-prescriptive)

Shape only — the implementer owns the wording.

- **Sweep by entry, in id order — never by owner.** Reading the log owner-first is how an item that
  belongs to nobody obvious gets skipped, and skipping is invisible. Walk `## Chronological record`
  top to bottom; the index is derived at the end, from what the entries say, not authored alongside
  them.
- **Do the ownership lookup with the table open.** For each routed item, find the BR, AC or DT the
  destination owns that the stumble breaks, and write that id into the reason. If the lookup takes
  more than a minute, EC-002 is probably in play and the honest move is to record the ambiguity as an
  entry rather than to pick a plausible owner.
- **Submit before writing the record.** The `### Submission record` block is evidence *of an act*.
  Writing it first and performing the submission afterwards inverts the thing derived requirement 8
  exists to establish, and produces a record that is true only by intention.
- **Name the channel concretely.** "Discussed" is not a channel. A commit, a PR comment, an issue, a
  message with a date — something a reviewer can go and look at. This is the field that separates
  AC-001 from an assurance.
- **Write the escalation the way its recipient asked for it.** HS-P0022 and HS-P0023 have each
  published the sentence they expect ("routes through HS-P0024, not back into this charter silently";
  "a new account is owed and is dispositioned through HS-P0024 — not absorbed here"). Mirror that
  wording rather than inventing a protocol for a recipient who already published theirs.
- **Derive the index last, and say so in its first line.** Regenerating it from the entries is what
  makes IQ-2's deletion test pass by construction. An index authored in parallel with the entries
  drifts, and the drift is invisible until someone deletes it.
- **When the escalation set is empty, write the sentence anyway.** One line in the roll-up saying no
  stumble would reopen a resolved tension. It costs nothing and it is the difference between an
  answered question and an unasked one.
- **Resist the automation the testing brief forbids by name.** A script confirming a `Disposition:`
  line exists passes `noted` as readily as `routed: HS-P0022 — owns BR-03`. Every check this story
  adds asserts on **content**: an id that `test -f` resolves, a DT id present in the table, two sets
  that do not intersect.
- **If a change wants a third path, stop and re-read AC-011.** A sibling charter edit, a new backlog
  item or a `.kb/` file is an escalation being performed. Record the id and let the owner act.

## Tests and CI (merge gate)

Grounded in [`../_decomposition.md`](../_decomposition.md)'s `## Testing brief` — its AC/tier table
(the AC-007 and AC-011 rows in particular), its definition of Artifact-evidence as ledger-cited
`file:line` rather than eyeballing, and its explicit instruction not to automate a presence check
into something nothing can fail. `<log>` is
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md` throughout.

| tier | command / path | proves |
| --- | --- | --- |
| Static — destination existence | `test -f .bklg/support/initiative.md`; `test -f .bklg/docs-that-teach/application-author-path/project.md`; `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md`; plus a `test -f` for every other id the routed sweep finds | Every destination resolves to a real item — the testing brief's own AC-007 mechanism (AC-002) |
| Static — no prose destination | `rg -n "^Disposition: routed:" <log>`, each payload matched against the id pattern | No routed item's destination is a description; rejects IQ-7's "route to the docs team" (AC-002) |
| Static — ownership lookup | `rg "^\| BR-" .bklg/docs-that-teach/_decomposition.md`; `rg "^\| AC-" …`; `rg "^\| DT-" …` — each cited id present, and its owner row naming the project the item was routed to | The correct-owner half of project AC-007 is checkable against a table, both sides (AC-003) |
| Static — no `.kb/` write | `git diff --name-only` for this PR contains no `.kb/` path | A deferral stages material; atom authorship stays closeout's (AC-004) |
| Static — DT vocabulary | `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md`; every escalated id a member of that set, `DT-9` a member of none | An escalation citing a tension that does not exist is caught mechanically, not trusted (AC-005) |
| Static — set disjointness | `FL-###` ids from `rg -n "^Disposition: escalated:" <log>` intersected with those from `rg -n "^Disposition: fixed:" <log>` | The escalation and fix sets share nothing — no settled tension half-re-decided inside a content fix (AC-006) |
| Static — submission record | `rg -n "^### Submission record" <log>` returns exactly one hit under `## Hand-off`; no `NOT-YET-RECORDED` inside it; `git show <commit>:<log>` resolves the commit it names | The submission happened, to a named person, at a checkable point in history (AC-001) |
| Static — structure and anchors | `rg -n "^## " <log>` returns the skeleton's eight headings in order; `rg -n "^### FL-" <log>` is byte-identical to its pre-PR output | No ninth section (which would break `friction-log-skeleton`'s banked AC-001), no id or label moved (AC-007, AC-008) |
| Static — plain text | `rg -n "<details>\|<summary>\|<script>" <log>` returns nothing; every checkbox line matches a single-line `- [ ] …` | Nothing behind a fold; the line-by-line gate parser can match every box (AC-008) |
| Static — the IQ-2 deletion check | Delete `## Dispositions index` in a scratch copy and re-read: every destination, ownership reason, escalation and the submission record survive | Non-occlusion, stated verbatim in `../_decomposition.md`'s IQ-2 (AC-001, AC-006, AC-008) |
| Static — mount | `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md`; `git diff` carries no frontmatter hunk | U3 reaches the log in one hop from the project card; the CLI's frontmatter is untouched (AC-008) |
| Artifact-evidence | `.bklg/docs-that-teach/comprehension-evidence/route-and-escalate/_ledger.md` — one row per AC, each citing a real `file:line` into the log | The claims a reviewer must read: that a reason is a reason, that an owner is the right owner, that an escalation states what would be re-decided. `require_ledger: true` at `.redkiln/config.yaml:67` |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | Falls through to the five file-reading lints and `spec-trace` — a markdown-only story still gets a real check rather than a vacuous pass on an empty package set |
| Story verify | `redkiln verify --grain story` | The `## PR boundary` path block holds (no file changed outside the two paths) and every ledger row is satisfied with non-placeholder evidence before `implement → report` |
| Backlog hygiene | `redkiln validate --kb && redkiln doctor` | Clean, at exactly the six standing `template-drift` advisories `CLAUDE.md` documents (`../project.md`, DoD-8) |
| Project integration bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7. **Not this story's to turn green** — it touches no crate — but it must remain green; a disposition that breaks `spec-trace` has broken something it should never have touched |

**Not run here, deliberately.** `cargo xtask ci` (the terminal `e2e` bar, `.redkiln/config.yaml:60`)
is HS-P0025's — this project is `terminal: false`. And the friction-log *session* is never folded
into a CI step: U1 and U2 are never mocked, scripted or simulated, because non-authorship is the
mechanism rather than a convenience (`../_decomposition.md`, `## Testing brief`, fixtures and seams).
A passing `cargo xtask ci --fast` says nothing about whether a routed item reached a person.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| No owner with authority to act can be named in time, and a team or a mailing list is written into the Owner field instead | Medium / High | EC-008 makes this a stated failure of the artefact, in `## Hand-off`, in those words. AC-001 types the field to a person and to a checkable authority line; the project risk table's first row is the precedent — a failure of the artefact is not a reason to redefine the bar |
| Routing becomes the sink the project risk register names: everything routed, nothing acted on | Medium / Medium | AC-003 makes each destination carry the owned id that makes it that owner's, so a plausible-sounding route is falsifiable against a table; AC-001 makes the submission an act with a checkable channel rather than a filing |
| A small content fix quietly re-decides DT-1 or DT-6 on the way past, in the slice-mate landing in the same context | Medium / High | AC-006's disjointness check runs over both sets; EC-004 gives the conversion path through `Revisions:`. The project charter's Medium/High row is the source: *"re-deciding one here would put half a decision in each"* |
| An escalation is written with a DT id that reads plausibly but is not in the table, or is DT-9 | Medium / Medium | AC-005's membership check is mechanical and DT-9 is excluded by construction (EC-005). This is the one half of AC-011 the testing brief insists must not be trusted |
| The submission record is written into `## Dispositions index` because that is where the routing lives, and IQ-2's deletion test then erases the evidence | Medium / High | Decision 6 of the context pack, AC-001's own deletion clause, and EC-010 for the case where the skeleton already landed a slot. The index is derived; the submission is evidence, and evidence does not live in a derived view |
| Routing "requires" editing a sibling charter or opening a backlog item, and the story quietly widens | Medium / High | The `## PR boundary` fenced block is two paths and `redkiln verify --grain story` fails on a third; EC-007 names the third path as the signal that an escalation is being performed rather than recorded |
| `disposition-every-stumble` has not landed and this story routes an unstable set | Low / High | EC-001 is a stop condition. Both stories are in the `dispositions-and-routing` slice and implemented sequentially in one context; the hard edge is stated in the context pack's ordering constraint |
| A well-meant automation is added that checks a `Disposition:` line exists | Low / Medium | Forbidden by name in the `## PR boundary` and in the implementation notes, with the testing brief's reason: it passes `noted` as readily as a real routed id, so it rejects no wrong implementation |
| The escalation set is empty and the log simply says nothing, which a later reader reads as "none were owed" | Medium / Medium | AC-006 makes zero an explicit assertion in the roll-up, the same discipline UX-AC-009 applies to zero facilitator interventions. "Not mentioned" is not one of this surface's states |

## Dependencies

**Blocks on**

- **`disposition-every-stumble`** (HS-S0166) — the only edge, and it is hard. It gives every
  severity-marked entry exactly one arm (project AC-006); this story makes the two outward-pointing
  arms land. Routing before the set is stable produces a routed item nobody submitted and a
  double-dispositioned item sent to two owners (EC-001). Both are in the `dispositions-and-routing`
  slice and are implemented sequentially in one context (`../_storymap.md`, `## Merge order`, step 3).

Transitively, through that edge: `session-run-against-pinned-tree` (the entries exist at all),
`non-insider-recruitment`, `friction-log-skeleton` (the eight sections, the `FL-###` scheme, the five
arms and the `Revisions:` slot this story writes into) and `dt9-and-fixed-protocol`.

**Slice-mate, not a dependency**

- **`content-fixes-from-dispositions`** (HS-S0168) — independent of this story and may land on either
  side of it (`../_storymap.md`, `## Merge order`, step 3). The relationship is a *constraint*, not an
  ordering: this story writes down the escalation set, and that set is what the fix story's inbox is
  checked against (AC-006). If it lands first, the disjointness check runs against what it took;
  if it lands second, the set is already on disk waiting for it.

**Unlocks**

- **`handoff-note-to-closeout`** (HS-S0171) — the story item's `blocks` field names it. It cannot
  write the hand-off note HS-P0025 lifts until the log has been submitted and every destination is
  real, because the note is the summary of a dispatch rather than of a record.

Outside this project: **HS-P0022**, **HS-P0023** and the **`support` initiative** receive what this
story routes, and **HS-P0025** reads the submitted log. None of them is edited here — routing names
their ids; it does not write into their files.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Open each at the moment its row names; do not preload the corpus.
Every path below was confirmed present with `test -f` at spec time. The mount point itself,
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, is deliberately **not** an anchor: it
does not exist yet and is created by `friction-log-skeleton`, whose spec below is the authority on its
shape until it does.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/_decomposition.md` | The three ownership tables are the lookup that makes a destination *correct* rather than plausible: BR 173–192, AC 200–215, and `## Design tension ownership` 150–159, which is the entire legal vocabulary of the `escalated:` arm and the row that identifies each tension's owning project | **Before the first routed entry**, with the file open beside the log — AC-003's reason and AC-005's DT id are both copied out of it, not recalled | AC-003, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` | Fixes the log's path, its eight `##` sections and their order, the `FL-###` id scheme, the six one-line entry fields, the five disposition arms and the `Revisions:` slot. Its AC-001 (`rg -n "^## "` returns exactly eight) is a criterion already banked that a ninth section would break; its `## Behavior and interfaces` is what EC-010 defers to | **Before writing anything into the log** — this story writes inside a shape it did not choose, and the shape wins where the two disagree | AC-001, AC-007, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief holds IQ-1…IQ-7 verbatim, the states enumeration (where **escalated** is a distinct state from routed), UX-AC-011, the accessibility floor and the named primitive layer; the Testing brief holds the AC-007 and AC-011 tier rows and the do-not-over-automate rule this story's checks are bounded by | Before the interaction-quality work and before writing any check — the invariant wording is quoted, not paraphrased | AC-006, AC-007, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | AC-007 and AC-011 are the traced criteria and their exact wording bounds this story; derived requirement 8 is AC-001's source; the risk table's routing-as-a-sink and reopened-tension rows are what AC-003 and AC-006 answer; `## Companions` (362–372) is the mount U3 arrives through | Before AC-001 (the submission's justification), before AC-003 (the sink risk), and again at the reach check for AC-008 | AC-001, AC-003, AC-006, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | Lines 94–152 verify all three destination classes against real files and quote the reciprocal declarations from HS-P0022 and HS-P0023 — the evidence that this routing edge is real and bidirectional rather than asserted from one side. Lines 143–152 are the deferral mechanism | Before AC-002 and AC-004 — it is the distilled form, and usually removes the need to open the sibling charters at all | AC-002, AC-004, AC-005 |
| `.bklg/docs-that-teach/application-author-path/project.md` | Lines 140–143 are HS-P0022's own published sentence for how it accepts a disposition from here: *"routes through HS-P0024, not back into this charter silently"* — the shape an escalation to this owner should mirror | When writing an escalation or route whose destination is HS-P0022 — copy the recipient's own wording rather than inventing one | AC-005 |
| `.bklg/docs-that-teach/reach-and-adapter-path/project.md` | Line 294 is HS-P0023's published acceptance sentence, and line 292 concedes that its DoD-9 second questions are author-chosen and that *"HS-P0024 is the instrument that can falsify the choice"* — which is the AC-05 fallback finding this story would route | When writing an escalation or route whose destination is HS-P0023, and specifically if the session produced the adapter-author verdict | AC-005 |
| `.bklg/support/initiative.md` | `HS-I0005`, `stage: intake`, zero projects — real and structurally empty. AC-004 requires the log to say that rather than imply a populated backlog | While writing any `support`-routed item | AC-002, AC-004 |
| `.redkiln/config.yaml` | Line 5 `support_initiative: support` is the authority for the library-bug destination; line 40 `affected_gate`, line 55 `integration_scoped`, line 67 `require_ledger` are the gate commands the `## Tests and CI` table names | Before writing the `support` route, and again when running the gate | AC-002, AC-008 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding on this work. It is the authority for a re-route appending rather than overwriting, and it is what makes an escalation safe to withdraw without the trail vanishing | Before appending the first `Revisions:` line — including the first time a destination is corrected | AC-007 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | `## Coverage` names this story sole owner of AC-007 and AC-011 and states why escalation is owned with routing; `## Merge order` step 3 fixes the ordering EC-001 depends on and states that the slice-mate may land either side | Before starting, to confirm the boundary; again if the work starts to feel like it should include a fix or a disposition | AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/disposition-every-stumble/spec.md` | The upstream story in this slice: its `## PR boundary` explicitly hands *submission, destination-owner correctness and DT-id correctness* to this story, and its behaviour table fixes the arm payload shapes this story fills | At the slice hand-over, to confirm what it left and to check no entry still reads `not yet dispositioned` (EC-001) | AC-002, AC-005 |
| `.bklg/docs-that-teach/comprehension-evidence/content-fixes-from-dispositions/spec.md` | The slice-mate whose inbox AC-006's disjointness check constrains. Reading it is how the fix set is known without guessing at it | When performing the disjointness check, whichever side of this story it lands on | AC-006 |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | Signed off 2026-08-17 with no conditions, `hasSurface: false`, empty items block, every shape section `N/A` — the reason the composition invariants above are stated against document primitives and counts rather than tokens and pixels, and the bound that inventing a CSS or token layer is out of scope | Before the composition work, and any time a shape decision feels like it needs a new format | AC-008 |
| `docs/README.md` | The live two-column routing table — "what you are looking for" against "where it is" — that `## Dispositions index` reuses rather than replaces. The primitive, in the repository, already rendering | Before regenerating `## Dispositions index` | AC-006, AC-008 |
| `.redkiln/templates/gates/discover.md` | Lines 9–13 are the one-line checkbox primitive; `CLAUDE.md` records that the gate parser matches line by line and a wrapped box can never match | Only if this story writes a checkbox | AC-008 |
| `.redkiln/templates/_ledger.md` | The `evidence: "file:line"` contract — the reason a stable `FL-###` anchor is worth having at all (NF-004), and the shape this story's own ledger takes | When authoring `_ledger.md`, and when justifying NF-004 | AC-007 |
| `.bklg/docs-that-teach/initiative.md` | BR-06 (routing is the non-optional final step), AC-10 (*"Someone acted on what that reader found"*), DoD scenario 6, and the assumption at 645–647 that the publication project's design gate stays closed — *"if it turns out that it does, that is escalated, not absorbed"*, which is AC-005's source | Only if a routing or escalation judgement needs its origin rather than its restatement in `../project.md` | AC-001, AC-005 |
| `.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` | The research angle behind the whole story: routing as the non-optional final step, and a log with no destination as opinion by another name | Only if the submission requirement is questioned as ceremony — `../_grounding.md` and `../project.md`'s derived requirement 8 are the distilled forms and are usually enough | AC-001 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `## Anti-patterns` names the bespoke navigation widget and the load-bearing-item-behind-a-fold failures the composition invariants forbid | Before adding any navigational affordance to the index — the answer is almost always a link, not a component | AC-008 |

## Clarifications resolved during spec

1. **The AC ids are exactly the eight the front half enumerated** — AC-001 … AC-008 — with none added
   and none dropped. The mapping is: AC-001 the submission record, AC-002 destinations that resolve,
   AC-003 the ownership reason, AC-004 the two uncontested destination classes described truthfully,
   AC-005 the escalation's DT id and its owner, AC-006 disjointness and the zero-is-an-assertion rule,
   AC-007 reversibility and anchor stability, AC-008 in-place reach plus the derived index and the
   static-text composition floor.

2. **The submission record is a `### Submission record` sub-block inside `## Hand-off`, not a ninth
   `##` section.** The front half fixed that it must live in a persistent section; the heading level
   and the literal label are fixed here so AC-001's check is an `rg` match rather than a judgement
   call. `friction-log-skeleton`'s AC-001 — `rg -n "^## "` returning exactly eight headings in order —
   is a criterion already banked, and a `###` under an existing `##` does not touch it. If the
   skeleton already landed a submission slot, EC-010 applies and its shape wins verbatim.

3. **The submission block has five one-line labelled fields: `Owner:`, `Authority:`, `Date:`,
   `Submitted:`, `Channel:`.** They reuse the one-line labelled-field primitive the skeleton fixed for
   the stumble entry rather than inventing a second field style, which is what the composition
   invariant "each thing maps to a named existing primitive" requires in a repository with no token
   file.

4. **The ownership reason travels on the same line as the id, as an em-dash clause.** For example
   `Disposition: routed: HS-P0021 — owns BR-04, the answered-need rule this page breaks`, and
   `Disposition: escalated: DT-6 — HS-P0022 owns it; would re-decide whether the wrong-model contrast
   is shown as compiled code`. One line, because the entry's fields are one-line fields and because a
   reason on a second line is a reason a sweep will miss.

5. **The escalation set is derived and deletable, and that is not a contradiction of "written down as
   a set".** Each escalation is authoritative at its own entry; the roll-up in `## Dispositions index`
   is the derived view that makes the disjointness check cheap. Deleting the index loses the
   convenience and no information — which is IQ-2 satisfied by construction, and is why AC-006's check
   is stated over the entries (`rg` on `Disposition:` lines) rather than over the roll-up.

6. **DT-9 is excluded from the escalation vocabulary by construction, leaving nine legal ids.** The
   ownership table assigns DT-9 to HS-P0024 itself, and this project *resolves* it in `../_design.md`.
   An "escalation" naming DT-9 is a category error rather than a routing mistake, which is why EC-005
   re-classifies it instead of correcting the id.

7. **Zero escalations is written as a sentence, not inferred from an absent section.** The states
   enumeration in `../_decomposition.md` has no "not mentioned" state, and UX-AC-009 already applies
   the same rule to facilitator interventions. AC-006 carries it so the check is a `rg` for the
   sentence when the set is empty, rather than an absence a reviewer has to interpret.

8. **No verifying test below is a compiled test, and that is honest rather than a shortfall.** This
   story adds no `pub` item and touches no crate; every check is a real, runnable Static-tier command
   or a ledger-cited Artifact-evidence read against a real path — the two tiers `../_decomposition.md`'s
   `## Testing brief` defines for this project. Each was chosen because it rejects a named wrong
   implementation, and the brief's forbidden check ("a `Disposition:` line exists") appears nowhere
   above.
