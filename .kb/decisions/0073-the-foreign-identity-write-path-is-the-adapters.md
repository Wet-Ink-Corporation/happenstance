---
id: kb-decision-0073
title: The write path that keeps a foreign identity is the adapter's row writer, and core grows nothing
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0073
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Settles ADR-0026's published-surface half, which the roadmap moved to phase 17: does
  happenstance-core need a write path that preserves a foreign EventId before IngestStore can be
  implemented? No. The write path is the adapter's own row writer, generalised to take a per-row
  origin, and nothing published changes. VT-10's falsifier, "a store adapter cannot implement
  IngestStore without duplicating append's write path", was run on its named instrument, SQLite, as
  a compiling spike: impl SendIngestStore for SqliteEventStore under #[cfg(test)], reaching the
  crate-private write_batch that append also calls, with one INSERT statement for both kinds of row.
  The ingest-only code is listable: the transaction frame (ingest_locked, ingest_groups), the
  watermark query, and the bound values of a foreign row. The falsifier did not fire, so VT-10 is
  frozen. Neon, a one-shot-HTTP store at the far end of the transport axis, builds a whole ingest
  batch as one statement composed from the same private builders its append uses, which is
  structural evidence that SY-14's bounded round trip stays reachable; it was not executed. Three
  things change in happenstance-sync, which is unpublished: the phase-2 placeholder identity types
  are deleted for core's (the u64 RecordedAt fired VT-9's restated falsifier by construction),
  ingest takes groups that carry their compensation, which SY-2 requires be atomic with the losing
  event, and holds is dropped as a duplicate of EventStore::contains_event_id. SY-2's prose example
  is given its reading: the batch is an IngestGroup, not an append.
depends_on:
  - kb-decision-0066
  - kb-decision-0072
related:
  - kb-decision-0014
  - kb-decision-0024
  - kb-decision-0061
source_paths:
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-sqlite/src/ingest_spike.rs
  - crates/happenstance-neon/src/event_store.rs
  - crates/happenstance-sync/src/ingest.rs
  - crates/happenstance-sync/src/identity.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-29
---

# The write path that keeps a foreign identity is the adapter's row writer, and core grows nothing

## The question

VT-10 puts the operation that accepts a caller-supplied `EventId` and `RecordedAt` on
`IngestStore` in `happenstance-sync`, not on `EventStore`. It left one question open, and phase 16
routed it to phase 17 because two of its three answers touch a published crate. Can an adapter
implement `IngestStore` without a new write path in `happenstance-core`?

- **(A) Core grows a write path**, as a required or provided method on `EventStore`.
- **(B) Each adapter owns one.**
- **(C) Neither**: ingest re-mints through `append` and keeps a side table mapping foreign
  identities to local ones.

VT-10's falsifier decides between (A) and (B), and it names its own instrument. The operation
belongs on `EventStore` "if a store adapter cannot implement `IngestStore` without duplicating
`append`'s write path; the SQLite adapter is the instrument and it settles the question the first
time it implements both". Nobody had run it.

## Decision

**(B). The write path is the adapter's, and it is the adapter's existing row writer.**
`happenstance-core` is unchanged and no published signature moves. ADR-0026's published half is
therefore empty, and phase 13's ADR-0026 cites this record for it.

### The spike, and why it is shaped as it is

`impl happenstance_sync::SendIngestStore for SqliteEventStore` lives in
`crates/happenstance-sqlite/src/ingest_spike.rs`, compiled only under `#[cfg(test)]`, with
`happenstance-sync` as a **path-only dev-dependency**. Three Rust facts decided that shape.

- **Coherence permits the impl in the adapter crate.** The orphan rule allows an impl in the crate
  that owns the trait or the crate that owns the type. Here the type is local, so the adapter may
  implement a foreign trait for it. That is VT-10's own argument, and it is what keeps `append`'s
  signature out of it.
- **Only the adapter crate can reach its private writer.** `write_batch` is private to
  `happenstance-sqlite`. An `experiments/` crate or a newtype wrapper could implement the trait but
  could not call `write_batch`, and would have to re-implement the insert, which is exactly the
  duplication the falsifier asks about. An in-crate module is the only placement that tests the
  claim.
- **Why `#[cfg(test)]` and not an optional `sync` feature.** `happenstance-sync` is
  `publish = false` and unclaimed on crates.io until phase 13. A real optional dependency on it
  would make `cargo publish -p happenstance-sqlite` fail at `0.4.0`. A dev-dependency with a `path`
  and no `version` is stripped from the packaged manifest. This is the precedent
  `happenstance-testkit` already set, and `tests/front_page.rs`'s
  `path_only_dev_dependencies_carry_no_version_requirement` now holds both. Phase 13 turns
  `cfg(test)` into `cfg(feature = "sync")`, which is additive.

### What is shared, and what is ingest-only

This list is what lets a reader check the verdict rather than take it.

**Shared** (`crates/happenstance-sqlite/src/event_store.rs`):

- `write_batch` (`:1274`) is the one row writer. It prepares **one** `INSERT` statement for every
  row: `… VALUES (?×7) ON CONFLICT (origin_store, origin_position) DO NOTHING`. A local row binds
  its origin `NULL`, which SQLite's `UNIQUE` treats as distinct, so the clause can never fire for
  an append's rows. A foreign row binds the origin its peer gave it, and one already held is a skip
  decided by the insert (VT-8), never by a lookup ahead of it.
- After the inserts the shared tail runs over the rows that went in: `write_tag_rows`,
  `bump_cardinality`, the `IS NULL`-marker stamp, and the decode of the last position.
- `check_identity` (`:1196`), newly factored out of `append_locked`, and `check_ceilings`.

**Ingest-only**, all under `#[cfg(test)]`, about 135 lines:

- the `Origin::Foreign` variant (`:1221`) and the values its arm binds;
- `ingest_locked` (`:795`), the counterpart of `append_locked`: `BEGIN IMMEDIATE`, the identity
  check, then per group the foreign rows, then the compensation as local rows **only if** that
  group inserted at least one row (SY-11), then commit. No condition is evaluated (SY-1);
- `ingest_groups` (`:861`), the counterpart of `append`'s ceilings-and-lock steps;
- `origin_watermark` (`:903`), one `GROUP BY` on the unique index's leading column.

**What changed for `append`, which is published.** It now builds a `Vec` of row descriptors and
prepares the shared statement, which carries the conflict clause. Its behaviour is unchanged. The
full conformance suite, the concurrency family and both query-plan assertions pass. It ships in
`0.4.0` as a changelog note, not a trace row, because nothing observable moved.

### The tests the spike carries

`ingest_spike.rs` holds eleven tests. Four planted wrong implementations were each watched turning
tests red before being reverted:
- compensation written unconditionally;
- a skipped row counted as inserted, which turned three tests red;
- the origin position converted by the saturating helper;
- the `IS NULL` marker deleted, which only the mixed-batch test below catches.

The tests:

- `ingest_preserves_foreign_event_id_and_recorded_at` includes a negative `RecordedAt`, which is
  VT-9's restated falsifier (ADR-0066), run early.
- `redelivery_is_a_skip_not_an_error`: no tag row and no cardinality bump for a skip.
- `ingested_events_land_above_the_local_head`, compared against assigned positions only.
- `compensation_is_atomic_with_the_losing_event_and_skipped_on_redelivery`, and
  `a_compensation_that_fails_in_the_transaction_takes_the_losing_event_with_it`. The second uses a
  trigger that fails the compensation insert, which is SY-2's atomicity under a real in-transaction
  fault.
- `contains_event_id_answers_for_a_foreign_id`, `ingest_ignores_append_conditions` and
  `watermark_is_max_origin_position_per_origin`.
- `a_local_append_after_ingest_does_not_restamp_foreign_rows` shows the behaviour, but it is not
  evidence for the marker: it passes with the marker deleted.
- `an_origin_position_past_i64_is_refused_not_skipped`. A saturating conversion would have written
  two distinct origins at `i64::MAX` and counted the second as a re-delivery.
- `pinned_vt6_breach_an_unheld_own_id_is_ingested_and_wedges_the_append_that_reaches_it` records
  current behaviour; it does not endorse it. See *Left to phase 13*.

`event_store::tests::a_mixed_batch_does_not_restamp_its_foreign_rows` calls `write_batch`
directly with a mixed batch. It is the only arrangement that fails if the `IS NULL` marker is
removed. `ingest_locked` never builds a mixed batch, so the marker is defence for a writer that
would.

### The spread check: Neon

The verdict above comes from one adapter of the serialise-the-writers shape, which is exactly what
CLAUDE.md's spread rule warns about. `happenstance-neon` sits at the far end of the transport axis:
no connection, no interactive transaction, one round trip per operation.

Neon's ingest statement, `ingest_statement` (`crates/happenstance-neon/src/event_store.rs:686`,
`#[cfg(test)]`), is built from the same private builders `insert_statement` uses:
- `insert_rows` (`:537`);
- `local_origin` (`:556`);
- `drawn_positions` (`:570`);
- `unpacked` (`:950`).

Append's SQL is byte-identical before and after that refactor.
`ingest_batch_is_one_statement_regardless_of_size` builds 1, 2, 64 and 1000 rows in mixed groups
and asserts one `SqlRequest::single` whose `ON CONFLICT` target and predicate match the migration's
partial unique index, which the test reads from the migration text rather than sharing it with the
builder.

**This is structural evidence only.** The statement was never sent to a server: no endpoint and no
Docker daemon were available when it was written. Partial-index inference, the order in which the
compensation's `nextval`s are drawn, and the gaps a skip leaves are phase 13's to confirm live. The
evidence shows two things. SY-14's bounded-round-trip ingest stays reachable. And (A) would have
foreclosed it: a core-level write path cannot express one adapter's single jsonb statement.

### Why not (A), and why not (C)

- **(A) loses on scope and on mechanics.** On scope, VT-10's existing text already says so: it puts
  `happenstance-core`'s publish schedule behind `happenstance-sync`'s design. On mechanics, a
  provided method has no default body worth having. Each adapter's ingest needs its own facts:
  xid8 visibility on Postgres (ADR-0024), the one-statement batch on Neon, and the `IS NULL`
  stamp on SQLite and the Durable Object. A required method breaks all four published adapters.
- **(C) breaks frozen clauses.** `crates/happenstance-testkit/tests/foreign_identity.rs` already
  shows that `append` re-mints. A read would then carry the local identity, and in an A–B–C
  topology the downstream peer deduplicates on the wrong one. That breaks SY-12 (identity travels
  beside the event), SY-4 and SY-24 (transitive convergence), and VT-8's store-level uniqueness.

## What changes in `happenstance-sync` (unpublished)

- **The placeholder `StoreId`, `EventId` and `RecordedAt` are deleted**, pulled forward from
  phase 13's list, because the spike's trait must speak the store's own types.
  - The placeholder `RecordedAt` was a `u64` and core's is an `i64`, so a pre-1970 recording could
    not travel through `ReplicatedEvent` unchanged. That is VT-9's restated falsifier, fired by
    construction.
  - No wire byte changes: `ReplicatedEvent` carries no serde derive, and WF-8's fixtures never
    encoded these types.
- **`IngestStore::ingest` takes `&[IngestGroup<'_>]`.** `IngestGroup` is `#[non_exhaustive]` and
  carries `events: &[ReplicatedEvent]` and `compensation: &[Event]`. The flat
  `&[ReplicatedEvent]` of the sketch lost group boundaries, and had nowhere to carry the
  compensation SY-2 requires be atomic with the losing event.
- **`holds` is dropped.** It duplicated `EventStore::contains_event_id`, which answers exactly that
  question, so a runner binds `S: EventStore + IngestStore`.
- **The `impl SendIngestStore for MemoryEventStore` and its four `todo!()` bodies are deleted**,
  rather than finished. Coherence let `happenstance-sync` add the trait to a core type but never
  reach inside it: `MemoryEventStore` mints every identity it writes. The in-memory oracle is phase
  13's, as a sync-owned store. `happenstance-sync` has no `todo!()` left in `src/`, so its crate-level
  `#![allow(clippy::todo)]` went with them.

## SY-2's prose example

SY-2 is `[FROZEN]` and says the losing event and its compensation are one batch, and gives the
example `append(&[losing, compensation], Some(&guard))`. Under VT-10 that call cannot carry the
losing event's foreign identity: `append` re-mints. This record gives the example its reading. The
batch SY-2 means is one `IngestGroup`, the losing event with its origin identity plus the
compensation, committed atomically by `IngestStore::ingest`. The normative sentence is unchanged,
and so is the marker.

## Left to phase 13, deliberately

- **An event that claims this store's own `StoreId` but is not held** is a VT-6 breach: the store
  was restored, or rewound. Today it is ingested at the claimed origin pair, and the later local
  append assigned that position then fails on the `UNIQUE` constraint, repeatedly, because the
  rollback returns `AUTOINCREMENT`'s counter. Refusing it would look like a refusal on local
  state, which SY-1 forbids. The pinned test records the behaviour so that phase 13's policy
  changes it on purpose.
- Ingest's own error type (the spike reuses `AppendError`), the watermark's query-plan assertion,
  and SY-14's live measurement.

## Falsifier

This record is wrong if either of the following happens:
- A second adapter that implements `IngestStore` for real (phase 13's Durable Object or
  Postgres peer) cannot route ingest through its own append writer, and needs a second insert path
  whose behaviour must be kept in step with `append`'s by hand.
- A phase-13 rule needs an ingest capability that only `happenstance-core` can provide.

Either one reopens (A), as a provided method, which is additive after 1.0.
