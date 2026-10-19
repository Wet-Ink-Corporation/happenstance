# happenstance-workerd-harness

Runs every event-store conformance rule against `happenstance-cloudflare` inside
`workerd`, through a real Durable Object. The gate runs the same rules against a
`node:sqlite` shim, which takes `workerd`'s four statement limits but none of the
platform's other properties, so this harness checks conformance on the real
runtime. It also measures where that
runtime's SQLite refuses a statement. Never published.

It is the `workerd` job in `.github/workflows/ci.yml`, a sibling of `gate`, and
it has two legs over one Worker:

- **local**: `@cloudflare/vitest-pool-workers` runs `workerd` itself, at the
  version `package-lock.json` pins.
- **deployed**: the same Worker deployed to a Cloudflare account and driven over
  HTTPS by `deployed.mjs`.

## Running it locally

You need Node 22+, the wasm32 target (`rust-toolchain.toml` installs it), and
`worker-build` at the `worker` crate's version:

```console
$ cargo install worker-build --version 0.8.5 --locked
$ cd harness/workerd
$ npm ci
$ npm run build        # worker-build --release, into build/
$ npx vitest run       # the rules, then the wall probe
$ npx vitest run test/limits.test.ts --reporter=verbose   # the probe's report
$ npx vitest run test/restart.test.ts --reporter=verbose  # across a restart (~15 s)
$ npx vitest run test/idle.test.ts --reporter=verbose     # idle eviction (~15 s)
$ npx vitest run test/remint.test.ts --reporter=verbose   # the re-mint leg's local half
$ npx vitest run test/transaction.test.ts --reporter=verbose  # transactionSync's properties
```

To drive the deployed path without deploying, run `wrangler dev` and point
`deployed.mjs` at it:

```console
$ printf 'HARNESS_TOKEN=local\n' > .dev.vars
$ npx wrangler dev --port 8799 &
$ HARNESS_TOKEN=local node deployed.mjs http://127.0.0.1:8799 local-1
$ HARNESS_TOKEN=local HIBERNATE_WAIT_MS=12000 node deployed-restart.mjs http://127.0.0.1:8799 local-1
```

## How it fits together

| File | What it is |
| --- | --- |
| `src/object.rs` | The `#[durable_object]` class. It routes `/rules`, `/rule/<name>` and `/probe`, and clears the object's storage after each rule or probe. It builds no adapter; `src/fixture.rs` does. |
| `src/dispatch.rs` | The emitter. `for_each_event_store_rule!` expands into a `match` from rule name to rule. The name list comes from the same enumeration through the testkit's `__rule_names` callback, so a rule missing from the `match` fails as `no such rule` instead of disappearing. |
| `src/fixture.rs` | Where the adapter is built, the way a production class builds it: `SqlStorage::from_state`, then `CloudflareEventStore::namespaced`, migrated before the first handle. One fixture instance is one log under its own `TableNamespace` in the object's single database. That is how two rules that open two isolated stores at once run inside one object. |
| `src/probe.rs` | Phase A of `experiments/durable-object-limits`, run on this runtime. |
| `src/incarnation.rs` | The nonce each construction of the object, and each wasm instance, names itself with: how a run proves a restart happened. |
| `src/restart.rs` | The cross-restart rules: a before-half and an after-half per rule, the evidence carried between them, and the three wrong stores with the registry of exactly what each fails. |
| `src/object/restarts.rs` | The `/restart/...` routes, and the store an object instance holds between the halves. |
| `src/idle.rs` | `IdleHold`: a store, and optionally a read stream opened and advanced by one event, parked inside one future the object owns and polls by hand. No task, no timer, no promise. |
| `src/object/idling.rs` | The `/idle/arm?stream=none\|held` and `/idle/release` routes. |
| `src/remint.rs` | The re-mint leg's ledger, carried across a bookmark restore, and its verdict. |
| `src/object/reminting.rs` | The `/remint/seed`, `/remint/lose` and `/remint/after?mode=remint\|control` routes. |
| `worker.mjs` | The entry point, in JavaScript on purpose. See below. It also answers `/bookmark/current` and `/bookmark/restore`, the bookmark calls the `worker` crate does not bind, and `/transaction-probe/<case>`, which drives `ctx.storage.transactionSync`, which it does not bind either. |
| `test/conformance.test.ts` | One `it()` per rule, each against its own object. |
| `test/conformance-cold.test.ts` | The same rules, each on an object primed and then evicted first. |
| `test/restart.test.ts` | The cross-restart rules: evicted (every subject), the no-restart control, and hibernated (the adapter). |
| `test/remint.test.ts` | The re-mint leg's refusals, and the pin that local `workerd` implements no restore. |
| `test/transaction.test.ts` | The `transactionSync` properties the `cfg(test)` ingest store's `node:sqlite` model assumes: a throw rolls back rows and `sqlite_sequence` and is rethrown, a return commits, `UPDATE OR IGNORE … RETURNING` answers no row on a conflict, and `exec` refuses `SAVEPOINT` and `BEGIN`. The routes are `/transaction-probe/<case>` in `worker.mjs`. |
| `test/idle.test.ts` | Idle objects holding the adapter, left alone for 12 s and for 8 s, and the timer control (`/idle-control/` in `worker.mjs`). |
| `test/limits.test.ts`, `test/platform-miss.test.ts` | The probe, and what the deployed leg retries. |
| `deployed.mjs` | The deployed leg's runner. |
| `deployed-restart.mjs` | The deployed leg's hibernation cells, over `wss://`, and its idle cells. Record-only. |
| `deployed-remint.mjs` | The deployed leg's bookmark-restore cells for `remint_identity`. Asserted when the restore runs, recorded when it cannot. |

### Why the entry point is JavaScript

A failing rule panics. Under `panic = "abort"` the panic traps the wasm instance
inside a microtask, and the promise the request is waiting on never settles. A
Rust entry point shares that instance with the object, so it dies too and the
request hangs until the runtime cancels it. To avoid that, `relay_panics` in
`src/object.rs` passes the panic message to `globalThis.__harnessPanic` before
the trap. `worker.mjs` races each object request against that relay, so a failing
rule rejects immediately with its assertion text. The shim that `worker-build`
generates rebuilds the instance on the next call.

### The token

On a deployed Worker the router refuses every request unless `HARNESS_TOKEN` is
set and the request sends it as a bearer token. The CI job mints a new token for
each run and passes it with `wrangler deploy --var`, so the code and the token
arrive in one deploy. A `wrangler secret put` after the deploy did not reach the
Worker within a minute on the first run. The job deletes the Worker at the end.

## What it discharges, and what it does not

Of the exclusions ADR-0023 lists for the shim, this harness runs the rules inside
an isolate, behind the I/O gate, and under the platform's SQLite limits.

A testkit rule runs inside a single request, and `REOPEN` is a fresh storage
handle, as it is on the shim. Eviction and hibernation are exercised by the
rules in `src/restart.rs` instead, phased across two requests with the restart
between them:

- **evicted**: `evictDurableObject` from the runner between the halves, for the
  adapter and for three wrong stores. `instance-log` keeps its log in the
  object instance. `fresh-high-water` keeps its events in the adapter but
  caches the high-water position in the instance, answers `head()` from it, and
  skips the conflict check for a condition at or above it. `mints-per-open`
  keeps its events in the adapter but mints its store identity per instance and
  stamps it on every event it reads, so events come back under new `EventId`s.
  Each must fail exactly the (rule, check) pairs `/restart/registry` lists, and
  the run prints `mutant rejections observed: N of N`. Two more wrong stores
  live in the host tests only (`src/restart/tests/stores.rs`), for the two
  checks no in-object store reaches: one that refuses every conditional append,
  and one that loses an old event after a reopen while still assigning fresh
  positions. A host test asserts that every check is some wrong store's verdict.
- **control**: the same cells with nothing between the halves. Every store
  passes every rule, so a wrong store's rejection above shows the rule ran
  across the restart and depended on it.
- **hibernated**: the adapter only. A hibernatable WebSocket is held open, the
  object is left alone past `workerd`'s 10-second inactivity timer, and a
  message on the socket wakes a new instance. The after-half requires the
  socket to have been restored.

Each construction of the object mints an incarnation nonce, and an after-half
told to expect a restart refuses with `409` if the before-half's instance
answers it, so no cell passes over a restart that did not happen. The runner
also checks that the wasm instance did *not* change: locally a restart tears
down the object and keeps the module instance, so a changed one means a trap.
The same fact is the limit of the local runs: state in the isolate's globals
survives a local restart, so a store that cached its high-water mark in a
`static` (the variant issue #262 itself describes) would pass here, and is not
built. The deployed leg runs the hibernation cells again over `wss://`,
record-only, because nothing pinned here documents Cloudflare's timing, and it
runs no wrong stores.

`test/idle.test.ts` asks the question the other way round: does holding the
adapter keep an idle object alive? An object holding a store, and one holding a
read stream it has opened and advanced by one event, are each left alone. At
12 s a new instance answers, so `workerd`'s 10-second timer evicted them; at
8 s the arming instance answers, and the held stream then yields every event it
had left, so what was parked was live. The positive control is the same hold
plus a pending `setInterval` set by `worker.mjs`, the kind of timer a
write-behind store would keep. It is not evicted at 12 s, which is what shows
the measurement can tell an object with something pending from one holding
only memory. The hold itself is one future the object owns and polls by hand,
because `read` borrows the store and a struct cannot hold both without
referring to itself. The deployed leg records the same cells and asserts
nothing, because nothing pinned here states when Cloudflare evicts an object
that holds no hibernatable socket.

`test/conformance-cold.test.ts` also runs the whole family on an object that
was primed (every log a rule may open migrated, each identity minted) and then
evicted. Each rule still runs inside one instance, so this is the weaker
evidence; what it adds is every rule against logs a previous instance created.

`remint_identity` is run across a real bookmark restore (#298), on the
deployed object only. Per cell, the object appends two events, the runner
takes a bookmark (`getCurrentBookmark`), the object appends two more and hands
the runner a ledger of all four, and the runner restores the bookmark
(`onNextSessionRestoreBookmark`, then `ctx.abort()`). On the next instance the
object checks that exactly the first two events remain under their old
`EventId`s, re-mints or does not, appends three more, and judges what it
yields against the ledger. The re-mint cell passes when no `EventId` names a
different event than it did before the restore and a lost position was handed
out again, so the restore did roll the position counter back. The control
skips the re-mint and passes when an `EventId` is reissued, which is the
hazard the re-mint exists for. Locally it cannot run: the `workerd` that
`package-lock.json` pins answers `getCurrentBookmark` with an all-zero bookmark
and refuses the restore with "does not implement point-in-time recovery".
`test/remint.test.ts` pins that refusal, and the two an after-half owes the
deployed run: across no restart, and across an eviction that restored nothing.
The deployed cells are asserted when the restore runs and recorded when it
cannot: a cell that observed the restore and failed the judge fails the step,
while a cell that could not run, or whose restore or restart never landed, only
warns, for the hibernation cells' reason.

It does not reach the memory ceiling.
