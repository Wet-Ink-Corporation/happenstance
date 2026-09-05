---
id: kb-reference-phase-8-spec-reconciliation-001
title: The phase-8 specification reconciliation — what one hand-run of the standing criterion cost
kind: reference
status: accepted
authority_tier: note
summary: >-
  The second census of the standing post-phase reconciliation criterion (RUNBOOK.md:3810-3820),
  hand-run for phase 8 against baseline 53a4764 and recorded in the story's own _reconciliation.md.
  Cost: 8 clauses in the phase's computed range plus 15 further passages whose prose cites the
  phase's crate and had to be read anyway; 3 verdicts unchanged, 5 repaired in range and 15 of 15
  repaired outside it; 0 normative gaps and 2 escalations, neither a clause; one pass, one context,
  no tooling written. Found: 6 stale file:line citations, every one still resolving and every one
  pointing at the wrong subject; 4 factually false counts; 1 PROVISIONAL falsifier naming an event
  that had already occurred. The strongest finding is mechanical — of 401 spec-trace citations
  checked this pass only 80 are anchored to their subject, against 69 of 358 at phase 4/5, because
  subject_before declines whenever the nearest code span is a type name, a quoted phrase or another
  citation, so all six wrong-subject citations went unreported while the tool behaved exactly as
  documented. checked is a coverage number; anchored is the only quality number. A second finding:
  the criterion's arithmetic bullet had nothing to close against, because kb-decision-0022 states
  no clause range in any form, so the second set had to be derived from the phase's own three
  disagreeing statements. A third: of 20 repairs only 6 were citation line numbers a machine could
  plausibly catch and 14 were sentences that were simply false. Three mechanisations are floated
  and none is implemented.
depends_on:
  - kb-reference-phase-4-5-spec-reconciliation-001
related:
  - kb-open-question-post-phase-reconciliation-001
  - kb-playbook-anchoring-citations-001
  - kb-playbook-ratchet-gate-landing-001
  - kb-reference-spec-trace-has-suite-001
  - kb-decision-0022
  - kb-open-question-adr-status-vocabulary-001
source_paths:
  - .kb/_intake/0034-what-the-phase-8-reconciliation-cost.md
  - RUNBOOK.md
  - spec/SPECIFICATION.md
  - xtask/src/spec_trace.rs
  - crates/happenstance-sqlite/src/event_store.rs
last_reviewed: 2026-08-20
---

# The phase-8 specification reconciliation — what one hand-run of the standing criterion cost

## What this is a pointer to

The evidence lives in `.bklg/from-contract-to-published-library/sqlite-durable-store/spec-and-code-reconciliation/_reconciliation.md`,
the phase-8 pass, run against baseline `53a4764`. This atom is the census and
the citable summary — read the source document for finding-by-finding detail.
It is the second data point against `kb-reference-phase-4-5-spec-reconciliation-001`,
which supplies the comparison numbers below; the entire value of this atom is
in reading against that one, not standing alone.

## What it cost, in the units the open question asks about

| Quantity | Measured |
| --- | --- |
| Clauses in the phase's computed range | 8 (CF-14, CF-17, CF-34, ES-35, VT-21 – VT-24) |
| Clauses whose prose cites the phase's crate and had to be read anyway | 15 further passages |
| Verdicts unchanged | 3 of 8 in range — all three already written by an upstream story in the same phase |
| Verdicts repaired | 5 of 8 in range, and 15 of 15 outside it |
| Verdicts gap | 0 normative; 2 escalations, neither a clause |
| Stale `file:line` citations found | 6 — every one still resolving, every one pointing at the wrong subject |
| Factually false counts found | 4 |
| Markers whose falsifier named an event already past | 1 — ES-41's, naming `happenstance-sqlite` at phase 8 as its instrument |
| Effort shape | one pass, one context, no tooling written |

## Finding 1 — `spec-trace` green is not evidence a citation is right

Six citations into `crates/happenstance-sqlite` resolved — file existed, line
in range — while pointing at an entirely different subject than the sentence
claimed: an error enum cited at a span that now holds a struct, a `+ Send`
impl cited at a span that now holds `const MIGRATION_1`, a `head` impl cited
at a span that now holds `SCHEMA_VERSION`, and four clauses citing one span as
"the tag side table" when that span is the `event` table and the side table
sits six lines later. None was reported, because `subject_before`
(`xtask/src/spec_trace.rs:2997-3043`) declines to derive a subject unless the
nearest code span is a lowercase identifier of four-plus characters on the
citation's own line or the one above — every one of the six sits beside a type
name, a quoted phrase, or another citation. Of 401 citations checked this
pass, 80 are anchored (versus 69 of 358 at phase 4/5): `checked` is coverage,
`anchored` is the only quality number, and a green `spec-trace` is a claim
about roughly a fifth of the corpus, not all of it.

## Finding 2 — the arithmetic bullet has nothing to close against

The criterion's second bullet asks for the phase's clause range against the
union of its ADRs' ranges. Phase 8 has one ADR, `kb-decision-0022`, and it
states no clause range in any form — no frontmatter field, no clause ID in
its Decision section, and its `RUNBOOK.md` queue row is the only one in that
queue with no parenthesised range. The comparison set had to be derived from
the phase's own three disagreeing statements: arithmetic comparing a number
against itself. The phase-4 precedent is the identical defect from the other
side — 35 clauses named against 64 discharged, 29 invisible.

## Finding 3 — the pass's real value was in prose nothing checks

Of 20 repairs, 6 were citation line numbers a machine could plausibly have
caught and 14 were sentences that were simply false — counts, tenses, and one
`[PROVISIONAL]` falsifier naming an event that had already occurred. No
extension of `spec-trace` catches "seven" when the answer is twelve. This
argues against a gate step replacing the pass, and for the pass needing an
owner rather than standing unclaimed — the failure mode is silent, durable,
and reads as authoritative.

## What was deliberately not done

Three mechanisations were floated and none implemented: an explicit anchor
comment beside a citation whose subject the heuristic cannot derive; a
per-run report of the unanchored fraction; and a `clauses:` list in a
decision atom's frontmatter. No citation baseline was committed, no gate step
or `xtask` check was added, no `ANCHOR_SLACK` was widened, no clause was
amended, no marker was moved, and no ADR was written. This atom does not
answer any of the five ordered sub-questions on
`kb-open-question-post-phase-reconciliation-001`, and states no `clauses:`
key on `kb-decision-0022` — that gap is a missing frontmatter key on an
immutable atom, a schema question rather than a defect an edit could fix.
