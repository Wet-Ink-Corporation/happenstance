# #143 — SY-30, SY-31, CF-40: real envelopes, the runner half, the budget unit

## Goal

Make SY-30, SY-31 and CF-40 each freezable (a rule with a mutant it rejects) or
re-disposition them, without flipping any marker (freezes are #124's). Append the
proposed marker texts to `/mnt/project-files/notes/0.6.0-freeze-proposals.md`.

## What was already built at f1e4ff4d (established, not assumed)

- **SY-30.** Peer half: `push_envelope_preserves_group_boundaries` (#420) with
  `MergesAPage`/`DropsTheGuard` (+ the flattener and pager) in `tests/wrong_peers.rs`,
  mounted on the Durable Object (`served::sync_conformance`, workerd job) and on
  `NeonSyncPeer` (`peer_family.rs`, live-neon). Store half:
  `ingest_group_is_atomic_under_a_mid_group_fault`. Runner half: `tests/runner/groups.rs`,
  but only against `MemorySyncPeer`. **Unbuilt:** the runner pushing a real store's
  write groups through the two unlike real peers and the groups arriving whole.
- **SY-31.** `ack_never_confirms_a_non_durable_event` (#242) built; reserved id
  settled (ADR-0082, `ProjectionId::sync_watermark`). **Unbuilt:**
  `watermark_advances_transactionally_with_a_read_model` (marked "new" and `†`), and
  the runner seam: `runner.rs` says "Where the watermark lives is not decided here".
- **CF-40.** Rule `append_reports_exceeded_store_limits` with 16 mutants; every real
  adapter states constant ceilings (SQLite, Postgres, Neon: three; Cloudflare: five,
  ADR-0093). **Unbuilt:** the decision on `payload_len` as the budget unit.

## Constraints and non-goals

- No marker flips; no new dependency; no public API break (additions only); no CI edit.
- Spec/runbook `file:line` citations stay resolvable (`cargo xtask spec-trace`).
- Non-goal: persisting the resume token (an optimisation; ingest dedupes).

## Design

### CF-40 — decision: the field is the store's unit; `payload_len` is the port's

Options (H-16-shaped, recommendation built):
1. **(recommended, built)** Field-wise in the contract. Each stated ceiling bounds its
   own field and holds with the other at its own ceiling at once; `payload_len` is
   `PeerLimits::max_event_bytes`'s unit only, held to a store's two field ceilings by
   SY-18's `peer_declares_its_own_limits` (boundary carried once in data, once in
   metadata). Evidence: five adapters state field-wise constants, Cloudflare carves
   one 2 MB row into four that fit together (ADR-0093). Pinned by a new branch of
   `append_reports_exceeded_store_limits`: data at `MAX_EVENT_DATA_LEN` and metadata
   at `MAX_METADATA_LEN` in one event, accepted. Mutant `SharedPayloadBudgetStore`
   (states both, enforces `data + metadata <= MAX_EVENT_DATA_LEN`) passes today.
2. Shared unit in the contract (`EventDataLen` means data + metadata). Lost: changes
   `StoreLimit::EventDataLen`'s published meaning; an append that lands today would
   be refused for the same bytes split differently (ADR-0043's own reason).
3. A sixth shared ceiling (`MAX_EVENT_PAYLOAD_LEN`, a new `StoreLimit`). Lost: additive
   but nothing needs it; the one shared-budget store carved constants instead. Kept
   as the remedy if CF-40's falsifier fires.

### SY-31 — the runner half

`happenstance-sync` (additive, `runner` module, new file `runner/watermark.rs`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Watermark { peer: StoreId, confirmed: SequencePosition }
impl Watermark {
    pub const fn new(peer: StoreId, confirmed: SequencePosition) -> Self;
    pub const fn peer(&self) -> StoreId;
    pub const fn confirmed(&self) -> SequencePosition;
    pub fn id(&self) -> ProjectionId;                      // sync_watermark(peer)
    pub async fn commit<P: ProjectionStore>(&self, store: &P, batch: P::Batch)
        -> Result<(), CommitError<P::Error>>;              // Authority::Live
    pub async fn load<P: ProjectionStore>(store: &P, peer: StoreId)
        -> Result<Option<Self>, LoadWatermarkError<P::Error>>;
}
#[non_exhaustive] pub enum LoadWatermarkError<E> { Store(E), NotLive(Checkpoint) }
impl<R> Edge<R> {
    pub const fn watermark(&self) -> Option<Watermark>;    // responder + confirmed
    pub const fn restored(watermark: Watermark) -> Self;   // resuming + with_responder
}
```

Ownership: `commit` consumes the caller's batch (the port does); it borrows the store.
The batch carries the caller's read-model rows: the watermark rides in the same unit
(SY-31, PS-9: no generic write vocabulary, so the caller fills the batch).

`happenstance-sync-testkit`: new family `watermark_suite.rs`,
`watermark_rules::watermark_advances_transactionally_with_a_read_model<F: ProjectionFixture>`,
`for_each_watermark_rule!`, no mounting macro (convergence's precedent). The rule plays
the runner: a real `Edge` over `MemoryIngestStore` -> `StoreSyncPeer` (over `MemoryIngestFixture`; `MemorySyncPeer` names no responder), its watermark
committed in a batch with a probe row; read back through a fresh handle; restored edge
pushes nothing; a second peer's watermark independent; a stale watermark refused as
`CheckpointRegression` with its row absent.

Mutants (`tests/wrong_watermark_stores.rs`, wrapping `MemoryProjectionStore`):
`RefusesTheReservedPrefix`, `HandleLocalWatermark`, `OneWatermarkForEveryPeer`,
`UpsertedWatermark`; a faithful wrapper control.

Mounts: memory (sync-testkit tests), SQLite projection (gate), Postgres buffered +
live (live-postgres `projection` binary, `#[ignore]`d), Neon (`neon_projection`).

### SY-30 — the runner through the real peers

`harness/sync-round-trip`: `runner_keeps_write_groups` generic helper: SQLite near
store with appends of 2, 3, 2 and a guarded foreign group ingested, an `Edge` pushing
to a peer declaring `max_batch_events = 4`; the far side's pull equals the near
store's write groups, each whole with its guard. `postgres::` and `neon::` cases.
`harness/workerd/tests/sqlite_round_trip`: the same through the Durable Object over
HTTP. A non-live test pins that the check rejects the pre-#420 per-event cut.

## Test list (drive in this order)

1. [x] CF-40: `SharedPayloadBudgetStore` declared failing `append_reports_exceeded_store_limits` (red: runs, passes).
2. [x] CF-40: the at-once branch (green).
3. [x] SY-31: `Watermark` unit tests in happenstance-sync (red: compile error).
4. [x] SY-31: rule against `MemoryProjectionFixture` (red: compile error, then green).
5. [x] SY-31: each mutant fails exactly the rule; faithful wrapper passes.
6. [x] SY-31: SQLite / Postgres (local Docker) mounts.
7. [x] SY-30: harness tests (postgres local; neon and workerd CI-only), mutant check.
8. [x] spec/ledger/CHANGELOG/notes; spec-trace, lints, gate.

## Decisions (H-16)

- CF-40's unit: option 1 built; flagged for the owner in the hand-back.
- New public items in `happenstance-sync` (additive) and a new testkit family: additive,
  within the coordinator's brief.

## Risks

- Inserting into `suite.rs` shifts `suite.rs:7116-7119` (cited): repoint by referent.
- Mutation-coverage inventories (Declared rows, fixture list, MODEL_COVERAGE).

## Progress

- **Red 1 (CF-40).** With `SharedPayloadBudgetStore` declared failing
  `append_reports_exceeded_store_limits` and no at-once branch, the mutation-coverage
  binary failed: the declared mutant passed the rule. An 8192 data ceiling first broke
  VT-21's floor (`store_accepts_the_guaranteed_minimum_payload`); raised to 65,536, so
  the mutant earns exactly `limit: EventDataLen, len: 69632`. Green after the branch.
- **Red 2 (SY-31 seam).** `tests/runner/watermark.rs` written first: E0432 (no
  `runner::Watermark`) and E0599 (no `Edge::watermark` / `Edge::restored`). Green after
  `runner/watermark.rs`.
- **SY-31 rule.** Its red is the four mutants: each fails SY-31 with the message
  its mistake earns, pinned in `wrong_watermark_stores.rs`; the reference and the
  faithful wrapper pass, so the wrapping is not what fails.
- **SY-30.** No red is possible against the runner: it already pushes whole groups
  (#420). The check's power is pinned instead by
  `the_group_check_rejects_a_push_cut_by_event_count` (the pre-#420 cut fails it).
  Postgres case green against local Docker; Neon and workerd cases compile and are
  listed, CI-only.
- Mounts: SQLite green in the gate; Postgres buffered and live green locally
  (`--ignored --test-threads=1`); Neon CI-only.
- spec-trace, lints, `cargo check -D warnings` (default and no-default) and
  `cargo doc -D warnings --document-private-items` green.
