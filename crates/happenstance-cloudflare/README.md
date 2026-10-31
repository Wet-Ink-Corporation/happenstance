# happenstance-cloudflare

The Cloudflare Durable Object event store adapter for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance). It turns one
Durable Object into a DCB-compliant event store, writing to that object's own
`SqlStorage` through the same `state.storage().sql()` handle a
`#[durable_object]` class already holds, on `wasm32-unknown-unknown`, inside
Workers.

New to happenstance? [The happenstance guide](https://wet-ink-corporation.github.io/happenstance/)
explains event sourcing and dynamic consistency boundaries from the beginning,
with programs you can run.

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

A Durable Object is a single-threaded actor with exclusive ownership of its
storage, which is most of an event store's hard part already decided: there is
one writer, the object's own turn is the transaction, and the consistency
boundary a DCB query describes fits inside an object rather than across a
cluster.

> **Status: released since `0.2.0`, in the `0.4.4` release set, and the release
> with the thinnest evidence of the four adapters.** Every
> [`EventStore`](https://docs.rs/happenstance-core) body is real — `migrate`,
> `append`, `head`, `contains_event_id` and `read` all execute SQL against the
> object's storage — and the event-store conformance suite runs against this
> adapter twice: inside the repository's own gate, on `wasm32-unknown-unknown`
> against a `node:sqlite`-backed Durable Object host; and in the `workerd` CI job,
> a sibling of the gate, in a real Durable Object under local `workerd` and again
> on an object deployed to Cloudflare, where the account secrets are present.
>
> **What the version number does and does not cover.** It inherits the
> workspace's version, so the frozen `EventStore` clauses it implements are
> semver-binding here as everywhere. What it is not is *mileage*: the `workerd`
> job is not a required check, only its local run asserts across an eviction
> or a hibernation (the deployed run records hibernation without asserting
> it), and nothing in the host test matrix exercises a `wasm32`-only crate.
> The section below says what each run does and does not cover, in the run's
> own terms.

## Use

The adapter is an ordinary library type, so a `#[durable_object]` class holds one
and hands it the storage it already has. Both constructors take the object's
`SqlStorage` — nothing here conjures storage out of nothing, because a real
Durable Object's cannot be. `CloudflareEventStore::new` keeps the log in the
object's own tables; `CloudflareEventStore::namespaced` keeps it in tables under
a `TableNamespace`, so one object can hold several logs that share nothing.

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
    let condition = AppendCondition::new(query.clone()).after_every_guard_opt(last_seen);
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
re-entering the object mid-`await`, and no row or database ceiling; it does
take `workerd`'s four statement limits, which the shim sets when it opens.

**`workerd` runs them too, outside the gate.** The `workerd` CI job, a sibling of
the gate, runs every rule in a real Durable Object through `harness/workerd`.
It runs under local `workerd` and again on an object deployed to Cloudflare
where the account secrets are present. That covers the isolate, the I/O gate and the platform's row and length limits, which
the shim does not take. Its statement limits are far tighter than SQLite's
defaults: 5 compound-`SELECT` terms, 100 bound parameters, and 100,000 bytes of
statement (`experiments/durable-object-limits`). To give each fixture instance its
own log inside one object, the harness uses `CloudflareEventStore::namespaced`.

Eviction and hibernation are exercised there too, by rules that span a restart
rather than a request. Five rules write before the object restarts and assert
after it: acknowledged writes are read back under the `EventId`s they were read
under before, `head()` is the last acknowledged position, the next append lands
above every earlier position with the whole log still there, and an append
condition captured before the restart still binds. The restart is an eviction
driven from the runner between two requests, and a hibernation through
`workerd`'s own 10-second inactivity timer with a hibernatable WebSocket carried
across. Three named wrong stores fail them only when the restart happens: one
keeps its log in the object instance, one resumes from a high-water position
cached in the instance, and one mints its store identity per instance, so its
events come back under new `EventId`s. The whole family also runs on an object
evicted after its logs were created. An idle object holding the store, with or
without a partly read stream, is evicted by that same timer, while the same
object with a pending timer is not: the store keeps nothing pending that would
hold an idle object open. On the deployed object the hibernation and idle cells
are recorded rather than asserted, because nothing pinned here documents
Cloudflare's timing. What a local eviction does not reach is state in the
isolate's globals, because it tears down the object and leaves the wasm instance
standing.

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

## Replication: the `sync` feature

Turned on with `features = ["sync"]` on the dependency line. `sync` is off by
default and is the store's half of replication: it adds
`CloudflareIngestStore`, which implements `happenstance-sync`'s `IngestStore` and
this crate's `EventStore`, and re-exports the port as
`happenstance_cloudflare::happenstance_sync`. The bare flavour only, as the event
store is. Each ingest call runs inside the object's own
`storage.transactionSync`, which `worker` does not bind, so the ingest store is
built from the object's raw `DurableObjectState` rather than from `SqlStorage`:
`CloudflareIngestStore::from_object_state(&raw)`, where `raw` is what
`State::_inner` hands back (convert it back with `State::from`), or
`namespaced_from_object_state` for a namespaced log. Migrate the log through
`CloudflareEventStore` first, as before the first append. The identity an ingest
checks claims against is read from `store_meta` on every call. It turns on
`happenstance-core`'s `serde` feature through `happenstance-sync`. The ingest
family runs on `wasm32` under the gate's Node host, not yet under `workerd`.
There is no runner (issue #244) and no network peer yet, so this is the port,
not replication.

## The limits this store declares

The adapter refuses an oversized batch **before it issues any SQL**, which is what
lets the refusal name which ceiling was crossed and guarantees no row of it
landed.

| Limit | Declared | Refused as |
| --- | --- | --- |
| payload (`data`) | 1,048,576 bytes (1 MiB) | `AppendError::ExceedsStoreLimit { limit: StoreLimit::EventDataLen, .. }` |
| tags per event | 1,024 | `… limit: StoreLimit::TagsPerEvent` |
| events per append | 1,024 | `… limit: StoreLimit::EventsPerBatch` |
| `metadata` per event | 524,288 bytes (512 KiB) | `… limit: StoreLimit::MetadataLen` |

Read those as **this adapter's stated refusal policy, not a measured physical
wall** — the difference is the whole of what is honest here. No physical wall was
observable on the executing host at eight times the payload figure, sixteen times
the tag figure or eight times the batch figure, and that host is a Node process
rather than `workerd`, so it does not enforce the platform's documented caps at
all. All four are carved from Cloudflare's documented row limit of 2 MB, read as
2,000,000 bytes, less this adapter's measured per-row overhead. The first three
clear the specification's guaranteed minimum by a wide margin. The metadata
ceiling has no minimum to clear (ADR-0043), and it is sized so that one row at
all four ceilings is 1,835,264 bytes and still fits ([ADR-0093](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/.kb/decisions/0093-cloudflare-metadata-ceiling-is-carved-from-the-documented-row-limit.md)). What *was* measured, twice
and reproducibly: the declared value is accepted and read back byte-for-byte, and
one more is refused, naming the ceiling it crossed.

One more limit belongs beside them, because it is the store's and not the
platform's: **positions are bounded by 2^53, not 2^64.** Workers SQL widens an
integer through a JavaScript number on the way out, so a position above
`Number.MAX_SAFE_INTEGER` is not round-trippable even though `NonZeroU64` permits
it. A stored value that crossed the line is reported as
`CloudflareEventStoreError::StoredPosition` rather than silently narrowed.

## After a point-in-time restore

A Durable Object's storage can be rolled back to a bookmark: `getBookmarkForTime`
or `getCurrentBookmark`, then `onNextSessionRestoreBookmark(bookmark)` and a
restart of the object. That rolls back everything this store keeps: its events,
the `store_meta` row that holds its `StoreId`, and the counter that hands out
positions. Without a re-mint, the restored object hands out positions it already
gave to events written after the bookmark, under the same `StoreId`, and every
replication peer that saw those events drops the new ones as duplicates, with no
error anywhere.

This store mints its `StoreId` once and keeps it. The specification (VT-6) allows
that only because the deployment is documented to re-mint after a restore, and
this is the document:

1. Restore: call `onNextSessionRestoreBookmark(bookmark)` and restart the object
   with `ctx.abort()`. `worker` 0.8.5 binds neither call, so this step is
   JavaScript, or `js_sys::Reflect` from Rust.
2. Before the restored object accepts its first append, call
   `CloudflareEventStore::remint_identity` on every log in it: the store `new`
   builds, and one per `TableNamespace`.
3. Resume appends.

The object cannot tell that it was restored. The platform sends no signal, and
anything the store could write to remember is rolled back with it. So step 2 has
to be driven from outside the object, by whatever performed the restore, while it
keeps appends away. Events from before the bookmark keep the identity they were
written with. Re-minting when no restore happened breaks nothing; it only starts a
new origin for later events. Every handle reads the identity on each append, so a
handle built before the re-mint stamps the new one.

```rust,no_run
use happenstance_cloudflare::{CloudflareEventStore, CloudflareEventStoreError, SqlStorage};

// Run once, in the first session after the restore, before any append.
fn after_restore(state: &worker::State) -> Result<(), CloudflareEventStoreError> {
    let store = CloudflareEventStore::new(SqlStorage::new(state.storage().sql()));
    let _new_incarnation = store.remint_identity()?;
    Ok(())
}
```

This is tested against a simulated restore in the crate's Node host, and run
against a real bookmark restore on a deployed Durable Object by the `workerd` CI
job: after the restore and the re-mint, no `EventId` is reissued for a different
event, while a control that skips the re-mint reissues one. That run is asserted
when the restore runs and recorded when it cannot, and local `workerd` cannot
make it, because it does not implement a restore.

## Where it runs

`wasm32-unknown-unknown` is the target this crate exists for and the only one
where its bindings resolve to a live JavaScript heap. It also compiles for the
host, which is a convenience for contributors rather than evidence: every
`worker` binding it links there resolves to a stub that panics when called.

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

MIT OR Apache-2.0, at your option. Both texts are in this directory.
