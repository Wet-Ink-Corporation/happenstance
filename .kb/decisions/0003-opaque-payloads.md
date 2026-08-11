---
id: kb-decision-0003
title: Opaque payloads in the contract crate
kind: decision
status: accepted
authority_tier: decision
adr_id: ADR-0003
reversibility: low
phase: 0
supersedes: null
superseded_by: null
summary: >-
  Provisional, and lifts at phase 13 when happenstance-sync round-trips an event between two
  stores without deserialising its payload. Event::data is bytes::Bytes; happenstance-core carries
  no serde in its default features and never parses a payload; an off-by-default serde feature
  adds Serialize and Deserialize for the envelope types only, so replication can put an envelope
  on the wire without making the payload any less opaque. Encoding and decoding — a Codec, a
  DomainEvent mapping — belong to happenstance, the layer above. Consequences: adapters carry no
  domain knowledge, a peer forwards bytes it cannot parse and therefore cannot corrupt, binary
  codecs stay possible, the contract crate's dependency graph and semver surface stay small, and
  Bytes clones are refcount bumps. The cost is that application code cannot pattern-match a domain
  event straight out of the store. Rejected: a generic payload parameter, which infects every
  adapter signature and blocks dyn storage, and serde_json::Value in core, which locks the
  ecosystem to JSON and forces a parse on every pass-through read.
depends_on: []
related: []
source_paths:
  - .kb/_intake/0003-opaque-payloads.md
  - docs/adr/0003-opaque-payloads.md
  - crates/happenstance-core/src/event.rs
  - crates/happenstance-core/Cargo.toml
  - CLAUDE.md
last_reviewed: 2026-08-10
---

# Opaque payloads in the contract crate

## Context

The DCB specification says an event carries "Event Data" and deliberately says nothing about its
format. A Rust library still has to pick a concrete representation, and whatever is chosen
propagates into every adapter's method signatures — so the choice had to be made once, early, and
in the crate everything else depends on.

## Decision

`Event::data` is [`bytes::Bytes`](https://docs.rs/bytes). The contract crate (`happenstance-core`
after [`kb-decision-0006`](0006-bare-name-to-the-typed-layer.md)) carries no `serde` dependency in
its default feature set, and never parses a payload anywhere in its own code.

A `serde` feature exists, off by default, adding `Serialize`/`Deserialize` for the *envelope*
types only — `Event`, `Tag`, `Query`, `AppendCondition`, and similar structural types, never the
payload bytes themselves. It exists for `happenstance-sync`, which has to put an envelope on the
wire; enabling it does not make the payload any less opaque, because the bytes inside `data` are
never touched by the derive.

Encoding and decoding — a `Codec` trait, a `DomainEvent` mapping — belong to `happenstance`, the
typed layer above the contract, not to the contract itself.

## Consequences

**Good.** Adapters carry no domain knowledge; a store is a store regardless of what is stored in
it. Replication forwards events byte-for-byte without deserialising them, so a peer cannot fail to
parse a payload it does not understand and cannot corrupt one by re-encoding it. Binary codecs
stay possible alongside JSON ones. The contract crate's dependency graph stays small, which
matters because its semver surface has to be the most stable one in the workspace — every adapter
and every application ultimately depends on it.

**Good.** `Bytes` clones are refcount bumps rather than copies, so fanning one event out to several
projections costs nothing extra in allocation.

**Bad.** Application code cannot pattern-match a domain event straight out of the store; something
has to decode it first. Until `happenstance`'s typed layer exists, that decoding is the
application's own job — visible in the `course-subscriptions` example, which hand-rolls a payload
parse and says so in its own comments.

## Alternatives rejected

- **Generic over a payload type `E`.** Ergonomic for a single application, but it infects every
  adapter signature with a type parameter, makes `dyn` storage of the store impossible, and would
  force a replication adapter to compile against the sender's own domain types — fatal for a
  storage-agnostic library that has to work for applications it has never seen.
- **`serde_json::Value` in the contract crate.** Easy to debug by inspection, but it locks the
  whole ecosystem to JSON, forces a parse on every read even for pass-through replication that
  never needed to look inside the payload, and forecloses binary codecs entirely.

## Status

Accepted and provisional: authored on 2026-08-05 alongside the initial scaffold, before any of the
code this decision constrains existed. The payoff it claims — that a peer forwards events without
deserialising them — has not yet been exercised, because no replication code exists yet. It lifts
at phase 13, when `happenstance-sync` round-trips an event between two stores without deserialising
its payload. Work that contradicts this decision before then still needs a superseding decision
atom rather than a quiet rewrite; it just does not yet owe deference to precedent the code has not
voted on.
