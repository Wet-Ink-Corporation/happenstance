# Wave `2026-08-10-intake-2` — integration summary

Thirty-eight operations executed exactly as adjudicated in `02-placement-and-adjudication.md`.
Thirty-six atoms created, two amended (`merge_existing`), zero superseded as a metadata flip —
`.kb/decisions/` was empty going in, so ADR-0002 arrives already `status: superseded` at creation
rather than via a flip (see `02`'s Adjudications).

## Atoms created

| # | destPath | id | kind | authority_tier |
| --- | --- | --- | --- | --- |
| 1 | `.kb/reference/port-traits-compiled-findings.md` | `kb-reference-port-traits-compiled-findings-001` | reference | note |
| 2 | `.kb/reference/position-visibility-experiment-2026-08.md` | `kb-reference-position-visibility-experiment-001` | reference | note |
| 3 | `.kb/reference/wire-format-encoding-measurements.md` | `kb-reference-wire-format-measurements-001` | reference | note |
| 4 | `.kb/concepts/torn-reads-and-the-append-condition-boundary.md` | `kb-concept-torn-read-append-boundary-001` | concept | note |
| 5 | `.kb/decisions/0001-async-port-flavours.md` | `kb-decision-0001` | decision | decision |
| 6 | `.kb/decisions/0002-crate-naming.md` | `kb-decision-0002` | decision | decision |
| 7 | `.kb/decisions/0003-opaque-payloads.md` | `kb-decision-0003` | decision | decision |
| 8 | `.kb/decisions/0004-edition-and-msrv.md` | `kb-decision-0004` | decision | decision |
| 9 | `.kb/decisions/0005-rename-to-happenstance.md` | `kb-decision-0005` | decision | decision |
| 10 | `.kb/decisions/0006-bare-name-to-the-typed-layer.md` | `kb-decision-0006` | decision | decision |
| 11 | `.kb/decisions/0007-projection-runner-decodes.md` | `kb-decision-0007` | decision | decision |
| 12 | `.kb/decisions/0008-one-derivation-for-both-ports.md` | `kb-decision-0008` | decision | decision |
| 13 | `.kb/decisions/0009-error-send-sync.md` | `kb-decision-0009` | decision | decision |
| 14 | `.kb/decisions/0010-the-suite-must-prove-itself.md` | `kb-decision-0010` | decision | decision |
| 15 | `.kb/decisions/0011-read-laziness-and-isolation.md` | `kb-decision-0011` | decision | decision |
| 16 | `.kb/decisions/0012-append-shape-and-preconditions.md` | `kb-decision-0012` | decision | decision |
| 17 | `.kb/decisions/0013-position-assignment-and-visibility.md` | `kb-decision-0013` | decision | decision |
| 18 | `.kb/decisions/0014-event-identity-and-recorded-time.md` | `kb-decision-0014` | decision | decision |
| 19 | `.kb/decisions/0015-validated-identifiers-and-store-limits.md` | `kb-decision-0015` | decision | decision |
| 20 | `.kb/decisions/0016-the-wire-format.md` | `kb-decision-0016` | decision | decision |
| 21 | `.kb/decisions/0029-msrv-raised-to-1-97-1.md` | `kb-decision-0029` | decision | decision |
| 22 | `.kb/governance/rewrite-the-referent-never-the-reasoning.md` | `kb-governance-referent-not-reasoning-001` | governance | guideline |
| 23 | `.kb/playbooks/one-decision-per-adr-title.md` | `kb-playbook-one-decision-per-adr-title-001` | playbook | guideline |
| 24 | `.kb/playbooks/testing-interleavings-with-cold-futures.md` | (playbook, ADR-0010's mutant-testing method) | playbook | guideline |
| 25–38 | `.kb/open-questions/{adr-status-vocabulary-exceeds-the-schema, projection-store-batch-has-no-apply-seam, query-union-rule-is-owed-and-unowned, global-versus-per-boundary-visibility-invariant, postgres-arm-c-structural-cost, poll-count-bounds-the-visibility-rule, es-38-and-gap-read-rules-are-unowned, projection-id-is-unvalidated, cf-40-fixture-limits-ownership, dcb-reference-publishes-no-wire-format, human-readable-payload-encoding-on-a-constrained-peer, sync-message-set-and-format-version}.md` | twelve distinct ids, one per `defer_open_question` disposition | open_question | note |

By kind: 3 reference, 1 concept, 17 decision, 1 governance, 2 playbook, 12 open_question — 36
created. By status: 16 decisions `accepted`, 1 (`kb-decision-0002`) `superseded`; all 19
non-decision creates `accepted`.

## Atoms amended

Two `open_question` atoms, both `merge_existing`, both amended by links plus a paragraph with no
existing sentence deleted (`02`, Ops 25–26):

- **`.kb/open-questions/es-6-names-an-unwritable-rule.md`** — the sentence recording that ADR-0009
  was named but not imported is now false and was replaced with a link; added ADR-0008's
  constraint on the unwritten rule (assert on the future's `Output`, not the future). `related`
  gained `kb-decision-0008`, `kb-decision-0009`, `kb-reference-port-traits-compiled-findings-001`.
  Question stays open — the rule is now writable and still unwritten.
- **`.kb/open-questions/es-7-and-vt-9-provisional-markers.md`** — added that both markers now have
  named owners in the imported ADR corpus (ES-7 via ADR-0001/ADR-0008, VT-9 via ADR-0014, owned by
  the phase-9 Workers skeleton). `related` gained `kb-decision-0001`, `kb-decision-0008`,
  `kb-decision-0014`, `kb-reference-port-traits-compiled-findings-001`. Question stays open —
  moving a maturity marker is an ADR's act, not an ingest wave's.

Four further atoms picked up reciprocal `related` backlinks from the Maps phase without a body
change (not counted as amendments — links only, per `02`'s "What the Maps phase inherits"):
`ps-1-states-no-progress-obligation.md` and `ps-19-scope-narrower-than-its-rule.md` each gained
`kb-open-question-projection-batch-no-apply-001`; `repairing-a-frozen-clause-without-amending-it.md`
gained five backlinks into the new decision/governance/open-question corpus;
`phase-4-5-specification-reconciliation-census.md` gained `kb-decision-0013` and two new open
questions.

## Atoms superseded

None via the `supersede` operation type — the layer was empty going in. `kb-decision-0002` is
authored `status: superseded, superseded_by: kb-decision-0005` at creation, since the document it
names as its successor was written the same day, thirty-two minutes later, and both land in this
wave (`02`, Op 6 and Adjudications). This is the wave's one forward reference, declared in `02` so
it is not mistaken for a dangling id.

## Map atoms created and updated

- **`.kb/maps/decision-map.md`** (new) — one row per `decision` atom, the supersession chain
  (0002 → 0005 → 0006 → 0007), the amendment (0004 ← 0029), and the **partial** supersessions
  among 0005/0006/0007 carried as annotation prose, since the frontmatter pair is reserved for full
  supersession only (`02`, Standing choice 2).
- **`.kb/maps/domain-map.md`** (amended) — new sections appended to the existing "Specification
  governance & conformance" domain: the storage ports and their two flavours; the contract's value
  types and identity; read, append and position semantics; crate naming and packaging; the
  toolchain; the wire format; the conformance suite.
- **`.kb/maps/open-questions-index.md`** (amended) — twelve new bullets, one per deferred question;
  the two amended questions' index bullets updated to reflect their new `related` links.

## New layers

`.kb/concepts/` and `.kb/governance/` did not exist before this wave. Both created with a
`README.md` mirroring `reference/README.md`'s voice, per `02`'s Standing choice 5, before their
first atoms (Ops 4 and 22) landed.

## Links wired

**Outbound**, authored at creation time (every link points to an atom created by an earlier
operation in this wave, or — for Op 6's `superseded_by` — to an atom created later in the same
wave, the one declared exception): full per-operation detail is in `02`, "Ops 1–3" through "Ops
27–38".

**Inbound (reciprocal backlinks)**, wired by the Maps phase per `02`'s "What the Maps phase
inherits": each decision atom back to the open questions it spawned (0007→28, 0011→29,
0013→30/31/32/33, 0015→34, 0015+0012→35, 0016→36/37/38); Op 1 (port-traits findings) back to Ops
5, 12, 13, 14, 15; Op 4 (torn-read concept) back to Ops 15, 16, 17; plus the four cross-wave
backlinks onto `ps-1`, `ps-19`, `repairing-a-frozen-clause` and the phase-4/5 census listed above.

`decisionMap: true` on nineteen operations — the seventeen decision atoms, plus Ops 25 and 26
(ES-6 and the provisional-falsifiers question), whose amended `related` lists now reach into the
decision corpus.

## Validation

`redkiln validate --kb` — pass (exit 0). `redkiln doctor` — pass (exit 0; six template-drift
warnings on `.redkiln/templates/*`, unrelated to the KB — see `04-retrospective.md`). No duplicate
atom ids, no dangling `depends_on`/`related`/`supersedes`/`superseded_by` references, no
`KbFrontmatter` violations, no accepted-decision immutability violations against `HEAD`. Both
instruments passed on the first run; no repair loop was needed.
