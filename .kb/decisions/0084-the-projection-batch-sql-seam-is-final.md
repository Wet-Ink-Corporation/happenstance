---
id: kb-decision-0084
title: The projection batch's SQL seam is &'static str plus a named escape hatch at 1.0, and the parameter count is stated once
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0084
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  Answers kb-open-question-projection-batch-sql-statement-type-001 with Option A, made final for
  1.0. A SQL projection batch has eight published entry points across three crates. SqliteBatch::push,
  PostgresProjectionBatch::push and LivePostgresBatch::execute keep sql: &'static str with the values
  bound beside it. push_raw_sql (twice) and execute_raw_sql stay the named escape hatches for a
  statement whose shape is computed. SQLite's PendingStatement::sql() keeps returning &str over a
  private Box<str>. The phase-16 classification and the open question both missed
  LivePostgresBatch::execute and execute_raw_sql; this record covers them. A minted Statement type
  made by a sql! macro (Option B) is declined. It would have to be per adapter, because ?1 and $1
  are different dialects. PS-9, which is frozen, leaves it no generic consumer. It is exported macro
  surface. Its one extra guarantee, a parameter-count check, cannot be made at compile time because
  params has a run-time length, and a run-time check needs no new type. The arity obligation is
  stated once here. The number of values bound to a statement must equal the number of placeholders
  its text declares. No batch counts them at push. rusqlite refuses a mismatch in either direction
  at commit, inside BEGIN IMMEDIATE. The Postgres server refuses too few at commit (buffered) or at
  execute (live). Too many is not enforced on Postgres today. The adapter's bind_all binds every
  value with no count check. sqlx declares one type per bound value at Parse, so a surplus on a
  connection that has not prepared that text before is an unused parameter, and it commits. sqlx
  caches the prepared statement by its text alone, so the same surplus is refused on a connection
  that already holds the text, and a surplus prepared first makes later correct calls of that text
  on that connection fail. The record therefore proposes an adapter-side count check in
  happenstance-postgres: the highest $n is found by a scan of the text, it is compared with the
  values before anything is sent, and a mismatch either way is refused. The live batch is left
  unable to commit. On Neon the server-side behaviour is unmeasured. That Neon narrows in 0.4.0 is the owner's default in
  force (runbook/handover.md:57-61, listed under "Waiting on the owner"); the shape is this
  record's proposal: NeonWriteBatch::push takes (&'static str, Vec<serde_json::Value>),
  push_raw_sql takes a SqlStatement, and statements becomes private behind a statements() accessor. Without the private
  field the narrowing would be decorative. The escape hatches stay inconsistent with each other on
  purpose. Additive commit-arity rollback tests are named, with their wrong implementations; the
  Postgres surplus leg lands with the count check. A
  minted type can still arrive later as an additive push_statement method; only replacing push is
  ruled out. Supersedes nothing.
depends_on:
  - kb-decision-0017
  - kb-decision-0063
  - kb-decision-0066
related:
  - kb-decision-0003
  - kb-decision-0017
  - kb-decision-0036
  - kb-decision-0062
  - kb-decision-0063
  - kb-decision-0066
  - kb-decision-0072
  - kb-decision-0074
  - kb-open-question-projection-batch-sql-statement-type-001
source_paths:
  - crates/happenstance-sqlite/src/projection_store.rs
  - crates/happenstance-sqlite/tests/projection.rs
  - crates/happenstance-postgres/src/projection_store.rs
  - crates/happenstance-postgres/src/live_projection_store.rs
  - crates/happenstance-postgres/tests/projection.rs
  - crates/happenstance-neon/src/projection_store.rs
  - crates/happenstance-neon/src/transport.rs
  - examples/rebuilding-read-models/src/main.rs
  - examples/tickets-over-http/src/lib.rs
  - examples/transfers-on-sqlite/src/main.rs
  - spec/SPECIFICATION.md
  - references/adr/0084-the-projection-batch-sql-seam-is-final.md
last_reviewed: 2026-10-07
---

# The projection batch's SQL seam is &'static str plus a named escape hatch at 1.0, and the parameter count is stated once

The full record has the per-adapter driver evidence, the Rust reasoning for each type and the
call-site inventory. It is
[`references/adr/0084-the-projection-batch-sql-seam-is-final.md`](../../references/adr/0084-the-projection-batch-sql-seam-is-final.md).

## The question

X-4 of the pre-publication review narrowed `SqliteBatch::push` to `&'static str` and added
`push_raw_sql` as the escape hatch (Option A). The open question asked whether that is the final
shape, or whether a minted `Statement` type should follow (Option B). It left the answer to
whoever froze `ProjectionStore`. ADR-0063 froze the port, and `0.3.x` published the seam. Phase 16
therefore classified the question as *breaking-if-answered*: Option B changes a published
parameter type, so only `0.4.0` can absorb it
(`.kb/open-questions/projection-batch-sql-seam-statement-type.md:146-156`).

The published seam has eight entry points, not the three that phase 16 named:

| Entry point | Signature today | Where |
|---|---|---|
| `SqliteBatch::push` | `(sql: &'static str, params: impl IntoIterator<Item = Value>)` | `crates/happenstance-sqlite/src/projection_store.rs:487-489` |
| `SqliteBatch::push_raw_sql` | `(sql: impl Into<String>, params: impl IntoIterator<Item = Value>)` | `:513-522` |
| `PendingStatement::sql` | `-> &str`, over a private `Box<str>` | `:383-393` |
| `PostgresProjectionBatch::push` | `(sql: &'static str, params: impl IntoIterator<Item = PgParam>)` | `crates/happenstance-postgres/src/projection_store.rs:252-257` |
| `PostgresProjectionBatch::push_raw_sql` | `(sql: impl Into<Box<str>>, params: Vec<PgParam>)` | `:264-269` |
| `LivePostgresBatch::execute` | `async (sql: &'static str, params: impl IntoIterator<Item = PgParam>)` | `crates/happenstance-postgres/src/live_projection_store.rs:114-121` |
| `LivePostgresBatch::execute_raw_sql` | `async (sql: &str, params: Vec<PgParam>)` | `:133-142` |
| `NeonWriteBatch::push` | `(statement: SqlStatement)`, beside `pub statements: Vec<SqlStatement>` | `crates/happenstance-neon/src/projection_store.rs:136-138`, `:181-184` |

## Decision

1. **Option A is the 1.0 seam for SQLite and Postgres (proposed).** All six SQLite and Postgres
   entry points above keep their current signatures through 1.x. That includes the live pair, which
   both the open question and phase 16 missed. A literal, a `const` or a `concat!` goes through
   `push` or `execute`. A statement whose shape is computed at run time goes through the named
   escape hatch. In every case the values are bound and never interpolated. `PendingStatement::sql`
   keeps returning `&str`, and its private `Box<str>` field can change later without a break.
2. **Option B is declined.**
   - **It has no shared form.** A `Statement` type would have to be minted separately by each
     adapter, because SQLite's `?1` and Postgres's `$1` are different dialects.
   - **It has no generic consumer.** PS-9 is `[FROZEN]`: "A projection writes through the concrete
     adapter's inherent API" (`spec/SPECIFICATION.md:5590-5591`). ADR-0017 refused a universal
     write vocabulary (`.kb/decisions/0017-what-a-projection-batch-owns.md:82-88`).
   - **It brings exported macro surface.** `standards/rust/41-declarative-macros.md` would apply in
     full.
   - **Its one extra guarantee needs no new type.** A parameter-count check cannot run at compile
     time, because `params` is an iterator or a `Vec` whose length is known only at run time. At run
     time the driver already makes the check for SQLite, and for too few values on Postgres. Item
     3's adapter-side check covers the rest at the same point.
   - **It is not foreclosed.** Option B is declined only as a *replacement* for `push`. A minted
     type can still arrive later as an additional inherent method, such as `push_statement`, in a
     minor release.
3. **The arity obligation, stated once.** The number of values bound to a statement MUST equal the
   number of placeholders its text declares. No batch counts placeholders at `push`. A mismatch is
   refused when the statement runs, as `CommitError::Store` (or the live `execute` error). Today
   that holds for SQLite in both directions and for Postgres only for too few:

   | Adapter | Where the check runs | Too few values | Too many values |
   |---|---|---|---|
   | SQLite | At commit, inside `BEGIN IMMEDIATE` | Refused (`InvalidParameterCount`) | Refused (`InvalidParameterCount`) |
   | Postgres, buffered | At commit, inside the transaction | Refused by the server's `Bind` | **Not enforced.** Accepted and committed on a connection that has not prepared the text; refused if its statement cache holds the text |
   | Postgres, live | At `execute`, which also poisons the transaction | Refused by the server's `Bind` | **Not enforced**, as buffered |
   | Neon | At commit, server-side | Unmeasured | Unmeasured |

   - **SQLite:** rusqlite 0.40.1's `bind_parameters` refuses both directions (`statement.rs:474-493`).
   - **Postgres:** an extra value is accepted on first preparation because `sqlx` declares one
     parameter type per bound value at `Parse` (sqlx-postgres 0.8.6, `executor.rs:23-53`,
     `:228-229`), so the surplus becomes an unused parameter. But sqlx-postgres 0.8.6 caches a prepared statement per connection keyed by its SQL text alone (`executor.rs:177-179`; `query()` is persistent and the cache holds 100 by default). On a
     connection that already prepared the same text with a different count, the `Bind` count no
     longer matches and the server refuses it; and a surplus prepared first leaves a cached
     statement that makes later *correct* calls of the same text on that connection fail. The
     too-many cell is therefore connection-state-dependent, not "accepted". Nothing on the
     adapter's side counts: `bind_all` binds every supplied value
     (`crates/happenstance-postgres/src/projection_store.rs:322-337`), and `push`, `push_raw_sql`
     and `execute_raw_sql` pass the values through as given.
   - **Proposed: `happenstance-postgres` counts before it binds.** `replay` and `execute_raw_sql`
     compare `params.len()` with the highest `$n` in the text, found by a scan that skips quoted
     strings, quoted identifiers, dollar-quoted bodies and comments. They refuse a mismatch either
     way before anything is sent. A refusal at `execute` leaves the live batch unable to commit,
     because a refusal the server never saw does not abort its transaction. Asking the server
     through `describe` is declined. It prepares persistently with no declared types
     (`executor.rs:465-485`), which changes how later calls of the same text bind. This is a
     behaviour change with no signature change, recorded under `### Changed` in `0.4.0`.
   - **Neon:** this record claims nothing for Neon until it is measured.

   This sentence goes on every `push` and `execute`, under `# Parameter count`.
4. **Neon narrows in `0.4.0`; the shape is proposed.** The owner's default "Neon's `push`
   narrowing rides `0.4.0`" is in force (`runbook/handover.md:57-61`, under "Waiting on the
   owner"). That default names the release only. The shape below is this record's proposal and is
   the owner's to accept:
   - `NeonWriteBatch::push(&mut self, sql: &'static str, params: Vec<serde_json::Value>)`.
   - `push_raw_sql(&mut self, statement: SqlStatement)`.
   - `statements` becomes private, read through `pub fn statements(&self) -> &[SqlStatement]`.

   Without the private field the narrowing is decorative. `#[non_exhaustive]` does not stop
   `batch.statements.push(…)` on a batch the caller already holds (RS-13-1). `SqlStatement` itself
   stays open, because the event store and every `SqlTransport` implementor use it.
   `push_raw_sql` is the one named way to put such a statement into a batch.

   Neon's own probe builds its table name from `NeonConfig`, so it moves to `push_raw_sql`. That is
   the honest computed-shape case. It is a break to `happenstance-neon`, paid in `0.4.0`.

   The narrowing's code is not part of the change that lands this record. It lands in its own
   lane PR, against this record's shape.
5. **The escape hatches stay inconsistent with each other.** Today they are:
   - `impl Into<String>` with `impl IntoIterator` (SQLite);
   - `impl Into<Box<str>>` with `Vec` (Postgres, buffered);
   - `&str` with `Vec` (Postgres, live);
   - `SqlStatement` (Neon).

   The borrow-against-own split follows semantics. A buffered batch must own its text, and a live
   one executes now and has nothing to keep. The remaining differences are visible to no caller who
   passes a `String` or a `Vec`. Making them uniform is breaking, cheap only in `0.4.0`, and buys
   nothing that a caller can observe. The record accepts them as they are.
6. **One additive test per buffered adapter, plus rustdoc.**
   - **SQLite.** `a_statement_with_the_wrong_parameter_count_fails_the_commit_and_moves_nothing` goes
     in `crates/happenstance-sqlite/tests/projection.rs`. It has two legs, too few values and too
     many. Each leg is one batch holding a valid probe write and a `DELETE … WHERE k = ?1 OR k = ?2`
     that is short a value. The commit must fail as `Store`. The previously committed row and
     checkpoint must stand, and the valid write must be absent.
   - **Postgres.** A live-gated twin has the same two legs. Too few lands on acceptance. Too many
     lands with the count check. It uses a text no other test issues, so without the check the
     surplus is always a first preparation and commits, and the leg fails every time. A live-batch
     test asserts that a surplus at `execute` errors and that `commit` then refuses.
   - **Rustdoc.** Every `compile_fail,E0308` fence gets a compiling twin (RS-62-1). Postgres's `push`
     and `execute` gain the fence that SQLite's has. A stale sentence in `push_raw_sql`'s doc, which
     names callers that in fact use `push`, is corrected.

## What the test rejects

- **A replay that binds what it has and runs.** It uses `raw_bind_parameter` and `raw_execute`, or
  `execute_batch`. SQLite reads an unbound `?2` as `NULL`, so the `DELETE` succeeds, the committed
  row disappears, and the checkpoint moves.
- **A replay that drops surplus values.** It truncates to the statement's parameter count.
- **A replay that runs statements outside the checkpoint's transaction**, for example on autocommit
  before `BEGIN`. The valid probe write survives the failure.
- **A replay that logs a statement's error and continues**, then upserts the checkpoint. The
  checkpoint advances past a statement that never ran.

## Why now

Option B, or any change to a `push` or `execute` parameter type, is a major after 1.0. It is free
only in `0.4.0`. Recording Option A as final costs no signature change for SQLite and Postgres. It
closes the question that ADR-0066 left inside the promise. `projection-store` is not on §5's exemption list
(`.kb/decisions/0066-what-1-0-promises.md:258-269`).

## What this changes for a caller

- **SQLite and Postgres:** no signature changes. On Postgres, a statement with a surplus value,
  which today sometimes commits, is always refused once the count check lands. Every in-tree caller already passes a literal or a `const`, in
  `examples/rebuilding-read-models/src/main.rs:1100-1123`, `:1218`, `:1356`,
  `examples/tickets-over-http/src/lib.rs:622-630` and `examples/transfers-on-sqlite/src/main.rs:562-567`.
- **Neon:** the change is breaking.
  - `push(SqlStatement::with_params(lit, v))` becomes `push(lit, v)`.
  - A computed statement becomes `push_raw_sql(SqlStatement::…)`.
  - `batch.statements` becomes `batch.statements()`.
  - The only callers in the workspace are the crate's own probe and one unit test. Nothing under
    `examples/` or `docs/` uses `NeonWriteBatch`.

## Falsifiers

- **The arity table is wrong for Postgres.** If the Postgres twin's first live run shows that the
  server *accepts* too few values, or rejects too many on a fresh connection, the table and its
  rustdoc are corrected. The count check is still the proposal.
- **PS-9 is falsified.** A bound on `Batch` that carries a write vocabulary, in any published crate,
  would give a minted `Statement` the generic consumer it lacks. That re-opens Option B as a major.
- **A field report of an arity bug that the adapters let through, after the count check lands.**
  Neon is the only remaining candidate. Such a report would justify an additive `sql!` +
  `push_statement` pair. It would not justify replacing `push`.

## What is not decided

- **Neon's arity behaviour.** It is unmeasured, and this record owes no live test for it.
- **A contract-level trust-boundary statement on `happenstance-core`** about `metadata` and `data`
  reaching a projection store. It is still unreached.
- **`happenstance-ladybug`'s `push_raw_cypher`.** It is moot under ADR-0078.
