# happenstance-neon

The [Neon](https://neon.tech) serverless-Postgres event store and projection
store adapters for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance) — a
storage-agnostic event sourcing library built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: conformant, and on crates.io since `0.2.0`.** Up to `0.3.2` two rules
> lost a race intermittently against a live Neon endpoint; from `0.4.0` a
> read-settlement fence closes it, within one transport — see *ES-11, and the
> fence that meets it*, below.

## Which crate do I want?

- **Writing an application?** Use [`happenstance`](https://crates.io/crates/happenstance),
  and reach for this crate only to choose where the events live.
- **Talking to Postgres over a normal connection?**
  [`happenstance-postgres`](https://crates.io/crates/happenstance-postgres) is
  almost certainly what you want. Use this crate when you *cannot* hold a
  connection.
- **Writing your own adapter?** Read this one for what a port looks like when
  every affordance is taken away, then measure yours against
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit).

## What makes this different from `happenstance-postgres`

Neon's `/sql` endpoint is reached over one-shot HTTPS. There is:

- **no connection** — nothing to hold, nothing to pool;
- **no interactive transaction** — you cannot read a result and decide what to
  send next inside the same transaction;
- **no cursor** — a response either arrived whole or did not, under a hard
  **64 MiB** ceiling.

Everything `happenstance-postgres` leans on is unavailable. That is the point:
this crate exists to sit at the far end of the transport axis and find out which
of the contract's promises survive there.

The one thing the endpoint does offer beyond a single statement is a
**non-interactive batch**: an array of statements run server-side inside one
`BEGIN`/`COMMIT` at a chosen isolation level. Every capability limit this crate
records comes from that one sentence.

## The transport is yours

```toml
happenstance-neon = "0.3"
happenstance-neon = { version = "0.3", features = ["projection-store"] }
```

**This crate owns no socket.** `SqlTransport` takes the whole request by value,
returns the whole response buffered, and says when earlier reads are answered
(`ReadLedger` does the bookkeeping); you supply the implementation. That is
deliberate: a real client on the host drags in a TLS stack, and on `wasm32-unknown-unknown` the only way out of the sandbox is the
host's `fetch`. Those are two different clients and neither is what this crate is
for. `NullTransport` ships in-tree and fails every round trip.

The crate compiles for **both** the host and `wasm32-unknown-unknown`, and
implements the **bare** `EventStore` flavour so that one source file serves both.

## ES-11, and the fence that meets it

Up to `0.3.2`, `read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot` failed **intermittently**, always in the same
direction: the read saw an event appended after it was issued. It was a
**network race**: a read and an append are two independent requests to a pooled
proxy, and nothing ordered one backend's snapshot against another backend's
commit. ES-11 asks for an order *the store itself honours* (ADR-0061), and spawn
order at the client is not one.

**From `0.4.0` the order is the read's answer.** `SqlTransport::reads_settled`
is a required method, and `NeonEventStore::append` waits on it, once, before it
sends anything: it resolves when every read-only request the same transport
dispatched earlier has been answered. An answer follows the statement's
execution, so the read's snapshot precedes the append's commit. Measured against
the live endpoint (ADR-0087): 172 of 1,500 trials of the racing shape red
without the fence, 0 of 1,500 with it, and both rules green in every attempt.
ES-11 and ES-12 are `[FROZEN]` on that record.

**If you write a transport**, its obligations are on the trait: register every
read-only request in a `ReadLedger` *before* `round_trip` returns, and settle it
from whatever drives your I/O — a spawned task on the host, a `fetch` promise's
callback on `wasm32` — **never** from polling `round_trip`'s future. A transport
whose I/O advances only while that future is polled deadlocks an append behind a
read the caller polled once and set aside. `ReadLedger::settled` is the method's
body; a transport that never sends anything returns a ready future.

**The ordering domain is one transport value.** Stores built over clones of one
transport are ordered; two transports with separate ledgers are not, and no
conformance rule can observe the difference. That is the stated limit of this
crate's ES-11 claim. An append with no read in flight on its transport does not
wait; one that does waits up to one read round trip.

## Things the endpoint does that will surprise you

Each of these was measured against the live service, and each cost a debugging
session:

- **The isolation-level header is honoured on a batch and ignored on a single
  statement.** A conditional append written as one clever CTE therefore runs at
  READ COMMITTED, and two racers both probe empty and both insert. This adapter's
  append is a two-statement batch for that reason.
- **`bigint` comes back as a JSON *string*.** Which is a mercy — no `f64`
  precision loss — but it decodes differently from `integer`.
- **An empty `text[]` renders as `[""]`, not `[]`.**
- **`options=-c search_path=…` in the connection string is discarded.** Isolation
  between deployments is schema-qualified identifiers, which is why `NeonConfig`
  carries a schema.
- **A failing statement fails the whole batch**, with a single top-level error
  object rather than a partial result array. That is the atomicity the
  non-interactive batch promises, confirmed rather than assumed.

## Point the projection store at the primary

Give `NeonProjectionStore` a **read-write primary endpoint, never a read
replica**. Its `checkpoint` must reflect every commit the store has
acknowledged, through any handle (PS-38). A read replica can lag the primary,
so it reports an older position, and a runner resuming from it applies events
again that it already committed. No conformance rule can detect this, because
the suite runs against one endpoint, so the obligation is documented rather
than tested.

## Limits this store enforces

| | |
|---|---|
| `MAX_EVENT_DATA_LEN` | 128 KiB |
| `MAX_TAGS_PER_EVENT` | 128 |
| `MAX_EVENTS_PER_BATCH` | 128 |
| response ceiling | 64 MiB, hard — there is no cursor to fall back on |

They are lower than `happenstance-postgres`'s deliberately: payloads travel
base64-encoded inside a JSON body, and the worst case has to stay clear of the
response ceiling in both directions.

## Running its tests

They need a Neon endpoint, and they are `#[ignore]`d so that a machine without
one still has a green build:

```console
NEON_CONNECTION=postgresql://… cargo test -p happenstance-neon --all-features -- --ignored --list
NEON_CONNECTION=postgresql://… cargo test -p happenstance-neon --all-features -- --ignored --show-output --test-threads=1
```

`--test-threads=1` is this adapter's **visibility mechanism**, not a flake
workaround: `pg_snapshot_xmin` is held back by any open write transaction on the
branch, including sibling rules of the same run. Run in parallel, most of the
suite reddens; run serially, it does not.

**The counts that used to sit here were a phase-10b bring-up measurement** taken
before three other defects were fixed, and they had drifted out of agreement with
the status line at the top of this page — it said one rule does not pass while
this paragraph said three. The mechanism is the durable part and it is unchanged;
the numbers were not re-measured after the fixes, so they are not restated.

The same frontier is why the conformance fixture declines
`READ_YOUR_OWN_WRITES`, which makes the generated model family report a stated
skip rather than run. That is a property of the *server*, and distinct from
ES-11 above, which is a property of the *transport* and which the fence meets.

## Licence

MIT or Apache-2.0, at your option.
