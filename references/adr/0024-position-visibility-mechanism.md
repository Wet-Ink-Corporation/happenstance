# ADR-0024: `happenstance-postgres` buys ES-10 with `xid8` + `pg_snapshot_xmin`, and the bill is read-side and structural

- **Status:** accepted
- **Date:** 2026-09-07
- **Settles:** RUNBOOK's ADR queue row 0024 — *"Postgres: how does the adapter buy
  the position-visibility invariant when `nextval()` allocates outside the
  transaction — measured, not preferred?"* — and
  `.kb/open-questions/postgres-arm-c-structural-cost.md`, whose four ordered
  sub-questions this record answers in order.
- **Rests on and does not settle:** [ADR-0013](0013-position-assignment-and-visibility.md),
  which lifted ES-10 to `[FROZEN]` and chose *nothing* about the adapter — it says
  so itself: *"This ADR needs one affordable mechanism to exist; it does not
  choose the adapter's."* This record chooses the adapter's.
- **Inherits without owning:** ES-10's **global** framing. See §6.
- **Does not amend:** `spec/SPECIFICATION.md`. ES-10 is untouched and still
  `[FROZEN]`.

## Context

`nextval()` allocates outside the transaction. A writer takes 99, a writer that
started later takes 100 and commits first, and a reader that has already observed
100 later sees 99 appear beneath it. A caller conditioned on `after: 100` never
saw 99, `is_violated_by` compares it against the boundary and reports no
violation, and the consistency boundary stops enforcing with no error anywhere.

That is ES-10's `Rejects:` clause naming this adapter by construction, and it is
why `happenstance-postgres` is in the workspace at all: every other store in the
portfolio assigns positions under a lock it holds until commit, so all of them
satisfy ES-10 for free and none of them votes for it.

Phase 2 measured four candidate mechanisms as SQL scripts and found arm C —
`xid8` + `pg_snapshot_xmin` — affordable. ADR-0013 took that as evidence that
*an* affordable mechanism exists, and deliberately left the adapter's choice open.
Phase 10 is where the adapter exists.

## Decision

### 1. The mechanism is arm C: `xid8` + `pg_snapshot_xmin`

Each row carries `pg_current_xact_id()` in an `xid8` column, written by the
appending transaction. Every read and `head()` admit only rows beneath
`pg_snapshot_xmin(pg_current_snapshot())` — the id below which no transaction can
still be in flight.

`position` keeps **no column default**. The sequence is real and is read
explicitly by the one `INSERT` that allocates, which keeps `information_schema`
reporting `column_default IS NULL` and `is_identity = 'NO'`: "position is not
`bigserial`" stays a fact a test can ask about rather than a turn of phrase.

Arm C does not deny that `nextval()` allocates outside the transaction. It stops
treating position order as **visibility** order and moves the guard to the read
side.

### 2. Steady-state cost: not measurable, and the precision is reported

`experiments/position-visibility/results/adapter-remeasurement.md` re-measures
against the **built** adapter — pool, transaction tied to `append`'s async
boundary, cursor, error mapping — with the naive arm (the same store, predicate
removed) as the paired baseline.

| Clients | arm C ratio, min | **median** | max |
|---|---|---|---|
| 1 | 0.837 | **0.985** | 1.219 |
| 8 | 0.865 | **1.131** | 1.190 |
| 32 | 0.863 | **1.026** | 1.118 |

All three medians straddle 1.0 with about ±20% spread, against an effect phase 2
measured at 0.99–1.03. **This measurement cannot resolve the cost**, and the
record says so rather than quoting a point estimate from it. Baseline drift within
a triple ran 1.01–1.88 on Docker Desktop's virtual disk; §3 of the results page
lists the four things tried and what each bought.

What it does establish: no cost large enough to matter hides inside ±20%. The two
arms that cost 16× and 30× would be unmissable here.

**This is deliberately not the phase-2 SQL-script figure**, which AC-001 forbids
relying on. It is a weaker statement taken against the right thing, rather than a
stronger statement taken against the wrong one.

### 3. The bill is read-side, and it is structural rather than incremental

Four gaps the phase-2 harness never had, each with what it actually cost.

**Connection pooling.** `sqlx` requires a tokio runtime in thread-local scope,
and the conformance suite's concurrency contenders run on raw OS threads that
have none — deliberately, because CF-20 and CF-23 mean the testkit must not take
a runtime dependency. The store therefore captures a `Handle` at construction and
runs every operation through it. `Handle::enter` is the smaller change and is
wrong: its `EnterGuard` is `!Send`, so holding one across an await makes the
future `!Send` and `SendEventStore` stops being implementable — ADR-0001's
constraint arriving from the other side. Cost: one field, one helper, and a
`NoRuntime` error variant.

**A transaction whose lifetime is tied to the trait method.** `append`'s signature
is untouched. What the async boundary did force is the conditional path: under
`READ COMMITTED` two writers racing one boundary both find no conflict and both
win, and the other adapters avoid that by holding a write lock — `BEGIN IMMEDIATE`
on SQLite — which is the move this crate exists *not* to make. A conditional
append therefore runs `SERIALIZABLE`, whose SSI is optimistic and takes no locks,
so disjoint boundaries still proceed in parallel; a `40001` retries, and the retry
sees the winner's row and returns `ConditionViolated`. An unconditional append
asserts nothing and stays at the pool default.

**The cursor.** `PoolConnection::drop` spawns to return the connection and panics
with no runtime, so the cursor carries the handle and rolls its transaction back
on it explicitly. Explicitly, because a read holds a `REPEATABLE READ` transaction
open and under this mechanism an open transaction is exactly what holds the
frontier back — when it ends is not a detail.

**Error mapping.** Four new `#[non_exhaustive]` variants, none of them per
`SQLSTATE`: `MissingIdentity`, `MalformedIdentity`, `UnstampedEvent`, `Worker`,
`NoRuntime`. The `40001` retry matches on `sqlx`'s own `DatabaseError::code`.
Neither error enum carries `ConditionViolated`: a violation is not an adapter
failure and travels as the contract's own `AppendError`.

**And the read half, which is part of this bill and is easy to omit.** The
frontier predicate is composed *into* the cursor's `DECLARE`, inside one
`BEGIN ISOLATION LEVEL REPEATABLE READ READ ONLY`, and evaluated once. Not per
`FETCH`: the frontier advances between chunks, and a per-chunk predicate lets rows
appear beneath positions the caller has already been handed — ES-10's violation
arriving through the read path while the write path was being fixed.

### 4. Staleness is the real cost, and it is a property of the cluster

Nine samples per cell, milliseconds:

| Arm | control, **median** | behind a 5 s held write transaction |
|---|---|---|
| baseline (unguarded) | **0.521** | **0.595** |
| **arm C** | **0.593** | **4799.3** |

**Unloaded the mechanism is free** — 0.593 against 0.521, overlapping — so
read-your-own-writes is available in practice when nothing else is running. It is
not *promised*, and the crate does not claim it.

**Behind a holder the bound is the whole holder.** 4799 ms against a 5,000 ms
hold, spread 3.6 ms across nine samples: the remainder of the transaction, not a
fraction of it.

The unguarded row is the control phase 2 did not have. Phase 2 compared arm C
unloaded against arm C behind a hold, which shows the frontier moves but not that
the frontier is what moved — a busy server slows everything. The unguarded arm
under the identical hold is unaffected. Same server, same hold, same load; only
the predicate differs.

**What a caller should plan for:** sub-millisecond unloaded, and *the duration of
the longest open write transaction anywhere on the cluster* otherwise. The holder
in this measurement writes to its own table, in rows this store has never heard
of. A migration, a batch job, an idle-in-transaction connection or an unrelated
tenant is enough. That is a documented capability limit, not a tuning parameter,
and it is why ES-30's rule asserts a bound rather than an equality.

### 5. The arms that lost, and they lost for two different reasons

| Arm | Verdict | Why |
|---|---|---|
| A — serialised sequence table | **cost** | 0.062 at 64 writers, p99 60× worse, throughput flat from 8 clients up. Correct, and it buys ES-10 by funnelling every writer through one row. |
| B-const — advisory lock, constant key | **cost** | 0.033 at 64 writers. Arm A wearing a different hat. |
| B-tag — advisory lock, tag-derived key | **invariant** | 0.935 at 64 writers — nearly free — and it reproduces the baseline inversion whenever two writers hold disjoint keys. |

A and B-const are *correct*. They are rejected because a store that serialises
every writer is `MemoryEventStore` with network latency, and this crate exists to
occupy the other end of the position-allocation axis.

**B-tag is the one to read twice.** It is the cheapest arm after C, and ranking
these by throughput would have selected it. It is cheap *because* it buys
something weaker: a **per-boundary** invariant, where ES-10 states a **global**
one. That is why this record prices the arms by what they buy first and what they
cost second.

### 6. The global premise is inherited, not owned, and phase 6 can reopen this

ES-10 is global. B-tag's per-boundary property would make `AppendCondition` sound
and the projection checkpoint unsound, because `head()` is not query-scoped
(ES-30) and a runner therefore checkpoints on a global position covering
boundaries it never reads.

**That argument rests on the checkpoint being global, and the checkpoint is phase
6's.** If a boundary-scoped checkpoint lands, B-tag's weaker invariant may become
sufficient, and both ADR-0013's global framing and this measurement are reopened
with it. This record inherits the premise and states it as inherited;
`.kb/open-questions/global-versus-per-boundary-visibility-invariant.md` owns it,
and nothing here settles it.

### 7. The rule this mechanism exists to pass, and what its pass is worth

Phase 10's exit criterion is that `nothing_below_an_observed_position_appears_later`
is a rule the adapter had to *work* to pass. Two predecessor runs are cited rather
than re-run.

**The concurrency family green at `CONTENDERS = 64`** under a multi-thread runtime
(HS-S0063) — the first adapter in the portfolio to clear that bar on a store whose
writers are not serialised, and `k_disjoint_boundaries_admit_exactly_k_commits`
green is the specific evidence that SSI did not quietly become a lock.

**The naive-arm control (HS-S0064), and it did not go as expected.** The naive arm
— this adapter with the predicate removed — **passes** CF-13. The defect is real:
a hand-built probe observes `[2]` then `[1, 2]`, position 1 appearing beneath an
already-observed position 2, while the shipped arm shows `[]` then `[1, 2]` and
never inverts. The rule cannot see it, because this adapter's `append` advances
*off-poll* and no poll-based schedule decides when its transaction commits.

**So the shipped arm's pass against CF-13 is not, by itself, evidence that this
mechanism works.** What is evidence is the probe in §4 and the staleness table:
the mechanism is observed doing the thing it exists to do, directly, rather than
inferred from a green rule that also passes the broken store.

That is an uncomfortable sentence for a decision record and it is the honest one.

### 8. The poll-count decorator, which fired

`spec/SPECIFICATION.md` names a poll-padding decorator over
`PreCommitPositionStore` and owes it to phase 10. It was built, calibrated and run
(HS-S0064). ADR-0013 refused to let an author choose the padding — *"an author
choosing n is the reference-store failure mode with one more step"* — so it comes
from this adapter: `append` measures at **3 polls** at a one-millisecond cadence
(and ~25,000 in a tight loop, which is the same fact wearing a different hat), the
unpadded mutant needs 2, so the padding is 1. Three is the number the
specification names.

**It fired.** The padded mutant passed the rule. ES-10's own disposition applies —
*"if it fires, the rule changes and this clause does not"* — and the rule's
schedule changed accordingly, with ES-10 untouched.

`POLL_BUDGET` as a fixture capability is now **moot**: the new schedule stopped
counting polls, so CF-33's tension never has to be resolved.

## Consequences

- `happenstance-postgres` occupies the position-allocation axis's far end with a
  mechanism whose cost is read-side and documented.
- **Read-your-own-writes does not hold** and is not claimed. `head()` is a
  frontier. `append` returning `Ok(P)` promises nothing about the next `head()`.
- A consumer inherits a staleness bound they do not control and may not be able to
  observe — the longest open write transaction anywhere on their cluster.
  `postgres-structural-bill` (HS-S0066) puts this where a caller meets it.
- The steady-state ratio is owed a machine that can resolve it. Nothing depends on
  it: no rival lost by a few percent.
- CF-13 cannot detect an off-poll adapter's visibility defect. That is a live
  limitation of the suite, recorded and not closed here.

## What this does not decide

**Whether the suite should be able to catch an off-poll adapter at all**, and with
what instrument. The port exposes no suspension point between allocation and
commit for a rule to wedge, and the thing that does catch it — holding a
transaction open — is adapter-specific and lives in the adapter's own tests.

**The global-versus-per-boundary question** (§6). Phase 6's.

**`contains_event_id`'s frontier disagreement.** It answers `true` for a
held-but-invisible row, so it briefly disagrees with `read`; the alternative lets
a replication ingest re-accept an event the store holds. Which is right is a
replication question and ES-41 stays `[PROVISIONAL]`.
