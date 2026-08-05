# eventum

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/) —
with batteries.

[![CI](https://github.com/Wet-Ink-Corporation/eventum/actions/workflows/ci.yml/badge.svg)](https://github.com/Wet-Ink-Corporation/eventum/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#licence)

> **Status: early.** The contract and its conformance suite are real and
> tested. Every storage adapter is a documented stub. See [status](#status).

---

## DCB in sixty seconds

Classical event sourcing makes you draw consistency boundaries *before* you know
what decisions you will make. Every command must fit inside one aggregate, and
anything that spans two needs a read model plus a saga to hold it together.

DCB draws the boundary *per decision* instead. A command handler reads exactly
the events its decision depends on — across as many entities as it likes — and
then appends conditioned on **nothing matching that same query having appeared
since**. The consistency boundary is whatever the handler actually looked at.

The canonical example: a student subscribes to a course, subject to three
invariants that do not share a boundary.

```rust
// Everything this decision depends on, in one query.
let query = Query::from_items([
    // The capacity, and everyone currently holding a seat.
    QueryItem::new(
        ["CourseDefined", "StudentSubscribed", "StudentUnsubscribed"],
        Tags::from_pairs([("course", "c1")])?,
    )?,
    // This student's own history with this course.
    QueryItem::new(
        ["StudentSubscribed", "StudentUnsubscribed"],
        Tags::from_pairs([("course", "c1"), ("student", "s1")])?,
    )?,
])?;

let (events, last_seen) = read_decision_model(&store, &query).await?;
// ... fold `events` into a decision ...

// Append only if nothing matching has landed since we looked.
let condition = AppendCondition::new(query).after_opt(last_seen);
store.append(&[subscribed], Some(&condition)).await?;
```

No aggregate. No saga. One append that is rejected precisely when — and only
when — something happened that could have changed the answer.

Run the full worked example:

```console
cargo run -p course-subscriptions
```

## Status

| Crate | Role | Status |
|---|---|---|
| [`eventum-core`](crates/eventum-core) | DCB types, storage ports, in-memory reference store | ✅ implemented and tested |
| [`eventum-testkit`](crates/eventum-testkit) | Conformance suite adapters must pass | ✅ 27 rules + property tests |
| [`eventum-sqlite`](crates/eventum-sqlite) | SQLite event store and projection store | 🔲 stub, design notes only |
| [`eventum-ladybug`](crates/eventum-ladybug) | LadybugDB graph projection store | 🔲 stub, design notes only |
| [`eventum-sync`](crates/eventum-sync) | Instance-to-instance replication | 🔲 stub, open questions written down |
| [`eventum-runtime`](crates/eventum-runtime) | Codecs, typed domain events, decision models | 🔲 named seam, not started |

The stubs are not placeholders in the empty sense: each carries the design
constraints and open decisions for its pass, so the next session starts from the
real questions rather than rediscovering them.

## Quick start

```toml
[dependencies]
eventum-core = "0.1"
```

```rust
use eventum_core::{Event, EventStore, MemoryEventStore, Query, ReadOptions, Tags, collect};

let store = MemoryEventStore::new();

store.append(
    &[Event::new("CourseDefined", &br#"{"capacity":2}"#[..])?
        .with_tags(Tags::from_pairs([("course", "c1")])?)],
    None,
).await?;

let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
assert_eq!(events.len(), 1);
```

## Design

**Illegal states are unrepresentable.** `Query` is an enum, not a vector,
because the specification allows "at least one item" or "match everything" and
nothing else — an empty query cannot be constructed. `SequencePosition` wraps a
`NonZeroU64`, so position zero does not exist and `Option<SequencePosition>`
costs no more than a bare one. `Tags` is canonically sorted at construction, so
it can never be observed out of order and set equality is just `==`.

**Payloads are opaque.** `Event` holds `Bytes`, and `eventum-core` has no
`serde` dependency by default. Adapters need no domain knowledge, and
replication forwards events byte-for-byte without deserialising them. Encoding
belongs to the layer above.

**The concurrency signal is in the type system.** `append` returns
`AppendError`, which separates `ConditionViolated` — routine under contention,
and a cue to retry — from adapter-specific failures. No caller has to match on a
string to tell "retry" from "something broke".

**Two flavours of each port.** `EventStore` carries no `Send` bound, so it can
be implemented on `wasm32` where futures are `!Send` — which is what makes a
Cloudflare Durable Object adapter possible at all. `SendEventStore` is derived
from it for native use, and implementing it gives you both. Generic code binds
the weaker one and accepts either. CI builds `eventum-core` for
`wasm32-unknown-unknown` on every commit so this stays true.

**Adapters are separate crates, not feature flags.** `rusqlite` bundles a C
library; LadybugDB's `lbug` compiles C++ through `cmake`. Nobody who wants one
should pay for the other, and a third party can publish `eventum-postgres` as a
first-class citizen.

## Writing an adapter

Implement `SendEventStore` (or `EventStore` if your target cannot be `Send`),
then inherit the entire conformance suite:

```rust
eventum_testkit::event_store_conformance!(MyEventStore::new());
```

That expands to one `#[tokio::test]` per rule, so a failure names the rule that
broke. **An adapter is not finished until it passes.** The suite covers query
semantics, read options, position uniqueness and monotonicity, append
atomicity, the full append-condition matrix including the exact `after`
boundary, and the concurrency case DCB exists to prevent.

## Development

```console
cargo xtask ci     # the whole gate: fmt, clippy, tests, wasm32, docs, deny
cargo test --workspace --all-features
cargo run -p course-subscriptions
```

`cargo xtask ci` is defined once, in `xtask/src/main.rs`, and is exactly what CI
runs. If it passes locally, it passes on CI.

## Prior art

[Disintegrate](https://github.com/disintegrate-es/disintegrate) is the other
Rust DCB library; it is Postgres-bound and macro-driven.
[`umadb-dcb`](https://crates.io/crates/umadb-dcb) ships duplicated sync and
async traits. eventum's bet is different: ports plus a *published* conformance
suite, so that "storage agnostic" is a claim anyone can check and third parties
can ship adapters against.

## Crate naming

The bare `eventum` name on crates.io is taken by an unrelated, dormant crate
(last published 2020). The prefixed names are unaffected; see
[ADR-0002](docs/adr/0002-crate-naming.md).

## Licence

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
