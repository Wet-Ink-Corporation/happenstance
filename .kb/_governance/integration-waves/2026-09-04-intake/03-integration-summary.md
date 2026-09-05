# Wave `2026-09-04-intake` — integration summary

Three staged files, fourteen operations, twelve atoms created, two existing atoms amended. Full
reasoning is in `02-placement-and-adjudication.md`; `00-corpus-match.md` establishes the corpus
state and the scoring behind every disposition. This is the roll-up.

## Atoms created (12)

| Atom | Kind | Layer | From |
| --- | --- | --- | --- |
| `kb-reference-event-clone-allocations-001` | reference | `.kb/reference/` | `2026-09-03-pre-publication-review.md` |
| `kb-reference-busy-timeout-margin-001` | reference | `.kb/reference/` | `2026-09-03-pre-publication-review.md` |
| `kb-reference-nested-block-on-lost-wakeup-001` | reference | `.kb/reference/` | `2026-09-03-pre-publication-review.md` |
| `kb-playbook-assert-execution-not-discovery-001` | playbook | `.kb/playbooks/` | `2026-09-03-pre-publication-review.md` |
| `kb-governance-what-may-refute-a-finding-001` | governance | `.kb/governance/` | `2026-09-03-pre-publication-review.md` |
| `kb-decision-0037` (ADR-0037) | decision | `.kb/decisions/` | `msrv-becomes-a-promise-at-0-2-0.md` |
| `kb-open-question-event-metadata-no-floor-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |
| `kb-open-question-read-page-budget-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |
| `kb-open-question-testkit-contention-tolerance-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |
| `kb-open-question-remint-precondition-trust-only-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |
| `kb-open-question-query-plan-parameter-chunking-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |
| `kb-open-question-adr-0022-falsifiers-fired-001` | open_question | `.kb/open-questions/` | `2026-09-03-pre-publication-review.md` |

`.kb/governance/` gains its second atom this wave (it has held exactly one since being
scaffolded). `.kb/decisions/` gains its first `ADR-0037`, an amendment-lineage decision that
supersedes nothing: ADR-0004 and ADR-0029 stay `status: accepted` and byte-identical, per
`02`'s Adjudication 10 — a claim `redkiln validate --kb` checks against `HEAD` rather than takes
on trust. `.kb/open-questions/` takes six new atoms in one wave, its largest single-wave addition.

## Atoms amended (2)

- `.kb/playbooks/anchoring-citations-in-a-long-lived-document.md`
  (`kb-playbook-anchoring-citations-001`) — extended, not merely linked: this corpus's first
  playbook merge (`02`, Adjudication 5). A new dated subsection records a second instance of the
  exact defect the playbook exists to prevent (a citation at `phase-7-contract-defects.md:10`
  that resolves but does not say what is claimed), the propagation mechanism (imitation across
  later evaluation documents, not edit), and a coverage statement (`standards/rust/` carries
  explicit anchors; `references/evaluation/` and `spec/` do not). `id`, `title`, `kind`, `status`,
  `authority_tier` and `depends_on` unchanged; no existing sentence reworded.
- `.kb/open-questions/projection-store-in-adapter-default-features.md`
  (`kb-open-question-adapter-default-projection-feature-001`) — resolved. `status: accepted` →
  `status: superseded`; body kept verbatim with one dated `## Resolved 2026-09-03` section
  appended, per `02`'s Adjudication 12. No `superseded_by` key added — that field is decision-only,
  matching this layer's two prior resolutions (`cf-40`, the `deny.toml` question).

## Map atoms updated (3)

- `.kb/maps/decision-map.md` — one new row for ADR-0037, in a new wave section (no existing
  section covered a phase-12 publication decision).
- `.kb/maps/domain-map.md` — the governance domain gains its second atom; the toolchain-and-edition
  domain gains its third; the contract-limits, SQLite-adapter, conformance-suite, identity-and-time
  and gate-and-tooling domains each gain the relevant open question or reference; the stale
  manifest description at `:245-248` is rewritten to the resolved form.
- `.kb/maps/open-questions-index.md` — six new bullets for the created open questions; the
  adapter-default-projection-feature bullet flips **Open → Resolved 2026-09-03**, its description
  rewritten off the now-stale manifest snippet to match the `deny.toml` resolution's established
  form.

## Reciprocal links wired

Per the wave's outbound-only authoring convention, the Maps phase wired the return edges into
eleven existing atoms:

- `kb-governance-referent-not-reasoning-001` — `related` gains `kb-governance-what-may-refute-a-
  finding-001` and `kb-decision-0037`.
- `kb-open-question-cf-40-ownership-001` — `related` gains `kb-open-question-event-metadata-no-
  floor-001`.
- `kb-open-question-es-17-two-adapter-measurement-001` — `related` gains
  `kb-reference-event-clone-allocations-001`, `kb-decision-0037` and
  `kb-open-question-adr-0022-falsifiers-fired-001`.
- `kb-open-question-poll-count-rule-strength-001` — `related` gains
  `kb-open-question-testkit-contention-tolerance-001`.
- `kb-open-question-query-union-rule-unowned-001` — `related` gains
  `kb-open-question-query-plan-parameter-chunking-001`.
- `kb-playbook-cold-future-hand-polling-001` — `related` gains
  `kb-reference-nested-block-on-lost-wakeup-001`.
- `kb-playbook-verify-referent-report-coverage-001` — `related` gains
  `kb-playbook-assert-execution-not-discovery-001` and
  `kb-governance-what-may-refute-a-finding-001`.
- `kb-reference-append-condition-experiment-001` — `related` gains
  `kb-reference-busy-timeout-margin-001` and `kb-open-question-adr-0022-falsifiers-fired-001`.
- `kb-reference-projection-fan-out-cost-001` — `related` gains
  `kb-open-question-read-page-budget-001`.
- `kb-reference-wf-11-memory-ceiling-verdict-001` — `related` gains
  `kb-open-question-read-page-budget-001`.
- `kb-reference-wire-format-measurements-001` — `related` gains
  `kb-reference-event-clone-allocations-001`.

No accepted decision atom's body was opened for writing at any point: five are named by incoming
claims (`kb-decision-0004`, `kb-decision-0029`, `kb-decision-0022`, `kb-decision-0015`,
`kb-decision-0036`) and none receives a `related`, `status`, or body edit — every one of them is
linked from the *new* atom's own `depends_on`/`related` list rather than amended in place.

## Intake cleared

All three staged files are removed from `.kb/_intake/` in this commit:
`2026-09-03-pre-publication-review.md`, `msrv-becomes-a-promise-at-0-2-0.md`,
`adapter-default-projection-feature-resolved.md`.

## Validation

`redkiln validate --kb` exited 0. `redkiln doctor --json` reported two `unconsumed-foundation`
problems, both under `.bklg/from-contract-to-published-library/`, outside this wave's `.kb` path
set — see `04-retrospective.md` for the full accounting. No problem this wave's own writes could
have caused was reported by either instrument.
