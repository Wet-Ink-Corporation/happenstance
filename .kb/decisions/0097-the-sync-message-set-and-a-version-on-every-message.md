---
id: kb-decision-0097
title: The sync message set is four types in the envelope, and the format version stays on every message
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0097
reversibility: high
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Proposed for the owner on 2026-10-14 (#114). The replication message set is four types, one per
  message, each wrapped as Envelope<T> and never gathered into an enum of kinds: PushBatch (push),
  Ack (its answer), PullRequest (pull) and Pulled<ResumeToken> (its answer). Which message a buffer
  holds is the transport's to say, through the route or the request it answers. PushBatch,
  EventGroup, ReplicatedEvent, Ack and Pulled gain serde derives, and Watermark a hand-written pair
  whose Deserialize re-canonicalises (WF-10). The format version stays on every message and is not
  negotiated per connection, which is the reading WF-8's frozen title already gives. Version 1 is
  this envelope and this message set, and landing it does not bump FORMAT_VERSION, because no
  message at version 1 has crossed a wire. Payloads forward through happenstance-core's own codec,
  so WF-11 holds through the message set with no second codec. Reversible while happenstance-sync
  is unpublished; one-way from its first publish, when the wire format becomes a 1.0 promise
  (ADR-0066).
depends_on:
  - kb-decision-0016
  - kb-decision-0026
  - kb-decision-0027
  - kb-decision-0066
related:
  - kb-decision-0003
  - kb-decision-0016
  - kb-decision-0026
  - kb-decision-0027
  - kb-open-question-sync-message-set-undesigned-001
source_paths:
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-sync/src/peer.rs
  - crates/happenstance-sync/src/identity.rs
  - crates/happenstance-sync/tests/wire.rs
  - spec/SPECIFICATION.md
last_reviewed: 2026-10-14
---

# The sync message set is four types in the envelope, and the format version stays on every message

**Proposed on 2026-10-14 for #114, and not accepted.** The owner accepts or rejects it. No
long-form record exists; this atom is the record. `happenstance-sync` is `publish = false`, so
every answer here is free to change until the crate first publishes, and none of it is free after.

## Context

ADR-0016 §11 shipped `Envelope<T>`, `FORMAT_VERSION = 1`, a hand-written `Deserialize` that refuses
an unknown version before the message decodes, and `WireError`. It shipped no message set on
purpose, because the message set was phase 13's design. So `FORMAT_VERSION = 1` named a vocabulary
that did not exist, and `kb-open-question-sync-message-set-undesigned-001` asked three things:
whether there is a message set at all, whether the version is carried per message or negotiated
per connection, and what removing the field would cost if negotiation won.

ADR-0026 decided what a peer is (one relationship, one round trip per method, an owned resume
token, classified errors). ADR-0027 decided that replication is whole-log at 1.0. Neither named
the messages. Two methods on `SyncPeer`, `push` and `pull`, are the only exchanges.

## Decision

### 1. Four messages, each its own type in `Envelope<T>`

| Message | Type | Answers |
|---|---|---|
| push | `PushBatch` | — |
| ack | `Ack` | a push |
| pull | `wire::PullRequest { from: Option<ResumeToken> }` | — |
| pulled | `Pulled<wire::ResumeToken>` | a pull |

There is no `enum Message { Push(..), Pull(..), .. }`, now or later. An enum of kinds inside the
versioned type makes adding a kind a shape change, so every new message would bump the version,
and every peer not yet redeployed would refuse every message from one that had been. Four separate
types keep a fifth message additive. The cost is that a buffer does not say which message it holds.
The transport says it: an HTTP route, a Durable Object method, or the request a response answers.
That pairing belongs to the peer protocol (#117), not to the envelope.

A transport that gets the pairing wrong is not always told. A mis-routed buffer can decode silently
as another message, and that fails open. In postcard, `Envelope::new(PushBatch::empty())` and
`Envelope::new(PullRequest::from_start())` are the same two bytes, `[01 00]`, so an empty push
handed to the pull decoder is a pull from the start of the log. In JSON, a push decodes as
`PullRequest { from: None }` too, because a derive defaults a missing `Option` field to `None`.
`PullRequest` therefore carries `#[serde(deny_unknown_fields)]`, which closes the JSON case.
Postcard has no equivalent. It carries no field names, so nothing in the bytes can refuse.

`PeerLimits` is not a message. How a remote peer's limits reach the caller is the transport's
business, and adding it later is the additive case above.

### 2. The derives, by name

`PushBatch`, `EventGroup`, `ReplicatedEvent`, `Ack` and `Pulled` derive `Serialize` and
`Deserialize`. None of them has a constructor invariant to re-run, so the derive meets WF-10.
`Watermark` is the exception. Its `get` binary-searches a vector it keeps sorted, so a derived
`Deserialize` would accept a peer's unsorted vector and then miss marks that are there. Its
hand-written `Deserialize` sorts the pairs and keeps the highest mark per store. No
`skip_serializing_if` is added anywhere (WF-2), so an absent guard travels as `null`, not as a
missing key.

`Ack`'s `appended` and `skipped` are `u64`, not `usize`. They cross the wire, and a count must have
the same width on a 64-bit sender and a 32-bit receiver.

`ResumeToken` is text, not bytes. A token is what an HTTP peer puts in a query string or a Durable
Object in a cursor column. Bytes would render in JSON as an array of integers (WF-11's own
measurement). How a peer's `SyncPeer::Resume` becomes a `ResumeToken` is the peer adapter's choice.

The alternative is bytes rendered as base64 in JSON, which is how WF-11 renders a payload. Two facts
weigh for it:

- The codec is already in this crate's graph. `happenstance-core`'s `serde` feature, which this
  crate enables, pulls in `base64` (`crates/happenstance-core/Cargo.toml:62`). Bytes would cost a
  manifest line, not a new crate.
- The workspace's own HTTP peer shape is binary. `real_peer_shapes.rs` declares
  `struct HttpResume(Box<[u8]>)` (`crates/happenstance-sync/tests/real_peer_shapes.rs:171`), and
  the cursor probe declares `type Resume = Box<[u8]>`
  (`crates/happenstance-sync/tests/cursor_shape_probe.rs:149`).

With a text token, every adapter whose resume point is binary text-encodes its own cursor and picks
its own alphabet. With a bytes token, the wire picks the alphabet once, and a text cursor is the
adapter that converts. This record proposes text; the choice is the owner's.

### 3. The version is carried on every message, and version 1 is this set

Per message, not per connection. WF-8, which is `[FROZEN]`, is titled "The envelope is versioned,
once per message", and its body requires every message to carry the field. Negotiation per
connection would break that clause. It would also assume a connection, and SY-15 forbids that
assumption, because the one-shot HTTP peer has no connection to negotiate on. A message that
outlives its connection, parked after an `UnsupportedFormatVersion` and retried after an upgrade,
must still say which version it is. So the field stays, and no removal needs a disposition.

`FORMAT_VERSION` stays 1, and version 1 now means this envelope and these four messages. Landing
the set is a shape change by WF-8's own test, but nothing at version 1 has ever crossed a wire: the
crate has never been published, and the envelope carried only test strings. A bump would protect
no peer. The next shape change bumps it, and a capacity bound still never does (WF-9).

### 4. WF-11 through the message set

The payload is not re-encoded. `ReplicatedEvent` carries core's `Event`, and `Event`'s own `serde`
impl renders `data` and `metadata` as base64 in human-readable formats and raw bytes otherwise. The
message set adds no codec, attribute or helper that touches either field.

## Evidence

In `crates/happenstance-sync/tests/wire.rs`. Each test says which formats it runs in, and whether
it has a control (a compiled wrong implementation that the same assertions reject):

- `wire::the_message_set_round_trips_in_both_formats`: all four messages round-trip in JSON and
  postcard, `Ack` with `appended` at `u64::MAX`. It has no control. A derived pair round-trips by
  construction, so this is a smoke test that each type encodes at all.
- `wire::a_real_message_is_refused_before_it_decodes`: in JSON, a malformed `PushBatch` at an
  unknown version fails as a version refusal, and a derived envelope over the same bytes fails as
  malformed instead. In postcard, the version followed by a batch that claims five groups and ends
  fails as `SerdeDeCustom`. At the supported version, and through the derived envelope, the same
  batch fails as `DeserializeUnexpectedEnd`.
- `wire::payload_bytes_render_through_the_core_codec`: `[de ad be ef]` is `"3q2+7w=="` in JSON
  and four raw bytes in postcard. Its control, a payload carried in a field of its own, round-trips
  equal in JSON and renders the integer array.
- `wire::push_batch_preserves_group_boundaries_on_the_wire`: the groups and each guard travel in
  JSON and postcard. Its control flattens a guard-free batch to its events in postcard. It
  round-trips the events equal, and regrouping them gives a different number of groups from the
  ones the origin wrote.
- `wire::a_watermark_decodes_canonical`: an unsorted, duplicated watermark decodes canonical in
  JSON and postcard. Its control, the derive, keeps both marks and the sender's order in both.
  `wire::a_descending_watermark_decodes_canonical` covers strictly descending input, which the
  single sort-and-fold decode handles in one pass.
- `wire::a_json_push_is_refused_as_a_pull_request`: a push handed to the `PullRequest` decoder is
  an error in JSON. Its wrong implementation is `PullRequest` without `deny_unknown_fields`. That
  version decodes the push as `PullRequest { from: None }`, and this test failed against it first.
- `wire::a_misrouted_postcard_buffer_decodes_as_another_message`: pins that an empty push and a
  first pull are byte-identical in postcard, and that the push decodes as a pull. It records the
  fail-open case in §1 and does not prevent it.

## Consequences

- An adapter author has a vocabulary to implement `SyncPeer` against over a real transport.
- The open question's sub-questions 1 and 2 are answered, and 3 does not arise. The atom is amended
  to point here, and it closes when this record is accepted.
- No clause changes. WF-8 already says "once per message". WF-11 stays `[PROVISIONAL]` on its own
  falsifier, which this record does not touch.
- After the first publish, removing a message type, changing one's shape, or moving the version
  into a handshake is a format break. Until then each is a one-line change.
- A transport that routes a postcard buffer to the wrong decoder can get a valid message of the wrong
  kind, and nothing in the bytes refuses it (§1). Routing is the transport's correctness to own, and
  #117 inherits it.

## What would make this wrong

A transport that cannot say which message a buffer holds. Every transport in the workspace pairs a
response with its request, and an HTTP peer routes by path. A transport that multiplexes unrelated
messages over one untyped stream would need the kind on the wire. That would be a new
wrapper type outside the envelope, or a version bump that adds one. The shape here leaves room for
either, and an enum inside the envelope would not.
