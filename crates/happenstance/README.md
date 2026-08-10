# happenstance

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

> **Status: early, and this crate is currently a facade.** It re-exports
> [`happenstance-core`](https://crates.io/crates/happenstance-core) and adds
> nothing yet. It is published anyway so that `cargo add happenstance` is true
> throughout and the name never has to move once anyone depends on it. The typed
> layer — codecs, domain events, decision models, the command loop, the
> projection runner — arrives here.

## Which crate do I want?

- **Writing an application?** This one.
- **Writing a storage adapter?** Depend on
  [`happenstance-core`](https://crates.io/crates/happenstance-core) instead. It is
  the smaller semver surface, and it is what
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit) measures
  you against.

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
  [ADR-0029](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decision/0029-msrv-raised-to-1-97-1.md)
  records the measurement and the trade.

## Design

The design is specified rather than described:
[`docs/architecture/SPECIFICATION.md`](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/docs/architecture/SPECIFICATION.md)
carries the normative clauses, each with a maturity marker, the conformance rule
that checks it, and the wrong implementation it forbids.

## Licence

MIT OR Apache-2.0.
