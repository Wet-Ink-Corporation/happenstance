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
  annotated.
depends_on: []
related:
  - kb-map-open-questions-index-001
source_paths:
  - .kb/_governance/integration-waves/2026-08-10-intake/01-claims-and-classification.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
last_reviewed: 2026-08-10
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
`kb-open-question-ps-1-no-progress-obligation-001`,
`kb-open-question-ps-19-scope-narrower-001`,
`kb-open-question-es-6-unwritable-rule-001`,
`kb-open-question-provisional-falsifiers-001`,
`kb-open-question-post-phase-reconciliation-001`.

## Adding a domain

Append a new `##` section rather than editing this one — a domain is a
subject area, not a wave, and sections should outlive the ingest that first
populated them. Group entries within a section by kind, cite each atom's id
next to its link, and keep the orientation line to one sentence: the atom
itself is the source of truth, not this map.
