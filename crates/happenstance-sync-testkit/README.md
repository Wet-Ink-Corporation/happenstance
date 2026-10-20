# happenstance-sync-testkit

The conformance suites for [happenstance](https://github.com/Wet-Ink-Corporation/happenstance)
replication: the bar a sync peer, or a store that ingests what a peer delivers,
must clear before this project considers it to exist. It is to
[`happenstance-sync`](https://docs.rs/happenstance-sync) what
[`happenstance-testkit`](https://docs.rs/happenstance-testkit) is to
`happenstance-core`.

New to happenstance? [The happenstance guide](https://wet-ink-corporation.github.io/happenstance/)
explains event sourcing and dynamic consistency boundaries from the beginning,
with programs you can run.

> **Status: published, and unfinished.** Phase 13 adds rules, and a new rule is
> a minor of this crate's number, which is exactly the bump `cargo update`
> takes without being asked and the one that can turn a passing adapter red.
> **Pin it exactly**, as you pin `happenstance-testkit`: an `=` requirement on
> the version you test against, in `[dev-dependencies]`.
>
> The version moves in lockstep with the rest of the workspace until 1.0. Each
> rule names the clause of the
> [specification](https://github.com/Wet-Ink-Corporation/happenstance/blob/main/spec/SPECIFICATION.md)
> it checks; several of those are still `[PROVISIONAL]` (SY-7, SY-10, SY-27 –
> SY-31, SY-36 – SY-38 among the ones a rule here checks), and a rule over a
> provisional clause moves when its clause does. There is no runner yet (issue
> #244), so nothing here checks one.
>
> Every rule returns `happenstance_testkit::RuleOutcome`, so that crate's type
> is part of this one's public API.

## Use

A store adapter that implements `IngestStore` mounts the ingest family beside
the event-store suite, over a fixture that builds one isolated store per call:

```rust,ignore
happenstance_sync_testkit::ingest_store_conformance!(MyIngestFixture::new());
```

A peer adapter mounts the peer family the same way:

```rust,ignore
happenstance_sync_testkit::sync_peer_conformance!(MyPeerFixture::new());
```

`MemoryIngestFixture` and `MemorySyncFixture` are the reference fixtures, and
each rule is shown to fail a named wrong implementation in this crate's own
`tests/` before it is trusted to fail yours. A rule that needs your fixture's
co-operation, such as a fault between two rows of one group, reports itself
skipped with your fixture's stated reason rather than passing.

The harness is a parameter: `emit_ingest_tokio!` (the default),
`emit_ingest_blocking!` for a `!Send` store, and `emit_ingest_wasm!` for
`wasm32-unknown-unknown`; the peer family has the same three.

## Features

- **`proptest`**, off by default: the model family, which drives seeded peer
  topologies through partitions, lost acknowledgements and duplicated pushes
  and checks every peer at quiescence. Absent on wasm32, where `proptest` does
  not build.

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

MIT OR Apache-2.0.
