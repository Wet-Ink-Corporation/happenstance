---
id: kb-open-question-postgres-arm-c-cost-001
title: Whether a real Postgres adapter can express arm C cleanly
kind: open_question
status: superseded
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
  Resolved 2026-09-07 by ADR-0024 (kb-decision-0024), at phase 10, which built the adapter and
  re-measured against it. Arm C is expressible cleanly: the trait signature is untouched, and the
  steady-state cost is not resolvable by the measurement rather than absent - medians 0.985 / 1.131
  / 1.026 at 1 / 8 / 32 clients against a spread of about plus or minus 20%, which establishes only
  that nothing large enough to matter hides inside that band, and the rivals cost 16x and 30x.
  Three structural costs were forced and none of them reached the port - a captured runtime Handle
  on every operation because Handle::enter is !Send and would break SendEventStore, the cursor
  carrying that handle so PoolConnection::drop can return its connection, and SERIALIZABLE plus a
  bounded retry on the conditional path. Sub-question 3 is answered and documented rather than only
  recorded: sub-millisecond unloaded, otherwise the remainder - not a fraction - of the longest open
  write transaction anywhere on the cluster, 4799.3 ms against a five-second hold, and it sits on
  happenstance-postgres's crate root where a consumer meets it before any method. B-tag stays
  rejected on the invariant rather than on cost, so sub-question 4's branch never opened. Superseded
  rather than withdrawn because two residuals moved to owners of their own -
  kb-open-question-off-poll-visibility-defect-001 for what sub-question 2's pass turned out to be
  worth, and kb-open-question-global-vs-boundary-visibility-001 for the premise this question
  inherited.
depends_on: []
related:
  - kb-decision-0013
  - kb-decision-0024
  - kb-decision-0001
  - kb-reference-position-visibility-experiment-001
  - kb-reference-position-visibility-adapter-remeasurement-001
  - kb-open-question-off-poll-visibility-defect-001
  - kb-open-question-global-vs-boundary-visibility-001
source_paths:
  - .kb/_intake/0013-position-assignment-and-visibility.md
  - .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md
  - references/adr/0013-position-assignment-and-visibility.md
  - references/adr/0024-position-visibility-mechanism.md
  - experiments/position-visibility/
  - experiments/position-visibility/results/adapter-remeasurement.md
  - crates/happenstance-postgres/src/lib.rs
  - RUNBOOK.md
last_reviewed: 2026-09-07
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

## Resolved 2026-09-07 — the adapter exists, and arm C cost nothing the machine can resolve

Everything above is the state of knowledge on 2026-08-10 and is left exactly as written, per this
layer's README. ADR-0024 (`kb-decision-0024`; long form at
`references/adr/0024-position-visibility-mechanism.md`) now holds the answer, so this atom moves to
`superseded` rather than `withdrawn` — the question was worth asking, and it is what the phase-10
measurement was designed against.

**Sub-question 1 — sqlx's pooling and transaction model.** No compromise reached the port: the
`EventStore::append` signature is untouched. Three real costs were forced inside the adapter, and
they are the answer to "at what structural cost" this atom asked for. Every operation captures a
runtime `Handle`, because `Handle::enter` is `!Send` and taking it per-call would have broken the
`SendEventStore` flavour ADR-0001 exists to preserve. The cursor carries that handle so
`PoolConnection::drop` can return its connection to the pool. And the conditional path runs
`SERIALIZABLE` with a bounded retry, because under `READ COMMITTED` two writers racing one boundary
can both win, and taking a write lock instead is the move this crate exists not to make.

**Sub-question 2 — does the rule still pass.** It does, **and the pass is not evidence.** The naive
arm — the same adapter with the visibility predicate removed — passes
`nothing_below_an_observed_position_appears_later` too, because this adapter advances off-poll. The
mechanism is evidenced by direct observation instead: the probe, and the staleness table below. That
the rule cannot discriminate here is a defect in the instrument rather than in the adapter, and it
is now `kb-open-question-off-poll-visibility-defect-001`'s, which ADR-0024 owns.

**Sub-question 3 — the bound the docs promise.** Sub-millisecond unloaded; otherwise the *remainder*
of the longest open write transaction anywhere on the cluster, not a fraction of it. Nine samples
per cell against the built adapter, with the unguarded arm as the control phase 2 never had
(`experiments/position-visibility/results/adapter-remeasurement.md`): baseline 0.521 ms control and
0.595 ms behind a five-second hold; arm C 0.593 ms control and **4799.3 ms** behind the same hold.
Same server, same hold, same load — only the predicate differs, which is exactly what phase 2's
arm-C-against-itself comparison could not show. The figure is on `happenstance-postgres`'s crate
root (`crates/happenstance-postgres/src/lib.rs`), so a consumer meets it before any method, and this
sub-question is closed in the documentation rather than only in a record.

The steady-state half of the cost is *unresolved rather than free*, and the record says so instead
of quoting a point estimate: medians 0.985 / 1.131 / 1.026 at 1 / 8 / 32 clients with a spread of
about ±20%, against an effect phase 2 put at 0.99–1.03, on a machine whose baseline drifted
1.01–1.88 within a single triple. It establishes only that no cost large enough to matter hides
inside ±20% — which is enough, because arm A and arm B-const cost 16× and 30×.

**Sub-question 4 — reconsider B-tag.** No, and its branch never opened. The condition this atom set
was "if arm C proves structurally expensive"; it did not. B-tag stays rejected on the invariant —
per-boundary where ES-10 is global — rather than on cost, which matters because B-tag is *cheaper*
(0.935 at 64 writers) precisely by buying something weaker, so ranking the arms by throughput would
have chosen it.

**Two things this resolution does not carry.** ES-10's `[FROZEN]` prose still quotes phase 2's
0.688 ms / 4010.719 ms, which the adapter re-measurement supersedes, and the clause is not edited —
`spec/SPECIFICATION.md` is unedited by phase 10, and ADR-0024 adds no clause of its own. And the
global framing this question inherited from ADR-0013 stays inherited rather than settled: a
boundary-scoped projection checkpoint could make B-tag's weaker invariant sufficient, which is
`kb-open-question-global-vs-boundary-visibility-001`'s.
