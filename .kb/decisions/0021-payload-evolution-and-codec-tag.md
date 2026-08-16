---
id: kb-decision-0021
title: The codec tag lives in Event::metadata, event types do not carry versions, and upcasting happens at decode
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0021
reversibility: low
phase: 7
supersedes: null
superseded_by: null
summary: >-
  Three answers, none of them taken at the design gate, because the public surface is invariant
  under all three - Codec::TAG is a &'static str either way. The codec tag lives in
  Event::metadata, inside a versioned framing region the typed layer owns and no store parses,
  with everything after it the application's own causation and correlation metadata, carried
  through unmodified. The decision turns on VT-3's second half rather than its first: anything a
  store, a peer, a conformance rule or a query must see belongs in EventType or Tags, and
  nothing below the port needs to see the codec tag because decoding is strictly above it. The
  falsifier is name the adapter change this choice forces, applied to both candidates: metadata
  forces none, while Tags obliges every adapter to store, index, match on and budget the tag,
  spending one of the 64 tags every store must accept on every event forever, and makes
  re-encoding change which QueryItems an event satisfies - so a consistency boundary silently
  changes shape as a consequence of a storage-format migration that was supposed to be
  invisible. EventType carries no version suffix: matching is exact equality, so CourseDefined.v2
  is invisible to every query already written and the first upcast makes existing decision models
  read an empty log with their append conditions matching nothing; an event type is a stable
  identity and the payload is what evolves. No read-path hook is needed, and the strategy that
  earns it is decode-time tolerance, which DomainEvent::decode already permits because it
  receives the event type and the raw bytes; its falsifier is an upcast needing information from
  outside the event being decoded, and if that is ever reached the route is a defect entry with a
  clause ID under AC-012, never a line edit of the frozen EventStore. An event with no framing
  region decodes with the codec in hand - Json under commit, C under commit_with - and not
  UnknownTag, because refusing it would make every event written before the typed layer existed
  unreadable with no legal repair, and because that fallback is the mechanism any future
  re-siting would reuse. UnknownTag is left meaning exactly one thing: a tag was written and this
  build cannot honour it. Rejected: the tag in Tags; either-home-is-fine; happenstance owning the
  whole metadata field; a serde-encoded framing region; a bare version suffix; a version suffix
  with a query-naming rule; an unearned no-hook answer; adding a hook to EventStore; UnknownTag
  for untagged events; answering in ADR-0016's terms. Reversibility is low, not high: reversing
  it after 0.2.0-alpha.1 means re-siting a tag on events already written into real stores, which
  VT-3 forbids any adapter from doing on the user's behalf.
depends_on:
  - kb-decision-0003
  - kb-decision-0006
  - kb-decision-0007
related:
  - kb-decision-0016
  - kb-decision-0020
  - kb-open-question-human-readable-encoding-limits-001
source_paths:
  - .kb/_intake/0021-payload-evolution-and-codec-tag.md
  - references/adr/0021-payload-evolution-and-codec-tag.md
  - .bklg/from-contract-to-published-library/typed-layer-and-alpha-release/_design.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/src/store.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-08-15
---

# The codec tag lives in Event::metadata, event types do not carry versions, and upcasting happens at decode

## Context

`happenstance-core` carries opaque `Bytes` and has no evolution problem of its own. The typed
layer has one from its first commit: a decoded `Enrolment::Defined { capacity: u32 }` breaks when
a field is added, and a store holding two encodings of the same event type at once needs a way to
tell them apart that no adapter is allowed to understand. Project AC-005 requires the tag to exist;
this record decides where it lives, whether `EventType` versions, and what happens on decode when
it does not.

`spec/SPECIFICATION.md:631-633` (`[FROZEN]`, VT-3) has two halves. The first — stores, peers and
the contract layer must not parse `data` or `metadata` — licenses nothing on its own. The second
decides the siting: anything a store, a peer, a conformance rule or a query *must* see must be
carried in `EventType` or `Tags`. The prior question is therefore whether anything below the port
needs to see the codec tag. It does not: decoding is strictly above the port
([kb-decision-0007](0007-projection-runner-decodes.md)), and `QueryItem::matches` looks at
`event_type` and `tags` only (`crates/happenstance-core/src/query.rs:112-115`). Had the answer
been yes, VT-3 would have *required* `Tags`, and this decision's own constraint — no adapter needs
to understand the tag — would already be violated. VT-3's `Rejects:` line, an ingest path writing
origin identity into `metadata` (`:643-646`), is the mirror case rather than a counter-example:
replication identity *is* something a peer must see.

## Decision

The codec tag lives in `Event::metadata` (`with_metadata` at
`crates/happenstance-core/src/event.rs:379`, read back at `:400`), inside a versioned framing
region the typed layer owns. Everything after that region is the application's own causation and
correlation metadata, carried through unmodified and never parsed by `happenstance`. The framing
region must be readable without knowing the payload codec, must add no dependency — so it works
under `--no-default-features` and on `wasm32` — and must be distinguishable from metadata that is
entirely the application's; its exact bytes are a later milestone's, not this record's.

`EventType` carries no version suffix. Matching is exact equality
(`crates/happenstance-core/src/query.rs:113`), so `"CourseDefined.v2"` is invisible to every query
already written, and the first upcast under a versioned type name would make an existing decision
model read an empty log — its append condition matching nothing. An event type is a stable
identity; the payload is what evolves. A genuinely new fact takes a new type name, and every query
that must see it names it deliberately.

No read-path hook is added to `EventStore`. The strategy that earns that is decode-time tolerance:
`DomainEvent::decode(codec, event_type, data) -> Result<Self, CodecError>` already receives the
event type and the raw bytes, so an older payload shape is handled inside the typed layer with no
port change. `EventStore` is `[FROZEN]` with exactly four required methods —
`read` (`crates/happenstance-core/src/store.rs:119`), `append` (`:213`), `head` (`:248`),
`contains_event_id` (`:268`) — and no hook; `read_decision_model` is a free function at `:321`, not
a fifth trait method, a citation `_design.md`, `_decomposition.md` and the story spec all get
wrong. The strategy's own falsifier is an upcast needing information from outside the event being
decoded; if that is ever reached, the route is a defect entry with a clause ID under AC-012, never
a line edit of the frozen `EventStore`.

The rule this implies, so no later milestone invents one: an event whose metadata carries no
framing region decodes with the codec already in hand — `Json` under `commit`, `C` under
`commit_with::<C>` — and not `CodecError::UnknownTag`. Refusing would make every event written
before the typed layer existed unreadable with no legal repair, since no adapter may rewrite stored
events (VT-3) and no backfill can run through a hook that does not exist. `UnknownTag` is left
meaning exactly one thing: a tag was written and this build cannot honour it.

## The falsifier

Naming the adapter change each candidate forces: `metadata` forces none. `Tags` obliges every
adapter to store, index, match on and budget the tag — spending one of the 64 tags every store
must accept on every event, forever (`crates/happenstance-core/src/limits.rs:27`) — and makes
re-encoding change which `QueryItem`s an event satisfies, so a consistency boundary silently
changes shape as a consequence of a storage-format migration that was supposed to be invisible.

## Consequences and alternatives rejected

No conformance rule exists for this, and the absence is the decision's own content: a testkit rule
that could observe the codec tag would prove it visible below the port, falsifying the decision
rather than verifying it. Event-type strings become part of the public commitment surface, since
the no-version-suffix decision makes them un-renameable. Reversibility is **low**, not the `high`
[kb-decision-0016](0016-the-wire-format.md) carries for the wire format: reversing this after
`0.2.0-alpha.1` means re-siting a tag on events already written into real stores, and the only
legal route is application-level re-emission — minting new identities and leaving the old events
in place forever.

Rejected: the tag in `Tags` (see falsifier); "either home is fine" (two admissible constructions
of one value is exactly `projection.rs:152-154`'s defect — an invalid value reachable through the
weaker constructor); `happenstance` owning the whole of `metadata` (removes the field an
application wants, with no replacement in the contract); a `serde`-encoded framing region
(unreadable under `--no-default-features`, and a `postcard`-only build could not read a
`json`-only build's tag — also barred by ADR-0003, which forbids `serde` in
`happenstance-core`'s default features); a bare version suffix on `EventType` (old events
unreachable in one commit); a version suffix with a query-naming rule (every query enumerating
every live version forever, duplicating what `decode` already does); an unearned no-hook answer
(the conclusion needs its strategy and its falsifier attached, not just its assertion); adding a
hook to `EventStore` (amends a `[FROZEN]` trait for a concern that belongs above the port);
`UnknownTag` for untagged events (unreadable legacy logs, no legal repair); and answering in
ADR-0016's terms (a second competing answer where the corpus already has one — ADR-0016 moves the
metadata bytes across a peer boundary and never reads them, this decision reads them and never
moves them, and a replicated event carries its framing region along as opaque bytes that
`happenstance-sync` never learns exist).
