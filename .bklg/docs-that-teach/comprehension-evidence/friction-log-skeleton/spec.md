---
item: HS-S0163
stage: spec
created: 2026-08-17T13:16:18.427Z
updated: 2026-08-17T13:16:18.427Z
template_sig: 87bbf1d0
rendered_sig: 039805ec
---

# Spec — The auditable-shape friction-log scaffold

## Scope lock

| What | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/docs-that-teach/initiative.md`](../../initiative.md) — BR-05, BR-06, BR-14; DoD scenarios 5 and 6 |
| Initiative decomposition | [`.bklg/docs-that-teach/_decomposition.md`](../../_decomposition.md) — `### Definition of Done` (row 5 is this story's), `## Design tension ownership` (DT-9), `### Merge order` |
| Project | [`.bklg/docs-that-teach/comprehension-evidence/project.md`](../project.md) — AC-004 is this story's traced criterion; DoD-7 is its integration bar |
| This spec | `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — `## UX brief` (the states list, the primitive layer, the accessibility floor, IQ-1…IQ-7, UX-AC-003/004/005/006/007/008/009) and `## Testing brief` (the AC/tier table; the "do not automate into a check that nothing can fail" note) |
| Grounding | [`../_grounding.md`](../_grounding.md) — `## The friction-log method: cited research, not invention` (lines 55–92), the source of the auditable shape |
| Signed-off design | [`../_design.md`](../_design.md) — `hasSurface: false`, signed off 2026-08-17 with no conditions. It declares **no** public API item and **no** rendered UI surface, and it is where `dt9-and-fixed-protocol` writes the protocol this scaffold inherits |
| Roadmap pointer | [`../_storymap.md`](../_storymap.md) — `## Merge order`, step 1 (`session-protocol`): `dt9-and-fixed-protocol` → **this story** → `non-insider-recruitment` |

## One-line PR slice

Land the auditable-shape log scaffold in-tree — scenario; logger identity, context and date; an
append-only chronological section with stable occurrence-ordered stumble ids; a text-token severity
mark on the named scale; a disposition slot at each stumble with a revision block; and the
scope-sentence and hand-off sections — so the session fills a shape it did not invent while running.

## Executive summary

This PR lands **one new file** — the friction log, empty of findings and complete in shape — plus the
two one-line wirings that make it reachable: a `## Companions` row in the project card and a path/
heading-vocabulary line in `_design.md`'s protocol section.

The delta against the project charter is only this: the charter says a dated log *exists* in the
auditable shape (`project.md`, AC-004). This story owns the **shape half** of that criterion and
nothing of the content half — `_storymap.md`'s `## Coverage` splits AC-004 explicitly ("the skeleton
owns the **shape**… the session owns the **content**"). Concretely, this PR decides the file's path,
its section vocabulary, the stumble-entry shape, the id scheme, the severity field's type, the
disposition slot and its revision block, and the emptiness marking that stops the scaffold from being
mis-cited as evidence. It decides **no** protocol content: the scenario text, the narration mode, the
severity scale and the disqualifying criteria all arrive from `dt9-and-fixed-protocol`, and this
scaffold transcribes them rather than restating them.

It is the project's only `foundation` story, and its consumer is one slice later
(`session-run-against-pinned-tree`), which is what keeps it from being an unproven island
(`_storymap.md`, `### Why the slices fall here`).

## Context pack

**Read this section and you can start. Everything below `## Anchors` is deferred depth, not optional
depth — open an anchor when its bound AC says to.**

### The decision this story exists to make early

**The shape is fixed before the session, because a shape improvised at minute 40 does not have the
properties the evidence depends on.** This is the whole reason the scaffold is its own story rather
than a heading the facilitator types as they go. The UX brief states the failure directly as what must
never happen to U2, the facilitator: *"Discovering after the session that the shape they needed
(scenario, context, severity scale) was never fixed, and reconstructing it from memory. Research 04
calls retrospective reconstruction a measurably different instrument, not a late version of the same
one"* (`../_decomposition.md`, `## UX brief`, the three-user table). IQ-2, IQ-3 and IQ-4 are properties
of the *shape*, not of the diligence of whoever is typing.

### The persona-journey slice this realizes

Two readers, and the scaffold serves them at different times:

- **U2, the facilitator, at minute 40 of a live session.** They are watching a stranger and typing at
  the same time. Every field they have to invent is attention taken off the reader, and every field
  they invent is a field that was not fixed before recruitment. The scaffold's job is that there is
  nothing left to decide except *what just happened*.
- **U3, the downstream actor, six months later** — a sibling-project owner, the `support` initiative,
  or HS-P0025. They open the log, find the items that are theirs, and act *without reading the whole
  thing and without asking the logger what an entry meant*. The scaffold's job for them is that a
  destination is an id and a disposition is at the item.

U1, the recruited reader, never sees this file. Nothing in the scaffold is shown to them; a form the
reader fills in is a different instrument.

### The decisions this story must honor, stated as decisions

1. **Inherit the protocol; invent none of it.** The scenario, the narration mode, the severity scale
   and the logger-disqualifying criteria are `dt9-and-fixed-protocol`'s output in `_design.md`. This
   scaffold **transcribes** them into the log verbatim and names `_design.md` as their source. A scale
   that appears in the log's severity legend but not in `_design.md` is a scale that got retrofitted —
   exactly the difference research 04 draws between a rankable record and a diary, and exactly what
   AC-002's provenance check (`git log --format=%aI` on `_design.md` predating the session date) exists
   to catch (`../_decomposition.md`, `## Testing brief`, AC-002 row).

2. **Severity is a text token, never colour or emoji alone.** Research 04's source convention is a
   red/yellow/green stoplight; a markdown log read on a diff view has no colour at all. The
   accessibility floor is explicit: *"Every severity mark is therefore a **text token** — the word, or a
   number on the named scale… and never an emoji or colour swatch carrying the meaning alone. A screen
   reader and a `git diff` must both read the severity"* (`../_decomposition.md`, `## UX brief`,
   `#### The accessibility floor`). The whole file is complete as static text: no recording, tool or
   rendering step is required to tick AC-004's six elements (UX-AC-004).

3. **Ids are occurrence-ordered and immutable once written; the chronological section is append-only.**
   IQ-3's reason is not tidiness — *"Renumbering after the fact, reordering by severity in place of
   chronology, or re-heading a section breaks every citation a sibling project has already made into
   the log. A later reader landing on a cited anchor must find the item that was cited."* Two sibling
   projects have already declared they will accept dispositions from here by reference
   (`../_decomposition.md`, `## UX brief`, `### Notes`, *Escalation has a shape already on record*), so
   an id is a public identifier from the moment it is written. **The heading line of an entry is
   therefore frozen at write time, label included** — a re-worded label changes the anchor and the
   citation dies silently.

4. **A disposition is readable at the stumble, and a change to it appends beside the original.** IQ-1
   allows an index but forbids requiring the hop: *"a reviewer must never have to leave the entry to
   learn whether it was resolved."* IQ-4 forbids the in-place overwrite, and names its authority —
   [`.kb/governance/rewrite-the-referent-never-the-reasoning.md`](../../../../.kb/governance/rewrite-the-referent-never-the-reasoning.md)
   *"applied to dispositions: the referent may be rewritten, the reasoning may not be erased."* So the
   entry carries **both** a disposition slot and a revisions slot, from the first moment it exists.
   Retrofitting a revisions slot when the first disposition changes is how the original gets
   overwritten.

5. **Any roll-up is additive and derived; the record survives its deletion.** IQ-2 gives the check
   verbatim: *"if the collapsed or filtered view were deleted, would the record still carry every
   stumble and its disposition?"* This constrains the scaffold, not just the filled log — a severity
   roll-up table that is authored as the primary home of a severity value has already failed, whatever
   the session does later.

6. **Destinations are ids, not descriptions (IQ-7).** The disposition slot's `routed` arm is typed to
   an id — `HS-P0022`, `HS-P0023`, the `support` initiative per `.redkiln/config.yaml:5`, or a named
   deferral staged for ingest — and the `escalated` arm carries a **DT id**, because escalation is a
   distinct state from routing (`project.md`, AC-011). *"Route to the docs team" is not a destination.*

7. **Compose from the repository's existing document primitives; hand-roll no format.** There is no CSS
   layer and no token file here, and inventing one is out of scope. The named primitives are
   [`.redkiln/templates/briefs/brief.md`](../../../../.redkiln/templates/briefs/brief.md)'s
   Intent/Acceptance-Criteria/Notes spine,
   [`.redkiln/templates/_design.md`](../../../../.redkiln/templates/_design.md)'s `## Shape decision`
   table and `## Sign-off` section, the `| Risk | Likelihood / Impact | Mitigation |` risk-table shape,
   [`docs/README.md`](../../../../docs/README.md)'s two-column routing table (the shape a disposition
   index reuses rather than replaces), and the one-line checkbox from
   [`.redkiln/templates/gates/`](../../../../.redkiln/templates/gates/).

8. **Any checkbox this file carries stays on one line.** The gate parser matches line by line and a
   wrapped box can never match — `CLAUDE.md` calls this out as the reason one of the six standing
   template customisations exists, and the UX brief calls it *"the closest thing in this repository to
   a hard token constraint, and it is load-bearing."*

9. **The scaffold must be unmistakable for evidence.** This project's central risk is the claim being
   overread downstream (`project.md`, risk table, row 2; BR-14). A template carrying illustrative
   stumbles is a fabricated observation waiting to be cited by someone who skimmed. So: a status banner
   saying no session has been run, and every unfilled slot carrying one explicit, greppable
   not-yet-recorded token — never an invented example, never a plausible-looking placeholder severity.

10. **This project edits nothing under `.kb/`, and this story authors no atom.** Hand-authoring atoms
    outside the ingest path is a named non-goal and the first attempt was reverted (`0269720`)
    (`project.md`, `## Out of scope`). The deferral arm of the disposition slot therefore *stages* a
    question with the id of that hand-off recorded; it does not write a file under `.kb/open-questions/`
    (`../_decomposition.md`, `## UX brief`, `### Notes`, *A deferral is staged material*).

### What this story is explicitly not deciding

Which persona the session walks, what the scenario says, which severity scale wins, whether narration
is concurrent or retrospective, and what disqualifies a logger. All five are DT-9's and
`dt9-and-fixed-protocol`'s, resolved in `_design.md` under its human sign-off gate — the only record
that decision will ever have, because `design.capture` is deliberately absent from
`.redkiln/config.yaml` and the perceptual review is a skip (`../_design.md`, `## Sign-off`).

### The one ordering constraint that can invalidate this work

`dt9-and-fixed-protocol` **must be committed first**. Both stories are in the `session-protocol` slice
and are implemented in one context, sequentially (`../_storymap.md`, `## Merge order`, step 1). If the
scaffold's severity legend is written before the scale is named, it will be written from this spec's
guesses and will then have to be edited — and an edit to the legend after the fact is the retrofit
decision 1 forbids. If `_design.md`'s protocol section already fixes the log's path or heading
vocabulary, **that wins verbatim** and the choices in `## Behavior and interfaces` below are discarded
in its favour; the storymap is explicit that those belong to `_design.md`, not to the map or to this
spec (`../_storymap.md`, `### Why the slices fall here`, closing sentence of the
`friction-log-skeleton` paragraph). Where `_design.md` is silent, this spec fixes them and the
implementer records the choice back into `_design.md` so the two can never disagree.

## Integration contract

| Field | Value |
| --- | --- |
| **Archetype** | `foundation` — real in-tree substrate, consumed inside this initiative one slice later. Never a double, never a `fixme` (`../_storymap.md`, slice table) |
| **Slice / milestone** | `session-protocol` |
| **Slice-mates** | `dt9-and-fixed-protocol` (lands **before** this story), `non-insider-recruitment` (lands **after**, and writes into this scaffold's declaration section) |
| **Mount point** | `.bklg/docs-that-teach/comprehension-evidence/project.md` → its `## Companions` list (currently lines 362–372). That list is the project card's drill-down index — the only render path by which a human reaching HS-P0024 from `redkiln board` / `redkiln status` finds a companion artifact. A log file that exists but is absent from it is constructed-but-unmounted |
| **Second mount** | `.bklg/docs-that-teach/comprehension-evidence/_design.md` → the protocol section `dt9-and-fixed-protocol` adds. The protocol names the file the protocol is executed into; without this line the scaffold and the protocol are two documents that happen to agree |
| **Wires into** | `_design.md`'s protocol section (scenario, narration mode, severity scale, disqualifying criteria — transcribed, not restated); `.redkiln/config.yaml:5` `support_initiative: support` (the `routed` arm's destination vocabulary); `../_decomposition.md`'s `## Design tension ownership` table (the `escalated` arm's DT-id vocabulary); `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract, which is what makes a stable stumble id worth having |
| **Design-system primitives consumed** | The five document primitives named in decision 7 above, plus the one-line checkbox of decision 8. No new format, no new heading vocabulary beyond what this spec fixes, no navigation widget (`../_decomposition.md`, `## UX brief`, `#### The primitive layer to compose from — do not hand-roll`) |
| **Renders surfaces** | **None.** `../_design.md`'s `## Items` block is empty (`# no items — no public API surface, no rendered UI surface`) and every shape section reads `N/A — no user-facing surface`. This story renders no `_design.md` item id. It builds surface **2** of the three *document* surfaces the UX brief enumerates ("The friction log — a dated markdown artifact whose reader is a reviewer, a sibling-project owner, and eventually HS-P0025"), which `_design.md` names in prose rather than as an item |
| **Conformance rule(s) / clause(s)** | None, and deliberately: this story adds no `pub` item, compiles nothing, touches no crate under `crates/` and discharges no `SPECIFICATION.md` clause. It is not adapter-observable. The `../_decomposition.md` `## Testing brief` names its actual instruments instead — `rg`-checkable structure plus ledger-cited artifact evidence — and `cargo xtask affected --base main` falls through to the five file-reading lints and `spec-trace`, so the story still gets a real check rather than a vacuous pass on an empty package set |
| **Advances DoD scenario** | **Initiative DoD scenario 5** — *"A non-author, non-insider completes a stated scenario; auditable log"* (`../../_decomposition.md`, `### Definition of Done`, row 5; owner HS-P0024). This story makes the *auditable* half possible; `session-run-against-pinned-tree` turns it green. It also pre-shapes scenario 6 (every stumble dispositioned) by landing the disposition slot, but does not advance it |

**Delivered mounted, not as an isolated component.** The acceptance bar for this PR includes that a
person who has never seen this story can reach the scaffold from the project card in one hop, and that
`_design.md`'s protocol points at it by path.

## PR boundary

**In this PR**

- The friction-log scaffold as a new file, complete in shape and empty of findings.
- The `## Companions` row in `project.md` that mounts it (body prose only — never the frontmatter).
- The line in `_design.md`'s protocol section naming the log's path and heading vocabulary, added only
  where `dt9-and-fixed-protocol` has not already fixed them.
- This story's own `_ledger.md` and its stage artifacts.

**Explicitly not in this PR**

- Any protocol *content* — persona, scenario, narration mode, severity scale, disqualifying criteria.
  Those are `dt9-and-fixed-protocol`'s, and this PR transcribes whatever it wrote.
- Any session content: no entries, no severities, no dispositions, no reader declaration, no tree
  commit, no date beyond the scaffold's own creation. Filling the shape is
  `session-run-against-pinned-tree`'s and `non-insider-recruitment`'s.
- Any edit under `crates/`, `docs/`, `examples/`, `spec/` or `standards/`. Content fixes are
  `content-fixes-from-dispositions`'s, and they cannot exist before a disposition exists.
- Any file under `.kb/`. Atom authorship is closeout's, through the ingest path (`project.md`,
  DoD-6 of the project's own list).
- Any automation that "checks" a disposition exists. The testing brief forbids it by name: a check that
  passes `"noted"` as readily as `"routed to HS-P0022, see FL-004"` rejects no wrong implementation.

**Merge DoD one-liner** — the scaffold is on disk with all eight sections and zero findings, reachable
in one hop from `project.md`'s `## Companions`, named by path in `_design.md`, with
`cargo xtask affected --base main` green and `redkiln validate --kb && redkiln doctor` clean at exactly
the six standing `template-drift` advisories `CLAUDE.md` documents.

**Paths this story may touch** — `redkiln verify --grain story` reads the first fenced block under this
heading and fails on any file changed outside it.

```
.bklg/docs-that-teach/comprehension-evidence/_friction-log.md
.bklg/docs-that-teach/comprehension-evidence/project.md
.bklg/docs-that-teach/comprehension-evidence/_design.md
.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/**
```

Narrow on purpose. No crate, no `docs/`, no `.kb/`. If a change here appears to need a fourth path,
that is a signal the story has absorbed a consumer's work — split it rather than widening the block.

## Behavior and interfaces

The scaffold's "interface" is its section vocabulary and its entry shape: the contract between the
facilitator who fills it and the three later stories that read it.

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **The file and its path** | `.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`. Underscore-prefixed, matching this project's own companion convention (`_decomposition.md`, `_grounding.md`, `_storymap.md`, `_design.md` are all companions, not item files) — so `redkiln` treats it as a companion and never as an item whose frontmatter the CLI owns. **Superseded verbatim if `_design.md` fixes a path** | `../_storymap.md`, `### Why the slices fall here` (path is `_design.md`'s to fix); `../project.md`, `## Companions` (the convention) |
| **Eight sections, in this order** | `## Status` · `## Scenario` · `## Session record` · `## Severity scale` · `## Chronological record` · `## Dispositions index` · `## Scope of the claim` · `## Hand-off` (the last carrying the second-session verdict). Sections 2–5 carry AC-004's six elements; 6 is additive; 7 and 8 are the slots `scope-the-claim`, `second-session-decision` and `handoff-note-to-closeout` fill | `../project.md`, AC-004, AC-008, AC-009, AC-010; `../_grounding.md:61-66` (the source template's shape) |
| **`## Status` — the anti-miscitation banner** | One line stating that no session has been run and the file contains no findings, plus the single not-yet-recorded token used throughout, named once so it is greppable. Replaced by the session date and tree when `session-run-against-pinned-tree` fills it | `../project.md`, `## Risks and coupling notes`, row 2 (overclaiming downstream); `.kb/product/README.md` (an unevidenced sketch is not evidence) |
| **`## Scenario` — transcribed, not authored** | One to two sentences, copied verbatim from `_design.md`'s protocol, with the source named inline. This story writes the slot and the transcription rule; it does not write a scenario | `../_decomposition.md`, UX-AC-002; `../project.md`, AC-002 |
| **`## Session record` — the six context fields** | Logger identity; the reader's own non-insider declaration against `_design.md`'s criteria, able to express **eligible / ineligible / ineligible-but-used-anyway** (the third is a real state and its occurrence means the artefact is unmet, not that the bar moved); platform, toolchain, browser and any assistive technology; session date; narration mode transcribed from `_design.md`; the tree walked, as a commit or merge point | `../_decomposition.md`, `## UX brief`, `#### The states this surface has to express`; `../project.md`, AC-003, AC-004, AC-005 |
| **`## Severity scale` — legend, transcribed** | The scale named in `_design.md`, reproduced as a token→meaning table so the log is self-contained for a reader who has not opened `_design.md`, with `_design.md` named as the source of record. The scaffold names **no** scale of its own | Decision 1 above; `../_decomposition.md`, UX-AC-003; `../_grounding.md:77-80` |
| **`## Chronological record` — append-only, id-stable** | Entries as `### FL-001`, `### FL-002`, … assigned in occurrence order, never reassigned, never reordered. The heading line — id **and** label — is frozen once written, because the anchor is a public identifier from that moment. New entries are appended; nothing already written is edited except by the revisions slot below | `../_decomposition.md`, IQ-3 / UX-AC-007 |
| **The stumble-entry shape** | Each entry carries, as one-line labelled fields: **Time** (offset from session start) · **Kind** (`observation` \| `reaction` \| `intervention` \| `abandonment`) · **Severity** (a text token from the legend, or the not-yet-recorded token; `n/a` for non-stumble kinds) · **What happened** (what the reader *did* — searches typed, links followed, files opened, commands run) · **Disposition** · **Revisions** | `../_decomposition.md`, `## UX brief` (U2's row: what must be captured); `../project.md`, derived requirement 3 |
| **`Kind: intervention` and `Kind: abandonment` are first-class** | An intervention is an entry with its timestamp, so a log with zero of them *asserts* that none occurred rather than leaving it unknown (UX-AC-009, IQ-5). Abandonment is a valid end state with its own kind, so a session that stops at the first blocker produces a finding rather than a void (IQ-6) | `../_decomposition.md`, IQ-5, IQ-6, UX-AC-009 |
| **The disposition slot — exactly one arm** | Arms: `not yet dispositioned` (legal during the session, illegal at hand-off) \| `fixed: <ref>` \| `accepted: <reason>` \| `routed: <id>` \| `escalated: DT-<n>`. The `routed` arm takes an **id** and the `escalated` arm takes a **DT id** — a prose destination fails AC-006 as surely as an undispositioned item. Zero arms and two arms are both failures | `../project.md`, AC-006, AC-007, AC-011; `../_decomposition.md`, IQ-7 / UX-AC-011 |
| **The revisions slot exists from the start** | Present on every entry as it is first written, defaulting to `none`. A changed disposition **appends** a dated line with the earlier disposition and the reason for the change; the original text is never edited. Retrofitting this slot at first revision is how the original gets overwritten | `.kb/governance/rewrite-the-referent-never-the-reasoning.md`; `../_decomposition.md`, IQ-4 / UX-AC-008 |
| **`## Dispositions index` — additive and derived** | A two-column routing table in `docs/README.md`'s existing shape ("what you are looking for" → "where it is"), stating in its own first line that it is derived from the chronological record and that the record is authoritative. Deleting the whole section must lose nothing | `docs/README.md` (the primitive); `../_decomposition.md`, IQ-2 / UX-AC-006 |
| **`## Scope of the claim` — pre-written, not post-hoc** | The scope sentence is authored **now**, before any finding exists: real stumbles were captured and are traceable; the evidence asserts nothing about exhaustiveness. Writing it before the session is what stops it being tuned to the findings | `../project.md`, AC-008; `../_grounding.md:72-76` (Nielsen & Landauer, the small-sample basis) |
| **`## Hand-off` — the slots HS-P0025 reads** | Named slots for one directly observed persona, the date, the tree walked, the scope sentence, and the second-session verdict (`run` \| `declined — <reason>`; "not mentioned" is not a state). All not-yet-recorded in this PR | `../project.md`, AC-009, AC-010; `../_decomposition.md`, UX-AC-013 |
| **Complete as static text** | No recording, script, rendering tool or widget is required to read any of the above; navigation is browser find and stable heading anchors only. Nothing carries meaning by colour or emoji alone | `../_decomposition.md`, `#### The accessibility floor`; UX-AC-004 |
| **One-line checkboxes** | Any checkbox the scaffold carries stays on a single line — the gate parser matches line by line and a wrapped box can never match | `CLAUDE.md`, `## Where the work lives`; `.redkiln/templates/gates/` |
| **Mounted at `project.md`'s `## Companions`** | One row, in the existing list's style, describing the file as the project's proof artifact and stating that it is a scaffold until the session runs. Body prose only — the CLI owns the frontmatter and a `PreToolUse` hook denies the edit | `../project.md:362-372`; `CLAUDE.md`, `## Where the work lives` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story adds one markdown file and two prose lines to
two existing markdown files. It touches no crate under `crates/`, defines no type, opens no connection
and writes no row; `../_design.md` records `hasSurface: false` and an empty items block for exactly
this reason.

The nearest thing to a data contract here is the **stumble id**, and it is worth naming as one because
it behaves like a primary key that other documents will foreign-key into: `FL-###`, assigned in
occurrence order, never reused and never reassigned, with its heading line frozen at write time.
Sibling projects and `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` rows will cite those
anchors, so the only "migration" this file can ever undergo is an append. Renumbering is not a
migration; it is a broken citation with no error message
(`../_decomposition.md`, `## UX brief`, IQ-3).

## Acceptance criteria

Nine criteria. Each is framed from a persona's intent crossing the whole artifact — U2 the
facilitator mid-session, U3 the downstream actor six months later — because a criterion framed as a
bare capability ("the log has a severity column") is satisfied by a column of empty cells. The
personas are `../_decomposition.md`'s `## UX brief` three-user table; the journey they cut through is
`../_storymap.md`'s backbone A1 → A5.

Throughout, **the scaffold** means
`.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`, and **the token** means the single
greppable emptiness marker `NOT-YET-RECORDED` fixed under `## Clarifications resolved during spec`.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** U2 opens the repository at session start with a stranger already in the room, **WHEN** they look for where the session gets recorded, **THEN** the scaffold exists carrying all eight sections in the fixed order (`## Status` · `## Scenario` · `## Session record` · `## Severity scale` · `## Chronological record` · `## Dispositions index` · `## Scope of the claim` · `## Hand-off`), zero findings, a `## Status` banner stating that no session has been run and naming the token once, and every unfilled slot carrying that token — so nothing is left for U2 to invent, and no later reader can mistake the scaffold for evidence | Static: `rg -n "^## " <scaffold>` returns exactly those eight headings in that order; `rg -c "NOT-YET-RECORDED" <scaffold>` is non-zero; `rg -n "^### FL-" <scaffold>` returns nothing. Rejects the two wrong implementations named in `../project.md`'s risk table row 2 and decision 9: a scaffold shipped with illustrative stumbles (a fabricated observation waiting to be cited), and one whose empty slots are blank rather than marked, so a half-filled log is indistinguishable from a complete one |
| **AC-002** | **GIVEN** the protocol was fixed before anyone was recruited, **WHEN** U3 six months later asks whether the severity scale was tuned to the findings, **THEN** the scaffold's `## Scenario`, `## Severity scale` and the narration-mode field of `## Session record` are verbatim transcriptions of `../_design.md`'s protocol section, each naming that file inline as the source of record — and the scaffold names no scenario, no scale, no narration mode and no disqualifying criterion of its own | Static provenance + artifact-evidence, exactly as `../_decomposition.md`'s `## Testing brief` AC-002 row specifies: the transcribed strings match `../_design.md` character for character, and `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` predates this story's commit. Rejects a legend invented in the log because `dt9-and-fixed-protocol` had not landed yet — the retrofit that research 04 says separates a rankable record from a diary (`../_grounding.md:77-80`) |
| **AC-003** | **GIVEN** U3 reads the log as a `git diff` with every style stripped, or through a screen reader, **WHEN** they try to rank what stopped the reader, **THEN** every severity mark is a text token drawn from the token→meaning legend reproduced in `## Severity scale`, and nothing anywhere in the file carries meaning by colour or emoji alone | Static: read the file as raw bytes (`git show HEAD:<scaffold>`) and confirm every severity position is a word or a number from the legend; a scan for emoji or colour-swatch glyphs in a `Severity:` position returns nothing. Rejects the stoplight-emoji column research 04's source convention invites and `../_decomposition.md`'s `#### The accessibility floor` forbids by name |
| **AC-004** | **GIVEN** U2 is watching a stranger and typing at minute 40, **WHEN** a stumble happens, **THEN** the entry template already sitting in `## Chronological record` offers six one-line labelled fields — Time, Kind, Severity, What happened, Disposition, Revisions — so the only thing U2 decides is what just occurred; and `intervention` and `abandonment` are first-class values of Kind alongside `observation` and `reaction`, so a log with zero interventions *asserts* that none occurred (IQ-5) and a session that stops at the first blocker produces a finding rather than a void (IQ-6) | Artifact-evidence, ledger-cited `file:line` at the template block: all six labels present, all four Kind values enumerated, and "What happened" glossed in the scaffold as what the reader *did* — searches typed, links followed, files opened, commands run (`../project.md`, derived requirement 3). Rejects a template that is a bare heading with free prose beneath it, and one whose Kind vocabulary omits `intervention`, which makes IQ-5 unrecordable rather than merely unrecorded |
| **AC-005** | **GIVEN** a sibling project has already cited `FL-004` in its own risk table, **WHEN** it follows that anchor after the log is finalised, **THEN** it lands on the item it cited — because the scaffold fixes, in `## Chronological record`'s own preamble, that ids are `FL-###` assigned in occurrence order, never reassigned, that the section is append-only, and that the heading line including its label is frozen at write time | Artifact-evidence, ledger-cited at the preamble, checked against `../_decomposition.md`'s IQ-3 / UX-AC-007 wording. Rejects a preamble that says "ids are stable" and stops there: an id is stable and the anchor still dies if the label is re-worded, silently and with no error message. Also rejects "sorted by severity" as a stated presentation of the chronological section |
| **AC-006** | **GIVEN** U3 opens the log to find the items that are theirs, **WHEN** they read one stumble entry, **THEN** its disposition is legible at that entry without navigating anywhere (IQ-1), exactly one of five arms is expressible — `not yet dispositioned` / `fixed: <ref>` / `accepted: <reason>` / `routed: <id>` / `escalated: DT-<n>`, with zero arms and two arms both failures — and a `Revisions:` slot is present from the moment the entry is first written, defaulting to `none`, so a later change appends beside the original instead of erasing it (IQ-4) | Artifact-evidence, ledger-cited at the template block and at the arm list. Rejects the two failures decision 4 names: a template that defers the revisions slot until a disposition first changes, which is exactly how the original gets overwritten, and one whose disposition lives only in `## Dispositions index`, which converts every review into a context jump. Authority: `.kb/governance/rewrite-the-referent-never-the-reasoning.md` |
| **AC-007** | **GIVEN** a routed stumble arrives at an owner, **WHEN** that owner asks whether it is theirs, **THEN** the shape has already answered — the `routed:` arm is typed to an item id that resolves to a real file (`HS-P0022`, `HS-P0023`, the `support` initiative per `.redkiln/config.yaml:5`, or a named staged deferral) and the `escalated:` arm to a DT id from `.bklg/docs-that-teach/_decomposition.md`'s ownership table, so a prose destination is not expressible in the shape at all | Static existence + artifact-evidence: `test -f .bklg/support/initiative.md`, `test -f .bklg/docs-that-teach/application-author-path/project.md`, `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md` all pass, and `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` yields the DT vocabulary the escalated arm names. Rejects IQ-7's own example, "route to the docs team", and an escalation arm typed to free text, which would let a DT id that does not exist pass as an escalation |
| **AC-008** | **GIVEN** a reviewer deletes the entire `## Dispositions index`, **WHEN** they re-read what is left, **THEN** not one stumble and not one disposition has been lost — the index says in its own first line that it is derived and that the chronological record is authoritative (IQ-2) — and the whole file is still readable and navigable with no script, no widget and no rendering step, browser find and stable heading anchors only, with every checkbox on a single line | Static + the deletion check stated verbatim in `../_decomposition.md`'s IQ-2 ("if the collapsed or filtered view were deleted, would the record still carry every stumble and its disposition?"): `rg -n "<details>\|<script>\|<summary>" <scaffold>` returns nothing, and every checkbox line matches a single-line `- [ ] …`. Rejects a severity roll-up authored as the primary home of a severity value, a load-bearing item inside a collapsed fold, and a wrapped checkbox — which `CLAUDE.md` records as unmatchable by the line-by-line gate parser |
| **AC-009** | **GIVEN** a person who has never seen this story reaches HS-P0024 from `redkiln board`, **WHEN** they open the project card, **THEN** one hop from `../project.md`'s `## Companions` list reaches the scaffold, `../_design.md`'s protocol section names it by path and heading vocabulary, and what they find is composed from this repository's existing document primitives — the brief spine, the `## Shape decision` table, the risk-table shape, `docs/README.md`'s two-column routing table, the one-line checkbox — rather than a hand-rolled format or a new navigation widget | Static: `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/comprehension-evidence/_design.md` returns both the Companions row and the protocol line; `git diff` shows no frontmatter hunk in either file. Artifact-evidence: a reviewer maps each of the eight sections to one of the five named primitives. Rejects the two failures the integration contract names: a scaffold that exists on disk but is absent from `## Companions` — constructed but unmounted — and a bespoke table-of-contents or fold widget, the anti-pattern `interaction-patterns.md` names for a medium that already renders the equivalent for free |

**Coverage of the traced project AC.** All nine serve `../project.md` AC-004, and specifically the
**shape half** of it that `../_storymap.md`'s `## Coverage` assigns here: six sections present, ids
stable, severity a text token. The content half — that the six are truthfully filled by an observed
walk — is `session-run-against-pinned-tree`'s and is not claimed by any row above. AC-006 and AC-007
pre-shape project AC-006/AC-007/AC-011 by landing the slots those criteria will be checked against;
they do not claim them, because no disposition exists yet to check.

## Interaction quality

The blocking invariants, in two families. **Every one of them is carried by an `AC-###` row in the
table above** — this section says only which row carries which, and how each is verified. Nothing
here is a free-floating bullet, because `redkiln verify` extracts ACs from table cells and bullets
with an `AC-###` label, and an invariant stated only as prose in this section would never be gated
and never be tested.

### State invariants

| Invariant | Carried by | How it is verified here |
| --- | --- | --- |
| **In place, not a context jump** (IQ-1) — the disposition is readable at the stumble; an index may exist in addition, and the hop is optional and at most one | **AC-006** | The entry template carries a `Disposition:` field itself, not a pointer into `## Dispositions index` |
| **Non-occlusion** (IQ-2) — a filter must not hide what it filters; nothing exists only in a roll-up or a fold | **AC-008** | The deletion check: delete the index, lose nothing. Plus the absence of `<details>`/`<summary>` |
| **Preserved position** (IQ-3) — ids and order are stable once written; append-only; a cited anchor still resolves | **AC-005** | The scaffold's own preamble fixes occurrence-ordered `FL-###`, no reassignment, and a frozen heading line **including its label** |
| **Reversibility** (IQ-4) — a changed disposition is recorded *as a change*, the earlier one still legible with its reason | **AC-006** | The `Revisions:` slot is present from first write, defaulting to `none` — never retrofitted at first revision |
| **Keyboard reachability** — the log is navigable with browser find and stable heading anchors alone; no pointer-only affordance, no widget, no script | **AC-008** | Static text completeness; nothing in the file requires a rendering tool to read |
| **Non-interference is recordable** (IQ-5) and **abandonment is a valid end state** (IQ-6) | **AC-004** | `intervention` and `abandonment` are first-class `Kind` values in the template, so zero interventions is an assertion rather than a silence |

### Composition invariants

`../_design.md` is signed off (2026-08-17, no conditions) with `hasSurface: false` and an empty items
block — it declares **no rendered surface and no public API item**, so there is no pixel density
budget, no token file and no component to compose from, and inventing one is out of scope by its own
words (*"there is no CSS layer and no token file, and inventing one is out of scope"*,
`../_design.md`, quoting `../_decomposition.md`'s `#### The primitive layer to compose from`). The
composition constraints it **does** carry are the five named document primitives and the one-line
checkbox, and those are binding on this story exactly as a component library would be.

| Invariant | Real numbers / named source | Carried by |
| --- | --- | --- |
| **Presentation exists at all** — every slot carries real composed presentation, not bare markup. A heading with an empty line under it is the document-medium equivalent of an unstyled render: it satisfies every structural check and teaches U2 nothing at minute 40 | Each of the eight sections carries a labelled, typed shape: a banner sentence, a transcribed paragraph, six named context fields, a token→meaning table, a copyable entry template, a two-column routing table, a scope sentence, five hand-off slots | **AC-001**, **AC-004**, **AC-009** |
| **Composition and placement** — each section maps to a named existing primitive, none hand-rolled | `.redkiln/templates/briefs/brief.md`'s Intent/AC/Notes spine; `.redkiln/templates/_design.md`'s `## Shape decision` table; the `\| Risk \| Likelihood / Impact \| Mitigation \|` shape; `docs/README.md:12-23`'s two-column routing table; `.redkiln/templates/gates/discover.md:9-13`'s one-line checkbox | **AC-009** |
| **Transience** — what is persistent chrome, what is revealed, what is opened on demand | Persistent and visible-by-default: `## Status`, `## Scenario`, `## Severity scale`, `## Chronological record` in full. Derived and deletable: `## Dispositions index`. Opened on demand: **nothing** — there is no fold, no tab and no collapse in this file, by rule | **AC-008** |
| **Density budget, with its numbers** | Exactly **8** sections; **6** one-line fields per stumble entry; **4** `Kind` values; **5** disposition arms; **1** scenario of one to two sentences; **2** columns in the dispositions index; **1** line per checkbox; **1** emptiness token (`NOT-YET-RECORDED`), used everywhere and nowhere varied | **AC-001**, **AC-004**, **AC-006** |
| **Hierarchy** — the reading order is the protocol's order, and a skimmer hits the disclaimer first | `## Status` is section 1 so that the first thing any reader meets is "no session has been run"; the six AC-004 elements occupy sections 2–5; the additive and post-session sections come last | **AC-001** |
| **Named anti-patterns** — a bespoke navigation widget over a medium that renders the equivalent for free; a load-bearing item behind a fold, an inactive tab or a collapsed admonition; colour or emoji as the sole carrier of meaning; a wrapped checkbox; an illustrative placeholder stumble | `../_decomposition.md`, `#### The primitive layer…` and `#### The accessibility floor`; `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md`, `## Anti-patterns`; `CLAUDE.md` on the line-by-line parser | **AC-003**, **AC-008**, **AC-009**, **AC-001** |

## Error conditions

| id | Condition | Required behaviour |
| --- | --- | --- |
| **EC-001** | `../_design.md`'s protocol section already fixes the log's path or its heading vocabulary | `_design.md` **wins verbatim**. The choices in `## Behavior and interfaces` are discarded in its favour, the second mount becomes a no-op rather than an addition, and the discrepancy is noted in the implementation report. `../_storymap.md` is explicit that the path and heading vocabulary are `_design.md`'s to fix, not this spec's |
| **EC-002** | `dt9-and-fixed-protocol` has not landed, so no severity scale, scenario, narration mode or disqualifying criteria exist to transcribe | **Stop; do not guess.** Writing a legend from this spec's guesses and editing it afterwards is precisely the retrofit decision 1 forbids and AC-002's provenance check exists to catch. Both stories are in one slice and implemented sequentially in one context; the correct action is to finish the protocol story first |
| **EC-003** | The `## Companions` mount appears to require a change above the closing `---` of `project.md` | It does not, and the `PreToolUse` hook denies it. The Companions list is body prose; `redkiln` is the only writer of `id`, `stage`, `status`, `updated` and `links` (`CLAUDE.md`, `## Where the work lives`). If a mount seems to need frontmatter, the mount is wrong |
| **EC-004** | A change appears to need a path outside the four in `## PR boundary`'s fenced block | The story has absorbed a consumer's work. **Split it rather than widening the block** — `redkiln verify --grain story` reads that block and fails on any file changed outside it |
| **EC-005** | Any slot in the scaffold carries a plausible-looking value instead of the token — a sample severity, an example stumble, a placeholder date | A defect, not a nicety. `../project.md`'s risk table row 2 is overclaiming downstream, and a template carrying illustrative stumbles is a fabricated observation waiting to be cited by someone who skimmed. Replace with the token |
| **EC-006** | `redkiln doctor` reports a seventh `template-drift` advisory, or fewer than six | A template was changed without a decision, or a customisation was reverted. Investigate; **never run `redkiln adopt --templates`**, which would overwrite all six customisations silently (`CLAUDE.md`) |
| **EC-007** | A disposition arm's destination id does not resolve to a real file at the time the scaffold is written | The scaffold names the destination *vocabulary*, not any specific routed item, so this cannot occur in this PR. If a worked example in the arm list cites an id, it must be one of the three verified in AC-007 — otherwise the scaffold teaches an unresolvable citation |

## Non-functional

| id | Requirement | Why it is here |
| --- | --- | --- |
| **NF-001** | The scaffold is complete and legible as raw text in a `git diff`, at ordinary terminal width, with no rendering step | `../_decomposition.md`'s accessibility floor: a screen reader and a `git diff` must both read the severity. This is the reduced-motion floor stated for a medium with no motion — the obligation is that the text is complete |
| **NF-002** | Exactly one emptiness token and one id prefix, both greppable, both used without variation | The gate for AC-001 is an `rg` count. Two spellings of "not yet recorded" make the count meaningless and a half-filled log indistinguishable from a complete one |
| **NF-003** | The story adds no compile cost and no new tool. `cargo xtask affected --base main` falls through to the five file-reading lints and `spec-trace` on an empty package set | `.redkiln/config.yaml:40`'s `affected_gate` comment block; the testing brief's Notes. The story still gets a real check rather than a vacuous pass |
| **NF-004** | Stumble ids are treated as public identifiers from the moment they are written — durability, not tidiness | Two sibling projects have already declared they will accept dispositions from here by reference (`../_decomposition.md`, `## UX brief`, `### Notes`), and `.redkiln/templates/_ledger.md`'s `evidence: "file:line"` contract is what makes a stable anchor worth having |
| **NF-005** | The scaffold introduces no dependency, no script, no generated file and no new heading vocabulary beyond the eight sections this spec fixes | `../_design.md`: no CSS layer, no token file, inventing one is out of scope. The file must survive being read by someone with nothing but a text editor |
| **NF-006** | The diff is small enough to review in one sitting: one new file plus two one-line prose additions | The PR boundary is narrow on purpose; a large diff here is a signal the story has absorbed the session |

## Implementation notes (non-prescriptive)

Shape only — the implementer owns the wording.

- **Order of work follows the slice, not the file.** `dt9-and-fixed-protocol` is committed first; then
  read `../_design.md`'s protocol section and transcribe from it. Writing the scaffold first and
  back-filling the legend produces the retrofit EC-002 exists to prevent, even if the final bytes
  happen to match.
- **Let the scaffold teach its own filler.** The rules that matter at minute 40 — append-only,
  occurrence-ordered ids, frozen heading lines, exactly one disposition arm, revisions appended not
  overwritten — belong in each section's own one-or-two-line preamble, not only in this spec. U2 will
  have this file open and will not have the spec open.
- **The entry template is a copyable block, not a description of one.** A fenced block with the six
  labels and a `NOT-YET-RECORDED` in each value position is copied in one keystroke; a paragraph
  describing what an entry should contain is retyped from memory and drifts by entry three.
- **Put `## Status` first and make it blunt.** One sentence, present tense, naming the token. A reader
  who skims two lines must come away knowing this file contains no findings.
- **Transcribe with attribution inline.** Each transcribed block names `_design.md` as its source of
  record in the same paragraph, so a reader who has only this file can still tell what was decided
  elsewhere and what was decided here.
- **Do the mount before the polish.** The `## Companions` row and the `_design.md` protocol line are
  two lines of prose and are the difference between a mounted artifact and an orphan; write them
  early rather than as the last commit, where they get forgotten.
- **Resist automating AC-006's future check.** The testing brief forbids by name a script that merely
  confirms a `Disposition:` line exists — it would pass `noted` as readily as
  `routed: HS-P0022, see FL-004`, and rejects no wrong implementation. This story lands the slot; it
  does not land a checker for it.

## Tests and CI (merge gate)

Grounded in `../_decomposition.md`'s `## Testing brief` — its AC/tier table, its
Artifact-evidence definition (ledger-cited `file:line`, not eyeballing), and its explicit
instruction not to automate a structural presence check into something nothing can fail.

| tier | command / path | proves |
| --- | --- | --- |
| Static — structure | `rg -n "^## " .bklg/docs-that-teach/comprehension-evidence/_friction-log.md` | The eight sections exist in the fixed order (AC-001) |
| Static — emptiness | `rg -c "NOT-YET-RECORDED" …/_friction-log.md`; `rg -n "^### FL-" …/_friction-log.md` returns nothing | Zero findings, every slot explicitly marked; rejects a scaffold shipped with illustrative stumbles (AC-001) |
| Static — provenance | `git log --format=%aI -- .bklg/docs-that-teach/comprehension-evidence/_design.md` | The protocol predates the scaffold, so the transcription cannot have been fitted to a session (AC-002) |
| Static — plain text | `git show HEAD:.bklg/docs-that-teach/comprehension-evidence/_friction-log.md`; `rg -n "<details>\|<summary>\|<script>" …/_friction-log.md` returns nothing | Severity is readable with all styling stripped; nothing is behind a fold; no rendering step is required (AC-003, AC-008) |
| Static — checkbox shape | every checkbox line matches a single-line `- [ ] …` | The line-by-line gate parser can match it — `CLAUDE.md`'s named reason one of the six template customisations exists (AC-008) |
| Static — destination vocabulary | `test -f .bklg/support/initiative.md`; `test -f .bklg/docs-that-teach/application-author-path/project.md`; `test -f .bklg/docs-that-teach/reach-and-adapter-path/project.md`; `rg "^\| DT-" .bklg/docs-that-teach/_decomposition.md` | Every id the arm list teaches resolves to a real item and a real tension; rejects a prose destination and a fictitious DT id (AC-007) |
| Static — mount | `rg -n "_friction-log" .bklg/docs-that-teach/comprehension-evidence/project.md .bklg/docs-that-teach/comprehension-evidence/_design.md`; `git diff` carries no frontmatter hunk in either | The scaffold is reachable in one hop from the project card and named by path in the protocol; the CLI's frontmatter is untouched (AC-009) |
| Artifact-evidence | `.bklg/docs-that-teach/comprehension-evidence/friction-log-skeleton/_ledger.md` — one row per AC, each citing a real `file:line` into the scaffold | The shape claims a reviewer must read: the entry template, the id/append-only preamble, the arm list, the revisions default, the derived-index first line (AC-004, AC-005, AC-006, AC-008). `require_ledger: true` is set at `.redkiln/config.yaml:67` |
| Story gate | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | Falls through to the five file-reading lints and `spec-trace` — a story whose whole deliverable is markdown still gets a real check rather than a vacuous pass on an empty package set |
| Story verify | `redkiln verify --grain story` | The PR-boundary path block holds (no file changed outside the four paths) and every ledger row is satisfied with non-placeholder evidence before `implement → report` |
| Backlog hygiene | `redkiln validate --kb && redkiln doctor` | Clean, at exactly the six standing `template-drift` advisories `CLAUDE.md` documents (`../project.md`, DoD-8) |
| Project integration bar | `cargo xtask ci --fast` (`.redkiln/config.yaml:55`) | DoD-7. **Not this story's to turn green** — it touches no crate — but it must remain green; a scaffold that breaks `spec-trace` has broken something it never should have touched |

**Not run here, deliberately.** `cargo xtask ci` (the terminal `e2e` bar, `.redkiln/config.yaml:60`)
is HS-P0025's. And the friction-log *session* is not a CI step and never becomes one: U1 and U2 are
never mocked, scripted or simulated, because non-authorship is the mechanism rather than a
convenience (`../_decomposition.md`, `## Testing brief`, fixtures and seams).

## Risks and coupling (PR-scoped)

| Risk | Likelihood / Impact | Mitigation |
| --- | --- | --- |
| The scaffold lands before `dt9-and-fixed-protocol` and its legend is written from this spec's guesses | Medium / High | EC-002 makes this a stop condition, not a fix-later. Both stories are in the `session-protocol` slice and implemented sequentially in one context (`../_storymap.md`, `## Merge order`, step 1); AC-002's provenance check catches it after the fact if the ordering is broken anyway |
| The empty scaffold is cited downstream as evidence that a session happened | Low / High | AC-001 requires the `## Status` banner and the token in every slot; EC-005 makes a plausible placeholder a defect. This is `../project.md`'s risk-table row 2 pushed one story earlier, where it is cheap |
| `_design.md` fixes a path or heading vocabulary this spec has already guessed, and the two disagree | Medium / Medium | EC-001: `_design.md` wins verbatim and this spec's choices are discarded. Where `_design.md` is silent, the implementer records the choice **back into** `_design.md` via the second mount, so the two can never disagree afterwards |
| The shape is right but the file is never mounted, and nobody reaching HS-P0024 from the board finds it | Medium / Medium | AC-009 is a first-class criterion with a static check, not an afterthought; the integration contract states the acceptance bar includes the one-hop reach. Implementation notes put the mount early rather than last |
| The story quietly absorbs its consumer — someone starts drafting entries, a severity, a reader declaration | Medium / High | The `## PR boundary` fenced path block is only four paths and `redkiln verify --grain story` enforces it; EC-004 makes a fifth path a signal to split. `zero findings` is a checked property, not a convention |
| Freezing heading labels at write time is felt as excessive and a label is "tidied" during review | Low / High | AC-005 states the label freeze explicitly rather than leaving it implied by "stable ids". IQ-3's reason is quoted in the context pack: a re-worded label changes the anchor and the citation dies with no error message |
| A well-meant automation is added that checks a `Disposition:` line exists | Low / Medium | Named as forbidden in the PR boundary and in the implementation notes, with the testing brief's reason: it passes `noted` as readily as a real routed id, and so rejects no wrong implementation |
| Backwards coupling — this scaffold's arm vocabulary constrains what `route-and-escalate` can express three slices later | Low / Medium | The arms are drawn from `../project.md` AC-006/AC-007/AC-011 rather than invented here, and IQ-4's revisions slot means a disposition recorded under the wrong arm is revised, not rewritten |

## Dependencies

**Blocks on**

- **`dt9-and-fixed-protocol`** — the only edge, and it is hard. It writes the scenario, the narration
  mode, the severity scale and the logger-disqualifying criteria into `../_design.md`; this story
  transcribes them and invents none of them. It may also fix the log's path and heading vocabulary,
  in which case EC-001 applies. Both are in the `session-protocol` slice and are implemented
  sequentially in one context (`../_storymap.md`, `## Merge order`, step 1).

**Unlocks**

- **`non-insider-recruitment`** — writes the reader's declaration, platform, toolchain and assistive
  technology into this scaffold's `## Session record`, including the ineligible-but-used-anyway state.
- **`session-run-against-pinned-tree`** — fills `## Chronological record` and `## Session record`'s
  tree field. This is the consumer that keeps the foundation from being an unproven island, and it is
  one slice later (`../_storymap.md`, `### Why the slices fall here`).
- **`disposition-every-stumble`** — writes into the disposition slot this story lands, transitively.
- **`scope-the-claim`**, **`second-session-decision`**, **`handoff-note-to-closeout`** — fill
  `## Scope of the claim` and `## Hand-off`, transitively.

No story outside this project depends on this one. HS-P0025 `durable-audience-closeout` reads the
*filled* log, not the scaffold.

## Anchors (progressive disclosure)

Deferred depth, not optional depth. Open each at the moment its row names; do not preload the corpus.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `.bklg/docs-that-teach/comprehension-evidence/_design.md` | The signed-off design and, after `dt9-and-fixed-protocol` lands, the only record of the scenario, narration mode, severity scale and disqualifying criteria. It also declares `hasSurface: false` and the "no CSS layer, no token file" bound on composition | **First, before writing a single line of the scaffold** — everything transcribed comes from here, and if it fixes a path or heading vocabulary, EC-001 discards this spec's choices | AC-002, AC-003, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/_decomposition.md` | The UX brief holds IQ-1…IQ-7 verbatim, the states enumeration, the accessibility floor and the named primitive layer; the Testing brief holds the AC/tier table and the do-not-over-automate rule | Before AC-004, AC-005, AC-006 and AC-008 — the invariant wording is quoted, not paraphrased, and the states list is what stops a missing column | AC-004, AC-005, AC-006, AC-008 |
| `.bklg/docs-that-teach/comprehension-evidence/project.md` | AC-004 is the traced criterion; the risk table's row 2 is the overclaiming risk AC-001's banner answers; `## Companions` (lines 362–372) is the mount point | Before AC-001 (the banner's justification) and again at the mount, before AC-009 | AC-001, AC-009 |
| `.bklg/docs-that-teach/comprehension-evidence/_storymap.md` | `## Coverage` states the AC-004 shape/content split this story is bounded by; `### Why the slices fall here` states that the path and heading vocabulary belong to `_design.md`; `## Merge order` fixes the ordering EC-002 depends on | Before starting, to confirm the boundary; again if the work starts to feel like it should include a first entry | AC-001, AC-002 |
| `.bklg/docs-that-teach/comprehension-evidence/_grounding.md` | Lines 55–92 are the cited source of the auditable shape — the friction-log template, the 0–4 scale, concurrent vs retrospective narration, and Nielsen & Landauer's basis for the scope sentence | Before writing `## Scenario`, `## Severity scale` and `## Scope of the claim` — it says which parts are cited research rather than local invention | AC-002, AC-003 |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | The one KB atom binding on this work. It is the authority for the revisions slot existing from first write rather than being retrofitted | Before implementing the entry template's `Revisions:` field | AC-006 |
| `.bklg/docs-that-teach/_decomposition.md` | `## Design tension ownership` is the DT-id vocabulary the `escalated:` arm is typed to; `### Definition of Done` row 5 is the scenario this story makes possible | Before writing the disposition arm list | AC-007 |
| `docs/README.md` | Lines 12–23 are the live two-column routing table the `## Dispositions index` reuses rather than replaces — the primitive, in the repository, already rendering | Before writing `## Dispositions index` | AC-008, AC-009 |
| `.redkiln/templates/gates/discover.md` | Lines 9–13 are the one-line checkbox primitive; the parser matches line by line and a wrapped box can never match | Before adding any checkbox to the scaffold | AC-008 |
| `.redkiln/templates/_design.md` | The `## Shape decision` table and `## Sign-off` section — the existing primitives for recording a decision with its rejected alternatives, which the disposition and hand-off shapes compose from | While composing the disposition arm list and `## Hand-off` | AC-006, AC-009 |
| `.redkiln/templates/briefs/brief.md` | The Intent / Acceptance Criteria / Notes spine, the third named primitive | While laying out the scaffold's section skeleton | AC-009 |
| `.redkiln/templates/_ledger.md` | The `evidence: "file:line"` contract — the reason a stable stumble id is worth having at all, and the shape this story's own ledger takes | When authoring the ledger, and when justifying NF-004 | AC-005 |
| `.redkiln/config.yaml` | Line 5 `support_initiative: support` (routed-arm vocabulary), line 40 `affected_gate`, line 55 `integration_scoped`, line 67 `require_ledger` | Before writing the routed arm, and when running the gate | AC-007 |
| `.bklg/support/initiative.md` | The `support` destination is real (`HS-I0005`) but structurally empty — `stage: intake`, zero projects. The scaffold must not imply a populated backlog | When writing the routed arm's worked example | AC-007 |
| `.bklg/docs-that-teach/_discovery/research/04-comprehension-as-evidence-documentation-usability-testing-co.md` | The research angle the whole shape derives from: the friction-log template, non-authorship as mechanism, routing as the non-optional final step | Only if a shape decision needs its source rather than its summary — `_grounding.md:55-92` is the distilled form and is usually enough | AC-002, AC-003 |
| `.bklg/docs-that-teach/_discovery/distillation/interaction-patterns.md` | `## Anti-patterns` names the bespoke navigation widget and the load-bearing-item-behind-a-fold failures the composition invariants forbid | Before adding any navigational affordance to the scaffold — the answer is almost always a link, not a component | AC-008, AC-009 |

## Clarifications resolved during spec

1. **The emptiness token is `NOT-YET-RECORDED`.** The front half required "one explicit, greppable
   not-yet-recorded token, named once" but did not fix the literal. Fixed here so AC-001's check is an
   `rg` count rather than a judgment call; NF-002 forbids a second spelling. If `_design.md`'s
   protocol section names a different marker, EC-001 applies and that one wins.

2. **The AC ids are exactly the nine the front half enumerated** — AC-001 … AC-009 — with none added
   and none dropped. The mapping is: AC-001 shape and emptiness, AC-002 transcription and provenance,
   AC-003 severity as text token, AC-004 the entry template and its four kinds, AC-005 id stability
   and the label freeze, AC-006 disposition-at-the-stumble plus the revisions slot, AC-007 id-typed
   destinations, AC-008 non-occlusion plus static-text completeness, AC-009 the mount plus primitive
   fidelity.

3. **Composition invariants are stated against document primitives, not a design system, and this is
   the signed-off position rather than a gap.** `../_design.md` records `hasSurface: false` with an
   empty items block and every shape section `N/A`, and it states that there is no CSS layer and no
   token file and that inventing one is out of scope. The composition family above is therefore bound
   to the five named repository document primitives and the one-line checkbox — which are real files,
   verified present — and the density budget is stated in counts of sections, fields and arms rather
   than in pixels.

4. **This story renders no `_design.md` item id, because `_design.md` declares none.** It builds
   surface **2** of the three *document* surfaces the UX brief enumerates (the friction log). The
   integration contract already records this; it is restated here so a reviewer does not read the
   empty items block as a missed binding.

5. **`## Dispositions index` is in the scaffold even though no disposition can exist yet.** It is
   section 6 of eight and lands empty with the token, because IQ-2's deletion property is a property
   of the *shape* — an index authored later, after entries exist, is exactly the roll-up that becomes
   the primary home of a severity value. Landing it empty with its "derived; the chronological record
   is authoritative" first line is what makes the property checkable now.

6. **No verifying test is a compiled test, and that is honest rather than a shortfall.** This story
   adds no `pub` item and touches no crate, so its ACs are verified by the Static and
   Artifact-evidence tiers the project's own testing brief defines, plus `cargo xtask affected --base
   main`'s fall-through. The brief's warning is respected: every static check above names the wrong
   implementation it rejects, and the one check that could not reject a wrong implementation — "a
   `Disposition:` line exists" — is deliberately not written.

7. **`escalated:` is listed as a fifth arm rather than folded into `routed:`.** `../project.md`
   AC-011 and the UX brief's states list both treat escalation as a distinct state from routing, and
   the two carry different id vocabularies — an item id versus a DT id. Folding them would make
   AC-007's mechanical DT-id check impossible.
