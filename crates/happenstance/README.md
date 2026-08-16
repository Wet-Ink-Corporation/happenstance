# happenstance

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

## Which crate do I want?

- **Writing an application?** This one.
- **Writing a storage adapter?** Depend on
  [`happenstance-core`](https://crates.io/crates/happenstance-core) instead. It is
  the smaller semver surface, and it is what
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit) measures
  you against.

## Stability

- **The API moves until the first stable `0.2.0`** — expect a small edit at each
  upgrade, and pin the exact version you built against.
- **What changed is in [`CHANGELOG.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/CHANGELOG.md)**, per release, in a caller's terms.
- **Only one alpha resolves at a time:** each is yanked when the next lands.

## What DCB buys you

There are no aggregates. An event carries a type, opaque bytes and a set of tags;
a query selects across them; a command reads what it needs, folds it, decides, and
appends conditioned on nothing new having appeared. The consistency boundary is
whatever that query selects — chosen per decision, and free to span what
aggregates would have separated without reaching for a saga.

```rust
use happenstance::{EventStore, MemoryEventStore, Query, ReadOptions, collect};

async fn count_everything() -> Result<(), Box<dyn std::error::Error>> {
    let store = MemoryEventStore::new();
    let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    assert!(events.is_empty());
    Ok(())
}
```

## Guarantees

- `#![forbid(unsafe_code)]`, workspace-wide.
- Every feature this crate has is forwarded from `happenstance-core`, so the two
  cannot disagree about what `default-features = false` means.
- MSRV 1.97.1, checked in CI. Raised from 1.85 at phase 2 by a *dependency's*
  build script rather than by this crate's own code —
  [ADR-0029](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0029-msrv-raised-to-1-97-1.md)
  records the measurement and the trade.

## Design

The design is specified rather than described:
[`spec/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
carries the normative clauses, each with a maturity marker, the conformance rule
that checks it, and the wrong implementation it forbids.

## Licence

MIT OR Apache-2.0.
