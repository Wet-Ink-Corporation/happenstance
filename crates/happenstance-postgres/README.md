# happenstance-postgres

The PostgreSQL event store and projection store adapters for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance) — a
storage-agnostic event sourcing library built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: conformant, and in the `0.2.0` release set.** **107 of 107** gated
> tests in the event-store binary pass against a live PostgreSQL 17.10 — the 95
> rules of the event-store family, the 5 of the concurrency family at 64
> contenders, the generated model family, and this crate's own six. The
> projection suite clears its own target beside it.
>
> Stated per target rather than as one total, because the total was wrong: this
> line said 132 and the four targets list 133 between them. A composite nobody
> can reproduce from a single command is a number that goes stale quietly, and
> this one had. This was the **first adapter in the portfolio to clear the concurrency
> family against a store whose writers are not serialised**, which is the whole
> reason the crate exists; `happenstance-neon` reaches the same server through a
> different transport and clears it too. Only the registry can say whether the
> release has happened yet.
>
> **One of those gated tests is a stated declension rather than a run.** The
> fixture declines `READ_YOUR_OWN_WRITES`, so the generated model family reports
> a skip carrying its reason instead of executing. That family predicts whether a
> conditional append will be refused from what a read showed it, and this store
> keeps those two sets apart on purpose — see *What this store costs a caller*.

## Which crate do I want?

- **Writing an application?** Use [`happenstance`](https://crates.io/crates/happenstance),
  and reach for this crate only to choose where the events live.
- **Want the contract without a driver?**
  [`happenstance-core`](https://crates.io/crates/happenstance-core) is the
  smaller semver surface, and it ships an in-memory reference store.
- **Want a single file rather than a server?**
  [`happenstance-sqlite`](https://crates.io/crates/happenstance-sqlite).
- **Writing your own adapter?** Read this one for the position-visibility
  problem, then measure yours against
  [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit). A
  crate that compiles is not an adapter until it has run that suite.

## Features

```toml
happenstance-postgres = "0.2"                              # the event store
happenstance-postgres = { version = "0.2", features = ["projection-store"] }

sqlx = "0.8"                                               # only for queries of your own
```

You do not need that third line to name the driver. `PgPool` and `sqlx::Error`
are on this crate's public signatures, so `sqlx` is re-exported: reach it as
`happenstance_postgres::sqlx::…`. That buys *type identity* — the `sqlx::Error`
you match on is the one this crate's error enum actually carries.

`projection-store` is off by default and forwards `happenstance-core`'s own
`unstable-projection` gate. **That port is not frozen**, carries a documented
semver exemption, and the reason it is still gated is recorded in ADR-0060: the
signatures a freeze would promise are the ones that forbid the second batch
shape from existing.

## The problem this crate is here to solve

`nextval()` allocates **outside** the transaction. So on any naive Postgres
schema, position order is not visibility order: a reader can see position 7
while 6 is still uncommitted, and a projection checkpointing on `after` skips
the events that appear below it. Data loss, no error, no failing test.

The specification calls that ES-10 — *once a position is visible, nothing below
it becomes visible later* — and every other adapter in this workspace satisfies
it for free by assigning positions under a lock it holds until commit. This one
cannot, which is why it is an **instrument before it is a target**.

What it does instead is ADR-0024's arm C, chosen on a measurement rather than a
preference: an `xid8` column defaulted to `pg_current_xact_id()`, and every read
bounded by `xact_id < pg_snapshot_xmin(pg_current_snapshot())`. Position order
stops being treated as visibility order, and a frontier predicate goes on the
read side. The two serialising alternatives measured 16× and 30× at 64 writers;
this one measured 0.99–1.03× of baseline.

**The cost is real and is stated rather than buried.** Any long-running
transaction anywhere in the database holds `pg_snapshot_xmin` back, so one
forgotten `BEGIN` in an unrelated application delays what this store's readers
can see. It does not corrupt anything and it does not error; it makes readers
lag. `head()` reports the frontier, not the newest row.

## Two roles, one pool

```rust,ignore
let pool = PgPool::connect(url).await?;
happenstance_postgres::migration::apply(&pool).await?;

let events = PostgresEventStore::new(pool.clone());
let views  = PostgresProjectionStore::new(pool);
```

Sharing one pool is the expected deployment, which is why neither type builds
its own. The schema lives in `migrations/` as SQL and is compiled in, so a
consumer needs nothing from the repository to apply it — `migration::MIGRATION_1`
for the event log and `migration::MIGRATION_2` for the projection checkpoint.
They are applied separately on purpose: an application using only the event store
gets no `projection_checkpoint` table.

## What the projection batch is, and what it costs you

`PostgresProjectionBatch` is an **owned buffered write set**, not a live
`sqlx::Transaction`. That is not a shortcut. `ProjectionStore::begin` is total,
synchronous and infallible, and every route to a `sqlx` transaction is `async`
and fallible — so the transaction is opened inside `commit`, which replays your
statements and writes the checkpoint in one unit.

The consequence for you: **a projection whose `apply` must read what it has
already written in the same batch cannot be written against this adapter.**
`READS_THROUGH_BATCH` is `false` and the conformance suite reports it as a skip
rather than passing quietly.

## Limits this store enforces

| | |
|---|---|
| `MAX_EVENT_DATA_LEN` | 1 MiB |
| `MAX_TAGS_PER_EVENT` | 128 |
| `MAX_EVENTS_PER_BATCH` | 256 |

## Running its tests

They need a Docker daemon, and they are `#[ignore]`d so that a machine without
one still has a green build:

```console
cargo test -p happenstance-postgres --all-features -- --ignored --list
cargo test -p happenstance-postgres --all-features -- --ignored --show-output --test-threads=1
```

`--list` first, so "no rule is absent from the run" is proven rather than
assumed. `--test-threads=1` is not a flake workaround: it is connection
arithmetic, and the reasoning is in the test module.

## Licence

MIT or Apache-2.0, at your option.
