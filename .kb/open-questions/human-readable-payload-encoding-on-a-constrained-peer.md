---
id: kb-open-question-human-readable-encoding-limits-001
title: Whether a human-readable payload encoding is available at all on a memory-limited peer
kind: open_question
status: accepted
authority_tier: note
summary: >-
  WF-11 adds base64 for the human-readable half of the wire format, and stays PROVISIONAL on a falsifier that turns out to be broader than base64. serde's Serializer offers no streaming entry point for a human-readable string: serialize_str and collect_str both materialise the whole payload first. That is a property of serde's data model rather than of base64, so it falsifies human-readable payload encoding as a category — hex has the same problem, and so would any alternative — and if it fires, WF-11's MUST has to be re-scoped to formats rather than to peers. What is not decided is whether any peer will actually meet the condition. Refuted by a Workers peer that must forward a payload it cannot buffer, which is the first realistic place a memory ceiling and a large payload meet. Owned by phase 9, the Cloudflare Durable Object adapter. The binary half is unaffected: postcard encodes the same 340 KiB payload in 348,163 bytes against base64's 464,218.
depends_on: []
related:
  - kb-decision-0016
  - kb-decision-0003
  - kb-reference-wire-format-measurements-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - references/adr/0016-the-wire-format.md
  - crates/happenstance-sync/src/wire.rs
  - experiments/wire-format/
last_reviewed: 2026-08-10
---

# Whether a human-readable payload encoding is available at all on a memory-limited peer

## What is true today

ADR-0016 (WF-11) settles that `Event::data` and `Event::metadata` encode as
standard-alphabet base64 in human-readable wire formats (JSON) and as raw
byte strings in binary formats (postcard) — `base64` is added as an optional
dependency reachable only through the `serde` feature, priced at zero new
crates because `sqlx-core`/`sqlx-postgres` already pull `base64 0.22.1` into
the workspace graph with no transitive dependencies of its own.

The clause's marker stays `[PROVISIONAL]`, and the ADR is explicit that its
two halves are now in different states. The `no_std` half is **closed by
measurement**: a standalone `#![no_std]` + `alloc` crate depending only on
`base64` with `default-features = false, features = ["alloc"]` checks clean
against `wasm32-unknown-unknown`. That half "leaves the falsifier rather than
staying in it with a note attached, because a falsifier that has been
answered is not a falsifier."

The **buffering half does not close**, and the finding widens rather than
narrows the risk. It is read from serde's `Serializer` API directly rather
than measured, and the ADR flags that difference explicitly because its
neighbour in the same section is a measurement: `serialize_str` takes a
`&str` and `collect_str` takes a `Display`, and both write the *entire*
rendering into the output before returning — serde's data model has no
streaming entry point for a human-readable string at all. The consequence
stated plainly: "**any** human-readable payload encoding must materialise
the whole payload; hex has the same property." So the falsifier is not about
base64's efficiency or dependency cost — it falsifies human-readable payload
encoding *as a category*, for any encoding scheme built on serde's current
trait shape.

The binary half carries no such exposure. WF-11's own measurement, run
against a 340 KiB GeoJSON payload standing in for Turnstile's seat map: raw
`bytes::Bytes` serialised as a JSON array of decimal integers costs 1,243,464
bytes (3.5715× raw); base64-as-a-JSON-string costs 464,218 bytes (1.3333×);
postcard's raw byte encoding costs 348,163 bytes (1.0000×, i.e. essentially
free). Base64-in-JSON is 2.6786× smaller than the naive integer-array
encoding it replaces, but it is still a full in-memory materialisation
regardless of size.

## What is not decided

Whether any peer in happenstance's actual deployment plan will ever meet the
condition that makes the falsifier fire — a peer that must *forward* a
payload larger than it can hold in memory at once. The ADR names the
condition precisely without asserting it will occur: "if it fires, WF-11's
MUST becomes unsatisfiable on that peer and the clause is re-scoped to
formats rather than peers." Re-scoping to formats would mean human-readable
(JSON) wire format stops being universally available and becomes a
capability some peers decline, the same declined-capability shape used
elsewhere in the fixture contract for numeric limits.

## What forces it

A Cloudflare Workers/Durable Object peer that must relay a payload it cannot
buffer whole — the first realistic point where a memory ceiling and a large
payload meet in this workspace's own adapter plan. The ADR assigns this
explicitly to **phase 9**, the Cloudflare adapter, and is careful to say why
it is not phase 5's (where WF-11 itself lands): recording it there "would be
a marker phase 5 can neither lift nor fail — the shape ADR-0015 §7 names and
CF-38 exists to prevent." A provisional marker must be owned by a phase that
can actually observe its falsifier.

## Ordered sub-questions

1. Does phase 9's Durable Object adapter actually need to forward a
   `MIN_SUPPORTED_EVENT_DATA_LEN`-sized (65,536 byte) or larger payload
   through JSON, or does its real workload stay small enough that the
   falsifier never fires in practice, leaving WF-11 provisional but
   unfalsified indefinitely?
2. If the falsifier does fire, is the fix a streaming-capable encoding scheme
   outside serde's current trait shape, a capability declaration analogous to
   `CF-40`'s numeric limits (a peer declares it cannot serve human-readable
   format above some size), or a hard scoping of JSON support to development
   and diagnostic use only?
3. Does this question's resolution have any bearing on ADR-0003's opaque-payload
   boundary, given that any fix touching how `Bytes` is streamed through
   `Serialize` sits adjacent to the byte-for-byte forwarding promise that ADR
   protects?
