# Adapter implementability review — happenstance

**Lens:** can `EventStore` / `ProjectionStore` be implemented *well, and fast* on the three
committed targets (rusqlite, Cloudflare Durable Object `SqlStorage`, LadybugDB)?

**Method.** Everything below was executed, not reasoned about.

* A **2,740,000-event / 4,740,000-tag-row / 370 MB** SQLite corpus was built to the schema sketched
  in `crates/happenstance-sqlite/src/event_store.rs:10-28`, with Zipf-distributed course popularity
  so that tag selectivity is a real variable (hottest tag `course:c0` = 34,159 postings; a typical
  student tag = 19). SQLite 3.50.4. Scripts: `build_db.py`, `q.py`, `q2.py`, `q3.py`, `q4.py`.
* Six compile probes against `rusqlite` 0.37 + the real `happenstance` crate, in
  `scratchpad/gat/`. All compiler output quoted below is real.

---

## Summary

The `EventStore` port is **implementable efficiently on SQLite** — but only via a physical plan
that the current documentation actively steers away from, and that every adapter author will have
to rediscover. Measured spread between the plan the contract's shape suggests and the plan that
works: **970×** (8.6 ms → 0.009 ms for a LIMIT-10 read over 2.7 M events).

Three things are broken rather than merely suboptimal:

1. `ProjectionStore::Batch<'a>` **cannot be instantiated with `rusqlite::Transaction<'a>` in either
   flavour.** The RUNBOOK's "survives with amendments" is refuted; compiler output below.
2. `EventStore` **can never gain a method with a default body** as currently declared. Proven with
   a minimal repro. This is a now-or-never, one-line fix.
3. The conformance suite **cannot fail a read-then-write adapter**, contradicting the RUNBOOK's
   phase-1 exit criterion which relies on exactly that.

---

## 1. Query pushdown

### 1.1 The query, and the arms it decomposes into

`Query::Items(Box<[QueryItem]>)`, items OR'd; each `QueryItem` is *(types OR'd)* AND *(tags
ALL-of)* (`query.rs:113-116`). Take a realistic 3-item DCB decision-model query:

```
item0  types {CourseDefined, CourseCapacityChanged}   tags {course:c1}
item1  types {StudentSubscribed, StudentUnsubscribed} tags {course:c1}
item2  types {}                                       tags {course:c1, student:s1}
```

### 1.2 The plan the sketched schema produces — and why it is wrong

The obvious translation is a materialised position set:

```sql
SELECT e.position, e.event_type, e.data, e.metadata, e.tags
FROM event e
WHERE e.position IN (
    SELECT position FROM event_tag WHERE tag = ?1
  UNION
    SELECT t0.position FROM event_tag t0
    WHERE t0.tag = ?1
      AND EXISTS (SELECT 1 FROM event_tag t1 WHERE t1.tag = ?2 AND t1.position = t0.position)
  UNION
    SELECT position FROM event_tag WHERE tag = ?2)
ORDER BY e.position;
```

`EXPLAIN QUERY PLAN`:

```
SEARCH e USING INTEGER PRIMARY KEY (rowid=?)
LIST SUBQUERY 4
  COMPOUND QUERY
    LEFT-MOST SUBQUERY
      SEARCH event_tag USING PRIMARY KEY (tag=?)
    UNION USING TEMP B-TREE          <-- materialises every matching position
    ...
```

| | time | rows |
|---|---|---|
| full read, hot course | **149.4 ms** | 34,178 |
| full read, cold course | 0.06 ms | 20 |
| same query + `LIMIT 10` | **21.7 ms** | 10 |

Two facts fall out. `UNION USING TEMP B-TREE` means the whole position set is **buffered before the
first row is produced** — the "stream a million-event replay without buffering" claim in
`store.rs:104-108` is already false at the SQL layer, before any Rust is written. And `LIMIT` does
**not** push down: 21.7 ms to produce 10 rows.

### 1.3 The plan that works

SQLite has a merge-join plan for compound `SELECT`s (`MERGE (UNION)`) that streams, respects
`ORDER BY`, and pushes `LIMIT` down — but only if every arm is a bare, already-position-ordered
index range. Any wrapping subquery or join to `event` degrades it to a co-routine over a temp
b-tree (measured: `CO-ROUTINE m` / `UNION USING TEMP B-TREE`, 8.9 ms at LIMIT 10, 365 ms at full
replay).

So the arms must be emitted as a top-level compound and the event bodies fetched **in Rust**, one
`rowid` seek per emitted row, driven by the merged position cursor.

**Recommended schema** — `event_type` becomes a covering payload column on the tag index:

```sql
CREATE TABLE event (
    position    INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type  TEXT    NOT NULL,
    data        BLOB    NOT NULL,
    metadata    BLOB,
    tags        BLOB    NOT NULL
);

-- CHANGED from the sketch: event_type is carried here so a per-item type filter
-- is a residual test on a covering index row, not a join back to `event`.
-- Key order is (tag, position) so the range stays sorted by position, which is
-- what keeps the MERGE(UNION) plan alive.
CREATE TABLE event_tag (
    tag        TEXT    NOT NULL,
    position   INTEGER NOT NULL,
    event_type TEXT    NOT NULL,
    PRIMARY KEY (tag, position)
) WITHOUT ROWID;

CREATE INDEX event_type_idx ON event(event_type, position);   -- items with no tags

-- Adapter-owned. SQLite's ANALYZE cannot supply per-value counts (see 1.4).
CREATE TABLE tag_cardinality (tag TEXT PRIMARY KEY, n INTEGER NOT NULL) WITHOUT ROWID;
```

**The read query** (arms shown for the 3-item example; `?p` is `ReadOptions::from`):

```sql
    SELECT position FROM event_tag
     WHERE tag = 'course:c1' AND position >= ?p
       AND event_type IN ('CourseDefined','CourseCapacityChanged')
UNION
    SELECT position FROM event_tag
     WHERE tag = 'course:c1' AND position >= ?p
       AND event_type IN ('StudentSubscribed','StudentUnsubscribed')
UNION
    -- multi-tag item: scan the MOST SELECTIVE tag, probe the rest
    SELECT t.position FROM event_tag t
     WHERE t.tag = 'student:s1' AND t.position >= ?p
       AND EXISTS (SELECT 1 FROM event_tag u
                    WHERE u.tag = 'course:c1' AND u.position = t.position)
ORDER BY position
LIMIT ?n;
```

Measured on the same 2.7 M corpus:

```
=== plan ===
MERGE (UNION)
  LEFT   SEARCH event_tag USING PRIMARY KEY (tag=? AND position>?)
  RIGHT  SEARCH event_tag USING PRIMARY KEY (tag=? AND position>?)
```

| arm shape | time (LIMIT 10) |
|---|---|
| typed arms, covering `event_type` | **0.009 ms** |
| untyped arms | 0.007 ms |
| multi-tag intersect, selective tag first | 0.012 ms |
| *(the sketch's join form, for comparison)* | *9.7 ms* |

Then `SELECT event_type, data, metadata, tags FROM event WHERE position = ?` per emitted position —
an integer-PK seek, and the only place the payload blobs are touched. `LIMIT` and `backwards`
(`ORDER BY position DESC`) both push straight through.

`Query::All` needs none of this: `SELECT ... FROM event WHERE position >= ? ORDER BY position` is a
plain PK scan, 0.03 ms to first 10 rows.

### 1.4 The one thing the adapter genuinely cannot get from `Query`

Multi-tag intersection is **650× sensitive to probe order**, and the ordering information does not
exist anywhere in the system:

```
self-join, unselective tag first   13.02 ms
self-join, selective tag first      0.02 ms
EXISTS,   unselective tag first    14.59 ms
EXISTS,   selective tag first       0.02 ms
```

SQLite cannot reorder for you, and `ANALYZE` cannot help: `sqlite_stat1` for `event_tag` is the
single row `'4740000 7 1'` — an *average* of 7 rows per tag. The actual values here are
`course:c0` = 34,159 and `student:s339603` = 19. Post-`ANALYZE` re-measurement confirms no change
(12.75 ms vs 0.014 ms).

`Tags` is a canonically **sorted** set (`tag.rs:258-266`) — sorted by string, which is uncorrelated
with selectivity. So the adapter must maintain its own `tag_cardinality` table and sort the probe
order itself.

**That part is the adapter's job and the contract need not change.** What the contract *should*
supply is the decomposition, because otherwise every adapter re-derives it and the second one gets
it wrong:

```rust
/// One index range an adapter can push down: at most one type, plus the tags
/// the event must carry. `Query::All` yields a single unconstrained arm.
pub struct IndexArm<'q> { pub event_type: Option<&'q EventType>, pub tags: &'q Tags }

impl Query {
    /// The cross product of items x types: the canonical set of single-range
    /// scans whose union is exactly this query. Arms may overlap, so a union
    /// must deduplicate.
    pub fn index_arms(&self) -> impl Iterator<Item = IndexArm<'_>>;
    /// Number of arms `index_arms` will yield. See `Query::MAX_PRACTICAL_ARMS`.
    pub fn arm_count(&self) -> usize;
}
```

This is purely additive — inherent methods on `Query`, no semver hazard — but shipping it in 0.1
is what makes the *fast* plan the *obvious* plan.

### 1.5 Arm count is a hard runtime limit

`arm_count` = Σ over items of `max(1, types.len())`. SQLite's `SQLITE_MAX_COMPOUND_SELECT` default
is **500**; probed on the bundled build:

```
  400 arms: ok
  500 arms: ok
  501 arms: OperationalError: too many terms in compound SELECT
```

`Query` places no bound on item count or type count. ADR/phase-3 material calls composing several
`DecisionModel`s into one query "the mechanism that makes a dynamic consistency boundary dynamic" —
i.e. the headline feature is precisely what multiplies arms. 63 composed models × 8 types each =
504 arms and the adapter fails to `prepare()` with an opaque SQLite error at runtime.

---

## 2. The append condition, atomically

### 2.1 `BEGIN IMMEDIATE` + probe (recommended)

```sql
BEGIN IMMEDIATE;                    -- acquires the write lock up front; no upgrade deadlock

-- one probe per arm of condition.fail_if_events_match; ?a is `condition.after` (0 for None)
SELECT min(position) FROM (
      SELECT min(position) AS position FROM event_tag
       WHERE tag = ?t1 AND position > ?a AND event_type IN (...)
  UNION ALL
      SELECT min(position) FROM event_tag
       WHERE tag = ?t2 AND position > ?a AND event_type IN (...)
);
-- non-NULL  -> ROLLBACK; return AppendError::ConditionViolated(at(pos))

INSERT INTO event (event_type, data, metadata, tags) VALUES (?,?,?,?);   -- per event
INSERT INTO event_tag (tag, position, event_type) VALUES (?, last_insert_rowid(), ?);
COMMIT;
```

**`conflicting_position` is free.** `min(position)` over a `(tag, position)` prefix is a single
b-tree seek to the low end of the range:

```
probe whole log, genuine conflict      0.006 ms   -> 740064
probe after = LAST (no conflict)       0.005 ms   -> None
probe after = LAST-50000               0.005 ms   -> 2690065
```

`error.rs:99-102` says adapters report `conflicting_position` "when the adapter can identify one
cheaply". On the recommended plan it is *always* cheap and always available — the caveat is more
pessimistic than reality, which is fine, but the docs should say the probe strategy gets it free.

**One real hazard.** The probe cost is unbounded in the size of the condition's tag posting list
when the *type* residual does not match early, and it runs **holding the global write lock**:

| probe | sketched schema (join to `event`) | recommended (covering `event_type`) |
|---|---|---|
| type matches first row | 0.006 ms | 0.005 ms |
| type matches nothing (34 k postings walked) | **100.3 ms** | **1.47 ms** |

100 ms under `BEGIN IMMEDIATE` serialises every writer in the process. The covering column takes
that to 1.5 ms; a second index `event_tag_typed(tag, event_type, position)` takes it to 0.005 ms if
measurement later says it matters. This is the strongest single argument for changing the schema
sketch.

### 2.2 Conditional `INSERT ... SELECT ... WHERE NOT EXISTS` (rejected)

```sql
INSERT INTO event (event_type, data, metadata, tags)
SELECT ?, ?, ?, ?
 WHERE NOT EXISTS (SELECT 1 FROM event_tag t JOIN event e ON e.position = t.position
                    WHERE t.tag = ?1 AND t.position > ?2
                      AND e.event_type IN ('StudentSubscribed','StudentUnsubscribed'));
```

Executed: `rows inserted = 0` when violated, `1` when it passes. Three problems, all contract-visible:

* No `conflicting_position` — the caller gets a boolean. `ConditionViolated::unspecified()`, always.
* `rowcount == 0` is ambiguous between "condition violated" and "nothing to insert". The contract
  distinguishes these (`AppendError::ConditionViolated` vs `AppendError::NoEvents`), so the adapter
  must re-probe to tell them apart — losing the single-statement advantage.
* It does not compose over a multi-event batch. See §3.

The contract permits either strategy; the measurements say there is no reason to pick the second.

### 2.3 Does the conformance suite catch a read-then-write adapter? **No.**

`racing_conditional_appends_elect_one_winner` (`suite.rs:596-638`) is sequential *by construction*
and says so (`suite.rs:588-595`):

```rust
let first  = store.append(core::slice::from_ref(&subscribe), Some(&handler_a)).await;
assert!(first.is_ok(), ...);
let second = store.append(core::slice::from_ref(&subscribe), Some(&handler_b)).await;
assert!(matches!(second, Err(AppendError::ConditionViolated(_))), ...);
```

An adapter that probes **outside** the transaction and then inserts passes this trivially: append #1
probes clean and inserts; append #2, running strictly afterwards, probes and sees #1. The rule can
only fail an adapter that gets the *semantics* wrong, never one that gets the *atomicity* wrong.

RUNBOOK phase 1 exit criteria state:

> Expect `racing_conditional_appends_elect_one_winner` to be the one that hurts — it is the case DCB
> exists to prevent, and the one a naive read-then-write implementation fails.

That is false. Nothing in the 27 rules exercises concurrency.

**Fix:** a second, opt-in macro, gated on the stronger bounds the parallel test genuinely needs —
which is exactly why it cannot live in the main suite:

```rust
happenstance_testkit::event_store_concurrency_conformance!(SqliteEventStore::open(dir)?);
// expands to #[tokio::test(flavor = "multi_thread", worker_threads = 8)] over
//   S: SendEventStore + Sync + 'static
// N tasks, same condition, same `after`; asserts exactly one Ok and N-1
// ConditionViolated, and that the store holds exactly one of the N events.
// Run with --test-threads and a loop count; a read-then-write adapter fails
// this within a handful of iterations and passes the sequential rule forever.
```

Flakiness is the objection CLAUDE.md's suite raises against a parallel test, and it is right — which
is why this belongs in a *separate* macro that an adapter opts into, not in the 27.

---

## 3. The self-conflict case

**The contract does not say, the reference implementation answers it silently, and the natural SQL
implementation answers it differently.**

`MemoryEventStore::append` (`memory.rs:195-221`) evaluates the condition against `stored` and then
extends:

```rust
let mut stored = self.events.write()...;
if let Some(condition) = condition {
    let conflict = stored.iter().find(|existing| { ... });   // PRE-STATE ONLY
    ...
}
let first_index = stored.len();
stored.extend(events.iter()...);
```

So the reference semantics is **pre-state**: a batch's own events never violate its own condition.
Nothing documents this, and no conformance rule pins it.

The natural SQLite implementation disagrees. Executed, with `event_tag` maintained per row:

```
first insert  rowcount = 1
second insert rowcount = 0      <-- the batch's own first event blocked its second
```

Any adapter that writes `event` and `event_tag` per event and re-probes per event — which is the
shape a trigger-maintained index or a conditional-INSERT strategy naturally produces — gets
post-state semantics and silently disagrees with the reference store.

**What it should say.** Pre-state, and for a reason stronger than "that's what the reference does":
an `AppendCondition` means *"nothing I did not see has appeared"*. The batch's own events are by
definition what the caller decided to write, so they cannot invalidate the decision that produced
them. Post-state semantics would also make `append` non-deterministic under batch reordering, since
`Event` carries no intra-batch order guarantee that the condition could be evaluated against.

Add to `AppendCondition`'s docs:

> The condition is evaluated against the store's state **before** this append. Events within the
> batch never violate their own condition, whatever order they are written in.

and a rule:

```rust
/// A batch whose own events match its condition still lands.
pub async fn condition_ignores_the_batchs_own_events<S: EventStore, F: Fn() -> S>(factory: F) {
    let store = factory();
    let condition = condition(query_of_types(&["A"]));   // "no A may exist"
    let result = store.append(&[event("A"), event("A")], Some(&condition)).await;
    assert!(result.is_ok(), "the batch's own events must not violate its own condition");
}
```

---

## 4. Streaming: is `read`'s promise deliverable?

`store.rs:102-110` promises the stream is "**lazy**: nothing is executed until it is first polled",
and that this "is what lets an adapter stream a million-event replay without buffering it".
ADR-0001 repeats it: *"Laziness follows: the query executes on first poll."*

### 4.1 The reference implementation does not honour it

`MemoryEventStore::read` (`memory.rs:150-181`) filters, orders, truncates, **collects into a `Vec`**
and returns an iterator over it — all before returning, none of it on poll. The only shipping store
is eager and fully buffered. That is defensible for an in-memory oracle, but it means the documented
property has never been tested and the suite has no rule for it.

### 4.2 Which rusqlite options actually exist

| option | verdict |
|---|---|
| self-referential (`ouroboros`) holder of `Connection`+`Statement`+`Rows` | **dead for `SendEventStore`** |
| `spawn_blocking` + bounded `mpsc` | works; pins one blocking thread + one connection per open stream |
| chunked pagination by position | works; fastest; **loses snapshot isolation** |
| `blocking` crate | same shape as `spawn_blocking`, own thread pool |

The self-referential option is dead because `rusqlite::Statement` is `!Send` — probed:

```
error[E0277]: `*mut sqlite3_stmt` cannot be sent between threads safely
  --> src\lib.rs:12:19
   |
12 |     assert_send::<Statement<'static>>();
   |                   ^^^^^^^^^^^^^^^^^^ `*mut sqlite3_stmt` cannot be sent between threads safely
note: required because it appears within the type `rusqlite::raw_statement::RawStatement`
```

The `spawn_blocking` + channel shape **does** compile against the real trait (probe 5,
`scratchpad/gat/src/lib.rs`, `Finished dev profile`). Its costs are real and worth stating in the
adapter docs: one blocking-pool thread and one pooled connection are held for the entire life of
the stream; cancellation is only observed on the *next* row, when `blocking_send` fails; and tokio's
blocking pool (default 512) becomes the ceiling on concurrently-open reads.

One footgun the non-`async` signature creates: `tokio::task::spawn_blocking` **panics** outside a
runtime — verified by test:

```
running 1 test
test spawn_blocking_outside_a_runtime ... ok      // i.e. it did panic
```

Since `read` is not `async`, the adapter cannot assume a runtime is installed at call time. It must
defer the spawn into `poll_next` (which is fine, and is what "lazy" would require anyway) — so
laziness stops being a nicety and becomes load-bearing. Worth one sentence in the adapter guide.

### 4.3 Chunked pagination is *faster*, and changes the semantics

```sql
SELECT position, event_type, data, metadata, tags
  FROM event WHERE position >= ?resume ORDER BY position LIMIT 5000;
-- resume = last_seen + 1
```

Full 2.74 M-event replay:

```
chunked, 5000/chunk (549 statements):  2.20 s
single long-lived cursor:              2.80 s
```

Chunking wins because it never holds a page-cache-hostile long read transaction, and each re-seek
is `O(log n)` against the PK. It also releases the connection between chunks, so it has no
thread-pinning ceiling at all.

**But it is not a consistent snapshot.** A concurrent append landing between chunks becomes visible
mid-stream. The `spawn_blocking` shape, holding one read transaction, *is* a snapshot.
`MemoryEventStore` is a snapshot. Nothing in the contract, the ADRs, or the 27 rules mentions read
isolation — I grepped `crates/happenstance/src`, `crates/happenstance-testkit/src` and `references/adr`
for `isolat|snapshot|consistent read|concurrent` and the only hits are `MemoryEventStore`'s own
prose and the append-side comments.

This is a **contract-level gap**, and it is not academic: it decides whether the flagship adapter
gets the 2.20 s plan or the 2.80 s one, and whether `read_decision_model` — the read half of the DCB
loop (`store.rs:198-208`) — can be torn. Two defensible answers:

* **(a) Require snapshot reads.** `read` observes the store as of first poll. Costs the chunking
  option; makes `read_decision_model`'s `(events, last_position)` pair trivially coherent.
* **(b) Permit either, and say so.** Then `read_decision_model` must document that `last` is a
  lower bound on what the caller saw — which is still *safe*, because the append condition
  re-checks. This is arguably the better answer for DCB specifically: the whole point of the
  condition is that a stale read cannot commit.

Either way, **write it down and add a rule**. Recommendation: **(b)**, plus a rule asserting that a
read started before an append does not *duplicate* or *reorder* events, which is the property
chunking could actually violate if `from` handling is off by one.

**Amend the `read` docs** so they stop promising what the reference store does not do:

> The returned stream is lazy where the adapter can make it so. Adapters MUST NOT buffer the whole
> result when a `limit` is set, and SHOULD stream unbounded reads. Whether the stream observes a
> consistent snapshot is adapter-defined; see `EventStore` isolation.

---

## 5. `Bytes` and zero-copy

`Event::data` is `bytes::Bytes` (`event.rs:185`). On the read path, **zero-copy is not achievable on
either committed adapter**, and exactly one copy per payload is unavoidable:

* **rusqlite.** `sqlite3_column_blob` returns a pointer into SQLite's own page buffer that is
  invalidated by the next `sqlite3_step`/`reset`. `row.get::<_, Vec<u8>>()` copies out; `Bytes::from(vec)`
  then takes ownership without a second copy. Total: one memcpy from the page cache. There is no
  API that hands out a lifetime-extended blob.
* **Cloudflare `SqlStorage`.** Values arrive as JS `ArrayBuffer` and cross the wasm boundary via
  `Uint8Array::copy_to`/`to_vec` — a copy by construction; `wasm-bindgen` cannot alias JS-heap
  memory into linear memory.

That does **not** make `Bytes` the wrong choice — it earns its keep in three other places, and the
crate should say which:

* **Write path.** `append(&[Event])` lets the adapter bind `event.data().as_ref()` straight into a
  SQL parameter with no copy and no clone.
* **Replication.** An envelope forwards the payload by refcount, never re-encoding it — the stated
  ADR-0003 payoff, and it holds.
* **`MemoryEventStore::read`'s snapshot** (`memory.rs:163-174`), which bumps refcounts rather than
  copying — as its docs claim (`memory.rs:29`).

The doc claim to fix is narrower than a design change: `event.rs:161-165` and `lib.rs:44-47` should
not leave a reader expecting zero-copy *reads*.

---

## 6. `ProjectionStore`'s GAT against a real transaction API

RUNBOOK decision ledger: *"Does the `ProjectionStore` port survive contact with a real transaction
API — **decided** — survives, with amendments."* **Refuted.** `type Batch<'a> = Transaction<'a>`
fails to compile in **both** flavours, for two independent reasons.

### 6.1 `SendProjectionStore` — the `Send` flavour

```
error: future cannot be sent between threads safely
  --> src\lib.rs:38:10
   |
38 |     ) -> Result<(), Self::Error> {
   |          ^^^^^^^^^^^^^^^^^^^^^^^ future returned by `commit` is not `Send`
   |
   = help: within `Connection`, the trait `Sync` is not implemented for
           `RefCell<rusqlite::inner_connection::InnerConnection>`
note: captured value is not `Send`
  --> src\lib.rs:35:9
   |
35 |         _batch: Self::Batch<'_>,
   |         ^^^^^^ has type `Transaction<'_>` which is not `Send`
note: required by a bound in `SendProjectionStore::commit::{anon_assoc#0}`
  --> D:\repos\happenstance\crates\happenstance\src\projection.rs:70:44
   |
70 | #[trait_variant::make(SendProjectionStore: Send)]
   |                                            ^^^^ required by this bound
```

(4 errors total, on `commit` and `rollback`.) Note what this proves generally: because `Batch` is a
*parameter* to two `async fn`s, the `Send` flavour transitively requires **`Batch<'a>: Send`** — a
bound that appears nowhere in `projection.rs` and that an adapter author will meet as this error
message rather than as a documented requirement.

### 6.2 `ProjectionStore` — the bare flavour fails too

Drop `Send` entirely and the shape *still* does not work, because `begin(&self)` must produce a
value borrowing from `&self`, while `Connection::transaction` takes `&mut self`:

```
error[E0515]: cannot return value referencing local variable `guard`
  --> src\lib.rs:35:9
   |
34 |         let tx = guard.transaction()?;
   |                  ----- `guard` is borrowed here
35 |         Ok(tx)
   |         ^^^^^^ returns a value referencing data owned by the current function
```

The GAT's stated justification (`projection.rs:80-82`) is exactly the thing that fails:

> Borrows from `Self` because a transaction cannot outlive the connection that opened it.

The lifetime it wants to name is the `MutexGuard`'s, not `Self`'s, and no signature taking `&self`
can name that.

### 6.3 The only shape that compiles

The batch must **own** its connection (probe 3, compiles clean):

```rust
pub struct OwnedBatch { conn: Connection }        // rusqlite::Connection IS Send

impl SendProjectionStore for SqliteProjectionStore {
    type Batch<'a> = OwnedBatch where Self: 'a;   // <-- lifetime never used

    async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
        let mut guard = self.writer.lock()...;
        let conn = guard.take().ok_or(E::Busy)?;  // checked OUT of the store
        conn.execute_batch("BEGIN IMMEDIATE")?;
        Ok(OwnedBatch { conn })
    }
    async fn commit(&self, batch: Self::Batch<'_>, id: &ProjectionId, position: SequencePosition)
        -> Result<(), Self::Error> { /* ... put the connection back ... */ }
}
```

`sqlx` is no different: `Pool::begin()` yields `Transaction<'static, DB>`, which also does not
borrow the store.

**So the GAT is unearned by any candidate adapter.** Recommendations, all cheap while the port is
still marked provisional:

1. **Drop the lifetime**: `type Batch;` A plain associated type says what is true — the batch is an
   owned handle. It also removes the `where Self: 'a` noise from every impl.
2. **Document the transitive `Send` requirement** on `SendProjectionStore::Batch`, with the error
   message above quoted, so the next author does not spend an afternoon on it.
3. **State a Drop contract, and test it.** With an owned batch, a dropped batch does not merely fail
   to roll back — in probe 3 it *permanently loses the connection*, and the store returns `E::Busy`
   forever. `rusqlite::Transaction`'s own `Drop` rolls back; a hand-rolled `OwnedBatch` must too,
   *and* must return the resource. The port must say:

   > Dropping a `Batch` without `commit` or `rollback` MUST roll back, and MUST release any resource
   > `begin` acquired. This is the crash path, and the conformance suite tests it.

   RUNBOOK phase 2 already lists "a dropped batch (the crash case) leaves both unchanged" — good;
   add "and the store remains usable", which is the failure probe 3 exhibits.
4. Reconsider `begin`/`commit`/`rollback` being `async` at all. They gain nothing for rusqlite
   (which is synchronous) and are what drags `Batch: Send` in. A synchronous `fn commit(...) -> Result<..>`
   would drop the requirement entirely — but it forecloses sqlx and any network-backed projection
   store, so on balance keep them async and document the bound.

---

## 7. Missing operations, and the semver clock

### 7.1 The finding that dominates this section

**`EventStore` as currently declared can never gain a method with a default body.** Two compile
probes, both minimal:

`trait_variant` does not wrap an `async fn` default body in an `async` block, so the body simply
does not compile:

```rust
#[trait_variant::make(SendStore: Send)]
pub trait Store {
    async fn ping(&self) -> u64;
    async fn ping_twice(&self) -> u64 { self.ping().await + self.ping().await }
}
```
```
error[E0728]: `await` is only allowed inside `async` functions and blocks
 --> src\lib.rs:5:53
  |
5 |     async fn ping_twice(&self) -> u64 { self.ping().await + self.ping().await }
  |                                                     ^^^^^ only allowed inside `async` functions and blocks
```

Hand-desugaring to `fn f(&self) -> impl Future<Output = T> { async move { ... } }` gets past that
and then hits the real wall:

```
error: future cannot be sent between threads safely
  --> src\lib.rs:4:34
   |
 4 | #[trait_variant::make(SendStore: Send)]
   |  __________________________________^
...
 9 | |     fn ping_twice(&self) -> impl Future<Output = u64> {
   | |________________________________^ future created by async block is not `Send`
note: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
  --> src\lib.rs:10:22
   |
10 |         async move { self.ping().await + self.ping().await }
   |                      ^^^^ has type `&Self` which is not `Send`, because `Self` is not `Sync`
help: consider further restricting `Self`
   |
 4 | #[trait_variant::make(SendStore: Send where Self: Sync)]
```

rustc's suggested spelling is **not valid `trait_variant` syntax** (`error: expected '+'`). The
spelling that works is a supertrait:

```rust
#[trait_variant::make(SendEventStore: Send + Sync)]     // <-- compiles
```

With that one change, all four things hold simultaneously — verified in one crate:

* the hand-desugared default body compiles;
* an adapter that does **not** override it compiles (`impl SendStore for Old`);
* an adapter that **does** override it compiles (`impl SendStore for New`);
* a `!Sync` store (`RefCell` inside — the wasm/Durable-Object shape) still implements the **bare**
  flavour, so nothing about the Cloudflare target is affected.

`Sync` on `SendEventStore` costs nothing today: every native adapter is already `Sync` (you cannot
put a non-`Sync` store behind `Arc` and share it across tasks, which is how all of them will be
used). Adding it after publication breaks every adapter. **This is a one-line, now-or-never change,
and without it `EventStore` is permanently frozen at two methods.**

### 7.2 With that fixed, what actually belongs on the trait

Measured, so the list is short and honest:

| candidate | expressible today? | cost today | cost pushed down | verdict |
|---|---|---|---|---|
| `head()` / `last_position()` | yes — `read(All, backwards, limit 1)` | 0.009 ms | 0.008 ms | **nowhere.** But it fetches `data`+`metadata` blobs to learn a `u64`; on large payloads that is a real read. Cheap default body, worth adding. |
| `exists(query)` | yes — `read(q, limit 1)`, check empty | 0.004 ms | 1.38 ms *(worse!)* | **nowhere.** The composed form is already optimal. |
| `read_backwards_first` / "latest matching" | yes — `read(q, backwards().limit(1))` | 0.008 ms | — | **nowhere.** Not a full scan; my brief's premise was wrong. A free helper in the runtime crate is enough. |
| `count(query)` | **no** — must drain the stream | **106.2 ms** | **4.66 ms** | **on the trait**, with a default body that drains `read`. 23× on the measured query, and the gap grows linearly with match count. |
| batch / multi-query read | no | — | — | **nowhere.** `Query::Items` already IS the multi-query form; a batch API would just move the union client-side. |
| subscription / tail | no | polling: **0.004 ms** | — | **nowhere for 0.1.** Polling `read(from = checkpoint+1)` costs 4 µs when nothing is new, on both `Query::All` and a 3-item query. A push API is an optimisation, not a capability. Revisit when a store with real notifications (Postgres `LISTEN`) lands. |

So the concrete additions are **`count`** and **`head`**, both with correct default bodies, both
worthless without §7.1. Everything else composes from what is already there — which is a good sign
about `ReadOptions`.

---

## 8. What I checked and judged correct

* **`append(&[Event])` is right — do not change it to `impl IntoIterator<Item = Event>`.** The DCB
  command loop retries the *same batch* after `AppendError::ConditionViolated`; taking events by
  value forces the caller to clone before every attempt, turning a zero-cost retry into a per-attempt
  allocation. For SQLite the slice costs nothing: parameters bind from `&Bytes`. `MemoryEventStore`
  pays one `Box<str>` + one `Box<[Tag]>` clone per event (`memory.rs:219-221`), which is the
  reference store's problem, not the contract's. `Event::into_parts` (`event.rs:244`) already exists
  for adapters that want to move.
* **`read` returning `impl Stream` at the top level, non-`async`.** Probe 5 confirms a real rusqlite
  adapter satisfies it and that the stream is genuinely `Send`. ADR-0001's reasoning holds.
* **`SequencePosition` as an opaque `NonZeroU64` with gaps permitted.** Maps exactly onto
  `INTEGER PRIMARY KEY AUTOINCREMENT`; the `AUTOINCREMENT` rationale in the sketch is correct.
  `ReadOptions::from` being *inclusive* maps to `position >= ?` with no off-by-one arithmetic —
  worth keeping precisely because the `after`-is-*exclusive* asymmetry in `AppendCondition` is the
  one that matters and is well tested.
* **`Tags` canonically sorted at construction.** Genuinely load-bearing: it makes the `tags` blob
  column a stable index key and `contains_all` a merge scan. (It does not help probe ordering —
  §1.4 — but that was never its job.)
* **`AppendError::ConditionViolated` lifted out of the adapter error type,** and
  `conflicting_position` being `Option`. Both correct; the recommended probe supplies the position
  for free, so the `Option` is cheap insurance for the `INSERT ... WHERE NOT EXISTS` adapters rather
  than a common case.
* **`Query` as an enum rather than `Vec<QueryItem>`.** `Query::All` genuinely needs a different
  physical plan (plain PK scan, 0.03 ms) and the enum forces the adapter to notice.
* **The two-flavour `trait_variant` design survives contact with rusqlite.** The `Send` flavour of
  `EventStore` is implementable; the problems found are in `ProjectionStore` and in default-method
  extensibility, not in the two-trait idea.
* **Polling-based projection tailing is fine.** 4 µs per empty poll; a subscription API would be
  premature.
