---
id: kb-open-question-cf-25-cf-26-portfolio-check-001
title: CF-25 and CF-26 name a portfolio check cargo xtask spec-trace does not perform
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-25 is FROZEN and gates every port freeze in the specification on cargo xtask spec-trace
  reading the section 6.5 instrument-portfolio table and failing when a FROZEN port clause has
  an axis with no far-end row and no named risk acceptance. CF-26 names the same mechanism
  against the table's Far end exists? column. Neither exists: grep -ci for "portfolio", "far end"
  and "axis" against xtask/src/spec_trace.rs each return zero, both before and after this
  remediation pass's own two new checks landed. kb-decision-0039 (ES-42's freeze) already argues
  the qualification does not bite a compile-level rule, which is one clause's escape and not a
  fix for the mechanism CF-25 claims exists. The deadline is not 0.2.0 — it is the ProjectionStore
  freeze, which section 6.5's own table shows sitting on axes (batch shape, completeness) with no
  far end built, and which would otherwise pass a green spec-trace that claims a check the file
  does not contain.
depends_on:
  - kb-decision-0039
related:
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-playbook-verify-referent-report-coverage-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/portfolio-check-and-the-port-freeze-bar.md
  - .kb/_intake/es-42-marker-earned-off-at-0-2-0.md
last_reviewed: 2026-09-07
---

# CF-25 and CF-26 name a portfolio check cargo xtask spec-trace does not perform

## What is true today

Two `[FROZEN]` clauses assert an instrument that the tree does not contain.
CF-25 gates every port freeze in `spec/SPECIFICATION.md` and names its own
checker: "`cargo xtask spec-trace` (CF-38), which reads the portfolio table
and the maturity markers and fails when a `[FROZEN]` port clause has an axis
with no far-end row and no named risk acceptance." CF-26 names the same
mechanism against §6.5's `Far end exists?` column specifically. Measured
against `xtask/src/spec_trace.rs`, both before this remediation pass and
after its own two new checks landed: `grep -ci "portfolio"` returns `0`,
`grep -ci "far end"` returns `0`, `grep -ci "axis"` returns `0`. Nothing reads
§6.5 at all — not a naming mismatch, an absent mechanism, and the same defect
shape `kb-open-question-cf-36-unperformed-cross-reference-001` describes for
a different clause: a `[FROZEN]` `Rule:` line naming a cross-reference
`spec-trace` was never given the code to perform.

§6.5's table carries seven axes; read out of the document, three carry a
far-end tick on both fixture and adapter sides, two carry a fixture
instrument with no adapter one, and two are empty at both ends — including
`Batch shape (ProjectionStore)`, whose far end is built and whose near end is
not, and `Completeness`, which nothing plans to build. The prose beneath the
table states this arithmetic in words and nothing checks the words against
the column, which is the same defect §1.3's stated-count census had before
`check_stated_census` closed it.

`kb-decision-0039` (ES-42's freeze at `0.2.0`) is adjacent but does not
discharge either clause: it argues, correctly and narrowly, that CF-25's
qualification does not bite a *compile-level* rule with no adapter axis to
falsify — which is why that one freeze was safe to take without the
portfolio check existing. It is one clause's escape from the requirement,
stated and reasoned; it is not evidence the requirement is met for a
*behavioural* port clause, and `ProjectionStore`'s gating clauses are exactly
that kind.

## What is not decided

Whether the portfolio check gets built before `ProjectionStore` freezes, or
whether that freeze instead takes CF-25's second permitted route — naming the
axis it accepts risk on and the ADR that accepts it — leaving the mechanism
itself still unbuilt for whichever port freezes after it. A full build of
CF-25 needs two readings the specification does not carry in machine-readable
form (which clauses are "port clauses," and which axes each port sits on);
a narrower build of CF-26 alone would hold the table's `Far end exists?`
column to §6.5's own prose census without gating any freeze, and could be
written today without either reading — at the cost of looking, on a green
run, like more of CF-25 is discharged than actually is.

## What forces it

The `ProjectionStore` freeze itself, which is earlier than first publish and
earlier than most other deadlines in this remediation's briefs. Whoever runs
that freeze inherits a `[FROZEN]` clause telling them the portfolio bar was
machine-checked; today it would not have been.

## Ordered sub-questions

1. Does §6.5 acquire a machine-readable port column before any check is
   written against it, or does the check duplicate a reading of prose that
   already exists?
2. Is CF-26's half worth landing alone, given the risk that a green run then
   reads as more of CF-25 discharged than it is — and if so, does its summary
   line have to name what it does not check, the way this pass's other new
   counts already do?
3. Does the `Batch shape (ProjectionStore)` row's one-sided tick (far end
   green, near end unbuilt) count as an axis with a far end for CF-25's
   purposes? That reading decides whether the `ProjectionStore` freeze passes
   outright or has to name an acceptance.
