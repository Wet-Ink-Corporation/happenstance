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
  configuration. What is not decided is whether the fixture contract grows a declared tolerance for
  transient contention, and if so what shape: a Capability the way SECOND_HANDLE and REOPEN are, a
  numeric associated constant the way CF-40's limits are, or nothing at all, on the ground that a
  suite which tolerates a busy store has stopped being a conformance suite. ADR-0034 already
  answers who would mint it rather than whether it should exist - the fixture contract has no
  single owning document, and a CF- clause is minted by the decision that first needs the
  capability - so this question is about the capability, not about its home. Forced by the first
  adapter whose conformance run fails on contention rather than on conformance, and by whoever next
  proposes changing CONTENDERS.
depends_on: []
related:
  - kb-decision-0034
  - kb-decision-0022
  - kb-decision-0010
  - kb-open-question-poll-count-rule-strength-001
  - kb-open-question-adr-0022-falsifiers-fired-001
  - kb-reference-busy-timeout-margin-001
  - kb-reference-nested-block-on-lost-wakeup-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - experiments/busy-timeout-margin/
  - crates/happenstance-testkit/src/concurrency.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
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
nonzero busy count recorded anywhere in this tree. Every prior run at this contender count had
`busy = 0` across five complete repetitions; this is a distinct, less loaded configuration
surfacing a case those runs did not exercise. The margin against the 5,000 ms budget stays
comfortable (1.31x–1.38x across the core sweep in the same experiment), so nothing failed — but an
adapter can now be contended inside the gate's own configuration without being wrong, and the suite
has no vocabulary to say that is what happened.

## What is not decided

Whether the fixture contract grows a declared tolerance for transient contention, and if so what
shape it takes. **A `Capability`**, the way `SECOND_HANDLE` and `REOPEN` are — a boolean a fixture
opts into, reported through the same declined-capability path, that lets a rule retry once before
failing rather than failing on the first busy signal. **A numeric associated constant**, the way
CF-40's `MAX_EVENT_DATA_LEN`-shaped limits are — a fixture-declared retry budget rather than a
boolean, closer to what the busy handler itself already tracks. **Nothing at all**, on the ground
that a conformance suite which tolerates a busy store has stopped measuring conformance and started
measuring luck, and that the right fix for a flaking `Attempt` is a better fixture or a better
contender count, not a new capability that lets the suite shrug.

ADR-0034 already settled a related but distinct question — who would mint a new `CF-` clause, not
whether this one should exist. It confirmed the fixture contract has no single owning document and
that a capability is minted by the decision that first needs it, beside the adapter that needed it.
That answers where this capability would live if it is created; it does not answer whether it
should be.

## What forces it

The first adapter whose conformance run fails on contention rather than on an actual defect —
Postgres and Neon are named in `RUNBOOK.md` as the deployments least likely to serialise writers the
way `MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all do, which is exactly
the axis a contention-tolerant retry would need to be honest about. Independently, whoever next
proposes changing `CONTENDERS` — raising it moves the busy-timeout margin further from its current
1.3x–1.4x floor, and lowering it trades contention realism for gate speed, and either change is
made blind without a stated view on what "busy" should mean to the suite.
