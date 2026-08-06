# CLAUDE.md

Operating notes for AI assistants working in this repository. Read this before
changing anything.

## What this is

A storage-agnostic, [DCB-compliant](https://dcb.events/specification/) event
sourcing library. `happenstance` defines the contract; adapter crates implement
it; `happenstance-testkit` decides whether they did.

## Who you are working with

The repository owner has 25+ years in distributed systems, databases and
enterprise architecture, and is fluent in C#, Python and JavaScript. They are
**new to idiomatic Rust**.

So: do not explain event sourcing, CQRS, consistency boundaries or concurrency
control — that ground is well covered. **Do** explain Rust-specific reasoning:
why a newtype instead of a type alias, why `NonZeroU64` earns its keep, what
coherence is refusing to allow, why a lint is on, why a lifetime is where it is.
When a construct is unusual, say what the alternative was and why it lost.

## Repository map

```
crates/happenstance/             the contract. types, ports, errors, in-memory store.
crates/happenstance-testkit/     conformance suite. the bar every adapter must clear.
crates/happenstance-runtime/     🔲 named seam: codecs, DomainEvent, decision models.
crates/happenstance-sqlite/      🔲 stub. event store + projection store.
crates/happenstance-ladybug/     🔲 stub. graph projection store only.
crates/happenstance-postgres/    🔲 planned. the target that does not serialise writers.
crates/happenstance-neon/        🔲 planned. Postgres over one-shot HTTP.
crates/happenstance-sync/        🔲 stub. the replication port + peers + a runner.
examples/course-subscriptions/   the canonical DCB worked example.
xtask/                           `cargo xtask ci` — the whole gate, defined once.
docs/adr/                        the decisions this design rests on.
```

Dependency rule: **everything depends on `happenstance`; `happenstance` depends
on nothing in this workspace.** No adapter may depend on another adapter.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters: `happenstance-sync` is itself a port crate. Peer adapters
depend on it the way store adapters depend on `happenstance`, and its conformance
suite lives in `happenstance-sync-testkit`. It stays out of the contract crate so
that publishing `happenstance` never waits on replication.

## Binding constraints

These come from `docs/adr/`. Changing one means writing a new ADR, not editing
code around it.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` / Cloudflare Workers target impossible. Ports are defined once
   without a `Send` bound and `trait_variant` derives the `Send` flavour.
   (ADR-0001)
2. **Never put `serde` in `happenstance`'s default features.** Payloads are
   opaque `Bytes`. The `serde` feature covers envelope types only, for
   replication. (ADR-0003)
3. **`EventStore::read` returns the stream at the top level and is not
   `async`.** Nesting it inside a future silently drops `+ Send` from the
   stream on the `Send` flavour, defeating the entire two-trait design. There
   is a unit test asserting this; if you find yourself deleting it, stop.
   (ADR-0001)
4. **Bind `EventStore`, not `SendEventStore`, in generic code.** It is the
   weaker requirement and accepts both flavours. Import only one of the two
   names per module — having both in scope makes method calls ambiguous.
5. **No let-chains.** Stable only from 1.88; the MSRV is 1.85. (ADR-0004)

## The rule that matters

**Any new event store adapter must invoke the conformance suite and pass it
before it is considered to exist.**

```rust
happenstance_testkit::event_store_conformance!(MyStore::new());
```

An adapter that compiles but has not run the suite is not an adapter. If a rule
seems wrong, fix the rule and explain why in the same change — do not skip it.

When adding a conformance rule, never assert on literal position values
(`[1, 2, 3]`). The specification permits gaps, and a conformant adapter may
leave them. Compare against positions the store actually assigned.

Two corollaries, both learned the expensive way and both easy to violate by
accident:

**A rule that no adapter can fail is decorative.** Before adding one, name a
plausible wrong implementation it rejects, and write that implementation into
the testkit's own `tests/` if one does not already exist there.

**A port is only as well-designed as the *spread* of what implements it.**
`MemoryEventStore`, a `RefCell` store, rusqlite and a Durable Object all
serialise their writers and assign positions under a lock — four adapters, one
storage shape, and any port frozen against them is frozen against SQLite wearing
four hats. Before freezing a port, name the axis it is most likely to be wrong
about and check that something in the workspace sits at the other end of it. That
is what `happenstance-postgres` (positions assigned outside the transaction) and
`happenstance-neon` (no connection, no interactive transaction, no cursor) are
for. They are instruments first and targets second.

## House style

- No `unwrap`/`expect` in library code. Tests and fixtures may, with
  `#![allow(clippy::unwrap_used)]` scoped to the test module.
- No `anyhow` in library crates. `xtask` and examples may use it.
- Every public item documented; `missing_docs` is a warning and CI denies
  warnings. Every fallible public function needs an `# Errors` section.
- `#[non_exhaustive]` on public structs and enums that will grow.
- Comments explain *why*, not *what*. Prefer one comment that names the
  constraint over three that narrate the code.
- Doctests are documentation that cannot rot — prefer a runnable example to a
  described one. Feature-gated examples belong in the feature-gated module, so
  they are only compiled when the feature is on.

## Commands

```console
cargo xtask ci                          # the whole gate — run this before saying "done"
cargo test --workspace --all-features
cargo run -p course-subscriptions        # the worked example
cargo xtask wasm                        # just the wasm32 check
```

`cargo xtask ci` runs: fmt, clippy with `-D warnings`, tests, the wasm32 build of
`happenstance`, docs, and — when installed — `cargo hack` feature-powerset and
`cargo deny`. It is defined once in `xtask/src/main.rs` and is exactly what CI
runs.

`cargo-hack` and `cargo-deny` are not installed locally; those two steps print
`skipped` and are enforced in CI. The MSRV is likewise verified only in CI.

## Open questions, deliberately unresolved

Do not settle these silently in passing; they need their own pass and probably
their own ADR. Two files carry the answers, and they answer different questions.
[`docs/architecture/SPECIFICATION.md`](docs/architecture/SPECIFICATION.md) says
what is **true now** — 193 numbered clauses, each marked frozen, provisional or
deferred, each naming the conformance rule that checks it and the wrong
implementation it forbids. [`docs/RUNBOOK.md`](docs/RUNBOOK.md) says **who settles
what is still open, and when**. Where a summary below disagrees with a clause, the
clause wins; the summaries are orientation only.

Changing a `[FROZEN]` clause requires a new ADR, not an edit.

- **The projection store port is provisional.** It has no conformance suite yet,
  and a port without one is a guess. It gets frozen when the first real
  projection adapter can be built against it. The invariant it must preserve —
  read-model write and checkpoint write in one transaction — is documented on
  the trait.
- **SQLite driver** (`rusqlite` vs `sqlx`) and the append-condition SQL
  strategy. Notes are in `crates/happenstance-sqlite/src/`.
- **Replication semantics.** `SequencePosition` is meaningful only within one
  store, so positions cannot be replicated as-is. Whether ingest re-checks
  append conditions is the central unanswered question; it is written up in
  `crates/happenstance-sync/src/lib.rs`, along with the shape of the peer port
  itself and whether hub-and-spoke and peer-to-peer are one abstraction or two.
- **How a Postgres adapter buys position visibility.** `nextval()` allocates
  outside the transaction, so a Postgres store violates the visibility invariant
  by construction unless it does something about it. `xid8` +
  `pg_snapshot_xmin`, transaction-scoped advisory locks and a serialised sequence
  table each cost something real, and the choice is owed a measurement rather
  than a preference.
- **Whether `happenstance-runtime` is the right name and the right seam.**
