# Seed — SQLite on `wasm32`

Raw material for `/redkiln:initiative`. This states a problem and a vision. It
deliberately does **not** decompose the work, name projects, or propose a design:
that is `/redkiln:plan`'s to decide, from its own grounding.

## Where this stands

happenstance has two SQLite event stores, and neither runs where a local-first
application's on-device store most often lives.

`happenstance-sqlite` is a host-only adapter. It drives `rusqlite` with the
`bundled` feature (`Cargo.toml:93`), and because `rusqlite` is synchronous every
read and every projection commit hops onto `tokio::task::spawn_blocking`
(`crates/happenstance-sqlite/src/event_store.rs:14-35`, `:2014`;
`src/projection_store.rs:108`, `:324`). `tokio` is not an implementation detail
there: `JoinError` and `TryCurrentError` are variants of the crate's public error
enums, which is why the crate root argues about whether to re-export it
(`src/lib.rs:159-186`). On `wasm32-unknown-unknown` the crate does not get as far
as its own code — `cargo check -p happenstance-sqlite --target
wasm32-unknown-unknown`, run on 2026-09-28 at `3916f29`, fails inside the driver
with `error[E0432]: unresolved import libsqlite3_sys`. Nothing in the gate would
notice either way: `cargo xtask ci`'s wasm32 steps build the contract crate, the
conformance harnesses, `happenstance-cloudflare` and `happenstance-neon`
(`xtask/src/main.rs:263-364`), and not this crate. Its documentation never says
"host only", so the limit is unstated rather than broken.

`happenstance-cloudflare` is SQLite on `wasm32`, but it is SQLite that Cloudflare
owns. Its `SqlStorage` binding (`crates/happenstance-cloudflare/src/sql_storage.rs:1-36`)
names four properties the store is built on: `exec` is synchronous, the cursor is
not a snapshot, everything is `!Send` and held through `Rc`, and the object is
single-threaded and re-entrant, which is what makes `SqlError::AlreadyBorrowed` a
real variant. It is the workspace's proof that the bare `EventStore` flavour holds
a real store on `wasm32`, and it cannot leave a Durable Object.

The two crates carry the same SQLite schema twice — `event`, `event_tag`,
`store_meta` at `crates/happenstance-sqlite/src/event_store.rs:48-80` and
`crates/happenstance-cloudflare/src/event_store.rs:15-40`, `:183-200` — because the
dependency rule forbids one adapter depending on another. The SQLite crate also
carries a `tag_cardinality` table the Durable Object does not.

## The problem

**The scenarios assume an on-device SQLite store the workspace cannot place in a
browser.** Four of the six deployments the contract was walked against put SQLite
on a device: the field engineer's tablet in Kestrel Cold Chain
(`references/scenarios/README.md:95`), Norvant's 900 handhelds, offline for hours
in customs yards (`:750-752`, `:901`), Kestrel Motor's 214 field engineers
(`:1052`), and Turnstile's venue terminals and scanners (`:1329`). A native
handheld can run `happenstance-sqlite` today. A progressive web app, a browser
extension, or a WASI component cannot, and for a local-first product the browser
is the device most often in hand.

**The specification has already reserved the word for it.** ES-1 declines
`trait_variant`'s own name for the bare flavour, `LocalEventStore`, because
"'Local' already names a local-first application's on-device store in this
project; the collision would be permanent" (`spec/SPECIFICATION.md:2560-2562`,
ADR-0001:93-96). The project paid that naming cost for a store it does not ship.

**Replication's spoke has no browser-shaped member.** Phase 13's exit criterion is
one sync suite green against three peers, two of them unlike
(`RUNBOOK.md:5582-5591`), and the scenarios' topology is a Postgres hub, Durable
Object regional hubs and on-device spokes that write while offline
(`references/scenarios/README.md:466`, `:750-752`). The spoke is the part of that
picture a browser would most often be, and it is the part with no candidate store.

**Runtime store selection is the first thing an application will ask for, and it
is already written down as unanswered.** Scenario 5 records that the genuine
consequence of there being no `dyn EventStore` is choosing *local SQLite when
offline, the Durable Object when online*, and that nothing raises it
(`references/scenarios/README.md:1416-1419`). A browser store is where that
question stops being hypothetical.

**The single-threaded, re-entrant shape has been proved once, and only inside a
platform that helps.** A Durable Object holds other requests back while storage
work is in flight, so atomicity between `await`s arrives from outside the
adapter. A browser offers no equivalent. The properties that made the Cloudflare
adapter correct would have to be provided by the adapter itself, and the
conformance suite has never been run against a `!Send` store that had to.

**Durability in a browser is a property of where the file lives, not of SQLite.**
A store's `REOPEN` and `SECOND_HANDLE` capabilities
(`crates/happenstance-testkit/src/contract.rs:148-172`, `:243`) are answered
differently by memory, by IndexedDB-backed storage and by the origin-private file
system, and the last of those is reachable synchronously only from a dedicated
worker and is exclusive to one opener. The Cloudflare fixture declines both
capabilities with stated reasons
(`crates/happenstance-cloudflare/tests/fixture_contract.rs:80-91`); a browser
store would decline, grant or split them for reasons of its own, and an honest
fixture has to say which.

## The vision

An application author building a local-first product in the browser can
`cargo add` a happenstance SQLite store, compile it for `wasm32`, and get the same
contract the host adapter gives them: conditional appends, lazy reads, projections
committed with their checkpoint in one transaction, and a log that survives a
reload wherever the storage underneath it can promise that.

Concretely, that means an adapter that has run the event store and projection
conformance suites on the target it claims, in the gate, rather than a wasm32
build that type-checks; a fixture whose declined capabilities say why in terms a
reader can check; a crate root that states which hosts and which storage backends
it supports and which it refuses; and documentation for `happenstance-sqlite` that
says "host only" before somebody has to learn it from a compiler error.

## Who it is for

**The local-first application author** whose device is a browser tab or a
browser-hosted worker, and who today has a memory store or a network round trip.

**The replication work.** A browser spoke that writes while offline is the peer
shape the scenarios draw most often, and phase 13 needs unlike peers to prove its
port.

**This project's own portability claim.** The design carries a `!Send` port
flavour at real cost so that stores can run on `wasm32`. Today the only store that
exercises it for real lives inside one vendor's runtime.

## What must remain true

These are constraints on any answer, not preferences:

- No `#[async_trait]`, and no `+ Send` injected by any other route. The bare
  `EventStore` flavour is the one a `wasm32` store implements; bind it, not
  `SendEventStore`.
- An adapter that compiles but has not run the conformance suite is not an
  adapter. A `wasm32` build step is evidence that it compiles, not that it
  conforms.
- No adapter depends on another adapter. Anything shared with
  `happenstance-sqlite` or `happenstance-cloudflare` is copied, or moved into
  something that is not an adapter, and the second of those is a decision record
  rather than a refactor.
- A declined capability runs and reports its reason; it never vanishes from the
  binary.
- `happenstance-sqlite`'s published API does not change shape by target. A crate
  whose public items differ between `wasm32` and the host is a crate whose semver
  baseline checks one of them.
- Evidence beats argument: the storage backend, the driver and the concurrency
  story are each settled against something that compiles or a measurement, and
  the artefact that proves each must be one that would not exist if it were wrong.
- Where any document and `spec/SPECIFICATION.md` disagree, the specification wins.
  A frozen clause changes by a new decision record, not an edit.

## Supporting material

Read these rather than trusting the summary above.

| Path | What it carries |
| --- | --- |
| `crates/happenstance-cloudflare/src/sql_storage.rs` | The four properties of a synchronous, `!Send`, re-entrant SQLite binding on `wasm32`, each checked against the real platform. The closest thing to a model that exists. |
| `crates/happenstance-cloudflare/tests/fixture_contract.rs` | How a `wasm32` fixture declines `SECOND_HANDLE` and `REOPEN` and says why. |
| `crates/happenstance-sqlite/src/event_store.rs` | The host adapter's schema, its lazy read state machine, and why `spawn_blocking` made laziness load-bearing. |
| `references/seeds/adr-0022-shipped-shape-drift.md` | The append-condition SQL the SQLite adapter actually ships, and why it is not the shape ADR-0022 names. Any third copy of that SQL inherits the question. |
| `references/scenarios/README.md` §1, §3, §5 | On-device SQLite in three deployments, and the runtime store selection nobody has raised. |
| `spec/SPECIFICATION.md` ES-1, ES-2 | The bare flavour, why it is not called `LocalEventStore`, and why `read` is not `async`. |
| `crates/happenstance-testkit/src/contract.rs` | The `Fixture` trait and its `Capability` constants. |
| `xtask/src/main.rs:263-364`, `xtask/src/proof.rs` (`WASM_UNIT_TARGETS`) | What the gate builds and runs on `wasm32` today, and how a `wasm-bindgen-test` target is wired in. |
| `RUNBOOK.md` phase 13 | The replication exit criterion, and the unlike peers it needs. |

## Deliberately not decided here

**Which driver.** `sqlite-wasm-rs` directly, or `rusqlite` over a wasm-capable
`libsqlite3-sys` backend. The observed failure above shows `rusqlite` reaching a
`wasm32` path it cannot resolve at the workspace's current features; whether a
feature or backend choice resolves it has not been tried. If it does, the host
adapter's SQL moves nearly verbatim, and that changes what "shared" costs.

**Which hosts and which storage.** Browser main thread, dedicated worker, shared
worker, WASI; memory, IndexedDB-backed and origin-private file system storage.
Each answers durability, exclusivity and `SECOND_HANDLE` differently, and whether
one crate spans several or declares one is a planning question.

**Whether it is a new crate.** The working assumption is a separate crate rather
than a `cfg(target_arch)` inside `happenstance-sqlite`, for the semver reason
stated above, but that is an assumption to test, not a settled shape.

**Whether the SQLite SQL gets a shared home.** A third copy is the point at which a
non-adapter crate holding the schema and the append-condition SQL starts to pay
for itself, and it is also a new crate in a workspace whose dependency rule was
written to keep crates few.

**Whether it waits for 1.0, or rides with replication.** For the port's design it
adds little — it is another store that serialises its writers, the shape the
workspace already has four of — so nothing about 1.0 waits on it. For
replication it may be the spoke phase 13 wants. Where it lands is a sequencing
question for the roadmap, not for this seed.

**How the gate runs it.** A browser-backed storage test needs a headless browser in
CI, which the gate does not carry today; a memory-backed run under the existing
`wasm-bindgen-test` wiring would check the logic and not the durability.
