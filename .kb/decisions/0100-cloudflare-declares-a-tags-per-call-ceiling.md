---
id: kb-decision-0100
title: happenstance-cloudflare declares a ceiling of 524,288 tags per call, refused as StoreLimit::TagsPerBatch
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0100
reversibility: medium
phase: null
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-11-01 for #356 step 2, on the owner's decision recorded on the issue
  (wi-021b4f, step 2, first bullet) and the owner's decision card of the same day. A deployed
  Durable Object reset on one ingest call of 757 events of 1,024 tags (775,168 tag rows, the CPU
  limit, 2026-11-01) and of 1,016 such events (the storage-operation timeout, 2026-10-31), below
  the declared 1,024 events per call. happenstance-core gains StoreLimit::TagsPerBatch, additive
  under #[non_exhaustive], and MIN_SUPPORTED_TAGS_PER_BATCH = 8,192, the product VT-22 and VT-24
  already oblige, so the floor asks nothing new (VT-36 [PROVISIONAL]). happenstance-sync classifies
  the refusal Split, as it does EventsPerBatch. happenstance-testkit gains a defaulted
  Fixture::MAX_TAGS_PER_BATCH and a fifth branch of append_reports_exceeded_store_limits, which also
  checks that a stated ceiling is at least the per-event one and reachable under the
  events-per-batch one. happenstance-cloudflare declares 524,288 = 512 x 1,024, 1.48 times below the
  lower wall, and refuses a call over it before any SQL, on append and on ingest, counting
  compensation. This narrows published behaviour, as ADR-0093 did, and ships in the next 0.4.x
  with a CHANGELOG flag.
depends_on:
  - kb-decision-0043
  - kb-decision-0093
related:
  - kb-decision-0015
  - kb-decision-0026
  - kb-decision-0072
source_paths:
  - crates/happenstance-core/src/limits.rs
  - crates/happenstance-sync/src/ingest/error.rs
  - crates/happenstance-testkit/src/contract.rs
  - crates/happenstance-testkit/src/suite.rs
  - crates/happenstance-testkit/tests/mutation_coverage/tags_per_batch_ceilings.rs
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-cloudflare/tests/support/mod.rs
  - crates/happenstance-cloudflare/tests/fixture_contract.rs
  - harness/workerd/src/fixture.rs
  - harness/workerd/src/ingest/ceiling.rs
  - experiments/durable-object-limits/README.md
  - spec/SPECIFICATION.md
last_reviewed: 2026-11-01
---

# happenstance-cloudflare declares a ceiling of 524,288 tags per call, refused as StoreLimit::TagsPerBatch

**Proposed on 2026-11-01 for #356 step 2.** The owner decided the shape on the issue (`wi-021b4f`,
step 2, first bullet: "refused as Split", "additive to happenstance-cloudflare, the owner merges
it"); that decision has no atom in `.kb/` yet, so it is cited here in prose. The owner's decision
card of the same day chose the variant, the floor of 8,192 and the number 524,288. The session that
wrote this record built it. Acceptance is the owner's, and it flips `status` alone. No long-form
record exists; this atom is the record.

## Context

`happenstance-cloudflare` declares 1,024 events per call and 1,024 tags per event, and refused
nothing between them. A call at both ceilings is 1,048,576 tag rows inside one `transactionSync`.
The `workerd` harness's deployed leg measured what a real object does with that:

| Run | Largest call committed | What happened one step above |
| --- | --- | --- |
| 2026-10-31, run 41173513607 | 1,016 × 1,024 = 1,040,384 tag rows | `storage operation exceeded timeout which caused object to be reset` |
| 2026-11-01, run 41326218022 | 757 × 1,024 = 775,168 tag rows | `exceeded its CPU time limit and was reset` |

Both are in `experiments/durable-object-limits/results/`. A reset is not an answer from the
adapter, so the call fails with no class, and a sync runner cannot tell it from a transport fault.
The second run was taken after #365 stopped the harness owning a copy of every tag name, so its wall
is the runtime's. The wall moved 1.34 times between two runs and changed which limit it was; no
count says exactly where it is.

The contract had no way to say "this call carries too many tags in total": every `StoreLimit`
named one factor of one event, or the batch's length. Refusing through `AppendError::Store` is
forbidden by `[FROZEN]` VT-25.

## Decision

1. **The variant.** `StoreLimit::TagsPerBatch`, "tags per batch", in `happenstance-core`.
   Additive, because the enum is `#[non_exhaustive]`.
2. **The floor.** `MIN_SUPPORTED_TAGS_PER_BATCH = MIN_SUPPORTED_EVENTS_PER_BATCH ×
   MIN_SUPPORTED_TAGS_PER_EVENT = 8,192`, and `TagsPerBatch.guaranteed_minimum()` returns it. It is
   not a new obligation: VT-22 and VT-24 hold at once, so a store already accepts 128 events of 64
   tags. VT-36 `[PROVISIONAL]` states it.
3. **The class.** `happenstance-sync` classifies `IngestError::ExceedsStoreLimit { limit:
   TagsPerBatch, .. }` as `Split`, as it does `EventsPerBatch`. The wildcard arm would have said
   `Park`.
4. **The coherence rule.** A stated ceiling must be at least the store's per-event ceiling, so that
   halving a refused batch reaches a batch that lands, and reachable under its events-per-batch
   ceiling, or it refuses nothing. The testkit checks both before the branch writes.
5. **The number.** `happenstance-cloudflare` declares **524,288 tags per call** (512 × 1,024),
   1.48 times below the lower wall and 64 times the floor. `Ceilings::exceeded` refuses a call over
   it before any SQL, after the per-event checks, so a call over both is parked rather than split
   down to the event that was always going to be refused. `append` and `ingest` share the check, so
   an ingest counts every foreign event and every compensation.

## Alternatives

- **Lower `events_per_batch` to 512.** No API change. It also refuses 513 to 1,024 minimal events,
  which land in 394 ms on the deployed object. It refuses what does not fail.
- **A byte budget on tags.** The walls measured were CPU and storage time per row, not bytes; a
  byte budget would mis-state the thing that bounds the call.
- **No ceiling.** The object resets, and the caller is told nothing it can act on.
- **262,144 (2.96 times below).** It halves the CPU the strict deployed leg spends on this rule.
  Rejected for now because the strict leg re-tests 524,288 on every run; if it ever resets there,
  262,144 is the remedy, never a retry.

## Consequences

- **This narrows published behaviour.** A call of more than 524,288 tags that a host, or local
  `workerd`, accepted on `0.4.x` is refused. No signature changes, so `cargo-semver-checks` reports
  nothing. Under ADR-0072's borderline rule this is the owner's call, as ADR-0093's was; the
  owner's note on the issue calls it additive and says they merge it, and the CHANGELOG flags it.
- The strict deployed conformance leg now runs the at-ceiling batch: 512 × 1,024 tag rows in one
  request. The `max-tags` ingest bisection finds 512, and the call above it is the adapter's own
  refusal, `class=Split`.
- `PeerLimits` has no tag axis. SY-18 is `[DEFERRED]`, and this record does not add one.
- Classifying the two reset throws themselves as `Split` is step 2's second bullet and a separate
  change (#356).
- No other adapter declares the ceiling. Each one's `MAX_EVENTS_PER_BATCH × MAX_TAGS_PER_EVENT`
  already bounds the total, so `None` is a true statement for them.

## Falsifier

This record is wrong if a deployed Durable Object resets on a call of 512 events of 1,024 tags, or
refuses one under 524,288 tags for any reason other than another declared ceiling. VT-36's floor is
wrong if a store's per-call wall is not a function of the tag count, so that no count says where it
is.
