# Sync a device to a central store

> **Answers:** `how-to` — How do I keep a device's event store in sync with a central one?

This page is for you if each device in your system keeps its own event log, so
it can take decisions while it has no signal, and a central store collects
what every device wrote. By the end you will have a device that syncs through
a transport you control, a failed sync that loses nothing, a sync position that
survives a restart, and a plan for the case where a device and central both
decided something the other did not see.

Everything here comes from
[`happenstance-sync`](../crates/happenstance-sync/README.md)'s README and the
documentation of the items named. The programs run in memory, on
`MemoryIngestStore` from that crate's `memory` feature. The lines that differ
on a real database are in the last sections, in fences marked as not compiled
here.

## The shape

One relationship between two stores is an **edge**. A device has one edge to
central, and three things make it up:

- **On the device:** its own store, a `runner::Edge` that remembers how far the
  two sides got, and a `RemoteSyncPeer` that stands for central.
- **On central:** a `StoreSyncPeer` over central's store, and `serve_sync`,
  which turns one request's bytes into one reply's bytes.
- **Between them:** an `Exchange`, which is yours. It carries the bytes of one
  request to `serve_sync` and brings the reply back, over HTTP or whatever your
  devices speak.

Each step of the edge pulls what central holds and ingests it, then pushes what
the device holds. A device pushes its whole log, the events it pulled from
central included, and central skips what it already holds
([SY-11](../spec/SPECIFICATION.md#53-identity-and-idempotent-ingest)). The port
and the runner ask for no `Send`
([SY-17](../spec/SPECIFICATION.md#54-the-transport-floor)), so a single-threaded
device runtime is a valid home for all of it.

## Write the exchange, and step the edge

The program below is a whole edge. `Uplink` is the exchange: here it calls
`serve_sync` directly, where yours would send the request over the network.
Its error states a failure class for each way it fails, and the runner acts on
the class it is given rather than guessing
([SY-36](../spec/SPECIFICATION.md#510-retention-across-a-peer-set-and-refusal)).
No signal is `Transient`: nothing was sent, and the same request will do later.

A real store's handle is cheap to clone, and `StoreSyncPeer` takes two: one to
read central's log, one to ingest into it. `MemoryIngestStore` cannot be
cloned, so the program shares it through a small wrapper, `Shared`, whose
forwarding code is hidden.

```rust
# use std::rc::Rc;
# use happenstance_core::{MemoryStoreError, SequencePosition, StoreId};
# use happenstance_sync::write_group::GroupOrderError;
# use happenstance_sync::{GroupPage, IngestError, IngestGroup, IngestStore, Ingested};
# use happenstance_sync::{Watermark as IngestWatermark, WriteGroupStore};
# #[derive(Clone, Default)]
# struct Shared(Rc<MemoryIngestStore>);
# impl std::ops::Deref for Shared {
#     type Target = MemoryIngestStore;
#     fn deref(&self) -> &MemoryIngestStore { &self.0 }
# }
# impl WriteGroupStore for Shared {
#     type Error = GroupOrderError;
#     async fn read_groups(&self, after: Option<SequencePosition>,
#         max_events: core::num::NonZeroUsize) -> Result<GroupPage, GroupOrderError> {
#         WriteGroupStore::read_groups(&*self.0, after, max_events).await
#     }
# }
# impl IngestStore for Shared {
#     type Error = MemoryStoreError;
#     async fn store_id(&self) -> Result<StoreId, MemoryStoreError> {
#         IngestStore::store_id(&*self.0).await
#     }
#     async fn ingest(&self, groups: &[IngestGroup<'_>])
#         -> Result<Ingested, IngestError<MemoryStoreError>> {
#         IngestStore::ingest(&*self.0, groups).await
#     }
#     async fn watermark(&self) -> Result<IngestWatermark, MemoryStoreError> {
#         IngestStore::watermark(&*self.0).await
#     }
# }
use std::cell::Cell;
use std::collections::BTreeSet;

use happenstance_core::{Event, EventStore, SequencedEvent};
use happenstance_sync::runner::Edge;
use happenstance_sync::wire::Route;
use happenstance_sync::{
    Classify, Exchange, FailureClass, MemoryIngestStore, RemoteSyncPeer, ServeError,
    StoreSyncPeer, SyncPeer, serve_sync,
};

/// The device's link to central, and whether it has signal.
struct Uplink {
    far: StoreSyncPeer<Shared, Shared>,
    signal: Cell<bool>,
}

#[derive(Debug, thiserror::Error)]
enum UplinkError {
    #[error("no signal: the request never left the device")]
    Offline,
    #[error(transparent)]
    Serve(#[from] ServeError<GroupOrderError, MemoryStoreError>),
}

impl Classify for UplinkError {
    fn class(&self) -> FailureClass {
        match self {
            Self::Offline => FailureClass::Transient,
            Self::Serve(error) => error.class(),
        }
    }
}

impl Exchange for Uplink {
    type Error = UplinkError;

    async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, UplinkError> {
        if !self.signal.get() {
            return Err(UplinkError::Offline);
        }
        Ok(serve_sync(&self.far, route, &request).await?)
    }
}

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let central = Shared::default();
let far = StoreSyncPeer::new(central.clone(), central.clone());
let limits = far.limits();
let signal = Cell::new(true);
let peer = RemoteSyncPeer::new(Uplink { far, signal }, limits);

let tablet = MemoryIngestStore::new();
central.append(&[Event::new("ShowScheduled", &b"{}"[..])?], None).await?;
tablet.append(&[Event::new("SeatSold", &b"{}"[..])?], None).await?;

let mut edge = Edge::new();
edge.step(&peer, &tablet).await?;

let ids = |log: Vec<SequencedEvent>| -> BTreeSet<_> { log.iter().map(|e| e.id).collect() };
assert_eq!(ids(tablet.snapshot()), ids(central.snapshot()));
assert_eq!(central.snapshot().len(), 2);
assert!(edge.status().is_synced());
# Ok(())
# }
```

The two logs are compared by `EventId`, the identity the writing store gave
each event. Positions are local to each store, so the same event sits at a
different position on each side.

Build the edge once per device and keep it for the life of the relationship.
It holds the token central handed it, so stepping it against a different peer
would hand that peer a token it never minted.

## When the signal drops

A step that fails is reported with the phase it failed in and its class, and
the edge decides what to do next from the class. A `Transient` failure leaves
the edge running and loses nothing: the events it did not push are pushed by
the next step, and a refused push leaves the device's log as it was
([SY-34](../spec/SPECIFICATION.md#510-retention-across-a-peer-set-and-refusal)).

```rust
# use std::rc::Rc;
# use std::cell::Cell;
# use happenstance_core::{Event, EventId, EventStore, MemoryStoreError, SequencePosition, StoreId};
# use happenstance_sync::write_group::GroupOrderError;
# use happenstance_sync::wire::Route;
# use happenstance_sync::{GroupPage, IngestError, IngestGroup, IngestStore, Ingested};
# use happenstance_sync::{Watermark as IngestWatermark, WriteGroupStore};
# use happenstance_sync::{Classify, Exchange, MemoryIngestStore, RemoteSyncPeer, ServeError};
# use happenstance_sync::{StoreSyncPeer, SyncPeer, serve_sync};
# #[derive(Clone, Default)]
# struct Shared(Rc<MemoryIngestStore>);
# impl std::ops::Deref for Shared {
#     type Target = MemoryIngestStore;
#     fn deref(&self) -> &MemoryIngestStore { &self.0 }
# }
# impl WriteGroupStore for Shared {
#     type Error = GroupOrderError;
#     async fn read_groups(&self, after: Option<SequencePosition>,
#         max_events: core::num::NonZeroUsize) -> Result<GroupPage, GroupOrderError> {
#         WriteGroupStore::read_groups(&*self.0, after, max_events).await
#     }
# }
# impl IngestStore for Shared {
#     type Error = MemoryStoreError;
#     async fn store_id(&self) -> Result<StoreId, MemoryStoreError> {
#         IngestStore::store_id(&*self.0).await
#     }
#     async fn ingest(&self, groups: &[IngestGroup<'_>])
#         -> Result<Ingested, IngestError<MemoryStoreError>> {
#         IngestStore::ingest(&*self.0, groups).await
#     }
#     async fn watermark(&self) -> Result<IngestWatermark, MemoryStoreError> {
#         IngestStore::watermark(&*self.0).await
#     }
# }
# struct Uplink { far: StoreSyncPeer<Shared, Shared>, signal: Cell<bool> }
# #[derive(Debug, thiserror::Error)]
# enum UplinkError {
#     #[error("no signal")]
#     Offline,
#     #[error(transparent)]
#     Serve(#[from] ServeError<GroupOrderError, MemoryStoreError>),
# }
# impl Classify for UplinkError {
#     fn class(&self) -> FailureClass {
#         match self { Self::Offline => FailureClass::Transient, Self::Serve(e) => e.class() }
#     }
# }
# impl Exchange for Uplink {
#     type Error = UplinkError;
#     async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, UplinkError> {
#         if !self.signal.get() { return Err(UplinkError::Offline); }
#         Ok(serve_sync(&self.far, route, &request).await?)
#     }
# }
use happenstance_sync::FailureClass;
use happenstance_sync::runner::{Edge, EdgeState};

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let central = Shared::default();
# let far = StoreSyncPeer::new(central.clone(), central.clone());
# let limits = far.limits();
# let peer = RemoteSyncPeer::new(Uplink { far, signal: Cell::new(true) }, limits);
# let tablet = MemoryIngestStore::new();
let mut edge = Edge::new();
peer.exchange().signal.set(false);
let sold = tablet.append(&[Event::new("SeatSold", &b"{}"[..])?], None).await?;

let failed = edge.step(&peer, &tablet).await;
let failure = failed.err().ok_or("a step with no signal fails")?.failure();
assert_eq!(failure.class, FailureClass::Transient);
assert_eq!(edge.status().state, EdgeState::Running);
assert!(central.snapshot().is_empty());

peer.exchange().signal.set(true);
edge.step(&peer, &tablet).await?;
let landed: Vec<EventId> = central.snapshot().iter().map(|e| e.id).collect();
assert_eq!(landed, [EventId::new(tablet.store_id(), sold)]);
# Ok(())
# }
```

The loop that keeps stepping is yours to choose. `Edge::run` steps until the
edge is synced, waiting between attempts by a backoff you supply. A device
that syncs when the user asks, or when the network comes back, calls `step`
itself.

## Keep the sync position across a restart

The edge's **watermark** says which of the device's events central has
confirmed. It is not an event, since no peer should replicate it. It is kept
through the device's projection store, in a batch of yours, so it lands in the
same commit as whatever read model that batch carries
([SY-31](../spec/SPECIFICATION.md#59-where-a-per-peer-watermark-lives)).

```rust
# use std::rc::Rc;
# use std::cell::Cell;
# use happenstance_core::{Event, EventStore, MemoryStoreError, SequencePosition, StoreId};
# use happenstance_sync::write_group::GroupOrderError;
# use happenstance_sync::wire::Route;
# use happenstance_sync::{GroupPage, IngestError, IngestGroup, IngestStore, Ingested};
# use happenstance_sync::{Watermark as IngestWatermark, WriteGroupStore};
# use happenstance_sync::{Classify, Exchange, FailureClass, MemoryIngestStore};
# use happenstance_sync::{RemoteSyncPeer, ServeError, StoreSyncPeer, SyncPeer, serve_sync};
# #[derive(Clone, Default)]
# struct Shared(Rc<MemoryIngestStore>);
# impl std::ops::Deref for Shared {
#     type Target = MemoryIngestStore;
#     fn deref(&self) -> &MemoryIngestStore { &self.0 }
# }
# impl WriteGroupStore for Shared {
#     type Error = GroupOrderError;
#     async fn read_groups(&self, after: Option<SequencePosition>,
#         max_events: core::num::NonZeroUsize) -> Result<GroupPage, GroupOrderError> {
#         WriteGroupStore::read_groups(&*self.0, after, max_events).await
#     }
# }
# impl IngestStore for Shared {
#     type Error = MemoryStoreError;
#     async fn store_id(&self) -> Result<StoreId, MemoryStoreError> {
#         IngestStore::store_id(&*self.0).await
#     }
#     async fn ingest(&self, groups: &[IngestGroup<'_>])
#         -> Result<Ingested, IngestError<MemoryStoreError>> {
#         IngestStore::ingest(&*self.0, groups).await
#     }
#     async fn watermark(&self) -> Result<IngestWatermark, MemoryStoreError> {
#         IngestStore::watermark(&*self.0).await
#     }
# }
# struct Uplink { far: StoreSyncPeer<Shared, Shared>, signal: Cell<bool> }
# #[derive(Debug, thiserror::Error)]
# enum UplinkError {
#     #[error("no signal")]
#     Offline,
#     #[error(transparent)]
#     Serve(#[from] ServeError<GroupOrderError, MemoryStoreError>),
# }
# impl Classify for UplinkError {
#     fn class(&self) -> FailureClass {
#         match self { Self::Offline => FailureClass::Transient, Self::Serve(e) => e.class() }
#     }
# }
# impl Exchange for Uplink {
#     type Error = UplinkError;
#     async fn exchange(&self, route: Route, request: Vec<u8>) -> Result<Vec<u8>, UplinkError> {
#         if !self.signal.get() { return Err(UplinkError::Offline); }
#         Ok(serve_sync(&self.far, route, &request).await?)
#     }
# }
use happenstance_core::MemoryProjectionStore;
use happenstance_sync::runner::{Edge, Watermark};

# #[tokio::main(flavor = "current_thread")]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
# let central = Shared::default();
# let far = StoreSyncPeer::new(central.clone(), central.clone());
# let limits = far.limits();
# let peer = RemoteSyncPeer::new(Uplink { far, signal: Cell::new(true) }, limits);
# let tablet = MemoryIngestStore::new();
# tablet.append(&[Event::new("SeatSold", &b"{}"[..])?], None).await?;
let models = MemoryProjectionStore::new();
let mut edge = Edge::new();
edge.step(&peer, &tablet).await?;

let kept = edge.watermark().ok_or("central confirmed a push and named itself")?;
let batch = models.open();
kept.commit(&models, batch).await?;

// The device restarts: the edge is rebuilt from what was kept.
let loaded = Watermark::load(&models, kept.peer()).await?;
let mut edge = Edge::restored(loaded.ok_or("kept above")?);
let stepped = edge.step(&peer, &tablet).await?;
assert_eq!(stepped.pushed, 0, "nothing above the watermark to push");
assert_eq!(stepped.ingested.appended, 0, "a re-pull lands nothing twice");
# Ok(())
# }
```

A restored edge pulls from the start of central's log, because the pull
token is not kept. That costs a re-read and nothing more: ingest skips what
the device already holds.

## On SQLite

`SqliteEventStore` is both halves a sync edge needs once its `sync` feature
is on, and a clone of it is a second handle onto the same file. Neither line
below is compiled on this page:

```text
happenstance-sqlite = { version = "0.7", features = ["sync"] }
happenstance-sync = "0.7"
```

```text
let central = SqliteEventStore::open("central.db")?;
let far = StoreSyncPeer::new(central.clone(), central.clone());

let tablet = SqliteEventStore::open("tablet.db")?;
edge.step(&peer, &tablet).await?;
```

For a Postgres central, `PostgresEventStore` takes `SqliteEventStore`'s place
behind `StoreSyncPeer` and `serve_sync`, with the `sync` feature of
`happenstance-postgres`. Neon is a different shape: `NeonSyncPeer` is the peer
itself, so the device talks to Neon over one-shot HTTP with no server between,
and takes it in place of `RemoteSyncPeer`.

## When both sides wrote

Two devices offline can each sell the last seat, and each decision is right
about the history that device held. When they sync, central ingests both:
ingest never refuses an event for a domain reason
([SY-1](../spec/SPECIFICATION.md#51-the-bound-decision-ingest-never-rejects)).
The conflict is answered afterwards, by an ordinary append that compensates
([SY-2](../spec/SPECIFICATION.md#51-the-bound-decision-ingest-never-rejects)),
written by one peer only
([SY-7](../spec/SPECIFICATION.md#51-the-bound-decision-ingest-never-rejects)).

`StoreSyncPeer` writes no compensation unless you give it a merge rule. From
`0.7` the runner has a hook for one
([#450](https://github.com/Wet-Ink-Corporation/happenstance/issues/450)): a
`runner::Adjudicate`, mounted with `Edge::with_adjudicator` for what an edge
pulls, or with `StoreSyncPeer::with_adjudicator`, served through
`serve_sync_adjudicated`, for what a far side is pushed. For central, that is
the second: mount the rule on central's peer, and leave each device's edge on
the default, which judges nothing, so central is the one peer that writes a
compensation. The rule is asked about each pushed group before the push
lands, reads what it needs from its own handle onto central, and returns the
compensation; it writes nothing itself. The runner lands each group and its
compensation in one write, and the compensation is an event, so it syncs
back to every device.

```text
let far = StoreSyncPeer::new(central.clone(), central.clone())
    .with_adjudicator(CentralRule::new(central.clone(), show));

serve_sync_adjudicated(&far, route, &request).await?
```

[`examples/two-devices-one-seat`](../examples/two-devices-one-seat/src/overview.md)
is that arrangement end to end on three SQLite files: both devices sell the
last seat, central's rule refunds the sale that reached it last as that sale
lands, and the three files converge on the same log and the same read model.
