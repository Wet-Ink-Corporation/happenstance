---
id: kb-decision-0106
title: ADR-0003 is no longer provisional, because two unlike peers round-trip a payload byte for byte
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0106
reversibility: low
phase: 13
supersedes: null
superseded_by: null
summary: >-
  Accepted 2026-11-09 for #126. ADR-0003 (opaque payloads) was accepted as provisional, to lift at phase 13
  "when happenstance-sync round-trips an event between two stores without deserialising its
  payload". Three named tests now do that, through two unlike real peers, on CI: a native SQLite
  store through a Durable Object over HTTP (workerd job, local and deployed), and through
  NeonSyncPeer over one-shot HTTP (live-neon), with Postgres alongside (live-postgres). Every
  payload, metadata and tag set comes back byte for byte, including empty, NUL and invalid UTF-8
  payloads, and no peer or store on the path names a codec. This record lifts the marker. It
  changes no code and supersedes nothing: ADR-0003's decision stands as written, and its body
  stays as accepted, because lint-kb forbids editing an accepted body.
depends_on:
  - kb-decision-0003
  - kb-decision-0097
related:
  - kb-decision-0104
  - kb-decision-0105
source_paths:
  - harness/workerd/tests/sqlite_round_trip/main.rs
  - harness/sync-round-trip/tests/round_trip/main.rs
  - harness/sync-round-trip/tests/round_trip/neon.rs
  - harness/sync-round-trip/tests/round_trip/postgres.rs
  - crates/happenstance-cloudflare/src/sync_http.rs
  - crates/happenstance-neon/src/event_store/sync_peer.rs
last_reviewed: 2026-11-08
---

# ADR-0003 is no longer provisional, because two unlike peers round-trip a payload byte for byte

## Context

ADR-0003 put payloads in the contract as opaque `Bytes` and kept `serde` out of
`happenstance-core`'s default features. It was written before any code it constrains, so it was
accepted as **provisional**, and its own status section named the lift: phase 13, when
`happenstance-sync` round-trips an event between two stores without deserialising its payload.
The phase 13 runbook carries the same exit criterion (`runbook/phases/13-sync.md`, #126).

An accepted atom's body cannot change (`cargo xtask lint-kb`), so the marker cannot be edited out
of ADR-0003. A separate record lifts it.

## Evidence

Each test below runs on CI against a real network peer. Each compares every payload, metadata
value and tag set byte for byte, in order and in both directions. Each also has a replay
companion showing that a second delivery appends nothing.

| Peer | Test | CI job |
| --- | --- | --- |
| Durable Object over HTTP (#130) | `sqlite_round_trips_through_the_object_byte_identical`, `harness/workerd/tests/sqlite_round_trip/main.rs` | `workerd`, local and deployed |
| `NeonSyncPeer`, one SQL request per call (#131) | `sqlite_through_the_neon_peer_is_byte_identical_both_ways`, `harness/sync-round-trip/tests/round_trip/neon.rs` | `live-neon` |
| `StoreSyncPeer` over Postgres (#131) | `sqlite_through_the_postgres_peer_is_byte_identical_both_ways`, `harness/sync-round-trip/tests/round_trip/postgres.rs` | `live-postgres` |

The test inputs include an empty payload, a NUL byte, invalid UTF-8, every byte value, metadata
and tags. Nothing on any path decodes them:

- The peers move `Event` envelopes over postcard (ADR-0097), whose payload field is `Bytes`.
- The stores bind and read `bytea` and `BLOB` columns.
- No crate on the path takes `happenstance`, the typed layer, as a normal dependency. Postgres
  takes it only as a dev-dependency, for its projection tests.

The Durable Object and Neon are deliberately unlike: one is a `!Send` SQLite object behind a
socket, and the other has no connection, no interactive transaction and no cursor. So the result
does not rest on one storage shape.

## Decision

ADR-0003 is no longer provisional. It is precedent: work that contradicts it owes a superseding
decision and must reckon with the code that now relies on it.

## Consequences

- ADR-0003's decision and body are unchanged. The `provisional` word in its summary and status
  section is a historical statement from now on, and readers should follow this record's
  pointer. When this record is accepted, ADR-0003's `related` key gains `kb-decision-0106` (a
  change lint-kb permits).
- When this record is accepted, the runbook's phase 13 exit line for #126 is ticked.
- Nothing is built or changed in code.

## Alternatives rejected

- **Superseding ADR-0003.** Supersession marks the decision replaced, but it is confirmed, not
  replaced. Readers would be sent to a successor that says the same thing.
- **Editing ADR-0003's status section.** lint-kb fails any change to an accepted body, and it is
  right to: the record of what was believed in August is worth keeping.
