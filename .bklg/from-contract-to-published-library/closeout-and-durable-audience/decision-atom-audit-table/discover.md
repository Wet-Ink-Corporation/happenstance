---
item: HS-S0129
stage: discover
created: 2026-08-12T13:03:54.408Z
updated: 2026-08-12T13:03:54.408Z
template_sig: 86ce4036
rendered_sig: 6fe45b11
---

# Discover — An accepted atom per settled answer, naming the alternatives that lost

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Produce the audit table mapping every question this initiative settled to an accepted atom under `.kb/decisions/` and the alternatives that atom records as rejected, with a stated reason for each reserved ADR number nobody needed to write." | `_storymap.md:57` | Every reserved number gets a row — either an atom or a stated reason, never a silent gap. |
| AC-005 | `project.md:207-210` | Atom must be `status: accepted` and its body must record rejected alternatives; every unwritten reserved number carries a stated reason. |
| DR-6 — Decision-atom audit | `project.md:148-152` | Cites `RUNBOOK.md:276-283`'s precedent: "a reserved number staying empty can be correct." |
| The ADR queue, verbatim | `RUNBOOK.md:262-308` | Numbers `0008`–`0028`, one question each, in phase order; `0009` may legitimately stay empty until phase 9 per `RUNBOOK.md:276-283`. |
| Current state on disk (checked directly) | `.kb/decisions/` listing | Seventeen atoms exist: `0001`–`0016`, `0029`. `0017`–`0028` are unwritten. Matches `_grounding.md:20-27`'s "still reserved and unwritten... belong to sibling projects... downstream in merge order." |
| The coverage-audit precedent | `RUNBOOK.md:309-320` | Phase 4's own experience: "a phase's clause range versus the union of its ADRs' ranges is 'two numbers, and nothing checks that they are equal.'" Directly applicable — this story must *compute* the union of what the written ADRs cover, not assume it equals what this initiative raised. |
| DR-6 audit procedure, not-yet-populatable | `_decomposition.md:125-146` | The testing brief states the row shape now (Settled question / Atom / Status / Alternatives) but explicitly defers filling rows: "ADR numbers `0017`–`0028` are reserved but unwritten at planning time... The audit table AC-005 requires has this row shape, to be filled at closeout, not now." |
| Immutability rule | `.kb/decisions/README.md:7-18` | An accepted decision's body is never edited; a gap this audit finds is corrected by a *new* superseding atom, never by editing the one found wanting — binds this story's own conduct if it surfaces a defect. |
| `dependsOn: []` | `_storymap.md:147` (merge order 3) | "Independent of slices 1–2 in principle; sequenced here so its findings reach slice 5." No upstream story dependency. |

## Questions

- **Have the reserved numbers `0017`–`0028` actually been written by the time this story executes?** Checked directly at discover time: they have not (`.kb/decisions/` holds only `0001`–`0016` and `0029`). Per `project.md` *Dependencies*, this project sits at rank 6, downstream of every sibling that would write them — so by the time this story's spec/implementation runs, they are expected to exist. **Deferred to spec**: the audit table is populated against whatever exists on the tree at execution time, not against this discover-time snapshot; if any expected number is *still* unwritten at execution time despite its owning sibling being closed out, that itself is a finding for `findings-disposition-register`, not a gap this story papers over.
- **Does the union of what the written ADRs cover actually equal what this initiative raised?** Not yet — this is exactly the `RUNBOOK.md:309-320` coverage-audit precedent, and it is computed at spec/implementation time against the real tree, not assumed here. **Deferred to spec**, which must state the computation explicitly (each ADR's scope statement vs. a re-derived list of what this initiative actually settled) rather than trust the queue's own parenthetical clause ranges, since phase 4 already showed those can be stale.
- The evaluator-persona question and DoD 13's caveat do not touch this story — it audits `.kb/decisions/`, not `.kb/product/` or the gate run. Noted for completeness.

## Decision

BR-15's audit exists because a settled answer that never became an accepted atom is indistinguishable, from outside the room where it was decided, from an answer nobody made — and an atom that does not name what it rejected is, per `.kb/decisions/README.md:31-33`, "indistinguishable from an accident." This story turns the ADR queue (`RUNBOOK.md:262-308`) from a plan into an audit: for every settled question, is there an accepted atom, and does its body actually record the alternatives it rejected; for every reserved number the queue names, is it written, and if not, is there a stated and legitimate reason (per the `0009` precedent) rather than silence. The spec that follows will specify the concrete cross-reference procedure — walking `RUNBOOK.md`'s queue table against `.kb/decisions/`'s actual contents at execution time — and the row shape the testing brief already fixed (`_decomposition.md:135-137`).

## The wrong implementation

An audit table that lists only the ADRs that exist on disk, presented as complete, with no row for a reserved-but-unwritten number and no attempt to verify that an atom's body actually names its rejected alternatives (versus just existing with `status: accepted`). It would pass a shallow reading of "an atom per settled question" but fails AC-005 on two fronts at once: a reserved number with genuinely no atom and no stated reason is a silent gap masquerading as completeness (exactly what DR-6 exists to prevent — "any ADR number the queue reserved and nobody wrote is listed with the reason it was not needed"), and an atom whose body was never checked for rejected alternatives could satisfy the letter of "an atom exists" while failing `.kb/decisions/README.md`'s own bar for what belongs in the layer at all. What catches it: cross-referencing `RUNBOOK.md`'s full `0008`–`0028` table against `.kb/decisions/`'s directory listing by number, not just by "some atoms exist" — every number gets a row, and every row's "alternatives" cell is checked against the atom's actual body text, not inferred from its existence.

## Gate: Discover

Tick each box once the sections above satisfy it. `redkiln advance` reads these
against `.redkiln/templates/gates/discover.md` — where each box's rationale is
written — and will not leave `discover` until every one is ticked. Keep each box on
one line; the parser matches line by line.

- [x] The problem is framed in one paragraph.
- [x] Prior art and constraints are recorded in the signal ledger.
- [x] Open questions are either answered or explicitly deferred.
- [x] The next stage (spec) has a clear starting point.
- [x] The wrong implementation this work rejects is named.
- [x] No conformance rule added here asserts a literal position value.
- [x] Any `[FROZEN]` clause this touches is changed by a new ADR, written first.
- [x] If a rule here seems wrong, it is fixed and the reason given in the same change.
