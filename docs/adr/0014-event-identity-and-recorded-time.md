# ADR-0014: The store mints identity, records a time, and the caller supplies neither

- **Status:** accepted — **provisional in three named parts**: VT-6 (which
  incarnation mechanism an adapter owes), VT-9 (that a wall clock is available at
  append time) and VT-10 (that the ingest seam belongs in `happenstance-sync`).
  Each falsifier and its owning phase is stated in
  [Provisional, and what would refute it](#provisional-and-what-would-refute-it).
  Nothing here is lifted by phase 4, because phase 4 has no instrument that could
  lift any of the three.
- **Date:** 2026-08-08
- **Settles:** the open half of VT-4 – VT-10. VT-4, VT-5, VT-7 and VT-8 are
  `[FROZEN]` and are **implemented, not reopened**; what this ADR decides is
  VT-6's mechanism, VT-9's representation, VT-10's confirmation, the constructor
  shape all four force, and the port operation VT-7 requires and no `ES` clause
  states.
- **Extends:** [ADR-0008](0008-one-derivation-for-both-ports.md), whose
  required-versus-provided finding decides the shape of the new port operation
  without re-deriving it, and [ADR-0009](0009-error-send-sync.md), whose
  `Self::Error` stays exactly as it is here.
- **Touches:** ES-19, where the returned position meets the minted identity;
  VT-3's prose, for `Event::into_parts`'s arity, jointly with ADR-0012; and
  ES-30, whose required-method cost this ADR adds a second method to rather than
  reopens.
- **Adds:** ES-41, the membership operation VT-7 `[FROZEN]` requires and
  forward-references into a §3 that does not contain it.
- **Adopts, after this ADR was drafted:** **VT-2** `[FROZEN]`, which the queue
  assigned to nobody and no ADR in this pass mentioned. Its MUST — *"appending
  two structurally equal `Event` values MUST produce two distinct events with two
  distinct positions and two distinct `EventId`s"* (`SPECIFICATION.md:601-604`) —
  is not writable before `EventId` exists, and its rule
  `appending_equal_events_yields_two_events` does not exist in `suite.rs`. It is
  adopted here rather than left to the coverage audit that found it, because a
  frozen clause whose rule is owned by no phase is indistinguishable, in a work
  queue, from a clause that is already done. See §9.
- **Ordered after:** [ADR-0013](0013-position-assignment-and-visibility.md).
  ES-41 is a `[FROZEN]` port clause with no adapter instrument at any axis's far
  end, so CF-25 requires it to name the ADR accepting that risk, and that ADR is
  0013. This is an ordering constraint on the code run, not a dependency between
  the decisions.

## A note on scope

This ADR answers one question: **what does an event carry beyond type, data and
tags, and who assigns each part.**

It answers a second one in passing and says so rather than smuggling it:
**`Event::into_parts`'s return arity** (`event.rs:243-246`). The *doc comment* on
that method is ES-17's correction and belongs to
[ADR-0012](0012-append-shape-and-preconditions.md); the *arity* is here, because
it is the identical defect to `SequencedEvent::new`'s arity — a positional shape
that is public API — and splitting one defect across two ADRs is how half of it
gets fixed. Both ADRs
amend the same paragraph of the specification (VT-3's prose at
`SPECIFICATION.md:670-677`), and the run that applies them must apply both.

It also writes a clause in §3, which is `EventStore`'s section and not this
ADR's. That is a **forced corollary rather than a second question**, and the
force is mechanical: VT-7 is `[FROZEN]`, its `Rule:` line names "§3's new
`contains_event_id_reports_membership`", and §3 contains no such clause
(`:797-798`). A frozen clause forward-referencing a clause nobody wrote is a
dangling reference that only the ADR settling identity is in a position to
discharge, because the operation's argument is the type this ADR defines. What
would have been a genuinely separate question — whether `EventStore` should grow
methods at all — was settled by ES-30 and is not reopened here.

Everything else this ADR touches, it touches by naming who owns it.

## Context

### What exists today: nothing here, and a placeholder next door

`happenstance-core` has no `EventId`, no `StoreId` and no `RecordedAt`.
`SequencedEvent` is `{ position, event }` (`event.rs:277-282`) and
`SequencedEvent::new` is a two-argument `const fn` (`event.rs:286`) that every
adapter and the testkit call.

What the reconciliation dossier records as "no instrument anywhere" is true of
the *conformance* surface — no rule, no mutant, no racer names any of the three —
and it is worth stating precisely what does exist, because it is not nothing.
`crates/happenstance-sync/src/identity.rs` contains a **phase-2 placeholder** of
all three types, written to make the `IngestStore` sketch compile, whose own
module documentation says (`identity.rs:24-32`):

> These three types are placeholders, and phase 4 deletes them. […] What is
> sketched here is the *shape the sketch needed in order to compile*, and it is a
> coincidence rather than a design if it matches what phase 4 arrives at.

It matches in two places and diverges in one, and the divergence is the point:
`identity.rs:144` is `pub struct RecordedAt(u64)`, unsigned. Its doc comment
argues for `u64` **milliseconds rather than a `chrono`/`time` type**, which is an
answer to a different question than signedness — the RUNBOOK's phase-4 body
(`RUNBOOK.md:2891-2894`) makes the same substitution, and VT-9 (`:851-856`,
`:881-895`) requires signed. The specification wins under RUNBOOK rule 5, and the
reason it should is in [the RecordedAt section](#4-recordedat-is-a-newtype-over-i64-and-core-owns-no-clock).

The placeholder also constrains the swap in a way nobody wrote down, and the
constraint is narrower than it first looks. `Watermark` (`identity.rs:219-254`)
keeps its marks sorted and reaches them with `binary_search_by_key`, which
requires `StoreId: Ord`; `Watermark::get` takes a `StoreId` by value and
`Watermark::iter` ends in `.copied()`, which requires `StoreId: Copy` and
`SequencePosition: Copy`. `ReplicatedEvent` (`identity.rs:171-180`) derives
`Debug, Clone, PartialEq, Eq`, so it requires those of both `EventId` and
`RecordedAt`. The minimum the swap forces is therefore
**`Copy + Ord + Eq + Clone + Debug`**. `Hash` is derived on all three
placeholders (`:61`, `:97`, `:143`) and is required by nothing in
`happenstance-sync` today — it ships here because an adapter keying a dedup map
on an `EventId` is the obvious implementation of VT-8 and a missing `Hash` would
be a needless breaking addition, not because the sync crate demands it. Note the
two containers that are *not* `Copy`: `Watermark` holds a `Vec` and
`ReplicatedEvent` holds an `Event`, so "the identity types are `Copy`" is a
statement about the three scalars and not about the file.

**This ADR has not compiled that swap** — `crates/` was off-limits to the run
that wrote it — and the phase-4 code run must treat `happenstance-sync` as a
downstream consumer of the new types on the first compile, not as an
afterthought.

### What is already decided, and is therefore not this ADR's to argue

Four clauses are `[FROZEN]` and settle the *shape* of identity:

| Clause | The sentence | What it forecloses |
|---|---|---|
| **VT-4** (`:683`) | `SequencedEvent` carries `SequencePosition`, `EventId`, `RecordedAt` and the `Event`; all four fields public, struct stays `#[non_exhaustive]` | Any arrangement that hides identity behind an accessor, and any that drops `#[non_exhaustive]` |
| **VT-5** (`:709`) | `EventId` is the pair `(StoreId, SequencePosition)`, minted by the store that first accepted the event, preserved unchanged through ingest, and a **newtype** rather than a tuple alias | A UUID; and `type EventId = (StoreId, SequencePosition)`, which has no coherence room — `impl Serialize for (StoreId, SequencePosition)` is a foreign trait for a foreign type and the orphan rule refuses it outright |
| **VT-7** (`:789`) | `EventId` is outside the query language, MUST NOT be a `Tag`, and membership is answered by **a dedicated port operation** | The tag-materialised identity, which puts a maximally high-cardinality entry in the one column adapters are told to index and makes identity writer-forgeable |
| **VT-8** (`:818`) | At most one event per `EventId`, enforced by the store | Idempotence by read-then-append: two round trips and an unclosed race between two concurrent ingests |

For a reader coming from C#: `#[non_exhaustive]` is **not** `sealed`. It does not
stop anyone constructing the type; it stops anyone *outside this crate*
constructing it with a struct literal (`error[E0639]`) or matching it
exhaustively. Inside `happenstance-core`, literals still work. Its entire
mitigation is therefore "no struct literal downstream", which routes every
downstream construction through `SequencedEvent::new` — and makes `new` the whole
compatibility surface rather than a convenience on top of one. That is the
opposite of the protection the attribute is usually credited with, and it is what
makes [the constructor section](#6-the-constructor-shape-new-is-superseded-not-widened)
the hard part of this ADR rather than a footnote.

## Decision

### 1. Three types, in `happenstance-core`, and the placeholders are deleted

```rust
// All three: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display.
// Copy + Ord + Eq + Clone + Debug are forced by `happenstance-sync`; Hash is not
// (see Context). Display is 32 hex chars for StoreId, "store:position" for EventId.
pub struct StoreId([u8; 16]);
pub struct EventId { store: StoreId, position: SequencePosition }
pub struct RecordedAt(i64);
```

`EventId`'s two fields are **private with accessors**, and this is not
inconsistent with VT-4: VT-4's "all four fields MUST be public" is about
`SequencedEvent`, whose fields are four independent facts a reader wants by name.
`EventId`'s two fields are one fact with two halves.

Privacy is **this ADR's decision and not VT-5's**, and the distinction matters
because VT-5 is frozen and this is not. VT-5's newtype argument (`:742-747`) is
three things — no `Display`, no place to hang WF-6's wire encoding, and no
coherence room for `impl Serialize` — closing on making an `EventId` and a bare
position non-interchangeable at every call site. Every one of those is satisfied
by a newtype with public fields, so VT-5 does not decide this. The argument that
does is the sync placeholder's, written down at `identity.rs:90-96`: a
transparent pair lets the compiler accept `EventId { store: their_id, position:
my_position }` without a word, and that is exactly the value the type exists to
make deliberate. Private fields buy that and cost a caller two accessor calls.
`store()` and `position()` return by value; both halves are `Copy`.

`StoreId` gets `from_bytes([u8; 16])`, `to_bytes()` and `Display` as 32 lowercase
hex characters with no dashes — deliberately not UUID-formatted, because it has
no version nibble and no variant bits and formatting it like a UUID invites an
operator to parse it as one. It gets **no** `FromStr` and no random constructor;
see [alternatives](#alternatives-rejected).

**`[u8; 16]` rather than `u128`,** which is the one representation choice a
reader will assume was made by default. Three reasons, none of them about size.
A `u128` has a byte order and an array does not, so every adapter persisting one
must pick `to_be_bytes` or `to_le_bytes` and two adapters that pick differently
produce logs that disagree about identity while both passing every in-process
rule. `Ord` on `[u8; 16]` is lexicographic over exactly the bytes an adapter
stores, so the sort order VT-5's convergent fold needs is the same order a
database's `BLOB`/`BYTEA` collation already gives; `Ord` on a `u128` is numeric
and agrees with that only under one endianness. And an integer invites
arithmetic — incrementing, ranging, comparing as a magnitude — on a value whose
128 bits are opaque by VT-6's construction. The array does not discourage any of
the three; it removes the first two questions and leaves the third with no
operator to write. It also keeps the wasm32 exposure
below in one place: bytes have a storage representation on Workers SQL and a
128-bit integer does not.

The three types acquire `Serialize`/`Deserialize` under `happenstance-core`'s
existing `serde` feature, which is where the envelope types already live
(ADR-0003, `event.rs:301`). **Their wire encoding is not decided here.** Whether
a `StoreId` crosses the wire as hex text or as a byte array, and a `RecordedAt`
as a number or a string, is WF-6's and phase 5's; this ADR ships the derives and
names the handoff so that phase 5 changes an encoding rather than a type.

### 2. Who assigns what

| Fact | Assigned by | When | Preserved through ingest? |
|---|---|---|---|
| `SequencePosition` | the store | at `append` | **No** — an ingested event gets a fresh local position; the origin's position survives inside its `EventId` |
| `StoreId` | the **deployment**, through the adapter, at database creation or at open | never by `happenstance-core` | n/a |
| `EventId` | the store, as `(own StoreId, the position it just assigned)` | at `append` | **Yes, unchanged** (VT-5) |
| `RecordedAt` | the store, from the host's clock | at `append` | **Yes, unchanged** (VT-9) |

The redundancy in row 1 and row 3 is deliberate and will be asked about: for a
locally appended event `id.position() == position`, so `SequencedEvent` carries
the same number twice. For an **ingested** event it does not — `position` is
arrival order here and `id.position()` is authorship order there, which is what
`Ingested::last_local` already says in the sync sketch
(`crates/happenstance-sync/src/ingest.rs:155-159`). A design that stored only one
of them would be correct for local appends and would lose either the local
ordering or the origin identity for replicated ones.

### 3. `StoreId` names an incarnation, and the "SHOULD prefer" becomes a conditional MUST

VT-6 states the invariant (never reissue a pair) and offers two mechanisms: mint
once at schema creation with an out-of-band re-mint after a restore, or mint a
fresh incarnation on every open. It says adapters SHOULD prefer the first.

**The brief asks whether it is survivable that an adapter can detect neither a
restore nor a clone, and can be given no re-mint operation. It is, and the reason
is that VT-6's second mechanism needs no detection at all.** A store that mints a
fresh incarnation on every open cannot reissue a pair after a restore, because
the restored copy is a different incarnation before it writes anything. So the
clause's own falsifier, read literally, describes a deployment the clause already
handles.

What an adapter must do, stated as a rule rather than a preference:

> An adapter MAY mint once and provide an out-of-band re-mint operation **only
> if** it can detect that its state was restored or cloned, **or** the deployment
> is documented to invoke the re-mint. An adapter that can do neither **MUST**
> mint a fresh incarnation on every open, and MUST record which mechanism it
> chose in `docs/adapter-shapes.md`.

That turns VT-6's SHOULD into a conditional with a stated default and makes the
undetectable case safe by construction rather than by procedure.

**It also invalidates VT-6's own named rule.** `store_id_is_stable_across_reopen`
(`:761`) would fail every adapter that takes the second mechanism — the one the
clause permits and this ADR sometimes *requires*. A rule that forbids a permitted
implementation is worse than a decorative one. The rule this ADR names instead is
the invariant both mechanisms satisfy:

> `reopened_store_does_not_reissue_an_event_id` — gated on `Capability::REOPEN`.
> Append **two** events, assert their `EventId`s share one `StoreId`; reopen,
> append again, and assert the third event's `EventId` differs from both of the
> first two.

The wrong implementation it rejects is sharp and is exactly the restore failure in
miniature: **a store that keeps its `StoreId` across a reopen and restarts its
position counter at 1.** That is what a restored backup looks like from the
inside, and the rule catches it by instruction on a single thread.

The first assertion is not padding, and it is there because **replacing a rule
loses whatever the replaced rule caught by accident.**
`store_id_is_stable_across_reopen` rejected mint-per-*append* — a store that
mints a fresh 128-bit value for every event — as a side effect of demanding
stability, and the reissue assertion alone does not, because per-append minting
produces `EventId`s that all differ. Per-append minting satisfies VT-6's literal
MUST and destroys everything the type is for: every event becomes its own origin,
`Watermark` grows one row per event rather than per incarnation, and VT-5's
peer-independent sort degenerates. "Two events in one open share a `StoreId`" is
the weakest assertion that rejects it, it is true under both permitted mechanisms,
and it costs the rule one extra append. Naming what a replacement stops catching
is the part of a rule swap that is easy to skip and expensive to skip.

This is where this ADR **extends** the dossier rather than agreeing with it. The
dossier records that VT-6's question is unanswerable at phase 4 because
`DurableFixture` reopens by instruction rather than by fault. That is right about
*detection* — nothing in the tree can present an adapter with a restore it must
notice — and it does not follow that VT-6 has no writable rule. The *harm* VT-6
names is reissue, and reissue is observable by instruction. Detection stays
unanswerable; reissue does not.

### 4. `RecordedAt` is a newtype over `i64`, and core owns no clock

**Signed, not unsigned**, per VT-9 and against `RUNBOOK.md:2891-2894` and the sync
placeholder. Pre-1970 costs nothing to represent, and — the reason that actually
bites — an unsigned epoch turns `a - b` between two timestamps into wrapping
arithmetic that produces a plausible-looking enormous positive number instead of a
negative one. The two clocks in Kestrel Rotor are 2.4 seconds apart (`:872-873`);
subtracting them is the first thing anyone debugging a replication lag does.

**A newtype, not `type RecordedAt = i64`.** For a reader new to Rust, the decisive
reason is coherence rather than taste. A type alias is fully transparent: `i64` and
`RecordedAt` are the *same type*, so an alias buys no `Display`, no protection
against passing a position-derived integer where a time is expected, and — this is
the one that closes the question — no `impl From<RecordedAt> for SystemTime`.
Rust's orphan rule permits implementing a foreign trait only when a local type
appears in the trait reference; with an alias, that impl reads `impl From<i64> for
SystemTime`, in which both types are foreign and the compiler refuses it outright.
The newtype makes `RecordedAt` local, and the impl becomes ordinary.

```rust
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl From<RecordedAt> for SystemTime { … }
```

behind the crate's existing `std` feature (`Cargo.toml:29`), so the contract crate
gains no time dependency and the `no_std` build gains nothing at all. Two
implementation notes for the code run, both about `i64` and neither a compile
claim: the conversion must branch on sign, and the negative arm must use
`i64::unsigned_abs` rather than negating, because negating `i64::MIN` overflows.
The reverse direction is `TryFrom<SystemTime> for RecordedAt`, fallible because
`SystemTime`'s range is wider than `i64` milliseconds.

**`happenstance-core` ships no `RecordedAt::now()`, not even behind `std`.** A
`std` feature is a compile-time choice, not a capability: a `wasm32` build with
`std` on would get a `now()` whose behaviour is the platform's problem, and the
platform in question is the one VT-9's falsifier is about. The clock stays where
the adapter is, which is also the only place that knows whether it has one.

**What the rules may assert.** VT-9's third MUST forbids *the contract* from
stating any relationship between `RecordedAt` order and `SequencePosition` order,
and the conformance suite is the contract's executable form — an adapter author
reads a failing rule as a requirement, so a rule asserting an order states the
order. Therefore:

- `append_stamps_a_recorded_time` may assert only that a value is **present** on
  read-back and **equal** to nothing but itself.
- `recorded_time_survives_a_reopen` may assert only that the same event's value
  is unchanged after `REOPEN`.
- **No rule may compare two events' `RecordedAt` values, in either direction, nor
  compare one against a position.** A store on a machine whose clock steps
  backwards under NTP correction is conformant.
- No rule may check plausibility against the test's own clock either — CF-33
  `[FROZEN]` (`:7327`) forbids a conformance rule from reading one.

**The RUNBOOK's "a rule that it is non-decreasing with position" is therefore
dropped**, and so is `RUNBOOK.md:557`'s ledger cell, which names that same
forbidden rule as VT-9's *falsifier*. Ledger, phase body and clause are three
different answers today; the clause is the one that survives.

### 5. VT-7's membership operation: a new required method on `EventStore`

VT-7 is `[FROZEN]` and its Rule line points at "§3's new
`contains_event_id_reports_membership`". §3 is the `EventStore` section and it
contains no such clause, so the frozen sentence forward-references a clause that
does not exist. This ADR writes it, as **ES-41**:

```rust
async fn contains_event_id(&self, id: EventId) -> Result<bool, Self::Error>;
```

Four choices in one line, each with the alternative that lost.

**Required, not provided.** ADR-0008 compiled the reason and ES-30 (`:3284-3288`)
already spends it on `head()`: the only provided form holds `&self` across an
`await` and therefore needs `where Self: Sync`, and the `!Send` edge adapter — the
entire reason the bare flavour exists — is `!Sync`, so the provided body is
`error[E0277]` at the call for exactly the adapter it was meant to spare. This is
the **second** required method phase 4 adds to the port — the first is `head()`,
which is ES-30's and belongs to the *signature* half, not to this one — and
[the exit criteria section](#8-what-does-not-change-and-one-thing-that-does) says
what the pair costs together.

**On `EventStore`, not on `IngestStore`.** The sync sketch already has this
operation, as `IngestStore::holds` (`ingest.rs:133`, documented from `:121`), and
putting it only there is
tempting: it keeps the contract port unchanged and follows VT-10's posture that
foreign identity is replication's business. It loses on two counts. VT-7 is frozen
on §3, and — the substantive reason — VT-8 already obliges **every** store to
enforce at-most-one-event-per-`EventId`, so every store already holds the index
that answers this question and none of them is being taxed with a new one. A store
that never replicates answers it without any index at all: if `id.store()` is not
its own incarnation the answer is `false`, because it has ingested nothing, and if
it is, the question reduces to whether that position exists. That shortcut costs
an adapter nothing it does not already have, and specifically it does not
contradict the refusal of a synchronous `store_id()` in
[§8](#8-what-does-not-change-and-one-thing-that-does): `contains_event_id` is
`async`, so an adapter that cannot hold its own incarnation between calls does
not need the shortcut and answers with the lookup itself — for
`happenstance-neon` that is one `WHERE origin_store = ? AND origin_position = ?`
in the single round trip it is allowed. The consequence is
that `IngestStore::holds` becomes a second operation answering one question on the
same concrete type, which is the duplicate-requirement failure VT-12 exists to
warn about. Removing it is `happenstance-sync`'s to do in the phase that designs
that port; recorded here, not decided here. Note also that the sketch spells it
`holds(&self, id: &EventId)`, by reference, so the removal is not a rename.

**By value, not `&EventId`.** `EventId` is a `[u8; 16]` beside a `NonZeroU64`,
both `Copy`, and nothing in the workspace has measured its `size_of` — `repr(Rust)`
promises no layout, so the honest statement is "two `Copy` scalars, sixteen bytes
and eight" rather than a number. The argument does not need one: at that size
there is nothing to save by borrowing, and an `&EventId` would additionally put a
lifetime on a method every `dyn`-erasure and every blanket forward has to carry.
The contrast with `read(query: &Query)` is worth drawing because it looks
inconsistent: `Query` is not `Copy` and a realistic two-clause query costs **9
allocations and 212 bytes** to clone (measured with a counting allocator, E3,
`phase-4-reconciliation.md:344-354`), which is why ES-13 borrows it
and why by-value would put that on the hottest path in the port. A pair of `Copy`
scalars has no such story.

**`bool`, not `Option<SequencePosition>`.** Returning where the event landed
locally looks strictly more useful, and it makes the operation answer two
questions instead of one — a store whose uniqueness is a database constraint
rather than a lookup table can prove membership without locating the row.
Widening a `bool` return to an `Option` later is a breaking change; adding a
second, locating method later is additive. Of the two ways to be wrong, this is
the cheaper one.

The wrong implementation ES-41 rejects, and the mutant the phase-4 code run owes:
**a store that answers by position alone and ignores the `StoreId`.** That is the
natural `SELECT 1 FROM events WHERE position = ?`, it passes every single-store
test in the suite, and it reports a peer's event as already present whenever the
local log happens to be at least that long — silently dropping real facts, which is
the same failure mode VT-6 spends a clause preventing, arriving by a different door.

**ES-41 ships `[FROZEN]` and must carry CF-25's qualification explicitly, or the
gate rejects it.** CF-25 (`:7096-7102`) is itself `[FROZEN]` and is not a
convention: `cargo xtask spec-trace` "reads the portfolio table and the maturity
markers and fails when a `[FROZEN]` port clause has an axis with no far-end row
and no named risk acceptance". ES-41 is a port clause in §3 and
[What this decision cannot prove](#what-this-decision-cannot-prove) supplies the
evidence that it would fail: no adapter instrument exists at the far end of any
axis, and by CF-26 a fixture does not substitute. So the clause inherits, by name
rather than by proximity, the acceptance ADR-0013 makes for the six axes phase 4
does not discharge (`RUNBOOK.md:3029-3032`), and it names **transport** and
**completeness** as the two it is most exposed on — transport because
`contains_event_id` is one round trip on an adapter with no connection, and
completeness because a store that cannot state what it does not hold (ES-39)
cannot distinguish "no such event" from "not visible to me". Writing ES-41 as
`[FROZEN]` without that sentence is not a stylistic omission; it is a red gate,
and it is the one ordering constraint between this ADR and ADR-0013 that a code
run cannot discover from the diff.

### 6. The constructor shape: `new` is superseded, not widened

Experiment E7 compiled the two halves of this and they point in opposite
directions, so both must be stated.

**What survives a field addition** is `const fn new(<intrinsically required
fields only>)` plus `#[must_use] const fn with_*(mut self) -> Self`. A fourth field
was added to a struct of that shape with **zero** downstream edits, including at a
`const` call site, where a separate builder type would not have been usable at all.
It is also the crate's existing idiom — `ReadOptions` (`query.rs:246-266`) and
`AppendCondition` (`append.rs:72-88`) are both written this way — so adopting it
adds no third spelling.

**The caveat that must travel with it, and that decides this phase:** it survives
only for fields with a meaningful default. Phase 4's two additions are required
store-assigned facts, and a required field has no non-breaking constructor shape
at all. Adding one to a positional constructor is `error[E0061]`, and
`#[non_exhaustive]` is precisely what makes that the *only* surface — the struct
literal is already `error[E0639]` downstream.

So:

```rust
pub const fn new(
    position: SequencePosition,
    id: EventId,
    recorded_at: RecordedAt,
    event: Event,
) -> Self;
```

`SequencedEvent::new` goes from two arguments to four in one commit, every call
site in the workspace is edited in that commit, and there is **no deprecated
two-argument arm**, because nothing is published and a compatibility shim would
have to invent a `StoreId` and a time — which is the wrong implementation VT-4
rejects by name (`:692-695`), shipped as a convenience.

The `with_*` idiom lands anyway, for the next field rather than for these two: a
per-store `ingested_at`, a retention marker or a redaction flag are all
defaultable, and each of them is additive under this shape.

**Exit criterion 9 (`RUNBOOK.md:3043-3044`) is therefore satisfiable only in the
defaultable case, and must be reworded.** "Adding a *third* field would not break
`new`" is true for a field with a default and false for another required
store-assigned fact, and the criterion as written does not distinguish them. The
honest form is: *`SequencedEvent` carries `id` and `recorded_at`; `new` is
superseded rather than widened; and the `with_*` shape is in place so that a
defaultable field can be added without touching `new` again.*

### 7. `Event::into_parts` returns a struct

```rust
#[non_exhaustive]
pub struct EventParts {
    pub event_type: EventType,
    pub data: Bytes,
    pub tags: Tags,
    pub metadata: Option<Bytes>,
}

pub fn into_parts(self) -> EventParts;
```

Same defect, same fix, one pass. A positional 4-tuple makes the *number* of an
`Event`'s parts public API: every `let (ty, data, tags, meta) = e.into_parts();`
breaks if a fifth part ever exists. With a `#[non_exhaustive]` struct, downstream
reads fields by name or destructures with `..`, and a later part is additive.
Ownership still moves out, which is the whole reason the method exists.

No clause claims this and this ADR is its authority; the amendment is to VT-3's
prose paragraph, which ADR-0012 also amends.

### 8. What does not change, and one thing that does

**`EventStore::append`'s signature is unchanged by identity.** Confirmed, and
stated out loud because `RUNBOOK.md:3045-3049` asks for it: `append` takes
`&[Event]` and a condition, and no slot for a foreign `EventId` or `RecordedAt`
appears anywhere on it. VT-10 is the reason — the operation that accepts a foreign
identity is `IngestStore`, in `happenstance-sync`, behind an adapter feature. The
value half of this phase does **not** reopen the signature half through `append`.

**`append`'s return type is unchanged too** (ES-19). It returns the
`SequencePosition` of the batch's last event, not an `EventId`. A caller that
wants the full identity of what it just wrote reads it back, for the same reason
ES-19's own prose (`:2848-2854`) gives for the returned position not being a sound
`after`: what comes back from a read is what the store actually holds.

**And `EventStore` gains no `store_id()` accessor.** This is the one place a
plausible convenience was refused on adapter evidence rather than on taste. A
caller holding the store's own `StoreId` could construct the last event's `EventId`
from `append`'s return value without a read, which is genuinely cheaper — but a
synchronous, infallible `fn store_id(&self) -> StoreId` requires every adapter to
have the value in hand before the call, which means reading a metadata row at
connect time, which `happenstance-neon` cannot do: it has **no connection and
exactly one round trip per operation** (`adapter-shapes.md:216-217`), and that row
exists precisely to catch port methods that assume a peer can hold state open
between calls. If the accessor is ever wanted it is `async fn store_id(&self) ->
Result<StoreId, Self::Error>`, and adding it later is additive. The sync sketch's
`IngestStore::store_id(&self) -> StoreId` (`ingest.rs:98`) is the sync form of the
same hazard; flagged for whoever freezes that port, not decided here.

**So the honest statement of exit criterion 10 is:** the signature half is not
reopened by `append`, and it *is* extended by the trait — `head()` from ES-30 and
`contains_event_id` from ES-41/VT-7 are two new required methods, and every
conformant impl, every skeleton and every mutant grows a body for each. The
criterion as written is true of `append` and false of `EventStore`, and it should
say which it means.

### 9. VT-2's rule is this ADR's, because identity is what makes it writable

VT-2 is `[FROZEN]` and needs nothing decided. What it needs is a **rule that does
not exist**, and the reason it does not exist is that nobody could write it: two
structurally equal `Event`s already get two distinct positions today, so a rule
written before this ADR could assert two-thirds of the clause and silently skip
the third. The MUST is *"two distinct events with two distinct positions and two
distinct `EventId`s"*, and the last conjunct is only expressible once
`SequencedEvent` carries `id`.

So the rule the code run writes is:

```rust
/// VT-2 -- structural equality is not identity.
///
/// Rejects a content-hash identity scheme, and any store that deduplicates on
/// payload equality: two byte-identical `VanStockConsumed` events collapse into
/// one and the van's stock balance is permanently one unit high, with nothing
/// reporting it (`SPECIFICATION.md:609-613`).
pub async fn appending_equal_events_yields_two_events<F: Fixture>(
    open: impl AsyncFn() -> F,
) -> RuleOutcome
```

It appends **one batch of two `Event` values that compare `PartialEq`-equal** —
same type, same payload, same tags, same metadata — and asserts three things:
two events come back, their positions differ, and their `EventId`s differ. One
batch rather than two appends is deliberate: it is the arrangement a content-hash
scheme collapses, and it also exercises ES-19's slice-order assignment on the
identity path.

**The mutant it needs, since a rule no adapter can fail is decorative.** A
`ContentHashIdentityStore` whose `EventId` is derived from the event's bytes
rather than from `(StoreId, SequencePosition)`. It passes every other rule in the
suite — its identities are unique across *distinct* events, stable across a
reopen, and never reissued — and fails this one alone. That is the provenance
CF-4 asks for, and it is a real adapter shape: content-addressed identity is what
anyone reaching for idempotent ingest proposes first, and it is exactly what VT-8
forbids by making uniqueness the store's obligation rather than the caller's.

**What this ADR does *not* do to VT-2:** it does not touch the clause. VT-2 is
frozen, correct as written, and needs no amendment — only an owner. The
amendments section below therefore carries no VT-2 entry, and that absence is the
point.

## Provisional, and what would refute it

Three markers stay, and none of them can be lifted here, because phase 4 has
neither a persistent adapter nor a platform.

**VT-6 — the incarnation mechanism.** The clause's current falsifier ("an adapter
that can neither detect a restore nor be given an out-of-band re-mint") is met by
`happenstance-sqlite` the moment it exists — `cp store.db backup.db` is
undetectable — and the clause survives it, because such an adapter mints per open.
**The live falsifier is the cost of that mechanism, not its availability:** a
deployment in which the mint-once path is unavailable *and* mint-per-open produces
so many incarnations that every peer's `Watermark` — one row per origin, forever
(`identity.rs:219-254`) — grows without bound. The candidate is a Cloudflare
Durable Object, whose isolate is evicted and revived as a matter of routine; the
first observation is **phase 9**, and **phase 13**'s sync testkit measures the
harm. Until then the clause is provisional on a falsifier that at least names a
number that can get too large.

**VT-9 — that a wall clock exists at append time.** Unchanged and restated: a
target that cannot supply one. A Durable Object returning a frozen clock between
I/O operations is the candidate and the Workers skeleton is the instrument, so the
observation is **phase 9**. Nothing before it can pose the question, because every
store in the tree runs on a host with `SystemTime`.

**VT-10 — the ingest seam's placement.** Falsified if a store adapter cannot
implement `IngestStore` without duplicating `append`'s write path, in which case
the operation belongs on `EventStore` after all. The instrument is the SQLite
adapter implementing both, which is **phase 8**'s adapter under the `sqlite/sync`
feature, so the first honest observation is whenever that feature lands. Note the
asymmetry this ADR creates and does not resolve: `contains_event_id` moves *onto*
`EventStore` while `ingest` stays off it. That is not inconsistent — membership is
a question about the store's own contents (VT-8) and ingest is a write with foreign
identity (VT-10) — but if VT-10 is ever falsified, the two land in the same place
and one of the two clauses was carrying the wrong reason.

## What this decision cannot prove

Stated plainly, because the phase this ADR belongs to is a freeze and a freeze
that does not name its exposure is a freeze pretending to evidence it lacks.

- **No adapter instrument, and only a partial fixture one.** No rule, mutant or
  racer in the workspace names `EventId`, `StoreId` or `RecordedAt` today —
  verified by grep over `happenstance-core/src`, `happenstance-testkit/src` and
  `happenstance-testkit/tests`, which return nothing. VT-4 – VT-9 name **ten**
  distinct rules of their own, not five, plus a cross-reference to SY-20's, and
  the split is eight to two rather than three to two. Writable in phase 4's code
  run, in the event-store suite:
  `append_stamps_identity_and_time` (VT-4), `event_ids_are_unique_within_a_store`
  (VT-5, VT-8), `append_stamps_a_local_event_id` (VT-5),
  `reopened_store_does_not_reissue_an_event_id` (VT-6, replacing
  `store_id_is_stable_across_reopen`), `event_id_is_not_matchable_by_query`
  (VT-7), `contains_event_id_reports_membership` (VT-7/ES-41),
  `append_stamps_a_recorded_time` and `recorded_time_survives_a_reopen` (VT-9).
  Three of those eight have a wrong implementation already named in this ADR and
  are the mutants the code run owes: a store that fabricates identity at read
  time, a store that keeps its `StoreId` and restarts its counter, and the
  position-only lookup. **The other five have no named wrong implementation yet,
  and by this repository's own corollary a rule no adapter can fail is
  decorative** — writing them is not the same as making them bite, and the code
  run owes a mutant per rule or an argument for why one cannot exist. Two of the
  ten cannot be written at all before their crate exists:
  `ingest_preserves_origin_identity` (VT-5) and
  `restored_peer_does_not_reissue_identities` (VT-6)
  both belong to `happenstance-sync-testkit`, which does not exist and whose every
  `SY` clause `Rule:` line says so. VT-9's third rule,
  `convergent_projection_is_interleaving_independent`, is SY-20's and is in the
  same position.
- **A 128-bit `StoreId` has never been checked against `wasm32`.**
  `adapter-shapes.md:220` records that Workers SQL widens integers through a JS
  number, bounding positions at 2⁵³ rather than 2⁶⁴, and that `NonZeroU64` already
  permits positions that adapter cannot round-trip. A 128-bit value cannot survive
  a JS number at all. In Rust memory `[u8; 16]` is unremarkable; the exposure is
  entirely at the storage and JS boundary, and the mitigation an adapter must take
  is to persist a `StoreId` as bytes or as text and **never** as an integer or a
  pair of integers. No capability row states this. One is owed, and phase 9 is the
  first place anyone finds out by running it.
- **The `happenstance-sync` swap has not been compiled.** The placeholders are
  deleted and the crate re-points at `happenstance-core`; the derives this ADR
  names (`Copy + Ord + Hash + Eq`) are what `Watermark`'s `binary_search_by_key`
  and `ReplicatedEvent` need by inspection, not by compilation.
- **Nothing here has been run on a store that survives its process.** Every
  durability claim about `StoreId` and `RecordedAt` rests on `Capability::REOPEN`,
  which reopens by instruction. Phase 8 is the first store that can lose one to a
  fault.

## Consequences

**Good.** The three types stop being a placeholder in the wrong crate whose own
documentation asks to be deleted. `MemoryEventStore`'s `impl IngestStore` — the
coherence proof at `crates/happenstance-sync/src/ingest.rs:161-175`, whose bodies
are `todo!()` because "`SequencedEvent` has no field an `EventId` fits in" — gets
the field it named.

**Good.** Identity costs no entropy source, no clock and no allocation to
generate, which is what makes it implementable on `wasm32` at all. That was VT-5's
argument for the pair over a UUID and this ADR spends none of it.

**Bad, and the largest cost in the decision.** Every implementation of
`EventStore` in the workspace changes in one commit: `SequencedEvent::new` gains
two arguments, and the trait gains two required methods (`head`, from ES-30, and
`contains_event_id`). ES-30 already counted the population and this ADR does not
get to recount it: **seven impls meant to be conformant, five of them
skeletons**, plus **three more written to be rejected** — `BorrowHoldingStore`,
`AwaitAcrossBorrowStore` and `SendStoreWithLocalError` — which ES-30 excludes
from the seven on exactly that criterion (`:3289-3304`). Ten bodies, not sixteen;
the earlier draft of this paragraph added the skeletons to the seven that already
contained them. Plus the mutant registry. There is no incremental path, and
`#[non_exhaustive]` is the reason there is not.

**Bad.** An adapter can satisfy every rule in the suite while minting its
`StoreId` from its hostname, because no in-process fixture can present it with a
restore. The rule catches reissue, not the mistake that causes it. That is the
gap phase 13 closes and nothing before it does.

**Neutral.** The redundant position (`SequencedEvent.position` versus
`id.position()`) will read as a defect to every reviewer who has not read the
ingest path. It is documented on the field rather than argued each time.

## Alternatives rejected

- **`RecordedAt` as a plain `u64`, or as a type alias** — the RUNBOOK's ask and
  the sync placeholder's shape. The alias loses on coherence: `impl From<i64> for
  SystemTime` is a foreign trait for a foreign type and the orphan rule refuses
  it, so the alias forecloses the one conversion every `std` caller wants.
  Unsigned loses on subtraction, which is the operation timestamps exist for.

- **A `std`-gated `RecordedAt::now()`.** One line and it would be used everywhere.
  Rejected because a Cargo feature is resolved per target, not per capability, so
  a `wasm32` build with `std` enabled would compile a `now()` on the exact platform
  VT-9's falsifier is about. Whether `SystemTime::now()` is usable there is a
  question this ADR has not verified and phase 9 answers; shipping the convenience
  now would mean shipping it without the answer.

- **`StoreId::random()`, or any minting in `happenstance-core`.** Requires an
  entropy source, which `no_std` does not have and which on `wasm32` is a
  JavaScript binding. VT-5 counts "no entropy source" among the reasons the pair
  beat a UUID; adding one to the contract crate would spend that on the first line
  of the implementation.

- **`FromStr` for `StoreId`.** An adapter persisting the hex form wants it, and
  every adapter writing its own hex parser is duplication. Rejected here anyway:
  an adapter may persist `to_bytes()` and needs no parser at all, and a string form
  brings a validation error type, which is ADR-0015's family and not this
  question's. It is additive later.

- **`EventId` with public fields**, matching `SequencedEvent`. Rejected on this
  ADR's own argument rather than on VT-5's, which is important because VT-5 is
  frozen and would otherwise appear to have decided it: VT-5 (`:742-747`) argues
  for a newtype over an alias and is satisfied either way. What decides it is the
  hazard the sync placeholder wrote down at `identity.rs:90-96` — a transparent
  pair lets a caller assemble an identity from the wrong store's identifier with
  no word from the compiler — and public fields hand that back under a different
  name. The cost is two accessor calls, and it is the whole cost.

- **`u128` for `StoreId`.** The obvious spelling of "a 128-bit value", and it
  loses on three counts stated in
  [§1](#1-three-types-in-happenstance-core-and-the-placeholders-are-deleted):
  it has an endianness every adapter must choose and two adapters may choose
  differently; its `Ord` is numeric where the persisted bytes' collation is
  lexicographic, so the convergent sort VT-5 relies on agrees only under one of
  those choices; and it invites arithmetic on a value VT-6 makes opaque. It is
  also the representation most likely to be silently truncated on Workers SQL,
  where integers pass through a JS number (`adapter-shapes.md:220`).

- **A nested `Stamp { position, id, recorded_at }` passed to
  `SequencedEvent::new(stamp, event)`**, so that a future store-assigned fact is
  additive inside `Stamp`. It moves the arity problem one type over —
  `Stamp::new` has the same three positional arguments and the same `error[E0061]`
  — and VT-4 `[FROZEN]` mandates four public fields on `SequencedEvent`, which a
  nested struct flatly contradicts.

- **Keeping the two-argument `new` and adding `with_id` / `with_recorded_at`
  defaulting to zero values.** The only option with zero downstream edits, and the
  worst one available: a zero `StoreId` and a zero `RecordedAt` are exactly the
  made-up-at-read-time values VT-4's `Rejects:` line names, they pass every rule
  that reads back what it just wrote in one process, and they fail the first
  reopen — in a deployment rather than in CI.

- **`contains_event_id` on `IngestStore` only**, where the sketch already has it as
  `holds`. Loses to VT-7's frozen §3 placement, and independently to VT-8: every
  store already owes the uniqueness guarantee, so the operation costs no adapter an
  index it does not already have.

- **A batched membership operation**, `&[EventId] -> Vec<bool>`, on the argument
  that one round trip per event is fatal to `happenstance-neon` and to Kestrel
  Rotor's 34-minute satellite window. Rejected because the bulk path does not go
  through this operation at all: VT-8's store-level uniqueness is what makes bulk
  ingest idempotent in a bounded number of round trips, and E2E-36's conclusion
  says so. If replication later wants a batched form it belongs on `IngestStore`,
  where the batch is already the unit, rather than as a widening of the contract
  port.

## Amendments this decision owes the specification

Recorded rather than applied, because four of the nine touch existing `[FROZEN]`
clauses — VT-4 (#6), VT-5 (#7), ES-19 (#8) and VT-3's prose (#9) — and a fifth
(#1) writes a new one, and this repository's rule is that a frozen clause changes
by ADR and not by edit.
**No existing frozen clause is overturned by this decision, and none has a
normative sentence altered.** Every amendment to one is to a
`Rule:` line, a citation or a body paragraph; the only new normative text is
ES-41, which is this ADR's to write because VT-7 `[FROZEN]` requires the
operation and forward-references the clause. This ADR is the authority for each.

1. **ES-41 is new** (§3, `EventStore`), `[FROZEN]`, inserted after ES-40
   (`:3679`), which is today the highest `ES` ID. There is none today, and VT-7
   `[FROZEN]` forward-references it at `:797-798`. Text: *"`EventStore` MUST
   declare `async fn contains_event_id(&self, id: EventId) -> Result<bool,
   Self::Error>`, reporting whether the store holds an event with that identity.
   It MUST be a **required** method, not a provided one."* — the `async fn`
   spelling matters and matches ES-30 (`:3278`); a clause written without it
   describes a different method under `trait_variant`.
   `Rule:` new `contains_event_id_reports_membership`.
   `Cases:` E2E-32, E2E-34, E2E-36 — VT-7's own, since this is the operation VT-7
   requires. `Rejects:` a store that answers by position
   alone and ignores the `StoreId` — the natural `SELECT 1 … WHERE position = ?`,
   which passes every single-store rule in the suite and reports a peer's event as
   present whenever the local log is long enough.

   **The clause MUST carry a CF-25 sentence or `spec-trace` fails it.** CF-25
   (`:7096-7102`) is `[FROZEN]` and mechanically checked: a `[FROZEN]` port clause
   with an axis that has no far-end row and no named risk acceptance is a build
   failure. Required addendum, in the clause body: *"Frozen with no adapter
   instrument at the far end of any axis; the exposure is transport (one round
   trip on an adapter with no connection) and completeness (ES-39), and the risk
   acceptance is ADR-0013's, which accepts the six axes phase 4 does not
   discharge."* This makes ES-41 **ordered after ADR-0013's amendments**, not
   merely concurrent with them.

2. **§1.3's census moves.** `:218-221` reads 193 clause IDs / 191 normative /
   135 `[FROZEN]`. ES-41 makes it 194 / 192 / 136. **This count is hand-computed
   and three ADRs in this phase move it** — ADR-0012 may add a CF clause for
   `MID_BATCH_FAULT` and ADR-0013 moves a maturity marker — so the run that applies
   these amendments must apply all of them and count **once**.

3. **VT-6 `[PROVISIONAL]`, Rule line (`:761-762`).** Current text, both lines:
   *"`Rule:` new `store_id_is_stable_across_reopen`; the non-reissue half by
   `happenstance-sync-testkit`'s new `restored_peer_does_not_reissue_identities`"*.
   Required text: *"`Rule:` new `reopened_store_does_not_reissue_an_event_id`,
   gated on `Capability::REOPEN`, which asserts both that two events appended in
   one open share a `StoreId` and that an event appended after a reopen has an
   `EventId` matching neither; the restore half by `happenstance-sync-testkit`'s
   new `restored_peer_does_not_reissue_identities`"*. **Only the first rule is
   replaced**, and the two assertions are one rule rather than two — the first
   exists only to retain what the replaced rule caught by accident (mint-per-append),
   per [§3](#3-storeid-names-an-incarnation-and-the-should-prefer-becomes-a-conditional-must). The sync-testkit half is untouched and stays — it is the only
   instrument for the failure the clause is actually about, and deleting it with
   the rule beside it would remove the clause's one far-end pointer. The first is
   replaced because the named rule fails the mint-per-open mechanism the clause's
   own body permits at `:784-787`.

4. **VT-6 `[PROVISIONAL]`, final paragraph (`:779-787`) and marker (`:757-760`).**
   The "SHOULD prefer the first" becomes the conditional MUST in
   [§3](#3-storeid-names-an-incarnation-and-the-should-prefer-becomes-a-conditional-must),
   and the falsifier is restated: not "an adapter that can detect neither" — which
   the second mechanism already survives — but **a deployment in which the
   mint-once path is unavailable and mint-per-open makes every peer's `Watermark`
   grow without bound**, candidate the Cloudflare Durable Object, observed at phase
   9 and measured at phase 13. Add: `happenstance-core` mints nothing; a `StoreId`
   MUST be persisted as bytes or text and never as an integer, because
   `adapter-shapes.md:220` bounds integers at 2⁵³ on Workers SQL.

5. **VT-9 `[PROVISIONAL]`, Rule line (`:861-865`).** The line names **three**
   rules, and only the first two are constrained here: `append_stamps_a_recorded_time`
   and `recorded_time_survives_a_reopen`, which are the event-store suite's. The
   third — SY-20's `convergent_projection_is_interleaving_independent`, which the
   line assigns the "not an ordering key" half — is `happenstance-sync-testkit`'s
   and is **not** touched; it asserts that a fold reading the time *fails*, which
   is the prohibition's enforcement rather than a violation of it. Add, after the
   two event-store rule names: *"Neither rule may compare two `RecordedAt` values
   in either direction, compare a `RecordedAt` against a `SequencePosition`, or
   check a value for plausibility against the harness's own clock — which CF-33
   forbids independently."* The reason belongs in the body: the clause's third
   MUST binds the contract, and the conformance suite is the contract's executable
   form, so a rule that asserts an order states one.

6. **VT-4 `[FROZEN]`, body (`:697-702`).** Says "Adding two fields is therefore
   one signature change across the workspace". Must say that
   `SequencedEvent::new` is **superseded, not widened** — arity 2 → 4,
   `error[E0061]` for anything else, with no deprecated arm — and that
   `#[non_exhaustive]` is what makes `new` the entire compatibility surface
   (`error[E0639]` on the literal downstream), which is the opposite of the
   protection the attribute is usually credited with.

7. **VT-5 `[FROZEN]`, newtype paragraph (`:742-747`).** Add that `EventId`'s two
   fields are private with `store()` / `position()` accessors, so that VT-4's "all
   four fields MUST be public" is not over-generalised from `SequencedEvent` to
   every new value type.

8. **ES-19 `[FROZEN]`, prose after `:2844`.** Add one sentence: identity does not
   change `append`'s return type; the caller that needs the `EventId` of what it
   wrote reads it back, for the same reason the returned position is not a sound
   `after`; and `EventStore` gains no `store_id()` accessor, because a synchronous
   infallible one excludes `happenstance-neon` by `adapter-shapes.md:216-217`.

9. **VT-3's prose paragraph (`:670-677`).** Add the arity half:
   `Event::into_parts` returns a `#[non_exhaustive] EventParts` struct rather than
   a positional 4-tuple, so the number of an `Event`'s parts stops being public
   API. **ADR-0012 amends the same paragraph** for the doc-comment half that ES-17
   owns; both must be applied together.

### …and the RUNBOOK, under rule 5

The phase body is corrected in the same commit as the clauses it disagrees with.
Four cells, all in phase 4's scope, none of them an edit this run may make:

- **`:2891-2894`** — "a plain `u64` of milliseconds since the epoch" becomes a
  newtype over `i64` (VT-9, `:881-895`), and "**A rule that it is non-decreasing
  with position**" is **struck**, because VT-9's third MUST forbids the contract
  from stating that relationship.
- **`:557` only** — the ledger cell naming that same forbidden rule as VT-9's
  *falsifier* is replaced by VT-9's real one (no wall clock at append time) and
  VT-6's restated one, with owning phases 4 (decision), 9 and 13 (observation)
  rather than "5, exercised 13". **`:558` is correct and must not be swept up
  with it**: it gives VT-10 to phase 13, which is where this ADR also leaves it.
- **`:447-448`, and not a line further.** Those two cells assign "Event identity
  across instances" (VT-4 – VT-10) and "A store-assigned time on `SequencedEvent`"
  (VT-9) to phase 5, while naming ADR-0014 — this one — as the ADR. Phase 4's own
  body claims both (`:2881-2894`, two unticked checkboxes), and under rule 5 the
  body a phase owns beats a ledger cell about it. **State the authority
  accurately**: unlike VT-14, VT-21, VT-28 and CF-18, the specification does *not*
  name a phase for VT-6 or VT-9 in terms, so "the specification names phase 4 and
  wins" would be false here. The correction is a RUNBOOK-internal one.
  `:449` and `:450` are ADR-0015's rows and `:451` is the wire format, which is
  *correctly* phase 5 — a run that widens this edit to `:447-451` will move two
  rows that belong to another ADR and one that is already right.
- **`:3043-3049`** — exit criterion 9 (`:3043-3044`) is reworded per
  [§6](#6-the-constructor-shape-new-is-superseded-not-widened), and criterion 10
  (`:3045-3049`) is reworded per
  [§8](#8-what-does-not-change-and-one-thing-that-does): it is
  true of `append` and false of `EventStore`, which gains `contains_event_id`
  from this ADR and `head` from ES-30. `:3050` begins a different criterion
  (phase 3's value-edge rules against the frozen minima) and is not in scope.
