# #144: CF-27, the completeness instrument (`WithholdingStore`)

## Goal

`happenstance-testkit` gets a decorator over any `EventStore` that holds an arbitrary
retained set of the inner store's log: a suffix, or a scattered subset with survivors
between the holes. Through every port method, the events it withholds behave as removed:
`read`, `head`, `contains_event_id`, and the condition evaluated by `append`. It reports
what it withholds through its own inherent API and never through an `EventStore` method
(ADR-0028 decision 8). It ships with:

- the full suite mounted against it, with the pass list recorded in a committed file;
- the rule `instrument_report_is_accurate_and_the_suite_cannot_tell`, which asserts both
  that the suite passes and that the report is accurate;
- three wrong instruments in testkit `tests/`, each one rejected by that rule.

## Constraints and non-goals

- **No `EventStore` change**, and no provided method either. That is ES-39 and #156's call.
- **No new dependency.** The testkit keeps `happenstance-core` and `futures-core`, and no
  `thiserror` (H-16). Error impls are written by hand, following `FaultyStoreError`.
- **Never `#[async_trait]`.** The trait is implemented by hand as the bare `EventStore`
  flavour, following `FaultyStore` (CLAUDE.md constraint 1, ADR-0001).
- **Only additive public API** in a published crate: a minor change, with a `CHANGELOG.md`
  `[Unreleased]` entry. `cargo-semver-checks` against `0.6.0` must stay silent.
- **No literal positions in any assertion.** Positions come only from what the store
  returned, or from indexes into that list.
- **Not built here, but the seam is kept:** #147's `Fixture::REMOVAL` and
  `remove_outside_port(&[SequencePosition])`; #151's ES-38 rule; #154's ES-40 rule;
  #156's reader experiment; #158 (SY-32).
- **No spec marker moves.** CF-27, ES-38, ES-39 and ES-40 keep their brackets; #163
  freezes them. The spec edits below are reconciliation only (D5).
- **No runbook edits.** `runbook/afk.md` forbids ticking exit criteria from a leaf.
- **No ADR.** `runbook/afk.md` also says never to write one as a side effect of a leaf.
- The module must compile on `wasm32-unknown-unknown`, because the gate checks the testkit
  there. `std::sync::RwLock` and `Arc` compile on that target. Nothing thread-only goes in
  `src/`.
- No clock under `src/` (CF-33, `lint-clock`).

## Invariants (each is a test below)

1. **Withheld positions are hidden.** No item yielded by `read`, under any `Query` or
   `ReadOptions`, has a withheld position.
2. **Nothing retained is hidden.** Every event the inner store yields at a position the
   instrument does not withhold is yielded by the instrument, in the inner's order.
3. **`limit` counts only retained events.** It applies after withholding.
4. **`head` is the highest retained position.** It is the highest position at or below the
   inner `head` that is not withheld, and `None` when every held event is withheld.
5. **Membership follows the withheld set.** `contains_event_id(id)` is `false` for a
   withheld event's id, and otherwise is the inner store's answer.
6. **Conditions see only retained events.** An `append` condition is violated if and only
   if some retained event violates it. A condition whose only match is withheld passes
   vacuously (ES-40). The evaluation is atomic with the write for every event that can
   appear concurrently.
7. **Positions are never reused.** Every position assigned after a withhold is greater
   than every position ever withheld (ES-38). This holds by delegation, because the inner
   store never forgets anything.
8. **The report is the hidden set.** `withheld()` equals the set of positions that the
   inner store assigned and the instrument's `read` does not yield. It is also exactly the
   union of the positions every successful `withhold` or `withhold_where` call asked for.
9. **Withholding nothing changes nothing.** With nothing withheld, every port call is a
   pure delegation, with the same arguments (including `limit`) and the same results.
10. **A failed withhold leaves the set unchanged.** A `withhold` that fails withholds
    nothing.

## Design

### Placement

- **New module `crates/happenstance-testkit/src/withholding.rs`**, private `mod withholding;`
  in `lib.rs`, re-exported as
  `pub use withholding::{WithholdError, WithholdingStore};`. It is **not** gated on
  `memory`, because it wraps any `EventStore` (the same choice as `faulty`).
  - Declare it at the **end of `lib.rs`**, beside `recording`, so that no `lib.rs:N`
    citation above it moves. `recording` is declared at the end for the same reason
    (lib.rs:832).
- **Why the testkit:** CF-27 says "testkit-adjacent", and the testkit already owns the
  caller-side instruments (`FaultyStore`, `GappyMemoryStore`, `RecordingProjectionStore`).
  The dependency direction is unchanged: testkit depends on core only (ARC-02).
  - It does not go in `happenstance-core`: an instrument is not contract.
  - It does not go in `happenstance-sync-testkit`: #156's projection runner is not sync.

### Mechanism: why a decorator, and how a condition is kept honest (D2)

The inner store still physically holds what the instrument withholds. Filtering `read`
alone would leave the inner store evaluating `append` conditions over withheld events,
and it would reject appends that a pruned store admits (it would fail ES-40). Three
shapes were weighed:

- **A. Hold its own log**, a `GappyMemoryStore`-style store with real removal. This is
  simple and faithful, but it is not "a decorator over any `EventStore`" (CF-27's words),
  and it could never wrap a real adapter or a `FaultyStore`. Rejected.
- **B. Remove physically from the inner store**, which would need a removal trait the inner
  implements. `MemoryEventStore` allocates from its own length (`gappy.rs:4-8`), so removing
  from it reuses positions. That reuse is ES-38's named wrong implementation, and B would
  put it inside the instrument. Real adapters get removal through #147's capability, which
  is the right place for it. Rejected.
- **C. Decorate, and rewrite the condition. Chosen.** Let `W` be the highest withheld
  position. For a conditional `append` with a non-empty withheld set:
  1. **Pre-check the frozen region.** Read the inner store with `Query::all()` and
     `ReadOptions::new().to(W)` (forward). For each event that is not withheld and that
     satisfies `condition.is_violated_by(position, event_type, tags)`, return
     `ConditionViolated::at(position)` for the first one, which is the lowest. A read error
     becomes `AppendError::Store(e)`.
  2. **Delegate the rest atomically.** Call
     `inner.append(events, Some(&raised(condition, W)))`. In the rewritten condition, every
     guard's `after` is replaced by `max(after, W)`, and `None` becomes `W`.

  **Why C is sound**, and the plan must keep this argument in the module comment:
  - Every event above `W` is retained, by the definition of `W`. So the inner store's own
    atomic evaluation of the raised condition covers exactly the retained events above `W`,
    under whatever lock or transaction it uses.
  - Retained events at or below `W` cannot change concurrently:
    - no append lands below the head, and `W` is a position the instrument has observed;
    - ES-10 (`nothing_below_an_observed_position_appears_later`) guarantees that nothing
      appears below an observed position later;
    - withholding only shrinks the set.

    So the non-atomic pre-check over that frozen region is exact.
  - Racing conditional appends still elect one winner, because the contended region is the
    inner store's.

  **Special cases:**
  - An **empty batch** is delegated untouched, before any pre-check, so the inner store
    refuses it in its own words. This is ES-20 and
    `empty_batch_is_refused_before_the_condition_is_evaluated`.
  - With **no condition**, or **nothing withheld**, `append` is a pure delegation.

### Types and signatures (ownership decided)

```rust
/// Private: the withheld set, copy-on-write.
#[derive(Debug, Default, Clone)]
struct Withheld {
    // Position -> the id of the event the inner store holds there. The id is captured
    // at withhold time, because membership (ES-41) is asked by id. An ingested event's id
    // is its origin's, so it cannot be derived from the position.
    by_position: BTreeMap<SequencePosition, EventId>,
}
impl Withheld {
    fn holds_position(&self, p: SequencePosition) -> bool;
    fn holds_id(&self, id: EventId) -> bool;          // values().any(..), O(n): an instrument
    fn highest(&self) -> Option<SequencePosition>;    // last_key_value
    fn is_empty(&self) -> bool;
}

#[derive(Debug, Clone)]                                // Clone iff S: Clone (derive bound)
pub struct WithholdingStore<S> {                       // no bound on the struct (API-21)
    inner: S,
    // Interior mutability (OWN-12): `withhold` takes &self because handles are shared and
    // the port takes &self. RwLock rather than RefCell so the type stays Sync/Send,
    // which the concurrency family needs (`ConcurrentFixture: Fixture<Store: Send>`) and
    // a later `Send` sibling would share. The inner Arc<Withheld> is a snapshot: readers
    // clone the Arc under a momentary read guard and drop the guard before any await
    // (H-14). Writers build the next set and swap it under the write guard. The outer Arc
    // makes a clone a second handle onto the *same* withheld set (FaultyStore's arming
    // precedent).
    withheld: Arc<RwLock<Arc<Withheld>>>,
}

impl<S> WithholdingStore<S> {
    /// Wraps `inner`, withholding nothing.
    #[must_use]
    pub fn new(inner: S) -> Self;

    /// The report: every position this instrument withholds, ascending.
    /// An owned snapshot, not a borrowing iterator (OWN-13 deviation, stated in the doc):
    /// the set lives behind a lock that cannot be held while the caller iterates, and the
    /// live set may change under a concurrent `withhold`.
    #[must_use]
    pub fn withheld(&self) -> BTreeSet<SequencePosition>;
}

impl<S: EventStore> WithholdingStore<S> {
    /// Withholds the events the inner store holds at `positions`. All or nothing.
    /// Re-withholding an already-withheld position is a no-op. An empty slice is Ok.
    /// The shape #147's `remove_outside_port(&[SequencePosition])` will share.
    ///
    /// # Errors
    /// `WithholdError::NotHeld { position }` (the lowest such position) if the inner store
    /// yields no event at a requested position: never assigned, a gap, above the head, or
    /// above the inner store's visibility frontier. In that case nothing is withheld.
    /// `WithholdError::Store(e)` if reading the inner store fails.
    pub async fn withhold(&self, positions: &[SequencePosition])
        -> Result<(), WithholdError<S::Error>>;

    /// Withholds every held, not-yet-withheld event for which `select` returns true, and
    /// returns their positions in ascending order (a fresh Vec: the data is new, OWN-13
    /// does not apply). This is the suffix (`|e| e.position <= p`), the purge by tag, and
    /// the "prune everything" the suite fixture uses.
    ///
    /// # Errors
    /// `WithholdError::Store(e)` if reading the inner store fails. Nothing is withheld.
    pub async fn withhold_where(&self, select: impl FnMut(&SequencedEvent) -> bool)
        -> Result<Vec<SequencePosition>, WithholdError<S::Error>>;
}

impl<S: EventStore> EventStore for WithholdingStore<S> {
    type Error = S::Error;   // the instrument adds no failure mode at the port; inner errors pass unchanged
    fn read(&self, query: &Query, options: ReadOptions)
        -> impl Stream<Item = Result<SequencedEvent, Self::Error>>;
    async fn append(&self, events: &[Event], condition: Option<&AppendCondition>)
        -> Result<SequencePosition, AppendError<Self::Error>>;
    async fn head(&self) -> Result<Option<SequencePosition>, Self::Error>;
    async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error>;
}
```

Every delegating call uses the fully qualified form `EventStore::read(&self.inner, ..)`,
following `faulty.rs:9-12`. Only `EventStore` is imported in the module (constraint 4).

**`read`** (synchronous, returns the stream, constraint 3):
- Take the snapshot.
- If it is empty, wrap the inner stream unchanged, with the same options (invariant 9).
- Otherwise, copy `options` (`ReadOptions` is `Copy`), set `inner_options.limit = None` (a
  public-field assignment on an owned value; struct-update syntax is E0639 on
  `#[non_exhaustive]`), and hand the original `limit` to the stream.

**`Withholding<St>`** (private stream, named per RS-23-4, `#[derive(Debug)]`):
- Fields: `inner: Pin<Box<St>>`, `withheld: Arc<Withheld>`, `remaining: Option<usize>`.
  Every field is `Unpin` (RS-23-1, the `InjectedRead` precedent, `faulty.rs:159-170`).
- `poll_next`:
  - If `remaining == Some(0)`, return `Ready(None)` without polling the inner stream.
  - Otherwise loop over the inner stream:
    - an `Ok` event at a withheld position: skip it;
    - an `Ok` retained event: decrement `remaining` with `checked_sub`, then yield it;
    - an `Err`: yield it unchanged (an error is not counted against `limit`);
    - `Ready(None)` or `Pending`: forward it.

**`head`**:
- If the snapshot is empty, delegate.
- Otherwise, let `top = inner.head()?`. If `top` is `None`, return `None`. If `top` is not
  withheld, return it.
- Otherwise, read the inner store backwards with `Query::all()` and
  `ReadOptions::new().backwards().from(top)`, and return the first non-withheld position,
  or `None` if there is none. Bounding by `from(top)` keeps the answer at or below the
  inner store's own visibility frontier.

**`contains_event_id`**: return `Ok(false)` if the snapshot holds `id`. Otherwise delegate.

**`withhold`** (both withhold methods share these steps):
1. Dedupe the request into a `BTreeSet`.
2. Read the inner store with `Query::all()`, bounded `from(min).to(max)`, and collect
   `(position, id)` for the requested positions.
3. If any requested position is missing, return `NotHeld` for the lowest one.
4. Otherwise, **merge under the write guard**: clone the *current* set (not the one read
   before the await), extend it, and swap in `Arc::new(next)`. Merging at swap time keeps
   two concurrent withholds from losing each other's entries.

The poisoning policy is `unwrap_or_else(PoisonError::into_inner)`, as in
`gappy.rs:131-133`. Every mutation is a single swap, so a poisoned lock carries no
information.

**`raised(condition, floor) -> AppendCondition`** (private fn):
1. Rebuild the first guard with
   `AppendCondition::new(first.query.clone()).after_every_guard_opt(lift(first.after))`.
2. Add each remaining guard with `.and_guard(g.query.clone(), lift(g.after))`, where
   `lift(a) = Some(a.map_or(floor, |a| a.max(floor)))`.

`after_every_guard_opt` must be called **before** any `and_guard`, because it erases the
`after` of every guard already present (`append.rs:165-168`). The `Query` clones are a real
duplication into a new owned condition (H-08). `guards()` is documented never-empty. If it
is empty anyway, return `condition.clone()` instead of panicking (H-03).

### Errors

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]                                  // published crate (API-05)
pub enum WithholdError<E> {
    /// The inner store yields no event at `position`, so there is nothing to withhold.
    /// A report naming it would claim the instrument hid an event that never existed.
    #[non_exhaustive]
    NotHeld { position: SequencePosition },
    /// Reading the inner store failed. Carried, not rendered (reachable via `source`).
    Store(E),
}
```

`Display` and `core::error::Error` (with `source() -> Some(e)` for `Store`) are written by
hand, mirroring `faulty.rs:130-153`. The port methods add no error type of their own:
`type Error = S::Error`.

### Send and `!Send` (D3)

- **One type, implementing the bare flavour only**, as `FaultyStore` does, with
  `impl<S: EventStore> EventStore for WithholdingStore<S>`.
- The futures are `Send` only when the compiler can see that. This flavour needs no `Send`
  for the event-store or model families. The concurrency family needs only
  `Store: Send`, which holds when `S: Send`, since the fields are `S` and an
  `Arc<RwLock<Arc<_>>>`.
- A `SendWithholdingStore` sibling, which `trait_variant`'s blanket impl forces to be a
  second *type* (E0119, `faulty.rs:3-12`), is deferred to whoever needs one (most likely
  #156's ingest path through `happenstance-sync`'s `SendPeer`). Adding it later is additive.

### What the instrument cannot model, stated on its page

- **Ingest deduplication.** The inner store still holds what is withheld. An `IngestStore`
  over the inner store would deduplicate a re-delivered withheld event (VT-8) where a
  physically pruned store would re-admit it. This is #156's to account for.
- **Orphan rule for #156.** `IngestStore` lives in `happenstance-sync`, which the testkit
  cannot depend on. A third crate cannot implement a foreign trait for this foreign type,
  so #156 wraps it in a newtype.
- **ES-38 against this instrument proves little.** The inner store never forgets, so
  non-reuse holds by delegation. #151's teeth are the real adapters through #147.

### The fixture, the mounts, the rule, and the record (test crate `tests/completeness_instrument.rs`)

The crate is gated `#![cfg(all(not(target_arch = "wasm32"), feature = "memory"))]`, because
it uses threads, `catch_unwind` and `MemoryFixture`. Layout follows the
`tests/mutation_coverage.rs` + `tests/mutation_coverage/` precedent: no `#[path]`, no
`mod.rs` (ARC-06).

```
tests/completeness_instrument.rs            crate root: docs, mods, the rule, the three rejection tests,
                                            event_store_conformance! and event_store_concurrency_conformance!
                                            mounts, and event_store_model_conformance! under cfg(feature = "proptest")
tests/completeness_instrument/fixture.rs    PrunedMemoryFixture
tests/completeness_instrument/accuracy.rs   Subject<D>, the Defect trait, Inaccuracy, report_is_accurate(), the scenarios
tests/completeness_instrument/wrong.rs      the three Defect impls (wrong instruments)
tests/completeness_instrument/record.rs     Probe, Recorded, run_families(), render/parse of pass_list.txt
tests/completeness_instrument/pass_list.txt the recorded pass list (committed; include_str!)
```

**`PrunedMemoryFixture`** holds `store: WithholdingStore<MemoryHandle>`.
- `async fn pruned() -> Self`:
  1. Make `MemoryFixture::new()` and `connect().await` to it.
  2. Wrap the handle.
  3. Seed it with three batches of 1, 2 and 3 events
     (`fixtures::tagged_event("Pruned", &[("pruned", "yes")])` and siblings, varied types).
  4. Call `withhold_where(|_| true)`.
  5. **Assert** the returned positions are six, strictly increasing, and equal to
     `withheld()`.
- `connect()` returns `ready(self.store.clone())`, a second handle onto the same
  instrument. So it declares `SECOND_HANDLE` supported.
- `REOPEN`, `MID_BATCH_FAULT` and `READ_FAULT` are each declined with a stated reason, so
  CF-18's declension check passes:
  - `REOPEN`: MemoryEventStore has no durable medium, and the withheld set is process state
    besides.
  - `MID_BATCH_FAULT`: the inner store extends one `Vec` under one lock.
  - `READ_FAULT`: the inner store iterates a `Vec` it already holds. A read fault through
    the instrument is shown with `FaultyStore` as the inner store in the instrument tests.
- `READ_YOUR_OWN_WRITES` stays at its default, supported.

**Why the suite runs against "pruned to nothing" and cannot run against a scattered
survivor set.** This finding goes on record in the `pass_list.txt` header and the crate
docs.
- Every rule starts by assuming a fresh store, and many read `Query::all()`:
  `reading_an_empty_store_yields_nothing`, `head_of_an_empty_store_is_none`,
  `query_all_matches_every_event`, `condition_against_an_empty_store_admits_the_append`.
- Any pre-rule survivor is therefore visible, and it fails those rules for a reason that
  has nothing to do with pruning.
- So the strongest staging is a store whose whole prior history is withheld. Holes at
  arbitrary positions inside a rule's own history would need the rule's cooperation, which
  is #151's and #154's job, not the suite's.
- That is exactly CF-27's claim: a pruned store and a young store are the same value at
  every seam.
- As a side effect, the run checks the house rule against literal positions: every rule
  starts above six withheld positions.

**The rule: `#[test] fn instrument_report_is_accurate_and_the_suite_cannot_tell()`.** It is
a plain `#[test]`, because it drives with `block_on` and `catch_unwind`. Its body has two
halves.

1. **The suite cannot tell.** `record::run_families()` builds
   `Vec<Probe { family: &'static str, name: &'static str, run: fn() -> RuleOutcome }>`:
   - an emitter `macro_rules!` passed to `for_each_event_store_rule!`, in which each `run` is
     a non-capturing closure
     `|| block_on(happenstance_testkit::rules::$name(pruned_fixture))`
     (RS-60-1: a non-capturing closure is a `fn()`, hence `UnwindSafe`, and the fixture is
     opened inside);
   - the same with `for_each_concurrency_rule!` and `happenstance_testkit::concurrency::rules`;
   - under `cfg(feature = "proptest")`, `for_each_model_rule!` and
     `happenstance_testkit::model::rules`.

   Here `async fn pruned_fixture() -> PrunedMemoryFixture { PrunedMemoryFixture::pruned().await }`.
   Each probe runs under `std::panic::catch_unwind(probe.run)` and maps to
   `Recorded::{Ran, Skipped { capability }, Failed { message }}`. No quiet panic hook: a
   panic here is unexpected and should print. The half then asserts:
   - **(a)** no `Failed`, listing every failed rule and its message;
   - **(b)** the rendered record equals the committed `pass_list.txt`, ignoring `#`
     comments and blank lines, and comparing only the families compiled in. On a mismatch
     the message prints the full expected file content, to paste after review. There is no
     environment-variable bless and no writing files from tests (H-12: a changed
     expectation is reviewed).

   The record format, one line per rule in registry order:
   `<family> ran <rule>` or `<family> skipped <rule> <CAPABILITY>`.
   Skip reasons are not recorded, because they are this fixture's own text and live in
   code.
2. **The report is accurate.** Run `accuracy::report_is_accurate::<Correct>()` over four
   scenarios and assert each is `Ok(())`.
   - **S1, suffix.** Append 6 events in 3 batches. Withhold the first 3 assigned positions
     in one call.
   - **S2, scattered, in two calls.** Append 8 events, tags alternating `claim:a` and
     `claim:b`.
     - Call 1 withholds `assigned[1]` and `assigned[3]`.
     - Call 2 withholds `assigned[5]`.
     - Survivors sit between the holes: `[0, 2, 4, 6, 7]`.
   - **S3, nothing withheld.**
   - **S4, everything withheld.**

**`accuracy.rs`** follows RS-60-3: one defect is one overridden default method, selected by
a `PhantomData` marker.

```rust
trait Defect: 'static {
    const NAME: &'static str;
    /// What the instrument reports. The default is the real report.
    fn report(subject: &Subject<Self>) -> BTreeSet<SequencePosition> where Self: Sized { subject.real.withheld() }
    /// Membership. The default is the real instrument's.
    async fn contains(subject: &Subject<Self>, id: EventId) -> Result<bool, MemoryStoreError> where Self: Sized {
        EventStore::contains_event_id(&subject.real, id).await
    }
}
struct Subject<D> {
    real: WithholdingStore<MemoryHandle>,
    inner: MemoryHandle,                     // a clone of the same handle, for the leaky defect
    calls: RefCell<Vec<Vec<SequencePosition>>>, // withhold calls in order, for the last-call defect
    _defect: PhantomData<D>,
}
impl<D: Defect> Subject<D> {
    async fn withhold(&self, positions: &[SequencePosition]);   // record, then real.withhold(..).await.expect(..)
    fn report(&self) -> BTreeSet<SequencePosition> { D::report(self) }
}
impl<D: Defect> EventStore for Subject<D> { /* read/append/head -> real; contains -> D::contains */ }
struct Correct; impl Defect for Correct { const NAME: &str = "correct"; }

#[derive(Debug, PartialEq, Eq)]
enum Inaccuracy {
    /// Reported withheld, but the store never assigned it.
    ReportedButNeverAssigned(SequencePosition),
    /// Reported withheld, while `read` still yields it (CF-27 Rejects, first half).
    ReportedWithheldButReadable(SequencePosition),
    /// `read` no longer yields it, but the report calls it retained (CF-27 Rejects, second half).
    DroppedButReportedRetained(SequencePosition),
    /// `contains_event_id` disagrees with the report for this event.
    MembershipDisagreesWithReport { position: SequencePosition, answered: bool },
    /// The instrument hid a different set from the one it was asked to hide.
    WithheldOtherThanAsked { asked: BTreeSet<SequencePosition>, hidden: BTreeSet<SequencePosition> },
}

/// One scenario. `assigned` is read through the subject BEFORE any withholding (the
/// inner store's whole log). `asked` is the union of every position passed to `withhold`.
async fn check<D: Defect>(subject: &Subject<D>, assigned: &[SequencedEvent],
                          asked: &BTreeSet<SequencePosition>) -> Result<(), Inaccuracy>;
fn report_is_accurate<D: Defect>() -> Result<(), Inaccuracy>;   // runs S1..S4 under block_on; first Err wins
```

The oracle shares no subroutine with the instrument (RS-60-4):
- `hidden = assigned positions − positions of collect(read(Query::all(), ReadOptions::new()))`;
- `report = subject.report()`.

The checks run in this order, and the first failure is returned:
1. every reported position is in `assigned`;
2. every reported position is in `hidden`;
3. every hidden position is in `report`;
4. `contains(id)` is `!report.contains(position)` for every assigned event;
5. `hidden == asked`.

**`wrong.rs`**, the three wrong instruments, each a plausible bug:
- **`RangeReport`** overrides `report`. It reports `min..=max` of the real report, mapped
  through `SequencePosition::new`: the floor or range thinking ES-39 rejects. On S2 the
  survivors `assigned[2]` and `assigned[4]` fall inside the range and are still readable.
  Expected result: `Err(ReportedWithheldButReadable(assigned[2].position))`.
- **`LastCallReport`** overrides `report`. It reports only the most recent call's
  positions, the "assign instead of extend" bug. On S2 the positions `assigned[1]` and
  `assigned[3]` are hidden but unreported. Expected result:
  `Err(DroppedButReportedRetained(assigned[1].position))`.
- **`LeakyMembership`** overrides `contains`. It asks `subject.inner`, the decorator that
  forgot ES-41. The suite cannot catch this, because it never withholds. Expected result:
  `Err(MembershipDisagreesWithReport { .., answered: true })`.

The rejection tests in the crate root are `range_report_is_rejected`,
`last_call_report_is_rejected` and `leaky_membership_is_rejected`. Each one asserts
`report_is_accurate::<Wrong>()` returns that exact variant. The expected position comes from
the scenario's `assigned` list, never from a literal: `report_is_accurate` returns it in the
error. So assert on the variant with `matches!`, plus a relation check: the position is
strictly between two withheld positions in S2. The report half of the rule calls the same
`report_is_accurate`, so the wrong instruments are rejected *by the rule's own code path*.

### Spec reconciliation (D5)

Run `cargo xtask spec-trace --write`, then `cargo xtask spec-trace`.

- **CF-27, `Rule:` line** (spec ~9883). Add that the rule is
  "a meta-test in `crates/happenstance-testkit/tests/completeness_instrument.rs`, built at
  #144, with the recorded pass list in
  `crates/happenstance-testkit/tests/completeness_instrument/pass_list.txt`".
  - Why: `rule_cell` (`xtask/src/spec_trace.rs:2105-2150`) daggers every name not in
    `RULE_FILES`. A `†` there would now assert "does not exist yet" about a written test.
  - The words "meta-test" set `rule_elsewhere` (`spec_trace.rs:2599`), so §7.2 renders the
    clause's prose instead.
  - `has_suite` excludes `CF-` (`spec_trace.rs:2731`), so check 4 does not look for it, and
    `UNRESOLVABLE_RULE_NAMES` is untouched.
  - Keep the `[DEFERRED …]` bracket. #163 freezes it.
- **§6.5 portfolio, Completeness row** (spec ~9945). Change the lead from
  `**No. Planned for phase 14**` to `**Fixture yes, adapter no.**` and add one sentence
  naming `WithholdingStore` and the record. The checker's vocabulary has that exact lead
  (`portfolio.rs:476`), and CF-26 counts a fixture as no adapter, so no freeze-ledger
  verdict changes.
- **§5, "Blocked on the completeness instrument (CF-27)"** (spec ~5114). Restate it: the
  instrument exists, and the two rules are now unwritten rather than blocked, owed by #151
  (ES-38) and #154 (ES-40). Keep the heading's text recognisable, or strike it through in
  the style of its neighbours (`~~…~~`).
- **Leave ES-38, ES-39 and ES-40's prose alone**, except that the phrase "That instrument
  does not exist" in ES-38's `Rule:` text is now false.
  - ES-38 is `[FROZEN]`. Changing a frozen clause's *content* needs an ADR, but correcting
    a stale statement of fact in its rule commentary is reconciliation.
  - **Recommend:** leave it for #151, which rewrites that `Rule:` line anyway, and note it
    in the PR body. Do not touch a `[FROZEN]` clause in this leaf.
- **`UNRESOLVABLE_RULE_NAMES`** in xtask: no change. ES-38, ES-39 and ES-40's
  `Scheduled(...)` entries stay true, because those rules are still unwritten.

### Docs and CHANGELOG

- **`lib.rs`**, the "Stores that misbehave on purpose" table (`lib.rs:394-414`):
  - add a row: `WithholdingStore` | a log with an arbitrary retained set, whose withheld
    events no port value reveals | a runner or ingest path that infers completeness from
    any port value (ES-39, ES-40);
  - fix the counts in the surrounding prose ("These four", "All three run");
  - say that it runs the full suite, pruned.
- **`WithholdingStore` rustdoc** (RS-70-x, H-15). It needs:
  - a summary;
  - **Rejects:** a reader that infers completeness from a port value;
  - why it is a decorator and not its own log (RS-70-5: the alternative that lost, said
    once);
  - the soundness argument for conditions;
  - "what it cannot model";
  - `# Examples`, a doctest over `MemoryEventStore`:
    1. append three events;
    2. withhold the middle one by position;
    3. show that `read` skips it, that `withheld()` names it, and that a condition matching
       only it admits the append;
    4. use relations, not literals.
- **Every pub fn** gets `# Errors` where it is fallible. `withhold_where` gets a doctest
  showing the suffix spelling.
- **`CHANGELOG.md` `[Unreleased]`**, under `happenstance-testkit` / Added:
  `WithholdingStore`, `WithholdError`, CF-27's completeness instrument, the recorded pass
  list. Additive: no rule added to any family, so no adapter goes red.

### Alternatives considered (one line each)

- **The report as an `EventStore` method.** Refused by ADR-0028.
- **The report as a public `Completeness` trait in the testkit.** "Its own inherent API" is
  the brief, and a trait is surface that nothing yet needs to be generic over.
- **The report as `Vec<RangeInclusive<SequencePosition>>`.** Ranges over integers
  misdescribe gaps, and that is precisely the `RangeReport` defect.
- **The rule as a `pub async fn` in `suite.rs`.** It would be mounted against every adapter,
  but its subject is the instrument, not the adapter. It would also need a mutant
  (`every_rule_has_a_mutant`) and a registry entry.
- **Auto-bless of `pass_list.txt` through an environment variable.** Rejected: a changed
  expectation is a reviewed edit (H-12).

## Test list (drive in this order)

Instrument behaviour tests go in a new test crate,
`crates/happenstance-testkit/tests/withholding_store_instruments.rs`, gated
`cfg(all(not(wasm32), feature = "memory"))` with `#[tokio::test]`, following the
`gappy_store_instruments.rs` precedent. The inner store is `MemoryEventStore` or
`MemoryHandle` unless a test says otherwise. No literal positions anywhere.

- [x] 1. `a_fresh_instrument_withholds_nothing_and_delegates`: `withheld()` is empty, `read`
  is empty and `head` is `None` over an empty inner store. *(first red: compile error,
  `WithholdingStore` does not exist)*
- [x] 2. `withholding_nothing_is_the_inner_store`: after 4 appends, `read` under several
  `ReadOptions` (including `limit(2)`, `backwards`, `from` and `to`) equals the inner
  store's results (invariant 9).
- [x] 3. `a_withheld_position_is_not_read`: withhold `assigned[1]`. `read(all)` is
  `assigned` minus it, and `withheld()` is `{assigned[1]}`.
- [x] 4. `withholding_is_cumulative_and_the_report_is_the_union`: two calls, the report is
  the union, and both are hidden.
- [x] 5. `withholding_an_unheld_position_is_refused_and_withholds_nothing`: request
  `[assigned[0], head.next()]`. The result is
  `Err(WithholdError::NotHeld { position })` with `position == head.next()`, and `withheld()`
  is still empty (invariant 10).
- [x] 6. `rewithholding_a_withheld_position_is_a_no_op`.
- [x] 7. `limit_counts_only_retained_events`: withhold the first of four, read with
  `limit(2)`, and get `assigned[1..3]`. Also cover `limit(0)`.
- [x] 8. `backwards_from_and_to_compose_with_withholding`.
- [x] 9. `head_skips_a_withheld_top`: withhold the last assigned event, and `head` is the one
  before it. `head_is_none_when_everything_is_withheld`.
- [x] 10. `membership_is_false_for_a_withheld_event_and_true_for_a_retained_one` (ES-41,
  store-relative).
- [x] 11. `positions_after_a_withheld_top_are_never_reused`: withhold the top, append, and
  the new position is greater than every withheld position (ES-38 shape, relation only).
- [x] 12. `a_condition_matching_only_withheld_history_passes_vacuously` (ES-40 behaviour;
  #154 writes the named rule): tag `k` lives only on a withheld event, and a conditional
  append over `k` with `after = None` is `Ok`.
- [x] 13. `a_retained_match_below_the_highest_withheld_still_rejects` (the pre-check path):
  the retained `A(k)` is below the withheld `B`. The result is `ConditionViolated` at
  `A.position`, and nothing was written (`head` unchanged).
- [x] 14. `a_retained_match_above_the_highest_withheld_still_rejects` (the inner path): the
  withheld `B` is below the retained `C(k)`, and the result is `ConditionViolated` at
  `C.position`.
- [x] 15. `guards_keep_independent_boundaries_over_a_holed_log`: two guards, the first
  matching only withheld history and the second matching a retained event beyond its own
  `after`. The append is rejected at the second guard's match. Changing the second guard's
  `after` past that match admits it.
- [x] 16. `an_empty_batch_is_refused_as_no_events_before_any_precheck`: a withheld set plus
  a condition that a retained event below `W` violates. The result is
  `Err(AppendError::NoEvents)`.
- [x] 17. `withhold_where_selects_a_scattered_set_and_returns_it_ascending`: withhold by tag
  `claim:a` across interleaved claims. The return value is strictly ascending, equals the
  `claim:a` positions, and leaves survivors between the holes. A second call returns only
  the newly withheld positions.
- [x] 18. `clones_share_one_withheld_set`: withhold through one clone, and the other's read
  hides the event.
- [x] 19. `inner_read_errors_pass_through` and `a_failed_inner_read_fails_withhold`: the
  inner store is `FaultyStore::new(MemoryEventStore::new()).fail_next_read(1)`. The read
  yields `Err(FaultyStoreError::Injected)` and `withhold` gives
  `Err(WithholdError::Store(FaultyStoreError::Injected))`. Then
  `WithholdError`'s `Display` and `source()`.
- [x] 20. `withholding_store_is_send_and_sync_over_a_send_inner`: a compile-time assertion
  (RS-61) with `fn assert_send_sync<T: Send + Sync>() {}` over
  `WithholdingStore<MemoryHandle>`.
- [x] 21. Doctests on `WithholdingStore` and `withhold_where` (RS-62).

Then the CF-27 crate, `tests/completeness_instrument.rs`:

- [x] 22. `PrunedMemoryFixture::pruned` withholds six seeded events, and
  `withheld() == returned`. Assert this inside the fixture constructor.
- [x] 23. `event_store_conformance!(PrunedMemoryFixture::pruned().await)` is green. The
  whole family passes, including CF-18's declension check.
- [x] 24. `event_store_concurrency_conformance!(…)` is green, and
  `event_store_model_conformance!(…)` is green under `proptest`.
- [x] 25. `range_report_is_rejected` gives `ReportedWithheldButReadable`, a survivor inside
  the range. *(Write this before the rule's accuracy half, to watch `check` go red, then
  green.)*
- [x] 26. `last_call_report_is_rejected` gives `DroppedButReportedRetained`.
- [x] 27. `leaky_membership_is_rejected` gives `MembershipDisagreesWithReport { answered: true, .. }`.
- [x] 28. `instrument_report_is_accurate_and_the_suite_cannot_tell`: both halves. Write it
  first with an empty `pass_list.txt` and watch the record half go red with the printed
  record. Review that record (every `ran` or `skipped`, and the skips match the declined
  capabilities `REOPEN`, `MID_BATCH_FAULT` and `READ_FAULT`), then commit it.
- [x] 29. Property test (`cfg(feature = "proptest")`, in `accuracy.rs`): for any `n` in
  `1..=12` events, any withheld mask, and any split of the mask into 1 to 3 calls,
  `check::<Correct>` is `Ok`. Use proptest's own `vec(any::<bool>(), n)`. The testkit's
  strategies generate events, not masks.

Then the gate:

- [x] 30. Spec edits, then `cargo xtask spec-trace --write`, then `cargo xtask spec-trace`.
- [x] 31. `lib.rs` table, rustdoc, CHANGELOG.
- [ ] 32. `CARGO_INCREMENTAL=0 cargo xtask ci`. This covers the wasm32 testkit check,
  `--no-default-features` (the new test crates must drop cleanly), the doc builds, and
  `lint-clock`.

## Decisions (H-16)

| # | Decision | Options | Recommendation | Door | Decided by | Outcome |
|---|---|---|---|---|---|---|
| D1 | Public names (published crate) | A) `WithholdingStore` + `WithholdError` + `withhold` / `withhold_where` / `withheld`; B) `CompletenessStore` (CF-27's noun); C) `PrunedStore` | **A.** Named by behaviour, like `FaultyStore`, `GappyMemoryStore` and `RecordingProjectionStore`. "Pruned" wrongly implies removal from the inner store. | Two-way until a release ships it; a rename after that is a major | owner (public API) | A (delegation, 2026-11-15) |
| D2 | Withholding mechanism | A) own log; B) physical removal from the inner store; C) decorator plus condition rewrite (pre-check at or below `W`, raise every guard's `after` to `W`) | **C**, for the soundness argument in Design. B puts ES-38's wrong implementation inside the instrument. | Two-way (private internals) | architect | as recommended |
| D3 | Flavours | A) `!Send` type only now; B) add `SendWithholdingStore` now | **A.** Nothing in #144 needs `Send` futures. #156 adds the sibling if its ingest path does, and adding it is additive. | Two-way | architect | as recommended |
| D4 | Where the pass list lives | A) committed `pass_list.txt`, asserted equal by the rule; B) a const array in the test; C) the CF-27 clause text | **A.** Reviewable diffs, cannot rot: any new rule fails the rule until the file is updated in the same change. **Cost:** every future suite rule touches this file. | Two-way | owner (process friction on every future rule) | A (delegation) |
| D5 | Spec reconciliation in this leaf | A) CF-27's `Rule:` line names the meta-test (removes a `†` that would now be false), the portfolio row becomes "Fixture yes, adapter no", and the §5 note is restated; markers untouched, ES-38's `[FROZEN]` prose left for #151. B) No spec edits, all deferred to #163 | **A.** Without it §7.2 asserts the written rule "does not exist yet". | Two-way | owner (spec wording) | A (delegation) |
| D6 | Families recorded | A) event-store + concurrency + model (model under `proptest`); B) event-store only | **A.** "The full suite" read honestly. All three bind `Fixture`, and the concurrency family is the one that exercises the atomic half of the condition rewrite. | Two-way | architect | as recommended |
| D7 | Issue #144 carries `status:blocked` | confirm unblocked (ADR-0028 accepted at phase 17; ADR-0027 took SY-27 off it), or name the blocker | Confirm before starting | Authority-bound (triage) | owner | label stale; proceed (delegation) |

Nothing here is one-way except D1 *after* a release publishes the names. No ADR is
required: ADR-0028 already fixed the report as instrument-local.

## Risks

- **A suite rule might fail against the pruned fixture.** Two likely causes:
  - it assumes the first position, or asserts a literal one;
  - the model family's reference model assumes a fresh allocator.

  Either is a finding, not a reason to bend the instrument:
  - fix the rule, and explain it in the same change (CLAUDE.md, "the rule that matters");
  - if the fix is to a suite rule's expectation, that is an H-16 decision. Stop and ask.

  The gappy suite already proves no rule assumes density from 1, so the residual risk is a
  rule that assumes `head == None` means "no position ever assigned". The `Failed` listing
  in the record half names it.
- **The concurrency family and the pre-check.** In the suite, `W` sits below every rule's
  events, so the pre-check reads only withheld rows and the race is wholly the inner store's.
  The pre-check-path race (retained rows at or below `W` under contention) is argued, not
  raced. Test 13 covers its logic single-threaded. If the owner wants it raced, add a
  `thread::scope` test over S2 with 8 contenders, still with no sleeps (TST-10).
- **`catch_unwind` around a rule that spawns threads.** The concurrency rules use
  `thread::scope`, which re-raises a contender's panic in the caller, so `catch_unwind`
  sees it. That is the existing precedent: `mutation_coverage/harness.rs` already runs the concurrency family under `catch_unwind` through its `probes` table.
- **Record churn.** Any PR adding a suite rule must update `pass_list.txt` (D4). The
  failure message prints the replacement content, which keeps the churn cheap.
- **Fidelity gaps** (ingest deduplication, ES-38 by delegation) are stated on the type's
  page so that #151 and #156 do not over-read the instrument.
- **`withheld()` returns an owned `BTreeSet`.** This deviates from OWN-13, and the reason
  is in the doc. If a reviewer objects, the alternative is a `Withheld` snapshot newtype
  with `iter()` / `contains()`. That is more surface for no current caller.

## Size estimate (lower bound)

| File | New/changed | ≈ lines |
|---|---|---|
| `src/withholding.rs` | new | 450 (≈ 200 code, the rest rustdoc and the soundness comment) |
| `src/lib.rs` | changed | +20 |
| `tests/withholding_store_instruments.rs` | new | 380 |
| `tests/completeness_instrument.rs` | new | 160 |
| `tests/completeness_instrument/fixture.rs` | new | 110 |
| `tests/completeness_instrument/accuracy.rs` | new | 260 (including the proptest) |
| `tests/completeness_instrument/wrong.rs` | new | 90 |
| `tests/completeness_instrument/record.rs` | new | 140 |
| `tests/completeness_instrument/pass_list.txt` | new | about 115 (about 105 rules + header) |
| `spec/SPECIFICATION.md` (+ regenerated §7.2) | changed | +15 / −5 |
| `CHANGELOG.md` | changed | +12 |

That is **11 files and about 1,750 lines** at the lower bound, of which about 470 are
production code with docs.

## Progress

- [x] Plan written (this file). D1-D7 settled as recommended by the delegation.
- [x] Module written ahead of its tests (deviation from TST-01, stated): the design was
  fully specified. Red evidence instead by mutation, each killed by
  `withholding_store_instruments`: no pre-check (1 failed), condition not raised (2),
  inner `limit` not widened (2), membership delegated (1), `head` delegated (2).
- [x] CF-27 record half red first with a header-only `pass_list.txt`
  (`instrument_report_is_accurate_and_the_suite_cannot_tell`: "the record differs");
  the printed record was reviewed (8 skips: REOPEN x3, HELD_ABOVE_FRONTIER, MID_BATCH_FAULT x2,
  the five MAX_* limits, READ_FAULT; no failure) and committed.
- [x] Suite half bites: with the condition not raised, `model ops_agree_with_the_model`
  is listed as failed. Finding: no event-store rule fails that mutant, because the
  seeded (withheld) events match no rule's own query; the model family's `Query::all`
  conditions are what catch it.
- [x] Spec edits line-neutral; spec-trace --write then spec-trace green.
