# Upgrading from 0.3 to 0.4

> **Answers:** `how-to` — How do I move my code from 0.3 to 0.4?

Work through the sections for the crates in your manifest, in the order below.
Each one names the break, says what to change, and links the
[`CHANGELOG.md`](../CHANGELOG.md#unreleased) entry that carries the reasoning and
the decision record. The row ids in brackets, such as **[H8]**, are the rows of
the `0.4.0` trace table at the end of that section of the changelog, which lists
every break this release carries with the decision that caused it.

The Rust blocks tagged `rust` below are compiled against this workspace by the
gate, so they are the 0.4 spelling. The blocks tagged `text` are not compiled.
They are either the 0.3 spelling, which no longer builds, or code for an adapter
the page's compiler does not link, and each one says which.

## `happenstance-core` and `happenstance`

### `ProjectionId::new` is fallible **[H8, H9]**

`ProjectionId::new` returns `Result<ProjectionId, InvalidProjectionId>`, because
an id is now validated
([VT-35](../spec/SPECIFICATION.md#vt-35--projectionid-is-validated-vt-14s-rules-a-byte-bound-and-a-reserved-namespace);
[ADR-0082](../.kb/decisions/0082-projection-id-is-validated-and-sync-is-reserved.md)).
There is no infallible conversion from a string any more.

In 0.3, not compiled:

```text
let id = ProjectionId::new("course_capacity");
```

In 0.4, add `?` where the id is computed, and use a free `const` built by
`from_static` where it is a literal, so the compiler validates it:

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

Make it a *free* `const`, not an associated one: `from_static`'s own
documentation explains why an associated `const` nothing reads is never checked.

**If a 0.3 checkpoint was written under an id that is now invalid**, rename it
in SQL before you upgrade, or the projection starts again from `NeverRun`:

```text
UPDATE projection_checkpoint SET projection_id = '<new>' WHERE projection_id = '<old>';
```

On Neon, name the table your `NeonConfig` names.

### `CommandError::Exhausted` carries an `AppendError` **[H7]**

The `source` of `CommandError::Exhausted` is now the last attempt's whole
`AppendError<E>`, where it was a `ConditionViolated`. A pattern that names only
`attempts` and `..` is unaffected. If you read `source.conflicting_position`,
match the violation first:

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

An append refused as `AppendError::Busy` used to come back at once as
`CommandError::Append`. It is now retried inside the same `Retry` bound as a
violated condition ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so), [ADR-0077](../.kb/decisions/0077-appenderror-busy.md)).
A busy store that outlasts the bound now arrives as `CommandError::Exhausted`
whose `source.is_busy()` is `true`, and never as `CommandError::Append`.

So if you wrapped `commit` in your own loop that caught a busy error inside
`CommandError::Append`, that arm no longer fires. Delete it, and either raise
the `Retry` bound you pass or handle `Exhausted`. If your loop retries
`Exhausted`, it now multiplies the bound you passed, so count both against one
budget.

### `happenstance-core`'s `unstable-projection` feature is gone **[H1]**

It has turned nothing on since 0.3.0. Delete it from your manifest, because
Cargo now refuses to resolve it. `happenstance`'s own `unstable-projection`,
which gates the typed projection runner, is unchanged.

## Every adapter you use

### A busy refusal is `AppendError::Busy` **[H4]**

`happenstance-sqlite`, `happenstance-postgres` and `happenstance-neon` report a
refusal that provably wrote nothing as `AppendError::Busy`, where it was
`AppendError::Store` ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)). This compiles unchanged and stops matching. A guard that
looked for the adapter's busy error inside `Store` no longer sees it. Ask
`is_busy()`, which is the same question for every adapter:

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

### The conformance emitters are public, without `__` **[H2]**

Rename every `emit =` argument. There are no aliases:

| 0.3 | 0.4 |
|---|---|
| `__emit_tokio`, `__emit_blocking`, `__emit_wasm` | `emit_tokio`, `emit_blocking`, `emit_wasm` |
| `__emit_projection_tokio`, `_blocking`, `_wasm` | `emit_projection_tokio`, `_blocking`, `_wasm` |
| `__emit_model_tokio`, `_blocking` | `emit_model_tokio`, `_blocking` |
| `__emit_concurrency_tokio`, `_blocking` | `emit_concurrency_tokio`, `_blocking` |
| `__emit_rule_names` | `__rule_names`, still hidden and not promised |

An invocation that names no emitter gets the tokio default, as before, and needs
no change.

### What the suite accepts **[T6, H10]**

- `k_disjoint_boundaries_admit_exactly_k_commits` is renamed
  `k_disjoint_boundaries_never_conflict`. Update any filter or CI step that
  names it.
- The five racing rules accept a contender refused as `Busy`, under structural
  floors. An adapter that was red only because it reported contention as
  `Store` should now report it as `Busy`.
- A new projection rule, `projection_ids_round_trip_by_bytes`, fails an adapter
  that keys a checkpoint lossily (PS-39, in
  [§4.7](../spec/SPECIFICATION.md#47-checkpoint-semantics)).
- A new concurrency rule, `a_busy_append_left_nothing_behind`, runs under every
  `event_store_concurrency_conformance!` invocation with no change to it. An
  adapter that answers `Busy` after its batch was written now fails it
  ([ES-43](../spec/SPECIFICATION.md#es-43--a-busy-refusal-wrote-nothing-and-says-so)).

## `happenstance-sqlite`

- **The busy timeout is 15 s, was 5 s [H6].** Nothing to change. A genuinely
  stuck writer now takes 15 s to report.
- **A store runs on the runtime it is called on [H11].** In a store that
  outlived the runtime it was built in, `read` and every `SqliteProjectionStore`
  method used to fail with `JoinError::Cancelled`, and now work. `append` and
  `head` were never affected. If you relied on a store's work staying on its original runtime while
  calling it from another, it no longer does.

## `happenstance-postgres`

- **Build the calling runtime with `enable_all` [H11].** The store now runs on
  the runtime it is called on, and `sqlx` needs that runtime's timer and I/O. A
  runtime without a timer fails as `Worker(JoinError::Panic)`. Open a `PgPool`,
  and call the stores, on runtimes that live at least as long as the pool.
- **The `naive-arm` feature is gone [T4, T5].** If your manifest names it,
  delete it. `PostgresEventStore::new_naive` is a negative control behind
  `--cfg happenstance_naive_arm`, and it is not API.

## `happenstance-neon`

### `NeonWriteBatch::push` takes the statement and its values **[T1–T3]**

In 0.3 and in 0.4 respectively. Neither is compiled here, because the page does
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

A statement written in source goes through `push`. One whose text is computed
goes through `push_raw_sql`, whose name is the warning. A write through the old
public field becomes `push_raw_sql`.

### A custom `SqlTransport` implements `reads_settled` **[T7]**

`SqlTransport` has a second required method, and `NeonEventStore::append` waits
on it before its first attempt. That is how the adapter meets
[ES-11](../spec/SPECIFICATION.md#es-11--a-read-is-a-snapshot) and
[ES-12](../spec/SPECIFICATION.md#es-12--all-items-of-one-query-share-one-snapshot)
([ADR-0087](../.kb/decisions/0087-es-11-is-met-on-one-shot-http-by-a-read-settlement-fence.md)).
If you wrote your own transport:

1. Hold a `ReadLedger`.
2. Call `dispatch()` for every request whose `SqlRequest::read_only` is set,
   before `round_trip` returns.
3. Move the `ReadTicket` to whatever observes the answer.
4. Return `self.ledger.settled()` from `reads_settled`.

A transport that never dispatches anything returns `core::future::ready(())`.
The trait's documentation states the obligation in full, including why
settlement must never be driven by polling `round_trip`.

`SqlTransport::round_trip` also states, under `# Retries`, that a transport must
not transparently re-send a request that may have reached the endpoint. A
transport that re-sent after a lost response must surface its error instead.

## `happenstance-cloudflare`

- **`planned_statement_count` returns different numbers [H3].** A query of *n*
  items is now `n.div_ceil(5)` statements, because a real Durable Object refuses
  more than five compound terms. If you asserted on the old values, update the
  expectation. Five items or fewer are still one statement.
- **The wasm32 test shim needs Node 24.** It enforces `workerd`'s statement
  limits through `node:sqlite`'s `limits` option, which Node 22 does not have.
  This affects only running the adapter's own tests.
