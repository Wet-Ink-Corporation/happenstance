# Exploration: happenstance as the orchestration layer for a Crux app core

Status: **exploration notes**. Nothing here is normative. No clause IDs are claimed.
Date: 2026-08-07. **Revision 3** — §1 records two corrections made in conversation and what each
changed, because both were load-bearing and both were wrong for instructive reasons.
Grounded against: `happenstance` @ `D:\repos\happenstance` (SPECIFICATION 2026-08-06, 193 clauses);
`crux_core` 0.20.0 (2026-08-06); the Photoroom series parts 1–5 (Jan 2026); dcb.events.

---

## 0. Thesis

Crux's core is a **synchronous, driven, side-effect-free state machine**. A DCB decision is
**a query, a fold, a decision, a guarded append** — a pure function plus two I/O moments, with no
persistent identity to load, lock, or rehydrate between calls. Aggregates would fight Crux's shape;
DCB does not.

The topology is **N peer stores, each authoritative for its own log, none authoritative for the
system.** One may happen to be a server. Reads are local projections; writes are local appends; the
network is `SyncPeer` and nothing else.

The store sits **behind an effect, with a Rust handler in the same binary** — not in the `Model`,
and not in Swift or Kotlin. That one sentence resolves most of the tension: the core stays pure and
portable, the store stays a real conformance-tested adapter, and DCB semantics are never
reimplemented per platform.

The hard part is not offline. **There is no offline.** Every store only ever knows its own log plus
what has arrived, and that is the steady state, not a degraded mode. So the two questions that
generate the design are: an `AppendCondition` is scoped to one store by construction and therefore
is not a distributed lock; and the read path, being local projections everywhere, must produce the
same answer on every device from a set of facts that arrived in a different order on each.

---

## 1. Two corrections, and what they changed

Both are recorded rather than quietly edited out, because the errors are the kind a reader would
otherwise repeat.

### 1.1 Revision 1 → 2: the smuggled-in authority

Revision 1 imported Photoroom's reconciliation model, and **Photoroom is server-authoritative** —
their backend accepts, drops, or *amends* each incoming diff, and clients rebase unacknowledged work
onto an authoritative prefix. Taking rebase as the reconnect story imported an authority, and
everything downstream inherited it: "confirmed prefix", "pending", "the server validates
client-authored facts."

SY-19's *Rejects* names the error exactly:

> *Rejects:* the belief that DCB's per-store total order extends across a peer set. […] the same
> fourteen events sit at vessel position 38,102 and depot position 3,918,442, each store internally
> correct, each peer's total order naming the other as the loser.

An authoritative prefix is that belief wearing a hat. Consequences: there is **no reconnect**,
because there is no connected state to return to — partition is permanent and normal, so the guard's
scoping to one store is what the guard *is*, not a failure mode. The **log is never rewritten**;
ingest only appends (SY-1..7); no unwind, no two-tier log, no rebase machinery. What changes when
facts arrive is the *projection*, not the history. **Compensation is domain code**, and in Crux it is
cheap: a projection notices a violated invariant during ingest, `update` fires an internal Intent,
the domain appends a compensating fact. No saga, no coordinator.

Withdrawn: revision 1's claim that "rebase requires Intents, not Facts." The Intent/Fact split
survives on weaker but sufficient grounds (§2).

### 1.2 Revision 2 → 3: the store does not live in the `Model`

Revision 2 put a `MemoryEventStore` in `App::Model` and had `update` run the whole DCB transaction
synchronously. That was a **testing configuration promoted to an architecture**. In production the
store is SQLite — rusqlite natively, a wasm build on the web — and:

- rusqlite is blocking file I/O, called from `update`, which the shell invokes synchronously and
  frequently on the UI thread. That is an fsync on the render path.
- There is **no synchronous filesystem on the web main thread**. Store-in-`Model` survives contact
  with iOS and Android and dies on the platform Crux exists to include.

The fix is not a synchronous store. It is that revision 2 rejected the store-as-effect option for the
wrong reason — "you would implement DCB query semantics in Swift and Kotlin." That is only true if
the operation crosses the **FFI** boundary. It need not. It crosses the **effect** boundary, and the
handler on the far side is Rust, in-process. §4 is rewritten around this.

---

## 2. Terminology, because the two vocabularies collide

| Term | What it is | Lives in | Crosses FFI |
|---|---|---|---|
| **Intent** | Crux `App::Event`. A user action, or an internal callback. Finite, UX-shaped, versioned with the app, discarded once handled. | Transient | Yes: `Facet`, `Serialize`, `#[repr(C)]` |
| **Fact** | happenstance `Event`. `EventType` + `Tags` + opaque `data`. Immutable, replicated, forever. | The log | No — opaque `Bytes` between peers, never through typegen |
| **Decision model** | The transient fold a `Decider` builds to evaluate one Intent | The stack, for one decision | Never |
| **View model** | Crux `ViewModel`. A convergent projection | Derived from `Model` | Yes |

The split is not optional, for three independent reasons: the FFI demands `Facet + repr(C) +
Serialize` while the log demands opaque bytes; the UX vocabulary churns at app-release speed while
facts are permanent; and Photoroom's part 3 records what happens when the input operations *become*
the output protocol — "we would just end up reflecting the input operations back to the shell, and
not solving the problem at all."

---

## 3. Topology

```
   ┌── Store A ──┐        ┌── Store B ──┐        ┌── Store C ──┐
   │ Crux core   │◄──────►│ Crux core   │◄──────►│  Worker /   │
   │ phone       │        │ laptop      │        │  DO / pg    │
   │ sqlite      │  sync  │ sqlite      │  sync  │  sqlite     │
   │ local proj. │        │ local proj. │        │ local proj. │
   └─────────────┘        └─────────────┘        └─────────────┘
```

Every node is the same shape. **A server, where one exists, is a peer with better uptime and more
disk** — not an authority over decisions. What it is genuinely for is §9. Two phones and no server is
a supported configuration and should be the one this is tested against, because it removes every
place an authority could hide.

The application's entire network API is `SyncPeer`. No endpoints, no request/response schema, no
per-feature route to version; feature work never touches the transport. That is the elegance the
architecture is reaching for, and §9.2 states its price.

---

## 4. Where the seam goes

Crux ports are **data** (`Operation` + `Output`). happenstance ports are **async traits**. The
mistake to avoid is assuming those meet at the FFI. They meet at the *effect*, and Crux offers three
lanes for an effect, only one of which reaches a foreign shell:

| Lane | Handled by | Reaches Swift/Kotlin? |
|---|---|---|
| `Serialized` (the bridge) | the shell, over bincode | yes |
| `EffectMiddleware` / effect-router Rust handler | **Rust, in-process, on a real runtime** | no |
| `Buffer` | the caller, drained synchronously | no |

`map_effect::<Effect>()` narrows the FFI effect enum after middleware has consumed a variant, so
`StoreOperation` never reaches the shell and **typegen never emits it**. `examples/counter-routing`
does exactly this with a `Random` effect. And the book states the intent plainly: *"If you want to
depend on a crate that requires a standard runtime like Tokio, you can integrate it through an effect
via middleware."* rusqlite plus a runtime is precisely that case.

### 4.1 The shape

```
Core (pure, sync, portable)      Effect handlers (Rust, async, same binary)     Shell
───────────────────────────      ──────────────────────────────────────────     ─────
update() -> Command              happenstance-sqlite   (native / wasm)          Render
Decider + retry loop             happenstance-* projection store                ViewModel
sync runner (policy)             peer transport (ws / http)                     user input
projections resident in Model    tokio  /  spawn_local
StoreId, watermarks
```

Where each port lands, and why:

- **`EventStore` → effect, Rust handler.** Query matching and conditional append are the store's job
  and belong with the database. `happenstance-sqlite` stays a real, conformance-suite-passing adapter
  rather than being reimplemented per platform.
- **`SyncPeer` → effect for the *transport*, runner in the core.** The spec already says policy —
  fan-out, ordering, merge — belongs to a runner above the port. Watermark bookkeeping, peer
  selection, and what to do with a failed push are core logic; frames on a socket are the effect.
  This is Photoroom's sans-I/O Phoenix client, hosted on WebSocket and Time capabilities, and it is
  what let their tests stand on both sides of the client.
- **`ProjectionStore` → effect, or projections resident in `Model`.** §5.6.
- **`Render` → the shell.** The only effect that was ever the shell's.

### 4.2 Why this is the *more* faithful reading of the ethos, not a compromise

The invariant Crux enforces is purity of `update`, not minimality of `Model` — `App::Model` has no
trait bounds at all, and 0.17.0 deliberately *removed* the `Default` requirement and added
`Core::new_with`. But purity of `update` is exactly what store-as-effect buys and store-in-`Model`
spends. Three side effects hide inside a real store, and all three are handled correctly once it sits
on the effect side:

- **A clock.** VT-9 requires a store to stamp every locally-appended event with a `RecordedAt`. A core
  cannot read a wall clock. On the handler side this is a non-issue.
- **An ID source.** `StoreId` is 128 opaque bits. Generating them is a side effect; it happens once,
  outside the core, and arrives at `Core::new_with` (G1).
- **Durability.** ES-35 requires acknowledged writes to survive a reopen. `append` returning `Ok` now
  means a committed transaction, not "accepted into a working set."

The decision logic is untouched by any of this: it is still pure Rust in the typed layer, reading
what it is given and returning facts. It simply runs inside a `Command` rather than inside `update`.

**One constraint to notice**: `Core<A>` implements `Layer` only where `A: App + Send + Sync + 'static`
and `A::Model: Send + Sync + 'static`. Using middleware therefore imposes `Send + Sync` on the Model —
no `Rc` in projections. Livable, but it must be designed for, and the newer `effects::EffectRouter`
may relax it.

### 4.3 What it costs

- **Two round-trips per decision** (read, append) instead of a synchronous fold. These are in-process
  channel hops to a Rust handler, not FFI — sub-millisecond. Budget for a family of internal Intents
  carrying results; mark them `#[serde(skip)] #[facet(skip)]` with non-`Facet` payload fields
  `#[facet(opaque)]`, which is the standard escape hatch.
- **Two in-flight decisions can interleave and both read stale state.** That is exactly what
  `AppendCondition` is for, and the retry loop lives in the typed layer. DCB doing its job, not a leak.
- **`view()` cannot fold the log**, because the log is behind an effect. §5.6.
- **Commands cannot touch the `Model`** — a Crux invariant, not a happenstance one. Every result
  round-trips as an internal Intent, which is also what keeps log mutation inside `update` where it
  belongs.

### 4.4 What this retires

- The scale ceiling. The store pages; `ReadOptions` does what it was designed for; VT-27/E2E-04's
  one-`ReadOptions`-per-`Query` limitation stops being architectural and becomes an ordinary query
  concern.
- The need for a synchronous decision path as *boundary enforcement*. A blocking flavour is still
  worth having as a test and CLI convenience (G2), but nothing rests on it.
- `MemoryEventStore` as an architectural component. It demotes to two better roles: the test double
  that effect resolutions are driven against, and possibly a read-through cache *inside* a store
  implementation — which is where that idea always belonged.

---

## 5. Projections: two divergences, and only one of them is a problem

Projections are derived, not authoritative, and rebuildable at any time from the log. That premise is
correct. But it does **not**, on its own, buy convergence, and two quite different phenomena look
alike from outside.

### 5.1 Divergence A — transient, benign, expected

My store holds ten facts; yours holds twelve; we render differently; sync lands; we agree. The normal
condition of a local-first application, not a defect. It resolves itself with no machinery, and needs
UX design — show provisional state, avoid layout thrash — and nothing else.

### 5.2 Divergence B — permanent, and rebuilding does not fix it

Both stores hold the **identical set** of facts, both rebuilt from scratch, and they still disagree.

```
Store A appends  TitleSet{"Sprint Planning"}   → A-local position 5
Store B appends  TitleSet{"Sprint Review"}     → B-local position 5
   … they sync; both stores now hold both facts …

A's log order:  [ … TitleSet{Planning}@5,  TitleSet{Review}@6  ]    ← Review ingested second
B's log order:  [ … TitleSet{Review}@5,    TitleSet{Planning}@6 ]    ← Planning ingested second

Projection: "last TitleSet wins", folded in local position order.
A renders "Sprint Review".   B renders "Sprint Planning".
Quiescent. Fully synced. Rebuilt from scratch. Permanently divergent.
```

Rebuilding cannot help, because **the local log's order is itself divergent, permanently**. SY-19
assigns ingested events *arrival-order* positions, so the same facts sit in a different sequence in
every store.

This is what SY-20 exists for:

> A projection declared convergent MUST produce byte-identical read models under any two ingest
> interleavings that agree on per-origin order and disagree everywhere else.

— with the rule `convergent_projection_is_interleaving_independent`, and **VT-9** forbidding any
ordering derived from `RecordedAt`. SY-20's *Rejects* names the wrong implementation directly: a
per-serial state machine folded over `SequencedEvent::position`.

Two things make B easy to miss: it is invisible in single-store testing, and it does not arise for
the folds people write first — counts, sums, set membership, "does X exist". `course-subscriptions`
is entirely immune, which is why nothing in the repo has tripped over it.

### 5.3 Three maintenance regimes

| Fold | Update strategy | Converges | Cost per event |
|---|---|---|---|
| Commutative across origins | apply incrementally in arrival order | yes | O(1) |
| Order-sensitive, folded in `EventId` order | re-fold from the insertion point | yes | O(n) worst case |
| Order-sensitive, folded in local position order | apply incrementally | **no — permanently** | O(1), and wrong |

Row 3 is the trap. Rows 1 and 2 are correct, and row 1 is the one to want — which is the real
engineering argument for commutative folds: they are not merely correct, they are the only ones
maintainable incrementally. `EventId` is what makes row 2 work — total, stable, computed identically
everywhere, respecting per-origin sequence. SY-21 asks whether a projection is handed an `EventId`
rather than a `SequencePosition`; here the answer is plainly yes.

### 5.4 Rewinding, and why "rebuild from the watermark" is not an operation

Two things that look similar and are not: a **projection checkpoint** is a single local
`SequencePosition`, a resume cursor into one log; a **`Watermark`** is a version vector, one position
per origin, and therefore not a point in any single ordering. Nothing can rewind *to* it.

Nor is a rewind needed for rows 1 or 3 — ingested events append at the **end** of the local log
(SY-19), so an incremental fold in local order never goes backwards. It is simply wrong for
order-sensitive folds. Row 2 is the only case that re-folds, and the trigger is not the watermark
moving; it is a late arrival from a low-sorting origin landing in the *middle* of the `EventId`
order. The re-fold starts at the insertion point.

Compensation needs no rewind at all. A compensating fact appended later changes the projection
**going forward**; nothing is retroactively un-shown. "Booking cancelled — the course was full"
rather than a row silently vanishing. `EventId` carrying its origin is exactly what lets domain code
notice that a fact from another store has invalidated something it had guarded.

### 5.5 What this actually asks of a read model

Not "never look at position or time" — that was overstated. Precisely:

- **A projection may declare itself non-convergent** (SY-22), and some should. "Which items have I not
  seen *on this device*" is legitimately per-store. The rule bites only on claims of convergence.
- **The prohibition is on folding read-model *content* over position or time**, not on touching them.
  Checkpointing and durability tracking are fine.
- **SY-20 requires commutativity across origins only**, not within one — per-origin order survives
  ingest. Far more tractable than full commutativity, and worth saying out loud.
- **Display order must be authored, not observed** — decided by the authoring store, carried in the
  fact. Fractional indexing (Photoroom, credited to Figma) fits because it authors a key rather than
  reading one. Clock skew is why a wall-clock stamp is not a substitute, and the spec has measured
  it: two clocks 2.4 seconds apart, one tablet six minutes fast after a factory reset.
- **Where the key lives is a real decision.** VT-3 (frozen) means only `EventType` and `Tags` are
  visible to the port. A key in the payload serves a projection and is invisible to a `Query`; if a
  *guard* must depend on ordering, it has to be a tag.

**The part that closes the loop with divergence A:** if display order is arrival order, the list
reshuffles every time sync lands — the *visible* kind of jitter, and the one users complain about. If
display order is authored, it does not move. Authoring the order key is not a tax convergence
imposes; it is the same fix as the jitter problem.

### 5.6 Where projections live, now that the log is behind an effect

`view()` is called on every render — twice per update if you adopt the pre/post diffing pattern — and
it can no longer fold the log. So a projection is either:

- **resident in `Model`**, maintained incrementally as local appends and ingested batches return as
  internal Intents. Cheap, simple, and viable exactly for row-1 folds — which is a second, independent
  reason to prefer commutativity; or
- **in a `ProjectionStore`**, behind its own effect, for read models too large to hold.

This gives `ProjectionStore` a concrete role it did not have in revision 2, and moves it onto the
critical path. It also re-acquires PS-25 (a checkpoint must not survive a change to the `Query` that
produced it), which for a device means a rebuild on app update — normal, but it must be designed for.

### 5.7 The read path: invalidate, don't push

Once the `ProjectionStore` has folded the log into a queryable, indexed surface, a second option
opens that is better than anything in the Photoroom series: **the shell reads that surface directly,
and Crux's job on the read side collapses to saying what changed.**

`Render` becomes `Invalidate(keys)`. Structurally this is what Photoroom did when they replaced
`Render` with `ChangeNotifications` — but carrying cache keys rather than patches, which maps
directly onto the key/invalidate model of TanStack Query, SWR, Compose paging, and every other
modern client cache. It needs no `difficient`, no `pathogen`, no whole-`ViewModel` transfer, and no
hand-authored change vocabulary. It is a proper CQRS split, and it sidesteps the problem that
consumed eighteen months of their series.

Two things bite, and they should be designed for rather than discovered:

- **Ordering.** The invalidation must be emitted **after the `ProjectionStore` transaction commits**,
  not when the fact is appended — otherwise the client refetches and reads pre-write data. PS-1's
  "read-model write and checkpoint write in one transaction" is what makes the correct point
  expressible; hang the invalidation off the commit.
- **The web read path is unresolved.** On native the shell can open the same SQLite file. On the web,
  "the shell queries SQLite" means sqlite-wasm plus OPFS, probably in a worker, with two sides of the
  FFI touching one database. The alternative is a `query(key) -> bytes` method on `CoreFfi` beside
  `view()` — trivially addable since that `impl` is ours, and a pure read that violates nothing. This
  is an open question, not a solved one.

Note that this also changes what §5.5 costs. If the shell queries projections directly, the
convergence rules bind the **projection**, which is where they belong, and `view()` may shrink to
almost nothing or disappear.

---

## 6. The guard across a peer set — what an `AppendCondition` is actually for

`AppendCondition { fail_if_events_match: Query, after: Position }` means "no event matching Q exists
after position P *in this store*." `P` is meaningful only inside one store. So the guard cannot be a
distributed constraint, ever, in any network condition. Not a limitation to work around — a fact to
design with. It has three real jobs:

1. **A genuine optimistic lock within one store.** Two in-flight decisions, or an ingest landing
   mid-fold — ES-36 is exactly this, and the guard is the right and sufficient mechanism. §4.3 makes
   this the common case rather than an edge one.
2. **A machine-readable statement of what the decision depended on.** The same `Query` reads and
   guards, so the condition *documents the boundary*. Valuable even where unenforceable, and why
   `EventGroup.guard` travelling as **evidence, not instruction** is right rather than a compromise.
3. **Input to an adjudicator**, where the domain nominates one (SY-7).

One sharp property: `fail_if_events_match` is **strictly coarser than the decision it protects**. It
re-reads the whole decision model's query, so *any* concurrent event in the boundary aborts, including
one that could not have changed the outcome. Tolerable within one store; another reason it is a poor
thing to enforce remotely.

### 6.1 Convergence taxonomy

Partition is permanent, so this is not an "offline mode" table — it is a permanent property of each
constraint, and it belongs in the type system.

| Class | Example | Fold shape | Behaviour across N stores |
|---|---|---|---|
| **1. Idempotent existence** | "not subscribed twice", "opt-in token used" | set union / lattice join | **Converges.** The guard is a local *optimisation*; correctness does not depend on it. |
| **2. Counting / resource** | "capacity ≤ n", "balance ≥ 0" | cardinality, sum | **Does not converge.** Needs a lease, escrow/quota partitioning, or accepted over-commit plus compensation. |
| **3. Uniqueness** | "unique username", "invoice number" | first-writer-wins over a global namespace | **Does not converge** unless the namespace is pre-partitioned per origin (`StoreId`-prefixed ids), in which case it trivially does. |
| **4. Ordering / causality** | "cannot unsubscribe before subscribing" | order-sensitive fold | **Converges only if authored.** SY-20 + VT-9 forbid folding on position or time. |

The lease for class 2 is elegant here rather than awkward, because **a lease is itself a replicated
fact**: `LeaseGranted(tag, store, until)` flows through the same log as everything else. Authority
that is *per-tag and pre-partitioned* rather than systemic — very DCB-shaped, and what the repo's own
`SiteLeaseGranted` scenario already does. It still needs a grantor, and naming one is a domain
decision.

### 6.2 Declare the class, then test the declaration

```rust
pub enum Convergence {
    /// Fold is a lattice join across origins; concurrent appends merge. Guard is an optimisation.
    Convergent,
    /// Sound only while this store holds a lease naming this boundary.
    Leased(Tag),
    /// May be violated across origins; the domain supplies a compensating fact.
    Compensable,
}
```

The enum is not the point. Each variant is a **falsifiable claim**, and the first is directly
property-testable:

> For any two disjoint fact-sets authored at different origins that each satisfy the guard against a
> common ancestor, the merged fold still satisfies the invariant — and produces a byte-identical read
> model under either interleaving.

That is a proptest, and the merge-shaped sibling of ADR-0010's "every rule owes a mutant": **every
`Convergent` declaration owes a merge mutant.** A domain crate that declares it and fails has lied,
and CI says so.

---

## 7. Testability — where this gets genuinely exciting

The spec's most honest paragraph says the instrument portfolio has "seven axes and seven empty far
ends" — every port checked against implementations that all agree with it, "one storage shape wearing
several hats." E2E-CASES lists multi-writer conformance and convergence as *unwritable today*.

**Crux is the missing instrument for exactly those axes.** Not for storage, but for **concurrency,
convergence, and restart** — and the reason is structural: the executor is driven and effects are
values, so you own the clock and every interleaving. N cores in one test process, no runtime, no
sockets, no containers.

Store-as-effect makes this *better*, not worse: the store is swappable in tests by construction,
because resolving a `StoreOperation` against a conformance-verified `MemoryEventStore` is just what
the harness does.

```rust
loop {
    if rng.gen_bool(0.005) { rehydrate_core(); continue; }                     // fuzz restart
    let step = if rng.gen_bool(0.2) { Step::Intent(gen_plausible_intent()) }
               else { step_buffer.remove(rng.gen_range(0..len)) };            // random completion order
    assert_invariants(&model);
    let mut cmd = app.update(step, &mut model);
    assert_invariants(&model);
    // resolve Store/Sync effects against N in-memory stores and a lossy simulated mesh
}
```

With partition injection and heal it yields, cheaply: **multi-writer conformance** (E2E-CASES blocked
item 8); **SY-20 convergence** — the harness that rule was written for and which does not exist;
**restart**, including the `StoreId` hazard in G1, which only a restart loop finds; and **the §6.2
property**.

Photoroom's calibration: ~500 LOC of fake effects, ~3000 scenarios/second, >100,000 tests/minute,
1325 Rust tests in under a second, a fuzz run in every CI build that "saved our bacon on many
occasions." Carry two disciplines: `assert_invariants` before **and** after every update, and
breadcrumbs for provenance. One caveat they flag — any change that adds a dice roll changes the
meaning of every seed.

---

## 8. Composability — reusable domain crates

**The good part.** Two domain crates that never share a tag never share a consistency boundary, so
they get parallel writes from one log for free — nothing declared, nothing configured. A cross-crate
invariant is then a *third* crate composing both `Decider`s, because `Query::Items` are OR'd:

```rust
impl<A: Decider, B: Decider> Decider for (A, B) {
    type State  = (A::State, B::State);
    type Intent = (A::Intent, B::Intent);
    fn query(i: &Self::Intent) -> Query { /* concatenate items */ }
}
```

The composed guard is the union of two boundaries, so its conflict probability is the union too.
Composition trades write concurrency for atomicity, one item at a time, *visibly*. The visibility is
the feature.

**The gap that breaks it.** Tags and event types are **flat global strings with no namespace
discipline** (VT-17: `key:value` is convention, not enforced). Two crates that both mint `"user:123"`
do not merely collide — they silently *widen each other's append conditions*, producing false
conflicts between unrelated features, intermittently, under load, with no error anywhere. Correct,
silent, and visible only as mysterious retry storms.

Recommendation: **mint tags from a namespace token, never from free strings.**

```rust
pub struct Namespace(&'static str);          // "courses.v1"
impl Namespace {
    pub fn tag(&self, key: &str, value: &str) -> Result<Tag, InvalidTag>;      // "courses.v1/course:c1"
    pub fn event_type(&self, name: &str) -> Result<EventType, InvalidEventType>;
}
```

The 255-byte budgets are ample. Costs nothing; closes a whole class of cross-crate bug.

**Effect libraries** compose the way Crux capabilities already do — generic over `Effect`, never
concrete: `where Effect: Send + From<Request<StoreOperation>> + 'static`.

**The weakest link** is `Model` composition. Crux's nested-state-machine protocol
(`Outcome`/`Status`/`Started` + `map_event`/`map_effect`) works but is manual. A domain crate
exporting a `Decider` and a `Projection` is clean; one exporting a sub-`Model` and a sub-`update` is
ceremony. Worth prototyping before committing.

**Still open:** payload versioning. WF-8 versions the envelope; a domain payload's evolution is the
crate's problem, and facts are permanent. Reserving an explicit protocol-version lever now is far
cheaper than adding one later.

---

## 9. What a server is for, and what the serverless case costs

### 9.1 Retention and bootstrap

In a mesh with **no always-on peer**, two things have no answer today:

- If every device compacts, history is lost and a projection needing the full log cannot be rebuilt.
  ES-38/ES-39: a purged log is currently indistinguishable from a young one at every value in §2.
- A **new device joining** must bootstrap, and `PushBatch` carries events, not snapshots. For a phone
  joining a mesh whose log is two years old, full transfer may be the only option and may not be
  acceptable.

So the serverless configuration needs either a **designated archival peer** or **first-class
snapshots**. An archival peer is a *retention* authority, not a *decision* authority — a distinction
that keeps "a server may exist" from smuggling back what §1.1 removed.

### 9.2 The price of the-event-stream-is-the-protocol

- **Schema coupling replaces API coupling, and it is harder.** You stop versioning endpoints and start
  versioning facts that are never deleted.
- **Nothing guards authorship.** VT-3 means only `EventType` and `Tags` are visible to the port;
  ingest never rejects (SY-1..7). A peer can inject any fact and it lands. There is **no authorship,
  no signing, and no authorization model anywhere in the spec** — review-blind-spots confirms zero
  security work of any kind. In a peer mesh this is mutual peer authentication plus a trusted device
  set, and possibly signed authorship so an adjudicator can attribute a fact. `EventId` carries the
  authoring `StoreId` already, but unsigned — a hint, not a claim.

---

## 10. Gaps

| # | Gap | Notes |
|---|---|---|
| **G0** | **Rust-side effect handling on wasm is unproven, and everything now pivots on it** | Both `counter-middleware` and `counter-routing` `#[cfg]` the middleware away on `target_family = "wasm"` because `thread::spawn` is unavailable. The machinery looks designed for it — there is a thread-ID guard rather than TLS, specifically for `spawn_local` — but no example demonstrates it. Also: middleware imposes `A::Model: Send + Sync + 'static`. Half-day spike, and it comes first. |
| **G1** | **`StoreId` lifecycle vs. `Core` lifecycle** | The spec offers mint-once-with-explicit-re-mint, or mint-fresh-on-every-open ("self-enforcing and impossible to forget"). Mint-on-open is defensible for a server opening monthly; for a phone app it splits one device's history into **one incarnation per launch** and grows the `Watermark` without bound. So a durable `StoreId` is loaded at `Core::new_with` — never generated in `update`, since generating 128 random bits is itself a side effect. And "explicit re-mint after restore" is now an **OS backup restore performed by a user with no operator supervising**. |
| **G2** | No synchronous decision path | Demoted by §4: no longer boundary enforcement, since the boundary is now the effect. Still worth having for tests, CLIs, and embedded drivers. A `BlockingEventStore` sibling trait, **not** a third `trait_variant` flavour (see ADR-0008's scars); the testkit's `__emit_blocking` harness already proves the shape. |
| **G3** | No tag / event-type namespace discipline | §8. The silent cross-crate false-conflict bug. Typed layer, not core. |
| **G4** | `Convergence` is undeclarable and untestable | §6.2. `happenstance-sync-testkit` does not exist, and `convergent_projection_is_interleaving_independent` is the rule it was named for. |
| **G5** | No convergent ordering primitive | §5.5. Every domain needs "display order that isn't position or time." Fractional indexing belongs in the typed layer or a companion crate, not in twelve applications. |
| **G6** | Snapshot / bootstrap absent from the sync port | §9.1. `PushBatch` carries events only. Blocks serverless meshes and new-device joins. |
| **G7** | `Event::new` rejects a held `EventType` (`E0271`, D4 / S9) | Blocks the typed layer's first line. Trivial fix. |
| **G8** | No authorship or authorization on ingest | §9.2. Out of scope everywhere today; a peer-authored stream makes it in scope. |
| **G9** | `head()` specified (ES-30) but absent from the port | The sync runner needs it every tick. |
| **G10** | `ProjectionStore` on disk contradicts the spec, and is now on the critical path | Code has `type Batch<'a>` + `async fn begin`; PS-4/5/6 say owned `Batch`, non-async, infallible `begin`. §5.6 promoted this port from optional to load-bearing. |
| **G11** | `facet` must not enter `happenstance-core` | Crux 0.20 pins `facet = "=0.46.5"` exactly; a mismatch surfaces as "`Facet<'_> is not satisfied`", not a version conflict. A contract crate cannot carry that pin. `happenstance-crux` owns wire mirror types — the orphan rule forces this anyway. |

Retired since revision 2: the working-set scale ceiling (the store pages now); VT-9's clock collision
and ES-35's durability gap (both handled on the effect side).

---

## 11. Where I'd go next

Each step falsifies the one before. Steps 1–4 need no shell, no FFI and no typegen.

0. **Prove G0.** A trivial async effect handled by Rust middleware or the effect router, resolved via
   `spawn_local`, running in a browser. Half a day, and the architecture rests on it.
1. **`Decider` + the generic command loop** against `MemoryEventStore`, no Crux at all. Port
   `course-subscriptions` onto it. *Falsifies:* whether the trait shape survives three real
   invariants. Useful even if Crux is abandoned.
2. **A convergent `Projection` trait + the SY-20 rule.** Two origins, two interleavings, assert
   byte-identical. Include one deliberately row-3 projection (§5.3) as a mutant, so the rule is shown
   to reject something. *Falsifies:* G4 and G5 — how many real read models are naturally commutative,
   and how painful the rest are.
3. **The Crux skeleton.** `StoreOperation` + `happenstance-sqlite` behind middleware, `map_effect` to
   keep it out of typegen, one `Decider` end to end. *Falsifies:* §4.3's round-trip cost and the
   `Send + Sync` Model constraint.
4. **N cores, simulated lossy mesh, partition injection, restart fuzzing.** *Falsifies:* §6.1's
   taxonomy and G1. The first thing in the workspace that can express two writers, and the experiment
   that decides whether "distributed DCB" is a coherent phrase.
5. Only then: typegen, FFI, a real platform shell.

### On a `happenstance-crux` crate

Reasonable, and it should be **thinner than it first appears** — which is the test of whether the
seam is right. Most of what looks Crux-specific is not: `Decider`, the convergent `Projection`,
`Namespace`, `Convergence`, and the ordering primitive all belong in `happenstance`; the sync runner
belongs above `happenstance-sync`; `StoreId` lifecycle belongs with the contract. What is left is the
`Operation` types and their `Facet`/`repr(C)` derives, the wire mirror types the orphan rule forces
(G11), command builders generic over `Effect`, and the mesh test harness. A few hundred lines.

**If it starts accumulating fold machinery or domain logic, the seam is wrong and the typed layer is
underbuilt.** Worth writing into its module docs as a falsifier on day one.

Three cautions: it is a **new category of crate** — a *host* adapter, not a store adapter or a port —
and the dependency rule does not name that category, which is an ADR rather than a directory. Its
dependencies churn (`crux_core` is pre-1.0 and says so; `facet` is pinned exactly), so keep it
`publish = false`, a leaf, and depended on by nothing. And the gate applies the moment it lands in
`crates/*`, so spike it in a worktree and promote when the shape stops moving.

One shape question worth leaving open: what the crate really does is express these seams as **managed
effects**, which is a sans-I/O pattern rather than a Crux one. There may be a host-agnostic
`happenstance-effects` with `happenstance-crux` reduced to a thin mapping onto `Operation`/`Request`,
which would let a server-side drive loop avoid depending on Crux at all. Don't design it
speculatively — the tell is simple: if writing a second driver (even the `Buffer`-lane one for tests)
duplicates real logic, the general seam exists.

---

## Open questions

1. **What is the smallest supported mesh?** Two phones with no server, or is an archival peer
   assumed? §9.1 has no answer without this, and it sets G6's priority.
2. **Which `StoreId` mint strategy?** G1. Needs deciding before anything persists; unfixable
   afterwards.
3. **Projections resident in `Model`, or in a `ProjectionStore`?** §5.6. Resident is far simpler and
   works for row-1 folds; the store is needed for read models too large to hold. The answer determines
   how urgent G10 is.
4. **Is a locally-decided outcome revocable in the UX?** Not for rebase — that is gone — but for
   `Compensable` constraints, where a compensating fact arrives later and the UI must show that
   something previously shown as done has been undone. A product decision that bounds how much of
   §6.1 class 2 you can tolerate.
5. **Does the Photoroom fine-grained-reactivity problem apply?** If projections are small, plain
   `Render` plus whole-`ViewModel` is fine and `difficient`/`pathogen` are unnecessary. Much cheaper
   to know before the ViewModel is load-bearing.

---

## Sources

- [Crux README](https://github.com/redbadger/crux/blob/master/README.md) · [crux_core 0.20.0](https://docs.rs/crux_core/0.20.0/crux_core/) · [CHANGELOG](https://github.com/redbadger/crux/blob/master/crux_core/CHANGELOG.md)
- Book: [Managed effects](https://redbadger.github.io/crux/part-2/effects.html) · [Capabilities](https://redbadger.github.io/crux/part-2/capabilities.html) · [Testing effects](https://redbadger.github.io/crux/part-2/testing-effects.html) · [Nested state machines](https://redbadger.github.io/crux/part-2/nested_state_machines.html) · [Middleware](https://redbadger.github.io/crux/part-3/middleware.html) · [Runtime](https://redbadger.github.io/crux/part-4/runtime.html) · [Typegen](https://redbadger.github.io/crux/part-4/typegen.html)
- [difficient](https://github.com/redbadger/difficient) · [pathogen](https://crates.io/crates/pathogen) · [BoltFFI](https://www.boltffi.dev/)
- Photoroom, *Building live collaboration in Rust for millions of users*, [parts 1–5](https://www.photoroom.com/inside-photoroom/building-live-collaboration-in-rust-for-millions-of-users-part-1) — **read as a server-authoritative design**; its reconciliation model does not transfer (§1.1), its testing and effect-hosting patterns do.
- [dcb.events](https://dcb.events/) · [Sara Pellegrini, *Kill the Aggregate*](https://sara.event-thinking.io/2023/04/kill-aggregate-chapter-1-I-am-here-to-kill-the-aggregate.html)
- In-repo: `spec/SPECIFICATION.md` (VT-3, VT-9, VT-27, ES-30/32/35/36/38/39, PS-4/5/6/25, SY-1..7, SY-19/20/21/22, §5.9, §5.10), `references/evaluation/PRESSURE-TEST.md`, `references/evaluation/review-blind-spots.md`, `spec/E2E-CASES.md`, ADRs 0001/0003/0006/0007/0008/0009/0010
