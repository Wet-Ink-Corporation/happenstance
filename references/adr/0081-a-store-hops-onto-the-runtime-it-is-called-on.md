# ADR-0081 — A store hops onto the runtime it is called on, and the handle it captured is the fallback

- **Status:** proposed. The owner decides; the pull request carrying it stays open until then.
- **Date:** 2026-10-07
- **Phase:** 17 (the breaking window), lane L9.
- **Answers:** ADR-0022 §9's reproduction, which
  [ADR-0068](../../.kb/decisions/0068-adr-0022-sections-8-9-16-settled.md) assigned to phase 17
  (`references/adr/0068-adr-0022-sections-8-9-16-settled.md:221-273`), and the work item and exit
  criterion at `runbook/phases/17-breaking-window.md:105-116` and `:267-269`.
- **Supersedes, on acceptance:** ADR-0022 §9 in part — the words *"prefer it"* in its chosen option
  (`references/adr/0022-append-condition-strategy.md:389-390`), and nothing else in §9. The capture,
  the fallback and the fate of `NoRuntime` (`0022:392-412`) stand. This is the ADR-0065 and
  ADR-0068 shape: a section-grain supersession, so `supersedes` stays `null` and
  `kb-decision-0022` stays `accepted`. **While this record is proposed, ADR-0022's atom and its row
  in the decision map are not touched.**
- **Evidence:** `crates/happenstance-sqlite/tests/runtime_seam.rs`,
  `crates/happenstance-postgres/tests/runtime_seam.rs`, and the red and green runs recorded in
  `.temper/plans/p17-l9.md` (Progress). tokio 1.53.1 and `sqlx` 0.8.6, from `Cargo.lock`.

---

## 1. The question, for a reader new to tokio

**What a `Handle` is.** A tokio `Runtime` owns three things: a scheduler that polls async tasks, a
pool of threads for blocking work, and an I/O driver that wakes a task when its socket becomes
readable. A `tokio::runtime::Handle` is a cheap, cloneable reference to one runtime — internally
an `Arc` — through which code can spawn onto it: `handle.spawn(future)` queues a task on its
scheduler, and `handle.spawn_blocking(closure)` hands a closure to its blocking pool. A `Handle`
does **not** keep the runtime alive. Dropping the `Runtime` value shuts the runtime down, and every
`Handle` cloned from it goes on existing, pointing at a runtime that no longer runs anything.
`Handle::try_current()` asks a different question: *which runtime is the code calling me executing
on?* It succeeds inside a task or a `block_on`, and fails on a bare OS thread.

**Why capture-at-construction strands.** ADR-0022 §9 had both stores in `happenstance-sqlite`, and
later `PostgresEventStore`, call `Handle::try_current().ok()` in their constructor and keep the
result (`crates/happenstance-sqlite/src/event_store.rs:513`,
`crates/happenstance-sqlite/src/projection_store.rs:234`,
`crates/happenstance-postgres/src/event_store.rs:337`), then **prefer** that handle at every use.
The reason was sound: the conformance suite's concurrency family drives stores from bare OS threads
with no runtime of their own, and the captured handle is the only one they can reach. But a
captured handle names the runtime that was current *when the store was built*, not the one that is
current when it is used. Build a store inside runtime A, drop A, and call it from runtime B: the
store still sends its work to A. The realistic shape is a `static STORE: OnceLock<_>` initialised
inside the first `#[tokio::test]` — each test owns its own runtime, so every later test reads
through a dead handle. Nothing in the type system sees this: a `Handle` to a dead runtime has the
same type as a live one, and tokio exposes no query that tells them apart.

**Why `spawn_blocking` on a shut-down pool yields `JoinError::Cancelled`.** Dropping a `Runtime`
drops its `BlockingPool`, whose `Drop` calls `shutdown`
(`tokio-1.53.1/src/runtime/blocking/pool.rs:282-286`), and `shutdown` sets the pool's `shutdown`
flag (`:254`). A later `handle.spawn_blocking(f)` still builds a task and a `JoinHandle` for it,
and then `spawn_task` finds the flag set, **shuts the task down without running it**, and returns
`SpawnError::ShuttingDown` (`:396-403`). `spawn_blocking` does not panic on that error; it hands
back the `JoinHandle` anyway, under a comment saying it *"will never resolve"* (`:322-323`). The
comment is stale: the task was already shut down, so awaiting the handle completes at once with
`Err(JoinError::Cancelled)`. `Handle::spawn` behaves the same way — `OwnedTasks::bind` checks the
list's `closed` flag and shuts the task down (`tokio-1.53.1/src/runtime/task/list.rs:139-143`). So a
stranded call does not hang and does not panic: the closure never runs, and the store maps the
`JoinError` to its `Worker` variant —
`SqliteEventStoreError::Worker` (`crates/happenstance-sqlite/src/event_store.rs:1577-1579`; ADR-0068
cites it at a stale `:1218-1220`), `SqliteProjectionStoreError::Worker`
(`crates/happenstance-sqlite/src/projection_store.rs:552-558`) and `PostgresEventStoreError::Worker`
(`crates/happenstance-postgres/src/error.rs:100-106`).

**The one way it does hang.** If A is alive but nothing drives it — a `current_thread` runtime
whose `block_on` has returned — `Handle::spawn` queues the task on A's scheduler and nobody polls
it. `spawn_blocking` is immune, because A's blocking threads run whether or not A is driven, so
only Postgres, whose every operation goes through `spawn`, can hang this way.

## 2. The reproduction

Both files are plain `#[test]`s that build their runtimes by hand, because dropping a `Runtime`
inside `#[tokio::test]` panics. In every case the store is constructed **inside** A (so the handle
under test is a captured one, not the `None` the fallback tests already cover), seeded on A (so a
stranded call ending as an empty `Ok` fails rather than looking like an empty log), and driven from
a **different** runtime B (a multi-thread one, so B cannot merely resemble A). The Postgres file is
`#[ignore]`d with a reason and runs in CI's `live-postgres` job (`.github/workflows/ci.yml:590-627`),
against a live `postgres:17.10` through testcontainers; its hang legs are bounded at 15 s.

"Before" is the base `6235224`; "after" is this change (remedy B, §4).

| # | Test (`file:line`) | Before | After |
| --- | --- | --- | --- |
| 1 | SQLite event store, read from B after A dropped: `a_read_from_the_next_runtime_sees_the_log` (`happenstance-sqlite/tests/runtime_seam.rs:96`) | `collect` returned `Err(Worker(JoinError::Cancelled))` — the arm at `:108` | the seeded event, alone |
| 2 | SQLite event store, append, head and read from B: `append_and_head_from_the_next_runtime_and_a_read_sees_both` (`:125`) | `append` and `head` succeeded (inline, ADR-0058); the read failed as `Worker(JoinError::Cancelled)` | both events |
| 3 | SQLite projection store, from B after A dropped: `checkpoint_and_commit_from_the_next_runtime_succeed` (`:189`) | the first call, `checkpoint`, failed as `Worker(JoinError::Cancelled)` — the arm at `:202`; every SQL-touching method shares the seam, so the whole store was unusable | checkpoint `Live { through: 3 }`, then a commit to 7 lands |
| 4 | SQLite projection store, no runtime anywhere: `a_projection_store_with_no_runtime_anywhere_reports_no_runtime` (`:232`) | `NoRuntime` (a control, not a red test) | `NoRuntime` |
| 5 | Postgres, pool opened on B, store constructed on A, A dropped: `a_handle_captured_on_a_dropped_runtime_strands_nothing` (`happenstance-postgres/tests/runtime_seam.rs:205`) | the first call, `append`, failed as `AppendError::Store(Worker(JoinError::Cancelled))` — the arm at `:216` | `append`, `head` and `read` all succeed |
| 6 | Postgres, A alive and undriven, pool on B: `a_read_from_b_does_not_wait_on_an_undriven_capturing_runtime` (`:240`) | **hung for the whole 15 s bound**; once A was dropped it yielded `Err(Worker(JoinError::Cancelled))` — the arm at `:260` | the seeded event, promptly |
| 7 | Postgres, raw `sqlx`, no store — a pooled connection opened on A, A dropped, a query from B: `a_pool_connection_opened_on_a_dropped_runtime_does_not_serve_the_next` (`:287`) | `PoolTimedOut` or `Elapsed` (passes) | the same — remedy B cannot reach it (§6) |
| 8 | Postgres, pool, store and seed all on A, A dropped: `a_store_built_wholly_on_a_dropped_runtime_is_stranded_by_its_pool_not_its_handle` (`:337`) | `head` from B failed as `Worker(JoinError::Cancelled)` — the arm at `:356` | the store's work runs on B; `head` then meets case 7's strand and ends on its arm in about 5 s, the fixture's `acquire_timeout`, so `PoolTimedOut` by its timing (the test accepts that or `Elapsed`) |
| 9 | Postgres, pool, store and seed on a full runtime that stays alive, called from a `current_thread` runtime built without `enable_all`: `a_call_from_a_runtime_without_drivers_fails_as_a_worker_panic` (`:404`) | `head` succeeded, on the captured runtime (run with the two Postgres sites put back captured-first) | `head`, `append` and `read` each fail as `Worker(JoinError::Panic)`, inside `sqlx`'s `tokio::time::timeout` (§6) |

The SQLite target ran in 0.13 s and never hung. The Postgres target ran twice red with the same
outcomes, and three times green after the change. Case 9 came from the change's review: it ran red
once with the two Postgres sites put back captured-first, and green under B. Under remedy B the concurrency families stay
green — `happenstance-sqlite`'s `concurrency` 10/10 and `postgres_conformance` 108/108 including the
five concurrency rules — because their contenders are bare threads, where `try_current` fails and
the captured fallback is what they use.

## 3. ADR-0068's falsifier did not fire

ADR-0068 §11 named the condition under which the hazard is not real: *"if the reproduction shows the
stranded read reporting `NoRuntime` after all … §9 is ratified as written, and the assignment closes
with nothing to do"* (`0068:331-332`). The stranded read reported `Worker(JoinError::Cancelled)` on
both adapters, never `NoRuntime`. So the hazard is real, the assignment does not close with nothing
to do, and a remedy is owed.

The open question's prediction — a stranded read *"hangs or yields one cancelled-task item and
terminates"* (`.kb/open-questions/adr-0022-falsifiers-have-fired.md`) — was half right. It yields
one cancelled item and terminates, on both adapters, deterministically. It hangs only in the
Postgres-only shape it did not name: a capturing runtime that is alive and undriven (case 6).

## 4. Decision: remedy B

**At use time, prefer the runtime the caller is executing on (`Handle::try_current()`), and fall
back to the handle captured at construction only when there is none.** `NoRuntime` keeps exactly
the meaning ADR-0022 §9 gave it: no runtime at construction and none at use. The capture stays,
because the bare-thread contenders still need it.

Four sites change order, and nothing else does:

- `SqliteReadStream::poll_next`, the `Idle` arm (`crates/happenstance-sqlite/src/event_store.rs:2385`);
- `SqliteProjectionStore::runtime` (`crates/happenstance-sqlite/src/projection_store.rs:344-349`);
- `PostgresEventStore::runtime` (`crates/happenstance-postgres/src/event_store.rs:387-392`);
- `PgReadStream::poll_next`, the `Unstarted` arm (`crates/happenstance-postgres/src/read_stream.rs:386`).

No signature, type or variant moves, and no dependency is added. The precedent is already in the
tree: `open_cursor` records the executing runtime ahead of the captured one for its cursor's `Drop`
(`crates/happenstance-postgres/src/read_stream.rs:458-480`), the handle-sampling defect ADR-0068
cites at `:265-273`. `PgCursor::drop` (`read_stream.rs:246`) still prefers the runtime its cursor
recorded; that is the runtime the cursor's work ran on, which after this change is the poll's own,
so it is consistent with the decision rather than an exception to it.

ES-11's prose describing the seam (`spec/SPECIFICATION.md:3104-3109`) is reconciled line-neutrally
in the same change. ES-11 is `[PROVISIONAL]` and no `[FROZEN]` clause names the seam.

## 5. The remedies considered

**A. New constructors — additive, not taken now.** `SqliteEventStore::open_with_runtime(path,
Option<Handle>)` and its siblings, and `PostgresEventStore::with_runtime(pool, Option<Handle>)`.
They let a careful caller avoid the strand and leave the default trap exactly where it is: the
caller who does not know about the hazard calls `open`. `kb-decision-0058` already calls such a
constructor *"additive either way"* (`0058:95-97`), so it stays available after 1.0 and needs no
decision now. Not rejected — deferred, and unneeded unless someone asks to pin a runtime.

**B. Prefer the executing runtime — behavioural, chosen.** It fixes the SQLite stores completely
(cases 1–3), removes the Postgres handle strand (cases 5, 6, 8), keeps the concurrency families
green, keeps `NoRuntime` reachable (case 4, and `tests/read.rs:162`, `tests/concurrency.rs:160`),
and changes no signature.

**C. Report a dead captured handle as `NoRuntime`, or a new `RuntimeGone` variant — rejected.**
The three error enums are `#[non_exhaustive]`, so a new variant compiles, but either form needs to
*know* the captured runtime is dead, and tokio 1.53.1 exposes no "is shut down" query on a
`Handle`. The only signal is a `JoinError::Cancelled` from a task the store never aborted, which is
an inference after the fact, not a check, and would rename a `Worker` the caller can already see.
Under B the one remaining route to it is narrow (§6), so C would buy little at real cost.

**D. Document only — not permitted.** Ratifying §9 with a known limitation is what ADR-0068's
falsifier allows only when the outcome is `NoRuntime` (`0068:331-332`). It was not.

## 6. Classification, and what remains

**B is behavioural on two published crates, so it ships in `0.4.0` and nowhere else.** No signature
changes, so `cargo semver-checks` will not flag it. But it changes which runtime runs a store's work
and which variant a given situation reports, which is ADR-0068's definition of the behavioural arm
(`0068:253-263`): *"a change to … which variant a stranded read reports"*. `happenstance-sqlite` and
`happenstance-postgres` are published at `0.3.2`. Phase 17 is the breaking window, so it lands
there, with a CHANGELOG entry and a hand row in the `0.4.0` trace table, as ADR-0079's changed
values do.

**Who could observe the change.** A caller whose store's work used to run on the captured runtime
and now runs on the calling one. For SQLite that is which blocking pool runs a read; for Postgres
it is which scheduler runs every operation. A caller that relied on a store's work staying on a
dedicated runtime it built the store in, while calling from another, loses that. Remedy A is the
tool for that caller, and it is additive. One narrower caller is affected on Postgres alone: `sqlx`
acquires every connection under `crate::rt::timeout`, which is `tokio::time::timeout`
(`sqlx-core-0.8.6/src/pool/inner.rs:250`, `src/rt/mod.rs:29`), so the calling runtime now needs
tokio's timer. Measured as case 9: from a `current_thread` runtime built without `enable_all`,
`head`, `append` and `read` each fail as `Worker(JoinError::Panic)`, the panic raised at
`src/rt/mod.rs:29` (*"timers are disabled"*); with the captured handle put back first, as before B,
the same `head` succeeded. The I/O driver is the narrower need: a runtime with `enable_time` alone
succeeded, in one run, because the calls reused an idle connection
opened on the store's full runtime. It is needed when a call must open a new connection, which
`sqlx` does inside `acquire`, on the calling runtime (`inner.rs:288`) — read, not run. SQLite is
unaffected: its work is `spawn_blocking`, which needs neither driver.

**What B does not fix: the Postgres pool strand.** A pooled `sqlx` connection's socket is
registered with the I/O driver of the runtime that **opened** it. When that runtime is dropped the
socket stays a good OS socket, but the readiness events that would wake a read on it go to a
driver that no longer exists. `sqlx` 0.8.6 does not treat such a connection as broken. Measured
(case 7, `.temper/plans/p17-l9.md`):

- with `SELECT 1`: three outcomes across five runs — success (twice), `PoolTimedOut`, and an
  **unbounded** hang in the query once `test_before_acquire`'s ping had raced through (killed after
  10 minutes). A success is the race in which the reply is already in the socket buffer when the
  read is first polled;
- with `SELECT pg_sleep(0.2)`, whose reply cannot be in the buffer: six runs, five `PoolTimedOut`
  after the fixture's 5 s `acquire_timeout` and one `Elapsed` at the test's 15 s bound, no success.

So **the handle was never the only strand**, and remedy B cannot reach the second one: it is the
pool's, not the store's. Case 8 shows both in one store: before B it failed on the handle, after B
it fails on the pool. The `PostgresProjectionStore` and `LivePostgresProjectionStore` capture no
handle and await `sqlx` directly, so they never had the first strand and have the second exactly as
the event store does. `happenstance-neon` and `happenstance-cloudflare` hold no pool and capture
nothing.

**The obligation, therefore:** *a `PgPool` handed to a `happenstance-postgres` store must be opened
on a runtime that lives at least as long as the pool.* That is the application's to keep, because
the application builds the pool (`PostgresEventStore::new` takes one, `event_store.rs:300-328`).
`PostgresEventStore::new`'s rustdoc carries it now, under *Runtimes*, beside case 9's driver
requirement, and so does the crate README; the citations the rustdoc moved were repointed in the
same change. Both add a corollary, read from `sqlx`'s source and not run: a pool opens connections
lazily inside `acquire`, on the runtime the call runs on (`inner.rs:288`), so under B a pooled
connection can belong to any runtime the store was called on, and those runtimes should outlive the
pool too. Before B that runtime was the captured one.

**One narrow residue in both adapters.** A store whose captured runtime is dead and which is then
driven from a bare thread with no runtime at all still reaches the dead handle and reports
`Worker(JoinError::Cancelled)`. No runtime exists anywhere that could run the work, so there is
nothing better to do than report it; it is not `NoRuntime` only because the store cannot tell a
dead handle from a live one (remedy C's problem).

## 7. Questions for the owner

1. **Accept B for `0.4.0`?** It is the behaviour change described in §6. Rejecting it leaves the
   stranded read as it was and §9's work item open.
2. **Is a documented obligation enough for the pool strand, or is more owed?** The options in
   reach: (a) the rustdoc and README obligation above, and nothing else — recommended, because the
   fix is the application's pool and the store cannot rebuild it; (b) the same, plus a conformance
   or adapter test pinning the obligation's wording to case 7, which already exists; (c) an
   adapter-side remedy, such as setting `sqlx`'s `test_before_acquire` with a timeout on the
   caller's pool — not possible, because the pool's options are fixed when the caller builds it.
   A pool-level answer would be `sqlx`'s, upstream.

## 8. Consequences

- On acceptance: `kb-decision-0022`'s row in `.kb/maps/decision-map.md` gains
  *"partly superseded by `kb-decision-0081`"*; the atom's `status` stays `accepted`.
- On acceptance: `kb-open-question-adr-0022-falsifiers-fired-001` can close for §9, its last open
  section. Until then it is amended, not closed.
- The pool obligation and case 9's driver requirement are in `PostgresEventStore::new`'s rustdoc
  and the README in this change; if B is declined, the driver requirement leaves with it.
- The `0.4.0` trace table owes a hand row for both crates, beside ADR-0079's.
- The phase-17 exit criterion at `runbook/phases/17-breaking-window.md:267-269` is met by this
  change on acceptance: the reproduction runs against both adapters, this record classifies the
  remedy as behavioural, and the remedy has landed on the branch.

## 9. Falsifiers

- **A runtime-seam regression:** any of cases 1, 2, 3, 5 or 6 going red. Each panics naming the
  strand if the work is spawned onto the dead or undriven runtime again.
- **`NoRuntime` losing its meaning:** case 4, `read_polled_outside_a_runtime_yields_an_error_item`
  or `a_store_with_no_runtime_anywhere_reports_no_runtime` going red. A remedy that ran the work
  inline when no runtime is found — ADR-0022 §9's rejected option (b) — fails them.
- **A remedy that drops the capture** and uses `try_current` alone: the concurrency families go
  red, because their contenders are bare threads.
- **The pool strand going away:** case 7 succeeding. It panics saying the record describing the
  strand is wrong; §6's obligation would then be reconsidered against the `sqlx` version that
  changed it.
- **The driver requirement going away, or the work moving back:** case 9 going red. A call from a
  driverless runtime that succeeds means either the store's work left the calling runtime, which
  is cases 5 and 6 at risk, or `sqlx` stopped needing tokio's timer, and the rustdoc's *Runtimes*
  section is then wrong.
- **tokio adding a shut-down query on `Handle`:** remedy C would become checkable rather than
  inferred, and is worth re-reading.
