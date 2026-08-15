---
id: kb-map-domain-001
title: Domain map
kind: map
status: accepted
authority_tier: note
summary: >-
  The corpus grouped by subject area rather than by kind or directory. One section per domain;
  within a section, atoms are grouped by kind (reference, playbook, open_question, ...) with a
  one-line orientation and a link. Updated by the Maps phase of every kb-ingest wave that adds a
  new canonical concept or domain path; entries are not removed when an atom is superseded, only
  annotated. The 2026-08-13 wave added ADR-0017–0019 (phase 6, ProjectionStore) to the existing
  "Contract ports, conformance, and the ADR corpus" domain and annotated one open question there
  as superseded. The 2026-08-15 wave added ADR-0030 and a new open question (PS-32) to that same
  domain, and a new reference atom on spec-trace's per-family suite switch to "Specification
  governance & conformance," annotating the PS-1 and PS-19 open questions in both places as
  superseded by ADR-0030.
depends_on: []
related:
  - kb-map-open-questions-index-001
  - kb-map-decision-001
source_paths:
  - .kb/_governance/integration-waves/2026-08-10-intake/01-claims-and-classification.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
  - .kb/_governance/integration-waves/2026-08-13-projection-adrs
  - .kb/_governance/integration-waves/2026-08-15-adr-0030-checkpoint-progress
last_reviewed: 2026-08-15
---

# Domain map

This is the corpus's subject-matter index. Where the directory layout
(`decisions/`, `playbooks/`, `reference/`, `open-questions/`, ...) groups
atoms by *kind*, this map groups them by *what they are about* — so a reader
who lands on one atom can find its neighbours without already knowing they
exist. See [`../maps/README.md`](README.md) for what belongs on a map atom
and what does not; see
[`open-questions-index.md`](open-questions-index.md) for the companion index
that this map does not duplicate.

This is the map's first wave. One domain exists so far.

## Specification governance & conformance

The area concerned with `spec/SPECIFICATION.md` itself — whether a clause's
prose, its assigned conformance rules, and the code agree, and what a pass
does when it finds they do not. Established by the 2026-08-10 phase 4/5
specification reconciliation, an unscheduled pass that read the specification
back against the tree phases 4 and 5 had already changed.

**Reference**

- [`phase-4-5-specification-reconciliation-census.md`](../reference/phase-4-5-specification-reconciliation-census.md)
  (`kb-reference-phase-4-5-spec-reconciliation-001`) — the census and pointer:
  what the pass found, counted by defect class, and where the full evidence
  lives. Everything else in this domain cites this atom rather than
  restating its counts.
- [`spec-trace-has-suite-family-switch.md`](../reference/spec-trace-has-suite-family-switch.md)
  (`kb-reference-spec-trace-has-suite-001`) — `cargo xtask spec-trace`'s
  citation check runs only against clause families `has_suite` admits; `PS`
  sat outside that switch for two slices after its conformance suite was
  written, with every gate green throughout. Added 2026-08-15.

**Playbooks** — transferable practice this pass extracted

- [`verify-the-referent-and-report-coverage.md`](../playbooks/verify-the-referent-and-report-coverage.md)
  (`kb-playbook-verify-referent-report-coverage-001`) — a cross-reference
  checker must verify the referent, not just the address, and must report
  its own coverage.
- [`anchoring-citations-in-a-long-lived-document.md`](../playbooks/anchoring-citations-in-a-long-lived-document.md)
  (`kb-playbook-anchoring-citations-001`) — how to check `file:line`
  citations against a source tree that moves, without a checker that either
  passes forever or fires on every ordinary edit.
- [`landing-a-stricter-gate-without-a-red-baseline.md`](../playbooks/landing-a-stricter-gate-without-a-red-baseline.md)
  (`kb-playbook-ratchet-gate-landing-001`) — what to do when tightening a
  gate check surfaces violations nobody is authorised to fix in the same
  change: the ratchet, and when it is the wrong instrument.
- [`repairing-a-frozen-clause-without-amending-it.md`](../playbooks/repairing-a-frozen-clause-without-amending-it.md)
  (`kb-playbook-repair-frozen-clause-001`) — the mechanical test for whether
  a correction to a `[FROZEN]` clause is a repair or a gap, and the safe form
  for each.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md)
for the full, self-contained list. The ones this domain owns:
`kb-open-question-disjoint-boundaries-no-clause-001`,
`kb-open-question-model-family-rule-no-clause-001`,
`kb-open-question-ps-1-no-progress-obligation-001` (**superseded** 2026-08-15
by `kb-decision-0030`; sub-questions 1, 2 and 4 answered, PS-1's own text
byte-identical),
`kb-open-question-ps-19-scope-narrower-001` (**superseded** 2026-08-15 by
`kb-decision-0030`; sub-questions 1 and 3 answered, sub-question 2's
2026-08-13 verdict stands),
`kb-open-question-es-6-unwritable-rule-001`,
`kb-open-question-provisional-falsifiers-001`,
`kb-open-question-post-phase-reconciliation-001`.

## Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)

The area concerned with `happenstance-core`'s async port design (`EventStore`,
`ProjectionStore`), the conformance suite that proves an adapter against it, and the
seventeen-ADR decision record — async port flavours through the wire format — that this
project's early phases rest on. Established by the 2026-08-10 ADR import, which brought
`.kb/decisions/0001` through `.kb/decisions/0016` and `.kb/decisions/0029` into `.kb/decisions/` as one wave.
The 2026-08-13 wave added three more to this same domain — `.kb/decisions/0017`, `0018` and
`0019` — settling `ProjectionStore::Batch`'s ownership, checkpoint reset, and the port's
no-op-on-`apply`-failure stance, phase 6's `ProjectionStore` freeze. The 2026-08-15 wave added a
fourth, `.kb/decisions/0030`, minting `[PROVISIONAL]` clause PS-38 — a successful `commit` MUST
advance its `ProjectionId`'s checkpoint, and an id no successful `commit` has named MUST read as
`Checkpoint::NeverRun` — the progress obligation section 4 never stated. The full decision list,
including status and supersession, is [`decision-map.md`](decision-map.md) rather than repeated
here.

**Reference**

- [`port-traits-compiled-findings.md`](../reference/port-traits-compiled-findings.md)
  (`kb-reference-port-traits-compiled-findings-001`) — what compiling `EventStore` and
  `ProjectionStore`, rather than reasoning about them, found across ADR-0001, ADR-0008,
  ADR-0009, ADR-0010 and ADR-0011.
- [`position-visibility-experiment-2026-08.md`](../reference/position-visibility-experiment-2026-08.md)
  (`kb-reference-position-visibility-experiment-001`) — the four-arm measurement against real
  PostgreSQL that ADR-0013's visibility invariant rests on.
- [`wire-format-encoding-measurements.md`](../reference/wire-format-encoding-measurements.md)
  (`kb-reference-wire-format-measurements-001`) — the encoding-size measurements ADR-0016 rests
  on, plus two instruments that measured wrong.

**Concepts**

- [`torn-reads-and-the-append-condition-boundary.md`](../concepts/torn-reads-and-the-append-condition-boundary.md)
  (`kb-concept-torn-read-append-boundary-001`) — why the append condition, derived from the
  read's own observed maximum, cannot catch a torn read; the mechanism ADR-0011, ADR-0012 and
  ADR-0013 each protect without stating.

**Governance**

- [`rewrite-the-referent-never-the-reasoning.md`](../governance/rewrite-the-referent-never-the-reasoning.md)
  (`kb-governance-referent-not-reasoning-001`) — the discrimination between a rename that may be
  rewritten in place and reasoning that, once a decision stands, is never touched, worked out
  against ADR-0001 through ADR-0007.

**Playbooks**

- [`one-decision-per-adr-title.md`](../playbooks/one-decision-per-adr-title.md)
  (`kb-playbook-one-decision-per-adr-title-001`) — an "and" in a decision's title is usually a
  strong decision and a weaker one bundled together, and the weaker half is the one likely to be
  reversed.
- [`testing-interleavings-with-cold-futures.md`](../playbooks/testing-interleavings-with-cold-futures.md)
  (`kb-playbook-cold-future-hand-polling-001`) — hand-polling two cold futures out of order to
  make a conformance rule observe a specific interleaving with no executor, thread or clock.

**Open questions** — see [`open-questions-index.md`](open-questions-index.md) for the full,
self-contained list. The ones this domain owns:
`kb-open-question-adr-status-vocabulary-001`,
`kb-open-question-projection-batch-no-apply-001` (**superseded** 2026-08-13 by
`kb-decision-0017`; sub-question 3 stays open, with the typed layer),
`kb-open-question-query-union-rule-unowned-001`,
`kb-open-question-global-vs-boundary-visibility-001`,
`kb-open-question-postgres-arm-c-cost-001`,
`kb-open-question-poll-count-rule-strength-001`,
`kb-open-question-es-38-and-gap-read-unowned-001`,
`kb-open-question-projection-id-unvalidated-001`,
`kb-open-question-cf-40-ownership-001`,
`kb-open-question-dcb-no-published-format-001`,
`kb-open-question-human-readable-encoding-limits-001`,
`kb-open-question-sync-message-set-undesigned-001`,
`kb-open-question-ps-32-adr-0007-correction-owed-001` (added 2026-08-15 — ADR-0007's Context
overstates what cannot be written against the port; only a superseding atom may correct it).

## Adding a domain

Append a new `##` section rather than editing this one — a domain is a
subject area, not a wave, and sections should outlive the ingest that first
populated them. Group entries within a section by kind, cite each atom's id
next to its link, and keep the orientation line to one sentence: the atom
itself is the source of truth, not this map.
