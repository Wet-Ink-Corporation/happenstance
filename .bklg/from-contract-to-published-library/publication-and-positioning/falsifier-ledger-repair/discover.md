---
item: HS-S0088
stage: discover
created: 2026-08-12T13:02:56.963Z
updated: 2026-08-12T13:02:56.963Z
template_sig: 86ce4036
rendered_sig: 4a037d5a
---

# Discover — The falsifier ledger is repaired before anything reads it

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:60 | "Repair the five known-short rows in `RUNBOOK.md`'s provisional ledger — add ES-41, ES-42, CF-39, CF-40 each with a deliberately chosen falsifier and an owning phase, remove ES-10's stale row — as its own change, landed before anything reads the table." |
| AC-004 | `project.md`:238-242 | The ledger carries rows for ES-41, ES-42, CF-39, CF-40, each with a falsifier and an owning phase; ES-10's row is gone; the ledger's clause set equals `spec-trace`'s parsed `[PROVISIONAL]` list, checked rather than counted by hand. |
| The self-diagnosis, verbatim | `RUNBOOK.md`:622-635 | "the groups above are short by **ES-41, ES-42, CF-39 and CF-40**, and the last row still carries **ES-10**, which is no longer provisional. That is five edits." Also: "adding a row means deciding a falsifier and an owning phase, which is not a thing to do in passing." |
| The ledger's methodology note | `RUNBOOK.md`:618-620 | The clause-ID set was checked against `spec-trace`'s own `[PROVISIONAL]` list once before and agreed only "at that commit" — hand-reconciliation rots; this is the precedent for AC-004's "checked, not counted" requirement in the *next* story. |
| The corrected total | `spec/SPECIFICATION.md`:219-221, `RUNBOOK.md`:623-624 | 49 `[PROVISIONAL]`, corrected from a stale "46" the phase-4 recount fixed at the heading but not in the rows. |
| Risk register, first row | `project.md`:344 | "The falsifier ledger is repaired *during* the audit rather than before it, and a row's falsifier is chosen to make the audit pass \| Medium/High \| DR-4 is a separate, earlier obligation than DR-3." Directly governs sequencing: this story must land, and be judged on its own falsifiers' honesty, *before* `clause-maturity-audit` exists to check it. |
| Merge-order constraint | `_storymap.md`:161-162, 180-182 | `falsifier-ledger-repair` has no blockers, can run alongside milestone 1, and strictly precedes `clause-maturity-audit` — a constraint the story map calls out as "must not be reordered by a later re-plan." |
| The existing table's shape | `RUNBOOK.md`:588-606 | Rows are grouped by shared falsifier, not one row per clause ID — e.g. the last row already groups five clauses (ES-10, ES-11, ES-12, ES-35, ES-40) under one falsifier description. The repair must decide, for each of the four added clauses, whether it joins an existing group or needs a new row — not just append four singleton rows without checking fit. |

## Questions

- **What is the actual falsifier and owning phase for each of ES-41, ES-42, CF-39, CF-40?** This is the substantive content decision `RUNBOOK.md:627` says "is not a thing to do in passing" — it requires reading each clause's own text in `spec/SPECIFICATION.md` to determine what would actually falsify it, not just picking a plausible-sounding phase number. Genuinely open at discovery time; deferred to spec, which must read each clause's marker text before proposing a falsifier, per the same discipline CF-38 already enforces on the specification's own inline falsifiers.
- **Does ES-10's removal need anything beyond deleting the row?** `RUNBOOK.md:630-631` states it plainly ("no longer provisional") but doesn't say what `spec-trace` currently reports for ES-10's marker. Deferred to spec: confirm ES-10's actual current marker in `SPECIFICATION.md` before deleting the row, so the deletion is a correction rather than an assumption.

## Decision

`RUNBOOK.md`'s provisional-clause ledger (`:588-606`) is short by exactly the five edits it already names against itself: add rows for ES-41, ES-42, CF-39 and CF-40 (each with a deliberately reasoned falsifier and owning phase, not a placeholder), and remove ES-10's stale row. This lands as its own change, before `clause-maturity-audit` exists to read the table — the ordering constraint the risk register and the merge order both name explicitly, so that no falsifier here is ever chosen to make a later audit pass rather than because it is the true falsifying condition. Spec will read each of the four clauses' own text in `spec/SPECIFICATION.md` to derive the falsifier and phase, and will confirm ES-10's current marker before deleting its row.

## The wrong implementation

A repair that adds all four missing rows with the right clause IDs in the right table shape and removes ES-10's row — satisfying a naive "does the row exist" check and even satisfying the *next* story's set-equality check between the ledger's clause-ID set and `spec-trace`'s parsed `[PROVISIONAL]` list — but where the added falsifier text for ES-41/ES-42 (or CF-39/CF-40) is copied from a neighbouring row rather than derived from what actually falsifies *those* clauses. For example, reusing the last row's "the far-end adapter on each axis" language for a clause whose falsifying condition is unrelated. This passes every mechanical check this project adds (the row exists, the ID set matches, a falsifier field is non-empty) while being exactly the defect `RUNBOOK.md:625-628` warns against: a falsifier chosen "in passing" rather than deliberately, which is indistinguishable at a glance from a falsifier chosen honestly. What catches it: a reviewer reading each new falsifier against the clause's own marker text in `SPECIFICATION.md` for whether it actually names a plausible falsifying mechanism specific to that clause — a judgement call no set-equality check can make, which is exactly why this story is sequenced before, and separately from, the mechanical audit.

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
