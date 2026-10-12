---
id: kb-decision-0093
title: happenstance-cloudflare's metadata ceiling is 512 KiB, carved from the documented row limit
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0093
reversibility: medium
phase: 17b
supersedes: null
superseded_by: null
summary: >-
  Proposed, not yet accepted by the owner. #92 builds ADR-0043's StoreLimit::MetadataLen and
  Fixture::MAX_METADATA_LEN, and happenstance-cloudflare is the adapter that states a metadata
  ceiling. The owner decided where it is carved from (wi-f97418): the documented 2 MB Durable
  Object row limit, not the 8,388,637 B wall the deployed object measured. This record decides the
  share. It reads "2 MB" as the smaller 2,000,000 bytes, so the answer holds under the 2 MiB
  reading too, and keeps the three published ceilings where they are. One event row at every
  ceiling at once carries 1,048,576 B of data, 262,145 B of tags blob (1 + 1,024 x 256), 255 B of
  event type and the metadata share. A 524,288 B (512 KiB) share makes that 1,835,264 B, which
  leaves 164,736 B for the fixed-width columns and SQLite's record header. 512 KiB is the largest
  power of two that fits. ADR-0043 sets no metadata floor, so no number is too small to be legal.
  The data ceiling is not re-derived: shrinking a published ceiling would refuse appends that
  succeed today. A host test, the_four_ceilings_fit_the_documented_row_limit, holds the sum to the
  limit. It implements wi-f97418. It is reopened by a change to the documented row limit, by a
  schema change that widens the event row, or by phase 13 choosing data plus metadata as one
  budget unit.
depends_on:
  - kb-decision-0043
  - kb-decision-wi-f97418
related:
  - kb-decision-0072
  - kb-decision-0079
source_paths:
  - crates/happenstance-cloudflare/src/event_store.rs
  - crates/happenstance-cloudflare/tests/support/mod.rs
  - crates/happenstance-cloudflare/tests/fixture_contract.rs
  - experiments/durable-object-limits/README.md
  - runbook/phases/17b-after-the-window.md
last_reviewed: 2026-10-12
---

# happenstance-cloudflare's metadata ceiling is 512 KiB, carved from the documented row limit

**Proposed.** Written by the #92 session and not yet accepted by the owner. It implements the
owner's decision `wi-f97418`, which chose the source of the carve. This record chooses the share.

## Context

ADR-0043 gave `Event::metadata` a refusal channel, `StoreLimit::MetadataLen`, with no guaranteed
capacity, and left whether any adapter states a ceiling to the adapters. `happenstance-cloudflare`
is the one that must. Its `event` row holds `metadata` beside `data`, and its `data` ceiling was
already set at half the documented row cap "to leave room for the metadata the contract does not
bound" (`experiments/durable-object-limits/README.md`, the derivation table). A metadata ceiling
turns that guess into a stated limit.

Phase 17's deployed Durable Object measured the row wall at 8,388,637 B. The owner decided
(`wi-f97418`) to carve from the documented 2 MB limit instead. A documented limit is one the
platform has promised to keep. A measured one moved once between two `workerd` releases already.

## Decision

`Ceilings::DECLARED.metadata_len`, the fixture's `MAX_METADATA_LEN` and the `workerd` harness
fixture's `MAX_METADATA_LEN` are all **524,288 bytes (512 KiB)**. A larger metadata value is
refused before any SQL as `AppendError::ExceedsStoreLimit { limit: StoreLimit::MetadataLen, .. }`.

The arithmetic, for one `event` row at every ceiling at once:

| Column | Bytes at the ceiling | Why |
| --- | --- | --- |
| `data` | 1,048,576 | `MAX_EVENT_DATA_LEN`, unchanged |
| `tags` | 262,145 | `UNIT tag UNIT … tag UNIT`: 1 + 1,024 × (255 + 1) at `MAX_TAGS_PER_EVENT` and `MAX_TAG_LEN` |
| `event_type` | 255 | `MAX_EVENT_TYPE_LEN` |
| `metadata` | 524,288 | this record |
| **sum** | **1,835,264** | |
| headroom | 164,736 | under 2,000,000; it holds `position`, `origin_store` (16), `origin_position`, `recorded_at` and SQLite's record header, which together take well under 256 |

"2 MB" is read as 2,000,000 bytes, the smaller of its two readings. Under the 2 MiB reading
(2,097,152) the headroom is 261,888 bytes, so the decision does not depend on which one Cloudflare
means.

## Alternatives

- **The measured 8,388,637 B wall.** The owner rejected it (`wi-f97418`).
- **256 KiB.** It also fits. It was rejected because no evidence asks for the extra 256 KiB of
  headroom, and the larger share is the less surprising one for a caller who writes trace context.
- **Shrinking `data` to make room.** `MAX_EVENT_DATA_LEN` is published in the crate's README. Lowering
  it would refuse appends that succeed today, which ADR-0072 does not allow in 17b. The sum fits
  without it.
- **No metadata ceiling.** It is false of a store whose row is bounded. It would also leave the
  rule's metadata branch skipped on the one adapter that can run it.

## Consequences

- `tests/fixture_contract.rs`'s `the_four_ceilings_fit_the_documented_row_limit` holds the sum on
  the host. Raising any ceiling without carving another down fails there, with no Durable Object
  needed.
- `append_reports_exceeded_store_limits` now runs a metadata branch against this adapter, on the
  shim and under `workerd`.
- The question of whether data and metadata share one budget stays open for phase 13. If it shares
  one, this ceiling is re-derived from that budget.

## Falsifier

This record is wrong if a deployed Durable Object refuses an event at the four ceilings, or if
Cloudflare's documented row limit falls below 1,835,520 bytes.
