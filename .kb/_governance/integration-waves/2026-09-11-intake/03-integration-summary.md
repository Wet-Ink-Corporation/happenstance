# Wave `2026-09-11-intake` — integration summary

Five staged files, fifteen operations, ordered and adjudicated in
`02-placement-and-adjudication.md`. This file states the result: what exists
now that did not before, what changed in place, and which map atoms carry the
wiring.

## Atoms created (7)

| Atom | Kind | id | Sources |
| --- | --- | --- | --- |
| `.kb/reference/host-clocksource-tsc-vs-hpet-2026-09.md` | reference | `kb-reference-host-clocksource-tsc-hpet-001` | `host` |
| `.kb/playbooks/a-control-that-can-fire-on-the-instrument.md` | playbook | `kb-playbook-control-fires-on-instrument-001` | `host` |
| `.kb/decisions/0064-the-measurement-host-has-declared-conditions.md` | decision | `kb-decision-0064` | `host` |
| `.kb/decisions/0062-the-probe-seam-moves-and-the-far-end-is-built.md` | decision | `kb-decision-0062` | `adr-0062` |
| `.kb/decisions/0063-the-projection-port-is-frozen.md` | decision | `kb-decision-0063` | `adr-0063` |
| `.kb/open-questions/projection-apply-is-synchronous-against-a-live-store.md` | open_question | `kb-open-question-apply-synchronous-live-store-001` | `adr-0062`, `adr-0063` |
| `.kb/open-questions/accepted-atom-immutability-check-is-pre-commit-only.md` | open_question | `kb-open-question-immutability-check-pre-commit-001` | `citation-scan` |

`0064` takes highest-taken + 1; `0062` and `0063` take the numbers their own
brief titles already carry — see Adjudication 1 of `02`. `0026`–`0028` remain
reserved and unclaimed.

Two of the seven are decision atoms minted for decisions this worktree cannot
verify against its own tree at `86a410c` (`kb-decision-0062`, `kb-decision-0063`
— the far-end store, the call-site counts and the census delta are all cited
as **the brief's** statement, never as this wave's verification; see
Adjudication 1 and Adjudication 5). Nothing in either atom is false at `HEAD`:
each states what the tree carries today, names the lane the brief's work sits
on, and attributes every unverifiable count to the brief rather than to this
wave's own observation.

## Atoms amended (8)

| Atom | id | What changed |
| --- | --- | --- |
| `.kb/open-questions/probe-read-through-signature-and-live-transaction-seam.md` | `kb-open-question-probe-read-through-signature-001` | **resolved** — `status: accepted` → `superseded` (no `superseded_by`, a decision closed it); a dated resolution section records that ADR-0062 moved the whole seam and `begin` with it, wider than the atom's own recommendation |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | `kb-open-question-provisional-falsifiers-001` | a third arm added to the falsifier taxonomy — PS-6's falsifier was made fireable by moving the signature rather than the marker, and then fired and was acted on; records that the "thirteen clauses on PS-2 alone" was the RUNBOOK ledger's simplification, narrowed by ADR-0063's clause-by-clause disposition |
| `.kb/open-questions/testkit-projection-module-unstable-projection-exemption-scope.md` | `kb-open-question-projection-module-exemption-scope-001` | annotated, not resolved — ADR-0063 removes the exemption the question was scoped to, but the brief's claim that the testkit no longer forwards the retired feature is unverifiable here (`crates/happenstance-testkit/Cargo.toml` still forwards it at `86a410c`) |
| `.kb/open-questions/cf-33-cf-34-scope-outside-the-testkit.md` | `kb-open-question-cf-33-cf-34-scope-001` | a third clock-adjacent assertion added (`preflight.sh`) — the first built to be structurally unreachable from the gate, with the named path to becoming a violation (a self-hosted runner) recorded as the brief's own residual-risk sentence |
| `.kb/open-questions/docs-citation-anchor-form-and-clause-contradiction-check.md` | `kb-open-question-docs-citation-anchor-contradiction-001` | records that the predicted event (an uncaught contradiction) arrived at ingest, not at ratification as the postscript argued; records the chosen repair (re-anchor at promotion, owned by the redkiln plugin, not this repository) and the interim habit (`cargo xtask ci` immediately after a wave merges); records a fifth citation-drift instance found in this wave's own staged files |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | `kb-governance-referent-not-reasoning-001` | a fifth worked instance added — a line-number repair (`kb-decision-0058:33`) refused by the immutability check on the uncommitted tree and passed once committed, the first instance in which the rule met its own enforcement and the check could not tell a referent repair from a reversal |
| `.kb/playbooks/require-the-property-not-the-mechanism.md` | `kb-playbook-require-property-not-mechanism-001` | a second instance added, from the specification rather than from `xtask`: PS-6's MUST named a mechanism (a synchronous, infallible signature); when the mechanism moved, the property it stood for (no round trip, resolves at first poll, never fails) was held by two named tests and the MUST was rewritten to state it |
| `.kb/open-questions/stale-0-0-0-name-reservations.md` | `kb-open-question-stale-0-0-0-name-reservations-001` | one dated paragraph added — the 2026-09-08 registry read (seven crates at `0.0.0`, three additionally at `0.2.0-alpha.1`, nothing yanked) that `CLAUDE.md`'s MSRV section was itself corrected against; no decision-layer operation, since `kb-decision-0037` already carries the file's proposition with correct wiring |

## Decision corpus: nothing destructive

Three decision atoms added; zero accepted bodies edited; zero `status` flips
on an existing atom; zero supersessions. Two accepted decisions,
`kb-decision-0036` and `kb-decision-0060`, are **discharged** by
`kb-decision-0063` on the shape `kb-decision-0037` set against `kb-decision-0004`:
each was conditional on PS-2's bar (PS-3's SHOULD — "until PS-2's bar is
met"), ADR-0062 met it, and a conditional decision whose condition arrives has
run its course rather than been found wrong. `supersedes: null` on 0063,
`superseded_by: null` on 0036 and 0060, zero changed bytes in either
predecessor — see Adjudication 5 of `02`. One accepted decision,
`kb-decision-0058`, was repaired in place (a referent-only line-number fix)
**before** this wave, under the same governance rule; the wave records that
repair as the fifth worked instance of the rule rather than repeating or
reversing it.

## Map atoms updated (3)

- **`maps/decision-map.md`** — three new rows (ADR-0064, ADR-0062, ADR-0063,
  all phase 12 except 0064 at phase `null`) and two "discharged by ADR-0063"
  annotations against the existing rows for `kb-decision-0036` and
  `kb-decision-0060`.
- **`maps/domain-map.md`** — the projection-port area gains ADR-0062,
  ADR-0063 and the resolved probe open question; the measurement/host area is
  new this wave and gains ADR-0064, the clocksource reference and the control
  playbook; the conformance/specification-governance area gains the `apply`
  open question and the immutability-check open question.
- **`maps/open-questions-index.md`** — two new bullets (the two deferred open
  questions) and six changed bullets (one flipped from Open to
  "Resolved by ADR-0062", five annotated in place). No bullet removed —
  nothing withdrawn this wave beyond the one resolution.

## Links wired

Every `create_new` and `defer_open_question` atom's `related`/`depends_on`
list points only at atoms that exist by the time it is written — the ordering
in `02-placement-and-adjudication.md` (reference, playbook, decision;
decisions before the open questions that cite them; merges last) guarantees
this. Every `merge_existing` atom gains at least one new `related` id pointing
at an atom this wave created, and every existing `related`/`source_paths`
entry on a merged atom is carried forward untouched, per `KbFrontmatter`'s
passthrough contract. Reciprocal backlinks from earlier new atoms to later new
atoms in the same wave (e.g. the clocksource reference to `kb-decision-0064`)
are the maps' responsibility and are carried on the map rows rather than by
editing the earlier atom's `related` list after the fact.

## Intake cleared

Four of the five staged files removed from `.kb/_intake/` on success (step 3
of this wave): `2026-09-08-adr-0004-msrv-becomes-a-promise-at-publication.md`,
`2026-09-09-the-measurement-host-and-its-clock.md`,
`2026-09-10-adr-0062-the-probe-seam-moves.md`, and
`2026-09-11-adr-0063-the-projection-port-is-frozen.md`. The fifth,
`2026-09-08-intake-is-outside-the-citation-scan.md`, is **not** cleared —
it was not on the orchestrator's clear list for this run even though this
wave used it as a source for ops 7, 12 and 13. See `04-retrospective.md`.
