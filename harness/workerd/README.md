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
```

To drive the deployed path without deploying, run `wrangler dev` and point
`deployed.mjs` at it:

```console
$ printf 'HARNESS_TOKEN=local\n' > .dev.vars
$ npx wrangler dev --port 8799 &
$ HARNESS_TOKEN=local node deployed.mjs http://127.0.0.1:8799 local-1
```

## How it fits together

| File | What it is |
| --- | --- |
| `src/object.rs` | The `#[durable_object]` class. It builds the adapter the way a production class does, from `SqlStorage::from_state`, and routes `/rules`, `/rule/<name>` and `/probe`. |
| `src/dispatch.rs` | The emitter. `for_each_event_store_rule!` expands into a `match` from rule name to rule. The name list comes from the same enumeration through the testkit's `__rule_names` callback, so a rule missing from the `match` fails as `no such rule` instead of disappearing. |
| `src/fixture.rs` | One fixture instance is one log under its own `TableNamespace` in the object's single database. That is how two rules that open two isolated stores at once run inside one object. |
| `src/probe.rs` | Phase A of `experiments/durable-object-limits`, run on this runtime. |
| `worker.mjs` | The entry point, in JavaScript on purpose. See below. |
| `test/*.test.ts` | One `it()` per rule, each against its own object, and the probe. |
| `deployed.mjs` | The deployed leg's runner. |

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
an isolate, behind the I/O gate, and under the platform's SQLite limits. It does
not exercise eviction or hibernation: a rule runs inside a single request, and
`REOPEN` is a fresh storage handle, as it is on the shim. It does not reach the
memory ceiling either.
