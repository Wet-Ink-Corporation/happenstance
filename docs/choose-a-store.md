# Choose a store

> **Answers:** `how-to` — Which event store should my deployment run on?

happenstance can keep your events in five different stores, and you need to
pick one before you deploy. They differ in where they can run, how many
writers they can serve at once, whether they can also hold your read models,
and what they will refuse. This page walks through those questions in order.
It also shows how to write your code so that changing your mind later is a
one-line change.

Every store below passes the event-store family of the conformance suite, so the rules
from [your first encounter](first-encounter.md), such as conditions being
checked and positions only going up, hold on all of them.

| Store | Crate | Runs on | Implements |
| --- | --- | --- | --- |
| `MemoryEventStore` | [`happenstance-core`](../crates/happenstance-core/README.md) | one process | `SendEventStore` |
| `SqliteEventStore` | [`happenstance-sqlite`](../crates/happenstance-sqlite/README.md) | a host | `SendEventStore` |
| `PostgresEventStore` | [`happenstance-postgres`](../crates/happenstance-postgres/README.md) | a host | `SendEventStore` |
| `NeonEventStore<T>` | [`happenstance-neon`](../crates/happenstance-neon/README.md) | a host or `wasm32` | `EventStore` only |
| `CloudflareEventStore` | [`happenstance-cloudflare`](../crates/happenstance-cloudflare/README.md) | a Durable Object, `wasm32` | `EventStore` only |

The last column matters once you write generic code, and the section on
[binding the bare trait](#bind-the-bare-trait-so-the-choice-stays-one-line)
explains it. `MemoryEventStore` is also available directly from the
`happenstance` crate, so you do not need to add `happenstance-core` to use it.

Where this page says something about a store, it comes from that store's
README or its crate documentation. A name in *italics* is a heading there you
can look up. Unless the sentence names another crate, it is in the README of
the store the paragraph is about.

## Start from where the code runs

Some stores only run in certain places, so this question rules options out
before any other.

### Inside a Cloudflare Durable Object

**Use `CloudflareEventStore`.** A Durable Object is Cloudflare's single-threaded
actor with its own private storage. This store writes your events into the
object's own SQL storage.

Build it with
`CloudflareEventStore::new(SqlStorage::new(state.storage().sql()))`, and call
`migrate()` before the first append (its README, *Use*). One object holds one
log by default. `CloudflareEventStore::namespaced` keeps several logs in one
object, each in its own tables. [Run on a Durable Object](run-on-a-durable-object.md)
walks through it.

Its event-store conformance tests run on `wasm32-unknown-unknown` in the CI gate.
They also run under `workerd`, Cloudflare's runtime, both locally and on a
deployed object, in a separate CI job (*What the conformance suite ran against
this adapter, and what it did not*).

### On `wasm32` without a Durable Object, or anywhere you cannot hold a connection

**Use `NeonEventStore`.** Neon is a hosted PostgreSQL that can be reached over
plain HTTPS. This store sends one HTTPS request to Neon's `/sql` endpoint per
operation, so it needs no long-lived database connection. That suits edge and
serverless runtimes that cannot keep one open.

The crate does not include an HTTP client. You implement its `SqlTransport`
trait: on `wasm32` over the runtime's `fetch`, and on a host over any client
you like. The only transport it ships is `NullTransport`, which fails every
request (*The transport is yours*).

One caveat: its live conformance run uses a host HTTP client kept in the crate's
tests. No `wasm32` transport has run the suite yet.

### In an ordinary native process

**Choose among `MemoryEventStore`, `SqliteEventStore` and
`PostgresEventStore`.** The next section decides between them.

Two limits apply here. `SqliteEventStore` does not build for
`wasm32-unknown-unknown` (its crate root, *The shape this crate represents*).

Both SQL stores expect the tokio async runtime. SQLite's reads run on a
tokio blocking task. `PostgresEventStore` has to be called from inside a tokio
runtime with its drivers enabled, as the one `#[tokio::main]` builds is,
because the store runs its work on the runtime it is called from (the Postgres
README's *Two roles, one pool*).

## Then count the writers

The next question is how many writers share the log, and where they are. The
answer decides how much the store has to do to keep their appends in a single
order.

### One process, nothing has to survive a restart

**Use `MemoryEventStore`.** It is meant for tests, examples and a first
prototype. It keeps the log in a `Vec` in memory, so a restart loses it.

The conformance suite knows this. The memory store declines the "reopen" test
with that reason, so `acknowledged_writes_survive_a_reopen` reports a skip
rather than a pass
([ES-35](../spec/SPECIFICATION.md#es-35--acknowledged-writes-survive-a-reopen-where-durability-is-claimed)).

### Writers on one machine, sharing a file

**Use `SqliteEventStore::open(path)`.** Every handle opened onto the same file
sees the same log and the same conditions
([ES-34](../spec/SPECIFICATION.md#es-34--two-handles-share-one-consistency-boundary)),
so separate processes can share one log.
[The tickets example](../examples/tickets-over-http/src/lib.rs) runs an HTTP API
and a projection runner as two processes over one file.

Writers take turns. Each append takes SQLite's write lock with
`BEGIN IMMEDIATE`. If another writer holds it, the append waits up to
`BUSY_TIMEOUT_MS`, which is 15,000 ms, and is then refused as
`AppendError::Busy` (*Durability, as configured rather than as hoped*).

### Writers on many machines

**Use `PostgresEventStore::new(pool)`**, and call `migration::apply(&pool)` once
before it to create its tables.

Unlike SQLite, Postgres writers do not wait in line behind one another. That
gives you more throughput, but it creates a subtle problem. Positions come from
a Postgres sequence (`nextval()`), which hands out numbers outside the
transaction that uses them. So position 11 can become visible to readers before
position 10 does, and a reader that saw 11 could miss 10 for good.

The store prevents that, and keeps the rule that readers see positions in order
([ES-10](../spec/SPECIFICATION.md#es-10--position-order-is-visibility-order)).
Each read admits only events written by transactions older than every
transaction still open, a *visibility frontier* worked out from Postgres's own
transaction ids (`pg_snapshot_xmin`, over `xid8`). `head()` reports the highest
position among those events. Why the store works this way is recorded in
[ADR-0024](../.kb/decisions/0024-position-visibility-mechanism.md). That has three costs, and you take all three:

- **`head()` can lag.** It reports the highest position below the frontier, which can sit below the newest
  position the store has handed out (ES-10).
- **You may not read your own write straight away.** A read made right after a
  successful append may not include that append's events yet (ES-10).
- **One slow transaction holds every reader back.** Readers lag by as long as
  the longest write transaction open anywhere on the Postgres cluster, in any
  database. A long migration or a connection left idle inside a transaction is
  enough (*The problem this crate is here to solve*). This is a deliberate
  choice: keeping the frontier global is what lets a read model keep a single
  checkpoint position.

### Writers that cannot hold a connection

**Use `NeonEventStore::new(transport, config)`**, and send
`migration::request(migration::MIGRATION_1, &config)` through your transport
once before it.

Behind the endpoint is PostgreSQL, so the same visibility frontier applies, with
the same three costs (its crate root, *Status*).

There is one more thing to know. A read is guaranteed to be ordered before a
later append
([ES-11](../spec/SPECIFICATION.md#es-11--a-read-is-a-snapshot)) only when both
go through clones of the same transport value. Two separately built transports
are not ordered against each other, and no conformance test can detect the
difference, so keep one transport and clone it (*ES-11, and the fence that
meets it*).

### Inside a Durable Object

**A Durable Object has exactly one writing thread.** The object is a
single-threaded actor that owns its storage, so its store never has two appends
running on different threads. Requests can interleave at `.await` points, so a
decision still needs its condition, but `append` awaits nothing between checking
the condition and writing.

For the same reason, the conformance suite's concurrency tests are not run
against it. Those tests run competing writers on separate OS threads, and this
store cannot be moved between threads (it is not `Send`) (its crate root, *The
concurrency family is not invoked, and that is a reason rather than a
silence*).

## Put the checkpoint where the read model is

If you will build read models, you also need a projection store: the place that
holds your read model's rows and the projection's **checkpoint**, the position
of the last event it applied.

A projection store commits the rows and the checkpoint together, as one
transaction
([PS-1](../spec/SPECIFICATION.md#41-status-and-what-would-freeze-the-port)).
That is what stops a crash from leaving rows that disagree with their
checkpoint. So choose the projection store by the database your read model
lives in. Each projection writes to exactly one projection store
([PS-10](../spec/SPECIFICATION.md#43-the-write-seam)).

The event log can be in a different database from the read model, but not in a
different process. The typed layer's `run_projection` (behind its
`unstable-projection` feature) takes the event store and the projection store
as two arguments to one call, so the process that runs it has to hold both.

| Event store | Projection store in the same crate | Feature |
| --- | --- | --- |
| `MemoryEventStore` | `MemoryProjectionStore` | `memory`, on by default |
| `SqliteEventStore` | `SqliteProjectionStore` | `projection-store` |
| `PostgresEventStore` | `PostgresProjectionStore` | `projection-store` |
| `NeonEventStore<T>` | `NeonProjectionStore<T>` | `projection-store` |
| `CloudflareEventStore` | none | none |

Each SQL projection store needs its checkpoint table before the first commit:

- **SQLite:** `SqliteProjectionStore::open` creates it every time it connects.
- **Postgres:** run `migration::apply_projection(&pool)`. It applies
  `migration::MIGRATION_2`, separately from the event log's `migration::apply`
  (its README, *Two roles, one pool*).
- **Neon:** send `migration::request(migration::MIGRATION_2, &config)` through
  your transport.

### A Durable Object has no projection store of its own

`happenstance-cloudflare` does not include a projection store (its crate root,
*Status: what publication this crate is ready for, and what it is not*).
`CloudflareEventStore` exists only inside the object, on `wasm32`, so a runner
reading it has to run in the object too. Only two projection stores can run
there:

- **`MemoryProjectionStore`.** It holds the read model and the checkpoint in the
  object's memory (its own documentation), so both are gone whenever that
  memory is. With no checkpoint, a runner replays the log from the start
  ([PS-20](../spec/SPECIFICATION.md#46-reset)), the same replay
  [Rebuild a read model](rebuild-a-read-model.md) walks through.
- **`NeonProjectionStore<T>`, over a `wasm32` transport you write.** The crate
  compiles for `wasm32`, but its live conformance run uses a host client, so no
  `wasm32` transport has run the projection suite.

`SqliteProjectionStore` cannot run in the object, because its crate does not
build for `wasm32`. Neither can `PostgresProjectionStore`: its crate supports
only native hosts, and nothing in this repository's build checks it on
`wasm32`.

You could read the object's log from outside, for example through an HTTP API
the object serves. No crate here provides or documents that, so if you build
it, the conformance suite has not checked it.

### Two more limits on the SQL projection stores

- **`apply` cannot read back what it wrote earlier in the same batch.** The
  batches of `SqliteProjectionStore`, `PostgresProjectionStore` and
  `NeonProjectionStore` have no way to read, so their
  `batch_reads_reflect_pending_writes` test reports a skip
  ([PS-12](../spec/SPECIFICATION.md#44-read-your-writes);
  `happenstance-postgres`'s *What the projection batch is, and what it costs
  you*).
- **Point `NeonProjectionStore` at a read-write primary, never a read
  replica.** A replica can report an older checkpoint, and a runner that
  resumes from it applies events a second time
  ([PS-38](../spec/SPECIFICATION.md#47-checkpoint-semantics); *Point the
  projection store at the primary*). No conformance test can detect a replica,
  because the suite runs against one endpoint.

## Bind the bare trait, so the choice stays one line

You will probably start on the memory store and deploy on another, so write
your code so that switching is a one-line change.

happenstance defines the store interface twice. `EventStore`, the "bare"
trait, places no threading requirement on a store. `SendEventStore` is the same
trait plus a promise that the store is `Send`: Rust's marker for values that
can be moved to another thread. Every `SendEventStore` is also an
`EventStore`, but not the other way round.

So write the code that takes a store as generic over `EventStore`
([ES-1](../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one)).
A function bound on `EventStore` accepts all five stores. Moving to the store
you deploy then changes the line that builds the store and nothing else.

Import only `EventStore` in that module. With both names in scope, a call such
as `store.read(…)` matches a method on each trait, and Rust refuses it as
ambiguous with error E0034.

### Test against more than one store

Run that code against more than one store before you choose. The memory store
has a habit no other store promises, and a test that passes only on the memory
store may be relying on it.

The `happenstance-testkit` crate has two stand-ins for what the memory store
cannot show you:

- `GappyMemoryStore` is a correct store whose positions skip numbers.
- `FaultyStore` implements only the bare `EventStore` trait, as
  `CloudflareEventStore` and `NeonEventStore` do.

This program runs one function on the memory store and on both stand-ins:

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

The count is the same on every store, and the distance is not. That is the
habit: the memory store happens to number its events 1, 2, 3 with no gaps, but
any store may leave gaps between positions
([VT-11](../spec/SPECIFICATION.md#vt-11--positions-are-unique-strictly-increasing-in-assignment-order-and-may-have-gaps)).
A test that passes only on dense positions is testing an accident. Count events
by reading them, never by subtracting positions.

### Where you do need `Send`

Bind `SendEventStore` only where the store has to cross to another thread, as
it does when you hand it to `tokio::spawn`. That bound rules out the Durable
Object and Neon stores, and the compiler says so with error E0277:

```rust,compile_fail,E0277
use happenstance::{MemoryEventStore, SendEventStore};
use happenstance_testkit::FaultyStore;

/// Bound on the `Send` flavour, because it hands the store to another thread.
fn spawnable<S: SendEventStore>(_store: &S) {}

fn main() {
    spawnable(&FaultyStore::new(MemoryEventStore::new()));
}
```

That example is meant to fail to compile. Stable Rust's documentation tests do
not check *which* error a `compile_fail` example produces, so it would also
pass if it failed for some unrelated reason. The program below is the control:
the same code with a store that does implement `SendEventStore`, and it
compiles:

```rust
use happenstance::{MemoryEventStore, SendEventStore};

/// Bound on the `Send` flavour, because it hands the store to another thread.
fn spawnable<S: SendEventStore>(_store: &S) {}

fn main() {
    spawnable(&MemoryEventStore::new());
}
```

## Check the limits before you need them

Each store has limits that are easy to miss until a production batch hits them.
Check these against the largest events and appends your application makes.

### Capacity

An append over a store's limit is refused as `AppendError::ExceedsStoreLimit`.
It is never quietly cut short
([VT-25](../spec/SPECIFICATION.md#vt-25--a-capacity-refusal-is-distinguishable-from-a-store-failure)):

| Store | Bytes of data per event | Tags per event | Events per append | Bytes of metadata per event |
| --- | --- | --- | --- | --- |
| `SqliteEventStore` | 1,048,576 | 128 | 256 | none declared |
| `PostgresEventStore` | 1,048,576 | 128 | 256 | none declared |
| `NeonEventStore<T>` | 131,072 | 128 | 128 | none declared |
| `CloudflareEventStore` | 1,048,576 | 1,024 | 1,024 | 524,288 |
| `MemoryEventStore` | none declared | none declared | none declared | none declared |

`SqliteEventStore`, `PostgresEventStore` and `NeonEventStore` publish these
limits as the constants `MAX_EVENT_DATA_LEN`, `MAX_TAGS_PER_EVENT` and
`MAX_EVENTS_PER_BATCH`. `CloudflareEventStore` has no public constants for
them. Its row comes from its README, *The limits this store declares*, which
calls them a stated policy rather than a measured limit of the platform.

If your events have to fit every store, stay within
`MIN_SUPPORTED_EVENT_DATA_LEN`, `MIN_SUPPORTED_TAGS_PER_EVENT` and
`MIN_SUPPORTED_EVENTS_PER_BATCH`. These are floors every store accepts
([VT-21](../spec/SPECIFICATION.md#vt-21--min_supported_event_data_len-is-a-floor-of-65536-bytes),
[VT-22](../spec/SPECIFICATION.md#vt-22--min_supported_tags_per_event-is-a-floor-of-64),
[VT-24](../spec/SPECIFICATION.md#vt-24--min_supported_events_per_batch-is-a-floor-of-128)).
Metadata has no such floor. A store may refuse any amount of it, as
`StoreLimit::MetadataLen`, so keep metadata small if an event has to fit
everywhere.

A Neon read has one more ceiling. Its whole response has to fit in
`MAX_RESPONSE_BYTES`, 64 MiB, because a one-shot HTTP request cannot stream
results a page at a time (*Limits this store enforces*).

### Busy refusals

A store may refuse an append because it was too busy, and when it does, it has
written nothing
([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).
Which stores do this, and when:

- **SQLite** refuses when its busy timeout runs out.
- **Postgres and Neon** refuse when a serialisation failure outlasts their own
  internal retries, which only an append with a condition can run into (the
  Postgres crate root's *What this store costs a caller*; `NeonEventStore`'s
  `SERIALISATION_ATTEMPTS`).
- **The memory store and the Durable Object store** never refuse as busy. The
  memory store queues its writers inside the process, and an object has only
  one writer, so neither has anything to wait out (`AppendError::Busy`'s own
  documentation; `CloudflareEventStoreError`'s *Why this store never reports
  `AppendError::Busy`*).

The typed layer's command loop retries a busy refusal within the same bound as
a violated condition. [Retry a command](retry-a-command.md) shows how to set
that bound.

### Positions

Every store may leave gaps between positions, as shown above. The Durable
Object store has one more limit: it cannot hold a position above 2^53 − 1,
because Workers SQL returns integers as JavaScript numbers. A stored position
past that bound is reported as `CloudflareEventStoreError::StoredPosition`
rather than silently rounded (*The limits this store declares*).

## How this page is checked

The three programs above are compiled and run by this repository's tests. The
rules this page relies on are checked by the conformance suite every store
passes, including `two_handles_observe_each_others_appends` (ES-34),
`head_is_the_highest_visible_position` (ES-10, which asserts a bound rather than
an equality, for the reason given above), `commit_is_atomic_with_the_read_model`
(PS-1), `positions_are_strictly_monotonic` (VT-11),
`append_reports_exceeded_store_limits` (VT-25) and
`a_busy_append_left_nothing_behind` (ES-43).
