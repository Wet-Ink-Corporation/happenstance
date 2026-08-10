# Research: what other DCB implementations ship that happenstance does not

Date: 2026-08-05. Scope: compare happenstance's contract crate + planned
`happenstance-runtime` seam against canonical DCB implementations across
languages, per the assignment brief.

## Sources actually read

- **Wwwision/dcb-eventstore** (PHP, Bastian Waidelich's reference DCB library)
  and **Wwwision/dcb-example-courses** (the reference worked example — the
  same domain happenstance's `course-subscriptions` example uses). Read
  `DecisionModel.php`, `CommandHandler.php` (`buildDecisionModel()`),
  `Projection.php`, `ClosureProjection.php`, `StreamCriteriaAware.php` verbatim
  via raw.githubusercontent.com.
- **dcb.events/resources/libraries/** — the canonical implementations list
  (fetched 2026-08-05).
- **disintegrate-es/disintegrate** (Rust, Postgres-bound, macro-driven) — its
  README: `StateQuery`/`Decision` traits, derive macros, tuple `StateQuery`
  composition, snapshot plumbing, listener feature.
- **umadb-dcb** (Rust, `docs.rs/umadb-dcb`) — the trait surface of UmaDB's
  Rust bindings.
- **eventsourcing (Python)**, DCB topic docs — enduring objects, slices,
  groups, `DCBRecorder.subscribe()`.
- Axon Framework 5 / AxonIQ blog posts on DCB (`EventCriteria`,
  `CriteriaResolver`, `TagResolver`, consistency markers) — searched, not
  deep-read; experimental/non-production as of Axon Server 2025.1.
- Search hits for Marten 9.0's new DCB support (.NET/Postgres, HSTORE-based) —
  noted as a recent data point, not deep-read.

Not found / not fruitful: `@dcb-es/event-store` (TS) does not appear to exist
under that name; disintegrate's retry-on-conflict mechanism is undocumented in
its README and no source dive was done (time-boxed).

## The one finding that matters most: query and fold are one artifact in the reference implementation, two in happenstance's example

`examples/course-subscriptions/src/main.rs:113-174` (`subscribe()`) hand-writes
a `Query` via `QueryItem::new([...], Tags::from_pairs([...]))`, then separately
hand-folds the events in a `for sequenced in &events` loop matching on
`event_type().as_str()`. The two are adjacent and clearly related by the
author's discipline, but nothing in the type system ties them together. It
would compile fine if the query added a new event type and the fold forgot to
handle it, or vice versa.

Wwwision's reference implementation does not allow that split to exist.
`Projection.php` is an interface with exactly two methods:

```php
interface Projection {
    public function initialState(): S;
    public function apply(S $state, DomainEvent $event, EventEnvelope $envelope): S;
}
```

and `ClosureProjection` — the concrete, closure-based implementation used
throughout `dcb-example-courses` — registers handlers with `when(EventClass,
callback)` and *derives* its `getCriteria()` (the query fragment) from the set
of registered handler classes:

> "The `getCriteria()` method generates filtering criteria by extracting
> event type names from registered handlers."

So the query is generated from the fold, not maintained beside it. Composition
follows the same shape — `CommandHandler::buildDecisionModel()`:

```php
private function buildDecisionModel(Projection ...$projections): DecisionModel
{
    $query = StreamQuery::wildcard();
    $compositeProjection = CompositeProjection::create($projections);
    $query = $query->withCriteria($compositeProjection->getCriteria());
    $expectedHighestSequenceNumber = ExpectedHighestSequenceNumber::none();
    $state = $compositeProjection->initialState();
    foreach ($this->eventStore->read($query) as $eventEnvelope) {
        $domainEvent = $this->eventSerializer->convertEvent($eventEnvelope->event);
        $state = $compositeProjection->apply($state, $domainEvent, $eventEnvelope);
        $expectedHighestSequenceNumber = ExpectedHighestSequenceNumber::fromSequenceNumber($eventEnvelope->sequenceNumber);
    }
    return new DecisionModel($state, new AppendCondition($query, $expectedHighestSequenceNumber));
}
```

`CompositeProjection::create($projections)` merges N projections' criteria
into **one** OR'd query, folds all of them in **one** pass over **one** read,
and the resulting `AppendCondition` falls out mechanically — nobody writes
`Query::from_items([...])` by hand at the call site the way
`course-subscriptions` does for the multi-item `subscribe()` case
(`main.rs:114-125`).

`happenstance-runtime/src/lib.rs:31-34` names the same shape as a planned
feature —

> "`DecisionModel` — folds a projection of read events into the state a
> command handler decides on, and produces the matching `Query`. Composing
> several of these into one query is the mechanism that makes a dynamic
> consistency boundary *dynamic*."

— and `docs/RUNBOOK.md:235-238` repeats the intent for phase 3. Nothing is
built yet. This is squarely on the critical path to 0.1 (phase 3 gates phase
7), so it is worth landing the concrete mechanism *now*, not reinventing it
under time pressure later. The PHP reference is the closest prior art; the
Rust-idiomatic version of "compose N of these" already exists too — see below.

## Rust-native precedent for composing decision models: disintegrate's tuple `StateQuery`

`disintegrate` (the other real Rust DCB library) composes multiple state
queries into one `Decision` by letting `StateQuery` be a **tuple**:

```rust
impl Decision for ApplyCoupon {
    type Event = DomainEvent;
    type StateQuery = (Cart, Coupon);
    ...
}
```

This is a good target shape for `happenstance-runtime`'s `DecisionModel`
composition: implement the projection-composition trait for tuples
`(P1, P2)`, `(P1, P2, P3)`, ... (a small macro, same trick `impl Trait for
(A,), (A, B), (A, B, C)` uses throughout the ecosystem — e.g. `axum`'s
extractor tuples), each producing its own `Query` fragment OR'd together and
its own slice of the folded state. That gives the PHP reference's
`CompositeProjection::create(...)` runtime-list ergonomics *and* a
compile-time-checked, no-heap-allocation-for-the-common-case Rust idiom, for
free once one macro is written.

`disintegrate`'s `#[derive(StateQuery)]` / `#[derive(Event)]` macros are
explicitly *not* something to copy yet: `happenstance-runtime/src/lib.rs:29`
already defers derive macros to a future `happenstance-macros` crate
deliberately, "until there is something for it to derive." That sequencing
is correct — disintegrate's own macros exist to remove boilerplate around a
hand-written trait API that has to be validated first. Building the derive
before the trait it derives is proven is the wrong order.

## Gap: no live subscription / catch-up tailing primitive, and it is not even in the decision ledger

`EventStore::read` (`crates/happenstance/src/store.rs:117-121`) is a bounded,
one-shot read of a `Query` snapshot: `ReadOptions` has `from`, `backwards`,
`limit` — no "keep going as new events land." Every implementation surveyed
that has matured past a toy example ships an explicit second primitive for
this:

- **Python `eventsourcing`**: `DCBRecorder.subscribe()` — "subscriptions will
  block when the last recorded event is received, and then continue when new
  events are recorded."
- **umadb-dcb**: `DcbSubscriptionAsync` / `DcbSubscriptionSync` are first-class
  types, distinct from `DcbReadResponseAsync/Sync` (the bounded read). The
  crate ships a stream-cancellation handle (`StreamCancelHandle`) as part of
  the subscription API, which is its own small piece of design (how does a
  caller stop tailing without dropping the whole connection).
- **Axon Framework 5 / Axon Server 2025.1**: DCB is explicitly built around
  live tag-based streaming with "consistency markers," not one-shot reads.

happenstance's own planned "projection runner"
(`happenstance-runtime/src/lib.rs:37-40`, `docs/RUNBOOK.md:242`) has nowhere
to live-tail from. Built against only `EventStore::read`, it will have to poll
— call `read` in a loop with `ReadOptions::from(checkpoint.next())` and sleep
between empty results — which is exactly the kind of thing a port abstraction
exists to avoid: every adapter reimplements its own poll loop and its own
guess at an interval, instead of a SQLite adapter using `LISTEN/NOTIFY`-free
polling internally today and a future Postgres-backed adapter using `LISTEN`
under the same trait method tomorrow.

Checked and confirmed absent: no `subscribe`, `watch`, `tail`, or "catch-up"
concept anywhere in `crates/` (grepped `subscri|watch|tail|live` across the
workspace — the only hits are the `course-subscriptions` example name and
`memory.rs`'s internal `Snapshot` stream type, which is a copy-on-read
implementation detail, not a live-tail feature). More importantly, unlike
snapshotting, this isn't even named as a *deliberately deferred* open
question in `docs/RUNBOOK.md`'s decision ledger — the ledger names the
Ladybug checkpoint placement, the sync merge rule, the SQLite driver choice,
and half a dozen other genuinely-open questions, but not this one. Given the
project's own stated failure mode ("settling a question silently in passing"
— CLAUDE.md, RUNBOOK.md rule 3), the absence of a row is itself the gap, even
before any code is written.

Recommendation: it does not need to land for 0.1 — a polling projection
runner is a legitimate, honest MVP — but the *port-level decision* (does
`EventStore` grow a `subscribe`/`watch` method now, while adding it is free,
or is live-tailing deliberately out of scope for 0.1 and left to a wrapper
crate later) should be made explicitly and recorded, precisely because
`EventStore` is the trait every adapter implements and retrofitting a new
required method onto it after phase 1 (SQLite) and phase 5 (Cloudflare) both
exist is the expensive order to do this in.

## Gap: snapshotting (decision-model state) is absent and also untracked

Not a single line in `docs/adr/`, `docs/RUNBOOK.md`, or any crate's module
docs mentions snapshotting a decision model's folded state to skip replay.
Compare:

- `disintegrate` threads a `Snapshot` strategy type through its
  `decision_maker` constructor (`NoSnapshot` in the examples), i.e. the API
  shape already has a slot for it even though the built-in strategies are
  thin.
- Python `eventsourcing`'s DCB docs gesture at snapshotting for enduring
  objects as a documented (if unelaborated) future direction.

This is a legitimately low-priority gap — a DCB query is usually narrow by
construction (a course + its subscribers, not "replay the whole log"), which
is precisely why DCB implementations lean on this less than classical
aggregate-per-stream event sourcing does. But "low priority" is not the same
as "silently absent," and CLAUDE.md's own house rule about the failure mode
of settling things without recording them applies here as much as it does to
the Ladybug or sync questions that *are* in the ledger. This is a one-line,
zero-cost fix: add a row.

## What happenstance already gets right relative to its peers (no gap — say so)

- **`trait_variant`-derived `Send`/`!Send` split (ADR-0001) beats duplicated
  trait hierarchies.** `umadb-dcb` ships `DcbEventStoreAsync` and
  `DcbEventStoreSync` as two hand-maintained trait families (plus
  `DcbReadResponseAsync/Sync`, `DcbSubscriptionAsync/Sync` — the duplication
  is pervasive, not a one-off). Every method, every associated type, doubled,
  with the obligation to keep both in sync by hand. happenstance's single
  `EventStore` trait with `#[trait_variant::make(SendEventStore: Send)]`
  deriving the `Send` flavour mechanically is strictly better engineering for
  the same problem, and CLAUDE.md already calls this out as the sort of thing
  new-to-Rust developers need explained — it is explained, correctly, in
  `store.rs`'s module docs.
- **`append` returning `SequencePosition`** (`store.rs:126`) matches the
  Python reference's `append()` returning an `int` specifically so callers
  can build a follow-up append condition without a second read — this is
  standard practice among the mature implementations, not an outlier
  choice, and happenstance already does it.
- **Deferring derive macros until the hand-written trait API is proven**
  mirrors what a mature implementation (disintegrate) did in the opposite
  order it should have: disintegrate's derive macros exist to remove
  boilerplate from an already-stable trait API. happenstance-runtime holding
  off on `happenstance-macros` until `DecisionModel`/`DomainEvent` exist and
  are validated is the correct sequencing, not a gap.
- **Multi-tenancy, partitioning, and observability** are absent from every
  implementation surveyed, not just happenstance's. Nothing here is a
  competitive gap; omitting them from 0.1 is correct and does not need a
  RUNBOOK row the way subscriptions/snapshotting do, because no peer treats
  them as core-port concerns either — they are typically deployment-layer
  (multi-tenancy = separate stores/schemas; observability = wrap the trait
  in a tracing decorator).
- **Retry-on-conflict is unimplemented everywhere, including the reference.**
  Checked `dcb-example-courses`' `CommandHandler.php` directly: no retry loop
  around `ConditionViolated`/append conflicts anywhere in it — the exception
  propagates to the caller, exactly like happenstance's
  `course-subscriptions/src/main.rs:216-221` (`bail!("concurrent
  modification...")`). This means happenstance is not behind the state of
  the art here — nobody has shipped this well yet — but it also means it is
  free real estate: a small `with_retry` combinator in the phase-3 command
  loop (bounded attempts, rebuild the decision model, re-append) would be a
  genuine differentiator rather than table stakes to catch up on.
