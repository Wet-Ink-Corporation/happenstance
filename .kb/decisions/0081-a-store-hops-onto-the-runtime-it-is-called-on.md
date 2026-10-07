---
id: kb-decision-0081
title: A store hops onto the runtime it is called on, and the handle it captured is the fallback
kind: decision
status: proposed
authority_tier: decision
adr_id: ADR-0081
reversibility: medium
phase: 17
supersedes: null
superseded_by: null
summary: >-
  ADR-0022 §9's reproduction, which kb-decision-0068 assigned to phase 17, and the remedy it calls
  for. Both happenstance-sqlite stores and PostgresEventStore capture a tokio Handle at
  construction and preferred it at every use. A store built on runtime A and driven from runtime B
  after A was dropped sent its work to A, whose blocking pool and task list had shut down, so tokio
  shut each task down unrun and the store reported Worker(JoinError::Cancelled): the SQLite read,
  every SQLite projection-store method, and every Postgres operation. A Postgres store whose
  capturing runtime was alive but undriven hung until A was dropped. NoRuntime was never reported,
  so ADR-0068's falsifier did not fire and a remedy is owed. Remedy B, chosen: at use time prefer
  Handle::try_current and fall back to the captured handle, at four sites, with no signature
  change. NoRuntime keeps its meaning and the concurrency families, whose contenders are bare
  threads, stay green. It is behavioural on two published crates, so it ships only in 0.4.0.
  Remedy A (new constructors) is additive and deferred; C (a variant for a dead handle) is rejected
  because tokio cannot say a Handle's runtime is gone; D (document only) is not permitted by
  ADR-0068. What B does not fix: a pooled sqlx 0.8.6 connection opened on a dropped runtime is not
  seen as broken and ends in PoolTimedOut or an unbounded hang, so a pool must be opened on a
  runtime that lives as long as the pool. Proposed; on acceptance it supersedes ADR-0022 §9's
  "prefer it" in part, and supersedes stays null.
depends_on:
  - kb-decision-0022
  - kb-decision-0068
related:
  - kb-decision-0022
  - kb-decision-0068
  - kb-decision-0058
  - kb-decision-0065
  - kb-decision-0079
  - kb-open-question-adr-0022-falsifiers-fired-001
source_paths:
  - crates/happenstance-sqlite/src/event_store.rs
  - crates/happenstance-sqlite/src/projection_store.rs
  - crates/happenstance-sqlite/tests/runtime_seam.rs
  - crates/happenstance-postgres/src/event_store.rs
  - crates/happenstance-postgres/src/read_stream.rs
  - crates/happenstance-postgres/tests/runtime_seam.rs
  - spec/SPECIFICATION.md
  - .github/workflows/ci.yml
  - references/adr/0081-a-store-hops-onto-the-runtime-it-is-called-on.md
last_reviewed: 2026-10-07
---

# A store hops onto the runtime it is called on, and the handle it captured is the fallback

The full record, with the tokio mechanism read from source, every case's before and after, and the
pool measurements, is
[`references/adr/0081-a-store-hops-onto-the-runtime-it-is-called-on.md`](../../references/adr/0081-a-store-hops-onto-the-runtime-it-is-called-on.md).

**Status: proposed.** It changes the behaviour of `happenstance-sqlite` and `happenstance-postgres`,
both published, and the owner decides. Until then `kb-decision-0022` is untouched.

## The question

ADR-0022 §9 had the stores capture a `tokio::runtime::Handle` at construction and **prefer** it,
so that the concurrency family's bare OS threads, which have no runtime of their own, still had one
to hop onto. A `Handle` is a cheap reference to a runtime that does not keep it alive. ADR-0068
asked whether a store that outlives the runtime it was built in is stranded, and assigned the
reproduction to phase 17, with a falsifier: if the stranded read reports `NoRuntime`, §9 stands
(`references/adr/0068-adr-0022-sections-8-9-16-settled.md:331-332`).

## What the reproduction found

A store built inside runtime A, A dropped, the store driven from runtime B:

- **`happenstance-sqlite`, event store.** `append` and `head` succeed (they run inline, ADR-0058).
  `read` yields one `Err(Worker(JoinError::Cancelled))` and ends.
- **`happenstance-sqlite`, projection store.** The first call, `checkpoint`, fails the same way.
  Every SQL-touching method shares the seam, so the whole store is unusable.
- **`happenstance-postgres`, event store.** Every operation, `append` included, fails as
  `Worker(JoinError::Cancelled)`. With A alive but undriven, a read from B **hangs** until A is
  dropped.

The mechanism, from tokio 1.53.1's source: dropping a `Runtime` shuts down its blocking pool and
closes its task list. A later `spawn_blocking` or `spawn` on a handle to it shuts the task down
without running it and still returns a `JoinHandle`, which completes at once with
`JoinError::Cancelled`. `NoRuntime` was never reported, so ADR-0068's falsifier did not fire.

## Decision

1. **Remedy B.** At use time prefer the runtime the caller is executing on
   (`Handle::try_current()`), and fall back to the handle captured at construction. Four sites change
   order: the SQLite read stream, `SqliteProjectionStore::runtime`, `PostgresEventStore::runtime`
   and the Postgres read stream. No signature, type or variant changes.
2. **`NoRuntime` keeps ADR-0022's meaning**: no runtime at construction and none at use. The
   capture stays for the bare-thread contenders, and the concurrency families stay green.
3. **It is behavioural**, by ADR-0068's own classification (`0068:253-263`), so it ships in
   `0.4.0` only, with a CHANGELOG entry and a hand row in the `0.4.0` trace table. No semver tool
   sees it.
4. **On acceptance it supersedes ADR-0022 §9 in part**: the words *"prefer it"*
   (`references/adr/0022-append-condition-strategy.md:389-390`). The capture, the fallback and
   `NoRuntime`'s fate stand. `supersedes` stays `null` and `kb-decision-0022` stays `accepted`, the
   ADR-0065 and ADR-0068 shape.

## Considered and not taken

- **A. New constructors taking an explicit `Handle`.** Additive (`kb-decision-0058`), so it can
  land after 1.0. It leaves the default trap in place for the caller who does not know of it.
- **C. Report a dead handle as `NoRuntime` or a new variant.** tokio exposes no "is shut down"
  query on a `Handle`; the only signal is the cancelled `JoinError` itself, after the fact.
- **D. Document only.** ADR-0068 permits it only if the outcome was `NoRuntime`. It was not.

## What remains

**The Postgres pool strand.** A pooled `sqlx` connection's socket belongs to the I/O driver of the
runtime that opened it. After that runtime is dropped, `sqlx` 0.8.6 does not see the connection as
broken, and a query on it ends in `PoolTimedOut` or hangs without bound. Remedy B cannot reach it:
it is the pool's, not the store's. **Obligation: a `PgPool` handed to a store must be opened on a
runtime that lives at least as long as the pool.** `PostgresEventStore::new`'s rustdoc and the
README carry it, with a corollary read from `sqlx`'s source: a pool opens connections lazily on the
runtime a call runs on, so under B the runtimes a store is called on should outlive the pool too.
**Owner question:** is that documented obligation enough, or is more owed?

**The calling runtime needs tokio's timer**, on Postgres, because `sqlx` acquires every connection
under `tokio::time::timeout`. Measured by `a_call_from_a_runtime_without_drivers_fails_as_a_worker_panic`:
from a runtime built without `enable_all`, `head`, `append` and `read` each fail as
`Worker(JoinError::Panic)`; before B the same `head` succeeded on the captured runtime. The I/O
driver is needed only when a call opens a new connection. The rustdoc and README say so.

A store whose captured runtime is dead, driven from a bare thread with no runtime at all, still
reports `Worker(JoinError::Cancelled)`; there is no runtime anywhere that could run the work.

## Falsifiers

- Any of `a_read_from_the_next_runtime_sees_the_log`,
  `append_and_head_from_the_next_runtime_and_a_read_sees_both`,
  `checkpoint_and_commit_from_the_next_runtime_succeed`,
  `a_handle_captured_on_a_dropped_runtime_strands_nothing` or
  `a_read_from_b_does_not_wait_on_an_undriven_capturing_runtime` going red.
- `NoRuntime` becoming unreachable: the `NoRuntime` controls in `tests/runtime_seam.rs`,
  `tests/read.rs` and `tests/concurrency.rs` go red.
- `a_pool_connection_opened_on_a_dropped_runtime_does_not_serve_the_next` succeeding: the pool
  strand has gone and the obligation above is reconsidered.
- `a_call_from_a_runtime_without_drivers_fails_as_a_worker_panic` going red: the work left the
  calling runtime, or `sqlx` stopped needing tokio's timer.
