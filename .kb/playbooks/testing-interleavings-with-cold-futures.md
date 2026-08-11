---
id: kb-playbook-cold-future-hand-polling-001
title: Testing an interleaving by hand-polling two cold futures
kind: playbook
status: accepted
authority_tier: guideline
summary: >-
  How to make a conformance rule observe a specific interleaving with no executor, no second
  thread and no clock. Build two append futures from one handle, pin them in place, and poll them
  out of order - A, B, B, A - to model a slow transaction that publishes late. It works because a
  Rust future is cold until polled, which is the property a reader coming from C# or JavaScript
  will not expect: a Task is already running when you hold it, and a future is not, so the test
  owns the schedule rather than racing it. That keeps the rule inside two constraints the fixture
  contract imposes: no Send bound, and no wall clock. Its known blind spot is that the window
  varies with the adapter's poll shape, so an adapter whose append needs three or more polls may
  slip through. The direct repair, a poll-budget capability on the fixture, was considered and
  rejected at this phase: it would be the first non-boolean capability, it sits close to the
  no-clock line, and calibrating it needs a real I/O-bound adapter that does not exist yet.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0010
  - kb-reference-position-visibility-experiment-001
  - kb-open-question-poll-count-rule-strength-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - references/adr/0013-position-assignment-and-visibility.md
  - crates/happenstance-testkit/src/concurrency.rs
  - crates/happenstance-testkit/src/fixtures.rs
last_reviewed: 2026-08-10
---

# Testing an interleaving by hand-polling two cold futures

## The problem

A conformance rule needs to observe a specific interleaving — two concurrent `append`s where the
writer that starts *second* commits and becomes visible *first* — to catch an adapter that
assigns a position before it commits, which violates the store's visibility invariant: once a
reader has observed an event at position *P*, no later read may yield an event at a position ≤
*P* that was not already visible. The fixture contract every conformance fixture implements
declares no `Send` bound and no wall clock, so the usual tools — a second OS thread, a timed
race, a real async executor scheduling two tasks concurrently — are all unavailable or
unreliable: a real scheduler's interleaving is exactly the thing under test, and racing it does
not guarantee it fires.

## The technique

Build two `append` futures from one store handle, `pin!` them in place, and poll them **by
hand**, in a chosen order — A, B, B, A — reading after every step. That order is not arbitrary:
it models a slow transaction (A) that takes a low position and publishes late, resumed after the
fast transaction (B) that takes a higher position and publishes first. Plain alternation
(A, B, A, B) was measured not to reproduce the failure; the reversed order is what does.

This works, and needs no executor, thread, or clock, because a Rust future is **cold**: calling
an `async fn` runs none of its body and returns an inert value. Nothing happens until something
calls `poll`. Constructing both futures is therefore not starting two writers — it is building
two suspended state machines — and the test owns the schedule completely rather than racing it.
`pin!` is what makes them pollable in place without a heap allocation, since `poll` requires
`Pin<&mut Self>`.

This is the point most likely to mislead someone arriving from C# or JavaScript: a `Task` or a
`Promise` is already running by the time you hold a reference to it, so reproducing a chosen
interleaving there needs a custom scheduler (a `SynchronizationContext`, an injected mock clock)
built for the purpose. In Rust, the ordinary building block — a bare future plus manual polling —
already gives full control, no framework required.

## The known blind spot

The rule drains whatever the hand-written schedule leaves unfinished, and its strength varies
**silently** with the adapter's poll shape: an adapter whose `append` genuinely needs three or
more polls to reach the point the schedule is testing for is not exercised in the window the
rule looks at, and the rule cannot fail against it. The in-memory reference store passes
trivially because its `append` body contains no `.await` at all — correctly, since a store with
no suspension point cannot have a suspension-point bug — but the same property means a real,
I/O-bound store's actual window can sit outside a fixed four-step schedule entirely.

## The repair considered and rejected

The direct fix is a poll-budget associated constant on the fixture contract that the schedule
consumes to calibrate how many polls to interleave. It was rejected at this phase for three
reasons: it would be the first fixture capability that is not a plain yes/no answer, in a
contract most of whose capability constants are already frozen; it sits close to a neighbouring
rule that forbids a conformance rule from asserting on an operation count, and the distinction
between a budget the schedule *consumes* and a count the rule *asserts on* deserves more care
than a passing decision; and calibrating the right budget needs a real adapter whose `append`
actually suspends more than once, which does not exist yet. The instrument that would settle it —
a poll-padding decorator inserting *n* extra `Pending` returns into a fixture's `append`,
observing whether the rule still rejects it — is named and cheap, and waits for that adapter.

## When to reach for this

Any interleaving test where the property under test is about *ordering of visible effects*
rather than *timing*, and where the test environment forbids a real executor, a second thread, or
a wall clock. The recipe generalises past this one rule: construct the futures you need to
interleave, pin them, and drive `poll` yourself in the order that models the failure you are
trying to catch.
