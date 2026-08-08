# happenstance

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/):
a contract for storage, and a published conformance suite that decides who meets
it.

[![CI](https://github.com/Wet-Ink-Corporation/happenstance/actions/workflows/ci.yml/badge.svg)](https://github.com/Wet-Ink-Corporation/happenstance/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#licence)

> **Status: early, and worth being precise about.** The contract and its 27-rule
> conformance suite are real and tested. The typed layer is a facade over the
> contract, and every storage adapter is a documented stub. `0.1.0` is the
> contract, the suite, the typed layer and SQLite; Postgres, Neon, Ladybug and
> replication come after it, and the ambition is the whole list rather than the
> first four. Nothing is published yet. See [status](#status).

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
use happenstance::{
    AppendCondition, Event, EventStore, Query, QueryItem, Tags, read_decision_model,
};

async fn subscribe<S: EventStore>(
    store: &S,
    subscribed: Event,
) -> Result<(), Box<dyn std::error::Error>>
where
    S::Error: std::error::Error + 'static,
{
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

    let (events, last_seen) = read_decision_model(store, &query).await?;
    // ... fold `events` into a decision ...

    // Append only if nothing matching has landed since we looked.
    let condition = AppendCondition::new(query).after_opt(last_seen);
    store.append(&[subscribed], Some(&condition)).await?;
    Ok(())
}
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
| [`happenstance-core`](crates/happenstance-core) | DCB types, storage ports, in-memory reference store | ✅ implemented and tested |
| [`happenstance-testkit`](crates/happenstance-testkit) | Conformance suite adapters must pass | ✅ 55 rules + property tests |
| [`happenstance-sqlite`](crates/happenstance-sqlite) | SQLite event store and projection store | 🔲 stub, design notes only |
| [`happenstance-ladybug`](crates/happenstance-ladybug) | LadybugDB graph projection store | 🔲 stub, design notes only |
| `happenstance-postgres` | Postgres event store and projection store — the target that does *not* serialise its writers | 🔲 planned |
| `happenstance-neon` | Postgres over one-shot HTTP: no connection, no interactive transaction, no cursor | 🔲 planned |
| [`happenstance-sync`](crates/happenstance-sync) | The replication port: peers, and a runner that fans out across them | 🔲 stub, open questions written down |
| [`happenstance`](crates/happenstance) | Codecs, typed domain events, decision models — the crate an application programs against | 🔲 a facade over `happenstance-core` today |

The stubs are not placeholders in the empty sense: each carries the design
constraints and open decisions for its pass, so the next session starts from the
real questions rather than rediscovering them.

[`docs/RUNBOOK.md`](docs/RUNBOOK.md) sequences the remaining work — what comes
next, why in that order, and what each phase has to prove before it counts as
finished.

## Quick start

```toml
[dependencies]
happenstance = "0.1"
```

That version does not resolve yet: the name is held on crates.io at `0.0.0` until
the typed layer and SQLite land, so today the only way to try this is a git
dependency on the repository.

```rust
use happenstance::{Event, EventStore, MemoryEventStore, Query, ReadOptions, Tags, collect};

async fn define_course() -> Result<(), Box<dyn std::error::Error>> {
    let store = MemoryEventStore::new();

    store
        .append(
            &[Event::new("CourseDefined", &br#"{"capacity":2}"#[..])?
                .with_tags(Tags::from_pairs([("course", "c1")])?)],
            None,
        )
        .await?;

    let events = collect(store.read(&Query::all(), ReadOptions::new())).await?;
    assert_eq!(events.len(), 1);
    Ok(())
}
```

This block and the DCB example above are compiled by CI, so neither can drift
from the API the way a README example normally does. They are doctests of
`xtask`, not of a published crate: `include_str!` resolves against the file tree,
and a path reaching outside the package does not exist inside a packaged
`.crate` — so attaching this file to `happenstance` would make `cargo test` fail
for anyone who ran it. `xtask` is never published. Each crate's own `README.md`
is compiled by that crate, where the relative path stays inside the package.

## Design

**Illegal states are unrepresentable.** `Query` is an enum, not a vector,
because the specification allows "at least one item" or "match everything" and
nothing else — an empty query cannot be constructed. `SequencePosition` wraps a
`NonZeroU64`, so position zero does not exist and `Option<SequencePosition>`
costs no more than a bare one. `Tags` is canonically sorted at construction, so
it can never be observed out of order and set equality is just `==`.

**Payloads are opaque.** `Event` holds `Bytes`, and `happenstance-core` has no
`serde` dependency by default — the optional feature covers the envelope types
only, for replication. Adapters need no domain knowledge, and replication can
forward events byte-for-byte without deserialising them. Encoding belongs to the
layer above, which is what `happenstance` itself is for.

**The concurrency signal is in the type system.** `append` returns
`AppendError`, which separates `ConditionViolated` — routine under contention,
and a cue to retry — from adapter-specific failures. No caller has to match on a
string to tell "retry" from "something broke".

**Two flavours of each port.** `EventStore` carries no `Send` bound, so it can
be implemented on `wasm32` where futures are `!Send` — which is what makes a
Cloudflare Durable Object adapter possible at all. `SendEventStore` is derived
from it for native use, and implementing it gives you both. Generic code binds
the weaker one and accepts either. CI builds `happenstance-core` for
`wasm32-unknown-unknown` on every commit so this stays true.

**Adapters are separate crates, not feature flags.** `rusqlite` bundles a C
library; LadybugDB's `lbug` compiles C++ through `cmake`; Postgres needs a
network and a container to test against. Nobody who wants one should pay for the
other, and a third-party adapter is a first-class citizen rather than a fork.

**The adapter portfolio is deliberately unlike itself.** Adapters are not only
targets to support — they are the instrument that keeps the contract honest, and
that only works if they disagree. A store that serialises its writers under a
lock, one that assigns positions outside the transaction, and one reached over
one-shot HTTP with no interactive transaction at all will each refuse a different
part of a badly-shaped port, and a portfolio where all three behave alike proves
nothing. So Postgres is here for what it *breaks* — it is the only target on the
roadmap that can violate the position-visibility invariant, which is what makes
that invariant testable rather than decorative. It is not a flagship, and if you
want DCB on Postgres today, `disintegrate` below is the mature choice.

## Writing an adapter

Implement `SendEventStore` (or `EventStore` if your target cannot be `Send`),
write a small `Fixture` for it, then inherit the entire conformance suite:

```rust,ignore
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

A fixture is what tells the suite how to reach your store: one fixture instance
is one isolated backing store, and each `connect()` on it is one handle onto
that store. That separation is what lets rules check the things a bare
constructor could not express — that two fixtures share nothing, and that two
handles onto one store observe each other's writes both when reading and when
evaluating an append condition. Your fixture also declares, as associated
constants, whether it can hand out a second handle and whether it can be
reopened; a rule needing something you decline still runs and prints your stated
reason rather than disappearing.

The macro expands to one `#[tokio::test]` per rule, so a failure names the rule
that broke. **An adapter is not finished until it passes.** The suite covers
query semantics, read options, position uniqueness and monotonicity, append
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
async traits. happenstance's bet is different: ports plus a *published* conformance
suite, so that "storage agnostic" is a claim anyone can check and third parties
can ship adapters against.

## Former name

This project was called **eventum** until 2026-08-05. It was renamed because the
bare `eventum` name on crates.io belongs to an unrelated crate, dormant since
2020, which forced an awkward layout — prefixed crates only, and a `-core` suffix
that existed for no reason but the collision. `happenstance` is free, so the
contract crate simply takes the name. Nothing had been published, so no release
is affected; see [ADR-0005](docs/adr/0005-rename-to-happenstance.md).

## Licence

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
