---
id: kb-open-question-es-11-sqlite-ceiling-sample-cost-001
title: ES-11's ceiling-first sample costs SqliteEventStore::read 635ms under contention, and nothing has measured a remedy
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ES-11 requires a read's ceiling to be fixed no later than the first poll, and happenstance-sqlite's
  read() cannot spawn before that poll because it may run with no ambient runtime, so sample_ceiling
  takes the connection mutex synchronously on the polling thread, before the spawn_blocking hop that
  everything else in the read path defers. Measured against a live SQLite database under WAL with a
  second connection holding BEGIN IMMEDIATE for 750ms — well inside the busy timeout, so the call
  still succeeds — the first poll of read() stalls 635ms, 39.3x the idle floor, and a control (the
  same crate's projection-store commit, which does not need a pre-spawn sample) stalls only 27.5ms
  under identical contention. The brief that measured this (sqlite-blocking-seam.md) recommends no
  remedy: it names two candidate fixes (a cooperative re-poll that turns the stall into a scheduling
  wait but needs a waker registration the read stream's state machine does not have, and a second
  connection dedicated to ceiling reads, since WAL means a writer does not block a reader and the
  serialization is the adapter's own choice) and declines to pick between them because neither has
  been measured and the second is entangled with an unrelated, higher-priority, breaking-change
  question about whether the store gains a Clone/connect() capability. It asks instead that this be
  recorded as a second, native-adapter data point bearing on ES-11's own [PROVISIONAL] marker, whose
  stated falsifier condition names a one-shot-HTTP transport rather than a native adapter under lock
  contention.
depends_on: []
related:
  - kb-decision-0011
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md
last_reviewed: 2026-09-07
---

# ES-11's ceiling-first sample costs SqliteEventStore::read 635ms under contention, and nothing has measured a remedy

## What is true today

`SqliteEventStore::read` is deliberately not `async` — the same constraint
that keeps `EventStore::read`'s stream `Send` in generic code — so it may be
called with no runtime in scope, where `spawn_blocking` would panic. The
adapter's own module doc states the forced consequence: the blocking hop has
to be deferred to the first `poll_next`, which by construction runs under an
executor, making the laziness load-bearing rather than a nicety. ES-9
governs `ReadOptions::from` semantics; ES-11 is the adjacent clause and
requires that a read's ceiling — the position beyond which it will not see
newly appended events — be fixed no later than that first poll, so a resumed
read is a real snapshot rather than one that silently grows while iterated.

Satisfying both at once means `sample_ceiling` has to take the connection
mutex and run its `SELECT max(position)` *before* the spawn — on whatever
thread called `poll_next`, ahead of the hop everything else in the read path
defers. That acquisition point is not an oversight; the adapter's own code
comment at the call site and a pair of named tests
(`read_result_is_stable_under_concurrent_append`,
`query_items_share_one_snapshot`) record that moving the sample later made
both tests fail roughly one run in two before the fix, which is the
concurrency bug ES-11 exists to prevent.

Measured with a control that isolates the two possible causes — SQLite's own
busy handler on the file write lock versus the adapter's own process-level
`Mutex` — a second connection holding `BEGIN IMMEDIATE` for 750ms (comfortably
inside the 5,000ms busy timeout, so the call succeeds) stalls the first poll
of `read()` by 635.056ms, a 39.3x multiple of the ~16ms idle floor measured on
the same machine. The same crate's `SqliteProjectionStore::commit`, run as a
control under identical contention, stalls only 27.5ms, because it has no
comparable pre-spawn sample to take. Under WAL a writer does not block a
reader's `SELECT`; the stall exists because this adapter serializes the
ceiling sample behind the same process mutex the writer holds, which is a
choice the adapter makes rather than a property SQLite imposes.

## What is not decided

Whether either of two named candidate remedies is worth building, and if so
which. A cooperative re-poll — returning `Poll::Pending` from
`sample_ceiling` and waking when the mutex frees — removes the hard stall in
exchange for a real rewrite: `SqliteReadStream`'s state machine has no waker
registration today, and its current shape is defended by the two named tests
above, so the rewrite has to preserve exactly the guarantee that motivated
the current design. A dedicated second connection for ceiling reads sidesteps
serialization entirely, since WAL permits it, but needs the store to carry an
origin (a path or connection factory) it does not have today — the same
missing field a separate, higher-priority, 0.2.0-dated question about
`Clone`/`connect()` semantics needs, so deciding this in isolation risks
settling that question's premise as a side effect. Neither remedy has been
measured; only the problem has.

## What forces it

Nothing forces a remedy today — ES-11 is `[PROVISIONAL]` and CF-34 puts
performance outside the conformance bar by construction, so no gate step can
see this stall and none will start seeing it without a deliberate decision to
add one. The measurement is offered as evidence for the next scheduled review
of ES-11's own maturity marker, whose stated falsifier condition currently
names a one-shot-HTTP transport (unable to sample within one round trip) —
a different failure shape from a native adapter finding the same obligation
expensive under lock contention. This adapter's finding is a second, native
data point for that review, not an independent trigger.

## Ordered sub-questions

1. Does the unrelated `Clone`/`connect()` decision resolve first, since it
   would give the store the origin field a dedicated ceiling connection
   needs, or is that sequencing a coincidence not worth waiting on?
2. If a cooperative re-poll is chosen instead, does the waker-registration
   rewrite risk reintroducing the concurrency failure the two named tests
   were written to catch, and does the fix need new coverage beyond those two
   before it can be trusted?
3. Should this finding change ES-11's `[PROVISIONAL]` marker or its stated
   falsifier condition, or does it simply sit as supporting evidence until a
   HTTP-transport adapter also reports against it?
