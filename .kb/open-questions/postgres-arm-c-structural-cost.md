---
id: kb-open-question-postgres-arm-c-cost-001
title: Whether a real Postgres adapter can express arm C cleanly
kind: open_question
status: accepted
authority_tier: note
summary: >-
  The mechanism by which a Postgres adapter buys position visibility is settled — xid8 plus
  pg_snapshot_xmin, the only arm that passed the inversion detector on both writer pairs while
  leaving writers unserialised. What is not settled is whether a real adapter can express it
  cleanly through sqlx and at what structural cost, because the experiment measured four SQL
  strategies and not four implementations of the port: no connection pooling, no transaction
  lifetime tied to a trait method, no cursor, no error mapping. The staleness the arm buys is also
  unpriced against a real workload — 0.688 ms unloaded, but 4010.719 ms behind an unrelated
  five-second write in an unrelated database, and nothing yet says which of those a caller should
  plan for. Refuted by an adapter that cannot express arm C cleanly, or whose frontier staleness is
  unacceptable under load. Owned by phase 10 and by ADR-0024, which also owns the choice of
  happenstance-postgres's actual mechanism.
depends_on: []
related:
  - kb-decision-0013
  - kb-reference-position-visibility-experiment-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - docs/adr/0013-position-assignment-and-visibility.md
  - experiments/position-visibility/
  - crates/happenstance-postgres/src/lib.rs
  - RUNBOOK.md
last_reviewed: 2026-08-10
---

# Whether a real Postgres adapter can express arm C cleanly

## What is true today

ADR-0013 lifts ES-10 (the global visibility invariant) to `[FROZEN]` on the strength of a phase-2
experiment (`experiments/position-visibility/`) run against a real PostgreSQL with `fsync=on`. The
experiment tested four SQL strategies for buying position visibility — a baseline, arm A (full
serialisation), arm B-const (a single constant advisory lock), arm B-tag (a per-boundary advisory
lock), and arm C (`xid8` + `pg_snapshot_xmin`) — and arm C is the only one that "both passes the
inversion detector on both writer pairs **and** leaves writers unserialised," with throughput
ratios of 0.987 / 0.993 / 1.015 / 1.026 against a bracketing baseline at 1 / 8 / 32 / 64 clients.

The ADR is explicit that this settles the mechanism and nothing more. Its own "What this ADR leaves
open" table states: "**Whether a real Postgres adapter can pass the rule, and at what structural
cost.** The mechanism is settled; its structural costs are not, and the experiment measured four
SQL strategies rather than four implementations of `SendEventStore`." The experiment's own
limitations section is quoted directly in decision §2: "Nothing here is an adapter... This measures
four SQL strategies, not four implementations of `SendEventStore`." Four concrete gaps are named
between "a SQL strategy passed" and "an adapter passed": no connection pooling, no transaction
lifetime tied to a trait method's async boundaries, no cursor, no error mapping — all real
`happenstance-postgres` concerns that the experiment's harness did not have to solve.

The ADR also documents, as part of arm C's cost rather than as a separate finding, that adapters
choosing this mechanism give up read-your-own-writes: "`append` returning `Ok(P)` does not promise
that the next `head()` is at or above `P`," with staleness "bounded by the longest open write
transaction *anywhere in the cluster*" — measured at 0.688 ms with no holder and 4010.719 ms behind
an unrelated five-second write in an unrelated database
(`experiments/position-visibility/results/staleness_pinned.txt`). That range is wide enough that
which end a real caller should plan for is itself unanswered.

## What is not decided

Whether `happenstance-postgres`, built against `sqlx` with real connection pooling and a
transaction whose lifetime is tied to the `EventStore::append` trait method's async boundary, can
express arm C's `xid8` + `pg_snapshot_xmin` read predicate cleanly, or whether the four named gaps
turn out to cost real design compromise. Separately, whether the staleness this arm buys is
acceptable under a real workload — the measured range spans four orders of magnitude between
unloaded and behind an unrelated five-second write, and nothing yet says which magnitude a caller
of `happenstance-postgres` should be told to expect.

## What forces it

Phase 10, where `happenstance-postgres`'s actual implementation lands, and ADR-0024, which
CLAUDE.md's own open-questions section names as owning "how a Postgres adapter buys position
visibility" and which the RUNBOOK schedules to settle the adapter's real mechanism. ADR-0013 is
explicit that it is not that ADR: "This ADR needs one affordable mechanism to exist; it does not
choose the adapter's." So arm C's structural cost is unmeasured until someone actually writes
`happenstance-postgres` against it.

## Ordered sub-questions

1. Does `sqlx`'s connection-pooling and transaction-lifetime model let a real `append` hold the
   snapshot arm C needs without contorting the trait method's signature or lifetime, or does it
   force a design compromise the experiment's bare harness never had to make?
2. Once a real adapter exists, does `nothing_below_an_observed_position_appears_later` (the rule
   the whole ES-10 lift rests on) still pass against it, given that rule's own separately-tracked
   poll-count limitation?
3. What staleness bound should `happenstance-postgres`'s documentation actually promise callers —
   is 0.688 ms unloaded representative, or does phase 10's real workload testing land closer to
   the 4010.719 ms figure under contention, and does that change whether arm C is the right choice
   at all?
4. If arm C turns out structurally expensive or the staleness bound proves unacceptable, does
   ADR-0024 reconsider arm B-tag (per-boundary, cheaper) in light of how phase 6 has by then shaped
   the projection checkpoint — tying this question to the global-versus-per-boundary open question
   this same ADR raises?
