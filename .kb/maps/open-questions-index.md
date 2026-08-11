---
id: kb-map-open-questions-index-001
title: Open-questions index
kind: map
status: accepted
authority_tier: note
summary: >-
  One bullet per open_question atom in .kb/open-questions/, grouped by the domain it concerns, so
  an unresolved question is discoverable from the area it belongs to rather than only from the
  directory listing. Updated whenever a defer_open_question disposition lands a new atom, per
  open-questions/README.md's instruction to add a bullet "on the map atom that indexes its area."
  A withdrawn or superseded question stays listed, annotated, rather than removed — the record
  that it was once open is itself worth keeping.
depends_on: []
related:
  - kb-map-domain-001
source_paths:
  - .kb/open-questions/README.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
last_reviewed: 2026-08-10
---

# Open-questions index

Every `open_question` atom, current status first. See
[`../open-questions/README.md`](../open-questions/README.md) for what belongs
in that layer and how a question gets resolved; see
[`domain-map.md`](domain-map.md) for the subject-area grouping this index's
questions sit inside.

## Specification governance & conformance

All seven questions below were filed by the 2026-08-10 phase 4/5
specification-reconciliation intake wave. Full grounding for each is in the
[census reference atom](../reference/phase-4-5-specification-reconciliation-census.md)
(`kb-reference-phase-4-5-spec-reconciliation-001`); this index states only
what the question is, not its evidence.

- **Open** — [`disjoint-boundaries-have-no-clause.md`](../open-questions/disjoint-boundaries-have-no-clause.md)
  (`kb-open-question-disjoint-boundaries-no-clause-001`) — the independence
  proposition DCB exists for is enforced by a live conformance rule and
  stated by no clause.
- **Open** — [`model-family-rule-has-no-clause.md`](../open-questions/model-family-rule-has-no-clause.md)
  (`kb-open-question-model-family-rule-no-clause-001`) — the model-based
  rule checks the composition of seven clauses and belongs to none of them.
- **Open** — [`ps-1-states-no-progress-obligation.md`](../open-questions/ps-1-states-no-progress-obligation.md)
  (`kb-open-question-ps-1-no-progress-obligation-001`) — PS-1's `MUST` is a
  coupling, not a progress obligation; the third rule assigned to it does
  not follow from the sentence. Owned by phase 6.
- **Open** — [`ps-19-scope-narrower-than-its-rule.md`](../open-questions/ps-19-scope-narrower-than-its-rule.md)
  (`kb-open-question-ps-19-scope-narrower-001`) — PS-19's `MUST` is scoped
  to after a reset; its second assigned rule asks about an id never seen.
  Owned by phase 6; interacts with the PS-1 question above.
- **Open** — [`es-6-names-an-unwritable-rule.md`](../open-questions/es-6-names-an-unwritable-rule.md)
  (`kb-open-question-es-6-unwritable-rule-001`) — ES-6 is `[FROZEN]` and
  names a conformance rule that cannot be written against today's port.
- **Open** — [`es-7-and-vt-9-provisional-markers.md`](../open-questions/es-7-and-vt-9-provisional-markers.md)
  (`kb-open-question-provisional-falsifiers-001`) — ES-7 and VT-9 are
  `[PROVISIONAL]` and each names a falsifier that no longer discriminates.
- **Open** — [`nothing-owns-the-post-phase-reconciliation.md`](../open-questions/nothing-owns-the-post-phase-reconciliation.md)
  (`kb-open-question-post-phase-reconciliation-001`) — no phase carries an
  item obliging anyone to read the specification back against the tree a
  phase just changed. Forced by phase 6's exit and, secondarily, by first
  publish at phase 12. Depends conceptually on the PS-1, PS-19 and
  ES-7/VT-9 questions above.

## Adding an entry

Append the bullet under the domain section the question belongs to (see
[`domain-map.md`](domain-map.md) for the list of domains); start a new `##`
section only when the question's domain has no section yet. State the
status (`Open`, `Withdrawn`, `Superseded`) first, then the id and one
sentence — the atom itself carries the "what is true today / what is not
decided / what forces it" structure, this index does not repeat it.
