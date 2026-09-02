---
id: kb-decision-0033
title: happenstance-macros is out of scope for 0.1
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0033
reversibility: high
phase: 7
supersedes: null
superseded_by: null
summary: >-
  AC-013's criterion is mechanical — if the rewritten worked example carries
  more mapping boilerplate than domain logic the derive is in scope, more
  meaning a ratio above one, and exactly 1.0 is out — so the whole judgement
  is in the classification, which is published line range by line range and
  summed rather than asserted. Measured against
  examples/course-subscriptions/src/main.rs at 78a2170, the ratio is 0.50:1
  with every contested line charged to ceremony and 0.12:1 with them charged
  to domain. Both extremes say out, so the contested block is a footnote and
  the verdict stands: no crates/happenstance-macros/ ships in 0.1. ADR-0020
  predicted the opposite, in at 2.4:1, over its own doctest, and the
  contradiction is the prediction working rather than a conflict to
  reconcile away: ADR-0020 published the number as falsifiable and asserts
  no must about the derive, so nothing accepted is superseded here. The two
  numbers differ for a durable reason worth keeping: the DomainEvent impl is
  a near-fixed cost, 26 lines for two variants and 40 for three, while
  domain logic grows with consistency concerns, refusals and handlers — so
  the ceremony ratio is a function of how much domain an artefact contains,
  and a minimal doctest measures it at the point of almost no domain. A
  first-program page is still held to 2.4:1, which is AC-U01; a scope
  decision is not taken on it, which is AC-013. What a derive would have
  bought is 40 lines of 532, 7.5 per cent, against a fourth published crate
  and a proc-macro in every consumer's build graph. Reopen if and only if
  D-1 is settled with an infallible Tags path: a derive that also handled
  tags from runtime values would reach into the contested 85 lines rather
  than only the 40, but even an 85-line swing does not cross 1.0 from
  0.50:1, so it stays a post-0.1 question and the measurement would be
  re-taken rather than re-argued. Consequences already carried out: no
  crate was created, since an out verdict escalates nothing; RUNBOOK.md's
  decision-table row moves off open; and publish-0-2-0-alpha-1 is
  unblocked, because it depends on the record and never on a crate.
depends_on:
  - kb-reference-macros-ceremony-measurement-001
related:
  - kb-decision-0020
  - kb-open-question-d-1-no-total-path-001
source_paths:
  - .kb/_intake/happenstance-macros-verdict.md
  - .kb/_intake/contract-defect-log-phase-7.md
  - references/evaluation/phase-7-macros-verdict.md
  - examples/course-subscriptions/src/main.rs
  - RUNBOOK.md
last_reviewed: 2026-08-17
---

# happenstance-macros is out of scope for 0.1

## Context

ADR-0020 recorded DT-2's resolution — a `DecisionModel`'s `query()` is
derived on a sealed `Boundary` trait rather than hand-written — and priced
it at 11 lines of domain logic to 26 of mapping ceremony in the signed-off
first program, a 2.4:1 ratio published as a **falsifiable prediction** that
AC-013's verdict would land "`happenstance-macros` is in scope for 0.1."
ADR-0020 itself asserts no *must* about the derive: the two-constructor
citation that grounds it is a consequence stated in that record, not a
second decision, and `RUNBOOK.md:525`'s ADR column naming 0020 is not
authority to edit it — 0020 is accepted and therefore immutable. AC-013's
own criterion is mechanical, stated at `RUNBOOK.md:4079-4083`: if the
*rewritten worked example* carries more mapping ceremony than domain logic,
the derive is in scope. More means the ratio exceeds one; exactly 1.0 is
out. Because the threshold is mechanical, the whole judgement lives in the
classification of each line, not in the number itself.

## Decision

`happenstance-macros` does not ship in 0.1. The classification behind this
verdict is published in full, line range by line range, as
`kb-reference-macros-ceremony-measurement-001` — 29 contiguous ranges
exhausting `examples/course-subscriptions/src/main.rs` at `78a2170`, 532 of
532 lines, checked plain (no `macro_rules!`, `impl DomainEvent for
Enrolment` written by hand) rather than against the design's doctest, which
`_design.md:1110-1111` forbids substituting for this measurement.

The contested 85 lines — the `CourseId`/`StudentId` identity newtypes — are
not adjudicated, because the verdict does not depend on how they are
classified. Charged entirely to ceremony: 125:249, 0.50:1. Charged entirely
to domain: 40:334, 0.12:1. Both are below the 1.0 threshold, so the
contested block is a footnote and the verdict is "out" either way.

This is not a supersession of ADR-0020. ADR-0020's prediction — 2.4:1, "in"
— was published as falsifiable specifically so a later measurement could
falsify it, and this measurement does, by roughly a factor of five in the
other direction. The two numbers disagree for a reason worth keeping on the
record rather than reconciling away: the `DomainEvent` impl is a near-fixed
cost (26 lines for two event variants, 40 for three) while domain logic —
consistency concerns, refusals, handlers — grows with the artefact. A
minimal doctest therefore measures the ceremony ratio at the one point where
domain content is smallest, which is exactly where the ratio is largest.
Both figures are true measurements of different objects: the doctest's
2.4:1 continues to govern AC-U01 (what a first-program page should read
like); this decision's 0.50:1/0.12:1 governs AC-013 (whether a scope
decision follows from the *worked example*, not the doctest).

## Consequences

No `crates/happenstance-macros/` was created — an "out" verdict escalates
nothing under `_decomposition.md:428`. `RUNBOOK.md`'s decision-table row for
AC-013 moves off `open` to this record. `publish-0-2-0-alpha-1` is
unblocked, since it depends on the record existing, never on a crate
shipping.

What a derive would have bought, for the record: 40 lines of 532, 7.5% of
the file, against the cost of a fourth published crate and a proc-macro
dependency in every consumer's build graph.

## Reopen condition

Reopen **if and only if** D-1 (`kb-open-question-d-1-no-total-path-001`) is
settled with an infallible `Tags` path. A derive that also handled tags
constructed from runtime values would reach into the contested 85 lines
rather than only the 40-line `DomainEvent` impl — but even the full 85-line
swing does not cross the 1.0 threshold from 0.50:1, so this stays a
post-0.1 question. If D-1 is settled that way, the measurement in
`kb-reference-macros-ceremony-measurement-001` should be re-taken against
whatever the resulting worked example looks like, not re-argued from this
record.

## Alternatives rejected

Reconciling ADR-0020's 2.4:1 with this measurement by treating the doctest
as authoritative for a scope decision: rejected, because that would use a
minimal illustration — deliberately thin on domain content — to answer a
question about a worked example with substantially more domain content,
which is precisely the confound this atom's classification exists to avoid.
Treating the contested 85 lines as decisive: rejected, because both of its
extreme assignments agree on the verdict, so adjudicating it changes no
outcome and would only manufacture false precision.
