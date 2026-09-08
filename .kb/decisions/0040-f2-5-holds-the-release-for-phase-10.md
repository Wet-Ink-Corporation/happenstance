---
id: kb-decision-0040
title: 0.2.0 waited for phase 10, and the hold was lifted the same day
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0040
reversibility: high
phase: 10
supersedes: null
superseded_by: null
summary: >-
  Record F2-5-HOLD: 0.2.0 was held for phase 10 because the testkit's rule
  dropped_append_future_leaves_no_partial_batch had only ever been proved against
  an Rc<RefCell<...>> store, never a severed connection. The finding that produced
  the hold was half wrong on measurement: arm 2 (the landed == 0 branch) is reached
  by two registered stores, PreCommitPositionStore and AwaitAcrossBorrowStore, on
  every run of tests/mutation_coverage, in both feature configurations, and has
  been since the rule's first commit. What survived as the real residual -- no store
  with a real medium under it has ever reached either arm -- was discharged the
  same day the hold was taken: lane/postgres-neon-stores already carried a
  substantially built happenstance-postgres whose AFTER INSERT trigger aborts a
  live multi-row INSERT mid-batch on a real pinned server, because append is one
  multi-row statement. The hold rested on a stale RUNBOOK.md status row reporting
  phase 10 as not started; it was actually well underway. The decision to hold was
  sound on the information available, and the information was wrong -- recorded as
  its own class of defect rather than folded into the correction.
depends_on: []
related:
  - kb-reference-mutation-coverage-arm-two-001
  - kb-decision-0010
  - kb-governance-what-may-refute-a-finding-001
  - kb-open-question-cf-5-per-rule-or-branch-001
source_paths:
  - .kb/_intake/f2-5-holds-the-release-for-phase-10.md
  - .kb/_intake/remediation-2026-09-04-briefs/es-22-arm-two-is-reached-the-finding-is-wrong.md
last_reviewed: 2026-09-07
---

# 0.2.0 waited for phase 10, and the hold was lifted the same day

## Decision

`0.2.0` waited for phase 10. The residual is answered and the hold is lifted,
both on the same day it was taken, 2026-09-06.

## What F2-5 claimed, and what survives

The pre-publication audit filed F2-5 as two claims about the testkit rule
`dropped_append_future_leaves_no_partial_batch`: that one of its two arms had
never executed, and that the rule had been certified against nothing but a
strawman.

**The first claim is false, measured.** Arm 2 — the byte-identical
`snapshot_of(&after) == snapshot_of(&before)` comparison — is reached by two
registered stores, `PreCommitPositionStore` and `AwaitAcrossBorrowStore`, on
every run of `tests/mutation_coverage`, in both feature configurations, and has
been since the rule's first commit (`d480446`). The lane sent to build "a store
that suspends before its first write" found two already in the tree and
stopped rather than adding a third; mutating arm 2's comparison forced a
verified `FAILED` on both stores in a live `cargo test` run.

**One sentence survives:** `dropped_append_future_leaves_no_partial_batch` had
never been answered by a store with a real medium under it. Both arms execute
and both are decisive against the instruments in `tests/mutation_coverage`, but
every store that had ever reached either arm was an `Rc<RefCell<…>>` in the
testkit's own test target, where "the future was dropped" meant a local went
out of scope rather than a connection was severed.

A smaller, separable residual also survives: arm 2 is exercised by two mutants
and by no `Kind::ConformantVariant`, so nothing in the portfolio would catch
arm 2 over-specifying. Whether `kb-decision-0010`'s conformant-control
obligation runs per rule or per branch is undecided, and this is the first
place the difference became visible. That question is not settled here.

## The decision as taken, and the argument against it

**The decision:** `happenstance-postgres` — the first adapter in this
workspace with a connection that can be severed rather than a `RefCell` that
can be dropped — is what the residual required, so `0.2.0` waits for it.

The strongest argument against, recorded rather than smoothed over: the review
that raised F2-5 argued its whole class of finding costs the same later,
because no fix here is a breaking change and every one is available at the
same price after `0.2.0` — what cannot be re-run is the release. `RUNBOOK.md`'s
own sequencing had put phase 10 off the trunk deliberately, as a branch that
never rejoins; this decision made it rejoin. And the cost was concrete: phase
10 read as `not started` and estimated at eleven days for two adapters, with
four finished crates and an entire release pass waiting behind it, plus the
repository staying private for that period on a separate, coupled decision.

The argument for it, and it carried the decision: this project's testkit is
the artefact it exists to be trusted, and a conformance suite whose atomicity
rule has only ever been proved against an in-process `RefCell` is asking
adapter authors to trust a guarantee about severed connections on the evidence
of a dropped local. Publishing is what makes the suite's verdict cost
something to be wrong about.

## Why the hold lifted the same day

The decision rested on a report that phase 10 was `not started` and eleven
days away, read out of `RUNBOOK.md`'s status table. That table was stale.
`lane/postgres-neon-stores` already held a substantially built
`happenstance-postgres` when the hold was taken. Its own commit records:
`MID_BATCH_FAULT` was declined earlier because `append` did not yet exist; once
it existed, the fixture armed an `AFTER INSERT` trigger and both fault rules
passed, because `append` is one multi-row statement and the raise aborts
precisely the unit it promises atomic. A trigger raising mid-batch on a live
pinned server is a real medium in exactly the sense the residual required, and
the lane merged.

**What the episode is evidence of.** A stale status row in the plan of record
deferred a release — the same class of defect this release pass spent its
length repairing elsewhere in the falsifier ledger and clause census, occurring
in the same file while that repair was in progress. This is recorded as its
own defect class rather than folded into the correction, because the failure
was in the information, not in the judgement applied to it: the hold was the
right call on what was known at the time.

## What this does not settle

Whether `happenstance-postgres` reaching both arms with a severed connection
is sufficient rather than merely necessary is not re-litigated here — building
the adapter was necessary and turned out, on this run, to be sufficient too.
Whether `kb-decision-0010`'s conformant-control obligation runs per rule or per
branch remains open and travels with the residual rather than being decided by
lifting the hold. The fallback of accepting the residual in writing and
publishing without a real-medium adapter was available throughout and was not
taken; taking it later would itself be a decision, not a drift.

## Alternatives rejected

Publishing `0.2.0` without holding for phase 10, accepting the residual in
writing. Rejected on the argument above: the testkit's own credibility is what
the whole project sells, and this was the first opportunity to test its
central atomicity rule against a real severable medium rather than a
process-local one.
