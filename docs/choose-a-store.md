# Choose a store

> **Answers:** `how-to` — Which store should my deployment run on?

Five event stores implement the port, and each one runs the event-store
conformance suite. They differ in where they run, how many writers they
admit, whether a projection store sits beside them, and what they refuse. Answer
the questions below in order, then write your code so the answer can change.

| Store | Crate | Runs on | Implements |
| --- | --- | --- | --- |
| `MemoryEventStore` | [`happenstance-core`](../crates/happenstance-core/README.md) | one process | `SendEventStore` |
| `SqliteEventStore` | [`happenstance-sqlite`](../crates/happenstance-sqlite/README.md) | a host | `SendEventStore` |
| `PostgresEventStore` | [`happenstance-postgres`](../crates/happenstance-postgres/README.md) | a host | `SendEventStore` |
| `NeonEventStore<T>` | [`happenstance-neon`](../crates/happenstance-neon/README.md) | a host or `wasm32` | `EventStore` only |
| `CloudflareEventStore` | [`happenstance-cloudflare`](../crates/happenstance-cloudflare/README.md) | a Durable Object, `wasm32` | `EventStore` only |

The typed layer re-exports `MemoryEventStore`. What this page says about an
adapter comes from that crate's README or its crate-root documentation. A name
in italics below is a heading in one of the two, and the sentence says which,
unless it is the README of the store the paragraph is about.

## Start from where the code runs

**In a Durable Object, use `CloudflareEventStore`.** It writes to the object's
own SQL storage. Build it with
`CloudflareEventStore::new(SqlStorage::new(state.storage().sql()))` and call
`migrate()` before the first append (its README, *Use*). Its event-store
conformance family runs on `wasm32-unknown-unknown` inside the gate. It also
runs under `workerd`, locally and on a deployed object, in a CI job beside the
gate (*What the conformance suite ran against this adapter, and what it did
not*). One object holds one log by default. `CloudflareEventStore::namespaced`
keeps several logs in one object, each in its own tables.

**On `wasm32` without a Durable Object, or on any host that cannot hold a
database connection, use `NeonEventStore`.** It reaches Neon's `/sql` endpoint
with one HTTPS request per operation. The crate owns no HTTP client. You
implement `SqlTransport`: on `wasm32` over the runtime's `fetch`, on a host over
a client of your choosing. The only transport it ships is `NullTransport`, which
fails every round trip (*The transport is yours*). Its live conformance run uses
a host client kept in the crate's tests, so no `wasm32` transport has run the
suite.

**In a native process, choose among `MemoryEventStore`, `SqliteEventStore` and
`PostgresEventStore`.** The next section decides between them. `SqliteEventStore`
does not build for `wasm32-unknown-unknown` (its crate root, *The shape this
crate represents*). Both SQL
stores expect tokio. SQLite's reads run on a blocking task, and
`PostgresEventStore` has to be called from a runtime with tokio's drivers
enabled, as `#[tokio::main]` builds one (*Two roles, one pool*;
[ADR-0081](../.kb/decisions/0081-a-store-hops-onto-the-runtime-it-is-called-on.md)).

## Then count the writers

**One process, and nothing has to outlive it: `MemoryEventStore`.** Use it for
tests, examples and a first prototype. It keeps the log in a `Vec`, so a restart
loses it. Its fixture declines the reopen capability with that reason, and
`acknowledged_writes_survive_a_reopen` reports a skip rather than a pass
([ES-35](../spec/SPECIFICATION.md#es-35--acknowledged-writes-survive-a-reopen-where-durability-is-claimed)).

**Writers on one machine, sharing a file: `SqliteEventStore::open(path)`.**
Every handle onto one file shares one consistency boundary
([ES-34](../spec/SPECIFICATION.md#es-34--two-handles-share-one-consistency-boundary),
checked by `two_handles_observe_each_others_appends`), so separate processes can
share a log. [The tickets example](../examples/tickets-over-http/src/lib.rs)
runs an HTTP API and a projection runner as two processes over one file. Writers
queue. Each append takes SQLite's write lock with `BEGIN IMMEDIATE`. It waits up
to `BUSY_TIMEOUT_MS`, 15,000 ms, and is then refused as `AppendError::Busy`
(*Durability, as configured rather than as hoped*).

**Writers on many machines: `PostgresEventStore::new(pool)`.** Call
`migration::apply(&pool)` once before it. Its writers do not queue behind one
another. Positions come from `nextval()`, outside the transaction that uses
them. The store keeps
[ES-10](../spec/SPECIFICATION.md#es-10--position-order-is-visibility-order)
anyway, with an `xid8` visibility frontier on every read
([ADR-0024](../.kb/decisions/0024-position-visibility-mechanism.md)). That has
three costs, and you take all three:

- `head()` reports the frontier, which can sit below the newest position the
  store has assigned. `head_is_the_highest_visible_position` asserts a bound
  for that reason, not an equality (ES-10).
- A read issued right after a successful append may not contain that append's
  events. Read-your-own-writes does not hold (ES-10).
- A reader lags by as long as the longest write transaction open anywhere on
  the cluster, in any database. A migration or an idle-in-transaction
  connection is enough (*The problem this crate is here to solve*;
  [ADR-0071](../.kb/decisions/0071-es-10-stays-global.md)).

**Writers with no connection to hold: `NeonEventStore::new(transport, config)`.**
Send `migration::request(migration::MIGRATION_1, &config)` through your
transport once before it. The endpoint is PostgreSQL, so the same frontier applies with the same three
costs (its crate root, *Status*). A read is ordered before a later append
([ES-11](../spec/SPECIFICATION.md#es-11--a-read-is-a-snapshot)) only when both
go through clones of one transport value. Two transports with separate ledgers
are not ordered against each other, and no conformance rule can see the
difference (*ES-11, and the fence that meets it*;
[ADR-0087](../.kb/decisions/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md)).

**A Durable Object has one writer by construction.** The object is a
single-threaded actor that owns its storage, so its store has nobody to lose a
race to. The concurrency family is not run against it, because that family's
contenders run on OS threads and the store is `!Send` (its crate root, *The
concurrency family is not invoked, and that is a reason rather than a
silence*).

## Put the checkpoint where the read model is

One projection store commits your read-model rows and the projection's
checkpoint, as one unit
([PS-1](../spec/SPECIFICATION.md#41-status-and-what-would-freeze-the-port),
checked by `commit_is_atomic_with_the_read_model`). So choose the projection
store by the database your read model lives in. One projection writes to one
store ([PS-10](../spec/SPECIFICATION.md#43-the-write-seam)). The event log can be
in a different database, but not in a different process: the typed layer's
`run_projection`, behind its `unstable-projection` feature, takes the event
store and the projection store as two arguments to one call, so the process
that runs it has to hold both.

| Event store | Projection store in the same crate | Feature |
| --- | --- | --- |
| `MemoryEventStore` | `MemoryProjectionStore` | `memory`, on by default |
| `SqliteEventStore` | `SqliteProjectionStore` | `projection-store` |
| `PostgresEventStore` | `PostgresProjectionStore` | `projection-store` |
| `NeonEventStore<T>` | `NeonProjectionStore<T>` | `projection-store` |
| `CloudflareEventStore` | none | none |

Each SQL projection store needs its checkpoint table before the first commit.
`SqliteProjectionStore::open` creates it on every connect. For Postgres, run
`migration::apply_projection(&pool)`, which applies `migration::MIGRATION_2`
separately from the event log's `migration::apply` (its README, *Two roles, one
pool*). For Neon, send `migration::request(migration::MIGRATION_2, &config)`
through your transport.

**A Durable Object's log has no projection store beside it.**
`happenstance-cloudflare` does not implement the projection port (its crate
root, *Status: what publication this crate is ready for, and what it is not*).
`CloudflareEventStore` exists only inside the object, on `wasm32`, so a runner
reading it has to run in the object too, and only two projection stores can
run there:

- **`MemoryProjectionStore`.** It holds the read model and the checkpoint in the
  object's memory, one lock over one struct (its own documentation), so both
  are gone when that memory is. With no checkpoint, a runner replays the log
  from its start
  ([PS-20](../spec/SPECIFICATION.md#46-reset)), the same replay
  [Rebuild a read model](rebuild-a-read-model.md) walks through.
- **`NeonProjectionStore<T>`, over a `wasm32` transport you write.** The crate
  compiles for `wasm32`, but its live conformance run uses a host client, so
  no `wasm32` transport has run the projection suite.

`SqliteProjectionStore` cannot run in the object, because its crate does not
build for `wasm32`. Neither can `PostgresProjectionStore`: its crate claims
only the host (`references/adapter-shapes.md`, *1. The portfolio at a
glance*), and nothing in the gate builds it for `wasm32`. Reading the log from outside the object, for
example through an HTTP API the object serves, is not something any crate here
provides or documents. If you build it, the conformance suite has not checked
it.

Two more limits come with the SQL projection stores:

- **`apply` cannot read back what it wrote earlier in the same batch.** The
  batches of `SqliteProjectionStore`, `PostgresProjectionStore` and
  `NeonProjectionStore` have no read path, and their
  `batch_reads_reflect_pending_writes` is a reported skip
  ([PS-12](../spec/SPECIFICATION.md#44-read-your-writes);
  `happenstance-postgres`'s *What the projection batch is, and what it costs
  you*).
- **Point `NeonProjectionStore` at a read-write primary, never a read
  replica.** A replica can report an older checkpoint, and a runner resuming
  from it applies events a second time
  ([PS-38](../spec/SPECIFICATION.md#47-checkpoint-semantics); *Point the
  projection store at the primary*). No conformance rule can detect a replica,
  because the suite runs against one endpoint.

## Bind the bare trait, so the choice stays one line

Write the code that takes a store generic over `EventStore`
([ES-1](../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one)).
Every `SendEventStore` is also an `EventStore`, and never the reverse. A
function bound on `EventStore` accepts all five stores, so moving from the
memory store to the one you deploy changes the line that builds the store and
nothing else. Import only `EventStore` in that module. With both names in scope,
`store.read(…)` is ambiguous, and rustc reports E0034.

Run that code against more than one store before you choose. The testkit has
two stand-ins for what the memory store cannot show you. `GappyMemoryStore` is
a conformant store whose positions skip. `FaultyStore` implements only the bare
flavour, as `CloudflareEventStore` and `NeonEventStore` do. This program runs
one function on the memory store and on both stand-ins:

```rust
use core::num::NonZeroU64;
use happenstance::{Event, EventStore, MemoryEventStore, Query, ReadOptions, collect};
use happenstance_testkit::{FaultyStore, GappyMemoryStore};

type Failure = Box<dyn core::error::Error>;

/// Appends three events, then reports how many landed after the first,
/// counted by reading them, and how far the last position is from the first.
async fn after_the_first<S: EventStore>(store: &S) -> Result<(usize, u64), Failure> {
    let seat = Event::new("Seated", &b"{}"[..])?;
    let first = store.append(&[seat.clone()], None).await?;
    let last = store.append(&[seat.clone(), seat], None).await?;
    let read = collect(store.read(&Query::all(), ReadOptions::new().from(first))).await?;
    let counted = read.iter().filter(|event| event.position > first).count();
    Ok((counted, last.get().saturating_sub(first.get())))
}

#[tokio::main]
async fn main() -> Result<(), Failure> {
    let stride = NonZeroU64::new(10).ok_or("ten is not zero")?;

    // The reference store: dense positions, so the two numbers agree.
    assert_eq!(after_the_first(&MemoryEventStore::new()).await?, (2, 2));
    // A conformant store with gaps: the count holds, the distance does not.
    let gappy = GappyMemoryStore::with_stride(stride);
    assert_eq!(after_the_first(&gappy).await?, (2, 20));
    // A store with only the bare flavour still fits the bound.
    let bare = FaultyStore::new(MemoryEventStore::new());
    assert_eq!(after_the_first(&bare).await?, (2, 2));
    Ok(())
}
```

The count is the same on every store, and the distance is not. Any store may
leave gaps between positions
([VT-11](../spec/SPECIFICATION.md#vt-11--positions-are-unique-strictly-increasing-in-assignment-order-and-may-have-gaps),
checked by `positions_are_strictly_monotonic`). The memory store's positions
happen to be dense, and a test that passes only there is testing that accident.
Count events by reading them, never by subtracting positions.

Bind `SendEventStore` only where the store crosses a thread, as it does when you
hand it to `tokio::spawn`. That bound rules out the Durable Object and Neon
stores, and the compiler says so with E0277:

```rust,compile_fail,E0277
use happenstance::{MemoryEventStore, SendEventStore};
use happenstance_testkit::FaultyStore;

/// Bound on the `Send` flavour, because it hands the store to another thread.
fn spawnable<S: SendEventStore>(_store: &S) {}

fn main() {
    spawnable(&FaultyStore::new(MemoryEventStore::new()));
}
```

Stable rustdoc does not check the error code on a `compile_fail` fence, so the
fence above would also pass if it failed for some other reason. This control is
the same program with a store that implements `SendEventStore`, and it compiles:

```rust
use happenstance::{MemoryEventStore, SendEventStore};

/// Bound on the `Send` flavour, because it hands the store to another thread.
fn spawnable<S: SendEventStore>(_store: &S) {}

fn main() {
    spawnable(&MemoryEventStore::new());
}
```

## Check the limits before you need them

**Capacity.** A batch over a store's limit is refused as
`AppendError::ExceedsStoreLimit`, and never truncated
([VT-25](../spec/SPECIFICATION.md#vt-25--a-capacity-refusal-is-distinguishable-from-a-store-failure),
checked by `append_reports_exceeded_store_limits`):

| Store | Bytes of data per event | Tags per event | Events per append |
| --- | --- | --- | --- |
| `SqliteEventStore` | 1,048,576 | 128 | 256 |
| `PostgresEventStore` | 1,048,576 | 128 | 256 |
| `NeonEventStore<T>` | 131,072 | 128 | 128 |
| `CloudflareEventStore` | 1,048,576 | 1,024 | 1,024 |
| `MemoryEventStore` | none declared | none declared | none declared |

`SqliteEventStore`, `PostgresEventStore` and `NeonEventStore` publish their
limits as the associated constants `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT`
and `MAX_EVENTS_PER_BATCH`. `CloudflareEventStore` has no public constants for
them: its row comes from its README, *The limits this store declares*, which
calls it a stated refusal policy and not a measured wall. If your events
have to fit every store, stay within `MIN_SUPPORTED_EVENT_DATA_LEN`,
`MIN_SUPPORTED_TAGS_PER_EVENT` and `MIN_SUPPORTED_EVENTS_PER_BATCH`, the floors
every store accepts
([VT-21](../spec/SPECIFICATION.md#vt-21--min_supported_event_data_len-is-a-floor-of-65536-bytes),
[VT-22](../spec/SPECIFICATION.md#vt-22--min_supported_tags_per_event-is-a-floor-of-64),
[VT-24](../spec/SPECIFICATION.md#vt-24--min_supported_events_per_batch-is-a-floor-of-128)).
A Neon read has one more ceiling: its whole response has to fit in
`MAX_RESPONSE_BYTES`, 64 MiB, because there is no cursor (*Limits this store
enforces*).

**Busy refusals.** A store may refuse an append because it was busy, and then
it wrote nothing
([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so),
checked by `a_busy_append_left_nothing_behind`). SQLite refuses when its busy
timeout runs out. Postgres and Neon refuse when a serialisation failure outlasts
their own retry budget, which only a conditional append can meet (the Postgres
crate root's *What this store costs a caller*; `NeonEventStore`'s
`SERIALISATION_ATTEMPTS`). The memory store and the Durable Object store never
refuse as busy. The memory store queues its writers in-process, and an object
has one writer, so neither has anything to wait out (`AppendError::Busy`'s own
documentation; `CloudflareEventStoreError`'s *Why this store never reports
`AppendError::Busy`*). The typed layer's command loop retries a busy refusal
inside the same bound as a violated condition, and
[Retry a command](retry-a-command.md) shows how to bound it.

**Positions.** Every store may leave gaps, as above. The Durable Object store
also cannot hold a position above 2^53 − 1, because Workers SQL returns integers
through a JavaScript number. A stored position past that bound is reported as
`CloudflareEventStoreError::StoredPosition` rather than narrowed (*The limits
this store declares*).

