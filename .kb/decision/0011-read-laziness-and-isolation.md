---
id: adr-0011-read-laziness-and-isolation
title: "ADR-0011: A read is one sample with a ceiling, and `&Query` stays"
kind: decision
status: accepted
authority_tier: decision
summary: >-
  What `read` promises about laziness and isolation — when the store's state is sampled,
  and that the items of one `Query` share one sample. Two clauses in scope stay
  provisional. Discharges ES-8, ES-9, ES-11 through ES-16, and VT-26 through VT-31's
  read half.
depends_on:
  - adr-0001-async-port-flavours
related: []
source_paths:
  - docs/architecture/SPECIFICATION.md
last_reviewed: 2026-08-09
adr_id: ADR-0011
phase: 4
supersedes: []
superseded_by: null
---

# ADR-0011: A read is one sample with a ceiling, and `&Query` stays

- **Status:** accepted. Two clauses in scope stay `[PROVISIONAL]` and **two** new
  clauses are minted — one `[PROVISIONAL]` with a phase-12 deadline (no `Unpin`
  bound), one `[FROZEN]` with no falsifier by design (no `ReadOptions::after`).
  Each provisional marker names its falsifier, its instrument and the phase that
  builds it (§*What this ADR leaves open*).
- **Date:** 2026-08-08
- **Settles:** ES-13 (**confirmed**, not superseded), ES-14, ES-16, VT-27's read
  half, VT-28, VT-29, and defects D5 and D7
- **Confirms unchanged, and owes no amendment:** ES-8, ES-9 and ES-15. All three
  are `[FROZEN]`, all three are read-side, all three are inside the scope this ADR
  was told to claim or decline, and nothing decided below changes any of their
  text. They are listed so the next reader knows they were looked at rather than
  missed. ES-9 is not decorative in that list: its `Rejects:` already names the
  empty-store hazard that §3's ceiling requirement walks into, and §6's third
  reason for refusing `ReadOptions::after` rests on it.
- **Explicitly declines:** VT-31. Its residue is a *rule* that is owed
  (`query_union_is_item_concatenation`, named at ES-15's `Rejects:` as the thing
  that would catch an item-sorting adapter) rather than a signature or a
  semantics question, so no ADR in this queue is the right instrument. It belongs
  to whoever writes rules next. **This is the one disposition in this ADR that a
  human should confirm rather than inherit** — declining is cheap and losing the
  rule is not.
- **Leaves provisional, deliberately:** ES-11 and ES-12 — their axis is
  *transport*, which is empty at both ends until phases 9 and 10
- **Extends:** [ADR-0008](0008-one-derivation-for-both-ports.md), whose
  derivation this decision declines to unpick, and
  [ADR-0010](0010-the-suite-must-prove-itself.md), whose "name the wrong
  implementation" obligation every clause below is written against
- **Corrects:** [ADR-0001](0001-async-port-flavours.md)'s consequence paragraph on
  `dynosaur`, which asserts an erasure that does not work as written

## The one question

**What does `read` promise about laziness and isolation — when is the store's
state sampled, and do the items of one `Query` share one sample?**

That is the queue's question (`RUNBOOK.md:284`), and sections 1–5 are it or a
direct consequence of it.

**Sections 6 and 7 are not, and pretending otherwise would be the failure this
repository's own "an ADR that cannot be stated as one question is two ADRs" rule
is written against.** They are a *scope extension*, taken deliberately and on
instruction rather than by drift. The dossier's correction 10 found that the ADR
queue under-covers phase 4 by twenty-nine clause IDs; its correction 11 records
that `ReadOptions::to`, `ReadOptions::after`, `limit(0)`, `EventStoreExt` and the
prelude were filed against this ADR by topic and not by its stated scope, and
warns the drafter to extend deliberately. The ADR-0011 brief then makes it an
obligation: *"It must also claim, or explicitly decline, the twenty-nine
unclaimed IDs nearest it: ES-8, ES-9, ES-14, ES-15, ES-16 and VT-26–VT-31."*
The extension is taken because splitting it produces an ADR-0016 that nobody
scheduled and that would have to be written by the same pass anyway; the cost is
that this document is one question plus a residue, and the residue is fenced into
its own two sections so a later reader can see the seam.

Three things that a topic-based reading would also file here are **not**
answered, and are named rather than smuggled:

- **`Query::Items`' constructibility (VT-26, defect D2)** is a constructor and
  error-composition question of the same family as `Event::new`'s bound. It goes
  to **ADR-0015**, which owns validated constructors, and ADR-0012 must cite it
  because the consequence — a silently unconditional append — lands on
  `AppendCondition`.
- **What `head()` returns on a Postgres adapter that allocates positions outside
  the transaction.** This ADR contributes an argument to it (§*The ceiling*) and
  deliberately does not settle it. It is **ADR-0013**'s, with ES-30 `[FROZEN]`.
- **`AppendCondition`'s guards and boundaries (VT-30).** VT-27's refusal is only
  sound if VT-30 lands, and this ADR states its dependence on that without
  deciding it. **ADR-0012**'s.

## Context

`read` is the only method on the port that returns something other than a value,
and it is the only one whose *shape* is load-bearing for the wasm target:

```rust
fn read(&self, query: &Query, options: ReadOptions)
    -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;   // store.rs:118-122
```

It is deliberately not `async`. ADR-0001 and ADR-0008 both rest on that: with the
stream at the top level of the return type, `#[trait_variant::make]` can mark the
**stream** `Send` on the derived flavour, and `memory.rs:364-391` and
`memory.rs:393-453` are the two tests that make the obligation stick. CLAUDE.md's
third constraint forbids deleting either. Nothing below touches that shape.

Three things are wrong or unsettled today.

**D7 — the port promises a laziness it does not have and does not need.**
`store.rs:104-109` says *"The returned stream is **lazy**: nothing is executed
until it is first polled"*. `MemoryEventStore` filters, orders, truncates and
`clone`s the entire result set under the read lock **at call time**, and streams
from the resulting `Vec` (`memory.rs:155-181`). Both of the store's own doctests
pass. Nothing checks the sentence, and no adapter in or planned for the workspace
delivers the behaviour the sentence advertises as its justification: Neon's read
is one buffered JSON document (`adapter-shapes.md:214`) and `MemoryEventStore`
buffers the whole selection. The one adapter that genuinely *needs* deferral is
`happenstance-sqlite`, for a reason the sentence does not give — `spawn_blocking`
panics with no runtime in scope and `read` is not `async`, so the spawn has to be
deferred into `poll_next` (`adapter-shapes.md:221`).

**The RUNBOOK and ES-13 disagree, and ES-13 is `[FROZEN]`.**
`RUNBOOK.md:2768-2774` says *"`read` takes its query by value"*; ES-13
(`SPECIFICATION.md:2500`) says *"`read` MUST continue to take `query: &Query` by
reference"*, and its `Rejects:` line (`:2530-2532`) names the RUNBOOK's item
verbatim. RUNBOOK rule 5 makes the clause win, so the only question left is
whether this ADR **supersedes** the clause — which is the one thing an edit may
never do.

**ES-11 and ES-12 are the only clauses in scope with real content left, and the
adapters that would test them do not exist.** ES-11 (`:2425`) says a read is one
snapshot; ES-12 (`:2474`) says every item of one `Query` shares it. Both are
`[PROVISIONAL]` on the **transport** axis, and §6.5's portfolio records that
axis as empty at *both* ends.

### Why isolation matters at all when the append condition re-checks

This is the objection worth answering first, because DCB does not obviously need
snapshot reads: a handler reads, decides, and appends with a condition, and the
condition is re-evaluated at the store. If a read tore, surely the condition
catches it?

It does not, and the reason is that the caller derives the condition's boundary
**from the read**. `read_decision_model` (`store.rs:205-215`) returns the maximum
position it observed, and `AppendCondition::after_opt` consumes it. A torn read
returns a maximum position *above* an event it silently missed. The condition
then says "reject if anything matched after *P*" — and the missed event is at a
position below *P*, so it is precisely what the boundary tells the store to
ignore. **A torn read does not turn into a rejected append; it turns into an
accepted one.** That is ES-12's "quieter failure mode" made concrete, and it is
why isolation is a contract obligation rather than an adapter's quality of
implementation.

Two further reasons, both structural. A decision model assembled from two states
can be internally inconsistent in a way no condition can express — the condition
checks that nothing *new* arrived, not that the fold was taken against one state.
And read-only consumers — projection runners, replays, exports — never append at
all, so for them the read is the *only* isolation there is and nothing downstream
re-checks anything.

## What was compiled for this decision

The dossier ([`docs/evaluation/phase-4-reconciliation.md`](../evaluation/phase-4-reconciliation.md))
carries nine experiments. This ADR leans on E1, E2, E3, E5 and E9 as recorded
there, and adds three of its own — E10, E11 and E12 — all run against
**unmodified** `happenstance-core` on rustc 1.97.1 (the pinned toolchain,
ADR-0029), from a scratch crate with a path dependency. Nothing under `crates/`
was touched.

**Provenance, because it is not uniform and the dossier's correction 14 says to
say so.** E1, E2 and E4 were recompiled for the dossier. **E3, E5, E6, E7, E8 and
E9 are scout transcripts that were not re-run**, so where this ADR leans on E3
(by-value cost) and E9 (the ext trait's coherence ceiling) it is leaning on a
recorded diagnostic rather than on a compile anyone repeated. E5's *diagnostic*
is a transcript; the conclusion drawn from it is refuted below by E11, which was
run. E10, E11 and E12 were run this pass.

### E10 — do the escaping cases compile under `&Query` today? **Yes, when the caller owns the query.**

`RUNBOOK.md:2990-2996` asserts that returning a `read` stream from a function and
storing one in a struct field are impossible under today's signature. Both
compile, and both **run** against `MemoryEventStore`:

```rust
pub fn replay<'a, S: EventStore>(store: &'a S, query: &'a Query)
    -> impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a
{
    store.read(query, ReadOptions::new())
}

pub fn open_replay<'a, S: EventStore>(store: &'a S, query: &'a Query)
    -> Replay<impl Stream<Item = Result<SequencedEvent, S::Error>> + 'a>
{
    Replay { stream: store.read(query, ReadOptions::new()) }
}

pub struct BoxedReplay<'a, E> {
    pub stream: Pin<Box<dyn Stream<Item = Result<SequencedEvent, E>> + 'a>>,
}
```

`test tests::case_one_and_two_run ... ok` — three shapes, one function return,
one generic struct field, one *named* struct with an explicit lifetime.

**The distinction the specification and the RUNBOOK both miss is local versus
parameter, not temporary versus named.** ES-13's own remedy (`:2504-2509`) says
the fix is "binding the query to a named local". For the two escaping cases that
is compiled to be *wrong*: a named local dies at the end of the function that
declared it, so a stream escaping that function still outlives its query. Three
distinct diagnostics, one per arrangement, all reproduced this pass:

| Arrangement | Diagnostic |
|---|---|
| `store.read(&Query::all(), ..)` bound to a `let` | `error[E0716]: temporary value dropped while borrowed` |
| named local, stream returned, return type **bare** `-> impl Stream<…>` | `error[E0597]: 'query' does not live long enough` |
| named local, stream returned, return type `+ '_` or `+ 'a` | `error[E0515]: cannot return value referencing local variable 'query'` |
| named local, stream returned inside a struct, **bare** | `error[E0597]` |
| named local, stream written into a slot that outlives the call | `error[E0597]` |

**Neither this ADR's first draft nor dossier correction 8 had this right, and the
disagreement between them was settled by compiling all four arrangements a third
time.** Correction 8 recorded the return form as `error[E0597]`; this ADR's
review recorded it as `error[E0515]` and called correction 8 wrong. Each had
compiled one spelling and generalised from it. Both codes are real:

```
error[E0515]: cannot return value referencing local variable `query`
 |     store.read(&query, ReadOptions::new())
 |     ^^^^^^^^^^^------^^^^^^^^^^^^^^^^^^^^^
 |     |          `query` is borrowed here
 |     returns a value referencing data owned by the current function
```

**Why the same defect reports under two codes, since this is the Rust-specific
point and not a clerical one.** With no lifetime bound on the opaque type, the
compiler reasons from the *borrow*: `&query` must outlive the returned opaque
type, and `query` drops at the end of the function — `E0597`. Write `+ 'a` and
the opaque type is now *required* to live for `'a`, so the compiler reasons from
the *value* instead and reports that what you are returning references a local —
`E0515`. The cause is identical in both: RPITIT's opaque type captures the
`&Query` lifetime whether or not you mention a lifetime in the signature.

**The consequence for the amendment below is that ES-13 must name the defect,
not a code.** A clause pinning one error code documents one spelling of one call
site, and the next reader who writes the other spelling concludes the clause is
wrong.

`E0597` is real but belongs to a *third* arrangement the dossier did not
separate: writing the stream into a slot that already outlives the call, where
the borrow must satisfy a caller-named region rather than escape a return. So
correction 8's substance survives — `ES-13:2504-2517`'s single `E0716` citation
is inadequate — and its arithmetic does not. Three arrangements, three codes, and
the spec amendment names all three.

### E11 — can the port be erased behind `dyn` today? **Yes, by hand, with no `unsafe` and no bound on `read`.**

The dossier's E5 established that `dynosaur` cannot erase the port:
`error[E0277]: dyn Stream cannot be unpinned`, because the generated wrapper
produces `Box<dyn Stream>` and `Box<T>` is only a `Stream` when `T` is `Unpin`.
The conclusion drawn from it — that `+ Unpin` must therefore join `read`'s return
type — does not follow. A hand-written object-safe trait with a blanket impl over
`EventStore` compiles and round-trips at runtime:

```rust
pub trait DynEventStore {
    fn read_dyn<'a>(&'a self, query: &'a Query, options: ReadOptions)
        -> Pin<Box<dyn Stream<Item = Result<SequencedEvent, ErasedError>> + 'a>>;
    fn append_dyn<'a>(&'a self, events: &'a [Event], condition: Option<&'a AppendCondition>)
        -> Pin<Box<dyn Future<Output = Result<SequencePosition, AppendError<ErasedError>>> + 'a>>;
}
impl<S: EventStore> DynEventStore for S { /* Box::pin at both sites */ }
```

`erased_store_round_trips ok`, driven through `&dyn DynEventStore` against
`MemoryEventStore`. Three Rust facts are doing the work, and they are worth
spelling out because they are the crux of the `Unpin` decision.

`Pin<Box<dyn Stream + 'a>>` is itself a `Stream`, via the standard blanket
`impl<P> Stream for Pin<P> where P: DerefMut, P::Target: Stream` — which is why
boxing is sufficient and no wrapper type is needed. `Pin<Box<T>>` is `Unpin`
**whatever `T` is**, because pinning is a property of the pointer's promise and
not of the pointee, so boxing-and-pinning an arbitrarily self-referential stream
produces one that satisfies every `Unpin` obligation downstream. And because the
result is `Unpin`, the error-mapping adapter over it needs no pin projection and
therefore **no `unsafe`**, which matters in a workspace that sets
`unsafe_code = "forbid"` (`Cargo.toml:79`).

One incidental finding, recorded because it will bite whoever writes this
wrapper: `AppendError` is `#[non_exhaustive]`, so the error-mapping `match` needs
a wildcard arm. That is a property of the wrapper, not of the port.

So the erasure `ADR-0001:110-112` promises is available; it is fifteen lines of
downstream code rather than a macro, and it costs the port nothing.

### E12 — does `+ Unpin` forbid a generator-backed stream? **Yes, compiled.**

This was the one load-bearing claim in §5 that no experiment settled — the
dossier's E5 asserts it ("it forbids a self-referential generator-backed stream,
which is a real axis to name") without compiling it, and an assertion is not
evidence in this repository. Compiled this pass. Declaring a chunked read as
`-> impl Stream<…> + Unpin` and building it with `async_stream::stream!`:

```
error[E0277]: `{async block@…/async-stream-0.3.6/src/lib.rs:193:9}` cannot be unpinned
     = note: required for `AsyncStream<Result<SequencedEvent, …>, …>` to implement `Unpin`
     = note: consider using the `pin!` macro
```

The generator's state machine holds a borrow across its own yield points, which
is exactly what `!Unpin` is for. So the cost §5 charges against `+ Unpin` is a
compiled cost and not a predicted one.

## Decision

### 1. ES-13 stands. `read` keeps `query: &Query`, and the clause is confirmed rather than superseded

No frozen clause is overturned by this ADR.

The Rust mechanism, for the reader who is new to it: `-> impl Trait` in return
position is an **opaque type**. The compiler knows the concrete type; the caller
knows only the bounds — and the opaque type *captures* every generic parameter
and lifetime in scope, including the one behind `&query`. Capture is not "the
value is stored"; it is "the compiler must assume the value might be", so the
returned stream is treated as borrowing the query for as long as it lives. That
is why the query has to outlive the stream, and it is the whole of the
difficulty.

One correction worth making because it is the usual mis-statement: this is **not**
an edition-2024 change. Return-position `impl Trait` *in a trait* (RPITIT) has
captured everything in scope since it stabilised; what edition 2024 changed is
free-function RPIT, which used to capture only the lifetimes it named and now
behaves the same way. `read` is RPITIT, so nothing about its capture behaviour
would be different on an older edition, and no edition bump escapes it.

The remedy is ordinary ownership discipline: whoever owns the stream's lifetime
owns the query. For a stream that stays inside one function that is a local; for
a stream that escapes it is a parameter (E10). Nothing about that is exotic —
it is the same rule that governs `std::slice::Iter`, and no one proposes that
`[T]::iter` should take the slice by value.

The two alternatives lost for reasons recorded in full under *Alternatives
rejected*; in one line each: **precise capturing** would require superseding
ES-1, a *different* frozen clause, to protect ES-13; **by value** would supersede
ES-13 to buy something E10 shows is already available.

### 2. The promise is the sampling instant, not laziness (D7)

`read` MUST be evaluated against one state of the store, sampled **no later than
the first poll** of the returned stream. Whether the adapter does its work at
call time or defers it to the first poll is the adapter's business and both are
conformant.

Three consequences the port must state and today does not:

- **Laziness is permitted, never required.** `happenstance-sqlite` needs it and
  `MemoryEventStore` does not have it, and both are conformant. The sentence at
  `store.rs:104-109` is corrected to say so.
- **Building a stream and never polling it is not guaranteed to be free.** A
  caller may not treat `read` as a cheap way to describe an intention.
- **Events appended between the call and the first poll may or may not appear**,
  and a caller MUST NOT depend on either. This is exactly why ES-11's rule polls
  once *before* it appends: the single leading poll is what makes the rule
  portable across a call-time sampler and a first-poll sampler, and a rule that
  appended before the first poll would fail conformant adapters at random. That
  sentence belongs in the rule's own documentation, because it looks like an
  incidental detail and is the reason the rule works at all.

### 3. The ceiling — one mechanism discharges ES-11 and ES-12 across all three store shapes

ES-11 and ES-12 have been argued against two stores. There are three shapes, and
the third is what turns the answer into a statement about a *class* rather than a
choice between the two that happen to exist:

| Shape | Instrument | Sample | What it costs |
|---|---|---|---|
| **Snapshot under a lock** | `MemoryEventStore` (`memory.rs:155-181`); `happenstance-sqlite` behind its `Mutex` (`adapter-shapes.md:221-222`) | one statement, taken **at call time or at the first poll** — `MemoryEventStore` samples at call time, `happenstance-sqlite` must defer into `poll_next`, and §2 is what makes both conformant | satisfies ES-11 and ES-12 for free, and buffers the entire result set — the "million-event replay without buffering" the docstring advertises is delivered by nothing in the tree |
| **Chunked cursor** | `happenstance-postgres`, whose recorded stream type is `PgReadStream` — a pooled cursor (`adapter-shapes.md:46`); a Durable Object's `SqlStorageCursor` | one cursor held across polls | Cloudflare documents that a cursor held across an `await` gives **no stable snapshot** (`adapter-shapes.md:219`), so this shape can fail ES-11 without doing anything unusual |
| **One buffered response body** | `happenstance-neon` — no cursor, a 64 MiB hard cap (`adapter-shapes.md:214-215`), exactly one round trip per operation (`:217`) | one statement per round trip | ES-11 for free *if the result fits*; above the cap the read fails as a transport error rather than yielding fewer items, and self-pagination is the only way out |

**The mechanism.** An adapter that issues more than one statement per `read` MUST
capture a **position ceiling** *H* no later than the first poll, and MUST bound
every statement after the first by `position <= H` (or `>= H` under `backwards`).
That single rule discharges both clauses on all three shapes:

- Shape 1 satisfies it vacuously: one statement, one sample.
- Shape 2 satisfies it by page bound, and **must**, because its cursor is not a
  snapshot on the one `!Send` target already chosen.
- Shape 3 satisfies it the same way, at the price of one extra round trip when
  the head is not returned alongside the first page — which is the price ES-11
  already names, and the reason ES-30 puts `head()` on the port rather than
  leaving it an inherent method.

**The mechanism has one already-documented failure mode and it is worth carrying
here rather than leaving it two clauses away.** ES-9 `[FROZEN]` (`:2340-2356`)
already names the adapter this ADR is now *requiring*: on an empty store
`head()` is `None` (ES-30), and a ceiling computed as arithmetic on that bound
errors or panics on a store whose only fault is being new — the state every
adapter is in on its first run. `reading_an_empty_store_yields_nothing` is the
rule, `NullHeadPagingStore` is the registered failing adapter, and both already
exist. Making the ceiling normative at ES-11 therefore does not create a new
hole; it points a second clause at a hole ES-9 already covers, and the ES-11
amendment below cites ES-9 so the next paginating adapter's author finds the
warning at the clause that told them to paginate.

**ES-12 is the same mechanism, not a second one, and this is the part nobody had
written down.** ES-12's tear is an event matching item 1 that lands between the
statement for item 1 and the statement for item 4. Under a ceiling captured at
the first poll, that event's position is above *H* and is excluded by the same
predicate — *provided nothing below H can become visible later*, which is exactly
ES-10. So:

> ES-11 and ES-12 reduce to **ES-10 plus a ceiling**. Neither is implementable on
> any multi-statement adapter without ES-10, and both are nearly free with it.

That is the load-bearing dependency of this ADR, and it runs the other way from
how the specification is ordered. Had ES-10 fallen at phase 2, ES-11 and ES-12
would not have become expensive — they would have become **unimplementable**
except on shape 1, and the port would have had to choose between snapshot reads
and any transport that is not a lock.

**What this contributes to ADR-0013's open question, without deciding it.** The
ceiling wants a position *P* such that everything at or below *P* that will ever
be visible is visible now. That is the **frontier** an `xid8` + `pg_snapshot_xmin`
adapter reports — not `max(position)` over committed rows, which is exactly the
value that can have an in-flight transaction underneath it. So ES-11's prescribed
mechanism is an argument *for* the frontier reading of "the highest visible
position", and the collision the dossier records is not in ES-30's **sentence**
but in its named **rule**: `head_is_the_highest_visible_position`, written as
"append, then assert `head()` equals the position `append` returned", tests
read-your-own-writes, which is strictly stronger than the clause states and which
arm C does not provide (0.688 ms with no holder; 4010.719 ms behind an unrelated
five-second write). Diagnosis recorded; disposition is ADR-0013's, and this ADR
does not write it.

### 4. ES-11 and ES-12 stay `[PROVISIONAL]`, with sharper falsifiers and one unblocked rule

Phase 4 may not lift them. The axis is transport and both ends are empty: the
only in-tree implementations are shape 1, so nothing here votes for or against
the clauses, and CF-25 forbids treating that silence as evidence. What phase 4
*can* do is make the falsifier cheap to evaluate when the instrument arrives:

- **ES-11's falsifier** is the first one-shot-HTTP adapter that cannot capture a
  ceiling within its round-trip budget — `happenstance-neon`, **phase 10** — or a
  cursor-based adapter that cannot hold a stable snapshot across an `await` and
  cannot afford the ceiling either — the Durable Object, **phase 9**.
- **ES-12's falsifier** is the same adapter emitting one statement per
  `QueryItem`, at the same phases.

**ES-12's rule is not blocked, and the dossier's account of why it was should be
corrected rather than inherited.** It is recorded as needing "a hostile fixture
that can be paused *between statements*, which nothing in the tree can do". That
is true of a *real* adapter and irrelevant to the rule, because the pause points
do not have to come from the fixture: a mutant store's own `Stream` impl supplies
them at its `poll_next` boundaries. A mutant that evaluates one query item per
poll, against freshly sampled state, tears on demand and deterministically — the
identical technique `nothing_below_an_observed_position_appears_later` already
uses on two `append` futures driven one poll at a time (`suite.rs:2778-2795`,
over the shared `poll_once` helper at `suite.rs:2838-2850`; CF-13 is the clause
that obliges that rule to exist). The portable rule is then ES-11's rule with a different
query and a later-matching append: build, poll once, append an event matching a
*late* item, drain, assert the drained set is the pre-append set. It passes on
every shape-1 adapter and on any adapter that implements the ceiling, and fails
the mutant. Whether a `Stream` impl can express "one item per poll" cleanly is a
compile the code run owes; nothing about it looks hard, and nothing about it is
asserted here.

### 5. `read`'s return type takes no `Unpin` bound

`+ Unpin` is **not** added, and this is a decision rather than a deferral: adding
a bound to an opaque return type later is a new obligation on every adapter and
therefore breaking, so it cannot be left to a cleanup. It is decided here or it is
decided by whoever notices it after publish.

What `+ Unpin` would buy is `dynosaur`, and E11 compiled that the same erasure is
available by hand today. What it would cost is real, permanent, and **compiled**
(E12): an `Unpin` bound forbids a **self-referential** stream, and
`async_stream::stream!` produces exactly one — `error[E0277]: … cannot be
unpinned`, with the note that `AsyncStream<…>` does not implement `Unpin`. A
generator is the natural way to write the chunked-cursor read of shape 2 — the
shape with no implementation in the tree, on the axis that is empty at both ends.
Freezing a bound that forbids the obvious implementation of the adapter nobody
has built yet is the exact mistake CLAUDE.md's "check that something in the
workspace sits at the other end of it" is written to prevent.

This mints a new clause (see *Amendments*), `[PROVISIONAL]` with a deadline
rather than a phase, and it **must be re-evaluated before phase 12**, because
after first publish the bound cannot be added at all.

**Its falsifier, stated so it can be recognised rather than merely satisfied.**
The wrapper of E11 fails for one identifiable reason: it must be able to *name*
and *box* every method's return. So the consumer that falsifies this clause is a
consumer needing `dyn EventStore` where the port has acquired a method the
wrapper cannot box — a **generic** method (which is not dyn-compatible and which
`dynosaur` also cannot erase — the RUNBOOK already names this as the cost of
`append(impl IntoIterator<Item = Event>)`, `RUNBOOK.md:2764-2766`), or a return
type whose lifetime the wrapper cannot spell. "Someone found boxing inconvenient"
is not the falsifier and must not be read as one.

### 6. `ReadOptions` gains `to`, gains `Option<usize>`, and does **not** gain `after`

**`to` (ES-16, VT-29, both `[FROZEN]`, inclusive).** It lands at phase 4 because
`ReadOptions` is `#[non_exhaustive]` and passed by value, so the field is not a
trait signature change but *is* a new obligation on every adapter, and no adapter
has shipped. Beyond transcription this ADR adds one thing: **`to` is the
caller-side spelling of the ceiling**. A caller replaying a large log in chunks
with explicit `from`/`to` windows is doing by hand what §3 requires a paginating
adapter to do internally, and the two must not be confused. One `read` is one
sample; N chunked reads are N samples, and what makes stitching them sound is not
ES-11 but ES-10 — nothing in the contract makes two `read` calls one snapshot,
and a reader who assumes otherwise has assumed the one thing this ADR most wants
to deny.

**`limit: Option<usize>`, `limit(0)` yields nothing (VT-28 `[FROZEN]`, defect
D5).** Today `limit` takes a `usize` and stores `NonZeroUsize::new(limit)`
(`query.rs:260-266`), so zero becomes `None` — unlimited — before any adapter
sees it, and a paging loop writing `.limit(budget - fetched)` that reaches parity
reads the entire log.

For the Rust reader, the type question is the interesting half.
`Option<NonZeroUsize>` is the same size as `usize`, because `NonZeroUsize` has a
*niche* — a bit pattern (zero) the type promises never to hold — and the compiler
spends that pattern on `None`. `Option<usize>` has no niche to spend and costs a
word more. So the current type is not an accident; it is the cheap one, and
`limit(NonZeroUsize)` would be the type-safe one. Both lose to the same argument:
the caller who legitimately computed zero is exactly the caller who must not be
made to special-case it, and a limit of zero has an obvious correct meaning.
Eight bytes on a `Copy` struct is the price.

**This is a deliberate divergence from the DCB reference implementation**, which
treats `limit: 0` as unlimited through JavaScript falsiness, and VT-28 requires
this ADR to say so rather than to present it as fixing someone's bug. It is said
here: the reference implementation's behaviour is a coherent reading of `0` in a
language where `0` is falsy; happenstance is not written in that language, and
matches SQL `LIMIT 0` and every paging API instead.

**`ReadOptions::after` is declined.** `RUNBOOK.md:2792-2797` asks for an exclusive
lower bound so that resuming a projection is `after_opt(checkpoint)` with no
arithmetic. It has no clause and is refused for three reasons:

1. **Two lower bounds on one struct is the failure ES-16 already names.** With
   both `from` (inclusive) and `after` (exclusive) present, `Option` each, on a
   `#[non_exhaustive]` struct with public fields and a derived `Default`, an
   adapter must invent a precedence rule for `from(5).after(7)`, and two
   conformant adapters will invent different ones. ES-16's `Rejects:` line
   already predicts the shape: "an adapter that accepts `ReadOptions` by value,
   matches on the fields it knows and ignores the rest".
2. **The asymmetry is deliberate and belongs where it is.** ES-26 `[FROZEN]`
   settles that `AppendCondition::after` is exclusive *because* it means
   "everything I did not see", while `ReadOptions::from` is inclusive because it
   means "start where I stopped". Putting both conventions on the same struct
   destroys the distinction that clause exists to draw.
3. **The idiom it replaces is frozen and sound.** VT-13 `[FROZEN]` names
   `checkpoint.next()` as the documented resume idiom and fixes it to signal
   overflow; ES-9 `[FROZEN]` makes it sound over gaps, because `from` is a range
   predicate rather than a seek, so resuming at an unoccupied position yields the
   higher neighbour rather than an error. `next()` is not the caller doing
   arithmetic on an opaque key; it is the one named method on the position type
   whose entire purpose is this.

The refusal is minted as a clause rather than left as silence, so that the next
reader finds an answer instead of an omission — and it is minted **`[FROZEN]`**,
not provisional, because the whole of its argument is available now: two lower
bounds with opposite inclusivity on one `#[non_exhaustive]` struct is a
precedence hole, and the idiom it would replace is itself frozen at VT-13 and
ES-9. Nothing an adapter or a runner could later measure bears on that.

So it has no falsifier, and this is deliberate rather than an omission. What
would reopen it is what reopens any frozen clause: a **new ADR**. The thing that
would justify writing one is concrete and worth naming so the next reader does
not have to invent it — a projection runner (**phase 7**, which builds it, or
**phase 6**, which freezes `ProjectionStore` under it) that cannot express its
resume through `checkpoint.next()`. That is a trigger for an ADR, not a marker
falsifier, and the two must not be confused: writing it into the clause as a
falsifier would make a `[FROZEN]` clause carry a provisional's furniture.

### 7. No `EventStoreExt` and no `prelude` at 0.1

`RUNBOOK.md:2775-2778` asks for a blanket extension trait on the
`Iterator`/`Itertools` pattern. Neither it nor a `happenstance_core::prelude` has
a clause, and both are declined here.

The decisive argument is that **an ext trait is the one surface that does not
need a freeze phase**. Its own justification in the RUNBOOK — "can be added later
non-breakingly" — is the reason to add it later: a freeze spends its authority on
decisions that become expensive, and this one does not.

Two supporting facts, one compiled. ES-30 `[FROZEN]` already moved the only
methods anyone proposed for it — `head()` — onto the port as a **required**
method, and settled that `count()` does not ship at all, so the trait as specified
would be empty. And the dossier's E9 recorded its ceiling — a scout transcript,
not re-run this pass, and the one place in this ADR where a conclusion rests on a
diagnostic nobody repeated: a blanket ext impl does *not* collide with
`trait_variant`'s (no `E0119` — they implement different traits, so there is
nothing to overlap), but it also cannot be overridden by
anybody. `error[E0119]` inside the crate that owns the trait, `error[E0117]` —
the orphan rule — from anywhere else. Coherence is refusing the same thing twice:
Rust permits at most one impl of a trait for a type, and a blanket impl has
already claimed every type. An adapter with a real `SELECT max(position)` fast
path therefore cannot override the ext method; it must add an **inherent** method
of the same name, which then silently shadows the trait method wherever the
concrete type is in scope and silently does not wherever the code is generic.
That is a hazard worth taking on for a method that earns it, and there is no such
method today.

Free functions already occupy the niche without the hazard — `collect` and
`read_decision_model` are both free functions in `store.rs` — and a free function
cannot be shadowed by an inherent method.

## Consequences

**Good.** Nothing in `read`'s signature changes, so no adapter signature changes,
`docs/adapter-shapes.md`'s recorded stream types stay valid, and ADR-0001's and
ADR-0008's derivation is untouched. The freeze is a freeze rather than a rewrite.

**Good.** ES-11 and ES-12 acquire an implementation strategy that is stated once
and works on three shapes, including the one that cannot stream at all. Three
things about that are new and it is worth being exact, because ES-11 already
carries the mechanism: "capture the head at the first poll and bound every
subsequent page by `position <= H`" is written at `SPECIFICATION.md:2441-2450`
today, inside a justification paragraph rather than in a MUST. What this ADR adds is that the sentence becomes **normative** rather than
advice in a justification paragraph; that **ES-12 is discharged by the same
ceiling** rather than by a second mechanism, which nobody had written down; and
that the argument is made against a **third** shape — the buffered single
response body, which cannot stream at all and which neither clause considered.

**Good.** The `Unpin`/erasure question is answered with a compile instead of an
assertion, and ADR-0001's false claim is corrected in the same change.

**Bad, and this is the honest cost.** Phase 4's proof artefact loses its stated
purpose. `RUNBOOK.md:3004-3005` says of its four cases *"each fails against the
current signatures. That is the whole point."* Case 3 is already green in the gate
as `spawns_from_generic`; case 4 compiles **and runs** (dossier E4); and E10
compiles cases 1 and 2. All four are green against the pre-freeze signatures.
`frozen_signatures.rs` is therefore a **regression pin** — it fails if someone
later makes `read` `async`, or adds a bound that ties the stream to the call —
and not a demonstration of change. Exit criterion 3 is unsatisfiable as written
and the amendment is recorded below. The genuinely red-today content phase 4 owns
for `read` is elsewhere and is better: `read_to_is_inclusive`,
`read_from_and_to_bound_a_closed_window` and
`read_to_under_backwards_bounds_the_older_end` cannot be written because the field
does not exist, and `read_limit_zero_yields_nothing` cannot be **expressed**,
because `limit(0)` is discarded by the builder before any adapter sees it.

**Bad.** ES-13 keeps a shape that costs its caller one parameter. A consumer that
wants to hand out a stream must also hand out the query, or own both in the same
struct. E10 shows all three spellings compile; none of them is free of thought.

**Bad.** Declining `+ Unpin` leaves a semver-shaped obligation on a later phase.
If phase 7 or phase 13 discovers it needs `dyn EventStore` in a form the boxing
wrapper cannot give, the bound has to be added *before* phase 12 or never. This
ADR sets the deadline; it cannot discharge it.

**Neutral.** ES-11 and ES-12 stay provisional, so phase 4 freezes `read`'s
signature while two of its semantic clauses remain open. That is the correct
shape rather than an embarrassment: the signature question and the isolation
question have different evidence bases, and the second one's evidence is built at
phases 9 and 10. Saying so is what CF-25 asks for.

## Alternatives rejected

- **Precise capturing: `fn read<'a>(&'a self, query: &Query, options: ReadOptions)
  -> impl Stream<…> + use<'a, Self>`.** `use<…>` is the syntax that *narrows* an
  opaque type's capture set — it says "this hidden type may mention only these
  parameters", which would drop the query's lifetime and let the stream outlive
  it. On a plain trait it works (dossier E2). It lost on four counts, any one of
  which is sufficient. **It requires superseding a different frozen clause:**
  `trait_variant` cannot emit it — `error[E0799]: 'Self' can't be captured in
  'use<...>' precise captures list, since it is an alias`, plus *"impl Trait must
  mention all type parameters in scope in use<...>"* — reproduced twice
  independently and reproduced **identically by a hand-written blanket impl**, so
  it is structural to any blanket-impl derivation rather than a bug in the macro.
  Escaping it means deleting `#[trait_variant::make]` from `store.rs` and
  hand-writing `SendEventStore` and the blanket impl, and ES-1 (`:2017-2018`) is
  `[FROZEN]` on *"the `Send` flavour MUST be derived by
  `#[trait_variant::make(…)]` rather than hand-written"*. Protecting ES-13 by
  superseding ES-1 is not a saving. **The obvious spelling is wrong:** `use<Self>`
  alone drops the `&self` lifetime too and makes every borrowing adapter
  impossible (`error[E0700]`); the correct form names both. **It taxes every
  adapter:** an impl whose hidden type captures less than the trait's declaration
  permits trips `refining_impl_trait_reachable`, which under this workspace's
  `-D warnings` is an error, so every non-borrowing adapter must write `use<'a>`
  it does not otherwise need. And **E10 leaves it nothing to buy.**

- **`read(query: Query)` by value, superseding ES-13.** It compiles under real
  `trait_variant` with both `memory.rs` tests passing (dossier E3), so it is a
  live option rather than a straw man. It lost on three counts. It supersedes a
  `[FROZEN]` clause whose `Rejects:` line names it verbatim, to buy escaping
  streams that E10 shows are already available. The clone is not free where it
  matters: `size_of::<Query>()` is 16, `Query::all().clone()` costs nothing — so a
  benchmark written against `Query::all()` measures nothing — and a realistic
  two-clause query costs **9 allocations / 212 bytes per read**. And it does not
  remove the clone, it *relocates* it: `error[E0308]` lands at every caller
  holding only a `&Query`, including `read_decision_model` (`store.rs:205-215`)
  and every rule that reads the same query twice, so the clone moves from
  zero-or-one sites to one per read with no caller opt-out.

- **Rewriting proof cases 1 and 2 to bind a named local**, the dossier's second
  option. Compiled to be insufficient: a local does not outlive the function, so
  the escaping cases fail with `E0515` and `E0597`. The correct rewrite is to a
  *parameter*, which is what this ADR adopts, and it keeps all four cases instead
  of costing two.

- **Adding `+ Unpin` to `read`'s return type.** Rejected under §5: it buys an
  erasure that E11 provides by hand and permanently forbids a generator-backed
  stream on the one adapter shape with no implementation in the tree.

- **Making laziness mandatory** — requiring that no work happen before the first
  poll. It would make `MemoryEventStore` non-conformant, and it is unobservable
  through the port anyway: nothing in the contract can distinguish "sampled at
  call time" from "sampled at the first poll" except by appending in between,
  which is precisely the window this ADR declares unspecified. A MUST no rule can
  check is the decorative-rule failure with the arrow reversed.

- **Making the sample instant strictly "call time"** — the mirror image, and it
  is worse. It forbids `happenstance-sqlite`'s deferred `spawn_blocking`, which
  is not a stylistic choice: `spawn_blocking` panics with no runtime in scope, and
  `read` is not `async`, so the work must move into `poll_next` or the adapter
  cannot exist.

- **`ReadOptions::after`**, declined under §6 with its three reasons. The refusal
  is minted `[FROZEN]` and carries **no falsifier by design**; what would reopen
  it is a new ADR, triggered by a phase 6/7 runner that cannot resume through
  `checkpoint.next()`.

- **`EventStoreExt` and a `prelude`**, declined under §7. The one-line version:
  they are additive, so a freeze phase is the wrong place to spend authority on
  them, and E9 recorded — on a scout transcript, not re-run — that the ext trait's
  methods could never be overridden by the adapters that would most want to.

## What this ADR leaves open, and who closes it

| Question | Marker | Falsifier | Owner |
|---|---|---|---|
| Is a read one snapshot on a transport that cannot hold one? (ES-11) | `[PROVISIONAL]` | a one-shot-HTTP adapter that cannot capture a ceiling in its round-trip budget; a cursor adapter that can hold neither a snapshot nor a ceiling | **phase 10** (`happenstance-neon`), **phase 9** (Durable Object) |
| Do all items of one query share it? (ES-12) | `[PROVISIONAL]` | the same adapter emitting one statement per `QueryItem` | phases 9, 10 |
| Does `read`'s return type need `+ Unpin`? (new clause) | `[PROVISIONAL]`, **deadline phase 12** | a consumer needing `dyn EventStore` where the port has grown a method the boxing wrapper of E11 cannot box — a generic method, or a return whose lifetime it cannot name. Not "boxing is inconvenient" | any phase before 12; the bound is unaddable afterwards |
| What does `head()` mean where positions are allocated outside the transaction? | ES-30 `[FROZEN]`; its **rule** is the problem | — | **ADR-0013** |
| Is `Query::Items` constructible downstream? (VT-26, D2) | `[FROZEN]`, unimplemented | — | **ADR-0015** |
| Does an `AppendCondition` carry one boundary or one per guard? (VT-30) | `[PROVISIONAL]` | — | **ADR-0012**; VT-27's refusal is sound only if it lands |
| Who writes `query_union_is_item_concatenation`? (VT-31) | `[FROZEN]`, rule owed | — | **Nobody yet — declined by this ADR.** The residue is a rule, not a signature, so no ADR in this queue is the instrument. **A human should confirm this rather than inherit it**; ES-15's `Rejects:` names the adapter it would catch (one that sorts and dedups query *items*) and nothing else in the suite catches it |

## Amendments this decision owes the specification

Recorded rather than applied, because five of the nine touch `[FROZEN]` clauses
and this repository's rule is that a frozen clause changes by ADR and not by
edit. **This ADR is the authority for every frozen change below.** No frozen
clause is *overturned*: the changes are to reasons, remedies, rule adequacy and
added obligations, and every existing MUST survives.

1. **ES-13 (`:2504-2517`), `[FROZEN]` — the remedy is wrong and the diagnostic is
   under-cited.**
   *Current text (`:2504`):* "Writing ES-11's and ES-12's rules requires binding
   the query to a named local", followed by a code block and a paragraph
   (`:2510-2517`) citing `error[E0716]` alone.
   *Required text:* the query must be owned by something that **outlives the
   stream** — a named local when the stream stays inside the function that
   declared it, a **parameter** when the stream escapes, since a local does not
   outlive its own function. It must state the **defect** — RPITIT's opaque type
   captures the query's lifetime — rather than pin an error code, because the
   code varies with the spelling of the return type. Where it gives examples it
   may cite all four arrangements compiled at E10 on 1.97.1: `error[E0716]` for
   the inline temporary `store.read(&Query::all(), …)` bound to a `let`;
   `error[E0597]` for a stream returned, or stored in a struct, under a **bare**
   `-> impl Stream<…>`; `error[E0515]: cannot return value referencing local
   variable` for the same return under `+ '_` or `+ 'a`; `error[E0597]` for a
   stream written into a slot that already outlives the call. The existing code
   block stays, relabelled as the in-function case rather than as the remedy.
   *Do not carry over:* either document's first answer to this. The dossier
   assigned `E0597` to every escaping arrangement and this ADR's review assigned
   `E0515` to every one; a third compile of all four showed each had generalised
   from the one spelling it had tried. **A clause that pins a code documents a
   spelling; this one must pin the cause.**

2. **ES-13 (`:2527-2528`), `[FROZEN]` — its named rule does not check it.**
   *Current text:* "- **Rule:** `read_result_is_stable_under_concurrent_append`
   **(new)** does not compile without this shape, so the rule is the check."
   *Why it is wrong, in the order the evidence arrives.* First, mechanically:
   that rule **does not exist**. It is absent from
   `crates/happenstance-testkit/src/registry.rs`'s `for_each_event_store_rule!`
   and from `suite.rs` except as a cross-reference in another rule's doc comment
   (`suite.rs:2641`); it is marked **(new)** at ES-11 and is unwritten. A rule
   that does not exist cannot be the check for a frozen clause. Second,
   substantively: even once written it would not discriminate, because dossier E3
   compiled that a by-value `read` supports every shape the rule needs — bind,
   build, poll once, append, drain — under real `trait_variant` with both
   `memory.rs` tests passing. (E3 is a scout transcript; the mechanical half above
   is verifiable in the tree today and does not depend on it.)
   *Required text:* `Rule:` compile-level, in VT-27's words, naming
   `crates/happenstance-core/tests/frozen_signatures.rs`'s cases 1 and 2 as the
   pin. **Gate note for whoever applies this:** `spec_trace`'s check 4 rejects a
   `Rule:` line naming a rule that is not in `suite.rs` unless the clause declares
   it new, so the replacement must not leave a bare rule identifier behind; a
   filename with a dot is not parsed as a rule identifier and is safe.

3. **ES-11 (`:2425-2472`), `[PROVISIONAL]` — promote the ceiling from advice to a
   MUST, and correct the sampling instant.**
   *Current text (`:2441-2450`):* a justification paragraph saying "A paginating
   adapter meets this by capturing the head at the first poll and bounding every
   subsequent page by `position <= H`". It is advice inside a paragraph arguing
   for `head()`, not an obligation.
   *Required text:* five additions. (a) A MUST: an adapter issuing more than one
   statement per `read` MUST capture a position ceiling *H* no later than the
   first poll and MUST bound every statement after the first by `position <= H`,
   or `>= H` under `backwards`. (b) That the sample instant may be *at or before*
   the first poll and a caller may not depend on which — this is defect D7, and
   it is what makes `MemoryEventStore` (call time) and `happenstance-sqlite`
   (deferred into `poll_next`) both conformant. (c) That its rule polls once
   *before* it appends **because** of (b), and that a rule appending before the
   first poll would fail conformant adapters at random — this belongs in the
   rule's own documentation, because it reads as an incidental detail and is the
   reason the rule is portable at all. (d) That the ceiling's soundness is
   inherited from ES-10: it is only a ceiling if nothing below *H* can become
   visible later. (e) A cross-reference to **ES-9**, whose `Rejects:` already
   names the empty-store hazard this MUST creates — `head()` is `None` on a new
   store (ES-30) and arithmetic on that bound errors or panics —
   with `reading_an_empty_store_yields_nothing` and `NullHeadPagingStore` as the
   existing rule and adapter.
   *Falsifier:* keep the sentence, add the two instruments and their phases —
   `happenstance-neon` at **phase 10** (one-shot HTTP that cannot capture a
   ceiling within its round-trip budget) and the Durable Object at **phase 9**
   (a cursor that can hold neither a stable snapshot nor the ceiling).

4. **ES-12 (`:2474-2496`), `[PROVISIONAL]` — the rule is not blocked.**
   *Current text (in the `Rule:` line, `:2484-2489`):* "Deterministic only against
   a fixture that can be paused between statements, so this rule ships with a
   hostile fixture in the testkit's own `tests/` — the same instrument ES-10
   needs."
   *Required text:* the pause points do not have to come from the fixture. A
   **mutant store's own `Stream` impl** supplies them at its `poll_next`
   boundaries: a mutant that evaluates one query item per poll against freshly
   sampled state tears on demand and deterministically. That is the technique
   `nothing_below_an_observed_position_appears_later` already uses on two
   `append` futures driven one poll at a time (`suite.rs:2778-2795`, over
   `poll_once` at `suite.rs:2838-2850`). So the rule is ES-11's rule with a
   multi-item query and a late-matching append — build, poll once, append an
   event matching a *late* item, drain, assert the drained set is the pre-append
   set — and it is writable now.
   *Also add:* ES-12 is discharged by ES-11's ceiling rather than by a second
   mechanism, so an adapter that implements the ceiling satisfies both.
   *What is not asserted:* whether a `Stream` impl expresses "one item per poll"
   cleanly. That is a compile the code run owes, not a claim made here.

5. **ES-14 (`:2534-2568`), `[FROZEN]` — add the per-*page* form of its rejected
   implementation.** It rejects `LIMIT n` per query *item*; a paginating adapter
   can also apply `LIMIT n` per *page*, which is a different bug with the same
   symptom. Add it, and record honestly that **no rule can reject it before phase
   10**, because nothing in the tree paginates.

6. **ES-16 (`:2648`) and VT-29 (`:1529`), both `[FROZEN]` — no normative change.**
   Add one sentence: `to` is the caller-side spelling of ES-11's ceiling, and N
   chunked reads are N samples, not one — nothing in the contract makes two `read`
   calls one snapshot.

7. **VT-28 (`:1495`), `[FROZEN]` — no normative change.** Its requirement that
   "the ADR that lands it must say so" about the DCB reference implementation's
   falsiness divergence is discharged by §6 of this ADR; cite it.

8. **New clause in §3.2 (`read`, `:2275-2685`) — `read`'s return type carries no
   `Unpin` bound.** Place it after ES-13, which it is adjacent to in subject.
   Sentence: `read`'s return type MUST NOT carry a `+ Unpin` bound.
   `[PROVISIONAL — falsified by a consumer that needs 'dyn EventStore' where the
   port has acquired a method the hand-written 'Pin<Box<…>>' wrapper cannot box:
   a generic method, which is not dyn-compatible, or a return whose lifetime the
   wrapper cannot name. Inconvenience is not the falsifier. Must be re-evaluated
   before phase 12: adding a bound to an opaque return type after publish is
   breaking, so this clause expires rather than drifts.]`
   `Rule:` compile-level — the erasure wrapper of E11 compiles and round-trips
   against `MemoryEventStore` through `&dyn DynEventStore`, with no `unsafe` and
   no change to the port.
   `Rejects:` the "fix" that adds `+ Unpin` so `dynosaur` can generate a wrapper.
   It buys an erasure fifteen lines of downstream code already provide (E11), and
   it permanently forbids a generator-backed stream — compiled at E12,
   `error[E0277]: … cannot be unpinned` on `async_stream::stream!` — which is the
   natural implementation of the chunked-cursor shape, the shape with no
   implementation in the workspace and therefore no vote.
   *No `Cases:` line;* `spec_trace` requires only that named cases exist, and
   this clause names none.

9. **New clause in §2.6 (`Query`, `QueryItem` and `ReadOptions`, `:1419-1635`) —
   `ReadOptions` carries exactly one lower bound and it is inclusive.** Place it
   after VT-29, which adds the upper bound. Sentence: `ReadOptions` MUST carry
   exactly one lower bound, `from`, and it MUST be inclusive; an exclusive
   `after` MUST NOT be added beside it.
   `[FROZEN]` — deliberately with no falsifier, because the argument is complete
   now and nothing an adapter or a runner could measure bears on it. Reopening it
   takes a new ADR; the thing that would justify one is a phase 6/7 projection
   runner that cannot express its resume through `checkpoint.next()`.
   `Rule:` compile-level, plus the existing `read_from_is_inclusive`
   (`suite.rs:802`, registered at `registry.rs:123`).
   `Rejects:` adding `ReadOptions::after` beside `from`,
   giving one `#[non_exhaustive]` options struct two optional lower bounds with
   opposite inclusivity and no precedence rule — the shape ES-16's `Rejects:` line
   already predicts an adapter will half-implement. Cites ES-26 for why the
   inclusive/exclusive asymmetry belongs between `ReadOptions` and
   `AppendCondition` rather than inside `ReadOptions`, and VT-13 plus ES-9 for why
   `checkpoint.next()` is the sound resume idiom over gaps.

10. **New prose in §3.5, beside ES-30's `count()` paragraph — `EventStoreExt` and
    `happenstance_core::prelude` do not ship at 0.1.** Prose rather than a clause,
    for ES-30's own stated reason: the absence of a trait is not checkable by any
    rule. It must record the compiled ceiling (E9): a blanket ext impl does not
    collide with `trait_variant`'s, and cannot be overridden by anyone —
    `error[E0119]` in-crate, `error[E0117]` orphan out-of-crate — so an adapter
    with a cheaper path must add an inherent method that silently shadows the ext
    method at concrete call sites and silently does not in generic code.

11. **§1.3's census (`:220-222`).** This ADR adds two clause IDs: one
    `[PROVISIONAL]` (item 8) and one `[FROZEN]` (item 9).
    *Current text:* "this document carries 193 clause IDs, of which 191 are
    normative: **135 `[FROZEN]`**, **46 `[PROVISIONAL]`**, **10 `[DEFERRED]`** and
    **two `[NON-NORMATIVE]`**".
    *This ADR's delta alone:* 193 → 195 total, 191 → 193 normative, 135 → 136
    frozen, 46 → 47 provisional. **Do not apply that arithmetic on its own.**
    ADR-0013 moves the same sentence when ES-10 lifts from `[PROVISIONAL]` to
    `[FROZEN]` (−1 provisional, +1 frozen), and any other phase-4 ADR minting a
    clause moves it again. The recount must be done **once**, in the code run,
    absorbing every ADR's delta together.
    *Mechanism, so nobody hand-edits the wrong half:* §7.1 and §7.2 sit between
    `<!-- BEGIN GENERATED: spec-trace §7.1–§7.2 -->` and `<!-- END GENERATED -->`
    and **regenerate themselves** — the new clauses appear there for free. It is
    §1.3's sentence that is hand-written, and `check_stated_census`
    (`xtask/src/spec_trace.rs:319`, anchored on `CENSUS_ANCHOR` at `:152`) fails
    the gate if it disagrees with the parsed clauses. So the census is not a
    convention that can be left stale; it is a build failure.

## Amendments this decision owes elsewhere

Outside the specification, and recorded for the same reason.

- **`RUNBOOK.md:2768-2774`** — *"`read` takes its query by value"* is withdrawn.
  Replace with: `read` keeps `&Query` (ES-13, confirmed by ADR-0011); the
  escaping cases are discharged by a caller-owned query, compiled in E10.
- **`RUNBOOK.md:1690-1692`** — phase 3's *"every rule is re-spelled when `read`
  takes its query by value"* argues from the withdrawn shape and goes with it.
- **`RUNBOOK.md:2792-2797`** (`ReadOptions::after`) and **`:2775-2778`**
  (`EventStoreExt`) — both withdrawn, with sections 6 and 7 of this ADR's Decision as the reason.
- **`RUNBOOK.md:2990-3002` and `:3004-3005`** — the proof artefact's framing.
  Case 1's parenthetical *"which E0716 refuses today"* (`:2994-2995`) is wrong
  twice over. The code is wrong: for a stream returned from the function that
  owns the query as a local, it is `error[E0515]`; `E0716` is the inline-temporary
  form, which is not what the case describes. And the premise is wrong: the case
  is **satisfiable today**, because the RUNBOOK does not say where the query
  lives, and with the query as a *parameter* it compiles and runs (E10). Case 2
  (`:2996`, "same reason") inherits both errors. The claim at `:3004-3005` that
  each of the four cases "fails against the current signatures" is false for all
  four. `frozen_signatures.rs` is a regression **pin**; exit criterion 3 must
  say so, and the red-today demonstrations phase 4 owns for `read` are
  `read_to_is_inclusive`, `read_from_and_to_bound_a_closed_window`,
  `read_to_under_backwards_bounds_the_older_end` and
  `read_limit_zero_yields_nothing` — three unwritable for a missing field and one
  **unexpressible**, because `limit(0)` is discarded by the builder.
- **`RUNBOOK.md:2779-2784`** — *"Provided `head()` and `count(&Query)`"*
  contradicts ES-30 `[FROZEN]` twice: `head()` is **required**, not provided, and
  `count()` does not ship. Recorded here because it sits in `read`'s neighbourhood;
  the decision is ES-30's and is not reopened.
- **[ADR-0001](0001-async-port-flavours.md), consequences** — *"If erasure is
  needed, [`dynosaur`](https://docs.rs/dynosaur) generates the wrapper"* is false
  for `read` as the port stands (`error[E0277]`, dossier E5). The correction: a
  hand-written object-safe trait with a blanket impl over `EventStore`, boxing and
  pinning at both methods, erases the port today with no `unsafe` and no change to
  the port (E11, compiled and run). `dynosaur` becomes usable only if `+ Unpin`
  is ever added, which item 8 above declines.
- **`docs/scenarios/E2E-CASES.md:1432`** — carries the same `dynosaur` assertion
  and takes the same correction.
- **`crates/happenstance-core/src/store.rs:104-109`** — the laziness docstring
  (defect D7). It must promise the **sampling instant** rather than deferral:
  laziness is permitted and not required; the state is sampled no later than the
  first poll; building a stream and not polling it is not guaranteed to be free.
  Its justification should name the case that actually needs deferral —
  `happenstance-sqlite`, which cannot `spawn_blocking` outside a runtime — rather
  than an unbuffered million-event replay, which no adapter in or planned for the
  workspace performs.
- **`crates/happenstance-core/src/query.rs:260-266`** — `limit` becomes
  `Option<usize>` and the doc comment *"A `limit` of zero is ignored, since
  requesting nothing is never what the caller meant"* is deleted; it is the
  premise VT-28 falsifies.
