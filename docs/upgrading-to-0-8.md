# Upgrading from 0.7 to 0.8

> **Answers:** `how-to` — How do I move my code from 0.7 to 0.8?

This page is for you if your project depends on happenstance 0.7 and runs a
typed projection, with `run_projection` or `run_fan_out`, and you want to move
to 0.8. By the end your code will build against 0.8, and each of your views will
have its checkpoint pinned to the event store it was built from.

0.8 breaks one thing at compile time and changes one thing at run time, and both
are about the same fact: a position means something only in the store that
assigned it. A view's checkpoint is a position, so 0.8 records which store it
came from and refuses to resume it against another
([PS-40](../spec/SPECIFICATION.md#47-checkpoint-semantics)). The decision is
[ADR-0109](../.kb/decisions/0109-a-checkpoint-is-pinned-to-its-source-store.md).

Separately, the event stores now record events deleted outside the port, so
that a replica pruned that way no longer takes back, or refuses, the events it
pruned (SY-40). Nothing in your code changes for it. A SQLite store migrates its
file to schema version 3 on open, and older builds refuse the migrated file; a
Postgres or Neon store needs migration 4 applied before this release ingests
into it.

If your application does not run a typed projection and does not use a SQLite,
Postgres or Neon event store, change the version number and you are done. No
SQLite or Durable Object store needs a migration step of yours. SQLite's schema
change migrates itself on open: see
[a SQLite store migrates to version 3](#a-sqlite-store-migrates-to-schema-version-3-h4).
A Durable Object's `migrate` adds one table and one trigger on its next open,
with no marker: see
[the removal record](#a-durable-object-gains-the-removal-record).
Postgres and Neon need migration 4 applied before you deploy: see
[Postgres and Neon add the removal record](#postgres-and-neon-add-the-removal-record-h5-h6).

## How to use this page

Read the sections in order. Each change says what broke, what to change, and
why.

| If you | Read |
| --- | --- |
| call `run_projection` or `run_fan_out` | [the runners take a `Source`](#the-runners-take-a-source-h1) |
| run a view whose checkpoint 0.7 wrote | [the first run adopts its store](#the-first-0-8-run-adopts-the-store-it-reads-h2) |
| move a view to another event store, or restore one | [another store is refused](#a-run-against-another-store-is-refused-h2) |
| wrote a `ProjectionStore` that counts its calls | [one more read and one more commit](#one-more-read-and-one-more-commit-h3) |
| open a `SqliteEventStore` | [a SQLite store migrates to version 3](#a-sqlite-store-migrates-to-schema-version-3-h4) |
| run a `CloudflareEventStore` that replicates | [the removal record](#a-durable-object-gains-the-removal-record) |
| run a `PostgresEventStore` or `NeonEventStore` | [Postgres and Neon add the removal record](#postgres-and-neon-add-the-removal-record-h5-h6) |

Each heading ends with a bracketed id, such as **[H1]**. That is the row for
this change in the `0.8.0` trace table, at the end of the
[`CHANGELOG.md`](../CHANGELOG.md) section for this release.

About the code on this page: blocks tagged `rust` are compiled and run against
this release by the project's build, over the in-memory store. Blocks tagged
`text` are not compiled, because this page does not link the database adapters.
Each one says what it is.

## The runners take a `Source` **[H1]**

**What changed.** The first argument of `run_projection` and `run_fan_out` is a
`Source<'_, S>`, not a `&S`. A `Source` is the event store the run reads paired
with that store's `StoreId`, the identity the checkpoint is pinned to.

In 0.7, not compiled:

```text
run_projection(&events, &views, &mut projection, &Json, chunk).await?;
```

**What to change.** Over a `MemoryEventStore`, write `(&events).into()`: the
conversion reads the identity from the store itself, so it cannot be wrong.
Over any other store, write `Source::new(&events, events.store_id())`, reading
the identity from **the store you pass**, never from configuration. A `Source`
whose identity belongs to a different store pins the view to the wrong one.

The SQLite spelling, not compiled here:

```text
let source = Source::new(&events, events.store_id());   // SqliteEventStore
run_projection(source, &views, &mut projection, &Json, chunk).await?;
```

`PostgresEventStore`, `NeonEventStore` and `CloudflareEventStore` gain a
`store_id` of their own in 0.8, with no feature to enable. It reads the
store's identity row, so it can fail. Also not compiled here:

```text
let id = events.store_id().await?;                      // Postgres, Neon
let id = events.store_id()?;                            // Cloudflare
run_projection(Source::new(&events, id), &views, &mut projection, &Json, chunk).await?;
```

The program below runs one view both ways over the same store:

```rust
use happenstance::bytes::Bytes;
use happenstance::{Chunk, Codec, CodecError, Delivered, DomainEvent, Event};
use happenstance::{EventStore, EventType, Json, MemoryEventStore};
use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
use happenstance::{MemoryProjectionStoreError, Projection, ProjectionId};
use happenstance::{Source, Tags, run_projection};

#[derive(serde::Serialize, serde::Deserialize)]
struct Placed;

const PLACED: EventType = EventType::from_static("Placed");

impl DomainEvent for Placed {
    const EVENT_TYPES: &'static [EventType] = &[PLACED];
    fn event_type(&self) -> EventType {
        PLACED
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

struct Count {
    id: ProjectionId,
    scope: Tags,
    seen: u64,
}

impl Projection for Count {
    type Event = Placed;
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
        _: Delivered<Placed>,
        batch: &mut MemoryProjectionBatch,
    ) -> Result<(), Self::Error> {
        self.seen += 1;
        batch.write("placed", self.seen);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let events = MemoryEventStore::new();
    events.append(&[Event::new("Placed", Json.encode(&Placed)?)?], None).await?;
    let views = MemoryProjectionStore::new();
    let mut count = Count { id: ProjectionId::new("count")?, scope: Tags::empty(), seen: 0 };
    let chunk = Chunk::of(64.try_into()?);

    // The memory store's own conversion.
    let first = run_projection((&events).into(), &views, &mut count, &Json, chunk).await?;
    assert_eq!(first.applied, 1);

    // The spelling every other store uses: the identity read off the store.
    events.append(&[Event::new("Placed", Json.encode(&Placed)?)?], None).await?;
    let source = Source::new(&events, events.store_id());
    let second = run_projection(source, &views, &mut count, &Json, chunk).await?;
    assert_eq!(second.applied, 1, "the same store resumes where it stopped");
    Ok(())
}
```

## The first 0.8 run adopts the store it reads **[H2]**

**What changed.** This one compiles unchanged, so check for it by hand. A
checkpoint 0.7 wrote has no pin. On its first 0.8 run, the runner pins it to
the store that run reads and then resumes from it, so nothing is replayed and
nothing is applied twice. The checkpoint's own id, `checkpoint_id(&projection)`,
does not change.

**What to change.** **Run each view's first 0.8 run against the store it was
built from.** The adoption trusts that store. If you upgrade and move a view to
another event store in the same deploy, the view adopts the new store and
resumes from the old store's position, which is the wrong resume this release
exists to prevent. Upgrade first, run once, and move afterwards: the move is
then refused, and the next section is how to make it.

The program below writes a checkpoint the way 0.7 did, under the main id only,
and shows the first run adopting it:

```rust
use happenstance::bytes::Bytes;
use happenstance::{Authority, Checkpoint, Chunk, Codec, CodecError, Delivered};
use happenstance::{DomainEvent, Event, EventStore, EventType, Json, MemoryEventStore};
use happenstance::{MemoryProjectionBatch, MemoryProjectionStore};
use happenstance::{MemoryProjectionStoreError, Projection, ProjectionId};
use happenstance::{ProjectionStore, SourceMark, Tags, checkpoint_id};
use happenstance::{run_projection, source_checkpoint_id};

#[derive(serde::Serialize, serde::Deserialize)]
struct Placed;

const PLACED: EventType = EventType::from_static("Placed");

impl DomainEvent for Placed {
    const EVENT_TYPES: &'static [EventType] = &[PLACED];
    fn event_type(&self) -> EventType {
        PLACED
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

struct Count {
    id: ProjectionId,
    scope: Tags,
}

impl Projection for Count {
    type Event = Placed;
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
        _: Delivered<Placed>,
        _: &mut MemoryProjectionBatch,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let events = MemoryEventStore::new();
    let payload = Json.encode(&Placed)?;
    let placed = || Event::new("Placed", payload.clone());
    let applied_by_0_7 = events.append(&[placed()?], None).await?;
    events.append(&[placed()?], None).await?;

    // What 0.7 left behind: the main id through the first event, and no pin.
    let mut count = Count { id: ProjectionId::new("count")?, scope: Tags::empty() };
    let views = MemoryProjectionStore::new();
    let empty = views.begin().await?;
    let key = checkpoint_id(&count)?;
    views.commit(empty, &key, applied_by_0_7, Authority::Live).await?;

    let chunk = Chunk::of(64.try_into()?);
    let run = run_projection((&events).into(), &views, &mut count, &Json, chunk).await?;
    assert_eq!(run.applied, 1, "it resumed past what 0.7 applied");

    // And the view is now pinned to the store that run read.
    let pin = views.checkpoint(&source_checkpoint_id(&count)?).await?;
    let Checkpoint::Live { through } = pin else {
        return Err("the first run left no pin".into());
    };
    assert_eq!(through.get(), SourceMark::of(events.store_id()).get());
    Ok(())
}
```

## A run against another store is refused **[H2]**

**What changed.** A run handed a store whose identity differs from the one the
view's pin records returns `ProjectionError::SourceChanged`. It has read
nothing, applied nothing and committed nothing, and both checkpoints are as they
were. In a fan-out, only that view stops; the others run. In 0.7 the same run
skipped the new store's events at or below the old checkpoint and applied again
the ones above it, and said nothing.

**What to change.** Decide what the move is, then reset what it needs. The pin
lives under `source_checkpoint_id(&projection)`, and the error carries it as
`pin`:

- **The view must be rebuilt for the new store.** Reset the main id with the
  view's deletes, as [Rebuild a read model](rebuild-a-read-model.md) shows, and
  reset the pin. The next run pins the new store and replays its log.
- **The event store and the projection store were restored together, and the
  store's identity was re-minted.** The rows are still right for that log.
  Reset the pin alone, and the next run adopts the store it reads.

`source_checkpoint_id`'s own documentation carries a compiled example of the
first case: a view built on one store, refused on another, reset, and rebuilt.

## One more read and one more commit **[H3]**

**What changed.** Every run reads the view's pin before it reads the
checkpoint, and a view's first run under 0.8 commits the pin with an empty
batch before it opens the stream, even when there is nothing to apply. A
`ProjectionStore` sees one more `checkpoint` call per run and, once per view,
one more `begin` and `commit`.

**What to change.** Nothing, unless a test of yours counts a store's calls or
asserts that an idle run writes nothing. Update its expectation to include the
pin. A store refusing the pin's commit is reported as
`ProjectionError::RecordSource`, and one refusing its `begin` as
`ProjectionError::Begin`, both with no progress.

## A SQLite store migrates to schema version 3 **[H4]**

**What changed.** `happenstance-sqlite`'s `SCHEMA_VERSION` is 3, was 2.
`SqliteEventStore::open`, `open_on` and `open_in_memory` apply migration 3 on
open: a table, `event_removed`, and an `AFTER DELETE` trigger on `event` that
records the origin identity of every row deleted from the file, by this crate or
by any other client. Ingest then skips an event whose identity is recorded,
counting it in `Ingested::skipped`, where 0.7 took a removed foreign event back
at the tail and refused a removed own event as `UnheldOwnIdentity` (SY-40,
[ADR-0110](../.kb/decisions/0110-a-removed-event-is-not-readmitted.md)).
The file is the only thing that changes; no signature does.

**What to change.** Two things, both about deploying:

- **Upgrade every process that opens the file before any of them opens it.** A
  0.7 build refuses a file 0.8 has opened, with
  `SqliteEventStoreError::UnsupportedSchemaVersion { found: 3, supported: 2 }`,
  as 0.6 builds refused 0.7's version 2.
- **If you wrap a connection with `SqliteEventStore::new`, open the file once
  through `open_on` first.** `new` never migrates, so it refuses a file still at
  2 with `SchemaNeedsMigration { found: 2, required: 3 }`.

Rows deleted **before** the migration are not recorded, because nothing was
listening. If you pruned a replica under 0.7 and it may be offered those events
again, record each one by hand, its origin store's 16 bytes and its origin
position. Not compiled here:

```text
INSERT INTO event_removed (origin_store, origin_position)
VALUES (X'0123456789ABCDEF0123456789ABCDEF', 42);
```

The record grows by one small row per removed event and is kept for ever.
Deleting a row of it lets that identity be ingested again, so do that only once
no peer can offer the event. A restore from a copy of the file rolls the record
back with the log, which is what keeps SY-38's refusal of a restored store true.

## A Durable Object gains the removal record

**What changed.** `CloudflareEventStore::migrate` creates one more table,
`event_removed`, and an `AFTER DELETE` trigger on the event table that records
the identity of every event deleted from the log, so that ingest skips a pruned
event offered again rather than taking it back or refusing it
([SY-40](../spec/SPECIFICATION.md)). A namespaced log gets its own pair under
its prefix. This is not a break, so it has no row in the trace table: `migrate`
creates both `IF NOT EXISTS` on the next open, and an older build ignores them.

**What to change.** Nothing in your code. Deploy 0.8 to every writer of an
object before it serves replication again, because an older build ingests
without consulting the record. Events your retention job deleted before the
upgrade are not recorded; the crate README gives the `INSERT` that records one
by hand.

## Postgres and Neon add the removal record **[H5, H6]**

**What changed.** Both adapters gain migration 4: a table that records the
origin identity of every event deleted from the log, and a `BEFORE DELETE`
trigger that writes it, for every client and every session. Ingest skips an
event whose identity is recorded, counting it in `Ingested::skipped`, where 0.7
took a removed foreign event back at the tail and refused a removed own event
as `UnheldOwnIdentity` (SY-40,
[ADR-0110](../.kb/decisions/0110-a-removed-event-is-not-readmitted.md)). To
close the race between a removal and an ingest of the same event, every ingest
first takes `LOCK TABLE ... IN SHARE MODE` on the record, in the same round trip
as before. `happenstance-postgres`'s `SCHEMA_VERSION` is 3, was 2. No signature
changes.

**What to change.** Apply the migration, then deploy 0.8. A 0.8 ingest against
a schema without the record fails with an undefined table.

On Postgres, `migration::apply` sends migrations 1, 3 and 4. Not compiled here:

```text
happenstance_postgres::migration::apply(&pool).await?;
```

On Neon, send `migration::removal_record_request`, the new table with its
two trigger statements in one request, after migrations 1 and 3. Sending
`migration::request(MIGRATION_4, &config)` alone creates the table and no
trigger, and nothing records a removal. Not compiled here:

```text
transport.round_trip(migration::removal_record_request(&config)).await?;
```

**Grant the ingesting role two privileges on the record.** Every ingest locks it
`IN SHARE MODE`, which Postgres admits only to a role holding `UPDATE`,
`DELETE`, `TRUNCATE` or, from 17, `MAINTAIN`, and reads it, which needs
`SELECT`. A role that deletes from the log needs nothing more: the trigger
runs as the role that applied the migration. On Neon the table is
`<event table>_removed`. Not compiled here:

```text
GRANT SELECT, UPDATE ON event_removed TO <ingest role>;
-- or, on Postgres 17 and later, a grant that writes nothing:
GRANT SELECT, MAINTAIN ON event_removed TO <ingest role>;
```

Both are idempotent. Appends and reads do not touch the record, so a process
that only appends keeps working before you migrate. An older build keeps
working against a migrated schema, but its ingest ignores the record and
re-admits what was removed, so upgrade every process that replicates into the
store.

A removal and an ingest now wait for each other, and under Postgres' default
`lock_timeout` of `0` an ingest waits for ever: a `psql` session left open
after a `DELETE` on the log holds every ingest until it ends. Set a
`lock_timeout` on the ingesting role (`ALTER ROLE <ingest role> SET
lock_timeout = '5s'`) or, on Postgres, on its connections. An ingest that gives
up fails with SQLSTATE `55P03`, which both adapters now classify as transient,
so the runner retries it. `TRUNCATE` fires no row trigger and
records nothing. Rows deleted **before** the migration are not recorded; if you
pruned a replica under 0.7 and it may be offered those events again, record each
one by hand, its origin store's 16 bytes and its origin position. On Neon the
table is `<event table>_removed`. Not compiled here:

```text
INSERT INTO event_removed (origin_store, origin_position)
VALUES ('\x0123456789abcdef0123456789abcdef'::bytea, 42)
ON CONFLICT DO NOTHING;
```

The record grows by one small row per removed event and is kept for ever.
Deleting a row of it lets that identity be ingested again, so do that only once
no peer can offer the event. A restore brings the record back with the log and
the sequence, which is what keeps SY-38's refusal of a restored store true.
