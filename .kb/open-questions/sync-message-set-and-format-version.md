---
id: kb-open-question-sync-message-set-undesigned-001
title: FORMAT_VERSION = 1 names a message set that does not exist yet
kind: open_question
status: accepted
authority_tier: note
summary: >-
  ADR-0016 ships a generic envelope with a hand-written Deserialize that refuses an unknown
  format_version before decoding the message, and deliberately ships no enum of message kinds —
  the message set is phase 13's design, and inventing one at phase 5 would be designing the
  replication protocol inside a wire-format decision. So the version field currently names nothing
  in particular: FORMAT_VERSION = 1 is a promise about a vocabulary that has not been chosen. What
  is not decided is what that vocabulary is, and therefore what a version bump would mean.
  Refuted by a phase-13 design in which versions are negotiated per connection rather than carried
  per message, which would make the field dead weight on every message and removing it a format
  break. Owned by phase 13. Distinct from the question about the DCB reference's absent format:
  that one is about whether anyone else's encoding matters, this one is about what our own
  messages are.
depends_on: []
related:
  - kb-decision-0016
  - kb-open-question-dcb-no-published-format-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - references/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-sync/src/lib.rs
last_reviewed: 2026-08-10
---

# FORMAT_VERSION = 1 names a message set that does not exist yet

## What is true today

`happenstance-sync::wire` (ADR-0016, §11) defines `Envelope<T>`, a generic wrapper carrying a
`format_version: u16` field stamped with the crate constant `FORMAT_VERSION = 1`, plus a
message payload of type `T`. The `Deserialize` impl is hand-written rather than derived, and for
a reason the module's own doc comment states directly: field order is part of the contract, so
`format_version` is declared first and checked by `check_format_version` before the message field
is touched at all. `serde::de::Error::custom` is the only channel a `Deserialize` impl has out, so
an envelope with an unrecognised version is refused as a deserialization error — the message body
is never decoded, never even inspected for shape. This is measured behaviour, not aspiration:
`{"message":null,"format_version":999}` decodes to `Err` under that same impl, version rejected
before `null` would have failed as a message in any case.

What `Envelope<T>` does **not** carry is any enumeration of what `T` might be. The module comment
is explicit about why: `enum Message { Push(..), Pull(..) }` would put the message set beside the
version check, and the message set is exactly what phase 13 — the peer protocol itself — has not
designed. ADR-0016 settles WF-2 through WF-12 but explicitly does not settle the replication
protocol; it lands a versioned envelope and nothing else, no message set, no negotiation, no
`SyncError` extension. Those stay ADR-0027's and ADR-0026's respectively, both still ahead in the
phase order.

So the field that exists — `format_version` — is fully specified and tested; the field's *meaning*
is not. `FORMAT_VERSION = 1` currently promises only that some vocabulary is version 1 of itself,
without saying what that vocabulary contains. A reader encountering the constant today cannot
answer "version 1 of what protocol" beyond "the envelope-wrapping and refusal behaviour ADR-0016
specifies" — nothing about `Push`, `Pull`, or any other message phase 13 will eventually define.

## What is not decided

What the message set is, and consequently what a `format_version` bump is even measuring. Two
shapes are live and mutually exclusive in their consequences for this field:

- **Per-message versioning** (what the field's placement implies today): every envelope carries
  its own version, and a connection can in principle mix versions across messages if the peers
  ever needed that.
- **Per-connection negotiation**: peers agree a protocol version once, at connection setup, and
  every message on that connection is implicitly that version — the field on `Envelope` becomes
  redundant on every message after the first.

## What forces it

Phase 13's peer-protocol design. The moment a message set is drafted, this question is answered as
a side effect of asking "how do two peers agree what version they speak" — but if that design
lands on per-connection negotiation, `format_version` on every envelope becomes dead weight that a
removal would make a wire-format break, exactly the kind of change ADR-0016 was written to make
cheap while there is no external format to diverge from (see
`kb-open-question-dcb-no-published-format-001`). The two questions are adjacent but distinct: that
one asks whether anyone else's encoding constrains ours; this one asks what our own messages, once
named, turn out to need from the version field already shipped.

## Ordered sub-questions

1. Does phase 13 draft a message set at all as a single enum, or does the peer port stay agnostic
   to message shape the way `Envelope<T>` is generic today?
2. If a message set exists, is version negotiated once per connection or carried once per message
   — the fork that decides whether `format_version` on `Envelope` stays load-bearing?
3. If negotiation wins, is removing the per-message field treated as a breaking wire-format change
   requiring its own ADR, or as an implementation detail ADR-0016 already left open by shipping no
   message set to protect?
