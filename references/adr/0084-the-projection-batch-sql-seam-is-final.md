# ADR-0084 — The projection batch's SQL seam is &'static str plus a named escape hatch at 1.0, and the parameter count is stated once

- **Status:** proposed. The owner decides §2.1, §2.2, §2.5, the Postgres count check in §2.3, and
  the *shape* in §2.4. That Neon
  narrows in `0.4.0` is a default the owner already holds; the shape §2.4 gives it is proposed.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window). This is the breaking open question
  `projection-batch-sql-seam-statement-type` (`runbook/phases/17-breaking-window.md:67-68`).
- **Decided by:** for §2.4's *release* only, the owner's default in force: "Neon's `push` narrowing rides `0.4.0`"
  (`runbook/handover.md:57-61`). For everything else, nobody yet. This record is the proposal.
- **Answers:** `kb-open-question-projection-batch-sql-statement-type-001`
  (`.kb/open-questions/projection-batch-sql-seam-statement-type.md`). It also answers that atom's
  "can be *stated* once, though, and is not yet" (`:118-121`).
- **Supersedes:** nothing. On acceptance it supersedes the open question named above.
  [ADR-0017](../../.kb/decisions/0017-what-a-projection-batch-owns.md),
  [ADR-0063](../../.kb/decisions/0063-the-projection-port-is-frozen.md) and
  [ADR-0074](../../.kb/decisions/0074-projection-apply-is-async.md) stay accepted and are relied
  on.
- **Evidence:** the source at `4fbfefa`, its citations re-checked at `6235224`. The vendored drivers this workspace locks:
  `rusqlite 0.40.1`, and `sqlx-core` and `sqlx-postgres 0.8.6` (`Cargo.lock`). Nothing new was
  run. §6 names the one measurement this record owes, and who runs it.

---

## 1. The seam as published

The projection port is frozen (ADR-0063). ADR-0066 §5 lists what 1.0 exempts from semver, and
`projection-store` is not on that list (`.kb/decisions/0066-what-1-0-promises.md:258-269`). Every
signature below is therefore inside the 1.0 promise, and has been published since `0.3.0`.

| # | Entry point | Signature at `4fbfefa` | Where |
|---|---|---|---|
| 1 | `SqliteBatch::push` | `(&mut self, sql: &'static str, params: impl IntoIterator<Item = Value>)` | `crates/happenstance-sqlite/src/projection_store.rs:487-489` |
| 2 | `SqliteBatch::push_raw_sql` | `(&mut self, sql: impl Into<String>, params: impl IntoIterator<Item = Value>)` | `:513-522` |
| 3 | `PendingStatement::sql` | `(&self) -> &str`, over a private `sql: Box<str>` | `:383-393` |
| 4 | `PostgresProjectionBatch::push` | `(&mut self, sql: &'static str, params: impl IntoIterator<Item = PgParam>)` | `crates/happenstance-postgres/src/projection_store.rs:252-257` |
| 5 | `PostgresProjectionBatch::push_raw_sql` | `(&mut self, sql: impl Into<Box<str>>, params: Vec<PgParam>)` | `:264-269` |
| 6 | `LivePostgresBatch::execute` | `async (&mut self, sql: &'static str, params: impl IntoIterator<Item = PgParam>) -> Result<(), PostgresProjectionStoreError>` | `crates/happenstance-postgres/src/live_projection_store.rs:114-121` |
| 7 | `LivePostgresBatch::execute_raw_sql` | `async (&mut self, sql: &str, params: Vec<PgParam>) -> Result<…>` | `:133-142` |
| 8 | `NeonWriteBatch::push` | `(&mut self, statement: SqlStatement)` | `crates/happenstance-neon/src/projection_store.rs:181-184` |

Rows 6 and 7 are new to this question. The open question named only SQLite, then added Postgres
and Neon in its phase-16 section (`.kb/open-questions/projection-batch-sql-seam-statement-type.md:146-156`).
Neither text mentions `LivePostgresBatch`, which is published through
`pub mod live_projection_store` (`crates/happenstance-postgres/src/lib.rs:208-210`). It is the batch
ADR-0074 drove its live leg through (`.kb/decisions/0074-projection-apply-is-async.md:208-212`), so
it is not an instrument a consumer may ignore. Any answer has to cover it.

Neon is open in more places than its `push`:

- `NeonWriteBatch` carries `pub statements: Vec<SqlStatement>`
  (`crates/happenstance-neon/src/projection_store.rs:136-138`).
- `NeonWriteBatch::new` is public and `const` (`:147-171`), and the type derives `Default` (`:134`).
- `SqlStatement` has `pub query: String`, and its constructors take `impl Into<String>`
  (`crates/happenstance-neon/src/transport.rs:95-119`).

The specification has no clause on any of this. `spec/SPECIFICATION.md` mentions `push_raw_sql`,
`SqlStatement` and `NeonWriteBatch::push` nowhere, and its `&'static str` hits are about
`EventType` (`:1274`, `:1300-1307`). No `[FROZEN]` clause moves. The clause that bounds the answer
is PS-9: "`Batch` MUST NOT carry a universal write vocabulary. A projection writes through the
concrete adapter's inherent API" (`spec/SPECIFICATION.md:5590-5591`, `[FROZEN]` by ADR-0074).

## 2. Decision

### 2.1 Option A is final for SQLite and Postgres (proposed)

Rows 1–7 keep their signatures through 1.x:

- `push` and `execute` take `&'static str`, with the values bound beside the text.
- `push_raw_sql` and `execute_raw_sql` are the separately named escape hatches, for a statement
  whose *shape* is computed at run time. An `IN (…)` list sized by a key count is the honest case.
- `PendingStatement::sql` keeps returning `&str`. The field it reads is private, so whether it
  stays `Box<str>` is an implementation detail and can change without a break. That answers the
  first item under the open question's "what this does not settle" (`:138`).

### 2.2 Option B, a minted `Statement` type, is declined as a replacement for `push`

The reasons are in §4. Declining it as a *replacement* is narrower than declining it altogether.
A `sql!` macro that mints a `Statement`, taken by a new inherent `push_statement`, can arrive in
any minor release, because adding an inherent method breaks nobody (§7). This record forecloses
only the second narrowing of `push` itself.

### 2.3 The arity obligation, stated once

> The number of values bound to a queued statement MUST equal the number of placeholders its text
> declares. No batch counts placeholders when a statement is queued. A mismatch is refused when the
> statement runs, and the adapter reports the refusal as `CommitError::Store` (buffered batches) or
> as the `execute` error (the live batch).

Its third sentence is true today **only for SQLite, and for Postgres in one direction.** The
driver does not discharge the whole obligation everywhere, and the table says where it does not.
The evidence is in §5.

| Adapter | When | Too few values | Too many values |
|---|---|---|---|
| `happenstance-sqlite` | At `commit`/`reset`, inside `BEGIN IMMEDIATE` | refused, `InvalidParameterCount` | refused, `InvalidParameterCount` |
| `happenstance-postgres`, buffered | At `commit`/`reset`, inside the transaction | refused by the server at `Bind` | **not enforced; depends on the connection.** Accepted, and committed, when this connection prepares the text for the first time (the surplus is an unused, typed parameter). Refused when the connection's statement cache already holds the text with the right count. A surplus accepted first leaves a cached statement that refuses every later *correct* call of that text on that connection |
| `happenstance-postgres`, live | At `execute`, and the transaction is then poisoned | refused by the server at `Bind` | **not enforced**, as above |
| `happenstance-neon` | At `commit`/`reset`, server-side | unmeasured | unmeasured |

**Who checks the count, at `4fbfefa`.** Neither Postgres batch checks it. `bind_all` binds every
supplied value, in order, with no count check
(`crates/happenstance-postgres/src/projection_store.rs:322-337`). `push` and `push_raw_sql`
store the values as given (`:252-269`). `execute_raw_sql` hands them straight to `bind_all`
(`live_projection_store.rs:133-142`). So on Postgres the too-many direction is checked by nothing
on the adapter's side, and only sometimes by the server.

**Proposed: `happenstance-postgres` counts before it binds.** A surplus that commits silently is
the failure the obligation exists to prevent. One that also makes later correct calls fail, on
whichever pooled connection it reached first, is worse. Leaving that to the caller is not an
honest reading of "stated once". So `replay` and `execute_raw_sql` compare `params.len()` with the
highest `$n` in the text before anything is sent, and refuse a mismatch in either direction as a
store error. A refusal at `execute` must also leave the live batch unable to commit, as a server
refusal does today. A refusal the server never saw does not abort the transaction, so the adapter
has to mark the batch itself. The check is a run-time one, at the same place the driver's check
sits, so it does not reopen Option B (§4.3). Two mechanisms were weighed:

- **A placeholder scan of the text.** This is the proposal. It finds the highest `$n` outside
  quoted strings (`'…'`, `E'…'`), quoted identifiers, dollar-quoted bodies and comments. It needs
  no round trip, it has the same answer on every connection, and a unit test can pin it without a
  server. Its cost is a small lexer in one crate. That cost is lower than §4.3's macro bar, because
  it runs on a `&str` at run time and not on a token at expansion.
- **Asking the server, through `describe`.** Declined. `describe` prepares *persistently* with no
  declared types (`…/sqlx-postgres-0.8.6/src/connection/executor.rs:465-485`). It would put the
  server-inferred types into the same per-text cache that `query()` reads. That changes how every
  later call of that text binds, and it adds a round trip per statement.

With the check in place, the Postgres rows read *refused, by the adapter, before `Bind`* for both
directions. The obligation then holds for every measured adapter, and Neon stays unmeasured
(§5.3).

The statement goes on each `push` and `execute` under a `# Parameter count` heading. The text is
in Appendix A.3. It is not a clause. The seam is each adapter's inherent API, PS-9 keeps it out of
the port, and the testkit cannot reach it.

### 2.4 Neon narrows in `0.4.0`, on the owner's default; the shape is proposed

That Neon's `push` narrows in `0.4.0` is the owner's default in force
(`runbook/handover.md:57-61`). The default names the release and nothing else. The shape below is
this record's proposal, and the owner accepts or amends it with the rest:

- `pub fn push(&mut self, sql: &'static str, params: Vec<serde_json::Value>)`. The narrow door,
  with the same type obligation as SQLite and Postgres.
- `pub fn push_raw_sql(&mut self, statement: SqlStatement)`. The named door for everything else.
  It takes the transport's own type, because that type is what a computed Neon statement already
  is.
- `statements` becomes private, read through `#[must_use] pub fn statements(&self) -> &[SqlStatement]`.
- Neon's conformance probe moves to `push_raw_sql`. Its table name is
  `qualified_probe(&self.config)`, a `format!` over the configured schema
  (`crates/happenstance-neon/src/projection_store.rs:636-644`, used at `:676-689` and `:696-701`).
  That is the computed-shape case the escape hatch exists for.

The narrowing's code is not in the change that lands this record, which is records only. It lands
in its own lane PR, against the shape above.

**Why the field has to go private, or the rest is decorative.** `NeonWriteBatch` is
`#[non_exhaustive]` (`:135`). That attribute stops a *struct-literal expression*
(`NeonWriteBatch { statements, stamp }`) outside the crate. It does nothing to a field access on a
value the caller already holds. `batch.statements.push(SqlStatement::new(format!(…)))` compiles
today and would still compile after `push` narrowed. The constitution records the trap as RS-13-1:
"`#[non_exhaustive]` … does nothing to `value.field = x` on a value the caller already holds, so an
invariant guarded only by the attribute is not guarded at all"
(`standards/rust/13-sealing-and-exhaustiveness.md:12-17`). A private field is the only way, short
of `unsafe`, to make that spelling fail, and it fails as `error[E0616]`.

**Why `SqlStatement` stays open.** It is the transport's vocabulary, not the batch's. The event
store builds its statements with it, and every `SqlTransport` implementor receives a `SqlRequest`
of them (`crates/happenstance-neon/src/transport.rs:122-137`). Narrowing it would break the
transport seam to fix the batch seam. With `statements` private, an open `SqlStatement` can enter
a batch only through `push_raw_sql`, whose name is the warning. That is exactly the guarantee
SQLite and Postgres give.

`params: Vec<serde_json::Value>` rather than `impl IntoIterator` is this record's choice (no
stated owner shape exists in tree). It is what `SqlStatement::with_params` already takes (`transport.rs:114`), so `push` stores
the vector without collecting it again.

### 2.5 The escape hatches stay as they are (proposed)

They differ:

| Escape hatch | Text | Values |
|---|---|---|
| `SqliteBatch::push_raw_sql` | `impl Into<String>` | `impl IntoIterator<Item = Value>` |
| `PostgresProjectionBatch::push_raw_sql` | `impl Into<Box<str>>` | `Vec<PgParam>` |
| `LivePostgresBatch::execute_raw_sql` | `&str` | `Vec<PgParam>` |
| `NeonWriteBatch::push_raw_sql` (after §2.4) | `SqlStatement` | inside it |

This record declines to make them uniform. The argument:

- **The borrow-against-own split is semantic, not accidental.** A buffered batch outlives the call
  that filled it and is carried across an await. It cannot borrow, so it must own its text, and
  both buffered hatches take something convertible into an owned string. The live batch sends the
  statement *now* and keeps nothing (`live_projection_store.rs:138-141`), so `&str` is its honest
  type. Making it own would force an allocation on every live statement and buy nothing.
- **What remains is invisible to an ordinary caller.**
  - `impl Into<String>` and `impl Into<Box<str>>` accept the same everyday arguments: a `&str`, a
    `String`, a `Box<str>` and a `Cow<str>`. Both `From` impls exist in `std`.
  - A caller passing a `Vec` to an `impl IntoIterator` parameter compiles unchanged.
- **Uniformity is a break, priced in `0.4.0`.** Widening `Vec<PgParam>` to
  `impl IntoIterator<Item = PgParam>` compiles at every call site this workspace has, but it is
  not a guaranteed-compatible change. Rust's API-evolution guidelines (RFC 1105) class
  "generalising a concrete parameter to a generic one" as a minor change that *can* break callers,
  through inference. The common inference cases, `Vec::new()` and `[]`, still resolve, because
  `Vec<T>` has exactly one `IntoIterator` impl with `Self = Vec<T>`. The guidance still makes it a
  change to make deliberately, with nothing to show for it here.
- **What would flip it:** a consumer who has to write a different call shape for each adapter in
  code that is otherwise identical. No such code exists. PS-9 makes every projection adapter-typed,
  so a projection is written against one adapter at a time.

### 2.6 One additive test per adapter, and rustdoc

Specified in §6, with code sketches in Appendix A. The tests and the rustdoc are additive and
two-way, so they are implemented on acceptance and need no further decision. The Postgres
surplus leg is the exception. It asserts §2.3's count check, so it lands with that check.

## 3. Why `&'static str` is the right type, and what it does not prove

For a reader new to Rust lifetimes, it is worth saying exactly what this type admits.

**`&'static str` is a borrowed string slice that is valid for the rest of the program.** The
`'static` lifetime does not mean "immutable". It means the bytes will never be freed while
anything can see them. The compiler can prove that for three spellings, and they are the ones a
statement written in source takes:

- a string literal, `"DELETE FROM t WHERE k = ?1"`, which is placed in the binary's read-only data;
- a `const` of type `&str` (`const UPSERT: &str = "…"`), which is a literal with a name. That is
  how `examples/rebuilding-read-models/src/main.rs:1100-1123` keeps two statements that differ in
  one table name;
- `concat!("…", "…")`, which the compiler expands to a single literal.

**It refuses a statement assembled at run time, in two different ways.**

- **Passing the `String` itself is a type mismatch.** `format!(…)` produces a `String`, an owned,
  heap-allocated buffer. Passing it where `&'static str` is expected is `error[E0308]: mismatched
  types`. Rust never converts an owned value into a borrow implicitly at a function argument. That
  is the spelling the `compile_fail,E0308` fence pins
  (`crates/happenstance-sqlite/src/projection_store.rs:468-478`).
- **Passing a borrow of the `String` is a lifetime error.** `&format!(…)`, or `&s` for a local
  `String`, has the right *type* (`&str`) but too short a *lifetime*: the borrow ends when the
  `String` is dropped, at the end of the statement or the function. That spelling fails as a
  borrow-check error (`E0716` for a temporary, `E0597` for a local), not `E0308`. The
  type-mismatch fence does not cover it. The borrow checker does, and that is why a fence's error
  code is not the check. RS-62-1 says the same: "rustdoc 1.97.1 compares the error code, finds no
  match, and reports the fence as **passing anyway**"
  (`standards/rust/62-doctests-and-harnesses.md:12-17`).

**What it does not prove.** Provenance, not shape. `const BAD: &str = "… WHERE k = 'x' OR 1=1"`
compiles, correctly, because it is in the source, and the review that reads the source is where it
is caught. `Box::leak(s.into_boxed_str())` turns any `String` into a `&'static str` by never
freeing it. That defeats the type for anyone determined to, and leaks memory on every call. Both
were known when Option A landed (`.kb/open-questions/projection-batch-sql-seam-statement-type.md:94-100`).
The type is a guard rail against the accidental `format!`. It is not a proof against a hostile
author, and no type in this seam could be, because `push_raw_sql` must exist.

## 4. Why not a minted `Statement` type

Option B was a `sql!(…)` macro producing a `Statement` newtype, which `push` would take in place of
`&'static str` (`.kb/open-questions/projection-batch-sql-seam-statement-type.md:101-107`). It is
declined as a replacement for `push` on five grounds.

**4.1 It cannot be one type.** SQLite's placeholders are `?`, `?NNN`, `:name`, `@name` and `$name`.
Postgres's are `$1`, `$2`, and so on. A `Statement` that promised anything about its placeholders
would have to know the dialect. That means one `Statement` and one macro per adapter:
`happenstance_sqlite::sql!` and `happenstance_postgres::sql!`, each its own public type. Two
newtypes that share a name and differ in meaning are what RS-10-1 warns about, from the other
side. A newtype earns its keep when it makes a wrong value unconstructible
(`standards/rust/10-newtypes-and-niches.md:11`). Here the only wrong value it could exclude beyond
`&'static str`'s exclusions is a placeholder-count mismatch, and §4.3 shows that cannot be checked
when the statement is minted.

**4.2 Nothing generic would consume it.** The reason to mint a shared type is to let generic code
name it. PS-9 is frozen on the ground that no library code writes into an unknown adapter's batch
(`spec/SPECIFICATION.md:5590-5600`). ADR-0074 made that a compiled fact rather than an argument: a
skip row was written through `SqliteBatch::push` and through `LivePostgresBatch::execute` with no
bound on `Batch` (`.kb/decisions/0074-projection-apply-is-async.md:196-221`). ADR-0017 refused the
universal write vocabulary first (`.kb/decisions/0017-what-a-projection-batch-owns.md:82-88`). A
per-adapter `Statement` would therefore be consumed only by the one adapter's own `push`, which
already has a type that does the provenance job.

**On coherence, since this is where a newcomer might expect a trait.** The alternative to two
newtypes is one trait, `trait Statement { fn text(&self) -> &str; }`, implemented by each adapter's
type. Rust's *coherence* rules decide who may write `impl Trait for Type`, so that two crates can
never supply conflicting impls for the same pair. Their downstream half is the *orphan rule*: a
crate may write an impl only if it defines the trait or the type (roughly; generics add detail).
So a downstream crate **cannot** write `impl happenstance_sqlite::Statement for String`. It owns
neither `Statement` nor `String`, and the compiler refuses with `error[E0117]`. What the orphan
rule *does* allow is `impl happenstance_sqlite::Statement for MyQuery`, where `MyQuery` is the
downstream crate's own type, because it owns the type. That is the hole. `MyQuery::text` can
return a `format!`-built string, and `push` would accept it, which reopens the seam `&'static str`
closed. To shut it the trait would have to be **sealed**. Sealing means a supertrait that lives in
a private module, so no outside crate can name it and therefore none can satisfy it, even for its
own types. Sealing a trait after it is published is a major (the codec atom that is this record's
sibling turns on exactly that). Sealing it from the start makes it a closed vocabulary across
adapters, which PS-9 forbids. Inherent methods on concrete types raise none of this. Nobody outside
`happenstance-sqlite` can add an inherent method to `SqliteBatch`, so the seam is closed by
construction, with no sealing machinery.

**4.3 Its one extra guarantee cannot be made at compile time.** The open question named the
placeholder/parameter-count check as the reason to want a minted type
(`.kb/open-questions/projection-batch-sql-seam-statement-type.md:88-92`). A `sql!` macro sees the
literal at expansion and can count its placeholders. It cannot see the values: `params` is
`impl IntoIterator<Item = Value>` or `Vec<PgParam>`, whose length exists only at run time. A check
between the two is therefore a run-time check. It would sit at `push`, which returns `()` and
cannot fail. It would have to be parked in the batch and reported at `commit`. For SQLite, and for
too few values on Postgres, the driver already does that (§5). For a surplus on Postgres, §2.3's
adapter-side count check does it, at the same point and with no new type.

The alternative that *would* move the check to compile time is a typed parameter tuple:
`sql!("… ?1, ?2", a, b)`, with the macro emitting a fixed-arity binding. It costs every call site
its iterator, makes the computed-shape case inexpressible except through the escape hatch, and
adds a proc-macro-grade parser for two dialects to two crates. A declarative macro cannot count
`?NNN` placeholders inside a string literal at all, because `macro_rules!` sees a literal as one
opaque token. That parser is the bar for a compile-time arity check here, and nothing in the tree
has asked for it.

**4.4 It is exported macro surface.** `standards/rust/41-declarative-macros.md` applies in full:
`$crate::`-qualified paths (RS-41-1), arm ordering (RS-41-3), a `compile_fail` per refusal.
Under ADR-0066 a published macro's accepted input is a semver surface as much as a function
signature is.

**4.5 It would be the second narrowing of the same parameter.** `push` narrowed once, from
`impl Into<String>` to `&'static str`, in `0.2.0` (`CHANGELOG.md:1896-1925`, under `## [0.2.0]`). A second
narrowing in `0.4.0` would break every caller in `examples/` (§8) for a guarantee that §4.3 shows
only a run-time check can give, and a run-time check needs no new type. The CHANGELOG entry that landed Option A said why that matters: "a signature narrowed twice is
worse than one narrowed once" (`CHANGELOG.md:1921-1925`).

## 5. The arity obligation: what each driver does

The open question said the obligation "cannot be resolved once for all three by construction. It
can be *stated* once, though, and is not yet" (`:118-121`). §2.3 states it. This section is the
evidence behind each row.

**5.1 SQLite refuses both directions.** The batch replays each statement with
`transaction.execute(statement.sql(), params_from_iter(statement.params()))`
(`crates/happenstance-sqlite/src/projection_store.rs:982-991`), inside the `BEGIN IMMEDIATE` that
`commit_locked` (`:927-962`) and `reset_locked` (`:964-979`) open. rusqlite 0.40.1 binds through
`Statement::bind_parameters`
(`~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rusqlite-0.40.1/src/statement.rs:474-493`):

```rust
let expected = self.stmt.bind_parameter_count();
let mut index = 0;
for p in params {
    index += 1; // The leftmost SQL parameter has an index of 1.
    if index > expected {
        break;
    }
    self.bind_parameter(&p, index)?;
}
if index != expected {
    Err(Error::InvalidParameterCount(index, expected))
} else {
    Ok(())
}
```

Too few values leaves `index < expected`, and too many breaks out at `expected + 1`. Either way
the result is `InvalidParameterCount`. The `?` in `apply` returns it before the checkpoint upsert
runs. The transaction is dropped uncommitted, and `commit` reports `CommitError::Store`
(`:700-706`).

This matters because of SQLite's own rule. A parameter that is never bound is `NULL`, and SQLite
does not complain about it. A replay that bound values one at a time with `raw_bind_parameter` and
ran the statement with `raw_execute`, skipping `bind_parameters`, would execute an under-bound
`DELETE … WHERE k = ?1 OR k = ?2` as `… OR k = NULL` and report success. The safe answer comes from
which rusqlite entry point the adapter calls. The test in §6 pins that choice.

**5.2 Postgres refuses too few; too many depends on the connection's statement cache.** Both Postgres batches bind with
`bind_all(sqlx::query(sql), params)` (`crates/happenstance-postgres/src/projection_store.rs:322-337`).
The buffered batch replays at commit inside the transaction (`:340-349`, called at `:502-504`). The
live batch executes at once (`live_projection_store.rs:133-142`). `sqlx-postgres` 0.8.6 prepares the
statement with the bound values' types (`…/sqlx-postgres-0.8.6/src/connection/executor.rs:228-229`,
`get_or_prepare(query, &arguments.types, …)`). It sends those types in the `Parse` message
(`:23-53`), then sends a `Bind` with every value (`:240-248`).

- **Too few.** `Parse` declares fewer types than the text uses. The server infers the rest, and the
  prepared statement requires as many parameters as its highest `$n`. The `Bind` then supplies fewer
  values, and the server refuses it (*"bind message supplies 1 parameters, but prepared statement
  "" requires 2"*, SQLSTATE `08P01`). In the buffered batch the error propagates out of `replay`
  before the checkpoint upsert, the transaction is dropped, and `commit` returns `Store`. In the
  live batch `execute` returns the error, and the existing poison test shows what follows: every
  later statement, the checkpoint included, is refused
  (`crates/happenstance-postgres/tests/live_projection.rs:152-210`).
- **Too many.** `Parse` declares *more* types than the text uses. Postgres accepts a declared but
  unreferenced parameter, the same way `PREPARE p(text, text) AS SELECT $1` is valid SQL. The
  statement then requires as many parameters as were declared, the `Bind` matches, and the surplus
  value is ignored — **on first preparation only.** sqlx-postgres 0.8.6 caches a prepared statement per connection keyed by its SQL text alone (`executor.rs:177-179`; `query()` is persistent and the cache holds 100 by default)
  (`…/sqlx-postgres-0.8.6/src/options/mod.rs:88`; `sqlx-core-0.8.6/src/query.rs:663`). `Parse` is
  skipped on a cache hit, so the declared types are whatever the *first* execution on that pooled
  connection declared. Two consequences: a surplus sent where the text is already cached with the
  correct count is refused at `Bind`; and a surplus sent *first* caches a statement requiring N+1
  values, after which every correct N-value call of the same text on that connection is refused
  until the entry is evicted. The happenstance-postgres adapter does not change the cache
  (`grep -rn statement_cache crates/happenstance-postgres/src` is empty).

This row is **reasoned from the protocol and the vendored driver, not measured.** The Postgres twin
in §6 measures the too-few leg. Its first run is the check on that half. The too-many half is not
something the driver can be left with. Today a surplus value is the caller's bug. It is reported
nondeterministically (per connection), it can commit, and it can make correct calls of the same
text fail on that connection later. That is why §2.3 proposes the adapter-side count check, and
why §6's Postgres test gains a surplus leg once the check exists. It does not argue for Option B,
because the check is still a run-time one.

**5.3 Neon is unmeasured.** The batch is a JSON array of `{query, params}` objects, run server-side
in one transaction (`crates/happenstance-neon/src/projection_store.rs:287-329`). How the endpoint
turns `params` into `Parse` and `Bind` is not visible from this side. This record claims only that
the obligation is the caller's and that a refusal arrives as `CommitError::Store`. It owes no live
Neon test. Neon's live suite is gated on a secret, and on this axis it adds nothing that Postgres
does not already show.

## 6. The test, and what it rejects

**SQLite, required.**
`a_statement_with_the_wrong_parameter_count_fails_the_commit_and_moves_nothing` goes in
`crates/happenstance-sqlite/tests/projection.rs`, among the adapter-private tests (after `commit_one`, `:241-248`).
It commits `KEY = VALUE` at `SequencePosition::FIRST` and then runs two legs. Each leg is one batch
holding a valid `probe_write("fresh", 7)` followed by one mismatched statement:

- **Too few:** `DELETE FROM projection_probe WHERE k = ?1 OR k = ?2` with one value, `KEY`.
- **Too many:** `DELETE FROM projection_probe WHERE k = ?1` with two values, `KEY` and `"unused"`.

Each leg must:

- fail as `CommitError::Store`;
- leave `KEY` reading `VALUE`;
- leave `"fresh"` absent;
- leave the checkpoint at `Live { through: FIRST }`.

The mismatched statement is a `DELETE` aimed at the committed row on purpose. Every wrong
implementation below then *changes something observable* instead of failing quietly in a way that
looks like a pass.

| Wrong implementation | What it does | Which assertion fails |
|---|---|---|
| A replay that binds what it has: `raw_bind_parameter` per value, then `raw_execute` | runs `… OR k = NULL`, deletes `KEY`, commits | `KEY` reads `None`, and the checkpoint is at 2 |
| A replay through `execute_batch(sql)` | ignores every parameter, so `?1` and `?2` are both `NULL`; deletes nothing; commits | the commit returns `Ok`, and the checkpoint is at 2 |
| A replay that truncates surplus values to `parameter_count()` | runs the too-many leg's `DELETE` with `KEY`, commits | `KEY` reads `None` (too-many leg) |
| A replay on autocommit before `BEGIN IMMEDIATE` | `"fresh"` lands before the failure | `"fresh"` reads `Some(7)` |
| A replay that logs a statement error and continues | the checkpoint upserts past a statement that never ran | the commit returns `Ok`, and the checkpoint is at 2 |

None of these exists in the tree, and none is plausible as a deliberate choice. Each is plausible
as a refactor, though. "Use `raw_execute` to avoid re-preparing" and "batch the statements with
`execute_batch`" are both performance-shaped edits. Today the adapter's choice of rusqlite entry
point is the only thing that discharges the obligation, and nothing would notice the change.

**Postgres, required, live-gated.**
`a_statement_with_the_wrong_parameter_count_fails_the_commit_and_moves_nothing` goes in
`crates/happenstance-postgres/tests/projection.rs`, under the same
`#[ignore = "needs a live Postgres; …"]` as its neighbours. It runs against
`PostgresProjectionBatch`, with `$1`/`$2` placeholders and the same four assertions, in two legs:

- **Too few.** This leg can land on acceptance, because the server already refuses it. It rejects
  the "replay outside the transaction" and "log and continue" implementations.
- **Too many.** This leg lands with §2.3's count check. Its text is used by no other test, so on
  every pooled connection the surplus is a first preparation. An adapter without the check then
  commits it. The leg fails *deterministically*, not by the luck of the cache. After the refused
  surplus, a correct call of the same text must still commit. That rejects a refusal that comes too
  late, after the statement has reached the server. By then the surplus is prepared and cached
  with two parameters, and the correct call is refused. The leg runs on a store whose pool holds
  one connection, so the correct call reuses the connection the surplus would have reached.

`LivePostgresBatch` gains one live-gated test with the surplus leg. An `execute` with one value too
many must return the error, and `commit` must then refuse. That rejects a check that refuses the
statement locally but leaves the transaction un-aborted, so that the checkpoint still commits. An
arity refusal from the server, the too-few leg, is a refused statement, and
`a_refused_statement_poisons_the_batch_and_commit_says_so` already covers what follows it.

The count scan gets unit tests in the adapter. They pin the highest `$n` it finds, and that it skips
`$n` inside quoted strings, dollar-quoted bodies and comments.

**Rustdoc, required.**

- **A compiling twin for the SQLite fence.** The `compile_fail,E0308` fence on `SqliteBatch::push`
  is unpaired today. Under RS-62-1 it would stay green if `SqliteBatch` were renamed or moved behind
  a feature. It gains a compiling twin.
- **Postgres gains the same pair.** `PostgresProjectionBatch::push` and `LivePostgresBatch::execute`
  gain the pair that SQLite's `push` has. Neither has a fence today. Their narrowing is held by
  nothing that runs (`grep -rn compile_fail crates/happenstance-postgres/src` is empty).
- **A stale sentence is corrected.** `SqliteBatch::push_raw_sql` says "What the two callers inside
  this crate do is the pattern (`probe_write` and `probe_delete_all` below)"
  (`crates/happenstance-sqlite/src/projection_store.rs:502-505`). Both of those call `push`
  (`:813`, `:832`). Nothing in the crate calls `push_raw_sql`.
- **The arity paragraph** of §2.3 is added under `# Parameter count` on rows 1, 2, 4, 5, 6, 7 and on
  Neon's new pair (Appendix A.3).

## 7. Semver: what this costs now and what it leaves open

**Proposed (Option A, §2.1, §2.5):** no signature changes. `cargo-semver-checks` reports nothing,
and the `0.4.0` trace table needs no row for SQLite or Postgres. The rustdoc and the tests are
additive. ADR-0066 §5 exempts rustdoc prose (`.kb/decisions/0066-what-1-0-promises.md:265`).

**The Postgres count check (§2.3)** changes no signature either. It changes behaviour: a statement
with a surplus value, which today sometimes commits, is now always refused. That is a `### Changed`
entry in `happenstance-postgres`'s CHANGELOG, and it rides `0.4.0` with the rest.

**Neon (§2.4; the release is the owner's default, the shape proposed):** three breaks to `happenstance-neon`, all compiler-caught:

- `push`'s parameter list changes from `(SqlStatement)` to `(&'static str, Vec<Value>)`, so every
  existing call is `error[E0061]` (wrong number of arguments).
- Reading `batch.statements` becomes `error[E0616]` (private field). A pattern
  `NeonWriteBatch { statements, .. }` becomes `error[E0451]`.
- `push_raw_sql` and `statements()` are additions.

`cargo-semver-checks` reports the removed public field and the changed method. The trace table
maps both to this record. The CHANGELOG entry is drafted in Appendix A.4.

**What stays open after 1.0, without a major.** Adding an inherent method to a concrete type is a
minor change. A downstream crate cannot have defined a method of the same name on
`SqliteBatch`, because it cannot write an `impl SqliteBatch` block at all; inherent impls are
allowed only in the defining crate. The one hazard is a *trait* method of the same name that a
caller has in scope. An inherent method wins method resolution over a trait method, so adding
`push_statement` could silently redirect a call to an extension trait's `push_statement`. RFC 1105
lists this as a minor change with known breakage, and it is the reason a future name should be
chosen to be unlikely in a caller's extension traits. So these remain additive after 1.0:

- `sql!` + `push_statement(Statement)`;
- an arity-checking `push_checked` that refuses at queue time, not at commit, returning a
  `Result`;
- Neon's arity measurement and a test for it.

What becomes a major after 1.0: any change to the parameter types of rows 1, 2, 4–7, or of Neon's
new pair.

## 8. Call sites, and what Option A costs them

Every caller of a SQL batch in the workspace (crates, examples and docs) at `4fbfefa`. Outside that
scope, `experiments/apply-shape/tests/sqlite.rs:66`, `:83` also call `SqliteBatch::push` with literals:

| Caller | Entry point | Text |
|---|---|---|
| `crates/happenstance-sqlite/src/projection_store.rs:813-823` (`probe_write`) | `SqliteBatch::push` | literal |
| `crates/happenstance-sqlite/src/projection_store.rs:832` (`probe_delete_all`) | `SqliteBatch::push` | literal |
| `crates/happenstance-postgres/src/projection_store.rs:644-…` (`probe_write`) | `PostgresProjectionBatch::push` | literal |
| `crates/happenstance-postgres/src/projection_store.rs:662` (`probe_delete_all`) | `PostgresProjectionBatch::push` | literal |
| `crates/happenstance-postgres/src/projection_store.rs:781` (unit test) | `PostgresProjectionBatch::push` | literal |
| `crates/happenstance-postgres/src/live_projection_store.rs:382-…`, `:392` | `LivePostgresBatch::execute` | literal |
| `crates/happenstance-neon/src/projection_store.rs:676-689` (`probe_write`) | `NeonWriteBatch::push` | **`format!`** over `qualified_probe` |
| `crates/happenstance-neon/src/projection_store.rs:696-701` (`probe_delete_all`) | `NeonWriteBatch::push` | **`format!`** over `qualified_probe` |
| `crates/happenstance-neon/src/projection_store.rs:878` (unit test) | `NeonWriteBatch::push` | literal |
| `examples/transfers-on-sqlite/src/main.rs:562-567` | `SqliteBatch::push` | literal |
| `examples/tickets-over-http/src/lib.rs:622-630` | `SqliteBatch::push` | literals |
| `examples/rebuilding-read-models/src/main.rs:1008`, `:1084`, `:1266` | `SqliteBatch::push` | literals |
| `examples/rebuilding-read-models/src/main.rs:1218` | `SqliteBatch::push` | `self.upsert: &'static str`, one of two `const`s |
| `examples/rebuilding-read-models/src/main.rs:1356` | `SqliteBatch::push` | `delete: &'static str` parameter |

- **No caller anywhere uses `push_raw_sql` or `execute_raw_sql`.** That is evidence about current
  use, not proof that no consumer needs them. The open question said the same (`.kb/open-questions/projection-batch-sql-seam-statement-type.md:72-77`).
- **The only computed statements in the workspace are Neon's own probe.** That is why §2.4 moves it
  to the escape hatch, and it is the one place the escape hatch is honestly needed.
- **No file under `docs/` calls a batch.**
- `rebuilding-read-models` already wrote down the cost of `&'static str` in one table name, two
  `const`s and a doc comment (`:1100-1123`). That is the whole recorded cost in three examples.

## 9. Consequences

- The open question closes on acceptance, as `superseded` by this record. Its amendment, the map
  rows and the ledger line land with this record. The rustdoc, the tests and the Neon entry are
  sketched in Appendix A.
- Every SQL batch entry point carries the arity paragraph. The obligation is written once, in this
  record, and repeated per method because rustdoc has no shared include.
- Three tests are added: SQLite's, the buffered Postgres pair of legs, and the live surplus test.
  None touches the testkit, because the seam is outside the port (PS-9).
- `happenstance-postgres` gains the count check of §2.3, and a `### Changed` entry for it.
- `happenstance-neon`'s `0.4.0` CHANGELOG carries one BREAKING entry for the batch, in the lane
  PR that lands the narrowing's code (§2.4), not in this record's change.
- Phase 17's item `projection-batch-sql-seam-statement-type` is struck when this record is
  accepted, not when it is proposed.

## 10. Falsifiers

- **The Postgres twin's first live run refuses to fail.** If the server accepts an under-bound
  `Bind`, §5.2 is wrong about the protocol. The statement and the rustdoc are corrected, and the
  decision is re-examined, because the obligation would then be met by nothing on Postgres.
- **A surplus value is refused on Postgres on a fresh connection.** Then §5.2's first-preparation
  half is wrong, and only the prose changes.
- **PS-9's falsifier fires.** That would be a bound on `Batch` that carries a write vocabulary, in
  a crate this workspace publishes. A shared `Statement` would then have a generic consumer, and
  Option B returns as a major.
- **A field report of an arity bug that the adapters let through, after §2.3's check lands.** Neon
  is the only remaining candidate. Such a report justifies the additive `push_checked` or `sql!` of
  §7. It does not justify replacing `push`.
- **The count scan disagrees with the server.** If a statement the server accepts is refused by
  the scan, or the reverse, the scan is wrong and is fixed. A unit test pins the case. The
  obligation is unchanged.

## 11. Out of scope

- **Neon's arity behaviour** (§5.3).
- **A trust-boundary statement for `metadata` and `data`** at the contract level. That is
  `happenstance-core`'s question, and the open question left it unreached (`:141-144`).
- **`happenstance-ladybug`'s `push_raw_cypher`.** It is retired with its crate (ADR-0078).
- **The `SqlStatement` transport type's own openness**, which the event store depends on.
- **Making the escape hatches uniform** (§2.5): declined here, and only cheap in `0.4.0`.

## Appendix A. What acceptance implements

These are sketches. The lane PR that implements each one owns its final text. Line numbers are
against `4fbfefa`.

### A.1 SQLite: the commit-arity test

The test goes in `crates/happenstance-sqlite/tests/projection.rs`, after `commit_one` (`:241-248`),
and imports `happenstance_sqlite::rusqlite::types::Value`.

```rust
#[tokio::test]
async fn a_statement_with_the_wrong_parameter_count_fails_the_commit_and_moves_nothing() {
    let fixture = SqliteProjectionFixture::new();
    let store = fixture.connect().await;
    let id = ProjectionId::new("a_statement_with_the_wrong_parameter_count");
    commit_one(&store, &id, SequencePosition::FIRST).await; // KEY = VALUE

    let legs: [(&str, &'static str, Vec<Value>); 2] = [
        ("too few", "DELETE FROM projection_probe WHERE k = ?1 OR k = ?2",
         vec![Value::Text(KEY.to_owned())]),
        ("too many", "DELETE FROM projection_probe WHERE k = ?1",
         vec![Value::Text(KEY.to_owned()), Value::Text("unused".to_owned())]),
    ];
    for (leg, sql, params) in legs {
        let mut batch = store.begin().await.unwrap();
        store.probe_write(&mut batch, "fresh", 7).await.unwrap();
        batch.push(sql, params);
        let refused = store
            .commit(batch, &id, SequencePosition::new(2).unwrap(), Authority::Live)
            .await
            .expect_err("a parameter-count mismatch must fail the commit");
        assert!(matches!(refused, CommitError::Store(_)), "{leg}: {refused:?}");
        assert_eq!(store.probe_read(KEY).await.unwrap(), Some(VALUE), "{leg}");
        assert_eq!(store.probe_read("fresh").await.unwrap(), None, "{leg}");
        assert_eq!(
            store.checkpoint(&id).await.unwrap(),
            Checkpoint::Live { through: SequencePosition::FIRST },
            "{leg}"
        );
    }
}
```

### A.2 Postgres: the legs

The buffered test in `crates/happenstance-postgres/tests/projection.rs` has the same shape as
A.1. It commits a `"kept"` row first, uses `$1`/`$2` and `PgParam::text`, and carries the
neighbours' `#[ignore = "needs a live Postgres; …"]`. Its too-many leg uses a text no other test
issues, `DELETE FROM projection_probe WHERE k = $1 /* arity-surplus */`. After that leg is
refused, it commits the same text with one value and asserts that the commit succeeds (§6). The
live test in `crates/happenstance-postgres/tests/live_projection.rs` issues that text through
`LivePostgresBatch::execute` with two values. It asserts the `Err`, then asserts that `commit`
refuses.

### A.3 The `# Parameter count` rustdoc

On `SqliteBatch::push`:

> The values must match the placeholders `sql` declares, one for one. Nothing counts them here.
> The driver does, when the batch is replayed inside `commit`'s or `reset`'s `BEGIN IMMEDIATE`,
> and it refuses a mismatch in either direction. The refusal is `CommitError::Store`, and neither
> the batch's statements nor the checkpoint move. This matters because SQLite reads an unbound
> placeholder as `NULL` and does not complain on its own.

On `PostgresProjectionBatch::push`, once §2.3's check lands:

> The values must match the highest `$n` that `sql` uses, one for one. Nothing counts them here.
> At `commit` the batch counts them before binding, and refuses a mismatch in either direction as
> `CommitError::Store`. Neither the batch's statements nor the checkpoint move. The server alone
> would refuse only too few values (ADR-0084).

`LivePostgresBatch::execute` carries the same paragraph, with "at this call, and the batch can then
no longer commit" in place of "at `commit`". Each escape hatch says "As for `push`" (or `execute`),
and SQLite's adds that a computed `IN (…)` list is where a count goes wrong. Neon's new `push` says
the endpoint refuses what it refuses at `commit`, as `CommitError::Store`, and that whether it
refuses a surplus is unmeasured.

The same change corrects the stale sentence on `SqliteBatch::push_raw_sql` (§6). It names
`probe_write` and `probe_delete_all` as the pattern "through `push`", not as callers of
`push_raw_sql`.

### A.4 Neon N2: the CHANGELOG entry

The entry goes under `## [Unreleased]` → `### Changed`:

```markdown
- **BREAKING (`happenstance-neon`, `projection-store` feature): `NeonWriteBatch::push`
  takes `&'static str` and its values, the free-form spelling moved to
  `push_raw_sql`, and `statements` is private.** A statement written in source goes
  through `push(sql, params)`; one whose shape is computed goes through
  `push_raw_sql(SqlStatement)`, whose name is the warning. `#[non_exhaustive]` does
  not stop `batch.statements.push(…)` on a batch the caller holds, so the field had
  to go private for the narrowing to mean anything; read it through `statements()`.
  To migrate: `batch.push(SqlStatement::with_params("…", v))` becomes
  `batch.push("…", v)`; a `format!`-built statement becomes
  `batch.push_raw_sql(SqlStatement::with_params(format!(…), v))`; `batch.statements`
  becomes `batch.statements()`. `SqlStatement` is unchanged
  ([ADR-0084](.kb/decisions/0084-the-projection-batch-sql-seam-is-final.md)).
```
