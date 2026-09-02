# Wave `2026-09-02-intake` — integration summary

Seven staged files, thirteen operations, twelve atoms created, one existing open question
resolved. Full reasoning is in `02-placement-and-adjudication.md`; this is the roll-up.

## Atoms created (12)

| Atom | Kind | Layer | From |
| --- | --- | --- | --- |
| `kb-open-question-adapter-default-projection-feature-001` | open_question | `.kb/open-questions/` | `ps-3-projection-port-ships-gated.md` |
| `kb-decision-0035` (ADR-0035) | decision | `.kb/decisions/` | `0035-async-trait-through-worker.md` |
| `kb-decision-0036` (ADR-0036) | decision | `.kb/decisions/` | `ps-3-projection-port-ships-gated.md` |
| `kb-playbook-declared-page-need-001` | playbook | `.kb/playbooks/` | `lesson-page-need-declaration-discipline.md` |
| `kb-open-question-trademark-search-001` | open_question | `.kb/open-questions/` | `brand-identity-commitments.md` + `brand-where-the-identity-lives.md` |
| `kb-reference-brand-geometry-palette-001` | reference | `.kb/reference/` | `brand-identity-commitments.md` + `brand-symbol-wordmark-lockup-pattern.md` |
| `kb-design-symbol-annotates-the-wordmark-001` | concept (`authority_tier: design`) | `.kb/design/` | `brand-symbol-wordmark-lockup-pattern.md` |
| `kb-design-radial-mark-collisions-001` | concept (`authority_tier: design`) | `.kb/design/` | `brand-symbol-wordmark-lockup-pattern.md` + `brand-identity-commitments.md` |
| `kb-decision-sd-0001` (SD-0001) | decision | `.kb/decisions/` | `brand-the-name-and-what-it-means.md` |
| `kb-decision-sd-0002` — standalone-SVG delivery rule | decision | `.kb/decisions/` | `brand-where-the-identity-lives.md` |
| `kb-decision-sd-0002` — the mark and its rules | decision | `.kb/decisions/` | `brand-where-the-identity-lives.md` |
| `kb-reference-brand-identity-source-locations-001` | reference | `.kb/reference/` | `brand-where-the-identity-lives.md` |

`.kb/design/` receives its first two atoms this wave, making it a populated layer rather than a
scaffolded one. `.kb/decisions/` gains its first `SD-`-numbered records (SD-0001, SD-0002 split
across two atoms sharing one `adr_id` — see Adjudication 7), alongside the two new ADR-numbered
ones (0035, 0036).

## Atom amended (1)

`.kb/open-questions/deny-bans-red-on-the-worker-dependency.md` — resolved. Frontmatter-only
hunks (`status: accepted → superseded`, `related` gains `kb-decision-0035`, `source_paths` and
`last_reviewed` updated) plus one appended `## Resolved 2026-09-02` section. The pre-existing
body is untouched, per `.kb/open-questions/README.md`'s resolution procedure. No `superseded_by`
was set — that key is decision-only.

## Map atoms updated (3)

- `.kb/maps/decision-map.md` — rows for ADR-0035, ADR-0036, SD-0001 and both SD-0002 atoms; a new
  section for `SD-`-numbered records alongside the ADR-numbered table.
- `.kb/maps/domain-map.md` — a new **brand identity** domain (four atoms) and a new
  **documentation & prose standards** domain (one playbook); the existing contract-ports-&-
  conformance domain gains the two new decisions and the one new open question.
- `.kb/maps/open-questions-index.md` — rows added for both new open questions; the
  `deny-bans-red-on-the-worker-dependency` row updated to `superseded`.

## Reciprocal links wired

Per the wave's outbound-only authoring convention, the Maps phase wired the return edges:

- `kb-decision-0001` ← no edge (deliberate — Op 2 is an amendment via `depends_on`, and the
  amended atom stays byte-identical per `kb-decision-0029`'s precedent).
- `kb-open-question-worker-async-trait-ban-001` (now `deny-bans-red-on-the-worker-dependency`) —
  `related` gains `kb-decision-0035` (the amendment above, done directly as part of Op 4).
- `kb-playbook-verify-referent-report-coverage-001` — `related` gains
  `kb-playbook-declared-page-need-001` (reciprocal of that atom's `depends_on`).
- `kb-governance-referent-not-reasoning-001` — `related` gains
  `kb-playbook-declared-page-need-001`.
- `kb-playbook-anchoring-citations-001`, `kb-playbook-ratchet-gate-landing-001` — each gains the
  reciprocal `related` edge back to `kb-playbook-declared-page-need-001`.
- `kb-concepts-torn-reads-and-the-append-condition-boundary` — reciprocal edge from the new
  decision-map wiring (see that file's diff).

## Intake cleared

All seven staged files are removed from `.kb/_intake/` in this commit:
`0035-async-trait-through-worker.md`, `ps-3-projection-port-ships-gated.md`,
`lesson-page-need-declaration-discipline.md`, `brand-the-name-and-what-it-means.md`,
`brand-identity-commitments.md`, `brand-symbol-wordmark-lockup-pattern.md`,
`brand-where-the-identity-lives.md`.

## Validation

`redkiln validate --kb` exited 0. `redkiln doctor --json` reported two `unconsumed-foundation`
problems, both under `.bklg/from-contract-to-published-library/`, outside this wave's `.kb` path
set — see `04-retrospective.md` for the full accounting. No problem this wave's own writes could
have caused was reported by either instrument.
