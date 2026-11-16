+++
title = "What is checked, and where"
description = "The four promises this library makes, the instrument that checks each one, and where each crate runs."
template = "page.html"
+++

This page routes. The normative text is the
[specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md),
and where this page and a clause disagree, the clause wins.

## Each promise, and what checks it

The home page makes four promises. Each row names the instrument that would
fail if the promise stopped being true.

| Promise | Checked by |
| --- | --- |
| **True offline**: nothing is lost or overwritten when stores sync | `happenstance-sync-testkit`'s ingest and peer suites, run against `MemorySyncPeer`, `NeonSyncPeer`, a Durable Object over HTTP and a Postgres `StoreSyncPeer`. A native SQLite store round-trips through each with identical payload bytes, and a replayed batch changes nothing (`harness/sync-round-trip`). |
| **True to your rules**: a write is refused when something it depended on changed | The event-store family's append-condition matrix, the concurrency family (64 contenders, exactly one winner, against a store that does not serialise its writers), and the model family, which replays generated histories against a model. |
| **True to its history**: views are rebuilt from the log and never drift from it | The projection family, which holds a read-model write and its checkpoint to one unit of work (`commit_is_atomic_with_the_read_model`). |
| **True wherever it lives**: the same code behaves the same on every store | Every adapter runs `happenstance_testkit::event_store_conformance!`; an adapter that has not passed it is not considered to exist. `happenstance-core` is built for `wasm32-unknown-unknown` in the gate, and the Durable Object adapter runs the suite under `workerd`. |

## And what keeps these claims honest

| Claim | Checked by |
| --- | --- |
| No rule in the suite is decorative | Each rule has a deliberately wrong store that fails it, kept in the testkit's own tests. |
| The contract is stable | `EventStore` has been frozen since `0.2.0` and `ProjectionStore` since `0.3.0`; `cargo-semver-checks` runs against the published releases. |
| The specification is current | `cargo xtask spec-trace` fails when a clause's citation or maturity marker stops resolving. |
| The guide is true of the code | Every Rust example in `docs/` is compiled against the real crates by `cargo xtask ci`. |

## Where it runs {#where-it-runs}

| Crate | What it is |
| --- | --- |
| [`happenstance`](../api/happenstance/index.html) | The typed layer an application depends on: typed events, decision models and projection runners. |
| [`happenstance-core`](../api/happenstance_core/index.html) | The contract: the `EventStore` and `ProjectionStore` ports, their value types, and in-memory reference stores. |
| [`happenstance-testkit`](../api/happenstance_testkit/index.html) | The conformance suites every adapter must pass. |
| [`happenstance-sqlite`](../api/happenstance_sqlite/index.html) | SQLite event and projection stores. |
| [`happenstance-postgres`](../api/happenstance_postgres/index.html) | PostgreSQL event and projection stores — the adapter whose writers are not serialised. |
| [`happenstance-neon`](../api/happenstance_neon/index.html) | Neon serverless PostgreSQL over one-shot HTTP: no connection, no interactive transaction, no cursor. Host and `wasm32`. |
| [`happenstance-cloudflare`](https://docs.rs/happenstance-cloudflare) | A Cloudflare Durable Object event store: the workspace's `!Send` store, on `wasm32`. |
| [`happenstance-sync`](../api/happenstance_sync/index.html) | The replication port: `SyncPeer`, the `IngestStore` port each adapter implements behind its `sync` feature, and the wire messages. Published from `0.4.4`; a runner over one edge, write groups and network peers from `0.6.0`. |
| [`happenstance-sync-testkit`](../api/happenstance_sync_testkit/index.html) | The conformance suites for sync peers and ingest stores. Pin it exactly: a new rule can turn a passing adapter red. |

## Further reading

- The [changelog](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/CHANGELOG.md), release by release.
- The [decision records](https://github.com/Wet-Ink-Corporation/happenstance/tree/main/.kb/decisions), each with the alternatives it rejected.
- The [runbook](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/runbook/README.md): what remains before `1.0`, and in what order.
