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
crates/happenstance-sync/        🔲 stub. instance-to-instance replication.
examples/course-subscriptions/   the canonical DCB worked example.
xtask/                           `cargo xtask ci` — the whole gate, defined once.
docs/adr/                        the decisions this design rests on.
```

Dependency rule: **everything depends on `happenstance`; `happenstance` depends
on nothing in this workspace.** No adapter may depend on another adapter.

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
their own ADR.

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
  `crates/happenstance-sync/src/lib.rs`.
- **Whether `happenstance-runtime` is the right name and the right seam.**
