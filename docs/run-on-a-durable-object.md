# Run on a Durable Object

> **Answers:** `how-to` — How do I run the event store inside a Cloudflare Durable Object?

[`happenstance-cloudflare`](../crates/happenstance-cloudflare/README.md) turns one
Durable Object into an event store. `CloudflareEventStore` writes to the object's
own SQL storage, on `wasm32-unknown-unknown`, inside Workers. If you have not yet
decided that a Durable Object is where your log belongs,
[Choose a store](choose-a-store.md) compares it with the other four stores first.

This page starts from a `#[durable_object]` class and gets a store into it. What
it says comes from the crate's README, its crate-root documentation or the
documentation of the item it names. A name in italics is a heading in one of
those, and the sentence says which.

## Build the store in the object

A `#[durable_object]` class is constructed with a `worker::State`. Build the
store from it, and call `migrate()` before the first append:

- **Construct.** `CloudflareEventStore::new(SqlStorage::from_state(&state))`.
  `SqlStorage::from_state` is shorthand for
  `SqlStorage::new(state.storage().sql())`, the spelling the README's *Use*
  section shows. Both of the store's constructors, this one and `namespaced`
  below, take the object's storage by injection, and nothing in the crate makes
  storage of its own (`CloudflareEventStore::new`'s and `SqlStorage::new`'s own
  documentation).
- **Migrate.** `migrate()` creates the schema with `CREATE … IF NOT EXISTS`
  throughout, so it is safe to call every time the object starts. Its first run
  also mints this object's `StoreId` and keeps it in the object's own tables,
  where every later handle reads it back (the `event_store` module, *Schema*).
  It is synchronous, and it returns a `CloudflareEventStoreError` if a statement
  fails.
- **Use it.** From here the store takes the same calls as any other:
  `read_decision_model`, then `append` under an `AppendCondition`. The README's
  *Use* section is a whole enrolment decision written that way. The crate's own
  doctests compile it, marked `no_run`, because off the target every `worker`
  binding is a stub that panics.

A clone of the store is a second handle onto the same log, not a copy of it:
cloning a `SqlStorage` aliases the object's storage (its own documentation).

The conformance harness has a working class, split across two files. It is a
test harness and is never published, so read it for shape and leave the
routing:

- [Its object](../harness/workerd/src/object.rs) is the class: a struct holding
  the `State` it was constructed with, and a `fetch` that handles each request.
- [Its fixture](../harness/workerd/src/fixture.rs) is where a store is built:
  `SqlStorage::from_state(state)`, then `CloudflareEventStore::namespaced`, then
  `migrate()`. It uses `namespaced` rather than `new` because it keeps one log
  per fixture instance in one object, which the next section explains.

## Keep several logs in one object

One object holds one log by default. To hold several, build each store with
`CloudflareEventStore::namespaced(sql, &namespace)`, where `namespace` is a
`TableNamespace`. Each namespace keeps its log in tables of its own, and two
namespaces share no rows, no position sequence and no incarnation, so an
`EventId` minted in one is never found in another. Two handles built under the
same namespace are two handles onto one log. `migrate()` runs per namespace, so
call it on each store before its first append (`CloudflareEventStore::namespaced`'s
own documentation, which carries a compiled `no_run` example).

`TableNamespace::new` validates the name rather than quoting it, because the name
becomes part of a SQL identifier. It accepts lowercase ASCII letters, digits and
`_`, starting with a letter, not beginning `sqlite`, and at most
`TableNamespace::MAX_LEN` bytes, which is 64. A refused name comes back as an
`InvalidTableNamespace` naming the first rule it breaks (`TableNamespace::new`'s
*Errors*). `TableNamespace`'s documentation suggests one log per tenant; the
`workerd` harness uses it to give each fixture instance its own log in one
object.

## Keep the store on the object's turn

`CloudflareEventStore` is `!Send` and `!Sync`. The crate holds every JavaScript
value behind an `Rc`, and `Rc` is `!Send` whatever it holds (the crate root,
*Findings*, 1). So the store implements the bare `EventStore` flavour and not
`SendEventStore` (*What this crate is for*), and its error type is `!Send` too
([ES-6](../spec/SPECIFICATION.md#es-6--error-carries-no-send-or-sync-bound-and-the-strength-is-a-marker)).

Write the code that decides generic over `EventStore`
([ES-1](../spec/SPECIFICATION.md#es-1--one-definition-two-flavours-and-generic-code-binds-the-weaker-one)).
What that buys here is one function that runs in the object and, unchanged,
against the memory store in a host test. The program below shows that with a stand-in, because this page's examples
cannot depend on the Cloudflare crate. `ObjectLocal` holds its storage behind an
`Rc` and implements only the bare flavour, which is the shape of
`CloudflareEventStore`:

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
    let guard = AppendCondition::new(seats.clone()).after_opt(last_seen);
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

In the object, `enrol(&store, …)` takes your `CloudflareEventStore` in the same
place. Await it in the handler that received the request, on the object's own
turn.

Do not hand the store to anything that needs `Send`. A `SendEventStore` bound
refuses it, and so does `tokio::spawn`, which may move the task to another
thread. The program below captures an `Rc` handle in a spawned task, and the
compiler refuses it: *future cannot be sent between threads safely*, because
the captured `Rc<MemoryEventStore>` is not `Send`:

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

A `compile_fail` fence passes whatever stops it compiling, so the fence above
would also pass if it failed for another reason. This control is the same
program with the handle behind an `Arc`, which is `Send`, and it compiles and
runs:

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

You lose nothing by keeping the store on one turn. A Durable Object is a
single-threaded actor that owns its storage, so there is no second writer to
race. `append` awaits nothing between checking the condition and writing the
batch, so it never reports `AppendError::Busy` (`CloudflareEventStoreError`'s
*Why this store never reports `AppendError::Busy`*). The conformance suite's
concurrency family is not run against it, for the same reason (the crate root,
*The concurrency family is not invoked, and that is a reason rather than a
silence*).

## Know what the store refuses

**Capacity.** The store checks a batch against three ceilings before it issues
any SQL. A batch over one is refused as `AppendError::ExceedsStoreLimit`, naming
the `StoreLimit` it crossed
([VT-25](../spec/SPECIFICATION.md#vt-25--a-capacity-refusal-is-distinguishable-from-a-store-failure)).
The README's *The limits this store declares* states them:

| Limit | Declared |
| --- | --- |
| payload (`data`) | 1,048,576 bytes (1 MiB) |
| tags per event | 1,024 |
| events per append | 1,024 |

These are the adapter's stated refusal policy, not a measured wall, and the crate
publishes no constants for them, so copy the numbers from the README rather than
naming them in code. They are seeded from Cloudflare's documented 2 MiB row cap,
and the physical wall was never located (the same section).

**Query width.** A wide query is not refused. The store splits it across several
statements and merges the results, because `workerd` caps one statement at 5
compound-`SELECT` terms, 100 bound parameters and 100,000 bytes (the README, *What
the conformance suite ran against this adapter, and what it did not*). The two
widths it splits at are public:
`CloudflareEventStore::MAX_QUERY_ARMS_PER_STATEMENT`, 5, and
`CloudflareEventStore::MAX_QUERY_PARAMETERS_PER_STATEMENT`, 90. A 128-item query
takes 26 statements here
([VT-23](../spec/SPECIFICATION.md#vt-23--min_supported_query_items-is-a-floor-of-128)).
To size a decision model before you deploy it, call
`CloudflareEventStore::planned_statement_count(&query)`, which counts the plan
the read path uses (the crate root, *The query widths this store does not
refuse*).

**Positions.** Workers SQL returns integers through a JavaScript number, so a
position above 2^53 − 1 cannot be read back. A stored position past that bound is
reported as `CloudflareEventStoreError::StoredPosition` rather than narrowed (the
README, *The limits this store declares*).

**A failed batch.** The object's turn commits when the handler returns normally,
and the store turns every throw into an `Err` and returns normally, so the turn
does not undo a failed batch. All-or-none rests on the write path instead
([ES-18](../spec/SPECIFICATION.md#es-18--atomicity)). A batch's rows become
events only at its last statement, which stamps every row with its event
identity at once. A batch that fails before that stamp leaves rows no read
treats as events, so nothing of it is visible. The store then sweeps those rows
away, but the sweep is cleanup and not what makes the batch all-or-none (the
`event_store` module, *Why the whole object is one consistency boundary*). If
the sweep itself fails, `append` reports
`CloudflareEventStoreError::PartialBatch`, and that variant's own documentation
says the batch did not land and a retry is safe. The crate root's older wording,
which says the opposite, is tracked in
[issue #257](https://github.com/Wet-Ink-Corporation/happenstance/issues/257).

**A dropped append.** `append` contains no `.await`, so its future has run to
completion by the time its first poll returns, and dropping it cannot cancel
it. That is this
adapter's answer to
[ES-23](../spec/SPECIFICATION.md#es-23--cancellation-outcome-is-unspecified-and-that-is-the-contract).
Code written generic over `EventStore` cannot rely on it, because other stores
answer differently (the `event_store` module, *Cancellation*).

## Keep the read model somewhere it can commit

The crate has no projection store. It does not implement the projection port at
all (the crate root, *Status: what publication this crate is ready for, and what
it is not*). [Choose a store](choose-a-store.md#put-the-checkpoint-where-the-read-model-is)
names the two projection stores that can run inside the object, and what each
costs. Read it before you build a read model here.

## See how the crate checks itself

The adapter's conformance target is one macro call:
`event_store_conformance!` with `emit = happenstance_testkit::emit_wasm` and a
`CloudflareFixture`, in
[`tests/durable_object_conformance.rs`](../crates/happenstance-cloudflare/tests/durable_object_conformance.rs).
`emit_wasm` wraps each rule for `wasm-bindgen-test`, because the emitter is the
caller's to choose
([CF-23](../spec/SPECIFICATION.md#64-runtime-independence)). The gate runs every
event-store rule that way on `wasm32-unknown-unknown`, against a
`node:sqlite`-backed host that stands in for the runtime. The fixture declines
no capability, so the run prints no skip line (the README, *What the conformance
suite ran against this adapter, and what it did not*).

The same rules also run in a real Durable Object, in the `workerd` CI job beside
the gate, through [`harness/workerd`](../harness/workerd/README.md): once under
local `workerd` and once on an object deployed to Cloudflare. That covers the
isolate, the I/O gate and the platform's SQLite limits, which the host does not.

If you are writing an adapter of your own for a runtime like this one,
[Pass the conformance suite](pass-conformance-suite.md) shows how to choose an
emitter, `emit_wasm` among them.

## Status: what this page does not cover

The crate's README calls its release the one with the thinnest evidence of the
seven. Its gaps, all named in the sources above:

- **Eviction and hibernation have never been exercised.** The `workerd` harness
  runs each rule inside a single request, and its `REOPEN` is a fresh storage
  handle, so what a real isolate restart does to an acknowledged write has not
  been observed. The harness does not reach the memory ceiling either
  (`harness/workerd/README.md`, *What it discharges, and what it does not*).
- **The three capacity ceilings are policy.** The physical wall behind them was
  never located (the README, *The limits this store declares*).
- **The concurrency and model families do not run here.** The first needs OS
  threads and a `Send` store. The second sits behind the testkit's `proptest`
  feature, which is absent on this target (the README, *What the conformance
  suite ran against this adapter, and what it did not*).
- **This crate checks no read model.** It has no projection store, so what you
  keep beside the log in the object is checked by whichever crate provides it,
  if any. [Choose a store](choose-a-store.md#put-the-checkpoint-where-the-read-model-is)
  says what each option has and has not run.
- **Wiring the class into a Worker is not covered.** The Worker's entry point,
  its `wrangler.toml` and the class's bindings belong to the platform, and no
  crate here documents them. [`harness/workerd`](../harness/workerd/README.md)
  has a working set, and its README says why its entry point is JavaScript.
