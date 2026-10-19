# happenstance-core

The contract for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance):
value types, the storage ports, the error taxonomy, and an in-memory reference
event store. Storage-agnostic, and built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

New to happenstance? [The happenstance guide](https://wet-ink-corporation.github.io/happenstance/)
explains event sourcing and dynamic consistency boundaries from the beginning,
with programs you can run.

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: `0.4.3`.** The contract and its 96-rule event-store conformance suite
> are real and tested, and **four adapters have cleared a suite against storage
> this workspace does not fully control** — a file on disk, a Durable Object on
> `wasm32`, a pooled PostgreSQL server, and a one-shot HTTP proxy with no
> connection to hold.
>
> **Two different maturity claims live here and they are not the same claim.** The
> `EventStore` clauses marked `[FROZEN]` in the specification are **semver-binding
> from `0.2.0`**. `ProjectionStore` was **not**, at `0.2.0`: it shipped behind
> the off-by-default `unstable-projection` feature with a written semver
> exemption. Its freeze condition was met by ADR-0062 and ADR-0063 lifted the
> gate, so it is **frozen and unconditional from `0.3.0`**. The feature name
> survived, empty, until `0.4.0` removed it.
>
> See [the specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md),
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

async fn count_everything() -> Result<(), Box<dyn std::error::Error>> {
    let store = MemoryEventStore::new();
    let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    assert!(events.is_empty());
    Ok(())
}
```

## Guarantees

- `#![forbid(unsafe_code)]`, workspace-wide and verified opted into by every
  member.
- No `serde` on any default path. Payloads are opaque `Bytes`, which is what lets
  a replication peer forward an event without ever parsing it.
- `no_std` + `alloc` supported; `std` and `memory` are on by default and the
  `memory` feature is fully dead-code-eliminated when unused.
- MSRV 1.97.1, raised from 1.85 at phase 2 by a *dependency's* build script
  rather than by this crate's own code. What bounds the next rise is under
  [*Minimum supported Rust version*](#minimum-supported-rust-version).

## An adapter is not an adapter until it passes the suite

```rust,ignore
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

The expression builds a *fixture* — one isolated backing store per instance, one
handle per `connect()` — not a store directly. See `happenstance-testkit`.

## Minimum supported Rust version

Rust **1.97.1**, the `rust-version` this crate declares
([ADR-0029](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0029-msrv-raised-to-1-97-1.md)).
After `1.0.0` the floor rises only in a minor release, never a patch; only to a
Rust stable at least six months old on the day that minor ships; and always with
a `CHANGELOG.md` entry naming the old floor, the new one and what forced it
([ADR-0067](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0067-msrv-after-1-0-rises-are-bounded.md)).

Before `1.0.0`, a rise ships in a `0.x` minor, which a `"0.x"` requirement never
resolves across
([ADR-0037](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0037-msrv-becomes-a-promise-at-0-2-0.md)).
From `1.0.0`, what keeps a rise from reaching you is your workspace's resolver,
not this crate's: under `resolver = "3"`, the default for an edition-2024 root,
or with `resolver.incompatible-rust-versions = "fallback"` set, Cargo prefers
the newest version whose `rust-version` your lowest `package.rust-version` (or
your `rustc`, if you declare none) satisfies, and `cargo update` says so by
printing *to latest Rust 1.xx compatible versions* where resolver 2 prints *to
latest compatible versions*. On resolver 2, only your lockfile holds you back.

## Licence

MIT OR Apache-2.0.
