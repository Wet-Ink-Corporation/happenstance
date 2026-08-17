# ADR-0022: The append condition is a `max(position)` guard inside `BEGIN IMMEDIATE`, tags live in a join table keyed `(tag, position)`, and both were measured before a line of the adapter was written

- **Status:** accepted.
- **Date:** 2026-08-16
- **Settles:** the ADR queue's row **0022** (`RUNBOOK.md:301`) — *"SQLite: driver,
  schema, tag storage, and the append-condition strategy."* One question, seven
  consequences that hang off it, two non-verdicts fenced off from both. It
  discharges **no clause**, amends none, and moves no marker.
- **Ratifies rather than decides:** the **driver**. `rusqlite`, bundled, without
  a pool, was already settled in the crate's own module documentation
  (`crates/happenstance-sqlite/src/lib.rs:47-53`), which says so in terms and
  names `sqlx` as `happenstance-postgres`'s end of that axis. That half of the
  queue row was **stale on arrival**, exactly as the first half of ADR-0008's
  was (`RUNBOOK.md:286`). §2.
- **Rests on and does not settle:**
  [ADR-0012](0012-append-shape-and-preconditions.md). `append` keeps
  `events: &[Event]` and the `[PROVISIONAL]` marker on ES-17 **does not lift
  here** — this record cannot produce the evidence ADR-0012's own falsifier
  demands, and says which item it cannot produce and why. §13.
- **Cites rather than edits:**
  [`.kb/open-questions/cf-40-fixture-limits-ownership.md`](../../.kb/open-questions/cf-40-fixture-limits-ownership.md).
  This project needs the *capability* a fixture has to state numeric limits; it
  gets no say in which clause owns it. §14.
- **Corrects, against a measurement:** the schema sketch published at
  `crates/happenstance-sqlite/src/event_store.rs:36-54`, which keys
  `event_tag(tag, position)` with **no type column**. §7. The correction is
  `schema-migration-and-identity`'s to apply; this record is where it becomes
  citable.
- **Reverses its own brief, on evidence:** the architecture brief recommends
  `BEGIN IMMEDIATE` + an `EXISTS` probe (`_decomposition.md` §6). The
  measurement says the **monotonic-position guard**, and AC-013's whole content
  is that this record quotes a figure rather than a preference. §4, §5.

---

## 1. The question, and the one it has quietly become

**As queued:** *SQLite: driver, schema, tag storage, and the append-condition
strategy.*

**As answered:** *How does a SQLite adapter evaluate an append condition
atomically — and what schema, tag storage, pragmas and runtime does that answer
force?*

`RUNBOOK.md:267` is the rule this record is held to: an ADR that cannot be stated
as one question is two ADRs. The queue row lists four subjects and the
architecture brief's §12 lists eight, which looks like eight decisions. It is
not. One of the four was already settled (§2), and everything else on both lists
is a **consequence** of the one question above — the schema is what the guard
must read, the tag storage is what the guard must read *through*, the pragmas are
what the guard's transaction runs under, and the runtime is what carries the
whole thing to the driver. This record states the hang rather than merely listing
them.

Two subjects on those lists are **not** consequences and are recorded as
non-verdicts with named owners (§13, §14). An accepted decision atom is
immutable, so a paragraph written on evidence this record cannot produce can
never be corrected — only superseded. That is why they are fenced rather than
absorbed.

## 2. The driver half was stale on arrival

`crates/happenstance-sqlite/src/lib.rs:47-53` already says:

> The **driver** is no longer one of them: `rusqlite` is what this crate is built
> on, chosen for the synchronous, bundled, local-first shape it gives — and it is
> `rusqlite` *without* a pool, because one `Mutex`-guarded connection is the
> serialising instrument the portfolio needs. `sqlx` is not discarded; it is
> where `happenstance-postgres` sits, at the other end of that axis.

That is a decision, taken at phase 2 by building both, and this record **ratifies
it rather than re-taking it**. The experiment inherits it wholesale: every
candidate is `rusqlite`, bundled, one `Mutex<Connection>` per handle, no pool.

The precedent for recording half a queue row as stale is ADR-0008
(`RUNBOOK.md:286`), whose first half — *is the second flavour derived or
hand-written* — was already `[FROZEN]` when the row was written. Saying so is
what stops the next reader looking for a decision that was never taken here.

## 3. The instrument, and why it is outside the workspace

`SqliteEventStore::append` is `todo!()` and stays that way until three stories
after this record lands: **AC-013 puts the record before the implementation**, and
a PR that did both would have destroyed the ordering it was written to prove. So
there was nothing in the workspace to measure.

The number comes from **`experiments/append-condition/`**: three candidate
`EventStore` implementations over `rusqlite`, three tag storages, one schema, one
read path, one identity story. It carries an empty `[workspace]` table so cargo
does not adopt it, it is in no `verify:` command and no `cargo xtask ci` step, and
it adds no dependency to any workspace manifest — the root `Cargo.lock` is
untouched. The precedent for every one of those choices is
`experiments/position-visibility/`, which measured the Postgres
position-visibility question one phase before any Postgres adapter existed.

**Three properties make the numbers usable rather than merely present.**

**Conformance first.** `tests/candidates_are_conformant.rs` points
`happenstance_testkit::event_store_conformance!` at each arm — **445 tests, 89
rules across 5 arms, all passing**. A wrong arm is always the fastest, so an arm
that had not cleared the suite would have had its figure discarded and the
failure recorded as what that arm costs.

**The workloads are the testkit's.** `tests/measure.rs` drives
`event_store_benchmarks!` **verbatim** — the family HS-S0034 added, at `n = 512`,
`k = 64`, `N = 5000` — so a later `event_store_benchmarks!(SqliteFixture::new())`
re-derives the same scenarios against the real adapter. A private timing loop
would have produced figures the adapter can never reproduce.

**The durability settings are read back and enforced.**
`spec/SPECIFICATION.md:7481-7484` names `PRAGMA synchronous = OFF` **by name** as
a wrong implementation CF-14's reopen rule exists to reject, so a figure produced
under it is a figure for a store that cannot ship. `Durability::read_back` reads
`journal_mode`, `synchronous` and `busy_timeout` off the live connection —
*read back*, because SQLite silently ignores a `journal_mode` it cannot honour —
and `require_shippable` refuses to proceed under `OFF`.
`tests/durability_settings_are_enforced.rs` **forces the refusal**, because a
control that cannot fire is decorative. This is the exact analogue of
`experiments/position-visibility/setup.sh` aborting under `fsync=off`.

### 3.1 What the instrument cannot do, stated before its numbers are read

**Two harness figures are noise-dominated on this host, and it is the host's
fault rather than the harness's.** `event_store_benchmarks!` emits one `#[test]`
per scenario, so each arm is timed in its own slot; on a shared developer machine
two runs an hour apart disagreed by up to 45%, and one arm's *unconditional*
append — a path no strategy participates in — varied by 4x between slots. The
harness's per-scenario timer also covers fixture construction, which at `k = 64`
is sixty-five `connect()` calls.

The remedy is the **emitter's**, which is what CF-23 makes the wrapper a
parameter for, and this experiment took it: the two figures this record decides
on come from caller-side controls that measure the arms **round-robin inside one
process**, one operation each per round, so a host that gets busy affects all of
them equally. They are labelled as controls everywhere they are quoted and they
replace no harness figure. `bench.rs`'s own module documentation delegates the
threaded half in terms — *"an adapter that wants thread-level contention supplies
it through its own emitter"*.

**Conditions.** 13th Gen Intel Core i9-13905H, 20 logical cores, 32 GB, Windows
11, NTFS on local NVMe, `rustc 1.97.1`, `--release`, SQLite 3.53.2 via `rusqlite`
0.40 `bundled`, `journal_mode=wal`, `synchronous=normal`, `busy_timeout=5000 ms`.
Every row prints them beside itself. `./run.sh` re-derives all of it in about
five minutes, of which the 64-contender races are 220 s.

## 4. The decision

**A SQLite adapter evaluates an append condition as a `SELECT max(position)`
guard, inside a transaction opened `BEGIN IMMEDIATE`.**

Per guard of the condition, in one statement:

```sql
SELECT max(position) FROM ( <the guard's query, per the tag storage> )
```

The guard is violated when that answer is `Some(p)` with `p > after`, and `p` is
the conflicting position reported through
`AppendError::ConditionViolated::at(p)`. `after` of `None` is a boundary of zero,
because positions start at one. Guards are checked in order and the first
violation ends the transaction with a rollback, which is what leaves a rejected
append byte-identical.

`BEGIN IMMEDIATE` — not deferred — is half the decision and is not negotiable:
the write lock is taken **before** the condition is read, so the snapshot the
guard sees is the snapshot the insert writes into, and no second writer can fit
between the two halves. That is the property `racing_conditional_appends_elect_one_winner`
exists to check, and the reason a probe followed by an unrelated insert is the
wrong implementation the whole port is built to reject.

### 4.1 Why `max(position)` and not `EXISTS`

The guard asks *"is there anything matching after this boundary"*, which reads
like an existence question and is an **inequality on the highest matching
position**. One `max(position)` answers both halves at once — whether the
condition is violated, **and by which event** — so the rejection path needs no
second query. `EXISTS` answers a boolean and needs a follow-up `min(position)` to
name the conflict.

The arm was written expecting to lose. `max()` cannot stop at the first hit,
where `EXISTS` short-circuits, so it should degrade as the matching set grows.
**It does not**, and the reason is §6's schema: with `event_tag` keyed
`(tag, position)`, `max(position)` over a single tag's range is a **seek to the
end of that range** rather than a walk. The two decisions compose, which is
exactly why they are one record rather than two.

## 5. What lost, with the figures and their conditions

The three arms are **indistinguishable** on the commit path and under contention,
and separate reproducibly on the **rejection** path — the one a DCB command loop
takes every time it loses a race and has to re-decide. Four hundred rounds per
arm, round-robin in one process, medians in microseconds
(`experiments/append-condition/results/append-condition.md` §1; raw rows in
`results/raw/contention.txt`):

| strategy | 1-tag @ 5,000 | 2-tag @ 5,000 | 1-tag @ 50,000 | 2-tag @ 50,000 |
| --- | --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + `EXISTS` probe | 32 | 1,513 | 311 | 65,637 |
| conditional `INSERT … WHERE NOT EXISTS` | 45 | 1,532 | 306 | 65,383 |
| **monotonic-position guard** | **23** | **972** | **213** | **42,399** |

**`BEGIN IMMEDIATE` + `EXISTS` probe — lost, narrowly, and it is the brief's own
recommendation.** 32 µs against 23 at 5,000 events and 311 against 213 at 50,000;
1,513 against 972 and 65,637 against 42,399 on a two-tag boundary. It is a close
second on every axis and it is architecturally identical to the winner — same
transaction behaviour, same lock discipline, same snapshot. What it cannot avoid
is the **second query** it needs on the rejection path to name the conflicting
position, which the guard gets for nothing.

**Conditional `INSERT … SELECT … WHERE NOT EXISTS` — lost, and on a second
ground as well.** 45 µs against 23, and 1,532 against 972, at 5,000 events; at
50,000 it is level with the probe arm and still behind the guard. It is also the
only arm whose transaction is **deferred**: it begins as a reader and upgrades to
a writer at the `INSERT`, so two writers that both hold a read lock and both try
to upgrade produce `SQLITE_BUSY` on one of them. That is a distinct and more
failure-prone concurrency story, bought for no measured gain.

**Where nothing separated, and it is recorded as nothing.** On the *accepted*
conditional append the guard costs 67, 79 and 74 µs across the three arms — an
18% spread where the same three numbers moved by more than that between runs, and
at 50,000 events the subtraction stops resolving at all because the commit rises
to ~18 ms. Under contention at 8 and 64 connections the ranges overlap almost
completely and the ordering changed twice across earlier runs. **A tie is a
finding; a manufactured winner is a lie with a table attached.** The decision
rests on the rejection path and on nothing else.

**Every arm is correct.** All three elected exactly one winner in all thirty
races at both contender counts, every loser learned it lost as a
`ConditionViolated` rather than as an adapter error, and
`committed + rejected + busy + failed` accounted for every contender in every
round. The choice between them is a cost decision, not a correctness one.

## 6. Consequence — tag storage is a join table keyed `(tag, position)`

**Chosen: `event_tag(tag, position)` `WITHOUT ROWID`, with `event_type` carried
as a covering column and the key left `(tag, position)`.**

50,000 seeded events, 25 rounds, round-robin in one process, each store
checkpointed and `ANALYZE`d before anything is timed
(`results/tag-storage.md`; raw rows in `results/raw/tag-storage.txt`). Medians in
microseconds:

| arm | unconditional append | probe | **selective read** (516 of 50,050) | broad read (16,684) | unfiltered (50,050) |
| --- | --- | --- | --- | --- | --- |
| **join table** | 862 | 556 | **10,744** | 75,599 | 131,611 |
| canonical blob | 414 | 623 | 33,992 | 71,351 | 116,036 |
| JSON1 | 590 | 434 | 49,766 | 99,329 | 126,410 |

**The join table is 3.16x the canonical blob and 4.63x JSON1 on a selective
read**, which is the shape a consistency boundary actually has — a handful of
events out of a log, where the cost is dominated by *finding* them. The noise
floor is **±7%**, measured as the unfiltered read, in which all three arms
execute identical SQL over identical rows.

**The canonical blob was measured, not dismissed**, and the distinction matters:
`Tags` is canonically sorted *precisely so* that a single-column encoding stays
viable, and discarding it on taste would contradict the reason that sorting
exists. It loses because it has no index over tags at all and must scan every
candidate row applying `instr`. **JSON1** loses harder and for the same reason
plus a per-row parse.

**What the win costs, stated rather than buried.** An unconditional single-event
append is 862 µs against 414 and 590 — about **1.5x to 2.1x** — for two extra
`event_tag` rows and two `tag_cardinality` upserts per event. The trade the
adapter is making is roughly a factor of two on writes for a factor of three to
five on the reads a DCB command loop performs **before every one of those
writes**.

**One regime where the advantage disappears**, recorded because a reader who
measured only that regime would reach the opposite conclusion: on a *broad* read
matching a third of the log, materialising `SequencedEvent`s dominates and the
three arms land inside the noise floor.

## 7. Consequence — migration 1, and what the published sketch gets wrong

`crates/happenstance-sqlite/src/event_store.rs:36-54` publishes a schema sketch
that is **known to be wrong**, in a way that serialises every writer: it keys
`event_tag(tag, position)` with **no type column**, so a query item constraining
both type and tags becomes a join back to `event` — walked *under the
`BEGIN IMMEDIATE` write lock*, with every other writer waiting behind it
(`RUNBOOK.md:4178-4187`). The correction is stated here and applied by
`schema-migration-and-identity`; this record is what makes it citable.

Migration 1, in full:

```sql
CREATE TABLE event (
    position        INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type      TEXT    NOT NULL,
    data            BLOB    NOT NULL,
    metadata        BLOB,             -- nullable: None and Some(<empty>) are two values
    tags            BLOB    NOT NULL, -- the canonical sorted encoding
    origin_store    BLOB,
    origin_position INTEGER,
    recorded_at     INTEGER NOT NULL,
    UNIQUE (origin_store, origin_position)
);
CREATE INDEX event_type_idx ON event(event_type, position);

CREATE TABLE event_tag (
    tag        TEXT    NOT NULL,
    position   INTEGER NOT NULL REFERENCES event(position),
    event_type TEXT    NOT NULL,      -- covering column, NOT part of the key
    PRIMARY KEY (tag, position)
) WITHOUT ROWID;

CREATE TABLE tag_cardinality (
    tag    TEXT    PRIMARY KEY,
    events INTEGER NOT NULL
) WITHOUT ROWID;

CREATE TABLE store_meta (k TEXT PRIMARY KEY, v BLOB NOT NULL) WITHOUT ROWID;
```

Each column, and why it is not negotiable:

- **`AUTOINCREMENT` is load-bearing rather than stylistic.** Positions must never
  be reused after a delete and plain `rowid` does not guarantee that. It also
  *permits gaps*, which is why no rule and no code may assume `+1`.
- **`event_type` on `event_tag` is a covering column and not part of the key.**
  In the key it would break the position ordering that makes a tag's range
  already sorted; absent — which is what the sketch publishes — it forces the
  join under the write lock.
- **`metadata` is nullable.** `None` and `Some(<empty>)` are two values the
  contract keeps apart and `metadata_distinguishes_absent_from_empty` is the rule
  that notices a store folding them.
- **The `EventId` origin pair is `UNIQUE` *together*.** It is both the index
  `contains_event_id`'s probe seeks and the constraint that stops ingest storing
  one event twice (`crates/happenstance-core/src/identity.rs:97`). It is nullable
  during the insert and stamped from the position just assigned in one statement
  at the end of the batch, because a local event's identity *is* this store's
  incarnation paired with the position it was given — and that is not known until
  the row exists. SQLite treats NULLs as distinct, which is what lets a
  multi-row batch stamp itself without tripping the constraint.
- **`recorded_at` is read back on reopen, never re-stamped.** A store whose
  reopen restamps hands every auditor the time of the last restart;
  `recorded_time_survives_a_reopen` is the rule.
- **The store's own `StoreId` is minted once at schema creation and persisted**
  in `store_meta`. Mint-per-open is permitted by VT-6 in general and is the wrong
  choice for a file-backed store: `reopened_store_does_not_reissue_an_event_id`
  is what it fails.

**Migration must be idempotent and safe under a concurrent open.**
`SqliteEventStore::open` calls `migrate` on every connect
(`event_store.rs:118-124`) and a fixture connects more than once onto one file:
`IF NOT EXISTS` inside `BEGIN IMMEDIATE`, and `INSERT OR IGNORE` then read-back
for the `StoreId` row. The projection store's checkpoint schema migrates
independently on its own connection and must be equally idempotent — an event
store and a projection store on one file are two connections, not one. All of
that is exercised by the experiment: 445 conformance tests over five arms, every
one of which opens the same file more than once.

## 8. Consequence — the single-tag fast path, and why `tag_cardinality` is a requirement

The general superset test is
`GROUP BY position HAVING COUNT(DISTINCT tag) = n`. **A `GROUP BY` is an
optimisation barrier**: SQLite cannot push the enclosing `position > ?`
predicate — the boundary every append-condition guard carries — through an
aggregate, so the probe materialises *every* matching position in the log and
then discards the ones at or below the boundary.

With **exactly one tag** the aggregate asserts nothing: *"carries at least this
one tag"* is membership, and the `(tag, position)` key answers it with a seek.
Dropping it is measured against its own negative control — the same table, the
same rows, the same index, always taking the grouped form — in one interleaved
run: **1,093 µs → 556 µs, a factor of 1.97**. A single-tag query item is the
overwhelmingly common shape of a consistency boundary.

**The multi-tag path is not merely slower, it is a different order of
magnitude.** A two-tag boundary over a 50,000-event log costs **42 ms to 66 ms
per guard evaluation** against 0.2 ms to 0.3 ms for a single-tag one — roughly
**200x** — on every strategy. Two things follow, and both belong in migration 1
rather than in a later optimisation pass:

1. **`tag_cardinality` is a requirement, not a convenience.** Multi-tag arms must
   be probed **most-selective-tag-first**, and SQLite cannot supply per-value
   cardinality on its own: `ANALYZE` stores an *average*, which is exactly wrong
   for a tag set where one value matches a third of the log and another matches
   one percent.
2. The 200x is measured **with `event_type` already covering**, so it is a floor
   on the multi-tag path rather than evidence against §7's correction.

**Rejected: relying on `ANALYZE` and the query planner.** It is what a reader
reaches for first, and it cannot work for the reason above — the statistic SQLite
keeps is not the statistic a most-selective-first probe needs.

## 9. Consequence — the runtime seam

**Chosen: (a) capture a `tokio::runtime::Handle` at construction, prefer it, and
keep `Handle::try_current()` as the fallback.**

`SqliteEventStore::open` / `::new` run on the harness's own thread, which *is*
inside the tokio test, so `Handle::try_current().ok()` there yields a handle. A
`Handle` is `Clone`, `Send`, `Sync` and `Unpin`, so carrying one in the store, in
`ReadCursor` and in `SqliteReadStream` costs the shape assertions nothing.

**Rejected: (b) run the statement inline on the calling thread when no runtime is
found.** It is defensible for a synchronous driver — "no runtime" means there is
no executor thread to starve — and it is what the experiment's candidates do, so
it is not *unsound*: sixty-four bare OS threads, each driving its own future with
the testkit's park-loop `block_on` and no tokio context anywhere, completed
correctly at both contender counts. It loses on the two grounds the architecture
brief names rather than on correctness: it keeps blocking work on an async
executor's thread whenever there **is** one, and it makes
`SqliteEventStoreError::NoRuntime` **unreachable**.

**The fate of `NoRuntime`, spelled out under the option that won.** Under (a) the
variant **keeps a real meaning** — a store both constructed *and* driven with no
runtime anywhere — so it stays, and `event_store.rs:26-32` and `:174-178` stay
true as written. Had (b) won, the variant, that module-doc paragraph and
`SqliteProjectionStoreError::NoRuntime` would all have had to be removed or
rewritten in the same change, rather than left documenting a state that cannot
occur.

**What this record did not measure, stated so it is not read as measured.** The
experiment has no tokio in it at all, so it has nothing to say about the *cost*
of (a) against (b). The decision is taken on the brief's reasoning, with the
experiment contributing only the negative result above: (b) does not break under
the concurrency family's bare-thread contenders. The seam is the one
`concurrency.rs:61-68` predicted, and this project solves it **inside the
adapter** rather than by tightening `ConcurrentFixture`'s bound — a breaking
change to a published crate for the benefit of one adapter.
`postgres-and-neon-stores` is the project that will have the evidence for whether
the seam generalises.

## 10. Consequence — `index_arms()` is rejected

`RUNBOOK.md:4203-4206` and
`references/evaluation/ARCHITECTURAL-EVALUATION.md:827` both write the work item
as *"handle `Query::index_arms()` exceeding SQLite's pushdown limits"*. **That API
does not exist.** `crates/happenstance-core/src/query.rs` has
`Query::items() -> Option<&[QueryItem]>` at `:204`, `QueryItem::types()` and
`QueryItem::tags()`, and no `index_arms`, `arm_count` or `IndexArm` anywhere. No
decision atom mints them; the evaluation *proposes* them. This is recorded
explicitly because the runbook's own work item names the API and a reader would
otherwise think it was forgotten.

**Decision: the decomposition stays adapter-private.** The experiment does it
that way — one `SELECT position …` per query item, `UNION`ed, entirely inside the
candidate crate — and clears
`store_evaluates_a_query_at_the_guaranteed_minimum_item_count` at VT-23's 128-item
floor on all five conformant arms. The behaviour is what the conformance rule
checks; the public name is not.

**Rejected: minting `index_arms()` in `happenstance-core`.** The addition would
be purely additive and `EventStore` being `[FROZEN]` does not forbid it. What
forbids it is CLAUDE.md's own rule: *a port is only as well-designed as the
spread of what implements it*, and one implementor is not a spread. Adding public
surface to the contract crate between the alpha and `0.2.0` for the benefit of
one adapter is the move that rule exists to stop.

**The re-open trigger is named rather than left to memory.** If
`postgres-and-neon-stores` independently needs the same decomposition, that is
two unlike storage shapes agreeing, and *that* is the evidence to mint
`index_arms()` — as its own ADR then, not as a side effect of this one.

## 11. Consequence — three pragma values

Each is a documented property of the adapter, each was read back off the live
connection rather than assumed, and each has an alternative that lost.

**`busy_timeout = 5000` ms — finite and generous.** With sixty-four connections
on one file, `BEGIN IMMEDIATE` on a busy database returns `SQLITE_BUSY`
*immediately* unless a handler is configured, and that error becomes
`AppendError::Store` → `Attempt::Failed` → a red rule that is not about the
adapter's logic. **Rejected: an unbounded busy handler.** There is no watchdog
anywhere in the conformance suite and there must not be one (CF-33,
`crates/happenstance-testkit/src/concurrency.rs:24-43`), so an unbounded handler
converts a livelock into a hung job naming no rule. Five seconds was enough:
`busy = 0` in every row of the 64-contender table, so the timeout did real work
and never ran out — which is what makes that zero mean something rather than
being the zero an unbounded handler would also have produced.

**`journal_mode = WAL`.** Readers do not block the writer, which is the property
a store whose read path is a long replay wants most. **Rejected: `DELETE`**, the
rollback journal, which serialises readers against the writer.

**`synchronous = NORMAL`.** **Rejected: `OFF`** — named by
`spec/SPECIFICATION.md:7481-7484` as a wrong implementation CF-14's reopen rule
exists to reject, so it is not a performance option at all. **Rejected: `FULL`**,
which fsyncs at every commit; under WAL, `NORMAL` is durable across a process
crash and loses at most the tail since the last checkpoint on power loss, and
that is the trade a local-first store should be making rather than paying an
fsync per event. An adapter whose deployment disagrees can raise it; the
*documented default* is `NORMAL`, and CF-14's reopen rule holds at both.

## 12. Consequence — 64 contenders is supportable, measured and not applied

`crates/happenstance-testkit/src/concurrency.rs:200-206` sets `CONTENDERS = 8`
and its own documentation says it is **not a tuning knob**. Both of phase 8's
stated proof artefacts nevertheless read *64*. This record supplies the number
and **does not apply it**: `concurrency::CONTENDERS` is still 8,
`crates/happenstance-testkit/**` is untouched by this change, and
`concurrency-family-and-contender-count` owns the decision and the edit.

Ten races per arm per count, round-robin, wall time per race
(`results/contention-64.md`):

| strategy | 8 | 64 | factor |
| --- | --- | --- | --- |
| `BEGIN IMMEDIATE` + probe | 136 ms | 2,725 ms | 20.0x |
| conditional insert | 123 ms | 1,878 ms | 15.3x |
| monotonic guard | 132 ms | 1,437 ms | 10.9x |

- **Sixty-four `rusqlite::Connection`s on one file opened** on every one of
  thirty races, with no file-descriptor or connection ceiling reached. That is a
  fact about Windows 11 / NTFS on this machine rather than a guarantee about
  every platform, and a platform that does hit a limit should record it rather
  than quietly measure at 32.
- **Correctness is unaffected**: exactly one winner per race at both counts, on
  every arm, with `busy = 0` and `failed = 0`.
- **The cost is wall time, and it is one order of magnitude.** The concurrency
  family carries five racing rules, so the arithmetic a raise has to justify is
  roughly *0.7 s → 7-14 s per adapter per CI run* on a machine like this one.

**The recommendation, which is a recommendation and not an edit:** the raise is
supportable. And a third option this measurement makes visible — the two are
separable, because the proof artefacts that read 64 could be satisfied by a
harness invocation at 64 without the shared constant moving for every adapter in
the world.

## 13. Non-verdict — ES-17 is **not** lifted, and the gap is escalated

ADR-0012 leaves `append`'s `&[Event]` on a `[PROVISIONAL]` marker and names
**phase 8** as its lifting measurement. **This record does not lift it**, and the
reason is item 1 of ADR-0012's own falsifier, restated verbatim
(`references/adr/0012-append-shape-and-preconditions.md:244-266`):

> 1. Two builds of the *same* SQLite adapter differing only in `append`'s
>    ownership, measured on the same harness.

Three candidate stores in an experiment crate are **not two builds of the same
adapter**. They differ in the thing under test — the guard — which is exactly
what makes them useful for §4 and useless for this. The falsifier was written to
stop a positive result being manufactured, and manufacturing one here would be
the failure mode it anticipated.

**What the experiment did observe, offered as an observation and not a verdict.**
The 512-event batch appends in `results/append-condition.md` §4 write every event
through `&[Event]`, cloning one `Box<str>` and one boxed tag slice per event on
the way into the row. At 512 events the whole batch commits in 10-14 ms on the
arms whose slot was quiet, so the per-event copy is somewhere inside ~25 µs
alongside a WAL write, an index insert, two `event_tag` inserts and two
`tag_cardinality` upserts. That is consistent with the copy being small relative
to the write, and it is **not** items 1 through 5, so it settles nothing.

**The gap, escalated rather than absorbed.** No story in this project's map is
currently assigned to produce falsifier item 1 — `_storymap.md`'s coverage table
has no row for it, and the four implementation stories after this one build *one*
adapter rather than two builds of one. That is recorded in the ADR queue rather
than left in a comment: a phase-8 obligation ADR-0012 named, which phase 8 as
planned does not discharge. Items 2 through 5 also remain unproduced, and item 5
in particular is a **design** obligation — *"a cheap way to keep a copy for
retry"* — that no measurement alone can supply.

## 14. Non-verdict — CF-40's clause home stays open

ADR-0015 both claims and disclaims the clause that lets a fixture declare numeric
limits, and
[`.kb/open-questions/cf-40-fixture-limits-ownership.md`](../../.kb/open-questions/cf-40-fixture-limits-ownership.md)
names phase 8 as what forces it.

**This project needs the capability and gets no say in the clause's home.** The
experiment's fixtures exercise it — `Fixture::MAX_EVENTS_PER_BATCH` and its two
siblings are answered on every candidate, and `append_reports_exceeded_store_limits`
runs against all five arms — which is the capability working, and says nothing
about which clause owns it. The open question is **cited and left untouched**;
this record does not edit it and does not settle it.

## 15. What this record deliberately does not do

- **It replaces no `todo!()`.** Not one file under `crates/happenstance-sqlite/src/`
  moved, `#![allow(clippy::todo)]` is still there, and the wrong module-doc
  sketch at `event_store.rs:36-54` is still wrong — its correction is
  `schema-migration-and-identity`'s and `instrument-markers-removed-and-gate-green`'s.
  AC-013's whole content is that the record precedes the implementation.
- **It edits no clause.** `spec/SPECIFICATION.md` is untouched: no text, no
  marker, no citation range. `reopen-negative-control-and-durability-verdicts`
  and `spec-and-code-reconciliation` own the clauses this record records verdict
  *positions* for — CF-14 via the `synchronous` value it fixes, and CF-17 / ES-35
  via the durability settings the reopen rules will run against.
- **It adds and changes no conformance rule.** The rule-name count from
  `for_each_event_store_rule!` is exactly where `benchmark-harness` left it,
  which is also what keeps that story's AC-012 claim true.
- **It does not touch `PAGE_SIZE`.** `event_store.rs:81` calls the current 512
  "a placeholder until it is measured" and it stays one; nothing here measured
  paging.

## 16. The falsifiers

Each verdict names what would re-open it, so a later reader has something to
check rather than a paragraph to defer to.

**§4, the strategy.** The verdict rests on `max(position)` over a single tag's
range being a **seek** rather than a walk, which is a property of §6's
`(tag, position)` key. Re-open it if the tag storage changes, or if a measurement
over a boundary whose matching set is a large *contiguous* range — where `EXISTS`
can stop at the first row past the boundary and `max()` still cannot — shows the
probe arm ahead.

**§6, the tag storage.** The verdict rests on selective reads being the common
shape. Re-open it if a deployment's dominant read is broad — matching a large
fraction of the log — where the three arms measured inside the noise floor and
the join table's 1.5x-2.1x write cost is paid for nothing.

**§8, the fast path.** Re-open it if a future SQLite pushes predicates through an
aggregate, at which point the special case stops earning its branch.

**§9, the runtime seam.** Re-open it if a deployment shows the captured `Handle`
costing something the inline path does not, or if `postgres-and-neon-stores`
finds the seam generalising — in which case it becomes a testkit question rather
than an adapter one.

**§11, the pragmas.** Re-open `synchronous` if a reopen rule is found that
`NORMAL` cannot pass under WAL. Re-open the busy timeout if any run ever reports
`busy > 0`, which would mean five seconds stopped being generous.

**§12, the contender count.** Not this record's to re-open: it supplies a number
and `concurrency-family-and-contender-count` decides.
