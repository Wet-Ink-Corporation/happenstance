# ADR-0003: Opaque payloads in the contract crate

- **Status:** accepted — **provisional**
- **Date:** 2026-08-05

> **Provisional.** Authored on 2026-08-05 alongside the initial scaffold, before
> any of the code this decision constrains existed. The payoff it claims —
> that a peer forwards events without deserialising them — has never been
> exercised, because no replication code exists. So this is a recorded
> intention, not settled precedent: work that contradicts it still needs a
> superseding ADR, but it does not owe deference to a decision the code has not
> yet voted on.
>
> **Lifts when** `happenstance-sync` round-trips an event between two stores
> without deserialising its payload
> ([phase 13](../../RUNBOOK.md#phase-13--happenstance-sync-and-its-testkit)).

## Context

The DCB specification says an event carries "Event Data" and says nothing about
its format — deliberately. But a Rust library has to choose a representation,
and the choice propagates into every adapter signature.

## Decision

`Event::data` is [`bytes::Bytes`](https://docs.rs/bytes). `happenstance-core`
has no `serde` dependency in its default feature set, and never parses a
payload.

A `serde` feature exists, off by default, adding `Serialize`/`Deserialize` for
the *envelope* types (`Event`, `Tag`, `Query`, `AppendCondition`, …). It exists
for `happenstance-sync`, which has to put an envelope on the wire. It does not make
the payload any less opaque.

Encoding and decoding — a `Codec` trait, a `DomainEvent` mapping — belong to
`happenstance`, the layer above.

## Consequences

**Good.** Adapters carry no domain knowledge; a store is a store. Replication
forwards events **byte-for-byte** without deserialising them, so a peer cannot
fail to parse a payload it does not understand, and cannot corrupt one by
re-encoding it. Binary codecs stay possible. The contract crate's dependency
graph stays tiny, which matters because its semver surface must be the most
stable in the workspace.

**Good.** `Bytes` clones are refcount bumps, so fanning an event out to several
projections copies nothing.

**Bad.** Application code cannot pattern-match a domain event straight out of
the store; something must decode it first. Until `happenstance`'s typed layer
exists, that is the application's job — visible in the `course-subscriptions`
example, which hand-rolls a payload parse and says so.

## Alternatives rejected

- **Generic over a payload type `E`** — ergonomic for one application, but it
  infects every adapter signature, makes `dyn` storage impossible, and would
  force the replication adapter to be compiled against the sender's domain
  types. Fatal for a storage-agnostic library.
- **`serde_json::Value` in the core** — easy to debug, but it locks the whole
  ecosystem to JSON, forces a parse on every read even for pass-through
  replication, and forecloses binary codecs.
