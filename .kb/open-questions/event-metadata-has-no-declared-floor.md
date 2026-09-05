---
id: kb-open-question-event-metadata-no-floor-001
title: Event::metadata is a fourth opaque payload with no floor and no refusal path
kind: open_question
status: accepted
authority_tier: note
summary: >-
  happenstance-core ships four MIN_SUPPORTED_* constants - EVENT_DATA_LEN, TAGS_PER_EVENT,
  QUERY_ITEMS and EVENTS_PER_BATCH - and a StoreLimit enum with three variants, EventDataLen,
  TagsPerEvent and EventsPerBatch. Event::metadata appears in neither set. It is a fourth opaque
  payload carrying, among other things, the codec tag ADR-0021 put there, and a store that cannot
  accept a metadata blob has no conformant way to say so: VT-25 is [FROZEN] and forecloses
  AppendError::Store for a capacity refusal, so the only distinguishable refusal is
  ExceedsStoreLimit and there is no variant to name. happenstance-cloudflare is already carving its
  margin out of data to compensate, which is a workaround chosen by one adapter rather than a
  property of the contract. What is not decided is which of three answers is right: metadata gets
  its own floor and its own StoreLimit variant, metadata shares data's floor and the two are
  budgeted together, or metadata is declared deliberately unbounded and the specification says so.
  ADR-0015 is accepted and immutable and minted the four floors and the three variants, so the
  first two answers require a superseding decision atom and the third requires a clause; the
  2026-09-03 pre-publication review names the question and supplies none of them. Forced by phase
  12, where a public constant set and a frozen error variant become a promise, and independently
  by the first adapter that has to refuse a metadata blob.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0021
  - kb-decision-0003
  - kb-open-question-cf-40-ownership-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-core/src/limits.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-04
---

# Event::metadata is a fourth opaque payload with no floor and no refusal path

## What is true today

`crates/happenstance-core/src/limits.rs` declares four `MIN_SUPPORTED_*` constants —
`MIN_SUPPORTED_EVENT_DATA_LEN` (65,536), `MIN_SUPPORTED_TAGS_PER_EVENT` (64),
`MIN_SUPPORTED_QUERY_ITEMS` (128) and `MIN_SUPPORTED_EVENTS_PER_BATCH` (128) — and a `StoreLimit`
enum with exactly three variants: `EventDataLen`, `TagsPerEvent` and `EventsPerBatch`. `Event`
carries four fields a store must accept: `data`, `tags`, `event_type` and `metadata`. Three of the
four have a floor and a named refusal path; `metadata` has neither. ADR-0015 minted all four
constants and all three variants, and its own text is silent on `metadata` — the string does not
appear in that atom's body or in `limits.rs` at all.

`metadata` is not incidental payload. ADR-0021 put the codec tag there, so it is opaque bytes a
store must persist and hand back unmodified, exactly as `data` is, and there is no reason internal
to the contract why a store's capacity for it would be unbounded when its capacity for `data` is
not. VT-25 is `[FROZEN]` and forecloses `AppendError::Store` as the escape hatch for a capacity
refusal — a store that cannot say *which* limit it hit is not conformant — so the only
distinguishable refusal available is `ExceedsStoreLimit`, and `StoreLimit` has no variant naming
`metadata`. An adapter that wants to refuse an oversized metadata blob today has to either accept
it unconditionally, refuse it as something else the port does not sanction, or find room inside an
existing limit it does not actually govern. `happenstance-cloudflare` has already done the third
thing: it carves its own margin for metadata out of the budget it allots to `data`, a workaround one
adapter chose under Durable Object storage-value limits, not a property the port states or a
guarantee any other adapter can rely on providing the same way.

## What is not decided

Which of three answers is right. **Metadata gets its own floor and its own `StoreLimit` variant** —
symmetric with `data`, `tags` and batch size, but a fourth number to justify and a fourth arm every
`match` on `StoreLimit` must grow. **Metadata shares `data`'s floor**, budgeted together as one
capacity — cheaper to specify, but ties two payloads with different lifecycles (a codec tag rarely
changes size; event data does) to one ceiling. **Metadata is declared deliberately unbounded**, and
the specification says so in words rather than leaving the gap to be rediscovered — the cheapest of
the three, and the one a reader is likeliest to assume by default until told otherwise.

ADR-0015 is `accepted` and immutable, so the first two answers require a new decision atom carrying
`supersedes: [kb-decision-0015]`; the third requires only a specification clause, since it commits
the contract to nothing new. The 2026-09-03 pre-publication review names the gap and does not
resolve it, and this atom exists so the absence is not silently normalised into whichever behaviour
`happenstance-cloudflare` happens to ship.

## What forces it

Phase 12, where a public constant set and a `[FROZEN]` error variant stop being an internal detail
and become a promise `cargo-semver-checks` polices. Independently, the first adapter — Postgres,
Neon or Ladybug — that has to refuse an oversized metadata blob and finds no conformant way to say
why.
