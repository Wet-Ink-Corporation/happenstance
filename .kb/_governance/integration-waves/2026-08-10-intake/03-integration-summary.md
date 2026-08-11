# Wave `2026-08-10-intake` — integration summary

Twelve operations executed exactly as adjudicated in `02-placement-and-adjudication.md`. Twelve
atoms created, zero amended, zero superseded — the corpus held no decision atoms to amend or
supersede against, and this wave minted none (see Adjudications in `02`).

## Atoms created

| # | destPath | id | kind | authority_tier |
| --- | --- | --- | --- | --- |
| 1 | `.kb/reference/phase-4-5-specification-reconciliation-census.md` | `kb-reference-phase-4-5-spec-reconciliation-001` | reference | note |
| 2 | `.kb/playbooks/verify-the-referent-and-report-coverage.md` | `kb-playbook-verify-referent-report-coverage-001` | playbook | guideline |
| 3 | `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md` | `kb-playbook-anchoring-citations-001` | playbook | guideline |
| 4 | `.kb/playbooks/landing-a-stricter-gate-without-a-red-baseline.md` | `kb-playbook-ratchet-gate-landing-001` | playbook | guideline |
| 5 | `.kb/playbooks/repairing-a-frozen-clause-without-amending-it.md` | `kb-playbook-repair-frozen-clause-001` | playbook | guideline |
| 6 | `.kb/open-questions/disjoint-boundaries-have-no-clause.md` | `kb-open-question-disjoint-boundaries-no-clause-001` | open_question | note |
| 7 | `.kb/open-questions/model-family-rule-has-no-clause.md` | `kb-open-question-model-family-rule-no-clause-001` | open_question | note |
| 8 | `.kb/open-questions/ps-1-states-no-progress-obligation.md` | `kb-open-question-ps-1-no-progress-obligation-001` | open_question | note |
| 9 | `.kb/open-questions/ps-19-scope-narrower-than-its-rule.md` | `kb-open-question-ps-19-scope-narrower-001` | open_question | note |
| 10 | `.kb/open-questions/es-6-names-an-unwritable-rule.md` | `kb-open-question-es-6-unwritable-rule-001` | open_question | note |
| 11 | `.kb/open-questions/es-7-and-vt-9-provisional-markers.md` | `kb-open-question-provisional-falsifiers-001` | open_question | note |
| 12 | `.kb/open-questions/nothing-owns-the-post-phase-reconciliation.md` | `kb-open-question-post-phase-reconciliation-001` | open_question | note |

By kind: 1 reference, 4 playbook, 7 open_question. By status: all 12 `accepted`. Zero `aligns`-only
atoms were skipped — the four `aligns`-labelled claims (see `01`) each still produced or fed an
atom, because each supplied a worked procedure or evidence the existing governance prose lacked
room for, not a bare restatement.

## Atoms amended or superseded

None. `.kb/decisions/` remains empty this wave — see `02`'s Adjudications section for why no
decision atom was authored despite thirteen claims labelled `requires-new-decision`.

## Map atoms created and updated

Two `map` atoms did not exist before this wave and were created to receive it, per `02`'s "What
the Maps phase inherits":

- **`.kb/maps/domain-map.md`** (`kb-map-domain-001`, new) — first wave. One domain established,
  "Specification governance & conformance", holding the reference atom, all four playbooks, and a
  pointer to all seven open questions in the open-questions index.
- **`.kb/maps/open-questions-index.md`** (`kb-map-open-questions-index-001`, new) — first wave. One
  domain section, all seven questions listed `Open`, each with a one-line orientation and a link
  to the census atom for full grounding.

## Links wired

**Outbound**, authored at creation time per the plan in `02` (every link points to an
earlier-created atom, so creation could run without dangling references):

- Op 2 → Op 1 (`related`)
- Op 3 → Op 2 (`depends_on`), → Op 1 (`related`)
- Op 4 → Op 1 (`related`)
- Op 5 → Op 1, Op 4 (`related`)
- Op 6 → Op 1, Op 4, Op 5 (`related`)
- Op 7 → Op 1, Op 4, Op 6 (`related`)
- Op 8 → Op 1, Op 5 (`related`)
- Op 9 → Op 1, Op 5, Op 8 (`related`)
- Op 10 → Op 1, Op 5 (`related`)
- Op 11 → Op 1, Op 5 (`related`)
- Op 12 → Op 1, Op 4, Op 8, Op 9, Op 11 (`related`)

**Inbound (reciprocal backlinks)**, wired by the Maps phase as `02` specified: Op 1 → Ops 6–12
(via the domain map's reference entry and the open-questions index), Op 5 → Ops 8, 9, 11 (via the
domain map's playbook entry), Op 4 → Ops 6, 7, 12 (via the domain map's playbook entry), Op 2 → Op
3 (via the domain map's playbook entry). Both map atoms carry `related: [the other map atom's id]`,
closing the pair.

`decisionMap`: no impact, as `02` predicted — zero decision atoms in this wave.

## Validation

`redkiln validate --kb` — pass (exit 0). `redkiln doctor` — pass (exit 0; six unrelated template-drift
warnings on `.redkiln/templates/*`, not KB findings). No duplicate atom ids, no dangling
`depends_on`/`related`/`supersedes`/`superseded_by` references, no `KbFrontmatter` violations, no
accepted-decision immutability violations (none exist to violate).
