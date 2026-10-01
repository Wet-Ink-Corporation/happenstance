+++
title = "What is checked, and where"
description = "The claims this library makes, the instrument that checks each one, and where each crate runs."
template = "page.html"
+++

This page routes. The normative text is the
[specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md),
and where this page and a clause disagree, the clause wins.

## Claims, and what checks them

| Claim | Checked by |
| --- | --- |
| Storage agnostic | Every adapter runs `happenstance_testkit::event_store_conformance!`. An adapter that has not passed it is not considered to exist. |
| No rule in the suite is decorative | Each rule has a deliberately wrong store that fails it, kept in the testkit's own tests. |
| The contract is stable | `EventStore` has been frozen since `0.2.0` and `ProjectionStore` since `0.3.0`; `cargo-semver-checks` runs against the published releases. |
| The specification is current | `cargo xtask spec-trace` fails when a clause's citation or maturity marker stops resolving. |
| The guide is true of the code | Every Rust example in `docs/` is compiled against the real crates by `cargo xtask ci`. |
| Works where `Send` does not | `happenstance-core` is built for `wasm32-unknown-unknown` in the gate, and the Durable Object adapter runs the suite under `workerd`. |

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

## Further reading

- The [changelog](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/CHANGELOG.md), release by release.
- The [decision records](https://github.com/Wet-Ink-Corporation/happenstance/tree/main/.kb/decisions), each with the alternatives it rejected.
- The [runbook](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/runbook/README.md): what remains before `1.0`, and in what order.
