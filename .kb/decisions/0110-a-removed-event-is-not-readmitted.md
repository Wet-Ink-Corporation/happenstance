---
id: kb-decision-0110
title: A store that removed an event records it, and ingest skips it, and SY-40 is minted provisional
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0110
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-17 for #483 and #484, on the owner's answers D-1..D-10 all A. A store that
  implements ingest keeps a durable record of the EventId of every event removed from it by any
  means that keeps its position allocator (a raw DELETE, an operator's prune), written by a
  trigger on the events table, and ingest treats a recorded identity as held: skipped, counted
  in Ingested::skipped, no compensation for a group of nothing else, and never refused under
  SY-38. A restore rolls the record back with the allocator, so SY-38 still catches it. It
  AMENDS the frozen SY-1 (a second exception) and SY-38 (its title and its "restored or rewound"
  sentence), points SY-11 and ES-39 at the new clause, and mints SY-40 [PROVISIONAL]. No port
  trait changes, no new IngestError arm, and the record is never reported through a port value,
  so ES-39's refusal stands.
depends_on:
  - kb-decision-0098
  - kb-decision-0107
  - kb-decision-0108
  - kb-decision-0028
related:
  - kb-decision-0105
  - kb-decision-0073
source_paths:
  - crates/happenstance-sync/src/memory_ingest.rs
  - crates/happenstance-sync/src/memory_ingest/removal.rs
  - crates/happenstance-sync/src/ingest.rs
  - crates/happenstance-sync/src/ingest/error.rs
  - crates/happenstance-sync/tests/runner/pruned.rs
  - crates/happenstance-sync-testkit/src/ingest_fixture.rs
  - crates/happenstance-sync-testkit/src/ingest_suite/ingest_rules/removal.rs
  - crates/happenstance-sync-testkit/tests/wrong_ingest_stores.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-17
---

# A store that removed an event records it, and ingest skips it, and SY-40 is minted provisional

**Proposed on 2026-11-17 for #483 and #484.** The owner answered the plan's ten questions
(`.temper/plans/483-484-pruned-identity-tombstones.md`, D-1..D-10) "all as recommended" on
2026-11-17, and this record writes them down. **Its text still needs the owner's yes**, as
ADR-0098's did, before the edits under "What acceptance changes" bind. No long-form record
exists; this atom is the record.

**This record amends two `[FROZEN]` clauses, SY-1 and SY-38.** Skipping a removed event breaks
SY-1's words, which require a store to append every event "it does not already hold", as
ADR-0098 said of its own exception. It does not read them narrowly.

## Context

`experiments/retention-reader/` (#156, PR #482) ran the ingest path into a store pruned by a raw
SQLite `DELETE` outside the port. Its findings F1 and F2:

- **#483**: `a_restored_edge_resurrects_pruned_foreign_events`. A spoke prunes two foreign events
  it has projected; `Edge::restored` pulls from the start of the hub's log; ingest's dedupe is the
  origin pair's uniqueness (VT-8), which holds only among rows still present, so both land again
  at the tail and the projection applies them twice. `runner/watermark.rs`'s "a re-read is
  harmless" was false.
- **#484**: `a_restored_edge_stops_on_a_pruned_own_event` and
  `a_fresh_edge_pushing_into_a_pruned_hub_stops`. A pruned own event, offered back, is refused
  as SY-38's `UnheldOwnIdentity`, class `Stop`, on the spoke side and the hub side. The store was
  neither restored nor rewound, and the edge is wedged.

ES-39's provided method cannot cure either (C4): the dedupe and the SY-38 check live inside
`IngestStore::ingest`, below any port value a caller could read.

## The clauses this amends, as they stand

- **SY-1:** "An ingesting peer MUST append every event it receives that it does not already
  hold", with SY-38 as "the single exception".
- **SY-38:** "An ingest that meets an event claiming the receiver's own `StoreId`, which the
  receiver did not hold when the ingest began, MUST refuse the whole call", whose body says
  such a receiver "was restored or rewound under an identity VT-6 required it to re-mint".
- **SY-11** (a pointer only): "Re-delivery of an event the receiver already holds MUST be a no-op".

## Options

1. **A removal record written by a trigger** on the events table (chosen, D-1 A). It fires for
   every client that deletes, the `sqlite3` CLI and `psql` included, and adds no public method.
2. An inherent `prune` method that writes the record. It fixes neither reproduction as filed,
   both of which are a raw `DELETE`.
3. An ever-held identity ledger. Race-free, but a second index write on every append in four
   adapters and a backfill of the whole log.
4. The allocator's high-water mark for own events (D-2 B). Retroactive, but covers no foreign
   event, so a second mechanism beside the record, and it re-plumbs SY-38 in four adapters.
5. A persisted pull token. `Edge::restored` exists for the case where none survived, and it does
   nothing for the push side.
6. A completeness report on `EventStore`. Refused by ES-39 (ADR-0108).
7. Soft delete. Forbidden by ES-38 and ES-40 (`RememberingConditionFixture`).

## Decision

1. Every store that implements ingest keeps a durable record of the identities removed from it
   by any means that keeps its position allocator.
2. Ingest treats a recorded identity as held: skipped, counted in `Ingested::skipped` (D-4 A),
   no compensation for a group of nothing else (SY-11), and never refused under SY-38.
3. The record is consulted under the write lock, or under a lock that excludes every removal
   (D-8 A: `LOCK TABLE ... IN SHARE MODE` on Postgres and Neon). Held events still dedupe on
   the origin pair's uniqueness; VT-8 is not reopened.
4. A restore rolls the record back with the allocator.
5. The record is never exposed through a port value. ES-39 stands.
6. The record is kept for ever (D-5 A). `TRUNCATE` voids it, and that is documented, not refused
   (D-9 A). Schema bumps for SQLite and Postgres (D-3 A) ride in `0.8.0`.

## Why a removal is not a restore

VT-6's hazard is a store that issues an `EventId` a second time, which needs a rolled-back
allocator. `DELETE` never rolls the allocator back: SQLite's and the Durable Object's
`AUTOINCREMENT` keep `sqlite_sequence`, and Postgres's and Neon's sequences keep their value.
ES-38 is the clause that guarantees it. So an identity a removal took can never be minted again,
and a copy offered back is the very event the store held, which makes skipping it exact. A
restore rolls back the record with the allocator and the log, so own events minted after the
backup are absent from the restored record and SY-38 still refuses them.

## What acceptance changes

- **SY-1** gains: "SY-40 is the second: an event whose identity the store has recorded as
  removed is skipped as held."
- **SY-38**'s title becomes "...which the receiver neither held when the ingest began nor has
  recorded as removed (SY-40)...", and its "restored or rewound" sentence gains "A removal keeps
  the allocator and is recorded, so it is a skip, not this refusal."
- **SY-11** gains a pointer to SY-40.
- **ES-39**'s note marks #483 and #484 resolved by this record.
- **SY-40** is new: "A store that implements ingest MUST durably record the `EventId` of every
  event removed from it by any means that leaves its position allocator where it was, and ingest
  MUST treat an event whose identity it has recorded as removed as one it holds (SY-11): skipped,
  never appended, never refused under SY-38. The record MUST reach every handle and survive
  reopening, and a restore that rolls the allocator back MUST roll the record back with it."
  Rule: `a_removed_event_is_not_readmitted`. Cases: E2E-46, E2E-42.

## Costs

One table per store, growing by one row per removal, about 30 bytes and an index entry (H-13).
A `SHARE`/`ROW EXCLUSIVE` exclusion between ingest and removal on Postgres and Neon. Schema bumps
for SQLite and Postgres. Removals made before the migration are not recorded; the manual remedy,
an `INSERT` into the record, is published with the upgrade notes.

## Consequences

- `MemoryIngestStore::remove` and `IngestFixture::{REMOVAL, remove_outside_port}` with
  `UNSTATED_INGEST_REMOVAL` are new and additive; the conformance rule is a minor for
  `happenstance-sync-testkit`, whose consumers pin it exactly.
- Each adapter's mounted ingest family reports the rule declined until its own change grants
  `REMOVAL`: SQLite, then the Durable Object, then Postgres and Neon.
- `experiments/retention-reader` stays a dated record (D-7 A).

## What would make this wrong

A deployment in which a removed event must come back on re-delivery, such as re-hydrating a
pruned replica from its hub. The remedy then is an explicit operation that clears the record for
a range, not a change to ingest. Also: an adapter that cannot keep its allocator across a removal.

## Freeze condition

SY-40 stays `[PROVISIONAL]` until `a_removed_event_is_not_readmitted` is green on SQLite, the
Durable Object (host and `workerd`), Postgres and Neon, so the freeze rests on the far end of
the writer-serialisation axis and not on SQLite alone. Then `[FROZEN]` by this record.
