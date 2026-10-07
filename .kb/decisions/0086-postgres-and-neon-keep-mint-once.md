---
id: kb-decision-0086
title: Postgres and Neon keep mint-once, earned by a documented re-mint, and mint-per-open is declined
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0086
reversibility: low
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Answers the phase-17 half of kb-open-question-postgres-neon-store-id-no-restore-001. Neither
  happenstance-postgres nor happenstance-neon mints a StoreId per open, now or by default later.
  Both keep the mint-once row their first migration writes, and earn it through VT-6's second
  permission, a deployment documented to invoke a re-mint. This is the one-way part. Mint-per-open
  changes what EventId.store() reports on two published crates, so after the 0.4.0 window it is a
  major. Mint-per-open loses on cost and on coverage, not on the suite. A server store has no
  natural open: NeonEventStore is typically built per request on Workers (the deployment shape is
  inferred, not shown in tree), which would mint one incarnation per request. That is the harm
  VT-6's marker describes (its named candidate is a Durable Object, and its falsifier presumes
  mint-once unavailable, which here it is not), and every peer's Watermark keeps one entry per
  incarnation for ever. A pooled Postgres handle, or a stateless HTTP handle, also
  outlives a pg_restore or a Neon branch reset, so per-open minting does not close the gap it is
  meant to close. Neon has no entropy on wasm32, so the id would cost an extra round trip, and
  minting it inside the append statement is per-append minting, which the suite rejects.
  The earlier claim that mint-per-open fails reopened_store_does_not_reissue_an_event_id is stale:
  that rule was rewritten by ADR-0014 to admit it, and is corrected in adapter-shapes and the open
  question. The procedure is rustdoc and README prose with the raw statement: stop every writer,
  re-mint, and on Postgres restart every process holding a store before appends resume, because
  the OnceLock cache shared by a handle's clones would keep stamping the retired id. Its text is
  in the long form's Appendix A. It lands in the crate roots and READMEs, with the adapter-shapes
  mechanism rows, in the PR that accepts this record, not with the proposal. The additive remedies go to phase 13, or 17b if
  earlier: a remint_identity method on each store, Postgres stamping the identity inside the append
  transaction instead of caching it, Neon refusing an append when the meta row is absent, a
  statement builder for Neon's branch tooling, and restored_peer_does_not_reissue_identities with
  a negative control. One item is bound to this window and is put to the owner. Detection that
  refuses appends by default must land in 0.4.0 or be opt-in or report-only for ever. The record
  recommends the second, because no candidate fingerprint has been measured, and none is expected
  to change under a restore into the same database, so none can detect it.
  Supersedes nothing. ADR-0014 is cited, not amended.
depends_on:
  - kb-decision-0014
  - kb-decision-0066
related:
  - kb-decision-0014
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-0073
  - kb-decision-0024
  - kb-open-question-postgres-neon-store-id-no-restore-001
  - kb-open-question-remint-precondition-trust-only-001
source_paths:
  - spec/SPECIFICATION.md
  - references/adapter-shapes.md
  - crates/happenstance-postgres/migrations/0001_event_log.sql
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-postgres/src/migration.rs
  - crates/happenstance-neon/migrations/0001_neon_log.sql
  - crates/happenstance-neon/src/event_store.rs
  - crates/happenstance-neon/src/error.rs
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-sync/src/identity.rs
  - runbook/phases/17-breaking-window.md
  - runbook/phases/13-sync.md
  - references/adr/0086-postgres-and-neon-keep-mint-once.md
last_reviewed: 2026-10-07
---

# Postgres and Neon keep mint-once, earned by a documented re-mint, and mint-per-open is declined

The full record, with the code walk, the Rust semver reasoning, the alternatives and the test
sketches, is
[`references/adr/0086-postgres-and-neon-keep-mint-once.md`](../../references/adr/0086-postgres-and-neon-keep-mint-once.md).

## The question

VT-6 (`spec/SPECIFICATION.md:848-906`, `[PROVISIONAL]`, freeze-by-13 at
`.kb/decisions/0066-what-1-0-promises.md:161`) lets an adapter mint its `StoreId` once only if it
can detect a restore or clone, or if the deployment is documented to invoke a re-mint. An adapter
that can do neither MUST mint on every open (`:892-898`). Both server adapters mint once, in
migration 1 (`crates/happenstance-postgres/migrations/0001_event_log.sql:102-118`,
`crates/happenstance-neon/migrations/0001_neon_log.sql:126-141`). Neither does anything else, so
`0.3.2` sits outside the clause. Phase 13 closes the gap, but it opens after this window, and
mint-per-open is the one remedy that changes behaviour on a published crate. Phase 17 therefore
takes that choice (`runbook/phases/17-breaking-window.md:128-136`).

## Decision

1. **Neither adapter mints per open**, in `0.4.0` or as a later default. Both keep mint-once. A
   later move to mint-per-open is a post-1.0 major.
2. **Mint-once is earned by the documented-re-mint arm.** The procedure is written as rustdoc and
   README prose, using the raw statement. **None of it lands with this proposed record.** The
   crate-root and README procedure text, and the `references/adapter-shapes.md` mechanism rows
   VT-6 requires (`spec/SPECIFICATION.md:897-898`), are acceptance edits: they land in the PR that
   flips this atom to `accepted`, and the long form's Appendix A carries their exact text for that
   PR to paste. Until then the adapter-shapes row for Postgres and Neon still reads "undecided",
   which is true while this is proposed. The record recommends that acceptance land in `0.4.0`
   rather than 17b: on this record's reading, that puts `0.4.0` inside VT-6 without any code.
   That reading is the owner's to accept. VT-6's first
   mechanism is to "provide an explicit re-mint operation the deployment invokes"
   (`spec/SPECIFICATION.md:884-886`), and the open question described this arm as "a
   `remint_identity`-shaped operation, plus the procedure"
   (`.kb/open-questions/postgres-neon-store-id-has-no-restore-detection.md:115-118`). Whether a
   documented raw statement, plus a restart on Postgres, is that operation is a judgement, not a
   fact; if the owner reads it strictly, both crates stay outside VT-6 until phase 13's methods. Rustdoc prose is outside the semver promise
   (`.kb/decisions/0066-what-1-0-promises.md:265`). The procedure applies after `pg_restore`,
   PITR, promoting a replica that lost acknowledged writes, or creating, resetting or restoring a
   Neon branch. It is one procedure, the same in both files (long form §6):
   1. **Stop every writer.** No append reaches the copy until step 4.
   2. **Run the re-mint `UPDATE`** against the copy.
   3. **Postgres only: restart every process holding a `PostgresEventStore`**, so every cached id
      is dropped. The cache is an `Arc<OnceLock<StoreId>>` shared by a handle's clones
      (`crates/happenstance-postgres/src/event_store.rs:183`, filled at `:396-420`).
   4. **Resume appends.**

   The order matters on Postgres. A writer still running after the restore would stamp its
   cached, retired id onto rewound positions (the stale-cache race), so step 1 comes first. A
   process restarted before the `UPDATE` commits would read the retired row and cache it for its
   lifetime (the early-recache race), so step 3 follows step 2. Neon caches nothing: `local_origin`
   reads the meta row inside every append statement (`crates/happenstance-neon/src/event_store.rs:570-575`),
   so for Neon steps 1, 2 and 4 suffice.
3. **The additive remedies go to phase 13, or 17b if one is taken earlier.**
   - `remint_identity` on both stores, matching SQLite's name
     (`crates/happenstance-sqlite/src/event_store.rs:631`).
   - Postgres stamps the identity inside the append transaction and drops the cache. Removing a
     private field from a `#[non_exhaustive]` struct is invisible to dependents.
   - Neon refuses an append when the meta row is absent, instead of committing a row with a NULL
     origin (`crates/happenstance-neon/src/error.rs:145-156`).
   - A Neon statement builder, so branch tooling can run the re-mint without this crate.
   - `restored_peer_does_not_reissue_identities`, with a no-re-mint negative control.
4. **Detection that refuses appends by default is ruled out after 1.0.** If it is ever built, it
   is opt-in or report-only. This is the one item tied to this window. It is put to the owner as
   a separate call (below), because the alternative is to build it into `0.4.0`.

## Why mint-per-open lost

- **There is no natural open.** `NeonEventStore::new` is a `const fn`
  (`crates/happenstance-neon/src/event_store.rs:311`), and a Workers deployment typically builds
  one per request (inferred; no deployment in the tree shows it). Minting per open would then mint
  per request: the unbounded-watermark harm VT-6's marker describes
  (`spec/SPECIFICATION.md:856-861`), though the marker's own candidate is a Durable Object and its
  falsifier presumes mint-once unavailable. A `Watermark` keeps one entry per `StoreId`
  (`crates/happenstance-sync/src/identity.rs:81-95`). Postgres would mint per process start, per
  replica and per deploy.
- **It does not close the gap.** A pool, or a stateless HTTP handle, outlives a `pg_restore` or a
  Neon branch reset. The id the handle minted at open then stamps a rewound sequence. ADR-0014's
  "safe by construction" (`references/adr/0014-event-identity-and-recorded-time.md:229-233`)
  holds only where every storage discontinuity coincides with an open.
- **Neon has nothing to mint with.** `happenstance-core` mints nothing
  (`spec/SPECIFICATION.md:900-902`), and the Neon crate has no entropy on `wasm32`, so each handle
  would pay an extra `gen_random_uuid()` round trip. Minting inside the append statement instead
  is per-append minting, which `append_stamps_a_local_event_id` rejects
  (`crates/happenstance-testkit/src/suite.rs:2339`).
- **It splits one log across many origins.** That loses the per-origin continuity VT-5's argument
  wants (`spec/SPECIFICATION.md:885-887`).
- **Making it const-safe would be a major change.** On Neon, keeping `new` a `const fn` needs a
  per-clone `OnceLock`, and then each clone mints its own id. An `Arc` would make `new` non-`const`,
  which is a signature break.

**It does not fail the suite.** `reopened_store_does_not_reissue_an_event_id` was rewritten under
ADR-0014 precisely so that it admits mint-per-open
(`crates/happenstance-testkit/src/suite.rs:2594-2601`, `:2625-2693`). Two texts said it
fails outright, `references/adapter-shapes.md:387-393` and the open question's `:124-127`. Both
are corrected in the change that lands this record.

## Why detection is not the arm taken

None of the candidates has been measured, and none is expected to change under, and so catch, a
same-database `pg_restore` of an older dump, which is the main hazard. The candidates are `system_identifier`,
the database OID, and Neon's timeline id. Turning on default refusal after `0.4.0` would also be a
behaviour change: a Neon branch that appends on `0.3.x` would start erroring.

## Consequences

- Safety rests on a procedure, which is the cost VT-6 names for this arm (`spec/SPECIFICATION.md:886-887`).
- Neon preview branches are the weakest point, because the platform's control plane creates
  them. The statement builder in item 3 is the mitigation.
- Until phase 13, a Postgres re-mint requires a restart.
- The VT-6 clause text is unchanged.

## Falsifiers

- A recorded incident or measurement in which a deployment that followed the documented procedure
  still re-issued a pair. That reopens this record, and a reversal is a major.
- Neon's control plane is shown to offer no point at which a re-mint can run before a branch's
  first append.
- Automatic failover with asynchronous replication is shown to rewind `event_position_seq` past
  positions a peer has already seen. That discontinuity has no operator in the loop, and it has
  not been measured.

## Not decided here

- Cloudflare's earning arm, given that Durable Object point-in-time recovery is unexamined. It is
  proposed as its own open question.
- Which report-only fingerprint, if any, is built.
- The method's exact placement: inherent, a `migration` function, or both.
- CTE stamping versus a `SELECT` before the insert.
