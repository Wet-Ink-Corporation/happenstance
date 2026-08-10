# CLAUDE.md

Operating notes for AI assistants working in this repository. Read this before
changing anything.

## What this is

A storage-agnostic, [DCB-compliant](https://dcb.events/specification/) event
sourcing library. `happenstance-core` defines the contract; adapter crates
implement it; `happenstance-testkit` decides whether they did. `happenstance`
itself is the typed layer an application reaches for — today a five-line facade
over the contract, holding the bare name because that is the crate most people
will `cargo add` ([ADR-0006](docs/adr/0006-bare-name-to-the-typed-layer.md)).

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
crates/happenstance-core/        the contract. types, ports, errors, in-memory store.
crates/happenstance/             the typed layer. today a facade over the contract.
crates/happenstance-testkit/     conformance suite. the bar every adapter must clear.
crates/happenstance-sqlite/      🔩 skeleton. event store + projection store.
crates/happenstance-cloudflare/  🔩 skeleton. the workspace's only !Send store. wasm32.
crates/happenstance-ladybug/     🔩 skeleton. graph projection store only.
crates/happenstance-postgres/    🔩 skeleton. the target that does not serialise writers.
crates/happenstance-neon/        🔩 skeleton. Postgres over one-shot HTTP. host + wasm32.
crates/happenstance-sync/        🔩 skeleton. the replication port + peers + a runner.
examples/course-subscriptions/   the canonical DCB worked example.
xtask/                           `cargo xtask ci` — the whole gate, defined once.
docs/adr/                        the decisions this design rests on.
docs/architecture/               SPECIFICATION.md — every clause that is true now.
docs/adapter-shapes.md           what the six skeletons told the type checker.
docs/experiments/                measurements. reproducible, and not in the gate.
```

**🔩 skeleton** means real associated types and `todo!()` bodies, `publish =
false`, and a scoped `#![allow(clippy::todo)]` naming the phase that removes it.
A skeleton exists to be disagreed with by a type checker — it is an *instrument*
first and a target second, and it is not an adapter until it has run the
conformance suite. None of them has.

Dependency rule: **everything depends on `happenstance-core`; `happenstance-core`
depends on nothing in this workspace.** No adapter may depend on another adapter.

One deliberate exception, and it is a port relationship rather than a dependency
between adapters: `happenstance-sync` is itself a port crate. Peer adapters
depend on it the way store adapters depend on `happenstance-core`, and its
conformance suite will live in `happenstance-sync-testkit`, which does not exist
yet. It stays out of the contract crate so that publishing `happenstance-core`
never waits on replication.

## Binding constraints

These come from `docs/adr/`. Changing one means writing a new ADR, not editing
code around it.

1. **Never introduce `#[async_trait]`.** It injects `+ Send`, which makes the
   `wasm32` / Cloudflare Workers target impossible. Ports are defined once
   without a `Send` bound and `trait_variant` derives the `Send` flavour.
   (ADR-0001)
2. **Never put `serde` in `happenstance-core`'s default features.** Payloads are
   opaque `Bytes`. The `serde` feature covers envelope types only, for
   replication. (ADR-0003)

   Read the crate name carefully: this constrains **`happenstance-core`**, and
   after ADR-0006's rename it says the opposite of what it used to. `happenstance`
   is now the *typed* layer, whose entire job is encoding — it is the crate that
   will depend on `serde`, and forbidding it there would forbid the thing the
   split exists to allow.
3. **`EventStore::read` returns the stream at the top level and is not
   `async`.** Nesting it inside a future silently drops `+ Send` from the
   stream on the `Send` flavour, defeating the entire two-trait design.
   **Two** tests in `memory.rs` assert this and it takes both; if you find
   yourself deleting either, stop.
   `send_flavour_stream_is_send_in_generic_code` writes the bound at the
   definition, so the obligation is discharged before monomorphisation — that
   is the fix for the old test, which asserted `Send` on a *concrete* stream
   and passed by auto-trait leakage whatever the trait said. But it is not
   sufficient on its own: under the `async fn read` refactor the outermost
   item is the future, `trait_variant` marks the future `Send`, and the
   assertion is satisfied by the wrong thing. `spawns_from_generic` is what
   rejects that refactor, because it holds the stream across an await inside a
   real `tokio::spawn`. (ADR-0001, ADR-0008)
4. **Bind `EventStore`, not `SendEventStore`, in generic code.** It is the
   weaker requirement and accepts both flavours. Import only one of the two
   names per module — having both in scope makes method calls ambiguous.
5. ~~**No let-chains.**~~ **The MSRV is 1.97.1**, raised from 1.85 at phase 2
   ([ADR-0029](docs/adr/0029-msrv-raised-to-1-97-1.md), amending ADR-0004).
   Let-chains stabilised in 1.88 and are now available.

   The instruction that replaced it is the same instruction, one level up:
   **weigh the floor, do not obey it.** Nothing is published, so no downstream
   consumer is pinned to anything, and ADR-0004 carries a **provisional** marker
   for exactly that reason — it loses the marker at phase 12, when first publish
   turns the MSRV into a promise. Raising it was a deliberate trade recorded in
   an ADR, which is what the old text asked for; what stays forbidden is moving
   it in silence.

   Two things follow that are easy to miss. The MSRV now **equals**
   `rust-toolchain.toml`'s pin, so the `msrv` CI job proves nothing until the two
   diverge — it is kept for the day they do, and says so. And the reason the
   floor moved was a *dependency's build script*, not our code: five of the five
   database crates in this workspace declare no `rust-version` at all, so
   `cargo hack --rust-version` cannot protect a floor against them and neither
   can `resolver = "3"`. Only running the compiler finds it.

## The rule that matters

**Any new event store adapter must invoke the conformance suite and pass it
before it is considered to exist.**

```rust
happenstance_testkit::event_store_conformance!(MyFixture::new());
```

The expression builds a **`Fixture`**, not a store — that changed at phase 3 and
the old `factory =` spelling is gone with no deprecated arm, because nothing is
published yet. One fixture instance is one isolated backing store; each
`connect()` on it is one handle onto that store. A fixture also declares
`SECOND_HANDLE` and `REOPEN` as `Capability` associated constants, and a rule
whose capability is declined still runs, reporting the fixture's stated reason
rather than vanishing from the binary. `happenstance_testkit::fixtures::MemoryFixture`
is the reference implementation.

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
cargo xtask spec-trace                  # just the specification's cross-references
```

`cargo xtask ci` runs: fmt, clippy with `-D warnings`, tests, four wasm32 steps —
the build of `happenstance-core`, which is the standing guard on constraint 1 and
names the contract crate on purpose, a check of the conformance harnesses, and
builds of `happenstance-cloudflare` and `happenstance-neon`, both of which claim
that target in their own documentation and neither of which was checked by
anything until phase 2 — docs, `cargo xtask spec-trace` over
`SPECIFICATION.md`, a `--no-default-features` doc build of `happenstance-core`,
and a `cargo package --list` assertion that each of the three publishable crates
carries both licence files and a README. Then, where the tool or toolchain is
present: `cargo hack` feature-powerset, `cargo deny`, a wasm32 feature-powerset
check above the mandatory plain one, and a nightly `--cfg docsrs` rustdoc build.
It is defined once in `xtask/src/main.rs` and is exactly what CI runs.

`cargo-hack` and `cargo-deny` both resolve on this machine, so those steps run
rather than printing `skipped`: a green local gate now proves more than it used
to, not less. A step skips only when its probe fails to find the tool — a tool
that runs and finds a problem always fails the gate.

The MSRV is the one thing the local gate still does not check. CI carries a
dedicated `msrv` job that runs `cargo hack check --no-dev-deps --rust-version` on
a pinned 1.97.1 toolchain, which is what a *consumer* sees, and then a full
`cargo test --workspace --all-features` at 1.97.1, because `--no-dev-deps` is
exactly the flag that hides `proptest` and `tokio` — both of which declare
`rust-version = "1.85"`, leaving no headroom at all. **That job now runs the
same compiler the gate runs** and proves nothing until the pin and the floor
diverge again; ADR-0029 explains why it is kept rather than deleted.

## Open questions, deliberately unresolved

Do not settle these silently in passing; they need their own pass and probably
their own ADR. Two files carry the answers, and they answer different questions.
[`docs/architecture/SPECIFICATION.md`](docs/architecture/SPECIFICATION.md) says
what is **true now** — 200 numbered clauses, each carrying a maturity marker
(frozen, provisional, deferred, or demoted to non-normative prose) and each
naming the conformance rule that checks it and the wrong implementation it
forbids. `cargo xtask spec-trace` is a gate step precisely so those markers and
citations cannot rot into decoration. [`docs/RUNBOOK.md`](docs/RUNBOOK.md) says
**who settles what is still open, and when**. Where a summary below disagrees
with a clause, the clause wins; the summaries are orientation only.

Changing a `[FROZEN]` clause requires a new ADR, not an edit.

- **The projection store port is provisional.** It has no conformance suite yet,
  and a port without one is a guess. It gets frozen when the first real
  projection adapter can be built against it. The invariant it must preserve —
  read-model write and checkpoint write in one transaction — is documented on
  the trait.
- ~~**SQLite driver** (`rusqlite` vs `sqlx`).~~ Settled at phase 2 by building
  both: `happenstance-sqlite` is `rusqlite`, `happenstance-postgres` is `sqlx`,
  and the two are in the tree for different reasons rather than as candidates.
  The **append-condition SQL strategy** is still open and is ADR-0022's; notes
  are in `crates/happenstance-sqlite/src/`.
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
- ~~**Whether `happenstance-runtime` is the right name and the right seam.**~~
  Settled and executed: [ADR-0006](docs/adr/0006-bare-name-to-the-typed-layer.md)
  gave the bare name to the typed layer and renamed the contract to
  `happenstance-core`; [ADR-0007](docs/adr/0007-projection-runner-decodes.md)
  corrected where the projection runner lives. Kept here struck through rather
  than deleted, because the crate names in older commits only make sense with it.
