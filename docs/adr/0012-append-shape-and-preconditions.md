# ADR-0012: `append` keeps its borrowed batch, and phase 4 declines the one question it cannot measure

- **Status:** accepted — with **one** part left provisional. ES-17 (batch
  ownership) keeps its `[PROVISIONAL]` marker, and the marker lifts at
  [phase 8](../RUNBOOK.md#phase-8--sqlite), which is the phase that builds the
  SQLite multi-row insert benchmark ES-17's own falsifier names. Nothing else
  below is provisional except CF-39, which is new and says why.

  **The two lines that needed a human's yes have it, 2026-08-08.** Amendment 2
  (ES-18's `Rule:` bullet, which CF-39 narrows in practice) and amendment 11(a)
  (`AppendCondition::guards` becomes private, against what VT-30 `:1584-1591` and
  ES-29 `:3253-3256` currently describe) were both accepted at sign-off, which is
  what moves this document from `proposed` to `accepted`. The reasoning recorded
  with the decision: 11(a) is a compiled defect — `c.guards = Box::new([])`
  compiles downstream today and yields a conditional append that is silently
  unconditional — whose remedy costs one accessor, and it makes the collection
  behave the way VT-26 already makes `Query::Items` behave. Amendment 2's cost,
  narrowing a `[FROZEN]` clause that permits either answer, is bounded by CF-39
  itself being `[PROVISIONAL]`: if a real adapter's driver absorbs every fault it
  can arm, the narrowing reverses by a marker edit rather than by a superseding
  ADR. Everything else here either transcribes a frozen clause, adds a rule name,
  or corrects a citation.
- **Date:** 2026-08-08
- **Settles:** ES-18 – ES-24 (transcription and rules for clauses already
  `[FROZEN]`), ES-25 – ES-29 and VT-30, none of which the ADR queue had assigned
  to anybody, and CF-39, which this ADR creates
- **Declines:** ES-17's lift, for a stated reason with a named owner, and
  forecloses two of the three shapes it could lift *to*
- **Extends:** [ADR-0010](0010-the-suite-must-prove-itself.md), whose corollary —
  a rule that no adapter can fail is decorative — every new rule below is written
  against by naming the implementation it rejects
- **Reads from:** [`docs/evaluation/phase-4-reconciliation.md`](../evaluation/phase-4-reconciliation.md),
  which is where the compiled results quoted here were produced. Every Rust claim
  in this document traces to that file's *What the compiler said* section, or to
  the two compiles this ADR's review ran for itself and reports inline in §9, or
  is marked as an obligation on the code run rather than a fact. There is no
  fourth category, and a sentence about Rust that fits none of the three is a
  defect in this document.

## One question, and four things that are not in it

The question is the queue's (`RUNBOOK.md:287`): **what shape does `append` take
and what are its preconditions — who owns the batch, what an empty batch is,
whether a batch can violate its own condition, and what a dropped future may have
done?** The row scopes itself to `(ES-17 – ES-24)`.

The `AppendCondition` is inside that question rather than beside it, because the
condition *is* the precondition: "whether a batch can violate its own condition"
cannot be answered without saying what a condition is. So this ADR claims ES-25 –
ES-29 and VT-30 in addition to the row's own range. That is not scope creep: no
queue row owns them — ADR-0011 has ES-11 – ES-13, ADR-0013 has VT-11 – VT-13,
ES-10 and ES-38, ADR-0015 has VT-14 – VT-25 — while phase 4 as a whole "discharges
ES-8 – ES-40, VT-1 – VT-31" (`RUNBOOK.md:2755`) and lists VT-30 in its own work
(`RUNBOOK.md:2805-2807`). An unowned clause inside a phase that must discharge it
is how a phase closes with a hole in it.

Four adjacent things are deliberately *not* answered here.

- **`read`'s signature, laziness and the `Query` borrow** are ADR-0011's. This
  ADR takes no position on whether `read` keeps `query: &Query`.
- **How a multi-guard condition is encoded on the wire** is ADR-0016's, at phase
  5. VT-30 changes `AppendCondition`'s fields and therefore its `serde` impl
  (`append.rs:110-144`); the private wire *format* is not decided here, and WF-4's
  "`after` is always present" becomes "per guard" as a hand-off, not as a
  decision.
- **How positions are assigned and when they become visible** is ADR-0013's.
  ES-19 below says what `append` *returns*; it does not say what a store may
  allocate.
- **`MIN_SUPPORTED_EVENTS_PER_BATCH`'s number and `AppendError::ExceedsStoreLimit`**
  are ADR-0015's. VT-24 appears below only for the sentence in it that forbids an
  `EventBatch` type, which is load-bearing for the ownership decision.

---

## Context

### What phase 4 was told to do, and why it may not do it

`RUNBOOK.md:2759-2767` opens phase 4 with an instruction:

> **`append` takes its events by value** (D2's family). … Choose between `impl
> IntoIterator<Item = Event>` and an owned batch type on the phase-2 evidence.

`SPECIFICATION.md:2693-2701` says the opposite and is the current truth:

> `append` MUST continue to take `events: &[Event]`.
> **[PROVISIONAL — falsified by a measurement on a real adapter showing the
> per-event clone is a material fraction of append cost. The named measurement is
> the SQLite adapter's multi-row insert benchmark, in the phase that builds it. A
> positive result changes the signature to take `Vec<Event>` **and** obliges the
> contract to give callers a cheap way to keep a copy for retry.]**

The marker is quoted whole on purpose. Its last sentence is the half most likely
to be dropped in a summary and it is the half that does the most work: it already
names `Vec<Event>` as the successor, and it already prices the change — a
by-value `append` is not one edit to one signature, it is that edit *plus* a new
piece of public API for callers who must keep a copy to retry with.

Under `RUNBOOK.md`'s own rule 5 the clause wins and the phase body is corrected.
That is the easy half. The hard half is that the clause does not merely disagree
with the instruction — it makes the instruction **unexecutable at this phase**,
and for three separate reasons that had not been assembled in one place.

**1. The evidence the instruction asks to decide on cannot exist here.**
`happenstance-sqlite` is `todo!()` bodies, and the workspace has no benchmark
harness at all. That is not an inference: ES-32's own marker states it
(`SPECIFICATION.md:3359-3363`), while declining a different question for the same
reason. A phase that decides a measurement question without the measurement has
not decided it; it has guessed and then frozen the guess, which is the failure
mode ADR-0010 exists to make expensive.

**2. Both options on the menu are independently dead.** The instruction offers a
choice of two and the tree has already refused both.

- *An owned batch type* is forbidden in terms by VT-24
  (`SPECIFICATION.md:1373-1374`): "There MUST NOT be a `MAX_EVENTS_PER_BATCH`
  constant and **no `EventBatch` type MUST be introduced** to carry one." Its
  `Rejects:` line (`:1386-1389`) gives the reason — a newtype puts a constructor
  in front of every `append` call site in every application in order to enforce a
  bound that is adapter-specific.
- *`impl IntoIterator<Item = Event>`* is **compiled** to be unerasable by
  `dynosaur`: `error[E0191]`, the associated `IntoIter` type must be specified,
  isolated against a minimal trait with no `read` and a concrete error type
  (dossier, E5). `dynosaur` is the escape hatch ADR-0001:110-112 names for the
  deliberately missing `dyn EventStore`, and E2E-54 — an application choosing
  SQLite when offline and a Durable Object when online — is the case that turns on
  it. The RUNBOOK names this cost itself (`:2765-2767`) and then offers the option
  anyway.

  The Rust reason is worth stating for a reader who has met C# generics first.
  `impl IntoIterator<Item = Event>` in argument position is sugar for a **generic
  method**: `fn append<I: IntoIterator<Item = Event>>(…)`. Rust monomorphises,
  so there is one machine-code copy of `append` per `I` a program instantiates,
  and a `dyn Trait` vtable holds exactly one function pointer per method. There is
  no single address to store, so the method cannot be in the vtable at all — this
  is not a limitation of `dynosaur`, which is only a macro that writes the erased
  wrapper by hand; it is why the wrapper cannot be written.

  Two honest qualifications, both from E5's own text. First, `dynosaur` cannot
  erase this port *today* either, and the blocker is `read`, not `append`
  (`error[E0277]`, `dyn Stream` cannot be unpinned). Whether `+ Unpin` joins
  `read`'s return type is ADR-0011's. So the E0191 argument is conditional on
  ADR-0011 restoring erasure — but it points the same way under either outcome,
  because a generic `append` forecloses erasure *permanently* while `read`'s
  `Unpin` question is still open. Second, *with* that fix E5 erased `&[Event]`,
  `Vec<Event>` **and an owned `EventBatch`** cleanly. Erasure is therefore not
  what distinguishes the borrow from the newtype and this ADR must not pretend it
  is: three of the four candidate shapes are concrete types and behave
  identically here. `impl IntoIterator` is the only one erasure rules out, and
  VT-24 — not `dynosaur` — is what rules out `EventBatch`.

  The ledger already predicted this. `RUNBOOK.md:563` gives ES-17's row the
  falsifier "the Cloudflare adapter, and **`dynosaur` failing to erase a generic
  `append`**". That second half is not a future measurement: it fired this pass,
  on a compile, and it is the evidence §2 rests on. What the row gets wrong is
  only the phase column and the bundling — see *Owed to `RUNBOOK.md`*.

**3. The instruction's label is wrong, and the mislabelling is why the item looks
larger than it is.** "D2's family" attaches this to a defect that is somewhere
else: `SPECIFICATION.md:1164` and `:1421` make D2 the infallible-constructor
mistake on `Query::Items`, which `RUNBOOK.md:2800` already fixes as its own item
nine lines further down. Ownership of `append`'s batch is not in D2's family and
never was.

### What the tree actually says about the cost

ES-17's three grounds (`SPECIFICATION.md:2710-2717`) are on the record and this
ADR does not repeat them as if they were new. What it adds is the shape of the
measurement that would overturn them, because "a material fraction of append
cost" as written can be satisfied by measuring the wrong thing:

- `Event`'s expensive fields are `Bytes` (`event.rs:185`, `:187`), which is
  refcounted, so `event.clone()` bumps a counter rather than copying a payload.
  What actually copies is one `Box<str>` for the type and one boxed tag slice.
- A **rejected** append clones nothing: `memory.rs:226` returns before the
  `extend`, and rejection is the routine outcome under contention.
- `ConditionViolated` obliges the caller to keep its events anyway, so by-value
  moves the clone off the adapter's success path and onto the caller's *every*
  path.

And the adapter that would benefit from ownership is a specific one: a store that
**moves** the payload into an owned row type it keeps. A SQL adapter does not —
it binds parameters from borrowed bytes and hands them to the driver, so it takes
nothing at all from owning the batch. The reference store is the wrong instrument
in the opposite direction: `memory.rs:232-235` performs the clone because an
in-memory log *is* the owned row type, which is the least representative shape in
the portfolio.

### Two claims about `append`'s ownership that are false, and are load-bearing

**Proof-artefact case 4 does not fail today.** `RUNBOOK.md:2999-3002` lists
"append a `Vec<Event>` and afterwards use `Event::into_parts` on an event it
still owns" among four cases that "each fails against the current signatures.
That is the whole point." It was re-compiled against unmodified
`happenstance-core` this pass and it compiles *and runs* (dossier, E4:
`test tests::runs ... ok`). The caller never gave the `Vec` away; `&events`
reborrows for the duration of the awaited temporary and the borrow ends at the
`?`. So case 4 is not a falsifier of ES-17 and must not be presented as one.

The unreachability that ES-17 (`:2703-2708`) and VT-3 (`:670-677`) actually
record is on the **adapter** side — `Event::into_parts` (`event.rs:243-246`) is
unreachable from any trait impl, because an impl is handed `&[Event]` and cannot
move out of a shared borrow. The artefact's own framing, "a **generic** consumer,
bound on `EventStore` and never on a concrete store" (`RUNBOOK.md:2990-2992`),
structurally excludes the side where the problem is.

**No rule in the suite fails if the ownership changes.** ES-17 cites
`append_preserves_event_payload`, a round-trip that passes identically under
`&[Event]` and under `Vec<Event>`. The sentence has no falsifier of its own in
either direction, which is exactly what a benchmark is for and exactly why a
conformance rule cannot substitute for one.

---

## Decision

### 1. `append` keeps `events: &[Event]`, and ES-17 stays `[PROVISIONAL]`

The signature phase 4 freezes is the one on disk (`store.rs:148-152`):

```rust
async fn append(
    &self,
    events: &[Event],
    condition: Option<&AppendCondition>,
) -> Result<SequencePosition, AppendError<Self::Error>>;
```

Phase 4 **may not** lift ES-17, and saying so is the result rather than a
deferral. The clause's falsifier is a measurement; the measurement's instruments
are an adapter with real bodies and a benchmark harness; neither exists, and
neither can be built inside a phase whose deliverable is a frozen contract. A
phase that declines a question by naming the evidence it lacks and the phase that
supplies it has not failed to decide — it has refused to decide by argument the
thing its own specification says is decided by measurement.

What "provisional" means here is narrower than it looks, and the distinction
matters to an adapter author reading this later. **The code is frozen; the
sentence is scheduled for re-examination.** Phase 4 ships `&[Event]`, every
adapter implements against `&[Event]`, and changing it is a contract change
requiring its own ADR — the marker does not license a later phase to edit the
signature, it licenses one specific measurement to *reopen* it.

**What phase 8 must produce for the marker to lift.** Restating the falsifier so
that a positive result cannot be manufactured:

1. Two builds of the *same* SQLite adapter differing only in `append`'s ownership,
   measured on the same harness.
2. A batch at or above `MIN_SUPPORTED_EVENTS_PER_BATCH` (VT-24's 128), with a
   realistic tag count per event, since the copies the borrow forces are one
   `Box<str>` and one boxed tag slice per event and both scale with the batch.
3. A stated success **and** rejection mix, because a rejected append clones
   nothing and a benchmark that never rejects measures the uncontended path only.
4. A statement of which adapter shape was measured — one that moves the payload
   into an owned row it keeps, or one that binds parameters from a borrow. Only
   the first can benefit, and a measurement that does not say which it is does not
   settle anything.
5. A design for the second half of the clause's own marker, which a measurement
   alone does not supply: "a cheap way to keep a copy for retry". A positive
   result is not a licence to change one signature. `ConditionViolated` obliges
   the caller to still hold its events, so by-value `append` is only a net win if
   the contract hands back something cheaper than `Vec::clone` — and until that
   shape exists, a measurement that shows the adapter saving a clone has not shown
   the *system* saving one.

If that fires, the successor is `Vec<Event>` **and nothing else** — see §2.

### 2. The two alternatives are foreclosed, so phase 8 inherits a binary choice

This is what phase 4 does *instead of* deciding ES-17, and it is the more useful
half. The RUNBOOK's choice-of-two has no surviving option: the owned batch type is
forbidden in terms by VT-24, and `impl IntoIterator<Item = Event>` is compiled to
foreclose `dynosaur` and with it E2E-54. Both are rejected here with reasons
(see *Alternatives rejected*), so the question phase 8 inherits is not "which of
four shapes" but "keep the borrow, or move to `Vec<Event>`" — which is exactly the
successor ES-17's own falsifier text already names (`:2700-2701`), and which E5
compiled to erase cleanly under `dynosaur` *once `read` gains `+ Unpin`* — a
qualification that has to travel with the claim, because without ADR-0011's fix
nothing erases at all.

A note on the Rust vocabulary, since three of these words are near-synonyms in C#
and are not here. `&[Event]` is a *shared borrow of a contiguous run of events*:
no ownership transfers, the caller keeps its `Vec` and may use it after the call,
and the adapter may read but not move out. `Vec<Event>` transfers ownership: the
caller's variable is gone at the call site, which is why `ConditionViolated`
would then force every caller to clone *before* appending in order to be able to
retry. `impl IntoIterator<Item = Event>` transfers ownership *and* makes the
method generic. The three differ in what the caller loses, not in what the
adapter gains, and ES-17's third ground is the one that turns on that.

### 3. An empty batch (ES-20 `[FROZEN]`) — transcription, and it is already true

`append(&[], _)` returns `AppendError::NoEvents`, and the emptiness check precedes
the condition check. `memory.rs:206-208` already does it, above the lock, and says
why. No new work beyond keeping the property when the condition becomes a guard
sequence: **emptiness of the batch is still checked before any guard is
evaluated**, and a zero-guard condition is unrepresentable rather than rejected
(§9).

### 4. Self-conflict (ES-21 `[FROZEN]`) — the sentence, and the rule that was missing

The documented sentence, which belongs on `EventStore::append` and on
`AppendCondition`:

> A condition is evaluated **only against events the store already held** when the
> append began. The events in the batch being appended are never considered, so a
> batch can never conflict with itself.

ES-21 has already decided this and this ADR records rather than re-decides it.
What it adds is why the sentence has to be written down at all, which the clause
gives at `:2903-2910`: the reference store answers correctly *by accident of
ordering* (`memory.rs:218-230` checks `stored` before extending), and the
per-row `INSERT … SELECT … WHERE NOT EXISTS` strategy — a live candidate in the
decision ledger — self-rejects on the canonical DCB uniqueness shape, where the
condition query matches the very event being written. An adapter built that way
rejects **every** conditional append and passes every rule in today's suite.

- **Rule owed:** `batch_is_not_evaluated_against_its_own_condition` — a two-event
  batch under a condition whose query matches both, `after` set past everything
  stored, asserting success and both events landing.
- **Wrong implementation owed:** `PerRowConditionStore` in the testkit's own
  `tests/` — re-evaluates the condition before each row and rolls back on
  rejection, so that it fails *this* rule and not `append_is_atomic`.

### 5. Cancellation (ES-22, ES-23, both `[FROZEN]`) — one checkable half, one documented half

In Rust, cancellation *is* dropping the future: there is no cancel token, the
caller simply stops polling and the state machine is destroyed at whatever
suspension point it had reached. Nothing runs afterwards except `Drop` impls, and
the call produces no `Result` at all — which is why ES-23's answer cannot be an
error variant. `AppendError` has nowhere to put "the outcome is unknown"
(`error.rs:148-166`) and adding a fourth variant would not help, because there is
no value to return it in.

**The documented half.** `EventStore::append` gains an explicit `# Cancellation`
section, and this is its text:

> # Cancellation
>
> Dropping this future does **not** cancel the append. An adapter MAY have
> committed the batch before the drop, and MAY commit it afterwards. A caller MUST
> NOT treat a dropped future as evidence that the append did not land. What the
> contract does guarantee is ES-22: the store is left either fully applied or
> unchanged, never partially applied. To resolve the outcome, see the verbatim
> reissue property documented on [`AppendCondition`] — and note that it holds only
> for a conditional append whose condition matches its own events.

Each adapter MUST state which of the two it does, in its own store module's docs.
That obligation is invisible to the conformance suite — no rule can read a
docstring — so the instrument this ADR proposes is the gate instead: `cargo xtask
ci` gains a check that every crate implementing `EventStore` carries a
`# Cancellation` heading in the module that implements it.

Three qualifications, because a proposed gate step is not a decided one and the
difference matters to whoever implements it. The step does not exist and has not
been written, so nothing here says it is buildable as described — "find every
crate implementing `EventStore`" is a source-text question the existing
`spec_trace` machinery does not answer today, and the code run may find that the
honest form is a per-crate allowlist rather than a discovery pass. Second, the
check proves the section **exists**, not that it is true; the truth of an
adapter's claim is reviewed when the adapter is built (phases 8–11). Third, and
consequently, this is a weaker instrument than a conformance rule and is recorded
as weaker rather than counted as one — if the code run cannot build it, the
fallback is the per-adapter review at phases 8–11 and ES-23's `Rule:` line says
so, not a rule that quietly appears instead.

The rejected implementation ES-23 names is worth repeating because it is the
plausible one: a pooled rusqlite adapter doing its work in `spawn_blocking` and
presenting itself as cancellation-safe. Dropping a `JoinHandle` does not cancel
the closure — the `COMMIT` executes, the caller is told nothing, an operator
retries, and the payment is issued twice.

**The checkable half.** ES-22: a dropped `append` future leaves no partial batch.

- **Rule owed:** `dropped_append_future_leaves_no_partial_batch` — build the
  future, poll it once with a no-op waker, drop it, read the whole store, assert
  the batch is present in full or absent in full.
- **Wrong implementation owed:** `YieldingRowAtATimeStore` — extends its log one
  row at a time with a suspension point between rows, so a drop after the first
  poll leaves a partial batch. It is the sibling of the existing `RowAtATimeStore`,
  which ES-18 registers against
  `a_concurrent_reader_never_sees_a_partial_batch`; that one tests *visibility*
  of rows that all land in the end, and this one tests rows that do not land at
  all. Neither substitutes for the other and the code run must not merge them.
- `MemoryEventStore` passes trivially — `memory.rs:184-238` contains no `.await` —
  which is precisely why the reference store cannot answer this question and a
  real adapter must.

**ES-23 carries `Rule:` none by design** (`:2949-2953`), and that is one of the
two `[FROZEN]` clauses that make phase 4's exit criterion 2 ("every semantic
sentence has a rule that fails without it") unsatisfiable as written. The
criterion is owed a rewrite; the clause is not. See *Amendments*.

### 6. Reissue (ES-24 `[FROZEN]`) — one guarantee, two limits, three rules

The guarantee, and it costs nothing to provide because it falls out of the
condition the caller already wrote: **a conditional append whose condition's query
matches its own events is at-most-once under verbatim reissue.** Reissue the
identical batch after a dropped future: `ConditionViolated` means the first
attempt landed, `Ok` means it had not and now has. Either way the store holds
exactly one copy, and the caller needs no identity, no idempotency key and no new
API.

The two limits are stated as limits, each with a rule that pins the
*non*-guarantee, so that an adapter cannot quietly strengthen it and leave callers
depending on behaviour the contract disclaims:

| Shape | Property | Rule |
|---|---|---|
| Conditional, condition matches its own events | at-most-once under verbatim reissue | `reissued_conditional_batch_lands_once` |
| Unconditional | none; a reissue appends a second copy | `reissued_unconditional_batch_lands_twice` |
| Conditional, condition does **not** match its own events | none; the retry is indistinguishable from a first attempt | `reissued_batch_conditioned_on_other_events_lands_twice` **(new here)** |

The third row is this ADR's addition. ES-24 states that limit in prose
(`:2977-2981`) and gives it no rule, and it is the *common* shape — a decision
that reads one thing and writes another, conditioning on `CourseCapacityChanged`
while appending `StudentSubscribed`.

- **Wrong implementation owed:** `PayloadDedupStore` — a store that hashes
  `(event_type, tags, data)` and refuses a duplicate. It passes row 1 and fails
  rows 2 and 3. This is not a strawman: a content-addressed or ingest-flavoured
  store gets that behaviour for free and would ship it as a feature.
- **Consequence stated rather than hidden:** rows 2 and 3 make duplicate-landing
  a **MUST**, so a store with natural payload dedup is non-conformant. That is
  deliberate. A contract whose idempotency varies silently by adapter is worse
  than one with none, because callers write retry loops against the adapter they
  happened to test on.
- Row 1's rejecting implementation should be ES-18's existing `WriteThenCheckStore`
  — it writes, then probes, then returns `Err`, so on reissue it holds two copies
  while answering `ConditionViolated`. That is a prediction from the registry's
  description, and the code run must confirm it **by running it**. ES-18's own
  retirement-and-reversal (`:2747-2791`) is the standing warning about reasoning
  from rule text: two of this specification's three retirements were written by
  reading a rule and both were wrong.

ES-24 needs no identity to be checkable, which is what makes it severable from
`EventId` and therefore from ADR-0014. Callers needing at-most-once for rows 2
and 3 supply their own dedup in the domain — a natural key in the **tags**, which
is queryable, and not in `metadata`, which is structurally not.

### 7. The returned position (ES-19 `[FROZEN]`) — and a doc comment that is wrong

`append` returns the position assigned to the **last** event of the batch;
positions within one batch are assigned in slice order and are strictly
ascending; the contract does **not** require them to be contiguous.

- **Rule owed:** `batch_positions_follow_slice_order` — append three
  distinguishable events, assert their read-back order matches the slice order,
  rather than inferring it from `all.last()` as the existing
  `append_returns_last_written_position` does.
- **Wrong implementation owed:** `ReverseOrderBatchStore` — writes the batch in
  reverse slice order and returns the maximum position, which passes
  `append_returns_last_written_position` on a quiescent store.

The port's own documentation is wrong about what the returned position is *for*.
`store.rs:126-128` offers it for "a follow-up append condition"; ES-19
(`:2846-2854`) says it is for checkpointing and reporting and is **not** a sound
`after`, because a foreign event may hold a position below it that the caller
never saw. The sound `after` comes from a read — `read_decision_model`
(`store.rs:205-215`). Correcting that comment is owed by this ADR.

### 8. `conflicting_position` is a **hint**, and the `Display` says so

Decided explicitly, as exit criterion 5 demands, and it agrees with ES-25
(`:3088-3093`), which is `[FROZEN]`: the field is informational, an adapter that
detects the conflict without learning which event caused it MUST be permitted to
report `None`, and callers MUST NOT depend on it.

**The `happenstance-neon` citation, which ES-25 does not carry.** Neon is why the
question was asked and it is **not** a reason to demote the field.
`adapter-shapes.md:116-125` records the result: Neon has no interactive
transaction, so the condition and the write must collapse into one statement — and
the collapse **keeps** `conflicting_position`. A CTE computes the probe and the
insert on one snapshot and projects both. The price is one extra aggregate index
scan on every append including uncontended ones, plus `Serializable` isolation for
soundness. Both are real; neither is a `None`. So the field survives, and "hint"
is what buys the *other* adapter — the one that writes
`INSERT … SELECT … WHERE NOT EXISTS` and gets back a boolean — the right to
report `None` without paying for a scan it does not need.

**The `Display`.** `error.rs:94` renders a fixed string and never interpolates the
field declared at `:96-104`. Both cases get a rendering, and the empty one is
specified rather than left to whoever writes the code:

- with a position: `append condition violated: the store already holds a matching event at position 42; rebuild the decision model and retry`
- without one: `append condition violated: the store already holds a matching event; rebuild the decision model and retry`

The no-position case renders **nothing at all** where the position would be. It
does not say "at position unknown" or `None`: a hint that is absent should read
as an absent hint, not as an advertised missing capability, and `Option`'s `Debug`
must never reach an operator's log. The remedy is named in both, because the one
thing a reader of this message needs to know is that it is routine.

Under VT-30's guards the field keeps its meaning and gains one sentence: with N
guards a store MAY name a conflicting position for **any** violated guard, or
`None`, and callers MUST NOT infer *which* guard was violated from it.

One implementation note that is an obligation and not a fact: `thiserror`'s
`#[error("…")]` attribute is a format string and cannot branch on an `Option`, so
the code run will either hand-write `impl Display` or find another shape. Nobody
has compiled that here and this ADR does not assert it compiles.

**Callers MUST NOT parse the message** to recover the position. The whole reason
`ConditionViolated` is lifted out of `Self::Error` is so that callers never
pattern-match on strings; a `Display` that tempts them back is worse than the
fixed string it replaces.

### 9. `AppendCondition` becomes a non-empty sequence of guards (VT-30)

VT-30 lands **at phase 4**, in code — `RUNBOOK.md:2805-2807` already has it in
this phase's work list — and its clause keeps its `[PROVISIONAL]` marker with the
falsifier the specification already gives it. Those two statements are not in
tension and the distinction is the point: shipping the shape is phase 4's, and
finding out whether an adapter can push it into one statement without a self-join
per guard needs the Postgres adapter (phase 10) and the benchmark harness (phase
8), which is what the clause's marker says (`:1570-1573`).

Most of what follows is transcription, and saying so keeps a later reader from
looking for a decision that is not there. VT-30 already names both rules, already
requires `Guard` to be `#[non_exhaustive]` with public fields, already says
`is_violated_by` becomes a fold, and already flags the SQL precedence hazard
(`:1574-1599`). What this ADR adds is three things and no more: the collection's
field visibility (settled by a compile, below), `MinCollapseStore` as the named
wrong implementation for a `Rejects:` line that today carries only prose, and the
falsifier correction against `RUNBOOK.md:561`.

**Why it must land now rather than later.** VT-27 is `[FROZEN]` and refuses
per-item bounds inside `Query`, telling an application that needs different
boundaries for different fragments to issue one read per fragment. That refusal is
only *sound* if the resulting four boundaries can be carried into one condition.
Without VT-30 the prescribed workaround is a liveness failure — the quiet
fragment's stale boundary governs the busy one under a `min(p₁…p₄)` collapse, and
the deployment reads the rejection rate as contention (`:1478-1485`). VT-27 frozen
plus VT-30 unlanded is a specification that mandates a bug.

**The falsifier conflict, resolved.** `RUNBOOK.md:561` gives VT-30 a different
falsifier — "E2E-04 and E2E-05 still unwritable after phase 4" — which is
phase-4-reachable, and that is exactly what is wrong with it. It is not a
falsifier of the *sentence*: it cannot show that per-guard boundaries are the
wrong design, only that a phase failed to ship them. A delivery check wearing a
falsifier's clothes satisfies CF-38's non-emptiness rule while defeating its
purpose, which is worth naming because CF-38 is a build failure and cannot catch
it. The specification's falsifier stands; the ledger cell is owed a correction.

**The shape.**

```rust
#[non_exhaustive]
pub struct Guard {
    pub query: Query,
    pub after: Option<SequencePosition>,
}

#[non_exhaustive]
pub struct AppendCondition {
    guards: Box<[Guard]>,          // private — see below
}

impl AppendCondition {
    pub fn guards(&self) -> &[Guard] { … }
}
```

Five points of Rust reasoning behind that, since each had an alternative, and the
first two were settled by compiling rather than by argument.

- **`Guard` is `#[non_exhaustive]` with public fields, which VT-30 already
  requires (`:1588-1591`) and this ADR only transcribes.** On a struct, that
  attribute forbids *literal construction* from another crate while leaving field
  reads and `..` pattern matches working. That is exactly the asymmetry an ingest
  policy needs (ES-29, E2E-37): a hub must be able to *read* `after` on a
  peer-supplied condition in order to refuse it, and must not be able to fabricate
  one. **Compiled this pass** on the pinned 1.97.1 toolchain, in a two-crate
  workspace: downstream, `Guard { query, .. }` in a pattern and `g.after` as a
  read both compile, and `Guard { query: 1, after: None }` as an expression is
  `error[E0639]: cannot create non-exhaustive struct using struct expression`.

- **`guards` is private, and the ADR's first draft of this bullet was wrong.**
  The tempting shape is `pub guards: Box<[Guard]>`, on the reasoning that
  `#[non_exhaustive]` makes zero guards unreachable from outside the crate "by
  construction" — no `from_guards`, `new` yields one guard, `and_guard` only adds.
  **That reasoning is false and was compiled to be false.** `#[non_exhaustive]`
  blocks the struct-literal *expression* and exhaustive matching; it does not
  block assignment to a public field of a value the caller owns. Downstream:

  ```rust
  let mut c = AppendCondition::new(q);
  c.guards = Box::new([]);      // compiles. E0639 never fires.
  ```

  which builds precisely the value VT-30's first sentence forbids and VT-26's
  `Rejects:` line (`:1430-1435`) describes the consequence of: a condition that
  matches nothing, so a conditional append is silently unconditional. A lost
  update, arrived at by one line of safe downstream code.

  VT-26 is worth citing carefully rather than as a precedent, because it is a
  *requirement* and not yet a fact: `query.rs:143-150` still has no
  `#[non_exhaustive]` on `Query::Items` and still derives `Default`, which is D2
  and which `RUNBOOK.md:2800` fixes in this same phase. So the two land together
  or neither does, and an ADR that borrowed VT-26's authority for a shape VT-26
  has not yet been given would be borrowing from itself.

  The remedy that keeps every property VT-30 asked for is a private field with a
  `guards(&self) -> &[Guard]` accessor, and it was compiled in the same workspace:
  the ingest policy's `c.guards().iter().any(|g| g.after.is_some())` compiles,
  `Guard`'s literal is still `E0639`, and there is no longer any expression that
  reaches zero guards. Read-but-not-forge survives; the hole does not. Two
  consequences travel with it and are owed to the code run rather than asserted
  here: `AppendCondition`'s `serde` impl (`append.rs:110-144`) becomes the other
  door into the field and must reject an empty guard array on deserialisation,
  which is what WF-10 already requires of every validity invariant; and the
  doctest at `append.rs:47` reads `condition.after` directly and will need the
  accessor.

  This is the one place where this ADR asks for something the specification's
  current prose does not say. VT-30 at `:1584-1591` and ES-29's `Rejects:` at
  `:3253-3256` both describe *today's* `AppendCondition` as "`#[non_exhaustive]`
  with public fields" and read "readable but not literal-constructible" as
  sufficient for the ingest refusal. It is sufficient for `Guard`. It is not
  sufficient for the collection, and neither clause had been compiled against.
  The amendment is recorded below; a human should confirm it before the code run
  acts on it, because it narrows a shape two frozen-adjacent clauses describe.

- **A `Guard` struct rather than a `(Query, Option<SequencePosition>)` tuple.**
  The tuple is one character shorter and cannot carry `#[non_exhaustive]`, cannot
  name its fields at a match site, and cannot grow a third field — and this crate
  already knows what a growable value type costs, since `AppendCondition` itself
  is `#[non_exhaustive]` (`append.rs:51`) for the same reason.
- **`Box<[Guard]>` rather than `Vec<Guard>`**, matching `Query::Items`
  (`query.rs:149`). A boxed slice is one `usize` smaller because it carries no
  spare capacity, and the type itself says the collection is finished growing —
  which is true of a condition, which is built and then handed to `append`. The
  price is real and worth naming for a reader coming from C#, where `List<T>` is
  the default and this trade does not exist: `and_guard` cannot push, so it
  unboxes to a `Vec`, pushes and re-boxes, making a chain of *n* `and_guard` calls
  quadratic in allocation. For the guard counts this exists to serve — VT-27's
  worked case is four — that is the right side of the trade, and the alternative
  is to carry spare capacity in every cloned condition forever.
- **`new`, `after` and `after_opt` stop being `const fn`.** A boxed slice
  allocates. Nothing in the tree calls them in a `const` context, and `Query`'s
  own constructors already allocate, so the crate loses no capability it was
  using. The code run confirms that by compiling; this sentence does not.

`AppendCondition::new(query)` MUST continue to produce a single unbounded guard,
and `after`/`after_opt` MUST continue to apply the given boundary to **every**
guard — so every existing call site and every existing `condition_after_*` rule
keeps its meaning and becomes the single-guard case. `is_violated_by`
(`append.rs:95-107`) becomes a fold: violated if **any** guard is violated.

- **Rules owed:** `condition_guards_carry_independent_boundaries` and
  `condition_with_one_guard_behaves_as_today`.
- **Wrong implementation owed:** `MinCollapseStore` — evaluates every guard
  against `min(after)` across the guards. It passes the entire existing
  `condition_after_*` family, because every one of them has a single guard, and it
  is the application-side workaround E2E-05 names, promoted into an adapter.
- Adapters pushing this into SQL must generate `(item AND position > p) OR …`
  with explicit parentheses. The precedence bug the scenarios' S7 flags for the
  single-boundary case becomes N times more likely, and
  `condition_guards_carry_independent_boundaries` is what catches it.

### 10. ES-25 – ES-29 are claimed, and generalise by conjunction

All five are `[FROZEN]` and this ADR records rather than decides them. §3.4's
preamble (`:3034-3041`) already establishes the generalisation and it is repeated
here only because each clause gains a per-guard reading that an implementer will
otherwise have to derive:

- **ES-25** (the *iff*): the store rejects if and only if it holds an event
  matching *some* guard's query strictly above *that guard's* `after`. Existing
  rules cover the single-guard case.
- **ES-26** (`after` exclusive, `from` inclusive): per guard.
- **ES-27** (a condition matches on tags, by the same algebra as a read): per
  guard.
- **ES-28** (degenerate inputs): an empty store rejects nothing whatever the
  guard count; an `after` at or beyond head rejects nothing, per guard.
- **ES-29** (`after` is store-local): the sentence generalises to "**any of
  whose guards'** `after` was assigned by a different store", and the hazard
  multiplies with the guard count — SY-6's blunt ingest refusal must trip if *any*
  guard carries `after: Some(_)`. The rule lives in `happenstance-sync-testkit`,
  which does not exist, so phase 4 owes the amended sentence and no rule.

### 11. `Fixture::MID_BATCH_FAULT` earns a clause: **CF-39**

CF-18 assigns this decision to phase 4 in terms (`:6793-6804`) and names the cost
of each exit. It is taken, and the capability gets a clause, because the hole is
real and has a wrong implementation.

`append_is_atomic_under_a_mid_batch_fault` (`suite.rs:1255-1300`) asserts that
after arming a fault at row *k*, the store holds all of the batch or none of it,
with which of the two decided by what `append` answered. **A fixture whose arm
does nothing passes that vacuously**: no fault, `Ok`, all rows present,
all-or-nothing satisfied. So a fixture can declare the capability supported,
contribute nothing, and produce a green rule — which is the same failure CF-18
itself was written to prevent one level up, where a rule `#[cfg]`-ed out of the
binary is indistinguishable from a rule that passed.

One precision, because it decides what the wrong implementation has to look like.
The trait's *provided* `arm_mid_batch_fault` panics rather than no-ops
(`contract.rs:217-227`), and its panic message names this exact mistake. So a
fixture that declares the capability and simply forgets the override does **not**
pass vacuously — it aborts loudly, which is the outcome that method was written
for. The vacuous pass needs a fixture that overrides `arm_mid_batch_fault` with an
*empty body*: honest-looking code, no panic, no fault, green. That — not the
forgotten override — is what `NoopFaultFixture` must be.

> **CF-39.** A fixture declaring `MID_BATCH_FAULT` supported MUST, when armed at
> *k* < `events.len()`, cause the write of the *k*-th event to fail **inside the
> store's own write path**, by a mechanism the store cannot absorb, so that the
> append returns `Err`. The fixture MUST state the mechanism. A fixture whose
> store can absorb every fault it is able to arm MUST decline the capability with
> that as its stated reason.
> `[PROVISIONAL — falsified by a real adapter whose only injectable mid-batch
> fault is one its driver transparently absorbs, such as a connection killed
> mid-statement behind a reconnect-and-retry pool; that would make "the append
> returns Err" a promise no fixture over that adapter can keep. The instruments are
> the rusqlite adapter at phase 8 and the Postgres adapter at phase 10, and no
> adapter has armed a fault yet.]`
> `Rule:` `arming_a_mid_batch_fault_makes_the_append_fail` **(new)**, plus the
> existing `append_is_atomic_under_a_mid_batch_fault`, which this one makes
> non-vacuous.
> `Cases:` E2E-07, E2E-39.
> `Rejects:` `NoopFaultFixture` in the testkit's own `tests/` — declares the
> capability supported and **overrides `arm_mid_batch_fault` with an empty body**,
> so it never reaches the trait's panic. It passes
> `append_is_atomic_under_a_mid_batch_fault` today and would report a green
> atomicity result for a store that has never been faulted.

The clause is deliberately about the **fixture's** promise, not the store's, for
the reason ES-18 establishes at `:2793-2801`: a decorator sitting above `append`
cannot reach between two rows of one transaction, so the injection has to be the
fixture's and there is nothing else to constrain.

**What CF-39 costs, stated rather than discovered later.** ES-18 is `[FROZEN]`
and its `Rule:` bullet (`:2735-2738`) permits **either** answer from a faulted
store — "with which of the two decided by what `append` answered" — and
`suite.rs:1250-1254` spells the permission out in terms: *"A store may swallow the
fault and commit the batch anyway — retrying the row, say. That is conformant."*
CF-39's "so that the append returns `Err`" does not contradict that sentence
about *stores*, but it does empty it in practice: a store that absorbs every
fault its fixture can arm must now decline the capability, so the rule's `Ok`
branch (`suite.rs:1289-1297`) becomes unreachable for any fixture that declares
support. This ADR takes that trade knowingly, and the reason is that the weaker
alternative is not a rule at all. "The fault must fire" is unobservable from
outside the store — firing-and-being-absorbed and never-arming produce the same
`Ok` and the same full log, which is the whole hole — so an anti-vacuity clause
that stops short of demanding `Err` demands nothing a test can see.

The `Ok` branch is **not** deleted. It stays as the assertion that catches a store
whose absorption is partial: `Ok` over a partial log is the violation the branch
names, and CF-39 does not make that state impossible, only harder to reach.
ES-18's `Rule:` bullet is owed one clarifying sentence saying which of the two
answers a *conformant fixture* may now produce, so that a later reader does not
find the branch, read it as dead, and delete it. That amendment is recorded below
and it is the one item in this ADR that touches a `[FROZEN]` clause's operative
content rather than its citations — **a human should sign that specific line
rather than the ADR as a whole.**

**Census.** CF-39 moves §1.3's hand-computed census (`:218-221`) by **+1 clause
ID, +1 normative, +1 `[PROVISIONAL]`**. It is stated as a delta rather than as a
total on purpose: ADR-0013's ES-10 lift moves the same three numbers, so whoever
applies these amendments must apply both and count once, not add two totals
computed independently.

Getting it wrong is a **build failure, not a silent drift**, and that is worth
knowing before the code run rather than after. `check_stated_census`
(`xtask/src/spec_trace.rs:319-346`) parses §1.3's sentence, compares it against
its own walk of the clause headers, and additionally checks the sentence's
internal arithmetic — total minus `[NON-NORMATIVE]` equals the stated normative
figure — precisely so a paragraph edited in pieces cannot agree with the checker
and disagree with itself. §1.3 stays a hand count on purpose; what the gate
removes is the possibility of an unnoticed one.

### 12. The stale MSRV comment at `append.rs:101-102`

The comment says `is_violated_by` is "written without a let-chain so the crate
keeps its 1.85 MSRV". ADR-0029 raised the floor to 1.97.1 at phase 2 and
let-chains stabilised at 1.88, so the reason is dead — and it reads as a live
constraint because ES-25 points readers at this function as the reference
definition of the condition predicate. It is **deleted rather than corrected**:
under §9 the body becomes a fold over guards, so whatever shape it takes, the
reason for it is "one predicate per guard", not a language version.

---

## Consequences

**Good.** `append`'s signature is settled for phase 4 in the only way the evidence
permits, and the question that survives is a binary one with a named owner, a
named instrument and a specified measurement — rather than a four-way choice
whose two most-cited options were already dead in two other documents nobody had
read together.

**Good.** Nine rules and one capability clause replace prose. Four sentences that
were `[FROZEN]` since the specification was assembled — self-conflict, drop
safety, slice order, reissue — become things an adapter can fail, each with a
wrong implementation named and owed to the testkit's own `tests/`.

**Good.** VT-27's frozen refusal becomes sound, because VT-30 lands with it.
Shipping one without the other was a specification that prescribed a liveness
failure as the workaround.

**Bad, and load-bearing for the code run.** This ADR owes **nine** rules
(`batch_positions_follow_slice_order`, `batch_is_not_evaluated_against_its_own_condition`,
`dropped_append_future_leaves_no_partial_batch`, `reissued_conditional_batch_lands_once`,
`reissued_unconditional_batch_lands_twice`,
`reissued_batch_conditioned_on_other_events_lands_twice`,
`condition_guards_carry_independent_boundaries`,
`condition_with_one_guard_behaves_as_today`,
`arming_a_mid_batch_fault_makes_the_append_fail`) and **six** wrong
implementations. Each rule costs five mechanical edits — the `suite.rs` function,
the `registry.rs` identifier, a `SPECIFICATION.md` clause naming it, a mutant and
its `Declared` row, and a CHANGELOG entry. Seven of the nine names already appear
in a clause, so those clause edits are `Rule:` line amendments rather than new
prose; two are new.

**Bad.** `AppendCondition::new`, `after` and `after_opt` lose `const`, and
`AppendCondition`'s public field is replaced by a private one plus an accessor —
so every downstream read of `condition.fail_if_events_match` or `condition.after`
becomes a call, including the crate's own doctest at `append.rs:47`. Nothing is
published so the break costs nothing today, and it is the last cheap moment: after
the first adapter ships, a store that ignores an unknown guard silently evaluates a
weaker condition than the caller asked for — the same failure mode VT-29 records
for `ReadOptions::to`, and worse, because this one is a lost update rather than an
over-read.

**Bad, and it is a correction to this ADR's own first draft.** That accessor is
not a stylistic preference. §9 compiled the public-field shape and found it
reaches zero guards from downstream in one line, which is the state VT-30's first
sentence forbids — so the specification, in two places, currently describes a
shape that does not enforce what it requires. Nothing shipped on it and nothing
was lost. What it cost was the assumption underneath: `#[non_exhaustive]` was
being read as sealing a struct, when what it seals is the struct *expression*.
Any future clause that reasons "unreachable by construction" about a public field
is making the same mistake and should be compiled before it is written.

**Bad, and it narrows a `[FROZEN]` clause in practice.** CF-39 makes the `Ok`
branch of `append_is_atomic_under_a_mid_batch_fault` unreachable for any fixture
that declares the capability, which is a real reduction in what ES-18's rule
exercises even though ES-18's sentence about *stores* is untouched. §11 states the
trade and why the weaker alternative is not a rule at all; ES-18's `Rule:` bullet
is owed the sentence that keeps a later reader from deleting the branch. **This is
the item in this ADR most in need of a human's explicit yes**, because it is the
only place where a decision here changes what a frozen rule can catch.

**Bad, and unresolved by design.** ES-23's caller-facing half has no conformance
rule and cannot have one. The gate check proposed in §5 catches a missing
`# Cancellation` section and cannot catch a section that lies. Phase 4 ships a
documentation obligation with a documentation-grade instrument, and says so rather
than counting it as a rule.

**Neutral.** `Event::into_parts` stays. It is unreachable from any adapter under
`&[Event]` and its doc comment claiming it "avoid[s] a clone in adapter write
paths" is false as written (ES-17 already requires that correction). Callers and
the typed layer's wire encoders can reach it, and those are the users it keeps.

---

## Alternatives rejected

- **Lift ES-17 to `[FROZEN]` on the three grounds the clause already states.**
  The tempting option, and it would let phase 4 claim its signature freeze without
  an asterisk. Rejected because the three grounds are exactly what the clause
  already says, so lifting on them is lifting on no new evidence — the marker
  would move because a phase wanted it to, which is the one reason a maturity
  marker must never move. The specification's own standing rule (`:264-267`) is
  that a clause is provisional where the far end could plausibly vote against, and
  the far end here is an adapter with real bodies that nobody has built.

- **Change `append` to `impl IntoIterator<Item = Event>`, as the RUNBOOK asks.**
  Rejected on a compile: `error[E0191]` under `dynosaur`, because the associated
  `IntoIter` type cannot be named in an erased wrapper (dossier, E5). It
  permanently forecloses `dyn EventStore`, which ADR-0001 accepted losing *only*
  because `dynosaur` could restore it, and E2E-54 — runtime store selection, the
  first thing a local-first application asks for — is the case that then has no
  answer at all. It also gains nothing the borrow does not already give: the
  adapter still cannot move payloads it does not own into rows it binds by
  reference.

- **Change `append` to an owned `EventBatch` newtype.** Rejected in terms by
  VT-24 (`:1373-1374`, `:1386-1389`): it puts a constructor in front of every
  `append` call site in every application in order to enforce a bound that is
  adapter-specific, so the type carries either the wrong number or no number.

- **Change `append` to `Vec<Event>` now, ahead of the measurement.** It is the
  only shape that survives the other two rejections, and it erases cleanly once
  ADR-0011 settles `read`'s `Unpin` question — though so does an `EventBatch`
  newtype, so erasure is not what picks it; VT-24 is. It is therefore a real
  candidate — for phase 8. Rejected here because it inverts ES-17's
  third ground with no evidence: `ConditionViolated` obliges callers to keep their
  events for the retry, so by-value moves a clone from the adapter's *success*
  path to the caller's *every* path, and rejection is the routine outcome under
  contention. Trading a measured-to-be-cheap clone for an unmeasured one, in the
  phase that has no way to measure either, is the failure this ADR is declining.

- **Take `RUNBOOK.md:561`'s falsifier for VT-30** ("E2E-04 and E2E-05 still
  unwritable after phase 4") and let the clause lift when phase 4 ships the
  guards. Rejected: it is a delivery check, not a falsifier. It can only report
  that a phase failed to do its work, never that per-guard boundaries were the
  wrong design — and adopting it would let VT-30 go `[FROZEN]` on the strength of
  having been implemented, which is circular. Rule 5 points the same way: the
  clause wins over the ledger.

- **Demote `conflicting_position` to nothing — remove the field.** The decision
  ledger names Neon as the forcing case. It does not force it:
  `adapter-shapes.md:116-125` compiled the CTE that collapses probe and write into
  one statement and **keeps** the field. Removing it would take a working
  capability away from the adapters that can afford it in order to spare the ones
  that cannot, when `Option` already spares them.

- **Make `ConditionViolated` an enum** (`Unspecified` / `At(position)`) so the
  two `Display` renderings fall out of the variant. Rejected: it turns a hint into
  a thing callers match on, which is precisely what ES-25's "callers MUST NOT
  depend on it" forbids, and it costs every caller a wildcard arm on a
  `#[non_exhaustive]` enum for a field they were told to ignore.

- **Add a fourth `AppendError` variant meaning "outcome unknown".** Rejected by
  ES-23's own argument, and it is a Rust point rather than a design preference:
  a dropped future produces no `Result` at all, so there is nowhere for the value
  to go. The variant would be constructible only by an adapter that *did* return —
  that is, in exactly the case where the outcome is known.

- **Leave `MID_BATCH_FAULT` as prose**, which CF-18 offers as the other exit and
  half-writes at `:6793-6804`. Rejected because the hole has a wrong
  implementation that passes today: a no-op arm satisfies
  `append_is_atomic_under_a_mid_batch_fault` vacuously. A capability whose
  promise is unstated cannot be broken, and a rule gated on it then reports
  something about the fixture's honesty rather than about the store.

---

## Amendments this decision owes the specification

Recorded rather than applied, per this repository's rule that a frozen clause
changes by ADR and not by edit. **This ADR is the authority for every one of them
marked `[FROZEN]`.** They are applied in the run that writes the code.

Each entry names the clause, what it says now, and what it must say, so that a
later run can apply the edit without re-reading this document. Where an entry only
adds a mutant name to a `Rejects:` line, it says so — those are cheap, and they
are listed rather than left implicit because a rule with no named wrong
implementation is exactly what ADR-0010's corollary forbids.

1. **ES-17 (`:2693`, `[PROVISIONAL]`, marker unchanged).** Says now: falsified by
   "a measurement on a real adapter showing the per-event clone is a material
   fraction of append cost", with the successor and the copy-for-retry obligation
   in the marker's last sentence. Must additionally say: the five conditions §1 of
   this ADR imposes on that measurement (paired builds, a batch at VT-24's floor, a
   stated success/rejection mix, a stated adapter shape, and a design for the
   marker's own copy-for-retry obligation); that the *only* surviving successor
   shape is `Vec<Event>`, because an `EventBatch` type is forbidden by VT-24 and
   `impl IntoIterator<Item = Event>` is compiled to be unerasable
   (`error[E0191]`) while `Vec<Event>` and `EventBatch` both erase and are
   therefore not separated by that argument; and that no rule in the suite fails in
   either direction, so `append_preserves_event_payload` is cited as a *round-trip*
   rather than as this clause's falsifier.

2. **ES-18 (`:2728`, `[FROZEN]`) — the one operative change, signed off
   2026-08-08.** Its `Rule:` bullet (`:2735-2738`) says the store must hold
   all or none of the batch "with which of the two decided by what `append`
   answered", and `suite.rs:1250-1254` reads that as permitting a store to swallow
   the fault and answer `Ok`. CF-39 requires a *fixture* declaring
   `MID_BATCH_FAULT` to arm a fault the store cannot absorb, which makes the `Ok`
   branch unreachable for a conformant fixture. The bullet must gain one sentence:
   the two answers remain conformant for a **store**, but a fixture declaring the
   capability must produce the `Err` one, and `suite.rs:1289-1297`'s `Ok` branch is
   retained deliberately — it catches `Ok` over a partial log, which CF-39 makes
   harder to reach and does not make impossible. Without that sentence a later
   reader finds an unreachable branch and deletes it.

3. **ES-19 (`:2826`, `[FROZEN]`) — three edits, two of them stale citations.**
   (a) The clause's prose says the returned position "is **not** a sound `after` …
   `store.rs:126-128` currently offers it for exactly that use and must be
   corrected"; that correction is owed in `store.rs`, not in the clause, and is
   recorded here so it is not lost with the phase body. (b) The same paragraph
   cites `read_decision_model` at `store.rs:198-208`; the function is at
   `store.rs:205-215` and `:198-208` is its doc comment. (c) `Rejects:`
   (`:2839-2843`) describes the wrong implementation in prose — "an adapter whose
   bulk insert returns generated keys in unspecified order" — and must name
   `ReverseOrderBatchStore` as the mutant that supplies it, since
   `batch_positions_follow_slice_order` is otherwise a rule with no registered
   failure. (`Rejects:` is at `:2839-2842`.)

4. **ES-21 (`:2892`, `[FROZEN]`) — mutant name only.** `Rejects:` (`:2903-2910`)
   describes the per-row `INSERT … SELECT … WHERE NOT EXISTS` strategy in prose
   and names no implementation. It must name `PerRowConditionStore`, the mutant
   §4 owes the testkit's own `tests/`, which re-evaluates the condition before each
   row and rolls back on rejection — so that it fails
   `batch_is_not_evaluated_against_its_own_condition` and **not** `append_is_atomic`.
   The clause already names the rule; only the wrong implementation is missing.

5. **ES-22 (`:2912`, `[FROZEN]`) — mutant name, and one stale citation.**
   `Rejects:` (`:2924-2928`) must name `YieldingRowAtATimeStore` — a suspension
   point between rows, so a drop after the first poll leaves a partial batch — and
   must say in terms that it is *not* a rename of `RowAtATimeStore`, which
   `a_concurrent_reader_never_sees_a_partial_batch` owns and which tests
   visibility of rows that all land in the end. The same bullet says
   `MemoryEventStore` passes trivially "because `memory.rs:184-224` contains no
   `.await` at all"; `append` runs `memory.rs:184-238` and the truncated range
   stops before the `extend`, so it must read `:184-238`.

6. **ES-23 (`:2930`, `[FROZEN]`).** Says now: `Rule:` none (`:2949-2953`). Must
   say: `Rule:` none in the conformance suite, because no rule can read a
   docstring; the documentation obligation's **intended** instrument is a
   `cargo xtask ci` step asserting that every crate implementing `EventStore`
   carries a `# Cancellation` heading in the implementing module, which proves the
   section exists and not that it is true. The clause must record it as intended
   rather than as existing — the step has not been written and its discovery half
   ("every crate implementing `EventStore`") is not something the current
   `spec_trace` machinery can answer — and must name the fallback if the code run
   cannot build it: per-adapter review at phases 8–11, with no rule substituted in
   silently.

7. **ES-24 (`:2960`, `[FROZEN]`).** Says now: two rules
   (`reissued_conditional_batch_lands_once`,
   `reissued_unconditional_batch_lands_twice`). Must say: three, adding
   `reissued_batch_conditioned_on_other_events_lands_twice` for the second stated
   limit, which currently has prose and no rule; and must name `PayloadDedupStore`
   in `Rejects:`, since a content-addressed store strengthens the guarantee for
   free and callers would then depend on it.

8. **ES-25 (`:3043`, `[FROZEN]`).** The `conflicting_position` paragraph
   (`:3088-3093`) must gain: the decision that the field is a **hint**, with
   `happenstance-neon` cited via `adapter-shapes.md:116-125` as the case that
   asked the question and did not force the demotion; the two required `Display`
   renderings including the shape of the empty case; the prohibition on parsing
   the message; and, under guards, that a store MAY name a position for any
   violated guard and callers MUST NOT infer which guard was violated.

9. **ES-29 (`:3223`, `[FROZEN]`) — two edits.** (a) Says now: "A store MUST NOT
   evaluate an `AppendCondition` whose `after` was assigned by a different store."
   Must say: "…**any of whose guards'** `after` was assigned by a different
   store", and must note that SY-6's ingest refusal trips if *any* guard carries
   `after: Some(_)`. (b) Its `Rejects:` bullet (`:3253-3256`) argues that the
   ingest refusal "is checkable today" because "`AppendCondition` is
   `#[non_exhaustive]` with **public** fields (`append.rs:51-61`), so … readable
   but not literal-constructible is exactly what makes an ingest-side policy
   possible". §9 compiled that argument and it holds for `Guard` and **not** for
   the collection: a public field on a `#[non_exhaustive]` struct is assignable
   from downstream, so a hub can forge as well as read. The bullet must be
   re-pointed at whatever shape lands — with `guards` private and a
   `guards() -> &[Guard]` accessor, the refusal is still checkable and the forgery
   is not, which is what the sentence was reaching for.

10. **VT-24 (`:1371`, `[PROVISIONAL]`).** The clause bundles two normative
   sentences under one marker whose falsifier — "a domain whose smallest
   indivisible unit of work exceeds 128 events" — reaches only the *floor*. A
   positive result there raises the number; it does not introduce an `EventBatch`
   type. The marker must be annotated to say which sentence it governs, because
   this ADR relies on the prohibition being effectively firm while the clause
   carries a provisional marker.

11. **VT-30 (`:1562`, `[PROVISIONAL]`, marker unchanged) — less than it looks.**
    The clause **already** carries `Guard` as `#[non_exhaustive]` with public
    fields and its reason (`:1588-1591`), already names both rules (`:1574-1576`),
    already says `is_violated_by` becomes a fold and flags the SQL precedence
    hazard (`:1593-1599`). Three things are actually owed. (a) `:1586` says
    "Replacing `fail_if_events_match` with `guards`" without saying whether
    `guards` is public; it must say **private, with a `guards(&self) -> &[Guard]`
    accessor**, and must carry §9's compiled reason — `#[non_exhaustive]` does not
    stop `c.guards = Box::new([])` from downstream, so a public field makes the
    clause's own first sentence ("MUST hold a non-empty sequence") unenforceable.
    Delete any claim that non-emptiness is structural via `new` + `and_guard`
    alone; it is structural via `new` + `and_guard` **plus** the private field,
    plus a `Deserialize` impl that refuses an empty array (WF-10). (b) `Rejects:`
    (`:1578-1582`) describes the `min(p₁…p₄)` collapse in prose and must name
    `MinCollapseStore` as the mutant for
    `condition_guards_carry_independent_boundaries`. (c) A sentence that
    `new`/`after`/`after_opt` lose `const`, since a boxed slice allocates.

12. **New clause CF-39 in §6.3**, as drafted in §11, `[PROVISIONAL]` with the
    falsifier stated there. §1.3's census (`:218-221`) moves by +1 clause ID, +1
    normative, +1 `[PROVISIONAL]`. **Recount by hand once**, after ADR-0013's ES-10
    lift is applied, since both decisions move the same three numbers, and note
    that `check_stated_census` (`xtask/src/spec_trace.rs:319-346`) fails the gate
    on a wrong count rather than letting it pass unnoticed.

13. **CF-18 (`:6771`, `[FROZEN]`).** Its closing paragraph (`:6793-6804`) records
    that `MID_BATCH_FAULT` has no clause of its own and assigns the decision to
    phase 4. It must be updated to cite CF-39 as the answer, so the paragraph
    stops reading as an open question.

### Owed to `RUNBOOK.md` rather than to the specification

Recorded here because the phase body is fixed in the same commit under rule 5, and
because a reader who fixes only the clauses will leave the instructions
contradicting them.

- **`:2759-2767`** ("`append` takes its events by value (D2's family)") is
  replaced by the decision above: the batch stays borrowed, ES-17 stays
  provisional, and the two named alternatives are foreclosed. The "D2's family"
  label is deleted — D2 is `Query::Items`' infallible constructor
  (`SPECIFICATION.md:1164`, `:1421`), fixed as its own item at `:2800`.
- **`:563`** bundles ES-7 and ES-17 in one row — "The `!Send` flavour and `append`
  ownership" — under one falsifier with two halves, "the Cloudflare adapter, and
  `dynosaur` failing to erase a generic `append`", in phases "1 and 4, confirmed
  9". Three corrections, and the row's own history is why it needs them. The
  `dynosaur` half **fired this pass**, on E5's compile, and the row must record it
  as fired rather than pending — it is the evidence that forecloses `impl
  IntoIterator` and it is sitting in the ledger as a future check. The ownership
  half belongs to phase 8, which is what ES-17's own falsifier says, not to 4. And
  the two clauses want splitting, because ES-7's instrument is an adapter on a
  different target from ES-17's and one row cannot report two verdicts.
- **`:561`** gives VT-30 the falsifier "E2E-04 and E2E-05 still unwritable after
  phase 4". Replaced by the clause's own falsifier (`SPECIFICATION.md:1570-1573`),
  with the phase column reading 4 for the implementation and 8/10 for the marker.
- **Exit criterion 2** (`:3021`, "Every semantic sentence has a rule that fails
  without it") is unsatisfiable as written for ES-23, whose deliberate content is
  that no conformance rule can observe it. It must read: *every semantic sentence
  has a rule that fails without it, or cites a clause whose stated content is that
  no rule can observe it and names the instrument that checks it instead.* ES-37 is
  the other `[FROZEN]` clause in that position; ES-32 is `[PROVISIONAL]` and VT-27
  is compile-level.
- **Exit criterion 5** (`:3027-3028`) is discharged by §8 above: the field is a
  hint, and `happenstance-neon` is cited for why the question was asked.
- **`:2955-2962`**, the stale-MSRV-comment item, is discharged by §12: the comment
  is deleted rather than corrected, because VT-30 replaces the body. The item
  itself cites `append.rs:100-101`; the comment is at `append.rs:101-102` and
  `:100` is the signature's closing paren. Worth fixing in passing, since the item
  is about a citation that stopped being true.
- **`:2805-2807`** puts VT-30 in this phase's work list and describes it as
  "per-**item** boundaries". The clause calls them guards and VT-27 is `[FROZEN]`
  on `Query` and `QueryItem` carrying no positions at all, so "per-item" reads as
  the very thing VT-27 forbids. It must read *per-guard*.
- **Proof-artefact case 4** (`:2999-3002`) is not a case that fails today. It
  compiles and runs against unmodified `happenstance-core` (dossier, E4). The
  artefact's framing — a generic consumer never bound on a concrete store —
  structurally excludes the adapter side, which is where `Event::into_parts` is
  actually unreachable. Whether the artefact is rewritten or reduced is ADR-0011's
  call, since three of its four cases are `read`'s; this ADR records only that the
  fourth is green.
