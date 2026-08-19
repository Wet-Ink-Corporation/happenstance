---
item: HS-S0088
stage: spec
created: 2026-08-12T13:47:26.282Z
updated: 2026-08-12T13:47:26.282Z
template_sig: 87bbf1d0
rendered_sig: 42e9f503
---

# Spec — The falsifier ledger is repaired before anything reads it

## Scope lock

| | Path |
| --- | --- |
| Initiative (gold source) | `.bklg/from-contract-to-published-library/initiative.md` — DoD 12 at `:393-395` |
| Initiative decomposition | `.bklg/from-contract-to-published-library/_decomposition.md` |
| Project | `.bklg/from-contract-to-published-library/publication-and-positioning/project.md` — DR-4 at `:170-173`, AC-004 at `:238-242` |
| This spec | `.bklg/from-contract-to-published-library/publication-and-positioning/falsifier-ledger-repair/spec.md` |
| Key briefs | `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` — **testing brief** AC-004 row at `:467`, CI implication at `:708-714`; **deployment brief** at `:685-699`. No `architecture` brief exists for this project, deliberately (`project.md`:72-77) |
| Signed-off design | `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` — approved 2026-08-12 (`:772-783`). **Binds no surface this story renders** (see Integration contract), but its DT-5 census sentence (`:284-318`) is downstream of this table |
| Story map / merge order | `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` — this row at `:60`, milestone at `:161-165`, the ordering constraint that must not be re-planned at `:179-184` |
| Roadmap pointer | `RUNBOOK.md`:163 (phase 12's row); the phase-12 exit criterion that reads this table at `RUNBOOK.md`:4492-4497 |

## One-line PR slice

Repair the five known-short rows in `RUNBOOK.md`'s provisional-clause ledger — add ES-41, ES-42,
CF-39 and CF-40 each with a deliberately chosen falsifier and an owning phase, remove ES-10's stale
row without deleting the disclosure it carried — as its own change, landed before anything reads the
table.

## Executive summary

**What lands.** `RUNBOOK.md`'s `### The 49 [PROVISIONAL] clauses` ledger (`RUNBOOK.md`:580-635) is
made true. Today its heading says 49 and its rows describe 46 clauses, four of which are the wrong
46: ES-41, ES-42, CF-39 and CF-40 have no row at all, and ES-10 still occupies one although phase 4
froze it (`spec/SPECIFICATION.md`:2816-2822). This PR adds four rows, removes one clause from the
residual-exposure row, rewrites the narrative that currently records the defect as *outstanding* into
one that records it as *repaired*, and fixes the dead intra-document anchor that phase 12's own exit
criterion uses to reach the table (`RUNBOOK.md`:4494-4495 still points at `#the-46-provisional-clauses`).

**Delta against what the project card already says.** `project.md`'s DR-4 (`:170-173`) states the five
edits. This spec adds the four things DR-4 does not decide, each of which is a live way to get the
repair wrong:

1. **Where each falsifier comes from.** Not invented here. Each new row's falsifier is its own
   clause's `[PROVISIONAL — …]` marker, distilled — because a ledger row that says something the
   clause does not is a second, competing statement of the same obligation, which is the defect
   `spec/SPECIFICATION.md`:1056-1068 (VT-12) already names and demotes a clause over.
2. **That removing ES-10 is not a deletion.** ES-10's own frozen note (`:2823-2831`) says lifting its
   `[PROVISIONAL]` marker moved the position-allocation axis *into* ADR-0013's CF-25 acceptance —
   "lifting it deletes the disclosure, and the acceptance has to move in the same act or the exposure
   disappears silently". Striking the ID from the ledger row without also correcting the row's
   *five*-clause framing repeats that failure one level out.
3. **That ES-42's owning phase is 12 — this one.** `spec/SPECIFICATION.md`:9028 records ES-42 as
   "the only one of the five that expires": its marker "must be re-evaluated before phase 12" because
   adding a bound to an opaque return type after publish is breaking. It is `[PROVISIONAL]`, not
   `[DEFERRED]`, so `deferred-clause-reread` does not cover it and no other story in this project
   does either. The row must therefore be written as a **handoff that is visible**, not as a schedule
   into a phase that will never come.
4. **That the table's shape is a contract with the next story.** `clause-maturity-audit` will assert
   set-equality between this table's clause IDs and `spec-trace`'s parsed `[PROVISIONAL]` list
   (`_storymap.md`:61, `_decomposition.md`:467). Rewriting the table into a prettier form is how that
   check gets written against a shape that no longer exists.

**What does not land.** No gate step, no `xtask` code, no edit to `spec/SPECIFICATION.md`. This is the
content half of AC-004; the mechanised half is `clause-maturity-audit`'s, and the split is deliberate
(see PR boundary).

## Context pack

Everything needed to start. Deeper material is behind the anchors, not repeated here.

**Decision 1 — the ledger is repaired *before* the audit exists, and this ordering may not be
re-planned.** `_storymap.md`:179-184 pins `falsifier-ledger-repair` strictly before
`clause-maturity-audit` (DR-4 before DR-3), and `project.md`'s risk register (`:344`) names the reason
as its first row: *the falsifier ledger is repaired during the audit rather than before it, and a
row's falsifier is chosen to make the audit pass*. `RUNBOOK.md`:626-627 says the same thing in the
runbook's own voice — adding a row means deciding a falsifier and an owning phase, and that *"is not
a thing to do in passing."* The practical consequence for the implementer: at the end of this PR the
ledger is correct and **nothing checks that it is**. That is the intended state. Do not add the check
to make the change feel finished.

**Decision 2 — each falsifier is the clause's own marker, distilled; never a fresh judgement.** The
four clauses already carry falsifiers in `spec/SPECIFICATION.md`, and CF-38 already makes an empty one
a build failure (`spec/SPECIFICATION.md`:213-217). The ledger's job is different and narrower: it
*groups* clauses by what falsifies them and *schedules* each group into an owning phase, because
"phase 12 cannot audit 'every provisional clause has its falsifier scheduled' against prose"
(`RUNBOOK.md`:582-586). So the row's third column is a distillation of the clause's marker and the
fourth column is the scheduling decision. If, while distilling, the clause's marker looks wrong, that
is a finding to record — not a row to write differently. A row that disagrees with its clause creates
exactly the two-copies-drift failure `spec/SPECIFICATION.md`:1056-1068 demoted VT-12 to prose over.

**Decision 3 — one row per distinct falsifier, and grouping is the ledger's whole argument.** The
table groups "because they do not fail independently — seventeen of the `PS` rows wait on the same
missing adapter, and counting them as seventeen open questions overstates the exposure by a factor of
seventeen" (`RUNBOOK.md`:583-586). Folding two clauses into one row is correct only when **one**
falsifier settles both — the way PS-2 alone settles thirteen rows (`RUNBOOK.md`:608-609). CF-39 and
CF-40 are adjacent in the specification and are both about what a *fixture* promises, and they are
still two rows: CF-39 is falsified by a driver that absorbs an injected fault, CF-40 by a ceiling that
is not a constant. Different falsifiers, different phase sets, two rows.

**Decision 4 — removing ES-10 moves a disclosure; it does not delete one.** The residual-exposure row
(`RUNBOOK.md`:606) is framed as *"the five clauses §1.3 names as carrying CF-25's risk in their own
markers"*. The specification says **four** (`:259-264`, and again at `:371`): ES-11, ES-12, ES-35,
ES-40 — *"ES-10 was the fifth until phase 4 froze it"*. So the repair is three coordinated edits to
one row: the clause list loses ES-10, the framing goes from five to four, and the "falsified by" cell
loses its position-allocation sentence. The axis itself stays on §6.5's portfolio table with a fixture
at its far end — what changed is only **where the disclosure lives**, and the row must say so, because
a reader who sees position allocation vanish from the ledger and does not know it landed in
ADR-0013's CF-25 acceptance will read the exposure as closed. The owning-phase cell drops ES-10 too:
`10 (ES-10, ES-11, ES-12)` becomes `10 (ES-11, ES-12)`.

**Decision 5 — the arithmetic is the acceptance test, and it is 46 − 1 + 4 = 49.** Counted at HEAD the
existing rows describe exactly 46 clause IDs; the specification's census says 49
(`spec/SPECIFICATION.md`:219-221, and §7.1's totals row at `:8138`). After the repair the two sets are
*equal*, not merely equinumerous — this is a set identity, and counting to 49 with the wrong 49 is the
precise failure the table's own history records (`RUNBOOK.md`:612-620: *"The heading was right; the
rows were short."*). Verify by set comparison against `cargo xtask spec-trace`'s parsed
`[PROVISIONAL]` list, not by counting.

**Decision 6 — the table's grammar is frozen by the next story's parser.** `clause-maturity-audit`
composes with `xtask/src/spec_trace.rs`'s existing parser rather than writing a second one
(`_decomposition.md`:711-714). This PR therefore preserves, exactly: the four-column header
`| Group | Clauses | Falsified by | Owning phase |`; the clause cell as comma-separated IDs where each
ID matches `[A-Z]{2}-\d+`; and the existing range spelling — an en dash with spaces on both sides, as
in `VT-21 – VT-24`, `PS-4 – PS-6`, `PS-22 – PS-25`. New rows use plain comma-separated IDs (all four
additions are single-clause rows, so no range is introduced). Any other formatting instinct is
deferred to `clause-maturity-audit`, which can then change shape and check in one act.

**Decision 7 — the narrative becomes a record, not a to-do.** `RUNBOOK.md`:622-635 currently reads
*"That is five edits and it is the next pass's, not this one's."* This PR **is** that pass. Left
as-is, the paragraph tells the phase-12 auditor to do work that is already done, and a runbook that
lies about its own state is the same defect the paragraph is complaining about. It is rewritten to
record what was repaired, against what, and when — and it keeps the phase-3/phase-5 history, because
that history is why the table is grouped and audited rather than trusted.

**Decision 8 — the mount is the reference, not just the table.** `RUNBOOK.md`:4494-4495, phase 12's
own exit criterion, links *"the provisional ledger"* at `#the-46-provisional-clauses`. The heading was
corrected to 49 and that anchor has been dead ever since — the one criterion whose entire instruction
is *"audit it against the ledger … not against prose"* cannot reach the ledger. Repairing the rows and
leaving the pointer dead delivers a table nothing arrives at. `references/adr/0016-the-wire-format.md`
:1432-1436 is the record that spotted the heading/row disagreement while walking past it; it is
evidence, not an obligation to edit — accepted ADR records are not amended by this story.

**Persona-journey slice.** No end user meets this file. The reader on the other side is the
**maintainer at the publish gate** — `_storymap.md`:40-43 marks A1/A2 as having no user intent of
their own, *"which is exactly why every story in them is either a foundation or a gate instrument
consumed by A3 and A4."* The user-visible consequence is one hop downstream and it is real: DT-5's
published census sentence takes its four numbers from the AC-003 audit *at the publish commit*
(`_design.md`:310-316), and that audit reads this table. A ledger that disagrees with the
specification becomes a wrong number on a page that cannot be edited after publish.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed inside this project by
  `clause-maturity-audit` (`_storymap.md`:108). Not a double and not a fixme.
- **Slice / milestone**: `publish-time-gate-instruments`. Slice-mates, implemented in one context:
  `clause-maturity-audit`, `deferred-clause-reread`, `registry-surface-diff`. **This story is first in
  the slice and has no blockers at all** (`_storymap.md`:162) — it may run alongside milestone 1.
- **Mount point**: **`RUNBOOK.md` § `The 49 [PROVISIONAL] clauses` — the ledger table at
  `RUNBOOK.md`:588-606, inside the section spanning `:580-635`.** This is the render path, not a
  staging area: it is the single table phase 12's exit criterion is instructed to audit against
  (`RUNBOOK.md`:4492-4497) and the single table `clause-maturity-audit` will parse. The repair is
  mounted when that criterion's link resolves to the repaired section and the section's rows are the
  49 the specification names — not when the rows exist somewhere.
- **Wires into** (consumed, none modified):
  - `spec/SPECIFICATION.md` — the source of every falsifier distilled here: ES-41 at `:4381-4420`,
    ES-42 at `:3079-3088`, CF-39 at `:7631-7642`, CF-40 at `:7661-7684`, ES-10's frozen note at
    `:2816-2831`, the census at `:219-221` and §7.1's totals row at `:8138`. **Read-only in this PR.**
  - `xtask/src/spec_trace.rs` — `parse_clauses` and the `[PROVISIONAL]` list it yields are the set
    this table must equal. Run it; do not change it. Its module doc (`:39-57`) is the standing reason
    §1.3 stays hand-computed, which is what makes the reconciliation mean anything.
  - `RUNBOOK.md`:4492-4497 — phase 12's exit criterion, the inbound reference that must resolve.
  - `.kb/decisions/0013-position-assignment-and-visibility.md` — the CF-25 acceptance that ES-10's
    disclosure moved into. Cited by the repaired residual row; **not edited** (accepted atoms are
    immutable, `CLAUDE.md`).
  - `.kb/decisions/0010-the-suite-must-prove-itself.md` — the *"a skip is reported, never silent"*
    discipline this repair applies to prose: a clause with no row is a skip nobody reported.
- **Renders surfaces**: **none.** `RUNBOOK.md` is not a packaged surface — it is not any crate's
  `readme =`, it is not reachable by `include_str!` from inside a package
  (`_design.md`:543-553), and it appears in none of `_design.md`'s seven surface ids (`:113-169`).
  The signed-off design is therefore not re-decided here and is not contradicted. The one link is
  downstream and one-way: DT-5's census sentence (`_design.md`:284-318) is sourced from the audit that
  reads this table.
- **Conformance rule(s)**: **none — not adapter-observable, and deliberately so.** This story changes
  no port, no type and no rule; it changes the schedule that says who will falsify a clause. The
  conformance suite cannot observe a runbook. What *will* observe it is `clause-maturity-audit`'s
  set-equality check, and building that check here is what DR-4's ordering forbids.
- **Clause(s)**: **none discharged and none amended.** Every clause named — ES-41, ES-42, CF-39,
  CF-40, ES-10 — is read, not written. `[FROZEN]` clauses are untouched (AC-015 is a property of the
  whole project's diff; this story's contribution to it is that `spec/SPECIFICATION.md` is outside its
  PR boundary entirely).
- **Advances DoD scenario**: initiative **DoD 12** — *"The clause ledger is audited at publish"*
  (`initiative.md`:393-395) — and project **AC-004** (`project.md`:238-242). It moves DoD 12 from
  un-auditable to auditable: the audit DoD 12 requires is `clause-maturity-audit`'s, and until this
  table equals the specification's `[PROVISIONAL]` set that audit has nothing true to reconcile
  against.

## PR boundary

`redkiln verify --grain story` reads the first fenced block below and fails on any file changed
outside it.

```
RUNBOOK.md
.bklg/from-contract-to-published-library/publication-and-positioning/falsifier-ledger-repair/**
```

**Two files, and the narrowness is the point.** `spec/SPECIFICATION.md` is deliberately outside the
boundary even though this story reads it end to end: the moment a ledger repair can edit a clause, the
cheapest way to make the sets equal is to edit the specification, and that is the AC-015 failure mode
this project's whole shape exists to prevent (`project.md`:281-283, `:350`). `xtask/**` is outside it
because the check belongs to the next story. If the implementer finds a reason to widen this, widen it
in the spec deliberately or split the story — do not widen it to make a gate pass.

**In this PR**

- Four new rows in the ledger table (`RUNBOOK.md`:588-606) for ES-41, ES-42, CF-39 and CF-40, each
  with a group name, its clause, its falsifier distilled from the clause's own marker, and an owning
  phase.
- The residual-exposure row (`RUNBOOK.md`:606) corrected: ES-10 removed from the clause cell, the
  "five clauses" framing corrected to four, the position-allocation sentence removed from the
  "falsified by" cell and replaced by a pointer to where that disclosure now lives, and the
  owning-phase cell corrected to `10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40)`.
- The narrative at `RUNBOOK.md`:612-635 rewritten from an outstanding-work note into a repair record,
  preserving the phase-3 and phase-5 history that explains why the table is grouped.
- The dead anchor at `RUNBOOK.md`:4495 repointed to the live heading, and any other in-tree reference
  to the ledger heading confirmed live.
- This story's own backlog folder: `_ledger.md`, and the implementation report the stage renders.

**Explicitly not in this PR**

- **The set-equality check** between the ledger's clause IDs and `spec-trace`'s parsed `[PROVISIONAL]`
  list. That is `clause-maturity-audit` (`_storymap.md`:61, `_decomposition.md`:467, `:708-714`), it
  is the other half of AC-004, and building it here collapses DR-4 into DR-3.
- **Any edit to `spec/SPECIFICATION.md`** — clause text, marker, falsifier, §1.3's census, or the
  generated §7.1/§7.2 regions.
- **Any edit to `xtask/`**, including `spec_trace.rs`, and any change to `xtask/src/main.rs`'s
  Mandatory list or its module doc (`:8-24`) — both belong to the slice-mate that adds a step
  (`_decomposition.md`:685-699).
- **Re-evaluating ES-42's marker.** The row records that phase 12 owns it; the re-evaluation is a
  decision, and under `project.md`'s AC-013 a settled answer is a `/redkiln:kb-ingest` decision atom.
  Settling it inside a table edit is the "resolved in passing" risk `project.md`:351 names.
- **Anything about the ten `[DEFERRED]` clauses** — `deferred-clause-reread` owns those.
- **Amending `references/adr/0016-the-wire-format.md`** or any accepted decision atom.

**Merge DoD.** The ledger's clause-ID set equals `cargo xtask spec-trace`'s `[PROVISIONAL]` list at
49; `cargo xtask affected --base main` is green; `git diff --stat` names exactly two paths.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **ES-41 gains a row** | Group: membership as a port operation. Falsifier, distilled from the clause's own marker: an adapter that cannot answer membership without a structure VT-8 does not already oblige it to keep — the exposed axes are **transport** (one more round trip on a store with no connection and no cursor) and **completeness** (a store that cannot state what it does not hold cannot distinguish "no such event" from "not visible to me"). Owning phase: **8** (`happenstance-sqlite`) or **9** (`happenstance-cloudflare`), whichever lands first | `spec/SPECIFICATION.md`:4381-4394 |
| **ES-42 gains a row, and it is the row that expires** | Group: `read`'s return type carries no `Unpin` bound. Falsifier: a consumer needing `dyn EventStore` where the port has acquired a method the hand-written `Pin<Box<…>>` wrapper cannot box — a generic method, or a return whose lifetime the wrapper cannot name; **inconvenience is not the falsifier**. Owning phase: **12 — re-evaluated at this publish rather than scheduled past it**, because adding a bound to an opaque return type after publish is breaking. The cell must say *expires*, not merely *12* | `spec/SPECIFICATION.md`:3083-3088, and §7.5's row at `:9028` |
| **CF-39 gains a row** | Group: a fixture's mid-batch fault promise. Falsifier: a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — which would make *"the append returns `Err`"* a promise no fixture over that adapter can keep. Owning phase: **8** (rusqlite), **10** (Postgres); no adapter has armed a fault yet | `spec/SPECIFICATION.md`:7631-7642 |
| **CF-40 gains a row** | Group: a fixture's stated capacity ceilings. Falsifier: a real adapter whose ceiling is not a constant — a Postgres row whose TOAST threshold depends on what else is in the row, or a KV store whose per-value cap varies with the key — making a single `Option<usize>` unable to say where the boundary is. Owning phase: **8**, **9**, **10**; no adapter has stated a ceiling yet | `spec/SPECIFICATION.md`:7661-7684 |
| **ES-10 leaves the residual-exposure row without taking its disclosure** | Three coordinated edits to one row: clause cell → `ES-11, ES-12, ES-35, ES-40`; framing → *four* clauses, matching `spec/SPECIFICATION.md`:259-264 and `:371`; "falsified by" cell → position-allocation sentence removed and replaced by a statement that the axis is now disclosed by ADR-0013's CF-25 acceptance rather than by a ledger row. Phase cell → `10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40)`. CF-26's *"a fixture instrument does not falsify any of them"* sentence stays — it is still true of the four that remain | `spec/SPECIFICATION.md`:2823-2831 (the "or the exposure disappears silently" instruction); `RUNBOOK.md`:606 |
| **The set identity holds at 49** | 46 rows' worth of clause IDs at HEAD, − ES-10, + four = 49. Checked as a **set** against `cargo xtask spec-trace`'s parsed `[PROVISIONAL]` list, not by counting to 49 — "the heading was right; the rows were short" is what counting produces | `RUNBOOK.md`:612-628; `spec/SPECIFICATION.md`:219-221, `:8138` |
| **Table grammar is preserved verbatim** | Header stays `| Group | Clauses | Falsified by | Owning phase |`; clause IDs stay comma-separated and match `[A-Z]{2}-\d+`; existing ranges keep their spaced en dash (`VT-21 – VT-24`). New rows are single-clause, so no new range form is introduced. Row order: the four additions sit with their families — the two `ES` rows near the existing `ES` rows, the two `CF` rows near `CF-17, CF-34` — and the residual-exposure row stays last, where its bolded framing is read as a summary | `_decomposition.md`:711-714; `RUNBOOK.md`:588-606 |
| **The narrative becomes a repair record** | `RUNBOOK.md`:622-635 rewritten: what was short, what was added, what was lifted, the arithmetic, the commit/date of the repair, and the standing fact that the set-equality check is `clause-maturity-audit`'s and does not exist yet. `:612-620`'s phase-3 history is preserved — it is the argument for why the table is audited rather than trusted | `RUNBOOK.md`:612-635 |
| **Phase 12's exit criterion reaches the table** | `RUNBOOK.md`:4494-4495's link target `#the-46-provisional-clauses` is dead against the heading `### The 49 [PROVISIONAL] clauses`. Repoint it to the live slug; then confirm no other in-tree reference to either slug is left dead — the single known occurrence is `RUNBOOK.md`:4495 | `RUNBOOK.md`:4492-4497, `:580` |
| **No falsifier is invented, and a disagreement is a finding** | Every third-column cell is a distillation of its clause's own `[PROVISIONAL — …]` marker. If distilling surfaces a marker that looks wrong, record it in `_ledger.md` and stop — do not write a row that disagrees with its clause. Two statements of one obligation drifting apart is the failure that demoted VT-12 to non-normative prose | `spec/SPECIFICATION.md`:1056-1068; `RUNBOOK.md`:626-627 |
| **Nothing normative moves** | `spec/SPECIFICATION.md` is unchanged byte-for-byte, so §1.3's census, §7.1/§7.2's generated regions and every `[FROZEN]` clause are untouched, and `cargo xtask spec-trace` must be green **before and after** with identical output. This is what keeps a table edit from becoming a specification edit | `xtask/src/spec_trace.rs`:39-57; `project.md`:281-283 |

## Data and migrations

**N/A — no schema, no store, no persisted state.** This story's entire diff is Markdown prose in
`RUNBOOK.md` plus this story's own backlog folder. There is no database, no serialised format and no
event payload in the change; the `[PROVISIONAL]` clause set it reconciles against is derived at run
time by `xtask/src/spec_trace.rs` from `spec/SPECIFICATION.md` and is never stored.

The one migration-shaped concern, named so it is not mistaken for absence: **the ledger's row grammar
is a de facto schema for a parser that does not exist yet.** `clause-maturity-audit` will parse this
table (`_decomposition.md`:711-714), so changing the header, the column count or the clause-cell
separators here would be a breaking change to a consumer that cannot yet fail. That is why the shape
is frozen by this spec rather than left to taste — see the grammar row in *Behavior and interfaces*.

## Acceptance criteria

The persona throughout is the **maintainer at the publish gate** — the reader `_storymap.md`:40-43
names as the only one A1/A2 has, arriving at phase 12's exit criteria with one question per clause:
*who will falsify this, and when?* The consequence one hop downstream is the evaluator's, through
DT-5's published census sentence (`_design.md`:284-318), which takes its numbers from the audit that
reads this table. Every criterion below is stated as that reader's goal crossing the whole path —
criterion → ledger row → clause → phase — not as a cell that exists.

| id | criterion | verification |
| --- | --- | --- |
| **AC-001** | **GIVEN** the maintainer at the publish gate must answer *"who falsifies ES-41, and when?"* without opening a 9,000-line specification (IQ-1's 0-hops-to-a-claim budget, `_decomposition.md`:232-239), **WHEN** they read `RUNBOOK.md`'s provisional ledger, **THEN** a row exists whose `Clauses` cell is exactly `ES-41`, whose `Falsified by` cell is a distillation of the clause's own marker (`spec/SPECIFICATION.md`:4388-4394) naming **both** exposed axes — transport (one more round trip on a store with no connection and no cursor) and completeness (a store that cannot state what it does not hold cannot distinguish *"no such event"* from *"not visible to me"*) — and the structure VT-8 does not already oblige, and whose `Owning phase` cell names the clause's own instruments: **8 (`happenstance-sqlite`) or 9 (`happenstance-cloudflare`), whichever lands first**; **AND** no word in either cell asserts anything the clause's marker does not | Cell-by-cell diff of the new row at `RUNBOOK.md`:588-606 against `spec/SPECIFICATION.md`:4388-4394, recorded as a two-column distillation transcript committed under this story's folder; `cargo xtask spec-trace` green |
| **AC-002** | **GIVEN** ES-42 is the only one of the four additions whose marker names *this* publish as its deadline — *"must be re-evaluated before phase 12: adding a bound to an opaque return type after publish is breaking, so this clause expires rather than drifts"* (`spec/SPECIFICATION.md`:3083-3088) — and no other story in this project owns it (it is `[PROVISIONAL]`, so `deferred-clause-reread` does not reach it), **WHEN** the maintainer reads its row at the gate, **THEN** the `Owning phase` cell says the clause **expires at 12 — re-evaluated at this publish**, in words, not merely `12`; the `Falsified by` cell carries the marker's own exclusion *"inconvenience is not the falsifier"* alongside the two shapes that do falsify it (a generic method, which is not dyn-compatible; a return whose lifetime the `Pin<Box<…>>` wrapper cannot name); **AND** the row makes the handoff **visible** without performing it — no re-evaluation verdict is written here | Content check that the phase cell contains the word *expires* and the falsifier cell contains the exclusion; `git diff` showing no verdict sentence and no new `.kb/` path (the re-evaluation is a decision atom under `project.md`'s AC-013, out of boundary — EC-006) |
| **AC-003** | **GIVEN** grouping is the ledger's whole argument — folding is correct only when **one** falsifier settles both, the way PS-2 alone settles thirteen rows (`RUNBOOK.md`:583-586, `:608-609`) — and CF-39 and CF-40 are adjacent in the specification and both about what a *fixture* promises, **WHEN** the maintainer reads the `CF` family, **THEN** they find **two** rows, not one: CF-39 falsified by a real adapter whose only injectable mid-batch fault is one its driver transparently absorbs — a connection killed mid-statement behind a reconnect-and-retry pool — owned by **8 (rusqlite) and 10 (Postgres)**; CF-40 falsified by a real adapter whose ceiling is not a constant — a Postgres row whose TOAST threshold depends on what else is in the row, a KV store whose per-value cap varies with the key — owned by **8, 9 and 10**; **AND** each row carries its clause's own standing fact that *no adapter has armed a fault* / *stated a ceiling* yet, so a reader cannot mistake an unexercised promise for an exercised one | Two rows present with distinct `Falsified by` and distinct `Owning phase` cells, diffed against `spec/SPECIFICATION.md`:7631-7642 and `:7678-7684`; row count of the table goes 17 → 21 (AC-006's density assertion) |
| **AC-004** | **GIVEN** ES-10's own frozen note instructs that *"lifting it deletes the disclosure, and the acceptance has to move in the same act or the exposure disappears silently"* (`spec/SPECIFICATION.md`:2823-2831), so a reader who watches position allocation vanish from the ledger and does not know where it went reads the exposure as **closed**, **WHEN** the residual-exposure row is repaired, **THEN** four coordinated edits land in that one row and none of them alone: the `Clauses` cell becomes `ES-11, ES-12, ES-35, ES-40`; the bolded framing goes from *five* to *four*, matching `spec/SPECIFICATION.md`:259-264 and `:371`; the `Falsified by` cell loses its position-allocation sentence and gains, in its place, a statement that the axis is now disclosed by ADR-0013's CF-25 acceptance — one hop, to the specific atom `.kb/decisions/0013-position-assignment-and-visibility.md`, never a repository root (IQ-1); and the `Owning phase` cell becomes `10 (ES-11, ES-12), 8 (ES-35), 14 (ES-40)`; **AND** CF-26's *"a **fixture** instrument does not falsify any of them"* sentence survives verbatim, because it is still true of the four that remain | `git diff` of `RUNBOOK.md`:606 showing all four edits in one hunk; a reviewer reading only the repaired row can name where the position-allocation disclosure went; link resolution of the cited atom path (`test -f`) |
| **AC-005** | **GIVEN** *"the heading was right; the rows were short"* (`RUNBOOK.md`:612-620) is the precise failure counting to 49 produces, **WHEN** the repaired ledger's clause IDs are bucketed by prefix, **THEN** they equal §7.1's `[PROVISIONAL]` column **per prefix, not merely in total** — `VT` 9, `WF` 1, `ES` 9, `PS` 17, `SY` 9, `CF` 4, total **49** (`spec/SPECIFICATION.md`:8511-8519) — which no compensating pair of errors can pass; the arithmetic is 46 − 1 + 4 = 49 and each term is exhibited; **AND** `cargo xtask spec-trace` is green with byte-identical output before and after the change, and `git diff --stat` shows `spec/SPECIFICATION.md` untouched, so the identity was reached by repairing the ledger and not by moving the specification | Per-prefix reconciliation worksheet committed under this story's folder (six buckets, each with its member IDs listed and its count, against `spec/SPECIFICATION.md`:8513-8518); `cargo xtask spec-trace` run before and after with both transcripts recorded; `git diff --stat` naming exactly two paths |
| **AC-006** | **GIVEN** `clause-maturity-audit` will parse this table by composing with `xtask/src/spec_trace.rs`'s existing parser rather than writing a second one (`_decomposition.md`:711-714), so the table's shape is a contract with a consumer that cannot yet fail, **WHEN** the repair lands, **THEN** the header row is byte-identical (`| Group | Clauses | Falsified by | Owning phase |`), the column count stays four, clause cells stay comma-separated IDs each matching `[A-Z]{2}-\d+`, the existing ranges keep their spaced en dash (`VT-21 – VT-24`, `PS-4 – PS-6`, `PS-22 – PS-25`), the four additions are single-clause rows introducing no new range form, each addition sits with its family (the two `ES` rows near the existing `ES` rows, the two `CF` rows near `CF-17, CF-34`), and the residual-exposure row stays **last**, where its bolded framing reads as the summary it is; **AND** the section heading `### The 49 [PROVISIONAL] clauses` is unchanged, because its slug is the anchor AC-008 repoints to | `git diff RUNBOOK.md` showing no change to the header row, the separator row, the heading, or any existing row other than `:606`; row count 17 → 21; a regex sweep of the repaired table confirming every clause cell parses as comma-separated `[A-Z]{2}-\d+` with the en-dash range form untouched |
| **AC-007** | **GIVEN** the narrative at `RUNBOOK.md`:622-635 currently ends *"That is five edits and it is the next pass's, not this one's"* — and this PR **is** that pass, so leaving it instructs the phase-12 auditor to redo finished work, which is the same class of defect as the short table it complains about (IQ-7, truth at the publish commit), **WHEN** an auditor reads `:612-635` after the change, **THEN** the phase-3 paragraph at `:612-620` survives — it is the argument for why the table is *audited* rather than trusted — and the phase-5 paragraph is replaced by a **dated repair record**: what was short (ES-41, ES-42, CF-39, CF-40), what was lifted (ES-10, and where its disclosure went), the arithmetic 46 − 1 + 4 = 49, the date and commit of the repair, and the standing, explicit fact that **the set-equality check is `clause-maturity-audit`'s and does not exist yet**; **AND** the record is worded so a later pass can supersede it with a further dated entry rather than rewriting it (IQ-4's wording discipline); **AND** the stale citation in that paragraph — `SPECIFICATION.md:8138` for §7.1's totals row, which now lands in §6.6 — is corrected to the live line | `git diff RUNBOOK.md` showing `:612-620` unchanged and `:622-635` replaced; the replacement contains a date, the four added IDs, the lifted ID, the arithmetic, and a sentence naming `clause-maturity-audit` as the missing check; every `file:line` citation inside the rewritten paragraph resolved against the tree at the commit |
| **AC-008** | **GIVEN** phase 12's own exit criterion instructs the auditor to *"audit it against [the provisional ledger] … and `cargo xtask spec-trace`, not against prose"* while linking `#the-46-provisional-clauses`, an anchor dead since the heading was corrected to 49 (`RUNBOOK.md`:4492-4497), so the one criterion whose entire instruction is *reach the ledger* cannot reach it, **WHEN** that criterion is read after this change, **THEN** its link resolves to the live heading; the string `#the-46-provisional-clauses` appears **nowhere** in `RUNBOOK.md`; **AND** every heading anchor `RUNBOOK.md` exposed before the edit still exists (IQ-3 — the repair may not buy one live anchor by breaking another); **AND** the four backlog and grounding files that quote the stale slug as *evidence of staleness* (`clause-maturity-audit/spec.md`:198, `crate-set-decision/spec.md`:82 and `:416`, `crate-set-decision/discover.md`:21, `_decomposition.md`:604, `_grounding.md`:117 and `:183`) are **not** edited — they are citations of a historical defect, not live links | `rg -n "the-46-provisional-clauses" RUNBOOK.md` returns nothing; an anchor sweep transcript committed under this story's folder listing every `#`-heading slug in `RUNBOOK.md` before and after with an empty removed-set; `git diff --stat` showing no `.bklg/**` path outside this story's folder |

## Interaction quality

**This story renders no surface, and that is a finding rather than an exemption.** `RUNBOOK.md` is not
any crate's `readme =`, is not reachable by `include_str!` from inside a package (`_design.md`:543-553)
and appears in none of `_design.md`'s seven surface ids (`:113-169`). The signed-off design's
**composition** family — its density budget, hierarchy, transience policy and named anti-patterns —
therefore binds nothing here and is deliberately **not** re-decided (the one link is downstream and
one-way: DT-5's census sentence, `_design.md`:284-318). What does bind is the project's
**interaction-quality invariants** at `_decomposition.md`:226-310, which were written for exactly this
medium — *"the reader's 'interaction' is navigating a claim to its evidence and back inside one
sitting"* — plus the table's own composition, which is a real composition contract because a parser
and a human both read it.

Every invariant below is carried by a numbered row in the table above. None is a bullet here, because
`redkiln verify` extracts ACs from a leading `| AC-001 |` cell and a bullet in this section would be
gated by nothing.

**State family**

| Invariant (source) | Carried by | How it is verified |
| --- | --- | --- |
| **IQ-1 — in place, not a context jump** (`_decomposition.md`:232-239): 0 hops to read a claim, ≤ 1 hop to its evidence, and the hop lands on a specific anchor — never a repository root, never *"see the specification"* | AC-001, AC-002, AC-003, AC-004 | Each falsifier cell is readable standing alone; AC-004's replacement pointer names `.kb/decisions/0013-position-assignment-and-visibility.md`, one hop to a specific atom, checked by `test -f` |
| **IQ-2 — non-occlusion: a filter must not hide what it filters** (`_decomposition.md`:241-258): maturity markers annotate, they never subtract | AC-004 (removing ES-10 relocates its disclosure and says so), AC-005 (the ledger lists all 49, not a curated subset) | The repaired residual row states where the position-allocation axis is now disclosed; the per-prefix worksheet exhibits every one of the 49 |
| **IQ-3 — preserved position: inbound anchors survive** (`_decomposition.md`:260-271) | AC-006 (the heading is not "tidied"), AC-008 (the dead pointer is repaired without breaking a live one) | Before/after slug sweep of `RUNBOOK.md` with an empty removed-set; `rg` for the stale slug returning nothing |
| **IQ-4 — reversibility is bought before the irreversible act** (`_decomposition.md`:272-288): claims are worded to be superseded by a later dated claim rather than silently rewritten | AC-007 | The repair record carries a date and a commit, in `README.md`:227-234's claim-with-evidence form, so a later pass appends rather than overwrites |
| **IQ-5 — reachable without running anything** (`_decomposition.md`:289-296) | AC-001, AC-002, AC-003 | No falsifier cell says *"run the gate"*; each names the adapter or consumer whose existence would falsify the clause, and cites the clause |
| **IQ-6 — no orphan vocabulary** (`_decomposition.md`:297-304) | AC-002, AC-007 | *Expires* is defined where it is used — the phase cell says re-evaluated at this publish, and the narrative says what expiry costs (a bound added after publish is breaking) |
| **IQ-7 — truth at the publish commit** (`_decomposition.md`:305-310) | AC-005, AC-007 | No sentence in the section describes a previous tree; the outstanding-work paragraph becomes a record, and the identity is checked at the commit rather than inherited from phase 3 |

**Composition family — the table's own, since no designed surface is rendered**

| Invariant | Carried by | Real number / rule |
| --- | --- | --- |
| **Presentation exists at all**: a row is not four strings, it is a group name, a clause set, a falsifier and an owner — a row missing any one is a clause with no schedule, the defect the table exists against | AC-001, AC-002, AC-003 | Four non-empty cells per added row; CF-38 already makes an empty falsifier a build failure one level down (`spec/SPECIFICATION.md`:213-217) |
| **Placement**: additions sit with their family, the summary row stays last | AC-006 | Two `ES` rows near the existing `ES` rows; two `CF` rows near `CF-17, CF-34`; residual row remains the final row |
| **Density budget**: one row per **distinct falsifier**, never one per clause | AC-003, AC-006 | 17 rows → **21**, describing 49 clauses; the two `CF` additions stay two rows because they have two falsifiers, and thirteen `PS` clauses stay one row because PS-2 alone settles them |
| **Hierarchy**: the bolded framing of the residual row marks it as the summary of an exposure, not a peer of the ordinary rows | AC-004 | The bold survives; the framing's number moves five → four |
| **Grammar as composition**: header, column count, ID form, range form are the shape a parser will bind to | AC-006 | Header byte-identical; clause cells `[A-Z]{2}-\d+` comma-separated; ranges keep the spaced en dash |
| **Named anti-pattern — the prettier table**: reformatting while repairing is how `clause-maturity-audit`'s check gets written against a shape that no longer exists | AC-006 | No existing row other than `:606` changes; no column added; no re-alignment of the separator row |

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| **EC-001** | While distilling, a clause's own `[PROVISIONAL — …]` marker looks **wrong** — the axis is misnamed, the instrument phase is impossible, the falsifier is unfalsifiable | **Stop. Record the disagreement in `_ledger.md` and in the story's notes; do not write a row that disagrees with its clause, and do not edit the clause.** Two statements of one obligation drifting apart is the failure `spec/SPECIFICATION.md`:1056-1068 demoted VT-12 to non-normative prose over. The adjudication is a decision, not a table edit |
| **EC-002** | The per-prefix reconciliation does not land on `VT` 9 / `WF` 1 / `ES` 9 / `PS` 17 / `SY` 9 / `CF` 4 | **Do not adjust the ledger until the buckets fit.** A mismatch means §7.1, the clause bodies and this table disagree, and that adjudication is `clause-maturity-audit`'s AC-003 reconciliation. Record the bucket, the IDs and the delta; the story stops rather than balancing the books |
| **EC-003** | `spec/SPECIFICATION.md`:8128-8129 states *"ES-10 stays `[PROVISIONAL]` against exactly that measurement"* while ES-10 carries `[FROZEN]` at `:2823` | **A real, already-present staleness, and out of this PR's boundary.** Record it as a finding for `clause-maturity-audit` / the specification's own pass. It does not affect any count — §7.1 already books ES-10 as frozen — and it is *corroborating*, not contradicting, evidence for AC-004: the axis is still disclosed in §6.5's portfolio prose after the clause froze |
| **EC-004** | The implementer reaches for an ad-hoc `grep`/`awk` pipeline to extract the `[PROVISIONAL]` set from the specification | **Do not treat it as the instrument.** Measured while writing this spec: a naive two-form extraction (`#### XX-nn —` plus `**XX-nn.**`) finds 127 of 200 clause leads and yields **29** of the 49 — a false instrument that reads green. Only `xtask/src/spec_trace.rs`'s `parse_clauses` reads every clause form, which is the standing reason §1.3 is hand-computed (`:39-57`) and the reason the mechanised check is `clause-maturity-audit`'s. Use the per-prefix worksheet against §7.1 instead |
| **EC-005** | Repointing the anchor at `RUNBOOK.md`:4495 would be easier by renaming the heading | **Refused.** IQ-3: the heading's slug is the anchor everything else must reach, and a rename buys one live link by breaking every held one. Change the link, never the heading |
| **EC-006** | The implementer forms a view on whether ES-42's marker should be lifted at this publish | **Record it; do not settle it.** `project.md`'s AC-013 routes a settled answer through `/redkiln:kb-ingest` as a decision atom, and `project.md`:351 names *resolved in passing* as a live risk. The row makes the handoff visible; the verdict is somebody's explicit pass |
| **EC-007** | The change feels unfinished because nothing checks the repaired table | **That is the intended end state** (Context pack, Decision 1). Adding the check here collapses DR-4 into DR-3 and re-creates the risk register's first row (`project.md`:344): a falsifier chosen during an audit to make the audit pass |

## Non-functional

| id | requirement | how it is held |
| --- | --- | --- |
| **NF-001** | **Diff containment.** `git diff --stat` against the merge base names exactly two paths: `RUNBOOK.md` and this story's own backlog folder | The PR boundary block above; `redkiln verify --grain story` reads it and fails on any file outside |
| **NF-002** | **Reading cost at the gate.** A maintainer answers *"who falsifies this clause, and when?"* for any of the 49 in **0 hops**, with **≤ 1 hop** to the clause itself | IQ-1's budget (`_decomposition.md`:232-239); every added falsifier cell is self-contained prose, not a pointer |
| **NF-003** | **Reviewability.** The whole change is human-reviewable in one sitting: four added rows, one repaired row, one rewritten paragraph, one repaired link. Nothing is generated, so nothing needs regenerating to review it | AC-006's no-reformatting rule keeps the diff to the lines that changed meaning |
| **NF-004** | **The story-grain gate still bites on a docs-only change.** `cargo xtask affected --base main` maps this diff to no workspace package, and runs the five file-reading lints and `spec-trace` unconditionally for exactly that reason | `.redkiln/config.yaml`:30-40 — *"a story whose whole deliverable is an edit to SPECIFICATION.md maps to no package, and a purely package-shaped gate would compile nothing, read nothing, and call it green"* |
| **NF-005** | **No new gate time.** This story adds no step to `xtask`, so `cargo xtask ci --fast` costs what it cost before | The `xtask/**` exclusion in the PR boundary |
| **NF-006** | **Provenance.** Every artefact this story commits under its own folder (the distillation transcript, the per-prefix worksheet, the anchor sweep) carries the commit sha and the date it was produced, so a later reader can tell whether it describes the tree they are looking at | IQ-4/IQ-7; the form is `README.md`:227-234's dated claim-with-evidence, the same one `crate-set-decision/_ledger.md` binds for its `package-check` capture |

## Implementation notes (non-prescriptive)

- **Read in this order.** `RUNBOOK.md`:580-635 whole, once, before touching anything — the table and
  the two paragraphs beneath it are one argument and editing the table without the narrative is how
  AC-007 gets forgotten. Then the four clause bodies, then ES-10's frozen note, then §7.1.
- **Distil one clause at a time, in writing.** For each of ES-41, ES-42, CF-39, CF-40: paste the
  marker into the transcript, write the cell beside it, and check that every noun in the cell appears
  in the marker. The transcript is the AC-001/AC-002/AC-003 evidence and it is cheaper to produce
  during the work than to reconstruct after it.
- **The per-prefix worksheet is the cheapest real check available.** Bucket every clause ID currently
  in the table by prefix, apply − ES-10 and + the four, and compare each bucket to
  `spec/SPECIFICATION.md`:8513-8518. Today's buckets are `VT` 9, `WF` 1, `ES` 8, `PS` 17, `SY` 9,
  `CF` 2 = 46; after the repair `ES` 9 and `CF` 4 = 49. `VT`, `WF`, `PS` and `SY` are already correct,
  which is itself worth recording — it localises the whole defect to two prefixes.
- **The en dash is U+2013 with a space on each side.** `VT-21 – VT-24`, not `VT-21-VT-24` and not an
  em dash. The four additions are single-clause and introduce none, but a well-meaning normalisation
  of the existing three is a silent breaking change to AC-006's grammar.
- **Write the narrative last**, after the table is right, so the record describes what actually
  landed rather than what was planned. Include the date, the arithmetic and the sentence naming
  `clause-maturity-audit` as the missing check — that sentence is what stops the next reader
  concluding the table is now self-checking.
- **The anchor is a two-line edit and a sweep.** Change the link target at `RUNBOOK.md`:4495, then
  list every heading slug in the file before and after and confirm the removed-set is empty. Leave
  every backlog file that *quotes* the stale slug alone; they are describing a defect, not linking.
- **If the work suggests a new rule, a new gate step or a specification edit, it is out of boundary
  by construction.** Write it down and hand it to `clause-maturity-audit`, which is the next story in
  this slice and is already scoped to receive it (`clause-maturity-audit/spec.md`:190-205).

## Tests and CI (merge gate)

Grounded in the testing brief's row for project AC-004 (`_decomposition.md`:467) and its CI
implication (`:708-714`), which is explicit that AC-004's ledger repair is *"a one-time content edit
to `RUNBOOK.md`, not a new gate step by itself"* — the permanent gate addition belongs to the
slice-mate.

| tier | command / path | proves |
| --- | --- | --- |
| static (story gate) | `cargo xtask affected --base main` (`.redkiln/config.yaml`:40) | The story-grain gate. Maps this diff to no package and therefore runs the five file-reading lints and `spec-trace` unconditionally — a docs-only story is still gated (NF-004) |
| static | `cargo xtask spec-trace` | The specification's cross-references and maturity markers are intact, and its output is **byte-identical before and after** — i.e. no clause, no census figure and no generated region moved (AC-005) |
| static | `cargo xtask lints` | The five file-reading checks four clauses name as gate steps; regression only — this story adds none |
| recorded (evidence artefact) | distillation transcript under `.bklg/from-contract-to-published-library/publication-and-positioning/falsifier-ledger-repair/` | Each new `Falsified by` cell is a distillation of its clause's own marker and asserts nothing the marker does not (AC-001, AC-002, AC-003) |
| recorded (evidence artefact) | per-prefix reconciliation worksheet, same folder | The ledger's clause-ID set equals §7.1's `[PROVISIONAL]` column per prefix — `VT` 9, `WF` 1, `ES` 9, `PS` 17, `SY` 9, `CF` 4, total 49 (AC-005) |
| recorded (evidence artefact) | anchor sweep transcript, same folder + `rg -n "the-46-provisional-clauses" RUNBOOK.md` | Phase 12's exit criterion resolves to the live heading and no previously-live anchor was removed (AC-008) |
| static (diff assertion) | `git diff --stat` against the merge base | Exactly two paths; `spec/SPECIFICATION.md`, `xtask/**` and every other `.bklg/**` folder untouched (NF-001, AC-005) |
| integration (slice) | `cargo xtask ci --fast` (`.redkiln/config.yaml`:50-56) | This project's non-terminal integration bar, run over the slice — all four `wasm32` steps and `spec-trace` included; unchanged in cost by this story (NF-005) |
| **deferred by design** | `clause-maturity-audit`'s mandatory set-equality step (`_storymap.md`:61, `_decomposition.md`:708-714) | The mechanised successor to the recorded worksheet. **It does not exist at this story's merge, and that is the intended state** (EC-007) |

## Risks and coupling (PR-scoped)

| risk | why it bites here | containment |
| --- | --- | --- |
| **A falsifier chosen to make a future audit pass** — `project.md`:344's first risk row | The implementer knows the next story asserts set-equality against this table, and the cheapest way to guarantee that is to write rows that fit the check rather than the clause | DR-4's ordering (`_storymap.md`:179-184) puts the repair strictly before the check, and the check is out of boundary. AC-001–AC-003 verify each cell against its **marker**, not against any tool. EC-001 makes a disagreement a stop |
| **Grammar coupling to a parser that cannot yet fail** | `clause-maturity-audit` will bind to the header, column count and ID form. Any reformatting now is a breaking change to a consumer with no test | AC-006 freezes the grammar in this spec, so the coupling is written down on both sides rather than discovered |
| **The specification is the cheaper place to make the sets equal** | Editing a clause is one line; repairing four rows and a narrative is a page. Every incentive points the wrong way | `spec/SPECIFICATION.md` is outside the PR boundary by construction (`project.md`:281-283, `:350`), and AC-005 requires `spec-trace` output identical before and after |
| **ES-42's marker gets re-evaluated in passing** | The row must state that phase 12 owns it, and stating that invites deciding it. `project.md`:351 names *resolved in passing* explicitly | AC-002 requires the handoff to be visible and the verdict absent; EC-006 routes any view through `/redkiln:kb-ingest` |
| **The disclosure disappears with the row** | ES-10's own note warns of exactly this, one level down (`spec/SPECIFICATION.md`:2823-2831); repeating it one level out is easy because striking an ID from a cell *looks* complete | AC-004 makes the four edits a single indivisible criterion — the clause cell, the framing, the replacement pointer and the phase cell — so three of four is a fail |
| **Coupling out**: `clause-maturity-audit` blocks on this story (`story.md` `blocks: HS-S0089`) | If this lands wrong, the next story's first real run reports a disagreement it did not cause, and the cheap fix there is to weaken the check | The recorded worksheet is the handover artefact: the next story reconciles against a written set, not against a table it must re-derive |
| **Coupling out**: DT-5's published census sentence | `_design.md`:284-318 takes its four numbers from the AC-003 audit at the publish commit, and that audit reads this table. A wrong number here becomes a wrong number on a page that cannot be edited after publish | AC-005's per-prefix identity, which is stricter than the total the sentence quotes |
| **Zero coupling in** | No blockers at all (`_storymap.md`:162); may run alongside milestone 1 | Nothing to wait on, and nothing that can invalidate the work mid-flight |

## Dependencies

- **Blocks on:** *(none)* — matches `story.md`'s `blocked_by: []` and `_storymap.md`:162, *"no
  blockers at all; can run alongside milestone 1"*. This story reads `spec/SPECIFICATION.md`,
  `RUNBOOK.md` and `xtask/src/spec_trace.rs` as they stand at `main`; none of them is being changed by
  a sibling in flight.
- **Unlocks:** `clause-maturity-audit` (`story.md`'s `blocks: HS-S0089`; `_storymap.md`:163, *"after
  2.1, so the audit never reads a short ledger"*), and transitively `deferred-clause-reread`, which
  extends that audit's module (`_storymap.md`:164).
- **Does not order against:** `registry-surface-diff` (unordered within the slice, `_storymap.md`:165)
  and every story in milestones 1 and 3.

## Anchors (progressive disclosure)

Open these at the moment named, not before. Everything needed to *start* is above.

| anchor | why it is load-bearing | when to open | serves |
| --- | --- | --- | --- |
| `spec/SPECIFICATION.md` | The source of every falsifier distilled here and the only authority on what each clause says. ES-41 at `:4381-4394`, ES-42 at `:3079-3088`, CF-39 at `:7631-7642`, CF-40 at `:7661-7684` | Once per clause, at the moment you write that clause's row — one clause open at a time, so no cell borrows another's language | AC-001, AC-002, AC-003 |
| `spec/SPECIFICATION.md` (ES-10's frozen note, `:2816-2831`) | The instruction that makes AC-004 four edits rather than one: *"lifting it deletes the disclosure, and the acceptance has to move in the same act or the exposure disappears silently"* | Before touching the residual-exposure row, and again when wording its replacement pointer | AC-004 |
| `spec/SPECIFICATION.md` (§1.3 census `:219-221`; §7.1 summary table `:8511-8519`; §6.5's four-clause framing `:259-264` and `:371`) | The per-prefix column is the reconciliation instrument — `VT` 9, `WF` 1, `ES` 9, `PS` 17, `SY` 9, `CF` 4 — and §6.5's *"ES-10 was the fifth until phase 4 froze it"* is the source of the corrected framing | When building the worksheet, and when changing *five* to *four* in the residual row | AC-004, AC-005 |
| `RUNBOOK.md` (`:580-635`) | The mount: the heading, the 17-row table and the two paragraphs beneath it. The narrative and the table are one argument; `:626-627` is the *"not a thing to do in passing"* instruction the whole story exists to honour | First, whole, before any edit; again when writing the repair record | AC-001, AC-004, AC-006, AC-007 |
| `RUNBOOK.md` (`:4492-4497`) | Phase 12's exit criterion — the inbound reference that must resolve, and the only known occurrence of the dead slug | Last, as its own edit, after the table and narrative are settled | AC-008 |
| `xtask/src/spec_trace.rs` (`:39-57`, and `parse_clauses`) | The standing reason §1.3 is hand-computed and never generated, which is what makes the reconciliation mean anything — and the demonstration that no ad-hoc grep reads every clause form (EC-004) | Before deciding *how* to verify the set identity; do not modify | AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_decomposition.md` (`:226-310` IQ-1…IQ-7; `:467` the AC-004 test row; `:708-714` the CI implication) | The interaction-quality invariants this medium is actually held to, the test-mix row for project AC-004, and the statement that this story adds no gate step | When writing the falsifier cells (IQ-1, IQ-5) and before wondering whether to add a check | AC-001, AC-004, AC-007, AC-008 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_storymap.md` (`:60`, `:161-165`, `:179-184`) | The story's row, its slice position, and the ordering constraint *"must not be reordered by a later re-plan"* | If any pressure appears to merge this with the audit | AC-005, AC-006 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/clause-maturity-audit/spec.md` (`:190-205`) | The consumer's own boundary statement — it explicitly disclaims `RUNBOOK.md` and the stale anchor as *"whoever owns that section"*, which is this story, and it declares that it will parse this table | When deciding the table's shape (AC-006) and before assuming someone else fixes the anchor | AC-006, AC-008 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | Where ES-10's position-allocation disclosure now lives; the target of AC-004's one-hop replacement pointer. Accepted and immutable — cite, never edit | When wording the replacement sentence in the residual row | AC-004 |
| `.kb/decisions/0010-the-suite-must-prove-itself.md` | The *"a skip is reported, never silent"* discipline, applied here to prose: a clause with no row is a skip nobody reported. It is why the repair is four visible rows rather than a corrected count | When tempted to fix the arithmetic without adding the rows | AC-005, AC-007 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/_design.md` (`:113-169` the seven surface ids; `:284-318` DT-5's census sentence; `:543-553` what is packaged) | Proof that this story renders none of the signed-off surfaces, and the one downstream consequence: four published numbers sourced from the audit that reads this table | Once, when confirming no design surface is touched; again if a number in the table is in doubt | AC-005 |
| `.bklg/from-contract-to-published-library/publication-and-positioning/crate-set-decision/_ledger.md` | The in-project precedent for a ledger whose rows are commands and diff assertions rather than compiled tests — the same shape this story's evidence takes | When filling `_ledger.md`'s `verifying_test` fields | all |
| `references/adr/0016-the-wire-format.md` (`:1432-1436`) | The record that spotted the heading/row disagreement while walking past it. **Evidence, not an obligation** — accepted records are not amended here | Only if the repair record needs the provenance of when the defect was first seen | AC-007 |

## Clarifications resolved during spec

1. **The AC set is unchanged at eight.** AC-001…AC-008, exactly the ids the first pass decided. The
   *Behavior and interfaces* table's nine rows fold into eight criteria: *"nothing normative moves"*
   is not its own AC but the second half of **AC-005**, because the set identity is only meaningful
   if it was reached by repairing the ledger rather than by moving the specification, and splitting
   the two would let a story pass one while failing the other.
2. **§7.1's totals row is at `spec/SPECIFICATION.md`:8519, not `:8138`.** `RUNBOOK.md`:624 cites
   `:8138` and the first pass inherited that citation; line 8138 is now inside §6.6. The §7.1 summary
   table spans `:8511-8519`. Because `RUNBOOK.md` is inside the PR boundary, correcting the citation
   is part of **AC-007**'s narrative rewrite; the specification itself is not touched.
3. **The reconciliation instrument is §7.1's per-prefix column, not a hand count and not a grep.**
   Measured while writing this spec: a naive two-form extraction of clause leads finds 127 of 200 and
   yields 29 of the 49, so an ad-hoc pipeline would read green while being wrong (EC-004). The
   per-prefix buckets — `VT` 9, `WF` 1, `ES` 9, `PS` 17, `SY` 9, `CF` 4 — are checkable by hand,
   localise the defect to two prefixes, and no compensating pair of errors passes them. Counted at
   HEAD the table's buckets are `VT` 9, `WF` 1, `ES` 8, `PS` 17, `SY` 9, `CF` 2 = 46, which
   independently confirms the first pass's arithmetic and confirms that `VT`, `WF`, `PS` and `SY`
   need no edit at all.
4. **A live specification staleness was found and deliberately not fixed.**
   `spec/SPECIFICATION.md`:8128-8129 says *"ES-10 stays `[PROVISIONAL]` against exactly that
   measurement"* while ES-10 carries `[FROZEN]` at `:2823`. It affects no count (§7.1 already books
   ES-10 as frozen) and it is out of boundary, so it is **EC-003**: recorded as a finding for
   `clause-maturity-audit`. It also corroborates AC-004 — the position-allocation axis really is
   still disclosed in §6.5's portfolio prose after the clause froze, which is why removing its ledger
   row is a relocation rather than a deletion.
5. **The anchor repair is unambiguously this story's.** `clause-maturity-audit/spec.md`:198 disclaims
   it in terms — *"it belongs to whoever owns that section, not to a gate step that happens to be
   reading nearby"* — and no other story in the project claims it. AC-008 takes it.
6. **The stale slug's other occurrences are citations, not links.** Seven occurrences across
   `_grounding.md`, `_decomposition.md`, `crate-set-decision/` and `clause-maturity-audit/` quote
   `#the-46-provisional-clauses` as *evidence that the phase-12 section is stale*. Editing them would
   destroy the evidence they exist to carry, so AC-008 scopes the sweep to `RUNBOOK.md` and asserts
   the backlog files are untouched.
7. **Composition invariants are the table's, not the design's.** `_design.md` binds seven surfaces
   and `RUNBOOK.md` is none of them, so the signed-off density budget and hierarchy do not apply. The
   *Interaction quality* section therefore states the table's own composition contract — one row per
   distinct falsifier, 17 → 21 rows, four columns, family placement, summary row last — as AC-003 and
   AC-006, rather than claiming a design conformance this story cannot have.
8. **`redkiln verify`'s AC extraction shaped the layout.** Every invariant that binds is a `| AC-### |`
   row in the acceptance table; the *Interaction quality* section maps invariants to those ids and
   contains no bullet that would silently escape the ledger.
