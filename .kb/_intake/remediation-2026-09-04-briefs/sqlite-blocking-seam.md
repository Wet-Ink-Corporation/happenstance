# Where does the blocking seam sit on the SQLite write path and the read path, and does the runtime `Handle` captured at construction get an escape hatch?

*A decision brief for ADR-0022-s9-blocking-seam. Nothing here is a decision; every
claim carries a `path:line` and every number is transcribed from
`experiments/*/results/raw/`, not from a summary.*

---

## Why this is owed

**One crate answers the same question two ways, and its own module doc says that
is the defect.** `crates/happenstance-sqlite/src/projection_store.rs:97-98`:

> That is ADR-0022 §9's decision for the event store's read path, applied here
> unchanged: **a second, different answer inside one crate is the defect.**

The event store did not apply it. Three of `SendEventStore`'s four methods take a
blocking `std::sync::Mutex` and run `rusqlite` inline, on whatever thread polled
them (`event_store.rs:1039-1042`, `:1070-1073`, `:1091-1094`).

**It is measured, with a control.** `experiments/one-connection-latency/results/raw/reactor-stall.txt`,
one `current_thread` runtime, a 1 ms `tokio::time::interval`, a second connection
holding `BEGIN IMMEDIATE` for 750 ms — well inside the 5,000 ms busy timeout, so
every arm *succeeds*:

| arm | call | max tick gap | ticks |
| --- | ---: | ---: | ---: |
| 0. idle (Windows timer resolution) | 762.6 ms | **16.177 ms** | 54 |
| 1. SHIPPED `SqliteEventStore::append` | 870.3 ms | **885.761 ms** | **5** |
| 2. CONTROL — SHIPPED `SqliteProjectionStore::commit` | 870.9 ms | **27.525 ms** | 62 |
| 3. the same append through an `in_blocking_task` seam | 778.9 ms | 34.812 ms | 54 |
| 4. SHIPPED `head()` behind an in-flight append | 873.4 ms | **717.545 ms** | 15 |
| 5. SHIPPED `read()` first poll, same contention | 793.4 ms | **639.290 ms** | 15 |
| 6. `head()` through the seam | 777.7 ms | 41.573 ms | 54 |
| 7. residual — replica `read()` first poll, after the seam | 789.4 ms | **635.056 ms** | 15 |

Arms 1 and 2 wait the same 750 ms for the same lock and one of them takes the
runtime down with it. Without arm 2 the 885.761 ms is a number with no scale.

**The two stalls have different causes, and the experiment separates them
deliberately** (`experiments/one-connection-latency/src/holder.rs:15-22`):

> Under WAL a writer does **not** block readers. So this holder blocks
> `BEGIN IMMEDIATE` — which is what `append` opens — and it does *not* block
> `SELECT max(position)`, which is what `head` and `sample_ceiling` run.

So arm 1 is SQLite's **busy handler** sleeping on the file write lock with the
process mutex free; arms 4, 5 and 7 are the **process mutex**, held by the
shipped `append` running inline. Arm 5's 639 ms is a stall the shipped `append`
inflicts on a concurrent reader inside one process, and WAL had already made it
avoidable.

**The published front page asserts the property the crate does not have.**
`crates/happenstance-sqlite/README.md:58`, which `Cargo.toml:16` makes the
crates.io page:

> `rusqlite` is synchronous, so every statement runs on a blocking task, and the
> read stream defers its `spawn_blocking` until the first poll

Only the second clause is true.

**And the crate states the principle against itself**, on the very field that
carries the handle (`event_store.rs:181-187`), explaining why §9 rejected inline:

> The rejected alternative was to run the statement inline on the calling
> thread when no runtime is found. … it keeps blocking work on an executor's
> thread whenever there *is* one, and it makes
> [`SqliteEventStoreError::NoRuntime`] unreachable.

**No clause governs any of this and two clauses guarantee none will.** CF-33
(`spec/SPECIFICATION.md:8720-8721`, `[FROZEN]`) forbids any conformance rule that
reads a clock or measures elapsed time; CF-34 (`:8747-8752`, `[PROVISIONAL]`)
puts performance outside the bar by construction. ES-36 (`:4249-4254`,
`[FROZEN]`) is the nearest clause and this adapter satisfies it *because of* the
defect: with no `.await` in `append`'s body there is nothing to interleave. The
gate cannot see this and never will.

**But a clause does govern the remedy**, which the first draft of this brief
missed entirely: ES-23 (`:3636-3660`, `[FROZEN]`) is about what a *dropped*
`append` future may have done, and the absent `.await` is this adapter's answer to
it as much as it is the cause of the stall. ES-23 also carries *"Rule: none"*, so
the gate cannot see that either — which is precisely why it has to be decided in
the record rather than discovered by a test.

---

## What is true today

### The write path runs inline — by omission, not by decision

`append` (`event_store.rs:1018-1053`) contains no `.await` anywhere:

```rust
let recorded_at = now();
let mut connection = self
    .connection
    .lock()
    .map_err(|_| AppendError::Store(SqliteEventStoreError::ConnectionPoisoned))?;

// Steps 3 and 4 are one transaction, and that is the whole of the
// atomicity claim.
Self::append_locked(
    &mut connection,
    self.store_id,
    events,
    condition,
    recorded_at,
)
```

`head` (`:1069-1078`) and `contains_event_id` (`:1090-1106`) are the same shape.
`settings` (`:406-412`) takes the seventh lock but is a synchronous `pub fn`, so
a caller already knows it blocks.

**Read `references/adr/0022-append-condition-strategy.md:387-425` before calling
this a reversal.** §9 is titled *"Consequence — the runtime seam"* and the word
`append` does not appear in it. Every sentence is about the read hop:
`SqliteEventStore::open`/`::new` capturing a handle, `ReadCursor` and
`SqliteReadStream` carrying it, and the fate of `NoRuntime`. §9 rejected inline
execution **as a fallback policy for the read path**. It never ratified inline as
the write path's normal operation, because it never looked at the write path.
The shipped `append` is inline by omission.

### The absent `.await` is also this adapter's ES-23 answer

**Added after review; the first draft of this brief did not mention ES-22 or
ES-23 anywhere, and they are the frozen clauses W2 moves.**

ES-23 (`spec/SPECIFICATION.md:3636-3660`, **`[FROZEN]`**) says the outcome of a
dropped `append` future is unspecified, that *"each adapter MUST state which of
the two it does"*, and names what it rejects, verbatim (`:3657-3660`):

> a pooled rusqlite adapter that does its work in `spawn_blocking` and presents
> itself as cancellation-safe. A dropped `JoinHandle` does not cancel the closure:
> the `COMMIT` executes, the caller is told nothing, an operator retries, and the
> payment is issued twice.

The port doc this adapter implements says the same on the method itself
(`crates/happenstance-core/src/store.rs:207-213`), calling it *"the shape that
looks cancellation-safe and is not."*

Today this adapter's answer is the strong one, by the same omission that causes
the stall: `append` contains no `.await`, so once polled it runs to completion on
the calling thread and a dropped future provably committed nothing. There is no
committed-but-unreported window. **And the crate states this nowhere** — a
repository-wide grep for `# Cancellation` in `crates/happenstance-sqlite/` returns
nothing, so ES-23's documentation obligation is unmet even while its substance is
satisfied.

### The read path deliberately does not, and cannot

`read` is not `async` (ADR-0001, ADR-0008), so it may legally be called with no
runtime in scope, where `spawn_blocking` panics — the module doc states the
forced consequence (`event_store.rs:24-28`):

> `read` runs on whatever thread called it, possibly outside any runtime, so it
> must not spawn. The spawn has to be deferred to the first `poll_next`, which by
> definition runs under an executor. **Laziness stops being a nicety and becomes
> load-bearing.**

But ES-11 requires the ceiling to be fixed *no later than the first poll*, so
`sample_ceiling` takes the mutex on the polling thread, before the hop
(`:1239-1246`):

```rust
fn sample_ceiling(&mut self) -> Result<(), SqliteEventStoreError> {
    if !matches!(self.ceiling, Ceiling::Unsampled) {
        return Ok(());
    }
    let connection = self
        .connection
        .lock()
        .map_err(|_| SqliteEventStoreError::ConnectionPoisoned)?;
```

and `poll_next:1450-1455` says why in its own comment: *"ES-11's sample, taken
**on this thread, before the spawn**."* Its doc records the failing runs that put
it there — `read_result_is_stable_under_concurrent_append` and
`query_items_share_one_snapshot` failing *"roughly one run in two"* until the
sample moved (`:1229-1233`). **The acquisition is a requirement of the clause,
not an oversight**, which is why arm 7 is 635.056 ms — 0.7% off arm 5, 39.3x the
idle floor, *after* the fix.

### The seam the audit says to transcribe

`projection_store.rs:342-361`:

```rust
async fn in_blocking_task<T, F>(&self, work: F) -> Result<T, SqliteProjectionStoreError>
where
    F: FnOnce(&mut Connection) -> Result<T, SqliteProjectionStoreError> + Send + 'static,
    T: Send + 'static,
{
    let connection = self.handle();
    let runtime = self.runtime()?;
    let joined = runtime
        .spawn_blocking(move || {
            let mut guard = connection
                .lock()
                .map_err(|_| SqliteProjectionStoreError::ConnectionPoisoned)?;
            work(&mut guard)
        })
        .await;
    match joined {
        Ok(outcome) => outcome,
        Err(join) => Err(SqliteProjectionStoreError::from(join)),
    }
}
```

**Transcribing it is not all that is required, and the reason is in the port
signature, not in the seam.** The projection store's seam is cheap because
`ProjectionStore::commit` takes `batch: Self::Batch` **by value** — the body
moves it into the closure and the only clone in it is `let id = id.clone();`
(`projection_store.rs:605-609`). `SendEventStore::append` takes
`events: &[Event]` and `condition: Option<&AppendCondition>`
(`event_store.rs:1018-1022`). `spawn_blocking` demands `'static`, so the seam has
to buy ownership the port declined to give it. The two modules are not the same
problem wearing two hats; one was handed ownership and one was not.

### What the clone costs, measured

`experiments/event-clone-allocations/results/raw/clone.txt`, steady-state
`event.clone()`, payload `Bytes::from_static` so nothing here is the payload:

| tags | owned tags (`Tags::from_pairs`) heap ops / bytes | static tags (`Tag::from_static`) heap ops / bytes |
| ---: | ---: | ---: |
| 0 | 1 / 17 | 0 / 0 |
| 1 | 3 / 48 | 1 / 24 |
| 8 | 10 / 265 | 1 / 192 |
| 32 | 34 / 1,009 | 1 / 768 |
| 64 | **66 / 2,001** | **1 / 1,536** |

The owned column is exactly `t + 2` for every `t >= 1` and **1** at `t = 0`
(`to_vec()` on an empty slice allocates nothing) — so the audit's flat "`t + 2`"
overcounts a tagless event by one. The static column is **1, flat**, because
every `Cow::Borrowed` clones as two words. A `Vec`-backed payload adds **+1 on
the first clone only** (`bytes` 1.x promotes to a shared header).

Extrapolated to this adapter's declared ceilings — `MAX_EVENTS_PER_BATCH = 256`,
`MAX_TAGS_PER_EVENT = 128` (`event_store.rs:252`, `:261`) — a worst-case owned
batch is ≈ 33,280 allocations and ≈ 1.0 MB of copied type/tag data per append.
**Extrapolated, not measured: the experiment stops at 64 tags.** The common DCB
append — one event, a handful of owned tags — is 5 to 10 allocations.

### The captured `Handle` has one door in and no door out

`event_store.rs:326-332`:

```rust
fn with_store_id(connection: Connection, store_id: StoreId) -> Self {
    Self {
        connection: Arc::new(Mutex::new(connection)),
        store_id,
        runtime: Handle::try_current().ok(),
    }
}
```

`projection_store.rs:216-222` is identical. `poll_next:1469-1475` and
`projection_store.rs:330-335` both prefer the captured handle and consult
`Handle::try_current()` only when it is `None`. `new` and `open` (`:316`, `:357`)
are the only constructors and neither takes a handle; there is no setter, no
builder, and the struct's three fields (`:159-188`) hold no path either.

### The off-tokio fact the tree already records — **restated; the first version was wrong**

An earlier draft of this section read the blocking-emitter paragraph
(`crates/happenstance-sqlite/tests/concurrency.rs:27-33`) and concluded: *"A store
whose `read` needs tokio is already unusable off tokio."* **That claim has been
removed, because the same file contradicts it twenty lines below and the brief
had cited only the half that agreed.** It was the sole ground on which W3's
benefit was priced at zero, so removing it removes the argument that W2 beats W3;
that argument is struck from the recommendation rather than rewritten around.

What the tree actually says is that **the binding constraint is the construction
site, not off-tokio-ness.** Two committed, green tests separate them:

* `store_serves_a_bare_thread_with_no_ambient_runtime`
  (`tests/concurrency.rs:96-144`) builds the store **inside** a
  `#[tokio::test(flavor = "multi_thread")]`, then drives it from a bare
  `std::thread::scope` thread on which `Handle::try_current().is_err()` is
  *asserted* — and **both `append` and `read` succeed** there. `read` succeeds
  because §9's captured handle serves the `spawn_blocking`; `append` succeeds
  because it runs inline. Its own doc states the distinction the earlier draft
  erased (`:86-89`): the test *"exercises both call paths, because they are two
  different sites: `append` takes the connection mutex on the calling thread,
  while `read` defers a `spawn_blocking` into `poll_next`."*
* `a_store_with_no_runtime_anywhere_reports_no_runtime` (`:159-178`) is the case
  that fails: a store both **constructed and driven** with no runtime anywhere
  reports `NoRuntime` from `read` — and, today, still serves `append`, `head` and
  `contains_event_id`.

The blocking emitter is declined for the second reason, not the first: the
*fixture* is built off tokio, so the handle captures `None`. Off-tokio use of a
runtime-constructed store is supported, tested and documented. The capability W2
would remove is therefore narrower and more real than the earlier draft claimed:
**a store constructed with no runtime anywhere can today append, `head` and
`contains_event_id`** — a synchronous consumer's write path, which W3 preserves
and W2 does not. It is a documented capability, not an unspendable one.

### Governing records, and what is immutable

| record | what it says | status |
| --- | --- | --- |
| `.kb/decisions/0022-append-condition-strategy.md:5` | `status: accepted` — immutable; its summary carries §9: *"The runtime seam captures a tokio Handle at construction with try_current as fallback so SqliteEventStoreError::NoRuntime keeps a real meaning."* | **accepted** |
| `references/adr/0022-append-condition-strategy.md:387-425` | §9, the long form. Read path only; `append` unmentioned | — |
| `:609-612` | §9's own falsifier (below) | — |
| `.kb/decisions/0011-read-laziness-and-isolation.md` | ADR-0011, the ceiling-first mechanism ES-11 rests on | accepted |
| `spec/SPECIFICATION.md:2976-2981` | **ES-11**, *A read is a snapshot* — **`[PROVISIONAL]`**, axis transport | commentary at `:2996-3004` names §9's capture-and-prefer as what settled the seam |
| `:3345-3353` | **ES-17**, *The batch is borrowed, not owned* — **`[PROVISIONAL]`**, falsified by a per-event-clone measurement on a real adapter | see the recommendation |
| `:3608-3634` | **ES-22**, *Dropping an `append` future leaves no partial batch* — **`[FROZEN]`**, rule `dropped_append_future_leaves_no_partial_batch` | satisfied under W1 and W2 alike: the transaction is what bounds it |
| `:3636-3660` | **ES-23**, *Cancellation outcome is unspecified* — **`[FROZEN]`**. Names *"a pooled rusqlite adapter that does its work in `spawn_blocking`"* as what it rejects; `store.rs:194-213` repeats it on the port method | **this adapter's answer flips under W2** — see the recommendation. No conformance rule can see it (*"Rule: none"*, `:3655`) |
| `:4249-4254` | **ES-36** — `[FROZEN]` | satisfied today because `append` never suspends |
| `:8720-8721` / `:8747-8752` | **CF-33** `[FROZEN]` / **CF-34** `[PROVISIONAL]` | the gate cannot see any of this |
| `:4307-4330` | the third obligation *"a `read` issued while an `append` on the same handle is suspended must complete"* — enforced by a rule, stated by no clause | becomes live only if `append` gains a suspension point |

### Did §9's falsifier fire?

`references/adr/0022-append-condition-strategy.md:609-612`, verbatim:

> **§9, the runtime seam.** Re-open it if a deployment shows the captured
> `Handle` costing something the inline path does not, or if
> `postgres-and-neon-stores` finds the seam generalising — in which case it
> becomes a testkit question rather than an adapter one.

**Half fired, and not by the mechanism the falsifier names.**

* **J-2 does not fire it.** J-2 is about the *write* path, which §9 never
  considered. This is a widening of an accepted decision's **scope**, not a
  disagreement with its **verdict**.
* **F2-3 identifies exactly the cost the falsifier describes** — a `Handle`
  captured in one runtime and served from another is a cost the inline path
  cannot have, because there is nothing to go stale. But the falsifier's
  condition is *"a deployment shows"*, and no deployment has. Nothing in
  `experiments/` builds a store inside `Runtime::new().block_on(…)`, drops the
  runtime, and observes `read`. F2-3's `Worker(task was cancelled)` claim is
  reasoned from tokio's semantics, not reproduced in this tree.
* This repository's own bar — *"name a plausible wrong implementation it
  rejects, and write that implementation"* (`CLAUDE.md`) — says the reproduction
  is owed before the remedy. It is roughly twenty lines and it is the cheapest
  item in this brief.

---

## Options

Three sub-decisions. They are separable and the evidence for them is of three
different qualities, so they are listed apart. Deciding them in one record is
right; deciding them by one argument is not.

### W — the write path: `append`, `head`, `contains_event_id`

**W1 — leave them inline; correct `README.md:58` only.**

* *Costs a caller:* the 885.761 ms stall stays, on the flavour
  `examples/transfers-on-sqlite` demonstrates and `#[tokio::test]` defaults to.
  Every timer, every I/O completion and every unrelated task on a
  `current_thread` runtime stops for the whole call, up to `BUSY_TIMEOUT_MS =
  5_000` (`connection.rs:62`) in the worst case. Nothing in the gate can see it.
* *Costs an adapter author:* nothing on the port; it publishes the crate with two
  answers to one question and the module doc naming that as the defect.
* *Semver:* none.
* *Forecloses:* nothing structurally. The README correction is one-shot at 0.2.0.

**W2 — route all three through `in_blocking_task` (the audit's remedy).**

* *Costs a caller:* the batch and the condition must be cloned to reach `'static`
  — measured above; single digits typically, ≈33k allocations at the declared
  ceilings. And **the clone is paid on the rejection path too**, because the
  condition can only be evaluated inside the transaction, i.e. inside the
  closure, i.e. after the clone. Also: `append`, `head` and `contains_event_id`
  begin to **require a reachable runtime** where today they require none — which
  bites a store *constructed* with no runtime anywhere
  (`tests/concurrency.rs:159-178`), not a store merely *driven* off tokio
  (`:96-144`, green today on both call paths).
* *Costs a caller, the one the first draft omitted:* **W2 flips this adapter's
  ES-23 answer.** Today a dropped `append` future provably committed nothing;
  under W2 a dropped `JoinHandle` does not cancel the closure, the `COMMIT` runs
  and the caller is told nothing. That is the exact shape ES-23 `[FROZEN]` names
  as what it rejects (`spec/SPECIFICATION.md:3657-3660`) and the port method's
  own `# Cancellation` section calls *"the shape that looks cancellation-safe and
  is not"* (`store.rs:207-213`). The window is as wide as the stall W2 removes —
  up to `BUSY_TIMEOUT_MS = 5_000` — and it opens for precisely the callers W2 is
  for, the ones using `timeout`/`select`, who drop futures. ES-22 still holds
  (the transaction bounds partiality); what changes is the promise ES-23 governs.
  W2 is not forbidden by ES-23 — an adapter MAY commit a dropped append — but it
  MUST then say so, and the crate has no `# Cancellation` section to say it in.
* *Costs an adapter author:* nothing on the port. It makes *"a synchronous driver
  means the adapter owns a runtime"* the crate's single answer, which is what
  `projection_store.rs:91-101` already claims in prose.
* *Semver:* **none on any signature.** A behavioural narrowing (three methods
  gain a reachable `NoRuntime`) and a new suspension point in `append`, which
  makes ES-36's `interleaved_appends_on_one_handle_elect_one_winner` stop being
  trivially satisfied and `spec/SPECIFICATION.md:4307-4330`'s unstated third
  obligation live for the first time on this adapter.
* *Forecloses:* nothing on the read path — arm 7 says the residual survives it
  unchanged. It does put weight on ES-17 (see the recommendation).
* *Measured:* 885.761 → **34.812 ms** and 717.545 → **41.573 ms**, both within
  1.5x of the projection store's own 27.525 ms.

**W3 — W2 with an inline fallback when no runtime is reachable.**

Hop if a `Handle` is available; run inline if not. §9's rejected option (b), demoted
from policy to fallback.

* *Costs a caller:* the clone can be skipped on the inline branch, so the
  allocation cost becomes conditional. `append` on a store constructed with no
  runtime anywhere keeps working — the capability `tests/concurrency.rs:159-178`
  exercises. It also keeps the strong ES-23 answer *on that branch only*, which
  is a two-answer cancellation contract and has to be documented as one.
* *Costs an adapter author:* two paths to test per method, and the crate now has
  a *third* answer — the projection store fails where the event store falls back
  — unless the projection store gets the same treatment, which is a wider change
  than the audit describes.
* *Semver:* none on signatures. `SqliteEventStoreError::NoRuntime` becomes
  **unreachable from three of four methods**, which is precisely §9's second
  ground for rejecting (b). §9's *first* ground does not apply, because the
  fallback fires only when there is no executor to starve.
* *Forecloses:* it contradicts the summary sentence in the accepted atom
  `kb-decision-0022`, so unlike W1 and W2 it requires a **superseding** atom
  rather than an amending one.

**W4 — `tokio::task::block_in_place` instead of `spawn_blocking`.** Named and
rejected here so nobody re-proposes it: it takes a borrowing closure and would
cost no clone at all, and it **panics on a `current_thread` runtime** — the exact
flavour every arm of the measurement uses.

**W5 — a `try_lock` heuristic: run inline when the mutex is free.** Also
rejected on the evidence, not on taste: in arm 1 the process mutex *was* free.
The 885.761 ms is SQLite's busy handler on the file write lock
(`experiments/one-connection-latency/src/holder.rs:15-22`), which a `try_lock`
cannot predict.

### R — the read path's ceiling sample (the J-2R residual)

**R1 — leave it.** `read()`'s first poll stays at ~635 ms under a concurrent
append, 39.3x the idle floor. *Costs a caller:* the stall, on the same
`current_thread` flavour, up to 5,000 ms in the arithmetic worst case. *Costs an
adapter author:* nothing. *Semver:* none. *Forecloses:* nothing, but the fix is
not free later either — see cost of delay.

**R2 — cooperative re-poll:** return `Poll::Pending` from `sample_ceiling` and
wake when the mutex frees. *Costs a caller:* nothing visible in signatures; the
wait is not eliminated, only converted from a hard stall into a scheduling
question. *Costs an adapter author:* `SqliteReadStream`'s polling contract
changes and it needs a waker registration this adapter does not have.
*Semver:* none on the surface; a behavioural change to a `Stream` impl.
*Forecloses:* it makes `std::sync::Mutex` the wrong primitive for the
connection, which is a larger change than it sounds.

**R3 — a second connection dedicated to the ceiling `SELECT`.** WAL means a
writer does not block readers (`holder.rs:15-22`); the only reason these
`SELECT`s serialise is that **the adapter** serialises them. *Costs a caller:*
one extra file descriptor and connection per store. *Costs an adapter author:*
the store must carry an origin it does not have — `event_store.rs:159-188` has
three fields and none is a path, which is the *same* missing field that
`SqliteFixture::connect` works around by calling `SqliteEventStore::open(&self.path)`
itself (`tests/support/mod.rs:208-210`). *Semver:* additive if it arrives as a
new constructor; **breaking if `open` starts opening two connections and callers
counted them**. *Forecloses:* nothing, and it is the option that also unblocks
the J-3+J-4 `connect()` question, which is why it should not be decided in
isolation from it.

### H — the escape hatch (F2-3)

**H1 — leave the capture unconditional.** *Costs a caller:* a store built in one
runtime and served from another `append`s and `head`s perfectly (they run inline)
and every `read` hangs or yields one `Worker` item and terminates, while
`NoRuntime` — the variant documented for exactly this — is unreachable
(`event_store.rs:876-882`). In `projection_store.rs` the same capture gates
*every write*: a checkpoint that never advances, reported as `Worker(JoinError)`,
which a projection runner reads as transient and retries forever. *Costs an
adapter author:* nothing. *Semver:* none. *Forecloses:* nothing today; after
0.2.0 the *default* is frozen.

**H2 — add an explicit-`Handle` door, in both modules.** A constructor or builder
that accepts a `Handle`, and one that declines to capture. *Costs a caller:*
nothing — opt-in, and the existing constructors keep their behaviour. *Costs an
adapter author:* nothing; crate-local. *Semver:* **additive**, now and later.
*Forecloses:* almost nothing. The one thing it must not do is land only in
`event_store.rs`: `projection_store.rs:220` is the writing half and the one that
retries forever.

---

## Recommendation

**This recommendation flipped under review. It previously read "W2 + H2"; it now
reads "W1 now, W2-or-W3 re-decided on evidence, H2 unchanged." What flipped it is
recorded in the revision record at the foot, and in the two paragraphs below.**

### W — the write path. **W1 now: take the README correction, and only that.**
### W2 is not rejected; it is unripe.

**Why this is no longer W2.** Two of the three legs the earlier W2 recommendation
stood on are gone, and neither was load-bearing for the *problem*, only for the
*remedy*:

1. **W2's decisive cost was never named.** W2 flips this adapter's ES-23
   `[FROZEN]` answer from *"a dropped `append` committed nothing"* to *"may have
   committed; the caller is told nothing"*, on a crate carrying no
   `# Cancellation` section, in the exact shape ES-23 and
   `happenstance-core`'s own port doc single out as the trap. The first draft's
   W2 cost bullets said *"Semver: none"* and priced the change as a clone plus a
   reachable `NoRuntime`; a safety contract was missing from the ledger. It is a
   0.2.0-dated item in the same way the README is — see the cost-of-delay table.
2. **The argument that W2 beats W3 rested on a false fact and has been struck.**
   It said off-tokio `append` was *"a per-method capability no consumer can
   spend."* `tests/concurrency.rs:96-144` is a committed green test asserting the
   opposite, and `:159-178` shows the capability W2 actually removes. With that
   ground removed there is no surviving argument in this brief for preferring W2
   to W3 — not an argument that W3 wins, an *absence* on the question.

**Why W1 rather than "do nothing".** The one dated item is real and is not the
seam: `crates/happenstance-sqlite/README.md:58` asserts a property the crate does
not have, and `Cargo.toml:16` makes it the crates.io page at 0.2.0. Correcting the
sentence is one-shot, costs nothing, and is correct under *every* outcome of the W
question. Ship it now. The same change should add the `# Cancellation` section
ES-23 requires and the crate lacks, stating today's answer — *a dropped `append`
future commits nothing* — because that is true today, it is cheap, and it makes
the flip visible if W2 later lands.

**What W1 does not claim.** It does not defend inline execution. 885.761 ms
against a same-crate control of 27.525 ms under identical contention is a real
defect, the shipped behaviour is inline *by omission* (§9, `0022:387-425`, never
mentions `append`), and nothing here says that should stand. W1 is *"not yet"*,
not *"no"*: the seam changes no signature and stays free to land at 0.3.0, which
is the same cost-of-delay row that made W2 look free — read in the other
direction.

**What is owed before W is re-decided**, and both items are cheap:

1. **The measurement this brief already said was owed** (below, unchanged):
   uncontended `append` latency, inline against seam, at `t ∈ {0, 8, 64}` tags and
   batch `∈ {1, 256}`, on the accepted *and* the rejected path.
2. **An explicit ES-23 answer for each of W2 and W3**, written as the
   `# Cancellation` section it would ship with. W3's is two answers and that is
   itself a cost; W2's is one, and it is the weaker one.

W4 and W5 stay rejected on measured mechanism rather than on taste:
`block_in_place` panics on the `current_thread` flavour every arm uses, and
`try_lock` cannot see the busy handler that produced arm 1. That part of the
earlier recommendation is unaffected.

**The strongest arguments against this position**, both verbatim from the review
that produced the flip, are the two that would push it back toward W2 now:

> Flip to W1 now, not forever. … Take the README fix (the one dated item);
> re-decide W2 against ES-23 plus the measurement the brief itself says is owed.

and, against W1's own quietism, the brief's own arm 1 and arm 2: a caller on
`current_thread` — the flavour `examples/transfers-on-sqlite` demonstrates and
`#[tokio::test]` defaults to — loses every timer and every I/O completion for up
to 5,000 ms today, and W1 leaves that in place for however long the measurement
takes. If the measurement is not actually taken, W1 decays into W1-forever by
default, which is the outcome nobody is arguing for.

**The strongest argument against W2 remains, in the specification's own words**
(`spec/SPECIFICATION.md:3373-3374`, ES-17's second ground for the borrow):

> A rejected append clones **nothing** — `memory.rs:377-383` returns before the
> `extend` — and rejection is the routine outcome under contention.

W2 breaks that ground on this adapter. `spawn_blocking` needs `'static`, so the
clone happens *before* the transaction opens and therefore before the condition
is evaluated: under W2 a rejected append clones the whole batch, and rejection is
the routine outcome under exactly the contention W2 exists to fix. W2 pays an
**unconditional** cost to remove a **conditional** one, and the unconditional
cost has not been measured in wall-clock terms — arm 3's 778.9 ms call is
dominated by the 750 ms holder and says nothing about uncontended throughput.

Two consequences follow and both belong in the decision rather than in a comment:

1. **The missing measurement is small and worth taking first.** Uncontended
   `append` latency, inline against seam, at `t ∈ {0, 8, 64}` tags and batch
   `∈ {1, 256}`, on the accepted *and* the rejected path. Nothing in
   `experiments/` measures the rejected path's clone.
2. **W2 is ADR-0012's falsifier item 1 arriving from an unexpected direction.**
   `0022:522-537` declined to lift ES-17 because *"Three candidate stores in an
   experiment crate are **not** two builds of the same adapter."* An inline
   `append` and a seam `append` in this crate **are** two builds of the same
   adapter differing in how the batch is owned. That is the measurement ES-17's
   `[PROVISIONAL]` marker names — and its outcome would bear on a
   `happenstance-core` port signature, i.e. on a frozen port, i.e. not on this
   record's authority. Record the connection; do not act on it here.

### H2 — add the explicit-`Handle` door, in both modules. Recommended, with the
### audit's urgency corrected.

Cost to callers is zero, cost to adapter authors is zero, and the projection
store's failure mode — a checkpoint that never advances, reported as a transient
`Worker(JoinError)` and retried forever — is the sharp end. But the audit calls
this *"the least dated item in the section"* and it is right: **a constructor is
additive now and additive after 0.2.0**, so the timing pressure the routing
implies is not there. What *is* dated is the **default**: `new` and `open`
capturing unconditionally becomes permanent at 0.2.0, and changing it afterwards
is breaking.

**And the reproduction is owed before the constructor.** §9's falsifier asks for
*"a deployment"*; the tree has none, and F2-3's failure mode is reasoned rather
than observed. A test that builds a store inside `Runtime::new().block_on(…)`,
drops the runtime, and asserts what `read` and `commit` do is twenty lines, and
it converts this from a hazard to a fact. If it does not fail the way F2-3
predicts, H2 is a constructor solving nothing.

### R — the read path. **No recommendation. The evidence does not favour either
### option and a forced choice here would be worse than none.**

* Nothing has measured R2 or R3. Arm 7 measures the *problem* surviving W2; no
  arm measures either remedy.
* R2 does not remove the wait, only reschedules it, and it needs a waker
  registration `SqliteReadStream` does not have — so its cost is a rewrite of a
  state machine whose current shape is defended by two named failing tests
  (`event_store.rs:1229-1233`).
* R3 is entangled with a decision this record does not own. It needs the store to
  carry its path — which is the same missing field the J-3+J-4 `Clone`/`connect()`
  question needs, and that question has a different owner, a different semver
  class (breaking) and a harder deadline (0.2.0). Deciding R3 here would settle
  J-3+J-4's premise by side effect.

What is owed instead: **name R as a residual with an owner and a measurement**,
and record that it bears on ES-11's `[PROVISIONAL]` marker from a second
direction. ES-11's stated falsifier is a one-shot-HTTP transport that cannot
sample in one round trip; this is a *native* adapter finding the same obligation
expensive, which is worth having on the record where that marker is next
reviewed.

### Which atom this supersedes

**Under the recommendation as it now stands (W1 + H2): none**, and trivially so —
W1 changes no behaviour at all. The analysis below is retained because it is the
live question the moment W is re-decided.

**Under W2: none.** `kb-decision-0022`'s summary
sentence — *"The runtime seam captures a tokio Handle at construction with
try_current as fallback so `SqliteEventStoreError::NoRuntime` keeps a real
meaning"* — stays true word for word. Capture-at-construction stays,
`try_current` stays the fallback, and `NoRuntime` keeps a real meaning; under W2
it becomes reachable from *more* methods, not fewer. The fit is ADR-0029's
precedent, which amends ADR-0004 without superseding it and says so in its own
frontmatter (`0029: supersedes: null`; *"This amends ADR-0004 rather than
superseding it - that decision's body stays verbatim"*).

**Under W3 it is `kb-decision-0022`, and the fit is ADR-0032's precedent** — *"A
repair of ADR-0021, not an amendment … What is withdrawn is one justification for
one rejected alternative"*, with `supersedes: [kb-decision-0021]`. W3 makes
`NoRuntime` unreachable on three methods, which is the clause of ADR-0022's
summary that would become false.

So: **the supersession question is decided by which option wins, not before it.**
Either way `references/adr/0022-append-condition-strategy.md` is not edited —
`.kb/decisions/0022` is `status: accepted` and `redkiln validate --kb` checks
accepted atoms against `HEAD`. `spec/SPECIFICATION.md` cites §9 by section number
(`:3000`) and not by line range, so a new record disturbs no `spec-trace`
citation; what it *does* disturb is `:2996-3004`'s prose, which describes the
seam as read-path-only and would need a clause-adjacent amendment under W2.

---

## Cost of delay

**Not uniform across the three, and the audit's routing flattens them.**

| item | free now? | permanent at 0.2.0? | equally cheap forever? |
| --- | --- | --- | --- |
| **W2** the write-path seam | yes | **the README is** | **the code is**. `README.md:58` ships with the crate at 0.2.0 and is what an evaluator reads first; the seam itself changes no signature and stays free to land at 0.3.0 |
| **W2's behavioural narrowing** (three methods gain `NoRuntime` when the store was *constructed* with no runtime) | yes | **yes** | no. Before publication it is a fact about an unpublished crate; after it is a behaviour change to a published one |
| **W2's ES-23 flip** (a dropped `append` may have committed) — *added after review; the first draft's table omitted it* | yes | **yes** | **no**, and this is the row that matters most. It is a safety contract, dated at 0.2.0 exactly like the README: before publication it is a property nobody relies on, after it is a promise being withdrawn from callers who read the `# Cancellation` section — which the crate must first acquire |
| **H2** the explicit-`Handle` door | yes | no — **additive forever** | **yes**. The audit's own semver paragraph says so |
| **H's default** (`new`/`open` capture unconditionally) | yes | **yes** | no. Changing the default after 0.2.0 is breaking |
| **R2 / R3** | yes | R3 partly — a second connection inside `open` is breaking after 0.2.0 if it arrives that way rather than as a new constructor | no |
| the F2-3 **reproduction test** | yes | n/a | **yes**, and it costs an hour |

**Two** items are genuinely dated, and the first draft named only one. The README
is dated by publication rather than by design:
`crates/happenstance-sqlite/README.md:58` asserts a property the crate does not
have, and 0.2.0 is when that assertion becomes a public one. It is correctable in
either direction — fix the sentence, or fix the code — and those are not the same
cost. The second is the **cancellation contract**: whichever answer this adapter
gives ES-23, 0.2.0 is when it becomes a promise, and the crate currently gives
none in writing. That makes *writing down today's answer* part of the same
one-shot README change, and it is why the recommendation now takes W1 first: it
lets the dated documentation be corrected without also spending the frozen
clause's answer on an unmeasured remedy.

---

## What this does not settle

* **Whether `Clone` survives on `SqliteEventStore` and whether the two
  `ConnectionPoisoned` variants are deleted.** That is J-3+J-4, it is
  **breaking**, and it is genuinely dated at 0.2.0 — unlike anything in this
  brief. R3 needs its premise (the store carrying its path) and must not settle
  it by side effect.
* **Whether ES-17's `&[Event]` marker is lifted.** W2 supplies ADR-0012's
  falsifier item 1 for the first time, and the answer is a `happenstance-core`
  port change against a frozen `EventStore`. Not this record's authority; record
  the connection and route it.
* **Whether ES-11's `[PROVISIONAL]` marker moves.** This is a second, native
  instance of the clause's obligation being expensive, against a marker whose
  stated falsifier is a one-shot-HTTP transport. Evidence for the next review of
  that marker, not a verdict on it.
* **Whether `spec/SPECIFICATION.md:4307-4330`'s unstated third obligation becomes
  a clause.** W2 gives `append` its first real suspension point on this adapter,
  which makes that obligation live here for the first time. The clause it would
  amend is `[FROZEN]`.
* **Whether `PAGE_SIZE = 512` stays** (`event_store.rs:141` calls it *"a
  placeholder until it is measured"*). It is the aggregate lock-hold knob and it
  is a different measurement, already reported at
  `experiments/one-connection-latency/results/page-lock-hold.md`.
* **Whether the seam generalises to `postgres-and-neon-stores`,** which §9's
  falsifier names as the trigger that turns this into a testkit question rather
  than an adapter one. Neither adapter has a body.
* **The uncontended cost of W2.** No arm measures it. It is the one measurement
  this decision would be visibly better for having, and it is cheap.
* **W2 against W3, on the merits.** The brief no longer contains an argument
  either way: the one it had was struck as false (see the revision record). W is
  re-decided when the uncontended measurement and the two candidate
  `# Cancellation` sections exist.

---

## Revision record

Three changes, all made after adversarial review of the first draft. Two critiques
were accepted in full; the recommendation flipped.

**1. A falsified premise, and the claim resting on it, removed.** The first draft
asserted *"A store whose `read` needs tokio is already unusable off tokio"*, citing
`crates/happenstance-sqlite/tests/concurrency.rs:27-33`. It had read only the half
of that file which agreed with it. `tests/concurrency.rs:96-144`
(`store_serves_a_bare_thread_with_no_ambient_runtime`) is a committed green test
asserting the opposite: on a bare thread where `Handle::try_current().is_err()` is
itself asserted, **both `append` and `read` succeed** on a store constructed inside
a runtime, and the test's own doc (`:86-89`) states the distinction the draft
erased. The blocking emitter is declined because the *fixture* is built off tokio
— the construction site — not because `read` needs an ambient runtime. The claim
is deleted, and with it the argument it was the sole support for: *"W2 beats W3
because W3's benefit — off-tokio `append` — is worth nothing here."* That
argument is **struck, not rewritten**; the brief now says it has no position on
W2 against W3, and the *"What this does not settle"* list says so too. The
capability W2 removes was also understated and is now stated correctly: a store
constructed with no runtime anywhere can today `append`, `head` and
`contains_event_id` (`:159-178`), which is documented and tested, not
unspendable.

**2. The recommendation flipped from "W2 + H2" to "W1 now + H2", and what
flipped it.** W2's decisive cost was never named in the first draft: it flips this
adapter's **ES-23 `[FROZEN]`** answer from *"a dropped `append` provably committed
nothing"* — true today, because `append` contains no `.await` — to *"may have
committed; the caller is told nothing"*, which is the shape ES-23
(`spec/SPECIFICATION.md:3657-3660`) and the port method's own `# Cancellation`
section (`crates/happenstance-core/src/store.rs:207-213`) name as the trap, on a
crate that has no `# Cancellation` section at all. The window it opens is as wide
as the stall it removes (`BUSY_TIMEOUT_MS = 5_000`) and it opens for exactly the
callers W2 exists to serve, who use `timeout`/`select` and therefore drop futures.
ES-22 and ES-23 appeared nowhere in the first draft — not in the records table,
not in W2's cost bullets, not in the cost-of-delay table, which said *"Semver:
none"* and *"free to land at 0.3.0"*. All four now carry it. W2 is **not
rejected**: it is unripe, pending the uncontended measurement the brief already
said was owed plus a written ES-23 answer for each of W2 and W3. What is taken now
is the one dated item — the `README.md:58` correction — plus the `# Cancellation`
section stating today's answer.

**3. Both objections are recorded under Recommendation**, the first verbatim, and
the strongest counter-argument *to the new position* is stated alongside it: W1
leaves an 885.761 ms `current_thread` stall in place, and decays into W1-forever
if the measurement is never taken.

**Unchanged:** the H2 recommendation and its urgency correction; the R
non-recommendation; W4 and W5's rejections; every measurement, table and
`path:line` citation in *"What is true today"* apart from the off-tokio section
named above.
