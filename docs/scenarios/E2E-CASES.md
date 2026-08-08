# End-to-end test cases

The test-case specification derived from the six scenarios in
[`README.md`](README.md). Every case is traced to the scenario that produced it,
stated concretely enough to implement, and carries two things the repository's own
standing rule demands:

> **A rule that no adapter can fail is decorative.** Before adding one, name a
> plausible wrong implementation it rejects, and write that implementation into
> the testkit's own `tests/` if one does not already exist there. — CLAUDE.md

So each case states **what it falsifies** — the design claim shown false if the
case fails — and **what it rejects** — the specific wrong implementation that
would pass everything else in the workspace and fail this. A case with no named
wrong implementation is not in this list.

## Levels

| Level | Where it belongs | What it may assume |
|---|---|---|
| **contract** | `happenstance-testkit`'s conformance suite, run against every adapter | Only the ports. No domain, no second crate. |
| **integration** | a workspace e2e test crate, `tests/e2e/`, not yet created | Two or more adapters wired together; a domain vocabulary invented for the test. |
| **scenario** | a worked example under `examples/` | A whole deployment, run for a human to read. |

A contract-level case that cannot be expressed against a single store handle is
still contract-level if it belongs in `happenstance-sync-testkit` — that crate is
named in `crates/happenstance-sync/src/lib.rs:9-11` and does not exist. Those are
marked **⚠ crate does not exist**.

**⚠ `happenstance`** means something narrower, because that crate *does* exist.
After [ADR-0006](../adr/0006-bare-name-to-the-typed-layer.md) the bare name
belongs to the **typed** layer, which is a facade over `happenstance-core` today:
the crate is on disk, the surface these cases need is not. The contract — ports,
types, errors, in-memory store — is `happenstance-core`, and that is what a
**Spans** line means when it names it.

## Index

| Group | Cases |
|---|---|
| [A — Event store semantics](#a--event-store-semantics) | E2E-01 … E2E-14 |
| [B — Projections and the runner](#b--projections-and-the-runner) | E2E-15 … E2E-32 |
| [C — Sync and convergence](#c--sync-and-convergence) | E2E-33 … E2E-45 |
| [D — Lifecycle and evolution](#d--lifecycle-and-evolution) | E2E-46 … E2E-51 |
| [E — Edge and `!Send`](#e--edge-and-send) | E2E-52 … E2E-56 |
| [What cannot be written yet](#what-cannot-be-written-yet) | — |

---

## A — Event store semantics

### E2E-01 — Once a position is observed, nothing below it appears later

- **From:** Wattline (the 19-millisecond window); Norvant (all fifteen tower projections); Turnstile (chunked backfill); Kestrel Motor (`AuthoriseRepair` on the hub)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`; ⚠ needs a hostile fixture and, for a real adapter, `happenstance-postgres` (does not exist)

**GIVEN** a store and a reader that has observed an event at position *P*.
**WHEN** any subsequent read is issued, at any later time, with any `ReadOptions`.
**THEN** no event at a position ≤ *P* is yielded that was not yielded before.

**Falsifies:** that `AppendCondition::after` enforces a consistency boundary.
`is_violated_by` compares position *values* (`crates/happenstance-core/src/append.rs:95-107`)
and nothing in the contract requires an event becoming visible later to carry a
higher position. `crates/happenstance-core/src/event.rs:92-97` documents uniqueness,
monotonicity and permitted gaps — all properties of *assignment*, none of
*visibility*.

**Rejects:** a Postgres adapter allocating positions with `nextval()` outside the
transaction. It passes `positions_are_unique` and
`positions_are_strictly_monotonic` (`crates/happenstance-testkit/src/suite.rs:1106-1162`)
because both read a quiescent store after a single sequential writer. The
deterministic fixture that must fail is a store that holds one append's row back
until a second, later-positioned append has committed.

---

### E2E-02 — A read's result set is stable under concurrent append

- **From:** Turnstile (`tour-fan-ledger`'s 480-statement paginated backfill); Wattline
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`; ⚠ `happenstance-neon` (does not exist)

**GIVEN** a store holding events 1…*N* and a read stream over `Query::all()` that
has been polled once.
**WHEN** further events are appended before the stream is drained.
**THEN** the store's documented answer is honoured — either the stream yields
exactly 1…*N*, or the contract states that it may grow and says what a caller may
conclude from the last position it saw.

**Falsifies:** that a checkpoint derived from a streamed read names a complete
prefix. `crates/happenstance-core/src/store.rs:101-121` specifies laziness, ordering
and inclusivity and says nothing about isolation.

**Rejects:** a self-paginating adapter over one-shot HTTP that issues one
independent `WHERE position > $last ORDER BY position LIMIT 5000` statement per
chunk. `MemoryEventStore` filters under the lock and streams from a snapshot
(`crates/happenstance-core/src/memory.rs:157-181`); the paginating adapter cannot.
Both are conformant today and they have opposite semantics.

---

### E2E-03 — All items of one `Query` are evaluated against one snapshot

- **From:** Wattline (`StartSession`'s four items); Turnstile
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a four-item query whose items select disjoint tag families.
**WHEN** an event matching item 1 is appended between the moment the adapter
evaluates item 1 and the moment it evaluates item 4.
**THEN** either that event is in the result, or the maximum position observed is
below it.

**Falsifies:** that `read_decision_model`'s returned position is a sound append
boundary for a multi-item query (`store.rs:198-208`). If item 1's late arrival is
missed while `last` sits above it, `is_violated_by` returns `false` for
`position <= after` and the boundary is not enforced.

**Rejects:** a one-round-trip adapter that emits one SQL statement per `QueryItem`
and unions the results client-side. This needs no `nextval()` and no visibility
hole — only two statements — and is the natural shape for the Neon peer, which the
scenarios describe as one round trip per operation with no interactive
transaction.

**Note:** this is *not* the same defect as E2E-01. On a store where visibility
order agrees with position order, a lazily-chunked *forward* read cannot miss a
matching event: anything committing after the cursor has passed lands above the
cursor and is still ahead of it. The tear is real only across separate statements
in separate snapshots.

---

### E2E-04 — A decision model can bound one query item without blinding the others

- **From:** Wattline (`StartSession`'s 828,000-event fleet item)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked on a contract change — see [what cannot be written yet](#what-cannot-be-written-yet)

**GIVEN** a four-item query where item 3 has a snapshot event at position *S* and
items 1, 2 and 4 depend on definitional events far below *S*.
**WHEN** the decision model is read.
**THEN** item 3 yields only events above *S*, and items 1, 2 and 4 yield their
definitional events.

**Falsifies:** that `ReadOptions` is sufficient to bound a decision model.
`ReadOptions.from` is one `Option<SequencePosition>` for the entire read
(`crates/happenstance-core/src/query.rs:226-234`) and `read` applies one `ReadOptions`
to the whole `Query` (`store.rs:118-122`).

**Rejects:** every adapter, today, and — more usefully — it rejects the
*workaround*: an application that sets `from = S` for the whole query. That
compiles, produces no error, and silently folds an absent circuit rating, a
vanished connector commissioning and an unknown token validity. The failure mode
is a confident decision over a fold with holes in it.

---

### E2E-05 — Four reads produce four boundaries and one condition cannot carry them

- **From:** Wattline
- **Level:** contract
- **Spans:** `happenstance-core`; blocked on the same change as E2E-04

**GIVEN** four separate reads, one per item, returning last-seen positions
*p₁ > p₂ > p₃ > p₄*.
**WHEN** an append is conditioned on the union query.
**THEN** each item is checked from its own boundary, not from `min(p₁…p₄)`.

**Falsifies:** that splitting a read to bound it is a sound workaround.
`AppendCondition` holds one `after` (`append.rs:52-60`) applied to every item of
`fail_if_events_match` (`:95-107`), so soundness forces the minimum, which
re-admits every event above it for all four items.

**Rejects:** the obvious application-side fix. A quiet token item's stale boundary
becomes the boundary the busy circuit item is checked from, and the resulting
rejection rate is attributed to physics rather than to the collapse.

---

### E2E-06 — A batch's own events are not evaluated against its condition

- **From:** Kestrel Motor (every one of six appends puts the type it is appending into its own condition)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a two-event batch under a condition whose query matches **both** events,
with `after` set past everything already stored.
**WHEN** the batch is appended.
**THEN** it succeeds, and both events land.

**Falsifies:** nothing is claimed today — which is the point. The reference answer
is "no": `MemoryEventStore` checks the condition against `stored` before extending
(`memory.rs:218-230`). That is an accident of implementation order, not a stated
rule, and **no conformance rule covers it.**

**Rejects:** the conditional `INSERT ... SELECT ... WHERE NOT EXISTS` strategy the
decision ledger names as a live candidate (`docs/RUNBOOK.md:64`). Applied per row,
it self-rejects on any single-event batch whose condition names the type it is
appending — which is `RegisterClaim`, `PlaceHold`, `ConvertHoldToTickets` and
`ReserveTourAllocation` across three scenarios — and on any multi-event batch
whose second event matches.

---

### E2E-07 — Dropping an `append` future has a stated outcome

- **From:** Turnstile (the cancelled Worker); Kestrel Motor (`IssuePayment`, the closed browser tab)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** an `append` future built but not driven to completion.
**WHEN** it is polled once with a no-op waker and then dropped.
**THEN** the store is either byte-identical or fully written — never partially —
and whichever of the two the contract states is the one observed.

**Falsifies:** that `store.rs:130-133`'s `# Atomicity` paragraph is a complete
durability statement. It covers partial batches and says nothing about a dropped
future, which at the edge is the *normal* termination path: client disconnect, CPU
limit, Durable Object eviction, pod eviction. `AppendError` has three variants and
none means "the outcome is unknown" (`crates/happenstance-core/src/error.rs:148-166`).

**Rejects:** a pooled rusqlite adapter that does its work in `spawn_blocking`. A
dropped `JoinHandle` does not cancel the closure: the COMMIT executes and the
caller is told nothing, so an operator retries and issues the payment twice.
`MemoryEventStore` passes trivially because `memory.rs:184-224` contains no
`.await` at all — which is exactly why the reference store cannot answer this and a
real adapter must.

---

### E2E-08 — Two handles onto one store see each other's conditions

- **From:** Kestrel Motor (compliance on one-shot HTTP and the purge batch on a pooled connection, same log)
- **Level:** contract
- **Spans:** `happenstance-testkit`; requires a documented obligation that a fixture can open two handles onto one backing store — **discharged** by CF-16 and ES-33, and by `Fixture::connect`

**GIVEN** two store handles *A* and *B* onto the same backing store.
**WHEN** *A* appends an event matching a condition *B* is about to use, and *B*
appends under that condition with `after` set before *A*'s write.
**THEN** *B* is rejected with `ConditionViolated`.

**Falsifies:** the port's central claim — that an `AppendCondition` is sufficient
coordination — for any deployment where one store is reached two ways. Every
conformance rule built one store from `factory()` and drove it through one
handle; `racing_conditional_appends_elect_one_winner` is sequential *and*
single-handle. `two_handles_observe_each_others_appends` (CF-16, CF-19) is what
now covers the read side and the append-condition side of this case; the *two
adapters* half of it is still uncovered.

**Rejects:** an adapter whose condition check is correct only within its own
session — a cached `max(position)` fast path (a strategy the ledger explicitly
defers rather than rules out, `RUNBOOK.md:64`), a per-connection repeatable-read
snapshot, or an advisory lock scoped to a single pool member. All three passed
all twenty-seven rules the suite had before CF-19's
`two_handles_observe_each_others_appends` landed; `CachedHeadFixture` in the
mutant registry is the first of them, compiled.

---

### E2E-09 — Two `append` futures on one `&self` interleave safely

- **From:** Turnstile (twelve concurrent handlers sharing one store through an `Rc` inside a Durable Object)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** one store behind an `Rc`, on a single-threaded executor.
**WHEN** two `append` futures with mutually-violating conditions are polled
alternately to completion.
**THEN** exactly one succeeds, the other returns `ConditionViolated`, and neither
panics.

**Falsifies:** that `append(&self, ..)` (`store.rs:148-152`) is safe to call
re-entrantly. The port says nothing about whether a second `append` may be entered
while a first is in flight, and on the `!Send` flavour both futures interleave at
every `.await`.

**Rejects:** a `RefCell`-backed adapter that holds its borrow across an awaited
storage call — that panics at runtime — and, more subtly, one that drops the borrow
around the await and therefore has an unspecified window between the condition
probe and the write. This is the rule the workspace is missing and the one a
Durable Object adapter would actually fail. `MemoryEventStore` cannot surface it:
`memory.rs:184-224` holds no lock across a suspension point because it has none.

---

### E2E-10 — Reading `from` a position no event occupies

- **From:** Kestrel Motor (resuming across purge holes); Norvant; Turnstile (cold-start resume)
- **Level:** contract
- **Spans:** `happenstance-testkit`

**GIVEN** a store whose assigned positions have a gap — events at 1, 2 and 7.
**WHEN** `read` is issued with `ReadOptions::new().from(5)`.
**THEN** it yields the event at 7, without erroring and without returning empty.

**Falsifies:** that the documented resume recipe is safe on a gapped store.
`ProjectionStore::checkpoint` says to feed its result to `ReadOptions::from`
"after advancing past it" (`crates/happenstance-core/src/projection.rs:87-88`), `from`
is inclusive (`query.rs:228-229`), and the only advance is
`SequencePosition::next()`, whose own documentation says it is "only meaningful
for adapters that allocate positions densely" (`event.rs:138-144`). Every existing
rule feeds `from` a position the store actually assigned
(`suite.rs:802-825`, `:875-909`).

**Rejects:** an adapter implementing `from` as an equality seek or a
`rowid`-offset lookup rather than a range predicate — plausible on a store where
positions came from a counter and the author assumed density. The purge and the
90-day prune both make this load-bearing.

---

### E2E-11 — A forward read can be given an upper bound

- **From:** Norvant (backfill beside a live tail); Kestrel Motor (pinning two phases of an erasure to one snapshot)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked on adding `ReadOptions::to`

**GIVEN** a log at head *H* and a backfill worker.
**WHEN** the worker is given the closed window [1, *H*] while a tail worker owns
(*H*, ∞).
**THEN** the backfill terminates at *H* without streaming past its own horizon.

**Falsifies:** that `ReadOptions` supports the two-worker split every
backfill-beside-tail deployment wants. `ReadOptions` has `from`, `backwards` and
`limit` and no upper bound (`query.rs:224-234`), and `limit` cannot stand in
because `event.rs:94-96` explicitly forbids treating position arithmetic as a
count.

**Rejects:** the workaround — `from(H).backwards()`, buffer, reverse in memory.
That *is* a pinned snapshot and it works at 211 events (Kestrel Motor's erasure);
it is fatal at 53 million (the regulatory backfill). Asserting the capability is
absent, as one scenario did, is wrong; asserting it is only expressible in the
direction that cannot stream is right.

---

### E2E-12 — `read_decision_model`'s boundary is the maximum position observed

- **From:** Norvant (`OpenTemperatureExcursion`'s `backwards().limit(1)` read)
- **Level:** contract
- **Spans:** `happenstance-core`

**GIVEN** a helper that reads a decision model with
`ReadOptions::new().backwards().limit(1)`.
**WHEN** it returns a boundary position.
**THEN** that position is the maximum observed, not the last yielded.

**Falsifies:** that the crate's only read helper generalises. `read_decision_model`
hardcodes `ReadOptions::new()` (`store.rs:212`) and derives its boundary from
`events.last()` (`:213`), which on a backwards read is the **oldest** match.

**Rejects:** any `read_decision_model_with(store, query, options)` that copies the
existing `.last()` idiom. The failure is not a compile error: it is a condition
starting at the first event of the entity's life, rejecting far more often, and
looking like contention rather than a bug.

---

### E2E-13 — The head of the log is reachable through the port

- **From:** Norvant (the 04:00 board that cannot say how stale it is); Turnstile (`block-heatmap`'s cold start)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a store holding *N* events, and generic code bound on `EventStore`.
**WHEN** it asks for the current head.
**THEN** it gets it, at a cost the adapter can bound.

**Falsifies:** that the two-method port is sufficient for an operator to
distinguish a healthy narrow projection from a dead one.
`MemoryEventStore::last_position` exists as an **inherent** method only
(`memory.rs:109-111`), so no generic code can reach it. One method missing, three
symptoms: a narrow projection cannot advance past events it examined and did not
match; an operator cannot tell "six hours behind because nothing matched" from
"dead"; and a dashboard cannot report its own staleness.

**Rejects:** the workaround as a universal answer.
`read(&Query::all(), ReadOptions::new().backwards().limit(1))` compiles and is one
cheap statement on local SQLite — and one full HTTP round trip on the adapter with
the smallest latency budget in the system. An adapter that implements it by
materialising all matches before truncating fails it outright; `MemoryEventStore`
reverses before truncating (`memory.rs:163-179`) and passes.

---

### E2E-14 — An unconditional append is distinguishable from a forgotten condition

- **From:** Kestrel Cold Chain (`ConsumeVanStock`); Kestrel Rotor (`RecordConsumption`)
- **Level:** contract
- **Spans:** `happenstance-core`

**GIVEN** two command handlers in one codebase: one that appends a fact that may
not be refused, and one whose author omitted the condition by mistake.
**WHEN** the source is inspected, linted or grepped.
**THEN** the two are distinguishable.

**Falsifies:** that `Option<AppendCondition>` is the right shape.
`append(&events, None)` (`store.rs:151`) renders "I asserted nothing because there
was nothing to assert" and "I am a decision whose author forgot" as the same bytes
— and a reconciliation projection must distinguish them, because an observation
cannot lose a conflict and a decision can.

**Rejects:** nothing, on its own, which is why this case is weak alone and is
recorded as such. It earns its keep only if the sync port classifies conditions by
replicability, in which case this is where the taxonomy surfaces. A named
constructor (`AppendCondition::unconditional()`) or an `Observation` marker costs
one item and turns a comment into something greppable.

---

## B — Projections and the runner

### E2E-15 — A projection can be returned to "never run"

- **From:** all six; sharpest in Kestrel Cold Chain (`van_stock`, several times a day on 138 devices) and Norvant (`excursion_register`)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`; ⚠ needs the phase-2 projection conformance suite

**GIVEN** a projection whose checkpoint is at position *P*.
**WHEN** it is reset through the port.
**THEN** `checkpoint(id)` subsequently returns `None`.

**Falsifies:** that `ProjectionStore`'s four methods are a complete lifecycle.
`checkpoint` returns `Option<SequencePosition>` (`projection.rs:93`) and `commit`
takes a bare `SequencePosition` (`:109-114`) backed by `NonZeroU64`
(`event.rs:118-119`), so "never run" is a state the port can **report** and no
method can **produce**. `store.commit(batch, id, None)` is
`error[E0308]: expected SequencePosition, found Option<_>`.

**Rejects:** the only implementation available today — every application reaching
around the port into the adapter's own checkpoint table with raw DDL. That defeats
the port, and it is what makes E2E-17 possible.

---

### E2E-16 — The reset that looks right does not skip event 1

- **From:** Norvant (03:18, the operator who "resets properly")
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** an operator with no `reset` who writes
`commit(empty_batch, id, SequencePosition::FIRST)`.
**WHEN** the runner resumes.
**THEN** the event at position 1 is applied.

**Falsifies:** that `SequencePosition::FIRST` is a usable stand-in for "never run".
The runner reads `Some(1)`, computes `1.next()`, and resumes at 2. Event 1 is
skipped **permanently**, silently, and nobody will ever find it.

**Rejects:** the documented-by-omission workaround itself. This case exists to make
the wrong reset fail loudly in a suite rather than quietly in production, and it
should stay in the suite even after `reset` lands, because the idiom will outlive
its necessity.

---

### E2E-17 — Reset clears the read model and the checkpoint in one unit of work

- **From:** Norvant (the truncate committed at 02:46:31; the pod died at 02:46:33)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a projection with rows and a checkpoint.
**WHEN** a reset is interrupted between clearing the rows and clearing the
checkpoint.
**THEN** the store is in one of the two consistent states, never the third.

**Falsifies:** that the port defends the invariant its own module documentation
names — *"the two writes must be one transaction"* (`projection.rs:13-19`). Reset
today is necessarily two transactions, because the port cannot see it, and the
window between them is exactly the failure Norvant's night desk hit.

**Rejects:** the runbook procedure. Two statements on two connections is the only
thing available, and it loses data with no signal: the runner returns, reads the
old checkpoint, resumes past it, applies sixty-one events into an empty table, and
**reports healthy.**

---

### E2E-18 — Reset is scoped per `(store, ProjectionId)`

- **From:** Kestrel Cold Chain (`van_stock` must reset; `fgas_ledger` must never); Norvant (`excursion_register` vs `audit_trail_export`)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** one projection store holding two projections, one of which must be
rebuildable and one of which must not.
**WHEN** the first is reset.
**THEN** the second's checkpoint and rows are untouched.

**Falsifies:** that reset could be a store-wide operation. Two projections on one
store, one requiring reset several times a day and one for which a rebuild would
reissue a hash chain a regulator already holds, is the case that decides it.

**Rejects:** a `SqliteProjectionStore::reset()` that truncates the checkpoint table.
Cheap, obvious, and it destroys the append-only regulatory ledger sharing the file.

---

### E2E-19 — A batch cannot be committed to a different store instance

- **From:** Norvant (41 same-typed depot Durable Objects behind one runner)
- **Level:** contract
- **Spans:** `happenstance-core`

**GIVEN** two instances of the same `ProjectionStore` type, *A* and *B*.
**WHEN** `let batch = A.begin().await?;` is followed by `B.commit(batch, id, p)`.
**THEN** it does not compile.

**Falsifies:** the module doc's claim that the batch "borrows from the store because
a transaction cannot outlive its connection" (`projection.rs:24-26`). That is true of
the lifetime's *duration* and not of its *identity*: `commit(&self, batch:
Self::Batch<'_>, ..)` (`:109-114`) elides a fresh method-level lifetime never tied to
`&self`, and the cross-instance commit is **accepted today**.

**Rejects:** a runner holding a `HashMap<DepotId, SqliteProjectionStore>` that
mis-keys a lookup. Mis-committing one depot's inventory writes under another depot's
checkpoint is, right now, a type-correct program. Tying the lifetime —
`async fn commit<'a>(&'a self, batch: Self::Batch<'a>, ..)` — costs nothing and turns
it into a borrow error. This is the cheapest fix in the entire catalogue.

---

### E2E-20 — Generic code can write a row into a batch

- **From:** all six
- **Level:** contract
- **Spans:** `happenstance-core`; blocked on the apply seam (`RUNBOOK.md:68`, **decided**, ADR pending)

**GIVEN** code generic over `P: ProjectionStore`.
**WHEN** it applies an event.
**THEN** a row lands in the batch.

**Falsifies:** that `ProjectionStore` is a port a runner can be written against.
`type Batch<'a> where Self: 'a` carries **no trait bounds**
(`projection.rs:80-83`), so generic code can `begin`, `commit` and `rollback` a
batch and cannot put anything in it. ADR-0007:34-40 records the same discovery and
defers it.

**Rejects:** the belief that ADR-0007's `pump` sketch closes this. It does compile —
verified — but only because `apply` is a caller-supplied higher-ranked closure
`F: FnMut(&mut P::Batch<'_>, &SequencedEvent)` (ADR-0007:62-67) written against a
**concrete** store's inherent methods. The pump is generic in name only, and a
heterogeneous projection registry is E2E-29.

**Counter-evidence to record:** Turnstile compiled exactly that closure against a
Durable Object batch and it worked. The unbounded GAT does not bite an application
that names its store. It bites a library that cannot.

---

### E2E-21 — Writes made into a batch are visible to reads through it

- **From:** Norvant (`exception_queue` looking up a route it wrote; `vehicle_contact_graph` finding co-loaded consignments)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** an open batch.
**WHEN** a row is written through it and then read back before commit.
**THEN** the row is visible — or the port declares read-your-own-uncommitted-writes
a capability an adapter may decline, and says so.

**Falsifies:** that a `Batch` has a defined read semantics. The port describes it
only as "an in-flight write" (`projection.rs:75-79`); the module's only guidance is
the idempotence escape hatch (`:28-30`), which does not address visibility.

**Rejects:** a Ladybug adapter that buffers Cypher and flushes at commit — which is
the *natural* shape, because `lbug` is synchronous and one round trip is what you
want. A SQLite transaction gives read-your-writes; the buffering adapter does not;
**both satisfy the trait**, and three of Norvant's twenty projections are correct
only under the first.

---

### E2E-22 — A rebuild produces the same read model at any chunk size

- **From:** Norvant (`vehicle_contact_graph`, the determinism casualty)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a projection whose `apply` reads state it wrote for an earlier event.
**WHEN** the same log is replayed at chunk sizes 1, 100 and 5,000.
**THEN** the resulting read models are byte-identical.

**Falsifies:** that rebuild determinism is a property of the log rather than of the
runner's chunking. Under a write-behind batch, pairs falling **inside** one chunk
are silently missed and a rebuild with a different chunk size produces a different
graph.

**Rejects:** the buffering adapter of E2E-21, and any runner whose chunk size is a
tuning parameter. Rebuild determinism is not a conformance property anywhere in the
workspace, and a regulator-facing projection — Norvant's `driver_hours` — is the one
that most needs a *reproducible* rebuild.

---

### E2E-23 — `commit`'s position is the position considered, not the position applied

- **From:** Norvant (`cold_chain_certificate_expiry`, ~40 matching events a day out of 37,000)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a projection whose query matched nothing in the range [*a*, *b*].
**WHEN** it commits an empty batch at position *b*.
**THEN** the commit succeeds and the checkpoint advances to *b*.

**Falsifies:** nothing today, which is the defect. `commit`'s documentation says only
that it "advances `id`'s checkpoint to `position`" (`projection.rs:102-114`) and no
rule pins whether `position` must be one the batch wrote.

**Rejects:** an adapter that validates `position` against what the batch actually
applied. That is a perfectly reasonable reading, it would be equally conformant, and
it makes a narrow projection re-scan the same range forever on every restart. **Two
adapters can disagree and both pass** — which is the sharper finding than the
"impossible" one the scenario reached for, because it is a real interoperability
hazard rather than a capability gap.

---

### E2E-24 — A `Batch` need not be a live transaction

- **From:** Kestrel Cold Chain (`pool_availability` in a Durable Object); Turnstile (`block-availability`, `tour-fan-ledger`)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`; ⚠ needs a Durable Object or Neon projection adapter

**GIVEN** an adapter whose storage API offers `transactionSync(callback)` and no
handle that can be held across an `await`, or no transaction at all.
**WHEN** it implements `ProjectionStore` with `Batch` as a buffer of pending
statements replayed in one call at `commit`.
**THEN** it passes the projection conformance suite.

**Falsifies:** the module documentation's own description — *"an adapter-owned batch
— a SQL transaction, a Ladybug write handle"* (`projection.rs:22-23`). `begin`,
`commit` and `rollback` are all `async` and `Batch<'a>` is passed between them, so a
batch is necessarily **held across await points**, which two of the four named
deployments cannot do with a live transaction.

**Rejects:** the port's own self-description, and with it the
`Batch<'a> where Self: 'a` borrow-from-`Self` clause, which buys nothing on the
majority of targets and buys something only on rusqlite — the one adapter the port
was designed against. This is precisely the "spread of what implements it" test
CLAUDE.md demands, and the projection port has not had it.

**Correction folded in:** the Neon variant of this claim as originally stated is
false. A Neon batch can borrow the client trivially. Neon's real problems are that
`begin` is `async` and fallible and therefore implies a round trip it does not need,
and that the whole write set must be buffered and emitted as one statement at
commit — the same conclusion the Durable Object reaches.

---

### E2E-25 — A rebuild chunked across many invocations does not lie about its own completeness

- **From:** Kestrel Cold Chain (`pool_availability`, 4.1M events, unsurvivable in one DO request); Norvant; Wattline (`fleet-usage`'s 210M-event backfill)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked on a projection lifecycle state or a documented swap protocol

**GIVEN** a projection rebuilding from position 1 across many separate invocations.
**WHEN** a reader asks for its checkpoint mid-rebuild.
**THEN** it can distinguish "caught up to *N*" from "rebuilding, currently at *N*, do
not treat these rows as authoritative".

**Falsifies:** that one `Option<SequencePosition>` is sufficient projection state
(`projection.rs:85-93`). There is no second field, no status, no shadow checkpoint.

**Rejects:** a rebuild in place, which is the obvious reading of the port. On the
Kestrel Cold Chain hub the mid-rebuild read model is precisely what the next
device's slice is cut from, so a checkpoint that lies **manufactures the conflict
the projection exists to prevent.** The honest cheaper alternative — rebuild into a
second `ProjectionId` and swap — is already permitted by the port and documented
nowhere.

---

### E2E-26 — A failing `apply` names the position it failed at

- **From:** Norvant (the panic at 4,000,001); Kestrel Motor (the deliberate crypto-shred); Wattline (the trailing-space currency code); Turnstile
- **Level:** contract
- **Spans:** ⚠ `happenstance` (a doc comment and one `pub use`, `crates/happenstance/src/lib.rs:76`)

**GIVEN** a pump and an `apply` that returns an application error at position *P*.
**WHEN** the pump returns.
**THEN** it reports "stopped at *P* because of the event at *P*", and the checkpoint
sits at the last good position.

**Falsifies:** that ADR-0007's `pump` signature is complete. It types the callback's
error as `P::Error` (ADR-0007:62-67) — a **projection store** error — so a *decode*
failure has no representable home: the application must forge one into the adapter's
`#[non_exhaustive]` error enum, which belongs to the adapter, or panic.

**Rejects:** the panic, which is what the scenario describes and what an
implementation will do. `PumpError<E::Error, P::Error>` cannot carry it.

---

### E2E-27 — The failure policy is per projection, not per runner

- **From:** Wattline (`revenue-recognition` must halt; `site-board` must not); Kestrel Motor (a shred needs "skip and record")
- **Level:** contract
- **Spans:** ⚠ `happenstance`; owns `RUNBOOK.md:73`

**GIVEN** two projections in one runner, one declaring halt-on-failure and one
declaring skip-and-record.
**WHEN** both hit an undecodable event.
**THEN** the first stops at that position and the second advances past it **and
records that it did**.

**Falsifies:** that RUNBOOK:73's question has one answer. Halting is correct for a
revenue read model — one that skips an event is worse than one that stops — and wrong
for an availability board, where a stale board is worse than one missing a connector.
So the policy belongs on the `Projection` trait rather than on the runner.

**Rejects:** a runner-level `on_error: SkipPolicy` configuration. It is the obvious
design and it forces one wrong answer onto one of the two projections.

**Note:** the skip *primitive* already exists and nobody has noticed. `begin()`
followed immediately by `commit(batch, id, poison_position)` applies nothing and
advances the checkpoint atomically, with the port exactly as written. What does not
exist is any way to record that it happened.

---

### E2E-28 — One poisoned projection does not stall the others

- **From:** Norvant (nineteen kept running); Turnstile (the halted graph projection)
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ `happenstance`, ⚠ e2e crate

**GIVEN** twenty projections over one log, one of which panics in `apply`.
**WHEN** the runner continues.
**THEN** the other nineteen advance, and the failure is observable.

**Falsifies:** nothing about the port — this is the one thing that works, and it works
because `Projection::Store` is an associated type and checkpoints are per
`(store, ProjectionId)` (ADR-0007:92-104). It is in the list because **the second half
fails**: in Norvant nothing alerted, because the `JoinHandle` went into a set nobody
drained.

**Rejects:** the fan-out runner. One read of the union query, one decode, twenty
applies is the cheap answer to the poll-cost problem — and it does **not** isolate a
panic, whereas per-projection `tokio::spawn` does. **The cheap runner and the safe
runner are opposites and the port adjudicates neither.** If a fan-out runner catches
panics it must use `AssertUnwindSafe` around `&mut P::Batch<'_>`, which is defensible
only because `ProjectionStore` supplies `rollback` — the transactional invariant is
what makes the unwind-safety assertion honest rather than a lie, and nobody has
written that down. (The workspace's release profile does not set `panic = "abort"`, so
unwinding is available; that too is undocumented and depended on.)

---

### E2E-29 — A heterogeneous projection registry

- **From:** Norvant (fourteen Postgres views and six Ladybug views under one control tower)
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ `happenstance-ladybug`, ⚠ e2e crate

**GIVEN** projections targeting two different `ProjectionStore` types.
**WHEN** they are held in one collection and driven by one supervisor.
**THEN** it compiles.

**Falsifies:** the premise "one runner, twenty views, two stores".
`Projection::Store` is an associated type, so
`error[E0271]: type mismatch resolving <NetworkTopology as Projection>::Store == Pg`.

**Rejects:** nothing wrong — the constraint is *correct*, because there is no
cross-store transaction and one that appeared to work would lie about the invariant
the port exists to defend (ADR-0007:92-98). What this case exists to force is that the
port's documentation says so, so nobody designs a heterogeneous supervisor first and
discovers E0271 second. The honest answer is that runners are per store.

---

### E2E-30 — A `!Send` projection store cannot be spawned, and the port says so

- **From:** Norvant (rusqlite and Ladybug batches); Kestrel Cold Chain (the DO hub)
- **Level:** contract
- **Spans:** `happenstance-core`

**GIVEN** a `ProjectionStore` whose `Batch` is `!Send` — `rusqlite::Transaction`, an
`lbug` handle.
**WHEN** the `Send` flavour is required.
**THEN** the failure is documented rather than discovered:
`#[trait_variant::make(SendProjectionStore: Send)]` (`projection.rs:70`) puts the
batch in `commit`'s and `rollback`'s parameter lists, so the `Send` flavour
transitively requires `Batch<'a>: Send` — *"future returned by `commit` is not `Send`
... captured value is not `Send`: `_batch`"*.

**Falsifies:** that runner topology is a free choice. It is not: the graph projections
and the on-device SQLite store are forced onto `LocalSet` / `spawn_blocking` while the
Postgres ones spawn normally, and Norvant's most expensive `apply` (~90 ms) is a graph
projection.

**Rejects:** nothing in the port — this is `trait_variant` behaving correctly. The case
exists because the consequence decides the deployment's threading model and is
currently discoverable only by writing the adapter and being told.

---

### E2E-31 — A projection that emits events back into the log

- **From:** Norvant (`scan_gap_monitor`); Kestrel Cold Chain (`StockReconciliationRequired`)
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ `happenstance`, ⚠ e2e crate

**GIVEN** a projection whose read model implies a domain event.
**WHEN** the projection is rebuilt.
**THEN** it does not re-emit events it previously caused.

**Falsifies:** the port's own escape hatch. `projection.rs:28-30` says projections
that cannot be transactional must instead be idempotent — and **an outward-writing
projection cannot be idempotent without event identity.** `ProjectionStore::commit`
is atomic with its checkpoint and nothing else; `EventStore::append` is a different
port; ADR-0007 makes a cross-store transaction deliberately unrepresentable.

**Rejects:** the obvious implementation — emit through a command handler after
`commit` returns. On the Kestrel Cold Chain hub both stores are the same SQLite file,
so the transaction is *physically available* and the ports forbid expressing it. The
honest answers are two, and one must be chosen: the emitted event is written
idempotently and duplicates are tolerated, or process-manager-style emission is
declared out of scope. Today it is neither.

---

### E2E-32 — The query union a fan-out runner depends on

- **From:** Norvant (one read, one decode, twenty applies)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** projections with queries *a* and *b*.
**WHEN** a runner reads `Query::from_items(items(a) ++ items(b))` and re-filters each
projection's stream locally with `Query::matches`.
**THEN** each projection sees exactly what its own query would have selected.

**Falsifies:** that the fan-out runner is safe to write. It rests entirely on
`Items(a) ∪ Items(b) == Items(a ++ b)`, and on anything unioned with `All` being
`All`. `Query::matches` is `pub` (`query.rs:194-204`) so the local re-filter is
available — but **nothing in the crate states the algebra and no conformance rule
pins it.**

**Rejects:** an adapter that deduplicates or reorders a query's items as an
optimisation. `QueryItem::new` already sorts and dedups *types* (`query.rs:62-67`),
so an author extending that to items would find it natural. Such an adapter still
passes every rule in the suite and breaks the runner: SPECIFICATION.md's ES-15
says in terms that no order-invariance rule can catch it, because sorting the
items is precisely what makes their order stop mattering. VT-31's
`query_union_is_item_concatenation` is the rule that would, and it is still owed.

---

## C — Sync and convergence

Every case in this group needs `happenstance-sync`, which today is a module doc
comment and a one-variant error enum (`crates/happenstance-sync/src/lib.rs:103-112`),
and most need `happenstance-sync-testkit`, which is named at `lib.rs:9-11` and does
not exist. They are specified anyway, because the port's shape is what they constrain.

### E2E-33 — Re-delivering an accepted group is a no-op

- **From:** Kestrel Cold Chain (`IngestPeerBatch` after a dropped socket) — **the sharpest single finding in the catalogue**
- **Level:** contract (sync)
- **Spans:** ⚠ `happenstance-sync`, ⚠ `happenstance-sync-testkit`; blocked on `EventId` (`RUNBOOK.md:70`)

**GIVEN** a hub that has accepted a peer's condition group and written its events.
**WHEN** the identical group is delivered again.
**THEN** nothing changes, and in particular the hub does **not** author a
compensation.

**Falsifies:** that "re-evaluate the origin condition verbatim" can be idempotent
against today's types. On first delivery the hub accepts and writes; on re-delivery
the same condition now matches **the hub's own copy**, returns `ConditionViolated`,
and the hub routes into the violated branch — **superseding the event it just
accepted.** `Event` is `(event_type, data, tags, metadata)` and `SequencedEvent` is
`(position, event)` (`event.rs:183-188`, `:277-282`): nothing globally unique.

**Rejects:** the design as the scenario wrote it. Its own idempotence guard — a
condition keyed on the `supersedes:` tag — protects the **compensation**, not the
**accept** path. The invariant "ingest is idempotent" is unimplementable, and the
failure is not a duplicate but an inversion.

---

### E2E-34 — A peer can dedupe without parsing metadata

- **From:** Kestrel Cold Chain; Turnstile; Kestrel Rotor
- **Level:** contract (sync)
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`; blocked on `RUNBOOK.md:70`'s shape

**GIVEN** an ingested event carrying a store-assigned identity.
**WHEN** a peer asks "have I already ingested this?".
**THEN** it can ask through the port, without deserialising anything.

**Falsifies:** the sync module's own proposal. `sync/lib.rs:73-75` suggests "a UUIDv7
or a content hash in the event's **metadata**". Metadata is opaque `Bytes`
(`event.rs:187`, `:239`) and `QueryItem::matches` filters on type and tags only
(`query.rs:113-116`), so **the identity a peer is told to carry is structurally
unqueryable.** A peer that must parse metadata to dedupe has broken ADR-0003 at the
exact point ADR-0003 claims to win.

**Rejects:** the metadata-key implementation, and also the naïve fix. Promoting
identity to a *tag* makes it queryable and costs more than it looks: a maximally
high-cardinality entry in the column adapters are told to index, an entry in every
`contains_all` merge-scan (`tag.rs:231-245`), a writer-forgeable identity, and an
identity dimension visible to every tag-only query. It also has a hole — a peer's own
locally-originated writes would carry no such tag, so it could not order its own
events against ingested ones. **The mechanism only works if the store stamps its own
writes at append time**, which is the Lamport pair the ledger already decided.

---

### E2E-35 — A peer's unit of work decomposes into N conditional appends

- **From:** Kestrel Cold Chain (39 events, 7 groups); Kestrel Rotor
- **Level:** contract (sync)
- **Spans:** ⚠ `happenstance-sync`

**GIVEN** a push carrying seven independently-conditioned groups.
**WHEN** the hub crashes after ingesting four.
**THEN** the outcome is the documented one, and the wire format made the
decomposition explicit rather than emergent.

**Falsifies:** that a peer's push is one unit of work. `EventStore::append` takes
exactly one `Option<&AppendCondition>` for the whole slice (`store.rs:148-152`), so
seven groups is seven appends with nothing spanning them. That is **correct DCB** —
the boundary is the query, not the batch — and it is stated nowhere in the contract,
the testkit or the sync prose.

**Rejects:** a flat-event-list wire format. It makes the decomposition invisible,
makes ingest cost look like O(events) when it is O(distinct conditions), and turns a
partial ingest into something discovered in production. The fix is documentation plus
a `[(AppendCondition, Vec<Event>)]` envelope; no contract change.

---

### E2E-36 — Idempotent bulk ingest inside one round trip

- **From:** Kestrel Rotor (1,840 events in a 34-minute satellite window) — the finding nothing else in the workspace would have produced
- **Level:** contract (sync)
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`

**GIVEN** a peer batch of 1,840 events, some already ingested.
**WHEN** the receiving store ingests over a transport that costs one round trip per
operation.
**THEN** the already-seen events are skipped, the new ones land, and it takes a
bounded number of round trips.

**Falsifies:** that one `AppendCondition` per batch is sufficient for a peer port.
Idempotent bulk ingest wants **one boundary per event**, and all three available
shapes fail: 1,840 appends is 1,840 round trips; one condition of 1,840
single-identity items rejects the whole batch on one already-seen event and
`ConditionViolated.conflicting_position` names at most one culprit
(`error.rs:96-104`), so recovery is a serial peel; read-then-filter-then-unconditional
is two round trips with an unclosed race when two peers ingest concurrently.

**Rejects:** all three, which is the point. The port needs either a per-event
conditional append (`append(&[(Event, Option<AppendCondition>)])`, or a separate
`append_idempotent` keyed on identity) or a **store-level uniqueness guarantee on
`EventId`** that the port states and the testkit checks. The second is cheaper and
lands with the already-decided identity row.

---

### E2E-37 — A wire condition carrying `after: Some(_)` is refused

- **From:** Kestrel Cold Chain (the cut `ChargeRefrigerant` decision); Kestrel Motor; Wattline (`after: Some(7730)` from a site controller)
- **Level:** contract (sync)
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`

**GIVEN** a peer-supplied `AppendCondition` deserialised from the wire.
**WHEN** its `after` field is `Some(p)`.
**THEN** ingest refuses it rather than evaluating it.

**Falsifies:** that `AppendCondition`'s serde impl is safe for the purpose it exists
for. `after` is store-local by definition (`append.rs:55-60`), `is_violated_by`
compares raw positions (`:95-107`), and the wire form serialises it as a naked
integer (`:117-123`). At the receiver, `after: 288455` names an unrelated recent
event, so the check runs over an arbitrary tail and **passes vacuously** — which is
worse than being rejected, because it looks like enforcement.

**Rejects:** the "sound because single-writer" argument in general. An exclusive-writer
grant makes a position-relative condition sound *within the granting store*; it says
nothing about the second store that re-evaluates it. `AppendCondition` is
`#[non_exhaustive]` with **public** fields (`append.rs:51-61`), so this refusal is
checkable today — readable but not literal-constructible is precisely what enables it.
The rule is blunt, and E2E-38 is why.

---

### E2E-38 — A condition can carry what its author actually knew

- **From:** Turnstile (`SellAtDoor`); Kestrel Motor; Norvant (`RecordProofOfDelivery`)
- **Level:** contract (sync)
- **Spans:** ⚠ `happenstance-sync`; blocked

**GIVEN** an offline terminal that decided at its own local position *L*, having
replicated the hub up to hub position *H*.
**WHEN** its events reach the hub.
**THEN** ingest can evaluate "reject if a matching event landed after *H*", and does
not reject on the terminal's own already-replicated events.

**Falsifies:** that `after: Option<SequencePosition>` is sufficient to cross a store
boundary. Re-checking with `after: None` rejects the terminal's own predecessors and
therefore rejects unconditionally and forever; re-checking with a translated position
is impossible because there is no translation and `SequencePosition` carries no
origin. Adding a field is `error[E0599]: no method named after_remote`.

**Rejects:** both binary answers in `sync/lib.rs:78-83` ("re-checks conditions, or
unconditional append-of-facts-already-decided"). Neither contains this. The honest
fix is an ingest-side condition type in `happenstance-sync` carrying
`(origin_peer, origin_position, local_after)` — the hub's re-check is a **different
operation** from a local append, and putting it on `AppendCondition` would put
replication semantics in the contract crate.

---

### E2E-39 — Ingest never rejects: the losing event and its compensation land atomically

- **From:** Kestrel Cold Chain (the whole scenario)
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`, ⚠ e2e crate

**GIVEN** two peers that each accepted a conflicting claim under byte-identical
position-free conditions.
**WHEN** the second reaches the hub and its condition fails.
**THEN** the hub appends `[losing_event, compensation]` as **one** batch, and no
reader ever observes the losing event unresolved.

**Falsifies:** that a peer must choose between accepting and rejecting.

**Rejects:** the two alternatives, and the cost of each is specific. **Reject**: the
device deletes a fact its user acted on, which is a product failure and a lie, and
the field force stops trusting the screen inside a week. **Accept unconditionally**:
the hub log holds two live claims and every downstream projection independently
reinvents a tie-break, and they will not agree. **Compensate non-atomically**: there
is a window in which the hub log holds the losing claim with nothing resolving it,
and a device syncing in that window cuts its slice from a log that says the unit is
doubly held.

**What survives today:** the compensating write is exactly one call —
`append(&[losing, superseded], Some(&guard))` — and `store.rs:130-133` guarantees
all-or-nothing, conformance-tested by `append_is_atomic` (`suite.rs:1200-1221`). This
half needs no new seam.

---

### E2E-40 — Ingest validates a peer-supplied condition before executing it

- **From:** Kestrel Cold Chain (a buggy spoke, not a malicious one)
- **Level:** contract (sync)
- **Spans:** ⚠ `happenstance-sync`

**GIVEN** a peer push whose condition is `Query::All`.
**WHEN** the hub ingests.
**THEN** it refuses the group rather than executing an unbounded scan.

**Falsifies:** that a transport-only sync port is sufficient. `AppendCondition` bounds
nothing about its own cost, `Query` deserialises `none` as `Query::All`
(`query.rs:314-321`), and `ReadOptions` is not part of a condition — so a peer can push
"fail if the log is non-empty" and the hub executes an arbitrary peer-authored query
against millions of rows inside a single-threaded actor with a fixed CPU ceiling.

**Rejects:** any ingest implementation that treats a wire condition as data to evaluate
rather than input to validate. This is the first concrete argument that the sync port
needs a **condition policy seam** and not just a transport, and it is sharper than
anything currently in that crate's prose.

**What already defends itself:** `Query::All` cannot be smuggled as an *empty items
list* — `Query::from_items` rejects it (`error.rs:62-67`) — and `QueryItem` and `Tags`
re-validate on deserialisation (`query.rs:297-302`), so non-canonical tags and fully
unconstrained items cannot reach hub memory. The envelope defends its own invariants;
only cost is unguarded.

---

### E2E-41 — A convergent projection produces the same read model under two interleavings

- **From:** Kestrel Rotor (`stock-on-hand` converges; `unit-ledger` does not) — the case that turns a hope into a check
- **Level:** contract (sync)
- **Spans:** ⚠ `happenstance-sync-testkit`

**GIVEN** a projection and a set of events from two origins.
**WHEN** they are applied under two interleavings that agree on per-origin order and
disagree everywhere else.
**THEN** the resulting read models are byte-identical — or the projection has declared
itself non-convergent and is excluded from the check.

**Falsifies:** that ingest-appends-at-the-tail is a complete answer. Local position is
arrival order, and two peers have different total orders over the same events. Folds
that must converge must be **commutative**, and today that is hoped for rather than
checked.

**Rejects:** `unit-ledger` — a per-serial state machine folded over
`SequencedEvent::position` — and more generally **any projection that reads the
position at all.** That points at the library change: a convergent projection should
be handed an `EventId`, not a position, and ADR-0007's `apply` hands it
`Sequenced<Self::Event>` carrying exactly the value it must not touch
(ADR-0007:76-81). `cost-layers` is the honest case that must be *excluded*: FIFO
consumption order **is** the fold, so it is permanently non-convergent and pinned to
one peer by fiat — and nothing lets a projection declare that.

---

### E2E-42 — Transitive convergence: a peer forwards what it holds, not what it originated

- **From:** Kestrel Rotor (vessel ↔ depot A ↔ depot B, with the vessel never reaching depot B)
- **Level:** integration
- **Spans:** ⚠ `happenstance-sync`, ⚠ e2e crate

**GIVEN** three peers *A*—*B*—*C* where *A* and *C* never communicate.
**WHEN** each peer syncs only with its neighbour.
**THEN** all three converge on the union of facts.

**Falsifies:** a forwarding rule based on origin. The proof is short and depends on two
things being true together: ingest is total and unconditional, and it appends **at the
tail** — so an event arriving late from *A* gets a *high* position at *B* and is
therefore picked up by *C*'s scalar cursor into *B*'s local order even though it is
old. Arrival order and forwarding order must be the same order.

**Rejects:** two implementations, and both are natural. A peer that **quarantines**
rather than appends: the quarantined event has no local position, is never forwarded,
and disappears permanently from *C*'s view — which independently kills quarantine as a
policy. And a peer that forwards only events it originated: *C* never sees *A* at all.

---

### E2E-43 — A store-assigned time on a `SequencedEvent`

- **From:** Kestrel Rotor (which side of midnight did the issue fall on); Kestrel Cold Chain (two tablets 90 seconds apart, one six minutes fast)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked, and should be decided in the same pass as `EventId`

**GIVEN** an event appended at a known instant.
**WHEN** an auditor asks when the store recorded it.
**THEN** the log answers.

**Falsifies:** that `SequencedEvent` is complete. It is `{ position, event }`
(`event.rs:277-282`) and nothing else, so every timestamp in every one of these six
scenarios is a *writer's* clock embedded in an opaque payload — including one drifted by
2.4 seconds and one six minutes fast after a factory reset.

**Rejects:** the belief that the Lamport pair covers this. It gives dedup and a
deterministic tiebreak; **a tiebreak is not a fact about the world.** `SequencedEvent`
is `#[non_exhaustive]` (`event.rs:276`) so the field is additive, but
`SequencedEvent::new` is a two-argument `const fn` every adapter and the testkit call
(`:286`), so this is a signature change across the workspace and belongs in one pass
with identity.

**Correction folded in:** the Kestrel Rotor decision that motivated this (a lifting
accessory issued past its certification expiry) is **cut**. Its premise — "there is no
event to condition on" — is false in the scenario's own vocabulary, which contains
`CertificationExpiryRecorded`. Materialise the clock as an event and the DCB condition
works. What survives is only the audit question above.

---

### E2E-44 — Retention is coordinated across the peer set

- **From:** Kestrel Rotor (a vessel on a 120-day charter against a 90-day compaction window); Kestrel Cold Chain (the 90-day device prune)
- **Level:** integration
- **Spans:** ⚠ `happenstance-sync`, ⚠ e2e crate

**GIVEN** a peer offline longer than another peer's compaction window.
**WHEN** it reconnects.
**THEN** either it converges, or it is told it cannot.

**Falsifies:** that convergence is a property of the protocol alone. It is also a
property of retention, and **no port in the workspace has a place to state or check the
constraint.**

**Rejects:** every retention implementation in these scenarios, all of which happen
entirely outside the port. Silent non-convergence with nothing anywhere reporting it is
the default behaviour.

---

### E2E-45 — A device whose ingest the hub refuses

- **From:** Kestrel Motor — **the direction no scenario in this catalogue walks**
- **Level:** integration
- **Spans:** ⚠ `happenstance-sync`, ⚠ e2e crate

**GIVEN** a device holding events it believes are committed.
**WHEN** the hub declines them.
**THEN** the device's own log, its own projections and its own user are left in a
defined state.

**Falsifies:** nothing yet, which is why it is here. Every scenario tests the hub's
behaviour on ingest and none tests the spoke's behaviour on refusal — the case where a
device holds committed facts and the authority disagrees. It is where the sync port's
shape is genuinely undetermined, and Kestrel Motor's own best original element
(hundreds of thousands of independent position spaces merging into one) is exercised in
exactly one direction.

**Rejects:** the assumption that a spoke's merge rule is the hub's merge rule read
backwards. `sync/lib.rs:45-51` already says it is not.

---

## D — Lifecycle and evolution

### E2E-46 — A store can say what it does not hold

- **From:** Kestrel Cold Chain (a 90-day slice); Kestrel Motor (a scattered purge); Kestrel Rotor (a compacted peer)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`; blocked — **not a ledger row anywhere**

**GIVEN** a store that has been pruned, purged or compacted.
**WHEN** a runner, an ingest path or a decision model reads it.
**THEN** it can distinguish that store from one that is simply young.

**Falsifies:** that `EventStore`'s two methods are sufficient for a store with a
history. `read` yields events and `checkpoint` yields a position; **neither has anywhere
to put "history below here is not retained"**, and a holed log and a young log are the
same value.

**Rejects:** the cheap primitive, and this is the finding. `earliest_position()` — the
only retention primitive anyone has proposed — is exactly correct for a *device pruning
old history* and useless for a *regulated purge*, because the purge is **scattered, not
a prefix**: a 2019 catastrophic-injury file with a periodical payment order sits at a
low position and must survive while its neighbours are destroyed. A floor is the shape a
prefix truncation has and precisely the shape this purge does not. That it ships looking
correct until a claim runs long is why it is worth naming.

**Three doors, one primitive.** A pruned spoke slice, a purged cohort and a compacted
peer arrive at the same missing thing from three unrelated directions, which is the
strongest evidence available that it belongs in the port and not in an adapter.

---

### E2E-47 — An append condition over destroyed history refuses rather than passes

- **From:** Kestrel Motor (Tuesday 18:02) — the load-bearing failure of that scenario
- **Level:** contract
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`; depends on E2E-46

**GIVEN** a condition whose query ranges over events that have been destroyed.
**WHEN** it is evaluated.
**THEN** the outcome is a third thing — "this log cannot evaluate this condition" —
rather than "no match".

**Falsifies:** the implicit claim that a conditional append is sound over any conformant
store. It is sound only over a **complete** one, and the contract has no notion of
completeness. `is_violated_by` is a pure predicate over events that still exist
(`append.rs:95-107`) and has no third outcome; the information is gone from the store,
not merely absent from the API.

**Rejects:** both options in `sync/lib.rs:78-83`. Re-checking passes vacuously —
which is what actually happened: three engineer notes landed on a claim with no
registration, no policy, no reserve and no closure, and the desktop projection
materialised a row for it in four seconds. Unconditional append is what the failure
already was.

---

### E2E-48 — Deletion and its marker are one unit of work

- **From:** Kestrel Motor (`ExecuteRetentionPurge`, `ExecuteErasure`)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked — no port surface exists

**GIVEN** a set of positions to destroy and a conditional marker append recording that
they were.
**WHEN** the operation is interrupted.
**THEN** the store holds both or neither.

**Falsifies:** that the port's atomicity guarantee covers the operations a regulated log
actually performs. `append` is closed over insertion (`store.rs:148-152`); there is no
delete, no redact, no truncate, no tombstone. A conformant adapter must open its own
transaction outside the port, at which point **the atomicity the port spent its whole
design defending is being provided by the adapter's private code, unobserved by any
rule.**

**Rejects:** the two states an interrupted purge can leave: a deletion with no marker is
undocumented destruction; a marker with no deletion is a false compliance record. The
regulator will find whichever one you produce.

**Note on shape:** this is the same problem `ProjectionStore` already solved with an
adapter-owned `Batch` GAT (`projection.rs:80-83`). Whatever the answer is, it has to be
the *same* answer, or the workspace has two incompatible theories of transactional
scope. An explicit written refusal — deletion is out of scope, and here is what a store
that has been deleted from is permitted to look like — is a legitimate answer. Silence
is not, because silence is what Tuesday's ingest read as "nothing ever happened here".

---

### E2E-49 — A tag can be redacted

- **From:** Kestrel Motor (`subject:S-88213` surviving a crypto-shred)
- **Level:** contract
- **Spans:** `happenstance-core`; blocked — not a ledger row anywhere

**GIVEN** an erased data subject whose payloads are undecryptable.
**WHEN** the fraud graph is rebuilt.
**THEN** the subject's node is not reconstructed.

**Falsifies:** that crypto-shredding is a complete erasure story for this contract.
Shredding covers `data`. It cannot cover `tags` — and `tags` is the half the port has
made **indexable and queryable**. `Event`'s fields are private and `with_tags`
constructs a new value (`event.rs:209-221`); there is no store-side update path.

**Rejects:** the entire crypto-shred design as sufficient. The singling-out identifier is
structurally immortal, the graph rebuild reconstructs the person node from surviving
tags, and — the case that lands hardest — a `subject-index` read model deleted on
erasure comes **back** six weeks later when a corrupted SQLite file is rebuilt from a
log whose tag indexes still carry the subject. The erasure was executed, recorded, and
undone by a routine recovery, and nothing anywhere noticed.

This is the strongest argument in the catalogue for a **redaction** seam rather than a
**delete** seam.

---

### E2E-50 — A projection's `Query` changed under its checkpoint

- **From:** Kestrel Motor (`retention-ledger`, silently under-reporting held claims for two years); Norvant; Wattline (`roaming-graph`)
- **Level:** contract
- **Spans:** `happenstance-core`, ⚠ `happenstance`

**GIVEN** a projection at checkpoint *P* whose `Query` is widened to include a type
that has existed since long before *P*.
**WHEN** the runner resumes.
**THEN** the change is detected, and the projection is not silently wrong.

**Falsifies:** the unstated half of ADR-0007's decision. Pinning a projection's
subscription to `Query` is right and free — one vocabulary, already conformance-tested,
no parallel filter language (ADR-0007:85-90). **The cost is that a query change is
indistinguishable from a rebuild, and nothing in the port ties a checkpoint to the query
that produced it.**

**Rejects:** every runner that stores a checkpoint keyed on `ProjectionId` alone —
which is every runner the port permits. In Kestrel Motor the projection that
under-reported for two years is the one the retention job depends on.

---

### E2E-51 — A command handler compiles with one error type

- **From:** Kestrel Cold Chain; Kestrel Rotor (six decisions, six times over)
- **Level:** contract
- **Spans:** `happenstance-core`

**GIVEN** a command handler in a library crate — so no `anyhow`, per house style.
**WHEN** it calls `Tags::from_pairs`, `QueryItem::new` and `Query::from_items` and
propagates with `?`.
**THEN** it compiles without a bespoke error enum.

**Falsifies:** that the validation error types compose. `InvalidTag`,
`InvalidEventType` and `InvalidQuery` are three unrelated enums
(`error.rs:19`, `:41`, `:62`) and the only `From` impl on `InvalidQuery` besides
`InvalidEventType` is from `Infallible` (`:77-84`). So a decision touching both tags
and queries needs a four-way union — the two validation errors, `S::Error` from the
read, and `AppendError<S::Error>` from the append — before it can write one line of
domain logic.

**Rejects:** every snippet in every one of these six scenarios, all of which used `?` on
both and compile only inside a `Box<dyn core::error::Error>` doctest — which is exactly
how the crate's own doctests escape it (`query.rs:141`, `append.rs:31`). One
`impl From<InvalidTag> for InvalidQuery`, or a single `InvalidInput` the three collapse
into, is two lines and lands on all thirty-odd queries in this catalogue.

---

## E — Edge and `!Send`

### E2E-52 — A whole command path composes with no `Send` bound

- **From:** Turnstile (`PlaceHold`, compiled); Kestrel Cold Chain (`IngestPeerBatch` in a Durable Object)
- **Level:** contract
- **Spans:** `happenstance-core`, `happenstance-testkit`

**GIVEN** a `!Send` `EventStore` behind an `Rc`, on a single-threaded executor.
**WHEN** a generic handler bound on `EventStore` reads with a five-item query, drains it
with `collect`, builds `AppendCondition::new(query).after_opt(last_seen)` and appends.
**THEN** the whole future is usable in a `!Send` context.

**Falsifies:** nothing — it **confirms** ADR-0001, which is still marked provisional
pending exactly this. It is in the list because no `!Send` implementation exists and
every previous justification was a `cargo check` for `wasm32`.

**Rejects:** any refactor that makes `read` `async`. Putting the stream inside a future
silently drops `+ Send` from the *stream* on the `Send` flavour and defeats the whole
two-trait design (`store.rs:104-108`, CLAUDE.md constraint 3). A unit test already
asserts this; this case extends it from a type-level assertion to a compiled command
path, and adds the free function, `collect`, and `Rc`-sharing to what is covered.

**Second thing it confirms:** `type Error: core::error::Error + 'static`
(`store.rs:99`) carries **no** `Send` or `Sync` bound, so an adapter error holding a
`JsValue` or an `Rc<str>` satisfies it today. The claim that the port forces
stringification at the wasm boundary is false against the current code; it is the cost
of a *proposed* change (E2E-53), and attributing a proposal's cost to the current code
gets the current code changed for the wrong reason.

---

### E2E-53 — `SendEventStore` is usable from several tasks

- **From:** Wattline (1,900 handlers a minute against one store)
- **Level:** contract
- **Spans:** `happenstance-core`; **semver-visible, must be decided before publish**

**GIVEN** one store shared as `Arc<S>` across tokio tasks.
**WHEN** generic code bound on `EventStore` — the bound CLAUDE.md rule 4 mandates —
spawns a handler.
**THEN** it compiles, and the handler's error can cross a `JoinHandle`.

**Falsifies:** that the two-flavour design is sufficient for a concurrent deployment.
`#[trait_variant::make(SendEventStore: Send)]` (`store.rs:92`) yields `Send` and `Send`
futures, never `Sync`; calling `&self` methods from several tasks requires `S: Sync`. And
`type Error: core::error::Error + 'static` (`:99`) is not `Send + Sync`, so a spawned
handler's error cannot be returned.

**Rejects:** deferring the decision. `trait_variant` copies associated-type bounds
verbatim, so `Error: Send + Sync` cannot be added to only one flavour — it has to go on
the **base** trait, which costs wasm nothing and is a breaking change once published.
The scenario that needs it is entirely a concurrency scenario, and the port cannot be
driven concurrently from generic code.

---

### E2E-54 — A store can be chosen at runtime

- **From:** Turnstile (local SQLite when offline, the Durable Object when online)
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ e2e crate

**GIVEN** an application that must pick its store at runtime.
**WHEN** it holds `Box<dyn EventStore>`.
**THEN** `error[E0038]: the trait EventStore is not dyn compatible`.

**Falsifies:** nothing — ADR-0001:88-90 accepts this knowingly, and `dynosaur` generates
the wrapper if something needs it. The case exists because the cost is misstated
wherever it is raised.

**Rejects:** the code-size framing. On a 10 MB compressed Worker bundle, two copies of a
read-fold-append loop is not a budget concern (a test-only memory store does not ship).
The genuine consequence is **runtime store selection**, which is the first thing a
local-first application asks for and which no scenario raised. Worth a line in the store
module docs pointing at `dynosaur`.

---

### E2E-55 — A type-only append condition has no lock key

- **From:** Turnstile (`ReserveTourAllocation` on Neon)
- **Level:** contract
- **Spans:** `happenstance-core`, ⚠ `happenstance-neon`

**GIVEN** a store that enforces conditions by taking an advisory lock per condition tag,
in sorted order.
**WHEN** a condition is `QueryItem::of_types([...])` with no tags.
**THEN** the degeneracy is documented rather than discovered.

**Falsifies:** that tag-keyed locking is a complete adapter strategy. It is exactly
correct under DCB's superset semantics — any event that could violate condition *C*
carries every tag of some item of *C* — provided writers also lock the tags of the events
they write. But **a tag-less condition has no key and collapses to a global lock over the
whole log**, and a tag-less condition is legal, idiomatic and *encouraged*:
`QueryItem::of_types` is a first-class constructor (`query.rs:85-91`), the append
documentation's own worked example uses it (`append.rs:23-31`), and only a fully
unconstrained item is rejected (`error.rs:66-71`).

**Rejects:** a Neon or Postgres adapter that ships tag-keyed locking without saying so.
One naive command handler makes the entire onsale single-threaded, and the cause is
invisible from the application side.

---

### E2E-56 — The corrected epoch case: a spurious conflict, not a silent double-claim

- **From:** Kestrel Cold Chain (`ClaimPooledUnit`) — recorded because the scenario got this wrong and the corrected version is a real test
- **Level:** integration
- **Spans:** `happenstance-core`, ⚠ `happenstance-sync`, ⚠ e2e crate

**GIVEN** two spokes holding different slices of one hub's log, and a decision whose
condition is made position-free by deriving a counter from the slice
(`epoch = count(releases) + 1`).
**WHEN** one spoke's slice has been pruned past a release so it under-counts by one.
**THEN** its condition guards the **previous** epoch, the hub still holds that epoch's
claim, and the append is **rejected and adjudicated** — not silently accepted.

**Falsifies:** the scenario's own claim that "both position-free appends succeed at the
hub with the invariant unenforced". They do not. The epoch scheme degrades into a
spurious conflict, which is survivable.

**Rejects:** the belief that epoch-tagging is unsafe under pruning at off-by-one. What it
does **not** rule out is off-by-more-than-one in a particular direction, which is
constructible and is what this case must actually construct: a slice pruned past two
releases derives an epoch nobody holds, and the condition then matches nothing anywhere.
That is the completeness failure, and it needs building explicitly rather than asserting.

**What survives regardless:** the trick itself needs no new type — just a tag — and
`AppendCondition::new` already makes the position-free form the default
(`append.rs:65-70`), with `after`/`after_opt` as opt-in builders. **The API already makes
the replicable shape the easy one**, which is worth recording because the only read helper
in the crate (`read_decision_model`, `store.rs:198-208`) exists to produce the
*position-relative* form and has no counterpart for the replicable one. If the sync design
lands on "conditions must be position-free to replicate", that helper is pointing the
wrong way, and it belongs in the ADR rather than being inverted silently.

---

## What cannot be written yet

Eleven cases above are marked blocked. They are blocked on decisions, not on effort, and
the decisions are the inputs to the architectural specification that follows this
document. Nothing here should be settled in passing.

**1. The shape of `EventId`.** `RUNBOOK.md:70` records it as **decided** — a Lamport pair
`(origin, origin_position)` on `SequencedEvent`, store-assigned — with no ADR. Three
things that row does not settle, and all three are load-bearing:

- **Is it queryable?** E2E-34 says a peer must dedupe without parsing `metadata`, or
  ADR-0003 breaks at the point it claims to win. Tag-visible has real costs
  (cardinality, every merge-scan, forgeability); a field on `SequencedEvent` that
  `Query` cannot see does not solve the dedup problem.
- **Does `origin` name a device or a store incarnation?** A tablet restored from
  backup, or an image cloned onto a second device, reissues the same origin positions to
  different events, and every peer's dedup then silently drops real facts. The fix — a
  128-bit value minted at database creation and re-minted on any restore — means origin
  cannot be the human-readable peer name, and the peer's stable identity has to live
  somewhere else.
- **Does it arrive with a store-assigned time?** E2E-43. Both are store-assigned facts
  about an event and both change `SequencedEvent::new`, which every adapter and the
  testkit call. One pass, or two breaking changes.

Blocks: E2E-33, E2E-34, E2E-36, E2E-41, E2E-43, and the whole of group C's dedup story.

**2. Whether ingest re-checks append conditions.** `RUNBOOK.md:78` calls this *the*
central question, and this catalogue supplies **three mutually contradictory answers from
three domains**, which is a result in itself:

- **Kestrel Cold Chain:** ingest must re-check *and never reject* — a violated condition
  produces an atomic losing-event-plus-compensation append, because a device that deletes
  a committed fact has lied to its user twice.
- **Kestrel Rotor:** ingest must **never** re-check, and the argument is general rather
  than domain-specific — rejection is a function of local state, so different peers reject
  different events and the union of facts is never reached. Convergence dies the moment
  ingest is allowed an opinion.
- **Wattline:** ingest need not re-check, because authority is pre-partitioned. A
  `SiteLeaseGranted` bounds the damage *before* the offline events are written, so the
  condition has nothing left to enforce.

These are reconcilable — Cold Chain's "re-check" produces a *compensation*, which is an
append, not a rejection, so it is a special case of unconditional ingest plus a hub-side
domain decision. But the reconciliation has to be written down, because the three
scenarios' wire formats and error taxonomies differ depending on which framing wins.

Blocks: E2E-37, E2E-38, E2E-39, E2E-40, E2E-45, E2E-47.

**3. The apply seam.** `RUNBOOK.md:68` records it as **decided — both**, ADR pending.
E2E-20 needs its write vocabulary; E2E-21 needs to know whether a batch is readable;
E2E-24 needs to know whether a batch is a live transaction or a deferred write set —
and the answer to the last one determines whether `Batch<'a> where Self: 'a` is
load-bearing or dead weight.

Blocks: E2E-20, E2E-21, E2E-22, E2E-24.

**4. Reset, and its scope.** `RUNBOOK.md:69`, open. E2E-15 through E2E-18 need the
method, its transactional scope, whether it is per `(store, ProjectionId)`, and whether
a projection can **refuse** it — `audit_trail_export` is the counter-pressure and
whatever lands must be refusable rather than merely discouraged.

**5. Projection failure policy.** `RUNBOOK.md:73`, open. E2E-26 and E2E-27 need it, and
this catalogue narrows the question: the policy cannot be one policy, and there is a
fourth option ("skip and record") that neither retry, halt nor dead-letter covers.

**6. The visibility invariant.** Not a ledger row; named in CLAUDE.md's open questions
and in `docs/evaluation/revised-runway.md`. E2E-01 cannot be written as a rule until the
invariant is stated, and cannot be written as a *useful* rule until the testkit has
something that can fail it — either a `Send + Sync` sub-trait to bind parallel rules on,
or a deterministic hostile fixture in the testkit's own `tests/` that assigns positions
out of visibility order. **Without the second, the rule ships having never failed
anything**, which is precisely what CLAUDE.md forbids.

**7. Whether a read is a snapshot**, and whether the items of one `Query` share one.
E2E-02 and E2E-03. Neon may not be able to meet the strong obligation in one round trip,
which is the reason to settle it before freezing rather than after.

**8. Multi-writer conformance.** ~~E2E-08 needs a documented requirement that
`factory()` called twice yields two handles onto the same backing store — the signature
`F: Fn() -> S` already permits it and nothing asks for it.~~ **Settled at phase 3.**
`Fn() -> S` was replaced by the `Fixture` trait, which names isolation and
sharing apart: one instance is one backing store, each `connect()` is a handle
onto it. CF-16 requires the second handle, CF-19 requires a rule to use it, and
`two_handles_observe_each_others_appends` is that rule. What remains open is the
far end rather than the obligation: every fixture in the workspace hands out
refcount clones of one in-process object, so no *connection* has yet been opened
twice.

**9. Cancellation.** E2E-07 needs one paragraph on `append` stating whether an adapter
may commit after its future is dropped. The honest statement is probably "may or may not
have committed", which is a documentable contract and is currently not documented at all.

**10. `Send` and `Sync` on `SendEventStore` and its `Error`.** E2E-53. Semver-visible,
costs wasm nothing, and cannot be added to one flavour because `trait_variant` copies
associated-type bounds verbatim. **Decide before publish.**

**11. Deletion, redaction and retained history.** E2E-46, E2E-47, E2E-48, E2E-49. Not a
ledger row anywhere, and the four are one decision rather than four: an append condition's
meaning is scoped to the store that evaluates it, and a store that has been deleted from
needs a way to say so. An explicit written refusal — deletion is out of scope for
`EventStore`, and here is what a store that has been deleted from is permitted to look
like — is a legitimate answer that closes E2E-46 and E2E-47 without new API. Silence is
not.

### Two things that are not blocked and are not cases

**A benchmark harness.** Norvant's `SuspendLane` decision motivates it and no conformance
rule can substitute: complexity is a benchmark, not an assertion, and a suite that asserted
on timings would be flaky. An adapter that scans where it should seek passes every rule
that can be written. The workspace has no benchmark harness and should not pretend a
conformance rule is one.

**An instrument for the completeness axis.** CLAUDE.md names `happenstance-postgres` and
`happenstance-neon` as instruments for the position-allocation and transport axes and names
nothing for completeness. The instrument this catalogue asks for is small: **a testkit-adjacent
store that deliberately holds only a suffix of its own log**, so that a runner or an ingest
path written against it fails loudly rather than being accidentally correct. Four of the six
scenarios would exercise it.
