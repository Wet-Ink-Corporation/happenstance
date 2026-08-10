---
id: map-the-crate-dependency-rule
title: "Map: the crate layout and the one dependency rule"
kind: map
status: accepted
authority_tier: note
summary: >-
  Everything depends on `happenstance-core`; `happenstance-core` depends on nothing in this
  workspace; no adapter may depend on another adapter. The single deliberate exception is
  `happenstance-sync`, which is itself a PORT crate — peer adapters depend on it the way
  store adapters depend on the contract.
depends_on:
  - adr-0006-bare-name-to-the-typed-layer
related:
  - map-the-instrument-portfolio
  - governance-no-serde-in-the-contract-crate
source_paths:
  - CLAUDE.md
  - Cargo.toml
last_reviewed: 2026-08-09
---

# The crate layout

```
crates/happenstance-core/        the contract. types, ports, errors, in-memory store.
crates/happenstance/             the typed layer. today a facade over the contract.
crates/happenstance-testkit/     conformance suite. the bar every adapter must clear.
crates/happenstance-sqlite/      skeleton. event store + projection store.
crates/happenstance-cloudflare/  skeleton. the workspace's only !Send store. wasm32.
crates/happenstance-ladybug/     skeleton. graph projection store only.
crates/happenstance-postgres/    skeleton. the target that does not serialise writers.
crates/happenstance-neon/        skeleton. Postgres over one-shot HTTP. host + wasm32.
crates/happenstance-sync/        skeleton. the replication port + peers + a runner.
examples/course-subscriptions/   the canonical DCB worked example.
xtask/                           `cargo xtask ci` — the whole gate, defined once.
```

## The rule

**Everything depends on `happenstance-core`; `happenstance-core` depends on nothing in this
workspace. No adapter may depend on another adapter.**

## The one exception, and why it is not really one

`happenstance-sync` is a **port crate**, not an adapter. Peer adapters depend on it the way
store adapters depend on `happenstance-core`, and its conformance suite will live in
`happenstance-sync-testkit`, which does not exist yet. It stays out of the contract crate so
that publishing `happenstance-core` never waits on replication.

## Which crate holds the bare name

`happenstance` is the **typed** layer — the crate most people will `cargo add`. The contract
is `happenstance-core`. This inverted at ADR-0006 and the constraint about `serde` reads
backwards if you miss it.
