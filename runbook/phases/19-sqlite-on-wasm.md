# Phase 19 — SQLite on `wasm32`

Two nodes, `19a` and `19b`, for the same reason phase 10 was: the first is cheap
and answers the questions that set the second's cost, and nothing should wait on
the second that could have waited on the first.

**Goal.** An application author building a local-first product in a browser can
`cargo add` a happenstance SQLite store, compile it for `wasm32`, and get the
contract `happenstance-sqlite` gives them on a host.

**Why here.** The problem statement is `references/seeds/sqlite-on-wasm.md`, and it
is the thing to read first. In one paragraph: `happenstance-sqlite` does not build
for `wasm32-unknown-unknown` — `rusqlite` fails at `libsqlite3_sys`, and every read
hops through `tokio::task::spawn_blocking` — while four of the six scenarios put
SQLite on a device, and ES-1 already declined the name `LocalEventStore` on the
ground that "local" names that store. For the port's design it adds little: it is
another store that serialises its writers. So it is **not** on the 1.0 path. For
replication it may be the offline spoke phase 13 wants.

**Decisions it settles.** Whether it is a new crate (the seed's working
assumption, for the semver reason it states); which driver; which hosts and
storage backends; whether the SQLite SQL, once a third copy exists, gets a home
that is not an adapter — an unnumbered row in the [ADR queue](../ledgers.md#the-adr-queue).

## 19a — the skeleton

Depends on 15 only. Run it alongside 16 and 17.

- [ ] Turn the seed's problem statement into this file's plan. There is no backlog
      to put it in: tracking is this file, by D-2.
- [ ] A skeleton crate: real associated types, `todo!()` bodies, `publish = false`,
      and a scoped `#![allow(clippy::todo)]` naming 19b as the phase that removes
      it — the convention `CLAUDE.md` defines for a skeleton.
- [ ] **The driver.** Whether `rusqlite` reaches a wasm-capable backend with the
      right features, or the crate drives `sqlite-wasm-rs` directly. Answered by
      something that compiles.
- [ ] **The storage and its capabilities.** For each of memory, IndexedDB-backed
      and origin-private file system storage, what `REOPEN` and `SECOND_HANDLE`
      answer and why. The file system's synchronous handles exist only in a
      dedicated worker and are exclusive to one opener.
- [ ] **The gate.** Whether a headless browser in CI is affordable, or a
      memory-backed run under the existing `wasm-bindgen-test` wiring
      (`xtask/src/proof.rs`, `WASM_UNIT_TARGETS`) is the bar, with durability
      checked some other way.

**Proof artefact (19a).** The skeleton building for `wasm32-unknown-unknown` in
`cargo xtask ci`, its fixture declaring both capabilities with stated reasons, and
a written verdict on each of the three questions in the crate root.

**Estimate (19a).** 3 days.

## 19b — the adapter

Depends on 17 and 19a: it is written against the `0.4.0` surface, not one about to
change.

- [ ] The event store and the projection store, on the bare `EventStore` flavour,
      `!Send` and held through `Rc` as `happenstance-cloudflare` does
      (`crates/happenstance-cloudflare/src/sql_storage.rs:1-36`).
- [ ] Atomicity provided by the adapter. A Durable Object holds other requests
      back while storage work is in flight; a browser does not, so every append is
      an explicit transaction completed within one synchronous `poll`, and the
      suite's fault rules are what prove it.
- [ ] Both conformance suites green on `wasm32`, in the gate.
- [ ] If it is ready when phase 13 is, it is offered as one of that phase's three
      peers.

**Proof artefact (19b).** `event_store_conformance!` and the projection suite green
on the target the crate claims, run by `cargo xtask ci`.

**Estimate (19b).** 8–10 days, by analogy with phase 9's eight for the Durable
Object — an analogy, not a measurement.

**Exit criteria (each node)**

- [ ] The specification is reconciled against this phase's changes (session
      protocol step 6), and `cargo xtask spec-trace` passes.

**Session log**
