# happenstance-cloudflare

The Cloudflare Durable Object event store adapter for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance). It turns one
Durable Object into a DCB-compliant event store, writing to that object's own
`SqlStorage` through the same `state.storage().sql()` handle a
`#[durable_object]` class already holds, on `wasm32-unknown-unknown`, inside
Workers.

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

A Durable Object is a single-threaded actor with exclusive ownership of its
storage, which is most of an event store's hard part already decided: there is
one writer, the object's own turn is the transaction, and the consistency
boundary a DCB query describes fits inside an object rather than across a
cluster.

> **Status: implemented and measured; not released, and not frozen.** Every
> [`EventStore`](https://docs.rs/happenstance-core) body is real — `migrate`,
> `append`, `head`, `contains_event_id` and `read` all execute SQL against the
> object's storage — and the event-store conformance suite runs against this
> adapter inside the repository's own gate, on `wasm32-unknown-unknown`.
>
> What has **not** happened: no release, no version to depend on, and no frozen
> API. The conformance run stands the adapter up against a Durable Object host
> backed by real SQLite rather than against `workerd`; the section below says
> what that does and does not cover, in the run's own terms.

## Use

The adapter is an ordinary library type, so a `#[durable_object]` class holds one
and hands it the storage it already has. There is a single constructor and it
takes the object's `SqlStorage` — nothing here conjures storage out of nothing,
because a real Durable Object's cannot be.

```rust,no_run
use happenstance_cloudflare::{CloudflareEventStore, SqlStorage};
use happenstance_core::{
    AppendCondition, Event, EventStore, Query, QueryItem, ReadOptions, Tags, collect,
    read_decision_model,
};

// `state` is the `State` a `#[durable_object]` class is constructed with.
async fn enrol(state: &worker::State) -> Result<(), Box<dyn core::error::Error>> {
    let store = CloudflareEventStore::new(SqlStorage::new(state.storage().sql()));
    store.migrate()?;

    // "Who is already enrolled on this course?" The query *is* the consistency
    // boundary — there is no aggregate to load.
    let query = Query::from_item(QueryItem::new(
        ["StudentEnrolled"],
        Tags::from_pairs([("course", "c1")])?,
    )?);
    let (enrolled, last_seen) = read_decision_model(&store, &query).await?;

    // Decide from what was read, not from a re-query: this is the whole of the
    // DCB command loop.
    if enrolled.len() >= 2 {
        return Ok(());
    }

    let event = Event::new("StudentEnrolled", &b"{\"student\":\"s1\"}"[..])?
        .with_tags(Tags::from_pairs([("course", "c1")])?);

    // Append only if nothing matching has appeared since that read.
    let condition = AppendCondition::new(query.clone()).after_opt(last_seen);
    let appended = store.append(&[event], Some(&condition)).await?;

    // The store assigns positions and the caller reads them back rather than
    // assuming them: the specification permits gaps, and this adapter leaves one
    // whenever a batch is discarded.
    let replayed = collect(store.read(&query, ReadOptions::new().from(appended))).await?;
    assert_eq!(replayed.last().map(|event| event.position), Some(appended));

    Ok(())
}
```

That block is compiled by this repository's gate, through
`#![cfg_attr(doctest, doc = include_str!("../README.md"))]` on the crate root: a
README example that does not compile is worse than no example, because it is the
first thing a reader tries. It is marked `no_run` for one honest reason — every
`worker` binding resolves to a stub that panics off-target, so an example that
*ran* on the host would be exercising `wasm-bindgen`'s stub rather than a Durable
Object.

## What the conformance suite ran against this adapter, and what it did not

Not "passes the DCB conformance suite". What is true is narrower and more useful,
and every qualifier below is one the run itself prints or the fixture itself
declares, so it can be diffed rather than believed.

**The event-store family runs in full, on the target, inside the gate.** The
adapter's conformance target is three lines of
`happenstance_testkit::event_store_conformance!` over a `CloudflareFixture`, and
the gate executes it on `wasm32-unknown-unknown` under `wasm-bindgen-test-runner`.
The rule set is not a list this crate keeps: the macro expands the suite's own
enumeration, and the gate asserts every name that enumeration declares out of the
compiled target's `--list` before the run starts.

**It is not `workerd`.** What the rules execute against is a
`DurableObjectState`-shaped host shipped in this crate, backed by Node's own
`node:sqlite` and reached through `worker`'s real `wasm-bindgen` externs —
`worker::State::from(…)` → `state.storage().sql()` → `worker::SqlStorage::exec`,
with real SQLite underneath. The *adapter* is unmodified; what is doubled is the
runtime. So there is no isolate, no eviction, no hibernation, no event loop
re-entering the object mid-`await`, and none of the platform's own storage
ceilings.

**The fixture declines nothing, so the conformance run prints no `SKIP` line.** A
rule whose capability a fixture declines still runs and reports the fixture's
stated reason rather than vanishing from the binary; there is nothing here for
that channel to report. The two `SKIP` lines the gate *does* print for this crate
come from a deliberately declining fixture in `tests/fixture_contract.rs`, which
exists so that the reporting channel has something that fails when it breaks —
they are a control, not this adapter's result.

| Capability | This fixture | What it buys the run |
| --- | --- | --- |
| `SECOND_HANDLE` | supported | two handles onto one object, so the multi-connection rules are real rather than skipped |
| `REOPEN` | supported | a second binding taken off the same `state`, so an acknowledged write is observed surviving one |
| `MID_BATCH_FAULT` | supported | a trigger on the object's own `event` table, so atomicity is checked against a fault inside the store's write path |

**The concurrency family is not invoked, and that is a reason rather than a
silence.** `happenstance_testkit::event_store_concurrency_conformance!` binds
`F::Store: EventStore + Send` and its module is `#[cfg(not(target_arch =
"wasm32"))]`, because it starts contenders on real OS threads. This adapter's
store is `!Send` by construction — it is the reason the bare `EventStore` flavour
exists — and `wasm32-unknown-unknown` has no threads, so a `!Send` adapter cannot
invoke that family and is not expected to. It costs this adapter nothing it could
otherwise have had: a Durable Object is a single-threaded actor, so there is no
second writer for a race to elect a winner between. The event-store family still
runs the single-threaded shapes of the same question — interleaved appends on one
handle electing one winner, and a live read stream that does not block an append.

The model family is behind the testkit's off-by-default `proptest` feature and is
absent on this target by construction, so there is nothing here to opt into.

## The limits this store declares

The adapter refuses an oversized batch **before it issues any SQL**, which is what
lets the refusal name which ceiling was crossed and guarantees no row of it
landed.

| Limit | Declared | Refused as |
| --- | --- | --- |
| payload (`data`) | 1,048,576 bytes (1 MiB) | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }` |
| tags per event | 1,024 | `… limit: StoreLimit::TagsPerEvent` |
| events per append | 1,024 | `… limit: StoreLimit::EventsPerBatch` |

Read those as **this adapter's stated refusal policy, not a measured physical
wall** — the difference is the whole of what is honest here. No physical wall was
observable on the executing host at eight times the payload figure, sixteen times
the tag figure or eight times the batch figure, and that host is a Node process
rather than `workerd`, so it does not enforce the platform's documented caps at
all. The three numbers are seeded from Cloudflare's documented 2 MiB row cap and
reduced by this adapter's measured per-row overhead. Each clears the
specification's guaranteed minimum by a wide margin. What *was* measured, twice
and reproducibly: the declared value is accepted and read back byte-for-byte, and
one more is refused, naming the ceiling it crossed.

One more limit belongs beside them, because it is the store's and not the
platform's: **positions are bounded by 2^53, not 2^64.** Workers SQL widens an
integer through a JavaScript number on the way out, so a position above
`Number.MAX_SAFE_INTEGER` is not round-trippable even though `NonZeroU64` permits
it. A stored value that crossed the line is reported as
`CloudflareEventStoreError::StoredPosition` rather than silently narrowed.

## Where it runs

`wasm32-unknown-unknown` is the target this crate exists for and the only one
where its bindings resolve to a live JavaScript heap. It also compiles for the
host, which is a convenience for contributors rather than evidence: every
`worker` binding it links there resolves to a stub that panics when called.

## Licence

MIT OR Apache-2.0, at your option. Both texts are in this directory.
