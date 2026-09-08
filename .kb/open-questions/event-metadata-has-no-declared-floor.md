---
id: kb-open-question-event-metadata-no-floor-001
title: Event::metadata is a fourth opaque payload with no floor and no refusal path
kind: open_question
status: superseded
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
  by the first adapter that has to refuse a metadata blob. Resolved 2026-09-07 by ADR-0043
  (kb-decision-0043) with a fourth answer none of the three named: metadata gets a StoreLimit
  variant, MetadataLen, and a Fixture::MAX_METADATA_LEN, both with guaranteed_minimum() == 0 - a
  refusal channel with no guaranteed capacity. What decided it is evidence this atom did not have
  and whose absence it assumed: happenstance-sync's ReplicatedEvent::payload_len already sums data
  and metadata into one budget checked against PeerLimits::max_event_bytes at 65,536, in shipped
  code, which makes any non-zero metadata floor incommensurable with PeerLimits::minimum() and
  removes the sole discriminator for the first answer. The third answer - metadata declared
  deliberately unbounded - is rejected, because it leaves the frozen VT-25 prohibition
  unsatisfiable for one field. The second - one shared payload budget - is not rejected and stays
  live behind ADR-0043 as the open 2-vs-3 question. ADR-0015 is amended rather than superseded,
  the ADR-0029 shape, because its three-variants reasoning is not wrong, only incomplete.
depends_on: []
related:
  - kb-decision-0015
  - kb-decision-0021
  - kb-decision-0003
  - kb-decision-0043
  - kb-decision-0029
  - kb-open-question-cf-40-ownership-001
  - kb-decision-0053
  - kb-open-question-read-page-budget-001
source_paths:
  - .kb/_intake/2026-09-03-pre-publication-review.md
  - .kb/_intake/remediation-2026-09-04-briefs/event-metadata-floor.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
  - references/evaluation/review-pre-publication-2026-09-03.md
  - crates/happenstance-core/src/limits.rs
  - crates/happenstance-sync/src/identity.rs
  - crates/happenstance-sync/src/peer.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-09-07
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

## Resolved 2026-09-07 — status `superseded`; the shared-budget reading moves to ADR-0043

`kb-decision-0043` answers this with a shape none of the three candidates above named: a fourth
`StoreLimit` variant, `MetadataLen`, and a matching `Fixture::MAX_METADATA_LEN`, both minted with
`guaranteed_minimum()` returning `0`. It is the first answer's *mechanism* — a variant of its own —
without the first answer's *floor*. A store that physically cannot hold an event because of its
metadata gains a conformant refusal path; the contract guarantees no metadata capacity at all.

**The question was decided by evidence this atom assumed did not exist.** The body above reasons
from the four `MIN_SUPPORTED_*` constants and the three `StoreLimit` variants and finds no term for
metadata anywhere in the port surface. There is one, and it had been there all along:
`crates/happenstance-sync/src/identity.rs:199` computes `ReplicatedEvent::payload_len` as
`data.len()` plus `metadata.map_or(0, Bytes::len)`, and `crates/happenstance-sync/src/peer.rs:329`
checks that sum against `PeerLimits::max_event_bytes`, whose `minimum()` is anchored at `65_536` —
`MIN_SUPPORTED_EVENT_DATA_LEN`'s own number (`:312`). Two consequences follow. A separate non-zero
metadata floor `N` would mint an event every store must accept — 65,536 bytes of `data` plus `N` of
metadata — that the contract's own minimum peer must reject, so the first answer costs a change to
`PeerLimits` rather than four lines in two adapters. And the *second* answer, one shared budget, is
not the weaker reading this atom treated it as: it is already half-implemented in a port with
rustdoc, alongside `happenstance-cloudflare`'s informal margin.

**So the residue is not the floor, it is the unit.** ADR-0043 takes the reversible minimum and says
so: whether `payload_len`'s arithmetic *is* the contract's answer or a convenience local to the
replication port is left open there, and it is that question — not this one — that would decide
between the zero-floor channel and folding metadata into `data`'s budget outright. Two smaller
residues travel with it: whether a non-zero `MetadataLen` floor is ever minted and what would
anchor it, and a clause blessing `None` contributing `0` where VT-1 makes `None` and `Some(&[])`
two distinguishable values.

**No accepted decision was edited to produce any of it.** ADR-0015 is amended rather than
superseded — the ADR-0029 shape, `supersedes: null` with `depends_on: [kb-decision-0015]` — because
its argument for three variants is correct on its own terms and merely never enumerated this
candidate. The third answer, a written statement that metadata is deliberately unbounded, is
rejected outright: it is the only option that leaves `[FROZEN]` VT-25 unsatisfiable by construction
for one field, and `happenstance-cloudflare`'s margin is what "size around it" already looks like —
a guess folded into a different constant's derivation rather than a stated limit of its own.
