---
item: HS-S0127
stage: discover
created: 2026-08-12T13:03:52.226Z
updated: 2026-08-12T13:03:52.226Z
template_sig: 86ce4036
rendered_sig: 4039544e
---

# Discover — DoD 1–12 and 14–15 re-observed as a set on this tree

## Signal Ledger

Evidence gathered: prior art, constraints, and the problem framing.

| Signal | Source | Implication |
| ------ | ------ | ----------- |
| Slice one-liner: "Re-run or re-inspect DoD 1–12 and 14–15 on this tree and record all fourteen in one table, each with the command and the artefact path — a sibling's `_ledger.md` may point at what to re-run, never stand in as the evidence." | `_storymap.md:55` | One table, fourteen rows, each independently re-run — not fourteen citations of memory. |
| AC-003 | `project.md:200-203` | "One record lists all fourteen, each with the command run on this tree and the artefact path it produced. No entry cites only the producing project's ledger; a ledger may be the pointer, never the evidence." |
| DR-3 — Re-observation as a set | `project.md:135-138` | Each scenario re-run or re-inspected on *this* tree, recorded together in one artefact with command + artefact path; "a scenario marked from the memory of the project that produced it does not count." |
| The sixteen DoD scenarios, verbatim | `../initiative.md:360-407` | DoD 1–12 and 14–15 are the fourteen this story owns (13 and 16 are split out to `published-tree-delta-statement` and `product-atom-promotion-via-kb-ingest` respectively per `_storymap.md:131-134`). |
| `require_ledger: true` | `.redkiln/config.yaml:67` | Makes every producing sibling's own `_ledger.md` mandatory and cited — the pointer this story follows to know *what* to re-run, never accepted as the re-run itself. |
| `_closeout-record.md` convergence | `_storymap.md:24-30` | This story's fourteen-row table is one section of that single companion artefact, not a standalone file — "fourteen ledger entries in fourteen places is the failure mode the charter's DoD preamble is written against." |
| `dependsOn: whole-gate-green-on-the-assembled-tree` | `_storymap.md:144-146` | Supplies the tree this story re-observes against — "consumes slice 1's run for the scenarios the gate itself re-proves" (e.g. DoD 1, 2 fold into the gate run directly). |

## Questions

- **Which of the fourteen scenarios are actually re-proved *by* the gate run itself, versus needing a separate re-run?** DoD 1 (`cargo run -p course-subscriptions`) and DoD 2 (the compile-fail case) are `@smoke` cases distinct from `cargo xtask ci`'s own steps; DoD 3–8 are conformance-suite and freeze-verdict scenarios exercised inside `tests`/`proof-artefact`; DoD 9–12 concern the *published* crate and its registry presentation. **Deferred to spec**: the spec must map each of the fourteen to the concrete command that re-proves it on this tree, distinguishing "re-run directly" from "re-inspect the artefact HS-P0016 produced and confirm it still holds on this SHA" — DR-3's own text permits either ("re-run or re-inspect") but requires the record to say which.
- **Do DoD 9–12 (registry/publish scenarios, owned by `publication-and-positioning`) get literally re-run here, or re-inspected?** They describe actions against an already-published registry crate (`cargo add` from outside the workspace), which this story cannot re-execute without re-publishing. **Deferred to spec**: the likely answer is re-inspection of HS-P0016's own cited artefacts against the current SHA (confirming nothing has since changed the claim), but the spec must state this explicitly rather than let "re-observed" silently mean "re-read" for exactly these four, which is the failure mode DR-3 forbids.
- The evaluator-persona question and DoD 13's literal-tree caveat are not this story's to resolve — DoD 13 is `published-tree-delta-statement`'s (next in this slice) and the persona question belongs to `durable-audience`. Flagged here only so their absence reads as scope, not oversight.

## Decision

Sixteen previously-separate, separately-remembered ticks are worth nothing as a *set* until they are re-observed together on one tree by one story, because a genuine interaction between two independently-proved scenarios — one sibling's DoD claim quietly invalidated by another's later merge — is invisible to sixteen separate rememberers and visible only to one reader holding fourteen rows side by side. The spec that follows will specify: the concrete re-proof or re-inspection command for each of the fourteen scenarios, the artefact path each produces on this tree, and — for the four registry-facing ones — the explicit statement of which the record treats as re-inspection rather than re-run.

## The wrong implementation

A table with fourteen rows, one per DoD scenario, where each row's "artefact path" column simply links to the *producing* sibling's own `_ledger.md` entry from when that project closed out — technically fourteen rows, technically fourteen citations, and it would pass a shallow read of AC-003. It fails DR-3's actual test, stated explicitly: "No entry cites only the producing project's ledger; a ledger may be the pointer, never the evidence" (`project.md:202-203`). A sibling's own ledger proves the scenario passed *on the tree that sibling's own project had at the time it merged* — which is precisely the assumption this whole project exists to stop making, per the charter's DoD preamble ("Each of these is run and observed to pass on the assembled library... a green gate is a precondition for looking at these, never a substitute," `../initiative.md:356-358`). What catches it: a reviewer diffing this story's fourteen artefact-path citations against the sibling ledgers they resemble — if every single one points at a pre-existing file with a timestamp before this project's own clean-checkout run, none of them was actually re-observed.

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
