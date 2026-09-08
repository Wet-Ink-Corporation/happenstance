# happenstance

An opinionated, storage-agnostic event sourcing library for Rust, built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/):
a contract for storage, and a published conformance suite that decides who meets
it.

[![CI](https://github.com/Wet-Ink-Corporation/happenstance/actions/workflows/ci.yml/badge.svg)](https://github.com/Wet-Ink-Corporation/happenstance/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/happenstance.svg)](https://crates.io/crates/happenstance)
[![docs.rs](https://img.shields.io/docsrs/happenstance)](https://docs.rs/happenstance)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.1-blue)](#minimum-supported-rust-version)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](#licence)
[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: `0.2.0`, and worth being precise about what that number claims.** The
> contract, the typed layer and a 116-rule conformance suite across four families
> are real and tested, and **two adapters have run the suite** — SQLite against a
> real file on disk, across the event-store, projection, concurrency and model
> families, and a Cloudflare Durable Object on `wasm32`. All five crates are on
> crates.io.
>
> What `0.2.0` promises: the `EventStore` clauses marked `[FROZEN]` in
> [the specification](spec/SPECIFICATION.md) are semver-binding from here.
> What it does not: `ProjectionStore` ships behind an off-by-default
> `unstable-projection` feature and is exempt from semver until two adapters at
> opposite ends of the batch-shape axis have passed its suite.
>
> What it is not is production mileage — see the note under [status](#status),
> which is narrower than the ticks suggest. Postgres, Neon, Ladybug and
> replication come next, and the ambition is the whole list rather than the first
> five.

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

That one runs in memory, which proves the semantics and nothing about durability.
For the same library against a real database — the typed layer's command loop and
its projection runner, both on one SQLite file, with everything read back after
every handle is dropped:

```console
cargo run -p transfers-on-sqlite
```

Four more, each on a real SQLite file and each showing something the two above
cannot. Read them in any order; none depends on another.

```console
cargo run -p rebuilding-read-models    # four views: skew, backfill, reset, rebuild
cargo run -p handles-and-quotas        # three boundaries, and no aggregate behind any
cargo run -p telemetry-across-codecs   # one log, two encodings, two event shapes
cargo run -p tickets-over-http         # two processes, one file, fourteen sockets
```

`rebuilding-read-models` is what an operator does to derived state: two views at
two checkpoints, a third backfilled beside the one it replaces and then promoted,
a reset and a rebuild at two chunk sizes, and a fourth poisoned by a field the log
does not carry — which stalls itself and nothing else.

`handles-and-quotas` is the case DCB exists for. A unique handle over a set nobody
can enumerate, a per-owner quota over a set nobody knows until it is read, and an
idempotent delivery, composed into one append condition. It prints the query it
derived, because the boundary is a value.

`telemetry-across-codecs` is a log that outlived both its encoding and its schema.
JSON payloads and postcard payloads, and readings in whole degrees beside readings
in thousandths, folded by one model that names one codec and branches on neither.

`tickets-over-http` is the shape people deploy: an HTTP API and a projection
runner in separate processes over one file. Fourteen clients race for a five-seat
show and five seats are sold; `GET /seats?at=N` answers `202` with its own
checkpoint until the runner has caught up, and `200` after.

## Status

| Crate | Role | Status |
|---|---|---|
| [`happenstance`](crates/happenstance) | Codecs, typed domain events, decision models — the crate an application programs against | ✅ on crates.io at `0.2.0` |
| [`happenstance-core`](crates/happenstance-core) | DCB types, storage ports, in-memory reference store | ✅ on crates.io at `0.2.0` |
| [`happenstance-testkit`](crates/happenstance-testkit) | Conformance suite adapters must pass | ✅ on crates.io at `0.2.0` — 116 rules across four families |
| [`happenstance-sqlite`](crates/happenstance-sqlite) | SQLite event store and projection store | ✅ on crates.io at `0.2.0`; passes the suite against a real file |
| [`happenstance-cloudflare`](crates/happenstance-cloudflare) | Durable Object event store — the workspace's only `!Send` store, and the reason the ports have two flavours | ✅ on crates.io at `0.2.0`; passes the suite on `wasm32` — read the note below |
| [`happenstance-postgres`](crates/happenstance-postgres) | Postgres event store and projection store — the target that does *not* serialise its writers | ✅ both roles pass their suites against a live server; **not** in the `0.2.0` release set |
| [`happenstance-neon`](crates/happenstance-neon) | Postgres over one-shot HTTP: no connection, no interactive transaction, no cursor | 🔲 stub, design notes only |
| [`happenstance-ladybug`](crates/happenstance-ladybug) | LadybugDB graph projection store | 🔲 stub, design notes only |
| [`happenstance-sync`](crates/happenstance-sync) | The replication port: peers, and a runner that fans out across them | 🔲 stub, open questions written down |

Read the ✅ rows narrowly. **"Passes the suite" is the only claim being made** —
that the adapter has run `happenstance-testkit` and cleared it, which is what
this project means by an adapter existing at all. It is not a claim of production
mileage: nothing here has run anywhere but a test.

The three 🔲 rows are not placeholders in the empty sense: each carries the design
constraints and open decisions for its pass, so the next session starts from the
real questions rather than rediscovering them.

**`happenstance-postgres` is the row that changed and the one most easily
misread.** It is no longer a stub: the event store clears 101 of 101 conformance
rules against a live PostgreSQL 17.10 including the concurrency family at 64
contenders — the first adapter here to clear that family against a store whose
writers are *not* serialised — and the projection store clears the projection
suite, 14 rules run and 3 reported as skips. It is nevertheless **not published**
at `0.2.0`, and that is a release-set decision rather than a readiness one: five
crates ship, decided at the release pass, and this is not one of them. A ✅ in
this column means *passes the suite*; the crates.io claim is in the same cell
only where it is true.

`happenstance-cloudflare` is the row most easily misread, and it ships in
`0.2.0` with the thinnest evidence of the five. Two things a reader should have
before weighing it. It passes the conformance suite under a `node:sqlite`-backed
shim rather than under `workerd` itself — `.kb/open-questions/no-workerd-class-runner-in-the-gate.md`
is the standing record of what that establishes and what it does not — and it is
`wasm32`-only, so nothing in the host test matrix exercises it. Its own front
page does not claim a frozen API, and neither does this row.

[`RUNBOOK.md`](RUNBOOK.md) explains why the remaining work is ordered as it is —
sequenced by blast radius, with the artefact each phase has to produce before it
counts as finished. Read it for that reasoning; the table above is the shorter
answer to what is built.

## Quick start

```toml
[dependencies]
happenstance = "0.2"
```

Add an adapter when you want durability — `happenstance-sqlite = "0.2"` — and
`happenstance-testkit` as a `[dev-dependencies]` if you are writing one of your
own. **Pin the testkit exactly.** Adding a conformance rule is a semver-*minor*
change that can turn a passing adapter's CI red, which is why it carries its own
version number rather than the workspace's.

## Minimum supported Rust version

**1.97.1**, and it is a promise from `0.2.0` rather than a preference: raising it
is a breaking change and needs a decision record, not a commit message. The
floor moved to 1.97.1 at phase 2 for a dependency's build script rather than for
anything in this workspace — five of the five database crates here declare no
`rust-version` at all, so neither `cargo hack --rust-version` nor `resolver = "3"`
can protect a floor against them, and only running the compiler finds it.

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
that broke. **An adapter is not finished until it passes.**

There are 116 rules in four families. The **event-store** family (93) covers
query semantics, read options, position uniqueness and monotonicity, append
atomicity and the full append-condition matrix including the exact `after`
boundary. The **concurrency** family (5) supplies the second caller a
one-at-a-time suite cannot, and is the case DCB exists to prevent. The
**projection** family (17) holds a read-model write and its checkpoint to one
unit of work. The **model** family (1) replays generated sequences of appends,
conditional appends and reads against a model and compares every answer — which
is the one that finds what the worked examples did not think to ask.

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

## The name

The everyday sense of the word is chance, which is the wrong idea for a durable
event store and worth displacing early. The sense meant here is the older half of
the compound: a happenstance is what *happened to be the case*, as against what was
arranged in advance. That distinction is the one DCB draws. A classical aggregate
fixes the consistency boundary when the schema is written, before anyone knows
which decisions will be made against it. A DCB boundary is deliberate but not
pre-declared — the handler chooses its query, and the boundary is then whatever
that query happened to match.

The project was called **eventum** until 2026-08-05. It was renamed because the
bare `eventum` name on crates.io belongs to an unrelated crate, dormant since
2020, which forced an awkward layout — prefixed crates only, and a `-core` suffix
that existed for no reason but the collision. `happenstance` was free. Nothing had
been published, so no release was affected; see
[ADR-0005](.kb/decisions/0005-rename-to-happenstance.md).

The bare name went to the contract crate for four hours, and then to the typed
layer where it belongs: `-core` is what an adapter author pins, and the bare name
is what an application installs — the allocation `serde_core`/`serde`,
`futures-core`/`futures` and `tracing-core`/`tracing` each arrived at
independently. See
[ADR-0006](.kb/decisions/0006-bare-name-to-the-typed-layer.md), which supersedes
the second half of ADR-0005 and explains why that half was wrong.

## Licence

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your
option.
