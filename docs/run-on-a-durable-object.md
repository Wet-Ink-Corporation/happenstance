# Run on a Durable Object

> **Answers:** `how-to` — How do I run the event store inside a Cloudflare Durable Object?

This page is for you if you build on Cloudflare Workers and want your event log
to live inside a Durable Object. By the end you will have a store built inside
your object's class, decision code that runs there unchanged from your host
tests, and a clear picture of what the store refuses and what has not yet been
proven about it.

Why a Durable Object? Each object is a single-threaded actor that owns its own
SQL storage. Its code never runs on two threads at once, so no two writes can
race at the storage level, which makes it a natural home for a log. It is
*re-entrant*, though: while one request is waiting at an `.await`, another can
start, so a decision still guards its write with a condition. The store from
[`happenstance-cloudflare`](../crates/happenstance-cloudflare/README.md),
`CloudflareEventStore`, writes the log to that storage, compiled for
`wasm32-unknown-unknown`.

If you have not yet decided that a Durable Object is where your log belongs,
[choose a store](choose-a-store.md) compares it with the other four stores first.

Everything below comes from the crate's README, its crate-root documentation, or
the documentation of the item named. Where a name appears *in italics*, it is a
section heading in one of those, so you can read more there.

## Build the store in the object

A `#[durable_object]` class is constructed with a `worker::State`. You build the
store from that state, and create its tables before the first append. Three
steps:

- **Construct.** Call
  `CloudflareEventStore::new(SqlStorage::from_state(&state))`.
  `SqlStorage::from_state` is shorthand for
  `SqlStorage::new(state.storage().sql())`, the spelling in the README's *Use*
  section. The store never creates storage of its own: both of its
  constructors, this one and `namespaced` below, take the object's storage as
  an argument.
- **Migrate.** Call `migrate()`. It creates the schema with
  `CREATE … IF NOT EXISTS` throughout, so it is safe to call every time the
  object starts. Its first run also mints this object's `StoreId` and keeps it
  in the object's own tables, where every later handle reads it back. It is
  synchronous, and returns a `CloudflareEventStoreError` if a statement fails.
- **Use it.** From here the store takes the same calls as any other store:
  `read_decision_model`, then `append` under an `AppendCondition`. The README's
  *Use* section shows a whole enrolment decision written that way.

The README's example is compiled by the crate's tests but marked `no_run`. Off
the Workers target, every `worker` binding is a stub that panics, so the code can
be type-checked on a laptop but only runs inside an object.

Cloning the store gives you a second handle onto the same log, not a copy of it,
because cloning a `SqlStorage` points at the same storage.

### A working class to read

The repository's test harness for this crate contains a real class, split over
two files. It is never published, so read it for its shape and ignore its
request routing:

- [Its object](../harness/workerd/src/object.rs) is the class: a struct holding
  the `State` it was constructed with, and a `fetch` that handles each request.
- [Its fixture](../harness/workerd/src/fixture.rs) is where a store is built:
  `SqlStorage::from_state(state)`, then `CloudflareEventStore::namespaced`, then
  `migrate()`. It uses `namespaced` rather than `new` because it keeps several
  logs in one object, which the next section explains.

## Keep several logs in one object

By default, one object holds one log. That is often what you want, but sometimes
you want several separate logs in the same object, for example one per tenant.

To do that, build each store with
`CloudflareEventStore::namespaced(sql, &namespace)`, where `namespace` is a
`TableNamespace`. Each namespace keeps its log in its own tables. Two namespaces
share no rows, no position sequence and no incarnation (the per-log identity
every `EventId` carries), so an `EventId` written in one is never found in
another. Two handles built under the same namespace are two handles
onto one log.

`migrate()` runs per namespace, so call it on each store before its first
append. `CloudflareEventStore::namespaced`'s own documentation carries a
compiled example.

The namespace becomes part of a SQL table name. Rather than trying to quote it
safely, `TableNamespace::new` only accepts names that are already safe:

- lowercase ASCII letters, digits and `_`;
- starting with a letter, and not beginning with `sqlite`;
- at most `TableNamespace::MAX_LEN` bytes, which is 64.

A name that breaks a rule comes back as an `InvalidTableNamespace` that names
the first rule it broke.

## Keep the store on the object's turn

This is the one part of the page that is specific to Rust, and it is worth
understanding before you write any code.

**What `Send` means.** In Rust, a type is `Send` if it is safe to move to
another thread. The compiler works this out for you and refuses, at compile
time, any code that would move a non-`Send` value across threads. Most types
are `Send`. A type that is not is written `!Send`.

**Why this store is `!Send`.** Inside Workers, the crate holds every JavaScript
value behind an `Rc`, a reference-counted pointer that is cheap because it does
no thread synchronisation. That same fact makes `Rc` `!Send`, whatever it holds,
and so `CloudflareEventStore` is `!Send` and `!Sync` too. Its error type is
`!Send` as well, which the contract allows
([ES-6](../spec/SPECIFICATION.md#es-6--error-carries-no-send-or-sync-bound-and-the-strength-is-a-marker)).

**Why there are two traits.** happenstance derives two flavours of the store
contract from one definition. `EventStore`, the bare flavour, asks for no
`Send`. `SendEventStore` also promises that the store and its futures are `Send`,
so a multi-threaded runtime can take them. Every `SendEventStore` is also an
`EventStore`, but not the other way round.

`CloudflareEventStore` can only implement the bare `EventStore`.

### Write decision code against `EventStore`

Write your decision functions generic over `EventStore`, the weaker of the two
([ES-1](../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one)).
Then one function runs in the object and, unchanged, against the in-memory store
in a host test. You test your rules on your laptop, and deploy the same code.

The program below shows this. This page's examples cannot depend on the
Cloudflare crate, so it uses a stand-in, `ObjectLocal`, with the same shape as
`CloudflareEventStore`: its storage sits behind an `Rc`, and it implements only
the bare flavour.

```rust
use std::rc::Rc;

use happenstance::futures_core::Stream;
use happenstance::{AppendCondition, AppendError, Event, EventId, EventStore};
use happenstance::{MemoryEventStore, Query, QueryItem, ReadOptions, SequencePosition};
use happenstance::{SequencedEvent, Tags, read_decision_model};

type Failure = Box<dyn core::error::Error>;

/// A stand-in with the Durable Object store's shape: its storage sits behind
/// an `Rc`, so it is `!Send`, and it implements only the bare flavour.
#[derive(Clone)]
struct ObjectLocal(Rc<MemoryEventStore>);

impl EventStore for ObjectLocal {
    type Error = <MemoryEventStore as EventStore>::Error;

    fn read(
        &self,
        query: &Query,
        options: ReadOptions,
    ) -> impl Stream<Item = Result<SequencedEvent, Self::Error>> {
        self.0.read(query, options)
    }

    async fn append(
        &self,
        events: &[Event],
        condition: Option<&AppendCondition>,
    ) -> Result<SequencePosition, AppendError<Self::Error>> {
        self.0.append(events, condition).await
    }

    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error> {
        self.0.head().await
    }

    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error> {
        self.0.contains_event_id(id).await
    }
}

/// Takes a seat on a course of two. Bound on the bare flavour, so it takes
/// the object's store and the memory store alike.
async fn enrol<S: EventStore>(store: &S, student: &str) -> Result<bool, Failure> {
    let course = Tags::from_pairs([("course", "c1")])?;
    let seats = Query::from_item(QueryItem::new(["StudentEnrolled"], course.clone())?);
    let (taken, last_seen) = read_decision_model(store, &seats).await?;
    if taken.len() >= 2 {
        return Ok(false);
    }
    let seat = Event::new("StudentEnrolled", student.as_bytes().to_vec())?.with_tags(course);
    let guard = AppendCondition::new(seats.clone()).after_every_guard_opt(last_seen);
    store.append(&[seat], Some(&guard)).await?;
    Ok(true)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Failure> {
    let object = ObjectLocal(Rc::new(MemoryEventStore::new()));
    let alias = object.clone();
    assert!(enrol(&object, "s1").await?);
    assert!(enrol(&alias, "s2").await?);
    // The clone is a second handle onto the same log, so the course is full.
    assert!(!enrol(&object, "s3").await?);

    // The same function, unchanged, against the memory store.
    let host = MemoryEventStore::new();
    assert!(enrol(&host, "s1").await?);
    Ok(())
}
```

In your object, you pass your `CloudflareEventStore` to `enrol` in the same
place. Await it in the handler that received the request, on the object's own
turn.

### Do not hand the store to anything that needs `Send`

Anything that might move work to another thread will refuse the store. A
`SendEventStore` bound refuses it, and so does `tokio::spawn`, which may run the
task on a different thread. This is the compiler protecting you, not a
limitation to work around.

The program below captures an `Rc` handle in a spawned task. The compiler
refuses it with *future cannot be sent between threads safely*, because the
captured `Rc<MemoryEventStore>` is not `Send`:

```rust,compile_fail
use std::rc::Rc;

use happenstance::{EventStore, MemoryEventStore, Query, ReadOptions, collect};

/// Counts the events in the log. Bound on the bare flavour.
async fn count<S: EventStore>(store: &S) -> Result<usize, S::Error> {
    Ok(collect(store.read(&Query::all(), ReadOptions::new())).await?.len())
}

#[tokio::main]
async fn main() {
    let object = Rc::new(MemoryEventStore::new());
    let task = tokio::spawn(async move { count(&*object).await });
    assert!(matches!(task.await, Ok(Ok(0))));
}
```

A `compile_fail` example passes however it fails to compile, so on its own it
would also pass if it failed for some unrelated reason. The control below is the
same program with the handle behind an `Arc`, the thread-safe counterpart of
`Rc`. It compiles and runs, which shows the `Rc` was the only problem:

```rust
use std::sync::Arc;

use happenstance::{EventStore, MemoryEventStore, Query, ReadOptions, collect};

/// Counts the events in the log. Bound on the bare flavour.
async fn count<S: EventStore>(store: &S) -> Result<usize, S::Error> {
    Ok(collect(store.read(&Query::all(), ReadOptions::new())).await?.len())
}

#[tokio::main]
async fn main() {
    let object = Arc::new(MemoryEventStore::new());
    let task = tokio::spawn(async move { count(&*object).await });
    assert!(matches!(task.await, Ok(Ok(0))));
}
```

### What staying on one turn costs you

Nothing. A Durable Object's code runs on one thread and owns its storage, so
there is no second thread to race against. Requests can still interleave at
`.await` points, which is why your decision keeps its condition. But `append`
itself awaits nothing between checking the condition and writing the batch, so
no other request can slip in there, and it never reports `AppendError::Busy`. `CloudflareEventStoreError`'s documentation
explains this under *Why this store never reports `AppendError::Busy`*.

For the same reason, the conformance suite's concurrency tests, which race
several threads against one store, are not run against this store. The crate
root explains why under *The concurrency family is not invoked, and that is a
reason rather than a silence*.

## Know what the store refuses

Every store has limits. Knowing these before you deploy saves you from
discovering them in production.

### Capacity

The store checks each batch against four ceilings before it sends any SQL. A
batch over one is refused as `AppendError::ExceedsStoreLimit`, naming the
`StoreLimit` it crossed, so you can tell a capacity refusal apart from a storage
failure
([VT-25](../spec/SPECIFICATION.md#vt-25--a-capacity-refusal-is-distinguishable-from-a-store-failure)).
The README states the ceilings under *The limits this store declares*:

| Limit | Declared |
| --- | --- |
| payload (`data`) | 1,048,576 bytes (1 MiB) |
| tags per event | 1,024 |
| events per append | 1,024 |
| `metadata` per event | 524,288 bytes (512 KiB) |

These numbers are the store's chosen policy, not a measured physical limit. They
were set under Cloudflare's documented row limit of 2 MB, read as 2,000,000
bytes, and the real wall was never located. An event at all four ceilings at once
still fits that limit. Metadata over its ceiling is refused as `StoreLimit::MetadataLen`. That
limit has no guaranteed minimum, so another store may accept less metadata. The
crate publishes no constants for them, so copy the numbers from the README rather
than naming them in code.

### Query width

A wide query is not refused, but it may cost several statements. `workerd` caps
one SQL statement at 5 compound `SELECT` terms, 100 bound parameters and 100,000
bytes. To stay inside those caps, the store splits a wide query across several
statements and merges the results.

The two widths it splits at are public:
`CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT`, which is 5, and
`CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT`, which is 90. A
128-item query takes 26 statements here, and every store must accept at least
that many items
([VT-23](../spec/SPECIFICATION.md#vt-23--min_supported_query_items-is-a-floor-of-128)).

To see what a decision's query will cost before you deploy it, call
`CloudflareEventStore::planned_statement_count(&query)`. It counts the plan the
read path will use (the crate root, *The query widths this store does not
refuse*).

### Positions

Workers SQL hands integers back through a JavaScript number, which can only
hold whole numbers exactly up to 2^53 − 1. A stored position past that bound is
reported as `CloudflareEventStoreError::StoredPosition` rather than silently
rounded.

### A failed batch

A batch of events must land whole or not at all
([ES-18](../spec/SPECIFICATION.md#es-18--atomicity)). On a Durable Object, the
platform's own commit does not give you that here. The object's turn commits
when the handler returns normally, and the store turns every failure into an
`Err` and returns normally, so the turn would happily commit a half-written
batch.

So the store makes the batch all-or-nothing itself. A batch's rows only become
events at its last statement, which stamps every row with its event identity at
once. If the batch fails before that stamp, its rows are invisible to every
read. The store then sweeps those rows away, but that sweep is tidying up, not
what makes the batch safe (the `event_store` module, *Why the whole object is
one consistency boundary*).

If the sweep itself fails, `append` reports
`CloudflareEventStoreError::PartialBatch`. Despite the name, the batch did not
land, and that variant's own documentation says a retry is safe. Older wording
in the crate root says the opposite; that is tracked in
[issue #257](https://github.com/Wet-Ink-Corporation/happenstance/issues/257).

### A dropped append

In async Rust, dropping a future before it finishes normally cancels the work.
The contract leaves open what a dropped `append` does, and asks each store to
say
([ES-23](../spec/SPECIFICATION.md#es-23--cancellation-outcome-is-unspecified-and-that-is-the-contract)).

This store's answer is simple: `append` contains no `.await`, so it has run to
completion by the time it is first polled, and dropping it cannot cancel it.
Code written generic over `EventStore` should not rely on that, because other
stores answer differently (the `event_store` module, *Cancellation*).

## Restore the object to an earlier point

Cloudflare lets you roll a Durable Object's storage back to a bookmark: take one
with `getBookmarkForTime` or `getCurrentBookmark`, then call
`onNextSessionRestoreBookmark(bookmark)` and restart the object with
`ctx.abort()`. The restore rolls back everything the store keeps, including the
row that holds its `StoreId` and the counter that hands out positions.

That combination is a hazard. The restored object hands out positions it already
gave to events written after the bookmark, under the same `StoreId`, so it issues
`EventId`s that already name other events. A replication peer that saw the
originals drops the new events as duplicates, and nothing reports an error.

So after a restore, and before the object accepts its first append, call
`remint_identity` on every log in the object: the store `new` builds, and one
for each `TableNamespace` you use. It returns the new `StoreId`, which is worth
logging.

The object cannot tell it was restored. The platform sends no signal, and
anything the store could write to remember is rolled back too. So whatever
performed the restore has to make this call, and has to keep appends away until
it has. Events from before the bookmark keep their identities, and a re-mint
when no restore happened does no harm: it only starts a new origin. The
procedure is in the
[crate README](../crates/happenstance-cloudflare/README.md), *After a
point-in-time restore*.

## Keep the read model somewhere it can commit

A read model, or projection, is a view you build from the log, such as a table
of course capacities. This crate does not provide a store for read models: it
does not implement the projection port at all.

[Choose a store](choose-a-store.md#put-the-checkpoint-where-the-read-model-is)
names the two projection stores that can run inside the object, and what each
costs. Read it before you build a read model here.

## See how the crate checks itself

You may want to know how much to trust this store. Like every store in the
project, it has to pass the conformance suite: the shared set of tests that
defines what a correct store does.

Its conformance test is one macro call, `event_store_conformance!`, with
`emit = happenstance_testkit::emit_wasm` and a `CloudflareFixture`, in
[`tests/durable_object_conformance.rs`](../crates/happenstance-cloudflare/tests/durable_object_conformance.rs).
`emit_wasm` wraps each test for `wasm-bindgen-test`, because the caller chooses
how the tests are run
([CF-23](../spec/SPECIFICATION.md#64-runtime-independence)).

The project's main build runs every event-store test that way on
`wasm32-unknown-unknown`, against a stand-in runtime backed by `node:sqlite`. The
fixture declines no capability, so no test is skipped.

The same tests also run in a real Durable Object, through
[`harness/workerd`](../harness/workerd/README.md): once under a local `workerd`,
and once on an object deployed to Cloudflare. That covers what the stand-in
cannot: the isolate, the I/O gate and the platform's own SQLite limits.

If you are writing a store of your own for a runtime like this one,
[pass the conformance suite](pass-conformance-suite.md) shows how to choose the
test wrapper, `emit_wasm` among them.

## What has not been proven yet

The crate's README is candid that this store has the thinnest evidence of the
four published adapters. Before you rely on it, know these gaps:

- **Eviction and hibernation have never been tested.** The `workerd` harness runs
  each test inside a single request, and its "reopen" is a fresh storage handle.
  What a real isolate restart does to an acknowledged write has not been
  observed, and the harness does not reach the memory ceiling either
  (`harness/workerd/README.md`, *What it discharges, and what it does not*).
- **A real bookmark restore runs only on a deployed object.** The re-mint is
  tested against a simulated restore in the crate's Node host. The `workerd`
  job also restores a deployed object from a bookmark and checks whether an
  `EventId` is reissued, with and without the re-mint. A wrong answer fails the
  job; a restore that never lands is only recorded. That job is not a required
  check, and local `workerd` cannot run it, because it does not implement a
  restore.
- **The three capacity ceilings are policy.** The physical wall behind them was
  never located.
- **The concurrency and model tests do not run here.** The concurrency tests
  need OS threads and a `Send` store. The model tests, which generate random
  operation sequences, sit behind the testkit's `proptest` feature, which is not
  available on this target.
- **No read model is checked by this crate.** It has no projection store, so
  whatever you keep beside the log is checked by whichever crate provides it, if
  any. [Choose a store](choose-a-store.md#put-the-checkpoint-where-the-read-model-is)
  says what each option has and has not run.
- **Wiring the class into a Worker is not covered.** The Worker's entry point,
  its `wrangler.toml` and the class's bindings belong to Cloudflare's platform,
  and no crate here documents them. [`harness/workerd`](../harness/workerd/README.md)
  has a working set, and its README says why its entry point is JavaScript.
