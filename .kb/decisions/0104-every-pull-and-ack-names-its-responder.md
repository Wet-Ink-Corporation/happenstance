---
id: kb-decision-0104
title: Every pulled and ack message names the store that answered, and the wire format is version 3
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0104
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Proposed on 2026-11-08 for #244. The owner chose the mechanism himself on the 2026-11-08
  project-thread card: Pulled and Ack each gain a last field, responder: Option<StoreId>, naming the
  store incarnation that answered. It is how a sync runner learns that a peer was restored or
  re-minted under a new StoreId (VT-6); nothing else through the frozen port says so. Optional,
  because MemorySyncPeer, the stand-in peers and any peer without a store of its own have no
  truthful answer. Pulled::new and Ack::default say None; with_responder says more. StoreSyncPeer
  stamps its ingest half's store_id() on every answer, read per call and never cached, and
  serve_sync and RemoteSyncPeer carry it through the envelope. A field added is a shape change, so
  wire::FORMAT_VERSION goes from 2 to 3, as ADR-0097 §3 and WF-8 require; a version-2 buffer is
  refused whole. Amends ADR-0097's message set, and supersedes nothing.
depends_on:
  - kb-decision-0097
  - kb-decision-0026
related:
  - kb-decision-0016
  - kb-decision-0086
  - kb-decision-0098
source_paths:
  - crates/happenstance-sync/src/peer.rs
  - crates/happenstance-sync/src/wire.rs
  - crates/happenstance-sync/src/store_peer.rs
  - crates/happenstance-sync/src/serve.rs
  - crates/happenstance-sync/tests/wire.rs
  - crates/happenstance-sync-testkit/tests/store_sync_conformance.rs
  - crates/happenstance-sync-testkit/tests/remote_sync_conformance.rs
last_reviewed: 2026-11-08
---

# Every pulled and ack message names the store that answered, and the wire format is version 3

**Proposed on 2026-11-08 for #244.** The owner made the call himself on the 2026-11-08
project-thread card: the runner learns of a restore from "an optional responder `StoreId` on every
`Pulled` and every `Ack`". The session that wrote this record built it. Acceptance is the owner's,
and it flips `status` alone. No long-form record exists; this atom is the record.

## Context

The sync runner (#244) drives one edge per peer relationship. When the far side is restored from a
backup, or re-minted after one, it becomes a different store incarnation (VT-6), and the edge has to
stop rather than carry on against a log whose history has moved under it. The runner has to learn
this from the peer, and nothing on the frozen port says it:

- `Ack::confirmed` names the stores the far side **holds**, per origin. It does not say which of
  them is the far side itself.
- VT-6 allows mint-per-open, so a new origin in a watermark is routine, not evidence of a restore.
- A confirmation that goes backwards is no signal either. The next push raises the high-water mark
  again, so a runner that watches for a regression sees it for one answer at most, and only if it
  happens to look then.

## Decision

### 1. A `responder` field, last, on `Pulled` and `Ack`

```rust
pub struct Pulled<R> { pub batch: PushBatch, pub resume: R, pub waiting: Waiting,
                       pub responder: Option<StoreId> }
pub struct Ack { pub appended: u64, pub skipped: u64, pub confirmed: Watermark,
                 pub responder: Option<StoreId> }
```

`responder` is the store incarnation that answered this pull or took this push. It is declared
**last** on both structs because postcard is positional: the fields before it keep their places,
as `Pulled::waiting` did for SY-37. Both structs were already `#[non_exhaustive]`, so the field is
not a source break for a consumer that builds them through `Pulled::new` or `Ack::default`.

`Pulled::new` and `Ack::default()` say `None`. A builder, `with_responder(StoreId)`, says more on
each type. That follows the `with_waiting` pattern: a peer that cannot tell keeps calling the
constructor and says nothing rather than something false. Like every field on the wire, it is
always present, and `None` travels as `null` in JSON and as `00` in postcard (WF-2).

### 2. Optional, not required

A required `StoreId` would force every peer to name a store, and not every peer has one. The
stand-in peers in `tests/` have none. A peer that relays for others, or one over a store it cannot
ask, has none it can vouch for. `MemorySyncPeer` holds a `StoreId` it was built with, but nothing
re-mints it, so stamping it would prove nothing. A required field would make all of these invent
an identity, and an invented identity is the false positive the runner check exists to avoid.
With `Option`, `None` means "this peer does not say", and a runner treats it as no evidence either
way.

The **value** is optional; the **key** is not. A JSON pulled or ack without `responder` is refused
as a missing field, rather than read as `None`. A derived `Deserialize` would read it as `None`,
because serde routes an absent `Option` through `missing_field`, so `Pulled` and `Ack` deserialize
through private shadow structs whose `responder` names a `deserialize_with` function, which turns
that fallback off. Postcard is positional and refuses a body one field short already.

This is the opposite of WF-2's read-side asymmetry at `Guard::after`, and on purpose. A missing
`after` decodes to `None`, which checks the whole log: the default is the strict reading, so it
fails closed. A missing `responder` decoding to `None` would read as "this peer does not say",
which is the lenient reading. A runner would treat a peer that dropped the key as offering no
evidence, and so would never see the re-mint the field exists to report. Here the fallback fails
open, so it is refused. WF-2 binds the encoder and permits either read side, so this needs no
clause change.

### 3. `StoreSyncPeer` stamps its store, read per call

`StoreSyncPeer` answers from a store, so it says which one. It reads its ingest half's
`IngestStore::store_id()` after the page on a pull, and after the watermark on a push, and puts it
on the answer. It does this on every call and never caches the value. A cached identity is the one
a re-mint does not reach, and the re-mint is the change this field exists to report. The Neon
store makes the same choice for the same reason (ADR-0086: nothing is cached).

It reads the identity after the page, not before. If a re-mint lands between the two, an old
incarnation's page is labelled with the new identity. That errs toward stopping the edge, which is
the safe direction for SY-34. Read first, the same race would label the new incarnation's page with
the old identity, and the edge would carry on.

The cost is one more store read per pull and per push. On a store that holds its identity in memory,
that is a field load. On SQLite and Postgres it is an indexed read of a one-row table. On Neon it is
one more read-only HTTP round trip **to the database**, because its `store_id` reads the meta row
each time. It adds nothing on the **sync** wire. A remote pull or push is still one exchange (SY-15),
and the identity travels in the reply.

A failure to read it is a new arm, `StorePeerError::Identity(W)`. The store's error states its class
(SY-36), and the enum was already `#[non_exhaustive]`. After a pull nothing was forwarded. After a
push the ingest has landed, and re-delivery converges as it does after a `Watermark` failure.

`serve_sync` copies the field onto the wire reply. It builds the reply as a struct literal, so a
field added to `Pulled` later is a compile error there until it is carried. `RemoteSyncPeer` decodes
the field with the rest of the message.

### 4. Wire format version 3

Adding a field is a change to a wire type's shape, so `wire::FORMAT_VERSION` goes from 2 to 3. The
constant's own documentation, "What bumping it means", ADR-0097 §3 ("the next shape change bumps
it") and WF-8 ("The version MUST be bumped whenever the shape of any wire type changes") all require
this. A version-2 buffer is refused whole as `WireError::UnsupportedFormatVersion { found: 2,
supported: 3 }`, and its message is never decoded.

The bump is not a formality. Without it, a version-2 body would reach the message decoder, and the
only thing between a version-2 peer and a quiet "no evidence" would be the required key in §2,
which is a missing-field error rather than a refusal that names the version. With the bump, a
mixed deployment gets the distinguishable refusal WF-8 exists to give, before the message is
decoded at all.

`happenstance-sync` has been published since `0.4.4`, so this is a wire break for any deployment
that runs two builds. Every peer must move to version 3 together. It ships in the next breaking
release (`0.6.0`), and the CHANGELOG says so.

## Rejected

- **A separate incarnation call.** This would be an `identity()` method on the port, or a fifth
  message on the wire. It costs its own round trip, and that is the round trip SY-15 makes the
  expensive one on the one-shot peer. It also has a race: the answer it gives can differ from the
  one that served the next pull, so a runner would have to make the call before every exchange to
  trust it, which doubles them. Putting the identity on the answer itself names the store that
  answered *that* exchange, at no cost on the wire. A new method on the frozen port would also need
  a new trait (RS-40-1).
- **Inferring the restore from regressing confirmations.** This is decorative. A confirmation that
  went backwards is raised again by the next push, so a runner sees it for at most one answer. A
  restore that loses no event the runner had confirmed shows no regression at all. And a new origin
  in the watermark is routine under VT-6's mint-per-open.
- **A required `StoreId`.** Rejected in §2. It forces the peers without a store to invent one.

## Evidence

- `wire::pulled_names_its_responder_last_in_both_formats` and
  `wire::ack_names_its_responder_last_in_both_formats` pin JSON and postcard for `None` and `Some`.
  The key comes last, `null` is present, and the postcard bytes are the earlier fields followed by
  `00`, or by `01` and the sixteen raw bytes. They reject the field declared earlier, and
  `skip_serializing_if`.
- `wire::a_version_2_pulled_is_refused_as_an_unsupported_version` checks that a version-2 pulled
  message is refused by number in both formats.
- `wire::a_current_version_body_without_a_responder_key_is_refused_in_json` checks that a pulled
  and an ack in the version-2 shape, stamped with the current number, are refused as ``missing
  field `responder` ``. It rejects a derived `Deserialize`: with the `deserialize_with` removed
  from either shadow struct, that half decodes `Ok` with `responder: None`, which was checked by
  mutant on each.
- `wire::pulled_carries_waiting_in_both_formats` moves its pins to include the trailing `null` and
  `00`. `wire/codec.rs`'s version byte, and the two doc examples that print an envelope, move to
  `3`.
- `every_answer_names_the_far_store_as_its_responder` (`store_sync_conformance.rs`) checks that an
  ack, a page, an empty pull, and a pull past the last numberable position (the early return) all
  name the far store, and that a second store's peer names the second store. It rejects a peer that
  forgets the early return, which was checked by applying that mutant locally.
- `a_far_store_re_minted_between_two_answers_is_named_anew_on_the_second`
  (`store_sync_conformance.rs`) puts an ingest half in front of the store whose identity changes
  after its first read, and checks that two pulls, and two pushes, name the two identities. It
  rejects a peer that caches the identity. With a `OnceCell` cache applied as a mutant, it failed
  at the second pull, which named the first identity.
- `the_responder_survives_the_envelope_on_pull_and_push` (`remote_sync_conformance.rs`) rejects a
  `serve_sync` that rebuilds the reply through `Pulled::new`, which was also checked by mutant.

## Consequences

- A runner can record the first responder it sees on an edge and stop the edge when a later answer
  names a different one. That check belongs to `happenstance_sync::runner::Edge` (#423) and lands
  after it.
- No `[FROZEN]` clause changes. WF-8 already requires the bump. SY-15's one round trip holds,
  because the identity rides in the reply. VT-6 is untouched. The field is evidence a runner may
  act on, and it does not change what a store mints.
- Neon pays one more database round trip per pull and per push through a `StoreSyncPeer`. If that
  cost bites, the remedy is a combined read on the store, such as a `store_id` returned beside the
  page. That needs a port change, and its own record.
- `MemorySyncPeer` names no responder. If a runner's tests want it to, stamping its `StoreId` is
  additive.

## What would make this wrong

A peer that can answer from more than one incarnation in one relationship, for example a load
balancer in front of replicas that each mint their own `StoreId`. Every answer would then name a
different responder, and a runner that stops on a change would never get going. The field would
still be truthful. What would have to change is the runner's rule: what a relationship's identity
is.
