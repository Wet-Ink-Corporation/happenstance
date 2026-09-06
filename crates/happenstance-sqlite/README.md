# happenstance-sqlite

The SQLite event store and projection store adapters for
[happenstance](https://github.com/Wet-Ink-Corporation/happenstance) — a
storage-agnostic event sourcing library built on the
[Dynamic Consistency Boundary specification](https://dcb.events/specification/).

[![Buy me a coffee](https://img.shields.io/badge/buy%20me%20a%20coffee-support-yellow?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ryanbritton)

> **Status: conformant, and in the `0.2.0` release set.** This is the first
> happenstance adapter to pass
> [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit)'s
> conformance suite against a real file on disk — the event store family, the
> concurrency family, the generated model family and the projection family. The
> `publish = false` this callout used to name is gone: the crate-set re-plan put
> it in the release beside `happenstance`, `happenstance-core` and
> `happenstance-testkit`. Only the registry can say whether that release has
> happened yet.

## Which crate do I want?

- **Writing an application?** Use [`happenstance`](https://crates.io/crates/happenstance),
  and reach for this crate only to choose where the events live.
- **Want the contract without a driver?**
  [`happenstance-core`](https://crates.io/crates/happenstance-core) is the
  smaller semver surface, and it ships an in-memory reference store.
- **Writing your own adapter?** Read this one as a worked example, then measure
  it against [`happenstance-testkit`](https://crates.io/crates/happenstance-testkit).
  A crate that compiles is not an adapter until it has run that suite.

## Features, and the one that costs you a promise

```toml
happenstance-sqlite = "0.2.0-alpha.1"                      # the event store
happenstance-sqlite = { version = "0.2.0-alpha.1", features = ["projection-store"] }

rusqlite = "0.40"                                          # only for tables of your own
```

**You do not need that third line to name the driver.** `Connection`,
`rusqlite::Error` and `rusqlite::types::Value` are all on this crate's public
signatures, so all three are re-exported: reach them as
`happenstance_sqlite::rusqlite::…`. What that buys is *type identity* — the
`rusqlite::Error` you match on is the one this crate's error enum actually
carries, rather than a second copy that prints the same and meets you as
`error[E0308]` the first time you try to act on a failure. Add a `rusqlite`
line of your own when you use SQLite for tables of your own; the re-export is
not a substitute for a dependency, and it forwards only the features this crate
enabled.

**`tokio` is not re-exported, and it is the one place you may need a line we do
not give you.** `tokio::task::JoinError` and `tokio::runtime::TryCurrentError`
are variants of the error enums, so matching on either means naming `tokio`
yourself:

```toml
tokio = "1"                                                # only to match on JoinError
```

It was re-exported and is not any more. This crate takes `tokio` at
`features = ["rt"]`, so `happenstance_sqlite::tokio` was a *partial* `tokio` —
no `macros`, no `rt-multi-thread`, no `time` — and reaching for it and then
writing `#[tokio::main]` produced an `error[E0433]` further from its cause than
the mismatch the re-export was there to prevent. Your own line resolves to the
same `tokio` this crate uses for any semver-compatible requirement, which is
every consumer who already had one.

**`event-store` is on by default. `projection-store` is not, and the asymmetry
is deliberate.**

The event store implements a port whose clauses are frozen. The projection store
implements one that is not: `ProjectionStore` is provisional, and PS-2's bar for
freezing it — a hostile store failing the suite, *and* two adapters at opposite
ends of the batch-shape axis passing it — is met on its first half only. One
adapter has run the projection suite. So `projection-store` forwards
`happenstance-core`'s `unstable-projection` gate, and enabling it opts you into a
surface that makes **no semver promise**: it can change shape in a patch release,
and `cargo-semver-checks` will not stop it.

That is worth having and it is worth choosing. Nothing about the event store
changes either way, and a projection built on this today is a projection you may
have to edit at the next release.

## The shape it represents

**Serialising, `Send`, native.** One `rusqlite::Connection` behind one `Mutex`,
with no pool: writers queue by construction and positions are assigned under a
lock. `rusqlite` is synchronous, so every statement runs on a blocking task, and
the read stream defers its `spawn_blocking` until the first poll — which is what
lets `read` return the stream at the top level rather than from inside a future.

It is deliberately *one* point in a portfolio rather than the reference. A port
frozen against this shape alone would be frozen against SQLite wearing four hats,
which is why the workspace keeps instruments at the other end of the same axis —
a store that assigns positions outside the transaction, and one with no
interactive transaction at all.

```rust,ignore
use happenstance_sqlite::event_store::SqliteEventStore;

let store = SqliteEventStore::open("events.db")?;
```

## Durability, as configured rather than as hoped

Every connection this crate opens runs under the same three settings, and each is
a public constant you can assert against rather than a paragraph you have to
trust:

| Setting | Value | Why |
| --- | --- | --- |
| `journal_mode` | `WAL` | readers do not block the writer, which is what makes a long replay and a second handle onto one file workable at the same time |
| `synchronous` | `NORMAL` (`1`) | durable across a process crash; on power loss it can lose the tail since the last checkpoint. `OFF` is not on the menu — the specification names it as the wrong implementation the reopen rule exists to reject |
| `busy_timeout` | `5,000` ms | finite *and* generous. Measured to absorb 64-way write contention with no `SQLITE_BUSY`; an unbounded handler would turn a livelock into a hung job that names no rule |

## Limits this store enforces

`append` refuses anything above these as `AppendError::ExceedsStoreLimit` — never
by truncating, and never as a generic store error:

| Limit | Value |
| --- | --- |
| bytes of `data` on one event | `1,048,576` |
| tags on one event | `128` |
| events in one append | `256` |

Every one of them is well above the floor the specification requires of any
store, and every one is checked from both sides: the suite appends exactly the
limit and requires it to be accepted, then one more and requires it to be
refused.

Positions may have gaps. The table is `AUTOINCREMENT`, so a position is never
reused after a delete, and nothing you write against this store may assume the
next one is `+ 1`.

## Licence

MIT OR Apache-2.0.
