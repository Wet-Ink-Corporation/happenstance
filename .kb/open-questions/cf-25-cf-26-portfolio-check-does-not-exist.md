---
id: kb-open-question-cf-25-cf-26-portfolio-check-001
title: CF-25 and CF-26 name a portfolio check cargo xtask spec-trace does not perform
kind: open_question
status: superseded
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
  Resolved 2026-11-08 (#125), on the owner's route: the check is built, not the Rule line repaired.
  cargo xtask spec-trace walks every port that carries a FROZEN clause, or that section 1.6
  declares frozen, against the portfolio and a new machine-readable freeze ledger in section 6.5.
  It fails a row discharged neither by an adapter at the far end, nor by an accepted record naming
  the axis and the port, nor by an open clause of the port. Sub-question 1: a ledger table, not a
  column. Sub-question 2: CF-26 lands with CF-25, not alone, and the summary line names what the
  walk does not read. Sub-question 3 stays moot. ADR-0103 accepts ProjectionStore's async-flavour row. The check
  passes since the merge of 2026-11-09: #422's projection reopen rule and the network peers #130
  and #131 put an adapter at the far end of the four rows that had none.
depends_on:
  - kb-decision-0039
related:
  - kb-open-question-cf-36-unperformed-cross-reference-001
  - kb-playbook-verify-referent-report-coverage-001
source_paths:
  - xtask/src/spec_trace/portfolio.rs
  - .kb/_intake/remediation-2026-09-04-briefs/portfolio-check-and-the-port-freeze-bar.md
  - .kb/_intake/es-42-marker-earned-off-at-0-2-0.md
last_reviewed: 2026-11-08
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

## Phase 16 — 2026-09-29

**Classification: additive** — repository tooling and specification text; no crate signature
moves. CF-25 and CF-26 are `[FROZEN]`, so neither has a row in `runbook/ledgers.md`, *The 1.0 dispositions*
(`kb-decision-0066`). The forcing event has passed: `kb-decision-0063` froze `ProjectionStore`
after `kb-decision-0062` built the far end (`LivePostgresProjectionStore`), which meets CF-25's
bar in substance, but neither record cites CF-25 or CF-26, and `xtask/src/spec_trace.rs` still
has no portfolio logic (`grep -ciE "portfolio|far end"` returns 0 at `3dcba41`). Sub-question 3
is moot for that freeze; 1 and 2 stand.

**Owner now: phase 13.** The next freezes CF-25 gates are the SY clauses on the peer port, and
each either lands with the check or takes CF-25's second route, naming the axis it accepts risk
on and the record that accepts it. A Rule-line repair saying the check is a reviewer's walk
rather than `spec-trace` is a `[FROZEN]`-clause repair under `kb-playbook-repair-frozen-clause-001`,
and phase 21's clause audit holds whichever route was taken.

## Closed — 2026-11-08

**Route: build the check** (the owner's call on #125, 2026-11-08). CF-25's `Rule:` line is not
repaired. `xtask/src/spec_trace/portfolio.rs` performs what it names, and `spec-trace` prints the
walk's summary on every run. The line is not yet *satisfied*: see the last section.

**Sub-question 1 — a machine-readable table, and a ledger rather than a column.** §6.5 gains
*The freeze ledger*: one row per axis a port sits on, naming how that port's freeze discharges
it. A port column on the portfolio table was the other shape and lost, because CF-25's own
*pro forma* sentence makes the discharge per port: an axis two ports sit on can be covered for
one and accepted for the other, and one cell cannot say both. The check reads the portfolio for
two things only, each axis's name and the bold verdict opening its `Far end exists?` cell, so
it does not re-read prose a person already wrote. Which ports are frozen comes from the clauses
themselves, as the `Rule:` line says: a port with a `[FROZEN]` clause under its §1.4 prefix is
walked. §1.6's `Maturity` column is held to the same bar against a closed vocabulary, so a port it
declares **Frozen** is walked too, and a lead the check cannot classify fails.

**Sub-question 2 — CF-26 did not land alone.** It landed with CF-25, as the rule that only a
verdict saying *adapter yes* discharges a `far end` row: a fixture-only verdict and a bare *yes*
do not, and a verdict outside the vocabulary is a failure rather than a pass. The summary line
names what the walk does not read on every run: the near end, whether a port really does not sit
on an axis it has no row for, and what an accepting record says beyond naming the axis and the
port.

**Sub-question 3 stays moot.** `ProjectionStore`'s batch-shape row is discharged by `far end`
on the verdict as written. Because the check does not read the near end, the stale *near end
no* in that cell does not decide anything. ADR-0062 built that end.

**What the check found on 2026-11-08: four gaps, recorded rather than hidden.** `EventStore`'s six rows
discharge: an adapter at the far end of transport, handle multiplicity and durability; ADR-0013 for
position allocation and async flavour; ES-40's `[PROVISIONAL]` marker for completeness.
`ProjectionStore`'s transport and batch-shape rows have an adapter at the far end, and its
async-flavour row is accepted by ADR-0103, the owner's decision of 2026-11-08. `SyncPeer`'s
transport row is carried by SY-14 and its completeness row by SY-32. Four rows have no adapter, no
accepting record and no open clause, and are written `undischarged`, so `spec-trace` fails on
exactly them. `ProjectionStore` on durability waits on #422, a projection-suite reopen rule.
`SyncPeer` on async flavour, durability and position allocation waits on the network peers, #130
and #131, which this branch merges after.

**What it finds since 2026-11-09: none.** The branch merged `phase-13` and each of the four rows
now names its far end. `ProjectionStore` on durability: `committed_rows_and_checkpoint_survive_a_reopen`
(#422), passed by the SQLite, Postgres and Neon projection stores. `SyncPeer` on position
allocation: the Postgres peer ingests at `nextval()` positions and passes the sync suite in
`live-postgres`. On async flavour: the Durable Object, a `!Send` `RemoteSyncPeer`, passes it under
`workerd`. On durability: `ack_never_confirms_a_non_durable_event` passes against the Neon, Postgres
and Durable Object peers. That rule reconnects rather than reopens, because `SyncFixture` has no
reopen capability yet, and the ledger row says so. `SyncPeer`'s transport row moved from SY-14 to
`NeonSyncPeer` at the far end. The position-allocation and async-flavour verdicts now open
*adapter yes*, which their cells had said in their bodies since phases 10b and 9. One limit is
now visible: an *adapter yes* verdict does not say which port the adapter implements, so
`ProjectionStore`'s async-flavour row would pass as `far end` though nothing `!Send` implements
that port. It stays on ADR-0103, and its Why cell says why.
