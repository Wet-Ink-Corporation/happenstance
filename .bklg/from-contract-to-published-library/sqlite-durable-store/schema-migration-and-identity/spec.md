---
item: HS-S0036
stage: spec
created: 2026-08-12T13:46:34.904Z
updated: 2026-08-12T13:46:34.904Z
template_sig: 87bbf1d0
rendered_sig: f926bbd0
---

# Spec — Migration 1: the amended schema, persisted StoreId, and idempotent concurrent open

## Scope lock

| Layer | Path |
| --- | --- |
| Initiative (gold source) | [`.bklg/from-contract-to-published-library/initiative.md`](../../initiative.md) |
| Initiative decomposition | [`.bklg/from-contract-to-published-library/_decomposition.md`](../../_decomposition.md) |
| Project | [`.bklg/from-contract-to-published-library/sqlite-durable-store/project.md`](../project.md) |
| This spec | `.bklg/from-contract-to-published-library/sqlite-durable-store/schema-migration-and-identity/spec.md` |
| Key briefs | [`../_decomposition.md`](../_decomposition.md) — **architecture brief §7** (schema, migration, identity), **§6** (journal and durability pragmas, `SQLITE_BUSY`), **§2** (the Accepted atoms that bind), **§1** (the mount table); **testing brief §3** (what may not be doubled), **§5** (`recorded_time_survives_a_reopen`'s missing negative control) |
| Signed-off design | [`../_design.md`](../_design.md) — records **N/A, no user-facing surface** for this whole project, approved 2026-08-12. Nothing in it binds this story to a surface; what it does bind is that the no-surface determination is not a licence to skip documenting the public Rust items this story adds |
| Story map row | [`../_storymap.md`](../_storymap.md) — slice `durable-event-store`, foundation, `depends_on: adr-0022-append-condition-strategy`, `traces_to: AC-001, AC-004` |
| Grounding | [`../_grounding.md`](../_grounding.md) — the Accepted atoms and existing code patterns this project was grounded against |
| Roadmap pointer | `RUNBOOK.md:4166-4237` (phase 8 in full); `RUNBOOK.md:4178-4187` (the two schema amendments this story implements) |

## One-line PR slice

Land migration 1 as ADR-0022 amends it — `event_tag` keyed `(tag, position)` with `event_type` as a covering column, `tag_cardinality`, the `EventId` origin pair `UNIQUE`, `recorded_at`, and a `StoreId` minted once and persisted — idempotent under two concurrent opens, with WAL, a stated `synchronous` and a finite busy timeout.

## Executive summary

`SqliteEventStore::migrate` is `todo!("SQLite event store: schema migration")`
(`crates/happenstance-sqlite/src/event_store.rs:139-141`). This PR gives it a real
body, and with it the four facts every other story in the `durable-event-store`
slice is waiting on: a table layout that does not force the append-condition probe
into a join under the write lock, a `StoreId` that is the *same* value after the
file is closed and re-opened, a `recorded_at` column that is read back rather than
re-stamped, and a connection whose journal mode, `synchronous` setting and busy
timeout are what ADR-0022 said they would be.

**Delta, not restatement.** `project.md` says the schema is in scope and the
architecture brief §7 says what it must hold; neither says what a *reviewer looks
at* to know it happened. This story's answer is a new real `cargo test` target,
`crates/happenstance-sqlite/tests/migration.rs`, that opens a real file, reads the
schema and the pragmas *back out of SQLite*, races N concurrent opens against one
fresh path, and closes-and-reopens to prove the identity and the stamp survived. It
is the first target in this crate that runs SQL at all.

Three things this story adds that the briefs imply and no artifact yet states as an
obligation: `SqliteEventStore` gains a persisted-identity accessor and an explicit
re-mint operation, because VT-6 (`spec/SPECIFICATION.md:783-841`) permits mint-once
**only** for an adapter that can detect a restore *or* whose deployment is
documented to invoke a re-mint — and a SQLite file that was `cp`'d cannot detect
anything; the chosen mechanism is recorded in `references/adapter-shapes.md`,
which VT-6 makes a MUST and which that file does not currently carry for any
adapter; and the module-doc SQL block at `crates/happenstance-sqlite/src/event_store.rs:34-58`
is rewritten in this PR, because the moment `migrate` is real, a wrong `# Intended
schema` block stops being a sketch and becomes published documentation that
contradicts the code beneath it.

## Context pack

Read this section and you can start. Everything below the Integration contract is
either a boundary or a signposted anchor.

### The decision this story is downstream of

**ADR-0022 is written first and this story spends it.** `adr-0022-append-condition-strategy`
(HS-S0035) is a hard predecessor, not a sequencing convenience: the append-condition
strategy decides the index shape, and the index shape *is* migration 1. Do not
re-open any of it here. Six values arrive from that ADR and are consumed, not
chosen, in this PR: the append-condition strategy the schema must serve, the tag
storage (join table vs canonical blob vs JSON1), the busy-timeout value, the
`synchronous` setting, the journal mode, and whether WAL is set per connection or
persisted in the file (architecture brief §12; `crates/happenstance-sqlite/src/lib.rs:47-65`).
**If ADR-0022 has not landed when this story starts, stop and say so** — inventing
the numbers here is exactly the "decision deferred to a measurement that never
happens" the runbook names (`RUNBOOK.md:4227-4228`).

### The schema the crate's own docs get wrong

The sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` is **known to be
wrong**, and the correction lives in `RUNBOOK.md:4178-4187` rather than in the code.
`event_tag(tag, position)` carries no type, so a `QueryItem`'s type constraint
becomes a join back to `event` — which collapses SQLite's `MERGE (UNION)` streaming
plan into a co-routine over a temp b-tree and makes the append-condition probe walk
the full posting list through a join *while holding the `BEGIN IMMEDIATE` write
lock*. That serialises every writer. **Implementing the doc comment as written is
the most likely way to ship a slow, conformant adapter** — it passes every rule.

The two amendments, therefore, are not optional and not this story's to re-argue:
carry `event_type` as a **covering column** on `event_tag` with the key left
`(tag, position)` so the range stays sorted by position; and add a
`tag_cardinality` table, because multi-tag arms must be probed
most-selective-tag-first and SQLite cannot supply per-value cardinality — `ANALYZE`
stores only an average.

`AUTOINCREMENT` on `event.position` is load-bearing and is not a style choice:
plain `rowid` reuses a value after a delete, and the specification requires
uniqueness across the store's whole lifetime. It is also half of what makes
mint-once identity safe (below).

### Identity: mint once, persist, and earn the right to do so

`EventId` is `EventId::new(StoreId, SequencePosition)` with private fields
(`crates/happenstance-core/src/identity.rs:97-124`), and `StoreId` is sixteen
opaque bytes with `from_bytes` as its only constructor — `happenstance-core` mints
nothing, because it has no entropy source and is `no_std`-capable (ADR-0014,
`.kb/decisions/0014-event-identity-and-recorded-time.md`).

**The decision: mint once at schema creation, persist, read back on every open.**
VT-6 permits either mechanism and the testkit's own `DurableFixture` switched to
mint-once for exactly this reason and says so
(`crates/happenstance-testkit/tests/fixture_instruments.rs:75-90`); mint-per-open
is the wrong choice for a file-backed store, because it splits one store's history
into many origins whose interleaving is no longer recoverable.

**What that costs, and it is the part most likely to be skipped.** VT-6 does not
grant mint-once unconditionally: an adapter may take it *only if* it can detect
that its state was restored or cloned, **or** the deployment is documented to
invoke a re-mint; an adapter that can do neither **MUST** mint fresh on every open
(`spec/SPECIFICATION.md:826-836`). A SQLite file copied from Friday's backup is
undetectable. So mint-once here is legitimate **only when this PR also ships an
explicit re-mint operation and documents when a deployment invokes it** — and VT-6
closes with a second MUST: the chosen mechanism is recorded in
`references/adapter-shapes.md`, which today records no mechanism for any adapter.

Two storage constraints travel with it. The `StoreId` is persisted **as bytes or as
text, never as an integer or a pair of integers** — Workers SQL widens integers
through a JS number and bounds them at 2^53, and a 128-bit value cannot survive
that (`spec/SPECIFICATION.md:838-841`). And the `EventId` origin pair —
`origin_store` and `origin_position` — is stored and `UNIQUE` **together**, which
is simultaneously the index `contains_event_id`'s probe seeks and the constraint
that keeps ingest from storing one event twice (`crates/happenstance-sqlite/src/event_store.rs:237-244`).

Mint-once plus `AUTOINCREMENT` is what satisfies
`reopened_store_does_not_reissue_an_event_id` (`crates/happenstance-testkit/src/suite.rs:2310`):
the `StoreId` is stable across the reopen, so the *position* is what must never
repeat — and `AUTOINCREMENT` is the mechanism. Mint-once with plain `rowid` would
fail that rule after a delete, which is the concrete reason the two decisions are
one decision.

### Migration runs on every connect, and two connects can race

`SqliteEventStore::open` calls `migrate` on **every** connection
(`crates/happenstance-sqlite/src/event_store.rs:120-124`), and the fixture that
arrives with `sqlite-fixture-and-whole-suite` connects twice onto one file. So two
`CREATE TABLE`s can race, and two racing opens of a *fresh* file must not mint two
`StoreId`s. The shape: `IF NOT EXISTS` inside a `BEGIN IMMEDIATE`, and
`INSERT OR IGNORE` **then read back** for the identity row — the read-back is the
load-bearing half, because the loser of the insert race must adopt the winner's
value rather than keep the one it generated.

The projection store migrates independently, on its own connection
(`crates/happenstance-sqlite/src/projection_store.rs:97-100`) — an event store and a
projection store on one file are two connections, not one. Its `migrate` body is
**not** this story's; what is this story's is that both `open` paths configure the
connection identically, because a projection connection with no busy timeout will
simply fail against the event store's `BEGIN IMMEDIATE` write lock.

### Pragmas: executed is not the same as in effect

WAL is what makes two connections onto one file workable. `synchronous` is a
documented property of this adapter rather than a knob: CF-14
(`spec/SPECIFICATION.md:7471-7490`) names `PRAGMA synchronous = OFF` **by name** as
the wrong implementation the reopen rule exists to reject, so `OFF` is forbidden
here by a clause and not by taste. The busy timeout is **finite and generous**:
with `CONTENDERS` connections on one file, `BEGIN IMMEDIATE` on a busy database
returns `SQLITE_BUSY` *immediately* unless a busy handler is configured, and that
error becomes `AppendError::Store` → `Attempt::Failed` → a red rule that is not
about the adapter's logic. But there is **no watchdog anywhere in the suite and
there must not be one** (CF-33), so an *unbounded* busy handler converts a livelock
into a hung CI job that names no rule.

A pragma that was executed is not a pragma that is in effect — a mis-spelled or
unsupported pragma is silently accepted by SQLite. This story therefore **reads
each one back** through `PRAGMA journal_mode` / `synchronous` / `busy_timeout` on
the live connection and asserts on the returned value, per handle.

### The persona-journey slice this realizes

The journey is *"Learn when you are finished"* — the adapter author's loop, from a
signature that type-checks to a suite that says pass or fail and names why
(`.bklg/from-contract-to-published-library/initiative.md:245-246`). This story is
the first step of that loop for a real database, and it is deliberately the least
glamorous one: nothing here is user-observable on its own. Its consumers are named
and in the same project — `append-atomicity-and-store-limits`,
`lazy-read-with-snapshot-ceiling` and `projection-store-passes-the-borrowed-suite`
(`../_storymap.md`, *Why the three foundations are foundations*). Its persisted
`StoreId` and read-back `recorded_at` are the architectural precondition that makes
the reopen rules **askable at all**, which is what ties a schema story to project
AC-004.

### Boundaries the implementer will be tempted to cross

- **`SqliteEventStore::open_in_memory` is not deleted and not used by anything in
  this slice's fixture.** A private in-memory database is per-*connection*, so a
  second `connect()` would open a second, empty database (architecture brief §1).
  It stays a convenience constructor for single-handle callers.
- **`head` must never become a cached field.** ADR-0013's corollary is already
  written into the skeleton (`crates/happenstance-sqlite/src/event_store.rs:226-235`):
  one file backs several handles, so a cached head would report a value that
  predates another connection's commit. The schema this PR lands must not
  introduce a materialised head row that invites it.
- **No new public API in `happenstance-core`.** Architecture brief §5's decision —
  query decomposition is adapter-private — applies here too; nothing this story
  needs is missing from the contract crate.
- **`publish = false` stays**, `PUBLISHABLE` in `xtask/src/package.rs` stays, and
  `Cargo.toml`'s stale description is `crates-io-name-and-packaging-facts`'s to
  replace (architecture brief §8).
- **No clause marker moves.** ES-35, CF-14 and CF-17 leave the project with
  verdicts, and that is `reopen-negative-control-and-durability-verdicts`'s work.
  This story supplies the *evidence*; it edits no `[PROVISIONAL]` marker.

## Integration contract

- **Archetype**: `foundation` — real in-tree substrate, consumed and demonstrated
  by capability slice-mates in this same project. It is not a double, not a flag
  and not a fixme: it ships real SQL against a real file.
- **Slice / milestone**: `durable-event-store`. Slice-mates, implemented in one
  context and mounted as one integrated surface:
  `append-atomicity-and-store-limits`, `lazy-read-with-snapshot-ceiling`,
  `wide-query-chunked-not-refused`, `sqlite-fixture-and-whole-suite`.
- **Mount point**: `crates/happenstance-sqlite/tests/migration.rs` — a **new, real
  `cargo test` target**, run by `cargo test -p happenstance-sqlite` inside
  `cargo xtask ci --fast`. It is the composition root for this story because the
  slice's ultimate root, `crates/happenstance-sqlite/tests/conformance.rs`, cannot
  exist yet: `event_store_conformance!` drives `append` and `read`, which are still
  `todo!()` until the slice-mates land. This target is **not** retired when
  `conformance.rs` arrives — it asks three questions the conformance suite never
  asks: what SQLite actually created, whether two concurrent opens agree, and
  whether a pragma is in effect. Architecture brief AC-A01's rule is honoured in
  substance: reachable from `tests/`, never from a `mod` that only `cargo check`
  sees.
- **Wires into**:
  - `crates/happenstance-core/src/identity.rs:44-170` — `StoreId::from_bytes` /
    `to_bytes`, `EventId::new`/`store()`/`position()`, `RecordedAt::from_millis`.
    The contract crate mints nothing; this adapter does.
  - `crates/happenstance-core/src/memory.rs:97-107, 163-166` — the reference
    shape for a store that carries a `StoreId` field and exposes it
    (`with_store_id`, `store_id()`, `restore`). Read it before inventing one.
  - `crates/happenstance-sqlite/src/event_store.rs:102-142` — `new` / `open` /
    `open_in_memory` / `migrate`, the four constructors this story rewires.
  - `crates/happenstance-sqlite/src/projection_store.rs:79-100` — the second
    connection onto the same file; it adopts this story's connection
    configuration and keeps its own `migrate` as `todo!()`.
  - `crates/happenstance-sqlite/tests/shapes.rs` — the existing type-level guard
    that catches a field added carelessly. It must still pass, unchanged in intent,
    after `SqliteEventStore` gains its identity field.
  - `crates/happenstance-testkit/tests/fixture_instruments.rs:75-102` — the
    mint-once precedent and the process-local-ordinal pattern for a temporary
    resource with no new dependency.
  - `references/adapter-shapes.md` — the VT-6 record of which identity mechanism
    this adapter chose.
- **Renders surfaces**: **none.** `../_design.md` records N/A — no user-facing
  surface — for this entire project, approved at the design sign-off gate. The
  public Rust items this story adds (an identity accessor, a re-mint operation)
  are library surface inside that determination, and each carries rustdoc with an
  `# Errors` section naming conditions rather than types.
- **Conformance rule(s)**: this story adds **no** conformance rule and changes no
  port, so nothing in `suite.rs` observes it *directly*. What it makes runnable, by
  name, is `acknowledged_writes_survive_a_reopen`
  (`crates/happenstance-testkit/src/suite.rs:332`),
  `reopened_store_does_not_reissue_an_event_id` (`:2310`),
  `recorded_time_survives_a_reopen` (`:2473`),
  `contains_event_id_reports_membership` (`:2551`) and
  `two_handles_observe_each_others_appends` (`:265`) — all of which stay red or
  unrunnable until the slice-mates supply `append`, `read` and the fixture. This
  story is where they stop being impossible.
- **Clause(s)**: discharges nothing on its own; supplies the evidence for **VT-6**
  (`spec/SPECIFICATION.md:783-841`, `[PROVISIONAL]`) by recording a mechanism, and
  the preconditions for **ES-35** (`:4142-4167`), **CF-14** (`:7471-7490`) and
  **CF-17** (`:7570-7583`). **No `[FROZEN]` clause is touched and no marker is
  edited in this PR** — the verdicts are
  `reopen-negative-control-and-durability-verdicts`'s.
- **Advances DoD scenario**: initiative **DoD 3** — *"The durable store passes the
  suite for real … including an acknowledged write surviving a process reopen"*
  (`.bklg/from-contract-to-published-library/initiative.md:366-368`). This story is
  the half of DoD 3 that makes "surviving a process reopen" a question the workspace
  can express; project AC-001 and AC-004 are the rows it traces to.

## PR boundary

```
crates/happenstance-sqlite/src/**
crates/happenstance-sqlite/tests/migration.rs
references/adapter-shapes.md
.bklg/from-contract-to-published-library/sqlite-durable-store/schema-migration-and-identity/**
```

**In this PR**

- A real `SqliteEventStore::migrate` body: migration 1's tables, indexes and
  constraints, applied idempotently inside one `BEGIN IMMEDIATE`.
- The persisted `StoreId`: minted once at schema creation, stored as bytes or text,
  read back on every open into a field on `SqliteEventStore`, with an accessor and
  an explicit, documented re-mint operation.
- Connection configuration shared by both stores in this crate — journal mode,
  `synchronous`, busy timeout — at the values ADR-0022 records, read back and
  asserted rather than assumed.
- `crates/happenstance-sqlite/tests/migration.rs`, the new target that executes all
  of the above against a real temporary file, including the concurrent-open race
  and a close-and-reopen.
- The `# Intended schema` block in `crates/happenstance-sqlite/src/event_store.rs`
  rewritten to the schema `migrate` actually creates, and the `contains_event_id`
  comment at `:237-244` updated where it says the origin columns are absent.
- The VT-6 mechanism record appended to `references/adapter-shapes.md` — **appended
  after line 297**, because `spec/SPECIFICATION.md` cites that file at `:97-102`,
  `:220` and `:297`, and an insertion above those lines silently re-points three
  citations `cargo xtask spec-trace` will not catch as wrong, only as resolving.
- The one-line adoption of the shared connection configuration in
  `SqliteProjectionStore::open`.

**Explicitly not in this PR**

- `append`, the page query, `head`, `contains_event_id` — the slice-mates'
  (`append-atomicity-and-store-limits`, `lazy-read-with-snapshot-ceiling`).
- `SqliteProjectionStore::migrate`'s body and any read-model or checkpoint SQL —
  `projection-store-passes-the-borrowed-suite`.
- `SqliteFixture`, `tests/conformance.rs`, and any testkit macro invocation —
  `sqlite-fixture-and-whole-suite`.
- The runtime-`Handle` seam (architecture brief §3) — first exercised by
  `concurrency-family-and-contender-count`.
- Deleting `#![allow(clippy::todo)]` or rewriting `lib.rs`'s status banner —
  `instrument-markers-removed-and-gate-green` (DR-01: the allow dies with the
  *last* `todo!()`, not before).
- `Cargo.toml` — no new dependency, no `publish` change, and the stale
  `description` is `crates-io-name-and-packaging-facts`'s.
- Any `spec/SPECIFICATION.md` edit, any `[PROVISIONAL]` marker move, any ADR
  authoring.

**Merge DoD one-liner** — `cargo xtask affected --base main` green with
`crates/happenstance-sqlite/tests/migration.rs` running real SQL against a real
file, the schema and pragmas read back out of SQLite rather than asserted from the
code that wrote them, `cargo xtask spec-trace` still resolving every citation into
`references/adapter-shapes.md`, and no `todo!()` removed outside `migrate`.

## Behavior and interfaces

| Behavior or contract | Details | Evidence path |
| --- | --- | --- |
| **Migration 1 creates the amended schema** | `event(position INTEGER PRIMARY KEY AUTOINCREMENT, event_type, data, metadata, tags, origin_store, origin_position, recorded_at)`; `event_tag(tag, position)` `WITHOUT ROWID` with **`event_type` as a covering column** and the key left `(tag, position)`; a `tag_cardinality` table. Column names and types beyond these are the implementer's. | `RUNBOOK.md:4178-4187`; `../_decomposition.md` architecture brief §7; against the wrong sketch at `crates/happenstance-sqlite/src/event_store.rs:36-54` |
| **`AUTOINCREMENT`, not `rowid`** | Positions are never reused after a delete. Gaps are permitted and no code or test may assume `+1`. | `crates/happenstance-sqlite/src/event_store.rs:56-58`; `CLAUDE.md` — *never assert on literal position values* |
| **The origin pair is stored and `UNIQUE` together** | `(origin_store, origin_position)` — one constraint serving two jobs: the index `contains_event_id` probes, and the guard that stops ingest storing one event twice. Not two separate uniques. | `crates/happenstance-sqlite/src/event_store.rs:237-244`; `crates/happenstance-core/src/identity.rs:97-124` |
| **`recorded_at` is a stored column, read back** | Written once at append time (by a later story) and returned as stored on every read. Never re-stamped on open — that is precisely the defect `recorded_time_survives_a_reopen` exists to reject. | `.kb/decisions/0014-event-identity-and-recorded-time.md`; `crates/happenstance-testkit/src/suite.rs:2473` |
| **A `StoreId` is minted once at schema creation and persisted** | Sixteen bytes from a real entropy source in *this* crate — `happenstance-core` has none. Persisted as bytes or text, **never** as an integer or pair of integers. Read back into a field on `SqliteEventStore` on every `open`. | `spec/SPECIFICATION.md:783-841` (VT-6); `crates/happenstance-core/src/identity.rs:44-60`; precedent `crates/happenstance-testkit/tests/fixture_instruments.rs:75-102` |
| **Mint-once is legitimised, not assumed** | A public, documented re-mint operation exists and its rustdoc states when a deployment invokes it (after a restore or a clone). Without it, VT-6 requires mint-per-open. | `spec/SPECIFICATION.md:826-836` |
| **The mechanism is recorded where VT-6 says** | A new section appended to `references/adapter-shapes.md` naming this adapter's choice and its condition. Appended below line 297 so the three `SPECIFICATION.md` citations keep pointing at what they meant. | `spec/SPECIFICATION.md:836-838`; citations at `spec/SPECIFICATION.md:840, 1208, 8095` |
| **`SqliteEventStore` exposes its identity** | A `store_id()` accessor in the shape `MemoryEventStore` already uses, so a fixture and a later `append` can mint `EventId`s without a round trip. | `crates/happenstance-core/src/memory.rs:104-107` |
| **Migration is idempotent** | Re-opening an already-migrated file is a no-op: no error, no second `StoreId`, no duplicated index. `CREATE TABLE IF NOT EXISTS` throughout. | `crates/happenstance-sqlite/src/event_store.rs:120-124` (migrate runs on every connect) |
| **Migration is safe under concurrent opens** | All statements inside one `BEGIN IMMEDIATE`; the identity row is `INSERT OR IGNORE` **then read back**, so the loser of the race adopts the winner's value rather than keeping its own. N concurrent opens of one fresh path all return `Ok` and all report one identical `StoreId`. | `../_decomposition.md` architecture brief §7, *Migration must be idempotent and safe under a concurrent open* |
| **Journal mode is WAL** | Required for two connections onto one file. Whether it is set per connection or persisted in the file is ADR-0022's; whichever, it is verified by reading `PRAGMA journal_mode` back. | architecture brief §6, §11; ADR-0022 |
| **`synchronous` is the ADR's value and is never `OFF`** | CF-14 names `PRAGMA synchronous = OFF` **by name** as a wrong implementation the reopen rule rejects. The chosen value is a documented property of the adapter. | `spec/SPECIFICATION.md:7471-7490` |
| **The busy timeout is finite and non-zero** | Set on every connection this crate opens. Finite because an unbounded handler turns a livelock into a hung CI job that names no rule, and the suite has no watchdog by design; non-zero because `BEGIN IMMEDIATE` on a busy database otherwise fails instantly and reports a conformance failure that is not about the adapter's logic. | architecture brief §6; CF-33 via `crates/happenstance-testkit/src/concurrency.rs:24-43` |
| **Pragmas are read back, not assumed** | SQLite silently accepts an unknown pragma. Each of the three is asserted from the value SQLite returns on the live connection, per handle. | this story's `crates/happenstance-sqlite/tests/migration.rs` |
| **Both stores in this crate configure their connection identically** | `SqliteProjectionStore::open` adopts the same configuration; a projection connection with no busy timeout fails against the event store's write lock. Its `migrate` body stays `todo!()`. | `crates/happenstance-sqlite/src/projection_store.rs:79-100`; architecture brief §7 (last paragraph) |
| **The shape guard still holds** | `SqliteEventStore: Send + Sync`, `SqliteReadStream: Send + Unpin`, `SqliteEventStoreError: Error + Send + Sync + 'static` after the store gains fields. No `rusqlite` handle that borrows the connection may reach a field. | `crates/happenstance-sqlite/tests/shapes.rs`; `crates/happenstance-sqlite/src/event_store.rs:248-271` |
| **The feature powerset still compiles clean** | A new shared module used by both feature-gated modules must be gated on `any(feature = "event-store", feature = "projection-store")`, or `--no-default-features` builds it dead and `-D warnings` fails. The gate runs a `cargo hack` powerset. | `crates/happenstance-sqlite/Cargo.toml:28-33`; `CLAUDE.md` — *Commands* |
| **The published schema doc matches the code** | The `# Intended schema` block is rewritten to what `migrate` creates. A wrong SQL block above a real body is a documentation defect this PR would otherwise introduce. | `crates/happenstance-sqlite/src/event_store.rs:34-58`; architecture brief §7 (*the module doc is published documentation*) |
| **Nothing user-observable ships alone** | Foundation archetype: this story's consumers are named in-project and land in the same slice. No capability is claimed complete by this PR. | `../_storymap.md`, *Why the three foundations are foundations* |

## Data and migrations

This story **is** the migration. There is no existing data and no upgrade path: the
crate has never written a byte to a database, so migration 1 is a create, not an
alter.

**Migration identity.** Migration 1 is recorded in the file so that migration 2 has
something to test against. `PRAGMA user_version`, or a schema-version row beside the
identity row — the implementer's call, and ADR-0022's if it already stated one. What
is not optional is that a version is written: a schema with no version marker cannot
be migrated later without guessing, and the crate's whole premise is durable storage.

**What the file holds after migration 1.**

| Object | Purpose | The wrong shape it forbids |
| --- | --- | --- |
| `event` | The log. `position INTEGER PRIMARY KEY AUTOINCREMENT` plus type, payload, metadata, canonical tags, the origin pair and `recorded_at` | Plain `rowid`, which reuses a position after a delete and breaks `reopened_store_does_not_reissue_an_event_id` |
| `event_tag` `WITHOUT ROWID`, key `(tag, position)`, **covering `event_type`** | Tag matching, ordered by position, with the type constraint answerable without leaving the index | The crate's own sketch: `(tag, position)` with no type, which forces a join back to `event` under the `BEGIN IMMEDIATE` write lock and serialises every writer |
| `tag_cardinality` | Per-tag counts, so a multi-tag arm can be probed most-selective-tag-first | Relying on `ANALYZE`, which stores only an average and cannot answer per-value |
| `UNIQUE(origin_store, origin_position)` | `contains_event_id`'s index and ingest's duplicate guard, in one constraint | Two separate unique constraints, or none — the second lets ingest store one event twice |
| The identity row (`StoreId`, as bytes or text) | The incarnation every `EventId` this store mints is keyed on | Storing it as an integer or a pair of integers — a 128-bit value cannot survive a JS-number widening (`spec/SPECIFICATION.md:838-841`) |
| A schema-version marker | Makes migration 2 writable | No marker, and a future migration that has to guess |

**Reversibility.** None is offered and none is owed: nothing consumes this schema
yet outside this workspace, the crate is `publish = false`, and the first published
version of `happenstance-sqlite` is a different project's decision entirely
(`../project.md`, *Out of scope*).

## Acceptance criteria

Nine criteria. Each is framed from the intent of a persona named in
`.bklg/from-contract-to-published-library/initiative.md:200-224` — the **adapter
author**, who needs an executable definition of "correct" rather than a prose
specification; the **application author**, whose stated fear is a second,
independent reader building a wrong answer from a torn or gapped log; and, for the
identity criteria, the **operator** that VT-6's re-mint procedure is written for.
Every verification names a real path. All nine are gated by the same command,
`cargo xtask affected --base main` (`.redkiln/config.yaml:40`), which compiles and
runs `happenstance-sqlite`'s test targets.

| id | criterion | verification |
| --- | --- | --- |
| AC-001 | **GIVEN** an adapter author holding ADR-0022 and needing a schema whose append-condition probe does not walk a join under the write lock, **WHEN** `SqliteEventStore::open` is called on a path that has never been migrated, **THEN** the objects SQLite reports through `sqlite_master` are: `event` with `position INTEGER PRIMARY KEY AUTOINCREMENT` plus columns for type, payload, metadata, canonical tags, the origin pair and `recorded_at`; `event_tag` `WITHOUT ROWID` keyed `(tag, position)` in that order with `event_type` carried as a **non-key covering column**; `tag_cardinality`; one `UNIQUE` constraint over `(origin_store, origin_position)` **together**; and a schema-version marker — **AND** `EXPLAIN QUERY PLAN` for a tag-plus-type probe names no access to `event`. | `crates/happenstance-sqlite/tests/migration.rs::migration_creates_the_amended_schema` — reads `sqlite_master` and `pragma_table_info` back out of a real file, and asserts the query plan rather than the SQL text |
| AC-002 | **GIVEN** the adapter author whose fixture will call `connect()` more than once and whose `open` therefore runs `migrate` on every connect, **WHEN** `SqliteEventStore::open` is called a second time on an already-migrated path, **THEN** it returns `Ok`, the set of objects in `sqlite_master` is unchanged from the first open, exactly one identity row exists, and the second handle's `store_id()` equals the first handle's. | `crates/happenstance-sqlite/tests/migration.rs::second_open_of_a_migrated_file_changes_nothing` |
| AC-003 | **GIVEN** an adapter author whose CI opens two handles onto one file and cannot control which wins, **WHEN** N threads (N ≥ 2, and at least the handle count the slice's fixture will use) call `SqliteEventStore::open` on one *fresh* path simultaneously, **THEN** every call returns `Ok`, every handle reports the **same** `store_id()`, and the file afterwards holds exactly one identity row and one copy of each schema object — the loser of the insert race having adopted the winner's value rather than kept its own. | `crates/happenstance-sqlite/tests/migration.rs::concurrent_opens_of_a_fresh_file_agree_on_one_store_id` |
| AC-004 | **GIVEN** the application author whose fear is a second, independent reader building a wrong answer from a log whose identities moved under it, **WHEN** a store is opened on a fresh path, its `store_id()` recorded, the handle dropped, and the same path re-opened into a new `SqliteEventStore`, **THEN** `store_id()` is byte-identical to the recorded value, `StoreId::to_bytes` round-trips it, and the persisted form is bytes or text — never an integer and never a pair of integers. | `crates/happenstance-sqlite/tests/migration.rs::store_id_survives_a_close_and_reopen`; the storage-form half asserted from `pragma_table_info`'s declared type and the value's SQLite type code |
| AC-005 | **GIVEN** an operator who restored this file from Friday's backup — a state a SQLite adapter cannot detect, which is why VT-6 grants mint-once only against a documented procedure — **WHEN** they invoke the adapter's public re-mint operation and re-open, **THEN** the persisted identity has been replaced by a fresh 128-bit value different from the old one, the re-opened handle reports the new one, the operation's rustdoc states in prose **when a deployment invokes it** (after a restore or a clone) with an `# Errors` section naming conditions rather than types, and `references/adapter-shapes.md` records mint-once-with-explicit-re-mint as this adapter's chosen VT-6 mechanism. | `crates/happenstance-sqlite/tests/migration.rs::remint_replaces_the_persisted_identity`; the record half by `cargo xtask spec-trace` still resolving every `references/adapter-shapes.md` citation, and by the rustdoc built in `cargo xtask ci --fast` |
| AC-006 | **GIVEN** `recorded_time_survives_a_reopen`, which has had nothing in the workspace able to fail it since phase 4, **WHEN** a row carrying a known `recorded_at` value is present and the store is closed and re-opened, **THEN** the column reads back **exactly** the stored value, and neither `open` nor `migrate` writes to `recorded_at` on any existing row — a re-stamp on open is the defect this criterion exists to reject. | `crates/happenstance-sqlite/tests/migration.rs::recorded_at_is_returned_as_stored_after_a_reopen` |
| AC-007 | **GIVEN** an adapter author who has been told the journal mode and busy timeout are set, and knows SQLite accepts an unknown pragma silently, **WHEN** either `SqliteEventStore::open` or `SqliteProjectionStore::open` returns, **THEN** reading the pragmas back on that live connection returns `wal` for `journal_mode`, ADR-0022's stated value for `synchronous` and **never** `OFF`/`0`, and a finite, non-zero millisecond value for `busy_timeout` — per handle, for both stores. | `crates/happenstance-sqlite/tests/migration.rs::pragmas_are_in_effect_on_every_connection` — values read from `PRAGMA journal_mode` / `synchronous` / `busy_timeout`, never from the constants that set them |
| AC-008 | **GIVEN** a reader of this crate's published documentation, for whom the `# Intended schema` block is the schema, **WHEN** they compare that block against the database `migrate` creates, **THEN** every table and index the block names exists in `sqlite_master` with the same key order and the same covering columns, and the `contains_event_id` comment no longer states that the origin columns are absent. | `crates/happenstance-sqlite/tests/migration.rs::module_doc_schema_matches_sqlite_master` — the doc block pulled in by `include_str!` on `src/event_store.rs` and matched object-by-object against `sqlite_master`; plus the `docs` step of `cargo xtask ci --fast` |
| AC-009 | **GIVEN** the workspace's standing guards — the shape target that catches a field added carelessly, and a feature powerset that builds each feature alone — **WHEN** `SqliteEventStore` has gained its persisted-identity field and both stores share one connection-configuration path, **THEN** `tests/shapes.rs` passes unchanged in intent, `--no-default-features` and each single-feature build compile clean under `-D warnings`, `publish = false` and `xtask/src/package.rs`'s `PUBLISHABLE` are unchanged, and no `todo!()` outside `migrate` has been removed. | `cargo test -p happenstance-sqlite --test shapes`; `cargo xtask affected --base main`; `cargo hack --feature-powerset check -p happenstance-sqlite` (run by `cargo xtask ci`) |

**Coverage of the traced project ACs.** Project **AC-001** (*the suite runs, whole*)
is served by AC-001, AC-002, AC-003, AC-007 and AC-009 — the schema, the idempotence
and the connection settings are what make any rule runnable at all against a file.
Project **AC-004** (*an acknowledged write survives a reopen*) is served by AC-004,
AC-005 and AC-006 — the persisted identity, the licence to persist it, and the
`recorded_at` that is read back rather than re-stamped. AC-008 serves both, because
a wrong published schema is how the next story implements the wrong table.

## Interaction quality

Two families, and this story sits in a medium where only one of them has content.

**COMPOSITION invariants — not applicable, by a signed-off determination, not by
omission.** `../_design.md` records **N/A — no user-facing surface** for this entire
project and was approved by the repository owner on 2026-08-12 at the `/redkiln:plan`
design sign-off gate; `design.capture` is deliberately absent from
`.redkiln/config.yaml`, which makes the perceptual review a declared *skip* rather
than a silent pass (`CLAUDE.md`, *Where the work lives*). There is no chrome, no
density budget and no transience policy to honour, so no composition AC row is
owed. What the sign-off *did* bind is stated in the Scope lock: the no-surface
determination is not a licence to skip documenting the public Rust items this story
adds. That obligation is the library medium's analogue of *presentation exists at
all* — a `pub fn` with no rustdoc is the unstyled render — and it is carried as
table rows, not as prose here:

- *Presentation exists at all* (every public item this story adds carries real
  rustdoc, with `# Errors` naming conditions rather than types) — **AC-005** for the
  re-mint operation, **AC-008** for the module-level schema block. Verified by the
  `docs` step inside `cargo xtask ci --fast` and by the include-and-compare test.
  The bar is `standards/rust/70-rustdoc-obligations.md`.

**STATE invariants — three apply, translated to this medium, and each is already an
AC row.** The translation is not a licence to weaken them; it is what "preserved
selection" and "reversibility" mean when the state in question is a database file
rather than a viewport.

- *In-place, not a context jump* — re-opening an already-migrated file must change
  nothing a caller could observe: no new schema objects, no second identity, no
  error. The database is the state that must survive being re-entered. Carried by
  **AC-002**, verified by `migration.rs::second_open_of_a_migrated_file_changes_nothing`.
- *Preserved identity across a re-entry* (the direct analogue of preserved
  focus/selection) — the `StoreId` and every `recorded_at` are the same values after
  a close-and-reopen that they were before it. Carried by **AC-004** and **AC-006**,
  verified by `::store_id_survives_a_close_and_reopen` and
  `::recorded_at_is_returned_as_stored_after_a_reopen`.
- *Reversibility* — the one irreversible act in this story is minting an identity,
  and VT-6 requires that a deployment be able to undo the consequence of a restore.
  The explicit re-mint operation **is** the undo, and it is what buys the right to
  mint once. Carried by **AC-005**, verified by `::remint_replaces_the_persisted_identity`.
- *Non-occlusion*, *keyboard reachability* — no rendered surface, no viewport, no
  focus ring. Nothing to assert and nothing quietly skipped.

**The named anti-pattern this story must not commit** is the one the design sign-off
recorded in spirit and the architecture brief records by name: shipping the crate's
own known-wrong `# Intended schema` sketch as if it were the schema. It passes every
conformance rule and serialises every writer. **AC-001** rejects it by query plan and
**AC-008** rejects it by comparison.

## Error conditions

| id | condition | required behaviour |
| --- | --- | --- |
| EC-001 | The path cannot be opened or created — a missing directory, a permission refusal, a corrupt header | `SqliteEventStoreError::Sqlite` propagated out of `open`, never a panic and never a partially-migrated file left behind. `open`'s existing `# Errors` section already promises exactly this (`crates/happenstance-sqlite/src/event_store.rs:114-124`) and must stay true |
| EC-002 | Two opens race and this one loses the identity insert | **Not an error.** `INSERT OR IGNORE` followed by a read-back; the loser adopts the winner's `StoreId`. An implementation that returns `Err` here, or that keeps the value it generated, fails AC-003 |
| EC-003 | `SQLITE_BUSY` while `migrate`'s `BEGIN IMMEDIATE` waits for the write lock | Absorbed by the configured busy timeout. It surfaces as `SqliteEventStoreError::Sqlite` **only** after that finite timeout expires — an unbounded handler is forbidden (no watchdog exists anywhere in the suite, `crates/happenstance-testkit/src/concurrency.rs:24-43`), and a zero timeout turns ordinary contention into a conformance failure that is not about this adapter's logic |
| EC-004 | The file is migrated but the identity row is missing, empty, or not sixteen bytes | A **distinct, named** error variant. Silently minting a replacement is the worst available behaviour: it re-issues `EventId`s under a new origin for events that already exist, which is the failure mode VT-6 describes as having *no error path and no observable symptom* (`spec/SPECIFICATION.md:806-810`) |
| EC-005 | The file carries a schema-version marker newer than this build knows | Refuse to open with a named error that reports both versions. Operating on an unknown schema is how a later migration loses data, and the marker exists precisely so migration 2 has something to test against |
| EC-006 | The entropy source refuses while minting | A named error out of `open`; the transaction rolls back and the file is left un-migrated rather than migrated-without-identity. A zero or constant `StoreId` is never a fallback |

## Non-functional

| id | requirement | why, and how it is held |
| --- | --- | --- |
| NF-001 | A tag-plus-type probe is answerable from `event_tag` alone | This is the entire reason the schema is amended. Without the covering `event_type`, SQLite's `MERGE (UNION)` streaming plan collapses into a co-routine over a temp b-tree and the probe walks the posting list through a join *while holding the `BEGIN IMMEDIATE` write lock* — serialising every writer in a store whose whole point is that they need not be. Asserted, not assumed, by the `EXPLAIN QUERY PLAN` half of AC-001 |
| NF-002 | No new dependency, and no `[workspace.dependencies]` change | `cargo deny` runs in the gate and a manifest change is a workspace-wide claim (architecture brief §11). Sixteen bytes of entropy are obtainable from the driver already in the tree — `SELECT randomblob(16)` — which is a real option to weigh against adding a crate, and either way the choice is stated in the implementation, not smuggled into `Cargo.toml` |
| NF-003 | `migrate` stays cheap enough to run on every connect | It runs on **every** `open` (`crates/happenstance-sqlite/src/event_store.rs:120-124`), including each of the fixture's handles and each contender's connection later in the slice. A bounded number of `IF NOT EXISTS` statements and one identity read-back; no `ANALYZE`, no table scan, no schema introspection loop at open |
| NF-004 | Nothing this story adds may hang unboundedly | CF-33's no-watchdog discipline means a hang is a silent CI failure that names no rule. The busy timeout is finite; the concurrent-open test joins every thread and asserts, and adds no timeout of its own — if it hangs, that is a finding about the timeout value, not something to paper over (testing brief §4) |
| NF-005 | The crate still builds at the MSRV, under every feature combination, with `-D warnings` | MSRV is 1.97.1 (`.kb/decisions/0029-msrv-raised-to-1-97-1.md`). A connection-configuration module shared by both roles must be gated `any(feature = "event-store", feature = "projection-store")` or `--no-default-features` compiles it dead and the powerset fails. Carried by AC-009 |
| NF-006 | The test target runs without a live network, a fixed port, or a shared temp path | One process-local ordinal per test file plus `Drop` cleanup, in the shape `crates/happenstance-testkit/tests/fixture_instruments.rs:95-102` already uses. Two tests in this file must never collide on a path, or AC-003 becomes flaky in exactly the way that gets a test deleted |

## Implementation notes (non-prescriptive)

- **Read ADR-0022 first and take six values from it verbatim**: the append-condition
  strategy, the tag storage, the busy-timeout number, the `synchronous` setting, the
  journal mode, and whether WAL is set per connection or persisted in the file. If it
  has not landed, stop — this story cannot invent them without becoming the
  "decision deferred to a measurement that never happens" the runbook names.
- **`WITHOUT ROWID` and the covering column are easy to conflate.** The key stays
  `PRIMARY KEY (tag, position)`; `event_type` is an *additional non-key column on the
  same table*. Writing `PRIMARY KEY (tag, position, event_type)` compiles, passes
  every conformance rule, and destroys the property the amendment exists for — the
  range must stay sorted by position.
- **The identity row wants the same table as the schema-version marker** if a
  key/value table is chosen; `PRAGMA user_version` is an equally legitimate home for
  the version and cannot hold the identity (it is a 32-bit integer, and VT-6 forbids
  integer storage for a `StoreId` regardless).
- **Shape of `migrate`**: one `BEGIN IMMEDIATE`; `CREATE TABLE ... IF NOT EXISTS` and
  `CREATE INDEX ... IF NOT EXISTS` throughout; `INSERT OR IGNORE` the identity;
  `SELECT` it back **inside the same transaction**; commit; then hand the value to
  the constructor. The read-back is the load-bearing half — see EC-002.
- **`new(connection)` needs a decision, not an oversight.** It currently promises the
  caller applied the schema (`event_store.rs:102-112`). Once the store carries an
  identity field, `new` either reads the identity back itself or stops being
  infallible. Whichever, its rustdoc has to stop being ambiguous about it.
- **Connection configuration is one function used by both roles**, feature-gated per
  NF-005; `SqliteProjectionStore::open` adopts it as a one-line change and keeps its
  own `migrate` as `todo!()`.
- **Look at `MemoryEventStore` before inventing an identity accessor**
  (`crates/happenstance-core/src/memory.rs:97-107, 163-166`): `with_store_id`,
  `store_id()` and `restore` are the shape the workspace already reads as normal, and
  a second unlike spelling costs the next adapter author a translation.
- **`EXPLAIN QUERY PLAN` returns rows of text**; assert on the *absence* of the
  `event` table in the plan rather than pattern-matching SQLite's phrasing, which
  changes between versions and would make the test brittle for no gain.
- **Append to `references/adapter-shapes.md` at the end of the file** (it is 365
  lines), never above line 297 — `spec/SPECIFICATION.md` cites `:97-102`, `:220` and
  `:297`, and an insertion above them silently re-points three citations that
  `cargo xtask spec-trace` would still report as resolving.

## Tests and CI (merge gate)

Tiers are the testing brief's (`../_decomposition.md`, testing brief §1 and §6); the
commands are the repository's own, wired from `.redkiln/config.yaml` rather than
typed fresh. There is no doubling anywhere below — testing brief AC-T03 forbids it,
and a doubled driver is the "compiles but never ran" failure this project exists to
retire.

| Tier | Command / path | Proves |
| --- | --- | --- |
| Static | `cargo xtask lints && cargo xtask spec-trace` (`reachability_static`, `.redkiln/config.yaml:48`) | Every `references/adapter-shapes.md` citation in `spec/SPECIFICATION.md` still resolves after the VT-6 record is appended, and the citation count has not fallen — AC-005 |
| Static / lint | `cargo fmt --check` and `cargo clippy -D warnings` inside `cargo xtask affected --base main` | The new module and test target are clean, and the feature gate on the shared connection-configuration module is correct — AC-009 |
| Unit / type-level | `cargo test -p happenstance-sqlite --test shapes` — `crates/happenstance-sqlite/tests/shapes.rs` | `SqliteEventStore: Send + Sync`, `SqliteReadStream: Send + Unpin` and the error bounds still hold once the store carries a persisted-identity field. This is the guard that catches a field added carelessly, *before* any SQL runs — AC-009 |
| Integration (real SQLite, this story's proof) | `cargo test -p happenstance-sqlite --test migration` — `crates/happenstance-sqlite/tests/migration.rs` | All of AC-001 – AC-008 against a real file through a real `rusqlite::Connection`: the schema read back from `sqlite_master`, the query plan, the second open, N concurrent opens, the close-and-reopen, the re-mint, the three pragmas, and the module doc compared object-by-object |
| Story gate | `cargo xtask affected --base main` (`affected_gate`, `.redkiln/config.yaml:40`) | The whole diff: `happenstance-sqlite` and its dependents compile, `fmt`/`clippy -D warnings` pass, and both test targets run. This is the story-grain merge bar |
| Project integration grain | `cargo xtask ci --fast` (`integration_scoped`, `.redkiln/config.yaml:55`) | Adds the `docs` build (AC-008's rustdoc half), the four `wasm32` steps and `spec-trace`. Run before the slice closes; it is this non-terminal project's terminal command |
| Feature powerset | `cargo hack --feature-powerset check -p happenstance-sqlite` (inside `cargo xtask ci`) | `--no-default-features` and each single-feature build compile clean — the shared module is gated on `any(...)` and not built dead — AC-009 |
| Conformance (**not yet runnable, deliberately**) | `crates/happenstance-sqlite/tests/conformance.rs` | Nothing here yet. `event_store_conformance!` drives `append` and `read`, both still `todo!()`; the slice-mates supply them. This row exists so that "no conformance run in this story" reads as a stated sequence rather than an omission |

**What is *not* added**: no timeout, watchdog or retry wrapper around any test
(CF-33, testing brief §4); no mutation-registry row — the negative control for
`recorded_time_survives_a_reopen` is
`reopen-negative-control-and-durability-verdicts`'s permanent registry row, and this
story supplies only the architectural precondition it needs.

## Risks and coupling (PR-scoped)

- **ADR-0022 has not landed.** Six values arrive from it and none may be invented
  here. *Mitigation*: the story is `blocked_by: HS-S0035` in its own frontmatter;
  if implementation starts anyway, stop and say so rather than choosing numbers.
- **The covering column is implemented as a key column.** Compiles, passes every
  rule, and silently destroys the position-sorted range that the whole amendment
  exists to preserve. *Mitigation*: AC-001 asserts key order from
  `pragma_table_info`/`sqlite_master`, and NF-001 asserts the query plan — not the
  DDL text, which is where this defect hides.
- **The identity read-back is skipped** because `INSERT OR IGNORE` "obviously"
  succeeded. Two opens of a fresh file then hold two different `StoreId`s and every
  reopen rule downstream becomes non-deterministic in a way that looks like
  flakiness. *Mitigation*: AC-003 is the test written specifically to fail against it.
- **The concurrent-open test is flaky on a single-core CI runner.** N threads that
  never actually overlap prove nothing; N threads with no barrier prove less.
  *Mitigation*: gate the threads on a rendezvous before calling `open`, join all of
  them, and assert on the *set* of `StoreId`s having one element — a set property
  that holds at any N above one, in the same spirit as `CONTENDERS`' own doc.
- **`SqliteEventStore` gains a field and `tests/shapes.rs` breaks.** `StoreId` is
  plain bytes and costs nothing, but an implementer reaching for a cached
  `rusqlite::Statement` or a `Transaction<'_>` in the same change breaks `Send`,
  `Sync` and ADR-0008 at once. *Mitigation*: AC-009; and
  `crates/happenstance-sqlite/src/event_store.rs:248-271`'s doc comment already says
  why no `Statement`, `Rows` or `Transaction` may appear in a field.
- **Appending to `references/adapter-shapes.md` above line 297** re-points three
  `SPECIFICATION.md` citations to text that is not what they meant, and
  `spec-trace` reports them as *resolving*. It is a silent documentation defect.
  *Mitigation*: append at the end of the file; the PR boundary says so explicitly.
- **Coupling forward, and it is real**: `append-atomicity-and-store-limits` writes
  the origin pair and `recorded_at` into the columns this story names, and
  `lazy-read-with-snapshot-ceiling` reads them back. A column rename after either
  lands is a migration 2, not an edit. Name the columns once, here, deliberately.
- **Coupling sideways**: `SqliteProjectionStore::open` adopts this story's connection
  configuration. If it does not, a projection connection with no busy timeout fails
  immediately against the event store's `BEGIN IMMEDIATE`, and the symptom appears in
  `projection-store-passes-the-borrowed-suite`, three stories away from its cause.
- **Scope creep into the `# Intended schema` rewrite.** Correcting the block is in
  scope (AC-008); rewriting `lib.rs`'s status banner or deleting
  `#![allow(clippy::todo)]` is not, and DR-01 says the allow dies with the *last*
  `todo!()` — which is in the projection store.

## Dependencies

**Blocks on**

- `adr-0022-append-condition-strategy` (HS-S0035) — hard, not a sequencing
  convenience. It supplies the append-condition strategy, the tag storage, the
  amended schema's ratification, the busy timeout, the `synchronous` setting and the
  journal-mode decision. AC-013 and DR-06 put the record **before** the
  implementation (`../_storymap.md`, *Why the three foundations are foundations*).

No other story in this project, and no project outside it, blocks this one. The
project-level edges (`projection-store-freeze`, `typed-layer-and-alpha-release`) are
recorded in `../project.md` *Dependencies* and are not story dependencies here.

**Unlocks** (matching this story's `blocks:` in its item frontmatter)

- `append-atomicity-and-store-limits` (HS-S0037) — writes into `event`, `event_tag`
  and the origin pair inside the `BEGIN IMMEDIATE` this schema is shaped for.
- `lazy-read-with-snapshot-ceiling` (HS-S0038) — pages `event` bounded by a position
  ceiling; needs `AUTOINCREMENT` positions and the tag index to exist.
- `projection-store-passes-the-borrowed-suite` (HS-S0044) — adopts this story's
  connection configuration on its own second connection.

Downstream but not directly blocked: `sqlite-fixture-and-whole-suite` (the fixture
that declares `REOPEN` and `SECOND_HANDLE` against this file layout) and
`reopen-negative-control-and-durability-verdicts` (whose negative control is only
writable because `recorded_at` is read back rather than re-stamped).

## Anchors (progressive disclosure)

Everything above is sufficient to start. These are the deeper artifacts — open each
at the moment named, and read the cited range rather than the file.

| Anchor | Why it is load-bearing | When to open | Serves |
| --- | --- | --- | --- |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/adr-0022-append-condition-strategy/spec.md` | The predecessor story's own spec, and the map to the six values this story consumes rather than chooses. The ADR atom it produces (under `.kb/decisions/`) is the authority once it exists; this spec is how you find it | **First, before writing a line of SQL.** If the ADR atom is absent, stop | AC-001, AC-007 |
| `spec/SPECIFICATION.md` (VT-6, `:783-841`) | The clause that both permits mint-once and prices it: the two MUSTs (a documented re-mint or detection; the mechanism recorded in `references/adapter-shapes.md`) and the bytes-or-text storage rule are all here, and the "no error path, no observable symptom" paragraph is why EC-004 refuses to re-mint silently | Before designing the identity row, and again before writing the re-mint's rustdoc | AC-004, AC-005 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_decomposition.md` (architecture brief §7, §6, §11) | The full statement of what migration 1 must hold, why the crate's own sketch serialises writers, the `SQLITE_BUSY`-is-architecture argument, and what §11 deliberately leaves to you (the timeout value, the `synchronous` setting, the temp-file ownership) | Before implementing AC-001; re-read §6 before AC-007 | AC-001, AC-007 |
| `crates/happenstance-sqlite/src/event_store.rs` (`:34-58`, `:102-142`, `:237-244`, `:248-271`) | The wrong schema sketch you are replacing, the four constructors you rewire, the `contains_event_id` comment that says the origin columns are absent, and the state-machine doc comment explaining why no `Statement` or `Transaction` may become a field | Open at the start; keep `:248-271` in view whenever adding a field | AC-001, AC-002, AC-008, AC-009 |
| `crates/happenstance-core/src/identity.rs` (`:44-124`) | `StoreId::from_bytes`/`to_bytes` are the only doors in and out; `EventId::new` fixes the origin pair's shape. The contract crate mints nothing — this adapter does, and the reason is written here | Before minting or persisting anything | AC-004 |
| `crates/happenstance-core/src/memory.rs` (`:97-107`, `:163-166`) | The reference shape for a store that carries a `StoreId` and exposes it — `with_store_id`, `store_id()`, `restore`. Read it before inventing a second, unlike spelling | Before adding the accessor and the re-mint operation | AC-004, AC-005 |
| `crates/happenstance-testkit/tests/fixture_instruments.rs` (`:75-102`) | The workspace's existing mint-once precedent, stated with its reason, plus the process-local-ordinal pattern for a temporary resource that needs no new dependency | Before AC-003 and AC-004; again when the test file needs its temp paths | AC-003, AC-004, AC-006 |
| `crates/happenstance-testkit/src/suite.rs` (`:265`, `:332`, `:2310`, `:2473`, `:2551`) | The five rules this story makes *askable*. Read what they actually assert, so the schema serves them rather than something adjacent — particularly `recorded_time_survives_a_reopen` at `:2473` | Before AC-004 and AC-006, to check the column shape against the assertion | AC-004, AC-006 |
| `references/adapter-shapes.md` | Where VT-6's second MUST lands. Also the file whose lines `:97-102`, `:220` and `:297` are cited from `SPECIFICATION.md` — which is why the record is appended at the end | When writing the VT-6 record, at the end of the change | AC-005 |
| `.kb/decisions/0014-event-identity-and-recorded-time.md` | The Accepted atom behind `recorded_at`: what the stamp means, who owns it, and why it is stored rather than derived | Before implementing AC-006 | AC-006 |
| `.kb/decisions/0013-position-assignment-and-visibility.md` | The global visibility invariant, whose concrete corollary here is that no materialised head row may enter this schema — one file backs several handles | While designing the tables, before AC-001 is written | AC-001 |
| `crates/happenstance-testkit/src/concurrency.rs` (`:24-43`, `:206`) | CF-33's no-watchdog discipline stated in full, and `CONTENDERS` — the number of connections that will eventually sit on one file, which is what makes the busy timeout an architecture decision rather than a knob | Before choosing the busy-timeout assertion in AC-007, and before writing AC-003's thread count | AC-003, AC-007 |
| `crates/happenstance-sqlite/tests/shapes.rs` | The type-level guard, and the comments explaining exactly which trait each assertion protects and why | Immediately after adding the identity field | AC-009 |
| `standards/rust/70-rustdoc-obligations.md` | The house bar for the public items this story adds — `# Errors` naming conditions rather than types, and what a doc comment owes a reader | Before writing the re-mint operation's and accessor's rustdoc | AC-005, AC-008 |
| `.bklg/from-contract-to-published-library/sqlite-durable-store/_design.md` | The signed-off no-surface determination, and the one obligation it leaves standing: the public Rust items are still owed real documentation | Once, when reading the Interaction quality section above | AC-005, AC-008 |
| `RUNBOOK.md` (`:4166-4237`, especially `:4178-4187`) | Phase 8 in full, and the two schema amendments in the words that first stated them | For orientation before AC-001; not needed again | AC-001 |

## Clarifications resolved during spec

1. **The AC set is exactly the nine the front half decided** — AC-001 through
   AC-009. None added, none dropped; the ledger carries the same nine ids.
2. **VT-6's two MUSTs became one AC rather than a note.** The front half established
   that mint-once is legitimate here *only* with an explicit re-mint operation and a
   recorded mechanism. Left as prose, both are the kind of obligation that ships as
   "we'll add it in the next PR". AC-005 gates them together, because they are one
   permission, not two chores.
3. **`recorded_at` gets an AC in a story that never writes one.** Appending is
   `append-atomicity-and-store-limits`'s. What is testable *here* is the half that
   matters for the reopen rules: a value present in the column is returned as stored,
   and neither `open` nor `migrate` touches it. AC-006 asserts that against a row
   inserted directly by the test, which is legitimate because the row is real SQL on
   a real file — not a double (testing brief §3).
4. **The module-doc correction is gated by a test, not by review.** The architecture
   brief calls the module doc *published documentation* and the story map gives the
   `lib.rs` banner rewrite to a later story. A reviewer comparing SQL prose to SQL
   code is exactly the check that passes by fatigue, so AC-008 compares the block to
   `sqlite_master` mechanically via `include_str!`.
5. **The `EXPLAIN QUERY PLAN` assertion was folded into AC-001 rather than left as
   NF-001 alone.** The covering column is this story's central architectural claim
   and an NF row gets no ledger entry — the plan check therefore lives inside a gated
   criterion, with NF-001 stating the cost it prevents.
6. **The Interaction quality section carries no bullet-only invariant.** Every
   applicable invariant is an AC row (AC-002, AC-004, AC-005, AC-006, AC-008);
   `redkiln verify` extracts ACs from the table, so a prose bullet would have been
   ungated and untested. The composition family is N/A against a signed-off design,
   stated rather than skipped.
7. **`new(connection)`'s contract is flagged, not decided.** Once the store carries
   an identity field, the constructor that promises "the caller applied the schema"
   is either fallible or reads the identity itself. It is recorded in the
   implementation notes as a decision the implementer must make explicitly, because
   silently leaving a `StoreId`-less store constructible is how a later `append`
   mints an `EventId` under a zero origin.
8. **The concurrent-open thread count is a set property, not a number.** AC-003 says
   N ≥ 2 and at least the fixture's handle count, and asserts the *set* of observed
   `StoreId`s has one element. Pinning a literal here would collide with
   `concurrency-family-and-contender-count`'s live 8-versus-64 question, which is not
   this story's to settle.
