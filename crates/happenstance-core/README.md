# happenstance-core

The contract for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance):
value types, the storage ports, the error taxonomy, and an in-memory reference
event store. Storage-agnostic, and built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

> **Status: early.** The contract and its conformance suite are real and tested;
> every storage adapter is a documented stub. The port is not frozen — see
> [the specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/architecture/SPECIFICATION.md),
> where every clause carries a maturity marker and, where it is not settled, the
> experiment that would settle it.

## Which crate do I want?

- **Writing an application?** Use [`happenstance`](https://crates.io/crates/happenstance).
  It re-exports everything here and is where the typed layer — codecs, domain
  events, decision models — arrives.
- **Writing a storage adapter?** Depend on this crate. It is the smaller semver
  surface, and it is what
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit)
  measures you against.

## What it gives you

DCB has no aggregates. An event carries a type, opaque bytes, and a set of tags;
a query selects across them; and a command reads what it needs, decides, and
appends conditioned on nothing new having appeared. The consistency boundary is
whatever that query selects — per decision, and free to span what aggregates
would have separated.

```rust
use happenstance_core::{EventStore, MemoryEventStore, Query, ReadOptions, collect};

# async fn example() -> Result<(), Box<dyn core::error::Error>> {
let store = MemoryEventStore::new();
let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
assert!(events.is_empty());
# Ok(())
# }
```

## Guarantees

- `#![forbid(unsafe_code)]`, workspace-wide and verified opted into by every
  member.
- No `serde` on any default path. Payloads are opaque `Bytes`, which is what lets
  a replication peer forward an event without ever parsing it.
- `no_std` + `alloc` supported; `std` and `memory` are on by default and the
  `memory` feature is fully dead-code-eliminated when unused.
- MSRV 1.85, checked in CI.

## An adapter is not an adapter until it passes the suite

```rust,ignore
happenstance_testkit::event_store_conformance!(MyStore::new());
```

## Licence

MIT OR Apache-2.0.
