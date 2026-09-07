# The position-visibility measurement, re-taken against the built adapter

Phase 2 measured four SQL scripts through `pgbench` and chose arm C on the
numbers. `.kb/open-questions/postgres-arm-c-structural-cost.md` records what that
could not answer: *"no connection pooling, no transaction lifetime tied to a trait
method, no cursor, no error mapping"* — four gaps between "a SQL strategy passed"
and "an adapter passed". This pass closes them by calling the shipped
`PostgresEventStore`.

Raw: `adapter-remeasurement.txt` (this run, over the unix socket) and
`adapter-remeasurement-host-tcp.txt` (an earlier attempt, kept because §3 is
about why it was discarded).

## Conditions

| | |
|---|---|
| Server | PostgreSQL 17.10 (Debian 17.10-1.pgdg13+1), `postgres:17.10` |
| `fsync` | **on** — the harness asserts it and aborts otherwise |
| `synchronous_commit` / `wal_sync_method` / `full_page_writes` | `on` / `fdatasync` / `on` |
| `max_connections` | 200, matching phase 2 |
| Client | this crate, **inside the container**, over `/var/run/postgresql` |
| Per level | 10,000 seeded rows, 5 s warmup, 20 s timed, 8 s settle, schema dropped after |
| Workload | probe-then-append: `read_decision_model`, then a conditional `append` under it |

## 1. The pairing, and why the baseline is what it is

**Baseline** is `PostgresEventStore::new_naive` — the shipped adapter with the
visibility predicate removed, which is unguarded `nextval()` and is what Postgres
does by default. **Arm C** is `PostgresEventStore::new`.

One adapter against itself with exactly the mechanism under test switched off.
Same pool, same transaction lifetime, same cursor, same error mapping, same
schema, same append path — so a difference is the predicate's and nothing else's.

## 2. Steady state: no cost is measurable, and the precision is the finding

Ratios above 1.0 are faster than baseline. Each is taken against the mean of the
baselines immediately before and after it, three repetitions per level.

| Clients | arm C ratio, min | **median** | max |
|---|---|---|---|
| 1 | 0.837 | **0.985** | 1.219 |
| 8 | 0.865 | **1.131** | 1.190 |
| 32 | 0.863 | **1.026** | 1.118 |

**All three medians straddle 1.0, and the spread is about ±20%.** The effect
being looked for is 0–3%: phase 2 measured arm C at 0.99–1.03 at the SQL level.
So this measurement **cannot resolve the cost**, and it would be dishonest to
quote a point estimate from it.

What it does say, and it is not nothing: nothing here contradicts phase 2, no
level shows arm C consistently below baseline, and no cost large enough to matter
is hiding inside ±20%. A mechanism costing 16× or 30% would be unmissable at this
precision, and the two arms that do cost that are named in §5.

## 3. Why the precision is what it is, stated rather than hidden

Baseline drift within a single paired triple ran 1.01 to 1.88. Docker Desktop's
virtual disk on Windows is the environment the parent README's §4 already
describes: *"an instance that has been fsyncing for twenty minutes is not the
instance that started."*

Four things were tried and are recorded so nobody repeats them:

1. **Pairing** — already the design, and it is why the drift is visible at all
   rather than silently inflating a ratio.
2. **Seeding and warmup** — 10,000 rows and 5 s, matching phase 2's own defaults.
   The first attempt had neither, and a table growing from empty changes plan,
   index depth and IO profile *during* the run.
3. **Dropping each pass's schema afterwards.** The first attempt named schemas by
   ordinal and dropped only the name it was about to create, so nine seeded
   schemas with GIN indexes accumulated and the baselines declined monotonically
   through a run. That was the dominant drift term and it was the harness's fault,
   not the disk's. Fixing it reduced the drift and did not remove it.
4. **Running inside the container over the unix socket**, as phase 2 did.
   Absolute throughput went from ~90 to ~285 ops/s — the TCP proxy was costing
   3× — and the staleness control fell from ~2.3 ms to 0.52 ms, which is where
   phase 2's 0.688 ms is. The ratio's spread did not close.

**What would resolve it:** a Linux host with a real disk, or CI. The workload is
also intrinsically harder to measure than phase 2's: one operation there was one
round trip to one SQL function, and one here is a boundary read plus a conditional
append across several.

## 4. Staleness: the number that characterises arm C

Append, then poll `head()` until it admits the position just written. Nine
samples per cell, milliseconds.

| Arm | control, min / **median** / max | behind a 5 s held write transaction |
|---|---|---|
| baseline (unguarded) | 0.489 / **0.521** / 0.770 | 0.532 / **0.595** / 1.154 |
| **arm C** (frontier) | 0.520 / **0.593** / 1.072 | 4797.4 / **4799.3** / 4801.0 |

**The control row is the contribution phase 2 could not make.** It measured arm C
unloaded against arm C behind a hold, which shows the frontier moves but not that
the *frontier* is what moved — a busy server slows everything. Running the
unguarded arm under the identical hold separates them: it is unaffected, at
0.595 ms, while arm C goes to 4.8 seconds. Same server, same hold, same load; only
the predicate differs.

Two things follow for a caller.

**Unloaded, the mechanism is free.** 0.593 ms against the unguarded arm's
0.521 ms, and the two overlap — read-your-own-writes is available in practice when
nothing else is running.

**Behind a holder, the bound is the whole holder.** The median is 4799 ms against
a 5,000 ms hold, and the spread across nine samples is 3.6 ms. It is not a
fraction of the transaction, it is the remainder of it. And it is a property of
the **cluster**: the holder in this measurement writes to its own table, in rows
this store has never heard of. A migration, a batch job, an idle-in-transaction
connection or an unrelated tenant is enough.

## 5. The arms that lost, and they lost for two different reasons

Not re-measured here; cited from phase 2, because nothing about building the
adapter changes them.

| Arm | Verdict | Why |
|---|---|---|
| A — serialised sequence table | rejected on cost | 0.062 at 64 writers. Correct, and it buys ES-10 by funnelling every writer through one row. |
| B-const — advisory lock, constant key | rejected on cost | 0.033 at 64 writers. Arm A wearing a different hat. |
| B-tag — advisory lock, tag-derived key | rejected on **invariant** | 0.935 at 64 writers, so nearly free — and it reproduces the baseline inversion on disjoint keys. It implements a *per-boundary* property where ES-10 states a *global* one. |

B-tag is the one to read twice. It is cheap **because** it buys something weaker,
so ranking these arms by throughput would have selected it. That is why this
record prices them by what they buy first and what they cost second.
