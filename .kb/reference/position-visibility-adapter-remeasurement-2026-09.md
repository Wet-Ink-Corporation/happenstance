---
id: kb-reference-position-visibility-adapter-remeasurement-001
title: The built Postgres adapter costs nothing in steady state and 4799.3 ms of staleness behind a five-second hold
kind: reference
status: accepted
authority_tier: note
summary: >-
  The 2026-09 remeasurement against the built happenstance-postgres adapter (pool,
  transaction tied to append's async boundary, cursor, error mapping), taken over
  the unix socket inside the container under fsync=on. Steady-state medians 0.985,
  1.131 and 1.026 at 1, 8 and 32 clients — a ±20% spread that cannot resolve
  phase 2's 0.99-1.03 effect but shows no cost large enough to matter. Staleness:
  a paired naive-arm baseline control of 0.521/0.595 ms (unloaded/behind a hold)
  against arm C's 0.593 ms unloaded and 4799.3 ms behind an unrelated five-second
  held write transaction on the same cluster.
depends_on: []
related:
  - kb-decision-0024
  - kb-reference-position-visibility-experiment-001
  - kb-decision-0013
  - kb-open-question-global-vs-boundary-visibility-001
  - kb-open-question-postgres-arm-c-cost-001
source_paths:
  - .kb/_intake/2026-09-07-adr-0024-position-visibility-mechanism.md
  - experiments/position-visibility/results/adapter-remeasurement.md
  - experiments/position-visibility/results/adapter-remeasurement.txt
last_reviewed: 2026-09-07
---

# The built Postgres adapter costs nothing in steady state and 4799.3 ms of staleness behind a five-second hold

## What this is a pointer to

The full instrument, raw run output and method live in
`experiments/position-visibility/results/adapter-remeasurement.{md,txt}`, outside
the workspace and the gate. This atom is the citable summary of what that pass
measured. The decision it feeds — ADR-0024's choice of `xid8` +
`pg_snapshot_xmin` as the mechanism a Postgres event store adapter buys position
visibility with — is `kb-decision-0024`'s, not this atom's; this record supplies
only the numbers.

## Why a second measurement, against `kb-reference-position-visibility-experiment-001`

Phase 2's experiment measured four SQL scripts through `pgbench`, against real
PostgreSQL but with no pool, no transaction lifetime tied to a trait method, no
cursor and no error mapping — a gap the open question
`kb-open-question-postgres-arm-c-cost-001` names explicitly. This pass closes
that gap by calling the shipped `PostgresEventStore` directly rather than a bare
SQL script, on a different date, host and harness. Per the dating rule this
reference layer holds, a later measurement with different methodology joins the
corpus as its own atom rather than editing the one it does not repeat.

## The pairing

**Baseline** is `PostgresEventStore::new_naive` — the shipped adapter with the
visibility predicate removed, which is unguarded `nextval()`. **Arm C** is
`PostgresEventStore::new`. Same pool, same transaction lifetime, same cursor,
same error mapping, same schema, same append path, so any difference is the
predicate's alone. Conditions: PostgreSQL 17.10, `fsync=on`,
`synchronous_commit`/`wal_sync_method`/`full_page_writes` all `on`, 200 max
connections, client running inside the container over `/var/run/postgresql`,
10,000 seeded rows with a 5 s warmup per level.

## Steady state — no measurable cost

Ratios above 1.0 are faster than baseline, each taken against the mean of the
baselines immediately before and after it, three repetitions per level: medians
0.985 (1 client), 1.131 (8 clients), 1.026 (32 clients), spread about ±20%.
Phase 2 measured the SQL-level effect at 0.99-1.03, which sits inside this run's
noise floor — the measurement cannot resolve the cost, and states that rather
than quoting a point estimate. Baseline drift within one paired triple ran
1.01-1.88 on the measuring host's virtual disk, which is why the precision is
what it is rather than tighter.

## Staleness — the number that characterises the mechanism

Nine samples per cell, milliseconds:

| arm | control (unloaded) | behind a 5 s hold |
|---|---|---|
| baseline (unguarded) | 0.521 | 0.595 |
| arm C | 0.593 | **4799.3** |

The unguarded row is the control phase 2 lacked: it compared arm C against
itself, which shows the frontier moves but not that the frontier is what moved.
Here, under an identical hold, the unguarded arm is unaffected at 0.595 ms while
arm C rises to 4.8 seconds — same server, same hold, same load, only the
predicate differs. Unloaded, the mechanism is free (0.593 ms against 0.521 ms,
overlapping). Behind a holder, the bound is the whole holder: 4799.3 ms against
a 5,000 ms hold is not a fraction of the transaction, it is the remainder of it,
and it is a property of the cluster — the holder in this measurement writes to
an unrelated table the store under test has never heard of.

## What this does not establish

The steady-state ratio is not resolved to a point estimate at this precision;
that is a separate, still-open measurement need. Whether ES-10's global framing
should narrow to a per-boundary invariant that could make a cheaper mechanism
(B-tag, measured at 0.935 at 64 writers in the phase 2 experiment) sufficient is
inherited from `kb-decision-0013` and is not decided by this measurement.
