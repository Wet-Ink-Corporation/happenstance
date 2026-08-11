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
  - kb-map-decision-001
source_paths:
  - .kb/open-questions/README.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
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
  Amended 2026-08-10: ADR-0008 and ADR-0009 (`kb-decision-0008`,
  `kb-decision-0009`) are now imported and supply the marker the rule would
  name; the question is still open because neither assigns an owning phase.
- **Open** — [`es-7-and-vt-9-provisional-markers.md`](../open-questions/es-7-and-vt-9-provisional-markers.md)
  (`kb-open-question-provisional-falsifiers-001`) — ES-7 and VT-9 are
  `[PROVISIONAL]` and each names a falsifier that no longer discriminates.
  Amended 2026-08-10: both markers now have named ADR owners in the imported
  corpus — ES-7 is ADR-0001's lift condition, discharged by ADR-0008
  (`kb-decision-0001`, `kb-decision-0008`); VT-9 is one of ADR-0014's four
  provisional parts (`kb-decision-0014`), owned by phase 9's Workers
  skeleton. Moving either marker is still an ADR's act, not this atom's.
- **Open** — [`nothing-owns-the-post-phase-reconciliation.md`](../open-questions/nothing-owns-the-post-phase-reconciliation.md)
  (`kb-open-question-post-phase-reconciliation-001`) — no phase carries an
  item obliging anyone to read the specification back against the tree a
  phase just changed. Forced by phase 6's exit and, secondarily, by first
  publish at phase 12. Depends conceptually on the PS-1, PS-19 and
  ES-7/VT-9 questions above.

## Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)

Eleven questions below were filed by the 2026-08-10 ADR-import intake wave, deferred rather than
settled because each is forced by a phase or an adapter that has not arrived yet. Grounding for
each is in the atom itself; see [`domain-map.md`](domain-map.md#contract-ports-conformance-and-the-adr-corpus-2026-08-10-adr-import)
for the reference, concept, governance and playbook atoms this domain also owns.

- **Open** — [`adr-status-vocabulary-exceeds-the-schema.md`](../open-questions/adr-status-vocabulary-exceeds-the-schema.md)
  (`kb-open-question-adr-status-vocabulary-001`) — `KbFrontmatter`'s status
  enum has no value for "accepted, provisional" or "partly superseded," both
  load-bearing in the imported ADR corpus.
- **Open** — [`projection-store-batch-has-no-apply-seam.md`](../open-questions/projection-store-batch-has-no-apply-seam.md)
  (`kb-open-question-projection-batch-no-apply-001`) — `ProjectionStore::Batch`
  carries no trait bounds, so generic code can open and commit a batch and
  cannot write anything into it. Owned by phase 6.
- **Open** — [`query-union-rule-is-owed-and-unowned.md`](../open-questions/query-union-rule-is-owed-and-unowned.md)
  (`kb-open-question-query-union-rule-unowned-001`) — `query_union_is_item_concatenation`
  is named as owed and declined by ADR-0011, which flags it as the one
  disposition a human should confirm rather than inherit.
- **Open** — [`global-versus-per-boundary-visibility-invariant.md`](../open-questions/global-versus-per-boundary-visibility-invariant.md)
  (`kb-open-question-global-vs-boundary-visibility-001`) — ADR-0013 froze the
  visibility invariant globally by decision, not by evidence; a boundary-scoped
  projection checkpoint at phase 6 would reopen it.
- **Open** — [`postgres-arm-c-structural-cost.md`](../open-questions/postgres-arm-c-structural-cost.md)
  (`kb-open-question-postgres-arm-c-cost-001`) — whether a real `sqlx`
  adapter can express ADR-0013's chosen mechanism (xid8 + pg_snapshot_xmin)
  cleanly; the experiment measured four SQL strategies, not four
  implementations. Owned by phase 10 / ADR-0024.
- **Open** — [`poll-count-bounds-the-visibility-rule.md`](../open-questions/poll-count-bounds-the-visibility-rule.md)
  (`kb-open-question-poll-count-rule-strength-001`) — the cold-future
  hand-polling rule that checks ADR-0013's invariant has a window bounded by
  an adapter's poll count, uncalibrated above two polls. Owned by phase 10.
- **Open** — [`es-38-and-gap-read-rules-are-unowned.md`](../open-questions/es-38-and-gap-read-rules-are-unowned.md)
  (`kb-open-question-es-38-and-gap-read-unowned-001`) — ES-38's rule needs a
  removal-capable store the fixture cannot declare, and `read_from_a_gap_position`
  is named by two accepted decisions (ADR-0011, ADR-0013) and owned by
  neither.
- **Open** — [`projection-id-is-unvalidated.md`](../open-questions/projection-id-is-unvalidated.md)
  (`kb-open-question-projection-id-unvalidated-001`) — `ProjectionId::new` is
  infallible and unvalidated; ADR-0015 declined to validate it, on the
  ground that the omission was never a decision. Forced by phase 6.
- **Open** — [`cf-40-fixture-limits-ownership.md`](../open-questions/cf-40-fixture-limits-ownership.md)
  (`kb-open-question-cf-40-ownership-001`) — ADR-0015 both claims and
  disclaims ownership of CF-40 in its own text; ADR-0012 is the other
  claimant. Forced by phase 8's first adapter with real limits.
- **Open** — [`dcb-reference-publishes-no-wire-format.md`](../open-questions/dcb-reference-publishes-no-wire-format.md)
  (`kb-open-question-dcb-no-published-format-001`) — WF-1's interoperability
  half stays `[DEFERRED]` because the DCB reference publishes no wire format
  to interoperate with at all. Owned by phase 13.
- **Open** — [`human-readable-payload-encoding-on-a-constrained-peer.md`](../open-questions/human-readable-payload-encoding-on-a-constrained-peer.md)
  (`kb-open-question-human-readable-encoding-limits-001`) — WF-11's
  falsifier is broader than base64: `serde`'s `Serializer` has no streaming
  entry point for a human-readable string, for any encoding. Owned by phase
  9's Durable Object adapter.
- **Open** — [`sync-message-set-and-format-version.md`](../open-questions/sync-message-set-and-format-version.md)
  (`kb-open-question-sync-message-set-undesigned-001`) — `FORMAT_VERSION = 1`
  is fully tested and names no message set yet; the vocabulary is phase 13's
  design.

## Adding an entry

Append the bullet under the domain section the question belongs to (see
[`domain-map.md`](domain-map.md) for the list of domains); start a new `##`
section only when the question's domain has no section yet. State the
status (`Open`, `Withdrawn`, `Superseded`) first, then the id and one
sentence — the atom itself carries the "what is true today / what is not
decided / what forces it" structure, this index does not repeat it.
