# happenstance — correctness, soundness and hidden defects

Reviewed: every line of `D:\repos\happenstance\crates\happenstance\src\` (2 453 loc), plus
`crates/happenstance-testkit/src/suite.rs` where it defines what "correct" means for an adapter.

Baseline: `cargo test --workspace --all-features` is green (rustc 1.97.1, edition 2024).
Every claim below is backed by a compiled, executed repro. Repros live in
`C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-happenstance\559ba08d-08bc-4f98-b677-6b98e7a98078\scratchpad\corr\probe\`.

---

## Headline

The contract types are in very good shape — `contains_all`, the backwards/from/limit
composition, `is_violated_by` and the hand-rolled `collect` are all *provably* correct, and
the two-flavour `Send` design works end to end. Three things are not:

1. **The `serde` feature does not round-trip.** Five `skip_serializing_if` attributes make
   `Event`, `SequencedEvent`, `QueryItem`, `Query` and `AppendCondition` fail to deserialise in
   any non-self-describing format (postcard, bincode) for their *most common* shapes. This
   feature exists for exactly one consumer — `happenstance-sync` — and there is not one
   round-trip test in the workspace.
2. **The conformance suite does not pin `read`.** I wrote an adapter that passes all 27 rules
   while silently ignoring `limit`, returning nothing for a `from` that lands in a gap, and
   disagreeing with the reference store about empty-batch precedence.
3. **`ProjectionStore` cannot be implemented by the next adapter on the roadmap**, and its
   `Batch` associated type has no bound, so nothing can be written into it.

---

# Findings

## C1 — `serde` wire types do not round-trip in non-self-describing formats  ·  HIGH  ·  defect

**Location:** `crates/happenstance/src/event.rs:342-345`, `crates/happenstance/src/query.rs:281-284`,
`crates/happenstance/src/append.rs:121-122`

```rust
// event.rs:337
#[derive(Serialize, Deserialize)]
#[serde(rename = "Event")]
struct EventWire {
    event_type: EventType,
    data: Bytes,
    #[serde(default, skip_serializing_if = "crate::Tags::is_empty")]   // <-- 342
    tags: crate::Tags,
    #[serde(default, skip_serializing_if = "Option::is_none")]         // <-- 344
    metadata: Option<Bytes>,
}
```

`skip_serializing_if` works by passing a *shortened* field count to `serialize_struct`. A
self-describing format writes field names, so the deserializer's `#[serde(default)]` fills the
gap. A non-self-describing format writes fields positionally with no names and no count, and
its deserializer feeds exactly `FIELDS.len()` values in order — so a skipped field desyncs the
stream and the next read runs off the end.

**Repro** (`src/main.rs` in the probe crate, postcard 1.x):

```
Event(bare)                bytes [1, 65, 2, 120, 121]        -> ERR DeserializeUnexpectedEnd
SequencedEvent(bare event)                                   -> ERR DeserializeUnexpectedEnd
Query::Items(types only)   bytes [1, 1, 1, 1, 65]            -> ERR DeserializeUnexpectedEnd
Query::Items(tags only)    bytes [1, 1, 1, 3, 99, 58, 49]    -> ERR DeserializeUnexpectedEnd
AppendCondition(no after)  bytes [1, 1, 1, 1, 65, 1, 3, ...] -> ERR DeserializeUnexpectedEnd
Event(full)                                                  -> OK
Query::Items(both)                                           -> OK
ReadOptions                                                  -> OK      (no skip attrs)
```

Note *which* shapes break. `Event::new("A", data)` with no tags — the shape every doctest in
this crate builds. `QueryItem::of_types([...])` and `QueryItem::tagged(...)` — the two
convenience constructors. `AppendCondition::new(q)` with no `after` — the "check the whole log"
case the docs call the classic uniqueness check. The only shapes that survive are the fully
populated ones.

This is worse than a normal bug because it is *asymmetric*: serialisation succeeds silently and
produces bytes that look fine. `happenstance-sync` will write a peer protocol, pick postcard or
bincode for the wire (the obvious choice for a Cloudflare Worker), and discover it at
integration time.

**Consequence.** The one feature flag that exists to serve replication is broken for the
formats replication would choose. `ReadOptions` works only by accident — its wire struct
happens to carry no `skip_serializing_if`.

**Fix.** Delete all five `skip_serializing_if` attributes. Keep `#[serde(default)]` so old JSON
still parses. The cost is ~10 bytes of JSON per event; the benefit is a format-agnostic wire
type. Then add the test that would have caught it, over both a self-describing and a
non-self-describing format:

```rust
// crates/happenstance/tests/wire.rs, #[cfg(feature = "serde")]
fn round_trip<T: Serialize + DeserializeOwned + PartialEq + Debug>(v: &T) {
    assert_eq!(v, &serde_json::from_str(&serde_json::to_string(v).unwrap()).unwrap());
    assert_eq!(v, &postcard::from_bytes(&postcard::to_allocvec(v).unwrap()).unwrap());
}
// exercise the *sparse* shapes: bare Event, types-only QueryItem, tags-only
// QueryItem, Query::All, AppendCondition without `after`.
```

Even if the decision is to keep the attributes, the crate must then say in `lib.rs` that the
`serde` feature supports self-describing formats only — right now nothing warns anyone.

---

## C2 — the conformance suite lets two adapters disagree about `read`  ·  HIGH  ·  gap

**Location:** `crates/happenstance-testkit/src/suite.rs:262-329` (read-option rules),
`suite.rs:406-414` (`append_rejects_empty_batch`)

I wrote a store — `tests/rogue.rs` in the probe crate — that **passes all 27 conformance
rules** while doing three things `MemoryEventStore` does not:

| deviation | rogue | memory |
|---|---|---|
| `limit` on a **forwards** read that also has `from` | ignored entirely | applied |
| `from` naming a position **no event occupies** (a gap) | yields nothing | comparison, yields the tail |
| `append(&[], Some(violated_condition))` | `NoEvents` | `ConditionViolated` |

```
running 28 tests
............................
test result: ok. 28 passed; 0 failed
```

(27 conformance rules + one assertion test proving the divergences.)

**Why each slips through.**

*Forwards + `from` + `limit` is never tested.* The read-option rules are
`read_from_is_inclusive` (from, no limit), `read_limit_truncates` (limit, no from),
`read_backwards_reverses_order`, and `read_backwards_from_with_limit` (from + backwards +
limit). There is no forwards + from + limit rule and no backwards + limit rule. Paging forward
through a log is the single most common read an application does.

*`from` in a gap is never tested.* Every rule sources its `from` from a position the store
actually assigned:

```rust
// suite.rs:314
let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;
let fourth = all[3].position;
```

That is the right instinct — CLAUDE.md's own rule about never asserting literal positions — but
it means the suite only ever asks about positions that exist. Since gaps are explicitly legal,
"what does `from` mean when it lands in a gap" is a question every sparse adapter must answer
and none is held to.

**This is not academic.** `ProjectionStore::checkpoint`'s doc (`projection.rs:86-88`) prescribes
the resume recipe:

> Feed this to `ReadOptions::from` — after advancing past it — to resume a replay.

Advancing past a `SequencePosition` means `next()`, i.e. `checkpoint + 1`. On any adapter with
gaps that position usually holds no event. Under memory's comparison semantics the replay
resumes correctly; under exact-seek semantics it returns nothing and **the projection stalls
forever, silently**. Both adapters are "conformant".

**Fix.** Three new rules, all expressible without literal positions:

```rust
/// `from` selects by comparison, not by identity: a position that no event
/// occupies still bounds the read. Gaps are legal, so this must be pinned.
pub async fn read_from_a_position_with_no_event<S: EventStore, F: Fn() -> S>(factory: F) {
    let store = factory();
    append_ok(&store, &[event("A"), event("B"), event("C")]).await;
    let all = read_ok(&store, &Query::all(), ReadOptions::new()).await;

    // `last.next()` is guaranteed to hold no event; ditto backwards from it.
    let past_the_end = all[2].position.next().expect("no overflow in a test");
    assert!(read_ok(&store, &Query::all(), ReadOptions::new().from(past_the_end)).await.is_empty(),
        "a forwards read starting past the head must be empty, not an error");
    assert_eq!(
        positions_of(&read_ok(&store, &Query::all(),
            ReadOptions::new().from(past_the_end).backwards()).await),
        { let mut p = positions_of(&all); p.reverse(); p },
        "`from` bounds by comparison: a backwards read from beyond the head sees the whole log");
}

/// forwards + `from` + `limit` compose.
pub async fn read_from_with_limit<S: EventStore, F: Fn() -> S>(factory: F) { /* ... */ }

/// backwards + `limit`, no `from`.
pub async fn read_backwards_with_limit<S: EventStore, F: Fn() -> S>(factory: F) { /* ... */ }
```

and a decision + rule for C3 below. Also: state the `from` semantics on `ReadOptions::from`
itself as "the read is bounded by `position >= from` (or `<= from` when backwards); `from` need
not name an existing event."

---

## C3 — empty-batch vs condition-violation precedence is unspecified  ·  MEDIUM  ·  gap

**Location:** `crates/happenstance/src/memory.rs:197-216`, `store.rs:135-141`,
`suite.rs:406-414`

```rust
// memory.rs:197 — condition first ...
if let Some(condition) = condition {
    let conflict = stored.iter().find(|existing| { ... });
    if let Some(conflict) = conflict { return Err(AppendError::ConditionViolated(...)); }
}
// memory.rs:211 — ... empty check second
if events.is_empty() { return Err(AppendError::NoEvents); }
```

Observed:

```
append(&[], Some(violated))     = Err(ConditionViolated(..{ conflicting_position: Some(1) }))
append(&[], Some(non-violated)) = Err(NoEvents)
```

The suite only ever calls `store.append(&[], None)` (`suite.rs:408`), so both orderings are
conformant. My rogue adapter checks empty-first and passes.

This matters more than it looks. A caller that treats `is_condition_violated()` as "rebuild and
retry" will, on a memory-backed store, retry forever on a batch that is empty because of a
caller bug — the condition is genuinely violated, so the retry loop never terminates and never
surfaces the real error. On a different adapter the same code reports `NoEvents` immediately.

**Fix.** Decide, document on `EventStore::append`, add a rule. I would put the empty check
first: `NoEvents` is a caller bug, `ConditionViolated` is a runtime outcome, and reporting a
runtime outcome for a malformed call is misleading. Argument for the current order: the store
should report what *it* observed regardless of the batch. Either is defensible; silence is not.

**Also:** `AppendError::NoEvents` is a documented variant of the return type and a mandatory
conformance rule, but it does **not** appear in `EventStore::append`'s `# Errors` section
(`store.rs:135-141` lists only `ConditionViolated` and `Store`). The house rule says every
fallible public function needs a complete `# Errors` section.

---

## C4 — `ProjectionStore::Batch` has no bound, so nothing can be written to it  ·  HIGH  ·  design-risk

**Location:** `crates/happenstance/src/projection.rs:80-82, 93-121`

```rust
type Batch<'a> where Self: 'a;                                    // no bound at all
async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error>;
async fn commit(&self, batch: Self::Batch<'_>, id: &ProjectionId, position: SequencePosition)
    -> Result<(), Self::Error>;
async fn rollback(&self, batch: Self::Batch<'_>) -> Result<(), Self::Error>;
```

The module doc says (`projection.rs:26`) "Applying an event mutates this". Nothing in the trait
permits that. `Batch<'a>` is an unconstrained associated type — generic code holding a
`P::Batch<'_>` knows nothing about it and can do exactly two things: hand it to `commit`, or
hand it to `rollback`. There is no `apply`, and no bound like
`type Batch<'a>: ReadModelWrite` that would give one.

So the only code that can actually project is code that names the concrete adapter — which is
precisely what a port exists to avoid. The port as written can express the *transaction
lifecycle* but not the *transaction*.

This is the load-bearing gap behind "the port is provisional": the missing piece is not a
conformance suite, it is the write half of the interface.

---

## C5 — `ProjectionStore` is not implementable by a `rusqlite` adapter in the `Send` flavour  ·  HIGH  ·  design-risk

**Location:** `crates/happenstance/src/projection.rs:80-82, 100`

`rusqlite::Transaction<'conn>` borrows the `Connection`. `rusqlite::Connection` is `Send` but
**not `Sync`**. Every `ProjectionStore` method takes `&self`. That forks into two options and
both fail.

**Option A — connection behind a `Mutex` (required to make the store `Sync`, which
`SendProjectionStore` needs).** The batch must carry the guard *and* the transaction borrowing
through it — a self-referential struct:

```rust
type Batch<'a> = (MutexGuard<'a, Connection>, Transaction<'a>) where Self: 'a;

async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error> {
    let guard = self.conn.lock().map_err(|_| Err0)?;
    let txn = guard.unchecked_transaction();
    Ok((guard, txn))
}
```

```
error[E0515]: cannot return value referencing local variable `guard`
error[E0505]: cannot move out of `guard` because it is borrowed
```

**Option B — store owns the `Connection` directly.** `Batch<'a> = Transaction<'a>` compiles,
but `Connection: !Sync` makes `&Store: !Send`, so every future is `!Send`:

```
error: future cannot be sent between threads safely
  = help: within `Store`, the trait `Sync` is not implemented for `Cell<u32>`
note: captured value is not `Send` because `&` references cannot be sent unless their referent is `Sync`
   |     async fn checkpoint(&self, ...) 
   |                         ^^^^^ has type `&Store` which is not `Send`, because `Store` is not `Sync`
```

So `happenstance-sqlite` can only ever implement the `!Send` flavour of the projection port —
on native, with tokio — which defeats the point of having two flavours.

The escape is the shape every mature Rust DB wrapper converges on: don't hand out a borrowed
transaction, hand out an **owned** batch and let the adapter open/commit its transaction inside
`commit`:

```rust
/// An accumulated set of read-model writes. Owned, so it can cross an await
/// and does not pin the connection open while the caller thinks.
type Batch: Default + Send;    // no GAT, no lifetime

fn batch(&self) -> Self::Batch;   // or just Default::default()
async fn commit(&self, batch: Self::Batch, id: &ProjectionId, position: SequencePosition)
    -> Result<(), Self::Error>;
```

The transactional invariant the module doc defends is *stronger* under this shape, not weaker:
the adapter opens the transaction, replays the batch, writes the checkpoint and commits, all
inside one function it fully controls. It also removes C6 below for free.

If the borrowed form must be kept, `Batch<'a>` needs a bound and the `rusqlite` case needs a
worked answer before publish — a published port that the flagship adapter cannot implement is
a breaking change waiting to happen.

---

## C6 — `commit` accepts a batch that came from a different store  ·  MEDIUM  ·  design-risk

**Location:** `crates/happenstance/src/projection.rs:109-114`

```rust
async fn commit(&self, batch: Self::Batch<'_>, id: &ProjectionId, position: SequencePosition)
```

The elided `'_` is a **fresh** lifetime parameter of the method, unrelated to `&self`'s. So
`commit<'a, 'b>(&'a self, batch: Self::Batch<'b>, ...)`. Nothing ties the batch to the receiver.
Compiled and executed:

```
  commit through A, batch built by A  -> same store? true
  commit through B, batch built by A  -> same store? false
cross-store commit COMPILED AND RAN
```

For a real adapter that means committing store A's SQL transaction through store B's connection
— B writes the checkpoint on its own connection while A's read-model writes sit in A's
transaction. The one invariant the module doc exists to defend ("the two writes must be **one**
transaction", `projection.rs:19`) is broken, at runtime, with no compiler complaint.

Not memory-unsafe — a logic hazard. The type system *can* prevent it (a brand/invariant
lifetime, or the owned-batch shape in C5, which makes it unrepresentable). Given how much of
this codebase's design rests on "illegal states are unrepresentable", this one deserves the
same treatment.

---

## C7 — `SequencePosition::next()` saturates instead of overflowing  ·  MEDIUM  ·  defect

**Location:** `crates/happenstance/src/event.rs:138-144`

```rust
/// The next position, or `None` on overflow.
pub const fn next(self) -> Option<Self> {
    Self::new(self.0.get().saturating_add(1))
}
```

`saturating_add` means the value can never reach zero, so `Self::new` can never return `None`.
Verified:

```
next(u64::MAX) = Some(SequencePosition(18446744073709551615))
equals self? true
```

The doc comment is simply false. Worse, this is the API an adapter reaches for to allocate the
next dense position, and at the boundary it hands back the position it was given — producing a
**duplicate position**, which violates the specification's uniqueness MUST that
`positions_are_unique` exists to enforce. Silently returning a wrong answer where the signature
already has a way to say "I can't" is the wrong trade in a contract crate.

```rust
pub const fn next(self) -> Option<Self> {
    match self.0.checked_add(1) {
        Some(value) => Some(Self(value)),
        None => None,
    }
}
```

(`NonZeroU64::checked_add` is const and returns `Option<NonZeroU64>` directly — no re-validation
needed.) Unreachable in practice at 2^64 events; a one-line fix that makes a documented
guarantee true.

---

## C8 — `EventStore::read` promises laziness; the reference implementation is eager  ·  MEDIUM  ·  doc

**Location:** `crates/happenstance/src/store.rs:101-110` vs `crates/happenstance/src/memory.rs:150-182`

The trait doc:

> The returned stream is **lazy**: nothing is executed until it is first polled, and failures
> surface as `Err` items rather than up front. That is what lets an adapter stream a
> million-event replay without buffering it

`MemoryEventStore::read` is not `async`, so `let guard = self.read_guard();` at `memory.rs:155`
runs at **call** time. Proved:

```
stream built before a later append yields 1 event(s) -> EAGER (snapshot at call time)
```

i.e. `let s = store.read(..); store.append(..).await; collect(s).await` does not see the append.
A lazy adapter would. Nothing in the conformance suite tests laziness, so both behaviours ship.

Two knock-ons. First, an observable semantic difference between adapters that no rule covers —
the same class of problem as C2. Second, the module doc's "so a read never holds the lock across
a poll — correct, and deliberately simple" (`memory.rs:26-27`) is true but describes a different
property than the trait promises.

**Fix (pick one, and say so):**
- Weaken the trait doc to "an adapter *may* be lazy; do not depend on when the snapshot is
  taken", and drop the "million-event replay" claim from a paragraph that the reference
  implementation contradicts; **or**
- state that the snapshot point is the `read` call and add a rule pinning it (this is the more
  useful contract for a DCB reader, because it makes `read_decision_model`'s returned position a
  meaningful boundary rather than a race).

The second is the better contract. Right now it is neither.

---

## C9 — `Option<Query>` collapses under `serde`  ·  MEDIUM  ·  defect

**Location:** `crates/happenstance/src/query.rs:304-321`

```rust
match self {
    Self::All => serializer.serialize_none(),
    Self::Items(items) => serializer.serialize_some(items),
}
```

Encoding an enum as `Option` means `Query` is *indistinguishable from absence*. Nesting one
inside an `Option` loses information:

```
Some(Query::All) -> {"q":null,"n":1}
None             -> {"q":null,"n":1}
same bytes? true
Some(Query::All) round trips to None
```

`happenstance-sync`'s protocol will plausibly carry an optional filter (`Option<Query>` on a
subscription request), and `Option<AppendCondition>` mirrors `EventStore::append`'s own
signature. In postcard the outer tag byte saves it; in JSON it does not.

The representation is also strange on its face: `Query::All` serialising as JSON `null` reads as
"no query" to any non-Rust peer, which is the opposite of what it means.

**Fix.** Use an externally-tagged enum — `{"all": null}` / `{"items": [...]}` — or serialise as
`Option<Vec<QueryItem>>` explicitly at the *field* level in `happenstance-sync` and leave
`Query` with a self-describing representation. Given the crate has not published, prefer the
honest representation.

---

## C10 — `Event::new` cannot accept an `EventType` you already hold  ·  MEDIUM  ·  ergonomics

**Location:** `crates/happenstance/src/event.rs:197-200`

```rust
pub fn new(
    event_type: impl TryInto<EventType, Error = InvalidEventType>,
    data: impl Into<Bytes>,
) -> Result<Self, InvalidEventType>
```

`Error = InvalidEventType` is an equality constraint. The blanket
`impl<T, U: Into<T>> TryFrom<U> for T` gives `EventType: TryInto<EventType, Error = Infallible>`,
which does not satisfy it:

```
error[E0271]: type mismatch resolving `<EventType as TryInto<EventType>>::Error == InvalidEventType`
  7 |     let e = Event::new(ty.clone(), &b"{}"[..]);
    |                        ^^^^^^^^^^ expected `InvalidEventType`, found `Infallible`
```

So a typed layer that interns its `EventType`s — which is exactly what `happenstance-runtime`
will do, one per `DomainEvent` — must convert back to a `&str` and re-validate on every event
construction, and still handle a `Result` that cannot fail.

The crate already knows the fix; `QueryItem::new` (`query.rs:57-61`) uses it:

```rust
pub fn new<T>(event_type: T, data: impl Into<Bytes>) -> Result<Self, InvalidEventType>
where
    T: TryInto<EventType>,
    InvalidEventType: From<T::Error>,
{ ... }
```

with `impl From<Infallible> for InvalidEventType { fn from(n: Infallible) -> Self { match n {} } }`
alongside the existing one in `error.rs:77-84`. Consider also an infallible
`Event::of(event_type: EventType, data: impl Into<Bytes>) -> Self` for the typed layer, so the
hot path returns no `Result` at all.

---

## C11 — `position_at`'s two unreachable fallbacks silently corrupt if reached  ·  LOW  ·  defect

**Location:** `crates/happenstance/src/memory.rs:130-136`

```rust
fn position_at(index: usize) -> SequencePosition {
    let raw = u64::try_from(index).unwrap_or(u64::MAX - 1).saturating_add(1);
    SequencePosition::new(raw).unwrap_or(SequencePosition::FIRST)
}
```

Both fallbacks are dead on every target Rust supports:

- `u64::try_from(usize)` is infallible for `usize <= 64` bits, so `unwrap_or` never fires;
- `raw >= 1` always after `saturating_add(1)` on a `u64`, so `SequencePosition::new` never
  returns `None` and `unwrap_or(FIRST)` never fires.

If either *did* fire the result would be a duplicate or a reset position — the exact corruption
this store is the oracle against. That is the wrong failure mode for a reference
implementation: an oracle should be loud. Given `unwrap_used` is a warn-level lint, the
idiomatic form is a documented `expect` (the panic *is* the correct outcome here) or a
`debug_assert!`. As it stands, three lines of defensive code make the store lie instead of stop.

Related and equally dead: `position_at(usize::MAX)` and `position_at(usize::MAX - 1)` both
return `u64::MAX` — a collision — for the same saturating reason as C7.

---

## C12 — the poison-recovery justification does not hold as written  ·  LOW  ·  doc

**Location:** `crates/happenstance/src/memory.rs:118-127`

```rust
/// A panic in another thread while holding the lock cannot have left this
/// store inconsistent: every mutation happens in one `extend` under the
/// write lock, and the validation that precedes it does not mutate. So
/// poisoning carries no information here and is ignored rather than
/// propagated as a spurious failure.
fn read_guard(&self) -> std::sync::RwLockReadGuard<'_, Vec<SequencedEvent>> {
    self.events.read().unwrap_or_else(PoisonError::into_inner)
}
```

"Every mutation happens in one `extend`" does not imply atomicity — `Vec::extend` is not atomic.
If the iterator panics part-way the earlier elements stay:

```
extend panicked: true
lock poisoned:   true
state after the panic: [0, 1, 2]  <- a PARTIALLY applied batch
```

That is precisely the state `read_guard()` would hand out with the poison ignored, and it
violates the atomicity guarantee `append_is_atomic` enforces.

The conclusion still holds, for a reason the comment does not state: the closure passed to
`extend` (`memory.rs:219-221`) calls only `position_at` and `Event::clone`, and neither can
panic — `Bytes::clone` is a refcount bump that aborts rather than panics on overflow, and
allocation failure aborts. Say *that*, because that is the property a future change could break:

```rust
// Ignoring poison is safe here for one reason only: the closure inside the
// `extend` below cannot panic (`Event::clone` is refcount bumps and
// allocations, both of which abort rather than unwind). `Vec::extend` is NOT
// atomic — a panicking element iterator would leave a partial batch visible
// through this guard. Anything added to that closure must preserve this.
```

---

## C13 — the backwards read materialises the whole match set before truncating  ·  LOW  ·  performance

**Location:** `crates/happenstance/src/memory.rs:163-178`

```rust
let mut selected: Vec<SequencedEvent> = if options.backwards {
    matched.rev().filter(..).cloned().collect()
} else {
    matched.filter(..).cloned().collect()
};
if let Some(limit) = options.limit { selected.truncate(limit.get()); }
```

`ReadOptions::new().backwards().limit(50)` on a 1 000 000-event store clones 1 000 000
`SequencedEvent`s (each of which is an `EventType` allocation plus a `Tags` allocation plus one
per tag) and then throws 999 950 away. `.take(limit.map_or(usize::MAX, NonZeroUsize::get))`
before `.collect()` is free and makes it O(limit). The store is explicitly "not built for
scale", but "the 50 most recent events" is the specification's own worked example and will be
the first thing anyone benchmarks.

---

## C14 — `append(&[Event])` forces every adapter to clone  ·  LOW  ·  performance

**Location:** `crates/happenstance/src/store.rs:141-145`, `crates/happenstance/src/memory.rs:219-221`

```rust
stored.extend(events.iter().enumerate().map(|(offset, event)| {
    SequencedEvent::new(position_at(first_index + offset), event.clone())   // <-- 220
}));
```

`&[Event]` means an adapter that *stores* the events (memory, and any future embedded store)
must clone each one. `Bytes` is cheap, but `EventType` is a `Box<str>` and `Tags` is a
`Box<[Tag]>` of `Box<str>` — so an event with three tags costs five allocations per append, per
event, on a path that already went to the trouble of providing `Event::into_parts`
(`event.rs:244`) "avoiding a clone in adapter write paths" that the signature then makes
unavoidable.

`events: impl IntoIterator<Item = Event>` would let both kinds of adapter be optimal (SQL
adapters bind and drop; owning adapters move), at the cost of making the events unavailable to
the caller after the call — which is the right trade for an append-only log. It does complicate
retry-on-`ConditionViolated`, so `Vec<Event>` (returned in the error on rejection) is the
alternative worth weighing. Either way, decide before publish: this is the signature everything
else is built on.

---

## C15 — assorted smaller items  ·  LOW / NIT

| # | location | item |
|---|---|---|
| a | `tag.rs:268-275` | `IntoIterator` exists for `&Tags` but not owned `Tags`; `for t in tags` fails with E0277. `From<Tags> for Vec<Tag>` is the only owned path. One five-line impl. |
| b | `projection.rs:46-48` | `ProjectionId::new` is infallible and validates nothing — empty strings, control characters and unbounded lengths are all accepted, unlike `EventType` and `Tag`. It becomes a primary key in a SQLite adapter. |
| c | `tag.rs:46-47`, `event.rs:36-37` | Docs say "ASCII control characters"; `char::is_control` is the Unicode `Cc` category, which also rejects U+0080–U+009F. Behaviour is fine (stricter is better); the doc is wrong. |
| d | `query.rs:143-150` | `Query: Default = All`. Harmless for reads; an `AppendCondition` built from `Query::default()` means "fail if the store holds *any* event". Consider not deriving `Default` on `Query`, so that choice is always explicit. |
| e | `store.rs:158-161` | `collect`'s `# Errors` documents dropping already-collected events on the first `Err`. Correct for a replay (a partial decision model is worse than none) and correctly documented — but the type could say it: `Result<Vec<T>, (Vec<T>, E)>` or a `collect_partial`. Judgment call; current choice is defensible. |
| f | `query.rs:113-116` | `QueryItem::matches` takes `&EventType`, so an adapter that filters in memory over a raw `String` column must allocate an `EventType` per row to call the reference matcher. A `matches_str(&self, &str, &Tags)` would remove that. |

---

# What I checked and found correct

These were the specific suspicions in the brief. Each was chased to a proof; none is a defect.
Recording them so nobody re-opens them.

**`Tags::contains_all` (`tag.rs:231-245`) is correct.** Exhaustively verified against a naive
`all(|n| hay.iter().any(..))` over all 256×256 subset pairs of an 8-element alphabet: **0
mismatches**. The two things that look wrong are both fine:

- `Ordering::Equal => return true` returns *after* `haystack.by_ref()` has consumed the matching
  candidate. Sound, because needles are strictly increasing (deduped), so the next needle is
  strictly greater than the consumed candidate and could never have matched it.
- `Ordering::Greater => return false` also consumes a candidate, but it short-circuits the whole
  `all`, so the consumed candidate can never be needed.

The invariant it rests on — both sides canonical — genuinely holds. `Tags`'s field is private
(`tag.rs:165`) and there are exactly two construction sites, `Tags::empty()` (`tag.rs:170`) and
`FromIterator` (`tag.rs:260-265`, `sort_unstable` + `dedup`). `from_pairs` routes through
`FromIterator`. `serde`'s `Deserialize` (`tag.rs:317-324`) re-canonicalises deliberately —
verified with hostile JSON `["z:9","a:1","a:1"]` → `{"a:1", "z:9"}`, len 2. `From<Tags> for
Vec<Tag>` is one-way; getting back requires `collect`. A duplicated needle is unconstructible.

**`memory.rs::read`'s backwards/from/limit composition is correct.** Brute-forced all 60
combinations of `from ∈ {None,1,3,5,6,99} × backwards ∈ {false,true} × limit ∈
{None,1,2,5,9}` against an independent reference model: **0 mismatches**. `matched.rev()` before
the `<= from` filter and `truncate` after is the right order — filtering after reversing is what
makes `truncate` take the *N highest* positions at or below `from`, which is what the
specification's worked example asks for.

**`AppendCondition::is_violated_by` (`append.rs:95-107`) matches its documentation exactly.**
`Some(after) if position <= after => false` — `after` exclusive, the boundary event itself never
violates. `_` covers both `after: None` and `position > after`, both of which fall through to
the query match. The three unit tests at `append.rs:161-191` pin all four cases. The let-chain
avoidance costs nothing in clarity.

**`collect` (`store.rs:162-184`) is sound.** `collected` is declared outside the `poll_fn`
closure, so the `FnMut` captures it by mutable reference and it survives across a `Pending`
re-poll — no events are lost when a stream yields part of a batch and suspends. Waker
registration is correctly delegated: `collect` returns `Poll::Pending` immediately after the
inner `poll_next` did, so whatever waker that stream registered is the live one. The temporary
`poll_fn` future is dropped at the end of the `.await?` statement, releasing the borrow before
`Ok(collected)`. No leak, no double-poll-after-`None` hazard (`vec::IntoIter` is fused).

**The two-flavour `Send` design works end to end**, including the free functions — this was the
thing most likely to be quietly broken and it is not:

- `read_decision_model(&MemoryEventStore, &q)` is `Send` (auto-trait leakage through the RPITIT
  and `trait_variant`'s blanket impl resolves correctly);
- it survives `tokio::spawn` on a multi-thread runtime;
- and — the real test — a **generic** `fn spawn_it<S: SendEventStore + Sync + Send + 'static>`
  that calls `read_decision_model` inside `tokio::spawn` compiles, meaning the Send-ness is
  provable for all `S`, not just concrete ones. Library authors building on happenstance can
  write generic spawnable code.
- The control case behaves as it should: the same call inside code bounded only on `EventStore`
  is correctly `!Send`.

**`trait_variant::make` handles the GAT.** `SendProjectionStore` is generated with
`type Batch<'a> where Self: 'a` intact, its futures are genuinely `Send`, and the blanket impl
gives `ProjectionStore` for free — verified by implementing `SendProjectionStore` and then
calling generic code bounded on `ProjectionStore`. (The problems in C4–C6 are in the port's
design, not in the macro.)

**Every `serde` deserialisation path is validating.** A hostile peer cannot smuggle in a
non-canonical value: `EventType` (`event.rs:315-320`) and `Tag` (`tag.rs:304-309`) route through
their checked constructors, `Tags` re-canonicalises, `QueryItem` goes through `QueryItem::new`
(so `UnconstrainedItem` is still rejected and `types` still gets sorted), and `Query` goes
through `from_items` (so `Query::Items([])` is unconstructible). Verified: `{"event_type":""}`
→ `Err("an event type must not be empty")`. `Event::deserialize` writing `tags` in directly
(`event.rs:363-368`) is safe *because* `Tags::deserialize` did the work — worth a one-line
comment saying so, since it reads like a bypass.

**`MemoryEventStore::append` is a valid oracle for concurrency.** One write lock spans the
condition check and the write, with no `.await` inside, so no reader can observe a partial batch
and no appender can slip between check and write. The `Snapshot` stream owns its data, borrows
nothing, and is correctly `Send`; the `read_stream_is_send` test (`memory.rs:328-340`) guards
the property the whole two-trait design exists for.

**`SequencePosition`'s `NonZeroU64`, `Query`-as-enum, `Tags`-canonical-at-construction, the
uninhabited `MemoryStoreError`, and lifting `ConditionViolated` out of the adapter error type**
are all the right calls and are well argued in place.

---

# Suggested order

1. **C1** (delete five attributes, add a round-trip test) — trivial, and the alternative is
   discovering it inside `happenstance-sync`.
2. **C7** (`checked_add`) — one line, makes a documented guarantee true.
3. **C2 + C3** (four conformance rules + a documented `from` semantics + a documented
   empty-batch precedence) — small, and it is the crate's central claim.
4. **C8** (decide what "lazy" means and pin it).
5. **C10, C15a** (ergonomics that the typed layer will hit immediately).
6. **C4/C5/C6** — one design pass on `ProjectionStore`, before publish, because the port ships
   as public API even while marked provisional.
7. **C14** — decide the `append` signature before anything is built on it.
8. **C11, C12, C13, C15b–f** — cleanup.
