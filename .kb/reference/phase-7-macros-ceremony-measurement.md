---
id: kb-reference-macros-ceremony-measurement-001
title: The ceremony-to-domain ratio of the worked example, measured line range by line range
kind: reference
status: accepted
authority_tier: note
summary: >-
  The classification AC-013's verdict rests on, taken 2026-08-16 against
  examples/course-subscriptions/src/main.rs at commit 78a2170, 532 lines, as
  worked-example-on-typed-layer left it. Checked plain before counting — no
  macro_rules!, and impl DomainEvent for Enrolment written out by hand — rather
  than against the design's doctest, which _design.md forbids substituting. The
  judgement is not in the threshold, which is mechanical, but entirely in the
  classification, which is why the classification is published as 29 contiguous
  line ranges that exhaust the file, 532 of 532, with the totals derived by
  summing rows. Ceremony, the impl DomainEvent block at :209-247, is 40 lines;
  domain is 249; neither — main, the transcript, imports and call plumbing — is
  158; contested, the CourseId and StudentId newtypes at :98-182, is 85.
  Assigning the contested block to ceremony gives 125:249, a ratio of 0.50:1;
  assigning it to domain gives 40:334, or 0.12:1. Both extremes are below 1.0,
  so the contested block is a footnote rather than the decision. One further
  figure, which is what a derive would have bought: 40 lines of 532, or 7.5 per
  cent. The reconciliation with _design.md's 2.4:1 prediction over its own
  doctest is that the DomainEvent impl is a near-fixed cost, 26 lines for two
  variants and 40 for three, while domain logic grows with consistency
  concerns, refusals and handlers — so the ceremony ratio is a function of how
  much domain an artefact contains, and a minimal doctest measures it where
  there is almost none. Both numbers are true and they answer different
  questions.
depends_on: []
related:
  - kb-decision-0020
  - kb-decision-0033
source_paths:
  - .kb/_intake/happenstance-macros-verdict.md
  - references/evaluation/phase-7-macros-verdict.md
  - examples/course-subscriptions/src/main.rs
  - RUNBOOK.md
last_reviewed: 2026-08-17
---

# The ceremony-to-domain ratio of the worked example, measured line range by line range

## What this is a pointer to

`RUNBOOK.md:4079-4083` states AC-013's criterion for `happenstance-macros` in
mechanical form: if the rewritten worked example carries more mapping
ceremony than domain logic, the derive is in scope for 0.1. "More" means the
ratio exceeds one; exactly 1.0 is out. Because the threshold is mechanical,
the entire judgement lives in the classification of each line — this atom is
that classification, published so it can be checked rather than trusted.

## Method

Substrate: `examples/course-subscriptions/src/main.rs` at commit `78a2170`,
532 lines, as the `worked-example-on-typed-layer` project left it. Checked
plain before counting — no `macro_rules!` anywhere in the file, and
`impl DomainEvent for Enrolment` written out by hand at `:209-247`. This is
deliberately **not** the design's doctest: `_design.md:1110-1111` forbids
substituting the doctest for the worked example when a ceremony ratio is
being measured, because the doctest is a minimal illustration and the worked
example is the artefact the criterion actually names.

Every line of the file was assigned to exactly one of four buckets, and the
29 resulting ranges are contiguous and exhaust the file — 532 of 532, with no
line counted twice and none left over:

| Bucket | Lines | Range |
| --- | ---: | --- |
| ceremony (`impl DomainEvent`) | 40 | `:209-247` |
| domain | 249 | (14 ranges) |
| neither (`main`, transcript, imports, call plumbing) | 158 | (13 ranges) |
| contested (`CourseId` / `StudentId` newtypes) | 85 | `:98-182` |

The contested block is the identity newtypes, kept separate because they are
validation ceremony in one reading and domain modelling in another, and the
measurement does not adjudicate that question — it reports both extremes.

## Findings

**Both extremes land below the threshold.** Assigning the contested 85 lines
entirely to ceremony gives 125:249, a ratio of 0.50:1. Assigning them
entirely to domain gives 40:334, or 0.12:1. Both are "out" under AC-013's
mechanical test, so the contested block is a footnote rather than the
decision — the verdict does not turn on how those 85 lines are classified.

**What a derive would have bought.** The only bucket a derive plausibly
removes is ceremony: 40 lines of 532, 7.5% of the file.

**The reconciliation with the design's own prediction.** `_design.md:1104-1111`
predicted "in", at 2.4:1, over its own doctest — the opposite verdict, by a
factor of roughly five. The reason is structural rather than a measurement
error: the `DomainEvent` impl is a near-fixed cost, 26 lines for two event
variants and 40 for three, while domain logic — consistency concerns,
refusals, handlers — grows with the artefact. A minimal doctest measures the
ceremony ratio at the one point where domain content is smallest, which is
exactly where the ratio is largest. Both numbers are true measurements of
different objects: the doctest's 2.4:1 still governs AC-U01 (what a
first-program page should read like), and this atom's 0.50:1/0.12:1 governs
AC-013 (whether a scope decision follows from it).

## Provenance

Reproduced against `examples/course-subscriptions/src/main.rs` pinned to
`78a2170`, dated 2026-08-16. The full 29-range table with line-by-line
justification lives in `references/evaluation/phase-7-macros-verdict.md`,
which this atom points to rather than duplicates. `kb-decision-0020` is the
record that logged the underlying gap (D-1: no infallible constructor for
pre-validated `Query`/`Tags` inputs) whose ceremony this measurement counts,
and which named 2.4:1 as a falsifiable prediction in the first place.
