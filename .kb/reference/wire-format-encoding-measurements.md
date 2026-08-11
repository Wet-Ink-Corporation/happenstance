---
id: kb-reference-wire-format-measurements-001
title: Wire-format encoding measurements, and two instruments that measured wrong
kind: reference
status: accepted
authority_tier: note
summary: >-
  The measurements ADR-0016 rests on, taken 2026-08-09, kept in experiments/wire-format/ outside
  the workspace and the gate, reproducible with cargo test -- --nocapture. A 340 KiB Turnstile
  payload encodes to 1,243,464 bytes as a JSON array of raw bytes, 464,218 as base64, 696,322 as
  hex and 348,163 in postcard. skip_serializing_if is worth 26 bytes of JSON and one byte per
  field of postcard, not the figures the clauses claimed; StoreId's JSON size ratio is 1.7353x,
  not 4x; base64 costs zero new crates, already in the graph via sqlx's 0.22 pin. Two instruments
  were measured and found wanting: a postcard round trip of a lone Event fails with "Hit the end
  of buffer" for three of four shapes, and a compile_fail doctest passes whenever the snippet
  fails to compile for any reason — three of four deliberately-broken spellings reported green
  against a false assertion. W7 found the DCB reference publishes no wire format: EventStore.ts
  contains no serialisation code.
depends_on: []
related:
  - kb-decision-0016
  - kb-open-question-dcb-no-published-format-001
  - kb-open-question-human-readable-encoding-limits-001
source_paths:
  - .kb/_intake/0016-the-wire-format.md
  - references/adr/0016-the-wire-format.md
  - experiments/wire-format/
  - crates/happenstance-sync/src/wire.rs
last_reviewed: 2026-08-10
---

# Wire-format encoding measurements, and two instruments that measured wrong

## What this is a pointer to

The programs that produced every figure below live in `experiments/wire-format/`, outside the
workspace and outside the gate, and `cargo test -- --nocapture` from there regenerates them,
seed included. This atom is the citable summary; what ADR-0016 decided because of these numbers —
which fields the format normalises, whether `#[serde(default)]` stays, how the payload is
encoded — is the decision atom's, not this one's. Kept separate from ADR-0020 (the wire-format
decision atom) for the ordinary reason: two of the findings below are about instruments that
measured *wrong*, dated facts about one toolchain rather than conclusions, and folding them into
the decision would make the decision unreadable on its own.

## Payload encoding, on a 340 KiB Turnstile GeoJSON payload

Fixed seed, `xorshift64*`, 348,160 raw bytes:

| Encoding | Bytes | Ratio to raw |
|---|---|---|
| `serde_json` array of decimal integers | 1,243,464 | 3.5715x |
| base64 (JSON string) | 464,218 | 1.3333x |
| lowercase hex (JSON string) | 696,322 | 2.0000x |
| postcard | 348,163 | 1.0000x |

Base64-in-JSON is 2.6786x smaller than the array-of-integers encoding it replaces. One unit
correction: a clause's "roughly 1.3 MB" overstates the array-of-integers figure by ~4.6% decimal
and ~9.6% binary — 1,243,464 bytes is 1.243 MB (10^6) and 1.186 MiB (2^20).

## `skip_serializing_if`, measured rather than assumed

Restoring the one absent field on a tagged `Event` in postcard takes it from 18 to 19 bytes;
restoring both absent fields on an all-defaults `Event` takes it from 7 to 9 — **one byte per
absent field**, not "nothing at all" as one clause claimed. In JSON, "35 to 61 bytes" is correct
only for a four-byte payload rendering as one decimal digit per byte; the same shape with
`11 22 33 44` goes 39 to 65, a zero-length payload 28 to 54, and the tagged case 60 to 76 because
only `metadata` is restored. The stable, format-independent figure a frozen clause can safely
cite is **+26 bytes** in JSON.

## `StoreId`'s JSON size

`StoreId` is `[u8; 16]`, not the 32-element array one clause's `Rejects:` line claimed. Measured:
59 bytes as a JSON integer array against 34 bytes of quoted hex — a ratio of **1.7353x**, not
4x — with an all-`0xff` ceiling of 1.9118x and an all-`0x01` floor of 0.9706x (cheaper than hex).
The decision to encode `StoreId` as hex survives the correction; what carries it is illegibility
of a raw byte array, not the size claim, which was wrong by roughly a factor of two in both
directions at once (element count *and* ratio).

## `base64`'s cost to the dependency graph

`base64 0.22.1` is already in the workspace graph: `cargo tree --workspace -i base64 --depth 1`
reports `sqlx-core` and `sqlx-postgres` pulling it in for `happenstance-postgres`, and
`cargo tree -p base64` is one line — no transitive dependencies of its own. Adding it as an
optional dependency of `happenstance-core`, gated behind the `serde` feature, costs the workspace
zero new crates and costs a consumer with no `sqlx` in their graph exactly one leaf crate. Its
`no_std` half was confirmed by a `#![no_std]` + `alloc` crate checking clean on
`wasm32-unknown-unknown` with `base64 = { version = "0.22", default-features = false, features =
["alloc"] }`.

## Two instruments measured and found wanting

**A postcard round trip of a lone `Event` fails outright for three of four field-presence shapes.**
With `skip_serializing_if` still in place, encoding an `Event` with no tags and no metadata (7
bytes), tags but no metadata (18 bytes), or no tags but metadata (11 bytes) all fail decode with
`Err(Hit the end of buffer)`. Only the shape carrying both tags and metadata (22 bytes) round-trips.
All four round-trip cleanly in `serde_json`, which is exactly the gap a JSON-only round-trip test
cannot see.

**A `compile_fail` doctest passes whenever the snippet fails to compile, for any reason at all.**
Measured against four deliberately broken spellings of the same "this type must not implement
`Serialize`" assertion — a misspelled type name, a misspelled trait path, a wrong crate path, and
the honest correct spelling — three of the four broken spellings reported green under
`compile_fail`, and only the honest one correctly failed with "marked `compile_fail`, but this
code compiles". A `compile_fail` doctest with no pinned error code cannot distinguish "the
assertion held" from "I made a typo," and the replacement instrument is a `const`-evaluation
assertion inside an integration test, which fails the *build* rather than merely reporting an
unpinned doctest outcome.

## W7 — does the DCB reference implementation publish a wire format?

No. `EventStore.ts`, fetched from its canonical source, contains no serialisation code of any
kind, and the DCB specification's own JSON snippets are labelled "a potential JSON
representation" beside "implementations are not required to use the same terms or function/field
names." A clause's phrase "the reference implementation's published shape" has no referent — this
is a finding about the reference, not a gap in the measurement.
