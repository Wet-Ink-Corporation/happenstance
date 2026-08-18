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
  that it was once open is itself worth keeping. The 2026-08-13 wave flipped
  kb-open-question-projection-batch-no-apply-001 to Superseded (by ADR-0017) and annotated the
  PS-1 and PS-19 gaps with the clause-pairing sweep's findings and ADR-0018/ADR-0019's attribution
  of the same defect shape; all three stayed listed rather than being replaced. The 2026-08-15
  wave flipped kb-open-question-ps-1-no-progress-obligation-001 and
  kb-open-question-ps-19-scope-narrower-001 to Superseded (both by ADR-0030, which mints PS-38)
  and added kb-open-question-ps-32-adr-0007-correction-owed-001 (ADR-0007's Context still
  overstates what cannot be written against the port). The 2026-08-17 wave flipped
  kb-open-question-ps-32-adr-0007-correction-owed-001 to Superseded (by ADR-0031, which carries the
  corrected Context riding with its own partial supersession of ADR-0007) and added four new
  questions: kb-open-question-es-17-two-adapter-measurement-001 (the two-build append-ownership
  measurement ADR-0012's falsifier asks for is scheduled by nobody), kb-open-question-d-1-no-total-path-001
  (no infallible route into or out of a validated `QueryItem`/`Tags`), kb-open-question-cf-36-unperformed-cross-reference-001
  (CF-36 names a level-marker cross-reference `spec-trace` does not perform), and
  kb-open-question-no-ps-rule-name-resolved-001 (a bare dagger, not the `has_suite` family switch,
  is what still leaves every `PS` rule name unresolved).
depends_on: []
related:
  - kb-map-domain-001
  - kb-map-decision-001
source_paths:
  - .kb/open-questions/README.md
  - .kb/_governance/integration-waves/2026-08-10-intake/02-placement-and-adjudication.md
  - .kb/_governance/integration-waves/2026-08-10-intake-2
  - .kb/_governance/integration-waves/2026-08-13-projection-adrs
  - .kb/_governance/integration-waves/2026-08-15-adr-0030-checkpoint-progress
  - .kb/_governance/integration-waves/2026-08-17-adr-0022-append-condition
last_reviewed: 2026-08-17
---

# Open-questions index

Every `open_question` atom, current status first. See
[`../open-questions/README.md`](../open-questions/README.md) for what belongs
in that layer and how a question gets resolved; see
[`domain-map.md`](domain-map.md) for the subject-area grouping this index's
questions sit inside.

## Specification governance & conformance

The first seven questions below were filed by the 2026-08-10 phase 4/5
specification-reconciliation intake wave. Full grounding for each is in the
[census reference atom](../reference/phase-4-5-specification-reconciliation-census.md)
(`kb-reference-phase-4-5-spec-reconciliation-001`); this index states only
what the question is, not its evidence. The last two were added by the
2026-08-17 wave from the phase-7 contract-defect log.

- **Open** — [`disjoint-boundaries-have-no-clause.md`](../open-questions/disjoint-boundaries-have-no-clause.md)
  (`kb-open-question-disjoint-boundaries-no-clause-001`) — the independence
  proposition DCB exists for is enforced by a live conformance rule and
  stated by no clause.
- **Open** — [`model-family-rule-has-no-clause.md`](../open-questions/model-family-rule-has-no-clause.md)
  (`kb-open-question-model-family-rule-no-clause-001`) — the model-based
  rule checks the composition of seven clauses and belongs to none of them.
- **Superseded** — [`ps-1-states-no-progress-obligation.md`](../open-questions/ps-1-states-no-progress-obligation.md)
  (`kb-open-question-ps-1-no-progress-obligation-001`) — PS-1's `MUST` is a
  coupling, not a progress obligation; the third rule assigned to it does
  not follow from the sentence. Owned by phase 6. Amended 2026-08-13: the
  clause-pairing sweep confirmed the defect isolated (29 sound / 7 defective
  / 1 undetermined) and ADR-0019 (`kb-decision-0019`) named the same
  intent-not-sentence habit recurring on PS-29; sub-questions 1, 2 and 4
  stayed open. Resolved 2026-08-15 by ADR-0030 (`kb-decision-0030`), which
  mints PS-38 rather than widening PS-1: sub-question 1 answered a clause of
  its own, sub-question 2 by reattributing `commit_advances_the_checkpoint`
  to PS-38, sub-question 4 by the decision itself; PS-1's own text stays
  byte-identical.
- **Superseded** — [`ps-19-scope-narrower-than-its-rule.md`](../open-questions/ps-19-scope-narrower-than-its-rule.md)
  (`kb-open-question-ps-19-scope-narrower-001`) — PS-19's `MUST` is scoped
  to after a reset; its second assigned rule asks about an id never seen.
  Owned by phase 6; interacts with the PS-1 question above. Amended
  2026-08-13: the same sweep confirmed this finding too, and ADR-0018
  (`kb-decision-0018`) scoped the defect out of its own clause range by
  name without repairing it; sub-questions 1 and 3 stayed open. Resolved
  2026-08-15 by ADR-0030 (`kb-decision-0030`): PS-19 keeps its post-reset
  scope and the never-seen-id obligation becomes PS-38's second sentence;
  sub-question 2's 2026-08-13 "isolated" verdict stands untouched.
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
- **Open** — [`cf-36-names-a-cross-reference-nothing-performs.md`](../open-questions/cf-36-names-a-cross-reference-nothing-performs.md)
  (`kb-open-question-cf-36-unperformed-cross-reference-001`) — CF-36 is
  `[FROZEN]` and its `Rule:` line claims `cargo xtask spec-trace`
  cross-references each case's level marker; it does not, and no other check
  performs the comparison under another name. Added 2026-08-17; forced by
  the next reader who cites a green `spec-trace` as evidence for a level
  marker, and by phase 12.
- **Open** — [`no-ps-rule-name-is-resolved.md`](../open-questions/no-ps-rule-name-is-resolved.md)
  (`kb-open-question-no-ps-rule-name-resolved-001`) — CF-38 is `[FROZEN]`;
  PS-27 and PS-30 are the visible symptom of a rule-name check that never
  runs against any `PS` clause carrying a bare dagger, since the dagger sets
  `schedules_new` regardless of what `has_suite` now admits. Added
  2026-08-17; forced by the next `PS` clause that cites a rule name nobody
  has written.

## Contract ports, conformance, and the ADR corpus (2026-08-10 ADR import)

The first eleven questions below were filed by the 2026-08-10 ADR-import intake wave, deferred
rather than settled because each is forced by a phase or an adapter that has not arrived yet. The
twelfth, ES-17, was added by the 2026-08-17 wave. Grounding for each is in the atom itself; see
[`domain-map.md`](domain-map.md#contract-ports-conformance-and-the-adr-corpus-2026-08-10-adr-import)
for the reference, concept, governance and playbook atoms this domain also owns.

- **Open** — [`adr-status-vocabulary-exceeds-the-schema.md`](../open-questions/adr-status-vocabulary-exceeds-the-schema.md)
  (`kb-open-question-adr-status-vocabulary-001`) — `KbFrontmatter`'s status
  enum has no value for "accepted, provisional" or "partly superseded," both
  load-bearing in the imported ADR corpus.
- **Superseded** — [`projection-store-batch-has-no-apply-seam.md`](../open-questions/projection-store-batch-has-no-apply-seam.md)
  (`kb-open-question-projection-batch-no-apply-001`) — `ProjectionStore::Batch`
  carries no trait bounds, so generic code can open and commit a batch and
  cannot write anything into it. Answered 2026-08-13 by ADR-0017
  (`kb-decision-0017`): `Batch` becomes an owned type with no lifetime
  parameter and no universal write vocabulary; sub-question 3 — whether this
  retroactively validates ADR-0006's discriminator — stays open, with the
  typed layer.
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
- **Superseded** — [`ps-32-adr-0007-context-correction-is-owed.md`](../open-questions/ps-32-adr-0007-context-correction-is-owed.md)
  (`kb-open-question-ps-32-adr-0007-correction-owed-001`) — PS-32 is `[FROZEN]`
  and states ADR-0007's Context must be corrected: a callback-driven pump
  *can* be written against the port as it stands, falsified by compilation
  rather than argument. What is not decided is who performs the correction
  and in which atom — ADR-0007 is accepted and immutable, so it is a
  superseding decision's act. Added 2026-08-15. **Resolved 2026-08-17** by
  ADR-0031 (`kb-decision-0031`): the correction rides with a partial
  supersession of ADR-0007 that collapses the checkpoint pump upward into
  `happenstance::run_projection`; `kb-decision-0007` stays accepted because
  its three shape decisions are implemented as written, and the pump
  sub-question is answered in the negative — no pump is written.
- **Open** — [`es-17-two-adapter-measurement-is-unscheduled.md`](../open-questions/es-17-two-adapter-measurement-is-unscheduled.md)
  (`kb-open-question-es-17-two-adapter-measurement-001`) — ADR-0012's
  falsifier item 1 asks for two builds of one SQLite adapter differing only
  in `append`'s batch ownership; the phase-8 append-condition experiment
  (`kb-reference-append-condition-experiment-001`) measured three strategies
  against the same `&[Event]` signature instead, and no story currently
  scheduled produces the two-build evidence. Added 2026-08-17; forced by
  whoever next proposes lifting ES-17 to `[FROZEN]`, or by phase 12.

## The typed layer: decision models, codecs, and payload evolution

One question, added by the 2026-08-17 wave from the phase-7 contract-defect log. See
[`domain-map.md`](domain-map.md#the-typed-layer-decision-models-codecs-and-payload-evolution) for
the decision and reference atoms this domain also owns.

- **Open** — [`d-1-the-validated-type-has-no-total-path.md`](../open-questions/d-1-the-validated-type-has-no-total-path.md)
  (`kb-open-question-d-1-no-total-path-001`) — `QueryItem::new` is fallible
  even over already-validated `EventType`/`Tag` inputs, and `Boundary`'s
  seal makes that error arm untestable from outside the crate; `DomainEvent::tags`
  is total over a `Tags` every constructor route into which is fallible, so an
  implementor with runtime tag values has no total path without a hand-rolled
  newtype. ADR-0020 named the pair defect candidate D-1 and routed it to a
  decision record that has not been written. Added 2026-08-17; forced by the
  first API change after 0.1, and named by ADR-0033 as the single condition
  that would reopen the `happenstance-macros` scope verdict.

## Adding an entry

Append the bullet under the domain section the question belongs to (see
[`domain-map.md`](domain-map.md) for the list of domains); start a new `##`
section only when the question's domain has no section yet. State the
status (`Open`, `Withdrawn`, `Superseded`) first, then the id and one
sentence — the atom itself carries the "what is true today / what is not
decided / what forces it" structure, this index does not repeat it.
