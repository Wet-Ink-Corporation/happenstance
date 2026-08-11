# happenstance — architectural evaluation

**Date:** 2026-08-06 · **Commit:** `9fd2337` · **Scope:** the whole workspace, against the goal of a
published, trusted 0.1.

Ten reviewers, four adversarial verifiers, a roadmap planner and a completeness critic produced 126
verified findings (8 were refuted and are excluded). After deduplication they collapse to about 55
distinct issues. This document is the synthesis; where reviewers disagreed I have read the code and
adjudicated, and I say so in place.

---

## 1. Verdict

The foundation is sound and, in the parts that are finished, better than most published Rust
libraries — the type design (`SequencePosition(NonZeroU64)`, `Query`-as-enum, canonical `Tags`,
`AppendError` lifting `ConditionViolated` out of the adapter's error) is careful, argued in place,
and conforms to every MUST in the DCB specification. The problem is not what is built; it is the
**order** in which the rest is scheduled. The RUNBOOK writes two adapters (phases 1–2) before the
typed layer that would discover contract defects (phase 3) and before the `!Send` proof that would
validate the port shape at all (phase 5) — so roughly forty of the fifty-six `blocksFirstPublish`
findings exist because the contract is being frozen by accretion rather than by decision. The single
highest-leverage change is therefore a sequencing change: **insert an explicit contract-freeze pass
before the first real adapter, and make its first item the trait-derivation decision** — because
`#[trait_variant::make(SendEventStore: Send)]` at `store.rs:91` silently forbids `EventStore` from
ever gaining a defaulted method, which means `head()` and `count()` can never be added, and it
cannot differentiate `SendEventStore::Error: Send + Sync` from `EventStore::Error`, which means no
generic code can `?` a store error into `anyhow`. Everything else in section 3 is downstream of that
one line. Nothing is published, so every fix below is free today and permanent after publication —
and the conformance suite, which is the project's differentiator, is measurably weaker than its own
README claims (four of six deliberately-broken adapters passed all 27 rules), so it must be
strengthened before it is used as the bar for anything.

---

## 2. What is right, and should not be touched

Defend these under future pressure. Each is a decision someone will later propose "simplifying".

| Decision | Location | Why it is right |
|---|---|---|
| `SequencePosition(NonZeroU64)` | `event.rs:119` | The niche is load-bearing, not decorative: `Option<SequencePosition>` sits in every `AppendCondition` and every `ReadOptions` and costs 8 bytes, not 16. The doctest at `event.rs:112-116` proves it in-repo. The official DCB reference uses `after: 0` as its absent-sentinel and tests it with a falsy check — this encoding is strictly better. |
| `Query` as an enum, not `Vec<QueryItem>` | `query.rs:143-150` | Spec clause Q1 permits "at least one item" **or** "match everything" and nothing else; the empty query is genuinely unrepresentable (modulo the `Items` variant leak — see §4). `umadb-dcb` uses `Option<Vec<..>>` and cannot make that claim. |
| Tags as opaque strings with `key:value` as advisory convention | `tag.rs:16-38` | Spec TG1 is explicit that key/value shape "is irrelevant to the Event Store", and the spec's own example query uses bare tags. Enforcing `key:value` in the type would make happenstance **non-conformant**. Do not "improve" this. |
| `Tags` canonical (sorted + deduped) at construction | `tag.rs:258-266` | Exceeds spec T1's SHOULD by making duplicates unrepresentable, makes `contains_all` an O(n+m) merge scan (`tag.rs:231-245`, verified exhaustively over all 256×256 subset pairs of an 8-element alphabet, 0 mismatches), and makes the tag blob a stable index key for adapters. The regression test at `tag.rs:384` shows the interleaving bug was already found and fixed. |
| `AppendError<E>` generic, with `ConditionViolated` lifted out | `error.rs:122-166` | Every store fails differently and every store fails *identically* on a violated condition. Keeping `E` generic rather than boxing avoids an allocation on a hot path (`SQLITE_BUSY` under contention is not rare) and avoids forcing a `Send + Sync` bound the wasm flavour may not want. `is_condition_violated()` and `map_store()` are the right two helpers and no more. |
| `read` returns `impl Stream` at the top level and is **not** `async` | `store.rs:117-121` | ADR-0001's reasoning is correct: `trait_variant` attaches `+ Send` only to the outermost item of the return type. Nesting the stream in a future silently drops it. This is constraint 3 and it is right. |
| `AppendCondition { fail_if_events_match, after }` | `append.rs:52-61` | Matches the spec field-for-field and name-for-name. `after` exclusive vs `ReadOptions::from` inclusive is a deliberate, documented, conformance-tested asymmetry — one means "I have already seen up to here", the other "start here". |
| Opaque `Bytes` payloads (ADR-0003) | `event.rs:185` | Buys byte-for-byte replication forwarding, keeps adapters free of domain knowledge — and, unrecognised by the ADR, makes crypto-shredding work unmodified, because no adapter ever parses a payload. |
| The single trait + `trait_variant` derivation *as an idea* | `store.rs:91` | `umadb-dcb` hand-maintains `DcbEventStoreAsync`/`DcbEventStoreSync`/`DcbReadResponseAsync`/`Sync` with nothing keeping them in sync — the exact anti-pattern. `cqrs-es` still recommends `async_trait` and has no wasm path. This design is ahead of the market. The *mechanism* needs an amendment (§3.1); the idea does not. |
| The conformance suite as a **published** artefact, validated against the oracle | `testkit/src/lib.rs`, `tests/memory_conformance.rs` | "A claim about behaviour is worth exactly as much as the test that checks it." Running the suite against `MemoryEventStore` validates the suite and the oracle simultaneously. This is the project's best idea. It needs strengthening, not replacing. |
| "Never assert on literal position values" | `CLAUDE.md`, `suite.rs:189-196` | Spec SP3 permits gaps. Comparing against positions the store actually assigned is the single most important design rule in the suite, and it is honoured everywhere it matters. |
| `#[non_exhaustive]` + `pub` fields on `SequencedEvent`, `ReadOptions`, `AppendCondition` | `event.rs:276`, `query.rs:225`, `append.rs:51` | Blocks external struct-literal construction while keeping fields readable. Correct — but see §3.3 for what it does *not* protect. |
| Uninhabited `MemoryStoreError` | `memory.rs:143-145` | Proves at the type level that the contract does not *require* a fallible read path. Most crates would have written `struct MemoryStoreError;` and lost the proof. |
| `forbid(unsafe_code)` workspace-wide, verified opted-into by all eight members | `Cargo.toml:42` | The strongest trust signal the project has. It appears nowhere in the README — fix that (§5.7). |
| `default-features = false` on the *internal* workspace deps | `Cargo.toml:20-25` | Cargo forbids a member from removing a workspace default, so the permissive choice would silently pull `memory` and `std` into every adapter. Subtle and correct. |
| ADR-0002 kept verbatim under supersession | `adr/0005:53-60` | "Rewriting `eventum` to `happenstance` inside it would invert the claim into a falsehood." Rare and correct ADR hygiene. |
| `memory` in default features | `Cargo.toml` | Measured, not asserted: two release builds of an external crate that never mentions `MemoryEventStore`, with and without the feature, came out byte-identical at 124,416 bytes. Fully dead-code-eliminated. Keep it. |

---

## 3. The decisions that must be made before anything else is built

Ranked by blast radius. Each has a verdict, not a menu.

### 3.1 How the two trait flavours are produced — **highest blast radius in the workspace**

**Today:** `store.rs:91` — `#[trait_variant::make(SendEventStore: Send)]`, and `store.rs:99` —
`type Error: core::error::Error + 'static;`.

**First: is return-type notation a way out?** No, and not on any horizon that matters. Tracking
issue `rust-lang/rust#109417` is still `S-tracking-impl-incomplete`; the stabilisation PR
`rust-lang/rust#138424` (which would have stabilised `where T: Trait<method(..): Send>`) was
**closed unmerged on 2025-12-27**, for a non-technical reason — the author left the project — after
the lang team had already resolved the substantive blocker in favour of shipping. There is no
successor PR. RTN appears in the April 2026 project-goals update only as an unstabilised blocker for
Rust-for-Linux. On a 1.85 MSRV it is unreachable twice over. **Do not design around it.** The
two-trait design is not a workaround for a feature that is about to land; it is the answer.

**But the derivation mechanism has two limits that are semver-locked the moment you publish:**

1. **`EventStore` can never gain a method with a default body.** `trait_variant` does not rewrite
   default `async fn` bodies (E0728), so a default must be hand-desugared to
   `fn f(&self) -> impl Future<Output = T> { async move { … } }`. The desugared body captures
   `&self` across an await, so the `Send` variant's future is `Send` only if `Self: Sync` — which
   `SendEventStore: Send` does not require. Adding `Sync` later is semver-visible.
2. **`trait_variant` copies associated-type bounds verbatim**, so `SendEventStore::Error` cannot be
   `Send + Sync` while `EventStore::Error` is not.

**Consequence of leaving both:** `head()` and `count()` can never be added — not as provided
methods, and adding them as *required* methods breaks every adapter in existence. And no generic
helper over `SendEventStore` can convert a store error into `anyhow::Error` or
`Box<dyn Error + Send + Sync>` without a viral `where S::Error: Send + Sync` at every call site.
That is the entire extensibility story of the port, and it is currently zero.

**Options.**

| | Trade |
|---|---|
| (a) Keep as-is | The port is frozen at two methods forever and generic code cannot box its errors. Free today, unfixable after publish. |
| (b) `make(SendEventStore: Send + Sync)` + `Error: Error + Send + Sync + 'static` on both | One line each. Unlocks provided methods permanently. Costs the wasm adapter the ability to carry a raw `JsValue` in its error type. |
| (c) Hand-write the two traits | Full control: differentiated `Error` bounds, natural default bodies. Doubles the surface and re-creates the `umadb-dcb` drift risk that ADR-0001 exists to avoid. |

**Verdict: (b).** The `Sync` addition is nearly free in practice — an adversarial verifier proved
that any impl written in the documented style (`async fn append(&self, …)`, which is what
`MemoryEventStore` uses at `memory.rs:184`) *already* requires `Self: Sync`, because the returned
future captures `&Self` and `&T: Send` holds only when `T: Sync`. A `!Sync` store can implement
`SendEventStore` today only by hand-desugaring to avoid capturing `&self`, which is exotic. So
adding `Sync` breaks approximately zero real adapters, and after publication it breaks all of them.

The `Send + Sync` on `Error` is the genuinely debatable half. The cost is that a Cloudflare adapter
must stringify a `JsValue` at the boundary rather than carrying it — which is exactly what
`worker::Error::JsError(String)` already does. An error that cannot cross a thread is a nuisance for
every consumer and buys the wasm adapter nothing it cannot get from `.to_string()`. Take (b), and
write the falsification test into ADR-0007: *if the Cloudflare skeleton demonstrates that
stringifying loses information the caller needs, supersede with (c).* That is what the skeleton
phase (§7, phase 2) exists to answer, and it is cheap to answer before any adapter is finished.

```rust
// before — store.rs:91, 99
#[trait_variant::make(SendEventStore: Send)]
pub trait EventStore {
    type Error: core::error::Error + 'static;

// after
#[trait_variant::make(SendEventStore: Send + Sync)]
pub trait EventStore {
    type Error: core::error::Error + Send + Sync + 'static;
```

Add a `CONTRIBUTING.md` note: default bodies on these traits must be written as
`fn f(&self) -> impl Future<Output = T> { async move { … } }`, never `async fn`, because
`trait_variant` cannot rewrite the latter.

**Also fix the test that guards the design.** `memory.rs:328-340` `read_stream_is_send` asserts the
property on a *concrete* type, so it passes by auto-trait leakage even if the generic composition
were broken. The property the design actually lives on is that `trait_variant`'s blanket impl
`impl<T: SendEventStore> EventStore for T` forwards `read` such that the RPITIT's `Send`-ness leaks
back through. Verified: `fn f<S: EventStore + Send + Sync + 'static>(s: Arc<S>)` that spawns
`read_decision_model` fails with **E0277**; the same bounded on `SendEventStore` **compiles** and
still calls the `EventStore`-bound free function. Write that as a generic test:

```rust
fn spawns_from_generic_code<S: SendEventStore + Send + Sync + 'static>(store: Arc<S>) {
    tokio::spawn(async move { let _ = read_decision_model(store.as_ref(), &Query::all()).await; });
}
```

`trait_variant` 0.1.3 shipped 2026-07-22 after a two-year gap; the expansion is not frozen. If it
changes, CI stays green today and every native user's `tokio::spawn` breaks.

### 3.2 The `append` signature

**Today:** `store.rs:141-145` — `async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)`.

Two things are wrong at once, and they should be fixed in one change.

**(i) The borrow.** An owning adapter must clone. `memory.rs:219-221` does exactly that.
`Event::clone()` is not cheap despite `Bytes`: `EventType(Box<str>)` is one allocation and
`Tags(Box<[Tag]>)` is n+1 because each `Tag` is its own `Box<str>` — five allocations per three-tag
event, per append. And `Event::into_parts` (`event.rs:243-246`), whose doc comment says it exists
"avoiding a clone in adapter write paths", is **unreachable from any implementation of the trait**.
That doc comment is false as written, which is worse than the allocation, because it tells adapter
authors a technique that does not exist.

*One reviewer argued the slice is correct and should stay* — on the grounds that a SQLite adapter
binds parameters straight out of the borrow and clones nothing, and that a retry loop wants to
resubmit the same batch. **Both halves are right and neither survives.** The serialising case only
shows the borrow is *free for one adapter class*, not that it is right; every owning or embedded
store pays. And the retry argument is wrong on DCB grounds: after a `ConditionViolated` you must
re-read and re-decide, so the events are re-derived anyway. The suite's own
`racing_conditional_appends_elect_one_winner` (`suite.rs:609-620`) re-appends the identical event
with `core::slice::from_ref`, but that is a test artefact demonstrating a race, not a pattern.

**(ii) The empty batch.** `AppendError::NoEvents` (`error.rs:155-161`) has a doc comment that says
"This is a caller bug, not a store failure" — an error variant admitting it should not be an error.
It exists only because the slice makes the empty batch representable. Worse, its **precedence
against the condition check is unspecified and the two obvious implementations disagree**:
`memory.rs:197-216` evaluates the condition *first*, so `append(&[], Some(&c))` returns
`ConditionViolated` when `c` matches and `NoEvents` when it does not — the error depends on store
state for a call that is a caller bug regardless. A SQL adapter that early-returns on
`events.is_empty()` before `BEGIN IMMEDIATE` (the obvious implementation) returns `NoEvents`
unconditionally. Both pass the suite, because `append_rejects_empty_batch` (`suite.rs:406-414`) only
ever calls `append(&[], None)`.

**Options:** document the precedence and add a rule; or take `Vec<Event>`; or take
`impl IntoIterator<Item = Event>`; or introduce a non-empty owned batch type.

**Verdict: introduce `EventBatch`, and delete `AppendError::NoEvents`.**

```rust
// before
async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
    -> Result<SequencePosition, AppendError<Self::Error>>;

// after
pub struct EventBatch(Vec<Event>);            // non-empty by construction
impl EventBatch {
    pub fn of(event: Event) -> Self;           // infallible — the common case
}
impl From<Event> for EventBatch { … }
impl TryFrom<Vec<Event>> for EventBatch { type Error = EmptyBatch; … }
impl IntoIterator for EventBatch { type Item = Event; … }

async fn append(&self, events: EventBatch, condition: Option<&AppendCondition>)
    -> Result<SequencePosition, AppendError<Self::Error>>;
```

Reasoning, in order: it *dissolves* the precedence question rather than answering it with a
documented ordering plus a conformance rule; it gives adapters owned events so `into_parts` becomes
true; it restores the crate's own headline principle (`lib.rs:35`, "illegal states are
unrepresentable") at the one place it was abandoned; and it deletes both an error variant and a
conformance rule. `AppendError` then has two variants, both genuine runtime outcomes, which is what
`error.rs:122-129` says it is for.

Explicitly rejected: `impl IntoIterator<Item = Event>` — the most flexible option, but it makes
`append` a *generic method*, and `dynosaur` (which ADR-0001 names as the escape hatch for the
missing `dyn EventStore`) cannot erase generic methods. Taking `impl IntoIterator` closes the only
door ADR-0001 left open. If `EventBatch` is judged too much ceremony, take `Vec<Event>` — still
owned, still fixes the clone, still keeps erasure open, and matches the only other Rust DCB library
(`umadb-dcb`'s `DcbEventStoreAsync::append(events: Vec<DcbEvent>)`). Then you must document the
precedence and add the rule.

Cost at the call site: `store.append(event.into(), None)`. That is the whole downside.

### 3.3 `Event`'s missing identity and timestamp

**Today:** `SequencedEvent { position, event }` (`event.rs:277-282`) carries nothing store-assigned
beyond the position. The RUNBOOK ledger row at `RUNBOOK.md:67` already records identity as
**decided** — a Lamport pair `(origin, origin_position)` as `EventId` on `SequencedEvent`,
store-assigned — but schedules it for phase 3, *after* phase 1 writes the SQLite schema. There is no
`recorded_at` anywhere in the workspace and no ledger row for one.

**Two things are wrong with the current plan.**

First, **the RUNBOOK names the wrong mitigation.** `RUNBOOK.md:390-412` identifies event identity as
a hazard to `Event`'s public API and cites `#[non_exhaustive]` as the protection. But
`#[non_exhaustive]` already makes field *addition* non-breaking — that is not the exposure. It also
makes `SequencedEvent::new` (`event.rs:286`) the **only** way any downstream crate can construct
one, because a non-exhaustive struct cannot be built with struct-literal syntax outside its crate.
So the attribute's entire effect is to funnel every adapter through a fixed-arity constructor, and
that constructor is exactly what a new field breaks. The RUNBOOK reaches the right conclusion
("answer identity before publish") for a reason that does not hold, which means the *shape* of the
change is unconstrained by the thing that actually costs money.

Second, **the schema lands two phases before the decision.** `recorded_at` is the one field that
cannot be backfilled. Every production event store records one (EventStoreDB, Marten, Axon, Python
`eventsourcing`) because operators need it for retention, debugging, audit and lag; and
`happenstance-sync`'s merge rule for two independently-ordered logs will want a physical clock
alongside the Lamport pair. The DCB spec reserves the slot: *"It MAY contain further fields, like
metadata defined by the Event Store."*

**Verdict:** settle identity **and** `recorded_at` in one ADR, before any schema is written, and
settle the **constructor shape** in the same ADR.

```rust
#[non_exhaustive]
pub struct SequencedEvent {
    pub position: SequencePosition,
    pub event: Event,
    /// When the store durably recorded this. Store-assigned; never client-supplied.
    pub recorded_at: Option<Timestamp>,   // plain u64 ms since epoch — no chrono/jiff, no_std-safe
    pub id: Option<EventId>,
}

// keep `new` at two arguments and grow by builder, not by arity:
SequencedEvent::new(position, event).with_id(id).with_recorded_at(t)
```

Use a plain `u64` of milliseconds, not `SystemTime` and not a date-time crate: the contract crate is
`no_std`-capable (`lib.rs:74`) and must stay dependency-thin, and the store is the only thing that
ever sets it. Also make `Event::into_parts` return a struct rather than a 4-tuple before 0.1, so its
arity stops being public API for the same reason.

The overstated version of this finding — "events written by a phase-1 store can never acquire an
honest `recorded_at`" — is not true, because nothing is published and no production data exists. The
real cost is ordering: the schema is cheap to change and the *decision* is expensive to defer past
the point where three adapters have baked an assumption about it.

### 3.4 The read-side operations the trait lacks

**Today:** `EventStore` has exactly two methods. `store.rs:162` and `store.rs:198` show the crate's
current answer to "where do conveniences go" — free functions at the crate root.

Measured (external SQLite benchmark, 2.74M events): `exists`, `head`/`last_position` and
"latest matching event" all already compose from `ReadOptions` at optimal cost — 0.004–0.009 ms — so
the premise that they need trait methods is wrong for three of the four. Only **`count(query)`** is
genuinely absent: composing it means draining the whole stream and materialising every payload blob
to produce an integer (4.66 ms pushed down vs 106.2 ms drained). **`head()`** is a weaker but real
case: composed from `read(All, backwards().limit(1))` it fetches the last event's payload blob to
learn a `u64`, and the projection runner that ADR-0006 moves *into* this crate needs it constantly
to answer "am I caught up?".

**Verdict:** ship both as **provided, overridable** methods, at the same time as the `Sync` change in
§3.1 — they are strictly blocked on it.

```rust
/// The highest position the store has assigned, or `None` if empty.
/// Default drains a backwards limit-1 read; override to push down (`SELECT max(position)`).
fn head(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>> {
    async move { /* … */ }
}
/// How many events match. Default drains `read`; override to push down (`SELECT count(*)`).
fn count(&self, query: &Query) -> impl Future<Output = Result<u64, Self::Error>> {
    async move { /* … */ }
}
```

`head()` also closes a spec-blessed affordance that is currently unreachable: DCB clause AC4
explicitly notes that `after` may be higher than the last matching position, which lets a uniqueness
command condition on a *head-derived* position instead of on a whole-log check. No port method
returns a head today, so that optimisation cannot be expressed.

Do **not** add `exists`, `read_backwards_first`, a batch/multi-query read (`Query::Items` already
*is* the multi-query form) or a subscription API — all measured as already optimal or premature.
Separately, ship a `pub trait EventStoreExt: EventStore` with a blanket impl for everything
derivable from `read`/`append` (`read_to_vec`, `fold_decision_model`) — this is the
`Iterator`/`Itertools`, `Future`/`FutureExt` pattern, it is non-breaking to grow forever, and it is
where `collect` and `read_decision_model` should live. Capabilities an adapter should *push down*
must **not** go there: a blanket impl cannot be overridden.

### 3.5 The projection port's GAT

**Today:** `projection.rs:80-82` — `type Batch<'a> where Self: 'a`, justified at `projection.rs:77-79`
as "Borrows from `Self` because a transaction cannot outlive the connection that opened it."

Two reviewers disagreed on how badly this fails. Adjudication: the **adapter-implementability**
reviewer is right and the **correctness** reviewer overstated. `type Batch<'a> = rusqlite::Transaction<'a>`
*does* compile on the bare `ProjectionStore` flavour if the store owns the `Connection` and `begin`
uses `Connection::unchecked_transaction(&self)`. It fails only on `SendProjectionStore`, for two
independent reasons: `Connection` is `Send` but not `Sync`, so `&Self` is not `Send`; and
`Transaction<'_>` is not `Send`, so the `commit`/`rollback` futures cannot be. `sqlx` does not rescue
it — `Pool::begin()` yields `Transaction<'static, DB>`, which does not borrow the store either.

So the GAT's stated justification is **unearned on the flavour every native adapter will use**, and
`RUNBOOK.md:66`'s "survives with amendments" is not refuted so much as silent about *which* flavour
survives.

There is a third defect the GAT does not even buy protection against. In
`async fn commit(&self, batch: Self::Batch<'_>, …)` at `projection.rs:109-114`, the elided lifetime
on `batch` is a **fresh method-level lifetime, unrelated to `&self`'s**. Nothing ties the batch to
the receiver, so store A's live transaction can be committed through store B's connection — B writes
the checkpoint on its own connection while A's read-model writes sit uncommitted. That breaks the
one invariant the module exists to defend (`projection.rs:19`: "the two writes must be **one**
transaction") at runtime with no compiler complaint.

**Verdict: drop the lifetime.**

```rust
// before
type Batch<'a> where Self: 'a;
async fn begin(&self) -> Result<Self::Batch<'_>, Self::Error>;
async fn commit(&self, batch: Self::Batch<'_>, id: &ProjectionId, position: SequencePosition) -> …;

// after
type Batch;                                   // owned. `Send` on the Send flavour.
async fn begin(&self) -> Result<Self::Batch, Self::Error>;
async fn commit(&self, batch: Self::Batch, id: &ProjectionId, position: SequencePosition) -> …;
```

The only Send-capable rusqlite shape is `type Batch = Vec<PendingWrite>` — buffer the read-model
writes, open the transaction inside `commit` alongside the checkpoint write. That **preserves the
one-transaction invariant exactly**, which is the whole point, and it makes the foreign-batch hazard
unrepresentable because the adapter never hands out a live transaction. A working
`SendProjectionStore` over `Mutex<Option<Connection>>` was compiled to prove it.

While reshaping, add the **Drop contract** the port lacks: "dropping a `Batch` without `commit`/
`rollback` MUST roll back AND MUST release any resource `begin` acquired." In a probe, a dropped
batch permanently lost the connection and the store returned `Busy` forever. Also ship a
`MemoryProjectionStore` behind the `memory` feature — the port currently has no oracle, no doctest,
no in-tree implementation, and implementing it fails with a bare E0195 unless you happen to spell the
parameter `Self::Batch<'_>`.

Finally: **ship the projection module behind an off-by-default `unstable-projection` feature at 0.1**
(the `tokio_unstable` / `tracing` idiom), with a doc paragraph saying it is exempt from semver until
its conformance suite lands. That decouples the release from the port freeze, which is currently a
hard dependency the release does not need to carry.

### 3.6 The ADR-0006 rename's timing

**Today:** ADR-0006 is `accepted` (dated today) and unexecuted; `RUNBOOK.md:223` schedules the rename
as phase-3 work.

**Verdict: execute it this week, in its own commit, with nothing else in it, and ship the new
`happenstance` as a five-line facade on day one.** Three reasons:

1. **An accepted-but-unexecuted rename is the worst state the repo can be in.** CLAUDE.md's binding
   constraint 2 — "Never put `serde` in `happenstance`'s default features" — does not become stale
   after the rename, it **inverts**: the new `happenstance` is precisely the crate whose job is
   encoding and which ADR-0006 plans to depend on serde. Today it is accurate; the day the rename
   lands it is an actively harmful instruction. (CLAUDE.md does route readers to the RUNBOOK ledger
   as the source of truth at lines 113-115, which mitigates but does not fix it.)
2. **`xtask/src/main.rs:66-77` hard-codes `-p happenstance` for the wasm32 step**, and
   `main.rs:121` selects that step with `&REQUIRED[3..4]` — by array index. After the rename, an
   un-repointed step would silently start checking the **typed** layer on wasm32, which compiles and
   proves nothing, leaving binding constraint 1 unguarded behind a green gate.
   `.github/workflows/ci.yml:77`'s `package: happenstance, happenstance-testkit` needs the same
   treatment. This is the one place a mechanical rename can go wrong without failing.
3. **Cost rises monotonically.** Deferring means phases 1-2 author a SQLite ADR, a projection
   conformance suite, a port-freeze ADR and the projection runner all against names known to be
   wrong, then phase 3 lands one commit mixing a 22-file mechanical rename with the design of the
   typed layer — the worst possible commit to review or bisect.

The only argument for deferring is that the bare name would point at an empty crate. Five lines
answer it, and it is what ADR-0006 says the crate is anyway:

```rust
//! The happenstance event sourcing library. Re-exports the contract from
//! `happenstance-core`; the typed layer lands in phase 6.
pub use happenstance_core::*;
```

`README.md:87-100`'s `cargo add happenstance` quick start then stays true throughout, and the typed
layer becomes purely additive. In the same commit, add an "On the historical record" section to
ADR-0006 stating that ADR-0001/0003/0004 and CLAUDE.md's constraints are rewritten to
`happenstance-core` because they were always statements about the ports crate, while ADR-0005's body
stays verbatim because rewriting it would invert its claim.

**And reserve the crates.io names first.** Both `happenstance` and `happenstance-core` were queried
during this review and are **unregistered**. Two accepted ADRs (`0005:87-91`, `0006:126-128`) name
squatting as the one risk that could reopen them, and it is tracked in no phase and no ledger row.
Registration is first-come. Publish `0.0.0` placeholders — `description`, `license`, `repository`, a
one-paragraph README — before anything else this week. Yanking later preserves the reservation. Note
a `0.0.0` baseline is useless to `cargo-semver-checks` (0.0.x versions are mutually incompatible in
Cargo), so it does not substitute for the `--baseline-rev` fix in §5.6.

### 3.7 Two smaller decisions that are also now-or-never

**Bound the query.** `Query::from_items` (`query.rs:164-170`) bounds nothing, and the append
condition is evaluated **while holding the write lock** (`memory.rs:195-209` takes the write lock at
:195 and evaluates `is_violated_by` against every stored event at :197-209 while still holding it).
Measured on a 2,000-event log with a 20,000-item condition built entirely through public
constructors: a trivial append on an idle store takes 58.7 µs; the hostile conditional append takes
123.4 ms; a **concurrent trivial append** takes 123.5 ms — a ~2,100× stall of every other writer from
one legal call. This is not a reference-store artefact: `RUNBOOK.md:64` has already decided the
SQLite strategy is `BEGIN IMMEDIATE` + a probe, which puts an unbounded query inside the exclusive
write transaction identically. And `Query::deserialize` (`query.rs:314-321`) validates semantics but
bounds nothing, so a peer-supplied query goes straight into a receiving instance's write path —
which is exactly what `happenstance-sync` will do. Add `MAX_QUERY_ITEMS` beside `MAX_TAG_LEN` and
enforce it in `Query::from_items`; the constant is public API and its absence is what freezes. Add
`MAX_EVENT_DATA_LEN` (16 MiB) and `MAX_TAGS` (64) at the same time — the contract currently bounds
the two fields no engine struggles with (255-byte identifiers) and leaves unbounded the three that
cost real money. Verified accepted today without complaint: a 67 MB payload, 100,000 tags on one
event, a 50,000-item query. SQLite's `SQLITE_MAX_VARIABLE_NUMBER` is 32,766, so a many-tag event
fails at write time *after* the caller has already made a decision.

**Decide whether `EventType` can be const-constructed.** `EventType(Box<str>)` (`event.rs:29`) and
`Tag(Box<str>)` (`tag.rs:39`) cannot be built in a const context — there is no const heap allocation
— which is why `EventType::new` is a runtime `Result`. That is the root of the six-to-eight `?` per
handler in `examples/course-subscriptions/src/main.rs:86-104` and :114-173. The alternative
(`Cow<'static, str>` plus `pub const fn from_static` validating with `assert!`) was compiled on the
pinned toolchain: `const COURSE_DEFINED: EventType = EventType::from_static("CourseDefined");`
works, `from_static("")` is a **compile error**, and `Cow`'s `PartialEq`/`Ord`/`Hash` all delegate so
`Borrowed("A") == Owned("A")` and both hash identically — every existing derive stays correct. Cost:
`size_of::<EventType>()` goes 16 → 24 bytes, irrelevant inside a heap-indirected `Event`. The trade
being made is explicit and should be stated rather than assumed either way: the current design
guarantees an `EventType` is *always* validated; `from_static` moves that guarantee to compile time
for literals and keeps it at runtime for everything else. **Verdict: do it** — the guarantee is
preserved, not weakened, and this is the largest single ergonomic tax in the library. Note it wins
much less for `Tag`, whose *value* half is usually runtime data; say so rather than overselling it.

---

## 4. Defects

Confirmed correctness bugs. Each has a file:line, a failing input, and a fix.

### D1 — `serde` wire types do not round-trip in postcard or bincode (critical)

**Where:** `event.rs:342-345`, `query.rs:281-284`, `append.rs:121-122` — five
`skip_serializing_if` attributes.

**Failing input:** `Event::new("A", data)` with no tags (what every doctest builds);
`QueryItem::of_types([…])`; `QueryItem::tagged(…)`; `AppendCondition::new(q)` with no `after`.

**Why:** `skip_serializing_if` shortens the field count passed to `serialize_struct`. A
non-self-describing format feeds the deserializer exactly `FIELDS.len()` values positionally, so a
skipped field desyncs the stream. Serialisation *succeeds* and produces plausible-looking bytes.
`AppendCondition` **with** `after` set is also broken, because the failure propagates from any nested
`QueryItem` that omits either field — the only shapes that survive are those where every field of
every nested wire struct is populated.

**Why it matters:** the `serde` feature exists solely for `happenstance-sync`, which will pick
postcard or bincode for its Worker-side wire, and there is **not one serde round-trip test anywhere
in the workspace** (~253 lines of hand-written wire format, zero assertions).

**Fix:** delete all five `skip_serializing_if`; keep `#[serde(default)]` so existing JSON still
parses. Cost ≈ 10 bytes of JSON per event. Add `crates/happenstance-core/tests/wire.rs` under
`#[cfg(feature = "serde")]` with `round_trip<T>(v: &T)` asserting equality through **both**
`serde_json` and `postcard`, exercising the sparse shapes specifically.

### D2 — `Query::Items` is publicly constructible with zero items

**Where:** `query.rs:143-150`. `Query` is a public enum, not `#[non_exhaustive]`, and `Items` carries
a public `Box<[QueryItem]>`.

**Failing input:** `Query::Items(Vec::new().into_boxed_slice())` from any downstream crate — the
exact state `Query::from_items` rejects with `InvalidQuery::NoItems`.

**Consequence:** `Query::matches` returns `false` for everything, so an `AppendCondition` built from
it **can never be violated** — a conditional append that is silently unconditional. The crate's
headline design claim is false at the boundary that matters.

**Exploitability, honestly:** the deserialise path is safe (`query.rs:318` routes through
`from_items`), and `Query::default()` is `All`, so nothing in the crate or its wire format produces
this state. It takes a caller deliberately naming the variant.

**Fix:**

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Query {
    All,
    #[non_exhaustive] Items(Box<[QueryItem]>),
}
```

A `#[non_exhaustive]` tuple variant cannot be constructed *or* tuple-destructured outside the
defining crate — exactly the property wanted. `Query::items() -> Option<&[QueryItem]>` (`query.rs:180-186`)
and `is_all()` already serve every legitimate adapter need. Audit for tuple patterns first. While
there, **delete the `Default` derive**: `Query::default()` is `Query::All`, nothing in the workspace
uses it, and any downstream struct that embeds a `Query` and derives `Default` silently gets a
full-log scan. (Inside an `AppendCondition` that fails *closed* — maximally strict, a loud spurious
rejection — so this is a nit, not a trap; but it is free to remove.)

### D3 — `SequencePosition::next()` returns `Some(MAX)` where its doc promises `None`

**Where:** `event.rs:138-144` — `Self::new(self.0.get().saturating_add(1))`.

**Failing input:** `SequencePosition::new(u64::MAX).unwrap().next()` returns `Some(u64::MAX)`, not
`None`. `saturating_add` cannot signal overflow, so the method whose sole purpose is signalling
overflow never does.

**Blast radius:** nil in practice — it needs 2^64 allocated positions. Fix it because a contract
crate returning a wrong answer where the signature has a way to say "I can't" is the wrong trade.

**Fix:** `NonZeroU64::checked_add` is `const` and returns `Option<NonZeroU64>` directly:

```rust
pub const fn next(self) -> Option<Self> {
    match self.0.checked_add(1) { Some(v) => Some(Self(v)), None => None }
}
```

(`?` in `const fn` is not stable on the MSRV, hence the `match`.) Related, same file:
`memory.rs:131-136` `position_at` uses `u64::try_from(index).unwrap_or(u64::MAX - 1)`, whose
fallback arm is unreachable on every platform Rust supports — dead code that reads like a considered
decision, and which would make the *oracle* assign a duplicate position rather than fail loudly if
it ever fired. Replace both fallbacks with a documented `expect` or a `debug_assert!`.

### D4 — `Event::new` cannot accept an `EventType` you already hold

**Where:** `event.rs:197-200` — `event_type: impl TryInto<EventType, Error = InvalidEventType>`.

**Failing input:** `Event::new(my_event_type, data)` where `my_event_type: EventType` — rejected,
because the blanket `impl<T, U: From<T>> TryFrom<T> for U` gives `Error = Infallible`, and the
equality constraint excludes it.

**Why it is a defect and not a preference:** the crate already knows the fix and applied it two files
over. `QueryItem::new` (`query.rs:57-61`) writes `T: TryInto<EventType>, InvalidQuery: From<T::Error>`
with `impl From<Infallible> for InvalidQuery` at `error.rs:77-84`, and its own doc comment at
`query.rs:55-56` says this "is what lets this accept both `&str` (fallible conversion) and an
already-built `EventType` (infallible)". `Event::new` is inconsistent with its sibling.

**Consequence:** a typed layer that interns one `EventType` per `DomainEvent` — exactly what phase 6
will do — must round-trip through `&str` and re-run validation on every event construction, then
handle a `Result` that cannot fail.

**Fix:** six lines.

```rust
impl From<core::convert::Infallible> for InvalidEventType {
    fn from(never: core::convert::Infallible) -> Self { match never {} }
}

pub fn new<T>(event_type: T, data: impl Into<Bytes>) -> Result<Self, InvalidEventType>
where T: TryInto<EventType>, InvalidEventType: From<T::Error> { … }
```

Consider also an infallible `Event::of(event_type: EventType, data: impl Into<Bytes>) -> Self` so the
typed layer's hot path returns no `Result` at all.

### D5 — `ReadOptions::limit(0)` silently means "unlimited"

**Where:** `query.rs:260-266` — `self.limit = NonZeroUsize::new(limit)`, with the doc "A `limit` of
zero is ignored, since requesting nothing is never what the caller meant."

**Failing input:** a paging loop writing `.limit(budget - fetched)` that reaches parity does not read
zero events — it reads **the entire log, unbounded, silently**. The premise is true for a literal and
false for a computed value. C-VALIDATE says functions validate their arguments; silently
reinterpreting one as its opposite is the anti-pattern the guideline exists for.

**Fix:** `limit: Option<usize>`, `0` meaning "return nothing" — matching SQL `LIMIT 0` and every
paging API in existence — plus one conformance rule. Cost is 8 bytes on a `Copy` struct passed by
value. Rejected `limit(NonZeroUsize)`: type-safe but forces
`.limit(NonZeroUsize::new(50).expect("nonzero"))` at every call site for a value that is almost
always a literal, and unlike `SequencePosition` — where `Option<SequencePosition>` sits in every
`AppendCondition` and the niche genuinely pays — the niche buys nothing here.

### D6 — `Some(Query::All)` round-trips to `None`; `Query::All` serialises to JSON `null`

**Where:** `query.rs:304-321` — `Serialize` maps `All` to `serialize_none` and `Items` to
`serialize_some`.

**Failing input:** `serde_json::from_str::<AppendCondition>("{}")` returns **`Ok`** with
`fail_if_events_match: Query::All` — a missing field *and* an explicit `null` both yield the
match-everything condition, because `Query`'s `Deserialize` routes through `Option<Vec<QueryItem>>`
and serde's `missing_field` succeeds for `Option`-shaped types. Separately,
`Option<Query>` collapses: `Some(Query::All)` and `None` are indistinguishable in JSON.

**Consequence:** the value a field takes when a peer emits `null` by accident is the broadest
condition in the protocol. Inside an `AppendCondition` that fails closed (a spurious rejection), so
it is not a data-loss path — but `happenstance-sync` will plausibly carry `Option<Query>` on a
subscription request, and there it silently loses the distinction.

**Fix:** an explicitly tagged representation — `{"all": null}` / `{"items": [...]}`, or a `"*"`
sentinel — so "match everything" cannot be reached by accident and `Query` is self-representing to a
non-Rust peer. Costs a few bytes and buys a protocol that fails closed. In the same pass: `Bytes`
currently renders as a JSON integer array (a 1 KiB payload becomes ~4 KiB and unreadable logs) —
either document that the `serde` feature targets binary formats, or branch on
`serializer.is_human_readable()` and emit base64.

### D7 — the reference store contradicts the contract's headline promise about `read`

**Where:** `store.rs:103-110` promises the stream "is **lazy**: nothing is executed until it is first
polled … what lets an adapter stream a million-event replay without buffering it".
`memory.rs:150-182` takes the read guard at call time, filters, clones and collects into a `Vec`
before returning.

Three reviewers reported this; they disagreed on the fix, and one added the mitigating fact that
`memory.rs:25-27` **does** disclose its own behaviour ("read snapshots the matching events under the
lock and streams from that snapshot"). So `MemoryEventStore` is not lying about itself — `store.rs`
is over-promising on behalf of every adapter, and the reference implementation everyone copies
demonstrates the opposite of the contract.

**Adjudication and fix.** Reviewers split three ways: snapshot-at-call, snapshot-at-first-poll, or
leave isolation unspecified. The third is right, for a reason none of them stated cleanly: **on an
append-only log with monotonic positions, isolation cannot cause skips or duplicates.** Forward
chunking by `position > last_seen` only ever sees new events at the tail; backward chunking by
`position < last_seen` only ever misses new events it has already passed. The *only* thing isolation
decides is whether concurrently-appended events become visible — and DCB is safe under either,
because the append condition re-checks. So specify the properties that matter and free the rest:

1. **`read` must not block, perform I/O, or acquire a contended lock.** It constructs a stream; all
   work happens in `poll_next`. This is load-bearing, not stylistic: `tokio::task::spawn_blocking`
   **panics** if called outside a runtime, so an adapter that starts blocking work at `read` time
   does not merely stall the reactor, it panics for any caller who builds a stream outside a runtime
   context.
2. **A stream must never yield the same position twice and must never yield out of requested order.**
3. **Whether events appended after `read` was called become visible is unspecified.** Say why — the
   append condition is what makes it safe — and say it on `read_decision_model` too: "`last` is a
   lower bound on what the caller observed; the append condition is what makes that safe."
4. Replace "is lazy" with "adapters SHOULD stream and MUST NOT buffer the whole result when a
   `limit` is set".

Then **move `MemoryEventStore`'s snapshot into the first `poll_next`** (~15 lines), so the oracle
demonstrates rule 1 rather than contradicting it. Add one rule: append between `read` and the first
poll, assert no duplicates and no reordering — not presence or absence.

### D8 — `append`'s empty-batch/condition precedence is unspecified and the oracle picks the surprising order

Covered as the second half of §3.2. `memory.rs:197-216` checks the condition before the emptiness
check; `error.rs:155-161` calls the empty batch "a caller bug, not a store failure"; the suite only
tests `append(&[], None)`. Two conformant adapters return different errors for the same call, and
`NoEvents` is also missing from `append`'s `# Errors` section (`store.rs:135-140`), which violates
the house rule. The `EventBatch` fix dissolves it; the cheap fix is to move the emptiness check to
the top of `MemoryEventStore::append` (before the lock is even taken — it is a pure precondition on
the argument), document the ordering, and add
`append_rejects_empty_batch_even_with_a_matching_condition`.

### D9 — `char::is_control` is Unicode Cc, not ASCII; and `Tag` equality is byte equality with no position taken

**Where:** `event.rs:46`, `tag.rs:56` use `char::is_control`; `event.rs:37`, `tag.rs:46-47`,
`error.rs:29` and `error.rs:53` all say "ASCII control characters". Four sites, wrong wording — the
check also rejects U+0080–U+009F. The *behaviour* is the better one; only the docs are wrong.
Two-word fix.

The half with teeth is unstated and is a positive design decision, not a bug: `Cf` format characters
are permitted, so U+200B (zero-width space) and U+202E (RTL override) can produce two visually
identical tags that are two different consistency boundaries — and U+202E in an event type is a
log-spoofing vector. Worse, **no position is taken on Unicode normalisation**: `"café"` in NFC
(U+00E9) and NFD (U+0065 U+0301) are different byte sequences, therefore different `Tag`s, therefore
different boundaries, indistinguishable on screen. A macOS client and a Linux client writing the
"same" tag would silently fail to conflict.

**Fix:** reject `Cf` as well (one extra check, no dependency), and add one sentence to `Tag`'s docs:
"Tags are compared as byte sequences. No Unicode normalisation is performed; callers accepting tag
values from user input must normalise before constructing a `Tag`." Explicitly reject pulling in
`unicode-normalization`: it is heavy for a `no_std`+`alloc` contract crate, and normalising means the
stored tag is no longer byte-identical to what the caller passed, which breaks ADR-0003's
byte-for-byte forwarding promise. **Doing nothing and saying nothing is the only option that is
definitely wrong.**

### D10 — the README's Quick start does not compile, and nothing checks it

**Where:** `README.md:87-100` (and the fragment at :29-50). Both blocks use `.await` outside an
`async` context and `?` outside a `Result`-returning function. `lib.rs` does not `include_str!` the
README, and `xtask`'s documentation step is `cargo doc`, which does not compile README fences.

The first ten lines a user copies fail to build, in a repository whose house style is "doctests are
documentation that cannot rot". **Fix:** `#![cfg_attr(doctest, doc = include_str!("../README.md"))]`
in `lib.rs`, plus the standard hidden `# #[tokio::main(flavor = "current_thread")] # async fn main()
-> Result<(), Box<dyn core::error::Error>> {` / `# Ok(()) # }` wrapper. Re-fence the deliberate
fragment at :29-50 as ```` ```rust,ignore ````. This also satisfies the phase-7 per-crate README task
with the same file.

### D11 — the published `.crate` would contain no licence text and no README

**Where:** `crates/happenstance/Cargo.toml:12-14`. `cargo package` only includes files under the
package root; `LICENSE-MIT` and `LICENSE-APACHE` are at the workspace root.

The artifact would declare `license = "MIT OR Apache-2.0"` and ship neither licence text, and render
an empty crates.io page. Apache-2.0 §4(a) requires giving recipients a copy of the License.
`cargo publish --dry-run` does **not** warn about this, so nothing in the current gate catches it.

**Fix:** copy both licence files into each publishable crate directory (Cargo will not follow paths
outside the package root), write per-crate `README.md`s, add `readme = "README.md"`, and add
`cargo package -p <each> --list` to the CI gate so the artifact contents are asserted rather than
assumed.

### D12 — the `serde` feature compiles only because `bytes` happens to enable `serde/alloc`

**Where:** `crates/happenstance/Cargo.toml:31`. The impls call `String::deserialize` and
`Vec::<Tag>::deserialize`, which live behind serde's `alloc` feature, but the manifest requests
serde with `default-features = false, features = ["derive"]` and never asks for `alloc` — it arrives
transitively from `bytes`.

The `--no-default-features --features serde` build (what a `no_std` replication peer uses) is
standing on a feature the crate does not declare. **Fix, one word:**
`serde = ["dep:serde", "bytes/serde", "serde/alloc"]`.

### D13 — `cargo doc --no-default-features` fails with three hard errors and the gate never runs it

**Where:** `lib.rs:62`, `lib.rs:68`, `store.rs:76` — unconditional intra-doc links to
`MemoryEventStore`, which only exists under the `memory` feature. With
`rustdoc::broken_intra_doc_links = "deny"` in the workspace lints, documenting the `no_std`
configuration is a hard error.

The canary is worse than the bug: the feature-powerset step (`xtask/src/main.rs:97-107`) runs
`cargo hack **check**`, not `cargo hack doc`, so feature-conditional doc breakage is invisible to
both doc steps *and* the powerset step. **Fix:** wrap the three links in
`#[cfg_attr(feature = "memory", doc = …)]` with an ungated fallback, and add
`cargo doc -p happenstance-core --no-default-features --no-deps` as its own gate step.

---

## 5. Findings by dimension

CONFIRMED unless marked **[PLAUSIBLE]**. "Blocks" = should be fixed before first publish.

### 5.1 The conformance suite — the differentiator, and measurably weak

This is the most important section after §3. The README leads with "a *published* conformance suite,
so that 'storage agnostic' is a claim anyone can check" (`README.md:163-166`). A reviewer built six
deliberately non-conformant event stores with one realistic adapter bug each and ran the full suite
against every one: **four of the six passed 27/27 undetected**, including two bugs the project's own
intended SQLite schema actively invites. (One store's construction could not be reproduced by a
verifier; two of six were caught, and a correct control passed cleanly — so the suite is not inert,
it is measurably weaker than "the bar every adapter must clear" implies.)

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| C1 | No stateful model-based property test | `testkit/tests/properties.rs` — all 8 properties test pure matching logic; none touches an `EventStore` | The combinatorial interaction of query × from × backwards × limit × condition × position-assignment is tested only by 27 hand-picked examples. Three of five behavioural bugs a prototype model caught are invisible to all 27 rules; shrunk counterexamples were 1–3 steps, found in under a second. | `event_store_model_conformance!(factory)`, exported so adapters inherit it. Four design points: generate **symbolic** position anchors (`None\|First\|Middle\|Head\|BeyondHead`) resolved at execution against positions the store assigned — the generalisation of the no-literal-positions rule, and what lets one test run against dense and gapped stores; build the model on `Query::matches` and `AppendCondition::is_violated_by` (~30 lines, not circular — those are already property-tested against naive oracles); **predict** the conditional-append outcome before calling and assert in both directions; cross-check `append`'s return against the head. Hand-roll `Vec<Op>` with `prop::collection::vec`; do not add `proptest-state-machine`. | M | **yes** |
| C2 | `limit` is only ever tested against `Query::all()` | `suite.rs:292-302` — `read_limit_truncates` uses `Query::all()` | An adapter that applies `LIMIT` in SQL **before** the tag filter passes all 27 rules. This is the classic SQLite/Postgres event-store bug and phase 1 ships exactly that adapter. Highest-value missing rule by a distance. | `read_limit_applies_after_filtering`: append interleaved [tagged A, untagged B, tagged A, untagged B, tagged A], read tag-constrained with `limit(2)`, assert exactly the first two *matching* events. Plus the `backwards` mirror. | T | **yes** |
| C3 | No genuinely concurrent rule, and the stated reason is wrong | `suite.rs:589-595` | `racing_conditional_appends_elect_one_winner` awaits each append to completion, so it cannot distinguish an atomic condition-check-and-write from a probe followed by a separate insert — the exact bug `RUNBOOK.md:153-156` claims it catches ("the one a naive read-then-write implementation fails"). A SQLite adapter with `SELECT EXISTS(…)` then `INSERT` outside `BEGIN IMMEDIATE` is conformance-green and loses writes under load. | The suite's own reasoning is half right: a parallel rule needs `S: SendEventStore + Send + Sync + 'static` and `Arc<S>`, which `EventStore` deliberately does not carry — so it is a **second, opt-in macro**, not a 28th rule. `event_store_concurrency_conformance!(factory)` over `#[tokio::test(flavor = "multi_thread", worker_threads = 8)]`, looped. Assert: exactly K of N contenders commit for capacity K; positions unique and totally ordered after N concurrent unconditional appends; `append`'s return is the caller's own last event not the global head (catches `SELECT max(position)` after commit — only observable in parallel); a concurrent reader never sees a partial batch; every task terminates within a timeout. Then correct `RUNBOOK.md:153-156` and `README.md:146`. | M | **yes** |
| C4 | The factory shape forbids any durability rule | `testkit/src/lib.rs:17-21` — the macro takes an expression re-evaluated per test producing a fresh, empty store | There is no way to obtain a second handle to the same store, so position reuse across a reopen is unobservable. A SQLite adapter using `SELECT COUNT(*) + 1` or `MAX(rowid) + 1` — both natural, both wrong after a delete — is conformant by the suite's verdict, and `happenstance-sqlite/src/event_store.rs:30-32` says the `AUTOINCREMENT` rationale is load-bearing precisely here. Reused positions silently re-point every stored `AppendCondition::after` and every projection checkpoint. | Not another rule — a second **factory shape**. `event_store_conformance!(fresh = …, reopen = …)` or a `Reopenable` fixture trait. Rules: events survive a reopen; positions after a reopen strictly exceed every position before it; append/reopen/append reuses nothing. `MemoryEventStore` omits the argument. This also unlocks any future rule needing two handles (multi-connection concurrency, WAL reader/writer isolation). | S | no |
| C5 | Three condition-query *shapes* are never used as a condition | `fixtures.rs:64-89` — every condition builder uses `Query::from_item`; `suite.rs:390,447,465,483,501,519,537,561,604` | `Query::all()`, multi-item, and **tag-only** (empty `types`) are the three shapes most likely to break generated SQL. Match-all degenerates to a missing WHERE; an empty types list can generate `IN ()`, a syntax error in SQLite; a multi-item OR conjoined with `position > after` is a textbook precedence bug (`WHERE a OR b AND position > ?`) that silently checks only part of the condition — **under-strict**, the failure direction that loses data rather than retrying. | Three rules using existing fixtures: `condition_with_match_all_query_rejects_any_later_event`; `condition_with_multi_item_query_rejects_on_either_item`; `condition_with_tag_only_query`. | T | **yes** |
| C6 | `from` at an unoccupied position is unspecified, and forwards `from`+`limit` is untested | `suite.rs:262-329` | `ProjectionStore::checkpoint`'s documented resume recipe (`projection.rs:86-88`, "Feed this to `ReadOptions::from` — after advancing past it") means `checkpoint + 1`, which on any sparse adapter usually names a gap. Under `>=` semantics the replay resumes; under exact-seek semantics it returns nothing and the projection stalls **forever, silently**. Both adapters pass all 27 rules. | Three rules + one doc sentence on `ReadOptions::from`: "the read is bounded by `position >= from`, or `position <= from` when backwards; `from` need not name an existing event." Rules: `read_from_a_position_with_no_event`, `read_from_with_limit` (forwards), `read_backwards_with_limit` (no `from`, asserts the *newest* comes back — an adapter applying `LIMIT` before `ORDER BY … DESC` returns the oldest, inverting a condition probe from "anything since?" to "anything ever?", a permanent retry livelock). | S | **yes** |
| C7 | Value-edge coverage is one event | `suite.rs:417-431` — `append_preserves_event_payload` | Every boundary of the column mapping is unchecked: zero-length BLOB vs NULL (`Some(Bytes::new())` collapsing to `None` in a nullable `metadata BLOB` — a bug the intended schema invites), `VARCHAR(n)` truncation at `MAX_TAG_LEN`, `NOCASE` collation breaking non-ASCII tag matching, `SQLITE_MAX_VARIABLE_NUMBER` on a many-tag event, and — most likely of all — an `INNER JOIN event_tag` silently dropping every **untagged** event from `Query::all()`. | Add: `empty_payload_round_trips`, `metadata_none_and_empty_are_distinct`, `event_with_no_tags_round_trips_and_matches_query_all`, `max_length_tag_and_event_type_round_trip`, `non_ascii_event_types_and_tags_round_trip_and_match`, `many_tags_on_one_event_round_trip` (64), `large_payload_round_trips` (1 MiB), `read_on_an_empty_store_is_empty`, `condition_on_an_empty_store_allows_the_append`. Five to ten lines each. | S | no |
| C8 | `append_is_atomic` does not test atomicity | `suite.rs:385-403` vs `:529-545` | It triggers rejection via the condition check, which **precedes any write**, so no mid-write failure is ever induced — it is `condition_rejection_leaves_store_unchanged` reworded. The suite advertises "Append \| atomic" at `lib.rs:36` and does not check it. A conformance-green adapter can leave a partial batch on a mid-write failure, which no replay can distinguish from a real one. | A `FaultyStore<S>` decorator in the testkit that injects `Self::Error` on the Nth write, plus `append_is_atomic_under_a_mid_batch_failure`. Rename the existing rule `rejected_append_writes_nothing`. Two rules asserting the same thing inflate the count and cost an adapter author two debugging sessions for one bug. | S | no |
| C9 | Multi-item de-duplication is untested | — | Two query items, one event matching both, must be returned **once**. An `event_tag` join without `DISTINCT` — which the schema sketch invites — returns it twice, and every fold double-counts. | `query_returns_an_event_matching_two_items_once`: assert `found.len() == 1`. Ten lines; one of the two rules that alone would have caught two of the four undetected bugs. | T | **yes** |
| C10 | Two rules assert only a count | `suite.rs:115-119`, `:140-144` | `query_item_tags_are_and` and `query_item_tags_match_supersets` assert a length, so an adapter returning the *right count of the wrong rows* passes — on the two rules carrying the AND-semantics of tag matching, the hottest path in any DCB query. (The third citation, `suite.rs:159-162`, is `assert!(found.is_empty())`, which is complete — not an instance.) | Compare `positions_of(&found)` against positions the store assigned, following `suite.rs:189-196`. Two lines each. | T | no |
| C11 | Batch self-conflict: pinned by accident, documented nowhere | `memory.rs:195-221`; `append.rs:56-59`; `suite.rs:604-621` | **Adjudication:** two reviewers disagreed. The one who said "no conformance rule pins it" is wrong — `racing_conditional_appends_elect_one_winner` appends a `StudentSubscribed` tagged `course:c1` under a condition whose query is exactly that, and asserts `first.is_ok()` at `suite.rs:614`. A post-insert adapter fails **there**. But its message reads "the first handler to commit must succeed", so the author will diagnose a bug in their happy path rather than "you are evaluating your condition against your own writes" — and since the standard DCB uniqueness shape has the condition matching the very event being written, such a store rejects *every* conditional append. `append.rs:56-59` ("if it holds at least one event matching … after `after`") is genuinely ambiguous about which state "holds" refers to. | One sentence on `AppendCondition`: "The condition is evaluated against the store's state **before** any event in this batch is written; a batch can never conflict with itself." Plus a dedicated rule `condition_ignores_the_batchs_own_events` whose failure message names post-insert evaluation as the cause. Justify pre-state on principle, not precedent: an `AppendCondition` means "nothing I did not see has appeared", and a batch's own events are by definition what the caller decided to write. | T | **yes** |
| C12 | The rule list is hand-duplicated; `#[tokio::test]` is welded in | `testkit/src/lib.rs:93-129` (27 names) vs `suite.rs` (27 fns) | Nothing enforces agreement, in the one artefact whose value proposition is that silent failure is impossible. Every downstream adapter also takes a tokio dev-dependency for rules that use no timers, no I/O and no `spawn`. **Adjudication:** the scheduling half of this finding is refuted — `RUNBOOK.md:72` already pulls the registry forward to phase 1. The duplication is not covered by that row and is worth fixing now. | Callback ("x-macro") pattern, verified to compile on 1.97.1 / edition 2024 including invoking a macro through a `$callback:path` metavariable. Two gotchas: the emitter must be `#[macro_export]`ed even when `#[doc(hidden)]` (macro_rules items live in a flat crate-root textual namespace), and the callback must be `$crate::`-prefixed or it resolves in the caller's scope — the exact bug being fixed. Each runtime is ~10 lines: `__emit_tokio`, `__emit_wasm`, `__emit_blocking`. Consider making blocking the default. Do **not** reach for `libtest-mimic`: the case list is statically known and you want one `#[test]` per rule so `cargo test <rule_name>` and IDE gutters work. | S | no |
| C13 | Both testkit doctests are inert; the rules table over-claims | `testkit/src/lib.rs:12-15`, `:35`, `:64-71` | The usage example is gated on a feature that does not exist; the macro's example is swallowed by an `ignore!` macro. Neither compiles a line of the API, in the crate that checks claims. The "Positions" row claims "gaps permitted" is *checked*; it is the absence of an assertion. | Make the crate-level example runnable by calling a rule directly — `happenstance` (with `memory`) and `tokio` are already dev-deps. Keep a separate ```` ```text ```` block for the macro invocation, honestly labelled. Change the row to "unique; strictly monotonic (gaps are permitted and not asserted on)". | T | no |
| C14 | `MAX_TAG_LEN` / `MAX_EVENT_TYPE_LEN` are promises no rule checks | `tag.rs:10-14`, `event.rs:13-14` | A SQLite adapter declaring `tag VARCHAR(64)` passes all 27 rules and truncates in production on a legitimate 200-byte tag. | `accepts_maximum_length_identifiers`: build an event at both bounds, append, read back, assert byte-identical. Keep the constants at contract level — 255 errs in the safe direction, since raising a limit later is non-breaking and lowering it is not. | T | no |
| C15 | No `cargo-mutants`, though it is the automated form of the experiment that exposed the suite | `.gitignore:12-14` reserves `**/mutants.out*/`; nothing runs it | The suite's strength is currently a **count** (27) rather than a measurement, in a project whose thesis is that a claim is worth what its test is worth. | Add `cargo mutants --package happenstance-core` as an optional xtask step (fits the existing probe-based skip at `xtask/src/main.rs:30` alongside hack and deny) plus a weekly CI job, and put the baseline in the README beside "27 rules". "27 rules, and they kill 94% of mutations of the contract's own semantics" is a claim no competing crate can make. Rank the rest honestly: **loom** is worth ~40 lines on the reference store (`memory.rs:189-194`'s atomicity claim is currently only a comment, and that store is the oracle); **coverage** (`cargo-llvm-cov`) is cheap; **fuzzing** is worth one structure-aware target on the serde impls once D1 and the allocate-before-validate bug are fixed; **miri should be skipped and said to be skipped** — `forbid(unsafe_code)` means there is no UB of the project's own to find. | S | no |

### 5.2 The contract's unstated semantics

Each of these is a sentence the contract does not say, which two conformant adapters can therefore
answer differently. All are free to fix now.

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| S1 | **The position-visibility invariant is stated nowhere** | `event.rs:92-97`; `suite.rs:355-365` | The entire safety argument for `AppendCondition::after` depends on it: transaction A takes position 10, B takes 11, B commits first, a reader observes 11 and conditions on `after: 11` — event 10 matches the query, the reader never saw it, and it is never checked. The consistency boundary DCB exists to enforce is simply not enforced. SQLite with `BEGIN IMMEDIATE` is immune because it serialises writers, so the flagship adapter will not surface it — but `happenstance-sqlite/src/lib.rs:16` names "a path to Postgres later", where `BIGSERIAL` exhibits exactly this, and `happenstance-sync`'s whole job is merging two independently-ordered logs. Found in production, by a user, as a violated invariant with no trace. | On `EventStore::read` and `SequencePosition`: "Once a reader has observed position P, no event may subsequently become visible at a position ≤ P. Positions must become visible in assignment order. Gaps in the visible sequence are permitted only if they are **permanent**. An adapter that allocates positions before commit (a SQL sequence, `BIGSERIAL`) does NOT satisfy this without additional work — serialise writers, or track a gap-aware watermark and never report a position above it." Give it an ADR, and an optional adapter-supplied hook in the concurrency macro that opens two overlapping write transactions. | S | **yes** |
| S2 | Read laziness / isolation | see D7 | — | see D7 | S | **yes** |
| S3 | Batch self-conflict | see C11 | — | see C11 | T | **yes** |
| S4 | Empty-batch precedence | see D8 / §3.2 | — | see §3.2 | T | **yes** |
| S5 | `append` cancel-safety is unspecified, and `AppendError` cannot say "unknown" | `store.rs:130-145`; `error.rs:150-166`; `memory.rs:184-224` | Callers drop append futures routinely — `tokio::time::timeout`, `select!`, `JoinHandle::abort`, client disconnect. The contract's only durability statement is about partial batches, not cancellation. The reference store answers by accident (no `.await` inside, so trivially cancel-safe; measured: dropped-before-first-poll leaves log len 0). No real adapter is — dropping a `spawn_blocking` `JoinHandle` does **not** cancel the closure; the COMMIT executes and the position is lost. And `Store(E)` conflates "malformed statement, definitely not written, safe to retry" with "socket timeout, may have been written, a retry duplicates" — for an event store that is the distinction that matters. The crate's headline claim (`lib.rs:49-52`) is one distinction short of what operators need. | Two changes, free with zero adapters in existence. (1) On `append`: "Dropping the returned future must leave the store in a state the caller can recover — either the whole batch is durable or none of it is. An adapter that cannot guarantee this must say so, and callers must treat a dropped append as an unknown outcome." (2) Add `AppendError::Indeterminate(E)`. The enum is `#[non_exhaustive]` so it is technically additive later, but **the adapter population is what freezes**, not the enum — a variant nobody produces is worse than none. Add `append_is_atomic_under_cancellation`: build the future, poll once with a no-op waker, drop it, assert the log is unchanged or fully written. | S | **yes** |
| S6 | **Self-matching conditions give retries at-most-once semantics — free, unclaimed, and undocumented** | `append.rs:18-32`; `examples/course-subscriptions/src/main.rs:86, :113, :177` | Measured against `MemoryEventStore`: (A) condition query matches the batch's own events → retry **rejected**, log len 1; (B) condition query does not match them → retry **accepted**, log len 3; (C) `condition: None` → accepted, len 2. Shape A is exactly-once with no idempotency key, no dedup table, no event id. Shape B is legal DCB ("append `StudentSubscribed` provided nobody changed the capacity") and double-writes silently. All three handlers in the canonical example are shape A **by accident**. The project is simultaneously carrying an unclaimed exactly-once story and an untelegraphed footgun, from the same mechanism. | Name it beside "The usual shape": "**Self-matching conditions are retry-safe.** If every event in the batch would itself match `fail_if_events_match` above `after`, re-submitting after an unknown outcome is rejected rather than duplicated. Prefer this shape: it is the cheapest exactly-once mechanism available and needs no event identity." Add `condition_makes_a_self_matching_retry_idempotent`. Have phase 6's command loop `debug_assert` when emitted events do not match the condition query — the same check as the fold/query divergence hazard, from the other side. | T | **yes** |
| S7 | Immutability is never stated, and **tags cannot be crypto-shredded** | `event.rs:159-247`; `tag.rs:11-14, :28`; ADR-0003 | `immutab`, `delete`, `redact`, `tombstone`, `GDPR`, `erasure` return **zero hits** across the contract, the ADRs and the suite. The most fundamental property of an event store is assumed everywhere and stated nowhere, so no rule checks it and an adapter that silently rewrote a payload passes all 27. The reason to state it is that you will eventually need to break it: crypto-shredding works beautifully here (opaque `Bytes` means ciphertext is indistinguishable from plaintext to every adapter — an unclaimed benefit of ADR-0003) — but it **cannot cover tags**. `student:s1` is plaintext, indexed and queryable, and must stay so because `Tags::contains_all` *is* the query mechanism. Encrypt the tag and DCB stops working. So the one erasure technique the payload design enables leaves the personal identifier in the index, permanently, in exactly the column `tag.rs:11-14` tells adapters to index — and `Tag::key_value("student", "s1")` is the crate's own doc example. Any EU adopter hits this in their first legal review. Note also that `RUNBOOK.md:67`'s content-hash-vs-Lamport identity choice silently made the erasure-friendly call: a content hash makes any redaction indistinguishable from corruption and breaks every peer's dedup. | Three things. (1) One sentence on `Event` stating immutability as an adapter obligation, plus a ten-line rule `event_is_immutable_across_reads`. (2) A short ADR *Erasure and the append-only log*: the payload is opaque so crypto-shredding is the supported mechanism; **tags are plaintext and therefore not erasable**, so callers must put a surrogate key in a tag and never unpseudonymised personal data; state explicitly whether an adapter MAY offer out-of-band redaction, requiring it to preserve `SequencePosition` exactly. (3) Put the tag guidance in `Tag`'s own docs, and add the privacy half of the reasoning to the identity ADR. | S | **yes** |
| S8 | A truncated log is indistinguishable from a complete one | `store.rs:198-208`; `projection.rs:86-88`; `suite.rs:336-365` | There is no `earliest_position()`, no `TruncatedBefore` error, and no way for a store to say "history below P is gone". `read_decision_model` reads from the beginning, folds whatever comes back, and hands the caller a position to condition on; if the beginning was discarded, the decision is made from a partial history and the append **succeeds**. The suite already permits this: `positions_are_unique` and `positions_are_strictly_monotonic` both hold on a truncated log, and SP3 permits gaps. Every store that runs for years needs retention. | Do not build retention now; reserve the seam. Add `earliest_position(&self) -> impl Future<Output = Result<Option<SequencePosition>, Self::Error>>` defaulting to `Ok(None)`, documented as: a caller whose read began below this has an incomplete history and must not treat the result as a decision model. Requires the `Sync` change in §3.1, so land them together. If the seam cannot be added now, add a ledger row and one sentence: this contract assumes a complete log; an adapter MUST NOT discard events. | S | no |
| S9 | `read` is a synchronous function on an async port with no non-blocking obligation | `store.rs:101-116`; `memory.rs:150-182` | See D7 rule 1. Load-bearing: `spawn_blocking` panics outside a runtime, so an adapter that starts blocking work at `read` time panics rather than merely stalling. | One sentence, plus fixing the oracle. Fold into D7. | T | **yes** |
| S10 | Multi-tenancy has exactly one workable answer and nothing says which | `query.rs:152-155`; `event.rs:209-214`; no ledger row | Tenant-as-tag means every query must carry the tag or it leaks, `Query::all()` **crosses every tenant**, and an `AppendCondition` on `Query::all()` is a global write lock across all tenants — while nothing can enforce that every `Event` carries the tag, because `with_tags` is optional. Store-per-tenant is natural for SQLite, impossible to query across, and multiplies the connection pool. The first adopter invents a convention and it will not be the one the typed layer assumes. | One paragraph in the crate docs plus a ledger row: a tenant is a tag by convention (`tenant:acme`); phase 6's `DecisionModel` must inject it into every query and every event so it cannot be forgotten; `Query::all()` is a single-tenant or administrative operation, never a multi-tenant one. | T | no |

### 5.3 API design and ergonomics

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| A1 | **The query and the fold state the consistency boundary twice, and nothing checks they agree** | `examples/course-subscriptions/src/main.rs:114-173`, `:212` | A reviewer added a sixth event type to the worked example, taught the fold about it, forgot it in the query — and the program **silently oversold 7 units of 0 stock**, with zero compiler, clippy or conformance signal. Both halves of DCB break from one omission: the decision is made on state that excludes those events, *and* the append condition fails to guard against them. Undetectable by review because the omission is an absence; undetectable at runtime because the store behaves exactly as instructed. This is the single largest correctness hazard in hand-rolled DCB and it is currently 100% on the user. | **Adjudication:** one reviewer said the derive macro is "the only construct that makes them provably agree" — that is wrong and would push the design the wrong way. Exhaustiveness comes from folding over a **decoded** enum, not from a proc macro. If `DecisionModel::apply` takes `Self::Event` (a domain enum) rather than a `SequencedEvent`, the `match` is exhaustive and the compiler catches the divergence; the derive then only removes `EventType`/`Tags` mapping boilerplate. So the binding requirement on phase 6 is: **the fold takes a decoded type, and `query()` is derived from the same declaration the decode is derived from** — never restated by hand. `execute()` must then use the same `Query` *value* for both the read and the `AppendCondition`. With that, deferring `happenstance-macros` past 0.1 is defensible. `RUNBOOK.md:236-238` already names the composition as "the part to get right"; make the agreement requirement explicit and add a `trybuild` compile-**fail** as the phase's proof artefact — a passing test proves nothing here. | L | **yes** |
| A2 | The retry loop every user must write **cannot be written as a closure-taking helper** | — | `AsyncFnMut` + `tokio::spawn` fails with "implementation of `Send` is not general enough" the moment the closure captures an entity id, which every real handler does. So the one piece of infrastructure every application needs cannot be provided in the obvious shape. | A trait with a **synchronous `decide`** — compiled and run by a reviewer: 8 spawned tasks, 4 won, 4 rejected, 4 units in stock. This is the same shape A1 wants, so they are one design. Make it phase 6's central artefact. | L | **yes** |
| A3 | `ReadOptions::from` is inclusive while `AppendCondition::after` is exclusive | `query.rs:228-229`, `:246-251`; `projection.rs:86-88` | `checkpoint` returns the last position **applied**, so resuming requires advancing past it by hand. The obvious `ReadOptions::new().from(checkpoint)` re-applies the last event on every restart — silent double-counting in any non-idempotent projection. The vocabulary asymmetry is exactly what a tired developer gets backwards, and the failure is invisible until the numbers drift. | Add `ReadOptions::after(SequencePosition)` (exclusive) and `after_opt`. Resume becomes `ReadOptions::new().after_opt(checkpoint)` — one call, no arithmetic, and the two `after`s in the crate then mean the same thing. Keep `from` for the genuine "start at exactly this position" case. Free now, breaking after publish. | T | **yes** |
| A4 | No extension-trait strategy | `store.rs:91-146`, `:162`, `:198` | `#[non_exhaustive]` does not apply to traits, so every future convenience is either a semver break or a free function bolted to the crate root. | See §3.1 and §3.4: `EventStoreExt` with a blanket impl for derivable conveniences; provided methods on the port for pushdown-capable ones; the `Sync` change is the prerequisite for the latter. Move `collect` and `read_decision_model` onto the ext trait. | M | **yes** |
| A5 | `read_decision_model` cannot stream and returns a bare tuple | `store.rs:198-208` | It hard-codes `ReadOptions::new()` and `collect`s, so a decision model over a hot tag (a popular course, a busy customer, a long-lived process) holds every matching event in memory when the fold needs only an accumulator — on a code path that runs under contention and retries, with no snapshotting story as an alternative. It is also named for a thing it does not return, and the operation the caller *always* performs next — turn what I read into the condition — is written once per application instead of once in the library. (The "calls the function its own docs warn against" framing is wrong: `store.rs:151-152` explicitly sanctions "rebuilding a decision model that fits in memory"; the prohibition is on full-log replay.) | Add `fold_decision_model(store, query, init, f)` beside it, tracking the last position while folding so it returns exactly what `after_opt` wants without ever holding more than one event. Return `#[non_exhaustive] struct DecisionSlice { events, last_position }` with `fn close(&self, query: Query) -> AppendCondition`, so the example's four-argument `commit` collapses to `store.append(batch, Some(slice.close(query)))`. Add `read_decision_model_with(store, query, options)`, and document the `backwards().limit(1)` idiom for "I only need the latest state" — a large constant-factor win that is not discoverable today. | S | no |
| A6 | Six to eight `?` per handler across four non-unifying error families | `examples/…/main.rs:86-104`, `:114-173`; `error.rs` | Five of the six are on **string literals the author typed** — compile-time information deferred to runtime and charged as `?` noise at every call site. `InvalidTag`, `InvalidEventType`, `InvalidQuery` and `AppendError<E>` do not unify, so the only worked example is written in a style a library author cannot copy (it uses `anyhow`, which is legitimate for an example per `CONTRIBUTING.md:71` — but the library ships nothing to copy instead). | Three changes: const-constructible `EventType` (§3.7); a `tags!{ course: c, student: s }` macro collapsing N pair-validations into one `?`; and a single `happenstance_core::Error` umbrella with `From` impls from all three. Together `define_course` drops from six `?` to two — the read and the append, the only two that can genuinely fail at runtime. (Note: adding `From<InvalidTag> for InvalidQuery` would **not** help — `QueryItem::new` takes an already-built `Tags` and never produces an `InvalidTag`; the two errors arrive at different call sites.) | M | **yes** |
| A7 | `futures_core` is not re-exported, though every adapter must name `Stream` | `lib.rs:104-106` vs `store.rs:117-121` | `bytes` is re-exported with an explicit rationale that applies verbatim to `futures_core`, which is the *harder* dependency to get wrong: it appears in `read`'s signature, so no adapter or decorator exists without it, and a version mismatch produces a baffling "expected `impl Stream`, found `impl Stream`" rather than a dependency error. The asymmetry falls on the wrong side, and the affected population is exactly the audience the contract crate exists for. | `pub use futures_core;` beside `pub use bytes;`, one line. Write the policy down in ADR form: `bytes` and `futures-core` are public API, so a **major** bump of either is a major bump of the contract crate. (`futures-core = "0.3"` is not broken by minor bumps — Cargo treats 0.3.x as compatible; only 0.4 would break, and there has been no 0.4 in seven years.) | T | **yes** |
| A8 | `ProjectionId` validates nothing and carries none of its siblings' conversions | `projection.rs:41-60` | It becomes the primary key of a checkpoint row. An empty or control-character-bearing id is a silent data-integrity problem in the table whose whole job is not losing track of where a replay got to. `ProjectionId::from("x")` fails with E0308. The unexplained inconsistency also teaches readers that validation in this crate is optional. | Make `new` fallible with `InvalidProjectionId` mirroring `InvalidEventType`, add `TryFrom<&str>`, `TryFrom<String>`, `FromStr`, `AsRef<str>`, `Borrow<str>`, and the serde impls. **Or** say in the docstring that it is a deliberately opaque operator-chosen key with no constraints — the finding is the silence, not the choice. Fold into the projection-port reshape. | S | **yes** |
| A9 | Standard traits missing where the guidelines expect them | `tag.rs:115-135, :258-281`; `event.rs:70-90, :182`; `query.rs:37, :143`; `append.rs:50` | `Borrow<str>` matters most: a `HashMap<EventType, Handler>` dispatch table is the core data structure of a codec/decision-model registry, and without it every lookup allocates an `EventType` first. `Hash` on `Event` is what idempotent-ingest dedup needs. `into_iter()` on `&Tags` yielding `&Tag` with no owned counterpart is a silent wrong-type trap. | Add `Borrow<str>` for `Tag`/`EventType` (sound — their `Hash`/`Eq`/`Ord` already delegate to the single `Box<str>` field, which is `Borrow`'s consistency requirement); `FromStr` for both; `Hash` on `Event`, `SequencedEvent`, `Query`, `QueryItem`, `AppendCondition`; `IntoIterator for Tags` yielding `Tag`; `Extend<Tag> for Tags` **re-canonicalising per call** (sort+dedup, not push — or it breaks the invariant `tag.rs:137-144` depends on), documented O(n log n). All additive and non-breaking, so they need not block 0.1 — but their absence is what a reviewer notices first. | S | no |
| A10 | `Tags` cannot be read by key | `tag.rs:198-255`; `examples/…/main.rs:136-138` | The single most common thing a fold does — "is this event about *my* student?" — is hand-written as an O(n) scan, twice, in the canonical example. The crate pays for sorting and then does not spend it. | `pub fn value_of(&self, key: &str) -> Option<&str>` via `partition_point` on the `"key:"` prefix — O(log n). Decide first whether repeated keys (`student:s1`, `student:s2` on one event) are legal: they are today, and `value_of` returning `Option` would silently pick one. Add `values_of` if so. Do it before the example is rewritten so the example teaches the good form. | T | no |
| A11 | `read`'s stream captures the `&Query` lifetime | `store.rs:117-121` | Under RPITIT the opaque return type captures every in-scope lifetime including the query's, though no sensible adapter's concrete stream borrows it. A caller cannot return a read stream from a function, store one in a struct, or spawn a replay without keeping the `Query` alive — the ergonomics of the API's most-used method — and it makes the hold-a-stream-across-an-append rules awkward to write. (The E0799 is caused by `trait_variant`'s expansion re-emitting the signature inside a blanket impl where `Self` is an alias, not by RPITIT alone — which is a third argument for the ADR-0007 question in §3.1.) | Take the query by value: `fn read(&self, query: Query, options: ReadOptions) -> impl Stream<…>`. `Query` is a `Box<[QueryItem]>`; one clone per read is nothing next to the I/O, and every adapter translates it to SQL immediately. Rejected `type Stream<'a>: Stream<…>` GAT: `trait_variant` cannot add `+ Send` to an associated type, so both flavours would have to be hand-written. | S | **yes** |
| A12 | No prelude; E0034 leaks `TraitVariantBlanketType` into user errors | `store.rs:31-45`; `lib.rs:92-102` | Both flavours are exported side by side, so `use happenstance::*` or an editor auto-import of the wrong name produces E0034 on every method call. **Adjudication:** one reviewer claimed the docs show a different rendering than the real error — that is wrong; `store.rs:36-41` contains both "error[E0034]: multiple applicable items in scope" and "multiple `read` found" verbatim, and rustc prints both fully-qualified fixes. So it is well documented and triggers mainly on glob imports — low, not high. | Ship `happenstance::prelude` exporting `EventStore` (**not** `SendEventStore`), the value types, and the free functions, so `use happenstance::prelude::*` cannot produce E0034 — convention over configuration applied to imports. Keep both names at the crate root for adapter authors. Paste the literal string `TraitVariantBlanketType` into `store.rs`'s error block so the module docs are findable by the text users actually see. | T | **yes** |
| A13 | `collect` is the wrong name; `Query::from_item` is documented infallible and returns `Result` | `store.rs:148-184`; `query.rs:172-179` | `collect` collides with every Rust programmer's `Iterator::collect` reflex and hides the stop-at-first-error behaviour that is the point of the function — and it is the first symbol a reader meets in the crate-root doctest. `from_item`'s own `# Errors` section says "Infallible in practice", which teaches readers that `Result`s in this API are noise — the wrong lesson for the ones that matter. | Rename to `try_collect` (matching `TryStreamExt::try_collect` exactly) and expose it as `EventStoreExt::read_to_vec`. Keep it hand-written rather than pulling in `futures-util` — that reasoning at `store.rs:154-156` is right. Make `from_item` infallible: `pub fn from_item(item: QueryItem) -> Self`. Symmetry is not worth a lie in a `# Errors` section CI enforces the presence of. | T | **yes** |
| A14 | `ConditionViolated`'s message names neither the decision nor the conflict it already holds | `error.rs:93-104` | The one error a developer sees at 2am is purely descriptive, and the `conflicting_position` field — which adapters populate when they can (`memory.rs:205-207` does) — never reaches the message. | `#[error("append condition violated at position {conflicting_position:?}: an event matching the append condition was written after the decision model was read — rebuild the decision model and retry")]`. Apply the same "name the fix in the message" standard as `InvalidQuery::NoItems` (`error.rs:66`), which is the best string in the crate. Also lead the `AppendError` doctest with `is_condition_violated()` (`error.rs:170`, already the right API and already used in `memory.rs:66`) and demote the `match` — then keep `#[non_exhaustive]`, which costs nothing on the `is_*` path. | T | **yes** |
| A15 | `InvalidTag::Empty` misdescribes the common case | `error.rs:20-22`; `tag.rs:70-72` | `Tag::key_value` returns it for an empty key **or** an empty value, but the message says "a tag must not be empty", which is a third, different failure from `Tag::new`. A developer with an unset `Option` flattened to `""` — the common case — is pointed at the wrong half of their input. The variant's own doc comment at `error.rs:20` is already honest; only `Display` is not. | Split into `EmptyKey` / `EmptyValue` alongside `Empty` — `InvalidTag` is `#[non_exhaustive]` (`error.rs:18`), so it is cheap now and awkward later. At minimum fix the message. | T | no |
| A16 | The serde path allocates the attacker's value before validating it | `tag.rs:304-309`; `event.rs:315-320` | `Tag::deserialize` is `String::deserialize(d)?` **then** `Tag::new(raw)`, so the 255-byte check runs after the whole string is materialised. Measured: an 8 MiB JSON string is built, then rejected in 2.4 ms. For `serde_json` over a buffer that is a nuisance; for a length-prefixed format read from a socket — `bincode`, or `postcard` over a streaming reader, both natural for `happenstance-sync` — the attacker controls a length prefix that becomes a `Vec::with_capacity`. That is the classic deserialisation bomb, in the one path whose purpose is ingesting bytes from another machine. | A `Visitor` with `visit_str`/`visit_string` checking length before constructing an owned `String`, for both `Tag` and `EventType`. ~10 lines each. Bound `Bytes` with `MAX_EVENT_DATA_LEN` the same way. | S | no |
| A17 | `Query`'s deliberate exhaustiveness is undocumented | `query.rs:143` | `Query` being exhaustive is **right** — an adapter must translate every variant into storage, and a `_ =>` arm it cannot implement correctly is worse than a compile error; adding a variant *should* break every adapter, loudly. But nothing records that reasoning, so a future contributor auditing for missing `#[non_exhaustive]` will add it and quietly convert a compile-time obligation into a runtime failure. | A comment at `query.rs:143`: "deliberately exhaustive: adding a variant MUST break every adapter." Note this coexists with D2 — the *variant* becomes non-exhaustive, the *enum* stays exhaustive at the variant-list level for adapters, or you accept both and rely on `items()`. Decide and write it down. Also worth a doc note on `ReadOptions`: `#[non_exhaustive]` blocks functional-update syntax, so `ReadOptions { backwards: true, ..Default::default() }` fails with E0639 — the idiom every Rust programmer tries first with an options struct. | T | **yes** |
| A18 | The poison-recovery justification does not hold as written | `memory.rs:118-127` | The comment argues ignoring poisoning is safe because "every mutation happens in one `extend`" — but `Vec::extend` is **not** atomic, so the premise does not imply the conclusion. The conclusion is true for a reason the comment does not state: the closure at `memory.rs:219-221` calls only `position_at` and `Event::clone`, and neither can panic (`Bytes::clone` is a refcount bump that aborts on overflow; allocation failure aborts). A future change adding anything fallible to that closure silently breaks the guarantee — and the same reasoning is what makes the `append_is_atomic` conformance rule true, so the comment is load-bearing for a rule, not just for `read_guard`. | Restate as "the extend closure is panic-free", and say that anything added to it must preserve that. | T | no |
| A19 | Both reads materialise the full match set before applying `limit` | `memory.rs:163-178` | The `truncate` at `:176-178` is **outside** the if/else, so the forward path buffers too. On a 1M-event store, `backwards().limit(50)` clones a million `SequencedEvent`s and discards 999,950. The store is explicitly "not built for scale", but "the 50 most recent events" is the specification's own worked example and the first thing anyone will benchmark. | Insert `.take(options.limit.map_or(usize::MAX, NonZeroUsize::get))` before `.cloned().collect()` on both arms and drop the `truncate`. Free, and makes the read O(limit). | T | no |
| A20 | `MemoryStoreError` is not `#[non_exhaustive]` | `memory.rs:143-145` | Close to theoretical — the enum is uninhabited, so no caller can construct or match a value; adding a variant breaks only code naming it in type position. | Add the attribute. | T | no |

### 5.4 Adapter implementability and performance

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| P1 | `Query` has no canonical decomposition into pushdown arms | `query.rs:143-205`, `:57-117` | The accessors are sufficient but not helpful. The fast physical plan requires decomposing into the cross product of items × types, emitting each as a **bare top-level compound arm** and merging in the engine — a non-obvious transformation. The obvious translation (a materialised `position IN (… UNION …)` set) is **970× slower** and buffers the entire result before the first row. So the SQLite adapter, the Cloudflare adapter and every third-party adapter each independently reinvent a query planner. Nothing signals that the shape of the emitted SQL is the whole game. | Purely additive inherent methods, so no semver hazard, but shipping in 0.1 is what makes the fast plan the obvious plan: `Query::index_arms() -> impl Iterator<Item = IndexArm<'_>>` (`{ event_type: Option<&EventType>, tags: &Tags }`, `Query::All` yielding one unconstrained arm, arms may overlap so a union must deduplicate) and `Query::arm_count()`. Document alongside that multi-tag arms must be probed **most-selective-tag-first**, and that the adapter must supply its own per-value cardinality because SQLite cannot: the probe-order swing is 650× (0.02 ms vs 13.0 ms), and `ANALYZE` stores only an average — `sqlite_stat1` for a 4.74M-row `event_tag` is the single row `'4740000 7 1'` while actual values range 19 to 34,159. Re-measured post-`ANALYZE`: unchanged. `arm_count()` also serves the `MAX_QUERY_ITEMS` bound in §3.7. | M | no |
| P2 | The schema sketch omits `event_type` from the tag index **[PLAUSIBLE — schema claim confirmed, figures unverified in-repo]** | `happenstance-sqlite/src/event_store.rs:10-28` | `event_tag(tag, position)` carries no type, so a `QueryItem`'s type constraint must be a join back to `event` — which collapses SQLite's `MERGE (UNION)` streaming plan into a co-routine over a temp b-tree, and makes the append-condition probe walk the full posting list through a join **while holding the `BEGIN IMMEDIATE` write lock**. A 100 ms probe inside that lock serialises every writer: a throughput cliff, not a latency nit, firing precisely on conditions over popular tags — the contended case. `RUNBOOK.md:136` names this sketch as phase 1's starting point. | Keep the key `(tag, position)` so the range stays sorted by position — that ordering is what keeps `MERGE (UNION)` alive — and carry `event_type` as a **covering payload column** so the type filter is a residual test on the index row with no table lookup. Add `CREATE INDEX event_type_idx ON event(event_type, position)` for items with no tags, and a `tag_cardinality` table. Reported measurements: typed arms 0.009 ms, untyped 0.007 ms, multi-tag intersect (selective first) 0.012 ms — one index serves all three. Also record that `BEGIN IMMEDIATE` + a `min(position)` probe yields `ConditionViolated::conflicting_position` for free (0.006 ms, same as a bare `EXISTS`) whereas `INSERT … SELECT … WHERE NOT EXISTS` yields only a boolean and cannot distinguish `ConditionViolated` from `NoEvents` without a second probe. Note the sketch is a doc comment on an unimplemented stub and `RUNBOOK.md:65` still lists tag storage as a phase-1 decision with its ADR pending — so this is a sketch to supersede, not a frozen default. | S | no |
| P3 | `Query` bounds nothing against adapter pushdown limits **[PLAUSIBLE — causal step is adapter-strategy-dependent]** | `query.rs:164-170`, `:57-77` | Compound SELECTs are only one way to push a `Query` down; a single flat `WHERE (…) OR (…)` has no compound-SELECT term at all and hits `SQLITE_MAX_EXPR_DEPTH`/`SQLITE_MAX_VARIABLE_NUMBER` at very different thresholds. The defensible version: an adapter's pushdown limits are the caller's problem with no contract-level signal. | `MAX_QUERY_ITEMS` (§3.7) plus `arm_count()` (P1); document in `QueryItem`'s docs that "an item with N types costs N index ranges in a pushdown adapter"; have the SQLite adapter chunk rather than fail. | S | no |
| P4 | No benchmarks anywhere, for a library whose first stated goal is "fast, efficient" | workspace — no `benches/`, no criterion | The append-condition strategy (`RUNBOOK.md:64`) is explicitly a performance decision deferred to a measurement that cannot happen. It also forecloses the most persuasive artefact this project could publish: a comparison table against `disintegrate` (`README.md:161-162`). | Do for performance what the testkit does for correctness: `event_store_benchmarks!(MyStore::new())` in the testkit behind a `bench` feature — append throughput, conditional append under contention, replay of N events with and without a tag filter. Adapters inherit it the way they inherit conformance, and "which adapter is faster" becomes answerable by the same mechanism. **No other Rust event-sourcing library ships that.** Not a 0.1 blocker, but the harness must exist before the SQLite append strategy is chosen, because that is the decision it is supposed to inform. | M | no |

### 5.5 Documentation, ADRs and the RUNBOOK

Drift against **code** is close to zero — "27 rules" is exactly 27, the Send-stream test CLAUDE.md
points to exists, every ADR link and RUNBOOK anchor resolves. The drift is internal.

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| R1 | CLAUDE.md names the wrong crate in five places once the rename lands | `CLAUDE.md:9-10, 26-39, 50-52, 128` | **Adjudication:** the rename is *not* executed (`RUNBOOK.md:223` lists it as unticked), so CLAUDE.md is accurate **today** — one reviewer called this critical, which is not honest. And CLAUDE.md pre-empts its own staleness at lines 113-115 ("the ledger there is the source of truth"), and `RUNBOOK.md:68` marks the row decided with a link. The genuine, unmitigated gap is that the rename's checklist has **no item for updating the constraint wording**. | Fold into §3.6: execute the rename and rewrite CLAUDE.md's "What this is", repository map, dependency rule and constraint 2 in the same commit. Same staleness in `happenstance-runtime/src/lib.rs:10` and its `Cargo.toml` description. | M | **yes** |
| R2 | Three "decided" ledger rows contradict the phase bodies they govern | `RUNBOOK.md:61-74` vs `:135-149, :226-228, :242-243, :318-330, :392-393` | **Adjudication:** downgraded, and one of the three does not hold. `RUNBOOK.md:58-59` states the convention explicitly ("a row marked **decided** has an agreed answer but no ADR yet"), so a decided row beside an open phase body is the documented state. The ledger:66 / phase-2 pairing is not a contradiction at all — the port still says "provisional" because phase 2 has not started. Two are real: a session reading phase 3's body implements UUIDv7-in-metadata while a session reading the ledger implements a Lamport pair on `SequencedEvent` — **different public APIs**; and the `!Send` reference store the ledger says was pulled forward to phase 1 appears in **no phase's work list**, so ADR-0001's own stated lift condition is scheduled nowhere. | Restate phase 3's identity bullet as the Lamport-pair answer, correct `happenstance-sync/src/lib.rs:33-36`, and put the `!Send` reference store into phase 1's Work and exit criteria where the ledger says it lives. Reduce each "decided" row to a one-line answer plus a link and push the reasoning into the phase body or an ADR — rule 3 (`RUNBOOK.md:18-21`) already says decisions are settled in ADRs, and 8 of 12 rows now violate it. | S | **yes** |
| R3 | The typed layer is scheduled after two adapter phases | `RUNBOOK.md:41, 202-228` | Phase 3's "Why here" conflates demonstration with discovery. The adapter proves implementability; the typed layer proves the contract is *usable*, and only the latter forces changes to the crate every adapter is built against. The unarguable part: `RUNBOOK.md:207` and `:242-243` are **stale with respect to ADR-0006**, which moved the projection runner into the contract crate and thereby dissolved phase 3's stated phase-2 dependency. The dependency graph was never updated. | See §7. | M | **yes** |
| R4 | The `!Send` proof is scheduled fifth, after four phases of code written against it | `RUNBOOK.md:43, 304-339`; ADR-0001:6-16 | **Adjudication:** the plan already contains the mitigation and one reviewer missed it — `RUNBOOK.md:72` reads "add a rule *registry* macro + a `!Send` reference store (**pulled forward to phase 1**)", which is precisely ADR-0001's own lift condition ("the cheapest such proof is a `RefCell`-backed reference store in the testkit"). The phase-5 prose at `:318-323` is **stale relative to the ledger** — that inconsistency is the real finding. Separately, a verifier ran `cargo check -p happenstance --target wasm32-unknown-unknown` **with** default features (including `memory`) and it succeeds, so the xtask gap is missing coverage, not latent breakage. | Reconcile `:318-323` with the ledger row; add the `!Send` store to phase 1's exit criteria; widen the xtask wasm step to check default features too (one word), which makes the README claim true as written. | M | **yes** |
| R5 | Four load-bearing contract decisions exist only as doc comments | `store.rs:125-128`; `error.rs:154-161`; `lib.rs:104-106` | **Adjudication:** two of the four claims do not hold — event identity **is** tracked (`RUNBOOK.md:67`, `:219`, `:226-228`, `:390-421`; only `recorded_at` is untracked), and `bytes` as a public dependency **is** decided by ADR-0003. Only `futures-core` has no ADR, and only the empty-batch/return-value pair is genuinely unrecorded. | Two short ADRs: *Public dependencies and the wire-format surface* (`bytes`, `futures_core`, the serde representation, which formats the feature supports) and *Batch non-emptiness and the append return value* (record the rejected alternatives — `Vec<SequencePosition>`, `Vec<SequencedEvent>` — since the identity decision may want the appended events back). The 255-byte limits and the `from`/`after` asymmetry need doc sentences, not ADRs: `MAX_EVENT_TYPE_LEN` (`event.rs:13-14`) currently has no stated reason at all. | M | **yes** |
| R6 | The release-hazard bias survives verbatim in phase 7's own justification | `RUNBOOK.md:408-421` vs `:430-432`, `:188-195` | The document diagnoses "treating an unpublished API as something to protect" as a bias that skews design toward the option that changes least — and then phase 7's "Why here" ("publishing freezes the public API") is that bias stated outright. The entire back half of the plan is ordered around avoiding a breaking change semver explicitly permits. | Edit two sentences to match the correction the document already makes: "publishing 0.1 starts the feedback loop"; replace "freeze" with "stabilise" throughout. Move a `0.1.0-alpha` ahead of the projection-port stabilisation and let 0.2 break. The stronger version of the argument: publishing early is how you get the outside feedback that would actually settle the projection port. | S | no |
| R7 | Phase 2 "freezes" the projection port against one adapter; phase 4 is not a publish dependency | `RUNBOOK.md:40, 45, 164-197, 259-268` | **Adjudication:** downgraded, because the argument aims at the wrong phase. The GAT's borrow-from-`Self` assumption already fails on the **SQLite** adapter's Send flavour (§3.5), so phase 2 — which *is* on the publish path — surfaces it, not phase 4. That strengthens the recommendation (do not freeze on one adapter) while refuting the framing. | Make the freeze conditional on a **second, structurally different** `Batch` shape (an owned buffer) being implemented against it, which phase 2 can do itself in an afternoon. Or ship behind `unstable-projection` (§3.5) and stop calling it a freeze. | S | **yes** |
| R8 | CLAUDE.md states constraint 5 without ADR-0004's "this is free to raise" caveat | `CLAUDE.md:61`; ADR-0004:10-13; `append.rs:101-102` | The nuance that makes the MSRV a preference rather than a limit lives only in the ADR and in `CONTRIBUTING.md:12-19`. CLAUDE.md — the document a session actually loads — states it as immovable, and code is already being written around it (`append.rs:101` avoids a let-chain). | One clause appended to `CLAUDE.md:61`: "Note ADR-0004: until first publish the MSRV is a preference, not a promise. Weigh it, do not obey it." | T | no |
| R9 | CONTRIBUTING claims a semver gate that cannot be running | `CONTRIBUTING.md:79-81`; `.github/workflows/ci.yml:66-77` | The job compares against the latest crates.io release of two unpublished crates, so it has no baseline; it is gated on `pull_request` and the history is direct commits to `main`, so it has very likely never executed. The one mechanism that would have caught D2 and the missing `Send + Sync` is inert. The RUNBOOK already knows (`:455-456`). | Point it at `--baseline-rev ${{ github.event.pull_request.base.sha }}` — works today with no registry involvement, keeps working after publication, and makes CONTRIBUTING true. Add one sentence to CONTRIBUTING marking it dormant until 0.1. Consider `cargo-public-api` snapshots committed to the repo: a diffable text file of the public surface in every PR is the single best review artefact for a crate at this stage. | T | no |
| R10 | ADR-0006's self-exemption is the weakest available defence of a correct conclusion | `adr/0006:19-26, 41-46, 150-160` | "A naming decision is settled by being made, not by being tested" is exactly the claim ADR-0005 could have made five hours earlier and would have been wrong to; and ADR-0006 criticises an unevidenced empirical claim about users, then makes one. **No cost to the code** — the conclusion is right and serde_core/serde is the correct precedent. The cost is to the corpus's credibility as a reasoning record. | Rewrite the closing note: naming decisions are settled by argument, not evidence; this one is settled because the argument is strong and no counter-argument survives — and here is the counter-argument that would reopen it (evidence that adapter authors, not applications, are the dominant import population). | T | no |

### 5.6 Packaging, CI and supply chain

The fundamentals are unusually strong — four healthy dependencies (`bytes` 1.12.1, `futures-core`
0.3.33, `thiserror` 2.0.19, `trait-variant` 0.1.3, all current, all tier-one maintainers), only two
deliberate public-API leaks, a feature powerset that actually builds (22 combos verified), a
`deny.toml` that passes, and an MSRV job that actually works (verified: `cargo hack --rust-version`
shells out to `rustup run 1.85`, which overrides `rust-toolchain.toml`).

| # | Finding | Location | Fix | Effort | Blocks |
|---|---|---|---|---|---|
| G1 | No licence text or README in the published `.crate` | D11 | copy licences into each crate dir, per-crate READMEs, `readme` key, `cargo package --list` in CI | T | **yes** |
| G2 | `serde` feature relies on `bytes` to enable `serde/alloc` | D12 | one word | T | **yes** |
| G3 | `cargo doc --no-default-features` fails; the powerset step is `check`-only | D13 | conditional doc links + a doc step | S | no |
| G4 | ~253 lines of hand-written wire format with **zero tests** | `event.rs:301-399`, `query.rs:269-352`, `append.rs:110-144`, `tag.rs:291-325` | Add `serde_json` **and** `postcard` dev-deps and round-trip every envelope type. Two of these have real logic worth pinning: `Query`'s `Serialize` maps `All`→`serialize_none` (so a wire `null` must come back as `All` and an empty array must be **rejected** via `NoItems`), and `QueryItem`'s `Deserialize` re-runs `QueryItem::new` (re-sorting types, rejecting the unconstrained item). Add a proptest that a deliberately non-canonical `Tags` payload comes back canonical — that guarantee matters most when a hostile peer is on the other end. Generators may belong in `testkit::fixtures` so `happenstance-sync` reuses them. | S | **yes** |
| G5 | `clippy::todo = "allow"` is workspace-wide for two `todo!()`s in one `publish = false` crate | `Cargo.toml:58-59` | `todo = "deny"` at the workspace, `#![allow(clippy::todo)]` at the top of `happenstance-sqlite/src/lib.rs`. Two lines; deletes a phase-7 checklist item. Same reasoning for `unwrap_used = "warn"` → `deny`, since test modules already opt out locally. (`RUNBOOK.md:442-444` schedules the restore, so the exposure is unenforced discipline, not a shipping hazard.) | T | no |
| G6 | `docsrs` cfg is only ever compiled **on** docs.rs, after publish, when failure is unfixable | `Cargo.toml:36-38`; `lib.rs:75, 88-89, 100-102` | `feature(doc_cfg)` is unstable and its spelling has moved. Add a nightly-probing xtask step and CI job: `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --all-features --no-deps`. Turns `RUNBOOK.md:440-441` from a manual check into a gate step. | T | no |
| G7 | CI runs a mutable branch reference, has no `permissions:`, and there is no release workflow | `.github/workflows/ci.yml:1-77`, `:52-54` (`dtolnay/rust-toolchain@master` — a **branch**), `:30/:34/:40/:74` | `permissions: contents: read` at workflow level; SHA-pin all five actions with a `# v4.2.2` comment; replace `@master` with a tag; add `--locked` so CI tests the dependency set the lockfile claims to pin. When phase 7 arrives, publish via crates.io **Trusted Publishing (OIDC)**, not a `CARGO_REGISTRY_TOKEN` secret — write the release workflow that way from the start. (`pull_request` rather than `pull_request_target` at `:6` is the safe trigger and is already correct.) | S | no |
| G8 | Advisory coverage stops when commits stop | `.github/workflows/ci.yml:3-7` | `schedule: - cron: "0 6 * * 1"` running only `cargo deny check advisories` (OS-independent, so not the three-OS gate). Set `unmaintained` explicitly in `deny.toml` rather than inheriting a default that has moved between releases. Low impact today with four dependencies; it scales badly once `rusqlite` (C) and `lbug` (C++ via cxx/cmake) are in the tree. | T | no |
| G9 | The wasm32 step checks only the `std` flavour, and `cargo xtask wasm` selects it by array index | `xtask/src/main.rs:64-77`, `:121` (`&REQUIRED[3..4]`) | Replace with `cargo hack check … --target wasm32-unknown-unknown --feature-powerset --no-dev-deps` (probed), keeping a mandatory plain `cargo check` so the constraint-1 guard is not skippable when cargo-hack is absent. Select the step by matching `step.name`, not by index. Both need `-p happenstance-core` after the rename (§3.6). | T | no |
| G10 | Release-hygiene inventory | repo root | Close the phase-0 clean-tree decision now (gitignore `.idea/`, commit `.mcp.json`) — it is load-bearing for `RUNBOOK.md:15-17`'s definition of "done" for **every** phase. Start `CHANGELOG.md` today with `## [Unreleased]` (ADR-0004 already promises MSRV bumps are "called out in the changelog", against a file that does not exist). Add `homepage` to `[workspace.package]`, `dependabot.yml` for cargo and github-actions, `SECURITY.md`, `CODE_OF_CONDUCT.md`. | S | no |

### 5.7 Governance and positioning

| # | Finding | Location | Consequence | Fix | Effort | Blocks |
|---|---|---|---|---|---|---|
| N1 | **The testkit has no compatibility policy, so adding a rule turns every passing adapter's CI red** | `happenstance-testkit/Cargo.toml` (`version.workspace = true`, no `publish = false`) | A third-party adapter writes `happenstance-testkit = "0.1"` — the caret default — and any `cargo update` picks up 0.1.5. If 0.1.5 adds the rules §5.1 shows the suite needs, that adapter's CI breaks on a day it did not change. Cargo calls it compatible; every adapter author experiences it as breaking. This creates a **permanent incentive not to strengthen the suite**, which is precisely what the project must do continuously for the suite to be worth anything. A conformance suite that cannot get stricter is a badge, not a bar — and the suite is the stated differentiator. Nothing in README, CONTRIBUTING or the RUNBOOK addresses it; `CONTRIBUTING.md:54-64` says how to *write* a rule and nothing about the compatibility of *adding* one. The testkit's version is also locked to the contract's, so the crate that must move fast and the crate that must move slowly move together. | Publish a compatibility policy in the testkit's crate docs **before any adapter ships**: "New rules may appear in any release, including a patch. A conformance suite that cannot get stricter is worthless. Pin exactly (`= \"0.1.3\"`) if you need a stable bar for a release branch, and treat a new failing rule as a bug report about your adapter." Two mechanical supports: a `since = "0.1.4"` tag per rule in the registry macro already scheduled for phase 1, so CI can report "3 new rules since the version you pinned" instead of a bare failure; and give `happenstance-testkit` **its own version key now** rather than `version.workspace = true` — one line, and impossible to separate later. | S | **yes** |
| N2 | Trust artefacts missing, and the strongest signal is invisible | `Cargo.toml:42`; README | `unsafe_code = "forbid"`, verified opted into by all eight members, appears **nowhere in the README**. The MSRV is in an ADR users do not read and is absent from the README. What semver covers is unstated. No SECURITY.md, CODE_OF_CONDUCT.md, CHANGELOG.md, dependabot.yml or issue template. Bus factor is one. An evaluator deciding whether to build a company's event store on this looks for exactly these and finds none, while the single most persuasive fact is buried in a manifest. | A "Guarantees" section above "Prior art": `#![forbid(unsafe_code)]` across every crate, enforced by CI; MSRV 1.85, raised only in a minor and called out in the changelog; semver covers the public APIs of both published crates, and **new conformance rules may appear in any release**; security reports to an address; four runtime dependencies, named. Then the files. Also write the honest bus-factor mitigation the project can already claim: the conformance suite means an adapter outlives the maintainer, and the dual licence means a fork is legitimate. | S | **yes** |
| N3 | Observability: the contract can only standardise **names**, and the port makes the obvious instrumentation measure nothing | `store.rs:117-121`; `lib.rs:74`; `error.rs:86-92` | The contract crate is `no_std`-capable with four dependencies, so it will never take `tracing` — instrumentation lives in each adapter, and left alone every adapter names its spans differently. Worse, because `read` is **not** async and returns the stream at the top level, `#[tracing::instrument]` on it opens a span that closes before a single event is read. An adapter must instrument `poll_next` and must not hold an `Entered` guard across polls. The port makes the classic tracing footgun **mandatory**, and an author who reaches for the attribute macro gets a span that measures nothing and will not notice. | Write *semantic conventions* into the contract docs — prose, zero dependencies, zero API surface: `happenstance.append` with `event_count`, `conditional`, `position`, `condition_violated`; `happenstance.read` **opened on first poll, closed on stream end**, with `query.items`/`query.all` for pushdown debugging. Add the one policy sentence that will otherwise be got wrong everywhere: `ConditionViolated` is the routine DCB outcome (`error.rs:91-92` says so) and belongs at **DEBUG with a counter, never at ERROR** — otherwise adapters page once per retry under contention, which is exactly when the system is working correctly. An hour of writing; makes "production-ready" checkable. | S | no |
| N4 | No panics-vs-errors policy for adapters, and no rule that a store survives a failure | `CONTRIBUTING.md:39-52`; suite (absent) | After `AppendError::Store(e)` from an aborted transaction, is the store usable? Nothing says, nothing tests. A probe found a dropped `ProjectionStore::Batch` leaves the store returning `Busy` forever — the same class, in the other port. Recoverability after failure decides whether a service degrades or dies. | One paragraph in CONTRIBUTING: adapters return `Err`, never panic, for every condition a caller can trigger; panicking is reserved for the adapter's own broken invariants. One rule: `store_is_usable_after_a_failed_append`. | T | no |
| N5 | **The differentiator is scheduled last** | `RUNBOOK.md:36-45`; `happenstance-sync/src/lib.rs:6-11`; ADR-0001; `README.md:15-53, :159-166` | The testkit is a **credibility** moat — it buys the first hundred users among the people right to be sceptical — but not an adoption moat, because nobody's daily work touches it and its audience is one ADR-0006 itself calls "small, and sophisticated". The typed layer is the **adoption** moat and has no credibility without the contract beneath it. Neither is what makes the project *unique*. `happenstance-sync/src/lib.rs:6-11` states the real target plainly: a local-first application holding its own store on-device, syncing with SQLite inside a Cloudflare Durable Object. The `!Send`/`Send` split is the hardest engineering in the repository and exists entirely to serve that. **No other Rust event-sourcing library can run in a Worker at all** — `cqrs-es` still recommends `async_trait`, `disintegrate` is Postgres-bound, `evento` is server-side. DCB is a good bet `disintegrate` has already validated; a conformance suite is good practice anyone can copy; "the event sourcing library for local-first Rust applications that sync" is a position nobody occupies. Yet wasm is phase 5 and replication phase 6 — last and second-to-last — while `README.md:15-53` leads with DCB and names `disintegrate` at :161, competing head-on on its ground, from behind. | A positioning decision, not a code change, but settle it before 0.1 because it re-orders everything. (1) Pull the `!Send`/wasm proof to the front (§7 phase 1) — the ledger row at `RUNBOOK.md:72` already half-decides it; it converts the differentiator from a claim into a demonstration before any adapter ossifies. (2) Make the testkit strong and give it a strengthening policy now (N1). (3) Settle the typed layer before the SQLite adapter. (4) Rewrite the README's lead: **DCB is the mechanism, local-first-that-syncs is the position.** | M | no |

---

## 6. The batteries gap

"Comes with batteries" is a product promise, and `README.md:3-5` makes it on the crate's front page.
`README.md:10-11` does carry a bold status disclaimer, which blunts the first-impression risk — but
the promise still outruns what phase 7 will ship. Here is what it has to mean concretely, and where
each capability stands.

| Capability | Status | Comparable | Recommendation |
|---|---|---|---|
| Storage-agnostic port + conformance suite | **present** | nobody | The credibility moat. Strengthen per §5.1 and give it a compatibility policy (N1). |
| In-memory reference store | **present** | most | Keep. Add `MemoryProjectionStore` so the projection port has an oracle (§3.5). |
| A durable adapter | planned (SQLite) | all | On the critical path. But it should follow the contract freeze, not precede it. |
| Typed events + codec | planned (phase 6) | `disintegrate`, `evento`, `cqrs-es` | Must land in 0.1. Without it the library is a header file. |
| Decision model that **cannot** disagree with its query | planned, shape unsettled | `disintegrate` via `#[derive(StateQuery)]` | **The single most important battery.** A1 is a proven silent-invariant-violation hazard. The requirement is the *decoded fold*, not the macro. |
| Composing decision models | planned (`RUNBOOK.md:236-238`) | PHP reference `CompositeProjection`, `disintegrate` | Implement composition for **tuples** `(P1, P2)`, `(P1, P2, P3)` via a small macro — the trick axum uses for extractor tuples — so N is compile-time and each fragment's `Query` is OR'd into the read automatically. Rust-idiomatic; strictly better than PHP's runtime list. |
| Command loop with retry | planned | nobody has solved it well; the PHP reference has no retry loop and `disintegrate`'s README shows no policy | **Room to lead, not catch-up debt.** Must be the trait-with-synchronous-`decide` shape (A2), because the closure shape provably does not compile. State the retry policy in the docs, not just in code. |
| Derive macro (`happenstance-macros`) | explicitly out of 0.1 (`RUNBOOK.md:233-234`) | `disintegrate` ships it today | Defensible **only if** the decoded fold lands (A1). Then the derive is boilerplate removal, not the correctness mechanism. Make it a stated exit criterion: "if the rewritten `course-subscriptions` carries more mapping boilerplate than domain logic, the macro is in scope for 0.1." Also require `DomainEvent`'s shape to be checked against a hand-written expansion of what the derive would emit, so the trait does not move when the macro lands. |
| given/when/then testing DSL for **application** logic | **absent, unplanned** | `cqrs-es` `TestFramework` | Add to phase 6. Purely additive API in an unpublished crate, so it can slip to 0.2 with zero breakage — but it is the sugar that most directly produces the "delight" the goal names, and `MemoryEventStore` already exists as the seeding target. |
| Fault injection for testing **user** retry paths | **absent, unplanned** | rare | `FaultyStore<S>` in the testkit (fail first N appends, every Nth, adapter error vs violation, fail reads) plus a `GappyMemoryStore` that deliberately leaves position gaps. Roughly a day; makes every user's retry loop testable, and the suite's own atomicity rule writable (C8). Cheapest high-value item in this table. |
| Projection runner | planned; ADR-0006 moves it **into** the contract crate | `evento` ships subscriptions first-class | Needs `head()` (§3.4) to answer "am I caught up?". Polling is fine for 0.1 — measured at 4 µs when nothing is new. |
| Live subscriptions / catch-up tail | **absent, and not even in the ledger** | `evento` | Add a ledger row now — even if the answer is "deferred, phase 6 polls". Adding a required trait method after two adapters implement it is the expensive order to discover this in. |
| Snapshots | absent, unplanned | `cqrs-es` | One ledger row: deferred, revisit if replay cost is measured. Legitimately low priority — DCB queries are narrow by construction — but it must be a recorded "not yet" rather than silence. |
| `tracing` / metrics | **absent, unplanned** | near-checklist in the ecosystem | Purely additive, so it does not get expensive by being deferred. But ship the **semantic conventions** now (N3), because those are what get set by accident. |
| Command-retry idempotency guidance | **absent** — the only idempotency discussion is replication ingest (`RUNBOOK.md:365-366`) and projection idempotence (`projection.rs:29`), neither of which is command retry | — | One paragraph, and it is nearly free: S6 shows the mechanism already exists and is *stronger* than a documented pattern — it is an at-most-once guarantee the project does not know it has. |
| Benchmarks | absent | nobody ships an inheritable one | `event_store_benchmarks!` in the testkit (P4). A differentiator, not just diligence. |
| Schema evolution / upcasting | **absent, unplanned, contract-shaping** | most mature stores | Does `EventType` carry a version suffix? Does an upcaster need a read-path hook the port does not have? Both touch the contract's public surface, so this **cannot** wait until after publish. ADR it alongside `Codec`. |
| Multi-tenancy guidance | absent | nobody treats it as a port concern | Correct to omit from the port; **wrong to omit the sentence** (S10). |

**The honest summary:** happenstance is ahead of the market on the port design and the conformance
suite, and behind `disintegrate` and `evento` on everything an application author touches daily —
and phase 3's planned scope does not close the gap. Either add the missing items to the typed
layer's scope with their own exit criteria, or scope the tagline down to what 0.1 actually ships.
Whichever, list the deferred items explicitly in the ledger rather than omitting them.

---

## 7. Revised runway

The current plan's ordering is what produced most of the `blocksFirstPublish` set: it freezes the
contract by accretion, behind adapters. The revision below inverts that. Phase numbers are new.

| # | Phase | Depends on | Proof artefact — never "the gate is green" |
|---|---|---|---|
| 0 | **Ground clear** (0.5 d) | — | Two reserved crates.io names, the ADR-0006 rename executed with a facade, a publishable `.crate` verified by `cargo package --list` |
| 1 | **The `!Send` proof + ADR-0007** (4 d) | 0 | The conformance suite green on `wasm32` against a `RefCell`-backed `!Send` reference store, off tokio, via the rule registry |
| 2 | **Adapter skeletons — three shapes, no behaviour** (3 d) | 1 | `references/adapter-shapes.md`: a table of the **compiler errors** each rejected signature produced, for rusqlite, Durable Object `SqlStorage` and Ladybug. The evidence base for five later ADRs. |
| 3 | **Freeze `EventStore`** (7 d) | 2 | Six deliberately-wrong stores the suite **catches**. Every semantic sentence paired with a rule. |
| 4 | **Freeze the value types** (5 d, overlaps 3) | 2 | A wire round-trip proptest in `serde_json` **and** `postcard` — the `skip_serializing_if` defect is invisible to the first |
| 5 | **Freeze `ProjectionStore`** (4 d) | 3, 4 | Two structurally unlike batch shapes passing one suite |
| 6 | **Typed layer + worked example** (8 d) | 3, 4, 5 | A `trybuild` compile-**fail** proving a fold that gained an event type the query lacks does not build. A passing test proves nothing here. |
| — | **`0.1.0-alpha.1`** | 6 | — |
| 7 | **`happenstance-sqlite`** (10 d) | 3, 4, 5 | Concurrency suite green + a measured append-condition probe |
| 8 | Cloudflare Durable Object (parallel) | 2, 3 | The suite green under `workerd` |
| 9 | Ladybug projection store (parallel) | 5 | The projection suite green on a third shape |
| 10 | **Publish `0.1.0`** (2 d) | 6, 7 | docs.rs green, `cargo-semver-checks` live against a real baseline |
| 11 | `happenstance-sync` | 7, 8, 10 | Byte-identical payload round trip between two stores |

**Critical path: 0 → 1 → 2 → 3 → 5 → 6 → 7 → 10 ≈ 8 weeks**, against roughly 11–12 for the current
ordering — and that is before counting the rework the current ordering builds in by settling the
contract after two adapters depend on it. Phases 3 and 4 genuinely overlap (different files, one
merge point at 5); 8 and 9 are off the 0.1 path.

**The five structural changes, and why each:**

1. **The `!Send` proof moves from 5 to 1.** ADR-0001 shapes every signature in the workspace and
   admits in its own provisional banner that no `!Send` implementation exists anywhere. Its stated
   lift condition costs four days. Scheduling it fifth means four phases written against a design
   nothing has voted on — and phase 5's own body concedes the fix may be a testkit restructure that
   phases 1–4 already depend on. `RUNBOOK.md:72` already half-decides this; the phase bodies were
   never updated.
2. **A new skeletons phase.** A port is falsified by a **type**, not a behaviour: the projection GAT
   fails on `SendProjectionStore` with E0515 and "`Transaction<'_>` is not `Send`", both visible from
   a skeleton in an afternoon. The current plan buys that same information by finishing two adapters
   across two phases and then freezing the port against the first of them.
3. **Three explicit freeze phases that did not exist.** In the current plan the port's signatures and
   semantics are implicit in "write the SQLite adapter". Phase 4 settles identity and `recorded_at`
   *before* the schema that must hold them; the current plan writes the schema in phase 1 and decides
   what an event carries in phase 3.
4. **The typed layer moves before the flagship adapter.** It is the consumer that discovers contract
   defects and needs only `MemoryEventStore` to do it. ADR-0006 already dissolved its phase-2
   dependency.
5. **Publication is decoupled from replication.** Old phase 7 depended on phase 6
   (`happenstance-sync`). Once `EventId` is on `SequencedEvent`, nothing in replication touches the
   public surface. Removing that dependency takes ~3 weeks off the critical path for free.

**On the alpha.** Cut `0.1.0-alpha.1` after phase 6, not after SQLite. Feedback is only worth having
about something a person can use, and a contract crate plus an in-memory store *is* usable for
writing and testing an application. Waiting for SQLite costs two weeks of not listening for a benefit
the feedback does not depend on. The commitment risk is mitigated by a README stability sentence, a
changelog of what broke between alphas, and yanking superseded alphas — and the party it actually
needs to bind is **you**, because an alpha in the wild makes "we can't change that now" available as
exactly the bias `RUNBOOK.md:414-421` diagnoses and does not act on.

---

## 8. ADRs to write, in order

| ADR | Phase | The single question it settles |
|---|---|---|
| 0007 | 1 | Is the `Send` flavour derived by `trait_variant` or hand-written — given that the variant needs `Sync` for provided methods and `SendEventStore::Error` may need `Send + Sync` while `EventStore::Error` may not, and `trait_variant` copies associated-type bounds verbatim? |
| 0008 | 3 | What does `read` promise about laziness and isolation — when is the store's state sampled, may a held stream observe a concurrent append, and what must never happen regardless? |
| 0009 | 3 | What shape does `append` take, and what are its preconditions — who owns the batch, what an empty batch is, whether a batch's own events can violate its own condition, and what the return value carries? |
| 0010 | 3 | What does a store promise about position assignment and visibility — gaps, reuse across a reopen, and the visibility invariant that makes `AppendCondition::after` sound? |
| 0011 | 3 | What is the conformance suite's own proof obligation — what must it be demonstrated to **fail**, and how are runtime-agnostic, concurrent and durability rules expressed? |
| 0012 | 4 | What does an event carry beyond type, data and tags — identity, `recorded_at`, who assigns them, and **what constructor shape survives adding a field**? |
| 0013 | 4 | How is a validated identifier constructed — const or fallible — and is `Tag` equality byte equality? |
| 0014 | 4 | What is the contract crate's public dependency and wire-format surface — `bytes`, `futures-core`, the serde representation, and which formats the feature supports? |
| 0015 | 5 | What does a projection batch own, what happens when it is dropped, and does the port ship behind `unstable-projection`? |
| 0016 | 6 | How does a decision model guarantee that its query and its fold cannot disagree? |
| 0017 | 6 | How does a payload's shape evolve — codec tag, versioned event types, upcasting, and does the read path need a hook it does not have? |
| 0018 | 6 | Erasure and the append-only log: immutability as an adapter obligation, crypto-shredding as the supported mechanism, and that tags are plaintext and therefore not erasable. |
| 0019 | 7 | SQLite: driver, schema, tag storage, and the append-condition strategy. |
| 0020 | 8 | Cloudflare: the `SqlStorage` mapping and the off-tokio conformance harness. |
| 0021 | 9 | Ladybug: checkpoint placement, how a projection expresses graph mutations, and the blocking API. |
| 0022 | 11 | Replication: does ingest re-check append conditions, what is the merge rule, and what makes re-delivery harmless? |

Scheduled amendments: ADR-0001 loses `provisional` in phase 1 (or is superseded by 0007); ADR-0003
in phase 11; ADR-0004 at publish; ADR-0006 gains an "On the historical record" section in phase 0.

---

## 9. Immediate next actions

Each is small enough to finish in a sitting. In order.

1. **Reserve `happenstance` and `happenstance-core` on crates.io** as `0.0.0` placeholders with a
   description, licence, repository and one-paragraph README. Both are free as of today. One hour.
   Then add a RUNBOOK ledger row marked done so the question stops being reopened.
2. **Execute the ADR-0006 rename**, in its own commit, with nothing else in it. Repoint
   `xtask/src/main.rs:69` (`-p happenstance` → `happenstance-core`) and `.github/workflows/ci.yml:77`
   in the same commit — that is the one place the rename can go wrong silently. Ship the new
   `happenstance` as the five-line facade. Update CLAUDE.md's constraint 2 and repository map.
3. **Close phase 0's clean-tree decision** — gitignore `.idea/`, commit `.mcp.json`. Every phase's
   exit gate references a clean tree that is currently undefined.
4. **Change one line and one bound**: `#[trait_variant::make(SendEventStore: Send + Sync)]` and
   `type Error: core::error::Error + Send + Sync + 'static`. Then add the generic
   `spawns_from_generic_code` test beside `read_stream_is_send`, and write ADR-0007 recording the
   falsification test.
5. **Delete the five `skip_serializing_if` attributes** and add `crates/happenstance-core/tests/wire.rs`
   with `serde_json` + `postcard` round trips for the sparse shapes. This is the one confirmed
   critical defect.
6. **Add the two highest-value conformance rules** — `read_limit_applies_after_filtering` and
   `query_returns_an_event_matching_two_items_once`. Ten lines each; together they catch two of the
   four bugs the suite currently misses.
7. **Fix the trivially-wrong things in one commit**: `SequencePosition::next` → `checked_add`;
   `Event::new` → the `QueryItem::new` pattern plus `From<Infallible> for InvalidEventType`;
   `ReadOptions::limit` → `Option<usize>`; `Query::from_item` → infallible; `Query`'s `Default`
   derive deleted; `#[non_exhaustive]` on `Query::Items`; `pub use futures_core;`;
   `serde = [… "serde/alloc"]`; the four "ASCII control" doc strings; the `take` before `collect` in
   `memory.rs`.
8. **Make the README compile** — `#![cfg_attr(doctest, doc = include_str!("../README.md"))]` plus the
   hidden `main` wrapper — and add the "Guarantees" section naming `forbid(unsafe_code)`, the MSRV,
   and what semver covers.
9. **Write the testkit's compatibility policy** into its crate docs, and give it its own version key
   instead of `version.workspace = true`. One line each; impossible to separate later.
10. **Add the four missing ledger rows** — live subscriptions, snapshots, `tracing`, `recorded_at` —
    even where the answer is "deferred". The RUNBOOK's own rule 3 exists to stop these being settled
    by accident, and these four are currently invisible to it.
