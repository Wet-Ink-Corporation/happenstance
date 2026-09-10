# Wave `2026-09-09-intake` — integration summary

Five staged files, fourteen operations, ordered and adjudicated in
`02-placement-and-adjudication.md`. This file states the result: what exists
now that did not before, what changed in place, and which map atoms carry the
wiring.

## Atoms created (6)

| Atom | Kind | id | Sources |
| --- | --- | --- | --- |
| `.kb/reference/ladybug-driver-probes-2026-09.md` | reference | `kb-reference-ladybug-driver-probes-001` | ADR-0025 brief |
| `.kb/decisions/0060-ps-2s-axis-re-evaluated.md` | decision | `kb-decision-0060` | ADR-0060 brief + PS-2 finding |
| `.kb/decisions/0061-es-11s-sufficiency-condition-assumed-a-queue.md` | decision | `kb-decision-0061` | ADR-0061 brief + ES-11 finding |
| `.kb/decisions/0025-the-ladybug-projection-adapter.md` | decision | `kb-decision-0025` | ADR-0025 brief |
| `.kb/open-questions/one-shot-http-conformance-to-es-11.md` | open_question | `kb-open-question-one-shot-http-es-11-001` | ADR-0061 brief + ES-11 finding |
| `.kb/open-questions/experiment-raw-output-eaten-by-the-ignore-rule.md` | open_question | `kb-open-question-experiment-raw-output-ignored-001` | wave-discovered (Op 6, from provenance check against ADR-0025/ADR-0061) |

`0025` takes the number `RUNBOOK.md`:392 already reserved for it, not the next
free integer — see Adjudication 1. `0026`–`0028` remain reserved and unclaimed.

Two of the six sources are cross-file merges the corpus itself performed
before this wave read it: `kb-decision-0060` folds one finding (`ps-2`) into
one decision (`adr-0060`), and `kb-decision-0061` folds one finding (`es-11`)
into one decision (`adr-0061`). Neither finding gets a second, separate atom —
minting one would file a live question over a question the corresponding
decision atom has already settled.

## Atoms amended (8)

| Atom | id | What changed |
| --- | --- | --- |
| `.kb/open-questions/probe-read-through-signature-and-live-transaction-seam.md` | `kb-open-question-probe-read-through-signature-001` | cause widened from scarcity to a compiled, two-mechanism forbiddance; scope widened from one method to the whole probe seam; records the suite cannot distinguish a declining live-transaction store from a buffering one |
| `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | `kb-open-question-provisional-falsifiers-001` | two more falsifiers-that-can-never-fire added (PS-4's Rust-level condition, PS-2's live-transaction end); observation sharpened to cover markers decorative *a priori*, not only in retrospect |
| `.kb/open-questions/es-11-ceiling-sample-cost-on-sqlite-read.md` | `kb-open-question-es-11-sqlite-ceiling-sample-cost-001` | sub-question 3 answered (the marker moves, from predicting a falsifier to recording one fired) — sub-questions 1 and 2 (the SQLite remedy) stay open |
| `.kb/open-questions/reset-refusal-declension-has-no-clause.md` | `kb-open-question-reset-refusal-declension-001` | the "empty population" premise updated: the first projection adapter (`happenstance-ladybug`) has now run the suite and declined `RESET_REFUSAL` as pre-registered; population stays empty, but the atom's own closing window has closed |
| `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | `kb-governance-referent-not-reasoning-001` | a fourth worked instance added — a decision stands while its stated reason expires (ADR-0060 against ADR-0036) |
| `.kb/governance/what-may-refute-a-finding.md` | `kb-governance-what-may-refute-a-finding-001` | a case added where the retracted claim is the *predecessor's*, and the transferable rule that a retracted claim does not poison its own neighbourhood |
| `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | `kb-playbook-repair-frozen-clause-001` | a case added where every textual signal says "repair" (MUST, marker, Rule and Cases untouched) but the admitted implementation set still moved, so it was correctly handled as an ADR |
| `.kb/playbooks/a-count-or-an-index-nobody-re-derives.md` | `kb-playbook-a-count-or-an-index-nobody-re-derives-001` | a third shape of decaying claim added — a status claim about an axis (not a cardinal number, not a curated index) — with four concrete passages the ES-11 falsifier made false |

## Decision corpus: nothing destructive

Three decision atoms added; zero accepted bodies edited; zero `status` flips;
zero supersessions. One accepted decision, `kb-decision-0036`, is amended
**from outside** by `kb-decision-0060` — its decision stands, only its stated
reason ("only one adapter has run the suite") is recorded as expired by an
adapter count that has since moved to four. `kb-decision-0036` stays
`accepted`, `superseded_by: null`, byte-identical. `kb-decision-0017` is
*applied* by `kb-decision-0025` and its PS-9/PS-11 falsifier is explicitly
**not** claimed as discharged — an adapter is evidence about a clause's cost,
not the data point the clause is waiting on.

## Map atoms updated (3)

- **`maps/decision-map.md`** — three new rows (ADR-0025 phase 11, ADR-0060
  phase 11, ADR-0061 phase 10) and a second "amended by" annotation on
  `kb-decision-0036`'s existing row.
- **`maps/domain-map.md`** — two sections extended: the projection-port area
  gains ADR-0025, ADR-0060 and the new reference atom; the
  conformance/specification-governance area gains ADR-0061, both new open
  questions, and the four transferable-practice merges.
- **`maps/open-questions-index.md`** — two new bullets (the deferred
  open questions) and four changed bullets (the four `merge_existing` open
  questions). No bullet struck — nothing resolved, withdrawn, or superseded
  this wave.

## Links wired

Every `create_new` and `defer_open_question` atom's `related`/`depends_on`
list points only at atoms that exist by the time it is written (the ordering
in `02-placement-and-adjudication.md` guarantees this — reference before
decision, decisions before open questions, merges last). Every `merge_existing`
atom gains at least one new `related` id pointing at an atom this wave created,
and every existing `related`/`source_paths` entry on a merged atom is carried
forward untouched, per `KbFrontmatter`'s passthrough contract.

## Intake cleared

All five staged files removed from `.kb/_intake/` on success (step 3 of this
wave).
