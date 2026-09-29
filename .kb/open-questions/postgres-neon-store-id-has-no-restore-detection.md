---
id: kb-open-question-postgres-neon-store-id-no-restore-001
title: happenstance-postgres and happenstance-neon mint their StoreId once, and take neither branch VT-6 requires of mint-once
kind: open_question
status: accepted
authority_tier: note
summary: >-
  VT-6 lets an adapter mint its StoreId once and keep it only if it can detect that its state was
  restored or cloned, or if the deployment is documented to invoke a re-mint. An adapter that can
  do neither MUST mint a fresh incarnation on every open. happenstance-postgres and
  happenstance-neon both mint once, in their first migration: one store_meta row, inserted with ON
  CONFLICT DO NOTHING and never updated by the adapter. Neither detects a restore, offers a re-mint
  or documents a procedure. references/adapter-shapes.md still records both as "undecided". So two
  published adapters sit outside the clause's permission. On these platforms a clone is an ordinary
  operation. A pg_restore of an older backup brings back the StoreId row and rewinds the position
  sequence, so the restored store re-issues (StoreId, SequencePosition) pairs it has already issued
  for different events. A Neon branch is a copy-on-write clone by construction, so a branch that
  accepts appends mints EventIds that collide with its parent's. Both are the failure VT-6's
  Rejects: paragraph calls the one with no error path. What is not decided is which branch each
  adapter takes, and whether Postgres's per-handle identity cache needs the append-time re-read
  happenstance-sqlite added in f719b2a. Found while closing
  kb-open-question-remint-precondition-trust-only-001. Owned by phase 13, because sync's dedup
  trusts the pair and restored_peer_does_not_reissue_identities would be red on a Postgres-backed
  peer. A re-mint operation or a documented procedure is additive. Switching to mint-per-open is a
  behaviour change and would be phase 17's.
depends_on: []
related:
  - kb-decision-0014
  - kb-decision-0024
  - kb-decision-0066
  - kb-open-question-remint-precondition-trust-only-001
source_paths:
  - spec/SPECIFICATION.md
  - references/adapter-shapes.md
  - crates/happenstance-postgres/migrations/0001_event_log.sql
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-neon/migrations/0001_neon_log.sql
  - crates/happenstance-neon/src/event_store.rs
  - crates/happenstance-sqlite/src/event_store.rs
  - runbook/phases/13-sync.md
last_reviewed: 2026-09-29
---

# happenstance-postgres and happenstance-neon mint their StoreId once, and take neither branch VT-6 requires of mint-once

## What is true today

VT-6 (`spec/SPECIFICATION.md:838-896`, `[PROVISIONAL]`) requires that a store never issue an
`EventId` whose `(StoreId, SequencePosition)` pair it has already issued for a different event
(`:840-844`). It permits two mechanisms, and it does not leave the choice between them free
(`:882-888`):

> An adapter MAY take the first mechanism **only if** it can detect that its state was restored or
> cloned, **or** the deployment is documented to invoke the re-mint. An adapter that can do neither
> **MUST** mint a fresh incarnation on every open. Either way it MUST record which mechanism it
> chose in `references/adapter-shapes.md`.

**Both adapters take the first mechanism.** `happenstance-postgres` mints in migration 1
(`crates/happenstance-postgres/migrations/0001_event_log.sql:102-118`). That is a `store_meta`
table and one row, `('store_id', uuid_send(gen_random_uuid()))`, inserted `ON CONFLICT DO NOTHING`,
and the comment says: *"One row, inserted once and never updated by the adapter."* `happenstance-neon`
has the same shape, with the table name placed by a template
(`crates/happenstance-neon/migrations/0001_neon_log.sql:126-141`). Both are mint-once, and neither
earns it:

- **No detection.** Nothing records a fingerprint of the server or database the row was minted on,
  so there is nothing to compare against when a copy is opened.
- **No re-mint and no procedure.** Neither crate has an equivalent of
  `SqliteEventStore::remint_identity`. A search of both crates' sources, their READMEs and `docs/`
  for *remint*, *re-mint*, *restore*, *pg_dump* and *pg_restore* finds nothing about identity.
- **No record.** The VT-6 table in `references/adapter-shapes.md` (§7) still lists
  `happenstance-cloudflare`, `happenstance-postgres`, `happenstance-neon` and
  `happenstance-ladybug` together as **undecided**, "Skeletons. Each owes this row before it can
  claim to have passed the suite" (`:385`). Three of those four are now published, conformant
  adapters. Cloudflare's answer does exist in code — `store_id_is_not_reminted_per_handle`
  (`crates/happenstance-cloudflare/src/event_store.rs:2096`), because Durable Object storage
  outlives the isolate — but it is not in the table either.

**How the identity is read differs, and the difference matters to any re-mint.**
`PostgresEventStore` reads the row once per handle and caches it in an `Arc<OnceLock<StoreId>>`
(`crates/happenstance-postgres/src/event_store.rs:175-183`, read at `:393-416`). A handle that was
open across a re-mint would keep stamping the retired identity. That is the stale-handle bug
`happenstance-sqlite` closed in `f719b2a` by re-reading the identity inside the append
(`crates/happenstance-sqlite/src/event_store.rs:680-743`). `NeonEventStore` has no cache. Its
append reads `store_id` inside the same `INSERT … SELECT` statement
(`crates/happenstance-neon/src/event_store.rs:515`), so a re-mint would be seen by the next append.

## Why a clone is not an edge case here

On a file-backed store, a copy is something an operator does deliberately, and
`references/adapter-shapes.md:395-401` tells them what to do afterwards. On these two platforms a
copy is routine:

- **`pg_dump` / `pg_restore`.** A logical dump carries `store_meta`'s row, and it carries the
  position sequence's value (`CREATE SEQUENCE … event_position_seq` at `0001_event_log.sql:73`).
  Restoring an older backup over a store that has since moved on rewinds the sequence and keeps the
  `StoreId`. The restored store then re-issues positions it has already issued, for different
  events, under the same identity, which is exactly the pair VT-6 forbids. Restoring into a second
  database while the original keeps running gives two stores with one identity, both minting.
- **A Neon branch.** A branch is a copy-on-write clone of the database at a point in time, the
  platform's standard way to make a preview or test environment. A branch that accepts appends
  mints `EventId`s that collide with the parent's from its first write.

VT-6's `Rejects:` paragraph describes the consequence. A peer's dedup treats the colliding events as
already seen, and real facts are silently dropped, with no error path and no observable symptom.
Nothing catches it today. `reopened_store_does_not_reissue_an_event_id`
(`crates/happenstance-testkit/src/suite.rs:2625`) tests a reopen, not a restore, and
`restored_peer_does_not_reissue_identities` is named by VT-6 for `happenstance-sync-testkit`, which
does not exist yet.

## What is not decided

Which of VT-6's arms each adapter takes, and at what cost:

- **Documented re-mint.** A `remint_identity`-shaped operation, plus the procedure written where an
  operator will find it (the crate README, and the adapter-shapes row). This is the cheapest arm and
  the one `happenstance-sqlite` took. For Postgres it probably brings the append-time re-read with
  it, because of the per-handle cache above.
- **Detection.** Persist a fingerprint of where the row was minted next to the row, and compare it on
  first use. Plausible candidates exist — the cluster's `system_identifier`, the database's OID, a
  Neon branch or endpoint identifier — but none has been measured against the operations above. A
  `pg_restore` into the *same* database preserves every one of them, so detection may cover the
  branch case and miss the restore case.
- **Mint-per-open.** This satisfies the clause by construction, and it is the one arm that changes
  behaviour on a published crate. A server store has no natural "open", and per pool or per handle
  would split one store's history into a great many origins. `references/adapter-shapes.md:387-393`
  records that mint-per-open fails `reopened_store_does_not_reissue_an_event_id` outright.

## What forces it

VT-6 already binds both crates, so the violation exists in `0.3.2` today. It has no consumer-visible
symptom until something dedups by `EventId`. **Phase 13** is that something. Sync's watermark trusts
the pair, VT-6 is dispositioned *freeze-by-13* in phase 16's table, and phase 13's exit criteria
require `restored_peer_does_not_reissue_identities` to be green against a Postgres-backed peer and
both adapters to document what they do on a restore (`runbook/phases/13-sync.md:63-77`, `:191-194`).
A re-mint operation is an additive inherent method, and a documented procedure is additive too, so
both fit phase 13. Switching either adapter to mint-per-open is a behaviour change, and it would
belong to phase 17's breaking window.

## Ordered sub-questions

1. Per adapter: documented re-mint, detection, or both? Does detection cover `pg_restore` into the
   same database at all, and if not, is it worth building for the branch case alone?
2. If Postgres gains a re-mint, does its `OnceLock` cache give way to a read inside the append
   transaction, as `f719b2a` did for SQLite, and what does that cost on a store that does not
   serialise its writers?
3. What does the procedure say for a Neon branch, which is created by the platform's control plane
   rather than by anyone running this crate? Is "re-mint every branch before its first append"
   something an operator can actually be relied on to do?
4. How does `restored_peer_does_not_reissue_identities` simulate a restore against a server fixture:
   a cloned schema, a second database, a new `Capability`?
5. The adapter-shapes row. VT-6's second MUST is owed by `happenstance-cloudflare`,
   `happenstance-postgres` and `happenstance-neon` regardless of how 1 is answered, and it is
   additive documentation that can land before any code does.

## Owner

Phase 13 (`runbook/phases/13-sync.md:63-77`).
