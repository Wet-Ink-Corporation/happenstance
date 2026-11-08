---
id: kb-decision-0105
title: A store records each write's group on its rows, and a peer reads and pushes in those groups
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0105
reversibility: medium
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Proposed for #420, on the owner's approval of the plan's option A on 2026-11-08. Before it,
  nothing in a store recorded which rows one write put in, so the runner cut a push at an event
  count and could split a losing event from the compensation it was ingested with, and a
  StoreSyncPeer served a page as one group with no guard, merging what it had ingested. This
  record adds a write group to every row: the lowest position the write put in, the same on every
  row of it, and the guard an ingested group arrived under on that first row. A new trait in
  happenstance-sync, WriteGroupStore, reads the log back in whole groups; the runner and the store
  peer read through it. SQLite's migration 2 adds the two columns and raises its marker to 2.
  SY-30's rule is written and a new provisional clause, SY-39, states the store's half. Postgres,
  Neon and the Durable Object follow in #420's second and third parts. Additive at the port; a
  schema change in every adapter that implements it.
depends_on:
  - kb-decision-0097
  - kb-decision-0098
related:
  - kb-decision-0024
  - kb-decision-0027
source_paths:
  - crates/happenstance-sync/src/write_group.rs
  - crates/happenstance-sync/src/runner.rs
  - crates/happenstance-sync/src/store_peer.rs
  - crates/happenstance-sync-testkit/src/write_group_suite.rs
  - crates/happenstance-sqlite/src/event_store/write_group.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-08
---

# A store records each write's group on its rows, and a peer reads and pushes in those groups

## Context

SY-30 says the push envelope carries groups, each under its own guard, and the wire has carried
them since ADR-0097. What fills the envelope had nowhere to get them from. `EventStore::read`
returns rows; no adapter recorded which rows one `append` or one ingested group wrote. Two
defects followed, both reported on #420:

* The runner gathered its push by event count up to the far side's batch limit, one group per
  event or one group per page. A three-event write with a two-event limit went out as two
  pushes, and a crash between them left the far side holding a losing event without its
  compensation, which SY-2 forbids.
* `StoreSyncPeer` served a pull as one group with no guard, whatever it had ingested. A relay
  therefore merged independently guarded groups into one unguarded group, and the guard, which
  SY-6 makes evidence that must travel, stopped at the first hop.

`a_page_travels_as_one_group_without_a_guard` pinned the second as the behaviour, and the
runner's module documentation called the first a known gap.

## Options

* **A, a write group per row.** Every row carries the lowest position its write put in, and the
  write's first row carries the guard. A new trait reads groups back. Additive at the port, a
  migration in every adapter.
* **B, a group table.** One row per write, holding its range and guard. Two writes per append,
  and a range is wrong on a store whose writers interleave.
* **C, regroup at read time** from the origin identities. Cannot recover a local append's
  boundary, which carries no origin, and cannot recover a guard at all.

The owner approved A on 2026-11-08.

## Decision

1. `happenstance-sync` gains `WriteGroupStore` and its `Send` flavour. `read_groups(after,
   max_events)` returns whole groups above `after`, in position order, at most `max_events`
   events except that the first group is whole however large; `Waiting::Nothing` only when the
   read reached the end. The error carries `Classify`.
2. A group is named by its **lowest** position, which is also the cursor. On a store whose
   writers interleave, a later write can commit with a lower position than an earlier write's
   last row; resuming above the lowest position cannot skip a group not yet visible.
3. The guard is stored as one format byte and postcard, `encode_guard` and `decode_guard` in
   `happenstance-sync`, so every adapter stores the same bytes.
4. A row written before an adapter's migration reads as a group of its own. No backfill.
5. The runner pushes whole groups up to the far side's limits and never splits one; a first group
   that does not fit is offered alone, so the far side's refusal states its class (SY-36).
   `StoreSyncPeer` pulls whole groups up to its own budget; a head group larger than the budget
   stalls the pull on one token, the port's existing rule for an oversized event, extended.
6. SQLite: migration 2 adds `write_group INTEGER` and `group_guard BLOB` and raises the marker to
   2, so an older build refuses the file.
7. SY-30's rule `push_envelope_preserves_group_boundaries` is written, and SY-39 `[PROVISIONAL]`
   states the store's half with six rules in a new family, `write_group_store_conformance!`.

## Costs

* A schema change in four published adapters, one per part of #420.
* An older SQLite build refuses a file a newer one opened. The alternative left the marker at 1;
  the owner's call on it is open, and the change to it is one function.
* A head group larger than a store peer's page budget stalls the pull, documented, not loud.

## What would make this wrong

A store whose writers can commit a group whose lowest position is below a group a reader has
already passed, so that resuming above a name skips a write. That is SY-39's falsifier.
