---
id: kb-decision-0109
title: A typed checkpoint is pinned to the event store it was built from, and PS-40 is minted provisional
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0109
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Proposed 2026-11-16 for #485, with the owner's answers 485-Q1..Q6 all A. The typed runners
  (run_projection, run_fan_out) take a Source, an event store paired with its StoreId, and
  record a 63-bit mark of that id under a sibling checkpoint, name#digest, beside PS-25's
  unchanged name@digest. A run whose source differs from the recorded mark stops with
  ProjectionError::SourceChanged before it reads or applies anything. A 0.7.x checkpoint with no
  pin is adopted by the store its first 0.8 run reads. Postgres, Neon and Cloudflare gain an
  inherent store_id so a view runs without the sync feature. PS-40 is minted [PROVISIONAL],
  falsified by callers handing the runner an identity that is not the store's. The frozen
  ProjectionStore port (ADR-0063) and PS-25's id and goldens (ADR-0102) do not change.
depends_on:
  - kb-decision-0063
  - kb-decision-0102
  - kb-decision-0108
related:
  - kb-decision-0030
  - kb-decision-0082
  - kb-decision-0028
source_paths:
  - crates/happenstance/src/source.rs
  - crates/happenstance/src/checkpoint.rs
  - crates/happenstance/src/checkpoint/pin.rs
  - crates/happenstance/src/runner.rs
  - crates/happenstance/src/fan_out.rs
  - crates/happenstance/tests/projection_runner.rs
  - crates/happenstance/tests/fan_out/pins.rs
  - examples/rebuilding-read-models/tests/rebuild_through_reset.rs
  - experiments/retention-reader/tests/it/r1_runner.rs
  - spec/SPECIFICATION.md
  - docs/upgrading-to-0-8.md
last_reviewed: 2026-11-16
---

# A typed checkpoint is pinned to the event store it was built from, and PS-40 is minted provisional

**Proposed on 2026-11-16 for #485.** It stays `proposed` until the owner accepts it. The owner's
answers to the plan's six questions (`.temper/plans/485-checkpoint-pinned-to-source.md`,
485-Q1..Q6) were all the recommended option, and this record writes them down. No long-form
record exists; this atom is the record.

## Context

ADR-0108 §1.1 froze ES-39 as the refusal and named three harms whose cures belong outside
`EventStore`. #485 is the third: `experiments/retention-reader`'s R1-4 showed that a typed
runner resumed against a different event store, with the same projection store, silently skips
that store's events at or below the old checkpoint and applies again the ones above it.
Positions are meaningful only within one store (ES-29, SY-19), and nothing in the checkpoint
said which store it came from.

Two facts shaped the cure. `ProjectionStore` is frozen (ADR-0063): a checkpoint is a
`(ProjectionId, position, authority)` triple and nothing else. And `EventStore` has no
identity accessor: the async `store_id` is `happenstance_sync::IngestStore`'s, behind each
adapter's `sync` feature, and `happenstance` depends on `happenstance-core` alone.

## Decision

1. **The caller supplies the identity (485-Q1 A).** Both runners take a `Source<'_, S>`, a
   borrowed store and its `StoreId`, in place of `&S`. `From<&MemoryEventStore>` is the door
   where the id cannot be wrong; elsewhere the docs say to read it from the store being passed,
   never from configuration. Adding `store_id` to `EventStore` (B) was rejected: it changes the
   contract crate and every adapter, and runs against ES-39's verdict. Binding `IngestStore`
   (C) was rejected: a new edge into replication, and a handle that can forge history.
2. **The identity is recorded in a sibling checkpoint (485-Q2 A).** The pin's id is
   `name#digest`, the main key's name and query digest with `#` for `@`, so the two namespaces
   are disjoint, a query change moves both, and the length cap is the main key's. Its position
   is `SourceMark::of(id)`: FNV-1a 64 over the id's bytes, shifted right one bit, `FIRST` for a
   zero. It is non-zero and at most `i64::MAX`, so it round-trips through SQLite's and Neon's
   saturating columns and Postgres's refusing one. It is committed with an empty batch (PS-21)
   **before** the stream opens, so no chunk is ever durable under an unrecorded source. Folding
   the id into the main key (B) was rejected: a switch then replays into rows never reset, which
   moves the bug rather than closing it, and it breaks PS-25's frozen goldens.
3. **A 0.7.x checkpoint is adopted (485-Q3 A).** A main key with no pin is pinned to the store
   its first 0.8 run reads, then resumed. It is wrong only if the store was switched in the same
   deploy as the upgrade, and the upgrade page says so. It makes one remedy serve every refusal:
   reset the pin, and the next run re-adopts.
4. **Postgres, Neon and Cloudflare gain an inherent `store_id` (485-Q4 A).** Additive, behind
   no feature, with `IngestStore::store_id` delegating. Without it a Postgres user must enable
   replication to run a view.
5. **PS-40 is minted `[PROVISIONAL]` (485-Q5 A),** in §4.7 after PS-39. Its falsifier is the
   one 485-Q1 A leaves open: callers handing the runner an identity that is not the store's.
   Its rule is `a_checkpoint_is_not_resumed_against_another_store`; its wrong implementations
   are W1 (no pin), W2 (a pin never compared), W3 (the id folded into the key), W4 (a mark
   above `i64::MAX`) and W5 (a pin written after the first chunk). The mark's goldens bind the
   encoding either way.
6. **The experiment's R1-4 now asserts the refusal (485-Q6 A).** Its doc keeps the recorded
   hazard and says #485 closed it.

## Consequences

- `run_projection` and `run_fan_out` break in `0.8.0`; `ProjectionError` gains `SourceChanged`
  and `RecordSource` on a `#[non_exhaustive]` enum. `docs/upgrading-to-0-8.md` is the recipe.
- A first run under 0.8 performs one more `begin` and `commit`, and every run one more
  `checkpoint` read. A store that counts calls sees them.
- After an identity re-mint (VT-6), every typed view over that store refuses. When the
  projection store was restored with the event store, resetting the pin is the remedy.
- **Reopened by** a deployment in which the caller-supplied identity is shown to drift from the
  store's, which would make 485-Q1 B, an identity the store itself reports, the cure.
