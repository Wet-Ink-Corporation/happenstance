# happenstance-neon

The [Neon](https://neon.tech) serverless-Postgres event store and projection
store adapters for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance) — a
storage-agnostic event sourcing library built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: conformant with one open clause question, and in the `0.3.1` release
> set.** 124 gated tests pass against a live Neon endpoint. One does not, and it
> is stated here rather than hidden — see *The one rule this adapter does not
> pass*, below. Only the registry can say whether the release has happened yet.

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

**This crate owns no socket.** `SqlTransport` is a one-method trait taking the
whole request by value and returning the whole response buffered, and you supply
the implementation. That is deliberate: a real client on the host drags in a TLS
stack, and on `wasm32-unknown-unknown` the only way out of the sandbox is the
host's `fetch`. Those are two different clients and neither is what this crate is
for. `NullTransport` ships in-tree and fails every round trip.

The crate compiles for **both** the host and `wasm32-unknown-unknown`, and
implements the **bare** `EventStore` flavour so that one source file serves both.

## The one rule this adapter does not pass

`read_result_is_stable_under_concurrent_append`, intermittently. It fails less
often over a single multiplexed HTTP/2 connection than over HTTP/1.1 with a
default pool, which is why the reference transport uses the former, and every
observed failure was in the same direction: the read saw an event appended after
it was issued.

**No failure rate is quoted here.** Frequencies were counted during phase 10b
bring-up and no raw log of that session was retained; every other measurement
this project cites lives beside its own output under `experiments/`, and a ratio
with nothing behind it does not belong on the page a reader trusts most.

It is a **network race, not a bug we have not found yet.** A read and an append
are two independent requests to a pooled proxy, and nothing orders one backend's
snapshot against another backend's commit. The specification's ES-11 tells an
asynchronous driver that *"a read spawned at its first poll and an append spawned
afterwards land in the same queue in that order"* — true where both operations
enter one pool, and false by construction where there is no queue at all.

ES-11 is `[PROVISIONAL]`, and its own marker named a one-shot-HTTP adapter as the
thing that would falsify it. This is that adapter. ADR-0061 settled what follows:
the clause's sufficiency condition is corrected to say *spawned at the first poll,
**and** ordered against a later append by something the store itself honours* —
which removes spawn order alone as a route to a conformance claim rather than
weakening what a store must do — and **this crate does not claim ES-11**. The
conformance job that checks it is deliberately kept strict rather than taught to
tolerate a named failure.

The figure is **104 of 105 rules observed**, and the missing certainty is worth a
sentence: `query_items_share_one_snapshot` passes, but it appends after the first
poll and asserts the drained set unchanged, which is structurally the same race.
It has not lost it in sixty measured runs. That is not the same as immunity.

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
skip rather than run. That is a property of the *server*, and distinct from the
ES-11 limitation above, which is a property of the *transport*.

## Licence

MIT or Apache-2.0, at your option.
