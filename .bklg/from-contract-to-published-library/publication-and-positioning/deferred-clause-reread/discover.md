---
item: HS-S0090
stage: discover
created: 2026-08-12T13:02:59.090Z
updated: 2026-08-12T13:02:59.090Z
template_sig: 86ce4036
rendered_sig: b8188991
---

# Discover — Every deferral is re-read at publish and says why, dated

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Storymap slice | `_storymap.md`:62 | "Re-read all ten `[DEFERRED]` clauses for whether the deferral is still honest at publish, give each a dated, consumer-readable reason, and extend the audit in the same change so an inherited (undated, unchanged) reason fails the gate — the *'a skip is reported, never silent'* discipline of `.kb/decisions/0010-the-suite-must-prove-itself.md` applied to prose." |
| AC-005 | `project.md`:243-245 | "Each of the ten `[DEFERRED]` clauses carries a stated reason, readable by a consumer, that was re-read at this publish rather than inherited." |
| `dependsOn: clause-maturity-audit` | manifest, `_storymap.md`:164 | "after 2.2, so the ten reasons and the check that requires them land together and the gate is never red between them." |
| DR-5 | `project.md`:174-175 | Restates AC-005 as a derived requirement. |
| Clause count, verified | `spec/SPECIFICATION.md`:219-221 | 10 `[DEFERRED]` clauses of the document's 200 total — the exact set this story re-reads. |
| `[DEFERRED]` marker's own definition | `spec/SPECIFICATION.md`, lines 205-208 (the marker-vocabulary section preceding line 213's CF-38 discussion) | "The clause records the shape of the question and the thing that answers it. A deferral with no owning phase is a decision the next pass makes by accident." — the standing bar a deferral's reason must already clear; this story adds *re-reading at publish* and *dating* on top of it. |
| The reused discipline | `.kb/decisions/0010-the-suite-must-prove-itself.md` | "A skip is reported, never silent" — cited by `_grounding.md`:171-175 as the pattern AC-005 applies to documentation instead of test output: a declined capability's constructor rejects an empty reason, and this story's analogue is a deferred clause whose reason string may not be empty or stale. |
| Testing brief's named wrong implementation | `_decomposition.md`, Testing brief AC table, AC-005 row | "a deferred clause whose reason string is unchanged from a prior phase's audit — the 're-read' requirement is the axis the existing CF-38 check does not cover today." |
| CF-38's existing coverage and its limit | `spec/SPECIFICATION.md`:213-217 | Already forbids an *empty* falsifier/experiment on a `[PROVISIONAL]` or `[DEFERRED]` clause. It does not check that the reason was re-examined at this publish rather than copy-pasted forward from a previous phase — that gap is exactly what this story closes. |

## Questions

- **What does "re-read... rather than inherited" mean mechanically?** A reason string cannot, by inspection alone, prove it was re-considered rather than copied. Deferred to spec: the likely mechanism is a per-clause dated annotation (e.g. "as of `YYYY-MM-DD`, still honest because...") that changes with each genuine re-read, checked by the audit for a date matching the current publish pass — spec must pick the concrete form, since no brief specifies it beyond "dated."
- **Do all ten deferrals still hold, or does re-reading surface one that should be resolved now?** Genuinely unknown until the content work happens — this is the point of "re-read," not a rhetorical exercise. Deferred to spec/implementation: if a re-read finds a deferral that is no longer honest (the experiment that would resolve it has already run, say), that is new information changing this story's scope, not a discovery-stage question this story can pre-answer.

## Decision

Each of the specification's ten `[DEFERRED]` clauses is re-read at this publish, and its stated reason is replaced or reconfirmed with a dated, consumer-readable sentence — not silently carried forward from whichever phase last touched it. The audit story this depends on is extended, in the same change, to fail when a deferred clause's reason is undated or textually identical to what a prior audit recorded, applying `.kb/decisions/0010-the-suite-must-prove-itself.md`'s "a skip is reported, never silent" discipline to prose rather than test output. Spec will decide the concrete dating mechanism and will read each of the ten clauses' current text before writing its reason, rather than assuming the existing reasons are already adequate.

## The wrong implementation

An extension to the audit that checks each of the ten `[DEFERRED]` clauses has a non-empty reason — which CF-38 already substantially covers — but does not check that the reason is *dated to this publish* or *different from what a prior phase's audit recorded*. A deferred clause whose reason string reads identically to what it said at phase 4, silently carried forward through phases 5 through 12 without anyone actually rereading whether the deferral is still honest, would pass such a check: the string is non-empty, it names an owning phase, CF-38 is satisfied. This is precisely the testing brief's named wrong implementation for AC-005, and it matters because a stale-but-present reason is functionally the same dishonesty as an empty one — it reads as "still deliberately deferred" when it may only be "nobody looked." What catches it: the audit's own new check compares each reason's recorded date (or a content hash) against the previous audit's recorded value and fails when they match unchanged, forcing an actual re-read rather than a re-assertion.

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
