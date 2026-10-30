# happenstance-sync

The replication port for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance):
the `SyncPeer` and `IngestStore` traits, the wire messages that travel between
two stores, and an in-memory reference peer and ingest store to test against.

New to happenstance? [The happenstance guide](https://wet-ink-corporation.github.io/happenstance/)
explains event sourcing and dynamic consistency boundaries from the beginning,
with programs you can run.

> **Status: published, and unfinished.** This crate is published so that a
> store adapter can implement `IngestStore` behind an optional `sync` feature,
> and so that an outside adapter can be measured by
> [`happenstance-sync-testkit`](https://docs.rs/happenstance-sync-testkit).
> Two things a replicating application needs do not exist yet:
>
> - **a runner**, the loop that pulls from a peer, ingests, and acknowledges
>   (issue #244), and
> - **a network peer**: the only `SyncPeer` is `MemorySyncPeer`, which lives in
>   one process.
>
> A `[FROZEN]` `SY` clause of the
> [specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
> binds this crate's semver as an `ES` clause binds `happenstance-core`'s.
> SY-7, SY-10, SY-20 – SY-23, SY-27 – SY-31 and SY-36 – SY-38 are
> `[PROVISIONAL]`, binding until the falsifier each names, and SY-14, SY-18 and
> SY-32 are `[DEFERRED]`. The version moves in lockstep with the rest of the
> workspace.

## Which stores implement the port

Each adapter implements `IngestStore` behind its own `sync` feature, which is
off by default:

| Adapter | Flavour |
|---|---|
| `happenstance-sqlite` | `SendIngestStore` |
| `happenstance-postgres` | `SendIngestStore` |
| `happenstance-neon` | `IngestStore` |
| `happenstance-cloudflare` | `IngestStore`, through `CloudflareIngestStore` |

Bind `IngestStore` in generic code, as you bind `EventStore`: it is the weaker
requirement and accepts both flavours.

## Use

Code that lands a pulled batch is written once against the port, and runs on
any store that implements it:

```rust
use happenstance_core::{Event, EventId, RecordedAt, SequencePosition, StoreId};
use happenstance_sync::{IngestError, IngestGroup, IngestStore, Ingested, ReplicatedEvent};

/// Lands one batch another store minted. Ingest never refuses an event for
/// what it says (SY-1); it skips one it already holds (SY-11).
async fn land<S: IngestStore>(
    store: &S,
    batch: &[ReplicatedEvent],
) -> Result<Ingested, IngestError<S::Error>> {
    store.ingest(&[IngestGroup::new(batch, &[])]).await
}

# #[cfg(feature = "memory")]
# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn core::error::Error>> {
let store = happenstance_sync::MemoryIngestStore::new();
let origin = StoreId::from_bytes([7; 16]);
let shipped = ReplicatedEvent::new(
    EventId::new(origin, SequencePosition::new(4).ok_or("position zero")?),
    RecordedAt::from_millis(1_700_000_000_000),
    Event::new("Shipped", &b"opaque"[..])?,
);

assert_eq!(land(&store, core::slice::from_ref(&shipped)).await?.appended, 1);
// Delivered twice, landed once.
assert_eq!(land(&store, core::slice::from_ref(&shipped)).await?.skipped, 1);
# Ok(())
# }
# #[cfg(not(feature = "memory"))]
# fn main() {}
```

## Features

- **`memory`**, off by default: `MemorySyncPeer` and `MemoryIngestStore`, the
  in-memory reference peer and ingest store, for tests and doctests. It turns
  on `happenstance-core/memory`. No deployment replicates to a peer in its own
  memory, so nothing else should need it.

Payloads are never decoded: a peer forwards the bytes it was given (SY-35), so
this crate needs none of your domain types.

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
