---
id: kb-open-question-vt-30-marker-stale-unscheduled-001
title: VT-30's provisional marker names two falsifiers, one built, and no phase schedules lifting it
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-30 is PROVISIONAL, falsified if no adapter can push a multi-guard AppendCondition into a
  single statement without a self-join per guard, or if the min() collapse measures immaterial
  on a Wattline-shaped workload, naming the Postgres adapter and the benchmark harness as the two
  unbuilt instruments. kb-decision-0054 pins the current blanket-scope behaviour and renames
  after_opt with a deprecated alias, discharging that decision's own scope, but leaves the marker
  itself untouched. Re-checked against the tree: the Postgres adapter is still todo!(), so that
  limb stands as written, but the benchmark harness (crates/happenstance-testkit/src/bench.rs)
  now exists and is not unbuilt — the real gap is narrower than the marker states, a multi-guard
  workload the harness does not yet contain. No phase after 4 names VT-30 in RUNBOOK.md, "Wattline"
  appears nowhere in it, and phase 12's exit criterion audits provisional clauses against that
  ledger rather than against the clause text, so VT-30 clears the audit unexamined on a row naming
  a phase that already shipped. Moving a provisional marker is an ADR's act; this atom records
  that no ADR has been asked to.
depends_on:
  - kb-decision-0054
related:
  - kb-decision-0012
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/after-opt-scope.md
last_reviewed: 2026-09-07
---

# VT-30's provisional marker names two falsifiers, one built, and no phase schedules lifting it

## What is true today

VT-30 (`spec/SPECIFICATION.md:1861-1867`, `[PROVISIONAL]`) requires that
`AppendCondition::new(query)` continue to produce a single unbounded guard and
that `after`/`after_opt` continue to apply the given boundary to every guard
already present. Its marker names a falsifier in two limbs: no adapter can
push a multi-guard condition into a single statement without one self-join
per guard, or the `min()` collapse this shape implies measures as immaterial
on a Wattline-shaped workload — and it names the Postgres adapter and the
benchmark harness as the two instruments, calling both "unbuilt."

`kb-decision-0054` addresses a different, narrower question raised by the
same brief — whether `after_opt`'s blanket-rewrite behaviour is pinned by a
test and named honestly in its own doc block — and answers it by pinning the
behaviour and introducing a scope-carrying name with a deprecated alias for
the old one. That decision is complete on its own terms and leaves VT-30's
marker itself untouched, exactly as its own body records.

Re-checked against the tree, the marker is partly stale. The Postgres limb
still stands as written: `crates/happenstance-postgres/src/event_store.rs`
carries `todo!("postgres event store: append")` and six more `todo!()` calls
in the same file, so no adapter exists yet that could attempt the pushdown
the falsifier asks about. The benchmark-harness limb does not: `bench.rs`
exists at HEAD, exports `event_store_benchmarks!` behind an off-by-default
feature, with a real emitter in the testkit's own tests. So "both are
unbuilt" is no longer true as written. What the built harness cannot yet do
is the part that actually matters — none of its three scenarios constructs a
multi-guard condition, so the gap is not "no harness" but "no multi-guard
workload in the harness," a materially smaller thing to build than the
marker implies. A third fact bears on the falsifier's first limb without
discharging it either way: the one built adapter, `happenstance-sqlite`,
deliberately issues one `SELECT max(position)` per guard rather than pushing
multiple guards into a single statement, a measured preference recorded
against the rejection path rather than a failure to achieve pushdown.

## What is not decided

Whether VT-30 lifts, is restated, or stays as written once these two limbs
are separately correct. Restating it well is not free: the honest gap is
narrower than the marker's text, and a restatement would need to name "a
multi-guard workload in the benchmark harness" rather than "the benchmark
harness," which is a different, smaller falsifier than the one currently
published. Moving a provisional marker changes what a downstream consumer is
entitled to rely on and is an ADR's act rather than an editorial correction,
the same test that governs `[FROZEN]` clause repairs.

## What forces it

Nothing currently does, and that absence is the finding. `RUNBOOK.md`'s
provisional ledger names VT-30's owning phase as **4**, already shipped, with
a falsifier ("E2E-04 and E2E-05 still unwritable after phase 4") that
`kb-decision-0012`'s own long-form record voids as "a delivery check wearing
a falsifier's clothes … the ledger cell is owed a correction" — a correction
nobody made. No phase after 4 mentions VT-30 by name; "Wattline," the
workload the specification's marker names, does not occur in `RUNBOOK.md` at
all; and phase 10's text, which is where a Postgres append path would land,
never mentions multi-guard pushdown or the `min()` collapse. Phase 12's exit
criterion instructs an auditor to check provisional clauses **against the
ledger rather than against prose**, so VT-30 clears that audit unexamined,
on a row naming a phase that has already shipped — the marker is on the
published surface, behind no feature flag, and the mechanism meant to catch
exactly this case does not bind here.

## Ordered sub-questions

1. Does the marker text get corrected now (both instruments named precisely,
   "harness has no multi-guard scenario" replacing "harness is unbuilt"),
   independent of when the falsifier itself is chased down?
2. Who schedules the multi-guard benchmark scenario, and in which phase —
   since nothing on the current runbook does?
3. Does the Postgres append path's eventual design get asked to weigh
   `happenstance-sqlite`'s statement-per-guard precedent before VT-30's first
   limb is evaluated against it?
