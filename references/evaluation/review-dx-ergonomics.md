# happenstance — DX / ergonomics review

**Lens:** what it feels like to build a real application on this.
**Date:** 2026-08-05 · **Toolchain:** rustc 1.97.1, edition 2024
**Method:** read every source file, doctest and README line in the contract crate and the
worked example; then *wrote and compiled* a second, harder worked example plus eight targeted
compiler probes against the real crate.

Artefacts (all compiled, most run):

| Path | What it is |
|---|---|
| `…/scratchpad/eval/warehouse/src/main.rs` | the harder worked example — 5 event types, entity update, projection, retry loop, and a live demonstration of the divergence hazard. **Runs.** |
| `…/scratchpad/eval/probe/src/main.rs` | probes B/E/G/H/C/D/F — retry-loop shapes, `Send` leakage, `E0034` |
| `…/scratchpad/eval/probe/src/projection_probe.rs` | a hand-written `MemoryProjectionStore` + generic projection runner |
| `…/scratchpad/eval/probe/src/faulty.rs` | a fault-injecting decorator store (~45 lines) |
| `…/scratchpad/eval/probe/src/constprobe.rs` | proof that const-evaluated `EventType` validation works |

Full absolute prefix: `C:\Users\ryanm\AppData\Local\Temp\claude\D--repos-happenstance\559ba08d-08bc-4f98-b677-6b98e7a98078\scratchpad\eval\`

---

## Headline

The contract types are excellent and I would change very few of them. But **nothing above the
byte layer exists yet, and the two things that are missing are the two things that decide
whether an application is correct**: the query is stated twice with nothing checking the two
statements agree (I made the example oversell stock silently, with zero warnings), and the
retry loop — which every user must write — *cannot be written as a closure-taking helper at
all* on tokio. Both must be solved by a trait, not a closure, and I have compiled the trait
that solves them.

---

## 1. The `?`-tax

### The count, measured

Per handler in `examples/course-subscriptions/src/main.rs`:

```
6  define_course
8  subscribe
6  unsubscribe
6  main
```

The minimal DCB path — read a decision model, decide, append one event — is
`define_course`, lines 86–104. Six `?`, of which **five are on string literals the author
typed themselves**:

```rust
let query = Query::from_item(QueryItem::new(          // ?3
    [COURSE_DEFINED],
    Tags::from_pairs([("course", course)])?,          // ?1
)?)?;                                                 // ?2

let (existing, last_seen) = read_decision_model(store, &query).await?;   // ?4 (genuine I/O)

let event = Event::new(
    COURSE_DEFINED,
    format!("{{\"capacity\":{capacity}}}").into_bytes(),
)?                                                    // ?5
.with_tags(Tags::from_pairs([("course", course)])?);  // ?6
```

Note also that `Tags::from_pairs([("course", course)])` is constructed **twice** — once for the
query, once for the event — and `Tags` is `Clone`.

### The real cost is not the `?`, it is the four error families

`InvalidTag`, `InvalidEventType`, `InvalidQuery`, `AppendError<E>` do not unify. `InvalidQuery`
has `From<InvalidEventType>` (`error.rs:74`) but **no** `From<InvalidTag>`, even though
`QueryItem::new` takes a `Tags` that was built by a function returning `InvalidTag`.

The consequence is visible in the repository itself: the canonical worked example is written
in `anyhow`, and CLAUDE.md's house style says *"No `anyhow` in library crates."* **A user
writing command handlers in their own library crate cannot follow the only example shipped.**

I compiled what they must write instead (`probe/src/lib.rs`): a five-variant error enum, four
`#[from]`s, plus a hand-written adapter, because `read_decision_model` returns a bare
`S::Error` while `append` returns `AppendError<S::Error>` and the two `#[from]` impls conflict:

```rust
#[derive(Debug, thiserror::Error)]
pub enum CommandError<E: core::error::Error + 'static> {
    #[error(transparent)] Tag(#[from] InvalidTag),
    #[error(transparent)] EventType(#[from] InvalidEventType),
    #[error(transparent)] Query(#[from] InvalidQuery),
    #[error(transparent)] Append(#[from] AppendError<E>),
    #[error("{0}")] Rejected(String),
}

// Cannot be `#[from]`: it would conflict with `AppendError<E>` above.
fn store_err<E: core::error::Error + 'static>(e: E) -> CommandError<E> {
    CommandError::Append(AppendError::Store(e))
}
```

That is 15 lines of pure ceremony before the first command exists.

### Is the validation earning its cost?

Partly. Control characters in a tag are a genuine hazard for an adapter that indexes
`key:value` strings, and 255 bytes is a defensible column width. But the validation is being
paid at the wrong time. Almost every failure it can catch is a *typo in a literal*, which is
compile-time information being deferred to runtime.

### Recommendation — three changes, in order of value

**(a) `const fn EventType::from_static`.** `EventType(Box<str>)` cannot be built in const
context, because there is no const allocation — that is the entire reason `EventType::new` is a
runtime `Result` today. Change the representation to admit a static arm and the constructor
becomes const, and a violation becomes a *compile* error. I verified this works on this
toolchain (`probe/src/constprobe.rs`):

```rust
pub const fn from_static(value: &'static str) -> Self {
    assert!(!value.is_empty(), "an event type must not be empty");
    assert!(value.len() <= 255, "an event type must be at most 255 bytes");
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {                       // const loops: stable since 1.46
        assert!(bytes[i] >= 0x20 && bytes[i] != 0x7f, "…control characters");
        i += 1;
    }
    Self(value)
}
```

```
error[E0080]: evaluation panicked: an event type must not be empty
  --> src\constprobe.rs:42:34
   | pub const BAD: StaticEventType = StaticEventType::from_static("");
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

The Rust-specific reasoning: `Box<str>` is a heap pointer, and `const` evaluation cannot call
the allocator, so *any* heap-backed newtype forces a runtime constructor. Switching to
`Cow<'static, str>` (or a two-variant enum `Static(&'static str) | Owned(Box<str>)`) keeps the
`Box` path for genuinely dynamic types while making the 99% case — a literal — free, infallible
and allocation-free. The cost is `EventType` grows from 16 to 24 bytes; it is cloned once per
event and compared by `&str`, so this does not matter.

**(b) A `tags!` macro.** Tag *values* are runtime data (a SKU, a UUID) and cannot be const-checked.
But `tags!` can hide the fallibility behind one `?` instead of one per pair, and const-check the
*keys*:

```rust
// before                                       // after
Tags::from_pairs([("course", course),           tags!{ course: course, student: student }?
                  ("student", student)])?
```

**(c) One umbrella error.** Add `InvalidEvent` (or make `InvalidQuery` the umbrella) with `From`
impls from all three validation errors, so a handler has one error family rather than four. This
is what actually removes the `anyhow` dependency.

Together, `define_course` goes from six `?` to **two** — one for the read, one for the append —
which is the honest number, because those are the only two operations that can genuinely fail
at runtime.

---

## 2. The duplicated query — the single most important finding

### The hazard, demonstrated

In `subscribe` (`main.rs:114–173`) the query is built once (`:114`), read with (`:127`),
hand-folded over (`:134–156`), and then handed to `AppendCondition::new(query.clone())` inside
`commit` (`:212`). **The fold and the query must describe the same set of events or the
consistency boundary is silently wrong, and nothing — not the type system, not clippy, not the
conformance suite — checks that they do.**

I did not assert this. I built it. `warehouse/src/main.rs` adds a sixth event type,
`StockWrittenOff`, teaches the *fold* about it and forgets it in the *query*:

```rust
let query = Query::from_items([
    QueryItem::new(
        [SKU_CATALOGED, SKU_DISCONTINUED, STOCK_RECEIVED,
         STOCK_RESERVED, RESERVATION_RELEASED,
         // STOCK_WRITTEN_OFF,   <-- forgotten
        ], …)?,
    …
])?;

match sequenced.event_type().as_str() {
    …
    STOCK_WRITTEN_OFF => on_hand -= i64::from(payload_qty(&sequenced.event)),  // dead code
}
```

`cargo run` output:

```
== the query/fold divergence hazard ==
   OVERSOLD: reserved 7 of 0 real units, no error raised
```

Ten units received, ten written off, seven reserved, no error. The match arm is dead code the
compiler is happy about (it is a `&str` match, so there is no exhaustiveness signal), and the
append condition does not name `StockWrittenOff` either — so a *concurrent* write-off cannot
reject the append. Both halves of DCB fail from one omission.

This is not a hypothetical maintenance risk. It is the normal shape of the change "we added an
event type", which happens every sprint.

### What the typed layer must provide

The query must be **derived from the decision model, not restated beside it**. The minimum
viable shape is a trait whose implementer supplies the query *and* the fold, and a free function
that owns the loop:

```rust
pub trait Decision {
    type State;
    type Rejection;

    /// The consistency boundary. Used for BOTH the read and the append
    /// condition — they are now the same value, by construction.
    fn query(&self) -> Result<Query, InvalidQuery>;

    /// Fold what that query returned.
    fn fold(&self, state: &mut Self::State, event: &SequencedEvent);

    /// Decide.
    fn decide(&self, state: &Self::State) -> Result<Vec<Event>, Self::Rejection>;
}
```

This closes the *restatement* hole — one query, two uses — and it is a large improvement on
today. It does **not** close the *divergence* hole: `fold` can still match on a type the query
omits.

To close that too, the query has to be *accumulated by the fold's own declaration of what it
reads*. `disintegrate` solves this with a derive macro over the event enum: `#[derive(Event)]`
generates the `EventType`/tag mapping, and `#[derive(StateQuery)]` on the state struct with
`#[state_query(DomainEvent)]` generates the query from the same declaration the `mutate` method
matches on — so there is exactly one place that names the event set, and the query and the fold
are both generated from it. That is the correct target and it is why `disintegrate` is
macro-driven; the macro is not sugar, it is the mechanism that makes the two agree.

happenstance can get most of the way without a proc macro, using an enum-typed fold. If the
decision model folds over a *typed* `DomainEvent` enum rather than `&str`:

```rust
#[non_exhaustive]
enum StockEvent { Cataloged(..), Received(..), Reserved(..), Released(..), WrittenOff(..) }
```

then `DomainEvent::EVENT_TYPES` is a single associated const, `query()` is generated from it,
and adding a variant to the enum makes the `fold`'s `match` non-exhaustive — a **compile error**
at exactly the site of the hazard. That is the payoff of ADR-0006's typed layer and it is worth
the whole crate on its own.

**Concrete recommendation for phase 3, in priority order:**

1. `DomainEvent` with `const EVENT_TYPES: &'static [EventType]` and a `tags()` method.
2. `Decision` deriving its `Query` from the `DomainEvent` types it folds — never hand-written.
3. Composition: `(A, B)` implements `Decision` by OR-ing the two queries and folding both.
   This is the mechanism that makes the boundary *dynamic*, and it must be in 0.1 — it is what
   `subscribe`'s two-`QueryItem` query is doing by hand.
4. `happenstance-macros` for the derive. RUNBOOK phase 3 says *"No derive macro … out of scope
   for 0.1"*. **I disagree, and this is the one scope call I would change.** Without a derive,
   the user writes `EVENT_TYPES` by hand next to a `match`, and the divergence hazard returns in
   a new costume. The macro is not ergonomics; it is the correctness mechanism.

---

## 3. The retry loop — it cannot be written the obvious way

`commit` bails at `main.rs:216–220` with *"under real contention this is where a retry loop
would go"*. I wrote that loop. It does not compile.

### Attempt 1 — the natural signature

```rust
async fn with_retry<T, F>(max: u32, mut attempt: F) -> Result<T>
where F: AsyncFnMut() -> Result<T> { … }

tokio::spawn(async move {
    let order = format!("o{n}");
    with_retry(8, async || reserve(&store, &order, "gizmo", 1).await).await
});
```

```
error: implementation of `Send` is not general enough
   --> src\main.rs:388:20
    = note: `Send` would have to be implemented for the type
            `&'0 Arc<MemoryEventStore>`, for any lifetime `'0`...
    = note: ...but `Send` is actually implemented for the type
            `&'1 Arc<MemoryEventStore>`, for some specific lifetime `'1`
```

Two of these, and neither mentions `with_retry`, the closure, or anything the user can act on.
This is among the worst diagnostics in the language.

### Attempt 2 — pass the store as a closure argument (probe B)

Works. **Only because the closure captures nothing.**

### Attempt 3 — probe G: store as an argument, one owned local captured by reference

```rust
with_retry_b(&*s2, 4, async |st: &MemoryEventStore| {
    st.append(&[… Tags::from_pairs([("sku", sku.as_str())]) …], None).await
})
```

```
error: implementation of `Send` is not general enough
    = note: `Send` would have to be implemented for the type `&MemoryEventStore`
    = note: ...but `Send` is actually implemented for the type `&'0 MemoryEventStore`,
            for some specific lifetime `'0`
```

Every real command handler captures the entity id. **So a closure-based retry helper is
categorically unusable with `tokio::spawn`** — which is to say, unusable in a web handler.

The Rust reason: an `AsyncFnMut` closure's returned future is an opaque type parameterised by
the closure's borrow of its environment. `tokio::spawn` needs `Send` for *all* instantiations of
that borrow (higher-ranked), and the compiler can only prove it for the specific one — a known
limitation of `AsyncFn*` with borrowed captures.

### What actually works (probe H) — and it is what should ship

A trait, whose `decide` is a **plain synchronous method**. The lifetime is then bound in the
method signature rather than in a closure capture, so the future produced by `execute` is a
single `async fn` future and the compiler proves `Send` for all lifetimes:

```rust
async fn execute<S, C>(store: &S, command: &C, max: u32)
    -> Result<(), Outcome<C::Rejection, S::Error>>
where S: EventStore, C: Decide
{
    let query = command.query().map_err(Outcome::Query)?;
    let mut attempts = 0;
    loop {
        attempts += 1;
        // The retry RE-READS. This is the part users get wrong.
        let (history, last_seen) = read_decision_model(store, &query).await…;
        let events = command.decide(&history).map_err(Outcome::Rejected)?;
        if events.is_empty() { return Ok(()); }          // no-op commands are free
        let condition = AppendCondition::new(query.clone()).after_opt(last_seen);
        match store.append(&events, Some(&condition)).await {
            Ok(_) => return Ok(()),
            Err(e) if e.is_condition_violated() => {
                if attempts >= max { return Err(Outcome::Exhausted { attempts }); }
            }
            Err(e) => return Err(Outcome::Store(e)),
        }
    }
}
```

Eight tasks, each `tokio::spawn`ed, each retrying, four units of stock:

```
probe H: 4 won, 4 rejected (4 units in stock)
```

It compiles, it spawns, and it is correct under real contention.

### What the shipped version needs beyond the probe

- **Re-read on every attempt.** Non-negotiable and easy to get wrong; `execute` above puts the
  read *inside* the loop. A user who hoists it out has written an infinite loop.
- **A bounded budget** with a distinct `Exhausted { attempts }` outcome — this is the livelock
  signal, and it must be a different variant from `Rejected`, because one means "your invariant
  said no" and the other means "the system is thrashing". Metrics and alerting want them apart.
- **Jitter.** Full-jitter exponential backoff (`sleep(rand(0..base * 2^n))`). The version I wrote
  in `warehouse` (`sleep(attempts ms)`, no jitter, no cap) is what a user writes and it
  synchronises retriers under load.
- **A `RetryPolicy` type with a default**, per "convention over configuration": `RetryPolicy::default()`
  = 8 attempts, 1 ms base, full jitter, 100 ms cap. `execute(store, &cmd)` uses it; `execute_with`
  takes an override.
- **A sleep seam.** `tokio::time::sleep` is not available on `wasm32`/Workers, and the whole point
  of ADR-0001 is that target. The policy needs an injectable delay (a `Sleeper` trait, or
  `RetryPolicy::none()` plus a caller-driven loop) or the retry helper cannot live in a crate that
  claims wasm support. **This is a constraint the RUNBOOK does not currently mention.**

### Where it lives

**In the contract crate (`happenstance-core` after ADR-0006), beside `read_decision_model`.**
ADR-0006 already made exactly this argument to move the projection runner in: *"the discriminator
for what belongs in the typed layer is encoding, not orchestration."* `execute` decodes nothing —
`decide` hands back `Vec<Event>` of opaque bytes. By the ADR's own test it belongs in the
contract crate, and putting it there means the retry loop is available to someone who has not
adopted the typed layer. The typed `Decision`/`DomainEvent` layer then builds on it.

---

## 4. Reading the docs cold

**For someone who knows event sourcing but not DCB:** yes. `lib.rs:13–21` ("DCB in a paragraph")
is genuinely good — it names the pain (aggregates drawn before you know the decisions) and the
resolution in five sentences. The type table at `lib.rs:25–31` is the right second thing to read.

**For someone who knows neither:** no, and it should not try. The README correctly links the
specification. Fine.

**The E0034 question.** I reproduced it (probe F):

```
error[E0034]: multiple applicable items in scope
   |  let _ = store.read(&Query::all(), ReadOptions::new());
   |                ^^^^ multiple `read` found
   = note: candidate #1 is defined in an impl of the trait `SendEventStore` for the type `MemoryEventStore`
   = note: candidate #2 is defined in an impl of the trait `EventStore` for the type `TraitVariantBlanketType`
help: disambiguate the method for candidate #2
   + let _ = EventStore::read(&store, …);
```

Verdict: **it will be the #1 support question, but it is survivable.** rustc's `help` gives both
fixes verbatim, and `store.rs:31–45` documents it precisely. The leak is `TraitVariantBlanketType`
in candidate #2 — a name from a dependency's macro internals, appearing in a user's error, with no
explanation. Two cheap mitigations:

1. **Ship a `prelude` that exports `EventStore` and *not* `SendEventStore`.** `use happenstance::prelude::*;`
   then cannot produce E0034. This is the single highest-leverage doc fix in the crate.
2. Add the literal string `TraitVariantBlanketType` to `store.rs`'s error block so the module docs
   are findable by searching the error text. Today the documented error at `store.rs:36–41` shows a
   *different* rendering from what rustc actually prints, so grepping the real message finds nothing.

**Where the getting-started path breaks: the README quick start does not compile.** It is not
doctested — `lib.rs` has no `#![doc = include_str!("../README.md")]`, and `xtask/src/main.rs`'s
documentation step is `cargo doc`, which does not compile README fences. I pasted `README.md:87–100`
verbatim into a doctest:

```
error[E0728]: `await` is only allowed inside `async` functions and blocks
error[E0277]: the `?` operator can only be used in a function that returns `Result`…
error: aborting due to 4 previous errors
```

Four errors in the first ten lines a user copies. Same for the DCB snippet at `README.md:29–50`.
Fix: `#![doc = include_str!("../README.md")]` behind `#[cfg(doctest)]` — then CI compiles the
README on every commit and it can never rot. (It will need `# #[tokio::main] async fn main()`
hidden lines, which is standard.)

---

## 5. Error messages

I read all fifteen `#[error(...)]` strings. Most are good — they state the rule, not the
symptom, which is right. `InvalidQuery::NoItems` is the best in the crate because it names the
fix in the message: *"a query must contain at least one item; use `Query::all()` to match
everything"* (`error.rs:66`). More of them should do that.

Two are wrong or weak:

**`InvalidTag::Empty` — "a tag must not be empty" (`error.rs:21`).** `Tag::key_value` returns
this when *either* half is empty (`tag.rs:70–72`). A developer who passed a non-empty key and an
empty value reads "a tag must not be empty", looks at their non-empty key, and is stuck. Split it:
`EmptyKey` / `EmptyValue` / `Empty`, or at minimum `"a tag and both halves of a `key:value` pair
must be non-empty"`.

**`ConditionViolated` — "append condition violated: the store already contains a matching event"
(`error.rs:94`).** At 2am this says what happened and nothing about what to do, and it drops the
`conflicting_position` the struct went to the trouble of carrying. Make it actionable and use the
field:

```rust
#[error("append condition violated at position {conflicting_position:?}: an event matching \
         the append condition was written after the decision model was read — rebuild the \
         decision model and retry")]
```

**`AppendError::NoEvents`** is fine, and putting it in `AppendError` rather than making `append`
take a non-empty type is a reasonable call — a `NonEmpty<[Event]>` in the port signature would be
worse ergonomics than one runtime variant.

**On `#[non_exhaustive]` for `AppendError`.** The doc at `error.rs:146–147` warns the wildcard is
forced. Keep it. The primary flow should not be a `match` at all — `is_condition_violated()`
already exists at `error.rs:170` and is exactly right:

```rust
if let Err(e) = store.append(&events, Some(&cond)).await {
    if e.is_condition_violated() { /* retry */ } else { return Err(e); }
}
```

The problem is that the doctest at `error.rs:133–143` teaches the `match` form, which is the one
that needs the wildcard. **Lead the example with `is_condition_violated()` and demote the match.**
Then `#[non_exhaustive]` costs nothing, and the freedom to add variants (a `Conflict` carrying the
full conflicting event, say) is worth keeping while nothing is published.

One thing that *does* work and should be kept: I verified the retry signal survives erasure into
`anyhow::Error` via `downcast_ref::<AppendError<S::Error>>()`. Fragile — it requires naming the
concrete store error — but it works, and `is_condition_violated` on the concrete type is the
better path.

---

## 6. Testing an application

`MemoryEventStore` is a good start and better than most libraries ship: `with_events` for
arranging (`memory.rs:86`), `snapshot` for asserting (`memory.rs:114`), dense positions, and
an uninhabited error type that proves the read path need not be fallible. But writing the
warehouse tests, four things were missing.

**(a) A fault-injecting decorator. Cheap, and the highest-value thing on this list.** There is
today *no way* to make an append fail, so the retry path — the hardest code in any DCB
application — is untestable without real concurrency and a prayer. I wrote one in ~45 lines
(`probe/src/faulty.rs`):

```rust
impl SendEventStore for FlakyStore {
    type Error = MemoryStoreError;
    fn read(&self, query: &Query, options: ReadOptions)
        -> impl Stream<Item = Result<SequencedEvent, Self::Error>> + Send {
        SendEventStore::read(&self.inner, query, options)
    }
    async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
        -> Result<SequencePosition, AppendError<Self::Error>> {
        self.append_calls.fetch_add(1, Ordering::SeqCst);
        if self.remaining_failures.fetch_update(…).is_ok() {
            return Err(AppendError::ConditionViolated(ConditionViolated::unspecified()));
        }
        SendEventStore::append(&self.inner, events, condition).await
    }
}
```

```
flaky: result=true, append calls=4      // 3 injected failures, succeeded on the 4th
```

Ship this in `happenstance-testkit` as `FaultyStore` with a policy: fail the first *n* appends,
fail every *n*th, fail with a store error rather than a violation, fail reads. It is a day's work
and it makes every user's retry logic testable. **Note it also validates the port design** — a
decorator was writable at all because `read`'s `impl Stream` return could be forwarded directly.

**(b) Given/when/then helpers.** `testkit::fixtures` (`fixtures.rs`) is public and good, but it
serves *adapter* authors. Application authors want the mirror image:

```rust
let store = MemoryEventStore::with_events(vec![…]);   // given — exists
subscribe(&store, "c1", "s1").await?;                  // when
assert_appended!(store, [("StudentSubscribed", [("course","c1"),("student","s1")])]);  // then — missing
```

Today the "then" is `store.snapshot()` and hand-comparing `SequencedEvent`s, whose `PartialEq`
includes the exact payload bytes — so a JSON field-order change breaks every test. An
`assert_events_matching(&store, query, expected_types_and_tags)` helper would fix that.

**(c) A clock.** No event carries a timestamp and no seam exists to inject one. Most real
domains need `occurred_at`; today it goes in `metadata` as bytes with no convention, so every
application invents its own and none of them interoperate. This is a phase-3 decision (it pairs
with event identity, already flagged as a release hazard) and it deserves a ledger row it does
not currently have.

**(d) Deterministic positions.** `MemoryEventStore` is already dense-from-1 and documented as such
(`memory.rs:31–32`), with the warning that nothing may depend on it. That is the right call — and
the counterpart is missing: a **`GappyMemoryStore`** that deliberately leaves gaps, so an
application's tests catch a handler that assumed `position + 1`. CLAUDE.md already forbids the
conformance suite from asserting literal positions for this reason; give application authors the
same protection.

---

## 7. Convention over configuration

### Where the user is made to decide something the library could decide

| Decision forced on the user | What the default should be |
|---|---|
| The entire retry policy | `RetryPolicy::default()` — 8 attempts, full-jitter backoff, 100 ms cap (§3) |
| Payload encoding | phase 3's `Codec`; default JSON, because it is debuggable in `sqlite3` |
| Tag key conventions | nothing suggests `entity:id`. A `Tags::entity("course", "c1")` helper plus one paragraph of guidance would make tags across two codebases comparable |
| Checkpoint→resume arithmetic | see below — this one is a correctness footgun |
| `?` on every literal | §1 |

**The checkpoint footgun.** `ReadOptions::from` is *inclusive* (`query.rs:228`), and
`ProjectionStore::checkpoint` returns the last position **applied** (`projection.rs:86–88`). So
resuming a projection requires advancing past it, and the doc says exactly that — *"Feed this to
`ReadOptions::from` — after advancing past it"*. In both my projection implementations I wrote:

```rust
let options = match from.and_then(SequencePosition::next) {
    Some(next) => ReadOptions::new().from(next),
    None       => ReadOptions::new(),
};
```

Six lines of ceremony at every resume point, and a developer who writes the obvious
`ReadOptions::new().from(checkpoint)` **re-applies the last event on every restart** — silent
double-counting in any non-idempotent projection. Add `ReadOptions::after(SequencePosition)`
(exclusive) and the whole hazard disappears into one call. `AppendCondition::after` is already
exclusive (`append.rs:72`), so the vocabulary is established and the asymmetry is the bug.

**Missing re-export.** `lib.rs:104–106` re-exports `bytes` with the comment *"so adapters and
callers can name payload types without adding a direct dependency on a specific `bytes` version"*.
That reasoning applies with equal force to `futures_core`, which every adapter must name in
`read`'s return type — and it is **not** re-exported. Writing `faulty.rs` I had to add
`futures-core = "0.3"` to `Cargo.toml` and keep it version-locked to happenstance's by hand.
One-line fix: `pub use futures_core;`.

**Missing accessor.** `Tags` has `contains`, `contains_all`, `iter`, `as_slice` — but no way to
read a value by key. So the canonical example does this scan twice
(`course-subscriptions/src/main.rs:136–138`), and I wrote it four more times in `warehouse`:

```rust
let is_this_student = tags.iter()
    .any(|tag| tag.key() == Some("student") && tag.value() == Some(student));
```

Add `Tags::value_of(&self, key: &str) -> Option<&str>`. Because `Tags` is sorted and a
`key:value` tag sorts by key first, this is a `partition_point` — `O(log n)`, not the `O(n)` scan
users write.

### Where the library decides something it should have let the user choose

Genuinely little, which is the right side to err on. Two:

- **`read_decision_model` takes no `ReadOptions`** (`store.rs:198`) and always
  `collect`s the full matched history into a `Vec`. `collect`'s own doc says *"Do not use it to
  replay an entire log"* (`store.rs:152`) — and `read_decision_model` does exactly that when the
  query is broad. `unsubscribe` (`main.rs:183–187`) reads a student's entire course history to
  look at `.last()`; `ReadOptions::new().backwards().limit(1)` would read one row and still yield
  the correct `last_seen`. Add `read_decision_model_with(store, query, options)`.
- **`MAX_TAG_LEN`/`MAX_EVENT_TYPE_LEN` are hard-coded at 255** and not adapter-negotiable. This is
  fine — a portable contract needs a fixed floor, and an adapter that can store more loses nothing.
  No change.

---

## What is right, and should not be relitigated

- **`SequencePosition(NonZeroU64)`.** The niche optimisation is real and the doctest at
  `event.rs:112–116` proves it in-repo. `Option<SequencePosition>` appears in every `ReadOptions`
  and every `AppendCondition`; making it free is exactly the right use of a newtype.
- **`Query` as an enum, not `Vec<QueryItem>`.** An empty query is unrepresentable. Correct.
- **`Tags` canonical at construction**, with `contains_all` as an `O(n+m)` merge-scan
  (`tag.rs:231–245`) and a regression test for the interleaving bug at `tag.rs:384`. This is
  careful work.
- **`AppendError` separating `ConditionViolated` from adapter errors.** The single best decision
  in the API. I verified the signal survives even erasure into `anyhow` via downcast.
- **`read` returning the stream at the top level and not being `async`** (ADR-0001 constraint 3).
  I verified the consequence holds: a helper bound on the weak `EventStore` still produces a
  `Send` future when instantiated at a `SendEventStore` type (probe C — `assert_send` passes on
  both flavours). **Constraint 4 is sound** and generic code really can bind the weaker trait
  without losing spawnability. This is the load-bearing property of the whole two-trait design and
  it works.
- **Opaque `Bytes` payloads** (ADR-0003) — and the `Debug` impl printing `<n bytes>`
  (`event.rs:249–271`) rather than garbage is a small kindness that will pay off in every test
  failure.
- **`MemoryEventStore` as the conformance oracle**, with the atomicity comment at
  `memory.rs:189–194` explaining *why* the whole append runs under one write lock.
- **`testkit::fixtures` being public** so adapter authors write extra tests in the suite's own
  vocabulary.
- **All 16 doctests compile and run.** The house rule "prefer a runnable example to a described
  one" is being honoured everywhere except the README.

---

## Appendix — friction log, in order encountered

| # | What I did | What happened |
|---|---|---|
| 1 | Wrote `with_retry` with `AsyncFnMut()`, spawned it | `error: implementation of Send is not general enough` ×2, naming neither the helper nor the closure |
| 2 | Passed the store as a closure argument (probe B) | compiles — only because the closure captured nothing |
| 3 | Added one captured `String` (probe G) | same opaque `Send` error |
| 4 | Wrote a trait with a sync `decide` (probe H) | compiles, spawns, 4 won / 4 rejected under real contention |
| 5 | Implemented `SendProjectionStore` with `Batch<'_>` in the signature | `E0195: lifetime parameters or bounds on method 'commit' do not match the trait declaration` — no hint that the fix is spelling it `Self::Batch<'_>` |
| 6 | Spelled it `Self::Batch<'_>` | compiles. This needs a line in `projection.rs`'s docs |
| 7 | Called `projections.checkpoint(&id)` with both projection flavours imported | E0034, same as the store traits |
| 8 | Wrote a *generic* projection runner | `P::Batch<'_>` has no bounds and the trait has no `apply`, so the runner can `begin`/`commit`/`rollback` but cannot write anything — the apply step must be a higher-ranked closure argument. **ADR-0006 moves the projection runner into the contract crate; it cannot be written against this port as it stands.** |
| 9 | `commit(batch, id, position)` takes `SequencePosition`, not `Option` | a run that matched zero events has no position to commit; I had to early-return. Fine, but undocumented |
| 10 | Looked for a `MemoryProjectionStore` | none exists. An application with a projection has nothing to build or test against |
| 11 | Named `Stream` in the decorator's `read` | had to add a direct `futures-core` dependency; `bytes` is re-exported and `futures_core` is not |
| 12 | Tried `Event::new(pre_built_event_type, data)` | **E0271 — does not compile.** See below |
| 13 | Pasted the README quick start into a doctest | 4 compile errors |

### The one outright defect

`Event::new` cannot accept an `EventType`:

```rust
pub fn new(
    event_type: impl TryInto<EventType, Error = InvalidEventType>,   // event.rs:198
    data: impl Into<Bytes>,
) -> Result<Self, InvalidEventType>
```

```
error[E0271]: type mismatch resolving `<EventType as TryInto<EventType>>::Error == InvalidEventType`
   |     Event::new(ty, &b"{}"[..]).unwrap()
   |     ---------- ^^ expected `InvalidEventType`, found `Infallible`
```

`EventType: TryInto<EventType>` comes from the blanket `impl<T, U: From<T>> TryFrom<T> for U`,
whose `Error` is `Infallible`, not `InvalidEventType`. So the newtype cannot be reused —
which defeats its purpose and blocks the obvious `?`-tax workaround of validating once into a
`static` and constructing events from it.

`QueryItem::new` gets this right (`query.rs:57–61`) with the weaker bound, and
`impl From<Infallible> for InvalidQuery` at `error.rs:77–84` exists for exactly this reason. The
fix is to copy it:

```rust
impl From<core::convert::Infallible> for InvalidEventType {
    fn from(never: core::convert::Infallible) -> Self { match never {} }
}

pub fn new<T>(event_type: T, data: impl Into<Bytes>) -> Result<Self, InvalidEventType>
where T: TryInto<EventType>, InvalidEventType: From<T::Error> { … }
```

Six lines, and the precedent is already in the codebase.
