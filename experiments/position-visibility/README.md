# The position-visibility measurement

`nextval()` allocates outside the transaction, so a Postgres event store breaks
[ES-10](../../spec/SPECIFICATION.md) by construction: a writer takes 99, a
writer that started later takes 100 and commits first, and a reader that has
already observed 100 later sees 99 appear beneath it.
[`happenstance-postgres/src/event_store.rs`](../../crates/happenstance-postgres/src/event_store.rs)
works through what each of the three candidate mechanisms costs the adapter's
types and its append path, and closes: *"Nothing above is a measurement, and the
choice is owed one."* This is the measurement. It does not revise that analysis;
every qualitative claim in it survived contact with a server.

**The short answer.** One of the four mechanisms tested buys ES-10 as written,
at no measurable throughput cost: `xid8` + `pg_snapshot_xmin`. Two buy it by
serialising every writer in the store, at 16× and 30× the throughput
respectively — which is the price of not being an instrument. The fourth,
advisory locks keyed by tags, is the cheap one *because it does not buy ES-10 at
all*: it reproduces the inversion whenever two writers hold disjoint keys, and
what it actually buys is a per-boundary invariant that ES-10 does not state.

This directory is a reproducible experiment. It is **not** a crate, it is not in
the workspace members, it is not a `cargo xtask ci` step, and it adds no
dependency to any `Cargo.toml`. The default gate stays runnable with no Docker.

---

## 1. What was measured, on what

| | |
|---|---|
| Server | PostgreSQL 17.10 (Debian 17.10-1.pgdg13+1), x86_64, stock `postgres:17` image |
| **`fsync`** | **`on`** — `setup.sh` refuses to run otherwise |
| `synchronous_commit` | `on` |
| `wal_sync_method` | `fdatasync` |
| `full_page_writes` | `on` |
| `shared_buffers` | 128 MB (16384 × 8 kB) |
| `max_wal_size` / `checkpoint_timeout` | 1024 MB / 300 s |
| Non-default settings | `max_connections=200`, `log_min_messages=warning`. Nothing else. |
| Container limits | none. `NanoCpus=0`, `Memory=0`; cgroup `cpu.max = max 100000`, `memory.max = max` |
| Visible to the container | 20 CPUs, 15.5 GiB RAM |
| Host | 13th Gen Intel Core i9-13905H, Docker Desktop 4.85.0, WSL2 kernel 6.18.33.2 |
| Client | `pgbench` 17.10, in the same container, over the unix socket |

`fsync=on` is the setting the whole experiment stands on. Under `fsync=off` a
commit costs nothing, and what these mechanisms charge for is precisely *the
length of the interval a lock is held across a durable commit* — so `fsync=off`
would have shown all four arms as free. `setup.sh` reads `SHOW fsync` and aborts
rather than produce that number.

**The schema is the one the crate intends** (`event_store.rs:9-20`): `position
bigint PRIMARY KEY`, `event_type text`, `data bytea`, `metadata bytea`, `tags
text[]`, a GIN index on tags and a btree on `(event_type, position)`. Arm C adds
one column and one index; nothing else differs between arms except the body of
`hs_append` and `hs_probe`.

### The arms

| Arm | Allocation | Read side |
|---|---|---|
| **Baseline** | `nextval()` via a column default | unguarded |
| **A** — serialised sequence table | `UPDATE hs_sequence SET n = n + 1 RETURNING n` inside the append transaction | unguarded |
| **B-const** — advisory lock, constant key | `pg_advisory_xact_lock(0)` then `nextval()` | unguarded |
| **B-tag** — advisory lock, tag-derived key | `pg_advisory_xact_lock(hashtext(tags[1]))` then `nextval()` | unguarded |
| **C** — `xid8` | `nextval()`, unchanged; row carries `pg_current_xact_id()` | `WHERE xact_id < pg_snapshot_xmin(pg_current_snapshot())` on every read and on `head` |

Arm B is run as two arms rather than one because they are different mechanisms
with different answers, and collapsing them loses the finding in §3.

---

## 2. The two positive controls

Both were declared before the runs and both fired. They are here rather than in
an appendix because if either had failed, nothing else on this page would mean
anything.

**Control 1 — the baseline must reproduce the inversion.** It does, on both
writer pairs, deterministically:

```
A position: 6
B position: 7
B blocked : no
head H    : 7   (observation 1, A still uncommitted)
obs 1     : 1,2,3,4,5,7
obs 2     : 1,2,3,4,5,6,7
appeared <= H and not in obs 1: 6
verdict   : INVERSION
```

A took 6, B took 7 and committed first, a fresh reader observed head 7 with 6
invisible, and 6 then materialised *below* an already-observed position. That is
ES-10 violated, with no error, no rejected append and no failing test — exactly
the failure `event_store.rs:30-35` describes. Full transcript:
`results/inversion_baseline__shared_tag.txt`.

**Control 2 — arm A at 64 writers must collapse.** It does. 674 tps against a
bracketing baseline of 10,844 tps — a ratio of **0.062**, with p99 latency 60×
worse (605 ms against 10.0 ms). Arm A's throughput is essentially flat from 8
clients to 64 (822 → 455 → 674 tps) while the baseline climbs from 1,548 to
10,844: a single row that every writer must update converts added concurrency
into latency rather than throughput, which is what a total serialisation point
looks like from the outside.

---

## 3. Correctness — the inversion detector

Three held sessions, driven deterministically. There is **no sleep that
establishes the interleaving**: every step is followed by a barrier that waits
for `psql` to close a marker file, which it cannot do until the preceding
statement has returned. The one deliberate wait is `lock_timeout = 3s`, which is
what turns "session B blocks forever" into a scored result rather than a hang.

```
A: BEGIN, append, do NOT commit
B: BEGIN, append, COMMIT           (may block on A — that is a result, not a failure)
C: fresh READ COMMITTED snapshot; record head H and the visible position set
A: COMMIT
C: fresh snapshot again; anything at a position <= H that was not in the first
   observation is an ES-10 violation
```

Every arm is run with a **shared-key** writer pair (both appends tagged
`course:shared`) and a **disjoint-key** pair (`course:alpha` / `course:beta`).
Ten cases, all in `results/inversion_*.txt`.

| Arm | Shared-key pair | Disjoint-key pair | What it actually buys |
|---|---|---|---|
| **Baseline** | **INVERSION** | **INVERSION** | nothing |
| **A** — sequence table | pass-by-serialisation | pass-by-serialisation | ES-10, globally, by making writers queue |
| **B-const** | pass-by-serialisation | pass-by-serialisation | ES-10, globally, by making writers queue |
| **B-tag** | pass-by-serialisation | **INVERSION** | a **per-boundary** invariant, not ES-10 |
| **C** — `xid8` | pass | pass | ES-10, globally, without blocking anyone |

Three things in that table need saying plainly rather than being read off it.

**"Pass-by-serialisation" is not a win, it is the cost.** Under arms A and
B-const, session B does not return a different answer; it *blocks*, and is
killed by the timeout. The mechanism is not ordering the two writers, it is
preventing the second one from existing while the first is in flight:

```
# arm A
ERROR:  canceling statement due to statement timeout
CONTEXT:  while updating tuple (0,13) in relation "hs_sequence"
SQL statement "UPDATE hs_sequence SET n = n + 1 WHERE id = 1 RETURNING n"

# arm B-const
ERROR:  canceling statement due to statement timeout
CONTEXT:  SQL statement "SELECT pg_advisory_xact_lock(0::bigint)"
```

These arms pass the correctness table and then pay for it in §4. Scoring them
as green without the annotation would rank the two worst mechanisms first.

**Arm B-tag buys a different invariant from the one ES-10 states, and this is
the most useful finding in the correctness half.** With a shared key it blocks
exactly like B-const:

```
CONTEXT:  SQL statement "SELECT pg_advisory_xact_lock(hashtext(p_tags[1])::bigint)"
```

With disjoint keys the two writers never meet, and the result is byte-identical
to the baseline: `A=6 B=7 H=7`, position 6 appearing below an observed 7. ES-10
as written is a **global** statement about the store — *once any reader has
observed an event at position P* — and a lock keyed by tags can only order
writers that share a tag. So the mechanism whose throughput is affordable is the
one that does not implement the clause. Whether ES-10 *should* be global, given
that `AppendCondition` is evaluated against a boundary rather than against the
whole log, is a clause question this experiment deliberately does not settle;
it is raised in §7.

**Arm C passes without blocking anyone.** `A=6 B=7`, both appends succeed, and
the first observation simply reports `H=5` — the frontier, not the maximum
position — so neither 6 nor 7 is ever observed out of order. The cost is that
neither is visible yet, which is §5.

**What this does not prove:** the detector exercises one interleaving of two
writers. It *proves presence* of the inversion on the baseline and on B-tag with
disjoint keys. It does not prove absence on arms A, B-const and C under all
schedules — it proves that this schedule, which is the one that breaks the
baseline, does not break them.

---

## 4. Cost — throughput

`pgbench -M prepared`, one script for every arm, at 1 / 8 / 32 / 64 clients,
30-second runs after a 5-second warm-up, table reset to exactly 10,000 rows and
`CHECKPOINT`ed before each measured run. The transaction is probe-then-append,
because that is the shape of a DCB append and because the interval a lock is
held over is the thing being charged for:

```sql
BEGIN;
SELECT hs_probe(ARRAY['course:' || :tagid]);   -- the consistency boundary
SELECT hs_append('CourseCapacityChanged', …);  -- the write
END;
```

`hs_probe` and `hs_append` are the only things that differ between arms, so the
client work, the round-trip count, the transaction shape and the tag
distribution (1000 keys, uniform) are identical across the table.

### A methods note, because the first design failed

The obvious design — all four levels of the baseline, then all four of each arm,
then the baseline again last to bound drift — produced a baseline that moved
**2.7× at one client and 3.0× at 64 clients** between its first and last
measurement (`results/discarded-sequential/throughput.csv`). On Docker Desktop's
virtual disk, an instance that has been fsyncing for twenty minutes is not the
instance that started. A drift of 3× is larger than two of the three effects
being measured, so every ratio computed against a session-wide baseline would
have been noise wearing a decimal point. The re-run-last control is what caught
it, and it is the only reason this page is not reporting fiction.

The design is therefore **paired**: at each level the baseline is re-measured
between every pair of arms, and each arm's ratio is taken against the mean of
the baselines immediately before and after it. The residual drift is reported in
its own column so the reader can see how much the ratio is being asked to
survive.

### Ratios to baseline

Ratios above 1.0 are faster than baseline; **lower is worse**.

| Clients | Arm | tps | **tps ratio** | p99 (ms) | **p99 ratio** | bracketing baseline tps | baseline drift at this level |
|---|---|---|---|---|---|---|---|
| 1 † | A | 525.9 | 1.031 | 5.34 | 0.97 | 510.0 | 1.94× |
| 1 † | B-const | 371.7 | 0.985 | 6.25 | 1.03 | 377.3 | 1.94× |
| 1 † | B-tag | 336.2 | 0.949 | 6.27 | 1.00 | 354.3 | 1.94× |
| 1 † | C | 327.7 | 0.987 | 6.45 | 1.03 | 332.1 | 1.94× |
| 8 | **A** | 821.6 | **0.531** | 57.67 | **6.06** | 1547.5 | 1.05× |
| 8 | **B-const** | 374.0 | **0.247** | 36.66 | **3.54** | 1515.8 | 1.05× |
| 8 | B-tag | 1492.0 | 0.986 | 10.61 | 1.02 | 1514.0 | 1.05× |
| 8 | C | 1498.0 | 0.993 | 10.45 | 1.02 | 1509.1 | 1.05× |
| 32 | **A** | 455.2 | **0.067** | 549.99 | **76.46** | 6767.5 | 1.03× |
| 32 | **B-const** | 358.0 | **0.052** | 152.71 | **21.75** | 6844.4 | 1.03× |
| 32 | B-tag | 6562.7 | 0.969 | 8.67 | 1.21 | 6774.6 | 1.03× |
| 32 | C | 6870.6 | 1.015 | 7.04 | 0.97 | 6766.5 | 1.03× |
| 64 | **A** | 674.1 | **0.062** | 604.86 | **60.31** | 10843.6 | 1.13× |
| 64 | **B-const** | 369.3 | **0.033** | 243.35 | **24.94** | 11355.4 | 1.13× |
| 64 | B-tag | 10607.5 | 0.935 | 11.77 | 1.21 | 11350.9 | 1.13× |
| 64 | C | 11245.4 | 1.026 | 9.87 | 1.00 | 10961.9 | 1.13× |

† **The one-client row is from a separate 90-second pass**
(`results/ratios-c1long.csv`), because at 30 seconds the baseline spread at this
level was 3.7× — larger than every effect in the row. At 90 seconds it is still
1.94×, and the ratios are paired against adjacent baselines that agree with each
other to within 1%, except arm A's, which is bracketed by 642 and 378 tps and is
the one number on this page to distrust. It does not matter much: with one
writer there is nothing to serialise, so a serialisation point is free, and all
four ratios sitting within 5% of 1.0 is the expected result rather than an
interesting one.

Raw data: `results/throughput.csv`, `results/ratios.csv`, and the 36 individual
`pgbench` reports under `results/`.

### What the table says

**Arms A and B-const stop scaling entirely.** Both sit between roughly 350 and
820 tps at every level from 8 clients up, while the baseline goes from 1,548 to
10,844. That is the signature of a total serialisation point: throughput is
bounded by `1 / (transaction duration)` no matter how many writers arrive, and
every additional writer is converted into queueing delay. Arm A's p99 at 32
clients is 550 ms against a baseline 7.2 ms.

This is the outcome `event_store.rs:49-53` predicted in words — *"an adapter that
funnels all writes through a single lock is `MemoryEventStore` with network
latency"* — and the number is 16× (arm A) and 30× (arm B-const) at 64 writers.
An adapter built on either would not occupy the axis `happenstance-postgres`
exists to occupy; it would be the fifth adapter in the workspace with the same
storage shape as the other four.

**B-const is consistently worse than A**, by 1.3×–2.2× at every level above 1.
The cause was not investigated and this experiment does not offer one; it is
recorded because it was measured and because it is the opposite of what "an
advisory lock is lighter than a row lock" would suggest.

**Arms B-tag and C are indistinguishable from baseline.** Every ratio is between
0.935 and 1.026, against a residual baseline drift of 1.03×–1.13× — that is, the
effect, if any, is smaller than the instrument's noise at these levels. For arm
C this means the read-side predicate and the extra `xid8` column cost nothing
measurable at this table size. For arm B-tag it means the mechanism is nearly
free, which follows directly from the fact that at 1000 keys and 64 writers, two
writers rarely collide — which is the same fact that makes it fail §3.

**The direction this harness errs in.** `hs_append` and `hs_probe` are
server-side functions, so a transaction here holds its lock for one server-side
round trip. A real adapter issues the probe and the write from a client over the
network, holding the lock across that latency too — which lengthens the
serialised critical section and makes arms A and B-const **worse** than measured
here, not better. Arms B-tag and C add latency uniformly and their ratios would
not move.

---

## 5. Arm C's cost: read-side staleness

Arm C neither blocks nor inverts. What it does is refuse to admit a row until
`pg_snapshot_xmin` has passed its `xid8`, so a freshly committed event is
invisible for a while. Measured directly — append, commit, then poll a *fresh*
snapshot until the row is admitted — under the same concurrency levels, 20
samples each, with `pgbench` load running on the same database:

| Concurrent writers | Samples | min (ms) | median (ms) | p95 (ms) | max (ms) |
|---|---|---|---|---|---|
| 1 | 20 | 0.519 | 0.601 | 0.671 | 0.863 |
| 8 | 20 | 0.457 | 0.508 | 0.690 | 0.833 |
| 32 | 20 | 0.627 | 0.782 | 1.079 | 1.188 |
| 64 | 20 | 0.743 | 0.958 | 2.377 | 2.879 |

Sub-millisecond at the median, under 3 ms at the worst sample with 64 writers.
The probe polls at 500 µs, so the minima are at the measurement floor and these
figures are upper bounds; the shape — staleness tracking the lifetime of the
concurrent write transactions, which here are ~6 ms — is what matters.

**And then the number that actually characterises arm C.** `pg_snapshot_xmin`
is the oldest transaction still in flight **anywhere in the cluster**, not in
this store. One long-lived write transaction pins the frontier and every reader
of the event log stops seeing new events for as long as it is held. Held for
five seconds, in a *different database*, on rows this store has never heard of:

```
--- control: no holder ---
STALENESS_MS 0.688

--- with holder ---
STALENESS_MS 4010.719
```

(`results/staleness_pinned.txt`. The probe starts one second in, so 4.01 s is
the whole remainder of the hold.) Arm C's staleness is not a property of the
event store's workload; it is a property of the cluster. A migration, a batch
job, an idle-in-transaction application connection, or an unrelated tenant is
enough. That is the capability limit `event_store.rs:72-75` names — the adapter
compiles and then read-your-own-writes does not hold — and its magnitude is
"however long the longest open write transaction on the server is", which is
unbounded.

---

## 6. What none of this proves

- **The absolute figures are not portable.** Docker Desktop on a Windows laptop,
  on a virtual disk whose throughput moved 3× over twenty minutes. Only the
  ratios are claimed, and only against a bracketing baseline.
- **One node.** No replicas, no hot standby, no connection pooler. Arm C's
  frontier on a standby, and every arm's behaviour behind PgBouncer in
  transaction mode, are unmeasured.
- **One contention profile.** 1000 tag keys, uniform, 64 writers. Arm B-tag's
  ratio is a direct function of key-collision probability; a workload with one
  hot consistency boundary would move it toward B-const's numbers, and a
  workload with no collisions at all would move it toward the baseline's — in
  both throughput *and* in how often it inverts. Neither was measured.
- **One table size.** 10,000 rows. Arm C's visibility predicate cost nothing
  measurable here; at 10⁸ rows the planner may make a different choice, and the
  8-byte-per-row storage cost of the `xid8` column was not evaluated at all.
- **One hold length for the pinned-frontier probe.** Five seconds, one holder.
  The finding is the mechanism, not a distribution.
- **The detector shows presence, not absence.** See §3.
- **Nothing here is an adapter.** `happenstance-postgres` is phase 10. This
  measures four SQL strategies, not four implementations of `SendEventStore`,
  and it says nothing about whether `sqlx` can express the winning one cleanly.

---

## 7. Verdict

**At least one mechanism makes ES-10 affordable, and it is arm C.**
`xid8` + `pg_snapshot_xmin` passed the inversion detector on both writer pairs
without blocking either writer, and its throughput ratio to baseline is 0.987 /
0.993 / 1.015 / 1.026 at 1 / 8 / 32 / 64 clients — inside the instrument's noise
at every level. It is the only arm that both passes and leaves writers
unserialised, which is the pair of properties `happenstance-postgres` needs in
order to be the far end of the position-allocation axis rather than a fifth
adapter with the same shape as the other four.

That is the condition ES-10's `[PROVISIONAL]` marker names — *"one affordable
answer lifts this clause to `[FROZEN]` at phase 4"* — and it is met. **ES-25 and
ES-26 are not reopened.** The finding is not "three unaffordable answers"; it is
one affordable answer and three instructive failures.

But the affordability is bought in a currency the clause does not mention, and
the freeze should say so out loud:

1. **Arm C moves the cost from the write path to the read path, structurally.**
   Every read gains a predicate, `head` reports a frontier rather than
   `max(position)`, and read-your-own-writes does not hold. Under load that
   costs 0.5–3 ms; behind one long-lived transaction anywhere in the cluster it
   costs the whole duration of that transaction. An adapter choosing arm C is
   choosing a store whose freshness is coupled to the health of every other
   workload on the same server, and that is a documented capability limit rather
   than a tuning parameter.

2. **Arms A and B-const buy ES-10 by deleting the reason for the adapter.** 16×
   and 30× at 64 writers, with throughput flat from 8 clients upward. They are
   correct and they are available; what they are not is an instrument.

3. **Arm B-tag is the finding that deserves its own decision.** It is nearly
   free, and it buys a *per-boundary* invariant while ES-10 states a *global*
   one. Two writers on disjoint tags reproduce the baseline inversion exactly.
   This experiment does not settle whether the global statement is the one
   happenstance needs — DCB evaluates `AppendCondition` against a boundary, not
   against the whole log, so a per-boundary invariant may be sufficient for
   everything the contract actually promises, and ES-10 may be stronger than its
   consumers require. That is a clause question, it is worth an ADR, and
   settling it silently in either direction would be the wrong move. **What is
   now measured is that the two are not the same property and that the cheap
   mechanism only implements the weaker one.**

One further consequence for the port, which is the reason phase 2 owns this
rather than phase 10: **arm C pushes nothing back toward the contract.** It needs
no new information from `AppendCondition`, no lock-key derivation, no change to
`append`'s signature. Arm B-tag is the one that would have — it must derive a key
from a contract type the adapter currently only forwards (`event_store.rs:59-63`)
— and it is also the one that does not implement the clause. So the mechanism
that works is the one that leaves the port alone.

---

## 8. How to re-run it

Requires Docker and nothing else. No Rust, no `psql`, no `pgbench` on the host —
both ship inside `postgres:17` and are driven through `docker exec`.

```console
$ bash experiments/position-visibility/run.sh
```

That pulls nothing you do not already have if `postgres:17` is cached, starts a
throwaway container named `hs-pgvis`, applies the five schemas, runs all four
phases, copies `results/` back out, and removes the container. It takes about an hour,
most of it the two throughput passes.

Individual phases:

```console
$ bash experiments/position-visibility/run.sh inversion    # §3, ~1 minute
$ bash experiments/position-visibility/run.sh throughput   # §4, ~30 minutes
$ bash experiments/position-visibility/run.sh staleness    # §5, ~4 minutes
$ bash experiments/position-visibility/run.sh keep         # everything, leave the container up
```

Knobs, all read from the environment by `container/env.sh`:

```console
$ docker exec -u postgres -e HS_LEVELS="1 8" -e HS_RUN_SECS=60 -e HS_TAG=-mypass \
      hs-pgvis bash /harness/container/throughput.sh
```

`HS_LEVELS`, `HS_RUN_SECS`, `HS_WARMUP_SECS`, `HS_SEED_ROWS`, `HS_HOLD_SECS`,
and `HS_TAG` (a suffix for the output files, so a re-measurement does not
overwrite the pass it is being compared with).

**Two things not to change without knowing why.** Do not add `-c fsync=off` to
the `docker run` line; §1 explains what it destroys, and `setup.sh` will abort
anyway. Do not replace the paired throughput design with a sequential one; §4
explains what that cost the first attempt.

### Layout

```
run.sh                        host-side driver: container up, harness in, results out
schema/
  baseline.sql                position bigserial — the unguarded control
  arm-a.sql                   UPDATE hs_sequence … RETURNING n
  arm-b-const.sql             pg_advisory_xact_lock(0)
  arm-b-tag.sql               pg_advisory_xact_lock(hashtext(tags[1]))
  arm-c.sql                   xid8 column + pg_snapshot_xmin predicate on every read
bench/
  append.sql                  one pgbench script, every arm
container/
  env.sh                      settings, arm→schema map
  setup.sh                    five databases; records the environment; aborts on fsync=off
  inversion.sh                §3, three held sessions, ten cases
  throughput.sh               §4, paired design, writes throughput.csv + ratios.csv
  staleness.sh                §5, arm C under load
  staleness.sql               the append→commit→poll-until-admitted probe
  staleness-pinned.sh         §5, arm C behind a long transaction in another database
results/                      everything the above produced, as produced
  environment.txt             §1, read off the running server
  inversion_*.txt             §3, ten full transcripts, plus inversion_summary.txt
  throughput.csv, ratios.csv  §4, the 30-second paired pass
  *-c1long.csv                §4, the 90-second single-client pass
  staleness.csv               §5, with the raw samples beside it
  staleness_pinned.txt        §5, the frontier held by another database
  discarded-sequential/       §4's failed first design, kept as its evidence
```

`results/discarded-sequential/throughput.csv` is kept deliberately. It is the
failed first design, and it is the evidence for the methods note in §4.
