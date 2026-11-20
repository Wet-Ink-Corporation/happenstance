---
id: kb-decision-0115
title: SY-40 is frozen, on its rule green on all five adapters that implement ingest
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0115
reversibility: low
phase: 20
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-20 by the owner, who authorised the freeze for 0.8.0 once a live Neon run of
  SY-40 was green, as ADR-0110's answer D-10 had put it. SY-40 moves from PROVISIONAL to FROZEN.
  The freeze condition ADR-0110 wrote into the marker is met: a_removed_event_is_not_readmitted
  ran green, not declined, on SQLite and the Durable Object on the host in the gate, under
  workerd warm and cold, against a live PostgreSQL in live-postgres and against a live Neon
  endpoint in live-neon, all on CI run 44344907519 (#516's merge into phase-21-prep). The only
  declined run is the sync testkit's own stand-in fixture, which inherits the default. The
  marker's falsifier stands as the reopening condition. The rule still reaches the record through
  handles onto one open store, so reopening and restore remain each adapter's own regression
  tests. The ledger's freeze-by-20 row closes.
depends_on:
  - kb-decision-0110
  - kb-decision-0107
  - kb-decision-0066
related:
  - kb-decision-0108
  - kb-decision-0098
  - kb-decision-0028
source_paths:
  - spec/SPECIFICATION.md
  - runbook/ledgers.md
  - crates/happenstance-sync-testkit/src/ingest_suite/ingest_rules/removal.rs
  - crates/happenstance-sync-testkit/tests/wrong_ingest_stores.rs
last_reviewed: 2026-11-20
---

# SY-40 is frozen, on its rule green on all five adapters that implement ingest

**Accepted on 2026-11-20.** The owner authorised the freeze in this session on the condition that
SY-40 had run green against live Neon. No long-form record exists; this atom is the record.

## Context

[ADR-0110](0110-a-removed-event-is-not-readmitted.md) minted SY-40 `[PROVISIONAL]` at `0.8.0` for
#483 and #484. A store records the `EventId` of every event removed outside the port, and ingest
skips a recorded identity instead of re-admitting it or refusing it under SY-38. The owner's
answer D-10 freezes it in the same `0.8.0` change set once it runs on four green adapters, and the
marker wrote the condition out: the rule runs green, *not declined*, on SQLite, the Durable Object
(host and `workerd`), Postgres and Neon. That sets the freeze on the stores whose writers do not
serialise, not on SQLite alone. `runbook/ledgers.md` carried it as `freeze-by-20`, because
`freeze-by-21` is refused.

#517's first item held the freeze back. Neon's removal record and its ingest skip (#503) had been
checked by hand against PostgreSQL 17.10 and by scripted-transport tests, and the risk named was
that Neon's `/sql` batch would refuse the leading `LOCK TABLE event_removed IN SHARE MODE`.

## Evidence

CI run 44344907519 is #516's merge of `main` into `phase-21-prep`, which carries #495, #498, #499
and #503. Its jobs:

| Adapter | Job | Result |
|---|---|---|
| SQLite (`ingest_spike` family) | `gate (ubuntu-latest)` | `a_removed_event_is_not_readmitted ... ok` |
| Durable Object, host shim | `gate` (wasm32 under Node 24) | ran, green |
| Durable Object under `workerd` | `conformance under workerd` (local leg) | ✓ warm (`ingest.test.ts`) and ✓ cold (`ingest-cold.test.ts`) |
| Postgres | `conformance against a live Postgres` | `... ok`, beside the six `removal::` unit tests |
| Neon | `conformance against a live Neon endpoint` | `... ok`, and Neon's own `removal::` tests, against a live endpoint |

So the `LOCK TABLE ... IN SHARE MODE` lead is accepted by Neon's `/sql` batch. The only `SKIP` of
the rule in the run is `local_ingest_conformance`, the sync testkit's stand-in fixture. It inherits
`UNSTATED_INGEST_REMOVAL` and is not an adapter. The deployed `workerd` leg did not run, because
since #515 it runs only from a manual dispatch. The marker names `workerd`, not a deployed object,
and the local leg runs the real runtime.

## Decision

1. **SY-40 is `[FROZEN]`.** The marker keeps ADR-0110's falsifier as what reopens it: a deployment
   in which a removed event must come back on re-delivery, or an adapter that cannot keep its
   allocator across a removal. From `1.0.0`, loosening it is a major.
2. **The rule's recorded limit carries over unchanged.** The rule reaches the record through
   handles onto one open store. So it checks neither that the record survives reopening nor that a
   restore rolls it back, and each adapter's regression tests (ADR-0110's I-4 and I-5) keep those
   halves until the ingest family has a `REOPEN` capability. Freezing does not convert those tests
   into conformance rules. That work, if anyone does it, is additive.
3. **The ledger closes.** SY-40's `freeze-by-20` row and its residual-exposure row are struck
   through as frozen. The §7 summary moves SY-40 from provisional to frozen.

## Costs

None of the stores changes. A future adapter that implements `IngestStore` takes on the removal
record as a frozen obligation. A deployment that wants a pruned replica re-hydrated from its hub
needs the explicit clear-the-record operation that SY-40's body already names, not a change to
ingest.

## Door

One-way at `1.0.0`, two-way until then by a superseding record, like every freeze before 1.0.
