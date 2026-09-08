---
id: kb-open-question-testkit-contention-tolerance-001
title: A busy store and a broken store are the same Attempt, and the suite cannot tell them apart
kind: open_question
status: accepted
authority_tier: note
summary: >-
  CF-33 is [FROZEN] and forbids a conformance rule a clock, an elapsed-time measurement or an
  assertion on an operation count, which guarantees the suite cannot distinguish a store that is
  momentarily contended from one that is wrong: both surface as the same failed Attempt. ADR-0022
  section 12 says explicitly that the contender count is not its call, so no decision owns the
  question either. What the 2026-09-03 busy-timeout measurement adds is that the case is no longer
  hypothetical - busy > 0 was observed at the shipped CONTENDERS = 64, one launch in seven, the
  first nonzero busy count in this tree - so an adapter can now be contended inside the gate's own
  configuration. The 2026-09-04 brief then sharpened the fork in three ways. The defect is three
  rules and not one classification arm: two of them assert that every contender commits, so they
  fail a busy refusal however the private Attempt enum is spelled, and a tolerance is therefore a
  change to what those rules assert rather than a fourth arm. CONTENDERS moved 8 to 64 the day
  after 0.2.0-alpha.1 published, with no version bump and no CF-31 citation, so a lowering is a
  revert to the published value and whether CF-31 governs the constant at all is itself unsettled.
  And the third shape is not nothing at all but AppendError::Busy in happenstance-core, refused in
  an earlier draft on the precedent of an API convenience when the nearer precedent - VT-25 /
  CF-40, the identical harm on the identical channel, minted at phase 4 with zero adapters - points
  the other way. Both live arms are gated by the same instrument, which does not exist. Forced by
  the first adapter whose conformance run fails on contention rather than on conformance, and by
  whoever next proposes changing CONTENDERS.
depends_on:
  - kb-decision-0042
related:
  - kb-decision-0034
  - kb-decision-0022
  - kb-decision-0010
  - kb-open-question-poll-count-rule-strength-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-open-question-cf-33-cf-34-scope-001
  - kb-open-question-adapter-version-lockstep-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-nested-block-on-lost-wakeup-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/remediation-2026-09-04-briefs/transient-contention-tolerance.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - crates/happenstance-testkit/src/concurrency.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
---

# A busy store and a broken store are the same Attempt, and the suite cannot tell them apart

## What is true today

`crates/happenstance-testkit/src/concurrency.rs` declares `pub const CONTENDERS: usize = 64` and
runs the shipped concurrency rules against that many simultaneous handles onto one fixture. CF-33
(`spec/SPECIFICATION.md:8720`) is `[FROZEN]`: *"No conformance rule may read a clock, measure
elapsed time, or assert an operation count."* The rule exists so a conformance run is deterministic
and portable — no wall-clock deadline, no watchdog, nothing that varies with the machine — and its
cost is unavoidable given what it forbids: a store contended for a moment longer than usual and a
store that is genuinely broken both surface through the same channel, a failed `Attempt`, and CF-33
forecloses the one mechanism (a clock) that could tell them apart from inside the suite. ADR-0022
section 12 states plainly that the contender count itself is *"not this record's to re-open"* — it
supplies a number and does not own the question of whether that number, or any number, needs a
tolerance around it.

What was hypothetical became observed in the 2026-09-03 busy-timeout measurement
(`kb-reference-busy-timeout-margin-001`): at the shipped `CONTENDERS = 64`, under the busy handler
this crate installs, a nonzero `busy` count appeared in roughly one launch in seven — the first
nonzero busy count recorded anywhere in this tree, three attempts in 6,720, all inside one launch.
Every prior run at this contender count had `busy = 0` across five complete repetitions. The margin
against the 5,000 ms budget stays comfortable (1.31x–1.38x across the core sweep in the same
experiment), so nothing failed — but an adapter can now be contended inside the gate's own
configuration without being wrong, and the suite has no vocabulary to say that is what happened.

**The defect is three rules, not one classification arm.** `Attempt` is private
(`concurrency.rs:247`), so a fourth arm costs nothing in semver — but re-spelling it repairs only
`exactly_one_of_n_contenders_commits`'s `failures.is_empty()` assertion. The other two sites fail on
a *count*: `positions_are_unique_under_concurrent_appends` asserts `committed.len() == CONTENDERS`,
because an unconditional append has nothing to be rejected by, and
`append_returns_the_callers_own_last_position` panics per contender on any non-commit. Those are
exactly the two rules that carried `busy = 1` and `busy = 2` in the incident run. A tolerance is
therefore a change to *how many contenders three rules require to commit*, and for two of them the
current requirement is all of them.

**`CONTENDERS` has already moved after publication, unversioned.** `0.2.0-alpha.1` shipped on
2026-08-16 carrying `CONTENDERS = 8`; the raise to 64 landed the following day, touching no
`Cargo.toml` and citing no clause. So the in-tree constant and the published one differ, a lowering
to 8 is a *revert to the published value* rather than a one-way move, and whether CF-31 `[FROZEN]`
governs a contention level at all — or is only about a rule's *meaning* — is itself now unsettled
rather than a bar. Any reasoning that treats publication as still ahead of this constant is wrong on
its face.

## What is not decided

Whether the tolerance is bought on the fixture side or the contract side, and whether it is bought
at all. **On the fixture side**, its placement is forced: `ConcurrentFixture`'s blanket impl is
closed to required items, so a declaration lands on `Fixture` and lands **defaulted** — the policy
`kb-decision-0042` states, and the fifth consecutive instance of it. Its floors must be structural
minima rather than a tolerated fraction, which is what CF-34's `Rejects:` forbids by name, and its
surface is additive to add and **breaking to remove**. **On the contract side**,
`AppendError::Busy` in `happenstance-core` adds no published `Fixture` surface, helps the
out-of-tree author who has declared nothing, and switches the liveness assertion off per error
rather than per fixture. It was refused in an earlier draft on the precedent of `index_arms()`, an
API convenience on a query type; the nearer precedent is VT-25 / CF-40, which repaired the identical
harm — a refusal arriving on the `Store(E)` channel that a caller must tell from a real failure — as
a pair of core variant, defaulted fixture fact, one rule and four mutants, minted at phase 4 with
zero adapters in the tree. On that precedent the variant is live, not refused. What still cuts the
other way is that `happenstance-sqlite` is the only event-store adapter to have run the suite, and
*one implementor is not a spread*. **Nothing at all** remains an arm: a suite that tolerates a busy
store has stopped measuring conformance and started measuring luck.

**Both live arms are gated by the same missing instrument.** No registered racer produces
`Attempt::Failed`, so the assertion either arm would soften has no named wrong implementation today,
and the floors either introduces would arrive with nothing able to fail them. The instrument is a
decorator over `MemoryEventStore` whose `append` refuses the first *m* callers with a transient
store error; it discriminates the two arms as well as gating them, because a per-error channel and a
per-fixture capability behave differently against it.

ADR-0034 already settled a related but distinct question — who would mint a new `CF-` clause, not
whether this one should exist. That answers where a capability would live if it is created; it does
not answer whether it should be.

## What forces it

The first adapter whose conformance run fails on contention rather than on an actual defect —
Postgres and Neon are named in `RUNBOOK.md` as the deployments least likely to serialise writers the
way `MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all do, which is exactly
the axis a contention-tolerant retry would need to be honest about, and the two unlike storage
shapes whose agreement is the named re-open trigger for the contract-side arm. Independently,
whoever next proposes changing `CONTENDERS` — raising it moves the busy-timeout margin further from
its current 1.31x–1.38x floor, lowering it trades contention realism for gate speed and reverts to
the published value, and either change must amend `RUNBOOK.md`'s phase-8 and phase-10 proof
artefacts and the status table in the same commit or recreate the discrepancy the raise cured.
