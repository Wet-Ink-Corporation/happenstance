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
  question about whether the store gains a Clone/connect() capability. It asked instead that this be
  recorded as a second, native-adapter data point bearing on ES-11's own [PROVISIONAL] marker, whose
  stated falsifier condition named a one-shot-HTTP transport rather than a native adapter under lock
  contention. That third sub-question is now answered, and the atom stays open. happenstance-neon
  reported at phase 10b and kb-decision-0061 is that review; both halves are decided — the marker
  moves, rewritten to record a fired falsifier rather than predict one, and it stays [PROVISIONAL],
  because what is now open is not whether a one-shot-HTTP shape fails but whether a conformant one
  exists at all (kb-open-question-one-shot-http-es-11-001). ES-11's asynchronous-driver sufficiency
  condition was narrowed in the same change to require ordering against a later append by something
  the store itself honours, which this adapter's ceiling sample satisfies and a one-shot HTTP
  transport cannot. Two things follow for the SQLite half. The clause text this atom quotes has moved
  under the quotation — ES-11's Rejects: bullet ended "It is conformant today", written 2026-08-06
  and untouched through two phases, and was repaired in ADR-0061's change. And sub-questions 1 and 2
  are untouched: ADR-0061 decides nothing about happenstance-sqlite, neither candidate remedy has
  been measured, and the 635ms stall is exactly where it was, so this atom is not resolved.
depends_on: []
related:
  - kb-decision-0011
  - kb-decision-0061
  - kb-open-question-one-shot-http-es-11-001
  - kb-decision-0058
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/sqlite-blocking-seam.md
  - .kb/_intake/2026-09-08-adr-0061-es-11s-sufficiency-condition-assumed-a-queue.md
  - .kb/_intake/2026-09-08-es-11s-falsifier-fired-on-the-adapter-it-named.md
last_reviewed: 2026-09-09
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

The clause this atom argues around has since moved under it.
`kb-decision-0061` narrowed ES-11's asynchronous-driver sufficiency condition
— a read must now be spawned at its first poll *and* ordered against a later
append by something the *store* honours — and this adapter meets the narrowed
form for precisely the reason the stall exists: the pre-spawn sample takes the
same process mutex the writer takes, so the ordering is one the store honours.
The same change repaired ES-11's `Rejects:` bullet, which ended *"It is
conformant today"* — written 2026-08-06 and untouched through the two phases
in which the adapter that falsified it was built and shipped. Read the current
clause text, not the wording this atom preserves.

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
measured; only the problem has, and `kb-decision-0061` changed none of that —
it decides nothing about `happenstance-sqlite`, and the 635ms is where it was.

## What forces it

Nothing forces a remedy today — ES-11 keeps `[PROVISIONAL]` and CF-34 puts
performance outside the conformance bar by construction, so no gate step can
see this stall and none will start seeing it without a deliberate decision to
add one. What has changed is the review this measurement was being held for:
it has happened. `happenstance-neon` — the one-shot-HTTP adapter the marker
named — reported at phase 10b, `kb-decision-0061` weighed both findings, and
the SQLite stall did not become a clause problem there. It stayed a cost this
adapter pays to satisfy a clause it does satisfy. So there is no longer a
scheduled marker review to hold this for, and whatever forces a remedy next
will be an operating complaint about read latency rather than a maturity pass.

## Ordered sub-questions

1. Does the unrelated `Clone`/`connect()` decision resolve first, since it
   would give the store the origin field a dedicated ceiling connection
   needs, or is that sequencing a coincidence not worth waiting on?
2. If a cooperative re-poll is chosen instead, does the waker-registration
   rewrite risk reintroducing the concurrency failure the two named tests
   were written to catch, and does the fix need new coverage beyond those two
   before it can be trusted?
3. ~~Should this finding change ES-11's `[PROVISIONAL]` marker or its stated
   falsifier condition, or does it simply sit as supporting evidence until a
   HTTP-transport adapter also reports against it?~~ **Answered by
   `kb-decision-0061`, in both halves.** The HTTP-transport adapter reported.
   The marker changed — rewritten to record the falsifier that fired rather
   than predict one — and the falsifier condition changed with it, narrowing
   to require store-honoured ordering. The marker stays `[PROVISIONAL]`,
   because what it now holds open is a different question and belongs to
   `kb-open-question-one-shot-http-es-11-001`: whether any conformant
   one-shot-HTTP shape exists at all. Sub-questions 1 and 2 survive that
   answer untouched, which is why this atom stays open rather than resolving
   on a one-in-three.
