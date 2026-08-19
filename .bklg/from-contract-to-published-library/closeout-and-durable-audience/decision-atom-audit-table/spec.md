---
item: HS-S0129
stage: spec
created: 2026-08-12T13:48:08.437Z
updated: 2026-08-12T13:48:08.437Z
template_sig: 87bbf1d0
rendered_sig: 4e0e43e5
---

# Spec — An accepted atom per settled answer, naming the alternatives that lost

## Scope lock

| What | Path |
| ---- | ---- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — BR-15 at `:298`; DoD 14/15 at `:394-403`; the DT-1…DT-8 tension table at `:420-427` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` — AC-005 at `:207-210`, DR-6 at `:148-152`, DoD item 3 |
| This spec | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/spec.md` |
| Key brief (the only warranted one) | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` — *Testing brief* AC-005 row at `:48`; **DR-6 audit procedure** and the fixed row shape at `:125-146` |
| Grounding | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md:20-27` — seventeen atoms on disk at planning time, `0017`–`0028` reserved and unwritten |
| Signed-off design | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` — **no public API surface**, `## Items` is `N/A`. Binding as a *prohibition*: this story renders and changes zero Rust API items |
| Story map (slice + one-liner) | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md:57` (this row), `:147` (merge order 3), `:26-30` (where evidence lands) |
| Discover (this story) | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/discover.md` — the signal ledger, the two deferred questions, and the named wrong implementation |
| Roadmap pointer | `RUNBOOK.md:262-307` — the ADR queue this story audits; `:276-283` the reserved-number precedent; `:309-320` the coverage-audit precedent |

## One-line PR slice

Produce the audit table mapping every question this initiative settled to an accepted atom under
`.kb/decisions/` and the alternatives that atom records as rejected, with a stated reason for each
reserved ADR number nobody needed to write.

## Executive summary

This PR lands **one section of `_closeout-record.md`** — the decision-atom audit — and this story's
`_ledger.md`. Nothing else. It writes no Rust, edits no `.kb/` atom, and repairs no defect it finds.

The delta over what already exists: the row *shape* is already fixed by the testing brief
(`_decomposition.md:135-137` — Settled question / Atom / Status / Alternatives), and the brief
explicitly deferred filling it: "to be filled at closeout, not now." This story fills it, against
the tree as it actually is at execution time rather than against the planning-time snapshot, and it
adds the three things the row shape alone does not force:

1. the enumeration runs in **both directions** — every reserved number in `RUNBOOK.md`'s queue *and*
   every atom on disk, so an answer nobody reserved a number for (ADR-0029 is the standing example)
   cannot fall out of the table;
2. the "alternatives" cell is **read out of the atom's body** and cited `file:line`, never inferred
   from the atom's existence;
3. coverage is **computed**, not assumed — the union of what the written atoms actually discharge is
   compared against a re-derived list of what this initiative settled, because `RUNBOOK.md:309-320`
   already recorded phase 4 discovering that the queue's own parenthetical clause ranges were stale
   by 29 clause IDs and that "nothing checks that they are equal."

## Context pack

Read this and you can start. Everything deeper is a signposted anchor (second pass).

**This story audits; it never repairs.** BR-15's audit half exists because a settled answer that
never became an accepted atom is, from outside the room where it was decided, indistinguishable from
an answer nobody made. If the audit finds a gap, the gap becomes a **row plus a routed finding**, not
a fix. Two things make this non-negotiable rather than stylistic: `.kb/decisions/README.md:7-18` —
an accepted decision's body is *never* edited, `redkiln validate --kb` checks each accepted atom
against `HEAD` and fails the gate on a changed body, so a correction is a *new* atom carrying
`supersedes:`; and `project.md` AC-013 / DR-12, which make routing the deliverable of a sibling story
(`findings-disposition-register`) and make "zero fixed inside this project" an observable. A story
that repairs a defect it found has failed its own acceptance criterion (`_storymap.md:92-94`).

**The bar for a row is not "an atom exists."** `.kb/decisions/README.md:31-33`: "A decision recorded
without its rejected options is indistinguishable from an accident, and the next person to meet the
same fork has no way to know it was a fork." AC-005 (`project.md:207-210`) therefore demands three
independent facts per row — the atom's *path*, its *status: accepted*, and its *body actually
recording what lost. The named wrong implementation (`discover.md:38-40`) is precisely a table that
satisfies the first, assumes the second and never checks the third.

**A reserved number staying empty can be correct.** `RUNBOOK.md:276-283` is the precedent, in the
queue's own words: ADR-0009 "may not be written at phase 2 at all... A reserved number that stays
empty for seven phases is the cost of ES-6 being genuinely open, not a scheduling defect." So an
unwritten `0017`–`0028` is not automatically a gap. It is a gap only when nobody can state why it was
not needed — and the discriminator is mechanical rather than a judgement call: **the owning sibling
project is closed out and the question it was reserved for was nonetheless answered somewhere.** That
combination is a finding for `findings-disposition-register`, not a reason this story is allowed to
write on the sibling's behalf (`discover.md:30`).

**Enumeration is bidirectional, and the queue is not the authority on its own completeness.** Walking
`RUNBOOK.md:262-307`'s table alone produces a table that cannot see ADR-0029 — an atom that exists,
is accepted, amends ADR-0004, and which the queue itself lists as "*(unscheduled — the queue had no
number for it)*". Walking `.kb/decisions/`'s listing alone produces a table with no row for anything
reserved and unwritten, which is the exact failure DR-6 names. Both walks, unioned by number, or the
table is wrong in one of two ways it is already known to be wrong in.

**Coverage is computed against four enumerations, not one.** What "this initiative settled" is
re-derived from: (a) the ADR queue `0008`–`0028` (`RUNBOOK.md:262-307`); (b) `.kb/decisions/`'s
actual listing, which catches unscheduled numbers; (c) the open questions this initiative consumed —
the six DR-7 names at `project.md:152-161`, cross-read against `.kb/maps/open-questions-index.md`,
whose standing rule (`:11-13`, restated in its own `summary`) is that a resolved question stays
listed and annotated rather than removed; and (d) the initiative's own design tensions **DT-1…DT-8**
(`initiative.md:420-427`), each of which was assigned an owning project and resolved in that
project's `_design.md`. (d) is the enumeration most likely to be skipped and the one most likely to
find something: a DT resolution that commits the project — a *must*, *shall*, *must not* — belongs in
`.kb/decisions/` per `README.md:25-29` and not only in a `_design.md`, because a `_design.md` carries
no immutability.

**The mount is the shared record, not a loose file.** `_storymap.md:26-30`: the stories that must be
read together — the DoD set, the delta, the audit tables, the findings — converge on one companion
artefact, `_closeout-record.md`, in the project directory. Fourteen ledger entries in fourteen places
is the failure mode the charter's DoD preamble is written against. This story appends its section to
that record; the record itself is stood up by `clean-checkout-harness` in slice 1, which precedes
this story in merge order (`_storymap.md:140-148`).

**Its slice-mate consumes this story's enumeration and nothing else.**
`open-question-preservation-audit` depends on this story (`_storymap.md:58`) because it needs the
list of "what this initiative settled" to check that each consumed open-question atom is still on
disk with a resolution-bearing status. The enumeration therefore has a second reader inside the same
context and must be a named, stable artefact in the record — not a working list held in the head of
whoever ran the audit.

**The persona-journey slice this serves.** No screen, and the design sign-off says so explicitly
(`_design.md:41-49`: `## Items` and `## Signatures` are both `N/A`, and the `design.capture`
perceptual review is a *declared* skip). The reader this story is written for is the one
`_storymap.md:17-22` names: someone who must be able to believe the initiative closed honestly
**without re-deriving the evidence**. Every row therefore carries a citation a reader can follow;
a row whose evidence is "I checked" is not done.

## Integration contract

| Field | Value |
| ----- | ----- |
| **Archetype** | `capability` (`story.md` frontmatter `archetype: capability`) — a reader-observable slice: the audit table is the thing a reader meets |
| **Slice / milestone** | `answers-audit` (`_storymap.md:57-58`) |
| **Slice-mates** (one context, one integrated surface) | `open-question-preservation-audit` (HS-S0130) — it consumes this story's settled-question enumeration; `_storymap.md:78-80`: "the second is meaningless without the first's list" |
| **Mount point** | `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md` — the project's single convergent closeout artefact (`_storymap.md:26-30`). This story appends a `## Decision-atom audit (AC-005)` section to it. The record is stood up by `clean-checkout-harness` in slice 1 (`_storymap.md:53`), which merges first (`_storymap.md:140-148`); if it is absent at execution time that is a **blocking finding against slice 1**, not a licence to invent a different landing place |
| **Wires into** | `.kb/decisions/` (the atoms and their frontmatter — the audited surface, read-only); `.kb/decisions/README.md:7-18,25-33` (the immutability rule and the "state what lost" bar the audit applies); `RUNBOOK.md:262-307` (the queue being walked); `.kb/maps/decision-map.md` (the supersession graph, so a superseded atom is reported as superseded rather than as missing); `.kb/maps/open-questions-index.md:11-13` (the annotation convention the slice-mate checks); `initiative.md:420-427` (DT-1…DT-8); `redkiln validate --kb` (`.github/workflows/ci.yml` `backlog` job) as the conformance instrument over every cited atom |
| **Renders surfaces** | **none** — `_design.md` records no public API surface and no screen (`## Items`: "N/A — no public surface"). This story renders a markdown section of a backlog artefact, which the design sign-off explicitly places outside its scope (`_design.md:31-39`) |
| **Conformance rule(s)** | **Not adapter-observable, and deliberately so.** This story adds no rule to `crates/happenstance-testkit/src/suite.rs` and changes no port. Its subject is the knowledge base, not the contract; the instrument that observes it is `redkiln validate --kb` plus the cited-row check defined below. Adding a testkit rule here would be a rule no adapter could fail — decorative by `CLAUDE.md`'s own corollary |
| **Clause(s)** | **None discharged or amended.** No `spec/SPECIFICATION.md` clause changes; no `[FROZEN]` clause is touched, so no new ADR is owed by this story. If the audit *finds* that a settled answer moved a `[FROZEN]` clause without an atom, that is a finding routed per DR-12 ("anything touching a `[FROZEN]` clause → a new decision atom and a re-plan"), authored by whoever owns it, not here |
| **Advances DoD scenario** | **`project.md` Definition of done item 3** — "The audit table exists: every settled answer has an accepted decision atom naming its rejected alternatives" (AC-005 half; the `.kb/open-questions/` zero-deletion half is the slice-mate's). Upstream this is BR-15's audit half (`initiative.md:298`). It supplies the *atom-exists-and-is-accepted* evidence that initiative DoD 14 and DoD 15 (`initiative.md:394-403`) each assert about replication and retention — without displacing `dod-set-re-observation-record`, which owns re-observing those two scenarios and merges before this story |

**Delivered mounted.** The deliverable is the section inside `_closeout-record.md` plus the story's
`_ledger.md` (mandatory and cited per `.redkiln/config.yaml:62-67`, `require_ledger: true`, and tied
to a commit by `:69-73`, `require_commit_provenance: true`). A table that exists only in this story's
folder, unreferenced by the record a reader actually opens, is an unmounted component.

## PR boundary

The paths this story may touch. `redkiln verify --grain story` reads the first fenced block under
this heading and fails on any file changed outside it.

```
.bklg/from-contract-to-published-library/closeout-and-durable-audience/_closeout-record.md
.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/**
```

**In this PR**

- The `## Decision-atom audit (AC-005)` section appended to `_closeout-record.md`: the enumeration of
  settled questions, the audit table itself, the reserved-number disposition list, the computed
  coverage statement, and the findings list handed to `findings-disposition-register`.
- The named, stable **settled-question enumeration** the slice-mate consumes.
- This story's `_ledger.md` and, if it is authored, `_notes.md` in its own folder.
- The mount is inside the allowed globs, so appending to `_closeout-record.md` is not scope drift —
  it is the story's integration point.

**Explicitly not in this PR**

- **Any change under `.kb/`.** No atom is written, edited, superseded, or given a status flip; no map
  atom is updated. Writing a missing ADR is the owning sibling's, and `.kb/decisions/README.md:7-18`
  forbids editing an accepted body regardless of who is holding the pen.
- **Any change under `crates/`, `spec/`, `xtask/`, `standards/` or `references/`.** This project owns
  no code (`project.md` *Out of scope*).
- **The open-question preservation check** (`.kb/open-questions/` zero-deletion diff, the index
  annotations) — the slice-mate's, AC-006.
- **Routing the findings.** This story *produces* the findings list with enough detail to route;
  `findings-disposition-register` (HS-S0134) assigns each a destination (AC-013).
- **Re-litigating any recorded decision**, including HS-P0016's DT-1 resolution against the amended
  evaluator decision (`_decomposition.md:150-180`). Inconsistency is a finding, not a re-decision
  (`_storymap.md:100-106`).

**Merge DoD**: `_closeout-record.md` carries a decision-atom audit in which every reserved queue
number and every atom on disk has exactly one row, every atom row cites `file:line` for both its
accepted status and the alternatives it rejected, every unwritten number carries a stated reason or a
finding, and `redkiln validate --kb` is green on the audited tree — with zero files changed under
`.kb/`.

## Behavior and interfaces

`redkiln validate --kb` and `redkiln doctor` are run from the clean checkout slice 1 produced
(`_decomposition.md:103-107`: "All four commands run against the clean-checkout tree described in
DR-1, not against the working tree these planning artefacts are written in").

| Behavior or contract | Details | Evidence path |
| -------------------- | ------- | ------------- |
| **Bidirectional enumeration by number** | Build the row set as the *union* of (a) every ADR number the queue names, `0008`–`0028`, and (b) every `NNNN-*.md` in `.kb/decisions/`. Exactly one row per number. A number in (b) and not (a) is a real row, labelled unscheduled — ADR-0029 is the standing case and the queue itself marks it "*(unscheduled — the queue had no number for it)*". Superseded atoms (`0002`, and the partial ones the README names) are rows too, reported as superseded with their successor, not omitted as "not accepted" | `RUNBOOK.md:262-307`; `.kb/decisions/` listing; `.kb/decisions/README.md:15-18`; `.kb/maps/decision-map.md` |
| **Row shape is fixed upstream, not invented here** | Four columns, exactly as the testing brief fixed them: *Settled question* / *Atom* / *Status* / *Alternatives it records as rejected*. Two columns are added and are additive only: *Evidence* (`file:line` into the atom body for the alternatives) and *Finding* (empty, or the id handed to `findings-disposition-register`) | `_decomposition.md:135-137` |
| **Status is read, never assumed** | Each atom row states `status:` as read from that atom's frontmatter on the audited tree. `accepted` is what AC-005 requires; `superseded` is reported with `superseded_by`, and a settled question whose only atom is superseded with no accepted successor is a finding | `project.md:207-210`; `.kb/decisions/README.md:7-13` |
| **The alternatives cell quotes the body** | The cell names the rejected options and cites `file:line` in the atom body where they are stated. An atom that is `accepted` but whose body records no rejected alternative is a row with an **empty alternatives cell and a finding** — never a row silently left blank, and never repaired here. This is the exact discriminator between this audit and the wrong implementation | `.kb/decisions/README.md:31-33`; `discover.md:38-40` |
| **Reserved-but-unwritten numbers carry a reason or a finding** | For each of `0017`–`0028` (or whatever remains unwritten at execution time), the row states the reason it was not needed, following the ADR-0009 precedent. The discriminator: if the owning sibling project is closed out **and** the question the number was reserved for was answered elsewhere with no atom, the row records a finding instead of a reason. A blank row is neither | `RUNBOOK.md:276-283`; `project.md:148-152`; `discover.md:30` |
| **Coverage is computed, not assumed** | State, as a short computation rather than a claim: the union of what the written atoms actually discharge, versus the re-derived list of what this initiative settled, drawn from the four enumerations in the Context pack — the queue, the directory, the six consumed open questions (`project.md:152-161`), and DT-1…DT-8 (`initiative.md:420-427`). Any settled question with no atom is a row with no atom plus a finding. Phase 4's queue rows were stale by 29 clause IDs and nothing caught it; the parenthetical clause ranges are a scope statement, not a coverage guarantee | `RUNBOOK.md:309-320`; `initiative.md:420-427`; `project.md:152-161` |
| **The enumeration is a named artefact, for the slice-mate** | The settled-question list is written as its own subsection with stable item labels the slice-mate can cite by name, not embedded only as the first column of the table | `_storymap.md:58,78-80` |
| **Mounted into the shared record** | The section is appended to `_closeout-record.md` under the project directory, alongside the DoD set, the delta and the findings, so a reader meets it in one sitting | `_storymap.md:26-30` |
| **The audit mutates nothing it audits** | Zero files changed under `.kb/`, provable by `git diff --stat -- .kb/` over this story's commit range. Every defect is a finding routed to HS-S0134 | `project.md` AC-013/DR-12; `.redkiln/config.yaml:69-73` |
| **Conformance of every cited atom** | `redkiln validate --kb` is run on the audited tree and its exit code and output are cited in the ledger. It is what proves the cited atoms carry valid `KbFrontmatter` and that no accepted body has drifted from `HEAD` — the audit reads frontmatter, and an unvalidated frontmatter read is a read of something that may not conform | `_decomposition.md:48`; `.kb/decisions/README.md:9-11` |
| **No new fixture, no new test, no mock** | The project introduces no fixtures (`_decomposition.md:109-123`) and mocks nothing: "A project whose entire deliverable is re-observation would falsify its own purpose by substituting a double for anything it claims to have re-run" | `_decomposition.md:121-123` |

## Data and migrations

**N/A — no schema, no store, no migration.** This story reads markdown frontmatter and bodies under
`.kb/decisions/` and writes markdown under `.bklg/`. No database, no serialised format, no wire
format and no persisted state is created, versioned or migrated; the project owns no code at all
(`project.md` *Out of scope*), and `_design.md` records no public API item that could carry a
compatibility obligation.

One adjacent constraint is worth stating so it is not mistaken for a migration: the knowledge base's
own change discipline is **supersession, not mutation** (`.kb/decisions/README.md:7-18`). Any
correction this audit motivates is a new atom authored by the owning sibling, which is a *decision*,
not a data migration — and it is out of this PR's boundary either way.

## Acceptance criteria

The reader these are written from is the one `_storymap.md:17-22` names: **someone who must be able
to believe the initiative closed honestly without re-deriving the evidence.** Two more readers touch
this section directly — the author of the slice-mate `open-question-preservation-audit`, who consumes
the enumeration, and the author of `findings-disposition-register`, who must route what this audit
finds. Each criterion below is a goal one of those three has, crossing the whole path from the tree
on disk to the record they open.

| id | criterion | verification |
| -- | --------- | ------------ |
| **AC-001** | **GIVEN** a reader who knows only that the initiative "settled some things", **WHEN** they open the decision-atom audit in `_closeout-record.md`, **THEN** they find exactly one row for every ADR number in the union of the queue's `0008`–`0028` (`RUNBOOK.md:284-307`) and every `NNNN-*.md` on disk under `.kb/decisions/` — so a number that exists on disk but was never queued (ADR-0029, which the queue itself marks "*(unscheduled — the queue had no number for it)*") has a row, and a number the queue reserved but nobody wrote has a row too; no number appears twice and none is absent | **Union-completeness check**: the row-number set of the section's table, compared against `ls .kb/decisions/*.md` and the queue rows at `RUNBOOK.md:284-307`, both listings quoted in the ledger evidence. Set difference in either direction = fail. This is the `_decomposition.md:48` Static tier for AC-005 ("`redkiln validate --kb` … plus a manual cross-reference against the ADR queue") |
| **AC-002** | **GIVEN** a reader deciding whether an answer actually stands, **WHEN** they read a row, **THEN** the row carries the four columns the testing brief fixed (`_decomposition.md:135-137` — *Settled question* / *Atom* / *Status* / *Alternatives it records as rejected*) plus the two additive ones (*Evidence*, *Finding*), and its *Status* is the value **read from that atom's frontmatter on the audited tree**, never assumed from the atom's existence — a `superseded` atom is reported as superseded **with its `superseded_by`**, not omitted and not silently counted as an answer | **Status-provenance check**: for every atom row, the `status:` (and `superseded_by:` where present) in the cell matches the atom's own frontmatter line, cited `file:line`; cross-read against the supersession graph in `.kb/maps/decision-map.md`. `redkiln validate --kb` exits zero on the audited tree, which is what makes those frontmatter reads reads of something that conforms (`.kb/decisions/README.md:9-13`) |
| **AC-003** | **GIVEN** the reader who meets the same fork later and needs to know it *was* a fork (`.kb/decisions/README.md:31-33`), **WHEN** they read the *Alternatives* cell, **THEN** it names the options that lost **as quoted from the atom's body** and cites the `file:line` where the body states them — and an atom that is `accepted` but whose body records no rejected alternative yields an **empty alternatives cell plus a finding id**, never a blank cell, never a plausible-sounding alternative inferred from the atom's title, and never a repair | **Citation-resolution sweep**: every *Evidence* `file:line` in the table is opened and the named alternative is present at that location; every empty *Alternatives* cell carries a non-empty *Finding*. This is the discriminator against the named wrong implementation (`discover.md:38-40`), which passes AC-001 and AC-002 and fails only here |
| **AC-004** | **GIVEN** a reader who counts `0017`–`0028` missing from `.kb/decisions/` and wants to know whether that is a gap, **WHEN** they read the reserved-number disposition list, **THEN** every number unwritten at execution time carries either a **stated reason it was not needed** — following the ADR-0009 precedent that "a reserved number that stays empty … is the cost of [the question] being genuinely open, not a scheduling defect" (`RUNBOOK.md:276-283`) — or, where its owning sibling project is closed out **and** the question it was reserved for was nonetheless answered somewhere with no atom, a **finding id**; and never a blank | **Reserved-number disposition check**: for each unwritten number, the row's reason cites the sibling project and the state that makes the emptiness correct, or names a finding. A row with neither = fail. The owning-sibling mapping is read from `project.md:148-152` (DR-6) and `_decomposition.md:125-131` |
| **AC-005** | **GIVEN** a reader who wants to know whether the audit is *complete* rather than merely internally consistent, **WHEN** they read the coverage statement, **THEN** they find a **computation, not a claim** — the union of what the written atoms actually discharge set against a re-derived list of what this initiative settled, drawn from all four enumerations (the queue `RUNBOOK.md:284-307`; the `.kb/decisions/` listing; the six consumed open questions at `project.md:153-161` cross-read against `.kb/maps/open-questions-index.md`; and DT-1…DT-8 at `initiative.md:420-427`) — with every settled question that has no atom appearing as a row with no atom plus a finding | **Coverage computation check**: the section shows the four input lists and the set difference between them, with a per-DT line stating where each of DT-1…DT-8 was resolved and whether that resolution is a commitment (*must* / *shall* / *must not*) that `.kb/decisions/README.md:25-29` puts in the decisions layer rather than only in a `_design.md`. A statement of the form "coverage is complete" with no shown difference = fail — that is exactly the phase-4 failure `RUNBOOK.md:309-320` records ("two numbers, and nothing checks that they are equal") |
| **AC-006** | **GIVEN** the author of the slice-mate `open-question-preservation-audit`, working in the same context, **WHEN** they need "what this initiative settled" to check each consumed open-question atom, **THEN** they cite a **named subsection with stable per-item labels** in `_closeout-record.md` rather than re-deriving the list or lifting it out of the first table column — the enumeration is an artefact with an address, not a working list held in the auditor's head (`_storymap.md:58,78-80`) | **Consumer-citability check**: the enumeration exists as its own heading with one labelled item per settled question, and the slice-mate's own record cites those labels verbatim. A first-column-only enumeration = fail |
| **AC-007** | **GIVEN** a reader who opens the project's one closeout artefact, **WHEN** they reach the decision-atom audit, **THEN** they meet it **composed in place** in `_closeout-record.md` — a `## Decision-atom audit (AC-005)` section carrying its four subsections in the order enumeration → table → reserved-number disposition → coverage-and-findings, the table rendered with its full six columns, no cell reading "I checked" or "yes", and the prior sections slice 1 and slices 1–2 wrote left byte-identical — rather than as a loose file in the story folder that the record merely mentions (`_storymap.md:26-30`: "fourteen ledger entries in fourteen places is the failure mode") | **Composition + non-occlusion check**: `git diff` of `_closeout-record.md` over this story's commit range shows **additions only**, with every pre-existing line unchanged; the appended section carries all four subsections in that order, a six-column table of no fewer than 29 rows (0001–0029 on the planning-time tree), and every claim cell carrying a `file:line`. A primary table living only under `decision-atom-audit-table/` = unmounted = fail |
| **AC-008** | **GIVEN** the author of `findings-disposition-register` (AC-013), **WHEN** they pick this story's findings up, **THEN** each is listed with enough detail to route (what was expected, what was observed, the owning sibling or `[FROZEN]` clause it touches) **and the audited surface is provably unmutated** — `git diff --stat -- .kb/` over this story's commit range is empty, no atom body was edited, no status flipped, no missing ADR written, and `redkiln validate --kb` exits zero on the audited tree with its output cited | **Read-only + routability check**: `git diff --stat -- .kb/` empty over the range; the findings list has one entry per finding id referenced anywhere in the section, each with expected/observed/owner; `redkiln validate --kb` exit code and output recorded in `_ledger.md` (`.redkiln/config.yaml:67,73`). Any change under `.kb/` = fail, however correct the change is (`.kb/decisions/README.md:7-18`) |

Traceability: all eight rows serve **project AC-005** (`project.md:207-210`) and, through it, DoD item
3 and BR-15's audit half (`initiative.md:298`). AC-006 additionally exists because the slice-mate
`open-question-preservation-audit` (project AC-006) declares a dependency on this story
(`_storymap.md:58`); it does not discharge AC-006, it makes discharging it possible.

## Interaction quality

RFC §6.7/D6. This story renders **no screen and no public API item** — the signed-off `_design.md`
records `## Items` as "N/A — no public surface" (`_design.md:41-45`) and the `design.capture`
perceptual review is a *declared* skip (`_design.md:93-97`, `.redkiln/config.yaml:75-83`). That
sign-off is binding here as a **prohibition**, not a licence: it forbids this story from inventing a
surface, and it leaves the composition rules for the artefact it *does* render to be taken from the
two places that actually fixed them — the testing brief's row shape (`_decomposition.md:135-137`) and
the story map's convergence rule (`_storymap.md:26-30`).

Every invariant below is carried by an AC row in the table above; none is stated only here.

**STATE invariants**

| Invariant | Carried by | How verified |
| --------- | ---------- | ------------ |
| **In place, not a context jump** — the reader meets the audit inside the record they already have open; the primary content never lives in a second file the record only points at | AC-007 | The mount is `_closeout-record.md`; a story-folder-only table fails AC-007 |
| **Non-occlusion** — appending this section leaves slices 1–2's sections in that record untouched and still reachable | AC-007 | `git diff` of `_closeout-record.md` shows additions only; every pre-existing line byte-identical |
| **Reversibility** — the whole change is one revertible commit, and reverting it restores the audited tree exactly, because nothing under `.kb/` was touched | AC-008 | `git diff --stat -- .kb/` empty; commit provenance recorded per `.redkiln/config.yaml:73` |
| **Preserved reading position / selection / focus / scroll, keyboard reachability** | **N/A, declared not skipped** | There is no interactive surface to preserve state in (`_design.md:41-49`). Recording this as an explicit N/A is the point: a silent omission and an unrendered surface look identical, which is the same failure mode `.redkiln/config.yaml:75-83` names for an undeclared gate |

**COMPOSITION invariants**

| Invariant | Carried by | How verified |
| --------- | ---------- | ------------ |
| **Presentation exists at all** — the section is composed (heading, four ordered subsections, a real six-column table), not a bare dump of atom filenames | AC-007 | Structure check: the four subsections present in the fixed order enumeration → table → reserved-number disposition → coverage-and-findings |
| **Placement** — inside `_closeout-record.md` under the project directory, adjacent to the DoD set, the delta and the findings | AC-007 | Mount-point check (`_storymap.md:26-30`) |
| **Transience** — the audit is *persistent chrome* in the record: nothing collapsed, nothing gated behind a link the reader must follow to get the answer. The deeper layer — the atom bodies, the queue table — is the on-demand tier, reached through `file:line` citations | AC-003, AC-007 | Every conclusion is legible in the section; every conclusion is also citable to a deeper artefact |
| **Density budget, with its numbers** — exactly 6 columns; exactly 1 row per ADR number; **no fewer than 29 rows** on the planning-time tree (`0001`–`0016` and `0029` on disk, `0017`–`0028` reserved — `.kb/decisions/` listing, `_grounding.md:20-27`); 4 subsections, no fifth; 0 cells whose evidence is "I checked" | AC-001, AC-007 | Row/column count and citation sweep |
| **Hierarchy** — the enumeration comes *first* (the slice-mate reads it before the table), the coverage computation comes *after* the table (it is the conclusion, not a preamble), and the findings come last (they are the handoff to HS-S0134) | AC-006, AC-007 | Subsection order check |
| **Anti-pattern: the on-disk-only table presented as complete** (`discover.md:38-40`) | AC-001, AC-003 | Union-completeness + citation-resolution checks; either one alone lets it through |
| **Anti-pattern: the loose companion file** — an audit that exists but is not in the artefact the reader opens (`_storymap.md:26-30`) | AC-007 | Mount check |
| **Anti-pattern: the silent repair** — fixing what the audit found and reporting a clean table (`_storymap.md:92-94`) | AC-008 | `git diff --stat -- .kb/` empty |

## Error conditions

| id | Condition | Required behaviour |
| -- | --------- | ------------------ |
| **EC-001** | `_closeout-record.md` does not exist at execution time — `clean-checkout-harness` (slice 1, merge order 1 at `_storymap.md:140-143`) has not landed | **Halt and report a blocking finding against slice 1.** Do not create a substitute landing place, and do not stand the record up on slice 1's behalf: the record's own preamble and the DoD-set sections are that story's deliverable, and inventing them here makes two competing records |
| **EC-002** | `redkiln validate --kb` exits non-zero on the audited tree | Record the failing output **verbatim** in the section and in `_ledger.md`, and route it as a finding. Every frontmatter read this audit performs is then a read of something not known to conform, so the affected rows say so rather than quietly asserting a status. Do **not** repair the atom to make the command green |
| **EC-003** | An atom on disk has an absent, malformed or unexpected `status:` value | The row records the raw value as read, plus a finding. `accepted` is never defaulted to — assuming the status is precisely the half of AC-005 the wrong implementation satisfies by omission (`discover.md:38-40`) |
| **EC-004** | A settled question's only atom is `superseded` with no accepted successor on the tree | The row reports `superseded` with its `superseded_by`, and the question is carried into the coverage computation as **unanswered**, with a finding. A supersession chain that ends nowhere is an answer that no longer stands |
| **EC-005** | A reserved number is still unwritten **and** its owning sibling project is closed out **and** the question it was reserved for was answered somewhere | Finding, not a reason (`discover.md:30`). The row states the sibling, the number, and where the answer actually landed. This story does not write the missing atom — `.kb/decisions/README.md:7-18` and the PR boundary both forbid it |
| **EC-006** | A DT-1…DT-8 resolution that commits the project (*must* / *shall* / *must not*) is recorded only in a project's `_design.md` and in no decision atom | Finding routed to `findings-disposition-register`, quoting the committing sentence and citing `.kb/decisions/README.md:25-29` ("a commitment that can be edited is not a commitment"). Not a decision this story takes, and not a re-litigation of the resolution itself (`_storymap.md:100-106`) |
| **EC-007** | The audit's own working tree shows any change under `.kb/` | The story has failed AC-008. Revert the change; do not justify it in the record. If the change was genuinely needed, it becomes a finding with an owner |
| **EC-008** | The queue's parenthesised clause ranges disagree with what the written atoms say they discharge | Report both numbers and the difference, exactly as phase 4 did (`RUNBOOK.md:309-320`); do not reconcile by editing `RUNBOOK.md`, which is outside this PR's boundary. The disagreement is a finding |
| **EC-009** | A new decision atom lands under `.kb/decisions/` between this story's audit run and its merge | The audited tree's commit SHA is stated in the section (NF-003), so the table is honest about *what* it audited. A number appearing after that SHA is the next reader's problem, not a silent inaccuracy |

## Non-functional

| id | Requirement | Why, and how it is observed |
| -- | ----------- | --------------------------- |
| **NF-001** | **Every normative cell carries a resolvable citation.** Zero cells whose justification is the auditor's assertion | The reader this project exists for cannot re-derive the evidence (`_storymap.md:17-22`); an uncited row asks them to. Observed by the citation-resolution sweep (AC-003) |
| **NF-002** | **Read-only over `.kb/`.** The audit's only writes are `_closeout-record.md` and its own story folder | `git diff --stat -- .kb/` empty over the commit range; the PR boundary block is enforced by `redkiln verify --grain story` |
| **NF-003** | **Reproducible.** The section states the commit SHA of the audited tree and the two listings it was derived from, so the audit can be re-run and the result compared | Without the SHA, "29 rows" is unfalsifiable a week later. Cited in the section header and in `_ledger.md` |
| **NF-004** | **Self-contained for a reader of the record alone.** No conclusion requires opening this story's folder; the story folder holds the ledger and working notes, never the answer | Mount check (AC-007) |
| **NF-005** | **No new tooling, dependency, CI step or fixture.** The audit is a read of ~29 atom frontmatters and one queue table, using commands the project already declares (`_decomposition.md:96-101`, `:109-123`) | A project whose deliverable is re-observation must not grow an instrument only it uses; `_decomposition.md:121-123` — nothing is mocked, and nothing new is built to check what already exists |
| **NF-006** | **Finding ids are stable and unique across the project's record**, so `findings-disposition-register` can address each one | AC-008's routability check; ids are referenced from both the table cells and the findings list |

## Implementation notes (non-prescriptive)

Shape suggestions only. Anything here loses to an AC row, to the testing brief's fixed row shape
(`_decomposition.md:135-137`), or to `.kb/decisions/README.md`.

- **Build the two listings first, mechanically, and paste both into the ledger evidence.** The
  directory side is a listing of `.kb/decisions/*.md`; the queue side is the rows at
  `RUNBOOK.md:284-307`. Union them by four-digit number *before* writing any prose — a table written
  atom-first drifts toward the wrong implementation because the numbers nobody wrote are exactly the
  ones that do not announce themselves. At planning time that union is `0001`–`0029` with `0017`–`0028`
  absent; `0001`–`0007` predate this initiative and are legitimately rows whose *Settled question*
  cell says so.
- **Read `status:` and `superseded_by:` out of frontmatter directly** rather than trusting
  `.kb/maps/decision-map.md`'s summary of them; use the map as the cross-check on the supersession
  graph, which is what it is for (`.kb/maps/decision-map.md:24-34`). The map records that ADR-0002 is
  fully superseded, ADR-0005 and ADR-0006 partly, and ADR-0004 amended by ADR-0029 — a row that
  reports one of those as a plain `accepted` answer is wrong in a way `validate --kb` will not catch.
- **Label the enumeration items** with something short and stable the slice-mate can cite (`SQ-01`,
  `SQ-02`, …) and the findings with their own series (`F-01`, …). The labels are the interface between
  the two stories in this slice; renaming them after `open-question-preservation-audit` cites them
  breaks its record.
- **Take DT-1…DT-8 one at a time** (`initiative.md:420-427`) and, for each, name the project's
  `_design.md` that resolved it and quote the resolving sentence. This is the enumeration most likely
  to be skipped and, per the Context pack, the one most likely to find something.
- **ADR-0029 is the standing proof the queue is not authoritative about itself** — it is on disk,
  accepted, amends ADR-0004, and the queue lists it as unscheduled (`RUNBOOK.md:288`). If the finished
  table has no row for it, the enumeration ran in one direction only.
- **Write findings as you go, not at the end.** A finding discovered mid-table and remembered for
  later is the pressure point the project's own risk table names; the cheapest defence is that the
  *Finding* column is filled in the same pass as the row it belongs to.

## Tests and CI (merge gate)

Grounded in the testing brief's AC-005 row (`_decomposition.md:48`) and its merge-gate command block
(`_decomposition.md:96-107`). All four project-level commands run against the **clean-checkout tree
slice 1 produced**, not the working tree these artefacts are written in (`_decomposition.md:107`).

| Tier | Command / path | Proves |
| ---- | -------------- | ------ |
| **Static** | `redkiln validate --kb` | Every cited atom carries valid `KbFrontmatter` and no accepted body has drifted from `HEAD` (`.kb/decisions/README.md:9-13`) — the precondition that makes this audit's frontmatter reads meaningful. **AC-002, AC-008**; exit code cited in `_ledger.md` |
| **Static** | `ls .kb/decisions/*.md` × the queue rows at `RUNBOOK.md:284-307` | Union completeness by number in both directions — the manual cross-reference the testing brief names as AC-005's proof (`_decomposition.md:48`). **AC-001** |
| **Static** | Citation-resolution sweep over `_closeout-record.md` § *Decision-atom audit* — every `file:line` opened, the named alternative present there | The *Alternatives* cell was read out of the atom body, not inferred from the atom's existence. **AC-003**; this is the check the wrong implementation fails (`discover.md:38-40`) |
| **Static** | Reserved-number disposition sweep over the same section against `project.md:148-152` and `_decomposition.md:125-131` | Every unwritten reserved number carries a reason or a finding, never a blank. **AC-004** |
| **Static** | Coverage computation shown in the section: the four input lists and their set differences, incl. one line per DT-1…DT-8 (`initiative.md:420-427`) | Coverage was computed rather than asserted — the `RUNBOOK.md:309-320` precedent. **AC-005** |
| **Process** | `git diff --stat -- .kb/` over this story's commit range (empty) | The audit mutated nothing it audited. **AC-008** |
| **Process** | `git diff -- .bklg/.../closeout-and-durable-audience/_closeout-record.md` — additions only | Mounted in place without occluding slices 1–2's sections. **AC-007** |
| **Process** | Slice-mate cross-citation: `open-question-preservation-audit`'s record cites the `SQ-##` labels verbatim | The enumeration is a consumable artefact, not a private working list. **AC-006** |
| **Story grain** | `cargo xtask affected --base main` (`.redkiln/config.yaml:40`) | The story-grain gate. This story maps to no workspace package, which is precisely why the command also runs the five file-reading lints and `spec-trace` unconditionally (`.redkiln/config.yaml:36-39`) — a purely package-shaped gate would compile nothing and call it green |
| **Story grain** | `redkiln verify --grain story` with `_ledger.md` (`require_ledger`, `require_commit_provenance` — `.redkiln/config.yaml:67,73`) | Every AC-### has a ledger row with real cited evidence and a work commit; the PR-boundary block is enforced against the changed-file set |
| **Project / terminal grain** | `cargo xtask ci` (`.redkiln/config.yaml:60`) — **not this story's to run** | The tree this story audits is the one slice 1 ran the whole gate on (`_storymap.md:54`). Cited, not re-run here; `--fast` is never substituted (DR-2, `_decomposition.md:103-107`) |

**Merge gate for this story** = the four Static rows + the three Process rows + the two Story-grain
rows, all green, with `git diff --stat -- .kb/` empty.

## Risks and coupling (PR-scoped)

| Risk | Likelihood / impact | Mitigation inside this PR |
| ---- | ------------------- | ------------------------- |
| **The mount is not there.** `_closeout-record.md` is stood up by slice 1, which merges first but is not a declared `depends_on` edge | Low / blocking | EC-001: halt with a blocking finding against slice 1, never invent a landing place. The merge-order precondition is stated in *Dependencies* below so it cannot be discovered at implementation time |
| **Pressure to fix what the audit finds** — the project's own named risk (`project.md` risk table; `_storymap.md:92-94`) | Medium / fatal to the AC | AC-008 makes "zero changes under `.kb/`" observable rather than aspirational; EC-007 says revert, do not justify. The routing is a *sibling's* deliverable (AC-013), so the finding has somewhere to go |
| **Many reserved numbers still unwritten at execution time**, turning the table into a wall of findings | Medium / scope pressure | This is the correct outcome, not a defect of the audit: `RUNBOOK.md:276-283` makes an empty reserved number legitimate, and EC-005 gives the one mechanical discriminator that turns it into a finding. The story does not grow to write them |
| **The coverage computation is performed as a claim** ("all settled questions have atoms") | Medium / silent failure | AC-005 requires the four input lists and the set difference to be *shown*; a conclusion with no shown difference fails. Phase 4 is the precedent that this exact failure survives a gate (`RUNBOOK.md:309-320`) |
| **Label churn breaks the slice-mate.** `open-question-preservation-audit` cites `SQ-##` labels from this section | Medium / rework in the same slice | Labels are fixed in the same context (both stories are implemented together, `_storymap.md:78-80`); AC-006 makes the labelled enumeration the interface, so a rename is a visible contract change rather than a silent one |
| **Re-litigating a recorded decision** while auditing it — especially HS-P0016's DT-1 resolution against the amended evaluator decision (`_decomposition.md:150-180`) | Medium / out of scope | The PR boundary forbids it and `_storymap.md:100-106` names it explicitly: inconsistency is a finding, not a re-decision |
| **Auditing a moving tree** — an atom lands between the audit run and the merge | Low / accuracy | NF-003 / EC-009: the audited commit SHA is stated in the section header |

**Coupling**, all read-only except the mount: `.kb/decisions/` and `.kb/maps/decision-map.md` (read),
`RUNBOOK.md` (read), `initiative.md` and `project.md` (read), `_closeout-record.md` (append-only).
No crate, no `spec/SPECIFICATION.md` clause, no conformance rule, no CI job is touched.

## Dependencies

**Blocks-on (declared `depends_on`): none.** `_storymap.md:57` records `—` for this story and
`:147` explains why: "Independent of slices 1–2 in principle; sequenced here so its findings reach
slice 5." Nothing in this story's subject — the state of `.kb/decisions/` — is produced by another
story in this project.

**Merge-order precondition, distinct from a dependency.** The mount point `_closeout-record.md` is
stood up by `clean-checkout-harness` in slice 1 (`_storymap.md:53`), which merges first
(`_storymap.md:140-148`). That is a sequencing fact, not a declared edge, and it is handled by
EC-001 rather than by adding a `depends_on` this story does not have.

**Unlocks**

| Story | Why it waits on this one |
| ----- | ------------------------ |
| `open-question-preservation-audit` (HS-S0130, project AC-006) | Declares `depends_on: decision-atom-audit-table` (`_storymap.md:58`) because it consumes the settled-question enumeration: "the second is meaningless without the first's list" (`_storymap.md:78-80`) |
| `findings-disposition-register` (HS-S0134, project AC-013) | Lists this story among its dependencies (`_storymap.md:62`); it routes the findings this audit produces, and can only show "zero fixed inside this project" once the findings exist unfixed |

**Upstream expectation, not a dependency**: sibling projects HS-P0010…HS-P0018 merge before this
project (rank 6, `../_decomposition.md` *Merge order*) and own ADR numbers `0017`–`0028`
(`_decomposition.md:125-131`). If they did not write them, that is the audit's subject matter — the
story executes either way.

## Anchors (progressive disclosure)

Everything above is enough to start. Open these when the bound AC says to — link, do not pre-read.

| Anchor | Why it is load-bearing | When to open | Serves |
| ------ | ---------------------- | ------------ | ------ |
| `RUNBOOK.md` (`:262-307` the queue; `:276-283` the reserved-number precedent; `:309-320` the coverage-audit precedent) | The queue table is one of the two enumeration inputs, and `:276-283` is the *only* authority that makes an empty reserved number legitimate rather than a gap. `:309-320` is the recorded instance of the exact failure AC-005 guards against | Before building the union (AC-001); again before writing any "not needed" reason (AC-004); again before writing the coverage statement (AC-005) | AC-001, AC-004, AC-005 |
| `.kb/decisions/README.md` (`:7-18` immutability; `:25-33` what belongs and the "state what lost" bar) | `:31-33` *is* the bar AC-003 applies ("a decision recorded without its rejected options is indistinguishable from an accident"); `:7-18` is why a gap is never repaired here; `:25-29` is the test EC-006 applies to a DT resolution | Before writing the first *Alternatives* cell (AC-003); before touching anything on discovering a gap (AC-008) | AC-002, AC-003, AC-008 |
| `.kb/decisions/` (the atoms themselves, `0001`–`0016`, `0029`) | The directory listing is the second enumeration input, and each atom's frontmatter is the *only* acceptable source for its `status` and each body the only acceptable source for its rejected alternatives | Per row, as the row is written — never in bulk up front | AC-001, AC-002, AC-003 |
| `.kb/maps/decision-map.md` (`:24-34` and the supersession rows) | The whole supersession graph at a glance: ADR-0002 fully superseded, ADR-0005/0006 partly, ADR-0004 amended by ADR-0029. A row calling a partly-superseded atom a plain answer is wrong in a way `validate --kb` cannot catch | When filling the *Status* column, as the cross-check on frontmatter (AC-002) | AC-002 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_decomposition.md` (`:48` the AC-005 tier; `:125-146` DR-6 audit procedure and the fixed row shape; `:96-107` merge-gate commands) | Fixes the four columns this story may not redesign, names the tier that proves AC-005, and states that all commands run on the clean-checkout tree | Before drawing the table (AC-002); before running any command (all) | AC-002, AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/project.md` (`:148-152` DR-6; `:153-161` DR-7's six named atoms; `:179-183` DR-12; `:207-210` AC-005) | The charter text of the AC this story traces to, the owning-sibling mapping for reserved numbers, the six consumed open questions that form coverage input (c), and the routing rule that forbids absorbing findings | Before the reserved-number list (AC-004); before the coverage computation (AC-005); before routing anything (AC-008) | AC-004, AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/initiative.md` (`:298` BR-15; `:420-427` DT-1…DT-8) | DT-1…DT-8 is coverage input (d) — the enumeration most likely to be skipped and most likely to find something, since a `_design.md` carries no immutability | Immediately before the coverage computation (AC-005) | AC-005 |
| `.kb/maps/open-questions-index.md` (`:11-13` the annotation convention) | Coverage input (c) is cross-read here; the index's standing rule is that a resolved question stays listed and annotated, so absence means deletion, not resolution — and the slice-mate's whole check rests on the same convention | While assembling the settled-question enumeration (AC-005, AC-006) | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_storymap.md` (`:17-22` the reader; `:26-30` the convergence rule; `:57-58`, `:78-80` the slice; `:92-94` no story fixes anything) | Fixes the mount, names the reader every row is written for, and states the slice contract with `open-question-preservation-audit` | Before mounting (AC-007); before labelling the enumeration (AC-006) | AC-006, AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/decision-atom-audit-table/discover.md` (`:38-40` the wrong implementation; `:30` the reserved-number discriminator) | The named wrong implementation is the acceptance test for AC-003 in prose form — a table that satisfies AC-001 and AC-002 and still fails | Before declaring the table finished (AC-003, AC-004) | AC-003, AC-004 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_design.md` (`:41-49` `## Items` N/A; `:93-100` the sign-off) | The signed-off design binds as a prohibition: no surface is invented, and the perceptual review is a *declared* skip. It is what makes the Interaction-quality N/A rows legitimate rather than omissions | Before writing anything that looks like a surface (AC-007) | AC-007 |
| `.bklg/from-contract-to-published-library/closeout-and-durable-audience/_grounding.md` (`:20-27`) | The planning-time snapshot — seventeen atoms on disk, `0017`–`0028` reserved and unwritten — which is the baseline the execution-time listing is compared against, and the source of the ≥29-row density number | When the union comes out smaller than expected (AC-001) | AC-001 |
| `.redkiln/config.yaml` (`:36-40` the affected gate; `:67`, `:73` ledger and commit provenance) | Explains why the story-grain gate is meaningful for a story that maps to no package, and why `_ledger.md` plus a work commit are mandatory rather than customary | Before opening the PR (AC-008) | AC-008 |

## Clarifications resolved during spec

1. **How many AC-### this story carries: eight.** The front half's behaviour table has eleven rows;
   several are two statements of one obligation (row shape and status provenance; mounting and the
   record's composition). They are folded into AC-001…AC-008 with no obligation dropped — the
   "no new fixture, no new test, no mock" row (`_decomposition.md:121-123`) becomes NF-005 rather
   than an AC, because it is a property of *how* the story is done, not something a reader observes
   in the record. The ledger carries exactly these eight ids.
2. **Discover's first deferred question — are `0017`–`0028` written by execution time?** Resolved as
   a procedure rather than a prediction: the table is populated against the tree at execution time
   (its SHA stated per NF-003), and the discriminator between "legitimately empty" and "finding" is
   the mechanical one in EC-005, not a judgement about whether the sibling should have written it.
3. **Discover's second deferred question — does the union of what the written ADRs cover equal what
   this initiative raised?** Resolved as AC-005's shown computation over four named enumerations,
   with the per-DT line spelled out. The queue's parenthesised clause ranges are treated as a scope
   statement and never as a coverage guarantee (`RUNBOOK.md:309-320`).
4. **`0001`–`0007` are rows.** The bidirectional rule unions the queue (`0008`–`0028`) with the
   directory listing, and the directory holds `0001`–`0007` from before this initiative. They are
   not silently dropped for being out of the queue's range; their *Settled question* cell says they
   predate this initiative, and their *Status* is still read rather than assumed — three of them
   carry supersessions (`.kb/maps/decision-map.md`). This is what fixes the density number at **no
   fewer than 29 rows**.
5. **The signed-off `_design.md` binds as a prohibition, and the interaction-quality N/As are
   declared.** With `## Items` recorded as N/A (`_design.md:41-45`) and `design.capture` deliberately
   absent (`.redkiln/config.yaml:75-83`), focus/scroll/selection/keyboard invariants have nothing to
   apply to. They are written into the Interaction-quality section as explicit N/A rows rather than
   omitted, because an omitted invariant and an unrendered surface are indistinguishable — the same
   reason the config file gives for declaring gates it does not wire.
6. **The composition invariants come from the brief and the story map, not from the design.** Since
   `_design.md` records no surface, the binding composition rules for the artefact this story *does*
   render are the fixed four-column row shape (`_decomposition.md:135-137`) and the single-record
   convergence rule (`_storymap.md:26-30`). Both are cited on the AC rows that carry them.
7. **No project AC beyond AC-005 is claimed.** AC-006's preservation check belongs to the slice-mate;
   this story's AC-006 (the labelled enumeration) exists to make that check possible and does not
   discharge it.
