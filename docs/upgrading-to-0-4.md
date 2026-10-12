# Upgrading from 0.3 to 0.4

> **Answers:** `how-to` — How do I move my code from 0.3 to 0.4?

This page is for you if your project depends on happenstance 0.3 and you want to
move to 0.4. By the end your code will build against 0.4 and behave as you
intend. Some changes are caught by the compiler; a few compile unchanged but
behave differently, and those are the ones this page is most careful to point
out.

0.4 is a breaking release. Most of the changes make an error clearer or a
refusal safer: identifiers are validated, a busy database is reported as busy,
and the testkit's public names lose their `__` prefix.

## How to use this page

Work through the sections for the crates in your `Cargo.toml`, in the order
below. You can skip any section for a crate you do not use. Each change says
what broke, what to change, and why.

| If you use | Read |
| --- | --- |
| `happenstance` or `happenstance-core` | [the first section](#happenstance-core-and-happenstance) |
| any store adapter | [every adapter you use](#every-adapter-you-use) |
| `happenstance-testkit` (you wrote an adapter) | [`happenstance-testkit`](#happenstance-testkit) |
| a specific adapter | its own section, near the end |

Each heading ends with a bracketed id, such as **[H8]**. That is the row for
this change in the `0.4.0` trace table, at the end of the
[`CHANGELOG.md`](../CHANGELOG.md#040--2026-10-11) section for this release. The
table lists every breaking change with the decision behind it, if you want the
full reasoning.

About the code on this page: blocks tagged `rust` are compiled and run against
this release by the project's build, so they show the 0.4 spelling. Blocks
tagged `text` are not compiled. Each one says whether it is the old 0.3
spelling, which no longer builds, or code for an adapter this page cannot link.

## `happenstance-core` and `happenstance`

### `ProjectionId::new` is fallible **[H8, H9]**

**What changed.** A projection's id is now validated, so a bad one is caught
when you create it rather than later, when a checkpoint is stored under it
([VT-35](../spec/SPECIFICATION.md#vt-35--projectionid-is-validated-vt-14s-rules-a-byte-bound-and-a-reserved-namespace)).
`ProjectionId::new` now returns `Result<ProjectionId, InvalidProjectionId>`, and
there is no longer a conversion from a string that cannot fail. The reasoning is
in [ADR-0082](../.kb/decisions/0082-projection-id-is-validated-and-sync-is-reserved.md).

In 0.3, not compiled:

```text
let id = ProjectionId::new("course_capacity");
```

**What to change.** Where the id is computed at runtime, add `?`. Where it is a
fixed string, make it a free-standing `const` built with `from_static`, so the
compiler checks it when it builds your code:

```rust
use happenstance::{InvalidProjectionId, ProjectionId};

const COURSE_CAPACITY: ProjectionId = ProjectionId::from_static("course_capacity");

fn per_tenant(tenant: &str) -> Result<ProjectionId, InvalidProjectionId> {
    ProjectionId::new(format!("course_capacity.{tenant}"))
}

fn main() -> Result<(), InvalidProjectionId> {
    assert_eq!(COURSE_CAPACITY.as_str(), "course_capacity");
    assert_eq!(per_tenant("north")?.as_str(), "course_capacity.north");
    assert!(ProjectionId::new("sync/mine").is_err());
    Ok(())
}
```

Make it a free-standing `const`, not an associated `const` inside an `impl`
block. Rust only evaluates an associated `const` when something uses it, so an
invalid one that nothing reads would never be checked. `from_static`'s own
documentation explains this in full.

**If a 0.3 checkpoint was stored under an id that is now invalid**, rename it
in SQL before you upgrade. Otherwise the projection will not find its
checkpoint, and will start again from `NeverRun`, rebuilding from the start of
the log:

```text
UPDATE projection_checkpoint SET projection_id = '<new>' WHERE projection_id = '<old>';
```

On Neon, use the table name your `NeonConfig` names.

### `CommandError::Exhausted` carries an `AppendError` **[H7]**

**What changed.** `CommandError::Exhausted` is what `commit` returns when it ran
out of retries. Its `source` is now the last attempt's whole `AppendError<E>`,
where before it was always a `ConditionViolated`. That is because a busy store
can now exhaust the retries too (see the next section).

**What to change.** A pattern that names only `attempts` and `..` is
unaffected. If you read `source.conflicting_position`, match the violation
first:

```rust
use happenstance::{AppendError, CommandError, ConditionViolated, SequencePosition};

fn hot_spot<E, D>(error: &CommandError<E, D>) -> Option<SequencePosition>
where
    E: core::error::Error + 'static,
    D: core::error::Error + 'static,
{
    match error {
        CommandError::Exhausted {
            source: AppendError::ConditionViolated(violation),
            ..
        } => violation.conflicting_position,
        _ => None,
    }
}

fn main() {
    let at = SequencePosition::FIRST;
    let hot: CommandError<std::io::Error, std::io::Error> = CommandError::Exhausted {
        attempts: 3,
        source: AppendError::ConditionViolated(ConditionViolated::at(at)),
    };
    assert_eq!(hot_spot(&hot), Some(at));

    let busy: CommandError<std::io::Error, std::io::Error> = CommandError::Exhausted {
        attempts: 3,
        source: AppendError::Busy(std::io::Error::other("database is locked")),
    };
    assert_eq!(hot_spot(&busy), None);
}
```

### `commit` and `commit_with` retry a busy store **[H5]**

**What changed.** When a database is too busy to take a write, an append is now
refused as `AppendError::Busy`, which means nothing was written. In 0.3, `commit`
returned that refusal to you at once as `CommandError::Append`. In 0.4 it
retries it inside the same `Retry` limit it already uses for a violated
condition
([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).
The decision is [ADR-0077](../.kb/decisions/0077-appenderror-busy.md).

If the store stays busy past that limit, you now get `CommandError::Exhausted`
whose `source.is_busy()` is `true`. A busy store never arrives as
`CommandError::Append` any more.

**What to change.** This one compiles unchanged, so check for it by hand. If you
wrapped `commit` in your own loop that caught a busy error inside
`CommandError::Append`, that arm no longer fires. Delete it, and either raise the
`Retry` limit you pass or handle `Exhausted`.

If your own loop retries `Exhausted`, each of its retries now runs `commit`'s
full set of retries, so the two multiply. Count both against one budget.

### `happenstance-core`'s `unstable-projection` feature is gone **[H1]**

**What changed.** The feature has turned nothing on since 0.3.0, and has now been
removed. Cargo refuses to resolve a feature that does not exist.

**What to change.** Delete it from your `Cargo.toml`. `happenstance`'s own
`unstable-projection` feature, which gates the typed projection runner, is
unchanged and stays.

## Every adapter you use

### A busy refusal is `AppendError::Busy` **[H4]**

**What changed.** `happenstance-sqlite`, `happenstance-postgres` and
`happenstance-neon` now report a refusal that provably wrote nothing as
`AppendError::Busy`. In 0.3 it was `AppendError::Store`
([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).

**What to change.** This also compiles unchanged and quietly stops matching. Code
that looked for the adapter's own busy error inside `Store` no longer sees it.
Ask `is_busy()` instead, which is the same question for every adapter:

```rust
use happenstance::AppendError;

fn worth_deciding_again<E>(error: &AppendError<E>) -> bool {
    matches!(error, AppendError::ConditionViolated(_)) || error.is_busy()
}

fn main() {
    let busy = AppendError::Busy(std::io::Error::other("database is locked"));
    let lost = AppendError::Store(std::io::Error::other("connection reset"));
    assert!(worth_deciding_again(&busy));
    assert!(!worth_deciding_again(&lost));
}
```

`happenstance-cloudflare` never reports `Busy`, and its documentation says why.

## `happenstance-testkit`

This section matters only if you wrote a store adapter and run the conformance
suite against it.

### The conformance emitters are public, without `__` **[H2]**

**What changed.** The emitters, the functions that wrap each conformance rule
into a test for your runtime, are now a public, stable API. Their names lost the
`__` prefix that marked them as hidden.

**What to change.** Rename every `emit =` argument. There are no aliases for the
old names:

| 0.3 | 0.4 |
|---|---|
| `__emit_tokio`, `__emit_blocking`, `__emit_wasm` | `emit_tokio`, `emit_blocking`, `emit_wasm` |
| `__emit_projection_tokio`, `_blocking`, `_wasm` | `emit_projection_tokio`, `_blocking`, `_wasm` |
| `__emit_model_tokio`, `_blocking` | `emit_model_tokio`, `_blocking` |
| `__emit_concurrency_tokio`, `_blocking` | `emit_concurrency_tokio`, `_blocking` |
| `__emit_rule_names` | `__rule_names`, still hidden and not promised |

An invocation that names no emitter gets the tokio default, as before, and needs
no change.

### What the suite accepts **[T6, H10, H13]**

Four changes to the rules themselves. Check each against your adapter and your
CI configuration:

- **A renamed rule.** `k_disjoint_boundaries_admit_exactly_k_commits` is now
  `k_disjoint_boundaries_never_conflict`. Update any test filter or CI step that
  names it.
- **Busy is now accepted under contention.** The five rules that race writers
  against each other accept a contender refused as `Busy`, within fixed minimums
  on how many must succeed. An adapter that failed only because it reported
  contention as `Store` should now report it as `Busy`.
- **A new projection rule.** `projection_ids_round_trip_by_bytes` fails an
  adapter that stores a checkpoint's id in a way that loses bytes (PS-39, in
  [§4.7](../spec/SPECIFICATION.md#47-checkpoint-semantics)).
- **A new concurrency rule.** `a_busy_append_left_nothing_behind` runs under
  every `event_store_concurrency_conformance!` invocation, with no change needed
  to the invocation. An adapter that answers `Busy` after its batch was in fact
  written now fails it
  ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).

## `happenstance-sqlite`

- **The busy timeout is 15 s, was 5 s [H6].** SQLite waits this long for a lock
  before giving up. Nothing to change, but a genuinely stuck writer now takes
  15 s to report.
- **A store runs on the runtime it is called on [H11].** In 0.3, a store that
  outlived the async runtime it was built in failed `read` and every
  `SqliteProjectionStore` method with `JoinError::Cancelled`. In 0.4 those calls
  work. `append` and `head` were never affected. If you relied on a store's work
  staying on its original runtime while calling it from another, it no longer
  does.

## `happenstance-postgres`

- **Build the calling runtime with `enable_all` [H11].** The store now runs on
  the runtime it is called on, and `sqlx` needs that runtime's timer and I/O
  drivers. A runtime without a timer fails as `Worker(JoinError::Panic)`. Open a
  `PgPool`, and call the stores, on runtimes that live at least as long as the
  pool.
- **The `naive-arm` feature is gone [T4, T5].** If your `Cargo.toml` names it,
  delete it. `PostgresEventStore::new_naive` is now a deliberately broken
  reference used by the crate's own tests, behind `--cfg happenstance_naive_arm`,
  and it is not part of the API.
- **A projection statement's values must match its `$n` [H12].** The store
  counts the highest `$n` placeholder in each statement's text, and refuses a
  mismatch in either direction before anything is sent, as
  `PostgresProjectionStoreError::ParameterCount`. In 0.3, extra values were
  silently committed on a fresh connection.

  If you passed more values than the text uses, drop the extra ones. On
  `LivePostgresProjectionStore`, a refused `execute` poisons the batch:
  everything after it, `commit` included, answers `Poisoned`. Roll back and start
  a new batch.

## `happenstance-neon`

- **`ProbeThenWriteStore` is gone [T8].** It was a store built to lose updates on
  purpose, for this crate's own tests, and it is now private to them. If you
  named it, use `NeonEventStore`.

### `NeonWriteBatch::push` takes the statement and its values **[T1–T3]**

**What changed.** `push` now takes the SQL text and its values separately, and
the batch's statements are no longer a public field. A statement whose text is
built at runtime goes through `push_raw_sql`, whose name is a reminder to check
that text is safe to send.

Here are 0.3 and 0.4 side by side. Neither is compiled, because this page does
not link the Neon adapter:

```text
// 0.3
batch.push(SqlStatement::with_params("UPDATE stock SET n = $1", fixed));
batch.push(SqlStatement::with_params(format!("UPDATE {table} SET n = $1"), computed));
let all = batch.statements.clone();

// 0.4
batch.push("UPDATE stock SET n = $1", fixed);
batch.push_raw_sql(SqlStatement::with_params(format!("UPDATE {table} SET n = $1"), computed));
let all = batch.statements().to_vec();
```

**What to change.** A statement written as a literal in your source goes through
`push`. One whose text is computed goes through `push_raw_sql`. A write through
the old public field also becomes `push_raw_sql`, and a read of it becomes
`statements()`.

### A custom `SqlTransport` implements `reads_settled` **[T7]**

This applies only if you wrote your own `SqlTransport`, the type that carries
Neon's HTTP requests.

**What changed.** `SqlTransport` has a second required method, `reads_settled`,
and `NeonEventStore::append` waits on it before its first attempt. Waiting for
earlier reads to finish is how the adapter guarantees that a read sees one
consistent snapshot of the log
([ES-11](../spec/SPECIFICATION.md#es-11--a-read-is-a-snapshot),
[ES-12](../spec/SPECIFICATION.md#es-12--all-items-of-one-query-share-one-snapshot)).
The decision is
[ADR-0087](../.kb/decisions/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md).

**What to change.** In your transport:

1. Hold a `ReadLedger`.
2. Call `dispatch()` for every request whose `SqlRequest::read_only` is set,
   before `round_trip` returns.
3. Move the `ReadTicket` it gives you to whatever observes the answer.
4. Return `self.ledger.settled()` from `reads_settled`.

A transport that never dispatches anything returns `core::future::ready(())`.
The trait's documentation states the obligation in full, including why
settlement must never be driven by polling `round_trip`.

`SqlTransport::round_trip` also now states, under `# Retries`, that a transport
must not silently re-send a request that may already have reached the endpoint.
Re-sending could apply a write twice. A transport that loses a response must
return its error instead of retrying.

## `happenstance-cloudflare`

- **`planned_statement_count` returns different numbers [H3].** A real Durable
  Object refuses a statement with more than five compound terms, so a query of
  *n* items now takes `n.div_ceil(5)` statements. If you asserted on the old
  values, update the expectation. A query of five items or fewer is still one
  statement.
- **The wasm32 test shim needs Node 24.** It enforces `workerd`'s statement
  limits through `node:sqlite`'s `limits` option, which Node 22 does not have.
  This affects only running the adapter's own tests, not your application.
