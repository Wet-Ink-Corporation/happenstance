# Upgrading from 0.4 to 0.5

> **Answers:** `how-to` — How do I move my code from 0.4 to 0.5?

This page is for you if your project depends on happenstance 0.4 and you want to
move to 0.5. By the end your code will build against 0.5 and behave as you
intend. Most of the changes are caught by the compiler. One compiles unchanged
and behaves differently, and this page points it out where it comes.

0.5 is a breaking release, and almost all of it is about the typed projection
runner. In 0.4 the runner sat behind `happenstance`'s `unstable-projection`
feature, which made no semver promise. In 0.5 that feature is gone, the runner
is in every build, and semver covers it like the rest of the crate. If you never
enabled `unstable-projection`, the first section is the only one about your
application code, and it is short.

## How to use this page

Work through the sections for the crates in your `Cargo.toml`, in the order
below. You can skip any section for a crate you do not use. Each change says
what broke, what to change, and why.

| If you use | Read |
| --- | --- |
| `happenstance` | [the feature](#the-unstable-projection-feature-is-gone-h1), and every section after it if you enabled that feature |
| `happenstance-sqlite` with `projection-store` | [`happenstance-sqlite`](#happenstance-sqlite) |
| `happenstance-testkit` or `happenstance-sync-testkit` (you wrote an adapter) | [the testkits](#the-testkits) |

Each heading ends with a bracketed id, such as **[H5]**. That is the row for
this change in the `0.5.0` trace table, at the end of the
[`CHANGELOG.md`](../CHANGELOG.md#050--2026-11-07) section for this release. The
table lists every breaking change with the decision behind it, if you want the
full reasoning.

About the code on this page: blocks tagged `rust` are compiled and run against
this release by the project's build, so they show the 0.5 spelling. Blocks
tagged `text` are not compiled. Each one says whether it is the old 0.4
spelling, which no longer builds, or a manifest.

## `happenstance`

### The `unstable-projection` feature is gone **[H1]**

**What changed.** `Projection`, `SendProjection`, `run_projection`, and every
type the runner uses are in every build of `happenstance`, whatever features you
choose. The feature that gated them has been removed, and Cargo refuses to
resolve a feature that does not exist. The decision is
[ADR-0066](../.kb/decisions/0066-what-1-0-promises.md) §5.

A 0.4 manifest that no longer resolves:

```text
happenstance = { version = "0.4", features = ["unstable-projection"] }
```

**What to change.** Delete `"unstable-projection"` from `happenstance`'s
`features`, or a `happenstance/unstable-projection` line from your own
`[features]` table. Drop `--features unstable-projection` from any script or CI
job that passes it to `cargo`. Nothing you use changes when it goes:

```text
happenstance = "0.5"
```

If you used `default-features = false` to keep the runner out of your build, it
is in it now. It adds no dependency: the one crate it named, `futures-core`, was
already in your graph through `happenstance-core`.

Every section below this one is about a change made while the feature still
gated the runner. If you never enabled it, skip to the next crate you use.

### `apply` is `async` and is handed a `Delivered` event **[H2]**

**What changed.** `Projection::apply` is an `async fn`, so a projection over a
live transaction can await a statement through its batch. It is handed a
`Delivered<Self::Event>`, which pairs the decoded event with the `EventId` it
was read under. The decision is
[ADR-0074](../.kb/decisions/0074-projection-apply-is-async.md).

In 0.4, not compiled:

```text
fn apply(&mut self, event: Seat, batch: &mut MemoryProjectionBatch)
    -> Result<(), MemoryProjectionStoreError>
```

**What to change.** Add `async` and change the event parameter's type. Read the
event with `event.into_event()`, or borrow it with `event.event()`. A projection
over a buffered batch never awaits anything, so its run behaves as it did. The
program in the next section is a whole 0.5 projection.

`Delivered` has no accessor for the local position the event arrived at, on
purpose: a replicated event arrives at a different position on every store
([SY-21](../spec/SPECIFICATION.md#55-convergence-and-the-merge-rule)).
If a row must mean the same on every store, key it on `event.id()`.

### A projection has its own error type **[H3]**

**What changed.** `Projection` has a new associated type, `Error`, and `apply`
returns `Result<(), Self::Error>`. The error must convert from the store's, so
`?` on a store call inside `apply` still compiles
([PS-28](../spec/SPECIFICATION.md#48-failure-policy)). `ProjectionError`, what
`run_projection` returns, gains a third type parameter for that error, with no
default.

**What to change.** If the store's error is your projection's only failure, add
one line, `type Error = <your store's error>;`. If you named
`ProjectionError<R, W>` in a signature, add the projection's error as a third
parameter. The program below is a 0.5 projection with both changes, and it
calls `apply` directly, as a unit test of your own would:

```rust
use happenstance::bytes::Bytes;
use happenstance::{Codec, CodecError, Delivered, DomainEvent, EventId, EventType};
use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
use happenstance::{MemoryProjectionStoreError, Projection, ProjectionId};
use happenstance::{ProjectionStore, SequencePosition, StoreId, Tags};

#[derive(serde::Serialize, serde::Deserialize)]
enum Seat {
    Taken,
}

const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");

impl DomainEvent for Seat {
    const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
    fn event_type(&self) -> EventType {
        SEAT_TAKEN
    }
    fn tags(&self) -> Tags {
        Tags::empty()
    }
    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }
    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

struct SeatsSold {
    id: ProjectionId,
    scope: Tags,
    sold: u64,
}

impl Projection for SeatsSold {
    type Event = Seat;
    type Store = MemoryProjectionStore;
    // New in 0.5. The store's error is this projection's only failure.
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    // `async`, and handed a `Delivered<Seat>` rather than a `Seat`.
    async fn apply(
        &mut self,
        event: Delivered<Seat>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Self::Error> {
        match event.into_event() {
            Seat::Taken => self.sold += 1,
        }
        batch.write("seats_sold", self.sold);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let mut projection = SeatsSold {
        id: ProjectionId::new("seats_sold")?,
        scope: Tags::empty(),
        sold: 0,
    };
    let views = MemoryProjectionStore::new();
    let mut batch = views.begin().await?;

    let id = EventId::new(StoreId::from_bytes([7; 16]), SequencePosition::FIRST);
    projection.apply(Delivered::new(id, Seat::Taken), &mut batch).await?;
    assert_eq!(projection.sold, 1);
    Ok(())
}
```

### `run_projection` takes a `Chunk` **[H4]**

**What changed.** The last argument of `run_projection`, how many events it
applies before each commit, is a `Chunk` rather than a bare `NonZeroUsize`. The
decision is [ADR-0070](../.kb/decisions/0070-the-runner-takes-a-named-chunk.md).

In 0.4, not compiled:

```text
run_projection(&events, &views, &mut projection, &Json, size).await?;
```

**What to change.** Wrap the size: pass `Chunk::of(size)`. A `Chunk` cannot be
empty, and it has no `Default`, so a chunk size is always one you chose:

```rust
use std::num::NonZeroUsize;

use happenstance::Chunk;

fn chunk_of(size: usize) -> Option<Chunk> {
    NonZeroUsize::new(size).map(Chunk::of)
}

fn main() {
    assert!(chunk_of(64).is_some());
    assert!(chunk_of(0).is_none(), "an empty chunk cannot be written");
}
```

### The runner keys its checkpoint on a derived id **[H5]**

**What changed.** This one compiles unchanged, so check for it by hand. The
runner now reads and commits a projection's checkpoint under
`checkpoint_id(&projection)`, not under `projection.id()`. The derived id is the
name, `@`, and a digest of the query the runner reads with, so a projection
whose query changes starts a new checkpoint and replays from the start
([PS-25](../spec/SPECIFICATION.md#47-checkpoint-semantics)). In 0.4 it resumed
past events its old query never selected.

The consequence for an upgrade: **a checkpoint the 0.4 runner wrote reads
`NeverRun` under 0.5, so the projection rebuilds from the start of the log.**
That is correct, and for a small log it may be all you need.

A name longer than 238 bytes is now refused, with
`ProjectionError::CheckpointId`, because the name and its digest must fit in a
`ProjectionId`.

**What to change.** To keep a view's position instead of rebuilding it, adopt
the old checkpoint. Do it **only if the projection's event types and scope are
the same as in 0.4, and before the new runner first runs.** Adopting across a
changed query brings back the wrong resume this change exists to prevent. The
`adopt` function below copies the old checkpoint to the derived id, under the
same authority:

```rust
use happenstance::bytes::Bytes;
use happenstance::{Authority, Checkpoint, Codec, CodecError, Delivered};
use happenstance::{DomainEvent, Event, EventStore, EventType, MemoryEventStore};
use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
use happenstance::{MemoryProjectionStoreError, Projection, ProjectionId};
use happenstance::{ProjectionStore, Tags, checkpoint_id};

type Failure = Box<dyn core::error::Error>;

/// Copies a checkpoint the 0.4 runner wrote under `projection.id()` to the id
/// the 0.5 runner reads. Answers whether it copied one.
async fn adopt<P, St>(views: &St, projection: &P) -> Result<bool, Failure>
where
    P: Projection<Store = St>,
    St: ProjectionStore,
{
    let derived = checkpoint_id(projection)?;
    if views.checkpoint(&derived).await? != Checkpoint::NeverRun {
        return Ok(false); // the 0.5 runner has already run
    }
    let (through, authority) = match views.checkpoint(projection.id()).await? {
        Checkpoint::Live { through } => (through, Authority::Live),
        Checkpoint::Rebuilding { through } => (through, Authority::Rebuilding),
        _ => return Ok(false), // nothing to adopt
    };
    let empty = views.begin().await?;
    views.commit(empty, &derived, through, authority).await?;
    Ok(true)
}

#[derive(serde::Serialize, serde::Deserialize)]
enum Seat {
    Taken,
}

const SEAT_TAKEN: EventType = EventType::from_static("SeatTaken");

impl DomainEvent for Seat {
    const EVENT_TYPES: &'static [EventType] = &[SEAT_TAKEN];
    fn event_type(&self) -> EventType {
        SEAT_TAKEN
    }
    fn tags(&self) -> Tags {
        Tags::empty()
    }
    fn encode<C: Codec>(&self, codec: &C) -> Result<Bytes, CodecError> {
        codec.encode(self)
    }
    fn decode<C: Codec>(codec: &C, _: &EventType, data: &Bytes) -> Result<Self, CodecError> {
        codec.decode(data)
    }
}

struct SeatsSold {
    id: ProjectionId,
    scope: Tags,
}

impl Projection for SeatsSold {
    type Event = Seat;
    type Store = MemoryProjectionStore;
    type Error = MemoryProjectionStoreError;

    fn id(&self) -> &ProjectionId {
        &self.id
    }

    fn scope(&self) -> &Tags {
        &self.scope
    }

    async fn apply(
        &mut self,
        event: Delivered<Seat>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Self::Error> {
        let Seat::Taken = event.into_event();
        batch.write("seats_sold", 1);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Failure> {
    let log = MemoryEventStore::new();
    log.append(&[Event::new("SeatTaken", &b"{}"[..])?], None).await?;
    let head = log.snapshot().last().ok_or("the log is empty")?.position;

    let views = MemoryProjectionStore::new();
    let projection = SeatsSold {
        id: ProjectionId::new("seats_sold")?,
        scope: Tags::empty(),
    };

    // What the 0.4 runner left behind: a checkpoint under the bare name.
    let old = views.begin().await?;
    views.commit(old, projection.id(), head, Authority::Live).await?;

    assert!(adopt(&views, &projection).await?);
    let derived = checkpoint_id(&projection)?;
    let adopted = views.checkpoint(&derived).await?;
    assert_eq!(adopted, Checkpoint::Live { through: head });
    assert!(!adopt(&views, &projection).await?, "a second call adopts nothing");
    Ok(())
}
```

The old checkpoint under the bare name is left behind, and it is harmless. To
clear it, `reset` that id with an **empty** batch: `begin()` one and write
nothing into it. A reset applies its batch, and the view's rows are the ones the
derived id now owns, so a batch that deletes them deletes the adopted view's. Anything else that read a runner's checkpoint
through `projection.id()`, such as a health check or a reset script, reads it
through `checkpoint_id(&projection)` now.

### A projection can choose to skip a failing event **[H6]**

**What changed.** `Projection` has a new provided method, `on_error`. The runner
calls it when an event fails to decode or to apply, and it answers a `Policy`:
`Halt` or `Skip`
([PS-27](../spec/SPECIFICATION.md#48-failure-policy)). The default answers
`Halt`, so a run stops on a failure exactly as it did in 0.4, at the same
position, with the same checkpoint and the same `ProjectionError` arm. `Progressed` gains a `skipped` count.

**What to change.** Nothing. An inherent method named `on_error` on your type
does not collide, and the runner never calls it. If another trait in scope also
provides an `on_error`, a call `x.on_error(..)` in your own code becomes
ambiguous (`E0034`); spell it `Projection::on_error(&mut x, ..)` or name the
other trait.

### Code that spawns a generic projection's run implements `SendProjection` **[H10]**

**What changed.** In 0.4, `apply` was a plain `fn`, so code generic over the
projection could spawn its run, for example with `tokio::spawn`, with a bound on
the projection alone. In 0.4, not compiled:

```text
P: Projection<Store = St> + Send + 'static,
```

In 0.5, `apply` returns a future, and `P: Projection` says nothing about whether
that future is `Send`. So that bound no longer lets the run be spawned.
`SendProjection`, new in 0.5, is the flavour of `Projection` whose futures are
`Send`, and it is the one such code bounds on. The runner also holds the failure
it hands `on_error` across an await, so the projection's error must be `Sync` as
well as `Send`. The decision is
[ADR-0074](../.kb/decisions/0074-projection-apply-is-async.md).

**What to change.** Implement `SendProjection` instead of `Projection` for a
projection handed to such code, with the same items: every `SendProjection` is
a `Projection` too. Implement exactly one of the two. Then write the bound as
`Projection`'s own page compiles it, under *Spawning a run from code generic
over the projection*:

```text
P: SendProjection<Store = St, Event: Send, Error: Send + Sync> + 'static,
```

`St` is a type parameter of your own, bounded
`St: SendProjectionStore<Batch: Send> + Sync + 'static`. Written as `P::Store`,
the store's futures are not `Send`, and the spawn does not compile. A projection
whose concrete type is spawned, rather than a generic `P`, needs none of this.

## `happenstance-sqlite`

### The projection schema is version 2 **[H7]**

**What changed.** With the `projection-store` feature, a projection can now be
protected from a reset, with `SqliteProjectionStore::protect_from_reset`
([PS-18](../spec/SPECIFICATION.md#46-reset)). The protection is kept in a new
table, created on every open, and `projection_store::SCHEMA_VERSION` is `2`. A
file stays at version 1 until the first protection raises it to 2, so a 0.4
build keeps opening a file in which nothing is protected. A 0.4 build refuses a
file with a protected projection, with `UnsupportedSchemaVersion`, rather than
reset it without knowing.

**What to change.** Deploy 0.5 to every process that opens the file **before**
you protect anything. A 0.4 process that opened the file before the first
protection is not stopped by it. If you wrap a connection with
`SqliteProjectionStore::new` over a file last migrated by 0.4, open it once
through `open` first, which adds the table.

## The testkits

### Each suite has a new rule **[H8, H9]**

**What changed.** `happenstance-testkit` adds
`store_accepts_the_guaranteed_minimum_tags_per_batch`. It appends 128 events of
64 tags each, 8,192 tags in one call, and requires every tag back
([VT-36](../spec/SPECIFICATION.md#vt-36--min_supported_tags_per_batch-is-a-floor-of-8192)).
`happenstance-sync-testkit` adds
`ingest_holds_the_call_to_its_tags_per_batch_ceiling`, which runs when your
ingest fixture declares `IngestFixture::MAX_TAGS_PER_BATCH`, and is reported
skipped when it does not.

An adapter that passed 0.4's suite can fail the first rule, for example by
binding every tag row of an append into one statement and reaching its
driver's parameter limit. That adapter was already short of what the contract
required: 64 tags per event and 128 events per call, together.

**What to change.** Pin both testkits to `=0.5.0` and run your suites. If the
first rule fails, split the statement, or otherwise accept the whole call.
Declaring a `MAX_TAGS_PER_BATCH` below 8,192 does not get you past it: the rule
fails that declaration before it writes anything. If your store declares a
tags-per-call ceiling for ingest, declare the same number on your ingest fixture
so the second rule can hold you to it. Both suites gain rules again in 0.6: see
[`upgrading-to-0-6.md`](upgrading-to-0-6.md).
