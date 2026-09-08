---
id: kb-decision-0043
title: Event::metadata gets a refusal channel and no guaranteed capacity
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0043
reversibility: medium
phase: 12
supersedes: null
superseded_by: null
summary: >-
  Event::metadata was an opaque, unbounded field with no conformant way for a
  store to refuse an oversized value: AppendError::Store is forbidden from
  carrying a capacity refusal, and the only capacity variant that could stand in,
  StoreLimit::EventDataLen, names a different field. This decision mints a fourth
  StoreLimit variant, MetadataLen, and a matching Fixture::MAX_METADATA_LEN, both
  with guaranteed_minimum() == 0: a store that cannot hold an event because of its
  metadata gains a conformant refusal path, and no store is obliged to guarantee
  any particular capacity for it. A separate non-zero floor was rejected because it
  is incommensurable with happenstance-sync's already-shipped
  ReplicatedEvent::payload_len, which sums data and metadata into one budget
  checked against PeerLimits -- a separate floor would make a conformant-minimum
  event fail the contract's own minimum peer. The zero-floor channel is deliberately
  the reversible minimum under acknowledged uncertainty about whether the shared
  sum is the contract's real answer, not a ranking of that question on evidence.
  The code implementing this decision is not yet written.
depends_on:
  - kb-decision-0015
related:
  - kb-open-question-event-metadata-no-floor-001
  - kb-decision-0029
  - kb-open-question-cf-40-ownership-001
source_paths:
  - .kb/_intake/remediation-2026-09-04-briefs/event-metadata-floor.md
  - .kb/_intake/ratifications-2026-09-06-pre-publication.md
last_reviewed: 2026-09-07
---

# Event::metadata gets a refusal channel and no guaranteed capacity

## Decision

`StoreLimit` gains a fourth variant, `MetadataLen`, and `Fixture` gains a
matching `MAX_METADATA_LEN: Option<usize> = None` constant. Both are minted
with `guaranteed_minimum()` returning `0` for the new variant: a store that
cannot hold an event because of oversized metadata now has a conformant path
to refuse it, and the contract guarantees no minimum metadata capacity at all.
The code implementing this — the variant, the constant, the gated ceiling rule
— is not yet written; this decision settles the shape it will take.

## The gap this closes

Three MUSTs did not compose. `Event` is frozen at four fields, one of which —
`metadata` — is an opaque payload nothing bounds. The capacity vocabulary has
three `StoreLimit` variants, `EventDataLen`, `TagsPerEvent`, `EventsPerBatch`,
and none of them names metadata; `StoreLimit::EventDataLen`'s own documentation
says explicitly it is about `data`, not `data` and `metadata` together. And the
error channel a store would otherwise reach for is closed: `AppendError` MUST
NOT report a capacity refusal through `AppendError::Store`, and that clause is
`[FROZEN]`. A store that physically cannot hold an event because of its
metadata therefore had two moves, and both were non-conformant — `Store` is
forbidden, and naming `EventDataLen` for a metadata overflow is a lie about
which field failed. One shipping adapter, `happenstance-cloudflare`, had
already paid for this gap in writing, carving its `data` ceiling down to leave
room for the metadata the contract does not bound and saying so in its own
derivation comment.

## Why a shared floor was rejected

The natural alternative to a fourth variant is folding metadata into the
existing `data` floor and ceiling — treating the pair as one payload budget.
That reading is not a strawman: `happenstance-cloudflare`'s derivation already
works that way informally, and `happenstance-sync`'s `ReplicatedEvent::payload_len`
sums `data.len()` and `metadata.map_or(0, Bytes::len)` in shipped code,
checked by `PeerLimits::admits` against `max_event_bytes`, anchored at
`MIN_SUPPORTED_EVENT_DATA_LEN`'s own 65,536 — two implementations already
agreeing with the shared reading, one a port with rustdoc rather than an
internal habit.

What that same evidence rules out is a fourth variant carrying its own
**non-zero** floor. A separate floor `N` is incommensurable with
`payload_len`: a conformant-minimum event under a separate floor would be
65,536 bytes of `data` plus `N` of metadata, and `payload_len()` of
`65_536 + N` fails `PeerLimits::minimum().admits`, which compares against a
`max_event_bytes` of exactly `65_536`. A non-zero floor would mint an event
every store must accept that the contract's own minimum peer must reject,
forcing `PeerLimits` to change to absorb a number nobody has measured. No
observation in the workspace derives a floor in the first place — the only
code that writes metadata today is the typed layer's fixed framing prefix, on
the order of tens of bytes — so there is no evidence to anchor one against
even setting the incommensurability aside.

## Why zero rather than folding into the shared sum outright

Folding `data` and `metadata` into one budget now (making `StoreLimit::EventDataLen`
mean the pair, and rewriting its documentation to match) was a live option and
is deferred rather than rejected. Taking it now would make an append that
succeeds today fail tomorrow for the same byte count split differently, and it
would make `StoreLimit::EventDataLen`'s already-published documentation false
the moment it changed. This decision takes the narrower, more reversible step:
mint the refusal channel with no guaranteed capacity, without deciding whether
the contract's real unit of budget is the field or the pair. That is a
preference for reversibility under a genuinely unresolved question, not a
finding that the shared-sum reading is wrong — the evidence in the tree is
roughly two-to-one in the shared sum's favour and this decision does not
contest that count.

## What zero capacity costs and does not cost

A caller loses the ability to plan a metadata size and expect every conformant
store to honour it — `guaranteed_minimum()` returning zero means the
conformance-bug branch that number exists to signal is unreachable for this
variant until a future decision raises it. An adapter author's cost is the
same four-line ceiling check every other `StoreLimit` variant needs, minus any
floor obligation: strictly cheaper than a floor, because no adapter has to
guarantee a capacity it has not measured. The whole surface is gated on
`Fixture::MAX_METADATA_LEN` defaulting to `None`, so nothing here turns an
existing conformant adapter's suite red on its own — the ceiling rule skips by
default, with no ungated floor rule of the kind that would.

## What this does not settle

Whether `payload_len`'s shared-sum arithmetic is the contract's own answer or
a convenience local to the replication port is left open, and is the question
that would decide between this zero-floor channel and folding metadata into
`data`'s existing budget outright. Whether a future non-zero floor is ever
minted for `MetadataLen`, and what number it would carry, is also left open —
this decision states only that none is owed today and that any future floor
must be reconciled with `PeerLimits::minimum()`. Whether `happenstance-sqlite`
or `happenstance-cloudflare` declare a `MAX_METADATA_LEN` at all, and whether
cloudflare's existing `data` ceiling should be re-derived once a fourth term
exists, are adapter decisions this atom does not make. The wire-format and
read-path halves of the same question — base64 expansion on human-readable
formats, and `happenstance-neon`'s hard response ceiling — are untouched; no
`StoreLimit` variant addresses a read.

## Alternatives rejected

A fourth floor with a guaranteed non-zero minimum. Rejected on the
incommensurability with `PeerLimits::minimum()` above, and on the absence of
any measured application that needs one. A written statement that metadata is
deliberately unbounded, with no new variant at all. Rejected because it leaves
the `[FROZEN]` `AppendError::Store` prohibition permanently unsatisfiable for
one field — "size around it" is not an instruction an adapter can follow
against a field with no bound, and `happenstance-cloudflare`'s existing margin
shows what following it looks like: a guess folded into a different constant's
derivation rather than a stated limit of its own.
