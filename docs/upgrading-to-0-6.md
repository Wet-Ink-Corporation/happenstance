# Upgrading from 0.5 to 0.6

> **Answers:** `how-to` — How do I move my code from 0.5 to 0.6?

This page is for you if your project depends on happenstance 0.5 and you want to
move to 0.6. By the end your code will build against 0.6, your stored data will
be at the schema 0.6 expects, and your processes will have been upgraded in an
order that keeps them working.

0.6 is a breaking release, but very little of it breaks at compile time. Almost
every change below compiles unchanged and changes what happens when you deploy:
a wire format, three storage schemas, and two refusals. So most of this page is
about **the order you deploy in**, not about the code you write.

`happenstance` and `happenstance-core` do not change. If your application uses
only those two, on the in-memory store and with no `sync` feature, change the
version number and you are done. The last section, on what is new, may still be
worth a read.

## How to use this page

Work through the sections for the crates in your `Cargo.toml`, in the order
below. You can skip any section for a crate you do not use. Each change says
what broke, what to change, and why.

| If you use | Read |
| --- | --- |
| `happenstance-sync`, or an adapter's `sync` feature | [replication](#replication) |
| `happenstance-sqlite` | [`happenstance-sqlite`](#happenstance-sqlite) |
| `happenstance-postgres` or `happenstance-neon` | [Postgres and Neon](#postgres-and-neon) |
| `happenstance-cloudflare` | [`happenstance-cloudflare`](#happenstance-cloudflare) |
| `happenstance-testkit` or `happenstance-sync-testkit` (you wrote an adapter) | [the testkits](#the-testkits) |

Each heading ends with a bracketed id, such as **[H3]**. That is the row for
this change in the `0.6.0` trace table, at the end of the
[`CHANGELOG.md`](../CHANGELOG.md#060--2026-11-09) section for this release. The
table lists every breaking change with the decision behind it, if you want the
full reasoning.

About the code on this page: blocks tagged `rust` are compiled and run against
this release by the project's build, so they show the 0.6 spelling. Blocks
tagged `text` are not compiled, because this page does not link the database
adapters. Each one says what it is.

## Replication

This section is for you if you replicate between stores: you depend on
`happenstance-sync`, or you turned on an adapter's `sync` feature.

### Every peer upgrades together: the wire format is version 3 **[H1]**

**What changed.** Every replication message carries the format version it was
written at, and a reader refuses a version it does not speak
([WF-8](../spec/SPECIFICATION.md#wf-8--the-envelope-is-versioned-once-per-message-and-an-unknown-version-is-refused-whole)).
`wire::FORMAT_VERSION` is now `3`, because two messages changed shape (the next
section). A buffer written by 0.5, at version 2, is refused whole as
`WireError::UnsupportedFormatVersion`, and nothing inside it is read. The
decision is
[ADR-0104](../.kb/decisions/0104-every-pull-and-ack-names-its-responder.md).

The program below shows both ways the refusal reaches you. A postcard reader
reads the version off the front of the buffer and asks `check_format_version`,
because postcard's own errors cannot carry the reason. A JSON reader gets a
decode error whose text carries `WireError::UNSUPPORTED_FORMAT_VERSION`:

```rust
use happenstance::{Codec, CodecError, Json};
use happenstance_sync::Ack;
use happenstance_sync::wire::{self, Envelope, WireError};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    assert_eq!(wire::FORMAT_VERSION, 3);

    // What a postcard reader asks of the version at the front of a buffer.
    assert_eq!(
        wire::check_format_version(2),
        Err(WireError::UnsupportedFormatVersion { found: 2, supported: 3 }),
    );

    // An envelope 0.6 writes, and the same envelope stamped as 0.5 stamped it.
    // The version is checked first, so nothing past it is read.
    let current = String::from_utf8(Json.encode(&Envelope::new(Ack::default()))?.to_vec())?;
    let from_0_5 = current.replacen(r#""format_version":3"#, r#""format_version":2"#, 1);
    assert_ne!(current, from_0_5);

    let Err(CodecError::Decode(refused)) = Json.decode::<Envelope<Ack>>(from_0_5.as_bytes())
    else {
        return Err("a version-2 envelope decoded".into());
    };
    assert!(refused.to_string().contains(WireError::UNSUPPORTED_FORMAT_VERSION));
    Ok(())
}
```

**What to change.** Plan the upgrade of a deployment that replicates as one
change: stop replicating, upgrade every peer, then start again. A 0.6 peer and a
0.5 peer cannot exchange a single message in either direction, and the refusal
is class `Stop`
([SY-36](../spec/SPECIFICATION.md#510-retention-across-a-peer-set-and-refusal)),
so retrying it does nothing.

If you keep replication messages anywhere, such as an outbox, a queue, or
messages parked after a refusal, drain them before you upgrade. Bytes written
at version 2 never decode under 0.6.

### `Pulled` and `Ack` name the store that answered **[H2]**

**What changed.** `Pulled` and `Ack` have a new last field,
`responder: Option<StoreId>`. It names the store incarnation that answered the
pull or took the push, so a runner can tell that the far side was restored or
re-minted under a new `StoreId`
([VT-6](../spec/SPECIFICATION.md#vt-6--storeid-names-a-store-incarnation-not-a-device-and-not-a-peer)).
Both types were already `#[non_exhaustive]`, so every way you could build one
in 0.5 still compiles. The decision is
[ADR-0104](../.kb/decisions/0104-every-pull-and-ack-names-its-responder.md).

The serialised shape did change, and that is what the version bump above is
for. In JSON the `responder` key is required: it is written as `null` when
there is no responder, and a body without it is refused as a missing field.
Reading a missing key as "no responder" would have made a claim the sender
never made.

**What to change.** Nothing, if you build these types through `Pulled::new` and
`Ack::default`: both name no responder. If you wrote a `SyncPeer` of your own
that answers from a store, name that store with `with_responder`, as the
shipped `StoreSyncPeer` does, so the runner can see a restore. If you put
`Pulled` or `Ack` into JSON yourself, every reader and writer of that JSON
moves to 0.6 together:

```rust
use happenstance::{Codec, CodecError, Json, StoreId};
use happenstance_sync::{Ack, Pulled, PushBatch};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // A 0.5 constructor call compiles unchanged, and names no responder.
    let pulled = Pulled::new(PushBatch::empty(), 0_u64);
    assert_eq!(pulled.responder, None);
    assert_eq!(Ack::default().responder, None);

    // A peer that answers from a store of its own says which one.
    let store = StoreId::from_bytes([7; 16]);
    assert_eq!(Ack::default().with_responder(store).responder, Some(store));
    assert_eq!(pulled.with_responder(store).responder, Some(store));

    // In JSON the key is always written, as `null` when nothing answered...
    let json = String::from_utf8(Json.encode(&Ack::default())?.to_vec())?;
    assert!(json.contains(r#""responder":null"#));
    let _round_trip: Ack = Json.decode(json.as_bytes())?;

    // ...and a body without it, the shape 0.5 wrote, is refused.
    let from_0_5 = json.replace(r#","responder":null"#, "");
    let Err(CodecError::Decode(missing)) = Json.decode::<Ack>(from_0_5.as_bytes()) else {
        return Err("a body without `responder` decoded".into());
    };
    assert!(missing.to_string().contains("missing field `responder`"));
    Ok(())
}
```

## `happenstance-sqlite`

### Opening a file migrates it to schema 2 **[H3]**

**What changed.** The event store records which rows one write put in, so a
runner and a sync peer can forward the log in whole writes
([SY-39](../spec/SPECIFICATION.md#58-scope-whole-log-or-filtered-replication)).
That takes two new columns, and the event store's `SCHEMA_VERSION` is `2`. The
decision is
[ADR-0105](../.kb/decisions/0105-write-groups-are-recorded-per-row.md). This
happens in every build, with or without the `sync` feature.

- `SqliteEventStore::open`, `open_on` and `open_in_memory` migrate a file at
  schema 1 when they open it. The migration is two `ALTER TABLE … ADD COLUMN`
  statements in one transaction, and it does not rewrite the table. Rows written
  before it read back as writes of one event each.
- **After that, a 0.5 build refuses the file**, with
  `UnsupportedSchemaVersion`.
- `SqliteEventStore::new` never migrates. It now refuses a file still at
  schema 1 with the new `SqliteEventStoreError::SchemaNeedsMigration`, where it
  would otherwise have failed the first append with a `no such column` error.

**What to change.** Deploy 0.6 to every process that opens the file, and stop
every 0.5 process, **before** any 0.6 process opens it. A 0.5 process that
already has the file open is not refused until it opens the file again, so stop
it first rather than find out at its next restart.

If you wrap a connection you opened yourself with `new`, open the file once
through `open_on` first, which migrates it and leaves your pragmas alone. In
0.6, not compiled:

```text
// `new` refuses a file at schema 1; `open_on` migrates it and wraps it.
let store = SqliteEventStore::open_on(connection)?;
```

## Postgres and Neon

### Apply migration 3, then deploy **[H4, H5]**

**What changed.** The same write groups as SQLite's, on the server: migration 3
adds the nullable columns `write_group` and `group_guard` to the event table,
and a partial index. Every `INSERT` 0.6 sends names `write_group`. So **a 0.6
process writing to a schema without migration 3 fails its first append** with
an undefined-column error. `happenstance-postgres`'s `SCHEMA_VERSION` is `2`.
The decision is
[ADR-0105](../.kb/decisions/0105-write-groups-are-recorded-per-row.md).

A 0.5 build keeps working against a migrated schema, so migrating first is safe.

**What to change.** Migrate every schema, then deploy. Both migrations are
idempotent, so it is safe to run them against a schema that is already
migrated, or to run them twice.

On Postgres, run `migration::apply` against each schema before any 0.6 process
writes to it. It now sends migrations 1 and 3 together. In 0.6, not compiled:

```text
happenstance_postgres::migration::apply(&pool).await?;
```

On Neon, send migration 3 after migration 1, as you send migration 1, through
your transport. In 0.6, not compiled:

```text
use happenstance_neon::migration::{self, MIGRATION_1, MIGRATION_3};

transport.round_trip(migration::request(MIGRATION_1, &config)).await?;
transport.round_trip(migration::request(MIGRATION_3, &config)).await?;
// `round_trip` returns the HTTP response: check each one's status as you
// check migration 1's, or a migration the endpoint rejects reads as done.
```

If your own tooling renders the Neon templates instead of
`migration::render`, teach it the new `@write_group_idx@` placeholder, which
`render` fills with the index name. An unfilled placeholder is a syntax error at
the server, so it cannot be missed silently.

The `ALTER TABLE`s rewrite no row but take a brief exclusive lock, and the index
build blocks writes while it runs. On a large table, build the index first with
`CREATE INDEX CONCURRENTLY`. Each crate's README says how:
[`happenstance-postgres`](../crates/happenstance-postgres/README.md) and
[`happenstance-neon`](../crates/happenstance-neon/README.md).

### A schema with no identity row refuses with `MissingIdentity` **[H6, H7]**

**What changed.** Each store keeps its identity, its `StoreId`, in one row that
migration 1 writes. An append or an ingest on a schema whose row is gone now
writes nothing and refuses:

- `happenstance-postgres` reads the identity row inside every append and ingest,
  instead of caching it on the handle. A handle that had read the row before it
  went used to keep stamping events with it. It now refuses with
  `PostgresEventStoreError::MissingIdentity`. The read costs one primary-key
  lookup per append, and it means a re-mint is seen by every handle at once,
  with no restart.
- `happenstance-neon` refuses an append with `NeonError::MissingIdentity`, and
  an ingest with `IngestError::Store(NeonError::MissingIdentity)`. In 0.5 the
  append committed rows with no origin, and so did an ingest's compensation,
  and every later read refused them as `UnstampedEvent`.
  `NeonError::MissingIdentity` is no longer behind `sync`.

The decision is
[ADR-0086](../.kb/decisions/0086-postgres-and-neon-keep-mint-once.md).

**What to change.** Nothing, unless your schema has lost its identity row. Then
0.6 stops writing to it, where 0.5 kept going. The usual cause is a schema that
migration 1 never ran on, so run it. `remint_identity` does not create the row
either: it refuses with the same error.

## `happenstance-cloudflare`

There is no break here. `migrate` adds the same two columns where they are
missing, and there is no schema marker, so a 0.5 build keeps working against an
object 0.6 has migrated. Rows a 0.5 build writes read back as writes of one
event each. Deploy 0.6 to every writer and peer of an object before it serves
replication.

## The testkits

This section matters only if you wrote an adapter and run the conformance
suites against it. Pin both testkits to `=0.6.0` and run your suites. Each
change below can turn a passing adapter's CI red with no change to the adapter.
That is why a testkit's version moves when it gains a rule
([CF-32](../spec/SPECIFICATION.md#66-compatibility-policy-for-the-testkit)).

### A projection fixture states whether it can reopen **[H8]**

**What changed.** `happenstance-testkit` has a new projection rule,
`committed_rows_and_checkpoint_survive_a_reopen`. It commits a row and a
checkpoint, drops every handle, reopens the storage, and reads both back
([PS-1](../spec/SPECIFICATION.md#41-status-and-what-would-freeze-the-port),
[CF-17](../spec/SPECIFICATION.md#63-the-fixture-contract)). It runs only on a
fixture that declares the new `ProjectionFixture::REOPEN` capability.

`REOPEN` has a default, so your fixture still compiles. But the suite's
`every_declined_capability_is_stated_by_this_fixture` now fails a projection
fixture that keeps that default, `UNSTATED_PROJECTION_REOPEN`.

**What to change.** State `REOPEN` on your projection fixture. A store with no
storage outside the process declines it, with that as its reason:

```rust
use std::sync::Arc;

use happenstance_core::MemoryProjectionStore;
use happenstance_testkit::fixtures::MemoryProjectionHandle;
use happenstance_testkit::{Capability, ProjectionFixture, UNSTATED_PROJECTION_REOPEN};

struct MyFixture(Arc<MemoryProjectionStore>);

impl ProjectionFixture for MyFixture {
    type Store = MemoryProjectionHandle;

    const SECOND_HANDLE: Capability = Capability::SUPPORTED;
    const RESET_REFUSAL: Capability = Capability::declined(
        "this store protects nothing, so it has no reset to refuse",
    );
    const COMMIT_FAULT: Capability = Capability::declined(
        "this store applies both halves of a commit under one lock and has \
         no write it can be made to fail between them",
    );
    // New in 0.6. Inheriting the default fails the suite.
    const REOPEN: Capability = Capability::declined(
        "this store keeps its read model and checkpoints in process memory \
         only, so there is no medium a reopen could read them back from",
    );

    fn connect(&self) -> impl core::future::Future<Output = Self::Store> {
        core::future::ready(MemoryProjectionHandle::new(Arc::clone(&self.0)))
    }
}

fn main() {
    let reopen = <MyFixture as ProjectionFixture>::REOPEN;
    assert!(!reopen.is_supported());
    assert_ne!(reopen.reason(), Some(UNSTATED_PROJECTION_REOPEN));
}
```

A store over a file or a server declares `REOPEN` supported instead, and
overrides `reopen` to close what it opened (a file, a pool) and answer
`Reopened::Discarded`. The default body answers `Reopened::Nothing`, which the
rule fails, so a fixture that declares the capability and forgets the override
cannot pass by accident.

### The store-limits rule checks each byte ceiling on its own **[H9]**

**What changed.** If your fixture states both `MAX_EVENT_DATA_LEN` and
`MAX_METADATA_LEN`, `append_reports_exceeded_store_limits` now appends one
event with both fields at their ceilings at once, and requires it to be
accepted ([CF-40](../spec/SPECIFICATION.md#63-the-fixture-contract)). Each
ceiling bounds its own field.

**What to change.** If your store holds `data` and metadata to one shared
budget, it now fails. Accept each field up to its own ceiling.

### The ingest family opens two stores **[H10]**

**What changed.** `happenstance-sync-testkit`'s ingest family has four new
rules: `ingest_preserves_the_recorded_time`
([VT-9](../spec/SPECIFICATION.md#vt-9--a-store-records-when-it-accepted-an-event-and-that-time-is-not-an-ordering-key)),
`bulk_ingest_is_idempotent_in_bounded_round_trips`
([SY-14](../spec/SPECIFICATION.md#53-identity-and-idempotent-ingest)), which
ingests 128 events in one call, `only_the_adjudicator_compensates`
([SY-7](../spec/SPECIFICATION.md#51-the-bound-decision-ingest-never-rejects))
and `directional_merge_rules_compose`
([SY-10](../spec/SPECIFICATION.md#52-one-peer-and-a-runner-above-it)). The last
two open **two** stores from your `IngestFixture`.

**What to change.** Make sure each store your fixture opens is a store of its
own, with its own identity. A fixture whose instances share one file, one
schema or one namespace now fails. So does a store that rewrites the recorded
time of an event it ingests, or that refuses a 128-event call.

### The peer family checks the whole log, its order and its writes **[H11]**

**What changed.** The peer family has three new rules:
`round_trip_preserves_the_whole_log`
([SY-28](../spec/SPECIFICATION.md#58-scope-whole-log-or-filtered-replication)),
`event_id_order_is_identical_on_every_peer`
([SY-23](../spec/SPECIFICATION.md#55-convergence-and-the-merge-rule)) and
`push_envelope_preserves_group_boundaries`
([SY-30](../spec/SPECIFICATION.md#58-scope-whole-log-or-filtered-replication)).

**What to change.** Nothing in the fixture. A peer that serves only some of the
events it was pushed, hands them back in a different order, or merges separate
writes into one group now fails. Fix the peer.

## What is new and worth adopting

None of this is required to upgrade. Each item links to where it is documented
in full.

- **The runner.** `runner::Edge` drives one replication edge, from a local store
  to one peer, a step at a time with `Edge::step`, or until it is synced or
  quiet with `Edge::run` and a backoff you supply. It spawns nothing, acts on each failure
  class, and keeps its watermark through your projection store, in the same
  commit as your read model
  ([SY-31](../spec/SPECIFICATION.md#59-where-a-per-peer-watermark-lives)).
- **A network peer in two halves.** `StoreSyncPeer` serves a store you hold, and
  `serve_sync` answers one request for it. `RemoteSyncPeer` is the client, over
  an `Exchange`, the one-method transport you write for your network.
- **Real peers.** `happenstance-neon`'s `NeonSyncPeer` serves a Neon store in
  one SQL request per pull and one per push. `happenstance-cloudflare`'s
  `sync_http` module serves a Durable Object over HTTP. A `StoreSyncPeer` over
  `PostgresEventStore` is the Postgres peer.
- **Write groups.** `WriteGroupStore` reads a store's log back in the writes
  that made it. Every adapter implements it behind `sync`, and the runner pushes
  through it, so a push never splits a write.
- **`SendPeer`.** Wraps a `StoreSyncPeer` or a `RemoteSyncPeer` so that code
  generic over `SendSyncPeer`, such as an edge run inside `tokio::spawn`, takes
  it.

The [`happenstance-sync` README](../crates/happenstance-sync/README.md) says
which store implements what, and the crate documentation in
[`lib.rs`](../crates/happenstance-sync/src/lib.rs) covers each type.
